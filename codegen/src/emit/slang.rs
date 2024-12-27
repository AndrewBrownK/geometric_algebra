// TODO integrate with "slang" shader language
//  hypothetically should be nice to avoid monolith file

// I could emit to slang directly,
// but might also want to check out rust-slang integrations like https://github.com/tangmi/slang-rs/

use std::{fs, thread};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::io::{BufRead, BufReader, BufWriter, ErrorKind, Write};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use anyhow::bail;
use indicatif::ProgressFinish;
use tokio::task::JoinSet;

use crate::algebra::basis::BasisElement;
use crate::algebra::multivector::{MultiVec, MultiVecRepository};
use crate::ast::datatype::{ExpressionType, MultiVector};
use crate::ast::expressions::{AnyExpression, FloatExpr, IntExpr, MultiVectorExpr, MultiVectorGroupExpr, MultiVectorVia, Vec2Expr, Vec3Expr, Vec4Expr};
use crate::ast::RawVariableDeclaration;
use crate::ast::traits::{BinaryOps, CommentOrVariableDeclaration, progress_style, RawTraitDefinition, RawTraitImplementation, TraitArity, TraitImplRegistry, TraitKey, TraitParam, TraitTypeConsensus};
use crate::emit::sort_trait_impls;
use crate::utility::CollectResults;

#[derive(Copy, Clone)]
pub struct Slang {
    pub prefer_fancy_infix: bool,
    // Internal/private stuff
    fancy_infix: Option<BinaryOps>,
}

impl Slang {
    pub fn new() -> Self {
        Slang {
            prefer_fancy_infix: false,
            fancy_infix: None,
        }
    }
    pub fn write_src<P: AsRef<Path>, const AntiScalar: BasisElement>(
        mut self,
        src_folder: P,
        algebra_name: &'static str,
        multi_vecs: Arc<MultiVecRepository<AntiScalar>>,
        impls: Arc<TraitImplRegistry>,
    ) {

        let rt = tokio::runtime::Runtime::new().expect("Tokio must work");
        let result = rt.block_on(async {
            self.write_src_inner(src_folder, algebra_name, multi_vecs, impls).await
        });
        if let Err(e) = result {
            panic!("Slang Errors: {e:?}");
        }
    }

    //
    async fn write_src_inner<P: AsRef<Path>, const AntiScalar: BasisElement>(
        mut self,
        src_folder: P,
        algebra_name: &'static str,
        multi_vecs: Arc<MultiVecRepository<AntiScalar>>,
        impls: Arc<TraitImplRegistry>,
    ) -> anyhow::Result<()> {
        let src_folder = src_folder.as_ref().to_path_buf().join(Path::new("integrations/slang"));
        let folder_data = src_folder.join(Path::new("data"));
        let folder_data_impls = src_folder.join(Path::new("data/impls"));
        let folder_traits = src_folder.join(Path::new("traits"));
        let folder_traits_impls = src_folder.join(Path::new("traits/impls"));

        fs::create_dir_all(&folder_data_impls)?;
        fs::create_dir_all(&folder_traits_impls)?;

        let mut defs = impls.get_defs().await;
        defs.sort_by(|a, b| a.names.trait_key.cmp(&b.names.trait_key));
        let defs = defs;
        let fancy_infix = *impls.infix_trick.lock();
        self.fancy_infix = fancy_infix;
        let impls = impls.get_impls().await;
        let mut mvs = multi_vecs.declarations();
        mvs.sort_by(|a, b| a.name.cmp(&b.name));
        let mvs = mvs;
        let mv_docs = multi_vecs.documentation();

        let file_path = src_folder.join(Path::new("generated-files.txt"));
        if let Ok(file) = fs::OpenOptions::new().read(true).open(&file_path) {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let line = line?;
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                let path = Path::new(line);
                match fs::remove_file(path) {
                    Ok(_) => {}
                    Err(e) if e.kind() == ErrorKind::NotFound => {}
                    e => e?,
                };
            }
        }

        let multi_progress = Arc::new(indicatif::MultiProgress::new());
        let mut join_set: JoinSet<anyhow::Result<()>> = JoinSet::new();
        let (started_file, mut rx) = tokio::sync::mpsc::unbounded_channel::<PathBuf>();
        join_set.spawn(async move {
            let file = fs::OpenOptions::new().write(true).create(true).truncate(true).open(&file_path)?;
            let mut file = BufWriter::new(file);
            let mut paths = vec![];
            while let Some(p) = rx.recv().await {
                use std::fmt::Write;
                let mut path = String::new();
                write!(&mut path, ".")?;
                for p in p.iter() {
                    let p = p.to_string_lossy();
                    write!(&mut path, "/{p}")?;
                }
                paths.push(path);
            }
            paths.sort();
            for p in paths {
                writeln!(&mut file, "{p}")?;
            }
            Ok(())
        });

        let qty_mvs = mvs.len() as u64;
        let data_pb = Arc::new(multi_progress.add(indicatif::ProgressBar::new(qty_mvs).with_finish(ProgressFinish::AndLeave)));
        data_pb.set_style(progress_style());
        data_pb.set_message("Slang - Data Definitions");

        let qty_defs = defs.len() as u64;
        let trait_pb = Arc::new(multi_progress.add(indicatif::ProgressBar::new(qty_defs).with_finish(ProgressFinish::AndLeave)));
        trait_pb.set_style(progress_style());
        trait_pb.set_message("Slang - Trait Definitions");

        let qty_impls = impls.len() as u64;
        let impls_pb = Arc::new(multi_progress.add(indicatif::ProgressBar::new(qty_impls)));
        impls_pb.set_style(progress_style());
        impls_pb.set_message("Slang - Distributing Trait Implementations");

        // let qty_files = qty_mvs + qty_defs + 4; // traits.slang, data.slang, lib.slang, simd.slang
        // let fmt_pb = Arc::new(multi_progress.add(indicatif::ProgressBar::new(qty_files).with_finish(ProgressFinish::AndLeave)));
        // fmt_pb.set_style(progress_style());
        // fmt_pb.set_message("Slang - rustfmt");

        // let (finished_file, mut rx) = tokio::sync::mpsc::unbounded_channel::<PathBuf>();
        // let fmt_pb2 = fmt_pb.clone();
        // join_set.spawn(async move {
        //     while let Some(p) = rx.recv().await {
        //         Self::format_file(p).await?;
        //         fmt_pb2.inc(1);
        //     }
        //     fmt_pb2.finish();
        //     Ok(())
        // });

        // data definitions
        for multi_vec in mvs.iter() {
            let multi_vec = *multi_vec;
            let mv = MultiVector::from(multi_vec);
            let n = mv.name();
            let lsc = TraitKey::new(n).as_lower_snake();
            let folder_data = folder_data.clone();
            let doc = mv_docs.get(&n.to_string()).cloned();
            let tx2 = started_file.clone();
            // let tx3 = finished_file.clone();

            let pb2 = data_pb.clone();
            join_set.spawn(async move {
                let file_path = folder_data.join(Path::new(&lsc)).with_extension("slang");
                tx2.send(file_path.clone())?;
                let file = fs::OpenOptions::new().write(true).create(true).truncate(true).open(&file_path)?;
                let mut file = BufWriter::new(file);
                writeln!(&mut file, "implementing {algebra_name};")?;
                writeln!(&mut file, "using data::*;")?;
                self.declare_multi_vector(&mut file, multi_vec, doc)?;
                writeln!(&mut file, "__include \"impls/{lsc}\";")?;
                // tx3.send(file_path)?;
                pb2.inc(1);
                Ok(())
            });
        }

        // trait definitions
        for td in defs.iter() {
            let td = td.clone();
            if let TraitTypeConsensus::NoVotes = *td.output.read() {
                trait_pb.inc(1);
                continue;
            }
            let k = td.names.trait_key;
            let n = k.as_upper_camel();
            let lsc = k.as_lower_snake();
            let folder_traits = folder_traits.clone();
            match n.as_str() {
                "Add" | "Sub" | "Mul" | "Div" | "Shl" | "Shr" | "BitAnd" | "BitOr" | "BitXor" | "Neg" | "Not" => {
                    trait_pb.inc(1);
                    continue;
                }
                "Into" | "TryInto" => {
                    trait_pb.inc(1);
                    continue;
                }
                _ => {}
            }
            let tx2 = started_file.clone();
            // let tx3 = finished_file.clone();
            let pb2 = trait_pb.clone();
            join_set.spawn(async move {
                let file_path = folder_traits.join(Path::new(lsc.as_str())).with_extension("slang");
                tx2.send(file_path.clone())?;
                let file = fs::OpenOptions::new().write(true).create(true).truncate(true).open(&file_path)?;
                let mut file = BufWriter::new(file);
                // TODO remove these imports when they are unused
                writeln!(&mut file, "using data::*;")?;
                self.declare_trait_def(&mut file, td)?;
                writeln!(&mut file, "__include ./impls/{lsc};")?;
                // tx3.send(file_path)?;
                pb2.inc(1);
                Ok(())
            });
        }

        let mut impl_files: HashMap<String, (Vec<Arc<RawTraitImplementation>>, BTreeSet<TraitKey>)> = HashMap::new();

        // trait impl distribution
        for i in impls {
            let k = i.definition.names.trait_key;
            let (folder, name) = match k.as_upper_camel().as_str() {
                "Add" | "Sub" | "Mul" | "Div" | "Shl" | "Shr" | "BitAnd" | "BitOr" | "BitXor" | "Neg" | "Not" => {
                    let ExpressionType::Class(mv) = i.owner else { continue };
                    let n = TraitKey::new(mv.name()).as_lower_snake();
                    ("data", n)
                }
                "Into" | "TryInto" => {
                    let Some(ExpressionType::Class(mv)) = i.other_type_params.get(0) else { continue };
                    let n = TraitKey::new(mv.name()).as_lower_snake();
                    ("data", n)
                }
                _ => ("traits", k.as_lower_snake()),
            };
            let i2 = i.clone();
            impl_files
                .entry(format!("{folder}/impls/{name}.slang"))
                .and_modify(move |(v, deps)| {
                    for t_dep in i2.definition.dependencies.lock().iter().cloned() {
                        deps.insert(t_dep);
                    }
                    v.push(i2);
                })
                .or_insert_with(|| {
                    let t_deps: BTreeSet<_> = i.definition.dependencies.lock().iter().cloned().collect();
                    (vec![i], t_deps)
                });
            impls_pb.inc(1);
        }
        impls_pb.finish();
        // fmt_pb.inc_length(impl_files.len() as u64);

        // trait implementations
        let (closed_types, mut closed_types_rx) = tokio::sync::mpsc::unbounded_channel::<String>();
        for (file_path, (mut impls, deps)) in impl_files {
            let src_folder = src_folder.clone();
            let tx2 = started_file.clone();
            // let tx3 = finished_file.clone();
            let multi_progress = multi_progress.clone();
            join_set.spawn(async move {
                let skip_dependencies = file_path.ends_with("constraint_valid.slang");
                let file_path = src_folder.join(Path::new(file_path.as_str())).with_extension("slang");

                let qty_impls = impls.len() as u64;
                let qty_deps = deps.len() as u64;
                let mut pb = None;
                if qty_impls > 100 {
                    pb = Some(Arc::new(multi_progress.add(indicatif::ProgressBar::new(qty_impls + qty_deps + 2))));
                }
                if let Some(pb) = &pb {
                    pb.set_style(progress_style());
                    let fpd = file_path.display();
                    pb.set_message(format!("Slang - {fpd}"));
                }

                tx2.send(file_path.clone())?;
                let file = fs::OpenOptions::new().write(true).create(true).truncate(true).open(&file_path)?;
                let mut file = BufWriter::new(file);
                let mut deps_set = HashSet::new();
                for dep in deps {
                    if let Some(pb) = &pb {
                        pb.inc(1);
                    }
                    deps_set.insert(dep);
                    if skip_dependencies { continue }
                    if self.prefer_fancy_infix {
                        let lsc = dep.as_lower_snake();
                        writeln!(&mut file, "using traits::{lsc};")?;
                    } else {
                        let ucc = dep.as_upper_camel();
                        writeln!(&mut file, "using traits::{ucc};")?;
                    }
                }
                sort_trait_impls(&mut impls, deps_set)?;
                if let Some(pb) = &pb {
                    pb.inc(1);
                }


                // writeln!(&mut file, "use crate::data::*;")?;
                // writeln!(&mut file, "use crate::simd::*;")?;
                let mut already_granted_infix = BTreeSet::new();
                for i in impls {
                    let ucc = i.definition.names.trait_key.as_upper_camel();
                    match ucc.as_str() {
                        "Into" => self.write_trait_from(&mut file, i),
                        "TryInto" => self.write_trait_try_from(&mut file, i),
                        _ => self.declare_trait_impl(&mut file, i, &mut already_granted_infix),
                    }?;
                    if let Some(pb) = &pb {
                        pb.inc(1);
                    }
                }
                // tx3.send(file_path)?;
                if let Some(pb) = &pb {
                    pb.inc(1);
                    pb.finish_and_clear();
                }
                Ok(())
            });
        }

        // data.slang
        let src_folder2 = src_folder.clone();
        let tx2 = started_file.clone();
        // let tx3 = finished_file.clone();
        join_set.spawn(async move {
            let file_path = src_folder2.join(Path::new("data.slang"));
            tx2.send(file_path.clone())?;
            let file = fs::OpenOptions::new().write(true).create(true).truncate(true).open(&file_path)?;
            let mut file = BufWriter::new(file);
            writeln!(&mut file, "implementing {algebra_name};")?;
            for mv in mvs {
                let n = mv.name;
                let lsc = TraitKey::new(n).as_lower_snake();
                writeln!(&mut file, "__include \"data/{lsc}\";")?;
            }
            // tx3.send(file_path)?;
            Ok(())
        });

        // traits.slang
        let src_folder2 = src_folder.clone();
        let tx2 = started_file.clone();
        // let tx3 = finished_file.clone();
        join_set.spawn(async move {
            let file_path = src_folder2.join(Path::new("traits.slang"));
            tx2.send(file_path.clone())?;
            let file = fs::OpenOptions::new().write(true).create(true).truncate(true).open(&file_path)?;
            let mut file = BufWriter::new(file);
            writeln!(&mut file, "implementing {algebra_name};")?;
            for td in defs.iter() {
                if let TraitTypeConsensus::NoVotes = *td.output.read() {
                    continue;
                }
                let td = td.clone();
                let k = td.names.trait_key;
                let n = k.as_upper_camel();
                let lsc = k.as_lower_snake();
                match n.as_str() {
                    "Add" | "Sub" | "Mul" | "Div" | "Shl" | "Shr" | "BitAnd" | "BitOr" | "BitXor" | "Neg" | "Not" => continue,
                    "Into" | "TryInto" => continue,
                    _ => {}
                }
                writeln!(&mut file, "__include \"traits/{lsc}\";")?;
            }
            // tx3.send(file_path)?;
            Ok(())
        });

        // lib.slang
        let src_folder2 = src_folder.clone();
        let tx2 = started_file.clone();
        // let tx3 = finished_file.clone();
        join_set.spawn(async move {
            let n = format!("{algebra_name}.slang");
            let file_path = src_folder2.join(Path::new(n.as_str()));
            tx2.send(file_path.clone())?;
            let file = fs::OpenOptions::new().write(true).create(true).truncate(true).open(&file_path)?;
            let mut file = BufWriter::new(file);
            writeln!(&mut file, "namespace data {{")?;
            writeln!(&mut file, "    __include data;")?;
            writeln!(&mut file, "}}")?;
            writeln!(&mut file, "namespace traits {{")?;
            writeln!(&mut file, "    __include traits;")?;
            writeln!(&mut file, "}}")?;
            Ok(())
        });

        drop(started_file);
        // drop(finished_file);
        let result = join_set.collect_results().await;

        tokio::task::spawn_blocking(move || {
            thread::sleep(Duration::from_secs(5));
            drop(closed_types);
            while let Some(thing) = closed_types_rx.blocking_recv() {
                println!("{thing}");
            }
        });

        result
    }

    fn write_type<W: Write>(&self, w: &mut W, data_type: ExpressionType) -> anyhow::Result<()> {
        match data_type {
            ExpressionType::Int(_) => write!(w, "uint")?,
            ExpressionType::Float(_) => write!(w, "float")?,
            ExpressionType::Vec2(_) => write!(w, "float2")?,
            ExpressionType::Vec3(_) => write!(w, "float3")?,
            ExpressionType::Vec4(_) => write!(w, "float4")?,
            ExpressionType::Class(mv) => {
                let n = mv.name();
                write!(w, "{n}")?;
            }
        }
        Ok(())
    }

    fn write_expression<W: Write>(&self, w: &mut W, expr: &AnyExpression, grouping_provided: bool) -> anyhow::Result<()> {
        match expr {
            AnyExpression::Int(e) => self.write_int(w, e)?,
            AnyExpression::Float(e) => self.write_float(w, e, grouping_provided)?,
            AnyExpression::Vec2(e) => self.write_vec2(w, e, grouping_provided)?,
            AnyExpression::Vec3(e) => self.write_vec3(w, e, grouping_provided)?,
            AnyExpression::Vec4(e) => self.write_vec4(w, e, grouping_provided)?,
            AnyExpression::Class(e) => self.write_multi_vec(w, e)?,
        }
        Ok(())
    }

    fn write_int<W: Write>(&self, w: &mut W, expr: &IntExpr) -> anyhow::Result<()> {
        match expr {
            IntExpr::Variable(v) => {
                let name = &v.decl.name.0;
                let no = v.decl.name.1;
                if name.as_str() == "self" && no == 0 {
                    write!(w, "this")?;
                } else if no == 0 {
                    write!(w, "{name}")?;
                } else {
                    let no = no + 1;
                    write!(w, "{name}_{no}")?;
                }
            }
            IntExpr::Literal(l) => {
                write!(w, "{l}")?;
            }
            IntExpr::TraitInvoke10ToInt(t, mv) => {
                let n = mv.name();
                let method = t.as_lower_snake();
                write!(w, "{n}.{method}()")?;
            }
        }
        Ok(())
    }

    fn write_f32<W: Write>(&self, w: &mut W, f: f32) -> anyhow::Result<()> {
        if f.fract() == 0.0 {
            write!(w, "{f:.1}")?;
        } else {
            write!(w, "{f}")?;
        }
        Ok(())
    }

    fn write_float<W: Write>(&self, w: &mut W, expr: &FloatExpr, grouping_provided: bool) -> anyhow::Result<()> {
        match expr {
            FloatExpr::Variable(v) => {
                let name = &v.decl.name.0;
                let no = v.decl.name.1;
                if name.as_str() == "self" && no == 0 {
                    write!(w, "this")?;
                } else if no == 0 {
                    write!(w, "{name}")?;
                } else {
                    let no = no + 1;
                    write!(w, "{name}_{no}")?;
                }
            }
            FloatExpr::Literal(l) => self.write_f32(w, *l)?,
            FloatExpr::FromInt(i) => {
                write!(w, "float(")?;
                self.write_int(w, i)?;
                write!(w, ")")?;
            }
            FloatExpr::AccessVec2(v, i) => {
                self.write_vec2(w, v.as_ref(), false)?;
                write!(w, "[{i}]")?;
            }
            FloatExpr::AccessVec3(v, i) => {
                self.write_vec3(w, v.as_ref(), false)?;
                write!(w, "[{i}]")?;
            }
            FloatExpr::AccessVec4(v, i) => {
                self.write_vec4(w, v.as_ref(), false)?;
                write!(w, "[{i}]")?;
            }
            FloatExpr::AccessMultiVecGroup(mv, i) => {
                self.write_multi_vec(w, mv)?;
                let el = mv.mv_class.elements()[*i as usize];
                write!(w, ".{el}")?;
            }
            FloatExpr::AccessMultiVecFlat(mv, i) => {
                self.write_multi_vec(w, mv)?;
                let el = mv.mv_class.elements()[*i as usize];
                write!(w, ".{el}")?;
            }
            FloatExpr::TraitInvoke11ToFloat(t, arg) => {
                let method = t.as_lower_snake();
                // TODO fancy infix can conflict with variable names
                match (&self.fancy_infix, self.prefer_fancy_infix) {
                    (Some(infix), true) => {
                        let op = infix.rust_operator();
                        write!(w, "(")?;
                        self.write_multi_vec(w, arg)?;
                        write!(w, " {op}{method}")?;
                        write!(w, ")")?;
                    }
                    _ => {
                        self.write_multi_vec(w, arg)?;
                        write!(w, ".{method}()")?;
                    }
                }
            }
            FloatExpr::Product(v, last_factor) => {
                let has_last_factor = *last_factor != 1.0;
                if v.is_empty() && !has_last_factor {
                    bail!("Attempted to write an empty product that should have been simplified");
                }
                let mut len = v.len();
                if has_last_factor {
                    len += 1;
                }
                if len > 1 && !grouping_provided {
                    write!(w, "(")?;
                }
                for (i, (factor, exponent)) in v.iter().enumerate() {
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    match (*exponent, i > 0) {
                        (f, _) if f == 0.0 => continue,

                        (1.0, false) => self.write_float(w, factor, false)?,
                        (-1.0, false) => {
                            if !grouping_provided {
                                write!(w, "(")?;
                            }
                            write!(w, "1.0/")?;
                            self.write_float(w, factor, false)?;
                            if !grouping_provided {
                                write!(w, ")")?;
                            }
                        }
                        (e, false) => {
                            if e.fract() == 0.0 && e <= i32::MAX as f32 && e >= i32::MIN as f32 {
                                let e = e as i32;
                                write!(w, "powi(")?;
                                self.write_float(w, factor, true)?;
                                write!(w, ", {e})")?;
                            } else {
                                write!(w, "powf(")?;
                                self.write_float(w, factor, true)?;
                                write!(w, ", {e})")?;
                            }
                        }

                        (1.0, true) => {
                            write!(w, " * ")?;
                            self.write_float(w, factor, false)?;
                        }
                        (-1.0, true) => {
                            write!(w, " / (")?;
                            self.write_float(w, factor, true)?;
                            write!(w, ")")?;
                        }
                        (e, true) => {
                            if e.fract() == 0.0 && e <= i32::MAX as f32 && e >= i32::MIN as f32 {
                                let e = e as i32;
                                write!(w, " * powi(")?;
                                self.write_float(w, factor, true)?;
                                write!(w, ", {e})")?;
                            } else {
                                write!(w, " * powf(")?;
                                self.write_float(w, factor, true)?;
                                write!(w, ", {e})")?;
                            }
                        }
                    }
                }
                match (*last_factor, len > 1) {
                    (fl, _) if fl == 1.0 => {}
                    (fl, false) => self.write_f32(w, fl)?,
                    (fl, true) => {
                        write!(w, " * ")?;
                        self.write_f32(w, fl)?
                    }
                }
                if len > 1 && !grouping_provided {
                    write!(w, ")")?;
                }
            }
            FloatExpr::Sum(v, last_addend) => {
                let has_last_addend = *last_addend != 0.0;
                if v.is_empty() && !has_last_addend {
                    bail!("Attempted to write an empty sum that should have been simplified");
                }
                let mut len = v.len();
                if has_last_addend {
                    len += 1;
                }
                if len > 1 && !grouping_provided {
                    write!(w, "(")?;
                }
                for (i, (addend, factor)) in v.iter().enumerate() {
                    match (*factor, i > 0) {
                        (f, _) if f == 0.0 => continue,

                        (1.0, false) => {}
                        (-1.0, false) => write!(w, "-")?,
                        (f, false) => {
                            self.write_f32(w, f)?;
                            write!(w, "*")?
                        }

                        (1.0, true) => write!(w, " + ")?,
                        (-1.0, true) => write!(w, " - ")?,
                        (f, true) if f > 0.0 => {
                            write!(w, " + ")?;
                            self.write_f32(w, f)?;
                            write!(w, " * ")?;
                        }
                        (f, true) if f < 0.0 => {
                            let f = -f;
                            write!(w, " - ")?;
                            self.write_f32(w, f)?;
                            write!(w, " * ")?;
                        }
                        _ => unreachable!("This match is complete across if conditions (unless NaN?)"),
                    }
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    self.write_float(w, addend, false)?;
                }
                match (*last_addend, len > 1) {
                    (fl, _) if fl == 0.0 => {}
                    (fl, false) => self.write_f32(w, fl)?,
                    (fl, true) if fl > 0.0 => {
                        write!(w, " + ")?;
                        self.write_f32(w, fl)?
                    }
                    (fl, true) if fl < 0.0 => {
                        let fl = -fl;
                        write!(w, " - ")?;
                        self.write_f32(w, fl)?
                    }
                    _ => {}
                }
                if len > 1 && !grouping_provided {
                    write!(w, ")")?;
                }
            }
            FloatExpr::Exp(factor, exponent, last_exponent) => {

                if exponent.is_none() && last_exponent.fract() == 0.0 {
                    write!(w, "powi(")?;
                } else {
                    write!(w, "powf(")?;
                }
                self.write_float(w, factor, true)?;
                write!(w, ", ")?;
                if let Some(exponent) = exponent {
                    self.write_float(w, exponent, *last_exponent == 1.0)?;
                    if *last_exponent != 1.0 {
                        write!(w, " * ")?;
                    }
                }
                if *last_exponent != 1.0 {
                    write!(w, "{last_exponent}")?;
                }
                write!(w, ")")?;
            }
        }
        Ok(())
    }

    fn write_vec2<W: Write>(&self, w: &mut W, expr: &Vec2Expr, grouping_provided: bool) -> anyhow::Result<()> {
        match expr {
            Vec2Expr::Variable(v) => {
                let name = &v.decl.name.0;
                let no = v.decl.name.1;
                if name.as_str() == "self" && no == 0 {
                    write!(w, "this")?;
                } else if no == 0 {
                    write!(w, "{name}")?;
                } else {
                    let no = no + 1;
                    write!(w, "{name}_{no}")?;
                }
            }
            Vec2Expr::Gather1(f) => {
                write!(w, "float2(")?;
                self.write_float(w, f, true)?;
                write!(w, ")")?;
            }
            Vec2Expr::Gather2(f0, f1) => {
                write!(w, "float2(")?;
                self.write_float(w, f0, true)?;
                write!(w, ", ")?;
                self.write_float(w, f1, true)?;
                write!(w, ")")?;
            }
            Vec2Expr::AccessMultiVecGroup(mv, i) => {
                self.write_multi_vec(w, mv)?;
                write!(w, ".group{i}")?;
            }
            Vec2Expr::Product(v, last_factor) => {
                let has_last_factor = *last_factor != [1.0; 2];
                if v.is_empty() && !has_last_factor {
                    bail!("Attempted to write an empty product that should have been simplified");
                }
                let mut len = v.len();
                if has_last_factor {
                    len += 1;
                }
                if len > 1 && !grouping_provided {
                    write!(w, "(")?;
                }
                for (i, (factor, exponent)) in v.iter().enumerate() {
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    match (*exponent, i > 0) {
                        (f, _) if f == 0.0 => continue,

                        (1.0, false) => self.write_vec2(w, factor, false)?,
                        (-1.0, false) => {
                            write!(w, "(float2(1.0) / ")?;
                            self.write_vec2(w, factor, false)?;
                            write!(w, ")")?;
                        }
                        (e, false) => {
                            if e.fract() == 0.0 && e <= i32::MAX as f32 && e >= i32::MIN as f32 {
                                let e = e as i32;
                                write!(w, "Simd32x2::powi(")?;
                                self.write_vec2(w, factor, true)?;
                                write!(w, ", {e})")?;
                            } else {
                                write!(w, "Simd32x2::powf(")?;
                                self.write_vec2(w, factor, true)?;
                                write!(w, ", {e})")?;
                            }
                        }

                        (1.0, true) => {
                            write!(w, " * ")?;
                            self.write_vec2(w, factor, false)?
                        }
                        (-1.0, true) => {
                            write!(w, " / (")?;
                            self.write_vec2(w, factor, true)?;
                            write!(w, ")")?;
                        }
                        (e, true) => {
                            if e.fract() == 0.0 && e <= i32::MAX as f32 && e >= i32::MIN as f32 {
                                let e = e as i32;
                                write!(w, " * Simd32x2::powi(")?;
                                self.write_vec2(w, factor, true)?;
                                write!(w, ", {e})")?;
                            } else {
                                write!(w, " * Simd32x2::powf(")?;
                                self.write_vec2(w, factor, true)?;
                                write!(w, ", {e})")?;
                            }
                        }
                    }
                }
                if *last_factor != [1.0; 2] {
                    if len > 1 {
                        write!(w, " * ")?;
                    }
                    let a = last_factor[0];
                    let b = last_factor[1];
                    if a == b {
                        write!(w, "float2(")?;
                        self.write_f32(w, a)?;
                        write!(w, ")")?;
                    } else {
                        write!(w, "float2(")?;
                        self.write_f32(w, a)?;
                        write!(w, ", ")?;
                        self.write_f32(w, b)?;
                        write!(w, ")")?;
                    }
                }
                if len > 1 && !grouping_provided {
                    write!(w, ")")?;
                }
            }
            Vec2Expr::Sum(v, last_addend) => {
                let has_last_addend = *last_addend != [0.0; 2];
                if v.is_empty() && !has_last_addend {
                    bail!("Attempted to write an empty sum that should have been simplified");
                }
                let mut len = v.len();
                if has_last_addend {
                    len += 1;
                }
                if len > 1 && !grouping_provided {
                    write!(w, "(")?;
                }
                for (i, (addend, factor)) in v.iter().enumerate() {
                    match (*factor, i > 0) {
                        (f, _) if f == 0.0 => continue,

                        (1.0, false) => {}
                        (-1.0, false) => write!(w, "-")?,
                        (f, false) => {
                            write!(w, "float2(")?;
                            self.write_f32(w, f)?;
                            write!(w, ")*")?;
                        }

                        (1.0, true) => write!(w, " + ")?,
                        (-1.0, true) => write!(w, " - ")?,
                        (f, true) if f > 0.0 => {
                            write!(w, " + float2(")?;
                            self.write_f32(w, f)?;
                            write!(w, ")*")?;
                        }
                        (f, true) if f < 0.0 => {
                            let f = -f;
                            write!(w, " - float2(")?;
                            self.write_f32(w, f)?;
                            write!(w, ")*")?;
                        }
                        _ => unreachable!("This match is complete across if conditions (unless NaN?)"),
                    }
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    self.write_vec2(w, addend, false)?;
                }
                let a0 = last_addend[0];
                let a1 = last_addend[1];
                if *last_addend != [0.0; 2] {
                    if len > 1 {
                        write!(w, " + ")?;
                    }
                    if a0 == a1 {
                        write!(w, "float2(")?;
                        self.write_f32(w, a0)?;
                        write!(w, ")")?;
                    } else {
                        write!(w, "float2(")?;
                        self.write_f32(w, a0)?;
                        write!(w, ", ")?;
                        self.write_f32(w, a1)?;
                        write!(w, ")")?;
                    }
                }
                if len > 1 && !grouping_provided {
                    write!(w, ")")?;
                }
            }
            Vec2Expr::SwizzleVec2(box v, i0, i1) => {
                match v {
                    Vec2Expr::Truncate3to2(box v3) => self.write_vec3(w, v3, false)?,
                    Vec2Expr::Truncate4to2(box v4) => self.write_vec4(w, v4, false)?,
                    _ => self.write_vec2(w, v, false)?,
                }
                let x = swizzle_term(i0)?;
                let y = swizzle_term(i1)?;
                write!(w, ".{x}{y}")?;
            },
            Vec2Expr::Truncate3to2(box v3) => {
                self.write_vec3(w, v3, false)?;
                write!(w, ".xy")?;
            }
            Vec2Expr::Truncate4to2(box v4) => {
                self.write_vec4(w, v4, false)?;
                write!(w, ".xy")?;
            }
        }
        Ok(())
    }

    fn write_vec3<W: Write>(&self, w: &mut W, expr: &Vec3Expr, grouping_provided: bool) -> anyhow::Result<()> {
        match expr {
            Vec3Expr::Variable(v) => {
                let name = &v.decl.name.0;
                let no = v.decl.name.1;
                if name.as_str() == "self" && no == 0 {
                    write!(w, "this")?;
                } else if no == 0 {
                    write!(w, "{name}")?;
                } else {
                    let no = no + 1;
                    write!(w, "{name}_{no}")?;
                }
            }
            Vec3Expr::Gather1(f) => {
                write!(w, "float3(")?;
                self.write_float(w, f, true)?;
                write!(w, ")")?;
            }
            Vec3Expr::Gather3(f0, f1, f2) => {
                write!(w, "float3(")?;
                self.write_float(w, f0, true)?;
                write!(w, ", ")?;
                self.write_float(w, f1, true)?;
                write!(w, ", ")?;
                self.write_float(w, f2, true)?;
                write!(w, ")")?;
            }
            Vec3Expr::Extend2to3(v2, f1) => {
                write!(w, "float3(")?;
                self.write_vec2(w, v2, false)?;
                write!(w, ", ")?;
                self.write_float(w, f1, true)?;
                write!(w, ")")?;
            }
            Vec3Expr::AccessMultiVecGroup(mv, i) => {
                self.write_multi_vec(w, mv)?;
                write!(w, ".group{i}")?;
            }
            Vec3Expr::Product(v, last_factor) => {
                let has_last_factor = *last_factor != [1.0; 3];
                if v.is_empty() && !has_last_factor {
                    bail!("Attempted to write an empty product that should have been simplified");
                }
                let mut len = v.len();
                if has_last_factor {
                    len += 1;
                }
                if len > 1 && !grouping_provided {
                    write!(w, "(")?;
                }
                for (i, (factor, exponent)) in v.iter().enumerate() {
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    match (*exponent, i > 0) {
                        (f, _) if f == 0.0 => continue,

                        (1.0, false) => self.write_vec3(w, factor, false)?,
                        (-1.0, false) => {
                            write!(w, "(float3(1.0) / ")?;
                            self.write_vec3(w, factor, false)?;
                            write!(w, ")")?;
                        }
                        (e, false) => {
                            if e.fract() == 0.0 && e <= i32::MAX as f32 && e >= i32::MIN as f32 {
                                let e = e as i32;
                                write!(w, "powi(")?;
                                self.write_vec3(w, factor, true)?;
                                write!(w, ", {e})")?;
                            } else {
                                write!(w, "powf(")?;
                                self.write_vec3(w, factor, true)?;
                                write!(w, ", {e})")?;
                            }
                        }

                        (1.0, true) => {
                            write!(w, " * ")?;
                            self.write_vec3(w, factor, false)?
                        }
                        (-1.0, true) => {
                            write!(w, " / (")?;
                            self.write_vec3(w, factor, true)?;
                            write!(w, ")")?;
                        }
                        (e, true) => {
                            if e.fract() == 0.0 && e <= i32::MAX as f32 && e >= i32::MIN as f32 {
                                let e = e as i32;
                                write!(w, " * powi(")?;
                                self.write_vec3(w, factor, true)?;
                                write!(w, ", {e})")?;
                            } else {
                                write!(w, " * powf(")?;
                                self.write_vec3(w, factor, true)?;
                                write!(w, ", {e})")?;
                            }
                        }
                    }
                }
                if *last_factor != [1.0; 3] {
                    if len > 1 {
                        write!(w, " * ")?;
                    }
                    let a = last_factor[0];
                    let b = last_factor[1];
                    let c = last_factor[2];
                    if a == b && b == c {
                        write!(w, "float3(")?;
                        self.write_f32(w, a)?;
                        write!(w, ")")?;
                    } else {
                        write!(w, "float3(")?;
                        self.write_f32(w, a)?;
                        write!(w, ", ")?;
                        self.write_f32(w, b)?;
                        write!(w, ", ")?;
                        self.write_f32(w, c)?;
                        write!(w, ")")?;
                    }
                }
                if len > 1 && !grouping_provided {
                    write!(w, ")")?;
                }
            }
            Vec3Expr::Sum(v, last_addend) => {
                let has_last_addend = *last_addend != [0.0; 3];
                if v.is_empty() && !has_last_addend {
                    bail!("Attempted to write an empty sum that should have been simplified");
                }
                let mut len = v.len();
                if has_last_addend {
                    len += 1;
                }
                if len > 1 && !grouping_provided {
                    write!(w, "(")?;
                }
                for (i, (addend, factor)) in v.iter().enumerate() {
                    match (*factor, i > 0) {
                        (f, _) if f == 0.0 => continue,

                        (1.0, false) => {}
                        (-1.0, false) => write!(w, "-")?,
                        (f, false) => {
                            write!(w, "float3(")?;
                            self.write_f32(w, f)?;
                            write!(w, ")*")?;
                        }

                        (1.0, true) => write!(w, " + ")?,
                        (-1.0, true) => write!(w, " - ")?,
                        (f, true) if f > 0.0 => {
                            write!(w, " + float3(")?;
                            self.write_f32(w, f)?;
                            write!(w, ")*")?;
                        }
                        (f, true) if f < 0.0 => {
                            let f = -f;
                            write!(w, " - float3(")?;
                            self.write_f32(w, f)?;
                            write!(w, ")*")?;
                        }
                        _ => unreachable!("This match is complete across if conditions (unless NaN?)"),
                    }
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    self.write_vec3(w, addend, false)?;
                }
                if *last_addend != [0.0; 3] {
                    if len > 1 {
                        write!(w, " + ")?;
                    }
                    let a = last_addend[0];
                    let b = last_addend[1];
                    let c = last_addend[2];
                    if a == b && b == c {
                        write!(w, "float3(")?;
                        self.write_f32(w, a)?;
                        write!(w, ")")?;
                    } else {
                        write!(w, "float3(")?;
                        self.write_f32(w, a)?;
                        write!(w, ", ")?;
                        self.write_f32(w, b)?;
                        write!(w, ", ")?;
                        self.write_f32(w, c)?;
                        write!(w, ")")?;
                    }
                }
                if len > 1 && !grouping_provided {
                    write!(w, ")")?;
                }
            }
            Vec3Expr::SwizzleVec3(box v, i0, i1, i2) => {
                match v {
                    Vec3Expr::Truncate4to3(box v4) => self.write_vec4(w, v4, false)?,
                    _ => self.write_vec3(w, v, false)?,
                }
                let x = swizzle_term(i0)?;
                let y = swizzle_term(i1)?;
                let z = swizzle_term(i2)?;
                write!(w, ".{x}{y}{z}")?;
            },
            Vec3Expr::Truncate4to3(box v4) => {
                self.write_vec4(w, v4, false)?;
                write!(w, ".xyz")?;
            }
        }
        Ok(())
    }

    fn write_vec4<W: Write>(&self, w: &mut W, expr: &Vec4Expr, grouping_provided: bool) -> anyhow::Result<()> {
        match expr {
            Vec4Expr::Variable(v) => {
                let name = &v.decl.name.0;
                let no = v.decl.name.1;
                if name.as_str() == "self" && no == 0 {
                    write!(w, "this")?;
                } else if no == 0 {
                    write!(w, "{name}")?;
                } else {
                    let no = no + 1;
                    write!(w, "{name}_{no}")?;
                }
            }
            Vec4Expr::Gather1(f) => {
                write!(w, "float4(")?;
                self.write_float(w, f, true)?;
                write!(w, ")")?;
            }
            Vec4Expr::Gather4(f0, f1, f2, f3) => {
                write!(w, "float4(")?;
                self.write_float(w, f0, true)?;
                write!(w, ", ")?;
                self.write_float(w, f1, true)?;
                write!(w, ", ")?;
                self.write_float(w, f2, true)?;
                write!(w, ", ")?;
                self.write_float(w, f3, true)?;
                write!(w, ")")?;
            }
            Vec4Expr::Extend2to4(v2, f1, f2) => {
                write!(w, "float4(")?;
                self.write_vec2(w, v2, false)?;
                write!(w, ", ")?;
                self.write_float(w, f1, true)?;
                write!(w, ", ")?;
                self.write_float(w, f2, true)?;
                write!(w, ")")?;
            }
            Vec4Expr::Extend3to4(v3, f1) => {
                write!(w, "float4(")?;
                self.write_vec3(w, v3, false)?;
                write!(w, ", ")?;
                self.write_float(w, f1, true)?;
                write!(w, ")")?;
            }
            Vec4Expr::AccessMultiVecGroup(mv, i) => {
                self.write_multi_vec(w, mv)?;
                write!(w, ".group{i}")?;
            }
            Vec4Expr::Product(v, last_factor) => {
                let has_last_factor = *last_factor != [1.0; 4];
                if v.is_empty() && !has_last_factor {
                    bail!("Attempted to write an empty product that should have been simplified");
                }
                let mut len = v.len();
                if has_last_factor {
                    len += 1;
                }
                if len > 1 && !grouping_provided {
                    write!(w, "(")?;
                }
                for (i, (factor, exponent)) in v.iter().enumerate() {
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    match (*exponent, i > 0) {
                        (f, _) if f == 0.0 => continue,

                        (1.0, false) => self.write_vec4(w, factor, false)?,
                        (-1.0, false) => {
                            write!(w, "(float4(1.0) / ")?;
                            self.write_vec4(w, factor, false)?;
                            write!(w, ")")?;
                        }
                        (e, false) => {
                            if e.fract() == 0.0 && e <= i32::MAX as f32 && e >= i32::MIN as f32 {
                                let e = e as i32;
                                write!(w, "powi(")?;
                                self.write_vec4(w, factor, true)?;
                                write!(w, ", {e})")?;
                            } else {
                                write!(w, "powf(")?;
                                self.write_vec4(w, factor, true)?;
                                write!(w, ", {e})")?;
                            }
                        }

                        (1.0, true) => {
                            write!(w, " * ")?;
                            self.write_vec4(w, factor, false)?
                        }
                        (-1.0, true) => {
                            write!(w, " / (")?;
                            self.write_vec4(w, factor, true)?;
                            write!(w, ")")?;
                        }
                        (e, true) => {
                            if e.fract() == 0.0 && e <= i32::MAX as f32 && e >= i32::MIN as f32 {
                                let e = e as i32;
                                write!(w, " * powi(")?;
                                self.write_vec4(w, factor, true)?;
                                write!(w, ", {e})")?;
                            } else {
                                write!(w, " * powf(")?;
                                self.write_vec4(w, factor, true)?;
                                write!(w, ", {e})")?;
                            }
                        }
                    }
                }
                if *last_factor != [1.0; 4] {
                    if len > 1 {
                        write!(w, " * ")?;
                    }
                    let a = last_factor[0];
                    let b = last_factor[1];
                    let c = last_factor[2];
                    let d = last_factor[3];
                    if a == b && b == c && c == d {
                        write!(w, "float4(")?;
                        self.write_f32(w, a)?;
                        write!(w, ")")?;
                    } else {
                        write!(w, "float4(")?;
                        self.write_f32(w, a)?;
                        write!(w, ", ")?;
                        self.write_f32(w, b)?;
                        write!(w, ", ")?;
                        self.write_f32(w, c)?;
                        write!(w, ", ")?;
                        self.write_f32(w, d)?;
                        write!(w, ")")?;
                    }
                }
                if len > 1 && !grouping_provided {
                    write!(w, ")")?;
                }
            }
            Vec4Expr::Sum(v, last_addend) => {
                let has_last_addend = *last_addend != [0.0; 4];
                if v.is_empty() && !has_last_addend {
                    bail!("Attempted to write an empty sum that should have been simplified");
                }
                let mut len = v.len();
                if has_last_addend {
                    len += 1;
                }
                if len > 1 && !grouping_provided {
                    write!(w, "(")?;
                }
                for (i, (addend, factor)) in v.iter().enumerate() {
                    match (*factor, i > 0) {
                        (f, _) if f == 0.0 => continue,

                        (1.0, false) => {}
                        (-1.0, false) => write!(w, "-")?,
                        (f, false) => {
                            write!(w, "float4(")?;
                            self.write_f32(w, f)?;
                            write!(w, ")*")?;
                        }

                        (1.0, true) => write!(w, " + ")?,
                        (-1.0, true) => write!(w, " - ")?,
                        (f, true) if f > 0.0 => {
                            write!(w, " + float4(")?;
                            self.write_f32(w, f)?;
                            write!(w, ")*")?;
                        }
                        (f, true) if f < 0.0 => {
                            let f = -f;
                            write!(w, " - float4(")?;
                            self.write_f32(w, f)?;
                            write!(w, ")*")?;
                        }
                        _ => unreachable!("This match is complete across if conditions (unless NaN?)"),
                    }
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    self.write_vec4(w, addend, false)?;
                }
                if *last_addend != [0.0; 4] {
                    if len > 1 {
                        write!(w, " + ")?;
                    }
                    let a = last_addend[0];
                    let b = last_addend[1];
                    let c = last_addend[2];
                    let d = last_addend[3];
                    if a == b && b == c && c == d {
                        write!(w, "float4(")?;
                        self.write_f32(w, a)?;
                        write!(w, ")")?;
                    } else {
                        write!(w, "float4(")?;
                        self.write_f32(w, a)?;
                        write!(w, ", ")?;
                        self.write_f32(w, b)?;
                        write!(w, ", ")?;
                        self.write_f32(w, c)?;
                        write!(w, ", ")?;
                        self.write_f32(w, d)?;
                        write!(w, ")")?;
                    }
                }
                if len > 1 && !grouping_provided {
                    write!(w, ")")?;
                }
            }
            Vec4Expr::SwizzleVec4(box v, i0, i1, i2, i3) => {
                self.write_vec4(w, v, false)?;
                let x = swizzle_term(i0)?;
                let y = swizzle_term(i1)?;
                let z = swizzle_term(i2)?;
                let w2 = swizzle_term(i3)?;
                write!(w, ".{x}{y}{z}{w2}")?;
            }
        }
        Ok(())
    }

    fn write_multi_vec<W: Write>(&self, w: &mut W, expr: &MultiVectorExpr) -> anyhow::Result<()> {
        let mv = expr.mv_class;
        match &*expr.expr {
            MultiVectorVia::Variable(v) => {
                let name = &v.decl.name.0;
                let no = v.decl.name.1;
                if name.as_str() == "self" && no == 0 {
                    write!(w, "this")?;
                } else if no == 0 {
                    write!(w, "{name}")?;
                } else {
                    let no = no + 1;
                    write!(w, "{name}_{no}")?;
                }
            }
            MultiVectorVia::Construct(v) => {
                let n = mv.name();
                // TODO
                write!(w, "{n}.from_groups(")?;
                let groups = mv.groups();
                for (i, g) in v.iter().enumerate() {
                    if i > 0 {
                        write!(w, ", ")?;
                    }
                    write!(w, "\n            /* ")?;
                    for (i, el) in groups[i].into_vec().into_iter().enumerate() {
                        if i > 0 {
                            write!(w, ", ")?;
                        }
                        write!(w, "{el}")?;
                    }
                    write!(w, " */\n            ")?;
                    match g {
                        MultiVectorGroupExpr::JustFloat(f) => self.write_float(w, f, true)?,
                        MultiVectorGroupExpr::Vec2(g) => self.write_vec2(w, g, true)?,
                        MultiVectorGroupExpr::Vec3(g) => self.write_vec3(w, g, true)?,
                        MultiVectorGroupExpr::Vec4(g) => self.write_vec4(w, g, true)?,
                    }
                }
                write!(w, "\n        )")?;
            }
            MultiVectorVia::TraitInvoke11ToClass(t, arg) => {
                let method = t.as_lower_snake();
                // TODO fancy infix can conflict with variable names
                match (&self.fancy_infix, self.prefer_fancy_infix) {
                    (Some(infix), true) => {
                        let op = infix.rust_operator();
                        write!(w, "(")?;
                        self.write_multi_vec(w, arg)?;
                        write!(w, " {op}{method}")?;
                        write!(w, ")")?;
                    }
                    _ => {
                        self.write_multi_vec(w, arg)?;
                        write!(w, ".{method}()")?;
                    }
                }
            }
            MultiVectorVia::TraitInvoke21ToClass(t, arg, mv) => {
                self.write_multi_vec(w, arg)?;
                let method = t.as_lower_snake();
                let b = mv.name();
                // TODO
                write!(w, ".{method}::<{b}>()")?;
            }
            MultiVectorVia::TraitInvoke22ToClass(t, a, b) => {
                let method = t.as_lower_snake();
                // TODO fancy infix can conflict with variable names
                match (&self.fancy_infix, self.prefer_fancy_infix) {
                    (Some(infix), true) => {
                        let op = infix.rust_operator();
                        write!(w, "(")?;
                        self.write_multi_vec(w, a)?;
                        write!(w, " {op}")?;
                        write!(w, "{method}")?;
                        write!(w, "{op} ")?;
                        self.write_multi_vec(w, b)?;
                        write!(w, ")")?;
                    }
                    _ => {
                        self.write_multi_vec(w, a)?;
                        write!(w, ".{method}(")?;
                        self.write_multi_vec(w, b)?;
                        write!(w, ")")?;
                    }
                }
            }
            MultiVectorVia::TraitInvoke12iToClass(t, a, b) => {
                let method = t.as_lower_snake();
                // TODO fancy infix can conflict with variable names
                match (&self.fancy_infix, self.prefer_fancy_infix) {
                    (Some(infix), true) => {
                        let op = infix.rust_operator();
                        write!(w, "(")?;
                        self.write_multi_vec(w, a)?;
                        write!(w, " {op}")?;
                        write!(w, "{method}")?;
                        write!(w, "{op} ")?;
                        self.write_int(w, b)?;
                        write!(w, ")")?;
                    }
                    _ => {
                        self.write_multi_vec(w, a)?;
                        write!(w, ".{method}(")?;
                        self.write_int(w, b)?;
                        write!(w, ")")?;
                    }
                }
            }
            MultiVectorVia::TraitInvoke12fToClass(t, a, b) => {
                let method = t.as_lower_snake();
                // TODO fancy infix can conflict with variable names
                match (&self.fancy_infix, self.prefer_fancy_infix) {
                    (Some(infix), true) => {
                        let op = infix.rust_operator();
                        write!(w, "(")?;
                        self.write_multi_vec(w, a)?;
                        write!(w, " {op}")?;
                        write!(w, "{method}")?;
                        write!(w, "{op} ")?;
                        self.write_float(w, b, false)?;
                        write!(w, ")")?;
                    }
                    _ => {
                        self.write_multi_vec(w, a)?;
                        write!(w, ".{method}(")?;
                        self.write_float(w, b, true)?;
                        write!(w, ")")?;
                    }
                }
            }
        }
        Ok(())
    }

    fn write_trait_from<W: Write>(&self, w: &mut W, impls: Arc<RawTraitImplementation>) -> anyhow::Result<()> {
        // TODO
        let ExpressionType::Class(other) = impls.owner else {
            bail!("Owner of Into (Other of From) impl is not a MultiVector")
        };
        let Some(ExpressionType::Class(owner)) = impls.other_type_params.get(0) else {
            bail!("Other of Into (Owner of From) impl is not a MultiVector")
        };
        let other = other.name();
        let owner = owner.name();
        let lsc = TraitKey::new(other).as_lower_snake();
        let lsc = format!("from_{lsc}");
        writeln!(
            w,
            r#"
impl From<{other}> for {owner} {{
    fn from({lsc}: {other}) -> Self {{"#
        )?;
        if impls.statistics.basis_element_struct_access {
            writeln!(w, "        use crate::elements::*;")?;
        }
        let mut ret = impls.return_expr.clone();
        let old_var = Arc::new(RawVariableDeclaration {
            comment: None,
            name: ("self".to_string(), 0),
            expr: None,
        });
        let new_var = Arc::new(RawVariableDeclaration {
            comment: None,
            name: (lsc, 0),
            expr: None,
        });
        ret.substitute_variable(old_var, new_var);
        write!(w, "        return ")?;
        self.write_expression(w, &ret, true)?;
        writeln!(w, ";\n    }}\n}}")?;
        Ok(())
    }

    fn write_trait_try_from<W: Write>(&self, w: &mut W, impls: Arc<RawTraitImplementation>) -> anyhow::Result<()> {
        // TODO
        let ExpressionType::Class(other) = impls.owner else {
            bail!("Owner of Into (Other of From) impl is not a MultiVector")
        };
        let Some(ExpressionType::Class(owner)) = impls.other_type_params.get(0) else {
            bail!("Other of TryInto (Owner of TryFrom) impl is not a MultiVector")
        };
        let destination_elements: BTreeSet<_> = owner.elements().into_iter().collect();
        let misfit_elements: Vec<_> = other.elements().into_iter().enumerate().filter(|(_, el)| !destination_elements.contains(el)).collect();
        let other = other.name();
        let owner = owner.name();
        let lsc = TraitKey::new(other).as_lower_snake();
        write!(
            w,
            r#"
impl TryFrom<{other}> for {owner} {{
    type Error = String;
    fn try_from({lsc}: {other}) -> Result<Self, Self::Error> {{"#
        )?;
        if impls.statistics.basis_element_struct_access {
            writeln!(w, "        use crate::elements::*;")?;
        }
        let mut ret = impls.return_expr.clone();
        let old_var = Arc::new(RawVariableDeclaration {
            comment: None,
            name: ("self".to_string(), 0),
            expr: None,
        });
        let new_var = Arc::new(RawVariableDeclaration {
            comment: None,
            name: (lsc.clone(), 0),
            expr: None,
        });
        ret.substitute_variable(old_var, new_var);
        writeln!(w, "        let mut error_string = String::new();")?;
        write!(w, "        let mut fail = false;")?;
        for (i, el) in misfit_elements {
            write!(
                w,
                r#"
        let el = {lsc}[{i}];
        if el != 0.0 {{
            fail = true;
            error_string.push_str("{el}: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }}"#
            )?;
        }
        write!(
            w,
            r#"
        if fail {{
            let mut error = "Elements from {other} do not fit into {owner} {{ ".to_string();
            error.push_str(error_string.as_str());
            error.push('}}');
            return Err(error);
        }}
        return Ok("#
        )?;
        self.write_expression(w, &ret, true)?;
        writeln!(w, ");\n    }}\n}}")?;
        Ok(())
    }

    #[allow(non_upper_case_globals)]
    fn declare_multi_vector<W: Write, const AntiScalar: BasisElement>(
        &self, w: &mut W, multi_vec: &'static MultiVec<AntiScalar>, docs: Option<String>
    ) -> anyhow::Result<()> {
        let name = TraitKey::new(multi_vec.name);
        let ucc = name.as_upper_camel();
        // let lcc = name.as_lower_camel();
        // let lsc = name.as_lower_snake();
        let ssc = name.as_screaming_snake();
        // TODO built in documentation, statistics, and traits that output this type
        let docs = docs.unwrap_or(ucc.clone());
        self.emit_comment(w, true, docs)?;

        // TODO hybrid rust/wgsl - only one struct, based on vecs, but with properties

        let name = TraitKey::new(multi_vec.name);
        let ucc = name.as_upper_camel();
        let lcc = name.as_lower_camel();
        writeln!(w, "public struct {ucc} {{")?;
        for (i, g) in multi_vec.groups().into_iter().enumerate() {
            if i > 0 {
                writeln!(w, ",")?;
            }
            write!(w, "    // ")?;
            let g = g.into_vec();
            for (i, el) in g.clone().into_iter().enumerate() {
                if i > 0 {
                    write!(w, ", ")?;
                }
                write!(w, "{el}")?;
            }
            let l = g.len();
            match l {
                1 => write!(w, ", 0, 0, 0")?,
                2 => write!(w, ", 0, 0")?,
                3 => write!(w, ", 0")?,
                _ => (),
            }
            // TODO consider modifying visibility of groups in rust implementation
            write!(w, "\n    internal group{i}: float4")?;
        }
        writeln!(w, "\n}}")?;



        writeln!(w, "extension {ucc} {{")?;
        for (outer_idx, g) in multi_vec.groups().into_iter().enumerate() {
            let g = g.into_vec();
            for (inner_idx, el) in g.clone().into_iter().enumerate() {
                // TODO consider nicer property access API in rust implementation
                writeln!(w, "    public property {el}: float {{")?;
                writeln!(w, "        get {{ return group{outer_idx}[{inner_idx}]; }}")?;
                writeln!(w, "        set {{ group{outer_idx}[{inner_idx}] = newValue; }}")?;
                writeln!(w, "    }}")?;
            }
        }

        writeln!(w, "    public static func from_elements(")?;
        for (i, el) in multi_vec.elements().into_iter().enumerate() {
            if i > 0 {
                write!(w, ", ")?;
            } else {
                write!(w, "        ")?;
            }
            write!(w, "{el}: float")?;
        }
        writeln!(w, "\n    ) -> {ucc} {{")?;
        write!(w, "        return {ucc} {{ ")?;
        for (outer_idx, g) in multi_vec.groups().into_iter().enumerate() {
            if outer_idx > 0 {
                write!(w, ", ")?;
            }
            write!(w, "float4(")?;
            let mut count = 0;
            for (inner_idx, el) in g.into_iter().enumerate() {
                if inner_idx > 0 {
                    write!(w, ", ")?;
                }
                write!(w, "{el}")?;
                count += 1;
            }
            while count < 4 {
                write!(w, ", 0.0")?;
                count += 1;
            }
            write!(w, ")")?;
        }
        writeln!(w, " }};")?;
        writeln!(w, "    }}")?;
        writeln!(w, "    internal static func from_groups(")?;
        for (i, g) in multi_vec.groups().into_iter().enumerate() {
            if i > 0 {
                write!(w, ", ")?;
            } else {
                write!(w, "        ")?;
            }
            write!(w, "g{i}: ")?;
            self.write_type(w, g.expr_type())?;
        }
        writeln!(w, "\n    ) -> {ucc} {{")?;
        writeln!(w, "        return {ucc} {{")?;
        for (i, _) in multi_vec.groups().into_iter().enumerate() {
            if i > 0 {
                write!(w, ", ")?;
            } else {
                write!(w, "            ")?;
            }
            write!(w, "group{i}: g{i}")?;
        }
        writeln!(w, "\n        }};\n    }}\n}}")?;


        writeln!(w, "extension {ucc}: IComparable {{")?;
        writeln!(w, "    bool lessThan(IComparable another) {{")?;
        writeln!(w, "        {ucc} other = ({ucc})another;")?;
        let len = multi_vec.groups().len();
        for (outer_idx, _) in multi_vec.groups().into_iter().enumerate() {
            if outer_idx < len - 1 {
                write!(w, "        if (this.group{outer_idx} != other.group{outer_idx})\n    ")?;
            }
            writeln!(w, "        return this.group{outer_idx}.lessThan(other.group{outer_idx});")?;
        }
        writeln!(w, "    }}")?;
        writeln!(w, "    bool equals(IComparable another) {{")?;
        writeln!(w, "        {ucc} other = ({ucc})another;")?;
        write!(w, "        return ")?;
        for (outer_idx, _) in multi_vec.groups().into_iter().enumerate() {
            if outer_idx > 0 {
                write!(w, " && ")?;
            }
            write!(w, "this.group{outer_idx}.equals(other.group{outer_idx})")?;
        }
        writeln!(w, ";\n    }}")?;
        writeln!(w, "}}")?;

        Ok(())
    }

    fn declare_trait_def<W: Write>(&self, w: &mut W, def: Arc<RawTraitDefinition>) -> anyhow::Result<()> {
        let ucc = def.names.trait_key.as_upper_camel();
        let lsc = def.names.trait_key.as_lower_snake();
        self.emit_comment(w, true, &def.documentation)?;
        // todo alias documentation
        write!(w, "public interface {ucc}")?;
        if let TraitArity::Two = def.arity {
            write!(w, "<T>")?;
        }
        writeln!(w, " {{")?;

        let output_ty = def.output.read();
        match *output_ty {
            TraitTypeConsensus::AlwaysSelf | TraitTypeConsensus::AllAgree(_, _) => {
                // We don't actually output it here
                // self.write_type(w, et)?;
            }
            TraitTypeConsensus::NoVotes | TraitTypeConsensus::Disagreement => {
                writeln!(w, "    associatedtype Output;")?;
            }
        }
        write!(w, "    func {lsc}(")?;
        match def.arity {
            TraitArity::Zero => {}
            TraitArity::One => {},
            TraitArity::Two => write!(w, "other: T")?,
        }
        write!(w, ") -> ")?;
        match *output_ty {
            TraitTypeConsensus::AlwaysSelf => write!(w, "Output")?,
            TraitTypeConsensus::AllAgree(et, _) => self.write_type(w, et)?,
            TraitTypeConsensus::NoVotes | TraitTypeConsensus::Disagreement => write!(w, "Output")?,
        }
        writeln!(w, ";\n}}")?;

        let op = *def.op.lock();
        if let Some(_op) = op {
            // TODO
        }

        if let Some(op) = &self.fancy_infix {
            writeln!(w, "public struct {lsc} {{}}")?;
            if let TraitArity::Two = def.arity {
                writeln!(w, "public struct {lsc}_partial<A> {{ a: A }}")?;
            }
            let operator_method = op.slang_trait_method();
            if let TraitArity::Two = def.arity {
                writeln!(w, "extension {lsc}_partial<A> for A: {ucc}<B> {{")?;
                // writeln!(w, "impl<A: {ucc}<B>, B> {operator_name}<B> for {lsc}_partial<A> {{")?;
                // write!(w, "    associatedtype Output = ")?;
                // match *output_ty {
                //     TraitTypeConsensus::AlwaysSelf => write!(w, "A.Output")?,
                //     TraitTypeConsensus::AllAgree(et, _) => self.write_type(w, et)?,
                //     TraitTypeConsensus::NoVotes | TraitTypeConsensus::Disagreement => write!(w, "A.Output")?,
                // }
                // writeln!(w, ";")?;
                writeln!(w, "    func {operator_method}(rhs: B) -> A.Output {{")?;
                writeln!(w, "        return this.a.{lsc}(rhs);")?;
                writeln!(w, "    }}\n}}")?;
            }
            if let TraitArity::One = def.arity {
                writeln!(w, "extension {lsc} for A: {ucc} {{")?;
                // writeln!(w, "impl<A: {ucc}> std::ops::{operator_name}<A> for {lsc} {{")?;
                // write!(w, "    associatedtype Output = ")?;
                // match *output_ty {
                //     TraitTypeConsensus::AlwaysSelf => write!(w, "A.Output")?,
                //     TraitTypeConsensus::AllAgree(et, _) => self.write_type(w, et)?,
                //     TraitTypeConsensus::NoVotes | TraitTypeConsensus::Disagreement => write!(w, "A.Output")?,
                // }
                // writeln!(w, ";")?;
                writeln!(w, "    func {operator_method}(rhs: A) -> A.Output {{")?;
                writeln!(w, "        return rhs.{lsc}();")?;
                writeln!(w, "    }}\n}}")?;
            }
        }

        Ok(())
    }

    pub(crate) fn declare_trait_impl<W: Write>(
        &self, w: &mut W, impls: Arc<RawTraitImplementation>, already_granted_infix: &mut BTreeSet<&'static str>
    ) -> anyhow::Result<()> {
        let def = &impls.definition;

        let output_ty = impls.return_expr.expression_type();
        let owner_ty = &impls.owner;
        if impls.other_var_params.len() > 1 || impls.other_type_params.len() > 1 {
            bail!("We do not support high arity traits yet");
        }

        let op = def.op.lock().clone();
        let ucc = def.names.trait_key.as_upper_camel();
        let mut lsc = def.names.trait_key.as_lower_snake();
        let mut do_assign_impl = false;
        let mut is_op = false;
        if let Some(op) = op {
            if op.rust_trait_name() == ucc.as_str() {
                is_op = true;
                lsc = op.slang_trait_method().to_string();
                do_assign_impl = def.arity == TraitArity::Two && *owner_ty == output_ty;
            }
        }
        let is_op = is_op;
        let lsc = lsc;
        let do_assign_impl = do_assign_impl;

        let mut var_param = None;
        if !impls.other_var_params.is_empty() {
            let v_param = &impls.other_var_params[0];
            if !impls.other_type_params.is_empty() {
                let ty_param = &impls.other_type_params[0];
                if ty_param != v_param {
                    // TODO I feel like this is a representation problem, need to review and maybe
                    //  refactor the algebraic data types involved here
                    bail!("Type of trait implementation does not agree");
                }
            }
            var_param = Some(v_param);
        }

        if let Some(op) = self.fancy_infix {
            let operator_method = op.rust_trait_method();
            if let TraitParam::Class(mv) = &owner_ty {
                let n = mv.name();
                if !is_op && !already_granted_infix.contains(n) {
                    already_granted_infix.insert(n);
                    if let TraitArity::Two = def.arity {
                        writeln!(w, "extension {n} {{")?;
                        writeln!(w, "    func {operator_method}(_rhs: {lsc}) -> {lsc}_partial<{n}> {{")?;
                        writeln!(w, "        {lsc}_partial(this)")?;
                        writeln!(w, "    }}\n}}")?;
                    }
                    if let TraitArity::One = def.arity {
                        writeln!(w, "extension {n} {{")?;
                        writeln!(w, "    func {operator_method}(_rhs: {lsc}) -> ")?;
                        self.write_type(w, output_ty)?;
                        writeln!(w, " {{\n        this.{lsc}()")?;
                        writeln!(w, "    }}\n}}")?;
                        if &output_ty == owner_ty {
                            writeln!(w, "extension {n} {{")?;
                            writeln!(w, "    func {operator_method}=(const {lsc}& _rhs) -> {n}& {{")?;
                            writeln!(w, "        this = this.{lsc}()")?;
                            writeln!(w, "        return *this;")?;
                            writeln!(w, "    }}\n}}")?;
                        }
                    }
                }
            }
        }

        // todo alias documentation
        write!(w, "extension ")?;
        self.write_type(w, *owner_ty)?;
        write!(w, ": {ucc}")?;
        if let (TraitArity::Two, Some(var_param)) = (def.arity, var_param) {
            write!(w, "<")?;
            self.write_type(w, *var_param)?;
            write!(w, ">")?;
        }
        writeln!(w, " {{")?;
        write!(w, "    func {lsc}(")?;
        match (def.arity, var_param) {
            (TraitArity::Zero, _) => {}
            (TraitArity::One, _) => {},
            (TraitArity::Two, Some(other_ty)) => {
                write!(w, "other: ")?;
                self.write_type(w, *other_ty)?;
            }
            _ => panic!("Arity 2 should always have other type"),
        }
        write!(w, ") -> ")?;
        self.write_type(w, output_ty)?;
        writeln!(w, " {{")?;
        for line in impls.lines.iter() {
            match line {
                CommentOrVariableDeclaration::Comment(c) => {
                    self.emit_comment(w, false, c.to_string())?;
                }
                CommentOrVariableDeclaration::VarDec(var_dec) => {
                    let Some(var_dec) = var_dec.upgrade() else { continue };
                    let Some(expr) = &var_dec.expr else { continue };
                    if let Some(c) = &var_dec.comment {
                        self.emit_comment(w, false, c.to_string())?;
                    }
                    let name = var_dec.name.0.to_string();
                    let no = var_dec.name.1;
                    let expr = expr.read();
                    let expr = expr.deref();
                    self.write_type(w, expr.expression_type())?;
                    if no == 0 {
                        write!(w, " {name} = ")?;
                    } else {
                        let no = no + 1;
                        write!(w, " {name}_{no} = ")?;
                    }
                    self.write_expression(w, expr, true)?;
                    writeln!(w, ";")?;
                }
            }
        }
        if let Some(c) = &impls.return_comment {
            self.emit_comment(w, false, c.to_string())?;
        }
        write!(w, "        return ")?;
        self.write_expression(w, &impls.return_expr, true)?;
        writeln!(w, ";")?;
        writeln!(w, "    }}\n}}")?;

        if !do_assign_impl {
            return Ok(());
        }

        write!(w, "extension ")?;
        self.write_type(w, *owner_ty)?;
        writeln!(w, " {{")?;
        write!(w, "    func {lsc}=(")?;
        match (def.arity, var_param) {
            (TraitArity::Zero, _) => {}
            (TraitArity::One, _) => {},
            (TraitArity::Two, Some(other_ty)) => {
                write!(w, "const ")?;
                self.write_type(w, *other_ty)?;
                write!(w, "& other")?;
            }
            _ => panic!("Arity 2 should always have other type"),
        }
        writeln!(w, ") {{")?;
        for line in impls.lines.iter() {
            match line {
                CommentOrVariableDeclaration::Comment(c) => {
                    self.emit_comment(w, false, c.to_string())?;
                }
                CommentOrVariableDeclaration::VarDec(var_dec) => {
                    let Some(var_dec) = var_dec.upgrade() else { continue };
                    let Some(expr) = &var_dec.expr else { continue };
                    if let Some(c) = &var_dec.comment {
                        self.emit_comment(w, false, c.to_string())?;
                    }
                    let name = var_dec.name.0.to_string();
                    let no = var_dec.name.1;
                    let expr = expr.read();
                    let expr = expr.deref();
                    self.write_type(w, expr.expression_type())?;
                    if no == 0 {
                        write!(w, " {name} = ")?;
                    } else {
                        let no = no + 1;
                        write!(w, " {name}_{no} = ")?;
                    }
                    self.write_expression(w, expr, true)?;
                    writeln!(w, ";")?;
                }
            }
        }
        if let Some(c) = &impls.return_comment {
            self.emit_comment(w, false, c.to_string())?;
        }
        write!(w, "        *this = ")?;
        self.write_expression(w, &impls.return_expr, true)?;
        writeln!(w, ";")?;
        writeln!(w, "    }}\n}}")?;

        Ok(())
    }

    fn emit_comment<W: Write, S: Into<String>>(&self, w: &mut W, is_documentation: bool, s: S) -> anyhow::Result<()> {
        let slashy = if is_documentation { "/// " } else { "// " };
        let s = s.into();
        let comment = s.trim();
        if comment.is_empty() {
            writeln!(w, "\n{slashy}")?;
            return Ok(());
        }
        let mut comment_iter = comment.split("\n").map(|it| it.trim()).skip_while(|it| it.is_empty()).peekable();
        writeln!(w)?;
        while let Some(line) = comment_iter.next() {
            if line.is_empty() {
                if let Some(next_line) = comment_iter.peek() {
                    if !next_line.is_empty() {
                        writeln!(w, "{slashy}{next_line}")?;
                    }
                }
            } else {
                writeln!(w, "{slashy}{line}")?;
            }
        }
        Ok(())
    }
}

fn swizzle_term(idx: &u8) -> anyhow::Result<&'static str> {
    match *idx {
        0 => Ok("x"),
        1 => Ok("y"),
        2 => Ok("z"),
        3 => Ok("w"),
        _ => bail!("swizzle index out of bounds")
    }
}
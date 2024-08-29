use anyhow::{anyhow, bail, Context, Error};
use indicatif::ProgressFinish;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::io::{BufRead, BufReader, ErrorKind, Read, Write};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;
use std::time::Duration;
use std::{fs, thread};
use tokio::task::JoinSet;

use crate::algebra2::basis::grades::{plane_based_k_reflections, point_based_k_reflections};
use crate::algebra2::basis::BasisElement;
use crate::algebra2::multivector::{MultiVec, MultiVecRepository};
use crate::ast2::datatype::{ExpressionType, MultiVector};
use crate::ast2::expressions::{AnyExpression, FloatExpr, IntExpr, MultiVectorExpr, MultiVectorGroupExpr, MultiVectorVia, Vec2Expr, Vec3Expr, Vec4Expr};
use crate::ast2::traits::{
    progress_style, BinaryOps, CommentOrVariableDeclaration, RawTraitDefinition, RawTraitImplementation, TraitArity, TraitImplRegistry, TraitKey, TraitParam, TraitTypeConsensus,
};
use crate::ast2::RawVariableDeclaration;
use crate::emit2::sort_trait_impls;
use crate::utility::CollectResults;
use crate::SIMD_SRC;

/// Generate a Rust project.
#[derive(Copy, Clone)]
pub struct Rust {
    /// Required to distinguish meet, join, and k-reflection modules.
    /// True means that grade 1 vectors are points.
    /// False means that grade 1 vectors are hyper-planes.
    pub point_based: bool,

    // Major features
    /// Generate rust-to-wgsl integration.
    /// TODO this should depend on WGSL language generation
    /// Dependencies added: naga, naga_oil, anyhow, bytemuck, encase
    pub wgsl: bool,

    /// Generate rust-to-glsl integration.
    /// TODO this should depend on GLSL language generation
    /// Dependencies added: naga, naga_oil, anyhow, bytemuck, encase
    pub glsl: bool,

    /// Generate rust-to-sql integration.
    /// TODO this should depend on SQL language generation
    /// Dependencies added: pgx, postgres-types
    pub sql: bool,

    /// Generate Eq, Ord, and Hash implementations for the MultiVector types.
    /// Dependencies added: float-ord
    pub eq_ord_hash: bool,

    /// Generate "NearEq" and "NearOrd" implementations for the MultiVector types.
    /// Dependencies added: nearly
    pub nearly_eq_ord: bool,

    /// Generate "Serialize" and "Deserialize" implementations for the MultiVector types.
    /// Dependencies added: serde
    pub serde: bool,

    // Small aesthetic features
    pub prefer_fancy_infix: bool,
    pub censor_grades: bool,

    // Internal/private stuff
    fancy_infix: Option<BinaryOps>,
}

impl Rust {
    pub fn new(point_based: bool) -> Self {
        Rust {
            point_based,

            wgsl: false,
            glsl: false,
            sql: false,
            eq_ord_hash: false,
            nearly_eq_ord: false,
            serde: false,

            prefer_fancy_infix: false,
            censor_grades: false,

            fancy_infix: None,
        }
    }
    pub fn all_features(mut self) -> Self {
        self.wgsl = true;
        self.glsl = true;
        self.sql = true;
        self.eq_ord_hash = true;
        self.nearly_eq_ord = true;
        self.serde = true;
        self
    }

    pub fn write_crate<P: AsRef<Path>, const AntiScalar: BasisElement>(
        self,
        crate_folder: P,
        algebra_name: &str,
        version_major: usize,
        version_minor: usize,
        version_patch: usize,
        version_pre: &str,
        description: &str,
        repository: &str,
        authors: &[&str],
        multi_vecs: Arc<MultiVecRepository<AntiScalar>>,
        impls: Arc<TraitImplRegistry>,
    ) {
        let file_path = crate_folder.as_ref().to_path_buf();
        let rt = tokio::runtime::Runtime::new().expect("tokio works");
        let e = rt.block_on(async move {
            self.write_cargo_toml(crate_folder, algebra_name, version_major, version_minor, version_patch, version_pre, description, repository, authors)
                .await?;
            self.write_src(file_path.join(Path::new("src")), multi_vecs, impls).await
        });
        if let Err(e) = e {
            panic!("Rust Errors: {e:?}");
        }
    }

    async fn write_cargo_toml<P: AsRef<Path>>(
        &self,
        crate_folder: P,
        algebra_name: &str,
        version_major: usize,
        version_minor: usize,
        version_patch: usize,
        version_pre: &str,
        description: &str,
        repository: &str,
        authors: &[&str],
    ) -> anyhow::Result<()> {
        let crate_folder = crate_folder.as_ref().to_path_buf();
        fs::create_dir_all(&crate_folder)?;
        let file_path = crate_folder.join(Path::new("Cargo.toml"));
        let mut file = fs::OpenOptions::new().write(true).create(true).truncate(true).open(&file_path)?;
        let mut pre = version_pre.to_string();
        if !pre.is_empty() {
            pre = format!("-{pre}");
        }
        // TODO look forward to rust edition 2024
        //  https://doc.rust-lang.org/edition-guide/rust-2024/index.html

        // TODO see if we can upgrade dependency versions or not (naga was sensitive, last I remember)

        let mut additional_authors = String::new();
        for a in authors {
            additional_authors.push_str(", \"");
            additional_authors.push_str(a);
            additional_authors.push('"');
        }

        write!(
            &mut file,
            r#"[package]
name = "{algebra_name}"
version = "{version_major}.{version_minor}.{version_patch}{pre}"
authors = ["Andrew Brown <Andrew.Brown.UNL@gmail.com>", "Alexander Meißner <AlexanderMeissner@gmx.net>"{additional_authors}]
description = "{description}"
repository = "{repository}"
keywords = ["math", "simd", "vector", "geometric-algebra", "geometry"]
license = "MIT"
edition = "2021"

[features]
default = []
wgsl = []
glsl = []
postgres = []

[dependencies]
"#
        )?;
        if self.wgsl || self.glsl {
            writeln!(
                &mut file,
                r#"naga_oil = {{ version =  "0.13.0", features = ["prune", "glsl"] }}
naga = "0.19.2"
anyhow = "1.0.86"
bytemuck = {{ version = "1.16.3", features = ["derive"] }}
encase = "0.9.0""#
            )?;
        }
        if self.nearly_eq_ord {
            writeln!(&mut file, r#"nearly = "0.4.0""#)?;
        }
        if self.eq_ord_hash {
            writeln!(&mut file, r#"float-ord = "0.3.2""#)?;
        }
        if self.sql {
            // TODO pgx seems to be a picky builder, so will need close inspection.
            writeln!(
                &mut file,
                r#"pgx = "0.7.4"
postgres-types = "0.2.7""#
            )?;
        }
        if self.serde {
            writeln!(&mut file, r#"serde = {{ version = "1.0.204", features = ["derive"] }}"#)?;
        }

        Ok(())
    }

    async fn write_src<P: AsRef<Path>, const AntiScalar: BasisElement>(
        mut self,
        src_folder: P,
        multi_vecs: Arc<MultiVecRepository<AntiScalar>>,
        impls: Arc<TraitImplRegistry>,
    ) -> anyhow::Result<()> {
        // Copilot Advice:
        //
        // Using the include! macro to pull in multiple large files into a single unifying file can impact
        // compile times, but it won't be due to the effort of concatenation. The include! macro essentially
        // inserts the content of the included files directly into the source file at the point of the macro
        // call, so the compiler treats it as if the code were written directly in the file
        // https://doc.rust-lang.org/std/macro.include.html.
        //
        // However, the overall compile time can be affected by the size and complexity of the generated
        // code. When you include many large files, the compiler has to process a significant amount of
        // code in a single compilation unit, which can slow down the compilation process. On the other hand,
        // using separate modules can help distribute the compilation workload, potentially allowing for
        // better parallelism and incremental compilation benefits
        // https://corrode.dev/blog/tips-for-faster-rust-compile-times/.
        //
        // In summary, while the include! macro itself doesn't add significant overhead, the way you
        // structure your code can impact compile times. If you find that compile times are becoming an
        // issue, you might want to consider organizing your code into separate modules to take advantage
        // of Rust's incremental compilation and parallel processing capabilities
        // https://corrode.dev/blog/tips-for-faster-rust-compile-times/.

        // So because of the above advice, we will give each MultiVec its own module after all.
        //
        // It just gets a little tedious to fully qualify my_trait_name::MyTraitName and my_vector::MyVector,
        // so we will re-export in a more convenient module.

        let src_folder = src_folder.as_ref().to_path_buf();
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
            let mut reader = BufReader::new(file);
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
            let mut file = fs::OpenOptions::new().write(true).create(true).truncate(true).open(&file_path)?;
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
        data_pb.set_message("Rust - Data Definitions");

        let qty_defs = defs.len() as u64;
        let trait_pb = Arc::new(multi_progress.add(indicatif::ProgressBar::new(qty_defs).with_finish(ProgressFinish::AndLeave)));
        trait_pb.set_style(progress_style());
        trait_pb.set_message("Rust - Trait Definitions");

        let qty_impls = impls.len() as u64;
        let impls_pb = Arc::new(multi_progress.add(indicatif::ProgressBar::new(qty_impls)));
        impls_pb.set_style(progress_style());
        impls_pb.set_message("Rust - Distributing Trait Implementations");

        let qty_files = qty_mvs + qty_defs + 4; // traits.rs, data.rs, lib.rs, simd.rs
        let fmt_pb = Arc::new(multi_progress.add(indicatif::ProgressBar::new(qty_files).with_finish(ProgressFinish::AndLeave)));
        fmt_pb.set_style(progress_style());
        fmt_pb.set_message("Rust - rustfmt");

        let (finished_file, mut rx) = tokio::sync::mpsc::unbounded_channel::<PathBuf>();
        let fmt_pb2 = fmt_pb.clone();
        join_set.spawn(async move {
            while let Some(p) = rx.recv().await {
                Self::format_file(p).await?;
                fmt_pb2.inc(1);
            }
            fmt_pb2.finish();
            Ok(())
        });

        // data definitions
        let mut data_mods: BTreeMap<String, Vec<_>> = BTreeMap::new();
        for multi_vec in mvs.iter() {
            let multi_vec = *multi_vec;
            let mv = MultiVector::from(multi_vec);
            let n = mv.name();
            let lsc = TraitKey::new(n).as_lower_snake();
            let folder_data = folder_data.clone();
            let doc = mv_docs.get(&n.to_string()).cloned();
            let tx2 = started_file.clone();
            let tx3 = finished_file.clone();

            let pb2 = data_pb.clone();
            join_set.spawn(async move {
                let file_path = folder_data.join(Path::new(&lsc)).with_extension("rs");
                tx2.send(file_path.clone())?;
                let mut file = fs::OpenOptions::new().write(true).create(true).truncate(true).open(&file_path)?;
                writeln!(&mut file, "use crate::data::*;")?;
                writeln!(&mut file, "use crate::simd::*;")?;
                self.declare_multi_vector(&mut file, multi_vec, doc)?;
                writeln!(&mut file, "include!(\"./impls/{lsc}.rs\");")?;
                tx3.send(file_path)?;
                pb2.inc(1);
                Ok(())
            });

            if let Some(gr) = mv.grade() {
                if !self.censor_grades && gr == 1 {
                    data_mods.entry("base".to_string()).and_modify(|v| v.push(multi_vec)).or_insert(vec![multi_vec]);
                }
                let as_gr = AntiScalar.grade();
                if gr > 0 && gr < as_gr {
                    let (m, j) = if self.point_based { ((as_gr - 1) - gr, gr - 1) } else { (gr - 1, (as_gr - 1) - gr) };
                    data_mods.entry(format!("join_{j}")).and_modify(|v| v.push(multi_vec)).or_insert(vec![multi_vec]);
                    data_mods.entry(format!("meet_{m}")).and_modify(|v| v.push(multi_vec)).or_insert(vec![multi_vec]);
                }
            }

            let gr = mv.grades();
            if !self.censor_grades {
                let d = AntiScalar.signature().bits().count_ones();
                let gr = gr.into_bits();
                let vec_name = if gr.count_ones() == 1 {
                    if d < 10 {
                        format!("vector_{}", gr.trailing_zeros())
                    } else {
                        format!("vector_{:02}", gr.trailing_zeros())
                    }
                } else {
                    "vector_mixed".to_string()
                };
                data_mods.entry(vec_name.to_string()).and_modify(|v| v.push(multi_vec)).or_insert(vec![multi_vec]);
            }
            let k_reflection = if self.point_based {
                point_based_k_reflections::<AntiScalar>()
            } else {
                plane_based_k_reflections()
            }
            .into_iter()
            .enumerate()
            .find(|(i, it)| it.contains(gr))
            .map(|(i, _)| i);
            if let Some(i) = k_reflection {
                data_mods.entry(format!("reflection_{i}")).and_modify(|v| v.push(multi_vec)).or_insert(vec![multi_vec]);
            }
        }

        // trait definitions
        let mut trait_mods: BTreeMap<String, Vec<_>> = BTreeMap::new();
        for td in defs.iter() {
            let td = td.clone();
            if let TraitTypeConsensus::NoVotes = *td.output.read() {
                trait_pb.inc(1);
                continue;
            }
            let k = td.names.trait_key;
            let n = k.as_upper_camel();
            let lsc = k.as_lower_snake();
            let arity = td.arity.as_str().to_string();
            trait_mods.entry(arity).and_modify(|v| v.push(td.clone())).or_insert(vec![td.clone()]);
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
            let tx3 = finished_file.clone();
            let pb2 = trait_pb.clone();
            join_set.spawn(async move {
                let file_path = folder_traits.join(Path::new(lsc.as_str())).with_extension("rs");
                tx2.send(file_path.clone())?;
                let mut file = fs::OpenOptions::new().write(true).create(true).truncate(true).open(&file_path)?;
                // TODO remove these imports when they are unused
                writeln!(&mut file, "use crate::data::*;")?;
                writeln!(&mut file, "use crate::simd::*;")?;
                self.declare_trait_def(&mut file, td)?;
                writeln!(&mut file, "include!(\"./impls/{lsc}.rs\");")?;
                tx3.send(file_path)?;
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
                .entry(format!("{folder}/impls/{name}.rs"))
                .and_modify(move |(v, deps)| {
                    for t_dep in i2.definition.dependencies.lock().iter().cloned() {
                        deps.insert(t_dep);
                    }
                    v.push(i2);
                })
                .or_insert_with(|| {
                    let mut t_deps: BTreeSet<_> = i.definition.dependencies.lock().iter().cloned().collect();
                    (vec![i], t_deps)
                });
            impls_pb.inc(1);
        }
        impls_pb.finish();
        fmt_pb.inc_length(impl_files.len() as u64);

        // trait implementations
        let (closed_types, mut closed_types_rx) = tokio::sync::mpsc::unbounded_channel::<String>();
        for (file_path, (mut impls, deps)) in impl_files {
            let src_folder = src_folder.clone();
            let tx2 = started_file.clone();
            let tx3 = finished_file.clone();
            let multi_progress = multi_progress.clone();
            let closed_types = closed_types.clone();
            join_set.spawn(async move {
                let file_path = src_folder.join(Path::new(file_path.as_str())).with_extension("rs");

                let qty_impls = impls.len() as u64;
                let qty_deps = deps.len() as u64;
                let mut pb = None;
                if qty_impls > 100 {
                    pb = Some(Arc::new(multi_progress.add(indicatif::ProgressBar::new(qty_impls + qty_deps + 2))));
                }
                if let Some(pb) = &pb {
                    pb.set_style(progress_style());
                    let fpd = file_path.display();
                    pb.set_message(format!("Rust - {fpd}"));
                }

                tx2.send(file_path.clone())?;
                let mut file = fs::OpenOptions::new().write(true).create(true).truncate(true).open(&file_path)?;
                let mut deps_set = HashSet::new();
                for dep in deps {
                    deps_set.insert(dep);
                    if self.prefer_fancy_infix {
                        let lsc = dep.as_lower_snake();
                        writeln!(&mut file, "use crate::traits::{lsc};")?;
                    } else {
                        let ucc = dep.as_upper_camel();
                        writeln!(&mut file, "use crate::traits::{ucc};")?;
                    }
                    if let Some(pb) = &pb {
                        pb.inc(1);
                    }
                }
                sort_trait_impls(&mut impls, deps_set)?;
                if let Some(pb) = &pb {
                    pb.inc(1);
                }

                writeln!(&mut file, "// Note on Operative Statistics: ")?;
                writeln!(&mut file, "// Operative Statistics are not a precise predictor of performance or performance comparisons. ")?;
                writeln!(&mut file, "// This is due to varying hardware capabilities and compiler optimizations. ")?;
                writeln!(&mut file, "// As always, where performance is a concern, there is no substitute for ")?;
                writeln!(&mut file, "// real measurements on real work-loads on real hardware.")?;
                writeln!(&mut file, "// Disclaimer aside, enjoy the fun information =)")?;

                if qty_impls > 1 {
                    let qty_impls = qty_impls as usize;
                    let mut add_w = vec![];
                    let mut mul_w = vec![];
                    let mut div_w = vec![];
                    let mut add_wo = vec![];
                    let mut mul_wo = vec![];
                    let mut div_wo = vec![];
                    for i in &impls {
                        let w = i.statistics.with_simd();
                        add_w.push(w.add_sub);
                        mul_w.push(w.mul);
                        div_w.push(w.div);
                        let wo = i.statistics.without_simd();
                        add_wo.push(wo.add_sub);
                        mul_wo.push(wo.mul);
                        div_wo.push(wo.div);
                    }
                    add_w.sort();
                    mul_w.sort();
                    div_w.sort();
                    add_wo.sort();
                    mul_wo.sort();
                    div_wo.sort();

                    let add_w_min = add_w.first().cloned().unwrap_or(0);
                    let add_w_max = add_w.last().cloned().unwrap_or(0);
                    let add_w_med = add_w.get(qty_impls / 2).cloned().unwrap_or(0);
                    let add_w_avg = add_w.into_iter().sum::<usize>() / qty_impls;

                    let mul_w_min = mul_w.first().cloned().unwrap_or(0);
                    let mul_w_max = mul_w.last().cloned().unwrap_or(0);
                    let mul_w_med = mul_w.get(qty_impls / 2).cloned().unwrap_or(0);
                    let mul_w_avg = mul_w.into_iter().sum::<usize>() / qty_impls;

                    let div_w_min = div_w.first().cloned().unwrap_or(0);
                    let div_w_max = div_w.last().cloned().unwrap_or(0);
                    let div_w_med = div_w.get(qty_impls / 2).cloned().unwrap_or(0);
                    let div_w_avg = div_w.into_iter().sum::<usize>() / qty_impls;

                    let add_wo_min = add_wo.first().cloned().unwrap_or(0);
                    let add_wo_max = add_wo.last().cloned().unwrap_or(0);
                    let add_wo_med = add_wo.get(qty_impls / 2).cloned().unwrap_or(0);
                    let add_wo_avg = add_wo.into_iter().sum::<usize>() / qty_impls;

                    let mul_wo_min = mul_wo.first().cloned().unwrap_or(0);
                    let mul_wo_max = mul_wo.last().cloned().unwrap_or(0);
                    let mul_wo_med = mul_wo.get(qty_impls / 2).cloned().unwrap_or(0);
                    let mul_wo_avg = mul_wo.into_iter().sum::<usize>() / qty_impls;

                    let div_wo_min = div_wo.first().cloned().unwrap_or(0);
                    let div_wo_max = div_wo.last().cloned().unwrap_or(0);
                    let div_wo_med = div_wo.get(qty_impls / 2).cloned().unwrap_or(0);
                    let div_wo_avg = div_wo.into_iter().sum::<usize>() / qty_impls;

                    writeln!(&mut file, "//\n// Total Implementations: {qty_impls}")?;
                    writeln!(&mut file, "//\n// Yes SIMD:   add/sub     mul     div")?;
                    writeln!(&mut file, "//  Minimum:   {add_w_min:>7} {mul_w_min:>7} {div_w_min:>7}")?;
                    writeln!(&mut file, "//   Median:   {add_w_med:>7} {mul_w_med:>7} {div_w_med:>7}")?;
                    writeln!(&mut file, "//  Average:   {add_w_avg:>7} {mul_w_avg:>7} {div_w_avg:>7}")?;
                    writeln!(&mut file, "//  Maximum:   {add_w_max:>7} {mul_w_max:>7} {div_w_max:>7}")?;
                    writeln!(&mut file, "//\n//  No SIMD:   add/sub     mul     div")?;
                    writeln!(&mut file, "//  Minimum:   {add_wo_min:>7} {mul_wo_min:>7} {div_wo_min:>7}")?;
                    writeln!(&mut file, "//   Median:   {add_wo_med:>7} {mul_wo_med:>7} {div_wo_med:>7}")?;
                    writeln!(&mut file, "//  Average:   {add_wo_avg:>7} {mul_wo_avg:>7} {div_wo_avg:>7}")?;
                    writeln!(&mut file, "//  Maximum:   {add_wo_max:>7} {mul_wo_max:>7} {div_wo_max:>7}")?;
                }

                // writeln!(&mut file, "use crate::data::*;")?;
                // writeln!(&mut file, "use crate::simd::*;")?;
                let mut already_granted_infix = BTreeSet::new();
                for i in impls {
                    let ucc = i.definition.names.trait_key.as_upper_camel();
                    let j = i.clone();
                    match ucc.as_str() {
                        "Into" => self.write_trait_from(&mut file, i),
                        "TryInto" => self.write_trait_try_from(&mut file, i),
                        _ => self.declare_trait_impl(&mut file, i, &mut already_granted_infix),
                    }?;
                    let i = j;
                    if let TraitArity::Two = i.definition.arity {
                        let owner = i.owner;
                        let other = i.other_type_params.first();
                        let output = i.return_expr.expression_type();
                        // TODO I could output information like this in a comment on the type itself. And trait definition.
                        match (owner, other, output) {
                            (TraitParam::Class(a), Some(TraitParam::Class(b)), ExpressionType::Class(c))
                                if a == *b && a == c && (ucc.as_str() == "GeometricProduct" || ucc.as_str() == "GeometricAntiProduct") =>
                            {
                                let n = a.name();
                                let msg = format!("{n} is closed under {ucc}");
                                closed_types.send(msg)?;
                            }
                            _ => {}
                        }
                    }
                    if let Some(pb) = &pb {
                        pb.inc(1);
                    }
                }
                tx3.send(file_path)?;
                if let Some(pb) = &pb {
                    pb.inc(1);
                    pb.finish_and_clear();
                }
                Ok(())
            });
        }

        // data.rs
        let src_folder2 = src_folder.clone();
        let tx2 = started_file.clone();
        let tx3 = finished_file.clone();
        join_set.spawn(async move {
            let file_path = src_folder2.join(Path::new("data.rs"));
            tx2.send(file_path.clone())?;
            let mut file = fs::OpenOptions::new().write(true).create(true).truncate(true).open(&file_path)?;
            for (the_mod, mvs) in data_mods {
                writeln!(&mut file, "pub mod {the_mod} {{")?;
                for mv in mvs {
                    let n = mv.name;
                    let lsc = TraitKey::new(n).as_lower_snake();
                    writeln!(&mut file, "pub use crate::data::{lsc}::{n};")?;
                }
                writeln!(&mut file, "}}")?;
            }
            for mv in mvs {
                let n = mv.name;
                let lsc = TraitKey::new(n).as_lower_snake();
                writeln!(&mut file, "mod {lsc};")?;
                writeln!(&mut file, "pub use {lsc}::{n};")?;
            }
            tx3.send(file_path)?;
            Ok(())
        });

        // traits.rs
        let src_folder2 = src_folder.clone();
        let tx2 = started_file.clone();
        let tx3 = finished_file.clone();
        join_set.spawn(async move {
            let file_path = src_folder2.join(Path::new("traits.rs"));
            tx2.send(file_path.clone())?;
            let mut file = fs::OpenOptions::new().write(true).create(true).truncate(true).open(&file_path)?;
            for (the_mod, ts) in trait_mods {
                writeln!(&mut file, "pub mod {the_mod} {{")?;
                for t in ts {
                    if let TraitTypeConsensus::NoVotes = *t.output.read() {
                        continue;
                    }
                    let n = t.names.trait_key;
                    let lsc = n.as_lower_snake();
                    let n = n.as_upper_camel();
                    match n.as_str() {
                        "Add" | "Sub" | "Mul" | "Div" | "Shl" | "Shr" | "BitAnd" | "BitOr" | "BitXor" | "Neg" | "Not" => continue,
                        "Into" | "TryInto" => continue,
                        _ => {}
                    }
                    writeln!(&mut file, "pub use crate::traits::{lsc}::{n};")?;
                }
                writeln!(&mut file, "}}")?;
            }
            if fancy_infix.is_some() {
                writeln!(&mut file, "pub mod infix {{")?;
                for td in defs.iter() {
                    if let TraitTypeConsensus::NoVotes = *td.output.read() {
                        continue;
                    }
                    let n = td.names.trait_key.as_upper_camel();
                    match n.as_str() {
                        "Add" | "Sub" | "Mul" | "Div" | "Shl" | "Shr" | "BitAnd" | "BitOr" | "BitXor" | "Neg" | "Not" => continue,
                        "Into" | "TryInto" => continue,
                        _ => {}
                    }
                    let lsc = td.names.trait_key.as_lower_snake();
                    match td.arity {
                        TraitArity::Zero => {}
                        TraitArity::One => {
                            writeln!(&mut file, "pub use crate::traits::{lsc}::{lsc};")?;
                        }
                        TraitArity::Two => {
                            writeln!(&mut file, "pub use crate::traits::{lsc}::{lsc};")?;
                            writeln!(&mut file, "pub use crate::traits::{lsc}::{lsc}_partial;")?;
                        }
                    }
                }
                writeln!(&mut file, "}}")?;
            }
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
                writeln!(&mut file, "mod {lsc};")?;
                writeln!(&mut file, "pub use {lsc}::{n};")?;
            }
            tx3.send(file_path)?;
            Ok(())
        });

        // lib.rs
        let src_folder2 = src_folder.clone();
        let tx2 = started_file.clone();
        let tx3 = finished_file.clone();
        join_set.spawn(async move {
            let file_path = src_folder2.join(Path::new("lib.rs"));
            tx2.send(file_path.clone())?;
            let mut file = fs::OpenOptions::new().write(true).create(true).truncate(true).open(&file_path)?;
            writeln!(&mut file, "pub mod data;")?;
            writeln!(&mut file, "pub mod traits;")?;
            writeln!(&mut file, "pub mod simd;")?;
            writeln!(&mut file, "#[allow(non_camel_case_types)]")?;
            writeln!(&mut file, "pub mod elements {{")?;
            let mut els = multi_vecs.full_multi_vector().elements();
            els.sort();
            for el in els {
                writeln!(&mut file, "    pub struct {el};")?;
            }
            writeln!(&mut file, "}}")?;
            tx3.send(file_path)?;
            Ok(())
        });

        // simd.rs
        let src_folder2 = src_folder.clone();
        let tx2 = started_file.clone();
        let tx3 = finished_file.clone();
        join_set.spawn(async move {
            let file_path = src_folder2.join(Path::new("simd.rs"));
            tx2.send(file_path.clone())?;
            let mut file = fs::OpenOptions::new().write(true).create(true).truncate(true).open(&file_path)?;
            let src = SIMD_SRC;
            write!(&mut file, "{src}")?;
            tx3.send(file_path)?;
            Ok(())
        });

        drop(started_file);
        drop(finished_file);
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
            ExpressionType::Int(i) => write!(w, "usize")?,
            ExpressionType::Float(f) => write!(w, "f32")?,
            ExpressionType::Vec2(v) => write!(w, "Simd32x2")?,
            ExpressionType::Vec3(v) => write!(w, "Simd32x3")?,
            ExpressionType::Vec4(v) => write!(w, "Simd32x4")?,
            ExpressionType::Class(mv) => {
                let n = mv.name();
                write!(w, "{n}")?;
            }
        }
        Ok(())
    }

    fn write_expression<W: Write>(&self, w: &mut W, expr: &AnyExpression) -> anyhow::Result<()> {
        match expr {
            AnyExpression::Int(e) => self.write_int(w, e)?,
            AnyExpression::Float(e) => self.write_float(w, e)?,
            AnyExpression::Vec2(e) => self.write_vec2(w, e)?,
            AnyExpression::Vec3(e) => self.write_vec3(w, e)?,
            AnyExpression::Vec4(e) => self.write_vec4(w, e)?,
            AnyExpression::Class(e) => self.write_multi_vec(w, e)?,
        }
        Ok(())
    }

    fn write_int<W: Write>(&self, w: &mut W, expr: &IntExpr) -> anyhow::Result<()> {
        match expr {
            IntExpr::Variable(v) => {
                let name = &v.decl.name.0;
                let mut no = v.decl.name.1;
                if no == 0 {
                    write!(w, "{name}")?;
                } else {
                    no += 1;
                    write!(w, "{name}_{no}")?;
                }
            }
            IntExpr::Literal(l) => {
                write!(w, "{l}")?;
            }
            IntExpr::TraitInvoke10ToInt(t, mv) => {
                let n = mv.name();
                let method = t.as_lower_snake();
                write!(w, "{n}::{method}()")?;
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

    fn write_float<W: Write>(&self, w: &mut W, expr: &FloatExpr) -> anyhow::Result<()> {
        match expr {
            FloatExpr::Variable(v) => {
                let name = &v.decl.name.0;
                let mut no = v.decl.name.1;
                if no == 0 {
                    write!(w, "{name}")?;
                } else {
                    no += 1;
                    write!(w, "{name}_{no}")?;
                }
            }
            FloatExpr::Literal(l) => self.write_f32(w, *l)?,
            FloatExpr::AccessVec2(v, i) => {
                self.write_vec2(w, v.as_ref())?;
                write!(w, "[{i}]")?;
            }
            FloatExpr::AccessVec3(v, i) => {
                self.write_vec3(w, v.as_ref())?;
                write!(w, "[{i}]")?;
            }
            FloatExpr::AccessVec4(v, i) => {
                self.write_vec4(w, v.as_ref())?;
                write!(w, "[{i}]")?;
            }
            FloatExpr::AccessMultiVecGroup(mv, i) => {
                self.write_multi_vec(w, mv)?;
                let el = mv.mv_class.elements()[*i as usize];
                write!(w, "[{el}]")?;
            }
            FloatExpr::AccessMultiVecFlat(mv, i) => {
                self.write_multi_vec(w, mv)?;
                let el = mv.mv_class.elements()[*i as usize];
                write!(w, "[{el}]")?;
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
                if len > 1 {
                    write!(w, "(")?;
                }
                for (i, (factor, exponent)) in v.iter().enumerate() {
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    match (*exponent, i > 0) {
                        (f, _) if f == 0.0 => continue,

                        (1.0, false) => self.write_float(w, factor)?,
                        (-1.0, false) => {
                            write!(w, "(1.0/")?;
                            self.write_float(w, factor)?;
                            write!(w, ")")?;
                        }
                        (e, false) => {
                            if e.fract() == 0.0 && e <= i32::MAX as f32 && e >= i32::MIN as f32 {
                                let e = e as i32;
                                write!(w, "f32::powi(")?;
                                self.write_float(w, factor)?;
                                write!(w, ", {e})")?;
                            } else {
                                write!(w, "f32::powf(")?;
                                self.write_float(w, factor)?;
                                write!(w, ", {e})")?;
                            }
                        }

                        (1.0, true) => {
                            write!(w, " * ")?;
                            self.write_float(w, factor)?;
                        }
                        (-1.0, true) => {
                            write!(w, " / (")?;
                            self.write_float(w, factor)?;
                            write!(w, ")")?;
                        }
                        (e, true) => {
                            if e.fract() == 0.0 && e <= i32::MAX as f32 && e >= i32::MIN as f32 {
                                let e = e as i32;
                                write!(w, " * f32::powi(")?;
                                self.write_float(w, factor)?;
                                write!(w, ", {e})")?;
                            } else {
                                write!(w, " * f32::powf(")?;
                                self.write_float(w, factor)?;
                                write!(w, ", {e})")?;
                            }
                        }

                        _ => unreachable!("This match is complete across conditions (unless NaN?)"),
                    }
                }
                match (*last_factor, len > 1) {
                    (fl, _) if fl == 1.0 => {}
                    (fl, false) => self.write_f32(w, fl)?,
                    (fl, true) => {
                        write!(w, " * ")?;
                        self.write_f32(w, fl)?
                    }
                    _ => {}
                }
                if len > 1 {
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
                if len > 1 {
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
                            write!(w, " + *")?;
                            self.write_f32(w, f)?
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
                    self.write_float(w, addend)?;
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
                if len > 1 {
                    write!(w, ")")?;
                }
            }
        }
        Ok(())
    }

    fn write_vec2<W: Write>(&self, w: &mut W, expr: &Vec2Expr) -> anyhow::Result<()> {
        match expr {
            Vec2Expr::Variable(v) => {
                let name = &v.decl.name.0;
                let mut no = v.decl.name.1;
                if no == 0 {
                    write!(w, "{name}")?;
                } else {
                    no += 1;
                    write!(w, "{name}_{no}")?;
                }
            }
            Vec2Expr::Gather1(f) => {
                write!(w, "Simd32x2::from(")?;
                self.write_float(w, f)?;
                write!(w, ")")?;
            }
            Vec2Expr::Gather2(f0, f1) => {
                write!(w, "Simd32x2::from([")?;
                self.write_float(w, f0)?;
                write!(w, ", ")?;
                self.write_float(w, f1)?;
                write!(w, "])")?;
            }
            Vec2Expr::AccessMultiVecGroup(mv, i) => {
                self.write_multi_vec(w, mv)?;
                write!(w, ".group{i}()")?;
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
                if len > 1 {
                    write!(w, "(")?;
                }
                for (i, (factor, exponent)) in v.iter().enumerate() {
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    match (*exponent, i > 0) {
                        (f, _) if f == 0.0 => continue,

                        (1.0, false) => self.write_vec2(w, factor)?,
                        (-1.0, false) => {
                            write!(w, "(Simd32x2::from(1.0) / ")?;
                            self.write_vec2(w, factor)?;
                            write!(w, ")")?;
                        }
                        (e, false) => {
                            if e.fract() == 0.0 && e <= i32::MAX as f32 && e >= i32::MIN as f32 {
                                let e = e as i32;
                                write!(w, "Simd32x2::powi(")?;
                                self.write_vec2(w, factor)?;
                                write!(w, ", {e})")?;
                            } else {
                                write!(w, "Simd32x2::powf(")?;
                                self.write_vec2(w, factor)?;
                                write!(w, ", {e})")?;
                            }
                        }

                        (1.0, true) => {
                            write!(w, " * ")?;
                            self.write_vec2(w, factor)?
                        }
                        (-1.0, true) => {
                            write!(w, " / (")?;
                            self.write_vec2(w, factor)?;
                            write!(w, ")")?;
                        }
                        (e, true) => {
                            if e.fract() == 0.0 && e <= i32::MAX as f32 && e >= i32::MIN as f32 {
                                let e = e as i32;
                                write!(w, " * Simd32x2::powi(")?;
                                self.write_vec2(w, factor)?;
                                write!(w, ", {e})")?;
                            } else {
                                write!(w, " * Simd32x2::powf(")?;
                                self.write_vec2(w, factor)?;
                                write!(w, ", {e})")?;
                            }
                        }

                        _ => unreachable!("This match is complete across conditions (unless NaN?)"),
                    }
                }
                if *last_factor != [1.0; 2] {
                    if len > 1 {
                        write!(w, " * ")?;
                    }
                    let a = last_factor[0];
                    let b = last_factor[1];
                    if a == b {
                        write!(w, "Simd32x2::from(")?;
                        self.write_f32(w, a)?;
                        write!(w, ")")?;
                    } else {
                        write!(w, "Simd32x2::from([")?;
                        self.write_f32(w, a)?;
                        write!(w, ", ")?;
                        self.write_f32(w, b)?;
                        write!(w, "])")?;
                    }
                }
                if len > 1 {
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
                if len > 1 {
                    write!(w, "(")?;
                }
                for (i, (addend, factor)) in v.iter().enumerate() {
                    match (*factor, i > 0) {
                        (f, _) if f == 0.0 => continue,

                        (1.0, false) => {}
                        (-1.0, false) => write!(w, "-")?,
                        (f, false) => {
                            write!(w, "Simd32x2::from(")?;
                            self.write_f32(w, f)?;
                            write!(w, ")*")?;
                        }

                        (1.0, true) => write!(w, " + ")?,
                        (-1.0, true) => write!(w, " - ")?,
                        (f, true) if f > 0.0 => {
                            write!(w, " + Simd32x2::from(")?;
                            self.write_f32(w, f)?;
                            write!(w, ")*")?;
                        }
                        (f, true) if f < 0.0 => {
                            let f = -f;
                            write!(w, " - Simd32x2::from(")?;
                            self.write_f32(w, f)?;
                            write!(w, ")*")?;
                        }
                        _ => unreachable!("This match is complete across if conditions (unless NaN?)"),
                    }
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    self.write_vec2(w, addend)?;
                }
                let a0 = last_addend[0];
                let a1 = last_addend[1];
                if *last_addend != [0.0; 2] {
                    if len > 1 {
                        write!(w, " + ")?;
                    }
                    if a0 == a1 {
                        write!(w, "Simd32x2::from(")?;
                        self.write_f32(w, a0)?;
                        write!(w, ")")?;
                    } else {
                        write!(w, "Simd32x2::from([")?;
                        self.write_f32(w, a0)?;
                        write!(w, ", ")?;
                        self.write_f32(w, a1)?;
                        write!(w, "])")?;
                    }
                }
                if len > 1 {
                    write!(w, ")")?;
                }
            }
            Vec2Expr::SwizzleVec2(box v, i0, i1) => {
                write!(w, "swizzle!(")?;
                self.write_vec2(w, v)?;
                write!(w, ", {i0}, {i1})")?;
            }
        }
        Ok(())
    }

    fn write_vec3<W: Write>(&self, w: &mut W, expr: &Vec3Expr) -> anyhow::Result<()> {
        match expr {
            Vec3Expr::Variable(v) => {
                let name = &v.decl.name.0;
                let mut no = v.decl.name.1;
                if no == 0 {
                    write!(w, "{name}")?;
                } else {
                    no += 1;
                    write!(w, "{name}_{no}")?;
                }
            }
            Vec3Expr::Gather1(f) => {
                write!(w, "Simd32x3::from(")?;
                self.write_float(w, f)?;
                write!(w, ")")?;
            }
            Vec3Expr::Gather3(f0, f1, f2) => {
                write!(w, "Simd32x3::from([")?;
                self.write_float(w, f0)?;
                write!(w, ", ")?;
                self.write_float(w, f1)?;
                write!(w, ", ")?;
                self.write_float(w, f2)?;
                write!(w, "])")?;
            }
            Vec3Expr::AccessMultiVecGroup(mv, i) => {
                self.write_multi_vec(w, mv)?;
                write!(w, ".group{i}()")?;
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
                if len > 1 {
                    write!(w, "(")?;
                }
                for (i, (factor, exponent)) in v.iter().enumerate() {
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    match (*exponent, i > 0) {
                        (f, _) if f == 0.0 => continue,

                        (1.0, false) => self.write_vec3(w, factor)?,
                        (-1.0, false) => {
                            write!(w, "(Simd32x3::from(1.0) / ")?;
                            self.write_vec3(w, factor)?;
                            write!(w, ")")?;
                        }
                        (e, false) => {
                            if e.fract() == 0.0 && e <= i32::MAX as f32 && e >= i32::MIN as f32 {
                                let e = e as i32;
                                write!(w, "Simd32x3::powi(")?;
                                self.write_vec3(w, factor)?;
                                write!(w, ", {e})")?;
                            } else {
                                write!(w, "Simd32x3::powf(")?;
                                self.write_vec3(w, factor)?;
                                write!(w, ", {e})")?;
                            }
                        }

                        (1.0, true) => {
                            write!(w, " * ")?;
                            self.write_vec3(w, factor)?
                        }
                        (-1.0, true) => {
                            write!(w, " / (")?;
                            self.write_vec3(w, factor)?;
                            write!(w, ")")?;
                        }
                        (e, true) => {
                            if e.fract() == 0.0 && e <= i32::MAX as f32 && e >= i32::MIN as f32 {
                                let e = e as i32;
                                write!(w, " * Simd32x3::powi(")?;
                                self.write_vec3(w, factor)?;
                                write!(w, ", {e})")?;
                            } else {
                                write!(w, " * Simd32x3::powf(")?;
                                self.write_vec3(w, factor)?;
                                write!(w, ", {e})")?;
                            }
                        }

                        _ => unreachable!("This match is complete across conditions (unless NaN?)"),
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
                        write!(w, "Simd32x3::from(")?;
                        self.write_f32(w, a)?;
                        write!(w, ")")?;
                    } else {
                        write!(w, "Simd32x3::from([")?;
                        self.write_f32(w, a)?;
                        write!(w, ", ")?;
                        self.write_f32(w, b)?;
                        write!(w, ", ")?;
                        self.write_f32(w, c)?;
                        write!(w, "])")?;
                    }
                }
                if len > 1 {
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
                if len > 1 {
                    write!(w, "(")?;
                }
                for (i, (addend, factor)) in v.iter().enumerate() {
                    match (*factor, i > 0) {
                        (f, _) if f == 0.0 => continue,

                        (1.0, false) => {}
                        (-1.0, false) => write!(w, "-")?,
                        (f, false) => {
                            write!(w, "Simd32x3::from(")?;
                            self.write_f32(w, f)?;
                            write!(w, ")*")?;
                        }

                        (1.0, true) => write!(w, " + ")?,
                        (-1.0, true) => write!(w, " - ")?,
                        (f, true) if f > 0.0 => {
                            write!(w, " + Simd32x3::from(")?;
                            self.write_f32(w, f)?;
                            write!(w, ")*")?;
                        }
                        (f, true) if f < 0.0 => {
                            let f = -f;
                            write!(w, " - Simd32x3::from(")?;
                            self.write_f32(w, f)?;
                            write!(w, ")*")?;
                        }
                        _ => unreachable!("This match is complete across if conditions (unless NaN?)"),
                    }
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    self.write_vec3(w, addend)?;
                }
                if *last_addend != [0.0; 3] {
                    if len > 1 {
                        write!(w, " + ")?;
                    }
                    let a = last_addend[0];
                    let b = last_addend[1];
                    let c = last_addend[2];
                    if a == b && b == c {
                        write!(w, "Simd32x3::from(")?;
                        self.write_f32(w, a)?;
                        write!(w, ")")?;
                    } else {
                        write!(w, "Simd32x3::from([")?;
                        self.write_f32(w, a)?;
                        write!(w, ", ")?;
                        self.write_f32(w, b)?;
                        write!(w, ", ")?;
                        self.write_f32(w, c)?;
                        write!(w, "])")?;
                    }
                }
                if len > 1 {
                    write!(w, ")")?;
                }
            }
            Vec3Expr::SwizzleVec3(box v, i0, i1, i2) => {
                write!(w, "swizzle!(")?;
                self.write_vec3(w, v)?;
                write!(w, ", {i0}, {i1}, {i2})")?;
            }
        }
        Ok(())
    }

    fn write_vec4<W: Write>(&self, w: &mut W, expr: &Vec4Expr) -> anyhow::Result<()> {
        match expr {
            Vec4Expr::Variable(v) => {
                let name = &v.decl.name.0;
                let mut no = v.decl.name.1;
                if no == 0 {
                    write!(w, "{name}")?;
                } else {
                    no += 1;
                    write!(w, "{name}_{no}")?;
                }
            }
            Vec4Expr::Gather1(f) => {
                write!(w, "Simd32x4::from(")?;
                self.write_float(w, f)?;
                write!(w, ")")?;
            }
            Vec4Expr::Gather4(f0, f1, f2, f3) => {
                write!(w, "Simd32x4::from([")?;
                self.write_float(w, f0)?;
                write!(w, ", ")?;
                self.write_float(w, f1)?;
                write!(w, ", ")?;
                self.write_float(w, f2)?;
                write!(w, ", ")?;
                self.write_float(w, f3)?;
                write!(w, "])")?;
            }
            Vec4Expr::AccessMultiVecGroup(mv, i) => {
                self.write_multi_vec(w, mv)?;
                write!(w, ".group{i}()")?;
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
                if len > 1 {
                    write!(w, "(")?;
                }
                for (i, (factor, exponent)) in v.iter().enumerate() {
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    match (*exponent, i > 0) {
                        (f, _) if f == 0.0 => continue,

                        (1.0, false) => self.write_vec4(w, factor)?,
                        (-1.0, false) => {
                            write!(w, "(Simd32x4::from(1.0) / ")?;
                            self.write_vec4(w, factor)?;
                            write!(w, ")")?;
                        }
                        (e, false) => {
                            if e.fract() == 0.0 && e <= i32::MAX as f32 && e >= i32::MIN as f32 {
                                let e = e as i32;
                                write!(w, "Simd32x4::powi(")?;
                                self.write_vec4(w, factor)?;
                                write!(w, ", {e})")?;
                            } else {
                                write!(w, "Simd32x4::powf(")?;
                                self.write_vec4(w, factor)?;
                                write!(w, ", {e})")?;
                            }
                        }

                        (1.0, true) => {
                            write!(w, " * ")?;
                            self.write_vec4(w, factor)?
                        }
                        (-1.0, true) => {
                            write!(w, " / (")?;
                            self.write_vec4(w, factor)?;
                            write!(w, ")")?;
                        }
                        (e, true) => {
                            if e.fract() == 0.0 && e <= i32::MAX as f32 && e >= i32::MIN as f32 {
                                let e = e as i32;
                                write!(w, " * Simd32x4::powi(")?;
                                self.write_vec4(w, factor)?;
                                write!(w, ", {e})")?;
                            } else {
                                write!(w, " * Simd32x4::powf(")?;
                                self.write_vec4(w, factor)?;
                                write!(w, ", {e})")?;
                            }
                        }

                        _ => unreachable!("This match is complete across conditions (unless NaN?)"),
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
                        write!(w, "Simd32x4::from(")?;
                        self.write_f32(w, a)?;
                        write!(w, ")")?;
                    } else {
                        write!(w, "Simd32x4::from([")?;
                        self.write_f32(w, a)?;
                        write!(w, ", ")?;
                        self.write_f32(w, b)?;
                        write!(w, ", ")?;
                        self.write_f32(w, c)?;
                        write!(w, ", ")?;
                        self.write_f32(w, d)?;
                        write!(w, "])")?;
                    }
                }
                if len > 1 {
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
                if len > 1 {
                    write!(w, "(")?;
                }
                for (i, (addend, factor)) in v.iter().enumerate() {
                    match (*factor, i > 0) {
                        (f, _) if f == 0.0 => continue,

                        (1.0, false) => {}
                        (-1.0, false) => write!(w, "-")?,
                        (f, false) => {
                            write!(w, "Simd32x4::from(")?;
                            self.write_f32(w, f)?;
                            write!(w, ")*")?;
                        }

                        (1.0, true) => write!(w, " + ")?,
                        (-1.0, true) => write!(w, " - ")?,
                        (f, true) if f > 0.0 => {
                            write!(w, " + Simd32x4::from(")?;
                            self.write_f32(w, f)?;
                            write!(w, ")*")?;
                        }
                        (f, true) if f < 0.0 => {
                            let f = -f;
                            write!(w, " - Simd32x4::from(")?;
                            self.write_f32(w, f)?;
                            write!(w, ")*")?;
                        }
                        _ => unreachable!("This match is complete across if conditions (unless NaN?)"),
                    }
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    self.write_vec4(w, addend)?;
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
                        write!(w, "Simd32x4::from(")?;
                        self.write_f32(w, a)?;
                        write!(w, ")")?;
                    } else {
                        write!(w, "Simd32x4::from([")?;
                        self.write_f32(w, a)?;
                        write!(w, ", ")?;
                        self.write_f32(w, b)?;
                        write!(w, ", ")?;
                        self.write_f32(w, c)?;
                        write!(w, ", ")?;
                        self.write_f32(w, d)?;
                        write!(w, "])")?;
                    }
                }
                if len > 1 {
                    write!(w, ")")?;
                }
            }
            Vec4Expr::SwizzleVec4(box v, i0, i1, i2, i3) => {
                write!(w, "swizzle!(")?;
                self.write_vec4(w, v)?;
                write!(w, ", {i0}, {i1}, {i2}, {i3})")?;
            }
        }
        Ok(())
    }

    fn write_multi_vec<W: Write>(&self, w: &mut W, expr: &MultiVectorExpr) -> anyhow::Result<()> {
        let mv = expr.mv_class;
        match &*expr.expr {
            MultiVectorVia::Variable(v) => {
                let name = &v.decl.name.0;
                let mut no = v.decl.name.1;
                if no == 0 {
                    write!(w, "{name}")?;
                } else {
                    no += 1;
                    write!(w, "{name}_{no}")?;
                }
            }
            MultiVectorVia::Construct(v) => {
                let n = mv.name();
                write!(w, "{n}::from_groups(")?;
                let groups = mv.groups();
                for (i, g) in v.iter().enumerate() {
                    if i > 0 {
                        write!(w, ", ")?;
                    }
                    write!(w, "/* ")?;
                    for (i, el) in groups[i].into_vec().into_iter().enumerate() {
                        if i > 0 {
                            write!(w, ", ")?;
                        }
                        write!(w, "{el}")?;
                    }
                    write!(w, " */")?;
                    match g {
                        MultiVectorGroupExpr::JustFloat(f) => self.write_float(w, f)?,
                        MultiVectorGroupExpr::Vec2(g) => self.write_vec2(w, g)?,
                        MultiVectorGroupExpr::Vec3(g) => self.write_vec3(w, g)?,
                        MultiVectorGroupExpr::Vec4(g) => self.write_vec4(w, g)?,
                    }
                }
                write!(w, ")")?;
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
        }
        Ok(())
    }

    fn write_trait_from<W: Write>(&self, w: &mut W, impls: Arc<RawTraitImplementation>) -> anyhow::Result<()> {
        let ExpressionType::Class(other) = impls.owner else {
            bail!("Owner of Into (Other of From) impl is not a MultiVector")
        };
        let Some(ExpressionType::Class(owner)) = impls.other_type_params.get(0) else {
            bail!("Other of Into (Owner of From) impl is not a MultiVector")
        };
        let other = other.name();
        let owner = owner.name();
        let lsc = TraitKey::new(other).as_lower_snake();
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
        writeln!(w, "        return ")?;
        self.write_expression(w, &ret)?;
        writeln!(w, "    }}\n}}")?;
        Ok(())
    }

    fn write_trait_try_from<W: Write>(&self, w: &mut W, impls: Arc<RawTraitImplementation>) -> anyhow::Result<()> {
        let ExpressionType::Class(other) = impls.owner else {
            bail!("Owner of Into (Other of From) impl is not a MultiVector")
        };
        let Some(ExpressionType::Class(owner)) = impls.other_type_params.get(0) else {
            bail!("Other of Into (Owner of From) impl is not a MultiVector")
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
        self.write_expression(w, &ret)?;
        writeln!(w, ")\n    }}\n}}")?;
        Ok(())
    }

    fn file_extension() -> &'static str {
        "rs"
    }

    fn declare_multi_vector<W: Write, const AntiScalar: BasisElement>(&self, w: &mut W, multi_vec: &'static MultiVec<AntiScalar>, docs: Option<String>) -> anyhow::Result<()> {
        let name = TraitKey::new(multi_vec.name);
        let ucc = name.as_upper_camel();
        let lcc = name.as_lower_camel();
        let lsc = name.as_lower_snake();
        let ssc = name.as_screaming_snake();
        // TODO built in documentation, statistics, and traits that output this type
        let docs = docs.unwrap_or(ucc.clone());
        self.emit_comment(w, true, docs)?;

        write!(w, "#[derive(Clone, Copy")?;
        if self.nearly_eq_ord {
            write!(w, ", nearly::NearlyEq, nearly::NearlyOrd")?;
        }
        if self.wgsl || self.glsl {
            write!(w, ", bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType")?;
        }
        if self.serde {
            write!(w, ", serde::Serialize, serde::Deserialize")?;
        }
        writeln!(w, ")]")?;
        writeln!(w, "pub union {ucc} {{")?;
        writeln!(w, "    groups: {ucc}Groups,")?;
        write!(w, "    /// ")?;
        let mut total_len = 0;
        for (i, g) in multi_vec.groups().into_iter().enumerate() {
            if i > 0 {
                write!(w, ", ")?;
            }
            let mut g = g.into_vec();
            while g.len() < 4 {
                g.push(BasisElement::zero());
            }
            for (i, el) in g.into_iter().enumerate() {
                if i > 0 {
                    write!(w, ", ")?;
                }
                write!(w, "{el}")?;
            }
            total_len += 4;
        }
        writeln!(w, "\n    elements: [f32; {total_len}],")?;
        writeln!(w, "}}")?;

        write!(w, "#[derive(Clone, Copy")?;
        if self.nearly_eq_ord {
            write!(w, ", nearly::NearlyEq, nearly::NearlyOrd")?;
        }
        if self.wgsl || self.glsl {
            write!(w, ", bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType")?;
        }
        if self.serde {
            write!(w, ", serde::Serialize, serde::Deserialize")?;
        }
        writeln!(w, ")]")?;
        writeln!(w, "pub struct {ucc}Groups {{")?;
        for (g, group) in multi_vec.groups().into_iter().enumerate() {
            write!(w, "    /// ")?;
            for (i, el) in group.into_iter().enumerate() {
                if i > 0 {
                    write!(w, ", ")?;
                }
                write!(w, "{el}")?;
            }
            write!(w, "\n    g{g}: ")?;
            self.write_type(w, group.expr_type())?;
            writeln!(w, ",")?;
        }
        writeln!(w, "}}")?;

        writeln!(w, "impl {ucc} {{")?;
        writeln!(w, "#[allow(clippy::too_many_arguments)]")?;
        writeln!(w, "pub const fn from_elements(")?;
        for (i, el) in multi_vec.elements().into_iter().enumerate() {
            if i > 0 {
                write!(w, ", ")?;
            } else {
                write!(w, "    ")?;
            }
            write!(w, "{el}: f32")?;
        }
        writeln!(w, "\n) -> Self {{")?;
        write!(w, "    Self {{ elements: [")?;
        for (i, g) in multi_vec.groups().into_iter().enumerate() {
            if i > 0 {
                write!(w, ", ")?;
            }
            let mut count = 0;
            for (i, el) in g.into_iter().enumerate() {
                if i > 0 {
                    write!(w, ", ")?;
                }
                write!(w, "{el}")?;
                count += 1;
            }
            while count < 4 {
                write!(w, ", 0.0")?;
                count += 1;
            }
        }
        writeln!(w, "] }}")?;
        writeln!(w, "}}")?;
        writeln!(w, "pub const fn from_groups(")?;
        for (i, g) in multi_vec.groups().into_iter().enumerate() {
            if i > 0 {
                write!(w, ", ")?;
            } else {
                write!(w, "    ")?;
            }
            write!(w, "g{i}: ")?;
            self.write_type(w, g.expr_type())?;
        }
        writeln!(w, "\n) -> Self {{")?;
        writeln!(w, "    Self {{\n        groups: {ucc}Groups {{")?;
        for (i, _) in multi_vec.groups().into_iter().enumerate() {
            if i > 0 {
                write!(w, ", ")?;
            } else {
                write!(w, "            ")?;
            }
            write!(w, "g{i}")?;
        }
        writeln!(w, "\n        }}\n    }}\n}}")?;

        for (i, g) in multi_vec.groups().into_iter().enumerate() {
            let t = g.expr_type();

            writeln!(w, "#[inline(always)]")?;
            write!(w, "pub fn group{i}(&self) -> ")?;
            self.write_type(w, t)?;
            writeln!(w, " {{")?;
            writeln!(w, "    unsafe {{ self.groups.g{i} }}")?;
            writeln!(w, "}}")?;

            writeln!(w, "#[inline(always)]")?;
            write!(w, "pub fn group{i}_mut(&mut self) -> &mut ")?;
            self.write_type(w, t)?;
            writeln!(w, " {{")?;
            writeln!(w, "    unsafe {{ &mut self.groups.g{i} }}")?;
            writeln!(w, "}}")?;
        }
        writeln!(w, "}}")?;

        let element_count = multi_vec.elements().len();
        write!(w, "const {ssc}_INDEX_REMAP: [usize; {element_count}] = [")?;
        let mut i = 0;
        for (j, g) in multi_vec.groups().into_iter().enumerate() {
            if j > 0 {
                write!(w, ", ")?;
            }
            let g = g.into_vec();
            for (k, _) in g.into_iter().enumerate() {
                if k > 0 {
                    write!(w, ", ")?;
                }
                write!(w, "{i}")?;
                i += 1;
            }
            i += 4 - g.len();
        }
        writeln!(w, "];")?;

        writeln!(w, "impl std::ops::Index<usize> for {ucc} {{")?;
        writeln!(w, "type Output = f32;")?;
        writeln!(w, "fn index(&self, index: usize) -> &Self::Output {{")?;
        writeln!(w, "    unsafe {{ &self.elements[{ssc}_INDEX_REMAP[index]] }}")?;
        writeln!(w, "}}\n}}")?;

        writeln!(w, "impl std::ops::IndexMut<usize> for {ucc} {{")?;
        writeln!(w, "fn index_mut(&mut self, index: usize) -> &mut Self::Output {{")?;
        writeln!(w, "    unsafe {{ &mut self.elements[{ssc}_INDEX_REMAP[index]] }}")?;
        writeln!(w, "}}\n}}")?;

        writeln!(w, "impl From<{ucc}> for [f32; {element_count}] {{")?;
        writeln!(w, "fn from(vector: {ucc}) -> Self {{")?;
        write!(w, "    unsafe {{\n        [")?;
        let mut i = 0;
        for (j, g) in multi_vec.groups().into_iter().enumerate() {
            if j > 0 {
                write!(w, ", ")?;
            }
            let g = g.into_vec();
            for (k, _) in g.into_iter().enumerate() {
                if k > 0 {
                    write!(w, ", ")?;
                }
                write!(w, "vector.elements[{i}]")?;
                i += 1;
            }
            i += 4 - g.len();
        }
        writeln!(w, "] }}\n    }}\n}}")?;

        writeln!(w, "impl From<[f32; {element_count}]> for {ucc} {{")?;
        writeln!(w, "    fn from(array: [f32; {element_count}]) -> Self {{")?;
        write!(w, "        Self {{ elements: [")?;
        let mut i = 0;
        for (j, g) in multi_vec.groups().into_iter().enumerate() {
            if j > 0 {
                write!(w, ", ")?;
            }
            let g = g.into_vec();
            for (k, _) in g.into_iter().enumerate() {
                if k > 0 {
                    write!(w, ", ")?;
                }
                let l = j + k;
                write!(w, "array[{l}]")?;
                i += 1;
            }
            while i % 4 > 0 {
                write!(w, ", 0.0")?;
                i += 1;
            }
        }
        writeln!(w, "] }}\n    }}\n}}")?;

        writeln!(w, "impl std::fmt::Debug for {ucc} {{")?;
        writeln!(w, "fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {{")?;
        writeln!(w, "    formatter.debug_struct(\"{ucc}\")")?;
        for (i, el) in multi_vec.elements().into_iter().enumerate() {
            writeln!(w, "        .field(\"{el}\", &self[{i}])")?;
        }
        writeln!(w, "        .finish()\n}}\n}}")?;

        let els = multi_vec.elements();
        let els_len = els.len();

        writeln!(
            w,
            r#"
impl {ucc} {{
    pub const LEN: usize = {els_len};
}}"#
        )?;
        if self.nearly_eq_ord {
            writeln!(
                w,
                r#"
impl {ucc} {{
    pub fn clamp_zeros(mut self, tolerance: nearly::Tolerance<f32>) -> Self {{
        for i in 0..Self::LEN {{
            let f = self[i];
            if nearly::nearly!(0.0 == f, tol = tolerance) {{
                self[i] = 0.0;
            }}
        }}
        self
    }}
}}"#
            )?;
        }
        if self.eq_ord_hash {
            writeln!(
                w,
                r#"
impl PartialOrd for {ucc} {{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {{
        for i in 0..Self::LEN {{
            let a = float_ord::FloatOrd(self[i]);
            let b = float_ord::FloatOrd(other[i]);
            match a.cmp(&b) {{
                std::cmp::Ordering::Equal => continue,
                result => return Some(result),
            }}
        }}
        Some(std::cmp::Ordering::Equal)
    }}
}}
impl Ord for {ucc} {{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {{
        for i in 0..Self::LEN {{
            let a = float_ord::FloatOrd(self[i]);
            let b = float_ord::FloatOrd(other[i]);
            match a.cmp(&b) {{
                std::cmp::Ordering::Equal => continue,
                result => return result,
            }}
        }}
        std::cmp::Ordering::Equal
    }}
}}
impl PartialEq for {ucc} {{
    fn eq(&self, other: &Self) -> bool {{
        for i in 0..Self::LEN {{
            let a = float_ord::FloatOrd(self[i]);
            let b = float_ord::FloatOrd(other[i]);
            if a != b {{
                return false
            }}
        }}
        true
    }}
}}
impl Eq for {ucc} {{}}
impl std::hash::Hash for {ucc} {{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {{
        for i in 0..Self::LEN {{
            self[i].to_bits().hash(state);
        }}
    }}
}}
"#
            )?;
        }

        for (i, el) in els.clone().into_iter().enumerate() {
            writeln!(w, "impl std::ops::Index<crate::elements::{el}> for {ucc} {{")?;
            writeln!(w, "    type Output = f32;")?;
            writeln!(w, "    fn index(&self, _: crate::elements::{el}) -> &Self::Output {{")?;
            writeln!(w, "       &self[{i}]")?;
            writeln!(w, "    }}\n}}")?;
        }
        for (i, el) in els.into_iter().enumerate() {
            writeln!(w, "impl std::ops::IndexMut<crate::elements::{el}> for {ucc} {{")?;
            writeln!(w, "    fn index_mut(&self, _: crate::elements::{el}) -> &mut Self::Output {{")?;
            writeln!(w, "       &mut self[{i}]")?;
            writeln!(w, "    }}\n}}")?;
        }
        // for len in 1..(els_len+1) {
        //     write!(w, "impl<")?;
        //     for i in 0..len {
        //         write!(w, "I{i}, ")?;
        //     }
        //     write!(w, "> std::ops::Index<(")?;
        //     for i in 0..len {
        //         write!(w, "I{i}, ")?;
        //     }
        //     writeln!(w, ")> for {ucc} where Self: std::ops::Index<I0, Output=f32>")?;
        //     for i in 1..len {
        //         write!(w, " + std::ops::Index<I{i}, Output=f32>")?;
        //     }
        //     writeln!(w, " {{")?;
        //     writeln!(w, "    type Output = [f32; {len}];")?;
        //     write!(w, "    fn index(&self, (")?;
        //     for i in 0..len {
        //         write!(w, "i{i}, ")?;
        //     }
        //     write!(w, "): (")?;
        //     for i in 0..len {
        //         write!(w, "I{i}, ")?;
        //     }
        //     writeln!(w, ")) -> &Self::Output {{")?;
        //     write!(w, "       [")?;
        //     for i in 0..len {
        //         write!(w, "self[i{i}], ")?;
        //     }
        //     writeln!(w, "]")?;
        //     writeln!(w, "    }}\n}}")?;
        // }

        Ok(())
    }

    fn declare_trait_def<W: Write>(&self, w: &mut W, def: Arc<RawTraitDefinition>) -> anyhow::Result<()> {
        let ucc = def.names.trait_key.as_upper_camel();
        let lsc = def.names.trait_key.as_lower_snake();
        self.emit_comment(w, true, &def.documentation)?;
        // todo alias documentation
        write!(w, "pub trait {ucc}")?;
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
                writeln!(w, "    type Output;")?;
            }
        }
        write!(w, "    fn {lsc}(")?;
        match def.arity {
            TraitArity::Zero => {}
            TraitArity::One => write!(w, "self")?,
            TraitArity::Two => write!(w, "self, other: T")?,
        }
        write!(w, ") -> ")?;
        match *output_ty {
            TraitTypeConsensus::AlwaysSelf => write!(w, "Self")?,
            TraitTypeConsensus::AllAgree(et, _) => self.write_type(w, et)?,
            TraitTypeConsensus::NoVotes | TraitTypeConsensus::Disagreement => write!(w, "Self::Output")?,
        }
        writeln!(w, ";\n}}")?;

        let op = *def.op.lock();
        if let Some(op) = op {
            // TODO
        }

        if let Some(op) = &self.fancy_infix {
            if let TraitArity::Two = def.arity {
                writeln!(w, "pub trait Infix{ucc} {{}}")?;
            }
            writeln!(w, "#[allow(non_camel_case_types)]")?;
            writeln!(w, "pub struct {lsc};")?;
            if let TraitArity::Two = def.arity {
                writeln!(w, "#[allow(non_camel_case_types)]")?;
                writeln!(w, "pub struct {lsc}_partial<A>(A);")?;
            }
            let operator_name = op.rust_trait_name();
            let operator_method = op.rust_trait_method();
            if let TraitArity::Two = def.arity {
                writeln!(w, "impl<A: Infix{ucc}> std::ops::{operator_name}<{lsc}> for A {{")?;
                writeln!(w, "    type Output = {lsc}_partial<A>;")?;
                writeln!(w, "    fn {operator_method}(self, _rhs: {lsc}) -> Self::Output {{")?;
                writeln!(w, "        {lsc}_partial(self)")?;
                writeln!(w, "    }}\n}}")?;
                writeln!(w, "impl<A: {ucc}<B>, B> std::ops::{operator_name}<B> for {lsc}_partial<A> {{")?;
                write!(w, "    type Output = ")?;
                match *output_ty {
                    TraitTypeConsensus::AlwaysSelf => write!(w, "A")?,
                    TraitTypeConsensus::AllAgree(et, _) => self.write_type(w, et)?,
                    TraitTypeConsensus::NoVotes | TraitTypeConsensus::Disagreement => write!(w, "<A as {ucc}<B>>::Output")?,
                }
                writeln!(w, ";")?;
                writeln!(w, "    fn {operator_method}(self, rhs: B) -> Self::Output {{")?;
                writeln!(w, "        self.0.{lsc}(rhs)")?;
                writeln!(w, "    }}\n}}")?;
            }
            if let TraitArity::One = def.arity {
                writeln!(w, "impl<A: {ucc}> std::ops::{operator_name}<{lsc}> for A {{")?;
                write!(w, "    type Output = ")?;
                match *output_ty {
                    TraitTypeConsensus::AlwaysSelf => write!(w, "A")?,
                    TraitTypeConsensus::AllAgree(et, _) => self.write_type(w, et)?,
                    TraitTypeConsensus::NoVotes | TraitTypeConsensus::Disagreement => write!(w, "<A as {ucc}>::Output")?,
                }
                writeln!(w, ";")?;
                writeln!(w, "    fn {operator_method}(self, _rhs: {lsc}) -> Self::Output {{")?;
                writeln!(w, "        self.{lsc}()")?;
                writeln!(w, "    }}\n}}")?;
                writeln!(w, "impl<A: {ucc}> std::ops::{operator_name}<A> for {lsc} {{")?;
                write!(w, "    type Output = ")?;
                match *output_ty {
                    TraitTypeConsensus::AlwaysSelf => write!(w, "A")?,
                    TraitTypeConsensus::AllAgree(et, _) => self.write_type(w, et)?,
                    TraitTypeConsensus::NoVotes | TraitTypeConsensus::Disagreement => write!(w, "<A as {ucc}>::Output")?,
                }
                writeln!(w, ";")?;
                writeln!(w, "    fn {operator_method}(self, rhs: A) -> Self::Output {{")?;
                writeln!(w, "        rhs.{lsc}()")?;
                writeln!(w, "    }}\n}}")?;
                if let TraitTypeConsensus::AlwaysSelf = *output_ty {
                    writeln!(w, "impl<A: {ucc}> std::ops::{operator_name}Assign<A> for {lsc} {{")?;
                    writeln!(w, "    fn {operator_method}_assign(&mut self, rhs: {lsc}) {{")?;
                    writeln!(w, "        *self = *self.{lsc}()")?;
                    writeln!(w, "    }}\n}}")?;
                }
            }
        }

        Ok(())
    }

    fn declare_trait_impl<W: Write>(&self, w: &mut W, impls: Arc<RawTraitImplementation>, already_granted_infix: &mut BTreeSet<&'static str>) -> anyhow::Result<()> {
        let def = &impls.definition;

        let output_kind = def.output.read();
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
        let mut module = "";
        if let Some(op) = op {
            if op.rust_trait_name() == ucc.as_str() {
                is_op = true;
                module = op.rust_mod();
                lsc = op.rust_trait_method().to_string();
                do_assign_impl = def.arity == TraitArity::Two && *owner_ty == output_ty;
            }
        }
        let module = module;
        let is_op = is_op;
        let lsc = lsc;
        let do_assign_impl = do_assign_impl;

        let mut var_param = None;
        if !impls.other_var_params.is_empty() {
            let ty_param = &impls.other_type_params[0];
            let v_param = &impls.other_var_params[0];
            if ty_param != v_param {
                // TODO I feel like this is a representation problem, need to review and maybe
                //  refactor the algebraic data types involved here
                bail!("Type of trait implementation does not agree");
            }
            var_param = Some(v_param);
        }
        if let TraitArity::Two = def.arity {
            if let TraitParam::Class(mv) = &owner_ty {
                let n = mv.name();
                if !is_op && !already_granted_infix.contains(n) {
                    already_granted_infix.insert(n);
                    writeln!(w, "impl Infix{ucc} for {n} {{}}")?;
                }
            }
        }

        // todo alias documentation
        write!(w, "impl {module}{ucc}")?;
        if let (TraitArity::Two, Some(var_param)) = (def.arity, var_param) {
            write!(w, "<")?;
            self.write_type(w, *var_param)?;
            write!(w, ">")?;
        }
        write!(w, " for ")?;
        self.write_type(w, *owner_ty)?;
        writeln!(w, " {{")?;
        if let TraitTypeConsensus::Disagreement = output_kind.deref() {
            write!(w, "    type Output = ")?;
            self.write_type(w, output_ty)?;
            writeln!(w, ";")?;
        }

        {
            let stats = impls.statistics;
            let ws = stats.with_simd();
            let wos = stats.without_simd();
            let mut qty_types = 0;
            let mut has_simd = false;
            if !stats.floats.is_zero() {
                qty_types += 1;
            }
            if !stats.simd2.is_zero() {
                qty_types += 1;
                has_simd = true;
            }
            if !stats.simd3.is_zero() {
                qty_types += 1;
                has_simd = true;
            }
            if !stats.simd4.is_zero() {
                qty_types += 1;
                has_simd = true;
            }
            if !wos.is_zero() {
                writeln!(w, "// Operative Statistics for this implementation:")?;
                let space = if qty_types == 1 {
                    if has_simd {
                        "    "
                    } else {
                        ""
                    }
                } else {
                    "     "
                };
                writeln!(w, "//{space}      add/sub      mul      div")?;
            }
            if !stats.floats.is_zero() {
                let f_a = stats.floats.add_sub;
                let f_m = stats.floats.mul;
                let f_d = stats.floats.div;
                let space = if qty_types > 1 { "     " } else { "" };
                writeln!(w, "//{space} f32  {f_a:>7}  {f_m:>7}  {f_d:>7}")?;
            }
            if !stats.simd2.is_zero() {
                let s2_a = stats.simd2.add_sub;
                let s2_m = stats.simd2.mul;
                let s2_d = stats.simd2.div;
                let space = if qty_types > 1 { " " } else { "" };
                writeln!(w, "//{space}   simd2  {s2_a:>7}  {s2_m:>7}  {s2_d:>7}")?;
            }
            if !stats.simd3.is_zero() {
                let s3_a = stats.simd3.add_sub;
                let s3_m = stats.simd3.mul;
                let s3_d = stats.simd3.div;
                let space = if qty_types > 1 { " " } else { "" };
                writeln!(w, "//{space}   simd3  {s3_a:>7}  {s3_m:>7}  {s3_d:>7}")?;
            }
            if !stats.simd4.is_zero() {
                let s4_a = stats.simd4.add_sub;
                let s4_m = stats.simd4.mul;
                let s4_d = stats.simd4.div;
                let space = if qty_types > 1 { " " } else { "" };
                writeln!(w, "//{space}   simd4  {s4_a:>7}  {s4_m:>7}  {s4_d:>7}")?;
            }
            if has_simd {
                let y_a = ws.add_sub;
                let y_m = ws.mul;
                let y_d = ws.div;
                let n_a = wos.add_sub;
                let n_m = wos.mul;
                let n_d = wos.div;
                if qty_types > 1 {
                    writeln!(w, "// Totals...")?;
                    writeln!(w, "// yes simd  {y_a:>7}  {y_m:>7}  {y_d:>7}")?;
                    writeln!(w, "//  no simd  {n_a:>7}  {n_m:>7}  {n_d:>7}")?;
                } else {
                    writeln!(w, "// no simd  {n_a:>7}  {n_m:>7}  {n_d:>7}")?;
                }
            }
        }

        write!(w, "    fn {lsc}(")?;
        match (def.arity, var_param) {
            (TraitArity::Zero, _) => {}
            (TraitArity::One, _) => write!(w, "self")?,
            (TraitArity::Two, Some(other_ty)) => {
                write!(w, "self, other: ")?;
                self.write_type(w, *other_ty)?;
            }
            _ => panic!("Arity 2 should always have other type"),
        }
        write!(w, ") -> ")?;
        match output_kind.deref() {
            TraitTypeConsensus::AlwaysSelf => write!(w, "Self")?,
            TraitTypeConsensus::Disagreement => write!(w, "Self::Output")?,
            TraitTypeConsensus::AllAgree(mv, _) => self.write_type(w, *mv)?,
            TraitTypeConsensus::NoVotes => {
                // Currently, we have no use for traits that do not return values
                bail!("Unsupported or invalid trait def implementation: {ucc} for {owner_ty:?}");
            }
        }
        writeln!(w, " {{")?;
        if impls.statistics.basis_element_struct_access {
            writeln!(w, "        use crate::elements::*;")?;
        }
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
                    let mut no = var_dec.name.1;
                    if no == 0 {
                        write!(w, "let {name} = ")?;
                    } else {
                        no += 1;
                        write!(w, "let {name}_{no} = ")?;
                    }
                    self.write_expression(w, expr)?;
                    writeln!(w, ";")?;
                }
            }
        }
        if let Some(c) = &impls.return_comment {
            self.emit_comment(w, false, c.to_string())?;
        }
        write!(w, "        return ")?;
        self.write_expression(w, &impls.return_expr)?;
        writeln!(w, ";")?;
        writeln!(w, "    }}\n}}")?;

        if !do_assign_impl {
            return Ok(());
        }

        write!(w, "impl {module}{ucc}Assign")?;
        if let (TraitArity::Two, Some(var_param)) = (def.arity, var_param) {
            write!(w, "<")?;
            self.write_type(w, *var_param)?;
            write!(w, ">")?;
        }
        write!(w, " for ")?;
        self.write_type(w, *owner_ty)?;
        writeln!(w, " {{")?;

        write!(w, "    fn {lsc}_assign(")?;
        match (def.arity, var_param) {
            (TraitArity::Zero, _) => {}
            (TraitArity::One, _) => write!(w, "&mut self")?,
            (TraitArity::Two, Some(other_ty)) => {
                write!(w, "&mut self, other: ")?;
                self.write_type(w, *other_ty)?;
            }
            _ => panic!("Arity 2 should always have other type"),
        }
        writeln!(w, ") {{")?;
        if impls.statistics.basis_element_struct_access {
            writeln!(w, "        use crate::elements::*;")?;
        }
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
                    let mut no = var_dec.name.1;
                    if no == 0 {
                        write!(w, "let {name} = ")?;
                    } else {
                        no += 1;
                        write!(w, "let {name}_{no} = ")?;
                    }
                    self.write_expression(w, expr)?;
                    writeln!(w, ";")?;
                }
            }
        }
        if let Some(c) = &impls.return_comment {
            self.emit_comment(w, false, c.to_string())?;
        }
        write!(w, "        *self = ")?;
        self.write_expression(w, &impls.return_expr)?;
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

    async fn format_file<P: AsRef<Path>>(p: P) -> anyhow::Result<()> {
        let mut cmd = Command::new("rustfmt");
        let file_name = p.as_ref().to_string_lossy().to_string();
        cmd.arg(file_name.clone());
        match cmd.spawn() {
            Ok(mut child) => {
                tokio::task::spawn_blocking::<_, anyhow::Result<()>>(move || {
                    match child.wait_with_output() {
                        Ok(o) if o.status.success() => {}
                        Ok(o) => {
                            let stderr = String::from_utf8_lossy(&o.stderr);
                            let stdout = String::from_utf8_lossy(&o.stdout);
                            let e = Err(anyhow!("rustfmt failure: {stderr} {stdout}"))
                                .with_context(move || file_name);
                            return e;
                        }
                        Err(e) => Err(e).with_context(move || file_name)?,
                    }
                    Ok(())
                })
                .await??;
            }
            Err(e) => Err(Error::from(e)).with_context(move || file_name)?,
        }
        Ok(())
    }
}

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs;
use std::io::Write;
use std::ops::Deref;
use std::path::Path;
use std::process::Command;
use std::sync::Arc;

use anyhow::{bail, Error};
use tokio::task::JoinSet;

use crate::algebra2::basis::BasisElement;
use crate::algebra2::basis::grades::{plane_based_k_reflections, point_based_k_reflections};
use crate::algebra2::multivector::{MultiVec, MultiVecRepository};
use crate::ast2::datatype::{ExpressionType, MultiVector};
use crate::ast2::expressions::{AnyExpression, FloatExpr, IntExpr, MultiVectorExpr, MultiVectorGroupExpr, MultiVectorVia, Vec2Expr, Vec3Expr, Vec4Expr};
use crate::ast2::RawVariableDeclaration;
use crate::ast2::traits::{BinaryOps, CommentOrVariableDeclaration, RawTraitDefinition, RawTraitImplementation, TraitArity, TraitImplRegistry, TraitKey, TraitParam, TraitTypeConsensus};
use crate::emit2::{AstEmitter, sort_trait_impls};
use crate::utility::CollectResults;

#[derive(Copy, Clone)]
pub struct Rust {
    pub prefer_fancy_infix: bool,
    pub point_based: bool,
    pub censor_grades: bool,
    pub wgsl: bool,
    pub glsl: bool,
    pub sql: bool,
    pub eq_ord_hash: bool,
    pub nearly_eq_ord: bool,
    pub serde: bool,
}

// TODO f64 support??? maybe???

impl Rust {
    pub async fn write_crate<P: AsRef<Path>>(
        &self,
        crate_folder: P,
        algebra_name: &str,
        version: semver::Version,
        description: &str,
        repository: &str,
        authors: &[&str],
    ) -> anyhow::Result<()> {
        let crate_folder = crate_folder.as_ref().to_path_buf();
        fs::create_dir_all(&crate_folder)?;
        let file_path = crate_folder.join(Path::new("Cargo.toml"));
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&file_path)?;
        let major = version.major;
        let minor = version.minor;
        let patch = version.patch;
        let mut pre = version.pre.as_str().to_string();
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

        write!(&mut file, r#"[package]
name = "{algebra_name}"
version = "{major}.{minor}.{patch}{pre}"
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
"#)?;
        if self.wgsl || self.glsl {
            writeln!(&mut file , r#"naga_oil = {{ version =  "0.13.0", features = ["prune", "glsl"] }}
naga = "0.19.2"
anyhow = "1.0.86"
bytemuck = {{ version = "1.16.3", features = ["derive"] }}
encase = "0.9.0""#)?;
        }
        if self.nearly_eq_ord {
            writeln!(&mut file , r#"nearly = "0.4.0""#)?;
        }
        if self.eq_ord_hash {
            writeln!(&mut file , r#"float-ord = "0.3.2""#)?;
        }
        if self.sql {
            // TODO pgx seems to be a picky builder, so will need close inspection.
            writeln!(&mut file , r#"pgx = "0.7.4"
postgres-types = "0.2.7""#)?;
        }
        if self.serde {
            writeln!(&mut file, r#"serde = {{ version = "1.0.204", features = ["derive"] }}"#)?;
        }

        Ok(())
    }

    pub async fn write_src<P: AsRef<Path>, const AntiScalar: BasisElement>(
        self,
        src_folder: P,
        multi_vecs: Arc<MultiVecRepository<AntiScalar>>,
        impls: Arc<TraitImplRegistry>,
    ) -> anyhow::Result<()> {


        /*
        Copilot Advice:

        Using the include! macro to pull in multiple large files into a single unifying file can impact
        compile times, but it won't be due to the effort of concatenation. The include! macro essentially
        inserts the content of the included files directly into the source file at the point of the macro
        call, so the compiler treats it as if the code were written directly in the file
        https://doc.rust-lang.org/std/macro.include.html.

        However, the overall compile time can be affected by the size and complexity of the generated
        code. When you include many large files, the compiler has to process a significant amount of
        code in a single compilation unit, which can slow down the compilation process. On the other hand,
        using separate modules can help distribute the compilation workload, potentially allowing for
        better parallelism and incremental compilation benefits
        https://corrode.dev/blog/tips-for-faster-rust-compile-times/.

        In summary, while the include! macro itself doesn't add significant overhead, the way you
        structure your code can impact compile times. If you find that compile times are becoming an
        issue, you might want to consider organizing your code into separate modules to take advantage
        of Rust's incremental compilation and parallel processing capabilities
        https://corrode.dev/blog/tips-for-faster-rust-compile-times/.
        */

        /*
        So because of the above advice, we will give each MultiVec its own module after all.

        It just gets a little tedious to fully qualify my_trait_name::MyTraitName and my_vector::MyVector,
        so we will re-export in a more convenient module.
         */

        // TODO create a text file of all created files, so they can be safely deleted in subsequent rounds
        //  and we don't leave junk files around if for example we give a generated variant a tailored name

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
        let impls = impls.get_impls().await;
        let mut mvs = multi_vecs.declarations();
        mvs.sort_by(|a, b| a.name.cmp(&b.name));
        let mvs = mvs;
        let mv_docs = multi_vecs.documentation();


        let mut join_set: JoinSet<anyhow::Result<()>> = JoinSet::new();

        let mut data_mods: BTreeMap<String, Vec<_>> = BTreeMap::new();
        for multi_vec in mvs.iter() {
            let multi_vec = *multi_vec;
            let mv = MultiVector::from(multi_vec);
            let n = mv.name();
            let lsc = TraitKey::new(n).as_lower_snake();
            let folder_data = folder_data.clone();
            let doc = mv_docs.get(&n.to_string()).cloned();
            join_set.spawn(async move {
                let file_path = folder_data.join(Path::new(&lsc)).with_extension("rs");
                let mut file = fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&file_path)?;
                writeln!(&mut file, "use crate::data::*;")?;
                writeln!(&mut file, "use crate::simd::*;")?;
                self.declare_multi_vector(&mut file, multi_vec, doc)?;
                // TODO what if this file is empty? ...nahhh. not after Add, Sub, etc.
                writeln!(&mut file, "include!(\"./impls/{lsc}.rs\");")?;
                self.format_file(&file_path)?;
                Ok(())
            });

            if let Some(gr) = mv.grade() {
                if !self.censor_grades && gr == 1 {
                    data_mods.entry("base".to_string())
                        .and_modify(|v| v.push(multi_vec))
                        .or_insert(vec![multi_vec]);
                }
                let as_gr = AntiScalar.grade();
                if gr > 0 && gr < as_gr {
                    let (m, j) = if self.point_based {
                        ((as_gr - 1) - gr, gr - 1)
                    } else {
                        (gr - 1, (as_gr - 1) - gr)
                    };
                    data_mods.entry(format!("join_{j}"))
                        .and_modify(|v| v.push(multi_vec))
                        .or_insert(vec![multi_vec]);
                    data_mods.entry(format!("meet_{m}"))
                        .and_modify(|v| v.push(multi_vec))
                        .or_insert(vec![multi_vec]);
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
                data_mods.entry(vec_name.to_string())
                    .and_modify(|v| v.push(multi_vec))
                    .or_insert(vec![multi_vec]);
            }
            let k_reflection = if self.point_based {
                point_based_k_reflections::<AntiScalar>()
            } else {
                plane_based_k_reflections()
            }.into_iter()
                .enumerate()
                .find(|(i, it)| it.contains(gr))
                .map(|(i, _)| i);
            if let Some(i) = k_reflection {
                data_mods.entry(format!("reflection_{i}"))
                    .and_modify(|v| v.push(multi_vec))
                    .or_insert(vec![multi_vec]);
            }
        }


        let mut trait_mods: BTreeMap<String, Vec<_>>  = BTreeMap::new();
        for td in defs.iter() {
            let td = td.clone();
            let k = td.names.trait_key;
            let n = k.as_upper_camel();
            let lsc = k.as_lower_snake();
            let arity = td.arity.as_str().to_string();
            trait_mods.entry(arity)
                .and_modify(|v| v.push(td.clone()))
                .or_insert(vec![td.clone()]);
            let folder_traits = folder_traits.clone();
            match n.as_str() {
                "Into" | "TryInto" => continue,
                _ => {}
            }
            join_set.spawn(async move {
                let file_path = folder_traits.join(Path::new(lsc.as_str())).with_extension("rs");
                let mut file = fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&file_path)?;
                writeln!(&mut file, "use crate::data::*;")?;
                writeln!(&mut file, "use crate::simd::*;")?;
                self.declare_trait_def(&mut file, td, fancy_infix)?;
                writeln!(&mut file, "include!(\"./impls/{lsc}.rs\");")?;
                self.format_file(&file_path)?;
                Ok(())
            });
        }

        let mut impl_files: HashMap<String, Vec<Arc<RawTraitImplementation>>> = HashMap::new();

        for i in impls {
            let k = i.definition.names.trait_key;
            let (folder, name) = match k.as_upper_camel().as_str() {
                "Add" | "Sub" | "Mul" | "Div"
                | "Shl" | "Shr" | "BitAnd" | "BitOr" | "BitXor"
                | "Neg" | "Not" => {
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
            impl_files.entry(format!("{folder}/impls/{name}.rs"))
                .and_modify(move |v| v.push(i2))
                .or_insert(vec![i]);
        }

        for (file_path, mut impls) in impl_files {
            let src_folder = src_folder.clone();
            join_set.spawn(async move {
                // TODO actually you should / need to pull in the dependencies and imports
                sort_trait_impls(&mut impls, HashSet::new())?;
                let file_path = src_folder.join(Path::new(file_path.as_str())).with_extension("rs");
                let mut file = fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&file_path)?;
                // writeln!(&mut file, "use crate::data::*;")?;
                // writeln!(&mut file, "use crate::simd::*;")?;
                let mut already_granted_infix = BTreeSet::new();
                for i in impls {
                    let ucc = i.definition.names.trait_key.as_upper_camel();
                    match ucc.as_str() {
                        "Into" => self.write_trait_from(&mut file, i),
                        "TryInto" => self.write_trait_try_from(&mut file, i),
                        _ => self.declare_trait_impl(&mut file, i, &mut already_granted_infix),
                    }?
                }
                self.format_file(&file_path)?;
                Ok(())
            });
        }

        let src_folder2 = src_folder.clone();
        join_set.spawn(async move {
            let file_path = src_folder2.join(Path::new("data.rs"));
            let mut file = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&file_path)?;
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
            self.format_file(&file_path)?;
            Ok(())
        });

        let src_folder2 = src_folder.clone();
        join_set.spawn(async move {
            let file_path = src_folder2.join(Path::new("traits.rs"));
            let mut file = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&file_path)?;
            for (the_mod, ts) in trait_mods {
                writeln!(&mut file, "pub mod {the_mod} {{")?;
                for t in ts {
                    let n = t.names.trait_key;
                    let lsc = n.as_lower_snake();
                    let n = n.as_upper_camel();
                    match n.as_str() {
                        "Into" | "TryInto" => continue,
                        _ => {}
                    }
                    writeln!(&mut file, "pub use crate::traits::{lsc}::{n};")?;
                    if fancy_infix.is_some() {
                        match t.arity {
                            TraitArity::Zero => {}
                            TraitArity::One => {
                                writeln!(&mut file, "pub use crate::traits::{lsc}::{lsc};")?;
                            },
                            TraitArity::Two => {
                                writeln!(&mut file, "pub use crate::traits::{lsc}::{lsc};")?;
                                writeln!(&mut file, "pub use crate::traits::{lsc}::{lsc}_partial;")?;
                            },
                        }
                    }
                }
                writeln!(&mut file, "}}")?;
            }
            for td in defs.iter() {
                let td = td.clone();
                let k = td.names.trait_key;
                let n = k.as_upper_camel();
                let lsc = k.as_lower_snake();
                match n.as_str() {
                    "Into" | "TryInto" => continue,
                    _ => {}
                }
                writeln!(&mut file, "mod {lsc};")?;
                writeln!(&mut file, "pub use {lsc}::{n};")?;
                if fancy_infix.is_some() {
                    match td.arity {
                        TraitArity::Zero => {}
                        TraitArity::One => {
                            writeln!(&mut file, "pub use crate::traits::{lsc}::{lsc};")?;
                        },
                        TraitArity::Two => {
                            writeln!(&mut file, "pub use crate::traits::{lsc}::{lsc};")?;
                            writeln!(&mut file, "pub use crate::traits::{lsc}::{lsc}_partial;")?;
                        },
                    }
                }
            }
            self.format_file(&file_path)?;
            Ok(())
        });

        let src_folder2 = src_folder.clone();
        join_set.spawn(async move {
            let file_path = src_folder2.join(Path::new("lib.rs"));
            let mut file = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&file_path)?;
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
            self.format_file(&file_path)?;
            Ok(())
        });

        let src_folder2 = src_folder.clone();
        join_set.spawn(async move {
            let file_path = src_folder2.join(Path::new("simd.rs"));
            let mut file = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&file_path)?;
            write!(&mut file, "{SIMD_SRC}")?;
            self.format_file(&file_path)?;
            Ok(())
        });

        join_set.collect_results().await
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
            FloatExpr::Literal(l) => {
                if l.fract() == 0.0 {
                    write!(w, "{l:.1}")?;
                } else {
                    write!(w, "{l}")?;
                }
            }
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
                self.write_multi_vec(w, arg)?;
                let method = t.as_lower_snake();
                write!(w, ".{method}()")?;
            }
            FloatExpr::Product(v) => {
                if v.is_empty() {
                    bail!("Attempted to write an empty product that should have been simplified");
                }
                if v.len() > 1 {
                    write!(w, "(")?;
                }
                for (i, e) in v.iter().enumerate() {
                    if i > 0 {
                        write!(w, " * ")?;
                    }
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    self.write_float(w, e)?;
                }
                if v.len() > 1 {
                    write!(w, ")")?;
                }
            }
            FloatExpr::Sum(v) => {
                if v.is_empty() {
                    bail!("Attempted to write an empty sum that should have been simplified");
                }
                if v.len() > 1 {
                    write!(w, "(")?;
                }
                for (i, e) in v.iter().enumerate() {
                    if i > 0 {
                        write!(w, " + ")?;
                    }
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    self.write_float(w, e)?;
                }
                if v.len() > 1 {
                    write!(w, ")")?;
                }
            }
            FloatExpr::Divide(v) => {
                if v.is_empty() {
                    bail!("Attempted to write an empty division that should have been simplified");
                }
                if v.len() > 1 {
                    write!(w, "(")?;
                }
                for (i, e) in v.iter().enumerate() {
                    if i > 0 {
                        write!(w, " / ")?;
                    }
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    // TODO but division is not associative, so I should reconsider if
                    //  FloatExpr::Divide should contain a Vec to begin with
                    self.write_float(w, e)?;
                }
                if v.len() > 1 {
                    write!(w, ")")?;
                }
            }
            FloatExpr::Pow(a, b) => {
                write!(w, "f32::pow(")?;
                self.write_float(w, a.as_ref())?;
                write!(w, ", ")?;
                self.write_float(w, b.as_ref())?;
                write!(w, ")")?;
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
            Vec2Expr::Product(v) => {
                if v.is_empty() {
                    bail!("Attempted to write an empty product that should have been simplified");
                }
                if v.len() > 1 {
                    write!(w, "(")?;
                }
                for (i, e) in v.iter().enumerate() {
                    if i > 0 {
                        write!(w, " * ")?;
                    }
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    self.write_vec2(w, e)?;
                }
                if v.len() > 1 {
                    write!(w, ")")?;
                }
            }
            Vec2Expr::Sum(v) => {
                if v.is_empty() {
                    bail!("Attempted to write an empty sum that should have been simplified");
                }
                if v.len() > 1 {
                    write!(w, "(")?;
                }
                for (i, e) in v.iter().enumerate() {
                    if i > 0 {
                        write!(w, " + ")?;
                    }
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    self.write_vec2(w, e)?;
                }
                if v.len() > 1 {
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
            Vec3Expr::Product(v) => {
                if v.is_empty() {
                    bail!("Attempted to write an empty product that should have been simplified");
                }
                if v.len() > 1 {
                    write!(w, "(")?;
                }
                for (i, e) in v.iter().enumerate() {
                    if i > 0 {
                        write!(w, " * ")?;
                    }
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    self.write_vec3(w, e)?;
                }
                if v.len() > 1 {
                    write!(w, ")")?;
                }
            }
            Vec3Expr::Sum(v) => {
                if v.is_empty() {
                    bail!("Attempted to write an empty sum that should have been simplified");
                }
                if v.len() > 1 {
                    write!(w, "(")?;
                }
                for (i, e) in v.iter().enumerate() {
                    if i > 0 {
                        write!(w, " + ")?;
                    }
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    self.write_vec3(w, e)?;
                }
                if v.len() > 1 {
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
            Vec4Expr::Product(v) => {
                if v.is_empty() {
                    // TODO these sorts of cases should really be handled by making Vec4Expr::Product
                    //  (and other similar Expr types obviously) have an additional element
                    //  that is not inside the Vec, so that it can never be empty. Heck... maybe
                    //  even 2 elements. But we'll see.
                    bail!("Attempted to write an empty product that should have been simplified");
                }
                if v.len() > 1 {
                    write!(w, "(")?;
                }
                for (i, e) in v.iter().enumerate() {
                    if i > 0 {
                        write!(w, " * ")?;
                    }
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    self.write_vec4(w, e)?;
                }
                if v.len() > 1 {
                    write!(w, ")")?;
                }
            }
            Vec4Expr::Sum(v) => {
                if v.is_empty() {
                    bail!("Attempted to write an empty sum that should have been simplified");
                }
                if v.len() > 1 {
                    write!(w, "(")?;
                }
                for (i, e) in v.iter().enumerate() {
                    if i > 0 {
                        write!(w, " + ")?;
                    }
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    self.write_vec4(w, e)?;
                }
                if v.len() > 1 {
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
                self.write_multi_vec(w, arg)?;
                let method = t.as_lower_snake();
                write!(w, ".{method}()")?;
            }
            MultiVectorVia::TraitInvoke21ToClass(t, arg, mv) => {
                self.write_multi_vec(w, arg)?;
                let method = t.as_lower_snake();
                let b = mv.name();
                write!(w, ".{method}::<{b}>()")?;
            }
            MultiVectorVia::TraitInvoke22ToClass(t, a, b) => {
                // TODO fancy infix
                self.write_multi_vec(w, a)?;
                let method = t.as_lower_snake();
                write!(w, ".{method}(")?;
                self.write_multi_vec(w, b)?;
                write!(w, ")")?;
            }
        }
        Ok(())
    }

    fn write_trait_from<W: Write>(
        &self, w: &mut W, impls: Arc<RawTraitImplementation>
    ) -> anyhow::Result<()> {
        let ExpressionType::Class(other) = impls.owner else {
            bail!("Owner of Into (Other of From) impl is not a MultiVector")
        };
        let Some(ExpressionType::Class(owner)) = impls.other_type_params.get(0) else {
            bail!("Other of Into (Owner of From) impl is not a MultiVector")
        };
        let other = other.name();
        let owner = owner.name();
        let lsc = TraitKey::new(other).as_lower_snake();
writeln!(w, r#"
impl From<{other}> for {owner} {{
    fn from({lsc}: {other}) -> Self {{"#)?;
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

    fn write_trait_try_from<W: Write>(
        &self, w: &mut W, impls: Arc<RawTraitImplementation>
    ) -> anyhow::Result<()> {
        let ExpressionType::Class(other) = impls.owner else {
            bail!("Owner of Into (Other of From) impl is not a MultiVector")
        };
        let Some(ExpressionType::Class(owner)) = impls.other_type_params.get(0) else {
            bail!("Other of Into (Owner of From) impl is not a MultiVector")
        };
        let destination_elements: BTreeSet<_> = owner.elements().into_iter().collect();
        let misfit_elements: Vec<_> = other.elements().into_iter().enumerate()
            .filter(|(_, el)| !destination_elements.contains(el)).collect();
        let other = other.name();
        let owner = owner.name();
        let lsc = TraitKey::new(other).as_lower_snake();
        write!(w, r#"
impl TryFrom<{other}> for {owner} {{
    type Error = String;
    fn try_from({lsc}: {other}) -> Result<Self, Self::Error> {{"#)?;
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
            write!(w, r#"
        let el = {lsc}[{i}];
        if el != 0.0 {{
            fail = true;
            error_string.push_str("{el}: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }}"#)?;
        }
        write!(w, r#"
        if fail {{
            let mut error = "Elements from {other} do not fit into {owner} {{ ".to_string();
            error.push_str(error_string.as_str());
            error.push('}}');
            return Err(error);
        }}
        return Ok("#)?;
        self.write_expression(w, &ret)?;
        writeln!(w, ")\n    }}\n}}")?;
        Ok(())
    }
}



impl AstEmitter for Rust {
    fn file_extension() -> &'static str { "rs" }

    fn declare_multi_vector<W: Write, const AntiScalar: BasisElement>(
        &self, w: &mut W, multi_vec: &'static MultiVec<AntiScalar>, docs: Option<String>,
    ) -> anyhow::Result<()> {
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
        for (i, g) in  multi_vec.groups().into_iter().enumerate() {
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

        writeln!(w, r#"
impl {ucc} {{
    pub const LEN: usize = {els_len};
}}"#)?;
        if self.nearly_eq_ord {
            writeln!(w, r#"
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
}}"#)?;
        }
        if self.eq_ord_hash {
            writeln!(w, r#"
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
"#)?;
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

    fn declare_trait_def<W: Write>(
        &self, w: &mut W, def: Arc<RawTraitDefinition>,
        fancy_infix: Option<BinaryOps>
    ) -> anyhow::Result<()> {
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

        if let Some(op) = fancy_infix {
            if let TraitArity::Two = def.arity {
                writeln!(w, "trait Infix{ucc} {{}}")?;
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
                    writeln!(w, "impl<A: {ucc}> {operator_name}Assign<A> for {lsc} {{")?;
                    writeln!(w, "    fn {operator_method}_assign(&mut self, rhs: {lsc}) {{")?;
                    writeln!(w, "        *self = *self.{lsc}()")?;
                    writeln!(w, "    }}\n}}")?;
                }
            }
        }


        Ok(())
    }

    fn declare_trait_impl<W: Write>(
        &self, w: &mut W, impls: Arc<RawTraitImplementation>, already_granted_infix: &mut BTreeSet<&'static str>
    ) -> anyhow::Result<()> {
        let def = &impls.definition;
        let ucc = def.names.trait_key.as_upper_camel();
        let lsc = def.names.trait_key.as_lower_snake();
        let output_kind = def.output.read();
        let output_ty = impls.return_expr.expression_type();
        let owner_ty = &impls.owner;
        if impls.other_var_params.len() > 1 || impls.other_type_params.len() > 1 {
            bail!("We do not support high arity traits yet");
        }
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
                if !already_granted_infix.contains(n) {
                    already_granted_infix.insert(n);
                    writeln!(w, "impl Infix{ucc} for {n} {{}}")?;
                }
            }
        }

        // todo alias documentation
        write!(w, "impl {ucc}")?;
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
        write!(w, "    fn {lsc}(")?;
        match (def.arity, var_param) {
            (TraitArity::Zero, _) => {}
            (TraitArity::One, _) => write!(w, "self")?,
            (TraitArity::Two, Some(other_ty)) => {
                write!(w, "self, other: ")?;
                self.write_type(w, *other_ty)?;
            }
            _ => panic!("Arity 2 should always have other type")
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
        Ok(())
    }

    fn emit_comment<W: Write, S: Into<String>>(
        &self, w: &mut W, is_documentation: bool, s: S
    ) -> anyhow::Result<()> {
        let slashy = if is_documentation { "/// " } else { "// " };
        let s = s.into();
        let comment = s.trim();
        if comment.is_empty() {
            writeln!(w, "\n{slashy}")?;
            return Ok(())
        }
        let mut comment_iter = comment.split("\n")
            .map(|it| it.trim())
            .skip_while(|it| it.is_empty())
            .peekable();
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

    fn format_file<P: AsRef<Path>>(&self, p: P) -> anyhow::Result<()> {
        let mut cmd = Command::new("rustfmt");
        cmd.arg(p.as_ref().to_string_lossy().to_string());
        match cmd.spawn() {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::from(e))
        }
    }
}
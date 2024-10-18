use std::collections::BTreeSet;
use std::fs;
use std::io::Write;
use std::ops::Deref;
use std::path::Path;
use std::sync::Arc;
use anyhow::bail;
use indicatif::ProgressFinish;
use crate::algebra2::basis::BasisElement;
use crate::algebra2::multivector::{MultiVec, MultiVecRepository};
use crate::ast2::datatype::{ExpressionType, MultiVector};
use crate::ast2::expressions::{AnyExpression, Expression, FloatExpr, IntExpr, MultiVectorExpr, MultiVectorGroupExpr, MultiVectorVia, Vec2Expr, Vec3Expr, Vec4Expr};
use crate::ast2::traits::{CommentOrVariableDeclaration, progress_style, RawTraitImplementation, TraitArity, TraitImplRegistry, TraitKey, TraitParam, TraitTypeConsensus};

// Generate WGSL shader source code
#[derive(Copy, Clone)]
pub struct Wgsl {
    // TODO support this feature.
    //  Why have this feature?
    //  It'd just be interesting to see if there is any performance difference after all.
    //  Supposedly GPUs use mostly scalar processing these days. So doing SIMD style
    //  vector addition and multiplication might not actually help on the GPU. And if that's true,
    //  then a more flat data structure without the layers of vec2, vec3, and vec4 will
    //  make for a nicer API to use in application shader code.
    pub just_scalars_no_vectors: bool,
    // TODO support this feature
    pub emit_comments: bool,
}


impl Wgsl {
    pub fn new() -> Self {
        Wgsl {
            just_scalars_no_vectors: false,
            emit_comments: false,
        }
    }

    pub fn write_shader_file<P: AsRef<Path>, const AntiScalar: BasisElement>(
        self,
        src_folder: P,
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
        // So... are we really going to put all the contents into a single source file?
        // It probably sounds crazy at first glance, but the answer is yes.
        // The reason it sounds crazy can be considered from several steps back though, not just
        // for wgsl and shaders, but the rust code generation too.
        //
        // So, the real question is, why is it okay to have several megabyte source files? Or in the
        // case of putting EVERYTHING into one source file like right here, perhaps even gigabytes?
        // Won't things break at some point? Shouldn't there be a limit?
        //
        // The answer is yes. The limit at which things break is unknown at the time of writing.
        // But if you don't put a limit on the number of dimensions of your algebra, the number of
        // multivector types, and the number of traits you want to define, then you're going to have
        // explosive quantities of code. If you don't hold yourself back, then maybe some
        // compiler or file system or VCS will choke at some point. Our purpose here isn't to tell
        // you what that limit should be.... our purpose here is to generate the code, and it's up
        // to you to define your algebra with sane limits that won't inconvenience you or break
        // your build pipeline.
        //
        // Q: So is there a good motivation for having so much code to begin with?
        // A: Yes, as when you 'facet' multivector types down to narrower types, you can get
        //    much more precise geometric information on the type of object you are working with,
        //    and also can avoid unnecessary floating point math that is supposed to cancel to zero.
        //    So what you get is a quadratic quantity of arity 2 trait implementations with respect
        //    to the quantity of multivector types. The quadratic quantity might seem crazy for a
        //    source file, but after it is compiled, you can be sure it is reasonably efficient
        //    floating point math, and every single unused trait implementation will simply not
        //    be included in your binary.
        //
        // Q: HOWEVER, DOESN'T THIS RELY ON THE ASSUMPTION THAT THE CODE IS COMPILED???
        // A: Yes, absolutely. Ideally, compiled ahead of time by a developer, not just-in-time on
        //    some client's machine. It would be utterly insane to send megabytes of javascript
        //    source over the network, or ask a python consumer to compile gigabytes from scratch
        //    every time. So what about shaders? USUALLY and HISTORICALLY shaders are compiled on
        //    the client machine (if I understand correctly, I'm still pretty new to shaders).
        //    However, these days we have some fancy new-ish technology. That is, we can use naga
        //    to compile shaders down to SPIRV. And we can use naga_oil to prune unused content
        //    before doing so. Therefore, by putting the entire algebra into a single source file,
        //    it makes it easy to compile an application's shader with a single dependency (and let
        //    naga_oil sort out the details), rather than wrangling a bajillion files and trying to
        //    plug them into naga_oil manually and hopefully not forget something or mess something
        //    up. In any case, as long as naga_oil doesn't choke, then gigabytes of "shader library
        //    source" can be stripped down to only the parts you use in your "shader application
        //    source", and the final SPIRV artifact should be very reasonably sized to ship to end
        //    users.
        //
        // Q: So what is the limit at which naga_oil will break?
        //    (Not to mention VCS, filesystems, IDEs)
        // A: I don't know, but some daring (courageous? foolhardy?) soul creating algebras here is
        //    likely to be the one who finds out. Or if we never find that limit? Then even better,
        //    and kudos to the team working on naga_oil (and naga too, of course).

        // Alright, enough excuses for this insanity, let's get started

        let src_folder = src_folder.as_ref().to_path_buf();
        let folder_integrations = src_folder.join(Path::new("integrations"));
        fs::create_dir_all(&folder_integrations)?;

        let file_name = format!("{algebra_name}.wgsl");
        let file_path = folder_integrations.join(Path::new(file_name.as_str()));

        let e: anyhow::Result<()> = try {
            let mut file = fs::OpenOptions::new().truncate(true).write(true).open(&file_path)?;
            let mut pre = version_pre.to_string();
            if !pre.is_empty() {
                pre = format!("-{pre}");
            }
            writeln!(&mut file, r#"

#define_import_path {algebra_name}

//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

// v{version_major}.{version_minor}.{version_patch}{pre}
// {description}
// authors = ["Andrew Brown <Andrew.Brown.UNL@gmail.com>", "{additional_authors}]
// License: MIWT

"#)?;
            let mut mvs = multi_vecs.declarations();
            mvs.sort_by(|a, b| a.name.cmp(&b.name));
            let mvs = mvs;

            let multi_progress = Arc::new(indicatif::MultiProgress::new());
            let qty_mvs = mvs.len() as u64;
            let data_pb = Arc::new(multi_progress.add(indicatif::ProgressBar::new(qty_mvs).with_finish(ProgressFinish::AndLeave)));
            data_pb.set_style(progress_style());
            data_pb.set_message("WGSL - Data Definitions");

            let qty_impls = impls.len() as u64;
            let impls_pb = Arc::new(multi_progress.add(indicatif::ProgressBar::new(qty_impls)));
            impls_pb.set_style(progress_style());
            impls_pb.set_message("WGSL - Trait Implementations");

            for multi_vec in mvs.iter() {
                self.declare_multi_vector(&mut file, *multi_vec)?;
                data_pb.inc(1);
            }

            for i in impls {
                self.declare_trait_impl(&mut file, i)?;
                impls_pb.inc(1);
            }

            Ok(())
        };
        if let Err(e) = e {
            panic!("WGSL Errors: {e:?}");
        }
    }

    fn declare_multi_vector<W: Write, const AntiScalar: BasisElement>(
        &self, w: &mut W, multi_vec: &'static MultiVec<AntiScalar>
    ) -> anyhow::Result<()> {
        let name = TraitKey::new(multi_vec.name);
        let ucc = name.as_upper_camel();
        writeln!(w, "struct {ucc} {{")?;
        for (i, g) in multi_vec.groups().into_iter().enumerate() {
            if i > 0 {
                writeln!(w, ",")?;
            }
            write!(w, "    // ")?;
            let mut g = g.into_vec();
            for (i, el) in g.clone().into_iter().enumerate() {
                if i > 0 {
                    write!(w, ", ")?;
                }
                write!(w, "{el}")?;
            }
            let l = g.len();
            write!(w, "\n    g{i}_: vec{l}<f32>")?;
        }
        writeln!(w, "}}")?;
        Ok(())
    }

    fn declare_trait_impl<W: Write>(
        &self, w: &mut W, impls: Arc<RawTraitImplementation>
    ) -> anyhow::Result<()> {
        let def = &impls.definition;
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
        let owner_ty = &impls.owner;

        let trait_ucc = def.names.trait_key.as_upper_camel();
        let trait_lcc = def.names.trait_key.as_lower_camel();
        if impls.other_var_params.len() > 1 || impls.other_type_params.len() > 1 {
            bail!("We do not support high arity traits yet");
        }
        write!(w, "fn ")?;
        self.write_type(w, *owner_ty, false)?;
        write!(w, "_{trait_lcc}")?;
        if let (TraitArity::Two, Some(other_ty)) = (def.arity, var_param) {
            write!(w, "_")?;
            self.write_type(w, *other_ty, false)?;
        }
        write!(w, "(")?;
        match (def.arity, var_param) {
            (TraitArity::Zero, _) => {}
            (TraitArity::One, _) => {
                write!(w, "self: ")?;
                self.write_type(w, *owner_ty, true)?
            },
            (TraitArity::Two, Some(other_ty)) => {
                write!(w, "self: ")?;
                self.write_type(w, *owner_ty, true)?;
                write!(w, ", other: ")?;
                self.write_type(w, *other_ty, true)?
            }
            _ => panic!("Arity 2 should always have other type"),
        }
        writeln!(w, ") -> ")?;
        let output_ty = impls.return_expr.expression_type();
        self.write_type(w, output_ty, true)?;
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
                    let mut no = var_dec.name.1;
                    if no == 0 {
                        write!(w, "    let {name}: ")?;
                    } else {
                        no += 1;
                        write!(w, "    let {name}_{no}: ")?;
                    }
                    let x = expr.read().deref();
                    let x_ty = x.expression_type();
                    self.write_type(w, x_ty, true)?;
                    write!(w, " = ")?;
                    self.write_expression(w, x)?;
                    writeln!(w, ";")?;
                }
            }
        }
        if let Some(c) = &impls.return_comment {
            self.emit_comment(w, false, c.to_string())?;
        }
        write!(w, "    return ")?;
        self.write_expression(w, &impls.return_expr)?;
        writeln!(w, ";")?;
        writeln!(w, "}}")?;
        Ok(())
    }

    fn write_type<W: Write>(&self, w: &mut W, data_type: ExpressionType, upper_camel_case: bool) -> anyhow::Result<()> {
        match data_type {
            ExpressionType::Int(i) => write!(w, "i32")?,
            ExpressionType::Float(f) => write!(w, "f32")?,
            ExpressionType::Vec2(v) => write!(w, "vec2<f32>")?,
            ExpressionType::Vec3(v) => write!(w, "vec3<f32>")?,
            ExpressionType::Vec4(v) => write!(w, "vec4<f32>")?,
            ExpressionType::Class(mv) => {
                let n = mv.name();
                if upper_camel_case {
                    write!(w, "{n}")?;
                } else {
                    let n = TraitKey::new(n);
                    let lcc = n.as_lower_camel();
                    write!(w, "{lcc}")?;
                }
            }
        }
        Ok(())
    }

    fn emit_comment<W: Write, S: Into<String>>(&self, w: &mut W, is_documentation: bool, s: S) -> anyhow::Result<()> {
        if !self.emit_comments {
            return Ok(())
        }
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
                let method = t.as_lower_camel();
                let arg_type = TraitKey::new(mv.name()).as_lower_camel();
                write!(w, "{arg_type}_{method}()")?;
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
            FloatExpr::FromInt(i) => {
                write!(w, "(")?;
                self.write_int(w, i)?;
                write!(w, " as f32)")?;
            }
            FloatExpr::AccessVec2(v, i) => {
                self.write_vec2(w, v.as_ref())?;
                match *i {
                    0 => write!(w, ".x")?,
                    1 => write!(w, ".y")?,
                    _ => bail!("index {i} is outside range of vec2")
                }
            }
            FloatExpr::AccessVec3(v, i) => {
                self.write_vec3(w, v.as_ref())?;
                match *i {
                    0 => write!(w, ".x")?,
                    1 => write!(w, ".y")?,
                    2 => write!(w, ".z")?,
                    _ => bail!("index {i} is outside range of vec2")
                }
            }
            FloatExpr::AccessVec4(v, i) => {
                self.write_vec4(w, v.as_ref())?;
                match *i {
                    0 => write!(w, ".x")?,
                    1 => write!(w, ".y")?,
                    2 => write!(w, ".z")?,
                    3 => write!(w, ".w")?,
                    _ => bail!("index {i} is outside range of vec2")
                }
            }
            FloatExpr::AccessMultiVecGroup(mv, i) => {
                self.write_multi_vec(w, mv)?;
                let mut i = *i;
                write!(w, ".g{i}_")?;
            }
            FloatExpr::AccessMultiVecFlat(mv, i) => {
                self.write_multi_vec(w, mv)?;
                let mut i = *i as usize;
                for (group_index, group) in mv.mv_class.groups().iter().enumerate() {
                    let group_size = group.into_vec().len();
                    if i < group_size {
                        let inner_index = match i {
                            0 => "x",
                            1 => "y",
                            2 => "z",
                            3 => "w",
                            _ => unreachable!("i should be less than group_size which is always 4 or less")
                        };
                        write!(w, ".g{group_index}_.{inner_index}")?;
                        break;
                    }
                    i = i - group_size;
                }
            }
            FloatExpr::TraitInvoke11ToFloat(t, arg) => {
                let method = t.as_lower_camel();
                let arg_type = TraitKey::new(arg.expression_type().name()).as_lower_camel();
                write!(w, "{arg_type}_{method}(")?;
                self.write_multi_vec(w, arg)?;
                write!(w, ")")?;
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
                                write!(w, "pow(")?;
                                self.write_float(w, factor)?;
                                write!(w, ", {e})")?;
                            } else {
                                write!(w, "pow(")?;
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
                                write!(w, " * pow(")?;
                                self.write_float(w, factor)?;
                                write!(w, ", {e})")?;
                            } else {
                                write!(w, " * pow(")?;
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
            FloatExpr::Exp(factor, exponent, last_exponent) => {

                if exponent.is_none() && last_exponent.fract() == 0.0 {
                    write!(w, "pow(")?;
                } else {
                    write!(w, "pow(")?;
                }
                self.write_float(w, factor)?;
                write!(w, ", ")?;
                if let Some(exponent) = exponent {
                    self.write_float(w, exponent)?;
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
                write!(w, "vec2<f32>(")?;
                self.write_float(w, f)?;
                write!(w, ")")?;
            }
            Vec2Expr::Gather2(f0, f1) => {
                write!(w, "vec2<f32>(")?;
                self.write_float(w, f0)?;
                write!(w, ", ")?;
                self.write_float(w, f1)?;
                write!(w, ")")?;
            }
            Vec2Expr::AccessMultiVecGroup(mv, i) => {
                self.write_multi_vec(w, mv)?;
                write!(w, ".g{i}_")?;
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
                            write!(w, "(vec2<f32>(1.0) / ")?;
                            self.write_vec2(w, factor)?;
                            write!(w, ")")?;
                        }
                        (e, false) => {
                            if e.fract() == 0.0 && e <= i32::MAX as f32 && e >= i32::MIN as f32 {
                                let e = e as i32;
                                write!(w, "pow(")?;
                                self.write_vec2(w, factor)?;
                                write!(w, ", vec2<f32>({e}))")?;
                            } else {
                                write!(w, "pow(")?;
                                self.write_vec2(w, factor)?;
                                write!(w, ", vec2<f32>({e}))")?;
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
                                write!(w, " * pow(")?;
                                self.write_vec2(w, factor)?;
                                write!(w, ", vec2<f32>({e}))")?;
                            } else {
                                write!(w, " * pow(")?;
                                self.write_vec2(w, factor)?;
                                write!(w, ", vec2<f32>({e}))")?;
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
                        write!(w, "vec2<f32>(")?;
                        self.write_f32(w, a)?;
                        write!(w, ")")?;
                    } else {
                        write!(w, "vec2<f32>(")?;
                        self.write_f32(w, a)?;
                        write!(w, ", ")?;
                        self.write_f32(w, b)?;
                        write!(w, ")")?;
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
                            write!(w, "vec2<f32>(")?;
                            self.write_f32(w, f)?;
                            write!(w, ")*")?;
                        }

                        (1.0, true) => write!(w, " + ")?,
                        (-1.0, true) => write!(w, " - ")?,
                        (f, true) if f > 0.0 => {
                            write!(w, " + vec2<f32>(")?;
                            self.write_f32(w, f)?;
                            write!(w, ")*")?;
                        }
                        (f, true) if f < 0.0 => {
                            let f = -f;
                            write!(w, " - vec2<f32>(")?;
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
                        write!(w, "vec2<f32>(")?;
                        self.write_f32(w, a0)?;
                        write!(w, ")")?;
                    } else {
                        write!(w, "vec2<f32>(")?;
                        self.write_f32(w, a0)?;
                        write!(w, ", ")?;
                        self.write_f32(w, a1)?;
                        write!(w, ")")?;
                    }
                }
                if len > 1 {
                    write!(w, ")")?;
                }
            }
            Vec2Expr::SwizzleVec2(box v, i0, i1) => {
                self.write_vec2(w, v)?;
                let x = match *i0 {
                    0 => "x",
                    1 => "y",
                    _ => bail!("swizzle index out of bounds")
                };
                let y = match *i1 {
                    0 => "x",
                    1 => "y",
                    _ => bail!("swizzle index out of bounds")
                };
                write!(w, ".{x}{y}")?;
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
                write!(w, "vec3<f32>(")?;
                self.write_float(w, f)?;
                write!(w, ")")?;
            }
            Vec3Expr::Gather3(f0, f1, f2) => {
                write!(w, "vec3<f32>(")?;
                self.write_float(w, f0)?;
                write!(w, ", ")?;
                self.write_float(w, f1)?;
                write!(w, ", ")?;
                self.write_float(w, f2)?;
                write!(w, ")")?;
            }
            Vec3Expr::AccessMultiVecGroup(mv, i) => {
                self.write_multi_vec(w, mv)?;
                write!(w, ".g{i}_")?;
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
                            write!(w, "(vec3<f32>(1.0) / ")?;
                            self.write_vec3(w, factor)?;
                            write!(w, ")")?;
                        }
                        (e, false) => {
                            if e.fract() == 0.0 && e <= i32::MAX as f32 && e >= i32::MIN as f32 {
                                let e = e as i32;
                                write!(w, "pow(")?;
                                self.write_vec3(w, factor)?;
                                write!(w, ", vec3<f32>({e}))")?;
                            } else {
                                write!(w, "pow(")?;
                                self.write_vec3(w, factor)?;
                                write!(w, ", vec3<f32>({e}))")?;
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
                                write!(w, " * pow(")?;
                                self.write_vec3(w, factor)?;
                                write!(w, ", vec3<f32>({e}))")?;
                            } else {
                                write!(w, " * pow(")?;
                                self.write_vec3(w, factor)?;
                                write!(w, ", vec3<f32>({e}))")?;
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
                        write!(w, "vec3<f32>(")?;
                        self.write_f32(w, a)?;
                        write!(w, ")")?;
                    } else {
                        write!(w, "vec3<f32>(")?;
                        self.write_f32(w, a)?;
                        write!(w, ", ")?;
                        self.write_f32(w, b)?;
                        write!(w, ", ")?;
                        self.write_f32(w, c)?;
                        write!(w, ")")?;
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
                            write!(w, "vec3<f32>(")?;
                            self.write_f32(w, f)?;
                            write!(w, ")*")?;
                        }

                        (1.0, true) => write!(w, " + ")?,
                        (-1.0, true) => write!(w, " - ")?,
                        (f, true) if f > 0.0 => {
                            write!(w, " + vec3<f32>(")?;
                            self.write_f32(w, f)?;
                            write!(w, ")*")?;
                        }
                        (f, true) if f < 0.0 => {
                            let f = -f;
                            write!(w, " - vec3<f32>(")?;
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
                        write!(w, "vec3<f32>(")?;
                        self.write_f32(w, a)?;
                        write!(w, ")")?;
                    } else {
                        write!(w, "vec3<f32>(")?;
                        self.write_f32(w, a)?;
                        write!(w, ", ")?;
                        self.write_f32(w, b)?;
                        write!(w, ", ")?;
                        self.write_f32(w, c)?;
                        write!(w, ")")?;
                    }
                }
                if len > 1 {
                    write!(w, ")")?;
                }
            }
            Vec3Expr::SwizzleVec3(box v, i0, i1, i2) => {
                self.write_vec3(w, v)?;
                let x = match *i0 {
                    0 => "x",
                    1 => "y",
                    2 => "z",
                    _ => bail!("swizzle index out of bounds")
                };
                let y = match *i1 {
                    0 => "x",
                    1 => "y",
                    2 => "z",
                    _ => bail!("swizzle index out of bounds")
                };
                let z = match *i2 {
                    0 => "x",
                    1 => "y",
                    2 => "z",
                    _ => bail!("swizzle index out of bounds")
                };
                write!(w, ".{x}{y}{z}")?;
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
                write!(w, "vec3<f32>(")?;
                self.write_float(w, f)?;
                write!(w, ")")?;
            }
            Vec4Expr::Gather4(f0, f1, f2, f3) => {
                write!(w, "vec3<f32>(")?;
                self.write_float(w, f0)?;
                write!(w, ", ")?;
                self.write_float(w, f1)?;
                write!(w, ", ")?;
                self.write_float(w, f2)?;
                write!(w, ", ")?;
                self.write_float(w, f3)?;
                write!(w, ")")?;
            }
            Vec4Expr::AccessMultiVecGroup(mv, i) => {
                self.write_multi_vec(w, mv)?;
                write!(w, ".g{i}_")?;
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
                            write!(w, "(vec4<f32>(1.0) / ")?;
                            self.write_vec4(w, factor)?;
                            write!(w, ")")?;
                        }
                        (e, false) => {
                            if e.fract() == 0.0 && e <= i32::MAX as f32 && e >= i32::MIN as f32 {
                                let e = e as i32;
                                write!(w, "pow(")?;
                                self.write_vec4(w, factor)?;
                                write!(w, ", vec4<f32>({e}))")?;
                            } else {
                                write!(w, "pow(")?;
                                self.write_vec4(w, factor)?;
                                write!(w, ", vec4<f32>({e}))")?;
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
                                write!(w, " * pow(")?;
                                self.write_vec4(w, factor)?;
                                write!(w, ", vec4<f32>({e}))")?;
                            } else {
                                write!(w, " * pow(")?;
                                self.write_vec4(w, factor)?;
                                write!(w, ", vec4<f32>({e}))")?;
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
                        write!(w, "vec4<f32>(")?;
                        self.write_f32(w, a)?;
                        write!(w, ")")?;
                    } else {
                        write!(w, "vec4<f32>(")?;
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
                            write!(w, "vec4<f32>(")?;
                            self.write_f32(w, f)?;
                            write!(w, ")*")?;
                        }

                        (1.0, true) => write!(w, " + ")?,
                        (-1.0, true) => write!(w, " - ")?,
                        (f, true) if f > 0.0 => {
                            write!(w, " + vec4<f32>(")?;
                            self.write_f32(w, f)?;
                            write!(w, ")*")?;
                        }
                        (f, true) if f < 0.0 => {
                            let f = -f;
                            write!(w, " - vec4<f32>(")?;
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
                        write!(w, "vec4<f32>(")?;
                        self.write_f32(w, a)?;
                        write!(w, ")")?;
                    } else {
                        write!(w, "vec4<f32>(")?;
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
                if len > 1 {
                    write!(w, ")")?;
                }
            }
            Vec4Expr::SwizzleVec4(box v, i0, i1, i2, i3) => {
                self.write_vec4(w, v)?;
                let x = match *i0 {
                    0 => "x",
                    1 => "y",
                    2 => "z",
                    3 => "w",
                    _ => bail!("swizzle index out of bounds")
                };
                let y = match *i1 {
                    0 => "x",
                    1 => "y",
                    2 => "z",
                    3 => "w",
                    _ => bail!("swizzle index out of bounds")
                };
                let z = match *i2 {
                    0 => "x",
                    1 => "y",
                    2 => "z",
                    3 => "w",
                    _ => bail!("swizzle index out of bounds")
                };
                let w = match *i3 {
                    0 => "x",
                    1 => "y",
                    2 => "z",
                    3 => "w",
                    _ => bail!("swizzle index out of bounds")
                };
                write!(w, ".{x}{y}{z}{w}")?;
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
                write!(w, "{n}(")?;
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
                let method = t.as_lower_camel();
                let arg_type = TraitKey::new(arg.expression_type().name()).as_lower_camel();
                write!(w, "{arg_type}_{method}(")?;
                self.write_multi_vec(w, arg)?;
                write!(w, ")")?;
            }
            MultiVectorVia::TraitInvoke21ToClass(t, arg, mv) => {
                let method = t.as_lower_camel();
                let a_type = TraitKey::new(arg.expression_type().name()).as_lower_camel();
                let b_type = TraitKey::new(mv.name()).as_lower_camel();
                write!(w, "{a_type}_{method}_{b_type}(")?;
                self.write_multi_vec(w, arg)?;
                write!(w, ")")?;
            }
            MultiVectorVia::TraitInvoke22ToClass(t, a, b) => {
                let method = t.as_lower_camel();
                let a_type = TraitKey::new(a.expression_type().name()).as_lower_camel();
                let b_type = TraitKey::new(a.expression_type().name()).as_lower_camel();
                write!(w, "{a_type}_{method}_{b_type}(")?;
                self.write_multi_vec(w, a)?;
                write!(w, ", ")?;
                self.write_multi_vec(w, b)?;
                write!(w, ")")?;
            }
            MultiVectorVia::TraitInvoke12iToClass(t, a, b) => {
                let method = t.as_lower_camel();
                let a_type = TraitKey::new(a.expression_type().name()).as_lower_camel();
                let b_type = TraitKey::new(a.expression_type().name()).as_lower_camel();
                write!(w, "{a_type}_{method}_{b_type}(")?;
                self.write_multi_vec(w, a)?;
                write!(w, ", ")?;
                self.write_int(w, b)?;
                write!(w, ")")?;
            }
            MultiVectorVia::TraitInvoke12fToClass(t, a, b) => {
                let method = t.as_lower_camel();
                let a_type = TraitKey::new(a.expression_type().name()).as_lower_camel();
                let b_type = TraitKey::new(a.expression_type().name()).as_lower_camel();
                write!(w, "{a_type}_{method}_{b_type}(")?;
                self.write_multi_vec(w, a)?;
                write!(w, ", ")?;
                self.write_float(w, b)?;
                write!(w, ")")?;
            }
        }
        Ok(())
    }
}

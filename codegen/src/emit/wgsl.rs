use std::collections::{HashSet};
use std::fs;
use std::io::{BufWriter, Write};
use std::ops::Deref;
use std::path::Path;
use std::sync::Arc;
use anyhow::{bail};
use indicatif::ProgressFinish;
use crate::algebra::basis::BasisElement;
use crate::algebra::multivector::{MultiVec, MultiVecRepository};
use crate::ast::datatype::{ExpressionType};
use crate::ast::expressions::{AnyExpression, Expression, FloatExpr, IntExpr, MultiVectorExpr, MultiVectorGroupExpr, MultiVectorVia, Vec2Expr, Vec3Expr, Vec4Expr};
use crate::ast::traits::{CommentOrVariableDeclaration, progress_style, RawTraitImplementation, TraitArity, TraitImplRegistry, TraitKey};
use crate::emit::sort_trait_impls;

// Awesome function reference:
// https://webgpufundamentals.org/webgpu/lessons/webgpu-wgsl-function-reference.html
// TODO add AST support for more of the above functions. If there's a good reason for it, I guess.


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


// Pruner error.... adding an expression.... Expression::Compose which I think is struct construction....
// It seems to imply I'm providing 3 arguments to a 2 parameter function.
// in fn dualNum_wedge_antiPlane
// if I'm digging into the memory correctly (FUCK debugging arenas), it's constructing RoundPointGroups
// And that seems accurate to RoundPoints having 2 groups, not 3...
// However there is no RoundPointGroups in fn dualNum_wedge_antiPlane
// So do I have the wrong function/
// maybe it is invoking the function? and composing the RoundPoint inside the args....

// fs_main
/*
fs_main
    add_entrypoint
    add_function_ref
    add_block
    add_statement (call?)
    add_function (roundPoint_unitizedRadiusNormSquared)
roundPoint_unitizedRadiusNormSquared
    add_function_ref
    add_block
    add_statement
    add_function (roundPoint_roundWeightNormSquared)
roundPoint_roundWeightNormSquared
    add_function_ref
    add_block
    add_statement
    add_function (dualNum_wedge_antiPlane)
dualNum_wedge_antiPlane
    add_function_ref
    add_block
    add_statement (call function, can't quite pick out which one)
        well... since DipoleGroups is a struct, it's probably dipole_degroup
    add_expression (Compose 3 expressions)
    add_expression (Binary op multiply)
    add_expression (Compose 2 expressions)
        this caries the same exact &PartReq from the above expressions...
        until it's in a compose again, where it does "sub-requirements" or whatever

This helps narrow it down
There is only one multiply that is a direct child of compose 3 in dualNum_wedge_antiPlane
and inside that multiply, there is only one compose 2

So the problem seems to be with expressions like this: vec4<f32>(vec3<f32>(self_.e4_), 0.0)

So where does the PartReq get out of sync with the actual composition structure?
add_statement
    PartReq is immutable borrow from input_context
    input_context is owned value
    input_context might come from self.add_function(...), or might come from func_req.exprs_required, or both
    func_req.exprs_required[handle expression matching 'result'] so yeah that's a thing
    then from self.add_function(...) that uses func_req.body_required
    so func_req.body_required and func_req.exprs_required
    func_req is mutable
    we get a func_req from above, but also create one from scratch in add_function_ref
    it would seem the req for arguments to called function is derived in body_required (see add_function)
    so I'm looking for 'context' field on BlockReq returned from add_block
    that iterates through statements, passing in an &mut RequiredContext
    so then we're in add_statement again, but looking at the mutable context variable now
    goes into add_expression.... back to compose....
UGh
I'm just going to try to work around it.... not ideal.

vec4<f32>(vec3<f32>(self_.e4_), 0.0)
I can only guess that the PartReq determination gets confused after it sees vec3

*/


impl Wgsl {
    pub fn new() -> Self {
        Wgsl {
            just_scalars_no_vectors: false,
            emit_comments: false,
        }
    }

    #[allow(non_upper_case_globals)]
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
        let folder_integrations = src_folder.join(Path::new("integrations/wgsl"));
        let mut mvs = multi_vecs.declarations();
        mvs.sort_by(|a, b| a.name.cmp(&b.name));
        let mvs = mvs;
        let rt = tokio::runtime::Runtime::new().expect("Tokio must work");
        let mut impls = rt.block_on(async {
            impls.get_impls().await
        });

        let file_name = format!("{algebra_name}.wgsl");
        let file_path = folder_integrations.join(Path::new(file_name.as_str()));
        let mut additional_authors = String::new();
        for a in authors {
            additional_authors.push_str(", \"");
            additional_authors.push_str(a);
            additional_authors.push('"');
        }

        let e: anyhow::Result<()> = try {
            fs::create_dir_all(&folder_integrations)?;
            let file = fs::OpenOptions::new().create(true).truncate(true).write(true).open(&file_path)?;
            let mut file = BufWriter::new(file);
            let mut pre = version_pre.to_string();
            if !pre.is_empty() {
                pre = format!("-{pre}");
            }
            writeln!(&mut file, r#"#define_import_path {algebra_name}
//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// {repository}
//
// v{version_major}.{version_minor}.{version_patch}{pre}
// {description}
// authors = ["Andrew Brown <Andrew.Brown.UNL@gmail.com>"{additional_authors}]
// License: MIWT
//"#)?;

            let multi_progress = Arc::new(indicatif::MultiProgress::new());
            let qty_mvs = mvs.len() as u64;
            let data_pb = Arc::new(multi_progress.add(indicatif::ProgressBar::new(qty_mvs).with_finish(ProgressFinish::AndLeave)));
            data_pb.set_style(progress_style());
            data_pb.set_message("WGSL - Data Definitions");

            let qty_impls = impls.len() as u64;
            // let impls_pb = Arc::new(multi_progress.add(indicatif::ProgressBar::new(qty_impls).with_finish(ProgressFinish::AndLeave)));
            // impls_pb.set_style(progress_style());
            // impls_pb.set_message("WGSL - Trait Implementations");

            for multi_vec in mvs.iter() {
                self.declare_multi_vector(&mut file, *multi_vec)?;
                data_pb.inc(1);
            }


            write!(&mut file, r#"
// The naga_oil pruner has a bug where it can't handle these expressions inlined
// so we have to wrap the operation in a function so it doesn't choke.
// It's an index out of bounds (3 exceeding 2, or 4 exceeding 3)
// in Pruner.add_expression -> match Expression::Compose -> match PartReq::Part.
// Obviously we want an issue/PR fix in naga_oil but the PartReq tracking is
// insufferable to read and/or debug, so I'm procrastinating that.
fn extendVec2toVec3(v: vec2<f32>, z: f32) -> vec3<f32> {{
    // return vec3<f32>(v, z);
    return vec3<f32>(v[0], v[1], z);
}}
fn extendVec2toVec4(v: vec2<f32>, z: f32, w: f32) -> vec4<f32> {{
    // return vec4<f32>(v, z, w);
    return vec4<f32>(v[0], v[1], z, w);
}}
fn extendVec3toVec4(v: vec2<f32>, w: f32) -> vec4<f32> {{
    // return vec4<f32>(v, w);
    return vec4<f32>(v[0], v[1], v[2], w);
}}
            "#)?;

            // sort_trait_impls(&mut impls, HashSet::new())?;
            // for i in impls {
            //     self.declare_trait_impl(&mut file, i)?;
            //     impls_pb.inc(1);
            // }
        };
        if let Err(e) = e {
            panic!("WGSL Errors: {e:?}");
        }
    }

    #[allow(non_upper_case_globals)]
    fn declare_multi_vector<W: Write, const AntiScalar: BasisElement>(
        &self, w: &mut W, multi_vec: &'static MultiVec<AntiScalar>
    ) -> anyhow::Result<()> {
        let name = TraitKey::new(multi_vec.name);
        let ucc = name.as_upper_camel();
        let lcc = name.as_lower_camel();
        write!(w, "\nstruct {ucc} {{\n    ")?;
        for (i, g) in multi_vec.groups().into_iter().enumerate() {
            if i > 0 {
                write!(w, ",\n    ")?;
            }
            let g = g.into_vec();
            for (i, el) in g.clone().into_iter().enumerate() {
                if i > 0 {
                    write!(w, ", ")?;
                }
                let suffix = if let Some(char) = format!("{el}").chars().last() {
                    if char.is_numeric() {
                        "_"
                    } else { "" }
                } else { "" };
                write!(w, "{el}{suffix}: f32")?;
            }
        }
        writeln!(w, "\n}}")?;


        writeln!(w, "struct {ucc}Groups {{")?;
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
            if l > 1 {
                write!(w, "\n    group{i}_: vec4<f32>")?;
            } else {
                write!(w, "\n    group{i}_: vec4<f32>")?;
            }
        }
        writeln!(w, "\n}}")?;



        writeln!(w, "fn {lcc}_grouped(self_: {ucc}) -> {ucc}Groups {{")?;
        write!(w, "    return {ucc}Groups(\n        ")?;
        for (i, g) in multi_vec.groups().into_iter().enumerate() {
            if i > 0 {
                write!(w, ",\n        ")?;
            }
            write!(w, "vec4<f32>(")?;
            let g = g.into_vec();
            let l = g.len();
            for (i, el) in g.clone().into_iter().enumerate() {
                if i > 0 {
                    write!(w, ", ")?;
                }
                let suffix = if let Some(char) = format!("{el}").chars().last() {
                    if char.is_numeric() {
                        "_"
                    } else { "" }
                } else { "" };
                write!(w, "self_.{el}{suffix}")?;
            }
            match l {
                1 => write!(w, ", 0.0, 0.0, 0.0")?,
                2 => write!(w, ", 0.0, 0.0")?,
                3 => write!(w, ", 0.0")?,
                _ => (),
            }
            write!(w, ")")?;
        }
        writeln!(w, "\n    );")?;
        writeln!(w, "}}")?;


        writeln!(w, "fn {lcc}_degroup(self_: {ucc}Groups) -> {ucc} {{")?;
        write!(w, "    return {ucc}(\n        ")?;
        for (i, g) in multi_vec.groups().into_iter().enumerate() {
            if i > 0 {
                write!(w, ",\n        ")?;
            }
            let g = g.into_vec();
            for (j, _) in g.clone().into_iter().enumerate() {
                if j > 0 {
                    write!(w, ", ")?;
                }
                let j = match j {
                    0 => "x",
                    1 => "y",
                    2 => "z",
                    3 => "w",
                    _ => unreachable!("simd vecs max length of 4")
                };
                write!(w, "self_.group{i}_.{j}")?;
            }
        }
        writeln!(w, "\n    );")?;
        writeln!(w, "}}\n")?;

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
        let mut type_param = None;
        if !impls.other_type_params.is_empty() {
            type_param = Some(impls.other_type_params[0]);
        }
        let owner_ty = &impls.owner;

        let trait_lcc = def.names.trait_key.as_lower_camel();

        // Unsupported traits in wgsl - no algebraic data types
        if trait_lcc == "tryInto" {
            return Ok(());
        }

        if impls.other_var_params.len() > 1 || impls.other_type_params.len() > 1 {
            bail!("We do not support high arity traits yet");
        }
        write!(w, "fn ")?;
        self.write_type(w, *owner_ty, false, false)?;
        write!(w, "_{trait_lcc}")?;
        if trait_lcc == "into" {
            let other_ty = type_param.expect("into always has a type param, even if only 1 arg");
            write!(w, "_")?;
            self.write_type(w, other_ty, false, false)?;
        }
        if let (TraitArity::Two, Some(other_ty)) = (def.arity, var_param) {
            write!(w, "_")?;
            self.write_type(w, *other_ty, false, false)?;
        }
        write!(w, "(")?;
        match (def.arity, var_param) {
            (TraitArity::Zero, _) => {}
            (TraitArity::One, _) => {
                write!(w, "self_: ")?;
                self.write_type(w, *owner_ty, true, false)?
            },
            (TraitArity::Two, Some(other_ty)) => {
                write!(w, "self_: ")?;
                self.write_type(w, *owner_ty, true, false)?;
                write!(w, ", other: ")?;
                self.write_type(w, *other_ty, true, false)?
            }
            _ => panic!("Arity 2 should always have other type"),
        }
        write!(w, ") -> ")?;
        let output_ty = impls.return_expr.expression_type();
        self.write_type(w, output_ty, true, false)?;
        writeln!(w, " {{")?;
        match (def.arity, var_param) {
            (TraitArity::Zero, _) => {}
            (TraitArity::One, _) => {
                let needs_group_var = impls.statistics.mv_vars_access_grouped.contains(&("self".to_string(), 0));
                if needs_group_var {
                    write!(w, "    let self_groups = ")?;
                    self.write_type(w, *owner_ty, false, false)?;
                    writeln!(w, "_grouped(self_);")?;
                }
            },
            (TraitArity::Two, Some(other_ty)) => {
                let needs_group_var = impls.statistics.mv_vars_access_grouped.contains(&("self".to_string(), 0));
                if needs_group_var {
                    write!(w, "    let self_groups = ")?;
                    self.write_type(w, *owner_ty, false, false)?;
                    writeln!(w, "_grouped(self_);")?;
                }
                let needs_group_var = impls.statistics.mv_vars_access_grouped.contains(&("other".to_string(), 0));
                if needs_group_var {
                    write!(w, "    let other_groups = ")?;
                    self.write_type(w, *other_ty, false, false)?;
                    writeln!(w, "_grouped(other);")?;
                }
            }
            _ => panic!("Arity 2 should always have other type"),
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
                    let no = var_dec.name.1;
                    let guard = expr.read();
                    let x = guard.deref();
                    let x_ty = x.expression_type();

                    let mut is_multivector_var = false;
                    let mut is_multivector_expr_grouped = false;
                    if let AnyExpression::Class(mve) = &x {
                        is_multivector_var = true;
                        if let box MultiVectorVia::Construct(_) = &mve.expr {
                            is_multivector_expr_grouped = true;
                        }
                    }

                    let needs_flat_var = impls.statistics.mv_vars_access_flat.contains(&(name.clone(), no));
                    let needs_group_var = impls.statistics.mv_vars_access_grouped.contains(&(name.clone(), no));
                    let needs_second_var = needs_group_var && needs_flat_var;


                    let g = if is_multivector_expr_grouped && needs_group_var { "_groups" } else { "" };
                    if no == 0 {
                        write!(w, "    let {name}{g}: ")?;
                    } else {
                        let no = no + 1;
                        write!(w, "    let {name}{g}_{no}: ")?;
                    }
                    if needs_second_var {
                        self.write_type(w, x_ty, true, is_multivector_expr_grouped)?;
                    } else {
                        self.write_type(w, x_ty, true, needs_group_var)?;
                    }
                    write!(w, " = ")?;
                    if needs_second_var || (needs_group_var == is_multivector_expr_grouped) {
                        self.write_expression(w, x, true, false, is_multivector_expr_grouped)?;
                    } else {
                        self.write_expression(w, x, true, false, needs_group_var)?;
                    }
                    writeln!(w, ";")?;

                    if is_multivector_var && needs_second_var {
                        let g = if !is_multivector_expr_grouped { "_groups" } else { "" };
                        if no == 0 {
                            write!(w, "    let {name}{g}: ")?;
                        } else {
                            let no = no + 1;
                            write!(w, "    let {name}{g}_{no}: ")?;
                        }
                        self.write_type(w, x_ty, true, !is_multivector_expr_grouped)?;
                        write!(w, " = ")?;
                        self.write_type(w, x_ty, false, false)?;
                        if is_multivector_expr_grouped {
                            write!(w, "_degroup(")?;
                        } else {
                            write!(w, "_grouped(")?;
                        }
                        let g = if is_multivector_expr_grouped { "_groups" } else { "" };
                        if no == 0 {
                            write!(w, "{name}{g}")?;
                        } else {
                            let no = no + 1;
                            write!(w, "{name}{g}_{no}")?;
                        }
                        writeln!(w, ");")?;
                    }
                }
            }
        }
        if let Some(c) = &impls.return_comment {
            self.emit_comment(w, false, c.to_string())?;
        }
        write!(w, "    return ")?;
        self.write_expression(w, &impls.return_expr, true, false, false)?;
        writeln!(w, ";")?;
        writeln!(w, "}}")?;
        Ok(())
    }

    fn write_type<W: Write>(&self, w: &mut W, data_type: ExpressionType, upper_camel_case: bool, multivector_grouped: bool) -> anyhow::Result<()> {
        match data_type {
            ExpressionType::Int(_) => write!(w, "i32")?,
            ExpressionType::Float(_) => write!(w, "f32")?,
            ExpressionType::Vec2(_) => write!(w, "vec4<f32>")?,
            ExpressionType::Vec3(_) => write!(w, "vec4<f32>")?,
            ExpressionType::Vec4(_) => write!(w, "vec4<f32>")?,
            ExpressionType::Class(mv) => {
                let n = mv.name();
                let g = if multivector_grouped { "Groups" } else { "" };
                if upper_camel_case {
                    write!(w, "{n}{g}")?;
                } else {
                    let n = TraitKey::new(n);
                    let lcc = n.as_lower_camel();
                    write!(w, "{lcc}{g}")?;
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

    fn write_expression<W: Write>(&self, w: &mut W, expr: &AnyExpression, grouping_provided: bool, f32_as_vec4: bool, multivector_grouped: bool) -> anyhow::Result<()> {
        match expr {
            AnyExpression::Int(e) => self.write_int(w, e)?,
            AnyExpression::Float(e) => self.write_float(w, e, grouping_provided, f32_as_vec4)?,
            AnyExpression::Vec2(e) => self.write_vec2(w, e, grouping_provided)?,
            AnyExpression::Vec3(e) => self.write_vec3(w, e, grouping_provided)?,
            AnyExpression::Vec4(e) => self.write_vec4(w, e, grouping_provided)?,
            AnyExpression::Class(e) => self.write_multi_vec(w, e, multivector_grouped)?,
        }
        Ok(())
    }

    fn write_int<W: Write>(&self, w: &mut W, expr: &IntExpr) -> anyhow::Result<()> {
        match expr {
            IntExpr::Variable(v) => {
                let name = &v.decl.name.0;
                let no = v.decl.name.1;
                if no == 0 {
                    if name.as_str() == "self" {
                        write!(w, "self_")?;
                    } else {
                        write!(w, "{name}")?;
                    }
                } else {
                    let no = no + 1;
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

    fn write_float<W: Write>(&self, w: &mut W, expr: &FloatExpr, grouping_provided: bool, as_vec4: bool) -> anyhow::Result<()> {
        let grouping_provided = grouping_provided || as_vec4;
        if as_vec4 {
            write!(w, "vec4<f32>(")?;
        }
        match expr {
            FloatExpr::Variable(v) => {
                let name = &v.decl.name.0;
                let no = v.decl.name.1;
                if no == 0 {
                    if name.as_str() == "self" {
                        write!(w, "self_")?;
                    } else {
                        write!(w, "{name}")?;
                    }
                } else {
                    let no = no + 1;
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
                self.write_vec2(w, v.as_ref(), false)?;
                match *i {
                    0 => write!(w, ".x")?,
                    1 => write!(w, ".y")?,
                    _ => bail!("index {i} is outside range of vec2")
                }
            }
            FloatExpr::AccessVec3(v, i) => {
                self.write_vec3(w, v.as_ref(), false)?;
                match *i {
                    0 => write!(w, ".x")?,
                    1 => write!(w, ".y")?,
                    2 => write!(w, ".z")?,
                    _ => bail!("index {i} is outside range of vec3")
                }
            }
            FloatExpr::AccessVec4(v, i) => {
                self.write_vec4(w, v.as_ref(), false)?;
                match *i {
                    0 => write!(w, ".x")?,
                    1 => write!(w, ".y")?,
                    2 => write!(w, ".z")?,
                    3 => write!(w, ".w")?,
                    _ => bail!("index {i} is outside range of vec4")
                }
            }
            FloatExpr::AccessMultiVecGroup(mv, i) => {
                self.write_multi_vec(w, mv, true)?;
                let i = *i;
                write!(w, ".group{i}_")?;
            }
            FloatExpr::AccessMultiVecFlat(mv, i) => {
                self.write_multi_vec(w, mv, false)?;
                let mut i = *i as usize;
                for group in mv.mv_class.groups().iter() {
                    let group_size = group.into_vec().len();
                    if i < group_size {
                        let el = group.into_vec()[i];
                        let suffix = if let Some(char) = format!("{el}").chars().last() {
                            if char.is_numeric() {
                                "_"
                            } else { "" }
                        } else { "" };
                        write!(w, ".{el}{suffix}")?;
                        break;
                    }
                    i = i - group_size;
                }
            }
            FloatExpr::TraitInvoke11ToFloat(t, arg) => {
                let method = t.as_lower_camel();
                let arg_type = TraitKey::new(arg.expression_type().name()).as_lower_camel();
                write!(w, "{arg_type}_{method}(")?;
                self.write_multi_vec(w, arg, false)?;
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
                if len > 1 && !grouping_provided {
                    write!(w, "(")?;
                }
                for (i, (factor, exponent)) in v.iter().enumerate() {
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    match (*exponent, i > 0) {
                        (f, _) if f == 0.0 => continue,

                        (1.0, false) => self.write_float(w, factor, false, false)?,
                        (-1.0, false) => {
                            if !grouping_provided {
                                write!(w, "(")?;
                            }
                            write!(w, "1.0/(")?;
                            self.write_float(w, factor, true, false)?;
                            write!(w, ")")?;
                            if !grouping_provided {
                                write!(w, ")")?;
                            }
                        }
                        (e, false) => {
                            if e.fract() == 0.0 && e <= i32::MAX as f32 && e >= i32::MIN as f32 {
                                let e = e as i32;
                                write!(w, "pow(")?;
                                self.write_float(w, factor, true, false)?;
                                write!(w, ", {e})")?;
                            } else {
                                write!(w, "pow(")?;
                                self.write_float(w, factor, true, false)?;
                                write!(w, ", {e})")?;
                            }
                        }

                        (1.0, true) => {
                            write!(w, " * ")?;
                            self.write_float(w, factor, false, false)?;
                        }
                        (-1.0, true) => {
                            write!(w, " / (")?;
                            self.write_float(w, factor, true, false)?;
                            write!(w, ")")?;
                        }
                        (e, true) => {
                            if e.fract() == 0.0 && e <= i32::MAX as f32 && e >= i32::MIN as f32 {
                                let e = e as i32;
                                write!(w, " * pow(")?;
                                self.write_float(w, factor, true, false)?;
                                write!(w, ", {e})")?;
                            } else {
                                write!(w, " * pow(")?;
                                self.write_float(w, factor, true, false)?;
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
                    self.write_float(w, addend, false, false)?;
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
                    write!(w, "pow(")?;
                } else {
                    write!(w, "pow(")?;
                }
                self.write_float(w, factor, true, false)?;
                write!(w, ", ")?;
                if let Some(exponent) = exponent {
                    self.write_float(w, exponent, *last_exponent == 1.0, false)?;
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
        if as_vec4 {
            write!(w, ", 0.0, 0.0, 0.0)")?;
        }
        Ok(())
    }

    fn write_vec2<W: Write>(&self, w: &mut W, expr: &Vec2Expr, grouping_provided: bool) -> anyhow::Result<()> {
        match expr {
            Vec2Expr::Variable(v) => {
                let name = &v.decl.name.0;
                let no = v.decl.name.1;
                if no == 0 {
                    if name.as_str() == "self" {
                        write!(w, "self_")?;
                    } else {
                        write!(w, "{name}")?;
                    }
                } else {
                    let no = no + 1;
                    write!(w, "{name}_{no}")?;
                }
            }
            Vec2Expr::Gather1(f) => {
                if let FloatExpr::Literal(0.0) = f {
                    write!(w, "vec4<f32>(0.0)")?;
                } else {
                    // naga_oil Pruner bug if we try to only invoke the expression only once
                    // (in case it is not a trivial expression) but also try to keep z and w as 0

                    // write!(w, "vec4<f32>(vec2<f32>(")?;
                    // self.write_float(w, f, true, false)?;
                    // write!(w, "), 0.0, 0.0)")?;

                    // So we'll just allow z and w to be non-zero until we can file an issue
                    // to naga_oil and/or fix properly
                    write!(w, "(vec4<f32>(")?;
                    self.write_float(w, f, true, false)?;
                    write!(w, ") * vec4<f32>(1.0, 1.0, 0.0, 0.0))")?;
                }
            }
            Vec2Expr::Gather2(f0, f1) => {
                write!(w, "vec4<f32>(")?;
                self.write_float(w, f0, true, false)?;
                write!(w, ", ")?;
                self.write_float(w, f1, true, false)?;
                write!(w, ", 0.0, 0.0)")?;
            }
            Vec2Expr::AccessMultiVecGroup(mv, i) => {
                self.write_multi_vec(w, mv, true)?;
                write!(w, ".group{i}_")?;
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
                            write!(w, "(vec4<f32>(1.0) / ")?;
                            self.write_vec2(w, factor, false)?;
                            write!(w, ")")?;
                        }
                        (e, false) => {
                            if e.fract() == 0.0 && e <= i32::MAX as f32 && e >= i32::MIN as f32 {
                                let e = e as i32;
                                write!(w, "pow(")?;
                                self.write_vec2(w, factor, true)?;
                                write!(w, ", vec4<f32>({e}))")?;
                            } else {
                                write!(w, "pow(")?;
                                self.write_vec2(w, factor, true)?;
                                write!(w, ", vec4<f32>({e}))")?;
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
                                write!(w, " * pow(")?;
                                self.write_vec2(w, factor, true)?;
                                write!(w, ", vec4<f32>({e}))")?;
                            } else {
                                write!(w, " * pow(")?;
                                self.write_vec2(w, factor, true)?;
                                write!(w, ", vec4<f32>({e}))")?;
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
                        write!(w, "vec4<f32>(")?;
                        self.write_f32(w, a)?;
                        write!(w, ")")?;
                    } else {
                        write!(w, "vec4<f32>(")?;
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
                    self.write_vec2(w, addend, false)?;
                }
                let a0 = last_addend[0];
                let a1 = last_addend[1];
                if *last_addend != [0.0; 2] {
                    if len > 1 {
                        write!(w, " + ")?;
                    }
                    if a0 == a1 {
                        write!(w, "vec4<f32>(")?;
                        self.write_f32(w, a0)?;
                        write!(w, ")")?;
                    } else {
                        write!(w, "vec4<f32>(")?;
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
                write!(w, ".{x}{y}zw")?;
            }
            Vec2Expr::Truncate3to2(box v3) => {
                self.write_vec3(w, v3, false)?;
            }
            Vec2Expr::Truncate4to2(box v4) => {
                self.write_vec4(w, v4, false)?;
            }
        }
        Ok(())
    }

    fn write_vec3<W: Write>(&self, w: &mut W, expr: &Vec3Expr, grouping_provided: bool) -> anyhow::Result<()> {
        match expr {
            Vec3Expr::Variable(v) => {
                let name = &v.decl.name.0;
                let no = v.decl.name.1;
                if no == 0 {
                    if name.as_str() == "self" {
                        write!(w, "self_")?;
                    } else {
                        write!(w, "{name}")?;
                    }
                } else {
                    let no = no + 1;
                    write!(w, "{name}_{no}")?;
                }
            }
            Vec3Expr::Gather1(f) => {
                if let FloatExpr::Literal(0.0) = f {
                    write!(w, "vec4<f32>(0.0)")?;
                } else {
                    // naga_oil Pruner bug if we try to only invoke the expression only once
                    // (in case it is not a trivial expression) but also try to keep w as 0

                    // write!(w, "vec4<f32>(vec3<f32>(")?;
                    // self.write_float(w, f, true, false)?;
                    // write!(w, "), 0.0)")?;

                    // So we'll just allow w to be non-zero until we can file an issue
                    // to naga_oil and/or fix properly
                    write!(w, "(vec4<f32>(")?;
                    self.write_float(w, f, true, false)?;
                    write!(w, ") * vec4<f32>(1.0, 1.0, 1.0, 0.0))")?;
                }
            }
            Vec3Expr::Gather3(f0, f1, f2) => {
                write!(w, "vec4<f32>(")?;
                self.write_float(w, f0, true, false)?;
                write!(w, ", ")?;
                self.write_float(w, f1, true, false)?;
                write!(w, ", ")?;
                self.write_float(w, f2, true, false)?;
                write!(w, ", 0.0)")?;
            }
            Vec3Expr::Extend2to3(v2, f1) => {
                // write!(w, "vec4<f32>(")?;
                // self.write_vec2(w, v2, false)?;
                // write!(w, ".xy, ")?;
                // self.write_float(w, f1, true, false)?;
                // write!(w, ", 0.0)")?;

                write!(w, "extendVec2toVec3(")?;
                self.write_vec2(w, v2, false)?;
                write!(w, ".xy, ")?;
                self.write_float(w, f1, true, false)?;
                write!(w, ", 0.0)")?;
            }
            Vec3Expr::AccessMultiVecGroup(mv, i) => {
                self.write_multi_vec(w, mv, true)?;
                write!(w, ".group{i}_")?;
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
                            write!(w, "(vec4<f32>(1.0) / ")?;
                            self.write_vec3(w, factor, false)?;
                            write!(w, ")")?;
                        }
                        (e, false) => {
                            if e.fract() == 0.0 && e <= i32::MAX as f32 && e >= i32::MIN as f32 {
                                let e = e as i32;
                                write!(w, "pow(")?;
                                self.write_vec3(w, factor, true)?;
                                write!(w, ", vec4<f32>({e}))")?;
                            } else {
                                write!(w, "pow(")?;
                                self.write_vec3(w, factor, true)?;
                                write!(w, ", vec4<f32>({e}))")?;
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
                                write!(w, " * pow(")?;
                                self.write_vec3(w, factor, true)?;
                                write!(w, ", vec4<f32>({e}))")?;
                            } else {
                                write!(w, " * pow(")?;
                                self.write_vec3(w, factor, true)?;
                                write!(w, ", vec4<f32>({e}))")?;
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
                write!(w, ".{x}{y}{z}w")?;
            }
            Vec3Expr::Truncate4to3(box v4) => {
                self.write_vec4(w, v4, false)?;
            }
        }
        Ok(())
    }

    fn write_vec4<W: Write>(&self, w: &mut W, expr: &Vec4Expr, grouping_provided: bool) -> anyhow::Result<()> {
        match expr {
            Vec4Expr::Variable(v) => {
                let name = &v.decl.name.0;
                let no = v.decl.name.1;
                if no == 0 {
                    if name.as_str() == "self" {
                        write!(w, "self_")?;
                    } else {
                        write!(w, "{name}")?;
                    }
                } else {
                    let no = no + 1;
                    write!(w, "{name}_{no}")?;
                }
            }
            Vec4Expr::Gather1(f) => {
                write!(w, "vec4<f32>(")?;
                self.write_float(w, f, true, false)?;
                write!(w, ")")?;
            }
            Vec4Expr::Gather4(f0, f1, f2, f3) => {
                write!(w, "vec4<f32>(")?;
                self.write_float(w, f0, true, false)?;
                write!(w, ", ")?;
                self.write_float(w, f1, true, false)?;
                write!(w, ", ")?;
                self.write_float(w, f2, true, false)?;
                write!(w, ", ")?;
                self.write_float(w, f3, true, false)?;
                write!(w, ")")?;
            }
            Vec4Expr::Extend2to4(v2, f1, f2) => {


                // TODO problem... that's making me very fed up with naga_oil.
                // fn roundPoint_roundWeight(self_: RoundPoint) -> RoundPoint {
                //     return roundPoint_degroup(RoundPointGroups(
                //         /* e1, e2, e3, e4 */ vec4<f32>(vec4<f32>(0.0).xyz, self_.e4_),
                //         /* e5 */ vec4<f32>(0.0, 0.0, 0.0, 0.0)
                //     ));
                // }
                // TODO it breaks on vec4<f32>(vec3<f32>(...), w)
                //  I'm fricken sick of babying naga_oil, and having to use a debugger to figure
                //  out what is wrong, and the debugging to be horrific torture because they're
                //  using arenas everywhere. So.... instead of this shit.... I'm going to try
                //  to side step my frustration and work on slang integration!
                //  Valid wgsl should be valid wgsl and I'm sick of it not being so.
                //  The fact we have to naga_oil compose and prune shit to begin with is beyond
                //  the pale. This is 2024. We deserve an actually competent shading language
                //  capable of being organized and modular. So here we go with slang.


                // write!(w, "vec4<f32>(")?;
                // self.write_vec2(w, v2, false)?;
                // write!(w, ".xy, ")?;
                // self.write_float(w, f1, true, false)?;
                // write!(w, ", ")?;
                // self.write_float(w, f2, true, false)?;
                // write!(w, ")")?;

                write!(w, "extendVec2toVec4(")?;
                self.write_vec2(w, v2, false)?;
                write!(w, ".xy, ")?;
                self.write_float(w, f1, true, false)?;
                write!(w, ", ")?;
                self.write_float(w, f2, true, false)?;
                write!(w, ")")?;
            }
            Vec4Expr::Extend3to4(v2, f1) => {
                // write!(w, "vec4<f32>(")?;
                // self.write_vec3(w, v2, false)?;
                // write!(w, ".xyz, ")?;
                // self.write_float(w, f1, true, false)?;
                // write!(w, ")")?;

                write!(w, "extendVec3toVec4(")?;
                self.write_vec3(w, v2, false)?;
                write!(w, ".xyz, ")?;
                self.write_float(w, f1, true, false)?;
                write!(w, ")")?;
            }
            Vec4Expr::AccessMultiVecGroup(mv, i) => {
                self.write_multi_vec(w, mv, true)?;
                write!(w, ".group{i}_")?;
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
                            write!(w, "(vec4<f32>(1.0) / ")?;
                            self.write_vec4(w, factor, false)?;
                            write!(w, ")")?;
                        }
                        (e, false) => {
                            if e.fract() == 0.0 && e <= i32::MAX as f32 && e >= i32::MIN as f32 {
                                let e = e as i32;
                                write!(w, "pow(")?;
                                self.write_vec4(w, factor, true)?;
                                write!(w, ", vec4<f32>({e}))")?;
                            } else {
                                write!(w, "pow(")?;
                                self.write_vec4(w, factor, true)?;
                                write!(w, ", vec4<f32>({e}))")?;
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
                                write!(w, " * pow(")?;
                                self.write_vec4(w, factor, true)?;
                                write!(w, ", vec4<f32>({e}))")?;
                            } else {
                                write!(w, " * pow(")?;
                                self.write_vec4(w, factor, true)?;
                                write!(w, ", vec4<f32>({e}))")?;
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

    // TODO fn horizon_geometricQuotient_scalar
    //  it'd be nice to skip constructing by groups, and construct flat instead. Sometimes.

    fn write_multi_vec<W: Write>(&self, w: &mut W, expr: &MultiVectorExpr, grouped: bool) -> anyhow::Result<()> {
        let mv = expr.mv_class;
        let n = mv.name();
        let tk = TraitKey::new(n);
        let ucc = tk.as_upper_camel();
        let lcc = tk.as_lower_camel();
        match &*expr.expr {
            MultiVectorVia::Variable(v) => {
                let name = &v.decl.name.0;
                let no = v.decl.name.1;
                let g = if grouped { "_groups" } else { "" };
                if no == 0 {
                    if name.as_str() == "self" {
                        if grouped {
                            write!(w, "self{g}")?;
                        } else {
                            write!(w, "self_")?;
                        }
                    } else {
                        write!(w, "{name}{g}")?;
                    }
                } else {
                    let no = no + 1;
                    write!(w, "{name}{g}_{no}")?;
                }
            }
            MultiVectorVia::Construct(v) => {
                if !grouped && mv.groups().into_iter().all(|it| it.simd_width() == 1) {
                    write!(w, "{ucc}(")?;
                    for (i, g) in v.iter().enumerate() {
                        if i > 0 {
                            write!(w, ", ")?;
                        }
                        match g {
                            MultiVectorGroupExpr::JustFloat(f) => self.write_float(w, f, true, false)?,
                            _ => unreachable!("We checked that there are only length 1 groups")
                        }
                    }
                    write!(w, ")")?;
                    return Ok(())
                }

                if !grouped {
                    write!(w, "{lcc}_degroup(")?;
                }
                write!(w, "{ucc}Groups(")?;
                let groups = mv.groups();
                for (i, g) in v.iter().enumerate() {
                    if i > 0 {
                        write!(w, ", ")?;
                    }
                    write!(w, "\n        /* ")?;
                    for (i, el) in groups[i].into_vec().into_iter().enumerate() {
                        if i > 0 {
                            write!(w, ", ")?;
                        }
                        write!(w, "{el}")?;
                    }
                    write!(w, " */ ")?;
                    match g {
                        MultiVectorGroupExpr::JustFloat(f) => self.write_float(w, f, true, true)?,
                        MultiVectorGroupExpr::Vec2(g) => {
                            // write!(w, "vec4<f32>(1.0, 1.0, 0.0, 0.0) * (")?;
                            self.write_vec2(w, g, true)?;
                            // write!(w, ")")?;
                        },
                        MultiVectorGroupExpr::Vec3(g) => {
                            // write!(w, "vec4<f32>(1.0, 1.0, 1.0, 0.0) * (")?;
                            self.write_vec3(w, g, true)?;
                            // write!(w, ")")?;
                        },
                        MultiVectorGroupExpr::Vec4(g) => self.write_vec4(w, g, true)?,
                    }
                }
                write!(w, "\n    )")?;
                if !grouped {
                    write!(w, ")")?;
                }
            }
            MultiVectorVia::TraitInvoke11ToClass(t, arg) => {
                let method = t.as_lower_camel();
                let arg_type = TraitKey::new(arg.expression_type().name()).as_lower_camel();
                if grouped {
                    write!(w, "{lcc}_grouped(")?;
                }
                write!(w, "{arg_type}_{method}(")?;
                self.write_multi_vec(w, arg, false)?;
                write!(w, ")")?;
                if grouped {
                    write!(w, ")")?;
                }
            }
            MultiVectorVia::TraitInvoke21ToClass(t, arg, mv) => {
                let method = t.as_lower_camel();
                let a_type = TraitKey::new(arg.expression_type().name()).as_lower_camel();
                let b_type = TraitKey::new(mv.name()).as_lower_camel();
                if grouped {
                    write!(w, "{lcc}_grouped(")?;
                }
                write!(w, "{a_type}_{method}_{b_type}(")?;
                self.write_multi_vec(w, arg, false)?;
                write!(w, ")")?;
                if grouped {
                    write!(w, ")")?;
                }
            }
            MultiVectorVia::TraitInvoke22ToClass(t, a, b) => {
                let method = t.as_lower_camel();
                let a_type = TraitKey::new(a.expression_type().name()).as_lower_camel();
                let b_type = TraitKey::new(b.expression_type().name()).as_lower_camel();
                if grouped {
                    write!(w, "{lcc}_grouped(")?;
                }
                write!(w, "{a_type}_{method}_{b_type}(")?;
                self.write_multi_vec(w, a, false)?;
                write!(w, ", ")?;
                self.write_multi_vec(w, b, false)?;
                write!(w, ")")?;
                if grouped {
                    write!(w, ")")?;
                }
            }
            MultiVectorVia::TraitInvoke12iToClass(t, a, b) => {
                let method = t.as_lower_camel();
                let a_type = TraitKey::new(a.expression_type().name()).as_lower_camel();
                let b_type = TraitKey::new(a.expression_type().name()).as_lower_camel();
                if grouped {
                    write!(w, "{lcc}_grouped(")?;
                }
                write!(w, "{a_type}_{method}_{b_type}(")?;
                self.write_multi_vec(w, a, false)?;
                write!(w, ", ")?;
                self.write_int(w, b)?;
                write!(w, ")")?;
                if grouped {
                    write!(w, ")")?;
                }
            }
            MultiVectorVia::TraitInvoke12fToClass(t, a, b) => {
                let method = t.as_lower_camel();
                let a_type = TraitKey::new(a.expression_type().name()).as_lower_camel();
                let b_type = TraitKey::new(a.expression_type().name()).as_lower_camel();
                if grouped {
                    write!(w, "{lcc}_grouped(")?;
                }
                write!(w, "{a_type}_{method}_{b_type}(")?;
                self.write_multi_vec(w, a, false)?;
                write!(w, ", ")?;
                self.write_float(w, b, true, false)?;
                write!(w, ")")?;
                if grouped {
                    write!(w, ")")?;
                }
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
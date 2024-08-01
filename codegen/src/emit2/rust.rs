use std::io::Write;
use std::ops::Deref;
use std::path::Path;
use std::process::Command;
use std::sync::Arc;

use anyhow::{bail, Error};

use crate::algebra2::basis::BasisElement;
use crate::algebra2::multivector::MultiVec;
use crate::ast2::datatype::ExpressionType;
use crate::ast2::expressions::{AnyExpression, FloatExpr, IntExpr, MultiVectorExpr, MultiVectorGroupExpr, MultiVectorVia, Vec2Expr, Vec3Expr, Vec4Expr};
use crate::ast2::traits::{CommentOrVariableDeclaration, RawTraitDefinition, RawTraitImplementation, TraitArity, TraitKey, TraitTypeConsensus};
use crate::emit2::{AstEmitter, IdentifierQualifier};

#[derive(Copy, Clone)]
pub struct Rust {
    pub prefer_fancy_infix: bool,
}


impl Rust {
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
                write!(w, "{l}")?;
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
                write!(w, ".group{i}()")?;
            }
            FloatExpr::AccessMultiVecFlat(mv, i) => {
                self.write_multi_vec(w, mv)?;
                write!(w, "[{i}]")?;
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
                write!(w, "(")?;
                for (i, e) in v.iter().enumerate() {
                    if i > 0 {
                        write!(w, " * ")?;
                    }
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    self.write_float(w, e)?;
                }
                write!(w, ")")?;
            }
            FloatExpr::Sum(v) => {
                if v.is_empty() {
                    bail!("Attempted to write an empty sum that should have been simplified");
                }
                write!(w, "(")?;
                for (i, e) in v.iter().enumerate() {
                    if i > 0 {
                        write!(w, " + ")?;
                    }
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    self.write_float(w, e)?;
                }
                write!(w, ")")?;
            }
            FloatExpr::Divide(v) => {
                if v.is_empty() {
                    bail!("Attempted to write an empty division that should have been simplified");
                }
                write!(w, "(")?;
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
                write!(w, ")")?;
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
                write!(w, "(")?;
                for (i, e) in v.iter().enumerate() {
                    if i > 0 {
                        write!(w, " * ")?;
                    }
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    self.write_vec2(w, e)?;
                }
                write!(w, ")")?;
            }
            Vec2Expr::Sum(v) => {
                if v.is_empty() {
                    bail!("Attempted to write an empty sum that should have been simplified");
                }
                write!(w, "(")?;
                for (i, e) in v.iter().enumerate() {
                    if i > 0 {
                        write!(w, " + ")?;
                    }
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    self.write_vec2(w, e)?;
                }
                write!(w, ")")?;
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
                write!(w, "(")?;
                for (i, e) in v.iter().enumerate() {
                    if i > 0 {
                        write!(w, " * ")?;
                    }
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    self.write_vec3(w, e)?;
                }
                write!(w, ")")?;
            }
            Vec3Expr::Sum(v) => {
                if v.is_empty() {
                    bail!("Attempted to write an empty sum that should have been simplified");
                }
                write!(w, "(")?;
                for (i, e) in v.iter().enumerate() {
                    if i > 0 {
                        write!(w, " + ")?;
                    }
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    self.write_vec3(w, e)?;
                }
                write!(w, ")")?;
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
                write!(w, "(")?;
                for (i, e) in v.iter().enumerate() {
                    if i > 0 {
                        write!(w, " * ")?;
                    }
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    self.write_vec4(w, e)?;
                }
                write!(w, ")")?;
            }
            Vec4Expr::Sum(v) => {
                if v.is_empty() {
                    bail!("Attempted to write an empty sum that should have been simplified");
                }
                write!(w, "(")?;
                for (i, e) in v.iter().enumerate() {
                    if i > 0 {
                        write!(w, " + ")?;
                    }
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    self.write_vec4(w, e)?;
                }
                write!(w, ")")?;
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
                for (i, g) in v.iter().enumerate() {
                    if i > 0 {
                        write!(w, ", ")?;
                    }
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
}




impl AstEmitter for Rust {
    fn file_extension() -> &'static str { "rs" }

    fn supports_includes(&self) -> bool { true }
    fn include_file<W: Write, P: AsRef<Path>>(&self, w: &mut W, p: P) -> anyhow::Result<()> {
        let p = p.as_ref();
        let path_str = p.to_string_lossy().replace("\\", "/");
        writeln!(w, "include!(\"{path_str}\");")?;
        Ok(())
    }
    fn supports_imports(&self) -> bool { true }
    fn import_multi_vector<W: Write, Q: IdentifierQualifier, const AntiScalar: BasisElement>(
        &self, w: &mut W, q: &Q, multi_vec: &'static MultiVec<AntiScalar>
    ) -> anyhow::Result<()> {
        let n = multi_vec.name;
        let p = q.qualifying_path_of_data_type(multi_vec);
        let path_str = p.to_string_lossy().replace("/", "::").replace("\\", "::");
        writeln!(w, "use crate::{path_str}::{n};")?;
        Ok(())
    }
    fn import_trait_def<W: Write, Q: IdentifierQualifier>(
        &self, w: &mut W, q: &Q, def: Arc<RawTraitDefinition>
    ) -> anyhow::Result<()> {
        let ucc = def.names.trait_key.as_upper_camel();
        let p = q.qualifying_path_of_trait_def(def);
        let path_str = p.to_string_lossy().replace("/", "::");
        writeln!(w, "use crate::{path_str}::{ucc};")?;
        Ok(())
    }

    fn emit_multi_vector<W: Write, Q: IdentifierQualifier, const AntiScalar: BasisElement>(
        &self, w: &mut W, q: &Q, multi_vec: &'static MultiVec<AntiScalar>
    ) -> anyhow::Result<()> {
        let name = TraitKey::new(multi_vec.name);
        let ucc = name.as_upper_camel();
        let lcc = name.as_lower_camel();
        let lsc = name.as_lower_snake();
        let ssc = name.as_screaming_snake();
        // TODO built in documentation, statistics, and traits that output this type
        writeln!(w, "/// TODO documentation")?;

        // TODO special traits like serde and bytemuck etc
        writeln!(w, "#[derive(Clone, Copy)]")?;
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

        // TODO special traits like serde and bytemuck etc
        writeln!(w, "#[derive(Clone, Copy)]")?;
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
        writeln!(w, "] }};")?;
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


        writeln!(w, "impl std::convert::From<{ucc}> for [f32; {element_count}] {{")?;
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


        writeln!(w, "impl std::convert::From<[f32; {element_count}]> for {ucc} {{")?;
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



        Ok(())
    }

    fn emit_trait_def<W: Write, Q: IdentifierQualifier>(
        &self, w: &mut W, q: &Q, def: Arc<RawTraitDefinition>
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

        // TODO fancy infix


        Ok(())
    }

    fn emit_trait_impl<W: Write, Q: IdentifierQualifier>(
        &self, w: &mut W, q: &Q, impls: Arc<RawTraitImplementation>
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
        match def.arity {
            TraitArity::Zero => {}
            TraitArity::One => write!(w, "self")?,
            TraitArity::Two => write!(w, "self, other: T")?,
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
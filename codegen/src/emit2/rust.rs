use std::io::Write;
use std::ops::Deref;
use std::path::Path;
use std::sync::Arc;
use parking_lot::lock_api::RwLockReadGuard;
use parking_lot::RawRwLock;
use crate::algebra2::basis::BasisElement;
use crate::algebra2::multivector::{BasisElementGroup, MultiVec};
use crate::ast2::datatype::ExpressionType;
use crate::ast2::traits::{RawTraitDefinition, RawTraitImplementation, TraitArity, TraitKey, TraitParam, TraitTypeConsensus};
use crate::emit2::{AstEmitter, IdentifierQualifier};

#[derive(Copy, Clone)]
pub struct Rust {
    pub prefer_fancy_infix: bool,
}


impl Rust {
    // TODO various methods to emit line, etc

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
        let owner_ty = match &impls.owner {
            TraitParam::Generic => panic!("Generic output types are not supported yet."),
            TraitParam::Fixed(ty) => ty,
        };
        if impls.other_var_params.len() > 1 || impls.other_type_params.len() > 1 {
            panic!("We do not support high arity traits yet")
        }
        let mut var_param = None;
        if !impls.other_var_params.is_empty() {
            let ty_param = &impls.other_type_params[0];
            let v_param = &impls.other_var_params[0];
            let TraitParam::Fixed(ty_param) = ty_param else {
                panic!("We do not support generic implementations yet (but hope too eventually)")
            };
            let TraitParam::Fixed(v_param) = v_param else {
                panic!("We do not support generic implementations yet (but hope too eventually)")
            };
            if ty_param != v_param {
                // TODO I feel like this is a representation problem, need to review and maybe
                //  refactor the algebraic data types involved here
                panic!("Type of trait implementation does not agree")
            }
            var_param = Some(v_param);
        }
        // todo alias documentation
        // TODO using a match on tuple here already seems like a mistake. Sure it might be easier
        //  to read each block, but there's too many duplicate snippets. It'd be harder to read
        //  but better for maintenance to only branch each independent condition as few times as
        //  necessary
        match (def.arity, &impls.owner, output_kind.deref()) {
            (TraitArity::Two, TraitParam::Fixed(owner_ty), TraitTypeConsensus::AllAgree(output_ty, _)) => {
                let var_param = var_param.unwrap();
                write!(w, "impl {ucc}<")?;
                self.write_type(w, *var_param)?;
                write!(w, "> for ")?;
                self.write_type(w, *owner_ty)?;
                write!(w, " {{\n    fn {lsc}(self, other: T) -> ")?;
                self.write_type(w, *output_ty)?;
                writeln!(w, " {{")?;
            }
            (TraitArity::Two, TraitParam::Fixed(owner_ty), TraitTypeConsensus::AlwaysSelf) => {
                let var_param = var_param.unwrap();
                write!(w, "impl {ucc}<")?;
                self.write_type(w, *var_param)?;
                write!(w, "> for ")?;
                self.write_type(w, *owner_ty)?;
                writeln!(w, " {{\n    fn {lsc}(self, other: T) -> Self {{")?;
            }
            (TraitArity::Two, TraitParam::Fixed(owner_ty), _) => {
                let var_param = var_param.unwrap();
                write!(w, "impl {ucc}<")?;
                self.write_type(w, *var_param)?;
                write!(w, "> for ")?;
                self.write_type(w, *owner_ty)?;
                write!(w, " {{\n    type Output = ")?;
                self.write_type(w, output_ty)?;
                writeln!(w, ";\n    fn {lsc}(self, other: T) -> Self::Output {{")?;
            }
            (TraitArity::Two, TraitParam::Generic, TraitTypeConsensus::AllAgree(output_ty, _)) => {
                let var_param = var_param.unwrap();
                write!(w, "impl<Owner> {ucc}<")?;
                self.write_type(w, *var_param)?;
                writeln!(w, "> for Owner {{")?;
                write!(w, "{{\n    fn {lsc}(self, other: T) -> ")?;
                self.write_type(w, *output_ty)?;
                writeln!(w, " {{")?;
            }
            (TraitArity::Two, TraitParam::Generic, TraitTypeConsensus::AlwaysSelf) => {
                let var_param = var_param.unwrap();
                write!(w, "impl<Owner> {ucc}<")?;
                self.write_type(w, *var_param)?;
                writeln!(w, "> for Owner {{")?;
                write!(w, "{{\n    fn {lsc}(self, other: T) -> Self {{")?;
            }
            (TraitArity::Two, TraitParam::Generic, _) => {
                let var_param = var_param.unwrap();
                write!(w, "impl<Owner> {ucc}<")?;
                self.write_type(w, *var_param)?;
                writeln!(w, "> for Owner {{")?;
                write!(w, "    type Output = ")?;
                self.write_type(w, output_ty)?;
                writeln!(w, ";\n    fn {lsc}(self, other: T) -> Self::Output {{")?;
            }
            (TraitArity::Two, TraitParam::Fixed(owner_ty), TraitTypeConsensus::AllAgree(output_ty, _)) => {
                write!(w, "impl {ucc} for ")?;
                self.write_type(w, *owner_ty)?;
                writeln!(w, " {{")?;
                write!(w, "    fn {lsc}(self) -> ")?;
                self.write_type(w, *output_ty)?;
                writeln!(w, " {{")?;
            }
            (TraitArity::One, TraitParam::Fixed(owner_ty), TraitTypeConsensus::AlwaysSelf) => {
                write!(w, "impl {ucc} for ")?;
                self.write_type(w, *owner_ty)?;
                writeln!(w, " {{")?;
                write!(w, "    fn {lsc}(self) -> Self {{")?;
            }
            (TraitArity::One, TraitParam::Fixed(owner_ty), _) => {
                write!(w, "impl {ucc} for ")?;
                self.write_type(w, *owner_ty)?;
                write!(w, " {{\n    type Output = ")?;
                self.write_type(w, output_ty)?;
                writeln!(w, ";\n    fn {lsc}(self) -> Self::Output {{")?;
            }
            (TraitArity::One, TraitParam::Generic, TraitTypeConsensus::AllAgree(output_ty, _)) => {
                writeln!(w, "impl<Owner> {ucc} for Owner {{")?;
                write!(w, "    fn {lsc}(self) -> ")?;
                self.write_type(w, *output_ty)?;
                writeln!(w, " {{")?;
            }
            (TraitArity::One, TraitParam::Generic, TraitTypeConsensus::AlwaysSelf) => {
                writeln!(w, "impl<Owner> {ucc} for Owner {{")?;
                write!(w, "    fn {lsc}(self) -> Self {{")?;
            }
            (TraitArity::One, TraitParam::Generic, _) => {
                writeln!(w, "impl<Owner> {ucc} for Owner {{")?;
                write!(w, "    type Output = ")?;
                self.write_type(w, output_ty)?;
                writeln!(w, ";\n    fn {lsc}(self) -> Self::Output {{")?;
            }
            (TraitArity::Zero, TraitParam::Fixed(owner_ty), TraitTypeConsensus::AllAgree(output_ty, _)) => {
                write!(w, "impl {ucc} for ")?;
                self.write_type(w, *owner_ty)?;
                write!(w, " {{\n    fn {lsc}() -> ")?;
                self.write_type(w, *output_ty)?;
                writeln!(w, " {{")?;
            }
            (TraitArity::Zero, TraitParam::Fixed(owner_ty), TraitTypeConsensus::AlwaysSelf) => {
                write!(w, "impl {ucc} for ")?;
                self.write_type(w, *owner_ty)?;
                writeln!(w, " {{\n    fn {lsc}() -> Self {{")?;
            }
            (TraitArity::Zero, TraitParam::Fixed(owner_ty), _) => {
                write!(w, "impl {ucc} for ")?;
                self.write_type(w, *owner_ty)?;
                write!(w, " {{\n    type Output = ")?;
                self.write_type(w, output_ty)?;
                writeln!(w, ";\n    fn {lsc}() -> Self::Output {{")?;
            }
            _ => {
                panic!("Unsupported or invalid trait def implementation: {ucc} for {owner_ty:?}")
            }
        }
        writeln!(w, "        todo!();\n    }}\n}}")?;
        Ok(())
    }

    fn emit_comment<W: Write, S: Into<String>>(
        &self, w: &mut W, is_documentation: bool, s: S
    ) -> anyhow::Result<()> {
        let slashy = if is_documentation { "/// " } else { "// " };
        let s = s.into();
        let comment = s.trim();
        if comment.is_empty() {
            writeln!(w, "{slashy}")?;
            return Ok(())
        }
        let mut comment_iter = comment.split("\n")
            .map(|it| it.trim())
            .skip_while(|it| it.is_empty())
            .peekable();
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
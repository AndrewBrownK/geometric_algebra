use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use crate::algebra2::basis::BasisElement;
use crate::algebra2::multivector::{BasisElementGroup, MultiVec};
use crate::ast2::datatype::ExpressionType;
use crate::ast2::traits::{RawTraitDefinition, RawTraitImplementation, TraitKey};
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
        let path_str = p.to_string_lossy();
        writeln!(w, "include!(\"{path_str}\");")?;
        Ok(())
    }
    fn supports_imports(&self) -> bool { true }
    fn import_multi_vector<W: Write, Q: IdentifierQualifier, const AntiScalar: BasisElement>(
        &self, w: &mut W, q: &Q, multi_vec: &'static MultiVec<AntiScalar>
    ) -> anyhow::Result<()> {
        let p = q.qualifying_path_of_data_type(multi_vec);
        let path_str = p.to_string_lossy().replace("/", "::");
        writeln!(w, "use crate::{path_str};")?;
        Ok(())
    }
    fn import_trait_def<W: Write, Q: IdentifierQualifier>(
        &self, w: &mut W, q: &Q, def: Arc<RawTraitDefinition>
    ) -> anyhow::Result<()> {
        let p = q.qualifying_path_of_trait_def(def);
        let path_str = p.to_string_lossy().replace("/", "::");
        writeln!(w, "use crate::{path_str};")?;
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
            writeln!(w, "/// ")?;
            for (i, el) in group.into_iter().enumerate() {
                if i > 0 {
                    write!(w, ", ")?;
                }
                write!(w, "{el}")?;
            }
            write!(w, "\ng{g}: ")?;
            self.write_type(w, group.expr_type())?;
            writeln!(w, ",")?;
        }
        writeln!(w, "}}")?;

        // TODO special traits like serde and bytemuck etc
        writeln!(w, "#[derive(Clone, Copy)]")?;
        writeln!(w, "pub union {ucc} {{")?;
        writeln!(w, "groups: {ucc}Groups,")?;
        write!(w, "/// ")?;
        let mut total_len = 0;
        for (i, g) in  multi_vec.groups().into_iter().enumerate() {
            if i > 0 {
                write!(w, ",")?;
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
        writeln!(w, "\nelements: [f32; {total_len}],")?;
        writeln!(w, "\n}}")?;

        writeln!(w, "impl {ucc} {{")?;
        writeln!(w, "#[allow(clippy::too_many_arguments)]")?;
        writeln!(w, "pub const fn from_elements(")?;
        for (i, el) in multi_vec.elements().into_iter().enumerate() {
            if i > 0 {
                write!(w, ", ")?;
            }
            write!(w, "{el}: f32")?;
        }
        writeln!(w, "\n) -> Self {{")?;
        writeln!(w, "Self {{ elements: [")?;
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
            }
            write!(w, "g{i}: ")?;
            self.write_type(w, g.expr_type())?;
        }
        writeln!(w, ") -> Self {{")?;
        writeln!(w, "Self {{\ngroups: {ucc}Groups {{")?;
        for (i, _) in multi_vec.groups().into_iter().enumerate() {
            if i > 0 {
                write!(w, ", ")?;
            }
            write!(w, "g{i}")?;
        }
        writeln!(w, " }} }}\n}}")?;

        for (i, g) in multi_vec.groups().into_iter().enumerate() {
            let t = g.expr_type();

            writeln!(w, "#[inline(always)]")?;
            write!(w, "pub fn group{i}(&self) -> ")?;
            self.write_type(w, t)?;
            writeln!(w, " {{")?;
            writeln!(w, "unsafe {{ self.groups.g{i} }}")?;
            writeln!(w, "}}")?;

            writeln!(w, "#[inline(always)]")?;
            write!(w, "pub fn group{i}_mut(&mut self) -> &mut ")?;
            self.write_type(w, t)?;
            writeln!(w, " {{")?;
            writeln!(w, "unsafe {{ &mut self.groups.g{i} }}")?;
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
        writeln!(w, "unsafe {{ &self.elements[{ssc}_INDEX_REMAP[index]] }}")?;
        writeln!(w, "}}\n}}")?;

        writeln!(w, "impl std::ops::IndexMut<usize> for {ucc} {{")?;
        writeln!(w, "fn index_mut(&mut self, index: usize) -> &mut Self::Output {{")?;
        writeln!(w, "unsafe {{ &mut self.elements[{ssc}_INDEX_REMAP[index]] }}")?;
        writeln!(w, "}}\n}}")?;


        writeln!(w, "impl std::convert::From<{ucc}> for [f32; {element_count}] {{")?;
        writeln!(w, "fn from(vector: {ucc}) -> Self {{")?;
        writeln!(w, "unsafe {{ [")?;
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
        writeln!(w, "] }}\n}}\n}}")?;


        writeln!(w, "impl std::convert::From<[f32; {element_count}]> for {ucc} {{")?;
        writeln!(w, "fn from(array: [f32; {element_count}]) -> Self {{")?;
        writeln!(w, "Self {{ elements: [")?;
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
        writeln!(w, "] }}\n}}\n}}")?;


        writeln!(w, "impl std::fmt::Debug for {ucc} {{")?;
        writeln!(w, "fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {{")?;
        writeln!(w, "formatter.debug_struct(\"{ucc}\")")?;
        for (i, el) in multi_vec.elements().into_iter().enumerate() {
            writeln!(w, ".field(\"{el}\", &self[{i}])")?;
        }
        writeln!(w, ".finish()\n}}\n}}")?;



        Ok(())
    }

    fn emit_trait_def<W: Write, Q: IdentifierQualifier>(
        &self, w: &mut W, q: &Q, defs: Arc<RawTraitDefinition>
    ) -> anyhow::Result<()> {
        write!(w, "")?;
        Ok(())
    }

    fn emit_trait_impl<W: Write, Q: IdentifierQualifier>(
        &self, w: &mut W, q: &Q, impls: Arc<RawTraitImplementation>
    ) -> anyhow::Result<()> {
        write!(w, "")?;
        Ok(())
    }

    fn emit_comment<W: Write, S: Into<String>>(
        &self, w: &mut W, s: S, is_documentation: bool
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
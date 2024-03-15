pub fn camel_to_snake_case<W: Write>(collector: &mut W, name: &str) -> std::io::Result<()> {
    let mut underscores = name.chars().enumerate().filter(|(_i, c)| c.is_uppercase()).map(|(i, _c)| i).peekable();
    for (i, c) in name.to_lowercase().bytes().enumerate() {
        if let Some(next_underscores) = underscores.peek() {
            if i == *next_underscores {
                if i > 0 {
                    collector.write_all(b"_")?;
                }
                underscores.next();
            }
        }
        collector.write_all(&[c])?;
    }
    Ok(())
}

pub fn emit_indentation<W: Write>(collector: &mut W, indentation: usize) -> std::io::Result<()> {
    for _ in 0..indentation {
        collector.write_all(b"    ")?;
    }
    Ok(())
}

use crate::ast::AstNode;
use std::io::Write;

mod glsl;
mod rust;
mod wgsl;

pub struct Emitter<W: Write> {
    pub actually_emit: bool,
    rust_collector: W,
    glsl_collector: W,
    wgsl_collector: W,
}

impl Emitter<std::fs::File> {
    pub fn new(actually_emit: bool, path: &std::path::Path) -> Self {
        Self {
            actually_emit,
            rust_collector: std::fs::File::create(path.with_extension("rs")).unwrap(),
            glsl_collector: std::fs::File::create(path.with_extension("glsl")).unwrap(),
            wgsl_collector: std::fs::File::create(path.with_extension("wgsl")).unwrap(),
        }
    }

    pub fn new_rust_collector(&mut self, path: &std::path::Path) {
        self.rust_collector = std::fs::File::create(path.with_extension("rs")).unwrap();
    }
}

impl<W: Write> Emitter<W> {
    pub fn emit(&mut self, ast_node: &AstNode) -> std::io::Result<()> {
        if self.actually_emit {
            rust::emit_code(&mut self.rust_collector, ast_node, 0)?;
            glsl::emit_code(&mut self.glsl_collector, ast_node, 0)?;
            wgsl::emit_code(&mut self.wgsl_collector, ast_node, 0)?;
        }
        Ok(())
    }

    pub fn emit_rust_preamble(&mut self, preamble: &'static str) -> std::io::Result<()> {
        self.rust_collector.write_all(&preamble.as_bytes())?;
        self.rust_collector.write_all(b"\n\n")
    }
}

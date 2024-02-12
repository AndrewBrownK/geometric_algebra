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

use std::io::Write;
use crate::{ast::AstNode, glsl, rust, wgsl};


pub struct Emitter<W: Write> {
    pub rust_collector: W,
    pub glsl_collector: W,
    pub wgsl_collector: W,
}

impl Emitter<std::fs::File> {
    pub fn new(path: &std::path::Path) -> Self {
        Self {
            rust_collector: std::fs::File::create(path.with_extension("rs")).unwrap(),
            glsl_collector: std::fs::File::create(path.with_extension("glsl")).unwrap(),
            wgsl_collector: std::fs::File::create(path.with_extension("wgsl")).unwrap(),
        }
    }

    pub fn with_new_rust_collector(&mut self, path: &std::path::Path, preamble: &'static str) -> std::io::Result<()>{
        self.rust_collector = std::fs::File::create(path.with_extension("rs")).unwrap();
        self.rust_collector.write_all(&preamble.as_bytes())?;
        self.rust_collector.write_all(b"\n\n")
    }
}


impl<W: Write> Emitter<W> {
    pub fn emit(&mut self, ast_node: &AstNode) -> std::io::Result<()> {
        rust::emit_code(&mut self.rust_collector, ast_node, 0)?;
        glsl::emit_code(&mut self.glsl_collector, ast_node, 0)?;
        wgsl::emit_code(&mut self.wgsl_collector, ast_node, 0)?;
        Ok(())
    }
}

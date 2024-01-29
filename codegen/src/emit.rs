pub fn camel_to_snake_case<W: std::io::Write>(collector: &mut W, name: &str) -> std::io::Result<()> {
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

pub fn emit_indentation<W: std::io::Write>(collector: &mut W, indentation: usize) -> std::io::Result<()> {
    for _ in 0..indentation {
        collector.write_all(b"    ")?;
    }
    Ok(())
}

use crate::{ast::AstNode, glsl, rust, wgsl};

// TODO change this to an enum
pub struct Emitter<W: std::io::Write> {
    pub rust_collector: Option<W>,
    pub glsl_collector: Option<W>,
    pub wgsl_collector: Option<W>,
}

impl Emitter<std::fs::File> {
    pub fn new(path: &std::path::Path) -> Self {
        Self {
            rust_collector: Some(std::fs::File::create(path.with_extension("rs")).unwrap()),
            glsl_collector: Some(std::fs::File::create(path.with_extension("glsl")).unwrap()),
            wgsl_collector: Some(std::fs::File::create(path.with_extension("wgsl")).unwrap()),
        }
    }

    pub fn new_rust_only(path: &std::path::Path) -> Self {
        Self {
            rust_collector: Some(std::fs::File::create(path.with_extension("rs")).unwrap()),
            glsl_collector: None,
            wgsl_collector: None,
        }
    }

    pub fn new_glsl_only(path: &std::path::Path) -> Self {
        Self {
            rust_collector: None,
            glsl_collector: Some(std::fs::File::create(path.with_extension("glsl")).unwrap()),
            wgsl_collector: None,
        }
    }

    pub fn new_wgsl_only(path: &std::path::Path) -> Self {
        Self {
            rust_collector: None,
            glsl_collector: None,
            wgsl_collector: Some(std::fs::File::create(path.with_extension("wgsl")).unwrap()),
        }
    }
}

// TODO do things differently so that I can have multiple rust files, but still keep one shader file
impl<W: std::io::Write> Emitter<W> {
    pub fn emit(&mut self, ast_node: &AstNode) -> std::io::Result<()> {
        if let Some(rc) = &mut self.rust_collector {
            rust::emit_code(rc, ast_node, 0)?;
        }
        if let Some(gc) = &mut self.glsl_collector {
            glsl::emit_code(gc, ast_node, 0)?;
        }
        if let Some(wc) = &mut self.wgsl_collector {
            wgsl::emit_code(wc, ast_node, 0)?;
        }
        Ok(())
    }
}

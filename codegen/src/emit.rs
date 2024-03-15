use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

use crate::ast::AstNode;

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

mod glsl;
mod rust;
mod wgsl;

const CODEGEN_DISCLAIMER: &str = "\
//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/geometric_algebra/
//

";

pub struct Emitter<W: Write> {
    pub actually_emit: bool,
    rust_collector: W,
    glsl_collector: W,
    wgsl_collector: W,
    rust_files_so_far: Vec<String>,
}

impl Emitter<File> {
    pub fn new(actually_emit: bool, path: &Path, rust_name: &str, shader_name: &str) -> Self {
        let rust_path_buff = path.join(Path::new(rust_name)).with_extension("rs");
        let rust_path_str = rust_path_buff.as_path().to_string_lossy().to_string();
        Self {
            actually_emit,
            rust_collector: File::create(rust_path_buff).unwrap(),
            glsl_collector: File::create(path.join(Path::new("shaders")).join(Path::new(shader_name)).with_extension("glsl")).unwrap(),
            wgsl_collector: File::create(path.join(Path::new("shaders")).join(Path::new(shader_name)).with_extension("wgsl")).unwrap(),
            rust_files_so_far: vec![rust_path_str],
        }
    }

    pub fn new_rust_collector(&mut self, path: &Path) {
        let rust_path_buff = path.with_extension("rs");
        let rust_path_str = rust_path_buff.as_path().to_string_lossy().to_string();
        self.rust_collector = File::create(rust_path_buff).unwrap();
        self.rust_files_so_far.push(rust_path_str);
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
        if self.actually_emit {
            self.rust_collector.write_all(CODEGEN_DISCLAIMER.as_bytes())?;
            self.rust_collector.write_all(&preamble.as_bytes())?;
            self.rust_collector.write_all(b"\n\n")?;
        }
        Ok(())
    }

    pub fn emit_shader_preamble(&mut self) -> std::io::Result<()> {
        if self.actually_emit {
            self.glsl_collector.write_all(CODEGEN_DISCLAIMER.as_bytes())?;
            self.wgsl_collector.write_all(CODEGEN_DISCLAIMER.as_bytes())?;
        }
        Ok(())
    }

    pub fn end_with_rust_fmt(self) {
        if !self.actually_emit {
            return;
        }
        let result: anyhow::Result<()> = try {
            let mut cmd = Command::new("rustfmt");
            for rust_file in self.rust_files_so_far {
                cmd.arg(rust_file);
            }
            let mut child = cmd.spawn()?;
            let exit_status = child.wait()?;
            exit_status.exit_ok()?;
        };
        match result {
            Ok(()) => {}
            Err(err) => {
                eprintln!("Could not format generated rust files: {err}");
            }
        }
    }
}
#[cfg(feature = "slang")]
pub mod slang {

    use std::path::Path;
    use std::process::Command;

    pub const CGA3D_SLANG: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/integrations/slang/cga3d.slang");

    /// This requires slangc to be installed and added to your path
    /// Warning, this can be an extremely slow operation. Give it time.
    pub fn compile_binary_slang_module<P: AsRef<Path>>(output_dir: P) {
        let mut cmd = Command::new("slangc");
        cmd.arg(CGA3D_SLANG);
        cmd.arg("-o");
        let output_dir = output_dir.as_ref().to_string_lossy();
        cmd.arg(format!("{output_dir}/cga3d.slang-module"));
        // TODO better error handling and propagation
        cmd.spawn().expect("failed to run slangc");
    }

    pub enum Target {
        Spirv,
        Wgsl,
        Glsl,
        SlangIR,
    }

    pub enum Stage {}

    pub struct Output<P, S> {
        output_file: P,
        target: Target,
        entry: Option<S>,
    }

    // https://github.com/shader-slang/slang/blob/master/docs/command-line-slangc-reference.md#-o
    pub enum OutputScheme<'a, P, S> {
        // If no -target or -stage is specified, one may be inferred from file extension
        InferStage(Target, P),

        // If multiple -target options and a single -entry are present, each -o associates
        // with the first -target to its left.
        OneEntryManyTargets { entry: S, targets: &'a [(Target, P)] },

        // Otherwise, if multiple -entry options are present, each -o associates with the first
        // -entry to its left, and with the -target that matches the one inferred from <path>.
        TargetsPerEntries { entries: &'a [(S, &'a [(Target, P)])] },
    }

    /// Use slangc to compile an application shader that depends on cga3d.slang and no
    /// other slang dependencies. If you need to compile with other dependencies, you can
    /// use slangc normally in the command line instead of using this function.
    pub fn compile_application_shader<'a, P1: AsRef<Path>, P2: AsRef<Path>, S: Into<String>>(app_src_file: P1, output_scheme: OutputScheme<'a, P2, S>) {
        let mut cmd = Command::new("slangc");
        cmd.arg(CGA3D_SLANG);
        cmd.arg("-o");
        // TODO actual code for this function, not just the copy paste of other function
        let output_dir = app_src_file.as_ref().to_string_lossy();
        cmd.arg(format!("{output_dir}/cga3d.slang-module"));
        // TODO better error handling and propagation
        cmd.spawn().expect("failed to run slangc");
    }
}

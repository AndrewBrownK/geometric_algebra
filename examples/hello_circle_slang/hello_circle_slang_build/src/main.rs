/*
Measure-Command { slangc -I "../../../libraries/cga3d/src/integrations/slang" -entry "fs_main" -stage "fragment" -o "res/shader.fs.spv" -- "src/shader.slang" }
Measure-Command { slangc -I "../../../libraries/cga3d/src/integrations/slang" -entry "vs_main" -stage "vertex" -o "res/shader.vs.spv" -- "src/shader.slang" }
 */

pub fn main() -> anyhow::Result<()> {
    hello_circle_slang_build::compile_shaders_in("examples/hello_circle_slang/hello_circle_slang")
}
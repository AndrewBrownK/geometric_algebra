#import cga3d_min;

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    let x = f32(i32(in_vertex_index) - 1);
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
    return vec4<f32>(x, y, 0.0, 1.0);
}

// Question.. how to render true circles on the GPU? Don't want lots of vertexes if it can be helped.
// Well yeah. It seems you can make smooth circles in the fragment shader.
// https://www.youtube.com/watch?v=Q9Domz1Qlbw
@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(0.0, 0.0, 1.0, 1.0);
}
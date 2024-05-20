#import cga3d_min as cga;

// Quick refresher on shader stages:
// 1) Objects start in homogenous space, perhaps also known as "view space"
// 2) In the vertex shader, you project this space so the view frustum = the normalized device coordinates (NDC) box.
//    This is called "clip space".
// 3) After the vertex shader, the clipping stage will remove/ignore objects outside the frustum (which is the NDC box).
//    If an object is partially inside and partially outside the clipped space, the GPU automatically creates new
//    vertices at the intersection points.
// 4) The clipped geometry is given to the fragment shader for coloring.


struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) percent_position: vec4<f32>
}


@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    let expand = 3.0;
    // Full screen triangle
    // 0   ->   -1, -1   ->   -3, -3
    // 1   ->    0,  1   ->    0,  3
    // 2   ->    1, -1   ->    3, -3
    let x = f32(i32(in_vertex_index) - 1);
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
    var out: VertexOutput;
    out.position = vec4<f32>(expand * x, expand * y, 0.0, 1.0);
    out.percent_position = vec4<f32>(0.5 * (1.0 + (expand * x)), 0.5 * (1.0 - (expand * y)), 0.0, 1.0);
    return out;
}


@group(0) @binding(0)
var<uniform> screen_size: vec2<f32>;


@fragment
fn fs_main(vertex: VertexOutput) -> @location(0) vec4<f32> {

    // Start by recovering a nice coordinate space
    // (0, 0) is in the center
    // lesser of x or y will range from -1 to 1
    // Unlike NDC, this will be square because it is corrected for aspect ratio
    var new_x = vertex.percent_position.x;
    var new_y = vertex.percent_position.y;
    new_x = (2.0 * new_x) - 1.0;
    new_y = 1.0 - (2.0 * new_y);
    let max_aspect = max(screen_size.x, screen_size.y);
    let x_correction = max_aspect / screen_size.y;
    let y_correction = max_aspect / screen_size.x;
    new_x = new_x * x_correction;
    new_y = new_y * y_correction;

    // Choose some background colors that will help us confirm that we got the coordinate space we wanted
    let color = vec4<f32>(new_x, new_y, 0.0, 1.0);
    let background_color = select(
        color,
        vec4<f32>(color.x, color.y, 0.5, 1.0),
        (color.x < -1.0) || (color.y < -0.5)
    );
    // return background_color;

    // Now let's do some CGA

    // Start by constructing a Circle.
    // We'll create our circle by joining 3 RoundPoints.

    // Create 3 round points
    let left = cga::RoundPoint(vec3<f32>(-1.0, 0.0, 0.0), vec2<f32>(1.0, 0.5));
    let top = cga::RoundPoint(vec3<f32>(0.0, 1.0, 0.0), vec2<f32>(1.0, 0.5));
    let right = cga::RoundPoint(vec3<f32>(1.0, 0.0, 0.0), vec2<f32>(1.0, 0.5));

    // Join the first pair
    let left_to_top_dipole = cga::roundPoint_wedge_roundPoint(left, top);

    // Joint the remaining point
    let circle = cga::dipole_wedge_roundPoint(left_to_top_dipole, right);

    // Now that we have our circle, we can determine what we want to do with this fragment

    // Let's create a line representing this fragment
    let fp_e5 = (new_x * new_x + new_y * new_y + 1.0) / 2.0;
    let fragment_point = cga::RoundPoint(vec3<f32>(new_x, new_y, 1.0), vec2<f32>(1.0, fp_e5));
    let direction = cga::FlatPoint(vec4<f32>(0.0, 0.0, 1.0, 0.0));

    // Join a round point and point at infinity to create a line
    let fragment_as_line = cga::flatPoint_wedge_roundPoint(direction, fragment_point);

    // Now perform a meet to get a RoundPoint centered on the line and contained by the circle
    let meet_fragment_and_circle = cga::circle_antiWedge_line(circle, fragment_as_line);

    // Now get the squared radius of the meet
    let radius_squared = cga::roundPoint_unitizedRadiusNormSquared(meet_fragment_and_circle);

    // And lastly, make pixels more bright the closer they are to the circle.
    let destructive_circle_color = vec4<f32>(1.0) - vec4<f32>(abs(radius_squared) * 100.0);
    var circle_color = destructive_circle_color;
//    circle_color = max(vec4<f32>(0.0), destructive_circle_color);
    return background_color + circle_color;
}
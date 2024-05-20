#import cga3d_min as cga;
#version 450


// Quick refresher on shader stages:
// 1) Objects start in homogenous space, perhaps also known as "view space"
// 2) In the vertex shader, you project this space so the view frustum = the normalized device coordinates (NDC) box.
//    This is called "clip space".
// 3) After the vertex shader, the clipping stage will remove/ignore objects outside the frustum (which is the NDC box).
//    If an object is partially inside and partially outside the clipped space, the GPU automatically creates new
//    vertices at the intersection points.
// 4) The clipped geometry is given to the fragment shader for coloring.

layout(binding = 0) uniform vec2 screen_size;

layout(location = 0) in vec4 percent_position;
layout(location = 0) out vec4 fragColor;

void main() {
    // Start by recovering a nice coordinate space
    // (0, 0) is in the center
    // lesser of x or y will range from -1 to 1
    // Unlike NDC, this will be square because it is corrected for aspect ratio
    float new_x = percent_position.x;
    float new_y = percent_position.y;
    new_x = (2.0 * new_x) - 1.0;
    new_y = 1.0 - (2.0 * new_y);
    float max_aspect = max(screen_size.x, screen_size.y);
    float x_correction = max_aspect / screen_size.y;
    float y_correction = max_aspect / screen_size.x;
    new_x = new_x * x_correction;
    new_y = new_y * y_correction;

    // Choose some background colors that will help us confirm that we got the coordinate space we wanted
    vec4 color = vec4(new_x, new_y, 0.0, 1.0);
    vec4 background_color = ((color.x < -1.0) || (color.y < -0.5)) ? vec4(color.x, color.y, 0.5, 1.0) : color;
    // fragColor = background_color;

    // Now let's do some CGA

    // Start by constructing a Circle.
    // We'll create our circle by joining 3 RoundPoints.

    /*
    TODO
        wgpu error: Validation Error
        Caused by:
            In Device::create_shader_module
        Shader validation error:
          ┌─ :1:1
          │
        1 │
          │   naga::Type [6]
            Function [12] 'main' is invalid
            Local variable [9] 'left' is invalid
            Initializer doesn't match the variable type
    */


    // Create 3 round points
    cga::RoundPoint left = cga::RoundPoint(vec3(-1.0, 0.0, 0.0), vec2(1.0, 0.5));
    cga::RoundPoint top = cga::RoundPoint(vec3(0.0, 1.0, 0.0), vec2(1.0, 0.5));
    cga::RoundPoint right = cga::RoundPoint(vec3(1.0, 0.0, 0.0), vec2(1.0, 0.5));

    // Join the first pair
    cga::Dipole left_to_top_dipole = cga::roundPoint_wedge_roundPoint(left, top);

    // Join the remaining point
    cga::Circle circle = cga::dipole_wedge_roundPoint(left_to_top_dipole, right);

    // Now that we have our circle, we can determine what we want to do with this fragment

    // Let's create a line representing this fragment
    float fp_e5 = (new_x * new_x + new_y * new_y + 1.0) / 2.0;
    cga::RoundPoint fragment_point = cga::RoundPoint(vec3(new_x, new_y, 1.0), vec2(1.0, fp_e5));
    cga::FlatPoint direction = cga::FlatPoint(vec4(0.0, 0.0, 1.0, 0.0));

    // Join a round point and point at infinity to create a line
    cga::Line fragment_as_line = cga::flatPoint_wedge_roundPoint(direction, fragment_point);

    // Now perform a meet to get a RoundPoint centered on the line and contained by the circle
    cga::RoundPoint meet_fragment_and_circle = cga::circle_antiWedge_line(circle, fragment_as_line);

    // Now get the squared radius of the meet
    float radius_squared = cga::roundPoint_unitizedRadiusNormSquared(meet_fragment_and_circle);

    // And lastly, make pixels more bright the closer they are to the circle.
    vec4 destructive_circle_color = vec4(1.0) - vec4(abs(radius_squared) * 100.0);
    vec4 circle_color = destructive_circle_color;
    // circle_color = max(vec4(0.0), destructive_circle_color);
    fragColor = background_color + circle_color;
}
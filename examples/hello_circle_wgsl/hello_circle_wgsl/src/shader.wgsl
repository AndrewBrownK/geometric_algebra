#import cga3d as cga;

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
    // Full screen triangle
    let expand = 3.0;
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
    let left = cga::RoundPoint(-1.0, 0.0, 0.0, 1.0, 0.5);
    let top = cga::RoundPoint(0.0, 1.0, 0.0, 1.0, 0.5);
    let right = cga::RoundPoint(1.0, 0.0, 0.0, 1.0, 0.5);

    // Join the first pair
    let left_to_top_dipole = cga::roundPoint_wedge_roundPoint(left, top);

    // Join the remaining point
    let circle = cga::dipole_wedge_roundPoint(left_to_top_dipole, right);

    // Now that we have our circle, we can determine what we want to do with this fragment

    // Let's create a line representing this fragment
    let fp_e5 = (new_x * new_x + new_y * new_y + 1.0) / 2.0;
    let fragment_point = cga::RoundPoint(new_x, new_y, 1.0, 1.0, fp_e5);
    let direction = cga::FlatPoint(0.0, 0.0, 1.0, 0.0);

    // Join a round point and point at infinity to create a line
    let fragment_as_line = cga::flatPoint_wedge_roundPoint(direction, fragment_point);

    // Now perform a meet to get a RoundPoint centered on the line and contained by the circle
    let meet_fragment_and_circle = cga::circle_antiWedge_line(circle, fragment_as_line);

    // Now get the squared radius of the meet
    let radius_squared = cga::roundPoint_unitizedRadiusNormSquared(meet_fragment_and_circle);

    // And lastly, make pixels more bright the closer they are to the circle.
    let destructive_circle_color = vec4<f32>(1.0) - vec4<f32>(abs(radius_squared) * 100.0);
    var circle_color = destructive_circle_color;
    // circle_color = max(vec4<f32>(0.0), destructive_circle_color);
    return background_color + circle_color;
}

//fn roundPoint_wedge_roundPoint(self_: RoundPoint, other: RoundPoint) -> Dipole {
//    let self_groups = roundPoint_grouped(self_);
//    let other_groups = roundPoint_grouped(other);
//    return dipole_degroup(DipoleGroups(
//        /* e41, e42, e43 */ ((vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group0_.xyz) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_.xyz),
//        /* e23, e31, e12, e45 */ vec4<f32>(other_groups.group0_.zxy * self_groups.group0_.yzx.xyz, other.e5_ * self_.e4_) - (other_groups.group0_.yzxw * vec4<f32>(self_groups.group0_.zxy.xyz, self_.e5_)),
//        /* e15, e25, e35 */ ((vec4<f32>(other.e5_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_.xyz) - ((vec4<f32>(self_.e5_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group0_.xyz)
//    ));
//}
//
//
//fn dipole_wedge_roundPoint(self_: Dipole, other: RoundPoint) -> Circle {
//    let self_groups = dipole_grouped(self_);
//    let other_groups = roundPoint_grouped(other);
//    return circle_degroup(CircleGroups(
//        /* e423, e431, e412 */ ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_.xyz) + (self_groups.group0_.yzx * other_groups.group0_.zxy) - (self_groups.group0_.zxy * other_groups.group0_.yzx),
//        /* e415, e425, e435, e321 */ vec4<f32>((self_.e41_ * other.e5_) + (self_.e15_ * other.e4_), (self_.e42_ * other.e5_) + (self_.e25_ * other.e4_), (self_.e43_ * other.e5_) + (self_.e35_ * other.e4_), -(self_.e31_ * other.e2_) - (self_.e12_ * other.e3_)) - (self_groups.group1_.wwwx * other_groups.group0_.xyzx),
//        /* e235, e315, e125 */ ((vec4<f32>(other.e5_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_.xyz) + (self_groups.group2_.zxy * other_groups.group0_.yzx) - (self_groups.group2_.yzx * other_groups.group0_.zxy)
//    ));
//}
//
//
//fn flatPoint_wedge_roundPoint(self_: FlatPoint, other: RoundPoint) -> Line {
//    let self_groups = flatPoint_grouped(self_);
//    let other_groups = roundPoint_grouped(other);
//    return line_degroup(LineGroups(
//        /* e415, e425, e435 */ ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_.xyz) - ((vec4<f32>(self_.e45_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group0_.xyz),
//        /* e235, e315, e125 */ (self_groups.group0_.zxy * other_groups.group0_.yzx) - (self_groups.group0_.yzx * other_groups.group0_.zxy)
//    ));
//}
//
//
//fn circle_antiWedge_line(self_: Circle, other: Line) -> RoundPoint {
//    let self_groups = circle_grouped(self_);
//    let other_groups = line_grouped(other);
//    return roundPoint_degroup(RoundPointGroups(
//        /* e1, e2, e3, e4 */ vec4<f32>((self_.e412_ * other.e315_) + (self_.e321_ * other.e415_), (self_.e423_ * other.e125_) + (self_.e321_ * other.e425_), (self_.e431_ * other.e235_) + (self_.e321_ * other.e435_), -(self_.e431_ * other.e425_) - (self_.e412_ * other.e435_)) - vec4<f32>(self_groups.group0_.yzx * other_groups.group1_.zxy.xyz, self_.e423_ * other.e415_),
//        /* e5 */ vec4<f32>(-(self_.e415_ * other.e235_) - (self_.e425_ * other.e315_) - (self_.e435_ * other.e125_) - (self_.e235_ * other.e415_) - (self_.e315_ * other.e425_) - (self_.e125_ * other.e435_), 0.0, 0.0, 0.0)
//    ));
//}

//fn roundPoint_unitizedRadiusNormSquared(self_: RoundPoint) -> f32 {
//    let self_groups = roundPoint_grouped(self_);
//    return roundPoint_radiusNormSquared(self_).scalar / (roundPoint_roundWeightNormSquared(self_).e12345_);
//}
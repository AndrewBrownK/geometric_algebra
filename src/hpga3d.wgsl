struct Scalar {
    // 1
     g0: f32,
}

struct MultiVector {
    // 1, e23, -e13, e12
     g0: vec4<f32>,
    // e0, -e023, e013, -e012
     g1: vec4<f32>,
    // e123, e1, e2, e3
     g2: vec4<f32>,
    // e0123, e01, e02, e03
     g3: vec4<f32>,
}

struct Rotor {
    // 1, e23, -e13, e12
     g0: vec4<f32>,
}

struct Point {
    // e123, -e023, e013, -e012
     g0: vec4<f32>,
}

struct IdealPoint {
    // e01, e02, e03
     g0: vec3<f32>,
}

struct Plane {
    // e0, e1, e2, e3
     g0: vec4<f32>,
}

struct Line {
    // e01, e02, e03
     g0: vec3<f32>,
    // e23, -e13, e12
     g1: vec3<f32>,
}

struct Translator {
    // 1, e01, e02, e03
     g0: vec4<f32>,
}

struct Motor {
    // 1, e23, -e13, e12
     g0: vec4<f32>,
    // e0123, e01, e02, e03
     g1: vec4<f32>,
}

struct PointAndPlane {
    // e123, -e023, e013, -e012
     g0: vec4<f32>,
    // e0, e1, e2, e3
     g1: vec4<f32>,
}

fn scalar_zero() -> Scalar  {
    return Scalar(0.0);
}

fn scalar_one() -> Scalar  {
    return Scalar(1.0);
}

fn scalar_neg(self_: Scalar) -> Scalar  {
    return Scalar(self_.g0 * -1.0);
}

fn scalar_automorphism(self_: Scalar) -> Scalar  {
    return Scalar(self_.g0);
}

fn scalar_reversal(self_: Scalar) -> Scalar  {
    return Scalar(self_.g0);
}

fn scalar_conjugation(self_: Scalar) -> Scalar  {
    return Scalar(self_.g0);
}

fn scalar_scalar_add(self_: Scalar, other: Scalar) -> Scalar  {
    return Scalar(self_.g0 + other.g0);
}

fn scalar_scalar_sub(self_: Scalar, other: Scalar) -> Scalar  {
    return Scalar(self_.g0 - other.g0);
}

fn scalar_scalar_mul(self_: Scalar, other: Scalar) -> Scalar  {
    return Scalar(self_.g0 * other.g0);
}

fn scalar_scalar_div(self_: Scalar, other: Scalar) -> Scalar  {
    return Scalar(self_.g0 * 1.0 / other.g0 * 1.0);
}

fn scalar_scalar_geometric_product(self_: Scalar, other: Scalar) -> Scalar  {
    return Scalar(self_.g0 * other.g0);
}

fn scalar_scalar_outer_product(self_: Scalar, other: Scalar) -> Scalar  {
    return Scalar(self_.g0 * other.g0);
}

fn scalar_scalar_inner_product(self_: Scalar, other: Scalar) -> Scalar  {
    return Scalar(self_.g0 * other.g0);
}

fn scalar_scalar_left_contraction(self_: Scalar, other: Scalar) -> Scalar  {
    return Scalar(self_.g0 * other.g0);
}

fn scalar_scalar_right_contraction(self_: Scalar, other: Scalar) -> Scalar  {
    return Scalar(self_.g0 * other.g0);
}

fn scalar_scalar_scalar_product(self_: Scalar, other: Scalar) -> Scalar  {
    return Scalar(self_.g0 * other.g0);
}

fn scalar_multi_vector_add(self_: Scalar, other: MultiVector) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + other.g0, other.g1, other.g2, other.g3);
}

fn scalar_multi_vector_sub(self_: Scalar, other: MultiVector) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0) * vec4<f32>(1.0, 0.0, 0.0, 0.0) - other.g0, vec4<f32>(0.0) - other.g1, vec4<f32>(0.0) - other.g2, vec4<f32>(0.0) - other.g3);
}

fn scalar_multi_vector_geometric_product(self_: Scalar, other: MultiVector) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0) * other.g0, vec4<f32>(self_.g0) * other.g1, vec4<f32>(self_.g0) * other.g2, vec4<f32>(self_.g0) * other.g3);
}

fn scalar_multi_vector_regressive_product(self_: Scalar, other: MultiVector) -> Scalar  {
    return Scalar(self_.g0 * other.g3.x);
}

fn scalar_multi_vector_outer_product(self_: Scalar, other: MultiVector) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0) * other.g0, vec4<f32>(self_.g0) * other.g1, vec4<f32>(self_.g0) * other.g2, vec4<f32>(self_.g0) * other.g3);
}

fn scalar_multi_vector_inner_product(self_: Scalar, other: MultiVector) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0) * other.g0, vec4<f32>(self_.g0) * other.g1, vec4<f32>(self_.g0) * other.g2, vec4<f32>(self_.g0) * other.g3);
}

fn scalar_multi_vector_left_contraction(self_: Scalar, other: MultiVector) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0) * other.g0, vec4<f32>(self_.g0) * other.g1, vec4<f32>(self_.g0) * other.g2, vec4<f32>(self_.g0) * other.g3);
}

fn scalar_multi_vector_right_contraction(self_: Scalar, other: MultiVector) -> Scalar  {
    return Scalar(self_.g0 * other.g0.x);
}

fn scalar_multi_vector_scalar_product(self_: Scalar, other: MultiVector) -> Scalar  {
    return Scalar(self_.g0 * other.g0.x);
}

fn scalar_rotor_add(self_: Scalar, other: Rotor) -> Rotor  {
    return Rotor(vec4<f32>(self_.g0) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + other.g0);
}

fn scalar_rotor_sub(self_: Scalar, other: Rotor) -> Rotor  {
    return Rotor(vec4<f32>(self_.g0) * vec4<f32>(1.0, 0.0, 0.0, 0.0) - other.g0);
}

fn scalar_rotor_geometric_product(self_: Scalar, other: Rotor) -> Rotor  {
    return Rotor(vec4<f32>(self_.g0) * other.g0);
}

fn scalar_rotor_outer_product(self_: Scalar, other: Rotor) -> Rotor  {
    return Rotor(vec4<f32>(self_.g0) * other.g0);
}

fn scalar_rotor_inner_product(self_: Scalar, other: Rotor) -> Rotor  {
    return Rotor(vec4<f32>(self_.g0) * other.g0);
}

fn scalar_rotor_left_contraction(self_: Scalar, other: Rotor) -> Rotor  {
    return Rotor(vec4<f32>(self_.g0) * other.g0);
}

fn scalar_rotor_right_contraction(self_: Scalar, other: Rotor) -> Scalar  {
    return Scalar(self_.g0 * other.g0.x);
}

fn scalar_rotor_scalar_product(self_: Scalar, other: Rotor) -> Scalar  {
    return Scalar(self_.g0 * other.g0.x);
}

fn scalar_point_geometric_product(self_: Scalar, other: Point) -> Point  {
    return Point(vec4<f32>(self_.g0) * other.g0);
}

fn scalar_point_outer_product(self_: Scalar, other: Point) -> Point  {
    return Point(vec4<f32>(self_.g0) * other.g0);
}

fn scalar_point_inner_product(self_: Scalar, other: Point) -> Point  {
    return Point(vec4<f32>(self_.g0) * other.g0);
}

fn scalar_point_left_contraction(self_: Scalar, other: Point) -> Point  {
    return Point(vec4<f32>(self_.g0) * other.g0);
}

fn scalar_ideal_point_add(self_: Scalar, other: IdealPoint) -> Translator  {
    return Translator(vec4<f32>(self_.g0) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0));
}

fn scalar_ideal_point_sub(self_: Scalar, other: IdealPoint) -> Translator  {
    return Translator(vec4<f32>(self_.g0) * vec4<f32>(1.0, 0.0, 0.0, 0.0) - vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0));
}

fn scalar_ideal_point_geometric_product(self_: Scalar, other: IdealPoint) -> IdealPoint  {
    return IdealPoint(vec3<f32>(self_.g0) * other.g0);
}

fn scalar_ideal_point_outer_product(self_: Scalar, other: IdealPoint) -> IdealPoint  {
    return IdealPoint(vec3<f32>(self_.g0) * other.g0);
}

fn scalar_ideal_point_inner_product(self_: Scalar, other: IdealPoint) -> IdealPoint  {
    return IdealPoint(vec3<f32>(self_.g0) * other.g0);
}

fn scalar_ideal_point_left_contraction(self_: Scalar, other: IdealPoint) -> IdealPoint  {
    return IdealPoint(vec3<f32>(self_.g0) * other.g0);
}

fn scalar_plane_geometric_product(self_: Scalar, other: Plane) -> Plane  {
    return Plane(vec4<f32>(self_.g0) * other.g0);
}

fn scalar_plane_outer_product(self_: Scalar, other: Plane) -> Plane  {
    return Plane(vec4<f32>(self_.g0) * other.g0);
}

fn scalar_plane_inner_product(self_: Scalar, other: Plane) -> Plane  {
    return Plane(vec4<f32>(self_.g0) * other.g0);
}

fn scalar_plane_left_contraction(self_: Scalar, other: Plane) -> Plane  {
    return Plane(vec4<f32>(self_.g0) * other.g0);
}

fn scalar_line_geometric_product(self_: Scalar, other: Line) -> Line  {
    return Line(vec3<f32>(self_.g0) * other.g0, vec3<f32>(self_.g0) * other.g1);
}

fn scalar_line_outer_product(self_: Scalar, other: Line) -> Line  {
    return Line(vec3<f32>(self_.g0) * other.g0, vec3<f32>(self_.g0) * other.g1);
}

fn scalar_line_inner_product(self_: Scalar, other: Line) -> Line  {
    return Line(vec3<f32>(self_.g0) * other.g0, vec3<f32>(self_.g0) * other.g1);
}

fn scalar_line_left_contraction(self_: Scalar, other: Line) -> Line  {
    return Line(vec3<f32>(self_.g0) * other.g0, vec3<f32>(self_.g0) * other.g1);
}

fn scalar_translator_add(self_: Scalar, other: Translator) -> Translator  {
    return Translator(vec4<f32>(self_.g0) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + other.g0);
}

fn scalar_translator_sub(self_: Scalar, other: Translator) -> Translator  {
    return Translator(vec4<f32>(self_.g0) * vec4<f32>(1.0, 0.0, 0.0, 0.0) - other.g0);
}

fn scalar_translator_geometric_product(self_: Scalar, other: Translator) -> Translator  {
    return Translator(vec4<f32>(self_.g0) * other.g0);
}

fn scalar_translator_outer_product(self_: Scalar, other: Translator) -> Translator  {
    return Translator(vec4<f32>(self_.g0) * other.g0);
}

fn scalar_translator_inner_product(self_: Scalar, other: Translator) -> Translator  {
    return Translator(vec4<f32>(self_.g0) * other.g0);
}

fn scalar_translator_left_contraction(self_: Scalar, other: Translator) -> Translator  {
    return Translator(vec4<f32>(self_.g0) * other.g0);
}

fn scalar_translator_right_contraction(self_: Scalar, other: Translator) -> Scalar  {
    return Scalar(self_.g0 * other.g0.x);
}

fn scalar_translator_scalar_product(self_: Scalar, other: Translator) -> Scalar  {
    return Scalar(self_.g0 * other.g0.x);
}

fn scalar_motor_add(self_: Scalar, other: Motor) -> Motor  {
    return Motor(vec4<f32>(self_.g0) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + other.g0, other.g1);
}

fn scalar_motor_sub(self_: Scalar, other: Motor) -> Motor  {
    return Motor(vec4<f32>(self_.g0) * vec4<f32>(1.0, 0.0, 0.0, 0.0) - other.g0, vec4<f32>(0.0) - other.g1);
}

fn scalar_motor_geometric_product(self_: Scalar, other: Motor) -> Motor  {
    return Motor(vec4<f32>(self_.g0) * other.g0, vec4<f32>(self_.g0) * other.g1);
}

fn scalar_motor_regressive_product(self_: Scalar, other: Motor) -> Scalar  {
    return Scalar(self_.g0 * other.g1.x);
}

fn scalar_motor_outer_product(self_: Scalar, other: Motor) -> Motor  {
    return Motor(vec4<f32>(self_.g0) * other.g0, vec4<f32>(self_.g0) * other.g1);
}

fn scalar_motor_inner_product(self_: Scalar, other: Motor) -> Motor  {
    return Motor(vec4<f32>(self_.g0) * other.g0, vec4<f32>(self_.g0) * other.g1);
}

fn scalar_motor_left_contraction(self_: Scalar, other: Motor) -> Motor  {
    return Motor(vec4<f32>(self_.g0) * other.g0, vec4<f32>(self_.g0) * other.g1);
}

fn scalar_motor_right_contraction(self_: Scalar, other: Motor) -> Scalar  {
    return Scalar(self_.g0 * other.g0.x);
}

fn scalar_motor_scalar_product(self_: Scalar, other: Motor) -> Scalar  {
    return Scalar(self_.g0 * other.g0.x);
}

fn scalar_point_and_plane_geometric_product(self_: Scalar, other: PointAndPlane) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0) * other.g0, vec4<f32>(self_.g0) * other.g1);
}

fn scalar_point_and_plane_outer_product(self_: Scalar, other: PointAndPlane) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0) * other.g0, vec4<f32>(self_.g0) * other.g1);
}

fn scalar_point_and_plane_inner_product(self_: Scalar, other: PointAndPlane) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0) * other.g0, vec4<f32>(self_.g0) * other.g1);
}

fn scalar_point_and_plane_left_contraction(self_: Scalar, other: PointAndPlane) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0) * other.g0, vec4<f32>(self_.g0) * other.g1);
}

fn scalar_squared_magnitude(self_: Scalar) -> Scalar  {
    return scalar_scalar_scalar_product(self_, scalar_reversal(self_));
}

fn scalar_magnitude(self_: Scalar) -> Scalar  {
    return Scalar(sqrt(scalar_squared_magnitude(self_).g0));
}

fn scalar_scale(self_: Scalar, other: f32) -> Scalar  {
    return scalar_scalar_geometric_product(self_, Scalar(other));
}

fn scalar_signum(self_: Scalar) -> Scalar  {
    return scalar_scalar_geometric_product(self_, Scalar(1.0 / scalar_magnitude(self_).g0));
}

fn scalar_inverse(self_: Scalar) -> Scalar  {
    return scalar_scalar_geometric_product(scalar_reversal(self_), Scalar(1.0 / scalar_squared_magnitude(self_).g0));
}

fn multi_vector_zero() -> MultiVector  {
    return MultiVector(vec4<f32>(0.0), vec4<f32>(0.0), vec4<f32>(0.0), vec4<f32>(0.0));
}

fn multi_vector_one() -> MultiVector  {
    return MultiVector(vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(0.0), vec4<f32>(0.0), vec4<f32>(0.0));
}

fn multi_vector_neg(self_: MultiVector) -> MultiVector  {
    return MultiVector(self_.g0 * vec4<f32>(-1.0), self_.g1 * vec4<f32>(-1.0), self_.g2 * vec4<f32>(-1.0), self_.g3 * vec4<f32>(-1.0));
}

fn multi_vector_automorphism(self_: MultiVector) -> MultiVector  {
    return MultiVector(self_.g0, self_.g1 * vec4<f32>(-1.0), self_.g2 * vec4<f32>(-1.0), self_.g3);
}

fn multi_vector_reversal(self_: MultiVector) -> MultiVector  {
    return MultiVector(self_.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0), self_.g1 * vec4<f32>(1.0, -1.0, -1.0, -1.0), self_.g2 * vec4<f32>(-1.0, 1.0, 1.0, 1.0), self_.g3 * vec4<f32>(1.0, -1.0, -1.0, -1.0));
}

fn multi_vector_conjugation(self_: MultiVector) -> MultiVector  {
    return MultiVector(self_.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0), self_.g1 * vec4<f32>(-1.0, 1.0, 1.0, 1.0), self_.g2 * vec4<f32>(1.0, -1.0, -1.0, -1.0), self_.g3 * vec4<f32>(1.0, -1.0, -1.0, -1.0));
}

fn multi_vector_dual(self_: MultiVector) -> MultiVector  {
    return MultiVector(self_.g3, self_.g2 * vec4<f32>(-1.0, 1.0, 1.0, 1.0), self_.g1 * vec4<f32>(1.0, -1.0, -1.0, -1.0), self_.g0);
}

fn multi_vector_scalar_into(self_: MultiVector) -> Scalar  {
    return Scalar(self_.g0.x);
}

fn multi_vector_scalar_add(self_: MultiVector, other: Scalar) -> MultiVector  {
    return MultiVector(self_.g0 + vec4<f32>(other.g0) * vec4<f32>(1.0, 0.0, 0.0, 0.0), self_.g1, self_.g2, self_.g3);
}

fn multi_vector_scalar_sub(self_: MultiVector, other: Scalar) -> MultiVector  {
    return MultiVector(self_.g0 - vec4<f32>(other.g0) * vec4<f32>(1.0, 0.0, 0.0, 0.0), self_.g1, self_.g2, self_.g3);
}

fn multi_vector_scalar_geometric_product(self_: MultiVector, other: Scalar) -> MultiVector  {
    return MultiVector(self_.g0 * vec4<f32>(other.g0), self_.g1 * vec4<f32>(other.g0), self_.g2 * vec4<f32>(other.g0), self_.g3 * vec4<f32>(other.g0));
}

fn multi_vector_scalar_regressive_product(self_: MultiVector, other: Scalar) -> Scalar  {
    return Scalar(self_.g3.x * other.g0);
}

fn multi_vector_scalar_outer_product(self_: MultiVector, other: Scalar) -> MultiVector  {
    return MultiVector(self_.g0 * vec4<f32>(other.g0), self_.g1 * vec4<f32>(other.g0), self_.g2 * vec4<f32>(other.g0), self_.g3 * vec4<f32>(other.g0));
}

fn multi_vector_scalar_inner_product(self_: MultiVector, other: Scalar) -> MultiVector  {
    return MultiVector(self_.g0 * vec4<f32>(other.g0), self_.g1 * vec4<f32>(other.g0), self_.g2 * vec4<f32>(other.g0), self_.g3 * vec4<f32>(other.g0));
}

fn multi_vector_scalar_left_contraction(self_: MultiVector, other: Scalar) -> Scalar  {
    return Scalar(self_.g0.x * other.g0);
}

fn multi_vector_scalar_right_contraction(self_: MultiVector, other: Scalar) -> MultiVector  {
    return MultiVector(self_.g0 * vec4<f32>(other.g0), self_.g1 * vec4<f32>(other.g0), self_.g2 * vec4<f32>(other.g0), self_.g3 * vec4<f32>(other.g0));
}

fn multi_vector_scalar_scalar_product(self_: MultiVector, other: Scalar) -> Scalar  {
    return Scalar(self_.g0.x * other.g0);
}

fn multi_vector_multi_vector_add(self_: MultiVector, other: MultiVector) -> MultiVector  {
    return MultiVector(self_.g0 + other.g0, self_.g1 + other.g1, self_.g2 + other.g2, self_.g3 + other.g3);
}

fn multi_vector_multi_vector_sub(self_: MultiVector, other: MultiVector) -> MultiVector  {
    return MultiVector(self_.g0 - other.g0, self_.g1 - other.g1, self_.g2 - other.g2, self_.g3 - other.g3);
}

fn multi_vector_multi_vector_mul(self_: MultiVector, other: MultiVector) -> MultiVector  {
    return MultiVector(self_.g0 * other.g0, self_.g1 * other.g1, self_.g2 * other.g2, self_.g3 * other.g3);
}

fn multi_vector_multi_vector_div(self_: MultiVector, other: MultiVector) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0) / vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0), vec4<f32>(self_.g1.x, self_.g1.y, self_.g1.z, self_.g1.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0) / vec4<f32>(other.g1.x, other.g1.y, other.g1.z, other.g1.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0), vec4<f32>(self_.g2.x, self_.g2.y, self_.g2.z, self_.g2.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0) / vec4<f32>(other.g2.x, other.g2.y, other.g2.z, other.g2.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0), vec4<f32>(self_.g3.x, self_.g3.y, self_.g3.z, self_.g3.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0) / vec4<f32>(other.g3.x, other.g3.y, other.g3.z, other.g3.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0));
}

fn multi_vector_multi_vector_geometric_product(self_: MultiVector, other: MultiVector) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.y) * other.g0.yxwz * vec4<f32>(-1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g0.zwxy * vec4<f32>(-1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g0.w) * other.g0.wzyx * vec4<f32>(-1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g1.x) * other.g1 * vec4<f32>(-1.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g1.y) * other.g1.yxwz * vec4<f32>(1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g1.z) * other.g1.zwxy * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g1.w) * other.g1.wzyx * vec4<f32>(1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g2.x) * other.g2 * vec4<f32>(-1.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g2.y) * other.g2.yxwz * vec4<f32>(1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g2.z) * other.g2.zwxy * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g2.w) * other.g2.wzyx * vec4<f32>(1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g3.x) * other.g3 * vec4<f32>(-1.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g3.y) * other.g3.yxwz * vec4<f32>(1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g3.z) * other.g3.zwxy * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g3.w) * other.g3.wzyx * vec4<f32>(1.0, -1.0, 1.0, 1.0), vec4<f32>(self_.g0.x) * other.g1 + vec4<f32>(self_.g0.y) * other.g1.yxwz * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g1.zwxy * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g0.w) * other.g1.wzyx * vec4<f32>(1.0, 1.0, -1.0, -1.0) + vec4<f32>(self_.g1.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g1.y) * other.g0.yxwz * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g1.z) * other.g0.zwxy * vec4<f32>(1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g1.w) * other.g0.wzyx * vec4<f32>(1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g2.x) * other.g3 + vec4<f32>(self_.g2.y) * other.g3.yxwz * vec4<f32>(-1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g2.z) * other.g3.zwxy * vec4<f32>(-1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g2.w) * other.g3.wzyx * vec4<f32>(-1.0, -1.0, 1.0, 1.0) - vec4<f32>(self_.g3.x) * other.g2 + vec4<f32>(self_.g3.y) * other.g2.yxwz * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g3.z) * other.g2.zwxy * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g3.w) * other.g2.wzyx * vec4<f32>(1.0, 1.0, -1.0, -1.0), vec4<f32>(self_.g0.x) * other.g2 + vec4<f32>(self_.g0.y) * other.g2.yxwz * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g2.zwxy * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g0.w) * other.g2.wzyx * vec4<f32>(1.0, 1.0, -1.0, -1.0) - vec4<f32>(self_.g1.x) * other.g3 + vec4<f32>(self_.g1.y) * other.g3.yxwz * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g1.z) * other.g3.zwxy * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g1.w) * other.g3.wzyx * vec4<f32>(1.0, 1.0, -1.0, -1.0) + vec4<f32>(self_.g2.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g2.y) * other.g0.yxwz * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g2.z) * other.g0.zwxy * vec4<f32>(1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g2.w) * other.g0.wzyx * vec4<f32>(1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g3.x) * other.g1 + vec4<f32>(self_.g3.y) * other.g1.yxwz * vec4<f32>(-1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g3.z) * other.g1.zwxy * vec4<f32>(-1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g3.w) * other.g1.wzyx * vec4<f32>(-1.0, -1.0, 1.0, 1.0), vec4<f32>(self_.g0.x) * other.g3 + vec4<f32>(self_.g0.y) * other.g3.yxwz * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g3.zwxy * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g0.w) * other.g3.wzyx * vec4<f32>(1.0, 1.0, -1.0, -1.0) + vec4<f32>(self_.g1.x) * other.g2 + vec4<f32>(self_.g1.y) * other.g2.yxwz * vec4<f32>(-1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g1.z) * other.g2.zwxy * vec4<f32>(-1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g1.w) * other.g2.wzyx * vec4<f32>(-1.0, -1.0, 1.0, 1.0) - vec4<f32>(self_.g2.x) * other.g1 + vec4<f32>(self_.g2.y) * other.g1.yxwz * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g2.z) * other.g1.zwxy * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g2.w) * other.g1.wzyx * vec4<f32>(1.0, 1.0, -1.0, -1.0) + vec4<f32>(self_.g3.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g3.y) * other.g0.yxwz * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g3.z) * other.g0.zwxy * vec4<f32>(1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g3.w) * other.g0.wzyx * vec4<f32>(1.0, 1.0, -1.0, 1.0));
}

fn multi_vector_multi_vector_regressive_product(self_: MultiVector, other: MultiVector) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0.y) * other.g3.yxyy * vec4<f32>(1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g0.z) * other.g3.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g3.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g2.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.y) * other.g2.yxyy * vec4<f32>(-1.0, -1.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * other.g2.zzxz * vec4<f32>(-1.0, 0.0, -1.0, 0.0) + vec4<f32>(self_.g1.w) * other.g2.wwwx * vec4<f32>(-1.0, 0.0, 0.0, -1.0) + vec4<f32>(self_.g2.x) * other.g1 * vec4<f32>(-1.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g2.y) * vec4<f32>(other.g1.y) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g2.z) * vec4<f32>(other.g1.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g2.w) * vec4<f32>(other.g1.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g3.x) * other.g0 + vec4<f32>(self_.g3.y) * vec4<f32>(other.g0.y) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g3.z) * vec4<f32>(other.g0.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g3.w) * vec4<f32>(other.g0.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g3.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(self_.g1.y) * other.g3.yxyy * vec4<f32>(-1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * other.g3.zzxz * vec4<f32>(-1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g1.w) * other.g3.wwwx * vec4<f32>(-1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g3.x) * other.g1 + vec4<f32>(self_.g3.y) * vec4<f32>(other.g1.y) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g3.z) * vec4<f32>(other.g1.z) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g3.w) * vec4<f32>(other.g1.w) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g3.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(self_.g0.z) * other.g1.wwwy * vec4<f32>(0.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g0.w) * other.g1.zzyz * vec4<f32>(0.0, -1.0, 1.0, 0.0) + vec4<f32>(self_.g1.y) * other.g0.wwwz * vec4<f32>(0.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g1.z) * other.g0.wwwy * vec4<f32>(0.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g1.w) * other.g0.zzyz * vec4<f32>(0.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g2.x) * other.g3 + vec4<f32>(self_.g2.z) * vec4<f32>(other.g3.x) * vec4<f32>(0.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g2.w) * vec4<f32>(other.g3.x) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g3.x) * other.g2 + vec4<f32>(self_.g3.y) * vec4<f32>(other.g2.x) * vec4<f32>(0.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g3.z) * vec4<f32>(other.g2.x) * vec4<f32>(0.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g3.w) * vec4<f32>(other.g2.x) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g0.x, self_.g2.y, self_.g0.y, self_.g0.y) * vec4<f32>(other.g1.x, other.g3.x, other.g1.w, other.g1.z) * vec4<f32>(0.0, 1.0, -1.0, 1.0), vec4<f32>(self_.g1.z) * other.g1.wwwy * vec4<f32>(0.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g1.w) * other.g1.zzyz * vec4<f32>(0.0, -1.0, 1.0, 0.0) + vec4<f32>(self_.g3.x) * other.g3 + vec4<f32>(self_.g3.z) * vec4<f32>(other.g3.x) * vec4<f32>(0.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g3.w) * vec4<f32>(other.g3.x) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g1.x, self_.g3.y, self_.g1.y, self_.g1.y) * vec4<f32>(other.g1.x, other.g3.x, other.g1.w, other.g1.z) * vec4<f32>(0.0, 1.0, -1.0, 1.0));
}

fn multi_vector_multi_vector_outer_product(self_: MultiVector, other: MultiVector) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g2.y) * other.g2.wwwz * vec4<f32>(0.0, 0.0, -1.0, 1.0) + vec4<f32>(self_.g2.z) * other.g2.wwwy * vec4<f32>(0.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g2.w) * other.g2.zzyz * vec4<f32>(0.0, -1.0, 1.0, 0.0) + self_.g0 * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 1.0, 1.0), vec4<f32>(self_.g0.x) * other.g1 + vec4<f32>(self_.g1.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g2.y) * other.g3.wwwz * vec4<f32>(0.0, 0.0, -1.0, 1.0) + vec4<f32>(self_.g2.z) * other.g3.wwwy * vec4<f32>(0.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g2.w) * other.g3.zzyz * vec4<f32>(0.0, -1.0, 1.0, 0.0) + vec4<f32>(self_.g3.y) * other.g2.wwwz * vec4<f32>(0.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g3.z) * other.g2.wwwy * vec4<f32>(0.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g3.w) * other.g2.zzyz * vec4<f32>(0.0, 1.0, -1.0, 0.0) + self_.g0 * vec4<f32>(other.g1.x) * vec4<f32>(0.0, -1.0, -1.0, -1.0), vec4<f32>(self_.g0.x) * other.g2 + vec4<f32>(self_.g0.z) * vec4<f32>(other.g2.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g2.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g2.x) * vec4<f32>(other.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g2.y) * other.g0.yxyy * vec4<f32>(1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g2.z) * other.g0.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g2.w) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + self_.g0.yxxx * other.g2.yxxx * vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(self_.g0.x) * other.g3 + vec4<f32>(self_.g0.z) * vec4<f32>(other.g3.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g3.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.x) * other.g2 + vec4<f32>(self_.g1.y) * vec4<f32>(other.g2.y) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g2.z) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g2.w) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g2.x) * vec4<f32>(other.g1.x) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g2.y) * other.g1.yxyy * vec4<f32>(1.0, -1.0, 0.0, 0.0) + vec4<f32>(self_.g2.z) * other.g1.zzxz * vec4<f32>(1.0, 0.0, -1.0, 0.0) + vec4<f32>(self_.g2.w) * other.g1.wwwx * vec4<f32>(1.0, 0.0, 0.0, -1.0) + vec4<f32>(self_.g3.x) * vec4<f32>(other.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g3.y) * other.g0.yxyy * vec4<f32>(1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g3.z) * other.g0.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g3.w) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + self_.g0.yxxx * other.g3.yxxx * vec4<f32>(1.0, 0.0, 0.0, 0.0));
}

fn multi_vector_multi_vector_inner_product(self_: MultiVector, other: MultiVector) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.z) * other.g0.zzxz * vec4<f32>(-1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g0.wwwx * vec4<f32>(-1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g1.x) * other.g1 * vec4<f32>(-1.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g1.y) * other.g1.yxyy * vec4<f32>(1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * other.g1.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g1.w) * other.g1.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g2.x) * other.g2 * vec4<f32>(-1.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g2.y) * other.g2.yxyy * vec4<f32>(1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g2.z) * other.g2.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g2.w) * other.g2.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g3.x) * other.g3 * vec4<f32>(-1.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g3.y) * other.g3.yxyy * vec4<f32>(1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g3.z) * other.g3.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g3.w) * other.g3.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + self_.g0.yyxx * other.g0.yxxx * vec4<f32>(-1.0, 1.0, 0.0, 0.0), vec4<f32>(self_.g0.x) * other.g1 + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g1.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.y) * other.g0.yxyy * vec4<f32>(1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * other.g0.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g1.w) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g2.x) * vec4<f32>(other.g3.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g2.y) * other.g3.yxyy * vec4<f32>(-1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g2.z) * other.g3.zzxz * vec4<f32>(-1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g2.w) * other.g3.wwwx * vec4<f32>(-1.0, 0.0, 0.0, 1.0) - vec4<f32>(self_.g3.x) * other.g2 + vec4<f32>(self_.g3.y) * vec4<f32>(other.g2.y) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g3.z) * vec4<f32>(other.g2.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g3.w) * vec4<f32>(other.g2.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + self_.g0.yxxx * other.g1.yxxx * vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(self_.g0.x) * other.g2 + vec4<f32>(self_.g0.z) * other.g2.wwxy * vec4<f32>(0.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g0.w) * other.g2.zzyx * vec4<f32>(0.0, 1.0, -1.0, -1.0) - vec4<f32>(self_.g1.x) * other.g3 + vec4<f32>(self_.g1.y) * other.g3.xxwz * vec4<f32>(0.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g1.z) * other.g3.wwxy * vec4<f32>(0.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g1.w) * other.g3.zzyx * vec4<f32>(0.0, 1.0, -1.0, -1.0) + vec4<f32>(self_.g2.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g2.y) * other.g0.xxwz * vec4<f32>(0.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g2.z) * other.g0.wwxy * vec4<f32>(0.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g2.w) * other.g0.zzyx * vec4<f32>(0.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g3.x) * other.g1 + vec4<f32>(self_.g3.y) * other.g1.xxwz * vec4<f32>(0.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g3.z) * other.g1.wwxy * vec4<f32>(0.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g3.w) * other.g1.zzyx * vec4<f32>(0.0, -1.0, 1.0, 1.0) + self_.g0.xyyy * other.g2.xxwz * vec4<f32>(0.0, -1.0, 1.0, -1.0), vec4<f32>(self_.g0.x) * other.g3 + vec4<f32>(self_.g1.y) * other.g2.wwwz * vec4<f32>(0.0, 0.0, -1.0, 1.0) + vec4<f32>(self_.g1.z) * other.g2.wwwy * vec4<f32>(0.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g1.w) * other.g2.zzyz * vec4<f32>(0.0, -1.0, 1.0, 0.0) + vec4<f32>(self_.g2.y) * other.g1.wwwz * vec4<f32>(0.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g2.z) * other.g1.wwwy * vec4<f32>(0.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g2.w) * other.g1.zzyz * vec4<f32>(0.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g3.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g3.y) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g3.z) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g3.w) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + self_.g0 * vec4<f32>(other.g3.x) * vec4<f32>(0.0, -1.0, -1.0, -1.0));
}

fn multi_vector_multi_vector_left_contraction(self_: MultiVector, other: MultiVector) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.x) * other.g1 * vec4<f32>(-1.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g1.y) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g1.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g1.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g2.x) * vec4<f32>(other.g2.x) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g2.y) * other.g2.yxyy * vec4<f32>(1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g2.z) * other.g2.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g2.w) * other.g2.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g3.x) * vec4<f32>(other.g3.x) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g3.y) * other.g3.yxyy * vec4<f32>(1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g3.z) * other.g3.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g3.w) * other.g3.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + self_.g0.yxxx * other.g0.yxxx * vec4<f32>(-1.0, 0.0, 0.0, 0.0), vec4<f32>(self_.g0.x) * other.g1 + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g1.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g2.x) * vec4<f32>(other.g3.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g2.y) * other.g3.yxyy * vec4<f32>(-1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g2.z) * other.g3.zzxz * vec4<f32>(-1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g2.w) * other.g3.wwwx * vec4<f32>(-1.0, 0.0, 0.0, 1.0) + self_.g0.yxxx * other.g1.yxxx * vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(self_.g0.x) * other.g2 - vec4<f32>(self_.g1.x) * other.g3 + vec4<f32>(self_.g1.y) * vec4<f32>(other.g3.x) * vec4<f32>(0.0, -1.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g3.x) * vec4<f32>(0.0, 0.0, -1.0, 0.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g3.x) * vec4<f32>(0.0, 0.0, 0.0, -1.0) + vec4<f32>(self_.g2.y) * other.g0.wwwz * vec4<f32>(0.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g2.z) * other.g0.wwwy * vec4<f32>(0.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g2.w) * other.g0.zzyz * vec4<f32>(0.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g3.y) * other.g1.wwwz * vec4<f32>(0.0, 0.0, -1.0, 1.0) + vec4<f32>(self_.g3.z) * other.g1.wwwy * vec4<f32>(0.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g3.w) * other.g1.zzyz * vec4<f32>(0.0, -1.0, 1.0, 0.0) + self_.g0 * vec4<f32>(other.g2.x) * vec4<f32>(0.0, -1.0, -1.0, -1.0), vec4<f32>(self_.g0.x) * other.g3 + vec4<f32>(self_.g2.y) * other.g1.wwwz * vec4<f32>(0.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g2.z) * other.g1.wwwy * vec4<f32>(0.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g2.w) * other.g1.zzyz * vec4<f32>(0.0, 1.0, -1.0, 0.0) + self_.g0 * vec4<f32>(other.g3.x) * vec4<f32>(0.0, -1.0, -1.0, -1.0));
}

fn multi_vector_multi_vector_right_contraction(self_: MultiVector, other: MultiVector) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0.y) * other.g0.yxyy * vec4<f32>(-1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g0.z) * other.g0.zzxz * vec4<f32>(-1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g0.wwwx * vec4<f32>(-1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g1.x) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.y) * other.g1.yxyy * vec4<f32>(1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * other.g1.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g1.w) * other.g1.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g2.x) * other.g2 * vec4<f32>(-1.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g2.y) * vec4<f32>(other.g2.y) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g2.z) * vec4<f32>(other.g2.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g2.w) * vec4<f32>(other.g2.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g3.x) * other.g3 * vec4<f32>(-1.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g3.y) * vec4<f32>(other.g3.y) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g3.z) * vec4<f32>(other.g3.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g3.w) * vec4<f32>(other.g3.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(self_.g1.y) * other.g0.yxyy * vec4<f32>(1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * other.g0.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g1.w) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) - vec4<f32>(self_.g3.x) * other.g2 + vec4<f32>(self_.g3.y) * vec4<f32>(other.g2.y) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g3.z) * vec4<f32>(other.g2.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g3.w) * vec4<f32>(other.g2.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(self_.g0.z) * other.g2.wwwy * vec4<f32>(0.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * other.g2.zzyz * vec4<f32>(0.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g1.y) * other.g3.wwwz * vec4<f32>(0.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g1.z) * other.g3.wwwy * vec4<f32>(0.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g1.w) * other.g3.zzyz * vec4<f32>(0.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g2.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g2.z) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g2.w) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g3.x) * other.g1 + vec4<f32>(self_.g3.y) * vec4<f32>(other.g1.x) * vec4<f32>(0.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g3.z) * vec4<f32>(other.g1.x) * vec4<f32>(0.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g3.w) * vec4<f32>(other.g1.x) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g0.x, self_.g2.y, self_.g0.y, self_.g0.y) * vec4<f32>(other.g2.x, other.g0.x, other.g2.w, other.g2.z) * vec4<f32>(0.0, 1.0, 1.0, -1.0), vec4<f32>(self_.g1.z) * other.g2.wwwy * vec4<f32>(0.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g1.w) * other.g2.zzyz * vec4<f32>(0.0, -1.0, 1.0, 0.0) + vec4<f32>(self_.g3.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g3.z) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g3.w) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g1.x, self_.g3.y, self_.g1.y, self_.g1.y) * vec4<f32>(other.g2.x, other.g0.x, other.g2.w, other.g2.z) * vec4<f32>(0.0, 1.0, -1.0, 1.0));
}

fn multi_vector_multi_vector_scalar_product(self_: MultiVector, other: MultiVector) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.x - self_.g0.y * other.g0.y - self_.g0.z * other.g0.z - self_.g0.w * other.g0.w - self_.g1.x * other.g1.x + self_.g1.y * other.g1.y + self_.g1.z * other.g1.z + self_.g1.w * other.g1.w - self_.g2.x * other.g2.x + self_.g2.y * other.g2.y + self_.g2.z * other.g2.z + self_.g2.w * other.g2.w - self_.g3.x * other.g3.x + self_.g3.y * other.g3.y + self_.g3.z * other.g3.z + self_.g3.w * other.g3.w);
}

fn multi_vector_rotor_into(self_: MultiVector) -> Rotor  {
    return Rotor(self_.g0);
}

fn multi_vector_rotor_add(self_: MultiVector, other: Rotor) -> MultiVector  {
    return MultiVector(self_.g0 + other.g0, self_.g1, self_.g2, self_.g3);
}

fn multi_vector_rotor_sub(self_: MultiVector, other: Rotor) -> MultiVector  {
    return MultiVector(self_.g0 - other.g0, self_.g1, self_.g2, self_.g3);
}

fn multi_vector_rotor_geometric_product(self_: MultiVector, other: Rotor) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.y) * other.g0.yxwz * vec4<f32>(-1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g0.zwxy * vec4<f32>(-1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g0.w) * other.g0.wzyx * vec4<f32>(-1.0, 1.0, -1.0, 1.0), vec4<f32>(self_.g1.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g1.y) * other.g0.yxwz * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g1.z) * other.g0.zwxy * vec4<f32>(1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g1.w) * other.g0.wzyx * vec4<f32>(1.0, 1.0, -1.0, 1.0), vec4<f32>(self_.g2.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g2.y) * other.g0.yxwz * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g2.z) * other.g0.zwxy * vec4<f32>(1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g2.w) * other.g0.wzyx * vec4<f32>(1.0, 1.0, -1.0, 1.0), vec4<f32>(self_.g3.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g3.y) * other.g0.yxwz * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g3.z) * other.g0.zwxy * vec4<f32>(1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g3.w) * other.g0.wzyx * vec4<f32>(1.0, 1.0, -1.0, 1.0));
}

fn multi_vector_rotor_outer_product(self_: MultiVector, other: Rotor) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0.x) * other.g0 + self_.g0 * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 1.0, 1.0), vec4<f32>(self_.g1.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + self_.g1 * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 1.0, 1.0), vec4<f32>(self_.g2.y) * other.g0.yxyy * vec4<f32>(1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g2.z) * other.g0.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g2.w) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g2.x) * vec4<f32>(other.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(self_.g3.y) * other.g0.yxyy * vec4<f32>(1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g3.z) * other.g0.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g3.w) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g3.x) * vec4<f32>(other.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0));
}

fn multi_vector_rotor_inner_product(self_: MultiVector, other: Rotor) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.z) * other.g0.zzxz * vec4<f32>(-1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g0.wwwx * vec4<f32>(-1.0, 0.0, 0.0, 1.0) + self_.g0.yyxx * other.g0.yxxx * vec4<f32>(-1.0, 1.0, 0.0, 0.0), vec4<f32>(self_.g1.y) * other.g0.yxyy * vec4<f32>(1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * other.g0.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g1.w) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(self_.g2.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g2.z) * other.g0.wwxy * vec4<f32>(0.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g2.w) * other.g0.zzyx * vec4<f32>(0.0, 1.0, -1.0, 1.0) + self_.g2.xyyy * other.g0.xxwz * vec4<f32>(0.0, 1.0, 1.0, -1.0), vec4<f32>(self_.g3.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + self_.g3 * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 1.0, 1.0));
}

fn multi_vector_rotor_right_contraction(self_: MultiVector, other: Rotor) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0.y) * other.g0.yxyy * vec4<f32>(-1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g0.z) * other.g0.zzxz * vec4<f32>(-1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g0.wwwx * vec4<f32>(-1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(self_.g1.y) * other.g0.yxyy * vec4<f32>(1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * other.g0.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g1.w) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(self_.g2.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + self_.g2 * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 1.0, 1.0), vec4<f32>(self_.g3.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + self_.g3 * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 1.0, 1.0));
}

fn multi_vector_rotor_scalar_product(self_: MultiVector, other: Rotor) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.x - self_.g0.y * other.g0.y - self_.g0.z * other.g0.z - self_.g0.w * other.g0.w);
}

fn multi_vector_point_into(self_: MultiVector) -> Point  {
    return Point(vec4<f32>(self_.g2.x, self_.g1.y, self_.g1.z, self_.g1.w));
}

fn multi_vector_point_add(self_: MultiVector, other: Point) -> MultiVector  {
    return MultiVector(self_.g0, self_.g1 + other.g0 * vec4<f32>(0.0, 1.0, 1.0, 1.0), self_.g2 + vec4<f32>(other.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0), self_.g3);
}

fn multi_vector_point_sub(self_: MultiVector, other: Point) -> MultiVector  {
    return MultiVector(self_.g0, self_.g1 - other.g0 * vec4<f32>(0.0, 1.0, 1.0, 1.0), self_.g2 - vec4<f32>(other.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0), self_.g3);
}

fn multi_vector_point_geometric_product(self_: MultiVector, other: Point) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g1.y) * other.g0.yywz * vec4<f32>(1.0, 0.0, -1.0, 1.0) + vec4<f32>(self_.g1.z) * other.g0.zwzy * vec4<f32>(1.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g1.w) * other.g0.wzyw * vec4<f32>(1.0, -1.0, 1.0, 0.0) + vec4<f32>(self_.g2.y) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g2.z) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g2.w) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g2.x, self_.g1.x, self_.g1.x, self_.g1.x) * other.g0 * vec4<f32>(-1.0, 1.0, 1.0, 1.0), vec4<f32>(self_.g0.y) * other.g0.yywz * vec4<f32>(1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g0.zwzy * vec4<f32>(1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * other.g0.wzyw * vec4<f32>(1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g3.y) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, -1.0, 0.0, 0.0) + vec4<f32>(self_.g3.z) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, -1.0, 0.0) + vec4<f32>(self_.g3.w) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 0.0, -1.0) + vec4<f32>(self_.g3.x, self_.g0.x, self_.g0.x, self_.g0.x) * other.g0 * vec4<f32>(-1.0, 1.0, 1.0, 1.0), vec4<f32>(self_.g3.x) * other.g0.yyzw * vec4<f32>(0.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g3.y) * other.g0.yywz * vec4<f32>(-1.0, 0.0, -1.0, 1.0) + vec4<f32>(self_.g3.z) * other.g0.zwzy * vec4<f32>(-1.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g3.w) * other.g0.wzyw * vec4<f32>(-1.0, -1.0, 1.0, 0.0) + self_.g0 * vec4<f32>(other.g0.x) * vec4<f32>(1.0, -1.0, -1.0, -1.0), vec4<f32>(self_.g2.x) * other.g0.yyzw * vec4<f32>(0.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g2.y) * other.g0.yywz * vec4<f32>(1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g2.z) * other.g0.zwzy * vec4<f32>(1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g2.w) * other.g0.wzyw * vec4<f32>(1.0, 1.0, -1.0, 0.0) + self_.g1 * vec4<f32>(other.g0.x));
}

fn multi_vector_point_scalar_product(self_: MultiVector, other: Point) -> Scalar  {
    return Scalar(self_.g1.y * other.g0.y + self_.g1.z * other.g0.z + self_.g1.w * other.g0.w - self_.g2.x * other.g0.x);
}

fn multi_vector_ideal_point_into(self_: MultiVector) -> IdealPoint  {
    return IdealPoint(vec3<f32>(self_.g3.y, self_.g3.z, self_.g3.w));
}

fn multi_vector_ideal_point_add(self_: MultiVector, other: IdealPoint) -> MultiVector  {
    return MultiVector(self_.g0, self_.g1, self_.g2, self_.g3 + vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0));
}

fn multi_vector_ideal_point_sub(self_: MultiVector, other: IdealPoint) -> MultiVector  {
    return MultiVector(self_.g0, self_.g1, self_.g2, self_.g3 - vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0));
}

fn multi_vector_ideal_point_geometric_product(self_: MultiVector, other: IdealPoint) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g3.y) * vec4<f32>(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4<f32>(1.0, 0.0, -1.0, 1.0) + vec4<f32>(self_.g3.z) * vec4<f32>(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4<f32>(1.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g3.w) * vec4<f32>(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4<f32>(1.0, -1.0, 1.0, 0.0) + vec4<f32>(self_.g3.x) * vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0), vec4<f32>(self_.g2.y) * vec4<f32>(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4<f32>(-1.0, 0.0, -1.0, 1.0) + vec4<f32>(self_.g2.z) * vec4<f32>(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4<f32>(-1.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g2.w) * vec4<f32>(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4<f32>(-1.0, -1.0, 1.0, 0.0) + vec4<f32>(self_.g2.x) * vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0), vec4<f32>(self_.g1.y) * vec4<f32>(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4<f32>(1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4<f32>(1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4<f32>(1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(0.0, -1.0, -1.0, -1.0), vec4<f32>(self_.g0.y) * vec4<f32>(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4<f32>(1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4<f32>(1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4<f32>(1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0));
}

fn multi_vector_ideal_point_scalar_product(self_: MultiVector, other: IdealPoint) -> Scalar  {
    return Scalar(self_.g3.y * other.g0.x + self_.g3.z * other.g0.y + self_.g3.w * other.g0.z);
}

fn multi_vector_plane_into(self_: MultiVector) -> Plane  {
    return Plane(vec4<f32>(self_.g1.x, self_.g2.y, self_.g2.z, self_.g2.w));
}

fn multi_vector_plane_add(self_: MultiVector, other: Plane) -> MultiVector  {
    return MultiVector(self_.g0, self_.g1 + vec4<f32>(other.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0), self_.g2 + other.g0 * vec4<f32>(0.0, 1.0, 1.0, 1.0), self_.g3);
}

fn multi_vector_plane_sub(self_: MultiVector, other: Plane) -> MultiVector  {
    return MultiVector(self_.g0, self_.g1 - vec4<f32>(other.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0), self_.g2 - other.g0 * vec4<f32>(0.0, 1.0, 1.0, 1.0), self_.g3);
}

fn multi_vector_plane_geometric_product(self_: MultiVector, other: Plane) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g2.x) * other.g0.yyzw * vec4<f32>(0.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g2.y) * other.g0.yywz * vec4<f32>(1.0, 0.0, -1.0, 1.0) + vec4<f32>(self_.g2.z) * other.g0.zwzy * vec4<f32>(1.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g2.w) * other.g0.wzyw * vec4<f32>(1.0, -1.0, 1.0, 0.0) + self_.g1 * vec4<f32>(other.g0.x) * vec4<f32>(-1.0, 1.0, 1.0, 1.0), vec4<f32>(self_.g3.x) * other.g0.yyzw * vec4<f32>(0.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g3.y) * other.g0.yywz * vec4<f32>(1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g3.z) * other.g0.zwzy * vec4<f32>(1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g3.w) * other.g0.wzyw * vec4<f32>(1.0, 1.0, -1.0, 0.0) + self_.g0 * vec4<f32>(other.g0.x) * vec4<f32>(1.0, -1.0, -1.0, -1.0), vec4<f32>(self_.g0.y) * other.g0.yywz * vec4<f32>(1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g0.zwzy * vec4<f32>(1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * other.g0.wzyw * vec4<f32>(1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g3.y) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g3.z) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g3.w) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g3.x, self_.g0.x, self_.g0.x, self_.g0.x) * other.g0, vec4<f32>(self_.g1.y) * other.g0.yywz * vec4<f32>(-1.0, 0.0, -1.0, 1.0) + vec4<f32>(self_.g1.z) * other.g0.zwzy * vec4<f32>(-1.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g1.w) * other.g0.wzyw * vec4<f32>(-1.0, -1.0, 1.0, 0.0) + vec4<f32>(self_.g2.y) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, -1.0, 0.0, 0.0) + vec4<f32>(self_.g2.z) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, -1.0, 0.0) + vec4<f32>(self_.g2.w) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 0.0, -1.0) + vec4<f32>(self_.g2.x, self_.g1.x, self_.g1.x, self_.g1.x) * other.g0 * vec4<f32>(-1.0, 1.0, 1.0, 1.0));
}

fn multi_vector_plane_scalar_product(self_: MultiVector, other: Plane) -> Scalar  {
    return Scalar(0.0 - self_.g1.x * other.g0.x + self_.g2.y * other.g0.y + self_.g2.z * other.g0.z + self_.g2.w * other.g0.w);
}

fn multi_vector_line_into(self_: MultiVector) -> Line  {
    return Line(vec3<f32>(self_.g3.y, self_.g3.z, self_.g3.w), vec3<f32>(self_.g0.y, self_.g0.z, self_.g0.w));
}

fn multi_vector_line_add(self_: MultiVector, other: Line) -> MultiVector  {
    return MultiVector(self_.g0 + vec4<f32>(other.g0.x, other.g1.x, other.g1.y, other.g1.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0), self_.g1, self_.g2, self_.g3 + vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0));
}

fn multi_vector_line_sub(self_: MultiVector, other: Line) -> MultiVector  {
    return MultiVector(self_.g0 - vec4<f32>(other.g0.x, other.g1.x, other.g1.y, other.g1.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0), self_.g1, self_.g2, self_.g3 - vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0));
}

fn multi_vector_line_geometric_product(self_: MultiVector, other: Line) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0.y) * vec4<f32>(other.g1.x, other.g1.x, other.g1.z, other.g1.y) * vec4<f32>(-1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.y, other.g1.z, other.g1.y, other.g1.x) * vec4<f32>(-1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g1.z, other.g1.y, other.g1.x, other.g1.z) * vec4<f32>(-1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g3.x) * vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g3.y) * vec4<f32>(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4<f32>(1.0, 0.0, -1.0, 1.0) + vec4<f32>(self_.g3.z) * vec4<f32>(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4<f32>(1.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g3.w) * vec4<f32>(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4<f32>(1.0, -1.0, 1.0, 0.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0), vec4<f32>(self_.g1.y) * vec4<f32>(other.g1.x, other.g1.x, other.g1.z, other.g1.y) * vec4<f32>(1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g1.y, other.g1.z, other.g1.y, other.g1.x) * vec4<f32>(1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g1.z, other.g1.y, other.g1.x, other.g1.z) * vec4<f32>(1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g2.x) * vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g2.y) * vec4<f32>(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4<f32>(-1.0, 0.0, -1.0, 1.0) + vec4<f32>(self_.g2.z) * vec4<f32>(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4<f32>(-1.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g2.w) * vec4<f32>(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4<f32>(-1.0, -1.0, 1.0, 0.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4<f32>(0.0, -1.0, -1.0, -1.0), vec4<f32>(self_.g1.y) * vec4<f32>(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4<f32>(1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4<f32>(1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4<f32>(1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g2.x) * vec4<f32>(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4<f32>(0.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g2.y) * vec4<f32>(other.g1.x, other.g1.x, other.g1.z, other.g1.y) * vec4<f32>(1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g2.z) * vec4<f32>(other.g1.y, other.g1.z, other.g1.y, other.g1.x) * vec4<f32>(1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g2.w) * vec4<f32>(other.g1.z, other.g1.y, other.g1.x, other.g1.z) * vec4<f32>(1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(0.0, -1.0, -1.0, -1.0), vec4<f32>(self_.g0.y) * vec4<f32>(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4<f32>(1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4<f32>(1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4<f32>(1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g3.x) * vec4<f32>(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4<f32>(0.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g3.y) * vec4<f32>(other.g1.x, other.g1.x, other.g1.z, other.g1.y) * vec4<f32>(1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g3.z) * vec4<f32>(other.g1.y, other.g1.z, other.g1.y, other.g1.x) * vec4<f32>(1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g3.w) * vec4<f32>(other.g1.z, other.g1.y, other.g1.x, other.g1.z) * vec4<f32>(1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0));
}

fn multi_vector_line_scalar_product(self_: MultiVector, other: Line) -> Scalar  {
    return Scalar(0.0 - self_.g0.y * other.g1.x - self_.g0.z * other.g1.y - self_.g0.w * other.g1.z + self_.g3.y * other.g0.x + self_.g3.z * other.g0.y + self_.g3.w * other.g0.z);
}

fn multi_vector_translator_into(self_: MultiVector) -> Translator  {
    return Translator(vec4<f32>(self_.g0.x, self_.g3.y, self_.g3.z, self_.g3.w));
}

fn multi_vector_translator_add(self_: MultiVector, other: Translator) -> MultiVector  {
    return MultiVector(self_.g0 + vec4<f32>(other.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0), self_.g1, self_.g2, self_.g3 + other.g0 * vec4<f32>(0.0, 1.0, 1.0, 1.0));
}

fn multi_vector_translator_sub(self_: MultiVector, other: Translator) -> MultiVector  {
    return MultiVector(self_.g0 - vec4<f32>(other.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0), self_.g1, self_.g2, self_.g3 - other.g0 * vec4<f32>(0.0, 1.0, 1.0, 1.0));
}

fn multi_vector_translator_geometric_product(self_: MultiVector, other: Translator) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g3.x) * other.g0.yyzw * vec4<f32>(0.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g3.y) * other.g0.yywz * vec4<f32>(1.0, 0.0, -1.0, 1.0) + vec4<f32>(self_.g3.z) * other.g0.zwzy * vec4<f32>(1.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g3.w) * other.g0.wzyw * vec4<f32>(1.0, -1.0, 1.0, 0.0) + self_.g0 * vec4<f32>(other.g0.x), vec4<f32>(self_.g2.x) * other.g0.yyzw * vec4<f32>(0.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g2.y) * other.g0.yywz * vec4<f32>(-1.0, 0.0, -1.0, 1.0) + vec4<f32>(self_.g2.z) * other.g0.zwzy * vec4<f32>(-1.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g2.w) * other.g0.wzyw * vec4<f32>(-1.0, -1.0, 1.0, 0.0) + self_.g1 * vec4<f32>(other.g0.x), vec4<f32>(self_.g1.y) * other.g0.yywz * vec4<f32>(1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g1.z) * other.g0.zwzy * vec4<f32>(1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g1.w) * other.g0.wzyw * vec4<f32>(1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g2.y) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g2.z) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g2.w) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g2.x, self_.g1.x, self_.g1.x, self_.g1.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0), vec4<f32>(self_.g0.y) * other.g0.yywz * vec4<f32>(1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g0.zwzy * vec4<f32>(1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * other.g0.wzyw * vec4<f32>(1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g3.y) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g3.z) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g3.w) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g3.x, self_.g0.x, self_.g0.x, self_.g0.x) * other.g0);
}

fn multi_vector_translator_outer_product(self_: MultiVector, other: Translator) -> MultiVector  {
    return MultiVector(self_.g0 * vec4<f32>(other.g0.x), vec4<f32>(self_.g2.y) * other.g0.wwwz * vec4<f32>(0.0, 0.0, -1.0, 1.0) + vec4<f32>(self_.g2.z) * other.g0.wwwy * vec4<f32>(0.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g2.w) * other.g0.zzyz * vec4<f32>(0.0, -1.0, 1.0, 0.0) + self_.g1 * vec4<f32>(other.g0.x), self_.g2 * vec4<f32>(other.g0.x), vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g3.x) * vec4<f32>(other.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g3.y) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g3.z) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g3.w) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + self_.g0.yxxx * other.g0.yyzw);
}

fn multi_vector_translator_inner_product(self_: MultiVector, other: Translator) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g3.x) * other.g0.yyzw * vec4<f32>(0.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g3.y) * vec4<f32>(other.g0.y) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g3.z) * vec4<f32>(other.g0.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g3.w) * vec4<f32>(other.g0.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + self_.g0 * vec4<f32>(other.g0.x), vec4<f32>(self_.g2.y) * vec4<f32>(other.g0.y) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g2.z) * vec4<f32>(other.g0.z) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g2.w) * vec4<f32>(other.g0.w) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + self_.g1 * vec4<f32>(other.g0.x), vec4<f32>(self_.g1.y) * other.g0.wwwz * vec4<f32>(0.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g1.z) * other.g0.wwwy * vec4<f32>(0.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g1.w) * other.g0.zzyz * vec4<f32>(0.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g2.y) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g2.z) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g2.w) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g2.x, self_.g1.x, self_.g1.x, self_.g1.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0), vec4<f32>(self_.g3.y) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g3.z) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g3.w) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g3.x, self_.g0.x, self_.g0.x, self_.g0.x) * other.g0);
}

fn multi_vector_translator_right_contraction(self_: MultiVector, other: Translator) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g3.x) * other.g0.yyzw * vec4<f32>(0.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g3.y) * vec4<f32>(other.g0.y) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g3.z) * vec4<f32>(other.g0.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g3.w) * vec4<f32>(other.g0.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + self_.g0 * vec4<f32>(other.g0.x), self_.g1 * vec4<f32>(other.g0.x), vec4<f32>(self_.g1.z) * other.g0.wwwy * vec4<f32>(0.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g1.w) * other.g0.zzyz * vec4<f32>(0.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g2.z) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g2.w) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g2.x, self_.g2.y, self_.g1.y, self_.g1.y) * other.g0.xxwz * vec4<f32>(1.0, 1.0, 1.0, -1.0), self_.g3 * vec4<f32>(other.g0.x));
}

fn multi_vector_translator_scalar_product(self_: MultiVector, other: Translator) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.x + self_.g3.y * other.g0.y + self_.g3.z * other.g0.z + self_.g3.w * other.g0.w);
}

fn multi_vector_motor_into(self_: MultiVector) -> Motor  {
    return Motor(self_.g0, self_.g3);
}

fn multi_vector_motor_add(self_: MultiVector, other: Motor) -> MultiVector  {
    return MultiVector(self_.g0 + other.g0, self_.g1, self_.g2, self_.g3 + other.g1);
}

fn multi_vector_motor_sub(self_: MultiVector, other: Motor) -> MultiVector  {
    return MultiVector(self_.g0 - other.g0, self_.g1, self_.g2, self_.g3 - other.g1);
}

fn multi_vector_motor_geometric_product(self_: MultiVector, other: Motor) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.y) * other.g0.yxwz * vec4<f32>(-1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g0.zwxy * vec4<f32>(-1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g0.w) * other.g0.wzyx * vec4<f32>(-1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g3.x) * other.g1 * vec4<f32>(-1.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g3.y) * other.g1.yxwz * vec4<f32>(1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g3.z) * other.g1.zwxy * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g3.w) * other.g1.wzyx * vec4<f32>(1.0, -1.0, 1.0, 1.0), vec4<f32>(self_.g1.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g1.y) * other.g0.yxwz * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g1.z) * other.g0.zwxy * vec4<f32>(1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g1.w) * other.g0.wzyx * vec4<f32>(1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g2.x) * other.g1 + vec4<f32>(self_.g2.y) * other.g1.yxwz * vec4<f32>(-1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g2.z) * other.g1.zwxy * vec4<f32>(-1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g2.w) * other.g1.wzyx * vec4<f32>(-1.0, -1.0, 1.0, 1.0), vec4<f32>(0.0) - vec4<f32>(self_.g1.x) * other.g1 + vec4<f32>(self_.g1.y) * other.g1.yxwz * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g1.z) * other.g1.zwxy * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g1.w) * other.g1.wzyx * vec4<f32>(1.0, 1.0, -1.0, -1.0) + vec4<f32>(self_.g2.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g2.y) * other.g0.yxwz * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g2.z) * other.g0.zwxy * vec4<f32>(1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g2.w) * other.g0.wzyx * vec4<f32>(1.0, 1.0, -1.0, 1.0), vec4<f32>(self_.g0.x) * other.g1 + vec4<f32>(self_.g0.y) * other.g1.yxwz * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g1.zwxy * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g0.w) * other.g1.wzyx * vec4<f32>(1.0, 1.0, -1.0, -1.0) + vec4<f32>(self_.g3.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g3.y) * other.g0.yxwz * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g3.z) * other.g0.zwxy * vec4<f32>(1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g3.w) * other.g0.wzyx * vec4<f32>(1.0, 1.0, -1.0, 1.0));
}

fn multi_vector_motor_regressive_product(self_: MultiVector, other: Motor) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0.y) * other.g1.yxyy * vec4<f32>(1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g0.z) * other.g1.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g1.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g3.x) * other.g0 + vec4<f32>(self_.g3.y) * vec4<f32>(other.g0.y) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g3.z) * vec4<f32>(other.g0.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g3.w) * vec4<f32>(other.g0.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g1.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(self_.g1.y) * other.g1.yxyy * vec4<f32>(-1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * other.g1.zzxz * vec4<f32>(-1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g1.w) * other.g1.wwwx * vec4<f32>(-1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g1.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(self_.g1.z) * other.g0.wwwy * vec4<f32>(0.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g1.w) * other.g0.zzyz * vec4<f32>(0.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g2.x) * other.g1 + vec4<f32>(self_.g2.z) * vec4<f32>(other.g1.x) * vec4<f32>(0.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g2.w) * vec4<f32>(other.g1.x) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g1.x, self_.g2.y, self_.g1.y, self_.g1.y) * vec4<f32>(other.g0.x, other.g1.x, other.g0.w, other.g0.z) * vec4<f32>(0.0, 1.0, 1.0, -1.0), vec4<f32>(self_.g3.x) * other.g1 + self_.g3 * vec4<f32>(other.g1.x) * vec4<f32>(0.0, 1.0, 1.0, 1.0));
}

fn multi_vector_motor_outer_product(self_: MultiVector, other: Motor) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0.x) * other.g0 + self_.g0 * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 1.0, 1.0), vec4<f32>(self_.g1.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g2.y) * other.g1.wwwz * vec4<f32>(0.0, 0.0, -1.0, 1.0) + vec4<f32>(self_.g2.z) * other.g1.wwwy * vec4<f32>(0.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g2.w) * other.g1.zzyz * vec4<f32>(0.0, -1.0, 1.0, 0.0) + self_.g1 * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 1.0, 1.0), vec4<f32>(self_.g2.y) * other.g0.yxyy * vec4<f32>(1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g2.z) * other.g0.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g2.w) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g2.x) * vec4<f32>(other.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(self_.g0.x) * other.g1 + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g1.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g3.x) * vec4<f32>(other.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g3.y) * other.g0.yxyy * vec4<f32>(1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g3.z) * other.g0.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g3.w) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + self_.g0.yxxx * other.g1.yxxx * vec4<f32>(1.0, 0.0, 0.0, 0.0));
}

fn multi_vector_motor_inner_product(self_: MultiVector, other: Motor) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.z) * other.g0.zzxz * vec4<f32>(-1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g0.wwwx * vec4<f32>(-1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g3.x) * other.g1 * vec4<f32>(-1.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g3.y) * other.g1.yxyy * vec4<f32>(1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g3.z) * other.g1.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g3.w) * other.g1.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + self_.g0.yyxx * other.g0.yxxx * vec4<f32>(-1.0, 1.0, 0.0, 0.0), vec4<f32>(self_.g1.y) * other.g0.yxyy * vec4<f32>(1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * other.g0.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g1.w) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g2.x) * vec4<f32>(other.g1.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g2.y) * other.g1.yxyy * vec4<f32>(-1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g2.z) * other.g1.zzxz * vec4<f32>(-1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g2.w) * other.g1.wwwx * vec4<f32>(-1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(0.0) - vec4<f32>(self_.g1.x) * other.g1 + vec4<f32>(self_.g1.z) * other.g1.wwxy * vec4<f32>(0.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g1.w) * other.g1.zzyx * vec4<f32>(0.0, 1.0, -1.0, -1.0) + vec4<f32>(self_.g2.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g2.y) * other.g0.xxwz * vec4<f32>(0.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g2.z) * other.g0.wwxy * vec4<f32>(0.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g2.w) * other.g0.zzyx * vec4<f32>(0.0, 1.0, -1.0, 1.0) + self_.g1.xyyy * other.g1.xxwz * vec4<f32>(0.0, -1.0, 1.0, -1.0), vec4<f32>(self_.g0.x) * other.g1 + vec4<f32>(self_.g3.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g3.y) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g3.z) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g3.w) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + self_.g0 * vec4<f32>(other.g1.x) * vec4<f32>(0.0, -1.0, -1.0, -1.0));
}

fn multi_vector_motor_left_contraction(self_: MultiVector, other: Motor) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g3.x) * vec4<f32>(other.g1.x) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g3.y) * other.g1.yxyy * vec4<f32>(1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g3.z) * other.g1.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g3.w) * other.g1.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + self_.g0.yxxx * other.g0.yxxx * vec4<f32>(-1.0, 0.0, 0.0, 0.0), vec4<f32>(self_.g2.y) * other.g1.yxyy * vec4<f32>(-1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g2.z) * other.g1.zzxz * vec4<f32>(-1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g2.w) * other.g1.wwwx * vec4<f32>(-1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g2.x) * vec4<f32>(other.g1.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(0.0) - vec4<f32>(self_.g1.x) * other.g1 + vec4<f32>(self_.g2.y) * other.g0.wwwz * vec4<f32>(0.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g2.z) * other.g0.wwwy * vec4<f32>(0.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g2.w) * other.g0.zzyz * vec4<f32>(0.0, 1.0, -1.0, 0.0) + self_.g1 * vec4<f32>(other.g1.x) * vec4<f32>(0.0, -1.0, -1.0, -1.0), vec4<f32>(self_.g0.x) * other.g1 + self_.g0 * vec4<f32>(other.g1.x) * vec4<f32>(0.0, -1.0, -1.0, -1.0));
}

fn multi_vector_motor_right_contraction(self_: MultiVector, other: Motor) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0.y) * other.g0.yxyy * vec4<f32>(-1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g0.z) * other.g0.zzxz * vec4<f32>(-1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g0.wwwx * vec4<f32>(-1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g3.x) * other.g1 * vec4<f32>(-1.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g3.y) * vec4<f32>(other.g1.y) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g3.z) * vec4<f32>(other.g1.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g3.w) * vec4<f32>(other.g1.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(self_.g1.y) * other.g0.yxyy * vec4<f32>(1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * other.g0.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g1.w) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(self_.g1.z) * other.g1.wwwy * vec4<f32>(0.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g1.w) * other.g1.zzyz * vec4<f32>(0.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g2.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g2.z) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g2.w) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g1.x, self_.g2.y, self_.g1.y, self_.g1.y) * vec4<f32>(other.g1.x, other.g0.x, other.g1.w, other.g1.z) * vec4<f32>(0.0, 1.0, 1.0, -1.0), vec4<f32>(self_.g3.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + self_.g3 * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 1.0, 1.0));
}

fn multi_vector_motor_scalar_product(self_: MultiVector, other: Motor) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.x - self_.g0.y * other.g0.y - self_.g0.z * other.g0.z - self_.g0.w * other.g0.w - self_.g3.x * other.g1.x + self_.g3.y * other.g1.y + self_.g3.z * other.g1.z + self_.g3.w * other.g1.w);
}

fn multi_vector_point_and_plane_into(self_: MultiVector) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g2.x, self_.g1.y, self_.g1.z, self_.g1.w), vec4<f32>(self_.g1.x, self_.g2.y, self_.g2.z, self_.g2.w));
}

fn multi_vector_point_and_plane_add(self_: MultiVector, other: PointAndPlane) -> MultiVector  {
    return MultiVector(self_.g0, self_.g1 + vec4<f32>(other.g1.x, other.g0.y, other.g0.z, other.g0.w), self_.g2 + vec4<f32>(other.g0.x, other.g1.y, other.g1.z, other.g1.w), self_.g3);
}

fn multi_vector_point_and_plane_sub(self_: MultiVector, other: PointAndPlane) -> MultiVector  {
    return MultiVector(self_.g0, self_.g1 - vec4<f32>(other.g1.x, other.g0.y, other.g0.z, other.g0.w), self_.g2 - vec4<f32>(other.g0.x, other.g1.y, other.g1.z, other.g1.w), self_.g3);
}

fn multi_vector_point_and_plane_geometric_product(self_: MultiVector, other: PointAndPlane) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g1.x) * vec4<f32>(other.g1.x, other.g0.y, other.g0.z, other.g0.w) * vec4<f32>(-1.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g0.y, other.g1.x, other.g0.w, other.g0.z) * vec4<f32>(1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.z, other.g0.w, other.g1.x, other.g0.y) * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g0.w, other.g0.z, other.g0.y, other.g1.x) * vec4<f32>(1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g2.x) * vec4<f32>(other.g0.x, other.g1.y, other.g1.z, other.g1.w) * vec4<f32>(-1.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g2.y) * vec4<f32>(other.g1.y, other.g0.x, other.g1.w, other.g1.z) * vec4<f32>(1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g2.z) * vec4<f32>(other.g1.z, other.g1.w, other.g0.x, other.g1.y) * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g2.w) * vec4<f32>(other.g1.w, other.g1.z, other.g1.y, other.g0.x) * vec4<f32>(1.0, -1.0, 1.0, 1.0), vec4<f32>(self_.g0.x) * vec4<f32>(other.g1.x, other.g0.y, other.g0.z, other.g0.w) + vec4<f32>(self_.g0.y) * vec4<f32>(other.g0.y, other.g1.x, other.g0.w, other.g0.z) * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z, other.g0.w, other.g1.x, other.g0.y) * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w, other.g0.z, other.g0.y, other.g1.x) * vec4<f32>(1.0, 1.0, -1.0, -1.0) - vec4<f32>(self_.g3.x) * vec4<f32>(other.g0.x, other.g1.y, other.g1.z, other.g1.w) + vec4<f32>(self_.g3.y) * vec4<f32>(other.g1.y, other.g0.x, other.g1.w, other.g1.z) * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g3.z) * vec4<f32>(other.g1.z, other.g1.w, other.g0.x, other.g1.y) * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g3.w) * vec4<f32>(other.g1.w, other.g1.z, other.g1.y, other.g0.x) * vec4<f32>(1.0, 1.0, -1.0, -1.0), vec4<f32>(self_.g0.x) * vec4<f32>(other.g0.x, other.g1.y, other.g1.z, other.g1.w) + vec4<f32>(self_.g0.y) * vec4<f32>(other.g1.y, other.g0.x, other.g1.w, other.g1.z) * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.z, other.g1.w, other.g0.x, other.g1.y) * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g1.w, other.g1.z, other.g1.y, other.g0.x) * vec4<f32>(1.0, 1.0, -1.0, -1.0) + vec4<f32>(self_.g3.x) * vec4<f32>(other.g1.x, other.g0.y, other.g0.z, other.g0.w) + vec4<f32>(self_.g3.y) * vec4<f32>(other.g0.y, other.g1.x, other.g0.w, other.g0.z) * vec4<f32>(-1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g3.z) * vec4<f32>(other.g0.z, other.g0.w, other.g1.x, other.g0.y) * vec4<f32>(-1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g3.w) * vec4<f32>(other.g0.w, other.g0.z, other.g0.y, other.g1.x) * vec4<f32>(-1.0, -1.0, 1.0, 1.0), vec4<f32>(self_.g1.x) * vec4<f32>(other.g0.x, other.g1.y, other.g1.z, other.g1.w) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g1.y, other.g0.x, other.g1.w, other.g1.z) * vec4<f32>(-1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g1.z, other.g1.w, other.g0.x, other.g1.y) * vec4<f32>(-1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g1.w, other.g1.z, other.g1.y, other.g0.x) * vec4<f32>(-1.0, -1.0, 1.0, 1.0) - vec4<f32>(self_.g2.x) * vec4<f32>(other.g1.x, other.g0.y, other.g0.z, other.g0.w) + vec4<f32>(self_.g2.y) * vec4<f32>(other.g0.y, other.g1.x, other.g0.w, other.g0.z) * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g2.z) * vec4<f32>(other.g0.z, other.g0.w, other.g1.x, other.g0.y) * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g2.w) * vec4<f32>(other.g0.w, other.g0.z, other.g0.y, other.g1.x) * vec4<f32>(1.0, 1.0, -1.0, -1.0));
}

fn multi_vector_point_and_plane_scalar_product(self_: MultiVector, other: PointAndPlane) -> Scalar  {
    return Scalar(0.0 - self_.g1.x * other.g1.x + self_.g1.y * other.g0.y + self_.g1.z * other.g0.z + self_.g1.w * other.g0.w - self_.g2.x * other.g0.x + self_.g2.y * other.g1.y + self_.g2.z * other.g1.z + self_.g2.w * other.g1.w);
}

fn multi_vector_squared_magnitude(self_: MultiVector) -> Scalar  {
    return multi_vector_multi_vector_scalar_product(self_, multi_vector_reversal(self_));
}

fn multi_vector_magnitude(self_: MultiVector) -> Scalar  {
    return Scalar(sqrt(multi_vector_squared_magnitude(self_).g0));
}

fn multi_vector_scale(self_: MultiVector, other: f32) -> MultiVector  {
    return multi_vector_scalar_geometric_product(self_, Scalar(other));
}

fn multi_vector_signum(self_: MultiVector) -> MultiVector  {
    return multi_vector_scalar_geometric_product(self_, Scalar(1.0 / multi_vector_magnitude(self_).g0));
}

fn multi_vector_inverse(self_: MultiVector) -> MultiVector  {
    return multi_vector_scalar_geometric_product(multi_vector_reversal(self_), Scalar(1.0 / multi_vector_squared_magnitude(self_).g0));
}

fn rotor_zero() -> Rotor  {
    return Rotor(vec4<f32>(0.0));
}

fn rotor_one() -> Rotor  {
    return Rotor(vec4<f32>(1.0, 0.0, 0.0, 0.0));
}

fn rotor_neg(self_: Rotor) -> Rotor  {
    return Rotor(self_.g0 * vec4<f32>(-1.0));
}

fn rotor_automorphism(self_: Rotor) -> Rotor  {
    return Rotor(self_.g0);
}

fn rotor_reversal(self_: Rotor) -> Rotor  {
    return Rotor(self_.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0));
}

fn rotor_conjugation(self_: Rotor) -> Rotor  {
    return Rotor(self_.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0));
}

fn rotor_scalar_into(self_: Rotor) -> Scalar  {
    return Scalar(self_.g0.x);
}

fn rotor_scalar_add(self_: Rotor, other: Scalar) -> Rotor  {
    return Rotor(self_.g0 + vec4<f32>(other.g0) * vec4<f32>(1.0, 0.0, 0.0, 0.0));
}

fn rotor_scalar_sub(self_: Rotor, other: Scalar) -> Rotor  {
    return Rotor(self_.g0 - vec4<f32>(other.g0) * vec4<f32>(1.0, 0.0, 0.0, 0.0));
}

fn rotor_scalar_geometric_product(self_: Rotor, other: Scalar) -> Rotor  {
    return Rotor(self_.g0 * vec4<f32>(other.g0));
}

fn rotor_scalar_outer_product(self_: Rotor, other: Scalar) -> Rotor  {
    return Rotor(self_.g0 * vec4<f32>(other.g0));
}

fn rotor_scalar_inner_product(self_: Rotor, other: Scalar) -> Rotor  {
    return Rotor(self_.g0 * vec4<f32>(other.g0));
}

fn rotor_scalar_left_contraction(self_: Rotor, other: Scalar) -> Scalar  {
    return Scalar(self_.g0.x * other.g0);
}

fn rotor_scalar_right_contraction(self_: Rotor, other: Scalar) -> Rotor  {
    return Rotor(self_.g0 * vec4<f32>(other.g0));
}

fn rotor_scalar_scalar_product(self_: Rotor, other: Scalar) -> Scalar  {
    return Scalar(self_.g0.x * other.g0);
}

fn rotor_multi_vector_add(self_: Rotor, other: MultiVector) -> MultiVector  {
    return MultiVector(self_.g0 + other.g0, other.g1, other.g2, other.g3);
}

fn rotor_multi_vector_sub(self_: Rotor, other: MultiVector) -> MultiVector  {
    return MultiVector(self_.g0 - other.g0, vec4<f32>(0.0) - other.g1, vec4<f32>(0.0) - other.g2, vec4<f32>(0.0) - other.g3);
}

fn rotor_multi_vector_geometric_product(self_: Rotor, other: MultiVector) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.y) * other.g0.yxwz * vec4<f32>(-1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g0.zwxy * vec4<f32>(-1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g0.w) * other.g0.wzyx * vec4<f32>(-1.0, 1.0, -1.0, 1.0), vec4<f32>(self_.g0.x) * other.g1 + vec4<f32>(self_.g0.y) * other.g1.yxwz * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g1.zwxy * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g0.w) * other.g1.wzyx * vec4<f32>(1.0, 1.0, -1.0, -1.0), vec4<f32>(self_.g0.x) * other.g2 + vec4<f32>(self_.g0.y) * other.g2.yxwz * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g2.zwxy * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g0.w) * other.g2.wzyx * vec4<f32>(1.0, 1.0, -1.0, -1.0), vec4<f32>(self_.g0.x) * other.g3 + vec4<f32>(self_.g0.y) * other.g3.yxwz * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g3.zwxy * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g0.w) * other.g3.wzyx * vec4<f32>(1.0, 1.0, -1.0, -1.0));
}

fn rotor_multi_vector_outer_product(self_: Rotor, other: MultiVector) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0.x) * other.g0 + self_.g0 * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 1.0, 1.0), vec4<f32>(self_.g0.x) * other.g1 + self_.g0 * vec4<f32>(other.g1.x) * vec4<f32>(0.0, -1.0, -1.0, -1.0), vec4<f32>(self_.g0.x) * other.g2 + vec4<f32>(self_.g0.z) * vec4<f32>(other.g2.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g2.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + self_.g0.yxxx * other.g2.yxxx * vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(self_.g0.x) * other.g3 + vec4<f32>(self_.g0.z) * vec4<f32>(other.g3.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g3.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + self_.g0.yxxx * other.g3.yxxx * vec4<f32>(1.0, 0.0, 0.0, 0.0));
}

fn rotor_multi_vector_inner_product(self_: Rotor, other: MultiVector) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.z) * other.g0.zzxz * vec4<f32>(-1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g0.wwwx * vec4<f32>(-1.0, 0.0, 0.0, 1.0) + self_.g0.yyxx * other.g0.yxxx * vec4<f32>(-1.0, 1.0, 0.0, 0.0), vec4<f32>(self_.g0.x) * other.g1 + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g1.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + self_.g0.yxxx * other.g1.yxxx * vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(self_.g0.x) * other.g2 + vec4<f32>(self_.g0.z) * other.g2.wwxy * vec4<f32>(0.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g0.w) * other.g2.zzyx * vec4<f32>(0.0, 1.0, -1.0, -1.0) + self_.g0.xyyy * other.g2.xxwz * vec4<f32>(0.0, -1.0, 1.0, -1.0), vec4<f32>(self_.g0.x) * other.g3 + self_.g0 * vec4<f32>(other.g3.x) * vec4<f32>(0.0, -1.0, -1.0, -1.0));
}

fn rotor_multi_vector_left_contraction(self_: Rotor, other: MultiVector) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + self_.g0.yxxx * other.g0.yxxx * vec4<f32>(-1.0, 0.0, 0.0, 0.0), vec4<f32>(self_.g0.x) * other.g1 + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g1.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + self_.g0.yxxx * other.g1.yxxx * vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(self_.g0.x) * other.g2 + self_.g0 * vec4<f32>(other.g2.x) * vec4<f32>(0.0, -1.0, -1.0, -1.0), vec4<f32>(self_.g0.x) * other.g3 + self_.g0 * vec4<f32>(other.g3.x) * vec4<f32>(0.0, -1.0, -1.0, -1.0));
}

fn rotor_multi_vector_scalar_product(self_: Rotor, other: MultiVector) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.x - self_.g0.y * other.g0.y - self_.g0.z * other.g0.z - self_.g0.w * other.g0.w);
}

fn rotor_rotor_add(self_: Rotor, other: Rotor) -> Rotor  {
    return Rotor(self_.g0 + other.g0);
}

fn rotor_rotor_sub(self_: Rotor, other: Rotor) -> Rotor  {
    return Rotor(self_.g0 - other.g0);
}

fn rotor_rotor_mul(self_: Rotor, other: Rotor) -> Rotor  {
    return Rotor(self_.g0 * other.g0);
}

fn rotor_rotor_div(self_: Rotor, other: Rotor) -> Rotor  {
    return Rotor(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0) / vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0));
}

fn rotor_rotor_geometric_product(self_: Rotor, other: Rotor) -> Rotor  {
    return Rotor(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.y) * other.g0.yxwz * vec4<f32>(-1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g0.zwxy * vec4<f32>(-1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g0.w) * other.g0.wzyx * vec4<f32>(-1.0, 1.0, -1.0, 1.0));
}

fn rotor_rotor_outer_product(self_: Rotor, other: Rotor) -> Rotor  {
    return Rotor(vec4<f32>(self_.g0.x) * other.g0 + self_.g0 * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 1.0, 1.0));
}

fn rotor_rotor_inner_product(self_: Rotor, other: Rotor) -> Rotor  {
    return Rotor(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.z) * other.g0.zzxz * vec4<f32>(-1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g0.wwwx * vec4<f32>(-1.0, 0.0, 0.0, 1.0) + self_.g0.yyxx * other.g0.yxxx * vec4<f32>(-1.0, 1.0, 0.0, 0.0));
}

fn rotor_rotor_left_contraction(self_: Rotor, other: Rotor) -> Rotor  {
    return Rotor(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + self_.g0.yxxx * other.g0.yxxx * vec4<f32>(-1.0, 0.0, 0.0, 0.0));
}

fn rotor_rotor_right_contraction(self_: Rotor, other: Rotor) -> Rotor  {
    return Rotor(vec4<f32>(self_.g0.y) * other.g0.yxyy * vec4<f32>(-1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g0.z) * other.g0.zzxz * vec4<f32>(-1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g0.wwwx * vec4<f32>(-1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0));
}

fn rotor_rotor_scalar_product(self_: Rotor, other: Rotor) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.x - self_.g0.y * other.g0.y - self_.g0.z * other.g0.z - self_.g0.w * other.g0.w);
}

fn rotor_point_geometric_product(self_: Rotor, other: Point) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.z) * other.g0.wwwy * vec4<f32>(0.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * other.g0.zzyz * vec4<f32>(0.0, 1.0, -1.0, 0.0) + self_.g0.xxyy * other.g0.xxwz * vec4<f32>(0.0, 0.0, 1.0, -1.0), vec4<f32>(self_.g0.z) * other.g0.zzxz * vec4<f32>(1.0, 0.0, -1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, -1.0) + self_.g0.yyxx * other.g0.yxxx * vec4<f32>(1.0, -1.0, 0.0, 0.0));
}

fn rotor_point_outer_product(self_: Rotor, other: Point) -> Point  {
    return Point(vec4<f32>(self_.g0.x) * other.g0);
}

fn rotor_point_inner_product(self_: Rotor, other: Point) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0.x) * other.g0, vec4<f32>(self_.g0.z) * other.g0.zzxz * vec4<f32>(1.0, 0.0, -1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, -1.0) + self_.g0.yyxx * other.g0.yxxx * vec4<f32>(1.0, -1.0, 0.0, 0.0));
}

fn rotor_point_left_contraction(self_: Rotor, other: Point) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0.x) * other.g0, vec4<f32>(self_.g0.z) * other.g0.zzxz * vec4<f32>(1.0, 0.0, -1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, -1.0) + self_.g0.yyxx * other.g0.yxxx * vec4<f32>(1.0, -1.0, 0.0, 0.0));
}

fn rotor_ideal_point_regressive_product(self_: Rotor, other: IdealPoint) -> Scalar  {
    return Scalar(self_.g0.y * other.g0.x + self_.g0.z * other.g0.y + self_.g0.w * other.g0.z);
}

fn rotor_ideal_point_inner_product(self_: Rotor, other: IdealPoint) -> IdealPoint  {
    return IdealPoint(vec3<f32>(self_.g0.x) * other.g0);
}

fn rotor_ideal_point_left_contraction(self_: Rotor, other: IdealPoint) -> IdealPoint  {
    return IdealPoint(vec3<f32>(self_.g0.x) * other.g0);
}

fn rotor_plane_geometric_product(self_: Rotor, other: Plane) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0.z) * other.g0.zzxz * vec4<f32>(1.0, 0.0, -1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, -1.0) + self_.g0.yyxx * other.g0.yxxx * vec4<f32>(1.0, -1.0, 0.0, 0.0), vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.z) * other.g0.wwwy * vec4<f32>(0.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * other.g0.zzyz * vec4<f32>(0.0, 1.0, -1.0, 0.0) + self_.g0.xxyy * other.g0.xxwz * vec4<f32>(0.0, 0.0, 1.0, -1.0));
}

fn rotor_plane_outer_product(self_: Rotor, other: Plane) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0.z) * other.g0.zzxz * vec4<f32>(1.0, 0.0, -1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, -1.0) + self_.g0.yyxx * other.g0.yxxx * vec4<f32>(1.0, -1.0, 0.0, 0.0), vec4<f32>(self_.g0.x) * other.g0);
}

fn rotor_plane_inner_product(self_: Rotor, other: Plane) -> Plane  {
    return Plane(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.z) * other.g0.wwwy * vec4<f32>(0.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * other.g0.zzyz * vec4<f32>(0.0, 1.0, -1.0, 0.0) + self_.g0.xxyy * other.g0.xxwz * vec4<f32>(0.0, 0.0, 1.0, -1.0));
}

fn rotor_plane_left_contraction(self_: Rotor, other: Plane) -> Plane  {
    return Plane(vec4<f32>(self_.g0.x) * other.g0);
}

fn rotor_line_geometric_product(self_: Rotor, other: Line) -> Motor  {
    return Motor(vec4<f32>(self_.g0.y) * vec4<f32>(other.g1.x, other.g1.x, other.g1.z, other.g1.y) * vec4<f32>(-1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.y, other.g1.z, other.g1.y, other.g1.x) * vec4<f32>(-1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g1.z, other.g1.y, other.g1.x, other.g1.z) * vec4<f32>(-1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0), vec4<f32>(self_.g0.y) * vec4<f32>(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4<f32>(1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4<f32>(1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4<f32>(1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0));
}

fn rotor_line_regressive_product(self_: Rotor, other: Line) -> Scalar  {
    return Scalar(self_.g0.y * other.g0.x + self_.g0.z * other.g0.y + self_.g0.w * other.g0.z);
}

fn rotor_line_right_contraction(self_: Rotor, other: Line) -> Scalar  {
    return Scalar(0.0 - self_.g0.y * other.g1.x - self_.g0.z * other.g1.y - self_.g0.w * other.g1.z);
}

fn rotor_line_scalar_product(self_: Rotor, other: Line) -> Scalar  {
    return Scalar(0.0 - self_.g0.y * other.g1.x - self_.g0.z * other.g1.y - self_.g0.w * other.g1.z);
}

fn rotor_translator_geometric_product(self_: Rotor, other: Translator) -> Motor  {
    return Motor(self_.g0 * vec4<f32>(other.g0.x), vec4<f32>(self_.g0.y) * other.g0.yywz * vec4<f32>(1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g0.zwzy * vec4<f32>(1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * other.g0.wzyw * vec4<f32>(1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g0.x) * other.g0 * vec4<f32>(0.0, 1.0, 1.0, 1.0));
}

fn rotor_translator_regressive_product(self_: Rotor, other: Translator) -> Scalar  {
    return Scalar(self_.g0.y * other.g0.y + self_.g0.z * other.g0.z + self_.g0.w * other.g0.w);
}

fn rotor_translator_outer_product(self_: Rotor, other: Translator) -> Motor  {
    return Motor(self_.g0 * vec4<f32>(other.g0.x), vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + self_.g0.yxxx * other.g0.yyzw);
}

fn rotor_translator_left_contraction(self_: Rotor, other: Translator) -> Translator  {
    return Translator(vec4<f32>(self_.g0.x) * other.g0);
}

fn rotor_translator_right_contraction(self_: Rotor, other: Translator) -> Rotor  {
    return Rotor(self_.g0 * vec4<f32>(other.g0.x));
}

fn rotor_translator_scalar_product(self_: Rotor, other: Translator) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.x);
}

fn rotor_motor_add(self_: Rotor, other: Motor) -> Motor  {
    return Motor(self_.g0 + other.g0, other.g1);
}

fn rotor_motor_sub(self_: Rotor, other: Motor) -> Motor  {
    return Motor(self_.g0 - other.g0, vec4<f32>(0.0) - other.g1);
}

fn rotor_motor_geometric_product(self_: Rotor, other: Motor) -> Motor  {
    return Motor(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.y) * other.g0.yxwz * vec4<f32>(-1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g0.zwxy * vec4<f32>(-1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g0.w) * other.g0.wzyx * vec4<f32>(-1.0, 1.0, -1.0, 1.0), vec4<f32>(self_.g0.x) * other.g1 + vec4<f32>(self_.g0.y) * other.g1.yxwz * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g1.zwxy * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g0.w) * other.g1.wzyx * vec4<f32>(1.0, 1.0, -1.0, -1.0));
}

fn rotor_motor_regressive_product(self_: Rotor, other: Motor) -> Rotor  {
    return Rotor(vec4<f32>(self_.g0.y) * other.g1.yxyy * vec4<f32>(1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g0.z) * other.g1.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g1.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g1.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0));
}

fn rotor_motor_outer_product(self_: Rotor, other: Motor) -> Motor  {
    return Motor(vec4<f32>(self_.g0.x) * other.g0 + self_.g0 * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 1.0, 1.0), vec4<f32>(self_.g0.x) * other.g1 + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g1.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + self_.g0.yxxx * other.g1.yxxx * vec4<f32>(1.0, 0.0, 0.0, 0.0));
}

fn rotor_motor_inner_product(self_: Rotor, other: Motor) -> Motor  {
    return Motor(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.z) * other.g0.zzxz * vec4<f32>(-1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g0.wwwx * vec4<f32>(-1.0, 0.0, 0.0, 1.0) + self_.g0.yyxx * other.g0.yxxx * vec4<f32>(-1.0, 1.0, 0.0, 0.0), vec4<f32>(self_.g0.x) * other.g1 + self_.g0 * vec4<f32>(other.g1.x) * vec4<f32>(0.0, -1.0, -1.0, -1.0));
}

fn rotor_motor_left_contraction(self_: Rotor, other: Motor) -> Motor  {
    return Motor(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + self_.g0.yxxx * other.g0.yxxx * vec4<f32>(-1.0, 0.0, 0.0, 0.0), vec4<f32>(self_.g0.x) * other.g1 + self_.g0 * vec4<f32>(other.g1.x) * vec4<f32>(0.0, -1.0, -1.0, -1.0));
}

fn rotor_motor_right_contraction(self_: Rotor, other: Motor) -> Rotor  {
    return Rotor(vec4<f32>(self_.g0.y) * other.g0.yxyy * vec4<f32>(-1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g0.z) * other.g0.zzxz * vec4<f32>(-1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g0.wwwx * vec4<f32>(-1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0));
}

fn rotor_motor_scalar_product(self_: Rotor, other: Motor) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.x - self_.g0.y * other.g0.y - self_.g0.z * other.g0.z - self_.g0.w * other.g0.w);
}

fn rotor_point_and_plane_geometric_product(self_: Rotor, other: PointAndPlane) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.y) * vec4<f32>(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4<f32>(1.0, 1.0, -1.0, -1.0), vec4<f32>(self_.g0.x) * other.g1 + vec4<f32>(self_.g0.y) * vec4<f32>(other.g0.y, other.g0.x, other.g1.w, other.g1.z) * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z, other.g1.w, other.g0.x, other.g1.y) * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4<f32>(1.0, 1.0, -1.0, -1.0));
}

fn rotor_point_and_plane_outer_product(self_: Rotor, other: PointAndPlane) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.z) * other.g1.zzxz * vec4<f32>(1.0, 0.0, -1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g1.wwwx * vec4<f32>(1.0, 0.0, 0.0, -1.0) + self_.g0.yyxx * other.g1.yxxx * vec4<f32>(1.0, -1.0, 0.0, 0.0), vec4<f32>(self_.g0.x) * other.g1);
}

fn rotor_point_and_plane_inner_product(self_: Rotor, other: PointAndPlane) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0.x) * other.g0, vec4<f32>(self_.g0.x) * other.g1 + vec4<f32>(self_.g0.y) * vec4<f32>(other.g0.y, other.g0.x, other.g1.w, other.g1.z) * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z, other.g1.w, other.g0.x, other.g1.y) * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4<f32>(1.0, 1.0, -1.0, -1.0));
}

fn rotor_point_and_plane_left_contraction(self_: Rotor, other: PointAndPlane) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0.x) * other.g0, vec4<f32>(self_.g0.x) * other.g1 + vec4<f32>(self_.g0.z) * other.g0.zzxz * vec4<f32>(1.0, 0.0, -1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, -1.0) + self_.g0.yyxx * other.g0.yxxx * vec4<f32>(1.0, -1.0, 0.0, 0.0));
}

fn rotor_squared_magnitude(self_: Rotor) -> Scalar  {
    return rotor_rotor_scalar_product(self_, rotor_reversal(self_));
}

fn rotor_magnitude(self_: Rotor) -> Scalar  {
    return Scalar(sqrt(rotor_squared_magnitude(self_).g0));
}

fn rotor_scale(self_: Rotor, other: f32) -> Rotor  {
    return rotor_scalar_geometric_product(self_, Scalar(other));
}

fn rotor_signum(self_: Rotor) -> Rotor  {
    return rotor_scalar_geometric_product(self_, Scalar(1.0 / rotor_magnitude(self_).g0));
}

fn rotor_inverse(self_: Rotor) -> Rotor  {
    return rotor_scalar_geometric_product(rotor_reversal(self_), Scalar(1.0 / rotor_squared_magnitude(self_).g0));
}

fn point_zero() -> Point  {
    return Point(vec4<f32>(0.0));
}

fn point_one() -> Point  {
    return Point(vec4<f32>(0.0));
}

fn point_neg(self_: Point) -> Point  {
    return Point(self_.g0 * vec4<f32>(-1.0));
}

fn point_automorphism(self_: Point) -> Point  {
    return Point(self_.g0 * vec4<f32>(-1.0));
}

fn point_reversal(self_: Point) -> Point  {
    return Point(self_.g0 * vec4<f32>(-1.0));
}

fn point_conjugation(self_: Point) -> Point  {
    return Point(self_.g0);
}

fn point_dual(self_: Point) -> Plane  {
    return Plane(self_.g0 * vec4<f32>(-1.0));
}

fn point_scalar_geometric_product(self_: Point, other: Scalar) -> Point  {
    return Point(self_.g0 * vec4<f32>(other.g0));
}

fn point_scalar_outer_product(self_: Point, other: Scalar) -> Point  {
    return Point(self_.g0 * vec4<f32>(other.g0));
}

fn point_scalar_inner_product(self_: Point, other: Scalar) -> Point  {
    return Point(self_.g0 * vec4<f32>(other.g0));
}

fn point_scalar_right_contraction(self_: Point, other: Scalar) -> Point  {
    return Point(self_.g0 * vec4<f32>(other.g0));
}

fn point_multi_vector_add(self_: Point, other: MultiVector) -> MultiVector  {
    return MultiVector(other.g0, self_.g0 * vec4<f32>(0.0, 1.0, 1.0, 1.0) + other.g1, vec4<f32>(self_.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + other.g2, other.g3);
}

fn point_multi_vector_sub(self_: Point, other: MultiVector) -> MultiVector  {
    return MultiVector(vec4<f32>(0.0) - other.g0, self_.g0 * vec4<f32>(0.0, 1.0, 1.0, 1.0) - other.g1, vec4<f32>(self_.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0) - other.g2, vec4<f32>(0.0) - other.g3);
}

fn point_multi_vector_geometric_product(self_: Point, other: MultiVector) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0.x) * other.g2 * vec4<f32>(-1.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g0.y) * other.g1.yxwz * vec4<f32>(1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g0.z) * other.g1.zwxy * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.w) * other.g1.wzyx * vec4<f32>(1.0, -1.0, 1.0, 1.0), vec4<f32>(self_.g0.x) * other.g3 + vec4<f32>(self_.g0.y) * other.g0.yxwz * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g0.zwxy * vec4<f32>(1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g0.w) * other.g0.wzyx * vec4<f32>(1.0, 1.0, -1.0, 1.0), vec4<f32>(self_.g0.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g0.y) * other.g3.yxwz * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g3.zwxy * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g0.w) * other.g3.wzyx * vec4<f32>(1.0, 1.0, -1.0, -1.0), vec4<f32>(0.0) - vec4<f32>(self_.g0.x) * other.g1 + vec4<f32>(self_.g0.y) * other.g2.yxwz * vec4<f32>(-1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g0.z) * other.g2.zwxy * vec4<f32>(-1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.w) * other.g2.wzyx * vec4<f32>(-1.0, -1.0, 1.0, 1.0));
}

fn point_multi_vector_scalar_product(self_: Point, other: MultiVector) -> Scalar  {
    return Scalar(0.0 - self_.g0.x * other.g2.x + self_.g0.y * other.g1.y + self_.g0.z * other.g1.z + self_.g0.w * other.g1.w);
}

fn point_rotor_geometric_product(self_: Point, other: Rotor) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0.z) * other.g0.wwxy * vec4<f32>(0.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g0.w) * other.g0.zzyx * vec4<f32>(0.0, 1.0, -1.0, 1.0) + self_.g0.xyyy * other.g0.xxwz * vec4<f32>(1.0, 1.0, 1.0, -1.0), vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + self_.g0.yxxx * other.g0.yyzw * vec4<f32>(1.0, -1.0, -1.0, -1.0));
}

fn point_rotor_outer_product(self_: Point, other: Rotor) -> Point  {
    return Point(self_.g0 * vec4<f32>(other.g0.x));
}

fn point_rotor_inner_product(self_: Point, other: Rotor) -> PointAndPlane  {
    return PointAndPlane(self_.g0 * vec4<f32>(other.g0.x), vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + self_.g0.yxxx * other.g0.yyzw * vec4<f32>(1.0, -1.0, -1.0, -1.0));
}

fn point_rotor_right_contraction(self_: Point, other: Rotor) -> PointAndPlane  {
    return PointAndPlane(self_.g0 * vec4<f32>(other.g0.x), vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + self_.g0.yxxx * other.g0.yyzw * vec4<f32>(1.0, -1.0, -1.0, -1.0));
}

fn point_point_add(self_: Point, other: Point) -> Point  {
    return Point(self_.g0 + other.g0);
}

fn point_point_sub(self_: Point, other: Point) -> Point  {
    return Point(self_.g0 - other.g0);
}

fn point_point_mul(self_: Point, other: Point) -> Point  {
    return Point(self_.g0 * other.g0);
}

fn point_point_div(self_: Point, other: Point) -> Point  {
    return Point(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0) / vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0));
}

fn point_point_regressive_product(self_: Point, other: Point) -> Line  {
    return Line(vec3<f32>(self_.g0.z) * vec3<f32>(other.g0.w, other.g0.w, other.g0.y) * vec3<f32>(1.0, 0.0, -1.0) + vec3<f32>(self_.g0.w) * vec3<f32>(other.g0.z, other.g0.y, other.g0.z) * vec3<f32>(-1.0, 1.0, 0.0) + vec3<f32>(self_.g0.x, self_.g0.y, self_.g0.y) * vec3<f32>(other.g0.x, other.g0.w, other.g0.z) * vec3<f32>(0.0, -1.0, 1.0), vec3<f32>(self_.g0.x) * vec3<f32>(other.g0.y, other.g0.z, other.g0.w) + vec3<f32>(self_.g0.y, self_.g0.z, self_.g0.w) * vec3<f32>(other.g0.x) * vec3<f32>(-1.0));
}

fn point_point_inner_product(self_: Point, other: Point) -> Scalar  {
    return Scalar(0.0 - self_.g0.x * other.g0.x + self_.g0.y * other.g0.y + self_.g0.z * other.g0.z + self_.g0.w * other.g0.w);
}

fn point_point_left_contraction(self_: Point, other: Point) -> Scalar  {
    return Scalar(0.0 - self_.g0.x * other.g0.x + self_.g0.y * other.g0.y + self_.g0.z * other.g0.z + self_.g0.w * other.g0.w);
}

fn point_point_right_contraction(self_: Point, other: Point) -> Scalar  {
    return Scalar(0.0 - self_.g0.x * other.g0.x + self_.g0.y * other.g0.y + self_.g0.z * other.g0.z + self_.g0.w * other.g0.w);
}

fn point_point_scalar_product(self_: Point, other: Point) -> Scalar  {
    return Scalar(0.0 - self_.g0.x * other.g0.x + self_.g0.y * other.g0.y + self_.g0.z * other.g0.z + self_.g0.w * other.g0.w);
}

fn point_ideal_point_regressive_product(self_: Point, other: IdealPoint) -> Plane  {
    return Plane(vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.y) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.z) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + self_.g0.yxxx * vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(-1.0, 1.0, 1.0, 1.0));
}

fn point_plane_add(self_: Point, other: Plane) -> PointAndPlane  {
    return PointAndPlane(self_.g0, other.g0);
}

fn point_plane_sub(self_: Point, other: Plane) -> PointAndPlane  {
    return PointAndPlane(self_.g0, vec4<f32>(0.0) - other.g0);
}

fn point_plane_regressive_product(self_: Point, other: Plane) -> Scalar  {
    return Scalar(0.0 - self_.g0.x * other.g0.x - self_.g0.y * other.g0.y - self_.g0.z * other.g0.z - self_.g0.w * other.g0.w);
}

fn point_plane_inner_product(self_: Point, other: Plane) -> Line  {
    return Line(vec3<f32>(self_.g0.z) * vec3<f32>(other.g0.w, other.g0.w, other.g0.y) * vec3<f32>(1.0, 0.0, -1.0) + vec3<f32>(self_.g0.w) * vec3<f32>(other.g0.z, other.g0.y, other.g0.z) * vec3<f32>(-1.0, 1.0, 0.0) + vec3<f32>(self_.g0.x, self_.g0.y, self_.g0.y) * vec3<f32>(other.g0.x, other.g0.w, other.g0.z) * vec3<f32>(0.0, -1.0, 1.0), vec3<f32>(self_.g0.x) * vec3<f32>(other.g0.y, other.g0.z, other.g0.w) + vec3<f32>(self_.g0.y, self_.g0.z, self_.g0.w) * vec3<f32>(other.g0.x));
}

fn point_plane_right_contraction(self_: Point, other: Plane) -> Line  {
    return Line(vec3<f32>(self_.g0.z) * vec3<f32>(other.g0.w, other.g0.w, other.g0.y) * vec3<f32>(1.0, 0.0, -1.0) + vec3<f32>(self_.g0.w) * vec3<f32>(other.g0.z, other.g0.y, other.g0.z) * vec3<f32>(-1.0, 1.0, 0.0) + vec3<f32>(self_.g0.x, self_.g0.y, self_.g0.y) * vec3<f32>(other.g0.x, other.g0.w, other.g0.z) * vec3<f32>(0.0, -1.0, 1.0), vec3<f32>(self_.g0.x) * vec3<f32>(other.g0.y, other.g0.z, other.g0.w) + vec3<f32>(self_.g0.y, self_.g0.z, self_.g0.w) * vec3<f32>(other.g0.x));
}

fn point_line_geometric_product(self_: Point, other: Line) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0.y) * vec4<f32>(other.g0.x, other.g0.x, other.g1.z, other.g1.y) * vec4<f32>(1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.y, other.g1.z, other.g0.y, other.g1.x) * vec4<f32>(1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.z, other.g1.y, other.g1.x, other.g0.z) * vec4<f32>(1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0), vec4<f32>(self_.g0.y) * vec4<f32>(other.g1.x, other.g1.x, other.g0.z, other.g0.y) * vec4<f32>(1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.y, other.g0.z, other.g1.y, other.g0.x) * vec4<f32>(1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g1.z, other.g0.y, other.g0.x, other.g1.z) * vec4<f32>(1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4<f32>(0.0, -1.0, -1.0, -1.0));
}

fn point_line_regressive_product(self_: Point, other: Line) -> Plane  {
    return Plane(vec4<f32>(self_.g0.y) * vec4<f32>(other.g0.x, other.g0.x, other.g1.z, other.g1.y) * vec4<f32>(-1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.y, other.g1.z, other.g0.y, other.g1.x) * vec4<f32>(-1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.z, other.g1.y, other.g1.x, other.g0.z) * vec4<f32>(-1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0));
}

fn point_line_inner_product(self_: Point, other: Line) -> Plane  {
    return Plane(vec4<f32>(self_.g0.y) * vec4<f32>(other.g1.x, other.g1.x, other.g0.z, other.g0.y) * vec4<f32>(1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.y, other.g0.z, other.g1.y, other.g0.x) * vec4<f32>(1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g1.z, other.g0.y, other.g0.x, other.g1.z) * vec4<f32>(1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4<f32>(0.0, -1.0, -1.0, -1.0));
}

fn point_line_right_contraction(self_: Point, other: Line) -> Plane  {
    return Plane(vec4<f32>(self_.g0.y) * vec4<f32>(other.g1.x, other.g1.x, other.g0.z, other.g0.y) * vec4<f32>(1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.y, other.g0.z, other.g1.y, other.g0.x) * vec4<f32>(1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g1.z, other.g0.y, other.g0.x, other.g1.z) * vec4<f32>(1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4<f32>(0.0, -1.0, -1.0, -1.0));
}

fn point_translator_regressive_product(self_: Point, other: Translator) -> Plane  {
    return Plane(vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + self_.g0.yxxx * other.g0.yyzw * vec4<f32>(-1.0, 1.0, 1.0, 1.0));
}

fn point_translator_outer_product(self_: Point, other: Translator) -> Point  {
    return Point(self_.g0 * vec4<f32>(other.g0.x));
}

fn point_motor_geometric_product(self_: Point, other: Motor) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0.x) * vec4<f32>(other.g0.x, other.g1.y, other.g1.z, other.g1.w) + vec4<f32>(self_.g0.y) * vec4<f32>(other.g1.y, other.g0.x, other.g0.w, other.g0.z) * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.z, other.g0.w, other.g0.x, other.g0.y) * vec4<f32>(1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g1.w, other.g0.z, other.g0.y, other.g0.x) * vec4<f32>(1.0, 1.0, -1.0, 1.0), vec4<f32>(self_.g0.x) * vec4<f32>(other.g1.x, other.g0.y, other.g0.z, other.g0.w) * vec4<f32>(1.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g0.y) * vec4<f32>(other.g0.y, other.g1.x, other.g1.w, other.g1.z) * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z, other.g1.w, other.g1.x, other.g1.y) * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w, other.g1.z, other.g1.y, other.g1.x) * vec4<f32>(1.0, 1.0, -1.0, -1.0));
}

fn point_motor_regressive_product(self_: Point, other: Motor) -> PointAndPlane  {
    return PointAndPlane(self_.g0 * vec4<f32>(other.g1.x), vec4<f32>(self_.g0.y) * vec4<f32>(other.g1.y, other.g1.y, other.g0.w, other.g0.z) * vec4<f32>(-1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.z, other.g0.w, other.g1.z, other.g0.y) * vec4<f32>(-1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g1.w, other.g0.z, other.g0.y, other.g1.w) * vec4<f32>(-1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g0.x) * other.g1 * vec4<f32>(0.0, 1.0, 1.0, 1.0));
}

fn point_motor_outer_product(self_: Point, other: Motor) -> Point  {
    return Point(self_.g0 * vec4<f32>(other.g0.x));
}

fn point_motor_inner_product(self_: Point, other: Motor) -> PointAndPlane  {
    return PointAndPlane(self_.g0 * vec4<f32>(other.g0.x), vec4<f32>(self_.g0.x) * vec4<f32>(other.g1.x, other.g0.y, other.g0.z, other.g0.w) * vec4<f32>(1.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g0.y) * vec4<f32>(other.g0.y, other.g1.x, other.g1.w, other.g1.z) * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z, other.g1.w, other.g1.x, other.g1.y) * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w, other.g1.z, other.g1.y, other.g1.x) * vec4<f32>(1.0, 1.0, -1.0, -1.0));
}

fn point_motor_left_contraction(self_: Point, other: Motor) -> Plane  {
    return Plane(self_.g0 * vec4<f32>(other.g1.x) * vec4<f32>(1.0, -1.0, -1.0, -1.0));
}

fn point_motor_right_contraction(self_: Point, other: Motor) -> PointAndPlane  {
    return PointAndPlane(self_.g0 * vec4<f32>(other.g0.x), vec4<f32>(self_.g0.y) * vec4<f32>(other.g0.y, other.g0.y, other.g1.w, other.g1.z) * vec4<f32>(1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z, other.g1.w, other.g0.z, other.g1.y) * vec4<f32>(1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w, other.g1.z, other.g1.y, other.g0.w) * vec4<f32>(1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g0.x) * other.g0 * vec4<f32>(0.0, -1.0, -1.0, -1.0));
}

fn point_point_and_plane_add(self_: Point, other: PointAndPlane) -> PointAndPlane  {
    return PointAndPlane(self_.g0 + other.g0, other.g1);
}

fn point_point_and_plane_sub(self_: Point, other: PointAndPlane) -> PointAndPlane  {
    return PointAndPlane(self_.g0 - other.g0, vec4<f32>(0.0) - other.g1);
}

fn point_point_and_plane_geometric_product(self_: Point, other: PointAndPlane) -> Motor  {
    return Motor(vec4<f32>(self_.g0.x) * vec4<f32>(other.g0.x, other.g1.y, other.g1.z, other.g1.w) * vec4<f32>(-1.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g0.y) * vec4<f32>(other.g0.y, other.g1.x, other.g0.w, other.g0.z) * vec4<f32>(1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z, other.g0.w, other.g1.x, other.g0.y) * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w, other.g0.z, other.g0.y, other.g1.x) * vec4<f32>(1.0, -1.0, 1.0, 1.0), vec4<f32>(0.0) - vec4<f32>(self_.g0.x) * vec4<f32>(other.g1.x, other.g0.y, other.g0.z, other.g0.w) + vec4<f32>(self_.g0.y) * vec4<f32>(other.g1.y, other.g0.x, other.g1.w, other.g1.z) * vec4<f32>(-1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.z, other.g1.w, other.g0.x, other.g1.y) * vec4<f32>(-1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g1.w, other.g1.z, other.g1.y, other.g0.x) * vec4<f32>(-1.0, -1.0, 1.0, 1.0));
}

fn point_point_and_plane_left_contraction(self_: Point, other: PointAndPlane) -> Scalar  {
    return Scalar(0.0 - self_.g0.x * other.g0.x + self_.g0.y * other.g0.y + self_.g0.z * other.g0.z + self_.g0.w * other.g0.w);
}

fn point_point_and_plane_scalar_product(self_: Point, other: PointAndPlane) -> Scalar  {
    return Scalar(0.0 - self_.g0.x * other.g0.x + self_.g0.y * other.g0.y + self_.g0.z * other.g0.z + self_.g0.w * other.g0.w);
}

fn point_squared_magnitude(self_: Point) -> Scalar  {
    return point_point_scalar_product(self_, point_reversal(self_));
}

fn point_magnitude(self_: Point) -> Scalar  {
    return Scalar(sqrt(point_squared_magnitude(self_).g0));
}

fn point_scale(self_: Point, other: f32) -> Point  {
    return point_scalar_geometric_product(self_, Scalar(other));
}

fn point_signum(self_: Point) -> Point  {
    return point_scalar_geometric_product(self_, Scalar(1.0 / point_magnitude(self_).g0));
}

fn point_inverse(self_: Point) -> Point  {
    return point_scalar_geometric_product(point_reversal(self_), Scalar(1.0 / point_squared_magnitude(self_).g0));
}

fn ideal_point_zero() -> IdealPoint  {
    return IdealPoint(vec3<f32>(0.0));
}

fn ideal_point_one() -> IdealPoint  {
    return IdealPoint(vec3<f32>(0.0));
}

fn ideal_point_neg(self_: IdealPoint) -> IdealPoint  {
    return IdealPoint(self_.g0 * vec3<f32>(-1.0));
}

fn ideal_point_automorphism(self_: IdealPoint) -> IdealPoint  {
    return IdealPoint(self_.g0);
}

fn ideal_point_reversal(self_: IdealPoint) -> IdealPoint  {
    return IdealPoint(self_.g0 * vec3<f32>(-1.0));
}

fn ideal_point_conjugation(self_: IdealPoint) -> IdealPoint  {
    return IdealPoint(self_.g0 * vec3<f32>(-1.0));
}

fn ideal_point_scalar_add(self_: IdealPoint, other: Scalar) -> Translator  {
    return Translator(vec4<f32>(self_.g0.x, self_.g0.x, self_.g0.y, self_.g0.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0) + vec4<f32>(other.g0) * vec4<f32>(1.0, 0.0, 0.0, 0.0));
}

fn ideal_point_scalar_sub(self_: IdealPoint, other: Scalar) -> Translator  {
    return Translator(vec4<f32>(self_.g0.x, self_.g0.x, self_.g0.y, self_.g0.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0) - vec4<f32>(other.g0) * vec4<f32>(1.0, 0.0, 0.0, 0.0));
}

fn ideal_point_scalar_geometric_product(self_: IdealPoint, other: Scalar) -> IdealPoint  {
    return IdealPoint(self_.g0 * vec3<f32>(other.g0));
}

fn ideal_point_scalar_outer_product(self_: IdealPoint, other: Scalar) -> IdealPoint  {
    return IdealPoint(self_.g0 * vec3<f32>(other.g0));
}

fn ideal_point_scalar_inner_product(self_: IdealPoint, other: Scalar) -> IdealPoint  {
    return IdealPoint(self_.g0 * vec3<f32>(other.g0));
}

fn ideal_point_scalar_right_contraction(self_: IdealPoint, other: Scalar) -> IdealPoint  {
    return IdealPoint(self_.g0 * vec3<f32>(other.g0));
}

fn ideal_point_multi_vector_add(self_: IdealPoint, other: MultiVector) -> MultiVector  {
    return MultiVector(other.g0, other.g1, other.g2, vec4<f32>(self_.g0.x, self_.g0.x, self_.g0.y, self_.g0.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0) + other.g3);
}

fn ideal_point_multi_vector_sub(self_: IdealPoint, other: MultiVector) -> MultiVector  {
    return MultiVector(vec4<f32>(0.0) - other.g0, vec4<f32>(0.0) - other.g1, vec4<f32>(0.0) - other.g2, vec4<f32>(self_.g0.x, self_.g0.x, self_.g0.y, self_.g0.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0) - other.g3);
}

fn ideal_point_multi_vector_geometric_product(self_: IdealPoint, other: MultiVector) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0.x) * other.g3.yxwz * vec4<f32>(1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g0.y) * other.g3.zwxy * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g3.wzyx * vec4<f32>(1.0, -1.0, 1.0, 1.0), vec4<f32>(self_.g0.x) * other.g2.yxwz * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g0.y) * other.g2.zwxy * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g0.z) * other.g2.wzyx * vec4<f32>(1.0, 1.0, -1.0, -1.0), vec4<f32>(self_.g0.x) * other.g1.yxwz * vec4<f32>(-1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g0.y) * other.g1.zwxy * vec4<f32>(-1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g1.wzyx * vec4<f32>(-1.0, -1.0, 1.0, 1.0), vec4<f32>(self_.g0.x) * other.g0.yxwz * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.y) * other.g0.zwxy * vec4<f32>(1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g0.z) * other.g0.wzyx * vec4<f32>(1.0, 1.0, -1.0, 1.0));
}

fn ideal_point_multi_vector_scalar_product(self_: IdealPoint, other: MultiVector) -> Scalar  {
    return Scalar(self_.g0.x * other.g3.y + self_.g0.y * other.g3.z + self_.g0.z * other.g3.w);
}

fn ideal_point_rotor_regressive_product(self_: IdealPoint, other: Rotor) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.y + self_.g0.y * other.g0.z + self_.g0.z * other.g0.w);
}

fn ideal_point_rotor_inner_product(self_: IdealPoint, other: Rotor) -> IdealPoint  {
    return IdealPoint(self_.g0 * vec3<f32>(other.g0.x));
}

fn ideal_point_rotor_right_contraction(self_: IdealPoint, other: Rotor) -> IdealPoint  {
    return IdealPoint(self_.g0 * vec3<f32>(other.g0.x));
}

fn ideal_point_point_regressive_product(self_: IdealPoint, other: Point) -> Plane  {
    return Plane(vec4<f32>(self_.g0.y) * other.g0.zzxz * vec4<f32>(-1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.z) * other.g0.wwwx * vec4<f32>(-1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g0.x) * other.g0.yxxx * vec4<f32>(-1.0, 1.0, 0.0, 0.0));
}

fn ideal_point_ideal_point_add(self_: IdealPoint, other: IdealPoint) -> IdealPoint  {
    return IdealPoint(self_.g0 + other.g0);
}

fn ideal_point_ideal_point_sub(self_: IdealPoint, other: IdealPoint) -> IdealPoint  {
    return IdealPoint(self_.g0 - other.g0);
}

fn ideal_point_ideal_point_mul(self_: IdealPoint, other: IdealPoint) -> IdealPoint  {
    return IdealPoint(self_.g0 * other.g0);
}

fn ideal_point_ideal_point_div(self_: IdealPoint, other: IdealPoint) -> IdealPoint  {
    return IdealPoint(vec3<f32>(self_.g0.x, self_.g0.y, self_.g0.z) * vec3<f32>(1.0, 1.0, 1.0) / vec3<f32>(other.g0.x, other.g0.y, other.g0.z) * vec3<f32>(1.0, 1.0, 1.0));
}

fn ideal_point_ideal_point_geometric_product(self_: IdealPoint, other: IdealPoint) -> Rotor  {
    return Rotor(vec4<f32>(self_.g0.y) * vec4<f32>(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4<f32>(1.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4<f32>(1.0, -1.0, 1.0, 0.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4<f32>(1.0, 0.0, -1.0, 1.0));
}

fn ideal_point_ideal_point_inner_product(self_: IdealPoint, other: IdealPoint) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.x + self_.g0.y * other.g0.y + self_.g0.z * other.g0.z);
}

fn ideal_point_ideal_point_left_contraction(self_: IdealPoint, other: IdealPoint) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.x + self_.g0.y * other.g0.y + self_.g0.z * other.g0.z);
}

fn ideal_point_ideal_point_right_contraction(self_: IdealPoint, other: IdealPoint) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.x + self_.g0.y * other.g0.y + self_.g0.z * other.g0.z);
}

fn ideal_point_ideal_point_scalar_product(self_: IdealPoint, other: IdealPoint) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.x + self_.g0.y * other.g0.y + self_.g0.z * other.g0.z);
}

fn ideal_point_plane_inner_product(self_: IdealPoint, other: Plane) -> Plane  {
    return Plane(vec4<f32>(self_.g0.y) * other.g0.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.z) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g0.x) * other.g0.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0));
}

fn ideal_point_plane_right_contraction(self_: IdealPoint, other: Plane) -> Plane  {
    return Plane(vec4<f32>(self_.g0.y) * other.g0.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.z) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g0.x) * other.g0.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0));
}

fn ideal_point_line_add(self_: IdealPoint, other: Line) -> Line  {
    return Line(self_.g0 + other.g0, other.g1);
}

fn ideal_point_line_sub(self_: IdealPoint, other: Line) -> Line  {
    return Line(self_.g0 - other.g0, vec3<f32>(0.0) - other.g1);
}

fn ideal_point_line_geometric_product(self_: IdealPoint, other: Line) -> Motor  {
    return Motor(vec4<f32>(self_.g0.y) * vec4<f32>(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4<f32>(1.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4<f32>(1.0, -1.0, 1.0, 0.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4<f32>(1.0, 0.0, -1.0, 1.0), vec4<f32>(self_.g0.y) * vec4<f32>(other.g1.y, other.g1.z, other.g1.y, other.g1.x) * vec4<f32>(1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.z, other.g1.y, other.g1.x, other.g1.z) * vec4<f32>(1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g1.x, other.g1.x, other.g1.z, other.g1.y) * vec4<f32>(1.0, 0.0, 1.0, -1.0));
}

fn ideal_point_line_regressive_product(self_: IdealPoint, other: Line) -> Scalar  {
    return Scalar(self_.g0.x * other.g1.x + self_.g0.y * other.g1.y + self_.g0.z * other.g1.z);
}

fn ideal_point_line_inner_product(self_: IdealPoint, other: Line) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.x + self_.g0.y * other.g0.y + self_.g0.z * other.g0.z);
}

fn ideal_point_line_left_contraction(self_: IdealPoint, other: Line) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.x + self_.g0.y * other.g0.y + self_.g0.z * other.g0.z);
}

fn ideal_point_line_right_contraction(self_: IdealPoint, other: Line) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.x + self_.g0.y * other.g0.y + self_.g0.z * other.g0.z);
}

fn ideal_point_line_scalar_product(self_: IdealPoint, other: Line) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.x + self_.g0.y * other.g0.y + self_.g0.z * other.g0.z);
}

fn ideal_point_translator_add(self_: IdealPoint, other: Translator) -> Translator  {
    return Translator(vec4<f32>(self_.g0.x, self_.g0.x, self_.g0.y, self_.g0.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0) + other.g0);
}

fn ideal_point_translator_sub(self_: IdealPoint, other: Translator) -> Translator  {
    return Translator(vec4<f32>(self_.g0.x, self_.g0.x, self_.g0.y, self_.g0.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0) - other.g0);
}

fn ideal_point_translator_outer_product(self_: IdealPoint, other: Translator) -> IdealPoint  {
    return IdealPoint(self_.g0 * vec3<f32>(other.g0.x));
}

fn ideal_point_translator_inner_product(self_: IdealPoint, other: Translator) -> Translator  {
    return Translator(vec4<f32>(self_.g0.y) * other.g0.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.z) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g0.x) * other.g0.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0));
}

fn ideal_point_translator_left_contraction(self_: IdealPoint, other: Translator) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.y + self_.g0.y * other.g0.z + self_.g0.z * other.g0.w);
}

fn ideal_point_translator_right_contraction(self_: IdealPoint, other: Translator) -> Translator  {
    return Translator(vec4<f32>(self_.g0.y) * other.g0.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.z) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g0.x) * other.g0.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0));
}

fn ideal_point_translator_scalar_product(self_: IdealPoint, other: Translator) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.y + self_.g0.y * other.g0.z + self_.g0.z * other.g0.w);
}

fn ideal_point_motor_add(self_: IdealPoint, other: Motor) -> Motor  {
    return Motor(other.g0, vec4<f32>(self_.g0.x, self_.g0.x, self_.g0.y, self_.g0.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0) + other.g1);
}

fn ideal_point_motor_sub(self_: IdealPoint, other: Motor) -> Motor  {
    return Motor(vec4<f32>(0.0) - other.g0, vec4<f32>(self_.g0.x, self_.g0.x, self_.g0.y, self_.g0.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0) - other.g1);
}

fn ideal_point_motor_geometric_product(self_: IdealPoint, other: Motor) -> Motor  {
    return Motor(vec4<f32>(self_.g0.x) * other.g1.yxwz * vec4<f32>(1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g0.y) * other.g1.zwxy * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g1.wzyx * vec4<f32>(1.0, -1.0, 1.0, 1.0), vec4<f32>(self_.g0.x) * other.g0.yxwz * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.y) * other.g0.zwxy * vec4<f32>(1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g0.z) * other.g0.wzyx * vec4<f32>(1.0, 1.0, -1.0, 1.0));
}

fn ideal_point_motor_regressive_product(self_: IdealPoint, other: Motor) -> Translator  {
    return Translator(vec4<f32>(self_.g0.y) * vec4<f32>(other.g0.z, other.g0.z, other.g1.x, other.g0.z) * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.w, other.g0.w, other.g0.w, other.g1.x) * vec4<f32>(1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g0.y, other.g1.x, other.g0.x, other.g0.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0));
}

fn ideal_point_motor_left_contraction(self_: IdealPoint, other: Motor) -> Rotor  {
    return Rotor(vec4<f32>(self_.g0.y) * other.g1.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.z) * other.g1.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g0.x) * other.g1.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0));
}

fn ideal_point_motor_right_contraction(self_: IdealPoint, other: Motor) -> Translator  {
    return Translator(vec4<f32>(self_.g0.y) * vec4<f32>(other.g1.z, other.g1.z, other.g0.x, other.g1.z) * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.w, other.g1.w, other.g1.w, other.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g1.y, other.g0.x, other.g1.x, other.g1.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0));
}

fn ideal_point_motor_scalar_product(self_: IdealPoint, other: Motor) -> Scalar  {
    return Scalar(self_.g0.x * other.g1.y + self_.g0.y * other.g1.z + self_.g0.z * other.g1.w);
}

fn ideal_point_point_and_plane_geometric_product(self_: IdealPoint, other: PointAndPlane) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0.x) * vec4<f32>(other.g0.y, other.g0.x, other.g1.w, other.g1.z) * vec4<f32>(-1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g0.y) * vec4<f32>(other.g0.z, other.g1.w, other.g0.x, other.g1.y) * vec4<f32>(-1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4<f32>(-1.0, 1.0, -1.0, -1.0), vec4<f32>(self_.g0.x) * vec4<f32>(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4<f32>(1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g0.y) * vec4<f32>(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4<f32>(1.0, -1.0, 1.0, 1.0));
}

fn ideal_point_point_and_plane_regressive_product(self_: IdealPoint, other: PointAndPlane) -> Plane  {
    return Plane(vec4<f32>(self_.g0.y) * other.g0.zzxz * vec4<f32>(-1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.z) * other.g0.wwwx * vec4<f32>(-1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g0.x) * other.g0.yxxx * vec4<f32>(-1.0, 1.0, 0.0, 0.0));
}

fn ideal_point_point_and_plane_inner_product(self_: IdealPoint, other: PointAndPlane) -> Plane  {
    return Plane(vec4<f32>(self_.g0.x) * vec4<f32>(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4<f32>(1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g0.y) * vec4<f32>(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4<f32>(1.0, -1.0, 1.0, 1.0));
}

fn ideal_point_point_and_plane_right_contraction(self_: IdealPoint, other: PointAndPlane) -> Plane  {
    return Plane(vec4<f32>(self_.g0.y) * other.g1.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.z) * other.g1.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g0.x) * other.g1.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0));
}

fn ideal_point_squared_magnitude(self_: IdealPoint) -> Scalar  {
    return ideal_point_ideal_point_scalar_product(self_, ideal_point_reversal(self_));
}

fn ideal_point_magnitude(self_: IdealPoint) -> Scalar  {
    return Scalar(sqrt(ideal_point_squared_magnitude(self_).g0));
}

fn ideal_point_scale(self_: IdealPoint, other: f32) -> IdealPoint  {
    return ideal_point_scalar_geometric_product(self_, Scalar(other));
}

fn ideal_point_signum(self_: IdealPoint) -> IdealPoint  {
    return ideal_point_scalar_geometric_product(self_, Scalar(1.0 / ideal_point_magnitude(self_).g0));
}

fn ideal_point_inverse(self_: IdealPoint) -> IdealPoint  {
    return ideal_point_scalar_geometric_product(ideal_point_reversal(self_), Scalar(1.0 / ideal_point_squared_magnitude(self_).g0));
}

fn plane_zero() -> Plane  {
    return Plane(vec4<f32>(0.0));
}

fn plane_one() -> Plane  {
    return Plane(vec4<f32>(0.0));
}

fn plane_neg(self_: Plane) -> Plane  {
    return Plane(self_.g0 * vec4<f32>(-1.0));
}

fn plane_automorphism(self_: Plane) -> Plane  {
    return Plane(self_.g0 * vec4<f32>(-1.0));
}

fn plane_reversal(self_: Plane) -> Plane  {
    return Plane(self_.g0);
}

fn plane_conjugation(self_: Plane) -> Plane  {
    return Plane(self_.g0 * vec4<f32>(-1.0));
}

fn plane_dual(self_: Plane) -> Point  {
    return Point(self_.g0);
}

fn plane_scalar_geometric_product(self_: Plane, other: Scalar) -> Plane  {
    return Plane(self_.g0 * vec4<f32>(other.g0));
}

fn plane_scalar_outer_product(self_: Plane, other: Scalar) -> Plane  {
    return Plane(self_.g0 * vec4<f32>(other.g0));
}

fn plane_scalar_inner_product(self_: Plane, other: Scalar) -> Plane  {
    return Plane(self_.g0 * vec4<f32>(other.g0));
}

fn plane_scalar_right_contraction(self_: Plane, other: Scalar) -> Plane  {
    return Plane(self_.g0 * vec4<f32>(other.g0));
}

fn plane_multi_vector_add(self_: Plane, other: MultiVector) -> MultiVector  {
    return MultiVector(other.g0, vec4<f32>(self_.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + other.g1, self_.g0 * vec4<f32>(0.0, 1.0, 1.0, 1.0) + other.g2, other.g3);
}

fn plane_multi_vector_sub(self_: Plane, other: MultiVector) -> MultiVector  {
    return MultiVector(vec4<f32>(0.0) - other.g0, vec4<f32>(self_.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0) - other.g1, self_.g0 * vec4<f32>(0.0, 1.0, 1.0, 1.0) - other.g2, vec4<f32>(0.0) - other.g3);
}

fn plane_multi_vector_geometric_product(self_: Plane, other: MultiVector) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0.x) * other.g1 * vec4<f32>(-1.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g0.y) * other.g2.yxwz * vec4<f32>(1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g0.z) * other.g2.zwxy * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.w) * other.g2.wzyx * vec4<f32>(1.0, -1.0, 1.0, 1.0), vec4<f32>(self_.g0.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g0.y) * other.g3.yxwz * vec4<f32>(-1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g0.z) * other.g3.zwxy * vec4<f32>(-1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.w) * other.g3.wzyx * vec4<f32>(-1.0, -1.0, 1.0, 1.0), vec4<f32>(0.0) - vec4<f32>(self_.g0.x) * other.g3 + vec4<f32>(self_.g0.y) * other.g0.yxwz * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g0.zwxy * vec4<f32>(1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g0.w) * other.g0.wzyx * vec4<f32>(1.0, 1.0, -1.0, 1.0), vec4<f32>(self_.g0.x) * other.g2 + vec4<f32>(self_.g0.y) * other.g1.yxwz * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g1.zwxy * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g0.w) * other.g1.wzyx * vec4<f32>(1.0, 1.0, -1.0, -1.0));
}

fn plane_multi_vector_scalar_product(self_: Plane, other: MultiVector) -> Scalar  {
    return Scalar(0.0 - self_.g0.x * other.g1.x + self_.g0.y * other.g2.y + self_.g0.z * other.g2.z + self_.g0.w * other.g2.w);
}

fn plane_rotor_geometric_product(self_: Plane, other: Rotor) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + self_.g0.yxxx * other.g0.yyzw * vec4<f32>(1.0, -1.0, -1.0, -1.0), vec4<f32>(self_.g0.z) * other.g0.wwxy * vec4<f32>(0.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g0.w) * other.g0.zzyx * vec4<f32>(0.0, 1.0, -1.0, 1.0) + self_.g0.xyyy * other.g0.xxwz * vec4<f32>(1.0, 1.0, 1.0, -1.0));
}

fn plane_rotor_outer_product(self_: Plane, other: Rotor) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + self_.g0.yxxx * other.g0.yyzw * vec4<f32>(1.0, -1.0, -1.0, -1.0), self_.g0 * vec4<f32>(other.g0.x));
}

fn plane_rotor_inner_product(self_: Plane, other: Rotor) -> Plane  {
    return Plane(vec4<f32>(self_.g0.z) * other.g0.wwxy * vec4<f32>(0.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g0.w) * other.g0.zzyx * vec4<f32>(0.0, 1.0, -1.0, 1.0) + self_.g0.xyyy * other.g0.xxwz * vec4<f32>(1.0, 1.0, 1.0, -1.0));
}

fn plane_rotor_right_contraction(self_: Plane, other: Rotor) -> Plane  {
    return Plane(self_.g0 * vec4<f32>(other.g0.x));
}

fn plane_point_add(self_: Plane, other: Point) -> PointAndPlane  {
    return PointAndPlane(other.g0, self_.g0);
}

fn plane_point_sub(self_: Plane, other: Point) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(0.0) - other.g0, self_.g0);
}

fn plane_point_regressive_product(self_: Plane, other: Point) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.x + self_.g0.y * other.g0.y + self_.g0.z * other.g0.z + self_.g0.w * other.g0.w);
}

fn plane_point_inner_product(self_: Plane, other: Point) -> Line  {
    return Line(vec3<f32>(self_.g0.z) * vec3<f32>(other.g0.w, other.g0.w, other.g0.y) * vec3<f32>(-1.0, 0.0, 1.0) + vec3<f32>(self_.g0.w) * vec3<f32>(other.g0.z, other.g0.y, other.g0.z) * vec3<f32>(1.0, -1.0, 0.0) + vec3<f32>(self_.g0.x, self_.g0.y, self_.g0.y) * vec3<f32>(other.g0.x, other.g0.w, other.g0.z) * vec3<f32>(0.0, 1.0, -1.0), vec3<f32>(self_.g0.x) * vec3<f32>(other.g0.y, other.g0.z, other.g0.w) + vec3<f32>(self_.g0.y, self_.g0.z, self_.g0.w) * vec3<f32>(other.g0.x));
}

fn plane_point_left_contraction(self_: Plane, other: Point) -> Line  {
    return Line(vec3<f32>(self_.g0.z) * vec3<f32>(other.g0.w, other.g0.w, other.g0.y) * vec3<f32>(-1.0, 0.0, 1.0) + vec3<f32>(self_.g0.w) * vec3<f32>(other.g0.z, other.g0.y, other.g0.z) * vec3<f32>(1.0, -1.0, 0.0) + vec3<f32>(self_.g0.x, self_.g0.y, self_.g0.y) * vec3<f32>(other.g0.x, other.g0.w, other.g0.z) * vec3<f32>(0.0, 1.0, -1.0), vec3<f32>(self_.g0.x) * vec3<f32>(other.g0.y, other.g0.z, other.g0.w) + vec3<f32>(self_.g0.y, self_.g0.z, self_.g0.w) * vec3<f32>(other.g0.x));
}

fn plane_ideal_point_inner_product(self_: Plane, other: IdealPoint) -> Plane  {
    return Plane(vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.y) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.z) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + self_.g0.yxxx * vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(-1.0));
}

fn plane_ideal_point_left_contraction(self_: Plane, other: IdealPoint) -> Plane  {
    return Plane(vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.y) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.z) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + self_.g0.yxxx * vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(-1.0));
}

fn plane_plane_add(self_: Plane, other: Plane) -> Plane  {
    return Plane(self_.g0 + other.g0);
}

fn plane_plane_sub(self_: Plane, other: Plane) -> Plane  {
    return Plane(self_.g0 - other.g0);
}

fn plane_plane_mul(self_: Plane, other: Plane) -> Plane  {
    return Plane(self_.g0 * other.g0);
}

fn plane_plane_div(self_: Plane, other: Plane) -> Plane  {
    return Plane(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0) / vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0));
}

fn plane_plane_outer_product(self_: Plane, other: Plane) -> Line  {
    return Line(vec3<f32>(self_.g0.x) * vec3<f32>(other.g0.y, other.g0.z, other.g0.w) + vec3<f32>(self_.g0.y, self_.g0.z, self_.g0.w) * vec3<f32>(other.g0.x) * vec3<f32>(-1.0), vec3<f32>(self_.g0.z) * vec3<f32>(other.g0.w, other.g0.w, other.g0.y) * vec3<f32>(1.0, 0.0, -1.0) + vec3<f32>(self_.g0.w) * vec3<f32>(other.g0.z, other.g0.y, other.g0.z) * vec3<f32>(-1.0, 1.0, 0.0) + vec3<f32>(self_.g0.x, self_.g0.y, self_.g0.y) * vec3<f32>(other.g0.x, other.g0.w, other.g0.z) * vec3<f32>(0.0, -1.0, 1.0));
}

fn plane_plane_inner_product(self_: Plane, other: Plane) -> Scalar  {
    return Scalar(0.0 - self_.g0.x * other.g0.x + self_.g0.y * other.g0.y + self_.g0.z * other.g0.z + self_.g0.w * other.g0.w);
}

fn plane_plane_left_contraction(self_: Plane, other: Plane) -> Scalar  {
    return Scalar(0.0 - self_.g0.x * other.g0.x + self_.g0.y * other.g0.y + self_.g0.z * other.g0.z + self_.g0.w * other.g0.w);
}

fn plane_plane_right_contraction(self_: Plane, other: Plane) -> Scalar  {
    return Scalar(0.0 - self_.g0.x * other.g0.x + self_.g0.y * other.g0.y + self_.g0.z * other.g0.z + self_.g0.w * other.g0.w);
}

fn plane_plane_scalar_product(self_: Plane, other: Plane) -> Scalar  {
    return Scalar(0.0 - self_.g0.x * other.g0.x + self_.g0.y * other.g0.y + self_.g0.z * other.g0.z + self_.g0.w * other.g0.w);
}

fn plane_line_geometric_product(self_: Plane, other: Line) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0.y) * vec4<f32>(other.g1.x, other.g1.x, other.g0.z, other.g0.y) * vec4<f32>(1.0, 0.0, -1.0, 1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.y, other.g0.z, other.g1.y, other.g0.x) * vec4<f32>(1.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g1.z, other.g0.y, other.g0.x, other.g1.z) * vec4<f32>(1.0, -1.0, 1.0, 0.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4<f32>(0.0, -1.0, -1.0, -1.0), vec4<f32>(self_.g0.y) * vec4<f32>(other.g0.x, other.g0.x, other.g1.z, other.g1.y) * vec4<f32>(-1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.y, other.g1.z, other.g0.y, other.g1.x) * vec4<f32>(-1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.z, other.g1.y, other.g1.x, other.g0.z) * vec4<f32>(-1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(0.0, -1.0, -1.0, -1.0));
}

fn plane_line_outer_product(self_: Plane, other: Line) -> Point  {
    return Point(vec4<f32>(self_.g0.y) * vec4<f32>(other.g1.x, other.g1.x, other.g0.z, other.g0.y) * vec4<f32>(1.0, 0.0, -1.0, 1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.y, other.g0.z, other.g1.y, other.g0.x) * vec4<f32>(1.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g1.z, other.g0.y, other.g0.x, other.g1.z) * vec4<f32>(1.0, -1.0, 1.0, 0.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4<f32>(0.0, -1.0, -1.0, -1.0));
}

fn plane_line_inner_product(self_: Plane, other: Line) -> Plane  {
    return Plane(vec4<f32>(self_.g0.y) * vec4<f32>(other.g0.x, other.g0.x, other.g1.z, other.g1.y) * vec4<f32>(-1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.y, other.g1.z, other.g0.y, other.g1.x) * vec4<f32>(-1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.z, other.g1.y, other.g1.x, other.g0.z) * vec4<f32>(-1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(0.0, -1.0, -1.0, -1.0));
}

fn plane_line_left_contraction(self_: Plane, other: Line) -> Plane  {
    return Plane(vec4<f32>(self_.g0.y) * vec4<f32>(other.g0.x, other.g0.x, other.g1.z, other.g1.y) * vec4<f32>(-1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.y, other.g1.z, other.g0.y, other.g1.x) * vec4<f32>(-1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.z, other.g1.y, other.g1.x, other.g0.z) * vec4<f32>(-1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(0.0, -1.0, -1.0, -1.0));
}

fn plane_translator_inner_product(self_: Plane, other: Translator) -> Plane  {
    return Plane(vec4<f32>(self_.g0.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g0.zzxz * vec4<f32>(-1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g0.wwwx * vec4<f32>(-1.0, 0.0, 0.0, 1.0) + self_.g0.yyxx * other.g0.yxxx * vec4<f32>(-1.0, 1.0, 0.0, 0.0));
}

fn plane_translator_left_contraction(self_: Plane, other: Translator) -> Plane  {
    return Plane(vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + self_.g0.yxxx * other.g0.yyzw * vec4<f32>(-1.0));
}

fn plane_translator_right_contraction(self_: Plane, other: Translator) -> Plane  {
    return Plane(self_.g0 * vec4<f32>(other.g0.x));
}

fn plane_motor_geometric_product(self_: Plane, other: Motor) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(0.0) - vec4<f32>(self_.g0.x) * vec4<f32>(other.g1.x, other.g0.y, other.g0.z, other.g0.w) + vec4<f32>(self_.g0.y) * vec4<f32>(other.g0.y, other.g1.x, other.g1.w, other.g1.z) * vec4<f32>(1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z, other.g1.w, other.g1.x, other.g1.y) * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w, other.g1.z, other.g1.y, other.g1.x) * vec4<f32>(1.0, -1.0, 1.0, 1.0), vec4<f32>(self_.g0.x) * vec4<f32>(other.g0.x, other.g1.y, other.g1.z, other.g1.w) * vec4<f32>(1.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g0.y) * vec4<f32>(other.g1.y, other.g0.x, other.g0.w, other.g0.z) * vec4<f32>(-1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.z, other.g0.w, other.g0.x, other.g0.y) * vec4<f32>(-1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g1.w, other.g0.z, other.g0.y, other.g0.x) * vec4<f32>(-1.0, 1.0, -1.0, 1.0));
}

fn plane_motor_regressive_product(self_: Plane, other: Motor) -> Plane  {
    return Plane(self_.g0 * vec4<f32>(other.g1.x));
}

fn plane_motor_outer_product(self_: Plane, other: Motor) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0.y) * vec4<f32>(other.g0.y, other.g0.y, other.g1.w, other.g1.z) * vec4<f32>(1.0, 0.0, -1.0, 1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z, other.g1.w, other.g0.z, other.g1.y) * vec4<f32>(1.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w, other.g1.z, other.g1.y, other.g0.w) * vec4<f32>(1.0, -1.0, 1.0, 0.0) + vec4<f32>(self_.g0.x) * other.g0 * vec4<f32>(0.0, -1.0, -1.0, -1.0), self_.g0 * vec4<f32>(other.g0.x));
}

fn plane_motor_inner_product(self_: Plane, other: Motor) -> PointAndPlane  {
    return PointAndPlane(self_.g0 * vec4<f32>(other.g1.x) * vec4<f32>(-1.0, 1.0, 1.0, 1.0), vec4<f32>(self_.g0.x) * vec4<f32>(other.g0.x, other.g1.y, other.g1.z, other.g1.w) * vec4<f32>(1.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g0.y) * vec4<f32>(other.g1.y, other.g0.x, other.g0.w, other.g0.z) * vec4<f32>(-1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.z, other.g0.w, other.g0.x, other.g0.y) * vec4<f32>(-1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g1.w, other.g0.z, other.g0.y, other.g0.x) * vec4<f32>(-1.0, 1.0, -1.0, 1.0));
}

fn plane_motor_left_contraction(self_: Plane, other: Motor) -> PointAndPlane  {
    return PointAndPlane(self_.g0 * vec4<f32>(other.g1.x) * vec4<f32>(-1.0, 1.0, 1.0, 1.0), vec4<f32>(self_.g0.y) * vec4<f32>(other.g1.y, other.g1.y, other.g0.w, other.g0.z) * vec4<f32>(-1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.z, other.g0.w, other.g1.z, other.g0.y) * vec4<f32>(-1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g1.w, other.g0.z, other.g0.y, other.g1.w) * vec4<f32>(-1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g0.x) * other.g1 * vec4<f32>(0.0, -1.0, -1.0, -1.0));
}

fn plane_motor_right_contraction(self_: Plane, other: Motor) -> Plane  {
    return Plane(self_.g0 * vec4<f32>(other.g0.x));
}

fn plane_point_and_plane_add(self_: Plane, other: PointAndPlane) -> PointAndPlane  {
    return PointAndPlane(other.g0, self_.g0 + other.g1);
}

fn plane_point_and_plane_sub(self_: Plane, other: PointAndPlane) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(0.0) - other.g0, self_.g0 - other.g1);
}

fn plane_point_and_plane_geometric_product(self_: Plane, other: PointAndPlane) -> Motor  {
    return Motor(vec4<f32>(self_.g0.x) * vec4<f32>(other.g1.x, other.g0.y, other.g0.z, other.g0.w) * vec4<f32>(-1.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g0.y) * vec4<f32>(other.g1.y, other.g0.x, other.g1.w, other.g1.z) * vec4<f32>(1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.z, other.g1.w, other.g0.x, other.g1.y) * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g1.w, other.g1.z, other.g1.y, other.g0.x) * vec4<f32>(1.0, -1.0, 1.0, 1.0), vec4<f32>(self_.g0.x) * vec4<f32>(other.g0.x, other.g1.y, other.g1.z, other.g1.w) + vec4<f32>(self_.g0.y) * vec4<f32>(other.g0.y, other.g1.x, other.g0.w, other.g0.z) * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z, other.g0.w, other.g1.x, other.g0.y) * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w, other.g0.z, other.g0.y, other.g1.x) * vec4<f32>(1.0, 1.0, -1.0, -1.0));
}

fn plane_point_and_plane_regressive_product(self_: Plane, other: PointAndPlane) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.x + self_.g0.y * other.g0.y + self_.g0.z * other.g0.z + self_.g0.w * other.g0.w);
}

fn plane_point_and_plane_right_contraction(self_: Plane, other: PointAndPlane) -> Scalar  {
    return Scalar(0.0 - self_.g0.x * other.g1.x + self_.g0.y * other.g1.y + self_.g0.z * other.g1.z + self_.g0.w * other.g1.w);
}

fn plane_point_and_plane_scalar_product(self_: Plane, other: PointAndPlane) -> Scalar  {
    return Scalar(0.0 - self_.g0.x * other.g1.x + self_.g0.y * other.g1.y + self_.g0.z * other.g1.z + self_.g0.w * other.g1.w);
}

fn plane_squared_magnitude(self_: Plane) -> Scalar  {
    return plane_plane_scalar_product(self_, plane_reversal(self_));
}

fn plane_magnitude(self_: Plane) -> Scalar  {
    return Scalar(sqrt(plane_squared_magnitude(self_).g0));
}

fn plane_scale(self_: Plane, other: f32) -> Plane  {
    return plane_scalar_geometric_product(self_, Scalar(other));
}

fn plane_signum(self_: Plane) -> Plane  {
    return plane_scalar_geometric_product(self_, Scalar(1.0 / plane_magnitude(self_).g0));
}

fn plane_inverse(self_: Plane) -> Plane  {
    return plane_scalar_geometric_product(plane_reversal(self_), Scalar(1.0 / plane_squared_magnitude(self_).g0));
}

fn line_zero() -> Line  {
    return Line(vec3<f32>(0.0), vec3<f32>(0.0));
}

fn line_one() -> Line  {
    return Line(vec3<f32>(0.0), vec3<f32>(0.0));
}

fn line_neg(self_: Line) -> Line  {
    return Line(self_.g0 * vec3<f32>(-1.0), self_.g1 * vec3<f32>(-1.0));
}

fn line_automorphism(self_: Line) -> Line  {
    return Line(self_.g0, self_.g1);
}

fn line_reversal(self_: Line) -> Line  {
    return Line(self_.g0 * vec3<f32>(-1.0), self_.g1 * vec3<f32>(-1.0));
}

fn line_conjugation(self_: Line) -> Line  {
    return Line(self_.g0 * vec3<f32>(-1.0), self_.g1 * vec3<f32>(-1.0));
}

fn line_dual(self_: Line) -> Line  {
    return Line(self_.g1, self_.g0);
}

fn line_scalar_geometric_product(self_: Line, other: Scalar) -> Line  {
    return Line(self_.g0 * vec3<f32>(other.g0), self_.g1 * vec3<f32>(other.g0));
}

fn line_scalar_outer_product(self_: Line, other: Scalar) -> Line  {
    return Line(self_.g0 * vec3<f32>(other.g0), self_.g1 * vec3<f32>(other.g0));
}

fn line_scalar_inner_product(self_: Line, other: Scalar) -> Line  {
    return Line(self_.g0 * vec3<f32>(other.g0), self_.g1 * vec3<f32>(other.g0));
}

fn line_scalar_right_contraction(self_: Line, other: Scalar) -> Line  {
    return Line(self_.g0 * vec3<f32>(other.g0), self_.g1 * vec3<f32>(other.g0));
}

fn line_multi_vector_add(self_: Line, other: MultiVector) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0.x, self_.g1.x, self_.g1.y, self_.g1.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0) + other.g0, other.g1, other.g2, vec4<f32>(self_.g0.x, self_.g0.x, self_.g0.y, self_.g0.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0) + other.g3);
}

fn line_multi_vector_sub(self_: Line, other: MultiVector) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0.x, self_.g1.x, self_.g1.y, self_.g1.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0) - other.g0, vec4<f32>(0.0) - other.g1, vec4<f32>(0.0) - other.g2, vec4<f32>(self_.g0.x, self_.g0.x, self_.g0.y, self_.g0.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0) - other.g3);
}

fn line_multi_vector_geometric_product(self_: Line, other: MultiVector) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0.x) * other.g3.yxwz * vec4<f32>(1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g0.y) * other.g3.zwxy * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g3.wzyx * vec4<f32>(1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g1.x) * other.g0.yxwz * vec4<f32>(-1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g1.y) * other.g0.zwxy * vec4<f32>(-1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g1.z) * other.g0.wzyx * vec4<f32>(-1.0, 1.0, -1.0, 1.0), vec4<f32>(self_.g0.x) * other.g2.yxwz * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g0.y) * other.g2.zwxy * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g0.z) * other.g2.wzyx * vec4<f32>(1.0, 1.0, -1.0, -1.0) + vec4<f32>(self_.g1.x) * other.g1.yxwz * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g1.y) * other.g1.zwxy * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g1.z) * other.g1.wzyx * vec4<f32>(1.0, 1.0, -1.0, -1.0), vec4<f32>(self_.g0.x) * other.g1.yxwz * vec4<f32>(-1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g0.y) * other.g1.zwxy * vec4<f32>(-1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g1.wzyx * vec4<f32>(-1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g1.x) * other.g2.yxwz * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g1.y) * other.g2.zwxy * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g1.z) * other.g2.wzyx * vec4<f32>(1.0, 1.0, -1.0, -1.0), vec4<f32>(self_.g0.x) * other.g0.yxwz * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.y) * other.g0.zwxy * vec4<f32>(1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g0.z) * other.g0.wzyx * vec4<f32>(1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g1.x) * other.g3.yxwz * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g1.y) * other.g3.zwxy * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g1.z) * other.g3.wzyx * vec4<f32>(1.0, 1.0, -1.0, -1.0));
}

fn line_multi_vector_scalar_product(self_: Line, other: MultiVector) -> Scalar  {
    return Scalar(self_.g0.x * other.g3.y + self_.g0.y * other.g3.z + self_.g0.z * other.g3.w - self_.g1.x * other.g0.y - self_.g1.y * other.g0.z - self_.g1.z * other.g0.w);
}

fn line_rotor_geometric_product(self_: Line, other: Rotor) -> Motor  {
    return Motor(vec4<f32>(self_.g1.x) * other.g0.yxwz * vec4<f32>(-1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g1.y) * other.g0.zwxy * vec4<f32>(-1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g1.z) * other.g0.wzyx * vec4<f32>(-1.0, 1.0, -1.0, 1.0), vec4<f32>(self_.g0.x) * other.g0.yxwz * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.y) * other.g0.zwxy * vec4<f32>(1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g0.z) * other.g0.wzyx * vec4<f32>(1.0, 1.0, -1.0, 1.0));
}

fn line_rotor_regressive_product(self_: Line, other: Rotor) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.y + self_.g0.y * other.g0.z + self_.g0.z * other.g0.w);
}

fn line_rotor_left_contraction(self_: Line, other: Rotor) -> Scalar  {
    return Scalar(0.0 - self_.g1.x * other.g0.y - self_.g1.y * other.g0.z - self_.g1.z * other.g0.w);
}

fn line_rotor_scalar_product(self_: Line, other: Rotor) -> Scalar  {
    return Scalar(0.0 - self_.g1.x * other.g0.y - self_.g1.y * other.g0.z - self_.g1.z * other.g0.w);
}

fn line_point_geometric_product(self_: Line, other: Point) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0.y) * other.g0.zzxz * vec4<f32>(-1.0, 0.0, -1.0, 0.0) + vec4<f32>(self_.g0.z) * other.g0.wwwx * vec4<f32>(-1.0, 0.0, 0.0, -1.0) + vec4<f32>(self_.g1.y) * other.g0.wwwy * vec4<f32>(0.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g1.z) * other.g0.zzyz * vec4<f32>(0.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g0.x, self_.g0.x, self_.g1.x, self_.g1.x) * other.g0.yxwz * vec4<f32>(-1.0, -1.0, 1.0, -1.0), vec4<f32>(self_.g0.y) * other.g0.wwwy * vec4<f32>(0.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g0.z) * other.g0.zzyz * vec4<f32>(0.0, -1.0, 1.0, 0.0) + vec4<f32>(self_.g1.y) * other.g0.zzxz * vec4<f32>(1.0, 0.0, -1.0, 0.0) + vec4<f32>(self_.g1.z) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, -1.0) + vec4<f32>(self_.g1.x, self_.g1.x, self_.g0.x, self_.g0.x) * other.g0.yxwz * vec4<f32>(1.0, -1.0, -1.0, 1.0));
}

fn line_point_regressive_product(self_: Line, other: Point) -> Plane  {
    return Plane(vec4<f32>(self_.g0.y) * other.g0.zzxz * vec4<f32>(-1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.z) * other.g0.wwwx * vec4<f32>(-1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g1.y) * other.g0.wwwy * vec4<f32>(0.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g1.z) * other.g0.zzyz * vec4<f32>(0.0, -1.0, 1.0, 0.0) + vec4<f32>(self_.g0.x, self_.g0.x, self_.g1.x, self_.g1.x) * other.g0.yxwz * vec4<f32>(-1.0, 1.0, -1.0, 1.0));
}

fn line_point_inner_product(self_: Line, other: Point) -> Plane  {
    return Plane(vec4<f32>(self_.g0.y) * other.g0.wwwy * vec4<f32>(0.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g0.z) * other.g0.zzyz * vec4<f32>(0.0, -1.0, 1.0, 0.0) + vec4<f32>(self_.g1.y) * other.g0.zzxz * vec4<f32>(1.0, 0.0, -1.0, 0.0) + vec4<f32>(self_.g1.z) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, -1.0) + vec4<f32>(self_.g1.x, self_.g1.x, self_.g0.x, self_.g0.x) * other.g0.yxwz * vec4<f32>(1.0, -1.0, -1.0, 1.0));
}

fn line_point_left_contraction(self_: Line, other: Point) -> Plane  {
    return Plane(vec4<f32>(self_.g0.y) * other.g0.wwwy * vec4<f32>(0.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g0.z) * other.g0.zzyz * vec4<f32>(0.0, -1.0, 1.0, 0.0) + vec4<f32>(self_.g1.y) * other.g0.zzxz * vec4<f32>(1.0, 0.0, -1.0, 0.0) + vec4<f32>(self_.g1.z) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, -1.0) + vec4<f32>(self_.g1.x, self_.g1.x, self_.g0.x, self_.g0.x) * other.g0.yxwz * vec4<f32>(1.0, -1.0, -1.0, 1.0));
}

fn line_ideal_point_into(self_: Line) -> IdealPoint  {
    return IdealPoint(self_.g0);
}

fn line_ideal_point_add(self_: Line, other: IdealPoint) -> Line  {
    return Line(self_.g0 + other.g0, self_.g1);
}

fn line_ideal_point_sub(self_: Line, other: IdealPoint) -> Line  {
    return Line(self_.g0 - other.g0, self_.g1);
}

fn line_ideal_point_geometric_product(self_: Line, other: IdealPoint) -> Motor  {
    return Motor(vec4<f32>(self_.g0.y) * vec4<f32>(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4<f32>(1.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4<f32>(1.0, -1.0, 1.0, 0.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4<f32>(1.0, 0.0, -1.0, 1.0), vec4<f32>(self_.g1.y) * vec4<f32>(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4<f32>(1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4<f32>(1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4<f32>(1.0, 0.0, 1.0, -1.0));
}

fn line_ideal_point_regressive_product(self_: Line, other: IdealPoint) -> Scalar  {
    return Scalar(self_.g1.x * other.g0.x + self_.g1.y * other.g0.y + self_.g1.z * other.g0.z);
}

fn line_ideal_point_inner_product(self_: Line, other: IdealPoint) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.x + self_.g0.y * other.g0.y + self_.g0.z * other.g0.z);
}

fn line_ideal_point_left_contraction(self_: Line, other: IdealPoint) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.x + self_.g0.y * other.g0.y + self_.g0.z * other.g0.z);
}

fn line_ideal_point_right_contraction(self_: Line, other: IdealPoint) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.x + self_.g0.y * other.g0.y + self_.g0.z * other.g0.z);
}

fn line_ideal_point_scalar_product(self_: Line, other: IdealPoint) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.x + self_.g0.y * other.g0.y + self_.g0.z * other.g0.z);
}

fn line_plane_geometric_product(self_: Line, other: Plane) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0.y) * other.g0.wwwy * vec4<f32>(0.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.z) * other.g0.zzyz * vec4<f32>(0.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g1.y) * other.g0.zzxz * vec4<f32>(1.0, 0.0, -1.0, 0.0) + vec4<f32>(self_.g1.z) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, -1.0) + vec4<f32>(self_.g1.x, self_.g1.x, self_.g0.x, self_.g0.x) * other.g0.yxwz * vec4<f32>(1.0, -1.0, 1.0, -1.0), vec4<f32>(self_.g0.y) * other.g0.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.z) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g1.y) * other.g0.wwwy * vec4<f32>(0.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g1.z) * other.g0.zzyz * vec4<f32>(0.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g0.x, self_.g0.x, self_.g1.x, self_.g1.x) * other.g0.yxwz * vec4<f32>(1.0, 1.0, 1.0, -1.0));
}

fn line_plane_outer_product(self_: Line, other: Plane) -> Point  {
    return Point(vec4<f32>(self_.g0.y) * other.g0.wwwy * vec4<f32>(0.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.z) * other.g0.zzyz * vec4<f32>(0.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g1.y) * other.g0.zzxz * vec4<f32>(1.0, 0.0, -1.0, 0.0) + vec4<f32>(self_.g1.z) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, -1.0) + vec4<f32>(self_.g1.x, self_.g1.x, self_.g0.x, self_.g0.x) * other.g0.yxwz * vec4<f32>(1.0, -1.0, 1.0, -1.0));
}

fn line_plane_inner_product(self_: Line, other: Plane) -> Plane  {
    return Plane(vec4<f32>(self_.g0.y) * other.g0.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.z) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g1.y) * other.g0.wwwy * vec4<f32>(0.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g1.z) * other.g0.zzyz * vec4<f32>(0.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g0.x, self_.g0.x, self_.g1.x, self_.g1.x) * other.g0.yxwz * vec4<f32>(1.0, 1.0, 1.0, -1.0));
}

fn line_plane_right_contraction(self_: Line, other: Plane) -> Plane  {
    return Plane(vec4<f32>(self_.g0.y) * other.g0.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.z) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g1.y) * other.g0.wwwy * vec4<f32>(0.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g1.z) * other.g0.zzyz * vec4<f32>(0.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g0.x, self_.g0.x, self_.g1.x, self_.g1.x) * other.g0.yxwz * vec4<f32>(1.0, 1.0, 1.0, -1.0));
}

fn line_line_add(self_: Line, other: Line) -> Line  {
    return Line(self_.g0 + other.g0, self_.g1 + other.g1);
}

fn line_line_sub(self_: Line, other: Line) -> Line  {
    return Line(self_.g0 - other.g0, self_.g1 - other.g1);
}

fn line_line_mul(self_: Line, other: Line) -> Line  {
    return Line(self_.g0 * other.g0, self_.g1 * other.g1);
}

fn line_line_div(self_: Line, other: Line) -> Line  {
    return Line(vec3<f32>(self_.g0.x, self_.g0.y, self_.g0.z) * vec3<f32>(1.0, 1.0, 1.0) / vec3<f32>(other.g0.x, other.g0.y, other.g0.z) * vec3<f32>(1.0, 1.0, 1.0), vec3<f32>(self_.g1.x, self_.g1.y, self_.g1.z) * vec3<f32>(1.0, 1.0, 1.0) / vec3<f32>(other.g1.x, other.g1.y, other.g1.z) * vec3<f32>(1.0, 1.0, 1.0));
}

fn line_line_geometric_product(self_: Line, other: Line) -> Motor  {
    return Motor(vec4<f32>(self_.g0.y) * vec4<f32>(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4<f32>(1.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4<f32>(1.0, -1.0, 1.0, 0.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g1.x, other.g1.x, other.g1.z, other.g1.y) * vec4<f32>(-1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g1.y, other.g1.z, other.g1.y, other.g1.x) * vec4<f32>(-1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g1.z, other.g1.y, other.g1.x, other.g1.z) * vec4<f32>(-1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4<f32>(1.0, 0.0, -1.0, 1.0), vec4<f32>(self_.g0.y) * vec4<f32>(other.g1.y, other.g1.z, other.g1.y, other.g1.x) * vec4<f32>(1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.z, other.g1.y, other.g1.x, other.g1.z) * vec4<f32>(1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4<f32>(1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4<f32>(1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4<f32>(1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g1.x, other.g1.x, other.g1.z, other.g1.y) * vec4<f32>(1.0, 0.0, 1.0, -1.0));
}

fn line_line_regressive_product(self_: Line, other: Line) -> Scalar  {
    return Scalar(self_.g0.x * other.g1.x + self_.g0.y * other.g1.y + self_.g0.z * other.g1.z + self_.g1.x * other.g0.x + self_.g1.y * other.g0.y + self_.g1.z * other.g0.z);
}

fn line_line_inner_product(self_: Line, other: Line) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.x + self_.g0.y * other.g0.y + self_.g0.z * other.g0.z - self_.g1.x * other.g1.x - self_.g1.y * other.g1.y - self_.g1.z * other.g1.z);
}

fn line_line_left_contraction(self_: Line, other: Line) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.x + self_.g0.y * other.g0.y + self_.g0.z * other.g0.z - self_.g1.x * other.g1.x - self_.g1.y * other.g1.y - self_.g1.z * other.g1.z);
}

fn line_line_right_contraction(self_: Line, other: Line) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.x + self_.g0.y * other.g0.y + self_.g0.z * other.g0.z - self_.g1.x * other.g1.x - self_.g1.y * other.g1.y - self_.g1.z * other.g1.z);
}

fn line_line_scalar_product(self_: Line, other: Line) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.x + self_.g0.y * other.g0.y + self_.g0.z * other.g0.z - self_.g1.x * other.g1.x - self_.g1.y * other.g1.y - self_.g1.z * other.g1.z);
}

fn line_translator_geometric_product(self_: Line, other: Translator) -> Motor  {
    return Motor(vec4<f32>(self_.g0.y) * other.g0.zwzy * vec4<f32>(1.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g0.z) * other.g0.wzyw * vec4<f32>(1.0, -1.0, 1.0, 0.0) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g0.x, self_.g1.x, self_.g0.x, self_.g0.x) * other.g0.yxwz * vec4<f32>(1.0, 1.0, -1.0, 1.0), vec4<f32>(self_.g1.x) * other.g0.yywz * vec4<f32>(1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g1.y) * other.g0.zwzy * vec4<f32>(1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g1.z) * other.g0.wzyw * vec4<f32>(1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g0.x, self_.g0.x, self_.g0.y, self_.g0.z) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 1.0, 1.0));
}

fn line_translator_regressive_product(self_: Line, other: Translator) -> Scalar  {
    return Scalar(self_.g1.x * other.g0.y + self_.g1.y * other.g0.z + self_.g1.z * other.g0.w);
}

fn line_translator_left_contraction(self_: Line, other: Translator) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.y + self_.g0.y * other.g0.z + self_.g0.z * other.g0.w);
}

fn line_translator_scalar_product(self_: Line, other: Translator) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.y + self_.g0.y * other.g0.z + self_.g0.z * other.g0.w);
}

fn line_motor_add(self_: Line, other: Motor) -> Motor  {
    return Motor(vec4<f32>(self_.g0.x, self_.g1.x, self_.g1.y, self_.g1.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0) + other.g0, vec4<f32>(self_.g0.x, self_.g0.x, self_.g0.y, self_.g0.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0) + other.g1);
}

fn line_motor_sub(self_: Line, other: Motor) -> Motor  {
    return Motor(vec4<f32>(self_.g0.x, self_.g1.x, self_.g1.y, self_.g1.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0) - other.g0, vec4<f32>(self_.g0.x, self_.g0.x, self_.g0.y, self_.g0.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0) - other.g1);
}

fn line_motor_geometric_product(self_: Line, other: Motor) -> Motor  {
    return Motor(vec4<f32>(self_.g0.x) * other.g1.yxwz * vec4<f32>(1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g0.y) * other.g1.zwxy * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g1.wzyx * vec4<f32>(1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g1.x) * other.g0.yxwz * vec4<f32>(-1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g1.y) * other.g0.zwxy * vec4<f32>(-1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g1.z) * other.g0.wzyx * vec4<f32>(-1.0, 1.0, -1.0, 1.0), vec4<f32>(self_.g0.x) * other.g0.yxwz * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.y) * other.g0.zwxy * vec4<f32>(1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g0.z) * other.g0.wzyx * vec4<f32>(1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g1.x) * other.g1.yxwz * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g1.y) * other.g1.zwxy * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g1.z) * other.g1.wzyx * vec4<f32>(1.0, 1.0, -1.0, -1.0));
}

fn line_motor_scalar_product(self_: Line, other: Motor) -> Scalar  {
    return Scalar(self_.g0.x * other.g1.y + self_.g0.y * other.g1.z + self_.g0.z * other.g1.w - self_.g1.x * other.g0.y - self_.g1.y * other.g0.z - self_.g1.z * other.g0.w);
}

fn line_point_and_plane_geometric_product(self_: Line, other: PointAndPlane) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0.x) * vec4<f32>(other.g0.y, other.g0.x, other.g1.w, other.g1.z) * vec4<f32>(-1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g0.y) * vec4<f32>(other.g0.z, other.g1.w, other.g0.x, other.g1.y) * vec4<f32>(-1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4<f32>(-1.0, 1.0, -1.0, -1.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4<f32>(1.0, 1.0, -1.0, -1.0), vec4<f32>(self_.g0.x) * vec4<f32>(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4<f32>(1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g0.y) * vec4<f32>(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4<f32>(1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g0.y, other.g0.x, other.g1.w, other.g1.z) * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g0.z, other.g1.w, other.g0.x, other.g1.y) * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4<f32>(1.0, 1.0, -1.0, -1.0));
}

fn line_point_and_plane_regressive_product(self_: Line, other: PointAndPlane) -> Plane  {
    return Plane(vec4<f32>(self_.g0.y) * other.g0.zzxz * vec4<f32>(-1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.z) * other.g0.wwwx * vec4<f32>(-1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g1.y) * other.g0.wwwy * vec4<f32>(0.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g1.z) * other.g0.zzyz * vec4<f32>(0.0, -1.0, 1.0, 0.0) + vec4<f32>(self_.g0.x, self_.g0.x, self_.g1.x, self_.g1.x) * other.g0.yxwz * vec4<f32>(-1.0, 1.0, -1.0, 1.0));
}

fn line_point_and_plane_outer_product(self_: Line, other: PointAndPlane) -> Point  {
    return Point(vec4<f32>(self_.g0.y) * other.g1.wwwy * vec4<f32>(0.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.z) * other.g1.zzyz * vec4<f32>(0.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g1.y) * other.g1.zzxz * vec4<f32>(1.0, 0.0, -1.0, 0.0) + vec4<f32>(self_.g1.z) * other.g1.wwwx * vec4<f32>(1.0, 0.0, 0.0, -1.0) + vec4<f32>(self_.g1.x, self_.g1.x, self_.g0.x, self_.g0.x) * other.g1.yxwz * vec4<f32>(1.0, -1.0, 1.0, -1.0));
}

fn line_point_and_plane_inner_product(self_: Line, other: PointAndPlane) -> Plane  {
    return Plane(vec4<f32>(self_.g0.x) * vec4<f32>(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4<f32>(1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g0.y) * vec4<f32>(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4<f32>(1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g0.y, other.g0.x, other.g1.w, other.g1.z) * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g0.z, other.g1.w, other.g0.x, other.g1.y) * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4<f32>(1.0, 1.0, -1.0, -1.0));
}

fn line_point_and_plane_left_contraction(self_: Line, other: PointAndPlane) -> Plane  {
    return Plane(vec4<f32>(self_.g0.y) * other.g0.wwwy * vec4<f32>(0.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g0.z) * other.g0.zzyz * vec4<f32>(0.0, -1.0, 1.0, 0.0) + vec4<f32>(self_.g1.y) * other.g0.zzxz * vec4<f32>(1.0, 0.0, -1.0, 0.0) + vec4<f32>(self_.g1.z) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, -1.0) + vec4<f32>(self_.g1.x, self_.g1.x, self_.g0.x, self_.g0.x) * other.g0.yxwz * vec4<f32>(1.0, -1.0, -1.0, 1.0));
}

fn line_point_and_plane_right_contraction(self_: Line, other: PointAndPlane) -> Plane  {
    return Plane(vec4<f32>(self_.g0.y) * other.g1.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.z) * other.g1.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g1.y) * other.g1.wwwy * vec4<f32>(0.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g1.z) * other.g1.zzyz * vec4<f32>(0.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g0.x, self_.g0.x, self_.g1.x, self_.g1.x) * other.g1.yxwz * vec4<f32>(1.0, 1.0, 1.0, -1.0));
}

fn line_squared_magnitude(self_: Line) -> Scalar  {
    return line_line_scalar_product(self_, line_reversal(self_));
}

fn line_magnitude(self_: Line) -> Scalar  {
    return Scalar(sqrt(line_squared_magnitude(self_).g0));
}

fn line_scale(self_: Line, other: f32) -> Line  {
    return line_scalar_geometric_product(self_, Scalar(other));
}

fn line_signum(self_: Line) -> Line  {
    return line_scalar_geometric_product(self_, Scalar(1.0 / line_magnitude(self_).g0));
}

fn line_inverse(self_: Line) -> Line  {
    return line_scalar_geometric_product(line_reversal(self_), Scalar(1.0 / line_squared_magnitude(self_).g0));
}

fn translator_zero() -> Translator  {
    return Translator(vec4<f32>(0.0));
}

fn translator_one() -> Translator  {
    return Translator(vec4<f32>(1.0, 0.0, 0.0, 0.0));
}

fn translator_neg(self_: Translator) -> Translator  {
    return Translator(self_.g0 * vec4<f32>(-1.0));
}

fn translator_automorphism(self_: Translator) -> Translator  {
    return Translator(self_.g0);
}

fn translator_reversal(self_: Translator) -> Translator  {
    return Translator(self_.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0));
}

fn translator_conjugation(self_: Translator) -> Translator  {
    return Translator(self_.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0));
}

fn translator_scalar_into(self_: Translator) -> Scalar  {
    return Scalar(self_.g0.x);
}

fn translator_scalar_add(self_: Translator, other: Scalar) -> Translator  {
    return Translator(self_.g0 + vec4<f32>(other.g0) * vec4<f32>(1.0, 0.0, 0.0, 0.0));
}

fn translator_scalar_sub(self_: Translator, other: Scalar) -> Translator  {
    return Translator(self_.g0 - vec4<f32>(other.g0) * vec4<f32>(1.0, 0.0, 0.0, 0.0));
}

fn translator_scalar_geometric_product(self_: Translator, other: Scalar) -> Translator  {
    return Translator(self_.g0 * vec4<f32>(other.g0));
}

fn translator_scalar_outer_product(self_: Translator, other: Scalar) -> Translator  {
    return Translator(self_.g0 * vec4<f32>(other.g0));
}

fn translator_scalar_inner_product(self_: Translator, other: Scalar) -> Translator  {
    return Translator(self_.g0 * vec4<f32>(other.g0));
}

fn translator_scalar_left_contraction(self_: Translator, other: Scalar) -> Scalar  {
    return Scalar(self_.g0.x * other.g0);
}

fn translator_scalar_right_contraction(self_: Translator, other: Scalar) -> Translator  {
    return Translator(self_.g0 * vec4<f32>(other.g0));
}

fn translator_scalar_scalar_product(self_: Translator, other: Scalar) -> Scalar  {
    return Scalar(self_.g0.x * other.g0);
}

fn translator_multi_vector_add(self_: Translator, other: MultiVector) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + other.g0, other.g1, other.g2, self_.g0 * vec4<f32>(0.0, 1.0, 1.0, 1.0) + other.g3);
}

fn translator_multi_vector_sub(self_: Translator, other: MultiVector) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0) - other.g0, vec4<f32>(0.0) - other.g1, vec4<f32>(0.0) - other.g2, self_.g0 * vec4<f32>(0.0, 1.0, 1.0, 1.0) - other.g3);
}

fn translator_multi_vector_geometric_product(self_: Translator, other: MultiVector) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.y) * other.g3.yxwz * vec4<f32>(1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g0.z) * other.g3.zwxy * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.w) * other.g3.wzyx * vec4<f32>(1.0, -1.0, 1.0, 1.0), vec4<f32>(self_.g0.x) * other.g1 + vec4<f32>(self_.g0.y) * other.g2.yxwz * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g2.zwxy * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g0.w) * other.g2.wzyx * vec4<f32>(1.0, 1.0, -1.0, -1.0), vec4<f32>(self_.g0.x) * other.g2 + vec4<f32>(self_.g0.y) * other.g1.yxwz * vec4<f32>(-1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g0.z) * other.g1.zwxy * vec4<f32>(-1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.w) * other.g1.wzyx * vec4<f32>(-1.0, -1.0, 1.0, 1.0), vec4<f32>(self_.g0.x) * other.g3 + vec4<f32>(self_.g0.y) * other.g0.yxwz * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g0.zwxy * vec4<f32>(1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g0.w) * other.g0.wzyx * vec4<f32>(1.0, 1.0, -1.0, 1.0));
}

fn translator_multi_vector_outer_product(self_: Translator, other: MultiVector) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0.x) * other.g0, vec4<f32>(self_.g0.x) * other.g1 + vec4<f32>(self_.g0.z) * other.g2.wwwy * vec4<f32>(0.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * other.g2.zzyz * vec4<f32>(0.0, 1.0, -1.0, 0.0) + self_.g0.xxyy * other.g2.xxwz * vec4<f32>(0.0, 0.0, 1.0, -1.0), vec4<f32>(self_.g0.x) * other.g2, vec4<f32>(self_.g0.x) * other.g3 + vec4<f32>(self_.g0.z) * other.g0.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + self_.g0.yyxx * other.g0.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0));
}

fn translator_multi_vector_inner_product(self_: Translator, other: MultiVector) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.z) * other.g3.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g3.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + self_.g0.yyxx * other.g3.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0), vec4<f32>(self_.g0.x) * other.g1 + vec4<f32>(self_.g0.z) * vec4<f32>(other.g2.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g2.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + self_.g0.yxxx * other.g2.yxxx * vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(self_.g0.x) * other.g2 + vec4<f32>(self_.g0.z) * other.g1.wwxy * vec4<f32>(0.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.w) * other.g1.zzyx * vec4<f32>(0.0, -1.0, 1.0, 1.0) + self_.g0.xyyy * other.g1.xxwz * vec4<f32>(0.0, 1.0, -1.0, 1.0), vec4<f32>(self_.g0.x) * other.g3 + self_.g0 * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 1.0, 1.0));
}

fn translator_multi_vector_left_contraction(self_: Translator, other: MultiVector) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.z) * other.g3.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g3.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + self_.g0.yyxx * other.g3.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0), vec4<f32>(self_.g0.x) * other.g1, vec4<f32>(self_.g0.x) * other.g2 + vec4<f32>(self_.g0.z) * other.g1.wwwy * vec4<f32>(0.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g0.w) * other.g1.zzyz * vec4<f32>(0.0, -1.0, 1.0, 0.0) + self_.g0.xxyy * other.g1.xxwz * vec4<f32>(0.0, 0.0, -1.0, 1.0), vec4<f32>(self_.g0.x) * other.g3);
}

fn translator_multi_vector_scalar_product(self_: Translator, other: MultiVector) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.x + self_.g0.y * other.g3.y + self_.g0.z * other.g3.z + self_.g0.w * other.g3.w);
}

fn translator_rotor_geometric_product(self_: Translator, other: Rotor) -> Motor  {
    return Motor(vec4<f32>(self_.g0.x) * other.g0, vec4<f32>(self_.g0.y) * other.g0.yxwz * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g0.zwxy * vec4<f32>(1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g0.w) * other.g0.wzyx * vec4<f32>(1.0, 1.0, -1.0, 1.0));
}

fn translator_rotor_regressive_product(self_: Translator, other: Rotor) -> Scalar  {
    return Scalar(self_.g0.y * other.g0.y + self_.g0.z * other.g0.z + self_.g0.w * other.g0.w);
}

fn translator_rotor_outer_product(self_: Translator, other: Rotor) -> Motor  {
    return Motor(vec4<f32>(self_.g0.x) * other.g0, vec4<f32>(self_.g0.z) * other.g0.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + self_.g0.yyxx * other.g0.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0));
}

fn translator_rotor_left_contraction(self_: Translator, other: Rotor) -> Rotor  {
    return Rotor(vec4<f32>(self_.g0.x) * other.g0);
}

fn translator_rotor_right_contraction(self_: Translator, other: Rotor) -> Translator  {
    return Translator(self_.g0 * vec4<f32>(other.g0.x));
}

fn translator_rotor_scalar_product(self_: Translator, other: Rotor) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.x);
}

fn translator_point_regressive_product(self_: Translator, other: Point) -> Plane  {
    return Plane(vec4<f32>(self_.g0.z) * other.g0.zzxz * vec4<f32>(-1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g0.wwwx * vec4<f32>(-1.0, 0.0, 0.0, 1.0) + self_.g0.yyxx * other.g0.yxxx * vec4<f32>(-1.0, 1.0, 0.0, 0.0));
}

fn translator_point_outer_product(self_: Translator, other: Point) -> Point  {
    return Point(vec4<f32>(self_.g0.x) * other.g0);
}

fn translator_ideal_point_into(self_: Translator) -> IdealPoint  {
    return IdealPoint(vec3<f32>(self_.g0.y, self_.g0.z, self_.g0.w));
}

fn translator_ideal_point_add(self_: Translator, other: IdealPoint) -> Translator  {
    return Translator(self_.g0 + vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0));
}

fn translator_ideal_point_sub(self_: Translator, other: IdealPoint) -> Translator  {
    return Translator(self_.g0 - vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0));
}

fn translator_ideal_point_outer_product(self_: Translator, other: IdealPoint) -> IdealPoint  {
    return IdealPoint(vec3<f32>(self_.g0.x) * other.g0);
}

fn translator_ideal_point_inner_product(self_: Translator, other: IdealPoint) -> Translator  {
    return Translator(vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.y) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + self_.g0.yxxx * vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z));
}

fn translator_ideal_point_left_contraction(self_: Translator, other: IdealPoint) -> Translator  {
    return Translator(vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.y) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + self_.g0.yxxx * vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z));
}

fn translator_ideal_point_right_contraction(self_: Translator, other: IdealPoint) -> Scalar  {
    return Scalar(self_.g0.y * other.g0.x + self_.g0.z * other.g0.y + self_.g0.w * other.g0.z);
}

fn translator_ideal_point_scalar_product(self_: Translator, other: IdealPoint) -> Scalar  {
    return Scalar(self_.g0.y * other.g0.x + self_.g0.z * other.g0.y + self_.g0.w * other.g0.z);
}

fn translator_plane_inner_product(self_: Translator, other: Plane) -> Plane  {
    return Plane(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.z) * other.g0.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + self_.g0.yyxx * other.g0.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0));
}

fn translator_plane_left_contraction(self_: Translator, other: Plane) -> Plane  {
    return Plane(vec4<f32>(self_.g0.x) * other.g0);
}

fn translator_plane_right_contraction(self_: Translator, other: Plane) -> Plane  {
    return Plane(vec4<f32>(self_.g0.z) * other.g0.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + self_.g0.yyxx * other.g0.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0));
}

fn translator_line_geometric_product(self_: Translator, other: Line) -> Motor  {
    return Motor(vec4<f32>(self_.g0.y) * vec4<f32>(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4<f32>(1.0, 0.0, -1.0, 1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4<f32>(1.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4<f32>(1.0, -1.0, 1.0, 0.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0), vec4<f32>(self_.g0.y) * vec4<f32>(other.g1.x, other.g1.x, other.g1.z, other.g1.y) * vec4<f32>(1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.y, other.g1.z, other.g1.y, other.g1.x) * vec4<f32>(1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g1.z, other.g1.y, other.g1.x, other.g1.z) * vec4<f32>(1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0));
}

fn translator_line_regressive_product(self_: Translator, other: Line) -> Scalar  {
    return Scalar(self_.g0.y * other.g1.x + self_.g0.z * other.g1.y + self_.g0.w * other.g1.z);
}

fn translator_line_right_contraction(self_: Translator, other: Line) -> Scalar  {
    return Scalar(self_.g0.y * other.g0.x + self_.g0.z * other.g0.y + self_.g0.w * other.g0.z);
}

fn translator_line_scalar_product(self_: Translator, other: Line) -> Scalar  {
    return Scalar(self_.g0.y * other.g0.x + self_.g0.z * other.g0.y + self_.g0.w * other.g0.z);
}

fn translator_translator_add(self_: Translator, other: Translator) -> Translator  {
    return Translator(self_.g0 + other.g0);
}

fn translator_translator_sub(self_: Translator, other: Translator) -> Translator  {
    return Translator(self_.g0 - other.g0);
}

fn translator_translator_mul(self_: Translator, other: Translator) -> Translator  {
    return Translator(self_.g0 * other.g0);
}

fn translator_translator_div(self_: Translator, other: Translator) -> Translator  {
    return Translator(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0) / vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0));
}

fn translator_translator_outer_product(self_: Translator, other: Translator) -> Translator  {
    return Translator(vec4<f32>(self_.g0.x) * other.g0 + self_.g0 * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 1.0, 1.0));
}

fn translator_translator_inner_product(self_: Translator, other: Translator) -> Translator  {
    return Translator(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.z) * other.g0.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + self_.g0.yyxx * other.g0.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0));
}

fn translator_translator_left_contraction(self_: Translator, other: Translator) -> Translator  {
    return Translator(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + self_.g0.yxxx * other.g0.yxxx * vec4<f32>(1.0, 0.0, 0.0, 0.0));
}

fn translator_translator_right_contraction(self_: Translator, other: Translator) -> Translator  {
    return Translator(vec4<f32>(self_.g0.y) * other.g0.yxyy * vec4<f32>(1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g0.z) * other.g0.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0));
}

fn translator_translator_scalar_product(self_: Translator, other: Translator) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.x + self_.g0.y * other.g0.y + self_.g0.z * other.g0.z + self_.g0.w * other.g0.w);
}

fn translator_motor_add(self_: Translator, other: Motor) -> Motor  {
    return Motor(vec4<f32>(self_.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + other.g0, self_.g0 * vec4<f32>(0.0, 1.0, 1.0, 1.0) + other.g1);
}

fn translator_motor_sub(self_: Translator, other: Motor) -> Motor  {
    return Motor(vec4<f32>(self_.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0) - other.g0, self_.g0 * vec4<f32>(0.0, 1.0, 1.0, 1.0) - other.g1);
}

fn translator_motor_geometric_product(self_: Translator, other: Motor) -> Motor  {
    return Motor(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.y) * other.g1.yxwz * vec4<f32>(1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g0.z) * other.g1.zwxy * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.w) * other.g1.wzyx * vec4<f32>(1.0, -1.0, 1.0, 1.0), vec4<f32>(self_.g0.x) * other.g1 + vec4<f32>(self_.g0.y) * other.g0.yxwz * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g0.zwxy * vec4<f32>(1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g0.w) * other.g0.wzyx * vec4<f32>(1.0, 1.0, -1.0, 1.0));
}

fn translator_motor_regressive_product(self_: Translator, other: Motor) -> Translator  {
    return Translator(vec4<f32>(self_.g0.y) * vec4<f32>(other.g0.y, other.g1.x, other.g0.y, other.g0.y) * vec4<f32>(1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z, other.g0.z, other.g1.x, other.g0.z) * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w, other.g0.w, other.g0.w, other.g1.x) * vec4<f32>(1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g1.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0));
}

fn translator_motor_outer_product(self_: Translator, other: Motor) -> Motor  {
    return Motor(vec4<f32>(self_.g0.x) * other.g0, vec4<f32>(self_.g0.x) * other.g1 + vec4<f32>(self_.g0.z) * other.g0.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + self_.g0.yyxx * other.g0.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0));
}

fn translator_motor_inner_product(self_: Translator, other: Motor) -> Motor  {
    return Motor(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.z) * other.g1.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g1.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + self_.g0.yyxx * other.g1.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0), vec4<f32>(self_.g0.x) * other.g1 + self_.g0 * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 1.0, 1.0));
}

fn translator_motor_left_contraction(self_: Translator, other: Motor) -> Motor  {
    return Motor(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.z) * other.g1.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g1.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + self_.g0.yyxx * other.g1.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0), vec4<f32>(self_.g0.x) * other.g1);
}

fn translator_motor_right_contraction(self_: Translator, other: Motor) -> Translator  {
    return Translator(vec4<f32>(self_.g0.y) * vec4<f32>(other.g1.y, other.g0.x, other.g1.y, other.g1.y) * vec4<f32>(1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.z, other.g1.z, other.g0.x, other.g1.z) * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g1.w, other.g1.w, other.g1.w, other.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0));
}

fn translator_motor_scalar_product(self_: Translator, other: Motor) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.x + self_.g0.y * other.g1.y + self_.g0.z * other.g1.z + self_.g0.w * other.g1.w);
}

fn translator_point_and_plane_geometric_product(self_: Translator, other: PointAndPlane) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.y) * vec4<f32>(other.g0.y, other.g0.x, other.g1.w, other.g1.z) * vec4<f32>(-1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z, other.g1.w, other.g0.x, other.g1.y) * vec4<f32>(-1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4<f32>(-1.0, 1.0, -1.0, -1.0), vec4<f32>(self_.g0.x) * other.g1 + vec4<f32>(self_.g0.y) * vec4<f32>(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4<f32>(1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4<f32>(1.0, -1.0, 1.0, 1.0));
}

fn translator_point_and_plane_regressive_product(self_: Translator, other: PointAndPlane) -> Plane  {
    return Plane(vec4<f32>(self_.g0.z) * other.g0.zzxz * vec4<f32>(-1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g0.wwwx * vec4<f32>(-1.0, 0.0, 0.0, 1.0) + self_.g0.yyxx * other.g0.yxxx * vec4<f32>(-1.0, 1.0, 0.0, 0.0));
}

fn translator_point_and_plane_outer_product(self_: Translator, other: PointAndPlane) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.z) * other.g1.wwwy * vec4<f32>(0.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * other.g1.zzyz * vec4<f32>(0.0, 1.0, -1.0, 0.0) + self_.g0.xxyy * other.g1.xxwz * vec4<f32>(0.0, 0.0, 1.0, -1.0), vec4<f32>(self_.g0.x) * other.g1);
}

fn translator_point_and_plane_inner_product(self_: Translator, other: PointAndPlane) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0.x) * other.g0, vec4<f32>(self_.g0.x) * other.g1 + vec4<f32>(self_.g0.y) * vec4<f32>(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4<f32>(1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4<f32>(1.0, -1.0, 1.0, 1.0));
}

fn translator_point_and_plane_left_contraction(self_: Translator, other: PointAndPlane) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0.x) * other.g0, vec4<f32>(self_.g0.x) * other.g1 + vec4<f32>(self_.g0.z) * other.g0.wwwy * vec4<f32>(0.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g0.w) * other.g0.zzyz * vec4<f32>(0.0, -1.0, 1.0, 0.0) + self_.g0.xxyy * other.g0.xxwz * vec4<f32>(0.0, 0.0, -1.0, 1.0));
}

fn translator_point_and_plane_right_contraction(self_: Translator, other: PointAndPlane) -> Plane  {
    return Plane(vec4<f32>(self_.g0.z) * other.g1.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g1.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + self_.g0.yyxx * other.g1.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0));
}

fn translator_squared_magnitude(self_: Translator) -> Scalar  {
    return translator_translator_scalar_product(self_, translator_reversal(self_));
}

fn translator_magnitude(self_: Translator) -> Scalar  {
    return Scalar(sqrt(translator_squared_magnitude(self_).g0));
}

fn translator_scale(self_: Translator, other: f32) -> Translator  {
    return translator_scalar_geometric_product(self_, Scalar(other));
}

fn translator_signum(self_: Translator) -> Translator  {
    return translator_scalar_geometric_product(self_, Scalar(1.0 / translator_magnitude(self_).g0));
}

fn translator_inverse(self_: Translator) -> Translator  {
    return translator_scalar_geometric_product(translator_reversal(self_), Scalar(1.0 / translator_squared_magnitude(self_).g0));
}

fn motor_zero() -> Motor  {
    return Motor(vec4<f32>(0.0), vec4<f32>(0.0));
}

fn motor_one() -> Motor  {
    return Motor(vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(0.0));
}

fn motor_neg(self_: Motor) -> Motor  {
    return Motor(self_.g0 * vec4<f32>(-1.0), self_.g1 * vec4<f32>(-1.0));
}

fn motor_automorphism(self_: Motor) -> Motor  {
    return Motor(self_.g0, self_.g1);
}

fn motor_reversal(self_: Motor) -> Motor  {
    return Motor(self_.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0), self_.g1 * vec4<f32>(1.0, -1.0, -1.0, -1.0));
}

fn motor_conjugation(self_: Motor) -> Motor  {
    return Motor(self_.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0), self_.g1 * vec4<f32>(1.0, -1.0, -1.0, -1.0));
}

fn motor_dual(self_: Motor) -> Motor  {
    return Motor(self_.g1, self_.g0);
}

fn motor_scalar_into(self_: Motor) -> Scalar  {
    return Scalar(self_.g0.x);
}

fn motor_scalar_add(self_: Motor, other: Scalar) -> Motor  {
    return Motor(self_.g0 + vec4<f32>(other.g0) * vec4<f32>(1.0, 0.0, 0.0, 0.0), self_.g1);
}

fn motor_scalar_sub(self_: Motor, other: Scalar) -> Motor  {
    return Motor(self_.g0 - vec4<f32>(other.g0) * vec4<f32>(1.0, 0.0, 0.0, 0.0), self_.g1);
}

fn motor_scalar_geometric_product(self_: Motor, other: Scalar) -> Motor  {
    return Motor(self_.g0 * vec4<f32>(other.g0), self_.g1 * vec4<f32>(other.g0));
}

fn motor_scalar_regressive_product(self_: Motor, other: Scalar) -> Scalar  {
    return Scalar(self_.g1.x * other.g0);
}

fn motor_scalar_outer_product(self_: Motor, other: Scalar) -> Motor  {
    return Motor(self_.g0 * vec4<f32>(other.g0), self_.g1 * vec4<f32>(other.g0));
}

fn motor_scalar_inner_product(self_: Motor, other: Scalar) -> Motor  {
    return Motor(self_.g0 * vec4<f32>(other.g0), self_.g1 * vec4<f32>(other.g0));
}

fn motor_scalar_left_contraction(self_: Motor, other: Scalar) -> Scalar  {
    return Scalar(self_.g0.x * other.g0);
}

fn motor_scalar_right_contraction(self_: Motor, other: Scalar) -> Motor  {
    return Motor(self_.g0 * vec4<f32>(other.g0), self_.g1 * vec4<f32>(other.g0));
}

fn motor_scalar_scalar_product(self_: Motor, other: Scalar) -> Scalar  {
    return Scalar(self_.g0.x * other.g0);
}

fn motor_multi_vector_add(self_: Motor, other: MultiVector) -> MultiVector  {
    return MultiVector(self_.g0 + other.g0, other.g1, other.g2, self_.g1 + other.g3);
}

fn motor_multi_vector_sub(self_: Motor, other: MultiVector) -> MultiVector  {
    return MultiVector(self_.g0 - other.g0, vec4<f32>(0.0) - other.g1, vec4<f32>(0.0) - other.g2, self_.g1 - other.g3);
}

fn motor_multi_vector_geometric_product(self_: Motor, other: MultiVector) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.y) * other.g0.yxwz * vec4<f32>(-1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g0.zwxy * vec4<f32>(-1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g0.w) * other.g0.wzyx * vec4<f32>(-1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g1.x) * other.g3 * vec4<f32>(-1.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g1.y) * other.g3.yxwz * vec4<f32>(1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g1.z) * other.g3.zwxy * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g1.w) * other.g3.wzyx * vec4<f32>(1.0, -1.0, 1.0, 1.0), vec4<f32>(self_.g0.x) * other.g1 + vec4<f32>(self_.g0.y) * other.g1.yxwz * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g1.zwxy * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g0.w) * other.g1.wzyx * vec4<f32>(1.0, 1.0, -1.0, -1.0) - vec4<f32>(self_.g1.x) * other.g2 + vec4<f32>(self_.g1.y) * other.g2.yxwz * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g1.z) * other.g2.zwxy * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g1.w) * other.g2.wzyx * vec4<f32>(1.0, 1.0, -1.0, -1.0), vec4<f32>(self_.g0.x) * other.g2 + vec4<f32>(self_.g0.y) * other.g2.yxwz * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g2.zwxy * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g0.w) * other.g2.wzyx * vec4<f32>(1.0, 1.0, -1.0, -1.0) + vec4<f32>(self_.g1.x) * other.g1 + vec4<f32>(self_.g1.y) * other.g1.yxwz * vec4<f32>(-1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g1.z) * other.g1.zwxy * vec4<f32>(-1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g1.w) * other.g1.wzyx * vec4<f32>(-1.0, -1.0, 1.0, 1.0), vec4<f32>(self_.g0.x) * other.g3 + vec4<f32>(self_.g0.y) * other.g3.yxwz * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g3.zwxy * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g0.w) * other.g3.wzyx * vec4<f32>(1.0, 1.0, -1.0, -1.0) + vec4<f32>(self_.g1.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g1.y) * other.g0.yxwz * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g1.z) * other.g0.zwxy * vec4<f32>(1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g1.w) * other.g0.wzyx * vec4<f32>(1.0, 1.0, -1.0, 1.0));
}

fn motor_multi_vector_regressive_product(self_: Motor, other: MultiVector) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0.y) * other.g3.yxyy * vec4<f32>(1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g0.z) * other.g3.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g3.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g1.x) * other.g0 + vec4<f32>(self_.g1.y) * vec4<f32>(other.g0.y) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g0.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g3.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(self_.g1.x) * other.g1 + vec4<f32>(self_.g1.z) * vec4<f32>(other.g1.z) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g1.w) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + self_.g1.yxxx * other.g1.yxxx * vec4<f32>(-1.0, 0.0, 0.0, 0.0), vec4<f32>(self_.g0.z) * other.g1.wwwy * vec4<f32>(0.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g0.w) * other.g1.zzyz * vec4<f32>(0.0, -1.0, 1.0, 0.0) + vec4<f32>(self_.g1.x) * other.g2 + vec4<f32>(self_.g1.z) * vec4<f32>(other.g2.x) * vec4<f32>(0.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g2.x) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g0.x, self_.g1.y, self_.g0.y, self_.g0.y) * vec4<f32>(other.g1.x, other.g2.x, other.g1.w, other.g1.z) * vec4<f32>(0.0, 1.0, -1.0, 1.0), vec4<f32>(self_.g1.x) * other.g3 + self_.g1 * vec4<f32>(other.g3.x) * vec4<f32>(0.0, 1.0, 1.0, 1.0));
}

fn motor_multi_vector_outer_product(self_: Motor, other: MultiVector) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0.x) * other.g0 + self_.g0 * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 1.0, 1.0), vec4<f32>(self_.g0.x) * other.g1 + vec4<f32>(self_.g1.y) * other.g2.wwwz * vec4<f32>(0.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g1.z) * other.g2.wwwy * vec4<f32>(0.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g1.w) * other.g2.zzyz * vec4<f32>(0.0, 1.0, -1.0, 0.0) + self_.g0 * vec4<f32>(other.g1.x) * vec4<f32>(0.0, -1.0, -1.0, -1.0), vec4<f32>(self_.g0.x) * other.g2 + vec4<f32>(self_.g0.z) * vec4<f32>(other.g2.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g2.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + self_.g0.yxxx * other.g2.yxxx * vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(self_.g0.x) * other.g3 + vec4<f32>(self_.g0.z) * vec4<f32>(other.g3.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g3.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.y) * other.g0.yxyy * vec4<f32>(1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * other.g0.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g1.w) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + self_.g0.yxxx * other.g3.yxxx * vec4<f32>(1.0, 0.0, 0.0, 0.0));
}

fn motor_multi_vector_inner_product(self_: Motor, other: MultiVector) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.z) * other.g0.zzxz * vec4<f32>(-1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g0.wwwx * vec4<f32>(-1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g1.x) * other.g3 * vec4<f32>(-1.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g1.y) * other.g3.yxyy * vec4<f32>(1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * other.g3.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g1.w) * other.g3.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + self_.g0.yyxx * other.g0.yxxx * vec4<f32>(-1.0, 1.0, 0.0, 0.0), vec4<f32>(self_.g0.x) * other.g1 + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g1.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) - vec4<f32>(self_.g1.x) * other.g2 + vec4<f32>(self_.g1.y) * vec4<f32>(other.g2.y) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g2.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g2.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + self_.g0.yxxx * other.g1.yxxx * vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(self_.g0.x) * other.g2 + vec4<f32>(self_.g0.z) * other.g2.wwxy * vec4<f32>(0.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g0.w) * other.g2.zzyx * vec4<f32>(0.0, 1.0, -1.0, -1.0) + vec4<f32>(self_.g1.x) * other.g1 + vec4<f32>(self_.g1.y) * other.g1.xxwz * vec4<f32>(0.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g1.z) * other.g1.wwxy * vec4<f32>(0.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g1.w) * other.g1.zzyx * vec4<f32>(0.0, -1.0, 1.0, 1.0) + self_.g0.xyyy * other.g2.xxwz * vec4<f32>(0.0, -1.0, 1.0, -1.0), vec4<f32>(self_.g0.x) * other.g3 + vec4<f32>(self_.g1.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + self_.g0 * vec4<f32>(other.g3.x) * vec4<f32>(0.0, -1.0, -1.0, -1.0));
}

fn motor_multi_vector_left_contraction(self_: Motor, other: MultiVector) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g3.x) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.y) * other.g3.yxyy * vec4<f32>(1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * other.g3.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g1.w) * other.g3.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + self_.g0.yxxx * other.g0.yxxx * vec4<f32>(-1.0, 0.0, 0.0, 0.0), vec4<f32>(self_.g0.x) * other.g1 + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g1.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + self_.g0.yxxx * other.g1.yxxx * vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(self_.g0.x) * other.g2 + vec4<f32>(self_.g1.y) * other.g1.wwwz * vec4<f32>(0.0, 0.0, -1.0, 1.0) + vec4<f32>(self_.g1.z) * other.g1.wwwy * vec4<f32>(0.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g1.w) * other.g1.zzyz * vec4<f32>(0.0, -1.0, 1.0, 0.0) + self_.g0 * vec4<f32>(other.g2.x) * vec4<f32>(0.0, -1.0, -1.0, -1.0), vec4<f32>(self_.g0.x) * other.g3 + self_.g0 * vec4<f32>(other.g3.x) * vec4<f32>(0.0, -1.0, -1.0, -1.0));
}

fn motor_multi_vector_right_contraction(self_: Motor, other: MultiVector) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0.y) * other.g0.yxyy * vec4<f32>(-1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g0.z) * other.g0.zzxz * vec4<f32>(-1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g0.wwwx * vec4<f32>(-1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g1.x) * other.g3 * vec4<f32>(-1.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g3.y) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g3.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g3.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(0.0) - vec4<f32>(self_.g1.x) * other.g2 + vec4<f32>(self_.g1.z) * vec4<f32>(other.g2.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g2.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + self_.g1.yxxx * other.g2.yxxx * vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(self_.g0.z) * other.g2.wwwy * vec4<f32>(0.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * other.g2.zzyz * vec4<f32>(0.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g1.x) * other.g1 + vec4<f32>(self_.g1.z) * vec4<f32>(other.g1.x) * vec4<f32>(0.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g1.x) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g0.x, self_.g1.y, self_.g0.y, self_.g0.y) * vec4<f32>(other.g2.x, other.g1.x, other.g2.w, other.g2.z) * vec4<f32>(0.0, 1.0, 1.0, -1.0), vec4<f32>(self_.g1.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + self_.g1 * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 1.0, 1.0));
}

fn motor_multi_vector_scalar_product(self_: Motor, other: MultiVector) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.x - self_.g0.y * other.g0.y - self_.g0.z * other.g0.z - self_.g0.w * other.g0.w - self_.g1.x * other.g3.x + self_.g1.y * other.g3.y + self_.g1.z * other.g3.z + self_.g1.w * other.g3.w);
}

fn motor_rotor_into(self_: Motor) -> Rotor  {
    return Rotor(self_.g0);
}

fn motor_rotor_add(self_: Motor, other: Rotor) -> Motor  {
    return Motor(self_.g0 + other.g0, self_.g1);
}

fn motor_rotor_sub(self_: Motor, other: Rotor) -> Motor  {
    return Motor(self_.g0 - other.g0, self_.g1);
}

fn motor_rotor_geometric_product(self_: Motor, other: Rotor) -> Motor  {
    return Motor(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.y) * other.g0.yxwz * vec4<f32>(-1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g0.zwxy * vec4<f32>(-1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g0.w) * other.g0.wzyx * vec4<f32>(-1.0, 1.0, -1.0, 1.0), vec4<f32>(self_.g1.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g1.y) * other.g0.yxwz * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g1.z) * other.g0.zwxy * vec4<f32>(1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g1.w) * other.g0.wzyx * vec4<f32>(1.0, 1.0, -1.0, 1.0));
}

fn motor_rotor_regressive_product(self_: Motor, other: Rotor) -> Rotor  {
    return Rotor(vec4<f32>(self_.g1.x) * other.g0 + vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g0.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + self_.g1.yxxx * other.g0.yxxx * vec4<f32>(1.0, 0.0, 0.0, 0.0));
}

fn motor_rotor_outer_product(self_: Motor, other: Rotor) -> Motor  {
    return Motor(vec4<f32>(self_.g0.x) * other.g0 + self_.g0 * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 1.0, 1.0), vec4<f32>(self_.g1.y) * other.g0.yxyy * vec4<f32>(1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * other.g0.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g1.w) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0));
}

fn motor_rotor_inner_product(self_: Motor, other: Rotor) -> Motor  {
    return Motor(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.z) * other.g0.zzxz * vec4<f32>(-1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g0.wwwx * vec4<f32>(-1.0, 0.0, 0.0, 1.0) + self_.g0.yyxx * other.g0.yxxx * vec4<f32>(-1.0, 1.0, 0.0, 0.0), vec4<f32>(self_.g1.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + self_.g1 * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 1.0, 1.0));
}

fn motor_rotor_left_contraction(self_: Motor, other: Rotor) -> Rotor  {
    return Rotor(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + self_.g0.yxxx * other.g0.yxxx * vec4<f32>(-1.0, 0.0, 0.0, 0.0));
}

fn motor_rotor_right_contraction(self_: Motor, other: Rotor) -> Motor  {
    return Motor(vec4<f32>(self_.g0.y) * other.g0.yxyy * vec4<f32>(-1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g0.z) * other.g0.zzxz * vec4<f32>(-1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g0.wwwx * vec4<f32>(-1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(self_.g1.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + self_.g1 * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 1.0, 1.0));
}

fn motor_rotor_scalar_product(self_: Motor, other: Rotor) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.x - self_.g0.y * other.g0.y - self_.g0.z * other.g0.z - self_.g0.w * other.g0.w);
}

fn motor_point_geometric_product(self_: Motor, other: Point) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.z) * other.g0.wwwy * vec4<f32>(0.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * other.g0.zzyz * vec4<f32>(0.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g1.z) * other.g0.zzxz * vec4<f32>(-1.0, 0.0, -1.0, 0.0) + vec4<f32>(self_.g1.w) * other.g0.wwwx * vec4<f32>(-1.0, 0.0, 0.0, -1.0) + vec4<f32>(self_.g1.y, self_.g1.y, self_.g0.y, self_.g0.y) * other.g0.yxwz * vec4<f32>(-1.0, -1.0, 1.0, -1.0), vec4<f32>(self_.g0.z) * other.g0.zzxz * vec4<f32>(1.0, 0.0, -1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, -1.0) + vec4<f32>(self_.g1.x) * other.g0 * vec4<f32>(-1.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g1.z) * other.g0.wwwy * vec4<f32>(0.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g1.w) * other.g0.zzyz * vec4<f32>(0.0, -1.0, 1.0, 0.0) + vec4<f32>(self_.g0.y, self_.g0.y, self_.g1.y, self_.g1.y) * other.g0.yxwz * vec4<f32>(1.0, -1.0, -1.0, 1.0));
}

fn motor_point_regressive_product(self_: Motor, other: Point) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g1.x) * other.g0, vec4<f32>(self_.g0.z) * other.g0.wwwy * vec4<f32>(0.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g0.w) * other.g0.zzyz * vec4<f32>(0.0, -1.0, 1.0, 0.0) + vec4<f32>(self_.g1.z) * other.g0.zzxz * vec4<f32>(-1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g1.w) * other.g0.wwwx * vec4<f32>(-1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g1.y, self_.g1.y, self_.g0.y, self_.g0.y) * other.g0.yxwz * vec4<f32>(-1.0, 1.0, -1.0, 1.0));
}

fn motor_point_outer_product(self_: Motor, other: Point) -> Point  {
    return Point(vec4<f32>(self_.g0.x) * other.g0);
}

fn motor_point_inner_product(self_: Motor, other: Point) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0.x) * other.g0, vec4<f32>(self_.g0.z) * other.g0.zzxz * vec4<f32>(1.0, 0.0, -1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, -1.0) + vec4<f32>(self_.g1.x) * other.g0 * vec4<f32>(-1.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g1.z) * other.g0.wwwy * vec4<f32>(0.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g1.w) * other.g0.zzyz * vec4<f32>(0.0, -1.0, 1.0, 0.0) + vec4<f32>(self_.g0.y, self_.g0.y, self_.g1.y, self_.g1.y) * other.g0.yxwz * vec4<f32>(1.0, -1.0, -1.0, 1.0));
}

fn motor_point_left_contraction(self_: Motor, other: Point) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0.x) * other.g0, vec4<f32>(self_.g0.z) * other.g0.zzxz * vec4<f32>(1.0, 0.0, -1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, -1.0) + vec4<f32>(self_.g1.z) * other.g0.wwwy * vec4<f32>(0.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g1.w) * other.g0.zzyz * vec4<f32>(0.0, -1.0, 1.0, 0.0) + vec4<f32>(self_.g0.y, self_.g0.y, self_.g1.y, self_.g1.y) * other.g0.yxwz * vec4<f32>(1.0, -1.0, -1.0, 1.0));
}

fn motor_point_right_contraction(self_: Motor, other: Point) -> Plane  {
    return Plane(vec4<f32>(self_.g1.x) * other.g0 * vec4<f32>(-1.0, 1.0, 1.0, 1.0));
}

fn motor_ideal_point_into(self_: Motor) -> IdealPoint  {
    return IdealPoint(vec3<f32>(self_.g1.y, self_.g1.z, self_.g1.w));
}

fn motor_ideal_point_add(self_: Motor, other: IdealPoint) -> Motor  {
    return Motor(self_.g0, self_.g1 + vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0));
}

fn motor_ideal_point_sub(self_: Motor, other: IdealPoint) -> Motor  {
    return Motor(self_.g0, self_.g1 - vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0));
}

fn motor_ideal_point_geometric_product(self_: Motor, other: IdealPoint) -> Motor  {
    return Motor(vec4<f32>(self_.g1.y) * vec4<f32>(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4<f32>(1.0, 0.0, -1.0, 1.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4<f32>(1.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4<f32>(1.0, -1.0, 1.0, 0.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0), vec4<f32>(self_.g0.y) * vec4<f32>(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4<f32>(1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4<f32>(1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4<f32>(1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0));
}

fn motor_ideal_point_regressive_product(self_: Motor, other: IdealPoint) -> Translator  {
    return Translator(vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.y) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.y, self_.g1.x, self_.g1.x, self_.g1.x) * vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z));
}

fn motor_ideal_point_left_contraction(self_: Motor, other: IdealPoint) -> Translator  {
    return Translator(vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.y) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g0.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.y, self_.g0.x, self_.g0.x, self_.g0.x) * vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z));
}

fn motor_ideal_point_right_contraction(self_: Motor, other: IdealPoint) -> Rotor  {
    return Rotor(vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.y) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g0.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + self_.g1.yxxx * vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z));
}

fn motor_ideal_point_scalar_product(self_: Motor, other: IdealPoint) -> Scalar  {
    return Scalar(self_.g1.y * other.g0.x + self_.g1.z * other.g0.y + self_.g1.w * other.g0.z);
}

fn motor_plane_geometric_product(self_: Motor, other: Plane) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0.z) * other.g0.zzxz * vec4<f32>(1.0, 0.0, -1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, -1.0) + vec4<f32>(self_.g1.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g1.z) * other.g0.wwwy * vec4<f32>(0.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g1.w) * other.g0.zzyz * vec4<f32>(0.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g0.y, self_.g0.y, self_.g1.y, self_.g1.y) * other.g0.yxwz * vec4<f32>(1.0, -1.0, 1.0, -1.0), vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.z) * other.g0.wwwy * vec4<f32>(0.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * other.g0.zzyz * vec4<f32>(0.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g1.z) * other.g0.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g1.w) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g1.y, self_.g1.y, self_.g0.y, self_.g0.y) * other.g0.yxwz * vec4<f32>(1.0, 1.0, 1.0, -1.0));
}

fn motor_plane_regressive_product(self_: Motor, other: Plane) -> Plane  {
    return Plane(vec4<f32>(self_.g1.x) * other.g0);
}

fn motor_plane_outer_product(self_: Motor, other: Plane) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0.z) * other.g0.zzxz * vec4<f32>(1.0, 0.0, -1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, -1.0) + vec4<f32>(self_.g1.z) * other.g0.wwwy * vec4<f32>(0.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g1.w) * other.g0.zzyz * vec4<f32>(0.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g0.y, self_.g0.y, self_.g1.y, self_.g1.y) * other.g0.yxwz * vec4<f32>(1.0, -1.0, 1.0, -1.0), vec4<f32>(self_.g0.x) * other.g0);
}

fn motor_plane_inner_product(self_: Motor, other: Plane) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g1.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0), vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.z) * other.g0.wwwy * vec4<f32>(0.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * other.g0.zzyz * vec4<f32>(0.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g1.z) * other.g0.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g1.w) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g1.y, self_.g1.y, self_.g0.y, self_.g0.y) * other.g0.yxwz * vec4<f32>(1.0, 1.0, 1.0, -1.0));
}

fn motor_plane_left_contraction(self_: Motor, other: Plane) -> Plane  {
    return Plane(vec4<f32>(self_.g0.x) * other.g0);
}

fn motor_plane_right_contraction(self_: Motor, other: Plane) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g1.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0), vec4<f32>(self_.g0.z) * other.g0.wwwy * vec4<f32>(0.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * other.g0.zzyz * vec4<f32>(0.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g1.z) * other.g0.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g1.w) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g1.y, self_.g1.y, self_.g0.y, self_.g0.y) * other.g0.yxwz * vec4<f32>(1.0, 1.0, 1.0, -1.0));
}

fn motor_line_into(self_: Motor) -> Line  {
    return Line(vec3<f32>(self_.g1.y, self_.g1.z, self_.g1.w), vec3<f32>(self_.g0.y, self_.g0.z, self_.g0.w));
}

fn motor_line_add(self_: Motor, other: Line) -> Motor  {
    return Motor(self_.g0 + vec4<f32>(other.g0.x, other.g1.x, other.g1.y, other.g1.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0), self_.g1 + vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0));
}

fn motor_line_sub(self_: Motor, other: Line) -> Motor  {
    return Motor(self_.g0 - vec4<f32>(other.g0.x, other.g1.x, other.g1.y, other.g1.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0), self_.g1 - vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0));
}

fn motor_line_geometric_product(self_: Motor, other: Line) -> Motor  {
    return Motor(vec4<f32>(self_.g0.y) * vec4<f32>(other.g1.x, other.g1.x, other.g1.z, other.g1.y) * vec4<f32>(-1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.y, other.g1.z, other.g1.y, other.g1.x) * vec4<f32>(-1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g1.z, other.g1.y, other.g1.x, other.g1.z) * vec4<f32>(-1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4<f32>(1.0, 0.0, -1.0, 1.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4<f32>(1.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4<f32>(1.0, -1.0, 1.0, 0.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0), vec4<f32>(self_.g0.y) * vec4<f32>(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4<f32>(1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4<f32>(1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4<f32>(1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4<f32>(0.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g1.x, other.g1.x, other.g1.z, other.g1.y) * vec4<f32>(1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g1.y, other.g1.z, other.g1.y, other.g1.x) * vec4<f32>(1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g1.z, other.g1.y, other.g1.x, other.g1.z) * vec4<f32>(1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0));
}

fn motor_line_scalar_product(self_: Motor, other: Line) -> Scalar  {
    return Scalar(0.0 - self_.g0.y * other.g1.x - self_.g0.z * other.g1.y - self_.g0.w * other.g1.z + self_.g1.y * other.g0.x + self_.g1.z * other.g0.y + self_.g1.w * other.g0.z);
}

fn motor_translator_into(self_: Motor) -> Translator  {
    return Translator(vec4<f32>(self_.g0.x, self_.g1.y, self_.g1.z, self_.g1.w));
}

fn motor_translator_add(self_: Motor, other: Translator) -> Motor  {
    return Motor(self_.g0 + vec4<f32>(other.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0), self_.g1 + other.g0 * vec4<f32>(0.0, 1.0, 1.0, 1.0));
}

fn motor_translator_sub(self_: Motor, other: Translator) -> Motor  {
    return Motor(self_.g0 - vec4<f32>(other.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0), self_.g1 - other.g0 * vec4<f32>(0.0, 1.0, 1.0, 1.0));
}

fn motor_translator_geometric_product(self_: Motor, other: Translator) -> Motor  {
    return Motor(vec4<f32>(self_.g1.x) * other.g0.yyzw * vec4<f32>(0.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g1.y) * other.g0.yywz * vec4<f32>(1.0, 0.0, -1.0, 1.0) + vec4<f32>(self_.g1.z) * other.g0.zwzy * vec4<f32>(1.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g1.w) * other.g0.wzyw * vec4<f32>(1.0, -1.0, 1.0, 0.0) + self_.g0 * vec4<f32>(other.g0.x), vec4<f32>(self_.g0.y) * other.g0.yywz * vec4<f32>(1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g0.zwzy * vec4<f32>(1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * other.g0.wzyw * vec4<f32>(1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g1.x, self_.g0.x, self_.g0.x, self_.g0.x) * other.g0);
}

fn motor_translator_regressive_product(self_: Motor, other: Translator) -> Translator  {
    return Translator(vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.x) * other.g0 + self_.g0.yxxx * other.g0.yxxx * vec4<f32>(1.0, 0.0, 0.0, 0.0));
}

fn motor_translator_outer_product(self_: Motor, other: Translator) -> Motor  {
    return Motor(self_.g0 * vec4<f32>(other.g0.x), vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + self_.g0.yxxx * other.g0.yyzw);
}

fn motor_translator_inner_product(self_: Motor, other: Translator) -> Motor  {
    return Motor(vec4<f32>(self_.g1.x) * other.g0.yyzw * vec4<f32>(0.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g0.y) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g0.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + self_.g0 * vec4<f32>(other.g0.x), vec4<f32>(self_.g1.y) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g1.x, self_.g0.x, self_.g0.x, self_.g0.x) * other.g0);
}

fn motor_translator_left_contraction(self_: Motor, other: Translator) -> Translator  {
    return Translator(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g0.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + self_.g1.yxxx * other.g0.yxxx * vec4<f32>(1.0, 0.0, 0.0, 0.0));
}

fn motor_translator_right_contraction(self_: Motor, other: Translator) -> Motor  {
    return Motor(vec4<f32>(self_.g1.x) * other.g0.yyzw * vec4<f32>(0.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g0.y) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g0.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + self_.g0 * vec4<f32>(other.g0.x), self_.g1 * vec4<f32>(other.g0.x));
}

fn motor_translator_scalar_product(self_: Motor, other: Translator) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.x + self_.g1.y * other.g0.y + self_.g1.z * other.g0.z + self_.g1.w * other.g0.w);
}

fn motor_motor_add(self_: Motor, other: Motor) -> Motor  {
    return Motor(self_.g0 + other.g0, self_.g1 + other.g1);
}

fn motor_motor_sub(self_: Motor, other: Motor) -> Motor  {
    return Motor(self_.g0 - other.g0, self_.g1 - other.g1);
}

fn motor_motor_mul(self_: Motor, other: Motor) -> Motor  {
    return Motor(self_.g0 * other.g0, self_.g1 * other.g1);
}

fn motor_motor_div(self_: Motor, other: Motor) -> Motor  {
    return Motor(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0) / vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0), vec4<f32>(self_.g1.x, self_.g1.y, self_.g1.z, self_.g1.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0) / vec4<f32>(other.g1.x, other.g1.y, other.g1.z, other.g1.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0));
}

fn motor_motor_geometric_product(self_: Motor, other: Motor) -> Motor  {
    return Motor(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.y) * other.g0.yxwz * vec4<f32>(-1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g0.zwxy * vec4<f32>(-1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g0.w) * other.g0.wzyx * vec4<f32>(-1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g1.x) * other.g1 * vec4<f32>(-1.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g1.y) * other.g1.yxwz * vec4<f32>(1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g1.z) * other.g1.zwxy * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g1.w) * other.g1.wzyx * vec4<f32>(1.0, -1.0, 1.0, 1.0), vec4<f32>(self_.g0.x) * other.g1 + vec4<f32>(self_.g0.y) * other.g1.yxwz * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g1.zwxy * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g0.w) * other.g1.wzyx * vec4<f32>(1.0, 1.0, -1.0, -1.0) + vec4<f32>(self_.g1.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g1.y) * other.g0.yxwz * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g1.z) * other.g0.zwxy * vec4<f32>(1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g1.w) * other.g0.wzyx * vec4<f32>(1.0, 1.0, -1.0, 1.0));
}

fn motor_motor_regressive_product(self_: Motor, other: Motor) -> Motor  {
    return Motor(vec4<f32>(self_.g0.y) * other.g1.yxyy * vec4<f32>(1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g0.z) * other.g1.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g1.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g1.x) * other.g0 + vec4<f32>(self_.g1.y) * vec4<f32>(other.g0.y) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g0.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g1.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(self_.g1.x) * other.g1 + self_.g1 * vec4<f32>(other.g1.x) * vec4<f32>(0.0, 1.0, 1.0, 1.0));
}

fn motor_motor_outer_product(self_: Motor, other: Motor) -> Motor  {
    return Motor(vec4<f32>(self_.g0.x) * other.g0 + self_.g0 * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 1.0, 1.0), vec4<f32>(self_.g0.x) * other.g1 + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g1.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.y) * other.g0.yxyy * vec4<f32>(1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * other.g0.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g1.w) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + self_.g0.yxxx * other.g1.yxxx * vec4<f32>(1.0, 0.0, 0.0, 0.0));
}

fn motor_motor_inner_product(self_: Motor, other: Motor) -> Motor  {
    return Motor(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.z) * other.g0.zzxz * vec4<f32>(-1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g0.wwwx * vec4<f32>(-1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g1.x) * other.g1 * vec4<f32>(-1.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g1.y) * other.g1.yxyy * vec4<f32>(1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * other.g1.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g1.w) * other.g1.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + self_.g0.yyxx * other.g0.yxxx * vec4<f32>(-1.0, 1.0, 0.0, 0.0), vec4<f32>(self_.g0.x) * other.g1 + vec4<f32>(self_.g1.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + self_.g0 * vec4<f32>(other.g1.x) * vec4<f32>(0.0, -1.0, -1.0, -1.0));
}

fn motor_motor_left_contraction(self_: Motor, other: Motor) -> Motor  {
    return Motor(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g1.x) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.y) * other.g1.yxyy * vec4<f32>(1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * other.g1.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g1.w) * other.g1.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + self_.g0.yxxx * other.g0.yxxx * vec4<f32>(-1.0, 0.0, 0.0, 0.0), vec4<f32>(self_.g0.x) * other.g1 + self_.g0 * vec4<f32>(other.g1.x) * vec4<f32>(0.0, -1.0, -1.0, -1.0));
}

fn motor_motor_right_contraction(self_: Motor, other: Motor) -> Motor  {
    return Motor(vec4<f32>(self_.g0.y) * other.g0.yxyy * vec4<f32>(-1.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g0.z) * other.g0.zzxz * vec4<f32>(-1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g0.wwwx * vec4<f32>(-1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g1.x) * other.g1 * vec4<f32>(-1.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g1.y) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g1.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g1.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(self_.g1.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + self_.g1 * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 1.0, 1.0));
}

fn motor_motor_scalar_product(self_: Motor, other: Motor) -> Scalar  {
    return Scalar(self_.g0.x * other.g0.x - self_.g0.y * other.g0.y - self_.g0.z * other.g0.z - self_.g0.w * other.g0.w - self_.g1.x * other.g1.x + self_.g1.y * other.g1.y + self_.g1.z * other.g1.z + self_.g1.w * other.g1.w);
}

fn motor_point_and_plane_add(self_: Motor, other: PointAndPlane) -> MultiVector  {
    return MultiVector(self_.g0, vec4<f32>(other.g1.x, other.g0.y, other.g0.z, other.g0.w), vec4<f32>(other.g0.x, other.g1.y, other.g1.z, other.g1.w), self_.g1);
}

fn motor_point_and_plane_sub(self_: Motor, other: PointAndPlane) -> MultiVector  {
    return MultiVector(self_.g0, vec4<f32>(0.0) - vec4<f32>(other.g1.x, other.g0.y, other.g0.z, other.g0.w), vec4<f32>(0.0) - vec4<f32>(other.g0.x, other.g1.y, other.g1.z, other.g1.w), self_.g1);
}

fn motor_point_and_plane_geometric_product(self_: Motor, other: PointAndPlane) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.y) * vec4<f32>(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4<f32>(1.0, 1.0, -1.0, -1.0) + vec4<f32>(self_.g1.x) * other.g1 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g0.y, other.g0.x, other.g1.w, other.g1.z) * vec4<f32>(-1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.z, other.g1.w, other.g0.x, other.g1.y) * vec4<f32>(-1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4<f32>(-1.0, 1.0, -1.0, -1.0), vec4<f32>(self_.g0.x) * other.g1 + vec4<f32>(self_.g0.y) * vec4<f32>(other.g0.y, other.g0.x, other.g1.w, other.g1.z) * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z, other.g1.w, other.g0.x, other.g1.y) * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4<f32>(1.0, 1.0, -1.0, -1.0) + vec4<f32>(self_.g1.x) * other.g0 * vec4<f32>(-1.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4<f32>(1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4<f32>(1.0, -1.0, 1.0, 1.0));
}

fn motor_point_and_plane_regressive_product(self_: Motor, other: PointAndPlane) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g1.x) * other.g0, vec4<f32>(self_.g0.z) * other.g0.wwwy * vec4<f32>(0.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g0.w) * other.g0.zzyz * vec4<f32>(0.0, -1.0, 1.0, 0.0) + vec4<f32>(self_.g1.x) * other.g1 + vec4<f32>(self_.g1.z) * other.g0.zzxz * vec4<f32>(-1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g1.w) * other.g0.wwwx * vec4<f32>(-1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g1.y, self_.g1.y, self_.g0.y, self_.g0.y) * other.g0.yxwz * vec4<f32>(-1.0, 1.0, -1.0, 1.0));
}

fn motor_point_and_plane_outer_product(self_: Motor, other: PointAndPlane) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.z) * other.g1.zzxz * vec4<f32>(1.0, 0.0, -1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g1.wwwx * vec4<f32>(1.0, 0.0, 0.0, -1.0) + vec4<f32>(self_.g1.z) * other.g1.wwwy * vec4<f32>(0.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g1.w) * other.g1.zzyz * vec4<f32>(0.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g0.y, self_.g0.y, self_.g1.y, self_.g1.y) * other.g1.yxwz * vec4<f32>(1.0, -1.0, 1.0, -1.0), vec4<f32>(self_.g0.x) * other.g1);
}

fn motor_point_and_plane_inner_product(self_: Motor, other: PointAndPlane) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g1.x) * other.g1 * vec4<f32>(1.0, -1.0, -1.0, -1.0), vec4<f32>(self_.g0.x) * other.g1 + vec4<f32>(self_.g0.y) * vec4<f32>(other.g0.y, other.g0.x, other.g1.w, other.g1.z) * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z, other.g1.w, other.g0.x, other.g1.y) * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4<f32>(1.0, 1.0, -1.0, -1.0) + vec4<f32>(self_.g1.x) * other.g0 * vec4<f32>(-1.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4<f32>(1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4<f32>(1.0, -1.0, 1.0, 1.0));
}

fn motor_point_and_plane_left_contraction(self_: Motor, other: PointAndPlane) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0.x) * other.g0, vec4<f32>(self_.g0.x) * other.g1 + vec4<f32>(self_.g0.z) * other.g0.zzxz * vec4<f32>(1.0, 0.0, -1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, -1.0) + vec4<f32>(self_.g1.z) * other.g0.wwwy * vec4<f32>(0.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g1.w) * other.g0.zzyz * vec4<f32>(0.0, -1.0, 1.0, 0.0) + vec4<f32>(self_.g0.y, self_.g0.y, self_.g1.y, self_.g1.y) * other.g0.yxwz * vec4<f32>(1.0, -1.0, -1.0, 1.0));
}

fn motor_point_and_plane_right_contraction(self_: Motor, other: PointAndPlane) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g1.x) * other.g1 * vec4<f32>(1.0, -1.0, -1.0, -1.0), vec4<f32>(self_.g0.z) * other.g1.wwwy * vec4<f32>(0.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * other.g1.zzyz * vec4<f32>(0.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g1.x) * other.g0 * vec4<f32>(-1.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g1.z) * other.g1.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g1.w) * other.g1.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g1.y, self_.g1.y, self_.g0.y, self_.g0.y) * other.g1.yxwz * vec4<f32>(1.0, 1.0, 1.0, -1.0));
}

fn motor_squared_magnitude(self_: Motor) -> Scalar  {
    return motor_motor_scalar_product(self_, motor_reversal(self_));
}

fn motor_magnitude(self_: Motor) -> Scalar  {
    return Scalar(sqrt(motor_squared_magnitude(self_).g0));
}

fn motor_scale(self_: Motor, other: f32) -> Motor  {
    return motor_scalar_geometric_product(self_, Scalar(other));
}

fn motor_signum(self_: Motor) -> Motor  {
    return motor_scalar_geometric_product(self_, Scalar(1.0 / motor_magnitude(self_).g0));
}

fn motor_inverse(self_: Motor) -> Motor  {
    return motor_scalar_geometric_product(motor_reversal(self_), Scalar(1.0 / motor_squared_magnitude(self_).g0));
}

fn point_and_plane_zero() -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(0.0), vec4<f32>(0.0));
}

fn point_and_plane_one() -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(0.0), vec4<f32>(0.0));
}

fn point_and_plane_neg(self_: PointAndPlane) -> PointAndPlane  {
    return PointAndPlane(self_.g0 * vec4<f32>(-1.0), self_.g1 * vec4<f32>(-1.0));
}

fn point_and_plane_automorphism(self_: PointAndPlane) -> PointAndPlane  {
    return PointAndPlane(self_.g0 * vec4<f32>(-1.0), self_.g1 * vec4<f32>(-1.0));
}

fn point_and_plane_reversal(self_: PointAndPlane) -> PointAndPlane  {
    return PointAndPlane(self_.g0 * vec4<f32>(-1.0), self_.g1);
}

fn point_and_plane_conjugation(self_: PointAndPlane) -> PointAndPlane  {
    return PointAndPlane(self_.g0, self_.g1 * vec4<f32>(-1.0));
}

fn point_and_plane_dual(self_: PointAndPlane) -> PointAndPlane  {
    return PointAndPlane(self_.g1, self_.g0 * vec4<f32>(-1.0));
}

fn point_and_plane_scalar_geometric_product(self_: PointAndPlane, other: Scalar) -> PointAndPlane  {
    return PointAndPlane(self_.g0 * vec4<f32>(other.g0), self_.g1 * vec4<f32>(other.g0));
}

fn point_and_plane_scalar_outer_product(self_: PointAndPlane, other: Scalar) -> PointAndPlane  {
    return PointAndPlane(self_.g0 * vec4<f32>(other.g0), self_.g1 * vec4<f32>(other.g0));
}

fn point_and_plane_scalar_inner_product(self_: PointAndPlane, other: Scalar) -> PointAndPlane  {
    return PointAndPlane(self_.g0 * vec4<f32>(other.g0), self_.g1 * vec4<f32>(other.g0));
}

fn point_and_plane_scalar_right_contraction(self_: PointAndPlane, other: Scalar) -> PointAndPlane  {
    return PointAndPlane(self_.g0 * vec4<f32>(other.g0), self_.g1 * vec4<f32>(other.g0));
}

fn point_and_plane_multi_vector_add(self_: PointAndPlane, other: MultiVector) -> MultiVector  {
    return MultiVector(other.g0, vec4<f32>(self_.g1.x, self_.g0.y, self_.g0.z, self_.g0.w) + other.g1, vec4<f32>(self_.g0.x, self_.g1.y, self_.g1.z, self_.g1.w) + other.g2, other.g3);
}

fn point_and_plane_multi_vector_sub(self_: PointAndPlane, other: MultiVector) -> MultiVector  {
    return MultiVector(vec4<f32>(0.0) - other.g0, vec4<f32>(self_.g1.x, self_.g0.y, self_.g0.z, self_.g0.w) - other.g1, vec4<f32>(self_.g0.x, self_.g1.y, self_.g1.z, self_.g1.w) - other.g2, vec4<f32>(0.0) - other.g3);
}

fn point_and_plane_multi_vector_geometric_product(self_: PointAndPlane, other: MultiVector) -> MultiVector  {
    return MultiVector(vec4<f32>(self_.g0.x) * other.g2 * vec4<f32>(-1.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g0.y) * other.g1.yxwz * vec4<f32>(1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g0.z) * other.g1.zwxy * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.w) * other.g1.wzyx * vec4<f32>(1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g1.x) * other.g1 * vec4<f32>(-1.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g1.y) * other.g2.yxwz * vec4<f32>(1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g1.z) * other.g2.zwxy * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g1.w) * other.g2.wzyx * vec4<f32>(1.0, -1.0, 1.0, 1.0), vec4<f32>(self_.g0.x) * other.g3 + vec4<f32>(self_.g0.y) * other.g0.yxwz * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g0.zwxy * vec4<f32>(1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g0.w) * other.g0.wzyx * vec4<f32>(1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g1.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g1.y) * other.g3.yxwz * vec4<f32>(-1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g1.z) * other.g3.zwxy * vec4<f32>(-1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g1.w) * other.g3.wzyx * vec4<f32>(-1.0, -1.0, 1.0, 1.0), vec4<f32>(self_.g0.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g0.y) * other.g3.yxwz * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * other.g3.zwxy * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g0.w) * other.g3.wzyx * vec4<f32>(1.0, 1.0, -1.0, -1.0) - vec4<f32>(self_.g1.x) * other.g3 + vec4<f32>(self_.g1.y) * other.g0.yxwz * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g1.z) * other.g0.zwxy * vec4<f32>(1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g1.w) * other.g0.wzyx * vec4<f32>(1.0, 1.0, -1.0, 1.0), vec4<f32>(0.0) - vec4<f32>(self_.g0.x) * other.g1 + vec4<f32>(self_.g0.y) * other.g2.yxwz * vec4<f32>(-1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g0.z) * other.g2.zwxy * vec4<f32>(-1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.w) * other.g2.wzyx * vec4<f32>(-1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g1.x) * other.g2 + vec4<f32>(self_.g1.y) * other.g1.yxwz * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g1.z) * other.g1.zwxy * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g1.w) * other.g1.wzyx * vec4<f32>(1.0, 1.0, -1.0, -1.0));
}

fn point_and_plane_multi_vector_scalar_product(self_: PointAndPlane, other: MultiVector) -> Scalar  {
    return Scalar(0.0 - self_.g0.x * other.g2.x + self_.g0.y * other.g1.y + self_.g0.z * other.g1.z + self_.g0.w * other.g1.w - self_.g1.x * other.g1.x + self_.g1.y * other.g2.y + self_.g1.z * other.g2.z + self_.g1.w * other.g2.w);
}

fn point_and_plane_rotor_geometric_product(self_: PointAndPlane, other: Rotor) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0.z) * other.g0.wwxy * vec4<f32>(0.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g0.w) * other.g0.zzyx * vec4<f32>(0.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g1.x) * other.g0.yyzw * vec4<f32>(0.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g0.y) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g0.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + self_.g0.xyyy * other.g0.xxwz * vec4<f32>(1.0, 1.0, 1.0, -1.0), vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.y) * other.g0.xxwz * vec4<f32>(0.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g1.z) * other.g0.wwxy * vec4<f32>(0.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g1.w) * other.g0.zzyx * vec4<f32>(0.0, 1.0, -1.0, 1.0) + self_.g0.yxxx * other.g0.yyzw * vec4<f32>(1.0, -1.0, -1.0, -1.0));
}

fn point_and_plane_rotor_outer_product(self_: PointAndPlane, other: Rotor) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g1.x) * other.g0.yyzw * vec4<f32>(0.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g0.y) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g0.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + self_.g0 * vec4<f32>(other.g0.x), self_.g1 * vec4<f32>(other.g0.x));
}

fn point_and_plane_rotor_inner_product(self_: PointAndPlane, other: Rotor) -> PointAndPlane  {
    return PointAndPlane(self_.g0 * vec4<f32>(other.g0.x), vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.y) * other.g0.xxwz * vec4<f32>(0.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g1.z) * other.g0.wwxy * vec4<f32>(0.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g1.w) * other.g0.zzyx * vec4<f32>(0.0, 1.0, -1.0, 1.0) + self_.g0.yxxx * other.g0.yyzw * vec4<f32>(1.0, -1.0, -1.0, -1.0));
}

fn point_and_plane_rotor_right_contraction(self_: PointAndPlane, other: Rotor) -> PointAndPlane  {
    return PointAndPlane(self_.g0 * vec4<f32>(other.g0.x), vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g0.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + self_.g0.yxxx * other.g0.yyzw * vec4<f32>(1.0, -1.0, -1.0, -1.0));
}

fn point_and_plane_point_into(self_: PointAndPlane) -> Point  {
    return Point(self_.g0);
}

fn point_and_plane_point_add(self_: PointAndPlane, other: Point) -> PointAndPlane  {
    return PointAndPlane(self_.g0 + other.g0, self_.g1);
}

fn point_and_plane_point_sub(self_: PointAndPlane, other: Point) -> PointAndPlane  {
    return PointAndPlane(self_.g0 - other.g0, self_.g1);
}

fn point_and_plane_point_geometric_product(self_: PointAndPlane, other: Point) -> Motor  {
    return Motor(vec4<f32>(self_.g0.y) * other.g0.yywz * vec4<f32>(1.0, 0.0, -1.0, 1.0) + vec4<f32>(self_.g0.z) * other.g0.zwzy * vec4<f32>(1.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g0.w) * other.g0.wzyw * vec4<f32>(1.0, -1.0, 1.0, 0.0) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g0.x, self_.g1.x, self_.g1.x, self_.g1.x) * other.g0 * vec4<f32>(-1.0, 1.0, 1.0, 1.0), vec4<f32>(self_.g0.y) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g1.y) * other.g0.yywz * vec4<f32>(1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g1.z) * other.g0.zwzy * vec4<f32>(1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g1.w) * other.g0.wzyw * vec4<f32>(1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g1.x, self_.g0.x, self_.g0.x, self_.g0.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0));
}

fn point_and_plane_point_right_contraction(self_: PointAndPlane, other: Point) -> Scalar  {
    return Scalar(0.0 - self_.g0.x * other.g0.x + self_.g0.y * other.g0.y + self_.g0.z * other.g0.z + self_.g0.w * other.g0.w);
}

fn point_and_plane_point_scalar_product(self_: PointAndPlane, other: Point) -> Scalar  {
    return Scalar(0.0 - self_.g0.x * other.g0.x + self_.g0.y * other.g0.y + self_.g0.z * other.g0.z + self_.g0.w * other.g0.w);
}

fn point_and_plane_ideal_point_geometric_product(self_: PointAndPlane, other: IdealPoint) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.y) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.z) * vec4<f32>(1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g0.z, other.g0.z, other.g0.z, other.g0.y) * vec4<f32>(0.0, 0.0, -1.0, 1.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.z, other.g0.z, other.g0.z, other.g0.x) * vec4<f32>(0.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g0.y, other.g0.y, other.g0.x, other.g0.y) * vec4<f32>(0.0, -1.0, 1.0, 0.0) + self_.g0.yxxx * vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z), vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z, other.g0.z, other.g0.z, other.g0.x) * vec4<f32>(0.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.y, other.g0.y, other.g0.x, other.g0.y) * vec4<f32>(0.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(0.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.y) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g0.z) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.y, self_.g0.x, self_.g0.y, self_.g0.y) * vec4<f32>(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4<f32>(-1.0, 0.0, 1.0, -1.0));
}

fn point_and_plane_ideal_point_regressive_product(self_: PointAndPlane, other: IdealPoint) -> Plane  {
    return Plane(vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.y) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.z) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + self_.g0.yxxx * vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(-1.0, 1.0, 1.0, 1.0));
}

fn point_and_plane_ideal_point_inner_product(self_: PointAndPlane, other: IdealPoint) -> Plane  {
    return Plane(vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z, other.g0.z, other.g0.z, other.g0.x) * vec4<f32>(0.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.y, other.g0.y, other.g0.x, other.g0.y) * vec4<f32>(0.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(0.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.y) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g0.z) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.y, self_.g0.x, self_.g0.y, self_.g0.y) * vec4<f32>(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4<f32>(-1.0, 0.0, 1.0, -1.0));
}

fn point_and_plane_ideal_point_left_contraction(self_: PointAndPlane, other: IdealPoint) -> Plane  {
    return Plane(vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.y) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g0.z) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + self_.g1.yxxx * vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(-1.0));
}

fn point_and_plane_plane_into(self_: PointAndPlane) -> Plane  {
    return Plane(self_.g1);
}

fn point_and_plane_plane_add(self_: PointAndPlane, other: Plane) -> PointAndPlane  {
    return PointAndPlane(self_.g0, self_.g1 + other.g0);
}

fn point_and_plane_plane_sub(self_: PointAndPlane, other: Plane) -> PointAndPlane  {
    return PointAndPlane(self_.g0, self_.g1 - other.g0);
}

fn point_and_plane_plane_geometric_product(self_: PointAndPlane, other: Plane) -> Motor  {
    return Motor(vec4<f32>(self_.g0.y) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g1.y) * other.g0.yywz * vec4<f32>(1.0, 0.0, -1.0, 1.0) + vec4<f32>(self_.g1.z) * other.g0.zwzy * vec4<f32>(1.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g1.w) * other.g0.wzyw * vec4<f32>(1.0, -1.0, 1.0, 0.0) + vec4<f32>(self_.g1.x, self_.g0.x, self_.g0.x, self_.g0.x) * other.g0 * vec4<f32>(-1.0, 1.0, 1.0, 1.0), vec4<f32>(self_.g0.y) * other.g0.yywz * vec4<f32>(-1.0, 0.0, -1.0, 1.0) + vec4<f32>(self_.g0.z) * other.g0.zwzy * vec4<f32>(-1.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g0.w) * other.g0.wzyw * vec4<f32>(-1.0, -1.0, 1.0, 0.0) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, -1.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, -1.0, 0.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 0.0, -1.0) + vec4<f32>(self_.g0.x, self_.g1.x, self_.g1.x, self_.g1.x) * other.g0 * vec4<f32>(-1.0, 1.0, 1.0, 1.0));
}

fn point_and_plane_plane_regressive_product(self_: PointAndPlane, other: Plane) -> Scalar  {
    return Scalar(0.0 - self_.g0.x * other.g0.x - self_.g0.y * other.g0.y - self_.g0.z * other.g0.z - self_.g0.w * other.g0.w);
}

fn point_and_plane_plane_left_contraction(self_: PointAndPlane, other: Plane) -> Scalar  {
    return Scalar(0.0 - self_.g1.x * other.g0.x + self_.g1.y * other.g0.y + self_.g1.z * other.g0.z + self_.g1.w * other.g0.w);
}

fn point_and_plane_plane_scalar_product(self_: PointAndPlane, other: Plane) -> Scalar  {
    return Scalar(0.0 - self_.g1.x * other.g0.x + self_.g1.y * other.g0.y + self_.g1.z * other.g0.z + self_.g1.w * other.g0.w);
}

fn point_and_plane_line_geometric_product(self_: PointAndPlane, other: Line) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0.y) * vec4<f32>(other.g0.x, other.g0.x, other.g1.z, other.g1.y) * vec4<f32>(1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.y, other.g1.z, other.g0.y, other.g1.x) * vec4<f32>(1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.z, other.g1.y, other.g1.x, other.g0.z) * vec4<f32>(1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4<f32>(0.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g1.x, other.g1.x, other.g0.z, other.g0.y) * vec4<f32>(1.0, 0.0, -1.0, 1.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g1.y, other.g0.z, other.g1.y, other.g0.x) * vec4<f32>(1.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g1.z, other.g0.y, other.g0.x, other.g1.z) * vec4<f32>(1.0, -1.0, 1.0, 0.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0), vec4<f32>(self_.g0.y) * vec4<f32>(other.g1.x, other.g1.x, other.g0.z, other.g0.y) * vec4<f32>(1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.y, other.g0.z, other.g1.y, other.g0.x) * vec4<f32>(1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g1.z, other.g0.y, other.g0.x, other.g1.z) * vec4<f32>(1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(0.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g0.x, other.g0.x, other.g1.z, other.g1.y) * vec4<f32>(-1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.y, other.g1.z, other.g0.y, other.g1.x) * vec4<f32>(-1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g0.z, other.g1.y, other.g1.x, other.g0.z) * vec4<f32>(-1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4<f32>(0.0, -1.0, -1.0, -1.0));
}

fn point_and_plane_line_regressive_product(self_: PointAndPlane, other: Line) -> Plane  {
    return Plane(vec4<f32>(self_.g0.y) * vec4<f32>(other.g0.x, other.g0.x, other.g1.z, other.g1.y) * vec4<f32>(-1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.y, other.g1.z, other.g0.y, other.g1.x) * vec4<f32>(-1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.z, other.g1.y, other.g1.x, other.g0.z) * vec4<f32>(-1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0));
}

fn point_and_plane_line_outer_product(self_: PointAndPlane, other: Line) -> Point  {
    return Point(vec4<f32>(self_.g1.y) * vec4<f32>(other.g1.x, other.g1.x, other.g0.z, other.g0.y) * vec4<f32>(1.0, 0.0, -1.0, 1.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g1.y, other.g0.z, other.g1.y, other.g0.x) * vec4<f32>(1.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g1.z, other.g0.y, other.g0.x, other.g1.z) * vec4<f32>(1.0, -1.0, 1.0, 0.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4<f32>(0.0, -1.0, -1.0, -1.0));
}

fn point_and_plane_line_inner_product(self_: PointAndPlane, other: Line) -> Plane  {
    return Plane(vec4<f32>(self_.g0.y) * vec4<f32>(other.g1.x, other.g1.x, other.g0.z, other.g0.y) * vec4<f32>(1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.y, other.g0.z, other.g1.y, other.g0.x) * vec4<f32>(1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g1.z, other.g0.y, other.g0.x, other.g1.z) * vec4<f32>(1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(0.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g0.x, other.g0.x, other.g1.z, other.g1.y) * vec4<f32>(-1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.y, other.g1.z, other.g0.y, other.g1.x) * vec4<f32>(-1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g0.z, other.g1.y, other.g1.x, other.g0.z) * vec4<f32>(-1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4<f32>(0.0, -1.0, -1.0, -1.0));
}

fn point_and_plane_line_left_contraction(self_: PointAndPlane, other: Line) -> Plane  {
    return Plane(vec4<f32>(self_.g1.y) * vec4<f32>(other.g0.x, other.g0.x, other.g1.z, other.g1.y) * vec4<f32>(-1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.y, other.g1.z, other.g0.y, other.g1.x) * vec4<f32>(-1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g0.z, other.g1.y, other.g1.x, other.g0.z) * vec4<f32>(-1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4<f32>(0.0, -1.0, -1.0, -1.0));
}

fn point_and_plane_line_right_contraction(self_: PointAndPlane, other: Line) -> Plane  {
    return Plane(vec4<f32>(self_.g0.y) * vec4<f32>(other.g1.x, other.g1.x, other.g0.z, other.g0.y) * vec4<f32>(1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.y, other.g0.z, other.g1.y, other.g0.x) * vec4<f32>(1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g1.z, other.g0.y, other.g0.x, other.g1.z) * vec4<f32>(1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g0.x) * vec4<f32>(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4<f32>(0.0, -1.0, -1.0, -1.0));
}

fn point_and_plane_translator_geometric_product(self_: PointAndPlane, other: Translator) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0.x) * other.g0 + vec4<f32>(self_.g0.z) * other.g0.zzxz * vec4<f32>(1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g0.w) * other.g0.wwwx * vec4<f32>(1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g1.z) * other.g0.wwwy * vec4<f32>(0.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g1.w) * other.g0.zzyz * vec4<f32>(0.0, -1.0, 1.0, 0.0) + vec4<f32>(self_.g0.y, self_.g0.y, self_.g1.y, self_.g1.y) * other.g0.yxwz * vec4<f32>(1.0, 1.0, -1.0, 1.0), vec4<f32>(self_.g0.z) * other.g0.wwwy * vec4<f32>(0.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * other.g0.zzyz * vec4<f32>(0.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g1.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g1.z) * other.g0.zzxz * vec4<f32>(-1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g1.w) * other.g0.wwwx * vec4<f32>(-1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g1.y, self_.g1.y, self_.g0.y, self_.g0.y) * other.g0.yxwz * vec4<f32>(-1.0, 1.0, 1.0, -1.0));
}

fn point_and_plane_translator_regressive_product(self_: PointAndPlane, other: Translator) -> Plane  {
    return Plane(vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + self_.g0.yxxx * other.g0.yyzw * vec4<f32>(-1.0, 1.0, 1.0, 1.0));
}

fn point_and_plane_translator_outer_product(self_: PointAndPlane, other: Translator) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g1.y) * other.g0.wwwz * vec4<f32>(0.0, 0.0, -1.0, 1.0) + vec4<f32>(self_.g1.z) * other.g0.wwwy * vec4<f32>(0.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g1.w) * other.g0.zzyz * vec4<f32>(0.0, -1.0, 1.0, 0.0) + self_.g0 * vec4<f32>(other.g0.x), self_.g1 * vec4<f32>(other.g0.x));
}

fn point_and_plane_translator_inner_product(self_: PointAndPlane, other: Translator) -> PointAndPlane  {
    return PointAndPlane(self_.g0 * vec4<f32>(other.g0.x), vec4<f32>(self_.g0.z) * other.g0.wwwy * vec4<f32>(0.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * other.g0.zzyz * vec4<f32>(0.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g1.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g1.z) * other.g0.zzxz * vec4<f32>(-1.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g1.w) * other.g0.wwwx * vec4<f32>(-1.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g1.y, self_.g1.y, self_.g0.y, self_.g0.y) * other.g0.yxwz * vec4<f32>(-1.0, 1.0, 1.0, -1.0));
}

fn point_and_plane_translator_left_contraction(self_: PointAndPlane, other: Translator) -> Plane  {
    return Plane(vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.z) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g0.w) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + self_.g1.yxxx * other.g0.yyzw * vec4<f32>(-1.0));
}

fn point_and_plane_translator_right_contraction(self_: PointAndPlane, other: Translator) -> PointAndPlane  {
    return PointAndPlane(self_.g0 * vec4<f32>(other.g0.x), vec4<f32>(self_.g0.z) * other.g0.wwwy * vec4<f32>(0.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * other.g0.zzyz * vec4<f32>(0.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g1.x, self_.g1.y, self_.g0.y, self_.g0.y) * other.g0.xxwz * vec4<f32>(1.0, 1.0, 1.0, -1.0));
}

fn point_and_plane_motor_add(self_: PointAndPlane, other: Motor) -> MultiVector  {
    return MultiVector(other.g0, vec4<f32>(self_.g1.x, self_.g0.y, self_.g0.z, self_.g0.w), vec4<f32>(self_.g0.x, self_.g1.y, self_.g1.z, self_.g1.w), other.g1);
}

fn point_and_plane_motor_sub(self_: PointAndPlane, other: Motor) -> MultiVector  {
    return MultiVector(vec4<f32>(0.0) - other.g0, vec4<f32>(self_.g1.x, self_.g0.y, self_.g0.z, self_.g0.w), vec4<f32>(self_.g0.x, self_.g1.y, self_.g1.z, self_.g1.w), vec4<f32>(0.0) - other.g1);
}

fn point_and_plane_motor_geometric_product(self_: PointAndPlane, other: Motor) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0.x) * vec4<f32>(other.g0.x, other.g1.y, other.g1.z, other.g1.w) + vec4<f32>(self_.g0.y) * vec4<f32>(other.g1.y, other.g0.x, other.g0.w, other.g0.z) * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.z, other.g0.w, other.g0.x, other.g0.y) * vec4<f32>(1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g1.w, other.g0.z, other.g0.y, other.g0.x) * vec4<f32>(1.0, 1.0, -1.0, 1.0) - vec4<f32>(self_.g1.x) * vec4<f32>(other.g1.x, other.g0.y, other.g0.z, other.g0.w) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g0.y, other.g1.x, other.g1.w, other.g1.z) * vec4<f32>(1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.z, other.g1.w, other.g1.x, other.g1.y) * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g0.w, other.g1.z, other.g1.y, other.g1.x) * vec4<f32>(1.0, -1.0, 1.0, 1.0), vec4<f32>(self_.g0.x) * vec4<f32>(other.g1.x, other.g0.y, other.g0.z, other.g0.w) * vec4<f32>(1.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g0.y) * vec4<f32>(other.g0.y, other.g1.x, other.g1.w, other.g1.z) * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z, other.g1.w, other.g1.x, other.g1.y) * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w, other.g1.z, other.g1.y, other.g1.x) * vec4<f32>(1.0, 1.0, -1.0, -1.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g0.x, other.g1.y, other.g1.z, other.g1.w) * vec4<f32>(1.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g1.y, other.g0.x, other.g0.w, other.g0.z) * vec4<f32>(-1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g1.z, other.g0.w, other.g0.x, other.g0.y) * vec4<f32>(-1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g1.w, other.g0.z, other.g0.y, other.g0.x) * vec4<f32>(-1.0, 1.0, -1.0, 1.0));
}

fn point_and_plane_motor_regressive_product(self_: PointAndPlane, other: Motor) -> PointAndPlane  {
    return PointAndPlane(self_.g0 * vec4<f32>(other.g1.x), vec4<f32>(self_.g0.y) * vec4<f32>(other.g1.y, other.g1.y, other.g0.w, other.g0.z) * vec4<f32>(-1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.z, other.g0.w, other.g1.z, other.g0.y) * vec4<f32>(-1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g1.w, other.g0.z, other.g0.y, other.g1.w) * vec4<f32>(-1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g1.x) * vec4<f32>(0.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g1.x) * vec4<f32>(0.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g1.x) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g1.x, self_.g0.x, self_.g0.x, self_.g0.x) * other.g1);
}

fn point_and_plane_motor_outer_product(self_: PointAndPlane, other: Motor) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g1.x) * other.g0.yyzw * vec4<f32>(0.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g0.y, other.g0.y, other.g1.w, other.g1.z) * vec4<f32>(1.0, 0.0, -1.0, 1.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.z, other.g1.w, other.g0.z, other.g1.y) * vec4<f32>(1.0, 1.0, 0.0, -1.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g0.w, other.g1.z, other.g1.y, other.g0.w) * vec4<f32>(1.0, -1.0, 1.0, 0.0) + self_.g0 * vec4<f32>(other.g0.x), self_.g1 * vec4<f32>(other.g0.x));
}

fn point_and_plane_motor_inner_product(self_: PointAndPlane, other: Motor) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g1.x) * vec4<f32>(other.g1.x) * vec4<f32>(-1.0, 0.0, 0.0, 0.0) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g1.x) * vec4<f32>(0.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g1.x) * vec4<f32>(0.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g1.x) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + self_.g0 * vec4<f32>(other.g0.x), vec4<f32>(self_.g0.x) * vec4<f32>(other.g1.x, other.g0.y, other.g0.z, other.g0.w) * vec4<f32>(1.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g0.y) * vec4<f32>(other.g0.y, other.g1.x, other.g1.w, other.g1.z) * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z, other.g1.w, other.g1.x, other.g1.y) * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w, other.g1.z, other.g1.y, other.g1.x) * vec4<f32>(1.0, 1.0, -1.0, -1.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g0.x, other.g1.y, other.g1.z, other.g1.w) * vec4<f32>(1.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g1.y, other.g0.x, other.g0.w, other.g0.z) * vec4<f32>(-1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g1.z, other.g0.w, other.g0.x, other.g0.y) * vec4<f32>(-1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g1.w, other.g0.z, other.g0.y, other.g0.x) * vec4<f32>(-1.0, 1.0, -1.0, 1.0));
}

fn point_and_plane_motor_left_contraction(self_: PointAndPlane, other: Motor) -> PointAndPlane  {
    return PointAndPlane(self_.g1 * vec4<f32>(other.g1.x) * vec4<f32>(-1.0, 1.0, 1.0, 1.0), vec4<f32>(self_.g1.x) * other.g1.yyzw * vec4<f32>(0.0, -1.0, -1.0, -1.0) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g1.y, other.g1.y, other.g0.w, other.g0.z) * vec4<f32>(-1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g1.z, other.g0.w, other.g1.z, other.g0.y) * vec4<f32>(-1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g1.w, other.g0.z, other.g0.y, other.g1.w) * vec4<f32>(-1.0, 1.0, -1.0, 0.0) + self_.g0 * vec4<f32>(other.g1.x) * vec4<f32>(1.0, -1.0, -1.0, -1.0));
}

fn point_and_plane_motor_right_contraction(self_: PointAndPlane, other: Motor) -> PointAndPlane  {
    return PointAndPlane(self_.g0 * vec4<f32>(other.g0.x), vec4<f32>(self_.g0.y) * vec4<f32>(other.g0.y, other.g0.y, other.g1.w, other.g1.z) * vec4<f32>(1.0, 0.0, 1.0, -1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z, other.g1.w, other.g0.z, other.g1.y) * vec4<f32>(1.0, -1.0, 0.0, 1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w, other.g1.z, other.g1.y, other.g0.w) * vec4<f32>(1.0, 1.0, -1.0, 0.0) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 1.0, 0.0, 0.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 1.0, 0.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g0.x) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + vec4<f32>(self_.g1.x, self_.g0.x, self_.g0.x, self_.g0.x) * other.g0 * vec4<f32>(1.0, -1.0, -1.0, -1.0));
}

fn point_and_plane_point_and_plane_add(self_: PointAndPlane, other: PointAndPlane) -> PointAndPlane  {
    return PointAndPlane(self_.g0 + other.g0, self_.g1 + other.g1);
}

fn point_and_plane_point_and_plane_sub(self_: PointAndPlane, other: PointAndPlane) -> PointAndPlane  {
    return PointAndPlane(self_.g0 - other.g0, self_.g1 - other.g1);
}

fn point_and_plane_point_and_plane_mul(self_: PointAndPlane, other: PointAndPlane) -> PointAndPlane  {
    return PointAndPlane(self_.g0 * other.g0, self_.g1 * other.g1);
}

fn point_and_plane_point_and_plane_div(self_: PointAndPlane, other: PointAndPlane) -> PointAndPlane  {
    return PointAndPlane(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0) / vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0), vec4<f32>(self_.g1.x, self_.g1.y, self_.g1.z, self_.g1.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0) / vec4<f32>(other.g1.x, other.g1.y, other.g1.z, other.g1.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0));
}

fn point_and_plane_point_and_plane_geometric_product(self_: PointAndPlane, other: PointAndPlane) -> Motor  {
    return Motor(vec4<f32>(self_.g0.x) * vec4<f32>(other.g0.x, other.g1.y, other.g1.z, other.g1.w) * vec4<f32>(-1.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g0.y) * vec4<f32>(other.g0.y, other.g1.x, other.g0.w, other.g0.z) * vec4<f32>(1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g0.z, other.g0.w, other.g1.x, other.g0.y) * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g0.w, other.g0.z, other.g0.y, other.g1.x) * vec4<f32>(1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g1.x, other.g0.y, other.g0.z, other.g0.w) * vec4<f32>(-1.0, 1.0, 1.0, 1.0) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g1.y, other.g0.x, other.g1.w, other.g1.z) * vec4<f32>(1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g1.z, other.g1.w, other.g0.x, other.g1.y) * vec4<f32>(1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g1.w, other.g1.z, other.g1.y, other.g0.x) * vec4<f32>(1.0, -1.0, 1.0, 1.0), vec4<f32>(0.0) - vec4<f32>(self_.g0.x) * vec4<f32>(other.g1.x, other.g0.y, other.g0.z, other.g0.w) + vec4<f32>(self_.g0.y) * vec4<f32>(other.g1.y, other.g0.x, other.g1.w, other.g1.z) * vec4<f32>(-1.0, 1.0, -1.0, 1.0) + vec4<f32>(self_.g0.z) * vec4<f32>(other.g1.z, other.g1.w, other.g0.x, other.g1.y) * vec4<f32>(-1.0, 1.0, 1.0, -1.0) + vec4<f32>(self_.g0.w) * vec4<f32>(other.g1.w, other.g1.z, other.g1.y, other.g0.x) * vec4<f32>(-1.0, -1.0, 1.0, 1.0) + vec4<f32>(self_.g1.x) * vec4<f32>(other.g0.x, other.g1.y, other.g1.z, other.g1.w) + vec4<f32>(self_.g1.y) * vec4<f32>(other.g0.y, other.g1.x, other.g0.w, other.g0.z) * vec4<f32>(1.0, -1.0, 1.0, -1.0) + vec4<f32>(self_.g1.z) * vec4<f32>(other.g0.z, other.g0.w, other.g1.x, other.g0.y) * vec4<f32>(1.0, -1.0, -1.0, 1.0) + vec4<f32>(self_.g1.w) * vec4<f32>(other.g0.w, other.g0.z, other.g0.y, other.g1.x) * vec4<f32>(1.0, 1.0, -1.0, -1.0));
}

fn point_and_plane_point_and_plane_scalar_product(self_: PointAndPlane, other: PointAndPlane) -> Scalar  {
    return Scalar(0.0 - self_.g0.x * other.g0.x + self_.g0.y * other.g0.y + self_.g0.z * other.g0.z + self_.g0.w * other.g0.w - self_.g1.x * other.g1.x + self_.g1.y * other.g1.y + self_.g1.z * other.g1.z + self_.g1.w * other.g1.w);
}

fn point_and_plane_squared_magnitude(self_: PointAndPlane) -> Scalar  {
    return point_and_plane_point_and_plane_scalar_product(self_, point_and_plane_reversal(self_));
}

fn point_and_plane_magnitude(self_: PointAndPlane) -> Scalar  {
    return Scalar(sqrt(point_and_plane_squared_magnitude(self_).g0));
}

fn point_and_plane_scale(self_: PointAndPlane, other: f32) -> PointAndPlane  {
    return point_and_plane_scalar_geometric_product(self_, Scalar(other));
}

fn point_and_plane_signum(self_: PointAndPlane) -> PointAndPlane  {
    return point_and_plane_scalar_geometric_product(self_, Scalar(1.0 / point_and_plane_magnitude(self_).g0));
}

fn point_and_plane_inverse(self_: PointAndPlane) -> PointAndPlane  {
    return point_and_plane_scalar_geometric_product(point_and_plane_reversal(self_), Scalar(1.0 / point_and_plane_squared_magnitude(self_).g0));
}

fn ideal_point_ideal_point_geometric_quotient(self_: IdealPoint, other: IdealPoint) -> Rotor  {
    return ideal_point_ideal_point_geometric_product(self_, ideal_point_inverse(other));
}

fn ideal_point_line_geometric_quotient(self_: IdealPoint, other: Line) -> Motor  {
    return ideal_point_line_geometric_product(self_, line_inverse(other));
}

fn ideal_point_line_transformation(self_: IdealPoint, other: Line) -> Line  {
    return motor_line_into(motor_ideal_point_geometric_product(ideal_point_line_geometric_product(self_, other), ideal_point_reversal(self_)));
}

fn ideal_point_motor_geometric_quotient(self_: IdealPoint, other: Motor) -> Motor  {
    return ideal_point_motor_geometric_product(self_, motor_inverse(other));
}

fn ideal_point_motor_transformation(self_: IdealPoint, other: Motor) -> Motor  {
    return motor_ideal_point_geometric_product(ideal_point_motor_geometric_product(self_, other), ideal_point_reversal(self_));
}

fn ideal_point_multi_vector_geometric_quotient(self_: IdealPoint, other: MultiVector) -> MultiVector  {
    return ideal_point_multi_vector_geometric_product(self_, multi_vector_inverse(other));
}

fn ideal_point_multi_vector_transformation(self_: IdealPoint, other: MultiVector) -> MultiVector  {
    return multi_vector_ideal_point_geometric_product(ideal_point_multi_vector_geometric_product(self_, other), ideal_point_reversal(self_));
}

fn ideal_point_point_and_plane_geometric_quotient(self_: IdealPoint, other: PointAndPlane) -> PointAndPlane  {
    return ideal_point_point_and_plane_geometric_product(self_, point_and_plane_inverse(other));
}

fn ideal_point_point_and_plane_transformation(self_: IdealPoint, other: PointAndPlane) -> PointAndPlane  {
    return point_and_plane_ideal_point_geometric_product(ideal_point_point_and_plane_geometric_product(self_, other), ideal_point_reversal(self_));
}

fn ideal_point_scalar_geometric_quotient(self_: IdealPoint, other: Scalar) -> IdealPoint  {
    return ideal_point_scalar_geometric_product(self_, scalar_inverse(other));
}

fn ideal_point_scalar_transformation(self_: IdealPoint, other: Scalar) -> Scalar  {
    return rotor_scalar_into(ideal_point_ideal_point_geometric_product(ideal_point_scalar_geometric_product(self_, other), ideal_point_reversal(self_)));
}

fn line_ideal_point_geometric_quotient(self_: Line, other: IdealPoint) -> Motor  {
    return line_ideal_point_geometric_product(self_, ideal_point_inverse(other));
}

fn line_ideal_point_transformation(self_: Line, other: IdealPoint) -> IdealPoint  {
    return motor_ideal_point_into(motor_line_geometric_product(line_ideal_point_geometric_product(self_, other), line_reversal(self_)));
}

fn line_line_geometric_quotient(self_: Line, other: Line) -> Motor  {
    return line_line_geometric_product(self_, line_inverse(other));
}

fn line_line_transformation(self_: Line, other: Line) -> Line  {
    return motor_line_into(motor_line_geometric_product(line_line_geometric_product(self_, other), line_reversal(self_)));
}

fn line_motor_geometric_quotient(self_: Line, other: Motor) -> Motor  {
    return line_motor_geometric_product(self_, motor_inverse(other));
}

fn line_motor_transformation(self_: Line, other: Motor) -> Motor  {
    return motor_line_geometric_product(line_motor_geometric_product(self_, other), line_reversal(self_));
}

fn line_multi_vector_geometric_quotient(self_: Line, other: MultiVector) -> MultiVector  {
    return line_multi_vector_geometric_product(self_, multi_vector_inverse(other));
}

fn line_multi_vector_transformation(self_: Line, other: MultiVector) -> MultiVector  {
    return multi_vector_line_geometric_product(line_multi_vector_geometric_product(self_, other), line_reversal(self_));
}

fn line_plane_geometric_quotient(self_: Line, other: Plane) -> PointAndPlane  {
    return line_plane_geometric_product(self_, plane_inverse(other));
}

fn line_plane_transformation(self_: Line, other: Plane) -> Plane  {
    return point_and_plane_plane_into(point_and_plane_line_geometric_product(line_plane_geometric_product(self_, other), line_reversal(self_)));
}

fn line_point_geometric_quotient(self_: Line, other: Point) -> PointAndPlane  {
    return line_point_geometric_product(self_, point_inverse(other));
}

fn line_point_transformation(self_: Line, other: Point) -> Point  {
    return point_and_plane_point_into(point_and_plane_line_geometric_product(line_point_geometric_product(self_, other), line_reversal(self_)));
}

fn line_point_and_plane_geometric_quotient(self_: Line, other: PointAndPlane) -> PointAndPlane  {
    return line_point_and_plane_geometric_product(self_, point_and_plane_inverse(other));
}

fn line_point_and_plane_transformation(self_: Line, other: PointAndPlane) -> PointAndPlane  {
    return point_and_plane_line_geometric_product(line_point_and_plane_geometric_product(self_, other), line_reversal(self_));
}

fn line_rotor_geometric_quotient(self_: Line, other: Rotor) -> Motor  {
    return line_rotor_geometric_product(self_, rotor_inverse(other));
}

fn line_rotor_transformation(self_: Line, other: Rotor) -> Rotor  {
    return motor_rotor_into(motor_line_geometric_product(line_rotor_geometric_product(self_, other), line_reversal(self_)));
}

fn line_scalar_geometric_quotient(self_: Line, other: Scalar) -> Line  {
    return line_scalar_geometric_product(self_, scalar_inverse(other));
}

fn line_scalar_transformation(self_: Line, other: Scalar) -> Scalar  {
    return motor_scalar_into(line_line_geometric_product(line_scalar_geometric_product(self_, other), line_reversal(self_)));
}

fn line_translator_geometric_quotient(self_: Line, other: Translator) -> Motor  {
    return line_translator_geometric_product(self_, translator_inverse(other));
}

fn line_translator_transformation(self_: Line, other: Translator) -> Translator  {
    return motor_translator_into(motor_line_geometric_product(line_translator_geometric_product(self_, other), line_reversal(self_)));
}

fn motor_ideal_point_geometric_quotient(self_: Motor, other: IdealPoint) -> Motor  {
    return motor_ideal_point_geometric_product(self_, ideal_point_inverse(other));
}

fn motor_ideal_point_transformation(self_: Motor, other: IdealPoint) -> IdealPoint  {
    return motor_ideal_point_into(motor_motor_geometric_product(motor_ideal_point_geometric_product(self_, other), motor_reversal(self_)));
}

fn motor_line_geometric_quotient(self_: Motor, other: Line) -> Motor  {
    return motor_line_geometric_product(self_, line_inverse(other));
}

fn motor_line_transformation(self_: Motor, other: Line) -> Line  {
    return motor_line_into(motor_motor_geometric_product(motor_line_geometric_product(self_, other), motor_reversal(self_)));
}

fn motor_powi(self_: Motor, exponent: int) -> Motor  {
    if (exponent == 0) {
        return motor_one();
    }
    let x: Motor = select(self_, motor_inverse(self_), exponent < 0);
    let y: Motor = motor_one();
    let n: int = abs(exponent);
    while (1 < n) {
        if ((n & 1) == 1) {
            let y = motor_motor_geometric_product(x, y);
        }
        let x = motor_motor_geometric_product(x, x);
        let n = n >> 1;
    }
    return motor_motor_geometric_product(x, y);
}

fn motor_motor_geometric_quotient(self_: Motor, other: Motor) -> Motor  {
    return motor_motor_geometric_product(self_, motor_inverse(other));
}

fn motor_motor_transformation(self_: Motor, other: Motor) -> Motor  {
    return motor_motor_geometric_product(motor_motor_geometric_product(self_, other), motor_reversal(self_));
}

fn motor_multi_vector_geometric_quotient(self_: Motor, other: MultiVector) -> MultiVector  {
    return motor_multi_vector_geometric_product(self_, multi_vector_inverse(other));
}

fn motor_multi_vector_transformation(self_: Motor, other: MultiVector) -> MultiVector  {
    return multi_vector_motor_geometric_product(motor_multi_vector_geometric_product(self_, other), motor_reversal(self_));
}

fn motor_plane_geometric_quotient(self_: Motor, other: Plane) -> PointAndPlane  {
    return motor_plane_geometric_product(self_, plane_inverse(other));
}

fn motor_plane_transformation(self_: Motor, other: Plane) -> Plane  {
    return point_and_plane_plane_into(point_and_plane_motor_geometric_product(motor_plane_geometric_product(self_, other), motor_reversal(self_)));
}

fn motor_point_geometric_quotient(self_: Motor, other: Point) -> PointAndPlane  {
    return motor_point_geometric_product(self_, point_inverse(other));
}

fn motor_point_transformation(self_: Motor, other: Point) -> Point  {
    return point_and_plane_point_into(point_and_plane_motor_geometric_product(motor_point_geometric_product(self_, other), motor_reversal(self_)));
}

fn motor_point_and_plane_geometric_quotient(self_: Motor, other: PointAndPlane) -> PointAndPlane  {
    return motor_point_and_plane_geometric_product(self_, point_and_plane_inverse(other));
}

fn motor_point_and_plane_transformation(self_: Motor, other: PointAndPlane) -> PointAndPlane  {
    return point_and_plane_motor_geometric_product(motor_point_and_plane_geometric_product(self_, other), motor_reversal(self_));
}

fn motor_rotor_geometric_quotient(self_: Motor, other: Rotor) -> Motor  {
    return motor_rotor_geometric_product(self_, rotor_inverse(other));
}

fn motor_rotor_transformation(self_: Motor, other: Rotor) -> Rotor  {
    return motor_rotor_into(motor_motor_geometric_product(motor_rotor_geometric_product(self_, other), motor_reversal(self_)));
}

fn motor_scalar_geometric_quotient(self_: Motor, other: Scalar) -> Motor  {
    return motor_scalar_geometric_product(self_, scalar_inverse(other));
}

fn motor_scalar_transformation(self_: Motor, other: Scalar) -> Scalar  {
    return motor_scalar_into(motor_motor_geometric_product(motor_scalar_geometric_product(self_, other), motor_reversal(self_)));
}

fn motor_translator_geometric_quotient(self_: Motor, other: Translator) -> Motor  {
    return motor_translator_geometric_product(self_, translator_inverse(other));
}

fn motor_translator_transformation(self_: Motor, other: Translator) -> Translator  {
    return motor_translator_into(motor_motor_geometric_product(motor_translator_geometric_product(self_, other), motor_reversal(self_)));
}

fn multi_vector_ideal_point_geometric_quotient(self_: MultiVector, other: IdealPoint) -> MultiVector  {
    return multi_vector_ideal_point_geometric_product(self_, ideal_point_inverse(other));
}

fn multi_vector_ideal_point_transformation(self_: MultiVector, other: IdealPoint) -> IdealPoint  {
    return multi_vector_ideal_point_into(multi_vector_multi_vector_geometric_product(multi_vector_ideal_point_geometric_product(self_, other), multi_vector_reversal(self_)));
}

fn multi_vector_line_geometric_quotient(self_: MultiVector, other: Line) -> MultiVector  {
    return multi_vector_line_geometric_product(self_, line_inverse(other));
}

fn multi_vector_line_transformation(self_: MultiVector, other: Line) -> Line  {
    return multi_vector_line_into(multi_vector_multi_vector_geometric_product(multi_vector_line_geometric_product(self_, other), multi_vector_reversal(self_)));
}

fn multi_vector_motor_geometric_quotient(self_: MultiVector, other: Motor) -> MultiVector  {
    return multi_vector_motor_geometric_product(self_, motor_inverse(other));
}

fn multi_vector_motor_transformation(self_: MultiVector, other: Motor) -> Motor  {
    return multi_vector_motor_into(multi_vector_multi_vector_geometric_product(multi_vector_motor_geometric_product(self_, other), multi_vector_reversal(self_)));
}

fn multi_vector_powi(self_: MultiVector, exponent: int) -> MultiVector  {
    if (exponent == 0) {
        return multi_vector_one();
    }
    let x: MultiVector = select(self_, multi_vector_inverse(self_), exponent < 0);
    let y: MultiVector = multi_vector_one();
    let n: int = abs(exponent);
    while (1 < n) {
        if ((n & 1) == 1) {
            let y = multi_vector_multi_vector_geometric_product(x, y);
        }
        let x = multi_vector_multi_vector_geometric_product(x, x);
        let n = n >> 1;
    }
    return multi_vector_multi_vector_geometric_product(x, y);
}

fn multi_vector_multi_vector_geometric_quotient(self_: MultiVector, other: MultiVector) -> MultiVector  {
    return multi_vector_multi_vector_geometric_product(self_, multi_vector_inverse(other));
}

fn multi_vector_multi_vector_transformation(self_: MultiVector, other: MultiVector) -> MultiVector  {
    return multi_vector_multi_vector_geometric_product(multi_vector_multi_vector_geometric_product(self_, other), multi_vector_reversal(self_));
}

fn multi_vector_plane_geometric_quotient(self_: MultiVector, other: Plane) -> MultiVector  {
    return multi_vector_plane_geometric_product(self_, plane_inverse(other));
}

fn multi_vector_plane_transformation(self_: MultiVector, other: Plane) -> Plane  {
    return multi_vector_plane_into(multi_vector_multi_vector_geometric_product(multi_vector_plane_geometric_product(self_, other), multi_vector_reversal(self_)));
}

fn multi_vector_point_geometric_quotient(self_: MultiVector, other: Point) -> MultiVector  {
    return multi_vector_point_geometric_product(self_, point_inverse(other));
}

fn multi_vector_point_transformation(self_: MultiVector, other: Point) -> Point  {
    return multi_vector_point_into(multi_vector_multi_vector_geometric_product(multi_vector_point_geometric_product(self_, other), multi_vector_reversal(self_)));
}

fn multi_vector_point_and_plane_geometric_quotient(self_: MultiVector, other: PointAndPlane) -> MultiVector  {
    return multi_vector_point_and_plane_geometric_product(self_, point_and_plane_inverse(other));
}

fn multi_vector_point_and_plane_transformation(self_: MultiVector, other: PointAndPlane) -> PointAndPlane  {
    return multi_vector_point_and_plane_into(multi_vector_multi_vector_geometric_product(multi_vector_point_and_plane_geometric_product(self_, other), multi_vector_reversal(self_)));
}

fn multi_vector_rotor_geometric_quotient(self_: MultiVector, other: Rotor) -> MultiVector  {
    return multi_vector_rotor_geometric_product(self_, rotor_inverse(other));
}

fn multi_vector_rotor_transformation(self_: MultiVector, other: Rotor) -> Rotor  {
    return multi_vector_rotor_into(multi_vector_multi_vector_geometric_product(multi_vector_rotor_geometric_product(self_, other), multi_vector_reversal(self_)));
}

fn multi_vector_scalar_geometric_quotient(self_: MultiVector, other: Scalar) -> MultiVector  {
    return multi_vector_scalar_geometric_product(self_, scalar_inverse(other));
}

fn multi_vector_scalar_transformation(self_: MultiVector, other: Scalar) -> Scalar  {
    return multi_vector_scalar_into(multi_vector_multi_vector_geometric_product(multi_vector_scalar_geometric_product(self_, other), multi_vector_reversal(self_)));
}

fn multi_vector_translator_geometric_quotient(self_: MultiVector, other: Translator) -> MultiVector  {
    return multi_vector_translator_geometric_product(self_, translator_inverse(other));
}

fn multi_vector_translator_transformation(self_: MultiVector, other: Translator) -> Translator  {
    return multi_vector_translator_into(multi_vector_multi_vector_geometric_product(multi_vector_translator_geometric_product(self_, other), multi_vector_reversal(self_)));
}

fn plane_line_geometric_quotient(self_: Plane, other: Line) -> PointAndPlane  {
    return plane_line_geometric_product(self_, line_inverse(other));
}

fn plane_line_transformation(self_: Plane, other: Line) -> Line  {
    return motor_line_into(point_and_plane_plane_geometric_product(plane_line_geometric_product(self_, other), plane_reversal(self_)));
}

fn plane_motor_geometric_quotient(self_: Plane, other: Motor) -> PointAndPlane  {
    return plane_motor_geometric_product(self_, motor_inverse(other));
}

fn plane_motor_transformation(self_: Plane, other: Motor) -> Motor  {
    return point_and_plane_plane_geometric_product(plane_motor_geometric_product(self_, other), plane_reversal(self_));
}

fn plane_multi_vector_geometric_quotient(self_: Plane, other: MultiVector) -> MultiVector  {
    return plane_multi_vector_geometric_product(self_, multi_vector_inverse(other));
}

fn plane_multi_vector_transformation(self_: Plane, other: MultiVector) -> MultiVector  {
    return multi_vector_plane_geometric_product(plane_multi_vector_geometric_product(self_, other), plane_reversal(self_));
}

fn plane_point_and_plane_geometric_quotient(self_: Plane, other: PointAndPlane) -> Motor  {
    return plane_point_and_plane_geometric_product(self_, point_and_plane_inverse(other));
}

fn plane_point_and_plane_transformation(self_: Plane, other: PointAndPlane) -> PointAndPlane  {
    return motor_plane_geometric_product(plane_point_and_plane_geometric_product(self_, other), plane_reversal(self_));
}

fn plane_rotor_geometric_quotient(self_: Plane, other: Rotor) -> PointAndPlane  {
    return plane_rotor_geometric_product(self_, rotor_inverse(other));
}

fn plane_rotor_transformation(self_: Plane, other: Rotor) -> Rotor  {
    return motor_rotor_into(point_and_plane_plane_geometric_product(plane_rotor_geometric_product(self_, other), plane_reversal(self_)));
}

fn plane_scalar_geometric_quotient(self_: Plane, other: Scalar) -> Plane  {
    return plane_scalar_geometric_product(self_, scalar_inverse(other));
}

fn point_line_geometric_quotient(self_: Point, other: Line) -> PointAndPlane  {
    return point_line_geometric_product(self_, line_inverse(other));
}

fn point_line_transformation(self_: Point, other: Line) -> Line  {
    return motor_line_into(point_and_plane_point_geometric_product(point_line_geometric_product(self_, other), point_reversal(self_)));
}

fn point_motor_geometric_quotient(self_: Point, other: Motor) -> PointAndPlane  {
    return point_motor_geometric_product(self_, motor_inverse(other));
}

fn point_motor_transformation(self_: Point, other: Motor) -> Motor  {
    return point_and_plane_point_geometric_product(point_motor_geometric_product(self_, other), point_reversal(self_));
}

fn point_multi_vector_geometric_quotient(self_: Point, other: MultiVector) -> MultiVector  {
    return point_multi_vector_geometric_product(self_, multi_vector_inverse(other));
}

fn point_multi_vector_transformation(self_: Point, other: MultiVector) -> MultiVector  {
    return multi_vector_point_geometric_product(point_multi_vector_geometric_product(self_, other), point_reversal(self_));
}

fn point_point_and_plane_geometric_quotient(self_: Point, other: PointAndPlane) -> Motor  {
    return point_point_and_plane_geometric_product(self_, point_and_plane_inverse(other));
}

fn point_point_and_plane_transformation(self_: Point, other: PointAndPlane) -> PointAndPlane  {
    return motor_point_geometric_product(point_point_and_plane_geometric_product(self_, other), point_reversal(self_));
}

fn point_rotor_geometric_quotient(self_: Point, other: Rotor) -> PointAndPlane  {
    return point_rotor_geometric_product(self_, rotor_inverse(other));
}

fn point_rotor_transformation(self_: Point, other: Rotor) -> Rotor  {
    return motor_rotor_into(point_and_plane_point_geometric_product(point_rotor_geometric_product(self_, other), point_reversal(self_)));
}

fn point_scalar_geometric_quotient(self_: Point, other: Scalar) -> Point  {
    return point_scalar_geometric_product(self_, scalar_inverse(other));
}

fn point_and_plane_ideal_point_geometric_quotient(self_: PointAndPlane, other: IdealPoint) -> PointAndPlane  {
    return point_and_plane_ideal_point_geometric_product(self_, ideal_point_inverse(other));
}

fn point_and_plane_ideal_point_transformation(self_: PointAndPlane, other: IdealPoint) -> IdealPoint  {
    return motor_ideal_point_into(point_and_plane_point_and_plane_geometric_product(point_and_plane_ideal_point_geometric_product(self_, other), point_and_plane_reversal(self_)));
}

fn point_and_plane_line_geometric_quotient(self_: PointAndPlane, other: Line) -> PointAndPlane  {
    return point_and_plane_line_geometric_product(self_, line_inverse(other));
}

fn point_and_plane_line_transformation(self_: PointAndPlane, other: Line) -> Line  {
    return motor_line_into(point_and_plane_point_and_plane_geometric_product(point_and_plane_line_geometric_product(self_, other), point_and_plane_reversal(self_)));
}

fn point_and_plane_motor_geometric_quotient(self_: PointAndPlane, other: Motor) -> PointAndPlane  {
    return point_and_plane_motor_geometric_product(self_, motor_inverse(other));
}

fn point_and_plane_motor_transformation(self_: PointAndPlane, other: Motor) -> Motor  {
    return point_and_plane_point_and_plane_geometric_product(point_and_plane_motor_geometric_product(self_, other), point_and_plane_reversal(self_));
}

fn point_and_plane_multi_vector_geometric_quotient(self_: PointAndPlane, other: MultiVector) -> MultiVector  {
    return point_and_plane_multi_vector_geometric_product(self_, multi_vector_inverse(other));
}

fn point_and_plane_multi_vector_transformation(self_: PointAndPlane, other: MultiVector) -> MultiVector  {
    return multi_vector_point_and_plane_geometric_product(point_and_plane_multi_vector_geometric_product(self_, other), point_and_plane_reversal(self_));
}

fn point_and_plane_plane_geometric_quotient(self_: PointAndPlane, other: Plane) -> Motor  {
    return point_and_plane_plane_geometric_product(self_, plane_inverse(other));
}

fn point_and_plane_plane_transformation(self_: PointAndPlane, other: Plane) -> Plane  {
    return point_and_plane_plane_into(motor_point_and_plane_geometric_product(point_and_plane_plane_geometric_product(self_, other), point_and_plane_reversal(self_)));
}

fn point_and_plane_point_geometric_quotient(self_: PointAndPlane, other: Point) -> Motor  {
    return point_and_plane_point_geometric_product(self_, point_inverse(other));
}

fn point_and_plane_point_transformation(self_: PointAndPlane, other: Point) -> Point  {
    return point_and_plane_point_into(motor_point_and_plane_geometric_product(point_and_plane_point_geometric_product(self_, other), point_and_plane_reversal(self_)));
}

fn point_and_plane_point_and_plane_geometric_quotient(self_: PointAndPlane, other: PointAndPlane) -> Motor  {
    return point_and_plane_point_and_plane_geometric_product(self_, point_and_plane_inverse(other));
}

fn point_and_plane_point_and_plane_transformation(self_: PointAndPlane, other: PointAndPlane) -> PointAndPlane  {
    return motor_point_and_plane_geometric_product(point_and_plane_point_and_plane_geometric_product(self_, other), point_and_plane_reversal(self_));
}

fn point_and_plane_rotor_geometric_quotient(self_: PointAndPlane, other: Rotor) -> PointAndPlane  {
    return point_and_plane_rotor_geometric_product(self_, rotor_inverse(other));
}

fn point_and_plane_rotor_transformation(self_: PointAndPlane, other: Rotor) -> Rotor  {
    return motor_rotor_into(point_and_plane_point_and_plane_geometric_product(point_and_plane_rotor_geometric_product(self_, other), point_and_plane_reversal(self_)));
}

fn point_and_plane_scalar_geometric_quotient(self_: PointAndPlane, other: Scalar) -> PointAndPlane  {
    return point_and_plane_scalar_geometric_product(self_, scalar_inverse(other));
}

fn point_and_plane_scalar_transformation(self_: PointAndPlane, other: Scalar) -> Scalar  {
    return motor_scalar_into(point_and_plane_point_and_plane_geometric_product(point_and_plane_scalar_geometric_product(self_, other), point_and_plane_reversal(self_)));
}

fn point_and_plane_translator_geometric_quotient(self_: PointAndPlane, other: Translator) -> PointAndPlane  {
    return point_and_plane_translator_geometric_product(self_, translator_inverse(other));
}

fn point_and_plane_translator_transformation(self_: PointAndPlane, other: Translator) -> Translator  {
    return motor_translator_into(point_and_plane_point_and_plane_geometric_product(point_and_plane_translator_geometric_product(self_, other), point_and_plane_reversal(self_)));
}

fn rotor_line_geometric_quotient(self_: Rotor, other: Line) -> Motor  {
    return rotor_line_geometric_product(self_, line_inverse(other));
}

fn rotor_line_transformation(self_: Rotor, other: Line) -> Line  {
    return motor_line_into(motor_rotor_geometric_product(rotor_line_geometric_product(self_, other), rotor_reversal(self_)));
}

fn rotor_motor_geometric_quotient(self_: Rotor, other: Motor) -> Motor  {
    return rotor_motor_geometric_product(self_, motor_inverse(other));
}

fn rotor_motor_transformation(self_: Rotor, other: Motor) -> Motor  {
    return motor_rotor_geometric_product(rotor_motor_geometric_product(self_, other), rotor_reversal(self_));
}

fn rotor_multi_vector_geometric_quotient(self_: Rotor, other: MultiVector) -> MultiVector  {
    return rotor_multi_vector_geometric_product(self_, multi_vector_inverse(other));
}

fn rotor_multi_vector_transformation(self_: Rotor, other: MultiVector) -> MultiVector  {
    return multi_vector_rotor_geometric_product(rotor_multi_vector_geometric_product(self_, other), rotor_reversal(self_));
}

fn rotor_plane_geometric_quotient(self_: Rotor, other: Plane) -> PointAndPlane  {
    return rotor_plane_geometric_product(self_, plane_inverse(other));
}

fn rotor_plane_transformation(self_: Rotor, other: Plane) -> Plane  {
    return point_and_plane_plane_into(point_and_plane_rotor_geometric_product(rotor_plane_geometric_product(self_, other), rotor_reversal(self_)));
}

fn rotor_point_geometric_quotient(self_: Rotor, other: Point) -> PointAndPlane  {
    return rotor_point_geometric_product(self_, point_inverse(other));
}

fn rotor_point_transformation(self_: Rotor, other: Point) -> Point  {
    return point_and_plane_point_into(point_and_plane_rotor_geometric_product(rotor_point_geometric_product(self_, other), rotor_reversal(self_)));
}

fn rotor_point_and_plane_geometric_quotient(self_: Rotor, other: PointAndPlane) -> PointAndPlane  {
    return rotor_point_and_plane_geometric_product(self_, point_and_plane_inverse(other));
}

fn rotor_point_and_plane_transformation(self_: Rotor, other: PointAndPlane) -> PointAndPlane  {
    return point_and_plane_rotor_geometric_product(rotor_point_and_plane_geometric_product(self_, other), rotor_reversal(self_));
}

fn rotor_powi(self_: Rotor, exponent: int) -> Rotor  {
    if (exponent == 0) {
        return rotor_one();
    }
    let x: Rotor = select(self_, rotor_inverse(self_), exponent < 0);
    let y: Rotor = rotor_one();
    let n: int = abs(exponent);
    while (1 < n) {
        if ((n & 1) == 1) {
            let y = rotor_rotor_geometric_product(x, y);
        }
        let x = rotor_rotor_geometric_product(x, x);
        let n = n >> 1;
    }
    return rotor_rotor_geometric_product(x, y);
}

fn rotor_rotor_geometric_quotient(self_: Rotor, other: Rotor) -> Rotor  {
    return rotor_rotor_geometric_product(self_, rotor_inverse(other));
}

fn rotor_rotor_transformation(self_: Rotor, other: Rotor) -> Rotor  {
    return rotor_rotor_geometric_product(rotor_rotor_geometric_product(self_, other), rotor_reversal(self_));
}

fn rotor_scalar_geometric_quotient(self_: Rotor, other: Scalar) -> Rotor  {
    return rotor_scalar_geometric_product(self_, scalar_inverse(other));
}

fn rotor_scalar_transformation(self_: Rotor, other: Scalar) -> Scalar  {
    return rotor_scalar_into(rotor_rotor_geometric_product(rotor_scalar_geometric_product(self_, other), rotor_reversal(self_)));
}

fn rotor_translator_geometric_quotient(self_: Rotor, other: Translator) -> Motor  {
    return rotor_translator_geometric_product(self_, translator_inverse(other));
}

fn rotor_translator_transformation(self_: Rotor, other: Translator) -> Translator  {
    return motor_translator_into(motor_rotor_geometric_product(rotor_translator_geometric_product(self_, other), rotor_reversal(self_)));
}

fn scalar_ideal_point_geometric_quotient(self_: Scalar, other: IdealPoint) -> IdealPoint  {
    return scalar_ideal_point_geometric_product(self_, ideal_point_inverse(other));
}

fn scalar_ideal_point_transformation(self_: Scalar, other: IdealPoint) -> IdealPoint  {
    return ideal_point_scalar_geometric_product(scalar_ideal_point_geometric_product(self_, other), scalar_reversal(self_));
}

fn scalar_line_geometric_quotient(self_: Scalar, other: Line) -> Line  {
    return scalar_line_geometric_product(self_, line_inverse(other));
}

fn scalar_line_transformation(self_: Scalar, other: Line) -> Line  {
    return line_scalar_geometric_product(scalar_line_geometric_product(self_, other), scalar_reversal(self_));
}

fn scalar_motor_geometric_quotient(self_: Scalar, other: Motor) -> Motor  {
    return scalar_motor_geometric_product(self_, motor_inverse(other));
}

fn scalar_motor_transformation(self_: Scalar, other: Motor) -> Motor  {
    return motor_scalar_geometric_product(scalar_motor_geometric_product(self_, other), scalar_reversal(self_));
}

fn scalar_multi_vector_geometric_quotient(self_: Scalar, other: MultiVector) -> MultiVector  {
    return scalar_multi_vector_geometric_product(self_, multi_vector_inverse(other));
}

fn scalar_multi_vector_transformation(self_: Scalar, other: MultiVector) -> MultiVector  {
    return multi_vector_scalar_geometric_product(scalar_multi_vector_geometric_product(self_, other), scalar_reversal(self_));
}

fn scalar_plane_geometric_quotient(self_: Scalar, other: Plane) -> Plane  {
    return scalar_plane_geometric_product(self_, plane_inverse(other));
}

fn scalar_plane_transformation(self_: Scalar, other: Plane) -> Plane  {
    return plane_scalar_geometric_product(scalar_plane_geometric_product(self_, other), scalar_reversal(self_));
}

fn scalar_point_geometric_quotient(self_: Scalar, other: Point) -> Point  {
    return scalar_point_geometric_product(self_, point_inverse(other));
}

fn scalar_point_transformation(self_: Scalar, other: Point) -> Point  {
    return point_scalar_geometric_product(scalar_point_geometric_product(self_, other), scalar_reversal(self_));
}

fn scalar_point_and_plane_geometric_quotient(self_: Scalar, other: PointAndPlane) -> PointAndPlane  {
    return scalar_point_and_plane_geometric_product(self_, point_and_plane_inverse(other));
}

fn scalar_point_and_plane_transformation(self_: Scalar, other: PointAndPlane) -> PointAndPlane  {
    return point_and_plane_scalar_geometric_product(scalar_point_and_plane_geometric_product(self_, other), scalar_reversal(self_));
}

fn scalar_rotor_geometric_quotient(self_: Scalar, other: Rotor) -> Rotor  {
    return scalar_rotor_geometric_product(self_, rotor_inverse(other));
}

fn scalar_rotor_transformation(self_: Scalar, other: Rotor) -> Rotor  {
    return rotor_scalar_geometric_product(scalar_rotor_geometric_product(self_, other), scalar_reversal(self_));
}

fn scalar_powi(self_: Scalar, exponent: int) -> Scalar  {
    if (exponent == 0) {
        return scalar_one();
    }
    let x: Scalar = select(self_, scalar_inverse(self_), exponent < 0);
    let y: Scalar = scalar_one();
    let n: int = abs(exponent);
    while (1 < n) {
        if ((n & 1) == 1) {
            let y = scalar_scalar_geometric_product(x, y);
        }
        let x = scalar_scalar_geometric_product(x, x);
        let n = n >> 1;
    }
    return scalar_scalar_geometric_product(x, y);
}

fn scalar_scalar_geometric_quotient(self_: Scalar, other: Scalar) -> Scalar  {
    return scalar_scalar_geometric_product(self_, scalar_inverse(other));
}

fn scalar_scalar_transformation(self_: Scalar, other: Scalar) -> Scalar  {
    return scalar_scalar_geometric_product(scalar_scalar_geometric_product(self_, other), scalar_reversal(self_));
}

fn scalar_translator_geometric_quotient(self_: Scalar, other: Translator) -> Translator  {
    return scalar_translator_geometric_product(self_, translator_inverse(other));
}

fn scalar_translator_transformation(self_: Scalar, other: Translator) -> Translator  {
    return translator_scalar_geometric_product(scalar_translator_geometric_product(self_, other), scalar_reversal(self_));
}

fn translator_line_geometric_quotient(self_: Translator, other: Line) -> Motor  {
    return translator_line_geometric_product(self_, line_inverse(other));
}

fn translator_line_transformation(self_: Translator, other: Line) -> Line  {
    return motor_line_into(motor_translator_geometric_product(translator_line_geometric_product(self_, other), translator_reversal(self_)));
}

fn translator_motor_geometric_quotient(self_: Translator, other: Motor) -> Motor  {
    return translator_motor_geometric_product(self_, motor_inverse(other));
}

fn translator_motor_transformation(self_: Translator, other: Motor) -> Motor  {
    return motor_translator_geometric_product(translator_motor_geometric_product(self_, other), translator_reversal(self_));
}

fn translator_multi_vector_geometric_quotient(self_: Translator, other: MultiVector) -> MultiVector  {
    return translator_multi_vector_geometric_product(self_, multi_vector_inverse(other));
}

fn translator_multi_vector_transformation(self_: Translator, other: MultiVector) -> MultiVector  {
    return multi_vector_translator_geometric_product(translator_multi_vector_geometric_product(self_, other), translator_reversal(self_));
}

fn translator_point_and_plane_geometric_quotient(self_: Translator, other: PointAndPlane) -> PointAndPlane  {
    return translator_point_and_plane_geometric_product(self_, point_and_plane_inverse(other));
}

fn translator_point_and_plane_transformation(self_: Translator, other: PointAndPlane) -> PointAndPlane  {
    return point_and_plane_translator_geometric_product(translator_point_and_plane_geometric_product(self_, other), translator_reversal(self_));
}

fn translator_rotor_geometric_quotient(self_: Translator, other: Rotor) -> Motor  {
    return translator_rotor_geometric_product(self_, rotor_inverse(other));
}

fn translator_rotor_transformation(self_: Translator, other: Rotor) -> Rotor  {
    return motor_rotor_into(motor_translator_geometric_product(translator_rotor_geometric_product(self_, other), translator_reversal(self_)));
}

fn translator_scalar_geometric_quotient(self_: Translator, other: Scalar) -> Translator  {
    return translator_scalar_geometric_product(self_, scalar_inverse(other));
}


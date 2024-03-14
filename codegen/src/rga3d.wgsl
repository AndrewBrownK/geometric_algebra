struct Scalar {
    // 1
     g0: f32,
}

struct AntiScalar {
    // e1234
     g0: f32,
}

struct Magnitude {
    // 1, e1234
     g0: vec2<f32>,
}

struct Point {
    // e1, e2, e3, e4
     g0: vec4<f32>,
}

struct Origin {
    // e4
     g0: f32,
}

struct PointAtInfinity {
    // e1, e2, e3
     g0: vec3<f32>,
}

struct Line {
    // -e14, -e24, -e34
     g0: vec3<f32>,
    // e23, -e13, e12
     g1: vec3<f32>,
}

struct LineAtOrigin {
    // -e14, -e24, -e34
     g0: vec3<f32>,
}

struct LineAtInfinity {
    // e23, -e13, e12
     g0: vec3<f32>,
}

struct Plane {
    // e234, -e134, e124, -e123
     g0: vec4<f32>,
}

struct PlaneAtOrigin {
    // e234, -e134, e124
     g0: vec3<f32>,
}

struct Horizon {
    // -e123
     g0: f32,
}

struct Motor {
    // -e14, -e24, -e34, e1234
     g0: vec4<f32>,
    // e23, -e13, e12
     g1: vec3<f32>,
}

struct Rotor {
    // -e14, -e24, -e34, e1234
     g0: vec4<f32>,
}

struct Translator {
    // e23, -e13, e12, e1234
     g0: vec4<f32>,
}

struct Flector {
    // e1, e2, e3, e4
     g0: vec4<f32>,
    // e234, -e134, e124, -e123
     g1: vec4<f32>,
}

struct MultiVector {
    // 1, e1234
     g0: vec2<f32>,
    // e1, e2, e3, e4
     g1: vec4<f32>,
    // -e14, -e24, -e34
     g2: vec3<f32>,
    // e23, -e13, e12
     g3: vec3<f32>,
    // e234, -e134, e124, -e123
     g4: vec4<f32>,
}

fn anti_scalar_one() -> AntiScalar {
    return AntiScalar(0.0);
}

fn flector_one() -> Flector {
    return Flector(vec4<f32>(0.0), vec4<f32>(0.0));
}

fn horizon_one() -> Horizon {
    return Horizon(0.0);
}

fn line_one() -> Line {
    return Line(vec3<f32>(0.0), vec3<f32>(0.0));
}

fn line_at_infinity_one() -> LineAtInfinity {
    return LineAtInfinity(vec3<f32>(0.0));
}

fn line_at_origin_one() -> LineAtOrigin {
    return LineAtOrigin(vec3<f32>(0.0));
}

fn magnitude_one() -> Magnitude {
    return Magnitude(vec2<f32>(1.0, 0.0));
}

fn motor_one() -> Motor {
    return Motor(vec4<f32>(0.0), vec3<f32>(0.0));
}

fn multi_vector_one() -> MultiVector {
    return MultiVector(vec2<f32>(1.0, 0.0), vec4<f32>(0.0), vec3<f32>(0.0), vec3<f32>(0.0), vec4<f32>(0.0));
}

fn origin_one() -> Origin {
    return Origin(0.0);
}

fn plane_one() -> Plane {
    return Plane(vec4<f32>(0.0));
}

fn plane_at_origin_one() -> PlaneAtOrigin {
    return PlaneAtOrigin(vec3<f32>(0.0));
}

fn point_one() -> Point {
    return Point(vec4<f32>(0.0));
}

fn point_at_infinity_one() -> PointAtInfinity {
    return PointAtInfinity(vec3<f32>(0.0));
}

fn rotor_one() -> Rotor {
    return Rotor(vec4<f32>(0.0));
}

fn scalar_one() -> Scalar {
    return Scalar(1.0);
}

fn translator_one() -> Translator {
    return Translator(vec4<f32>(0.0));
}

fn anti_scalar_zero() -> AntiScalar {
    return AntiScalar(0.0);
}

fn flector_zero() -> Flector {
    return Flector(vec4<f32>(0.0), vec4<f32>(0.0));
}

fn horizon_zero() -> Horizon {
    return Horizon(0.0);
}

fn line_zero() -> Line {
    return Line(vec3<f32>(0.0), vec3<f32>(0.0));
}

fn line_at_infinity_zero() -> LineAtInfinity {
    return LineAtInfinity(vec3<f32>(0.0));
}

fn line_at_origin_zero() -> LineAtOrigin {
    return LineAtOrigin(vec3<f32>(0.0));
}

fn magnitude_zero() -> Magnitude {
    return Magnitude(vec2<f32>(0.0));
}

fn motor_zero() -> Motor {
    return Motor(vec4<f32>(0.0), vec3<f32>(0.0));
}

fn multi_vector_zero() -> MultiVector {
    return MultiVector(vec2<f32>(0.0), vec4<f32>(0.0), vec3<f32>(0.0), vec3<f32>(0.0), vec4<f32>(0.0));
}

fn origin_zero() -> Origin {
    return Origin(0.0);
}

fn plane_zero() -> Plane {
    return Plane(vec4<f32>(0.0));
}

fn plane_at_origin_zero() -> PlaneAtOrigin {
    return PlaneAtOrigin(vec3<f32>(0.0));
}

fn point_zero() -> Point {
    return Point(vec4<f32>(0.0));
}

fn point_at_infinity_zero() -> PointAtInfinity {
    return PointAtInfinity(vec3<f32>(0.0));
}

fn rotor_zero() -> Rotor {
    return Rotor(vec4<f32>(0.0));
}

fn scalar_zero() -> Scalar {
    return Scalar(0.0);
}

fn translator_zero() -> Translator {
    return Translator(vec4<f32>(0.0));
}

fn anti_scalar_neg(self_: AntiScalar) -> AntiScalar {
    return AntiScalar(self_.g0 * -1.0);
}

fn flector_neg(self_: Flector) -> Flector {
    return Flector(self_.g0 * vec4<f32>(-1.0), self_.g1 * vec4<f32>(1.0, -1.0, 1.0, -1.0));
}

fn horizon_neg(self_: Horizon) -> Horizon {
    return Horizon(self_.g0 * -1.0);
}

fn line_neg(self_: Line) -> Line {
    return Line(self_.g0 * vec3<f32>(-1.0, 1.0, -1.0), self_.g1 * vec3<f32>(-1.0));
}

fn line_at_infinity_neg(self_: LineAtInfinity) -> LineAtInfinity {
    return LineAtInfinity(self_.g0 * vec3<f32>(-1.0));
}

fn line_at_origin_neg(self_: LineAtOrigin) -> LineAtOrigin {
    return LineAtOrigin(self_.g0 * vec3<f32>(-1.0, 1.0, -1.0));
}

fn magnitude_neg(self_: Magnitude) -> Magnitude {
    return Magnitude(self_.g0 * vec2<f32>(-1.0));
}

fn motor_neg(self_: Motor) -> Motor {
    return Motor(self_.g0 * vec4<f32>(-1.0, 1.0, -1.0, -1.0), self_.g1 * vec3<f32>(-1.0));
}

fn multi_vector_neg(self_: MultiVector) -> MultiVector {
    return MultiVector(self_.g0 * vec2<f32>(-1.0), self_.g1 * vec4<f32>(-1.0), self_.g2 * vec3<f32>(-1.0, 1.0, -1.0), self_.g3 * vec3<f32>(-1.0), self_.g4 * vec4<f32>(1.0, -1.0, 1.0, -1.0));
}

fn origin_neg(self_: Origin) -> Origin {
    return Origin(self_.g0 * -1.0);
}

fn plane_neg(self_: Plane) -> Plane {
    return Plane(self_.g0 * vec4<f32>(1.0, -1.0, 1.0, -1.0));
}

fn plane_at_origin_neg(self_: PlaneAtOrigin) -> PlaneAtOrigin {
    return PlaneAtOrigin(self_.g0 * vec3<f32>(1.0, -1.0, 1.0));
}

fn point_neg(self_: Point) -> Point {
    return Point(self_.g0 * vec4<f32>(-1.0));
}

fn point_at_infinity_neg(self_: PointAtInfinity) -> PointAtInfinity {
    return PointAtInfinity(self_.g0 * vec3<f32>(-1.0));
}

fn rotor_neg(self_: Rotor) -> Rotor {
    return Rotor(self_.g0 * vec4<f32>(-1.0, 1.0, -1.0, -1.0));
}

fn scalar_neg(self_: Scalar) -> Scalar {
    return Scalar(self_.g0 * -1.0);
}

fn translator_neg(self_: Translator) -> Translator {
    return Translator(self_.g0 * vec4<f32>(-1.0));
}

fn anti_scalar_anti_scalar_add(self_: AntiScalar, other: AntiScalar) -> AntiScalar {
    return AntiScalar(self_.g0 + other.g0);
}

fn anti_scalar_line_add(self_: AntiScalar, other: Line) -> Motor {
    return Motor(vec4<f32>(self_.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0), other.g1);
}

fn anti_scalar_line_at_infinity_add(self_: AntiScalar, other: LineAtInfinity) -> Translator {
    return Translator(vec4<f32>(self_.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0));
}

fn anti_scalar_line_at_origin_add(self_: AntiScalar, other: LineAtOrigin) -> Rotor {
    return Rotor(vec4<f32>(self_.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0));
}

fn anti_scalar_magnitude_add(self_: AntiScalar, other: Magnitude) -> Magnitude {
    return Magnitude(vec2<f32>(self_.g0) * vec2<f32>(0.0, 1.0) + other.g0);
}

fn anti_scalar_motor_add(self_: AntiScalar, other: Motor) -> Motor {
    return Motor(vec4<f32>(self_.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + other.g0, other.g1);
}

fn anti_scalar_multi_vector_add(self_: AntiScalar, other: MultiVector) -> MultiVector {
    return MultiVector(vec2<f32>(self_.g0) * vec2<f32>(0.0, 1.0) + other.g0, other.g1, other.g2, other.g3, other.g4);
}

fn anti_scalar_rotor_add(self_: AntiScalar, other: Rotor) -> Rotor {
    return Rotor(vec4<f32>(self_.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + other.g0);
}

fn anti_scalar_scalar_add(self_: AntiScalar, other: Scalar) -> Magnitude {
    return Magnitude(vec2<f32>(self_.g0) * vec2<f32>(0.0, 1.0) + vec2<f32>(other.g0) * vec2<f32>(1.0, 0.0));
}

fn anti_scalar_translator_add(self_: AntiScalar, other: Translator) -> Translator {
    return Translator(vec4<f32>(self_.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + other.g0);
}

fn flector_flector_add(self_: Flector, other: Flector) -> Flector {
    return Flector(self_.g0 + other.g0, self_.g1 + other.g1);
}

fn flector_horizon_add(self_: Flector, other: Horizon) -> Flector {
    return Flector(self_.g0, self_.g1 + vec4<f32>(other.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0));
}

fn flector_multi_vector_add(self_: Flector, other: MultiVector) -> MultiVector {
    return MultiVector(other.g0, self_.g0 + other.g1, other.g2, other.g3, self_.g1 + other.g4);
}

fn flector_origin_add(self_: Flector, other: Origin) -> Flector {
    return Flector(self_.g0 + vec4<f32>(other.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0), self_.g1);
}

fn flector_plane_add(self_: Flector, other: Plane) -> Flector {
    return Flector(self_.g0, self_.g1 + other.g0);
}

fn flector_plane_at_origin_add(self_: Flector, other: PlaneAtOrigin) -> Flector {
    return Flector(self_.g0, self_.g1 + vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0));
}

fn flector_point_add(self_: Flector, other: Point) -> Flector {
    return Flector(self_.g0 + other.g0, self_.g1);
}

fn flector_point_at_infinity_add(self_: Flector, other: PointAtInfinity) -> Flector {
    return Flector(self_.g0 + vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0), self_.g1);
}

fn horizon_flector_add(self_: Horizon, other: Flector) -> Flector {
    return Flector(other.g0, vec4<f32>(self_.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + other.g1);
}

fn horizon_horizon_add(self_: Horizon, other: Horizon) -> Horizon {
    return Horizon(self_.g0 + other.g0);
}

fn horizon_multi_vector_add(self_: Horizon, other: MultiVector) -> MultiVector {
    return MultiVector(other.g0, other.g1, other.g2, other.g3, vec4<f32>(self_.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + other.g4);
}

fn horizon_plane_add(self_: Horizon, other: Plane) -> Plane {
    return Plane(vec4<f32>(self_.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + other.g0);
}

fn horizon_plane_at_origin_add(self_: Horizon, other: PlaneAtOrigin) -> Plane {
    return Plane(vec4<f32>(self_.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0));
}

fn line_anti_scalar_add(self_: Line, other: AntiScalar) -> Motor {
    return Motor(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0) + vec4<f32>(other.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0), self_.g1);
}

fn line_line_add(self_: Line, other: Line) -> Line {
    return Line(self_.g0 + other.g0, self_.g1 + other.g1);
}

fn line_line_at_infinity_add(self_: Line, other: LineAtInfinity) -> Line {
    return Line(self_.g0, self_.g1 + other.g0);
}

fn line_line_at_origin_add(self_: Line, other: LineAtOrigin) -> Line {
    return Line(self_.g0 + other.g0, self_.g1);
}

fn line_motor_add(self_: Line, other: Motor) -> Motor {
    return Motor(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0) + other.g0, self_.g1 + other.g1);
}

fn line_multi_vector_add(self_: Line, other: MultiVector) -> MultiVector {
    return MultiVector(other.g0, other.g1, self_.g0 + other.g2, self_.g1 + other.g3, other.g4);
}

fn line_rotor_add(self_: Line, other: Rotor) -> Motor {
    return Motor(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0) + other.g0, self_.g1);
}

fn line_translator_add(self_: Line, other: Translator) -> Motor {
    return Motor(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0) + other.g0.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0), self_.g1 + vec3<f32>(other.g0.x, other.g0.y, other.g0.z));
}

fn line_at_infinity_anti_scalar_add(self_: LineAtInfinity, other: AntiScalar) -> Translator {
    return Translator(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0) + vec4<f32>(other.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0));
}

fn line_at_infinity_line_add(self_: LineAtInfinity, other: Line) -> Line {
    return Line(other.g0, self_.g0 + other.g1);
}

fn line_at_infinity_line_at_infinity_add(self_: LineAtInfinity, other: LineAtInfinity) -> LineAtInfinity {
    return LineAtInfinity(self_.g0 + other.g0);
}

fn line_at_infinity_line_at_origin_add(self_: LineAtInfinity, other: LineAtOrigin) -> Line {
    return Line(other.g0, self_.g0);
}

fn line_at_infinity_motor_add(self_: LineAtInfinity, other: Motor) -> Motor {
    return Motor(other.g0, self_.g0 + other.g1);
}

fn line_at_infinity_multi_vector_add(self_: LineAtInfinity, other: MultiVector) -> MultiVector {
    return MultiVector(other.g0, other.g1, other.g2, self_.g0 + other.g3, other.g4);
}

fn line_at_infinity_rotor_add(self_: LineAtInfinity, other: Rotor) -> Motor {
    return Motor(other.g0, self_.g0);
}

fn line_at_infinity_translator_add(self_: LineAtInfinity, other: Translator) -> Translator {
    return Translator(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0) + other.g0);
}

fn line_at_origin_anti_scalar_add(self_: LineAtOrigin, other: AntiScalar) -> Rotor {
    return Rotor(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0) + vec4<f32>(other.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0));
}

fn line_at_origin_line_add(self_: LineAtOrigin, other: Line) -> Line {
    return Line(self_.g0 + other.g0, other.g1);
}

fn line_at_origin_line_at_infinity_add(self_: LineAtOrigin, other: LineAtInfinity) -> Line {
    return Line(self_.g0, other.g0);
}

fn line_at_origin_line_at_origin_add(self_: LineAtOrigin, other: LineAtOrigin) -> LineAtOrigin {
    return LineAtOrigin(self_.g0 + other.g0);
}

fn line_at_origin_motor_add(self_: LineAtOrigin, other: Motor) -> Motor {
    return Motor(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0) + other.g0, other.g1);
}

fn line_at_origin_multi_vector_add(self_: LineAtOrigin, other: MultiVector) -> MultiVector {
    return MultiVector(other.g0, other.g1, self_.g0 + other.g2, other.g3, other.g4);
}

fn line_at_origin_rotor_add(self_: LineAtOrigin, other: Rotor) -> Rotor {
    return Rotor(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0) + other.g0);
}

fn line_at_origin_translator_add(self_: LineAtOrigin, other: Translator) -> Motor {
    return Motor(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0) + other.g0.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0), vec3<f32>(other.g0.x, other.g0.y, other.g0.z));
}

fn magnitude_anti_scalar_add(self_: Magnitude, other: AntiScalar) -> Magnitude {
    return Magnitude(self_.g0 + vec2<f32>(other.g0) * vec2<f32>(0.0, 1.0));
}

fn magnitude_magnitude_add(self_: Magnitude, other: Magnitude) -> Magnitude {
    return Magnitude(self_.g0 + other.g0);
}

fn magnitude_multi_vector_add(self_: Magnitude, other: MultiVector) -> MultiVector {
    return MultiVector(self_.g0 + other.g0, other.g1, other.g2, other.g3, other.g4);
}

fn magnitude_scalar_add(self_: Magnitude, other: Scalar) -> Magnitude {
    return Magnitude(self_.g0 + vec2<f32>(other.g0) * vec2<f32>(1.0, 0.0));
}

fn motor_anti_scalar_add(self_: Motor, other: AntiScalar) -> Motor {
    return Motor(self_.g0 + vec4<f32>(other.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0), self_.g1);
}

fn motor_line_add(self_: Motor, other: Line) -> Motor {
    return Motor(self_.g0 + vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0), self_.g1 + other.g1);
}

fn motor_line_at_infinity_add(self_: Motor, other: LineAtInfinity) -> Motor {
    return Motor(self_.g0, self_.g1 + other.g0);
}

fn motor_line_at_origin_add(self_: Motor, other: LineAtOrigin) -> Motor {
    return Motor(self_.g0 + vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0), self_.g1);
}

fn motor_motor_add(self_: Motor, other: Motor) -> Motor {
    return Motor(self_.g0 + other.g0, self_.g1 + other.g1);
}

fn motor_multi_vector_add(self_: Motor, other: MultiVector) -> MultiVector {
    return MultiVector(vec2<f32>(self_.g0.x, self_.g0.w) * vec2<f32>(0.0, 1.0) + other.g0, other.g1, vec3<f32>(self_.g0.x, self_.g0.y, self_.g0.z) + other.g2, self_.g1 + other.g3, other.g4);
}

fn motor_rotor_add(self_: Motor, other: Rotor) -> Motor {
    return Motor(self_.g0 + other.g0, self_.g1);
}

fn motor_translator_add(self_: Motor, other: Translator) -> Motor {
    return Motor(self_.g0 + other.g0.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0), self_.g1 + vec3<f32>(other.g0.x, other.g0.y, other.g0.z));
}

fn multi_vector_anti_scalar_add(self_: MultiVector, other: AntiScalar) -> MultiVector {
    return MultiVector(self_.g0 + vec2<f32>(other.g0) * vec2<f32>(0.0, 1.0), self_.g1, self_.g2, self_.g3, self_.g4);
}

fn multi_vector_flector_add(self_: MultiVector, other: Flector) -> MultiVector {
    return MultiVector(self_.g0, self_.g1 + other.g0, self_.g2, self_.g3, self_.g4 + other.g1);
}

fn multi_vector_horizon_add(self_: MultiVector, other: Horizon) -> MultiVector {
    return MultiVector(self_.g0, self_.g1, self_.g2, self_.g3, self_.g4 + vec4<f32>(other.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0));
}

fn multi_vector_line_add(self_: MultiVector, other: Line) -> MultiVector {
    return MultiVector(self_.g0, self_.g1, self_.g2 + other.g0, self_.g3 + other.g1, self_.g4);
}

fn multi_vector_line_at_infinity_add(self_: MultiVector, other: LineAtInfinity) -> MultiVector {
    return MultiVector(self_.g0, self_.g1, self_.g2, self_.g3 + other.g0, self_.g4);
}

fn multi_vector_line_at_origin_add(self_: MultiVector, other: LineAtOrigin) -> MultiVector {
    return MultiVector(self_.g0, self_.g1, self_.g2 + other.g0, self_.g3, self_.g4);
}

fn multi_vector_magnitude_add(self_: MultiVector, other: Magnitude) -> MultiVector {
    return MultiVector(self_.g0 + other.g0, self_.g1, self_.g2, self_.g3, self_.g4);
}

fn multi_vector_motor_add(self_: MultiVector, other: Motor) -> MultiVector {
    return MultiVector(self_.g0 + vec2<f32>(other.g0.x, other.g0.w) * vec2<f32>(0.0, 1.0), self_.g1, self_.g2 + vec3<f32>(other.g0.x, other.g0.y, other.g0.z), self_.g3 + other.g1, self_.g4);
}

fn multi_vector_multi_vector_add(self_: MultiVector, other: MultiVector) -> MultiVector {
    return MultiVector(self_.g0 + other.g0, self_.g1 + other.g1, self_.g2 + other.g2, self_.g3 + other.g3, self_.g4 + other.g4);
}

fn multi_vector_origin_add(self_: MultiVector, other: Origin) -> MultiVector {
    return MultiVector(self_.g0, self_.g1 + vec4<f32>(other.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0), self_.g2, self_.g3, self_.g4);
}

fn multi_vector_plane_add(self_: MultiVector, other: Plane) -> MultiVector {
    return MultiVector(self_.g0, self_.g1, self_.g2, self_.g3, self_.g4 + other.g0);
}

fn multi_vector_plane_at_origin_add(self_: MultiVector, other: PlaneAtOrigin) -> MultiVector {
    return MultiVector(self_.g0, self_.g1, self_.g2, self_.g3, self_.g4 + vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0));
}

fn multi_vector_point_add(self_: MultiVector, other: Point) -> MultiVector {
    return MultiVector(self_.g0, self_.g1 + other.g0, self_.g2, self_.g3, self_.g4);
}

fn multi_vector_point_at_infinity_add(self_: MultiVector, other: PointAtInfinity) -> MultiVector {
    return MultiVector(self_.g0, self_.g1 + vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0), self_.g2, self_.g3, self_.g4);
}

fn multi_vector_rotor_add(self_: MultiVector, other: Rotor) -> MultiVector {
    return MultiVector(self_.g0 + vec2<f32>(other.g0.x, other.g0.w) * vec2<f32>(0.0, 1.0), self_.g1, self_.g2 + vec3<f32>(other.g0.x, other.g0.y, other.g0.z), self_.g3, self_.g4);
}

fn multi_vector_scalar_add(self_: MultiVector, other: Scalar) -> MultiVector {
    return MultiVector(self_.g0 + vec2<f32>(other.g0) * vec2<f32>(1.0, 0.0), self_.g1, self_.g2, self_.g3, self_.g4);
}

fn multi_vector_translator_add(self_: MultiVector, other: Translator) -> MultiVector {
    return MultiVector(self_.g0 + vec2<f32>(other.g0.x, other.g0.w) * vec2<f32>(0.0, 1.0), self_.g1, self_.g2, self_.g3 + vec3<f32>(other.g0.x, other.g0.y, other.g0.z), self_.g4);
}

fn origin_flector_add(self_: Origin, other: Flector) -> Flector {
    return Flector(vec4<f32>(self_.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + other.g0, other.g1);
}

fn origin_multi_vector_add(self_: Origin, other: MultiVector) -> MultiVector {
    return MultiVector(other.g0, vec4<f32>(self_.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + other.g1, other.g2, other.g3, other.g4);
}

fn origin_origin_add(self_: Origin, other: Origin) -> Origin {
    return Origin(self_.g0 + other.g0);
}

fn origin_point_add(self_: Origin, other: Point) -> Point {
    return Point(vec4<f32>(self_.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + other.g0);
}

fn origin_point_at_infinity_add(self_: Origin, other: PointAtInfinity) -> Point {
    return Point(vec4<f32>(self_.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0) + vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0));
}

fn plane_flector_add(self_: Plane, other: Flector) -> Flector {
    return Flector(other.g0, self_.g0 + other.g1);
}

fn plane_horizon_add(self_: Plane, other: Horizon) -> Plane {
    return Plane(self_.g0 + vec4<f32>(other.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0));
}

fn plane_multi_vector_add(self_: Plane, other: MultiVector) -> MultiVector {
    return MultiVector(other.g0, other.g1, other.g2, other.g3, self_.g0 + other.g4);
}

fn plane_plane_add(self_: Plane, other: Plane) -> Plane {
    return Plane(self_.g0 + other.g0);
}

fn plane_plane_at_origin_add(self_: Plane, other: PlaneAtOrigin) -> Plane {
    return Plane(self_.g0 + vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0));
}

fn plane_point_add(self_: Plane, other: Point) -> Flector {
    return Flector(other.g0, self_.g0);
}

fn plane_at_origin_flector_add(self_: PlaneAtOrigin, other: Flector) -> Flector {
    return Flector(other.g0, vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0) + other.g1);
}

fn plane_at_origin_horizon_add(self_: PlaneAtOrigin, other: Horizon) -> Plane {
    return Plane(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0) + vec4<f32>(other.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0));
}

fn plane_at_origin_multi_vector_add(self_: PlaneAtOrigin, other: MultiVector) -> MultiVector {
    return MultiVector(other.g0, other.g1, other.g2, other.g3, vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0) + other.g4);
}

fn plane_at_origin_plane_add(self_: PlaneAtOrigin, other: Plane) -> Plane {
    return Plane(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0) + other.g0);
}

fn plane_at_origin_plane_at_origin_add(self_: PlaneAtOrigin, other: PlaneAtOrigin) -> PlaneAtOrigin {
    return PlaneAtOrigin(self_.g0 + other.g0);
}

fn point_flector_add(self_: Point, other: Flector) -> Flector {
    return Flector(self_.g0 + other.g0, other.g1);
}

fn point_multi_vector_add(self_: Point, other: MultiVector) -> MultiVector {
    return MultiVector(other.g0, self_.g0 + other.g1, other.g2, other.g3, other.g4);
}

fn point_origin_add(self_: Point, other: Origin) -> Point {
    return Point(self_.g0 + vec4<f32>(other.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0));
}

fn point_plane_add(self_: Point, other: Plane) -> Flector {
    return Flector(self_.g0, other.g0);
}

fn point_point_add(self_: Point, other: Point) -> Point {
    return Point(self_.g0 + other.g0);
}

fn point_point_at_infinity_add(self_: Point, other: PointAtInfinity) -> Point {
    return Point(self_.g0 + vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0));
}

fn point_at_infinity_flector_add(self_: PointAtInfinity, other: Flector) -> Flector {
    return Flector(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0) + other.g0, other.g1);
}

fn point_at_infinity_multi_vector_add(self_: PointAtInfinity, other: MultiVector) -> MultiVector {
    return MultiVector(other.g0, vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0) + other.g1, other.g2, other.g3, other.g4);
}

fn point_at_infinity_origin_add(self_: PointAtInfinity, other: Origin) -> Point {
    return Point(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0) + vec4<f32>(other.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0));
}

fn point_at_infinity_point_add(self_: PointAtInfinity, other: Point) -> Point {
    return Point(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0) + other.g0);
}

fn point_at_infinity_point_at_infinity_add(self_: PointAtInfinity, other: PointAtInfinity) -> PointAtInfinity {
    return PointAtInfinity(self_.g0 + other.g0);
}

fn rotor_anti_scalar_add(self_: Rotor, other: AntiScalar) -> Rotor {
    return Rotor(self_.g0 + vec4<f32>(other.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0));
}

fn rotor_line_add(self_: Rotor, other: Line) -> Motor {
    return Motor(self_.g0 + vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0), other.g1);
}

fn rotor_line_at_infinity_add(self_: Rotor, other: LineAtInfinity) -> Motor {
    return Motor(self_.g0, other.g0);
}

fn rotor_line_at_origin_add(self_: Rotor, other: LineAtOrigin) -> Rotor {
    return Rotor(self_.g0 + vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0));
}

fn rotor_motor_add(self_: Rotor, other: Motor) -> Motor {
    return Motor(self_.g0 + other.g0, other.g1);
}

fn rotor_multi_vector_add(self_: Rotor, other: MultiVector) -> MultiVector {
    return MultiVector(vec2<f32>(self_.g0.x, self_.g0.w) * vec2<f32>(0.0, 1.0) + other.g0, other.g1, vec3<f32>(self_.g0.x, self_.g0.y, self_.g0.z) + other.g2, other.g3, other.g4);
}

fn rotor_rotor_add(self_: Rotor, other: Rotor) -> Rotor {
    return Rotor(self_.g0 + other.g0);
}

fn rotor_translator_add(self_: Rotor, other: Translator) -> Motor {
    return Motor(self_.g0 + other.g0.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0), vec3<f32>(other.g0.x, other.g0.y, other.g0.z));
}

fn scalar_anti_scalar_add(self_: Scalar, other: AntiScalar) -> Magnitude {
    return Magnitude(vec2<f32>(self_.g0) * vec2<f32>(1.0, 0.0) + vec2<f32>(other.g0) * vec2<f32>(0.0, 1.0));
}

fn scalar_magnitude_add(self_: Scalar, other: Magnitude) -> Magnitude {
    return Magnitude(vec2<f32>(self_.g0) * vec2<f32>(1.0, 0.0) + other.g0);
}

fn scalar_multi_vector_add(self_: Scalar, other: MultiVector) -> MultiVector {
    return MultiVector(vec2<f32>(self_.g0) * vec2<f32>(1.0, 0.0) + other.g0, other.g1, other.g2, other.g3, other.g4);
}

fn scalar_scalar_add(self_: Scalar, other: Scalar) -> Scalar {
    return Scalar(self_.g0 + other.g0);
}

fn translator_anti_scalar_add(self_: Translator, other: AntiScalar) -> Translator {
    return Translator(self_.g0 + vec4<f32>(other.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0));
}

fn translator_line_add(self_: Translator, other: Line) -> Motor {
    return Motor(self_.g0.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0) + vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0), vec3<f32>(self_.g0.x, self_.g0.y, self_.g0.z) + other.g1);
}

fn translator_line_at_infinity_add(self_: Translator, other: LineAtInfinity) -> Translator {
    return Translator(self_.g0 + vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0));
}

fn translator_line_at_origin_add(self_: Translator, other: LineAtOrigin) -> Motor {
    return Motor(self_.g0.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0) + vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0), vec3<f32>(self_.g0.x, self_.g0.y, self_.g0.z));
}

fn translator_motor_add(self_: Translator, other: Motor) -> Motor {
    return Motor(self_.g0.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0) + other.g0, vec3<f32>(self_.g0.x, self_.g0.y, self_.g0.z) + other.g1);
}

fn translator_multi_vector_add(self_: Translator, other: MultiVector) -> MultiVector {
    return MultiVector(vec2<f32>(self_.g0.x, self_.g0.w) * vec2<f32>(0.0, 1.0) + other.g0, other.g1, other.g2, vec3<f32>(self_.g0.x, self_.g0.y, self_.g0.z) + other.g3, other.g4);
}

fn translator_rotor_add(self_: Translator, other: Rotor) -> Motor {
    return Motor(self_.g0.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0) + other.g0, vec3<f32>(self_.g0.x, self_.g0.y, self_.g0.z));
}

fn translator_translator_add(self_: Translator, other: Translator) -> Translator {
    return Translator(self_.g0 + other.g0);
}

fn anti_scalar_anti_scalar_div(self_: AntiScalar, other: AntiScalar) -> AntiScalar {
    return AntiScalar(self_.g0 * 1.0 / other.g0 * 1.0);
}

fn flector_flector_div(self_: Flector, other: Flector) -> Flector {
    return Flector(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0) / vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0), vec4<f32>(self_.g1.x, self_.g1.y, self_.g1.z, self_.g1.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0) / vec4<f32>(other.g1.x, other.g1.y, other.g1.z, other.g1.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0));
}

fn horizon_horizon_div(self_: Horizon, other: Horizon) -> Horizon {
    return Horizon(self_.g0 * 1.0 / other.g0 * 1.0);
}

fn line_line_div(self_: Line, other: Line) -> Line {
    return Line(vec3<f32>(self_.g0.x, self_.g0.y, self_.g0.z) * vec3<f32>(1.0, 1.0, 1.0) / vec3<f32>(other.g0.x, other.g0.y, other.g0.z) * vec3<f32>(1.0, 1.0, 1.0), vec3<f32>(self_.g1.x, self_.g1.y, self_.g1.z) * vec3<f32>(1.0, 1.0, 1.0) / vec3<f32>(other.g1.x, other.g1.y, other.g1.z) * vec3<f32>(1.0, 1.0, 1.0));
}

fn line_at_infinity_line_at_infinity_div(self_: LineAtInfinity, other: LineAtInfinity) -> LineAtInfinity {
    return LineAtInfinity(vec3<f32>(self_.g0.x, self_.g0.y, self_.g0.z) * vec3<f32>(1.0, 1.0, 1.0) / vec3<f32>(other.g0.x, other.g0.y, other.g0.z) * vec3<f32>(1.0, 1.0, 1.0));
}

fn line_at_origin_line_at_origin_div(self_: LineAtOrigin, other: LineAtOrigin) -> LineAtOrigin {
    return LineAtOrigin(vec3<f32>(self_.g0.x, self_.g0.y, self_.g0.z) * vec3<f32>(1.0, 1.0, 1.0) / vec3<f32>(other.g0.x, other.g0.y, other.g0.z) * vec3<f32>(1.0, 1.0, 1.0));
}

fn magnitude_magnitude_div(self_: Magnitude, other: Magnitude) -> Magnitude {
    return Magnitude(vec2<f32>(self_.g0.x, self_.g0.y) * vec2<f32>(1.0, 1.0) / vec2<f32>(other.g0.x, other.g0.y) * vec2<f32>(1.0, 1.0));
}

fn motor_motor_div(self_: Motor, other: Motor) -> Motor {
    return Motor(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0) / vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0), vec3<f32>(self_.g1.x, self_.g1.y, self_.g1.z) * vec3<f32>(1.0, 1.0, 1.0) / vec3<f32>(other.g1.x, other.g1.y, other.g1.z) * vec3<f32>(1.0, 1.0, 1.0));
}

fn multi_vector_multi_vector_div(self_: MultiVector, other: MultiVector) -> MultiVector {
    return MultiVector(vec2<f32>(self_.g0.x, self_.g0.y) * vec2<f32>(1.0, 1.0) / vec2<f32>(other.g0.x, other.g0.y) * vec2<f32>(1.0, 1.0), vec4<f32>(self_.g1.x, self_.g1.y, self_.g1.z, self_.g1.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0) / vec4<f32>(other.g1.x, other.g1.y, other.g1.z, other.g1.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0), vec3<f32>(self_.g2.x, self_.g2.y, self_.g2.z) * vec3<f32>(1.0, 1.0, 1.0) / vec3<f32>(other.g2.x, other.g2.y, other.g2.z) * vec3<f32>(1.0, 1.0, 1.0), vec3<f32>(self_.g3.x, self_.g3.y, self_.g3.z) * vec3<f32>(1.0, 1.0, 1.0) / vec3<f32>(other.g3.x, other.g3.y, other.g3.z) * vec3<f32>(1.0, 1.0, 1.0), vec4<f32>(self_.g4.x, self_.g4.y, self_.g4.z, self_.g4.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0) / vec4<f32>(other.g4.x, other.g4.y, other.g4.z, other.g4.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0));
}

fn origin_origin_div(self_: Origin, other: Origin) -> Origin {
    return Origin(self_.g0 * 1.0 / other.g0 * 1.0);
}

fn plane_plane_div(self_: Plane, other: Plane) -> Plane {
    return Plane(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0) / vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0));
}

fn plane_at_origin_plane_at_origin_div(self_: PlaneAtOrigin, other: PlaneAtOrigin) -> PlaneAtOrigin {
    return PlaneAtOrigin(vec3<f32>(self_.g0.x, self_.g0.y, self_.g0.z) * vec3<f32>(1.0, 1.0, 1.0) / vec3<f32>(other.g0.x, other.g0.y, other.g0.z) * vec3<f32>(1.0, 1.0, 1.0));
}

fn point_point_div(self_: Point, other: Point) -> Point {
    return Point(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0) / vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0));
}

fn point_at_infinity_point_at_infinity_div(self_: PointAtInfinity, other: PointAtInfinity) -> PointAtInfinity {
    return PointAtInfinity(vec3<f32>(self_.g0.x, self_.g0.y, self_.g0.z) * vec3<f32>(1.0, 1.0, 1.0) / vec3<f32>(other.g0.x, other.g0.y, other.g0.z) * vec3<f32>(1.0, 1.0, 1.0));
}

fn rotor_rotor_div(self_: Rotor, other: Rotor) -> Rotor {
    return Rotor(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0) / vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0));
}

fn scalar_scalar_div(self_: Scalar, other: Scalar) -> Scalar {
    return Scalar(self_.g0 * 1.0 / other.g0 * 1.0);
}

fn translator_translator_div(self_: Translator, other: Translator) -> Translator {
    return Translator(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0) / vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0));
}

fn flector_horizon_into(self_: Flector) -> Horizon {
    return Horizon(self_.g1.w);
}

fn flector_origin_into(self_: Flector) -> Origin {
    return Origin(self_.g0.w);
}

fn flector_plane_into(self_: Flector) -> Plane {
    return Plane(self_.g1);
}

fn flector_plane_at_origin_into(self_: Flector) -> PlaneAtOrigin {
    return PlaneAtOrigin(vec3<f32>(self_.g1.x, self_.g1.y, self_.g1.z));
}

fn flector_point_into(self_: Flector) -> Point {
    return Point(self_.g0);
}

fn flector_point_at_infinity_into(self_: Flector) -> PointAtInfinity {
    return PointAtInfinity(vec3<f32>(self_.g0.x, self_.g0.y, self_.g0.z));
}

fn line_line_at_infinity_into(self_: Line) -> LineAtInfinity {
    return LineAtInfinity(self_.g1);
}

fn line_line_at_origin_into(self_: Line) -> LineAtOrigin {
    return LineAtOrigin(self_.g0);
}

fn magnitude_anti_scalar_into(self_: Magnitude) -> AntiScalar {
    return AntiScalar(self_.g0.y);
}

fn magnitude_scalar_into(self_: Magnitude) -> Scalar {
    return Scalar(self_.g0.x);
}

fn motor_anti_scalar_into(self_: Motor) -> AntiScalar {
    return AntiScalar(self_.g0.w);
}

fn motor_line_into(self_: Motor) -> Line {
    return Line(vec3<f32>(self_.g0.x, self_.g0.y, self_.g0.z), self_.g1);
}

fn motor_line_at_infinity_into(self_: Motor) -> LineAtInfinity {
    return LineAtInfinity(self_.g1);
}

fn motor_line_at_origin_into(self_: Motor) -> LineAtOrigin {
    return LineAtOrigin(vec3<f32>(self_.g0.x, self_.g0.y, self_.g0.z));
}

fn motor_rotor_into(self_: Motor) -> Rotor {
    return Rotor(self_.g0);
}

fn motor_translator_into(self_: Motor) -> Translator {
    return Translator(vec4<f32>(self_.g1.x, self_.g1.y, self_.g1.z, self_.g0.w));
}

fn multi_vector_anti_scalar_into(self_: MultiVector) -> AntiScalar {
    return AntiScalar(self_.g0.y);
}

fn multi_vector_flector_into(self_: MultiVector) -> Flector {
    return Flector(self_.g1, self_.g4);
}

fn multi_vector_horizon_into(self_: MultiVector) -> Horizon {
    return Horizon(self_.g4.w);
}

fn multi_vector_line_into(self_: MultiVector) -> Line {
    return Line(self_.g2, self_.g3);
}

fn multi_vector_line_at_infinity_into(self_: MultiVector) -> LineAtInfinity {
    return LineAtInfinity(self_.g3);
}

fn multi_vector_line_at_origin_into(self_: MultiVector) -> LineAtOrigin {
    return LineAtOrigin(self_.g2);
}

fn multi_vector_magnitude_into(self_: MultiVector) -> Magnitude {
    return Magnitude(self_.g0);
}

fn multi_vector_motor_into(self_: MultiVector) -> Motor {
    return Motor(vec4<f32>(self_.g2.x, self_.g2.y, self_.g2.z, self_.g0.y), self_.g3);
}

fn multi_vector_origin_into(self_: MultiVector) -> Origin {
    return Origin(self_.g1.w);
}

fn multi_vector_plane_into(self_: MultiVector) -> Plane {
    return Plane(self_.g4);
}

fn multi_vector_plane_at_origin_into(self_: MultiVector) -> PlaneAtOrigin {
    return PlaneAtOrigin(vec3<f32>(self_.g4.x, self_.g4.y, self_.g4.z));
}

fn multi_vector_point_into(self_: MultiVector) -> Point {
    return Point(self_.g1);
}

fn multi_vector_point_at_infinity_into(self_: MultiVector) -> PointAtInfinity {
    return PointAtInfinity(vec3<f32>(self_.g1.x, self_.g1.y, self_.g1.z));
}

fn multi_vector_rotor_into(self_: MultiVector) -> Rotor {
    return Rotor(vec4<f32>(self_.g2.x, self_.g2.y, self_.g2.z, self_.g0.y));
}

fn multi_vector_scalar_into(self_: MultiVector) -> Scalar {
    return Scalar(self_.g0.x);
}

fn multi_vector_translator_into(self_: MultiVector) -> Translator {
    return Translator(vec4<f32>(self_.g3.x, self_.g3.y, self_.g3.z, self_.g0.y));
}

fn plane_horizon_into(self_: Plane) -> Horizon {
    return Horizon(self_.g0.w);
}

fn plane_plane_at_origin_into(self_: Plane) -> PlaneAtOrigin {
    return PlaneAtOrigin(vec3<f32>(self_.g0.x, self_.g0.y, self_.g0.z));
}

fn point_origin_into(self_: Point) -> Origin {
    return Origin(self_.g0.w);
}

fn point_point_at_infinity_into(self_: Point) -> PointAtInfinity {
    return PointAtInfinity(vec3<f32>(self_.g0.x, self_.g0.y, self_.g0.z));
}

fn rotor_anti_scalar_into(self_: Rotor) -> AntiScalar {
    return AntiScalar(self_.g0.w);
}

fn rotor_line_at_origin_into(self_: Rotor) -> LineAtOrigin {
    return LineAtOrigin(vec3<f32>(self_.g0.x, self_.g0.y, self_.g0.z));
}

fn translator_anti_scalar_into(self_: Translator) -> AntiScalar {
    return AntiScalar(self_.g0.w);
}

fn translator_line_at_infinity_into(self_: Translator) -> LineAtInfinity {
    return LineAtInfinity(vec3<f32>(self_.g0.x, self_.g0.y, self_.g0.z));
}

fn anti_scalar_anti_scalar_mul(self_: AntiScalar, other: AntiScalar) -> AntiScalar {
    return AntiScalar(self_.g0 * other.g0);
}

fn flector_flector_mul(self_: Flector, other: Flector) -> Flector {
    return Flector(self_.g0 * other.g0, self_.g1 * other.g1);
}

fn horizon_horizon_mul(self_: Horizon, other: Horizon) -> Horizon {
    return Horizon(self_.g0 * other.g0);
}

fn line_line_mul(self_: Line, other: Line) -> Line {
    return Line(self_.g0 * other.g0, self_.g1 * other.g1);
}

fn line_at_infinity_line_at_infinity_mul(self_: LineAtInfinity, other: LineAtInfinity) -> LineAtInfinity {
    return LineAtInfinity(self_.g0 * other.g0);
}

fn line_at_origin_line_at_origin_mul(self_: LineAtOrigin, other: LineAtOrigin) -> LineAtOrigin {
    return LineAtOrigin(self_.g0 * other.g0);
}

fn magnitude_magnitude_mul(self_: Magnitude, other: Magnitude) -> Magnitude {
    return Magnitude(self_.g0 * other.g0);
}

fn motor_motor_mul(self_: Motor, other: Motor) -> Motor {
    return Motor(self_.g0 * other.g0, self_.g1 * other.g1);
}

fn multi_vector_multi_vector_mul(self_: MultiVector, other: MultiVector) -> MultiVector {
    return MultiVector(self_.g0 * other.g0, self_.g1 * other.g1, self_.g2 * other.g2, self_.g3 * other.g3, self_.g4 * other.g4);
}

fn origin_origin_mul(self_: Origin, other: Origin) -> Origin {
    return Origin(self_.g0 * other.g0);
}

fn plane_plane_mul(self_: Plane, other: Plane) -> Plane {
    return Plane(self_.g0 * other.g0);
}

fn plane_at_origin_plane_at_origin_mul(self_: PlaneAtOrigin, other: PlaneAtOrigin) -> PlaneAtOrigin {
    return PlaneAtOrigin(self_.g0 * other.g0);
}

fn point_point_mul(self_: Point, other: Point) -> Point {
    return Point(self_.g0 * other.g0);
}

fn point_at_infinity_point_at_infinity_mul(self_: PointAtInfinity, other: PointAtInfinity) -> PointAtInfinity {
    return PointAtInfinity(self_.g0 * other.g0);
}

fn rotor_rotor_mul(self_: Rotor, other: Rotor) -> Rotor {
    return Rotor(self_.g0 * other.g0);
}

fn scalar_scalar_mul(self_: Scalar, other: Scalar) -> Scalar {
    return Scalar(self_.g0 * other.g0);
}

fn translator_translator_mul(self_: Translator, other: Translator) -> Translator {
    return Translator(self_.g0 * other.g0);
}

fn anti_scalar_anti_scalar_sub(self_: AntiScalar, other: AntiScalar) -> AntiScalar {
    return AntiScalar(self_.g0 - other.g0);
}

fn anti_scalar_line_sub(self_: AntiScalar, other: Line) -> Motor {
    return Motor(vec4<f32>(self_.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0) - vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0), vec3<f32>(0.0) - other.g1);
}

fn anti_scalar_line_at_infinity_sub(self_: AntiScalar, other: LineAtInfinity) -> Translator {
    return Translator(vec4<f32>(self_.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0) - vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0));
}

fn anti_scalar_line_at_origin_sub(self_: AntiScalar, other: LineAtOrigin) -> Rotor {
    return Rotor(vec4<f32>(self_.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0) - vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0));
}

fn anti_scalar_magnitude_sub(self_: AntiScalar, other: Magnitude) -> Magnitude {
    return Magnitude(vec2<f32>(self_.g0) * vec2<f32>(0.0, 1.0) - other.g0);
}

fn anti_scalar_motor_sub(self_: AntiScalar, other: Motor) -> Motor {
    return Motor(vec4<f32>(self_.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0) - other.g0, vec3<f32>(0.0) - other.g1);
}

fn anti_scalar_multi_vector_sub(self_: AntiScalar, other: MultiVector) -> MultiVector {
    return MultiVector(vec2<f32>(self_.g0) * vec2<f32>(0.0, 1.0) - other.g0, vec4<f32>(0.0) - other.g1, vec3<f32>(0.0) - other.g2, vec3<f32>(0.0) - other.g3, vec4<f32>(0.0) - other.g4);
}

fn anti_scalar_rotor_sub(self_: AntiScalar, other: Rotor) -> Rotor {
    return Rotor(vec4<f32>(self_.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0) - other.g0);
}

fn anti_scalar_scalar_sub(self_: AntiScalar, other: Scalar) -> Magnitude {
    return Magnitude(vec2<f32>(self_.g0) * vec2<f32>(0.0, 1.0) - vec2<f32>(other.g0) * vec2<f32>(1.0, 0.0));
}

fn anti_scalar_translator_sub(self_: AntiScalar, other: Translator) -> Translator {
    return Translator(vec4<f32>(self_.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0) - other.g0);
}

fn flector_flector_sub(self_: Flector, other: Flector) -> Flector {
    return Flector(self_.g0 - other.g0, self_.g1 - other.g1);
}

fn flector_horizon_sub(self_: Flector, other: Horizon) -> Flector {
    return Flector(self_.g0, self_.g1 - vec4<f32>(other.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0));
}

fn flector_multi_vector_sub(self_: Flector, other: MultiVector) -> MultiVector {
    return MultiVector(vec2<f32>(0.0) - other.g0, self_.g0 - other.g1, vec3<f32>(0.0) - other.g2, vec3<f32>(0.0) - other.g3, self_.g1 - other.g4);
}

fn flector_origin_sub(self_: Flector, other: Origin) -> Flector {
    return Flector(self_.g0 - vec4<f32>(other.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0), self_.g1);
}

fn flector_plane_sub(self_: Flector, other: Plane) -> Flector {
    return Flector(self_.g0, self_.g1 - other.g0);
}

fn flector_plane_at_origin_sub(self_: Flector, other: PlaneAtOrigin) -> Flector {
    return Flector(self_.g0, self_.g1 - vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0));
}

fn flector_point_sub(self_: Flector, other: Point) -> Flector {
    return Flector(self_.g0 - other.g0, self_.g1);
}

fn flector_point_at_infinity_sub(self_: Flector, other: PointAtInfinity) -> Flector {
    return Flector(self_.g0 - vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0), self_.g1);
}

fn horizon_flector_sub(self_: Horizon, other: Flector) -> Flector {
    return Flector(vec4<f32>(0.0) - other.g0, vec4<f32>(self_.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0) - other.g1);
}

fn horizon_horizon_sub(self_: Horizon, other: Horizon) -> Horizon {
    return Horizon(self_.g0 - other.g0);
}

fn horizon_multi_vector_sub(self_: Horizon, other: MultiVector) -> MultiVector {
    return MultiVector(vec2<f32>(0.0) - other.g0, vec4<f32>(0.0) - other.g1, vec3<f32>(0.0) - other.g2, vec3<f32>(0.0) - other.g3, vec4<f32>(self_.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0) - other.g4);
}

fn horizon_plane_sub(self_: Horizon, other: Plane) -> Plane {
    return Plane(vec4<f32>(self_.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0) - other.g0);
}

fn horizon_plane_at_origin_sub(self_: Horizon, other: PlaneAtOrigin) -> Plane {
    return Plane(vec4<f32>(self_.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0) - vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0));
}

fn line_anti_scalar_sub(self_: Line, other: AntiScalar) -> Motor {
    return Motor(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0) - vec4<f32>(other.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0), self_.g1);
}

fn line_line_sub(self_: Line, other: Line) -> Line {
    return Line(self_.g0 - other.g0, self_.g1 - other.g1);
}

fn line_line_at_infinity_sub(self_: Line, other: LineAtInfinity) -> Line {
    return Line(self_.g0, self_.g1 - other.g0);
}

fn line_line_at_origin_sub(self_: Line, other: LineAtOrigin) -> Line {
    return Line(self_.g0 - other.g0, self_.g1);
}

fn line_motor_sub(self_: Line, other: Motor) -> Motor {
    return Motor(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0) - other.g0, self_.g1 - other.g1);
}

fn line_multi_vector_sub(self_: Line, other: MultiVector) -> MultiVector {
    return MultiVector(vec2<f32>(0.0) - other.g0, vec4<f32>(0.0) - other.g1, self_.g0 - other.g2, self_.g1 - other.g3, vec4<f32>(0.0) - other.g4);
}

fn line_rotor_sub(self_: Line, other: Rotor) -> Motor {
    return Motor(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0) - other.g0, self_.g1);
}

fn line_translator_sub(self_: Line, other: Translator) -> Motor {
    return Motor(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0) - other.g0.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0), self_.g1 - vec3<f32>(other.g0.x, other.g0.y, other.g0.z));
}

fn line_at_infinity_anti_scalar_sub(self_: LineAtInfinity, other: AntiScalar) -> Translator {
    return Translator(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0) - vec4<f32>(other.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0));
}

fn line_at_infinity_line_sub(self_: LineAtInfinity, other: Line) -> Line {
    return Line(vec3<f32>(0.0) - other.g0, self_.g0 - other.g1);
}

fn line_at_infinity_line_at_infinity_sub(self_: LineAtInfinity, other: LineAtInfinity) -> LineAtInfinity {
    return LineAtInfinity(self_.g0 - other.g0);
}

fn line_at_infinity_line_at_origin_sub(self_: LineAtInfinity, other: LineAtOrigin) -> Line {
    return Line(vec3<f32>(0.0) - other.g0, self_.g0);
}

fn line_at_infinity_motor_sub(self_: LineAtInfinity, other: Motor) -> Motor {
    return Motor(vec4<f32>(0.0) - other.g0, self_.g0 - other.g1);
}

fn line_at_infinity_multi_vector_sub(self_: LineAtInfinity, other: MultiVector) -> MultiVector {
    return MultiVector(vec2<f32>(0.0) - other.g0, vec4<f32>(0.0) - other.g1, vec3<f32>(0.0) - other.g2, self_.g0 - other.g3, vec4<f32>(0.0) - other.g4);
}

fn line_at_infinity_rotor_sub(self_: LineAtInfinity, other: Rotor) -> Motor {
    return Motor(vec4<f32>(0.0) - other.g0, self_.g0);
}

fn line_at_infinity_translator_sub(self_: LineAtInfinity, other: Translator) -> Translator {
    return Translator(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0) - other.g0);
}

fn line_at_origin_anti_scalar_sub(self_: LineAtOrigin, other: AntiScalar) -> Rotor {
    return Rotor(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0) - vec4<f32>(other.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0));
}

fn line_at_origin_line_sub(self_: LineAtOrigin, other: Line) -> Line {
    return Line(self_.g0 - other.g0, vec3<f32>(0.0) - other.g1);
}

fn line_at_origin_line_at_infinity_sub(self_: LineAtOrigin, other: LineAtInfinity) -> Line {
    return Line(self_.g0, vec3<f32>(0.0) - other.g0);
}

fn line_at_origin_line_at_origin_sub(self_: LineAtOrigin, other: LineAtOrigin) -> LineAtOrigin {
    return LineAtOrigin(self_.g0 - other.g0);
}

fn line_at_origin_motor_sub(self_: LineAtOrigin, other: Motor) -> Motor {
    return Motor(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0) - other.g0, vec3<f32>(0.0) - other.g1);
}

fn line_at_origin_multi_vector_sub(self_: LineAtOrigin, other: MultiVector) -> MultiVector {
    return MultiVector(vec2<f32>(0.0) - other.g0, vec4<f32>(0.0) - other.g1, self_.g0 - other.g2, vec3<f32>(0.0) - other.g3, vec4<f32>(0.0) - other.g4);
}

fn line_at_origin_rotor_sub(self_: LineAtOrigin, other: Rotor) -> Rotor {
    return Rotor(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0) - other.g0);
}

fn line_at_origin_translator_sub(self_: LineAtOrigin, other: Translator) -> Motor {
    return Motor(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0) - other.g0.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0), vec3<f32>(0.0) - vec3<f32>(other.g0.x, other.g0.y, other.g0.z));
}

fn magnitude_anti_scalar_sub(self_: Magnitude, other: AntiScalar) -> Magnitude {
    return Magnitude(self_.g0 - vec2<f32>(other.g0) * vec2<f32>(0.0, 1.0));
}

fn magnitude_magnitude_sub(self_: Magnitude, other: Magnitude) -> Magnitude {
    return Magnitude(self_.g0 - other.g0);
}

fn magnitude_multi_vector_sub(self_: Magnitude, other: MultiVector) -> MultiVector {
    return MultiVector(self_.g0 - other.g0, vec4<f32>(0.0) - other.g1, vec3<f32>(0.0) - other.g2, vec3<f32>(0.0) - other.g3, vec4<f32>(0.0) - other.g4);
}

fn magnitude_scalar_sub(self_: Magnitude, other: Scalar) -> Magnitude {
    return Magnitude(self_.g0 - vec2<f32>(other.g0) * vec2<f32>(1.0, 0.0));
}

fn motor_anti_scalar_sub(self_: Motor, other: AntiScalar) -> Motor {
    return Motor(self_.g0 - vec4<f32>(other.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0), self_.g1);
}

fn motor_line_sub(self_: Motor, other: Line) -> Motor {
    return Motor(self_.g0 - vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0), self_.g1 - other.g1);
}

fn motor_line_at_infinity_sub(self_: Motor, other: LineAtInfinity) -> Motor {
    return Motor(self_.g0, self_.g1 - other.g0);
}

fn motor_line_at_origin_sub(self_: Motor, other: LineAtOrigin) -> Motor {
    return Motor(self_.g0 - vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0), self_.g1);
}

fn motor_motor_sub(self_: Motor, other: Motor) -> Motor {
    return Motor(self_.g0 - other.g0, self_.g1 - other.g1);
}

fn motor_multi_vector_sub(self_: Motor, other: MultiVector) -> MultiVector {
    return MultiVector(vec2<f32>(self_.g0.x, self_.g0.w) * vec2<f32>(0.0, 1.0) - other.g0, vec4<f32>(0.0) - other.g1, vec3<f32>(self_.g0.x, self_.g0.y, self_.g0.z) - other.g2, self_.g1 - other.g3, vec4<f32>(0.0) - other.g4);
}

fn motor_rotor_sub(self_: Motor, other: Rotor) -> Motor {
    return Motor(self_.g0 - other.g0, self_.g1);
}

fn motor_translator_sub(self_: Motor, other: Translator) -> Motor {
    return Motor(self_.g0 - other.g0.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0), self_.g1 - vec3<f32>(other.g0.x, other.g0.y, other.g0.z));
}

fn multi_vector_anti_scalar_sub(self_: MultiVector, other: AntiScalar) -> MultiVector {
    return MultiVector(self_.g0 - vec2<f32>(other.g0) * vec2<f32>(0.0, 1.0), self_.g1, self_.g2, self_.g3, self_.g4);
}

fn multi_vector_flector_sub(self_: MultiVector, other: Flector) -> MultiVector {
    return MultiVector(self_.g0, self_.g1 - other.g0, self_.g2, self_.g3, self_.g4 - other.g1);
}

fn multi_vector_horizon_sub(self_: MultiVector, other: Horizon) -> MultiVector {
    return MultiVector(self_.g0, self_.g1, self_.g2, self_.g3, self_.g4 - vec4<f32>(other.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0));
}

fn multi_vector_line_sub(self_: MultiVector, other: Line) -> MultiVector {
    return MultiVector(self_.g0, self_.g1, self_.g2 - other.g0, self_.g3 - other.g1, self_.g4);
}

fn multi_vector_line_at_infinity_sub(self_: MultiVector, other: LineAtInfinity) -> MultiVector {
    return MultiVector(self_.g0, self_.g1, self_.g2, self_.g3 - other.g0, self_.g4);
}

fn multi_vector_line_at_origin_sub(self_: MultiVector, other: LineAtOrigin) -> MultiVector {
    return MultiVector(self_.g0, self_.g1, self_.g2 - other.g0, self_.g3, self_.g4);
}

fn multi_vector_magnitude_sub(self_: MultiVector, other: Magnitude) -> MultiVector {
    return MultiVector(self_.g0 - other.g0, self_.g1, self_.g2, self_.g3, self_.g4);
}

fn multi_vector_motor_sub(self_: MultiVector, other: Motor) -> MultiVector {
    return MultiVector(self_.g0 - vec2<f32>(other.g0.x, other.g0.w) * vec2<f32>(0.0, 1.0), self_.g1, self_.g2 - vec3<f32>(other.g0.x, other.g0.y, other.g0.z), self_.g3 - other.g1, self_.g4);
}

fn multi_vector_multi_vector_sub(self_: MultiVector, other: MultiVector) -> MultiVector {
    return MultiVector(self_.g0 - other.g0, self_.g1 - other.g1, self_.g2 - other.g2, self_.g3 - other.g3, self_.g4 - other.g4);
}

fn multi_vector_origin_sub(self_: MultiVector, other: Origin) -> MultiVector {
    return MultiVector(self_.g0, self_.g1 - vec4<f32>(other.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0), self_.g2, self_.g3, self_.g4);
}

fn multi_vector_plane_sub(self_: MultiVector, other: Plane) -> MultiVector {
    return MultiVector(self_.g0, self_.g1, self_.g2, self_.g3, self_.g4 - other.g0);
}

fn multi_vector_plane_at_origin_sub(self_: MultiVector, other: PlaneAtOrigin) -> MultiVector {
    return MultiVector(self_.g0, self_.g1, self_.g2, self_.g3, self_.g4 - vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0));
}

fn multi_vector_point_sub(self_: MultiVector, other: Point) -> MultiVector {
    return MultiVector(self_.g0, self_.g1 - other.g0, self_.g2, self_.g3, self_.g4);
}

fn multi_vector_point_at_infinity_sub(self_: MultiVector, other: PointAtInfinity) -> MultiVector {
    return MultiVector(self_.g0, self_.g1 - vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0), self_.g2, self_.g3, self_.g4);
}

fn multi_vector_rotor_sub(self_: MultiVector, other: Rotor) -> MultiVector {
    return MultiVector(self_.g0 - vec2<f32>(other.g0.x, other.g0.w) * vec2<f32>(0.0, 1.0), self_.g1, self_.g2 - vec3<f32>(other.g0.x, other.g0.y, other.g0.z), self_.g3, self_.g4);
}

fn multi_vector_scalar_sub(self_: MultiVector, other: Scalar) -> MultiVector {
    return MultiVector(self_.g0 - vec2<f32>(other.g0) * vec2<f32>(1.0, 0.0), self_.g1, self_.g2, self_.g3, self_.g4);
}

fn multi_vector_translator_sub(self_: MultiVector, other: Translator) -> MultiVector {
    return MultiVector(self_.g0 - vec2<f32>(other.g0.x, other.g0.w) * vec2<f32>(0.0, 1.0), self_.g1, self_.g2, self_.g3 - vec3<f32>(other.g0.x, other.g0.y, other.g0.z), self_.g4);
}

fn origin_flector_sub(self_: Origin, other: Flector) -> Flector {
    return Flector(vec4<f32>(self_.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0) - other.g0, vec4<f32>(0.0) - other.g1);
}

fn origin_multi_vector_sub(self_: Origin, other: MultiVector) -> MultiVector {
    return MultiVector(vec2<f32>(0.0) - other.g0, vec4<f32>(self_.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0) - other.g1, vec3<f32>(0.0) - other.g2, vec3<f32>(0.0) - other.g3, vec4<f32>(0.0) - other.g4);
}

fn origin_origin_sub(self_: Origin, other: Origin) -> Origin {
    return Origin(self_.g0 - other.g0);
}

fn origin_point_sub(self_: Origin, other: Point) -> Point {
    return Point(vec4<f32>(self_.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0) - other.g0);
}

fn origin_point_at_infinity_sub(self_: Origin, other: PointAtInfinity) -> Point {
    return Point(vec4<f32>(self_.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0) - vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0));
}

fn plane_flector_sub(self_: Plane, other: Flector) -> Flector {
    return Flector(vec4<f32>(0.0) - other.g0, self_.g0 - other.g1);
}

fn plane_horizon_sub(self_: Plane, other: Horizon) -> Plane {
    return Plane(self_.g0 - vec4<f32>(other.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0));
}

fn plane_multi_vector_sub(self_: Plane, other: MultiVector) -> MultiVector {
    return MultiVector(vec2<f32>(0.0) - other.g0, vec4<f32>(0.0) - other.g1, vec3<f32>(0.0) - other.g2, vec3<f32>(0.0) - other.g3, self_.g0 - other.g4);
}

fn plane_plane_sub(self_: Plane, other: Plane) -> Plane {
    return Plane(self_.g0 - other.g0);
}

fn plane_plane_at_origin_sub(self_: Plane, other: PlaneAtOrigin) -> Plane {
    return Plane(self_.g0 - vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0));
}

fn plane_point_sub(self_: Plane, other: Point) -> Flector {
    return Flector(vec4<f32>(0.0) - other.g0, self_.g0);
}

fn plane_at_origin_flector_sub(self_: PlaneAtOrigin, other: Flector) -> Flector {
    return Flector(vec4<f32>(0.0) - other.g0, vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0) - other.g1);
}

fn plane_at_origin_horizon_sub(self_: PlaneAtOrigin, other: Horizon) -> Plane {
    return Plane(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0) - vec4<f32>(other.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0));
}

fn plane_at_origin_multi_vector_sub(self_: PlaneAtOrigin, other: MultiVector) -> MultiVector {
    return MultiVector(vec2<f32>(0.0) - other.g0, vec4<f32>(0.0) - other.g1, vec3<f32>(0.0) - other.g2, vec3<f32>(0.0) - other.g3, vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0) - other.g4);
}

fn plane_at_origin_plane_sub(self_: PlaneAtOrigin, other: Plane) -> Plane {
    return Plane(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0) - other.g0);
}

fn plane_at_origin_plane_at_origin_sub(self_: PlaneAtOrigin, other: PlaneAtOrigin) -> PlaneAtOrigin {
    return PlaneAtOrigin(self_.g0 - other.g0);
}

fn point_flector_sub(self_: Point, other: Flector) -> Flector {
    return Flector(self_.g0 - other.g0, vec4<f32>(0.0) - other.g1);
}

fn point_multi_vector_sub(self_: Point, other: MultiVector) -> MultiVector {
    return MultiVector(vec2<f32>(0.0) - other.g0, self_.g0 - other.g1, vec3<f32>(0.0) - other.g2, vec3<f32>(0.0) - other.g3, vec4<f32>(0.0) - other.g4);
}

fn point_origin_sub(self_: Point, other: Origin) -> Point {
    return Point(self_.g0 - vec4<f32>(other.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0));
}

fn point_plane_sub(self_: Point, other: Plane) -> Flector {
    return Flector(self_.g0, vec4<f32>(0.0) - other.g0);
}

fn point_point_sub(self_: Point, other: Point) -> Point {
    return Point(self_.g0 - other.g0);
}

fn point_point_at_infinity_sub(self_: Point, other: PointAtInfinity) -> Point {
    return Point(self_.g0 - vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0));
}

fn point_at_infinity_flector_sub(self_: PointAtInfinity, other: Flector) -> Flector {
    return Flector(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0) - other.g0, vec4<f32>(0.0) - other.g1);
}

fn point_at_infinity_multi_vector_sub(self_: PointAtInfinity, other: MultiVector) -> MultiVector {
    return MultiVector(vec2<f32>(0.0) - other.g0, vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0) - other.g1, vec3<f32>(0.0) - other.g2, vec3<f32>(0.0) - other.g3, vec4<f32>(0.0) - other.g4);
}

fn point_at_infinity_origin_sub(self_: PointAtInfinity, other: Origin) -> Point {
    return Point(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0) - vec4<f32>(other.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0));
}

fn point_at_infinity_point_sub(self_: PointAtInfinity, other: Point) -> Point {
    return Point(vec4<f32>(self_.g0.x, self_.g0.y, self_.g0.z, self_.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0) - other.g0);
}

fn point_at_infinity_point_at_infinity_sub(self_: PointAtInfinity, other: PointAtInfinity) -> PointAtInfinity {
    return PointAtInfinity(self_.g0 - other.g0);
}

fn rotor_anti_scalar_sub(self_: Rotor, other: AntiScalar) -> Rotor {
    return Rotor(self_.g0 - vec4<f32>(other.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0));
}

fn rotor_line_sub(self_: Rotor, other: Line) -> Motor {
    return Motor(self_.g0 - vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0), vec3<f32>(0.0) - other.g1);
}

fn rotor_line_at_infinity_sub(self_: Rotor, other: LineAtInfinity) -> Motor {
    return Motor(self_.g0, vec3<f32>(0.0) - other.g0);
}

fn rotor_line_at_origin_sub(self_: Rotor, other: LineAtOrigin) -> Rotor {
    return Rotor(self_.g0 - vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0));
}

fn rotor_motor_sub(self_: Rotor, other: Motor) -> Motor {
    return Motor(self_.g0 - other.g0, vec3<f32>(0.0) - other.g1);
}

fn rotor_multi_vector_sub(self_: Rotor, other: MultiVector) -> MultiVector {
    return MultiVector(vec2<f32>(self_.g0.x, self_.g0.w) * vec2<f32>(0.0, 1.0) - other.g0, vec4<f32>(0.0) - other.g1, vec3<f32>(self_.g0.x, self_.g0.y, self_.g0.z) - other.g2, vec3<f32>(0.0) - other.g3, vec4<f32>(0.0) - other.g4);
}

fn rotor_rotor_sub(self_: Rotor, other: Rotor) -> Rotor {
    return Rotor(self_.g0 - other.g0);
}

fn rotor_translator_sub(self_: Rotor, other: Translator) -> Motor {
    return Motor(self_.g0 - other.g0.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0), vec3<f32>(0.0) - vec3<f32>(other.g0.x, other.g0.y, other.g0.z));
}

fn scalar_anti_scalar_sub(self_: Scalar, other: AntiScalar) -> Magnitude {
    return Magnitude(vec2<f32>(self_.g0) * vec2<f32>(1.0, 0.0) - vec2<f32>(other.g0) * vec2<f32>(0.0, 1.0));
}

fn scalar_magnitude_sub(self_: Scalar, other: Magnitude) -> Magnitude {
    return Magnitude(vec2<f32>(self_.g0) * vec2<f32>(1.0, 0.0) - other.g0);
}

fn scalar_multi_vector_sub(self_: Scalar, other: MultiVector) -> MultiVector {
    return MultiVector(vec2<f32>(self_.g0) * vec2<f32>(1.0, 0.0) - other.g0, vec4<f32>(0.0) - other.g1, vec3<f32>(0.0) - other.g2, vec3<f32>(0.0) - other.g3, vec4<f32>(0.0) - other.g4);
}

fn scalar_scalar_sub(self_: Scalar, other: Scalar) -> Scalar {
    return Scalar(self_.g0 - other.g0);
}

fn translator_anti_scalar_sub(self_: Translator, other: AntiScalar) -> Translator {
    return Translator(self_.g0 - vec4<f32>(other.g0) * vec4<f32>(0.0, 0.0, 0.0, 1.0));
}

fn translator_line_sub(self_: Translator, other: Line) -> Motor {
    return Motor(self_.g0.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0) - vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0), vec3<f32>(self_.g0.x, self_.g0.y, self_.g0.z) - other.g1);
}

fn translator_line_at_infinity_sub(self_: Translator, other: LineAtInfinity) -> Translator {
    return Translator(self_.g0 - vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0));
}

fn translator_line_at_origin_sub(self_: Translator, other: LineAtOrigin) -> Motor {
    return Motor(self_.g0.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0) - vec4<f32>(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0), vec3<f32>(self_.g0.x, self_.g0.y, self_.g0.z));
}

fn translator_motor_sub(self_: Translator, other: Motor) -> Motor {
    return Motor(self_.g0.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0) - other.g0, vec3<f32>(self_.g0.x, self_.g0.y, self_.g0.z) - other.g1);
}

fn translator_multi_vector_sub(self_: Translator, other: MultiVector) -> MultiVector {
    return MultiVector(vec2<f32>(self_.g0.x, self_.g0.w) * vec2<f32>(0.0, 1.0) - other.g0, vec4<f32>(0.0) - other.g1, vec3<f32>(0.0) - other.g2, vec3<f32>(self_.g0.x, self_.g0.y, self_.g0.z) - other.g3, vec4<f32>(0.0) - other.g4);
}

fn translator_rotor_sub(self_: Translator, other: Rotor) -> Motor {
    return Motor(self_.g0.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0) - other.g0, vec3<f32>(self_.g0.x, self_.g0.y, self_.g0.z));
}

fn translator_translator_sub(self_: Translator, other: Translator) -> Translator {
    return Translator(self_.g0 - other.g0);
}


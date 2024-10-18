#define_import_path rga3d
//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//
// v1.0.0
// Latest generation test case
// authors = ["Andrew Brown <Andrew.Brown.UNL@gmail.com>"]
// License: MIWT
//
struct AntiScalar {
    // e1234
    g0_: f32
}
struct DualNum {
    // scalar, e1234
    g0_: vec2<f32>
}
struct Flector {
    // e1, e2, e3, e4
    g0_: vec4<f32>,
    // e423, e431, e412, e321
    g1_: vec4<f32>
}
struct Horizon {
    // e321
    g0_: f32
}
struct Line {
    // e41, e42, e43
    g0_: vec3<f32>,
    // e23, e31, e12
    g1_: vec3<f32>
}
struct Motor {
    // e41, e42, e43, e1234
    g0_: vec4<f32>,
    // e23, e31, e12, scalar
    g1_: vec4<f32>
}
struct MultiVector {
    // scalar, e1234
    g0_: vec2<f32>,
    // e1, e2, e3, e4
    g1_: vec4<f32>,
    // e41, e42, e43
    g2_: vec3<f32>,
    // e23, e31, e12
    g3_: vec3<f32>,
    // e423, e431, e412, e321
    g4_: vec4<f32>
}
struct Origin {
    // e4
    g0_: f32
}
struct Plane {
    // e423, e431, e412, e321
    g0_: vec4<f32>
}
struct Point {
    // e1, e2, e3, e4
    g0_: vec4<f32>
}
struct Scalar {
    // scalar
    g0_: f32
}
fn antiScalar_add_antiScalar(self_: AntiScalar, other: AntiScalar) -> AntiScalar {
    return AntiScalar(/* e1234 */ (other.g0_ + self_.g0_));
}
fn antiScalar_add_dualNum(self_: AntiScalar, other: DualNum) -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>(other.g0_.x, (other.g0_.y + self_.g0_)));
}
fn antiScalar_add_flector(self_: AntiScalar, other: Flector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, self_.g0_), /* e1, e2, e3, e4 */ other.g0_, /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ other.g1_);
}
fn antiScalar_add_horizon(self_: AntiScalar, other: Horizon) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, self_.g0_), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other.g0_));
}
fn antiScalar_add_line(self_: AntiScalar, other: Line) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, self_.g0_), /* e23, e31, e12, scalar */ vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, 0.0));
}
fn antiScalar_add_motor(self_: AntiScalar, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, (other.g0_.w + self_.g0_)), /* e23, e31, e12, scalar */ other.g1_);
}
fn antiScalar_add_multiVector(self_: AntiScalar, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(other.g0_.x, (other.g0_.y + self_.g0_)), /* e1, e2, e3, e4 */ other.g1_, /* e41, e42, e43 */ other.g2_, /* e23, e31, e12 */ other.g3_, /* e423, e431, e412, e321 */ other.g4_);
}
fn antiScalar_add_origin(self_: AntiScalar, other: Origin) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, self_.g0_), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other.g0_), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn antiScalar_add_plane(self_: AntiScalar, other: Plane) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, self_.g0_), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ other.g0_);
}
fn antiScalar_add_point(self_: AntiScalar, other: Point) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, self_.g0_), /* e1, e2, e3, e4 */ other.g0_, /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn antiScalar_add_scalar(self_: AntiScalar, other: Scalar) -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>(other.g0_, self_.g0_));
}
fn dualNum_add_antiScalar(self_: DualNum, other: AntiScalar) -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>(self_.g0_.x, (self_.g0_.y + other.g0_)));
}
fn dualNum_add_dualNum(self_: DualNum, other: DualNum) -> DualNum {
    return DualNum(/* scalar, e1234 */ (other.g0_ + self_.g0_));
}
fn dualNum_add_flector(self_: DualNum, other: Flector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ other.g0_, /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ other.g1_);
}
fn dualNum_add_horizon(self_: DualNum, other: Horizon) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other.g0_));
}
fn dualNum_add_line(self_: DualNum, other: Line) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, self_.g0_.y), /* e23, e31, e12, scalar */ vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, self_.g0_.x));
}
fn dualNum_add_motor(self_: DualNum, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, (self_.g0_.y + other.g0_.w)), /* e23, e31, e12, scalar */ vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, (self_.g0_.x + other.g1_.w)));
}
fn dualNum_add_multiVector(self_: DualNum, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (self_.g0_ + other.g0_), /* e1, e2, e3, e4 */ other.g1_, /* e41, e42, e43 */ other.g2_, /* e23, e31, e12 */ other.g3_, /* e423, e431, e412, e321 */ other.g4_);
}
fn dualNum_add_origin(self_: DualNum, other: Origin) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other.g0_), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn dualNum_add_plane(self_: DualNum, other: Plane) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ other.g0_);
}
fn dualNum_add_point(self_: DualNum, other: Point) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ other.g0_, /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn dualNum_add_scalar(self_: DualNum, other: Scalar) -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>((self_.g0_.x + other.g0_), self_.g0_.y));
}
fn flector_add_antiScalar(self_: Flector, other: AntiScalar) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, other.g0_), /* e1, e2, e3, e4 */ self_.g0_, /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ self_.g1_);
}
fn flector_add_dualNum(self_: Flector, other: DualNum) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ other.g0_, /* e1, e2, e3, e4 */ self_.g0_, /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ self_.g1_);
}
fn flector_add_flector(self_: Flector, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (other.g0_ + self_.g0_), /* e423, e431, e412, e321 */ (other.g1_ + self_.g1_));
}
fn flector_add_horizon(self_: Flector, other: Horizon) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ self_.g0_, /* e423, e431, e412, e321 */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, (self_.g1_.w + other.g0_)));
}
fn flector_add_line(self_: Flector, other: Line) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ self_.g0_, /* e41, e42, e43 */ other.g0_, /* e23, e31, e12 */ other.g1_, /* e423, e431, e412, e321 */ self_.g1_);
}
fn flector_add_motor(self_: Flector, other: Motor) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(other.g1_.w, other.g0_.w), /* e1, e2, e3, e4 */ self_.g0_, /* e41, e42, e43 */ vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z), /* e23, e31, e12 */ vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z), /* e423, e431, e412, e321 */ self_.g1_);
}
fn flector_add_multiVector(self_: Flector, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ other.g0_, /* e1, e2, e3, e4 */ (self_.g0_ + other.g1_), /* e41, e42, e43 */ other.g2_, /* e23, e31, e12 */ other.g3_, /* e423, e431, e412, e321 */ (self_.g1_ + other.g4_));
}
fn flector_add_origin(self_: Flector, other: Origin) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, (self_.g0_.w + other.g0_)), /* e423, e431, e412, e321 */ self_.g1_);
}
fn flector_add_plane(self_: Flector, other: Plane) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ self_.g0_, /* e423, e431, e412, e321 */ (self_.g1_ + other.g0_));
}
fn flector_add_point(self_: Flector, other: Point) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (self_.g0_ + other.g0_), /* e423, e431, e412, e321 */ self_.g1_);
}
fn flector_add_scalar(self_: Flector, other: Scalar) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(other.g0_, 0.0), /* e1, e2, e3, e4 */ self_.g0_, /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ self_.g1_);
}
fn horizon_add_antiScalar(self_: Horizon, other: AntiScalar) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, other.g0_), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_));
}
fn horizon_add_dualNum(self_: Horizon, other: DualNum) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ other.g0_, /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_));
}
fn horizon_add_flector(self_: Horizon, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ other.g0_, /* e423, e431, e412, e321 */ vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, (other.g1_.w + self_.g0_)));
}
fn horizon_add_horizon(self_: Horizon, other: Horizon) -> Horizon {
    return Horizon(/* e321 */ (other.g0_ + self_.g0_));
}
fn horizon_add_line(self_: Horizon, other: Line) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ other.g0_, /* e23, e31, e12 */ other.g1_, /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_));
}
fn horizon_add_motor(self_: Horizon, other: Motor) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(other.g1_.w, other.g0_.w), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z), /* e23, e31, e12 */ vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_));
}
fn horizon_add_multiVector(self_: Horizon, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ other.g0_, /* e1, e2, e3, e4 */ other.g1_, /* e41, e42, e43 */ other.g2_, /* e23, e31, e12 */ other.g3_, /* e423, e431, e412, e321 */ vec4<f32>(other.g4_.x, other.g4_.y, other.g4_.z, (other.g4_.w + self_.g0_)));
}
fn horizon_add_origin(self_: Horizon, other: Origin) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other.g0_), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_));
}
fn horizon_add_plane(self_: Horizon, other: Plane) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, (other.g0_.w + self_.g0_)));
}
fn horizon_add_point(self_: Horizon, other: Point) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ other.g0_, /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_));
}
fn horizon_add_scalar(self_: Horizon, other: Scalar) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(other.g0_, 0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_));
}
fn line_add_antiScalar(self_: Line, other: AntiScalar) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, other.g0_), /* e23, e31, e12, scalar */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, 0.0));
}
fn line_add_dualNum(self_: Line, other: DualNum) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, other.g0_.y), /* e23, e31, e12, scalar */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, other.g0_.x));
}
fn line_add_flector(self_: Line, other: Flector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ other.g0_, /* e41, e42, e43 */ self_.g0_, /* e23, e31, e12 */ self_.g1_, /* e423, e431, e412, e321 */ other.g1_);
}
fn line_add_horizon(self_: Line, other: Horizon) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ self_.g0_, /* e23, e31, e12 */ self_.g1_, /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other.g0_));
}
fn line_add_line(self_: Line, other: Line) -> Line {
    return Line(/* e41, e42, e43 */ (other.g0_ + self_.g0_), /* e23, e31, e12 */ (other.g1_ + self_.g1_));
}
fn line_add_motor(self_: Line, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g0_.x + other.g0_.x), (self_.g0_.y + other.g0_.y), (self_.g0_.z + other.g0_.z), other.g0_.w), /* e23, e31, e12, scalar */ vec4<f32>((self_.g1_.x + other.g1_.x), (self_.g1_.y + other.g1_.y), (self_.g1_.z + other.g1_.z), other.g1_.w));
}
fn line_add_multiVector(self_: Line, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ other.g0_, /* e1, e2, e3, e4 */ other.g1_, /* e41, e42, e43 */ (self_.g0_ + other.g2_), /* e23, e31, e12 */ (self_.g1_ + other.g3_), /* e423, e431, e412, e321 */ other.g4_);
}
fn line_add_origin(self_: Line, other: Origin) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other.g0_), /* e41, e42, e43 */ self_.g0_, /* e23, e31, e12 */ self_.g1_, /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn line_add_plane(self_: Line, other: Plane) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ self_.g0_, /* e23, e31, e12 */ self_.g1_, /* e423, e431, e412, e321 */ other.g0_);
}
fn line_add_point(self_: Line, other: Point) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ other.g0_, /* e41, e42, e43 */ self_.g0_, /* e23, e31, e12 */ self_.g1_, /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn line_add_scalar(self_: Line, other: Scalar) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, 0.0), /* e23, e31, e12, scalar */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, other.g0_));
}
fn motor_add_antiScalar(self_: Motor, other: AntiScalar) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, (self_.g0_.w + other.g0_)), /* e23, e31, e12, scalar */ self_.g1_);
}
fn motor_add_dualNum(self_: Motor, other: DualNum) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, (other.g0_.y + self_.g0_.w)), /* e23, e31, e12, scalar */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, (other.g0_.x + self_.g1_.w)));
}
fn motor_add_flector(self_: Motor, other: Flector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g1_.w, self_.g0_.w), /* e1, e2, e3, e4 */ other.g0_, /* e41, e42, e43 */ vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z), /* e23, e31, e12 */ vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z), /* e423, e431, e412, e321 */ other.g1_);
}
fn motor_add_horizon(self_: Motor, other: Horizon) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g1_.w, self_.g0_.w), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z), /* e23, e31, e12 */ vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other.g0_));
}
fn motor_add_line(self_: Motor, other: Line) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g0_.x + self_.g0_.x), (other.g0_.y + self_.g0_.y), (other.g0_.z + self_.g0_.z), self_.g0_.w), /* e23, e31, e12, scalar */ vec4<f32>((other.g1_.x + self_.g1_.x), (other.g1_.y + self_.g1_.y), (other.g1_.z + self_.g1_.z), self_.g1_.w));
}
fn motor_add_motor(self_: Motor, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (other.g0_ + self_.g0_), /* e23, e31, e12, scalar */ (other.g1_ + self_.g1_));
}
fn motor_add_multiVector(self_: Motor, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(self_.g1_.w, self_.g0_.w) + other.g0_), /* e1, e2, e3, e4 */ other.g1_, /* e41, e42, e43 */ (vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z) + other.g2_), /* e23, e31, e12 */ (vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z) + other.g3_), /* e423, e431, e412, e321 */ other.g4_);
}
fn motor_add_origin(self_: Motor, other: Origin) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g1_.w, self_.g0_.w), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other.g0_), /* e41, e42, e43 */ vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z), /* e23, e31, e12 */ vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn motor_add_plane(self_: Motor, other: Plane) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g1_.w, self_.g0_.w), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z), /* e23, e31, e12 */ vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z), /* e423, e431, e412, e321 */ other.g0_);
}
fn motor_add_point(self_: Motor, other: Point) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g1_.w, self_.g0_.w), /* e1, e2, e3, e4 */ other.g0_, /* e41, e42, e43 */ vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z), /* e23, e31, e12 */ vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn motor_add_scalar(self_: Motor, other: Scalar) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ self_.g0_, /* e23, e31, e12, scalar */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, (self_.g1_.w + other.g0_)));
}
fn multiVector_add_antiScalar(self_: MultiVector, other: AntiScalar) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g0_.x, (self_.g0_.y + other.g0_)), /* e1, e2, e3, e4 */ self_.g1_, /* e41, e42, e43 */ self_.g2_, /* e23, e31, e12 */ self_.g3_, /* e423, e431, e412, e321 */ self_.g4_);
}
fn multiVector_add_dualNum(self_: MultiVector, other: DualNum) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (other.g0_ + self_.g0_), /* e1, e2, e3, e4 */ self_.g1_, /* e41, e42, e43 */ self_.g2_, /* e23, e31, e12 */ self_.g3_, /* e423, e431, e412, e321 */ self_.g4_);
}
fn multiVector_add_flector(self_: MultiVector, other: Flector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ (other.g0_ + self_.g1_), /* e41, e42, e43 */ self_.g2_, /* e23, e31, e12 */ self_.g3_, /* e423, e431, e412, e321 */ (other.g1_ + self_.g4_));
}
fn multiVector_add_horizon(self_: MultiVector, other: Horizon) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ self_.g1_, /* e41, e42, e43 */ self_.g2_, /* e23, e31, e12 */ self_.g3_, /* e423, e431, e412, e321 */ vec4<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z, (self_.g4_.w + other.g0_)));
}
fn multiVector_add_line(self_: MultiVector, other: Line) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ self_.g1_, /* e41, e42, e43 */ (other.g0_ + self_.g2_), /* e23, e31, e12 */ (other.g1_ + self_.g3_), /* e423, e431, e412, e321 */ self_.g4_);
}
fn multiVector_add_motor(self_: MultiVector, other: Motor) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(other.g1_.w, other.g0_.w) + self_.g0_), /* e1, e2, e3, e4 */ self_.g1_, /* e41, e42, e43 */ (vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z) + self_.g2_), /* e23, e31, e12 */ (vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z) + self_.g3_), /* e423, e431, e412, e321 */ self_.g4_);
}
fn multiVector_add_multiVector(self_: MultiVector, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (other.g0_ + self_.g0_), /* e1, e2, e3, e4 */ (other.g1_ + self_.g1_), /* e41, e42, e43 */ (other.g2_ + self_.g2_), /* e23, e31, e12 */ (other.g3_ + self_.g3_), /* e423, e431, e412, e321 */ (other.g4_ + self_.g4_));
}
fn multiVector_add_origin(self_: MultiVector, other: Origin) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, (self_.g1_.w + other.g0_)), /* e41, e42, e43 */ self_.g2_, /* e23, e31, e12 */ self_.g3_, /* e423, e431, e412, e321 */ self_.g4_);
}
fn multiVector_add_plane(self_: MultiVector, other: Plane) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ self_.g1_, /* e41, e42, e43 */ self_.g2_, /* e23, e31, e12 */ self_.g3_, /* e423, e431, e412, e321 */ (self_.g4_ + other.g0_));
}
fn multiVector_add_point(self_: MultiVector, other: Point) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ (self_.g1_ + other.g0_), /* e41, e42, e43 */ self_.g2_, /* e23, e31, e12 */ self_.g3_, /* e423, e431, e412, e321 */ self_.g4_);
}
fn multiVector_add_scalar(self_: MultiVector, other: Scalar) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((self_.g0_.x + other.g0_), self_.g0_.y), /* e1, e2, e3, e4 */ self_.g1_, /* e41, e42, e43 */ self_.g2_, /* e23, e31, e12 */ self_.g3_, /* e423, e431, e412, e321 */ self_.g4_);
}
fn origin_add_antiScalar(self_: Origin, other: AntiScalar) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, other.g0_), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn origin_add_dualNum(self_: Origin, other: DualNum) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ other.g0_, /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn origin_add_flector(self_: Origin, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, (other.g0_.w + self_.g0_)), /* e423, e431, e412, e321 */ other.g1_);
}
fn origin_add_horizon(self_: Origin, other: Horizon) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other.g0_));
}
fn origin_add_line(self_: Origin, other: Line) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_), /* e41, e42, e43 */ other.g0_, /* e23, e31, e12 */ other.g1_, /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn origin_add_motor(self_: Origin, other: Motor) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(other.g1_.w, other.g0_.w), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_), /* e41, e42, e43 */ vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z), /* e23, e31, e12 */ vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn origin_add_multiVector(self_: Origin, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ other.g0_, /* e1, e2, e3, e4 */ vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, (other.g1_.w + self_.g0_)), /* e41, e42, e43 */ other.g2_, /* e23, e31, e12 */ other.g3_, /* e423, e431, e412, e321 */ other.g4_);
}
fn origin_add_origin(self_: Origin, other: Origin) -> Origin {
    return Origin(/* e4 */ (other.g0_ + self_.g0_));
}
fn origin_add_plane(self_: Origin, other: Plane) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_), /* e423, e431, e412, e321 */ other.g0_);
}
fn origin_add_point(self_: Origin, other: Point) -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, (other.g0_.w + self_.g0_)));
}
fn origin_add_scalar(self_: Origin, other: Scalar) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(other.g0_, 0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn plane_add_antiScalar(self_: Plane, other: AntiScalar) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, other.g0_), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ self_.g0_);
}
fn plane_add_dualNum(self_: Plane, other: DualNum) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ other.g0_, /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ self_.g0_);
}
fn plane_add_flector(self_: Plane, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ other.g0_, /* e423, e431, e412, e321 */ (other.g1_ + self_.g0_));
}
fn plane_add_horizon(self_: Plane, other: Horizon) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, (self_.g0_.w + other.g0_)));
}
fn plane_add_line(self_: Plane, other: Line) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ other.g0_, /* e23, e31, e12 */ other.g1_, /* e423, e431, e412, e321 */ self_.g0_);
}
fn plane_add_motor(self_: Plane, other: Motor) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(other.g1_.w, other.g0_.w), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z), /* e23, e31, e12 */ vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z), /* e423, e431, e412, e321 */ self_.g0_);
}
fn plane_add_multiVector(self_: Plane, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ other.g0_, /* e1, e2, e3, e4 */ other.g1_, /* e41, e42, e43 */ other.g2_, /* e23, e31, e12 */ other.g3_, /* e423, e431, e412, e321 */ (other.g4_ + self_.g0_));
}
fn plane_add_origin(self_: Plane, other: Origin) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other.g0_), /* e423, e431, e412, e321 */ self_.g0_);
}
fn plane_add_plane(self_: Plane, other: Plane) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ (other.g0_ + self_.g0_));
}
fn plane_add_point(self_: Plane, other: Point) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ other.g0_, /* e423, e431, e412, e321 */ self_.g0_);
}
fn plane_add_scalar(self_: Plane, other: Scalar) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(other.g0_, 0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ self_.g0_);
}
fn point_add_antiScalar(self_: Point, other: AntiScalar) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, other.g0_), /* e1, e2, e3, e4 */ self_.g0_, /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn point_add_dualNum(self_: Point, other: DualNum) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ other.g0_, /* e1, e2, e3, e4 */ self_.g0_, /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn point_add_flector(self_: Point, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (other.g0_ + self_.g0_), /* e423, e431, e412, e321 */ other.g1_);
}
fn point_add_horizon(self_: Point, other: Horizon) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ self_.g0_, /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other.g0_));
}
fn point_add_line(self_: Point, other: Line) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ self_.g0_, /* e41, e42, e43 */ other.g0_, /* e23, e31, e12 */ other.g1_, /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn point_add_motor(self_: Point, other: Motor) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(other.g1_.w, other.g0_.w), /* e1, e2, e3, e4 */ self_.g0_, /* e41, e42, e43 */ vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z), /* e23, e31, e12 */ vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn point_add_multiVector(self_: Point, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ other.g0_, /* e1, e2, e3, e4 */ (other.g1_ + self_.g0_), /* e41, e42, e43 */ other.g2_, /* e23, e31, e12 */ other.g3_, /* e423, e431, e412, e321 */ other.g4_);
}
fn point_add_origin(self_: Point, other: Origin) -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, (self_.g0_.w + other.g0_)));
}
fn point_add_plane(self_: Point, other: Plane) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ self_.g0_, /* e423, e431, e412, e321 */ other.g0_);
}
fn point_add_point(self_: Point, other: Point) -> Point {
    return Point(/* e1, e2, e3, e4 */ (other.g0_ + self_.g0_));
}
fn point_add_scalar(self_: Point, other: Scalar) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(other.g0_, 0.0), /* e1, e2, e3, e4 */ self_.g0_, /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn scalar_add_antiScalar(self_: Scalar, other: AntiScalar) -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>(self_.g0_, other.g0_));
}
fn scalar_add_dualNum(self_: Scalar, other: DualNum) -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>((other.g0_.x + self_.g0_), other.g0_.y));
}
fn scalar_add_flector(self_: Scalar, other: Flector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g0_, 0.0), /* e1, e2, e3, e4 */ other.g0_, /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ other.g1_);
}
fn scalar_add_horizon(self_: Scalar, other: Horizon) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g0_, 0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other.g0_));
}
fn scalar_add_line(self_: Scalar, other: Line) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, 0.0), /* e23, e31, e12, scalar */ vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, self_.g0_));
}
fn scalar_add_motor(self_: Scalar, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ other.g0_, /* e23, e31, e12, scalar */ vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, (other.g1_.w + self_.g0_)));
}
fn scalar_add_multiVector(self_: Scalar, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((other.g0_.x + self_.g0_), other.g0_.y), /* e1, e2, e3, e4 */ other.g1_, /* e41, e42, e43 */ other.g2_, /* e23, e31, e12 */ other.g3_, /* e423, e431, e412, e321 */ other.g4_);
}
fn scalar_add_origin(self_: Scalar, other: Origin) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g0_, 0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other.g0_), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn scalar_add_plane(self_: Scalar, other: Plane) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g0_, 0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ other.g0_);
}
fn scalar_add_point(self_: Scalar, other: Point) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g0_, 0.0), /* e1, e2, e3, e4 */ other.g0_, /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn scalar_add_scalar(self_: Scalar, other: Scalar) -> Scalar {
    return Scalar(/* scalar */ (other.g0_ + self_.g0_));
}
fn horizon_antiConstraintValid(self_: Horizon) -> Horizon {
    return self_;
}
fn scalar_antiConstraintValid(self_: Scalar) -> Scalar {
    return self_;
}
fn antiScalar_antiConstraintViolation(self_: AntiScalar) -> Scalar {
    return Scalar(/* scalar */ 0.0);
}
fn dualNum_antiConstraintViolation(self_: DualNum) -> Scalar {
    let geometric_anti_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>((self_.g0_.x * self_.g0_.y), pow(self_.g0_.y, 2)) * vec2<f32>(2.0, 1.0)));
    return Scalar(/* scalar */ geometric_anti_product.g0_.x);
}
fn flector_antiConstraintViolation(self_: Flector) -> DualNum {
    let anti_reverse: Flector = Flector(/* e1, e2, e3, e4 */ (self_.g0_ * vec4<f32>(-1.0)), /* e423, e431, e412, e321 */ self_.g1_);
    let geometric_anti_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>((-(anti_reverse.g0_.x * self_.g1_.x) - (anti_reverse.g0_.y * self_.g1_.y) - (anti_reverse.g0_.z * self_.g1_.z) + (anti_reverse.g1_.w * self_.g0_.w)), 0.0) - (vec2<f32>(anti_reverse.g0_.w) * vec2<f32>(self_.g1_.w, self_.g0_.w)) + (vec2<f32>(anti_reverse.g1_.x) * vec2<f32>(self_.g0_.x, self_.g1_.x)) + (vec2<f32>(anti_reverse.g1_.y) * vec2<f32>(self_.g0_.y, self_.g1_.y)) + (vec2<f32>(anti_reverse.g1_.z) * vec2<f32>(self_.g0_.z, self_.g1_.z))));
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(self_.g0_.w, 2) + pow(self_.g1_.x, 2) + pow(self_.g1_.y, 2) + pow(self_.g1_.z, 2)));
    return DualNum(/* scalar, e1234 */ vec2<f32>(geometric_anti_product.g0_.x, (geometric_anti_product.g0_.y - anti_scalar_product.g0_)));
}
fn line_antiConstraintViolation(self_: Line) -> DualNum {
    let anti_reverse: Line = Line(/* e41, e42, e43 */ (self_.g0_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (self_.g1_ * vec3<f32>(-1.0)));
    let geometric_anti_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>((-(anti_reverse.g1_.x * self_.g0_.x) - (anti_reverse.g1_.y * self_.g0_.y) - (anti_reverse.g1_.z * self_.g0_.z)), 0.0) - (vec2<f32>(anti_reverse.g0_.x) * vec2<f32>(self_.g1_.x, self_.g0_.x)) - (vec2<f32>(anti_reverse.g0_.y) * vec2<f32>(self_.g1_.y, self_.g0_.y)) - (vec2<f32>(anti_reverse.g0_.z) * vec2<f32>(self_.g1_.z, self_.g0_.z))));
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(self_.g0_.x, 2) - pow(self_.g0_.y, 2) - pow(self_.g0_.z, 2)));
    return DualNum(/* scalar, e1234 */ vec2<f32>(geometric_anti_product.g0_.x, (geometric_anti_product.g0_.y - anti_scalar_product.g0_)));
}
fn motor_antiConstraintViolation(self_: Motor) -> DualNum {
    let anti_reverse: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g0_.x * -1.0), (self_.g0_.y * -1.0), (self_.g0_.z * -1.0), self_.g0_.w), /* e23, e31, e12, scalar */ vec4<f32>((self_.g1_.x * -1.0), (self_.g1_.y * -1.0), (self_.g1_.z * -1.0), self_.g1_.w));
    let geometric_anti_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>((-(anti_reverse.g1_.x * self_.g0_.x) - (anti_reverse.g1_.y * self_.g0_.y) - (anti_reverse.g1_.z * self_.g0_.z) + (anti_reverse.g1_.w * self_.g0_.w)), 0.0) - (vec2<f32>(anti_reverse.g0_.x) * vec2<f32>(self_.g1_.x, self_.g0_.x)) - (vec2<f32>(anti_reverse.g0_.y) * vec2<f32>(self_.g1_.y, self_.g0_.y)) - (vec2<f32>(anti_reverse.g0_.z) * vec2<f32>(self_.g1_.z, self_.g0_.z)) + (vec2<f32>(anti_reverse.g0_.w) * vec2<f32>(self_.g1_.w, self_.g0_.w))));
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(self_.g0_.x, 2) - pow(self_.g0_.y, 2) - pow(self_.g0_.z, 2) + pow(self_.g0_.w, 2)));
    return DualNum(/* scalar, e1234 */ vec2<f32>(geometric_anti_product.g0_.x, (geometric_anti_product.g0_.y - anti_scalar_product.g0_)));
}
fn multiVector_antiConstraintViolation(self_: MultiVector) -> MultiVector {
    let anti_reverse: MultiVector = MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ (self_.g1_ * vec4<f32>(-1.0)), /* e41, e42, e43 */ (self_.g2_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (self_.g3_ * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ self_.g4_);
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(((anti_reverse.g0_.y * self_.g0_.x) - (anti_reverse.g3_.x * self_.g2_.x) - (anti_reverse.g3_.y * self_.g2_.y) - (anti_reverse.g3_.z * self_.g2_.z) - (anti_reverse.g1_.x * self_.g4_.x) - (anti_reverse.g1_.y * self_.g4_.y) - (anti_reverse.g1_.z * self_.g4_.z) + (anti_reverse.g4_.w * self_.g1_.w)), 0.0) + (vec2<f32>(self_.g0_.y) * anti_reverse.g0_) - (vec2<f32>(anti_reverse.g2_.x) * vec2<f32>(self_.g3_.x, self_.g2_.x)) - (vec2<f32>(anti_reverse.g2_.y) * vec2<f32>(self_.g3_.y, self_.g2_.y)) - (vec2<f32>(anti_reverse.g2_.z) * vec2<f32>(self_.g3_.z, self_.g2_.z)) - (vec2<f32>(anti_reverse.g1_.w) * vec2<f32>(self_.g4_.w, self_.g1_.w)) + (vec2<f32>(anti_reverse.g4_.x) * vec2<f32>(self_.g1_.x, self_.g4_.x)) + (vec2<f32>(anti_reverse.g4_.y) * vec2<f32>(self_.g1_.y, self_.g4_.y)) + (vec2<f32>(anti_reverse.g4_.z) * vec2<f32>(self_.g1_.z, self_.g4_.z))), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ (vec4<f32>(((anti_reverse.g2_.x * self_.g1_.w) - (anti_reverse.g2_.y * self_.g4_.z) + (self_.g2_.y * anti_reverse.g4_.z)), ((anti_reverse.g2_.y * self_.g1_.w) - (anti_reverse.g2_.z * self_.g4_.x) + (self_.g2_.z * anti_reverse.g4_.x)), (-(anti_reverse.g2_.x * self_.g4_.y) + (anti_reverse.g2_.z * self_.g1_.w) + (self_.g2_.x * anti_reverse.g4_.y)), (-(anti_reverse.g0_.x * self_.g1_.w) - (anti_reverse.g2_.x * self_.g1_.x) - (anti_reverse.g2_.y * self_.g1_.y) - (anti_reverse.g2_.z * self_.g1_.z) + (anti_reverse.g3_.y * self_.g4_.y) + (anti_reverse.g3_.z * self_.g4_.z) - (self_.g2_.x * anti_reverse.g1_.x) - (self_.g2_.y * anti_reverse.g1_.y) - (self_.g2_.z * anti_reverse.g1_.z) - (self_.g3_.y * anti_reverse.g4_.y) - (self_.g3_.z * anti_reverse.g4_.z))) + (vec4<f32>(anti_reverse.g0_.y) * self_.g4_) + (vec4<f32>(self_.g0_.y) * anti_reverse.g4_) + (vec4<f32>(anti_reverse.g1_.w) * vec4<f32>(self_.g2_.x, self_.g2_.y, self_.g2_.z, self_.g0_.x)) + (vec4<f32>(anti_reverse.g2_.z, anti_reverse.g2_.x, anti_reverse.g2_.y, anti_reverse.g3_.x) * self_.g4_.yzxx) - (vec4<f32>(self_.g2_.z, self_.g2_.x, self_.g2_.y, self_.g3_.x) * anti_reverse.g4_.yzxx)));
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(self_.g0_.y, 2) - pow(self_.g2_.x, 2) - pow(self_.g2_.y, 2) - pow(self_.g2_.z, 2) - pow(self_.g1_.w, 2) + pow(self_.g4_.x, 2) + pow(self_.g4_.y, 2) + pow(self_.g4_.z, 2)));
    return MultiVector(/* scalar, e1234 */ vec2<f32>(geometric_anti_product.g0_.x, (geometric_anti_product.g0_.y - anti_scalar_product.g0_)), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ geometric_anti_product.g4_);
}
fn origin_antiConstraintViolation(self_: Origin) -> AntiScalar {
    let anti_reverse: Origin = Origin(/* e4 */ (self_.g0_ * -1.0));
    let geometric_anti_product: AntiScalar = AntiScalar(/* e1234 */ (anti_reverse.g0_ * self_.g0_ * -1.0));
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(self_.g0_, 2) * -1.0));
    return AntiScalar(/* e1234 */ (-anti_scalar_product.g0_ + geometric_anti_product.g0_));
}
fn plane_antiConstraintViolation(self_: Plane) -> Scalar {
    return Scalar(/* scalar */ 0.0);
}
fn point_antiConstraintViolation(self_: Point) -> AntiScalar {
    let anti_reverse: Point = Point(/* e1, e2, e3, e4 */ (self_.g0_ * vec4<f32>(-1.0)));
    let geometric_anti_product: AntiScalar = AntiScalar(/* e1234 */ (anti_reverse.g0_.w * self_.g0_.w * -1.0));
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(self_.g0_.w, 2) * -1.0));
    return AntiScalar(/* e1234 */ (-anti_scalar_product.g0_ + geometric_anti_product.g0_));
}
fn antiScalar_antiFixImpl(self_: AntiScalar) -> AntiScalar {
    let geometric_anti_product: AntiScalar = AntiScalar(/* e1234 */ pow(self_.g0_, 2));
    let anti_square_root: AntiScalar = AntiScalar(/* e1234 */ pow(geometric_anti_product.g0_.x, 0.5));
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(anti_square_root.g0_, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return AntiScalar(/* e1234 */ (anti_inverse.g0_ * self_.g0_));
}
fn origin_antiFixImpl(self_: Origin) -> Origin {
    let anti_reverse: Origin = Origin(/* e4 */ (self_.g0_ * -1.0));
    let geometric_anti_product: AntiScalar = AntiScalar(/* e1234 */ (anti_reverse.g0_ * self_.g0_ * -1.0));
    let anti_square_root: AntiScalar = AntiScalar(/* e1234 */ pow(geometric_anti_product.g0_.x, 0.5));
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(anti_square_root.g0_, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Origin(/* e4 */ (anti_inverse.g0_ * self_.g0_));
}
fn plane_antiFixImpl(self_: Plane) -> Plane {
    let geometric_anti_product: AntiScalar = AntiScalar(/* e1234 */ (pow(self_.g0_.x, 2) + pow(self_.g0_.y, 2) + pow(self_.g0_.z, 2)));
    let anti_square_root: AntiScalar = AntiScalar(/* e1234 */ pow(geometric_anti_product.g0_.x, 0.5));
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(anti_square_root.g0_, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Plane(/* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_));
}
fn point_antiFixImpl(self_: Point) -> Point {
    let anti_reverse: Point = Point(/* e1, e2, e3, e4 */ (self_.g0_ * vec4<f32>(-1.0)));
    let geometric_anti_product: AntiScalar = AntiScalar(/* e1234 */ (anti_reverse.g0_.w * self_.g0_.w * -1.0));
    let anti_square_root: AntiScalar = AntiScalar(/* e1234 */ pow(geometric_anti_product.g0_.x, 0.5));
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(anti_square_root.g0_, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Point(/* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_));
}
fn antiScalar_antiGrade() -> i32 {
    return 0;
}
fn horizon_antiGrade() -> i32 {
    return 1;
}
fn line_antiGrade() -> i32 {
    return 2;
}
fn origin_antiGrade() -> i32 {
    return 3;
}
fn plane_antiGrade() -> i32 {
    return 1;
}
fn point_antiGrade() -> i32 {
    return 3;
}
fn scalar_antiGrade() -> i32 {
    return 4;
}
fn antiScalar_antiInverse(self_: AntiScalar) -> AntiScalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(self_.g0_, 2));
    return AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
}
fn dualNum_antiInverse(self_: DualNum) -> AntiScalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(self_.g0_.y, 2));
    return AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
}
fn flector_antiInverse(self_: Flector) -> AntiScalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(self_.g0_.w, 2) + pow(self_.g1_.x, 2) + pow(self_.g1_.y, 2) + pow(self_.g1_.z, 2)));
    return AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
}
fn line_antiInverse(self_: Line) -> AntiScalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(self_.g0_.x, 2) - pow(self_.g0_.y, 2) - pow(self_.g0_.z, 2)));
    return AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
}
fn motor_antiInverse(self_: Motor) -> AntiScalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(self_.g0_.x, 2) - pow(self_.g0_.y, 2) - pow(self_.g0_.z, 2) + pow(self_.g0_.w, 2)));
    return AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
}
fn multiVector_antiInverse(self_: MultiVector) -> AntiScalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(self_.g0_.y, 2) - pow(self_.g2_.x, 2) - pow(self_.g2_.y, 2) - pow(self_.g2_.z, 2) - pow(self_.g1_.w, 2) + pow(self_.g4_.x, 2) + pow(self_.g4_.y, 2) + pow(self_.g4_.z, 2)));
    return AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
}
fn origin_antiInverse(self_: Origin) -> AntiScalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(self_.g0_, 2) * -1.0));
    return AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
}
fn plane_antiInverse(self_: Plane) -> AntiScalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(self_.g0_.x, 2) + pow(self_.g0_.y, 2) + pow(self_.g0_.z, 2)));
    return AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
}
fn point_antiInverse(self_: Point) -> AntiScalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(self_.g0_.w, 2) * -1.0));
    return AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
}
fn antiScalar_antiOne() -> AntiScalar {
    return AntiScalar(/* e1234 */ 1.0);
}
fn dualNum_antiOne() -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>(0.0, 1.0));
}
fn motor_antiOne() -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, 1.0), /* e23, e31, e12, scalar */ vec4<f32>(0.0));
}
fn multiVector_antiOne() -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, 1.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn antiScalar_antiReverse(self_: AntiScalar) -> AntiScalar {
    return self_;
}
fn dualNum_antiReverse(self_: DualNum) -> DualNum {
    return self_;
}
fn flector_antiReverse(self_: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (self_.g0_ * vec4<f32>(-1.0)), /* e423, e431, e412, e321 */ self_.g1_);
}
fn horizon_antiReverse(self_: Horizon) -> Horizon {
    return self_;
}
fn line_antiReverse(self_: Line) -> Line {
    return Line(/* e41, e42, e43 */ (self_.g0_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (self_.g1_ * vec3<f32>(-1.0)));
}
fn motor_antiReverse(self_: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g0_.x * -1.0), (self_.g0_.y * -1.0), (self_.g0_.z * -1.0), self_.g0_.w), /* e23, e31, e12, scalar */ vec4<f32>((self_.g1_.x * -1.0), (self_.g1_.y * -1.0), (self_.g1_.z * -1.0), self_.g1_.w));
}
fn multiVector_antiReverse(self_: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ (self_.g1_ * vec4<f32>(-1.0)), /* e41, e42, e43 */ (self_.g2_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (self_.g3_ * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ self_.g4_);
}
fn origin_antiReverse(self_: Origin) -> Origin {
    return Origin(/* e4 */ (self_.g0_ * -1.0));
}
fn plane_antiReverse(self_: Plane) -> Plane {
    return self_;
}
fn point_antiReverse(self_: Point) -> Point {
    return Point(/* e1, e2, e3, e4 */ (self_.g0_ * vec4<f32>(-1.0)));
}
fn scalar_antiReverse(self_: Scalar) -> Scalar {
    return self_;
}
fn antiScalar_antiSquareRoot(self_: AntiScalar) -> AntiScalar {
    return AntiScalar(/* e1234 */ pow(self_.g0_.x, 0.5));
}
fn antiScalar_antiWedge_antiScalar(self_: AntiScalar, other: AntiScalar) -> AntiScalar {
    return AntiScalar(/* e1234 */ (other.g0_ * self_.g0_));
}
fn antiScalar_antiWedge_dualNum(self_: AntiScalar, other: DualNum) -> DualNum {
    return DualNum(/* scalar, e1234 */ (vec2<f32>(self_.g0_) * other.g0_));
}
fn antiScalar_antiWedge_flector(self_: AntiScalar, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_) * other.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_) * other.g1_));
}
fn antiScalar_antiWedge_horizon(self_: AntiScalar, other: Horizon) -> Horizon {
    return Horizon(/* e321 */ (self_.g0_ * other.g0_));
}
fn antiScalar_antiWedge_line(self_: AntiScalar, other: Line) -> Line {
    return Line(/* e41, e42, e43 */ (vec3<f32>(self_.g0_) * other.g0_), /* e23, e31, e12 */ (vec3<f32>(self_.g0_) * other.g1_));
}
fn antiScalar_antiWedge_motor(self_: AntiScalar, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(self_.g0_) * other.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(self_.g0_) * other.g1_));
}
fn antiScalar_antiWedge_multiVector(self_: AntiScalar, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(self_.g0_) * other.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_) * other.g1_), /* e41, e42, e43 */ (vec3<f32>(self_.g0_) * other.g2_), /* e23, e31, e12 */ (vec3<f32>(self_.g0_) * other.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_) * other.g4_));
}
fn antiScalar_antiWedge_origin(self_: AntiScalar, other: Origin) -> Origin {
    return Origin(/* e4 */ (self_.g0_ * other.g0_));
}
fn antiScalar_antiWedge_plane(self_: AntiScalar, other: Plane) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_) * other.g0_));
}
fn antiScalar_antiWedge_point(self_: AntiScalar, other: Point) -> Point {
    return Point(/* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_) * other.g0_));
}
fn antiScalar_antiWedge_scalar(self_: AntiScalar, other: Scalar) -> Scalar {
    return Scalar(/* scalar */ (self_.g0_ * other.g0_));
}
fn dualNum_antiWedge_antiScalar(self_: DualNum, other: AntiScalar) -> DualNum {
    return DualNum(/* scalar, e1234 */ (vec2<f32>(other.g0_) * self_.g0_));
}
fn dualNum_antiWedge_dualNum(self_: DualNum, other: DualNum) -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>(((other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x)), (other.g0_.y * self_.g0_.y)));
}
fn dualNum_antiWedge_flector(self_: DualNum, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_.y) * other.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_.y) * other.g1_));
}
fn dualNum_antiWedge_horizon(self_: DualNum, other: Horizon) -> Horizon {
    return Horizon(/* e321 */ (self_.g0_.y * other.g0_));
}
fn dualNum_antiWedge_line(self_: DualNum, other: Line) -> Line {
    return Line(/* e41, e42, e43 */ (vec3<f32>(self_.g0_.y) * other.g0_), /* e23, e31, e12 */ (vec3<f32>(self_.g0_.y) * other.g1_));
}
fn dualNum_antiWedge_motor(self_: DualNum, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(self_.g0_.y) * other.g0_), /* e23, e31, e12, scalar */ vec4<f32>((self_.g0_.y * other.g1_.x), (self_.g0_.y * other.g1_.y), (self_.g0_.y * other.g1_.z), ((self_.g0_.x * other.g0_.w) + (self_.g0_.y * other.g1_.w))));
}
fn dualNum_antiWedge_multiVector(self_: DualNum, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(((self_.g0_.x * other.g0_.y) + (self_.g0_.y * other.g0_.x)), (self_.g0_.y * other.g0_.y)), /* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_.y) * other.g1_), /* e41, e42, e43 */ (vec3<f32>(self_.g0_.y) * other.g2_), /* e23, e31, e12 */ (vec3<f32>(self_.g0_.y) * other.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_.y) * other.g4_));
}
fn dualNum_antiWedge_origin(self_: DualNum, other: Origin) -> Origin {
    return Origin(/* e4 */ (self_.g0_.y * other.g0_));
}
fn dualNum_antiWedge_plane(self_: DualNum, other: Plane) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_.y) * other.g0_));
}
fn dualNum_antiWedge_point(self_: DualNum, other: Point) -> Point {
    return Point(/* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_.y) * other.g0_));
}
fn dualNum_antiWedge_scalar(self_: DualNum, other: Scalar) -> Scalar {
    return Scalar(/* scalar */ (self_.g0_.y * other.g0_));
}
fn flector_antiWedge_antiScalar(self_: Flector, other: AntiScalar) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(other.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_) * self_.g1_));
}
fn flector_antiWedge_dualNum(self_: Flector, other: DualNum) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(other.g0_.y) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_.y) * self_.g1_));
}
fn flector_antiWedge_flector(self_: Flector, other: Flector) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(((other.g1_.y * self_.g1_.z) - (other.g1_.z * self_.g1_.y)), (-(other.g1_.x * self_.g1_.z) + (other.g1_.z * self_.g1_.x)), ((other.g1_.x * self_.g1_.y) - (other.g1_.y * self_.g1_.x)), 0.0), /* e23, e31, e12, scalar */ (vec4<f32>(0.0, 0.0, 0.0, (-(other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g0_.w * self_.g1_.w) + (other.g1_.y * self_.g0_.y) + (other.g1_.z * self_.g0_.z) + (other.g1_.w * self_.g0_.w))) - (vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, other.g0_.x) * self_.g1_.wwwx) + (vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g0_.x) * other.g1_.wwwx)));
}
fn flector_antiWedge_horizon(self_: Flector, other: Horizon) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0), /* e23, e31, e12, scalar */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g0_.w)));
}
fn flector_antiWedge_line(self_: Flector, other: Line) -> Point {
    return Point(/* e1, e2, e3, e4 */ (vec4<f32>(((other.g0_.x * self_.g1_.w) + (other.g1_.y * self_.g1_.z)), ((other.g0_.y * self_.g1_.w) + (other.g1_.z * self_.g1_.x)), ((other.g0_.z * self_.g1_.w) + (other.g1_.x * self_.g1_.y)), (-(other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z))) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.x) * self_.g1_.yzxx)));
}
fn flector_antiWedge_motor(self_: Flector, other: Motor) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(((self_.g1_.z * other.g1_.y) + (self_.g1_.w * other.g0_.x)), ((self_.g1_.x * other.g1_.z) + (self_.g1_.w * other.g0_.y)), ((self_.g1_.y * other.g1_.x) + (self_.g1_.w * other.g0_.z)), (-(self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))) + (vec4<f32>(other.g0_.w) * self_.g0_) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.x) * self_.g1_.yzxx)), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_.w) * self_.g1_));
}
fn flector_antiWedge_multiVector(self_: Flector, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(((self_.g0_.x * other.g4_.x) + (self_.g0_.y * other.g4_.y) + (self_.g0_.z * other.g4_.z) + (self_.g0_.w * other.g4_.w) - (self_.g1_.x * other.g1_.x) - (self_.g1_.y * other.g1_.y) - (self_.g1_.z * other.g1_.z) - (self_.g1_.w * other.g1_.w)), 0.0), /* e1, e2, e3, e4 */ (vec4<f32>(((other.g2_.x * self_.g1_.w) + (other.g3_.y * self_.g1_.z)), ((other.g2_.y * self_.g1_.w) + (other.g3_.z * self_.g1_.x)), ((other.g2_.z * self_.g1_.w) + (other.g3_.x * self_.g1_.y)), (-(other.g2_.y * self_.g1_.y) - (other.g2_.z * self_.g1_.z))) + (vec4<f32>(other.g0_.y) * self_.g0_) - (vec4<f32>(other.g3_.z, other.g3_.x, other.g3_.y, other.g2_.x) * self_.g1_.yzxx)), /* e41, e42, e43 */ vec3<f32>((-(self_.g1_.y * other.g4_.z) + (self_.g1_.z * other.g4_.y)), ((self_.g1_.x * other.g4_.z) - (self_.g1_.z * other.g4_.x)), (-(self_.g1_.x * other.g4_.y) + (self_.g1_.y * other.g4_.x))), /* e23, e31, e12 */ (-(vec3<f32>(self_.g1_.w) * vec3<f32>(other.g4_.x, other.g4_.y, other.g4_.z)) + (vec3<f32>(other.g4_.w) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z))), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_.y) * self_.g1_));
}
fn flector_antiWedge_origin(self_: Flector, other: Origin) -> Scalar {
    return Scalar(/* scalar */ (self_.g1_.w * other.g0_ * -1.0));
}
fn flector_antiWedge_plane(self_: Flector, other: Plane) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((-(self_.g1_.y * other.g0_.z) + (self_.g1_.z * other.g0_.y)), ((self_.g1_.x * other.g0_.z) - (self_.g1_.z * other.g0_.x)), (-(self_.g1_.x * other.g0_.y) + (self_.g1_.y * other.g0_.x)), 0.0), /* e23, e31, e12, scalar */ (vec4<f32>(((self_.g1_.w * other.g0_.x) * -1.0), ((self_.g1_.w * other.g0_.y) * -1.0), ((self_.g1_.w * other.g0_.z) * -1.0), ((self_.g0_.y * other.g0_.y) + (self_.g0_.z * other.g0_.z) + (self_.g0_.w * other.g0_.w))) + (vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g0_.x) * other.g0_.wwwx)));
}
fn flector_antiWedge_point(self_: Flector, other: Point) -> Scalar {
    return Scalar(/* scalar */ (-(self_.g1_.x * other.g0_.x) - (self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z) - (self_.g1_.w * other.g0_.w)));
}
fn horizon_antiWedge_antiScalar(self_: Horizon, other: AntiScalar) -> Horizon {
    return Horizon(/* e321 */ (other.g0_ * self_.g0_));
}
fn horizon_antiWedge_dualNum(self_: Horizon, other: DualNum) -> Horizon {
    return Horizon(/* e321 */ (other.g0_.y * self_.g0_));
}
fn horizon_antiWedge_flector(self_: Horizon, other: Flector) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0), /* e23, e31, e12, scalar */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, other.g0_.w) * vec4<f32>(-1.0)));
}
fn horizon_antiWedge_line(self_: Horizon, other: Line) -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), 0.0));
}
fn horizon_antiWedge_motor(self_: Horizon, other: Motor) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), 0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.w * self_.g0_)));
}
fn horizon_antiWedge_multiVector(self_: Horizon, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((other.g1_.w * self_.g0_ * -1.0), 0.0), /* e1, e2, e3, e4 */ vec4<f32>((other.g2_.x * self_.g0_), (other.g2_.y * self_.g0_), (other.g2_.z * self_.g0_), 0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ (vec3<f32>(self_.g0_) * vec3<f32>(other.g4_.x, other.g4_.y, other.g4_.z) * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.y * self_.g0_)));
}
fn horizon_antiWedge_origin(self_: Horizon, other: Origin) -> Scalar {
    return Scalar(/* scalar */ (self_.g0_ * other.g0_ * -1.0));
}
fn horizon_antiWedge_plane(self_: Horizon, other: Plane) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ (vec3<f32>(self_.g0_) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z) * vec3<f32>(-1.0)));
}
fn horizon_antiWedge_point(self_: Horizon, other: Point) -> Scalar {
    return Scalar(/* scalar */ (other.g0_.w * self_.g0_ * -1.0));
}
fn line_antiWedge_antiScalar(self_: Line, other: AntiScalar) -> Line {
    return Line(/* e41, e42, e43 */ (vec3<f32>(other.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(other.g0_) * self_.g1_));
}
fn line_antiWedge_dualNum(self_: Line, other: DualNum) -> Line {
    return Line(/* e41, e42, e43 */ (vec3<f32>(other.g0_.y) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(other.g0_.y) * self_.g1_));
}
fn line_antiWedge_flector(self_: Line, other: Flector) -> Point {
    return Point(/* e1, e2, e3, e4 */ (vec4<f32>(((self_.g0_.x * other.g1_.w) + (self_.g1_.y * other.g1_.z)), ((self_.g0_.y * other.g1_.w) + (self_.g1_.z * other.g1_.x)), ((self_.g0_.z * other.g1_.w) + (self_.g1_.x * other.g1_.y)), (-(self_.g0_.y * other.g1_.y) - (self_.g0_.z * other.g1_.z))) - (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g0_.x) * other.g1_.yzxx)));
}
fn line_antiWedge_horizon(self_: Line, other: Horizon) -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), 0.0));
}
fn line_antiWedge_line(self_: Line, other: Line) -> Scalar {
    return Scalar(/* scalar */ (-(other.g0_.x * self_.g1_.x) - (other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z)));
}
fn line_antiWedge_motor(self_: Line, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g0_.x * other.g0_.w), (self_.g0_.y * other.g0_.w), (self_.g0_.z * other.g0_.w), 0.0), /* e23, e31, e12, scalar */ vec4<f32>((self_.g1_.x * other.g0_.w), (self_.g1_.y * other.g0_.w), (self_.g1_.z * other.g0_.w), (-(self_.g0_.x * other.g1_.x) - (self_.g0_.y * other.g1_.y) - (self_.g0_.z * other.g1_.z) - (self_.g1_.x * other.g0_.x) - (self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))));
}
fn line_antiWedge_multiVector(self_: Line, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((-(self_.g0_.x * other.g3_.x) - (self_.g0_.y * other.g3_.y) - (self_.g0_.z * other.g3_.z) - (self_.g1_.x * other.g2_.x) - (self_.g1_.y * other.g2_.y) - (self_.g1_.z * other.g2_.z)), 0.0), /* e1, e2, e3, e4 */ (vec4<f32>(((self_.g0_.x * other.g4_.w) + (self_.g1_.y * other.g4_.z)), ((self_.g0_.y * other.g4_.w) + (self_.g1_.z * other.g4_.x)), ((self_.g0_.z * other.g4_.w) + (self_.g1_.x * other.g4_.y)), (-(self_.g0_.y * other.g4_.y) - (self_.g0_.z * other.g4_.z))) - (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g0_.x) * other.g4_.yzxx)), /* e41, e42, e43 */ (vec3<f32>(other.g0_.y) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(other.g0_.y) * self_.g1_), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn line_antiWedge_plane(self_: Line, other: Plane) -> Point {
    return Point(/* e1, e2, e3, e4 */ (vec4<f32>(((self_.g0_.x * other.g0_.w) + (self_.g1_.y * other.g0_.z)), ((self_.g0_.y * other.g0_.w) + (self_.g1_.z * other.g0_.x)), ((self_.g0_.z * other.g0_.w) + (self_.g1_.x * other.g0_.y)), (-(self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z))) - (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g0_.x) * other.g0_.yzxx)));
}
fn motor_antiWedge_antiScalar(self_: Motor, other: AntiScalar) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(other.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(other.g0_) * self_.g1_));
}
fn motor_antiWedge_dualNum(self_: Motor, other: DualNum) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(other.g0_.y) * self_.g0_), /* e23, e31, e12, scalar */ vec4<f32>((other.g0_.y * self_.g1_.x), (other.g0_.y * self_.g1_.y), (other.g0_.y * self_.g1_.z), ((other.g0_.x * self_.g0_.w) + (other.g0_.y * self_.g1_.w))));
}
fn motor_antiWedge_flector(self_: Motor, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(((other.g1_.z * self_.g1_.y) + (other.g1_.w * self_.g0_.x)), ((other.g1_.x * self_.g1_.z) + (other.g1_.w * self_.g0_.y)), ((other.g1_.y * self_.g1_.x) + (other.g1_.w * self_.g0_.z)), (-(other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))) + (vec4<f32>(self_.g0_.w) * other.g0_) - (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g0_.x) * other.g1_.yzxx)), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_.w) * other.g1_));
}
fn motor_antiWedge_horizon(self_: Motor, other: Horizon) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), 0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.w * other.g0_)));
}
fn motor_antiWedge_line(self_: Motor, other: Line) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g0_.x * self_.g0_.w), (other.g0_.y * self_.g0_.w), (other.g0_.z * self_.g0_.w), 0.0), /* e23, e31, e12, scalar */ vec4<f32>((other.g1_.x * self_.g0_.w), (other.g1_.y * self_.g0_.w), (other.g1_.z * self_.g0_.w), (-(other.g0_.x * self_.g1_.x) - (other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))));
}
fn motor_antiWedge_motor(self_: Motor, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(((other.g0_.x * self_.g0_.w) + (other.g0_.w * self_.g0_.x)), ((other.g0_.y * self_.g0_.w) + (other.g0_.w * self_.g0_.y)), ((other.g0_.z * self_.g0_.w) + (other.g0_.w * self_.g0_.z)), (other.g0_.w * self_.g0_.w)), /* e23, e31, e12, scalar */ (vec4<f32>(0.0, 0.0, 0.0, (-(other.g0_.x * self_.g1_.x) - (other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))) + (vec4<f32>(other.g0_.w) * self_.g1_) + (vec4<f32>(self_.g0_.w) * other.g1_)));
}
fn motor_antiWedge_multiVector(self_: Motor, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(((other.g0_.x * self_.g0_.w) + (other.g0_.y * self_.g1_.w) - (other.g2_.x * self_.g1_.x) - (other.g2_.y * self_.g1_.y) - (other.g2_.z * self_.g1_.z) - (other.g3_.x * self_.g0_.x) - (other.g3_.y * self_.g0_.y) - (other.g3_.z * self_.g0_.z)), (other.g0_.y * self_.g0_.w)), /* e1, e2, e3, e4 */ (vec4<f32>(((self_.g0_.w * other.g1_.x) + (self_.g1_.y * other.g4_.z)), ((self_.g0_.w * other.g1_.y) + (self_.g1_.z * other.g4_.x)), ((self_.g0_.w * other.g1_.z) + (self_.g1_.x * other.g4_.y)), (-(self_.g0_.y * other.g4_.y) - (self_.g0_.z * other.g4_.z))) - (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g0_.x) * other.g4_.yzxx) + (vec4<f32>(other.g4_.w, other.g4_.w, other.g4_.w, other.g1_.w) * self_.g0_)), /* e41, e42, e43 */ ((vec3<f32>(other.g0_.y) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z)) + (vec3<f32>(self_.g0_.w) * other.g2_)), /* e23, e31, e12 */ ((vec3<f32>(other.g0_.y) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z)) + (vec3<f32>(self_.g0_.w) * other.g3_)), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_.w) * other.g4_));
}
fn motor_antiWedge_origin(self_: Motor, other: Origin) -> Origin {
    return Origin(/* e4 */ (self_.g0_.w * other.g0_));
}
fn motor_antiWedge_plane(self_: Motor, other: Plane) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(((self_.g0_.x * other.g0_.w) + (self_.g1_.y * other.g0_.z)), ((self_.g0_.y * other.g0_.w) + (self_.g1_.z * other.g0_.x)), ((self_.g0_.z * other.g0_.w) + (self_.g1_.x * other.g0_.y)), (-(self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z))) - (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g0_.x) * other.g0_.yzxx)), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_.w) * other.g0_));
}
fn motor_antiWedge_point(self_: Motor, other: Point) -> Point {
    return Point(/* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_.w) * other.g0_));
}
fn motor_antiWedge_scalar(self_: Motor, other: Scalar) -> Scalar {
    return Scalar(/* scalar */ (self_.g0_.w * other.g0_));
}
fn multiVector_antiWedge_antiScalar(self_: MultiVector, other: AntiScalar) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(other.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(other.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(other.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(other.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_) * self_.g4_));
}
fn multiVector_antiWedge_dualNum(self_: MultiVector, other: DualNum) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(((other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x)), (other.g0_.y * self_.g0_.y)), /* e1, e2, e3, e4 */ (vec4<f32>(other.g0_.y) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(other.g0_.y) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(other.g0_.y) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_.y) * self_.g4_));
}
fn multiVector_antiWedge_flector(self_: MultiVector, other: Flector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((-(other.g0_.x * self_.g4_.x) - (other.g0_.y * self_.g4_.y) - (other.g0_.z * self_.g4_.z) - (other.g0_.w * self_.g4_.w) + (other.g1_.x * self_.g1_.x) + (other.g1_.y * self_.g1_.y) + (other.g1_.z * self_.g1_.z) + (other.g1_.w * self_.g1_.w)), 0.0), /* e1, e2, e3, e4 */ (vec4<f32>(((self_.g2_.x * other.g1_.w) + (self_.g3_.y * other.g1_.z)), ((self_.g2_.y * other.g1_.w) + (self_.g3_.z * other.g1_.x)), ((self_.g2_.z * other.g1_.w) + (self_.g3_.x * other.g1_.y)), (-(self_.g2_.y * other.g1_.y) - (self_.g2_.z * other.g1_.z))) + (vec4<f32>(self_.g0_.y) * other.g0_) - (vec4<f32>(self_.g3_.z, self_.g3_.x, self_.g3_.y, self_.g2_.x) * other.g1_.yzxx)), /* e41, e42, e43 */ vec3<f32>(((other.g1_.y * self_.g4_.z) - (other.g1_.z * self_.g4_.y)), (-(other.g1_.x * self_.g4_.z) + (other.g1_.z * self_.g4_.x)), ((other.g1_.x * self_.g4_.y) - (other.g1_.y * self_.g4_.x))), /* e23, e31, e12 */ ((vec3<f32>(other.g1_.w) * vec3<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z)) - (vec3<f32>(self_.g4_.w) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z))), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_.y) * other.g1_));
}
fn multiVector_antiWedge_horizon(self_: MultiVector, other: Horizon) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((self_.g1_.w * other.g0_), 0.0), /* e1, e2, e3, e4 */ vec4<f32>((self_.g2_.x * other.g0_), (self_.g2_.y * other.g0_), (self_.g2_.z * other.g0_), 0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ (vec3<f32>(other.g0_) * vec3<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z)), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.y * other.g0_)));
}
fn multiVector_antiWedge_line(self_: MultiVector, other: Line) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((-(other.g0_.x * self_.g3_.x) - (other.g0_.y * self_.g3_.y) - (other.g0_.z * self_.g3_.z) - (other.g1_.x * self_.g2_.x) - (other.g1_.y * self_.g2_.y) - (other.g1_.z * self_.g2_.z)), 0.0), /* e1, e2, e3, e4 */ (vec4<f32>(((other.g0_.x * self_.g4_.w) + (other.g1_.y * self_.g4_.z)), ((other.g0_.y * self_.g4_.w) + (other.g1_.z * self_.g4_.x)), ((other.g0_.z * self_.g4_.w) + (other.g1_.x * self_.g4_.y)), (-(other.g0_.y * self_.g4_.y) - (other.g0_.z * self_.g4_.z))) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.x) * self_.g4_.yzxx)), /* e41, e42, e43 */ (vec3<f32>(self_.g0_.y) * other.g0_), /* e23, e31, e12 */ (vec3<f32>(self_.g0_.y) * other.g1_), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn multiVector_antiWedge_motor(self_: MultiVector, other: Motor) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(((self_.g0_.x * other.g0_.w) + (self_.g0_.y * other.g1_.w) - (self_.g2_.x * other.g1_.x) - (self_.g2_.y * other.g1_.y) - (self_.g2_.z * other.g1_.z) - (self_.g3_.x * other.g0_.x) - (self_.g3_.y * other.g0_.y) - (self_.g3_.z * other.g0_.z)), (self_.g0_.y * other.g0_.w)), /* e1, e2, e3, e4 */ (vec4<f32>(((other.g0_.w * self_.g1_.x) + (other.g1_.y * self_.g4_.z)), ((other.g0_.w * self_.g1_.y) + (other.g1_.z * self_.g4_.x)), ((other.g0_.w * self_.g1_.z) + (other.g1_.x * self_.g4_.y)), (-(other.g0_.y * self_.g4_.y) - (other.g0_.z * self_.g4_.z))) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.x) * self_.g4_.yzxx) + (vec4<f32>(self_.g4_.w, self_.g4_.w, self_.g4_.w, self_.g1_.w) * other.g0_)), /* e41, e42, e43 */ ((vec3<f32>(self_.g0_.y) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z)) + (vec3<f32>(other.g0_.w) * self_.g2_)), /* e23, e31, e12 */ ((vec3<f32>(self_.g0_.y) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z)) + (vec3<f32>(other.g0_.w) * self_.g3_)), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_.w) * self_.g4_));
}
fn multiVector_antiWedge_multiVector(self_: MultiVector, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(((other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x) - (other.g2_.x * self_.g3_.x) - (other.g2_.y * self_.g3_.y) - (other.g2_.z * self_.g3_.z) - (other.g3_.x * self_.g2_.x) - (other.g3_.y * self_.g2_.y) - (other.g3_.z * self_.g2_.z) - (other.g1_.x * self_.g4_.x) - (other.g1_.y * self_.g4_.y) - (other.g1_.z * self_.g4_.z) - (other.g1_.w * self_.g4_.w) + (other.g4_.x * self_.g1_.x) + (other.g4_.y * self_.g1_.y) + (other.g4_.z * self_.g1_.z) + (other.g4_.w * self_.g1_.w)), (other.g0_.y * self_.g0_.y)), /* e1, e2, e3, e4 */ (vec4<f32>(((other.g2_.x * self_.g4_.w) + (other.g3_.y * self_.g4_.z) + (self_.g2_.x * other.g4_.w) + (self_.g3_.y * other.g4_.z)), ((other.g2_.y * self_.g4_.w) + (other.g3_.z * self_.g4_.x) + (self_.g2_.y * other.g4_.w) + (self_.g3_.z * other.g4_.x)), ((other.g2_.z * self_.g4_.w) + (other.g3_.x * self_.g4_.y) + (self_.g2_.z * other.g4_.w) + (self_.g3_.x * other.g4_.y)), (-(other.g2_.y * self_.g4_.y) - (other.g2_.z * self_.g4_.z) - (self_.g2_.y * other.g4_.y) - (self_.g2_.z * other.g4_.z))) + (vec4<f32>(other.g0_.y) * self_.g1_) + (vec4<f32>(self_.g0_.y) * other.g1_) - (vec4<f32>(other.g3_.z, other.g3_.x, other.g3_.y, other.g2_.x) * self_.g4_.yzxx) - (vec4<f32>(self_.g3_.z, self_.g3_.x, self_.g3_.y, self_.g2_.x) * other.g4_.yzxx)), /* e41, e42, e43 */ (vec3<f32>(((other.g4_.y * self_.g4_.z) - (other.g4_.z * self_.g4_.y)), (-(other.g4_.x * self_.g4_.z) + (other.g4_.z * self_.g4_.x)), ((other.g4_.x * self_.g4_.y) - (other.g4_.y * self_.g4_.x))) + (vec3<f32>(other.g0_.y) * self_.g2_) + (vec3<f32>(self_.g0_.y) * other.g2_)), /* e23, e31, e12 */ ((vec3<f32>(other.g0_.y) * self_.g3_) + (vec3<f32>(self_.g0_.y) * other.g3_) + (vec3<f32>(other.g4_.w) * vec3<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z)) - (vec3<f32>(self_.g4_.w) * vec3<f32>(other.g4_.x, other.g4_.y, other.g4_.z))), /* e423, e431, e412, e321 */ ((vec4<f32>(other.g0_.y) * self_.g4_) + (vec4<f32>(self_.g0_.y) * other.g4_)));
}
fn multiVector_antiWedge_origin(self_: MultiVector, other: Origin) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((self_.g4_.w * other.g0_ * -1.0), 0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.y * other.g0_)), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn multiVector_antiWedge_plane(self_: MultiVector, other: Plane) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(((self_.g1_.x * other.g0_.x) + (self_.g1_.y * other.g0_.y) + (self_.g1_.z * other.g0_.z) + (self_.g1_.w * other.g0_.w)), 0.0), /* e1, e2, e3, e4 */ (vec4<f32>(((self_.g2_.x * other.g0_.w) + (self_.g3_.y * other.g0_.z)), ((self_.g2_.y * other.g0_.w) + (self_.g3_.z * other.g0_.x)), ((self_.g2_.z * other.g0_.w) + (self_.g3_.x * other.g0_.y)), (-(self_.g2_.y * other.g0_.y) - (self_.g2_.z * other.g0_.z))) - (vec4<f32>(self_.g3_.z, self_.g3_.x, self_.g3_.y, self_.g2_.x) * other.g0_.yzxx)), /* e41, e42, e43 */ vec3<f32>((-(self_.g4_.y * other.g0_.z) + (self_.g4_.z * other.g0_.y)), ((self_.g4_.x * other.g0_.z) - (self_.g4_.z * other.g0_.x)), (-(self_.g4_.x * other.g0_.y) + (self_.g4_.y * other.g0_.x))), /* e23, e31, e12 */ (-(vec3<f32>(self_.g4_.w) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z)) + (vec3<f32>(other.g0_.w) * vec3<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z))), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_.y) * other.g0_));
}
fn multiVector_antiWedge_point(self_: MultiVector, other: Point) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((-(self_.g4_.x * other.g0_.x) - (self_.g4_.y * other.g0_.y) - (self_.g4_.z * other.g0_.z) - (self_.g4_.w * other.g0_.w)), 0.0), /* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_.y) * other.g0_), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn multiVector_antiWedge_scalar(self_: MultiVector, other: Scalar) -> Scalar {
    return Scalar(/* scalar */ (self_.g0_.y * other.g0_));
}
fn origin_antiWedge_antiScalar(self_: Origin, other: AntiScalar) -> Origin {
    return Origin(/* e4 */ (other.g0_ * self_.g0_));
}
fn origin_antiWedge_dualNum(self_: Origin, other: DualNum) -> Origin {
    return Origin(/* e4 */ (other.g0_.y * self_.g0_));
}
fn origin_antiWedge_flector(self_: Origin, other: Flector) -> Scalar {
    return Scalar(/* scalar */ (other.g1_.w * self_.g0_));
}
fn origin_antiWedge_horizon(self_: Origin, other: Horizon) -> Scalar {
    return Scalar(/* scalar */ (other.g0_ * self_.g0_));
}
fn origin_antiWedge_motor(self_: Origin, other: Motor) -> Origin {
    return Origin(/* e4 */ (other.g0_.w * self_.g0_));
}
fn origin_antiWedge_multiVector(self_: Origin, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((other.g4_.w * self_.g0_), 0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.y * self_.g0_)), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn origin_antiWedge_plane(self_: Origin, other: Plane) -> Scalar {
    return Scalar(/* scalar */ (other.g0_.w * self_.g0_));
}
fn plane_antiWedge_antiScalar(self_: Plane, other: AntiScalar) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ (vec4<f32>(other.g0_) * self_.g0_));
}
fn plane_antiWedge_dualNum(self_: Plane, other: DualNum) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ (vec4<f32>(other.g0_.y) * self_.g0_));
}
fn plane_antiWedge_flector(self_: Plane, other: Flector) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(((other.g1_.y * self_.g0_.z) - (other.g1_.z * self_.g0_.y)), (-(other.g1_.x * self_.g0_.z) + (other.g1_.z * self_.g0_.x)), ((other.g1_.x * self_.g0_.y) - (other.g1_.y * self_.g0_.x)), 0.0), /* e23, e31, e12, scalar */ (vec4<f32>((other.g1_.w * self_.g0_.x), (other.g1_.w * self_.g0_.y), (other.g1_.w * self_.g0_.z), (-(other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z) - (other.g0_.w * self_.g0_.w))) - (vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, other.g0_.x) * self_.g0_.wwwx)));
}
fn plane_antiWedge_horizon(self_: Plane, other: Horizon) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ (vec3<f32>(other.g0_) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z)));
}
fn plane_antiWedge_line(self_: Plane, other: Line) -> Point {
    return Point(/* e1, e2, e3, e4 */ (vec4<f32>(((other.g0_.x * self_.g0_.w) + (other.g1_.y * self_.g0_.z)), ((other.g0_.y * self_.g0_.w) + (other.g1_.z * self_.g0_.x)), ((other.g0_.z * self_.g0_.w) + (other.g1_.x * self_.g0_.y)), (-(other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z))) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.x) * self_.g0_.yzxx)));
}
fn plane_antiWedge_motor(self_: Plane, other: Motor) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(((other.g0_.x * self_.g0_.w) + (other.g1_.y * self_.g0_.z)), ((other.g0_.y * self_.g0_.w) + (other.g1_.z * self_.g0_.x)), ((other.g0_.z * self_.g0_.w) + (other.g1_.x * self_.g0_.y)), (-(other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z))) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.x) * self_.g0_.yzxx)), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_.w) * self_.g0_));
}
fn plane_antiWedge_multiVector(self_: Plane, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((-(other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z) - (other.g1_.w * self_.g0_.w)), 0.0), /* e1, e2, e3, e4 */ (vec4<f32>(((other.g2_.x * self_.g0_.w) + (other.g3_.y * self_.g0_.z)), ((other.g2_.y * self_.g0_.w) + (other.g3_.z * self_.g0_.x)), ((other.g2_.z * self_.g0_.w) + (other.g3_.x * self_.g0_.y)), (-(other.g2_.y * self_.g0_.y) - (other.g2_.z * self_.g0_.z))) - (vec4<f32>(other.g3_.z, other.g3_.x, other.g3_.y, other.g2_.x) * self_.g0_.yzxx)), /* e41, e42, e43 */ vec3<f32>(((other.g4_.y * self_.g0_.z) - (other.g4_.z * self_.g0_.y)), (-(other.g4_.x * self_.g0_.z) + (other.g4_.z * self_.g0_.x)), ((other.g4_.x * self_.g0_.y) - (other.g4_.y * self_.g0_.x))), /* e23, e31, e12 */ ((vec3<f32>(other.g4_.w) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z)) - (vec3<f32>(self_.g0_.w) * vec3<f32>(other.g4_.x, other.g4_.y, other.g4_.z))), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_.y) * self_.g0_));
}
fn plane_antiWedge_origin(self_: Plane, other: Origin) -> Scalar {
    return Scalar(/* scalar */ (self_.g0_.w * other.g0_ * -1.0));
}
fn plane_antiWedge_plane(self_: Plane, other: Plane) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>(((other.g0_.y * self_.g0_.z) - (other.g0_.z * self_.g0_.y)), (-(other.g0_.x * self_.g0_.z) + (other.g0_.z * self_.g0_.x)), ((other.g0_.x * self_.g0_.y) - (other.g0_.y * self_.g0_.x))), /* e23, e31, e12 */ ((vec3<f32>(other.g0_.w) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z)) - (vec3<f32>(self_.g0_.w) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z))));
}
fn plane_antiWedge_point(self_: Plane, other: Point) -> Scalar {
    return Scalar(/* scalar */ (-(self_.g0_.x * other.g0_.x) - (self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z) - (self_.g0_.w * other.g0_.w)));
}
fn point_antiWedge_antiScalar(self_: Point, other: AntiScalar) -> Point {
    return Point(/* e1, e2, e3, e4 */ (vec4<f32>(other.g0_) * self_.g0_));
}
fn point_antiWedge_dualNum(self_: Point, other: DualNum) -> Point {
    return Point(/* e1, e2, e3, e4 */ (vec4<f32>(other.g0_.y) * self_.g0_));
}
fn point_antiWedge_flector(self_: Point, other: Flector) -> Scalar {
    return Scalar(/* scalar */ ((other.g1_.x * self_.g0_.x) + (other.g1_.y * self_.g0_.y) + (other.g1_.z * self_.g0_.z) + (other.g1_.w * self_.g0_.w)));
}
fn point_antiWedge_horizon(self_: Point, other: Horizon) -> Scalar {
    return Scalar(/* scalar */ (self_.g0_.w * other.g0_));
}
fn point_antiWedge_motor(self_: Point, other: Motor) -> Point {
    return Point(/* e1, e2, e3, e4 */ (vec4<f32>(other.g0_.w) * self_.g0_));
}
fn point_antiWedge_multiVector(self_: Point, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(((other.g4_.x * self_.g0_.x) + (other.g4_.y * self_.g0_.y) + (other.g4_.z * self_.g0_.z) + (other.g4_.w * self_.g0_.w)), 0.0), /* e1, e2, e3, e4 */ (vec4<f32>(other.g0_.y) * self_.g0_), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn point_antiWedge_plane(self_: Point, other: Plane) -> Scalar {
    return Scalar(/* scalar */ ((other.g0_.x * self_.g0_.x) + (other.g0_.y * self_.g0_.y) + (other.g0_.z * self_.g0_.z) + (other.g0_.w * self_.g0_.w)));
}
fn scalar_antiWedge_antiScalar(self_: Scalar, other: AntiScalar) -> Scalar {
    return Scalar(/* scalar */ (other.g0_ * self_.g0_));
}
fn scalar_antiWedge_dualNum(self_: Scalar, other: DualNum) -> Scalar {
    return Scalar(/* scalar */ (other.g0_.y * self_.g0_));
}
fn scalar_antiWedge_motor(self_: Scalar, other: Motor) -> Scalar {
    return Scalar(/* scalar */ (other.g0_.w * self_.g0_));
}
fn scalar_antiWedge_multiVector(self_: Scalar, other: MultiVector) -> Scalar {
    return Scalar(/* scalar */ (other.g0_.y * self_.g0_));
}
fn antiScalar_constraintValid(self_: AntiScalar) -> AntiScalar {
    return self_;
}
fn origin_constraintValid(self_: Origin) -> Origin {
    return self_;
}
fn dualNum_constraintViolation(self_: DualNum) -> AntiScalar {
    let geometric_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>(pow(self_.g0_.x, 2), (self_.g0_.x * self_.g0_.y)) * vec2<f32>(1.0, 2.0)));
    return AntiScalar(/* e1234 */ geometric_product.g0_.y);
}
fn flector_constraintViolation(self_: Flector) -> DualNum {
    let reverse: Flector = Flector(/* e1, e2, e3, e4 */ self_.g0_, /* e423, e431, e412, e321 */ (self_.g1_ * vec4<f32>(-1.0)));
    let geometric_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>(0.0, (-(reverse.g0_.x * self_.g1_.x) - (reverse.g0_.y * self_.g1_.y) - (reverse.g0_.z * self_.g1_.z) + (reverse.g1_.w * self_.g0_.w))) + (vec2<f32>(self_.g0_.x) * vec2<f32>(reverse.g0_.x, reverse.g1_.x)) + (vec2<f32>(self_.g0_.y) * vec2<f32>(reverse.g0_.y, reverse.g1_.y)) + (vec2<f32>(self_.g0_.z) * vec2<f32>(reverse.g0_.z, reverse.g1_.z)) - (vec2<f32>(self_.g1_.w) * vec2<f32>(reverse.g1_.w, reverse.g0_.w))));
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(self_.g0_.x, 2) + pow(self_.g0_.y, 2) + pow(self_.g0_.z, 2) - pow(self_.g1_.w, 2)));
    return DualNum(/* scalar, e1234 */ vec2<f32>((geometric_product.g0_.x - scalar_product.g0_), geometric_product.g0_.y));
}
fn horizon_constraintViolation(self_: Horizon) -> Scalar {
    let reverse: Horizon = Horizon(/* e321 */ (self_.g0_ * -1.0));
    let geometric_product: Scalar = Scalar(/* scalar */ (reverse.g0_ * self_.g0_ * -1.0));
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(self_.g0_, 2) * -1.0));
    return Scalar(/* scalar */ (geometric_product.g0_ - scalar_product.g0_));
}
fn line_constraintViolation(self_: Line) -> DualNum {
    let reverse: Line = Line(/* e41, e42, e43 */ (self_.g0_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (self_.g1_ * vec3<f32>(-1.0)));
    let geometric_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>(0.0, (-(reverse.g1_.x * self_.g0_.x) - (reverse.g1_.y * self_.g0_.y) - (reverse.g1_.z * self_.g0_.z))) - (vec2<f32>(self_.g1_.x) * vec2<f32>(reverse.g1_.x, reverse.g0_.x)) - (vec2<f32>(self_.g1_.y) * vec2<f32>(reverse.g1_.y, reverse.g0_.y)) - (vec2<f32>(self_.g1_.z) * vec2<f32>(reverse.g1_.z, reverse.g0_.z))));
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(self_.g1_.x, 2) - pow(self_.g1_.y, 2) - pow(self_.g1_.z, 2)));
    return DualNum(/* scalar, e1234 */ vec2<f32>((geometric_product.g0_.x - scalar_product.g0_), geometric_product.g0_.y));
}
fn motor_constraintViolation(self_: Motor) -> DualNum {
    let reverse: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g0_.x * -1.0), (self_.g0_.y * -1.0), (self_.g0_.z * -1.0), self_.g0_.w), /* e23, e31, e12, scalar */ vec4<f32>((self_.g1_.x * -1.0), (self_.g1_.y * -1.0), (self_.g1_.z * -1.0), self_.g1_.w));
    let geometric_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>(0.0, (-(reverse.g1_.x * self_.g0_.x) - (reverse.g1_.y * self_.g0_.y) - (reverse.g1_.z * self_.g0_.z) + (reverse.g1_.w * self_.g0_.w))) - (vec2<f32>(self_.g1_.x) * vec2<f32>(reverse.g1_.x, reverse.g0_.x)) - (vec2<f32>(self_.g1_.y) * vec2<f32>(reverse.g1_.y, reverse.g0_.y)) - (vec2<f32>(self_.g1_.z) * vec2<f32>(reverse.g1_.z, reverse.g0_.z)) + (vec2<f32>(self_.g1_.w) * vec2<f32>(reverse.g1_.w, reverse.g0_.w))));
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(self_.g1_.x, 2) - pow(self_.g1_.y, 2) - pow(self_.g1_.z, 2) + pow(self_.g1_.w, 2)));
    return DualNum(/* scalar, e1234 */ vec2<f32>((geometric_product.g0_.x - scalar_product.g0_), geometric_product.g0_.y));
}
fn multiVector_constraintViolation(self_: MultiVector) -> MultiVector {
    let reverse: MultiVector = MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ self_.g1_, /* e41, e42, e43 */ (self_.g2_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (self_.g3_ * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ (self_.g4_ * vec4<f32>(-1.0)));
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(0.0, ((reverse.g0_.y * self_.g0_.x) - (reverse.g3_.x * self_.g2_.x) - (reverse.g3_.y * self_.g2_.y) - (reverse.g3_.z * self_.g2_.z) - (reverse.g1_.x * self_.g4_.x) - (reverse.g1_.y * self_.g4_.y) - (reverse.g1_.z * self_.g4_.z) + (reverse.g4_.w * self_.g1_.w))) + (vec2<f32>(reverse.g0_.x) * self_.g0_) - (vec2<f32>(self_.g3_.x) * vec2<f32>(reverse.g3_.x, reverse.g2_.x)) - (vec2<f32>(self_.g3_.y) * vec2<f32>(reverse.g3_.y, reverse.g2_.y)) - (vec2<f32>(self_.g3_.z) * vec2<f32>(reverse.g3_.z, reverse.g2_.z)) + (vec2<f32>(self_.g1_.x) * vec2<f32>(reverse.g1_.x, reverse.g4_.x)) + (vec2<f32>(self_.g1_.y) * vec2<f32>(reverse.g1_.y, reverse.g4_.y)) + (vec2<f32>(self_.g1_.z) * vec2<f32>(reverse.g1_.z, reverse.g4_.z)) - (vec2<f32>(self_.g4_.w) * vec2<f32>(reverse.g4_.w, reverse.g1_.w))), /* e1, e2, e3, e4 */ (vec4<f32>(((reverse.g3_.y * self_.g1_.z) + (self_.g3_.x * reverse.g4_.w) - (self_.g3_.y * reverse.g1_.z)), ((reverse.g3_.z * self_.g1_.x) + (self_.g3_.y * reverse.g4_.w) - (self_.g3_.z * reverse.g1_.x)), ((reverse.g3_.x * self_.g1_.y) - (self_.g3_.x * reverse.g1_.y) + (self_.g3_.z * reverse.g4_.w)), (-(self_.g0_.y * reverse.g4_.w) - (reverse.g2_.y * self_.g1_.y) - (reverse.g2_.z * self_.g1_.z) - (reverse.g3_.x * self_.g4_.x) - (reverse.g3_.y * self_.g4_.y) - (reverse.g3_.z * self_.g4_.z) + (self_.g2_.y * reverse.g1_.y) + (self_.g2_.z * reverse.g1_.z) - (self_.g3_.x * reverse.g4_.x) - (self_.g3_.y * reverse.g4_.y) - (self_.g3_.z * reverse.g4_.z))) + (vec4<f32>(reverse.g0_.x) * self_.g1_) + (vec4<f32>(self_.g0_.x) * reverse.g1_) + (vec4<f32>(self_.g4_.w) * vec4<f32>(reverse.g3_.x, reverse.g3_.y, reverse.g3_.z, reverse.g0_.y)) - (vec4<f32>(reverse.g3_.z, reverse.g3_.x, reverse.g3_.y, reverse.g2_.x) * self_.g1_.yzxx) + (vec4<f32>(self_.g3_.z, self_.g3_.x, self_.g3_.y, self_.g2_.x) * reverse.g1_.yzxx)), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(self_.g0_.x, 2) - pow(self_.g3_.x, 2) - pow(self_.g3_.y, 2) - pow(self_.g3_.z, 2) + pow(self_.g1_.x, 2) + pow(self_.g1_.y, 2) + pow(self_.g1_.z, 2) - pow(self_.g4_.w, 2)));
    return MultiVector(/* scalar, e1234 */ vec2<f32>((geometric_product.g0_.x - scalar_product.g0_), geometric_product.g0_.y), /* e1, e2, e3, e4 */ geometric_product.g1_, /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn plane_constraintViolation(self_: Plane) -> Scalar {
    let reverse: Plane = Plane(/* e423, e431, e412, e321 */ (self_.g0_ * vec4<f32>(-1.0)));
    let geometric_product: Scalar = Scalar(/* scalar */ (reverse.g0_.w * self_.g0_.w * -1.0));
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(self_.g0_.w, 2) * -1.0));
    return Scalar(/* scalar */ (geometric_product.g0_ - scalar_product.g0_));
}
fn point_constraintViolation(self_: Point) -> Scalar {
    return Scalar(/* scalar */ 0.0);
}
fn scalar_constraintViolation(self_: Scalar) -> Scalar {
    return Scalar(/* scalar */ 0.0);
}
fn horizon_fix(self_: Horizon) -> Horizon {
    let reverse: Horizon = Horizon(/* e321 */ (self_.g0_ * -1.0));
    let geometric_product: Scalar = Scalar(/* scalar */ (reverse.g0_ * self_.g0_ * -1.0));
    let square_root: Scalar = Scalar(/* scalar */ pow(geometric_product.g0_.x, 0.5));
    let scalar_product: Scalar = Scalar(/* scalar */ pow(square_root.g0_, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Horizon(/* e321 */ (self_.g0_ * inverse.g0_));
}
fn plane_fix(self_: Plane) -> Plane {
    let reverse: Plane = Plane(/* e423, e431, e412, e321 */ (self_.g0_ * vec4<f32>(-1.0)));
    let geometric_product: Scalar = Scalar(/* scalar */ (reverse.g0_.w * self_.g0_.w * -1.0));
    let square_root: Scalar = Scalar(/* scalar */ pow(geometric_product.g0_.x, 0.5));
    let scalar_product: Scalar = Scalar(/* scalar */ pow(square_root.g0_, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Plane(/* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g0_));
}
fn point_fix(self_: Point) -> Point {
    let geometric_product: Scalar = Scalar(/* scalar */ (pow(self_.g0_.x, 2) + pow(self_.g0_.y, 2) + pow(self_.g0_.z, 2)));
    let square_root: Scalar = Scalar(/* scalar */ pow(geometric_product.g0_.x, 0.5));
    let scalar_product: Scalar = Scalar(/* scalar */ pow(square_root.g0_, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Point(/* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g0_));
}
fn scalar_fix(self_: Scalar) -> Scalar {
    let geometric_product: Scalar = Scalar(/* scalar */ pow(self_.g0_, 2));
    let square_root: Scalar = Scalar(/* scalar */ pow(geometric_product.g0_.x, 0.5));
    let scalar_product: Scalar = Scalar(/* scalar */ pow(square_root.g0_, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Scalar(/* scalar */ (inverse.g0_ * self_.g0_));
}
fn antiScalar_geometricAntiProduct_antiScalar(self_: AntiScalar, other: AntiScalar) -> AntiScalar {
    return AntiScalar(/* e1234 */ (other.g0_ * self_.g0_));
}
fn antiScalar_geometricAntiProduct_dualNum(self_: AntiScalar, other: DualNum) -> DualNum {
    return DualNum(/* scalar, e1234 */ (vec2<f32>(self_.g0_) * other.g0_));
}
fn antiScalar_geometricAntiProduct_flector(self_: AntiScalar, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_) * other.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_) * other.g1_));
}
fn antiScalar_geometricAntiProduct_horizon(self_: AntiScalar, other: Horizon) -> Horizon {
    return Horizon(/* e321 */ (self_.g0_ * other.g0_));
}
fn antiScalar_geometricAntiProduct_line(self_: AntiScalar, other: Line) -> Line {
    return Line(/* e41, e42, e43 */ (vec3<f32>(self_.g0_) * other.g0_), /* e23, e31, e12 */ (vec3<f32>(self_.g0_) * other.g1_));
}
fn antiScalar_geometricAntiProduct_motor(self_: AntiScalar, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(self_.g0_) * other.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(self_.g0_) * other.g1_));
}
fn antiScalar_geometricAntiProduct_multiVector(self_: AntiScalar, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(self_.g0_) * other.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_) * other.g1_), /* e41, e42, e43 */ (vec3<f32>(self_.g0_) * other.g2_), /* e23, e31, e12 */ (vec3<f32>(self_.g0_) * other.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_) * other.g4_));
}
fn antiScalar_geometricAntiProduct_origin(self_: AntiScalar, other: Origin) -> Origin {
    return Origin(/* e4 */ (self_.g0_ * other.g0_));
}
fn antiScalar_geometricAntiProduct_plane(self_: AntiScalar, other: Plane) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_) * other.g0_));
}
fn antiScalar_geometricAntiProduct_point(self_: AntiScalar, other: Point) -> Point {
    return Point(/* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_) * other.g0_));
}
fn antiScalar_geometricAntiProduct_scalar(self_: AntiScalar, other: Scalar) -> Scalar {
    return Scalar(/* scalar */ (self_.g0_ * other.g0_));
}
fn dualNum_geometricAntiProduct_antiScalar(self_: DualNum, other: AntiScalar) -> DualNum {
    return DualNum(/* scalar, e1234 */ (vec2<f32>(other.g0_) * self_.g0_));
}
fn dualNum_geometricAntiProduct_dualNum(self_: DualNum, other: DualNum) -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>(((other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x)), (other.g0_.y * self_.g0_.y)));
}
fn dualNum_geometricAntiProduct_flector(self_: DualNum, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(((self_.g0_.x * other.g1_.x) + (self_.g0_.y * other.g0_.x)), ((self_.g0_.x * other.g1_.y) + (self_.g0_.y * other.g0_.y)), ((self_.g0_.x * other.g1_.z) + (self_.g0_.y * other.g0_.z)), (self_.g0_.y * other.g0_.w)), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.y * other.g1_.x), (self_.g0_.y * other.g1_.y), (self_.g0_.y * other.g1_.z), ((self_.g0_.x * other.g0_.w) + (self_.g0_.y * other.g1_.w))));
}
fn dualNum_geometricAntiProduct_horizon(self_: DualNum, other: Horizon) -> Horizon {
    return Horizon(/* e321 */ (self_.g0_.y * other.g0_));
}
fn dualNum_geometricAntiProduct_line(self_: DualNum, other: Line) -> Line {
    return Line(/* e41, e42, e43 */ (vec3<f32>(self_.g0_.y) * other.g0_), /* e23, e31, e12 */ ((vec3<f32>(self_.g0_.x) * other.g0_) + (vec3<f32>(self_.g0_.y) * other.g1_)));
}
fn dualNum_geometricAntiProduct_motor(self_: DualNum, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(self_.g0_.y) * other.g0_), /* e23, e31, e12, scalar */ ((vec4<f32>(self_.g0_.x) * other.g0_) + (vec4<f32>(self_.g0_.y) * other.g1_)));
}
fn dualNum_geometricAntiProduct_multiVector(self_: DualNum, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(((self_.g0_.x * other.g0_.y) + (self_.g0_.y * other.g0_.x)), (self_.g0_.y * other.g0_.y)), /* e1, e2, e3, e4 */ vec4<f32>(((self_.g0_.x * other.g4_.x) + (self_.g0_.y * other.g1_.x)), ((self_.g0_.x * other.g4_.y) + (self_.g0_.y * other.g1_.y)), ((self_.g0_.x * other.g4_.z) + (self_.g0_.y * other.g1_.z)), (self_.g0_.y * other.g1_.w)), /* e41, e42, e43 */ (vec3<f32>(self_.g0_.y) * other.g2_), /* e23, e31, e12 */ ((vec3<f32>(self_.g0_.x) * other.g2_) + (vec3<f32>(self_.g0_.y) * other.g3_)), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.y * other.g4_.x), (self_.g0_.y * other.g4_.y), (self_.g0_.y * other.g4_.z), ((self_.g0_.x * other.g1_.w) + (self_.g0_.y * other.g4_.w))));
}
fn dualNum_geometricAntiProduct_origin(self_: DualNum, other: Origin) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.y * other.g0_)), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.x * other.g0_)));
}
fn dualNum_geometricAntiProduct_plane(self_: DualNum, other: Plane) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g0_.x), (self_.g0_.x * other.g0_.y), (self_.g0_.x * other.g0_.z), 0.0), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_.y) * other.g0_));
}
fn dualNum_geometricAntiProduct_point(self_: DualNum, other: Point) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_.y) * other.g0_), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.x * other.g0_.w)));
}
fn dualNum_geometricAntiProduct_scalar(self_: DualNum, other: Scalar) -> Scalar {
    return Scalar(/* scalar */ (self_.g0_.y * other.g0_));
}
fn flector_geometricAntiProduct_antiScalar(self_: Flector, other: AntiScalar) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(other.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_) * self_.g1_));
}
fn flector_geometricAntiProduct_dualNum(self_: Flector, other: DualNum) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((-(other.g0_.x * self_.g1_.x) + (other.g0_.y * self_.g0_.x)), (-(other.g0_.x * self_.g1_.y) + (other.g0_.y * self_.g0_.y)), (-(other.g0_.x * self_.g1_.z) + (other.g0_.y * self_.g0_.z)), (other.g0_.y * self_.g0_.w)), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.y * self_.g1_.x), (other.g0_.y * self_.g1_.y), (other.g0_.y * self_.g1_.z), (-(other.g0_.x * self_.g0_.w) + (other.g0_.y * self_.g1_.w))));
}
fn flector_geometricAntiProduct_flector(self_: Flector, other: Flector) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>((-(other.g1_.x * self_.g0_.w) - (other.g1_.z * self_.g1_.y)), (-(other.g1_.x * self_.g1_.z) - (other.g1_.y * self_.g0_.w)), (-(other.g1_.y * self_.g1_.x) - (other.g1_.z * self_.g0_.w)), ((other.g1_.y * self_.g1_.y) + (other.g1_.z * self_.g1_.z))) - (vec4<f32>(other.g0_.w) * vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g0_.w)) + (other.g1_.yzxx * self_.g1_.zxyx)), /* e23, e31, e12, scalar */ (vec4<f32>(((other.g0_.z * self_.g1_.y) - (other.g1_.z * self_.g0_.y)), ((other.g0_.x * self_.g1_.z) - (other.g1_.x * self_.g0_.z)), ((other.g0_.y * self_.g1_.x) - (other.g1_.y * self_.g0_.x)), (-(other.g0_.w * self_.g1_.w) + (other.g1_.w * self_.g0_.w))) + (vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.x) * self_.g0_.wwwx) - (vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, other.g0_.z) * self_.g1_.wwwz) - (vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g1_.y) * other.g0_.wwwy) + (vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g0_.z) * other.g1_.wwwz) - (other.g0_.yzxx * self_.g1_.zxyx) + (other.g1_.yzxy * self_.g0_.zxyy)));
}
fn flector_geometricAntiProduct_horizon(self_: Flector, other: Horizon) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0), /* e23, e31, e12, scalar */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g0_.w)));
}
fn flector_geometricAntiProduct_line(self_: Flector, other: Line) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(((other.g0_.x * self_.g1_.w) - (other.g0_.y * self_.g0_.z) + (other.g0_.z * self_.g0_.y) - (other.g1_.x * self_.g0_.w) + (other.g1_.y * self_.g1_.z)), ((other.g0_.x * self_.g0_.z) + (other.g0_.y * self_.g1_.w) - (other.g0_.z * self_.g0_.x) - (other.g1_.y * self_.g0_.w) + (other.g1_.z * self_.g1_.x)), (-(other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x) + (other.g0_.z * self_.g1_.w) + (other.g1_.x * self_.g1_.y) - (other.g1_.z * self_.g0_.w)), (-(other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z))) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.x) * self_.g1_.yzxx)), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.x * self_.g0_.w) - (other.g0_.y * self_.g1_.z)), ((other.g0_.y * self_.g0_.w) - (other.g0_.z * self_.g1_.x)), (-(other.g0_.x * self_.g1_.y) + (other.g0_.z * self_.g0_.w)), (-(other.g0_.x * self_.g0_.x) - (other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z) + (other.g1_.y * self_.g1_.y) + (other.g1_.z * self_.g1_.z))) + (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.y, other.g1_.x) * self_.g1_.yzxx)));
}
fn flector_geometricAntiProduct_motor(self_: Flector, other: Motor) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(((self_.g0_.y * other.g0_.z) - (self_.g0_.w * other.g1_.x) + (self_.g1_.z * other.g1_.y) + (self_.g1_.w * other.g0_.x)), ((self_.g0_.z * other.g0_.x) - (self_.g0_.w * other.g1_.y) + (self_.g1_.x * other.g1_.z) + (self_.g1_.w * other.g0_.y)), ((self_.g0_.z * other.g0_.w) - (self_.g0_.w * other.g1_.z) + (self_.g1_.y * other.g1_.x) + (self_.g1_.w * other.g0_.z)), 0.0) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g1_.x) * other.g0_.yzxx) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.w, other.g0_.z) * self_.g1_.yzzz) - (vec4<f32>(other.g1_.w, other.g1_.w, other.g1_.y, other.g0_.y) * self_.g1_.xyxy) + (self_.g0_.xyxw * other.g0_.wwyw)), /* e423, e431, e412, e321 */ (vec4<f32>(0.0, 0.0, 0.0, (-(self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z) - (self_.g0_.w * other.g1_.w) + (self_.g1_.z * other.g1_.z))) + (vec4<f32>(self_.g0_.w, self_.g0_.w, self_.g0_.w, self_.g1_.w) * other.g0_) - (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g0_.x) * other.g0_.yzxx) + (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.w, other.g1_.y) * self_.g1_.yzzy) + (vec4<f32>(other.g0_.w, other.g0_.w, other.g0_.y, other.g1_.x) * self_.g1_.xyxx)));
}
fn flector_geometricAntiProduct_multiVector(self_: Flector, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(((self_.g0_.w * other.g4_.w) - (self_.g1_.x * other.g1_.x) - (self_.g1_.y * other.g1_.y) - (self_.g1_.z * other.g1_.z)), 0.0) - (vec2<f32>(other.g1_.w) * vec2<f32>(self_.g1_.w, self_.g0_.w)) + (vec2<f32>(other.g4_.x) * vec2<f32>(self_.g0_.x, self_.g1_.x)) + (vec2<f32>(other.g4_.y) * vec2<f32>(self_.g0_.y, self_.g1_.y)) + (vec2<f32>(other.g4_.z) * vec2<f32>(self_.g0_.z, self_.g1_.z))), /* e1, e2, e3, e4 */ (vec4<f32>(((other.g2_.x * self_.g1_.w) - (other.g2_.y * self_.g0_.z) + (other.g2_.z * self_.g0_.y) - (other.g3_.x * self_.g0_.w) + (other.g3_.y * self_.g1_.z)), ((other.g2_.x * self_.g0_.z) + (other.g2_.y * self_.g1_.w) - (other.g2_.z * self_.g0_.x) - (other.g3_.y * self_.g0_.w) + (other.g3_.z * self_.g1_.x)), (-(other.g2_.x * self_.g0_.y) + (other.g2_.y * self_.g0_.x) + (other.g2_.z * self_.g1_.w) + (other.g3_.x * self_.g1_.y) - (other.g3_.z * self_.g0_.w)), ((other.g2_.z * self_.g1_.z) * -1.0)) + (vec4<f32>(other.g0_.y) * self_.g0_) - (vec4<f32>(other.g0_.x, other.g0_.x, other.g0_.x, other.g2_.x) * self_.g1_.xyzx) - (vec4<f32>(other.g3_.z, other.g3_.x, other.g3_.y, other.g2_.y) * self_.g1_.yzxy)), /* e41, e42, e43 */ (vec3<f32>((-(self_.g1_.y * other.g4_.z) + (self_.g1_.z * other.g4_.y)), ((self_.g1_.x * other.g4_.z) - (self_.g1_.z * other.g4_.x)), (-(self_.g1_.x * other.g4_.y) + (self_.g1_.y * other.g4_.x))) - (vec3<f32>(self_.g0_.w) * vec3<f32>(other.g4_.x, other.g4_.y, other.g4_.z)) - (vec3<f32>(other.g1_.w) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z))), /* e23, e31, e12 */ (vec3<f32>((-(self_.g0_.y * other.g4_.z) + (self_.g0_.z * other.g4_.y) + (self_.g1_.y * other.g1_.z) - (self_.g1_.z * other.g1_.y)), ((self_.g0_.x * other.g4_.z) - (self_.g0_.z * other.g4_.x) - (self_.g1_.x * other.g1_.z) + (self_.g1_.z * other.g1_.x)), (-(self_.g0_.x * other.g4_.y) + (self_.g0_.y * other.g4_.x) + (self_.g1_.x * other.g1_.y) - (self_.g1_.y * other.g1_.x))) + (vec3<f32>(self_.g0_.w) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z)) - (vec3<f32>(self_.g1_.w) * vec3<f32>(other.g4_.x, other.g4_.y, other.g4_.z)) - (vec3<f32>(other.g1_.w) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z)) + (vec3<f32>(other.g4_.w) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z))), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g2_.x * self_.g0_.w) - (other.g2_.y * self_.g1_.z)), ((other.g2_.y * self_.g0_.w) - (other.g2_.z * self_.g1_.x)), (-(other.g2_.x * self_.g1_.y) + (other.g2_.z * self_.g0_.w)), (-(other.g0_.x * self_.g0_.w) - (other.g2_.x * self_.g0_.x) - (other.g2_.y * self_.g0_.y) - (other.g2_.z * self_.g0_.z) + (other.g3_.y * self_.g1_.y) + (other.g3_.z * self_.g1_.z))) + (vec4<f32>(other.g0_.y) * self_.g1_) + (vec4<f32>(other.g2_.z, other.g2_.x, other.g2_.y, other.g3_.x) * self_.g1_.yzxx)));
}
fn flector_geometricAntiProduct_origin(self_: Flector, other: Origin) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g0_.w) * vec4<f32>(-1.0)), /* e23, e31, e12, scalar */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g1_.w) * vec4<f32>(-1.0)));
}
fn flector_geometricAntiProduct_plane(self_: Flector, other: Plane) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>((-(self_.g0_.w * other.g0_.x) - (self_.g1_.y * other.g0_.z)), (-(self_.g0_.w * other.g0_.y) - (self_.g1_.z * other.g0_.x)), (-(self_.g0_.w * other.g0_.z) - (self_.g1_.x * other.g0_.y)), ((self_.g1_.y * other.g0_.y) + (self_.g1_.z * other.g0_.z))) + (self_.g1_.zxyx * other.g0_.yzxx)), /* e23, e31, e12, scalar */ (vec4<f32>((-(self_.g0_.y * other.g0_.z) - (self_.g1_.w * other.g0_.x)), (-(self_.g0_.z * other.g0_.x) - (self_.g1_.w * other.g0_.y)), (-(self_.g0_.x * other.g0_.y) - (self_.g1_.w * other.g0_.z)), ((self_.g0_.z * other.g0_.z) + (self_.g0_.w * other.g0_.w))) + (vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g0_.y) * other.g0_.wwwy) + (self_.g0_.zxyx * other.g0_.yzxx)));
}
fn flector_geometricAntiProduct_point(self_: Flector, other: Point) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(other.g0_.w) * vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g0_.w) * vec4<f32>(-1.0)), /* e23, e31, e12, scalar */ (vec4<f32>(((self_.g0_.w * other.g0_.x) + (self_.g1_.y * other.g0_.z)), ((self_.g0_.w * other.g0_.y) + (self_.g1_.z * other.g0_.x)), ((self_.g0_.w * other.g0_.z) + (self_.g1_.x * other.g0_.y)), (-(self_.g1_.z * other.g0_.z) - (self_.g1_.w * other.g0_.w))) - (vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g1_.x) * other.g0_.wwwx) - (self_.g1_.zxyy * other.g0_.yzxy)));
}
fn flector_geometricAntiProduct_scalar(self_: Flector, other: Scalar) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g1_.x * other.g0_ * -1.0), (self_.g1_.y * other.g0_ * -1.0), (self_.g1_.z * other.g0_ * -1.0), 0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.w * other.g0_ * -1.0)));
}
fn horizon_geometricAntiProduct_antiScalar(self_: Horizon, other: AntiScalar) -> Horizon {
    return Horizon(/* e321 */ (other.g0_ * self_.g0_));
}
fn horizon_geometricAntiProduct_dualNum(self_: Horizon, other: DualNum) -> Horizon {
    return Horizon(/* e321 */ (other.g0_.y * self_.g0_));
}
fn horizon_geometricAntiProduct_flector(self_: Horizon, other: Flector) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0), /* e23, e31, e12, scalar */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, other.g0_.w) * vec4<f32>(-1.0)));
}
fn horizon_geometricAntiProduct_line(self_: Horizon, other: Line) -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), 0.0));
}
fn horizon_geometricAntiProduct_motor(self_: Horizon, other: Motor) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), 0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.w * self_.g0_)));
}
fn horizon_geometricAntiProduct_multiVector(self_: Horizon, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((other.g1_.w * self_.g0_ * -1.0), 0.0), /* e1, e2, e3, e4 */ vec4<f32>((other.g2_.x * self_.g0_), (other.g2_.y * self_.g0_), (other.g2_.z * self_.g0_), 0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ (vec3<f32>(self_.g0_) * vec3<f32>(other.g4_.x, other.g4_.y, other.g4_.z) * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.y * self_.g0_)));
}
fn horizon_geometricAntiProduct_origin(self_: Horizon, other: Origin) -> Scalar {
    return Scalar(/* scalar */ (self_.g0_ * other.g0_ * -1.0));
}
fn horizon_geometricAntiProduct_plane(self_: Horizon, other: Plane) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ (vec3<f32>(self_.g0_) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z) * vec3<f32>(-1.0)));
}
fn horizon_geometricAntiProduct_point(self_: Horizon, other: Point) -> Scalar {
    return Scalar(/* scalar */ (other.g0_.w * self_.g0_ * -1.0));
}
fn line_geometricAntiProduct_antiScalar(self_: Line, other: AntiScalar) -> Line {
    return Line(/* e41, e42, e43 */ (vec3<f32>(other.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(other.g0_) * self_.g1_));
}
fn line_geometricAntiProduct_dualNum(self_: Line, other: DualNum) -> Line {
    return Line(/* e41, e42, e43 */ (vec3<f32>(other.g0_.y) * self_.g0_), /* e23, e31, e12 */ ((vec3<f32>(other.g0_.x) * self_.g0_) + (vec3<f32>(other.g0_.y) * self_.g1_)));
}
fn line_geometricAntiProduct_flector(self_: Line, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(((self_.g0_.x * other.g1_.w) + (self_.g0_.y * other.g0_.z) - (self_.g0_.z * other.g0_.y) + (self_.g1_.x * other.g0_.w) + (self_.g1_.y * other.g1_.z)), (-(self_.g0_.x * other.g0_.z) + (self_.g0_.y * other.g1_.w) + (self_.g0_.z * other.g0_.x) + (self_.g1_.y * other.g0_.w) + (self_.g1_.z * other.g1_.x)), ((self_.g0_.x * other.g0_.y) - (self_.g0_.y * other.g0_.x) + (self_.g0_.z * other.g1_.w) + (self_.g1_.x * other.g1_.y) + (self_.g1_.z * other.g0_.w)), (-(self_.g0_.y * other.g1_.y) - (self_.g0_.z * other.g1_.z))) - (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g0_.x) * other.g1_.yzxx)), /* e423, e431, e412, e321 */ (vec4<f32>(((self_.g0_.x * other.g0_.w) + (self_.g0_.y * other.g1_.z)), ((self_.g0_.y * other.g0_.w) + (self_.g0_.z * other.g1_.x)), ((self_.g0_.x * other.g1_.y) + (self_.g0_.z * other.g0_.w)), (-(self_.g0_.x * other.g0_.x) - (self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z) - (self_.g1_.y * other.g1_.y) - (self_.g1_.z * other.g1_.z))) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g1_.x) * other.g1_.yzxx)));
}
fn line_geometricAntiProduct_horizon(self_: Line, other: Horizon) -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), 0.0));
}
fn line_geometricAntiProduct_line(self_: Line, other: Line) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((-(other.g0_.y * self_.g0_.z) + (other.g0_.z * self_.g0_.y)), ((other.g0_.x * self_.g0_.z) - (other.g0_.z * self_.g0_.x)), (-(other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x)), (-(other.g0_.x * self_.g0_.x) - (other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z))), /* e23, e31, e12, scalar */ vec4<f32>((-(other.g0_.y * self_.g1_.z) + (other.g0_.z * self_.g1_.y) - (other.g1_.y * self_.g0_.z) + (other.g1_.z * self_.g0_.y)), ((other.g0_.x * self_.g1_.z) - (other.g0_.z * self_.g1_.x) + (other.g1_.x * self_.g0_.z) - (other.g1_.z * self_.g0_.x)), (-(other.g0_.x * self_.g1_.y) + (other.g0_.y * self_.g1_.x) - (other.g1_.x * self_.g0_.y) + (other.g1_.y * self_.g0_.x)), (-(other.g0_.x * self_.g1_.x) - (other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))));
}
fn line_geometricAntiProduct_motor(self_: Line, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(((self_.g0_.x * other.g0_.w) + (self_.g0_.y * other.g0_.z)), ((self_.g0_.y * other.g0_.w) + (self_.g0_.z * other.g0_.x)), ((self_.g0_.x * other.g0_.y) + (self_.g0_.z * other.g0_.w)), (-(self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z))) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g0_.x) * other.g0_.yzxx)), /* e23, e31, e12, scalar */ (vec4<f32>(((self_.g0_.x * other.g1_.w) + (self_.g0_.y * other.g1_.z) + (self_.g1_.x * other.g0_.w) + (self_.g1_.y * other.g0_.z)), ((self_.g0_.y * other.g1_.w) + (self_.g0_.z * other.g1_.x) + (self_.g1_.y * other.g0_.w) + (self_.g1_.z * other.g0_.x)), ((self_.g0_.x * other.g1_.y) + (self_.g0_.z * other.g1_.w) + (self_.g1_.x * other.g0_.y) + (self_.g1_.z * other.g0_.w)), (-(self_.g0_.y * other.g1_.y) - (self_.g0_.z * other.g1_.z) - (self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g0_.x) * other.g1_.yzxx) - (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g1_.x) * other.g0_.yzxx)));
}
fn line_geometricAntiProduct_multiVector(self_: Line, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (vec2<f32>((-(self_.g1_.x * other.g2_.x) - (self_.g1_.y * other.g2_.y) - (self_.g1_.z * other.g2_.z)), 0.0) - (vec2<f32>(self_.g0_.x) * vec2<f32>(other.g3_.x, other.g2_.x)) - (vec2<f32>(self_.g0_.y) * vec2<f32>(other.g3_.y, other.g2_.y)) - (vec2<f32>(self_.g0_.z) * vec2<f32>(other.g3_.z, other.g2_.z))), /* e1, e2, e3, e4 */ (vec4<f32>(((self_.g0_.x * other.g4_.w) + (self_.g0_.y * other.g1_.z) - (self_.g0_.z * other.g1_.y) + (self_.g1_.x * other.g1_.w) + (self_.g1_.y * other.g4_.z)), (-(self_.g0_.x * other.g1_.z) + (self_.g0_.y * other.g4_.w) + (self_.g0_.z * other.g1_.x) + (self_.g1_.y * other.g1_.w) + (self_.g1_.z * other.g4_.x)), ((self_.g0_.x * other.g1_.y) - (self_.g0_.y * other.g1_.x) + (self_.g0_.z * other.g4_.w) + (self_.g1_.x * other.g4_.y) + (self_.g1_.z * other.g1_.w)), (-(self_.g0_.y * other.g4_.y) - (self_.g0_.z * other.g4_.z))) - (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g0_.x) * other.g4_.yzxx)), /* e41, e42, e43 */ ((vec3<f32>(other.g0_.y) * self_.g0_) + (self_.g0_.yzx * other.g2_.zxy) - (self_.g0_.zxy * other.g2_.yzx)), /* e23, e31, e12 */ ((vec3<f32>(other.g0_.x) * self_.g0_) + (vec3<f32>(other.g0_.y) * self_.g1_) + (self_.g0_.yzx * other.g3_.zxy) - (self_.g0_.zxy * other.g3_.yzx) + (self_.g1_.yzx * other.g2_.zxy) - (self_.g1_.zxy * other.g2_.yzx)), /* e423, e431, e412, e321 */ (vec4<f32>(((self_.g0_.x * other.g1_.w) + (self_.g0_.y * other.g4_.z)), ((self_.g0_.y * other.g1_.w) + (self_.g0_.z * other.g4_.x)), ((self_.g0_.x * other.g4_.y) + (self_.g0_.z * other.g1_.w)), (-(self_.g0_.x * other.g1_.x) - (self_.g0_.y * other.g1_.y) - (self_.g0_.z * other.g1_.z) - (self_.g1_.y * other.g4_.y) - (self_.g1_.z * other.g4_.z))) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g1_.x) * other.g4_.yzxx)));
}
fn line_geometricAntiProduct_origin(self_: Line, other: Origin) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), 0.0), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), 0.0));
}
fn line_geometricAntiProduct_plane(self_: Line, other: Plane) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(((self_.g0_.x * other.g0_.w) + (self_.g1_.y * other.g0_.z)), ((self_.g0_.y * other.g0_.w) + (self_.g1_.z * other.g0_.x)), ((self_.g0_.z * other.g0_.w) + (self_.g1_.x * other.g0_.y)), (-(self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z))) - (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g0_.x) * other.g0_.yzxx)), /* e423, e431, e412, e321 */ (vec4<f32>((self_.g0_.y * other.g0_.z), (self_.g0_.z * other.g0_.x), (self_.g0_.x * other.g0_.y), (-(self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g1_.x) * other.g0_.yzxx)));
}
fn line_geometricAntiProduct_point(self_: Line, other: Point) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(((self_.g0_.y * other.g0_.z) - (self_.g0_.z * other.g0_.y) + (self_.g1_.x * other.g0_.w)), (-(self_.g0_.x * other.g0_.z) + (self_.g0_.z * other.g0_.x) + (self_.g1_.y * other.g0_.w)), ((self_.g0_.x * other.g0_.y) - (self_.g0_.y * other.g0_.x) + (self_.g1_.z * other.g0_.w)), 0.0), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.x * other.g0_.w), (self_.g0_.y * other.g0_.w), (self_.g0_.z * other.g0_.w), (-(self_.g0_.x * other.g0_.x) - (self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z))));
}
fn line_geometricAntiProduct_scalar(self_: Line, other: Scalar) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ (vec3<f32>(other.g0_) * self_.g0_));
}
fn motor_geometricAntiProduct_antiScalar(self_: Motor, other: AntiScalar) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(other.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(other.g0_) * self_.g1_));
}
fn motor_geometricAntiProduct_dualNum(self_: Motor, other: DualNum) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(other.g0_.y) * self_.g0_), /* e23, e31, e12, scalar */ ((vec4<f32>(other.g0_.x) * self_.g0_) + (vec4<f32>(other.g0_.y) * self_.g1_)));
}
fn motor_geometricAntiProduct_flector(self_: Motor, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(((other.g0_.z * self_.g0_.y) + (other.g0_.w * self_.g1_.x) + (other.g1_.x * self_.g1_.w) + (other.g1_.z * self_.g1_.y) + (other.g1_.w * self_.g0_.x)), ((other.g0_.y * self_.g0_.w) + (other.g0_.w * self_.g1_.y) + (other.g1_.x * self_.g1_.z) + (other.g1_.y * self_.g1_.w) + (other.g1_.w * self_.g0_.y)), ((other.g0_.z * self_.g0_.w) + (other.g0_.w * self_.g1_.z) + (other.g1_.y * self_.g1_.x) + (other.g1_.z * self_.g1_.w) + (other.g1_.w * self_.g0_.z)), ((other.g1_.z * self_.g0_.z) * -1.0)) - (vec4<f32>(other.g0_.y, other.g0_.z, other.g0_.x, other.g1_.x) * self_.g0_.zxyx) - (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g0_.y) * other.g1_.yzxy) + (other.g0_.xxyw * self_.g0_.wzxw)), /* e423, e431, e412, e321 */ (vec4<f32>((other.g1_.z * self_.g0_.y), (other.g1_.y * self_.g0_.w), (other.g1_.z * self_.g0_.w), (-(other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z) - (other.g1_.x * self_.g1_.x) - (other.g1_.y * self_.g1_.y) - (other.g1_.z * self_.g1_.z))) + (vec4<f32>(other.g0_.w) * vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g1_.w)) - (vec4<f32>(other.g1_.y, other.g1_.z, other.g1_.x, other.g0_.x) * self_.g0_.zxyx) + (other.g1_.xxyw * self_.g0_.wzxw)));
}
fn motor_geometricAntiProduct_horizon(self_: Motor, other: Horizon) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), 0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.w * other.g0_)));
}
fn motor_geometricAntiProduct_line(self_: Motor, other: Line) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(((other.g0_.x * self_.g0_.w) + (other.g0_.z * self_.g0_.y)), ((other.g0_.x * self_.g0_.z) + (other.g0_.y * self_.g0_.w)), ((other.g0_.y * self_.g0_.x) + (other.g0_.z * self_.g0_.w)), (-(other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z))) - (vec4<f32>(other.g0_.y, other.g0_.z, other.g0_.x, other.g0_.x) * self_.g0_.zxyx)), /* e23, e31, e12, scalar */ (vec4<f32>(((other.g0_.x * self_.g1_.w) + (other.g0_.z * self_.g1_.y) + (other.g1_.x * self_.g0_.w) + (other.g1_.z * self_.g0_.y)), ((other.g0_.x * self_.g1_.z) + (other.g0_.y * self_.g1_.w) + (other.g1_.x * self_.g0_.z) + (other.g1_.y * self_.g0_.w)), ((other.g0_.y * self_.g1_.x) + (other.g0_.z * self_.g1_.w) + (other.g1_.y * self_.g0_.x) + (other.g1_.z * self_.g0_.w)), (-(other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))) - (vec4<f32>(other.g0_.y, other.g0_.z, other.g0_.x, other.g0_.x) * self_.g1_.zxyx) - (vec4<f32>(other.g1_.y, other.g1_.z, other.g1_.x, other.g1_.x) * self_.g0_.zxyx)));
}
fn motor_geometricAntiProduct_motor(self_: Motor, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(((other.g0_.z * self_.g0_.y) + (other.g0_.w * self_.g0_.x)), ((other.g0_.y * self_.g0_.w) + (other.g0_.w * self_.g0_.y)), ((other.g0_.z * self_.g0_.w) + (other.g0_.w * self_.g0_.z)), (-(other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z))) + (other.g0_.xxyw * self_.g0_.wzxw) - (other.g0_.yzxx * self_.g0_.zxyx)), /* e23, e31, e12, scalar */ (vec4<f32>(((other.g0_.z * self_.g1_.y) + (other.g0_.w * self_.g1_.x) + (other.g1_.z * self_.g0_.y) + (other.g1_.w * self_.g0_.x)), ((other.g0_.y * self_.g1_.w) + (other.g0_.w * self_.g1_.y) + (other.g1_.y * self_.g0_.w) + (other.g1_.w * self_.g0_.y)), ((other.g0_.z * self_.g1_.w) + (other.g0_.w * self_.g1_.z) + (other.g1_.z * self_.g0_.w) + (other.g1_.w * self_.g0_.z)), (-(other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))) + (other.g0_.xxyw * self_.g1_.wzxw) - (other.g0_.yzxx * self_.g1_.zxyx) + (other.g1_.xxyw * self_.g0_.wzxw) - (other.g1_.yzxx * self_.g0_.zxyx)));
}
fn motor_geometricAntiProduct_multiVector(self_: Motor, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(((other.g0_.y * self_.g1_.w) - (other.g3_.x * self_.g0_.x) - (other.g3_.y * self_.g0_.y) - (other.g3_.z * self_.g0_.z)), 0.0) - (vec2<f32>(other.g2_.x) * vec2<f32>(self_.g1_.x, self_.g0_.x)) - (vec2<f32>(other.g2_.y) * vec2<f32>(self_.g1_.y, self_.g0_.y)) - (vec2<f32>(other.g2_.z) * vec2<f32>(self_.g1_.z, self_.g0_.z)) + (vec2<f32>(self_.g0_.w) * other.g0_)), /* e1, e2, e3, e4 */ (vec4<f32>(((self_.g0_.y * other.g1_.z) + (self_.g0_.w * other.g1_.x) + (self_.g1_.x * other.g1_.w) + (self_.g1_.y * other.g4_.z) + (self_.g1_.w * other.g4_.x)), ((self_.g0_.z * other.g1_.x) + (self_.g0_.w * other.g1_.y) + (self_.g1_.y * other.g1_.w) + (self_.g1_.z * other.g4_.x) + (self_.g1_.w * other.g4_.y)), ((self_.g0_.z * other.g4_.w) + (self_.g0_.w * other.g1_.z) + (self_.g1_.x * other.g4_.y) + (self_.g1_.z * other.g1_.w) + (self_.g1_.w * other.g4_.z)), ((self_.g0_.z * other.g4_.z) * -1.0)) - (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g0_.y) * other.g4_.yzxy) - (vec4<f32>(other.g1_.y, other.g1_.z, other.g1_.x, other.g4_.x) * self_.g0_.zxyx) + (vec4<f32>(other.g4_.w, other.g4_.w, other.g1_.y, other.g1_.w) * self_.g0_.xyxw)), /* e41, e42, e43 */ ((vec3<f32>(other.g0_.y) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z)) + (vec3<f32>(self_.g0_.y, self_.g0_.w, self_.g0_.w) * other.g2_.zyz) - (vec3<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y) * other.g2_.yzx) + (vec3<f32>(self_.g0_.w, self_.g0_.z, self_.g0_.x) * other.g2_.xxy)), /* e23, e31, e12 */ ((vec3<f32>(other.g0_.x) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z)) + (vec3<f32>(other.g0_.y) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z)) + (vec3<f32>(self_.g0_.y, self_.g0_.w, self_.g0_.w) * other.g3_.zyz) - (vec3<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y) * other.g3_.yzx) + (vec3<f32>(self_.g0_.w, self_.g0_.z, self_.g0_.x) * other.g3_.xxy) + (vec3<f32>(self_.g1_.y, self_.g1_.w, self_.g1_.w) * other.g2_.zyz) - (vec3<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y) * other.g2_.yzx) + (vec3<f32>(self_.g1_.w, self_.g1_.z, self_.g1_.x) * other.g2_.xxy)), /* e423, e431, e412, e321 */ (vec4<f32>(((self_.g0_.y * other.g4_.z) + (self_.g0_.w * other.g4_.x)), ((self_.g0_.z * other.g4_.x) + (self_.g0_.w * other.g4_.y)), ((self_.g0_.z * other.g1_.w) + (self_.g0_.w * other.g4_.z)), (-(self_.g0_.y * other.g1_.y) - (self_.g0_.z * other.g1_.z) - (self_.g1_.x * other.g4_.x) - (self_.g1_.y * other.g4_.y) - (self_.g1_.z * other.g4_.z) + (self_.g1_.w * other.g1_.w))) + (vec4<f32>(other.g1_.w, other.g1_.w, other.g4_.y, other.g4_.w) * self_.g0_.xyxw) - (vec4<f32>(other.g4_.y, other.g4_.z, other.g4_.x, other.g1_.x) * self_.g0_.zxyx)));
}
fn motor_geometricAntiProduct_origin(self_: Motor, other: Origin) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g0_.w)), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g1_.w)));
}
fn motor_geometricAntiProduct_plane(self_: Motor, other: Plane) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(((self_.g0_.x * other.g0_.w) + (self_.g1_.y * other.g0_.z) + (self_.g1_.w * other.g0_.x)), ((self_.g0_.y * other.g0_.w) + (self_.g1_.z * other.g0_.x) + (self_.g1_.w * other.g0_.y)), ((self_.g0_.z * other.g0_.w) + (self_.g1_.x * other.g0_.y) + (self_.g1_.w * other.g0_.z)), (-(self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z))) - (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g0_.x) * other.g0_.yzxx)), /* e423, e431, e412, e321 */ (vec4<f32>((self_.g0_.w * other.g0_.x), (self_.g0_.w * other.g0_.y), (self_.g0_.w * other.g0_.z), (-(self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g1_.x) * other.g0_.yzxx) + (self_.g0_.yzxw * other.g0_.zxyw)));
}
fn motor_geometricAntiProduct_point(self_: Motor, other: Point) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(((self_.g0_.y * other.g0_.z) - (self_.g0_.z * other.g0_.y) + (self_.g0_.w * other.g0_.x) + (self_.g1_.x * other.g0_.w)), (-(self_.g0_.x * other.g0_.z) + (self_.g0_.z * other.g0_.x) + (self_.g0_.w * other.g0_.y) + (self_.g1_.y * other.g0_.w)), ((self_.g0_.x * other.g0_.y) - (self_.g0_.y * other.g0_.x) + (self_.g0_.w * other.g0_.z) + (self_.g1_.z * other.g0_.w)), (self_.g0_.w * other.g0_.w)), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.x * other.g0_.w), (self_.g0_.y * other.g0_.w), (self_.g0_.z * other.g0_.w), (-(self_.g0_.x * other.g0_.x) - (self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z) + (self_.g1_.w * other.g0_.w))));
}
fn motor_geometricAntiProduct_scalar(self_: Motor, other: Scalar) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0), /* e23, e31, e12, scalar */ (vec4<f32>(other.g0_) * self_.g0_));
}
fn multiVector_geometricAntiProduct_antiScalar(self_: MultiVector, other: AntiScalar) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(other.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(other.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(other.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(other.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_) * self_.g4_));
}
fn multiVector_geometricAntiProduct_dualNum(self_: MultiVector, other: DualNum) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(((other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x)), (other.g0_.y * self_.g0_.y)), /* e1, e2, e3, e4 */ vec4<f32>((-(other.g0_.x * self_.g4_.x) + (other.g0_.y * self_.g1_.x)), (-(other.g0_.x * self_.g4_.y) + (other.g0_.y * self_.g1_.y)), (-(other.g0_.x * self_.g4_.z) + (other.g0_.y * self_.g1_.z)), (other.g0_.y * self_.g1_.w)), /* e41, e42, e43 */ (vec3<f32>(other.g0_.y) * self_.g2_), /* e23, e31, e12 */ ((vec3<f32>(other.g0_.x) * self_.g2_) + (vec3<f32>(other.g0_.y) * self_.g3_)), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.y * self_.g4_.x), (other.g0_.y * self_.g4_.y), (other.g0_.y * self_.g4_.z), (-(other.g0_.x * self_.g1_.w) + (other.g0_.y * self_.g4_.w))));
}
fn multiVector_geometricAntiProduct_flector(self_: MultiVector, other: Flector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (vec2<f32>((-(other.g0_.x * self_.g4_.x) - (other.g0_.y * self_.g4_.y) - (other.g0_.z * self_.g4_.z) + (other.g1_.w * self_.g1_.w)), 0.0) - (vec2<f32>(other.g0_.w) * vec2<f32>(self_.g4_.w, self_.g1_.w)) + (vec2<f32>(other.g1_.x) * vec2<f32>(self_.g1_.x, self_.g4_.x)) + (vec2<f32>(other.g1_.y) * vec2<f32>(self_.g1_.y, self_.g4_.y)) + (vec2<f32>(other.g1_.z) * vec2<f32>(self_.g1_.z, self_.g4_.z))), /* e1, e2, e3, e4 */ (vec4<f32>(((self_.g0_.x * other.g1_.x) + (self_.g2_.x * other.g1_.w) + (self_.g2_.y * other.g0_.z) - (self_.g2_.z * other.g0_.y) + (self_.g3_.x * other.g0_.w) + (self_.g3_.y * other.g1_.z)), ((self_.g0_.x * other.g1_.y) - (self_.g2_.x * other.g0_.z) + (self_.g2_.y * other.g1_.w) + (self_.g2_.z * other.g0_.x) + (self_.g3_.y * other.g0_.w) + (self_.g3_.z * other.g1_.x)), ((self_.g0_.x * other.g1_.z) + (self_.g2_.x * other.g0_.y) - (self_.g2_.y * other.g0_.x) + (self_.g2_.z * other.g1_.w) + (self_.g3_.x * other.g1_.y) + (self_.g3_.z * other.g0_.w)), (-(self_.g2_.y * other.g1_.y) - (self_.g2_.z * other.g1_.z))) + (vec4<f32>(self_.g0_.y) * other.g0_) - (vec4<f32>(self_.g3_.z, self_.g3_.x, self_.g3_.y, self_.g2_.x) * other.g1_.yzxx)), /* e41, e42, e43 */ (vec3<f32>(((other.g1_.y * self_.g4_.z) - (other.g1_.z * self_.g4_.y)), (-(other.g1_.x * self_.g4_.z) + (other.g1_.z * self_.g4_.x)), ((other.g1_.x * self_.g4_.y) - (other.g1_.y * self_.g4_.x))) - (vec3<f32>(other.g0_.w) * vec3<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z)) - (vec3<f32>(self_.g1_.w) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z))), /* e23, e31, e12 */ (vec3<f32>((-(other.g0_.y * self_.g4_.z) + (other.g0_.z * self_.g4_.y) + (other.g1_.y * self_.g1_.z) - (other.g1_.z * self_.g1_.y)), ((other.g0_.x * self_.g4_.z) - (other.g0_.z * self_.g4_.x) - (other.g1_.x * self_.g1_.z) + (other.g1_.z * self_.g1_.x)), (-(other.g0_.x * self_.g4_.y) + (other.g0_.y * self_.g4_.x) + (other.g1_.x * self_.g1_.y) - (other.g1_.y * self_.g1_.x))) - (vec3<f32>(other.g0_.w) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z)) + (vec3<f32>(other.g1_.w) * vec3<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z)) + (vec3<f32>(self_.g1_.w) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z)) - (vec3<f32>(self_.g4_.w) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z))), /* e423, e431, e412, e321 */ (vec4<f32>((self_.g2_.y * other.g1_.z), (self_.g2_.z * other.g1_.x), (self_.g2_.x * other.g1_.y), (-(self_.g2_.x * other.g0_.x) - (self_.g2_.y * other.g0_.y) - (self_.g2_.z * other.g0_.z) - (self_.g3_.y * other.g1_.y) - (self_.g3_.z * other.g1_.z))) + (vec4<f32>(self_.g0_.y) * other.g1_) + (vec4<f32>(other.g0_.w) * vec4<f32>(self_.g2_.x, self_.g2_.y, self_.g2_.z, self_.g0_.x)) - (vec4<f32>(self_.g2_.z, self_.g2_.x, self_.g2_.y, self_.g3_.x) * other.g1_.yzxx)));
}
fn multiVector_geometricAntiProduct_horizon(self_: MultiVector, other: Horizon) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((self_.g1_.w * other.g0_), 0.0), /* e1, e2, e3, e4 */ vec4<f32>((self_.g2_.x * other.g0_), (self_.g2_.y * other.g0_), (self_.g2_.z * other.g0_), 0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ (vec3<f32>(other.g0_) * vec3<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z)), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.y * other.g0_)));
}
fn multiVector_geometricAntiProduct_line(self_: MultiVector, other: Line) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (vec2<f32>((-(other.g1_.x * self_.g2_.x) - (other.g1_.y * self_.g2_.y) - (other.g1_.z * self_.g2_.z)), 0.0) - (vec2<f32>(other.g0_.x) * vec2<f32>(self_.g3_.x, self_.g2_.x)) - (vec2<f32>(other.g0_.y) * vec2<f32>(self_.g3_.y, self_.g2_.y)) - (vec2<f32>(other.g0_.z) * vec2<f32>(self_.g3_.z, self_.g2_.z))), /* e1, e2, e3, e4 */ (vec4<f32>(((other.g0_.x * self_.g4_.w) - (other.g0_.y * self_.g1_.z) + (other.g0_.z * self_.g1_.y) - (other.g1_.x * self_.g1_.w) + (other.g1_.y * self_.g4_.z)), ((other.g0_.x * self_.g1_.z) + (other.g0_.y * self_.g4_.w) - (other.g0_.z * self_.g1_.x) - (other.g1_.y * self_.g1_.w) + (other.g1_.z * self_.g4_.x)), (-(other.g0_.x * self_.g1_.y) + (other.g0_.y * self_.g1_.x) + (other.g0_.z * self_.g4_.w) + (other.g1_.x * self_.g4_.y) - (other.g1_.z * self_.g1_.w)), (-(other.g0_.y * self_.g4_.y) - (other.g0_.z * self_.g4_.z))) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.x) * self_.g4_.yzxx)), /* e41, e42, e43 */ ((vec3<f32>(self_.g0_.y) * other.g0_) - (other.g0_.yzx * self_.g2_.zxy) + (other.g0_.zxy * self_.g2_.yzx)), /* e23, e31, e12 */ ((vec3<f32>(self_.g0_.x) * other.g0_) + (vec3<f32>(self_.g0_.y) * other.g1_) - (other.g0_.yzx * self_.g3_.zxy) + (other.g0_.zxy * self_.g3_.yzx) - (other.g1_.yzx * self_.g2_.zxy) + (other.g1_.zxy * self_.g2_.yzx)), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.x * self_.g1_.w) - (other.g0_.y * self_.g4_.z)), ((other.g0_.y * self_.g1_.w) - (other.g0_.z * self_.g4_.x)), (-(other.g0_.x * self_.g4_.y) + (other.g0_.z * self_.g1_.w)), (-(other.g0_.x * self_.g1_.x) - (other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) + (other.g1_.y * self_.g4_.y) + (other.g1_.z * self_.g4_.z))) + (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.y, other.g1_.x) * self_.g4_.yzxx)));
}
fn multiVector_geometricAntiProduct_motor(self_: MultiVector, other: Motor) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(((self_.g0_.y * other.g1_.w) - (self_.g3_.x * other.g0_.x) - (self_.g3_.y * other.g0_.y) - (self_.g3_.z * other.g0_.z)), 0.0) - (vec2<f32>(self_.g2_.x) * vec2<f32>(other.g1_.x, other.g0_.x)) - (vec2<f32>(self_.g2_.y) * vec2<f32>(other.g1_.y, other.g0_.y)) - (vec2<f32>(self_.g2_.z) * vec2<f32>(other.g1_.z, other.g0_.z)) + (vec2<f32>(other.g0_.w) * self_.g0_)), /* e1, e2, e3, e4 */ (vec4<f32>(((other.g0_.z * self_.g1_.y) + (other.g0_.w * self_.g1_.x) - (other.g1_.x * self_.g1_.w) + (other.g1_.y * self_.g4_.z)), ((other.g0_.y * self_.g4_.w) + (other.g0_.w * self_.g1_.y) - (other.g1_.y * self_.g1_.w) + (other.g1_.z * self_.g4_.x)), ((other.g0_.z * self_.g4_.w) + (other.g0_.w * self_.g1_.z) + (other.g1_.x * self_.g4_.y) - (other.g1_.z * self_.g1_.w)), 0.0) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.y) * self_.g4_.yzxy) - (vec4<f32>(other.g1_.w, other.g1_.w, other.g1_.w, other.g0_.z) * self_.g4_.xyzz) - (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g4_.x) * other.g0_.yzxx) + (vec4<f32>(self_.g4_.w, self_.g1_.z, self_.g1_.x, self_.g1_.w) * other.g0_.xxyw)), /* e41, e42, e43 */ ((vec3<f32>(self_.g0_.y) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z)) - (vec3<f32>(other.g0_.y, other.g0_.z, other.g0_.x) * self_.g2_.zxy) + (vec3<f32>(other.g0_.z, other.g0_.x, other.g0_.w) * self_.g2_.yzz) + (vec3<f32>(other.g0_.w, other.g0_.w, other.g0_.y) * self_.g2_.xyx)), /* e23, e31, e12 */ ((vec3<f32>(self_.g0_.x) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z)) + (vec3<f32>(self_.g0_.y) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z)) - (vec3<f32>(other.g0_.y, other.g0_.z, other.g0_.x) * self_.g3_.zxy) + (vec3<f32>(other.g0_.z, other.g0_.x, other.g0_.w) * self_.g3_.yzz) + (vec3<f32>(other.g0_.w, other.g0_.w, other.g0_.y) * self_.g3_.xyx) - (vec3<f32>(other.g1_.y, other.g1_.z, other.g1_.x) * self_.g2_.zxy) + (vec3<f32>(other.g1_.z, other.g1_.x, other.g1_.w) * self_.g2_.yzz) + (vec3<f32>(other.g1_.w, other.g1_.w, other.g1_.y) * self_.g2_.xyx)), /* e423, e431, e412, e321 */ (vec4<f32>((other.g0_.w * self_.g4_.x), (other.g0_.y * self_.g1_.w), (other.g0_.z * self_.g1_.w), (-(other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) + (other.g1_.y * self_.g4_.y) + (other.g1_.z * self_.g4_.z) - (other.g1_.w * self_.g1_.w))) + (vec4<f32>(other.g0_.z, other.g0_.w, other.g0_.w, other.g1_.x) * self_.g4_.yyzx) + (vec4<f32>(self_.g1_.w, self_.g4_.z, self_.g4_.x, self_.g4_.w) * other.g0_.xxyw) - (vec4<f32>(self_.g4_.z, self_.g4_.x, self_.g4_.y, self_.g1_.x) * other.g0_.yzxx)));
}
fn multiVector_geometricAntiProduct_multiVector(self_: MultiVector, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(((other.g0_.y * self_.g0_.x) - (other.g3_.x * self_.g2_.x) - (other.g3_.y * self_.g2_.y) - (other.g3_.z * self_.g2_.z) - (other.g1_.x * self_.g4_.x) - (other.g1_.y * self_.g4_.y) - (other.g1_.z * self_.g4_.z) + (other.g4_.w * self_.g1_.w)), 0.0) + (vec2<f32>(self_.g0_.y) * other.g0_) - (vec2<f32>(other.g2_.x) * vec2<f32>(self_.g3_.x, self_.g2_.x)) - (vec2<f32>(other.g2_.y) * vec2<f32>(self_.g3_.y, self_.g2_.y)) - (vec2<f32>(other.g2_.z) * vec2<f32>(self_.g3_.z, self_.g2_.z)) - (vec2<f32>(other.g1_.w) * vec2<f32>(self_.g4_.w, self_.g1_.w)) + (vec2<f32>(other.g4_.x) * vec2<f32>(self_.g1_.x, self_.g4_.x)) + (vec2<f32>(other.g4_.y) * vec2<f32>(self_.g1_.y, self_.g4_.y)) + (vec2<f32>(other.g4_.z) * vec2<f32>(self_.g1_.z, self_.g4_.z))), /* e1, e2, e3, e4 */ (vec4<f32>(((self_.g0_.x * other.g4_.x) + (other.g2_.x * self_.g4_.w) - (other.g2_.y * self_.g1_.z) + (other.g2_.z * self_.g1_.y) - (other.g3_.x * self_.g1_.w) + (other.g3_.y * self_.g4_.z) + (self_.g2_.x * other.g4_.w) + (self_.g2_.y * other.g1_.z) - (self_.g2_.z * other.g1_.y) + (self_.g3_.x * other.g1_.w) + (self_.g3_.y * other.g4_.z)), ((self_.g0_.x * other.g4_.y) + (other.g2_.x * self_.g1_.z) + (other.g2_.y * self_.g4_.w) - (other.g2_.z * self_.g1_.x) - (other.g3_.y * self_.g1_.w) + (other.g3_.z * self_.g4_.x) - (self_.g2_.x * other.g1_.z) + (self_.g2_.y * other.g4_.w) + (self_.g2_.z * other.g1_.x) + (self_.g3_.y * other.g1_.w) + (self_.g3_.z * other.g4_.x)), ((self_.g0_.x * other.g4_.z) - (other.g2_.x * self_.g1_.y) + (other.g2_.y * self_.g1_.x) + (other.g2_.z * self_.g4_.w) + (other.g3_.x * self_.g4_.y) - (other.g3_.z * self_.g1_.w) + (self_.g2_.x * other.g1_.y) - (self_.g2_.y * other.g1_.x) + (self_.g2_.z * other.g4_.w) + (self_.g3_.x * other.g4_.y) + (self_.g3_.z * other.g1_.w)), (-(other.g2_.z * self_.g4_.z) - (self_.g2_.y * other.g4_.y) - (self_.g2_.z * other.g4_.z))) + (vec4<f32>(other.g0_.y) * self_.g1_) + (vec4<f32>(self_.g0_.y) * other.g1_) - (vec4<f32>(other.g0_.x, other.g0_.x, other.g0_.x, other.g2_.x) * self_.g4_.xyzx) - (vec4<f32>(other.g3_.z, other.g3_.x, other.g3_.y, other.g2_.y) * self_.g4_.yzxy) - (vec4<f32>(self_.g3_.z, self_.g3_.x, self_.g3_.y, self_.g2_.x) * other.g4_.yzxx)), /* e41, e42, e43 */ (vec3<f32>(((other.g4_.y * self_.g4_.z) - (other.g4_.z * self_.g4_.y)), (-(other.g4_.x * self_.g4_.z) + (other.g4_.z * self_.g4_.x)), ((other.g4_.x * self_.g4_.y) - (other.g4_.y * self_.g4_.x))) + (vec3<f32>(other.g0_.y) * self_.g2_) + (vec3<f32>(self_.g0_.y) * other.g2_) - (vec3<f32>(other.g1_.w) * vec3<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z)) - (vec3<f32>(self_.g1_.w) * vec3<f32>(other.g4_.x, other.g4_.y, other.g4_.z)) - (other.g2_.yzx * self_.g2_.zxy) + (other.g2_.zxy * self_.g2_.yzx)), /* e23, e31, e12 */ (vec3<f32>((-(other.g1_.y * self_.g4_.z) + (other.g1_.z * self_.g4_.y) + (other.g4_.y * self_.g1_.z) - (other.g4_.z * self_.g1_.y)), ((other.g1_.x * self_.g4_.z) - (other.g1_.z * self_.g4_.x) - (other.g4_.x * self_.g1_.z) + (other.g4_.z * self_.g1_.x)), (-(other.g1_.x * self_.g4_.y) + (other.g1_.y * self_.g4_.x) + (other.g4_.x * self_.g1_.y) - (other.g4_.y * self_.g1_.x))) + (vec3<f32>(other.g0_.x) * self_.g2_) + (vec3<f32>(other.g0_.y) * self_.g3_) + (vec3<f32>(self_.g0_.x) * other.g2_) + (vec3<f32>(self_.g0_.y) * other.g3_) - (vec3<f32>(other.g1_.w) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z)) + (vec3<f32>(other.g4_.w) * vec3<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z)) + (vec3<f32>(self_.g1_.w) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z)) - (vec3<f32>(self_.g4_.w) * vec3<f32>(other.g4_.x, other.g4_.y, other.g4_.z)) - (other.g2_.yzx * self_.g3_.zxy) + (other.g2_.zxy * self_.g3_.yzx) - (other.g3_.yzx * self_.g2_.zxy) + (other.g3_.zxy * self_.g2_.yzx)), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g2_.x * self_.g1_.w) - (other.g2_.y * self_.g4_.z) + (self_.g2_.y * other.g4_.z)), ((other.g2_.y * self_.g1_.w) - (other.g2_.z * self_.g4_.x) + (self_.g2_.z * other.g4_.x)), (-(other.g2_.x * self_.g4_.y) + (other.g2_.z * self_.g1_.w) + (self_.g2_.x * other.g4_.y)), (-(other.g0_.x * self_.g1_.w) - (other.g2_.x * self_.g1_.x) - (other.g2_.y * self_.g1_.y) - (other.g2_.z * self_.g1_.z) + (other.g3_.y * self_.g4_.y) + (other.g3_.z * self_.g4_.z) - (self_.g2_.x * other.g1_.x) - (self_.g2_.y * other.g1_.y) - (self_.g2_.z * other.g1_.z) - (self_.g3_.y * other.g4_.y) - (self_.g3_.z * other.g4_.z))) + (vec4<f32>(other.g0_.y) * self_.g4_) + (vec4<f32>(self_.g0_.y) * other.g4_) + (vec4<f32>(other.g1_.w) * vec4<f32>(self_.g2_.x, self_.g2_.y, self_.g2_.z, self_.g0_.x)) + (vec4<f32>(other.g2_.z, other.g2_.x, other.g2_.y, other.g3_.x) * self_.g4_.yzxx) - (vec4<f32>(self_.g2_.z, self_.g2_.x, self_.g2_.y, self_.g3_.x) * other.g4_.yzxx)));
}
fn multiVector_geometricAntiProduct_origin(self_: MultiVector, other: Origin) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(other.g0_) * vec2<f32>(self_.g4_.w, self_.g1_.w) * vec2<f32>(-1.0)), /* e1, e2, e3, e4 */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g3_.x, self_.g3_.y, self_.g3_.z, self_.g0_.y)), /* e41, e42, e43 */ (vec3<f32>(other.g0_) * vec3<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z) * vec3<f32>(-1.0)), /* e23, e31, e12 */ (vec3<f32>(other.g0_) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z) * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g2_.x, self_.g2_.y, self_.g2_.z, self_.g0_.x)));
}
fn multiVector_geometricAntiProduct_plane(self_: MultiVector, other: Plane) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (vec2<f32>((self_.g1_.w * other.g0_.w), 0.0) + (vec2<f32>(other.g0_.x) * vec2<f32>(self_.g1_.x, self_.g4_.x)) + (vec2<f32>(other.g0_.y) * vec2<f32>(self_.g1_.y, self_.g4_.y)) + (vec2<f32>(other.g0_.z) * vec2<f32>(self_.g1_.z, self_.g4_.z))), /* e1, e2, e3, e4 */ (vec4<f32>(((self_.g0_.x * other.g0_.x) + (self_.g2_.x * other.g0_.w) + (self_.g3_.y * other.g0_.z)), ((self_.g0_.x * other.g0_.y) + (self_.g2_.y * other.g0_.w) + (self_.g3_.z * other.g0_.x)), ((self_.g0_.x * other.g0_.z) + (self_.g2_.z * other.g0_.w) + (self_.g3_.x * other.g0_.y)), (-(self_.g2_.y * other.g0_.y) - (self_.g2_.z * other.g0_.z))) - (vec4<f32>(self_.g3_.z, self_.g3_.x, self_.g3_.y, self_.g2_.x) * other.g0_.yzxx)), /* e41, e42, e43 */ (vec3<f32>((-(self_.g4_.y * other.g0_.z) + (self_.g4_.z * other.g0_.y)), ((self_.g4_.x * other.g0_.z) - (self_.g4_.z * other.g0_.x)), (-(self_.g4_.x * other.g0_.y) + (self_.g4_.y * other.g0_.x))) - (vec3<f32>(self_.g1_.w) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z))), /* e23, e31, e12 */ (vec3<f32>((-(self_.g1_.y * other.g0_.z) + (self_.g1_.z * other.g0_.y)), ((self_.g1_.x * other.g0_.z) - (self_.g1_.z * other.g0_.x)), (-(self_.g1_.x * other.g0_.y) + (self_.g1_.y * other.g0_.x))) - (vec3<f32>(self_.g4_.w) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z)) + (vec3<f32>(other.g0_.w) * vec3<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z))), /* e423, e431, e412, e321 */ (vec4<f32>((self_.g2_.y * other.g0_.z), (self_.g2_.z * other.g0_.x), (self_.g2_.x * other.g0_.y), (-(self_.g3_.y * other.g0_.y) - (self_.g3_.z * other.g0_.z))) + (vec4<f32>(self_.g0_.y) * other.g0_) - (vec4<f32>(self_.g2_.z, self_.g2_.x, self_.g2_.y, self_.g3_.x) * other.g0_.yzxx)));
}
fn multiVector_geometricAntiProduct_point(self_: MultiVector, other: Point) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((-(self_.g4_.x * other.g0_.x) - (self_.g4_.y * other.g0_.y) - (self_.g4_.z * other.g0_.z) - (self_.g4_.w * other.g0_.w)), (self_.g1_.w * other.g0_.w * -1.0)), /* e1, e2, e3, e4 */ vec4<f32>(((self_.g0_.y * other.g0_.x) + (self_.g2_.y * other.g0_.z) - (self_.g2_.z * other.g0_.y) + (self_.g3_.x * other.g0_.w)), ((self_.g0_.y * other.g0_.y) - (self_.g2_.x * other.g0_.z) + (self_.g2_.z * other.g0_.x) + (self_.g3_.y * other.g0_.w)), ((self_.g0_.y * other.g0_.z) + (self_.g2_.x * other.g0_.y) - (self_.g2_.y * other.g0_.x) + (self_.g3_.z * other.g0_.w)), (self_.g0_.y * other.g0_.w)), /* e41, e42, e43 */ (vec3<f32>(other.g0_.w) * vec3<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z) * vec3<f32>(-1.0)), /* e23, e31, e12 */ (vec3<f32>(((self_.g4_.y * other.g0_.z) - (self_.g4_.z * other.g0_.y)), (-(self_.g4_.x * other.g0_.z) + (self_.g4_.z * other.g0_.x)), ((self_.g4_.x * other.g0_.y) - (self_.g4_.y * other.g0_.x))) + (vec3<f32>(self_.g1_.w) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z)) - (vec3<f32>(other.g0_.w) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z))), /* e423, e431, e412, e321 */ vec4<f32>((self_.g2_.x * other.g0_.w), (self_.g2_.y * other.g0_.w), (self_.g2_.z * other.g0_.w), ((self_.g0_.x * other.g0_.w) - (self_.g2_.x * other.g0_.x) - (self_.g2_.y * other.g0_.y) - (self_.g2_.z * other.g0_.z))));
}
fn multiVector_geometricAntiProduct_scalar(self_: MultiVector, other: Scalar) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((self_.g0_.y * other.g0_), 0.0), /* e1, e2, e3, e4 */ vec4<f32>((self_.g4_.x * other.g0_ * -1.0), (self_.g4_.y * other.g0_ * -1.0), (self_.g4_.z * other.g0_ * -1.0), 0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ (vec3<f32>(other.g0_) * self_.g2_), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g1_.w * other.g0_ * -1.0)));
}
fn origin_geometricAntiProduct_antiScalar(self_: Origin, other: AntiScalar) -> Origin {
    return Origin(/* e4 */ (other.g0_ * self_.g0_));
}
fn origin_geometricAntiProduct_dualNum(self_: Origin, other: DualNum) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.y * self_.g0_)), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.x * self_.g0_ * -1.0)));
}
fn origin_geometricAntiProduct_flector(self_: Origin, other: Flector) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, other.g0_.w) * vec4<f32>(-1.0)), /* e23, e31, e12, scalar */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.w)));
}
fn origin_geometricAntiProduct_horizon(self_: Origin, other: Horizon) -> Scalar {
    return Scalar(/* scalar */ (other.g0_ * self_.g0_));
}
fn origin_geometricAntiProduct_line(self_: Origin, other: Line) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g1_.x * self_.g0_ * -1.0), (other.g1_.y * self_.g0_ * -1.0), (other.g1_.z * self_.g0_ * -1.0), 0.0), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), 0.0));
}
fn origin_geometricAntiProduct_motor(self_: Origin, other: Motor) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, other.g0_.w) * vec4<f32>(-1.0, -1.0, -1.0, 1.0)), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.w) * vec4<f32>(1.0, 1.0, 1.0, -1.0)));
}
fn origin_geometricAntiProduct_multiVector(self_: Origin, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(self_.g0_) * vec2<f32>(other.g4_.w, other.g1_.w) * vec2<f32>(1.0, -1.0)), /* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g3_.x, other.g3_.y, other.g3_.z, other.g0_.y) * vec4<f32>(-1.0, -1.0, -1.0, 1.0)), /* e41, e42, e43 */ (vec3<f32>(self_.g0_) * vec3<f32>(other.g4_.x, other.g4_.y, other.g4_.z) * vec3<f32>(-1.0)), /* e23, e31, e12 */ (vec3<f32>(self_.g0_) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z)), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g2_.x, other.g2_.y, other.g2_.z, other.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, -1.0)));
}
fn origin_geometricAntiProduct_origin(self_: Origin, other: Origin) -> AntiScalar {
    return AntiScalar(/* e1234 */ (other.g0_ * self_.g0_ * -1.0));
}
fn origin_geometricAntiProduct_plane(self_: Origin, other: Plane) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g0_.x * self_.g0_ * -1.0), (other.g0_.y * self_.g0_ * -1.0), (other.g0_.z * self_.g0_ * -1.0), 0.0), /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.w * self_.g0_)));
}
fn origin_geometricAntiProduct_point(self_: Origin, other: Point) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.w * self_.g0_ * -1.0)), /* e23, e31, e12, scalar */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), 0.0));
}
fn origin_geometricAntiProduct_scalar(self_: Origin, other: Scalar) -> Horizon {
    return Horizon(/* e321 */ (self_.g0_ * other.g0_ * -1.0));
}
fn plane_geometricAntiProduct_antiScalar(self_: Plane, other: AntiScalar) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ (vec4<f32>(other.g0_) * self_.g0_));
}
fn plane_geometricAntiProduct_dualNum(self_: Plane, other: DualNum) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g0_.x * -1.0), (other.g0_.x * self_.g0_.y * -1.0), (other.g0_.x * self_.g0_.z * -1.0), 0.0), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_.y) * self_.g0_));
}
fn plane_geometricAntiProduct_flector(self_: Plane, other: Flector) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>((-(other.g0_.w * self_.g0_.x) - (other.g1_.z * self_.g0_.y)), (-(other.g0_.w * self_.g0_.y) - (other.g1_.x * self_.g0_.z)), (-(other.g0_.w * self_.g0_.z) - (other.g1_.y * self_.g0_.x)), ((other.g1_.y * self_.g0_.y) + (other.g1_.z * self_.g0_.z))) + (other.g1_.yzxx * self_.g0_.zxyx)), /* e23, e31, e12, scalar */ (vec4<f32>(((other.g0_.z * self_.g0_.y) + (other.g1_.w * self_.g0_.x)), ((other.g0_.x * self_.g0_.z) + (other.g1_.w * self_.g0_.y)), ((other.g0_.y * self_.g0_.x) + (other.g1_.w * self_.g0_.z)), (-(other.g0_.z * self_.g0_.z) - (other.g0_.w * self_.g0_.w))) - (vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, other.g0_.y) * self_.g0_.wwwy) - (other.g0_.yzxx * self_.g0_.zxyx)));
}
fn plane_geometricAntiProduct_horizon(self_: Plane, other: Horizon) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ (vec3<f32>(other.g0_) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z)));
}
fn plane_geometricAntiProduct_line(self_: Plane, other: Line) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(((other.g0_.x * self_.g0_.w) + (other.g1_.y * self_.g0_.z)), ((other.g0_.y * self_.g0_.w) + (other.g1_.z * self_.g0_.x)), ((other.g0_.z * self_.g0_.w) + (other.g1_.x * self_.g0_.y)), (-(other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z))) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.x) * self_.g0_.yzxx)), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.y * self_.g0_.z) * -1.0), ((other.g0_.z * self_.g0_.x) * -1.0), ((other.g0_.x * self_.g0_.y) * -1.0), ((other.g1_.y * self_.g0_.y) + (other.g1_.z * self_.g0_.z))) + (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.y, other.g1_.x) * self_.g0_.yzxx)));
}
fn plane_geometricAntiProduct_motor(self_: Plane, other: Motor) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(((other.g0_.x * self_.g0_.w) + (other.g1_.y * self_.g0_.z)), ((other.g0_.y * self_.g0_.w) + (other.g1_.z * self_.g0_.x)), ((other.g0_.z * self_.g0_.w) + (other.g1_.x * self_.g0_.y)), ((other.g0_.z * self_.g0_.z) * -1.0)) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.x) * self_.g0_.yzxx) - (vec4<f32>(other.g1_.w, other.g1_.w, other.g1_.w, other.g0_.y) * self_.g0_.xyzy)), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.y * self_.g0_.z) * -1.0), ((other.g0_.z * self_.g0_.x) * -1.0), ((other.g0_.x * self_.g0_.y) * -1.0), ((other.g1_.y * self_.g0_.y) + (other.g1_.z * self_.g0_.z))) + (vec4<f32>(other.g0_.w, other.g0_.w, other.g0_.w, other.g1_.x) * self_.g0_.xyzx) + (other.g0_.zxyw * self_.g0_.yzxw)));
}
fn plane_geometricAntiProduct_multiVector(self_: Plane, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((-(other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z) - (other.g1_.w * self_.g0_.w)), ((other.g4_.x * self_.g0_.x) + (other.g4_.y * self_.g0_.y) + (other.g4_.z * self_.g0_.z))), /* e1, e2, e3, e4 */ (vec4<f32>(((other.g2_.x * self_.g0_.w) + (other.g3_.y * self_.g0_.z)), ((other.g2_.y * self_.g0_.w) + (other.g3_.z * self_.g0_.x)), ((other.g2_.z * self_.g0_.w) + (other.g3_.x * self_.g0_.y)), ((other.g2_.z * self_.g0_.z) * -1.0)) - (vec4<f32>(other.g0_.x, other.g0_.x, other.g0_.x, other.g2_.x) * self_.g0_.xyzx) - (vec4<f32>(other.g3_.z, other.g3_.x, other.g3_.y, other.g2_.y) * self_.g0_.yzxy)), /* e41, e42, e43 */ (vec3<f32>(((other.g4_.y * self_.g0_.z) - (other.g4_.z * self_.g0_.y)), (-(other.g4_.x * self_.g0_.z) + (other.g4_.z * self_.g0_.x)), ((other.g4_.x * self_.g0_.y) - (other.g4_.y * self_.g0_.x))) - (vec3<f32>(other.g1_.w) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z))), /* e23, e31, e12 */ (vec3<f32>((-(other.g1_.y * self_.g0_.z) + (other.g1_.z * self_.g0_.y)), ((other.g1_.x * self_.g0_.z) - (other.g1_.z * self_.g0_.x)), (-(other.g1_.x * self_.g0_.y) + (other.g1_.y * self_.g0_.x))) + (vec3<f32>(other.g4_.w) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z)) - (vec3<f32>(self_.g0_.w) * vec3<f32>(other.g4_.x, other.g4_.y, other.g4_.z))), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g2_.y * self_.g0_.z) * -1.0), ((other.g2_.z * self_.g0_.x) * -1.0), ((other.g2_.x * self_.g0_.y) * -1.0), ((other.g3_.y * self_.g0_.y) + (other.g3_.z * self_.g0_.z))) + (vec4<f32>(other.g0_.y) * self_.g0_) + (vec4<f32>(other.g2_.z, other.g2_.x, other.g2_.y, other.g3_.x) * self_.g0_.yzxx)));
}
fn plane_geometricAntiProduct_origin(self_: Plane, other: Origin) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g0_.x * other.g0_ * -1.0), (self_.g0_.y * other.g0_ * -1.0), (self_.g0_.z * other.g0_ * -1.0), 0.0), /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.w * other.g0_ * -1.0)));
}
fn plane_geometricAntiProduct_plane(self_: Plane, other: Plane) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(((other.g0_.z * self_.g0_.y) * -1.0), ((other.g0_.x * self_.g0_.z) * -1.0), ((other.g0_.y * self_.g0_.x) * -1.0), ((other.g0_.y * self_.g0_.y) + (other.g0_.z * self_.g0_.z))) + (other.g0_.yzxx * self_.g0_.zxyx)), /* e23, e31, e12, scalar */ vec4<f32>((-(other.g0_.x * self_.g0_.w) + (other.g0_.w * self_.g0_.x)), (-(other.g0_.y * self_.g0_.w) + (other.g0_.w * self_.g0_.y)), (-(other.g0_.z * self_.g0_.w) + (other.g0_.w * self_.g0_.z)), 0.0));
}
fn plane_geometricAntiProduct_point(self_: Plane, other: Point) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g0_.x * other.g0_.w * -1.0), (self_.g0_.y * other.g0_.w * -1.0), (self_.g0_.z * other.g0_.w * -1.0), 0.0), /* e23, e31, e12, scalar */ (vec4<f32>((self_.g0_.y * other.g0_.z), (self_.g0_.z * other.g0_.x), (self_.g0_.x * other.g0_.y), (-(self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z) - (self_.g0_.w * other.g0_.w))) - (self_.g0_.zxyx * other.g0_.yzxx)));
}
fn plane_geometricAntiProduct_scalar(self_: Plane, other: Scalar) -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g0_ * -1.0), (self_.g0_.y * other.g0_ * -1.0), (self_.g0_.z * other.g0_ * -1.0), 0.0));
}
fn point_geometricAntiProduct_antiScalar(self_: Point, other: AntiScalar) -> Point {
    return Point(/* e1, e2, e3, e4 */ (vec4<f32>(other.g0_) * self_.g0_));
}
fn point_geometricAntiProduct_dualNum(self_: Point, other: DualNum) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(other.g0_.y) * self_.g0_), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.x * self_.g0_.w * -1.0)));
}
fn point_geometricAntiProduct_flector(self_: Point, other: Flector) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(self_.g0_.w) * vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, other.g0_.w) * vec4<f32>(-1.0)), /* e23, e31, e12, scalar */ (vec4<f32>((-(other.g0_.w * self_.g0_.x) - (other.g1_.z * self_.g0_.y)), (-(other.g0_.w * self_.g0_.y) - (other.g1_.x * self_.g0_.z)), (-(other.g0_.w * self_.g0_.z) - (other.g1_.y * self_.g0_.x)), ((other.g1_.z * self_.g0_.z) + (other.g1_.w * self_.g0_.w))) + (vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.x) * self_.g0_.wwwx) + (other.g1_.yzxy * self_.g0_.zxyy)));
}
fn point_geometricAntiProduct_horizon(self_: Point, other: Horizon) -> Scalar {
    return Scalar(/* scalar */ (self_.g0_.w * other.g0_));
}
fn point_geometricAntiProduct_line(self_: Point, other: Line) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((-(other.g0_.y * self_.g0_.z) + (other.g0_.z * self_.g0_.y) - (other.g1_.x * self_.g0_.w)), ((other.g0_.x * self_.g0_.z) - (other.g0_.z * self_.g0_.x) - (other.g1_.y * self_.g0_.w)), (-(other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x) - (other.g1_.z * self_.g0_.w)), 0.0), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.x * self_.g0_.w), (other.g0_.y * self_.g0_.w), (other.g0_.z * self_.g0_.w), (-(other.g0_.x * self_.g0_.x) - (other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z))));
}
fn point_geometricAntiProduct_motor(self_: Point, other: Motor) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((-(other.g0_.y * self_.g0_.z) + (other.g0_.z * self_.g0_.y) + (other.g0_.w * self_.g0_.x) - (other.g1_.x * self_.g0_.w)), ((other.g0_.x * self_.g0_.z) - (other.g0_.z * self_.g0_.x) + (other.g0_.w * self_.g0_.y) - (other.g1_.y * self_.g0_.w)), (-(other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x) + (other.g0_.w * self_.g0_.z) - (other.g1_.z * self_.g0_.w)), (other.g0_.w * self_.g0_.w)), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.x * self_.g0_.w), (other.g0_.y * self_.g0_.w), (other.g0_.z * self_.g0_.w), (-(other.g0_.x * self_.g0_.x) - (other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z) - (other.g1_.w * self_.g0_.w))));
}
fn point_geometricAntiProduct_multiVector(self_: Point, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(((other.g4_.x * self_.g0_.x) + (other.g4_.y * self_.g0_.y) + (other.g4_.z * self_.g0_.z) + (other.g4_.w * self_.g0_.w)), (other.g1_.w * self_.g0_.w * -1.0)), /* e1, e2, e3, e4 */ vec4<f32>(((other.g0_.y * self_.g0_.x) - (other.g2_.y * self_.g0_.z) + (other.g2_.z * self_.g0_.y) - (other.g3_.x * self_.g0_.w)), ((other.g0_.y * self_.g0_.y) + (other.g2_.x * self_.g0_.z) - (other.g2_.z * self_.g0_.x) - (other.g3_.y * self_.g0_.w)), ((other.g0_.y * self_.g0_.z) - (other.g2_.x * self_.g0_.y) + (other.g2_.y * self_.g0_.x) - (other.g3_.z * self_.g0_.w)), (other.g0_.y * self_.g0_.w)), /* e41, e42, e43 */ (vec3<f32>(self_.g0_.w) * vec3<f32>(other.g4_.x, other.g4_.y, other.g4_.z) * vec3<f32>(-1.0)), /* e23, e31, e12 */ (vec3<f32>(((other.g4_.y * self_.g0_.z) - (other.g4_.z * self_.g0_.y)), (-(other.g4_.x * self_.g0_.z) + (other.g4_.z * self_.g0_.x)), ((other.g4_.x * self_.g0_.y) - (other.g4_.y * self_.g0_.x))) - (vec3<f32>(other.g1_.w) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z)) + (vec3<f32>(self_.g0_.w) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z))), /* e423, e431, e412, e321 */ vec4<f32>((other.g2_.x * self_.g0_.w), (other.g2_.y * self_.g0_.w), (other.g2_.z * self_.g0_.w), (-(other.g0_.x * self_.g0_.w) - (other.g2_.x * self_.g0_.x) - (other.g2_.y * self_.g0_.y) - (other.g2_.z * self_.g0_.z))));
}
fn point_geometricAntiProduct_origin(self_: Point, other: Origin) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.w * other.g0_ * -1.0)), /* e23, e31, e12, scalar */ vec4<f32>((self_.g0_.x * other.g0_ * -1.0), (self_.g0_.y * other.g0_ * -1.0), (self_.g0_.z * other.g0_ * -1.0), 0.0));
}
fn point_geometricAntiProduct_plane(self_: Point, other: Plane) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g0_.x * self_.g0_.w * -1.0), (other.g0_.y * self_.g0_.w * -1.0), (other.g0_.z * self_.g0_.w * -1.0), 0.0), /* e23, e31, e12, scalar */ (vec4<f32>(((other.g0_.z * self_.g0_.y) * -1.0), ((other.g0_.x * self_.g0_.z) * -1.0), ((other.g0_.y * self_.g0_.x) * -1.0), ((other.g0_.y * self_.g0_.y) + (other.g0_.z * self_.g0_.z) + (other.g0_.w * self_.g0_.w))) + (other.g0_.yzxx * self_.g0_.zxyx)));
}
fn point_geometricAntiProduct_point(self_: Point, other: Point) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.w * self_.g0_.w * -1.0)), /* e23, e31, e12, scalar */ vec4<f32>(((other.g0_.x * self_.g0_.w) - (other.g0_.w * self_.g0_.x)), ((other.g0_.y * self_.g0_.w) - (other.g0_.w * self_.g0_.y)), ((other.g0_.z * self_.g0_.w) - (other.g0_.w * self_.g0_.z)), 0.0));
}
fn point_geometricAntiProduct_scalar(self_: Point, other: Scalar) -> Horizon {
    return Horizon(/* e321 */ (self_.g0_.w * other.g0_ * -1.0));
}
fn scalar_geometricAntiProduct_antiScalar(self_: Scalar, other: AntiScalar) -> Scalar {
    return Scalar(/* scalar */ (other.g0_ * self_.g0_));
}
fn scalar_geometricAntiProduct_dualNum(self_: Scalar, other: DualNum) -> Scalar {
    return Scalar(/* scalar */ (other.g0_.y * self_.g0_));
}
fn scalar_geometricAntiProduct_flector(self_: Scalar, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_), 0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.w * self_.g0_)));
}
fn scalar_geometricAntiProduct_line(self_: Scalar, other: Line) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ (vec3<f32>(self_.g0_) * other.g0_));
}
fn scalar_geometricAntiProduct_motor(self_: Scalar, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0), /* e23, e31, e12, scalar */ (vec4<f32>(self_.g0_) * other.g0_));
}
fn scalar_geometricAntiProduct_multiVector(self_: Scalar, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((other.g0_.y * self_.g0_), 0.0), /* e1, e2, e3, e4 */ vec4<f32>((other.g4_.x * self_.g0_), (other.g4_.y * self_.g0_), (other.g4_.z * self_.g0_), 0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ (vec3<f32>(self_.g0_) * other.g2_), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g1_.w * self_.g0_)));
}
fn scalar_geometricAntiProduct_origin(self_: Scalar, other: Origin) -> Horizon {
    return Horizon(/* e321 */ (other.g0_ * self_.g0_));
}
fn scalar_geometricAntiProduct_plane(self_: Scalar, other: Plane) -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), 0.0));
}
fn scalar_geometricAntiProduct_point(self_: Scalar, other: Point) -> Horizon {
    return Horizon(/* e321 */ (other.g0_.w * self_.g0_));
}
fn antiScalar_geometricAntiQuotient_antiScalar(self_: AntiScalar, other: AntiScalar) -> AntiScalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return AntiScalar(/* e1234 */ (anti_inverse.g0_ * self_.g0_));
}
fn antiScalar_geometricAntiQuotient_dualNum(self_: AntiScalar, other: DualNum) -> AntiScalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_.y, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return AntiScalar(/* e1234 */ (anti_inverse.g0_ * self_.g0_));
}
fn antiScalar_geometricAntiQuotient_flector(self_: AntiScalar, other: Flector) -> AntiScalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.w, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return AntiScalar(/* e1234 */ (anti_inverse.g0_ * self_.g0_));
}
fn antiScalar_geometricAntiQuotient_line(self_: AntiScalar, other: Line) -> AntiScalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return AntiScalar(/* e1234 */ (anti_inverse.g0_ * self_.g0_));
}
fn antiScalar_geometricAntiQuotient_motor(self_: AntiScalar, other: Motor) -> AntiScalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2) + pow(other.g0_.w, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return AntiScalar(/* e1234 */ (anti_inverse.g0_ * self_.g0_));
}
fn antiScalar_geometricAntiQuotient_multiVector(self_: AntiScalar, other: MultiVector) -> AntiScalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.y, 2) - pow(other.g2_.x, 2) - pow(other.g2_.y, 2) - pow(other.g2_.z, 2) - pow(other.g1_.w, 2) + pow(other.g4_.x, 2) + pow(other.g4_.y, 2) + pow(other.g4_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return AntiScalar(/* e1234 */ (anti_inverse.g0_ * self_.g0_));
}
fn antiScalar_geometricAntiQuotient_origin(self_: AntiScalar, other: Origin) -> AntiScalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return AntiScalar(/* e1234 */ (anti_inverse.g0_ * self_.g0_));
}
fn antiScalar_geometricAntiQuotient_plane(self_: AntiScalar, other: Plane) -> AntiScalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return AntiScalar(/* e1234 */ (anti_inverse.g0_ * self_.g0_));
}
fn antiScalar_geometricAntiQuotient_point(self_: AntiScalar, other: Point) -> AntiScalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.w, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return AntiScalar(/* e1234 */ (anti_inverse.g0_ * self_.g0_));
}
fn dualNum_geometricAntiQuotient_antiScalar(self_: DualNum, other: AntiScalar) -> DualNum {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return DualNum(/* scalar, e1234 */ (vec2<f32>(anti_inverse.g0_) * self_.g0_));
}
fn dualNum_geometricAntiQuotient_dualNum(self_: DualNum, other: DualNum) -> DualNum {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_.y, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return DualNum(/* scalar, e1234 */ (vec2<f32>(anti_inverse.g0_) * self_.g0_));
}
fn dualNum_geometricAntiQuotient_flector(self_: DualNum, other: Flector) -> DualNum {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.w, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return DualNum(/* scalar, e1234 */ (vec2<f32>(anti_inverse.g0_) * self_.g0_));
}
fn dualNum_geometricAntiQuotient_line(self_: DualNum, other: Line) -> DualNum {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return DualNum(/* scalar, e1234 */ (vec2<f32>(anti_inverse.g0_) * self_.g0_));
}
fn dualNum_geometricAntiQuotient_motor(self_: DualNum, other: Motor) -> DualNum {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2) + pow(other.g0_.w, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return DualNum(/* scalar, e1234 */ (vec2<f32>(anti_inverse.g0_) * self_.g0_));
}
fn dualNum_geometricAntiQuotient_multiVector(self_: DualNum, other: MultiVector) -> DualNum {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.y, 2) - pow(other.g2_.x, 2) - pow(other.g2_.y, 2) - pow(other.g2_.z, 2) - pow(other.g1_.w, 2) + pow(other.g4_.x, 2) + pow(other.g4_.y, 2) + pow(other.g4_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return DualNum(/* scalar, e1234 */ (vec2<f32>(anti_inverse.g0_) * self_.g0_));
}
fn dualNum_geometricAntiQuotient_origin(self_: DualNum, other: Origin) -> DualNum {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return DualNum(/* scalar, e1234 */ (vec2<f32>(anti_inverse.g0_) * self_.g0_));
}
fn dualNum_geometricAntiQuotient_plane(self_: DualNum, other: Plane) -> DualNum {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return DualNum(/* scalar, e1234 */ (vec2<f32>(anti_inverse.g0_) * self_.g0_));
}
fn dualNum_geometricAntiQuotient_point(self_: DualNum, other: Point) -> DualNum {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.w, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return DualNum(/* scalar, e1234 */ (vec2<f32>(anti_inverse.g0_) * self_.g0_));
}
fn flector_geometricAntiQuotient_antiScalar(self_: Flector, other: AntiScalar) -> Flector {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g1_));
}
fn flector_geometricAntiQuotient_dualNum(self_: Flector, other: DualNum) -> Flector {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_.y, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g1_));
}
fn flector_geometricAntiQuotient_flector(self_: Flector, other: Flector) -> Flector {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.w, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g1_));
}
fn flector_geometricAntiQuotient_line(self_: Flector, other: Line) -> Flector {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g1_));
}
fn flector_geometricAntiQuotient_motor(self_: Flector, other: Motor) -> Flector {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2) + pow(other.g0_.w, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g1_));
}
fn flector_geometricAntiQuotient_multiVector(self_: Flector, other: MultiVector) -> Flector {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.y, 2) - pow(other.g2_.x, 2) - pow(other.g2_.y, 2) - pow(other.g2_.z, 2) - pow(other.g1_.w, 2) + pow(other.g4_.x, 2) + pow(other.g4_.y, 2) + pow(other.g4_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g1_));
}
fn flector_geometricAntiQuotient_origin(self_: Flector, other: Origin) -> Flector {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g1_));
}
fn flector_geometricAntiQuotient_plane(self_: Flector, other: Plane) -> Flector {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g1_));
}
fn flector_geometricAntiQuotient_point(self_: Flector, other: Point) -> Flector {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.w, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g1_));
}
fn horizon_geometricAntiQuotient_antiScalar(self_: Horizon, other: AntiScalar) -> Horizon {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Horizon(/* e321 */ (anti_inverse.g0_ * self_.g0_));
}
fn horizon_geometricAntiQuotient_dualNum(self_: Horizon, other: DualNum) -> Horizon {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_.y, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Horizon(/* e321 */ (anti_inverse.g0_ * self_.g0_));
}
fn horizon_geometricAntiQuotient_flector(self_: Horizon, other: Flector) -> Horizon {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.w, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Horizon(/* e321 */ (anti_inverse.g0_ * self_.g0_));
}
fn horizon_geometricAntiQuotient_line(self_: Horizon, other: Line) -> Horizon {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Horizon(/* e321 */ (anti_inverse.g0_ * self_.g0_));
}
fn horizon_geometricAntiQuotient_motor(self_: Horizon, other: Motor) -> Horizon {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2) + pow(other.g0_.w, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Horizon(/* e321 */ (anti_inverse.g0_ * self_.g0_));
}
fn horizon_geometricAntiQuotient_multiVector(self_: Horizon, other: MultiVector) -> Horizon {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.y, 2) - pow(other.g2_.x, 2) - pow(other.g2_.y, 2) - pow(other.g2_.z, 2) - pow(other.g1_.w, 2) + pow(other.g4_.x, 2) + pow(other.g4_.y, 2) + pow(other.g4_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Horizon(/* e321 */ (anti_inverse.g0_ * self_.g0_));
}
fn horizon_geometricAntiQuotient_origin(self_: Horizon, other: Origin) -> Horizon {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Horizon(/* e321 */ (anti_inverse.g0_ * self_.g0_));
}
fn horizon_geometricAntiQuotient_plane(self_: Horizon, other: Plane) -> Horizon {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Horizon(/* e321 */ (anti_inverse.g0_ * self_.g0_));
}
fn horizon_geometricAntiQuotient_point(self_: Horizon, other: Point) -> Horizon {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.w, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Horizon(/* e321 */ (anti_inverse.g0_ * self_.g0_));
}
fn line_geometricAntiQuotient_antiScalar(self_: Line, other: AntiScalar) -> Line {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Line(/* e41, e42, e43 */ (vec3<f32>(anti_inverse.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(anti_inverse.g0_) * self_.g1_));
}
fn line_geometricAntiQuotient_dualNum(self_: Line, other: DualNum) -> Line {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_.y, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Line(/* e41, e42, e43 */ (vec3<f32>(anti_inverse.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(anti_inverse.g0_) * self_.g1_));
}
fn line_geometricAntiQuotient_flector(self_: Line, other: Flector) -> Line {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.w, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Line(/* e41, e42, e43 */ (vec3<f32>(anti_inverse.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(anti_inverse.g0_) * self_.g1_));
}
fn line_geometricAntiQuotient_line(self_: Line, other: Line) -> Line {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Line(/* e41, e42, e43 */ (vec3<f32>(anti_inverse.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(anti_inverse.g0_) * self_.g1_));
}
fn line_geometricAntiQuotient_motor(self_: Line, other: Motor) -> Line {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2) + pow(other.g0_.w, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Line(/* e41, e42, e43 */ (vec3<f32>(anti_inverse.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(anti_inverse.g0_) * self_.g1_));
}
fn line_geometricAntiQuotient_multiVector(self_: Line, other: MultiVector) -> Line {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.y, 2) - pow(other.g2_.x, 2) - pow(other.g2_.y, 2) - pow(other.g2_.z, 2) - pow(other.g1_.w, 2) + pow(other.g4_.x, 2) + pow(other.g4_.y, 2) + pow(other.g4_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Line(/* e41, e42, e43 */ (vec3<f32>(anti_inverse.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(anti_inverse.g0_) * self_.g1_));
}
fn line_geometricAntiQuotient_origin(self_: Line, other: Origin) -> Line {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Line(/* e41, e42, e43 */ (vec3<f32>(anti_inverse.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(anti_inverse.g0_) * self_.g1_));
}
fn line_geometricAntiQuotient_plane(self_: Line, other: Plane) -> Line {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Line(/* e41, e42, e43 */ (vec3<f32>(anti_inverse.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(anti_inverse.g0_) * self_.g1_));
}
fn line_geometricAntiQuotient_point(self_: Line, other: Point) -> Line {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.w, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Line(/* e41, e42, e43 */ (vec3<f32>(anti_inverse.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(anti_inverse.g0_) * self_.g1_));
}
fn motor_geometricAntiQuotient_antiScalar(self_: Motor, other: AntiScalar) -> Motor {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(anti_inverse.g0_) * self_.g1_));
}
fn motor_geometricAntiQuotient_dualNum(self_: Motor, other: DualNum) -> Motor {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_.y, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(anti_inverse.g0_) * self_.g1_));
}
fn motor_geometricAntiQuotient_flector(self_: Motor, other: Flector) -> Motor {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.w, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(anti_inverse.g0_) * self_.g1_));
}
fn motor_geometricAntiQuotient_line(self_: Motor, other: Line) -> Motor {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(anti_inverse.g0_) * self_.g1_));
}
fn motor_geometricAntiQuotient_motor(self_: Motor, other: Motor) -> Motor {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2) + pow(other.g0_.w, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(anti_inverse.g0_) * self_.g1_));
}
fn motor_geometricAntiQuotient_multiVector(self_: Motor, other: MultiVector) -> Motor {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.y, 2) - pow(other.g2_.x, 2) - pow(other.g2_.y, 2) - pow(other.g2_.z, 2) - pow(other.g1_.w, 2) + pow(other.g4_.x, 2) + pow(other.g4_.y, 2) + pow(other.g4_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(anti_inverse.g0_) * self_.g1_));
}
fn motor_geometricAntiQuotient_origin(self_: Motor, other: Origin) -> Motor {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(anti_inverse.g0_) * self_.g1_));
}
fn motor_geometricAntiQuotient_plane(self_: Motor, other: Plane) -> Motor {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(anti_inverse.g0_) * self_.g1_));
}
fn motor_geometricAntiQuotient_point(self_: Motor, other: Point) -> Motor {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.w, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(anti_inverse.g0_) * self_.g1_));
}
fn multiVector_geometricAntiQuotient_antiScalar(self_: MultiVector, other: AntiScalar) -> MultiVector {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(anti_inverse.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(anti_inverse.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(anti_inverse.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g4_));
}
fn multiVector_geometricAntiQuotient_dualNum(self_: MultiVector, other: DualNum) -> MultiVector {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_.y, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(anti_inverse.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(anti_inverse.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(anti_inverse.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g4_));
}
fn multiVector_geometricAntiQuotient_flector(self_: MultiVector, other: Flector) -> MultiVector {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.w, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(anti_inverse.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(anti_inverse.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(anti_inverse.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g4_));
}
fn multiVector_geometricAntiQuotient_line(self_: MultiVector, other: Line) -> MultiVector {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(anti_inverse.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(anti_inverse.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(anti_inverse.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g4_));
}
fn multiVector_geometricAntiQuotient_motor(self_: MultiVector, other: Motor) -> MultiVector {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2) + pow(other.g0_.w, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(anti_inverse.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(anti_inverse.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(anti_inverse.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g4_));
}
fn multiVector_geometricAntiQuotient_multiVector(self_: MultiVector, other: MultiVector) -> MultiVector {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.y, 2) - pow(other.g2_.x, 2) - pow(other.g2_.y, 2) - pow(other.g2_.z, 2) - pow(other.g1_.w, 2) + pow(other.g4_.x, 2) + pow(other.g4_.y, 2) + pow(other.g4_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(anti_inverse.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(anti_inverse.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(anti_inverse.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g4_));
}
fn multiVector_geometricAntiQuotient_origin(self_: MultiVector, other: Origin) -> MultiVector {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(anti_inverse.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(anti_inverse.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(anti_inverse.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g4_));
}
fn multiVector_geometricAntiQuotient_plane(self_: MultiVector, other: Plane) -> MultiVector {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(anti_inverse.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(anti_inverse.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(anti_inverse.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g4_));
}
fn multiVector_geometricAntiQuotient_point(self_: MultiVector, other: Point) -> MultiVector {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.w, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(anti_inverse.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(anti_inverse.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(anti_inverse.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g4_));
}
fn origin_geometricAntiQuotient_antiScalar(self_: Origin, other: AntiScalar) -> Origin {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Origin(/* e4 */ (anti_inverse.g0_ * self_.g0_));
}
fn origin_geometricAntiQuotient_dualNum(self_: Origin, other: DualNum) -> Origin {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_.y, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Origin(/* e4 */ (anti_inverse.g0_ * self_.g0_));
}
fn origin_geometricAntiQuotient_flector(self_: Origin, other: Flector) -> Origin {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.w, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Origin(/* e4 */ (anti_inverse.g0_ * self_.g0_));
}
fn origin_geometricAntiQuotient_line(self_: Origin, other: Line) -> Origin {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Origin(/* e4 */ (anti_inverse.g0_ * self_.g0_));
}
fn origin_geometricAntiQuotient_motor(self_: Origin, other: Motor) -> Origin {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2) + pow(other.g0_.w, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Origin(/* e4 */ (anti_inverse.g0_ * self_.g0_));
}
fn origin_geometricAntiQuotient_multiVector(self_: Origin, other: MultiVector) -> Origin {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.y, 2) - pow(other.g2_.x, 2) - pow(other.g2_.y, 2) - pow(other.g2_.z, 2) - pow(other.g1_.w, 2) + pow(other.g4_.x, 2) + pow(other.g4_.y, 2) + pow(other.g4_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Origin(/* e4 */ (anti_inverse.g0_ * self_.g0_));
}
fn origin_geometricAntiQuotient_origin(self_: Origin, other: Origin) -> Origin {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Origin(/* e4 */ (anti_inverse.g0_ * self_.g0_));
}
fn origin_geometricAntiQuotient_plane(self_: Origin, other: Plane) -> Origin {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Origin(/* e4 */ (anti_inverse.g0_ * self_.g0_));
}
fn origin_geometricAntiQuotient_point(self_: Origin, other: Point) -> Origin {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.w, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Origin(/* e4 */ (anti_inverse.g0_ * self_.g0_));
}
fn plane_geometricAntiQuotient_antiScalar(self_: Plane, other: AntiScalar) -> Plane {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Plane(/* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_));
}
fn plane_geometricAntiQuotient_dualNum(self_: Plane, other: DualNum) -> Plane {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_.y, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Plane(/* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_));
}
fn plane_geometricAntiQuotient_flector(self_: Plane, other: Flector) -> Plane {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.w, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Plane(/* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_));
}
fn plane_geometricAntiQuotient_line(self_: Plane, other: Line) -> Plane {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Plane(/* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_));
}
fn plane_geometricAntiQuotient_motor(self_: Plane, other: Motor) -> Plane {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2) + pow(other.g0_.w, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Plane(/* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_));
}
fn plane_geometricAntiQuotient_multiVector(self_: Plane, other: MultiVector) -> Plane {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.y, 2) - pow(other.g2_.x, 2) - pow(other.g2_.y, 2) - pow(other.g2_.z, 2) - pow(other.g1_.w, 2) + pow(other.g4_.x, 2) + pow(other.g4_.y, 2) + pow(other.g4_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Plane(/* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_));
}
fn plane_geometricAntiQuotient_origin(self_: Plane, other: Origin) -> Plane {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Plane(/* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_));
}
fn plane_geometricAntiQuotient_plane(self_: Plane, other: Plane) -> Plane {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Plane(/* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_));
}
fn plane_geometricAntiQuotient_point(self_: Plane, other: Point) -> Plane {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.w, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Plane(/* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_));
}
fn point_geometricAntiQuotient_antiScalar(self_: Point, other: AntiScalar) -> Point {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Point(/* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_));
}
fn point_geometricAntiQuotient_dualNum(self_: Point, other: DualNum) -> Point {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_.y, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Point(/* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_));
}
fn point_geometricAntiQuotient_flector(self_: Point, other: Flector) -> Point {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.w, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Point(/* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_));
}
fn point_geometricAntiQuotient_line(self_: Point, other: Line) -> Point {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Point(/* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_));
}
fn point_geometricAntiQuotient_motor(self_: Point, other: Motor) -> Point {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2) + pow(other.g0_.w, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Point(/* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_));
}
fn point_geometricAntiQuotient_multiVector(self_: Point, other: MultiVector) -> Point {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.y, 2) - pow(other.g2_.x, 2) - pow(other.g2_.y, 2) - pow(other.g2_.z, 2) - pow(other.g1_.w, 2) + pow(other.g4_.x, 2) + pow(other.g4_.y, 2) + pow(other.g4_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Point(/* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_));
}
fn point_geometricAntiQuotient_origin(self_: Point, other: Origin) -> Point {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Point(/* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_));
}
fn point_geometricAntiQuotient_plane(self_: Point, other: Plane) -> Point {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Point(/* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_));
}
fn point_geometricAntiQuotient_point(self_: Point, other: Point) -> Point {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.w, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Point(/* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_));
}
fn scalar_geometricAntiQuotient_antiScalar(self_: Scalar, other: AntiScalar) -> Scalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Scalar(/* scalar */ (anti_inverse.g0_ * self_.g0_));
}
fn scalar_geometricAntiQuotient_dualNum(self_: Scalar, other: DualNum) -> Scalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_.y, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Scalar(/* scalar */ (anti_inverse.g0_ * self_.g0_));
}
fn scalar_geometricAntiQuotient_flector(self_: Scalar, other: Flector) -> Scalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.w, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Scalar(/* scalar */ (anti_inverse.g0_ * self_.g0_));
}
fn scalar_geometricAntiQuotient_line(self_: Scalar, other: Line) -> Scalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Scalar(/* scalar */ (anti_inverse.g0_ * self_.g0_));
}
fn scalar_geometricAntiQuotient_motor(self_: Scalar, other: Motor) -> Scalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2) + pow(other.g0_.w, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Scalar(/* scalar */ (anti_inverse.g0_ * self_.g0_));
}
fn scalar_geometricAntiQuotient_multiVector(self_: Scalar, other: MultiVector) -> Scalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.y, 2) - pow(other.g2_.x, 2) - pow(other.g2_.y, 2) - pow(other.g2_.z, 2) - pow(other.g1_.w, 2) + pow(other.g4_.x, 2) + pow(other.g4_.y, 2) + pow(other.g4_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Scalar(/* scalar */ (anti_inverse.g0_ * self_.g0_));
}
fn scalar_geometricAntiQuotient_origin(self_: Scalar, other: Origin) -> Scalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Scalar(/* scalar */ (anti_inverse.g0_ * self_.g0_));
}
fn scalar_geometricAntiQuotient_plane(self_: Scalar, other: Plane) -> Scalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Scalar(/* scalar */ (anti_inverse.g0_ * self_.g0_));
}
fn scalar_geometricAntiQuotient_point(self_: Scalar, other: Point) -> Scalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.w, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    return Scalar(/* scalar */ (anti_inverse.g0_ * self_.g0_));
}
fn antiScalar_geometricProduct_dualNum(self_: AntiScalar, other: DualNum) -> AntiScalar {
    return AntiScalar(/* e1234 */ (other.g0_.x * self_.g0_));
}
fn antiScalar_geometricProduct_flector(self_: AntiScalar, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g1_.w * self_.g0_ * -1.0)), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.x * self_.g0_ * -1.0), (other.g0_.y * self_.g0_ * -1.0), (other.g0_.z * self_.g0_ * -1.0), 0.0));
}
fn antiScalar_geometricProduct_horizon(self_: AntiScalar, other: Horizon) -> Origin {
    return Origin(/* e4 */ (self_.g0_ * other.g0_ * -1.0));
}
fn antiScalar_geometricProduct_line(self_: AntiScalar, other: Line) -> Line {
    return Line(/* e41, e42, e43 */ (vec3<f32>(self_.g0_) * other.g1_), /* e23, e31, e12 */ vec3<f32>(0.0));
}
fn antiScalar_geometricProduct_motor(self_: AntiScalar, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(self_.g0_) * other.g1_), /* e23, e31, e12, scalar */ vec4<f32>(0.0));
}
fn antiScalar_geometricProduct_multiVector(self_: AntiScalar, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (other.g0_.x * self_.g0_)), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g4_.w * self_.g0_ * -1.0)), /* e41, e42, e43 */ (vec3<f32>(self_.g0_) * other.g3_), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>((other.g1_.x * self_.g0_ * -1.0), (other.g1_.y * self_.g0_ * -1.0), (other.g1_.z * self_.g0_ * -1.0), 0.0));
}
fn antiScalar_geometricProduct_plane(self_: AntiScalar, other: Plane) -> Origin {
    return Origin(/* e4 */ (other.g0_.w * self_.g0_ * -1.0));
}
fn antiScalar_geometricProduct_point(self_: AntiScalar, other: Point) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>((other.g0_.x * self_.g0_ * -1.0), (other.g0_.y * self_.g0_ * -1.0), (other.g0_.z * self_.g0_ * -1.0), 0.0));
}
fn antiScalar_geometricProduct_scalar(self_: AntiScalar, other: Scalar) -> AntiScalar {
    return AntiScalar(/* e1234 */ (self_.g0_ * other.g0_));
}
fn dualNum_geometricProduct_antiScalar(self_: DualNum, other: AntiScalar) -> AntiScalar {
    return AntiScalar(/* e1234 */ (self_.g0_.x * other.g0_));
}
fn dualNum_geometricProduct_dualNum(self_: DualNum, other: DualNum) -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>((other.g0_.x * self_.g0_.x), ((other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x))));
}
fn dualNum_geometricProduct_flector(self_: DualNum, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g0_.x), (self_.g0_.x * other.g0_.y), (self_.g0_.x * other.g0_.z), ((self_.g0_.x * other.g0_.w) - (self_.g0_.y * other.g1_.w))), /* e423, e431, e412, e321 */ vec4<f32>(((self_.g0_.x * other.g1_.x) - (self_.g0_.y * other.g0_.x)), ((self_.g0_.x * other.g1_.y) - (self_.g0_.y * other.g0_.y)), ((self_.g0_.x * other.g1_.z) - (self_.g0_.y * other.g0_.z)), (self_.g0_.x * other.g1_.w)));
}
fn dualNum_geometricProduct_horizon(self_: DualNum, other: Horizon) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.y * other.g0_ * -1.0)), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.x * other.g0_)));
}
fn dualNum_geometricProduct_line(self_: DualNum, other: Line) -> Line {
    return Line(/* e41, e42, e43 */ ((vec3<f32>(self_.g0_.x) * other.g0_) + (vec3<f32>(self_.g0_.y) * other.g1_)), /* e23, e31, e12 */ (vec3<f32>(self_.g0_.x) * other.g1_));
}
fn dualNum_geometricProduct_motor(self_: DualNum, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ ((vec4<f32>(self_.g0_.x) * other.g0_) + (vec4<f32>(self_.g0_.y) * other.g1_)), /* e23, e31, e12, scalar */ (vec4<f32>(self_.g0_.x) * other.g1_));
}
fn dualNum_geometricProduct_multiVector(self_: DualNum, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((self_.g0_.x * other.g0_.x), ((self_.g0_.x * other.g0_.y) + (self_.g0_.y * other.g0_.x))), /* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g1_.x), (self_.g0_.x * other.g1_.y), (self_.g0_.x * other.g1_.z), ((self_.g0_.x * other.g1_.w) - (self_.g0_.y * other.g4_.w))), /* e41, e42, e43 */ ((vec3<f32>(self_.g0_.x) * other.g2_) + (vec3<f32>(self_.g0_.y) * other.g3_)), /* e23, e31, e12 */ (vec3<f32>(self_.g0_.x) * other.g3_), /* e423, e431, e412, e321 */ vec4<f32>(((self_.g0_.x * other.g4_.x) - (self_.g0_.y * other.g1_.x)), ((self_.g0_.x * other.g4_.y) - (self_.g0_.y * other.g1_.y)), ((self_.g0_.x * other.g4_.z) - (self_.g0_.y * other.g1_.z)), (self_.g0_.x * other.g4_.w)));
}
fn dualNum_geometricProduct_origin(self_: DualNum, other: Origin) -> Origin {
    return Origin(/* e4 */ (self_.g0_.x * other.g0_));
}
fn dualNum_geometricProduct_plane(self_: DualNum, other: Plane) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.y * other.g0_.w * -1.0)), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_.x) * other.g0_));
}
fn dualNum_geometricProduct_point(self_: DualNum, other: Point) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_.x) * other.g0_), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.y * other.g0_.x * -1.0), (self_.g0_.y * other.g0_.y * -1.0), (self_.g0_.y * other.g0_.z * -1.0), 0.0));
}
fn dualNum_geometricProduct_scalar(self_: DualNum, other: Scalar) -> DualNum {
    return DualNum(/* scalar, e1234 */ (vec2<f32>(other.g0_) * self_.g0_));
}
fn flector_geometricProduct_antiScalar(self_: Flector, other: AntiScalar) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g1_.w * other.g0_)), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), 0.0));
}
fn flector_geometricProduct_dualNum(self_: Flector, other: DualNum) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g0_.x), (other.g0_.x * self_.g0_.y), (other.g0_.x * self_.g0_.z), ((other.g0_.x * self_.g0_.w) + (other.g0_.y * self_.g1_.w))), /* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.x * self_.g1_.x) + (other.g0_.y * self_.g0_.x)), ((other.g0_.x * self_.g1_.y) + (other.g0_.y * self_.g0_.y)), ((other.g0_.x * self_.g1_.z) + (other.g0_.y * self_.g0_.z)), (other.g0_.x * self_.g1_.w)));
}
fn flector_geometricProduct_flector(self_: Flector, other: Flector) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(((other.g0_.y * self_.g1_.z) - (other.g1_.y * self_.g0_.z)), ((other.g0_.z * self_.g1_.x) - (other.g1_.z * self_.g0_.x)), ((other.g0_.x * self_.g1_.y) - (other.g1_.x * self_.g0_.y)), (-(other.g0_.w * self_.g1_.w) + (other.g1_.w * self_.g0_.w))) + (vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.x) * self_.g0_.wwwx) - (vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, other.g0_.z) * self_.g1_.wwwz) - (vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g1_.y) * other.g0_.wwwy) + (vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g0_.z) * other.g1_.wwwz) - (other.g0_.zxyx * self_.g1_.yzxx) + (other.g1_.zxyy * self_.g0_.yzxy)), /* e23, e31, e12, scalar */ (vec4<f32>((-(other.g0_.y * self_.g0_.z) - (other.g1_.w * self_.g0_.x)), (-(other.g0_.z * self_.g0_.x) - (other.g1_.w * self_.g0_.y)), (-(other.g0_.x * self_.g0_.y) - (other.g1_.w * self_.g0_.z)), ((other.g0_.y * self_.g0_.y) + (other.g0_.z * self_.g0_.z))) - (vec4<f32>(self_.g1_.w) * vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.w)) + (other.g0_.zxyx * self_.g0_.yzxx)));
}
fn flector_geometricProduct_horizon(self_: Flector, other: Horizon) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g0_.w)), /* e23, e31, e12, scalar */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g1_.w) * vec4<f32>(-1.0)));
}
fn flector_geometricProduct_line(self_: Flector, other: Line) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(((other.g1_.x * self_.g1_.w) + (other.g1_.y * self_.g0_.z)), ((other.g1_.y * self_.g1_.w) + (other.g1_.z * self_.g0_.x)), ((other.g1_.x * self_.g0_.y) + (other.g1_.z * self_.g1_.w)), (-(other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z) - (other.g1_.x * self_.g1_.x) - (other.g1_.y * self_.g1_.y) - (other.g1_.z * self_.g1_.z))) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.x) * self_.g0_.yzxx)), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.x * self_.g1_.w) + (other.g0_.y * self_.g0_.z) + (other.g1_.x * self_.g0_.w) + (other.g1_.y * self_.g1_.z) - (other.g1_.z * self_.g1_.y)), ((other.g0_.y * self_.g1_.w) + (other.g0_.z * self_.g0_.x) - (other.g1_.x * self_.g1_.z) + (other.g1_.y * self_.g0_.w) + (other.g1_.z * self_.g1_.x)), ((other.g0_.x * self_.g0_.y) + (other.g0_.z * self_.g1_.w) + (other.g1_.x * self_.g1_.y) - (other.g1_.y * self_.g1_.x) + (other.g1_.z * self_.g0_.w)), (-(other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))) - (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.y, other.g1_.x) * self_.g0_.yzxx)));
}
fn flector_geometricProduct_motor(self_: Flector, other: Motor) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>((self_.g0_.z * other.g1_.y), (self_.g0_.y * other.g1_.w), (self_.g0_.z * other.g1_.w), (-(self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z) - (self_.g1_.x * other.g1_.x) - (self_.g1_.y * other.g1_.y) - (self_.g1_.z * other.g1_.z))) + (vec4<f32>(self_.g1_.w) * vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, other.g0_.w)) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.x) * self_.g0_.yzxx) + (self_.g0_.xxyw * other.g1_.wzxw)), /* e423, e431, e412, e321 */ (vec4<f32>(((self_.g0_.x * other.g0_.w) + (self_.g0_.z * other.g0_.y) + (self_.g1_.x * other.g1_.w) + (self_.g1_.z * other.g1_.y) + (self_.g1_.w * other.g0_.x)), ((self_.g0_.x * other.g0_.z) + (self_.g0_.y * other.g0_.w) + (self_.g1_.x * other.g1_.z) + (self_.g1_.y * other.g1_.w) + (self_.g1_.w * other.g0_.y)), ((self_.g0_.y * other.g0_.x) + (self_.g0_.z * other.g0_.w) + (self_.g1_.y * other.g1_.x) + (self_.g1_.z * other.g1_.w) + (self_.g1_.w * other.g0_.z)), ((self_.g0_.z * other.g1_.z) * -1.0)) + (vec4<f32>(self_.g0_.w, self_.g0_.w, self_.g0_.w, self_.g1_.w) * other.g1_) - (vec4<f32>(self_.g1_.y, self_.g1_.z, self_.g1_.x, self_.g0_.y) * other.g1_.zxyy) - (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.y, other.g1_.x) * self_.g0_.yzxx)));
}
fn flector_geometricProduct_multiVector(self_: Flector, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(0.0, ((self_.g0_.w * other.g4_.w) - (self_.g1_.x * other.g1_.x) - (self_.g1_.y * other.g1_.y) - (self_.g1_.z * other.g1_.z))) + (vec2<f32>(self_.g0_.x) * vec2<f32>(other.g1_.x, other.g4_.x)) + (vec2<f32>(self_.g0_.y) * vec2<f32>(other.g1_.y, other.g4_.y)) + (vec2<f32>(self_.g0_.z) * vec2<f32>(other.g1_.z, other.g4_.z)) - (vec2<f32>(self_.g1_.w) * vec2<f32>(other.g4_.w, other.g1_.w))), /* e1, e2, e3, e4 */ (vec4<f32>((other.g3_.y * self_.g0_.z), (other.g3_.z * self_.g0_.x), (other.g3_.x * self_.g0_.y), (-(other.g2_.y * self_.g0_.y) - (other.g2_.z * self_.g0_.z) - (other.g3_.x * self_.g1_.x) - (other.g3_.y * self_.g1_.y) - (other.g3_.z * self_.g1_.z))) + (vec4<f32>(other.g0_.x) * self_.g0_) + (vec4<f32>(self_.g1_.w) * vec4<f32>(other.g3_.x, other.g3_.y, other.g3_.z, other.g0_.y)) - (vec4<f32>(other.g3_.z, other.g3_.x, other.g3_.y, other.g2_.x) * self_.g0_.yzxx)), /* e41, e42, e43 */ (vec3<f32>(((self_.g0_.y * other.g4_.z) - (self_.g0_.z * other.g4_.y) - (self_.g1_.y * other.g1_.z) + (self_.g1_.z * other.g1_.y)), (-(self_.g0_.x * other.g4_.z) + (self_.g0_.z * other.g4_.x) + (self_.g1_.x * other.g1_.z) - (self_.g1_.z * other.g1_.x)), ((self_.g0_.x * other.g4_.y) - (self_.g0_.y * other.g4_.x) - (self_.g1_.x * other.g1_.y) + (self_.g1_.y * other.g1_.x))) + (vec3<f32>(self_.g0_.w) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z)) - (vec3<f32>(self_.g1_.w) * vec3<f32>(other.g4_.x, other.g4_.y, other.g4_.z)) - (vec3<f32>(other.g1_.w) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z)) + (vec3<f32>(other.g4_.w) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z))), /* e23, e31, e12 */ (vec3<f32>(((self_.g0_.y * other.g1_.z) - (self_.g0_.z * other.g1_.y)), (-(self_.g0_.x * other.g1_.z) + (self_.g0_.z * other.g1_.x)), ((self_.g0_.x * other.g1_.y) - (self_.g0_.y * other.g1_.x))) - (vec3<f32>(self_.g1_.w) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z)) - (vec3<f32>(other.g4_.w) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z))), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.y * self_.g0_.x) + (other.g2_.x * self_.g1_.w) + (other.g2_.y * self_.g0_.z) + (other.g3_.x * self_.g0_.w) + (other.g3_.y * self_.g1_.z) - (other.g3_.z * self_.g1_.y)), ((other.g0_.y * self_.g0_.y) + (other.g2_.y * self_.g1_.w) + (other.g2_.z * self_.g0_.x) - (other.g3_.x * self_.g1_.z) + (other.g3_.y * self_.g0_.w) + (other.g3_.z * self_.g1_.x)), ((other.g0_.y * self_.g0_.z) + (other.g2_.x * self_.g0_.y) + (other.g2_.z * self_.g1_.w) + (other.g3_.x * self_.g1_.y) - (other.g3_.y * self_.g1_.x) + (other.g3_.z * self_.g0_.w)), (-(other.g3_.y * self_.g0_.y) - (other.g3_.z * self_.g0_.z))) + (vec4<f32>(other.g0_.x) * self_.g1_) - (vec4<f32>(other.g2_.z, other.g2_.x, other.g2_.y, other.g3_.x) * self_.g0_.yzxx)));
}
fn flector_geometricProduct_origin(self_: Flector, other: Origin) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g1_.w) * vec4<f32>(-1.0)), /* e23, e31, e12, scalar */ vec4<f32>(0.0));
}
fn flector_geometricProduct_plane(self_: Flector, other: Plane) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>((-(self_.g0_.z * other.g0_.y) - (self_.g1_.w * other.g0_.x)), (-(self_.g0_.x * other.g0_.z) - (self_.g1_.w * other.g0_.y)), (-(self_.g0_.y * other.g0_.x) - (self_.g1_.w * other.g0_.z)), ((self_.g0_.z * other.g0_.z) + (self_.g0_.w * other.g0_.w))) + (vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g0_.y) * other.g0_.wwwy) + (self_.g0_.yzxx * other.g0_.zxyx)), /* e23, e31, e12, scalar */ (vec4<f32>(other.g0_.w) * vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g1_.w) * vec4<f32>(-1.0)));
}
fn flector_geometricProduct_point(self_: Flector, other: Point) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(((self_.g0_.w * other.g0_.x) + (self_.g1_.z * other.g0_.y)), ((self_.g0_.w * other.g0_.y) + (self_.g1_.x * other.g0_.z)), ((self_.g0_.w * other.g0_.z) + (self_.g1_.y * other.g0_.x)), (-(self_.g1_.z * other.g0_.z) - (self_.g1_.w * other.g0_.w))) - (vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g1_.x) * other.g0_.wwwx) - (self_.g1_.yzxy * other.g0_.zxyy)), /* e23, e31, e12, scalar */ (vec4<f32>((-(self_.g0_.z * other.g0_.y) - (self_.g1_.w * other.g0_.x)), (-(self_.g0_.x * other.g0_.z) - (self_.g1_.w * other.g0_.y)), (-(self_.g0_.y * other.g0_.x) - (self_.g1_.w * other.g0_.z)), ((self_.g0_.y * other.g0_.y) + (self_.g0_.z * other.g0_.z))) + (self_.g0_.yzxx * other.g0_.zxyx)));
}
fn flector_geometricProduct_scalar(self_: Flector, other: Scalar) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(other.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_) * self_.g1_));
}
fn horizon_geometricProduct_antiScalar(self_: Horizon, other: AntiScalar) -> Origin {
    return Origin(/* e4 */ (other.g0_ * self_.g0_));
}
fn horizon_geometricProduct_dualNum(self_: Horizon, other: DualNum) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.y * self_.g0_)), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.x * self_.g0_)));
}
fn horizon_geometricProduct_flector(self_: Horizon, other: Flector) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, other.g0_.w) * vec4<f32>(-1.0)), /* e23, e31, e12, scalar */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.w) * vec4<f32>(-1.0)));
}
fn horizon_geometricProduct_horizon(self_: Horizon, other: Horizon) -> Scalar {
    return Scalar(/* scalar */ (other.g0_ * self_.g0_ * -1.0));
}
fn horizon_geometricProduct_line(self_: Horizon, other: Line) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_), 0.0), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), 0.0));
}
fn horizon_geometricProduct_motor(self_: Horizon, other: Motor) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, other.g0_.w)), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.w)));
}
fn horizon_geometricProduct_multiVector(self_: Horizon, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(self_.g0_) * vec2<f32>(other.g4_.w, other.g1_.w) * vec2<f32>(-1.0)), /* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g3_.x, other.g3_.y, other.g3_.z, other.g0_.y)), /* e41, e42, e43 */ (vec3<f32>(self_.g0_) * vec3<f32>(other.g4_.x, other.g4_.y, other.g4_.z) * vec3<f32>(-1.0)), /* e23, e31, e12 */ (vec3<f32>(self_.g0_) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z) * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g2_.x, other.g2_.y, other.g2_.z, other.g0_.x)));
}
fn horizon_geometricProduct_origin(self_: Horizon, other: Origin) -> AntiScalar {
    return AntiScalar(/* e1234 */ (self_.g0_ * other.g0_ * -1.0));
}
fn horizon_geometricProduct_plane(self_: Horizon, other: Plane) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g0_.x * self_.g0_ * -1.0), (other.g0_.y * self_.g0_ * -1.0), (other.g0_.z * self_.g0_ * -1.0), 0.0), /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.w * self_.g0_ * -1.0)));
}
fn horizon_geometricProduct_point(self_: Horizon, other: Point) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.w * self_.g0_ * -1.0)), /* e23, e31, e12, scalar */ vec4<f32>((other.g0_.x * self_.g0_ * -1.0), (other.g0_.y * self_.g0_ * -1.0), (other.g0_.z * self_.g0_ * -1.0), 0.0));
}
fn horizon_geometricProduct_scalar(self_: Horizon, other: Scalar) -> Horizon {
    return Horizon(/* e321 */ (self_.g0_ * other.g0_));
}
fn line_geometricProduct_antiScalar(self_: Line, other: AntiScalar) -> Line {
    return Line(/* e41, e42, e43 */ (vec3<f32>(other.g0_) * self_.g1_), /* e23, e31, e12 */ vec3<f32>(0.0));
}
fn line_geometricProduct_dualNum(self_: Line, other: DualNum) -> Line {
    return Line(/* e41, e42, e43 */ ((vec3<f32>(other.g0_.x) * self_.g0_) + (vec3<f32>(other.g0_.y) * self_.g1_)), /* e23, e31, e12 */ (vec3<f32>(other.g0_.x) * self_.g1_));
}
fn line_geometricProduct_flector(self_: Line, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(((self_.g1_.x * other.g1_.w) - (self_.g1_.y * other.g0_.z)), ((self_.g1_.y * other.g1_.w) - (self_.g1_.z * other.g0_.x)), (-(self_.g1_.x * other.g0_.y) + (self_.g1_.z * other.g1_.w)), ((self_.g0_.y * other.g0_.y) + (self_.g0_.z * other.g0_.z) - (self_.g1_.x * other.g1_.x) - (self_.g1_.y * other.g1_.y) - (self_.g1_.z * other.g1_.z))) + (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g0_.x) * other.g0_.yzxx)), /* e423, e431, e412, e321 */ (vec4<f32>((-(self_.g0_.x * other.g1_.w) + (self_.g0_.y * other.g0_.z) + (self_.g1_.x * other.g0_.w) - (self_.g1_.y * other.g1_.z) + (self_.g1_.z * other.g1_.y)), (-(self_.g0_.y * other.g1_.w) + (self_.g0_.z * other.g0_.x) + (self_.g1_.x * other.g1_.z) + (self_.g1_.y * other.g0_.w) - (self_.g1_.z * other.g1_.x)), ((self_.g0_.x * other.g0_.y) - (self_.g0_.z * other.g1_.w) - (self_.g1_.x * other.g1_.y) + (self_.g1_.y * other.g1_.x) + (self_.g1_.z * other.g0_.w)), (-(self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g1_.x) * other.g0_.yzxx)));
}
fn line_geometricProduct_horizon(self_: Line, other: Horizon) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), 0.0), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.x * other.g0_ * -1.0), (self_.g0_.y * other.g0_ * -1.0), (self_.g0_.z * other.g0_ * -1.0), 0.0));
}
fn line_geometricProduct_line(self_: Line, other: Line) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(((other.g0_.y * self_.g1_.z) - (other.g0_.z * self_.g1_.y) + (other.g1_.y * self_.g0_.z) - (other.g1_.z * self_.g0_.y)), (-(other.g0_.x * self_.g1_.z) + (other.g0_.z * self_.g1_.x) - (other.g1_.x * self_.g0_.z) + (other.g1_.z * self_.g0_.x)), ((other.g0_.x * self_.g1_.y) - (other.g0_.y * self_.g1_.x) + (other.g1_.x * self_.g0_.y) - (other.g1_.y * self_.g0_.x)), (-(other.g0_.x * self_.g1_.x) - (other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))), /* e23, e31, e12, scalar */ vec4<f32>(((other.g1_.y * self_.g1_.z) - (other.g1_.z * self_.g1_.y)), (-(other.g1_.x * self_.g1_.z) + (other.g1_.z * self_.g1_.x)), ((other.g1_.x * self_.g1_.y) - (other.g1_.y * self_.g1_.x)), (-(other.g1_.x * self_.g1_.x) - (other.g1_.y * self_.g1_.y) - (other.g1_.z * self_.g1_.z))));
}
fn line_geometricProduct_motor(self_: Line, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(((self_.g0_.x * other.g1_.w) + (self_.g0_.z * other.g1_.y) + (self_.g1_.x * other.g0_.w) + (self_.g1_.z * other.g0_.y)), ((self_.g0_.x * other.g1_.z) + (self_.g0_.y * other.g1_.w) + (self_.g1_.x * other.g0_.z) + (self_.g1_.y * other.g0_.w)), ((self_.g0_.y * other.g1_.x) + (self_.g0_.z * other.g1_.w) + (self_.g1_.y * other.g0_.x) + (self_.g1_.z * other.g0_.w)), (-(self_.g0_.y * other.g1_.y) - (self_.g0_.z * other.g1_.z) - (self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))) - (vec4<f32>(self_.g0_.y, self_.g0_.z, self_.g0_.x, self_.g0_.x) * other.g1_.zxyx) - (vec4<f32>(self_.g1_.y, self_.g1_.z, self_.g1_.x, self_.g1_.x) * other.g0_.zxyx)), /* e23, e31, e12, scalar */ (vec4<f32>(((self_.g1_.x * other.g1_.w) + (self_.g1_.z * other.g1_.y)), ((self_.g1_.x * other.g1_.z) + (self_.g1_.y * other.g1_.w)), ((self_.g1_.y * other.g1_.x) + (self_.g1_.z * other.g1_.w)), (-(self_.g1_.y * other.g1_.y) - (self_.g1_.z * other.g1_.z))) - (vec4<f32>(self_.g1_.y, self_.g1_.z, self_.g1_.x, self_.g1_.x) * other.g1_.zxyx)));
}
fn line_geometricProduct_multiVector(self_: Line, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(0.0, (-(self_.g1_.x * other.g2_.x) - (self_.g1_.y * other.g2_.y) - (self_.g1_.z * other.g2_.z))) - (vec2<f32>(other.g3_.x) * vec2<f32>(self_.g1_.x, self_.g0_.x)) - (vec2<f32>(other.g3_.y) * vec2<f32>(self_.g1_.y, self_.g0_.y)) - (vec2<f32>(other.g3_.z) * vec2<f32>(self_.g1_.z, self_.g0_.z))), /* e1, e2, e3, e4 */ (vec4<f32>(((self_.g1_.x * other.g4_.w) - (self_.g1_.y * other.g1_.z)), ((self_.g1_.y * other.g4_.w) - (self_.g1_.z * other.g1_.x)), (-(self_.g1_.x * other.g1_.y) + (self_.g1_.z * other.g4_.w)), ((self_.g0_.y * other.g1_.y) + (self_.g0_.z * other.g1_.z) - (self_.g1_.x * other.g4_.x) - (self_.g1_.y * other.g4_.y) - (self_.g1_.z * other.g4_.z))) + (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g0_.x) * other.g1_.yzxx)), /* e41, e42, e43 */ ((vec3<f32>(other.g0_.x) * self_.g0_) + (vec3<f32>(other.g0_.y) * self_.g1_) - (self_.g0_.yzx * other.g3_.zxy) + (self_.g0_.zxy * other.g3_.yzx) - (self_.g1_.yzx * other.g2_.zxy) + (self_.g1_.zxy * other.g2_.yzx)), /* e23, e31, e12 */ ((vec3<f32>(other.g0_.x) * self_.g1_) - (self_.g1_.yzx * other.g3_.zxy) + (self_.g1_.zxy * other.g3_.yzx)), /* e423, e431, e412, e321 */ (vec4<f32>((-(self_.g0_.x * other.g4_.w) + (self_.g0_.y * other.g1_.z) + (self_.g1_.x * other.g1_.w) - (self_.g1_.y * other.g4_.z) + (self_.g1_.z * other.g4_.y)), (-(self_.g0_.y * other.g4_.w) + (self_.g0_.z * other.g1_.x) + (self_.g1_.x * other.g4_.z) + (self_.g1_.y * other.g1_.w) - (self_.g1_.z * other.g4_.x)), ((self_.g0_.x * other.g1_.y) - (self_.g0_.z * other.g4_.w) - (self_.g1_.x * other.g4_.y) + (self_.g1_.y * other.g4_.x) + (self_.g1_.z * other.g1_.w)), (-(self_.g1_.y * other.g1_.y) - (self_.g1_.z * other.g1_.z))) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g1_.x) * other.g1_.yzxx)));
}
fn line_geometricProduct_origin(self_: Line, other: Origin) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), 0.0));
}
fn line_geometricProduct_plane(self_: Line, other: Plane) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g1_.x * other.g0_.w), (self_.g1_.y * other.g0_.w), (self_.g1_.z * other.g0_.w), (-(self_.g1_.x * other.g0_.x) - (self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))), /* e423, e431, e412, e321 */ vec4<f32>((-(self_.g0_.x * other.g0_.w) - (self_.g1_.y * other.g0_.z) + (self_.g1_.z * other.g0_.y)), (-(self_.g0_.y * other.g0_.w) + (self_.g1_.x * other.g0_.z) - (self_.g1_.z * other.g0_.x)), (-(self_.g0_.z * other.g0_.w) - (self_.g1_.x * other.g0_.y) + (self_.g1_.y * other.g0_.x)), 0.0));
}
fn line_geometricProduct_point(self_: Line, other: Point) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(((self_.g1_.y * other.g0_.z) * -1.0), ((self_.g1_.z * other.g0_.x) * -1.0), ((self_.g1_.x * other.g0_.y) * -1.0), ((self_.g0_.y * other.g0_.y) + (self_.g0_.z * other.g0_.z))) + (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g0_.x) * other.g0_.yzxx)), /* e423, e431, e412, e321 */ (vec4<f32>(((self_.g0_.y * other.g0_.z) + (self_.g1_.x * other.g0_.w)), ((self_.g0_.z * other.g0_.x) + (self_.g1_.y * other.g0_.w)), ((self_.g0_.x * other.g0_.y) + (self_.g1_.z * other.g0_.w)), (-(self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g1_.x) * other.g0_.yzxx)));
}
fn line_geometricProduct_scalar(self_: Line, other: Scalar) -> Line {
    return Line(/* e41, e42, e43 */ (vec3<f32>(other.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(other.g0_) * self_.g1_));
}
fn motor_geometricProduct_antiScalar(self_: Motor, other: AntiScalar) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(other.g0_) * self_.g1_), /* e23, e31, e12, scalar */ vec4<f32>(0.0));
}
fn motor_geometricProduct_dualNum(self_: Motor, other: DualNum) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ ((vec4<f32>(other.g0_.x) * self_.g0_) + (vec4<f32>(other.g0_.y) * self_.g1_)), /* e23, e31, e12, scalar */ (vec4<f32>(other.g0_.x) * self_.g1_));
}
fn motor_geometricProduct_flector(self_: Motor, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(0.0, 0.0, 0.0, ((other.g0_.z * self_.g0_.z) - (other.g1_.y * self_.g1_.y) - (other.g1_.z * self_.g1_.z) - (other.g1_.w * self_.g0_.w))) - (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.y, other.g1_.x) * self_.g1_.yzxx) + (vec4<f32>(other.g1_.w, other.g1_.w, other.g1_.w, other.g0_.w) * self_.g1_) + (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.w, self_.g0_.y) * other.g0_.yzzy) + (vec4<f32>(self_.g1_.w, self_.g1_.w, self_.g1_.y, self_.g0_.x) * other.g0_.xyxx)), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.z * self_.g0_.y) + (other.g1_.x * self_.g1_.w) + (other.g1_.y * self_.g1_.z) - (other.g1_.w * self_.g0_.x)), ((other.g0_.x * self_.g0_.z) + (other.g1_.y * self_.g1_.w) + (other.g1_.z * self_.g1_.x) - (other.g1_.w * self_.g0_.y)), ((other.g0_.y * self_.g0_.x) + (other.g1_.x * self_.g1_.y) + (other.g1_.z * self_.g1_.w) - (other.g1_.w * self_.g0_.z)), 0.0) + (vec4<f32>(other.g0_.w, other.g0_.w, other.g0_.w, other.g1_.w) * self_.g1_) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.z) * self_.g1_.yzxz) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.w, self_.g1_.y) * other.g0_.yzzy) - (vec4<f32>(self_.g0_.w, self_.g0_.w, self_.g0_.y, self_.g1_.x) * other.g0_.xyxx)));
}
fn motor_geometricProduct_horizon(self_: Motor, other: Horizon) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, -1.0)), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g1_.w) * vec4<f32>(-1.0, -1.0, -1.0, 1.0)));
}
fn motor_geometricProduct_line(self_: Motor, other: Line) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(((other.g0_.x * self_.g1_.w) + (other.g0_.y * self_.g1_.z) + (other.g1_.x * self_.g0_.w) + (other.g1_.y * self_.g0_.z)), ((other.g0_.y * self_.g1_.w) + (other.g0_.z * self_.g1_.x) + (other.g1_.y * self_.g0_.w) + (other.g1_.z * self_.g0_.x)), ((other.g0_.x * self_.g1_.y) + (other.g0_.z * self_.g1_.w) + (other.g1_.x * self_.g0_.y) + (other.g1_.z * self_.g0_.w)), (-(other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))) - (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.y, other.g0_.x) * self_.g1_.yzxx) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g1_.x) * self_.g0_.yzxx)), /* e23, e31, e12, scalar */ (vec4<f32>(((other.g1_.x * self_.g1_.w) + (other.g1_.y * self_.g1_.z)), ((other.g1_.y * self_.g1_.w) + (other.g1_.z * self_.g1_.x)), ((other.g1_.x * self_.g1_.y) + (other.g1_.z * self_.g1_.w)), (-(other.g1_.y * self_.g1_.y) - (other.g1_.z * self_.g1_.z))) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g1_.x) * self_.g1_.yzxx)));
}
fn motor_geometricProduct_motor(self_: Motor, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(((other.g0_.y * self_.g1_.z) + (other.g0_.w * self_.g1_.x) + (other.g1_.y * self_.g0_.z) + (other.g1_.w * self_.g0_.x)), ((other.g0_.z * self_.g1_.x) + (other.g0_.w * self_.g1_.y) + (other.g1_.z * self_.g0_.x) + (other.g1_.w * self_.g0_.y)), ((other.g0_.z * self_.g1_.w) + (other.g0_.w * self_.g1_.z) + (other.g1_.z * self_.g0_.w) + (other.g1_.w * self_.g0_.z)), (-(other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))) + (other.g0_.xyxw * self_.g1_.wwyw) - (other.g0_.zxyx * self_.g1_.yzxx) + (other.g1_.xyxw * self_.g0_.wwyw) - (other.g1_.zxyx * self_.g0_.yzxx)), /* e23, e31, e12, scalar */ (vec4<f32>(((other.g1_.y * self_.g1_.z) + (other.g1_.w * self_.g1_.x)), ((other.g1_.z * self_.g1_.x) + (other.g1_.w * self_.g1_.y)), ((other.g1_.z * self_.g1_.w) + (other.g1_.w * self_.g1_.z)), (-(other.g1_.y * self_.g1_.y) - (other.g1_.z * self_.g1_.z))) + (other.g1_.xyxw * self_.g1_.wwyw) - (other.g1_.zxyx * self_.g1_.yzxx)));
}
fn motor_geometricProduct_multiVector(self_: Motor, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(0.0, ((other.g0_.y * self_.g1_.w) - (other.g3_.x * self_.g0_.x) - (other.g3_.y * self_.g0_.y) - (other.g3_.z * self_.g0_.z))) + (vec2<f32>(other.g0_.x) * vec2<f32>(self_.g1_.w, self_.g0_.w)) - (vec2<f32>(self_.g1_.x) * vec2<f32>(other.g3_.x, other.g2_.x)) - (vec2<f32>(self_.g1_.y) * vec2<f32>(other.g3_.y, other.g2_.y)) - (vec2<f32>(self_.g1_.z) * vec2<f32>(other.g3_.z, other.g2_.z))), /* e1, e2, e3, e4 */ (vec4<f32>((self_.g1_.w * other.g1_.x), (self_.g1_.y * other.g4_.w), (self_.g1_.z * other.g4_.w), ((self_.g0_.y * other.g1_.y) + (self_.g0_.z * other.g1_.z) - (self_.g0_.w * other.g4_.w) - (self_.g1_.y * other.g4_.y) - (self_.g1_.z * other.g4_.z))) + (vec4<f32>(self_.g1_.z, self_.g1_.w, self_.g1_.w, self_.g0_.x) * other.g1_.yyzx) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g4_.x) * self_.g1_.yzxx) + (vec4<f32>(other.g4_.w, other.g1_.z, other.g1_.x, other.g1_.w) * self_.g1_.xxyw)), /* e41, e42, e43 */ ((vec3<f32>(other.g0_.x) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z)) + (vec3<f32>(other.g0_.y) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z)) - (vec3<f32>(self_.g0_.y, self_.g0_.z, self_.g0_.x) * other.g3_.zxy) + (vec3<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.w) * other.g3_.yzz) + (vec3<f32>(self_.g0_.w, self_.g0_.w, self_.g0_.y) * other.g3_.xyx) - (vec3<f32>(self_.g1_.y, self_.g1_.z, self_.g1_.x) * other.g2_.zxy) + (vec3<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.w) * other.g2_.yzz) + (vec3<f32>(self_.g1_.w, self_.g1_.w, self_.g1_.y) * other.g2_.xyx)), /* e23, e31, e12 */ ((vec3<f32>(other.g0_.x) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z)) - (vec3<f32>(self_.g1_.y, self_.g1_.z, self_.g1_.x) * other.g3_.zxy) + (vec3<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.w) * other.g3_.yzz) + (vec3<f32>(self_.g1_.w, self_.g1_.w, self_.g1_.y) * other.g3_.xyx)), /* e423, e431, e412, e321 */ (vec4<f32>((-(self_.g0_.x * other.g4_.w) + (self_.g0_.y * other.g1_.z) + (self_.g1_.z * other.g4_.y) + (self_.g1_.w * other.g4_.x)), (-(self_.g0_.y * other.g4_.w) + (self_.g0_.z * other.g1_.x) + (self_.g1_.y * other.g1_.w) + (self_.g1_.w * other.g4_.y)), ((self_.g0_.x * other.g1_.y) - (self_.g0_.z * other.g4_.w) + (self_.g1_.z * other.g1_.w) + (self_.g1_.w * other.g4_.z)), 0.0) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g1_.x) * other.g1_.yzxx) - (vec4<f32>(self_.g0_.w, self_.g0_.w, self_.g0_.w, self_.g1_.y) * other.g1_.xyzy) + (vec4<f32>(other.g1_.w, other.g4_.z, other.g4_.x, other.g4_.w) * self_.g1_.xxyw) - (vec4<f32>(other.g4_.z, other.g4_.x, other.g4_.y, other.g1_.z) * self_.g1_.yzxz)));
}
fn motor_geometricProduct_origin(self_: Motor, other: Origin) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g1_.w * other.g0_)), /* e423, e431, e412, e321 */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), 0.0));
}
fn motor_geometricProduct_plane(self_: Motor, other: Plane) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g1_.x * other.g0_.w), (self_.g1_.y * other.g0_.w), (self_.g1_.z * other.g0_.w), (-(self_.g0_.w * other.g0_.w) - (self_.g1_.x * other.g0_.x) - (self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))), /* e423, e431, e412, e321 */ vec4<f32>((-(self_.g0_.x * other.g0_.w) - (self_.g1_.y * other.g0_.z) + (self_.g1_.z * other.g0_.y) + (self_.g1_.w * other.g0_.x)), (-(self_.g0_.y * other.g0_.w) + (self_.g1_.x * other.g0_.z) - (self_.g1_.z * other.g0_.x) + (self_.g1_.w * other.g0_.y)), (-(self_.g0_.z * other.g0_.w) - (self_.g1_.x * other.g0_.y) + (self_.g1_.y * other.g0_.x) + (self_.g1_.w * other.g0_.z)), (self_.g1_.w * other.g0_.w)));
}
fn motor_geometricProduct_point(self_: Motor, other: Point) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(((self_.g1_.y * other.g0_.z) * -1.0), ((self_.g1_.z * other.g0_.x) * -1.0), ((self_.g1_.x * other.g0_.y) * -1.0), ((self_.g0_.z * other.g0_.z) + (self_.g1_.w * other.g0_.w))) + (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g0_.x) * other.g0_.yzxx) + (vec4<f32>(self_.g1_.w, self_.g1_.w, self_.g1_.w, self_.g0_.y) * other.g0_.xyzy)), /* e423, e431, e412, e321 */ (vec4<f32>(((self_.g0_.y * other.g0_.z) + (self_.g1_.x * other.g0_.w)), ((self_.g0_.z * other.g0_.x) + (self_.g1_.y * other.g0_.w)), ((self_.g0_.x * other.g0_.y) + (self_.g1_.z * other.g0_.w)), ((self_.g1_.z * other.g0_.z) * -1.0)) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g1_.x) * other.g0_.yzxx) - (vec4<f32>(self_.g0_.w, self_.g0_.w, self_.g0_.w, self_.g1_.y) * other.g0_.xyzy)));
}
fn motor_geometricProduct_scalar(self_: Motor, other: Scalar) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(other.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(other.g0_) * self_.g1_));
}
fn multiVector_geometricProduct_antiScalar(self_: MultiVector, other: AntiScalar) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (self_.g0_.x * other.g0_)), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g4_.w * other.g0_)), /* e41, e42, e43 */ (vec3<f32>(other.g0_) * self_.g3_), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), 0.0));
}
fn multiVector_geometricProduct_dualNum(self_: MultiVector, other: DualNum) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((other.g0_.x * self_.g0_.x), ((other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x))), /* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g1_.x), (other.g0_.x * self_.g1_.y), (other.g0_.x * self_.g1_.z), ((other.g0_.x * self_.g1_.w) + (other.g0_.y * self_.g4_.w))), /* e41, e42, e43 */ ((vec3<f32>(other.g0_.x) * self_.g2_) + (vec3<f32>(other.g0_.y) * self_.g3_)), /* e23, e31, e12 */ (vec3<f32>(other.g0_.x) * self_.g3_), /* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.x * self_.g4_.x) + (other.g0_.y * self_.g1_.x)), ((other.g0_.x * self_.g4_.y) + (other.g0_.y * self_.g1_.y)), ((other.g0_.x * self_.g4_.z) + (other.g0_.y * self_.g1_.z)), (other.g0_.x * self_.g4_.w)));
}
fn multiVector_geometricProduct_flector(self_: MultiVector, other: Flector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(0.0, (-(other.g0_.x * self_.g4_.x) - (other.g0_.y * self_.g4_.y) - (other.g0_.z * self_.g4_.z) + (other.g1_.w * self_.g1_.w))) + (vec2<f32>(self_.g1_.x) * vec2<f32>(other.g0_.x, other.g1_.x)) + (vec2<f32>(self_.g1_.y) * vec2<f32>(other.g0_.y, other.g1_.y)) + (vec2<f32>(self_.g1_.z) * vec2<f32>(other.g0_.z, other.g1_.z)) - (vec2<f32>(self_.g4_.w) * vec2<f32>(other.g1_.w, other.g0_.w))), /* e1, e2, e3, e4 */ (vec4<f32>(((self_.g3_.x * other.g1_.w) - (self_.g3_.y * other.g0_.z)), ((self_.g3_.y * other.g1_.w) - (self_.g3_.z * other.g0_.x)), (-(self_.g3_.x * other.g0_.y) + (self_.g3_.z * other.g1_.w)), (-(self_.g0_.y * other.g1_.w) + (self_.g2_.y * other.g0_.y) + (self_.g2_.z * other.g0_.z) - (self_.g3_.x * other.g1_.x) - (self_.g3_.y * other.g1_.y) - (self_.g3_.z * other.g1_.z))) + (vec4<f32>(self_.g0_.x) * other.g0_) + (vec4<f32>(self_.g3_.z, self_.g3_.x, self_.g3_.y, self_.g2_.x) * other.g0_.yzxx)), /* e41, e42, e43 */ (vec3<f32>(((other.g0_.y * self_.g4_.z) - (other.g0_.z * self_.g4_.y) - (other.g1_.y * self_.g1_.z) + (other.g1_.z * self_.g1_.y)), (-(other.g0_.x * self_.g4_.z) + (other.g0_.z * self_.g4_.x) + (other.g1_.x * self_.g1_.z) - (other.g1_.z * self_.g1_.x)), ((other.g0_.x * self_.g4_.y) - (other.g0_.y * self_.g4_.x) - (other.g1_.x * self_.g1_.y) + (other.g1_.y * self_.g1_.x))) - (vec3<f32>(other.g0_.w) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z)) + (vec3<f32>(other.g1_.w) * vec3<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z)) + (vec3<f32>(self_.g1_.w) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z)) - (vec3<f32>(self_.g4_.w) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z))), /* e23, e31, e12 */ (vec3<f32>((-(other.g0_.y * self_.g1_.z) + (other.g0_.z * self_.g1_.y)), ((other.g0_.x * self_.g1_.z) - (other.g0_.z * self_.g1_.x)), (-(other.g0_.x * self_.g1_.y) + (other.g0_.y * self_.g1_.x))) - (vec3<f32>(other.g1_.w) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z)) - (vec3<f32>(self_.g4_.w) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z))), /* e423, e431, e412, e321 */ (vec4<f32>((-(self_.g2_.x * other.g1_.w) + (self_.g2_.y * other.g0_.z) + (self_.g3_.x * other.g0_.w) - (self_.g3_.y * other.g1_.z) + (self_.g3_.z * other.g1_.y)), (-(self_.g2_.y * other.g1_.w) + (self_.g2_.z * other.g0_.x) + (self_.g3_.x * other.g1_.z) + (self_.g3_.y * other.g0_.w) - (self_.g3_.z * other.g1_.x)), ((self_.g2_.x * other.g0_.y) - (self_.g2_.z * other.g1_.w) - (self_.g3_.x * other.g1_.y) + (self_.g3_.y * other.g1_.x) + (self_.g3_.z * other.g0_.w)), ((self_.g3_.z * other.g0_.z) * -1.0)) + (vec4<f32>(self_.g0_.x) * other.g1_) - (vec4<f32>(self_.g0_.y, self_.g0_.y, self_.g0_.y, self_.g3_.x) * other.g0_.xyzx) - (vec4<f32>(self_.g2_.z, self_.g2_.x, self_.g2_.y, self_.g3_.y) * other.g0_.yzxy)));
}
fn multiVector_geometricProduct_horizon(self_: MultiVector, other: Horizon) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(other.g0_) * vec2<f32>(self_.g4_.w, self_.g1_.w) * vec2<f32>(-1.0, 1.0)), /* e1, e2, e3, e4 */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g3_.x, self_.g3_.y, self_.g3_.z, self_.g0_.y) * vec4<f32>(1.0, 1.0, 1.0, -1.0)), /* e41, e42, e43 */ (vec3<f32>(other.g0_) * vec3<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z)), /* e23, e31, e12 */ (vec3<f32>(other.g0_) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z) * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g2_.x, self_.g2_.y, self_.g2_.z, self_.g0_.x) * vec4<f32>(-1.0, -1.0, -1.0, 1.0)));
}
fn multiVector_geometricProduct_line(self_: MultiVector, other: Line) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(0.0, (-(other.g1_.x * self_.g2_.x) - (other.g1_.y * self_.g2_.y) - (other.g1_.z * self_.g2_.z))) - (vec2<f32>(self_.g3_.x) * vec2<f32>(other.g1_.x, other.g0_.x)) - (vec2<f32>(self_.g3_.y) * vec2<f32>(other.g1_.y, other.g0_.y)) - (vec2<f32>(self_.g3_.z) * vec2<f32>(other.g1_.z, other.g0_.z))), /* e1, e2, e3, e4 */ (vec4<f32>(((other.g1_.x * self_.g4_.w) + (other.g1_.y * self_.g1_.z)), ((other.g1_.y * self_.g4_.w) + (other.g1_.z * self_.g1_.x)), ((other.g1_.x * self_.g1_.y) + (other.g1_.z * self_.g4_.w)), (-(other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g1_.x * self_.g4_.x) - (other.g1_.y * self_.g4_.y) - (other.g1_.z * self_.g4_.z))) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.x) * self_.g1_.yzxx)), /* e41, e42, e43 */ ((vec3<f32>(self_.g0_.x) * other.g0_) + (vec3<f32>(self_.g0_.y) * other.g1_) + (other.g0_.yzx * self_.g3_.zxy) - (other.g0_.zxy * self_.g3_.yzx) + (other.g1_.yzx * self_.g2_.zxy) - (other.g1_.zxy * self_.g2_.yzx)), /* e23, e31, e12 */ ((vec3<f32>(self_.g0_.x) * other.g1_) + (other.g1_.yzx * self_.g3_.zxy) - (other.g1_.zxy * self_.g3_.yzx)), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.x * self_.g4_.w) + (other.g0_.y * self_.g1_.z) + (other.g1_.x * self_.g1_.w) + (other.g1_.y * self_.g4_.z) - (other.g1_.z * self_.g4_.y)), ((other.g0_.y * self_.g4_.w) + (other.g0_.z * self_.g1_.x) - (other.g1_.x * self_.g4_.z) + (other.g1_.y * self_.g1_.w) + (other.g1_.z * self_.g4_.x)), ((other.g0_.x * self_.g1_.y) + (other.g0_.z * self_.g4_.w) + (other.g1_.x * self_.g4_.y) - (other.g1_.y * self_.g4_.x) + (other.g1_.z * self_.g1_.w)), (-(other.g1_.y * self_.g1_.y) - (other.g1_.z * self_.g1_.z))) - (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.y, other.g1_.x) * self_.g1_.yzxx)));
}
fn multiVector_geometricProduct_motor(self_: MultiVector, other: Motor) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(0.0, ((self_.g0_.y * other.g1_.w) - (self_.g3_.x * other.g0_.x) - (self_.g3_.y * other.g0_.y) - (self_.g3_.z * other.g0_.z))) + (vec2<f32>(self_.g0_.x) * vec2<f32>(other.g1_.w, other.g0_.w)) - (vec2<f32>(other.g1_.x) * vec2<f32>(self_.g3_.x, self_.g2_.x)) - (vec2<f32>(other.g1_.y) * vec2<f32>(self_.g3_.y, self_.g2_.y)) - (vec2<f32>(other.g1_.z) * vec2<f32>(self_.g3_.z, self_.g2_.z))), /* e1, e2, e3, e4 */ (vec4<f32>(((other.g1_.y * self_.g1_.z) + (other.g1_.w * self_.g1_.x)), ((other.g1_.z * self_.g1_.x) + (other.g1_.w * self_.g1_.y)), ((other.g1_.z * self_.g4_.w) + (other.g1_.w * self_.g1_.z)), (-(other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) + (other.g0_.w * self_.g4_.w) - (other.g1_.x * self_.g4_.x) - (other.g1_.y * self_.g4_.y) - (other.g1_.z * self_.g4_.z))) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.x) * self_.g1_.yzxx) + (vec4<f32>(self_.g4_.w, self_.g4_.w, self_.g1_.y, self_.g1_.w) * other.g1_.xyxw)), /* e41, e42, e43 */ ((vec3<f32>(self_.g0_.x) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z)) + (vec3<f32>(self_.g0_.y) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z)) + (vec3<f32>(other.g0_.y, other.g0_.w, other.g0_.w) * self_.g3_.zyz) - (vec3<f32>(other.g0_.z, other.g0_.x, other.g0_.y) * self_.g3_.yzx) + (vec3<f32>(other.g0_.w, other.g0_.z, other.g0_.x) * self_.g3_.xxy) + (vec3<f32>(other.g1_.y, other.g1_.w, other.g1_.w) * self_.g2_.zyz) - (vec3<f32>(other.g1_.z, other.g1_.x, other.g1_.y) * self_.g2_.yzx) + (vec3<f32>(other.g1_.w, other.g1_.z, other.g1_.x) * self_.g2_.xxy)), /* e23, e31, e12 */ ((vec3<f32>(self_.g0_.x) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z)) + (vec3<f32>(other.g1_.y, other.g1_.w, other.g1_.w) * self_.g3_.zyz) - (vec3<f32>(other.g1_.z, other.g1_.x, other.g1_.y) * self_.g3_.yzx) + (vec3<f32>(other.g1_.w, other.g1_.z, other.g1_.x) * self_.g3_.xxy)), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.y * self_.g1_.z) + (other.g0_.w * self_.g1_.x) + (other.g1_.x * self_.g1_.w) + (other.g1_.y * self_.g4_.z) + (other.g1_.w * self_.g4_.x)), ((other.g0_.z * self_.g1_.x) + (other.g0_.w * self_.g1_.y) + (other.g1_.y * self_.g1_.w) + (other.g1_.z * self_.g4_.x) + (other.g1_.w * self_.g4_.y)), ((other.g0_.x * self_.g1_.y) + (other.g0_.w * self_.g1_.z) + (other.g1_.x * self_.g4_.y) + (other.g1_.z * self_.g1_.w) + (other.g1_.w * self_.g4_.z)), ((other.g1_.z * self_.g1_.z) * -1.0)) + (vec4<f32>(self_.g4_.w) * vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.w)) - (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.y, other.g1_.x) * self_.g1_.yzxx) - (vec4<f32>(self_.g4_.y, self_.g4_.z, self_.g4_.x, self_.g1_.y) * other.g1_.zxyy)));
}
fn multiVector_geometricProduct_multiVector(self_: MultiVector, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(0.0, ((other.g0_.y * self_.g0_.x) - (other.g3_.x * self_.g2_.x) - (other.g3_.y * self_.g2_.y) - (other.g3_.z * self_.g2_.z) - (other.g1_.x * self_.g4_.x) - (other.g1_.y * self_.g4_.y) - (other.g1_.z * self_.g4_.z) + (other.g4_.w * self_.g1_.w))) + (vec2<f32>(other.g0_.x) * self_.g0_) - (vec2<f32>(self_.g3_.x) * vec2<f32>(other.g3_.x, other.g2_.x)) - (vec2<f32>(self_.g3_.y) * vec2<f32>(other.g3_.y, other.g2_.y)) - (vec2<f32>(self_.g3_.z) * vec2<f32>(other.g3_.z, other.g2_.z)) + (vec2<f32>(self_.g1_.x) * vec2<f32>(other.g1_.x, other.g4_.x)) + (vec2<f32>(self_.g1_.y) * vec2<f32>(other.g1_.y, other.g4_.y)) + (vec2<f32>(self_.g1_.z) * vec2<f32>(other.g1_.z, other.g4_.z)) - (vec2<f32>(self_.g4_.w) * vec2<f32>(other.g4_.w, other.g1_.w))), /* e1, e2, e3, e4 */ (vec4<f32>(((other.g3_.y * self_.g1_.z) + (self_.g3_.x * other.g4_.w) - (self_.g3_.y * other.g1_.z)), ((other.g3_.z * self_.g1_.x) + (self_.g3_.y * other.g4_.w) - (self_.g3_.z * other.g1_.x)), ((other.g3_.x * self_.g1_.y) - (self_.g3_.x * other.g1_.y) + (self_.g3_.z * other.g4_.w)), (-(self_.g0_.y * other.g4_.w) - (other.g2_.y * self_.g1_.y) - (other.g2_.z * self_.g1_.z) - (other.g3_.x * self_.g4_.x) - (other.g3_.y * self_.g4_.y) - (other.g3_.z * self_.g4_.z) + (self_.g2_.y * other.g1_.y) + (self_.g2_.z * other.g1_.z) - (self_.g3_.x * other.g4_.x) - (self_.g3_.y * other.g4_.y) - (self_.g3_.z * other.g4_.z))) + (vec4<f32>(other.g0_.x) * self_.g1_) + (vec4<f32>(self_.g0_.x) * other.g1_) + (vec4<f32>(self_.g4_.w) * vec4<f32>(other.g3_.x, other.g3_.y, other.g3_.z, other.g0_.y)) - (vec4<f32>(other.g3_.z, other.g3_.x, other.g3_.y, other.g2_.x) * self_.g1_.yzxx) + (vec4<f32>(self_.g3_.z, self_.g3_.x, self_.g3_.y, self_.g2_.x) * other.g1_.yzxx)), /* e41, e42, e43 */ (vec3<f32>(((other.g1_.y * self_.g4_.z) - (other.g1_.z * self_.g4_.y) - (other.g4_.y * self_.g1_.z) + (other.g4_.z * self_.g1_.y)), (-(other.g1_.x * self_.g4_.z) + (other.g1_.z * self_.g4_.x) + (other.g4_.x * self_.g1_.z) - (other.g4_.z * self_.g1_.x)), ((other.g1_.x * self_.g4_.y) - (other.g1_.y * self_.g4_.x) - (other.g4_.x * self_.g1_.y) + (other.g4_.y * self_.g1_.x))) + (vec3<f32>(other.g0_.x) * self_.g2_) + (vec3<f32>(other.g0_.y) * self_.g3_) + (vec3<f32>(self_.g0_.x) * other.g2_) + (vec3<f32>(self_.g0_.y) * other.g3_) - (vec3<f32>(other.g1_.w) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z)) + (vec3<f32>(other.g4_.w) * vec3<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z)) + (vec3<f32>(self_.g1_.w) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z)) - (vec3<f32>(self_.g4_.w) * vec3<f32>(other.g4_.x, other.g4_.y, other.g4_.z)) + (other.g2_.yzx * self_.g3_.zxy) - (other.g2_.zxy * self_.g3_.yzx) + (other.g3_.yzx * self_.g2_.zxy) - (other.g3_.zxy * self_.g2_.yzx)), /* e23, e31, e12 */ (vec3<f32>((-(other.g1_.y * self_.g1_.z) + (other.g1_.z * self_.g1_.y)), ((other.g1_.x * self_.g1_.z) - (other.g1_.z * self_.g1_.x)), (-(other.g1_.x * self_.g1_.y) + (other.g1_.y * self_.g1_.x))) + (vec3<f32>(other.g0_.x) * self_.g3_) + (vec3<f32>(self_.g0_.x) * other.g3_) - (vec3<f32>(other.g4_.w) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z)) - (vec3<f32>(self_.g4_.w) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z)) + (other.g3_.yzx * self_.g3_.zxy) - (other.g3_.zxy * self_.g3_.yzx)), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.y * self_.g1_.x) + (other.g2_.x * self_.g4_.w) + (other.g2_.y * self_.g1_.z) + (other.g3_.x * self_.g1_.w) + (other.g3_.y * self_.g4_.z) - (other.g3_.z * self_.g4_.y) - (self_.g2_.x * other.g4_.w) + (self_.g2_.y * other.g1_.z) + (self_.g3_.x * other.g1_.w) - (self_.g3_.y * other.g4_.z) + (self_.g3_.z * other.g4_.y)), ((other.g0_.y * self_.g1_.y) + (other.g2_.y * self_.g4_.w) + (other.g2_.z * self_.g1_.x) - (other.g3_.x * self_.g4_.z) + (other.g3_.y * self_.g1_.w) + (other.g3_.z * self_.g4_.x) - (self_.g2_.y * other.g4_.w) + (self_.g2_.z * other.g1_.x) + (self_.g3_.x * other.g4_.z) + (self_.g3_.y * other.g1_.w) - (self_.g3_.z * other.g4_.x)), ((other.g0_.y * self_.g1_.z) + (other.g2_.x * self_.g1_.y) + (other.g2_.z * self_.g4_.w) + (other.g3_.x * self_.g4_.y) - (other.g3_.y * self_.g4_.x) + (other.g3_.z * self_.g1_.w) + (self_.g2_.x * other.g1_.y) - (self_.g2_.z * other.g4_.w) - (self_.g3_.x * other.g4_.y) + (self_.g3_.y * other.g4_.x) + (self_.g3_.z * other.g1_.w)), (-(other.g3_.y * self_.g1_.y) - (other.g3_.z * self_.g1_.z) - (self_.g3_.z * other.g1_.z))) + (vec4<f32>(other.g0_.x) * self_.g4_) + (vec4<f32>(self_.g0_.x) * other.g4_) - (vec4<f32>(self_.g0_.y, self_.g0_.y, self_.g0_.y, self_.g3_.x) * other.g1_.xyzx) - (vec4<f32>(other.g2_.z, other.g2_.x, other.g2_.y, other.g3_.x) * self_.g1_.yzxx) - (vec4<f32>(self_.g2_.z, self_.g2_.x, self_.g2_.y, self_.g3_.y) * other.g1_.yzxy)));
}
fn multiVector_geometricProduct_origin(self_: MultiVector, other: Origin) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (self_.g4_.w * other.g0_ * -1.0)), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.x * other.g0_)), /* e41, e42, e43 */ (vec3<f32>(other.g0_) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z) * vec3<f32>(-1.0)), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>((self_.g3_.x * other.g0_), (self_.g3_.y * other.g0_), (self_.g3_.z * other.g0_), 0.0));
}
fn multiVector_geometricProduct_plane(self_: MultiVector, other: Plane) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((self_.g4_.w * other.g0_.w * -1.0), ((self_.g1_.x * other.g0_.x) + (self_.g1_.y * other.g0_.y) + (self_.g1_.z * other.g0_.z) + (self_.g1_.w * other.g0_.w))), /* e1, e2, e3, e4 */ vec4<f32>((self_.g3_.x * other.g0_.w), (self_.g3_.y * other.g0_.w), (self_.g3_.z * other.g0_.w), (-(self_.g0_.y * other.g0_.w) - (self_.g3_.x * other.g0_.x) - (self_.g3_.y * other.g0_.y) - (self_.g3_.z * other.g0_.z))), /* e41, e42, e43 */ (vec3<f32>(((self_.g1_.y * other.g0_.z) - (self_.g1_.z * other.g0_.y)), (-(self_.g1_.x * other.g0_.z) + (self_.g1_.z * other.g0_.x)), ((self_.g1_.x * other.g0_.y) - (self_.g1_.y * other.g0_.x))) - (vec3<f32>(self_.g4_.w) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z)) + (vec3<f32>(other.g0_.w) * vec3<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z))), /* e23, e31, e12 */ (vec3<f32>(other.g0_.w) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z) * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ vec4<f32>(((self_.g0_.x * other.g0_.x) - (self_.g2_.x * other.g0_.w) - (self_.g3_.y * other.g0_.z) + (self_.g3_.z * other.g0_.y)), ((self_.g0_.x * other.g0_.y) - (self_.g2_.y * other.g0_.w) + (self_.g3_.x * other.g0_.z) - (self_.g3_.z * other.g0_.x)), ((self_.g0_.x * other.g0_.z) - (self_.g2_.z * other.g0_.w) - (self_.g3_.x * other.g0_.y) + (self_.g3_.y * other.g0_.x)), (self_.g0_.x * other.g0_.w)));
}
fn multiVector_geometricProduct_point(self_: MultiVector, other: Point) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(((self_.g1_.x * other.g0_.x) + (self_.g1_.y * other.g0_.y) + (self_.g1_.z * other.g0_.z)), (-(self_.g4_.x * other.g0_.x) - (self_.g4_.y * other.g0_.y) - (self_.g4_.z * other.g0_.z) - (self_.g4_.w * other.g0_.w))), /* e1, e2, e3, e4 */ (vec4<f32>(((self_.g3_.y * other.g0_.z) * -1.0), ((self_.g3_.z * other.g0_.x) * -1.0), ((self_.g3_.x * other.g0_.y) * -1.0), ((self_.g2_.y * other.g0_.y) + (self_.g2_.z * other.g0_.z))) + (vec4<f32>(self_.g0_.x) * other.g0_) + (vec4<f32>(self_.g3_.z, self_.g3_.x, self_.g3_.y, self_.g2_.x) * other.g0_.yzxx)), /* e41, e42, e43 */ (vec3<f32>((-(self_.g4_.y * other.g0_.z) + (self_.g4_.z * other.g0_.y)), ((self_.g4_.x * other.g0_.z) - (self_.g4_.z * other.g0_.x)), (-(self_.g4_.x * other.g0_.y) + (self_.g4_.y * other.g0_.x))) + (vec3<f32>(self_.g1_.w) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z)) - (vec3<f32>(other.g0_.w) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z))), /* e23, e31, e12 */ (vec3<f32>(((self_.g1_.y * other.g0_.z) - (self_.g1_.z * other.g0_.y)), (-(self_.g1_.x * other.g0_.z) + (self_.g1_.z * other.g0_.x)), ((self_.g1_.x * other.g0_.y) - (self_.g1_.y * other.g0_.x))) - (vec3<f32>(self_.g4_.w) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z))), /* e423, e431, e412, e321 */ (vec4<f32>(((self_.g2_.y * other.g0_.z) + (self_.g3_.x * other.g0_.w)), ((self_.g2_.z * other.g0_.x) + (self_.g3_.y * other.g0_.w)), ((self_.g2_.x * other.g0_.y) + (self_.g3_.z * other.g0_.w)), ((self_.g3_.z * other.g0_.z) * -1.0)) - (vec4<f32>(self_.g0_.y, self_.g0_.y, self_.g0_.y, self_.g3_.x) * other.g0_.xyzx) - (vec4<f32>(self_.g2_.z, self_.g2_.x, self_.g2_.y, self_.g3_.y) * other.g0_.yzxy)));
}
fn multiVector_geometricProduct_scalar(self_: MultiVector, other: Scalar) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(other.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(other.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(other.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(other.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_) * self_.g4_));
}
fn origin_geometricProduct_dualNum(self_: Origin, other: DualNum) -> Origin {
    return Origin(/* e4 */ (other.g0_.x * self_.g0_));
}
fn origin_geometricProduct_flector(self_: Origin, other: Flector) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.w)), /* e23, e31, e12, scalar */ vec4<f32>(0.0));
}
fn origin_geometricProduct_horizon(self_: Origin, other: Horizon) -> AntiScalar {
    return AntiScalar(/* e1234 */ (other.g0_ * self_.g0_));
}
fn origin_geometricProduct_line(self_: Origin, other: Line) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_), 0.0));
}
fn origin_geometricProduct_motor(self_: Origin, other: Motor) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g1_.w * self_.g0_)), /* e423, e431, e412, e321 */ vec4<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_), 0.0));
}
fn origin_geometricProduct_multiVector(self_: Origin, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (other.g4_.w * self_.g0_)), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.x * self_.g0_)), /* e41, e42, e43 */ (vec3<f32>(self_.g0_) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z)), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>((other.g3_.x * self_.g0_), (other.g3_.y * self_.g0_), (other.g3_.z * self_.g0_), 0.0));
}
fn origin_geometricProduct_plane(self_: Origin, other: Plane) -> AntiScalar {
    return AntiScalar(/* e1234 */ (other.g0_.w * self_.g0_));
}
fn origin_geometricProduct_point(self_: Origin, other: Point) -> Line {
    return Line(/* e41, e42, e43 */ (vec3<f32>(self_.g0_) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z)), /* e23, e31, e12 */ vec3<f32>(0.0));
}
fn origin_geometricProduct_scalar(self_: Origin, other: Scalar) -> Origin {
    return Origin(/* e4 */ (self_.g0_ * other.g0_));
}
fn plane_geometricProduct_antiScalar(self_: Plane, other: AntiScalar) -> Origin {
    return Origin(/* e4 */ (self_.g0_.w * other.g0_));
}
fn plane_geometricProduct_dualNum(self_: Plane, other: DualNum) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.y * self_.g0_.w)), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_.x) * self_.g0_));
}
fn plane_geometricProduct_flector(self_: Plane, other: Flector) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(((other.g0_.y * self_.g0_.z) + (other.g1_.w * self_.g0_.x)), ((other.g0_.z * self_.g0_.x) + (other.g1_.w * self_.g0_.y)), ((other.g0_.x * self_.g0_.y) + (other.g1_.w * self_.g0_.z)), (-(other.g0_.z * self_.g0_.z) - (other.g0_.w * self_.g0_.w))) - (vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, other.g0_.y) * self_.g0_.wwwy) - (other.g0_.zxyx * self_.g0_.yzxx)), /* e23, e31, e12, scalar */ (vec4<f32>(self_.g0_.w) * vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.w) * vec4<f32>(-1.0)));
}
fn plane_geometricProduct_horizon(self_: Plane, other: Horizon) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), 0.0), /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.w * other.g0_ * -1.0)));
}
fn plane_geometricProduct_line(self_: Plane, other: Line) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g1_.x * self_.g0_.w), (other.g1_.y * self_.g0_.w), (other.g1_.z * self_.g0_.w), (-(other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))), /* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.x * self_.g0_.w) + (other.g1_.y * self_.g0_.z) - (other.g1_.z * self_.g0_.y)), ((other.g0_.y * self_.g0_.w) - (other.g1_.x * self_.g0_.z) + (other.g1_.z * self_.g0_.x)), ((other.g0_.z * self_.g0_.w) + (other.g1_.x * self_.g0_.y) - (other.g1_.y * self_.g0_.x)), 0.0));
}
fn plane_geometricProduct_motor(self_: Plane, other: Motor) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g1_.x * self_.g0_.w), (other.g1_.y * self_.g0_.w), (other.g1_.z * self_.g0_.w), ((other.g0_.w * self_.g0_.w) - (other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))), /* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.x * self_.g0_.w) + (other.g1_.y * self_.g0_.z) - (other.g1_.z * self_.g0_.y) + (other.g1_.w * self_.g0_.x)), ((other.g0_.y * self_.g0_.w) - (other.g1_.x * self_.g0_.z) + (other.g1_.z * self_.g0_.x) + (other.g1_.w * self_.g0_.y)), ((other.g0_.z * self_.g0_.w) + (other.g1_.x * self_.g0_.y) - (other.g1_.y * self_.g0_.x) + (other.g1_.w * self_.g0_.z)), (other.g1_.w * self_.g0_.w)));
}
fn plane_geometricProduct_multiVector(self_: Plane, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((other.g4_.w * self_.g0_.w * -1.0), (-(other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z) - (other.g1_.w * self_.g0_.w))), /* e1, e2, e3, e4 */ vec4<f32>((other.g3_.x * self_.g0_.w), (other.g3_.y * self_.g0_.w), (other.g3_.z * self_.g0_.w), ((other.g0_.y * self_.g0_.w) - (other.g3_.x * self_.g0_.x) - (other.g3_.y * self_.g0_.y) - (other.g3_.z * self_.g0_.z))), /* e41, e42, e43 */ (vec3<f32>(((other.g1_.y * self_.g0_.z) - (other.g1_.z * self_.g0_.y)), (-(other.g1_.x * self_.g0_.z) + (other.g1_.z * self_.g0_.x)), ((other.g1_.x * self_.g0_.y) - (other.g1_.y * self_.g0_.x))) + (vec3<f32>(other.g4_.w) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z)) - (vec3<f32>(self_.g0_.w) * vec3<f32>(other.g4_.x, other.g4_.y, other.g4_.z))), /* e23, e31, e12 */ (vec3<f32>(self_.g0_.w) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z) * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.x * self_.g0_.x) + (other.g2_.x * self_.g0_.w) + (other.g3_.y * self_.g0_.z) - (other.g3_.z * self_.g0_.y)), ((other.g0_.x * self_.g0_.y) + (other.g2_.y * self_.g0_.w) - (other.g3_.x * self_.g0_.z) + (other.g3_.z * self_.g0_.x)), ((other.g0_.x * self_.g0_.z) + (other.g2_.z * self_.g0_.w) + (other.g3_.x * self_.g0_.y) - (other.g3_.y * self_.g0_.x)), (other.g0_.x * self_.g0_.w)));
}
fn plane_geometricProduct_origin(self_: Plane, other: Origin) -> AntiScalar {
    return AntiScalar(/* e1234 */ (self_.g0_.w * other.g0_ * -1.0));
}
fn plane_geometricProduct_plane(self_: Plane, other: Plane) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((-(other.g0_.x * self_.g0_.w) + (other.g0_.w * self_.g0_.x)), (-(other.g0_.y * self_.g0_.w) + (other.g0_.w * self_.g0_.y)), (-(other.g0_.z * self_.g0_.w) + (other.g0_.w * self_.g0_.z)), 0.0), /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.w * self_.g0_.w * -1.0)));
}
fn plane_geometricProduct_point(self_: Plane, other: Point) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>((self_.g0_.z * other.g0_.y), (self_.g0_.x * other.g0_.z), (self_.g0_.y * other.g0_.x), (-(self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z) - (self_.g0_.w * other.g0_.w))) - (self_.g0_.yzxx * other.g0_.zxyx)), /* e23, e31, e12, scalar */ vec4<f32>((self_.g0_.w * other.g0_.x * -1.0), (self_.g0_.w * other.g0_.y * -1.0), (self_.g0_.w * other.g0_.z * -1.0), 0.0));
}
fn plane_geometricProduct_scalar(self_: Plane, other: Scalar) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ (vec4<f32>(other.g0_) * self_.g0_));
}
fn point_geometricProduct_antiScalar(self_: Point, other: AntiScalar) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), 0.0));
}
fn point_geometricProduct_dualNum(self_: Point, other: DualNum) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(other.g0_.x) * self_.g0_), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.y * self_.g0_.x), (other.g0_.y * self_.g0_.y), (other.g0_.y * self_.g0_.z), 0.0));
}
fn point_geometricProduct_flector(self_: Point, other: Flector) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>((-(other.g0_.w * self_.g0_.x) - (other.g1_.y * self_.g0_.z)), (-(other.g0_.w * self_.g0_.y) - (other.g1_.z * self_.g0_.x)), (-(other.g0_.w * self_.g0_.z) - (other.g1_.x * self_.g0_.y)), ((other.g1_.z * self_.g0_.z) + (other.g1_.w * self_.g0_.w))) + (vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.x) * self_.g0_.wwwx) + (other.g1_.zxyy * self_.g0_.yzxy)), /* e23, e31, e12, scalar */ (vec4<f32>((-(other.g0_.y * self_.g0_.z) - (other.g1_.w * self_.g0_.x)), (-(other.g0_.z * self_.g0_.x) - (other.g1_.w * self_.g0_.y)), (-(other.g0_.x * self_.g0_.y) - (other.g1_.w * self_.g0_.z)), ((other.g0_.y * self_.g0_.y) + (other.g0_.z * self_.g0_.z))) + (other.g0_.zxyx * self_.g0_.yzxx)));
}
fn point_geometricProduct_horizon(self_: Point, other: Horizon) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.w * other.g0_)), /* e23, e31, e12, scalar */ vec4<f32>((self_.g0_.x * other.g0_ * -1.0), (self_.g0_.y * other.g0_ * -1.0), (self_.g0_.z * other.g0_ * -1.0), 0.0));
}
fn point_geometricProduct_line(self_: Point, other: Line) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>((other.g1_.y * self_.g0_.z), (other.g1_.z * self_.g0_.x), (other.g1_.x * self_.g0_.y), (-(other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z))) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.x) * self_.g0_.yzxx)), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.y * self_.g0_.z) + (other.g1_.x * self_.g0_.w)), ((other.g0_.z * self_.g0_.x) + (other.g1_.y * self_.g0_.w)), ((other.g0_.x * self_.g0_.y) + (other.g1_.z * self_.g0_.w)), (-(other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))) - (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.y, other.g1_.x) * self_.g0_.yzxx)));
}
fn point_geometricProduct_motor(self_: Point, other: Motor) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>((other.g1_.w * self_.g0_.x), (other.g1_.w * self_.g0_.y), (other.g1_.w * self_.g0_.z), (-(other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z))) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.x) * self_.g0_.yzxx) + (other.g1_.yzxw * self_.g0_.zxyw)), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.y * self_.g0_.z) + (other.g0_.w * self_.g0_.x) + (other.g1_.x * self_.g0_.w)), ((other.g0_.z * self_.g0_.x) + (other.g0_.w * self_.g0_.y) + (other.g1_.y * self_.g0_.w)), ((other.g0_.x * self_.g0_.y) + (other.g0_.w * self_.g0_.z) + (other.g1_.z * self_.g0_.w)), (-(other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))) - (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.y, other.g1_.x) * self_.g0_.yzxx)));
}
fn point_geometricProduct_multiVector(self_: Point, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(0.0, (other.g4_.w * self_.g0_.w)) + (vec2<f32>(self_.g0_.x) * vec2<f32>(other.g1_.x, other.g4_.x)) + (vec2<f32>(self_.g0_.y) * vec2<f32>(other.g1_.y, other.g4_.y)) + (vec2<f32>(self_.g0_.z) * vec2<f32>(other.g1_.z, other.g4_.z))), /* e1, e2, e3, e4 */ (vec4<f32>((other.g3_.y * self_.g0_.z), (other.g3_.z * self_.g0_.x), (other.g3_.x * self_.g0_.y), (-(other.g2_.y * self_.g0_.y) - (other.g2_.z * self_.g0_.z))) + (vec4<f32>(other.g0_.x) * self_.g0_) - (vec4<f32>(other.g3_.z, other.g3_.x, other.g3_.y, other.g2_.x) * self_.g0_.yzxx)), /* e41, e42, e43 */ (vec3<f32>((-(other.g4_.y * self_.g0_.z) + (other.g4_.z * self_.g0_.y)), ((other.g4_.x * self_.g0_.z) - (other.g4_.z * self_.g0_.x)), (-(other.g4_.x * self_.g0_.y) + (other.g4_.y * self_.g0_.x))) - (vec3<f32>(other.g1_.w) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z)) + (vec3<f32>(self_.g0_.w) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z))), /* e23, e31, e12 */ (vec3<f32>((-(other.g1_.y * self_.g0_.z) + (other.g1_.z * self_.g0_.y)), ((other.g1_.x * self_.g0_.z) - (other.g1_.z * self_.g0_.x)), (-(other.g1_.x * self_.g0_.y) + (other.g1_.y * self_.g0_.x))) - (vec3<f32>(other.g4_.w) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z))), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.y * self_.g0_.x) + (other.g2_.y * self_.g0_.z) + (other.g3_.x * self_.g0_.w)), ((other.g0_.y * self_.g0_.y) + (other.g2_.z * self_.g0_.x) + (other.g3_.y * self_.g0_.w)), ((other.g0_.y * self_.g0_.z) + (other.g2_.x * self_.g0_.y) + (other.g3_.z * self_.g0_.w)), (-(other.g3_.y * self_.g0_.y) - (other.g3_.z * self_.g0_.z))) - (vec4<f32>(other.g2_.z, other.g2_.x, other.g2_.y, other.g3_.x) * self_.g0_.yzxx)));
}
fn point_geometricProduct_origin(self_: Point, other: Origin) -> Line {
    return Line(/* e41, e42, e43 */ (vec3<f32>(other.g0_) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z) * vec3<f32>(-1.0)), /* e23, e31, e12 */ vec3<f32>(0.0));
}
fn point_geometricProduct_plane(self_: Point, other: Plane) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(((other.g0_.y * self_.g0_.z) * -1.0), ((other.g0_.z * self_.g0_.x) * -1.0), ((other.g0_.x * self_.g0_.y) * -1.0), ((other.g0_.y * self_.g0_.y) + (other.g0_.z * self_.g0_.z) + (other.g0_.w * self_.g0_.w))) + (other.g0_.zxyx * self_.g0_.yzxx)), /* e23, e31, e12, scalar */ vec4<f32>((other.g0_.w * self_.g0_.x * -1.0), (other.g0_.w * self_.g0_.y * -1.0), (other.g0_.w * self_.g0_.z * -1.0), 0.0));
}
fn point_geometricProduct_point(self_: Point, other: Point) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(((other.g0_.x * self_.g0_.w) - (other.g0_.w * self_.g0_.x)), ((other.g0_.y * self_.g0_.w) - (other.g0_.w * self_.g0_.y)), ((other.g0_.z * self_.g0_.w) - (other.g0_.w * self_.g0_.z)), 0.0), /* e23, e31, e12, scalar */ (vec4<f32>(((other.g0_.y * self_.g0_.z) * -1.0), ((other.g0_.z * self_.g0_.x) * -1.0), ((other.g0_.x * self_.g0_.y) * -1.0), ((other.g0_.y * self_.g0_.y) + (other.g0_.z * self_.g0_.z))) + (other.g0_.zxyx * self_.g0_.yzxx)));
}
fn point_geometricProduct_scalar(self_: Point, other: Scalar) -> Point {
    return Point(/* e1, e2, e3, e4 */ (vec4<f32>(other.g0_) * self_.g0_));
}
fn scalar_geometricProduct_antiScalar(self_: Scalar, other: AntiScalar) -> AntiScalar {
    return AntiScalar(/* e1234 */ (other.g0_ * self_.g0_));
}
fn scalar_geometricProduct_dualNum(self_: Scalar, other: DualNum) -> DualNum {
    return DualNum(/* scalar, e1234 */ (vec2<f32>(self_.g0_) * other.g0_));
}
fn scalar_geometricProduct_flector(self_: Scalar, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_) * other.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_) * other.g1_));
}
fn scalar_geometricProduct_horizon(self_: Scalar, other: Horizon) -> Horizon {
    return Horizon(/* e321 */ (other.g0_ * self_.g0_));
}
fn scalar_geometricProduct_line(self_: Scalar, other: Line) -> Line {
    return Line(/* e41, e42, e43 */ (vec3<f32>(self_.g0_) * other.g0_), /* e23, e31, e12 */ (vec3<f32>(self_.g0_) * other.g1_));
}
fn scalar_geometricProduct_motor(self_: Scalar, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(self_.g0_) * other.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(self_.g0_) * other.g1_));
}
fn scalar_geometricProduct_multiVector(self_: Scalar, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(self_.g0_) * other.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_) * other.g1_), /* e41, e42, e43 */ (vec3<f32>(self_.g0_) * other.g2_), /* e23, e31, e12 */ (vec3<f32>(self_.g0_) * other.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_) * other.g4_));
}
fn scalar_geometricProduct_origin(self_: Scalar, other: Origin) -> Origin {
    return Origin(/* e4 */ (other.g0_ * self_.g0_));
}
fn scalar_geometricProduct_plane(self_: Scalar, other: Plane) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_) * other.g0_));
}
fn scalar_geometricProduct_point(self_: Scalar, other: Point) -> Point {
    return Point(/* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_) * other.g0_));
}
fn scalar_geometricProduct_scalar(self_: Scalar, other: Scalar) -> Scalar {
    return Scalar(/* scalar */ (other.g0_ * self_.g0_));
}
fn antiScalar_geometricQuotient_dualNum(self_: AntiScalar, other: DualNum) -> AntiScalar {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_.x, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return AntiScalar(/* e1234 */ (self_.g0_ * inverse.g0_));
}
fn antiScalar_geometricQuotient_flector(self_: AntiScalar, other: Flector) -> AntiScalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2) - pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return AntiScalar(/* e1234 */ (self_.g0_ * inverse.g0_));
}
fn antiScalar_geometricQuotient_horizon(self_: AntiScalar, other: Horizon) -> AntiScalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return AntiScalar(/* e1234 */ (self_.g0_ * inverse.g0_));
}
fn antiScalar_geometricQuotient_line(self_: AntiScalar, other: Line) -> AntiScalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return AntiScalar(/* e1234 */ (self_.g0_ * inverse.g0_));
}
fn antiScalar_geometricQuotient_motor(self_: AntiScalar, other: Motor) -> AntiScalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2) + pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return AntiScalar(/* e1234 */ (self_.g0_ * inverse.g0_));
}
fn antiScalar_geometricQuotient_multiVector(self_: AntiScalar, other: MultiVector) -> AntiScalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) - pow(other.g3_.x, 2) - pow(other.g3_.y, 2) - pow(other.g3_.z, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2) - pow(other.g4_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return AntiScalar(/* e1234 */ (self_.g0_ * inverse.g0_));
}
fn antiScalar_geometricQuotient_plane(self_: AntiScalar, other: Plane) -> AntiScalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.w, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return AntiScalar(/* e1234 */ (self_.g0_ * inverse.g0_));
}
fn antiScalar_geometricQuotient_point(self_: AntiScalar, other: Point) -> AntiScalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return AntiScalar(/* e1234 */ (self_.g0_ * inverse.g0_));
}
fn antiScalar_geometricQuotient_scalar(self_: AntiScalar, other: Scalar) -> AntiScalar {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return AntiScalar(/* e1234 */ (self_.g0_ * inverse.g0_));
}
fn dualNum_geometricQuotient_dualNum(self_: DualNum, other: DualNum) -> DualNum {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_.x, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return DualNum(/* scalar, e1234 */ (vec2<f32>(inverse.g0_) * self_.g0_));
}
fn dualNum_geometricQuotient_flector(self_: DualNum, other: Flector) -> DualNum {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2) - pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return DualNum(/* scalar, e1234 */ (vec2<f32>(inverse.g0_) * self_.g0_));
}
fn dualNum_geometricQuotient_horizon(self_: DualNum, other: Horizon) -> DualNum {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return DualNum(/* scalar, e1234 */ (vec2<f32>(inverse.g0_) * self_.g0_));
}
fn dualNum_geometricQuotient_line(self_: DualNum, other: Line) -> DualNum {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return DualNum(/* scalar, e1234 */ (vec2<f32>(inverse.g0_) * self_.g0_));
}
fn dualNum_geometricQuotient_motor(self_: DualNum, other: Motor) -> DualNum {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2) + pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return DualNum(/* scalar, e1234 */ (vec2<f32>(inverse.g0_) * self_.g0_));
}
fn dualNum_geometricQuotient_multiVector(self_: DualNum, other: MultiVector) -> DualNum {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) - pow(other.g3_.x, 2) - pow(other.g3_.y, 2) - pow(other.g3_.z, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2) - pow(other.g4_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return DualNum(/* scalar, e1234 */ (vec2<f32>(inverse.g0_) * self_.g0_));
}
fn dualNum_geometricQuotient_plane(self_: DualNum, other: Plane) -> DualNum {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.w, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return DualNum(/* scalar, e1234 */ (vec2<f32>(inverse.g0_) * self_.g0_));
}
fn dualNum_geometricQuotient_point(self_: DualNum, other: Point) -> DualNum {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return DualNum(/* scalar, e1234 */ (vec2<f32>(inverse.g0_) * self_.g0_));
}
fn dualNum_geometricQuotient_scalar(self_: DualNum, other: Scalar) -> DualNum {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return DualNum(/* scalar, e1234 */ (vec2<f32>(inverse.g0_) * self_.g0_));
}
fn flector_geometricQuotient_dualNum(self_: Flector, other: DualNum) -> Flector {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_.x, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g1_));
}
fn flector_geometricQuotient_flector(self_: Flector, other: Flector) -> Flector {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2) - pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g1_));
}
fn flector_geometricQuotient_horizon(self_: Flector, other: Horizon) -> Flector {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g1_));
}
fn flector_geometricQuotient_line(self_: Flector, other: Line) -> Flector {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g1_));
}
fn flector_geometricQuotient_motor(self_: Flector, other: Motor) -> Flector {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2) + pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g1_));
}
fn flector_geometricQuotient_multiVector(self_: Flector, other: MultiVector) -> Flector {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) - pow(other.g3_.x, 2) - pow(other.g3_.y, 2) - pow(other.g3_.z, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2) - pow(other.g4_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g1_));
}
fn flector_geometricQuotient_plane(self_: Flector, other: Plane) -> Flector {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.w, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g1_));
}
fn flector_geometricQuotient_point(self_: Flector, other: Point) -> Flector {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g1_));
}
fn flector_geometricQuotient_scalar(self_: Flector, other: Scalar) -> Flector {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g1_));
}
fn horizon_geometricQuotient_dualNum(self_: Horizon, other: DualNum) -> Horizon {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_.x, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Horizon(/* e321 */ (self_.g0_ * inverse.g0_));
}
fn horizon_geometricQuotient_flector(self_: Horizon, other: Flector) -> Horizon {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2) - pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Horizon(/* e321 */ (self_.g0_ * inverse.g0_));
}
fn horizon_geometricQuotient_horizon(self_: Horizon, other: Horizon) -> Horizon {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Horizon(/* e321 */ (self_.g0_ * inverse.g0_));
}
fn horizon_geometricQuotient_line(self_: Horizon, other: Line) -> Horizon {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Horizon(/* e321 */ (self_.g0_ * inverse.g0_));
}
fn horizon_geometricQuotient_motor(self_: Horizon, other: Motor) -> Horizon {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2) + pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Horizon(/* e321 */ (self_.g0_ * inverse.g0_));
}
fn horizon_geometricQuotient_multiVector(self_: Horizon, other: MultiVector) -> Horizon {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) - pow(other.g3_.x, 2) - pow(other.g3_.y, 2) - pow(other.g3_.z, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2) - pow(other.g4_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Horizon(/* e321 */ (self_.g0_ * inverse.g0_));
}
fn horizon_geometricQuotient_plane(self_: Horizon, other: Plane) -> Horizon {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.w, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Horizon(/* e321 */ (self_.g0_ * inverse.g0_));
}
fn horizon_geometricQuotient_point(self_: Horizon, other: Point) -> Horizon {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Horizon(/* e321 */ (self_.g0_ * inverse.g0_));
}
fn horizon_geometricQuotient_scalar(self_: Horizon, other: Scalar) -> Horizon {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Horizon(/* e321 */ (self_.g0_ * inverse.g0_));
}
fn line_geometricQuotient_dualNum(self_: Line, other: DualNum) -> Line {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_.x, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Line(/* e41, e42, e43 */ (vec3<f32>(inverse.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(inverse.g0_) * self_.g1_));
}
fn line_geometricQuotient_flector(self_: Line, other: Flector) -> Line {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2) - pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Line(/* e41, e42, e43 */ (vec3<f32>(inverse.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(inverse.g0_) * self_.g1_));
}
fn line_geometricQuotient_horizon(self_: Line, other: Horizon) -> Line {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Line(/* e41, e42, e43 */ (vec3<f32>(inverse.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(inverse.g0_) * self_.g1_));
}
fn line_geometricQuotient_line(self_: Line, other: Line) -> Line {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Line(/* e41, e42, e43 */ (vec3<f32>(inverse.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(inverse.g0_) * self_.g1_));
}
fn line_geometricQuotient_motor(self_: Line, other: Motor) -> Line {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2) + pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Line(/* e41, e42, e43 */ (vec3<f32>(inverse.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(inverse.g0_) * self_.g1_));
}
fn line_geometricQuotient_multiVector(self_: Line, other: MultiVector) -> Line {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) - pow(other.g3_.x, 2) - pow(other.g3_.y, 2) - pow(other.g3_.z, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2) - pow(other.g4_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Line(/* e41, e42, e43 */ (vec3<f32>(inverse.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(inverse.g0_) * self_.g1_));
}
fn line_geometricQuotient_plane(self_: Line, other: Plane) -> Line {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.w, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Line(/* e41, e42, e43 */ (vec3<f32>(inverse.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(inverse.g0_) * self_.g1_));
}
fn line_geometricQuotient_point(self_: Line, other: Point) -> Line {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Line(/* e41, e42, e43 */ (vec3<f32>(inverse.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(inverse.g0_) * self_.g1_));
}
fn line_geometricQuotient_scalar(self_: Line, other: Scalar) -> Line {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Line(/* e41, e42, e43 */ (vec3<f32>(inverse.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(inverse.g0_) * self_.g1_));
}
fn motor_geometricQuotient_dualNum(self_: Motor, other: DualNum) -> Motor {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_.x, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(inverse.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(inverse.g0_) * self_.g1_));
}
fn motor_geometricQuotient_flector(self_: Motor, other: Flector) -> Motor {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2) - pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(inverse.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(inverse.g0_) * self_.g1_));
}
fn motor_geometricQuotient_horizon(self_: Motor, other: Horizon) -> Motor {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(inverse.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(inverse.g0_) * self_.g1_));
}
fn motor_geometricQuotient_line(self_: Motor, other: Line) -> Motor {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(inverse.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(inverse.g0_) * self_.g1_));
}
fn motor_geometricQuotient_motor(self_: Motor, other: Motor) -> Motor {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2) + pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(inverse.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(inverse.g0_) * self_.g1_));
}
fn motor_geometricQuotient_multiVector(self_: Motor, other: MultiVector) -> Motor {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) - pow(other.g3_.x, 2) - pow(other.g3_.y, 2) - pow(other.g3_.z, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2) - pow(other.g4_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(inverse.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(inverse.g0_) * self_.g1_));
}
fn motor_geometricQuotient_plane(self_: Motor, other: Plane) -> Motor {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.w, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(inverse.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(inverse.g0_) * self_.g1_));
}
fn motor_geometricQuotient_point(self_: Motor, other: Point) -> Motor {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(inverse.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(inverse.g0_) * self_.g1_));
}
fn motor_geometricQuotient_scalar(self_: Motor, other: Scalar) -> Motor {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(inverse.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(inverse.g0_) * self_.g1_));
}
fn multiVector_geometricQuotient_dualNum(self_: MultiVector, other: DualNum) -> MultiVector {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_.x, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(inverse.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(inverse.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(inverse.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g4_));
}
fn multiVector_geometricQuotient_flector(self_: MultiVector, other: Flector) -> MultiVector {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2) - pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(inverse.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(inverse.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(inverse.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g4_));
}
fn multiVector_geometricQuotient_horizon(self_: MultiVector, other: Horizon) -> MultiVector {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(inverse.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(inverse.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(inverse.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g4_));
}
fn multiVector_geometricQuotient_line(self_: MultiVector, other: Line) -> MultiVector {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(inverse.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(inverse.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(inverse.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g4_));
}
fn multiVector_geometricQuotient_motor(self_: MultiVector, other: Motor) -> MultiVector {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2) + pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(inverse.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(inverse.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(inverse.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g4_));
}
fn multiVector_geometricQuotient_multiVector(self_: MultiVector, other: MultiVector) -> MultiVector {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) - pow(other.g3_.x, 2) - pow(other.g3_.y, 2) - pow(other.g3_.z, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2) - pow(other.g4_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(inverse.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(inverse.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(inverse.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g4_));
}
fn multiVector_geometricQuotient_plane(self_: MultiVector, other: Plane) -> MultiVector {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.w, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(inverse.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(inverse.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(inverse.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g4_));
}
fn multiVector_geometricQuotient_point(self_: MultiVector, other: Point) -> MultiVector {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(inverse.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(inverse.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(inverse.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g4_));
}
fn multiVector_geometricQuotient_scalar(self_: MultiVector, other: Scalar) -> MultiVector {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(inverse.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(inverse.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(inverse.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g4_));
}
fn origin_geometricQuotient_dualNum(self_: Origin, other: DualNum) -> Origin {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_.x, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Origin(/* e4 */ (self_.g0_ * inverse.g0_));
}
fn origin_geometricQuotient_flector(self_: Origin, other: Flector) -> Origin {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2) - pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Origin(/* e4 */ (self_.g0_ * inverse.g0_));
}
fn origin_geometricQuotient_horizon(self_: Origin, other: Horizon) -> Origin {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Origin(/* e4 */ (self_.g0_ * inverse.g0_));
}
fn origin_geometricQuotient_line(self_: Origin, other: Line) -> Origin {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Origin(/* e4 */ (self_.g0_ * inverse.g0_));
}
fn origin_geometricQuotient_motor(self_: Origin, other: Motor) -> Origin {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2) + pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Origin(/* e4 */ (self_.g0_ * inverse.g0_));
}
fn origin_geometricQuotient_multiVector(self_: Origin, other: MultiVector) -> Origin {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) - pow(other.g3_.x, 2) - pow(other.g3_.y, 2) - pow(other.g3_.z, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2) - pow(other.g4_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Origin(/* e4 */ (self_.g0_ * inverse.g0_));
}
fn origin_geometricQuotient_plane(self_: Origin, other: Plane) -> Origin {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.w, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Origin(/* e4 */ (self_.g0_ * inverse.g0_));
}
fn origin_geometricQuotient_point(self_: Origin, other: Point) -> Origin {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Origin(/* e4 */ (self_.g0_ * inverse.g0_));
}
fn origin_geometricQuotient_scalar(self_: Origin, other: Scalar) -> Origin {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Origin(/* e4 */ (self_.g0_ * inverse.g0_));
}
fn plane_geometricQuotient_dualNum(self_: Plane, other: DualNum) -> Plane {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_.x, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Plane(/* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g0_));
}
fn plane_geometricQuotient_flector(self_: Plane, other: Flector) -> Plane {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2) - pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Plane(/* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g0_));
}
fn plane_geometricQuotient_horizon(self_: Plane, other: Horizon) -> Plane {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Plane(/* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g0_));
}
fn plane_geometricQuotient_line(self_: Plane, other: Line) -> Plane {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Plane(/* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g0_));
}
fn plane_geometricQuotient_motor(self_: Plane, other: Motor) -> Plane {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2) + pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Plane(/* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g0_));
}
fn plane_geometricQuotient_multiVector(self_: Plane, other: MultiVector) -> Plane {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) - pow(other.g3_.x, 2) - pow(other.g3_.y, 2) - pow(other.g3_.z, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2) - pow(other.g4_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Plane(/* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g0_));
}
fn plane_geometricQuotient_plane(self_: Plane, other: Plane) -> Plane {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.w, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Plane(/* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g0_));
}
fn plane_geometricQuotient_point(self_: Plane, other: Point) -> Plane {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Plane(/* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g0_));
}
fn plane_geometricQuotient_scalar(self_: Plane, other: Scalar) -> Plane {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Plane(/* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g0_));
}
fn point_geometricQuotient_dualNum(self_: Point, other: DualNum) -> Point {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_.x, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Point(/* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g0_));
}
fn point_geometricQuotient_flector(self_: Point, other: Flector) -> Point {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2) - pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Point(/* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g0_));
}
fn point_geometricQuotient_horizon(self_: Point, other: Horizon) -> Point {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Point(/* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g0_));
}
fn point_geometricQuotient_line(self_: Point, other: Line) -> Point {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Point(/* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g0_));
}
fn point_geometricQuotient_motor(self_: Point, other: Motor) -> Point {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2) + pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Point(/* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g0_));
}
fn point_geometricQuotient_multiVector(self_: Point, other: MultiVector) -> Point {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) - pow(other.g3_.x, 2) - pow(other.g3_.y, 2) - pow(other.g3_.z, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2) - pow(other.g4_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Point(/* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g0_));
}
fn point_geometricQuotient_plane(self_: Point, other: Plane) -> Point {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.w, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Point(/* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g0_));
}
fn point_geometricQuotient_point(self_: Point, other: Point) -> Point {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Point(/* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g0_));
}
fn point_geometricQuotient_scalar(self_: Point, other: Scalar) -> Point {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Point(/* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g0_));
}
fn scalar_geometricQuotient_dualNum(self_: Scalar, other: DualNum) -> Scalar {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_.x, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Scalar(/* scalar */ (inverse.g0_ * self_.g0_));
}
fn scalar_geometricQuotient_flector(self_: Scalar, other: Flector) -> Scalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2) - pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Scalar(/* scalar */ (inverse.g0_ * self_.g0_));
}
fn scalar_geometricQuotient_horizon(self_: Scalar, other: Horizon) -> Scalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Scalar(/* scalar */ (inverse.g0_ * self_.g0_));
}
fn scalar_geometricQuotient_line(self_: Scalar, other: Line) -> Scalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Scalar(/* scalar */ (inverse.g0_ * self_.g0_));
}
fn scalar_geometricQuotient_motor(self_: Scalar, other: Motor) -> Scalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2) + pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Scalar(/* scalar */ (inverse.g0_ * self_.g0_));
}
fn scalar_geometricQuotient_multiVector(self_: Scalar, other: MultiVector) -> Scalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) - pow(other.g3_.x, 2) - pow(other.g3_.y, 2) - pow(other.g3_.z, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2) - pow(other.g4_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Scalar(/* scalar */ (inverse.g0_ * self_.g0_));
}
fn scalar_geometricQuotient_plane(self_: Scalar, other: Plane) -> Scalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.w, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Scalar(/* scalar */ (inverse.g0_ * self_.g0_));
}
fn scalar_geometricQuotient_point(self_: Scalar, other: Point) -> Scalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Scalar(/* scalar */ (inverse.g0_ * self_.g0_));
}
fn scalar_geometricQuotient_scalar(self_: Scalar, other: Scalar) -> Scalar {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    return Scalar(/* scalar */ (inverse.g0_ * self_.g0_));
}
fn antiScalar_grade() -> i32 {
    return 4;
}
fn horizon_grade() -> i32 {
    return 3;
}
fn line_grade() -> i32 {
    return 2;
}
fn origin_grade() -> i32 {
    return 1;
}
fn plane_grade() -> i32 {
    return 3;
}
fn point_grade() -> i32 {
    return 1;
}
fn scalar_grade() -> i32 {
    return 0;
}
fn antiScalar_into(self_: AntiScalar) -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>(0.0, self_.g0_.x));
}
fn antiScalar_into(self_: AntiScalar) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_.x), /* e23, e31, e12, scalar */ vec4<f32>(0.0));
}
fn antiScalar_into(self_: AntiScalar) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, self_.g0_.x), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn dualNum_into(self_: DualNum) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_.y), /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_.x));
}
fn dualNum_into(self_: DualNum) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g0_.x, self_.g0_.y), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn flector_into(self_: Flector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g0_.w), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g1_.w));
}
fn horizon_into(self_: Horizon) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_.x));
}
fn horizon_into(self_: Horizon) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_.x));
}
fn horizon_into(self_: Horizon) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_.x));
}
fn line_into(self_: Line) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, 0.0), /* e23, e31, e12, scalar */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, 0.0));
}
fn line_into(self_: Line) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z), /* e23, e31, e12 */ vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn motor_into(self_: Motor) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g1_.w, self_.g0_.w), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z), /* e23, e31, e12 */ vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn origin_into(self_: Origin) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_.x), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn origin_into(self_: Origin) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_.x), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn origin_into(self_: Origin) -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_.x));
}
fn plane_into(self_: Plane) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g0_.w));
}
fn plane_into(self_: Plane) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g0_.w));
}
fn point_into(self_: Point) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g0_.w), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn point_into(self_: Point) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g0_.w), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn scalar_into(self_: Scalar) -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>(self_.g0_.x, 0.0));
}
fn scalar_into(self_: Scalar) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0), /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_.x));
}
fn scalar_into(self_: Scalar) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g0_.x, 0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn dualNum_inverse(self_: DualNum) -> Scalar {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(self_.g0_.x, 2));
    return Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
}
fn flector_inverse(self_: Flector) -> Scalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(self_.g0_.x, 2) + pow(self_.g0_.y, 2) + pow(self_.g0_.z, 2) - pow(self_.g1_.w, 2)));
    return Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
}
fn horizon_inverse(self_: Horizon) -> Scalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(self_.g0_, 2) * -1.0));
    return Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
}
fn line_inverse(self_: Line) -> Scalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(self_.g1_.x, 2) - pow(self_.g1_.y, 2) - pow(self_.g1_.z, 2)));
    return Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
}
fn motor_inverse(self_: Motor) -> Scalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(self_.g1_.x, 2) - pow(self_.g1_.y, 2) - pow(self_.g1_.z, 2) + pow(self_.g1_.w, 2)));
    return Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
}
fn multiVector_inverse(self_: MultiVector) -> Scalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(self_.g0_.x, 2) - pow(self_.g3_.x, 2) - pow(self_.g3_.y, 2) - pow(self_.g3_.z, 2) + pow(self_.g1_.x, 2) + pow(self_.g1_.y, 2) + pow(self_.g1_.z, 2) - pow(self_.g4_.w, 2)));
    return Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
}
fn plane_inverse(self_: Plane) -> Scalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(self_.g0_.w, 2) * -1.0));
    return Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
}
fn point_inverse(self_: Point) -> Scalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(self_.g0_.x, 2) + pow(self_.g0_.y, 2) + pow(self_.g0_.z, 2)));
    return Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
}
fn scalar_inverse(self_: Scalar) -> Scalar {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(self_.g0_, 2));
    return Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
}
fn antiScalar_mul_dualNum(self_: AntiScalar, other: DualNum) -> AntiScalar {
    return antiScalar_geometricProduct_antiScalar(self_, other);
}
fn antiScalar_mul_flector(self_: AntiScalar, other: Flector) -> Flector {
    return antiScalar_geometricProduct_antiScalar(self_, other);
}
fn antiScalar_mul_horizon(self_: AntiScalar, other: Horizon) -> Origin {
    return antiScalar_geometricProduct_antiScalar(self_, other);
}
fn antiScalar_mul_line(self_: AntiScalar, other: Line) -> Line {
    return antiScalar_geometricProduct_antiScalar(self_, other);
}
fn antiScalar_mul_motor(self_: AntiScalar, other: Motor) -> Motor {
    return antiScalar_geometricProduct_antiScalar(self_, other);
}
fn antiScalar_mul_multiVector(self_: AntiScalar, other: MultiVector) -> MultiVector {
    return antiScalar_geometricProduct_antiScalar(self_, other);
}
fn antiScalar_mul_plane(self_: AntiScalar, other: Plane) -> Origin {
    return antiScalar_geometricProduct_antiScalar(self_, other);
}
fn antiScalar_mul_point(self_: AntiScalar, other: Point) -> Plane {
    return antiScalar_geometricProduct_antiScalar(self_, other);
}
fn antiScalar_mul_scalar(self_: AntiScalar, other: Scalar) -> AntiScalar {
    return antiScalar_geometricProduct_antiScalar(self_, other);
}
fn dualNum_mul_antiScalar(self_: DualNum, other: AntiScalar) -> AntiScalar {
    return dualNum_geometricProduct_dualNum(self_, other);
}
fn dualNum_mul_dualNum(self_: DualNum, other: DualNum) -> DualNum {
    return dualNum_geometricProduct_dualNum(self_, other);
}
fn dualNum_mul_flector(self_: DualNum, other: Flector) -> Flector {
    return dualNum_geometricProduct_dualNum(self_, other);
}
fn dualNum_mul_horizon(self_: DualNum, other: Horizon) -> Flector {
    return dualNum_geometricProduct_dualNum(self_, other);
}
fn dualNum_mul_line(self_: DualNum, other: Line) -> Line {
    return dualNum_geometricProduct_dualNum(self_, other);
}
fn dualNum_mul_motor(self_: DualNum, other: Motor) -> Motor {
    return dualNum_geometricProduct_dualNum(self_, other);
}
fn dualNum_mul_multiVector(self_: DualNum, other: MultiVector) -> MultiVector {
    return dualNum_geometricProduct_dualNum(self_, other);
}
fn dualNum_mul_origin(self_: DualNum, other: Origin) -> Origin {
    return dualNum_geometricProduct_dualNum(self_, other);
}
fn dualNum_mul_plane(self_: DualNum, other: Plane) -> Flector {
    return dualNum_geometricProduct_dualNum(self_, other);
}
fn dualNum_mul_point(self_: DualNum, other: Point) -> Flector {
    return dualNum_geometricProduct_dualNum(self_, other);
}
fn dualNum_mul_scalar(self_: DualNum, other: Scalar) -> DualNum {
    return dualNum_geometricProduct_dualNum(self_, other);
}
fn flector_mul_antiScalar(self_: Flector, other: AntiScalar) -> Flector {
    return flector_geometricProduct_flector(self_, other);
}
fn flector_mul_dualNum(self_: Flector, other: DualNum) -> Flector {
    return flector_geometricProduct_flector(self_, other);
}
fn flector_mul_flector(self_: Flector, other: Flector) -> Motor {
    return flector_geometricProduct_flector(self_, other);
}
fn flector_mul_horizon(self_: Flector, other: Horizon) -> Motor {
    return flector_geometricProduct_flector(self_, other);
}
fn flector_mul_line(self_: Flector, other: Line) -> Flector {
    return flector_geometricProduct_flector(self_, other);
}
fn flector_mul_motor(self_: Flector, other: Motor) -> Flector {
    return flector_geometricProduct_flector(self_, other);
}
fn flector_mul_multiVector(self_: Flector, other: MultiVector) -> MultiVector {
    return flector_geometricProduct_flector(self_, other);
}
fn flector_mul_origin(self_: Flector, other: Origin) -> Motor {
    return flector_geometricProduct_flector(self_, other);
}
fn flector_mul_plane(self_: Flector, other: Plane) -> Motor {
    return flector_geometricProduct_flector(self_, other);
}
fn flector_mul_point(self_: Flector, other: Point) -> Motor {
    return flector_geometricProduct_flector(self_, other);
}
fn flector_mul_scalar(self_: Flector, other: Scalar) -> Flector {
    return flector_geometricProduct_flector(self_, other);
}
fn horizon_mul_antiScalar(self_: Horizon, other: AntiScalar) -> Origin {
    return horizon_geometricProduct_horizon(self_, other);
}
fn horizon_mul_dualNum(self_: Horizon, other: DualNum) -> Flector {
    return horizon_geometricProduct_horizon(self_, other);
}
fn horizon_mul_flector(self_: Horizon, other: Flector) -> Motor {
    return horizon_geometricProduct_horizon(self_, other);
}
fn horizon_mul_horizon(self_: Horizon, other: Horizon) -> Scalar {
    return horizon_geometricProduct_horizon(self_, other);
}
fn horizon_mul_line(self_: Horizon, other: Line) -> Flector {
    return horizon_geometricProduct_horizon(self_, other);
}
fn horizon_mul_motor(self_: Horizon, other: Motor) -> Flector {
    return horizon_geometricProduct_horizon(self_, other);
}
fn horizon_mul_multiVector(self_: Horizon, other: MultiVector) -> MultiVector {
    return horizon_geometricProduct_horizon(self_, other);
}
fn horizon_mul_origin(self_: Horizon, other: Origin) -> AntiScalar {
    return horizon_geometricProduct_horizon(self_, other);
}
fn horizon_mul_plane(self_: Horizon, other: Plane) -> Motor {
    return horizon_geometricProduct_horizon(self_, other);
}
fn horizon_mul_point(self_: Horizon, other: Point) -> Motor {
    return horizon_geometricProduct_horizon(self_, other);
}
fn horizon_mul_scalar(self_: Horizon, other: Scalar) -> Horizon {
    return horizon_geometricProduct_horizon(self_, other);
}
fn line_mul_antiScalar(self_: Line, other: AntiScalar) -> Line {
    return line_geometricProduct_line(self_, other);
}
fn line_mul_dualNum(self_: Line, other: DualNum) -> Line {
    return line_geometricProduct_line(self_, other);
}
fn line_mul_flector(self_: Line, other: Flector) -> Flector {
    return line_geometricProduct_line(self_, other);
}
fn line_mul_horizon(self_: Line, other: Horizon) -> Flector {
    return line_geometricProduct_line(self_, other);
}
fn line_mul_line(self_: Line, other: Line) -> Motor {
    return line_geometricProduct_line(self_, other);
}
fn line_mul_motor(self_: Line, other: Motor) -> Motor {
    return line_geometricProduct_line(self_, other);
}
fn line_mul_multiVector(self_: Line, other: MultiVector) -> MultiVector {
    return line_geometricProduct_line(self_, other);
}
fn line_mul_origin(self_: Line, other: Origin) -> Plane {
    return line_geometricProduct_line(self_, other);
}
fn line_mul_plane(self_: Line, other: Plane) -> Flector {
    return line_geometricProduct_line(self_, other);
}
fn line_mul_point(self_: Line, other: Point) -> Flector {
    return line_geometricProduct_line(self_, other);
}
fn line_mul_scalar(self_: Line, other: Scalar) -> Line {
    return line_geometricProduct_line(self_, other);
}
fn motor_mul_antiScalar(self_: Motor, other: AntiScalar) -> Motor {
    return motor_geometricProduct_motor(self_, other);
}
fn motor_mul_dualNum(self_: Motor, other: DualNum) -> Motor {
    return motor_geometricProduct_motor(self_, other);
}
fn motor_mul_flector(self_: Motor, other: Flector) -> Flector {
    return motor_geometricProduct_motor(self_, other);
}
fn motor_mul_horizon(self_: Motor, other: Horizon) -> Flector {
    return motor_geometricProduct_motor(self_, other);
}
fn motor_mul_line(self_: Motor, other: Line) -> Motor {
    return motor_geometricProduct_motor(self_, other);
}
fn motor_mul_motor(self_: Motor, other: Motor) -> Motor {
    return motor_geometricProduct_motor(self_, other);
}
fn motor_mul_multiVector(self_: Motor, other: MultiVector) -> MultiVector {
    return motor_geometricProduct_motor(self_, other);
}
fn motor_mul_origin(self_: Motor, other: Origin) -> Flector {
    return motor_geometricProduct_motor(self_, other);
}
fn motor_mul_plane(self_: Motor, other: Plane) -> Flector {
    return motor_geometricProduct_motor(self_, other);
}
fn motor_mul_point(self_: Motor, other: Point) -> Flector {
    return motor_geometricProduct_motor(self_, other);
}
fn motor_mul_scalar(self_: Motor, other: Scalar) -> Motor {
    return motor_geometricProduct_motor(self_, other);
}
fn multiVector_mul_antiScalar(self_: MultiVector, other: AntiScalar) -> MultiVector {
    return multiVector_geometricProduct_multiVector(self_, other);
}
fn multiVector_mul_dualNum(self_: MultiVector, other: DualNum) -> MultiVector {
    return multiVector_geometricProduct_multiVector(self_, other);
}
fn multiVector_mul_flector(self_: MultiVector, other: Flector) -> MultiVector {
    return multiVector_geometricProduct_multiVector(self_, other);
}
fn multiVector_mul_horizon(self_: MultiVector, other: Horizon) -> MultiVector {
    return multiVector_geometricProduct_multiVector(self_, other);
}
fn multiVector_mul_line(self_: MultiVector, other: Line) -> MultiVector {
    return multiVector_geometricProduct_multiVector(self_, other);
}
fn multiVector_mul_motor(self_: MultiVector, other: Motor) -> MultiVector {
    return multiVector_geometricProduct_multiVector(self_, other);
}
fn multiVector_mul_multiVector(self_: MultiVector, other: MultiVector) -> MultiVector {
    return multiVector_geometricProduct_multiVector(self_, other);
}
fn multiVector_mul_origin(self_: MultiVector, other: Origin) -> MultiVector {
    return multiVector_geometricProduct_multiVector(self_, other);
}
fn multiVector_mul_plane(self_: MultiVector, other: Plane) -> MultiVector {
    return multiVector_geometricProduct_multiVector(self_, other);
}
fn multiVector_mul_point(self_: MultiVector, other: Point) -> MultiVector {
    return multiVector_geometricProduct_multiVector(self_, other);
}
fn multiVector_mul_scalar(self_: MultiVector, other: Scalar) -> MultiVector {
    return multiVector_geometricProduct_multiVector(self_, other);
}
fn origin_mul_dualNum(self_: Origin, other: DualNum) -> Origin {
    return origin_geometricProduct_origin(self_, other);
}
fn origin_mul_flector(self_: Origin, other: Flector) -> Motor {
    return origin_geometricProduct_origin(self_, other);
}
fn origin_mul_horizon(self_: Origin, other: Horizon) -> AntiScalar {
    return origin_geometricProduct_origin(self_, other);
}
fn origin_mul_line(self_: Origin, other: Line) -> Plane {
    return origin_geometricProduct_origin(self_, other);
}
fn origin_mul_motor(self_: Origin, other: Motor) -> Flector {
    return origin_geometricProduct_origin(self_, other);
}
fn origin_mul_multiVector(self_: Origin, other: MultiVector) -> MultiVector {
    return origin_geometricProduct_origin(self_, other);
}
fn origin_mul_plane(self_: Origin, other: Plane) -> AntiScalar {
    return origin_geometricProduct_origin(self_, other);
}
fn origin_mul_point(self_: Origin, other: Point) -> Line {
    return origin_geometricProduct_origin(self_, other);
}
fn origin_mul_scalar(self_: Origin, other: Scalar) -> Origin {
    return origin_geometricProduct_origin(self_, other);
}
fn plane_mul_antiScalar(self_: Plane, other: AntiScalar) -> Origin {
    return plane_geometricProduct_plane(self_, other);
}
fn plane_mul_dualNum(self_: Plane, other: DualNum) -> Flector {
    return plane_geometricProduct_plane(self_, other);
}
fn plane_mul_flector(self_: Plane, other: Flector) -> Motor {
    return plane_geometricProduct_plane(self_, other);
}
fn plane_mul_horizon(self_: Plane, other: Horizon) -> Motor {
    return plane_geometricProduct_plane(self_, other);
}
fn plane_mul_line(self_: Plane, other: Line) -> Flector {
    return plane_geometricProduct_plane(self_, other);
}
fn plane_mul_motor(self_: Plane, other: Motor) -> Flector {
    return plane_geometricProduct_plane(self_, other);
}
fn plane_mul_multiVector(self_: Plane, other: MultiVector) -> MultiVector {
    return plane_geometricProduct_plane(self_, other);
}
fn plane_mul_origin(self_: Plane, other: Origin) -> AntiScalar {
    return plane_geometricProduct_plane(self_, other);
}
fn plane_mul_plane(self_: Plane, other: Plane) -> Motor {
    return plane_geometricProduct_plane(self_, other);
}
fn plane_mul_point(self_: Plane, other: Point) -> Motor {
    return plane_geometricProduct_plane(self_, other);
}
fn plane_mul_scalar(self_: Plane, other: Scalar) -> Plane {
    return plane_geometricProduct_plane(self_, other);
}
fn point_mul_antiScalar(self_: Point, other: AntiScalar) -> Plane {
    return point_geometricProduct_point(self_, other);
}
fn point_mul_dualNum(self_: Point, other: DualNum) -> Flector {
    return point_geometricProduct_point(self_, other);
}
fn point_mul_flector(self_: Point, other: Flector) -> Motor {
    return point_geometricProduct_point(self_, other);
}
fn point_mul_horizon(self_: Point, other: Horizon) -> Motor {
    return point_geometricProduct_point(self_, other);
}
fn point_mul_line(self_: Point, other: Line) -> Flector {
    return point_geometricProduct_point(self_, other);
}
fn point_mul_motor(self_: Point, other: Motor) -> Flector {
    return point_geometricProduct_point(self_, other);
}
fn point_mul_multiVector(self_: Point, other: MultiVector) -> MultiVector {
    return point_geometricProduct_point(self_, other);
}
fn point_mul_origin(self_: Point, other: Origin) -> Line {
    return point_geometricProduct_point(self_, other);
}
fn point_mul_plane(self_: Point, other: Plane) -> Motor {
    return point_geometricProduct_point(self_, other);
}
fn point_mul_point(self_: Point, other: Point) -> Motor {
    return point_geometricProduct_point(self_, other);
}
fn point_mul_scalar(self_: Point, other: Scalar) -> Point {
    return point_geometricProduct_point(self_, other);
}
fn scalar_mul_antiScalar(self_: Scalar, other: AntiScalar) -> AntiScalar {
    return scalar_geometricProduct_scalar(self_, other);
}
fn scalar_mul_dualNum(self_: Scalar, other: DualNum) -> DualNum {
    return scalar_geometricProduct_scalar(self_, other);
}
fn scalar_mul_flector(self_: Scalar, other: Flector) -> Flector {
    return scalar_geometricProduct_scalar(self_, other);
}
fn scalar_mul_horizon(self_: Scalar, other: Horizon) -> Horizon {
    return scalar_geometricProduct_scalar(self_, other);
}
fn scalar_mul_line(self_: Scalar, other: Line) -> Line {
    return scalar_geometricProduct_scalar(self_, other);
}
fn scalar_mul_motor(self_: Scalar, other: Motor) -> Motor {
    return scalar_geometricProduct_scalar(self_, other);
}
fn scalar_mul_multiVector(self_: Scalar, other: MultiVector) -> MultiVector {
    return scalar_geometricProduct_scalar(self_, other);
}
fn scalar_mul_origin(self_: Scalar, other: Origin) -> Origin {
    return scalar_geometricProduct_scalar(self_, other);
}
fn scalar_mul_plane(self_: Scalar, other: Plane) -> Plane {
    return scalar_geometricProduct_scalar(self_, other);
}
fn scalar_mul_point(self_: Scalar, other: Point) -> Point {
    return scalar_geometricProduct_scalar(self_, other);
}
fn scalar_mul_scalar(self_: Scalar, other: Scalar) -> Scalar {
    return scalar_geometricProduct_scalar(self_, other);
}
fn antiScalar_neg(self_: AntiScalar) -> AntiScalar {
    return AntiScalar(/* e1234 */ (self_.g0_ * -1.0));
}
fn dualNum_neg(self_: DualNum) -> DualNum {
    return DualNum(/* scalar, e1234 */ (self_.g0_ * vec2<f32>(-1.0)));
}
fn flector_neg(self_: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (self_.g0_ * vec4<f32>(-1.0)), /* e423, e431, e412, e321 */ (self_.g1_ * vec4<f32>(-1.0)));
}
fn horizon_neg(self_: Horizon) -> Horizon {
    return Horizon(/* e321 */ (self_.g0_ * -1.0));
}
fn line_neg(self_: Line) -> Line {
    return Line(/* e41, e42, e43 */ (self_.g0_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (self_.g1_ * vec3<f32>(-1.0)));
}
fn motor_neg(self_: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (self_.g0_ * vec4<f32>(-1.0)), /* e23, e31, e12, scalar */ (self_.g1_ * vec4<f32>(-1.0)));
}
fn multiVector_neg(self_: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (self_.g0_ * vec2<f32>(-1.0)), /* e1, e2, e3, e4 */ (self_.g1_ * vec4<f32>(-1.0)), /* e41, e42, e43 */ (self_.g2_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (self_.g3_ * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ (self_.g4_ * vec4<f32>(-1.0)));
}
fn origin_neg(self_: Origin) -> Origin {
    return Origin(/* e4 */ (self_.g0_ * -1.0));
}
fn plane_neg(self_: Plane) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ (self_.g0_ * vec4<f32>(-1.0)));
}
fn point_neg(self_: Point) -> Point {
    return Point(/* e1, e2, e3, e4 */ (self_.g0_ * vec4<f32>(-1.0)));
}
fn scalar_neg(self_: Scalar) -> Scalar {
    return Scalar(/* scalar */ (self_.g0_ * -1.0));
}
fn dualNum_one() -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>(1.0, 0.0));
}
fn motor_one() -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0), /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, 1.0));
}
fn multiVector_one() -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(1.0, 0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn scalar_one() -> Scalar {
    return Scalar(/* scalar */ 1.0);
}
fn antiScalar_reverse(self_: AntiScalar) -> AntiScalar {
    return self_;
}
fn dualNum_reverse(self_: DualNum) -> DualNum {
    return self_;
}
fn flector_reverse(self_: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ self_.g0_, /* e423, e431, e412, e321 */ (self_.g1_ * vec4<f32>(-1.0)));
}
fn horizon_reverse(self_: Horizon) -> Horizon {
    return Horizon(/* e321 */ (self_.g0_ * -1.0));
}
fn line_reverse(self_: Line) -> Line {
    return Line(/* e41, e42, e43 */ (self_.g0_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (self_.g1_ * vec3<f32>(-1.0)));
}
fn motor_reverse(self_: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g0_.x * -1.0), (self_.g0_.y * -1.0), (self_.g0_.z * -1.0), self_.g0_.w), /* e23, e31, e12, scalar */ vec4<f32>((self_.g1_.x * -1.0), (self_.g1_.y * -1.0), (self_.g1_.z * -1.0), self_.g1_.w));
}
fn multiVector_reverse(self_: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ self_.g1_, /* e41, e42, e43 */ (self_.g2_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (self_.g3_ * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ (self_.g4_ * vec4<f32>(-1.0)));
}
fn origin_reverse(self_: Origin) -> Origin {
    return self_;
}
fn plane_reverse(self_: Plane) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ (self_.g0_ * vec4<f32>(-1.0)));
}
fn point_reverse(self_: Point) -> Point {
    return self_;
}
fn scalar_reverse(self_: Scalar) -> Scalar {
    return self_;
}
fn dualNum_rightAntiDual(self_: DualNum) -> AntiScalar {
    return AntiScalar(/* e1234 */ self_.g0_.x);
}
fn flector_rightAntiDual(self_: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g1_.w * -1.0)), /* e423, e431, e412, e321 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, 0.0));
}
fn horizon_rightAntiDual(self_: Horizon) -> Origin {
    return Origin(/* e4 */ (self_.g0_ * -1.0));
}
fn line_rightAntiDual(self_: Line) -> Line {
    return Line(/* e41, e42, e43 */ (self_.g1_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ vec3<f32>(0.0));
}
fn motor_rightAntiDual(self_: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g1_.x * -1.0), (self_.g1_.y * -1.0), (self_.g1_.z * -1.0), self_.g1_.w), /* e23, e31, e12, scalar */ vec4<f32>(0.0));
}
fn multiVector_rightAntiDual(self_: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, self_.g0_.x), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g4_.w * -1.0)), /* e41, e42, e43 */ (self_.g3_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, 0.0));
}
fn plane_rightAntiDual(self_: Plane) -> Origin {
    return Origin(/* e4 */ (self_.g0_.w * -1.0));
}
fn point_rightAntiDual(self_: Point) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, 0.0));
}
fn scalar_rightAntiDual(self_: Scalar) -> Scalar {
    return self_;
}
fn dualNum_rightDual(self_: DualNum) -> AntiScalar {
    return AntiScalar(/* e1234 */ self_.g0_.x);
}
fn flector_rightDual(self_: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g1_.w * -1.0)), /* e423, e431, e412, e321 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, 0.0));
}
fn horizon_rightDual(self_: Horizon) -> Origin {
    return Origin(/* e4 */ (self_.g0_ * -1.0));
}
fn line_rightDual(self_: Line) -> Line {
    return Line(/* e41, e42, e43 */ (self_.g1_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ vec3<f32>(0.0));
}
fn motor_rightDual(self_: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g1_.x * -1.0), (self_.g1_.y * -1.0), (self_.g1_.z * -1.0), self_.g1_.w), /* e23, e31, e12, scalar */ vec4<f32>(0.0));
}
fn multiVector_rightDual(self_: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, self_.g0_.x), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g4_.w * -1.0)), /* e41, e42, e43 */ (self_.g3_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, 0.0));
}
fn plane_rightDual(self_: Plane) -> Origin {
    return Origin(/* e4 */ (self_.g0_.w * -1.0));
}
fn point_rightDual(self_: Point) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, 0.0));
}
fn scalar_rightDual(self_: Scalar) -> Scalar {
    return self_;
}
fn antiScalar_sandwich_flector(self_: AntiScalar, other: Flector) -> Flector {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g1_.w * self_.g0_ * -1.0)), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.x * self_.g0_ * -1.0), (other.g0_.y * self_.g0_ * -1.0), (other.g0_.z * self_.g0_ * -1.0), 0.0));
    return flector_geometricProduct_flector(geometric_product, antiScalar_reverse(self_));
}
fn antiScalar_sandwich_line(self_: AntiScalar, other: Line) -> Line {
    let geometric_product: Line = Line(/* e41, e42, e43 */ (vec3<f32>(self_.g0_) * other.g1_), /* e23, e31, e12 */ vec3<f32>(0.0));
    return line_geometricProduct_line(geometric_product, antiScalar_reverse(self_));
}
fn antiScalar_sandwich_motor(self_: AntiScalar, other: Motor) -> Motor {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(self_.g0_) * other.g1_), /* e23, e31, e12, scalar */ vec4<f32>(0.0));
    return motor_geometricProduct_motor(geometric_product, antiScalar_reverse(self_));
}
fn antiScalar_sandwich_multiVector(self_: AntiScalar, other: MultiVector) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (other.g0_.x * self_.g0_)), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g4_.w * self_.g0_ * -1.0)), /* e41, e42, e43 */ (vec3<f32>(self_.g0_) * other.g3_), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>((other.g1_.x * self_.g0_ * -1.0), (other.g1_.y * self_.g0_ * -1.0), (other.g1_.z * self_.g0_ * -1.0), 0.0));
    return multiVector_geometricProduct_multiVector(geometric_product, antiScalar_reverse(self_));
}
fn antiScalar_sandwich_point(self_: AntiScalar, other: Point) -> Origin {
    let geometric_product: Plane = Plane(/* e423, e431, e412, e321 */ vec4<f32>((other.g0_.x * self_.g0_ * -1.0), (other.g0_.y * self_.g0_ * -1.0), (other.g0_.z * self_.g0_ * -1.0), 0.0));
    return plane_geometricProduct_plane(geometric_product, antiScalar_reverse(self_));
}
fn dualNum_sandwich_antiScalar(self_: DualNum, other: AntiScalar) -> AntiScalar {
    let geometric_product: AntiScalar = AntiScalar(/* e1234 */ (self_.g0_.x * other.g0_));
    return antiScalar_geometricProduct_antiScalar(geometric_product, dualNum_reverse(self_));
}
fn dualNum_sandwich_dualNum(self_: DualNum, other: DualNum) -> DualNum {
    let geometric_product: DualNum = DualNum(/* scalar, e1234 */ vec2<f32>((other.g0_.x * self_.g0_.x), ((other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x))));
    return dualNum_geometricProduct_dualNum(geometric_product, dualNum_reverse(self_));
}
fn dualNum_sandwich_flector(self_: DualNum, other: Flector) -> Flector {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g0_.x), (self_.g0_.x * other.g0_.y), (self_.g0_.x * other.g0_.z), ((self_.g0_.x * other.g0_.w) - (self_.g0_.y * other.g1_.w))), /* e423, e431, e412, e321 */ vec4<f32>(((self_.g0_.x * other.g1_.x) - (self_.g0_.y * other.g0_.x)), ((self_.g0_.x * other.g1_.y) - (self_.g0_.y * other.g0_.y)), ((self_.g0_.x * other.g1_.z) - (self_.g0_.y * other.g0_.z)), (self_.g0_.x * other.g1_.w)));
    return flector_geometricProduct_flector(geometric_product, dualNum_reverse(self_));
}
fn dualNum_sandwich_horizon(self_: DualNum, other: Horizon) -> Flector {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.y * other.g0_ * -1.0)), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.x * other.g0_)));
    return flector_geometricProduct_flector(geometric_product, dualNum_reverse(self_));
}
fn dualNum_sandwich_line(self_: DualNum, other: Line) -> Line {
    let geometric_product: Line = Line(/* e41, e42, e43 */ ((vec3<f32>(self_.g0_.x) * other.g0_) + (vec3<f32>(self_.g0_.y) * other.g1_)), /* e23, e31, e12 */ (vec3<f32>(self_.g0_.x) * other.g1_));
    return line_geometricProduct_line(geometric_product, dualNum_reverse(self_));
}
fn dualNum_sandwich_motor(self_: DualNum, other: Motor) -> Motor {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ ((vec4<f32>(self_.g0_.x) * other.g0_) + (vec4<f32>(self_.g0_.y) * other.g1_)), /* e23, e31, e12, scalar */ (vec4<f32>(self_.g0_.x) * other.g1_));
    return motor_geometricProduct_motor(geometric_product, dualNum_reverse(self_));
}
fn dualNum_sandwich_multiVector(self_: DualNum, other: MultiVector) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>((self_.g0_.x * other.g0_.x), ((self_.g0_.x * other.g0_.y) + (self_.g0_.y * other.g0_.x))), /* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g1_.x), (self_.g0_.x * other.g1_.y), (self_.g0_.x * other.g1_.z), ((self_.g0_.x * other.g1_.w) - (self_.g0_.y * other.g4_.w))), /* e41, e42, e43 */ ((vec3<f32>(self_.g0_.x) * other.g2_) + (vec3<f32>(self_.g0_.y) * other.g3_)), /* e23, e31, e12 */ (vec3<f32>(self_.g0_.x) * other.g3_), /* e423, e431, e412, e321 */ vec4<f32>(((self_.g0_.x * other.g4_.x) - (self_.g0_.y * other.g1_.x)), ((self_.g0_.x * other.g4_.y) - (self_.g0_.y * other.g1_.y)), ((self_.g0_.x * other.g4_.z) - (self_.g0_.y * other.g1_.z)), (self_.g0_.x * other.g4_.w)));
    return multiVector_geometricProduct_multiVector(geometric_product, dualNum_reverse(self_));
}
fn dualNum_sandwich_origin(self_: DualNum, other: Origin) -> Origin {
    let geometric_product: Origin = Origin(/* e4 */ (self_.g0_.x * other.g0_));
    return origin_geometricProduct_origin(geometric_product, dualNum_reverse(self_));
}
fn dualNum_sandwich_plane(self_: DualNum, other: Plane) -> Flector {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.y * other.g0_.w * -1.0)), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_.x) * other.g0_));
    return flector_geometricProduct_flector(geometric_product, dualNum_reverse(self_));
}
fn dualNum_sandwich_point(self_: DualNum, other: Point) -> Flector {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_.x) * other.g0_), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.y * other.g0_.x * -1.0), (self_.g0_.y * other.g0_.y * -1.0), (self_.g0_.y * other.g0_.z * -1.0), 0.0));
    return flector_geometricProduct_flector(geometric_product, dualNum_reverse(self_));
}
fn dualNum_sandwich_scalar(self_: DualNum, other: Scalar) -> DualNum {
    let geometric_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>(other.g0_) * self_.g0_));
    return dualNum_geometricProduct_dualNum(geometric_product, dualNum_reverse(self_));
}
fn flector_sandwich_antiScalar(self_: Flector, other: AntiScalar) -> Motor {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g1_.w * other.g0_)), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), 0.0));
    return flector_geometricProduct_flector(geometric_product, flector_reverse(self_));
}
fn flector_sandwich_dualNum(self_: Flector, other: DualNum) -> Motor {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g0_.x), (other.g0_.x * self_.g0_.y), (other.g0_.x * self_.g0_.z), ((other.g0_.x * self_.g0_.w) + (other.g0_.y * self_.g1_.w))), /* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.x * self_.g1_.x) + (other.g0_.y * self_.g0_.x)), ((other.g0_.x * self_.g1_.y) + (other.g0_.y * self_.g0_.y)), ((other.g0_.x * self_.g1_.z) + (other.g0_.y * self_.g0_.z)), (other.g0_.x * self_.g1_.w)));
    return flector_geometricProduct_flector(geometric_product, flector_reverse(self_));
}
fn flector_sandwich_flector(self_: Flector, other: Flector) -> Flector {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(((other.g0_.y * self_.g1_.z) - (other.g1_.y * self_.g0_.z)), ((other.g0_.z * self_.g1_.x) - (other.g1_.z * self_.g0_.x)), ((other.g0_.x * self_.g1_.y) - (other.g1_.x * self_.g0_.y)), (-(other.g0_.w * self_.g1_.w) + (other.g1_.w * self_.g0_.w))) + (vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.x) * self_.g0_.wwwx) - (vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, other.g0_.z) * self_.g1_.wwwz) - (vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g1_.y) * other.g0_.wwwy) + (vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g0_.z) * other.g1_.wwwz) - (other.g0_.zxyx * self_.g1_.yzxx) + (other.g1_.zxyy * self_.g0_.yzxy)), /* e23, e31, e12, scalar */ (vec4<f32>((-(other.g0_.y * self_.g0_.z) - (other.g1_.w * self_.g0_.x)), (-(other.g0_.z * self_.g0_.x) - (other.g1_.w * self_.g0_.y)), (-(other.g0_.x * self_.g0_.y) - (other.g1_.w * self_.g0_.z)), ((other.g0_.y * self_.g0_.y) + (other.g0_.z * self_.g0_.z))) - (vec4<f32>(self_.g1_.w) * vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.w)) + (other.g0_.zxyx * self_.g0_.yzxx)));
    return motor_geometricProduct_motor(geometric_product, flector_reverse(self_));
}
fn flector_sandwich_horizon(self_: Flector, other: Horizon) -> Flector {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g0_.w)), /* e23, e31, e12, scalar */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g1_.w) * vec4<f32>(-1.0)));
    return motor_geometricProduct_motor(geometric_product, flector_reverse(self_));
}
fn flector_sandwich_line(self_: Flector, other: Line) -> Motor {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(((other.g1_.x * self_.g1_.w) + (other.g1_.y * self_.g0_.z)), ((other.g1_.y * self_.g1_.w) + (other.g1_.z * self_.g0_.x)), ((other.g1_.x * self_.g0_.y) + (other.g1_.z * self_.g1_.w)), (-(other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z) - (other.g1_.x * self_.g1_.x) - (other.g1_.y * self_.g1_.y) - (other.g1_.z * self_.g1_.z))) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.x) * self_.g0_.yzxx)), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.x * self_.g1_.w) + (other.g0_.y * self_.g0_.z) + (other.g1_.x * self_.g0_.w) + (other.g1_.y * self_.g1_.z) - (other.g1_.z * self_.g1_.y)), ((other.g0_.y * self_.g1_.w) + (other.g0_.z * self_.g0_.x) - (other.g1_.x * self_.g1_.z) + (other.g1_.y * self_.g0_.w) + (other.g1_.z * self_.g1_.x)), ((other.g0_.x * self_.g0_.y) + (other.g0_.z * self_.g1_.w) + (other.g1_.x * self_.g1_.y) - (other.g1_.y * self_.g1_.x) + (other.g1_.z * self_.g0_.w)), (-(other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))) - (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.y, other.g1_.x) * self_.g0_.yzxx)));
    return flector_geometricProduct_flector(geometric_product, flector_reverse(self_));
}
fn flector_sandwich_motor(self_: Flector, other: Motor) -> Motor {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>((self_.g0_.z * other.g1_.y), (self_.g0_.y * other.g1_.w), (self_.g0_.z * other.g1_.w), (-(self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z) - (self_.g1_.x * other.g1_.x) - (self_.g1_.y * other.g1_.y) - (self_.g1_.z * other.g1_.z))) + (vec4<f32>(self_.g1_.w) * vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, other.g0_.w)) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.x) * self_.g0_.yzxx) + (self_.g0_.xxyw * other.g1_.wzxw)), /* e423, e431, e412, e321 */ (vec4<f32>(((self_.g0_.x * other.g0_.w) + (self_.g0_.z * other.g0_.y) + (self_.g1_.x * other.g1_.w) + (self_.g1_.z * other.g1_.y) + (self_.g1_.w * other.g0_.x)), ((self_.g0_.x * other.g0_.z) + (self_.g0_.y * other.g0_.w) + (self_.g1_.x * other.g1_.z) + (self_.g1_.y * other.g1_.w) + (self_.g1_.w * other.g0_.y)), ((self_.g0_.y * other.g0_.x) + (self_.g0_.z * other.g0_.w) + (self_.g1_.y * other.g1_.x) + (self_.g1_.z * other.g1_.w) + (self_.g1_.w * other.g0_.z)), ((self_.g0_.z * other.g1_.z) * -1.0)) + (vec4<f32>(self_.g0_.w, self_.g0_.w, self_.g0_.w, self_.g1_.w) * other.g1_) - (vec4<f32>(self_.g1_.y, self_.g1_.z, self_.g1_.x, self_.g0_.y) * other.g1_.zxyy) - (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.y, other.g1_.x) * self_.g0_.yzxx)));
    return flector_geometricProduct_flector(geometric_product, flector_reverse(self_));
}
fn flector_sandwich_multiVector(self_: Flector, other: MultiVector) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(0.0, ((self_.g0_.w * other.g4_.w) - (self_.g1_.x * other.g1_.x) - (self_.g1_.y * other.g1_.y) - (self_.g1_.z * other.g1_.z))) + (vec2<f32>(self_.g0_.x) * vec2<f32>(other.g1_.x, other.g4_.x)) + (vec2<f32>(self_.g0_.y) * vec2<f32>(other.g1_.y, other.g4_.y)) + (vec2<f32>(self_.g0_.z) * vec2<f32>(other.g1_.z, other.g4_.z)) - (vec2<f32>(self_.g1_.w) * vec2<f32>(other.g4_.w, other.g1_.w))), /* e1, e2, e3, e4 */ (vec4<f32>((other.g3_.y * self_.g0_.z), (other.g3_.z * self_.g0_.x), (other.g3_.x * self_.g0_.y), (-(other.g2_.y * self_.g0_.y) - (other.g2_.z * self_.g0_.z) - (other.g3_.x * self_.g1_.x) - (other.g3_.y * self_.g1_.y) - (other.g3_.z * self_.g1_.z))) + (vec4<f32>(other.g0_.x) * self_.g0_) + (vec4<f32>(self_.g1_.w) * vec4<f32>(other.g3_.x, other.g3_.y, other.g3_.z, other.g0_.y)) - (vec4<f32>(other.g3_.z, other.g3_.x, other.g3_.y, other.g2_.x) * self_.g0_.yzxx)), /* e41, e42, e43 */ (vec3<f32>(((self_.g0_.y * other.g4_.z) - (self_.g0_.z * other.g4_.y) - (self_.g1_.y * other.g1_.z) + (self_.g1_.z * other.g1_.y)), (-(self_.g0_.x * other.g4_.z) + (self_.g0_.z * other.g4_.x) + (self_.g1_.x * other.g1_.z) - (self_.g1_.z * other.g1_.x)), ((self_.g0_.x * other.g4_.y) - (self_.g0_.y * other.g4_.x) - (self_.g1_.x * other.g1_.y) + (self_.g1_.y * other.g1_.x))) + (vec3<f32>(self_.g0_.w) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z)) - (vec3<f32>(self_.g1_.w) * vec3<f32>(other.g4_.x, other.g4_.y, other.g4_.z)) - (vec3<f32>(other.g1_.w) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z)) + (vec3<f32>(other.g4_.w) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z))), /* e23, e31, e12 */ (vec3<f32>(((self_.g0_.y * other.g1_.z) - (self_.g0_.z * other.g1_.y)), (-(self_.g0_.x * other.g1_.z) + (self_.g0_.z * other.g1_.x)), ((self_.g0_.x * other.g1_.y) - (self_.g0_.y * other.g1_.x))) - (vec3<f32>(self_.g1_.w) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z)) - (vec3<f32>(other.g4_.w) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z))), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.y * self_.g0_.x) + (other.g2_.x * self_.g1_.w) + (other.g2_.y * self_.g0_.z) + (other.g3_.x * self_.g0_.w) + (other.g3_.y * self_.g1_.z) - (other.g3_.z * self_.g1_.y)), ((other.g0_.y * self_.g0_.y) + (other.g2_.y * self_.g1_.w) + (other.g2_.z * self_.g0_.x) - (other.g3_.x * self_.g1_.z) + (other.g3_.y * self_.g0_.w) + (other.g3_.z * self_.g1_.x)), ((other.g0_.y * self_.g0_.z) + (other.g2_.x * self_.g0_.y) + (other.g2_.z * self_.g1_.w) + (other.g3_.x * self_.g1_.y) - (other.g3_.y * self_.g1_.x) + (other.g3_.z * self_.g0_.w)), (-(other.g3_.y * self_.g0_.y) - (other.g3_.z * self_.g0_.z))) + (vec4<f32>(other.g0_.x) * self_.g1_) - (vec4<f32>(other.g2_.z, other.g2_.x, other.g2_.y, other.g3_.x) * self_.g0_.yzxx)));
    return multiVector_geometricProduct_multiVector(geometric_product, flector_reverse(self_));
}
fn flector_sandwich_origin(self_: Flector, other: Origin) -> Flector {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g1_.w) * vec4<f32>(-1.0)), /* e23, e31, e12, scalar */ vec4<f32>(0.0));
    return motor_geometricProduct_motor(geometric_product, flector_reverse(self_));
}
fn flector_sandwich_plane(self_: Flector, other: Plane) -> Flector {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>((-(self_.g0_.z * other.g0_.y) - (self_.g1_.w * other.g0_.x)), (-(self_.g0_.x * other.g0_.z) - (self_.g1_.w * other.g0_.y)), (-(self_.g0_.y * other.g0_.x) - (self_.g1_.w * other.g0_.z)), ((self_.g0_.z * other.g0_.z) + (self_.g0_.w * other.g0_.w))) + (vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g0_.y) * other.g0_.wwwy) + (self_.g0_.yzxx * other.g0_.zxyx)), /* e23, e31, e12, scalar */ (vec4<f32>(other.g0_.w) * vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g1_.w) * vec4<f32>(-1.0)));
    return motor_geometricProduct_motor(geometric_product, flector_reverse(self_));
}
fn flector_sandwich_point(self_: Flector, other: Point) -> Flector {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(((self_.g0_.w * other.g0_.x) + (self_.g1_.z * other.g0_.y)), ((self_.g0_.w * other.g0_.y) + (self_.g1_.x * other.g0_.z)), ((self_.g0_.w * other.g0_.z) + (self_.g1_.y * other.g0_.x)), (-(self_.g1_.z * other.g0_.z) - (self_.g1_.w * other.g0_.w))) - (vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g1_.x) * other.g0_.wwwx) - (self_.g1_.yzxy * other.g0_.zxyy)), /* e23, e31, e12, scalar */ (vec4<f32>((-(self_.g0_.z * other.g0_.y) - (self_.g1_.w * other.g0_.x)), (-(self_.g0_.x * other.g0_.z) - (self_.g1_.w * other.g0_.y)), (-(self_.g0_.y * other.g0_.x) - (self_.g1_.w * other.g0_.z)), ((self_.g0_.y * other.g0_.y) + (self_.g0_.z * other.g0_.z))) + (self_.g0_.yzxx * other.g0_.zxyx)));
    return motor_geometricProduct_motor(geometric_product, flector_reverse(self_));
}
fn flector_sandwich_scalar(self_: Flector, other: Scalar) -> Motor {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(other.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_) * self_.g1_));
    return flector_geometricProduct_flector(geometric_product, flector_reverse(self_));
}
fn horizon_sandwich_antiScalar(self_: Horizon, other: AntiScalar) -> AntiScalar {
    let geometric_product: Origin = Origin(/* e4 */ (other.g0_ * self_.g0_));
    return origin_geometricProduct_origin(geometric_product, horizon_reverse(self_));
}
fn horizon_sandwich_dualNum(self_: Horizon, other: DualNum) -> Motor {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.y * self_.g0_)), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.x * self_.g0_)));
    return flector_geometricProduct_flector(geometric_product, horizon_reverse(self_));
}
fn horizon_sandwich_flector(self_: Horizon, other: Flector) -> Flector {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, other.g0_.w) * vec4<f32>(-1.0)), /* e23, e31, e12, scalar */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.w) * vec4<f32>(-1.0)));
    return motor_geometricProduct_motor(geometric_product, horizon_reverse(self_));
}
fn horizon_sandwich_horizon(self_: Horizon, other: Horizon) -> Horizon {
    let geometric_product: Scalar = Scalar(/* scalar */ (other.g0_ * self_.g0_ * -1.0));
    return scalar_geometricProduct_scalar(geometric_product, horizon_reverse(self_));
}
fn horizon_sandwich_line(self_: Horizon, other: Line) -> Motor {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_), 0.0), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), 0.0));
    return flector_geometricProduct_flector(geometric_product, horizon_reverse(self_));
}
fn horizon_sandwich_motor(self_: Horizon, other: Motor) -> Motor {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, other.g0_.w)), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.w)));
    return flector_geometricProduct_flector(geometric_product, horizon_reverse(self_));
}
fn horizon_sandwich_multiVector(self_: Horizon, other: MultiVector) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(self_.g0_) * vec2<f32>(other.g4_.w, other.g1_.w) * vec2<f32>(-1.0)), /* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g3_.x, other.g3_.y, other.g3_.z, other.g0_.y)), /* e41, e42, e43 */ (vec3<f32>(self_.g0_) * vec3<f32>(other.g4_.x, other.g4_.y, other.g4_.z) * vec3<f32>(-1.0)), /* e23, e31, e12 */ (vec3<f32>(self_.g0_) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z) * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g2_.x, other.g2_.y, other.g2_.z, other.g0_.x)));
    return multiVector_geometricProduct_multiVector(geometric_product, horizon_reverse(self_));
}
fn horizon_sandwich_origin(self_: Horizon, other: Origin) -> Origin {
    let geometric_product: AntiScalar = AntiScalar(/* e1234 */ (self_.g0_ * other.g0_ * -1.0));
    return antiScalar_geometricProduct_antiScalar(geometric_product, horizon_reverse(self_));
}
fn horizon_sandwich_plane(self_: Horizon, other: Plane) -> Flector {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g0_.x * self_.g0_ * -1.0), (other.g0_.y * self_.g0_ * -1.0), (other.g0_.z * self_.g0_ * -1.0), 0.0), /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.w * self_.g0_ * -1.0)));
    return motor_geometricProduct_motor(geometric_product, horizon_reverse(self_));
}
fn horizon_sandwich_point(self_: Horizon, other: Point) -> Flector {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.w * self_.g0_ * -1.0)), /* e23, e31, e12, scalar */ vec4<f32>((other.g0_.x * self_.g0_ * -1.0), (other.g0_.y * self_.g0_ * -1.0), (other.g0_.z * self_.g0_ * -1.0), 0.0));
    return motor_geometricProduct_motor(geometric_product, horizon_reverse(self_));
}
fn horizon_sandwich_scalar(self_: Horizon, other: Scalar) -> Scalar {
    let geometric_product: Horizon = Horizon(/* e321 */ (self_.g0_ * other.g0_));
    return horizon_geometricProduct_horizon(geometric_product, horizon_reverse(self_));
}
fn line_sandwich_antiScalar(self_: Line, other: AntiScalar) -> Motor {
    let geometric_product: Line = Line(/* e41, e42, e43 */ (vec3<f32>(other.g0_) * self_.g1_), /* e23, e31, e12 */ vec3<f32>(0.0));
    return line_geometricProduct_line(geometric_product, line_reverse(self_));
}
fn line_sandwich_dualNum(self_: Line, other: DualNum) -> Motor {
    let geometric_product: Line = Line(/* e41, e42, e43 */ ((vec3<f32>(other.g0_.x) * self_.g0_) + (vec3<f32>(other.g0_.y) * self_.g1_)), /* e23, e31, e12 */ (vec3<f32>(other.g0_.x) * self_.g1_));
    return line_geometricProduct_line(geometric_product, line_reverse(self_));
}
fn line_sandwich_flector(self_: Line, other: Flector) -> Flector {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(((self_.g1_.x * other.g1_.w) - (self_.g1_.y * other.g0_.z)), ((self_.g1_.y * other.g1_.w) - (self_.g1_.z * other.g0_.x)), (-(self_.g1_.x * other.g0_.y) + (self_.g1_.z * other.g1_.w)), ((self_.g0_.y * other.g0_.y) + (self_.g0_.z * other.g0_.z) - (self_.g1_.x * other.g1_.x) - (self_.g1_.y * other.g1_.y) - (self_.g1_.z * other.g1_.z))) + (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g0_.x) * other.g0_.yzxx)), /* e423, e431, e412, e321 */ (vec4<f32>((-(self_.g0_.x * other.g1_.w) + (self_.g0_.y * other.g0_.z) + (self_.g1_.x * other.g0_.w) - (self_.g1_.y * other.g1_.z) + (self_.g1_.z * other.g1_.y)), (-(self_.g0_.y * other.g1_.w) + (self_.g0_.z * other.g0_.x) + (self_.g1_.x * other.g1_.z) + (self_.g1_.y * other.g0_.w) - (self_.g1_.z * other.g1_.x)), ((self_.g0_.x * other.g0_.y) - (self_.g0_.z * other.g1_.w) - (self_.g1_.x * other.g1_.y) + (self_.g1_.y * other.g1_.x) + (self_.g1_.z * other.g0_.w)), (-(self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g1_.x) * other.g0_.yzxx)));
    return flector_geometricProduct_flector(geometric_product, line_reverse(self_));
}
fn line_sandwich_horizon(self_: Line, other: Horizon) -> Flector {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), 0.0), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.x * other.g0_ * -1.0), (self_.g0_.y * other.g0_ * -1.0), (self_.g0_.z * other.g0_ * -1.0), 0.0));
    return flector_geometricProduct_flector(geometric_product, line_reverse(self_));
}
fn line_sandwich_line(self_: Line, other: Line) -> Motor {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>(((other.g0_.y * self_.g1_.z) - (other.g0_.z * self_.g1_.y) + (other.g1_.y * self_.g0_.z) - (other.g1_.z * self_.g0_.y)), (-(other.g0_.x * self_.g1_.z) + (other.g0_.z * self_.g1_.x) - (other.g1_.x * self_.g0_.z) + (other.g1_.z * self_.g0_.x)), ((other.g0_.x * self_.g1_.y) - (other.g0_.y * self_.g1_.x) + (other.g1_.x * self_.g0_.y) - (other.g1_.y * self_.g0_.x)), (-(other.g0_.x * self_.g1_.x) - (other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))), /* e23, e31, e12, scalar */ vec4<f32>(((other.g1_.y * self_.g1_.z) - (other.g1_.z * self_.g1_.y)), (-(other.g1_.x * self_.g1_.z) + (other.g1_.z * self_.g1_.x)), ((other.g1_.x * self_.g1_.y) - (other.g1_.y * self_.g1_.x)), (-(other.g1_.x * self_.g1_.x) - (other.g1_.y * self_.g1_.y) - (other.g1_.z * self_.g1_.z))));
    return motor_geometricProduct_motor(geometric_product, line_reverse(self_));
}
fn line_sandwich_motor(self_: Line, other: Motor) -> Motor {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(((self_.g0_.x * other.g1_.w) + (self_.g0_.z * other.g1_.y) + (self_.g1_.x * other.g0_.w) + (self_.g1_.z * other.g0_.y)), ((self_.g0_.x * other.g1_.z) + (self_.g0_.y * other.g1_.w) + (self_.g1_.x * other.g0_.z) + (self_.g1_.y * other.g0_.w)), ((self_.g0_.y * other.g1_.x) + (self_.g0_.z * other.g1_.w) + (self_.g1_.y * other.g0_.x) + (self_.g1_.z * other.g0_.w)), (-(self_.g0_.y * other.g1_.y) - (self_.g0_.z * other.g1_.z) - (self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))) - (vec4<f32>(self_.g0_.y, self_.g0_.z, self_.g0_.x, self_.g0_.x) * other.g1_.zxyx) - (vec4<f32>(self_.g1_.y, self_.g1_.z, self_.g1_.x, self_.g1_.x) * other.g0_.zxyx)), /* e23, e31, e12, scalar */ (vec4<f32>(((self_.g1_.x * other.g1_.w) + (self_.g1_.z * other.g1_.y)), ((self_.g1_.x * other.g1_.z) + (self_.g1_.y * other.g1_.w)), ((self_.g1_.y * other.g1_.x) + (self_.g1_.z * other.g1_.w)), (-(self_.g1_.y * other.g1_.y) - (self_.g1_.z * other.g1_.z))) - (vec4<f32>(self_.g1_.y, self_.g1_.z, self_.g1_.x, self_.g1_.x) * other.g1_.zxyx)));
    return motor_geometricProduct_motor(geometric_product, line_reverse(self_));
}
fn line_sandwich_multiVector(self_: Line, other: MultiVector) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(0.0, (-(self_.g1_.x * other.g2_.x) - (self_.g1_.y * other.g2_.y) - (self_.g1_.z * other.g2_.z))) - (vec2<f32>(other.g3_.x) * vec2<f32>(self_.g1_.x, self_.g0_.x)) - (vec2<f32>(other.g3_.y) * vec2<f32>(self_.g1_.y, self_.g0_.y)) - (vec2<f32>(other.g3_.z) * vec2<f32>(self_.g1_.z, self_.g0_.z))), /* e1, e2, e3, e4 */ (vec4<f32>(((self_.g1_.x * other.g4_.w) - (self_.g1_.y * other.g1_.z)), ((self_.g1_.y * other.g4_.w) - (self_.g1_.z * other.g1_.x)), (-(self_.g1_.x * other.g1_.y) + (self_.g1_.z * other.g4_.w)), ((self_.g0_.y * other.g1_.y) + (self_.g0_.z * other.g1_.z) - (self_.g1_.x * other.g4_.x) - (self_.g1_.y * other.g4_.y) - (self_.g1_.z * other.g4_.z))) + (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g0_.x) * other.g1_.yzxx)), /* e41, e42, e43 */ ((vec3<f32>(other.g0_.x) * self_.g0_) + (vec3<f32>(other.g0_.y) * self_.g1_) - (self_.g0_.yzx * other.g3_.zxy) + (self_.g0_.zxy * other.g3_.yzx) - (self_.g1_.yzx * other.g2_.zxy) + (self_.g1_.zxy * other.g2_.yzx)), /* e23, e31, e12 */ ((vec3<f32>(other.g0_.x) * self_.g1_) - (self_.g1_.yzx * other.g3_.zxy) + (self_.g1_.zxy * other.g3_.yzx)), /* e423, e431, e412, e321 */ (vec4<f32>((-(self_.g0_.x * other.g4_.w) + (self_.g0_.y * other.g1_.z) + (self_.g1_.x * other.g1_.w) - (self_.g1_.y * other.g4_.z) + (self_.g1_.z * other.g4_.y)), (-(self_.g0_.y * other.g4_.w) + (self_.g0_.z * other.g1_.x) + (self_.g1_.x * other.g4_.z) + (self_.g1_.y * other.g1_.w) - (self_.g1_.z * other.g4_.x)), ((self_.g0_.x * other.g1_.y) - (self_.g0_.z * other.g4_.w) - (self_.g1_.x * other.g4_.y) + (self_.g1_.y * other.g4_.x) + (self_.g1_.z * other.g1_.w)), (-(self_.g1_.y * other.g1_.y) - (self_.g1_.z * other.g1_.z))) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g1_.x) * other.g1_.yzxx)));
    return multiVector_geometricProduct_multiVector(geometric_product, line_reverse(self_));
}
fn line_sandwich_origin(self_: Line, other: Origin) -> Flector {
    let geometric_product: Plane = Plane(/* e423, e431, e412, e321 */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), 0.0));
    return plane_geometricProduct_plane(geometric_product, line_reverse(self_));
}
fn line_sandwich_plane(self_: Line, other: Plane) -> Flector {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g1_.x * other.g0_.w), (self_.g1_.y * other.g0_.w), (self_.g1_.z * other.g0_.w), (-(self_.g1_.x * other.g0_.x) - (self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))), /* e423, e431, e412, e321 */ vec4<f32>((-(self_.g0_.x * other.g0_.w) - (self_.g1_.y * other.g0_.z) + (self_.g1_.z * other.g0_.y)), (-(self_.g0_.y * other.g0_.w) + (self_.g1_.x * other.g0_.z) - (self_.g1_.z * other.g0_.x)), (-(self_.g0_.z * other.g0_.w) - (self_.g1_.x * other.g0_.y) + (self_.g1_.y * other.g0_.x)), 0.0));
    return flector_geometricProduct_flector(geometric_product, line_reverse(self_));
}
fn line_sandwich_point(self_: Line, other: Point) -> Flector {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(((self_.g1_.y * other.g0_.z) * -1.0), ((self_.g1_.z * other.g0_.x) * -1.0), ((self_.g1_.x * other.g0_.y) * -1.0), ((self_.g0_.y * other.g0_.y) + (self_.g0_.z * other.g0_.z))) + (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g0_.x) * other.g0_.yzxx)), /* e423, e431, e412, e321 */ (vec4<f32>(((self_.g0_.y * other.g0_.z) + (self_.g1_.x * other.g0_.w)), ((self_.g0_.z * other.g0_.x) + (self_.g1_.y * other.g0_.w)), ((self_.g0_.x * other.g0_.y) + (self_.g1_.z * other.g0_.w)), (-(self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g1_.x) * other.g0_.yzxx)));
    return flector_geometricProduct_flector(geometric_product, line_reverse(self_));
}
fn line_sandwich_scalar(self_: Line, other: Scalar) -> Motor {
    let geometric_product: Line = Line(/* e41, e42, e43 */ (vec3<f32>(other.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(other.g0_) * self_.g1_));
    return line_geometricProduct_line(geometric_product, line_reverse(self_));
}
fn motor_sandwich_antiScalar(self_: Motor, other: AntiScalar) -> Motor {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(other.g0_) * self_.g1_), /* e23, e31, e12, scalar */ vec4<f32>(0.0));
    return motor_geometricProduct_motor(geometric_product, motor_reverse(self_));
}
fn motor_sandwich_dualNum(self_: Motor, other: DualNum) -> Motor {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ ((vec4<f32>(other.g0_.x) * self_.g0_) + (vec4<f32>(other.g0_.y) * self_.g1_)), /* e23, e31, e12, scalar */ (vec4<f32>(other.g0_.x) * self_.g1_));
    return motor_geometricProduct_motor(geometric_product, motor_reverse(self_));
}
fn motor_sandwich_flector(self_: Motor, other: Flector) -> Flector {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(0.0, 0.0, 0.0, ((other.g0_.z * self_.g0_.z) - (other.g1_.y * self_.g1_.y) - (other.g1_.z * self_.g1_.z) - (other.g1_.w * self_.g0_.w))) - (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.y, other.g1_.x) * self_.g1_.yzxx) + (vec4<f32>(other.g1_.w, other.g1_.w, other.g1_.w, other.g0_.w) * self_.g1_) + (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.w, self_.g0_.y) * other.g0_.yzzy) + (vec4<f32>(self_.g1_.w, self_.g1_.w, self_.g1_.y, self_.g0_.x) * other.g0_.xyxx)), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.z * self_.g0_.y) + (other.g1_.x * self_.g1_.w) + (other.g1_.y * self_.g1_.z) - (other.g1_.w * self_.g0_.x)), ((other.g0_.x * self_.g0_.z) + (other.g1_.y * self_.g1_.w) + (other.g1_.z * self_.g1_.x) - (other.g1_.w * self_.g0_.y)), ((other.g0_.y * self_.g0_.x) + (other.g1_.x * self_.g1_.y) + (other.g1_.z * self_.g1_.w) - (other.g1_.w * self_.g0_.z)), 0.0) + (vec4<f32>(other.g0_.w, other.g0_.w, other.g0_.w, other.g1_.w) * self_.g1_) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.z) * self_.g1_.yzxz) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.w, self_.g1_.y) * other.g0_.yzzy) - (vec4<f32>(self_.g0_.w, self_.g0_.w, self_.g0_.y, self_.g1_.x) * other.g0_.xyxx)));
    return flector_geometricProduct_flector(geometric_product, motor_reverse(self_));
}
fn motor_sandwich_horizon(self_: Motor, other: Horizon) -> Flector {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, -1.0)), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g1_.w) * vec4<f32>(-1.0, -1.0, -1.0, 1.0)));
    return flector_geometricProduct_flector(geometric_product, motor_reverse(self_));
}
fn motor_sandwich_line(self_: Motor, other: Line) -> Motor {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(((other.g0_.x * self_.g1_.w) + (other.g0_.y * self_.g1_.z) + (other.g1_.x * self_.g0_.w) + (other.g1_.y * self_.g0_.z)), ((other.g0_.y * self_.g1_.w) + (other.g0_.z * self_.g1_.x) + (other.g1_.y * self_.g0_.w) + (other.g1_.z * self_.g0_.x)), ((other.g0_.x * self_.g1_.y) + (other.g0_.z * self_.g1_.w) + (other.g1_.x * self_.g0_.y) + (other.g1_.z * self_.g0_.w)), (-(other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))) - (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.y, other.g0_.x) * self_.g1_.yzxx) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g1_.x) * self_.g0_.yzxx)), /* e23, e31, e12, scalar */ (vec4<f32>(((other.g1_.x * self_.g1_.w) + (other.g1_.y * self_.g1_.z)), ((other.g1_.y * self_.g1_.w) + (other.g1_.z * self_.g1_.x)), ((other.g1_.x * self_.g1_.y) + (other.g1_.z * self_.g1_.w)), (-(other.g1_.y * self_.g1_.y) - (other.g1_.z * self_.g1_.z))) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g1_.x) * self_.g1_.yzxx)));
    return motor_geometricProduct_motor(geometric_product, motor_reverse(self_));
}
fn motor_sandwich_motor(self_: Motor, other: Motor) -> Motor {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(((other.g0_.y * self_.g1_.z) + (other.g0_.w * self_.g1_.x) + (other.g1_.y * self_.g0_.z) + (other.g1_.w * self_.g0_.x)), ((other.g0_.z * self_.g1_.x) + (other.g0_.w * self_.g1_.y) + (other.g1_.z * self_.g0_.x) + (other.g1_.w * self_.g0_.y)), ((other.g0_.z * self_.g1_.w) + (other.g0_.w * self_.g1_.z) + (other.g1_.z * self_.g0_.w) + (other.g1_.w * self_.g0_.z)), (-(other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))) + (other.g0_.xyxw * self_.g1_.wwyw) - (other.g0_.zxyx * self_.g1_.yzxx) + (other.g1_.xyxw * self_.g0_.wwyw) - (other.g1_.zxyx * self_.g0_.yzxx)), /* e23, e31, e12, scalar */ (vec4<f32>(((other.g1_.y * self_.g1_.z) + (other.g1_.w * self_.g1_.x)), ((other.g1_.z * self_.g1_.x) + (other.g1_.w * self_.g1_.y)), ((other.g1_.z * self_.g1_.w) + (other.g1_.w * self_.g1_.z)), (-(other.g1_.y * self_.g1_.y) - (other.g1_.z * self_.g1_.z))) + (other.g1_.xyxw * self_.g1_.wwyw) - (other.g1_.zxyx * self_.g1_.yzxx)));
    return motor_geometricProduct_motor(geometric_product, motor_reverse(self_));
}
fn motor_sandwich_multiVector(self_: Motor, other: MultiVector) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(0.0, ((other.g0_.y * self_.g1_.w) - (other.g3_.x * self_.g0_.x) - (other.g3_.y * self_.g0_.y) - (other.g3_.z * self_.g0_.z))) + (vec2<f32>(other.g0_.x) * vec2<f32>(self_.g1_.w, self_.g0_.w)) - (vec2<f32>(self_.g1_.x) * vec2<f32>(other.g3_.x, other.g2_.x)) - (vec2<f32>(self_.g1_.y) * vec2<f32>(other.g3_.y, other.g2_.y)) - (vec2<f32>(self_.g1_.z) * vec2<f32>(other.g3_.z, other.g2_.z))), /* e1, e2, e3, e4 */ (vec4<f32>((self_.g1_.w * other.g1_.x), (self_.g1_.y * other.g4_.w), (self_.g1_.z * other.g4_.w), ((self_.g0_.y * other.g1_.y) + (self_.g0_.z * other.g1_.z) - (self_.g0_.w * other.g4_.w) - (self_.g1_.y * other.g4_.y) - (self_.g1_.z * other.g4_.z))) + (vec4<f32>(self_.g1_.z, self_.g1_.w, self_.g1_.w, self_.g0_.x) * other.g1_.yyzx) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g4_.x) * self_.g1_.yzxx) + (vec4<f32>(other.g4_.w, other.g1_.z, other.g1_.x, other.g1_.w) * self_.g1_.xxyw)), /* e41, e42, e43 */ ((vec3<f32>(other.g0_.x) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z)) + (vec3<f32>(other.g0_.y) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z)) - (vec3<f32>(self_.g0_.y, self_.g0_.z, self_.g0_.x) * other.g3_.zxy) + (vec3<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.w) * other.g3_.yzz) + (vec3<f32>(self_.g0_.w, self_.g0_.w, self_.g0_.y) * other.g3_.xyx) - (vec3<f32>(self_.g1_.y, self_.g1_.z, self_.g1_.x) * other.g2_.zxy) + (vec3<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.w) * other.g2_.yzz) + (vec3<f32>(self_.g1_.w, self_.g1_.w, self_.g1_.y) * other.g2_.xyx)), /* e23, e31, e12 */ ((vec3<f32>(other.g0_.x) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z)) - (vec3<f32>(self_.g1_.y, self_.g1_.z, self_.g1_.x) * other.g3_.zxy) + (vec3<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.w) * other.g3_.yzz) + (vec3<f32>(self_.g1_.w, self_.g1_.w, self_.g1_.y) * other.g3_.xyx)), /* e423, e431, e412, e321 */ (vec4<f32>((-(self_.g0_.x * other.g4_.w) + (self_.g0_.y * other.g1_.z) + (self_.g1_.z * other.g4_.y) + (self_.g1_.w * other.g4_.x)), (-(self_.g0_.y * other.g4_.w) + (self_.g0_.z * other.g1_.x) + (self_.g1_.y * other.g1_.w) + (self_.g1_.w * other.g4_.y)), ((self_.g0_.x * other.g1_.y) - (self_.g0_.z * other.g4_.w) + (self_.g1_.z * other.g1_.w) + (self_.g1_.w * other.g4_.z)), 0.0) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g1_.x) * other.g1_.yzxx) - (vec4<f32>(self_.g0_.w, self_.g0_.w, self_.g0_.w, self_.g1_.y) * other.g1_.xyzy) + (vec4<f32>(other.g1_.w, other.g4_.z, other.g4_.x, other.g4_.w) * self_.g1_.xxyw) - (vec4<f32>(other.g4_.z, other.g4_.x, other.g4_.y, other.g1_.z) * self_.g1_.yzxz)));
    return multiVector_geometricProduct_multiVector(geometric_product, motor_reverse(self_));
}
fn motor_sandwich_origin(self_: Motor, other: Origin) -> Flector {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g1_.w * other.g0_)), /* e423, e431, e412, e321 */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), 0.0));
    return flector_geometricProduct_flector(geometric_product, motor_reverse(self_));
}
fn motor_sandwich_plane(self_: Motor, other: Plane) -> Flector {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g1_.x * other.g0_.w), (self_.g1_.y * other.g0_.w), (self_.g1_.z * other.g0_.w), (-(self_.g0_.w * other.g0_.w) - (self_.g1_.x * other.g0_.x) - (self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))), /* e423, e431, e412, e321 */ vec4<f32>((-(self_.g0_.x * other.g0_.w) - (self_.g1_.y * other.g0_.z) + (self_.g1_.z * other.g0_.y) + (self_.g1_.w * other.g0_.x)), (-(self_.g0_.y * other.g0_.w) + (self_.g1_.x * other.g0_.z) - (self_.g1_.z * other.g0_.x) + (self_.g1_.w * other.g0_.y)), (-(self_.g0_.z * other.g0_.w) - (self_.g1_.x * other.g0_.y) + (self_.g1_.y * other.g0_.x) + (self_.g1_.w * other.g0_.z)), (self_.g1_.w * other.g0_.w)));
    return flector_geometricProduct_flector(geometric_product, motor_reverse(self_));
}
fn motor_sandwich_point(self_: Motor, other: Point) -> Flector {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(((self_.g1_.y * other.g0_.z) * -1.0), ((self_.g1_.z * other.g0_.x) * -1.0), ((self_.g1_.x * other.g0_.y) * -1.0), ((self_.g0_.z * other.g0_.z) + (self_.g1_.w * other.g0_.w))) + (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g0_.x) * other.g0_.yzxx) + (vec4<f32>(self_.g1_.w, self_.g1_.w, self_.g1_.w, self_.g0_.y) * other.g0_.xyzy)), /* e423, e431, e412, e321 */ (vec4<f32>(((self_.g0_.y * other.g0_.z) + (self_.g1_.x * other.g0_.w)), ((self_.g0_.z * other.g0_.x) + (self_.g1_.y * other.g0_.w)), ((self_.g0_.x * other.g0_.y) + (self_.g1_.z * other.g0_.w)), ((self_.g1_.z * other.g0_.z) * -1.0)) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g1_.x) * other.g0_.yzxx) - (vec4<f32>(self_.g0_.w, self_.g0_.w, self_.g0_.w, self_.g1_.y) * other.g0_.xyzy)));
    return flector_geometricProduct_flector(geometric_product, motor_reverse(self_));
}
fn motor_sandwich_scalar(self_: Motor, other: Scalar) -> Motor {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(other.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(other.g0_) * self_.g1_));
    return motor_geometricProduct_motor(geometric_product, motor_reverse(self_));
}
fn multiVector_sandwich_antiScalar(self_: MultiVector, other: AntiScalar) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (self_.g0_.x * other.g0_)), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g4_.w * other.g0_)), /* e41, e42, e43 */ (vec3<f32>(other.g0_) * self_.g3_), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), 0.0));
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self_));
}
fn multiVector_sandwich_dualNum(self_: MultiVector, other: DualNum) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>((other.g0_.x * self_.g0_.x), ((other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x))), /* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g1_.x), (other.g0_.x * self_.g1_.y), (other.g0_.x * self_.g1_.z), ((other.g0_.x * self_.g1_.w) + (other.g0_.y * self_.g4_.w))), /* e41, e42, e43 */ ((vec3<f32>(other.g0_.x) * self_.g2_) + (vec3<f32>(other.g0_.y) * self_.g3_)), /* e23, e31, e12 */ (vec3<f32>(other.g0_.x) * self_.g3_), /* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.x * self_.g4_.x) + (other.g0_.y * self_.g1_.x)), ((other.g0_.x * self_.g4_.y) + (other.g0_.y * self_.g1_.y)), ((other.g0_.x * self_.g4_.z) + (other.g0_.y * self_.g1_.z)), (other.g0_.x * self_.g4_.w)));
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self_));
}
fn multiVector_sandwich_flector(self_: MultiVector, other: Flector) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(0.0, (-(other.g0_.x * self_.g4_.x) - (other.g0_.y * self_.g4_.y) - (other.g0_.z * self_.g4_.z) + (other.g1_.w * self_.g1_.w))) + (vec2<f32>(self_.g1_.x) * vec2<f32>(other.g0_.x, other.g1_.x)) + (vec2<f32>(self_.g1_.y) * vec2<f32>(other.g0_.y, other.g1_.y)) + (vec2<f32>(self_.g1_.z) * vec2<f32>(other.g0_.z, other.g1_.z)) - (vec2<f32>(self_.g4_.w) * vec2<f32>(other.g1_.w, other.g0_.w))), /* e1, e2, e3, e4 */ (vec4<f32>(((self_.g3_.x * other.g1_.w) - (self_.g3_.y * other.g0_.z)), ((self_.g3_.y * other.g1_.w) - (self_.g3_.z * other.g0_.x)), (-(self_.g3_.x * other.g0_.y) + (self_.g3_.z * other.g1_.w)), (-(self_.g0_.y * other.g1_.w) + (self_.g2_.y * other.g0_.y) + (self_.g2_.z * other.g0_.z) - (self_.g3_.x * other.g1_.x) - (self_.g3_.y * other.g1_.y) - (self_.g3_.z * other.g1_.z))) + (vec4<f32>(self_.g0_.x) * other.g0_) + (vec4<f32>(self_.g3_.z, self_.g3_.x, self_.g3_.y, self_.g2_.x) * other.g0_.yzxx)), /* e41, e42, e43 */ (vec3<f32>(((other.g0_.y * self_.g4_.z) - (other.g0_.z * self_.g4_.y) - (other.g1_.y * self_.g1_.z) + (other.g1_.z * self_.g1_.y)), (-(other.g0_.x * self_.g4_.z) + (other.g0_.z * self_.g4_.x) + (other.g1_.x * self_.g1_.z) - (other.g1_.z * self_.g1_.x)), ((other.g0_.x * self_.g4_.y) - (other.g0_.y * self_.g4_.x) - (other.g1_.x * self_.g1_.y) + (other.g1_.y * self_.g1_.x))) - (vec3<f32>(other.g0_.w) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z)) + (vec3<f32>(other.g1_.w) * vec3<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z)) + (vec3<f32>(self_.g1_.w) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z)) - (vec3<f32>(self_.g4_.w) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z))), /* e23, e31, e12 */ (vec3<f32>((-(other.g0_.y * self_.g1_.z) + (other.g0_.z * self_.g1_.y)), ((other.g0_.x * self_.g1_.z) - (other.g0_.z * self_.g1_.x)), (-(other.g0_.x * self_.g1_.y) + (other.g0_.y * self_.g1_.x))) - (vec3<f32>(other.g1_.w) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z)) - (vec3<f32>(self_.g4_.w) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z))), /* e423, e431, e412, e321 */ (vec4<f32>((-(self_.g2_.x * other.g1_.w) + (self_.g2_.y * other.g0_.z) + (self_.g3_.x * other.g0_.w) - (self_.g3_.y * other.g1_.z) + (self_.g3_.z * other.g1_.y)), (-(self_.g2_.y * other.g1_.w) + (self_.g2_.z * other.g0_.x) + (self_.g3_.x * other.g1_.z) + (self_.g3_.y * other.g0_.w) - (self_.g3_.z * other.g1_.x)), ((self_.g2_.x * other.g0_.y) - (self_.g2_.z * other.g1_.w) - (self_.g3_.x * other.g1_.y) + (self_.g3_.y * other.g1_.x) + (self_.g3_.z * other.g0_.w)), ((self_.g3_.z * other.g0_.z) * -1.0)) + (vec4<f32>(self_.g0_.x) * other.g1_) - (vec4<f32>(self_.g0_.y, self_.g0_.y, self_.g0_.y, self_.g3_.x) * other.g0_.xyzx) - (vec4<f32>(self_.g2_.z, self_.g2_.x, self_.g2_.y, self_.g3_.y) * other.g0_.yzxy)));
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self_));
}
fn multiVector_sandwich_horizon(self_: MultiVector, other: Horizon) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(other.g0_) * vec2<f32>(self_.g4_.w, self_.g1_.w) * vec2<f32>(-1.0, 1.0)), /* e1, e2, e3, e4 */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g3_.x, self_.g3_.y, self_.g3_.z, self_.g0_.y) * vec4<f32>(1.0, 1.0, 1.0, -1.0)), /* e41, e42, e43 */ (vec3<f32>(other.g0_) * vec3<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z)), /* e23, e31, e12 */ (vec3<f32>(other.g0_) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z) * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g2_.x, self_.g2_.y, self_.g2_.z, self_.g0_.x) * vec4<f32>(-1.0, -1.0, -1.0, 1.0)));
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self_));
}
fn multiVector_sandwich_line(self_: MultiVector, other: Line) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(0.0, (-(other.g1_.x * self_.g2_.x) - (other.g1_.y * self_.g2_.y) - (other.g1_.z * self_.g2_.z))) - (vec2<f32>(self_.g3_.x) * vec2<f32>(other.g1_.x, other.g0_.x)) - (vec2<f32>(self_.g3_.y) * vec2<f32>(other.g1_.y, other.g0_.y)) - (vec2<f32>(self_.g3_.z) * vec2<f32>(other.g1_.z, other.g0_.z))), /* e1, e2, e3, e4 */ (vec4<f32>(((other.g1_.x * self_.g4_.w) + (other.g1_.y * self_.g1_.z)), ((other.g1_.y * self_.g4_.w) + (other.g1_.z * self_.g1_.x)), ((other.g1_.x * self_.g1_.y) + (other.g1_.z * self_.g4_.w)), (-(other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g1_.x * self_.g4_.x) - (other.g1_.y * self_.g4_.y) - (other.g1_.z * self_.g4_.z))) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.x) * self_.g1_.yzxx)), /* e41, e42, e43 */ ((vec3<f32>(self_.g0_.x) * other.g0_) + (vec3<f32>(self_.g0_.y) * other.g1_) + (other.g0_.yzx * self_.g3_.zxy) - (other.g0_.zxy * self_.g3_.yzx) + (other.g1_.yzx * self_.g2_.zxy) - (other.g1_.zxy * self_.g2_.yzx)), /* e23, e31, e12 */ ((vec3<f32>(self_.g0_.x) * other.g1_) + (other.g1_.yzx * self_.g3_.zxy) - (other.g1_.zxy * self_.g3_.yzx)), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.x * self_.g4_.w) + (other.g0_.y * self_.g1_.z) + (other.g1_.x * self_.g1_.w) + (other.g1_.y * self_.g4_.z) - (other.g1_.z * self_.g4_.y)), ((other.g0_.y * self_.g4_.w) + (other.g0_.z * self_.g1_.x) - (other.g1_.x * self_.g4_.z) + (other.g1_.y * self_.g1_.w) + (other.g1_.z * self_.g4_.x)), ((other.g0_.x * self_.g1_.y) + (other.g0_.z * self_.g4_.w) + (other.g1_.x * self_.g4_.y) - (other.g1_.y * self_.g4_.x) + (other.g1_.z * self_.g1_.w)), (-(other.g1_.y * self_.g1_.y) - (other.g1_.z * self_.g1_.z))) - (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.y, other.g1_.x) * self_.g1_.yzxx)));
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self_));
}
fn multiVector_sandwich_motor(self_: MultiVector, other: Motor) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(0.0, ((self_.g0_.y * other.g1_.w) - (self_.g3_.x * other.g0_.x) - (self_.g3_.y * other.g0_.y) - (self_.g3_.z * other.g0_.z))) + (vec2<f32>(self_.g0_.x) * vec2<f32>(other.g1_.w, other.g0_.w)) - (vec2<f32>(other.g1_.x) * vec2<f32>(self_.g3_.x, self_.g2_.x)) - (vec2<f32>(other.g1_.y) * vec2<f32>(self_.g3_.y, self_.g2_.y)) - (vec2<f32>(other.g1_.z) * vec2<f32>(self_.g3_.z, self_.g2_.z))), /* e1, e2, e3, e4 */ (vec4<f32>(((other.g1_.y * self_.g1_.z) + (other.g1_.w * self_.g1_.x)), ((other.g1_.z * self_.g1_.x) + (other.g1_.w * self_.g1_.y)), ((other.g1_.z * self_.g4_.w) + (other.g1_.w * self_.g1_.z)), (-(other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) + (other.g0_.w * self_.g4_.w) - (other.g1_.x * self_.g4_.x) - (other.g1_.y * self_.g4_.y) - (other.g1_.z * self_.g4_.z))) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.x) * self_.g1_.yzxx) + (vec4<f32>(self_.g4_.w, self_.g4_.w, self_.g1_.y, self_.g1_.w) * other.g1_.xyxw)), /* e41, e42, e43 */ ((vec3<f32>(self_.g0_.x) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z)) + (vec3<f32>(self_.g0_.y) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z)) + (vec3<f32>(other.g0_.y, other.g0_.w, other.g0_.w) * self_.g3_.zyz) - (vec3<f32>(other.g0_.z, other.g0_.x, other.g0_.y) * self_.g3_.yzx) + (vec3<f32>(other.g0_.w, other.g0_.z, other.g0_.x) * self_.g3_.xxy) + (vec3<f32>(other.g1_.y, other.g1_.w, other.g1_.w) * self_.g2_.zyz) - (vec3<f32>(other.g1_.z, other.g1_.x, other.g1_.y) * self_.g2_.yzx) + (vec3<f32>(other.g1_.w, other.g1_.z, other.g1_.x) * self_.g2_.xxy)), /* e23, e31, e12 */ ((vec3<f32>(self_.g0_.x) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z)) + (vec3<f32>(other.g1_.y, other.g1_.w, other.g1_.w) * self_.g3_.zyz) - (vec3<f32>(other.g1_.z, other.g1_.x, other.g1_.y) * self_.g3_.yzx) + (vec3<f32>(other.g1_.w, other.g1_.z, other.g1_.x) * self_.g3_.xxy)), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.y * self_.g1_.z) + (other.g0_.w * self_.g1_.x) + (other.g1_.x * self_.g1_.w) + (other.g1_.y * self_.g4_.z) + (other.g1_.w * self_.g4_.x)), ((other.g0_.z * self_.g1_.x) + (other.g0_.w * self_.g1_.y) + (other.g1_.y * self_.g1_.w) + (other.g1_.z * self_.g4_.x) + (other.g1_.w * self_.g4_.y)), ((other.g0_.x * self_.g1_.y) + (other.g0_.w * self_.g1_.z) + (other.g1_.x * self_.g4_.y) + (other.g1_.z * self_.g1_.w) + (other.g1_.w * self_.g4_.z)), ((other.g1_.z * self_.g1_.z) * -1.0)) + (vec4<f32>(self_.g4_.w) * vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.w)) - (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.y, other.g1_.x) * self_.g1_.yzxx) - (vec4<f32>(self_.g4_.y, self_.g4_.z, self_.g4_.x, self_.g1_.y) * other.g1_.zxyy)));
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self_));
}
fn multiVector_sandwich_multiVector(self_: MultiVector, other: MultiVector) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(0.0, ((other.g0_.y * self_.g0_.x) - (other.g3_.x * self_.g2_.x) - (other.g3_.y * self_.g2_.y) - (other.g3_.z * self_.g2_.z) - (other.g1_.x * self_.g4_.x) - (other.g1_.y * self_.g4_.y) - (other.g1_.z * self_.g4_.z) + (other.g4_.w * self_.g1_.w))) + (vec2<f32>(other.g0_.x) * self_.g0_) - (vec2<f32>(self_.g3_.x) * vec2<f32>(other.g3_.x, other.g2_.x)) - (vec2<f32>(self_.g3_.y) * vec2<f32>(other.g3_.y, other.g2_.y)) - (vec2<f32>(self_.g3_.z) * vec2<f32>(other.g3_.z, other.g2_.z)) + (vec2<f32>(self_.g1_.x) * vec2<f32>(other.g1_.x, other.g4_.x)) + (vec2<f32>(self_.g1_.y) * vec2<f32>(other.g1_.y, other.g4_.y)) + (vec2<f32>(self_.g1_.z) * vec2<f32>(other.g1_.z, other.g4_.z)) - (vec2<f32>(self_.g4_.w) * vec2<f32>(other.g4_.w, other.g1_.w))), /* e1, e2, e3, e4 */ (vec4<f32>(((other.g3_.y * self_.g1_.z) + (self_.g3_.x * other.g4_.w) - (self_.g3_.y * other.g1_.z)), ((other.g3_.z * self_.g1_.x) + (self_.g3_.y * other.g4_.w) - (self_.g3_.z * other.g1_.x)), ((other.g3_.x * self_.g1_.y) - (self_.g3_.x * other.g1_.y) + (self_.g3_.z * other.g4_.w)), (-(self_.g0_.y * other.g4_.w) - (other.g2_.y * self_.g1_.y) - (other.g2_.z * self_.g1_.z) - (other.g3_.x * self_.g4_.x) - (other.g3_.y * self_.g4_.y) - (other.g3_.z * self_.g4_.z) + (self_.g2_.y * other.g1_.y) + (self_.g2_.z * other.g1_.z) - (self_.g3_.x * other.g4_.x) - (self_.g3_.y * other.g4_.y) - (self_.g3_.z * other.g4_.z))) + (vec4<f32>(other.g0_.x) * self_.g1_) + (vec4<f32>(self_.g0_.x) * other.g1_) + (vec4<f32>(self_.g4_.w) * vec4<f32>(other.g3_.x, other.g3_.y, other.g3_.z, other.g0_.y)) - (vec4<f32>(other.g3_.z, other.g3_.x, other.g3_.y, other.g2_.x) * self_.g1_.yzxx) + (vec4<f32>(self_.g3_.z, self_.g3_.x, self_.g3_.y, self_.g2_.x) * other.g1_.yzxx)), /* e41, e42, e43 */ (vec3<f32>(((other.g1_.y * self_.g4_.z) - (other.g1_.z * self_.g4_.y) - (other.g4_.y * self_.g1_.z) + (other.g4_.z * self_.g1_.y)), (-(other.g1_.x * self_.g4_.z) + (other.g1_.z * self_.g4_.x) + (other.g4_.x * self_.g1_.z) - (other.g4_.z * self_.g1_.x)), ((other.g1_.x * self_.g4_.y) - (other.g1_.y * self_.g4_.x) - (other.g4_.x * self_.g1_.y) + (other.g4_.y * self_.g1_.x))) + (vec3<f32>(other.g0_.x) * self_.g2_) + (vec3<f32>(other.g0_.y) * self_.g3_) + (vec3<f32>(self_.g0_.x) * other.g2_) + (vec3<f32>(self_.g0_.y) * other.g3_) - (vec3<f32>(other.g1_.w) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z)) + (vec3<f32>(other.g4_.w) * vec3<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z)) + (vec3<f32>(self_.g1_.w) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z)) - (vec3<f32>(self_.g4_.w) * vec3<f32>(other.g4_.x, other.g4_.y, other.g4_.z)) + (other.g2_.yzx * self_.g3_.zxy) - (other.g2_.zxy * self_.g3_.yzx) + (other.g3_.yzx * self_.g2_.zxy) - (other.g3_.zxy * self_.g2_.yzx)), /* e23, e31, e12 */ (vec3<f32>((-(other.g1_.y * self_.g1_.z) + (other.g1_.z * self_.g1_.y)), ((other.g1_.x * self_.g1_.z) - (other.g1_.z * self_.g1_.x)), (-(other.g1_.x * self_.g1_.y) + (other.g1_.y * self_.g1_.x))) + (vec3<f32>(other.g0_.x) * self_.g3_) + (vec3<f32>(self_.g0_.x) * other.g3_) - (vec3<f32>(other.g4_.w) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z)) - (vec3<f32>(self_.g4_.w) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z)) + (other.g3_.yzx * self_.g3_.zxy) - (other.g3_.zxy * self_.g3_.yzx)), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.y * self_.g1_.x) + (other.g2_.x * self_.g4_.w) + (other.g2_.y * self_.g1_.z) + (other.g3_.x * self_.g1_.w) + (other.g3_.y * self_.g4_.z) - (other.g3_.z * self_.g4_.y) - (self_.g2_.x * other.g4_.w) + (self_.g2_.y * other.g1_.z) + (self_.g3_.x * other.g1_.w) - (self_.g3_.y * other.g4_.z) + (self_.g3_.z * other.g4_.y)), ((other.g0_.y * self_.g1_.y) + (other.g2_.y * self_.g4_.w) + (other.g2_.z * self_.g1_.x) - (other.g3_.x * self_.g4_.z) + (other.g3_.y * self_.g1_.w) + (other.g3_.z * self_.g4_.x) - (self_.g2_.y * other.g4_.w) + (self_.g2_.z * other.g1_.x) + (self_.g3_.x * other.g4_.z) + (self_.g3_.y * other.g1_.w) - (self_.g3_.z * other.g4_.x)), ((other.g0_.y * self_.g1_.z) + (other.g2_.x * self_.g1_.y) + (other.g2_.z * self_.g4_.w) + (other.g3_.x * self_.g4_.y) - (other.g3_.y * self_.g4_.x) + (other.g3_.z * self_.g1_.w) + (self_.g2_.x * other.g1_.y) - (self_.g2_.z * other.g4_.w) - (self_.g3_.x * other.g4_.y) + (self_.g3_.y * other.g4_.x) + (self_.g3_.z * other.g1_.w)), (-(other.g3_.y * self_.g1_.y) - (other.g3_.z * self_.g1_.z) - (self_.g3_.z * other.g1_.z))) + (vec4<f32>(other.g0_.x) * self_.g4_) + (vec4<f32>(self_.g0_.x) * other.g4_) - (vec4<f32>(self_.g0_.y, self_.g0_.y, self_.g0_.y, self_.g3_.x) * other.g1_.xyzx) - (vec4<f32>(other.g2_.z, other.g2_.x, other.g2_.y, other.g3_.x) * self_.g1_.yzxx) - (vec4<f32>(self_.g2_.z, self_.g2_.x, self_.g2_.y, self_.g3_.y) * other.g1_.yzxy)));
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self_));
}
fn multiVector_sandwich_origin(self_: MultiVector, other: Origin) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (self_.g4_.w * other.g0_ * -1.0)), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.x * other.g0_)), /* e41, e42, e43 */ (vec3<f32>(other.g0_) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z) * vec3<f32>(-1.0)), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>((self_.g3_.x * other.g0_), (self_.g3_.y * other.g0_), (self_.g3_.z * other.g0_), 0.0));
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self_));
}
fn multiVector_sandwich_plane(self_: MultiVector, other: Plane) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>((self_.g4_.w * other.g0_.w * -1.0), ((self_.g1_.x * other.g0_.x) + (self_.g1_.y * other.g0_.y) + (self_.g1_.z * other.g0_.z) + (self_.g1_.w * other.g0_.w))), /* e1, e2, e3, e4 */ vec4<f32>((self_.g3_.x * other.g0_.w), (self_.g3_.y * other.g0_.w), (self_.g3_.z * other.g0_.w), (-(self_.g0_.y * other.g0_.w) - (self_.g3_.x * other.g0_.x) - (self_.g3_.y * other.g0_.y) - (self_.g3_.z * other.g0_.z))), /* e41, e42, e43 */ (vec3<f32>(((self_.g1_.y * other.g0_.z) - (self_.g1_.z * other.g0_.y)), (-(self_.g1_.x * other.g0_.z) + (self_.g1_.z * other.g0_.x)), ((self_.g1_.x * other.g0_.y) - (self_.g1_.y * other.g0_.x))) - (vec3<f32>(self_.g4_.w) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z)) + (vec3<f32>(other.g0_.w) * vec3<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z))), /* e23, e31, e12 */ (vec3<f32>(other.g0_.w) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z) * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ vec4<f32>(((self_.g0_.x * other.g0_.x) - (self_.g2_.x * other.g0_.w) - (self_.g3_.y * other.g0_.z) + (self_.g3_.z * other.g0_.y)), ((self_.g0_.x * other.g0_.y) - (self_.g2_.y * other.g0_.w) + (self_.g3_.x * other.g0_.z) - (self_.g3_.z * other.g0_.x)), ((self_.g0_.x * other.g0_.z) - (self_.g2_.z * other.g0_.w) - (self_.g3_.x * other.g0_.y) + (self_.g3_.y * other.g0_.x)), (self_.g0_.x * other.g0_.w)));
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self_));
}
fn multiVector_sandwich_point(self_: MultiVector, other: Point) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(((self_.g1_.x * other.g0_.x) + (self_.g1_.y * other.g0_.y) + (self_.g1_.z * other.g0_.z)), (-(self_.g4_.x * other.g0_.x) - (self_.g4_.y * other.g0_.y) - (self_.g4_.z * other.g0_.z) - (self_.g4_.w * other.g0_.w))), /* e1, e2, e3, e4 */ (vec4<f32>(((self_.g3_.y * other.g0_.z) * -1.0), ((self_.g3_.z * other.g0_.x) * -1.0), ((self_.g3_.x * other.g0_.y) * -1.0), ((self_.g2_.y * other.g0_.y) + (self_.g2_.z * other.g0_.z))) + (vec4<f32>(self_.g0_.x) * other.g0_) + (vec4<f32>(self_.g3_.z, self_.g3_.x, self_.g3_.y, self_.g2_.x) * other.g0_.yzxx)), /* e41, e42, e43 */ (vec3<f32>((-(self_.g4_.y * other.g0_.z) + (self_.g4_.z * other.g0_.y)), ((self_.g4_.x * other.g0_.z) - (self_.g4_.z * other.g0_.x)), (-(self_.g4_.x * other.g0_.y) + (self_.g4_.y * other.g0_.x))) + (vec3<f32>(self_.g1_.w) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z)) - (vec3<f32>(other.g0_.w) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z))), /* e23, e31, e12 */ (vec3<f32>(((self_.g1_.y * other.g0_.z) - (self_.g1_.z * other.g0_.y)), (-(self_.g1_.x * other.g0_.z) + (self_.g1_.z * other.g0_.x)), ((self_.g1_.x * other.g0_.y) - (self_.g1_.y * other.g0_.x))) - (vec3<f32>(self_.g4_.w) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z))), /* e423, e431, e412, e321 */ (vec4<f32>(((self_.g2_.y * other.g0_.z) + (self_.g3_.x * other.g0_.w)), ((self_.g2_.z * other.g0_.x) + (self_.g3_.y * other.g0_.w)), ((self_.g2_.x * other.g0_.y) + (self_.g3_.z * other.g0_.w)), ((self_.g3_.z * other.g0_.z) * -1.0)) - (vec4<f32>(self_.g0_.y, self_.g0_.y, self_.g0_.y, self_.g3_.x) * other.g0_.xyzx) - (vec4<f32>(self_.g2_.z, self_.g2_.x, self_.g2_.y, self_.g3_.y) * other.g0_.yzxy)));
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self_));
}
fn multiVector_sandwich_scalar(self_: MultiVector, other: Scalar) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(other.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(other.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(other.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(other.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_) * self_.g4_));
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self_));
}
fn origin_sandwich_flector(self_: Origin, other: Flector) -> Flector {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.w)), /* e23, e31, e12, scalar */ vec4<f32>(0.0));
    return motor_geometricProduct_motor(geometric_product, origin_reverse(self_));
}
fn origin_sandwich_line(self_: Origin, other: Line) -> AntiScalar {
    let geometric_product: Plane = Plane(/* e423, e431, e412, e321 */ vec4<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_), 0.0));
    return plane_geometricProduct_plane(geometric_product, origin_reverse(self_));
}
fn origin_sandwich_motor(self_: Origin, other: Motor) -> Motor {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g1_.w * self_.g0_)), /* e423, e431, e412, e321 */ vec4<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_), 0.0));
    return flector_geometricProduct_flector(geometric_product, origin_reverse(self_));
}
fn origin_sandwich_multiVector(self_: Origin, other: MultiVector) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (other.g4_.w * self_.g0_)), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.x * self_.g0_)), /* e41, e42, e43 */ (vec3<f32>(self_.g0_) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z)), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>((other.g3_.x * self_.g0_), (other.g3_.y * self_.g0_), (other.g3_.z * self_.g0_), 0.0));
    return multiVector_geometricProduct_multiVector(geometric_product, origin_reverse(self_));
}
fn origin_sandwich_point(self_: Origin, other: Point) -> Plane {
    let geometric_product: Line = Line(/* e41, e42, e43 */ (vec3<f32>(self_.g0_) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z)), /* e23, e31, e12 */ vec3<f32>(0.0));
    return line_geometricProduct_line(geometric_product, origin_reverse(self_));
}
fn plane_sandwich_antiScalar(self_: Plane, other: AntiScalar) -> AntiScalar {
    let geometric_product: Origin = Origin(/* e4 */ (self_.g0_.w * other.g0_));
    return origin_geometricProduct_origin(geometric_product, plane_reverse(self_));
}
fn plane_sandwich_dualNum(self_: Plane, other: DualNum) -> Motor {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.y * self_.g0_.w)), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_.x) * self_.g0_));
    return flector_geometricProduct_flector(geometric_product, plane_reverse(self_));
}
fn plane_sandwich_flector(self_: Plane, other: Flector) -> Flector {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(((other.g0_.y * self_.g0_.z) + (other.g1_.w * self_.g0_.x)), ((other.g0_.z * self_.g0_.x) + (other.g1_.w * self_.g0_.y)), ((other.g0_.x * self_.g0_.y) + (other.g1_.w * self_.g0_.z)), (-(other.g0_.z * self_.g0_.z) - (other.g0_.w * self_.g0_.w))) - (vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, other.g0_.y) * self_.g0_.wwwy) - (other.g0_.zxyx * self_.g0_.yzxx)), /* e23, e31, e12, scalar */ (vec4<f32>(self_.g0_.w) * vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.w) * vec4<f32>(-1.0)));
    return motor_geometricProduct_motor(geometric_product, plane_reverse(self_));
}
fn plane_sandwich_horizon(self_: Plane, other: Horizon) -> Flector {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), 0.0), /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.w * other.g0_ * -1.0)));
    return motor_geometricProduct_motor(geometric_product, plane_reverse(self_));
}
fn plane_sandwich_line(self_: Plane, other: Line) -> Motor {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g1_.x * self_.g0_.w), (other.g1_.y * self_.g0_.w), (other.g1_.z * self_.g0_.w), (-(other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))), /* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.x * self_.g0_.w) + (other.g1_.y * self_.g0_.z) - (other.g1_.z * self_.g0_.y)), ((other.g0_.y * self_.g0_.w) - (other.g1_.x * self_.g0_.z) + (other.g1_.z * self_.g0_.x)), ((other.g0_.z * self_.g0_.w) + (other.g1_.x * self_.g0_.y) - (other.g1_.y * self_.g0_.x)), 0.0));
    return flector_geometricProduct_flector(geometric_product, plane_reverse(self_));
}
fn plane_sandwich_motor(self_: Plane, other: Motor) -> Motor {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g1_.x * self_.g0_.w), (other.g1_.y * self_.g0_.w), (other.g1_.z * self_.g0_.w), ((other.g0_.w * self_.g0_.w) - (other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))), /* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.x * self_.g0_.w) + (other.g1_.y * self_.g0_.z) - (other.g1_.z * self_.g0_.y) + (other.g1_.w * self_.g0_.x)), ((other.g0_.y * self_.g0_.w) - (other.g1_.x * self_.g0_.z) + (other.g1_.z * self_.g0_.x) + (other.g1_.w * self_.g0_.y)), ((other.g0_.z * self_.g0_.w) + (other.g1_.x * self_.g0_.y) - (other.g1_.y * self_.g0_.x) + (other.g1_.w * self_.g0_.z)), (other.g1_.w * self_.g0_.w)));
    return flector_geometricProduct_flector(geometric_product, plane_reverse(self_));
}
fn plane_sandwich_multiVector(self_: Plane, other: MultiVector) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>((other.g4_.w * self_.g0_.w * -1.0), (-(other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z) - (other.g1_.w * self_.g0_.w))), /* e1, e2, e3, e4 */ vec4<f32>((other.g3_.x * self_.g0_.w), (other.g3_.y * self_.g0_.w), (other.g3_.z * self_.g0_.w), ((other.g0_.y * self_.g0_.w) - (other.g3_.x * self_.g0_.x) - (other.g3_.y * self_.g0_.y) - (other.g3_.z * self_.g0_.z))), /* e41, e42, e43 */ (vec3<f32>(((other.g1_.y * self_.g0_.z) - (other.g1_.z * self_.g0_.y)), (-(other.g1_.x * self_.g0_.z) + (other.g1_.z * self_.g0_.x)), ((other.g1_.x * self_.g0_.y) - (other.g1_.y * self_.g0_.x))) + (vec3<f32>(other.g4_.w) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z)) - (vec3<f32>(self_.g0_.w) * vec3<f32>(other.g4_.x, other.g4_.y, other.g4_.z))), /* e23, e31, e12 */ (vec3<f32>(self_.g0_.w) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z) * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.x * self_.g0_.x) + (other.g2_.x * self_.g0_.w) + (other.g3_.y * self_.g0_.z) - (other.g3_.z * self_.g0_.y)), ((other.g0_.x * self_.g0_.y) + (other.g2_.y * self_.g0_.w) - (other.g3_.x * self_.g0_.z) + (other.g3_.z * self_.g0_.x)), ((other.g0_.x * self_.g0_.z) + (other.g2_.z * self_.g0_.w) + (other.g3_.x * self_.g0_.y) - (other.g3_.y * self_.g0_.x)), (other.g0_.x * self_.g0_.w)));
    return multiVector_geometricProduct_multiVector(geometric_product, plane_reverse(self_));
}
fn plane_sandwich_origin(self_: Plane, other: Origin) -> Origin {
    let geometric_product: AntiScalar = AntiScalar(/* e1234 */ (self_.g0_.w * other.g0_ * -1.0));
    return antiScalar_geometricProduct_antiScalar(geometric_product, plane_reverse(self_));
}
fn plane_sandwich_plane(self_: Plane, other: Plane) -> Flector {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>((-(other.g0_.x * self_.g0_.w) + (other.g0_.w * self_.g0_.x)), (-(other.g0_.y * self_.g0_.w) + (other.g0_.w * self_.g0_.y)), (-(other.g0_.z * self_.g0_.w) + (other.g0_.w * self_.g0_.z)), 0.0), /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.w * self_.g0_.w * -1.0)));
    return motor_geometricProduct_motor(geometric_product, plane_reverse(self_));
}
fn plane_sandwich_point(self_: Plane, other: Point) -> Flector {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>((self_.g0_.z * other.g0_.y), (self_.g0_.x * other.g0_.z), (self_.g0_.y * other.g0_.x), (-(self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z) - (self_.g0_.w * other.g0_.w))) - (self_.g0_.yzxx * other.g0_.zxyx)), /* e23, e31, e12, scalar */ vec4<f32>((self_.g0_.w * other.g0_.x * -1.0), (self_.g0_.w * other.g0_.y * -1.0), (self_.g0_.w * other.g0_.z * -1.0), 0.0));
    return motor_geometricProduct_motor(geometric_product, plane_reverse(self_));
}
fn plane_sandwich_scalar(self_: Plane, other: Scalar) -> Motor {
    let geometric_product: Plane = Plane(/* e423, e431, e412, e321 */ (vec4<f32>(other.g0_) * self_.g0_));
    return plane_geometricProduct_plane(geometric_product, plane_reverse(self_));
}
fn point_sandwich_antiScalar(self_: Point, other: AntiScalar) -> Motor {
    let geometric_product: Plane = Plane(/* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), 0.0));
    return plane_geometricProduct_plane(geometric_product, point_reverse(self_));
}
fn point_sandwich_dualNum(self_: Point, other: DualNum) -> Motor {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(other.g0_.x) * self_.g0_), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.y * self_.g0_.x), (other.g0_.y * self_.g0_.y), (other.g0_.y * self_.g0_.z), 0.0));
    return flector_geometricProduct_flector(geometric_product, point_reverse(self_));
}
fn point_sandwich_flector(self_: Point, other: Flector) -> Flector {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>((-(other.g0_.w * self_.g0_.x) - (other.g1_.y * self_.g0_.z)), (-(other.g0_.w * self_.g0_.y) - (other.g1_.z * self_.g0_.x)), (-(other.g0_.w * self_.g0_.z) - (other.g1_.x * self_.g0_.y)), ((other.g1_.z * self_.g0_.z) + (other.g1_.w * self_.g0_.w))) + (vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.x) * self_.g0_.wwwx) + (other.g1_.zxyy * self_.g0_.yzxy)), /* e23, e31, e12, scalar */ (vec4<f32>((-(other.g0_.y * self_.g0_.z) - (other.g1_.w * self_.g0_.x)), (-(other.g0_.z * self_.g0_.x) - (other.g1_.w * self_.g0_.y)), (-(other.g0_.x * self_.g0_.y) - (other.g1_.w * self_.g0_.z)), ((other.g0_.y * self_.g0_.y) + (other.g0_.z * self_.g0_.z))) + (other.g0_.zxyx * self_.g0_.yzxx)));
    return motor_geometricProduct_motor(geometric_product, point_reverse(self_));
}
fn point_sandwich_horizon(self_: Point, other: Horizon) -> Flector {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.w * other.g0_)), /* e23, e31, e12, scalar */ vec4<f32>((self_.g0_.x * other.g0_ * -1.0), (self_.g0_.y * other.g0_ * -1.0), (self_.g0_.z * other.g0_ * -1.0), 0.0));
    return motor_geometricProduct_motor(geometric_product, point_reverse(self_));
}
fn point_sandwich_line(self_: Point, other: Line) -> Motor {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>((other.g1_.y * self_.g0_.z), (other.g1_.z * self_.g0_.x), (other.g1_.x * self_.g0_.y), (-(other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z))) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.x) * self_.g0_.yzxx)), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.y * self_.g0_.z) + (other.g1_.x * self_.g0_.w)), ((other.g0_.z * self_.g0_.x) + (other.g1_.y * self_.g0_.w)), ((other.g0_.x * self_.g0_.y) + (other.g1_.z * self_.g0_.w)), (-(other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))) - (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.y, other.g1_.x) * self_.g0_.yzxx)));
    return flector_geometricProduct_flector(geometric_product, point_reverse(self_));
}
fn point_sandwich_motor(self_: Point, other: Motor) -> Motor {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>((other.g1_.w * self_.g0_.x), (other.g1_.w * self_.g0_.y), (other.g1_.w * self_.g0_.z), (-(other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z))) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.x) * self_.g0_.yzxx) + (other.g1_.yzxw * self_.g0_.zxyw)), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.y * self_.g0_.z) + (other.g0_.w * self_.g0_.x) + (other.g1_.x * self_.g0_.w)), ((other.g0_.z * self_.g0_.x) + (other.g0_.w * self_.g0_.y) + (other.g1_.y * self_.g0_.w)), ((other.g0_.x * self_.g0_.y) + (other.g0_.w * self_.g0_.z) + (other.g1_.z * self_.g0_.w)), (-(other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))) - (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.y, other.g1_.x) * self_.g0_.yzxx)));
    return flector_geometricProduct_flector(geometric_product, point_reverse(self_));
}
fn point_sandwich_multiVector(self_: Point, other: MultiVector) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(0.0, (other.g4_.w * self_.g0_.w)) + (vec2<f32>(self_.g0_.x) * vec2<f32>(other.g1_.x, other.g4_.x)) + (vec2<f32>(self_.g0_.y) * vec2<f32>(other.g1_.y, other.g4_.y)) + (vec2<f32>(self_.g0_.z) * vec2<f32>(other.g1_.z, other.g4_.z))), /* e1, e2, e3, e4 */ (vec4<f32>((other.g3_.y * self_.g0_.z), (other.g3_.z * self_.g0_.x), (other.g3_.x * self_.g0_.y), (-(other.g2_.y * self_.g0_.y) - (other.g2_.z * self_.g0_.z))) + (vec4<f32>(other.g0_.x) * self_.g0_) - (vec4<f32>(other.g3_.z, other.g3_.x, other.g3_.y, other.g2_.x) * self_.g0_.yzxx)), /* e41, e42, e43 */ (vec3<f32>((-(other.g4_.y * self_.g0_.z) + (other.g4_.z * self_.g0_.y)), ((other.g4_.x * self_.g0_.z) - (other.g4_.z * self_.g0_.x)), (-(other.g4_.x * self_.g0_.y) + (other.g4_.y * self_.g0_.x))) - (vec3<f32>(other.g1_.w) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z)) + (vec3<f32>(self_.g0_.w) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z))), /* e23, e31, e12 */ (vec3<f32>((-(other.g1_.y * self_.g0_.z) + (other.g1_.z * self_.g0_.y)), ((other.g1_.x * self_.g0_.z) - (other.g1_.z * self_.g0_.x)), (-(other.g1_.x * self_.g0_.y) + (other.g1_.y * self_.g0_.x))) - (vec3<f32>(other.g4_.w) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z))), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.y * self_.g0_.x) + (other.g2_.y * self_.g0_.z) + (other.g3_.x * self_.g0_.w)), ((other.g0_.y * self_.g0_.y) + (other.g2_.z * self_.g0_.x) + (other.g3_.y * self_.g0_.w)), ((other.g0_.y * self_.g0_.z) + (other.g2_.x * self_.g0_.y) + (other.g3_.z * self_.g0_.w)), (-(other.g3_.y * self_.g0_.y) - (other.g3_.z * self_.g0_.z))) - (vec4<f32>(other.g2_.z, other.g2_.x, other.g2_.y, other.g3_.x) * self_.g0_.yzxx)));
    return multiVector_geometricProduct_multiVector(geometric_product, point_reverse(self_));
}
fn point_sandwich_origin(self_: Point, other: Origin) -> Flector {
    let geometric_product: Line = Line(/* e41, e42, e43 */ (vec3<f32>(other.g0_) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z) * vec3<f32>(-1.0)), /* e23, e31, e12 */ vec3<f32>(0.0));
    return line_geometricProduct_line(geometric_product, point_reverse(self_));
}
fn point_sandwich_plane(self_: Point, other: Plane) -> Flector {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(((other.g0_.y * self_.g0_.z) * -1.0), ((other.g0_.z * self_.g0_.x) * -1.0), ((other.g0_.x * self_.g0_.y) * -1.0), ((other.g0_.y * self_.g0_.y) + (other.g0_.z * self_.g0_.z) + (other.g0_.w * self_.g0_.w))) + (other.g0_.zxyx * self_.g0_.yzxx)), /* e23, e31, e12, scalar */ vec4<f32>((other.g0_.w * self_.g0_.x * -1.0), (other.g0_.w * self_.g0_.y * -1.0), (other.g0_.w * self_.g0_.z * -1.0), 0.0));
    return motor_geometricProduct_motor(geometric_product, point_reverse(self_));
}
fn point_sandwich_point(self_: Point, other: Point) -> Flector {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>(((other.g0_.x * self_.g0_.w) - (other.g0_.w * self_.g0_.x)), ((other.g0_.y * self_.g0_.w) - (other.g0_.w * self_.g0_.y)), ((other.g0_.z * self_.g0_.w) - (other.g0_.w * self_.g0_.z)), 0.0), /* e23, e31, e12, scalar */ (vec4<f32>(((other.g0_.y * self_.g0_.z) * -1.0), ((other.g0_.z * self_.g0_.x) * -1.0), ((other.g0_.x * self_.g0_.y) * -1.0), ((other.g0_.y * self_.g0_.y) + (other.g0_.z * self_.g0_.z))) + (other.g0_.zxyx * self_.g0_.yzxx)));
    return motor_geometricProduct_motor(geometric_product, point_reverse(self_));
}
fn point_sandwich_scalar(self_: Point, other: Scalar) -> Motor {
    let geometric_product: Point = Point(/* e1, e2, e3, e4 */ (vec4<f32>(other.g0_) * self_.g0_));
    return point_geometricProduct_point(geometric_product, point_reverse(self_));
}
fn scalar_sandwich_antiScalar(self_: Scalar, other: AntiScalar) -> AntiScalar {
    let geometric_product: AntiScalar = AntiScalar(/* e1234 */ (other.g0_ * self_.g0_));
    return antiScalar_geometricProduct_antiScalar(geometric_product, scalar_reverse(self_));
}
fn scalar_sandwich_dualNum(self_: Scalar, other: DualNum) -> DualNum {
    let geometric_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>(self_.g0_) * other.g0_));
    return dualNum_geometricProduct_dualNum(geometric_product, scalar_reverse(self_));
}
fn scalar_sandwich_flector(self_: Scalar, other: Flector) -> Flector {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_) * other.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_) * other.g1_));
    return flector_geometricProduct_flector(geometric_product, scalar_reverse(self_));
}
fn scalar_sandwich_horizon(self_: Scalar, other: Horizon) -> Horizon {
    let geometric_product: Horizon = Horizon(/* e321 */ (other.g0_ * self_.g0_));
    return horizon_geometricProduct_horizon(geometric_product, scalar_reverse(self_));
}
fn scalar_sandwich_line(self_: Scalar, other: Line) -> Line {
    let geometric_product: Line = Line(/* e41, e42, e43 */ (vec3<f32>(self_.g0_) * other.g0_), /* e23, e31, e12 */ (vec3<f32>(self_.g0_) * other.g1_));
    return line_geometricProduct_line(geometric_product, scalar_reverse(self_));
}
fn scalar_sandwich_motor(self_: Scalar, other: Motor) -> Motor {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(self_.g0_) * other.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(self_.g0_) * other.g1_));
    return motor_geometricProduct_motor(geometric_product, scalar_reverse(self_));
}
fn scalar_sandwich_multiVector(self_: Scalar, other: MultiVector) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(self_.g0_) * other.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_) * other.g1_), /* e41, e42, e43 */ (vec3<f32>(self_.g0_) * other.g2_), /* e23, e31, e12 */ (vec3<f32>(self_.g0_) * other.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_) * other.g4_));
    return multiVector_geometricProduct_multiVector(geometric_product, scalar_reverse(self_));
}
fn scalar_sandwich_origin(self_: Scalar, other: Origin) -> Origin {
    let geometric_product: Origin = Origin(/* e4 */ (other.g0_ * self_.g0_));
    return origin_geometricProduct_origin(geometric_product, scalar_reverse(self_));
}
fn scalar_sandwich_plane(self_: Scalar, other: Plane) -> Plane {
    let geometric_product: Plane = Plane(/* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_) * other.g0_));
    return plane_geometricProduct_plane(geometric_product, scalar_reverse(self_));
}
fn scalar_sandwich_point(self_: Scalar, other: Point) -> Point {
    let geometric_product: Point = Point(/* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_) * other.g0_));
    return point_geometricProduct_point(geometric_product, scalar_reverse(self_));
}
fn scalar_sandwich_scalar(self_: Scalar, other: Scalar) -> Scalar {
    let geometric_product: Scalar = Scalar(/* scalar */ (other.g0_ * self_.g0_));
    return scalar_geometricProduct_scalar(geometric_product, scalar_reverse(self_));
}
fn scalar_squareRoot(self_: Scalar) -> Scalar {
    return Scalar(/* scalar */ pow(self_.g0_.x, 0.5));
}
fn antiScalar_sub_antiScalar(self_: AntiScalar, other: AntiScalar) -> AntiScalar {
    return AntiScalar(/* e1234 */ (-other.g0_ + self_.g0_));
}
fn antiScalar_sub_dualNum(self_: AntiScalar, other: DualNum) -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>((other.g0_.x * -1.0), (-other.g0_.y + self_.g0_)));
}
fn antiScalar_sub_flector(self_: AntiScalar, other: Flector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, self_.g0_), /* e1, e2, e3, e4 */ (other.g0_ * vec4<f32>(-1.0)), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ (other.g1_ * vec4<f32>(-1.0)));
}
fn antiScalar_sub_horizon(self_: AntiScalar, other: Horizon) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, self_.g0_), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_ * -1.0)));
}
fn antiScalar_sub_line(self_: AntiScalar, other: Line) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g0_.x * -1.0), (other.g0_.y * -1.0), (other.g0_.z * -1.0), self_.g0_), /* e23, e31, e12, scalar */ vec4<f32>((other.g1_.x * -1.0), (other.g1_.y * -1.0), (other.g1_.z * -1.0), 0.0));
}
fn antiScalar_sub_motor(self_: AntiScalar, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g0_.x * -1.0), (other.g0_.y * -1.0), (other.g0_.z * -1.0), (-other.g0_.w + self_.g0_)), /* e23, e31, e12, scalar */ (other.g1_ * vec4<f32>(-1.0)));
}
fn antiScalar_sub_multiVector(self_: AntiScalar, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((other.g0_.x * -1.0), (-other.g0_.y + self_.g0_)), /* e1, e2, e3, e4 */ (other.g1_ * vec4<f32>(-1.0)), /* e41, e42, e43 */ (other.g2_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (other.g3_ * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ (other.g4_ * vec4<f32>(-1.0)));
}
fn antiScalar_sub_origin(self_: AntiScalar, other: Origin) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, self_.g0_), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_ * -1.0)), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn antiScalar_sub_plane(self_: AntiScalar, other: Plane) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, self_.g0_), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ (other.g0_ * vec4<f32>(-1.0)));
}
fn antiScalar_sub_point(self_: AntiScalar, other: Point) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, self_.g0_), /* e1, e2, e3, e4 */ (other.g0_ * vec4<f32>(-1.0)), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn antiScalar_sub_scalar(self_: AntiScalar, other: Scalar) -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>((other.g0_ * -1.0), self_.g0_));
}
fn dualNum_sub_antiScalar(self_: DualNum, other: AntiScalar) -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>(self_.g0_.x, (self_.g0_.y - other.g0_)));
}
fn dualNum_sub_dualNum(self_: DualNum, other: DualNum) -> DualNum {
    return DualNum(/* scalar, e1234 */ (-other.g0_ + self_.g0_));
}
fn dualNum_sub_flector(self_: DualNum, other: Flector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ (other.g0_ * vec4<f32>(-1.0)), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ (other.g1_ * vec4<f32>(-1.0)));
}
fn dualNum_sub_horizon(self_: DualNum, other: Horizon) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_ * -1.0)));
}
fn dualNum_sub_line(self_: DualNum, other: Line) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g0_.x * -1.0), (other.g0_.y * -1.0), (other.g0_.z * -1.0), self_.g0_.y), /* e23, e31, e12, scalar */ vec4<f32>((other.g1_.x * -1.0), (other.g1_.y * -1.0), (other.g1_.z * -1.0), self_.g0_.x));
}
fn dualNum_sub_motor(self_: DualNum, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g0_.x * -1.0), (other.g0_.y * -1.0), (other.g0_.z * -1.0), (self_.g0_.y - other.g0_.w)), /* e23, e31, e12, scalar */ vec4<f32>((other.g1_.x * -1.0), (other.g1_.y * -1.0), (other.g1_.z * -1.0), (self_.g0_.x - other.g1_.w)));
}
fn dualNum_sub_multiVector(self_: DualNum, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (self_.g0_ - other.g0_), /* e1, e2, e3, e4 */ (other.g1_ * vec4<f32>(-1.0)), /* e41, e42, e43 */ (other.g2_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (other.g3_ * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ (other.g4_ * vec4<f32>(-1.0)));
}
fn dualNum_sub_origin(self_: DualNum, other: Origin) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_ * -1.0)), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn dualNum_sub_plane(self_: DualNum, other: Plane) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ (other.g0_ * vec4<f32>(-1.0)));
}
fn dualNum_sub_point(self_: DualNum, other: Point) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ (other.g0_ * vec4<f32>(-1.0)), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn dualNum_sub_scalar(self_: DualNum, other: Scalar) -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>((self_.g0_.x - other.g0_), self_.g0_.y));
}
fn flector_sub_antiScalar(self_: Flector, other: AntiScalar) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (other.g0_ * -1.0)), /* e1, e2, e3, e4 */ self_.g0_, /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ self_.g1_);
}
fn flector_sub_dualNum(self_: Flector, other: DualNum) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (other.g0_ * vec2<f32>(-1.0)), /* e1, e2, e3, e4 */ self_.g0_, /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ self_.g1_);
}
fn flector_sub_flector(self_: Flector, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (-other.g0_ + self_.g0_), /* e423, e431, e412, e321 */ (-other.g1_ + self_.g1_));
}
fn flector_sub_horizon(self_: Flector, other: Horizon) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ self_.g0_, /* e423, e431, e412, e321 */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, (self_.g1_.w - other.g0_)));
}
fn flector_sub_line(self_: Flector, other: Line) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ self_.g0_, /* e41, e42, e43 */ (other.g0_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (other.g1_ * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ self_.g1_);
}
fn flector_sub_motor(self_: Flector, other: Motor) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(other.g1_.w, other.g0_.w) * vec2<f32>(-1.0)), /* e1, e2, e3, e4 */ self_.g0_, /* e41, e42, e43 */ (vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z) * vec3<f32>(-1.0)), /* e23, e31, e12 */ (vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z) * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ self_.g1_);
}
fn flector_sub_multiVector(self_: Flector, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (other.g0_ * vec2<f32>(-1.0)), /* e1, e2, e3, e4 */ (self_.g0_ - other.g1_), /* e41, e42, e43 */ (other.g2_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (other.g3_ * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ (self_.g1_ - other.g4_));
}
fn flector_sub_origin(self_: Flector, other: Origin) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, (self_.g0_.w - other.g0_)), /* e423, e431, e412, e321 */ self_.g1_);
}
fn flector_sub_plane(self_: Flector, other: Plane) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ self_.g0_, /* e423, e431, e412, e321 */ (self_.g1_ - other.g0_));
}
fn flector_sub_point(self_: Flector, other: Point) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (self_.g0_ - other.g0_), /* e423, e431, e412, e321 */ self_.g1_);
}
fn flector_sub_scalar(self_: Flector, other: Scalar) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((other.g0_ * -1.0), 0.0), /* e1, e2, e3, e4 */ self_.g0_, /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ self_.g1_);
}
fn horizon_sub_antiScalar(self_: Horizon, other: AntiScalar) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (other.g0_ * -1.0)), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_));
}
fn horizon_sub_dualNum(self_: Horizon, other: DualNum) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (other.g0_ * vec2<f32>(-1.0)), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_));
}
fn horizon_sub_flector(self_: Horizon, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (other.g0_ * vec4<f32>(-1.0)), /* e423, e431, e412, e321 */ vec4<f32>((other.g1_.x * -1.0), (other.g1_.y * -1.0), (other.g1_.z * -1.0), (-other.g1_.w + self_.g0_)));
}
fn horizon_sub_horizon(self_: Horizon, other: Horizon) -> Horizon {
    return Horizon(/* e321 */ (-other.g0_ + self_.g0_));
}
fn horizon_sub_line(self_: Horizon, other: Line) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ (other.g0_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (other.g1_ * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_));
}
fn horizon_sub_motor(self_: Horizon, other: Motor) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(other.g1_.w, other.g0_.w) * vec2<f32>(-1.0)), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ (vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z) * vec3<f32>(-1.0)), /* e23, e31, e12 */ (vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z) * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_));
}
fn horizon_sub_multiVector(self_: Horizon, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (other.g0_ * vec2<f32>(-1.0)), /* e1, e2, e3, e4 */ (other.g1_ * vec4<f32>(-1.0)), /* e41, e42, e43 */ (other.g2_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (other.g3_ * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ vec4<f32>((other.g4_.x * -1.0), (other.g4_.y * -1.0), (other.g4_.z * -1.0), (-other.g4_.w + self_.g0_)));
}
fn horizon_sub_origin(self_: Horizon, other: Origin) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_ * -1.0)), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_));
}
fn horizon_sub_plane(self_: Horizon, other: Plane) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>((other.g0_.x * -1.0), (other.g0_.y * -1.0), (other.g0_.z * -1.0), (-other.g0_.w + self_.g0_)));
}
fn horizon_sub_point(self_: Horizon, other: Point) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (other.g0_ * vec4<f32>(-1.0)), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_));
}
fn horizon_sub_scalar(self_: Horizon, other: Scalar) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((other.g0_ * -1.0), 0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_));
}
fn line_sub_antiScalar(self_: Line, other: AntiScalar) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, (other.g0_ * -1.0)), /* e23, e31, e12, scalar */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, 0.0));
}
fn line_sub_dualNum(self_: Line, other: DualNum) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, (other.g0_.y * -1.0)), /* e23, e31, e12, scalar */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, (other.g0_.x * -1.0)));
}
fn line_sub_flector(self_: Line, other: Flector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ (other.g0_ * vec4<f32>(-1.0)), /* e41, e42, e43 */ self_.g0_, /* e23, e31, e12 */ self_.g1_, /* e423, e431, e412, e321 */ (other.g1_ * vec4<f32>(-1.0)));
}
fn line_sub_horizon(self_: Line, other: Horizon) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ self_.g0_, /* e23, e31, e12 */ self_.g1_, /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_ * -1.0)));
}
fn line_sub_line(self_: Line, other: Line) -> Line {
    return Line(/* e41, e42, e43 */ (-other.g0_ + self_.g0_), /* e23, e31, e12 */ (-other.g1_ + self_.g1_));
}
fn line_sub_motor(self_: Line, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g0_.x - other.g0_.x), (self_.g0_.y - other.g0_.y), (self_.g0_.z - other.g0_.z), (other.g0_.w * -1.0)), /* e23, e31, e12, scalar */ vec4<f32>((self_.g1_.x - other.g1_.x), (self_.g1_.y - other.g1_.y), (self_.g1_.z - other.g1_.z), (other.g1_.w * -1.0)));
}
fn line_sub_multiVector(self_: Line, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (other.g0_ * vec2<f32>(-1.0)), /* e1, e2, e3, e4 */ (other.g1_ * vec4<f32>(-1.0)), /* e41, e42, e43 */ (self_.g0_ - other.g2_), /* e23, e31, e12 */ (self_.g1_ - other.g3_), /* e423, e431, e412, e321 */ (other.g4_ * vec4<f32>(-1.0)));
}
fn line_sub_origin(self_: Line, other: Origin) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_ * -1.0)), /* e41, e42, e43 */ self_.g0_, /* e23, e31, e12 */ self_.g1_, /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn line_sub_plane(self_: Line, other: Plane) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ self_.g0_, /* e23, e31, e12 */ self_.g1_, /* e423, e431, e412, e321 */ (other.g0_ * vec4<f32>(-1.0)));
}
fn line_sub_point(self_: Line, other: Point) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ (other.g0_ * vec4<f32>(-1.0)), /* e41, e42, e43 */ self_.g0_, /* e23, e31, e12 */ self_.g1_, /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn line_sub_scalar(self_: Line, other: Scalar) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, 0.0), /* e23, e31, e12, scalar */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, (other.g0_ * -1.0)));
}
fn motor_sub_antiScalar(self_: Motor, other: AntiScalar) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, (self_.g0_.w - other.g0_)), /* e23, e31, e12, scalar */ self_.g1_);
}
fn motor_sub_dualNum(self_: Motor, other: DualNum) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, (-other.g0_.y + self_.g0_.w)), /* e23, e31, e12, scalar */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, (-other.g0_.x + self_.g1_.w)));
}
fn motor_sub_flector(self_: Motor, other: Flector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g1_.w, self_.g0_.w), /* e1, e2, e3, e4 */ (other.g0_ * vec4<f32>(-1.0)), /* e41, e42, e43 */ vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z), /* e23, e31, e12 */ vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z), /* e423, e431, e412, e321 */ (other.g1_ * vec4<f32>(-1.0)));
}
fn motor_sub_horizon(self_: Motor, other: Horizon) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g1_.w, self_.g0_.w), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z), /* e23, e31, e12 */ vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_ * -1.0)));
}
fn motor_sub_line(self_: Motor, other: Line) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((-other.g0_.x + self_.g0_.x), (-other.g0_.y + self_.g0_.y), (-other.g0_.z + self_.g0_.z), self_.g0_.w), /* e23, e31, e12, scalar */ vec4<f32>((-other.g1_.x + self_.g1_.x), (-other.g1_.y + self_.g1_.y), (-other.g1_.z + self_.g1_.z), self_.g1_.w));
}
fn motor_sub_motor(self_: Motor, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (-other.g0_ + self_.g0_), /* e23, e31, e12, scalar */ (-other.g1_ + self_.g1_));
}
fn motor_sub_multiVector(self_: Motor, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(self_.g1_.w, self_.g0_.w) - other.g0_), /* e1, e2, e3, e4 */ (other.g1_ * vec4<f32>(-1.0)), /* e41, e42, e43 */ (vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z) - other.g2_), /* e23, e31, e12 */ (vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z) - other.g3_), /* e423, e431, e412, e321 */ (other.g4_ * vec4<f32>(-1.0)));
}
fn motor_sub_origin(self_: Motor, other: Origin) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g1_.w, self_.g0_.w), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_ * -1.0)), /* e41, e42, e43 */ vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z), /* e23, e31, e12 */ vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn motor_sub_plane(self_: Motor, other: Plane) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g1_.w, self_.g0_.w), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z), /* e23, e31, e12 */ vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z), /* e423, e431, e412, e321 */ (other.g0_ * vec4<f32>(-1.0)));
}
fn motor_sub_point(self_: Motor, other: Point) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g1_.w, self_.g0_.w), /* e1, e2, e3, e4 */ (other.g0_ * vec4<f32>(-1.0)), /* e41, e42, e43 */ vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z), /* e23, e31, e12 */ vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn motor_sub_scalar(self_: Motor, other: Scalar) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ self_.g0_, /* e23, e31, e12, scalar */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, (self_.g1_.w - other.g0_)));
}
fn multiVector_sub_antiScalar(self_: MultiVector, other: AntiScalar) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g0_.x, (self_.g0_.y - other.g0_)), /* e1, e2, e3, e4 */ self_.g1_, /* e41, e42, e43 */ self_.g2_, /* e23, e31, e12 */ self_.g3_, /* e423, e431, e412, e321 */ self_.g4_);
}
fn multiVector_sub_dualNum(self_: MultiVector, other: DualNum) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (-other.g0_ + self_.g0_), /* e1, e2, e3, e4 */ self_.g1_, /* e41, e42, e43 */ self_.g2_, /* e23, e31, e12 */ self_.g3_, /* e423, e431, e412, e321 */ self_.g4_);
}
fn multiVector_sub_flector(self_: MultiVector, other: Flector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ (-other.g0_ + self_.g1_), /* e41, e42, e43 */ self_.g2_, /* e23, e31, e12 */ self_.g3_, /* e423, e431, e412, e321 */ (-other.g1_ + self_.g4_));
}
fn multiVector_sub_horizon(self_: MultiVector, other: Horizon) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ self_.g1_, /* e41, e42, e43 */ self_.g2_, /* e23, e31, e12 */ self_.g3_, /* e423, e431, e412, e321 */ vec4<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z, (self_.g4_.w - other.g0_)));
}
fn multiVector_sub_line(self_: MultiVector, other: Line) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ self_.g1_, /* e41, e42, e43 */ (-other.g0_ + self_.g2_), /* e23, e31, e12 */ (-other.g1_ + self_.g3_), /* e423, e431, e412, e321 */ self_.g4_);
}
fn multiVector_sub_motor(self_: MultiVector, other: Motor) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (vec2<f32>((other.g1_.w * -1.0), (other.g0_.w * -1.0)) + self_.g0_), /* e1, e2, e3, e4 */ self_.g1_, /* e41, e42, e43 */ (vec3<f32>((other.g0_.x * -1.0), (other.g0_.y * -1.0), (other.g0_.z * -1.0)) + self_.g2_), /* e23, e31, e12 */ (vec3<f32>((other.g1_.x * -1.0), (other.g1_.y * -1.0), (other.g1_.z * -1.0)) + self_.g3_), /* e423, e431, e412, e321 */ self_.g4_);
}
fn multiVector_sub_multiVector(self_: MultiVector, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (-other.g0_ + self_.g0_), /* e1, e2, e3, e4 */ (-other.g1_ + self_.g1_), /* e41, e42, e43 */ (-other.g2_ + self_.g2_), /* e23, e31, e12 */ (-other.g3_ + self_.g3_), /* e423, e431, e412, e321 */ (-other.g4_ + self_.g4_));
}
fn multiVector_sub_origin(self_: MultiVector, other: Origin) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, (self_.g1_.w - other.g0_)), /* e41, e42, e43 */ self_.g2_, /* e23, e31, e12 */ self_.g3_, /* e423, e431, e412, e321 */ self_.g4_);
}
fn multiVector_sub_plane(self_: MultiVector, other: Plane) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ self_.g1_, /* e41, e42, e43 */ self_.g2_, /* e23, e31, e12 */ self_.g3_, /* e423, e431, e412, e321 */ (self_.g4_ - other.g0_));
}
fn multiVector_sub_point(self_: MultiVector, other: Point) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ (self_.g1_ - other.g0_), /* e41, e42, e43 */ self_.g2_, /* e23, e31, e12 */ self_.g3_, /* e423, e431, e412, e321 */ self_.g4_);
}
fn multiVector_sub_scalar(self_: MultiVector, other: Scalar) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((self_.g0_.x - other.g0_), self_.g0_.y), /* e1, e2, e3, e4 */ self_.g1_, /* e41, e42, e43 */ self_.g2_, /* e23, e31, e12 */ self_.g3_, /* e423, e431, e412, e321 */ self_.g4_);
}
fn origin_sub_antiScalar(self_: Origin, other: AntiScalar) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (other.g0_ * -1.0)), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn origin_sub_dualNum(self_: Origin, other: DualNum) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (other.g0_ * vec2<f32>(-1.0)), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn origin_sub_flector(self_: Origin, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * -1.0), (other.g0_.y * -1.0), (other.g0_.z * -1.0), (-other.g0_.w + self_.g0_)), /* e423, e431, e412, e321 */ (other.g1_ * vec4<f32>(-1.0)));
}
fn origin_sub_horizon(self_: Origin, other: Horizon) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_ * -1.0)));
}
fn origin_sub_line(self_: Origin, other: Line) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_), /* e41, e42, e43 */ (other.g0_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (other.g1_ * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn origin_sub_motor(self_: Origin, other: Motor) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(other.g1_.w, other.g0_.w) * vec2<f32>(-1.0)), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_), /* e41, e42, e43 */ (vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z) * vec3<f32>(-1.0)), /* e23, e31, e12 */ (vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z) * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn origin_sub_multiVector(self_: Origin, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (other.g0_ * vec2<f32>(-1.0)), /* e1, e2, e3, e4 */ vec4<f32>((other.g1_.x * -1.0), (other.g1_.y * -1.0), (other.g1_.z * -1.0), (-other.g1_.w + self_.g0_)), /* e41, e42, e43 */ (other.g2_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (other.g3_ * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ (other.g4_ * vec4<f32>(-1.0)));
}
fn origin_sub_origin(self_: Origin, other: Origin) -> Origin {
    return Origin(/* e4 */ (-other.g0_ + self_.g0_));
}
fn origin_sub_plane(self_: Origin, other: Plane) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_), /* e423, e431, e412, e321 */ (other.g0_ * vec4<f32>(-1.0)));
}
fn origin_sub_point(self_: Origin, other: Point) -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * -1.0), (other.g0_.y * -1.0), (other.g0_.z * -1.0), (-other.g0_.w + self_.g0_)));
}
fn origin_sub_scalar(self_: Origin, other: Scalar) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((other.g0_ * -1.0), 0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn plane_sub_antiScalar(self_: Plane, other: AntiScalar) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (other.g0_ * -1.0)), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ self_.g0_);
}
fn plane_sub_dualNum(self_: Plane, other: DualNum) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (other.g0_ * vec2<f32>(-1.0)), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ self_.g0_);
}
fn plane_sub_flector(self_: Plane, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (other.g0_ * vec4<f32>(-1.0)), /* e423, e431, e412, e321 */ (-other.g1_ + self_.g0_));
}
fn plane_sub_horizon(self_: Plane, other: Horizon) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, (self_.g0_.w - other.g0_)));
}
fn plane_sub_line(self_: Plane, other: Line) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ (other.g0_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (other.g1_ * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ self_.g0_);
}
fn plane_sub_motor(self_: Plane, other: Motor) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(other.g1_.w, other.g0_.w) * vec2<f32>(-1.0)), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ (vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z) * vec3<f32>(-1.0)), /* e23, e31, e12 */ (vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z) * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ self_.g0_);
}
fn plane_sub_multiVector(self_: Plane, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (other.g0_ * vec2<f32>(-1.0)), /* e1, e2, e3, e4 */ (other.g1_ * vec4<f32>(-1.0)), /* e41, e42, e43 */ (other.g2_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (other.g3_ * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ (-other.g4_ + self_.g0_));
}
fn plane_sub_origin(self_: Plane, other: Origin) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_ * -1.0)), /* e423, e431, e412, e321 */ self_.g0_);
}
fn plane_sub_plane(self_: Plane, other: Plane) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ (-other.g0_ + self_.g0_));
}
fn plane_sub_point(self_: Plane, other: Point) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (other.g0_ * vec4<f32>(-1.0)), /* e423, e431, e412, e321 */ self_.g0_);
}
fn plane_sub_scalar(self_: Plane, other: Scalar) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((other.g0_ * -1.0), 0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ self_.g0_);
}
fn point_sub_antiScalar(self_: Point, other: AntiScalar) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (other.g0_ * -1.0)), /* e1, e2, e3, e4 */ self_.g0_, /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn point_sub_dualNum(self_: Point, other: DualNum) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (other.g0_ * vec2<f32>(-1.0)), /* e1, e2, e3, e4 */ self_.g0_, /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn point_sub_flector(self_: Point, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (-other.g0_ + self_.g0_), /* e423, e431, e412, e321 */ (other.g1_ * vec4<f32>(-1.0)));
}
fn point_sub_horizon(self_: Point, other: Horizon) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ self_.g0_, /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_ * -1.0)));
}
fn point_sub_line(self_: Point, other: Line) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ self_.g0_, /* e41, e42, e43 */ (other.g0_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (other.g1_ * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn point_sub_motor(self_: Point, other: Motor) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(other.g1_.w, other.g0_.w) * vec2<f32>(-1.0)), /* e1, e2, e3, e4 */ self_.g0_, /* e41, e42, e43 */ (vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z) * vec3<f32>(-1.0)), /* e23, e31, e12 */ (vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z) * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn point_sub_multiVector(self_: Point, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (other.g0_ * vec2<f32>(-1.0)), /* e1, e2, e3, e4 */ (-other.g1_ + self_.g0_), /* e41, e42, e43 */ (other.g2_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (other.g3_ * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ (other.g4_ * vec4<f32>(-1.0)));
}
fn point_sub_origin(self_: Point, other: Origin) -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, (self_.g0_.w - other.g0_)));
}
fn point_sub_plane(self_: Point, other: Plane) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ self_.g0_, /* e423, e431, e412, e321 */ (other.g0_ * vec4<f32>(-1.0)));
}
fn point_sub_point(self_: Point, other: Point) -> Point {
    return Point(/* e1, e2, e3, e4 */ (-other.g0_ + self_.g0_));
}
fn point_sub_scalar(self_: Point, other: Scalar) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((other.g0_ * -1.0), 0.0), /* e1, e2, e3, e4 */ self_.g0_, /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn scalar_sub_antiScalar(self_: Scalar, other: AntiScalar) -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>(self_.g0_, (other.g0_ * -1.0)));
}
fn scalar_sub_dualNum(self_: Scalar, other: DualNum) -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>((-other.g0_.x + self_.g0_), (other.g0_.y * -1.0)));
}
fn scalar_sub_flector(self_: Scalar, other: Flector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g0_, 0.0), /* e1, e2, e3, e4 */ (other.g0_ * vec4<f32>(-1.0)), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ (other.g1_ * vec4<f32>(-1.0)));
}
fn scalar_sub_horizon(self_: Scalar, other: Horizon) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g0_, 0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_ * -1.0)));
}
fn scalar_sub_line(self_: Scalar, other: Line) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g0_.x * -1.0), (other.g0_.y * -1.0), (other.g0_.z * -1.0), 0.0), /* e23, e31, e12, scalar */ vec4<f32>((other.g1_.x * -1.0), (other.g1_.y * -1.0), (other.g1_.z * -1.0), self_.g0_));
}
fn scalar_sub_motor(self_: Scalar, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (other.g0_ * vec4<f32>(-1.0)), /* e23, e31, e12, scalar */ vec4<f32>((other.g1_.x * -1.0), (other.g1_.y * -1.0), (other.g1_.z * -1.0), (-other.g1_.w + self_.g0_)));
}
fn scalar_sub_multiVector(self_: Scalar, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((-other.g0_.x + self_.g0_), (other.g0_.y * -1.0)), /* e1, e2, e3, e4 */ (other.g1_ * vec4<f32>(-1.0)), /* e41, e42, e43 */ (other.g2_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (other.g3_ * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ (other.g4_ * vec4<f32>(-1.0)));
}
fn scalar_sub_origin(self_: Scalar, other: Origin) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g0_, 0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_ * -1.0)), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn scalar_sub_plane(self_: Scalar, other: Plane) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g0_, 0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ (other.g0_ * vec4<f32>(-1.0)));
}
fn scalar_sub_point(self_: Scalar, other: Point) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g0_, 0.0), /* e1, e2, e3, e4 */ (other.g0_ * vec4<f32>(-1.0)), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn scalar_sub_scalar(self_: Scalar, other: Scalar) -> Scalar {
    return Scalar(/* scalar */ (-other.g0_ + self_.g0_));
}
fn dualNum_tryInto(self_: DualNum) -> AntiScalar {
    return AntiScalar(/* e1234 */ self_.g0_.y);
}
fn dualNum_tryInto(self_: DualNum) -> Scalar {
    return Scalar(/* scalar */ self_.g0_.x);
}
fn flector_tryInto(self_: Flector) -> Horizon {
    return Horizon(/* e321 */ self_.g1_.w);
}
fn flector_tryInto(self_: Flector) -> Origin {
    return Origin(/* e4 */ self_.g0_.w);
}
fn flector_tryInto(self_: Flector) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g1_.w));
}
fn flector_tryInto(self_: Flector) -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g0_.w));
}
fn motor_tryInto(self_: Motor) -> AntiScalar {
    return AntiScalar(/* e1234 */ self_.g0_.w);
}
fn motor_tryInto(self_: Motor) -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>(self_.g1_.w, self_.g0_.w));
}
fn motor_tryInto(self_: Motor) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z), /* e23, e31, e12 */ vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z));
}
fn motor_tryInto(self_: Motor) -> Scalar {
    return Scalar(/* scalar */ self_.g1_.w);
}
fn multiVector_tryInto(self_: MultiVector) -> AntiScalar {
    return AntiScalar(/* e1234 */ self_.g0_.y);
}
fn multiVector_tryInto(self_: MultiVector) -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>(self_.g0_.x, self_.g0_.y));
}
fn multiVector_tryInto(self_: MultiVector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g1_.w), /* e423, e431, e412, e321 */ vec4<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z, self_.g4_.w));
}
fn multiVector_tryInto(self_: MultiVector) -> Horizon {
    return Horizon(/* e321 */ self_.g4_.w);
}
fn multiVector_tryInto(self_: MultiVector) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>(self_.g2_.x, self_.g2_.y, self_.g2_.z), /* e23, e31, e12 */ vec3<f32>(self_.g3_.x, self_.g3_.y, self_.g3_.z));
}
fn multiVector_tryInto(self_: MultiVector) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(self_.g2_.x, self_.g2_.y, self_.g2_.z, self_.g0_.y), /* e23, e31, e12, scalar */ vec4<f32>(self_.g3_.x, self_.g3_.y, self_.g3_.z, self_.g0_.x));
}
fn multiVector_tryInto(self_: MultiVector) -> Origin {
    return Origin(/* e4 */ self_.g1_.w);
}
fn multiVector_tryInto(self_: MultiVector) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z, self_.g4_.w));
}
fn multiVector_tryInto(self_: MultiVector) -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g1_.w));
}
fn multiVector_tryInto(self_: MultiVector) -> Scalar {
    return Scalar(/* scalar */ self_.g0_.x);
}
fn plane_tryInto(self_: Plane) -> Horizon {
    return Horizon(/* e321 */ self_.g0_.w);
}
fn point_tryInto(self_: Point) -> Origin {
    return Origin(/* e4 */ self_.g0_.w);
}
fn antiScalar_unit() -> AntiScalar {
    return AntiScalar(/* e1234 */ 1.0);
}
fn dualNum_unit() -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>(1.0));
}
fn flector_unit() -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(1.0), /* e423, e431, e412, e321 */ vec4<f32>(1.0));
}
fn horizon_unit() -> Horizon {
    return Horizon(/* e321 */ 1.0);
}
fn line_unit() -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>(1.0), /* e23, e31, e12 */ vec3<f32>(1.0));
}
fn motor_unit() -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(1.0), /* e23, e31, e12, scalar */ vec4<f32>(1.0));
}
fn multiVector_unit() -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(1.0), /* e1, e2, e3, e4 */ vec4<f32>(1.0), /* e41, e42, e43 */ vec3<f32>(1.0), /* e23, e31, e12 */ vec3<f32>(1.0), /* e423, e431, e412, e321 */ vec4<f32>(1.0));
}
fn origin_unit() -> Origin {
    return Origin(/* e4 */ 1.0);
}
fn plane_unit() -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>(1.0));
}
fn point_unit() -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>(1.0));
}
fn scalar_unit() -> Scalar {
    return Scalar(/* scalar */ 1.0);
}
fn antiScalar_wedge_dualNum(self_: AntiScalar, other: DualNum) -> AntiScalar {
    return AntiScalar(/* e1234 */ (other.g0_.x * self_.g0_));
}
fn antiScalar_wedge_motor(self_: AntiScalar, other: Motor) -> AntiScalar {
    return AntiScalar(/* e1234 */ (other.g1_.w * self_.g0_));
}
fn antiScalar_wedge_multiVector(self_: AntiScalar, other: MultiVector) -> AntiScalar {
    return AntiScalar(/* e1234 */ (other.g0_.x * self_.g0_));
}
fn antiScalar_wedge_scalar(self_: AntiScalar, other: Scalar) -> AntiScalar {
    return AntiScalar(/* e1234 */ (self_.g0_ * other.g0_));
}
fn dualNum_wedge_antiScalar(self_: DualNum, other: AntiScalar) -> AntiScalar {
    return AntiScalar(/* e1234 */ (self_.g0_.x * other.g0_));
}
fn dualNum_wedge_dualNum(self_: DualNum, other: DualNum) -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>((other.g0_.x * self_.g0_.x), ((other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x))));
}
fn dualNum_wedge_flector(self_: DualNum, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_.x) * other.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_.x) * other.g1_));
}
fn dualNum_wedge_horizon(self_: DualNum, other: Horizon) -> Horizon {
    return Horizon(/* e321 */ (self_.g0_.x * other.g0_));
}
fn dualNum_wedge_line(self_: DualNum, other: Line) -> Line {
    return Line(/* e41, e42, e43 */ (vec3<f32>(self_.g0_.x) * other.g0_), /* e23, e31, e12 */ (vec3<f32>(self_.g0_.x) * other.g1_));
}
fn dualNum_wedge_motor(self_: DualNum, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g0_.x * other.g0_.x), (self_.g0_.x * other.g0_.y), (self_.g0_.x * other.g0_.z), ((self_.g0_.x * other.g0_.w) + (self_.g0_.y * other.g1_.w))), /* e23, e31, e12, scalar */ (vec4<f32>(self_.g0_.x) * other.g1_));
}
fn dualNum_wedge_multiVector(self_: DualNum, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((self_.g0_.x * other.g0_.x), ((self_.g0_.x * other.g0_.y) + (self_.g0_.y * other.g0_.x))), /* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_.x) * other.g1_), /* e41, e42, e43 */ (vec3<f32>(self_.g0_.x) * other.g2_), /* e23, e31, e12 */ (vec3<f32>(self_.g0_.x) * other.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_.x) * other.g4_));
}
fn dualNum_wedge_origin(self_: DualNum, other: Origin) -> Origin {
    return Origin(/* e4 */ (self_.g0_.x * other.g0_));
}
fn dualNum_wedge_plane(self_: DualNum, other: Plane) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_.x) * other.g0_));
}
fn dualNum_wedge_point(self_: DualNum, other: Point) -> Point {
    return Point(/* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_.x) * other.g0_));
}
fn dualNum_wedge_scalar(self_: DualNum, other: Scalar) -> DualNum {
    return DualNum(/* scalar, e1234 */ (vec2<f32>(other.g0_) * self_.g0_));
}
fn flector_wedge_dualNum(self_: Flector, other: DualNum) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(other.g0_.x) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_.x) * self_.g1_));
}
fn flector_wedge_flector(self_: Flector, other: Flector) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(0.0, 0.0, 0.0, (-(other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g0_.w * self_.g1_.w) + (other.g1_.y * self_.g0_.y) + (other.g1_.z * self_.g0_.z) + (other.g1_.w * self_.g0_.w))) + (vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.x) * self_.g0_.wwwx) - (vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g1_.x) * other.g0_.wwwx)), /* e23, e31, e12, scalar */ vec4<f32>((-(other.g0_.y * self_.g0_.z) + (other.g0_.z * self_.g0_.y)), ((other.g0_.x * self_.g0_.z) - (other.g0_.z * self_.g0_.x)), (-(other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x)), 0.0));
}
fn flector_wedge_horizon(self_: Flector, other: Horizon) -> AntiScalar {
    return AntiScalar(/* e1234 */ (self_.g0_.w * other.g0_));
}
fn flector_wedge_line(self_: Flector, other: Line) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.y * self_.g0_.z) + (other.g1_.x * self_.g0_.w)), ((other.g0_.z * self_.g0_.x) + (other.g1_.y * self_.g0_.w)), ((other.g0_.x * self_.g0_.y) + (other.g1_.z * self_.g0_.w)), (-(other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))) - (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.y, other.g1_.x) * self_.g0_.yzxx)));
}
fn flector_wedge_motor(self_: Flector, other: Motor) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(other.g1_.w) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(((self_.g0_.z * other.g0_.y) + (self_.g1_.x * other.g1_.w)), ((self_.g0_.x * other.g0_.z) + (self_.g1_.y * other.g1_.w)), ((self_.g0_.y * other.g0_.x) + (self_.g1_.z * other.g1_.w)), (-(self_.g0_.y * other.g1_.y) - (self_.g0_.z * other.g1_.z))) + (vec4<f32>(self_.g0_.w, self_.g0_.w, self_.g0_.w, self_.g1_.w) * other.g1_) - (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.y, other.g1_.x) * self_.g0_.yzxx)));
}
fn flector_wedge_multiVector(self_: Flector, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, ((self_.g0_.x * other.g4_.x) + (self_.g0_.y * other.g4_.y) + (self_.g0_.z * other.g4_.z) + (self_.g0_.w * other.g4_.w) - (self_.g1_.x * other.g1_.x) - (self_.g1_.y * other.g1_.y) - (self_.g1_.z * other.g1_.z) - (self_.g1_.w * other.g1_.w))), /* e1, e2, e3, e4 */ (vec4<f32>(other.g0_.x) * self_.g0_), /* e41, e42, e43 */ ((vec3<f32>(self_.g0_.w) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z)) - (vec3<f32>(other.g1_.w) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z))), /* e23, e31, e12 */ vec3<f32>(((self_.g0_.y * other.g1_.z) - (self_.g0_.z * other.g1_.y)), (-(self_.g0_.x * other.g1_.z) + (self_.g0_.z * other.g1_.x)), ((self_.g0_.x * other.g1_.y) - (self_.g0_.y * other.g1_.x))), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g2_.y * self_.g0_.z) + (other.g3_.x * self_.g0_.w)), ((other.g2_.z * self_.g0_.x) + (other.g3_.y * self_.g0_.w)), ((other.g2_.x * self_.g0_.y) + (other.g3_.z * self_.g0_.w)), (-(other.g3_.y * self_.g0_.y) - (other.g3_.z * self_.g0_.z))) + (vec4<f32>(other.g0_.x) * self_.g1_) - (vec4<f32>(other.g2_.z, other.g2_.x, other.g2_.y, other.g3_.x) * self_.g0_.yzxx)));
}
fn flector_wedge_origin(self_: Flector, other: Origin) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g1_.w) * vec4<f32>(-1.0)), /* e23, e31, e12, scalar */ vec4<f32>(0.0));
}
fn flector_wedge_plane(self_: Flector, other: Plane) -> AntiScalar {
    return AntiScalar(/* e1234 */ ((self_.g0_.x * other.g0_.x) + (self_.g0_.y * other.g0_.y) + (self_.g0_.z * other.g0_.z) + (self_.g0_.w * other.g0_.w)));
}
fn flector_wedge_point(self_: Flector, other: Point) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>((self_.g0_.w * other.g0_.x), (self_.g0_.w * other.g0_.y), (self_.g0_.w * other.g0_.z), (-(self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z) - (self_.g1_.w * other.g0_.w))) - (vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g1_.x) * other.g0_.wwwx)), /* e23, e31, e12, scalar */ vec4<f32>(((self_.g0_.y * other.g0_.z) - (self_.g0_.z * other.g0_.y)), (-(self_.g0_.x * other.g0_.z) + (self_.g0_.z * other.g0_.x)), ((self_.g0_.x * other.g0_.y) - (self_.g0_.y * other.g0_.x)), 0.0));
}
fn flector_wedge_scalar(self_: Flector, other: Scalar) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(other.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_) * self_.g1_));
}
fn horizon_wedge_dualNum(self_: Horizon, other: DualNum) -> Horizon {
    return Horizon(/* e321 */ (other.g0_.x * self_.g0_));
}
fn horizon_wedge_flector(self_: Horizon, other: Flector) -> AntiScalar {
    return AntiScalar(/* e1234 */ (other.g0_.w * self_.g0_ * -1.0));
}
fn horizon_wedge_motor(self_: Horizon, other: Motor) -> Horizon {
    return Horizon(/* e321 */ (other.g1_.w * self_.g0_));
}
fn horizon_wedge_multiVector(self_: Horizon, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (other.g1_.w * self_.g0_ * -1.0)), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.x * self_.g0_)));
}
fn horizon_wedge_origin(self_: Horizon, other: Origin) -> AntiScalar {
    return AntiScalar(/* e1234 */ (self_.g0_ * other.g0_ * -1.0));
}
fn horizon_wedge_point(self_: Horizon, other: Point) -> AntiScalar {
    return AntiScalar(/* e1234 */ (other.g0_.w * self_.g0_ * -1.0));
}
fn horizon_wedge_scalar(self_: Horizon, other: Scalar) -> Horizon {
    return Horizon(/* e321 */ (self_.g0_ * other.g0_));
}
fn line_wedge_dualNum(self_: Line, other: DualNum) -> Line {
    return Line(/* e41, e42, e43 */ (vec3<f32>(other.g0_.x) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(other.g0_.x) * self_.g1_));
}
fn line_wedge_flector(self_: Line, other: Flector) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ (vec4<f32>(((self_.g0_.y * other.g0_.z) + (self_.g1_.x * other.g0_.w)), ((self_.g0_.z * other.g0_.x) + (self_.g1_.y * other.g0_.w)), ((self_.g0_.x * other.g0_.y) + (self_.g1_.z * other.g0_.w)), (-(self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g1_.x) * other.g0_.yzxx)));
}
fn line_wedge_line(self_: Line, other: Line) -> AntiScalar {
    return AntiScalar(/* e1234 */ (-(other.g0_.x * self_.g1_.x) - (other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z)));
}
fn line_wedge_motor(self_: Line, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g0_.x * other.g1_.w), (self_.g0_.y * other.g1_.w), (self_.g0_.z * other.g1_.w), (-(self_.g0_.x * other.g1_.x) - (self_.g0_.y * other.g1_.y) - (self_.g0_.z * other.g1_.z) - (self_.g1_.x * other.g0_.x) - (self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))), /* e23, e31, e12, scalar */ vec4<f32>((self_.g1_.x * other.g1_.w), (self_.g1_.y * other.g1_.w), (self_.g1_.z * other.g1_.w), 0.0));
}
fn line_wedge_multiVector(self_: Line, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (-(self_.g0_.x * other.g3_.x) - (self_.g0_.y * other.g3_.y) - (self_.g0_.z * other.g3_.z) - (self_.g1_.x * other.g2_.x) - (self_.g1_.y * other.g2_.y) - (self_.g1_.z * other.g2_.z))), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ (vec3<f32>(other.g0_.x) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(other.g0_.x) * self_.g1_), /* e423, e431, e412, e321 */ (vec4<f32>(((self_.g0_.y * other.g1_.z) + (self_.g1_.x * other.g1_.w)), ((self_.g0_.z * other.g1_.x) + (self_.g1_.y * other.g1_.w)), ((self_.g0_.x * other.g1_.y) + (self_.g1_.z * other.g1_.w)), (-(self_.g1_.y * other.g1_.y) - (self_.g1_.z * other.g1_.z))) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g1_.x) * other.g1_.yzxx)));
}
fn line_wedge_origin(self_: Line, other: Origin) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), 0.0));
}
fn line_wedge_point(self_: Line, other: Point) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ (vec4<f32>(((self_.g0_.y * other.g0_.z) + (self_.g1_.x * other.g0_.w)), ((self_.g0_.z * other.g0_.x) + (self_.g1_.y * other.g0_.w)), ((self_.g0_.x * other.g0_.y) + (self_.g1_.z * other.g0_.w)), (-(self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g1_.x) * other.g0_.yzxx)));
}
fn line_wedge_scalar(self_: Line, other: Scalar) -> Line {
    return Line(/* e41, e42, e43 */ (vec3<f32>(other.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(other.g0_) * self_.g1_));
}
fn motor_wedge_antiScalar(self_: Motor, other: AntiScalar) -> AntiScalar {
    return AntiScalar(/* e1234 */ (self_.g1_.w * other.g0_));
}
fn motor_wedge_dualNum(self_: Motor, other: DualNum) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g0_.x * self_.g0_.x), (other.g0_.x * self_.g0_.y), (other.g0_.x * self_.g0_.z), ((other.g0_.x * self_.g0_.w) + (other.g0_.y * self_.g1_.w))), /* e23, e31, e12, scalar */ (vec4<f32>(other.g0_.x) * self_.g1_));
}
fn motor_wedge_flector(self_: Motor, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(self_.g1_.w) * other.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.z * self_.g0_.y) + (other.g1_.x * self_.g1_.w)), ((other.g0_.x * self_.g0_.z) + (other.g1_.y * self_.g1_.w)), ((other.g0_.y * self_.g0_.x) + (other.g1_.z * self_.g1_.w)), (-(other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z))) + (vec4<f32>(other.g0_.w, other.g0_.w, other.g0_.w, other.g1_.w) * self_.g1_) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g1_.x) * other.g0_.yzxx)));
}
fn motor_wedge_horizon(self_: Motor, other: Horizon) -> Horizon {
    return Horizon(/* e321 */ (self_.g1_.w * other.g0_));
}
fn motor_wedge_line(self_: Motor, other: Line) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g0_.x * self_.g1_.w), (other.g0_.y * self_.g1_.w), (other.g0_.z * self_.g1_.w), (-(other.g0_.x * self_.g1_.x) - (other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))), /* e23, e31, e12, scalar */ vec4<f32>((other.g1_.x * self_.g1_.w), (other.g1_.y * self_.g1_.w), (other.g1_.z * self_.g1_.w), 0.0));
}
fn motor_wedge_motor(self_: Motor, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(0.0, 0.0, 0.0, (-(other.g0_.x * self_.g1_.x) - (other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))) + (vec4<f32>(other.g1_.w) * self_.g0_) + (vec4<f32>(self_.g1_.w) * other.g0_)), /* e23, e31, e12, scalar */ vec4<f32>(((other.g1_.x * self_.g1_.w) + (other.g1_.w * self_.g1_.x)), ((other.g1_.y * self_.g1_.w) + (other.g1_.w * self_.g1_.y)), ((other.g1_.z * self_.g1_.w) + (other.g1_.w * self_.g1_.z)), (other.g1_.w * self_.g1_.w)));
}
fn motor_wedge_multiVector(self_: Motor, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((other.g0_.x * self_.g1_.w), ((other.g0_.x * self_.g0_.w) + (other.g0_.y * self_.g1_.w) - (other.g2_.x * self_.g1_.x) - (other.g2_.y * self_.g1_.y) - (other.g2_.z * self_.g1_.z) - (other.g3_.x * self_.g0_.x) - (other.g3_.y * self_.g0_.y) - (other.g3_.z * self_.g0_.z))), /* e1, e2, e3, e4 */ (vec4<f32>(self_.g1_.w) * other.g1_), /* e41, e42, e43 */ ((vec3<f32>(other.g0_.x) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z)) + (vec3<f32>(self_.g1_.w) * other.g2_)), /* e23, e31, e12 */ ((vec3<f32>(other.g0_.x) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z)) + (vec3<f32>(self_.g1_.w) * other.g3_)), /* e423, e431, e412, e321 */ (vec4<f32>(((self_.g0_.y * other.g1_.z) + (self_.g1_.w * other.g4_.x)), ((self_.g0_.z * other.g1_.x) + (self_.g1_.w * other.g4_.y)), ((self_.g0_.x * other.g1_.y) + (self_.g1_.w * other.g4_.z)), (-(self_.g1_.y * other.g1_.y) - (self_.g1_.z * other.g1_.z))) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g1_.x) * other.g1_.yzxx) + (vec4<f32>(other.g1_.w, other.g1_.w, other.g1_.w, other.g4_.w) * self_.g1_)));
}
fn motor_wedge_origin(self_: Motor, other: Origin) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g1_.w * other.g0_)), /* e423, e431, e412, e321 */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), 0.0));
}
fn motor_wedge_plane(self_: Motor, other: Plane) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ (vec4<f32>(self_.g1_.w) * other.g0_));
}
fn motor_wedge_point(self_: Motor, other: Point) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(self_.g1_.w) * other.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(((self_.g0_.y * other.g0_.z) + (self_.g1_.x * other.g0_.w)), ((self_.g0_.z * other.g0_.x) + (self_.g1_.y * other.g0_.w)), ((self_.g0_.x * other.g0_.y) + (self_.g1_.z * other.g0_.w)), (-(self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g1_.x) * other.g0_.yzxx)));
}
fn motor_wedge_scalar(self_: Motor, other: Scalar) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(other.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(other.g0_) * self_.g1_));
}
fn multiVector_wedge_antiScalar(self_: MultiVector, other: AntiScalar) -> AntiScalar {
    return AntiScalar(/* e1234 */ (self_.g0_.x * other.g0_));
}
fn multiVector_wedge_dualNum(self_: MultiVector, other: DualNum) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((other.g0_.x * self_.g0_.x), ((other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x))), /* e1, e2, e3, e4 */ (vec4<f32>(other.g0_.x) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(other.g0_.x) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(other.g0_.x) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_.x) * self_.g4_));
}
fn multiVector_wedge_flector(self_: MultiVector, other: Flector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (-(other.g0_.x * self_.g4_.x) - (other.g0_.y * self_.g4_.y) - (other.g0_.z * self_.g4_.z) - (other.g0_.w * self_.g4_.w) + (other.g1_.x * self_.g1_.x) + (other.g1_.y * self_.g1_.y) + (other.g1_.z * self_.g1_.z) + (other.g1_.w * self_.g1_.w))), /* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_.x) * other.g0_), /* e41, e42, e43 */ (-(vec3<f32>(other.g0_.w) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z)) + (vec3<f32>(self_.g1_.w) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z))), /* e23, e31, e12 */ vec3<f32>((-(other.g0_.y * self_.g1_.z) + (other.g0_.z * self_.g1_.y)), ((other.g0_.x * self_.g1_.z) - (other.g0_.z * self_.g1_.x)), (-(other.g0_.x * self_.g1_.y) + (other.g0_.y * self_.g1_.x))), /* e423, e431, e412, e321 */ (vec4<f32>(((self_.g2_.y * other.g0_.z) + (self_.g3_.x * other.g0_.w)), ((self_.g2_.z * other.g0_.x) + (self_.g3_.y * other.g0_.w)), ((self_.g2_.x * other.g0_.y) + (self_.g3_.z * other.g0_.w)), (-(self_.g3_.y * other.g0_.y) - (self_.g3_.z * other.g0_.z))) + (vec4<f32>(self_.g0_.x) * other.g1_) - (vec4<f32>(self_.g2_.z, self_.g2_.x, self_.g2_.y, self_.g3_.x) * other.g0_.yzxx)));
}
fn multiVector_wedge_horizon(self_: MultiVector, other: Horizon) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (self_.g1_.w * other.g0_)), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.x * other.g0_)));
}
fn multiVector_wedge_line(self_: MultiVector, other: Line) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (-(other.g0_.x * self_.g3_.x) - (other.g0_.y * self_.g3_.y) - (other.g0_.z * self_.g3_.z) - (other.g1_.x * self_.g2_.x) - (other.g1_.y * self_.g2_.y) - (other.g1_.z * self_.g2_.z))), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ (vec3<f32>(self_.g0_.x) * other.g0_), /* e23, e31, e12 */ (vec3<f32>(self_.g0_.x) * other.g1_), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.y * self_.g1_.z) + (other.g1_.x * self_.g1_.w)), ((other.g0_.z * self_.g1_.x) + (other.g1_.y * self_.g1_.w)), ((other.g0_.x * self_.g1_.y) + (other.g1_.z * self_.g1_.w)), (-(other.g1_.y * self_.g1_.y) - (other.g1_.z * self_.g1_.z))) - (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.y, other.g1_.x) * self_.g1_.yzxx)));
}
fn multiVector_wedge_motor(self_: MultiVector, other: Motor) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((self_.g0_.x * other.g1_.w), ((self_.g0_.x * other.g0_.w) + (self_.g0_.y * other.g1_.w) - (self_.g2_.x * other.g1_.x) - (self_.g2_.y * other.g1_.y) - (self_.g2_.z * other.g1_.z) - (self_.g3_.x * other.g0_.x) - (self_.g3_.y * other.g0_.y) - (self_.g3_.z * other.g0_.z))), /* e1, e2, e3, e4 */ (vec4<f32>(other.g1_.w) * self_.g1_), /* e41, e42, e43 */ ((vec3<f32>(self_.g0_.x) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z)) + (vec3<f32>(other.g1_.w) * self_.g2_)), /* e23, e31, e12 */ ((vec3<f32>(self_.g0_.x) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z)) + (vec3<f32>(other.g1_.w) * self_.g3_)), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.y * self_.g1_.z) + (other.g1_.w * self_.g4_.x)), ((other.g0_.z * self_.g1_.x) + (other.g1_.w * self_.g4_.y)), ((other.g0_.x * self_.g1_.y) + (other.g1_.w * self_.g4_.z)), (-(other.g1_.y * self_.g1_.y) - (other.g1_.z * self_.g1_.z))) - (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.y, other.g1_.x) * self_.g1_.yzxx) + (vec4<f32>(self_.g1_.w, self_.g1_.w, self_.g1_.w, self_.g4_.w) * other.g1_)));
}
fn multiVector_wedge_multiVector(self_: MultiVector, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((other.g0_.x * self_.g0_.x), ((other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x) - (other.g2_.x * self_.g3_.x) - (other.g2_.y * self_.g3_.y) - (other.g2_.z * self_.g3_.z) - (other.g3_.x * self_.g2_.x) - (other.g3_.y * self_.g2_.y) - (other.g3_.z * self_.g2_.z) - (other.g1_.x * self_.g4_.x) - (other.g1_.y * self_.g4_.y) - (other.g1_.z * self_.g4_.z) - (other.g1_.w * self_.g4_.w) + (other.g4_.x * self_.g1_.x) + (other.g4_.y * self_.g1_.y) + (other.g4_.z * self_.g1_.z) + (other.g4_.w * self_.g1_.w))), /* e1, e2, e3, e4 */ ((vec4<f32>(other.g0_.x) * self_.g1_) + (vec4<f32>(self_.g0_.x) * other.g1_)), /* e41, e42, e43 */ ((vec3<f32>(other.g0_.x) * self_.g2_) + (vec3<f32>(self_.g0_.x) * other.g2_) - (vec3<f32>(other.g1_.w) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z)) + (vec3<f32>(self_.g1_.w) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z))), /* e23, e31, e12 */ (vec3<f32>((-(other.g1_.y * self_.g1_.z) + (other.g1_.z * self_.g1_.y)), ((other.g1_.x * self_.g1_.z) - (other.g1_.z * self_.g1_.x)), (-(other.g1_.x * self_.g1_.y) + (other.g1_.y * self_.g1_.x))) + (vec3<f32>(other.g0_.x) * self_.g3_) + (vec3<f32>(self_.g0_.x) * other.g3_)), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g2_.y * self_.g1_.z) + (other.g3_.x * self_.g1_.w) + (self_.g2_.y * other.g1_.z) + (self_.g3_.x * other.g1_.w)), ((other.g2_.z * self_.g1_.x) + (other.g3_.y * self_.g1_.w) + (self_.g2_.z * other.g1_.x) + (self_.g3_.y * other.g1_.w)), ((other.g2_.x * self_.g1_.y) + (other.g3_.z * self_.g1_.w) + (self_.g2_.x * other.g1_.y) + (self_.g3_.z * other.g1_.w)), (-(other.g3_.y * self_.g1_.y) - (other.g3_.z * self_.g1_.z) - (self_.g3_.y * other.g1_.y) - (self_.g3_.z * other.g1_.z))) + (vec4<f32>(other.g0_.x) * self_.g4_) + (vec4<f32>(self_.g0_.x) * other.g4_) - (vec4<f32>(other.g2_.z, other.g2_.x, other.g2_.y, other.g3_.x) * self_.g1_.yzxx) - (vec4<f32>(self_.g2_.z, self_.g2_.x, self_.g2_.y, self_.g3_.x) * other.g1_.yzxx)));
}
fn multiVector_wedge_origin(self_: MultiVector, other: Origin) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (self_.g4_.w * other.g0_ * -1.0)), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.x * other.g0_)), /* e41, e42, e43 */ (vec3<f32>(other.g0_) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z) * vec3<f32>(-1.0)), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>((self_.g3_.x * other.g0_), (self_.g3_.y * other.g0_), (self_.g3_.z * other.g0_), 0.0));
}
fn multiVector_wedge_plane(self_: MultiVector, other: Plane) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, ((self_.g1_.x * other.g0_.x) + (self_.g1_.y * other.g0_.y) + (self_.g1_.z * other.g0_.z) + (self_.g1_.w * other.g0_.w))), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_.x) * other.g0_));
}
fn multiVector_wedge_point(self_: MultiVector, other: Point) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (-(self_.g4_.x * other.g0_.x) - (self_.g4_.y * other.g0_.y) - (self_.g4_.z * other.g0_.z) - (self_.g4_.w * other.g0_.w))), /* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_.x) * other.g0_), /* e41, e42, e43 */ ((vec3<f32>(self_.g1_.w) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z)) - (vec3<f32>(other.g0_.w) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z))), /* e23, e31, e12 */ vec3<f32>(((self_.g1_.y * other.g0_.z) - (self_.g1_.z * other.g0_.y)), (-(self_.g1_.x * other.g0_.z) + (self_.g1_.z * other.g0_.x)), ((self_.g1_.x * other.g0_.y) - (self_.g1_.y * other.g0_.x))), /* e423, e431, e412, e321 */ (vec4<f32>(((self_.g2_.y * other.g0_.z) + (self_.g3_.x * other.g0_.w)), ((self_.g2_.z * other.g0_.x) + (self_.g3_.y * other.g0_.w)), ((self_.g2_.x * other.g0_.y) + (self_.g3_.z * other.g0_.w)), (-(self_.g3_.y * other.g0_.y) - (self_.g3_.z * other.g0_.z))) - (vec4<f32>(self_.g2_.z, self_.g2_.x, self_.g2_.y, self_.g3_.x) * other.g0_.yzxx)));
}
fn multiVector_wedge_scalar(self_: MultiVector, other: Scalar) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(other.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(other.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(other.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(other.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_) * self_.g4_));
}
fn origin_wedge_dualNum(self_: Origin, other: DualNum) -> Origin {
    return Origin(/* e4 */ (other.g0_.x * self_.g0_));
}
fn origin_wedge_flector(self_: Origin, other: Flector) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.w)), /* e23, e31, e12, scalar */ vec4<f32>(0.0));
}
fn origin_wedge_horizon(self_: Origin, other: Horizon) -> AntiScalar {
    return AntiScalar(/* e1234 */ (other.g0_ * self_.g0_));
}
fn origin_wedge_line(self_: Origin, other: Line) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_), 0.0));
}
fn origin_wedge_motor(self_: Origin, other: Motor) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g1_.w * self_.g0_)), /* e423, e431, e412, e321 */ vec4<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_), 0.0));
}
fn origin_wedge_multiVector(self_: Origin, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (other.g4_.w * self_.g0_)), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.x * self_.g0_)), /* e41, e42, e43 */ (vec3<f32>(self_.g0_) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z)), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>((other.g3_.x * self_.g0_), (other.g3_.y * self_.g0_), (other.g3_.z * self_.g0_), 0.0));
}
fn origin_wedge_plane(self_: Origin, other: Plane) -> AntiScalar {
    return AntiScalar(/* e1234 */ (other.g0_.w * self_.g0_));
}
fn origin_wedge_point(self_: Origin, other: Point) -> Line {
    return Line(/* e41, e42, e43 */ (vec3<f32>(self_.g0_) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z)), /* e23, e31, e12 */ vec3<f32>(0.0));
}
fn origin_wedge_scalar(self_: Origin, other: Scalar) -> Origin {
    return Origin(/* e4 */ (self_.g0_ * other.g0_));
}
fn plane_wedge_dualNum(self_: Plane, other: DualNum) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ (vec4<f32>(other.g0_.x) * self_.g0_));
}
fn plane_wedge_flector(self_: Plane, other: Flector) -> AntiScalar {
    return AntiScalar(/* e1234 */ (-(other.g0_.x * self_.g0_.x) - (other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z) - (other.g0_.w * self_.g0_.w)));
}
fn plane_wedge_motor(self_: Plane, other: Motor) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ (vec4<f32>(other.g1_.w) * self_.g0_));
}
fn plane_wedge_multiVector(self_: Plane, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (-(other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z) - (other.g1_.w * self_.g0_.w))), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_.x) * self_.g0_));
}
fn plane_wedge_origin(self_: Plane, other: Origin) -> AntiScalar {
    return AntiScalar(/* e1234 */ (self_.g0_.w * other.g0_ * -1.0));
}
fn plane_wedge_point(self_: Plane, other: Point) -> AntiScalar {
    return AntiScalar(/* e1234 */ (-(self_.g0_.x * other.g0_.x) - (self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z) - (self_.g0_.w * other.g0_.w)));
}
fn plane_wedge_scalar(self_: Plane, other: Scalar) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ (vec4<f32>(other.g0_) * self_.g0_));
}
fn point_wedge_dualNum(self_: Point, other: DualNum) -> Point {
    return Point(/* e1, e2, e3, e4 */ (vec4<f32>(other.g0_.x) * self_.g0_));
}
fn point_wedge_flector(self_: Point, other: Flector) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(((other.g0_.w * self_.g0_.x) * -1.0), ((other.g0_.w * self_.g0_.y) * -1.0), ((other.g0_.w * self_.g0_.z) * -1.0), ((other.g1_.y * self_.g0_.y) + (other.g1_.z * self_.g0_.z) + (other.g1_.w * self_.g0_.w))) + (vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.x) * self_.g0_.wwwx)), /* e23, e31, e12, scalar */ vec4<f32>((-(other.g0_.y * self_.g0_.z) + (other.g0_.z * self_.g0_.y)), ((other.g0_.x * self_.g0_.z) - (other.g0_.z * self_.g0_.x)), (-(other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x)), 0.0));
}
fn point_wedge_horizon(self_: Point, other: Horizon) -> AntiScalar {
    return AntiScalar(/* e1234 */ (self_.g0_.w * other.g0_));
}
fn point_wedge_line(self_: Point, other: Line) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.y * self_.g0_.z) + (other.g1_.x * self_.g0_.w)), ((other.g0_.z * self_.g0_.x) + (other.g1_.y * self_.g0_.w)), ((other.g0_.x * self_.g0_.y) + (other.g1_.z * self_.g0_.w)), (-(other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))) - (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.y, other.g1_.x) * self_.g0_.yzxx)));
}
fn point_wedge_motor(self_: Point, other: Motor) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(other.g1_.w) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.y * self_.g0_.z) + (other.g1_.x * self_.g0_.w)), ((other.g0_.z * self_.g0_.x) + (other.g1_.y * self_.g0_.w)), ((other.g0_.x * self_.g0_.y) + (other.g1_.z * self_.g0_.w)), (-(other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))) - (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.y, other.g1_.x) * self_.g0_.yzxx)));
}
fn point_wedge_multiVector(self_: Point, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, ((other.g4_.x * self_.g0_.x) + (other.g4_.y * self_.g0_.y) + (other.g4_.z * self_.g0_.z) + (other.g4_.w * self_.g0_.w))), /* e1, e2, e3, e4 */ (vec4<f32>(other.g0_.x) * self_.g0_), /* e41, e42, e43 */ (-(vec3<f32>(other.g1_.w) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z)) + (vec3<f32>(self_.g0_.w) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z))), /* e23, e31, e12 */ vec3<f32>((-(other.g1_.y * self_.g0_.z) + (other.g1_.z * self_.g0_.y)), ((other.g1_.x * self_.g0_.z) - (other.g1_.z * self_.g0_.x)), (-(other.g1_.x * self_.g0_.y) + (other.g1_.y * self_.g0_.x))), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g2_.y * self_.g0_.z) + (other.g3_.x * self_.g0_.w)), ((other.g2_.z * self_.g0_.x) + (other.g3_.y * self_.g0_.w)), ((other.g2_.x * self_.g0_.y) + (other.g3_.z * self_.g0_.w)), (-(other.g3_.y * self_.g0_.y) - (other.g3_.z * self_.g0_.z))) - (vec4<f32>(other.g2_.z, other.g2_.x, other.g2_.y, other.g3_.x) * self_.g0_.yzxx)));
}
fn point_wedge_origin(self_: Point, other: Origin) -> Line {
    return Line(/* e41, e42, e43 */ (vec3<f32>(other.g0_) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z) * vec3<f32>(-1.0)), /* e23, e31, e12 */ vec3<f32>(0.0));
}
fn point_wedge_plane(self_: Point, other: Plane) -> AntiScalar {
    return AntiScalar(/* e1234 */ ((other.g0_.x * self_.g0_.x) + (other.g0_.y * self_.g0_.y) + (other.g0_.z * self_.g0_.z) + (other.g0_.w * self_.g0_.w)));
}
fn point_wedge_point(self_: Point, other: Point) -> Line {
    return Line(/* e41, e42, e43 */ (-(vec3<f32>(other.g0_.w) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z)) + (vec3<f32>(self_.g0_.w) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z))), /* e23, e31, e12 */ vec3<f32>((-(other.g0_.y * self_.g0_.z) + (other.g0_.z * self_.g0_.y)), ((other.g0_.x * self_.g0_.z) - (other.g0_.z * self_.g0_.x)), (-(other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x))));
}
fn point_wedge_scalar(self_: Point, other: Scalar) -> Point {
    return Point(/* e1, e2, e3, e4 */ (vec4<f32>(other.g0_) * self_.g0_));
}
fn scalar_wedge_antiScalar(self_: Scalar, other: AntiScalar) -> AntiScalar {
    return AntiScalar(/* e1234 */ (other.g0_ * self_.g0_));
}
fn scalar_wedge_dualNum(self_: Scalar, other: DualNum) -> DualNum {
    return DualNum(/* scalar, e1234 */ (vec2<f32>(self_.g0_) * other.g0_));
}
fn scalar_wedge_flector(self_: Scalar, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_) * other.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_) * other.g1_));
}
fn scalar_wedge_horizon(self_: Scalar, other: Horizon) -> Horizon {
    return Horizon(/* e321 */ (other.g0_ * self_.g0_));
}
fn scalar_wedge_line(self_: Scalar, other: Line) -> Line {
    return Line(/* e41, e42, e43 */ (vec3<f32>(self_.g0_) * other.g0_), /* e23, e31, e12 */ (vec3<f32>(self_.g0_) * other.g1_));
}
fn scalar_wedge_motor(self_: Scalar, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(self_.g0_) * other.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(self_.g0_) * other.g1_));
}
fn scalar_wedge_multiVector(self_: Scalar, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ (vec2<f32>(self_.g0_) * other.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_) * other.g1_), /* e41, e42, e43 */ (vec3<f32>(self_.g0_) * other.g2_), /* e23, e31, e12 */ (vec3<f32>(self_.g0_) * other.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_) * other.g4_));
}
fn scalar_wedge_origin(self_: Scalar, other: Origin) -> Origin {
    return Origin(/* e4 */ (other.g0_ * self_.g0_));
}
fn scalar_wedge_plane(self_: Scalar, other: Plane) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_) * other.g0_));
}
fn scalar_wedge_point(self_: Scalar, other: Point) -> Point {
    return Point(/* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_) * other.g0_));
}
fn scalar_wedge_scalar(self_: Scalar, other: Scalar) -> Scalar {
    return Scalar(/* scalar */ (other.g0_ * self_.g0_));
}
fn antiScalar_zero() -> AntiScalar {
    return AntiScalar(/* e1234 */ 0.0);
}
fn dualNum_zero() -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>(0.0));
}
fn flector_zero() -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn horizon_zero() -> Horizon {
    return Horizon(/* e321 */ 0.0);
}
fn line_zero() -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0));
}
fn motor_zero() -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0), /* e23, e31, e12, scalar */ vec4<f32>(0.0));
}
fn multiVector_zero() -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn origin_zero() -> Origin {
    return Origin(/* e4 */ 0.0);
}
fn plane_zero() -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn point_zero() -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>(0.0));
}
fn scalar_zero() -> Scalar {
    return Scalar(/* scalar */ 0.0);
}
fn antiScalar_antiSandwich_antiScalar(self_: AntiScalar, other: AntiScalar) -> AntiScalar {
    let geometric_anti_product: AntiScalar = AntiScalar(/* e1234 */ (other.g0_ * self_.g0_));
    return antiScalar_geometricAntiProduct_antiScalar(geometric_anti_product, antiScalar_antiReverse(self_));
}
fn antiScalar_antiSandwich_dualNum(self_: AntiScalar, other: DualNum) -> DualNum {
    let geometric_anti_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>(self_.g0_) * other.g0_));
    return dualNum_geometricAntiProduct_dualNum(geometric_anti_product, antiScalar_antiReverse(self_));
}
fn antiScalar_antiSandwich_flector(self_: AntiScalar, other: Flector) -> Flector {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_) * other.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_) * other.g1_));
    return flector_geometricAntiProduct_flector(geometric_anti_product, antiScalar_antiReverse(self_));
}
fn antiScalar_antiSandwich_horizon(self_: AntiScalar, other: Horizon) -> Horizon {
    let geometric_anti_product: Horizon = Horizon(/* e321 */ (self_.g0_ * other.g0_));
    return horizon_geometricAntiProduct_horizon(geometric_anti_product, antiScalar_antiReverse(self_));
}
fn antiScalar_antiSandwich_line(self_: AntiScalar, other: Line) -> Line {
    let geometric_anti_product: Line = Line(/* e41, e42, e43 */ (vec3<f32>(self_.g0_) * other.g0_), /* e23, e31, e12 */ (vec3<f32>(self_.g0_) * other.g1_));
    return line_geometricAntiProduct_line(geometric_anti_product, antiScalar_antiReverse(self_));
}
fn antiScalar_antiSandwich_motor(self_: AntiScalar, other: Motor) -> Motor {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(self_.g0_) * other.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(self_.g0_) * other.g1_));
    return motor_geometricAntiProduct_motor(geometric_anti_product, antiScalar_antiReverse(self_));
}
fn antiScalar_antiSandwich_multiVector(self_: AntiScalar, other: MultiVector) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(self_.g0_) * other.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_) * other.g1_), /* e41, e42, e43 */ (vec3<f32>(self_.g0_) * other.g2_), /* e23, e31, e12 */ (vec3<f32>(self_.g0_) * other.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_) * other.g4_));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, antiScalar_antiReverse(self_));
}
fn antiScalar_antiSandwich_origin(self_: AntiScalar, other: Origin) -> Origin {
    let geometric_anti_product: Origin = Origin(/* e4 */ (self_.g0_ * other.g0_));
    return origin_geometricAntiProduct_origin(geometric_anti_product, antiScalar_antiReverse(self_));
}
fn antiScalar_antiSandwich_plane(self_: AntiScalar, other: Plane) -> Plane {
    let geometric_anti_product: Plane = Plane(/* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_) * other.g0_));
    return plane_geometricAntiProduct_plane(geometric_anti_product, antiScalar_antiReverse(self_));
}
fn antiScalar_antiSandwich_point(self_: AntiScalar, other: Point) -> Point {
    let geometric_anti_product: Point = Point(/* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_) * other.g0_));
    return point_geometricAntiProduct_point(geometric_anti_product, antiScalar_antiReverse(self_));
}
fn antiScalar_antiSandwich_scalar(self_: AntiScalar, other: Scalar) -> Scalar {
    let geometric_anti_product: Scalar = Scalar(/* scalar */ (self_.g0_ * other.g0_));
    return scalar_geometricAntiProduct_scalar(geometric_anti_product, antiScalar_antiReverse(self_));
}
fn dualNum_antiSandwich_antiScalar(self_: DualNum, other: AntiScalar) -> DualNum {
    let geometric_anti_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>(other.g0_) * self_.g0_));
    return dualNum_geometricAntiProduct_dualNum(geometric_anti_product, dualNum_antiReverse(self_));
}
fn dualNum_antiSandwich_dualNum(self_: DualNum, other: DualNum) -> DualNum {
    let geometric_anti_product: DualNum = DualNum(/* scalar, e1234 */ vec2<f32>(((other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x)), (other.g0_.y * self_.g0_.y)));
    return dualNum_geometricAntiProduct_dualNum(geometric_anti_product, dualNum_antiReverse(self_));
}
fn dualNum_antiSandwich_flector(self_: DualNum, other: Flector) -> Flector {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>(((self_.g0_.x * other.g1_.x) + (self_.g0_.y * other.g0_.x)), ((self_.g0_.x * other.g1_.y) + (self_.g0_.y * other.g0_.y)), ((self_.g0_.x * other.g1_.z) + (self_.g0_.y * other.g0_.z)), (self_.g0_.y * other.g0_.w)), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.y * other.g1_.x), (self_.g0_.y * other.g1_.y), (self_.g0_.y * other.g1_.z), ((self_.g0_.x * other.g0_.w) + (self_.g0_.y * other.g1_.w))));
    return flector_geometricAntiProduct_flector(geometric_anti_product, dualNum_antiReverse(self_));
}
fn dualNum_antiSandwich_horizon(self_: DualNum, other: Horizon) -> Horizon {
    let geometric_anti_product: Horizon = Horizon(/* e321 */ (self_.g0_.y * other.g0_));
    return horizon_geometricAntiProduct_horizon(geometric_anti_product, dualNum_antiReverse(self_));
}
fn dualNum_antiSandwich_line(self_: DualNum, other: Line) -> Line {
    let geometric_anti_product: Line = Line(/* e41, e42, e43 */ (vec3<f32>(self_.g0_.y) * other.g0_), /* e23, e31, e12 */ ((vec3<f32>(self_.g0_.x) * other.g0_) + (vec3<f32>(self_.g0_.y) * other.g1_)));
    return line_geometricAntiProduct_line(geometric_anti_product, dualNum_antiReverse(self_));
}
fn dualNum_antiSandwich_motor(self_: DualNum, other: Motor) -> Motor {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(self_.g0_.y) * other.g0_), /* e23, e31, e12, scalar */ ((vec4<f32>(self_.g0_.x) * other.g0_) + (vec4<f32>(self_.g0_.y) * other.g1_)));
    return motor_geometricAntiProduct_motor(geometric_anti_product, dualNum_antiReverse(self_));
}
fn dualNum_antiSandwich_multiVector(self_: DualNum, other: MultiVector) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(((self_.g0_.x * other.g0_.y) + (self_.g0_.y * other.g0_.x)), (self_.g0_.y * other.g0_.y)), /* e1, e2, e3, e4 */ vec4<f32>(((self_.g0_.x * other.g4_.x) + (self_.g0_.y * other.g1_.x)), ((self_.g0_.x * other.g4_.y) + (self_.g0_.y * other.g1_.y)), ((self_.g0_.x * other.g4_.z) + (self_.g0_.y * other.g1_.z)), (self_.g0_.y * other.g1_.w)), /* e41, e42, e43 */ (vec3<f32>(self_.g0_.y) * other.g2_), /* e23, e31, e12 */ ((vec3<f32>(self_.g0_.x) * other.g2_) + (vec3<f32>(self_.g0_.y) * other.g3_)), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.y * other.g4_.x), (self_.g0_.y * other.g4_.y), (self_.g0_.y * other.g4_.z), ((self_.g0_.x * other.g1_.w) + (self_.g0_.y * other.g4_.w))));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, dualNum_antiReverse(self_));
}
fn dualNum_antiSandwich_origin(self_: DualNum, other: Origin) -> Flector {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.y * other.g0_)), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.x * other.g0_)));
    return flector_geometricAntiProduct_flector(geometric_anti_product, dualNum_antiReverse(self_));
}
fn dualNum_antiSandwich_plane(self_: DualNum, other: Plane) -> Flector {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g0_.x), (self_.g0_.x * other.g0_.y), (self_.g0_.x * other.g0_.z), 0.0), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_.y) * other.g0_));
    return flector_geometricAntiProduct_flector(geometric_anti_product, dualNum_antiReverse(self_));
}
fn dualNum_antiSandwich_point(self_: DualNum, other: Point) -> Flector {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_.y) * other.g0_), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.x * other.g0_.w)));
    return flector_geometricAntiProduct_flector(geometric_anti_product, dualNum_antiReverse(self_));
}
fn dualNum_antiSandwich_scalar(self_: DualNum, other: Scalar) -> Scalar {
    let geometric_anti_product: Scalar = Scalar(/* scalar */ (self_.g0_.y * other.g0_));
    return scalar_geometricAntiProduct_scalar(geometric_anti_product, dualNum_antiReverse(self_));
}
fn flector_antiSandwich_antiScalar(self_: Flector, other: AntiScalar) -> Motor {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(other.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_) * self_.g1_));
    return flector_geometricAntiProduct_flector(geometric_anti_product, flector_antiReverse(self_));
}
fn flector_antiSandwich_dualNum(self_: Flector, other: DualNum) -> Motor {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>((-(other.g0_.x * self_.g1_.x) + (other.g0_.y * self_.g0_.x)), (-(other.g0_.x * self_.g1_.y) + (other.g0_.y * self_.g0_.y)), (-(other.g0_.x * self_.g1_.z) + (other.g0_.y * self_.g0_.z)), (other.g0_.y * self_.g0_.w)), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.y * self_.g1_.x), (other.g0_.y * self_.g1_.y), (other.g0_.y * self_.g1_.z), (-(other.g0_.x * self_.g0_.w) + (other.g0_.y * self_.g1_.w))));
    return flector_geometricAntiProduct_flector(geometric_anti_product, flector_antiReverse(self_));
}
fn flector_antiSandwich_flector(self_: Flector, other: Flector) -> Flector {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>((-(other.g1_.x * self_.g0_.w) - (other.g1_.z * self_.g1_.y)), (-(other.g1_.x * self_.g1_.z) - (other.g1_.y * self_.g0_.w)), (-(other.g1_.y * self_.g1_.x) - (other.g1_.z * self_.g0_.w)), ((other.g1_.y * self_.g1_.y) + (other.g1_.z * self_.g1_.z))) - (vec4<f32>(other.g0_.w) * vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g0_.w)) + (other.g1_.yzxx * self_.g1_.zxyx)), /* e23, e31, e12, scalar */ (vec4<f32>(((other.g0_.z * self_.g1_.y) - (other.g1_.z * self_.g0_.y)), ((other.g0_.x * self_.g1_.z) - (other.g1_.x * self_.g0_.z)), ((other.g0_.y * self_.g1_.x) - (other.g1_.y * self_.g0_.x)), (-(other.g0_.w * self_.g1_.w) + (other.g1_.w * self_.g0_.w))) + (vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.x) * self_.g0_.wwwx) - (vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, other.g0_.z) * self_.g1_.wwwz) - (vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g1_.y) * other.g0_.wwwy) + (vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g0_.z) * other.g1_.wwwz) - (other.g0_.yzxx * self_.g1_.zxyx) + (other.g1_.yzxy * self_.g0_.zxyy)));
    return motor_geometricAntiProduct_motor(geometric_anti_product, flector_antiReverse(self_));
}
fn flector_antiSandwich_horizon(self_: Flector, other: Horizon) -> Flector {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0), /* e23, e31, e12, scalar */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g0_.w)));
    return motor_geometricAntiProduct_motor(geometric_anti_product, flector_antiReverse(self_));
}
fn flector_antiSandwich_line(self_: Flector, other: Line) -> Motor {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(((other.g0_.x * self_.g1_.w) - (other.g0_.y * self_.g0_.z) + (other.g0_.z * self_.g0_.y) - (other.g1_.x * self_.g0_.w) + (other.g1_.y * self_.g1_.z)), ((other.g0_.x * self_.g0_.z) + (other.g0_.y * self_.g1_.w) - (other.g0_.z * self_.g0_.x) - (other.g1_.y * self_.g0_.w) + (other.g1_.z * self_.g1_.x)), (-(other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x) + (other.g0_.z * self_.g1_.w) + (other.g1_.x * self_.g1_.y) - (other.g1_.z * self_.g0_.w)), (-(other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z))) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.x) * self_.g1_.yzxx)), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.x * self_.g0_.w) - (other.g0_.y * self_.g1_.z)), ((other.g0_.y * self_.g0_.w) - (other.g0_.z * self_.g1_.x)), (-(other.g0_.x * self_.g1_.y) + (other.g0_.z * self_.g0_.w)), (-(other.g0_.x * self_.g0_.x) - (other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z) + (other.g1_.y * self_.g1_.y) + (other.g1_.z * self_.g1_.z))) + (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.y, other.g1_.x) * self_.g1_.yzxx)));
    return flector_geometricAntiProduct_flector(geometric_anti_product, flector_antiReverse(self_));
}
fn flector_antiSandwich_motor(self_: Flector, other: Motor) -> Motor {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(((self_.g0_.y * other.g0_.z) - (self_.g0_.w * other.g1_.x) + (self_.g1_.z * other.g1_.y) + (self_.g1_.w * other.g0_.x)), ((self_.g0_.z * other.g0_.x) - (self_.g0_.w * other.g1_.y) + (self_.g1_.x * other.g1_.z) + (self_.g1_.w * other.g0_.y)), ((self_.g0_.z * other.g0_.w) - (self_.g0_.w * other.g1_.z) + (self_.g1_.y * other.g1_.x) + (self_.g1_.w * other.g0_.z)), 0.0) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g1_.x) * other.g0_.yzxx) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.w, other.g0_.z) * self_.g1_.yzzz) - (vec4<f32>(other.g1_.w, other.g1_.w, other.g1_.y, other.g0_.y) * self_.g1_.xyxy) + (self_.g0_.xyxw * other.g0_.wwyw)), /* e423, e431, e412, e321 */ (vec4<f32>(0.0, 0.0, 0.0, (-(self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z) - (self_.g0_.w * other.g1_.w) + (self_.g1_.z * other.g1_.z))) + (vec4<f32>(self_.g0_.w, self_.g0_.w, self_.g0_.w, self_.g1_.w) * other.g0_) - (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g0_.x) * other.g0_.yzxx) + (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.w, other.g1_.y) * self_.g1_.yzzy) + (vec4<f32>(other.g0_.w, other.g0_.w, other.g0_.y, other.g1_.x) * self_.g1_.xyxx)));
    return flector_geometricAntiProduct_flector(geometric_anti_product, flector_antiReverse(self_));
}
fn flector_antiSandwich_multiVector(self_: Flector, other: MultiVector) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(((self_.g0_.w * other.g4_.w) - (self_.g1_.x * other.g1_.x) - (self_.g1_.y * other.g1_.y) - (self_.g1_.z * other.g1_.z)), 0.0) - (vec2<f32>(other.g1_.w) * vec2<f32>(self_.g1_.w, self_.g0_.w)) + (vec2<f32>(other.g4_.x) * vec2<f32>(self_.g0_.x, self_.g1_.x)) + (vec2<f32>(other.g4_.y) * vec2<f32>(self_.g0_.y, self_.g1_.y)) + (vec2<f32>(other.g4_.z) * vec2<f32>(self_.g0_.z, self_.g1_.z))), /* e1, e2, e3, e4 */ (vec4<f32>(((other.g2_.x * self_.g1_.w) - (other.g2_.y * self_.g0_.z) + (other.g2_.z * self_.g0_.y) - (other.g3_.x * self_.g0_.w) + (other.g3_.y * self_.g1_.z)), ((other.g2_.x * self_.g0_.z) + (other.g2_.y * self_.g1_.w) - (other.g2_.z * self_.g0_.x) - (other.g3_.y * self_.g0_.w) + (other.g3_.z * self_.g1_.x)), (-(other.g2_.x * self_.g0_.y) + (other.g2_.y * self_.g0_.x) + (other.g2_.z * self_.g1_.w) + (other.g3_.x * self_.g1_.y) - (other.g3_.z * self_.g0_.w)), ((other.g2_.z * self_.g1_.z) * -1.0)) + (vec4<f32>(other.g0_.y) * self_.g0_) - (vec4<f32>(other.g0_.x, other.g0_.x, other.g0_.x, other.g2_.x) * self_.g1_.xyzx) - (vec4<f32>(other.g3_.z, other.g3_.x, other.g3_.y, other.g2_.y) * self_.g1_.yzxy)), /* e41, e42, e43 */ (vec3<f32>((-(self_.g1_.y * other.g4_.z) + (self_.g1_.z * other.g4_.y)), ((self_.g1_.x * other.g4_.z) - (self_.g1_.z * other.g4_.x)), (-(self_.g1_.x * other.g4_.y) + (self_.g1_.y * other.g4_.x))) - (vec3<f32>(self_.g0_.w) * vec3<f32>(other.g4_.x, other.g4_.y, other.g4_.z)) - (vec3<f32>(other.g1_.w) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z))), /* e23, e31, e12 */ (vec3<f32>((-(self_.g0_.y * other.g4_.z) + (self_.g0_.z * other.g4_.y) + (self_.g1_.y * other.g1_.z) - (self_.g1_.z * other.g1_.y)), ((self_.g0_.x * other.g4_.z) - (self_.g0_.z * other.g4_.x) - (self_.g1_.x * other.g1_.z) + (self_.g1_.z * other.g1_.x)), (-(self_.g0_.x * other.g4_.y) + (self_.g0_.y * other.g4_.x) + (self_.g1_.x * other.g1_.y) - (self_.g1_.y * other.g1_.x))) + (vec3<f32>(self_.g0_.w) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z)) - (vec3<f32>(self_.g1_.w) * vec3<f32>(other.g4_.x, other.g4_.y, other.g4_.z)) - (vec3<f32>(other.g1_.w) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z)) + (vec3<f32>(other.g4_.w) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z))), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g2_.x * self_.g0_.w) - (other.g2_.y * self_.g1_.z)), ((other.g2_.y * self_.g0_.w) - (other.g2_.z * self_.g1_.x)), (-(other.g2_.x * self_.g1_.y) + (other.g2_.z * self_.g0_.w)), (-(other.g0_.x * self_.g0_.w) - (other.g2_.x * self_.g0_.x) - (other.g2_.y * self_.g0_.y) - (other.g2_.z * self_.g0_.z) + (other.g3_.y * self_.g1_.y) + (other.g3_.z * self_.g1_.z))) + (vec4<f32>(other.g0_.y) * self_.g1_) + (vec4<f32>(other.g2_.z, other.g2_.x, other.g2_.y, other.g3_.x) * self_.g1_.yzxx)));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, flector_antiReverse(self_));
}
fn flector_antiSandwich_origin(self_: Flector, other: Origin) -> Flector {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g0_.w) * vec4<f32>(-1.0)), /* e23, e31, e12, scalar */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g1_.w) * vec4<f32>(-1.0)));
    return motor_geometricAntiProduct_motor(geometric_anti_product, flector_antiReverse(self_));
}
fn flector_antiSandwich_plane(self_: Flector, other: Plane) -> Flector {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>((-(self_.g0_.w * other.g0_.x) - (self_.g1_.y * other.g0_.z)), (-(self_.g0_.w * other.g0_.y) - (self_.g1_.z * other.g0_.x)), (-(self_.g0_.w * other.g0_.z) - (self_.g1_.x * other.g0_.y)), ((self_.g1_.y * other.g0_.y) + (self_.g1_.z * other.g0_.z))) + (self_.g1_.zxyx * other.g0_.yzxx)), /* e23, e31, e12, scalar */ (vec4<f32>((-(self_.g0_.y * other.g0_.z) - (self_.g1_.w * other.g0_.x)), (-(self_.g0_.z * other.g0_.x) - (self_.g1_.w * other.g0_.y)), (-(self_.g0_.x * other.g0_.y) - (self_.g1_.w * other.g0_.z)), ((self_.g0_.z * other.g0_.z) + (self_.g0_.w * other.g0_.w))) + (vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g0_.y) * other.g0_.wwwy) + (self_.g0_.zxyx * other.g0_.yzxx)));
    return motor_geometricAntiProduct_motor(geometric_anti_product, flector_antiReverse(self_));
}
fn flector_antiSandwich_point(self_: Flector, other: Point) -> Flector {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(other.g0_.w) * vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g0_.w) * vec4<f32>(-1.0)), /* e23, e31, e12, scalar */ (vec4<f32>(((self_.g0_.w * other.g0_.x) + (self_.g1_.y * other.g0_.z)), ((self_.g0_.w * other.g0_.y) + (self_.g1_.z * other.g0_.x)), ((self_.g0_.w * other.g0_.z) + (self_.g1_.x * other.g0_.y)), (-(self_.g1_.z * other.g0_.z) - (self_.g1_.w * other.g0_.w))) - (vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g1_.x) * other.g0_.wwwx) - (self_.g1_.zxyy * other.g0_.yzxy)));
    return motor_geometricAntiProduct_motor(geometric_anti_product, flector_antiReverse(self_));
}
fn flector_antiSandwich_scalar(self_: Flector, other: Scalar) -> Motor {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g1_.x * other.g0_ * -1.0), (self_.g1_.y * other.g0_ * -1.0), (self_.g1_.z * other.g0_ * -1.0), 0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.w * other.g0_ * -1.0)));
    return flector_geometricAntiProduct_flector(geometric_anti_product, flector_antiReverse(self_));
}
fn horizon_antiSandwich_flector(self_: Horizon, other: Flector) -> Flector {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0), /* e23, e31, e12, scalar */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, other.g0_.w) * vec4<f32>(-1.0)));
    return motor_geometricAntiProduct_motor(geometric_anti_product, horizon_antiReverse(self_));
}
fn horizon_antiSandwich_line(self_: Horizon, other: Line) -> Scalar {
    let geometric_anti_product: Point = Point(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), 0.0));
    return point_geometricAntiProduct_point(geometric_anti_product, horizon_antiReverse(self_));
}
fn horizon_antiSandwich_motor(self_: Horizon, other: Motor) -> Motor {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), 0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.w * self_.g0_)));
    return flector_geometricAntiProduct_flector(geometric_anti_product, horizon_antiReverse(self_));
}
fn horizon_antiSandwich_multiVector(self_: Horizon, other: MultiVector) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>((other.g1_.w * self_.g0_ * -1.0), 0.0), /* e1, e2, e3, e4 */ vec4<f32>((other.g2_.x * self_.g0_), (other.g2_.y * self_.g0_), (other.g2_.z * self_.g0_), 0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ (vec3<f32>(self_.g0_) * vec3<f32>(other.g4_.x, other.g4_.y, other.g4_.z) * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.y * self_.g0_)));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, horizon_antiReverse(self_));
}
fn horizon_antiSandwich_plane(self_: Horizon, other: Plane) -> Point {
    let geometric_anti_product: Line = Line(/* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ (vec3<f32>(self_.g0_) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z) * vec3<f32>(-1.0)));
    return line_geometricAntiProduct_line(geometric_anti_product, horizon_antiReverse(self_));
}
fn line_antiSandwich_antiScalar(self_: Line, other: AntiScalar) -> Motor {
    let geometric_anti_product: Line = Line(/* e41, e42, e43 */ (vec3<f32>(other.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(other.g0_) * self_.g1_));
    return line_geometricAntiProduct_line(geometric_anti_product, line_antiReverse(self_));
}
fn line_antiSandwich_dualNum(self_: Line, other: DualNum) -> Motor {
    let geometric_anti_product: Line = Line(/* e41, e42, e43 */ (vec3<f32>(other.g0_.y) * self_.g0_), /* e23, e31, e12 */ ((vec3<f32>(other.g0_.x) * self_.g0_) + (vec3<f32>(other.g0_.y) * self_.g1_)));
    return line_geometricAntiProduct_line(geometric_anti_product, line_antiReverse(self_));
}
fn line_antiSandwich_flector(self_: Line, other: Flector) -> Flector {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(((self_.g0_.x * other.g1_.w) + (self_.g0_.y * other.g0_.z) - (self_.g0_.z * other.g0_.y) + (self_.g1_.x * other.g0_.w) + (self_.g1_.y * other.g1_.z)), (-(self_.g0_.x * other.g0_.z) + (self_.g0_.y * other.g1_.w) + (self_.g0_.z * other.g0_.x) + (self_.g1_.y * other.g0_.w) + (self_.g1_.z * other.g1_.x)), ((self_.g0_.x * other.g0_.y) - (self_.g0_.y * other.g0_.x) + (self_.g0_.z * other.g1_.w) + (self_.g1_.x * other.g1_.y) + (self_.g1_.z * other.g0_.w)), (-(self_.g0_.y * other.g1_.y) - (self_.g0_.z * other.g1_.z))) - (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g0_.x) * other.g1_.yzxx)), /* e423, e431, e412, e321 */ (vec4<f32>(((self_.g0_.x * other.g0_.w) + (self_.g0_.y * other.g1_.z)), ((self_.g0_.y * other.g0_.w) + (self_.g0_.z * other.g1_.x)), ((self_.g0_.x * other.g1_.y) + (self_.g0_.z * other.g0_.w)), (-(self_.g0_.x * other.g0_.x) - (self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z) - (self_.g1_.y * other.g1_.y) - (self_.g1_.z * other.g1_.z))) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g1_.x) * other.g1_.yzxx)));
    return flector_geometricAntiProduct_flector(geometric_anti_product, line_antiReverse(self_));
}
fn line_antiSandwich_horizon(self_: Line, other: Horizon) -> Flector {
    let geometric_anti_product: Point = Point(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), 0.0));
    return point_geometricAntiProduct_point(geometric_anti_product, line_antiReverse(self_));
}
fn line_antiSandwich_line(self_: Line, other: Line) -> Motor {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>((-(other.g0_.y * self_.g0_.z) + (other.g0_.z * self_.g0_.y)), ((other.g0_.x * self_.g0_.z) - (other.g0_.z * self_.g0_.x)), (-(other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x)), (-(other.g0_.x * self_.g0_.x) - (other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z))), /* e23, e31, e12, scalar */ vec4<f32>((-(other.g0_.y * self_.g1_.z) + (other.g0_.z * self_.g1_.y) - (other.g1_.y * self_.g0_.z) + (other.g1_.z * self_.g0_.y)), ((other.g0_.x * self_.g1_.z) - (other.g0_.z * self_.g1_.x) + (other.g1_.x * self_.g0_.z) - (other.g1_.z * self_.g0_.x)), (-(other.g0_.x * self_.g1_.y) + (other.g0_.y * self_.g1_.x) - (other.g1_.x * self_.g0_.y) + (other.g1_.y * self_.g0_.x)), (-(other.g0_.x * self_.g1_.x) - (other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))));
    return motor_geometricAntiProduct_motor(geometric_anti_product, line_antiReverse(self_));
}
fn line_antiSandwich_motor(self_: Line, other: Motor) -> Motor {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(((self_.g0_.x * other.g0_.w) + (self_.g0_.y * other.g0_.z)), ((self_.g0_.y * other.g0_.w) + (self_.g0_.z * other.g0_.x)), ((self_.g0_.x * other.g0_.y) + (self_.g0_.z * other.g0_.w)), (-(self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z))) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g0_.x) * other.g0_.yzxx)), /* e23, e31, e12, scalar */ (vec4<f32>(((self_.g0_.x * other.g1_.w) + (self_.g0_.y * other.g1_.z) + (self_.g1_.x * other.g0_.w) + (self_.g1_.y * other.g0_.z)), ((self_.g0_.y * other.g1_.w) + (self_.g0_.z * other.g1_.x) + (self_.g1_.y * other.g0_.w) + (self_.g1_.z * other.g0_.x)), ((self_.g0_.x * other.g1_.y) + (self_.g0_.z * other.g1_.w) + (self_.g1_.x * other.g0_.y) + (self_.g1_.z * other.g0_.w)), (-(self_.g0_.y * other.g1_.y) - (self_.g0_.z * other.g1_.z) - (self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g0_.x) * other.g1_.yzxx) - (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g1_.x) * other.g0_.yzxx)));
    return motor_geometricAntiProduct_motor(geometric_anti_product, line_antiReverse(self_));
}
fn line_antiSandwich_multiVector(self_: Line, other: MultiVector) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>((-(self_.g1_.x * other.g2_.x) - (self_.g1_.y * other.g2_.y) - (self_.g1_.z * other.g2_.z)), 0.0) - (vec2<f32>(self_.g0_.x) * vec2<f32>(other.g3_.x, other.g2_.x)) - (vec2<f32>(self_.g0_.y) * vec2<f32>(other.g3_.y, other.g2_.y)) - (vec2<f32>(self_.g0_.z) * vec2<f32>(other.g3_.z, other.g2_.z))), /* e1, e2, e3, e4 */ (vec4<f32>(((self_.g0_.x * other.g4_.w) + (self_.g0_.y * other.g1_.z) - (self_.g0_.z * other.g1_.y) + (self_.g1_.x * other.g1_.w) + (self_.g1_.y * other.g4_.z)), (-(self_.g0_.x * other.g1_.z) + (self_.g0_.y * other.g4_.w) + (self_.g0_.z * other.g1_.x) + (self_.g1_.y * other.g1_.w) + (self_.g1_.z * other.g4_.x)), ((self_.g0_.x * other.g1_.y) - (self_.g0_.y * other.g1_.x) + (self_.g0_.z * other.g4_.w) + (self_.g1_.x * other.g4_.y) + (self_.g1_.z * other.g1_.w)), (-(self_.g0_.y * other.g4_.y) - (self_.g0_.z * other.g4_.z))) - (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g0_.x) * other.g4_.yzxx)), /* e41, e42, e43 */ ((vec3<f32>(other.g0_.y) * self_.g0_) + (self_.g0_.yzx * other.g2_.zxy) - (self_.g0_.zxy * other.g2_.yzx)), /* e23, e31, e12 */ ((vec3<f32>(other.g0_.x) * self_.g0_) + (vec3<f32>(other.g0_.y) * self_.g1_) + (self_.g0_.yzx * other.g3_.zxy) - (self_.g0_.zxy * other.g3_.yzx) + (self_.g1_.yzx * other.g2_.zxy) - (self_.g1_.zxy * other.g2_.yzx)), /* e423, e431, e412, e321 */ (vec4<f32>(((self_.g0_.x * other.g1_.w) + (self_.g0_.y * other.g4_.z)), ((self_.g0_.y * other.g1_.w) + (self_.g0_.z * other.g4_.x)), ((self_.g0_.x * other.g4_.y) + (self_.g0_.z * other.g1_.w)), (-(self_.g0_.x * other.g1_.x) - (self_.g0_.y * other.g1_.y) - (self_.g0_.z * other.g1_.z) - (self_.g1_.y * other.g4_.y) - (self_.g1_.z * other.g4_.z))) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g1_.x) * other.g4_.yzxx)));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, line_antiReverse(self_));
}
fn line_antiSandwich_origin(self_: Line, other: Origin) -> Flector {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), 0.0), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), 0.0));
    return flector_geometricAntiProduct_flector(geometric_anti_product, line_antiReverse(self_));
}
fn line_antiSandwich_plane(self_: Line, other: Plane) -> Flector {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(((self_.g0_.x * other.g0_.w) + (self_.g1_.y * other.g0_.z)), ((self_.g0_.y * other.g0_.w) + (self_.g1_.z * other.g0_.x)), ((self_.g0_.z * other.g0_.w) + (self_.g1_.x * other.g0_.y)), (-(self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z))) - (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g0_.x) * other.g0_.yzxx)), /* e423, e431, e412, e321 */ (vec4<f32>((self_.g0_.y * other.g0_.z), (self_.g0_.z * other.g0_.x), (self_.g0_.x * other.g0_.y), (-(self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g1_.x) * other.g0_.yzxx)));
    return flector_geometricAntiProduct_flector(geometric_anti_product, line_antiReverse(self_));
}
fn line_antiSandwich_point(self_: Line, other: Point) -> Flector {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>(((self_.g0_.y * other.g0_.z) - (self_.g0_.z * other.g0_.y) + (self_.g1_.x * other.g0_.w)), (-(self_.g0_.x * other.g0_.z) + (self_.g0_.z * other.g0_.x) + (self_.g1_.y * other.g0_.w)), ((self_.g0_.x * other.g0_.y) - (self_.g0_.y * other.g0_.x) + (self_.g1_.z * other.g0_.w)), 0.0), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.x * other.g0_.w), (self_.g0_.y * other.g0_.w), (self_.g0_.z * other.g0_.w), (-(self_.g0_.x * other.g0_.x) - (self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z))));
    return flector_geometricAntiProduct_flector(geometric_anti_product, line_antiReverse(self_));
}
fn line_antiSandwich_scalar(self_: Line, other: Scalar) -> Motor {
    let geometric_anti_product: Line = Line(/* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ (vec3<f32>(other.g0_) * self_.g0_));
    return line_geometricAntiProduct_line(geometric_anti_product, line_antiReverse(self_));
}
fn motor_antiSandwich_antiScalar(self_: Motor, other: AntiScalar) -> Motor {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(other.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(other.g0_) * self_.g1_));
    return motor_geometricAntiProduct_motor(geometric_anti_product, motor_antiReverse(self_));
}
fn motor_antiSandwich_dualNum(self_: Motor, other: DualNum) -> Motor {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(other.g0_.y) * self_.g0_), /* e23, e31, e12, scalar */ ((vec4<f32>(other.g0_.x) * self_.g0_) + (vec4<f32>(other.g0_.y) * self_.g1_)));
    return motor_geometricAntiProduct_motor(geometric_anti_product, motor_antiReverse(self_));
}
fn motor_antiSandwich_flector(self_: Motor, other: Flector) -> Flector {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(((other.g0_.z * self_.g0_.y) + (other.g0_.w * self_.g1_.x) + (other.g1_.x * self_.g1_.w) + (other.g1_.z * self_.g1_.y) + (other.g1_.w * self_.g0_.x)), ((other.g0_.y * self_.g0_.w) + (other.g0_.w * self_.g1_.y) + (other.g1_.x * self_.g1_.z) + (other.g1_.y * self_.g1_.w) + (other.g1_.w * self_.g0_.y)), ((other.g0_.z * self_.g0_.w) + (other.g0_.w * self_.g1_.z) + (other.g1_.y * self_.g1_.x) + (other.g1_.z * self_.g1_.w) + (other.g1_.w * self_.g0_.z)), ((other.g1_.z * self_.g0_.z) * -1.0)) - (vec4<f32>(other.g0_.y, other.g0_.z, other.g0_.x, other.g1_.x) * self_.g0_.zxyx) - (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g0_.y) * other.g1_.yzxy) + (other.g0_.xxyw * self_.g0_.wzxw)), /* e423, e431, e412, e321 */ (vec4<f32>((other.g1_.z * self_.g0_.y), (other.g1_.y * self_.g0_.w), (other.g1_.z * self_.g0_.w), (-(other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z) - (other.g1_.x * self_.g1_.x) - (other.g1_.y * self_.g1_.y) - (other.g1_.z * self_.g1_.z))) + (vec4<f32>(other.g0_.w) * vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g1_.w)) - (vec4<f32>(other.g1_.y, other.g1_.z, other.g1_.x, other.g0_.x) * self_.g0_.zxyx) + (other.g1_.xxyw * self_.g0_.wzxw)));
    return flector_geometricAntiProduct_flector(geometric_anti_product, motor_antiReverse(self_));
}
fn motor_antiSandwich_horizon(self_: Motor, other: Horizon) -> Flector {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), 0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.w * other.g0_)));
    return flector_geometricAntiProduct_flector(geometric_anti_product, motor_antiReverse(self_));
}
fn motor_antiSandwich_line(self_: Motor, other: Line) -> Motor {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(((other.g0_.x * self_.g0_.w) + (other.g0_.z * self_.g0_.y)), ((other.g0_.x * self_.g0_.z) + (other.g0_.y * self_.g0_.w)), ((other.g0_.y * self_.g0_.x) + (other.g0_.z * self_.g0_.w)), (-(other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z))) - (vec4<f32>(other.g0_.y, other.g0_.z, other.g0_.x, other.g0_.x) * self_.g0_.zxyx)), /* e23, e31, e12, scalar */ (vec4<f32>(((other.g0_.x * self_.g1_.w) + (other.g0_.z * self_.g1_.y) + (other.g1_.x * self_.g0_.w) + (other.g1_.z * self_.g0_.y)), ((other.g0_.x * self_.g1_.z) + (other.g0_.y * self_.g1_.w) + (other.g1_.x * self_.g0_.z) + (other.g1_.y * self_.g0_.w)), ((other.g0_.y * self_.g1_.x) + (other.g0_.z * self_.g1_.w) + (other.g1_.y * self_.g0_.x) + (other.g1_.z * self_.g0_.w)), (-(other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))) - (vec4<f32>(other.g0_.y, other.g0_.z, other.g0_.x, other.g0_.x) * self_.g1_.zxyx) - (vec4<f32>(other.g1_.y, other.g1_.z, other.g1_.x, other.g1_.x) * self_.g0_.zxyx)));
    return motor_geometricAntiProduct_motor(geometric_anti_product, motor_antiReverse(self_));
}
fn motor_antiSandwich_motor(self_: Motor, other: Motor) -> Motor {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(((other.g0_.z * self_.g0_.y) + (other.g0_.w * self_.g0_.x)), ((other.g0_.y * self_.g0_.w) + (other.g0_.w * self_.g0_.y)), ((other.g0_.z * self_.g0_.w) + (other.g0_.w * self_.g0_.z)), (-(other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z))) + (other.g0_.xxyw * self_.g0_.wzxw) - (other.g0_.yzxx * self_.g0_.zxyx)), /* e23, e31, e12, scalar */ (vec4<f32>(((other.g0_.z * self_.g1_.y) + (other.g0_.w * self_.g1_.x) + (other.g1_.z * self_.g0_.y) + (other.g1_.w * self_.g0_.x)), ((other.g0_.y * self_.g1_.w) + (other.g0_.w * self_.g1_.y) + (other.g1_.y * self_.g0_.w) + (other.g1_.w * self_.g0_.y)), ((other.g0_.z * self_.g1_.w) + (other.g0_.w * self_.g1_.z) + (other.g1_.z * self_.g0_.w) + (other.g1_.w * self_.g0_.z)), (-(other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))) + (other.g0_.xxyw * self_.g1_.wzxw) - (other.g0_.yzxx * self_.g1_.zxyx) + (other.g1_.xxyw * self_.g0_.wzxw) - (other.g1_.yzxx * self_.g0_.zxyx)));
    return motor_geometricAntiProduct_motor(geometric_anti_product, motor_antiReverse(self_));
}
fn motor_antiSandwich_multiVector(self_: Motor, other: MultiVector) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(((other.g0_.y * self_.g1_.w) - (other.g3_.x * self_.g0_.x) - (other.g3_.y * self_.g0_.y) - (other.g3_.z * self_.g0_.z)), 0.0) - (vec2<f32>(other.g2_.x) * vec2<f32>(self_.g1_.x, self_.g0_.x)) - (vec2<f32>(other.g2_.y) * vec2<f32>(self_.g1_.y, self_.g0_.y)) - (vec2<f32>(other.g2_.z) * vec2<f32>(self_.g1_.z, self_.g0_.z)) + (vec2<f32>(self_.g0_.w) * other.g0_)), /* e1, e2, e3, e4 */ (vec4<f32>(((self_.g0_.y * other.g1_.z) + (self_.g0_.w * other.g1_.x) + (self_.g1_.x * other.g1_.w) + (self_.g1_.y * other.g4_.z) + (self_.g1_.w * other.g4_.x)), ((self_.g0_.z * other.g1_.x) + (self_.g0_.w * other.g1_.y) + (self_.g1_.y * other.g1_.w) + (self_.g1_.z * other.g4_.x) + (self_.g1_.w * other.g4_.y)), ((self_.g0_.z * other.g4_.w) + (self_.g0_.w * other.g1_.z) + (self_.g1_.x * other.g4_.y) + (self_.g1_.z * other.g1_.w) + (self_.g1_.w * other.g4_.z)), ((self_.g0_.z * other.g4_.z) * -1.0)) - (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g0_.y) * other.g4_.yzxy) - (vec4<f32>(other.g1_.y, other.g1_.z, other.g1_.x, other.g4_.x) * self_.g0_.zxyx) + (vec4<f32>(other.g4_.w, other.g4_.w, other.g1_.y, other.g1_.w) * self_.g0_.xyxw)), /* e41, e42, e43 */ ((vec3<f32>(other.g0_.y) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z)) + (vec3<f32>(self_.g0_.y, self_.g0_.w, self_.g0_.w) * other.g2_.zyz) - (vec3<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y) * other.g2_.yzx) + (vec3<f32>(self_.g0_.w, self_.g0_.z, self_.g0_.x) * other.g2_.xxy)), /* e23, e31, e12 */ ((vec3<f32>(other.g0_.x) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z)) + (vec3<f32>(other.g0_.y) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z)) + (vec3<f32>(self_.g0_.y, self_.g0_.w, self_.g0_.w) * other.g3_.zyz) - (vec3<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y) * other.g3_.yzx) + (vec3<f32>(self_.g0_.w, self_.g0_.z, self_.g0_.x) * other.g3_.xxy) + (vec3<f32>(self_.g1_.y, self_.g1_.w, self_.g1_.w) * other.g2_.zyz) - (vec3<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y) * other.g2_.yzx) + (vec3<f32>(self_.g1_.w, self_.g1_.z, self_.g1_.x) * other.g2_.xxy)), /* e423, e431, e412, e321 */ (vec4<f32>(((self_.g0_.y * other.g4_.z) + (self_.g0_.w * other.g4_.x)), ((self_.g0_.z * other.g4_.x) + (self_.g0_.w * other.g4_.y)), ((self_.g0_.z * other.g1_.w) + (self_.g0_.w * other.g4_.z)), (-(self_.g0_.y * other.g1_.y) - (self_.g0_.z * other.g1_.z) - (self_.g1_.x * other.g4_.x) - (self_.g1_.y * other.g4_.y) - (self_.g1_.z * other.g4_.z) + (self_.g1_.w * other.g1_.w))) + (vec4<f32>(other.g1_.w, other.g1_.w, other.g4_.y, other.g4_.w) * self_.g0_.xyxw) - (vec4<f32>(other.g4_.y, other.g4_.z, other.g4_.x, other.g1_.x) * self_.g0_.zxyx)));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, motor_antiReverse(self_));
}
fn motor_antiSandwich_origin(self_: Motor, other: Origin) -> Flector {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g0_.w)), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g1_.w)));
    return flector_geometricAntiProduct_flector(geometric_anti_product, motor_antiReverse(self_));
}
fn motor_antiSandwich_plane(self_: Motor, other: Plane) -> Flector {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(((self_.g0_.x * other.g0_.w) + (self_.g1_.y * other.g0_.z) + (self_.g1_.w * other.g0_.x)), ((self_.g0_.y * other.g0_.w) + (self_.g1_.z * other.g0_.x) + (self_.g1_.w * other.g0_.y)), ((self_.g0_.z * other.g0_.w) + (self_.g1_.x * other.g0_.y) + (self_.g1_.w * other.g0_.z)), (-(self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z))) - (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g0_.x) * other.g0_.yzxx)), /* e423, e431, e412, e321 */ (vec4<f32>((self_.g0_.w * other.g0_.x), (self_.g0_.w * other.g0_.y), (self_.g0_.w * other.g0_.z), (-(self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g1_.x) * other.g0_.yzxx) + (self_.g0_.yzxw * other.g0_.zxyw)));
    return flector_geometricAntiProduct_flector(geometric_anti_product, motor_antiReverse(self_));
}
fn motor_antiSandwich_point(self_: Motor, other: Point) -> Flector {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>(((self_.g0_.y * other.g0_.z) - (self_.g0_.z * other.g0_.y) + (self_.g0_.w * other.g0_.x) + (self_.g1_.x * other.g0_.w)), (-(self_.g0_.x * other.g0_.z) + (self_.g0_.z * other.g0_.x) + (self_.g0_.w * other.g0_.y) + (self_.g1_.y * other.g0_.w)), ((self_.g0_.x * other.g0_.y) - (self_.g0_.y * other.g0_.x) + (self_.g0_.w * other.g0_.z) + (self_.g1_.z * other.g0_.w)), (self_.g0_.w * other.g0_.w)), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.x * other.g0_.w), (self_.g0_.y * other.g0_.w), (self_.g0_.z * other.g0_.w), (-(self_.g0_.x * other.g0_.x) - (self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z) + (self_.g1_.w * other.g0_.w))));
    return flector_geometricAntiProduct_flector(geometric_anti_product, motor_antiReverse(self_));
}
fn motor_antiSandwich_scalar(self_: Motor, other: Scalar) -> Motor {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0), /* e23, e31, e12, scalar */ (vec4<f32>(other.g0_) * self_.g0_));
    return motor_geometricAntiProduct_motor(geometric_anti_product, motor_antiReverse(self_));
}
fn multiVector_antiSandwich_antiScalar(self_: MultiVector, other: AntiScalar) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(other.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(other.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(other.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(other.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_) * self_.g4_));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self_));
}
fn multiVector_antiSandwich_dualNum(self_: MultiVector, other: DualNum) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(((other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x)), (other.g0_.y * self_.g0_.y)), /* e1, e2, e3, e4 */ vec4<f32>((-(other.g0_.x * self_.g4_.x) + (other.g0_.y * self_.g1_.x)), (-(other.g0_.x * self_.g4_.y) + (other.g0_.y * self_.g1_.y)), (-(other.g0_.x * self_.g4_.z) + (other.g0_.y * self_.g1_.z)), (other.g0_.y * self_.g1_.w)), /* e41, e42, e43 */ (vec3<f32>(other.g0_.y) * self_.g2_), /* e23, e31, e12 */ ((vec3<f32>(other.g0_.x) * self_.g2_) + (vec3<f32>(other.g0_.y) * self_.g3_)), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.y * self_.g4_.x), (other.g0_.y * self_.g4_.y), (other.g0_.y * self_.g4_.z), (-(other.g0_.x * self_.g1_.w) + (other.g0_.y * self_.g4_.w))));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self_));
}
fn multiVector_antiSandwich_flector(self_: MultiVector, other: Flector) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>((-(other.g0_.x * self_.g4_.x) - (other.g0_.y * self_.g4_.y) - (other.g0_.z * self_.g4_.z) + (other.g1_.w * self_.g1_.w)), 0.0) - (vec2<f32>(other.g0_.w) * vec2<f32>(self_.g4_.w, self_.g1_.w)) + (vec2<f32>(other.g1_.x) * vec2<f32>(self_.g1_.x, self_.g4_.x)) + (vec2<f32>(other.g1_.y) * vec2<f32>(self_.g1_.y, self_.g4_.y)) + (vec2<f32>(other.g1_.z) * vec2<f32>(self_.g1_.z, self_.g4_.z))), /* e1, e2, e3, e4 */ (vec4<f32>(((self_.g0_.x * other.g1_.x) + (self_.g2_.x * other.g1_.w) + (self_.g2_.y * other.g0_.z) - (self_.g2_.z * other.g0_.y) + (self_.g3_.x * other.g0_.w) + (self_.g3_.y * other.g1_.z)), ((self_.g0_.x * other.g1_.y) - (self_.g2_.x * other.g0_.z) + (self_.g2_.y * other.g1_.w) + (self_.g2_.z * other.g0_.x) + (self_.g3_.y * other.g0_.w) + (self_.g3_.z * other.g1_.x)), ((self_.g0_.x * other.g1_.z) + (self_.g2_.x * other.g0_.y) - (self_.g2_.y * other.g0_.x) + (self_.g2_.z * other.g1_.w) + (self_.g3_.x * other.g1_.y) + (self_.g3_.z * other.g0_.w)), (-(self_.g2_.y * other.g1_.y) - (self_.g2_.z * other.g1_.z))) + (vec4<f32>(self_.g0_.y) * other.g0_) - (vec4<f32>(self_.g3_.z, self_.g3_.x, self_.g3_.y, self_.g2_.x) * other.g1_.yzxx)), /* e41, e42, e43 */ (vec3<f32>(((other.g1_.y * self_.g4_.z) - (other.g1_.z * self_.g4_.y)), (-(other.g1_.x * self_.g4_.z) + (other.g1_.z * self_.g4_.x)), ((other.g1_.x * self_.g4_.y) - (other.g1_.y * self_.g4_.x))) - (vec3<f32>(other.g0_.w) * vec3<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z)) - (vec3<f32>(self_.g1_.w) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z))), /* e23, e31, e12 */ (vec3<f32>((-(other.g0_.y * self_.g4_.z) + (other.g0_.z * self_.g4_.y) + (other.g1_.y * self_.g1_.z) - (other.g1_.z * self_.g1_.y)), ((other.g0_.x * self_.g4_.z) - (other.g0_.z * self_.g4_.x) - (other.g1_.x * self_.g1_.z) + (other.g1_.z * self_.g1_.x)), (-(other.g0_.x * self_.g4_.y) + (other.g0_.y * self_.g4_.x) + (other.g1_.x * self_.g1_.y) - (other.g1_.y * self_.g1_.x))) - (vec3<f32>(other.g0_.w) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z)) + (vec3<f32>(other.g1_.w) * vec3<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z)) + (vec3<f32>(self_.g1_.w) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z)) - (vec3<f32>(self_.g4_.w) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z))), /* e423, e431, e412, e321 */ (vec4<f32>((self_.g2_.y * other.g1_.z), (self_.g2_.z * other.g1_.x), (self_.g2_.x * other.g1_.y), (-(self_.g2_.x * other.g0_.x) - (self_.g2_.y * other.g0_.y) - (self_.g2_.z * other.g0_.z) - (self_.g3_.y * other.g1_.y) - (self_.g3_.z * other.g1_.z))) + (vec4<f32>(self_.g0_.y) * other.g1_) + (vec4<f32>(other.g0_.w) * vec4<f32>(self_.g2_.x, self_.g2_.y, self_.g2_.z, self_.g0_.x)) - (vec4<f32>(self_.g2_.z, self_.g2_.x, self_.g2_.y, self_.g3_.x) * other.g1_.yzxx)));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self_));
}
fn multiVector_antiSandwich_horizon(self_: MultiVector, other: Horizon) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>((self_.g1_.w * other.g0_), 0.0), /* e1, e2, e3, e4 */ vec4<f32>((self_.g2_.x * other.g0_), (self_.g2_.y * other.g0_), (self_.g2_.z * other.g0_), 0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ (vec3<f32>(other.g0_) * vec3<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z)), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.y * other.g0_)));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self_));
}
fn multiVector_antiSandwich_line(self_: MultiVector, other: Line) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>((-(other.g1_.x * self_.g2_.x) - (other.g1_.y * self_.g2_.y) - (other.g1_.z * self_.g2_.z)), 0.0) - (vec2<f32>(other.g0_.x) * vec2<f32>(self_.g3_.x, self_.g2_.x)) - (vec2<f32>(other.g0_.y) * vec2<f32>(self_.g3_.y, self_.g2_.y)) - (vec2<f32>(other.g0_.z) * vec2<f32>(self_.g3_.z, self_.g2_.z))), /* e1, e2, e3, e4 */ (vec4<f32>(((other.g0_.x * self_.g4_.w) - (other.g0_.y * self_.g1_.z) + (other.g0_.z * self_.g1_.y) - (other.g1_.x * self_.g1_.w) + (other.g1_.y * self_.g4_.z)), ((other.g0_.x * self_.g1_.z) + (other.g0_.y * self_.g4_.w) - (other.g0_.z * self_.g1_.x) - (other.g1_.y * self_.g1_.w) + (other.g1_.z * self_.g4_.x)), (-(other.g0_.x * self_.g1_.y) + (other.g0_.y * self_.g1_.x) + (other.g0_.z * self_.g4_.w) + (other.g1_.x * self_.g4_.y) - (other.g1_.z * self_.g1_.w)), (-(other.g0_.y * self_.g4_.y) - (other.g0_.z * self_.g4_.z))) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.x) * self_.g4_.yzxx)), /* e41, e42, e43 */ ((vec3<f32>(self_.g0_.y) * other.g0_) - (other.g0_.yzx * self_.g2_.zxy) + (other.g0_.zxy * self_.g2_.yzx)), /* e23, e31, e12 */ ((vec3<f32>(self_.g0_.x) * other.g0_) + (vec3<f32>(self_.g0_.y) * other.g1_) - (other.g0_.yzx * self_.g3_.zxy) + (other.g0_.zxy * self_.g3_.yzx) - (other.g1_.yzx * self_.g2_.zxy) + (other.g1_.zxy * self_.g2_.yzx)), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.x * self_.g1_.w) - (other.g0_.y * self_.g4_.z)), ((other.g0_.y * self_.g1_.w) - (other.g0_.z * self_.g4_.x)), (-(other.g0_.x * self_.g4_.y) + (other.g0_.z * self_.g1_.w)), (-(other.g0_.x * self_.g1_.x) - (other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) + (other.g1_.y * self_.g4_.y) + (other.g1_.z * self_.g4_.z))) + (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.y, other.g1_.x) * self_.g4_.yzxx)));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self_));
}
fn multiVector_antiSandwich_motor(self_: MultiVector, other: Motor) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(((self_.g0_.y * other.g1_.w) - (self_.g3_.x * other.g0_.x) - (self_.g3_.y * other.g0_.y) - (self_.g3_.z * other.g0_.z)), 0.0) - (vec2<f32>(self_.g2_.x) * vec2<f32>(other.g1_.x, other.g0_.x)) - (vec2<f32>(self_.g2_.y) * vec2<f32>(other.g1_.y, other.g0_.y)) - (vec2<f32>(self_.g2_.z) * vec2<f32>(other.g1_.z, other.g0_.z)) + (vec2<f32>(other.g0_.w) * self_.g0_)), /* e1, e2, e3, e4 */ (vec4<f32>(((other.g0_.z * self_.g1_.y) + (other.g0_.w * self_.g1_.x) - (other.g1_.x * self_.g1_.w) + (other.g1_.y * self_.g4_.z)), ((other.g0_.y * self_.g4_.w) + (other.g0_.w * self_.g1_.y) - (other.g1_.y * self_.g1_.w) + (other.g1_.z * self_.g4_.x)), ((other.g0_.z * self_.g4_.w) + (other.g0_.w * self_.g1_.z) + (other.g1_.x * self_.g4_.y) - (other.g1_.z * self_.g1_.w)), 0.0) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.y) * self_.g4_.yzxy) - (vec4<f32>(other.g1_.w, other.g1_.w, other.g1_.w, other.g0_.z) * self_.g4_.xyzz) - (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g4_.x) * other.g0_.yzxx) + (vec4<f32>(self_.g4_.w, self_.g1_.z, self_.g1_.x, self_.g1_.w) * other.g0_.xxyw)), /* e41, e42, e43 */ ((vec3<f32>(self_.g0_.y) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z)) - (vec3<f32>(other.g0_.y, other.g0_.z, other.g0_.x) * self_.g2_.zxy) + (vec3<f32>(other.g0_.z, other.g0_.x, other.g0_.w) * self_.g2_.yzz) + (vec3<f32>(other.g0_.w, other.g0_.w, other.g0_.y) * self_.g2_.xyx)), /* e23, e31, e12 */ ((vec3<f32>(self_.g0_.x) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z)) + (vec3<f32>(self_.g0_.y) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z)) - (vec3<f32>(other.g0_.y, other.g0_.z, other.g0_.x) * self_.g3_.zxy) + (vec3<f32>(other.g0_.z, other.g0_.x, other.g0_.w) * self_.g3_.yzz) + (vec3<f32>(other.g0_.w, other.g0_.w, other.g0_.y) * self_.g3_.xyx) - (vec3<f32>(other.g1_.y, other.g1_.z, other.g1_.x) * self_.g2_.zxy) + (vec3<f32>(other.g1_.z, other.g1_.x, other.g1_.w) * self_.g2_.yzz) + (vec3<f32>(other.g1_.w, other.g1_.w, other.g1_.y) * self_.g2_.xyx)), /* e423, e431, e412, e321 */ (vec4<f32>((other.g0_.w * self_.g4_.x), (other.g0_.y * self_.g1_.w), (other.g0_.z * self_.g1_.w), (-(other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) + (other.g1_.y * self_.g4_.y) + (other.g1_.z * self_.g4_.z) - (other.g1_.w * self_.g1_.w))) + (vec4<f32>(other.g0_.z, other.g0_.w, other.g0_.w, other.g1_.x) * self_.g4_.yyzx) + (vec4<f32>(self_.g1_.w, self_.g4_.z, self_.g4_.x, self_.g4_.w) * other.g0_.xxyw) - (vec4<f32>(self_.g4_.z, self_.g4_.x, self_.g4_.y, self_.g1_.x) * other.g0_.yzxx)));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self_));
}
fn multiVector_antiSandwich_multiVector(self_: MultiVector, other: MultiVector) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(((other.g0_.y * self_.g0_.x) - (other.g3_.x * self_.g2_.x) - (other.g3_.y * self_.g2_.y) - (other.g3_.z * self_.g2_.z) - (other.g1_.x * self_.g4_.x) - (other.g1_.y * self_.g4_.y) - (other.g1_.z * self_.g4_.z) + (other.g4_.w * self_.g1_.w)), 0.0) + (vec2<f32>(self_.g0_.y) * other.g0_) - (vec2<f32>(other.g2_.x) * vec2<f32>(self_.g3_.x, self_.g2_.x)) - (vec2<f32>(other.g2_.y) * vec2<f32>(self_.g3_.y, self_.g2_.y)) - (vec2<f32>(other.g2_.z) * vec2<f32>(self_.g3_.z, self_.g2_.z)) - (vec2<f32>(other.g1_.w) * vec2<f32>(self_.g4_.w, self_.g1_.w)) + (vec2<f32>(other.g4_.x) * vec2<f32>(self_.g1_.x, self_.g4_.x)) + (vec2<f32>(other.g4_.y) * vec2<f32>(self_.g1_.y, self_.g4_.y)) + (vec2<f32>(other.g4_.z) * vec2<f32>(self_.g1_.z, self_.g4_.z))), /* e1, e2, e3, e4 */ (vec4<f32>(((self_.g0_.x * other.g4_.x) + (other.g2_.x * self_.g4_.w) - (other.g2_.y * self_.g1_.z) + (other.g2_.z * self_.g1_.y) - (other.g3_.x * self_.g1_.w) + (other.g3_.y * self_.g4_.z) + (self_.g2_.x * other.g4_.w) + (self_.g2_.y * other.g1_.z) - (self_.g2_.z * other.g1_.y) + (self_.g3_.x * other.g1_.w) + (self_.g3_.y * other.g4_.z)), ((self_.g0_.x * other.g4_.y) + (other.g2_.x * self_.g1_.z) + (other.g2_.y * self_.g4_.w) - (other.g2_.z * self_.g1_.x) - (other.g3_.y * self_.g1_.w) + (other.g3_.z * self_.g4_.x) - (self_.g2_.x * other.g1_.z) + (self_.g2_.y * other.g4_.w) + (self_.g2_.z * other.g1_.x) + (self_.g3_.y * other.g1_.w) + (self_.g3_.z * other.g4_.x)), ((self_.g0_.x * other.g4_.z) - (other.g2_.x * self_.g1_.y) + (other.g2_.y * self_.g1_.x) + (other.g2_.z * self_.g4_.w) + (other.g3_.x * self_.g4_.y) - (other.g3_.z * self_.g1_.w) + (self_.g2_.x * other.g1_.y) - (self_.g2_.y * other.g1_.x) + (self_.g2_.z * other.g4_.w) + (self_.g3_.x * other.g4_.y) + (self_.g3_.z * other.g1_.w)), (-(other.g2_.z * self_.g4_.z) - (self_.g2_.y * other.g4_.y) - (self_.g2_.z * other.g4_.z))) + (vec4<f32>(other.g0_.y) * self_.g1_) + (vec4<f32>(self_.g0_.y) * other.g1_) - (vec4<f32>(other.g0_.x, other.g0_.x, other.g0_.x, other.g2_.x) * self_.g4_.xyzx) - (vec4<f32>(other.g3_.z, other.g3_.x, other.g3_.y, other.g2_.y) * self_.g4_.yzxy) - (vec4<f32>(self_.g3_.z, self_.g3_.x, self_.g3_.y, self_.g2_.x) * other.g4_.yzxx)), /* e41, e42, e43 */ (vec3<f32>(((other.g4_.y * self_.g4_.z) - (other.g4_.z * self_.g4_.y)), (-(other.g4_.x * self_.g4_.z) + (other.g4_.z * self_.g4_.x)), ((other.g4_.x * self_.g4_.y) - (other.g4_.y * self_.g4_.x))) + (vec3<f32>(other.g0_.y) * self_.g2_) + (vec3<f32>(self_.g0_.y) * other.g2_) - (vec3<f32>(other.g1_.w) * vec3<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z)) - (vec3<f32>(self_.g1_.w) * vec3<f32>(other.g4_.x, other.g4_.y, other.g4_.z)) - (other.g2_.yzx * self_.g2_.zxy) + (other.g2_.zxy * self_.g2_.yzx)), /* e23, e31, e12 */ (vec3<f32>((-(other.g1_.y * self_.g4_.z) + (other.g1_.z * self_.g4_.y) + (other.g4_.y * self_.g1_.z) - (other.g4_.z * self_.g1_.y)), ((other.g1_.x * self_.g4_.z) - (other.g1_.z * self_.g4_.x) - (other.g4_.x * self_.g1_.z) + (other.g4_.z * self_.g1_.x)), (-(other.g1_.x * self_.g4_.y) + (other.g1_.y * self_.g4_.x) + (other.g4_.x * self_.g1_.y) - (other.g4_.y * self_.g1_.x))) + (vec3<f32>(other.g0_.x) * self_.g2_) + (vec3<f32>(other.g0_.y) * self_.g3_) + (vec3<f32>(self_.g0_.x) * other.g2_) + (vec3<f32>(self_.g0_.y) * other.g3_) - (vec3<f32>(other.g1_.w) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z)) + (vec3<f32>(other.g4_.w) * vec3<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z)) + (vec3<f32>(self_.g1_.w) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z)) - (vec3<f32>(self_.g4_.w) * vec3<f32>(other.g4_.x, other.g4_.y, other.g4_.z)) - (other.g2_.yzx * self_.g3_.zxy) + (other.g2_.zxy * self_.g3_.yzx) - (other.g3_.yzx * self_.g2_.zxy) + (other.g3_.zxy * self_.g2_.yzx)), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g2_.x * self_.g1_.w) - (other.g2_.y * self_.g4_.z) + (self_.g2_.y * other.g4_.z)), ((other.g2_.y * self_.g1_.w) - (other.g2_.z * self_.g4_.x) + (self_.g2_.z * other.g4_.x)), (-(other.g2_.x * self_.g4_.y) + (other.g2_.z * self_.g1_.w) + (self_.g2_.x * other.g4_.y)), (-(other.g0_.x * self_.g1_.w) - (other.g2_.x * self_.g1_.x) - (other.g2_.y * self_.g1_.y) - (other.g2_.z * self_.g1_.z) + (other.g3_.y * self_.g4_.y) + (other.g3_.z * self_.g4_.z) - (self_.g2_.x * other.g1_.x) - (self_.g2_.y * other.g1_.y) - (self_.g2_.z * other.g1_.z) - (self_.g3_.y * other.g4_.y) - (self_.g3_.z * other.g4_.z))) + (vec4<f32>(other.g0_.y) * self_.g4_) + (vec4<f32>(self_.g0_.y) * other.g4_) + (vec4<f32>(other.g1_.w) * vec4<f32>(self_.g2_.x, self_.g2_.y, self_.g2_.z, self_.g0_.x)) + (vec4<f32>(other.g2_.z, other.g2_.x, other.g2_.y, other.g3_.x) * self_.g4_.yzxx) - (vec4<f32>(self_.g2_.z, self_.g2_.x, self_.g2_.y, self_.g3_.x) * other.g4_.yzxx)));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self_));
}
fn multiVector_antiSandwich_origin(self_: MultiVector, other: Origin) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(other.g0_) * vec2<f32>(self_.g4_.w, self_.g1_.w) * vec2<f32>(-1.0)), /* e1, e2, e3, e4 */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g3_.x, self_.g3_.y, self_.g3_.z, self_.g0_.y)), /* e41, e42, e43 */ (vec3<f32>(other.g0_) * vec3<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z) * vec3<f32>(-1.0)), /* e23, e31, e12 */ (vec3<f32>(other.g0_) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z) * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g2_.x, self_.g2_.y, self_.g2_.z, self_.g0_.x)));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self_));
}
fn multiVector_antiSandwich_plane(self_: MultiVector, other: Plane) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>((self_.g1_.w * other.g0_.w), 0.0) + (vec2<f32>(other.g0_.x) * vec2<f32>(self_.g1_.x, self_.g4_.x)) + (vec2<f32>(other.g0_.y) * vec2<f32>(self_.g1_.y, self_.g4_.y)) + (vec2<f32>(other.g0_.z) * vec2<f32>(self_.g1_.z, self_.g4_.z))), /* e1, e2, e3, e4 */ (vec4<f32>(((self_.g0_.x * other.g0_.x) + (self_.g2_.x * other.g0_.w) + (self_.g3_.y * other.g0_.z)), ((self_.g0_.x * other.g0_.y) + (self_.g2_.y * other.g0_.w) + (self_.g3_.z * other.g0_.x)), ((self_.g0_.x * other.g0_.z) + (self_.g2_.z * other.g0_.w) + (self_.g3_.x * other.g0_.y)), (-(self_.g2_.y * other.g0_.y) - (self_.g2_.z * other.g0_.z))) - (vec4<f32>(self_.g3_.z, self_.g3_.x, self_.g3_.y, self_.g2_.x) * other.g0_.yzxx)), /* e41, e42, e43 */ (vec3<f32>((-(self_.g4_.y * other.g0_.z) + (self_.g4_.z * other.g0_.y)), ((self_.g4_.x * other.g0_.z) - (self_.g4_.z * other.g0_.x)), (-(self_.g4_.x * other.g0_.y) + (self_.g4_.y * other.g0_.x))) - (vec3<f32>(self_.g1_.w) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z))), /* e23, e31, e12 */ (vec3<f32>((-(self_.g1_.y * other.g0_.z) + (self_.g1_.z * other.g0_.y)), ((self_.g1_.x * other.g0_.z) - (self_.g1_.z * other.g0_.x)), (-(self_.g1_.x * other.g0_.y) + (self_.g1_.y * other.g0_.x))) - (vec3<f32>(self_.g4_.w) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z)) + (vec3<f32>(other.g0_.w) * vec3<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z))), /* e423, e431, e412, e321 */ (vec4<f32>((self_.g2_.y * other.g0_.z), (self_.g2_.z * other.g0_.x), (self_.g2_.x * other.g0_.y), (-(self_.g3_.y * other.g0_.y) - (self_.g3_.z * other.g0_.z))) + (vec4<f32>(self_.g0_.y) * other.g0_) - (vec4<f32>(self_.g2_.z, self_.g2_.x, self_.g2_.y, self_.g3_.x) * other.g0_.yzxx)));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self_));
}
fn multiVector_antiSandwich_point(self_: MultiVector, other: Point) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>((-(self_.g4_.x * other.g0_.x) - (self_.g4_.y * other.g0_.y) - (self_.g4_.z * other.g0_.z) - (self_.g4_.w * other.g0_.w)), (self_.g1_.w * other.g0_.w * -1.0)), /* e1, e2, e3, e4 */ vec4<f32>(((self_.g0_.y * other.g0_.x) + (self_.g2_.y * other.g0_.z) - (self_.g2_.z * other.g0_.y) + (self_.g3_.x * other.g0_.w)), ((self_.g0_.y * other.g0_.y) - (self_.g2_.x * other.g0_.z) + (self_.g2_.z * other.g0_.x) + (self_.g3_.y * other.g0_.w)), ((self_.g0_.y * other.g0_.z) + (self_.g2_.x * other.g0_.y) - (self_.g2_.y * other.g0_.x) + (self_.g3_.z * other.g0_.w)), (self_.g0_.y * other.g0_.w)), /* e41, e42, e43 */ (vec3<f32>(other.g0_.w) * vec3<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z) * vec3<f32>(-1.0)), /* e23, e31, e12 */ (vec3<f32>(((self_.g4_.y * other.g0_.z) - (self_.g4_.z * other.g0_.y)), (-(self_.g4_.x * other.g0_.z) + (self_.g4_.z * other.g0_.x)), ((self_.g4_.x * other.g0_.y) - (self_.g4_.y * other.g0_.x))) + (vec3<f32>(self_.g1_.w) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z)) - (vec3<f32>(other.g0_.w) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z))), /* e423, e431, e412, e321 */ vec4<f32>((self_.g2_.x * other.g0_.w), (self_.g2_.y * other.g0_.w), (self_.g2_.z * other.g0_.w), ((self_.g0_.x * other.g0_.w) - (self_.g2_.x * other.g0_.x) - (self_.g2_.y * other.g0_.y) - (self_.g2_.z * other.g0_.z))));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self_));
}
fn multiVector_antiSandwich_scalar(self_: MultiVector, other: Scalar) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>((self_.g0_.y * other.g0_), 0.0), /* e1, e2, e3, e4 */ vec4<f32>((self_.g4_.x * other.g0_ * -1.0), (self_.g4_.y * other.g0_ * -1.0), (self_.g4_.z * other.g0_ * -1.0), 0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ (vec3<f32>(other.g0_) * self_.g2_), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g1_.w * other.g0_ * -1.0)));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self_));
}
fn origin_antiSandwich_antiScalar(self_: Origin, other: AntiScalar) -> AntiScalar {
    let geometric_anti_product: Origin = Origin(/* e4 */ (other.g0_ * self_.g0_));
    return origin_geometricAntiProduct_origin(geometric_anti_product, origin_antiReverse(self_));
}
fn origin_antiSandwich_dualNum(self_: Origin, other: DualNum) -> Motor {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.y * self_.g0_)), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.x * self_.g0_ * -1.0)));
    return flector_geometricAntiProduct_flector(geometric_anti_product, origin_antiReverse(self_));
}
fn origin_antiSandwich_flector(self_: Origin, other: Flector) -> Flector {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, other.g0_.w) * vec4<f32>(-1.0)), /* e23, e31, e12, scalar */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.w)));
    return motor_geometricAntiProduct_motor(geometric_anti_product, origin_antiReverse(self_));
}
fn origin_antiSandwich_horizon(self_: Origin, other: Horizon) -> Horizon {
    let geometric_anti_product: Scalar = Scalar(/* scalar */ (other.g0_ * self_.g0_));
    return scalar_geometricAntiProduct_scalar(geometric_anti_product, origin_antiReverse(self_));
}
fn origin_antiSandwich_line(self_: Origin, other: Line) -> Motor {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g1_.x * self_.g0_ * -1.0), (other.g1_.y * self_.g0_ * -1.0), (other.g1_.z * self_.g0_ * -1.0), 0.0), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), 0.0));
    return flector_geometricAntiProduct_flector(geometric_anti_product, origin_antiReverse(self_));
}
fn origin_antiSandwich_motor(self_: Origin, other: Motor) -> Motor {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, other.g0_.w) * vec4<f32>(-1.0, -1.0, -1.0, 1.0)), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.w) * vec4<f32>(1.0, 1.0, 1.0, -1.0)));
    return flector_geometricAntiProduct_flector(geometric_anti_product, origin_antiReverse(self_));
}
fn origin_antiSandwich_multiVector(self_: Origin, other: MultiVector) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(self_.g0_) * vec2<f32>(other.g4_.w, other.g1_.w) * vec2<f32>(1.0, -1.0)), /* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g3_.x, other.g3_.y, other.g3_.z, other.g0_.y) * vec4<f32>(-1.0, -1.0, -1.0, 1.0)), /* e41, e42, e43 */ (vec3<f32>(self_.g0_) * vec3<f32>(other.g4_.x, other.g4_.y, other.g4_.z) * vec3<f32>(-1.0)), /* e23, e31, e12 */ (vec3<f32>(self_.g0_) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z)), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g2_.x, other.g2_.y, other.g2_.z, other.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, -1.0)));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, origin_antiReverse(self_));
}
fn origin_antiSandwich_origin(self_: Origin, other: Origin) -> Origin {
    let geometric_anti_product: AntiScalar = AntiScalar(/* e1234 */ (other.g0_ * self_.g0_ * -1.0));
    return antiScalar_geometricAntiProduct_antiScalar(geometric_anti_product, origin_antiReverse(self_));
}
fn origin_antiSandwich_plane(self_: Origin, other: Plane) -> Flector {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g0_.x * self_.g0_ * -1.0), (other.g0_.y * self_.g0_ * -1.0), (other.g0_.z * self_.g0_ * -1.0), 0.0), /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.w * self_.g0_)));
    return motor_geometricAntiProduct_motor(geometric_anti_product, origin_antiReverse(self_));
}
fn origin_antiSandwich_point(self_: Origin, other: Point) -> Flector {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.w * self_.g0_ * -1.0)), /* e23, e31, e12, scalar */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), 0.0));
    return motor_geometricAntiProduct_motor(geometric_anti_product, origin_antiReverse(self_));
}
fn origin_antiSandwich_scalar(self_: Origin, other: Scalar) -> Scalar {
    let geometric_anti_product: Horizon = Horizon(/* e321 */ (self_.g0_ * other.g0_ * -1.0));
    return horizon_geometricAntiProduct_horizon(geometric_anti_product, origin_antiReverse(self_));
}
fn plane_antiSandwich_antiScalar(self_: Plane, other: AntiScalar) -> Motor {
    let geometric_anti_product: Plane = Plane(/* e423, e431, e412, e321 */ (vec4<f32>(other.g0_) * self_.g0_));
    return plane_geometricAntiProduct_plane(geometric_anti_product, plane_antiReverse(self_));
}
fn plane_antiSandwich_dualNum(self_: Plane, other: DualNum) -> Motor {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g0_.x * -1.0), (other.g0_.x * self_.g0_.y * -1.0), (other.g0_.x * self_.g0_.z * -1.0), 0.0), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_.y) * self_.g0_));
    return flector_geometricAntiProduct_flector(geometric_anti_product, plane_antiReverse(self_));
}
fn plane_antiSandwich_flector(self_: Plane, other: Flector) -> Flector {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>((-(other.g0_.w * self_.g0_.x) - (other.g1_.z * self_.g0_.y)), (-(other.g0_.w * self_.g0_.y) - (other.g1_.x * self_.g0_.z)), (-(other.g0_.w * self_.g0_.z) - (other.g1_.y * self_.g0_.x)), ((other.g1_.y * self_.g0_.y) + (other.g1_.z * self_.g0_.z))) + (other.g1_.yzxx * self_.g0_.zxyx)), /* e23, e31, e12, scalar */ (vec4<f32>(((other.g0_.z * self_.g0_.y) + (other.g1_.w * self_.g0_.x)), ((other.g0_.x * self_.g0_.z) + (other.g1_.w * self_.g0_.y)), ((other.g0_.y * self_.g0_.x) + (other.g1_.w * self_.g0_.z)), (-(other.g0_.z * self_.g0_.z) - (other.g0_.w * self_.g0_.w))) - (vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, other.g0_.y) * self_.g0_.wwwy) - (other.g0_.yzxx * self_.g0_.zxyx)));
    return motor_geometricAntiProduct_motor(geometric_anti_product, plane_antiReverse(self_));
}
fn plane_antiSandwich_horizon(self_: Plane, other: Horizon) -> Flector {
    let geometric_anti_product: Line = Line(/* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ (vec3<f32>(other.g0_) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z)));
    return line_geometricAntiProduct_line(geometric_anti_product, plane_antiReverse(self_));
}
fn plane_antiSandwich_line(self_: Plane, other: Line) -> Motor {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(((other.g0_.x * self_.g0_.w) + (other.g1_.y * self_.g0_.z)), ((other.g0_.y * self_.g0_.w) + (other.g1_.z * self_.g0_.x)), ((other.g0_.z * self_.g0_.w) + (other.g1_.x * self_.g0_.y)), (-(other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z))) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.x) * self_.g0_.yzxx)), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.y * self_.g0_.z) * -1.0), ((other.g0_.z * self_.g0_.x) * -1.0), ((other.g0_.x * self_.g0_.y) * -1.0), ((other.g1_.y * self_.g0_.y) + (other.g1_.z * self_.g0_.z))) + (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.y, other.g1_.x) * self_.g0_.yzxx)));
    return flector_geometricAntiProduct_flector(geometric_anti_product, plane_antiReverse(self_));
}
fn plane_antiSandwich_motor(self_: Plane, other: Motor) -> Motor {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(((other.g0_.x * self_.g0_.w) + (other.g1_.y * self_.g0_.z)), ((other.g0_.y * self_.g0_.w) + (other.g1_.z * self_.g0_.x)), ((other.g0_.z * self_.g0_.w) + (other.g1_.x * self_.g0_.y)), ((other.g0_.z * self_.g0_.z) * -1.0)) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.x) * self_.g0_.yzxx) - (vec4<f32>(other.g1_.w, other.g1_.w, other.g1_.w, other.g0_.y) * self_.g0_.xyzy)), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.y * self_.g0_.z) * -1.0), ((other.g0_.z * self_.g0_.x) * -1.0), ((other.g0_.x * self_.g0_.y) * -1.0), ((other.g1_.y * self_.g0_.y) + (other.g1_.z * self_.g0_.z))) + (vec4<f32>(other.g0_.w, other.g0_.w, other.g0_.w, other.g1_.x) * self_.g0_.xyzx) + (other.g0_.zxyw * self_.g0_.yzxw)));
    return flector_geometricAntiProduct_flector(geometric_anti_product, plane_antiReverse(self_));
}
fn plane_antiSandwich_multiVector(self_: Plane, other: MultiVector) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>((-(other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z) - (other.g1_.w * self_.g0_.w)), ((other.g4_.x * self_.g0_.x) + (other.g4_.y * self_.g0_.y) + (other.g4_.z * self_.g0_.z))), /* e1, e2, e3, e4 */ (vec4<f32>(((other.g2_.x * self_.g0_.w) + (other.g3_.y * self_.g0_.z)), ((other.g2_.y * self_.g0_.w) + (other.g3_.z * self_.g0_.x)), ((other.g2_.z * self_.g0_.w) + (other.g3_.x * self_.g0_.y)), ((other.g2_.z * self_.g0_.z) * -1.0)) - (vec4<f32>(other.g0_.x, other.g0_.x, other.g0_.x, other.g2_.x) * self_.g0_.xyzx) - (vec4<f32>(other.g3_.z, other.g3_.x, other.g3_.y, other.g2_.y) * self_.g0_.yzxy)), /* e41, e42, e43 */ (vec3<f32>(((other.g4_.y * self_.g0_.z) - (other.g4_.z * self_.g0_.y)), (-(other.g4_.x * self_.g0_.z) + (other.g4_.z * self_.g0_.x)), ((other.g4_.x * self_.g0_.y) - (other.g4_.y * self_.g0_.x))) - (vec3<f32>(other.g1_.w) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z))), /* e23, e31, e12 */ (vec3<f32>((-(other.g1_.y * self_.g0_.z) + (other.g1_.z * self_.g0_.y)), ((other.g1_.x * self_.g0_.z) - (other.g1_.z * self_.g0_.x)), (-(other.g1_.x * self_.g0_.y) + (other.g1_.y * self_.g0_.x))) + (vec3<f32>(other.g4_.w) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z)) - (vec3<f32>(self_.g0_.w) * vec3<f32>(other.g4_.x, other.g4_.y, other.g4_.z))), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g2_.y * self_.g0_.z) * -1.0), ((other.g2_.z * self_.g0_.x) * -1.0), ((other.g2_.x * self_.g0_.y) * -1.0), ((other.g3_.y * self_.g0_.y) + (other.g3_.z * self_.g0_.z))) + (vec4<f32>(other.g0_.y) * self_.g0_) + (vec4<f32>(other.g2_.z, other.g2_.x, other.g2_.y, other.g3_.x) * self_.g0_.yzxx)));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, plane_antiReverse(self_));
}
fn plane_antiSandwich_origin(self_: Plane, other: Origin) -> Flector {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g0_.x * other.g0_ * -1.0), (self_.g0_.y * other.g0_ * -1.0), (self_.g0_.z * other.g0_ * -1.0), 0.0), /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.w * other.g0_ * -1.0)));
    return motor_geometricAntiProduct_motor(geometric_anti_product, plane_antiReverse(self_));
}
fn plane_antiSandwich_plane(self_: Plane, other: Plane) -> Flector {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(((other.g0_.z * self_.g0_.y) * -1.0), ((other.g0_.x * self_.g0_.z) * -1.0), ((other.g0_.y * self_.g0_.x) * -1.0), ((other.g0_.y * self_.g0_.y) + (other.g0_.z * self_.g0_.z))) + (other.g0_.yzxx * self_.g0_.zxyx)), /* e23, e31, e12, scalar */ vec4<f32>((-(other.g0_.x * self_.g0_.w) + (other.g0_.w * self_.g0_.x)), (-(other.g0_.y * self_.g0_.w) + (other.g0_.w * self_.g0_.y)), (-(other.g0_.z * self_.g0_.w) + (other.g0_.w * self_.g0_.z)), 0.0));
    return motor_geometricAntiProduct_motor(geometric_anti_product, plane_antiReverse(self_));
}
fn plane_antiSandwich_point(self_: Plane, other: Point) -> Flector {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g0_.x * other.g0_.w * -1.0), (self_.g0_.y * other.g0_.w * -1.0), (self_.g0_.z * other.g0_.w * -1.0), 0.0), /* e23, e31, e12, scalar */ (vec4<f32>((self_.g0_.y * other.g0_.z), (self_.g0_.z * other.g0_.x), (self_.g0_.x * other.g0_.y), (-(self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z) - (self_.g0_.w * other.g0_.w))) - (self_.g0_.zxyx * other.g0_.yzxx)));
    return motor_geometricAntiProduct_motor(geometric_anti_product, plane_antiReverse(self_));
}
fn plane_antiSandwich_scalar(self_: Plane, other: Scalar) -> Motor {
    let geometric_anti_product: Point = Point(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g0_ * -1.0), (self_.g0_.y * other.g0_ * -1.0), (self_.g0_.z * other.g0_ * -1.0), 0.0));
    return point_geometricAntiProduct_point(geometric_anti_product, plane_antiReverse(self_));
}
fn point_antiSandwich_antiScalar(self_: Point, other: AntiScalar) -> Motor {
    let geometric_anti_product: Point = Point(/* e1, e2, e3, e4 */ (vec4<f32>(other.g0_) * self_.g0_));
    return point_geometricAntiProduct_point(geometric_anti_product, point_antiReverse(self_));
}
fn point_antiSandwich_dualNum(self_: Point, other: DualNum) -> Motor {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(other.g0_.y) * self_.g0_), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.x * self_.g0_.w * -1.0)));
    return flector_geometricAntiProduct_flector(geometric_anti_product, point_antiReverse(self_));
}
fn point_antiSandwich_flector(self_: Point, other: Flector) -> Flector {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(self_.g0_.w) * vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, other.g0_.w) * vec4<f32>(-1.0)), /* e23, e31, e12, scalar */ (vec4<f32>((-(other.g0_.w * self_.g0_.x) - (other.g1_.z * self_.g0_.y)), (-(other.g0_.w * self_.g0_.y) - (other.g1_.x * self_.g0_.z)), (-(other.g0_.w * self_.g0_.z) - (other.g1_.y * self_.g0_.x)), ((other.g1_.z * self_.g0_.z) + (other.g1_.w * self_.g0_.w))) + (vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.x) * self_.g0_.wwwx) + (other.g1_.yzxy * self_.g0_.zxyy)));
    return motor_geometricAntiProduct_motor(geometric_anti_product, point_antiReverse(self_));
}
fn point_antiSandwich_horizon(self_: Point, other: Horizon) -> Horizon {
    let geometric_anti_product: Scalar = Scalar(/* scalar */ (self_.g0_.w * other.g0_));
    return scalar_geometricAntiProduct_scalar(geometric_anti_product, point_antiReverse(self_));
}
fn point_antiSandwich_line(self_: Point, other: Line) -> Motor {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>((-(other.g0_.y * self_.g0_.z) + (other.g0_.z * self_.g0_.y) - (other.g1_.x * self_.g0_.w)), ((other.g0_.x * self_.g0_.z) - (other.g0_.z * self_.g0_.x) - (other.g1_.y * self_.g0_.w)), (-(other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x) - (other.g1_.z * self_.g0_.w)), 0.0), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.x * self_.g0_.w), (other.g0_.y * self_.g0_.w), (other.g0_.z * self_.g0_.w), (-(other.g0_.x * self_.g0_.x) - (other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z))));
    return flector_geometricAntiProduct_flector(geometric_anti_product, point_antiReverse(self_));
}
fn point_antiSandwich_motor(self_: Point, other: Motor) -> Motor {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>((-(other.g0_.y * self_.g0_.z) + (other.g0_.z * self_.g0_.y) + (other.g0_.w * self_.g0_.x) - (other.g1_.x * self_.g0_.w)), ((other.g0_.x * self_.g0_.z) - (other.g0_.z * self_.g0_.x) + (other.g0_.w * self_.g0_.y) - (other.g1_.y * self_.g0_.w)), (-(other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x) + (other.g0_.w * self_.g0_.z) - (other.g1_.z * self_.g0_.w)), (other.g0_.w * self_.g0_.w)), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.x * self_.g0_.w), (other.g0_.y * self_.g0_.w), (other.g0_.z * self_.g0_.w), (-(other.g0_.x * self_.g0_.x) - (other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z) - (other.g1_.w * self_.g0_.w))));
    return flector_geometricAntiProduct_flector(geometric_anti_product, point_antiReverse(self_));
}
fn point_antiSandwich_multiVector(self_: Point, other: MultiVector) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(((other.g4_.x * self_.g0_.x) + (other.g4_.y * self_.g0_.y) + (other.g4_.z * self_.g0_.z) + (other.g4_.w * self_.g0_.w)), (other.g1_.w * self_.g0_.w * -1.0)), /* e1, e2, e3, e4 */ vec4<f32>(((other.g0_.y * self_.g0_.x) - (other.g2_.y * self_.g0_.z) + (other.g2_.z * self_.g0_.y) - (other.g3_.x * self_.g0_.w)), ((other.g0_.y * self_.g0_.y) + (other.g2_.x * self_.g0_.z) - (other.g2_.z * self_.g0_.x) - (other.g3_.y * self_.g0_.w)), ((other.g0_.y * self_.g0_.z) - (other.g2_.x * self_.g0_.y) + (other.g2_.y * self_.g0_.x) - (other.g3_.z * self_.g0_.w)), (other.g0_.y * self_.g0_.w)), /* e41, e42, e43 */ (vec3<f32>(self_.g0_.w) * vec3<f32>(other.g4_.x, other.g4_.y, other.g4_.z) * vec3<f32>(-1.0)), /* e23, e31, e12 */ (vec3<f32>(((other.g4_.y * self_.g0_.z) - (other.g4_.z * self_.g0_.y)), (-(other.g4_.x * self_.g0_.z) + (other.g4_.z * self_.g0_.x)), ((other.g4_.x * self_.g0_.y) - (other.g4_.y * self_.g0_.x))) - (vec3<f32>(other.g1_.w) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z)) + (vec3<f32>(self_.g0_.w) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z))), /* e423, e431, e412, e321 */ vec4<f32>((other.g2_.x * self_.g0_.w), (other.g2_.y * self_.g0_.w), (other.g2_.z * self_.g0_.w), (-(other.g0_.x * self_.g0_.w) - (other.g2_.x * self_.g0_.x) - (other.g2_.y * self_.g0_.y) - (other.g2_.z * self_.g0_.z))));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, point_antiReverse(self_));
}
fn point_antiSandwich_origin(self_: Point, other: Origin) -> Flector {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.w * other.g0_ * -1.0)), /* e23, e31, e12, scalar */ vec4<f32>((self_.g0_.x * other.g0_ * -1.0), (self_.g0_.y * other.g0_ * -1.0), (self_.g0_.z * other.g0_ * -1.0), 0.0));
    return motor_geometricAntiProduct_motor(geometric_anti_product, point_antiReverse(self_));
}
fn point_antiSandwich_plane(self_: Point, other: Plane) -> Flector {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g0_.x * self_.g0_.w * -1.0), (other.g0_.y * self_.g0_.w * -1.0), (other.g0_.z * self_.g0_.w * -1.0), 0.0), /* e23, e31, e12, scalar */ (vec4<f32>(((other.g0_.z * self_.g0_.y) * -1.0), ((other.g0_.x * self_.g0_.z) * -1.0), ((other.g0_.y * self_.g0_.x) * -1.0), ((other.g0_.y * self_.g0_.y) + (other.g0_.z * self_.g0_.z) + (other.g0_.w * self_.g0_.w))) + (other.g0_.yzxx * self_.g0_.zxyx)));
    return motor_geometricAntiProduct_motor(geometric_anti_product, point_antiReverse(self_));
}
fn point_antiSandwich_point(self_: Point, other: Point) -> Flector {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.w * self_.g0_.w * -1.0)), /* e23, e31, e12, scalar */ vec4<f32>(((other.g0_.x * self_.g0_.w) - (other.g0_.w * self_.g0_.x)), ((other.g0_.y * self_.g0_.w) - (other.g0_.w * self_.g0_.y)), ((other.g0_.z * self_.g0_.w) - (other.g0_.w * self_.g0_.z)), 0.0));
    return motor_geometricAntiProduct_motor(geometric_anti_product, point_antiReverse(self_));
}
fn point_antiSandwich_scalar(self_: Point, other: Scalar) -> Scalar {
    let geometric_anti_product: Horizon = Horizon(/* e321 */ (self_.g0_.w * other.g0_ * -1.0));
    return horizon_geometricAntiProduct_horizon(geometric_anti_product, point_antiReverse(self_));
}
fn scalar_antiSandwich_flector(self_: Scalar, other: Flector) -> Flector {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_), 0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.w * self_.g0_)));
    return flector_geometricAntiProduct_flector(geometric_anti_product, scalar_antiReverse(self_));
}
fn scalar_antiSandwich_line(self_: Scalar, other: Line) -> Line {
    let geometric_anti_product: Line = Line(/* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ (vec3<f32>(self_.g0_) * other.g0_));
    return line_geometricAntiProduct_line(geometric_anti_product, scalar_antiReverse(self_));
}
fn scalar_antiSandwich_motor(self_: Scalar, other: Motor) -> Motor {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0), /* e23, e31, e12, scalar */ (vec4<f32>(self_.g0_) * other.g0_));
    return motor_geometricAntiProduct_motor(geometric_anti_product, scalar_antiReverse(self_));
}
fn scalar_antiSandwich_multiVector(self_: Scalar, other: MultiVector) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>((other.g0_.y * self_.g0_), 0.0), /* e1, e2, e3, e4 */ vec4<f32>((other.g4_.x * self_.g0_), (other.g4_.y * self_.g0_), (other.g4_.z * self_.g0_), 0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ (vec3<f32>(self_.g0_) * other.g2_), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g1_.w * self_.g0_)));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, scalar_antiReverse(self_));
}
fn scalar_antiSandwich_plane(self_: Scalar, other: Plane) -> Horizon {
    let geometric_anti_product: Point = Point(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), 0.0));
    return point_geometricAntiProduct_point(geometric_anti_product, scalar_antiReverse(self_));
}
fn antiScalar_bitXor_dualNum(self_: AntiScalar, other: DualNum) -> AntiScalar {
    return antiScalar_wedge_antiScalar(self_, other);
}
fn antiScalar_bitXor_motor(self_: AntiScalar, other: Motor) -> AntiScalar {
    return antiScalar_wedge_antiScalar(self_, other);
}
fn antiScalar_bitXor_multiVector(self_: AntiScalar, other: MultiVector) -> AntiScalar {
    return antiScalar_wedge_antiScalar(self_, other);
}
fn antiScalar_bitXor_scalar(self_: AntiScalar, other: Scalar) -> AntiScalar {
    return antiScalar_wedge_antiScalar(self_, other);
}
fn dualNum_bitXor_antiScalar(self_: DualNum, other: AntiScalar) -> AntiScalar {
    return dualNum_wedge_dualNum(self_, other);
}
fn dualNum_bitXor_dualNum(self_: DualNum, other: DualNum) -> DualNum {
    return dualNum_wedge_dualNum(self_, other);
}
fn dualNum_bitXor_flector(self_: DualNum, other: Flector) -> Flector {
    return dualNum_wedge_dualNum(self_, other);
}
fn dualNum_bitXor_horizon(self_: DualNum, other: Horizon) -> Horizon {
    return dualNum_wedge_dualNum(self_, other);
}
fn dualNum_bitXor_line(self_: DualNum, other: Line) -> Line {
    return dualNum_wedge_dualNum(self_, other);
}
fn dualNum_bitXor_motor(self_: DualNum, other: Motor) -> Motor {
    return dualNum_wedge_dualNum(self_, other);
}
fn dualNum_bitXor_multiVector(self_: DualNum, other: MultiVector) -> MultiVector {
    return dualNum_wedge_dualNum(self_, other);
}
fn dualNum_bitXor_origin(self_: DualNum, other: Origin) -> Origin {
    return dualNum_wedge_dualNum(self_, other);
}
fn dualNum_bitXor_plane(self_: DualNum, other: Plane) -> Plane {
    return dualNum_wedge_dualNum(self_, other);
}
fn dualNum_bitXor_point(self_: DualNum, other: Point) -> Point {
    return dualNum_wedge_dualNum(self_, other);
}
fn dualNum_bitXor_scalar(self_: DualNum, other: Scalar) -> DualNum {
    return dualNum_wedge_dualNum(self_, other);
}
fn flector_bitXor_dualNum(self_: Flector, other: DualNum) -> Flector {
    return flector_wedge_flector(self_, other);
}
fn flector_bitXor_flector(self_: Flector, other: Flector) -> Motor {
    return flector_wedge_flector(self_, other);
}
fn flector_bitXor_horizon(self_: Flector, other: Horizon) -> AntiScalar {
    return flector_wedge_flector(self_, other);
}
fn flector_bitXor_line(self_: Flector, other: Line) -> Plane {
    return flector_wedge_flector(self_, other);
}
fn flector_bitXor_motor(self_: Flector, other: Motor) -> Flector {
    return flector_wedge_flector(self_, other);
}
fn flector_bitXor_multiVector(self_: Flector, other: MultiVector) -> MultiVector {
    return flector_wedge_flector(self_, other);
}
fn flector_bitXor_origin(self_: Flector, other: Origin) -> Motor {
    return flector_wedge_flector(self_, other);
}
fn flector_bitXor_plane(self_: Flector, other: Plane) -> AntiScalar {
    return flector_wedge_flector(self_, other);
}
fn flector_bitXor_point(self_: Flector, other: Point) -> Motor {
    return flector_wedge_flector(self_, other);
}
fn flector_bitXor_scalar(self_: Flector, other: Scalar) -> Flector {
    return flector_wedge_flector(self_, other);
}
fn horizon_bitXor_dualNum(self_: Horizon, other: DualNum) -> Horizon {
    return horizon_wedge_horizon(self_, other);
}
fn horizon_bitXor_flector(self_: Horizon, other: Flector) -> AntiScalar {
    return horizon_wedge_horizon(self_, other);
}
fn horizon_bitXor_motor(self_: Horizon, other: Motor) -> Horizon {
    return horizon_wedge_horizon(self_, other);
}
fn horizon_bitXor_multiVector(self_: Horizon, other: MultiVector) -> MultiVector {
    return horizon_wedge_horizon(self_, other);
}
fn horizon_bitXor_origin(self_: Horizon, other: Origin) -> AntiScalar {
    return horizon_wedge_horizon(self_, other);
}
fn horizon_bitXor_point(self_: Horizon, other: Point) -> AntiScalar {
    return horizon_wedge_horizon(self_, other);
}
fn horizon_bitXor_scalar(self_: Horizon, other: Scalar) -> Horizon {
    return horizon_wedge_horizon(self_, other);
}
fn line_bitXor_dualNum(self_: Line, other: DualNum) -> Line {
    return line_wedge_line(self_, other);
}
fn line_bitXor_flector(self_: Line, other: Flector) -> Plane {
    return line_wedge_line(self_, other);
}
fn line_bitXor_line(self_: Line, other: Line) -> AntiScalar {
    return line_wedge_line(self_, other);
}
fn line_bitXor_motor(self_: Line, other: Motor) -> Motor {
    return line_wedge_line(self_, other);
}
fn line_bitXor_multiVector(self_: Line, other: MultiVector) -> MultiVector {
    return line_wedge_line(self_, other);
}
fn line_bitXor_origin(self_: Line, other: Origin) -> Plane {
    return line_wedge_line(self_, other);
}
fn line_bitXor_point(self_: Line, other: Point) -> Plane {
    return line_wedge_line(self_, other);
}
fn line_bitXor_scalar(self_: Line, other: Scalar) -> Line {
    return line_wedge_line(self_, other);
}
fn motor_bitXor_antiScalar(self_: Motor, other: AntiScalar) -> AntiScalar {
    return motor_wedge_motor(self_, other);
}
fn motor_bitXor_dualNum(self_: Motor, other: DualNum) -> Motor {
    return motor_wedge_motor(self_, other);
}
fn motor_bitXor_flector(self_: Motor, other: Flector) -> Flector {
    return motor_wedge_motor(self_, other);
}
fn motor_bitXor_horizon(self_: Motor, other: Horizon) -> Horizon {
    return motor_wedge_motor(self_, other);
}
fn motor_bitXor_line(self_: Motor, other: Line) -> Motor {
    return motor_wedge_motor(self_, other);
}
fn motor_bitXor_motor(self_: Motor, other: Motor) -> Motor {
    return motor_wedge_motor(self_, other);
}
fn motor_bitXor_multiVector(self_: Motor, other: MultiVector) -> MultiVector {
    return motor_wedge_motor(self_, other);
}
fn motor_bitXor_origin(self_: Motor, other: Origin) -> Flector {
    return motor_wedge_motor(self_, other);
}
fn motor_bitXor_plane(self_: Motor, other: Plane) -> Plane {
    return motor_wedge_motor(self_, other);
}
fn motor_bitXor_point(self_: Motor, other: Point) -> Flector {
    return motor_wedge_motor(self_, other);
}
fn motor_bitXor_scalar(self_: Motor, other: Scalar) -> Motor {
    return motor_wedge_motor(self_, other);
}
fn multiVector_bitXor_antiScalar(self_: MultiVector, other: AntiScalar) -> AntiScalar {
    return multiVector_wedge_multiVector(self_, other);
}
fn multiVector_bitXor_dualNum(self_: MultiVector, other: DualNum) -> MultiVector {
    return multiVector_wedge_multiVector(self_, other);
}
fn multiVector_bitXor_flector(self_: MultiVector, other: Flector) -> MultiVector {
    return multiVector_wedge_multiVector(self_, other);
}
fn multiVector_bitXor_horizon(self_: MultiVector, other: Horizon) -> MultiVector {
    return multiVector_wedge_multiVector(self_, other);
}
fn multiVector_bitXor_line(self_: MultiVector, other: Line) -> MultiVector {
    return multiVector_wedge_multiVector(self_, other);
}
fn multiVector_bitXor_motor(self_: MultiVector, other: Motor) -> MultiVector {
    return multiVector_wedge_multiVector(self_, other);
}
fn multiVector_bitXor_multiVector(self_: MultiVector, other: MultiVector) -> MultiVector {
    return multiVector_wedge_multiVector(self_, other);
}
fn multiVector_bitXor_origin(self_: MultiVector, other: Origin) -> MultiVector {
    return multiVector_wedge_multiVector(self_, other);
}
fn multiVector_bitXor_plane(self_: MultiVector, other: Plane) -> MultiVector {
    return multiVector_wedge_multiVector(self_, other);
}
fn multiVector_bitXor_point(self_: MultiVector, other: Point) -> MultiVector {
    return multiVector_wedge_multiVector(self_, other);
}
fn multiVector_bitXor_scalar(self_: MultiVector, other: Scalar) -> MultiVector {
    return multiVector_wedge_multiVector(self_, other);
}
fn origin_bitXor_dualNum(self_: Origin, other: DualNum) -> Origin {
    return origin_wedge_origin(self_, other);
}
fn origin_bitXor_flector(self_: Origin, other: Flector) -> Motor {
    return origin_wedge_origin(self_, other);
}
fn origin_bitXor_horizon(self_: Origin, other: Horizon) -> AntiScalar {
    return origin_wedge_origin(self_, other);
}
fn origin_bitXor_line(self_: Origin, other: Line) -> Plane {
    return origin_wedge_origin(self_, other);
}
fn origin_bitXor_motor(self_: Origin, other: Motor) -> Flector {
    return origin_wedge_origin(self_, other);
}
fn origin_bitXor_multiVector(self_: Origin, other: MultiVector) -> MultiVector {
    return origin_wedge_origin(self_, other);
}
fn origin_bitXor_plane(self_: Origin, other: Plane) -> AntiScalar {
    return origin_wedge_origin(self_, other);
}
fn origin_bitXor_point(self_: Origin, other: Point) -> Line {
    return origin_wedge_origin(self_, other);
}
fn origin_bitXor_scalar(self_: Origin, other: Scalar) -> Origin {
    return origin_wedge_origin(self_, other);
}
fn plane_bitXor_dualNum(self_: Plane, other: DualNum) -> Plane {
    return plane_wedge_plane(self_, other);
}
fn plane_bitXor_flector(self_: Plane, other: Flector) -> AntiScalar {
    return plane_wedge_plane(self_, other);
}
fn plane_bitXor_motor(self_: Plane, other: Motor) -> Plane {
    return plane_wedge_plane(self_, other);
}
fn plane_bitXor_multiVector(self_: Plane, other: MultiVector) -> MultiVector {
    return plane_wedge_plane(self_, other);
}
fn plane_bitXor_origin(self_: Plane, other: Origin) -> AntiScalar {
    return plane_wedge_plane(self_, other);
}
fn plane_bitXor_point(self_: Plane, other: Point) -> AntiScalar {
    return plane_wedge_plane(self_, other);
}
fn plane_bitXor_scalar(self_: Plane, other: Scalar) -> Plane {
    return plane_wedge_plane(self_, other);
}
fn point_bitXor_dualNum(self_: Point, other: DualNum) -> Point {
    return point_wedge_point(self_, other);
}
fn point_bitXor_flector(self_: Point, other: Flector) -> Motor {
    return point_wedge_point(self_, other);
}
fn point_bitXor_horizon(self_: Point, other: Horizon) -> AntiScalar {
    return point_wedge_point(self_, other);
}
fn point_bitXor_line(self_: Point, other: Line) -> Plane {
    return point_wedge_point(self_, other);
}
fn point_bitXor_motor(self_: Point, other: Motor) -> Flector {
    return point_wedge_point(self_, other);
}
fn point_bitXor_multiVector(self_: Point, other: MultiVector) -> MultiVector {
    return point_wedge_point(self_, other);
}
fn point_bitXor_origin(self_: Point, other: Origin) -> Line {
    return point_wedge_point(self_, other);
}
fn point_bitXor_plane(self_: Point, other: Plane) -> AntiScalar {
    return point_wedge_point(self_, other);
}
fn point_bitXor_point(self_: Point, other: Point) -> Line {
    return point_wedge_point(self_, other);
}
fn point_bitXor_scalar(self_: Point, other: Scalar) -> Point {
    return point_wedge_point(self_, other);
}
fn scalar_bitXor_antiScalar(self_: Scalar, other: AntiScalar) -> AntiScalar {
    return scalar_wedge_scalar(self_, other);
}
fn scalar_bitXor_dualNum(self_: Scalar, other: DualNum) -> DualNum {
    return scalar_wedge_scalar(self_, other);
}
fn scalar_bitXor_flector(self_: Scalar, other: Flector) -> Flector {
    return scalar_wedge_scalar(self_, other);
}
fn scalar_bitXor_horizon(self_: Scalar, other: Horizon) -> Horizon {
    return scalar_wedge_scalar(self_, other);
}
fn scalar_bitXor_line(self_: Scalar, other: Line) -> Line {
    return scalar_wedge_scalar(self_, other);
}
fn scalar_bitXor_motor(self_: Scalar, other: Motor) -> Motor {
    return scalar_wedge_scalar(self_, other);
}
fn scalar_bitXor_multiVector(self_: Scalar, other: MultiVector) -> MultiVector {
    return scalar_wedge_scalar(self_, other);
}
fn scalar_bitXor_origin(self_: Scalar, other: Origin) -> Origin {
    return scalar_wedge_scalar(self_, other);
}
fn scalar_bitXor_plane(self_: Scalar, other: Plane) -> Plane {
    return scalar_wedge_scalar(self_, other);
}
fn scalar_bitXor_point(self_: Scalar, other: Point) -> Point {
    return scalar_wedge_scalar(self_, other);
}
fn scalar_bitXor_scalar(self_: Scalar, other: Scalar) -> Scalar {
    return scalar_wedge_scalar(self_, other);
}
fn dualNum_not(self_: DualNum) -> AntiScalar {
    return dualNum_rightDual(self_);
}
fn flector_not(self_: Flector) -> Flector {
    return flector_rightDual(self_);
}
fn horizon_not(self_: Horizon) -> Origin {
    return horizon_rightDual(self_);
}
fn line_not(self_: Line) -> Line {
    return line_rightDual(self_);
}
fn motor_not(self_: Motor) -> Motor {
    return motor_rightDual(self_);
}
fn multiVector_not(self_: MultiVector) -> MultiVector {
    return multiVector_rightDual(self_);
}
fn plane_not(self_: Plane) -> Origin {
    return plane_rightDual(self_);
}
fn point_not(self_: Point) -> Plane {
    return point_rightDual(self_);
}
fn scalar_not(self_: Scalar) -> Scalar {
    return scalar_rightDual(self_);
}

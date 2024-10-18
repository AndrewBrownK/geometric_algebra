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
fn flector_zero() -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn horizon_grade() -> i32 {
    return 3;
}
fn plane_unit() -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>(1.0));
}
fn origin_antiGrade() -> i32 {
    return 3;
}
fn flector_unit() -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(1.0), /* e423, e431, e412, e321 */ vec4<f32>(1.0));
}
fn multiVector_unit() -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(1.0), /* e1, e2, e3, e4 */ vec4<f32>(1.0), /* e41, e42, e43 */ vec3<f32>(1.0), /* e23, e31, e12 */ vec3<f32>(1.0), /* e423, e431, e412, e321 */ vec4<f32>(1.0));
}
fn origin_grade() -> i32 {
    return 1;
}
fn scalar_one() -> Scalar {
    return Scalar(/* scalar */ 1.0);
}
fn motor_antiOne() -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, 1.0), /* e23, e31, e12, scalar */ vec4<f32>(0.0));
}
fn scalar_unit() -> Scalar {
    return Scalar(/* scalar */ 1.0);
}
fn multiVector_one() -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(1.0, 0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn motor_one() -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0), /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, 1.0));
}
fn dualNum_zero() -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>(0.0));
}
fn plane_zero() -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn origin_zero() -> Origin {
    return Origin(/* e4 */ 0.0);
}
fn antiScalar_antiOne() -> AntiScalar {
    return AntiScalar(/* e1234 */ 1.0);
}
fn multiVector_antiOne() -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, 1.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn point_grade() -> i32 {
    return 1;
}
fn antiScalar_antiGrade() -> i32 {
    return 0;
}
fn motor_zero() -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0), /* e23, e31, e12, scalar */ vec4<f32>(0.0));
}
fn point_antiGrade() -> i32 {
    return 3;
}
fn horizon_unit() -> Horizon {
    return Horizon(/* e321 */ 1.0);
}
fn line_grade() -> i32 {
    return 2;
}
fn horizon_antiGrade() -> i32 {
    return 1;
}
fn horizon_zero() -> Horizon {
    return Horizon(/* e321 */ 0.0);
}
fn line_unit() -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>(1.0), /* e23, e31, e12 */ vec3<f32>(1.0));
}
fn plane_antiGrade() -> i32 {
    return 1;
}
fn line_zero() -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0));
}
fn dualNum_unit() -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>(1.0));
}
fn dualNum_antiOne() -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>(0.0, 1.0));
}
fn point_unit() -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>(1.0));
}
fn plane_grade() -> i32 {
    return 3;
}
fn dualNum_one() -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>(1.0, 0.0));
}
fn origin_unit() -> Origin {
    return Origin(/* e4 */ 1.0);
}
fn motor_unit() -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(1.0), /* e23, e31, e12, scalar */ vec4<f32>(1.0));
}
fn line_antiGrade() -> i32 {
    return 2;
}
fn point_zero() -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>(0.0));
}
fn scalar_antiGrade() -> i32 {
    return 4;
}
fn scalar_grade() -> i32 {
    return 0;
}
fn multiVector_zero() -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn scalar_zero() -> Scalar {
    return Scalar(/* scalar */ 0.0);
}
fn antiScalar_zero() -> AntiScalar {
    return AntiScalar(/* e1234 */ 0.0);
}
fn antiScalar_grade() -> i32 {
    return 4;
}
fn antiScalar_unit() -> AntiScalar {
    return AntiScalar(/* e1234 */ 1.0);
}
fn dualNum_constraintViolation(self: DualNum) -> AntiScalar {
    let geometric_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>(pow(self_.g0_.x, 2), (self_.g0_.x * self_.g0_.y)) * vec2<f32>(1.0, 2.0)));
    let subtraction: AntiScalar = AntiScalar(/* e1234 */ geometric_product.g0_.y);
    return subtraction;
}
fn motor_neg(self: Motor) -> Motor {
    let negation: Motor = Motor(/* e41, e42, e43, e1234 */ (self_.g0_ * vec4<f32>(-1.0)), /* e23, e31, e12, scalar */ (self_.g1_ * vec4<f32>(-1.0)));
    return negation;
}
fn horizon_fix(self: Horizon) -> Horizon {
    let reverse: Horizon = Horizon(/* e321 */ (self_.g0_ * -1.0));
    let geometric_product: Scalar = Scalar(/* scalar */ (reverse.g0_ * self_.g0_ * -1.0));
    let square_root: Scalar = Scalar(/* scalar */ pow(geometric_product.g0_.x, 0.5));
    let scalar_product: Scalar = Scalar(/* scalar */ pow(square_root.g0_, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product_2: Horizon = Horizon(/* e321 */ (self_.g0_ * inverse.g0_));
    return geometric_product_2;
}
fn motor_rightDual(self: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g1_.x * -1.0), (self_.g1_.y * -1.0), (self_.g1_.z * -1.0), self_.g1_.w), /* e23, e31, e12, scalar */ vec4<f32>(0.0));
}
fn motor_inverse(self: Motor) -> Scalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(self_.g1_.x, 2) - pow(self_.g1_.y, 2) - pow(self_.g1_.z, 2) + pow(self_.g1_.w, 2)));
    return Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
}
fn plane_neg(self: Plane) -> Plane {
    let negation: Plane = Plane(/* e423, e431, e412, e321 */ (self_.g0_ * vec4<f32>(-1.0)));
    return negation;
}
fn flector_neg(self: Flector) -> Flector {
    let negation: Flector = Flector(/* e1, e2, e3, e4 */ (self_.g0_ * vec4<f32>(-1.0)), /* e423, e431, e412, e321 */ (self_.g1_ * vec4<f32>(-1.0)));
    return negation;
}
fn dualNum_rightAntiDual(self: DualNum) -> AntiScalar {
    return AntiScalar(/* e1234 */ self_.g0_.x);
}
fn flector_antiConstraintViolation(self: Flector) -> DualNum {
    let anti_reverse: Flector = Flector(/* e1, e2, e3, e4 */ (self_.g0_ * vec4<f32>(-1.0)), /* e423, e431, e412, e321 */ self_.g1_);
    let geometric_anti_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>((-(anti_reverse.g0_.x * self_.g1_.x) - (anti_reverse.g0_.y * self_.g1_.y) - (anti_reverse.g0_.z * self_.g1_.z) + (anti_reverse.g1_.w * self_.g0_.w)), 0.0) - (vec2<f32>(anti_reverse.g0_.w) * vec2<f32>(self_.g1_.w, self_.g0_.w)) + (vec2<f32>(anti_reverse.g1_.x) * vec2<f32>(self_.g0_.x, self_.g1_.x)) + (vec2<f32>(anti_reverse.g1_.y) * vec2<f32>(self_.g0_.y, self_.g1_.y)) + (vec2<f32>(anti_reverse.g1_.z) * vec2<f32>(self_.g0_.z, self_.g1_.z))));
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(self_.g0_.w, 2) + pow(self_.g1_.x, 2) + pow(self_.g1_.y, 2) + pow(self_.g1_.z, 2)));
    let subtraction: DualNum = DualNum(/* scalar, e1234 */ vec2<f32>(geometric_anti_product.g0_.x, (geometric_anti_product.g0_.y - anti_scalar_product.g0_)));
    return subtraction;
}
fn motor_antiReverse(self: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g0_.x * -1.0), (self_.g0_.y * -1.0), (self_.g0_.z * -1.0), self_.g0_.w), /* e23, e31, e12, scalar */ vec4<f32>((self_.g1_.x * -1.0), (self_.g1_.y * -1.0), (self_.g1_.z * -1.0), self_.g1_.w));
}
fn antiScalar_antiSquareRoot(self: AntiScalar) -> AntiScalar {
    return AntiScalar(/* e1234 */ pow(self_.g0_.x, 0.5));
}
fn horizon_constraintViolation(self: Horizon) -> Scalar {
    let reverse: Horizon = Horizon(/* e321 */ (self_.g0_ * -1.0));
    let geometric_product: Scalar = Scalar(/* scalar */ (reverse.g0_ * self_.g0_ * -1.0));
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(self_.g0_, 2) * -1.0));
    let subtraction: Scalar = Scalar(/* scalar */ (geometric_product.g0_ - scalar_product.g0_));
    return subtraction;
}
fn line_rightDual(self: Line) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>((self_.g1_.x * -1.0), (self_.g1_.y * -1.0), (self_.g1_.z * -1.0)), /* e23, e31, e12 */ vec3<f32>(0.0));
}
fn flector_rightAntiDual(self: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g1_.w * -1.0)), /* e423, e431, e412, e321 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, 0.0));
}
fn motor_antiInverse(self: Motor) -> AntiScalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(self_.g0_.x, 2) - pow(self_.g0_.y, 2) - pow(self_.g0_.z, 2) + pow(self_.g0_.w, 2)));
    return AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
}
fn line_inverse(self: Line) -> Scalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(self_.g1_.x, 2) - pow(self_.g1_.y, 2) - pow(self_.g1_.z, 2)));
    return Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
}
fn line_antiConstraintViolation(self: Line) -> DualNum {
    let anti_reverse: Line = Line(/* e41, e42, e43 */ (self_.g0_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (self_.g1_ * vec3<f32>(-1.0)));
    let geometric_anti_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>((-(anti_reverse.g1_.x * self_.g0_.x) - (anti_reverse.g1_.y * self_.g0_.y) - (anti_reverse.g1_.z * self_.g0_.z)), 0.0) - (vec2<f32>(anti_reverse.g0_.x) * vec2<f32>(self_.g1_.x, self_.g0_.x)) - (vec2<f32>(anti_reverse.g0_.y) * vec2<f32>(self_.g1_.y, self_.g0_.y)) - (vec2<f32>(anti_reverse.g0_.z) * vec2<f32>(self_.g1_.z, self_.g0_.z))));
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(self_.g0_.x, 2) - pow(self_.g0_.y, 2) - pow(self_.g0_.z, 2)));
    let subtraction: DualNum = DualNum(/* scalar, e1234 */ vec2<f32>(geometric_anti_product.g0_.x, (geometric_anti_product.g0_.y - anti_scalar_product.g0_)));
    return subtraction;
}
fn antiScalar_antiConstraintViolation(self: AntiScalar) -> Scalar {
    let subtraction: Scalar = Scalar(/* scalar */ 0.0);
    return subtraction;
}
fn multiVector_rightAntiDual(self: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, self_.g0_.x), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g4_.w * -1.0)), /* e41, e42, e43 */ vec3<f32>((self_.g3_.x * -1.0), (self_.g3_.y * -1.0), (self_.g3_.z * -1.0)), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, 0.0));
}
fn scalar_constraintViolation(self: Scalar) -> Scalar {
    let subtraction: Scalar = Scalar(/* scalar */ 0.0);
    return subtraction;
}
fn scalar_rightDual(self: Scalar) -> Scalar {
    return self_;
}
fn scalar_antiReverse(self: Scalar) -> Scalar {
    return self_;
}
fn plane_antiConstraintViolation(self: Plane) -> Scalar {
    let subtraction: Scalar = Scalar(/* scalar */ 0.0);
    return subtraction;
}
fn line_not(self: Line) -> Line {
    return line_rightDual(self_);
}
fn antiScalar_antiReverse(self: AntiScalar) -> AntiScalar {
    return self_;
}
fn multiVector_antiConstraintViolation(self: MultiVector) -> MultiVector {
    let anti_reverse: MultiVector = MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ (self_.g1_ * vec4<f32>(-1.0)), /* e41, e42, e43 */ (self_.g2_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (self_.g3_ * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ self_.g4_);
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(((anti_reverse.g0_.y * self_.g0_.x) - (anti_reverse.g3_.x * self_.g2_.x) - (anti_reverse.g3_.y * self_.g2_.y) - (anti_reverse.g3_.z * self_.g2_.z) - (anti_reverse.g1_.x * self_.g4_.x) - (anti_reverse.g1_.y * self_.g4_.y) - (anti_reverse.g1_.z * self_.g4_.z) + (anti_reverse.g4_.w * self_.g1_.w)), 0.0) + (vec2<f32>(self_.g0_.y) * anti_reverse.g0_) - (vec2<f32>(anti_reverse.g2_.x) * vec2<f32>(self_.g3_.x, self_.g2_.x)) - (vec2<f32>(anti_reverse.g2_.y) * vec2<f32>(self_.g3_.y, self_.g2_.y)) - (vec2<f32>(anti_reverse.g2_.z) * vec2<f32>(self_.g3_.z, self_.g2_.z)) - (vec2<f32>(anti_reverse.g1_.w) * vec2<f32>(self_.g4_.w, self_.g1_.w)) + (vec2<f32>(anti_reverse.g4_.x) * vec2<f32>(self_.g1_.x, self_.g4_.x)) + (vec2<f32>(anti_reverse.g4_.y) * vec2<f32>(self_.g1_.y, self_.g4_.y)) + (vec2<f32>(anti_reverse.g4_.z) * vec2<f32>(self_.g1_.z, self_.g4_.z))), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ (vec4<f32>(((anti_reverse.g2_.x * self_.g1_.w) - (anti_reverse.g2_.y * self_.g4_.z) + (self_.g2_.y * anti_reverse.g4_.z)), ((anti_reverse.g2_.y * self_.g1_.w) - (anti_reverse.g2_.z * self_.g4_.x) + (self_.g2_.z * anti_reverse.g4_.x)), (-(anti_reverse.g2_.x * self_.g4_.y) + (anti_reverse.g2_.z * self_.g1_.w) + (self_.g2_.x * anti_reverse.g4_.y)), (-(anti_reverse.g0_.x * self_.g1_.w) - (anti_reverse.g2_.x * self_.g1_.x) - (anti_reverse.g2_.y * self_.g1_.y) - (anti_reverse.g2_.z * self_.g1_.z) + (anti_reverse.g3_.y * self_.g4_.y) + (anti_reverse.g3_.z * self_.g4_.z) - (self_.g2_.x * anti_reverse.g1_.x) - (self_.g2_.y * anti_reverse.g1_.y) - (self_.g2_.z * anti_reverse.g1_.z) - (self_.g3_.y * anti_reverse.g4_.y) - (self_.g3_.z * anti_reverse.g4_.z))) + (vec4<f32>(anti_reverse.g0_.y) * self_.g4_) + (vec4<f32>(self_.g0_.y) * anti_reverse.g4_) + (vec4<f32>(anti_reverse.g1_.w) * vec4<f32>(self_.g2_.x, self_.g2_.y, self_.g2_.z, self_.g0_.x)) + (vec4<f32>(anti_reverse.g2_.z, anti_reverse.g2_.x, anti_reverse.g2_.y, anti_reverse.g3_.x) * self_.g4_.yzxx) - (vec4<f32>(self_.g2_.z, self_.g2_.x, self_.g2_.y, self_.g3_.x) * anti_reverse.g4_.yzxx)));
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(self_.g0_.y, 2) - pow(self_.g2_.x, 2) - pow(self_.g2_.y, 2) - pow(self_.g2_.z, 2) - pow(self_.g1_.w, 2) + pow(self_.g4_.x, 2) + pow(self_.g4_.y, 2) + pow(self_.g4_.z, 2)));
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(geometric_anti_product.g0_.x, (geometric_anti_product.g0_.y - anti_scalar_product.g0_)), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ geometric_anti_product.g4_);
    return subtraction;
}
fn dualNum_inverse(self: DualNum) -> Scalar {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(self_.g0_.x, 2));
    return Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
}
fn dualNum_antiConstraintViolation(self: DualNum) -> Scalar {
    let geometric_anti_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>((self_.g0_.x * self_.g0_.y), pow(self_.g0_.y, 2)) * vec2<f32>(2.0, 1.0)));
    let subtraction: Scalar = Scalar(/* scalar */ geometric_anti_product.g0_.x);
    return subtraction;
}
fn point_antiConstraintViolation(self: Point) -> AntiScalar {
    let anti_reverse: Point = Point(/* e1, e2, e3, e4 */ (self_.g0_ * vec4<f32>(-1.0)));
    let geometric_anti_product: AntiScalar = AntiScalar(/* e1234 */ (anti_reverse.g0_.w * self_.g0_.w * -1.0));
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(self_.g0_.w, 2) * -1.0));
    let subtraction: AntiScalar = AntiScalar(/* e1234 */ (-anti_scalar_product.g0_ + geometric_anti_product.g0_));
    return subtraction;
}
fn plane_reverse(self: Plane) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.x * -1.0), (self_.g0_.y * -1.0), (self_.g0_.z * -1.0), (self_.g0_.w * -1.0)));
}
fn multiVector_not(self: MultiVector) -> MultiVector {
    return multiVector_rightDual(self_);
}
fn dualNum_antiReverse(self: DualNum) -> DualNum {
    return self_;
}
fn plane_antiReverse(self: Plane) -> Plane {
    return self_;
}
fn horizon_antiReverse(self: Horizon) -> Horizon {
    return self_;
}
fn horizon_rightAntiDual(self: Horizon) -> Origin {
    return Origin(/* e4 */ (self_.g0_ * -1.0));
}
fn multiVector_neg(self: MultiVector) -> MultiVector {
    let negation: MultiVector = MultiVector(/* scalar, e1234 */ (self_.g0_ * vec2<f32>(-1.0)), /* e1, e2, e3, e4 */ (self_.g1_ * vec4<f32>(-1.0)), /* e41, e42, e43 */ (self_.g2_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (self_.g3_ * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ (self_.g4_ * vec4<f32>(-1.0)));
    return negation;
}
fn scalar_neg(self: Scalar) -> Scalar {
    let negation: Scalar = Scalar(/* scalar */ (self_.g0_ * -1.0));
    return negation;
}
fn line_constraintViolation(self: Line) -> DualNum {
    let reverse: Line = Line(/* e41, e42, e43 */ (self_.g0_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (self_.g1_ * vec3<f32>(-1.0)));
    let geometric_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>(0.0, (-(reverse.g1_.x * self_.g0_.x) - (reverse.g1_.y * self_.g0_.y) - (reverse.g1_.z * self_.g0_.z))) - (vec2<f32>(self_.g1_.x) * vec2<f32>(reverse.g1_.x, reverse.g0_.x)) - (vec2<f32>(self_.g1_.y) * vec2<f32>(reverse.g1_.y, reverse.g0_.y)) - (vec2<f32>(self_.g1_.z) * vec2<f32>(reverse.g1_.z, reverse.g0_.z))));
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(self_.g1_.x, 2) - pow(self_.g1_.y, 2) - pow(self_.g1_.z, 2)));
    let subtraction: DualNum = DualNum(/* scalar, e1234 */ vec2<f32>((geometric_product.g0_.x - scalar_product.g0_), geometric_product.g0_.y));
    return subtraction;
}
fn motor_constraintViolation(self: Motor) -> DualNum {
    let reverse: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g0_.x * -1.0), (self_.g0_.y * -1.0), (self_.g0_.z * -1.0), self_.g0_.w), /* e23, e31, e12, scalar */ vec4<f32>((self_.g1_.x * -1.0), (self_.g1_.y * -1.0), (self_.g1_.z * -1.0), self_.g1_.w));
    let geometric_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>(0.0, (-(reverse.g1_.x * self_.g0_.x) - (reverse.g1_.y * self_.g0_.y) - (reverse.g1_.z * self_.g0_.z) + (reverse.g1_.w * self_.g0_.w))) - (vec2<f32>(self_.g1_.x) * vec2<f32>(reverse.g1_.x, reverse.g0_.x)) - (vec2<f32>(self_.g1_.y) * vec2<f32>(reverse.g1_.y, reverse.g0_.y)) - (vec2<f32>(self_.g1_.z) * vec2<f32>(reverse.g1_.z, reverse.g0_.z)) + (vec2<f32>(self_.g1_.w) * vec2<f32>(reverse.g1_.w, reverse.g0_.w))));
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(self_.g1_.x, 2) - pow(self_.g1_.y, 2) - pow(self_.g1_.z, 2) + pow(self_.g1_.w, 2)));
    let subtraction: DualNum = DualNum(/* scalar, e1234 */ vec2<f32>((geometric_product.g0_.x - scalar_product.g0_), geometric_product.g0_.y));
    return subtraction;
}
fn point_constraintViolation(self: Point) -> Scalar {
    let subtraction: Scalar = Scalar(/* scalar */ 0.0);
    return subtraction;
}
fn motor_antiConstraintViolation(self: Motor) -> DualNum {
    let anti_reverse: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g0_.x * -1.0), (self_.g0_.y * -1.0), (self_.g0_.z * -1.0), self_.g0_.w), /* e23, e31, e12, scalar */ vec4<f32>((self_.g1_.x * -1.0), (self_.g1_.y * -1.0), (self_.g1_.z * -1.0), self_.g1_.w));
    let geometric_anti_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>((-(anti_reverse.g1_.x * self_.g0_.x) - (anti_reverse.g1_.y * self_.g0_.y) - (anti_reverse.g1_.z * self_.g0_.z) + (anti_reverse.g1_.w * self_.g0_.w)), 0.0) - (vec2<f32>(anti_reverse.g0_.x) * vec2<f32>(self_.g1_.x, self_.g0_.x)) - (vec2<f32>(anti_reverse.g0_.y) * vec2<f32>(self_.g1_.y, self_.g0_.y)) - (vec2<f32>(anti_reverse.g0_.z) * vec2<f32>(self_.g1_.z, self_.g0_.z)) + (vec2<f32>(anti_reverse.g0_.w) * vec2<f32>(self_.g1_.w, self_.g0_.w))));
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(self_.g0_.x, 2) - pow(self_.g0_.y, 2) - pow(self_.g0_.z, 2) + pow(self_.g0_.w, 2)));
    let subtraction: DualNum = DualNum(/* scalar, e1234 */ vec2<f32>(geometric_anti_product.g0_.x, (geometric_anti_product.g0_.y - anti_scalar_product.g0_)));
    return subtraction;
}
fn scalar_fix(self: Scalar) -> Scalar {
    let geometric_product: Scalar = Scalar(/* scalar */ pow(self_.g0_, 2));
    let square_root: Scalar = Scalar(/* scalar */ pow(geometric_product.g0_.x, 0.5));
    let scalar_product: Scalar = Scalar(/* scalar */ pow(square_root.g0_, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product_2: Scalar = Scalar(/* scalar */ (inverse.g0_ * self_.g0_));
    return geometric_product_2;
}
fn multiVector_antiInverse(self: MultiVector) -> AntiScalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(self_.g0_.y, 2) - pow(self_.g2_.x, 2) - pow(self_.g2_.y, 2) - pow(self_.g2_.z, 2) - pow(self_.g1_.w, 2) + pow(self_.g4_.x, 2) + pow(self_.g4_.y, 2) + pow(self_.g4_.z, 2)));
    return AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
}
fn scalar_inverse(self: Scalar) -> Scalar {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(self_.g0_, 2));
    return Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
}
fn origin_antiConstraintViolation(self: Origin) -> AntiScalar {
    let anti_reverse: Origin = Origin(/* e4 */ (self_.g0_ * -1.0));
    let geometric_anti_product: AntiScalar = AntiScalar(/* e1234 */ (anti_reverse.g0_ * self_.g0_ * -1.0));
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(self_.g0_, 2) * -1.0));
    let subtraction: AntiScalar = AntiScalar(/* e1234 */ (-anti_scalar_product.g0_ + geometric_anti_product.g0_));
    return subtraction;
}
fn antiScalar_reverse(self: AntiScalar) -> AntiScalar {
    return self_;
}
fn dualNum_not(self: DualNum) -> AntiScalar {
    return dualNum_rightDual(self_);
}
fn multiVector_reverse(self: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ self_.g1_, /* e41, e42, e43 */ vec3<f32>((self_.g2_.x * -1.0), (self_.g2_.y * -1.0), (self_.g2_.z * -1.0)), /* e23, e31, e12 */ vec3<f32>((self_.g3_.x * -1.0), (self_.g3_.y * -1.0), (self_.g3_.z * -1.0)), /* e423, e431, e412, e321 */ vec4<f32>((self_.g4_.x * -1.0), (self_.g4_.y * -1.0), (self_.g4_.z * -1.0), (self_.g4_.w * -1.0)));
}
fn origin_constraintValid(self: Origin) -> Origin {
    return self_;
}
fn multiVector_constraintViolation(self: MultiVector) -> MultiVector {
    let reverse: MultiVector = MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ self_.g1_, /* e41, e42, e43 */ (self_.g2_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (self_.g3_ * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ (self_.g4_ * vec4<f32>(-1.0)));
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(0.0, ((reverse.g0_.y * self_.g0_.x) - (reverse.g3_.x * self_.g2_.x) - (reverse.g3_.y * self_.g2_.y) - (reverse.g3_.z * self_.g2_.z) - (reverse.g1_.x * self_.g4_.x) - (reverse.g1_.y * self_.g4_.y) - (reverse.g1_.z * self_.g4_.z) + (reverse.g4_.w * self_.g1_.w))) + (vec2<f32>(reverse.g0_.x) * self_.g0_) - (vec2<f32>(self_.g3_.x) * vec2<f32>(reverse.g3_.x, reverse.g2_.x)) - (vec2<f32>(self_.g3_.y) * vec2<f32>(reverse.g3_.y, reverse.g2_.y)) - (vec2<f32>(self_.g3_.z) * vec2<f32>(reverse.g3_.z, reverse.g2_.z)) + (vec2<f32>(self_.g1_.x) * vec2<f32>(reverse.g1_.x, reverse.g4_.x)) + (vec2<f32>(self_.g1_.y) * vec2<f32>(reverse.g1_.y, reverse.g4_.y)) + (vec2<f32>(self_.g1_.z) * vec2<f32>(reverse.g1_.z, reverse.g4_.z)) - (vec2<f32>(self_.g4_.w) * vec2<f32>(reverse.g4_.w, reverse.g1_.w))), /* e1, e2, e3, e4 */ (vec4<f32>(((reverse.g3_.y * self_.g1_.z) + (self_.g3_.x * reverse.g4_.w) - (self_.g3_.y * reverse.g1_.z)), ((reverse.g3_.z * self_.g1_.x) + (self_.g3_.y * reverse.g4_.w) - (self_.g3_.z * reverse.g1_.x)), ((reverse.g3_.x * self_.g1_.y) - (self_.g3_.x * reverse.g1_.y) + (self_.g3_.z * reverse.g4_.w)), (-(self_.g0_.y * reverse.g4_.w) - (reverse.g2_.y * self_.g1_.y) - (reverse.g2_.z * self_.g1_.z) - (reverse.g3_.x * self_.g4_.x) - (reverse.g3_.y * self_.g4_.y) - (reverse.g3_.z * self_.g4_.z) + (self_.g2_.y * reverse.g1_.y) + (self_.g2_.z * reverse.g1_.z) - (self_.g3_.x * reverse.g4_.x) - (self_.g3_.y * reverse.g4_.y) - (self_.g3_.z * reverse.g4_.z))) + (vec4<f32>(reverse.g0_.x) * self_.g1_) + (vec4<f32>(self_.g0_.x) * reverse.g1_) + (vec4<f32>(self_.g4_.w) * vec4<f32>(reverse.g3_.x, reverse.g3_.y, reverse.g3_.z, reverse.g0_.y)) - (vec4<f32>(reverse.g3_.z, reverse.g3_.x, reverse.g3_.y, reverse.g2_.x) * self_.g1_.yzxx) + (vec4<f32>(self_.g3_.z, self_.g3_.x, self_.g3_.y, self_.g2_.x) * reverse.g1_.yzxx)), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(self_.g0_.x, 2) - pow(self_.g3_.x, 2) - pow(self_.g3_.y, 2) - pow(self_.g3_.z, 2) + pow(self_.g1_.x, 2) + pow(self_.g1_.y, 2) + pow(self_.g1_.z, 2) - pow(self_.g4_.w, 2)));
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>((geometric_product.g0_.x - scalar_product.g0_), geometric_product.g0_.y), /* e1, e2, e3, e4 */ geometric_product.g1_, /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    return subtraction;
}
fn scalar_not(self: Scalar) -> Scalar {
    return scalar_rightDual(self_);
}
fn antiScalar_constraintValid(self: AntiScalar) -> AntiScalar {
    return self_;
}
fn antiScalar_antiInverse(self: AntiScalar) -> AntiScalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(self_.g0_, 2));
    return AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
}
fn plane_antiFixImpl(self: Plane) -> Plane {
    let geometric_anti_product: AntiScalar = AntiScalar(/* e1234 */ (pow(self_.g0_.x, 2) + pow(self_.g0_.y, 2) + pow(self_.g0_.z, 2)));
    let anti_square_root: AntiScalar = AntiScalar(/* e1234 */ pow(geometric_anti_product.g0_.x, 0.5));
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(anti_square_root.g0_, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product_2: Plane = Plane(/* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_));
    return geometric_anti_product_2;
}
fn horizon_antiConstraintValid(self: Horizon) -> Horizon {
    return self_;
}
fn plane_rightDual(self: Plane) -> Origin {
    return Origin(/* e4 */ (self_.g0_.w * -1.0));
}
fn horizon_neg(self: Horizon) -> Horizon {
    let negation: Horizon = Horizon(/* e321 */ (self_.g0_ * -1.0));
    return negation;
}
fn dualNum_neg(self: DualNum) -> DualNum {
    let negation: DualNum = DualNum(/* scalar, e1234 */ (self_.g0_ * vec2<f32>(-1.0)));
    return negation;
}
fn line_rightAntiDual(self: Line) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>((self_.g1_.x * -1.0), (self_.g1_.y * -1.0), (self_.g1_.z * -1.0)), /* e23, e31, e12 */ vec3<f32>(0.0));
}
fn multiVector_antiReverse(self: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ vec4<f32>((self_.g1_.x * -1.0), (self_.g1_.y * -1.0), (self_.g1_.z * -1.0), (self_.g1_.w * -1.0)), /* e41, e42, e43 */ vec3<f32>((self_.g2_.x * -1.0), (self_.g2_.y * -1.0), (self_.g2_.z * -1.0)), /* e23, e31, e12 */ vec3<f32>((self_.g3_.x * -1.0), (self_.g3_.y * -1.0), (self_.g3_.z * -1.0)), /* e423, e431, e412, e321 */ self_.g4_);
}
fn point_not(self: Point) -> Plane {
    return point_rightDual(self_);
}
fn scalar_antiConstraintValid(self: Scalar) -> Scalar {
    return self_;
}
fn plane_antiInverse(self: Plane) -> AntiScalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(self_.g0_.x, 2) + pow(self_.g0_.y, 2) + pow(self_.g0_.z, 2)));
    return AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
}
fn horizon_inverse(self: Horizon) -> Scalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(self_.g0_, 2) * -1.0));
    return Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
}
fn flector_not(self: Flector) -> Flector {
    return flector_rightDual(self_);
}
fn point_rightAntiDual(self: Point) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, 0.0));
}
fn plane_constraintViolation(self: Plane) -> Scalar {
    let reverse: Plane = Plane(/* e423, e431, e412, e321 */ (self_.g0_ * vec4<f32>(-1.0)));
    let geometric_product: Scalar = Scalar(/* scalar */ (reverse.g0_.w * self_.g0_.w * -1.0));
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(self_.g0_.w, 2) * -1.0));
    let subtraction: Scalar = Scalar(/* scalar */ (geometric_product.g0_ - scalar_product.g0_));
    return subtraction;
}
fn point_antiFixImpl(self: Point) -> Point {
    let anti_reverse: Point = Point(/* e1, e2, e3, e4 */ (self_.g0_ * vec4<f32>(-1.0)));
    let geometric_anti_product: AntiScalar = AntiScalar(/* e1234 */ (anti_reverse.g0_.w * self_.g0_.w * -1.0));
    let anti_square_root: AntiScalar = AntiScalar(/* e1234 */ pow(geometric_anti_product.g0_.x, 0.5));
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(anti_square_root.g0_, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product_2: Point = Point(/* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_));
    return geometric_anti_product_2;
}
fn flector_reverse(self: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ self_.g0_, /* e423, e431, e412, e321 */ vec4<f32>((self_.g1_.x * -1.0), (self_.g1_.y * -1.0), (self_.g1_.z * -1.0), (self_.g1_.w * -1.0)));
}
fn flector_antiInverse(self: Flector) -> AntiScalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(self_.g0_.w, 2) + pow(self_.g1_.x, 2) + pow(self_.g1_.y, 2) + pow(self_.g1_.z, 2)));
    return AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
}
fn motor_not(self: Motor) -> Motor {
    return motor_rightDual(self_);
}
fn line_antiReverse(self: Line) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>((self_.g0_.x * -1.0), (self_.g0_.y * -1.0), (self_.g0_.z * -1.0)), /* e23, e31, e12 */ vec3<f32>((self_.g1_.x * -1.0), (self_.g1_.y * -1.0), (self_.g1_.z * -1.0)));
}
fn horizon_reverse(self: Horizon) -> Horizon {
    return Horizon(/* e321 */ (self_.g0_ * -1.0));
}
fn dualNum_rightDual(self: DualNum) -> AntiScalar {
    return AntiScalar(/* e1234 */ self_.g0_.x);
}
fn multiVector_inverse(self: MultiVector) -> Scalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(self_.g0_.x, 2) - pow(self_.g3_.x, 2) - pow(self_.g3_.y, 2) - pow(self_.g3_.z, 2) + pow(self_.g1_.x, 2) + pow(self_.g1_.y, 2) + pow(self_.g1_.z, 2) - pow(self_.g4_.w, 2)));
    return Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
}
fn antiScalar_neg(self: AntiScalar) -> AntiScalar {
    let negation: AntiScalar = AntiScalar(/* e1234 */ (self_.g0_ * -1.0));
    return negation;
}
fn point_neg(self: Point) -> Point {
    let negation: Point = Point(/* e1, e2, e3, e4 */ (self_.g0_ * vec4<f32>(-1.0)));
    return negation;
}
fn scalar_squareRoot(self: Scalar) -> Scalar {
    return Scalar(/* scalar */ pow(self_.g0_.x, 0.5));
}
fn antiScalar_antiFixImpl(self: AntiScalar) -> AntiScalar {
    let geometric_anti_product: AntiScalar = AntiScalar(/* e1234 */ pow(self_.g0_, 2));
    let anti_square_root: AntiScalar = AntiScalar(/* e1234 */ pow(geometric_anti_product.g0_.x, 0.5));
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(anti_square_root.g0_, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product_2: AntiScalar = AntiScalar(/* e1234 */ (anti_inverse.g0_ * self_.g0_));
    return geometric_anti_product_2;
}
fn flector_rightDual(self: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g1_.w * -1.0)), /* e423, e431, e412, e321 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, 0.0));
}
fn origin_neg(self: Origin) -> Origin {
    let negation: Origin = Origin(/* e4 */ (self_.g0_ * -1.0));
    return negation;
}
fn point_inverse(self: Point) -> Scalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(self_.g0_.x, 2) + pow(self_.g0_.y, 2) + pow(self_.g0_.z, 2)));
    return Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
}
fn origin_reverse(self: Origin) -> Origin {
    return self_;
}
fn scalar_rightAntiDual(self: Scalar) -> Scalar {
    return self_;
}
fn point_fix(self: Point) -> Point {
    let geometric_product: Scalar = Scalar(/* scalar */ (pow(self_.g0_.x, 2) + pow(self_.g0_.y, 2) + pow(self_.g0_.z, 2)));
    let square_root: Scalar = Scalar(/* scalar */ pow(geometric_product.g0_.x, 0.5));
    let scalar_product: Scalar = Scalar(/* scalar */ pow(square_root.g0_, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product_2: Point = Point(/* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g0_));
    return geometric_product_2;
}
fn dualNum_reverse(self: DualNum) -> DualNum {
    return self_;
}
fn plane_not(self: Plane) -> Origin {
    return plane_rightDual(self_);
}
fn point_rightDual(self: Point) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, 0.0));
}
fn plane_inverse(self: Plane) -> Scalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(self_.g0_.w, 2) * -1.0));
    return Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
}
fn point_antiInverse(self: Point) -> AntiScalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(self_.g0_.w, 2) * -1.0));
    return AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
}
fn flector_antiReverse(self: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * -1.0), (self_.g0_.y * -1.0), (self_.g0_.z * -1.0), (self_.g0_.w * -1.0)), /* e423, e431, e412, e321 */ self_.g1_);
}
fn motor_reverse(self: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g0_.x * -1.0), (self_.g0_.y * -1.0), (self_.g0_.z * -1.0), self_.g0_.w), /* e23, e31, e12, scalar */ vec4<f32>((self_.g1_.x * -1.0), (self_.g1_.y * -1.0), (self_.g1_.z * -1.0), self_.g1_.w));
}
fn flector_constraintViolation(self: Flector) -> DualNum {
    let reverse: Flector = Flector(/* e1, e2, e3, e4 */ self_.g0_, /* e423, e431, e412, e321 */ (self_.g1_ * vec4<f32>(-1.0)));
    let geometric_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>(0.0, (-(reverse.g0_.x * self_.g1_.x) - (reverse.g0_.y * self_.g1_.y) - (reverse.g0_.z * self_.g1_.z) + (reverse.g1_.w * self_.g0_.w))) + (vec2<f32>(self_.g0_.x) * vec2<f32>(reverse.g0_.x, reverse.g1_.x)) + (vec2<f32>(self_.g0_.y) * vec2<f32>(reverse.g0_.y, reverse.g1_.y)) + (vec2<f32>(self_.g0_.z) * vec2<f32>(reverse.g0_.z, reverse.g1_.z)) - (vec2<f32>(self_.g1_.w) * vec2<f32>(reverse.g1_.w, reverse.g0_.w))));
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(self_.g0_.x, 2) + pow(self_.g0_.y, 2) + pow(self_.g0_.z, 2) - pow(self_.g1_.w, 2)));
    let subtraction: DualNum = DualNum(/* scalar, e1234 */ vec2<f32>((geometric_product.g0_.x - scalar_product.g0_), geometric_product.g0_.y));
    return subtraction;
}
fn origin_antiInverse(self: Origin) -> AntiScalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(self_.g0_, 2) * -1.0));
    return AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
}
fn plane_fix(self: Plane) -> Plane {
    let reverse: Plane = Plane(/* e423, e431, e412, e321 */ (self_.g0_ * vec4<f32>(-1.0)));
    let geometric_product: Scalar = Scalar(/* scalar */ (reverse.g0_.w * self_.g0_.w * -1.0));
    let square_root: Scalar = Scalar(/* scalar */ pow(geometric_product.g0_.x, 0.5));
    let scalar_product: Scalar = Scalar(/* scalar */ pow(square_root.g0_, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product_2: Plane = Plane(/* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g0_));
    return geometric_product_2;
}
fn line_neg(self: Line) -> Line {
    let negation: Line = Line(/* e41, e42, e43 */ (self_.g0_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (self_.g1_ * vec3<f32>(-1.0)));
    return negation;
}
fn flector_inverse(self: Flector) -> Scalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(self_.g0_.x, 2) + pow(self_.g0_.y, 2) + pow(self_.g0_.z, 2) - pow(self_.g1_.w, 2)));
    return Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
}
fn motor_rightAntiDual(self: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g1_.x * -1.0), (self_.g1_.y * -1.0), (self_.g1_.z * -1.0), self_.g1_.w), /* e23, e31, e12, scalar */ vec4<f32>(0.0));
}
fn point_reverse(self: Point) -> Point {
    return self_;
}
fn point_antiReverse(self: Point) -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * -1.0), (self_.g0_.y * -1.0), (self_.g0_.z * -1.0), (self_.g0_.w * -1.0)));
}
fn origin_antiFixImpl(self: Origin) -> Origin {
    let anti_reverse: Origin = Origin(/* e4 */ (self_.g0_ * -1.0));
    let geometric_anti_product: AntiScalar = AntiScalar(/* e1234 */ (anti_reverse.g0_ * self_.g0_ * -1.0));
    let anti_square_root: AntiScalar = AntiScalar(/* e1234 */ pow(geometric_anti_product.g0_.x, 0.5));
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(anti_square_root.g0_, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product_2: Origin = Origin(/* e4 */ (anti_inverse.g0_ * self_.g0_));
    return geometric_anti_product_2;
}
fn line_antiInverse(self: Line) -> AntiScalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(self_.g0_.x, 2) - pow(self_.g0_.y, 2) - pow(self_.g0_.z, 2)));
    return AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
}
fn plane_rightAntiDual(self: Plane) -> Origin {
    return Origin(/* e4 */ (self_.g0_.w * -1.0));
}
fn multiVector_rightDual(self: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, self_.g0_.x), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g4_.w * -1.0)), /* e41, e42, e43 */ vec3<f32>((self_.g3_.x * -1.0), (self_.g3_.y * -1.0), (self_.g3_.z * -1.0)), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, 0.0));
}
fn horizon_rightDual(self: Horizon) -> Origin {
    return Origin(/* e4 */ (self_.g0_ * -1.0));
}
fn origin_antiReverse(self: Origin) -> Origin {
    return Origin(/* e4 */ (self_.g0_ * -1.0));
}
fn dualNum_antiInverse(self: DualNum) -> AntiScalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(self_.g0_.y, 2));
    return AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
}
fn scalar_reverse(self: Scalar) -> Scalar {
    return self_;
}
fn line_reverse(self: Line) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>((self_.g0_.x * -1.0), (self_.g0_.y * -1.0), (self_.g0_.z * -1.0)), /* e23, e31, e12 */ vec3<f32>((self_.g1_.x * -1.0), (self_.g1_.y * -1.0), (self_.g1_.z * -1.0)));
}
fn horizon_not(self: Horizon) -> Origin {
    return horizon_rightDual(self_);
}
fn multiVector_tryInto(self: MultiVector) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>(self_.g2_.x, self_.g2_.y, self_.g2_.z), /* e23, e31, e12 */ vec3<f32>(self_.g3_.x, self_.g3_.y, self_.g3_.z));
}
fn antiScalar_into(self: AntiScalar) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, self_.g0_.x), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn antiScalar_into(self: AntiScalar) -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>(0.0, self_.g0_.x));
}
fn multiVector_tryInto(self: MultiVector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g1_.w), /* e423, e431, e412, e321 */ vec4<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z, self_.g4_.w));
}
fn flector_tryInto(self: Flector) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g1_.w));
}
fn multiVector_tryInto(self: MultiVector) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(self_.g2_.x, self_.g2_.y, self_.g2_.z, self_.g0_.y), /* e23, e31, e12, scalar */ vec4<f32>(self_.g3_.x, self_.g3_.y, self_.g3_.z, self_.g0_.x));
}
fn dualNum_into(self: DualNum) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_.y), /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_.x));
}
fn point_into(self: Point) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g0_.w), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn flector_tryInto(self: Flector) -> Horizon {
    return Horizon(/* e321 */ self_.g1_.w);
}
fn flector_into(self: Flector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g0_.w), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g1_.w));
}
fn motor_tryInto(self: Motor) -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>(self_.g1_.w, self_.g0_.w));
}
fn horizon_into(self: Horizon) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_.x));
}
fn multiVector_tryInto(self: MultiVector) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z, self_.g4_.w));
}
fn motor_into(self: Motor) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g1_.w, self_.g0_.w), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z), /* e23, e31, e12 */ vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn dualNum_tryInto(self: DualNum) -> AntiScalar {
    return AntiScalar(/* e1234 */ self_.g0_.y);
}
fn multiVector_tryInto(self: MultiVector) -> Scalar {
    return Scalar(/* scalar */ self_.g0_.x);
}
fn multiVector_tryInto(self: MultiVector) -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g1_.w));
}
fn multiVector_tryInto(self: MultiVector) -> Horizon {
    return Horizon(/* e321 */ self_.g4_.w);
}
fn line_into(self: Line) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, 0.0), /* e23, e31, e12, scalar */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, 0.0));
}
fn scalar_into(self: Scalar) -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>(self_.g0_.x, 0.0));
}
fn motor_tryInto(self: Motor) -> Scalar {
    return Scalar(/* scalar */ self_.g1_.w);
}
fn multiVector_tryInto(self: MultiVector) -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>(self_.g0_.x, self_.g0_.y));
}
fn plane_into(self: Plane) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g0_.w));
}
fn dualNum_tryInto(self: DualNum) -> Scalar {
    return Scalar(/* scalar */ self_.g0_.x);
}
fn scalar_into(self: Scalar) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g0_.x, 0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn multiVector_tryInto(self: MultiVector) -> Origin {
    return Origin(/* e4 */ self_.g1_.w);
}
fn scalar_into(self: Scalar) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0), /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_.x));
}
fn plane_into(self: Plane) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g0_.w));
}
fn line_into(self: Line) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z), /* e23, e31, e12 */ vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn origin_into(self: Origin) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_.x), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn motor_tryInto(self: Motor) -> AntiScalar {
    return AntiScalar(/* e1234 */ self_.g0_.w);
}
fn dualNum_into(self: DualNum) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g0_.x, self_.g0_.y), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn antiScalar_into(self: AntiScalar) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_.x), /* e23, e31, e12, scalar */ vec4<f32>(0.0));
}
fn flector_tryInto(self: Flector) -> Origin {
    return Origin(/* e4 */ self_.g0_.w);
}
fn point_into(self: Point) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g0_.w), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn plane_tryInto(self: Plane) -> Horizon {
    return Horizon(/* e321 */ self_.g0_.w);
}
fn origin_into(self: Origin) -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_.x));
}
fn origin_into(self: Origin) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_.x), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn horizon_into(self: Horizon) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_.x));
}
fn motor_tryInto(self: Motor) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z), /* e23, e31, e12 */ vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z));
}
fn multiVector_tryInto(self: MultiVector) -> AntiScalar {
    return AntiScalar(/* e1234 */ self_.g0_.y);
}
fn flector_tryInto(self: Flector) -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g0_.w));
}
fn horizon_into(self: Horizon) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_.x));
}
fn point_tryInto(self: Point) -> Origin {
    return Origin(/* e4 */ self_.g0_.w);
}
fn horizon_geometricAntiQuotient_origin(self: Horizon, other: Origin) -> Horizon {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Horizon = Horizon(/* e321 */ (anti_inverse.g0_ * self_.g0_));
    return geometric_anti_product;
}
fn line_geometricQuotient_multiVector(self: Line, other: MultiVector) -> Line {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) - pow(other.g3_.x, 2) - pow(other.g3_.y, 2) - pow(other.g3_.z, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2) - pow(other.g4_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Line = Line(/* e41, e42, e43 */ (vec3<f32>(inverse.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(inverse.g0_) * self_.g1_));
    return geometric_product;
}
fn dualNum_add_origin(self: DualNum, other: Origin) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other.g0_), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    return addition;
}
fn motor_bitXor_motor(self: Motor, other: Motor) -> Motor {
    return motor_wedge_motor(self_, other);
}
fn motor_geometricAntiQuotient_antiScalar(self: Motor, other: AntiScalar) -> Motor {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(anti_inverse.g0_) * self_.g1_));
    return geometric_anti_product;
}
fn line_sandwich_line(self: Line, other: Line) -> Motor {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>(((other.g0_.y * self_.g1_.z) - (other.g0_.z * self_.g1_.y) + (other.g1_.y * self_.g0_.z) - (other.g1_.z * self_.g0_.y)), (-(other.g0_.x * self_.g1_.z) + (other.g0_.z * self_.g1_.x) - (other.g1_.x * self_.g0_.z) + (other.g1_.z * self_.g0_.x)), ((other.g0_.x * self_.g1_.y) - (other.g0_.y * self_.g1_.x) + (other.g1_.x * self_.g0_.y) - (other.g1_.y * self_.g0_.x)), (-(other.g0_.x * self_.g1_.x) - (other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))), /* e23, e31, e12, scalar */ vec4<f32>(((other.g1_.y * self_.g1_.z) - (other.g1_.z * self_.g1_.y)), (-(other.g1_.x * self_.g1_.z) + (other.g1_.z * self_.g1_.x)), ((other.g1_.x * self_.g1_.y) - (other.g1_.y * self_.g1_.x)), (-(other.g1_.x * self_.g1_.x) - (other.g1_.y * self_.g1_.y) - (other.g1_.z * self_.g1_.z))));
    return motor_geometricProduct_motor(geometric_product, line_reverse(self_));
}
fn scalar_wedge_scalar(self: Scalar, other: Scalar) -> Scalar {
    return Scalar(/* scalar */ (other.g0_ * self_.g0_));
}
fn point_wedge_line(self: Point, other: Line) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.y * self_.g0_.z) - (other.g0_.z * self_.g0_.y) + (other.g1_.x * self_.g0_.w)), (-(other.g0_.x * self_.g0_.z) + (other.g0_.z * self_.g0_.x) + (other.g1_.y * self_.g0_.w)), ((other.g0_.x * self_.g0_.y) - (other.g0_.y * self_.g0_.x) + (other.g1_.z * self_.g0_.w)), (-(other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))));
}
fn line_antiWedge_line(self: Line, other: Line) -> Scalar {
    return Scalar(/* scalar */ (-(other.g0_.x * self_.g1_.x) - (other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z)));
}
fn point_antiWedge_dualNum(self: Point, other: DualNum) -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.y * self_.g0_.x), (other.g0_.y * self_.g0_.y), (other.g0_.y * self_.g0_.z), (other.g0_.y * self_.g0_.w)));
}
fn scalar_geometricQuotient_motor(self: Scalar, other: Motor) -> Scalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2) + pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Scalar = Scalar(/* scalar */ (inverse.g0_ * self_.g0_));
    return geometric_product;
}
fn antiScalar_mul_flector(self: AntiScalar, other: Flector) -> Flector {
    return antiScalar_geometricProduct_antiScalar(self_, other);
}
fn origin_geometricProduct_motor(self: Origin, other: Motor) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g1_.w * self_.g0_)), /* e423, e431, e412, e321 */ vec4<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_), 0.0));
}
fn line_geometricQuotient_flector(self: Line, other: Flector) -> Line {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2) - pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Line = Line(/* e41, e42, e43 */ (vec3<f32>(inverse.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(inverse.g0_) * self_.g1_));
    return geometric_product;
}
fn origin_wedge_plane(self: Origin, other: Plane) -> AntiScalar {
    return AntiScalar(/* e1234 */ (other.g0_.w * self_.g0_));
}
fn flector_geometricAntiQuotient_multiVector(self: Flector, other: MultiVector) -> Flector {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.y, 2) - pow(other.g2_.x, 2) - pow(other.g2_.y, 2) - pow(other.g2_.z, 2) - pow(other.g1_.w, 2) + pow(other.g4_.x, 2) + pow(other.g4_.y, 2) + pow(other.g4_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g1_));
    return geometric_anti_product;
}
fn flector_bitXor_flector(self: Flector, other: Flector) -> Motor {
    return flector_wedge_flector(self_, other);
}
fn antiScalar_geometricAntiProduct_antiScalar(self: AntiScalar, other: AntiScalar) -> AntiScalar {
    return AntiScalar(/* e1234 */ (other.g0_ * self_.g0_));
}
fn scalar_sandwich_motor(self: Scalar, other: Motor) -> Motor {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(self_.g0_) * other.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(self_.g0_) * other.g1_));
    return motor_geometricProduct_motor(geometric_product, scalar_reverse(self_));
}
fn antiScalar_mul_point(self: AntiScalar, other: Point) -> Plane {
    return antiScalar_geometricProduct_antiScalar(self_, other);
}
fn dualNum_geometricProduct_line(self: DualNum, other: Line) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>(((self_.g0_.x * other.g0_.x) + (self_.g0_.y * other.g1_.x)), ((self_.g0_.x * other.g0_.y) + (self_.g0_.y * other.g1_.y)), ((self_.g0_.x * other.g0_.z) + (self_.g0_.y * other.g1_.z))), /* e23, e31, e12 */ vec3<f32>((self_.g0_.x * other.g1_.x), (self_.g0_.x * other.g1_.y), (self_.g0_.x * other.g1_.z)));
}
fn multiVector_add_plane(self: MultiVector, other: Plane) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ self_.g1_, /* e41, e42, e43 */ self_.g2_, /* e23, e31, e12 */ self_.g3_, /* e423, e431, e412, e321 */ (self_.g4_ + other.g0_));
    return addition;
}
fn flector_add_antiScalar(self: Flector, other: AntiScalar) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, other.g0_), /* e1, e2, e3, e4 */ self_.g0_, /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ self_.g1_);
    return addition;
}
fn horizon_geometricAntiProduct_line(self: Horizon, other: Line) -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), 0.0));
}
fn plane_mul_multiVector(self: Plane, other: MultiVector) -> MultiVector {
    return plane_geometricProduct_plane(self_, other);
}
fn origin_geometricProduct_scalar(self: Origin, other: Scalar) -> Origin {
    return Origin(/* e4 */ (self_.g0_ * other.g0_));
}
fn motor_mul_plane(self: Motor, other: Plane) -> Flector {
    return motor_geometricProduct_motor(self_, other);
}
fn point_add_antiScalar(self: Point, other: AntiScalar) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, other.g0_), /* e1, e2, e3, e4 */ self_.g0_, /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    return addition;
}
fn scalar_geometricAntiProduct_multiVector(self: Scalar, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((other.g0_.y * self_.g0_), 0.0), /* e1, e2, e3, e4 */ vec4<f32>((other.g4_.x * self_.g0_), (other.g4_.y * self_.g0_), (other.g4_.z * self_.g0_), 0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>((other.g2_.x * self_.g0_), (other.g2_.y * self_.g0_), (other.g2_.z * self_.g0_)), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g1_.w * self_.g0_)));
}
fn motor_bitXor_antiScalar(self: Motor, other: AntiScalar) -> AntiScalar {
    return motor_wedge_motor(self_, other);
}
fn dualNum_geometricProduct_horizon(self: DualNum, other: Horizon) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.y * other.g0_ * -1.0)), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.x * other.g0_)));
}
fn scalar_geometricAntiQuotient_plane(self: Scalar, other: Plane) -> Scalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Scalar = Scalar(/* scalar */ (anti_inverse.g0_ * self_.g0_));
    return geometric_anti_product;
}
fn point_geometricProduct_horizon(self: Point, other: Horizon) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.w * other.g0_)), /* e23, e31, e12, scalar */ vec4<f32>((self_.g0_.x * other.g0_ * -1.0), (self_.g0_.y * other.g0_ * -1.0), (self_.g0_.z * other.g0_ * -1.0), 0.0));
}
fn dualNum_geometricAntiProduct_flector(self: DualNum, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(((self_.g0_.x * other.g1_.x) + (self_.g0_.y * other.g0_.x)), ((self_.g0_.x * other.g1_.y) + (self_.g0_.y * other.g0_.y)), ((self_.g0_.x * other.g1_.z) + (self_.g0_.y * other.g0_.z)), (self_.g0_.y * other.g0_.w)), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.y * other.g1_.x), (self_.g0_.y * other.g1_.y), (self_.g0_.y * other.g1_.z), ((self_.g0_.x * other.g0_.w) + (self_.g0_.y * other.g1_.w))));
}
fn horizon_geometricQuotient_dualNum(self: Horizon, other: DualNum) -> Horizon {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_.x, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Horizon = Horizon(/* e321 */ (self_.g0_ * inverse.g0_));
    return geometric_product;
}
fn flector_wedge_origin(self: Flector, other: Origin) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g0_.x * other.g0_ * -1.0), (self_.g0_.y * other.g0_ * -1.0), (self_.g0_.z * other.g0_ * -1.0), (self_.g1_.w * other.g0_ * -1.0)), /* e23, e31, e12, scalar */ vec4<f32>(0.0));
}
fn antiScalar_geometricAntiProduct_point(self: AntiScalar, other: Point) -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), (other.g0_.w * self_.g0_)));
}
fn scalar_bitXor_antiScalar(self: Scalar, other: AntiScalar) -> AntiScalar {
    return scalar_wedge_scalar(self_, other);
}
fn antiScalar_mul_multiVector(self: AntiScalar, other: MultiVector) -> MultiVector {
    return antiScalar_geometricProduct_antiScalar(self_, other);
}
fn dualNum_geometricQuotient_point(self: DualNum, other: Point) -> DualNum {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>(inverse.g0_) * self_.g0_));
    return geometric_product;
}
fn plane_bitXor_multiVector(self: Plane, other: MultiVector) -> MultiVector {
    return plane_wedge_plane(self_, other);
}
fn flector_geometricAntiProduct_plane(self: Flector, other: Plane) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((-(self_.g0_.w * other.g0_.x) - (self_.g1_.y * other.g0_.z) + (self_.g1_.z * other.g0_.y)), (-(self_.g0_.w * other.g0_.y) + (self_.g1_.x * other.g0_.z) - (self_.g1_.z * other.g0_.x)), (-(self_.g0_.w * other.g0_.z) - (self_.g1_.x * other.g0_.y) + (self_.g1_.y * other.g0_.x)), ((self_.g1_.x * other.g0_.x) + (self_.g1_.y * other.g0_.y) + (self_.g1_.z * other.g0_.z))), /* e23, e31, e12, scalar */ vec4<f32>((-(self_.g0_.y * other.g0_.z) + (self_.g0_.z * other.g0_.y) + (self_.g1_.x * other.g0_.w) - (self_.g1_.w * other.g0_.x)), ((self_.g0_.x * other.g0_.z) - (self_.g0_.z * other.g0_.x) + (self_.g1_.y * other.g0_.w) - (self_.g1_.w * other.g0_.y)), (-(self_.g0_.x * other.g0_.y) + (self_.g0_.y * other.g0_.x) + (self_.g1_.z * other.g0_.w) - (self_.g1_.w * other.g0_.z)), ((self_.g0_.x * other.g0_.x) + (self_.g0_.y * other.g0_.y) + (self_.g0_.z * other.g0_.z) + (self_.g0_.w * other.g0_.w))));
}
fn multiVector_geometricAntiQuotient_point(self: MultiVector, other: Point) -> MultiVector {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.w, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(anti_inverse.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(anti_inverse.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(anti_inverse.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g4_));
    return geometric_anti_product;
}
fn dualNum_sub_motor(self: DualNum, other: Motor) -> Motor {
    let subtraction: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g0_.x * -1.0), (other.g0_.y * -1.0), (other.g0_.z * -1.0), (self_.g0_.y - other.g0_.w)), /* e23, e31, e12, scalar */ vec4<f32>((other.g1_.x * -1.0), (other.g1_.y * -1.0), (other.g1_.z * -1.0), (self_.g0_.x - other.g1_.w)));
    return subtraction;
}
fn flector_antiWedge_plane(self: Flector, other: Plane) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((-(self_.g1_.y * other.g0_.z) + (self_.g1_.z * other.g0_.y)), ((self_.g1_.x * other.g0_.z) - (self_.g1_.z * other.g0_.x)), (-(self_.g1_.x * other.g0_.y) + (self_.g1_.y * other.g0_.x)), 0.0), /* e23, e31, e12, scalar */ vec4<f32>(((self_.g1_.x * other.g0_.w) - (self_.g1_.w * other.g0_.x)), ((self_.g1_.y * other.g0_.w) - (self_.g1_.w * other.g0_.y)), ((self_.g1_.z * other.g0_.w) - (self_.g1_.w * other.g0_.z)), ((self_.g0_.x * other.g0_.x) + (self_.g0_.y * other.g0_.y) + (self_.g0_.z * other.g0_.z) + (self_.g0_.w * other.g0_.w))));
}
fn point_bitXor_multiVector(self: Point, other: MultiVector) -> MultiVector {
    return point_wedge_point(self_, other);
}
fn line_sandwich_scalar(self: Line, other: Scalar) -> Motor {
    let geometric_product: Line = Line(/* e41, e42, e43 */ (vec3<f32>(other.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(other.g0_) * self_.g1_));
    return line_geometricProduct_line(geometric_product, line_reverse(self_));
}
fn origin_geometricAntiProduct_point(self: Origin, other: Point) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.w * self_.g0_ * -1.0)), /* e23, e31, e12, scalar */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), 0.0));
}
fn scalar_wedge_origin(self: Scalar, other: Origin) -> Origin {
    return Origin(/* e4 */ (other.g0_ * self_.g0_));
}
fn line_wedge_line(self: Line, other: Line) -> AntiScalar {
    return AntiScalar(/* e1234 */ (-(other.g0_.x * self_.g1_.x) - (other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z)));
}
fn point_sandwich_dualNum(self: Point, other: DualNum) -> Motor {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(other.g0_.x) * self_.g0_), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.y * self_.g0_.x), (other.g0_.y * self_.g0_.y), (other.g0_.y * self_.g0_.z), 0.0));
    return flector_geometricProduct_flector(geometric_product, point_reverse(self_));
}
fn point_geometricAntiProduct_multiVector(self: Point, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(((other.g4_.x * self_.g0_.x) + (other.g4_.y * self_.g0_.y) + (other.g4_.z * self_.g0_.z) + (other.g4_.w * self_.g0_.w)), (other.g1_.w * self_.g0_.w * -1.0)), /* e1, e2, e3, e4 */ vec4<f32>(((other.g0_.y * self_.g0_.x) - (other.g2_.y * self_.g0_.z) + (other.g2_.z * self_.g0_.y) - (other.g3_.x * self_.g0_.w)), ((other.g0_.y * self_.g0_.y) + (other.g2_.x * self_.g0_.z) - (other.g2_.z * self_.g0_.x) - (other.g3_.y * self_.g0_.w)), ((other.g0_.y * self_.g0_.z) - (other.g2_.x * self_.g0_.y) + (other.g2_.y * self_.g0_.x) - (other.g3_.z * self_.g0_.w)), (other.g0_.y * self_.g0_.w)), /* e41, e42, e43 */ vec3<f32>((other.g4_.x * self_.g0_.w * -1.0), (other.g4_.y * self_.g0_.w * -1.0), (other.g4_.z * self_.g0_.w * -1.0)), /* e23, e31, e12 */ vec3<f32>(((other.g1_.x * self_.g0_.w) - (other.g1_.w * self_.g0_.x) + (other.g4_.y * self_.g0_.z) - (other.g4_.z * self_.g0_.y)), ((other.g1_.y * self_.g0_.w) - (other.g1_.w * self_.g0_.y) - (other.g4_.x * self_.g0_.z) + (other.g4_.z * self_.g0_.x)), ((other.g1_.z * self_.g0_.w) - (other.g1_.w * self_.g0_.z) + (other.g4_.x * self_.g0_.y) - (other.g4_.y * self_.g0_.x))), /* e423, e431, e412, e321 */ vec4<f32>((other.g2_.x * self_.g0_.w), (other.g2_.y * self_.g0_.w), (other.g2_.z * self_.g0_.w), (-(other.g0_.x * self_.g0_.w) - (other.g2_.x * self_.g0_.x) - (other.g2_.y * self_.g0_.y) - (other.g2_.z * self_.g0_.z))));
}
fn horizon_antiWedge_point(self: Horizon, other: Point) -> Scalar {
    return Scalar(/* scalar */ (other.g0_.w * self_.g0_ * -1.0));
}
fn point_wedge_horizon(self: Point, other: Horizon) -> AntiScalar {
    return AntiScalar(/* e1234 */ (self_.g0_.w * other.g0_));
}
fn flector_geometricAntiProduct_point(self: Flector, other: Point) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g1_.x * other.g0_.w * -1.0), (self_.g1_.y * other.g0_.w * -1.0), (self_.g1_.z * other.g0_.w * -1.0), (self_.g0_.w * other.g0_.w * -1.0)), /* e23, e31, e12, scalar */ vec4<f32>((-(self_.g0_.x * other.g0_.w) + (self_.g0_.w * other.g0_.x) + (self_.g1_.y * other.g0_.z) - (self_.g1_.z * other.g0_.y)), (-(self_.g0_.y * other.g0_.w) + (self_.g0_.w * other.g0_.y) - (self_.g1_.x * other.g0_.z) + (self_.g1_.z * other.g0_.x)), (-(self_.g0_.z * other.g0_.w) + (self_.g0_.w * other.g0_.z) + (self_.g1_.x * other.g0_.y) - (self_.g1_.y * other.g0_.x)), (-(self_.g1_.x * other.g0_.x) - (self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z) - (self_.g1_.w * other.g0_.w))));
}
fn line_antiWedge_antiScalar(self: Line, other: AntiScalar) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_)), /* e23, e31, e12 */ vec3<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_)));
}
fn line_antiSandwich_point(self: Line, other: Point) -> Flector {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>(((self_.g0_.y * other.g0_.z) - (self_.g0_.z * other.g0_.y) + (self_.g1_.x * other.g0_.w)), (-(self_.g0_.x * other.g0_.z) + (self_.g0_.z * other.g0_.x) + (self_.g1_.y * other.g0_.w)), ((self_.g0_.x * other.g0_.y) - (self_.g0_.y * other.g0_.x) + (self_.g1_.z * other.g0_.w)), 0.0), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.x * other.g0_.w), (self_.g0_.y * other.g0_.w), (self_.g0_.z * other.g0_.w), (-(self_.g0_.x * other.g0_.x) - (self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z))));
    return flector_geometricAntiProduct_flector(geometric_anti_product, line_antiReverse(self_));
}
fn antiScalar_geometricAntiProduct_flector(self: AntiScalar, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), (other.g0_.w * self_.g0_)), /* e423, e431, e412, e321 */ vec4<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_), (other.g1_.w * self_.g0_)));
}
fn origin_add_horizon(self: Origin, other: Horizon) -> Flector {
    let addition: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other.g0_));
    return addition;
}
fn dualNum_geometricProduct_scalar(self: DualNum, other: Scalar) -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_)));
}
fn horizon_mul_scalar(self: Horizon, other: Scalar) -> Horizon {
    return horizon_geometricProduct_horizon(self_, other);
}
fn dualNum_geometricProduct_point(self: DualNum, other: Point) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g0_.x), (self_.g0_.x * other.g0_.y), (self_.g0_.x * other.g0_.z), (self_.g0_.x * other.g0_.w)), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.y * other.g0_.x * -1.0), (self_.g0_.y * other.g0_.y * -1.0), (self_.g0_.y * other.g0_.z * -1.0), 0.0));
}
fn scalar_geometricProduct_antiScalar(self: Scalar, other: AntiScalar) -> AntiScalar {
    return AntiScalar(/* e1234 */ (other.g0_ * self_.g0_));
}
fn antiScalar_add_point(self: AntiScalar, other: Point) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, self_.g0_), /* e1, e2, e3, e4 */ other.g0_, /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    return addition;
}
fn dualNum_wedge_scalar(self: DualNum, other: Scalar) -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_)));
}
fn plane_antiSandwich_line(self: Plane, other: Line) -> Motor {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(((other.g0_.x * self_.g0_.w) + (other.g1_.y * self_.g0_.z)), ((other.g0_.y * self_.g0_.w) + (other.g1_.z * self_.g0_.x)), ((other.g0_.z * self_.g0_.w) + (other.g1_.x * self_.g0_.y)), (-(other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z))) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.x) * self_.g0_.yzxx)), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.y * self_.g0_.z) * -1.0), ((other.g0_.z * self_.g0_.x) * -1.0), ((other.g0_.x * self_.g0_.y) * -1.0), ((other.g1_.y * self_.g0_.y) + (other.g1_.z * self_.g0_.z))) + (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.y, other.g1_.x) * self_.g0_.yzxx)));
    return flector_geometricAntiProduct_flector(geometric_anti_product, plane_antiReverse(self_));
}
fn line_geometricAntiProduct_line(self: Line, other: Line) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((-(other.g0_.y * self_.g0_.z) + (other.g0_.z * self_.g0_.y)), ((other.g0_.x * self_.g0_.z) - (other.g0_.z * self_.g0_.x)), (-(other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x)), (-(other.g0_.x * self_.g0_.x) - (other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z))), /* e23, e31, e12, scalar */ vec4<f32>((-(other.g0_.y * self_.g1_.z) + (other.g0_.z * self_.g1_.y) - (other.g1_.y * self_.g0_.z) + (other.g1_.z * self_.g0_.y)), ((other.g0_.x * self_.g1_.z) - (other.g0_.z * self_.g1_.x) + (other.g1_.x * self_.g0_.z) - (other.g1_.z * self_.g0_.x)), (-(other.g0_.x * self_.g1_.y) + (other.g0_.y * self_.g1_.x) - (other.g1_.x * self_.g0_.y) + (other.g1_.y * self_.g0_.x)), (-(other.g0_.x * self_.g1_.x) - (other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))));
}
fn dualNum_geometricAntiProduct_dualNum(self: DualNum, other: DualNum) -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>(((other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x)), (other.g0_.y * self_.g0_.y)));
}
fn multiVector_antiSandwich_origin(self: MultiVector, other: Origin) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(other.g0_) * vec2<f32>(self_.g4_.w, self_.g1_.w) * vec2<f32>(-1.0)), /* e1, e2, e3, e4 */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g3_.x, self_.g3_.y, self_.g3_.z, self_.g0_.y)), /* e41, e42, e43 */ (vec3<f32>(other.g0_) * vec3<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z) * vec3<f32>(-1.0)), /* e23, e31, e12 */ (vec3<f32>(other.g0_) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z) * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g2_.x, self_.g2_.y, self_.g2_.z, self_.g0_.x)));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self_));
}
fn point_geometricAntiQuotient_line(self: Point, other: Line) -> Point {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Point = Point(/* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_));
    return geometric_anti_product;
}
fn horizon_sub_scalar(self: Horizon, other: Scalar) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>((other.g0_ * -1.0), 0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_));
    return subtraction;
}
fn line_geometricProduct_antiScalar(self: Line, other: AntiScalar) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_)), /* e23, e31, e12 */ vec3<f32>(0.0));
}
fn origin_bitXor_line(self: Origin, other: Line) -> Plane {
    return origin_wedge_origin(self_, other);
}
fn point_mul_origin(self: Point, other: Origin) -> Line {
    return point_geometricProduct_point(self_, other);
}
fn multiVector_mul_scalar(self: MultiVector, other: Scalar) -> MultiVector {
    return multiVector_geometricProduct_multiVector(self_, other);
}
fn line_antiSandwich_motor(self: Line, other: Motor) -> Motor {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(((self_.g0_.x * other.g0_.w) + (self_.g0_.y * other.g0_.z)), ((self_.g0_.y * other.g0_.w) + (self_.g0_.z * other.g0_.x)), ((self_.g0_.x * other.g0_.y) + (self_.g0_.z * other.g0_.w)), (-(self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z))) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g0_.x) * other.g0_.yzxx)), /* e23, e31, e12, scalar */ (vec4<f32>(((self_.g0_.x * other.g1_.w) + (self_.g0_.y * other.g1_.z) + (self_.g1_.x * other.g0_.w) + (self_.g1_.y * other.g0_.z)), ((self_.g0_.y * other.g1_.w) + (self_.g0_.z * other.g1_.x) + (self_.g1_.y * other.g0_.w) + (self_.g1_.z * other.g0_.x)), ((self_.g0_.x * other.g1_.y) + (self_.g0_.z * other.g1_.w) + (self_.g1_.x * other.g0_.y) + (self_.g1_.z * other.g0_.w)), (-(self_.g0_.y * other.g1_.y) - (self_.g0_.z * other.g1_.z) - (self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g0_.x) * other.g1_.yzxx) - (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g1_.x) * other.g0_.yzxx)));
    return motor_geometricAntiProduct_motor(geometric_anti_product, line_antiReverse(self_));
}
fn plane_antiSandwich_multiVector(self: Plane, other: MultiVector) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>((-(other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z) - (other.g1_.w * self_.g0_.w)), ((other.g4_.x * self_.g0_.x) + (other.g4_.y * self_.g0_.y) + (other.g4_.z * self_.g0_.z))), /* e1, e2, e3, e4 */ (vec4<f32>(((other.g2_.x * self_.g0_.w) + (other.g3_.y * self_.g0_.z)), ((other.g2_.y * self_.g0_.w) + (other.g3_.z * self_.g0_.x)), ((other.g2_.z * self_.g0_.w) + (other.g3_.x * self_.g0_.y)), ((other.g2_.z * self_.g0_.z) * -1.0)) - (vec4<f32>(other.g0_.x, other.g0_.x, other.g0_.x, other.g2_.x) * self_.g0_.xyzx) - (vec4<f32>(other.g3_.z, other.g3_.x, other.g3_.y, other.g2_.y) * self_.g0_.yzxy)), /* e41, e42, e43 */ (vec3<f32>(((other.g4_.y * self_.g0_.z) - (other.g4_.z * self_.g0_.y)), (-(other.g4_.x * self_.g0_.z) + (other.g4_.z * self_.g0_.x)), ((other.g4_.x * self_.g0_.y) - (other.g4_.y * self_.g0_.x))) - (vec3<f32>(other.g1_.w) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z))), /* e23, e31, e12 */ (vec3<f32>((-(other.g1_.y * self_.g0_.z) + (other.g1_.z * self_.g0_.y)), ((other.g1_.x * self_.g0_.z) - (other.g1_.z * self_.g0_.x)), (-(other.g1_.x * self_.g0_.y) + (other.g1_.y * self_.g0_.x))) + (vec3<f32>(other.g4_.w) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z)) - (vec3<f32>(self_.g0_.w) * vec3<f32>(other.g4_.x, other.g4_.y, other.g4_.z))), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g2_.y * self_.g0_.z) * -1.0), ((other.g2_.z * self_.g0_.x) * -1.0), ((other.g2_.x * self_.g0_.y) * -1.0), ((other.g3_.y * self_.g0_.y) + (other.g3_.z * self_.g0_.z))) + (vec4<f32>(other.g0_.y) * self_.g0_) + (vec4<f32>(other.g2_.z, other.g2_.x, other.g2_.y, other.g3_.x) * self_.g0_.yzxx)));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, plane_antiReverse(self_));
}
fn scalar_wedge_motor(self: Scalar, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), (other.g0_.w * self_.g0_)), /* e23, e31, e12, scalar */ vec4<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_), (other.g1_.w * self_.g0_)));
}
fn horizon_antiSandwich_flector(self: Horizon, other: Flector) -> Flector {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0), /* e23, e31, e12, scalar */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, other.g0_.w) * vec4<f32>(-1.0)));
    return motor_geometricAntiProduct_motor(geometric_anti_product, horizon_antiReverse(self_));
}
fn dualNum_sandwich_line(self: DualNum, other: Line) -> Line {
    let geometric_product: Line = Line(/* e41, e42, e43 */ ((vec3<f32>(self_.g0_.x) * other.g0_) + (vec3<f32>(self_.g0_.y) * other.g1_)), /* e23, e31, e12 */ (vec3<f32>(self_.g0_.x) * other.g1_));
    return line_geometricProduct_line(geometric_product, dualNum_reverse(self_));
}
fn antiScalar_add_line(self: AntiScalar, other: Line) -> Motor {
    let addition: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, self_.g0_), /* e23, e31, e12, scalar */ vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, 0.0));
    return addition;
}
fn motor_antiSandwich_dualNum(self: Motor, other: DualNum) -> Motor {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(other.g0_.y) * self_.g0_), /* e23, e31, e12, scalar */ ((vec4<f32>(other.g0_.x) * self_.g0_) + (vec4<f32>(other.g0_.y) * self_.g1_)));
    return motor_geometricAntiProduct_motor(geometric_anti_product, motor_antiReverse(self_));
}
fn motor_add_plane(self: Motor, other: Plane) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g1_.w, self_.g0_.w), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z), /* e23, e31, e12 */ vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z), /* e423, e431, e412, e321 */ other.g0_);
    return addition;
}
fn scalar_sandwich_line(self: Scalar, other: Line) -> Line {
    let geometric_product: Line = Line(/* e41, e42, e43 */ (vec3<f32>(self_.g0_) * other.g0_), /* e23, e31, e12 */ (vec3<f32>(self_.g0_) * other.g1_));
    return line_geometricProduct_line(geometric_product, scalar_reverse(self_));
}
fn horizon_sub_motor(self: Horizon, other: Motor) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(other.g1_.w, other.g0_.w) * vec2<f32>(-1.0)), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ (vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z) * vec3<f32>(-1.0)), /* e23, e31, e12 */ (vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z) * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_));
    return subtraction;
}
fn point_sub_motor(self: Point, other: Motor) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(other.g1_.w, other.g0_.w) * vec2<f32>(-1.0)), /* e1, e2, e3, e4 */ self_.g0_, /* e41, e42, e43 */ (vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z) * vec3<f32>(-1.0)), /* e23, e31, e12 */ (vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z) * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    return subtraction;
}
fn origin_bitXor_horizon(self: Origin, other: Horizon) -> AntiScalar {
    return origin_wedge_origin(self_, other);
}
fn scalar_geometricQuotient_horizon(self: Scalar, other: Horizon) -> Scalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Scalar = Scalar(/* scalar */ (inverse.g0_ * self_.g0_));
    return geometric_product;
}
fn motor_antiSandwich_flector(self: Motor, other: Flector) -> Flector {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(((other.g0_.z * self_.g0_.y) + (other.g0_.w * self_.g1_.x) + (other.g1_.x * self_.g1_.w) + (other.g1_.z * self_.g1_.y) + (other.g1_.w * self_.g0_.x)), ((other.g0_.y * self_.g0_.w) + (other.g0_.w * self_.g1_.y) + (other.g1_.x * self_.g1_.z) + (other.g1_.y * self_.g1_.w) + (other.g1_.w * self_.g0_.y)), ((other.g0_.z * self_.g0_.w) + (other.g0_.w * self_.g1_.z) + (other.g1_.y * self_.g1_.x) + (other.g1_.z * self_.g1_.w) + (other.g1_.w * self_.g0_.z)), ((other.g1_.z * self_.g0_.z) * -1.0)) - (vec4<f32>(other.g0_.y, other.g0_.z, other.g0_.x, other.g1_.x) * self_.g0_.zxyx) - (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g0_.y) * other.g1_.yzxy) + (other.g0_.xxyw * self_.g0_.wzxw)), /* e423, e431, e412, e321 */ (vec4<f32>((other.g1_.z * self_.g0_.y), (other.g1_.y * self_.g0_.w), (other.g1_.z * self_.g0_.w), (-(other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z) - (other.g1_.x * self_.g1_.x) - (other.g1_.y * self_.g1_.y) - (other.g1_.z * self_.g1_.z))) + (vec4<f32>(other.g0_.w) * vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g1_.w)) - (vec4<f32>(other.g1_.y, other.g1_.z, other.g1_.x, other.g0_.x) * self_.g0_.zxyx) + (other.g1_.xxyw * self_.g0_.wzxw)));
    return flector_geometricAntiProduct_flector(geometric_anti_product, motor_antiReverse(self_));
}
fn point_geometricAntiQuotient_plane(self: Point, other: Plane) -> Point {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Point = Point(/* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_));
    return geometric_anti_product;
}
fn point_geometricProduct_flector(self: Point, other: Flector) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(((other.g0_.x * self_.g0_.w) - (other.g0_.w * self_.g0_.x) - (other.g1_.y * self_.g0_.z) + (other.g1_.z * self_.g0_.y)), ((other.g0_.y * self_.g0_.w) - (other.g0_.w * self_.g0_.y) + (other.g1_.x * self_.g0_.z) - (other.g1_.z * self_.g0_.x)), ((other.g0_.z * self_.g0_.w) - (other.g0_.w * self_.g0_.z) - (other.g1_.x * self_.g0_.y) + (other.g1_.y * self_.g0_.x)), ((other.g1_.x * self_.g0_.x) + (other.g1_.y * self_.g0_.y) + (other.g1_.z * self_.g0_.z) + (other.g1_.w * self_.g0_.w))), /* e23, e31, e12, scalar */ vec4<f32>((-(other.g0_.y * self_.g0_.z) + (other.g0_.z * self_.g0_.y) - (other.g1_.w * self_.g0_.x)), ((other.g0_.x * self_.g0_.z) - (other.g0_.z * self_.g0_.x) - (other.g1_.w * self_.g0_.y)), (-(other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x) - (other.g1_.w * self_.g0_.z)), ((other.g0_.x * self_.g0_.x) + (other.g0_.y * self_.g0_.y) + (other.g0_.z * self_.g0_.z))));
}
fn multiVector_antiWedge_antiScalar(self: MultiVector, other: AntiScalar) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_)), /* e1, e2, e3, e4 */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), (self_.g1_.w * other.g0_)), /* e41, e42, e43 */ vec3<f32>((self_.g2_.x * other.g0_), (self_.g2_.y * other.g0_), (self_.g2_.z * other.g0_)), /* e23, e31, e12 */ vec3<f32>((self_.g3_.x * other.g0_), (self_.g3_.y * other.g0_), (self_.g3_.z * other.g0_)), /* e423, e431, e412, e321 */ vec4<f32>((self_.g4_.x * other.g0_), (self_.g4_.y * other.g0_), (self_.g4_.z * other.g0_), (self_.g4_.w * other.g0_)));
}
fn scalar_geometricAntiQuotient_antiScalar(self: Scalar, other: AntiScalar) -> Scalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Scalar = Scalar(/* scalar */ (anti_inverse.g0_ * self_.g0_));
    return geometric_anti_product;
}
fn antiScalar_geometricQuotient_flector(self: AntiScalar, other: Flector) -> AntiScalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2) - pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: AntiScalar = AntiScalar(/* e1234 */ (self_.g0_ * inverse.g0_));
    return geometric_product;
}
fn horizon_sub_multiVector(self: Horizon, other: MultiVector) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ (other.g0_ * vec2<f32>(-1.0)), /* e1, e2, e3, e4 */ (other.g1_ * vec4<f32>(-1.0)), /* e41, e42, e43 */ (other.g2_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (other.g3_ * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ vec4<f32>((other.g4_.x * -1.0), (other.g4_.y * -1.0), (other.g4_.z * -1.0), (-other.g4_.w + self_.g0_)));
    return subtraction;
}
fn motor_geometricProduct_flector(self: Motor, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(((other.g0_.x * self_.g1_.w) + (other.g0_.y * self_.g1_.z) - (other.g0_.z * self_.g1_.y) + (other.g1_.w * self_.g1_.x)), (-(other.g0_.x * self_.g1_.z) + (other.g0_.y * self_.g1_.w) + (other.g0_.z * self_.g1_.x) + (other.g1_.w * self_.g1_.y)), ((other.g0_.x * self_.g1_.y) - (other.g0_.y * self_.g1_.x) + (other.g0_.z * self_.g1_.w) + (other.g1_.w * self_.g1_.z)), ((other.g0_.x * self_.g0_.x) + (other.g0_.y * self_.g0_.y) + (other.g0_.z * self_.g0_.z) + (other.g0_.w * self_.g1_.w) - (other.g1_.x * self_.g1_.x) - (other.g1_.y * self_.g1_.y) - (other.g1_.z * self_.g1_.z) - (other.g1_.w * self_.g0_.w))), /* e423, e431, e412, e321 */ vec4<f32>((-(other.g0_.x * self_.g0_.w) - (other.g0_.y * self_.g0_.z) + (other.g0_.z * self_.g0_.y) + (other.g0_.w * self_.g1_.x) + (other.g1_.x * self_.g1_.w) + (other.g1_.y * self_.g1_.z) - (other.g1_.z * self_.g1_.y) - (other.g1_.w * self_.g0_.x)), ((other.g0_.x * self_.g0_.z) - (other.g0_.y * self_.g0_.w) - (other.g0_.z * self_.g0_.x) + (other.g0_.w * self_.g1_.y) - (other.g1_.x * self_.g1_.z) + (other.g1_.y * self_.g1_.w) + (other.g1_.z * self_.g1_.x) - (other.g1_.w * self_.g0_.y)), (-(other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x) - (other.g0_.z * self_.g0_.w) + (other.g0_.w * self_.g1_.z) + (other.g1_.x * self_.g1_.y) - (other.g1_.y * self_.g1_.x) + (other.g1_.z * self_.g1_.w) - (other.g1_.w * self_.g0_.z)), (-(other.g0_.x * self_.g1_.x) - (other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) + (other.g1_.w * self_.g1_.w))));
}
fn multiVector_geometricProduct_antiScalar(self: MultiVector, other: AntiScalar) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (self_.g0_.x * other.g0_)), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g4_.w * other.g0_)), /* e41, e42, e43 */ vec3<f32>((self_.g3_.x * other.g0_), (self_.g3_.y * other.g0_), (self_.g3_.z * other.g0_)), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), 0.0));
}
fn motor_geometricProduct_motor(self: Motor, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(((other.g0_.x * self_.g1_.w) + (other.g0_.y * self_.g1_.z) - (other.g0_.z * self_.g1_.y) + (other.g0_.w * self_.g1_.x) + (other.g1_.x * self_.g0_.w) + (other.g1_.y * self_.g0_.z) - (other.g1_.z * self_.g0_.y) + (other.g1_.w * self_.g0_.x)), (-(other.g0_.x * self_.g1_.z) + (other.g0_.y * self_.g1_.w) + (other.g0_.z * self_.g1_.x) + (other.g0_.w * self_.g1_.y) - (other.g1_.x * self_.g0_.z) + (other.g1_.y * self_.g0_.w) + (other.g1_.z * self_.g0_.x) + (other.g1_.w * self_.g0_.y)), ((other.g0_.x * self_.g1_.y) - (other.g0_.y * self_.g1_.x) + (other.g0_.z * self_.g1_.w) + (other.g0_.w * self_.g1_.z) + (other.g1_.x * self_.g0_.y) - (other.g1_.y * self_.g0_.x) + (other.g1_.z * self_.g0_.w) + (other.g1_.w * self_.g0_.z)), (-(other.g0_.x * self_.g1_.x) - (other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) + (other.g0_.w * self_.g1_.w) - (other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z) + (other.g1_.w * self_.g0_.w))), /* e23, e31, e12, scalar */ vec4<f32>(((other.g1_.x * self_.g1_.w) + (other.g1_.y * self_.g1_.z) - (other.g1_.z * self_.g1_.y) + (other.g1_.w * self_.g1_.x)), (-(other.g1_.x * self_.g1_.z) + (other.g1_.y * self_.g1_.w) + (other.g1_.z * self_.g1_.x) + (other.g1_.w * self_.g1_.y)), ((other.g1_.x * self_.g1_.y) - (other.g1_.y * self_.g1_.x) + (other.g1_.z * self_.g1_.w) + (other.g1_.w * self_.g1_.z)), (-(other.g1_.x * self_.g1_.x) - (other.g1_.y * self_.g1_.y) - (other.g1_.z * self_.g1_.z) + (other.g1_.w * self_.g1_.w))));
}
fn motor_sandwich_antiScalar(self: Motor, other: AntiScalar) -> Motor {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(other.g0_) * self_.g1_), /* e23, e31, e12, scalar */ vec4<f32>(0.0));
    return motor_geometricProduct_motor(geometric_product, motor_reverse(self_));
}
fn motor_geometricQuotient_plane(self: Motor, other: Plane) -> Motor {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.w, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(inverse.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(inverse.g0_) * self_.g1_));
    return geometric_product;
}
fn line_antiSandwich_line(self: Line, other: Line) -> Motor {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>((-(other.g0_.y * self_.g0_.z) + (other.g0_.z * self_.g0_.y)), ((other.g0_.x * self_.g0_.z) - (other.g0_.z * self_.g0_.x)), (-(other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x)), (-(other.g0_.x * self_.g0_.x) - (other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z))), /* e23, e31, e12, scalar */ vec4<f32>((-(other.g0_.y * self_.g1_.z) + (other.g0_.z * self_.g1_.y) - (other.g1_.y * self_.g0_.z) + (other.g1_.z * self_.g0_.y)), ((other.g0_.x * self_.g1_.z) - (other.g0_.z * self_.g1_.x) + (other.g1_.x * self_.g0_.z) - (other.g1_.z * self_.g0_.x)), (-(other.g0_.x * self_.g1_.y) + (other.g0_.y * self_.g1_.x) - (other.g1_.x * self_.g0_.y) + (other.g1_.y * self_.g0_.x)), (-(other.g0_.x * self_.g1_.x) - (other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))));
    return motor_geometricAntiProduct_motor(geometric_anti_product, line_antiReverse(self_));
}
fn point_antiSandwich_motor(self: Point, other: Motor) -> Motor {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>((-(other.g0_.y * self_.g0_.z) + (other.g0_.z * self_.g0_.y) + (other.g0_.w * self_.g0_.x) - (other.g1_.x * self_.g0_.w)), ((other.g0_.x * self_.g0_.z) - (other.g0_.z * self_.g0_.x) + (other.g0_.w * self_.g0_.y) - (other.g1_.y * self_.g0_.w)), (-(other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x) + (other.g0_.w * self_.g0_.z) - (other.g1_.z * self_.g0_.w)), (other.g0_.w * self_.g0_.w)), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.x * self_.g0_.w), (other.g0_.y * self_.g0_.w), (other.g0_.z * self_.g0_.w), (-(other.g0_.x * self_.g0_.x) - (other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z) - (other.g1_.w * self_.g0_.w))));
    return flector_geometricAntiProduct_flector(geometric_anti_product, point_antiReverse(self_));
}
fn origin_geometricAntiQuotient_plane(self: Origin, other: Plane) -> Origin {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Origin = Origin(/* e4 */ (anti_inverse.g0_ * self_.g0_));
    return geometric_anti_product;
}
fn origin_sub_multiVector(self: Origin, other: MultiVector) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ (other.g0_ * vec2<f32>(-1.0)), /* e1, e2, e3, e4 */ vec4<f32>((other.g1_.x * -1.0), (other.g1_.y * -1.0), (other.g1_.z * -1.0), (-other.g1_.w + self_.g0_)), /* e41, e42, e43 */ (other.g2_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (other.g3_ * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ (other.g4_ * vec4<f32>(-1.0)));
    return subtraction;
}
fn line_antiWedge_multiVector(self: Line, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((-(self_.g0_.x * other.g3_.x) - (self_.g0_.y * other.g3_.y) - (self_.g0_.z * other.g3_.z) - (self_.g1_.x * other.g2_.x) - (self_.g1_.y * other.g2_.y) - (self_.g1_.z * other.g2_.z)), 0.0), /* e1, e2, e3, e4 */ vec4<f32>(((self_.g0_.x * other.g4_.w) + (self_.g1_.y * other.g4_.z) - (self_.g1_.z * other.g4_.y)), ((self_.g0_.y * other.g4_.w) - (self_.g1_.x * other.g4_.z) + (self_.g1_.z * other.g4_.x)), ((self_.g0_.z * other.g4_.w) + (self_.g1_.x * other.g4_.y) - (self_.g1_.y * other.g4_.x)), (-(self_.g0_.x * other.g4_.x) - (self_.g0_.y * other.g4_.y) - (self_.g0_.z * other.g4_.z))), /* e41, e42, e43 */ vec3<f32>((other.g0_.y * self_.g0_.x), (other.g0_.y * self_.g0_.y), (other.g0_.y * self_.g0_.z)), /* e23, e31, e12 */ vec3<f32>((other.g0_.y * self_.g1_.x), (other.g0_.y * self_.g1_.y), (other.g0_.y * self_.g1_.z)), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn motor_geometricAntiProduct_antiScalar(self: Motor, other: AntiScalar) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), (self_.g0_.w * other.g0_)), /* e23, e31, e12, scalar */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), (self_.g1_.w * other.g0_)));
}
fn origin_antiWedge_horizon(self: Origin, other: Horizon) -> Scalar {
    return Scalar(/* scalar */ (other.g0_ * self_.g0_));
}
fn horizon_antiSandwich_multiVector(self: Horizon, other: MultiVector) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>((other.g1_.w * self_.g0_ * -1.0), 0.0), /* e1, e2, e3, e4 */ vec4<f32>((other.g2_.x * self_.g0_), (other.g2_.y * self_.g0_), (other.g2_.z * self_.g0_), 0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ (vec3<f32>(self_.g0_) * vec3<f32>(other.g4_.x, other.g4_.y, other.g4_.z) * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.y * self_.g0_)));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, horizon_antiReverse(self_));
}
fn origin_geometricAntiQuotient_antiScalar(self: Origin, other: AntiScalar) -> Origin {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Origin = Origin(/* e4 */ (anti_inverse.g0_ * self_.g0_));
    return geometric_anti_product;
}
fn horizon_add_point(self: Horizon, other: Point) -> Flector {
    let addition: Flector = Flector(/* e1, e2, e3, e4 */ other.g0_, /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_));
    return addition;
}
fn point_sub_scalar(self: Point, other: Scalar) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>((other.g0_ * -1.0), 0.0), /* e1, e2, e3, e4 */ self_.g0_, /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    return subtraction;
}
fn flector_antiSandwich_horizon(self: Flector, other: Horizon) -> Flector {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0), /* e23, e31, e12, scalar */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g0_.w)));
    return motor_geometricAntiProduct_motor(geometric_anti_product, flector_antiReverse(self_));
}
fn plane_sub_plane(self: Plane, other: Plane) -> Plane {
    let subtraction: Plane = Plane(/* e423, e431, e412, e321 */ (-other.g0_ + self_.g0_));
    return subtraction;
}
fn scalar_mul_antiScalar(self: Scalar, other: AntiScalar) -> AntiScalar {
    return scalar_geometricProduct_scalar(self_, other);
}
fn horizon_add_line(self: Horizon, other: Line) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ other.g0_, /* e23, e31, e12 */ other.g1_, /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_));
    return addition;
}
fn point_antiSandwich_flector(self: Point, other: Flector) -> Flector {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(self_.g0_.w) * vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, other.g0_.w) * vec4<f32>(-1.0)), /* e23, e31, e12, scalar */ (vec4<f32>((-(other.g0_.w * self_.g0_.x) - (other.g1_.z * self_.g0_.y)), (-(other.g0_.w * self_.g0_.y) - (other.g1_.x * self_.g0_.z)), (-(other.g0_.w * self_.g0_.z) - (other.g1_.y * self_.g0_.x)), ((other.g1_.z * self_.g0_.z) + (other.g1_.w * self_.g0_.w))) + (vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.x) * self_.g0_.wwwx) + (other.g1_.yzxy * self_.g0_.zxyy)));
    return motor_geometricAntiProduct_motor(geometric_anti_product, point_antiReverse(self_));
}
fn multiVector_geometricQuotient_point(self: MultiVector, other: Point) -> MultiVector {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(inverse.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(inverse.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(inverse.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g4_));
    return geometric_product;
}
fn plane_sandwich_multiVector(self: Plane, other: MultiVector) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>((other.g4_.w * self_.g0_.w * -1.0), (-(other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z) - (other.g1_.w * self_.g0_.w))), /* e1, e2, e3, e4 */ vec4<f32>((other.g3_.x * self_.g0_.w), (other.g3_.y * self_.g0_.w), (other.g3_.z * self_.g0_.w), ((other.g0_.y * self_.g0_.w) - (other.g3_.x * self_.g0_.x) - (other.g3_.y * self_.g0_.y) - (other.g3_.z * self_.g0_.z))), /* e41, e42, e43 */ (vec3<f32>(((other.g1_.y * self_.g0_.z) - (other.g1_.z * self_.g0_.y)), (-(other.g1_.x * self_.g0_.z) + (other.g1_.z * self_.g0_.x)), ((other.g1_.x * self_.g0_.y) - (other.g1_.y * self_.g0_.x))) + (vec3<f32>(other.g4_.w) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z)) - (vec3<f32>(self_.g0_.w) * vec3<f32>(other.g4_.x, other.g4_.y, other.g4_.z))), /* e23, e31, e12 */ (vec3<f32>(self_.g0_.w) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z) * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.x * self_.g0_.x) + (other.g2_.x * self_.g0_.w) + (other.g3_.y * self_.g0_.z) - (other.g3_.z * self_.g0_.y)), ((other.g0_.x * self_.g0_.y) + (other.g2_.y * self_.g0_.w) - (other.g3_.x * self_.g0_.z) + (other.g3_.z * self_.g0_.x)), ((other.g0_.x * self_.g0_.z) + (other.g2_.z * self_.g0_.w) + (other.g3_.x * self_.g0_.y) - (other.g3_.y * self_.g0_.x)), (other.g0_.x * self_.g0_.w)));
    return multiVector_geometricProduct_multiVector(geometric_product, plane_reverse(self_));
}
fn multiVector_geometricProduct_plane(self: MultiVector, other: Plane) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((self_.g4_.w * other.g0_.w * -1.0), ((self_.g1_.x * other.g0_.x) + (self_.g1_.y * other.g0_.y) + (self_.g1_.z * other.g0_.z) + (self_.g1_.w * other.g0_.w))), /* e1, e2, e3, e4 */ vec4<f32>((self_.g3_.x * other.g0_.w), (self_.g3_.y * other.g0_.w), (self_.g3_.z * other.g0_.w), (-(self_.g0_.y * other.g0_.w) - (self_.g3_.x * other.g0_.x) - (self_.g3_.y * other.g0_.y) - (self_.g3_.z * other.g0_.z))), /* e41, e42, e43 */ vec3<f32>(((self_.g1_.y * other.g0_.z) - (self_.g1_.z * other.g0_.y) + (self_.g4_.x * other.g0_.w) - (self_.g4_.w * other.g0_.x)), (-(self_.g1_.x * other.g0_.z) + (self_.g1_.z * other.g0_.x) + (self_.g4_.y * other.g0_.w) - (self_.g4_.w * other.g0_.y)), ((self_.g1_.x * other.g0_.y) - (self_.g1_.y * other.g0_.x) + (self_.g4_.z * other.g0_.w) - (self_.g4_.w * other.g0_.z))), /* e23, e31, e12 */ vec3<f32>((self_.g1_.x * other.g0_.w * -1.0), (self_.g1_.y * other.g0_.w * -1.0), (self_.g1_.z * other.g0_.w * -1.0)), /* e423, e431, e412, e321 */ vec4<f32>(((self_.g0_.x * other.g0_.x) - (self_.g2_.x * other.g0_.w) - (self_.g3_.y * other.g0_.z) + (self_.g3_.z * other.g0_.y)), ((self_.g0_.x * other.g0_.y) - (self_.g2_.y * other.g0_.w) + (self_.g3_.x * other.g0_.z) - (self_.g3_.z * other.g0_.x)), ((self_.g0_.x * other.g0_.z) - (self_.g2_.z * other.g0_.w) - (self_.g3_.x * other.g0_.y) + (self_.g3_.y * other.g0_.x)), (self_.g0_.x * other.g0_.w)));
}
fn multiVector_wedge_plane(self: MultiVector, other: Plane) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, ((self_.g1_.x * other.g0_.x) + (self_.g1_.y * other.g0_.y) + (self_.g1_.z * other.g0_.z) + (self_.g1_.w * other.g0_.w))), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.x * other.g0_.x), (self_.g0_.x * other.g0_.y), (self_.g0_.x * other.g0_.z), (self_.g0_.x * other.g0_.w)));
}
fn point_geometricProduct_point(self: Point, other: Point) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(((other.g0_.x * self_.g0_.w) - (other.g0_.w * self_.g0_.x)), ((other.g0_.y * self_.g0_.w) - (other.g0_.w * self_.g0_.y)), ((other.g0_.z * self_.g0_.w) - (other.g0_.w * self_.g0_.z)), 0.0), /* e23, e31, e12, scalar */ vec4<f32>((-(other.g0_.y * self_.g0_.z) + (other.g0_.z * self_.g0_.y)), ((other.g0_.x * self_.g0_.z) - (other.g0_.z * self_.g0_.x)), (-(other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x)), ((other.g0_.x * self_.g0_.x) + (other.g0_.y * self_.g0_.y) + (other.g0_.z * self_.g0_.z))));
}
fn motor_bitXor_horizon(self: Motor, other: Horizon) -> Horizon {
    return motor_wedge_motor(self_, other);
}
fn origin_mul_dualNum(self: Origin, other: DualNum) -> Origin {
    return origin_geometricProduct_origin(self_, other);
}
fn multiVector_geometricAntiProduct_antiScalar(self: MultiVector, other: AntiScalar) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_)), /* e1, e2, e3, e4 */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), (self_.g1_.w * other.g0_)), /* e41, e42, e43 */ vec3<f32>((self_.g2_.x * other.g0_), (self_.g2_.y * other.g0_), (self_.g2_.z * other.g0_)), /* e23, e31, e12 */ vec3<f32>((self_.g3_.x * other.g0_), (self_.g3_.y * other.g0_), (self_.g3_.z * other.g0_)), /* e423, e431, e412, e321 */ vec4<f32>((self_.g4_.x * other.g0_), (self_.g4_.y * other.g0_), (self_.g4_.z * other.g0_), (self_.g4_.w * other.g0_)));
}
fn origin_wedge_horizon(self: Origin, other: Horizon) -> AntiScalar {
    return AntiScalar(/* e1234 */ (other.g0_ * self_.g0_));
}
fn multiVector_sandwich_flector(self: MultiVector, other: Flector) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(0.0, (-(other.g0_.x * self_.g4_.x) - (other.g0_.y * self_.g4_.y) - (other.g0_.z * self_.g4_.z) + (other.g1_.w * self_.g1_.w))) + (vec2<f32>(self_.g1_.x) * vec2<f32>(other.g0_.x, other.g1_.x)) + (vec2<f32>(self_.g1_.y) * vec2<f32>(other.g0_.y, other.g1_.y)) + (vec2<f32>(self_.g1_.z) * vec2<f32>(other.g0_.z, other.g1_.z)) - (vec2<f32>(self_.g4_.w) * vec2<f32>(other.g1_.w, other.g0_.w))), /* e1, e2, e3, e4 */ (vec4<f32>(((self_.g3_.x * other.g1_.w) - (self_.g3_.y * other.g0_.z)), ((self_.g3_.y * other.g1_.w) - (self_.g3_.z * other.g0_.x)), (-(self_.g3_.x * other.g0_.y) + (self_.g3_.z * other.g1_.w)), (-(self_.g0_.y * other.g1_.w) + (self_.g2_.y * other.g0_.y) + (self_.g2_.z * other.g0_.z) - (self_.g3_.x * other.g1_.x) - (self_.g3_.y * other.g1_.y) - (self_.g3_.z * other.g1_.z))) + (vec4<f32>(self_.g0_.x) * other.g0_) + (vec4<f32>(self_.g3_.z, self_.g3_.x, self_.g3_.y, self_.g2_.x) * other.g0_.yzxx)), /* e41, e42, e43 */ (vec3<f32>(((other.g0_.y * self_.g4_.z) - (other.g0_.z * self_.g4_.y) - (other.g1_.y * self_.g1_.z) + (other.g1_.z * self_.g1_.y)), (-(other.g0_.x * self_.g4_.z) + (other.g0_.z * self_.g4_.x) + (other.g1_.x * self_.g1_.z) - (other.g1_.z * self_.g1_.x)), ((other.g0_.x * self_.g4_.y) - (other.g0_.y * self_.g4_.x) - (other.g1_.x * self_.g1_.y) + (other.g1_.y * self_.g1_.x))) - (vec3<f32>(other.g0_.w) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z)) + (vec3<f32>(other.g1_.w) * vec3<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z)) + (vec3<f32>(self_.g1_.w) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z)) - (vec3<f32>(self_.g4_.w) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z))), /* e23, e31, e12 */ (vec3<f32>((-(other.g0_.y * self_.g1_.z) + (other.g0_.z * self_.g1_.y)), ((other.g0_.x * self_.g1_.z) - (other.g0_.z * self_.g1_.x)), (-(other.g0_.x * self_.g1_.y) + (other.g0_.y * self_.g1_.x))) - (vec3<f32>(other.g1_.w) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z)) - (vec3<f32>(self_.g4_.w) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z))), /* e423, e431, e412, e321 */ (vec4<f32>((-(self_.g2_.x * other.g1_.w) + (self_.g2_.y * other.g0_.z) + (self_.g3_.x * other.g0_.w) - (self_.g3_.y * other.g1_.z) + (self_.g3_.z * other.g1_.y)), (-(self_.g2_.y * other.g1_.w) + (self_.g2_.z * other.g0_.x) + (self_.g3_.x * other.g1_.z) + (self_.g3_.y * other.g0_.w) - (self_.g3_.z * other.g1_.x)), ((self_.g2_.x * other.g0_.y) - (self_.g2_.z * other.g1_.w) - (self_.g3_.x * other.g1_.y) + (self_.g3_.y * other.g1_.x) + (self_.g3_.z * other.g0_.w)), ((self_.g3_.z * other.g0_.z) * -1.0)) + (vec4<f32>(self_.g0_.x) * other.g1_) - (vec4<f32>(self_.g0_.y, self_.g0_.y, self_.g0_.y, self_.g3_.x) * other.g0_.xyzx) - (vec4<f32>(self_.g2_.z, self_.g2_.x, self_.g2_.y, self_.g3_.y) * other.g0_.yzxy)));
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self_));
}
fn horizon_geometricQuotient_multiVector(self: Horizon, other: MultiVector) -> Horizon {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) - pow(other.g3_.x, 2) - pow(other.g3_.y, 2) - pow(other.g3_.z, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2) - pow(other.g4_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Horizon = Horizon(/* e321 */ (self_.g0_ * inverse.g0_));
    return geometric_product;
}
fn dualNum_bitXor_dualNum(self: DualNum, other: DualNum) -> DualNum {
    return dualNum_wedge_dualNum(self_, other);
}
fn multiVector_sandwich_plane(self: MultiVector, other: Plane) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>((self_.g4_.w * other.g0_.w * -1.0), ((self_.g1_.x * other.g0_.x) + (self_.g1_.y * other.g0_.y) + (self_.g1_.z * other.g0_.z) + (self_.g1_.w * other.g0_.w))), /* e1, e2, e3, e4 */ vec4<f32>((self_.g3_.x * other.g0_.w), (self_.g3_.y * other.g0_.w), (self_.g3_.z * other.g0_.w), (-(self_.g0_.y * other.g0_.w) - (self_.g3_.x * other.g0_.x) - (self_.g3_.y * other.g0_.y) - (self_.g3_.z * other.g0_.z))), /* e41, e42, e43 */ (vec3<f32>(((self_.g1_.y * other.g0_.z) - (self_.g1_.z * other.g0_.y)), (-(self_.g1_.x * other.g0_.z) + (self_.g1_.z * other.g0_.x)), ((self_.g1_.x * other.g0_.y) - (self_.g1_.y * other.g0_.x))) - (vec3<f32>(self_.g4_.w) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z)) + (vec3<f32>(other.g0_.w) * vec3<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z))), /* e23, e31, e12 */ (vec3<f32>(other.g0_.w) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z) * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ vec4<f32>(((self_.g0_.x * other.g0_.x) - (self_.g2_.x * other.g0_.w) - (self_.g3_.y * other.g0_.z) + (self_.g3_.z * other.g0_.y)), ((self_.g0_.x * other.g0_.y) - (self_.g2_.y * other.g0_.w) + (self_.g3_.x * other.g0_.z) - (self_.g3_.z * other.g0_.x)), ((self_.g0_.x * other.g0_.z) - (self_.g2_.z * other.g0_.w) - (self_.g3_.x * other.g0_.y) + (self_.g3_.y * other.g0_.x)), (self_.g0_.x * other.g0_.w)));
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self_));
}
fn line_wedge_multiVector(self: Line, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (-(self_.g0_.x * other.g3_.x) - (self_.g0_.y * other.g3_.y) - (self_.g0_.z * other.g3_.z) - (self_.g1_.x * other.g2_.x) - (self_.g1_.y * other.g2_.y) - (self_.g1_.z * other.g2_.z))), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>((other.g0_.x * self_.g0_.x), (other.g0_.x * self_.g0_.y), (other.g0_.x * self_.g0_.z)), /* e23, e31, e12 */ vec3<f32>((other.g0_.x * self_.g1_.x), (other.g0_.x * self_.g1_.y), (other.g0_.x * self_.g1_.z)), /* e423, e431, e412, e321 */ vec4<f32>(((self_.g0_.y * other.g1_.z) - (self_.g0_.z * other.g1_.y) + (self_.g1_.x * other.g1_.w)), (-(self_.g0_.x * other.g1_.z) + (self_.g0_.z * other.g1_.x) + (self_.g1_.y * other.g1_.w)), ((self_.g0_.x * other.g1_.y) - (self_.g0_.y * other.g1_.x) + (self_.g1_.z * other.g1_.w)), (-(self_.g1_.x * other.g1_.x) - (self_.g1_.y * other.g1_.y) - (self_.g1_.z * other.g1_.z))));
}
fn scalar_wedge_point(self: Scalar, other: Point) -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), (other.g0_.w * self_.g0_)));
}
fn horizon_sandwich_dualNum(self: Horizon, other: DualNum) -> Motor {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.y * self_.g0_)), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.x * self_.g0_)));
    return flector_geometricProduct_flector(geometric_product, horizon_reverse(self_));
}
fn plane_geometricProduct_origin(self: Plane, other: Origin) -> AntiScalar {
    return AntiScalar(/* e1234 */ (self_.g0_.w * other.g0_ * -1.0));
}
fn scalar_mul_multiVector(self: Scalar, other: MultiVector) -> MultiVector {
    return scalar_geometricProduct_scalar(self_, other);
}
fn scalar_sandwich_origin(self: Scalar, other: Origin) -> Origin {
    let geometric_product: Origin = Origin(/* e4 */ (other.g0_ * self_.g0_));
    return origin_geometricProduct_origin(geometric_product, scalar_reverse(self_));
}
fn scalar_geometricAntiQuotient_dualNum(self: Scalar, other: DualNum) -> Scalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_.y, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Scalar = Scalar(/* scalar */ (anti_inverse.g0_ * self_.g0_));
    return geometric_anti_product;
}
fn flector_add_flector(self: Flector, other: Flector) -> Flector {
    let addition: Flector = Flector(/* e1, e2, e3, e4 */ (other.g0_ + self_.g0_), /* e423, e431, e412, e321 */ (other.g1_ + self_.g1_));
    return addition;
}
fn dualNum_geometricQuotient_dualNum(self: DualNum, other: DualNum) -> DualNum {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_.x, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>(inverse.g0_) * self_.g0_));
    return geometric_product;
}
fn motor_geometricQuotient_line(self: Motor, other: Line) -> Motor {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(inverse.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(inverse.g0_) * self_.g1_));
    return geometric_product;
}
fn origin_sub_plane(self: Origin, other: Plane) -> Flector {
    let subtraction: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_), /* e423, e431, e412, e321 */ (other.g0_ * vec4<f32>(-1.0)));
    return subtraction;
}
fn antiScalar_add_horizon(self: AntiScalar, other: Horizon) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, self_.g0_), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other.g0_));
    return addition;
}
fn motor_antiSandwich_motor(self: Motor, other: Motor) -> Motor {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(((other.g0_.z * self_.g0_.y) + (other.g0_.w * self_.g0_.x)), ((other.g0_.y * self_.g0_.w) + (other.g0_.w * self_.g0_.y)), ((other.g0_.z * self_.g0_.w) + (other.g0_.w * self_.g0_.z)), (-(other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z))) + (other.g0_.xxyw * self_.g0_.wzxw) - (other.g0_.yzxx * self_.g0_.zxyx)), /* e23, e31, e12, scalar */ (vec4<f32>(((other.g0_.z * self_.g1_.y) + (other.g0_.w * self_.g1_.x) + (other.g1_.z * self_.g0_.y) + (other.g1_.w * self_.g0_.x)), ((other.g0_.y * self_.g1_.w) + (other.g0_.w * self_.g1_.y) + (other.g1_.y * self_.g0_.w) + (other.g1_.w * self_.g0_.y)), ((other.g0_.z * self_.g1_.w) + (other.g0_.w * self_.g1_.z) + (other.g1_.z * self_.g0_.w) + (other.g1_.w * self_.g0_.z)), (-(other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))) + (other.g0_.xxyw * self_.g1_.wzxw) - (other.g0_.yzxx * self_.g1_.zxyx) + (other.g1_.xxyw * self_.g0_.wzxw) - (other.g1_.yzxx * self_.g0_.zxyx)));
    return motor_geometricAntiProduct_motor(geometric_anti_product, motor_antiReverse(self_));
}
fn dualNum_sub_multiVector(self: DualNum, other: MultiVector) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ (self_.g0_ - other.g0_), /* e1, e2, e3, e4 */ (other.g1_ * vec4<f32>(-1.0)), /* e41, e42, e43 */ (other.g2_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (other.g3_ * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ (other.g4_ * vec4<f32>(-1.0)));
    return subtraction;
}
fn antiScalar_sandwich_motor(self: AntiScalar, other: Motor) -> Motor {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(self_.g0_) * other.g1_), /* e23, e31, e12, scalar */ vec4<f32>(0.0));
    return motor_geometricProduct_motor(geometric_product, antiScalar_reverse(self_));
}
fn point_bitXor_horizon(self: Point, other: Horizon) -> AntiScalar {
    return point_wedge_point(self_, other);
}
fn line_mul_plane(self: Line, other: Plane) -> Flector {
    return line_geometricProduct_line(self_, other);
}
fn motor_mul_horizon(self: Motor, other: Horizon) -> Flector {
    return motor_geometricProduct_motor(self_, other);
}
fn point_bitXor_line(self: Point, other: Line) -> Plane {
    return point_wedge_point(self_, other);
}
fn line_mul_motor(self: Line, other: Motor) -> Motor {
    return line_geometricProduct_line(self_, other);
}
fn flector_antiSandwich_scalar(self: Flector, other: Scalar) -> Motor {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g1_.x * other.g0_ * -1.0), (self_.g1_.y * other.g0_ * -1.0), (self_.g1_.z * other.g0_ * -1.0), 0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.w * other.g0_ * -1.0)));
    return flector_geometricAntiProduct_flector(geometric_anti_product, flector_antiReverse(self_));
}
fn horizon_add_plane(self: Horizon, other: Plane) -> Plane {
    let addition: Plane = Plane(/* e423, e431, e412, e321 */ vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, (other.g0_.w + self_.g0_)));
    return addition;
}
fn dualNum_wedge_origin(self: DualNum, other: Origin) -> Origin {
    return Origin(/* e4 */ (self_.g0_.x * other.g0_));
}
fn flector_antiSandwich_origin(self: Flector, other: Origin) -> Flector {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g0_.w) * vec4<f32>(-1.0)), /* e23, e31, e12, scalar */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g1_.w) * vec4<f32>(-1.0)));
    return motor_geometricAntiProduct_motor(geometric_anti_product, flector_antiReverse(self_));
}
fn dualNum_bitXor_flector(self: DualNum, other: Flector) -> Flector {
    return dualNum_wedge_dualNum(self_, other);
}
fn point_mul_antiScalar(self: Point, other: AntiScalar) -> Plane {
    return point_geometricProduct_point(self_, other);
}
fn dualNum_bitXor_scalar(self: DualNum, other: Scalar) -> DualNum {
    return dualNum_wedge_dualNum(self_, other);
}
fn plane_sub_motor(self: Plane, other: Motor) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(other.g1_.w, other.g0_.w) * vec2<f32>(-1.0)), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ (vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z) * vec3<f32>(-1.0)), /* e23, e31, e12 */ (vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z) * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ self_.g0_);
    return subtraction;
}
fn point_mul_motor(self: Point, other: Motor) -> Flector {
    return point_geometricProduct_point(self_, other);
}
fn scalar_sandwich_antiScalar(self: Scalar, other: AntiScalar) -> AntiScalar {
    let geometric_product: AntiScalar = AntiScalar(/* e1234 */ (other.g0_ * self_.g0_));
    return antiScalar_geometricProduct_antiScalar(geometric_product, scalar_reverse(self_));
}
fn scalar_sandwich_horizon(self: Scalar, other: Horizon) -> Horizon {
    let geometric_product: Horizon = Horizon(/* e321 */ (other.g0_ * self_.g0_));
    return horizon_geometricProduct_horizon(geometric_product, scalar_reverse(self_));
}
fn scalar_geometricProduct_multiVector(self: Scalar, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_)), /* e1, e2, e3, e4 */ vec4<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_), (other.g1_.w * self_.g0_)), /* e41, e42, e43 */ vec3<f32>((other.g2_.x * self_.g0_), (other.g2_.y * self_.g0_), (other.g2_.z * self_.g0_)), /* e23, e31, e12 */ vec3<f32>((other.g3_.x * self_.g0_), (other.g3_.y * self_.g0_), (other.g3_.z * self_.g0_)), /* e423, e431, e412, e321 */ vec4<f32>((other.g4_.x * self_.g0_), (other.g4_.y * self_.g0_), (other.g4_.z * self_.g0_), (other.g4_.w * self_.g0_)));
}
fn motor_sandwich_horizon(self: Motor, other: Horizon) -> Flector {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, -1.0)), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g1_.w) * vec4<f32>(-1.0, -1.0, -1.0, 1.0)));
    return flector_geometricProduct_flector(geometric_product, motor_reverse(self_));
}
fn scalar_add_motor(self: Scalar, other: Motor) -> Motor {
    let addition: Motor = Motor(/* e41, e42, e43, e1234 */ other.g0_, /* e23, e31, e12, scalar */ vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, (other.g1_.w + self_.g0_)));
    return addition;
}
fn multiVector_antiWedge_scalar(self: MultiVector, other: Scalar) -> Scalar {
    return Scalar(/* scalar */ (self_.g0_.y * other.g0_));
}
fn dualNum_bitXor_origin(self: DualNum, other: Origin) -> Origin {
    return dualNum_wedge_dualNum(self_, other);
}
fn point_sandwich_origin(self: Point, other: Origin) -> Flector {
    let geometric_product: Line = Line(/* e41, e42, e43 */ (vec3<f32>(other.g0_) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z) * vec3<f32>(-1.0)), /* e23, e31, e12 */ vec3<f32>(0.0));
    return line_geometricProduct_line(geometric_product, point_reverse(self_));
}
fn plane_sandwich_dualNum(self: Plane, other: DualNum) -> Motor {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.y * self_.g0_.w)), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_.x) * self_.g0_));
    return flector_geometricProduct_flector(geometric_product, plane_reverse(self_));
}
fn dualNum_mul_line(self: DualNum, other: Line) -> Line {
    return dualNum_geometricProduct_dualNum(self_, other);
}
fn origin_wedge_motor(self: Origin, other: Motor) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g1_.w * self_.g0_)), /* e423, e431, e412, e321 */ vec4<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_), 0.0));
}
fn horizon_geometricAntiProduct_plane(self: Horizon, other: Plane) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>((other.g0_.x * self_.g0_ * -1.0), (other.g0_.y * self_.g0_ * -1.0), (other.g0_.z * self_.g0_ * -1.0)));
}
fn plane_add_origin(self: Plane, other: Origin) -> Flector {
    let addition: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other.g0_), /* e423, e431, e412, e321 */ self_.g0_);
    return addition;
}
fn antiScalar_sub_antiScalar(self: AntiScalar, other: AntiScalar) -> AntiScalar {
    let subtraction: AntiScalar = AntiScalar(/* e1234 */ (-other.g0_ + self_.g0_));
    return subtraction;
}
fn multiVector_sub_antiScalar(self: MultiVector, other: AntiScalar) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g0_.x, (self_.g0_.y - other.g0_)), /* e1, e2, e3, e4 */ self_.g1_, /* e41, e42, e43 */ self_.g2_, /* e23, e31, e12 */ self_.g3_, /* e423, e431, e412, e321 */ self_.g4_);
    return subtraction;
}
fn origin_geometricAntiQuotient_line(self: Origin, other: Line) -> Origin {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Origin = Origin(/* e4 */ (anti_inverse.g0_ * self_.g0_));
    return geometric_anti_product;
}
fn point_antiWedge_motor(self: Point, other: Motor) -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.w * self_.g0_.x), (other.g0_.w * self_.g0_.y), (other.g0_.w * self_.g0_.z), (other.g0_.w * self_.g0_.w)));
}
fn line_wedge_dualNum(self: Line, other: DualNum) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>((other.g0_.x * self_.g0_.x), (other.g0_.x * self_.g0_.y), (other.g0_.x * self_.g0_.z)), /* e23, e31, e12 */ vec3<f32>((other.g0_.x * self_.g1_.x), (other.g0_.x * self_.g1_.y), (other.g0_.x * self_.g1_.z)));
}
fn dualNum_sandwich_plane(self: DualNum, other: Plane) -> Flector {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.y * other.g0_.w * -1.0)), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_.x) * other.g0_));
    return flector_geometricProduct_flector(geometric_product, dualNum_reverse(self_));
}
fn horizon_geometricQuotient_motor(self: Horizon, other: Motor) -> Horizon {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2) + pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Horizon = Horizon(/* e321 */ (self_.g0_ * inverse.g0_));
    return geometric_product;
}
fn plane_sandwich_line(self: Plane, other: Line) -> Motor {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g1_.x * self_.g0_.w), (other.g1_.y * self_.g0_.w), (other.g1_.z * self_.g0_.w), (-(other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))), /* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.x * self_.g0_.w) + (other.g1_.y * self_.g0_.z) - (other.g1_.z * self_.g0_.y)), ((other.g0_.y * self_.g0_.w) - (other.g1_.x * self_.g0_.z) + (other.g1_.z * self_.g0_.x)), ((other.g0_.z * self_.g0_.w) + (other.g1_.x * self_.g0_.y) - (other.g1_.y * self_.g0_.x)), 0.0));
    return flector_geometricProduct_flector(geometric_product, plane_reverse(self_));
}
fn plane_antiSandwich_origin(self: Plane, other: Origin) -> Flector {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g0_.x * other.g0_ * -1.0), (self_.g0_.y * other.g0_ * -1.0), (self_.g0_.z * other.g0_ * -1.0), 0.0), /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.w * other.g0_ * -1.0)));
    return motor_geometricAntiProduct_motor(geometric_anti_product, plane_antiReverse(self_));
}
fn line_geometricProduct_dualNum(self: Line, other: DualNum) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>(((other.g0_.x * self_.g0_.x) + (other.g0_.y * self_.g1_.x)), ((other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g1_.y)), ((other.g0_.x * self_.g0_.z) + (other.g0_.y * self_.g1_.z))), /* e23, e31, e12 */ vec3<f32>((other.g0_.x * self_.g1_.x), (other.g0_.x * self_.g1_.y), (other.g0_.x * self_.g1_.z)));
}
fn horizon_add_antiScalar(self: Horizon, other: AntiScalar) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, other.g0_), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_));
    return addition;
}
fn plane_geometricAntiQuotient_multiVector(self: Plane, other: MultiVector) -> Plane {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.y, 2) - pow(other.g2_.x, 2) - pow(other.g2_.y, 2) - pow(other.g2_.z, 2) - pow(other.g1_.w, 2) + pow(other.g4_.x, 2) + pow(other.g4_.y, 2) + pow(other.g4_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Plane = Plane(/* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_));
    return geometric_anti_product;
}
fn flector_mul_flector(self: Flector, other: Flector) -> Motor {
    return flector_geometricProduct_flector(self_, other);
}
fn origin_antiSandwich_origin(self: Origin, other: Origin) -> Origin {
    let geometric_anti_product: AntiScalar = AntiScalar(/* e1234 */ (other.g0_ * self_.g0_ * -1.0));
    return antiScalar_geometricAntiProduct_antiScalar(geometric_anti_product, origin_antiReverse(self_));
}
fn multiVector_geometricProduct_flector(self: MultiVector, other: Flector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(((other.g0_.x * self_.g1_.x) + (other.g0_.y * self_.g1_.y) + (other.g0_.z * self_.g1_.z) - (other.g1_.w * self_.g4_.w)), (-(other.g0_.x * self_.g4_.x) - (other.g0_.y * self_.g4_.y) - (other.g0_.z * self_.g4_.z) - (other.g0_.w * self_.g4_.w) + (other.g1_.x * self_.g1_.x) + (other.g1_.y * self_.g1_.y) + (other.g1_.z * self_.g1_.z) + (other.g1_.w * self_.g1_.w))), /* e1, e2, e3, e4 */ vec4<f32>(((self_.g0_.x * other.g0_.x) + (self_.g3_.x * other.g1_.w) - (self_.g3_.y * other.g0_.z) + (self_.g3_.z * other.g0_.y)), ((self_.g0_.x * other.g0_.y) + (self_.g3_.x * other.g0_.z) + (self_.g3_.y * other.g1_.w) - (self_.g3_.z * other.g0_.x)), ((self_.g0_.x * other.g0_.z) - (self_.g3_.x * other.g0_.y) + (self_.g3_.y * other.g0_.x) + (self_.g3_.z * other.g1_.w)), ((self_.g0_.x * other.g0_.w) - (self_.g0_.y * other.g1_.w) + (self_.g2_.x * other.g0_.x) + (self_.g2_.y * other.g0_.y) + (self_.g2_.z * other.g0_.z) - (self_.g3_.x * other.g1_.x) - (self_.g3_.y * other.g1_.y) - (self_.g3_.z * other.g1_.z))), /* e41, e42, e43 */ vec3<f32>(((other.g0_.x * self_.g1_.w) + (other.g0_.y * self_.g4_.z) - (other.g0_.z * self_.g4_.y) - (other.g0_.w * self_.g1_.x) - (other.g1_.x * self_.g4_.w) - (other.g1_.y * self_.g1_.z) + (other.g1_.z * self_.g1_.y) + (other.g1_.w * self_.g4_.x)), (-(other.g0_.x * self_.g4_.z) + (other.g0_.y * self_.g1_.w) + (other.g0_.z * self_.g4_.x) - (other.g0_.w * self_.g1_.y) + (other.g1_.x * self_.g1_.z) - (other.g1_.y * self_.g4_.w) - (other.g1_.z * self_.g1_.x) + (other.g1_.w * self_.g4_.y)), ((other.g0_.x * self_.g4_.y) - (other.g0_.y * self_.g4_.x) + (other.g0_.z * self_.g1_.w) - (other.g0_.w * self_.g1_.z) - (other.g1_.x * self_.g1_.y) + (other.g1_.y * self_.g1_.x) - (other.g1_.z * self_.g4_.w) + (other.g1_.w * self_.g4_.z))), /* e23, e31, e12 */ vec3<f32>((-(other.g0_.x * self_.g4_.w) - (other.g0_.y * self_.g1_.z) + (other.g0_.z * self_.g1_.y) - (other.g1_.w * self_.g1_.x)), ((other.g0_.x * self_.g1_.z) - (other.g0_.y * self_.g4_.w) - (other.g0_.z * self_.g1_.x) - (other.g1_.w * self_.g1_.y)), (-(other.g0_.x * self_.g1_.y) + (other.g0_.y * self_.g1_.x) - (other.g0_.z * self_.g4_.w) - (other.g1_.w * self_.g1_.z))), /* e423, e431, e412, e321 */ vec4<f32>(((self_.g0_.x * other.g1_.x) - (self_.g0_.y * other.g0_.x) - (self_.g2_.x * other.g1_.w) + (self_.g2_.y * other.g0_.z) - (self_.g2_.z * other.g0_.y) + (self_.g3_.x * other.g0_.w) - (self_.g3_.y * other.g1_.z) + (self_.g3_.z * other.g1_.y)), ((self_.g0_.x * other.g1_.y) - (self_.g0_.y * other.g0_.y) - (self_.g2_.x * other.g0_.z) - (self_.g2_.y * other.g1_.w) + (self_.g2_.z * other.g0_.x) + (self_.g3_.x * other.g1_.z) + (self_.g3_.y * other.g0_.w) - (self_.g3_.z * other.g1_.x)), ((self_.g0_.x * other.g1_.z) - (self_.g0_.y * other.g0_.z) + (self_.g2_.x * other.g0_.y) - (self_.g2_.y * other.g0_.x) - (self_.g2_.z * other.g1_.w) - (self_.g3_.x * other.g1_.y) + (self_.g3_.y * other.g1_.x) + (self_.g3_.z * other.g0_.w)), ((self_.g0_.x * other.g1_.w) - (self_.g3_.x * other.g0_.x) - (self_.g3_.y * other.g0_.y) - (self_.g3_.z * other.g0_.z))));
}
fn origin_antiWedge_multiVector(self: Origin, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((other.g4_.w * self_.g0_), 0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.y * self_.g0_)), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn motor_antiWedge_line(self: Motor, other: Line) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g0_.x * self_.g0_.w), (other.g0_.y * self_.g0_.w), (other.g0_.z * self_.g0_.w), 0.0), /* e23, e31, e12, scalar */ vec4<f32>((other.g1_.x * self_.g0_.w), (other.g1_.y * self_.g0_.w), (other.g1_.z * self_.g0_.w), (-(other.g0_.x * self_.g1_.x) - (other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))));
}
fn multiVector_antiSandwich_horizon(self: MultiVector, other: Horizon) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>((self_.g1_.w * other.g0_), 0.0), /* e1, e2, e3, e4 */ vec4<f32>((self_.g2_.x * other.g0_), (self_.g2_.y * other.g0_), (self_.g2_.z * other.g0_), 0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ (vec3<f32>(other.g0_) * vec3<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z)), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.y * other.g0_)));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self_));
}
fn antiScalar_sub_plane(self: AntiScalar, other: Plane) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, self_.g0_), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ (other.g0_ * vec4<f32>(-1.0)));
    return subtraction;
}
fn antiScalar_add_flector(self: AntiScalar, other: Flector) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, self_.g0_), /* e1, e2, e3, e4 */ other.g0_, /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ other.g1_);
    return addition;
}
fn line_sub_horizon(self: Line, other: Horizon) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ self_.g0_, /* e23, e31, e12 */ self_.g1_, /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_ * -1.0)));
    return subtraction;
}
fn scalar_mul_plane(self: Scalar, other: Plane) -> Plane {
    return scalar_geometricProduct_scalar(self_, other);
}
fn plane_mul_dualNum(self: Plane, other: DualNum) -> Flector {
    return plane_geometricProduct_plane(self_, other);
}
fn line_sandwich_motor(self: Line, other: Motor) -> Motor {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(((self_.g0_.x * other.g1_.w) + (self_.g0_.z * other.g1_.y) + (self_.g1_.x * other.g0_.w) + (self_.g1_.z * other.g0_.y)), ((self_.g0_.x * other.g1_.z) + (self_.g0_.y * other.g1_.w) + (self_.g1_.x * other.g0_.z) + (self_.g1_.y * other.g0_.w)), ((self_.g0_.y * other.g1_.x) + (self_.g0_.z * other.g1_.w) + (self_.g1_.y * other.g0_.x) + (self_.g1_.z * other.g0_.w)), (-(self_.g0_.y * other.g1_.y) - (self_.g0_.z * other.g1_.z) - (self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))) - (vec4<f32>(self_.g0_.y, self_.g0_.z, self_.g0_.x, self_.g0_.x) * other.g1_.zxyx) - (vec4<f32>(self_.g1_.y, self_.g1_.z, self_.g1_.x, self_.g1_.x) * other.g0_.zxyx)), /* e23, e31, e12, scalar */ (vec4<f32>(((self_.g1_.x * other.g1_.w) + (self_.g1_.z * other.g1_.y)), ((self_.g1_.x * other.g1_.z) + (self_.g1_.y * other.g1_.w)), ((self_.g1_.y * other.g1_.x) + (self_.g1_.z * other.g1_.w)), (-(self_.g1_.y * other.g1_.y) - (self_.g1_.z * other.g1_.z))) - (vec4<f32>(self_.g1_.y, self_.g1_.z, self_.g1_.x, self_.g1_.x) * other.g1_.zxyx)));
    return motor_geometricProduct_motor(geometric_product, line_reverse(self_));
}
fn motor_mul_flector(self: Motor, other: Flector) -> Flector {
    return motor_geometricProduct_motor(self_, other);
}
fn antiScalar_add_dualNum(self: AntiScalar, other: DualNum) -> DualNum {
    let addition: DualNum = DualNum(/* scalar, e1234 */ vec2<f32>(other.g0_.x, (other.g0_.y + self_.g0_)));
    return addition;
}
fn horizon_bitXor_multiVector(self: Horizon, other: MultiVector) -> MultiVector {
    return horizon_wedge_horizon(self_, other);
}
fn antiScalar_geometricAntiQuotient_antiScalar(self: AntiScalar, other: AntiScalar) -> AntiScalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: AntiScalar = AntiScalar(/* e1234 */ (anti_inverse.g0_ * self_.g0_));
    return geometric_anti_product;
}
fn origin_bitXor_multiVector(self: Origin, other: MultiVector) -> MultiVector {
    return origin_wedge_origin(self_, other);
}
fn plane_sub_dualNum(self: Plane, other: DualNum) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ (other.g0_ * vec2<f32>(-1.0)), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ self_.g0_);
    return subtraction;
}
fn dualNum_antiSandwich_antiScalar(self: DualNum, other: AntiScalar) -> DualNum {
    let geometric_anti_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>(other.g0_) * self_.g0_));
    return dualNum_geometricAntiProduct_dualNum(geometric_anti_product, dualNum_antiReverse(self_));
}
fn line_mul_line(self: Line, other: Line) -> Motor {
    return line_geometricProduct_line(self_, other);
}
fn multiVector_mul_horizon(self: MultiVector, other: Horizon) -> MultiVector {
    return multiVector_geometricProduct_multiVector(self_, other);
}
fn scalar_geometricProduct_plane(self: Scalar, other: Plane) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), (other.g0_.w * self_.g0_)));
}
fn line_sandwich_antiScalar(self: Line, other: AntiScalar) -> Motor {
    let geometric_product: Line = Line(/* e41, e42, e43 */ (vec3<f32>(other.g0_) * self_.g1_), /* e23, e31, e12 */ vec3<f32>(0.0));
    return line_geometricProduct_line(geometric_product, line_reverse(self_));
}
fn dualNum_geometricQuotient_horizon(self: DualNum, other: Horizon) -> DualNum {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>(inverse.g0_) * self_.g0_));
    return geometric_product;
}
fn plane_geometricProduct_point(self: Plane, other: Point) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((-(self_.g0_.y * other.g0_.z) + (self_.g0_.z * other.g0_.y)), ((self_.g0_.x * other.g0_.z) - (self_.g0_.z * other.g0_.x)), (-(self_.g0_.x * other.g0_.y) + (self_.g0_.y * other.g0_.x)), (-(self_.g0_.x * other.g0_.x) - (self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z) - (self_.g0_.w * other.g0_.w))), /* e23, e31, e12, scalar */ vec4<f32>((self_.g0_.w * other.g0_.x * -1.0), (self_.g0_.w * other.g0_.y * -1.0), (self_.g0_.w * other.g0_.z * -1.0), 0.0));
}
fn point_add_point(self: Point, other: Point) -> Point {
    let addition: Point = Point(/* e1, e2, e3, e4 */ (other.g0_ + self_.g0_));
    return addition;
}
fn origin_geometricAntiQuotient_point(self: Origin, other: Point) -> Origin {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.w, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Origin = Origin(/* e4 */ (anti_inverse.g0_ * self_.g0_));
    return geometric_anti_product;
}
fn point_antiSandwich_point(self: Point, other: Point) -> Flector {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.w * self_.g0_.w * -1.0)), /* e23, e31, e12, scalar */ vec4<f32>(((other.g0_.x * self_.g0_.w) - (other.g0_.w * self_.g0_.x)), ((other.g0_.y * self_.g0_.w) - (other.g0_.w * self_.g0_.y)), ((other.g0_.z * self_.g0_.w) - (other.g0_.w * self_.g0_.z)), 0.0));
    return motor_geometricAntiProduct_motor(geometric_anti_product, point_antiReverse(self_));
}
fn antiScalar_mul_motor(self: AntiScalar, other: Motor) -> Motor {
    return antiScalar_geometricProduct_antiScalar(self_, other);
}
fn antiScalar_geometricProduct_scalar(self: AntiScalar, other: Scalar) -> AntiScalar {
    return AntiScalar(/* e1234 */ (self_.g0_ * other.g0_));
}
fn scalar_antiWedge_antiScalar(self: Scalar, other: AntiScalar) -> Scalar {
    return Scalar(/* scalar */ (other.g0_ * self_.g0_));
}
fn line_bitXor_origin(self: Line, other: Origin) -> Plane {
    return line_wedge_line(self_, other);
}
fn multiVector_geometricAntiQuotient_flector(self: MultiVector, other: Flector) -> MultiVector {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.w, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(anti_inverse.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(anti_inverse.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(anti_inverse.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g4_));
    return geometric_anti_product;
}
fn dualNum_mul_horizon(self: DualNum, other: Horizon) -> Flector {
    return dualNum_geometricProduct_dualNum(self_, other);
}
fn antiScalar_add_multiVector(self: AntiScalar, other: MultiVector) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(other.g0_.x, (other.g0_.y + self_.g0_)), /* e1, e2, e3, e4 */ other.g1_, /* e41, e42, e43 */ other.g2_, /* e23, e31, e12 */ other.g3_, /* e423, e431, e412, e321 */ other.g4_);
    return addition;
}
fn antiScalar_geometricAntiQuotient_origin(self: AntiScalar, other: Origin) -> AntiScalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: AntiScalar = AntiScalar(/* e1234 */ (anti_inverse.g0_ * self_.g0_));
    return geometric_anti_product;
}
fn multiVector_sandwich_dualNum(self: MultiVector, other: DualNum) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>((other.g0_.x * self_.g0_.x), ((other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x))), /* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g1_.x), (other.g0_.x * self_.g1_.y), (other.g0_.x * self_.g1_.z), ((other.g0_.x * self_.g1_.w) + (other.g0_.y * self_.g4_.w))), /* e41, e42, e43 */ ((vec3<f32>(other.g0_.x) * self_.g2_) + (vec3<f32>(other.g0_.y) * self_.g3_)), /* e23, e31, e12 */ (vec3<f32>(other.g0_.x) * self_.g3_), /* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.x * self_.g4_.x) + (other.g0_.y * self_.g1_.x)), ((other.g0_.x * self_.g4_.y) + (other.g0_.y * self_.g1_.y)), ((other.g0_.x * self_.g4_.z) + (other.g0_.y * self_.g1_.z)), (other.g0_.x * self_.g4_.w)));
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self_));
}
fn dualNum_add_flector(self: DualNum, other: Flector) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ other.g0_, /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ other.g1_);
    return addition;
}
fn flector_antiWedge_antiScalar(self: Flector, other: AntiScalar) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), (self_.g0_.w * other.g0_)), /* e423, e431, e412, e321 */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), (self_.g1_.w * other.g0_)));
}
fn antiScalar_sub_scalar(self: AntiScalar, other: Scalar) -> DualNum {
    let subtraction: DualNum = DualNum(/* scalar, e1234 */ vec2<f32>((other.g0_ * -1.0), self_.g0_));
    return subtraction;
}
fn line_antiSandwich_horizon(self: Line, other: Horizon) -> Flector {
    let geometric_anti_product: Point = Point(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), 0.0));
    return point_geometricAntiProduct_point(geometric_anti_product, line_antiReverse(self_));
}
fn scalar_antiSandwich_plane(self: Scalar, other: Plane) -> Horizon {
    let geometric_anti_product: Point = Point(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), 0.0));
    return point_geometricAntiProduct_point(geometric_anti_product, scalar_antiReverse(self_));
}
fn line_wedge_flector(self: Line, other: Flector) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>(((self_.g0_.y * other.g0_.z) - (self_.g0_.z * other.g0_.y) + (self_.g1_.x * other.g0_.w)), (-(self_.g0_.x * other.g0_.z) + (self_.g0_.z * other.g0_.x) + (self_.g1_.y * other.g0_.w)), ((self_.g0_.x * other.g0_.y) - (self_.g0_.y * other.g0_.x) + (self_.g1_.z * other.g0_.w)), (-(self_.g1_.x * other.g0_.x) - (self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))));
}
fn horizon_sandwich_flector(self: Horizon, other: Flector) -> Flector {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, other.g0_.w) * vec4<f32>(-1.0)), /* e23, e31, e12, scalar */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.w) * vec4<f32>(-1.0)));
    return motor_geometricProduct_motor(geometric_product, horizon_reverse(self_));
}
fn scalar_geometricAntiQuotient_line(self: Scalar, other: Line) -> Scalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Scalar = Scalar(/* scalar */ (anti_inverse.g0_ * self_.g0_));
    return geometric_anti_product;
}
fn origin_geometricAntiProduct_origin(self: Origin, other: Origin) -> AntiScalar {
    return AntiScalar(/* e1234 */ (other.g0_ * self_.g0_ * -1.0));
}
fn scalar_sandwich_dualNum(self: Scalar, other: DualNum) -> DualNum {
    let geometric_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>(self_.g0_) * other.g0_));
    return dualNum_geometricProduct_dualNum(geometric_product, scalar_reverse(self_));
}
fn flector_antiSandwich_flector(self: Flector, other: Flector) -> Flector {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>((-(other.g1_.x * self_.g0_.w) - (other.g1_.z * self_.g1_.y)), (-(other.g1_.x * self_.g1_.z) - (other.g1_.y * self_.g0_.w)), (-(other.g1_.y * self_.g1_.x) - (other.g1_.z * self_.g0_.w)), ((other.g1_.y * self_.g1_.y) + (other.g1_.z * self_.g1_.z))) - (vec4<f32>(other.g0_.w) * vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g0_.w)) + (other.g1_.yzxx * self_.g1_.zxyx)), /* e23, e31, e12, scalar */ (vec4<f32>(((other.g0_.z * self_.g1_.y) - (other.g1_.z * self_.g0_.y)), ((other.g0_.x * self_.g1_.z) - (other.g1_.x * self_.g0_.z)), ((other.g0_.y * self_.g1_.x) - (other.g1_.y * self_.g0_.x)), (-(other.g0_.w * self_.g1_.w) + (other.g1_.w * self_.g0_.w))) + (vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.x) * self_.g0_.wwwx) - (vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, other.g0_.z) * self_.g1_.wwwz) - (vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g1_.y) * other.g0_.wwwy) + (vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g0_.z) * other.g1_.wwwz) - (other.g0_.yzxx * self_.g1_.zxyx) + (other.g1_.yzxy * self_.g0_.zxyy)));
    return motor_geometricAntiProduct_motor(geometric_anti_product, flector_antiReverse(self_));
}
fn dualNum_geometricAntiQuotient_point(self: DualNum, other: Point) -> DualNum {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.w, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>(anti_inverse.g0_) * self_.g0_));
    return geometric_anti_product;
}
fn horizon_geometricAntiProduct_point(self: Horizon, other: Point) -> Scalar {
    return Scalar(/* scalar */ (other.g0_.w * self_.g0_ * -1.0));
}
fn plane_sandwich_scalar(self: Plane, other: Scalar) -> Motor {
    let geometric_product: Plane = Plane(/* e423, e431, e412, e321 */ (vec4<f32>(other.g0_) * self_.g0_));
    return plane_geometricProduct_plane(geometric_product, plane_reverse(self_));
}
fn motor_antiWedge_origin(self: Motor, other: Origin) -> Origin {
    return Origin(/* e4 */ (self_.g0_.w * other.g0_));
}
fn flector_sandwich_flector(self: Flector, other: Flector) -> Flector {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(((other.g0_.y * self_.g1_.z) - (other.g1_.y * self_.g0_.z)), ((other.g0_.z * self_.g1_.x) - (other.g1_.z * self_.g0_.x)), ((other.g0_.x * self_.g1_.y) - (other.g1_.x * self_.g0_.y)), (-(other.g0_.w * self_.g1_.w) + (other.g1_.w * self_.g0_.w))) + (vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.x) * self_.g0_.wwwx) - (vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, other.g0_.z) * self_.g1_.wwwz) - (vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g1_.y) * other.g0_.wwwy) + (vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g0_.z) * other.g1_.wwwz) - (other.g0_.zxyx * self_.g1_.yzxx) + (other.g1_.zxyy * self_.g0_.yzxy)), /* e23, e31, e12, scalar */ (vec4<f32>((-(other.g0_.y * self_.g0_.z) - (other.g1_.w * self_.g0_.x)), (-(other.g0_.z * self_.g0_.x) - (other.g1_.w * self_.g0_.y)), (-(other.g0_.x * self_.g0_.y) - (other.g1_.w * self_.g0_.z)), ((other.g0_.y * self_.g0_.y) + (other.g0_.z * self_.g0_.z))) - (vec4<f32>(self_.g1_.w) * vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.w)) + (other.g0_.zxyx * self_.g0_.yzxx)));
    return motor_geometricProduct_motor(geometric_product, flector_reverse(self_));
}
fn plane_add_motor(self: Plane, other: Motor) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(other.g1_.w, other.g0_.w), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z), /* e23, e31, e12 */ vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z), /* e423, e431, e412, e321 */ self_.g0_);
    return addition;
}
fn motor_antiSandwich_multiVector(self: Motor, other: MultiVector) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(((other.g0_.y * self_.g1_.w) - (other.g3_.x * self_.g0_.x) - (other.g3_.y * self_.g0_.y) - (other.g3_.z * self_.g0_.z)), 0.0) - (vec2<f32>(other.g2_.x) * vec2<f32>(self_.g1_.x, self_.g0_.x)) - (vec2<f32>(other.g2_.y) * vec2<f32>(self_.g1_.y, self_.g0_.y)) - (vec2<f32>(other.g2_.z) * vec2<f32>(self_.g1_.z, self_.g0_.z)) + (vec2<f32>(self_.g0_.w) * other.g0_)), /* e1, e2, e3, e4 */ (vec4<f32>(((self_.g0_.y * other.g1_.z) + (self_.g0_.w * other.g1_.x) + (self_.g1_.x * other.g1_.w) + (self_.g1_.y * other.g4_.z) + (self_.g1_.w * other.g4_.x)), ((self_.g0_.z * other.g1_.x) + (self_.g0_.w * other.g1_.y) + (self_.g1_.y * other.g1_.w) + (self_.g1_.z * other.g4_.x) + (self_.g1_.w * other.g4_.y)), ((self_.g0_.z * other.g4_.w) + (self_.g0_.w * other.g1_.z) + (self_.g1_.x * other.g4_.y) + (self_.g1_.z * other.g1_.w) + (self_.g1_.w * other.g4_.z)), ((self_.g0_.z * other.g4_.z) * -1.0)) - (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g0_.y) * other.g4_.yzxy) - (vec4<f32>(other.g1_.y, other.g1_.z, other.g1_.x, other.g4_.x) * self_.g0_.zxyx) + (vec4<f32>(other.g4_.w, other.g4_.w, other.g1_.y, other.g1_.w) * self_.g0_.xyxw)), /* e41, e42, e43 */ ((vec3<f32>(other.g0_.y) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z)) + (vec3<f32>(self_.g0_.y, self_.g0_.w, self_.g0_.w) * other.g2_.zyz) - (vec3<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y) * other.g2_.yzx) + (vec3<f32>(self_.g0_.w, self_.g0_.z, self_.g0_.x) * other.g2_.xxy)), /* e23, e31, e12 */ ((vec3<f32>(other.g0_.x) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z)) + (vec3<f32>(other.g0_.y) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z)) + (vec3<f32>(self_.g0_.y, self_.g0_.w, self_.g0_.w) * other.g3_.zyz) - (vec3<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y) * other.g3_.yzx) + (vec3<f32>(self_.g0_.w, self_.g0_.z, self_.g0_.x) * other.g3_.xxy) + (vec3<f32>(self_.g1_.y, self_.g1_.w, self_.g1_.w) * other.g2_.zyz) - (vec3<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y) * other.g2_.yzx) + (vec3<f32>(self_.g1_.w, self_.g1_.z, self_.g1_.x) * other.g2_.xxy)), /* e423, e431, e412, e321 */ (vec4<f32>(((self_.g0_.y * other.g4_.z) + (self_.g0_.w * other.g4_.x)), ((self_.g0_.z * other.g4_.x) + (self_.g0_.w * other.g4_.y)), ((self_.g0_.z * other.g1_.w) + (self_.g0_.w * other.g4_.z)), (-(self_.g0_.y * other.g1_.y) - (self_.g0_.z * other.g1_.z) - (self_.g1_.x * other.g4_.x) - (self_.g1_.y * other.g4_.y) - (self_.g1_.z * other.g4_.z) + (self_.g1_.w * other.g1_.w))) + (vec4<f32>(other.g1_.w, other.g1_.w, other.g4_.y, other.g4_.w) * self_.g0_.xyxw) - (vec4<f32>(other.g4_.y, other.g4_.z, other.g4_.x, other.g1_.x) * self_.g0_.zxyx)));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, motor_antiReverse(self_));
}
fn dualNum_geometricQuotient_scalar(self: DualNum, other: Scalar) -> DualNum {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>(inverse.g0_) * self_.g0_));
    return geometric_product;
}
fn antiScalar_sub_horizon(self: AntiScalar, other: Horizon) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, self_.g0_), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_ * -1.0)));
    return subtraction;
}
fn antiScalar_sub_origin(self: AntiScalar, other: Origin) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, self_.g0_), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_ * -1.0)), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    return subtraction;
}
fn origin_geometricAntiProduct_scalar(self: Origin, other: Scalar) -> Horizon {
    return Horizon(/* e321 */ (self_.g0_ * other.g0_ * -1.0));
}
fn dualNum_wedge_flector(self: DualNum, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g0_.x), (self_.g0_.x * other.g0_.y), (self_.g0_.x * other.g0_.z), (self_.g0_.x * other.g0_.w)), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.x * other.g1_.x), (self_.g0_.x * other.g1_.y), (self_.g0_.x * other.g1_.z), (self_.g0_.x * other.g1_.w)));
}
fn horizon_sub_horizon(self: Horizon, other: Horizon) -> Horizon {
    let subtraction: Horizon = Horizon(/* e321 */ (-other.g0_ + self_.g0_));
    return subtraction;
}
fn flector_wedge_horizon(self: Flector, other: Horizon) -> AntiScalar {
    return AntiScalar(/* e1234 */ (self_.g0_.w * other.g0_));
}
fn motor_sub_scalar(self: Motor, other: Scalar) -> Motor {
    let subtraction: Motor = Motor(/* e41, e42, e43, e1234 */ self_.g0_, /* e23, e31, e12, scalar */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, (self_.g1_.w - other.g0_)));
    return subtraction;
}
fn origin_wedge_dualNum(self: Origin, other: DualNum) -> Origin {
    return Origin(/* e4 */ (other.g0_.x * self_.g0_));
}
fn antiScalar_antiWedge_scalar(self: AntiScalar, other: Scalar) -> Scalar {
    return Scalar(/* scalar */ (self_.g0_ * other.g0_));
}
fn dualNum_wedge_dualNum(self: DualNum, other: DualNum) -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>((other.g0_.x * self_.g0_.x), ((other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x))));
}
fn point_sandwich_antiScalar(self: Point, other: AntiScalar) -> Motor {
    let geometric_product: Plane = Plane(/* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), 0.0));
    return plane_geometricProduct_plane(geometric_product, point_reverse(self_));
}
fn motor_sandwich_line(self: Motor, other: Line) -> Motor {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(((other.g0_.x * self_.g1_.w) + (other.g0_.y * self_.g1_.z) + (other.g1_.x * self_.g0_.w) + (other.g1_.y * self_.g0_.z)), ((other.g0_.y * self_.g1_.w) + (other.g0_.z * self_.g1_.x) + (other.g1_.y * self_.g0_.w) + (other.g1_.z * self_.g0_.x)), ((other.g0_.x * self_.g1_.y) + (other.g0_.z * self_.g1_.w) + (other.g1_.x * self_.g0_.y) + (other.g1_.z * self_.g0_.w)), (-(other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))) - (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.y, other.g0_.x) * self_.g1_.yzxx) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g1_.x) * self_.g0_.yzxx)), /* e23, e31, e12, scalar */ (vec4<f32>(((other.g1_.x * self_.g1_.w) + (other.g1_.y * self_.g1_.z)), ((other.g1_.y * self_.g1_.w) + (other.g1_.z * self_.g1_.x)), ((other.g1_.x * self_.g1_.y) + (other.g1_.z * self_.g1_.w)), (-(other.g1_.y * self_.g1_.y) - (other.g1_.z * self_.g1_.z))) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g1_.x) * self_.g1_.yzxx)));
    return motor_geometricProduct_motor(geometric_product, motor_reverse(self_));
}
fn antiScalar_geometricAntiQuotient_motor(self: AntiScalar, other: Motor) -> AntiScalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2) + pow(other.g0_.w, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: AntiScalar = AntiScalar(/* e1234 */ (anti_inverse.g0_ * self_.g0_));
    return geometric_anti_product;
}
fn flector_geometricQuotient_point(self: Flector, other: Point) -> Flector {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g1_));
    return geometric_product;
}
fn dualNum_geometricQuotient_motor(self: DualNum, other: Motor) -> DualNum {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2) + pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>(inverse.g0_) * self_.g0_));
    return geometric_product;
}
fn origin_add_point(self: Origin, other: Point) -> Point {
    let addition: Point = Point(/* e1, e2, e3, e4 */ vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, (other.g0_.w + self_.g0_)));
    return addition;
}
fn point_geometricAntiProduct_horizon(self: Point, other: Horizon) -> Scalar {
    return Scalar(/* scalar */ (self_.g0_.w * other.g0_));
}
fn line_add_origin(self: Line, other: Origin) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other.g0_), /* e41, e42, e43 */ self_.g0_, /* e23, e31, e12 */ self_.g1_, /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    return addition;
}
fn line_geometricAntiQuotient_dualNum(self: Line, other: DualNum) -> Line {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_.y, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Line = Line(/* e41, e42, e43 */ (vec3<f32>(anti_inverse.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(anti_inverse.g0_) * self_.g1_));
    return geometric_anti_product;
}
fn plane_geometricProduct_horizon(self: Plane, other: Horizon) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), 0.0), /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.w * other.g0_ * -1.0)));
}
fn origin_sub_antiScalar(self: Origin, other: AntiScalar) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (other.g0_ * -1.0)), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    return subtraction;
}
fn motor_antiSandwich_point(self: Motor, other: Point) -> Flector {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>(((self_.g0_.y * other.g0_.z) - (self_.g0_.z * other.g0_.y) + (self_.g0_.w * other.g0_.x) + (self_.g1_.x * other.g0_.w)), (-(self_.g0_.x * other.g0_.z) + (self_.g0_.z * other.g0_.x) + (self_.g0_.w * other.g0_.y) + (self_.g1_.y * other.g0_.w)), ((self_.g0_.x * other.g0_.y) - (self_.g0_.y * other.g0_.x) + (self_.g0_.w * other.g0_.z) + (self_.g1_.z * other.g0_.w)), (self_.g0_.w * other.g0_.w)), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.x * other.g0_.w), (self_.g0_.y * other.g0_.w), (self_.g0_.z * other.g0_.w), (-(self_.g0_.x * other.g0_.x) - (self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z) + (self_.g1_.w * other.g0_.w))));
    return flector_geometricAntiProduct_flector(geometric_anti_product, motor_antiReverse(self_));
}
fn multiVector_sub_point(self: MultiVector, other: Point) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ (self_.g1_ - other.g0_), /* e41, e42, e43 */ self_.g2_, /* e23, e31, e12 */ self_.g3_, /* e423, e431, e412, e321 */ self_.g4_);
    return subtraction;
}
fn line_geometricQuotient_plane(self: Line, other: Plane) -> Line {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.w, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Line = Line(/* e41, e42, e43 */ (vec3<f32>(inverse.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(inverse.g0_) * self_.g1_));
    return geometric_product;
}
fn plane_bitXor_scalar(self: Plane, other: Scalar) -> Plane {
    return plane_wedge_plane(self_, other);
}
fn flector_sandwich_horizon(self: Flector, other: Horizon) -> Flector {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g0_.w)), /* e23, e31, e12, scalar */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g1_.w) * vec4<f32>(-1.0)));
    return motor_geometricProduct_motor(geometric_product, flector_reverse(self_));
}
fn motor_geometricProduct_point(self: Motor, other: Point) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((-(self_.g1_.y * other.g0_.z) + (self_.g1_.z * other.g0_.y) + (self_.g1_.w * other.g0_.x)), ((self_.g1_.x * other.g0_.z) - (self_.g1_.z * other.g0_.x) + (self_.g1_.w * other.g0_.y)), (-(self_.g1_.x * other.g0_.y) + (self_.g1_.y * other.g0_.x) + (self_.g1_.w * other.g0_.z)), ((self_.g0_.x * other.g0_.x) + (self_.g0_.y * other.g0_.y) + (self_.g0_.z * other.g0_.z) + (self_.g1_.w * other.g0_.w))), /* e423, e431, e412, e321 */ vec4<f32>(((self_.g0_.y * other.g0_.z) - (self_.g0_.z * other.g0_.y) - (self_.g0_.w * other.g0_.x) + (self_.g1_.x * other.g0_.w)), (-(self_.g0_.x * other.g0_.z) + (self_.g0_.z * other.g0_.x) - (self_.g0_.w * other.g0_.y) + (self_.g1_.y * other.g0_.w)), ((self_.g0_.x * other.g0_.y) - (self_.g0_.y * other.g0_.x) - (self_.g0_.w * other.g0_.z) + (self_.g1_.z * other.g0_.w)), (-(self_.g1_.x * other.g0_.x) - (self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))));
}
fn flector_sub_plane(self: Flector, other: Plane) -> Flector {
    let subtraction: Flector = Flector(/* e1, e2, e3, e4 */ self_.g0_, /* e423, e431, e412, e321 */ (self_.g1_ - other.g0_));
    return subtraction;
}
fn scalar_sub_flector(self: Scalar, other: Flector) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g0_, 0.0), /* e1, e2, e3, e4 */ (other.g0_ * vec4<f32>(-1.0)), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ (other.g1_ * vec4<f32>(-1.0)));
    return subtraction;
}
fn multiVector_bitXor_antiScalar(self: MultiVector, other: AntiScalar) -> AntiScalar {
    return multiVector_wedge_multiVector(self_, other);
}
fn origin_mul_horizon(self: Origin, other: Horizon) -> AntiScalar {
    return origin_geometricProduct_origin(self_, other);
}
fn dualNum_sub_antiScalar(self: DualNum, other: AntiScalar) -> DualNum {
    let subtraction: DualNum = DualNum(/* scalar, e1234 */ vec2<f32>(self_.g0_.x, (self_.g0_.y - other.g0_)));
    return subtraction;
}
fn horizon_geometricAntiProduct_multiVector(self: Horizon, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((other.g1_.w * self_.g0_ * -1.0), 0.0), /* e1, e2, e3, e4 */ vec4<f32>((other.g2_.x * self_.g0_), (other.g2_.y * self_.g0_), (other.g2_.z * self_.g0_), 0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>((other.g4_.x * self_.g0_ * -1.0), (other.g4_.y * self_.g0_ * -1.0), (other.g4_.z * self_.g0_ * -1.0)), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.y * self_.g0_)));
}
fn antiScalar_mul_line(self: AntiScalar, other: Line) -> Line {
    return antiScalar_geometricProduct_antiScalar(self_, other);
}
fn motor_antiWedge_motor(self: Motor, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(((other.g0_.x * self_.g0_.w) + (other.g0_.w * self_.g0_.x)), ((other.g0_.y * self_.g0_.w) + (other.g0_.w * self_.g0_.y)), ((other.g0_.z * self_.g0_.w) + (other.g0_.w * self_.g0_.z)), (other.g0_.w * self_.g0_.w)), /* e23, e31, e12, scalar */ vec4<f32>(((other.g0_.w * self_.g1_.x) + (other.g1_.x * self_.g0_.w)), ((other.g0_.w * self_.g1_.y) + (other.g1_.y * self_.g0_.w)), ((other.g0_.w * self_.g1_.z) + (other.g1_.z * self_.g0_.w)), (-(other.g0_.x * self_.g1_.x) - (other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) + (other.g0_.w * self_.g1_.w) - (other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z) + (other.g1_.w * self_.g0_.w))));
}
fn dualNum_wedge_point(self: DualNum, other: Point) -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g0_.x), (self_.g0_.x * other.g0_.y), (self_.g0_.x * other.g0_.z), (self_.g0_.x * other.g0_.w)));
}
fn point_geometricQuotient_scalar(self: Point, other: Scalar) -> Point {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Point = Point(/* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g0_));
    return geometric_product;
}
fn point_geometricQuotient_multiVector(self: Point, other: MultiVector) -> Point {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) - pow(other.g3_.x, 2) - pow(other.g3_.y, 2) - pow(other.g3_.z, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2) - pow(other.g4_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Point = Point(/* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g0_));
    return geometric_product;
}
fn motor_mul_point(self: Motor, other: Point) -> Flector {
    return motor_geometricProduct_motor(self_, other);
}
fn multiVector_sandwich_scalar(self: MultiVector, other: Scalar) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(other.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(other.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(other.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(other.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_) * self_.g4_));
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self_));
}
fn point_geometricAntiProduct_antiScalar(self: Point, other: AntiScalar) -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), (self_.g0_.w * other.g0_)));
}
fn scalar_wedge_plane(self: Scalar, other: Plane) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), (other.g0_.w * self_.g0_)));
}
fn point_bitXor_motor(self: Point, other: Motor) -> Flector {
    return point_wedge_point(self_, other);
}
fn dualNum_add_antiScalar(self: DualNum, other: AntiScalar) -> DualNum {
    let addition: DualNum = DualNum(/* scalar, e1234 */ vec2<f32>(self_.g0_.x, (self_.g0_.y + other.g0_)));
    return addition;
}
fn motor_geometricAntiProduct_line(self: Motor, other: Line) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(((other.g0_.x * self_.g0_.w) - (other.g0_.y * self_.g0_.z) + (other.g0_.z * self_.g0_.y)), ((other.g0_.x * self_.g0_.z) + (other.g0_.y * self_.g0_.w) - (other.g0_.z * self_.g0_.x)), (-(other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x) + (other.g0_.z * self_.g0_.w)), (-(other.g0_.x * self_.g0_.x) - (other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z))), /* e23, e31, e12, scalar */ vec4<f32>(((other.g0_.x * self_.g1_.w) - (other.g0_.y * self_.g1_.z) + (other.g0_.z * self_.g1_.y) + (other.g1_.x * self_.g0_.w) - (other.g1_.y * self_.g0_.z) + (other.g1_.z * self_.g0_.y)), ((other.g0_.x * self_.g1_.z) + (other.g0_.y * self_.g1_.w) - (other.g0_.z * self_.g1_.x) + (other.g1_.x * self_.g0_.z) + (other.g1_.y * self_.g0_.w) - (other.g1_.z * self_.g0_.x)), (-(other.g0_.x * self_.g1_.y) + (other.g0_.y * self_.g1_.x) + (other.g0_.z * self_.g1_.w) - (other.g1_.x * self_.g0_.y) + (other.g1_.y * self_.g0_.x) + (other.g1_.z * self_.g0_.w)), (-(other.g0_.x * self_.g1_.x) - (other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))));
}
fn motor_wedge_line(self: Motor, other: Line) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g0_.x * self_.g1_.w), (other.g0_.y * self_.g1_.w), (other.g0_.z * self_.g1_.w), (-(other.g0_.x * self_.g1_.x) - (other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))), /* e23, e31, e12, scalar */ vec4<f32>((other.g1_.x * self_.g1_.w), (other.g1_.y * self_.g1_.w), (other.g1_.z * self_.g1_.w), 0.0));
}
fn line_mul_horizon(self: Line, other: Horizon) -> Flector {
    return line_geometricProduct_line(self_, other);
}
fn dualNum_sandwich_multiVector(self: DualNum, other: MultiVector) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>((self_.g0_.x * other.g0_.x), ((self_.g0_.x * other.g0_.y) + (self_.g0_.y * other.g0_.x))), /* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g1_.x), (self_.g0_.x * other.g1_.y), (self_.g0_.x * other.g1_.z), ((self_.g0_.x * other.g1_.w) - (self_.g0_.y * other.g4_.w))), /* e41, e42, e43 */ ((vec3<f32>(self_.g0_.x) * other.g2_) + (vec3<f32>(self_.g0_.y) * other.g3_)), /* e23, e31, e12 */ (vec3<f32>(self_.g0_.x) * other.g3_), /* e423, e431, e412, e321 */ vec4<f32>(((self_.g0_.x * other.g4_.x) - (self_.g0_.y * other.g1_.x)), ((self_.g0_.x * other.g4_.y) - (self_.g0_.y * other.g1_.y)), ((self_.g0_.x * other.g4_.z) - (self_.g0_.y * other.g1_.z)), (self_.g0_.x * other.g4_.w)));
    return multiVector_geometricProduct_multiVector(geometric_product, dualNum_reverse(self_));
}
fn dualNum_antiWedge_scalar(self: DualNum, other: Scalar) -> Scalar {
    return Scalar(/* scalar */ (self_.g0_.y * other.g0_));
}
fn point_geometricAntiProduct_dualNum(self: Point, other: DualNum) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.y * self_.g0_.x), (other.g0_.y * self_.g0_.y), (other.g0_.y * self_.g0_.z), (other.g0_.y * self_.g0_.w)), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.x * self_.g0_.w * -1.0)));
}
fn scalar_antiWedge_multiVector(self: Scalar, other: MultiVector) -> Scalar {
    return Scalar(/* scalar */ (other.g0_.y * self_.g0_));
}
fn line_geometricQuotient_scalar(self: Line, other: Scalar) -> Line {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Line = Line(/* e41, e42, e43 */ (vec3<f32>(inverse.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(inverse.g0_) * self_.g1_));
    return geometric_product;
}
fn horizon_geometricAntiProduct_origin(self: Horizon, other: Origin) -> Scalar {
    return Scalar(/* scalar */ (self_.g0_ * other.g0_ * -1.0));
}
fn multiVector_geometricQuotient_horizon(self: MultiVector, other: Horizon) -> MultiVector {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(inverse.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(inverse.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(inverse.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g4_));
    return geometric_product;
}
fn multiVector_mul_origin(self: MultiVector, other: Origin) -> MultiVector {
    return multiVector_geometricProduct_multiVector(self_, other);
}
fn origin_sandwich_multiVector(self: Origin, other: MultiVector) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (other.g4_.w * self_.g0_)), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.x * self_.g0_)), /* e41, e42, e43 */ (vec3<f32>(self_.g0_) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z)), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>((other.g3_.x * self_.g0_), (other.g3_.y * self_.g0_), (other.g3_.z * self_.g0_), 0.0));
    return multiVector_geometricProduct_multiVector(geometric_product, origin_reverse(self_));
}
fn horizon_antiWedge_dualNum(self: Horizon, other: DualNum) -> Horizon {
    return Horizon(/* e321 */ (other.g0_.y * self_.g0_));
}
fn antiScalar_add_plane(self: AntiScalar, other: Plane) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, self_.g0_), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ other.g0_);
    return addition;
}
fn antiScalar_bitXor_dualNum(self: AntiScalar, other: DualNum) -> AntiScalar {
    return antiScalar_wedge_antiScalar(self_, other);
}
fn scalar_bitXor_line(self: Scalar, other: Line) -> Line {
    return scalar_wedge_scalar(self_, other);
}
fn origin_antiSandwich_horizon(self: Origin, other: Horizon) -> Horizon {
    let geometric_anti_product: Scalar = Scalar(/* scalar */ (other.g0_ * self_.g0_));
    return scalar_geometricAntiProduct_scalar(geometric_anti_product, origin_antiReverse(self_));
}
fn flector_geometricProduct_line(self: Flector, other: Line) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(((other.g1_.x * self_.g1_.w) + (other.g1_.y * self_.g0_.z) - (other.g1_.z * self_.g0_.y)), (-(other.g1_.x * self_.g0_.z) + (other.g1_.y * self_.g1_.w) + (other.g1_.z * self_.g0_.x)), ((other.g1_.x * self_.g0_.y) - (other.g1_.y * self_.g0_.x) + (other.g1_.z * self_.g1_.w)), (-(other.g0_.x * self_.g0_.x) - (other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z) - (other.g1_.x * self_.g1_.x) - (other.g1_.y * self_.g1_.y) - (other.g1_.z * self_.g1_.z))), /* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.x * self_.g1_.w) + (other.g0_.y * self_.g0_.z) - (other.g0_.z * self_.g0_.y) + (other.g1_.x * self_.g0_.w) + (other.g1_.y * self_.g1_.z) - (other.g1_.z * self_.g1_.y)), (-(other.g0_.x * self_.g0_.z) + (other.g0_.y * self_.g1_.w) + (other.g0_.z * self_.g0_.x) - (other.g1_.x * self_.g1_.z) + (other.g1_.y * self_.g0_.w) + (other.g1_.z * self_.g1_.x)), ((other.g0_.x * self_.g0_.y) - (other.g0_.y * self_.g0_.x) + (other.g0_.z * self_.g1_.w) + (other.g1_.x * self_.g1_.y) - (other.g1_.y * self_.g1_.x) + (other.g1_.z * self_.g0_.w)), (-(other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))));
}
fn motor_sub_line(self: Motor, other: Line) -> Motor {
    let subtraction: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>((-other.g0_.x + self_.g0_.x), (-other.g0_.y + self_.g0_.y), (-other.g0_.z + self_.g0_.z), self_.g0_.w), /* e23, e31, e12, scalar */ vec4<f32>((-other.g1_.x + self_.g1_.x), (-other.g1_.y + self_.g1_.y), (-other.g1_.z + self_.g1_.z), self_.g1_.w));
    return subtraction;
}
fn dualNum_sub_line(self: DualNum, other: Line) -> Motor {
    let subtraction: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g0_.x * -1.0), (other.g0_.y * -1.0), (other.g0_.z * -1.0), self_.g0_.y), /* e23, e31, e12, scalar */ vec4<f32>((other.g1_.x * -1.0), (other.g1_.y * -1.0), (other.g1_.z * -1.0), self_.g0_.x));
    return subtraction;
}
fn multiVector_antiSandwich_flector(self: MultiVector, other: Flector) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>((-(other.g0_.x * self_.g4_.x) - (other.g0_.y * self_.g4_.y) - (other.g0_.z * self_.g4_.z) + (other.g1_.w * self_.g1_.w)), 0.0) - (vec2<f32>(other.g0_.w) * vec2<f32>(self_.g4_.w, self_.g1_.w)) + (vec2<f32>(other.g1_.x) * vec2<f32>(self_.g1_.x, self_.g4_.x)) + (vec2<f32>(other.g1_.y) * vec2<f32>(self_.g1_.y, self_.g4_.y)) + (vec2<f32>(other.g1_.z) * vec2<f32>(self_.g1_.z, self_.g4_.z))), /* e1, e2, e3, e4 */ (vec4<f32>(((self_.g0_.x * other.g1_.x) + (self_.g2_.x * other.g1_.w) + (self_.g2_.y * other.g0_.z) - (self_.g2_.z * other.g0_.y) + (self_.g3_.x * other.g0_.w) + (self_.g3_.y * other.g1_.z)), ((self_.g0_.x * other.g1_.y) - (self_.g2_.x * other.g0_.z) + (self_.g2_.y * other.g1_.w) + (self_.g2_.z * other.g0_.x) + (self_.g3_.y * other.g0_.w) + (self_.g3_.z * other.g1_.x)), ((self_.g0_.x * other.g1_.z) + (self_.g2_.x * other.g0_.y) - (self_.g2_.y * other.g0_.x) + (self_.g2_.z * other.g1_.w) + (self_.g3_.x * other.g1_.y) + (self_.g3_.z * other.g0_.w)), (-(self_.g2_.y * other.g1_.y) - (self_.g2_.z * other.g1_.z))) + (vec4<f32>(self_.g0_.y) * other.g0_) - (vec4<f32>(self_.g3_.z, self_.g3_.x, self_.g3_.y, self_.g2_.x) * other.g1_.yzxx)), /* e41, e42, e43 */ (vec3<f32>(((other.g1_.y * self_.g4_.z) - (other.g1_.z * self_.g4_.y)), (-(other.g1_.x * self_.g4_.z) + (other.g1_.z * self_.g4_.x)), ((other.g1_.x * self_.g4_.y) - (other.g1_.y * self_.g4_.x))) - (vec3<f32>(other.g0_.w) * vec3<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z)) - (vec3<f32>(self_.g1_.w) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z))), /* e23, e31, e12 */ (vec3<f32>((-(other.g0_.y * self_.g4_.z) + (other.g0_.z * self_.g4_.y) + (other.g1_.y * self_.g1_.z) - (other.g1_.z * self_.g1_.y)), ((other.g0_.x * self_.g4_.z) - (other.g0_.z * self_.g4_.x) - (other.g1_.x * self_.g1_.z) + (other.g1_.z * self_.g1_.x)), (-(other.g0_.x * self_.g4_.y) + (other.g0_.y * self_.g4_.x) + (other.g1_.x * self_.g1_.y) - (other.g1_.y * self_.g1_.x))) - (vec3<f32>(other.g0_.w) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z)) + (vec3<f32>(other.g1_.w) * vec3<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z)) + (vec3<f32>(self_.g1_.w) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z)) - (vec3<f32>(self_.g4_.w) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z))), /* e423, e431, e412, e321 */ (vec4<f32>((self_.g2_.y * other.g1_.z), (self_.g2_.z * other.g1_.x), (self_.g2_.x * other.g1_.y), (-(self_.g2_.x * other.g0_.x) - (self_.g2_.y * other.g0_.y) - (self_.g2_.z * other.g0_.z) - (self_.g3_.y * other.g1_.y) - (self_.g3_.z * other.g1_.z))) + (vec4<f32>(self_.g0_.y) * other.g1_) + (vec4<f32>(other.g0_.w) * vec4<f32>(self_.g2_.x, self_.g2_.y, self_.g2_.z, self_.g0_.x)) - (vec4<f32>(self_.g2_.z, self_.g2_.x, self_.g2_.y, self_.g3_.x) * other.g1_.yzxx)));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self_));
}
fn motor_sub_flector(self: Motor, other: Flector) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g1_.w, self_.g0_.w), /* e1, e2, e3, e4 */ (other.g0_ * vec4<f32>(-1.0)), /* e41, e42, e43 */ vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z), /* e23, e31, e12 */ vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z), /* e423, e431, e412, e321 */ (other.g1_ * vec4<f32>(-1.0)));
    return subtraction;
}
fn horizon_mul_motor(self: Horizon, other: Motor) -> Flector {
    return horizon_geometricProduct_horizon(self_, other);
}
fn line_bitXor_point(self: Line, other: Point) -> Plane {
    return line_wedge_line(self_, other);
}
fn point_sub_flector(self: Point, other: Flector) -> Flector {
    let subtraction: Flector = Flector(/* e1, e2, e3, e4 */ (-other.g0_ + self_.g0_), /* e423, e431, e412, e321 */ (other.g1_ * vec4<f32>(-1.0)));
    return subtraction;
}
fn dualNum_mul_point(self: DualNum, other: Point) -> Flector {
    return dualNum_geometricProduct_dualNum(self_, other);
}
fn dualNum_sub_plane(self: DualNum, other: Plane) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ (other.g0_ * vec4<f32>(-1.0)));
    return subtraction;
}
fn multiVector_wedge_origin(self: MultiVector, other: Origin) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (self_.g4_.w * other.g0_ * -1.0)), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.x * other.g0_)), /* e41, e42, e43 */ vec3<f32>((self_.g1_.x * other.g0_ * -1.0), (self_.g1_.y * other.g0_ * -1.0), (self_.g1_.z * other.g0_ * -1.0)), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>((self_.g3_.x * other.g0_), (self_.g3_.y * other.g0_), (self_.g3_.z * other.g0_), 0.0));
}
fn dualNum_wedge_antiScalar(self: DualNum, other: AntiScalar) -> AntiScalar {
    return AntiScalar(/* e1234 */ (self_.g0_.x * other.g0_));
}
fn motor_wedge_point(self: Motor, other: Point) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g1_.w * other.g0_.x), (self_.g1_.w * other.g0_.y), (self_.g1_.w * other.g0_.z), (self_.g1_.w * other.g0_.w)), /* e423, e431, e412, e321 */ vec4<f32>(((self_.g0_.y * other.g0_.z) - (self_.g0_.z * other.g0_.y) + (self_.g1_.x * other.g0_.w)), (-(self_.g0_.x * other.g0_.z) + (self_.g0_.z * other.g0_.x) + (self_.g1_.y * other.g0_.w)), ((self_.g0_.x * other.g0_.y) - (self_.g0_.y * other.g0_.x) + (self_.g1_.z * other.g0_.w)), (-(self_.g1_.x * other.g0_.x) - (self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))));
}
fn line_antiWedge_horizon(self: Line, other: Horizon) -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), 0.0));
}
fn flector_antiWedge_motor(self: Flector, other: Motor) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(((self_.g0_.x * other.g0_.w) - (self_.g1_.y * other.g1_.z) + (self_.g1_.z * other.g1_.y) + (self_.g1_.w * other.g0_.x)), ((self_.g0_.y * other.g0_.w) + (self_.g1_.x * other.g1_.z) - (self_.g1_.z * other.g1_.x) + (self_.g1_.w * other.g0_.y)), ((self_.g0_.z * other.g0_.w) - (self_.g1_.x * other.g1_.y) + (self_.g1_.y * other.g1_.x) + (self_.g1_.w * other.g0_.z)), ((self_.g0_.w * other.g0_.w) - (self_.g1_.x * other.g0_.x) - (self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))), /* e423, e431, e412, e321 */ vec4<f32>((self_.g1_.x * other.g0_.w), (self_.g1_.y * other.g0_.w), (self_.g1_.z * other.g0_.w), (self_.g1_.w * other.g0_.w)));
}
fn antiScalar_geometricQuotient_multiVector(self: AntiScalar, other: MultiVector) -> AntiScalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) - pow(other.g3_.x, 2) - pow(other.g3_.y, 2) - pow(other.g3_.z, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2) - pow(other.g4_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: AntiScalar = AntiScalar(/* e1234 */ (self_.g0_ * inverse.g0_));
    return geometric_product;
}
fn motor_antiWedge_antiScalar(self: Motor, other: AntiScalar) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), (self_.g0_.w * other.g0_)), /* e23, e31, e12, scalar */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), (self_.g1_.w * other.g0_)));
}
fn plane_add_antiScalar(self: Plane, other: AntiScalar) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, other.g0_), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ self_.g0_);
    return addition;
}
fn antiScalar_geometricProduct_point(self: AntiScalar, other: Point) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>((other.g0_.x * self_.g0_ * -1.0), (other.g0_.y * self_.g0_ * -1.0), (other.g0_.z * self_.g0_ * -1.0), 0.0));
}
fn motor_antiWedge_horizon(self: Motor, other: Horizon) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), 0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.w * other.g0_)));
}
fn dualNum_geometricAntiProduct_antiScalar(self: DualNum, other: AntiScalar) -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_)));
}
fn scalar_wedge_antiScalar(self: Scalar, other: AntiScalar) -> AntiScalar {
    return AntiScalar(/* e1234 */ (other.g0_ * self_.g0_));
}
fn origin_add_motor(self: Origin, other: Motor) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(other.g1_.w, other.g0_.w), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_), /* e41, e42, e43 */ vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z), /* e23, e31, e12 */ vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    return addition;
}
fn point_sub_line(self: Point, other: Line) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ self_.g0_, /* e41, e42, e43 */ (other.g0_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (other.g1_ * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    return subtraction;
}
fn point_geometricAntiProduct_plane(self: Point, other: Plane) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g0_.x * self_.g0_.w * -1.0), (other.g0_.y * self_.g0_.w * -1.0), (other.g0_.z * self_.g0_.w * -1.0), 0.0), /* e23, e31, e12, scalar */ vec4<f32>(((other.g0_.y * self_.g0_.z) - (other.g0_.z * self_.g0_.y)), (-(other.g0_.x * self_.g0_.z) + (other.g0_.z * self_.g0_.x)), ((other.g0_.x * self_.g0_.y) - (other.g0_.y * self_.g0_.x)), ((other.g0_.x * self_.g0_.x) + (other.g0_.y * self_.g0_.y) + (other.g0_.z * self_.g0_.z) + (other.g0_.w * self_.g0_.w))));
}
fn multiVector_sub_dualNum(self: MultiVector, other: DualNum) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ (-other.g0_ + self_.g0_), /* e1, e2, e3, e4 */ self_.g1_, /* e41, e42, e43 */ self_.g2_, /* e23, e31, e12 */ self_.g3_, /* e423, e431, e412, e321 */ self_.g4_);
    return subtraction;
}
fn flector_bitXor_multiVector(self: Flector, other: MultiVector) -> MultiVector {
    return flector_wedge_flector(self_, other);
}
fn dualNum_geometricAntiProduct_line(self: DualNum, other: Line) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>((self_.g0_.y * other.g0_.x), (self_.g0_.y * other.g0_.y), (self_.g0_.y * other.g0_.z)), /* e23, e31, e12 */ vec3<f32>(((self_.g0_.x * other.g0_.x) + (self_.g0_.y * other.g1_.x)), ((self_.g0_.x * other.g0_.y) + (self_.g0_.y * other.g1_.y)), ((self_.g0_.x * other.g0_.z) + (self_.g0_.y * other.g1_.z))));
}
fn multiVector_geometricAntiQuotient_motor(self: MultiVector, other: Motor) -> MultiVector {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2) + pow(other.g0_.w, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(anti_inverse.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(anti_inverse.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(anti_inverse.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g4_));
    return geometric_anti_product;
}
fn flector_geometricAntiQuotient_plane(self: Flector, other: Plane) -> Flector {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g1_));
    return geometric_anti_product;
}
fn dualNum_geometricQuotient_plane(self: DualNum, other: Plane) -> DualNum {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.w, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>(inverse.g0_) * self_.g0_));
    return geometric_product;
}
fn origin_add_origin(self: Origin, other: Origin) -> Origin {
    let addition: Origin = Origin(/* e4 */ (other.g0_ + self_.g0_));
    return addition;
}
fn plane_wedge_origin(self: Plane, other: Origin) -> AntiScalar {
    return AntiScalar(/* e1234 */ (self_.g0_.w * other.g0_ * -1.0));
}
fn line_geometricAntiQuotient_flector(self: Line, other: Flector) -> Line {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.w, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Line = Line(/* e41, e42, e43 */ (vec3<f32>(anti_inverse.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(anti_inverse.g0_) * self_.g1_));
    return geometric_anti_product;
}
fn flector_geometricProduct_horizon(self: Flector, other: Horizon) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), (self_.g0_.w * other.g0_)), /* e23, e31, e12, scalar */ vec4<f32>((self_.g0_.x * other.g0_ * -1.0), (self_.g0_.y * other.g0_ * -1.0), (self_.g0_.z * other.g0_ * -1.0), (self_.g1_.w * other.g0_ * -1.0)));
}
fn flector_mul_plane(self: Flector, other: Plane) -> Motor {
    return flector_geometricProduct_flector(self_, other);
}
fn horizon_sandwich_multiVector(self: Horizon, other: MultiVector) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(self_.g0_) * vec2<f32>(other.g4_.w, other.g1_.w) * vec2<f32>(-1.0)), /* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g3_.x, other.g3_.y, other.g3_.z, other.g0_.y)), /* e41, e42, e43 */ (vec3<f32>(self_.g0_) * vec3<f32>(other.g4_.x, other.g4_.y, other.g4_.z) * vec3<f32>(-1.0)), /* e23, e31, e12 */ (vec3<f32>(self_.g0_) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z) * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g2_.x, other.g2_.y, other.g2_.z, other.g0_.x)));
    return multiVector_geometricProduct_multiVector(geometric_product, horizon_reverse(self_));
}
fn plane_sandwich_point(self: Plane, other: Point) -> Flector {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>((self_.g0_.z * other.g0_.y), (self_.g0_.x * other.g0_.z), (self_.g0_.y * other.g0_.x), (-(self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z) - (self_.g0_.w * other.g0_.w))) - (self_.g0_.yzxx * other.g0_.zxyx)), /* e23, e31, e12, scalar */ vec4<f32>((self_.g0_.w * other.g0_.x * -1.0), (self_.g0_.w * other.g0_.y * -1.0), (self_.g0_.w * other.g0_.z * -1.0), 0.0));
    return motor_geometricProduct_motor(geometric_product, plane_reverse(self_));
}
fn antiScalar_antiWedge_multiVector(self: AntiScalar, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_)), /* e1, e2, e3, e4 */ vec4<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_), (other.g1_.w * self_.g0_)), /* e41, e42, e43 */ vec3<f32>((other.g2_.x * self_.g0_), (other.g2_.y * self_.g0_), (other.g2_.z * self_.g0_)), /* e23, e31, e12 */ vec3<f32>((other.g3_.x * self_.g0_), (other.g3_.y * self_.g0_), (other.g3_.z * self_.g0_)), /* e423, e431, e412, e321 */ vec4<f32>((other.g4_.x * self_.g0_), (other.g4_.y * self_.g0_), (other.g4_.z * self_.g0_), (other.g4_.w * self_.g0_)));
}
fn line_bitXor_dualNum(self: Line, other: DualNum) -> Line {
    return line_wedge_line(self_, other);
}
fn point_sandwich_point(self: Point, other: Point) -> Flector {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>(((other.g0_.x * self_.g0_.w) - (other.g0_.w * self_.g0_.x)), ((other.g0_.y * self_.g0_.w) - (other.g0_.w * self_.g0_.y)), ((other.g0_.z * self_.g0_.w) - (other.g0_.w * self_.g0_.z)), 0.0), /* e23, e31, e12, scalar */ (vec4<f32>(((other.g0_.y * self_.g0_.z) * -1.0), ((other.g0_.z * self_.g0_.x) * -1.0), ((other.g0_.x * self_.g0_.y) * -1.0), ((other.g0_.y * self_.g0_.y) + (other.g0_.z * self_.g0_.z))) + (other.g0_.zxyx * self_.g0_.yzxx)));
    return motor_geometricProduct_motor(geometric_product, point_reverse(self_));
}
fn scalar_sub_motor(self: Scalar, other: Motor) -> Motor {
    let subtraction: Motor = Motor(/* e41, e42, e43, e1234 */ (other.g0_ * vec4<f32>(-1.0)), /* e23, e31, e12, scalar */ vec4<f32>((other.g1_.x * -1.0), (other.g1_.y * -1.0), (other.g1_.z * -1.0), (-other.g1_.w + self_.g0_)));
    return subtraction;
}
fn line_antiSandwich_dualNum(self: Line, other: DualNum) -> Motor {
    let geometric_anti_product: Line = Line(/* e41, e42, e43 */ (vec3<f32>(other.g0_.y) * self_.g0_), /* e23, e31, e12 */ ((vec3<f32>(other.g0_.x) * self_.g0_) + (vec3<f32>(other.g0_.y) * self_.g1_)));
    return line_geometricAntiProduct_line(geometric_anti_product, line_antiReverse(self_));
}
fn origin_antiSandwich_flector(self: Origin, other: Flector) -> Flector {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, other.g0_.w) * vec4<f32>(-1.0)), /* e23, e31, e12, scalar */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.w)));
    return motor_geometricAntiProduct_motor(geometric_anti_product, origin_antiReverse(self_));
}
fn motor_geometricAntiProduct_scalar(self: Motor, other: Scalar) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0), /* e23, e31, e12, scalar */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), (self_.g0_.w * other.g0_)));
}
fn flector_mul_line(self: Flector, other: Line) -> Flector {
    return flector_geometricProduct_flector(self_, other);
}
fn line_add_line(self: Line, other: Line) -> Line {
    let addition: Line = Line(/* e41, e42, e43 */ (other.g0_ + self_.g0_), /* e23, e31, e12 */ (other.g1_ + self_.g1_));
    return addition;
}
fn antiScalar_sandwich_flector(self: AntiScalar, other: Flector) -> Flector {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g1_.w * self_.g0_ * -1.0)), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.x * self_.g0_ * -1.0), (other.g0_.y * self_.g0_ * -1.0), (other.g0_.z * self_.g0_ * -1.0), 0.0));
    return flector_geometricProduct_flector(geometric_product, antiScalar_reverse(self_));
}
fn scalar_add_antiScalar(self: Scalar, other: AntiScalar) -> DualNum {
    let addition: DualNum = DualNum(/* scalar, e1234 */ vec2<f32>(self_.g0_, other.g0_));
    return addition;
}
fn plane_geometricAntiProduct_antiScalar(self: Plane, other: AntiScalar) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), (self_.g0_.w * other.g0_)));
}
fn point_add_plane(self: Point, other: Plane) -> Flector {
    let addition: Flector = Flector(/* e1, e2, e3, e4 */ self_.g0_, /* e423, e431, e412, e321 */ other.g0_);
    return addition;
}
fn horizon_sub_plane(self: Horizon, other: Plane) -> Plane {
    let subtraction: Plane = Plane(/* e423, e431, e412, e321 */ vec4<f32>((other.g0_.x * -1.0), (other.g0_.y * -1.0), (other.g0_.z * -1.0), (-other.g0_.w + self_.g0_)));
    return subtraction;
}
fn line_bitXor_scalar(self: Line, other: Scalar) -> Line {
    return line_wedge_line(self_, other);
}
fn flector_geometricQuotient_plane(self: Flector, other: Plane) -> Flector {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.w, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g1_));
    return geometric_product;
}
fn point_geometricProduct_multiVector(self: Point, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(((other.g1_.x * self_.g0_.x) + (other.g1_.y * self_.g0_.y) + (other.g1_.z * self_.g0_.z)), ((other.g4_.x * self_.g0_.x) + (other.g4_.y * self_.g0_.y) + (other.g4_.z * self_.g0_.z) + (other.g4_.w * self_.g0_.w))), /* e1, e2, e3, e4 */ vec4<f32>(((other.g0_.x * self_.g0_.x) + (other.g3_.y * self_.g0_.z) - (other.g3_.z * self_.g0_.y)), ((other.g0_.x * self_.g0_.y) - (other.g3_.x * self_.g0_.z) + (other.g3_.z * self_.g0_.x)), ((other.g0_.x * self_.g0_.z) + (other.g3_.x * self_.g0_.y) - (other.g3_.y * self_.g0_.x)), ((other.g0_.x * self_.g0_.w) - (other.g2_.x * self_.g0_.x) - (other.g2_.y * self_.g0_.y) - (other.g2_.z * self_.g0_.z))), /* e41, e42, e43 */ vec3<f32>(((other.g1_.x * self_.g0_.w) - (other.g1_.w * self_.g0_.x) - (other.g4_.y * self_.g0_.z) + (other.g4_.z * self_.g0_.y)), ((other.g1_.y * self_.g0_.w) - (other.g1_.w * self_.g0_.y) + (other.g4_.x * self_.g0_.z) - (other.g4_.z * self_.g0_.x)), ((other.g1_.z * self_.g0_.w) - (other.g1_.w * self_.g0_.z) - (other.g4_.x * self_.g0_.y) + (other.g4_.y * self_.g0_.x))), /* e23, e31, e12 */ vec3<f32>((-(other.g1_.y * self_.g0_.z) + (other.g1_.z * self_.g0_.y) - (other.g4_.w * self_.g0_.x)), ((other.g1_.x * self_.g0_.z) - (other.g1_.z * self_.g0_.x) - (other.g4_.w * self_.g0_.y)), (-(other.g1_.x * self_.g0_.y) + (other.g1_.y * self_.g0_.x) - (other.g4_.w * self_.g0_.z))), /* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.y * self_.g0_.x) + (other.g2_.y * self_.g0_.z) - (other.g2_.z * self_.g0_.y) + (other.g3_.x * self_.g0_.w)), ((other.g0_.y * self_.g0_.y) - (other.g2_.x * self_.g0_.z) + (other.g2_.z * self_.g0_.x) + (other.g3_.y * self_.g0_.w)), ((other.g0_.y * self_.g0_.z) + (other.g2_.x * self_.g0_.y) - (other.g2_.y * self_.g0_.x) + (other.g3_.z * self_.g0_.w)), (-(other.g3_.x * self_.g0_.x) - (other.g3_.y * self_.g0_.y) - (other.g3_.z * self_.g0_.z))));
}
fn point_add_dualNum(self: Point, other: DualNum) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ other.g0_, /* e1, e2, e3, e4 */ self_.g0_, /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    return addition;
}
fn horizon_sub_line(self: Horizon, other: Line) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ (other.g0_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (other.g1_ * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_));
    return subtraction;
}
fn multiVector_add_multiVector(self: MultiVector, other: MultiVector) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ (other.g0_ + self_.g0_), /* e1, e2, e3, e4 */ (other.g1_ + self_.g1_), /* e41, e42, e43 */ (other.g2_ + self_.g2_), /* e23, e31, e12 */ (other.g3_ + self_.g3_), /* e423, e431, e412, e321 */ (other.g4_ + self_.g4_));
    return addition;
}
fn dualNum_bitXor_multiVector(self: DualNum, other: MultiVector) -> MultiVector {
    return dualNum_wedge_dualNum(self_, other);
}
fn dualNum_geometricAntiProduct_scalar(self: DualNum, other: Scalar) -> Scalar {
    return Scalar(/* scalar */ (self_.g0_.y * other.g0_));
}
fn dualNum_geometricAntiQuotient_multiVector(self: DualNum, other: MultiVector) -> DualNum {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.y, 2) - pow(other.g2_.x, 2) - pow(other.g2_.y, 2) - pow(other.g2_.z, 2) - pow(other.g1_.w, 2) + pow(other.g4_.x, 2) + pow(other.g4_.y, 2) + pow(other.g4_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>(anti_inverse.g0_) * self_.g0_));
    return geometric_anti_product;
}
fn motor_wedge_motor(self: Motor, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(((other.g0_.x * self_.g1_.w) + (other.g1_.w * self_.g0_.x)), ((other.g0_.y * self_.g1_.w) + (other.g1_.w * self_.g0_.y)), ((other.g0_.z * self_.g1_.w) + (other.g1_.w * self_.g0_.z)), (-(other.g0_.x * self_.g1_.x) - (other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) + (other.g0_.w * self_.g1_.w) - (other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z) + (other.g1_.w * self_.g0_.w))), /* e23, e31, e12, scalar */ vec4<f32>(((other.g1_.x * self_.g1_.w) + (other.g1_.w * self_.g1_.x)), ((other.g1_.y * self_.g1_.w) + (other.g1_.w * self_.g1_.y)), ((other.g1_.z * self_.g1_.w) + (other.g1_.w * self_.g1_.z)), (other.g1_.w * self_.g1_.w)));
}
fn dualNum_antiWedge_line(self: DualNum, other: Line) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>((self_.g0_.y * other.g0_.x), (self_.g0_.y * other.g0_.y), (self_.g0_.y * other.g0_.z)), /* e23, e31, e12 */ vec3<f32>((self_.g0_.y * other.g1_.x), (self_.g0_.y * other.g1_.y), (self_.g0_.y * other.g1_.z)));
}
fn scalar_antiWedge_motor(self: Scalar, other: Motor) -> Scalar {
    return Scalar(/* scalar */ (other.g0_.w * self_.g0_));
}
fn motor_geometricQuotient_point(self: Motor, other: Point) -> Motor {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(inverse.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(inverse.g0_) * self_.g1_));
    return geometric_product;
}
fn flector_sub_flector(self: Flector, other: Flector) -> Flector {
    let subtraction: Flector = Flector(/* e1, e2, e3, e4 */ (-other.g0_ + self_.g0_), /* e423, e431, e412, e321 */ (-other.g1_ + self_.g1_));
    return subtraction;
}
fn horizon_mul_multiVector(self: Horizon, other: MultiVector) -> MultiVector {
    return horizon_geometricProduct_horizon(self_, other);
}
fn point_wedge_dualNum(self: Point, other: DualNum) -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g0_.x), (other.g0_.x * self_.g0_.y), (other.g0_.x * self_.g0_.z), (other.g0_.x * self_.g0_.w)));
}
fn dualNum_sandwich_antiScalar(self: DualNum, other: AntiScalar) -> AntiScalar {
    let geometric_product: AntiScalar = AntiScalar(/* e1234 */ (self_.g0_.x * other.g0_));
    return antiScalar_geometricProduct_antiScalar(geometric_product, dualNum_reverse(self_));
}
fn flector_wedge_motor(self: Flector, other: Motor) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g1_.w), (self_.g0_.y * other.g1_.w), (self_.g0_.z * other.g1_.w), (self_.g0_.w * other.g1_.w)), /* e423, e431, e412, e321 */ vec4<f32>((-(self_.g0_.y * other.g0_.z) + (self_.g0_.z * other.g0_.y) + (self_.g0_.w * other.g1_.x) + (self_.g1_.x * other.g1_.w)), ((self_.g0_.x * other.g0_.z) - (self_.g0_.z * other.g0_.x) + (self_.g0_.w * other.g1_.y) + (self_.g1_.y * other.g1_.w)), (-(self_.g0_.x * other.g0_.y) + (self_.g0_.y * other.g0_.x) + (self_.g0_.w * other.g1_.z) + (self_.g1_.z * other.g1_.w)), (-(self_.g0_.x * other.g1_.x) - (self_.g0_.y * other.g1_.y) - (self_.g0_.z * other.g1_.z) + (self_.g1_.w * other.g1_.w))));
}
fn flector_add_line(self: Flector, other: Line) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ self_.g0_, /* e41, e42, e43 */ other.g0_, /* e23, e31, e12 */ other.g1_, /* e423, e431, e412, e321 */ self_.g1_);
    return addition;
}
fn dualNum_wedge_plane(self: DualNum, other: Plane) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.x * other.g0_.x), (self_.g0_.x * other.g0_.y), (self_.g0_.x * other.g0_.z), (self_.g0_.x * other.g0_.w)));
}
fn motor_add_scalar(self: Motor, other: Scalar) -> Motor {
    let addition: Motor = Motor(/* e41, e42, e43, e1234 */ self_.g0_, /* e23, e31, e12, scalar */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, (self_.g1_.w + other.g0_)));
    return addition;
}
fn point_add_line(self: Point, other: Line) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ self_.g0_, /* e41, e42, e43 */ other.g0_, /* e23, e31, e12 */ other.g1_, /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    return addition;
}
fn motor_mul_dualNum(self: Motor, other: DualNum) -> Motor {
    return motor_geometricProduct_motor(self_, other);
}
fn horizon_wedge_dualNum(self: Horizon, other: DualNum) -> Horizon {
    return Horizon(/* e321 */ (other.g0_.x * self_.g0_));
}
fn antiScalar_antiWedge_dualNum(self: AntiScalar, other: DualNum) -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_)));
}
fn line_geometricAntiQuotient_line(self: Line, other: Line) -> Line {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Line = Line(/* e41, e42, e43 */ (vec3<f32>(anti_inverse.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(anti_inverse.g0_) * self_.g1_));
    return geometric_anti_product;
}
fn multiVector_antiWedge_line(self: MultiVector, other: Line) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((-(other.g0_.x * self_.g3_.x) - (other.g0_.y * self_.g3_.y) - (other.g0_.z * self_.g3_.z) - (other.g1_.x * self_.g2_.x) - (other.g1_.y * self_.g2_.y) - (other.g1_.z * self_.g2_.z)), 0.0), /* e1, e2, e3, e4 */ vec4<f32>(((other.g0_.x * self_.g4_.w) + (other.g1_.y * self_.g4_.z) - (other.g1_.z * self_.g4_.y)), ((other.g0_.y * self_.g4_.w) - (other.g1_.x * self_.g4_.z) + (other.g1_.z * self_.g4_.x)), ((other.g0_.z * self_.g4_.w) + (other.g1_.x * self_.g4_.y) - (other.g1_.y * self_.g4_.x)), (-(other.g0_.x * self_.g4_.x) - (other.g0_.y * self_.g4_.y) - (other.g0_.z * self_.g4_.z))), /* e41, e42, e43 */ vec3<f32>((self_.g0_.y * other.g0_.x), (self_.g0_.y * other.g0_.y), (self_.g0_.y * other.g0_.z)), /* e23, e31, e12 */ vec3<f32>((self_.g0_.y * other.g1_.x), (self_.g0_.y * other.g1_.y), (self_.g0_.y * other.g1_.z)), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn plane_geometricAntiQuotient_point(self: Plane, other: Point) -> Plane {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.w, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Plane = Plane(/* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_));
    return geometric_anti_product;
}
fn flector_mul_antiScalar(self: Flector, other: AntiScalar) -> Flector {
    return flector_geometricProduct_flector(self_, other);
}
fn dualNum_antiWedge_dualNum(self: DualNum, other: DualNum) -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>(((other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x)), (other.g0_.y * self_.g0_.y)));
}
fn multiVector_mul_dualNum(self: MultiVector, other: DualNum) -> MultiVector {
    return multiVector_geometricProduct_multiVector(self_, other);
}
fn point_sandwich_motor(self: Point, other: Motor) -> Motor {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>((other.g1_.w * self_.g0_.x), (other.g1_.w * self_.g0_.y), (other.g1_.w * self_.g0_.z), (-(other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z))) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.x) * self_.g0_.yzxx) + (other.g1_.yzxw * self_.g0_.zxyw)), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.y * self_.g0_.z) + (other.g0_.w * self_.g0_.x) + (other.g1_.x * self_.g0_.w)), ((other.g0_.z * self_.g0_.x) + (other.g0_.w * self_.g0_.y) + (other.g1_.y * self_.g0_.w)), ((other.g0_.x * self_.g0_.y) + (other.g0_.w * self_.g0_.z) + (other.g1_.z * self_.g0_.w)), (-(other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))) - (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.y, other.g1_.x) * self_.g0_.yzxx)));
    return flector_geometricProduct_flector(geometric_product, point_reverse(self_));
}
fn motor_geometricProduct_scalar(self: Motor, other: Scalar) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), (self_.g0_.w * other.g0_)), /* e23, e31, e12, scalar */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), (self_.g1_.w * other.g0_)));
}
fn horizon_geometricProduct_horizon(self: Horizon, other: Horizon) -> Scalar {
    return Scalar(/* scalar */ (other.g0_ * self_.g0_ * -1.0));
}
fn multiVector_geometricAntiQuotient_origin(self: MultiVector, other: Origin) -> MultiVector {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(anti_inverse.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(anti_inverse.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(anti_inverse.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g4_));
    return geometric_anti_product;
}
fn dualNum_add_line(self: DualNum, other: Line) -> Motor {
    let addition: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, self_.g0_.y), /* e23, e31, e12, scalar */ vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, self_.g0_.x));
    return addition;
}
fn scalar_sub_antiScalar(self: Scalar, other: AntiScalar) -> DualNum {
    let subtraction: DualNum = DualNum(/* scalar, e1234 */ vec2<f32>(self_.g0_, (other.g0_ * -1.0)));
    return subtraction;
}
fn flector_geometricAntiQuotient_flector(self: Flector, other: Flector) -> Flector {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.w, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g1_));
    return geometric_anti_product;
}
fn antiScalar_geometricProduct_plane(self: AntiScalar, other: Plane) -> Origin {
    return Origin(/* e4 */ (other.g0_.w * self_.g0_ * -1.0));
}
fn plane_mul_point(self: Plane, other: Point) -> Motor {
    return plane_geometricProduct_plane(self_, other);
}
fn dualNum_geometricAntiProduct_horizon(self: DualNum, other: Horizon) -> Horizon {
    return Horizon(/* e321 */ (self_.g0_.y * other.g0_));
}
fn horizon_antiWedge_line(self: Horizon, other: Line) -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), 0.0));
}
fn scalar_antiSandwich_multiVector(self: Scalar, other: MultiVector) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>((other.g0_.y * self_.g0_), 0.0), /* e1, e2, e3, e4 */ vec4<f32>((other.g4_.x * self_.g0_), (other.g4_.y * self_.g0_), (other.g4_.z * self_.g0_), 0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ (vec3<f32>(self_.g0_) * other.g2_), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g1_.w * self_.g0_)));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, scalar_antiReverse(self_));
}
fn multiVector_geometricQuotient_line(self: MultiVector, other: Line) -> MultiVector {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(inverse.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(inverse.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(inverse.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g4_));
    return geometric_product;
}
fn plane_geometricAntiQuotient_line(self: Plane, other: Line) -> Plane {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Plane = Plane(/* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_));
    return geometric_anti_product;
}
fn horizon_sub_origin(self: Horizon, other: Origin) -> Flector {
    let subtraction: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_ * -1.0)), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_));
    return subtraction;
}
fn dualNum_geometricProduct_plane(self: DualNum, other: Plane) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.y * other.g0_.w * -1.0)), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.x * other.g0_.x), (self_.g0_.x * other.g0_.y), (self_.g0_.x * other.g0_.z), (self_.g0_.x * other.g0_.w)));
}
fn point_wedge_motor(self: Point, other: Motor) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g1_.w * self_.g0_.x), (other.g1_.w * self_.g0_.y), (other.g1_.w * self_.g0_.z), (other.g1_.w * self_.g0_.w)), /* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.y * self_.g0_.z) - (other.g0_.z * self_.g0_.y) + (other.g1_.x * self_.g0_.w)), (-(other.g0_.x * self_.g0_.z) + (other.g0_.z * self_.g0_.x) + (other.g1_.y * self_.g0_.w)), ((other.g0_.x * self_.g0_.y) - (other.g0_.y * self_.g0_.x) + (other.g1_.z * self_.g0_.w)), (-(other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))));
}
fn origin_geometricQuotient_flector(self: Origin, other: Flector) -> Origin {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2) - pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Origin = Origin(/* e4 */ (self_.g0_ * inverse.g0_));
    return geometric_product;
}
fn horizon_sub_point(self: Horizon, other: Point) -> Flector {
    let subtraction: Flector = Flector(/* e1, e2, e3, e4 */ (other.g0_ * vec4<f32>(-1.0)), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_));
    return subtraction;
}
fn scalar_geometricQuotient_scalar(self: Scalar, other: Scalar) -> Scalar {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Scalar = Scalar(/* scalar */ (inverse.g0_ * self_.g0_));
    return geometric_product;
}
fn line_geometricProduct_motor(self: Line, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(((self_.g0_.x * other.g1_.w) - (self_.g0_.y * other.g1_.z) + (self_.g0_.z * other.g1_.y) + (self_.g1_.x * other.g0_.w) - (self_.g1_.y * other.g0_.z) + (self_.g1_.z * other.g0_.y)), ((self_.g0_.x * other.g1_.z) + (self_.g0_.y * other.g1_.w) - (self_.g0_.z * other.g1_.x) + (self_.g1_.x * other.g0_.z) + (self_.g1_.y * other.g0_.w) - (self_.g1_.z * other.g0_.x)), (-(self_.g0_.x * other.g1_.y) + (self_.g0_.y * other.g1_.x) + (self_.g0_.z * other.g1_.w) - (self_.g1_.x * other.g0_.y) + (self_.g1_.y * other.g0_.x) + (self_.g1_.z * other.g0_.w)), (-(self_.g0_.x * other.g1_.x) - (self_.g0_.y * other.g1_.y) - (self_.g0_.z * other.g1_.z) - (self_.g1_.x * other.g0_.x) - (self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))), /* e23, e31, e12, scalar */ vec4<f32>(((self_.g1_.x * other.g1_.w) - (self_.g1_.y * other.g1_.z) + (self_.g1_.z * other.g1_.y)), ((self_.g1_.x * other.g1_.z) + (self_.g1_.y * other.g1_.w) - (self_.g1_.z * other.g1_.x)), (-(self_.g1_.x * other.g1_.y) + (self_.g1_.y * other.g1_.x) + (self_.g1_.z * other.g1_.w)), (-(self_.g1_.x * other.g1_.x) - (self_.g1_.y * other.g1_.y) - (self_.g1_.z * other.g1_.z))));
}
fn point_mul_line(self: Point, other: Line) -> Flector {
    return point_geometricProduct_point(self_, other);
}
fn origin_geometricProduct_dualNum(self: Origin, other: DualNum) -> Origin {
    return Origin(/* e4 */ (other.g0_.x * self_.g0_));
}
fn origin_wedge_scalar(self: Origin, other: Scalar) -> Origin {
    return Origin(/* e4 */ (self_.g0_ * other.g0_));
}
fn plane_mul_horizon(self: Plane, other: Horizon) -> Motor {
    return plane_geometricProduct_plane(self_, other);
}
fn horizon_geometricAntiQuotient_dualNum(self: Horizon, other: DualNum) -> Horizon {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_.y, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Horizon = Horizon(/* e321 */ (anti_inverse.g0_ * self_.g0_));
    return geometric_anti_product;
}
fn plane_geometricProduct_plane(self: Plane, other: Plane) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((-(other.g0_.x * self_.g0_.w) + (other.g0_.w * self_.g0_.x)), (-(other.g0_.y * self_.g0_.w) + (other.g0_.w * self_.g0_.y)), (-(other.g0_.z * self_.g0_.w) + (other.g0_.w * self_.g0_.z)), 0.0), /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.w * self_.g0_.w * -1.0)));
}
fn motor_geometricProduct_origin(self: Motor, other: Origin) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g1_.w * other.g0_)), /* e423, e431, e412, e321 */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), 0.0));
}
fn line_sandwich_horizon(self: Line, other: Horizon) -> Flector {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), 0.0), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.x * other.g0_ * -1.0), (self_.g0_.y * other.g0_ * -1.0), (self_.g0_.z * other.g0_ * -1.0), 0.0));
    return flector_geometricProduct_flector(geometric_product, line_reverse(self_));
}
fn dualNum_antiWedge_horizon(self: DualNum, other: Horizon) -> Horizon {
    return Horizon(/* e321 */ (self_.g0_.y * other.g0_));
}
fn antiScalar_wedge_multiVector(self: AntiScalar, other: MultiVector) -> AntiScalar {
    return AntiScalar(/* e1234 */ (other.g0_.x * self_.g0_));
}
fn plane_antiSandwich_scalar(self: Plane, other: Scalar) -> Motor {
    let geometric_anti_product: Point = Point(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g0_ * -1.0), (self_.g0_.y * other.g0_ * -1.0), (self_.g0_.z * other.g0_ * -1.0), 0.0));
    return point_geometricAntiProduct_point(geometric_anti_product, plane_antiReverse(self_));
}
fn flector_geometricAntiQuotient_line(self: Flector, other: Line) -> Flector {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g1_));
    return geometric_anti_product;
}
fn plane_geometricQuotient_line(self: Plane, other: Line) -> Plane {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Plane = Plane(/* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g0_));
    return geometric_product;
}
fn multiVector_geometricAntiQuotient_dualNum(self: MultiVector, other: DualNum) -> MultiVector {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_.y, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(anti_inverse.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(anti_inverse.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(anti_inverse.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g4_));
    return geometric_anti_product;
}
fn origin_geometricProduct_point(self: Origin, other: Point) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_)), /* e23, e31, e12 */ vec3<f32>(0.0));
}
fn motor_geometricQuotient_scalar(self: Motor, other: Scalar) -> Motor {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(inverse.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(inverse.g0_) * self_.g1_));
    return geometric_product;
}
fn scalar_bitXor_motor(self: Scalar, other: Motor) -> Motor {
    return scalar_wedge_scalar(self_, other);
}
fn point_geometricAntiProduct_origin(self: Point, other: Origin) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.w * other.g0_ * -1.0)), /* e23, e31, e12, scalar */ vec4<f32>((self_.g0_.x * other.g0_ * -1.0), (self_.g0_.y * other.g0_ * -1.0), (self_.g0_.z * other.g0_ * -1.0), 0.0));
}
fn flector_bitXor_scalar(self: Flector, other: Scalar) -> Flector {
    return flector_wedge_flector(self_, other);
}
fn point_geometricAntiProduct_scalar(self: Point, other: Scalar) -> Horizon {
    return Horizon(/* e321 */ (self_.g0_.w * other.g0_ * -1.0));
}
fn multiVector_antiWedge_horizon(self: MultiVector, other: Horizon) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((self_.g1_.w * other.g0_), 0.0), /* e1, e2, e3, e4 */ vec4<f32>((self_.g2_.x * other.g0_), (self_.g2_.y * other.g0_), (self_.g2_.z * other.g0_), 0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>((self_.g4_.x * other.g0_), (self_.g4_.y * other.g0_), (self_.g4_.z * other.g0_)), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.y * other.g0_)));
}
fn multiVector_geometricQuotient_multiVector(self: MultiVector, other: MultiVector) -> MultiVector {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) - pow(other.g3_.x, 2) - pow(other.g3_.y, 2) - pow(other.g3_.z, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2) - pow(other.g4_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(inverse.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(inverse.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(inverse.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g4_));
    return geometric_product;
}
fn motor_geometricQuotient_motor(self: Motor, other: Motor) -> Motor {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2) + pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(inverse.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(inverse.g0_) * self_.g1_));
    return geometric_product;
}
fn line_geometricAntiProduct_horizon(self: Line, other: Horizon) -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), 0.0));
}
fn multiVector_antiWedge_plane(self: MultiVector, other: Plane) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(((self_.g1_.x * other.g0_.x) + (self_.g1_.y * other.g0_.y) + (self_.g1_.z * other.g0_.z) + (self_.g1_.w * other.g0_.w)), 0.0), /* e1, e2, e3, e4 */ vec4<f32>(((self_.g2_.x * other.g0_.w) + (self_.g3_.y * other.g0_.z) - (self_.g3_.z * other.g0_.y)), ((self_.g2_.y * other.g0_.w) - (self_.g3_.x * other.g0_.z) + (self_.g3_.z * other.g0_.x)), ((self_.g2_.z * other.g0_.w) + (self_.g3_.x * other.g0_.y) - (self_.g3_.y * other.g0_.x)), (-(self_.g2_.x * other.g0_.x) - (self_.g2_.y * other.g0_.y) - (self_.g2_.z * other.g0_.z))), /* e41, e42, e43 */ vec3<f32>((-(self_.g4_.y * other.g0_.z) + (self_.g4_.z * other.g0_.y)), ((self_.g4_.x * other.g0_.z) - (self_.g4_.z * other.g0_.x)), (-(self_.g4_.x * other.g0_.y) + (self_.g4_.y * other.g0_.x))), /* e23, e31, e12 */ vec3<f32>(((self_.g4_.x * other.g0_.w) - (self_.g4_.w * other.g0_.x)), ((self_.g4_.y * other.g0_.w) - (self_.g4_.w * other.g0_.y)), ((self_.g4_.z * other.g0_.w) - (self_.g4_.w * other.g0_.z))), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.y * other.g0_.x), (self_.g0_.y * other.g0_.y), (self_.g0_.y * other.g0_.z), (self_.g0_.y * other.g0_.w)));
}
fn point_add_flector(self: Point, other: Flector) -> Flector {
    let addition: Flector = Flector(/* e1, e2, e3, e4 */ (other.g0_ + self_.g0_), /* e423, e431, e412, e321 */ other.g1_);
    return addition;
}
fn antiScalar_sub_dualNum(self: AntiScalar, other: DualNum) -> DualNum {
    let subtraction: DualNum = DualNum(/* scalar, e1234 */ vec2<f32>((other.g0_.x * -1.0), (-other.g0_.y + self_.g0_)));
    return subtraction;
}
fn flector_bitXor_horizon(self: Flector, other: Horizon) -> AntiScalar {
    return flector_wedge_flector(self_, other);
}
fn origin_mul_line(self: Origin, other: Line) -> Plane {
    return origin_geometricProduct_origin(self_, other);
}
fn motor_mul_multiVector(self: Motor, other: MultiVector) -> MultiVector {
    return motor_geometricProduct_motor(self_, other);
}
fn scalar_geometricProduct_origin(self: Scalar, other: Origin) -> Origin {
    return Origin(/* e4 */ (other.g0_ * self_.g0_));
}
fn motor_antiSandwich_antiScalar(self: Motor, other: AntiScalar) -> Motor {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(other.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(other.g0_) * self_.g1_));
    return motor_geometricAntiProduct_motor(geometric_anti_product, motor_antiReverse(self_));
}
fn scalar_wedge_line(self: Scalar, other: Line) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_)), /* e23, e31, e12 */ vec3<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_)));
}
fn dualNum_geometricProduct_antiScalar(self: DualNum, other: AntiScalar) -> AntiScalar {
    return AntiScalar(/* e1234 */ (self_.g0_.x * other.g0_));
}
fn horizon_sandwich_origin(self: Horizon, other: Origin) -> Origin {
    let geometric_product: AntiScalar = AntiScalar(/* e1234 */ (self_.g0_ * other.g0_ * -1.0));
    return antiScalar_geometricProduct_antiScalar(geometric_product, horizon_reverse(self_));
}
fn multiVector_antiWedge_motor(self: MultiVector, other: Motor) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(((self_.g0_.x * other.g0_.w) + (self_.g0_.y * other.g1_.w) - (self_.g2_.x * other.g1_.x) - (self_.g2_.y * other.g1_.y) - (self_.g2_.z * other.g1_.z) - (self_.g3_.x * other.g0_.x) - (self_.g3_.y * other.g0_.y) - (self_.g3_.z * other.g0_.z)), (self_.g0_.y * other.g0_.w)), /* e1, e2, e3, e4 */ vec4<f32>(((other.g0_.x * self_.g4_.w) + (other.g0_.w * self_.g1_.x) + (other.g1_.y * self_.g4_.z) - (other.g1_.z * self_.g4_.y)), ((other.g0_.y * self_.g4_.w) + (other.g0_.w * self_.g1_.y) - (other.g1_.x * self_.g4_.z) + (other.g1_.z * self_.g4_.x)), ((other.g0_.z * self_.g4_.w) + (other.g0_.w * self_.g1_.z) + (other.g1_.x * self_.g4_.y) - (other.g1_.y * self_.g4_.x)), (-(other.g0_.x * self_.g4_.x) - (other.g0_.y * self_.g4_.y) - (other.g0_.z * self_.g4_.z) + (other.g0_.w * self_.g1_.w))), /* e41, e42, e43 */ vec3<f32>(((self_.g0_.y * other.g0_.x) + (self_.g2_.x * other.g0_.w)), ((self_.g0_.y * other.g0_.y) + (self_.g2_.y * other.g0_.w)), ((self_.g0_.y * other.g0_.z) + (self_.g2_.z * other.g0_.w))), /* e23, e31, e12 */ vec3<f32>(((self_.g0_.y * other.g1_.x) + (self_.g3_.x * other.g0_.w)), ((self_.g0_.y * other.g1_.y) + (self_.g3_.y * other.g0_.w)), ((self_.g0_.y * other.g1_.z) + (self_.g3_.z * other.g0_.w))), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.w * self_.g4_.x), (other.g0_.w * self_.g4_.y), (other.g0_.w * self_.g4_.z), (other.g0_.w * self_.g4_.w)));
}
fn origin_geometricQuotient_plane(self: Origin, other: Plane) -> Origin {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.w, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Origin = Origin(/* e4 */ (self_.g0_ * inverse.g0_));
    return geometric_product;
}
fn point_add_motor(self: Point, other: Motor) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(other.g1_.w, other.g0_.w), /* e1, e2, e3, e4 */ self_.g0_, /* e41, e42, e43 */ vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z), /* e23, e31, e12 */ vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    return addition;
}
fn line_geometricAntiProduct_origin(self: Line, other: Origin) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), 0.0), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), 0.0));
}
fn plane_geometricProduct_flector(self: Plane, other: Flector) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(((other.g0_.y * self_.g0_.z) - (other.g0_.z * self_.g0_.y) - (other.g1_.x * self_.g0_.w) + (other.g1_.w * self_.g0_.x)), (-(other.g0_.x * self_.g0_.z) + (other.g0_.z * self_.g0_.x) - (other.g1_.y * self_.g0_.w) + (other.g1_.w * self_.g0_.y)), ((other.g0_.x * self_.g0_.y) - (other.g0_.y * self_.g0_.x) - (other.g1_.z * self_.g0_.w) + (other.g1_.w * self_.g0_.z)), (-(other.g0_.x * self_.g0_.x) - (other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z) - (other.g0_.w * self_.g0_.w))), /* e23, e31, e12, scalar */ vec4<f32>((other.g0_.x * self_.g0_.w * -1.0), (other.g0_.y * self_.g0_.w * -1.0), (other.g0_.z * self_.g0_.w * -1.0), (other.g1_.w * self_.g0_.w * -1.0)));
}
fn antiScalar_add_antiScalar(self: AntiScalar, other: AntiScalar) -> AntiScalar {
    let addition: AntiScalar = AntiScalar(/* e1234 */ (other.g0_ + self_.g0_));
    return addition;
}
fn multiVector_antiSandwich_antiScalar(self: MultiVector, other: AntiScalar) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(other.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(other.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(other.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(other.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_) * self_.g4_));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self_));
}
fn origin_bitXor_point(self: Origin, other: Point) -> Line {
    return origin_wedge_origin(self_, other);
}
fn multiVector_geometricAntiQuotient_multiVector(self: MultiVector, other: MultiVector) -> MultiVector {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.y, 2) - pow(other.g2_.x, 2) - pow(other.g2_.y, 2) - pow(other.g2_.z, 2) - pow(other.g1_.w, 2) + pow(other.g4_.x, 2) + pow(other.g4_.y, 2) + pow(other.g4_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(anti_inverse.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(anti_inverse.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(anti_inverse.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g4_));
    return geometric_anti_product;
}
fn scalar_bitXor_dualNum(self: Scalar, other: DualNum) -> DualNum {
    return scalar_wedge_scalar(self_, other);
}
fn plane_geometricQuotient_scalar(self: Plane, other: Scalar) -> Plane {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Plane = Plane(/* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g0_));
    return geometric_product;
}
fn dualNum_bitXor_plane(self: DualNum, other: Plane) -> Plane {
    return dualNum_wedge_dualNum(self_, other);
}
fn multiVector_mul_line(self: MultiVector, other: Line) -> MultiVector {
    return multiVector_geometricProduct_multiVector(self_, other);
}
fn horizon_antiSandwich_motor(self: Horizon, other: Motor) -> Motor {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), 0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.w * self_.g0_)));
    return flector_geometricAntiProduct_flector(geometric_anti_product, horizon_antiReverse(self_));
}
fn line_bitXor_multiVector(self: Line, other: MultiVector) -> MultiVector {
    return line_wedge_line(self_, other);
}
fn flector_geometricQuotient_horizon(self: Flector, other: Horizon) -> Flector {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g1_));
    return geometric_product;
}
fn flector_wedge_line(self: Flector, other: Line) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.y * self_.g0_.z) - (other.g0_.z * self_.g0_.y) + (other.g1_.x * self_.g0_.w)), (-(other.g0_.x * self_.g0_.z) + (other.g0_.z * self_.g0_.x) + (other.g1_.y * self_.g0_.w)), ((other.g0_.x * self_.g0_.y) - (other.g0_.y * self_.g0_.x) + (other.g1_.z * self_.g0_.w)), (-(other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))));
}
fn antiScalar_geometricAntiProduct_line(self: AntiScalar, other: Line) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_)), /* e23, e31, e12 */ vec3<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_)));
}
fn horizon_sub_dualNum(self: Horizon, other: DualNum) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ (other.g0_ * vec2<f32>(-1.0)), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_));
    return subtraction;
}
fn flector_sub_point(self: Flector, other: Point) -> Flector {
    let subtraction: Flector = Flector(/* e1, e2, e3, e4 */ (self_.g0_ - other.g0_), /* e423, e431, e412, e321 */ self_.g1_);
    return subtraction;
}
fn dualNum_sandwich_flector(self: DualNum, other: Flector) -> Flector {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g0_.x), (self_.g0_.x * other.g0_.y), (self_.g0_.x * other.g0_.z), ((self_.g0_.x * other.g0_.w) - (self_.g0_.y * other.g1_.w))), /* e423, e431, e412, e321 */ vec4<f32>(((self_.g0_.x * other.g1_.x) - (self_.g0_.y * other.g0_.x)), ((self_.g0_.x * other.g1_.y) - (self_.g0_.y * other.g0_.y)), ((self_.g0_.x * other.g1_.z) - (self_.g0_.y * other.g0_.z)), (self_.g0_.x * other.g1_.w)));
    return flector_geometricProduct_flector(geometric_product, dualNum_reverse(self_));
}
fn line_antiSandwich_origin(self: Line, other: Origin) -> Flector {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), 0.0), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), 0.0));
    return flector_geometricAntiProduct_flector(geometric_anti_product, line_antiReverse(self_));
}
fn horizon_mul_flector(self: Horizon, other: Flector) -> Motor {
    return horizon_geometricProduct_horizon(self_, other);
}
fn antiScalar_geometricAntiQuotient_flector(self: AntiScalar, other: Flector) -> AntiScalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.w, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: AntiScalar = AntiScalar(/* e1234 */ (anti_inverse.g0_ * self_.g0_));
    return geometric_anti_product;
}
fn line_geometricAntiQuotient_multiVector(self: Line, other: MultiVector) -> Line {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.y, 2) - pow(other.g2_.x, 2) - pow(other.g2_.y, 2) - pow(other.g2_.z, 2) - pow(other.g1_.w, 2) + pow(other.g4_.x, 2) + pow(other.g4_.y, 2) + pow(other.g4_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Line = Line(/* e41, e42, e43 */ (vec3<f32>(anti_inverse.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(anti_inverse.g0_) * self_.g1_));
    return geometric_anti_product;
}
fn origin_mul_point(self: Origin, other: Point) -> Line {
    return origin_geometricProduct_origin(self_, other);
}
fn origin_geometricQuotient_dualNum(self: Origin, other: DualNum) -> Origin {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_.x, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Origin = Origin(/* e4 */ (self_.g0_ * inverse.g0_));
    return geometric_product;
}
fn multiVector_wedge_scalar(self: MultiVector, other: Scalar) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_)), /* e1, e2, e3, e4 */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), (self_.g1_.w * other.g0_)), /* e41, e42, e43 */ vec3<f32>((self_.g2_.x * other.g0_), (self_.g2_.y * other.g0_), (self_.g2_.z * other.g0_)), /* e23, e31, e12 */ vec3<f32>((self_.g3_.x * other.g0_), (self_.g3_.y * other.g0_), (self_.g3_.z * other.g0_)), /* e423, e431, e412, e321 */ vec4<f32>((self_.g4_.x * other.g0_), (self_.g4_.y * other.g0_), (self_.g4_.z * other.g0_), (self_.g4_.w * other.g0_)));
}
fn antiScalar_geometricQuotient_point(self: AntiScalar, other: Point) -> AntiScalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: AntiScalar = AntiScalar(/* e1234 */ (self_.g0_ * inverse.g0_));
    return geometric_product;
}
fn motor_geometricProduct_line(self: Motor, other: Line) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(((other.g0_.x * self_.g1_.w) + (other.g0_.y * self_.g1_.z) - (other.g0_.z * self_.g1_.y) + (other.g1_.x * self_.g0_.w) + (other.g1_.y * self_.g0_.z) - (other.g1_.z * self_.g0_.y)), (-(other.g0_.x * self_.g1_.z) + (other.g0_.y * self_.g1_.w) + (other.g0_.z * self_.g1_.x) - (other.g1_.x * self_.g0_.z) + (other.g1_.y * self_.g0_.w) + (other.g1_.z * self_.g0_.x)), ((other.g0_.x * self_.g1_.y) - (other.g0_.y * self_.g1_.x) + (other.g0_.z * self_.g1_.w) + (other.g1_.x * self_.g0_.y) - (other.g1_.y * self_.g0_.x) + (other.g1_.z * self_.g0_.w)), (-(other.g0_.x * self_.g1_.x) - (other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))), /* e23, e31, e12, scalar */ vec4<f32>(((other.g1_.x * self_.g1_.w) + (other.g1_.y * self_.g1_.z) - (other.g1_.z * self_.g1_.y)), (-(other.g1_.x * self_.g1_.z) + (other.g1_.y * self_.g1_.w) + (other.g1_.z * self_.g1_.x)), ((other.g1_.x * self_.g1_.y) - (other.g1_.y * self_.g1_.x) + (other.g1_.z * self_.g1_.w)), (-(other.g1_.x * self_.g1_.x) - (other.g1_.y * self_.g1_.y) - (other.g1_.z * self_.g1_.z))));
}
fn plane_sub_line(self: Plane, other: Line) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ (other.g0_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (other.g1_ * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ self_.g0_);
    return subtraction;
}
fn plane_geometricAntiProduct_motor(self: Plane, other: Motor) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(((other.g0_.x * self_.g0_.w) + (other.g1_.y * self_.g0_.z) - (other.g1_.z * self_.g0_.y) - (other.g1_.w * self_.g0_.x)), ((other.g0_.y * self_.g0_.w) - (other.g1_.x * self_.g0_.z) + (other.g1_.z * self_.g0_.x) - (other.g1_.w * self_.g0_.y)), ((other.g0_.z * self_.g0_.w) + (other.g1_.x * self_.g0_.y) - (other.g1_.y * self_.g0_.x) - (other.g1_.w * self_.g0_.z)), (-(other.g0_.x * self_.g0_.x) - (other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z))), /* e423, e431, e412, e321 */ vec4<f32>((-(other.g0_.y * self_.g0_.z) + (other.g0_.z * self_.g0_.y) + (other.g0_.w * self_.g0_.x)), ((other.g0_.x * self_.g0_.z) - (other.g0_.z * self_.g0_.x) + (other.g0_.w * self_.g0_.y)), (-(other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x) + (other.g0_.w * self_.g0_.z)), ((other.g0_.w * self_.g0_.w) + (other.g1_.x * self_.g0_.x) + (other.g1_.y * self_.g0_.y) + (other.g1_.z * self_.g0_.z))));
}
fn multiVector_mul_flector(self: MultiVector, other: Flector) -> MultiVector {
    return multiVector_geometricProduct_multiVector(self_, other);
}
fn point_mul_dualNum(self: Point, other: DualNum) -> Flector {
    return point_geometricProduct_point(self_, other);
}
fn motor_sandwich_scalar(self: Motor, other: Scalar) -> Motor {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(other.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(other.g0_) * self_.g1_));
    return motor_geometricProduct_motor(geometric_product, motor_reverse(self_));
}
fn motor_wedge_flector(self: Motor, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g1_.w), (other.g0_.y * self_.g1_.w), (other.g0_.z * self_.g1_.w), (other.g0_.w * self_.g1_.w)), /* e423, e431, e412, e321 */ vec4<f32>((-(other.g0_.y * self_.g0_.z) + (other.g0_.z * self_.g0_.y) + (other.g0_.w * self_.g1_.x) + (other.g1_.x * self_.g1_.w)), ((other.g0_.x * self_.g0_.z) - (other.g0_.z * self_.g0_.x) + (other.g0_.w * self_.g1_.y) + (other.g1_.y * self_.g1_.w)), (-(other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x) + (other.g0_.w * self_.g1_.z) + (other.g1_.z * self_.g1_.w)), (-(other.g0_.x * self_.g1_.x) - (other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) + (other.g1_.w * self_.g1_.w))));
}
fn plane_sub_horizon(self: Plane, other: Horizon) -> Plane {
    let subtraction: Plane = Plane(/* e423, e431, e412, e321 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, (self_.g0_.w - other.g0_)));
    return subtraction;
}
fn motor_bitXor_multiVector(self: Motor, other: MultiVector) -> MultiVector {
    return motor_wedge_motor(self_, other);
}
fn origin_geometricQuotient_point(self: Origin, other: Point) -> Origin {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Origin = Origin(/* e4 */ (self_.g0_ * inverse.g0_));
    return geometric_product;
}
fn origin_sub_horizon(self: Origin, other: Horizon) -> Flector {
    let subtraction: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_ * -1.0)));
    return subtraction;
}
fn horizon_mul_origin(self: Horizon, other: Origin) -> AntiScalar {
    return horizon_geometricProduct_horizon(self_, other);
}
fn origin_geometricAntiProduct_horizon(self: Origin, other: Horizon) -> Scalar {
    return Scalar(/* scalar */ (other.g0_ * self_.g0_));
}
fn point_geometricAntiProduct_point(self: Point, other: Point) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.w * self_.g0_.w * -1.0)), /* e23, e31, e12, scalar */ vec4<f32>(((other.g0_.x * self_.g0_.w) - (other.g0_.w * self_.g0_.x)), ((other.g0_.y * self_.g0_.w) - (other.g0_.w * self_.g0_.y)), ((other.g0_.z * self_.g0_.w) - (other.g0_.w * self_.g0_.z)), 0.0));
}
fn scalar_geometricQuotient_dualNum(self: Scalar, other: DualNum) -> Scalar {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_.x, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Scalar = Scalar(/* scalar */ (inverse.g0_ * self_.g0_));
    return geometric_product;
}
fn plane_sub_flector(self: Plane, other: Flector) -> Flector {
    let subtraction: Flector = Flector(/* e1, e2, e3, e4 */ (other.g0_ * vec4<f32>(-1.0)), /* e423, e431, e412, e321 */ (-other.g1_ + self_.g0_));
    return subtraction;
}
fn antiScalar_geometricAntiProduct_plane(self: AntiScalar, other: Plane) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), (other.g0_.w * self_.g0_)));
}
fn origin_add_plane(self: Origin, other: Plane) -> Flector {
    let addition: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_), /* e423, e431, e412, e321 */ other.g0_);
    return addition;
}
fn flector_sandwich_antiScalar(self: Flector, other: AntiScalar) -> Motor {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g1_.w * other.g0_)), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), 0.0));
    return flector_geometricProduct_flector(geometric_product, flector_reverse(self_));
}
fn dualNum_sandwich_motor(self: DualNum, other: Motor) -> Motor {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ ((vec4<f32>(self_.g0_.x) * other.g0_) + (vec4<f32>(self_.g0_.y) * other.g1_)), /* e23, e31, e12, scalar */ (vec4<f32>(self_.g0_.x) * other.g1_));
    return motor_geometricProduct_motor(geometric_product, dualNum_reverse(self_));
}
fn origin_antiSandwich_antiScalar(self: Origin, other: AntiScalar) -> AntiScalar {
    let geometric_anti_product: Origin = Origin(/* e4 */ (other.g0_ * self_.g0_));
    return origin_geometricAntiProduct_origin(geometric_anti_product, origin_antiReverse(self_));
}
fn multiVector_geometricProduct_motor(self: MultiVector, other: Motor) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(((self_.g0_.x * other.g1_.w) - (self_.g3_.x * other.g1_.x) - (self_.g3_.y * other.g1_.y) - (self_.g3_.z * other.g1_.z)), ((self_.g0_.x * other.g0_.w) + (self_.g0_.y * other.g1_.w) - (self_.g2_.x * other.g1_.x) - (self_.g2_.y * other.g1_.y) - (self_.g2_.z * other.g1_.z) - (self_.g3_.x * other.g0_.x) - (self_.g3_.y * other.g0_.y) - (self_.g3_.z * other.g0_.z))), /* e1, e2, e3, e4 */ vec4<f32>(((other.g1_.x * self_.g4_.w) + (other.g1_.y * self_.g1_.z) - (other.g1_.z * self_.g1_.y) + (other.g1_.w * self_.g1_.x)), (-(other.g1_.x * self_.g1_.z) + (other.g1_.y * self_.g4_.w) + (other.g1_.z * self_.g1_.x) + (other.g1_.w * self_.g1_.y)), ((other.g1_.x * self_.g1_.y) - (other.g1_.y * self_.g1_.x) + (other.g1_.z * self_.g4_.w) + (other.g1_.w * self_.g1_.z)), (-(other.g0_.x * self_.g1_.x) - (other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) + (other.g0_.w * self_.g4_.w) - (other.g1_.x * self_.g4_.x) - (other.g1_.y * self_.g4_.y) - (other.g1_.z * self_.g4_.z) + (other.g1_.w * self_.g1_.w))), /* e41, e42, e43 */ vec3<f32>(((self_.g0_.x * other.g0_.x) + (self_.g0_.y * other.g1_.x) + (self_.g2_.x * other.g1_.w) - (self_.g2_.y * other.g1_.z) + (self_.g2_.z * other.g1_.y) + (self_.g3_.x * other.g0_.w) - (self_.g3_.y * other.g0_.z) + (self_.g3_.z * other.g0_.y)), ((self_.g0_.x * other.g0_.y) + (self_.g0_.y * other.g1_.y) + (self_.g2_.x * other.g1_.z) + (self_.g2_.y * other.g1_.w) - (self_.g2_.z * other.g1_.x) + (self_.g3_.x * other.g0_.z) + (self_.g3_.y * other.g0_.w) - (self_.g3_.z * other.g0_.x)), ((self_.g0_.x * other.g0_.z) + (self_.g0_.y * other.g1_.z) - (self_.g2_.x * other.g1_.y) + (self_.g2_.y * other.g1_.x) + (self_.g2_.z * other.g1_.w) - (self_.g3_.x * other.g0_.y) + (self_.g3_.y * other.g0_.x) + (self_.g3_.z * other.g0_.w))), /* e23, e31, e12 */ vec3<f32>(((self_.g0_.x * other.g1_.x) + (self_.g3_.x * other.g1_.w) - (self_.g3_.y * other.g1_.z) + (self_.g3_.z * other.g1_.y)), ((self_.g0_.x * other.g1_.y) + (self_.g3_.x * other.g1_.z) + (self_.g3_.y * other.g1_.w) - (self_.g3_.z * other.g1_.x)), ((self_.g0_.x * other.g1_.z) - (self_.g3_.x * other.g1_.y) + (self_.g3_.y * other.g1_.x) + (self_.g3_.z * other.g1_.w))), /* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.x * self_.g4_.w) + (other.g0_.y * self_.g1_.z) - (other.g0_.z * self_.g1_.y) + (other.g0_.w * self_.g1_.x) + (other.g1_.x * self_.g1_.w) + (other.g1_.y * self_.g4_.z) - (other.g1_.z * self_.g4_.y) + (other.g1_.w * self_.g4_.x)), (-(other.g0_.x * self_.g1_.z) + (other.g0_.y * self_.g4_.w) + (other.g0_.z * self_.g1_.x) + (other.g0_.w * self_.g1_.y) - (other.g1_.x * self_.g4_.z) + (other.g1_.y * self_.g1_.w) + (other.g1_.z * self_.g4_.x) + (other.g1_.w * self_.g4_.y)), ((other.g0_.x * self_.g1_.y) - (other.g0_.y * self_.g1_.x) + (other.g0_.z * self_.g4_.w) + (other.g0_.w * self_.g1_.z) + (other.g1_.x * self_.g4_.y) - (other.g1_.y * self_.g4_.x) + (other.g1_.z * self_.g1_.w) + (other.g1_.w * self_.g4_.z)), (-(other.g1_.x * self_.g1_.x) - (other.g1_.y * self_.g1_.y) - (other.g1_.z * self_.g1_.z) + (other.g1_.w * self_.g4_.w))));
}
fn horizon_geometricAntiProduct_flector(self: Horizon, other: Flector) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0), /* e23, e31, e12, scalar */ vec4<f32>((other.g1_.x * self_.g0_ * -1.0), (other.g1_.y * self_.g0_ * -1.0), (other.g1_.z * self_.g0_ * -1.0), (other.g0_.w * self_.g0_ * -1.0)));
}
fn origin_geometricQuotient_line(self: Origin, other: Line) -> Origin {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Origin = Origin(/* e4 */ (self_.g0_ * inverse.g0_));
    return geometric_product;
}
fn line_sub_plane(self: Line, other: Plane) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ self_.g0_, /* e23, e31, e12 */ self_.g1_, /* e423, e431, e412, e321 */ (other.g0_ * vec4<f32>(-1.0)));
    return subtraction;
}
fn motor_bitXor_line(self: Motor, other: Line) -> Motor {
    return motor_wedge_motor(self_, other);
}
fn motor_sub_origin(self: Motor, other: Origin) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g1_.w, self_.g0_.w), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_ * -1.0)), /* e41, e42, e43 */ vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z), /* e23, e31, e12 */ vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    return subtraction;
}
fn antiScalar_mul_dualNum(self: AntiScalar, other: DualNum) -> AntiScalar {
    return antiScalar_geometricProduct_antiScalar(self_, other);
}
fn horizon_sandwich_horizon(self: Horizon, other: Horizon) -> Horizon {
    let geometric_product: Scalar = Scalar(/* scalar */ (other.g0_ * self_.g0_ * -1.0));
    return scalar_geometricProduct_scalar(geometric_product, horizon_reverse(self_));
}
fn horizon_add_dualNum(self: Horizon, other: DualNum) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ other.g0_, /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_));
    return addition;
}
fn flector_geometricProduct_point(self: Flector, other: Point) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((-(self_.g0_.x * other.g0_.w) + (self_.g0_.w * other.g0_.x) - (self_.g1_.y * other.g0_.z) + (self_.g1_.z * other.g0_.y)), (-(self_.g0_.y * other.g0_.w) + (self_.g0_.w * other.g0_.y) + (self_.g1_.x * other.g0_.z) - (self_.g1_.z * other.g0_.x)), (-(self_.g0_.z * other.g0_.w) + (self_.g0_.w * other.g0_.z) - (self_.g1_.x * other.g0_.y) + (self_.g1_.y * other.g0_.x)), (-(self_.g1_.x * other.g0_.x) - (self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z) - (self_.g1_.w * other.g0_.w))), /* e23, e31, e12, scalar */ vec4<f32>(((self_.g0_.y * other.g0_.z) - (self_.g0_.z * other.g0_.y) - (self_.g1_.w * other.g0_.x)), (-(self_.g0_.x * other.g0_.z) + (self_.g0_.z * other.g0_.x) - (self_.g1_.w * other.g0_.y)), ((self_.g0_.x * other.g0_.y) - (self_.g0_.y * other.g0_.x) - (self_.g1_.w * other.g0_.z)), ((self_.g0_.x * other.g0_.x) + (self_.g0_.y * other.g0_.y) + (self_.g0_.z * other.g0_.z))));
}
fn line_geometricProduct_scalar(self: Line, other: Scalar) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_)), /* e23, e31, e12 */ vec3<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_)));
}
fn motor_add_horizon(self: Motor, other: Horizon) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g1_.w, self_.g0_.w), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z), /* e23, e31, e12 */ vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other.g0_));
    return addition;
}
fn horizon_antiWedge_antiScalar(self: Horizon, other: AntiScalar) -> Horizon {
    return Horizon(/* e321 */ (other.g0_ * self_.g0_));
}
fn motor_geometricAntiQuotient_origin(self: Motor, other: Origin) -> Motor {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(anti_inverse.g0_) * self_.g1_));
    return geometric_anti_product;
}
fn line_mul_multiVector(self: Line, other: MultiVector) -> MultiVector {
    return line_geometricProduct_line(self_, other);
}
fn multiVector_bitXor_flector(self: MultiVector, other: Flector) -> MultiVector {
    return multiVector_wedge_multiVector(self_, other);
}
fn multiVector_geometricAntiQuotient_line(self: MultiVector, other: Line) -> MultiVector {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(anti_inverse.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(anti_inverse.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(anti_inverse.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g4_));
    return geometric_anti_product;
}
fn horizon_bitXor_origin(self: Horizon, other: Origin) -> AntiScalar {
    return horizon_wedge_horizon(self_, other);
}
fn multiVector_sub_scalar(self: MultiVector, other: Scalar) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>((self_.g0_.x - other.g0_), self_.g0_.y), /* e1, e2, e3, e4 */ self_.g1_, /* e41, e42, e43 */ self_.g2_, /* e23, e31, e12 */ self_.g3_, /* e423, e431, e412, e321 */ self_.g4_);
    return subtraction;
}
fn motor_sub_plane(self: Motor, other: Plane) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g1_.w, self_.g0_.w), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z), /* e23, e31, e12 */ vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z), /* e423, e431, e412, e321 */ (other.g0_ * vec4<f32>(-1.0)));
    return subtraction;
}
fn motor_geometricProduct_horizon(self: Motor, other: Horizon) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), (self_.g0_.w * other.g0_ * -1.0)), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.x * other.g0_ * -1.0), (self_.g0_.y * other.g0_ * -1.0), (self_.g0_.z * other.g0_ * -1.0), (self_.g1_.w * other.g0_)));
}
fn flector_antiWedge_origin(self: Flector, other: Origin) -> Scalar {
    return Scalar(/* scalar */ (self_.g1_.w * other.g0_ * -1.0));
}
fn antiScalar_antiSandwich_flector(self: AntiScalar, other: Flector) -> Flector {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_) * other.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_) * other.g1_));
    return flector_geometricAntiProduct_flector(geometric_anti_product, antiScalar_antiReverse(self_));
}
fn scalar_add_dualNum(self: Scalar, other: DualNum) -> DualNum {
    let addition: DualNum = DualNum(/* scalar, e1234 */ vec2<f32>((other.g0_.x + self_.g0_), other.g0_.y));
    return addition;
}
fn antiScalar_sub_motor(self: AntiScalar, other: Motor) -> Motor {
    let subtraction: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g0_.x * -1.0), (other.g0_.y * -1.0), (other.g0_.z * -1.0), (-other.g0_.w + self_.g0_)), /* e23, e31, e12, scalar */ (other.g1_ * vec4<f32>(-1.0)));
    return subtraction;
}
fn plane_antiWedge_horizon(self: Plane, other: Horizon) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_)));
}
fn antiScalar_geometricAntiProduct_origin(self: AntiScalar, other: Origin) -> Origin {
    return Origin(/* e4 */ (self_.g0_ * other.g0_));
}
fn dualNum_mul_motor(self: DualNum, other: Motor) -> Motor {
    return dualNum_geometricProduct_dualNum(self_, other);
}
fn flector_add_scalar(self: Flector, other: Scalar) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(other.g0_, 0.0), /* e1, e2, e3, e4 */ self_.g0_, /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ self_.g1_);
    return addition;
}
fn motor_wedge_antiScalar(self: Motor, other: AntiScalar) -> AntiScalar {
    return AntiScalar(/* e1234 */ (self_.g1_.w * other.g0_));
}
fn antiScalar_wedge_scalar(self: AntiScalar, other: Scalar) -> AntiScalar {
    return AntiScalar(/* e1234 */ (self_.g0_ * other.g0_));
}
fn horizon_bitXor_point(self: Horizon, other: Point) -> AntiScalar {
    return horizon_wedge_horizon(self_, other);
}
fn multiVector_wedge_motor(self: MultiVector, other: Motor) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((self_.g0_.x * other.g1_.w), ((self_.g0_.x * other.g0_.w) + (self_.g0_.y * other.g1_.w) - (self_.g2_.x * other.g1_.x) - (self_.g2_.y * other.g1_.y) - (self_.g2_.z * other.g1_.z) - (self_.g3_.x * other.g0_.x) - (self_.g3_.y * other.g0_.y) - (self_.g3_.z * other.g0_.z))), /* e1, e2, e3, e4 */ vec4<f32>((other.g1_.w * self_.g1_.x), (other.g1_.w * self_.g1_.y), (other.g1_.w * self_.g1_.z), (other.g1_.w * self_.g1_.w)), /* e41, e42, e43 */ vec3<f32>(((self_.g0_.x * other.g0_.x) + (self_.g2_.x * other.g1_.w)), ((self_.g0_.x * other.g0_.y) + (self_.g2_.y * other.g1_.w)), ((self_.g0_.x * other.g0_.z) + (self_.g2_.z * other.g1_.w))), /* e23, e31, e12 */ vec3<f32>(((self_.g0_.x * other.g1_.x) + (self_.g3_.x * other.g1_.w)), ((self_.g0_.x * other.g1_.y) + (self_.g3_.y * other.g1_.w)), ((self_.g0_.x * other.g1_.z) + (self_.g3_.z * other.g1_.w))), /* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.y * self_.g1_.z) - (other.g0_.z * self_.g1_.y) + (other.g1_.x * self_.g1_.w) + (other.g1_.w * self_.g4_.x)), (-(other.g0_.x * self_.g1_.z) + (other.g0_.z * self_.g1_.x) + (other.g1_.y * self_.g1_.w) + (other.g1_.w * self_.g4_.y)), ((other.g0_.x * self_.g1_.y) - (other.g0_.y * self_.g1_.x) + (other.g1_.z * self_.g1_.w) + (other.g1_.w * self_.g4_.z)), (-(other.g1_.x * self_.g1_.x) - (other.g1_.y * self_.g1_.y) - (other.g1_.z * self_.g1_.z) + (other.g1_.w * self_.g4_.w))));
}
fn multiVector_antiSandwich_line(self: MultiVector, other: Line) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>((-(other.g1_.x * self_.g2_.x) - (other.g1_.y * self_.g2_.y) - (other.g1_.z * self_.g2_.z)), 0.0) - (vec2<f32>(other.g0_.x) * vec2<f32>(self_.g3_.x, self_.g2_.x)) - (vec2<f32>(other.g0_.y) * vec2<f32>(self_.g3_.y, self_.g2_.y)) - (vec2<f32>(other.g0_.z) * vec2<f32>(self_.g3_.z, self_.g2_.z))), /* e1, e2, e3, e4 */ (vec4<f32>(((other.g0_.x * self_.g4_.w) - (other.g0_.y * self_.g1_.z) + (other.g0_.z * self_.g1_.y) - (other.g1_.x * self_.g1_.w) + (other.g1_.y * self_.g4_.z)), ((other.g0_.x * self_.g1_.z) + (other.g0_.y * self_.g4_.w) - (other.g0_.z * self_.g1_.x) - (other.g1_.y * self_.g1_.w) + (other.g1_.z * self_.g4_.x)), (-(other.g0_.x * self_.g1_.y) + (other.g0_.y * self_.g1_.x) + (other.g0_.z * self_.g4_.w) + (other.g1_.x * self_.g4_.y) - (other.g1_.z * self_.g1_.w)), (-(other.g0_.y * self_.g4_.y) - (other.g0_.z * self_.g4_.z))) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.x) * self_.g4_.yzxx)), /* e41, e42, e43 */ ((vec3<f32>(self_.g0_.y) * other.g0_) - (other.g0_.yzx * self_.g2_.zxy) + (other.g0_.zxy * self_.g2_.yzx)), /* e23, e31, e12 */ ((vec3<f32>(self_.g0_.x) * other.g0_) + (vec3<f32>(self_.g0_.y) * other.g1_) - (other.g0_.yzx * self_.g3_.zxy) + (other.g0_.zxy * self_.g3_.yzx) - (other.g1_.yzx * self_.g2_.zxy) + (other.g1_.zxy * self_.g2_.yzx)), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.x * self_.g1_.w) - (other.g0_.y * self_.g4_.z)), ((other.g0_.y * self_.g1_.w) - (other.g0_.z * self_.g4_.x)), (-(other.g0_.x * self_.g4_.y) + (other.g0_.z * self_.g1_.w)), (-(other.g0_.x * self_.g1_.x) - (other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) + (other.g1_.y * self_.g4_.y) + (other.g1_.z * self_.g4_.z))) + (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.y, other.g1_.x) * self_.g4_.yzxx)));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self_));
}
fn motor_geometricQuotient_horizon(self: Motor, other: Horizon) -> Motor {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(inverse.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(inverse.g0_) * self_.g1_));
    return geometric_product;
}
fn antiScalar_geometricQuotient_line(self: AntiScalar, other: Line) -> AntiScalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: AntiScalar = AntiScalar(/* e1234 */ (self_.g0_ * inverse.g0_));
    return geometric_product;
}
fn antiScalar_bitXor_multiVector(self: AntiScalar, other: MultiVector) -> AntiScalar {
    return antiScalar_wedge_antiScalar(self_, other);
}
fn dualNum_bitXor_antiScalar(self: DualNum, other: AntiScalar) -> AntiScalar {
    return dualNum_wedge_dualNum(self_, other);
}
fn dualNum_mul_flector(self: DualNum, other: Flector) -> Flector {
    return dualNum_geometricProduct_dualNum(self_, other);
}
fn scalar_wedge_flector(self: Scalar, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), (other.g0_.w * self_.g0_)), /* e423, e431, e412, e321 */ vec4<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_), (other.g1_.w * self_.g0_)));
}
fn origin_antiSandwich_dualNum(self: Origin, other: DualNum) -> Motor {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.y * self_.g0_)), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.x * self_.g0_ * -1.0)));
    return flector_geometricAntiProduct_flector(geometric_anti_product, origin_antiReverse(self_));
}
fn point_sub_origin(self: Point, other: Origin) -> Point {
    let subtraction: Point = Point(/* e1, e2, e3, e4 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, (self_.g0_.w - other.g0_)));
    return subtraction;
}
fn antiScalar_antiWedge_origin(self: AntiScalar, other: Origin) -> Origin {
    return Origin(/* e4 */ (self_.g0_ * other.g0_));
}
fn horizon_geometricProduct_point(self: Horizon, other: Point) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.w * self_.g0_ * -1.0)), /* e23, e31, e12, scalar */ vec4<f32>((other.g0_.x * self_.g0_ * -1.0), (other.g0_.y * self_.g0_ * -1.0), (other.g0_.z * self_.g0_ * -1.0), 0.0));
}
fn origin_antiSandwich_line(self: Origin, other: Line) -> Motor {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g1_.x * self_.g0_ * -1.0), (other.g1_.y * self_.g0_ * -1.0), (other.g1_.z * self_.g0_ * -1.0), 0.0), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), 0.0));
    return flector_geometricAntiProduct_flector(geometric_anti_product, origin_antiReverse(self_));
}
fn motor_wedge_horizon(self: Motor, other: Horizon) -> Horizon {
    return Horizon(/* e321 */ (self_.g1_.w * other.g0_));
}
fn horizon_geometricAntiQuotient_flector(self: Horizon, other: Flector) -> Horizon {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.w, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Horizon = Horizon(/* e321 */ (anti_inverse.g0_ * self_.g0_));
    return geometric_anti_product;
}
fn flector_geometricAntiProduct_multiVector(self: Flector, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(((self_.g0_.x * other.g4_.x) + (self_.g0_.y * other.g4_.y) + (self_.g0_.z * other.g4_.z) + (self_.g0_.w * other.g4_.w) - (self_.g1_.x * other.g1_.x) - (self_.g1_.y * other.g1_.y) - (self_.g1_.z * other.g1_.z) - (self_.g1_.w * other.g1_.w)), (-(self_.g0_.w * other.g1_.w) + (self_.g1_.x * other.g4_.x) + (self_.g1_.y * other.g4_.y) + (self_.g1_.z * other.g4_.z))), /* e1, e2, e3, e4 */ vec4<f32>((-(other.g0_.x * self_.g1_.x) + (other.g0_.y * self_.g0_.x) + (other.g2_.x * self_.g1_.w) - (other.g2_.y * self_.g0_.z) + (other.g2_.z * self_.g0_.y) - (other.g3_.x * self_.g0_.w) + (other.g3_.y * self_.g1_.z) - (other.g3_.z * self_.g1_.y)), (-(other.g0_.x * self_.g1_.y) + (other.g0_.y * self_.g0_.y) + (other.g2_.x * self_.g0_.z) + (other.g2_.y * self_.g1_.w) - (other.g2_.z * self_.g0_.x) - (other.g3_.x * self_.g1_.z) - (other.g3_.y * self_.g0_.w) + (other.g3_.z * self_.g1_.x)), (-(other.g0_.x * self_.g1_.z) + (other.g0_.y * self_.g0_.z) - (other.g2_.x * self_.g0_.y) + (other.g2_.y * self_.g0_.x) + (other.g2_.z * self_.g1_.w) + (other.g3_.x * self_.g1_.y) - (other.g3_.y * self_.g1_.x) - (other.g3_.z * self_.g0_.w)), ((other.g0_.y * self_.g0_.w) - (other.g2_.x * self_.g1_.x) - (other.g2_.y * self_.g1_.y) - (other.g2_.z * self_.g1_.z))), /* e41, e42, e43 */ vec3<f32>((-(self_.g0_.w * other.g4_.x) - (self_.g1_.x * other.g1_.w) - (self_.g1_.y * other.g4_.z) + (self_.g1_.z * other.g4_.y)), (-(self_.g0_.w * other.g4_.y) + (self_.g1_.x * other.g4_.z) - (self_.g1_.y * other.g1_.w) - (self_.g1_.z * other.g4_.x)), (-(self_.g0_.w * other.g4_.z) - (self_.g1_.x * other.g4_.y) + (self_.g1_.y * other.g4_.x) - (self_.g1_.z * other.g1_.w))), /* e23, e31, e12 */ vec3<f32>((-(self_.g0_.x * other.g1_.w) - (self_.g0_.y * other.g4_.z) + (self_.g0_.z * other.g4_.y) + (self_.g0_.w * other.g1_.x) + (self_.g1_.x * other.g4_.w) + (self_.g1_.y * other.g1_.z) - (self_.g1_.z * other.g1_.y) - (self_.g1_.w * other.g4_.x)), ((self_.g0_.x * other.g4_.z) - (self_.g0_.y * other.g1_.w) - (self_.g0_.z * other.g4_.x) + (self_.g0_.w * other.g1_.y) - (self_.g1_.x * other.g1_.z) + (self_.g1_.y * other.g4_.w) + (self_.g1_.z * other.g1_.x) - (self_.g1_.w * other.g4_.y)), (-(self_.g0_.x * other.g4_.y) + (self_.g0_.y * other.g4_.x) - (self_.g0_.z * other.g1_.w) + (self_.g0_.w * other.g1_.z) + (self_.g1_.x * other.g1_.y) - (self_.g1_.y * other.g1_.x) + (self_.g1_.z * other.g4_.w) - (self_.g1_.w * other.g4_.z))), /* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.y * self_.g1_.x) + (other.g2_.x * self_.g0_.w) - (other.g2_.y * self_.g1_.z) + (other.g2_.z * self_.g1_.y)), ((other.g0_.y * self_.g1_.y) + (other.g2_.x * self_.g1_.z) + (other.g2_.y * self_.g0_.w) - (other.g2_.z * self_.g1_.x)), ((other.g0_.y * self_.g1_.z) - (other.g2_.x * self_.g1_.y) + (other.g2_.y * self_.g1_.x) + (other.g2_.z * self_.g0_.w)), (-(other.g0_.x * self_.g0_.w) + (other.g0_.y * self_.g1_.w) - (other.g2_.x * self_.g0_.x) - (other.g2_.y * self_.g0_.y) - (other.g2_.z * self_.g0_.z) + (other.g3_.x * self_.g1_.x) + (other.g3_.y * self_.g1_.y) + (other.g3_.z * self_.g1_.z))));
}
fn plane_bitXor_origin(self: Plane, other: Origin) -> AntiScalar {
    return plane_wedge_plane(self_, other);
}
fn horizon_add_multiVector(self: Horizon, other: MultiVector) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ other.g0_, /* e1, e2, e3, e4 */ other.g1_, /* e41, e42, e43 */ other.g2_, /* e23, e31, e12 */ other.g3_, /* e423, e431, e412, e321 */ vec4<f32>(other.g4_.x, other.g4_.y, other.g4_.z, (other.g4_.w + self_.g0_)));
    return addition;
}
fn flector_mul_multiVector(self: Flector, other: MultiVector) -> MultiVector {
    return flector_geometricProduct_flector(self_, other);
}
fn horizon_geometricProduct_antiScalar(self: Horizon, other: AntiScalar) -> Origin {
    return Origin(/* e4 */ (other.g0_ * self_.g0_));
}
fn dualNum_geometricAntiQuotient_antiScalar(self: DualNum, other: AntiScalar) -> DualNum {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>(anti_inverse.g0_) * self_.g0_));
    return geometric_anti_product;
}
fn flector_geometricProduct_flector(self: Flector, other: Flector) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(((other.g0_.x * self_.g0_.w) + (other.g0_.y * self_.g1_.z) - (other.g0_.z * self_.g1_.y) - (other.g0_.w * self_.g0_.x) - (other.g1_.x * self_.g1_.w) - (other.g1_.y * self_.g0_.z) + (other.g1_.z * self_.g0_.y) + (other.g1_.w * self_.g1_.x)), (-(other.g0_.x * self_.g1_.z) + (other.g0_.y * self_.g0_.w) + (other.g0_.z * self_.g1_.x) - (other.g0_.w * self_.g0_.y) + (other.g1_.x * self_.g0_.z) - (other.g1_.y * self_.g1_.w) - (other.g1_.z * self_.g0_.x) + (other.g1_.w * self_.g1_.y)), ((other.g0_.x * self_.g1_.y) - (other.g0_.y * self_.g1_.x) + (other.g0_.z * self_.g0_.w) - (other.g0_.w * self_.g0_.z) - (other.g1_.x * self_.g0_.y) + (other.g1_.y * self_.g0_.x) - (other.g1_.z * self_.g1_.w) + (other.g1_.w * self_.g1_.z)), (-(other.g0_.x * self_.g1_.x) - (other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g0_.w * self_.g1_.w) + (other.g1_.x * self_.g0_.x) + (other.g1_.y * self_.g0_.y) + (other.g1_.z * self_.g0_.z) + (other.g1_.w * self_.g0_.w))), /* e23, e31, e12, scalar */ vec4<f32>((-(other.g0_.x * self_.g1_.w) - (other.g0_.y * self_.g0_.z) + (other.g0_.z * self_.g0_.y) - (other.g1_.w * self_.g0_.x)), ((other.g0_.x * self_.g0_.z) - (other.g0_.y * self_.g1_.w) - (other.g0_.z * self_.g0_.x) - (other.g1_.w * self_.g0_.y)), (-(other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x) - (other.g0_.z * self_.g1_.w) - (other.g1_.w * self_.g0_.z)), ((other.g0_.x * self_.g0_.x) + (other.g0_.y * self_.g0_.y) + (other.g0_.z * self_.g0_.z) - (other.g1_.w * self_.g1_.w))));
}
fn origin_geometricAntiProduct_dualNum(self: Origin, other: DualNum) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.y * self_.g0_)), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.x * self_.g0_ * -1.0)));
}
fn plane_geometricAntiQuotient_antiScalar(self: Plane, other: AntiScalar) -> Plane {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Plane = Plane(/* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_));
    return geometric_anti_product;
}
fn dualNum_sandwich_point(self: DualNum, other: Point) -> Flector {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_.x) * other.g0_), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.y * other.g0_.x * -1.0), (self_.g0_.y * other.g0_.y * -1.0), (self_.g0_.y * other.g0_.z * -1.0), 0.0));
    return flector_geometricProduct_flector(geometric_product, dualNum_reverse(self_));
}
fn dualNum_sub_origin(self: DualNum, other: Origin) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_ * -1.0)), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    return subtraction;
}
fn flector_geometricAntiQuotient_dualNum(self: Flector, other: DualNum) -> Flector {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_.y, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g1_));
    return geometric_anti_product;
}
fn point_sub_antiScalar(self: Point, other: AntiScalar) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (other.g0_ * -1.0)), /* e1, e2, e3, e4 */ self_.g0_, /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    return subtraction;
}
fn horizon_mul_antiScalar(self: Horizon, other: AntiScalar) -> Origin {
    return horizon_geometricProduct_horizon(self_, other);
}
fn horizon_antiWedge_plane(self: Horizon, other: Plane) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>((other.g0_.x * self_.g0_ * -1.0), (other.g0_.y * self_.g0_ * -1.0), (other.g0_.z * self_.g0_ * -1.0)));
}
fn antiScalar_antiSandwich_scalar(self: AntiScalar, other: Scalar) -> Scalar {
    let geometric_anti_product: Scalar = Scalar(/* scalar */ (self_.g0_ * other.g0_));
    return scalar_geometricAntiProduct_scalar(geometric_anti_product, antiScalar_antiReverse(self_));
}
fn scalar_geometricProduct_point(self: Scalar, other: Point) -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), (other.g0_.w * self_.g0_)));
}
fn origin_geometricAntiProduct_multiVector(self: Origin, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((other.g4_.w * self_.g0_), (other.g1_.w * self_.g0_ * -1.0)), /* e1, e2, e3, e4 */ vec4<f32>((other.g3_.x * self_.g0_ * -1.0), (other.g3_.y * self_.g0_ * -1.0), (other.g3_.z * self_.g0_ * -1.0), (other.g0_.y * self_.g0_)), /* e41, e42, e43 */ vec3<f32>((other.g4_.x * self_.g0_ * -1.0), (other.g4_.y * self_.g0_ * -1.0), (other.g4_.z * self_.g0_ * -1.0)), /* e23, e31, e12 */ vec3<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_)), /* e423, e431, e412, e321 */ vec4<f32>((other.g2_.x * self_.g0_), (other.g2_.y * self_.g0_), (other.g2_.z * self_.g0_), (other.g0_.x * self_.g0_ * -1.0)));
}
fn plane_sandwich_origin(self: Plane, other: Origin) -> Origin {
    let geometric_product: AntiScalar = AntiScalar(/* e1234 */ (self_.g0_.w * other.g0_ * -1.0));
    return antiScalar_geometricProduct_antiScalar(geometric_product, plane_reverse(self_));
}
fn multiVector_geometricProduct_point(self: MultiVector, other: Point) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(((self_.g1_.x * other.g0_.x) + (self_.g1_.y * other.g0_.y) + (self_.g1_.z * other.g0_.z)), (-(self_.g4_.x * other.g0_.x) - (self_.g4_.y * other.g0_.y) - (self_.g4_.z * other.g0_.z) - (self_.g4_.w * other.g0_.w))), /* e1, e2, e3, e4 */ vec4<f32>(((self_.g0_.x * other.g0_.x) - (self_.g3_.y * other.g0_.z) + (self_.g3_.z * other.g0_.y)), ((self_.g0_.x * other.g0_.y) + (self_.g3_.x * other.g0_.z) - (self_.g3_.z * other.g0_.x)), ((self_.g0_.x * other.g0_.z) - (self_.g3_.x * other.g0_.y) + (self_.g3_.y * other.g0_.x)), ((self_.g0_.x * other.g0_.w) + (self_.g2_.x * other.g0_.x) + (self_.g2_.y * other.g0_.y) + (self_.g2_.z * other.g0_.z))), /* e41, e42, e43 */ vec3<f32>((-(self_.g1_.x * other.g0_.w) + (self_.g1_.w * other.g0_.x) - (self_.g4_.y * other.g0_.z) + (self_.g4_.z * other.g0_.y)), (-(self_.g1_.y * other.g0_.w) + (self_.g1_.w * other.g0_.y) + (self_.g4_.x * other.g0_.z) - (self_.g4_.z * other.g0_.x)), (-(self_.g1_.z * other.g0_.w) + (self_.g1_.w * other.g0_.z) - (self_.g4_.x * other.g0_.y) + (self_.g4_.y * other.g0_.x))), /* e23, e31, e12 */ vec3<f32>(((self_.g1_.y * other.g0_.z) - (self_.g1_.z * other.g0_.y) - (self_.g4_.w * other.g0_.x)), (-(self_.g1_.x * other.g0_.z) + (self_.g1_.z * other.g0_.x) - (self_.g4_.w * other.g0_.y)), ((self_.g1_.x * other.g0_.y) - (self_.g1_.y * other.g0_.x) - (self_.g4_.w * other.g0_.z))), /* e423, e431, e412, e321 */ vec4<f32>((-(self_.g0_.y * other.g0_.x) + (self_.g2_.y * other.g0_.z) - (self_.g2_.z * other.g0_.y) + (self_.g3_.x * other.g0_.w)), (-(self_.g0_.y * other.g0_.y) - (self_.g2_.x * other.g0_.z) + (self_.g2_.z * other.g0_.x) + (self_.g3_.y * other.g0_.w)), (-(self_.g0_.y * other.g0_.z) + (self_.g2_.x * other.g0_.y) - (self_.g2_.y * other.g0_.x) + (self_.g3_.z * other.g0_.w)), (-(self_.g3_.x * other.g0_.x) - (self_.g3_.y * other.g0_.y) - (self_.g3_.z * other.g0_.z))));
}
fn dualNum_sandwich_scalar(self: DualNum, other: Scalar) -> DualNum {
    let geometric_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>(other.g0_) * self_.g0_));
    return dualNum_geometricProduct_dualNum(geometric_product, dualNum_reverse(self_));
}
fn line_geometricProduct_flector(self: Line, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(((self_.g1_.x * other.g1_.w) - (self_.g1_.y * other.g0_.z) + (self_.g1_.z * other.g0_.y)), ((self_.g1_.x * other.g0_.z) + (self_.g1_.y * other.g1_.w) - (self_.g1_.z * other.g0_.x)), (-(self_.g1_.x * other.g0_.y) + (self_.g1_.y * other.g0_.x) + (self_.g1_.z * other.g1_.w)), ((self_.g0_.x * other.g0_.x) + (self_.g0_.y * other.g0_.y) + (self_.g0_.z * other.g0_.z) - (self_.g1_.x * other.g1_.x) - (self_.g1_.y * other.g1_.y) - (self_.g1_.z * other.g1_.z))), /* e423, e431, e412, e321 */ vec4<f32>((-(self_.g0_.x * other.g1_.w) + (self_.g0_.y * other.g0_.z) - (self_.g0_.z * other.g0_.y) + (self_.g1_.x * other.g0_.w) - (self_.g1_.y * other.g1_.z) + (self_.g1_.z * other.g1_.y)), (-(self_.g0_.x * other.g0_.z) - (self_.g0_.y * other.g1_.w) + (self_.g0_.z * other.g0_.x) + (self_.g1_.x * other.g1_.z) + (self_.g1_.y * other.g0_.w) - (self_.g1_.z * other.g1_.x)), ((self_.g0_.x * other.g0_.y) - (self_.g0_.y * other.g0_.x) - (self_.g0_.z * other.g1_.w) - (self_.g1_.x * other.g1_.y) + (self_.g1_.y * other.g1_.x) + (self_.g1_.z * other.g0_.w)), (-(self_.g1_.x * other.g0_.x) - (self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))));
}
fn point_add_multiVector(self: Point, other: MultiVector) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ other.g0_, /* e1, e2, e3, e4 */ (other.g1_ + self_.g0_), /* e41, e42, e43 */ other.g2_, /* e23, e31, e12 */ other.g3_, /* e423, e431, e412, e321 */ other.g4_);
    return addition;
}
fn antiScalar_antiWedge_line(self: AntiScalar, other: Line) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_)), /* e23, e31, e12 */ vec3<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_)));
}
fn scalar_geometricAntiProduct_motor(self: Scalar, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0), /* e23, e31, e12, scalar */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), (other.g0_.w * self_.g0_)));
}
fn origin_geometricAntiProduct_motor(self: Origin, other: Motor) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g1_.x * self_.g0_ * -1.0), (other.g1_.y * self_.g0_ * -1.0), (other.g1_.z * self_.g0_ * -1.0), (other.g0_.w * self_.g0_)), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), (other.g1_.w * self_.g0_ * -1.0)));
}
fn flector_antiSandwich_plane(self: Flector, other: Plane) -> Flector {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>((-(self_.g0_.w * other.g0_.x) - (self_.g1_.y * other.g0_.z)), (-(self_.g0_.w * other.g0_.y) - (self_.g1_.z * other.g0_.x)), (-(self_.g0_.w * other.g0_.z) - (self_.g1_.x * other.g0_.y)), ((self_.g1_.y * other.g0_.y) + (self_.g1_.z * other.g0_.z))) + (self_.g1_.zxyx * other.g0_.yzxx)), /* e23, e31, e12, scalar */ (vec4<f32>((-(self_.g0_.y * other.g0_.z) - (self_.g1_.w * other.g0_.x)), (-(self_.g0_.z * other.g0_.x) - (self_.g1_.w * other.g0_.y)), (-(self_.g0_.x * other.g0_.y) - (self_.g1_.w * other.g0_.z)), ((self_.g0_.z * other.g0_.z) + (self_.g0_.w * other.g0_.w))) + (vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g0_.y) * other.g0_.wwwy) + (self_.g0_.zxyx * other.g0_.yzxx)));
    return motor_geometricAntiProduct_motor(geometric_anti_product, flector_antiReverse(self_));
}
fn multiVector_geometricAntiProduct_point(self: MultiVector, other: Point) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((-(self_.g4_.x * other.g0_.x) - (self_.g4_.y * other.g0_.y) - (self_.g4_.z * other.g0_.z) - (self_.g4_.w * other.g0_.w)), (self_.g1_.w * other.g0_.w * -1.0)), /* e1, e2, e3, e4 */ vec4<f32>(((self_.g0_.y * other.g0_.x) + (self_.g2_.y * other.g0_.z) - (self_.g2_.z * other.g0_.y) + (self_.g3_.x * other.g0_.w)), ((self_.g0_.y * other.g0_.y) - (self_.g2_.x * other.g0_.z) + (self_.g2_.z * other.g0_.x) + (self_.g3_.y * other.g0_.w)), ((self_.g0_.y * other.g0_.z) + (self_.g2_.x * other.g0_.y) - (self_.g2_.y * other.g0_.x) + (self_.g3_.z * other.g0_.w)), (self_.g0_.y * other.g0_.w)), /* e41, e42, e43 */ vec3<f32>((self_.g4_.x * other.g0_.w * -1.0), (self_.g4_.y * other.g0_.w * -1.0), (self_.g4_.z * other.g0_.w * -1.0)), /* e23, e31, e12 */ vec3<f32>((-(self_.g1_.x * other.g0_.w) + (self_.g1_.w * other.g0_.x) + (self_.g4_.y * other.g0_.z) - (self_.g4_.z * other.g0_.y)), (-(self_.g1_.y * other.g0_.w) + (self_.g1_.w * other.g0_.y) - (self_.g4_.x * other.g0_.z) + (self_.g4_.z * other.g0_.x)), (-(self_.g1_.z * other.g0_.w) + (self_.g1_.w * other.g0_.z) + (self_.g4_.x * other.g0_.y) - (self_.g4_.y * other.g0_.x))), /* e423, e431, e412, e321 */ vec4<f32>((self_.g2_.x * other.g0_.w), (self_.g2_.y * other.g0_.w), (self_.g2_.z * other.g0_.w), ((self_.g0_.x * other.g0_.w) - (self_.g2_.x * other.g0_.x) - (self_.g2_.y * other.g0_.y) - (self_.g2_.z * other.g0_.z))));
}
fn motor_antiSandwich_horizon(self: Motor, other: Horizon) -> Flector {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), 0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.w * other.g0_)));
    return flector_geometricAntiProduct_flector(geometric_anti_product, motor_antiReverse(self_));
}
fn multiVector_wedge_antiScalar(self: MultiVector, other: AntiScalar) -> AntiScalar {
    return AntiScalar(/* e1234 */ (self_.g0_.x * other.g0_));
}
fn point_mul_point(self: Point, other: Point) -> Motor {
    return point_geometricProduct_point(self_, other);
}
fn point_bitXor_flector(self: Point, other: Flector) -> Motor {
    return point_wedge_point(self_, other);
}
fn horizon_geometricAntiProduct_motor(self: Horizon, other: Motor) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), 0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.w * self_.g0_)));
}
fn multiVector_antiSandwich_motor(self: MultiVector, other: Motor) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(((self_.g0_.y * other.g1_.w) - (self_.g3_.x * other.g0_.x) - (self_.g3_.y * other.g0_.y) - (self_.g3_.z * other.g0_.z)), 0.0) - (vec2<f32>(self_.g2_.x) * vec2<f32>(other.g1_.x, other.g0_.x)) - (vec2<f32>(self_.g2_.y) * vec2<f32>(other.g1_.y, other.g0_.y)) - (vec2<f32>(self_.g2_.z) * vec2<f32>(other.g1_.z, other.g0_.z)) + (vec2<f32>(other.g0_.w) * self_.g0_)), /* e1, e2, e3, e4 */ (vec4<f32>(((other.g0_.z * self_.g1_.y) + (other.g0_.w * self_.g1_.x) - (other.g1_.x * self_.g1_.w) + (other.g1_.y * self_.g4_.z)), ((other.g0_.y * self_.g4_.w) + (other.g0_.w * self_.g1_.y) - (other.g1_.y * self_.g1_.w) + (other.g1_.z * self_.g4_.x)), ((other.g0_.z * self_.g4_.w) + (other.g0_.w * self_.g1_.z) + (other.g1_.x * self_.g4_.y) - (other.g1_.z * self_.g1_.w)), 0.0) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.y) * self_.g4_.yzxy) - (vec4<f32>(other.g1_.w, other.g1_.w, other.g1_.w, other.g0_.z) * self_.g4_.xyzz) - (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g4_.x) * other.g0_.yzxx) + (vec4<f32>(self_.g4_.w, self_.g1_.z, self_.g1_.x, self_.g1_.w) * other.g0_.xxyw)), /* e41, e42, e43 */ ((vec3<f32>(self_.g0_.y) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z)) - (vec3<f32>(other.g0_.y, other.g0_.z, other.g0_.x) * self_.g2_.zxy) + (vec3<f32>(other.g0_.z, other.g0_.x, other.g0_.w) * self_.g2_.yzz) + (vec3<f32>(other.g0_.w, other.g0_.w, other.g0_.y) * self_.g2_.xyx)), /* e23, e31, e12 */ ((vec3<f32>(self_.g0_.x) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z)) + (vec3<f32>(self_.g0_.y) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z)) - (vec3<f32>(other.g0_.y, other.g0_.z, other.g0_.x) * self_.g3_.zxy) + (vec3<f32>(other.g0_.z, other.g0_.x, other.g0_.w) * self_.g3_.yzz) + (vec3<f32>(other.g0_.w, other.g0_.w, other.g0_.y) * self_.g3_.xyx) - (vec3<f32>(other.g1_.y, other.g1_.z, other.g1_.x) * self_.g2_.zxy) + (vec3<f32>(other.g1_.z, other.g1_.x, other.g1_.w) * self_.g2_.yzz) + (vec3<f32>(other.g1_.w, other.g1_.w, other.g1_.y) * self_.g2_.xyx)), /* e423, e431, e412, e321 */ (vec4<f32>((other.g0_.w * self_.g4_.x), (other.g0_.y * self_.g1_.w), (other.g0_.z * self_.g1_.w), (-(other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) + (other.g1_.y * self_.g4_.y) + (other.g1_.z * self_.g4_.z) - (other.g1_.w * self_.g1_.w))) + (vec4<f32>(other.g0_.z, other.g0_.w, other.g0_.w, other.g1_.x) * self_.g4_.yyzx) + (vec4<f32>(self_.g1_.w, self_.g4_.z, self_.g4_.x, self_.g4_.w) * other.g0_.xxyw) - (vec4<f32>(self_.g4_.z, self_.g4_.x, self_.g4_.y, self_.g1_.x) * other.g0_.yzxx)));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self_));
}
fn multiVector_antiSandwich_dualNum(self: MultiVector, other: DualNum) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(((other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x)), (other.g0_.y * self_.g0_.y)), /* e1, e2, e3, e4 */ vec4<f32>((-(other.g0_.x * self_.g4_.x) + (other.g0_.y * self_.g1_.x)), (-(other.g0_.x * self_.g4_.y) + (other.g0_.y * self_.g1_.y)), (-(other.g0_.x * self_.g4_.z) + (other.g0_.y * self_.g1_.z)), (other.g0_.y * self_.g1_.w)), /* e41, e42, e43 */ (vec3<f32>(other.g0_.y) * self_.g2_), /* e23, e31, e12 */ ((vec3<f32>(other.g0_.x) * self_.g2_) + (vec3<f32>(other.g0_.y) * self_.g3_)), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.y * self_.g4_.x), (other.g0_.y * self_.g4_.y), (other.g0_.y * self_.g4_.z), (-(other.g0_.x * self_.g1_.w) + (other.g0_.y * self_.g4_.w))));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self_));
}
fn line_sandwich_origin(self: Line, other: Origin) -> Flector {
    let geometric_product: Plane = Plane(/* e423, e431, e412, e321 */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), 0.0));
    return plane_geometricProduct_plane(geometric_product, line_reverse(self_));
}
fn point_sub_dualNum(self: Point, other: DualNum) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ (other.g0_ * vec2<f32>(-1.0)), /* e1, e2, e3, e4 */ self_.g0_, /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    return subtraction;
}
fn scalar_sandwich_plane(self: Scalar, other: Plane) -> Plane {
    let geometric_product: Plane = Plane(/* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_) * other.g0_));
    return plane_geometricProduct_plane(geometric_product, scalar_reverse(self_));
}
fn origin_wedge_flector(self: Origin, other: Flector) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), (other.g1_.w * self_.g0_)), /* e23, e31, e12, scalar */ vec4<f32>(0.0));
}
fn point_sub_point(self: Point, other: Point) -> Point {
    let subtraction: Point = Point(/* e1, e2, e3, e4 */ (-other.g0_ + self_.g0_));
    return subtraction;
}
fn scalar_geometricAntiQuotient_origin(self: Scalar, other: Origin) -> Scalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Scalar = Scalar(/* scalar */ (anti_inverse.g0_ * self_.g0_));
    return geometric_anti_product;
}
fn plane_sandwich_plane(self: Plane, other: Plane) -> Flector {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>((-(other.g0_.x * self_.g0_.w) + (other.g0_.w * self_.g0_.x)), (-(other.g0_.y * self_.g0_.w) + (other.g0_.w * self_.g0_.y)), (-(other.g0_.z * self_.g0_.w) + (other.g0_.w * self_.g0_.z)), 0.0), /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.w * self_.g0_.w * -1.0)));
    return motor_geometricProduct_motor(geometric_product, plane_reverse(self_));
}
fn scalar_add_multiVector(self: Scalar, other: MultiVector) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>((other.g0_.x + self_.g0_), other.g0_.y), /* e1, e2, e3, e4 */ other.g1_, /* e41, e42, e43 */ other.g2_, /* e23, e31, e12 */ other.g3_, /* e423, e431, e412, e321 */ other.g4_);
    return addition;
}
fn plane_add_scalar(self: Plane, other: Scalar) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(other.g0_, 0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ self_.g0_);
    return addition;
}
fn line_sub_line(self: Line, other: Line) -> Line {
    let subtraction: Line = Line(/* e41, e42, e43 */ (-other.g0_ + self_.g0_), /* e23, e31, e12 */ (-other.g1_ + self_.g1_));
    return subtraction;
}
fn multiVector_geometricProduct_scalar(self: MultiVector, other: Scalar) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_)), /* e1, e2, e3, e4 */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), (self_.g1_.w * other.g0_)), /* e41, e42, e43 */ vec3<f32>((self_.g2_.x * other.g0_), (self_.g2_.y * other.g0_), (self_.g2_.z * other.g0_)), /* e23, e31, e12 */ vec3<f32>((self_.g3_.x * other.g0_), (self_.g3_.y * other.g0_), (self_.g3_.z * other.g0_)), /* e423, e431, e412, e321 */ vec4<f32>((self_.g4_.x * other.g0_), (self_.g4_.y * other.g0_), (self_.g4_.z * other.g0_), (self_.g4_.w * other.g0_)));
}
fn origin_wedge_point(self: Origin, other: Point) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_)), /* e23, e31, e12 */ vec3<f32>(0.0));
}
fn origin_bitXor_dualNum(self: Origin, other: DualNum) -> Origin {
    return origin_wedge_origin(self_, other);
}
fn plane_mul_antiScalar(self: Plane, other: AntiScalar) -> Origin {
    return plane_geometricProduct_plane(self_, other);
}
fn plane_geometricProduct_dualNum(self: Plane, other: DualNum) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.y * self_.g0_.w)), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.x * self_.g0_.x), (other.g0_.x * self_.g0_.y), (other.g0_.x * self_.g0_.z), (other.g0_.x * self_.g0_.w)));
}
fn dualNum_geometricQuotient_line(self: DualNum, other: Line) -> DualNum {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>(inverse.g0_) * self_.g0_));
    return geometric_product;
}
fn point_antiSandwich_dualNum(self: Point, other: DualNum) -> Motor {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(other.g0_.y) * self_.g0_), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.x * self_.g0_.w * -1.0)));
    return flector_geometricAntiProduct_flector(geometric_anti_product, point_antiReverse(self_));
}
fn flector_antiSandwich_antiScalar(self: Flector, other: AntiScalar) -> Motor {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(other.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_) * self_.g1_));
    return flector_geometricAntiProduct_flector(geometric_anti_product, flector_antiReverse(self_));
}
fn scalar_geometricQuotient_line(self: Scalar, other: Line) -> Scalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Scalar = Scalar(/* scalar */ (inverse.g0_ * self_.g0_));
    return geometric_product;
}
fn origin_antiWedge_plane(self: Origin, other: Plane) -> Scalar {
    return Scalar(/* scalar */ (other.g0_.w * self_.g0_));
}
fn antiScalar_geometricAntiQuotient_point(self: AntiScalar, other: Point) -> AntiScalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.w, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: AntiScalar = AntiScalar(/* e1234 */ (anti_inverse.g0_ * self_.g0_));
    return geometric_anti_product;
}
fn antiScalar_antiWedge_horizon(self: AntiScalar, other: Horizon) -> Horizon {
    return Horizon(/* e321 */ (self_.g0_ * other.g0_));
}
fn plane_add_line(self: Plane, other: Line) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ other.g0_, /* e23, e31, e12 */ other.g1_, /* e423, e431, e412, e321 */ self_.g0_);
    return addition;
}
fn flector_mul_point(self: Flector, other: Point) -> Motor {
    return flector_geometricProduct_flector(self_, other);
}
fn line_geometricAntiProduct_scalar(self: Line, other: Scalar) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_)));
}
fn multiVector_sub_origin(self: MultiVector, other: Origin) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, (self_.g1_.w - other.g0_)), /* e41, e42, e43 */ self_.g2_, /* e23, e31, e12 */ self_.g3_, /* e423, e431, e412, e321 */ self_.g4_);
    return subtraction;
}
fn horizon_geometricProduct_origin(self: Horizon, other: Origin) -> AntiScalar {
    return AntiScalar(/* e1234 */ (self_.g0_ * other.g0_ * -1.0));
}
fn motor_mul_motor(self: Motor, other: Motor) -> Motor {
    return motor_geometricProduct_motor(self_, other);
}
fn dualNum_antiWedge_multiVector(self: DualNum, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(((self_.g0_.x * other.g0_.y) + (self_.g0_.y * other.g0_.x)), (self_.g0_.y * other.g0_.y)), /* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.y * other.g1_.x), (self_.g0_.y * other.g1_.y), (self_.g0_.y * other.g1_.z), (self_.g0_.y * other.g1_.w)), /* e41, e42, e43 */ vec3<f32>((self_.g0_.y * other.g2_.x), (self_.g0_.y * other.g2_.y), (self_.g0_.y * other.g2_.z)), /* e23, e31, e12 */ vec3<f32>((self_.g0_.y * other.g3_.x), (self_.g0_.y * other.g3_.y), (self_.g0_.y * other.g3_.z)), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.y * other.g4_.x), (self_.g0_.y * other.g4_.y), (self_.g0_.y * other.g4_.z), (self_.g0_.y * other.g4_.w)));
}
fn dualNum_antiSandwich_motor(self: DualNum, other: Motor) -> Motor {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(self_.g0_.y) * other.g0_), /* e23, e31, e12, scalar */ ((vec4<f32>(self_.g0_.x) * other.g0_) + (vec4<f32>(self_.g0_.y) * other.g1_)));
    return motor_geometricAntiProduct_motor(geometric_anti_product, dualNum_antiReverse(self_));
}
fn multiVector_geometricAntiProduct_plane(self: MultiVector, other: Plane) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(((self_.g1_.x * other.g0_.x) + (self_.g1_.y * other.g0_.y) + (self_.g1_.z * other.g0_.z) + (self_.g1_.w * other.g0_.w)), ((self_.g4_.x * other.g0_.x) + (self_.g4_.y * other.g0_.y) + (self_.g4_.z * other.g0_.z))), /* e1, e2, e3, e4 */ vec4<f32>(((self_.g0_.x * other.g0_.x) + (self_.g2_.x * other.g0_.w) + (self_.g3_.y * other.g0_.z) - (self_.g3_.z * other.g0_.y)), ((self_.g0_.x * other.g0_.y) + (self_.g2_.y * other.g0_.w) - (self_.g3_.x * other.g0_.z) + (self_.g3_.z * other.g0_.x)), ((self_.g0_.x * other.g0_.z) + (self_.g2_.z * other.g0_.w) + (self_.g3_.x * other.g0_.y) - (self_.g3_.y * other.g0_.x)), (-(self_.g2_.x * other.g0_.x) - (self_.g2_.y * other.g0_.y) - (self_.g2_.z * other.g0_.z))), /* e41, e42, e43 */ vec3<f32>((-(self_.g1_.w * other.g0_.x) - (self_.g4_.y * other.g0_.z) + (self_.g4_.z * other.g0_.y)), (-(self_.g1_.w * other.g0_.y) + (self_.g4_.x * other.g0_.z) - (self_.g4_.z * other.g0_.x)), (-(self_.g1_.w * other.g0_.z) - (self_.g4_.x * other.g0_.y) + (self_.g4_.y * other.g0_.x))), /* e23, e31, e12 */ vec3<f32>((-(self_.g1_.y * other.g0_.z) + (self_.g1_.z * other.g0_.y) + (self_.g4_.x * other.g0_.w) - (self_.g4_.w * other.g0_.x)), ((self_.g1_.x * other.g0_.z) - (self_.g1_.z * other.g0_.x) + (self_.g4_.y * other.g0_.w) - (self_.g4_.w * other.g0_.y)), (-(self_.g1_.x * other.g0_.y) + (self_.g1_.y * other.g0_.x) + (self_.g4_.z * other.g0_.w) - (self_.g4_.w * other.g0_.z))), /* e423, e431, e412, e321 */ vec4<f32>(((self_.g0_.y * other.g0_.x) + (self_.g2_.y * other.g0_.z) - (self_.g2_.z * other.g0_.y)), ((self_.g0_.y * other.g0_.y) - (self_.g2_.x * other.g0_.z) + (self_.g2_.z * other.g0_.x)), ((self_.g0_.y * other.g0_.z) + (self_.g2_.x * other.g0_.y) - (self_.g2_.y * other.g0_.x)), ((self_.g0_.y * other.g0_.w) - (self_.g3_.x * other.g0_.x) - (self_.g3_.y * other.g0_.y) - (self_.g3_.z * other.g0_.z))));
}
fn origin_sub_point(self: Origin, other: Point) -> Point {
    let subtraction: Point = Point(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * -1.0), (other.g0_.y * -1.0), (other.g0_.z * -1.0), (-other.g0_.w + self_.g0_)));
    return subtraction;
}
fn scalar_mul_motor(self: Scalar, other: Motor) -> Motor {
    return scalar_geometricProduct_scalar(self_, other);
}
fn line_sandwich_point(self: Line, other: Point) -> Flector {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(((self_.g1_.y * other.g0_.z) * -1.0), ((self_.g1_.z * other.g0_.x) * -1.0), ((self_.g1_.x * other.g0_.y) * -1.0), ((self_.g0_.y * other.g0_.y) + (self_.g0_.z * other.g0_.z))) + (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g0_.x) * other.g0_.yzxx)), /* e423, e431, e412, e321 */ (vec4<f32>(((self_.g0_.y * other.g0_.z) + (self_.g1_.x * other.g0_.w)), ((self_.g0_.z * other.g0_.x) + (self_.g1_.y * other.g0_.w)), ((self_.g0_.x * other.g0_.y) + (self_.g1_.z * other.g0_.w)), (-(self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g1_.x) * other.g0_.yzxx)));
    return flector_geometricProduct_flector(geometric_product, line_reverse(self_));
}
fn scalar_geometricProduct_flector(self: Scalar, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), (other.g0_.w * self_.g0_)), /* e423, e431, e412, e321 */ vec4<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_), (other.g1_.w * self_.g0_)));
}
fn origin_antiWedge_antiScalar(self: Origin, other: AntiScalar) -> Origin {
    return Origin(/* e4 */ (other.g0_ * self_.g0_));
}
fn dualNum_sandwich_horizon(self: DualNum, other: Horizon) -> Flector {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.y * other.g0_ * -1.0)), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.x * other.g0_)));
    return flector_geometricProduct_flector(geometric_product, dualNum_reverse(self_));
}
fn motor_sandwich_motor(self: Motor, other: Motor) -> Motor {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(((other.g0_.y * self_.g1_.z) + (other.g0_.w * self_.g1_.x) + (other.g1_.y * self_.g0_.z) + (other.g1_.w * self_.g0_.x)), ((other.g0_.z * self_.g1_.x) + (other.g0_.w * self_.g1_.y) + (other.g1_.z * self_.g0_.x) + (other.g1_.w * self_.g0_.y)), ((other.g0_.z * self_.g1_.w) + (other.g0_.w * self_.g1_.z) + (other.g1_.z * self_.g0_.w) + (other.g1_.w * self_.g0_.z)), (-(other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))) + (other.g0_.xyxw * self_.g1_.wwyw) - (other.g0_.zxyx * self_.g1_.yzxx) + (other.g1_.xyxw * self_.g0_.wwyw) - (other.g1_.zxyx * self_.g0_.yzxx)), /* e23, e31, e12, scalar */ (vec4<f32>(((other.g1_.y * self_.g1_.z) + (other.g1_.w * self_.g1_.x)), ((other.g1_.z * self_.g1_.x) + (other.g1_.w * self_.g1_.y)), ((other.g1_.z * self_.g1_.w) + (other.g1_.w * self_.g1_.z)), (-(other.g1_.y * self_.g1_.y) - (other.g1_.z * self_.g1_.z))) + (other.g1_.xyxw * self_.g1_.wwyw) - (other.g1_.zxyx * self_.g1_.yzxx)));
    return motor_geometricProduct_motor(geometric_product, motor_reverse(self_));
}
fn scalar_geometricAntiProduct_flector(self: Scalar, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_), 0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.w * self_.g0_)));
}
fn plane_add_plane(self: Plane, other: Plane) -> Plane {
    let addition: Plane = Plane(/* e423, e431, e412, e321 */ (other.g0_ + self_.g0_));
    return addition;
}
fn point_antiWedge_horizon(self: Point, other: Horizon) -> Scalar {
    return Scalar(/* scalar */ (self_.g0_.w * other.g0_));
}
fn dualNum_add_scalar(self: DualNum, other: Scalar) -> DualNum {
    let addition: DualNum = DualNum(/* scalar, e1234 */ vec2<f32>((self_.g0_.x + other.g0_), self_.g0_.y));
    return addition;
}
fn horizon_wedge_multiVector(self: Horizon, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (other.g1_.w * self_.g0_ * -1.0)), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.x * self_.g0_)));
}
fn motor_bitXor_origin(self: Motor, other: Origin) -> Flector {
    return motor_wedge_motor(self_, other);
}
fn flector_geometricAntiProduct_line(self: Flector, other: Line) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(((other.g0_.x * self_.g1_.w) - (other.g0_.y * self_.g0_.z) + (other.g0_.z * self_.g0_.y) - (other.g1_.x * self_.g0_.w) + (other.g1_.y * self_.g1_.z) - (other.g1_.z * self_.g1_.y)), ((other.g0_.x * self_.g0_.z) + (other.g0_.y * self_.g1_.w) - (other.g0_.z * self_.g0_.x) - (other.g1_.x * self_.g1_.z) - (other.g1_.y * self_.g0_.w) + (other.g1_.z * self_.g1_.x)), (-(other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x) + (other.g0_.z * self_.g1_.w) + (other.g1_.x * self_.g1_.y) - (other.g1_.y * self_.g1_.x) - (other.g1_.z * self_.g0_.w)), (-(other.g0_.x * self_.g1_.x) - (other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z))), /* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.x * self_.g0_.w) - (other.g0_.y * self_.g1_.z) + (other.g0_.z * self_.g1_.y)), ((other.g0_.x * self_.g1_.z) + (other.g0_.y * self_.g0_.w) - (other.g0_.z * self_.g1_.x)), (-(other.g0_.x * self_.g1_.y) + (other.g0_.y * self_.g1_.x) + (other.g0_.z * self_.g0_.w)), (-(other.g0_.x * self_.g0_.x) - (other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z) + (other.g1_.x * self_.g1_.x) + (other.g1_.y * self_.g1_.y) + (other.g1_.z * self_.g1_.z))));
}
fn horizon_geometricAntiQuotient_plane(self: Horizon, other: Plane) -> Horizon {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Horizon = Horizon(/* e321 */ (anti_inverse.g0_ * self_.g0_));
    return geometric_anti_product;
}
fn motor_geometricProduct_dualNum(self: Motor, other: DualNum) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(((other.g0_.x * self_.g0_.x) + (other.g0_.y * self_.g1_.x)), ((other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g1_.y)), ((other.g0_.x * self_.g0_.z) + (other.g0_.y * self_.g1_.z)), ((other.g0_.x * self_.g0_.w) + (other.g0_.y * self_.g1_.w))), /* e23, e31, e12, scalar */ vec4<f32>((other.g0_.x * self_.g1_.x), (other.g0_.x * self_.g1_.y), (other.g0_.x * self_.g1_.z), (other.g0_.x * self_.g1_.w)));
}
fn plane_bitXor_point(self: Plane, other: Point) -> AntiScalar {
    return plane_wedge_plane(self_, other);
}
fn scalar_mul_flector(self: Scalar, other: Flector) -> Flector {
    return scalar_geometricProduct_scalar(self_, other);
}
fn horizon_antiSandwich_plane(self: Horizon, other: Plane) -> Point {
    let geometric_anti_product: Line = Line(/* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ (vec3<f32>(self_.g0_) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z) * vec3<f32>(-1.0)));
    return line_geometricAntiProduct_line(geometric_anti_product, horizon_antiReverse(self_));
}
fn point_geometricAntiQuotient_point(self: Point, other: Point) -> Point {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.w, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Point = Point(/* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_));
    return geometric_anti_product;
}
fn flector_bitXor_plane(self: Flector, other: Plane) -> AntiScalar {
    return flector_wedge_flector(self_, other);
}
fn origin_antiSandwich_motor(self: Origin, other: Motor) -> Motor {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, other.g0_.w) * vec4<f32>(-1.0, -1.0, -1.0, 1.0)), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.w) * vec4<f32>(1.0, 1.0, 1.0, -1.0)));
    return flector_geometricAntiProduct_flector(geometric_anti_product, origin_antiReverse(self_));
}
fn horizon_mul_line(self: Horizon, other: Line) -> Flector {
    return horizon_geometricProduct_horizon(self_, other);
}
fn multiVector_geometricProduct_dualNum(self: MultiVector, other: DualNum) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((other.g0_.x * self_.g0_.x), ((other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x))), /* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g1_.x), (other.g0_.x * self_.g1_.y), (other.g0_.x * self_.g1_.z), ((other.g0_.x * self_.g1_.w) + (other.g0_.y * self_.g4_.w))), /* e41, e42, e43 */ vec3<f32>(((other.g0_.x * self_.g2_.x) + (other.g0_.y * self_.g3_.x)), ((other.g0_.x * self_.g2_.y) + (other.g0_.y * self_.g3_.y)), ((other.g0_.x * self_.g2_.z) + (other.g0_.y * self_.g3_.z))), /* e23, e31, e12 */ vec3<f32>((other.g0_.x * self_.g3_.x), (other.g0_.x * self_.g3_.y), (other.g0_.x * self_.g3_.z)), /* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.x * self_.g4_.x) + (other.g0_.y * self_.g1_.x)), ((other.g0_.x * self_.g4_.y) + (other.g0_.y * self_.g1_.y)), ((other.g0_.x * self_.g4_.z) + (other.g0_.y * self_.g1_.z)), (other.g0_.x * self_.g4_.w)));
}
fn motor_geometricProduct_multiVector(self: Motor, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(((other.g0_.x * self_.g1_.w) - (other.g3_.x * self_.g1_.x) - (other.g3_.y * self_.g1_.y) - (other.g3_.z * self_.g1_.z)), ((other.g0_.x * self_.g0_.w) + (other.g0_.y * self_.g1_.w) - (other.g2_.x * self_.g1_.x) - (other.g2_.y * self_.g1_.y) - (other.g2_.z * self_.g1_.z) - (other.g3_.x * self_.g0_.x) - (other.g3_.y * self_.g0_.y) - (other.g3_.z * self_.g0_.z))), /* e1, e2, e3, e4 */ vec4<f32>(((self_.g1_.x * other.g4_.w) - (self_.g1_.y * other.g1_.z) + (self_.g1_.z * other.g1_.y) + (self_.g1_.w * other.g1_.x)), ((self_.g1_.x * other.g1_.z) + (self_.g1_.y * other.g4_.w) - (self_.g1_.z * other.g1_.x) + (self_.g1_.w * other.g1_.y)), (-(self_.g1_.x * other.g1_.y) + (self_.g1_.y * other.g1_.x) + (self_.g1_.z * other.g4_.w) + (self_.g1_.w * other.g1_.z)), ((self_.g0_.x * other.g1_.x) + (self_.g0_.y * other.g1_.y) + (self_.g0_.z * other.g1_.z) - (self_.g0_.w * other.g4_.w) - (self_.g1_.x * other.g4_.x) - (self_.g1_.y * other.g4_.y) - (self_.g1_.z * other.g4_.z) + (self_.g1_.w * other.g1_.w))), /* e41, e42, e43 */ vec3<f32>(((other.g0_.x * self_.g0_.x) + (other.g0_.y * self_.g1_.x) + (other.g2_.x * self_.g1_.w) + (other.g2_.y * self_.g1_.z) - (other.g2_.z * self_.g1_.y) + (other.g3_.x * self_.g0_.w) + (other.g3_.y * self_.g0_.z) - (other.g3_.z * self_.g0_.y)), ((other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g1_.y) - (other.g2_.x * self_.g1_.z) + (other.g2_.y * self_.g1_.w) + (other.g2_.z * self_.g1_.x) - (other.g3_.x * self_.g0_.z) + (other.g3_.y * self_.g0_.w) + (other.g3_.z * self_.g0_.x)), ((other.g0_.x * self_.g0_.z) + (other.g0_.y * self_.g1_.z) + (other.g2_.x * self_.g1_.y) - (other.g2_.y * self_.g1_.x) + (other.g2_.z * self_.g1_.w) + (other.g3_.x * self_.g0_.y) - (other.g3_.y * self_.g0_.x) + (other.g3_.z * self_.g0_.w))), /* e23, e31, e12 */ vec3<f32>(((other.g0_.x * self_.g1_.x) + (other.g3_.x * self_.g1_.w) + (other.g3_.y * self_.g1_.z) - (other.g3_.z * self_.g1_.y)), ((other.g0_.x * self_.g1_.y) - (other.g3_.x * self_.g1_.z) + (other.g3_.y * self_.g1_.w) + (other.g3_.z * self_.g1_.x)), ((other.g0_.x * self_.g1_.z) + (other.g3_.x * self_.g1_.y) - (other.g3_.y * self_.g1_.x) + (other.g3_.z * self_.g1_.w))), /* e423, e431, e412, e321 */ vec4<f32>((-(self_.g0_.x * other.g4_.w) + (self_.g0_.y * other.g1_.z) - (self_.g0_.z * other.g1_.y) - (self_.g0_.w * other.g1_.x) + (self_.g1_.x * other.g1_.w) - (self_.g1_.y * other.g4_.z) + (self_.g1_.z * other.g4_.y) + (self_.g1_.w * other.g4_.x)), (-(self_.g0_.x * other.g1_.z) - (self_.g0_.y * other.g4_.w) + (self_.g0_.z * other.g1_.x) - (self_.g0_.w * other.g1_.y) + (self_.g1_.x * other.g4_.z) + (self_.g1_.y * other.g1_.w) - (self_.g1_.z * other.g4_.x) + (self_.g1_.w * other.g4_.y)), ((self_.g0_.x * other.g1_.y) - (self_.g0_.y * other.g1_.x) - (self_.g0_.z * other.g4_.w) - (self_.g0_.w * other.g1_.z) - (self_.g1_.x * other.g4_.y) + (self_.g1_.y * other.g4_.x) + (self_.g1_.z * other.g1_.w) + (self_.g1_.w * other.g4_.z)), (-(self_.g1_.x * other.g1_.x) - (self_.g1_.y * other.g1_.y) - (self_.g1_.z * other.g1_.z) + (self_.g1_.w * other.g4_.w))));
}
fn origin_mul_multiVector(self: Origin, other: MultiVector) -> MultiVector {
    return origin_geometricProduct_origin(self_, other);
}
fn origin_sub_flector(self: Origin, other: Flector) -> Flector {
    let subtraction: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * -1.0), (other.g0_.y * -1.0), (other.g0_.z * -1.0), (-other.g0_.w + self_.g0_)), /* e423, e431, e412, e321 */ (other.g1_ * vec4<f32>(-1.0)));
    return subtraction;
}
fn motor_antiSandwich_origin(self: Motor, other: Origin) -> Flector {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g0_.w)), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g1_.w)));
    return flector_geometricAntiProduct_flector(geometric_anti_product, motor_antiReverse(self_));
}
fn plane_bitXor_dualNum(self: Plane, other: DualNum) -> Plane {
    return plane_wedge_plane(self_, other);
}
fn origin_add_scalar(self: Origin, other: Scalar) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(other.g0_, 0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    return addition;
}
fn antiScalar_mul_scalar(self: AntiScalar, other: Scalar) -> AntiScalar {
    return antiScalar_geometricProduct_antiScalar(self_, other);
}
fn dualNum_geometricProduct_dualNum(self: DualNum, other: DualNum) -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>((other.g0_.x * self_.g0_.x), ((other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x))));
}
fn horizon_sandwich_line(self: Horizon, other: Line) -> Motor {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_), 0.0), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), 0.0));
    return flector_geometricProduct_flector(geometric_product, horizon_reverse(self_));
}
fn antiScalar_bitXor_motor(self: AntiScalar, other: Motor) -> AntiScalar {
    return antiScalar_wedge_antiScalar(self_, other);
}
fn dualNum_geometricAntiProduct_point(self: DualNum, other: Point) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.y * other.g0_.x), (self_.g0_.y * other.g0_.y), (self_.g0_.y * other.g0_.z), (self_.g0_.y * other.g0_.w)), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.x * other.g0_.w)));
}
fn point_geometricAntiQuotient_flector(self: Point, other: Flector) -> Point {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.w, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Point = Point(/* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_));
    return geometric_anti_product;
}
fn line_geometricAntiProduct_flector(self: Line, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(((self_.g0_.x * other.g1_.w) + (self_.g0_.y * other.g0_.z) - (self_.g0_.z * other.g0_.y) + (self_.g1_.x * other.g0_.w) + (self_.g1_.y * other.g1_.z) - (self_.g1_.z * other.g1_.y)), (-(self_.g0_.x * other.g0_.z) + (self_.g0_.y * other.g1_.w) + (self_.g0_.z * other.g0_.x) - (self_.g1_.x * other.g1_.z) + (self_.g1_.y * other.g0_.w) + (self_.g1_.z * other.g1_.x)), ((self_.g0_.x * other.g0_.y) - (self_.g0_.y * other.g0_.x) + (self_.g0_.z * other.g1_.w) + (self_.g1_.x * other.g1_.y) - (self_.g1_.y * other.g1_.x) + (self_.g1_.z * other.g0_.w)), (-(self_.g0_.x * other.g1_.x) - (self_.g0_.y * other.g1_.y) - (self_.g0_.z * other.g1_.z))), /* e423, e431, e412, e321 */ vec4<f32>(((self_.g0_.x * other.g0_.w) + (self_.g0_.y * other.g1_.z) - (self_.g0_.z * other.g1_.y)), (-(self_.g0_.x * other.g1_.z) + (self_.g0_.y * other.g0_.w) + (self_.g0_.z * other.g1_.x)), ((self_.g0_.x * other.g1_.y) - (self_.g0_.y * other.g1_.x) + (self_.g0_.z * other.g0_.w)), (-(self_.g0_.x * other.g0_.x) - (self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z) - (self_.g1_.x * other.g1_.x) - (self_.g1_.y * other.g1_.y) - (self_.g1_.z * other.g1_.z))));
}
fn multiVector_sandwich_line(self: MultiVector, other: Line) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(0.0, (-(other.g1_.x * self_.g2_.x) - (other.g1_.y * self_.g2_.y) - (other.g1_.z * self_.g2_.z))) - (vec2<f32>(self_.g3_.x) * vec2<f32>(other.g1_.x, other.g0_.x)) - (vec2<f32>(self_.g3_.y) * vec2<f32>(other.g1_.y, other.g0_.y)) - (vec2<f32>(self_.g3_.z) * vec2<f32>(other.g1_.z, other.g0_.z))), /* e1, e2, e3, e4 */ (vec4<f32>(((other.g1_.x * self_.g4_.w) + (other.g1_.y * self_.g1_.z)), ((other.g1_.y * self_.g4_.w) + (other.g1_.z * self_.g1_.x)), ((other.g1_.x * self_.g1_.y) + (other.g1_.z * self_.g4_.w)), (-(other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g1_.x * self_.g4_.x) - (other.g1_.y * self_.g4_.y) - (other.g1_.z * self_.g4_.z))) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.x) * self_.g1_.yzxx)), /* e41, e42, e43 */ ((vec3<f32>(self_.g0_.x) * other.g0_) + (vec3<f32>(self_.g0_.y) * other.g1_) + (other.g0_.yzx * self_.g3_.zxy) - (other.g0_.zxy * self_.g3_.yzx) + (other.g1_.yzx * self_.g2_.zxy) - (other.g1_.zxy * self_.g2_.yzx)), /* e23, e31, e12 */ ((vec3<f32>(self_.g0_.x) * other.g1_) + (other.g1_.yzx * self_.g3_.zxy) - (other.g1_.zxy * self_.g3_.yzx)), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.x * self_.g4_.w) + (other.g0_.y * self_.g1_.z) + (other.g1_.x * self_.g1_.w) + (other.g1_.y * self_.g4_.z) - (other.g1_.z * self_.g4_.y)), ((other.g0_.y * self_.g4_.w) + (other.g0_.z * self_.g1_.x) - (other.g1_.x * self_.g4_.z) + (other.g1_.y * self_.g1_.w) + (other.g1_.z * self_.g4_.x)), ((other.g0_.x * self_.g1_.y) + (other.g0_.z * self_.g4_.w) + (other.g1_.x * self_.g4_.y) - (other.g1_.y * self_.g4_.x) + (other.g1_.z * self_.g1_.w)), (-(other.g1_.y * self_.g1_.y) - (other.g1_.z * self_.g1_.z))) - (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.y, other.g1_.x) * self_.g1_.yzxx)));
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self_));
}
fn dualNum_geometricProduct_flector(self: DualNum, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g0_.x), (self_.g0_.x * other.g0_.y), (self_.g0_.x * other.g0_.z), ((self_.g0_.x * other.g0_.w) - (self_.g0_.y * other.g1_.w))), /* e423, e431, e412, e321 */ vec4<f32>(((self_.g0_.x * other.g1_.x) - (self_.g0_.y * other.g0_.x)), ((self_.g0_.x * other.g1_.y) - (self_.g0_.y * other.g0_.y)), ((self_.g0_.x * other.g1_.z) - (self_.g0_.y * other.g0_.z)), (self_.g0_.x * other.g1_.w)));
}
fn origin_geometricQuotient_motor(self: Origin, other: Motor) -> Origin {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2) + pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Origin = Origin(/* e4 */ (self_.g0_ * inverse.g0_));
    return geometric_product;
}
fn multiVector_add_motor(self: MultiVector, other: Motor) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(other.g1_.w, other.g0_.w) + self_.g0_), /* e1, e2, e3, e4 */ self_.g1_, /* e41, e42, e43 */ (vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z) + self_.g2_), /* e23, e31, e12 */ (vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z) + self_.g3_), /* e423, e431, e412, e321 */ self_.g4_);
    return addition;
}
fn flector_sandwich_point(self: Flector, other: Point) -> Flector {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(((self_.g0_.w * other.g0_.x) + (self_.g1_.z * other.g0_.y)), ((self_.g0_.w * other.g0_.y) + (self_.g1_.x * other.g0_.z)), ((self_.g0_.w * other.g0_.z) + (self_.g1_.y * other.g0_.x)), (-(self_.g1_.z * other.g0_.z) - (self_.g1_.w * other.g0_.w))) - (vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g1_.x) * other.g0_.wwwx) - (self_.g1_.yzxy * other.g0_.zxyy)), /* e23, e31, e12, scalar */ (vec4<f32>((-(self_.g0_.z * other.g0_.y) - (self_.g1_.w * other.g0_.x)), (-(self_.g0_.x * other.g0_.z) - (self_.g1_.w * other.g0_.y)), (-(self_.g0_.y * other.g0_.x) - (self_.g1_.w * other.g0_.z)), ((self_.g0_.y * other.g0_.y) + (self_.g0_.z * other.g0_.z))) + (self_.g0_.yzxx * other.g0_.zxyx)));
    return motor_geometricProduct_motor(geometric_product, flector_reverse(self_));
}
fn antiScalar_geometricAntiProduct_horizon(self: AntiScalar, other: Horizon) -> Horizon {
    return Horizon(/* e321 */ (self_.g0_ * other.g0_));
}
fn line_geometricAntiQuotient_motor(self: Line, other: Motor) -> Line {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2) + pow(other.g0_.w, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Line = Line(/* e41, e42, e43 */ (vec3<f32>(anti_inverse.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(anti_inverse.g0_) * self_.g1_));
    return geometric_anti_product;
}
fn motor_wedge_plane(self: Motor, other: Plane) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>((self_.g1_.w * other.g0_.x), (self_.g1_.w * other.g0_.y), (self_.g1_.w * other.g0_.z), (self_.g1_.w * other.g0_.w)));
}
fn flector_sandwich_plane(self: Flector, other: Plane) -> Flector {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>((-(self_.g0_.z * other.g0_.y) - (self_.g1_.w * other.g0_.x)), (-(self_.g0_.x * other.g0_.z) - (self_.g1_.w * other.g0_.y)), (-(self_.g0_.y * other.g0_.x) - (self_.g1_.w * other.g0_.z)), ((self_.g0_.z * other.g0_.z) + (self_.g0_.w * other.g0_.w))) + (vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g0_.y) * other.g0_.wwwy) + (self_.g0_.yzxx * other.g0_.zxyx)), /* e23, e31, e12, scalar */ (vec4<f32>(other.g0_.w) * vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g1_.w) * vec4<f32>(-1.0)));
    return motor_geometricProduct_motor(geometric_product, flector_reverse(self_));
}
fn scalar_wedge_dualNum(self: Scalar, other: DualNum) -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_)));
}
fn plane_antiWedge_motor(self: Plane, other: Motor) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(((other.g0_.x * self_.g0_.w) + (other.g1_.y * self_.g0_.z) - (other.g1_.z * self_.g0_.y)), ((other.g0_.y * self_.g0_.w) - (other.g1_.x * self_.g0_.z) + (other.g1_.z * self_.g0_.x)), ((other.g0_.z * self_.g0_.w) + (other.g1_.x * self_.g0_.y) - (other.g1_.y * self_.g0_.x)), (-(other.g0_.x * self_.g0_.x) - (other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z))), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.w * self_.g0_.x), (other.g0_.w * self_.g0_.y), (other.g0_.w * self_.g0_.z), (other.g0_.w * self_.g0_.w)));
}
fn multiVector_sandwich_multiVector(self: MultiVector, other: MultiVector) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(0.0, ((other.g0_.y * self_.g0_.x) - (other.g3_.x * self_.g2_.x) - (other.g3_.y * self_.g2_.y) - (other.g3_.z * self_.g2_.z) - (other.g1_.x * self_.g4_.x) - (other.g1_.y * self_.g4_.y) - (other.g1_.z * self_.g4_.z) + (other.g4_.w * self_.g1_.w))) + (vec2<f32>(other.g0_.x) * self_.g0_) - (vec2<f32>(self_.g3_.x) * vec2<f32>(other.g3_.x, other.g2_.x)) - (vec2<f32>(self_.g3_.y) * vec2<f32>(other.g3_.y, other.g2_.y)) - (vec2<f32>(self_.g3_.z) * vec2<f32>(other.g3_.z, other.g2_.z)) + (vec2<f32>(self_.g1_.x) * vec2<f32>(other.g1_.x, other.g4_.x)) + (vec2<f32>(self_.g1_.y) * vec2<f32>(other.g1_.y, other.g4_.y)) + (vec2<f32>(self_.g1_.z) * vec2<f32>(other.g1_.z, other.g4_.z)) - (vec2<f32>(self_.g4_.w) * vec2<f32>(other.g4_.w, other.g1_.w))), /* e1, e2, e3, e4 */ (vec4<f32>(((other.g3_.y * self_.g1_.z) + (self_.g3_.x * other.g4_.w) - (self_.g3_.y * other.g1_.z)), ((other.g3_.z * self_.g1_.x) + (self_.g3_.y * other.g4_.w) - (self_.g3_.z * other.g1_.x)), ((other.g3_.x * self_.g1_.y) - (self_.g3_.x * other.g1_.y) + (self_.g3_.z * other.g4_.w)), (-(self_.g0_.y * other.g4_.w) - (other.g2_.y * self_.g1_.y) - (other.g2_.z * self_.g1_.z) - (other.g3_.x * self_.g4_.x) - (other.g3_.y * self_.g4_.y) - (other.g3_.z * self_.g4_.z) + (self_.g2_.y * other.g1_.y) + (self_.g2_.z * other.g1_.z) - (self_.g3_.x * other.g4_.x) - (self_.g3_.y * other.g4_.y) - (self_.g3_.z * other.g4_.z))) + (vec4<f32>(other.g0_.x) * self_.g1_) + (vec4<f32>(self_.g0_.x) * other.g1_) + (vec4<f32>(self_.g4_.w) * vec4<f32>(other.g3_.x, other.g3_.y, other.g3_.z, other.g0_.y)) - (vec4<f32>(other.g3_.z, other.g3_.x, other.g3_.y, other.g2_.x) * self_.g1_.yzxx) + (vec4<f32>(self_.g3_.z, self_.g3_.x, self_.g3_.y, self_.g2_.x) * other.g1_.yzxx)), /* e41, e42, e43 */ (vec3<f32>(((other.g1_.y * self_.g4_.z) - (other.g1_.z * self_.g4_.y) - (other.g4_.y * self_.g1_.z) + (other.g4_.z * self_.g1_.y)), (-(other.g1_.x * self_.g4_.z) + (other.g1_.z * self_.g4_.x) + (other.g4_.x * self_.g1_.z) - (other.g4_.z * self_.g1_.x)), ((other.g1_.x * self_.g4_.y) - (other.g1_.y * self_.g4_.x) - (other.g4_.x * self_.g1_.y) + (other.g4_.y * self_.g1_.x))) + (vec3<f32>(other.g0_.x) * self_.g2_) + (vec3<f32>(other.g0_.y) * self_.g3_) + (vec3<f32>(self_.g0_.x) * other.g2_) + (vec3<f32>(self_.g0_.y) * other.g3_) - (vec3<f32>(other.g1_.w) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z)) + (vec3<f32>(other.g4_.w) * vec3<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z)) + (vec3<f32>(self_.g1_.w) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z)) - (vec3<f32>(self_.g4_.w) * vec3<f32>(other.g4_.x, other.g4_.y, other.g4_.z)) + (other.g2_.yzx * self_.g3_.zxy) - (other.g2_.zxy * self_.g3_.yzx) + (other.g3_.yzx * self_.g2_.zxy) - (other.g3_.zxy * self_.g2_.yzx)), /* e23, e31, e12 */ (vec3<f32>((-(other.g1_.y * self_.g1_.z) + (other.g1_.z * self_.g1_.y)), ((other.g1_.x * self_.g1_.z) - (other.g1_.z * self_.g1_.x)), (-(other.g1_.x * self_.g1_.y) + (other.g1_.y * self_.g1_.x))) + (vec3<f32>(other.g0_.x) * self_.g3_) + (vec3<f32>(self_.g0_.x) * other.g3_) - (vec3<f32>(other.g4_.w) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z)) - (vec3<f32>(self_.g4_.w) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z)) + (other.g3_.yzx * self_.g3_.zxy) - (other.g3_.zxy * self_.g3_.yzx)), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.y * self_.g1_.x) + (other.g2_.x * self_.g4_.w) + (other.g2_.y * self_.g1_.z) + (other.g3_.x * self_.g1_.w) + (other.g3_.y * self_.g4_.z) - (other.g3_.z * self_.g4_.y) - (self_.g2_.x * other.g4_.w) + (self_.g2_.y * other.g1_.z) + (self_.g3_.x * other.g1_.w) - (self_.g3_.y * other.g4_.z) + (self_.g3_.z * other.g4_.y)), ((other.g0_.y * self_.g1_.y) + (other.g2_.y * self_.g4_.w) + (other.g2_.z * self_.g1_.x) - (other.g3_.x * self_.g4_.z) + (other.g3_.y * self_.g1_.w) + (other.g3_.z * self_.g4_.x) - (self_.g2_.y * other.g4_.w) + (self_.g2_.z * other.g1_.x) + (self_.g3_.x * other.g4_.z) + (self_.g3_.y * other.g1_.w) - (self_.g3_.z * other.g4_.x)), ((other.g0_.y * self_.g1_.z) + (other.g2_.x * self_.g1_.y) + (other.g2_.z * self_.g4_.w) + (other.g3_.x * self_.g4_.y) - (other.g3_.y * self_.g4_.x) + (other.g3_.z * self_.g1_.w) + (self_.g2_.x * other.g1_.y) - (self_.g2_.z * other.g4_.w) - (self_.g3_.x * other.g4_.y) + (self_.g3_.y * other.g4_.x) + (self_.g3_.z * other.g1_.w)), (-(other.g3_.y * self_.g1_.y) - (other.g3_.z * self_.g1_.z) - (self_.g3_.z * other.g1_.z))) + (vec4<f32>(other.g0_.x) * self_.g4_) + (vec4<f32>(self_.g0_.x) * other.g4_) - (vec4<f32>(self_.g0_.y, self_.g0_.y, self_.g0_.y, self_.g3_.x) * other.g1_.xyzx) - (vec4<f32>(other.g2_.z, other.g2_.x, other.g2_.y, other.g3_.x) * self_.g1_.yzxx) - (vec4<f32>(self_.g2_.z, self_.g2_.x, self_.g2_.y, self_.g3_.y) * other.g1_.yzxy)));
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self_));
}
fn horizon_antiWedge_origin(self: Horizon, other: Origin) -> Scalar {
    return Scalar(/* scalar */ (self_.g0_ * other.g0_ * -1.0));
}
fn origin_antiWedge_flector(self: Origin, other: Flector) -> Scalar {
    return Scalar(/* scalar */ (other.g1_.w * self_.g0_));
}
fn antiScalar_antiSandwich_multiVector(self: AntiScalar, other: MultiVector) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(self_.g0_) * other.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_) * other.g1_), /* e41, e42, e43 */ (vec3<f32>(self_.g0_) * other.g2_), /* e23, e31, e12 */ (vec3<f32>(self_.g0_) * other.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_) * other.g4_));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, antiScalar_antiReverse(self_));
}
fn antiScalar_geometricQuotient_motor(self: AntiScalar, other: Motor) -> AntiScalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2) + pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: AntiScalar = AntiScalar(/* e1234 */ (self_.g0_ * inverse.g0_));
    return geometric_product;
}
fn scalar_bitXor_plane(self: Scalar, other: Plane) -> Plane {
    return scalar_wedge_scalar(self_, other);
}
fn multiVector_geometricAntiProduct_origin(self: MultiVector, other: Origin) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((self_.g4_.w * other.g0_ * -1.0), (self_.g1_.w * other.g0_ * -1.0)), /* e1, e2, e3, e4 */ vec4<f32>((self_.g3_.x * other.g0_), (self_.g3_.y * other.g0_), (self_.g3_.z * other.g0_), (self_.g0_.y * other.g0_)), /* e41, e42, e43 */ vec3<f32>((self_.g4_.x * other.g0_ * -1.0), (self_.g4_.y * other.g0_ * -1.0), (self_.g4_.z * other.g0_ * -1.0)), /* e23, e31, e12 */ vec3<f32>((self_.g1_.x * other.g0_ * -1.0), (self_.g1_.y * other.g0_ * -1.0), (self_.g1_.z * other.g0_ * -1.0)), /* e423, e431, e412, e321 */ vec4<f32>((self_.g2_.x * other.g0_), (self_.g2_.y * other.g0_), (self_.g2_.z * other.g0_), (self_.g0_.x * other.g0_)));
}
fn flector_geometricAntiProduct_flector(self: Flector, other: Flector) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((-(other.g0_.w * self_.g1_.x) - (other.g1_.x * self_.g0_.w) + (other.g1_.y * self_.g1_.z) - (other.g1_.z * self_.g1_.y)), (-(other.g0_.w * self_.g1_.y) - (other.g1_.x * self_.g1_.z) - (other.g1_.y * self_.g0_.w) + (other.g1_.z * self_.g1_.x)), (-(other.g0_.w * self_.g1_.z) + (other.g1_.x * self_.g1_.y) - (other.g1_.y * self_.g1_.x) - (other.g1_.z * self_.g0_.w)), (-(other.g0_.w * self_.g0_.w) + (other.g1_.x * self_.g1_.x) + (other.g1_.y * self_.g1_.y) + (other.g1_.z * self_.g1_.z))), /* e23, e31, e12, scalar */ vec4<f32>(((other.g0_.x * self_.g0_.w) - (other.g0_.y * self_.g1_.z) + (other.g0_.z * self_.g1_.y) - (other.g0_.w * self_.g0_.x) - (other.g1_.x * self_.g1_.w) + (other.g1_.y * self_.g0_.z) - (other.g1_.z * self_.g0_.y) + (other.g1_.w * self_.g1_.x)), ((other.g0_.x * self_.g1_.z) + (other.g0_.y * self_.g0_.w) - (other.g0_.z * self_.g1_.x) - (other.g0_.w * self_.g0_.y) - (other.g1_.x * self_.g0_.z) - (other.g1_.y * self_.g1_.w) + (other.g1_.z * self_.g0_.x) + (other.g1_.w * self_.g1_.y)), (-(other.g0_.x * self_.g1_.y) + (other.g0_.y * self_.g1_.x) + (other.g0_.z * self_.g0_.w) - (other.g0_.w * self_.g0_.z) + (other.g1_.x * self_.g0_.y) - (other.g1_.y * self_.g0_.x) - (other.g1_.z * self_.g1_.w) + (other.g1_.w * self_.g1_.z)), (-(other.g0_.x * self_.g1_.x) - (other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g0_.w * self_.g1_.w) + (other.g1_.x * self_.g0_.x) + (other.g1_.y * self_.g0_.y) + (other.g1_.z * self_.g0_.z) + (other.g1_.w * self_.g0_.w))));
}
fn line_mul_antiScalar(self: Line, other: AntiScalar) -> Line {
    return line_geometricProduct_line(self_, other);
}
fn plane_geometricProduct_scalar(self: Plane, other: Scalar) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), (self_.g0_.w * other.g0_)));
}
fn origin_geometricProduct_plane(self: Origin, other: Plane) -> AntiScalar {
    return AntiScalar(/* e1234 */ (other.g0_.w * self_.g0_));
}
fn flector_geometricQuotient_motor(self: Flector, other: Motor) -> Flector {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2) + pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g1_));
    return geometric_product;
}
fn plane_add_dualNum(self: Plane, other: DualNum) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ other.g0_, /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ self_.g0_);
    return addition;
}
fn dualNum_geometricAntiQuotient_dualNum(self: DualNum, other: DualNum) -> DualNum {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_.y, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>(anti_inverse.g0_) * self_.g0_));
    return geometric_anti_product;
}
fn dualNum_wedge_motor(self: DualNum, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g0_.x * other.g0_.x), (self_.g0_.x * other.g0_.y), (self_.g0_.x * other.g0_.z), ((self_.g0_.x * other.g0_.w) + (self_.g0_.y * other.g1_.w))), /* e23, e31, e12, scalar */ vec4<f32>((self_.g0_.x * other.g1_.x), (self_.g0_.x * other.g1_.y), (self_.g0_.x * other.g1_.z), (self_.g0_.x * other.g1_.w)));
}
fn plane_antiWedge_point(self: Plane, other: Point) -> Scalar {
    return Scalar(/* scalar */ (-(self_.g0_.x * other.g0_.x) - (self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z) - (self_.g0_.w * other.g0_.w)));
}
fn origin_add_flector(self: Origin, other: Flector) -> Flector {
    let addition: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, (other.g0_.w + self_.g0_)), /* e423, e431, e412, e321 */ other.g1_);
    return addition;
}
fn scalar_sub_line(self: Scalar, other: Line) -> Motor {
    let subtraction: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g0_.x * -1.0), (other.g0_.y * -1.0), (other.g0_.z * -1.0), 0.0), /* e23, e31, e12, scalar */ vec4<f32>((other.g1_.x * -1.0), (other.g1_.y * -1.0), (other.g1_.z * -1.0), self_.g0_));
    return subtraction;
}
fn motor_antiSandwich_line(self: Motor, other: Line) -> Motor {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(((other.g0_.x * self_.g0_.w) + (other.g0_.z * self_.g0_.y)), ((other.g0_.x * self_.g0_.z) + (other.g0_.y * self_.g0_.w)), ((other.g0_.y * self_.g0_.x) + (other.g0_.z * self_.g0_.w)), (-(other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z))) - (vec4<f32>(other.g0_.y, other.g0_.z, other.g0_.x, other.g0_.x) * self_.g0_.zxyx)), /* e23, e31, e12, scalar */ (vec4<f32>(((other.g0_.x * self_.g1_.w) + (other.g0_.z * self_.g1_.y) + (other.g1_.x * self_.g0_.w) + (other.g1_.z * self_.g0_.y)), ((other.g0_.x * self_.g1_.z) + (other.g0_.y * self_.g1_.w) + (other.g1_.x * self_.g0_.z) + (other.g1_.y * self_.g0_.w)), ((other.g0_.y * self_.g1_.x) + (other.g0_.z * self_.g1_.w) + (other.g1_.y * self_.g0_.x) + (other.g1_.z * self_.g0_.w)), (-(other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))) - (vec4<f32>(other.g0_.y, other.g0_.z, other.g0_.x, other.g0_.x) * self_.g1_.zxyx) - (vec4<f32>(other.g1_.y, other.g1_.z, other.g1_.x, other.g1_.x) * self_.g0_.zxyx)));
    return motor_geometricAntiProduct_motor(geometric_anti_product, motor_antiReverse(self_));
}
fn origin_geometricAntiProduct_antiScalar(self: Origin, other: AntiScalar) -> Origin {
    return Origin(/* e4 */ (other.g0_ * self_.g0_));
}
fn line_sandwich_plane(self: Line, other: Plane) -> Flector {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g1_.x * other.g0_.w), (self_.g1_.y * other.g0_.w), (self_.g1_.z * other.g0_.w), (-(self_.g1_.x * other.g0_.x) - (self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))), /* e423, e431, e412, e321 */ vec4<f32>((-(self_.g0_.x * other.g0_.w) - (self_.g1_.y * other.g0_.z) + (self_.g1_.z * other.g0_.y)), (-(self_.g0_.y * other.g0_.w) + (self_.g1_.x * other.g0_.z) - (self_.g1_.z * other.g0_.x)), (-(self_.g0_.z * other.g0_.w) - (self_.g1_.x * other.g0_.y) + (self_.g1_.y * other.g0_.x)), 0.0));
    return flector_geometricProduct_flector(geometric_product, line_reverse(self_));
}
fn origin_sandwich_flector(self: Origin, other: Flector) -> Flector {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.w)), /* e23, e31, e12, scalar */ vec4<f32>(0.0));
    return motor_geometricProduct_motor(geometric_product, origin_reverse(self_));
}
fn flector_antiSandwich_line(self: Flector, other: Line) -> Motor {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(((other.g0_.x * self_.g1_.w) - (other.g0_.y * self_.g0_.z) + (other.g0_.z * self_.g0_.y) - (other.g1_.x * self_.g0_.w) + (other.g1_.y * self_.g1_.z)), ((other.g0_.x * self_.g0_.z) + (other.g0_.y * self_.g1_.w) - (other.g0_.z * self_.g0_.x) - (other.g1_.y * self_.g0_.w) + (other.g1_.z * self_.g1_.x)), (-(other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x) + (other.g0_.z * self_.g1_.w) + (other.g1_.x * self_.g1_.y) - (other.g1_.z * self_.g0_.w)), (-(other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z))) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.x) * self_.g1_.yzxx)), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.x * self_.g0_.w) - (other.g0_.y * self_.g1_.z)), ((other.g0_.y * self_.g0_.w) - (other.g0_.z * self_.g1_.x)), (-(other.g0_.x * self_.g1_.y) + (other.g0_.z * self_.g0_.w)), (-(other.g0_.x * self_.g0_.x) - (other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z) + (other.g1_.y * self_.g1_.y) + (other.g1_.z * self_.g1_.z))) + (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.y, other.g1_.x) * self_.g1_.yzxx)));
    return flector_geometricAntiProduct_flector(geometric_anti_product, flector_antiReverse(self_));
}
fn multiVector_bitXor_line(self: MultiVector, other: Line) -> MultiVector {
    return multiVector_wedge_multiVector(self_, other);
}
fn antiScalar_geometricAntiQuotient_dualNum(self: AntiScalar, other: DualNum) -> AntiScalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_.y, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: AntiScalar = AntiScalar(/* e1234 */ (anti_inverse.g0_ * self_.g0_));
    return geometric_anti_product;
}
fn line_geometricAntiProduct_multiVector(self: Line, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((-(self_.g0_.x * other.g3_.x) - (self_.g0_.y * other.g3_.y) - (self_.g0_.z * other.g3_.z) - (self_.g1_.x * other.g2_.x) - (self_.g1_.y * other.g2_.y) - (self_.g1_.z * other.g2_.z)), (-(self_.g0_.x * other.g2_.x) - (self_.g0_.y * other.g2_.y) - (self_.g0_.z * other.g2_.z))), /* e1, e2, e3, e4 */ vec4<f32>(((self_.g0_.x * other.g4_.w) + (self_.g0_.y * other.g1_.z) - (self_.g0_.z * other.g1_.y) + (self_.g1_.x * other.g1_.w) + (self_.g1_.y * other.g4_.z) - (self_.g1_.z * other.g4_.y)), (-(self_.g0_.x * other.g1_.z) + (self_.g0_.y * other.g4_.w) + (self_.g0_.z * other.g1_.x) - (self_.g1_.x * other.g4_.z) + (self_.g1_.y * other.g1_.w) + (self_.g1_.z * other.g4_.x)), ((self_.g0_.x * other.g1_.y) - (self_.g0_.y * other.g1_.x) + (self_.g0_.z * other.g4_.w) + (self_.g1_.x * other.g4_.y) - (self_.g1_.y * other.g4_.x) + (self_.g1_.z * other.g1_.w)), (-(self_.g0_.x * other.g4_.x) - (self_.g0_.y * other.g4_.y) - (self_.g0_.z * other.g4_.z))), /* e41, e42, e43 */ vec3<f32>(((other.g0_.y * self_.g0_.x) + (self_.g0_.y * other.g2_.z) - (self_.g0_.z * other.g2_.y)), ((other.g0_.y * self_.g0_.y) - (self_.g0_.x * other.g2_.z) + (self_.g0_.z * other.g2_.x)), ((other.g0_.y * self_.g0_.z) + (self_.g0_.x * other.g2_.y) - (self_.g0_.y * other.g2_.x))), /* e23, e31, e12 */ vec3<f32>(((other.g0_.x * self_.g0_.x) + (other.g0_.y * self_.g1_.x) + (self_.g0_.y * other.g3_.z) - (self_.g0_.z * other.g3_.y) + (self_.g1_.y * other.g2_.z) - (self_.g1_.z * other.g2_.y)), ((other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g1_.y) - (self_.g0_.x * other.g3_.z) + (self_.g0_.z * other.g3_.x) - (self_.g1_.x * other.g2_.z) + (self_.g1_.z * other.g2_.x)), ((other.g0_.x * self_.g0_.z) + (other.g0_.y * self_.g1_.z) + (self_.g0_.x * other.g3_.y) - (self_.g0_.y * other.g3_.x) + (self_.g1_.x * other.g2_.y) - (self_.g1_.y * other.g2_.x))), /* e423, e431, e412, e321 */ vec4<f32>(((self_.g0_.x * other.g1_.w) + (self_.g0_.y * other.g4_.z) - (self_.g0_.z * other.g4_.y)), (-(self_.g0_.x * other.g4_.z) + (self_.g0_.y * other.g1_.w) + (self_.g0_.z * other.g4_.x)), ((self_.g0_.x * other.g4_.y) - (self_.g0_.y * other.g4_.x) + (self_.g0_.z * other.g1_.w)), (-(self_.g0_.x * other.g1_.x) - (self_.g0_.y * other.g1_.y) - (self_.g0_.z * other.g1_.z) - (self_.g1_.x * other.g4_.x) - (self_.g1_.y * other.g4_.y) - (self_.g1_.z * other.g4_.z))));
}
fn flector_wedge_point(self: Flector, other: Point) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((-(self_.g0_.x * other.g0_.w) + (self_.g0_.w * other.g0_.x)), (-(self_.g0_.y * other.g0_.w) + (self_.g0_.w * other.g0_.y)), (-(self_.g0_.z * other.g0_.w) + (self_.g0_.w * other.g0_.z)), (-(self_.g1_.x * other.g0_.x) - (self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z) - (self_.g1_.w * other.g0_.w))), /* e23, e31, e12, scalar */ vec4<f32>(((self_.g0_.y * other.g0_.z) - (self_.g0_.z * other.g0_.y)), (-(self_.g0_.x * other.g0_.z) + (self_.g0_.z * other.g0_.x)), ((self_.g0_.x * other.g0_.y) - (self_.g0_.y * other.g0_.x)), 0.0));
}
fn line_antiSandwich_multiVector(self: Line, other: MultiVector) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>((-(self_.g1_.x * other.g2_.x) - (self_.g1_.y * other.g2_.y) - (self_.g1_.z * other.g2_.z)), 0.0) - (vec2<f32>(self_.g0_.x) * vec2<f32>(other.g3_.x, other.g2_.x)) - (vec2<f32>(self_.g0_.y) * vec2<f32>(other.g3_.y, other.g2_.y)) - (vec2<f32>(self_.g0_.z) * vec2<f32>(other.g3_.z, other.g2_.z))), /* e1, e2, e3, e4 */ (vec4<f32>(((self_.g0_.x * other.g4_.w) + (self_.g0_.y * other.g1_.z) - (self_.g0_.z * other.g1_.y) + (self_.g1_.x * other.g1_.w) + (self_.g1_.y * other.g4_.z)), (-(self_.g0_.x * other.g1_.z) + (self_.g0_.y * other.g4_.w) + (self_.g0_.z * other.g1_.x) + (self_.g1_.y * other.g1_.w) + (self_.g1_.z * other.g4_.x)), ((self_.g0_.x * other.g1_.y) - (self_.g0_.y * other.g1_.x) + (self_.g0_.z * other.g4_.w) + (self_.g1_.x * other.g4_.y) + (self_.g1_.z * other.g1_.w)), (-(self_.g0_.y * other.g4_.y) - (self_.g0_.z * other.g4_.z))) - (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g0_.x) * other.g4_.yzxx)), /* e41, e42, e43 */ ((vec3<f32>(other.g0_.y) * self_.g0_) + (self_.g0_.yzx * other.g2_.zxy) - (self_.g0_.zxy * other.g2_.yzx)), /* e23, e31, e12 */ ((vec3<f32>(other.g0_.x) * self_.g0_) + (vec3<f32>(other.g0_.y) * self_.g1_) + (self_.g0_.yzx * other.g3_.zxy) - (self_.g0_.zxy * other.g3_.yzx) + (self_.g1_.yzx * other.g2_.zxy) - (self_.g1_.zxy * other.g2_.yzx)), /* e423, e431, e412, e321 */ (vec4<f32>(((self_.g0_.x * other.g1_.w) + (self_.g0_.y * other.g4_.z)), ((self_.g0_.y * other.g1_.w) + (self_.g0_.z * other.g4_.x)), ((self_.g0_.x * other.g4_.y) + (self_.g0_.z * other.g1_.w)), (-(self_.g0_.x * other.g1_.x) - (self_.g0_.y * other.g1_.y) - (self_.g0_.z * other.g1_.z) - (self_.g1_.y * other.g4_.y) - (self_.g1_.z * other.g4_.z))) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g1_.x) * other.g4_.yzxx)));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, line_antiReverse(self_));
}
fn flector_geometricAntiProduct_horizon(self: Flector, other: Horizon) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0), /* e23, e31, e12, scalar */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), (self_.g0_.w * other.g0_)));
}
fn plane_antiSandwich_plane(self: Plane, other: Plane) -> Flector {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(((other.g0_.z * self_.g0_.y) * -1.0), ((other.g0_.x * self_.g0_.z) * -1.0), ((other.g0_.y * self_.g0_.x) * -1.0), ((other.g0_.y * self_.g0_.y) + (other.g0_.z * self_.g0_.z))) + (other.g0_.yzxx * self_.g0_.zxyx)), /* e23, e31, e12, scalar */ vec4<f32>((-(other.g0_.x * self_.g0_.w) + (other.g0_.w * self_.g0_.x)), (-(other.g0_.y * self_.g0_.w) + (other.g0_.w * self_.g0_.y)), (-(other.g0_.z * self_.g0_.w) + (other.g0_.w * self_.g0_.z)), 0.0));
    return motor_geometricAntiProduct_motor(geometric_anti_product, plane_antiReverse(self_));
}
fn origin_mul_motor(self: Origin, other: Motor) -> Flector {
    return origin_geometricProduct_origin(self_, other);
}
fn point_mul_horizon(self: Point, other: Horizon) -> Motor {
    return point_geometricProduct_point(self_, other);
}
fn horizon_geometricQuotient_scalar(self: Horizon, other: Scalar) -> Horizon {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Horizon = Horizon(/* e321 */ (self_.g0_ * inverse.g0_));
    return geometric_product;
}
fn horizon_mul_dualNum(self: Horizon, other: DualNum) -> Flector {
    return horizon_geometricProduct_horizon(self_, other);
}
fn line_geometricProduct_point(self: Line, other: Point) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((-(self_.g1_.y * other.g0_.z) + (self_.g1_.z * other.g0_.y)), ((self_.g1_.x * other.g0_.z) - (self_.g1_.z * other.g0_.x)), (-(self_.g1_.x * other.g0_.y) + (self_.g1_.y * other.g0_.x)), ((self_.g0_.x * other.g0_.x) + (self_.g0_.y * other.g0_.y) + (self_.g0_.z * other.g0_.z))), /* e423, e431, e412, e321 */ vec4<f32>(((self_.g0_.y * other.g0_.z) - (self_.g0_.z * other.g0_.y) + (self_.g1_.x * other.g0_.w)), (-(self_.g0_.x * other.g0_.z) + (self_.g0_.z * other.g0_.x) + (self_.g1_.y * other.g0_.w)), ((self_.g0_.x * other.g0_.y) - (self_.g0_.y * other.g0_.x) + (self_.g1_.z * other.g0_.w)), (-(self_.g1_.x * other.g0_.x) - (self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))));
}
fn motor_sandwich_plane(self: Motor, other: Plane) -> Flector {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g1_.x * other.g0_.w), (self_.g1_.y * other.g0_.w), (self_.g1_.z * other.g0_.w), (-(self_.g0_.w * other.g0_.w) - (self_.g1_.x * other.g0_.x) - (self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))), /* e423, e431, e412, e321 */ vec4<f32>((-(self_.g0_.x * other.g0_.w) - (self_.g1_.y * other.g0_.z) + (self_.g1_.z * other.g0_.y) + (self_.g1_.w * other.g0_.x)), (-(self_.g0_.y * other.g0_.w) + (self_.g1_.x * other.g0_.z) - (self_.g1_.z * other.g0_.x) + (self_.g1_.w * other.g0_.y)), (-(self_.g0_.z * other.g0_.w) - (self_.g1_.x * other.g0_.y) + (self_.g1_.y * other.g0_.x) + (self_.g1_.w * other.g0_.z)), (self_.g1_.w * other.g0_.w)));
    return flector_geometricProduct_flector(geometric_product, motor_reverse(self_));
}
fn horizon_add_flector(self: Horizon, other: Flector) -> Flector {
    let addition: Flector = Flector(/* e1, e2, e3, e4 */ other.g0_, /* e423, e431, e412, e321 */ vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, (other.g1_.w + self_.g0_)));
    return addition;
}
fn antiScalar_antiWedge_plane(self: AntiScalar, other: Plane) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), (other.g0_.w * self_.g0_)));
}
fn flector_bitXor_origin(self: Flector, other: Origin) -> Motor {
    return flector_wedge_flector(self_, other);
}
fn dualNum_add_multiVector(self: DualNum, other: MultiVector) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ (self_.g0_ + other.g0_), /* e1, e2, e3, e4 */ other.g1_, /* e41, e42, e43 */ other.g2_, /* e23, e31, e12 */ other.g3_, /* e423, e431, e412, e321 */ other.g4_);
    return addition;
}
fn horizon_antiWedge_multiVector(self: Horizon, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((other.g1_.w * self_.g0_ * -1.0), 0.0), /* e1, e2, e3, e4 */ vec4<f32>((other.g2_.x * self_.g0_), (other.g2_.y * self_.g0_), (other.g2_.z * self_.g0_), 0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>((other.g4_.x * self_.g0_ * -1.0), (other.g4_.y * self_.g0_ * -1.0), (other.g4_.z * self_.g0_ * -1.0)), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.y * self_.g0_)));
}
fn plane_geometricAntiProduct_dualNum(self: Plane, other: DualNum) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g0_.x * -1.0), (other.g0_.x * self_.g0_.y * -1.0), (other.g0_.x * self_.g0_.z * -1.0), 0.0), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.y * self_.g0_.x), (other.g0_.y * self_.g0_.y), (other.g0_.y * self_.g0_.z), (other.g0_.y * self_.g0_.w)));
}
fn dualNum_wedge_horizon(self: DualNum, other: Horizon) -> Horizon {
    return Horizon(/* e321 */ (self_.g0_.x * other.g0_));
}
fn line_antiWedge_flector(self: Line, other: Flector) -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>(((self_.g0_.x * other.g1_.w) + (self_.g1_.y * other.g1_.z) - (self_.g1_.z * other.g1_.y)), ((self_.g0_.y * other.g1_.w) - (self_.g1_.x * other.g1_.z) + (self_.g1_.z * other.g1_.x)), ((self_.g0_.z * other.g1_.w) + (self_.g1_.x * other.g1_.y) - (self_.g1_.y * other.g1_.x)), (-(self_.g0_.x * other.g1_.x) - (self_.g0_.y * other.g1_.y) - (self_.g0_.z * other.g1_.z))));
}
fn multiVector_geometricQuotient_scalar(self: MultiVector, other: Scalar) -> MultiVector {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(inverse.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(inverse.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(inverse.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g4_));
    return geometric_product;
}
fn horizon_add_origin(self: Horizon, other: Origin) -> Flector {
    let addition: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other.g0_), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_));
    return addition;
}
fn point_mul_scalar(self: Point, other: Scalar) -> Point {
    return point_geometricProduct_point(self_, other);
}
fn plane_geometricProduct_multiVector(self: Plane, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((other.g4_.w * self_.g0_.w * -1.0), (-(other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z) - (other.g1_.w * self_.g0_.w))), /* e1, e2, e3, e4 */ vec4<f32>((other.g3_.x * self_.g0_.w), (other.g3_.y * self_.g0_.w), (other.g3_.z * self_.g0_.w), ((other.g0_.y * self_.g0_.w) - (other.g3_.x * self_.g0_.x) - (other.g3_.y * self_.g0_.y) - (other.g3_.z * self_.g0_.z))), /* e41, e42, e43 */ vec3<f32>(((other.g1_.y * self_.g0_.z) - (other.g1_.z * self_.g0_.y) - (other.g4_.x * self_.g0_.w) + (other.g4_.w * self_.g0_.x)), (-(other.g1_.x * self_.g0_.z) + (other.g1_.z * self_.g0_.x) - (other.g4_.y * self_.g0_.w) + (other.g4_.w * self_.g0_.y)), ((other.g1_.x * self_.g0_.y) - (other.g1_.y * self_.g0_.x) - (other.g4_.z * self_.g0_.w) + (other.g4_.w * self_.g0_.z))), /* e23, e31, e12 */ vec3<f32>((other.g1_.x * self_.g0_.w * -1.0), (other.g1_.y * self_.g0_.w * -1.0), (other.g1_.z * self_.g0_.w * -1.0)), /* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.x * self_.g0_.x) + (other.g2_.x * self_.g0_.w) + (other.g3_.y * self_.g0_.z) - (other.g3_.z * self_.g0_.y)), ((other.g0_.x * self_.g0_.y) + (other.g2_.y * self_.g0_.w) - (other.g3_.x * self_.g0_.z) + (other.g3_.z * self_.g0_.x)), ((other.g0_.x * self_.g0_.z) + (other.g2_.z * self_.g0_.w) + (other.g3_.x * self_.g0_.y) - (other.g3_.y * self_.g0_.x)), (other.g0_.x * self_.g0_.w)));
}
fn point_sandwich_multiVector(self: Point, other: MultiVector) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(0.0, (other.g4_.w * self_.g0_.w)) + (vec2<f32>(self_.g0_.x) * vec2<f32>(other.g1_.x, other.g4_.x)) + (vec2<f32>(self_.g0_.y) * vec2<f32>(other.g1_.y, other.g4_.y)) + (vec2<f32>(self_.g0_.z) * vec2<f32>(other.g1_.z, other.g4_.z))), /* e1, e2, e3, e4 */ (vec4<f32>((other.g3_.y * self_.g0_.z), (other.g3_.z * self_.g0_.x), (other.g3_.x * self_.g0_.y), (-(other.g2_.y * self_.g0_.y) - (other.g2_.z * self_.g0_.z))) + (vec4<f32>(other.g0_.x) * self_.g0_) - (vec4<f32>(other.g3_.z, other.g3_.x, other.g3_.y, other.g2_.x) * self_.g0_.yzxx)), /* e41, e42, e43 */ (vec3<f32>((-(other.g4_.y * self_.g0_.z) + (other.g4_.z * self_.g0_.y)), ((other.g4_.x * self_.g0_.z) - (other.g4_.z * self_.g0_.x)), (-(other.g4_.x * self_.g0_.y) + (other.g4_.y * self_.g0_.x))) - (vec3<f32>(other.g1_.w) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z)) + (vec3<f32>(self_.g0_.w) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z))), /* e23, e31, e12 */ (vec3<f32>((-(other.g1_.y * self_.g0_.z) + (other.g1_.z * self_.g0_.y)), ((other.g1_.x * self_.g0_.z) - (other.g1_.z * self_.g0_.x)), (-(other.g1_.x * self_.g0_.y) + (other.g1_.y * self_.g0_.x))) - (vec3<f32>(other.g4_.w) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z))), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.y * self_.g0_.x) + (other.g2_.y * self_.g0_.z) + (other.g3_.x * self_.g0_.w)), ((other.g0_.y * self_.g0_.y) + (other.g2_.z * self_.g0_.x) + (other.g3_.y * self_.g0_.w)), ((other.g0_.y * self_.g0_.z) + (other.g2_.x * self_.g0_.y) + (other.g3_.z * self_.g0_.w)), (-(other.g3_.y * self_.g0_.y) - (other.g3_.z * self_.g0_.z))) - (vec4<f32>(other.g2_.z, other.g2_.x, other.g2_.y, other.g3_.x) * self_.g0_.yzxx)));
    return multiVector_geometricProduct_multiVector(geometric_product, point_reverse(self_));
}
fn scalar_sub_multiVector(self: Scalar, other: MultiVector) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>((-other.g0_.x + self_.g0_), (other.g0_.y * -1.0)), /* e1, e2, e3, e4 */ (other.g1_ * vec4<f32>(-1.0)), /* e41, e42, e43 */ (other.g2_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (other.g3_ * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ (other.g4_ * vec4<f32>(-1.0)));
    return subtraction;
}
fn motor_sub_motor(self: Motor, other: Motor) -> Motor {
    let subtraction: Motor = Motor(/* e41, e42, e43, e1234 */ (-other.g0_ + self_.g0_), /* e23, e31, e12, scalar */ (-other.g1_ + self_.g1_));
    return subtraction;
}
fn motor_bitXor_dualNum(self: Motor, other: DualNum) -> Motor {
    return motor_wedge_motor(self_, other);
}
fn antiScalar_geometricQuotient_scalar(self: AntiScalar, other: Scalar) -> AntiScalar {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: AntiScalar = AntiScalar(/* e1234 */ (self_.g0_ * inverse.g0_));
    return geometric_product;
}
fn multiVector_bitXor_origin(self: MultiVector, other: Origin) -> MultiVector {
    return multiVector_wedge_multiVector(self_, other);
}
fn flector_geometricProduct_scalar(self: Flector, other: Scalar) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), (self_.g0_.w * other.g0_)), /* e423, e431, e412, e321 */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), (self_.g1_.w * other.g0_)));
}
fn dualNum_geometricProduct_origin(self: DualNum, other: Origin) -> Origin {
    return Origin(/* e4 */ (self_.g0_.x * other.g0_));
}
fn dualNum_add_plane(self: DualNum, other: Plane) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ other.g0_);
    return addition;
}
fn dualNum_mul_multiVector(self: DualNum, other: MultiVector) -> MultiVector {
    return dualNum_geometricProduct_dualNum(self_, other);
}
fn plane_wedge_motor(self: Plane, other: Motor) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>((other.g1_.w * self_.g0_.x), (other.g1_.w * self_.g0_.y), (other.g1_.w * self_.g0_.z), (other.g1_.w * self_.g0_.w)));
}
fn motor_geometricAntiProduct_horizon(self: Motor, other: Horizon) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), 0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.w * other.g0_)));
}
fn horizon_antiWedge_flector(self: Horizon, other: Flector) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0), /* e23, e31, e12, scalar */ vec4<f32>((other.g1_.x * self_.g0_ * -1.0), (other.g1_.y * self_.g0_ * -1.0), (other.g1_.z * self_.g0_ * -1.0), (other.g0_.w * self_.g0_ * -1.0)));
}
fn antiScalar_antiSandwich_line(self: AntiScalar, other: Line) -> Line {
    let geometric_anti_product: Line = Line(/* e41, e42, e43 */ (vec3<f32>(self_.g0_) * other.g0_), /* e23, e31, e12 */ (vec3<f32>(self_.g0_) * other.g1_));
    return line_geometricAntiProduct_line(geometric_anti_product, antiScalar_antiReverse(self_));
}
fn dualNum_antiSandwich_flector(self: DualNum, other: Flector) -> Flector {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>(((self_.g0_.x * other.g1_.x) + (self_.g0_.y * other.g0_.x)), ((self_.g0_.x * other.g1_.y) + (self_.g0_.y * other.g0_.y)), ((self_.g0_.x * other.g1_.z) + (self_.g0_.y * other.g0_.z)), (self_.g0_.y * other.g0_.w)), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.y * other.g1_.x), (self_.g0_.y * other.g1_.y), (self_.g0_.y * other.g1_.z), ((self_.g0_.x * other.g0_.w) + (self_.g0_.y * other.g1_.w))));
    return flector_geometricAntiProduct_flector(geometric_anti_product, dualNum_antiReverse(self_));
}
fn scalar_geometricQuotient_multiVector(self: Scalar, other: MultiVector) -> Scalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) - pow(other.g3_.x, 2) - pow(other.g3_.y, 2) - pow(other.g3_.z, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2) - pow(other.g4_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Scalar = Scalar(/* scalar */ (inverse.g0_ * self_.g0_));
    return geometric_product;
}
fn point_sub_horizon(self: Point, other: Horizon) -> Flector {
    let subtraction: Flector = Flector(/* e1, e2, e3, e4 */ self_.g0_, /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_ * -1.0)));
    return subtraction;
}
fn multiVector_sub_multiVector(self: MultiVector, other: MultiVector) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ (-other.g0_ + self_.g0_), /* e1, e2, e3, e4 */ (-other.g1_ + self_.g1_), /* e41, e42, e43 */ (-other.g2_ + self_.g2_), /* e23, e31, e12 */ (-other.g3_ + self_.g3_), /* e423, e431, e412, e321 */ (-other.g4_ + self_.g4_));
    return subtraction;
}
fn plane_bitXor_motor(self: Plane, other: Motor) -> Plane {
    return plane_wedge_plane(self_, other);
}
fn plane_wedge_scalar(self: Plane, other: Scalar) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), (self_.g0_.w * other.g0_)));
}
fn plane_mul_flector(self: Plane, other: Flector) -> Motor {
    return plane_geometricProduct_plane(self_, other);
}
fn origin_antiWedge_motor(self: Origin, other: Motor) -> Origin {
    return Origin(/* e4 */ (other.g0_.w * self_.g0_));
}
fn plane_antiSandwich_antiScalar(self: Plane, other: AntiScalar) -> Motor {
    let geometric_anti_product: Plane = Plane(/* e423, e431, e412, e321 */ (vec4<f32>(other.g0_) * self_.g0_));
    return plane_geometricAntiProduct_plane(geometric_anti_product, plane_antiReverse(self_));
}
fn multiVector_antiWedge_dualNum(self: MultiVector, other: DualNum) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(((other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x)), (other.g0_.y * self_.g0_.y)), /* e1, e2, e3, e4 */ vec4<f32>((other.g0_.y * self_.g1_.x), (other.g0_.y * self_.g1_.y), (other.g0_.y * self_.g1_.z), (other.g0_.y * self_.g1_.w)), /* e41, e42, e43 */ vec3<f32>((other.g0_.y * self_.g2_.x), (other.g0_.y * self_.g2_.y), (other.g0_.y * self_.g2_.z)), /* e23, e31, e12 */ vec3<f32>((other.g0_.y * self_.g3_.x), (other.g0_.y * self_.g3_.y), (other.g0_.y * self_.g3_.z)), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.y * self_.g4_.x), (other.g0_.y * self_.g4_.y), (other.g0_.y * self_.g4_.z), (other.g0_.y * self_.g4_.w)));
}
fn flector_geometricAntiProduct_antiScalar(self: Flector, other: AntiScalar) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), (self_.g0_.w * other.g0_)), /* e423, e431, e412, e321 */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), (self_.g1_.w * other.g0_)));
}
fn origin_geometricAntiQuotient_multiVector(self: Origin, other: MultiVector) -> Origin {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.y, 2) - pow(other.g2_.x, 2) - pow(other.g2_.y, 2) - pow(other.g2_.z, 2) - pow(other.g1_.w, 2) + pow(other.g4_.x, 2) + pow(other.g4_.y, 2) + pow(other.g4_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Origin = Origin(/* e4 */ (anti_inverse.g0_ * self_.g0_));
    return geometric_anti_product;
}
fn motor_geometricAntiQuotient_motor(self: Motor, other: Motor) -> Motor {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2) + pow(other.g0_.w, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(anti_inverse.g0_) * self_.g1_));
    return geometric_anti_product;
}
fn plane_add_multiVector(self: Plane, other: MultiVector) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ other.g0_, /* e1, e2, e3, e4 */ other.g1_, /* e41, e42, e43 */ other.g2_, /* e23, e31, e12 */ other.g3_, /* e423, e431, e412, e321 */ (other.g4_ + self_.g0_));
    return addition;
}
fn dualNum_wedge_line(self: DualNum, other: Line) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>((self_.g0_.x * other.g0_.x), (self_.g0_.x * other.g0_.y), (self_.g0_.x * other.g0_.z)), /* e23, e31, e12 */ vec3<f32>((self_.g0_.x * other.g1_.x), (self_.g0_.x * other.g1_.y), (self_.g0_.x * other.g1_.z)));
}
fn antiScalar_antiWedge_motor(self: AntiScalar, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), (other.g0_.w * self_.g0_)), /* e23, e31, e12, scalar */ vec4<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_), (other.g1_.w * self_.g0_)));
}
fn horizon_geometricQuotient_horizon(self: Horizon, other: Horizon) -> Horizon {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Horizon = Horizon(/* e321 */ (self_.g0_ * inverse.g0_));
    return geometric_product;
}
fn origin_sub_line(self: Origin, other: Line) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_), /* e41, e42, e43 */ (other.g0_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (other.g1_ * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    return subtraction;
}
fn horizon_geometricProduct_line(self: Horizon, other: Line) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_), 0.0), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), 0.0));
}
fn origin_sub_dualNum(self: Origin, other: DualNum) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ (other.g0_ * vec2<f32>(-1.0)), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    return subtraction;
}
fn antiScalar_add_motor(self: AntiScalar, other: Motor) -> Motor {
    let addition: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, (other.g0_.w + self_.g0_)), /* e23, e31, e12, scalar */ other.g1_);
    return addition;
}
fn antiScalar_geometricProduct_horizon(self: AntiScalar, other: Horizon) -> Origin {
    return Origin(/* e4 */ (self_.g0_ * other.g0_ * -1.0));
}
fn flector_mul_horizon(self: Flector, other: Horizon) -> Motor {
    return flector_geometricProduct_flector(self_, other);
}
fn motor_wedge_origin(self: Motor, other: Origin) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g1_.w * other.g0_)), /* e423, e431, e412, e321 */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), 0.0));
}
fn origin_geometricAntiProduct_line(self: Origin, other: Line) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g1_.x * self_.g0_ * -1.0), (other.g1_.y * self_.g0_ * -1.0), (other.g1_.z * self_.g0_ * -1.0), 0.0), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), 0.0));
}
fn plane_wedge_dualNum(self: Plane, other: DualNum) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>((other.g0_.x * self_.g0_.x), (other.g0_.x * self_.g0_.y), (other.g0_.x * self_.g0_.z), (other.g0_.x * self_.g0_.w)));
}
fn flector_add_motor(self: Flector, other: Motor) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(other.g1_.w, other.g0_.w), /* e1, e2, e3, e4 */ self_.g0_, /* e41, e42, e43 */ vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z), /* e23, e31, e12 */ vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z), /* e423, e431, e412, e321 */ self_.g1_);
    return addition;
}
fn horizon_add_horizon(self: Horizon, other: Horizon) -> Horizon {
    let addition: Horizon = Horizon(/* e321 */ (other.g0_ + self_.g0_));
    return addition;
}
fn horizon_geometricAntiQuotient_motor(self: Horizon, other: Motor) -> Horizon {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2) + pow(other.g0_.w, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Horizon = Horizon(/* e321 */ (anti_inverse.g0_ * self_.g0_));
    return geometric_anti_product;
}
fn scalar_geometricProduct_motor(self: Scalar, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), (other.g0_.w * self_.g0_)), /* e23, e31, e12, scalar */ vec4<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_), (other.g1_.w * self_.g0_)));
}
fn motor_add_flector(self: Motor, other: Flector) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g1_.w, self_.g0_.w), /* e1, e2, e3, e4 */ other.g0_, /* e41, e42, e43 */ vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z), /* e23, e31, e12 */ vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z), /* e423, e431, e412, e321 */ other.g1_);
    return addition;
}
fn scalar_sub_origin(self: Scalar, other: Origin) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g0_, 0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_ * -1.0)), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    return subtraction;
}
fn line_antiWedge_dualNum(self: Line, other: DualNum) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>((other.g0_.y * self_.g0_.x), (other.g0_.y * self_.g0_.y), (other.g0_.y * self_.g0_.z)), /* e23, e31, e12 */ vec3<f32>((other.g0_.y * self_.g1_.x), (other.g0_.y * self_.g1_.y), (other.g0_.y * self_.g1_.z)));
}
fn plane_mul_plane(self: Plane, other: Plane) -> Motor {
    return plane_geometricProduct_plane(self_, other);
}
fn plane_geometricQuotient_horizon(self: Plane, other: Horizon) -> Plane {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Plane = Plane(/* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g0_));
    return geometric_product;
}
fn dualNum_mul_dualNum(self: DualNum, other: DualNum) -> DualNum {
    return dualNum_geometricProduct_dualNum(self_, other);
}
fn flector_sandwich_multiVector(self: Flector, other: MultiVector) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(0.0, ((self_.g0_.w * other.g4_.w) - (self_.g1_.x * other.g1_.x) - (self_.g1_.y * other.g1_.y) - (self_.g1_.z * other.g1_.z))) + (vec2<f32>(self_.g0_.x) * vec2<f32>(other.g1_.x, other.g4_.x)) + (vec2<f32>(self_.g0_.y) * vec2<f32>(other.g1_.y, other.g4_.y)) + (vec2<f32>(self_.g0_.z) * vec2<f32>(other.g1_.z, other.g4_.z)) - (vec2<f32>(self_.g1_.w) * vec2<f32>(other.g4_.w, other.g1_.w))), /* e1, e2, e3, e4 */ (vec4<f32>((other.g3_.y * self_.g0_.z), (other.g3_.z * self_.g0_.x), (other.g3_.x * self_.g0_.y), (-(other.g2_.y * self_.g0_.y) - (other.g2_.z * self_.g0_.z) - (other.g3_.x * self_.g1_.x) - (other.g3_.y * self_.g1_.y) - (other.g3_.z * self_.g1_.z))) + (vec4<f32>(other.g0_.x) * self_.g0_) + (vec4<f32>(self_.g1_.w) * vec4<f32>(other.g3_.x, other.g3_.y, other.g3_.z, other.g0_.y)) - (vec4<f32>(other.g3_.z, other.g3_.x, other.g3_.y, other.g2_.x) * self_.g0_.yzxx)), /* e41, e42, e43 */ (vec3<f32>(((self_.g0_.y * other.g4_.z) - (self_.g0_.z * other.g4_.y) - (self_.g1_.y * other.g1_.z) + (self_.g1_.z * other.g1_.y)), (-(self_.g0_.x * other.g4_.z) + (self_.g0_.z * other.g4_.x) + (self_.g1_.x * other.g1_.z) - (self_.g1_.z * other.g1_.x)), ((self_.g0_.x * other.g4_.y) - (self_.g0_.y * other.g4_.x) - (self_.g1_.x * other.g1_.y) + (self_.g1_.y * other.g1_.x))) + (vec3<f32>(self_.g0_.w) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z)) - (vec3<f32>(self_.g1_.w) * vec3<f32>(other.g4_.x, other.g4_.y, other.g4_.z)) - (vec3<f32>(other.g1_.w) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z)) + (vec3<f32>(other.g4_.w) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z))), /* e23, e31, e12 */ (vec3<f32>(((self_.g0_.y * other.g1_.z) - (self_.g0_.z * other.g1_.y)), (-(self_.g0_.x * other.g1_.z) + (self_.g0_.z * other.g1_.x)), ((self_.g0_.x * other.g1_.y) - (self_.g0_.y * other.g1_.x))) - (vec3<f32>(self_.g1_.w) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z)) - (vec3<f32>(other.g4_.w) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z))), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.y * self_.g0_.x) + (other.g2_.x * self_.g1_.w) + (other.g2_.y * self_.g0_.z) + (other.g3_.x * self_.g0_.w) + (other.g3_.y * self_.g1_.z) - (other.g3_.z * self_.g1_.y)), ((other.g0_.y * self_.g0_.y) + (other.g2_.y * self_.g1_.w) + (other.g2_.z * self_.g0_.x) - (other.g3_.x * self_.g1_.z) + (other.g3_.y * self_.g0_.w) + (other.g3_.z * self_.g1_.x)), ((other.g0_.y * self_.g0_.z) + (other.g2_.x * self_.g0_.y) + (other.g2_.z * self_.g1_.w) + (other.g3_.x * self_.g1_.y) - (other.g3_.y * self_.g1_.x) + (other.g3_.z * self_.g0_.w)), (-(other.g3_.y * self_.g0_.y) - (other.g3_.z * self_.g0_.z))) + (vec4<f32>(other.g0_.x) * self_.g1_) - (vec4<f32>(other.g2_.z, other.g2_.x, other.g2_.y, other.g3_.x) * self_.g0_.yzxx)));
    return multiVector_geometricProduct_multiVector(geometric_product, flector_reverse(self_));
}
fn horizon_wedge_scalar(self: Horizon, other: Scalar) -> Horizon {
    return Horizon(/* e321 */ (self_.g0_ * other.g0_));
}
fn multiVector_mul_motor(self: MultiVector, other: Motor) -> MultiVector {
    return multiVector_geometricProduct_multiVector(self_, other);
}
fn horizon_sandwich_antiScalar(self: Horizon, other: AntiScalar) -> AntiScalar {
    let geometric_product: Origin = Origin(/* e4 */ (other.g0_ * self_.g0_));
    return origin_geometricProduct_origin(geometric_product, horizon_reverse(self_));
}
fn line_antiWedge_plane(self: Line, other: Plane) -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>(((self_.g0_.x * other.g0_.w) + (self_.g1_.y * other.g0_.z) - (self_.g1_.z * other.g0_.y)), ((self_.g0_.y * other.g0_.w) - (self_.g1_.x * other.g0_.z) + (self_.g1_.z * other.g0_.x)), ((self_.g0_.z * other.g0_.w) + (self_.g1_.x * other.g0_.y) - (self_.g1_.y * other.g0_.x)), (-(self_.g0_.x * other.g0_.x) - (self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z))));
}
fn horizon_geometricAntiQuotient_antiScalar(self: Horizon, other: AntiScalar) -> Horizon {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Horizon = Horizon(/* e321 */ (anti_inverse.g0_ * self_.g0_));
    return geometric_anti_product;
}
fn flector_sub_scalar(self: Flector, other: Scalar) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>((other.g0_ * -1.0), 0.0), /* e1, e2, e3, e4 */ self_.g0_, /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ self_.g1_);
    return subtraction;
}
fn scalar_geometricAntiProduct_origin(self: Scalar, other: Origin) -> Horizon {
    return Horizon(/* e321 */ (other.g0_ * self_.g0_));
}
fn flector_wedge_flector(self: Flector, other: Flector) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(((other.g0_.x * self_.g0_.w) - (other.g0_.w * self_.g0_.x)), ((other.g0_.y * self_.g0_.w) - (other.g0_.w * self_.g0_.y)), ((other.g0_.z * self_.g0_.w) - (other.g0_.w * self_.g0_.z)), (-(other.g0_.x * self_.g1_.x) - (other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g0_.w * self_.g1_.w) + (other.g1_.x * self_.g0_.x) + (other.g1_.y * self_.g0_.y) + (other.g1_.z * self_.g0_.z) + (other.g1_.w * self_.g0_.w))), /* e23, e31, e12, scalar */ vec4<f32>((-(other.g0_.y * self_.g0_.z) + (other.g0_.z * self_.g0_.y)), ((other.g0_.x * self_.g0_.z) - (other.g0_.z * self_.g0_.x)), (-(other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x)), 0.0));
}
fn multiVector_geometricQuotient_plane(self: MultiVector, other: Plane) -> MultiVector {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.w, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(inverse.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(inverse.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(inverse.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g4_));
    return geometric_product;
}
fn line_mul_flector(self: Line, other: Flector) -> Flector {
    return line_geometricProduct_line(self_, other);
}
fn origin_geometricProduct_flector(self: Origin, other: Flector) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), (other.g1_.w * self_.g0_)), /* e23, e31, e12, scalar */ vec4<f32>(0.0));
}
fn point_geometricQuotient_point(self: Point, other: Point) -> Point {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Point = Point(/* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g0_));
    return geometric_product;
}
fn dualNum_add_dualNum(self: DualNum, other: DualNum) -> DualNum {
    let addition: DualNum = DualNum(/* scalar, e1234 */ (other.g0_ + self_.g0_));
    return addition;
}
fn plane_sandwich_antiScalar(self: Plane, other: AntiScalar) -> AntiScalar {
    let geometric_product: Origin = Origin(/* e4 */ (self_.g0_.w * other.g0_));
    return origin_geometricProduct_origin(geometric_product, plane_reverse(self_));
}
fn point_mul_multiVector(self: Point, other: MultiVector) -> MultiVector {
    return point_geometricProduct_point(self_, other);
}
fn origin_geometricAntiQuotient_flector(self: Origin, other: Flector) -> Origin {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.w, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Origin = Origin(/* e4 */ (anti_inverse.g0_ * self_.g0_));
    return geometric_anti_product;
}
fn antiScalar_geometricAntiProduct_motor(self: AntiScalar, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), (other.g0_.w * self_.g0_)), /* e23, e31, e12, scalar */ vec4<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_), (other.g1_.w * self_.g0_)));
}
fn flector_antiSandwich_multiVector(self: Flector, other: MultiVector) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(((self_.g0_.w * other.g4_.w) - (self_.g1_.x * other.g1_.x) - (self_.g1_.y * other.g1_.y) - (self_.g1_.z * other.g1_.z)), 0.0) - (vec2<f32>(other.g1_.w) * vec2<f32>(self_.g1_.w, self_.g0_.w)) + (vec2<f32>(other.g4_.x) * vec2<f32>(self_.g0_.x, self_.g1_.x)) + (vec2<f32>(other.g4_.y) * vec2<f32>(self_.g0_.y, self_.g1_.y)) + (vec2<f32>(other.g4_.z) * vec2<f32>(self_.g0_.z, self_.g1_.z))), /* e1, e2, e3, e4 */ (vec4<f32>(((other.g2_.x * self_.g1_.w) - (other.g2_.y * self_.g0_.z) + (other.g2_.z * self_.g0_.y) - (other.g3_.x * self_.g0_.w) + (other.g3_.y * self_.g1_.z)), ((other.g2_.x * self_.g0_.z) + (other.g2_.y * self_.g1_.w) - (other.g2_.z * self_.g0_.x) - (other.g3_.y * self_.g0_.w) + (other.g3_.z * self_.g1_.x)), (-(other.g2_.x * self_.g0_.y) + (other.g2_.y * self_.g0_.x) + (other.g2_.z * self_.g1_.w) + (other.g3_.x * self_.g1_.y) - (other.g3_.z * self_.g0_.w)), ((other.g2_.z * self_.g1_.z) * -1.0)) + (vec4<f32>(other.g0_.y) * self_.g0_) - (vec4<f32>(other.g0_.x, other.g0_.x, other.g0_.x, other.g2_.x) * self_.g1_.xyzx) - (vec4<f32>(other.g3_.z, other.g3_.x, other.g3_.y, other.g2_.y) * self_.g1_.yzxy)), /* e41, e42, e43 */ (vec3<f32>((-(self_.g1_.y * other.g4_.z) + (self_.g1_.z * other.g4_.y)), ((self_.g1_.x * other.g4_.z) - (self_.g1_.z * other.g4_.x)), (-(self_.g1_.x * other.g4_.y) + (self_.g1_.y * other.g4_.x))) - (vec3<f32>(self_.g0_.w) * vec3<f32>(other.g4_.x, other.g4_.y, other.g4_.z)) - (vec3<f32>(other.g1_.w) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z))), /* e23, e31, e12 */ (vec3<f32>((-(self_.g0_.y * other.g4_.z) + (self_.g0_.z * other.g4_.y) + (self_.g1_.y * other.g1_.z) - (self_.g1_.z * other.g1_.y)), ((self_.g0_.x * other.g4_.z) - (self_.g0_.z * other.g4_.x) - (self_.g1_.x * other.g1_.z) + (self_.g1_.z * other.g1_.x)), (-(self_.g0_.x * other.g4_.y) + (self_.g0_.y * other.g4_.x) + (self_.g1_.x * other.g1_.y) - (self_.g1_.y * other.g1_.x))) + (vec3<f32>(self_.g0_.w) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z)) - (vec3<f32>(self_.g1_.w) * vec3<f32>(other.g4_.x, other.g4_.y, other.g4_.z)) - (vec3<f32>(other.g1_.w) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z)) + (vec3<f32>(other.g4_.w) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z))), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g2_.x * self_.g0_.w) - (other.g2_.y * self_.g1_.z)), ((other.g2_.y * self_.g0_.w) - (other.g2_.z * self_.g1_.x)), (-(other.g2_.x * self_.g1_.y) + (other.g2_.z * self_.g0_.w)), (-(other.g0_.x * self_.g0_.w) - (other.g2_.x * self_.g0_.x) - (other.g2_.y * self_.g0_.y) - (other.g2_.z * self_.g0_.z) + (other.g3_.y * self_.g1_.y) + (other.g3_.z * self_.g1_.z))) + (vec4<f32>(other.g0_.y) * self_.g1_) + (vec4<f32>(other.g2_.z, other.g2_.x, other.g2_.y, other.g3_.x) * self_.g1_.yzxx)));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, flector_antiReverse(self_));
}
fn motor_wedge_scalar(self: Motor, other: Scalar) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), (self_.g0_.w * other.g0_)), /* e23, e31, e12, scalar */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), (self_.g1_.w * other.g0_)));
}
fn line_add_antiScalar(self: Line, other: AntiScalar) -> Motor {
    let addition: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, other.g0_), /* e23, e31, e12, scalar */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, 0.0));
    return addition;
}
fn point_wedge_multiVector(self: Point, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, ((other.g4_.x * self_.g0_.x) + (other.g4_.y * self_.g0_.y) + (other.g4_.z * self_.g0_.z) + (other.g4_.w * self_.g0_.w))), /* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g0_.x), (other.g0_.x * self_.g0_.y), (other.g0_.x * self_.g0_.z), (other.g0_.x * self_.g0_.w)), /* e41, e42, e43 */ vec3<f32>(((other.g1_.x * self_.g0_.w) - (other.g1_.w * self_.g0_.x)), ((other.g1_.y * self_.g0_.w) - (other.g1_.w * self_.g0_.y)), ((other.g1_.z * self_.g0_.w) - (other.g1_.w * self_.g0_.z))), /* e23, e31, e12 */ vec3<f32>((-(other.g1_.y * self_.g0_.z) + (other.g1_.z * self_.g0_.y)), ((other.g1_.x * self_.g0_.z) - (other.g1_.z * self_.g0_.x)), (-(other.g1_.x * self_.g0_.y) + (other.g1_.y * self_.g0_.x))), /* e423, e431, e412, e321 */ vec4<f32>(((other.g2_.y * self_.g0_.z) - (other.g2_.z * self_.g0_.y) + (other.g3_.x * self_.g0_.w)), (-(other.g2_.x * self_.g0_.z) + (other.g2_.z * self_.g0_.x) + (other.g3_.y * self_.g0_.w)), ((other.g2_.x * self_.g0_.y) - (other.g2_.y * self_.g0_.x) + (other.g3_.z * self_.g0_.w)), (-(other.g3_.x * self_.g0_.x) - (other.g3_.y * self_.g0_.y) - (other.g3_.z * self_.g0_.z))));
}
fn plane_add_point(self: Plane, other: Point) -> Flector {
    let addition: Flector = Flector(/* e1, e2, e3, e4 */ other.g0_, /* e423, e431, e412, e321 */ self_.g0_);
    return addition;
}
fn multiVector_add_horizon(self: MultiVector, other: Horizon) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ self_.g1_, /* e41, e42, e43 */ self_.g2_, /* e23, e31, e12 */ self_.g3_, /* e423, e431, e412, e321 */ vec4<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z, (self_.g4_.w + other.g0_)));
    return addition;
}
fn scalar_sandwich_flector(self: Scalar, other: Flector) -> Flector {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_) * other.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_) * other.g1_));
    return flector_geometricProduct_flector(geometric_product, scalar_reverse(self_));
}
fn origin_bitXor_scalar(self: Origin, other: Scalar) -> Origin {
    return origin_wedge_origin(self_, other);
}
fn horizon_sandwich_scalar(self: Horizon, other: Scalar) -> Scalar {
    let geometric_product: Horizon = Horizon(/* e321 */ (self_.g0_ * other.g0_));
    return horizon_geometricProduct_horizon(geometric_product, horizon_reverse(self_));
}
fn antiScalar_mul_plane(self: AntiScalar, other: Plane) -> Origin {
    return antiScalar_geometricProduct_antiScalar(self_, other);
}
fn flector_wedge_plane(self: Flector, other: Plane) -> AntiScalar {
    return AntiScalar(/* e1234 */ ((self_.g0_.x * other.g0_.x) + (self_.g0_.y * other.g0_.y) + (self_.g0_.z * other.g0_.z) + (self_.g0_.w * other.g0_.w)));
}
fn multiVector_add_flector(self: MultiVector, other: Flector) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ (other.g0_ + self_.g1_), /* e41, e42, e43 */ self_.g2_, /* e23, e31, e12 */ self_.g3_, /* e423, e431, e412, e321 */ (other.g1_ + self_.g4_));
    return addition;
}
fn point_wedge_flector(self: Point, other: Flector) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(((other.g0_.x * self_.g0_.w) - (other.g0_.w * self_.g0_.x)), ((other.g0_.y * self_.g0_.w) - (other.g0_.w * self_.g0_.y)), ((other.g0_.z * self_.g0_.w) - (other.g0_.w * self_.g0_.z)), ((other.g1_.x * self_.g0_.x) + (other.g1_.y * self_.g0_.y) + (other.g1_.z * self_.g0_.z) + (other.g1_.w * self_.g0_.w))), /* e23, e31, e12, scalar */ vec4<f32>((-(other.g0_.y * self_.g0_.z) + (other.g0_.z * self_.g0_.y)), ((other.g0_.x * self_.g0_.z) - (other.g0_.z * self_.g0_.x)), (-(other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x)), 0.0));
}
fn motor_add_line(self: Motor, other: Line) -> Motor {
    let addition: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g0_.x + self_.g0_.x), (other.g0_.y + self_.g0_.y), (other.g0_.z + self_.g0_.z), self_.g0_.w), /* e23, e31, e12, scalar */ vec4<f32>((other.g1_.x + self_.g1_.x), (other.g1_.y + self_.g1_.y), (other.g1_.z + self_.g1_.z), self_.g1_.w));
    return addition;
}
fn scalar_bitXor_horizon(self: Scalar, other: Horizon) -> Horizon {
    return scalar_wedge_scalar(self_, other);
}
fn line_geometricQuotient_line(self: Line, other: Line) -> Line {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Line = Line(/* e41, e42, e43 */ (vec3<f32>(inverse.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(inverse.g0_) * self_.g1_));
    return geometric_product;
}
fn multiVector_geometricAntiProduct_dualNum(self: MultiVector, other: DualNum) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(((other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x)), (other.g0_.y * self_.g0_.y)), /* e1, e2, e3, e4 */ vec4<f32>((-(other.g0_.x * self_.g4_.x) + (other.g0_.y * self_.g1_.x)), (-(other.g0_.x * self_.g4_.y) + (other.g0_.y * self_.g1_.y)), (-(other.g0_.x * self_.g4_.z) + (other.g0_.y * self_.g1_.z)), (other.g0_.y * self_.g1_.w)), /* e41, e42, e43 */ vec3<f32>((other.g0_.y * self_.g2_.x), (other.g0_.y * self_.g2_.y), (other.g0_.y * self_.g2_.z)), /* e23, e31, e12 */ vec3<f32>(((other.g0_.x * self_.g2_.x) + (other.g0_.y * self_.g3_.x)), ((other.g0_.x * self_.g2_.y) + (other.g0_.y * self_.g3_.y)), ((other.g0_.x * self_.g2_.z) + (other.g0_.y * self_.g3_.z))), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.y * self_.g4_.x), (other.g0_.y * self_.g4_.y), (other.g0_.y * self_.g4_.z), (-(other.g0_.x * self_.g1_.w) + (other.g0_.y * self_.g4_.w))));
}
fn flector_geometricAntiProduct_scalar(self: Flector, other: Scalar) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g1_.x * other.g0_ * -1.0), (self_.g1_.y * other.g0_ * -1.0), (self_.g1_.z * other.g0_ * -1.0), 0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.w * other.g0_ * -1.0)));
}
fn plane_antiWedge_line(self: Plane, other: Line) -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>(((other.g0_.x * self_.g0_.w) + (other.g1_.y * self_.g0_.z) - (other.g1_.z * self_.g0_.y)), ((other.g0_.y * self_.g0_.w) - (other.g1_.x * self_.g0_.z) + (other.g1_.z * self_.g0_.x)), ((other.g0_.z * self_.g0_.w) + (other.g1_.x * self_.g0_.y) - (other.g1_.y * self_.g0_.x)), (-(other.g0_.x * self_.g0_.x) - (other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z))));
}
fn antiScalar_antiWedge_antiScalar(self: AntiScalar, other: AntiScalar) -> AntiScalar {
    return AntiScalar(/* e1234 */ (other.g0_ * self_.g0_));
}
fn multiVector_sub_line(self: MultiVector, other: Line) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ self_.g1_, /* e41, e42, e43 */ (-other.g0_ + self_.g2_), /* e23, e31, e12 */ (-other.g1_ + self_.g3_), /* e423, e431, e412, e321 */ self_.g4_);
    return subtraction;
}
fn multiVector_mul_point(self: MultiVector, other: Point) -> MultiVector {
    return multiVector_geometricProduct_multiVector(self_, other);
}
fn multiVector_sandwich_horizon(self: MultiVector, other: Horizon) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(other.g0_) * vec2<f32>(self_.g4_.w, self_.g1_.w) * vec2<f32>(-1.0, 1.0)), /* e1, e2, e3, e4 */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g3_.x, self_.g3_.y, self_.g3_.z, self_.g0_.y) * vec4<f32>(1.0, 1.0, 1.0, -1.0)), /* e41, e42, e43 */ (vec3<f32>(other.g0_) * vec3<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z)), /* e23, e31, e12 */ (vec3<f32>(other.g0_) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z) * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g2_.x, self_.g2_.y, self_.g2_.z, self_.g0_.x) * vec4<f32>(-1.0, -1.0, -1.0, 1.0)));
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self_));
}
fn plane_sandwich_motor(self: Plane, other: Motor) -> Motor {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g1_.x * self_.g0_.w), (other.g1_.y * self_.g0_.w), (other.g1_.z * self_.g0_.w), ((other.g0_.w * self_.g0_.w) - (other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))), /* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.x * self_.g0_.w) + (other.g1_.y * self_.g0_.z) - (other.g1_.z * self_.g0_.y) + (other.g1_.w * self_.g0_.x)), ((other.g0_.y * self_.g0_.w) - (other.g1_.x * self_.g0_.z) + (other.g1_.z * self_.g0_.x) + (other.g1_.w * self_.g0_.y)), ((other.g0_.z * self_.g0_.w) + (other.g1_.x * self_.g0_.y) - (other.g1_.y * self_.g0_.x) + (other.g1_.w * self_.g0_.z)), (other.g1_.w * self_.g0_.w)));
    return flector_geometricProduct_flector(geometric_product, plane_reverse(self_));
}
fn dualNum_antiSandwich_multiVector(self: DualNum, other: MultiVector) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(((self_.g0_.x * other.g0_.y) + (self_.g0_.y * other.g0_.x)), (self_.g0_.y * other.g0_.y)), /* e1, e2, e3, e4 */ vec4<f32>(((self_.g0_.x * other.g4_.x) + (self_.g0_.y * other.g1_.x)), ((self_.g0_.x * other.g4_.y) + (self_.g0_.y * other.g1_.y)), ((self_.g0_.x * other.g4_.z) + (self_.g0_.y * other.g1_.z)), (self_.g0_.y * other.g1_.w)), /* e41, e42, e43 */ (vec3<f32>(self_.g0_.y) * other.g2_), /* e23, e31, e12 */ ((vec3<f32>(self_.g0_.x) * other.g2_) + (vec3<f32>(self_.g0_.y) * other.g3_)), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.y * other.g4_.x), (self_.g0_.y * other.g4_.y), (self_.g0_.y * other.g4_.z), ((self_.g0_.x * other.g1_.w) + (self_.g0_.y * other.g4_.w))));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, dualNum_antiReverse(self_));
}
fn motor_antiWedge_scalar(self: Motor, other: Scalar) -> Scalar {
    return Scalar(/* scalar */ (self_.g0_.w * other.g0_));
}
fn plane_geometricAntiQuotient_flector(self: Plane, other: Flector) -> Plane {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.w, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Plane = Plane(/* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_));
    return geometric_anti_product;
}
fn motor_geometricAntiProduct_flector(self: Motor, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(((other.g0_.x * self_.g0_.w) - (other.g0_.y * self_.g0_.z) + (other.g0_.z * self_.g0_.y) + (other.g0_.w * self_.g1_.x) + (other.g1_.x * self_.g1_.w) - (other.g1_.y * self_.g1_.z) + (other.g1_.z * self_.g1_.y) + (other.g1_.w * self_.g0_.x)), ((other.g0_.x * self_.g0_.z) + (other.g0_.y * self_.g0_.w) - (other.g0_.z * self_.g0_.x) + (other.g0_.w * self_.g1_.y) + (other.g1_.x * self_.g1_.z) + (other.g1_.y * self_.g1_.w) - (other.g1_.z * self_.g1_.x) + (other.g1_.w * self_.g0_.y)), (-(other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x) + (other.g0_.z * self_.g0_.w) + (other.g0_.w * self_.g1_.z) - (other.g1_.x * self_.g1_.y) + (other.g1_.y * self_.g1_.x) + (other.g1_.z * self_.g1_.w) + (other.g1_.w * self_.g0_.z)), ((other.g0_.w * self_.g0_.w) - (other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))), /* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.w * self_.g0_.x) + (other.g1_.x * self_.g0_.w) - (other.g1_.y * self_.g0_.z) + (other.g1_.z * self_.g0_.y)), ((other.g0_.w * self_.g0_.y) + (other.g1_.x * self_.g0_.z) + (other.g1_.y * self_.g0_.w) - (other.g1_.z * self_.g0_.x)), ((other.g0_.w * self_.g0_.z) - (other.g1_.x * self_.g0_.y) + (other.g1_.y * self_.g0_.x) + (other.g1_.z * self_.g0_.w)), (-(other.g0_.x * self_.g0_.x) - (other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z) + (other.g0_.w * self_.g1_.w) - (other.g1_.x * self_.g1_.x) - (other.g1_.y * self_.g1_.y) - (other.g1_.z * self_.g1_.z) + (other.g1_.w * self_.g0_.w))));
}
fn horizon_bitXor_scalar(self: Horizon, other: Scalar) -> Horizon {
    return horizon_wedge_horizon(self_, other);
}
fn line_mul_point(self: Line, other: Point) -> Flector {
    return line_geometricProduct_line(self_, other);
}
fn line_geometricProduct_line(self: Line, other: Line) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(((other.g0_.y * self_.g1_.z) - (other.g0_.z * self_.g1_.y) + (other.g1_.y * self_.g0_.z) - (other.g1_.z * self_.g0_.y)), (-(other.g0_.x * self_.g1_.z) + (other.g0_.z * self_.g1_.x) - (other.g1_.x * self_.g0_.z) + (other.g1_.z * self_.g0_.x)), ((other.g0_.x * self_.g1_.y) - (other.g0_.y * self_.g1_.x) + (other.g1_.x * self_.g0_.y) - (other.g1_.y * self_.g0_.x)), (-(other.g0_.x * self_.g1_.x) - (other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))), /* e23, e31, e12, scalar */ vec4<f32>(((other.g1_.y * self_.g1_.z) - (other.g1_.z * self_.g1_.y)), (-(other.g1_.x * self_.g1_.z) + (other.g1_.z * self_.g1_.x)), ((other.g1_.x * self_.g1_.y) - (other.g1_.y * self_.g1_.x)), (-(other.g1_.x * self_.g1_.x) - (other.g1_.y * self_.g1_.y) - (other.g1_.z * self_.g1_.z))));
}
fn line_geometricProduct_horizon(self: Line, other: Horizon) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), 0.0), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.x * other.g0_ * -1.0), (self_.g0_.y * other.g0_ * -1.0), (self_.g0_.z * other.g0_ * -1.0), 0.0));
}
fn point_geometricProduct_dualNum(self: Point, other: DualNum) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g0_.x), (other.g0_.x * self_.g0_.y), (other.g0_.x * self_.g0_.z), (other.g0_.x * self_.g0_.w)), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.y * self_.g0_.x), (other.g0_.y * self_.g0_.y), (other.g0_.y * self_.g0_.z), 0.0));
}
fn plane_antiSandwich_motor(self: Plane, other: Motor) -> Motor {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(((other.g0_.x * self_.g0_.w) + (other.g1_.y * self_.g0_.z)), ((other.g0_.y * self_.g0_.w) + (other.g1_.z * self_.g0_.x)), ((other.g0_.z * self_.g0_.w) + (other.g1_.x * self_.g0_.y)), ((other.g0_.z * self_.g0_.z) * -1.0)) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.x) * self_.g0_.yzxx) - (vec4<f32>(other.g1_.w, other.g1_.w, other.g1_.w, other.g0_.y) * self_.g0_.xyzy)), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.y * self_.g0_.z) * -1.0), ((other.g0_.z * self_.g0_.x) * -1.0), ((other.g0_.x * self_.g0_.y) * -1.0), ((other.g1_.y * self_.g0_.y) + (other.g1_.z * self_.g0_.z))) + (vec4<f32>(other.g0_.w, other.g0_.w, other.g0_.w, other.g1_.x) * self_.g0_.xyzx) + (other.g0_.zxyw * self_.g0_.yzxw)));
    return flector_geometricAntiProduct_flector(geometric_anti_product, plane_antiReverse(self_));
}
fn line_geometricAntiProduct_dualNum(self: Line, other: DualNum) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>((other.g0_.y * self_.g0_.x), (other.g0_.y * self_.g0_.y), (other.g0_.y * self_.g0_.z)), /* e23, e31, e12 */ vec3<f32>(((other.g0_.x * self_.g0_.x) + (other.g0_.y * self_.g1_.x)), ((other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g1_.y)), ((other.g0_.x * self_.g0_.z) + (other.g0_.y * self_.g1_.z))));
}
fn motor_bitXor_plane(self: Motor, other: Plane) -> Plane {
    return motor_wedge_motor(self_, other);
}
fn dualNum_geometricAntiProduct_motor(self: DualNum, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g0_.y * other.g0_.x), (self_.g0_.y * other.g0_.y), (self_.g0_.y * other.g0_.z), (self_.g0_.y * other.g0_.w)), /* e23, e31, e12, scalar */ vec4<f32>(((self_.g0_.x * other.g0_.x) + (self_.g0_.y * other.g1_.x)), ((self_.g0_.x * other.g0_.y) + (self_.g0_.y * other.g1_.y)), ((self_.g0_.x * other.g0_.z) + (self_.g0_.y * other.g1_.z)), ((self_.g0_.x * other.g0_.w) + (self_.g0_.y * other.g1_.w))));
}
fn flector_geometricProduct_plane(self: Flector, other: Plane) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(((self_.g0_.y * other.g0_.z) - (self_.g0_.z * other.g0_.y) + (self_.g1_.x * other.g0_.w) - (self_.g1_.w * other.g0_.x)), (-(self_.g0_.x * other.g0_.z) + (self_.g0_.z * other.g0_.x) + (self_.g1_.y * other.g0_.w) - (self_.g1_.w * other.g0_.y)), ((self_.g0_.x * other.g0_.y) - (self_.g0_.y * other.g0_.x) + (self_.g1_.z * other.g0_.w) - (self_.g1_.w * other.g0_.z)), ((self_.g0_.x * other.g0_.x) + (self_.g0_.y * other.g0_.y) + (self_.g0_.z * other.g0_.z) + (self_.g0_.w * other.g0_.w))), /* e23, e31, e12, scalar */ vec4<f32>((self_.g0_.x * other.g0_.w * -1.0), (self_.g0_.y * other.g0_.w * -1.0), (self_.g0_.z * other.g0_.w * -1.0), (self_.g1_.w * other.g0_.w * -1.0)));
}
fn flector_geometricProduct_antiScalar(self: Flector, other: AntiScalar) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g1_.w * other.g0_)), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), 0.0));
}
fn plane_geometricProduct_line(self: Plane, other: Line) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g1_.x * self_.g0_.w), (other.g1_.y * self_.g0_.w), (other.g1_.z * self_.g0_.w), (-(other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))), /* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.x * self_.g0_.w) + (other.g1_.y * self_.g0_.z) - (other.g1_.z * self_.g0_.y)), ((other.g0_.y * self_.g0_.w) - (other.g1_.x * self_.g0_.z) + (other.g1_.z * self_.g0_.x)), ((other.g0_.z * self_.g0_.w) + (other.g1_.x * self_.g0_.y) - (other.g1_.y * self_.g0_.x)), 0.0));
}
fn flector_sandwich_scalar(self: Flector, other: Scalar) -> Motor {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(other.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_) * self_.g1_));
    return flector_geometricProduct_flector(geometric_product, flector_reverse(self_));
}
fn dualNum_antiWedge_antiScalar(self: DualNum, other: AntiScalar) -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_)));
}
fn antiScalar_antiSandwich_point(self: AntiScalar, other: Point) -> Point {
    let geometric_anti_product: Point = Point(/* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_) * other.g0_));
    return point_geometricAntiProduct_point(geometric_anti_product, antiScalar_antiReverse(self_));
}
fn plane_wedge_point(self: Plane, other: Point) -> AntiScalar {
    return AntiScalar(/* e1234 */ (-(self_.g0_.x * other.g0_.x) - (self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z) - (self_.g0_.w * other.g0_.w)));
}
fn motor_geometricAntiQuotient_dualNum(self: Motor, other: DualNum) -> Motor {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_.y, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(anti_inverse.g0_) * self_.g1_));
    return geometric_anti_product;
}
fn scalar_add_flector(self: Scalar, other: Flector) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g0_, 0.0), /* e1, e2, e3, e4 */ other.g0_, /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ other.g1_);
    return addition;
}
fn multiVector_sub_horizon(self: MultiVector, other: Horizon) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ self_.g1_, /* e41, e42, e43 */ self_.g2_, /* e23, e31, e12 */ self_.g3_, /* e423, e431, e412, e321 */ vec4<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z, (self_.g4_.w - other.g0_)));
    return subtraction;
}
fn dualNum_bitXor_motor(self: DualNum, other: Motor) -> Motor {
    return dualNum_wedge_dualNum(self_, other);
}
fn antiScalar_geometricProduct_motor(self: AntiScalar, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_), (other.g1_.w * self_.g0_)), /* e23, e31, e12, scalar */ vec4<f32>(0.0));
}
fn multiVector_bitXor_point(self: MultiVector, other: Point) -> MultiVector {
    return multiVector_wedge_multiVector(self_, other);
}
fn line_geometricQuotient_motor(self: Line, other: Motor) -> Line {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2) + pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Line = Line(/* e41, e42, e43 */ (vec3<f32>(inverse.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(inverse.g0_) * self_.g1_));
    return geometric_product;
}
fn multiVector_geometricAntiProduct_multiVector(self: MultiVector, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(((other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x) - (other.g2_.x * self_.g3_.x) - (other.g2_.y * self_.g3_.y) - (other.g2_.z * self_.g3_.z) - (other.g3_.x * self_.g2_.x) - (other.g3_.y * self_.g2_.y) - (other.g3_.z * self_.g2_.z) - (other.g1_.x * self_.g4_.x) - (other.g1_.y * self_.g4_.y) - (other.g1_.z * self_.g4_.z) - (other.g1_.w * self_.g4_.w) + (other.g4_.x * self_.g1_.x) + (other.g4_.y * self_.g1_.y) + (other.g4_.z * self_.g1_.z) + (other.g4_.w * self_.g1_.w)), ((other.g0_.y * self_.g0_.y) - (other.g2_.x * self_.g2_.x) - (other.g2_.y * self_.g2_.y) - (other.g2_.z * self_.g2_.z) - (other.g1_.w * self_.g1_.w) + (other.g4_.x * self_.g4_.x) + (other.g4_.y * self_.g4_.y) + (other.g4_.z * self_.g4_.z))), /* e1, e2, e3, e4 */ vec4<f32>((-(other.g0_.x * self_.g4_.x) + (other.g0_.y * self_.g1_.x) + (self_.g0_.x * other.g4_.x) + (self_.g0_.y * other.g1_.x) + (other.g2_.x * self_.g4_.w) - (other.g2_.y * self_.g1_.z) + (other.g2_.z * self_.g1_.y) - (other.g3_.x * self_.g1_.w) + (other.g3_.y * self_.g4_.z) - (other.g3_.z * self_.g4_.y) + (self_.g2_.x * other.g4_.w) + (self_.g2_.y * other.g1_.z) - (self_.g2_.z * other.g1_.y) + (self_.g3_.x * other.g1_.w) + (self_.g3_.y * other.g4_.z) - (self_.g3_.z * other.g4_.y)), (-(other.g0_.x * self_.g4_.y) + (other.g0_.y * self_.g1_.y) + (self_.g0_.x * other.g4_.y) + (self_.g0_.y * other.g1_.y) + (other.g2_.x * self_.g1_.z) + (other.g2_.y * self_.g4_.w) - (other.g2_.z * self_.g1_.x) - (other.g3_.x * self_.g4_.z) - (other.g3_.y * self_.g1_.w) + (other.g3_.z * self_.g4_.x) - (self_.g2_.x * other.g1_.z) + (self_.g2_.y * other.g4_.w) + (self_.g2_.z * other.g1_.x) - (self_.g3_.x * other.g4_.z) + (self_.g3_.y * other.g1_.w) + (self_.g3_.z * other.g4_.x)), (-(other.g0_.x * self_.g4_.z) + (other.g0_.y * self_.g1_.z) + (self_.g0_.x * other.g4_.z) + (self_.g0_.y * other.g1_.z) - (other.g2_.x * self_.g1_.y) + (other.g2_.y * self_.g1_.x) + (other.g2_.z * self_.g4_.w) + (other.g3_.x * self_.g4_.y) - (other.g3_.y * self_.g4_.x) - (other.g3_.z * self_.g1_.w) + (self_.g2_.x * other.g1_.y) - (self_.g2_.y * other.g1_.x) + (self_.g2_.z * other.g4_.w) + (self_.g3_.x * other.g4_.y) - (self_.g3_.y * other.g4_.x) + (self_.g3_.z * other.g1_.w)), ((other.g0_.y * self_.g1_.w) + (self_.g0_.y * other.g1_.w) - (other.g2_.x * self_.g4_.x) - (other.g2_.y * self_.g4_.y) - (other.g2_.z * self_.g4_.z) - (self_.g2_.x * other.g4_.x) - (self_.g2_.y * other.g4_.y) - (self_.g2_.z * other.g4_.z))), /* e41, e42, e43 */ vec3<f32>(((other.g0_.y * self_.g2_.x) + (self_.g0_.y * other.g2_.x) - (other.g2_.y * self_.g2_.z) + (other.g2_.z * self_.g2_.y) - (other.g1_.w * self_.g4_.x) - (other.g4_.x * self_.g1_.w) + (other.g4_.y * self_.g4_.z) - (other.g4_.z * self_.g4_.y)), ((other.g0_.y * self_.g2_.y) + (self_.g0_.y * other.g2_.y) + (other.g2_.x * self_.g2_.z) - (other.g2_.z * self_.g2_.x) - (other.g1_.w * self_.g4_.y) - (other.g4_.x * self_.g4_.z) - (other.g4_.y * self_.g1_.w) + (other.g4_.z * self_.g4_.x)), ((other.g0_.y * self_.g2_.z) + (self_.g0_.y * other.g2_.z) - (other.g2_.x * self_.g2_.y) + (other.g2_.y * self_.g2_.x) - (other.g1_.w * self_.g4_.z) + (other.g4_.x * self_.g4_.y) - (other.g4_.y * self_.g4_.x) - (other.g4_.z * self_.g1_.w))), /* e23, e31, e12 */ vec3<f32>(((other.g0_.x * self_.g2_.x) + (other.g0_.y * self_.g3_.x) + (self_.g0_.x * other.g2_.x) + (self_.g0_.y * other.g3_.x) - (other.g2_.y * self_.g3_.z) + (other.g2_.z * self_.g3_.y) - (other.g3_.y * self_.g2_.z) + (other.g3_.z * self_.g2_.y) + (other.g1_.x * self_.g1_.w) - (other.g1_.y * self_.g4_.z) + (other.g1_.z * self_.g4_.y) - (other.g1_.w * self_.g1_.x) - (other.g4_.x * self_.g4_.w) + (other.g4_.y * self_.g1_.z) - (other.g4_.z * self_.g1_.y) + (other.g4_.w * self_.g4_.x)), ((other.g0_.x * self_.g2_.y) + (other.g0_.y * self_.g3_.y) + (self_.g0_.x * other.g2_.y) + (self_.g0_.y * other.g3_.y) + (other.g2_.x * self_.g3_.z) - (other.g2_.z * self_.g3_.x) + (other.g3_.x * self_.g2_.z) - (other.g3_.z * self_.g2_.x) + (other.g1_.x * self_.g4_.z) + (other.g1_.y * self_.g1_.w) - (other.g1_.z * self_.g4_.x) - (other.g1_.w * self_.g1_.y) - (other.g4_.x * self_.g1_.z) - (other.g4_.y * self_.g4_.w) + (other.g4_.z * self_.g1_.x) + (other.g4_.w * self_.g4_.y)), ((other.g0_.x * self_.g2_.z) + (other.g0_.y * self_.g3_.z) + (self_.g0_.x * other.g2_.z) + (self_.g0_.y * other.g3_.z) - (other.g2_.x * self_.g3_.y) + (other.g2_.y * self_.g3_.x) - (other.g3_.x * self_.g2_.y) + (other.g3_.y * self_.g2_.x) - (other.g1_.x * self_.g4_.y) + (other.g1_.y * self_.g4_.x) + (other.g1_.z * self_.g1_.w) - (other.g1_.w * self_.g1_.z) + (other.g4_.x * self_.g1_.y) - (other.g4_.y * self_.g1_.x) - (other.g4_.z * self_.g4_.w) + (other.g4_.w * self_.g4_.z))), /* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.y * self_.g4_.x) + (self_.g0_.y * other.g4_.x) + (other.g2_.x * self_.g1_.w) - (other.g2_.y * self_.g4_.z) + (other.g2_.z * self_.g4_.y) + (self_.g2_.x * other.g1_.w) + (self_.g2_.y * other.g4_.z) - (self_.g2_.z * other.g4_.y)), ((other.g0_.y * self_.g4_.y) + (self_.g0_.y * other.g4_.y) + (other.g2_.x * self_.g4_.z) + (other.g2_.y * self_.g1_.w) - (other.g2_.z * self_.g4_.x) - (self_.g2_.x * other.g4_.z) + (self_.g2_.y * other.g1_.w) + (self_.g2_.z * other.g4_.x)), ((other.g0_.y * self_.g4_.z) + (self_.g0_.y * other.g4_.z) - (other.g2_.x * self_.g4_.y) + (other.g2_.y * self_.g4_.x) + (other.g2_.z * self_.g1_.w) + (self_.g2_.x * other.g4_.y) - (self_.g2_.y * other.g4_.x) + (self_.g2_.z * other.g1_.w)), (-(other.g0_.x * self_.g1_.w) + (other.g0_.y * self_.g4_.w) + (self_.g0_.x * other.g1_.w) + (self_.g0_.y * other.g4_.w) - (other.g2_.x * self_.g1_.x) - (other.g2_.y * self_.g1_.y) - (other.g2_.z * self_.g1_.z) + (other.g3_.x * self_.g4_.x) + (other.g3_.y * self_.g4_.y) + (other.g3_.z * self_.g4_.z) - (self_.g2_.x * other.g1_.x) - (self_.g2_.y * other.g1_.y) - (self_.g2_.z * other.g1_.z) - (self_.g3_.x * other.g4_.x) - (self_.g3_.y * other.g4_.y) - (self_.g3_.z * other.g4_.z))));
}
fn origin_add_line(self: Origin, other: Line) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_), /* e41, e42, e43 */ other.g0_, /* e23, e31, e12 */ other.g1_, /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    return addition;
}
fn plane_mul_motor(self: Plane, other: Motor) -> Flector {
    return plane_geometricProduct_plane(self_, other);
}
fn line_bitXor_line(self: Line, other: Line) -> AntiScalar {
    return line_wedge_line(self_, other);
}
fn flector_geometricQuotient_line(self: Flector, other: Line) -> Flector {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g1_));
    return geometric_product;
}
fn horizon_geometricAntiQuotient_multiVector(self: Horizon, other: MultiVector) -> Horizon {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.y, 2) - pow(other.g2_.x, 2) - pow(other.g2_.y, 2) - pow(other.g2_.z, 2) - pow(other.g1_.w, 2) + pow(other.g4_.x, 2) + pow(other.g4_.y, 2) + pow(other.g4_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Horizon = Horizon(/* e321 */ (anti_inverse.g0_ * self_.g0_));
    return geometric_anti_product;
}
fn multiVector_geometricAntiProduct_motor(self: MultiVector, other: Motor) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(((self_.g0_.x * other.g0_.w) + (self_.g0_.y * other.g1_.w) - (self_.g2_.x * other.g1_.x) - (self_.g2_.y * other.g1_.y) - (self_.g2_.z * other.g1_.z) - (self_.g3_.x * other.g0_.x) - (self_.g3_.y * other.g0_.y) - (self_.g3_.z * other.g0_.z)), ((self_.g0_.y * other.g0_.w) - (self_.g2_.x * other.g0_.x) - (self_.g2_.y * other.g0_.y) - (self_.g2_.z * other.g0_.z))), /* e1, e2, e3, e4 */ vec4<f32>(((other.g0_.x * self_.g4_.w) - (other.g0_.y * self_.g1_.z) + (other.g0_.z * self_.g1_.y) + (other.g0_.w * self_.g1_.x) - (other.g1_.x * self_.g1_.w) + (other.g1_.y * self_.g4_.z) - (other.g1_.z * self_.g4_.y) - (other.g1_.w * self_.g4_.x)), ((other.g0_.x * self_.g1_.z) + (other.g0_.y * self_.g4_.w) - (other.g0_.z * self_.g1_.x) + (other.g0_.w * self_.g1_.y) - (other.g1_.x * self_.g4_.z) - (other.g1_.y * self_.g1_.w) + (other.g1_.z * self_.g4_.x) - (other.g1_.w * self_.g4_.y)), (-(other.g0_.x * self_.g1_.y) + (other.g0_.y * self_.g1_.x) + (other.g0_.z * self_.g4_.w) + (other.g0_.w * self_.g1_.z) + (other.g1_.x * self_.g4_.y) - (other.g1_.y * self_.g4_.x) - (other.g1_.z * self_.g1_.w) - (other.g1_.w * self_.g4_.z)), (-(other.g0_.x * self_.g4_.x) - (other.g0_.y * self_.g4_.y) - (other.g0_.z * self_.g4_.z) + (other.g0_.w * self_.g1_.w))), /* e41, e42, e43 */ vec3<f32>(((self_.g0_.y * other.g0_.x) + (self_.g2_.x * other.g0_.w) + (self_.g2_.y * other.g0_.z) - (self_.g2_.z * other.g0_.y)), ((self_.g0_.y * other.g0_.y) - (self_.g2_.x * other.g0_.z) + (self_.g2_.y * other.g0_.w) + (self_.g2_.z * other.g0_.x)), ((self_.g0_.y * other.g0_.z) + (self_.g2_.x * other.g0_.y) - (self_.g2_.y * other.g0_.x) + (self_.g2_.z * other.g0_.w))), /* e23, e31, e12 */ vec3<f32>(((self_.g0_.x * other.g0_.x) + (self_.g0_.y * other.g1_.x) + (self_.g2_.x * other.g1_.w) + (self_.g2_.y * other.g1_.z) - (self_.g2_.z * other.g1_.y) + (self_.g3_.x * other.g0_.w) + (self_.g3_.y * other.g0_.z) - (self_.g3_.z * other.g0_.y)), ((self_.g0_.x * other.g0_.y) + (self_.g0_.y * other.g1_.y) - (self_.g2_.x * other.g1_.z) + (self_.g2_.y * other.g1_.w) + (self_.g2_.z * other.g1_.x) - (self_.g3_.x * other.g0_.z) + (self_.g3_.y * other.g0_.w) + (self_.g3_.z * other.g0_.x)), ((self_.g0_.x * other.g0_.z) + (self_.g0_.y * other.g1_.z) + (self_.g2_.x * other.g1_.y) - (self_.g2_.y * other.g1_.x) + (self_.g2_.z * other.g1_.w) + (self_.g3_.x * other.g0_.y) - (self_.g3_.y * other.g0_.x) + (self_.g3_.z * other.g0_.w))), /* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.x * self_.g1_.w) - (other.g0_.y * self_.g4_.z) + (other.g0_.z * self_.g4_.y) + (other.g0_.w * self_.g4_.x)), ((other.g0_.x * self_.g4_.z) + (other.g0_.y * self_.g1_.w) - (other.g0_.z * self_.g4_.x) + (other.g0_.w * self_.g4_.y)), (-(other.g0_.x * self_.g4_.y) + (other.g0_.y * self_.g4_.x) + (other.g0_.z * self_.g1_.w) + (other.g0_.w * self_.g4_.z)), (-(other.g0_.x * self_.g1_.x) - (other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) + (other.g0_.w * self_.g4_.w) + (other.g1_.x * self_.g4_.x) + (other.g1_.y * self_.g4_.y) + (other.g1_.z * self_.g4_.z) - (other.g1_.w * self_.g1_.w))));
}
fn motor_geometricAntiQuotient_flector(self: Motor, other: Flector) -> Motor {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.w, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(anti_inverse.g0_) * self_.g1_));
    return geometric_anti_product;
}
fn dualNum_antiSandwich_horizon(self: DualNum, other: Horizon) -> Horizon {
    let geometric_anti_product: Horizon = Horizon(/* e321 */ (self_.g0_.y * other.g0_));
    return horizon_geometricAntiProduct_horizon(geometric_anti_product, dualNum_antiReverse(self_));
}
fn point_antiWedge_antiScalar(self: Point, other: AntiScalar) -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), (self_.g0_.w * other.g0_)));
}
fn point_geometricQuotient_dualNum(self: Point, other: DualNum) -> Point {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_.x, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Point = Point(/* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g0_));
    return geometric_product;
}
fn line_geometricAntiQuotient_plane(self: Line, other: Plane) -> Line {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Line = Line(/* e41, e42, e43 */ (vec3<f32>(anti_inverse.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(anti_inverse.g0_) * self_.g1_));
    return geometric_anti_product;
}
fn point_bitXor_point(self: Point, other: Point) -> Line {
    return point_wedge_point(self_, other);
}
fn point_sub_multiVector(self: Point, other: MultiVector) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ (other.g0_ * vec2<f32>(-1.0)), /* e1, e2, e3, e4 */ (-other.g1_ + self_.g0_), /* e41, e42, e43 */ (other.g2_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (other.g3_ * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ (other.g4_ * vec4<f32>(-1.0)));
    return subtraction;
}
fn line_wedge_origin(self: Line, other: Origin) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), 0.0));
}
fn plane_geometricAntiProduct_flector(self: Plane, other: Flector) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((-(other.g0_.w * self_.g0_.x) + (other.g1_.y * self_.g0_.z) - (other.g1_.z * self_.g0_.y)), (-(other.g0_.w * self_.g0_.y) - (other.g1_.x * self_.g0_.z) + (other.g1_.z * self_.g0_.x)), (-(other.g0_.w * self_.g0_.z) + (other.g1_.x * self_.g0_.y) - (other.g1_.y * self_.g0_.x)), ((other.g1_.x * self_.g0_.x) + (other.g1_.y * self_.g0_.y) + (other.g1_.z * self_.g0_.z))), /* e23, e31, e12, scalar */ vec4<f32>((-(other.g0_.y * self_.g0_.z) + (other.g0_.z * self_.g0_.y) - (other.g1_.x * self_.g0_.w) + (other.g1_.w * self_.g0_.x)), ((other.g0_.x * self_.g0_.z) - (other.g0_.z * self_.g0_.x) - (other.g1_.y * self_.g0_.w) + (other.g1_.w * self_.g0_.y)), (-(other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x) - (other.g1_.z * self_.g0_.w) + (other.g1_.w * self_.g0_.z)), (-(other.g0_.x * self_.g0_.x) - (other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z) - (other.g0_.w * self_.g0_.w))));
}
fn motor_antiWedge_multiVector(self: Motor, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(((other.g0_.x * self_.g0_.w) + (other.g0_.y * self_.g1_.w) - (other.g2_.x * self_.g1_.x) - (other.g2_.y * self_.g1_.y) - (other.g2_.z * self_.g1_.z) - (other.g3_.x * self_.g0_.x) - (other.g3_.y * self_.g0_.y) - (other.g3_.z * self_.g0_.z)), (other.g0_.y * self_.g0_.w)), /* e1, e2, e3, e4 */ vec4<f32>(((self_.g0_.x * other.g4_.w) + (self_.g0_.w * other.g1_.x) + (self_.g1_.y * other.g4_.z) - (self_.g1_.z * other.g4_.y)), ((self_.g0_.y * other.g4_.w) + (self_.g0_.w * other.g1_.y) - (self_.g1_.x * other.g4_.z) + (self_.g1_.z * other.g4_.x)), ((self_.g0_.z * other.g4_.w) + (self_.g0_.w * other.g1_.z) + (self_.g1_.x * other.g4_.y) - (self_.g1_.y * other.g4_.x)), (-(self_.g0_.x * other.g4_.x) - (self_.g0_.y * other.g4_.y) - (self_.g0_.z * other.g4_.z) + (self_.g0_.w * other.g1_.w))), /* e41, e42, e43 */ vec3<f32>(((other.g0_.y * self_.g0_.x) + (other.g2_.x * self_.g0_.w)), ((other.g0_.y * self_.g0_.y) + (other.g2_.y * self_.g0_.w)), ((other.g0_.y * self_.g0_.z) + (other.g2_.z * self_.g0_.w))), /* e23, e31, e12 */ vec3<f32>(((other.g0_.y * self_.g1_.x) + (other.g3_.x * self_.g0_.w)), ((other.g0_.y * self_.g1_.y) + (other.g3_.y * self_.g0_.w)), ((other.g0_.y * self_.g1_.z) + (other.g3_.z * self_.g0_.w))), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.w * other.g4_.x), (self_.g0_.w * other.g4_.y), (self_.g0_.w * other.g4_.z), (self_.g0_.w * other.g4_.w)));
}
fn scalar_sub_scalar(self: Scalar, other: Scalar) -> Scalar {
    let subtraction: Scalar = Scalar(/* scalar */ (-other.g0_ + self_.g0_));
    return subtraction;
}
fn horizon_geometricProduct_multiVector(self: Horizon, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((other.g4_.w * self_.g0_ * -1.0), (other.g1_.w * self_.g0_ * -1.0)), /* e1, e2, e3, e4 */ vec4<f32>((other.g3_.x * self_.g0_), (other.g3_.y * self_.g0_), (other.g3_.z * self_.g0_), (other.g0_.y * self_.g0_)), /* e41, e42, e43 */ vec3<f32>((other.g4_.x * self_.g0_ * -1.0), (other.g4_.y * self_.g0_ * -1.0), (other.g4_.z * self_.g0_ * -1.0)), /* e23, e31, e12 */ vec3<f32>((other.g1_.x * self_.g0_ * -1.0), (other.g1_.y * self_.g0_ * -1.0), (other.g1_.z * self_.g0_ * -1.0)), /* e423, e431, e412, e321 */ vec4<f32>((other.g2_.x * self_.g0_), (other.g2_.y * self_.g0_), (other.g2_.z * self_.g0_), (other.g0_.x * self_.g0_)));
}
fn flector_bitXor_motor(self: Flector, other: Motor) -> Flector {
    return flector_wedge_flector(self_, other);
}
fn origin_geometricQuotient_scalar(self: Origin, other: Scalar) -> Origin {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Origin = Origin(/* e4 */ (self_.g0_ * inverse.g0_));
    return geometric_product;
}
fn flector_geometricProduct_motor(self: Flector, other: Motor) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(((self_.g0_.x * other.g1_.w) - (self_.g0_.y * other.g1_.z) + (self_.g0_.z * other.g1_.y) + (self_.g1_.w * other.g1_.x)), ((self_.g0_.x * other.g1_.z) + (self_.g0_.y * other.g1_.w) - (self_.g0_.z * other.g1_.x) + (self_.g1_.w * other.g1_.y)), (-(self_.g0_.x * other.g1_.y) + (self_.g0_.y * other.g1_.x) + (self_.g0_.z * other.g1_.w) + (self_.g1_.w * other.g1_.z)), (-(self_.g0_.x * other.g0_.x) - (self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z) + (self_.g0_.w * other.g1_.w) - (self_.g1_.x * other.g1_.x) - (self_.g1_.y * other.g1_.y) - (self_.g1_.z * other.g1_.z) + (self_.g1_.w * other.g0_.w))), /* e423, e431, e412, e321 */ vec4<f32>(((self_.g0_.x * other.g0_.w) - (self_.g0_.y * other.g0_.z) + (self_.g0_.z * other.g0_.y) + (self_.g0_.w * other.g1_.x) + (self_.g1_.x * other.g1_.w) - (self_.g1_.y * other.g1_.z) + (self_.g1_.z * other.g1_.y) + (self_.g1_.w * other.g0_.x)), ((self_.g0_.x * other.g0_.z) + (self_.g0_.y * other.g0_.w) - (self_.g0_.z * other.g0_.x) + (self_.g0_.w * other.g1_.y) + (self_.g1_.x * other.g1_.z) + (self_.g1_.y * other.g1_.w) - (self_.g1_.z * other.g1_.x) + (self_.g1_.w * other.g0_.y)), (-(self_.g0_.x * other.g0_.y) + (self_.g0_.y * other.g0_.x) + (self_.g0_.z * other.g0_.w) + (self_.g0_.w * other.g1_.z) - (self_.g1_.x * other.g1_.y) + (self_.g1_.y * other.g1_.x) + (self_.g1_.z * other.g1_.w) + (self_.g1_.w * other.g0_.z)), (-(self_.g0_.x * other.g1_.x) - (self_.g0_.y * other.g1_.y) - (self_.g0_.z * other.g1_.z) + (self_.g1_.w * other.g1_.w))));
}
fn point_sandwich_scalar(self: Point, other: Scalar) -> Motor {
    let geometric_product: Point = Point(/* e1, e2, e3, e4 */ (vec4<f32>(other.g0_) * self_.g0_));
    return point_geometricProduct_point(geometric_product, point_reverse(self_));
}
fn plane_antiSandwich_point(self: Plane, other: Point) -> Flector {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g0_.x * other.g0_.w * -1.0), (self_.g0_.y * other.g0_.w * -1.0), (self_.g0_.z * other.g0_.w * -1.0), 0.0), /* e23, e31, e12, scalar */ (vec4<f32>((self_.g0_.y * other.g0_.z), (self_.g0_.z * other.g0_.x), (self_.g0_.x * other.g0_.y), (-(self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z) - (self_.g0_.w * other.g0_.w))) - (self_.g0_.zxyx * other.g0_.yzxx)));
    return motor_geometricAntiProduct_motor(geometric_anti_product, plane_antiReverse(self_));
}
fn scalar_geometricQuotient_plane(self: Scalar, other: Plane) -> Scalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.w, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Scalar = Scalar(/* scalar */ (inverse.g0_ * self_.g0_));
    return geometric_product;
}
fn multiVector_geometricAntiProduct_line(self: MultiVector, other: Line) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((-(other.g0_.x * self_.g3_.x) - (other.g0_.y * self_.g3_.y) - (other.g0_.z * self_.g3_.z) - (other.g1_.x * self_.g2_.x) - (other.g1_.y * self_.g2_.y) - (other.g1_.z * self_.g2_.z)), (-(other.g0_.x * self_.g2_.x) - (other.g0_.y * self_.g2_.y) - (other.g0_.z * self_.g2_.z))), /* e1, e2, e3, e4 */ vec4<f32>(((other.g0_.x * self_.g4_.w) - (other.g0_.y * self_.g1_.z) + (other.g0_.z * self_.g1_.y) - (other.g1_.x * self_.g1_.w) + (other.g1_.y * self_.g4_.z) - (other.g1_.z * self_.g4_.y)), ((other.g0_.x * self_.g1_.z) + (other.g0_.y * self_.g4_.w) - (other.g0_.z * self_.g1_.x) - (other.g1_.x * self_.g4_.z) - (other.g1_.y * self_.g1_.w) + (other.g1_.z * self_.g4_.x)), (-(other.g0_.x * self_.g1_.y) + (other.g0_.y * self_.g1_.x) + (other.g0_.z * self_.g4_.w) + (other.g1_.x * self_.g4_.y) - (other.g1_.y * self_.g4_.x) - (other.g1_.z * self_.g1_.w)), (-(other.g0_.x * self_.g4_.x) - (other.g0_.y * self_.g4_.y) - (other.g0_.z * self_.g4_.z))), /* e41, e42, e43 */ vec3<f32>(((self_.g0_.y * other.g0_.x) - (other.g0_.y * self_.g2_.z) + (other.g0_.z * self_.g2_.y)), ((self_.g0_.y * other.g0_.y) + (other.g0_.x * self_.g2_.z) - (other.g0_.z * self_.g2_.x)), ((self_.g0_.y * other.g0_.z) - (other.g0_.x * self_.g2_.y) + (other.g0_.y * self_.g2_.x))), /* e23, e31, e12 */ vec3<f32>(((self_.g0_.x * other.g0_.x) + (self_.g0_.y * other.g1_.x) - (other.g0_.y * self_.g3_.z) + (other.g0_.z * self_.g3_.y) - (other.g1_.y * self_.g2_.z) + (other.g1_.z * self_.g2_.y)), ((self_.g0_.x * other.g0_.y) + (self_.g0_.y * other.g1_.y) + (other.g0_.x * self_.g3_.z) - (other.g0_.z * self_.g3_.x) + (other.g1_.x * self_.g2_.z) - (other.g1_.z * self_.g2_.x)), ((self_.g0_.x * other.g0_.z) + (self_.g0_.y * other.g1_.z) - (other.g0_.x * self_.g3_.y) + (other.g0_.y * self_.g3_.x) - (other.g1_.x * self_.g2_.y) + (other.g1_.y * self_.g2_.x))), /* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.x * self_.g1_.w) - (other.g0_.y * self_.g4_.z) + (other.g0_.z * self_.g4_.y)), ((other.g0_.x * self_.g4_.z) + (other.g0_.y * self_.g1_.w) - (other.g0_.z * self_.g4_.x)), (-(other.g0_.x * self_.g4_.y) + (other.g0_.y * self_.g4_.x) + (other.g0_.z * self_.g1_.w)), (-(other.g0_.x * self_.g1_.x) - (other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) + (other.g1_.x * self_.g4_.x) + (other.g1_.y * self_.g4_.y) + (other.g1_.z * self_.g4_.z))));
}
fn line_add_plane(self: Line, other: Plane) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ self_.g0_, /* e23, e31, e12 */ self_.g1_, /* e423, e431, e412, e321 */ other.g0_);
    return addition;
}
fn plane_geometricProduct_motor(self: Plane, other: Motor) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g1_.x * self_.g0_.w), (other.g1_.y * self_.g0_.w), (other.g1_.z * self_.g0_.w), ((other.g0_.w * self_.g0_.w) - (other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))), /* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.x * self_.g0_.w) + (other.g1_.y * self_.g0_.z) - (other.g1_.z * self_.g0_.y) + (other.g1_.w * self_.g0_.x)), ((other.g0_.y * self_.g0_.w) - (other.g1_.x * self_.g0_.z) + (other.g1_.z * self_.g0_.x) + (other.g1_.w * self_.g0_.y)), ((other.g0_.z * self_.g0_.w) + (other.g1_.x * self_.g0_.y) - (other.g1_.y * self_.g0_.x) + (other.g1_.w * self_.g0_.z)), (other.g1_.w * self_.g0_.w)));
}
fn plane_antiWedge_plane(self: Plane, other: Plane) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>(((other.g0_.y * self_.g0_.z) - (other.g0_.z * self_.g0_.y)), (-(other.g0_.x * self_.g0_.z) + (other.g0_.z * self_.g0_.x)), ((other.g0_.x * self_.g0_.y) - (other.g0_.y * self_.g0_.x))), /* e23, e31, e12 */ vec3<f32>((-(other.g0_.x * self_.g0_.w) + (other.g0_.w * self_.g0_.x)), (-(other.g0_.y * self_.g0_.w) + (other.g0_.w * self_.g0_.y)), (-(other.g0_.z * self_.g0_.w) + (other.g0_.w * self_.g0_.z))));
}
fn motor_add_motor(self: Motor, other: Motor) -> Motor {
    let addition: Motor = Motor(/* e41, e42, e43, e1234 */ (other.g0_ + self_.g0_), /* e23, e31, e12, scalar */ (other.g1_ + self_.g1_));
    return addition;
}
fn antiScalar_sandwich_multiVector(self: AntiScalar, other: MultiVector) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (other.g0_.x * self_.g0_)), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g4_.w * self_.g0_ * -1.0)), /* e41, e42, e43 */ (vec3<f32>(self_.g0_) * other.g3_), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>((other.g1_.x * self_.g0_ * -1.0), (other.g1_.y * self_.g0_ * -1.0), (other.g1_.z * self_.g0_ * -1.0), 0.0));
    return multiVector_geometricProduct_multiVector(geometric_product, antiScalar_reverse(self_));
}
fn multiVector_wedge_multiVector(self: MultiVector, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((other.g0_.x * self_.g0_.x), ((other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x) - (other.g2_.x * self_.g3_.x) - (other.g2_.y * self_.g3_.y) - (other.g2_.z * self_.g3_.z) - (other.g3_.x * self_.g2_.x) - (other.g3_.y * self_.g2_.y) - (other.g3_.z * self_.g2_.z) - (other.g1_.x * self_.g4_.x) - (other.g1_.y * self_.g4_.y) - (other.g1_.z * self_.g4_.z) - (other.g1_.w * self_.g4_.w) + (other.g4_.x * self_.g1_.x) + (other.g4_.y * self_.g1_.y) + (other.g4_.z * self_.g1_.z) + (other.g4_.w * self_.g1_.w))), /* e1, e2, e3, e4 */ vec4<f32>(((other.g0_.x * self_.g1_.x) + (self_.g0_.x * other.g1_.x)), ((other.g0_.x * self_.g1_.y) + (self_.g0_.x * other.g1_.y)), ((other.g0_.x * self_.g1_.z) + (self_.g0_.x * other.g1_.z)), ((other.g0_.x * self_.g1_.w) + (self_.g0_.x * other.g1_.w))), /* e41, e42, e43 */ vec3<f32>(((other.g0_.x * self_.g2_.x) + (self_.g0_.x * other.g2_.x) + (other.g1_.x * self_.g1_.w) - (other.g1_.w * self_.g1_.x)), ((other.g0_.x * self_.g2_.y) + (self_.g0_.x * other.g2_.y) + (other.g1_.y * self_.g1_.w) - (other.g1_.w * self_.g1_.y)), ((other.g0_.x * self_.g2_.z) + (self_.g0_.x * other.g2_.z) + (other.g1_.z * self_.g1_.w) - (other.g1_.w * self_.g1_.z))), /* e23, e31, e12 */ vec3<f32>(((other.g0_.x * self_.g3_.x) + (self_.g0_.x * other.g3_.x) - (other.g1_.y * self_.g1_.z) + (other.g1_.z * self_.g1_.y)), ((other.g0_.x * self_.g3_.y) + (self_.g0_.x * other.g3_.y) + (other.g1_.x * self_.g1_.z) - (other.g1_.z * self_.g1_.x)), ((other.g0_.x * self_.g3_.z) + (self_.g0_.x * other.g3_.z) - (other.g1_.x * self_.g1_.y) + (other.g1_.y * self_.g1_.x))), /* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.x * self_.g4_.x) + (self_.g0_.x * other.g4_.x) + (other.g2_.y * self_.g1_.z) - (other.g2_.z * self_.g1_.y) + (other.g3_.x * self_.g1_.w) + (self_.g2_.y * other.g1_.z) - (self_.g2_.z * other.g1_.y) + (self_.g3_.x * other.g1_.w)), ((other.g0_.x * self_.g4_.y) + (self_.g0_.x * other.g4_.y) - (other.g2_.x * self_.g1_.z) + (other.g2_.z * self_.g1_.x) + (other.g3_.y * self_.g1_.w) - (self_.g2_.x * other.g1_.z) + (self_.g2_.z * other.g1_.x) + (self_.g3_.y * other.g1_.w)), ((other.g0_.x * self_.g4_.z) + (self_.g0_.x * other.g4_.z) + (other.g2_.x * self_.g1_.y) - (other.g2_.y * self_.g1_.x) + (other.g3_.z * self_.g1_.w) + (self_.g2_.x * other.g1_.y) - (self_.g2_.y * other.g1_.x) + (self_.g3_.z * other.g1_.w)), ((other.g0_.x * self_.g4_.w) + (self_.g0_.x * other.g4_.w) - (other.g3_.x * self_.g1_.x) - (other.g3_.y * self_.g1_.y) - (other.g3_.z * self_.g1_.z) - (self_.g3_.x * other.g1_.x) - (self_.g3_.y * other.g1_.y) - (self_.g3_.z * other.g1_.z))));
}
fn line_geometricProduct_multiVector(self: Line, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((-(self_.g1_.x * other.g3_.x) - (self_.g1_.y * other.g3_.y) - (self_.g1_.z * other.g3_.z)), (-(self_.g0_.x * other.g3_.x) - (self_.g0_.y * other.g3_.y) - (self_.g0_.z * other.g3_.z) - (self_.g1_.x * other.g2_.x) - (self_.g1_.y * other.g2_.y) - (self_.g1_.z * other.g2_.z))), /* e1, e2, e3, e4 */ vec4<f32>(((self_.g1_.x * other.g4_.w) - (self_.g1_.y * other.g1_.z) + (self_.g1_.z * other.g1_.y)), ((self_.g1_.x * other.g1_.z) + (self_.g1_.y * other.g4_.w) - (self_.g1_.z * other.g1_.x)), (-(self_.g1_.x * other.g1_.y) + (self_.g1_.y * other.g1_.x) + (self_.g1_.z * other.g4_.w)), ((self_.g0_.x * other.g1_.x) + (self_.g0_.y * other.g1_.y) + (self_.g0_.z * other.g1_.z) - (self_.g1_.x * other.g4_.x) - (self_.g1_.y * other.g4_.y) - (self_.g1_.z * other.g4_.z))), /* e41, e42, e43 */ vec3<f32>(((other.g0_.x * self_.g0_.x) + (other.g0_.y * self_.g1_.x) - (self_.g0_.y * other.g3_.z) + (self_.g0_.z * other.g3_.y) - (self_.g1_.y * other.g2_.z) + (self_.g1_.z * other.g2_.y)), ((other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g1_.y) + (self_.g0_.x * other.g3_.z) - (self_.g0_.z * other.g3_.x) + (self_.g1_.x * other.g2_.z) - (self_.g1_.z * other.g2_.x)), ((other.g0_.x * self_.g0_.z) + (other.g0_.y * self_.g1_.z) - (self_.g0_.x * other.g3_.y) + (self_.g0_.y * other.g3_.x) - (self_.g1_.x * other.g2_.y) + (self_.g1_.y * other.g2_.x))), /* e23, e31, e12 */ vec3<f32>(((other.g0_.x * self_.g1_.x) - (self_.g1_.y * other.g3_.z) + (self_.g1_.z * other.g3_.y)), ((other.g0_.x * self_.g1_.y) + (self_.g1_.x * other.g3_.z) - (self_.g1_.z * other.g3_.x)), ((other.g0_.x * self_.g1_.z) - (self_.g1_.x * other.g3_.y) + (self_.g1_.y * other.g3_.x))), /* e423, e431, e412, e321 */ vec4<f32>((-(self_.g0_.x * other.g4_.w) + (self_.g0_.y * other.g1_.z) - (self_.g0_.z * other.g1_.y) + (self_.g1_.x * other.g1_.w) - (self_.g1_.y * other.g4_.z) + (self_.g1_.z * other.g4_.y)), (-(self_.g0_.x * other.g1_.z) - (self_.g0_.y * other.g4_.w) + (self_.g0_.z * other.g1_.x) + (self_.g1_.x * other.g4_.z) + (self_.g1_.y * other.g1_.w) - (self_.g1_.z * other.g4_.x)), ((self_.g0_.x * other.g1_.y) - (self_.g0_.y * other.g1_.x) - (self_.g0_.z * other.g4_.w) - (self_.g1_.x * other.g4_.y) + (self_.g1_.y * other.g4_.x) + (self_.g1_.z * other.g1_.w)), (-(self_.g1_.x * other.g1_.x) - (self_.g1_.y * other.g1_.y) - (self_.g1_.z * other.g1_.z))));
}
fn origin_wedge_line(self: Origin, other: Line) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_), 0.0));
}
fn plane_antiWedge_antiScalar(self: Plane, other: AntiScalar) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), (self_.g0_.w * other.g0_)));
}
fn point_geometricProduct_line(self: Point, other: Line) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(((other.g1_.y * self_.g0_.z) - (other.g1_.z * self_.g0_.y)), (-(other.g1_.x * self_.g0_.z) + (other.g1_.z * self_.g0_.x)), ((other.g1_.x * self_.g0_.y) - (other.g1_.y * self_.g0_.x)), (-(other.g0_.x * self_.g0_.x) - (other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z))), /* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.y * self_.g0_.z) - (other.g0_.z * self_.g0_.y) + (other.g1_.x * self_.g0_.w)), (-(other.g0_.x * self_.g0_.z) + (other.g0_.z * self_.g0_.x) + (other.g1_.y * self_.g0_.w)), ((other.g0_.x * self_.g0_.y) - (other.g0_.y * self_.g0_.x) + (other.g1_.z * self_.g0_.w)), (-(other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))));
}
fn scalar_geometricAntiProduct_dualNum(self: Scalar, other: DualNum) -> Scalar {
    return Scalar(/* scalar */ (other.g0_.y * self_.g0_));
}
fn scalar_add_origin(self: Scalar, other: Origin) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g0_, 0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other.g0_), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    return addition;
}
fn multiVector_wedge_horizon(self: MultiVector, other: Horizon) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (self_.g1_.w * other.g0_)), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.x * other.g0_)));
}
fn origin_sub_origin(self: Origin, other: Origin) -> Origin {
    let subtraction: Origin = Origin(/* e4 */ (-other.g0_ + self_.g0_));
    return subtraction;
}
fn plane_sub_point(self: Plane, other: Point) -> Flector {
    let subtraction: Flector = Flector(/* e1, e2, e3, e4 */ (other.g0_ * vec4<f32>(-1.0)), /* e423, e431, e412, e321 */ self_.g0_);
    return subtraction;
}
fn antiScalar_sandwich_line(self: AntiScalar, other: Line) -> Line {
    let geometric_product: Line = Line(/* e41, e42, e43 */ (vec3<f32>(self_.g0_) * other.g1_), /* e23, e31, e12 */ vec3<f32>(0.0));
    return line_geometricProduct_line(geometric_product, antiScalar_reverse(self_));
}
fn line_geometricAntiProduct_plane(self: Line, other: Plane) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(((self_.g0_.x * other.g0_.w) + (self_.g1_.y * other.g0_.z) - (self_.g1_.z * other.g0_.y)), ((self_.g0_.y * other.g0_.w) - (self_.g1_.x * other.g0_.z) + (self_.g1_.z * other.g0_.x)), ((self_.g0_.z * other.g0_.w) + (self_.g1_.x * other.g0_.y) - (self_.g1_.y * other.g0_.x)), (-(self_.g0_.x * other.g0_.x) - (self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z))), /* e423, e431, e412, e321 */ vec4<f32>(((self_.g0_.y * other.g0_.z) - (self_.g0_.z * other.g0_.y)), (-(self_.g0_.x * other.g0_.z) + (self_.g0_.z * other.g0_.x)), ((self_.g0_.x * other.g0_.y) - (self_.g0_.y * other.g0_.x)), (-(self_.g1_.x * other.g0_.x) - (self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))));
}
fn flector_antiWedge_multiVector(self: Flector, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(((self_.g0_.x * other.g4_.x) + (self_.g0_.y * other.g4_.y) + (self_.g0_.z * other.g4_.z) + (self_.g0_.w * other.g4_.w) - (self_.g1_.x * other.g1_.x) - (self_.g1_.y * other.g1_.y) - (self_.g1_.z * other.g1_.z) - (self_.g1_.w * other.g1_.w)), 0.0), /* e1, e2, e3, e4 */ vec4<f32>(((other.g0_.y * self_.g0_.x) + (other.g2_.x * self_.g1_.w) + (other.g3_.y * self_.g1_.z) - (other.g3_.z * self_.g1_.y)), ((other.g0_.y * self_.g0_.y) + (other.g2_.y * self_.g1_.w) - (other.g3_.x * self_.g1_.z) + (other.g3_.z * self_.g1_.x)), ((other.g0_.y * self_.g0_.z) + (other.g2_.z * self_.g1_.w) + (other.g3_.x * self_.g1_.y) - (other.g3_.y * self_.g1_.x)), ((other.g0_.y * self_.g0_.w) - (other.g2_.x * self_.g1_.x) - (other.g2_.y * self_.g1_.y) - (other.g2_.z * self_.g1_.z))), /* e41, e42, e43 */ vec3<f32>((-(self_.g1_.y * other.g4_.z) + (self_.g1_.z * other.g4_.y)), ((self_.g1_.x * other.g4_.z) - (self_.g1_.z * other.g4_.x)), (-(self_.g1_.x * other.g4_.y) + (self_.g1_.y * other.g4_.x))), /* e23, e31, e12 */ vec3<f32>(((self_.g1_.x * other.g4_.w) - (self_.g1_.w * other.g4_.x)), ((self_.g1_.y * other.g4_.w) - (self_.g1_.w * other.g4_.y)), ((self_.g1_.z * other.g4_.w) - (self_.g1_.w * other.g4_.z))), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.y * self_.g1_.x), (other.g0_.y * self_.g1_.y), (other.g0_.y * self_.g1_.z), (other.g0_.y * self_.g1_.w)));
}
fn multiVector_geometricAntiProduct_horizon(self: MultiVector, other: Horizon) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((self_.g1_.w * other.g0_), 0.0), /* e1, e2, e3, e4 */ vec4<f32>((self_.g2_.x * other.g0_), (self_.g2_.y * other.g0_), (self_.g2_.z * other.g0_), 0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>((self_.g4_.x * other.g0_), (self_.g4_.y * other.g0_), (self_.g4_.z * other.g0_)), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.y * other.g0_)));
}
fn line_add_motor(self: Line, other: Motor) -> Motor {
    let addition: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g0_.x + other.g0_.x), (self_.g0_.y + other.g0_.y), (self_.g0_.z + other.g0_.z), other.g0_.w), /* e23, e31, e12, scalar */ vec4<f32>((self_.g1_.x + other.g1_.x), (self_.g1_.y + other.g1_.y), (self_.g1_.z + other.g1_.z), other.g1_.w));
    return addition;
}
fn scalar_geometricProduct_dualNum(self: Scalar, other: DualNum) -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_)));
}
fn point_geometricQuotient_horizon(self: Point, other: Horizon) -> Point {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Point = Point(/* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g0_));
    return geometric_product;
}
fn dualNum_sub_point(self: DualNum, other: Point) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ (other.g0_ * vec4<f32>(-1.0)), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    return subtraction;
}
fn antiScalar_sub_multiVector(self: AntiScalar, other: MultiVector) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>((other.g0_.x * -1.0), (-other.g0_.y + self_.g0_)), /* e1, e2, e3, e4 */ (other.g1_ * vec4<f32>(-1.0)), /* e41, e42, e43 */ (other.g2_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (other.g3_ * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ (other.g4_ * vec4<f32>(-1.0)));
    return subtraction;
}
fn plane_mul_line(self: Plane, other: Line) -> Flector {
    return plane_geometricProduct_plane(self_, other);
}
fn flector_geometricProduct_origin(self: Flector, other: Origin) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g0_.x * other.g0_ * -1.0), (self_.g0_.y * other.g0_ * -1.0), (self_.g0_.z * other.g0_ * -1.0), (self_.g1_.w * other.g0_ * -1.0)), /* e23, e31, e12, scalar */ vec4<f32>(0.0));
}
fn flector_antiWedge_flector(self: Flector, other: Flector) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(((other.g1_.y * self_.g1_.z) - (other.g1_.z * self_.g1_.y)), (-(other.g1_.x * self_.g1_.z) + (other.g1_.z * self_.g1_.x)), ((other.g1_.x * self_.g1_.y) - (other.g1_.y * self_.g1_.x)), 0.0), /* e23, e31, e12, scalar */ vec4<f32>((-(other.g1_.x * self_.g1_.w) + (other.g1_.w * self_.g1_.x)), (-(other.g1_.y * self_.g1_.w) + (other.g1_.w * self_.g1_.y)), (-(other.g1_.z * self_.g1_.w) + (other.g1_.w * self_.g1_.z)), (-(other.g0_.x * self_.g1_.x) - (other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g0_.w * self_.g1_.w) + (other.g1_.x * self_.g0_.x) + (other.g1_.y * self_.g0_.y) + (other.g1_.z * self_.g0_.z) + (other.g1_.w * self_.g0_.w))));
}
fn horizon_geometricProduct_flector(self: Horizon, other: Flector) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g1_.x * self_.g0_ * -1.0), (other.g1_.y * self_.g0_ * -1.0), (other.g1_.z * self_.g0_ * -1.0), (other.g0_.w * self_.g0_ * -1.0)), /* e23, e31, e12, scalar */ vec4<f32>((other.g0_.x * self_.g0_ * -1.0), (other.g0_.y * self_.g0_ * -1.0), (other.g0_.z * self_.g0_ * -1.0), (other.g1_.w * self_.g0_ * -1.0)));
}
fn scalar_sandwich_point(self: Scalar, other: Point) -> Point {
    let geometric_product: Point = Point(/* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_) * other.g0_));
    return point_geometricProduct_point(geometric_product, scalar_reverse(self_));
}
fn plane_add_flector(self: Plane, other: Flector) -> Flector {
    let addition: Flector = Flector(/* e1, e2, e3, e4 */ other.g0_, /* e423, e431, e412, e321 */ (other.g1_ + self_.g0_));
    return addition;
}
fn horizon_mul_point(self: Horizon, other: Point) -> Motor {
    return horizon_geometricProduct_horizon(self_, other);
}
fn plane_wedge_multiVector(self: Plane, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (-(other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z) - (other.g1_.w * self_.g0_.w))), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.x * self_.g0_.x), (other.g0_.x * self_.g0_.y), (other.g0_.x * self_.g0_.z), (other.g0_.x * self_.g0_.w)));
}
fn horizon_sandwich_motor(self: Horizon, other: Motor) -> Motor {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, other.g0_.w)), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.w)));
    return flector_geometricProduct_flector(geometric_product, horizon_reverse(self_));
}
fn scalar_sub_plane(self: Scalar, other: Plane) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g0_, 0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ (other.g0_ * vec4<f32>(-1.0)));
    return subtraction;
}
fn multiVector_wedge_line(self: MultiVector, other: Line) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (-(other.g0_.x * self_.g3_.x) - (other.g0_.y * self_.g3_.y) - (other.g0_.z * self_.g3_.z) - (other.g1_.x * self_.g2_.x) - (other.g1_.y * self_.g2_.y) - (other.g1_.z * self_.g2_.z))), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>((self_.g0_.x * other.g0_.x), (self_.g0_.x * other.g0_.y), (self_.g0_.x * other.g0_.z)), /* e23, e31, e12 */ vec3<f32>((self_.g0_.x * other.g1_.x), (self_.g0_.x * other.g1_.y), (self_.g0_.x * other.g1_.z)), /* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.y * self_.g1_.z) - (other.g0_.z * self_.g1_.y) + (other.g1_.x * self_.g1_.w)), (-(other.g0_.x * self_.g1_.z) + (other.g0_.z * self_.g1_.x) + (other.g1_.y * self_.g1_.w)), ((other.g0_.x * self_.g1_.y) - (other.g0_.y * self_.g1_.x) + (other.g1_.z * self_.g1_.w)), (-(other.g1_.x * self_.g1_.x) - (other.g1_.y * self_.g1_.y) - (other.g1_.z * self_.g1_.z))));
}
fn flector_wedge_multiVector(self: Flector, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, ((self_.g0_.x * other.g4_.x) + (self_.g0_.y * other.g4_.y) + (self_.g0_.z * other.g4_.z) + (self_.g0_.w * other.g4_.w) - (self_.g1_.x * other.g1_.x) - (self_.g1_.y * other.g1_.y) - (self_.g1_.z * other.g1_.z) - (self_.g1_.w * other.g1_.w))), /* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g0_.x), (other.g0_.x * self_.g0_.y), (other.g0_.x * self_.g0_.z), (other.g0_.x * self_.g0_.w)), /* e41, e42, e43 */ vec3<f32>((-(self_.g0_.x * other.g1_.w) + (self_.g0_.w * other.g1_.x)), (-(self_.g0_.y * other.g1_.w) + (self_.g0_.w * other.g1_.y)), (-(self_.g0_.z * other.g1_.w) + (self_.g0_.w * other.g1_.z))), /* e23, e31, e12 */ vec3<f32>(((self_.g0_.y * other.g1_.z) - (self_.g0_.z * other.g1_.y)), (-(self_.g0_.x * other.g1_.z) + (self_.g0_.z * other.g1_.x)), ((self_.g0_.x * other.g1_.y) - (self_.g0_.y * other.g1_.x))), /* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.x * self_.g1_.x) + (other.g2_.y * self_.g0_.z) - (other.g2_.z * self_.g0_.y) + (other.g3_.x * self_.g0_.w)), ((other.g0_.x * self_.g1_.y) - (other.g2_.x * self_.g0_.z) + (other.g2_.z * self_.g0_.x) + (other.g3_.y * self_.g0_.w)), ((other.g0_.x * self_.g1_.z) + (other.g2_.x * self_.g0_.y) - (other.g2_.y * self_.g0_.x) + (other.g3_.z * self_.g0_.w)), ((other.g0_.x * self_.g1_.w) - (other.g3_.x * self_.g0_.x) - (other.g3_.y * self_.g0_.y) - (other.g3_.z * self_.g0_.z))));
}
fn horizon_mul_horizon(self: Horizon, other: Horizon) -> Scalar {
    return horizon_geometricProduct_horizon(self_, other);
}
fn dualNum_sub_flector(self: DualNum, other: Flector) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ (other.g0_ * vec4<f32>(-1.0)), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ (other.g1_ * vec4<f32>(-1.0)));
    return subtraction;
}
fn flector_sub_origin(self: Flector, other: Origin) -> Flector {
    let subtraction: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, (self_.g0_.w - other.g0_)), /* e423, e431, e412, e321 */ self_.g1_);
    return subtraction;
}
fn point_antiSandwich_scalar(self: Point, other: Scalar) -> Scalar {
    let geometric_anti_product: Horizon = Horizon(/* e321 */ (self_.g0_.w * other.g0_ * -1.0));
    return horizon_geometricAntiProduct_horizon(geometric_anti_product, point_antiReverse(self_));
}
fn point_geometricAntiQuotient_motor(self: Point, other: Motor) -> Point {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2) + pow(other.g0_.w, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Point = Point(/* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_));
    return geometric_anti_product;
}
fn horizon_geometricProduct_plane(self: Horizon, other: Plane) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g0_.x * self_.g0_ * -1.0), (other.g0_.y * self_.g0_ * -1.0), (other.g0_.z * self_.g0_ * -1.0), 0.0), /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.w * self_.g0_ * -1.0)));
}
fn dualNum_geometricQuotient_flector(self: DualNum, other: Flector) -> DualNum {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2) - pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>(inverse.g0_) * self_.g0_));
    return geometric_product;
}
fn multiVector_add_point(self: MultiVector, other: Point) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ (self_.g1_ + other.g0_), /* e41, e42, e43 */ self_.g2_, /* e23, e31, e12 */ self_.g3_, /* e423, e431, e412, e321 */ self_.g4_);
    return addition;
}
fn motor_sandwich_flector(self: Motor, other: Flector) -> Flector {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(0.0, 0.0, 0.0, ((other.g0_.z * self_.g0_.z) - (other.g1_.y * self_.g1_.y) - (other.g1_.z * self_.g1_.z) - (other.g1_.w * self_.g0_.w))) - (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.y, other.g1_.x) * self_.g1_.yzxx) + (vec4<f32>(other.g1_.w, other.g1_.w, other.g1_.w, other.g0_.w) * self_.g1_) + (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.w, self_.g0_.y) * other.g0_.yzzy) + (vec4<f32>(self_.g1_.w, self_.g1_.w, self_.g1_.y, self_.g0_.x) * other.g0_.xyxx)), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.z * self_.g0_.y) + (other.g1_.x * self_.g1_.w) + (other.g1_.y * self_.g1_.z) - (other.g1_.w * self_.g0_.x)), ((other.g0_.x * self_.g0_.z) + (other.g1_.y * self_.g1_.w) + (other.g1_.z * self_.g1_.x) - (other.g1_.w * self_.g0_.y)), ((other.g0_.y * self_.g0_.x) + (other.g1_.x * self_.g1_.y) + (other.g1_.z * self_.g1_.w) - (other.g1_.w * self_.g0_.z)), 0.0) + (vec4<f32>(other.g0_.w, other.g0_.w, other.g0_.w, other.g1_.w) * self_.g1_) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.z) * self_.g1_.yzxz) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.w, self_.g1_.y) * other.g0_.yzzy) - (vec4<f32>(self_.g0_.w, self_.g0_.w, self_.g0_.y, self_.g1_.x) * other.g0_.xyxx)));
    return flector_geometricProduct_flector(geometric_product, motor_reverse(self_));
}
fn horizon_geometricAntiProduct_dualNum(self: Horizon, other: DualNum) -> Horizon {
    return Horizon(/* e321 */ (other.g0_.y * self_.g0_));
}
fn point_add_horizon(self: Point, other: Horizon) -> Flector {
    let addition: Flector = Flector(/* e1, e2, e3, e4 */ self_.g0_, /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other.g0_));
    return addition;
}
fn multiVector_sub_motor(self: MultiVector, other: Motor) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>((other.g1_.w * -1.0), (other.g0_.w * -1.0)) + self_.g0_), /* e1, e2, e3, e4 */ self_.g1_, /* e41, e42, e43 */ (vec3<f32>((other.g0_.x * -1.0), (other.g0_.y * -1.0), (other.g0_.z * -1.0)) + self_.g2_), /* e23, e31, e12 */ (vec3<f32>((other.g1_.x * -1.0), (other.g1_.y * -1.0), (other.g1_.z * -1.0)) + self_.g3_), /* e423, e431, e412, e321 */ self_.g4_);
    return subtraction;
}
fn motor_mul_antiScalar(self: Motor, other: AntiScalar) -> Motor {
    return motor_geometricProduct_motor(self_, other);
}
fn multiVector_geometricAntiProduct_scalar(self: MultiVector, other: Scalar) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((self_.g0_.y * other.g0_), 0.0), /* e1, e2, e3, e4 */ vec4<f32>((self_.g4_.x * other.g0_ * -1.0), (self_.g4_.y * other.g0_ * -1.0), (self_.g4_.z * other.g0_ * -1.0), 0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>((self_.g2_.x * other.g0_), (self_.g2_.y * other.g0_), (self_.g2_.z * other.g0_)), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g1_.w * other.g0_ * -1.0)));
}
fn dualNum_sandwich_origin(self: DualNum, other: Origin) -> Origin {
    let geometric_product: Origin = Origin(/* e4 */ (self_.g0_.x * other.g0_));
    return origin_geometricProduct_origin(geometric_product, dualNum_reverse(self_));
}
fn flector_geometricAntiProduct_dualNum(self: Flector, other: DualNum) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((-(other.g0_.x * self_.g1_.x) + (other.g0_.y * self_.g0_.x)), (-(other.g0_.x * self_.g1_.y) + (other.g0_.y * self_.g0_.y)), (-(other.g0_.x * self_.g1_.z) + (other.g0_.y * self_.g0_.z)), (other.g0_.y * self_.g0_.w)), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.y * self_.g1_.x), (other.g0_.y * self_.g1_.y), (other.g0_.y * self_.g1_.z), (-(other.g0_.x * self_.g0_.w) + (other.g0_.y * self_.g1_.w))));
}
fn point_geometricAntiProduct_motor(self: Point, other: Motor) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((-(other.g0_.y * self_.g0_.z) + (other.g0_.z * self_.g0_.y) + (other.g0_.w * self_.g0_.x) - (other.g1_.x * self_.g0_.w)), ((other.g0_.x * self_.g0_.z) - (other.g0_.z * self_.g0_.x) + (other.g0_.w * self_.g0_.y) - (other.g1_.y * self_.g0_.w)), (-(other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x) + (other.g0_.w * self_.g0_.z) - (other.g1_.z * self_.g0_.w)), (other.g0_.w * self_.g0_.w)), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.x * self_.g0_.w), (other.g0_.y * self_.g0_.w), (other.g0_.z * self_.g0_.w), (-(other.g0_.x * self_.g0_.x) - (other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z) - (other.g1_.w * self_.g0_.w))));
}
fn multiVector_antiSandwich_plane(self: MultiVector, other: Plane) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>((self_.g1_.w * other.g0_.w), 0.0) + (vec2<f32>(other.g0_.x) * vec2<f32>(self_.g1_.x, self_.g4_.x)) + (vec2<f32>(other.g0_.y) * vec2<f32>(self_.g1_.y, self_.g4_.y)) + (vec2<f32>(other.g0_.z) * vec2<f32>(self_.g1_.z, self_.g4_.z))), /* e1, e2, e3, e4 */ (vec4<f32>(((self_.g0_.x * other.g0_.x) + (self_.g2_.x * other.g0_.w) + (self_.g3_.y * other.g0_.z)), ((self_.g0_.x * other.g0_.y) + (self_.g2_.y * other.g0_.w) + (self_.g3_.z * other.g0_.x)), ((self_.g0_.x * other.g0_.z) + (self_.g2_.z * other.g0_.w) + (self_.g3_.x * other.g0_.y)), (-(self_.g2_.y * other.g0_.y) - (self_.g2_.z * other.g0_.z))) - (vec4<f32>(self_.g3_.z, self_.g3_.x, self_.g3_.y, self_.g2_.x) * other.g0_.yzxx)), /* e41, e42, e43 */ (vec3<f32>((-(self_.g4_.y * other.g0_.z) + (self_.g4_.z * other.g0_.y)), ((self_.g4_.x * other.g0_.z) - (self_.g4_.z * other.g0_.x)), (-(self_.g4_.x * other.g0_.y) + (self_.g4_.y * other.g0_.x))) - (vec3<f32>(self_.g1_.w) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z))), /* e23, e31, e12 */ (vec3<f32>((-(self_.g1_.y * other.g0_.z) + (self_.g1_.z * other.g0_.y)), ((self_.g1_.x * other.g0_.z) - (self_.g1_.z * other.g0_.x)), (-(self_.g1_.x * other.g0_.y) + (self_.g1_.y * other.g0_.x))) - (vec3<f32>(self_.g4_.w) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z)) + (vec3<f32>(other.g0_.w) * vec3<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z))), /* e423, e431, e412, e321 */ (vec4<f32>((self_.g2_.y * other.g0_.z), (self_.g2_.z * other.g0_.x), (self_.g2_.x * other.g0_.y), (-(self_.g3_.y * other.g0_.y) - (self_.g3_.z * other.g0_.z))) + (vec4<f32>(self_.g0_.y) * other.g0_) - (vec4<f32>(self_.g2_.z, self_.g2_.x, self_.g2_.y, self_.g3_.x) * other.g0_.yzxx)));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self_));
}
fn motor_geometricAntiProduct_multiVector(self: Motor, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(((other.g0_.x * self_.g0_.w) + (other.g0_.y * self_.g1_.w) - (other.g2_.x * self_.g1_.x) - (other.g2_.y * self_.g1_.y) - (other.g2_.z * self_.g1_.z) - (other.g3_.x * self_.g0_.x) - (other.g3_.y * self_.g0_.y) - (other.g3_.z * self_.g0_.z)), ((other.g0_.y * self_.g0_.w) - (other.g2_.x * self_.g0_.x) - (other.g2_.y * self_.g0_.y) - (other.g2_.z * self_.g0_.z))), /* e1, e2, e3, e4 */ vec4<f32>(((self_.g0_.x * other.g4_.w) + (self_.g0_.y * other.g1_.z) - (self_.g0_.z * other.g1_.y) + (self_.g0_.w * other.g1_.x) + (self_.g1_.x * other.g1_.w) + (self_.g1_.y * other.g4_.z) - (self_.g1_.z * other.g4_.y) + (self_.g1_.w * other.g4_.x)), (-(self_.g0_.x * other.g1_.z) + (self_.g0_.y * other.g4_.w) + (self_.g0_.z * other.g1_.x) + (self_.g0_.w * other.g1_.y) - (self_.g1_.x * other.g4_.z) + (self_.g1_.y * other.g1_.w) + (self_.g1_.z * other.g4_.x) + (self_.g1_.w * other.g4_.y)), ((self_.g0_.x * other.g1_.y) - (self_.g0_.y * other.g1_.x) + (self_.g0_.z * other.g4_.w) + (self_.g0_.w * other.g1_.z) + (self_.g1_.x * other.g4_.y) - (self_.g1_.y * other.g4_.x) + (self_.g1_.z * other.g1_.w) + (self_.g1_.w * other.g4_.z)), (-(self_.g0_.x * other.g4_.x) - (self_.g0_.y * other.g4_.y) - (self_.g0_.z * other.g4_.z) + (self_.g0_.w * other.g1_.w))), /* e41, e42, e43 */ vec3<f32>(((other.g0_.y * self_.g0_.x) + (other.g2_.x * self_.g0_.w) - (other.g2_.y * self_.g0_.z) + (other.g2_.z * self_.g0_.y)), ((other.g0_.y * self_.g0_.y) + (other.g2_.x * self_.g0_.z) + (other.g2_.y * self_.g0_.w) - (other.g2_.z * self_.g0_.x)), ((other.g0_.y * self_.g0_.z) - (other.g2_.x * self_.g0_.y) + (other.g2_.y * self_.g0_.x) + (other.g2_.z * self_.g0_.w))), /* e23, e31, e12 */ vec3<f32>(((other.g0_.x * self_.g0_.x) + (other.g0_.y * self_.g1_.x) + (other.g2_.x * self_.g1_.w) - (other.g2_.y * self_.g1_.z) + (other.g2_.z * self_.g1_.y) + (other.g3_.x * self_.g0_.w) - (other.g3_.y * self_.g0_.z) + (other.g3_.z * self_.g0_.y)), ((other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g1_.y) + (other.g2_.x * self_.g1_.z) + (other.g2_.y * self_.g1_.w) - (other.g2_.z * self_.g1_.x) + (other.g3_.x * self_.g0_.z) + (other.g3_.y * self_.g0_.w) - (other.g3_.z * self_.g0_.x)), ((other.g0_.x * self_.g0_.z) + (other.g0_.y * self_.g1_.z) - (other.g2_.x * self_.g1_.y) + (other.g2_.y * self_.g1_.x) + (other.g2_.z * self_.g1_.w) - (other.g3_.x * self_.g0_.y) + (other.g3_.y * self_.g0_.x) + (other.g3_.z * self_.g0_.w))), /* e423, e431, e412, e321 */ vec4<f32>(((self_.g0_.x * other.g1_.w) + (self_.g0_.y * other.g4_.z) - (self_.g0_.z * other.g4_.y) + (self_.g0_.w * other.g4_.x)), (-(self_.g0_.x * other.g4_.z) + (self_.g0_.y * other.g1_.w) + (self_.g0_.z * other.g4_.x) + (self_.g0_.w * other.g4_.y)), ((self_.g0_.x * other.g4_.y) - (self_.g0_.y * other.g4_.x) + (self_.g0_.z * other.g1_.w) + (self_.g0_.w * other.g4_.z)), (-(self_.g0_.x * other.g1_.x) - (self_.g0_.y * other.g1_.y) - (self_.g0_.z * other.g1_.z) + (self_.g0_.w * other.g4_.w) - (self_.g1_.x * other.g4_.x) - (self_.g1_.y * other.g4_.y) - (self_.g1_.z * other.g4_.z) + (self_.g1_.w * other.g1_.w))));
}
fn flector_sub_line(self: Flector, other: Line) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ self_.g0_, /* e41, e42, e43 */ (other.g0_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (other.g1_ * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ self_.g1_);
    return subtraction;
}
fn multiVector_bitXor_dualNum(self: MultiVector, other: DualNum) -> MultiVector {
    return multiVector_wedge_multiVector(self_, other);
}
fn antiScalar_sandwich_point(self: AntiScalar, other: Point) -> Origin {
    let geometric_product: Plane = Plane(/* e423, e431, e412, e321 */ vec4<f32>((other.g0_.x * self_.g0_ * -1.0), (other.g0_.y * self_.g0_ * -1.0), (other.g0_.z * self_.g0_ * -1.0), 0.0));
    return plane_geometricProduct_plane(geometric_product, antiScalar_reverse(self_));
}
fn motor_geometricQuotient_flector(self: Motor, other: Flector) -> Motor {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2) - pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(inverse.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(inverse.g0_) * self_.g1_));
    return geometric_product;
}
fn multiVector_add_dualNum(self: MultiVector, other: DualNum) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ (other.g0_ + self_.g0_), /* e1, e2, e3, e4 */ self_.g1_, /* e41, e42, e43 */ self_.g2_, /* e23, e31, e12 */ self_.g3_, /* e423, e431, e412, e321 */ self_.g4_);
    return addition;
}
fn line_bitXor_motor(self: Line, other: Motor) -> Motor {
    return line_wedge_line(self_, other);
}
fn point_bitXor_scalar(self: Point, other: Scalar) -> Point {
    return point_wedge_point(self_, other);
}
fn antiScalar_geometricProduct_line(self: AntiScalar, other: Line) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_)), /* e23, e31, e12 */ vec3<f32>(0.0));
}
fn origin_geometricQuotient_multiVector(self: Origin, other: MultiVector) -> Origin {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) - pow(other.g3_.x, 2) - pow(other.g3_.y, 2) - pow(other.g3_.z, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2) - pow(other.g4_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Origin = Origin(/* e4 */ (self_.g0_ * inverse.g0_));
    return geometric_product;
}
fn multiVector_bitXor_scalar(self: MultiVector, other: Scalar) -> MultiVector {
    return multiVector_wedge_multiVector(self_, other);
}
fn point_antiSandwich_multiVector(self: Point, other: MultiVector) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(((other.g4_.x * self_.g0_.x) + (other.g4_.y * self_.g0_.y) + (other.g4_.z * self_.g0_.z) + (other.g4_.w * self_.g0_.w)), (other.g1_.w * self_.g0_.w * -1.0)), /* e1, e2, e3, e4 */ vec4<f32>(((other.g0_.y * self_.g0_.x) - (other.g2_.y * self_.g0_.z) + (other.g2_.z * self_.g0_.y) - (other.g3_.x * self_.g0_.w)), ((other.g0_.y * self_.g0_.y) + (other.g2_.x * self_.g0_.z) - (other.g2_.z * self_.g0_.x) - (other.g3_.y * self_.g0_.w)), ((other.g0_.y * self_.g0_.z) - (other.g2_.x * self_.g0_.y) + (other.g2_.y * self_.g0_.x) - (other.g3_.z * self_.g0_.w)), (other.g0_.y * self_.g0_.w)), /* e41, e42, e43 */ (vec3<f32>(self_.g0_.w) * vec3<f32>(other.g4_.x, other.g4_.y, other.g4_.z) * vec3<f32>(-1.0)), /* e23, e31, e12 */ (vec3<f32>(((other.g4_.y * self_.g0_.z) - (other.g4_.z * self_.g0_.y)), (-(other.g4_.x * self_.g0_.z) + (other.g4_.z * self_.g0_.x)), ((other.g4_.x * self_.g0_.y) - (other.g4_.y * self_.g0_.x))) - (vec3<f32>(other.g1_.w) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z)) + (vec3<f32>(self_.g0_.w) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z))), /* e423, e431, e412, e321 */ vec4<f32>((other.g2_.x * self_.g0_.w), (other.g2_.y * self_.g0_.w), (other.g2_.z * self_.g0_.w), (-(other.g0_.x * self_.g0_.w) - (other.g2_.x * self_.g0_.x) - (other.g2_.y * self_.g0_.y) - (other.g2_.z * self_.g0_.z))));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, point_antiReverse(self_));
}
fn motor_geometricAntiQuotient_point(self: Motor, other: Point) -> Motor {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.w, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(anti_inverse.g0_) * self_.g1_));
    return geometric_anti_product;
}
fn line_antiSandwich_scalar(self: Line, other: Scalar) -> Motor {
    let geometric_anti_product: Line = Line(/* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ (vec3<f32>(other.g0_) * self_.g0_));
    return line_geometricAntiProduct_line(geometric_anti_product, line_antiReverse(self_));
}
fn antiScalar_geometricQuotient_plane(self: AntiScalar, other: Plane) -> AntiScalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.w, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: AntiScalar = AntiScalar(/* e1234 */ (self_.g0_ * inverse.g0_));
    return geometric_product;
}
fn horizon_sub_flector(self: Horizon, other: Flector) -> Flector {
    let subtraction: Flector = Flector(/* e1, e2, e3, e4 */ (other.g0_ * vec4<f32>(-1.0)), /* e423, e431, e412, e321 */ vec4<f32>((other.g1_.x * -1.0), (other.g1_.y * -1.0), (other.g1_.z * -1.0), (-other.g1_.w + self_.g0_)));
    return subtraction;
}
fn scalar_geometricAntiProduct_plane(self: Scalar, other: Plane) -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), 0.0));
}
fn line_antiSandwich_flector(self: Line, other: Flector) -> Flector {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(((self_.g0_.x * other.g1_.w) + (self_.g0_.y * other.g0_.z) - (self_.g0_.z * other.g0_.y) + (self_.g1_.x * other.g0_.w) + (self_.g1_.y * other.g1_.z)), (-(self_.g0_.x * other.g0_.z) + (self_.g0_.y * other.g1_.w) + (self_.g0_.z * other.g0_.x) + (self_.g1_.y * other.g0_.w) + (self_.g1_.z * other.g1_.x)), ((self_.g0_.x * other.g0_.y) - (self_.g0_.y * other.g0_.x) + (self_.g0_.z * other.g1_.w) + (self_.g1_.x * other.g1_.y) + (self_.g1_.z * other.g0_.w)), (-(self_.g0_.y * other.g1_.y) - (self_.g0_.z * other.g1_.z))) - (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g0_.x) * other.g1_.yzxx)), /* e423, e431, e412, e321 */ (vec4<f32>(((self_.g0_.x * other.g0_.w) + (self_.g0_.y * other.g1_.z)), ((self_.g0_.y * other.g0_.w) + (self_.g0_.z * other.g1_.x)), ((self_.g0_.x * other.g1_.y) + (self_.g0_.z * other.g0_.w)), (-(self_.g0_.x * other.g0_.x) - (self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z) - (self_.g1_.y * other.g1_.y) - (self_.g1_.z * other.g1_.z))) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g1_.x) * other.g1_.yzxx)));
    return flector_geometricAntiProduct_flector(geometric_anti_product, line_antiReverse(self_));
}
fn origin_geometricQuotient_horizon(self: Origin, other: Horizon) -> Origin {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Origin = Origin(/* e4 */ (self_.g0_ * inverse.g0_));
    return geometric_product;
}
fn scalar_geometricAntiProduct_antiScalar(self: Scalar, other: AntiScalar) -> Scalar {
    return Scalar(/* scalar */ (other.g0_ * self_.g0_));
}
fn motor_wedge_dualNum(self: Motor, other: DualNum) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g0_.x * self_.g0_.x), (other.g0_.x * self_.g0_.y), (other.g0_.x * self_.g0_.z), ((other.g0_.x * self_.g0_.w) + (other.g0_.y * self_.g1_.w))), /* e23, e31, e12, scalar */ vec4<f32>((other.g0_.x * self_.g1_.x), (other.g0_.x * self_.g1_.y), (other.g0_.x * self_.g1_.z), (other.g0_.x * self_.g1_.w)));
}
fn antiScalar_geometricAntiProduct_multiVector(self: AntiScalar, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_)), /* e1, e2, e3, e4 */ vec4<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_), (other.g1_.w * self_.g0_)), /* e41, e42, e43 */ vec3<f32>((other.g2_.x * self_.g0_), (other.g2_.y * self_.g0_), (other.g2_.z * self_.g0_)), /* e23, e31, e12 */ vec3<f32>((other.g3_.x * self_.g0_), (other.g3_.y * self_.g0_), (other.g3_.z * self_.g0_)), /* e423, e431, e412, e321 */ vec4<f32>((other.g4_.x * self_.g0_), (other.g4_.y * self_.g0_), (other.g4_.z * self_.g0_), (other.g4_.w * self_.g0_)));
}
fn line_geometricAntiProduct_point(self: Line, other: Point) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(((self_.g0_.y * other.g0_.z) - (self_.g0_.z * other.g0_.y) + (self_.g1_.x * other.g0_.w)), (-(self_.g0_.x * other.g0_.z) + (self_.g0_.z * other.g0_.x) + (self_.g1_.y * other.g0_.w)), ((self_.g0_.x * other.g0_.y) - (self_.g0_.y * other.g0_.x) + (self_.g1_.z * other.g0_.w)), 0.0), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.x * other.g0_.w), (self_.g0_.y * other.g0_.w), (self_.g0_.z * other.g0_.w), (-(self_.g0_.x * other.g0_.x) - (self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z))));
}
fn plane_geometricQuotient_flector(self: Plane, other: Flector) -> Plane {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2) - pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Plane = Plane(/* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g0_));
    return geometric_product;
}
fn scalar_sandwich_multiVector(self: Scalar, other: MultiVector) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(self_.g0_) * other.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_) * other.g1_), /* e41, e42, e43 */ (vec3<f32>(self_.g0_) * other.g2_), /* e23, e31, e12 */ (vec3<f32>(self_.g0_) * other.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_) * other.g4_));
    return multiVector_geometricProduct_multiVector(geometric_product, scalar_reverse(self_));
}
fn antiScalar_sub_flector(self: AntiScalar, other: Flector) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, self_.g0_), /* e1, e2, e3, e4 */ (other.g0_ * vec4<f32>(-1.0)), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ (other.g1_ * vec4<f32>(-1.0)));
    return subtraction;
}
fn scalar_geometricAntiProduct_line(self: Scalar, other: Line) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_)));
}
fn flector_mul_scalar(self: Flector, other: Scalar) -> Flector {
    return flector_geometricProduct_flector(self_, other);
}
fn multiVector_mul_antiScalar(self: MultiVector, other: AntiScalar) -> MultiVector {
    return multiVector_geometricProduct_multiVector(self_, other);
}
fn dualNum_antiSandwich_origin(self: DualNum, other: Origin) -> Flector {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.y * other.g0_)), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.x * other.g0_)));
    return flector_geometricAntiProduct_flector(geometric_anti_product, dualNum_antiReverse(self_));
}
fn flector_add_dualNum(self: Flector, other: DualNum) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ other.g0_, /* e1, e2, e3, e4 */ self_.g0_, /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ self_.g1_);
    return addition;
}
fn antiScalar_geometricAntiQuotient_line(self: AntiScalar, other: Line) -> AntiScalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: AntiScalar = AntiScalar(/* e1234 */ (anti_inverse.g0_ * self_.g0_));
    return geometric_anti_product;
}
fn motor_wedge_multiVector(self: Motor, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((other.g0_.x * self_.g1_.w), ((other.g0_.x * self_.g0_.w) + (other.g0_.y * self_.g1_.w) - (other.g2_.x * self_.g1_.x) - (other.g2_.y * self_.g1_.y) - (other.g2_.z * self_.g1_.z) - (other.g3_.x * self_.g0_.x) - (other.g3_.y * self_.g0_.y) - (other.g3_.z * self_.g0_.z))), /* e1, e2, e3, e4 */ vec4<f32>((self_.g1_.w * other.g1_.x), (self_.g1_.w * other.g1_.y), (self_.g1_.w * other.g1_.z), (self_.g1_.w * other.g1_.w)), /* e41, e42, e43 */ vec3<f32>(((other.g0_.x * self_.g0_.x) + (other.g2_.x * self_.g1_.w)), ((other.g0_.x * self_.g0_.y) + (other.g2_.y * self_.g1_.w)), ((other.g0_.x * self_.g0_.z) + (other.g2_.z * self_.g1_.w))), /* e23, e31, e12 */ vec3<f32>(((other.g0_.x * self_.g1_.x) + (other.g3_.x * self_.g1_.w)), ((other.g0_.x * self_.g1_.y) + (other.g3_.y * self_.g1_.w)), ((other.g0_.x * self_.g1_.z) + (other.g3_.z * self_.g1_.w))), /* e423, e431, e412, e321 */ vec4<f32>(((self_.g0_.y * other.g1_.z) - (self_.g0_.z * other.g1_.y) + (self_.g1_.x * other.g1_.w) + (self_.g1_.w * other.g4_.x)), (-(self_.g0_.x * other.g1_.z) + (self_.g0_.z * other.g1_.x) + (self_.g1_.y * other.g1_.w) + (self_.g1_.w * other.g4_.y)), ((self_.g0_.x * other.g1_.y) - (self_.g0_.y * other.g1_.x) + (self_.g1_.z * other.g1_.w) + (self_.g1_.w * other.g4_.z)), (-(self_.g1_.x * other.g1_.x) - (self_.g1_.y * other.g1_.y) - (self_.g1_.z * other.g1_.z) + (self_.g1_.w * other.g4_.w))));
}
fn motor_bitXor_flector(self: Motor, other: Flector) -> Flector {
    return motor_wedge_motor(self_, other);
}
fn plane_antiWedge_origin(self: Plane, other: Origin) -> Scalar {
    return Scalar(/* scalar */ (self_.g0_.w * other.g0_ * -1.0));
}
fn line_antiSandwich_antiScalar(self: Line, other: AntiScalar) -> Motor {
    let geometric_anti_product: Line = Line(/* e41, e42, e43 */ (vec3<f32>(other.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(other.g0_) * self_.g1_));
    return line_geometricAntiProduct_line(geometric_anti_product, line_antiReverse(self_));
}
fn flector_geometricAntiQuotient_origin(self: Flector, other: Origin) -> Flector {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g1_));
    return geometric_anti_product;
}
fn line_add_scalar(self: Line, other: Scalar) -> Motor {
    let addition: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, 0.0), /* e23, e31, e12, scalar */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, other.g0_));
    return addition;
}
fn origin_bitXor_plane(self: Origin, other: Plane) -> AntiScalar {
    return origin_wedge_origin(self_, other);
}
fn antiScalar_sub_point(self: AntiScalar, other: Point) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, self_.g0_), /* e1, e2, e3, e4 */ (other.g0_ * vec4<f32>(-1.0)), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    return subtraction;
}
fn antiScalar_antiSandwich_plane(self: AntiScalar, other: Plane) -> Plane {
    let geometric_anti_product: Plane = Plane(/* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_) * other.g0_));
    return plane_geometricAntiProduct_plane(geometric_anti_product, antiScalar_antiReverse(self_));
}
fn scalar_mul_line(self: Scalar, other: Line) -> Line {
    return scalar_geometricProduct_scalar(self_, other);
}
fn plane_bitXor_flector(self: Plane, other: Flector) -> AntiScalar {
    return plane_wedge_plane(self_, other);
}
fn multiVector_add_line(self: MultiVector, other: Line) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ self_.g1_, /* e41, e42, e43 */ (other.g0_ + self_.g2_), /* e23, e31, e12 */ (other.g1_ + self_.g3_), /* e423, e431, e412, e321 */ self_.g4_);
    return addition;
}
fn flector_antiSandwich_motor(self: Flector, other: Motor) -> Motor {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(((self_.g0_.y * other.g0_.z) - (self_.g0_.w * other.g1_.x) + (self_.g1_.z * other.g1_.y) + (self_.g1_.w * other.g0_.x)), ((self_.g0_.z * other.g0_.x) - (self_.g0_.w * other.g1_.y) + (self_.g1_.x * other.g1_.z) + (self_.g1_.w * other.g0_.y)), ((self_.g0_.z * other.g0_.w) - (self_.g0_.w * other.g1_.z) + (self_.g1_.y * other.g1_.x) + (self_.g1_.w * other.g0_.z)), 0.0) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g1_.x) * other.g0_.yzxx) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.w, other.g0_.z) * self_.g1_.yzzz) - (vec4<f32>(other.g1_.w, other.g1_.w, other.g1_.y, other.g0_.y) * self_.g1_.xyxy) + (self_.g0_.xyxw * other.g0_.wwyw)), /* e423, e431, e412, e321 */ (vec4<f32>(0.0, 0.0, 0.0, (-(self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z) - (self_.g0_.w * other.g1_.w) + (self_.g1_.z * other.g1_.z))) + (vec4<f32>(self_.g0_.w, self_.g0_.w, self_.g0_.w, self_.g1_.w) * other.g0_) - (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g0_.x) * other.g0_.yzxx) + (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.w, other.g1_.y) * self_.g1_.yzzy) + (vec4<f32>(other.g0_.w, other.g0_.w, other.g0_.y, other.g1_.x) * self_.g1_.xyxx)));
    return flector_geometricAntiProduct_flector(geometric_anti_product, flector_antiReverse(self_));
}
fn dualNum_bitXor_horizon(self: DualNum, other: Horizon) -> Horizon {
    return dualNum_wedge_dualNum(self_, other);
}
fn horizon_sandwich_point(self: Horizon, other: Point) -> Flector {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.w * self_.g0_ * -1.0)), /* e23, e31, e12, scalar */ vec4<f32>((other.g0_.x * self_.g0_ * -1.0), (other.g0_.y * self_.g0_ * -1.0), (other.g0_.z * self_.g0_ * -1.0), 0.0));
    return motor_geometricProduct_motor(geometric_product, horizon_reverse(self_));
}
fn multiVector_add_antiScalar(self: MultiVector, other: AntiScalar) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g0_.x, (self_.g0_.y + other.g0_)), /* e1, e2, e3, e4 */ self_.g1_, /* e41, e42, e43 */ self_.g2_, /* e23, e31, e12 */ self_.g3_, /* e423, e431, e412, e321 */ self_.g4_);
    return addition;
}
fn multiVector_sandwich_origin(self: MultiVector, other: Origin) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (self_.g4_.w * other.g0_ * -1.0)), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.x * other.g0_)), /* e41, e42, e43 */ (vec3<f32>(other.g0_) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z) * vec3<f32>(-1.0)), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>((self_.g3_.x * other.g0_), (self_.g3_.y * other.g0_), (self_.g3_.z * other.g0_), 0.0));
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self_));
}
fn line_sub_flector(self: Line, other: Flector) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ (other.g0_ * vec4<f32>(-1.0)), /* e41, e42, e43 */ self_.g0_, /* e23, e31, e12 */ self_.g1_, /* e423, e431, e412, e321 */ (other.g1_ * vec4<f32>(-1.0)));
    return subtraction;
}
fn multiVector_sub_plane(self: MultiVector, other: Plane) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ self_.g1_, /* e41, e42, e43 */ self_.g2_, /* e23, e31, e12 */ self_.g3_, /* e423, e431, e412, e321 */ (self_.g4_ - other.g0_));
    return subtraction;
}
fn multiVector_mul_plane(self: MultiVector, other: Plane) -> MultiVector {
    return multiVector_geometricProduct_multiVector(self_, other);
}
fn flector_sandwich_dualNum(self: Flector, other: DualNum) -> Motor {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g0_.x), (other.g0_.x * self_.g0_.y), (other.g0_.x * self_.g0_.z), ((other.g0_.x * self_.g0_.w) + (other.g0_.y * self_.g1_.w))), /* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.x * self_.g1_.x) + (other.g0_.y * self_.g0_.x)), ((other.g0_.x * self_.g1_.y) + (other.g0_.y * self_.g0_.y)), ((other.g0_.x * self_.g1_.z) + (other.g0_.y * self_.g0_.z)), (other.g0_.x * self_.g1_.w)));
    return flector_geometricProduct_flector(geometric_product, flector_reverse(self_));
}
fn point_geometricProduct_antiScalar(self: Point, other: AntiScalar) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), 0.0));
}
fn horizon_add_motor(self: Horizon, other: Motor) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(other.g1_.w, other.g0_.w), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z), /* e23, e31, e12 */ vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_));
    return addition;
}
fn flector_sub_horizon(self: Flector, other: Horizon) -> Flector {
    let subtraction: Flector = Flector(/* e1, e2, e3, e4 */ self_.g0_, /* e423, e431, e412, e321 */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, (self_.g1_.w - other.g0_)));
    return subtraction;
}
fn motor_sandwich_origin(self: Motor, other: Origin) -> Flector {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g1_.w * other.g0_)), /* e423, e431, e412, e321 */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), 0.0));
    return flector_geometricProduct_flector(geometric_product, motor_reverse(self_));
}
fn point_wedge_point(self: Point, other: Point) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>(((other.g0_.x * self_.g0_.w) - (other.g0_.w * self_.g0_.x)), ((other.g0_.y * self_.g0_.w) - (other.g0_.w * self_.g0_.y)), ((other.g0_.z * self_.g0_.w) - (other.g0_.w * self_.g0_.z))), /* e23, e31, e12 */ vec3<f32>((-(other.g0_.y * self_.g0_.z) + (other.g0_.z * self_.g0_.y)), ((other.g0_.x * self_.g0_.z) - (other.g0_.z * self_.g0_.x)), (-(other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x))));
}
fn motor_add_multiVector(self: Motor, other: MultiVector) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(self_.g1_.w, self_.g0_.w) + other.g0_), /* e1, e2, e3, e4 */ other.g1_, /* e41, e42, e43 */ (vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z) + other.g2_), /* e23, e31, e12 */ (vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z) + other.g3_), /* e423, e431, e412, e321 */ other.g4_);
    return addition;
}
fn scalar_add_point(self: Scalar, other: Point) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g0_, 0.0), /* e1, e2, e3, e4 */ other.g0_, /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    return addition;
}
fn scalar_mul_scalar(self: Scalar, other: Scalar) -> Scalar {
    return scalar_geometricProduct_scalar(self_, other);
}
fn horizon_mul_plane(self: Horizon, other: Plane) -> Motor {
    return horizon_geometricProduct_horizon(self_, other);
}
fn flector_antiWedge_horizon(self: Flector, other: Horizon) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0), /* e23, e31, e12, scalar */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), (self_.g0_.w * other.g0_)));
}
fn scalar_geometricProduct_line(self: Scalar, other: Line) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_)), /* e23, e31, e12 */ vec3<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_)));
}
fn flector_sub_antiScalar(self: Flector, other: AntiScalar) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (other.g0_ * -1.0)), /* e1, e2, e3, e4 */ self_.g0_, /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ self_.g1_);
    return subtraction;
}
fn scalar_geometricAntiQuotient_flector(self: Scalar, other: Flector) -> Scalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.w, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Scalar = Scalar(/* scalar */ (anti_inverse.g0_ * self_.g0_));
    return geometric_anti_product;
}
fn dualNum_antiWedge_flector(self: DualNum, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.y * other.g0_.x), (self_.g0_.y * other.g0_.y), (self_.g0_.y * other.g0_.z), (self_.g0_.y * other.g0_.w)), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.y * other.g1_.x), (self_.g0_.y * other.g1_.y), (self_.g0_.y * other.g1_.z), (self_.g0_.y * other.g1_.w)));
}
fn multiVector_sandwich_antiScalar(self: MultiVector, other: AntiScalar) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (self_.g0_.x * other.g0_)), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g4_.w * other.g0_)), /* e41, e42, e43 */ (vec3<f32>(other.g0_) * self_.g3_), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), 0.0));
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self_));
}
fn multiVector_geometricProduct_horizon(self: MultiVector, other: Horizon) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((self_.g4_.w * other.g0_ * -1.0), (self_.g1_.w * other.g0_)), /* e1, e2, e3, e4 */ vec4<f32>((self_.g3_.x * other.g0_), (self_.g3_.y * other.g0_), (self_.g3_.z * other.g0_), (self_.g0_.y * other.g0_ * -1.0)), /* e41, e42, e43 */ vec3<f32>((self_.g4_.x * other.g0_), (self_.g4_.y * other.g0_), (self_.g4_.z * other.g0_)), /* e23, e31, e12 */ vec3<f32>((self_.g1_.x * other.g0_ * -1.0), (self_.g1_.y * other.g0_ * -1.0), (self_.g1_.z * other.g0_ * -1.0)), /* e423, e431, e412, e321 */ vec4<f32>((self_.g2_.x * other.g0_ * -1.0), (self_.g2_.y * other.g0_ * -1.0), (self_.g2_.z * other.g0_ * -1.0), (self_.g0_.x * other.g0_)));
}
fn plane_geometricAntiQuotient_plane(self: Plane, other: Plane) -> Plane {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Plane = Plane(/* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_));
    return geometric_anti_product;
}
fn motor_geometricAntiProduct_motor(self: Motor, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(((other.g0_.x * self_.g0_.w) - (other.g0_.y * self_.g0_.z) + (other.g0_.z * self_.g0_.y) + (other.g0_.w * self_.g0_.x)), ((other.g0_.x * self_.g0_.z) + (other.g0_.y * self_.g0_.w) - (other.g0_.z * self_.g0_.x) + (other.g0_.w * self_.g0_.y)), (-(other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x) + (other.g0_.z * self_.g0_.w) + (other.g0_.w * self_.g0_.z)), (-(other.g0_.x * self_.g0_.x) - (other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z) + (other.g0_.w * self_.g0_.w))), /* e23, e31, e12, scalar */ vec4<f32>(((other.g0_.x * self_.g1_.w) - (other.g0_.y * self_.g1_.z) + (other.g0_.z * self_.g1_.y) + (other.g0_.w * self_.g1_.x) + (other.g1_.x * self_.g0_.w) - (other.g1_.y * self_.g0_.z) + (other.g1_.z * self_.g0_.y) + (other.g1_.w * self_.g0_.x)), ((other.g0_.x * self_.g1_.z) + (other.g0_.y * self_.g1_.w) - (other.g0_.z * self_.g1_.x) + (other.g0_.w * self_.g1_.y) + (other.g1_.x * self_.g0_.z) + (other.g1_.y * self_.g0_.w) - (other.g1_.z * self_.g0_.x) + (other.g1_.w * self_.g0_.y)), (-(other.g0_.x * self_.g1_.y) + (other.g0_.y * self_.g1_.x) + (other.g0_.z * self_.g1_.w) + (other.g0_.w * self_.g1_.z) - (other.g1_.x * self_.g0_.y) + (other.g1_.y * self_.g0_.x) + (other.g1_.z * self_.g0_.w) + (other.g1_.w * self_.g0_.z)), (-(other.g0_.x * self_.g1_.x) - (other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) + (other.g0_.w * self_.g1_.w) - (other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z) + (other.g1_.w * self_.g0_.w))));
}
fn horizon_add_scalar(self: Horizon, other: Scalar) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(other.g0_, 0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_));
    return addition;
}
fn plane_geometricQuotient_multiVector(self: Plane, other: MultiVector) -> Plane {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) - pow(other.g3_.x, 2) - pow(other.g3_.y, 2) - pow(other.g3_.z, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2) - pow(other.g4_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Plane = Plane(/* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g0_));
    return geometric_product;
}
fn dualNum_antiSandwich_line(self: DualNum, other: Line) -> Line {
    let geometric_anti_product: Line = Line(/* e41, e42, e43 */ (vec3<f32>(self_.g0_.y) * other.g0_), /* e23, e31, e12 */ ((vec3<f32>(self_.g0_.x) * other.g0_) + (vec3<f32>(self_.g0_.y) * other.g1_)));
    return line_geometricAntiProduct_line(geometric_anti_product, dualNum_antiReverse(self_));
}
fn line_sandwich_multiVector(self: Line, other: MultiVector) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(0.0, (-(self_.g1_.x * other.g2_.x) - (self_.g1_.y * other.g2_.y) - (self_.g1_.z * other.g2_.z))) - (vec2<f32>(other.g3_.x) * vec2<f32>(self_.g1_.x, self_.g0_.x)) - (vec2<f32>(other.g3_.y) * vec2<f32>(self_.g1_.y, self_.g0_.y)) - (vec2<f32>(other.g3_.z) * vec2<f32>(self_.g1_.z, self_.g0_.z))), /* e1, e2, e3, e4 */ (vec4<f32>(((self_.g1_.x * other.g4_.w) - (self_.g1_.y * other.g1_.z)), ((self_.g1_.y * other.g4_.w) - (self_.g1_.z * other.g1_.x)), (-(self_.g1_.x * other.g1_.y) + (self_.g1_.z * other.g4_.w)), ((self_.g0_.y * other.g1_.y) + (self_.g0_.z * other.g1_.z) - (self_.g1_.x * other.g4_.x) - (self_.g1_.y * other.g4_.y) - (self_.g1_.z * other.g4_.z))) + (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g0_.x) * other.g1_.yzxx)), /* e41, e42, e43 */ ((vec3<f32>(other.g0_.x) * self_.g0_) + (vec3<f32>(other.g0_.y) * self_.g1_) - (self_.g0_.yzx * other.g3_.zxy) + (self_.g0_.zxy * other.g3_.yzx) - (self_.g1_.yzx * other.g2_.zxy) + (self_.g1_.zxy * other.g2_.yzx)), /* e23, e31, e12 */ ((vec3<f32>(other.g0_.x) * self_.g1_) - (self_.g1_.yzx * other.g3_.zxy) + (self_.g1_.zxy * other.g3_.yzx)), /* e423, e431, e412, e321 */ (vec4<f32>((-(self_.g0_.x * other.g4_.w) + (self_.g0_.y * other.g1_.z) + (self_.g1_.x * other.g1_.w) - (self_.g1_.y * other.g4_.z) + (self_.g1_.z * other.g4_.y)), (-(self_.g0_.y * other.g4_.w) + (self_.g0_.z * other.g1_.x) + (self_.g1_.x * other.g4_.z) + (self_.g1_.y * other.g1_.w) - (self_.g1_.z * other.g4_.x)), ((self_.g0_.x * other.g1_.y) - (self_.g0_.z * other.g4_.w) - (self_.g1_.x * other.g4_.y) + (self_.g1_.y * other.g4_.x) + (self_.g1_.z * other.g1_.w)), (-(self_.g1_.y * other.g1_.y) - (self_.g1_.z * other.g1_.z))) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g1_.x) * other.g1_.yzxx)));
    return multiVector_geometricProduct_multiVector(geometric_product, line_reverse(self_));
}
fn point_antiSandwich_horizon(self: Point, other: Horizon) -> Horizon {
    let geometric_anti_product: Scalar = Scalar(/* scalar */ (self_.g0_.w * other.g0_));
    return scalar_geometricAntiProduct_scalar(geometric_anti_product, point_antiReverse(self_));
}
fn line_sub_point(self: Line, other: Point) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ (other.g0_ * vec4<f32>(-1.0)), /* e41, e42, e43 */ self_.g0_, /* e23, e31, e12 */ self_.g1_, /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    return subtraction;
}
fn plane_geometricAntiProduct_point(self: Plane, other: Point) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g0_.x * other.g0_.w * -1.0), (self_.g0_.y * other.g0_.w * -1.0), (self_.g0_.z * other.g0_.w * -1.0), 0.0), /* e23, e31, e12, scalar */ vec4<f32>(((self_.g0_.y * other.g0_.z) - (self_.g0_.z * other.g0_.y)), (-(self_.g0_.x * other.g0_.z) + (self_.g0_.z * other.g0_.x)), ((self_.g0_.x * other.g0_.y) - (self_.g0_.y * other.g0_.x)), (-(self_.g0_.x * other.g0_.x) - (self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z) - (self_.g0_.w * other.g0_.w))));
}
fn origin_sub_scalar(self: Origin, other: Scalar) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>((other.g0_ * -1.0), 0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    return subtraction;
}
fn plane_mul_scalar(self: Plane, other: Scalar) -> Plane {
    return plane_geometricProduct_plane(self_, other);
}
fn point_wedge_scalar(self: Point, other: Scalar) -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), (self_.g0_.w * other.g0_)));
}
fn point_antiSandwich_plane(self: Point, other: Plane) -> Flector {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g0_.x * self_.g0_.w * -1.0), (other.g0_.y * self_.g0_.w * -1.0), (other.g0_.z * self_.g0_.w * -1.0), 0.0), /* e23, e31, e12, scalar */ (vec4<f32>(((other.g0_.z * self_.g0_.y) * -1.0), ((other.g0_.x * self_.g0_.z) * -1.0), ((other.g0_.y * self_.g0_.x) * -1.0), ((other.g0_.y * self_.g0_.y) + (other.g0_.z * self_.g0_.z) + (other.g0_.w * self_.g0_.w))) + (other.g0_.yzxx * self_.g0_.zxyx)));
    return motor_geometricAntiProduct_motor(geometric_anti_product, point_antiReverse(self_));
}
fn motor_sandwich_multiVector(self: Motor, other: MultiVector) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(0.0, ((other.g0_.y * self_.g1_.w) - (other.g3_.x * self_.g0_.x) - (other.g3_.y * self_.g0_.y) - (other.g3_.z * self_.g0_.z))) + (vec2<f32>(other.g0_.x) * vec2<f32>(self_.g1_.w, self_.g0_.w)) - (vec2<f32>(self_.g1_.x) * vec2<f32>(other.g3_.x, other.g2_.x)) - (vec2<f32>(self_.g1_.y) * vec2<f32>(other.g3_.y, other.g2_.y)) - (vec2<f32>(self_.g1_.z) * vec2<f32>(other.g3_.z, other.g2_.z))), /* e1, e2, e3, e4 */ (vec4<f32>((self_.g1_.w * other.g1_.x), (self_.g1_.y * other.g4_.w), (self_.g1_.z * other.g4_.w), ((self_.g0_.y * other.g1_.y) + (self_.g0_.z * other.g1_.z) - (self_.g0_.w * other.g4_.w) - (self_.g1_.y * other.g4_.y) - (self_.g1_.z * other.g4_.z))) + (vec4<f32>(self_.g1_.z, self_.g1_.w, self_.g1_.w, self_.g0_.x) * other.g1_.yyzx) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g4_.x) * self_.g1_.yzxx) + (vec4<f32>(other.g4_.w, other.g1_.z, other.g1_.x, other.g1_.w) * self_.g1_.xxyw)), /* e41, e42, e43 */ ((vec3<f32>(other.g0_.x) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z)) + (vec3<f32>(other.g0_.y) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z)) - (vec3<f32>(self_.g0_.y, self_.g0_.z, self_.g0_.x) * other.g3_.zxy) + (vec3<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.w) * other.g3_.yzz) + (vec3<f32>(self_.g0_.w, self_.g0_.w, self_.g0_.y) * other.g3_.xyx) - (vec3<f32>(self_.g1_.y, self_.g1_.z, self_.g1_.x) * other.g2_.zxy) + (vec3<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.w) * other.g2_.yzz) + (vec3<f32>(self_.g1_.w, self_.g1_.w, self_.g1_.y) * other.g2_.xyx)), /* e23, e31, e12 */ ((vec3<f32>(other.g0_.x) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z)) - (vec3<f32>(self_.g1_.y, self_.g1_.z, self_.g1_.x) * other.g3_.zxy) + (vec3<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.w) * other.g3_.yzz) + (vec3<f32>(self_.g1_.w, self_.g1_.w, self_.g1_.y) * other.g3_.xyx)), /* e423, e431, e412, e321 */ (vec4<f32>((-(self_.g0_.x * other.g4_.w) + (self_.g0_.y * other.g1_.z) + (self_.g1_.z * other.g4_.y) + (self_.g1_.w * other.g4_.x)), (-(self_.g0_.y * other.g4_.w) + (self_.g0_.z * other.g1_.x) + (self_.g1_.y * other.g1_.w) + (self_.g1_.w * other.g4_.y)), ((self_.g0_.x * other.g1_.y) - (self_.g0_.z * other.g4_.w) + (self_.g1_.z * other.g1_.w) + (self_.g1_.w * other.g4_.z)), 0.0) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g1_.x) * other.g1_.yzxx) - (vec4<f32>(self_.g0_.w, self_.g0_.w, self_.g0_.w, self_.g1_.y) * other.g1_.xyzy) + (vec4<f32>(other.g1_.w, other.g4_.z, other.g4_.x, other.g4_.w) * self_.g1_.xxyw) - (vec4<f32>(other.g4_.z, other.g4_.x, other.g4_.y, other.g1_.z) * self_.g1_.yzxz)));
    return multiVector_geometricProduct_multiVector(geometric_product, motor_reverse(self_));
}
fn antiScalar_antiWedge_point(self: AntiScalar, other: Point) -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), (other.g0_.w * self_.g0_)));
}
fn motor_antiWedge_point(self: Motor, other: Point) -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.w * other.g0_.x), (self_.g0_.w * other.g0_.y), (self_.g0_.w * other.g0_.z), (self_.g0_.w * other.g0_.w)));
}
fn horizon_geometricAntiQuotient_line(self: Horizon, other: Line) -> Horizon {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Horizon = Horizon(/* e321 */ (anti_inverse.g0_ * self_.g0_));
    return geometric_anti_product;
}
fn plane_antiSandwich_dualNum(self: Plane, other: DualNum) -> Motor {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g0_.x * -1.0), (other.g0_.x * self_.g0_.y * -1.0), (other.g0_.x * self_.g0_.z * -1.0), 0.0), /* e423, e431, e412, e321 */ (vec4<f32>(other.g0_.y) * self_.g0_));
    return flector_geometricAntiProduct_flector(geometric_anti_product, plane_antiReverse(self_));
}
fn point_geometricQuotient_flector(self: Point, other: Flector) -> Point {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2) - pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Point = Point(/* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g0_));
    return geometric_product;
}
fn origin_antiWedge_dualNum(self: Origin, other: DualNum) -> Origin {
    return Origin(/* e4 */ (other.g0_.y * self_.g0_));
}
fn origin_add_multiVector(self: Origin, other: MultiVector) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ other.g0_, /* e1, e2, e3, e4 */ vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, (other.g1_.w + self_.g0_)), /* e41, e42, e43 */ other.g2_, /* e23, e31, e12 */ other.g3_, /* e423, e431, e412, e321 */ other.g4_);
    return addition;
}
fn plane_add_horizon(self: Plane, other: Horizon) -> Plane {
    let addition: Plane = Plane(/* e423, e431, e412, e321 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, (self_.g0_.w + other.g0_)));
    return addition;
}
fn plane_sandwich_horizon(self: Plane, other: Horizon) -> Flector {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), 0.0), /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.w * other.g0_ * -1.0)));
    return motor_geometricProduct_motor(geometric_product, plane_reverse(self_));
}
fn antiScalar_add_scalar(self: AntiScalar, other: Scalar) -> DualNum {
    let addition: DualNum = DualNum(/* scalar, e1234 */ vec2<f32>(other.g0_, self_.g0_));
    return addition;
}
fn scalar_geometricAntiQuotient_point(self: Scalar, other: Point) -> Scalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.w, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Scalar = Scalar(/* scalar */ (anti_inverse.g0_ * self_.g0_));
    return geometric_anti_product;
}
fn point_bitXor_origin(self: Point, other: Origin) -> Line {
    return point_wedge_point(self_, other);
}
fn line_wedge_point(self: Line, other: Point) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>(((self_.g0_.y * other.g0_.z) - (self_.g0_.z * other.g0_.y) + (self_.g1_.x * other.g0_.w)), (-(self_.g0_.x * other.g0_.z) + (self_.g0_.z * other.g0_.x) + (self_.g1_.y * other.g0_.w)), ((self_.g0_.x * other.g0_.y) - (self_.g0_.y * other.g0_.x) + (self_.g1_.z * other.g0_.w)), (-(self_.g1_.x * other.g0_.x) - (self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))));
}
fn horizon_wedge_point(self: Horizon, other: Point) -> AntiScalar {
    return AntiScalar(/* e1234 */ (other.g0_.w * self_.g0_ * -1.0));
}
fn plane_geometricAntiQuotient_dualNum(self: Plane, other: DualNum) -> Plane {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_.y, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Plane = Plane(/* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_));
    return geometric_anti_product;
}
fn point_geometricAntiQuotient_origin(self: Point, other: Origin) -> Point {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Point = Point(/* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_));
    return geometric_anti_product;
}
fn antiScalar_geometricQuotient_horizon(self: AntiScalar, other: Horizon) -> AntiScalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: AntiScalar = AntiScalar(/* e1234 */ (self_.g0_ * inverse.g0_));
    return geometric_product;
}
fn point_bitXor_dualNum(self: Point, other: DualNum) -> Point {
    return point_wedge_point(self_, other);
}
fn multiVector_geometricAntiQuotient_plane(self: MultiVector, other: Plane) -> MultiVector {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(anti_inverse.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(anti_inverse.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(anti_inverse.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g4_));
    return geometric_anti_product;
}
fn point_geometricAntiQuotient_dualNum(self: Point, other: DualNum) -> Point {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_.y, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Point = Point(/* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_));
    return geometric_anti_product;
}
fn scalar_geometricQuotient_point(self: Scalar, other: Point) -> Scalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Scalar = Scalar(/* scalar */ (inverse.g0_ * self_.g0_));
    return geometric_product;
}
fn plane_sandwich_flector(self: Plane, other: Flector) -> Flector {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(((other.g0_.y * self_.g0_.z) + (other.g1_.w * self_.g0_.x)), ((other.g0_.z * self_.g0_.x) + (other.g1_.w * self_.g0_.y)), ((other.g0_.x * self_.g0_.y) + (other.g1_.w * self_.g0_.z)), (-(other.g0_.z * self_.g0_.z) - (other.g0_.w * self_.g0_.w))) - (vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, other.g0_.y) * self_.g0_.wwwy) - (other.g0_.zxyx * self_.g0_.yzxx)), /* e23, e31, e12, scalar */ (vec4<f32>(self_.g0_.w) * vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.w) * vec4<f32>(-1.0)));
    return motor_geometricProduct_motor(geometric_product, plane_reverse(self_));
}
fn flector_geometricQuotient_flector(self: Flector, other: Flector) -> Flector {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2) - pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g1_));
    return geometric_product;
}
fn motor_sub_horizon(self: Motor, other: Horizon) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g1_.w, self_.g0_.w), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z), /* e23, e31, e12 */ vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_ * -1.0)));
    return subtraction;
}
fn plane_geometricProduct_antiScalar(self: Plane, other: AntiScalar) -> Origin {
    return Origin(/* e4 */ (self_.g0_.w * other.g0_));
}
fn origin_add_dualNum(self: Origin, other: DualNum) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ other.g0_, /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    return addition;
}
fn horizon_bitXor_motor(self: Horizon, other: Motor) -> Horizon {
    return horizon_wedge_horizon(self_, other);
}
fn dualNum_antiWedge_motor(self: DualNum, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g0_.y * other.g0_.x), (self_.g0_.y * other.g0_.y), (self_.g0_.y * other.g0_.z), (self_.g0_.y * other.g0_.w)), /* e23, e31, e12, scalar */ vec4<f32>((self_.g0_.y * other.g1_.x), (self_.g0_.y * other.g1_.y), (self_.g0_.y * other.g1_.z), ((self_.g0_.x * other.g0_.w) + (self_.g0_.y * other.g1_.w))));
}
fn origin_geometricAntiProduct_flector(self: Origin, other: Flector) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g1_.x * self_.g0_ * -1.0), (other.g1_.y * self_.g0_ * -1.0), (other.g1_.z * self_.g0_ * -1.0), (other.g0_.w * self_.g0_ * -1.0)), /* e23, e31, e12, scalar */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), (other.g1_.w * self_.g0_)));
}
fn multiVector_bitXor_motor(self: MultiVector, other: Motor) -> MultiVector {
    return multiVector_wedge_multiVector(self_, other);
}
fn point_geometricProduct_scalar(self: Point, other: Scalar) -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), (self_.g0_.w * other.g0_)));
}
fn multiVector_antiWedge_point(self: MultiVector, other: Point) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((-(self_.g4_.x * other.g0_.x) - (self_.g4_.y * other.g0_.y) - (self_.g4_.z * other.g0_.z) - (self_.g4_.w * other.g0_.w)), 0.0), /* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.y * other.g0_.x), (self_.g0_.y * other.g0_.y), (self_.g0_.y * other.g0_.z), (self_.g0_.y * other.g0_.w)), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn motor_geometricAntiProduct_plane(self: Motor, other: Plane) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(((self_.g0_.x * other.g0_.w) + (self_.g1_.y * other.g0_.z) - (self_.g1_.z * other.g0_.y) + (self_.g1_.w * other.g0_.x)), ((self_.g0_.y * other.g0_.w) - (self_.g1_.x * other.g0_.z) + (self_.g1_.z * other.g0_.x) + (self_.g1_.w * other.g0_.y)), ((self_.g0_.z * other.g0_.w) + (self_.g1_.x * other.g0_.y) - (self_.g1_.y * other.g0_.x) + (self_.g1_.w * other.g0_.z)), (-(self_.g0_.x * other.g0_.x) - (self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z))), /* e423, e431, e412, e321 */ vec4<f32>(((self_.g0_.y * other.g0_.z) - (self_.g0_.z * other.g0_.y) + (self_.g0_.w * other.g0_.x)), (-(self_.g0_.x * other.g0_.z) + (self_.g0_.z * other.g0_.x) + (self_.g0_.w * other.g0_.y)), ((self_.g0_.x * other.g0_.y) - (self_.g0_.y * other.g0_.x) + (self_.g0_.w * other.g0_.z)), ((self_.g0_.w * other.g0_.w) - (self_.g1_.x * other.g0_.x) - (self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))));
}
fn motor_add_antiScalar(self: Motor, other: AntiScalar) -> Motor {
    let addition: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, (self_.g0_.w + other.g0_)), /* e23, e31, e12, scalar */ self_.g1_);
    return addition;
}
fn dualNum_geometricProduct_multiVector(self: DualNum, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((self_.g0_.x * other.g0_.x), ((self_.g0_.x * other.g0_.y) + (self_.g0_.y * other.g0_.x))), /* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g1_.x), (self_.g0_.x * other.g1_.y), (self_.g0_.x * other.g1_.z), ((self_.g0_.x * other.g1_.w) - (self_.g0_.y * other.g4_.w))), /* e41, e42, e43 */ vec3<f32>(((self_.g0_.x * other.g2_.x) + (self_.g0_.y * other.g3_.x)), ((self_.g0_.x * other.g2_.y) + (self_.g0_.y * other.g3_.y)), ((self_.g0_.x * other.g2_.z) + (self_.g0_.y * other.g3_.z))), /* e23, e31, e12 */ vec3<f32>((self_.g0_.x * other.g3_.x), (self_.g0_.x * other.g3_.y), (self_.g0_.x * other.g3_.z)), /* e423, e431, e412, e321 */ vec4<f32>(((self_.g0_.x * other.g4_.x) - (self_.g0_.y * other.g1_.x)), ((self_.g0_.x * other.g4_.y) - (self_.g0_.y * other.g1_.y)), ((self_.g0_.x * other.g4_.z) - (self_.g0_.y * other.g1_.z)), (self_.g0_.x * other.g4_.w)));
}
fn flector_geometricQuotient_dualNum(self: Flector, other: DualNum) -> Flector {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_.x, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g1_));
    return geometric_product;
}
fn plane_geometricAntiProduct_horizon(self: Plane, other: Horizon) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_)));
}
fn point_add_scalar(self: Point, other: Scalar) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(other.g0_, 0.0), /* e1, e2, e3, e4 */ self_.g0_, /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    return addition;
}
fn flector_antiWedge_line(self: Flector, other: Line) -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>(((other.g0_.x * self_.g1_.w) + (other.g1_.y * self_.g1_.z) - (other.g1_.z * self_.g1_.y)), ((other.g0_.y * self_.g1_.w) - (other.g1_.x * self_.g1_.z) + (other.g1_.z * self_.g1_.x)), ((other.g0_.z * self_.g1_.w) + (other.g1_.x * self_.g1_.y) - (other.g1_.y * self_.g1_.x)), (-(other.g0_.x * self_.g1_.x) - (other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z))));
}
fn multiVector_wedge_dualNum(self: MultiVector, other: DualNum) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((other.g0_.x * self_.g0_.x), ((other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x))), /* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g1_.x), (other.g0_.x * self_.g1_.y), (other.g0_.x * self_.g1_.z), (other.g0_.x * self_.g1_.w)), /* e41, e42, e43 */ vec3<f32>((other.g0_.x * self_.g2_.x), (other.g0_.x * self_.g2_.y), (other.g0_.x * self_.g2_.z)), /* e23, e31, e12 */ vec3<f32>((other.g0_.x * self_.g3_.x), (other.g0_.x * self_.g3_.y), (other.g0_.x * self_.g3_.z)), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.x * self_.g4_.x), (other.g0_.x * self_.g4_.y), (other.g0_.x * self_.g4_.z), (other.g0_.x * self_.g4_.w)));
}
fn point_geometricProduct_origin(self: Point, other: Origin) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>((self_.g0_.x * other.g0_ * -1.0), (self_.g0_.y * other.g0_ * -1.0), (self_.g0_.z * other.g0_ * -1.0)), /* e23, e31, e12 */ vec3<f32>(0.0));
}
fn dualNum_geometricAntiProduct_origin(self: DualNum, other: Origin) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.y * other.g0_)), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.x * other.g0_)));
}
fn scalar_antiSandwich_flector(self: Scalar, other: Flector) -> Flector {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_), 0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.w * self_.g0_)));
    return flector_geometricAntiProduct_flector(geometric_anti_product, scalar_antiReverse(self_));
}
fn origin_antiSandwich_scalar(self: Origin, other: Scalar) -> Scalar {
    let geometric_anti_product: Horizon = Horizon(/* e321 */ (self_.g0_ * other.g0_ * -1.0));
    return horizon_geometricAntiProduct_horizon(geometric_anti_product, origin_antiReverse(self_));
}
fn plane_antiSandwich_horizon(self: Plane, other: Horizon) -> Flector {
    let geometric_anti_product: Line = Line(/* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ (vec3<f32>(other.g0_) * vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z)));
    return line_geometricAntiProduct_line(geometric_anti_product, plane_antiReverse(self_));
}
fn dualNum_geometricAntiQuotient_origin(self: DualNum, other: Origin) -> DualNum {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>(anti_inverse.g0_) * self_.g0_));
    return geometric_anti_product;
}
fn flector_geometricAntiProduct_motor(self: Flector, other: Motor) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(((self_.g0_.x * other.g0_.w) + (self_.g0_.y * other.g0_.z) - (self_.g0_.z * other.g0_.y) - (self_.g0_.w * other.g1_.x) - (self_.g1_.x * other.g1_.w) - (self_.g1_.y * other.g1_.z) + (self_.g1_.z * other.g1_.y) + (self_.g1_.w * other.g0_.x)), (-(self_.g0_.x * other.g0_.z) + (self_.g0_.y * other.g0_.w) + (self_.g0_.z * other.g0_.x) - (self_.g0_.w * other.g1_.y) + (self_.g1_.x * other.g1_.z) - (self_.g1_.y * other.g1_.w) - (self_.g1_.z * other.g1_.x) + (self_.g1_.w * other.g0_.y)), ((self_.g0_.x * other.g0_.y) - (self_.g0_.y * other.g0_.x) + (self_.g0_.z * other.g0_.w) - (self_.g0_.w * other.g1_.z) - (self_.g1_.x * other.g1_.y) + (self_.g1_.y * other.g1_.x) - (self_.g1_.z * other.g1_.w) + (self_.g1_.w * other.g0_.z)), ((self_.g0_.w * other.g0_.w) - (self_.g1_.x * other.g0_.x) - (self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))), /* e423, e431, e412, e321 */ vec4<f32>(((self_.g0_.w * other.g0_.x) + (self_.g1_.x * other.g0_.w) + (self_.g1_.y * other.g0_.z) - (self_.g1_.z * other.g0_.y)), ((self_.g0_.w * other.g0_.y) - (self_.g1_.x * other.g0_.z) + (self_.g1_.y * other.g0_.w) + (self_.g1_.z * other.g0_.x)), ((self_.g0_.w * other.g0_.z) + (self_.g1_.x * other.g0_.y) - (self_.g1_.y * other.g0_.x) + (self_.g1_.z * other.g0_.w)), (-(self_.g0_.x * other.g0_.x) - (self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z) - (self_.g0_.w * other.g1_.w) + (self_.g1_.x * other.g1_.x) + (self_.g1_.y * other.g1_.y) + (self_.g1_.z * other.g1_.z) + (self_.g1_.w * other.g0_.w))));
}
fn plane_geometricQuotient_dualNum(self: Plane, other: DualNum) -> Plane {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_.x, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Plane = Plane(/* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g0_));
    return geometric_product;
}
fn motor_geometricQuotient_multiVector(self: Motor, other: MultiVector) -> Motor {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) - pow(other.g3_.x, 2) - pow(other.g3_.y, 2) - pow(other.g3_.z, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2) - pow(other.g4_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(inverse.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(inverse.g0_) * self_.g1_));
    return geometric_product;
}
fn point_sandwich_plane(self: Point, other: Plane) -> Flector {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(((other.g0_.y * self_.g0_.z) * -1.0), ((other.g0_.z * self_.g0_.x) * -1.0), ((other.g0_.x * self_.g0_.y) * -1.0), ((other.g0_.y * self_.g0_.y) + (other.g0_.z * self_.g0_.z) + (other.g0_.w * self_.g0_.w))) + (other.g0_.zxyx * self_.g0_.yzxx)), /* e23, e31, e12, scalar */ vec4<f32>((other.g0_.w * self_.g0_.x * -1.0), (other.g0_.w * self_.g0_.y * -1.0), (other.g0_.w * self_.g0_.z * -1.0), 0.0));
    return motor_geometricProduct_motor(geometric_product, point_reverse(self_));
}
fn multiVector_wedge_point(self: MultiVector, other: Point) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (-(self_.g4_.x * other.g0_.x) - (self_.g4_.y * other.g0_.y) - (self_.g4_.z * other.g0_.z) - (self_.g4_.w * other.g0_.w))), /* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g0_.x), (self_.g0_.x * other.g0_.y), (self_.g0_.x * other.g0_.z), (self_.g0_.x * other.g0_.w)), /* e41, e42, e43 */ vec3<f32>((-(self_.g1_.x * other.g0_.w) + (self_.g1_.w * other.g0_.x)), (-(self_.g1_.y * other.g0_.w) + (self_.g1_.w * other.g0_.y)), (-(self_.g1_.z * other.g0_.w) + (self_.g1_.w * other.g0_.z))), /* e23, e31, e12 */ vec3<f32>(((self_.g1_.y * other.g0_.z) - (self_.g1_.z * other.g0_.y)), (-(self_.g1_.x * other.g0_.z) + (self_.g1_.z * other.g0_.x)), ((self_.g1_.x * other.g0_.y) - (self_.g1_.y * other.g0_.x))), /* e423, e431, e412, e321 */ vec4<f32>(((self_.g2_.y * other.g0_.z) - (self_.g2_.z * other.g0_.y) + (self_.g3_.x * other.g0_.w)), (-(self_.g2_.x * other.g0_.z) + (self_.g2_.z * other.g0_.x) + (self_.g3_.y * other.g0_.w)), ((self_.g2_.x * other.g0_.y) - (self_.g2_.y * other.g0_.x) + (self_.g3_.z * other.g0_.w)), (-(self_.g3_.x * other.g0_.x) - (self_.g3_.y * other.g0_.y) - (self_.g3_.z * other.g0_.z))));
}
fn antiScalar_antiWedge_flector(self: AntiScalar, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), (other.g0_.w * self_.g0_)), /* e423, e431, e412, e321 */ vec4<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_), (other.g1_.w * self_.g0_)));
}
fn scalar_antiSandwich_motor(self: Scalar, other: Motor) -> Motor {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0), /* e23, e31, e12, scalar */ (vec4<f32>(self_.g0_) * other.g0_));
    return motor_geometricAntiProduct_motor(geometric_anti_product, scalar_antiReverse(self_));
}
fn flector_geometricQuotient_multiVector(self: Flector, other: MultiVector) -> Flector {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) - pow(other.g3_.x, 2) - pow(other.g3_.y, 2) - pow(other.g3_.z, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2) - pow(other.g4_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g1_));
    return geometric_product;
}
fn multiVector_geometricQuotient_dualNum(self: MultiVector, other: DualNum) -> MultiVector {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_.x, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(inverse.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(inverse.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(inverse.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g4_));
    return geometric_product;
}
fn horizon_wedge_origin(self: Horizon, other: Origin) -> AntiScalar {
    return AntiScalar(/* e1234 */ (self_.g0_ * other.g0_ * -1.0));
}
fn line_sandwich_dualNum(self: Line, other: DualNum) -> Motor {
    let geometric_product: Line = Line(/* e41, e42, e43 */ ((vec3<f32>(other.g0_.x) * self_.g0_) + (vec3<f32>(other.g0_.y) * self_.g1_)), /* e23, e31, e12 */ (vec3<f32>(other.g0_.x) * self_.g1_));
    return line_geometricProduct_line(geometric_product, line_reverse(self_));
}
fn flector_sub_multiVector(self: Flector, other: MultiVector) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ (other.g0_ * vec2<f32>(-1.0)), /* e1, e2, e3, e4 */ (self_.g0_ - other.g1_), /* e41, e42, e43 */ (other.g2_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (other.g3_ * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ (self_.g1_ - other.g4_));
    return subtraction;
}
fn multiVector_sandwich_motor(self: MultiVector, other: Motor) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(0.0, ((self_.g0_.y * other.g1_.w) - (self_.g3_.x * other.g0_.x) - (self_.g3_.y * other.g0_.y) - (self_.g3_.z * other.g0_.z))) + (vec2<f32>(self_.g0_.x) * vec2<f32>(other.g1_.w, other.g0_.w)) - (vec2<f32>(other.g1_.x) * vec2<f32>(self_.g3_.x, self_.g2_.x)) - (vec2<f32>(other.g1_.y) * vec2<f32>(self_.g3_.y, self_.g2_.y)) - (vec2<f32>(other.g1_.z) * vec2<f32>(self_.g3_.z, self_.g2_.z))), /* e1, e2, e3, e4 */ (vec4<f32>(((other.g1_.y * self_.g1_.z) + (other.g1_.w * self_.g1_.x)), ((other.g1_.z * self_.g1_.x) + (other.g1_.w * self_.g1_.y)), ((other.g1_.z * self_.g4_.w) + (other.g1_.w * self_.g1_.z)), (-(other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) + (other.g0_.w * self_.g4_.w) - (other.g1_.x * self_.g4_.x) - (other.g1_.y * self_.g4_.y) - (other.g1_.z * self_.g4_.z))) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.x) * self_.g1_.yzxx) + (vec4<f32>(self_.g4_.w, self_.g4_.w, self_.g1_.y, self_.g1_.w) * other.g1_.xyxw)), /* e41, e42, e43 */ ((vec3<f32>(self_.g0_.x) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z)) + (vec3<f32>(self_.g0_.y) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z)) + (vec3<f32>(other.g0_.y, other.g0_.w, other.g0_.w) * self_.g3_.zyz) - (vec3<f32>(other.g0_.z, other.g0_.x, other.g0_.y) * self_.g3_.yzx) + (vec3<f32>(other.g0_.w, other.g0_.z, other.g0_.x) * self_.g3_.xxy) + (vec3<f32>(other.g1_.y, other.g1_.w, other.g1_.w) * self_.g2_.zyz) - (vec3<f32>(other.g1_.z, other.g1_.x, other.g1_.y) * self_.g2_.yzx) + (vec3<f32>(other.g1_.w, other.g1_.z, other.g1_.x) * self_.g2_.xxy)), /* e23, e31, e12 */ ((vec3<f32>(self_.g0_.x) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z)) + (vec3<f32>(other.g1_.y, other.g1_.w, other.g1_.w) * self_.g3_.zyz) - (vec3<f32>(other.g1_.z, other.g1_.x, other.g1_.y) * self_.g3_.yzx) + (vec3<f32>(other.g1_.w, other.g1_.z, other.g1_.x) * self_.g3_.xxy)), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.y * self_.g1_.z) + (other.g0_.w * self_.g1_.x) + (other.g1_.x * self_.g1_.w) + (other.g1_.y * self_.g4_.z) + (other.g1_.w * self_.g4_.x)), ((other.g0_.z * self_.g1_.x) + (other.g0_.w * self_.g1_.y) + (other.g1_.y * self_.g1_.w) + (other.g1_.z * self_.g4_.x) + (other.g1_.w * self_.g4_.y)), ((other.g0_.x * self_.g1_.y) + (other.g0_.w * self_.g1_.z) + (other.g1_.x * self_.g4_.y) + (other.g1_.z * self_.g1_.w) + (other.g1_.w * self_.g4_.z)), ((other.g1_.z * self_.g1_.z) * -1.0)) + (vec4<f32>(self_.g4_.w) * vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.w)) - (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.y, other.g1_.x) * self_.g1_.yzxx) - (vec4<f32>(self_.g4_.y, self_.g4_.z, self_.g4_.x, self_.g1_.y) * other.g1_.zxyy)));
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self_));
}
fn antiScalar_geometricAntiQuotient_multiVector(self: AntiScalar, other: MultiVector) -> AntiScalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.y, 2) - pow(other.g2_.x, 2) - pow(other.g2_.y, 2) - pow(other.g2_.z, 2) - pow(other.g1_.w, 2) + pow(other.g4_.x, 2) + pow(other.g4_.y, 2) + pow(other.g4_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: AntiScalar = AntiScalar(/* e1234 */ (anti_inverse.g0_ * self_.g0_));
    return geometric_anti_product;
}
fn horizon_wedge_flector(self: Horizon, other: Flector) -> AntiScalar {
    return AntiScalar(/* e1234 */ (other.g0_.w * self_.g0_ * -1.0));
}
fn flector_bitXor_dualNum(self: Flector, other: DualNum) -> Flector {
    return flector_wedge_flector(self_, other);
}
fn origin_geometricProduct_horizon(self: Origin, other: Horizon) -> AntiScalar {
    return AntiScalar(/* e1234 */ (other.g0_ * self_.g0_));
}
fn flector_add_point(self: Flector, other: Point) -> Flector {
    let addition: Flector = Flector(/* e1, e2, e3, e4 */ (self_.g0_ + other.g0_), /* e423, e431, e412, e321 */ self_.g1_);
    return addition;
}
fn flector_mul_motor(self: Flector, other: Motor) -> Flector {
    return flector_geometricProduct_flector(self_, other);
}
fn flector_antiSandwich_point(self: Flector, other: Point) -> Flector {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(other.g0_.w) * vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, self_.g0_.w) * vec4<f32>(-1.0)), /* e23, e31, e12, scalar */ (vec4<f32>(((self_.g0_.w * other.g0_.x) + (self_.g1_.y * other.g0_.z)), ((self_.g0_.w * other.g0_.y) + (self_.g1_.z * other.g0_.x)), ((self_.g0_.w * other.g0_.z) + (self_.g1_.x * other.g0_.y)), (-(self_.g1_.z * other.g0_.z) - (self_.g1_.w * other.g0_.w))) - (vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g1_.x) * other.g0_.wwwx) - (self_.g1_.zxyy * other.g0_.yzxy)));
    return motor_geometricAntiProduct_motor(geometric_anti_product, flector_antiReverse(self_));
}
fn antiScalar_geometricAntiQuotient_plane(self: AntiScalar, other: Plane) -> AntiScalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: AntiScalar = AntiScalar(/* e1234 */ (anti_inverse.g0_ * self_.g0_));
    return geometric_anti_product;
}
fn flector_geometricProduct_multiVector(self: Flector, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(((self_.g0_.x * other.g1_.x) + (self_.g0_.y * other.g1_.y) + (self_.g0_.z * other.g1_.z) - (self_.g1_.w * other.g4_.w)), ((self_.g0_.x * other.g4_.x) + (self_.g0_.y * other.g4_.y) + (self_.g0_.z * other.g4_.z) + (self_.g0_.w * other.g4_.w) - (self_.g1_.x * other.g1_.x) - (self_.g1_.y * other.g1_.y) - (self_.g1_.z * other.g1_.z) - (self_.g1_.w * other.g1_.w))), /* e1, e2, e3, e4 */ vec4<f32>(((other.g0_.x * self_.g0_.x) + (other.g3_.x * self_.g1_.w) + (other.g3_.y * self_.g0_.z) - (other.g3_.z * self_.g0_.y)), ((other.g0_.x * self_.g0_.y) - (other.g3_.x * self_.g0_.z) + (other.g3_.y * self_.g1_.w) + (other.g3_.z * self_.g0_.x)), ((other.g0_.x * self_.g0_.z) + (other.g3_.x * self_.g0_.y) - (other.g3_.y * self_.g0_.x) + (other.g3_.z * self_.g1_.w)), ((other.g0_.x * self_.g0_.w) + (other.g0_.y * self_.g1_.w) - (other.g2_.x * self_.g0_.x) - (other.g2_.y * self_.g0_.y) - (other.g2_.z * self_.g0_.z) - (other.g3_.x * self_.g1_.x) - (other.g3_.y * self_.g1_.y) - (other.g3_.z * self_.g1_.z))), /* e41, e42, e43 */ vec3<f32>((-(self_.g0_.x * other.g1_.w) + (self_.g0_.y * other.g4_.z) - (self_.g0_.z * other.g4_.y) + (self_.g0_.w * other.g1_.x) + (self_.g1_.x * other.g4_.w) - (self_.g1_.y * other.g1_.z) + (self_.g1_.z * other.g1_.y) - (self_.g1_.w * other.g4_.x)), (-(self_.g0_.x * other.g4_.z) - (self_.g0_.y * other.g1_.w) + (self_.g0_.z * other.g4_.x) + (self_.g0_.w * other.g1_.y) + (self_.g1_.x * other.g1_.z) + (self_.g1_.y * other.g4_.w) - (self_.g1_.z * other.g1_.x) - (self_.g1_.w * other.g4_.y)), ((self_.g0_.x * other.g4_.y) - (self_.g0_.y * other.g4_.x) - (self_.g0_.z * other.g1_.w) + (self_.g0_.w * other.g1_.z) - (self_.g1_.x * other.g1_.y) + (self_.g1_.y * other.g1_.x) + (self_.g1_.z * other.g4_.w) - (self_.g1_.w * other.g4_.z))), /* e23, e31, e12 */ vec3<f32>((-(self_.g0_.x * other.g4_.w) + (self_.g0_.y * other.g1_.z) - (self_.g0_.z * other.g1_.y) - (self_.g1_.w * other.g1_.x)), (-(self_.g0_.x * other.g1_.z) - (self_.g0_.y * other.g4_.w) + (self_.g0_.z * other.g1_.x) - (self_.g1_.w * other.g1_.y)), ((self_.g0_.x * other.g1_.y) - (self_.g0_.y * other.g1_.x) - (self_.g0_.z * other.g4_.w) - (self_.g1_.w * other.g1_.z))), /* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.x * self_.g1_.x) + (other.g0_.y * self_.g0_.x) + (other.g2_.x * self_.g1_.w) + (other.g2_.y * self_.g0_.z) - (other.g2_.z * self_.g0_.y) + (other.g3_.x * self_.g0_.w) + (other.g3_.y * self_.g1_.z) - (other.g3_.z * self_.g1_.y)), ((other.g0_.x * self_.g1_.y) + (other.g0_.y * self_.g0_.y) - (other.g2_.x * self_.g0_.z) + (other.g2_.y * self_.g1_.w) + (other.g2_.z * self_.g0_.x) - (other.g3_.x * self_.g1_.z) + (other.g3_.y * self_.g0_.w) + (other.g3_.z * self_.g1_.x)), ((other.g0_.x * self_.g1_.z) + (other.g0_.y * self_.g0_.z) + (other.g2_.x * self_.g0_.y) - (other.g2_.y * self_.g0_.x) + (other.g2_.z * self_.g1_.w) + (other.g3_.x * self_.g1_.y) - (other.g3_.y * self_.g1_.x) + (other.g3_.z * self_.g0_.w)), ((other.g0_.x * self_.g1_.w) - (other.g3_.x * self_.g0_.x) - (other.g3_.y * self_.g0_.y) - (other.g3_.z * self_.g0_.z))));
}
fn flector_sandwich_line(self: Flector, other: Line) -> Motor {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(((other.g1_.x * self_.g1_.w) + (other.g1_.y * self_.g0_.z)), ((other.g1_.y * self_.g1_.w) + (other.g1_.z * self_.g0_.x)), ((other.g1_.x * self_.g0_.y) + (other.g1_.z * self_.g1_.w)), (-(other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z) - (other.g1_.x * self_.g1_.x) - (other.g1_.y * self_.g1_.y) - (other.g1_.z * self_.g1_.z))) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.x) * self_.g0_.yzxx)), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.x * self_.g1_.w) + (other.g0_.y * self_.g0_.z) + (other.g1_.x * self_.g0_.w) + (other.g1_.y * self_.g1_.z) - (other.g1_.z * self_.g1_.y)), ((other.g0_.y * self_.g1_.w) + (other.g0_.z * self_.g0_.x) - (other.g1_.x * self_.g1_.z) + (other.g1_.y * self_.g0_.w) + (other.g1_.z * self_.g1_.x)), ((other.g0_.x * self_.g0_.y) + (other.g0_.z * self_.g1_.w) + (other.g1_.x * self_.g1_.y) - (other.g1_.y * self_.g1_.x) + (other.g1_.z * self_.g0_.w)), (-(other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))) - (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.y, other.g1_.x) * self_.g0_.yzxx)));
    return flector_geometricProduct_flector(geometric_product, flector_reverse(self_));
}
fn flector_mul_dualNum(self: Flector, other: DualNum) -> Flector {
    return flector_geometricProduct_flector(self_, other);
}
fn line_antiWedge_motor(self: Line, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g0_.x * other.g0_.w), (self_.g0_.y * other.g0_.w), (self_.g0_.z * other.g0_.w), 0.0), /* e23, e31, e12, scalar */ vec4<f32>((self_.g1_.x * other.g0_.w), (self_.g1_.y * other.g0_.w), (self_.g1_.z * other.g0_.w), (-(self_.g0_.x * other.g1_.x) - (self_.g0_.y * other.g1_.y) - (self_.g0_.z * other.g1_.z) - (self_.g1_.x * other.g0_.x) - (self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))));
}
fn antiScalar_sub_line(self: AntiScalar, other: Line) -> Motor {
    let subtraction: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g0_.x * -1.0), (other.g0_.y * -1.0), (other.g0_.z * -1.0), self_.g0_), /* e23, e31, e12, scalar */ vec4<f32>((other.g1_.x * -1.0), (other.g1_.y * -1.0), (other.g1_.z * -1.0), 0.0));
    return subtraction;
}
fn plane_geometricAntiProduct_origin(self: Plane, other: Origin) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g0_.x * other.g0_ * -1.0), (self_.g0_.y * other.g0_ * -1.0), (self_.g0_.z * other.g0_ * -1.0), 0.0), /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.w * other.g0_ * -1.0)));
}
fn antiScalar_geometricProduct_multiVector(self: AntiScalar, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (other.g0_.x * self_.g0_)), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g4_.w * self_.g0_ * -1.0)), /* e41, e42, e43 */ vec3<f32>((other.g3_.x * self_.g0_), (other.g3_.y * self_.g0_), (other.g3_.z * self_.g0_)), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>((other.g1_.x * self_.g0_ * -1.0), (other.g1_.y * self_.g0_ * -1.0), (other.g1_.z * self_.g0_ * -1.0), 0.0));
}
fn multiVector_antiWedge_origin(self: MultiVector, other: Origin) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((self_.g4_.w * other.g0_ * -1.0), 0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.y * other.g0_)), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn line_geometricAntiProduct_antiScalar(self: Line, other: AntiScalar) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_)), /* e23, e31, e12 */ vec3<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_)));
}
fn horizon_antiWedge_motor(self: Horizon, other: Motor) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), 0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.w * self_.g0_)));
}
fn flector_sub_motor(self: Flector, other: Motor) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(other.g1_.w, other.g0_.w) * vec2<f32>(-1.0)), /* e1, e2, e3, e4 */ self_.g0_, /* e41, e42, e43 */ (vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z) * vec3<f32>(-1.0)), /* e23, e31, e12 */ (vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z) * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ self_.g1_);
    return subtraction;
}
fn plane_wedge_flector(self: Plane, other: Flector) -> AntiScalar {
    return AntiScalar(/* e1234 */ (-(other.g0_.x * self_.g0_.x) - (other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z) - (other.g0_.w * self_.g0_.w)));
}
fn dualNum_antiWedge_plane(self: DualNum, other: Plane) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.y * other.g0_.x), (self_.g0_.y * other.g0_.y), (self_.g0_.y * other.g0_.z), (self_.g0_.y * other.g0_.w)));
}
fn multiVector_antiSandwich_point(self: MultiVector, other: Point) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>((-(self_.g4_.x * other.g0_.x) - (self_.g4_.y * other.g0_.y) - (self_.g4_.z * other.g0_.z) - (self_.g4_.w * other.g0_.w)), (self_.g1_.w * other.g0_.w * -1.0)), /* e1, e2, e3, e4 */ vec4<f32>(((self_.g0_.y * other.g0_.x) + (self_.g2_.y * other.g0_.z) - (self_.g2_.z * other.g0_.y) + (self_.g3_.x * other.g0_.w)), ((self_.g0_.y * other.g0_.y) - (self_.g2_.x * other.g0_.z) + (self_.g2_.z * other.g0_.x) + (self_.g3_.y * other.g0_.w)), ((self_.g0_.y * other.g0_.z) + (self_.g2_.x * other.g0_.y) - (self_.g2_.y * other.g0_.x) + (self_.g3_.z * other.g0_.w)), (self_.g0_.y * other.g0_.w)), /* e41, e42, e43 */ (vec3<f32>(other.g0_.w) * vec3<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z) * vec3<f32>(-1.0)), /* e23, e31, e12 */ (vec3<f32>(((self_.g4_.y * other.g0_.z) - (self_.g4_.z * other.g0_.y)), (-(self_.g4_.x * other.g0_.z) + (self_.g4_.z * other.g0_.x)), ((self_.g4_.x * other.g0_.y) - (self_.g4_.y * other.g0_.x))) + (vec3<f32>(self_.g1_.w) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z)) - (vec3<f32>(other.g0_.w) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z))), /* e423, e431, e412, e321 */ vec4<f32>((self_.g2_.x * other.g0_.w), (self_.g2_.y * other.g0_.w), (self_.g2_.z * other.g0_.w), ((self_.g0_.x * other.g0_.w) - (self_.g2_.x * other.g0_.x) - (self_.g2_.y * other.g0_.y) - (self_.g2_.z * other.g0_.z))));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self_));
}
fn line_add_multiVector(self: Line, other: MultiVector) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ other.g0_, /* e1, e2, e3, e4 */ other.g1_, /* e41, e42, e43 */ (self_.g0_ + other.g2_), /* e23, e31, e12 */ (self_.g1_ + other.g3_), /* e423, e431, e412, e321 */ other.g4_);
    return addition;
}
fn multiVector_bitXor_plane(self: MultiVector, other: Plane) -> MultiVector {
    return multiVector_wedge_multiVector(self_, other);
}
fn plane_sub_antiScalar(self: Plane, other: AntiScalar) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (other.g0_ * -1.0)), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ self_.g0_);
    return subtraction;
}
fn multiVector_geometricQuotient_motor(self: MultiVector, other: Motor) -> MultiVector {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2) + pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(inverse.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(inverse.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(inverse.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g4_));
    return geometric_product;
}
fn motor_sub_dualNum(self: Motor, other: DualNum) -> Motor {
    let subtraction: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, (-other.g0_.y + self_.g0_.w)), /* e23, e31, e12, scalar */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, (-other.g0_.x + self_.g1_.w)));
    return subtraction;
}
fn flector_bitXor_line(self: Flector, other: Line) -> Plane {
    return flector_wedge_flector(self_, other);
}
fn multiVector_sandwich_point(self: MultiVector, other: Point) -> MultiVector {
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(((self_.g1_.x * other.g0_.x) + (self_.g1_.y * other.g0_.y) + (self_.g1_.z * other.g0_.z)), (-(self_.g4_.x * other.g0_.x) - (self_.g4_.y * other.g0_.y) - (self_.g4_.z * other.g0_.z) - (self_.g4_.w * other.g0_.w))), /* e1, e2, e3, e4 */ (vec4<f32>(((self_.g3_.y * other.g0_.z) * -1.0), ((self_.g3_.z * other.g0_.x) * -1.0), ((self_.g3_.x * other.g0_.y) * -1.0), ((self_.g2_.y * other.g0_.y) + (self_.g2_.z * other.g0_.z))) + (vec4<f32>(self_.g0_.x) * other.g0_) + (vec4<f32>(self_.g3_.z, self_.g3_.x, self_.g3_.y, self_.g2_.x) * other.g0_.yzxx)), /* e41, e42, e43 */ (vec3<f32>((-(self_.g4_.y * other.g0_.z) + (self_.g4_.z * other.g0_.y)), ((self_.g4_.x * other.g0_.z) - (self_.g4_.z * other.g0_.x)), (-(self_.g4_.x * other.g0_.y) + (self_.g4_.y * other.g0_.x))) + (vec3<f32>(self_.g1_.w) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z)) - (vec3<f32>(other.g0_.w) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z))), /* e23, e31, e12 */ (vec3<f32>(((self_.g1_.y * other.g0_.z) - (self_.g1_.z * other.g0_.y)), (-(self_.g1_.x * other.g0_.z) + (self_.g1_.z * other.g0_.x)), ((self_.g1_.x * other.g0_.y) - (self_.g1_.y * other.g0_.x))) - (vec3<f32>(self_.g4_.w) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z))), /* e423, e431, e412, e321 */ (vec4<f32>(((self_.g2_.y * other.g0_.z) + (self_.g3_.x * other.g0_.w)), ((self_.g2_.z * other.g0_.x) + (self_.g3_.y * other.g0_.w)), ((self_.g2_.x * other.g0_.y) + (self_.g3_.z * other.g0_.w)), ((self_.g3_.z * other.g0_.z) * -1.0)) - (vec4<f32>(self_.g0_.y, self_.g0_.y, self_.g0_.y, self_.g3_.x) * other.g0_.xyzx) - (vec4<f32>(self_.g2_.z, self_.g2_.x, self_.g2_.y, self_.g3_.y) * other.g0_.yzxy)));
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self_));
}
fn dualNum_geometricProduct_motor(self: DualNum, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(((self_.g0_.x * other.g0_.x) + (self_.g0_.y * other.g1_.x)), ((self_.g0_.x * other.g0_.y) + (self_.g0_.y * other.g1_.y)), ((self_.g0_.x * other.g0_.z) + (self_.g0_.y * other.g1_.z)), ((self_.g0_.x * other.g0_.w) + (self_.g0_.y * other.g1_.w))), /* e23, e31, e12, scalar */ vec4<f32>((self_.g0_.x * other.g1_.x), (self_.g0_.x * other.g1_.y), (self_.g0_.x * other.g1_.z), (self_.g0_.x * other.g1_.w)));
}
fn dualNum_antiWedge_origin(self: DualNum, other: Origin) -> Origin {
    return Origin(/* e4 */ (self_.g0_.y * other.g0_));
}
fn antiScalar_antiSandwich_dualNum(self: AntiScalar, other: DualNum) -> DualNum {
    let geometric_anti_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>(self_.g0_) * other.g0_));
    return dualNum_geometricAntiProduct_dualNum(geometric_anti_product, antiScalar_antiReverse(self_));
}
fn flector_geometricAntiProduct_origin(self: Flector, other: Origin) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g1_.x * other.g0_ * -1.0), (self_.g1_.y * other.g0_ * -1.0), (self_.g1_.z * other.g0_ * -1.0), (self_.g0_.w * other.g0_ * -1.0)), /* e23, e31, e12, scalar */ vec4<f32>((self_.g0_.x * other.g0_ * -1.0), (self_.g0_.y * other.g0_ * -1.0), (self_.g0_.z * other.g0_ * -1.0), (self_.g1_.w * other.g0_ * -1.0)));
}
fn plane_geometricAntiProduct_scalar(self: Plane, other: Scalar) -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g0_ * -1.0), (self_.g0_.y * other.g0_ * -1.0), (self_.g0_.z * other.g0_ * -1.0), 0.0));
}
fn multiVector_geometricAntiQuotient_antiScalar(self: MultiVector, other: AntiScalar) -> MultiVector {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(anti_inverse.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(anti_inverse.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(anti_inverse.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g4_));
    return geometric_anti_product;
}
fn dualNum_sub_horizon(self: DualNum, other: Horizon) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_ * -1.0)));
    return subtraction;
}
fn dualNum_geometricAntiQuotient_line(self: DualNum, other: Line) -> DualNum {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>(anti_inverse.g0_) * self_.g0_));
    return geometric_anti_product;
}
fn flector_antiWedge_dualNum(self: Flector, other: DualNum) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.y * self_.g0_.x), (other.g0_.y * self_.g0_.y), (other.g0_.y * self_.g0_.z), (other.g0_.y * self_.g0_.w)), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.y * self_.g1_.x), (other.g0_.y * self_.g1_.y), (other.g0_.y * self_.g1_.z), (other.g0_.y * self_.g1_.w)));
}
fn plane_geometricQuotient_plane(self: Plane, other: Plane) -> Plane {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.w, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Plane = Plane(/* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g0_));
    return geometric_product;
}
fn scalar_geometricProduct_scalar(self: Scalar, other: Scalar) -> Scalar {
    return Scalar(/* scalar */ (other.g0_ * self_.g0_));
}
fn multiVector_geometricAntiProduct_flector(self: MultiVector, other: Flector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((-(other.g0_.x * self_.g4_.x) - (other.g0_.y * self_.g4_.y) - (other.g0_.z * self_.g4_.z) - (other.g0_.w * self_.g4_.w) + (other.g1_.x * self_.g1_.x) + (other.g1_.y * self_.g1_.y) + (other.g1_.z * self_.g1_.z) + (other.g1_.w * self_.g1_.w)), (-(other.g0_.w * self_.g1_.w) + (other.g1_.x * self_.g4_.x) + (other.g1_.y * self_.g4_.y) + (other.g1_.z * self_.g4_.z))), /* e1, e2, e3, e4 */ vec4<f32>(((self_.g0_.x * other.g1_.x) + (self_.g0_.y * other.g0_.x) + (self_.g2_.x * other.g1_.w) + (self_.g2_.y * other.g0_.z) - (self_.g2_.z * other.g0_.y) + (self_.g3_.x * other.g0_.w) + (self_.g3_.y * other.g1_.z) - (self_.g3_.z * other.g1_.y)), ((self_.g0_.x * other.g1_.y) + (self_.g0_.y * other.g0_.y) - (self_.g2_.x * other.g0_.z) + (self_.g2_.y * other.g1_.w) + (self_.g2_.z * other.g0_.x) - (self_.g3_.x * other.g1_.z) + (self_.g3_.y * other.g0_.w) + (self_.g3_.z * other.g1_.x)), ((self_.g0_.x * other.g1_.z) + (self_.g0_.y * other.g0_.z) + (self_.g2_.x * other.g0_.y) - (self_.g2_.y * other.g0_.x) + (self_.g2_.z * other.g1_.w) + (self_.g3_.x * other.g1_.y) - (self_.g3_.y * other.g1_.x) + (self_.g3_.z * other.g0_.w)), ((self_.g0_.y * other.g0_.w) - (self_.g2_.x * other.g1_.x) - (self_.g2_.y * other.g1_.y) - (self_.g2_.z * other.g1_.z))), /* e41, e42, e43 */ vec3<f32>((-(other.g0_.w * self_.g4_.x) - (other.g1_.x * self_.g1_.w) + (other.g1_.y * self_.g4_.z) - (other.g1_.z * self_.g4_.y)), (-(other.g0_.w * self_.g4_.y) - (other.g1_.x * self_.g4_.z) - (other.g1_.y * self_.g1_.w) + (other.g1_.z * self_.g4_.x)), (-(other.g0_.w * self_.g4_.z) + (other.g1_.x * self_.g4_.y) - (other.g1_.y * self_.g4_.x) - (other.g1_.z * self_.g1_.w))), /* e23, e31, e12 */ vec3<f32>(((other.g0_.x * self_.g1_.w) - (other.g0_.y * self_.g4_.z) + (other.g0_.z * self_.g4_.y) - (other.g0_.w * self_.g1_.x) - (other.g1_.x * self_.g4_.w) + (other.g1_.y * self_.g1_.z) - (other.g1_.z * self_.g1_.y) + (other.g1_.w * self_.g4_.x)), ((other.g0_.x * self_.g4_.z) + (other.g0_.y * self_.g1_.w) - (other.g0_.z * self_.g4_.x) - (other.g0_.w * self_.g1_.y) - (other.g1_.x * self_.g1_.z) - (other.g1_.y * self_.g4_.w) + (other.g1_.z * self_.g1_.x) + (other.g1_.w * self_.g4_.y)), (-(other.g0_.x * self_.g4_.y) + (other.g0_.y * self_.g4_.x) + (other.g0_.z * self_.g1_.w) - (other.g0_.w * self_.g1_.z) + (other.g1_.x * self_.g1_.y) - (other.g1_.y * self_.g1_.x) - (other.g1_.z * self_.g4_.w) + (other.g1_.w * self_.g4_.z))), /* e423, e431, e412, e321 */ vec4<f32>(((self_.g0_.y * other.g1_.x) + (self_.g2_.x * other.g0_.w) + (self_.g2_.y * other.g1_.z) - (self_.g2_.z * other.g1_.y)), ((self_.g0_.y * other.g1_.y) - (self_.g2_.x * other.g1_.z) + (self_.g2_.y * other.g0_.w) + (self_.g2_.z * other.g1_.x)), ((self_.g0_.y * other.g1_.z) + (self_.g2_.x * other.g1_.y) - (self_.g2_.y * other.g1_.x) + (self_.g2_.z * other.g0_.w)), ((self_.g0_.x * other.g0_.w) + (self_.g0_.y * other.g1_.w) - (self_.g2_.x * other.g0_.x) - (self_.g2_.y * other.g0_.y) - (self_.g2_.z * other.g0_.z) - (self_.g3_.x * other.g1_.x) - (self_.g3_.y * other.g1_.y) - (self_.g3_.z * other.g1_.z))));
}
fn flector_bitXor_point(self: Flector, other: Point) -> Motor {
    return flector_wedge_flector(self_, other);
}
fn antiScalar_geometricQuotient_dualNum(self: AntiScalar, other: DualNum) -> AntiScalar {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_.x, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: AntiScalar = AntiScalar(/* e1234 */ (self_.g0_ * inverse.g0_));
    return geometric_product;
}
fn multiVector_bitXor_multiVector(self: MultiVector, other: MultiVector) -> MultiVector {
    return multiVector_wedge_multiVector(self_, other);
}
fn scalar_wedge_multiVector(self: Scalar, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_)), /* e1, e2, e3, e4 */ vec4<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_), (other.g1_.w * self_.g0_)), /* e41, e42, e43 */ vec3<f32>((other.g2_.x * self_.g0_), (other.g2_.y * self_.g0_), (other.g2_.z * self_.g0_)), /* e23, e31, e12 */ vec3<f32>((other.g3_.x * self_.g0_), (other.g3_.y * self_.g0_), (other.g3_.z * self_.g0_)), /* e423, e431, e412, e321 */ vec4<f32>((other.g4_.x * self_.g0_), (other.g4_.y * self_.g0_), (other.g4_.z * self_.g0_), (other.g4_.w * self_.g0_)));
}
fn origin_bitXor_motor(self: Origin, other: Motor) -> Flector {
    return origin_wedge_origin(self_, other);
}
fn point_geometricProduct_plane(self: Point, other: Plane) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((-(other.g0_.y * self_.g0_.z) + (other.g0_.z * self_.g0_.y)), ((other.g0_.x * self_.g0_.z) - (other.g0_.z * self_.g0_.x)), (-(other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x)), ((other.g0_.x * self_.g0_.x) + (other.g0_.y * self_.g0_.y) + (other.g0_.z * self_.g0_.z) + (other.g0_.w * self_.g0_.w))), /* e23, e31, e12, scalar */ vec4<f32>((other.g0_.w * self_.g0_.x * -1.0), (other.g0_.w * self_.g0_.y * -1.0), (other.g0_.w * self_.g0_.z * -1.0), 0.0));
}
fn point_geometricAntiQuotient_antiScalar(self: Point, other: AntiScalar) -> Point {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Point = Point(/* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_));
    return geometric_anti_product;
}
fn origin_geometricAntiProduct_plane(self: Origin, other: Plane) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g0_.x * self_.g0_ * -1.0), (other.g0_.y * self_.g0_ * -1.0), (other.g0_.z * self_.g0_ * -1.0), 0.0), /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.w * self_.g0_)));
}
fn line_bitXor_flector(self: Line, other: Flector) -> Plane {
    return line_wedge_line(self_, other);
}
fn horizon_sub_antiScalar(self: Horizon, other: AntiScalar) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (other.g0_ * -1.0)), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_));
    return subtraction;
}
fn horizon_geometricProduct_dualNum(self: Horizon, other: DualNum) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.y * self_.g0_)), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.x * self_.g0_)));
}
fn multiVector_geometricProduct_multiVector(self: MultiVector, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(((other.g0_.x * self_.g0_.x) - (other.g3_.x * self_.g3_.x) - (other.g3_.y * self_.g3_.y) - (other.g3_.z * self_.g3_.z) + (other.g1_.x * self_.g1_.x) + (other.g1_.y * self_.g1_.y) + (other.g1_.z * self_.g1_.z) - (other.g4_.w * self_.g4_.w)), ((other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x) - (other.g2_.x * self_.g3_.x) - (other.g2_.y * self_.g3_.y) - (other.g2_.z * self_.g3_.z) - (other.g3_.x * self_.g2_.x) - (other.g3_.y * self_.g2_.y) - (other.g3_.z * self_.g2_.z) - (other.g1_.x * self_.g4_.x) - (other.g1_.y * self_.g4_.y) - (other.g1_.z * self_.g4_.z) - (other.g1_.w * self_.g4_.w) + (other.g4_.x * self_.g1_.x) + (other.g4_.y * self_.g1_.y) + (other.g4_.z * self_.g1_.z) + (other.g4_.w * self_.g1_.w))), /* e1, e2, e3, e4 */ vec4<f32>(((other.g0_.x * self_.g1_.x) + (self_.g0_.x * other.g1_.x) + (other.g3_.x * self_.g4_.w) + (other.g3_.y * self_.g1_.z) - (other.g3_.z * self_.g1_.y) + (self_.g3_.x * other.g4_.w) - (self_.g3_.y * other.g1_.z) + (self_.g3_.z * other.g1_.y)), ((other.g0_.x * self_.g1_.y) + (self_.g0_.x * other.g1_.y) - (other.g3_.x * self_.g1_.z) + (other.g3_.y * self_.g4_.w) + (other.g3_.z * self_.g1_.x) + (self_.g3_.x * other.g1_.z) + (self_.g3_.y * other.g4_.w) - (self_.g3_.z * other.g1_.x)), ((other.g0_.x * self_.g1_.z) + (self_.g0_.x * other.g1_.z) + (other.g3_.x * self_.g1_.y) - (other.g3_.y * self_.g1_.x) + (other.g3_.z * self_.g4_.w) - (self_.g3_.x * other.g1_.y) + (self_.g3_.y * other.g1_.x) + (self_.g3_.z * other.g4_.w)), ((other.g0_.x * self_.g1_.w) + (other.g0_.y * self_.g4_.w) + (self_.g0_.x * other.g1_.w) - (self_.g0_.y * other.g4_.w) - (other.g2_.x * self_.g1_.x) - (other.g2_.y * self_.g1_.y) - (other.g2_.z * self_.g1_.z) - (other.g3_.x * self_.g4_.x) - (other.g3_.y * self_.g4_.y) - (other.g3_.z * self_.g4_.z) + (self_.g2_.x * other.g1_.x) + (self_.g2_.y * other.g1_.y) + (self_.g2_.z * other.g1_.z) - (self_.g3_.x * other.g4_.x) - (self_.g3_.y * other.g4_.y) - (self_.g3_.z * other.g4_.z))), /* e41, e42, e43 */ vec3<f32>(((other.g0_.x * self_.g2_.x) + (other.g0_.y * self_.g3_.x) + (self_.g0_.x * other.g2_.x) + (self_.g0_.y * other.g3_.x) + (other.g2_.y * self_.g3_.z) - (other.g2_.z * self_.g3_.y) + (other.g3_.y * self_.g2_.z) - (other.g3_.z * self_.g2_.y) + (other.g1_.x * self_.g1_.w) + (other.g1_.y * self_.g4_.z) - (other.g1_.z * self_.g4_.y) - (other.g1_.w * self_.g1_.x) - (other.g4_.x * self_.g4_.w) - (other.g4_.y * self_.g1_.z) + (other.g4_.z * self_.g1_.y) + (other.g4_.w * self_.g4_.x)), ((other.g0_.x * self_.g2_.y) + (other.g0_.y * self_.g3_.y) + (self_.g0_.x * other.g2_.y) + (self_.g0_.y * other.g3_.y) - (other.g2_.x * self_.g3_.z) + (other.g2_.z * self_.g3_.x) - (other.g3_.x * self_.g2_.z) + (other.g3_.z * self_.g2_.x) - (other.g1_.x * self_.g4_.z) + (other.g1_.y * self_.g1_.w) + (other.g1_.z * self_.g4_.x) - (other.g1_.w * self_.g1_.y) + (other.g4_.x * self_.g1_.z) - (other.g4_.y * self_.g4_.w) - (other.g4_.z * self_.g1_.x) + (other.g4_.w * self_.g4_.y)), ((other.g0_.x * self_.g2_.z) + (other.g0_.y * self_.g3_.z) + (self_.g0_.x * other.g2_.z) + (self_.g0_.y * other.g3_.z) + (other.g2_.x * self_.g3_.y) - (other.g2_.y * self_.g3_.x) + (other.g3_.x * self_.g2_.y) - (other.g3_.y * self_.g2_.x) + (other.g1_.x * self_.g4_.y) - (other.g1_.y * self_.g4_.x) + (other.g1_.z * self_.g1_.w) - (other.g1_.w * self_.g1_.z) - (other.g4_.x * self_.g1_.y) + (other.g4_.y * self_.g1_.x) - (other.g4_.z * self_.g4_.w) + (other.g4_.w * self_.g4_.z))), /* e23, e31, e12 */ vec3<f32>(((other.g0_.x * self_.g3_.x) + (self_.g0_.x * other.g3_.x) + (other.g3_.y * self_.g3_.z) - (other.g3_.z * self_.g3_.y) - (other.g1_.x * self_.g4_.w) - (other.g1_.y * self_.g1_.z) + (other.g1_.z * self_.g1_.y) - (other.g4_.w * self_.g1_.x)), ((other.g0_.x * self_.g3_.y) + (self_.g0_.x * other.g3_.y) - (other.g3_.x * self_.g3_.z) + (other.g3_.z * self_.g3_.x) + (other.g1_.x * self_.g1_.z) - (other.g1_.y * self_.g4_.w) - (other.g1_.z * self_.g1_.x) - (other.g4_.w * self_.g1_.y)), ((other.g0_.x * self_.g3_.z) + (self_.g0_.x * other.g3_.z) + (other.g3_.x * self_.g3_.y) - (other.g3_.y * self_.g3_.x) - (other.g1_.x * self_.g1_.y) + (other.g1_.y * self_.g1_.x) - (other.g1_.z * self_.g4_.w) - (other.g4_.w * self_.g1_.z))), /* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.x * self_.g4_.x) + (other.g0_.y * self_.g1_.x) + (self_.g0_.x * other.g4_.x) - (self_.g0_.y * other.g1_.x) + (other.g2_.x * self_.g4_.w) + (other.g2_.y * self_.g1_.z) - (other.g2_.z * self_.g1_.y) + (other.g3_.x * self_.g1_.w) + (other.g3_.y * self_.g4_.z) - (other.g3_.z * self_.g4_.y) - (self_.g2_.x * other.g4_.w) + (self_.g2_.y * other.g1_.z) - (self_.g2_.z * other.g1_.y) + (self_.g3_.x * other.g1_.w) - (self_.g3_.y * other.g4_.z) + (self_.g3_.z * other.g4_.y)), ((other.g0_.x * self_.g4_.y) + (other.g0_.y * self_.g1_.y) + (self_.g0_.x * other.g4_.y) - (self_.g0_.y * other.g1_.y) - (other.g2_.x * self_.g1_.z) + (other.g2_.y * self_.g4_.w) + (other.g2_.z * self_.g1_.x) - (other.g3_.x * self_.g4_.z) + (other.g3_.y * self_.g1_.w) + (other.g3_.z * self_.g4_.x) - (self_.g2_.x * other.g1_.z) - (self_.g2_.y * other.g4_.w) + (self_.g2_.z * other.g1_.x) + (self_.g3_.x * other.g4_.z) + (self_.g3_.y * other.g1_.w) - (self_.g3_.z * other.g4_.x)), ((other.g0_.x * self_.g4_.z) + (other.g0_.y * self_.g1_.z) + (self_.g0_.x * other.g4_.z) - (self_.g0_.y * other.g1_.z) + (other.g2_.x * self_.g1_.y) - (other.g2_.y * self_.g1_.x) + (other.g2_.z * self_.g4_.w) + (other.g3_.x * self_.g4_.y) - (other.g3_.y * self_.g4_.x) + (other.g3_.z * self_.g1_.w) + (self_.g2_.x * other.g1_.y) - (self_.g2_.y * other.g1_.x) - (self_.g2_.z * other.g4_.w) - (self_.g3_.x * other.g4_.y) + (self_.g3_.y * other.g4_.x) + (self_.g3_.z * other.g1_.w)), ((other.g0_.x * self_.g4_.w) + (self_.g0_.x * other.g4_.w) - (other.g3_.x * self_.g1_.x) - (other.g3_.y * self_.g1_.y) - (other.g3_.z * self_.g1_.z) - (self_.g3_.x * other.g1_.x) - (self_.g3_.y * other.g1_.y) - (self_.g3_.z * other.g1_.z))));
}
fn plane_sub_scalar(self: Plane, other: Scalar) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>((other.g0_ * -1.0), 0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ self_.g0_);
    return subtraction;
}
fn antiScalar_wedge_motor(self: AntiScalar, other: Motor) -> AntiScalar {
    return AntiScalar(/* e1234 */ (other.g1_.w * self_.g0_));
}
fn flector_wedge_dualNum(self: Flector, other: DualNum) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g0_.x), (other.g0_.x * self_.g0_.y), (other.g0_.x * self_.g0_.z), (other.g0_.x * self_.g0_.w)), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.x * self_.g1_.x), (other.g0_.x * self_.g1_.y), (other.g0_.x * self_.g1_.z), (other.g0_.x * self_.g1_.w)));
}
fn flector_geometricProduct_dualNum(self: Flector, other: DualNum) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g0_.x), (other.g0_.x * self_.g0_.y), (other.g0_.x * self_.g0_.z), ((other.g0_.x * self_.g0_.w) + (other.g0_.y * self_.g1_.w))), /* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.x * self_.g1_.x) + (other.g0_.y * self_.g0_.x)), ((other.g0_.x * self_.g1_.y) + (other.g0_.y * self_.g0_.y)), ((other.g0_.x * self_.g1_.z) + (other.g0_.y * self_.g0_.z)), (other.g0_.x * self_.g1_.w)));
}
fn point_antiSandwich_origin(self: Point, other: Origin) -> Flector {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.w * other.g0_ * -1.0)), /* e23, e31, e12, scalar */ vec4<f32>((self_.g0_.x * other.g0_ * -1.0), (self_.g0_.y * other.g0_ * -1.0), (self_.g0_.z * other.g0_ * -1.0), 0.0));
    return motor_geometricAntiProduct_motor(geometric_anti_product, point_antiReverse(self_));
}
fn horizon_geometricAntiProduct_antiScalar(self: Horizon, other: AntiScalar) -> Horizon {
    return Horizon(/* e321 */ (other.g0_ * self_.g0_));
}
fn origin_antiSandwich_point(self: Origin, other: Point) -> Flector {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.w * self_.g0_ * -1.0)), /* e23, e31, e12, scalar */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), 0.0));
    return motor_geometricAntiProduct_motor(geometric_anti_product, origin_antiReverse(self_));
}
fn point_add_origin(self: Point, other: Origin) -> Point {
    let addition: Point = Point(/* e1, e2, e3, e4 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, (self_.g0_.w + other.g0_)));
    return addition;
}
fn scalar_sub_point(self: Scalar, other: Point) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g0_, 0.0), /* e1, e2, e3, e4 */ (other.g0_ * vec4<f32>(-1.0)), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    return subtraction;
}
fn dualNum_add_motor(self: DualNum, other: Motor) -> Motor {
    let addition: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, (self_.g0_.y + other.g0_.w)), /* e23, e31, e12, scalar */ vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, (self_.g0_.x + other.g1_.w)));
    return addition;
}
fn scalar_bitXor_scalar(self: Scalar, other: Scalar) -> Scalar {
    return scalar_wedge_scalar(self_, other);
}
fn scalar_add_plane(self: Scalar, other: Plane) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g0_, 0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ other.g0_);
    return addition;
}
fn plane_geometricAntiProduct_multiVector(self: Plane, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((-(other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z) - (other.g1_.w * self_.g0_.w)), ((other.g4_.x * self_.g0_.x) + (other.g4_.y * self_.g0_.y) + (other.g4_.z * self_.g0_.z))), /* e1, e2, e3, e4 */ vec4<f32>((-(other.g0_.x * self_.g0_.x) + (other.g2_.x * self_.g0_.w) + (other.g3_.y * self_.g0_.z) - (other.g3_.z * self_.g0_.y)), (-(other.g0_.x * self_.g0_.y) + (other.g2_.y * self_.g0_.w) - (other.g3_.x * self_.g0_.z) + (other.g3_.z * self_.g0_.x)), (-(other.g0_.x * self_.g0_.z) + (other.g2_.z * self_.g0_.w) + (other.g3_.x * self_.g0_.y) - (other.g3_.y * self_.g0_.x)), (-(other.g2_.x * self_.g0_.x) - (other.g2_.y * self_.g0_.y) - (other.g2_.z * self_.g0_.z))), /* e41, e42, e43 */ vec3<f32>((-(other.g1_.w * self_.g0_.x) + (other.g4_.y * self_.g0_.z) - (other.g4_.z * self_.g0_.y)), (-(other.g1_.w * self_.g0_.y) - (other.g4_.x * self_.g0_.z) + (other.g4_.z * self_.g0_.x)), (-(other.g1_.w * self_.g0_.z) + (other.g4_.x * self_.g0_.y) - (other.g4_.y * self_.g0_.x))), /* e23, e31, e12 */ vec3<f32>((-(other.g1_.y * self_.g0_.z) + (other.g1_.z * self_.g0_.y) - (other.g4_.x * self_.g0_.w) + (other.g4_.w * self_.g0_.x)), ((other.g1_.x * self_.g0_.z) - (other.g1_.z * self_.g0_.x) - (other.g4_.y * self_.g0_.w) + (other.g4_.w * self_.g0_.y)), (-(other.g1_.x * self_.g0_.y) + (other.g1_.y * self_.g0_.x) - (other.g4_.z * self_.g0_.w) + (other.g4_.w * self_.g0_.z))), /* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.y * self_.g0_.x) - (other.g2_.y * self_.g0_.z) + (other.g2_.z * self_.g0_.y)), ((other.g0_.y * self_.g0_.y) + (other.g2_.x * self_.g0_.z) - (other.g2_.z * self_.g0_.x)), ((other.g0_.y * self_.g0_.z) - (other.g2_.x * self_.g0_.y) + (other.g2_.y * self_.g0_.x)), ((other.g0_.y * self_.g0_.w) + (other.g3_.x * self_.g0_.x) + (other.g3_.y * self_.g0_.y) + (other.g3_.z * self_.g0_.z))));
}
fn origin_mul_scalar(self: Origin, other: Scalar) -> Origin {
    return origin_geometricProduct_origin(self_, other);
}
fn plane_geometricQuotient_motor(self: Plane, other: Motor) -> Plane {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2) + pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Plane = Plane(/* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g0_));
    return geometric_product;
}
fn dualNum_antiWedge_point(self: DualNum, other: Point) -> Point {
    return Point(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.y * other.g0_.x), (self_.g0_.y * other.g0_.y), (self_.g0_.y * other.g0_.z), (self_.g0_.y * other.g0_.w)));
}
fn scalar_mul_horizon(self: Scalar, other: Horizon) -> Horizon {
    return scalar_geometricProduct_scalar(self_, other);
}
fn line_geometricProduct_origin(self: Line, other: Origin) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), 0.0));
}
fn motor_antiWedge_flector(self: Motor, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(((other.g0_.x * self_.g0_.w) - (other.g1_.y * self_.g1_.z) + (other.g1_.z * self_.g1_.y) + (other.g1_.w * self_.g0_.x)), ((other.g0_.y * self_.g0_.w) + (other.g1_.x * self_.g1_.z) - (other.g1_.z * self_.g1_.x) + (other.g1_.w * self_.g0_.y)), ((other.g0_.z * self_.g0_.w) - (other.g1_.x * self_.g1_.y) + (other.g1_.y * self_.g1_.x) + (other.g1_.w * self_.g0_.z)), ((other.g0_.w * self_.g0_.w) - (other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))), /* e423, e431, e412, e321 */ vec4<f32>((other.g1_.x * self_.g0_.w), (other.g1_.y * self_.g0_.w), (other.g1_.z * self_.g0_.w), (other.g1_.w * self_.g0_.w)));
}
fn motor_geometricAntiProduct_dualNum(self: Motor, other: DualNum) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g0_.y * self_.g0_.x), (other.g0_.y * self_.g0_.y), (other.g0_.y * self_.g0_.z), (other.g0_.y * self_.g0_.w)), /* e23, e31, e12, scalar */ vec4<f32>(((other.g0_.x * self_.g0_.x) + (other.g0_.y * self_.g1_.x)), ((other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g1_.y)), ((other.g0_.x * self_.g0_.z) + (other.g0_.y * self_.g1_.z)), ((other.g0_.x * self_.g0_.w) + (other.g0_.y * self_.g1_.w))));
}
fn plane_antiWedge_flector(self: Plane, other: Flector) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(((other.g1_.y * self_.g0_.z) - (other.g1_.z * self_.g0_.y)), (-(other.g1_.x * self_.g0_.z) + (other.g1_.z * self_.g0_.x)), ((other.g1_.x * self_.g0_.y) - (other.g1_.y * self_.g0_.x)), 0.0), /* e23, e31, e12, scalar */ vec4<f32>((-(other.g1_.x * self_.g0_.w) + (other.g1_.w * self_.g0_.x)), (-(other.g1_.y * self_.g0_.w) + (other.g1_.w * self_.g0_.y)), (-(other.g1_.z * self_.g0_.w) + (other.g1_.w * self_.g0_.z)), (-(other.g0_.x * self_.g0_.x) - (other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z) - (other.g0_.w * self_.g0_.w))));
}
fn horizon_geometricQuotient_line(self: Horizon, other: Line) -> Horizon {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Horizon = Horizon(/* e321 */ (self_.g0_ * inverse.g0_));
    return geometric_product;
}
fn flector_add_plane(self: Flector, other: Plane) -> Flector {
    let addition: Flector = Flector(/* e1, e2, e3, e4 */ self_.g0_, /* e423, e431, e412, e321 */ (self_.g1_ + other.g0_));
    return addition;
}
fn multiVector_geometricQuotient_flector(self: MultiVector, other: Flector) -> MultiVector {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2) - pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(inverse.g0_) * self_.g0_), /* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g1_), /* e41, e42, e43 */ (vec3<f32>(inverse.g0_) * self_.g2_), /* e23, e31, e12 */ (vec3<f32>(inverse.g0_) * self_.g3_), /* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g4_));
    return geometric_product;
}
fn flector_geometricAntiQuotient_point(self: Flector, other: Point) -> Flector {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.w, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g1_));
    return geometric_anti_product;
}
fn plane_geometricQuotient_point(self: Plane, other: Point) -> Plane {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Plane = Plane(/* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g0_));
    return geometric_product;
}
fn plane_geometricAntiProduct_line(self: Plane, other: Line) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(((other.g0_.x * self_.g0_.w) + (other.g1_.y * self_.g0_.z) - (other.g1_.z * self_.g0_.y)), ((other.g0_.y * self_.g0_.w) - (other.g1_.x * self_.g0_.z) + (other.g1_.z * self_.g0_.x)), ((other.g0_.z * self_.g0_.w) + (other.g1_.x * self_.g0_.y) - (other.g1_.y * self_.g0_.x)), (-(other.g0_.x * self_.g0_.x) - (other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z))), /* e423, e431, e412, e321 */ vec4<f32>((-(other.g0_.y * self_.g0_.z) + (other.g0_.z * self_.g0_.y)), ((other.g0_.x * self_.g0_.z) - (other.g0_.z * self_.g0_.x)), (-(other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x)), ((other.g1_.x * self_.g0_.x) + (other.g1_.y * self_.g0_.y) + (other.g1_.z * self_.g0_.z))));
}
fn multiVector_add_scalar(self: MultiVector, other: Scalar) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>((self_.g0_.x + other.g0_), self_.g0_.y), /* e1, e2, e3, e4 */ self_.g1_, /* e41, e42, e43 */ self_.g2_, /* e23, e31, e12 */ self_.g3_, /* e423, e431, e412, e321 */ self_.g4_);
    return addition;
}
fn flector_antiWedge_point(self: Flector, other: Point) -> Scalar {
    return Scalar(/* scalar */ (-(self_.g1_.x * other.g0_.x) - (self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z) - (self_.g1_.w * other.g0_.w)));
}
fn motor_geometricAntiQuotient_multiVector(self: Motor, other: MultiVector) -> Motor {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.y, 2) - pow(other.g2_.x, 2) - pow(other.g2_.y, 2) - pow(other.g2_.z, 2) - pow(other.g1_.w, 2) + pow(other.g4_.x, 2) + pow(other.g4_.y, 2) + pow(other.g4_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(anti_inverse.g0_) * self_.g1_));
    return geometric_anti_product;
}
fn motor_antiWedge_dualNum(self: Motor, other: DualNum) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g0_.y * self_.g0_.x), (other.g0_.y * self_.g0_.y), (other.g0_.y * self_.g0_.z), (other.g0_.y * self_.g0_.w)), /* e23, e31, e12, scalar */ vec4<f32>((other.g0_.y * self_.g1_.x), (other.g0_.y * self_.g1_.y), (other.g0_.y * self_.g1_.z), ((other.g0_.x * self_.g0_.w) + (other.g0_.y * self_.g1_.w))));
}
fn scalar_mul_dualNum(self: Scalar, other: DualNum) -> DualNum {
    return scalar_geometricProduct_scalar(self_, other);
}
fn point_antiSandwich_antiScalar(self: Point, other: AntiScalar) -> Motor {
    let geometric_anti_product: Point = Point(/* e1, e2, e3, e4 */ (vec4<f32>(other.g0_) * self_.g0_));
    return point_geometricAntiProduct_point(geometric_anti_product, point_antiReverse(self_));
}
fn scalar_sub_dualNum(self: Scalar, other: DualNum) -> DualNum {
    let subtraction: DualNum = DualNum(/* scalar, e1234 */ vec2<f32>((-other.g0_.x + self_.g0_), (other.g0_.y * -1.0)));
    return subtraction;
}
fn line_antiSandwich_plane(self: Line, other: Plane) -> Flector {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(((self_.g0_.x * other.g0_.w) + (self_.g1_.y * other.g0_.z)), ((self_.g0_.y * other.g0_.w) + (self_.g1_.z * other.g0_.x)), ((self_.g0_.z * other.g0_.w) + (self_.g1_.x * other.g0_.y)), (-(self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z))) - (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g0_.x) * other.g0_.yzxx)), /* e423, e431, e412, e321 */ (vec4<f32>((self_.g0_.y * other.g0_.z), (self_.g0_.z * other.g0_.x), (self_.g0_.x * other.g0_.y), (-(self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g1_.x) * other.g0_.yzxx)));
    return flector_geometricAntiProduct_flector(geometric_anti_product, line_antiReverse(self_));
}
fn line_wedge_scalar(self: Line, other: Scalar) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_)), /* e23, e31, e12 */ vec3<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_)));
}
fn point_geometricAntiProduct_flector(self: Point, other: Flector) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g1_.x * self_.g0_.w * -1.0), (other.g1_.y * self_.g0_.w * -1.0), (other.g1_.z * self_.g0_.w * -1.0), (other.g0_.w * self_.g0_.w * -1.0)), /* e23, e31, e12, scalar */ vec4<f32>(((other.g0_.x * self_.g0_.w) - (other.g0_.w * self_.g0_.x) + (other.g1_.y * self_.g0_.z) - (other.g1_.z * self_.g0_.y)), ((other.g0_.y * self_.g0_.w) - (other.g0_.w * self_.g0_.y) - (other.g1_.x * self_.g0_.z) + (other.g1_.z * self_.g0_.x)), ((other.g0_.z * self_.g0_.w) - (other.g0_.w * self_.g0_.z) + (other.g1_.x * self_.g0_.y) - (other.g1_.y * self_.g0_.x)), ((other.g1_.x * self_.g0_.x) + (other.g1_.y * self_.g0_.y) + (other.g1_.z * self_.g0_.z) + (other.g1_.w * self_.g0_.w))));
}
fn plane_antiWedge_dualNum(self: Plane, other: DualNum) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>((other.g0_.y * self_.g0_.x), (other.g0_.y * self_.g0_.y), (other.g0_.y * self_.g0_.z), (other.g0_.y * self_.g0_.w)));
}
fn origin_geometricProduct_multiVector(self: Origin, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (other.g4_.w * self_.g0_)), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.x * self_.g0_)), /* e41, e42, e43 */ vec3<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_)), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>((other.g3_.x * self_.g0_), (other.g3_.y * self_.g0_), (other.g3_.z * self_.g0_), 0.0));
}
fn point_antiSandwich_line(self: Point, other: Line) -> Motor {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>((-(other.g0_.y * self_.g0_.z) + (other.g0_.z * self_.g0_.y) - (other.g1_.x * self_.g0_.w)), ((other.g0_.x * self_.g0_.z) - (other.g0_.z * self_.g0_.x) - (other.g1_.y * self_.g0_.w)), (-(other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x) - (other.g1_.z * self_.g0_.w)), 0.0), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.x * self_.g0_.w), (other.g0_.y * self_.g0_.w), (other.g0_.z * self_.g0_.w), (-(other.g0_.x * self_.g0_.x) - (other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z))));
    return flector_geometricAntiProduct_flector(geometric_anti_product, point_antiReverse(self_));
}
fn point_sandwich_line(self: Point, other: Line) -> Motor {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>((other.g1_.y * self_.g0_.z), (other.g1_.z * self_.g0_.x), (other.g1_.x * self_.g0_.y), (-(other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z))) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.x) * self_.g0_.yzxx)), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g0_.y * self_.g0_.z) + (other.g1_.x * self_.g0_.w)), ((other.g0_.z * self_.g0_.x) + (other.g1_.y * self_.g0_.w)), ((other.g0_.x * self_.g0_.y) + (other.g1_.z * self_.g0_.w)), (-(other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))) - (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.y, other.g1_.x) * self_.g0_.yzxx)));
    return flector_geometricProduct_flector(geometric_product, point_reverse(self_));
}
fn multiVector_antiWedge_flector(self: MultiVector, other: Flector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((-(other.g0_.x * self_.g4_.x) - (other.g0_.y * self_.g4_.y) - (other.g0_.z * self_.g4_.z) - (other.g0_.w * self_.g4_.w) + (other.g1_.x * self_.g1_.x) + (other.g1_.y * self_.g1_.y) + (other.g1_.z * self_.g1_.z) + (other.g1_.w * self_.g1_.w)), 0.0), /* e1, e2, e3, e4 */ vec4<f32>(((self_.g0_.y * other.g0_.x) + (self_.g2_.x * other.g1_.w) + (self_.g3_.y * other.g1_.z) - (self_.g3_.z * other.g1_.y)), ((self_.g0_.y * other.g0_.y) + (self_.g2_.y * other.g1_.w) - (self_.g3_.x * other.g1_.z) + (self_.g3_.z * other.g1_.x)), ((self_.g0_.y * other.g0_.z) + (self_.g2_.z * other.g1_.w) + (self_.g3_.x * other.g1_.y) - (self_.g3_.y * other.g1_.x)), ((self_.g0_.y * other.g0_.w) - (self_.g2_.x * other.g1_.x) - (self_.g2_.y * other.g1_.y) - (self_.g2_.z * other.g1_.z))), /* e41, e42, e43 */ vec3<f32>(((other.g1_.y * self_.g4_.z) - (other.g1_.z * self_.g4_.y)), (-(other.g1_.x * self_.g4_.z) + (other.g1_.z * self_.g4_.x)), ((other.g1_.x * self_.g4_.y) - (other.g1_.y * self_.g4_.x))), /* e23, e31, e12 */ vec3<f32>((-(other.g1_.x * self_.g4_.w) + (other.g1_.w * self_.g4_.x)), (-(other.g1_.y * self_.g4_.w) + (other.g1_.w * self_.g4_.y)), (-(other.g1_.z * self_.g4_.w) + (other.g1_.w * self_.g4_.z))), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.y * other.g1_.x), (self_.g0_.y * other.g1_.y), (self_.g0_.y * other.g1_.z), (self_.g0_.y * other.g1_.w)));
}
fn origin_sub_motor(self: Origin, other: Motor) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(other.g1_.w, other.g0_.w) * vec2<f32>(-1.0)), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_), /* e41, e42, e43 */ (vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z) * vec3<f32>(-1.0)), /* e23, e31, e12 */ (vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z) * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    return subtraction;
}
fn flector_geometricQuotient_scalar(self: Flector, other: Scalar) -> Flector {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(inverse.g0_) * self_.g1_));
    return geometric_product;
}
fn line_add_point(self: Line, other: Point) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ other.g0_, /* e41, e42, e43 */ self_.g0_, /* e23, e31, e12 */ self_.g1_, /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    return addition;
}
fn horizon_wedge_motor(self: Horizon, other: Motor) -> Horizon {
    return Horizon(/* e321 */ (other.g1_.w * self_.g0_));
}
fn point_geometricProduct_motor(self: Point, other: Motor) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(((other.g1_.y * self_.g0_.z) - (other.g1_.z * self_.g0_.y) + (other.g1_.w * self_.g0_.x)), (-(other.g1_.x * self_.g0_.z) + (other.g1_.z * self_.g0_.x) + (other.g1_.w * self_.g0_.y)), ((other.g1_.x * self_.g0_.y) - (other.g1_.y * self_.g0_.x) + (other.g1_.w * self_.g0_.z)), (-(other.g0_.x * self_.g0_.x) - (other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z) + (other.g1_.w * self_.g0_.w))), /* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.y * self_.g0_.z) - (other.g0_.z * self_.g0_.y) + (other.g0_.w * self_.g0_.x) + (other.g1_.x * self_.g0_.w)), (-(other.g0_.x * self_.g0_.z) + (other.g0_.z * self_.g0_.x) + (other.g0_.w * self_.g0_.y) + (other.g1_.y * self_.g0_.w)), ((other.g0_.x * self_.g0_.y) - (other.g0_.y * self_.g0_.x) + (other.g0_.w * self_.g0_.z) + (other.g1_.z * self_.g0_.w)), (-(other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z))));
}
fn line_sub_motor(self: Line, other: Motor) -> Motor {
    let subtraction: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g0_.x - other.g0_.x), (self_.g0_.y - other.g0_.y), (self_.g0_.z - other.g0_.z), (other.g0_.w * -1.0)), /* e23, e31, e12, scalar */ vec4<f32>((self_.g1_.x - other.g1_.x), (self_.g1_.y - other.g1_.y), (self_.g1_.z - other.g1_.z), (other.g1_.w * -1.0)));
    return subtraction;
}
fn plane_geometricAntiQuotient_origin(self: Plane, other: Origin) -> Plane {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Plane = Plane(/* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_));
    return geometric_anti_product;
}
fn horizon_geometricQuotient_point(self: Horizon, other: Point) -> Horizon {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Horizon = Horizon(/* e321 */ (self_.g0_ * inverse.g0_));
    return geometric_product;
}
fn origin_geometricAntiQuotient_dualNum(self: Origin, other: DualNum) -> Origin {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_.y, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Origin = Origin(/* e4 */ (anti_inverse.g0_ * self_.g0_));
    return geometric_anti_product;
}
fn line_sub_multiVector(self: Line, other: MultiVector) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ (other.g0_ * vec2<f32>(-1.0)), /* e1, e2, e3, e4 */ (other.g1_ * vec4<f32>(-1.0)), /* e41, e42, e43 */ (self_.g0_ - other.g2_), /* e23, e31, e12 */ (self_.g1_ - other.g3_), /* e423, e431, e412, e321 */ (other.g4_ * vec4<f32>(-1.0)));
    return subtraction;
}
fn origin_sandwich_point(self: Origin, other: Point) -> Plane {
    let geometric_product: Line = Line(/* e41, e42, e43 */ (vec3<f32>(self_.g0_) * vec3<f32>(other.g0_.x, other.g0_.y, other.g0_.z)), /* e23, e31, e12 */ vec3<f32>(0.0));
    return line_geometricProduct_line(geometric_product, origin_reverse(self_));
}
fn origin_geometricProduct_line(self: Origin, other: Line) -> Plane {
    return Plane(/* e423, e431, e412, e321 */ vec4<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_), 0.0));
}
fn antiScalar_geometricProduct_flector(self: AntiScalar, other: Flector) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g1_.w * self_.g0_ * -1.0)), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.x * self_.g0_ * -1.0), (other.g0_.y * self_.g0_ * -1.0), (other.g0_.z * self_.g0_ * -1.0), 0.0));
}
fn horizon_sandwich_plane(self: Horizon, other: Plane) -> Flector {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g0_.x * self_.g0_ * -1.0), (other.g0_.y * self_.g0_ * -1.0), (other.g0_.z * self_.g0_ * -1.0), 0.0), /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.w * self_.g0_ * -1.0)));
    return motor_geometricProduct_motor(geometric_product, horizon_reverse(self_));
}
fn antiScalar_antiSandwich_origin(self: AntiScalar, other: Origin) -> Origin {
    let geometric_anti_product: Origin = Origin(/* e4 */ (self_.g0_ * other.g0_));
    return origin_geometricAntiProduct_origin(geometric_anti_product, antiScalar_antiReverse(self_));
}
fn point_antiWedge_plane(self: Point, other: Plane) -> Scalar {
    return Scalar(/* scalar */ ((other.g0_.x * self_.g0_.x) + (other.g0_.y * self_.g0_.y) + (other.g0_.z * self_.g0_.z) + (other.g0_.w * self_.g0_.w)));
}
fn line_sub_scalar(self: Line, other: Scalar) -> Motor {
    let subtraction: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, 0.0), /* e23, e31, e12, scalar */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, (other.g0_ * -1.0)));
    return subtraction;
}
fn origin_bitXor_flector(self: Origin, other: Flector) -> Motor {
    return origin_wedge_origin(self_, other);
}
fn motor_geometricAntiProduct_point(self: Motor, other: Point) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(((self_.g0_.y * other.g0_.z) - (self_.g0_.z * other.g0_.y) + (self_.g0_.w * other.g0_.x) + (self_.g1_.x * other.g0_.w)), (-(self_.g0_.x * other.g0_.z) + (self_.g0_.z * other.g0_.x) + (self_.g0_.w * other.g0_.y) + (self_.g1_.y * other.g0_.w)), ((self_.g0_.x * other.g0_.y) - (self_.g0_.y * other.g0_.x) + (self_.g0_.w * other.g0_.z) + (self_.g1_.z * other.g0_.w)), (self_.g0_.w * other.g0_.w)), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.x * other.g0_.w), (self_.g0_.y * other.g0_.w), (self_.g0_.z * other.g0_.w), (-(self_.g0_.x * other.g0_.x) - (self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z) + (self_.g1_.w * other.g0_.w))));
}
fn dualNum_geometricAntiQuotient_plane(self: DualNum, other: Plane) -> DualNum {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>(anti_inverse.g0_) * self_.g0_));
    return geometric_anti_product;
}
fn dualNum_mul_scalar(self: DualNum, other: Scalar) -> DualNum {
    return dualNum_geometricProduct_dualNum(self_, other);
}
fn motor_geometricQuotient_dualNum(self: Motor, other: DualNum) -> Motor {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_.x, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(inverse.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(inverse.g0_) * self_.g1_));
    return geometric_product;
}
fn motor_add_point(self: Motor, other: Point) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g1_.w, self_.g0_.w), /* e1, e2, e3, e4 */ other.g0_, /* e41, e42, e43 */ vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z), /* e23, e31, e12 */ vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    return addition;
}
fn motor_geometricProduct_plane(self: Motor, other: Plane) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g1_.x * other.g0_.w), (self_.g1_.y * other.g0_.w), (self_.g1_.z * other.g0_.w), (-(self_.g0_.w * other.g0_.w) - (self_.g1_.x * other.g0_.x) - (self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))), /* e423, e431, e412, e321 */ vec4<f32>((-(self_.g0_.x * other.g0_.w) - (self_.g1_.y * other.g0_.z) + (self_.g1_.z * other.g0_.y) + (self_.g1_.w * other.g0_.x)), (-(self_.g0_.y * other.g0_.w) + (self_.g1_.x * other.g0_.z) - (self_.g1_.z * other.g0_.x) + (self_.g1_.w * other.g0_.y)), (-(self_.g0_.z * other.g0_.w) - (self_.g1_.x * other.g0_.y) + (self_.g1_.y * other.g0_.x) + (self_.g1_.w * other.g0_.z)), (self_.g1_.w * other.g0_.w)));
}
fn line_add_flector(self: Line, other: Flector) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ other.g0_, /* e41, e42, e43 */ self_.g0_, /* e23, e31, e12 */ self_.g1_, /* e423, e431, e412, e321 */ other.g1_);
    return addition;
}
fn horizon_bitXor_flector(self: Horizon, other: Flector) -> AntiScalar {
    return horizon_wedge_horizon(self_, other);
}
fn scalar_antiSandwich_line(self: Scalar, other: Line) -> Line {
    let geometric_anti_product: Line = Line(/* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ (vec3<f32>(self_.g0_) * other.g0_));
    return line_geometricAntiProduct_line(geometric_anti_product, scalar_antiReverse(self_));
}
fn line_mul_dualNum(self: Line, other: DualNum) -> Line {
    return line_geometricProduct_line(self_, other);
}
fn dualNum_wedge_multiVector(self: DualNum, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((self_.g0_.x * other.g0_.x), ((self_.g0_.x * other.g0_.y) + (self_.g0_.y * other.g0_.x))), /* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g1_.x), (self_.g0_.x * other.g1_.y), (self_.g0_.x * other.g1_.z), (self_.g0_.x * other.g1_.w)), /* e41, e42, e43 */ vec3<f32>((self_.g0_.x * other.g2_.x), (self_.g0_.x * other.g2_.y), (self_.g0_.x * other.g2_.z)), /* e23, e31, e12 */ vec3<f32>((self_.g0_.x * other.g3_.x), (self_.g0_.x * other.g3_.y), (self_.g0_.x * other.g3_.z)), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.x * other.g4_.x), (self_.g0_.x * other.g4_.y), (self_.g0_.x * other.g4_.z), (self_.g0_.x * other.g4_.w)));
}
fn antiScalar_antiSandwich_antiScalar(self: AntiScalar, other: AntiScalar) -> AntiScalar {
    let geometric_anti_product: AntiScalar = AntiScalar(/* e1234 */ (other.g0_ * self_.g0_));
    return antiScalar_geometricAntiProduct_antiScalar(geometric_anti_product, antiScalar_antiReverse(self_));
}
fn motor_sandwich_dualNum(self: Motor, other: DualNum) -> Motor {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ ((vec4<f32>(other.g0_.x) * self_.g0_) + (vec4<f32>(other.g0_.y) * self_.g1_)), /* e23, e31, e12, scalar */ (vec4<f32>(other.g0_.x) * self_.g1_));
    return motor_geometricProduct_motor(geometric_product, motor_reverse(self_));
}
fn flector_mul_origin(self: Flector, other: Origin) -> Motor {
    return flector_geometricProduct_flector(self_, other);
}
fn dualNum_sub_scalar(self: DualNum, other: Scalar) -> DualNum {
    let subtraction: DualNum = DualNum(/* scalar, e1234 */ vec2<f32>((self_.g0_.x - other.g0_), self_.g0_.y));
    return subtraction;
}
fn multiVector_antiSandwich_scalar(self: MultiVector, other: Scalar) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>((self_.g0_.y * other.g0_), 0.0), /* e1, e2, e3, e4 */ vec4<f32>((self_.g4_.x * other.g0_ * -1.0), (self_.g4_.y * other.g0_ * -1.0), (self_.g4_.z * other.g0_ * -1.0), 0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ (vec3<f32>(other.g0_) * self_.g2_), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g1_.w * other.g0_ * -1.0)));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self_));
}
fn line_geometricAntiProduct_motor(self: Line, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(((self_.g0_.x * other.g0_.w) + (self_.g0_.y * other.g0_.z) - (self_.g0_.z * other.g0_.y)), (-(self_.g0_.x * other.g0_.z) + (self_.g0_.y * other.g0_.w) + (self_.g0_.z * other.g0_.x)), ((self_.g0_.x * other.g0_.y) - (self_.g0_.y * other.g0_.x) + (self_.g0_.z * other.g0_.w)), (-(self_.g0_.x * other.g0_.x) - (self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z))), /* e23, e31, e12, scalar */ vec4<f32>(((self_.g0_.x * other.g1_.w) + (self_.g0_.y * other.g1_.z) - (self_.g0_.z * other.g1_.y) + (self_.g1_.x * other.g0_.w) + (self_.g1_.y * other.g0_.z) - (self_.g1_.z * other.g0_.y)), (-(self_.g0_.x * other.g1_.z) + (self_.g0_.y * other.g1_.w) + (self_.g0_.z * other.g1_.x) - (self_.g1_.x * other.g0_.z) + (self_.g1_.y * other.g0_.w) + (self_.g1_.z * other.g0_.x)), ((self_.g0_.x * other.g1_.y) - (self_.g0_.y * other.g1_.x) + (self_.g0_.z * other.g1_.w) + (self_.g1_.x * other.g0_.y) - (self_.g1_.y * other.g0_.x) + (self_.g1_.z * other.g0_.w)), (-(self_.g0_.x * other.g1_.x) - (self_.g0_.y * other.g1_.y) - (self_.g0_.z * other.g1_.z) - (self_.g1_.x * other.g0_.x) - (self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))));
}
fn point_antiWedge_multiVector(self: Point, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(((other.g4_.x * self_.g0_.x) + (other.g4_.y * self_.g0_.y) + (other.g4_.z * self_.g0_.z) + (other.g4_.w * self_.g0_.w)), 0.0), /* e1, e2, e3, e4 */ vec4<f32>((other.g0_.y * self_.g0_.x), (other.g0_.y * self_.g0_.y), (other.g0_.y * self_.g0_.z), (other.g0_.y * self_.g0_.w)), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
}
fn antiScalar_geometricProduct_dualNum(self: AntiScalar, other: DualNum) -> AntiScalar {
    return AntiScalar(/* e1234 */ (other.g0_.x * self_.g0_));
}
fn plane_antiSandwich_flector(self: Plane, other: Flector) -> Flector {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>((-(other.g0_.w * self_.g0_.x) - (other.g1_.z * self_.g0_.y)), (-(other.g0_.w * self_.g0_.y) - (other.g1_.x * self_.g0_.z)), (-(other.g0_.w * self_.g0_.z) - (other.g1_.y * self_.g0_.x)), ((other.g1_.y * self_.g0_.y) + (other.g1_.z * self_.g0_.z))) + (other.g1_.yzxx * self_.g0_.zxyx)), /* e23, e31, e12, scalar */ (vec4<f32>(((other.g0_.z * self_.g0_.y) + (other.g1_.w * self_.g0_.x)), ((other.g0_.x * self_.g0_.z) + (other.g1_.w * self_.g0_.y)), ((other.g0_.y * self_.g0_.x) + (other.g1_.w * self_.g0_.z)), (-(other.g0_.z * self_.g0_.z) - (other.g0_.w * self_.g0_.w))) - (vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, other.g0_.y) * self_.g0_.wwwy) - (other.g0_.yzxx * self_.g0_.zxyx)));
    return motor_geometricAntiProduct_motor(geometric_anti_product, plane_antiReverse(self_));
}
fn dualNum_antiSandwich_plane(self: DualNum, other: Plane) -> Flector {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g0_.x), (self_.g0_.x * other.g0_.y), (self_.g0_.x * other.g0_.z), 0.0), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_.y) * other.g0_));
    return flector_geometricAntiProduct_flector(geometric_anti_product, dualNum_antiReverse(self_));
}
fn point_geometricAntiProduct_line(self: Point, other: Line) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((-(other.g0_.y * self_.g0_.z) + (other.g0_.z * self_.g0_.y) - (other.g1_.x * self_.g0_.w)), ((other.g0_.x * self_.g0_.z) - (other.g0_.z * self_.g0_.x) - (other.g1_.y * self_.g0_.w)), (-(other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x) - (other.g1_.z * self_.g0_.w)), 0.0), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.x * self_.g0_.w), (other.g0_.y * self_.g0_.w), (other.g0_.z * self_.g0_.w), (-(other.g0_.x * self_.g0_.x) - (other.g0_.y * self_.g0_.y) - (other.g0_.z * self_.g0_.z))));
}
fn point_geometricAntiQuotient_multiVector(self: Point, other: MultiVector) -> Point {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.y, 2) - pow(other.g2_.x, 2) - pow(other.g2_.y, 2) - pow(other.g2_.z, 2) - pow(other.g1_.w, 2) + pow(other.g4_.x, 2) + pow(other.g4_.y, 2) + pow(other.g4_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Point = Point(/* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_));
    return geometric_anti_product;
}
fn origin_add_antiScalar(self: Origin, other: AntiScalar) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, other.g0_), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.g0_), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    return addition;
}
fn flector_add_origin(self: Flector, other: Origin) -> Flector {
    let addition: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, (self_.g0_.w + other.g0_)), /* e423, e431, e412, e321 */ self_.g1_);
    return addition;
}
fn line_sub_dualNum(self: Line, other: DualNum) -> Motor {
    let subtraction: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, (other.g0_.y * -1.0)), /* e23, e31, e12, scalar */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, (other.g0_.x * -1.0)));
    return subtraction;
}
fn line_sub_antiScalar(self: Line, other: AntiScalar) -> Motor {
    let subtraction: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, (other.g0_ * -1.0)), /* e23, e31, e12, scalar */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, 0.0));
    return subtraction;
}
fn motor_geometricAntiQuotient_line(self: Motor, other: Line) -> Motor {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(anti_inverse.g0_) * self_.g1_));
    return geometric_anti_product;
}
fn scalar_mul_origin(self: Scalar, other: Origin) -> Origin {
    return scalar_geometricProduct_scalar(self_, other);
}
fn point_geometricQuotient_motor(self: Point, other: Motor) -> Point {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2) + pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Point = Point(/* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g0_));
    return geometric_product;
}
fn line_wedge_motor(self: Line, other: Motor) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g0_.x * other.g1_.w), (self_.g0_.y * other.g1_.w), (self_.g0_.z * other.g1_.w), (-(self_.g0_.x * other.g1_.x) - (self_.g0_.y * other.g1_.y) - (self_.g0_.z * other.g1_.z) - (self_.g1_.x * other.g0_.x) - (self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))), /* e23, e31, e12, scalar */ vec4<f32>((self_.g1_.x * other.g1_.w), (self_.g1_.y * other.g1_.w), (self_.g1_.z * other.g1_.w), 0.0));
}
fn point_sandwich_flector(self: Point, other: Flector) -> Flector {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>((-(other.g0_.w * self_.g0_.x) - (other.g1_.y * self_.g0_.z)), (-(other.g0_.w * self_.g0_.y) - (other.g1_.z * self_.g0_.x)), (-(other.g0_.w * self_.g0_.z) - (other.g1_.x * self_.g0_.y)), ((other.g1_.z * self_.g0_.z) + (other.g1_.w * self_.g0_.w))) + (vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.x) * self_.g0_.wwwx) + (other.g1_.zxyy * self_.g0_.yzxy)), /* e23, e31, e12, scalar */ (vec4<f32>((-(other.g0_.y * self_.g0_.z) - (other.g1_.w * self_.g0_.x)), (-(other.g0_.z * self_.g0_.x) - (other.g1_.w * self_.g0_.y)), (-(other.g0_.x * self_.g0_.y) - (other.g1_.w * self_.g0_.z)), ((other.g0_.y * self_.g0_.y) + (other.g0_.z * self_.g0_.z))) + (other.g0_.zxyx * self_.g0_.yzxx)));
    return motor_geometricProduct_motor(geometric_product, point_reverse(self_));
}
fn point_geometricQuotient_plane(self: Point, other: Plane) -> Point {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.w, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Point = Point(/* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g0_));
    return geometric_product;
}
fn dualNum_mul_antiScalar(self: DualNum, other: AntiScalar) -> AntiScalar {
    return dualNum_geometricProduct_dualNum(self_, other);
}
fn plane_geometricAntiProduct_plane(self: Plane, other: Plane) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>(((other.g0_.y * self_.g0_.z) - (other.g0_.z * self_.g0_.y)), (-(other.g0_.x * self_.g0_.z) + (other.g0_.z * self_.g0_.x)), ((other.g0_.x * self_.g0_.y) - (other.g0_.y * self_.g0_.x)), ((other.g0_.x * self_.g0_.x) + (other.g0_.y * self_.g0_.y) + (other.g0_.z * self_.g0_.z))), /* e23, e31, e12, scalar */ vec4<f32>((-(other.g0_.x * self_.g0_.w) + (other.g0_.w * self_.g0_.x)), (-(other.g0_.y * self_.g0_.w) + (other.g0_.w * self_.g0_.y)), (-(other.g0_.z * self_.g0_.w) + (other.g0_.w * self_.g0_.z)), 0.0));
}
fn line_sandwich_flector(self: Line, other: Flector) -> Flector {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(((self_.g1_.x * other.g1_.w) - (self_.g1_.y * other.g0_.z)), ((self_.g1_.y * other.g1_.w) - (self_.g1_.z * other.g0_.x)), (-(self_.g1_.x * other.g0_.y) + (self_.g1_.z * other.g1_.w)), ((self_.g0_.y * other.g0_.y) + (self_.g0_.z * other.g0_.z) - (self_.g1_.x * other.g1_.x) - (self_.g1_.y * other.g1_.y) - (self_.g1_.z * other.g1_.z))) + (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g0_.x) * other.g0_.yzxx)), /* e423, e431, e412, e321 */ (vec4<f32>((-(self_.g0_.x * other.g1_.w) + (self_.g0_.y * other.g0_.z) + (self_.g1_.x * other.g0_.w) - (self_.g1_.y * other.g1_.z) + (self_.g1_.z * other.g1_.y)), (-(self_.g0_.y * other.g1_.w) + (self_.g0_.z * other.g0_.x) + (self_.g1_.x * other.g1_.z) + (self_.g1_.y * other.g0_.w) - (self_.g1_.z * other.g1_.x)), ((self_.g0_.x * other.g0_.y) - (self_.g0_.z * other.g1_.w) - (self_.g1_.x * other.g1_.y) + (self_.g1_.y * other.g1_.x) + (self_.g1_.z * other.g0_.w)), (-(self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g1_.x) * other.g0_.yzxx)));
    return flector_geometricProduct_flector(geometric_product, line_reverse(self_));
}
fn plane_sub_origin(self: Plane, other: Origin) -> Flector {
    let subtraction: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_ * -1.0)), /* e423, e431, e412, e321 */ self_.g0_);
    return subtraction;
}
fn multiVector_antiWedge_multiVector(self: MultiVector, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(((other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x) - (other.g2_.x * self_.g3_.x) - (other.g2_.y * self_.g3_.y) - (other.g2_.z * self_.g3_.z) - (other.g3_.x * self_.g2_.x) - (other.g3_.y * self_.g2_.y) - (other.g3_.z * self_.g2_.z) - (other.g1_.x * self_.g4_.x) - (other.g1_.y * self_.g4_.y) - (other.g1_.z * self_.g4_.z) - (other.g1_.w * self_.g4_.w) + (other.g4_.x * self_.g1_.x) + (other.g4_.y * self_.g1_.y) + (other.g4_.z * self_.g1_.z) + (other.g4_.w * self_.g1_.w)), (other.g0_.y * self_.g0_.y)), /* e1, e2, e3, e4 */ vec4<f32>(((other.g0_.y * self_.g1_.x) + (self_.g0_.y * other.g1_.x) + (other.g2_.x * self_.g4_.w) + (other.g3_.y * self_.g4_.z) - (other.g3_.z * self_.g4_.y) + (self_.g2_.x * other.g4_.w) + (self_.g3_.y * other.g4_.z) - (self_.g3_.z * other.g4_.y)), ((other.g0_.y * self_.g1_.y) + (self_.g0_.y * other.g1_.y) + (other.g2_.y * self_.g4_.w) - (other.g3_.x * self_.g4_.z) + (other.g3_.z * self_.g4_.x) + (self_.g2_.y * other.g4_.w) - (self_.g3_.x * other.g4_.z) + (self_.g3_.z * other.g4_.x)), ((other.g0_.y * self_.g1_.z) + (self_.g0_.y * other.g1_.z) + (other.g2_.z * self_.g4_.w) + (other.g3_.x * self_.g4_.y) - (other.g3_.y * self_.g4_.x) + (self_.g2_.z * other.g4_.w) + (self_.g3_.x * other.g4_.y) - (self_.g3_.y * other.g4_.x)), ((other.g0_.y * self_.g1_.w) + (self_.g0_.y * other.g1_.w) - (other.g2_.x * self_.g4_.x) - (other.g2_.y * self_.g4_.y) - (other.g2_.z * self_.g4_.z) - (self_.g2_.x * other.g4_.x) - (self_.g2_.y * other.g4_.y) - (self_.g2_.z * other.g4_.z))), /* e41, e42, e43 */ vec3<f32>(((other.g0_.y * self_.g2_.x) + (self_.g0_.y * other.g2_.x) + (other.g4_.y * self_.g4_.z) - (other.g4_.z * self_.g4_.y)), ((other.g0_.y * self_.g2_.y) + (self_.g0_.y * other.g2_.y) - (other.g4_.x * self_.g4_.z) + (other.g4_.z * self_.g4_.x)), ((other.g0_.y * self_.g2_.z) + (self_.g0_.y * other.g2_.z) + (other.g4_.x * self_.g4_.y) - (other.g4_.y * self_.g4_.x))), /* e23, e31, e12 */ vec3<f32>(((other.g0_.y * self_.g3_.x) + (self_.g0_.y * other.g3_.x) - (other.g4_.x * self_.g4_.w) + (other.g4_.w * self_.g4_.x)), ((other.g0_.y * self_.g3_.y) + (self_.g0_.y * other.g3_.y) - (other.g4_.y * self_.g4_.w) + (other.g4_.w * self_.g4_.y)), ((other.g0_.y * self_.g3_.z) + (self_.g0_.y * other.g3_.z) - (other.g4_.z * self_.g4_.w) + (other.g4_.w * self_.g4_.z))), /* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.y * self_.g4_.x) + (self_.g0_.y * other.g4_.x)), ((other.g0_.y * self_.g4_.y) + (self_.g0_.y * other.g4_.y)), ((other.g0_.y * self_.g4_.z) + (self_.g0_.y * other.g4_.z)), ((other.g0_.y * self_.g4_.w) + (self_.g0_.y * other.g4_.w))));
}
fn dualNum_antiSandwich_dualNum(self: DualNum, other: DualNum) -> DualNum {
    let geometric_anti_product: DualNum = DualNum(/* scalar, e1234 */ vec2<f32>(((other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x)), (other.g0_.y * self_.g0_.y)));
    return dualNum_geometricAntiProduct_dualNum(geometric_anti_product, dualNum_antiReverse(self_));
}
fn scalar_add_horizon(self: Scalar, other: Horizon) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g0_, 0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other.g0_));
    return addition;
}
fn plane_antiWedge_multiVector(self: Plane, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((-(other.g1_.x * self_.g0_.x) - (other.g1_.y * self_.g0_.y) - (other.g1_.z * self_.g0_.z) - (other.g1_.w * self_.g0_.w)), 0.0), /* e1, e2, e3, e4 */ vec4<f32>(((other.g2_.x * self_.g0_.w) + (other.g3_.y * self_.g0_.z) - (other.g3_.z * self_.g0_.y)), ((other.g2_.y * self_.g0_.w) - (other.g3_.x * self_.g0_.z) + (other.g3_.z * self_.g0_.x)), ((other.g2_.z * self_.g0_.w) + (other.g3_.x * self_.g0_.y) - (other.g3_.y * self_.g0_.x)), (-(other.g2_.x * self_.g0_.x) - (other.g2_.y * self_.g0_.y) - (other.g2_.z * self_.g0_.z))), /* e41, e42, e43 */ vec3<f32>(((other.g4_.y * self_.g0_.z) - (other.g4_.z * self_.g0_.y)), (-(other.g4_.x * self_.g0_.z) + (other.g4_.z * self_.g0_.x)), ((other.g4_.x * self_.g0_.y) - (other.g4_.y * self_.g0_.x))), /* e23, e31, e12 */ vec3<f32>((-(other.g4_.x * self_.g0_.w) + (other.g4_.w * self_.g0_.x)), (-(other.g4_.y * self_.g0_.w) + (other.g4_.w * self_.g0_.y)), (-(other.g4_.z * self_.g0_.w) + (other.g4_.w * self_.g0_.z))), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.y * self_.g0_.x), (other.g0_.y * self_.g0_.y), (other.g0_.y * self_.g0_.z), (other.g0_.y * self_.g0_.w)));
}
fn motor_add_dualNum(self: Motor, other: DualNum) -> Motor {
    let addition: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, (other.g0_.y + self_.g0_.w)), /* e23, e31, e12, scalar */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, (other.g0_.x + self_.g1_.w)));
    return addition;
}
fn scalar_geometricAntiQuotient_multiVector(self: Scalar, other: MultiVector) -> Scalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.y, 2) - pow(other.g2_.x, 2) - pow(other.g2_.y, 2) - pow(other.g2_.z, 2) - pow(other.g1_.w, 2) + pow(other.g4_.x, 2) + pow(other.g4_.y, 2) + pow(other.g4_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Scalar = Scalar(/* scalar */ (anti_inverse.g0_ * self_.g0_));
    return geometric_anti_product;
}
fn flector_sandwich_motor(self: Flector, other: Motor) -> Motor {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>((self_.g0_.z * other.g1_.y), (self_.g0_.y * other.g1_.w), (self_.g0_.z * other.g1_.w), (-(self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z) - (self_.g1_.x * other.g1_.x) - (self_.g1_.y * other.g1_.y) - (self_.g1_.z * other.g1_.z))) + (vec4<f32>(self_.g1_.w) * vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, other.g0_.w)) - (vec4<f32>(other.g1_.z, other.g1_.x, other.g1_.y, other.g0_.x) * self_.g0_.yzxx) + (self_.g0_.xxyw * other.g1_.wzxw)), /* e423, e431, e412, e321 */ (vec4<f32>(((self_.g0_.x * other.g0_.w) + (self_.g0_.z * other.g0_.y) + (self_.g1_.x * other.g1_.w) + (self_.g1_.z * other.g1_.y) + (self_.g1_.w * other.g0_.x)), ((self_.g0_.x * other.g0_.z) + (self_.g0_.y * other.g0_.w) + (self_.g1_.x * other.g1_.z) + (self_.g1_.y * other.g1_.w) + (self_.g1_.w * other.g0_.y)), ((self_.g0_.y * other.g0_.x) + (self_.g0_.z * other.g0_.w) + (self_.g1_.y * other.g1_.x) + (self_.g1_.z * other.g1_.w) + (self_.g1_.w * other.g0_.z)), ((self_.g0_.z * other.g1_.z) * -1.0)) + (vec4<f32>(self_.g0_.w, self_.g0_.w, self_.g0_.w, self_.g1_.w) * other.g1_) - (vec4<f32>(self_.g1_.y, self_.g1_.z, self_.g1_.x, self_.g0_.y) * other.g1_.zxyy) - (vec4<f32>(other.g0_.z, other.g0_.x, other.g0_.y, other.g1_.x) * self_.g0_.yzxx)));
    return flector_geometricProduct_flector(geometric_product, flector_reverse(self_));
}
fn antiScalar_geometricAntiProduct_dualNum(self: AntiScalar, other: DualNum) -> DualNum {
    return DualNum(/* scalar, e1234 */ vec2<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_)));
}
fn dualNum_geometricQuotient_multiVector(self: DualNum, other: MultiVector) -> DualNum {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) - pow(other.g3_.x, 2) - pow(other.g3_.y, 2) - pow(other.g3_.z, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2) - pow(other.g4_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>(inverse.g0_) * self_.g0_));
    return geometric_product;
}
fn motor_antiWedge_plane(self: Motor, other: Plane) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>(((self_.g0_.x * other.g0_.w) + (self_.g1_.y * other.g0_.z) - (self_.g1_.z * other.g0_.y)), ((self_.g0_.y * other.g0_.w) - (self_.g1_.x * other.g0_.z) + (self_.g1_.z * other.g0_.x)), ((self_.g0_.z * other.g0_.w) + (self_.g1_.x * other.g0_.y) - (self_.g1_.y * other.g0_.x)), (-(self_.g0_.x * other.g0_.x) - (self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z))), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.w * other.g0_.x), (self_.g0_.w * other.g0_.y), (self_.g0_.w * other.g0_.z), (self_.g0_.w * other.g0_.w)));
}
fn point_mul_flector(self: Point, other: Flector) -> Motor {
    return point_geometricProduct_point(self_, other);
}
fn motor_sub_multiVector(self: Motor, other: MultiVector) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(self_.g1_.w, self_.g0_.w) - other.g0_), /* e1, e2, e3, e4 */ (other.g1_ * vec4<f32>(-1.0)), /* e41, e42, e43 */ (vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z) - other.g2_), /* e23, e31, e12 */ (vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z) - other.g3_), /* e423, e431, e412, e321 */ (other.g4_ * vec4<f32>(-1.0)));
    return subtraction;
}
fn motor_geometricAntiQuotient_plane(self: Motor, other: Plane) -> Motor {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(anti_inverse.g0_) * self_.g1_));
    return geometric_anti_product;
}
fn point_wedge_origin(self: Point, other: Origin) -> Line {
    return Line(/* e41, e42, e43 */ vec3<f32>((self_.g0_.x * other.g0_ * -1.0), (self_.g0_.y * other.g0_ * -1.0), (self_.g0_.z * other.g0_ * -1.0)), /* e23, e31, e12 */ vec3<f32>(0.0));
}
fn motor_geometricAntiProduct_origin(self: Motor, other: Origin) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), (self_.g0_.w * other.g0_)), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), (self_.g1_.w * other.g0_)));
}
fn scalar_add_scalar(self: Scalar, other: Scalar) -> Scalar {
    let addition: Scalar = Scalar(/* scalar */ (other.g0_ + self_.g0_));
    return addition;
}
fn point_mul_plane(self: Point, other: Plane) -> Motor {
    return point_geometricProduct_point(self_, other);
}
fn plane_mul_origin(self: Plane, other: Origin) -> AntiScalar {
    return plane_geometricProduct_plane(self_, other);
}
fn dualNum_sandwich_dualNum(self: DualNum, other: DualNum) -> DualNum {
    let geometric_product: DualNum = DualNum(/* scalar, e1234 */ vec2<f32>((other.g0_.x * self_.g0_.x), ((other.g0_.x * self_.g0_.y) + (other.g0_.y * self_.g0_.x))));
    return dualNum_geometricProduct_dualNum(geometric_product, dualNum_reverse(self_));
}
fn horizon_geometricProduct_scalar(self: Horizon, other: Scalar) -> Horizon {
    return Horizon(/* e321 */ (self_.g0_ * other.g0_));
}
fn origin_sandwich_line(self: Origin, other: Line) -> AntiScalar {
    let geometric_product: Plane = Plane(/* e423, e431, e412, e321 */ vec4<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_), 0.0));
    return plane_geometricProduct_plane(geometric_product, origin_reverse(self_));
}
fn multiVector_add_origin(self: MultiVector, other: Origin) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, (self_.g1_.w + other.g0_)), /* e41, e42, e43 */ self_.g2_, /* e23, e31, e12 */ self_.g3_, /* e423, e431, e412, e321 */ self_.g4_);
    return addition;
}
fn origin_wedge_multiVector(self: Origin, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (other.g4_.w * self_.g0_)), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.x * self_.g0_)), /* e41, e42, e43 */ vec3<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_)), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>((other.g3_.x * self_.g0_), (other.g3_.y * self_.g0_), (other.g3_.z * self_.g0_), 0.0));
}
fn scalar_geometricProduct_horizon(self: Scalar, other: Horizon) -> Horizon {
    return Horizon(/* e321 */ (other.g0_ * self_.g0_));
}
fn point_sandwich_horizon(self: Point, other: Horizon) -> Flector {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.w * other.g0_)), /* e23, e31, e12, scalar */ vec4<f32>((self_.g0_.x * other.g0_ * -1.0), (self_.g0_.y * other.g0_ * -1.0), (self_.g0_.z * other.g0_ * -1.0), 0.0));
    return motor_geometricProduct_motor(geometric_product, point_reverse(self_));
}
fn motor_sandwich_point(self: Motor, other: Point) -> Flector {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(((self_.g1_.y * other.g0_.z) * -1.0), ((self_.g1_.z * other.g0_.x) * -1.0), ((self_.g1_.x * other.g0_.y) * -1.0), ((self_.g0_.z * other.g0_.z) + (self_.g1_.w * other.g0_.w))) + (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g0_.x) * other.g0_.yzxx) + (vec4<f32>(self_.g1_.w, self_.g1_.w, self_.g1_.w, self_.g0_.y) * other.g0_.xyzy)), /* e423, e431, e412, e321 */ (vec4<f32>(((self_.g0_.y * other.g0_.z) + (self_.g1_.x * other.g0_.w)), ((self_.g0_.z * other.g0_.x) + (self_.g1_.y * other.g0_.w)), ((self_.g0_.x * other.g0_.y) + (self_.g1_.z * other.g0_.w)), ((self_.g1_.z * other.g0_.z) * -1.0)) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g1_.x) * other.g0_.yzxx) - (vec4<f32>(self_.g0_.w, self_.g0_.w, self_.g0_.w, self_.g1_.y) * other.g0_.xyzy)));
    return flector_geometricProduct_flector(geometric_product, motor_reverse(self_));
}
fn point_antiWedge_flector(self: Point, other: Flector) -> Scalar {
    return Scalar(/* scalar */ ((other.g1_.x * self_.g0_.x) + (other.g1_.y * self_.g0_.y) + (other.g1_.z * self_.g0_.z) + (other.g1_.w * self_.g0_.w)));
}
fn scalar_bitXor_origin(self: Scalar, other: Origin) -> Origin {
    return scalar_wedge_scalar(self_, other);
}
fn point_wedge_plane(self: Point, other: Plane) -> AntiScalar {
    return AntiScalar(/* e1234 */ ((other.g0_.x * self_.g0_.x) + (other.g0_.y * self_.g0_.y) + (other.g0_.z * self_.g0_.z) + (other.g0_.w * self_.g0_.w)));
}
fn flector_add_multiVector(self: Flector, other: MultiVector) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ other.g0_, /* e1, e2, e3, e4 */ (self_.g0_ + other.g1_), /* e41, e42, e43 */ other.g2_, /* e23, e31, e12 */ other.g3_, /* e423, e431, e412, e321 */ (self_.g1_ + other.g4_));
    return addition;
}
fn origin_antiSandwich_multiVector(self: Origin, other: MultiVector) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(self_.g0_) * vec2<f32>(other.g4_.w, other.g1_.w) * vec2<f32>(1.0, -1.0)), /* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g3_.x, other.g3_.y, other.g3_.z, other.g0_.y) * vec4<f32>(-1.0, -1.0, -1.0, 1.0)), /* e41, e42, e43 */ (vec3<f32>(self_.g0_) * vec3<f32>(other.g4_.x, other.g4_.y, other.g4_.z) * vec3<f32>(-1.0)), /* e23, e31, e12 */ (vec3<f32>(self_.g0_) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z)), /* e423, e431, e412, e321 */ (vec4<f32>(self_.g0_) * vec4<f32>(other.g2_.x, other.g2_.y, other.g2_.z, other.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, -1.0)));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, origin_antiReverse(self_));
}
fn dualNum_geometricAntiQuotient_flector(self: DualNum, other: Flector) -> DualNum {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.w, 2) + pow(other.g1_.x, 2) + pow(other.g1_.y, 2) + pow(other.g1_.z, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>(anti_inverse.g0_) * self_.g0_));
    return geometric_anti_product;
}
fn line_mul_scalar(self: Line, other: Scalar) -> Line {
    return line_geometricProduct_line(self_, other);
}
fn antiScalar_add_origin(self: AntiScalar, other: Origin) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, self_.g0_), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other.g0_), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    return addition;
}
fn antiScalar_geometricAntiProduct_scalar(self: AntiScalar, other: Scalar) -> Scalar {
    return Scalar(/* scalar */ (self_.g0_ * other.g0_));
}
fn scalar_geometricAntiQuotient_motor(self: Scalar, other: Motor) -> Scalar {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2) + pow(other.g0_.w, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Scalar = Scalar(/* scalar */ (anti_inverse.g0_ * self_.g0_));
    return geometric_anti_product;
}
fn horizon_bitXor_dualNum(self: Horizon, other: DualNum) -> Horizon {
    return horizon_wedge_horizon(self_, other);
}
fn motor_mul_scalar(self: Motor, other: Scalar) -> Motor {
    return motor_geometricProduct_motor(self_, other);
}
fn antiScalar_antiSandwich_motor(self: AntiScalar, other: Motor) -> Motor {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(self_.g0_) * other.g0_), /* e23, e31, e12, scalar */ (vec4<f32>(self_.g0_) * other.g1_));
    return motor_geometricAntiProduct_motor(geometric_anti_product, antiScalar_antiReverse(self_));
}
fn scalar_bitXor_point(self: Scalar, other: Point) -> Point {
    return scalar_wedge_scalar(self_, other);
}
fn dualNum_bitXor_point(self: DualNum, other: Point) -> Point {
    return dualNum_wedge_dualNum(self_, other);
}
fn dualNum_add_horizon(self: DualNum, other: Horizon) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other.g0_));
    return addition;
}
fn scalar_mul_point(self: Scalar, other: Point) -> Point {
    return scalar_geometricProduct_scalar(self_, other);
}
fn horizon_geometricProduct_motor(self: Horizon, other: Motor) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_), (other.g0_.w * self_.g0_)), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), (other.g1_.w * self_.g0_)));
}
fn antiScalar_antiSandwich_horizon(self: AntiScalar, other: Horizon) -> Horizon {
    let geometric_anti_product: Horizon = Horizon(/* e321 */ (self_.g0_ * other.g0_));
    return horizon_geometricAntiProduct_horizon(geometric_anti_product, antiScalar_antiReverse(self_));
}
fn antiScalar_bitXor_scalar(self: AntiScalar, other: Scalar) -> AntiScalar {
    return antiScalar_wedge_antiScalar(self_, other);
}
fn multiVector_bitXor_horizon(self: MultiVector, other: Horizon) -> MultiVector {
    return multiVector_wedge_multiVector(self_, other);
}
fn dualNum_mul_origin(self: DualNum, other: Origin) -> Origin {
    return dualNum_geometricProduct_dualNum(self_, other);
}
fn line_mul_origin(self: Line, other: Origin) -> Plane {
    return line_geometricProduct_line(self_, other);
}
fn origin_mul_plane(self: Origin, other: Plane) -> AntiScalar {
    return origin_geometricProduct_origin(self_, other);
}
fn motor_add_origin(self: Motor, other: Origin) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g1_.w, self_.g0_.w), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other.g0_), /* e41, e42, e43 */ vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z), /* e23, e31, e12 */ vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    return addition;
}
fn scalar_wedge_horizon(self: Scalar, other: Horizon) -> Horizon {
    return Horizon(/* e321 */ (other.g0_ * self_.g0_));
}
fn line_geometricQuotient_dualNum(self: Line, other: DualNum) -> Line {
    let scalar_product: Scalar = Scalar(/* scalar */ pow(other.g0_.x, 2));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Line = Line(/* e41, e42, e43 */ (vec3<f32>(inverse.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(inverse.g0_) * self_.g1_));
    return geometric_product;
}
fn motor_antiSandwich_plane(self: Motor, other: Plane) -> Flector {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(((self_.g0_.x * other.g0_.w) + (self_.g1_.y * other.g0_.z) + (self_.g1_.w * other.g0_.x)), ((self_.g0_.y * other.g0_.w) + (self_.g1_.z * other.g0_.x) + (self_.g1_.w * other.g0_.y)), ((self_.g0_.z * other.g0_.w) + (self_.g1_.x * other.g0_.y) + (self_.g1_.w * other.g0_.z)), (-(self_.g0_.y * other.g0_.y) - (self_.g0_.z * other.g0_.z))) - (vec4<f32>(self_.g1_.z, self_.g1_.x, self_.g1_.y, self_.g0_.x) * other.g0_.yzxx)), /* e423, e431, e412, e321 */ (vec4<f32>((self_.g0_.w * other.g0_.x), (self_.g0_.w * other.g0_.y), (self_.g0_.w * other.g0_.z), (-(self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))) - (vec4<f32>(self_.g0_.z, self_.g0_.x, self_.g0_.y, self_.g1_.x) * other.g0_.yzxx) + (self_.g0_.yzxw * other.g0_.zxyw)));
    return flector_geometricAntiProduct_flector(geometric_anti_product, motor_antiReverse(self_));
}
fn scalar_geometricQuotient_flector(self: Scalar, other: Flector) -> Scalar {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2) - pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Scalar = Scalar(/* scalar */ (inverse.g0_ * self_.g0_));
    return geometric_product;
}
fn line_geometricAntiQuotient_point(self: Line, other: Point) -> Line {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.w, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Line = Line(/* e41, e42, e43 */ (vec3<f32>(anti_inverse.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(anti_inverse.g0_) * self_.g1_));
    return geometric_anti_product;
}
fn flector_geometricAntiQuotient_motor(self: Flector, other: Motor) -> Flector {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2) + pow(other.g0_.w, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g1_));
    return geometric_anti_product;
}
fn scalar_sandwich_scalar(self: Scalar, other: Scalar) -> Scalar {
    let geometric_product: Scalar = Scalar(/* scalar */ (other.g0_ * self_.g0_));
    return scalar_geometricProduct_scalar(geometric_product, scalar_reverse(self_));
}
fn multiVector_wedge_flector(self: MultiVector, other: Flector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (-(other.g0_.x * self_.g4_.x) - (other.g0_.y * self_.g4_.y) - (other.g0_.z * self_.g4_.z) - (other.g0_.w * self_.g4_.w) + (other.g1_.x * self_.g1_.x) + (other.g1_.y * self_.g1_.y) + (other.g1_.z * self_.g1_.z) + (other.g1_.w * self_.g1_.w))), /* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g0_.x), (self_.g0_.x * other.g0_.y), (self_.g0_.x * other.g0_.z), (self_.g0_.x * other.g0_.w)), /* e41, e42, e43 */ vec3<f32>(((other.g0_.x * self_.g1_.w) - (other.g0_.w * self_.g1_.x)), ((other.g0_.y * self_.g1_.w) - (other.g0_.w * self_.g1_.y)), ((other.g0_.z * self_.g1_.w) - (other.g0_.w * self_.g1_.z))), /* e23, e31, e12 */ vec3<f32>((-(other.g0_.y * self_.g1_.z) + (other.g0_.z * self_.g1_.y)), ((other.g0_.x * self_.g1_.z) - (other.g0_.z * self_.g1_.x)), (-(other.g0_.x * self_.g1_.y) + (other.g0_.y * self_.g1_.x))), /* e423, e431, e412, e321 */ vec4<f32>(((self_.g0_.x * other.g1_.x) + (self_.g2_.y * other.g0_.z) - (self_.g2_.z * other.g0_.y) + (self_.g3_.x * other.g0_.w)), ((self_.g0_.x * other.g1_.y) - (self_.g2_.x * other.g0_.z) + (self_.g2_.z * other.g0_.x) + (self_.g3_.y * other.g0_.w)), ((self_.g0_.x * other.g1_.z) + (self_.g2_.x * other.g0_.y) - (self_.g2_.y * other.g0_.x) + (self_.g3_.z * other.g0_.w)), ((self_.g0_.x * other.g1_.w) - (self_.g3_.x * other.g0_.x) - (self_.g3_.y * other.g0_.y) - (self_.g3_.z * other.g0_.z))));
}
fn line_sub_origin(self: Line, other: Origin) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_ * -1.0)), /* e41, e42, e43 */ self_.g0_, /* e23, e31, e12 */ self_.g1_, /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    return subtraction;
}
fn motor_sub_antiScalar(self: Motor, other: AntiScalar) -> Motor {
    let subtraction: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, (self_.g0_.w - other.g0_)), /* e23, e31, e12, scalar */ self_.g1_);
    return subtraction;
}
fn horizon_antiSandwich_line(self: Horizon, other: Line) -> Scalar {
    let geometric_anti_product: Point = Point(/* e1, e2, e3, e4 */ vec4<f32>((other.g0_.x * self_.g0_), (other.g0_.y * self_.g0_), (other.g0_.z * self_.g0_), 0.0));
    return point_geometricAntiProduct_point(geometric_anti_product, horizon_antiReverse(self_));
}
fn motor_bitXor_scalar(self: Motor, other: Scalar) -> Motor {
    return motor_wedge_motor(self_, other);
}
fn motor_bitXor_point(self: Motor, other: Point) -> Flector {
    return motor_wedge_motor(self_, other);
}
fn dualNum_mul_plane(self: DualNum, other: Plane) -> Flector {
    return dualNum_geometricProduct_dualNum(self_, other);
}
fn scalar_geometricAntiProduct_point(self: Scalar, other: Point) -> Horizon {
    return Horizon(/* e321 */ (other.g0_.w * self_.g0_));
}
fn origin_antiSandwich_plane(self: Origin, other: Plane) -> Flector {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>((other.g0_.x * self_.g0_ * -1.0), (other.g0_.y * self_.g0_ * -1.0), (other.g0_.z * self_.g0_ * -1.0), 0.0), /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_.w * self_.g0_)));
    return motor_geometricAntiProduct_motor(geometric_anti_product, origin_antiReverse(self_));
}
fn flector_antiSandwich_dualNum(self: Flector, other: DualNum) -> Motor {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>((-(other.g0_.x * self_.g1_.x) + (other.g0_.y * self_.g0_.x)), (-(other.g0_.x * self_.g1_.y) + (other.g0_.y * self_.g0_.y)), (-(other.g0_.x * self_.g1_.z) + (other.g0_.y * self_.g0_.z)), (other.g0_.y * self_.g0_.w)), /* e423, e431, e412, e321 */ vec4<f32>((other.g0_.y * self_.g1_.x), (other.g0_.y * self_.g1_.y), (other.g0_.y * self_.g1_.z), (-(other.g0_.x * self_.g0_.w) + (other.g0_.y * self_.g1_.w))));
    return flector_geometricAntiProduct_flector(geometric_anti_product, flector_antiReverse(self_));
}
fn flector_geometricAntiQuotient_antiScalar(self: Flector, other: AntiScalar) -> Flector {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_), /* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g1_));
    return geometric_anti_product;
}
fn line_geometricQuotient_point(self: Line, other: Point) -> Line {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Line = Line(/* e41, e42, e43 */ (vec3<f32>(inverse.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(inverse.g0_) * self_.g1_));
    return geometric_product;
}
fn horizon_geometricQuotient_flector(self: Horizon, other: Flector) -> Horizon {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.x, 2) + pow(other.g0_.y, 2) + pow(other.g0_.z, 2) - pow(other.g1_.w, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Horizon = Horizon(/* e321 */ (self_.g0_ * inverse.g0_));
    return geometric_product;
}
fn plane_sub_multiVector(self: Plane, other: MultiVector) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ (other.g0_ * vec2<f32>(-1.0)), /* e1, e2, e3, e4 */ (other.g1_ * vec4<f32>(-1.0)), /* e41, e42, e43 */ (other.g2_ * vec3<f32>(-1.0)), /* e23, e31, e12 */ (other.g3_ * vec3<f32>(-1.0)), /* e423, e431, e412, e321 */ (-other.g4_ + self_.g0_));
    return subtraction;
}
fn multiVector_geometricProduct_line(self: MultiVector, other: Line) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>((-(other.g1_.x * self_.g3_.x) - (other.g1_.y * self_.g3_.y) - (other.g1_.z * self_.g3_.z)), (-(other.g0_.x * self_.g3_.x) - (other.g0_.y * self_.g3_.y) - (other.g0_.z * self_.g3_.z) - (other.g1_.x * self_.g2_.x) - (other.g1_.y * self_.g2_.y) - (other.g1_.z * self_.g2_.z))), /* e1, e2, e3, e4 */ vec4<f32>(((other.g1_.x * self_.g4_.w) + (other.g1_.y * self_.g1_.z) - (other.g1_.z * self_.g1_.y)), (-(other.g1_.x * self_.g1_.z) + (other.g1_.y * self_.g4_.w) + (other.g1_.z * self_.g1_.x)), ((other.g1_.x * self_.g1_.y) - (other.g1_.y * self_.g1_.x) + (other.g1_.z * self_.g4_.w)), (-(other.g0_.x * self_.g1_.x) - (other.g0_.y * self_.g1_.y) - (other.g0_.z * self_.g1_.z) - (other.g1_.x * self_.g4_.x) - (other.g1_.y * self_.g4_.y) - (other.g1_.z * self_.g4_.z))), /* e41, e42, e43 */ vec3<f32>(((self_.g0_.x * other.g0_.x) + (self_.g0_.y * other.g1_.x) + (other.g0_.y * self_.g3_.z) - (other.g0_.z * self_.g3_.y) + (other.g1_.y * self_.g2_.z) - (other.g1_.z * self_.g2_.y)), ((self_.g0_.x * other.g0_.y) + (self_.g0_.y * other.g1_.y) - (other.g0_.x * self_.g3_.z) + (other.g0_.z * self_.g3_.x) - (other.g1_.x * self_.g2_.z) + (other.g1_.z * self_.g2_.x)), ((self_.g0_.x * other.g0_.z) + (self_.g0_.y * other.g1_.z) + (other.g0_.x * self_.g3_.y) - (other.g0_.y * self_.g3_.x) + (other.g1_.x * self_.g2_.y) - (other.g1_.y * self_.g2_.x))), /* e23, e31, e12 */ vec3<f32>(((self_.g0_.x * other.g1_.x) + (other.g1_.y * self_.g3_.z) - (other.g1_.z * self_.g3_.y)), ((self_.g0_.x * other.g1_.y) - (other.g1_.x * self_.g3_.z) + (other.g1_.z * self_.g3_.x)), ((self_.g0_.x * other.g1_.z) + (other.g1_.x * self_.g3_.y) - (other.g1_.y * self_.g3_.x))), /* e423, e431, e412, e321 */ vec4<f32>(((other.g0_.x * self_.g4_.w) + (other.g0_.y * self_.g1_.z) - (other.g0_.z * self_.g1_.y) + (other.g1_.x * self_.g1_.w) + (other.g1_.y * self_.g4_.z) - (other.g1_.z * self_.g4_.y)), (-(other.g0_.x * self_.g1_.z) + (other.g0_.y * self_.g4_.w) + (other.g0_.z * self_.g1_.x) - (other.g1_.x * self_.g4_.z) + (other.g1_.y * self_.g1_.w) + (other.g1_.z * self_.g4_.x)), ((other.g0_.x * self_.g1_.y) - (other.g0_.y * self_.g1_.x) + (other.g0_.z * self_.g4_.w) + (other.g1_.x * self_.g4_.y) - (other.g1_.y * self_.g4_.x) + (other.g1_.z * self_.g1_.w)), (-(other.g1_.x * self_.g1_.x) - (other.g1_.y * self_.g1_.y) - (other.g1_.z * self_.g1_.z))));
}
fn origin_sandwich_motor(self: Origin, other: Motor) -> Motor {
    let geometric_product: Flector = Flector(/* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other.g1_.w * self_.g0_)), /* e423, e431, e412, e321 */ vec4<f32>((other.g1_.x * self_.g0_), (other.g1_.y * self_.g0_), (other.g1_.z * self_.g0_), 0.0));
    return flector_geometricProduct_flector(geometric_product, origin_reverse(self_));
}
fn dualNum_geometricAntiProduct_plane(self: DualNum, other: Plane) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g0_.x), (self_.g0_.x * other.g0_.y), (self_.g0_.x * other.g0_.z), 0.0), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.y * other.g0_.x), (self_.g0_.y * other.g0_.y), (self_.g0_.y * other.g0_.z), (self_.g0_.y * other.g0_.w)));
}
fn dualNum_sub_dualNum(self: DualNum, other: DualNum) -> DualNum {
    let subtraction: DualNum = DualNum(/* scalar, e1234 */ (-other.g0_ + self_.g0_));
    return subtraction;
}
fn motor_mul_line(self: Motor, other: Line) -> Motor {
    return motor_geometricProduct_motor(self_, other);
}
fn motor_mul_origin(self: Motor, other: Origin) -> Flector {
    return motor_geometricProduct_motor(self_, other);
}
fn line_add_dualNum(self: Line, other: DualNum) -> Motor {
    let addition: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, other.g0_.y), /* e23, e31, e12, scalar */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, other.g0_.x));
    return addition;
}
fn antiScalar_mul_horizon(self: AntiScalar, other: Horizon) -> Origin {
    return antiScalar_geometricProduct_antiScalar(self_, other);
}
fn line_add_horizon(self: Line, other: Horizon) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ self_.g0_, /* e23, e31, e12 */ self_.g1_, /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other.g0_));
    return addition;
}
fn line_geometricProduct_plane(self: Line, other: Plane) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g1_.x * other.g0_.w), (self_.g1_.y * other.g0_.w), (self_.g1_.z * other.g0_.w), (-(self_.g1_.x * other.g0_.x) - (self_.g1_.y * other.g0_.y) - (self_.g1_.z * other.g0_.z))), /* e423, e431, e412, e321 */ vec4<f32>((-(self_.g0_.x * other.g0_.w) - (self_.g1_.y * other.g0_.z) + (self_.g1_.z * other.g0_.y)), (-(self_.g0_.y * other.g0_.w) + (self_.g1_.x * other.g0_.z) - (self_.g1_.z * other.g0_.x)), (-(self_.g0_.z * other.g0_.w) - (self_.g1_.x * other.g0_.y) + (self_.g1_.y * other.g0_.x)), 0.0));
}
fn dualNum_geometricAntiQuotient_motor(self: DualNum, other: Motor) -> DualNum {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2) + pow(other.g0_.w, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: DualNum = DualNum(/* scalar, e1234 */ (vec2<f32>(anti_inverse.g0_) * self_.g0_));
    return geometric_anti_product;
}
fn dualNum_geometricAntiProduct_multiVector(self: DualNum, other: MultiVector) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(((self_.g0_.x * other.g0_.y) + (self_.g0_.y * other.g0_.x)), (self_.g0_.y * other.g0_.y)), /* e1, e2, e3, e4 */ vec4<f32>(((self_.g0_.x * other.g4_.x) + (self_.g0_.y * other.g1_.x)), ((self_.g0_.x * other.g4_.y) + (self_.g0_.y * other.g1_.y)), ((self_.g0_.x * other.g4_.z) + (self_.g0_.y * other.g1_.z)), (self_.g0_.y * other.g1_.w)), /* e41, e42, e43 */ vec3<f32>((self_.g0_.y * other.g2_.x), (self_.g0_.y * other.g2_.y), (self_.g0_.y * other.g2_.z)), /* e23, e31, e12 */ vec3<f32>(((self_.g0_.x * other.g2_.x) + (self_.g0_.y * other.g3_.x)), ((self_.g0_.x * other.g2_.y) + (self_.g0_.y * other.g3_.y)), ((self_.g0_.x * other.g2_.z) + (self_.g0_.y * other.g3_.z))), /* e423, e431, e412, e321 */ vec4<f32>((self_.g0_.y * other.g4_.x), (self_.g0_.y * other.g4_.y), (self_.g0_.y * other.g4_.z), ((self_.g0_.x * other.g1_.w) + (self_.g0_.y * other.g4_.w))));
}
fn horizon_geometricQuotient_plane(self: Horizon, other: Plane) -> Horizon {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_.w, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Horizon = Horizon(/* e321 */ (self_.g0_ * inverse.g0_));
    return geometric_product;
}
fn flector_sandwich_origin(self: Flector, other: Origin) -> Flector {
    let geometric_product: Motor = Motor(/* e41, e42, e43, e1234 */ (vec4<f32>(other.g0_) * vec4<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z, self_.g1_.w) * vec4<f32>(-1.0)), /* e23, e31, e12, scalar */ vec4<f32>(0.0));
    return motor_geometricProduct_motor(geometric_product, flector_reverse(self_));
}
fn line_geometricQuotient_horizon(self: Line, other: Horizon) -> Line {
    let scalar_product: Scalar = Scalar(/* scalar */ (pow(other.g0_, 2) * -1.0));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Line = Line(/* e41, e42, e43 */ (vec3<f32>(inverse.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(inverse.g0_) * self_.g1_));
    return geometric_product;
}
fn point_bitXor_plane(self: Point, other: Plane) -> AntiScalar {
    return point_wedge_point(self_, other);
}
fn horizon_geometricAntiQuotient_point(self: Horizon, other: Point) -> Horizon {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_.w, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Horizon = Horizon(/* e321 */ (anti_inverse.g0_ * self_.g0_));
    return geometric_anti_product;
}
fn multiVector_geometricProduct_origin(self: MultiVector, other: Origin) -> MultiVector {
    return MultiVector(/* scalar, e1234 */ vec2<f32>(0.0, (self_.g4_.w * other.g0_ * -1.0)), /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.x * other.g0_)), /* e41, e42, e43 */ vec3<f32>((self_.g1_.x * other.g0_ * -1.0), (self_.g1_.y * other.g0_ * -1.0), (self_.g1_.z * other.g0_ * -1.0)), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>((self_.g3_.x * other.g0_), (self_.g3_.y * other.g0_), (self_.g3_.z * other.g0_), 0.0));
}
fn dualNum_add_point(self: DualNum, other: Point) -> MultiVector {
    let addition: MultiVector = MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ other.g0_, /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    return addition;
}
fn line_geometricAntiQuotient_origin(self: Line, other: Origin) -> Line {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Line = Line(/* e41, e42, e43 */ (vec3<f32>(anti_inverse.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(anti_inverse.g0_) * self_.g1_));
    return geometric_anti_product;
}
fn multiVector_mul_multiVector(self: MultiVector, other: MultiVector) -> MultiVector {
    return multiVector_geometricProduct_multiVector(self_, other);
}
fn plane_geometricAntiQuotient_motor(self: Plane, other: Motor) -> Plane {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2) + pow(other.g0_.w, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Plane = Plane(/* e423, e431, e412, e321 */ (vec4<f32>(anti_inverse.g0_) * self_.g0_));
    return geometric_anti_product;
}
fn point_geometricQuotient_line(self: Point, other: Line) -> Point {
    let scalar_product: Scalar = Scalar(/* scalar */ (-pow(other.g1_.x, 2) - pow(other.g1_.y, 2) - pow(other.g1_.z, 2)));
    let inverse: Scalar = Scalar(/* scalar */ (1.0/scalar_product.g0_.x));
    let geometric_product: Point = Point(/* e1, e2, e3, e4 */ (vec4<f32>(inverse.g0_) * self_.g0_));
    return geometric_product;
}
fn motor_antiSandwich_scalar(self: Motor, other: Scalar) -> Motor {
    let geometric_anti_product: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>(0.0), /* e23, e31, e12, scalar */ (vec4<f32>(other.g0_) * self_.g0_));
    return motor_geometricAntiProduct_motor(geometric_anti_product, motor_antiReverse(self_));
}
fn flector_sub_dualNum(self: Flector, other: DualNum) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ (other.g0_ * vec2<f32>(-1.0)), /* e1, e2, e3, e4 */ self_.g0_, /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ self_.g1_);
    return subtraction;
}
fn multiVector_sub_flector(self: MultiVector, other: Flector) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ self_.g0_, /* e1, e2, e3, e4 */ (-other.g0_ + self_.g1_), /* e41, e42, e43 */ self_.g2_, /* e23, e31, e12 */ self_.g3_, /* e423, e431, e412, e321 */ (-other.g1_ + self_.g4_));
    return subtraction;
}
fn origin_geometricAntiQuotient_motor(self: Origin, other: Motor) -> Origin {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (-pow(other.g0_.x, 2) - pow(other.g0_.y, 2) - pow(other.g0_.z, 2) + pow(other.g0_.w, 2)));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Origin = Origin(/* e4 */ (anti_inverse.g0_ * self_.g0_));
    return geometric_anti_product;
}
fn motor_geometricProduct_antiScalar(self: Motor, other: AntiScalar) -> Motor {
    return Motor(/* e41, e42, e43, e1234 */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), (self_.g1_.w * other.g0_)), /* e23, e31, e12, scalar */ vec4<f32>(0.0));
}
fn motor_sub_point(self: Motor, other: Point) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g1_.w, self_.g0_.w), /* e1, e2, e3, e4 */ (other.g0_ * vec4<f32>(-1.0)), /* e41, e42, e43 */ vec3<f32>(self_.g0_.x, self_.g0_.y, self_.g0_.z), /* e23, e31, e12 */ vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z), /* e423, e431, e412, e321 */ vec4<f32>(0.0));
    return subtraction;
}
fn flector_wedge_scalar(self: Flector, other: Scalar) -> Flector {
    return Flector(/* e1, e2, e3, e4 */ vec4<f32>((self_.g0_.x * other.g0_), (self_.g0_.y * other.g0_), (self_.g0_.z * other.g0_), (self_.g0_.w * other.g0_)), /* e423, e431, e412, e321 */ vec4<f32>((self_.g1_.x * other.g0_), (self_.g1_.y * other.g0_), (self_.g1_.z * other.g0_), (self_.g1_.w * other.g0_)));
}
fn flector_add_horizon(self: Flector, other: Horizon) -> Flector {
    let addition: Flector = Flector(/* e1, e2, e3, e4 */ self_.g0_, /* e423, e431, e412, e321 */ vec4<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z, (self_.g1_.w + other.g0_)));
    return addition;
}
fn origin_mul_flector(self: Origin, other: Flector) -> Motor {
    return origin_geometricProduct_origin(self_, other);
}
fn scalar_bitXor_multiVector(self: Scalar, other: MultiVector) -> MultiVector {
    return scalar_wedge_scalar(self_, other);
}
fn dualNum_bitXor_line(self: DualNum, other: Line) -> Line {
    return dualNum_wedge_dualNum(self_, other);
}
fn point_sub_plane(self: Point, other: Plane) -> Flector {
    let subtraction: Flector = Flector(/* e1, e2, e3, e4 */ self_.g0_, /* e423, e431, e412, e321 */ (other.g0_ * vec4<f32>(-1.0)));
    return subtraction;
}
fn scalar_add_line(self: Scalar, other: Line) -> Motor {
    let addition: Motor = Motor(/* e41, e42, e43, e1234 */ vec4<f32>(other.g0_.x, other.g0_.y, other.g0_.z, 0.0), /* e23, e31, e12, scalar */ vec4<f32>(other.g1_.x, other.g1_.y, other.g1_.z, self_.g0_));
    return addition;
}
fn dualNum_antiSandwich_point(self: DualNum, other: Point) -> Flector {
    let geometric_anti_product: Flector = Flector(/* e1, e2, e3, e4 */ (vec4<f32>(self_.g0_.y) * other.g0_), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (self_.g0_.x * other.g0_.w)));
    return flector_geometricAntiProduct_flector(geometric_anti_product, dualNum_antiReverse(self_));
}
fn line_geometricAntiQuotient_antiScalar(self: Line, other: AntiScalar) -> Line {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ pow(other.g0_, 2));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Line = Line(/* e41, e42, e43 */ (vec3<f32>(anti_inverse.g0_) * self_.g0_), /* e23, e31, e12 */ (vec3<f32>(anti_inverse.g0_) * self_.g1_));
    return geometric_anti_product;
}
fn dualNum_antiSandwich_scalar(self: DualNum, other: Scalar) -> Scalar {
    let geometric_anti_product: Scalar = Scalar(/* scalar */ (self_.g0_.y * other.g0_));
    return scalar_geometricAntiProduct_scalar(geometric_anti_product, dualNum_antiReverse(self_));
}
fn multiVector_antiSandwich_multiVector(self: MultiVector, other: MultiVector) -> MultiVector {
    let geometric_anti_product: MultiVector = MultiVector(/* scalar, e1234 */ (vec2<f32>(((other.g0_.y * self_.g0_.x) - (other.g3_.x * self_.g2_.x) - (other.g3_.y * self_.g2_.y) - (other.g3_.z * self_.g2_.z) - (other.g1_.x * self_.g4_.x) - (other.g1_.y * self_.g4_.y) - (other.g1_.z * self_.g4_.z) + (other.g4_.w * self_.g1_.w)), 0.0) + (vec2<f32>(self_.g0_.y) * other.g0_) - (vec2<f32>(other.g2_.x) * vec2<f32>(self_.g3_.x, self_.g2_.x)) - (vec2<f32>(other.g2_.y) * vec2<f32>(self_.g3_.y, self_.g2_.y)) - (vec2<f32>(other.g2_.z) * vec2<f32>(self_.g3_.z, self_.g2_.z)) - (vec2<f32>(other.g1_.w) * vec2<f32>(self_.g4_.w, self_.g1_.w)) + (vec2<f32>(other.g4_.x) * vec2<f32>(self_.g1_.x, self_.g4_.x)) + (vec2<f32>(other.g4_.y) * vec2<f32>(self_.g1_.y, self_.g4_.y)) + (vec2<f32>(other.g4_.z) * vec2<f32>(self_.g1_.z, self_.g4_.z))), /* e1, e2, e3, e4 */ (vec4<f32>(((self_.g0_.x * other.g4_.x) + (other.g2_.x * self_.g4_.w) - (other.g2_.y * self_.g1_.z) + (other.g2_.z * self_.g1_.y) - (other.g3_.x * self_.g1_.w) + (other.g3_.y * self_.g4_.z) + (self_.g2_.x * other.g4_.w) + (self_.g2_.y * other.g1_.z) - (self_.g2_.z * other.g1_.y) + (self_.g3_.x * other.g1_.w) + (self_.g3_.y * other.g4_.z)), ((self_.g0_.x * other.g4_.y) + (other.g2_.x * self_.g1_.z) + (other.g2_.y * self_.g4_.w) - (other.g2_.z * self_.g1_.x) - (other.g3_.y * self_.g1_.w) + (other.g3_.z * self_.g4_.x) - (self_.g2_.x * other.g1_.z) + (self_.g2_.y * other.g4_.w) + (self_.g2_.z * other.g1_.x) + (self_.g3_.y * other.g1_.w) + (self_.g3_.z * other.g4_.x)), ((self_.g0_.x * other.g4_.z) - (other.g2_.x * self_.g1_.y) + (other.g2_.y * self_.g1_.x) + (other.g2_.z * self_.g4_.w) + (other.g3_.x * self_.g4_.y) - (other.g3_.z * self_.g1_.w) + (self_.g2_.x * other.g1_.y) - (self_.g2_.y * other.g1_.x) + (self_.g2_.z * other.g4_.w) + (self_.g3_.x * other.g4_.y) + (self_.g3_.z * other.g1_.w)), (-(other.g2_.z * self_.g4_.z) - (self_.g2_.y * other.g4_.y) - (self_.g2_.z * other.g4_.z))) + (vec4<f32>(other.g0_.y) * self_.g1_) + (vec4<f32>(self_.g0_.y) * other.g1_) - (vec4<f32>(other.g0_.x, other.g0_.x, other.g0_.x, other.g2_.x) * self_.g4_.xyzx) - (vec4<f32>(other.g3_.z, other.g3_.x, other.g3_.y, other.g2_.y) * self_.g4_.yzxy) - (vec4<f32>(self_.g3_.z, self_.g3_.x, self_.g3_.y, self_.g2_.x) * other.g4_.yzxx)), /* e41, e42, e43 */ (vec3<f32>(((other.g4_.y * self_.g4_.z) - (other.g4_.z * self_.g4_.y)), (-(other.g4_.x * self_.g4_.z) + (other.g4_.z * self_.g4_.x)), ((other.g4_.x * self_.g4_.y) - (other.g4_.y * self_.g4_.x))) + (vec3<f32>(other.g0_.y) * self_.g2_) + (vec3<f32>(self_.g0_.y) * other.g2_) - (vec3<f32>(other.g1_.w) * vec3<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z)) - (vec3<f32>(self_.g1_.w) * vec3<f32>(other.g4_.x, other.g4_.y, other.g4_.z)) - (other.g2_.yzx * self_.g2_.zxy) + (other.g2_.zxy * self_.g2_.yzx)), /* e23, e31, e12 */ (vec3<f32>((-(other.g1_.y * self_.g4_.z) + (other.g1_.z * self_.g4_.y) + (other.g4_.y * self_.g1_.z) - (other.g4_.z * self_.g1_.y)), ((other.g1_.x * self_.g4_.z) - (other.g1_.z * self_.g4_.x) - (other.g4_.x * self_.g1_.z) + (other.g4_.z * self_.g1_.x)), (-(other.g1_.x * self_.g4_.y) + (other.g1_.y * self_.g4_.x) + (other.g4_.x * self_.g1_.y) - (other.g4_.y * self_.g1_.x))) + (vec3<f32>(other.g0_.x) * self_.g2_) + (vec3<f32>(other.g0_.y) * self_.g3_) + (vec3<f32>(self_.g0_.x) * other.g2_) + (vec3<f32>(self_.g0_.y) * other.g3_) - (vec3<f32>(other.g1_.w) * vec3<f32>(self_.g1_.x, self_.g1_.y, self_.g1_.z)) + (vec3<f32>(other.g4_.w) * vec3<f32>(self_.g4_.x, self_.g4_.y, self_.g4_.z)) + (vec3<f32>(self_.g1_.w) * vec3<f32>(other.g1_.x, other.g1_.y, other.g1_.z)) - (vec3<f32>(self_.g4_.w) * vec3<f32>(other.g4_.x, other.g4_.y, other.g4_.z)) - (other.g2_.yzx * self_.g3_.zxy) + (other.g2_.zxy * self_.g3_.yzx) - (other.g3_.yzx * self_.g2_.zxy) + (other.g3_.zxy * self_.g2_.yzx)), /* e423, e431, e412, e321 */ (vec4<f32>(((other.g2_.x * self_.g1_.w) - (other.g2_.y * self_.g4_.z) + (self_.g2_.y * other.g4_.z)), ((other.g2_.y * self_.g1_.w) - (other.g2_.z * self_.g4_.x) + (self_.g2_.z * other.g4_.x)), (-(other.g2_.x * self_.g4_.y) + (other.g2_.z * self_.g1_.w) + (self_.g2_.x * other.g4_.y)), (-(other.g0_.x * self_.g1_.w) - (other.g2_.x * self_.g1_.x) - (other.g2_.y * self_.g1_.y) - (other.g2_.z * self_.g1_.z) + (other.g3_.y * self_.g4_.y) + (other.g3_.z * self_.g4_.z) - (self_.g2_.x * other.g1_.x) - (self_.g2_.y * other.g1_.y) - (self_.g2_.z * other.g1_.z) - (self_.g3_.y * other.g4_.y) - (self_.g3_.z * other.g4_.z))) + (vec4<f32>(other.g0_.y) * self_.g4_) + (vec4<f32>(self_.g0_.y) * other.g4_) + (vec4<f32>(other.g1_.w) * vec4<f32>(self_.g2_.x, self_.g2_.y, self_.g2_.z, self_.g0_.x)) + (vec4<f32>(other.g2_.z, other.g2_.x, other.g2_.y, other.g3_.x) * self_.g4_.yzxx) - (vec4<f32>(self_.g2_.z, self_.g2_.x, self_.g2_.y, self_.g3_.x) * other.g4_.yzxx)));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self_));
}
fn scalar_antiWedge_dualNum(self: Scalar, other: DualNum) -> Scalar {
    return Scalar(/* scalar */ (other.g0_.y * self_.g0_));
}
fn scalar_sub_horizon(self: Scalar, other: Horizon) -> MultiVector {
    let subtraction: MultiVector = MultiVector(/* scalar, e1234 */ vec2<f32>(self_.g0_, 0.0), /* e1, e2, e3, e4 */ vec4<f32>(0.0), /* e41, e42, e43 */ vec3<f32>(0.0), /* e23, e31, e12 */ vec3<f32>(0.0), /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (other.g0_ * -1.0)));
    return subtraction;
}
fn origin_geometricAntiQuotient_origin(self: Origin, other: Origin) -> Origin {
    let anti_scalar_product: AntiScalar = AntiScalar(/* e1234 */ (pow(other.g0_, 2) * -1.0));
    let anti_inverse: AntiScalar = AntiScalar(/* e1234 */ (1.0/anti_scalar_product.g0_.x));
    let geometric_anti_product: Origin = Origin(/* e4 */ (anti_inverse.g0_ * self_.g0_));
    return geometric_anti_product;
}
fn antiScalar_wedge_dualNum(self: AntiScalar, other: DualNum) -> AntiScalar {
    return AntiScalar(/* e1234 */ (other.g0_.x * self_.g0_));
}
fn scalar_bitXor_flector(self: Scalar, other: Flector) -> Flector {
    return scalar_wedge_scalar(self_, other);
}

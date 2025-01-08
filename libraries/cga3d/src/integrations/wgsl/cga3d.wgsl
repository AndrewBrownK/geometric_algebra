#define_import_path cga3d
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

struct AntiCircleRotor {
    e41_: f32, e42_: f32, e43_: f32,
    e23_: f32, e31_: f32, e12_: f32, e45_: f32,
    e15_: f32, e25_: f32, e35_: f32, scalar: f32
}
struct AntiCircleRotorGroups {
    // e41, e42, e43, 0
    group0_: vec4<f32>,
    // e23, e31, e12, e45
    group1_: vec4<f32>,
    // e15, e25, e35, scalar
    group2_: vec4<f32>
}
fn antiCircleRotor_grouped(self_: AntiCircleRotor) -> AntiCircleRotorGroups {
    return AntiCircleRotorGroups(
        vec4<f32>(self_.e41_, self_.e42_, self_.e43_, 0.0),
        vec4<f32>(self_.e23_, self_.e31_, self_.e12_, self_.e45_),
        vec4<f32>(self_.e15_, self_.e25_, self_.e35_, self_.scalar)
    );
}
fn antiCircleRotor_degroup(self_: AntiCircleRotorGroups) -> AntiCircleRotor {
    return AntiCircleRotor(
        self_.group0_.x, self_.group0_.y, self_.group0_.z,
        self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group1_.w,
        self_.group2_.x, self_.group2_.y, self_.group2_.z, self_.group2_.w
    );
}


struct AntiDipoleInversion {
    e423_: f32, e431_: f32, e412_: f32,
    e415_: f32, e425_: f32, e435_: f32, e321_: f32,
    e235_: f32, e315_: f32, e125_: f32, e4_: f32,
    e1_: f32, e2_: f32, e3_: f32, e5_: f32
}
struct AntiDipoleInversionGroups {
    // e423, e431, e412, 0
    group0_: vec4<f32>,
    // e415, e425, e435, e321
    group1_: vec4<f32>,
    // e235, e315, e125, e4
    group2_: vec4<f32>,
    // e1, e2, e3, e5
    group3_: vec4<f32>
}
fn antiDipoleInversion_grouped(self_: AntiDipoleInversion) -> AntiDipoleInversionGroups {
    return AntiDipoleInversionGroups(
        vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0),
        vec4<f32>(self_.e415_, self_.e425_, self_.e435_, self_.e321_),
        vec4<f32>(self_.e235_, self_.e315_, self_.e125_, self_.e4_),
        vec4<f32>(self_.e1_, self_.e2_, self_.e3_, self_.e5_)
    );
}
fn antiDipoleInversion_degroup(self_: AntiDipoleInversionGroups) -> AntiDipoleInversion {
    return AntiDipoleInversion(
        self_.group0_.x, self_.group0_.y, self_.group0_.z,
        self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group1_.w,
        self_.group2_.x, self_.group2_.y, self_.group2_.z, self_.group2_.w,
        self_.group3_.x, self_.group3_.y, self_.group3_.z, self_.group3_.w
    );
}


struct AntiDualNum {
    e3215_: f32, scalar: f32
}
struct AntiDualNumGroups {
    // e3215, scalar, 0, 0
    group0_: vec4<f32>
}
fn antiDualNum_grouped(self_: AntiDualNum) -> AntiDualNumGroups {
    return AntiDualNumGroups(
        vec4<f32>(self_.e3215_, self_.scalar, 0.0, 0.0)
    );
}
fn antiDualNum_degroup(self_: AntiDualNumGroups) -> AntiDualNum {
    return AntiDualNum(
        self_.group0_.x, self_.group0_.y
    );
}


struct AntiFlatPoint {
    e235_: f32, e315_: f32, e125_: f32, e321_: f32
}
struct AntiFlatPointGroups {
    // e235, e315, e125, e321
    group0_: vec4<f32>
}
fn antiFlatPoint_grouped(self_: AntiFlatPoint) -> AntiFlatPointGroups {
    return AntiFlatPointGroups(
        vec4<f32>(self_.e235_, self_.e315_, self_.e125_, self_.e321_)
    );
}
fn antiFlatPoint_degroup(self_: AntiFlatPointGroups) -> AntiFlatPoint {
    return AntiFlatPoint(
        self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group0_.w
    );
}


struct AntiFlector {
    e235_: f32, e315_: f32, e125_: f32, e321_: f32,
    e1_: f32, e2_: f32, e3_: f32, e5_: f32
}
struct AntiFlectorGroups {
    // e235, e315, e125, e321
    group0_: vec4<f32>,
    // e1, e2, e3, e5
    group1_: vec4<f32>
}
fn antiFlector_grouped(self_: AntiFlector) -> AntiFlectorGroups {
    return AntiFlectorGroups(
        vec4<f32>(self_.e235_, self_.e315_, self_.e125_, self_.e321_),
        vec4<f32>(self_.e1_, self_.e2_, self_.e3_, self_.e5_)
    );
}
fn antiFlector_degroup(self_: AntiFlectorGroups) -> AntiFlector {
    return AntiFlector(
        self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group0_.w,
        self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group1_.w
    );
}


struct AntiLine {
    e23_: f32, e31_: f32, e12_: f32,
    e15_: f32, e25_: f32, e35_: f32
}
struct AntiLineGroups {
    // e23, e31, e12, 0
    group0_: vec4<f32>,
    // e15, e25, e35, 0
    group1_: vec4<f32>
}
fn antiLine_grouped(self_: AntiLine) -> AntiLineGroups {
    return AntiLineGroups(
        vec4<f32>(self_.e23_, self_.e31_, self_.e12_, 0.0),
        vec4<f32>(self_.e15_, self_.e25_, self_.e35_, 0.0)
    );
}
fn antiLine_degroup(self_: AntiLineGroups) -> AntiLine {
    return AntiLine(
        self_.group0_.x, self_.group0_.y, self_.group0_.z,
        self_.group1_.x, self_.group1_.y, self_.group1_.z
    );
}


struct AntiMotor {
    e23_: f32, e31_: f32, e12_: f32, scalar: f32,
    e15_: f32, e25_: f32, e35_: f32, e3215_: f32
}
struct AntiMotorGroups {
    // e23, e31, e12, scalar
    group0_: vec4<f32>,
    // e15, e25, e35, e3215
    group1_: vec4<f32>
}
fn antiMotor_grouped(self_: AntiMotor) -> AntiMotorGroups {
    return AntiMotorGroups(
        vec4<f32>(self_.e23_, self_.e31_, self_.e12_, self_.scalar),
        vec4<f32>(self_.e15_, self_.e25_, self_.e35_, self_.e3215_)
    );
}
fn antiMotor_degroup(self_: AntiMotorGroups) -> AntiMotor {
    return AntiMotor(
        self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group0_.w,
        self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group1_.w
    );
}


struct AntiPlane {
    e1_: f32, e2_: f32, e3_: f32, e5_: f32
}
struct AntiPlaneGroups {
    // e1, e2, e3, e5
    group0_: vec4<f32>
}
fn antiPlane_grouped(self_: AntiPlane) -> AntiPlaneGroups {
    return AntiPlaneGroups(
        vec4<f32>(self_.e1_, self_.e2_, self_.e3_, self_.e5_)
    );
}
fn antiPlane_degroup(self_: AntiPlaneGroups) -> AntiPlane {
    return AntiPlane(
        self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group0_.w
    );
}


struct AntiScalar {
    e12345_: f32
}
struct AntiScalarGroups {
    // e12345, 0, 0, 0
    group0_: vec4<f32>
}
fn antiScalar_grouped(self_: AntiScalar) -> AntiScalarGroups {
    return AntiScalarGroups(
        vec4<f32>(self_.e12345_, 0.0, 0.0, 0.0)
    );
}
fn antiScalar_degroup(self_: AntiScalarGroups) -> AntiScalar {
    return AntiScalar(
        self_.group0_.x
    );
}


struct Circle {
    e423_: f32, e431_: f32, e412_: f32,
    e415_: f32, e425_: f32, e435_: f32, e321_: f32,
    e235_: f32, e315_: f32, e125_: f32
}
struct CircleGroups {
    // e423, e431, e412, 0
    group0_: vec4<f32>,
    // e415, e425, e435, e321
    group1_: vec4<f32>,
    // e235, e315, e125, 0
    group2_: vec4<f32>
}
fn circle_grouped(self_: Circle) -> CircleGroups {
    return CircleGroups(
        vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0),
        vec4<f32>(self_.e415_, self_.e425_, self_.e435_, self_.e321_),
        vec4<f32>(self_.e235_, self_.e315_, self_.e125_, 0.0)
    );
}
fn circle_degroup(self_: CircleGroups) -> Circle {
    return Circle(
        self_.group0_.x, self_.group0_.y, self_.group0_.z,
        self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group1_.w,
        self_.group2_.x, self_.group2_.y, self_.group2_.z
    );
}


struct CircleRotor {
    e423_: f32, e431_: f32, e412_: f32,
    e415_: f32, e425_: f32, e435_: f32, e321_: f32,
    e235_: f32, e315_: f32, e125_: f32, e12345_: f32
}
struct CircleRotorGroups {
    // e423, e431, e412, 0
    group0_: vec4<f32>,
    // e415, e425, e435, e321
    group1_: vec4<f32>,
    // e235, e315, e125, e12345
    group2_: vec4<f32>
}
fn circleRotor_grouped(self_: CircleRotor) -> CircleRotorGroups {
    return CircleRotorGroups(
        vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0),
        vec4<f32>(self_.e415_, self_.e425_, self_.e435_, self_.e321_),
        vec4<f32>(self_.e235_, self_.e315_, self_.e125_, self_.e12345_)
    );
}
fn circleRotor_degroup(self_: CircleRotorGroups) -> CircleRotor {
    return CircleRotor(
        self_.group0_.x, self_.group0_.y, self_.group0_.z,
        self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group1_.w,
        self_.group2_.x, self_.group2_.y, self_.group2_.z, self_.group2_.w
    );
}


struct Dipole {
    e41_: f32, e42_: f32, e43_: f32,
    e23_: f32, e31_: f32, e12_: f32, e45_: f32,
    e15_: f32, e25_: f32, e35_: f32
}
struct DipoleGroups {
    // e41, e42, e43, 0
    group0_: vec4<f32>,
    // e23, e31, e12, e45
    group1_: vec4<f32>,
    // e15, e25, e35, 0
    group2_: vec4<f32>
}
fn dipole_grouped(self_: Dipole) -> DipoleGroups {
    return DipoleGroups(
        vec4<f32>(self_.e41_, self_.e42_, self_.e43_, 0.0),
        vec4<f32>(self_.e23_, self_.e31_, self_.e12_, self_.e45_),
        vec4<f32>(self_.e15_, self_.e25_, self_.e35_, 0.0)
    );
}
fn dipole_degroup(self_: DipoleGroups) -> Dipole {
    return Dipole(
        self_.group0_.x, self_.group0_.y, self_.group0_.z,
        self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group1_.w,
        self_.group2_.x, self_.group2_.y, self_.group2_.z
    );
}


struct DipoleInversion {
    e41_: f32, e42_: f32, e43_: f32,
    e23_: f32, e31_: f32, e12_: f32, e45_: f32,
    e15_: f32, e25_: f32, e35_: f32, e1234_: f32,
    e4235_: f32, e4315_: f32, e4125_: f32, e3215_: f32
}
struct DipoleInversionGroups {
    // e41, e42, e43, 0
    group0_: vec4<f32>,
    // e23, e31, e12, e45
    group1_: vec4<f32>,
    // e15, e25, e35, e1234
    group2_: vec4<f32>,
    // e4235, e4315, e4125, e3215
    group3_: vec4<f32>
}
fn dipoleInversion_grouped(self_: DipoleInversion) -> DipoleInversionGroups {
    return DipoleInversionGroups(
        vec4<f32>(self_.e41_, self_.e42_, self_.e43_, 0.0),
        vec4<f32>(self_.e23_, self_.e31_, self_.e12_, self_.e45_),
        vec4<f32>(self_.e15_, self_.e25_, self_.e35_, self_.e1234_),
        vec4<f32>(self_.e4235_, self_.e4315_, self_.e4125_, self_.e3215_)
    );
}
fn dipoleInversion_degroup(self_: DipoleInversionGroups) -> DipoleInversion {
    return DipoleInversion(
        self_.group0_.x, self_.group0_.y, self_.group0_.z,
        self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group1_.w,
        self_.group2_.x, self_.group2_.y, self_.group2_.z, self_.group2_.w,
        self_.group3_.x, self_.group3_.y, self_.group3_.z, self_.group3_.w
    );
}


struct DualNum {
    e5_: f32, e12345_: f32
}
struct DualNumGroups {
    // e5, e12345, 0, 0
    group0_: vec4<f32>
}
fn dualNum_grouped(self_: DualNum) -> DualNumGroups {
    return DualNumGroups(
        vec4<f32>(self_.e5_, self_.e12345_, 0.0, 0.0)
    );
}
fn dualNum_degroup(self_: DualNumGroups) -> DualNum {
    return DualNum(
        self_.group0_.x, self_.group0_.y
    );
}


struct FlatPoint {
    e15_: f32, e25_: f32, e35_: f32, e45_: f32
}
struct FlatPointGroups {
    // e15, e25, e35, e45
    group0_: vec4<f32>
}
fn flatPoint_grouped(self_: FlatPoint) -> FlatPointGroups {
    return FlatPointGroups(
        vec4<f32>(self_.e15_, self_.e25_, self_.e35_, self_.e45_)
    );
}
fn flatPoint_degroup(self_: FlatPointGroups) -> FlatPoint {
    return FlatPoint(
        self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group0_.w
    );
}


struct Flector {
    e15_: f32, e25_: f32, e35_: f32, e45_: f32,
    e4235_: f32, e4315_: f32, e4125_: f32, e3215_: f32
}
struct FlectorGroups {
    // e15, e25, e35, e45
    group0_: vec4<f32>,
    // e4235, e4315, e4125, e3215
    group1_: vec4<f32>
}
fn flector_grouped(self_: Flector) -> FlectorGroups {
    return FlectorGroups(
        vec4<f32>(self_.e15_, self_.e25_, self_.e35_, self_.e45_),
        vec4<f32>(self_.e4235_, self_.e4315_, self_.e4125_, self_.e3215_)
    );
}
fn flector_degroup(self_: FlectorGroups) -> Flector {
    return Flector(
        self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group0_.w,
        self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group1_.w
    );
}


struct Line {
    e415_: f32, e425_: f32, e435_: f32,
    e235_: f32, e315_: f32, e125_: f32
}
struct LineGroups {
    // e415, e425, e435, 0
    group0_: vec4<f32>,
    // e235, e315, e125, 0
    group1_: vec4<f32>
}
fn line_grouped(self_: Line) -> LineGroups {
    return LineGroups(
        vec4<f32>(self_.e415_, self_.e425_, self_.e435_, 0.0),
        vec4<f32>(self_.e235_, self_.e315_, self_.e125_, 0.0)
    );
}
fn line_degroup(self_: LineGroups) -> Line {
    return Line(
        self_.group0_.x, self_.group0_.y, self_.group0_.z,
        self_.group1_.x, self_.group1_.y, self_.group1_.z
    );
}


struct Motor {
    e415_: f32, e425_: f32, e435_: f32, e12345_: f32,
    e235_: f32, e315_: f32, e125_: f32, e5_: f32
}
struct MotorGroups {
    // e415, e425, e435, e12345
    group0_: vec4<f32>,
    // e235, e315, e125, e5
    group1_: vec4<f32>
}
fn motor_grouped(self_: Motor) -> MotorGroups {
    return MotorGroups(
        vec4<f32>(self_.e415_, self_.e425_, self_.e435_, self_.e12345_),
        vec4<f32>(self_.e235_, self_.e315_, self_.e125_, self_.e5_)
    );
}
fn motor_degroup(self_: MotorGroups) -> Motor {
    return Motor(
        self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group0_.w,
        self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group1_.w
    );
}


struct MultiVector {
    scalar: f32, e12345_: f32,
    e1_: f32, e2_: f32, e3_: f32, e4_: f32,
    e5_: f32,
    e15_: f32, e25_: f32, e35_: f32, e45_: f32,
    e41_: f32, e42_: f32, e43_: f32,
    e23_: f32, e31_: f32, e12_: f32,
    e415_: f32, e425_: f32, e435_: f32, e321_: f32,
    e423_: f32, e431_: f32, e412_: f32,
    e235_: f32, e315_: f32, e125_: f32,
    e4235_: f32, e4315_: f32, e4125_: f32, e3215_: f32,
    e1234_: f32
}
struct MultiVectorGroups {
    // scalar, e12345, 0, 0
    group0_: vec4<f32>,
    // e1, e2, e3, e4
    group1_: vec4<f32>,
    // e5, 0, 0, 0
    group2_: vec4<f32>,
    // e15, e25, e35, e45
    group3_: vec4<f32>,
    // e41, e42, e43, 0
    group4_: vec4<f32>,
    // e23, e31, e12, 0
    group5_: vec4<f32>,
    // e415, e425, e435, e321
    group6_: vec4<f32>,
    // e423, e431, e412, 0
    group7_: vec4<f32>,
    // e235, e315, e125, 0
    group8_: vec4<f32>,
    // e4235, e4315, e4125, e3215
    group9_: vec4<f32>,
    // e1234, 0, 0, 0
    group10_: vec4<f32>
}
fn multiVector_grouped(self_: MultiVector) -> MultiVectorGroups {
    return MultiVectorGroups(
        vec4<f32>(self_.scalar, self_.e12345_, 0.0, 0.0),
        vec4<f32>(self_.e1_, self_.e2_, self_.e3_, self_.e4_),
        vec4<f32>(self_.e5_, 0.0, 0.0, 0.0),
        vec4<f32>(self_.e15_, self_.e25_, self_.e35_, self_.e45_),
        vec4<f32>(self_.e41_, self_.e42_, self_.e43_, 0.0),
        vec4<f32>(self_.e23_, self_.e31_, self_.e12_, 0.0),
        vec4<f32>(self_.e415_, self_.e425_, self_.e435_, self_.e321_),
        vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0),
        vec4<f32>(self_.e235_, self_.e315_, self_.e125_, 0.0),
        vec4<f32>(self_.e4235_, self_.e4315_, self_.e4125_, self_.e3215_),
        vec4<f32>(self_.e1234_, 0.0, 0.0, 0.0)
    );
}
fn multiVector_degroup(self_: MultiVectorGroups) -> MultiVector {
    return MultiVector(
        self_.group0_.x, self_.group0_.y,
        self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group1_.w,
        self_.group2_.x,
        self_.group3_.x, self_.group3_.y, self_.group3_.z, self_.group3_.w,
        self_.group4_.x, self_.group4_.y, self_.group4_.z,
        self_.group5_.x, self_.group5_.y, self_.group5_.z,
        self_.group6_.x, self_.group6_.y, self_.group6_.z, self_.group6_.w,
        self_.group7_.x, self_.group7_.y, self_.group7_.z,
        self_.group8_.x, self_.group8_.y, self_.group8_.z,
        self_.group9_.x, self_.group9_.y, self_.group9_.z, self_.group9_.w,
        self_.group10_.x
    );
}


struct Plane {
    e4235_: f32, e4315_: f32, e4125_: f32, e3215_: f32
}
struct PlaneGroups {
    // e4235, e4315, e4125, e3215
    group0_: vec4<f32>
}
fn plane_grouped(self_: Plane) -> PlaneGroups {
    return PlaneGroups(
        vec4<f32>(self_.e4235_, self_.e4315_, self_.e4125_, self_.e3215_)
    );
}
fn plane_degroup(self_: PlaneGroups) -> Plane {
    return Plane(
        self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group0_.w
    );
}


struct RoundPoint {
    e1_: f32, e2_: f32, e3_: f32, e4_: f32,
    e5_: f32
}
struct RoundPointGroups {
    // e1, e2, e3, e4
    group0_: vec4<f32>,
    // e5, 0, 0, 0
    group1_: vec4<f32>
}
fn roundPoint_grouped(self_: RoundPoint) -> RoundPointGroups {
    return RoundPointGroups(
        vec4<f32>(self_.e1_, self_.e2_, self_.e3_, self_.e4_),
        vec4<f32>(self_.e5_, 0.0, 0.0, 0.0)
    );
}
fn roundPoint_degroup(self_: RoundPointGroups) -> RoundPoint {
    return RoundPoint(
        self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group0_.w,
        self_.group1_.x
    );
}


struct Scalar {
    scalar: f32
}
struct ScalarGroups {
    // scalar, 0, 0, 0
    group0_: vec4<f32>
}
fn scalar_grouped(self_: Scalar) -> ScalarGroups {
    return ScalarGroups(
        vec4<f32>(self_.scalar, 0.0, 0.0, 0.0)
    );
}
fn scalar_degroup(self_: ScalarGroups) -> Scalar {
    return Scalar(
        self_.group0_.x
    );
}


struct Sphere {
    e4235_: f32, e4315_: f32, e4125_: f32, e3215_: f32,
    e1234_: f32
}
struct SphereGroups {
    // e4235, e4315, e4125, e3215
    group0_: vec4<f32>,
    // e1234, 0, 0, 0
    group1_: vec4<f32>
}
fn sphere_grouped(self_: Sphere) -> SphereGroups {
    return SphereGroups(
        vec4<f32>(self_.e4235_, self_.e4315_, self_.e4125_, self_.e3215_),
        vec4<f32>(self_.e1234_, 0.0, 0.0, 0.0)
    );
}
fn sphere_degroup(self_: SphereGroups) -> Sphere {
    return Sphere(
        self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group0_.w,
        self_.group1_.x
    );
}


struct VersorEven {
    e423_: f32, e431_: f32, e412_: f32, e12345_: f32,
    e415_: f32, e425_: f32, e435_: f32, e321_: f32,
    e235_: f32, e315_: f32, e125_: f32, e5_: f32,
    e1_: f32, e2_: f32, e3_: f32, e4_: f32
}
struct VersorEvenGroups {
    // e423, e431, e412, e12345
    group0_: vec4<f32>,
    // e415, e425, e435, e321
    group1_: vec4<f32>,
    // e235, e315, e125, e5
    group2_: vec4<f32>,
    // e1, e2, e3, e4
    group3_: vec4<f32>
}
fn versorEven_grouped(self_: VersorEven) -> VersorEvenGroups {
    return VersorEvenGroups(
        vec4<f32>(self_.e423_, self_.e431_, self_.e412_, self_.e12345_),
        vec4<f32>(self_.e415_, self_.e425_, self_.e435_, self_.e321_),
        vec4<f32>(self_.e235_, self_.e315_, self_.e125_, self_.e5_),
        vec4<f32>(self_.e1_, self_.e2_, self_.e3_, self_.e4_)
    );
}
fn versorEven_degroup(self_: VersorEvenGroups) -> VersorEven {
    return VersorEven(
        self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group0_.w,
        self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group1_.w,
        self_.group2_.x, self_.group2_.y, self_.group2_.z, self_.group2_.w,
        self_.group3_.x, self_.group3_.y, self_.group3_.z, self_.group3_.w
    );
}


struct VersorOdd {
    e41_: f32, e42_: f32, e43_: f32, scalar: f32,
    e23_: f32, e31_: f32, e12_: f32, e45_: f32,
    e15_: f32, e25_: f32, e35_: f32, e1234_: f32,
    e4235_: f32, e4315_: f32, e4125_: f32, e3215_: f32
}
struct VersorOddGroups {
    // e41, e42, e43, scalar
    group0_: vec4<f32>,
    // e23, e31, e12, e45
    group1_: vec4<f32>,
    // e15, e25, e35, e1234
    group2_: vec4<f32>,
    // e4235, e4315, e4125, e3215
    group3_: vec4<f32>
}


// The naga_oil pruner has a bug where it can't handle these expressions inlined
// so we have to wrap the operation in a function so it doesn't choke.
// It's an index out of bounds (3 exceeding 2, or 4 exceeding 3)
// in Pruner.add_expression -> match Expression::Compose -> match PartReq::Part.
// Obviously we want an issue/PR fix in naga_oil but the PartReq tracking is
// insufferable to read and/or debug, so I'm procrastinating that.
fn extendVec2toVec3(v: vec2<f32>, z: f32) -> vec3<f32> {
    return vec3<f32>(v[0], v[1], z);
}
fn extendVec2toVec4(v: vec2<f32>, z: f32, w: f32) -> vec4<f32> {
    return vec4<f32>(v[0], v[1], z, w);
}
fn extendVec3toVec4(v: vec3<f32>, w: f32) -> vec4<f32> {
    return vec4<f32>(v[0], v[1], v[2], w);
}


fn versorOdd_grouped(self_: VersorOdd) -> VersorOddGroups {
    return VersorOddGroups(
        vec4<f32>(self_.e41_, self_.e42_, self_.e43_, self_.scalar),
        vec4<f32>(self_.e23_, self_.e31_, self_.e12_, self_.e45_),
        vec4<f32>(self_.e15_, self_.e25_, self_.e35_, self_.e1234_),
        vec4<f32>(self_.e4235_, self_.e4315_, self_.e4125_, self_.e3215_)
    );
}
fn versorOdd_degroup(self_: VersorOddGroups) -> VersorOdd {
    return VersorOdd(
        self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group0_.w,
        self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group1_.w,
        self_.group2_.x, self_.group2_.y, self_.group2_.z, self_.group2_.w,
        self_.group3_.x, self_.group3_.y, self_.group3_.z, self_.group3_.w
    );
}


fn roundPoint_wedge_roundPoint(self_: RoundPoint, other: RoundPoint) -> Dipole {
    let self_groups = roundPoint_grouped(self_);
    let other_groups = roundPoint_grouped(other);
    return dipole_degroup(DipoleGroups(
        /* e41, e42, e43 */ ((vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group0_) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_),
        /* e23, e31, e12, e45 */
        extendVec3toVec4(
        (other_groups.group0_.zxyw * self_groups.group0_.yzxw).xyz,
        other.e5_ * self_.e4_
        ) - (other_groups.group0_.yzxw * extendVec3toVec4(self_groups.group0_.zxyw.xyz, self_.e5_)),
        /* e15, e25, e35 */ ((vec4<f32>(other.e5_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_)
        - ((vec4<f32>(self_.e5_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group0_)
    ));
}
fn dipole_wedge_roundPoint(self_: Dipole, other: RoundPoint) -> Circle {
    let self_groups = dipole_grouped(self_);
    let other_groups = roundPoint_grouped(other);
    return circle_degroup(CircleGroups(
        /* e423, e431, e412 */ ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_)
        + (self_groups.group0_.yzxw * other_groups.group0_.zxyw) - (self_groups.group0_.zxyw * other_groups.group0_.yzxw),
        /* e415, e425, e435, e321 */ vec4<f32>(
            (self_.e41_ * other.e5_) + (self_.e15_ * other.e4_),
            (self_.e42_ * other.e5_) + (self_.e25_ * other.e4_),
            (self_.e43_ * other.e5_) + (self_.e35_ * other.e4_),
            -(self_.e31_ * other.e2_) - (self_.e12_ * other.e3_)
        ) - (self_groups.group1_.wwwx * other_groups.group0_.xyzx),
        /* e235, e315, e125 */ ((vec4<f32>(other.e5_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_)
        + (self_groups.group2_.zxyw * other_groups.group0_.yzxw) - (self_groups.group2_.yzxw * other_groups.group0_.zxyw)
    ));
}
fn flatPoint_wedge_roundPoint(self_: FlatPoint, other: RoundPoint) -> Line {
    let self_groups = flatPoint_grouped(self_);
    let other_groups = roundPoint_grouped(other);
    return line_degroup(LineGroups(
        /* e415, e425, e435 */ ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_)
        - ((vec4<f32>(self_.e45_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group0_),
        /* e235, e315, e125 */ (self_groups.group0_.zxyw * other_groups.group0_.yzxw) - (self_groups.group0_.yzxw * other_groups.group0_.zxyw)
    ));
}

fn circle_antiWedge_line(self_: Circle, other: Line) -> RoundPoint {
//    let self_groups = circle_grouped(self_);
//    let other_groups = line_grouped(other);
    return RoundPoint(self_.e412_, other.e315_, 0.0, 0.0, 0.0);
//    return roundPoint_degroup(RoundPointGroups(
//        /* e1, e2, e3, e4 */
//        vec4<f32>(0.0),
////        vec4<f32>((self_.e412_ * other.e315_) + (self_.e321_ * other.e415_), (self_.e423_ * other.e125_) + (self_.e321_ * other.e425_), (self_.e431_ * other.e235_) + (self_.e321_ * other.e435_), -(self_.e431_ * other.e425_) - (self_.e412_ * other.e435_)) - vec4<f32>((self_groups.group0_.yzxw * other_groups.group1_.zxyw).xyz, self_.e423_ * other.e415_),
//        /* e5 */
//        vec4<f32>(0.0),
////        vec4<f32>(-(self_.e415_ * other.e235_) - (self_.e425_ * other.e315_) - (self_.e435_ * other.e125_) - (self_.e235_ * other.e415_) - (self_.e315_ * other.e425_) - (self_.e125_ * other.e435_), 0.0, 0.0, 0.0)
//    ));
}

//fn circle_antiWedge_line(self_: Circle, other: Line) -> RoundPoint {
//    let self_groups = circle_grouped(self_);
//    let other_groups = line_grouped(other);
//    return roundPoint_degroup(RoundPointGroups(
//        /* e1, e2, e3, e4 */
//        vec4<f32>(
//            (self_.e412_ * other.e315_) + (self_.e321_ * other.e415_),
//            (self_.e423_ * other.e125_) + (self_.e321_ * other.e425_),
//            (self_.e431_ * other.e235_) + (self_.e321_ * other.e435_),
//            -(self_.e431_ * other.e425_) - (self_.e412_ * other.e435_)) - vec4<f32>((self_groups.group0_.yzxw * other_groups.group1_.zxyw).xyz, self_.e423_ * other.e415_
//        ),
//        /* e5 */
//        vec4<f32>(
//            -(self_.e415_ * other.e235_)
//            - (self_.e425_ * other.e315_)
//            - (self_.e435_ * other.e125_)
//            - (self_.e235_ * other.e415_)
//            - (self_.e315_ * other.e425_)
//            - (self_.e125_ * other.e435_),
//            0.0,
//            0.0,
//            0.0
//        )
//    ));
//}
fn antiScalar_rightAntiDual(self_: AntiScalar) -> Scalar {
    return Scalar(self_.e12345_ * -1.0);
}
fn roundPoint_antiDotProduct_roundPoint(self_: RoundPoint, other: RoundPoint) -> AntiScalar {
    return AntiScalar((other.e4_ * self_.e5_) + (other.e5_ * self_.e4_) - (other.e1_ * self_.e1_) - (other.e2_ * self_.e2_) - (other.e3_ * self_.e3_));
}

fn roundPoint_radiusNormSquared(self_: RoundPoint) -> Scalar {
    return antiScalar_rightAntiDual(roundPoint_antiDotProduct_roundPoint(self_, self_));
}
fn roundPoint_roundWeight(self_: RoundPoint) -> RoundPoint {
    return roundPoint_degroup(RoundPointGroups(
        /* e1, e2, e3, e4 */ extendVec3toVec4(vec4<f32>(0.0).xyz, self_.e4_),
        /* e5 */ vec4<f32>(0.0, 0.0, 0.0, 0.0)
    ));
}
fn roundPoint_wedge_dualNum(self_: RoundPoint, other: DualNum) -> FlatPoint {
    let self_groups = roundPoint_grouped(self_);
    return flatPoint_degroup(FlatPointGroups(
        /* e15, e25, e35, e45 */ vec4<f32>(other.e5_) * self_groups.group0_
    ));
}
fn flatPoint_antiDotProduct_flatPoint(self_: FlatPoint, other: FlatPoint) -> AntiScalar {
    return AntiScalar(other.e45_ * self_.e45_);
}
fn roundPoint_roundWeightNormSquared(self_: RoundPoint) -> AntiScalar {
    let self_groups = roundPoint_grouped(self_);
    let round_weight_carrier: FlatPoint = roundPoint_wedge_dualNum(roundPoint_roundWeight(self_), dualNum_degroup(DualNumGroups(
        /* e5, e12345 */ vec4<f32>(1.0, 0.0, 0.0, 0.0)
    )));
    return flatPoint_antiDotProduct_flatPoint(round_weight_carrier, round_weight_carrier);
}
fn roundPoint_unitizedRadiusNormSquared(self_: RoundPoint) -> f32 {
    let self_groups = roundPoint_grouped(self_);
    return roundPoint_radiusNormSquared(self_).scalar / (roundPoint_roundWeightNormSquared(self_).e12345_);
}
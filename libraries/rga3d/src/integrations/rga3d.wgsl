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
    e1234_: f32
}
struct AntiScalarGroups {
    // e1234, 0, 0, 0
    group0_: vec4<f32>
}
fn antiScalar_grouped(self_: AntiScalar) -> AntiScalarGroups {
    return AntiScalarGroups(
        vec4<f32>(self_.e1234_, 0.0, 0.0, 0.0)
    );
}
fn antiScalar_degroup(self_: AntiScalarGroups) -> AntiScalar {
    return AntiScalar(
        self_.group0_.x
    );
}


struct DualNum {
    scalar: f32, e1234_: f32
}
struct DualNumGroups {
    // scalar, e1234, 0, 0
    group0_: vec4<f32>
}
fn dualNum_grouped(self_: DualNum) -> DualNumGroups {
    return DualNumGroups(
        vec4<f32>(self_.scalar, self_.e1234_, 0.0, 0.0)
    );
}
fn dualNum_degroup(self_: DualNumGroups) -> DualNum {
    return DualNum(
        self_.group0_.x, self_.group0_.y
    );
}


struct Flector {
    e1_: f32, e2_: f32, e3_: f32, e4_: f32,
    e423_: f32, e431_: f32, e412_: f32, e321_: f32
}
struct FlectorGroups {
    // e1, e2, e3, e4
    group0_: vec4<f32>,
    // e423, e431, e412, e321
    group1_: vec4<f32>
}
fn flector_grouped(self_: Flector) -> FlectorGroups {
    return FlectorGroups(
        vec4<f32>(self_.e1_, self_.e2_, self_.e3_, self_.e4_),
        vec4<f32>(self_.e423_, self_.e431_, self_.e412_, self_.e321_)
    );
}
fn flector_degroup(self_: FlectorGroups) -> Flector {
    return Flector(
        self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group0_.w,
        self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group1_.w
    );
}


struct Horizon {
    e321_: f32
}
struct HorizonGroups {
    // e321, 0, 0, 0
    group0_: vec4<f32>
}
fn horizon_grouped(self_: Horizon) -> HorizonGroups {
    return HorizonGroups(
        vec4<f32>(self_.e321_, 0.0, 0.0, 0.0)
    );
}
fn horizon_degroup(self_: HorizonGroups) -> Horizon {
    return Horizon(
        self_.group0_.x
    );
}


struct Line {
    e41_: f32, e42_: f32, e43_: f32,
    e23_: f32, e31_: f32, e12_: f32
}
struct LineGroups {
    // e41, e42, e43, 0
    group0_: vec4<f32>,
    // e23, e31, e12, 0
    group1_: vec4<f32>
}
fn line_grouped(self_: Line) -> LineGroups {
    return LineGroups(
        vec4<f32>(self_.e41_, self_.e42_, self_.e43_, 0.0),
        vec4<f32>(self_.e23_, self_.e31_, self_.e12_, 0.0)
    );
}
fn line_degroup(self_: LineGroups) -> Line {
    return Line(
        self_.group0_.x, self_.group0_.y, self_.group0_.z,
        self_.group1_.x, self_.group1_.y, self_.group1_.z
    );
}


struct Motor {
    e41_: f32, e42_: f32, e43_: f32, e1234_: f32,
    e23_: f32, e31_: f32, e12_: f32, scalar: f32
}
struct MotorGroups {
    // e41, e42, e43, e1234
    group0_: vec4<f32>,
    // e23, e31, e12, scalar
    group1_: vec4<f32>
}
fn motor_grouped(self_: Motor) -> MotorGroups {
    return MotorGroups(
        vec4<f32>(self_.e41_, self_.e42_, self_.e43_, self_.e1234_),
        vec4<f32>(self_.e23_, self_.e31_, self_.e12_, self_.scalar)
    );
}
fn motor_degroup(self_: MotorGroups) -> Motor {
    return Motor(
        self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group0_.w,
        self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group1_.w
    );
}


struct MultiVector {
    scalar: f32, e1234_: f32,
    e1_: f32, e2_: f32, e3_: f32, e4_: f32,
    e41_: f32, e42_: f32, e43_: f32,
    e23_: f32, e31_: f32, e12_: f32,
    e423_: f32, e431_: f32, e412_: f32, e321_: f32
}
struct MultiVectorGroups {
    // scalar, e1234, 0, 0
    group0_: vec4<f32>,
    // e1, e2, e3, e4
    group1_: vec4<f32>,
    // e41, e42, e43, 0
    group2_: vec4<f32>,
    // e23, e31, e12, 0
    group3_: vec4<f32>,
    // e423, e431, e412, e321
    group4_: vec4<f32>
}
fn multiVector_grouped(self_: MultiVector) -> MultiVectorGroups {
    return MultiVectorGroups(
        vec4<f32>(self_.scalar, self_.e1234_, 0.0, 0.0),
        vec4<f32>(self_.e1_, self_.e2_, self_.e3_, self_.e4_),
        vec4<f32>(self_.e41_, self_.e42_, self_.e43_, 0.0),
        vec4<f32>(self_.e23_, self_.e31_, self_.e12_, 0.0),
        vec4<f32>(self_.e423_, self_.e431_, self_.e412_, self_.e321_)
    );
}
fn multiVector_degroup(self_: MultiVectorGroups) -> MultiVector {
    return MultiVector(
        self_.group0_.x, self_.group0_.y,
        self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group1_.w,
        self_.group2_.x, self_.group2_.y, self_.group2_.z,
        self_.group3_.x, self_.group3_.y, self_.group3_.z,
        self_.group4_.x, self_.group4_.y, self_.group4_.z, self_.group4_.w
    );
}


struct Origin {
    e4_: f32
}
struct OriginGroups {
    // e4, 0, 0, 0
    group0_: vec4<f32>
}
fn origin_grouped(self_: Origin) -> OriginGroups {
    return OriginGroups(
        vec4<f32>(self_.e4_, 0.0, 0.0, 0.0)
    );
}
fn origin_degroup(self_: OriginGroups) -> Origin {
    return Origin(
        self_.group0_.x
    );
}


struct Plane {
    e423_: f32, e431_: f32, e412_: f32, e321_: f32
}
struct PlaneGroups {
    // e423, e431, e412, e321
    group0_: vec4<f32>
}
fn plane_grouped(self_: Plane) -> PlaneGroups {
    return PlaneGroups(
        vec4<f32>(self_.e423_, self_.e431_, self_.e412_, self_.e321_)
    );
}
fn plane_degroup(self_: PlaneGroups) -> Plane {
    return Plane(
        self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group0_.w
    );
}


struct Point {
    e1_: f32, e2_: f32, e3_: f32, e4_: f32
}
struct PointGroups {
    // e1, e2, e3, e4
    group0_: vec4<f32>
}
fn point_grouped(self_: Point) -> PointGroups {
    return PointGroups(
        vec4<f32>(self_.e1_, self_.e2_, self_.e3_, self_.e4_)
    );
}
fn point_degroup(self_: PointGroups) -> Point {
    return Point(
        self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group0_.w
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

fn antiScalar_add_antiScalar(self_: AntiScalar, other: AntiScalar) -> AntiScalar {
    return AntiScalar(other.e1234_ + self_.e1234_);
}
fn antiScalar_add_dualNum(self_: AntiScalar, other: DualNum) -> DualNum {
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar, self_.e1234_ + other.e1234_, 0.0, 0.0)
    ));
}
fn antiScalar_add_flector(self_: AntiScalar, other: Flector) -> MultiVector {
    let other_groups = flector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, self_.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ other_groups.group0_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ other_groups.group1_
    ));
}
fn antiScalar_add_horizon(self_: AntiScalar, other: Horizon) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, self_.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other.e321_)
    ));
}
fn antiScalar_add_line(self_: AntiScalar, other: Line) -> Motor {
    let other_groups = line_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other_groups.group0_.xyz.xyz, self_.e1234_), 
        /* e23, e31, e12, scalar */ vec4<f32>(other_groups.group1_.xyz.xyz, 0.0)
    ));
}
fn antiScalar_add_motor(self_: AntiScalar, other: Motor) -> Motor {
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e41_, other.e42_, other.e43_, self_.e1234_ + other.e1234_), 
        /* e23, e31, e12, scalar */ other_groups.group1_
    ));
}
fn antiScalar_add_multiVector(self_: AntiScalar, other: MultiVector) -> MultiVector {
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar, self_.e1234_ + other.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ other_groups.group1_, 
        /* e41, e42, e43 */ other_groups.group2_, 
        /* e23, e31, e12 */ other_groups.group3_, 
        /* e423, e431, e412, e321 */ other_groups.group4_
    ));
}
fn antiScalar_add_origin(self_: AntiScalar, other: Origin) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, self_.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other.e4_), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn antiScalar_add_plane(self_: AntiScalar, other: Plane) -> MultiVector {
    let other_groups = plane_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, self_.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ other_groups.group0_
    ));
}
fn antiScalar_add_point(self_: AntiScalar, other: Point) -> MultiVector {
    let other_groups = point_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, self_.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ other_groups.group0_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn antiScalar_add_scalar(self_: AntiScalar, other: Scalar) -> DualNum {
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar, self_.e1234_, 0.0, 0.0)
    ));
}
fn dualNum_add_antiScalar(self_: DualNum, other: AntiScalar) -> DualNum {
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar, other.e1234_ + self_.e1234_, 0.0, 0.0)
    ));
}
fn dualNum_add_dualNum(self_: DualNum, other: DualNum) -> DualNum {
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar + self_.scalar, other.e1234_ + self_.e1234_, 0.0, 0.0)
    ));
}
fn dualNum_add_flector(self_: DualNum, other: Flector) -> MultiVector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = flector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_groups.group0_, 
        /* e1, e2, e3, e4 */ other_groups.group0_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ other_groups.group1_
    ));
}
fn dualNum_add_horizon(self_: DualNum, other: Horizon) -> MultiVector {
    let self_groups = dualNum_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other.e321_)
    ));
}
fn dualNum_add_line(self_: DualNum, other: Line) -> Motor {
    let other_groups = line_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other_groups.group0_.xyz.xyz, self_.e1234_), 
        /* e23, e31, e12, scalar */ vec4<f32>(other_groups.group1_.xyz.xyz, self_.scalar)
    ));
}
fn dualNum_add_motor(self_: DualNum, other: Motor) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e41_, other.e42_, other.e43_, self_.e1234_ + other.e1234_), 
        /* e23, e31, e12, scalar */ vec4<f32>(other.e23_, other.e31_, other.e12_, self_.scalar + other.scalar)
    ));
}
fn dualNum_add_multiVector(self_: DualNum, other: MultiVector) -> MultiVector {
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar + other.scalar, self_.e1234_ + other.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ other_groups.group1_, 
        /* e41, e42, e43 */ other_groups.group2_, 
        /* e23, e31, e12 */ other_groups.group3_, 
        /* e423, e431, e412, e321 */ other_groups.group4_
    ));
}
fn dualNum_add_origin(self_: DualNum, other: Origin) -> MultiVector {
    let self_groups = dualNum_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other.e4_), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn dualNum_add_plane(self_: DualNum, other: Plane) -> MultiVector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = plane_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ other_groups.group0_
    ));
}
fn dualNum_add_point(self_: DualNum, other: Point) -> MultiVector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = point_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_groups.group0_, 
        /* e1, e2, e3, e4 */ other_groups.group0_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn dualNum_add_scalar(self_: DualNum, other: Scalar) -> DualNum {
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar + other.scalar, self_.e1234_, 0.0, 0.0)
    ));
}
fn flector_add_antiScalar(self_: Flector, other: AntiScalar) -> MultiVector {
    let self_groups = flector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, other.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ self_groups.group0_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ self_groups.group1_
    ));
}
fn flector_add_dualNum(self_: Flector, other: DualNum) -> MultiVector {
    let self_groups = flector_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_, 
        /* e1, e2, e3, e4 */ self_groups.group0_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ self_groups.group1_
    ));
}
fn flector_add_flector(self_: Flector, other: Flector) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e1_ + self_.e1_, other.e2_ + self_.e2_, other.e3_ + self_.e3_, other.e4_ + self_.e4_), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e423_ + self_.e423_, other.e431_ + self_.e431_, other.e412_ + self_.e412_, other.e321_ + self_.e321_)
    ));
}
fn flector_add_horizon(self_: Flector, other: Horizon) -> Flector {
    let self_groups = flector_grouped(self_);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ self_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e423_, self_.e431_, self_.e412_, self_.e321_ + other.e321_)
    ));
}
fn flector_add_line(self_: Flector, other: Line) -> MultiVector {
    let self_groups = flector_grouped(self_);
    let other_groups = line_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ self_groups.group0_, 
        /* e41, e42, e43 */ other_groups.group0_, 
        /* e23, e31, e12 */ other_groups.group1_, 
        /* e423, e431, e412, e321 */ self_groups.group1_
    ));
}
fn flector_add_motor(self_: Flector, other: Motor) -> MultiVector {
    let self_groups = flector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar, other.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ self_groups.group0_, 
        /* e41, e42, e43 */ vec4<f32>(other.e41_, other.e42_, other.e43_, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(other.e23_, other.e31_, other.e12_, 0.0), 
        /* e423, e431, e412, e321 */ self_groups.group1_
    ));
}
fn flector_add_multiVector(self_: Flector, other: MultiVector) -> MultiVector {
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e1_ + other.e1_, self_.e2_ + other.e2_, self_.e3_ + other.e3_, self_.e4_ + other.e4_), 
        /* e41, e42, e43 */ other_groups.group2_, 
        /* e23, e31, e12 */ other_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e423_ + other.e423_, self_.e431_ + other.e431_, self_.e412_ + other.e412_, self_.e321_ + other.e321_)
    ));
}
fn flector_add_origin(self_: Flector, other: Origin) -> Flector {
    let self_groups = flector_grouped(self_);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e1_, self_.e2_, self_.e3_, self_.e4_ + other.e4_), 
        /* e423, e431, e412, e321 */ self_groups.group1_
    ));
}
fn flector_add_plane(self_: Flector, other: Plane) -> Flector {
    let self_groups = flector_grouped(self_);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ self_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e423_ + other.e423_, self_.e431_ + other.e431_, self_.e412_ + other.e412_, self_.e321_ + other.e321_)
    ));
}
fn flector_add_point(self_: Flector, other: Point) -> Flector {
    let self_groups = flector_grouped(self_);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e1_ + other.e1_, self_.e2_ + other.e2_, self_.e3_ + other.e3_, self_.e4_ + other.e4_), 
        /* e423, e431, e412, e321 */ self_groups.group1_
    ));
}
fn flector_add_scalar(self_: Flector, other: Scalar) -> MultiVector {
    let self_groups = flector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ self_groups.group0_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ self_groups.group1_
    ));
}
fn horizon_add_antiScalar(self_: Horizon, other: AntiScalar) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, other.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.e321_)
    ));
}
fn horizon_add_dualNum(self_: Horizon, other: DualNum) -> MultiVector {
    let other_groups = dualNum_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.e321_)
    ));
}
fn horizon_add_flector(self_: Horizon, other: Flector) -> Flector {
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ other_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e423_, other.e431_, other.e412_, other.e321_ + self_.e321_)
    ));
}
fn horizon_add_horizon(self_: Horizon, other: Horizon) -> Horizon {
    return Horizon(other.e321_ + self_.e321_);
}
fn horizon_add_line(self_: Horizon, other: Line) -> MultiVector {
    let other_groups = line_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ other_groups.group0_, 
        /* e23, e31, e12 */ other_groups.group1_, 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.e321_)
    ));
}
fn horizon_add_motor(self_: Horizon, other: Motor) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar, other.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(other.e41_, other.e42_, other.e43_, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(other.e23_, other.e31_, other.e12_, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.e321_)
    ));
}
fn horizon_add_multiVector(self_: Horizon, other: MultiVector) -> MultiVector {
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_, 
        /* e1, e2, e3, e4 */ other_groups.group1_, 
        /* e41, e42, e43 */ other_groups.group2_, 
        /* e23, e31, e12 */ other_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e423_, other.e431_, other.e412_, self_.e321_ + other.e321_)
    ));
}
fn horizon_add_origin(self_: Horizon, other: Origin) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other.e4_), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.e321_)
    ));
}
fn horizon_add_plane(self_: Horizon, other: Plane) -> Plane {
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(other.e423_, other.e431_, other.e412_, self_.e321_ + other.e321_)
    ));
}
fn horizon_add_point(self_: Horizon, other: Point) -> Flector {
    let other_groups = point_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ other_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.e321_)
    ));
}
fn horizon_add_scalar(self_: Horizon, other: Scalar) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.e321_)
    ));
}
fn line_add_antiScalar(self_: Line, other: AntiScalar) -> Motor {
    let self_groups = line_grouped(self_);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_groups.group0_.xyz.xyz, other.e1234_), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_groups.group1_.xyz.xyz, 0.0)
    ));
}
fn line_add_dualNum(self_: Line, other: DualNum) -> Motor {
    let self_groups = line_grouped(self_);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_groups.group0_.xyz.xyz, other.e1234_), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_groups.group1_.xyz.xyz, other.scalar)
    ));
}
fn line_add_flector(self_: Line, other: Flector) -> MultiVector {
    let self_groups = line_grouped(self_);
    let other_groups = flector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ other_groups.group0_, 
        /* e41, e42, e43 */ self_groups.group0_, 
        /* e23, e31, e12 */ self_groups.group1_, 
        /* e423, e431, e412, e321 */ other_groups.group1_
    ));
}
fn line_add_horizon(self_: Line, other: Horizon) -> MultiVector {
    let self_groups = line_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ self_groups.group0_, 
        /* e23, e31, e12 */ self_groups.group1_, 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other.e321_)
    ));
}
fn line_add_line(self_: Line, other: Line) -> Line {
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(other.e41_ + self_.e41_, other.e42_ + self_.e42_, other.e43_ + self_.e43_, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(other.e23_ + self_.e23_, other.e31_ + self_.e31_, other.e12_ + self_.e12_, 0.0)
    ));
}
fn line_add_motor(self_: Line, other: Motor) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.e41_ + other.e41_, self_.e42_ + other.e42_, self_.e43_ + other.e43_, other.e1234_), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.e23_ + other.e23_, self_.e31_ + other.e31_, self_.e12_ + other.e12_, other.scalar)
    ));
}
fn line_add_multiVector(self_: Line, other: MultiVector) -> MultiVector {
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_, 
        /* e1, e2, e3, e4 */ other_groups.group1_, 
        /* e41, e42, e43 */ vec4<f32>(self_.e41_ + other.e41_, self_.e42_ + other.e42_, self_.e43_ + other.e43_, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(self_.e23_ + other.e23_, self_.e31_ + other.e31_, self_.e12_ + other.e12_, 0.0), 
        /* e423, e431, e412, e321 */ other_groups.group4_
    ));
}
fn line_add_origin(self_: Line, other: Origin) -> MultiVector {
    let self_groups = line_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other.e4_), 
        /* e41, e42, e43 */ self_groups.group0_, 
        /* e23, e31, e12 */ self_groups.group1_, 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn line_add_plane(self_: Line, other: Plane) -> MultiVector {
    let self_groups = line_grouped(self_);
    let other_groups = plane_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ self_groups.group0_, 
        /* e23, e31, e12 */ self_groups.group1_, 
        /* e423, e431, e412, e321 */ other_groups.group0_
    ));
}
fn line_add_point(self_: Line, other: Point) -> MultiVector {
    let self_groups = line_grouped(self_);
    let other_groups = point_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ other_groups.group0_, 
        /* e41, e42, e43 */ self_groups.group0_, 
        /* e23, e31, e12 */ self_groups.group1_, 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn line_add_scalar(self_: Line, other: Scalar) -> Motor {
    let self_groups = line_grouped(self_);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_groups.group0_.xyz.xyz, 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_groups.group1_.xyz.xyz, other.scalar)
    ));
}
fn motor_add_antiScalar(self_: Motor, other: AntiScalar) -> Motor {
    let self_groups = motor_grouped(self_);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.e41_, self_.e42_, self_.e43_, other.e1234_ + self_.e1234_), 
        /* e23, e31, e12, scalar */ self_groups.group1_
    ));
}
fn motor_add_dualNum(self_: Motor, other: DualNum) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.e41_, self_.e42_, self_.e43_, other.e1234_ + self_.e1234_), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.e23_, self_.e31_, self_.e12_, other.scalar + self_.scalar)
    ));
}
fn motor_add_flector(self_: Motor, other: Flector) -> MultiVector {
    let other_groups = flector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar, self_.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ other_groups.group0_, 
        /* e41, e42, e43 */ vec4<f32>(self_.e41_, self_.e42_, self_.e43_, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(self_.e23_, self_.e31_, self_.e12_, 0.0), 
        /* e423, e431, e412, e321 */ other_groups.group1_
    ));
}
fn motor_add_horizon(self_: Motor, other: Horizon) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar, self_.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(self_.e41_, self_.e42_, self_.e43_, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(self_.e23_, self_.e31_, self_.e12_, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other.e321_)
    ));
}
fn motor_add_line(self_: Motor, other: Line) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e41_ + self_.e41_, other.e42_ + self_.e42_, other.e43_ + self_.e43_, self_.e1234_), 
        /* e23, e31, e12, scalar */ vec4<f32>(other.e23_ + self_.e23_, other.e31_ + self_.e31_, other.e12_ + self_.e12_, self_.scalar)
    ));
}
fn motor_add_motor(self_: Motor, other: Motor) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e41_ + self_.e41_, other.e42_ + self_.e42_, other.e43_ + self_.e43_, other.e1234_ + self_.e1234_), 
        /* e23, e31, e12, scalar */ vec4<f32>(other.e23_ + self_.e23_, other.e31_ + self_.e31_, other.e12_ + self_.e12_, other.scalar + self_.scalar)
    ));
}
fn motor_add_multiVector(self_: Motor, other: MultiVector) -> MultiVector {
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar + other.scalar, self_.e1234_ + other.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ other_groups.group1_, 
        /* e41, e42, e43 */ vec4<f32>(self_.e41_ + other.e41_, self_.e42_ + other.e42_, self_.e43_ + other.e43_, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(self_.e23_ + other.e23_, self_.e31_ + other.e31_, self_.e12_ + other.e12_, 0.0), 
        /* e423, e431, e412, e321 */ other_groups.group4_
    ));
}
fn motor_add_origin(self_: Motor, other: Origin) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar, self_.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other.e4_), 
        /* e41, e42, e43 */ vec4<f32>(self_.e41_, self_.e42_, self_.e43_, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(self_.e23_, self_.e31_, self_.e12_, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn motor_add_plane(self_: Motor, other: Plane) -> MultiVector {
    let other_groups = plane_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar, self_.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(self_.e41_, self_.e42_, self_.e43_, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(self_.e23_, self_.e31_, self_.e12_, 0.0), 
        /* e423, e431, e412, e321 */ other_groups.group0_
    ));
}
fn motor_add_point(self_: Motor, other: Point) -> MultiVector {
    let other_groups = point_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar, self_.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ other_groups.group0_, 
        /* e41, e42, e43 */ vec4<f32>(self_.e41_, self_.e42_, self_.e43_, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(self_.e23_, self_.e31_, self_.e12_, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn motor_add_scalar(self_: Motor, other: Scalar) -> Motor {
    let self_groups = motor_grouped(self_);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ self_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.e23_, self_.e31_, self_.e12_, self_.scalar + other.scalar)
    ));
}
fn multiVector_add_antiScalar(self_: MultiVector, other: AntiScalar) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar, other.e1234_ + self_.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ self_groups.group1_, 
        /* e41, e42, e43 */ self_groups.group2_, 
        /* e23, e31, e12 */ self_groups.group3_, 
        /* e423, e431, e412, e321 */ self_groups.group4_
    ));
}
fn multiVector_add_dualNum(self_: MultiVector, other: DualNum) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar + self_.scalar, other.e1234_ + self_.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ self_groups.group1_, 
        /* e41, e42, e43 */ self_groups.group2_, 
        /* e23, e31, e12 */ self_groups.group3_, 
        /* e423, e431, e412, e321 */ self_groups.group4_
    ));
}
fn multiVector_add_flector(self_: MultiVector, other: Flector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(other.e1_ + self_.e1_, other.e2_ + self_.e2_, other.e3_ + self_.e3_, other.e4_ + self_.e4_), 
        /* e41, e42, e43 */ self_groups.group2_, 
        /* e23, e31, e12 */ self_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e423_ + self_.e423_, other.e431_ + self_.e431_, other.e412_ + self_.e412_, other.e321_ + self_.e321_)
    ));
}
fn multiVector_add_horizon(self_: MultiVector, other: Horizon) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_groups.group0_, 
        /* e1, e2, e3, e4 */ self_groups.group1_, 
        /* e41, e42, e43 */ self_groups.group2_, 
        /* e23, e31, e12 */ self_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e423_, self_.e431_, self_.e412_, other.e321_ + self_.e321_)
    ));
}
fn multiVector_add_line(self_: MultiVector, other: Line) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_groups.group0_, 
        /* e1, e2, e3, e4 */ self_groups.group1_, 
        /* e41, e42, e43 */ vec4<f32>(other.e41_ + self_.e41_, other.e42_ + self_.e42_, other.e43_ + self_.e43_, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(other.e23_ + self_.e23_, other.e31_ + self_.e31_, other.e12_ + self_.e12_, 0.0), 
        /* e423, e431, e412, e321 */ self_groups.group4_
    ));
}
fn multiVector_add_motor(self_: MultiVector, other: Motor) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar + self_.scalar, other.e1234_ + self_.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ self_groups.group1_, 
        /* e41, e42, e43 */ vec4<f32>(other.e41_ + self_.e41_, other.e42_ + self_.e42_, other.e43_ + self_.e43_, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(other.e23_ + self_.e23_, other.e31_ + self_.e31_, other.e12_ + self_.e12_, 0.0), 
        /* e423, e431, e412, e321 */ self_groups.group4_
    ));
}
fn multiVector_add_multiVector(self_: MultiVector, other: MultiVector) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar + self_.scalar, other.e1234_ + self_.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(other.e1_ + self_.e1_, other.e2_ + self_.e2_, other.e3_ + self_.e3_, other.e4_ + self_.e4_), 
        /* e41, e42, e43 */ vec4<f32>(other.e41_ + self_.e41_, other.e42_ + self_.e42_, other.e43_ + self_.e43_, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(other.e23_ + self_.e23_, other.e31_ + self_.e31_, other.e12_ + self_.e12_, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e423_ + self_.e423_, other.e431_ + self_.e431_, other.e412_ + self_.e412_, other.e321_ + self_.e321_)
    ));
}
fn multiVector_add_origin(self_: MultiVector, other: Origin) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e1_, self_.e2_, self_.e3_, self_.e4_ + other.e4_), 
        /* e41, e42, e43 */ self_groups.group2_, 
        /* e23, e31, e12 */ self_groups.group3_, 
        /* e423, e431, e412, e321 */ self_groups.group4_
    ));
}
fn multiVector_add_plane(self_: MultiVector, other: Plane) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_groups.group0_, 
        /* e1, e2, e3, e4 */ self_groups.group1_, 
        /* e41, e42, e43 */ self_groups.group2_, 
        /* e23, e31, e12 */ self_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e423_ + other.e423_, self_.e431_ + other.e431_, self_.e412_ + other.e412_, self_.e321_ + other.e321_)
    ));
}
fn multiVector_add_point(self_: MultiVector, other: Point) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e1_ + other.e1_, self_.e2_ + other.e2_, self_.e3_ + other.e3_, self_.e4_ + other.e4_), 
        /* e41, e42, e43 */ self_groups.group2_, 
        /* e23, e31, e12 */ self_groups.group3_, 
        /* e423, e431, e412, e321 */ self_groups.group4_
    ));
}
fn multiVector_add_scalar(self_: MultiVector, other: Scalar) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar + other.scalar, self_.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ self_groups.group1_, 
        /* e41, e42, e43 */ self_groups.group2_, 
        /* e23, e31, e12 */ self_groups.group3_, 
        /* e423, e431, e412, e321 */ self_groups.group4_
    ));
}
fn origin_add_antiScalar(self_: Origin, other: AntiScalar) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, other.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.e4_), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn origin_add_dualNum(self_: Origin, other: DualNum) -> MultiVector {
    let other_groups = dualNum_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.e4_), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn origin_add_flector(self_: Origin, other: Flector) -> Flector {
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e1_, other.e2_, other.e3_, other.e4_ + self_.e4_), 
        /* e423, e431, e412, e321 */ other_groups.group1_
    ));
}
fn origin_add_horizon(self_: Origin, other: Horizon) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.e4_), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other.e321_)
    ));
}
fn origin_add_line(self_: Origin, other: Line) -> MultiVector {
    let other_groups = line_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.e4_), 
        /* e41, e42, e43 */ other_groups.group0_, 
        /* e23, e31, e12 */ other_groups.group1_, 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn origin_add_motor(self_: Origin, other: Motor) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar, other.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.e4_), 
        /* e41, e42, e43 */ vec4<f32>(other.e41_, other.e42_, other.e43_, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(other.e23_, other.e31_, other.e12_, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn origin_add_multiVector(self_: Origin, other: MultiVector) -> MultiVector {
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(other.e1_, other.e2_, other.e3_, other.e4_ + self_.e4_), 
        /* e41, e42, e43 */ other_groups.group2_, 
        /* e23, e31, e12 */ other_groups.group3_, 
        /* e423, e431, e412, e321 */ other_groups.group4_
    ));
}
fn origin_add_origin(self_: Origin, other: Origin) -> Origin {
    return Origin(other.e4_ + self_.e4_);
}
fn origin_add_plane(self_: Origin, other: Plane) -> Flector {
    let other_groups = plane_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.e4_), 
        /* e423, e431, e412, e321 */ other_groups.group0_
    ));
}
fn origin_add_point(self_: Origin, other: Point) -> Point {
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e1_, other.e2_, other.e3_, self_.e4_ + other.e4_)
    ));
}
fn origin_add_scalar(self_: Origin, other: Scalar) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.e4_), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn plane_add_antiScalar(self_: Plane, other: AntiScalar) -> MultiVector {
    let self_groups = plane_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, other.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ self_groups.group0_
    ));
}
fn plane_add_dualNum(self_: Plane, other: DualNum) -> MultiVector {
    let self_groups = plane_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ self_groups.group0_
    ));
}
fn plane_add_flector(self_: Plane, other: Flector) -> Flector {
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ other_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e423_ + self_.e423_, other.e431_ + self_.e431_, other.e412_ + self_.e412_, other.e321_ + self_.e321_)
    ));
}
fn plane_add_horizon(self_: Plane, other: Horizon) -> Plane {
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e423_, self_.e431_, self_.e412_, other.e321_ + self_.e321_)
    ));
}
fn plane_add_line(self_: Plane, other: Line) -> MultiVector {
    let self_groups = plane_grouped(self_);
    let other_groups = line_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ other_groups.group0_, 
        /* e23, e31, e12 */ other_groups.group1_, 
        /* e423, e431, e412, e321 */ self_groups.group0_
    ));
}
fn plane_add_motor(self_: Plane, other: Motor) -> MultiVector {
    let self_groups = plane_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar, other.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(other.e41_, other.e42_, other.e43_, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(other.e23_, other.e31_, other.e12_, 0.0), 
        /* e423, e431, e412, e321 */ self_groups.group0_
    ));
}
fn plane_add_multiVector(self_: Plane, other: MultiVector) -> MultiVector {
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_, 
        /* e1, e2, e3, e4 */ other_groups.group1_, 
        /* e41, e42, e43 */ other_groups.group2_, 
        /* e23, e31, e12 */ other_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e423_ + self_.e423_, other.e431_ + self_.e431_, other.e412_ + self_.e412_, other.e321_ + self_.e321_)
    ));
}
fn plane_add_origin(self_: Plane, other: Origin) -> Flector {
    let self_groups = plane_grouped(self_);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other.e4_), 
        /* e423, e431, e412, e321 */ self_groups.group0_
    ));
}
fn plane_add_plane(self_: Plane, other: Plane) -> Plane {
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(other.e423_ + self_.e423_, other.e431_ + self_.e431_, other.e412_ + self_.e412_, other.e321_ + self_.e321_)
    ));
}
fn plane_add_point(self_: Plane, other: Point) -> Flector {
    let self_groups = plane_grouped(self_);
    let other_groups = point_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ other_groups.group0_, 
        /* e423, e431, e412, e321 */ self_groups.group0_
    ));
}
fn plane_add_scalar(self_: Plane, other: Scalar) -> MultiVector {
    let self_groups = plane_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ self_groups.group0_
    ));
}
fn point_add_antiScalar(self_: Point, other: AntiScalar) -> MultiVector {
    let self_groups = point_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, other.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ self_groups.group0_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn point_add_dualNum(self_: Point, other: DualNum) -> MultiVector {
    let self_groups = point_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_, 
        /* e1, e2, e3, e4 */ self_groups.group0_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn point_add_flector(self_: Point, other: Flector) -> Flector {
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e1_ + self_.e1_, other.e2_ + self_.e2_, other.e3_ + self_.e3_, other.e4_ + self_.e4_), 
        /* e423, e431, e412, e321 */ other_groups.group1_
    ));
}
fn point_add_horizon(self_: Point, other: Horizon) -> Flector {
    let self_groups = point_grouped(self_);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ self_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other.e321_)
    ));
}
fn point_add_line(self_: Point, other: Line) -> MultiVector {
    let self_groups = point_grouped(self_);
    let other_groups = line_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ self_groups.group0_, 
        /* e41, e42, e43 */ other_groups.group0_, 
        /* e23, e31, e12 */ other_groups.group1_, 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn point_add_motor(self_: Point, other: Motor) -> MultiVector {
    let self_groups = point_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar, other.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ self_groups.group0_, 
        /* e41, e42, e43 */ vec4<f32>(other.e41_, other.e42_, other.e43_, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(other.e23_, other.e31_, other.e12_, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn point_add_multiVector(self_: Point, other: MultiVector) -> MultiVector {
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(other.e1_ + self_.e1_, other.e2_ + self_.e2_, other.e3_ + self_.e3_, other.e4_ + self_.e4_), 
        /* e41, e42, e43 */ other_groups.group2_, 
        /* e23, e31, e12 */ other_groups.group3_, 
        /* e423, e431, e412, e321 */ other_groups.group4_
    ));
}
fn point_add_origin(self_: Point, other: Origin) -> Point {
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e1_, self_.e2_, self_.e3_, other.e4_ + self_.e4_)
    ));
}
fn point_add_plane(self_: Point, other: Plane) -> Flector {
    let self_groups = point_grouped(self_);
    let other_groups = plane_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ self_groups.group0_, 
        /* e423, e431, e412, e321 */ other_groups.group0_
    ));
}
fn point_add_point(self_: Point, other: Point) -> Point {
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e1_ + self_.e1_, other.e2_ + self_.e2_, other.e3_ + self_.e3_, other.e4_ + self_.e4_)
    ));
}
fn point_add_scalar(self_: Point, other: Scalar) -> MultiVector {
    let self_groups = point_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ self_groups.group0_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn scalar_add_antiScalar(self_: Scalar, other: AntiScalar) -> DualNum {
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar, other.e1234_, 0.0, 0.0)
    ));
}
fn scalar_add_dualNum(self_: Scalar, other: DualNum) -> DualNum {
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar + self_.scalar, other.e1234_, 0.0, 0.0)
    ));
}
fn scalar_add_flector(self_: Scalar, other: Flector) -> MultiVector {
    let other_groups = flector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ other_groups.group0_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ other_groups.group1_
    ));
}
fn scalar_add_horizon(self_: Scalar, other: Horizon) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other.e321_)
    ));
}
fn scalar_add_line(self_: Scalar, other: Line) -> Motor {
    let other_groups = line_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other_groups.group0_.xyz.xyz, 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(other_groups.group1_.xyz.xyz, self_.scalar)
    ));
}
fn scalar_add_motor(self_: Scalar, other: Motor) -> Motor {
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ other_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(other.e23_, other.e31_, other.e12_, other.scalar + self_.scalar)
    ));
}
fn scalar_add_multiVector(self_: Scalar, other: MultiVector) -> MultiVector {
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar + self_.scalar, other.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ other_groups.group1_, 
        /* e41, e42, e43 */ other_groups.group2_, 
        /* e23, e31, e12 */ other_groups.group3_, 
        /* e423, e431, e412, e321 */ other_groups.group4_
    ));
}
fn scalar_add_origin(self_: Scalar, other: Origin) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other.e4_), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn scalar_add_plane(self_: Scalar, other: Plane) -> MultiVector {
    let other_groups = plane_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ other_groups.group0_
    ));
}
fn scalar_add_point(self_: Scalar, other: Point) -> MultiVector {
    let other_groups = point_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ other_groups.group0_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn scalar_add_scalar(self_: Scalar, other: Scalar) -> Scalar {
    return Scalar(other.scalar + self_.scalar);
}
fn antiScalar_antiAutoMorphism(self_: AntiScalar) -> AntiScalar {
    return self_;
}
fn dualNum_antiAutoMorphism(self_: DualNum) -> DualNum {
    return self_;
}
fn flector_antiAutoMorphism(self_: Flector) -> Flector {
    let self_groups = flector_grouped(self_);    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ self_groups.group0_ * vec4<f32>(-1.0), 
        /* e423, e431, e412, e321 */ self_groups.group1_ * vec4<f32>(-1.0)
    ));
}
fn horizon_antiAutoMorphism(self_: Horizon) -> Horizon {
    return Horizon(self_.e321_ * -1.0);
}
fn line_antiAutoMorphism(self_: Line) -> Line {
    return self_;
}
fn motor_antiAutoMorphism(self_: Motor) -> Motor {
    return self_;
}
fn multiVector_antiAutoMorphism(self_: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_groups.group0_, 
        /* e1, e2, e3, e4 */ self_groups.group1_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ self_groups.group2_, 
        /* e23, e31, e12 */ self_groups.group3_, 
        /* e423, e431, e412, e321 */ self_groups.group4_ * vec4<f32>(-1.0)
    ));
}
fn origin_antiAutoMorphism(self_: Origin) -> Origin {
    return Origin(self_.e4_ * -1.0);
}
fn plane_antiAutoMorphism(self_: Plane) -> Plane {
    let self_groups = plane_grouped(self_);    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ self_groups.group0_ * vec4<f32>(-1.0)
    ));
}
fn point_antiAutoMorphism(self_: Point) -> Point {
    let self_groups = point_grouped(self_);    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ self_groups.group0_ * vec4<f32>(-1.0)
    ));
}
fn scalar_antiAutoMorphism(self_: Scalar) -> Scalar {
    return self_;
}
fn horizon_antiConstraintValid(self_: Horizon) -> Horizon {
    return self_;
}
fn scalar_antiConstraintValid(self_: Scalar) -> Scalar {
    return self_;
}
fn antiScalar_antiConstraintViolation(self_: AntiScalar) -> Scalar {
    return Scalar(0.0);
}
fn dualNum_antiConstraintViolation(self_: DualNum) -> Scalar {
    let geometric_anti_product: DualNum = dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar * self_.e1234_, pow(self_.e1234_, 2), 0.0, 0.0) * vec2<f32>(2.0, 1.0)
    ));
    return Scalar(geometric_anti_product.scalar);
}
fn flector_antiConstraintViolation(self_: Flector) -> DualNum {
    let self_groups = flector_grouped(self_);    let anti_reverse: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ self_groups.group0_ * vec4<f32>(-1.0), 
        /* e423, e431, e412, e321 */ self_groups.group1_
    ));
    let geometric_anti_product: DualNum = dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>((anti_reverse.e321_ * self_.e4_) - (anti_reverse.e1_ * self_.e423_) - (anti_reverse.e2_ * self_.e431_) - (anti_reverse.e3_ * self_.e412_), 0.0, 0.0, 0.0) + ((vec4<f32>(anti_reverse.e423_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e1_, self_.e423_, 0.0, 0.0)) + ((vec4<f32>(anti_reverse.e431_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e2_, self_.e431_, 0.0, 0.0)) + ((vec4<f32>(anti_reverse.e412_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e3_, self_.e412_, 0.0, 0.0)) - ((vec4<f32>(anti_reverse.e4_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e321_, self_.e4_, 0.0, 0.0))
    ));
    let anti_dot_product: AntiScalar = AntiScalar(pow(self_.e423_, 2) + pow(self_.e431_, 2) + pow(self_.e412_, 2) - pow(self_.e4_, 2));
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(geometric_anti_product.scalar, geometric_anti_product.e1234_ - anti_dot_product.e1234_, 0.0, 0.0)
    ));
}
fn line_antiConstraintViolation(self_: Line) -> DualNum {
    let self_groups = line_grouped(self_);    let anti_reverse: Line = line_degroup(LineGroups(
        /* e41, e42, e43 */ self_groups.group0_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ self_groups.group1_ * vec3<f32>(-1.0)
    ));
    let geometric_anti_product: DualNum = dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(-(anti_reverse.e23_ * self_.e41_) - (anti_reverse.e31_ * self_.e42_) - (anti_reverse.e12_ * self_.e43_), 0.0, 0.0, 0.0) - ((vec4<f32>(anti_reverse.e41_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e23_, self_.e41_, 0.0, 0.0)) - ((vec4<f32>(anti_reverse.e42_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e31_, self_.e42_, 0.0, 0.0)) - ((vec4<f32>(anti_reverse.e43_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e12_, self_.e43_, 0.0, 0.0))
    ));
    let anti_dot_product: AntiScalar = AntiScalar(-pow(self_.e41_, 2) - pow(self_.e42_, 2) - pow(self_.e43_, 2));
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(geometric_anti_product.scalar, geometric_anti_product.e1234_ - anti_dot_product.e1234_, 0.0, 0.0)
    ));
}
fn motor_antiConstraintViolation(self_: Motor) -> DualNum {
    let self_groups = motor_grouped(self_);    let anti_reverse: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ self_groups.group0_ * vec4<f32>(-1.0, -1.0, -1.0, 1.0), 
        /* e23, e31, e12, scalar */ self_groups.group1_ * vec4<f32>(-1.0, -1.0, -1.0, 1.0)
    ));
    let geometric_anti_product: DualNum = dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>((anti_reverse.scalar * self_.e1234_) - (anti_reverse.e23_ * self_.e41_) - (anti_reverse.e31_ * self_.e42_) - (anti_reverse.e12_ * self_.e43_), 0.0, 0.0, 0.0) + ((vec4<f32>(anti_reverse.e1234_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.scalar, self_.e1234_, 0.0, 0.0)) - ((vec4<f32>(anti_reverse.e41_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e23_, self_.e41_, 0.0, 0.0)) - ((vec4<f32>(anti_reverse.e42_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e31_, self_.e42_, 0.0, 0.0)) - ((vec4<f32>(anti_reverse.e43_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e12_, self_.e43_, 0.0, 0.0))
    ));
    let anti_dot_product: AntiScalar = AntiScalar(pow(self_.e1234_, 2) - pow(self_.e41_, 2) - pow(self_.e42_, 2) - pow(self_.e43_, 2));
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(geometric_anti_product.scalar, geometric_anti_product.e1234_ - anti_dot_product.e1234_, 0.0, 0.0)
    ));
}
fn multiVector_antiConstraintViolation(self_: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);    let anti_reverse_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ self_groups.group0_, 
        /* e1, e2, e3, e4 */ self_groups.group1_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ self_groups.group2_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ self_groups.group3_ * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ self_groups.group4_
    );
    let anti_reverse: MultiVector = multiVector_degroup(anti_reverse_groups);
    let geometric_anti_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((anti_reverse.e1234_ * self_.scalar) + (anti_reverse.e321_ * self_.e4_) - (anti_reverse.e1_ * self_.e423_) - (anti_reverse.e2_ * self_.e431_) - (anti_reverse.e3_ * self_.e412_) - (anti_reverse.e23_ * self_.e41_) - (anti_reverse.e31_ * self_.e42_) - (anti_reverse.e12_ * self_.e43_), 0.0, 0.0, 0.0) + ((vec4<f32>(anti_reverse.e423_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e1_, self_.e423_, 0.0, 0.0)) + ((vec4<f32>(anti_reverse.e431_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e2_, self_.e431_, 0.0, 0.0)) + ((vec4<f32>(anti_reverse.e412_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e3_, self_.e412_, 0.0, 0.0)) + ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * anti_reverse_groups.group0_) - ((vec4<f32>(anti_reverse.e4_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e321_, self_.e4_, 0.0, 0.0)) - ((vec4<f32>(anti_reverse.e41_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e23_, self_.e41_, 0.0, 0.0)) - ((vec4<f32>(anti_reverse.e42_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e31_, self_.e42_, 0.0, 0.0)) - ((vec4<f32>(anti_reverse.e43_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e12_, self_.e43_, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>((anti_reverse.e41_ * self_.e4_) + (anti_reverse.e43_ * self_.e431_) + (anti_reverse.e412_ * self_.e42_) - (anti_reverse.e42_ * self_.e412_) - (anti_reverse.e431_ * self_.e43_), (anti_reverse.e41_ * self_.e412_) + (anti_reverse.e42_ * self_.e4_) + (anti_reverse.e423_ * self_.e43_) - (anti_reverse.e43_ * self_.e423_) - (anti_reverse.e412_ * self_.e41_), (anti_reverse.e42_ * self_.e423_) + (anti_reverse.e43_ * self_.e4_) + (anti_reverse.e431_ * self_.e41_) - (anti_reverse.e41_ * self_.e431_) - (anti_reverse.e423_ * self_.e42_), (anti_reverse.e23_ * self_.e423_) + (anti_reverse.e31_ * self_.e431_) + (anti_reverse.e12_ * self_.e412_) - (anti_reverse.scalar * self_.e4_) - (anti_reverse.e1_ * self_.e41_) - (anti_reverse.e2_ * self_.e42_) - (anti_reverse.e3_ * self_.e43_) - (anti_reverse.e41_ * self_.e1_) - (anti_reverse.e42_ * self_.e2_) - (anti_reverse.e43_ * self_.e3_) - (anti_reverse.e423_ * self_.e23_) - (anti_reverse.e431_ * self_.e31_) - (anti_reverse.e412_ * self_.e12_)) + (vec4<f32>(anti_reverse.e1234_) * self_groups.group4_) + (vec4<f32>(anti_reverse.e4_) * vec4<f32>(self_.e41_, self_.e42_, self_.e43_, self_.scalar)) + (vec4<f32>(self_.e1234_) * anti_reverse_groups.group4_)
    );
    let geometric_anti_product: MultiVector = multiVector_degroup(geometric_anti_product_groups);
    let anti_dot_product: AntiScalar = AntiScalar(pow(self_.e1234_, 2) + pow(self_.e423_, 2) + pow(self_.e431_, 2) + pow(self_.e412_, 2) - pow(self_.e4_, 2) - pow(self_.e41_, 2) - pow(self_.e42_, 2) - pow(self_.e43_, 2));
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(geometric_anti_product.scalar, geometric_anti_product.e1234_ - anti_dot_product.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ geometric_anti_product_groups.group4_
    ));
}
fn origin_antiConstraintViolation(self_: Origin) -> AntiScalar {
    let anti_reverse: Origin = Origin(self_.e4_ * -1.0);
    let geometric_anti_product: AntiScalar = AntiScalar(anti_reverse.e4_ * self_.e4_ * -1.0);
    let anti_dot_product: AntiScalar = AntiScalar(pow(self_.e4_, 2) * -1.0);
    return AntiScalar(geometric_anti_product.e1234_ - anti_dot_product.e1234_);
}
fn plane_antiConstraintViolation(self_: Plane) -> Scalar {
    return Scalar(0.0);
}
fn point_antiConstraintViolation(self_: Point) -> AntiScalar {
    let self_groups = point_grouped(self_);    let anti_reverse: Point = point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ self_groups.group0_ * vec4<f32>(-1.0)
    ));
    let geometric_anti_product: AntiScalar = AntiScalar(anti_reverse.e4_ * self_.e4_ * -1.0);
    let anti_dot_product: AntiScalar = AntiScalar(pow(self_.e4_, 2) * -1.0);
    return AntiScalar(geometric_anti_product.e1234_ - anti_dot_product.e1234_);
}
fn antiScalar_antiDotProduct_antiScalar(self_: AntiScalar, other: AntiScalar) -> AntiScalar {
    return AntiScalar(other.e1234_ * self_.e1234_);
}
fn antiScalar_antiDotProduct_dualNum(self_: AntiScalar, other: DualNum) -> AntiScalar {
    return AntiScalar(self_.e1234_ * other.e1234_);
}
fn antiScalar_antiDotProduct_motor(self_: AntiScalar, other: Motor) -> AntiScalar {
    return AntiScalar(self_.e1234_ * other.e1234_);
}
fn antiScalar_antiDotProduct_multiVector(self_: AntiScalar, other: MultiVector) -> AntiScalar {
    return AntiScalar(self_.e1234_ * other.e1234_);
}
fn dualNum_antiDotProduct_antiScalar(self_: DualNum, other: AntiScalar) -> AntiScalar {
    return AntiScalar(other.e1234_ * self_.e1234_);
}
fn dualNum_antiDotProduct_dualNum(self_: DualNum, other: DualNum) -> AntiScalar {
    return AntiScalar(other.e1234_ * self_.e1234_);
}
fn dualNum_antiDotProduct_motor(self_: DualNum, other: Motor) -> AntiScalar {
    return AntiScalar(self_.e1234_ * other.e1234_);
}
fn dualNum_antiDotProduct_multiVector(self_: DualNum, other: MultiVector) -> AntiScalar {
    return AntiScalar(self_.e1234_ * other.e1234_);
}
fn flector_antiDotProduct_flector(self_: Flector, other: Flector) -> AntiScalar {
    return AntiScalar((other.e423_ * self_.e423_) + (other.e431_ * self_.e431_) + (other.e412_ * self_.e412_) - (other.e4_ * self_.e4_));
}
fn flector_antiDotProduct_multiVector(self_: Flector, other: MultiVector) -> AntiScalar {
    return AntiScalar((self_.e423_ * other.e423_) + (self_.e431_ * other.e431_) + (self_.e412_ * other.e412_) - (self_.e4_ * other.e4_));
}
fn flector_antiDotProduct_origin(self_: Flector, other: Origin) -> AntiScalar {
    return AntiScalar(self_.e4_ * other.e4_ * -1.0);
}
fn flector_antiDotProduct_plane(self_: Flector, other: Plane) -> AntiScalar {
    return AntiScalar((self_.e423_ * other.e423_) + (self_.e431_ * other.e431_) + (self_.e412_ * other.e412_));
}
fn flector_antiDotProduct_point(self_: Flector, other: Point) -> AntiScalar {
    return AntiScalar(self_.e4_ * other.e4_ * -1.0);
}
fn line_antiDotProduct_line(self_: Line, other: Line) -> AntiScalar {
    return AntiScalar(-(other.e41_ * self_.e41_) - (other.e42_ * self_.e42_) - (other.e43_ * self_.e43_));
}
fn line_antiDotProduct_motor(self_: Line, other: Motor) -> AntiScalar {
    return AntiScalar(-(self_.e41_ * other.e41_) - (self_.e42_ * other.e42_) - (self_.e43_ * other.e43_));
}
fn line_antiDotProduct_multiVector(self_: Line, other: MultiVector) -> AntiScalar {
    return AntiScalar(-(self_.e41_ * other.e41_) - (self_.e42_ * other.e42_) - (self_.e43_ * other.e43_));
}
fn motor_antiDotProduct_antiScalar(self_: Motor, other: AntiScalar) -> AntiScalar {
    return AntiScalar(other.e1234_ * self_.e1234_);
}
fn motor_antiDotProduct_dualNum(self_: Motor, other: DualNum) -> AntiScalar {
    return AntiScalar(other.e1234_ * self_.e1234_);
}
fn motor_antiDotProduct_line(self_: Motor, other: Line) -> AntiScalar {
    return AntiScalar(-(other.e41_ * self_.e41_) - (other.e42_ * self_.e42_) - (other.e43_ * self_.e43_));
}
fn motor_antiDotProduct_motor(self_: Motor, other: Motor) -> AntiScalar {
    return AntiScalar((other.e1234_ * self_.e1234_) - (other.e41_ * self_.e41_) - (other.e42_ * self_.e42_) - (other.e43_ * self_.e43_));
}
fn motor_antiDotProduct_multiVector(self_: Motor, other: MultiVector) -> AntiScalar {
    return AntiScalar((self_.e1234_ * other.e1234_) - (self_.e41_ * other.e41_) - (self_.e42_ * other.e42_) - (self_.e43_ * other.e43_));
}
fn multiVector_antiDotProduct_antiScalar(self_: MultiVector, other: AntiScalar) -> AntiScalar {
    return AntiScalar(other.e1234_ * self_.e1234_);
}
fn multiVector_antiDotProduct_dualNum(self_: MultiVector, other: DualNum) -> AntiScalar {
    return AntiScalar(other.e1234_ * self_.e1234_);
}
fn multiVector_antiDotProduct_flector(self_: MultiVector, other: Flector) -> AntiScalar {
    return AntiScalar((other.e423_ * self_.e423_) + (other.e431_ * self_.e431_) + (other.e412_ * self_.e412_) - (other.e4_ * self_.e4_));
}
fn multiVector_antiDotProduct_line(self_: MultiVector, other: Line) -> AntiScalar {
    return AntiScalar(-(other.e41_ * self_.e41_) - (other.e42_ * self_.e42_) - (other.e43_ * self_.e43_));
}
fn multiVector_antiDotProduct_motor(self_: MultiVector, other: Motor) -> AntiScalar {
    return AntiScalar((other.e1234_ * self_.e1234_) - (other.e41_ * self_.e41_) - (other.e42_ * self_.e42_) - (other.e43_ * self_.e43_));
}
fn multiVector_antiDotProduct_multiVector(self_: MultiVector, other: MultiVector) -> AntiScalar {
    return AntiScalar((other.e1234_ * self_.e1234_) + (other.e423_ * self_.e423_) + (other.e431_ * self_.e431_) + (other.e412_ * self_.e412_) - (other.e4_ * self_.e4_) - (other.e41_ * self_.e41_) - (other.e42_ * self_.e42_) - (other.e43_ * self_.e43_));
}
fn multiVector_antiDotProduct_origin(self_: MultiVector, other: Origin) -> AntiScalar {
    return AntiScalar(self_.e4_ * other.e4_ * -1.0);
}
fn multiVector_antiDotProduct_plane(self_: MultiVector, other: Plane) -> AntiScalar {
    return AntiScalar((self_.e423_ * other.e423_) + (self_.e431_ * other.e431_) + (self_.e412_ * other.e412_));
}
fn multiVector_antiDotProduct_point(self_: MultiVector, other: Point) -> AntiScalar {
    return AntiScalar(self_.e4_ * other.e4_ * -1.0);
}
fn origin_antiDotProduct_flector(self_: Origin, other: Flector) -> AntiScalar {
    return AntiScalar(other.e4_ * self_.e4_ * -1.0);
}
fn origin_antiDotProduct_multiVector(self_: Origin, other: MultiVector) -> AntiScalar {
    return AntiScalar(other.e4_ * self_.e4_ * -1.0);
}
fn origin_antiDotProduct_origin(self_: Origin, other: Origin) -> AntiScalar {
    return AntiScalar(other.e4_ * self_.e4_ * -1.0);
}
fn origin_antiDotProduct_point(self_: Origin, other: Point) -> AntiScalar {
    return AntiScalar(self_.e4_ * other.e4_ * -1.0);
}
fn plane_antiDotProduct_flector(self_: Plane, other: Flector) -> AntiScalar {
    return AntiScalar((other.e423_ * self_.e423_) + (other.e431_ * self_.e431_) + (other.e412_ * self_.e412_));
}
fn plane_antiDotProduct_multiVector(self_: Plane, other: MultiVector) -> AntiScalar {
    return AntiScalar((other.e423_ * self_.e423_) + (other.e431_ * self_.e431_) + (other.e412_ * self_.e412_));
}
fn plane_antiDotProduct_plane(self_: Plane, other: Plane) -> AntiScalar {
    return AntiScalar((other.e423_ * self_.e423_) + (other.e431_ * self_.e431_) + (other.e412_ * self_.e412_));
}
fn point_antiDotProduct_flector(self_: Point, other: Flector) -> AntiScalar {
    return AntiScalar(other.e4_ * self_.e4_ * -1.0);
}
fn point_antiDotProduct_multiVector(self_: Point, other: MultiVector) -> AntiScalar {
    return AntiScalar(other.e4_ * self_.e4_ * -1.0);
}
fn point_antiDotProduct_origin(self_: Point, other: Origin) -> AntiScalar {
    return AntiScalar(other.e4_ * self_.e4_ * -1.0);
}
fn point_antiDotProduct_point(self_: Point, other: Point) -> AntiScalar {
    return AntiScalar(other.e4_ * self_.e4_ * -1.0);
}
fn antiScalar_antiFix(self_: AntiScalar) -> AntiScalar {
    let geometric_anti_product: AntiScalar = AntiScalar(pow(self_.e1234_, 2));
    let anti_square_root: AntiScalar = AntiScalar(pow(geometric_anti_product.e1234_, 0.5));
    let anti_dot_product: AntiScalar = AntiScalar(pow(anti_square_root.e1234_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return AntiScalar(anti_inverse.e1234_ * self_.e1234_);
}
fn origin_antiFix(self_: Origin) -> Origin {
    let anti_reverse: Origin = Origin(self_.e4_ * -1.0);
    let geometric_anti_product: AntiScalar = AntiScalar(anti_reverse.e4_ * self_.e4_ * -1.0);
    let anti_square_root: AntiScalar = AntiScalar(pow(geometric_anti_product.e1234_, 0.5));
    let anti_dot_product: AntiScalar = AntiScalar(pow(anti_square_root.e1234_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return Origin(anti_inverse.e1234_ * self_.e4_);
}
fn plane_antiFix(self_: Plane) -> Plane {
    let self_groups = plane_grouped(self_);    let geometric_anti_product: AntiScalar = AntiScalar(pow(self_.e423_, 2) + pow(self_.e431_, 2) + pow(self_.e412_, 2));
    let anti_square_root: AntiScalar = AntiScalar(pow(geometric_anti_product.e1234_, 0.5));
    let anti_dot_product: AntiScalar = AntiScalar(pow(anti_square_root.e1234_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group0_
    ));
}
fn point_antiFix(self_: Point) -> Point {
    let self_groups = point_grouped(self_);    let anti_reverse: Point = point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ self_groups.group0_ * vec4<f32>(-1.0)
    ));
    let geometric_anti_product: AntiScalar = AntiScalar(anti_reverse.e4_ * self_.e4_ * -1.0);
    let anti_square_root: AntiScalar = AntiScalar(pow(geometric_anti_product.e1234_, 0.5));
    let anti_dot_product: AntiScalar = AntiScalar(pow(anti_square_root.e1234_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group0_
    ));
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
    let anti_dot_product: AntiScalar = AntiScalar(pow(self_.e1234_, 2));
    return AntiScalar(1.0/(anti_dot_product.e1234_));
}
fn dualNum_antiInverse(self_: DualNum) -> AntiScalar {
    let anti_dot_product: AntiScalar = AntiScalar(pow(self_.e1234_, 2));
    return AntiScalar(1.0/(anti_dot_product.e1234_));
}
fn flector_antiInverse(self_: Flector) -> AntiScalar {
    let anti_dot_product: AntiScalar = AntiScalar(pow(self_.e423_, 2) + pow(self_.e431_, 2) + pow(self_.e412_, 2) - pow(self_.e4_, 2));
    return AntiScalar(1.0/(anti_dot_product.e1234_));
}
fn line_antiInverse(self_: Line) -> AntiScalar {
    let anti_dot_product: AntiScalar = AntiScalar(-pow(self_.e41_, 2) - pow(self_.e42_, 2) - pow(self_.e43_, 2));
    return AntiScalar(1.0/(anti_dot_product.e1234_));
}
fn motor_antiInverse(self_: Motor) -> AntiScalar {
    let anti_dot_product: AntiScalar = AntiScalar(pow(self_.e1234_, 2) - pow(self_.e41_, 2) - pow(self_.e42_, 2) - pow(self_.e43_, 2));
    return AntiScalar(1.0/(anti_dot_product.e1234_));
}
fn multiVector_antiInverse(self_: MultiVector) -> AntiScalar {
    let anti_dot_product: AntiScalar = AntiScalar(pow(self_.e1234_, 2) + pow(self_.e423_, 2) + pow(self_.e431_, 2) + pow(self_.e412_, 2) - pow(self_.e4_, 2) - pow(self_.e41_, 2) - pow(self_.e42_, 2) - pow(self_.e43_, 2));
    return AntiScalar(1.0/(anti_dot_product.e1234_));
}
fn origin_antiInverse(self_: Origin) -> AntiScalar {
    let anti_dot_product: AntiScalar = AntiScalar(pow(self_.e4_, 2) * -1.0);
    return AntiScalar(1.0/(anti_dot_product.e1234_));
}
fn plane_antiInverse(self_: Plane) -> AntiScalar {
    let anti_dot_product: AntiScalar = AntiScalar(pow(self_.e423_, 2) + pow(self_.e431_, 2) + pow(self_.e412_, 2));
    return AntiScalar(1.0/(anti_dot_product.e1234_));
}
fn point_antiInverse(self_: Point) -> AntiScalar {
    let anti_dot_product: AntiScalar = AntiScalar(pow(self_.e4_, 2) * -1.0);
    return AntiScalar(1.0/(anti_dot_product.e1234_));
}
fn antiScalar_antiOne() -> AntiScalar {
    return AntiScalar(1.0);
}
fn dualNum_antiOne() -> DualNum {
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, 1.0, 0.0, 0.0)
    ));
}
fn motor_antiOne() -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, 1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0)
    ));
}
fn multiVector_antiOne() -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, 1.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn antiScalar_antiReverse(self_: AntiScalar) -> AntiScalar {
    return self_;
}
fn dualNum_antiReverse(self_: DualNum) -> DualNum {
    return self_;
}
fn flector_antiReverse(self_: Flector) -> Flector {
    let self_groups = flector_grouped(self_);    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ self_groups.group0_ * vec4<f32>(-1.0), 
        /* e423, e431, e412, e321 */ self_groups.group1_
    ));
}
fn horizon_antiReverse(self_: Horizon) -> Horizon {
    return self_;
}
fn line_antiReverse(self_: Line) -> Line {
    let self_groups = line_grouped(self_);    return line_degroup(LineGroups(
        /* e41, e42, e43 */ self_groups.group0_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ self_groups.group1_ * vec3<f32>(-1.0)
    ));
}
fn motor_antiReverse(self_: Motor) -> Motor {
    let self_groups = motor_grouped(self_);    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ self_groups.group0_ * vec4<f32>(-1.0, -1.0, -1.0, 1.0), 
        /* e23, e31, e12, scalar */ self_groups.group1_ * vec4<f32>(-1.0, -1.0, -1.0, 1.0)
    ));
}
fn multiVector_antiReverse(self_: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_groups.group0_, 
        /* e1, e2, e3, e4 */ self_groups.group1_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ self_groups.group2_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ self_groups.group3_ * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ self_groups.group4_
    ));
}
fn origin_antiReverse(self_: Origin) -> Origin {
    return Origin(self_.e4_ * -1.0);
}
fn plane_antiReverse(self_: Plane) -> Plane {
    return self_;
}
fn point_antiReverse(self_: Point) -> Point {
    let self_groups = point_grouped(self_);    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ self_groups.group0_ * vec4<f32>(-1.0)
    ));
}
fn scalar_antiReverse(self_: Scalar) -> Scalar {
    return self_;
}
fn antiScalar_antiSquareRoot(self_: AntiScalar) -> AntiScalar {
    return AntiScalar(pow(self_.e1234_, 0.5));
}
fn antiScalar_antiWedge_antiScalar(self_: AntiScalar, other: AntiScalar) -> AntiScalar {
    return AntiScalar(other.e1234_ * self_.e1234_);
}
fn antiScalar_antiWedge_dualNum(self_: AntiScalar, other: DualNum) -> DualNum {
    let other_groups = dualNum_grouped(other);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ (vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * other_groups.group0_
    ));
}
fn antiScalar_antiWedge_flector(self_: AntiScalar, other: Flector) -> Flector {
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e1234_) * other_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e1234_) * other_groups.group1_
    ));
}
fn antiScalar_antiWedge_horizon(self_: AntiScalar, other: Horizon) -> Horizon {
    return Horizon(self_.e1234_ * other.e321_);
}
fn antiScalar_antiWedge_line(self_: AntiScalar, other: Line) -> Line {
    let other_groups = line_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group1_
    ));
}
fn antiScalar_antiWedge_motor(self_: AntiScalar, other: Motor) -> Motor {
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.e1234_) * other_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.e1234_) * other_groups.group1_
    ));
}
fn antiScalar_antiWedge_multiVector(self_: AntiScalar, other: MultiVector) -> MultiVector {
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ (vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * other_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e1234_) * other_groups.group1_, 
        /* e41, e42, e43 */ (vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group2_, 
        /* e23, e31, e12 */ (vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e1234_) * other_groups.group4_
    ));
}
fn antiScalar_antiWedge_origin(self_: AntiScalar, other: Origin) -> Origin {
    return Origin(self_.e1234_ * other.e4_);
}
fn antiScalar_antiWedge_plane(self_: AntiScalar, other: Plane) -> Plane {
    let other_groups = plane_grouped(other);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e1234_) * other_groups.group0_
    ));
}
fn antiScalar_antiWedge_point(self_: AntiScalar, other: Point) -> Point {
    let other_groups = point_grouped(other);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e1234_) * other_groups.group0_
    ));
}
fn antiScalar_antiWedge_scalar(self_: AntiScalar, other: Scalar) -> Scalar {
    return Scalar(self_.e1234_ * other.scalar);
}
fn dualNum_antiWedge_antiScalar(self_: DualNum, other: AntiScalar) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ (vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_
    ));
}
fn dualNum_antiWedge_dualNum(self_: DualNum, other: DualNum) -> DualNum {
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>((other.scalar * self_.e1234_) + (other.e1234_ * self_.scalar), other.e1234_ * self_.e1234_, 0.0, 0.0)
    ));
}
fn dualNum_antiWedge_flector(self_: DualNum, other: Flector) -> Flector {
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e1234_) * other_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e1234_) * other_groups.group1_
    ));
}
fn dualNum_antiWedge_horizon(self_: DualNum, other: Horizon) -> Horizon {
    return Horizon(self_.e1234_ * other.e321_);
}
fn dualNum_antiWedge_line(self_: DualNum, other: Line) -> Line {
    let other_groups = line_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group1_
    ));
}
fn dualNum_antiWedge_motor(self_: DualNum, other: Motor) -> Motor {
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.e1234_) * other_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.e1234_ * other.e23_, self_.e1234_ * other.e31_, self_.e1234_ * other.e12_, (self_.scalar * other.e1234_) + (self_.e1234_ * other.scalar))
    ));
}
fn dualNum_antiWedge_multiVector(self_: DualNum, other: MultiVector) -> MultiVector {
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((self_.scalar * other.e1234_) + (self_.e1234_ * other.scalar), self_.e1234_ * other.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e1234_) * other_groups.group1_, 
        /* e41, e42, e43 */ (vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group2_, 
        /* e23, e31, e12 */ (vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e1234_) * other_groups.group4_
    ));
}
fn dualNum_antiWedge_origin(self_: DualNum, other: Origin) -> Origin {
    return Origin(self_.e1234_ * other.e4_);
}
fn dualNum_antiWedge_plane(self_: DualNum, other: Plane) -> Plane {
    let other_groups = plane_grouped(other);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e1234_) * other_groups.group0_
    ));
}
fn dualNum_antiWedge_point(self_: DualNum, other: Point) -> Point {
    let other_groups = point_grouped(other);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e1234_) * other_groups.group0_
    ));
}
fn dualNum_antiWedge_scalar(self_: DualNum, other: Scalar) -> Scalar {
    return Scalar(self_.e1234_ * other.scalar);
}
fn flector_antiWedge_antiScalar(self_: Flector, other: AntiScalar) -> Flector {
    let self_groups = flector_grouped(self_);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e1234_) * self_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e1234_) * self_groups.group1_
    ));
}
fn flector_antiWedge_dualNum(self_: Flector, other: DualNum) -> Flector {
    let self_groups = flector_grouped(self_);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e1234_) * self_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e1234_) * self_groups.group1_
    ));
}
fn flector_antiWedge_flector(self_: Flector, other: Flector) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other.e431_ * self_.e412_) - (other.e412_ * self_.e431_), (other.e412_ * self_.e423_) - (other.e423_ * self_.e412_), (other.e423_ * self_.e431_) - (other.e431_ * self_.e423_), 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, (other.e423_ * self_.e1_) + (other.e431_ * self_.e2_) + (other.e412_ * self_.e3_) - (other.e1_ * self_.e423_) - (other.e2_ * self_.e431_) - (other.e3_ * self_.e412_)) + (vec4<f32>(other.e321_) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, self_.e4_)) - (vec4<f32>(self_.e321_) * vec4<f32>(other.e423_, other.e431_, other.e412_, other.e4_))
    ));
}
fn flector_antiWedge_horizon(self_: Flector, other: Horizon) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(other.e321_) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, self_.e4_)
    ));
}
fn flector_antiWedge_line(self_: Flector, other: Line) -> Point {
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e412_ * other.e31_) + (self_.e321_ * other.e41_) - (self_.e431_ * other.e12_), (self_.e423_ * other.e12_) + (self_.e321_ * other.e42_) - (self_.e412_ * other.e23_), (self_.e431_ * other.e23_) + (self_.e321_ * other.e43_) - (self_.e423_ * other.e31_), -(self_.e423_ * other.e41_) - (self_.e431_ * other.e42_) - (self_.e412_ * other.e43_))
    ));
}
fn flector_antiWedge_motor(self_: Flector, other: Motor) -> Flector {
    let self_groups = flector_grouped(self_);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e412_ * other.e31_) + (self_.e321_ * other.e41_) - (self_.e431_ * other.e12_), (self_.e423_ * other.e12_) + (self_.e321_ * other.e42_) - (self_.e412_ * other.e23_), (self_.e431_ * other.e23_) + (self_.e321_ * other.e43_) - (self_.e423_ * other.e31_), -(self_.e423_ * other.e41_) - (self_.e431_ * other.e42_) - (self_.e412_ * other.e43_)) + (vec4<f32>(other.e1234_) * self_groups.group0_), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e1234_) * self_groups.group1_
    ));
}
fn flector_antiWedge_multiVector(self_: Flector, other: MultiVector) -> MultiVector {
    let self_groups = flector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((self_.e1_ * other.e423_) + (self_.e2_ * other.e431_) + (self_.e3_ * other.e412_) + (self_.e4_ * other.e321_) - (self_.e423_ * other.e1_) - (self_.e431_ * other.e2_) - (self_.e412_ * other.e3_) - (self_.e321_ * other.e4_), 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e412_ * other.e31_) + (self_.e321_ * other.e41_) - (self_.e431_ * other.e12_), (self_.e423_ * other.e12_) + (self_.e321_ * other.e42_) - (self_.e412_ * other.e23_), (self_.e431_ * other.e23_) + (self_.e321_ * other.e43_) - (self_.e423_ * other.e31_), -(self_.e423_ * other.e41_) - (self_.e431_ * other.e42_) - (self_.e412_ * other.e43_)) + (vec4<f32>(other.e1234_) * self_groups.group0_), 
        /* e41, e42, e43 */ vec4<f32>((self_.e412_ * other.e431_) - (self_.e431_ * other.e412_), (self_.e423_ * other.e412_) - (self_.e412_ * other.e423_), (self_.e431_ * other.e423_) - (self_.e423_ * other.e431_), 0.0), 
        /* e23, e31, e12 */ ((vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0)) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e1234_) * self_groups.group1_
    ));
}
fn flector_antiWedge_origin(self_: Flector, other: Origin) -> Scalar {
    return Scalar(self_.e321_ * other.e4_ * -1.0);
}
fn flector_antiWedge_plane(self_: Flector, other: Plane) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((self_.e412_ * other.e431_) - (self_.e431_ * other.e412_), (self_.e423_ * other.e412_) - (self_.e412_ * other.e423_), (self_.e431_ * other.e423_) - (self_.e423_ * other.e431_), 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>((self_.e321_ * other.e423_) * -1.0, (self_.e321_ * other.e431_) * -1.0, (self_.e321_ * other.e412_) * -1.0, (self_.e1_ * other.e423_) + (self_.e2_ * other.e431_) + (self_.e3_ * other.e412_)) + (vec4<f32>(other.e321_) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, self_.e4_))
    ));
}
fn flector_antiWedge_point(self_: Flector, other: Point) -> Scalar {
    return Scalar(-(self_.e423_ * other.e1_) - (self_.e431_ * other.e2_) - (self_.e412_ * other.e3_) - (self_.e321_ * other.e4_));
}
fn horizon_antiWedge_antiScalar(self_: Horizon, other: AntiScalar) -> Horizon {
    return Horizon(other.e1234_ * self_.e321_);
}
fn horizon_antiWedge_dualNum(self_: Horizon, other: DualNum) -> Horizon {
    return Horizon(other.e1234_ * self_.e321_);
}
fn horizon_antiWedge_flector(self_: Horizon, other: Flector) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.e321_) * vec4<f32>(other.e423_, other.e431_, other.e412_, other.e4_) * vec4<f32>(-1.0)
    ));
}
fn horizon_antiWedge_line(self_: Horizon, other: Line) -> Point {
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e321_ * other.e41_, self_.e321_ * other.e42_, self_.e321_ * other.e43_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
}
fn horizon_antiWedge_motor(self_: Horizon, other: Motor) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e321_ * other.e41_, self_.e321_ * other.e42_, self_.e321_ * other.e43_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0, 1.0, 1.0, self_.e321_ * other.e1234_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)
    ));
}
fn horizon_antiWedge_multiVector(self_: Horizon, other: MultiVector) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.e321_ * other.e4_, 1.0, 0.0, 0.0) * vec2<f32>(-1.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e321_ * other.e41_, self_.e321_ * other.e42_, self_.e321_ * other.e43_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ (vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0) * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0, 1.0, 1.0, self_.e321_ * other.e1234_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)
    ));
}
fn horizon_antiWedge_origin(self_: Horizon, other: Origin) -> Scalar {
    return Scalar(self_.e321_ * other.e4_ * -1.0);
}
fn horizon_antiWedge_plane(self_: Horizon, other: Plane) -> Line {
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ (vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0) * vec3<f32>(-1.0)
    ));
}
fn horizon_antiWedge_point(self_: Horizon, other: Point) -> Scalar {
    return Scalar(self_.e321_ * other.e4_ * -1.0);
}
fn line_antiWedge_antiScalar(self_: Line, other: AntiScalar) -> Line {
    let self_groups = line_grouped(self_);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_
    ));
}
fn line_antiWedge_dualNum(self_: Line, other: DualNum) -> Line {
    let self_groups = line_grouped(self_);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_
    ));
}
fn line_antiWedge_flector(self_: Line, other: Flector) -> Point {
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other.e412_ * self_.e31_) + (other.e321_ * self_.e41_) - (other.e431_ * self_.e12_), (other.e423_ * self_.e12_) + (other.e321_ * self_.e42_) - (other.e412_ * self_.e23_), (other.e431_ * self_.e23_) + (other.e321_ * self_.e43_) - (other.e423_ * self_.e31_), -(other.e423_ * self_.e41_) - (other.e431_ * self_.e42_) - (other.e412_ * self_.e43_))
    ));
}
fn line_antiWedge_horizon(self_: Line, other: Horizon) -> Point {
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e321_ * self_.e41_, other.e321_ * self_.e42_, other.e321_ * self_.e43_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
}
fn line_antiWedge_line(self_: Line, other: Line) -> Scalar {
    return Scalar(-(other.e41_ * self_.e23_) - (other.e42_ * self_.e31_) - (other.e43_ * self_.e12_) - (other.e23_ * self_.e41_) - (other.e31_ * self_.e42_) - (other.e12_ * self_.e43_));
}
fn line_antiWedge_motor(self_: Line, other: Motor) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.e41_ * other.e1234_, self_.e42_ * other.e1234_, self_.e43_ * other.e1234_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.e23_ * other.e1234_, self_.e31_ * other.e1234_, self_.e12_ * other.e1234_, -(self_.e41_ * other.e23_) - (self_.e42_ * other.e31_) - (self_.e43_ * other.e12_) - (self_.e23_ * other.e41_) - (self_.e31_ * other.e42_) - (self_.e12_ * other.e43_))
    ));
}
fn line_antiWedge_multiVector(self_: Line, other: MultiVector) -> MultiVector {
    let self_groups = line_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(-(self_.e41_ * other.e23_) - (self_.e42_ * other.e31_) - (self_.e43_ * other.e12_) - (self_.e23_ * other.e41_) - (self_.e31_ * other.e42_) - (self_.e12_ * other.e43_), 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e41_ * other.e321_) + (self_.e31_ * other.e412_) - (self_.e12_ * other.e431_), (self_.e42_ * other.e321_) + (self_.e12_ * other.e423_) - (self_.e23_ * other.e412_), (self_.e43_ * other.e321_) + (self_.e23_ * other.e431_) - (self_.e31_ * other.e423_), -(self_.e41_ * other.e423_) - (self_.e42_ * other.e431_) - (self_.e43_ * other.e412_)), 
        /* e41, e42, e43 */ (vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_, 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn line_antiWedge_plane(self_: Line, other: Plane) -> Point {
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e41_ * other.e321_) + (self_.e31_ * other.e412_) - (self_.e12_ * other.e431_), (self_.e42_ * other.e321_) + (self_.e12_ * other.e423_) - (self_.e23_ * other.e412_), (self_.e43_ * other.e321_) + (self_.e23_ * other.e431_) - (self_.e31_ * other.e423_), -(self_.e41_ * other.e423_) - (self_.e42_ * other.e431_) - (self_.e43_ * other.e412_))
    ));
}
fn motor_antiWedge_antiScalar(self_: Motor, other: AntiScalar) -> Motor {
    let self_groups = motor_grouped(self_);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e1234_) * self_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(other.e1234_) * self_groups.group1_
    ));
}
fn motor_antiWedge_dualNum(self_: Motor, other: DualNum) -> Motor {
    let self_groups = motor_grouped(self_);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e1234_) * self_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(other.e1234_ * self_.e23_, other.e1234_ * self_.e31_, other.e1234_ * self_.e12_, (other.scalar * self_.e1234_) + (other.e1234_ * self_.scalar))
    ));
}
fn motor_antiWedge_flector(self_: Motor, other: Flector) -> Flector {
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other.e412_ * self_.e31_) + (other.e321_ * self_.e41_) - (other.e431_ * self_.e12_), (other.e423_ * self_.e12_) + (other.e321_ * self_.e42_) - (other.e412_ * self_.e23_), (other.e431_ * self_.e23_) + (other.e321_ * self_.e43_) - (other.e423_ * self_.e31_), -(other.e423_ * self_.e41_) - (other.e431_ * self_.e42_) - (other.e412_ * self_.e43_)) + (vec4<f32>(self_.e1234_) * other_groups.group0_), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e1234_) * other_groups.group1_
    ));
}
fn motor_antiWedge_horizon(self_: Motor, other: Horizon) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e321_ * self_.e41_, other.e321_ * self_.e42_, other.e321_ * self_.e43_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0, 1.0, 1.0, other.e321_ * self_.e1234_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)
    ));
}
fn motor_antiWedge_line(self_: Motor, other: Line) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e41_ * self_.e1234_, other.e42_ * self_.e1234_, other.e43_ * self_.e1234_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(other.e23_ * self_.e1234_, other.e31_ * self_.e1234_, other.e12_ * self_.e1234_, -(other.e41_ * self_.e23_) - (other.e42_ * self_.e31_) - (other.e43_ * self_.e12_) - (other.e23_ * self_.e41_) - (other.e31_ * self_.e42_) - (other.e12_ * self_.e43_))
    ));
}
fn motor_antiWedge_motor(self_: Motor, other: Motor) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other.e41_ * self_.e1234_) + (other.e1234_ * self_.e41_), (other.e42_ * self_.e1234_) + (other.e1234_ * self_.e42_), (other.e43_ * self_.e1234_) + (other.e1234_ * self_.e43_), other.e1234_ * self_.e1234_), 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, -(other.e41_ * self_.e23_) - (other.e42_ * self_.e31_) - (other.e43_ * self_.e12_) - (other.e23_ * self_.e41_) - (other.e31_ * self_.e42_) - (other.e12_ * self_.e43_)) + (vec4<f32>(other.e1234_) * self_groups.group1_) + (vec4<f32>(self_.e1234_) * other_groups.group1_)
    ));
}
fn motor_antiWedge_multiVector(self_: Motor, other: MultiVector) -> MultiVector {
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((self_.e1234_ * other.scalar) + (self_.scalar * other.e1234_) - (self_.e41_ * other.e23_) - (self_.e42_ * other.e31_) - (self_.e43_ * other.e12_) - (self_.e23_ * other.e41_) - (self_.e31_ * other.e42_) - (self_.e12_ * other.e43_), self_.e1234_ * other.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e41_ * other.e321_) + (self_.e31_ * other.e412_) - (self_.e12_ * other.e431_), (self_.e42_ * other.e321_) + (self_.e12_ * other.e423_) - (self_.e23_ * other.e412_), (self_.e43_ * other.e321_) + (self_.e23_ * other.e431_) - (self_.e31_ * other.e423_), -(self_.e41_ * other.e423_) - (self_.e42_ * other.e431_) - (self_.e43_ * other.e412_)) + (vec4<f32>(self_.e1234_) * other_groups.group1_), 
        /* e41, e42, e43 */ ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group2_) + ((vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e41_, self_.e42_, self_.e43_, 0.0)), 
        /* e23, e31, e12 */ ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group3_) + ((vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e23_, self_.e31_, self_.e12_, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e1234_) * other_groups.group4_
    ));
}
fn motor_antiWedge_origin(self_: Motor, other: Origin) -> Origin {
    return Origin(self_.e1234_ * other.e4_);
}
fn motor_antiWedge_plane(self_: Motor, other: Plane) -> Flector {
    let other_groups = plane_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e41_ * other.e321_) + (self_.e31_ * other.e412_) - (self_.e12_ * other.e431_), (self_.e42_ * other.e321_) + (self_.e12_ * other.e423_) - (self_.e23_ * other.e412_), (self_.e43_ * other.e321_) + (self_.e23_ * other.e431_) - (self_.e31_ * other.e423_), -(self_.e41_ * other.e423_) - (self_.e42_ * other.e431_) - (self_.e43_ * other.e412_)), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e1234_) * other_groups.group0_
    ));
}
fn motor_antiWedge_point(self_: Motor, other: Point) -> Point {
    let other_groups = point_grouped(other);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e1234_) * other_groups.group0_
    ));
}
fn motor_antiWedge_scalar(self_: Motor, other: Scalar) -> Scalar {
    return Scalar(self_.e1234_ * other.scalar);
}
fn multiVector_antiWedge_antiScalar(self_: MultiVector, other: AntiScalar) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ (vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(other.e1234_) * self_groups.group1_, 
        /* e41, e42, e43 */ (vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_, 
        /* e23, e31, e12 */ (vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e1234_) * self_groups.group4_
    ));
}
fn multiVector_antiWedge_dualNum(self_: MultiVector, other: DualNum) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((other.scalar * self_.e1234_) + (other.e1234_ * self_.scalar), other.e1234_ * self_.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(other.e1234_) * self_groups.group1_, 
        /* e41, e42, e43 */ (vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_, 
        /* e23, e31, e12 */ (vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e1234_) * self_groups.group4_
    ));
}
fn multiVector_antiWedge_flector(self_: MultiVector, other: Flector) -> MultiVector {
    let other_groups = flector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((other.e423_ * self_.e1_) + (other.e431_ * self_.e2_) + (other.e412_ * self_.e3_) + (other.e321_ * self_.e4_) - (other.e1_ * self_.e423_) - (other.e2_ * self_.e431_) - (other.e3_ * self_.e412_) - (other.e4_ * self_.e321_), 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((other.e412_ * self_.e31_) + (other.e321_ * self_.e41_) - (other.e431_ * self_.e12_), (other.e423_ * self_.e12_) + (other.e321_ * self_.e42_) - (other.e412_ * self_.e23_), (other.e431_ * self_.e23_) + (other.e321_ * self_.e43_) - (other.e423_ * self_.e31_), -(other.e423_ * self_.e41_) - (other.e431_ * self_.e42_) - (other.e412_ * self_.e43_)) + (vec4<f32>(self_.e1234_) * other_groups.group0_), 
        /* e41, e42, e43 */ vec4<f32>((other.e431_ * self_.e412_) - (other.e412_ * self_.e431_), (other.e412_ * self_.e423_) - (other.e423_ * self_.e412_), (other.e423_ * self_.e431_) - (other.e431_ * self_.e423_), 0.0), 
        /* e23, e31, e12 */ ((vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0)) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e1234_) * other_groups.group1_
    ));
}
fn multiVector_antiWedge_horizon(self_: MultiVector, other: Horizon) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.e321_ * self_.e4_, 1.0, 0.0, 0.0) * vec2<f32>(1.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(other.e321_ * self_.e41_, other.e321_ * self_.e42_, other.e321_ * self_.e43_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ (vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0, 1.0, 1.0, other.e321_ * self_.e1234_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)
    ));
}
fn multiVector_antiWedge_line(self_: MultiVector, other: Line) -> MultiVector {
    let other_groups = line_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(-(other.e41_ * self_.e23_) - (other.e42_ * self_.e31_) - (other.e43_ * self_.e12_) - (other.e23_ * self_.e41_) - (other.e31_ * self_.e42_) - (other.e12_ * self_.e43_), 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((other.e41_ * self_.e321_) + (other.e31_ * self_.e412_) - (other.e12_ * self_.e431_), (other.e42_ * self_.e321_) + (other.e12_ * self_.e423_) - (other.e23_ * self_.e412_), (other.e43_ * self_.e321_) + (other.e23_ * self_.e431_) - (other.e31_ * self_.e423_), -(other.e41_ * self_.e423_) - (other.e42_ * self_.e431_) - (other.e43_ * self_.e412_)), 
        /* e41, e42, e43 */ (vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group1_, 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn multiVector_antiWedge_motor(self_: MultiVector, other: Motor) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((other.e1234_ * self_.scalar) + (other.scalar * self_.e1234_) - (other.e41_ * self_.e23_) - (other.e42_ * self_.e31_) - (other.e43_ * self_.e12_) - (other.e23_ * self_.e41_) - (other.e31_ * self_.e42_) - (other.e12_ * self_.e43_), other.e1234_ * self_.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((other.e41_ * self_.e321_) + (other.e31_ * self_.e412_) - (other.e12_ * self_.e431_), (other.e42_ * self_.e321_) + (other.e12_ * self_.e423_) - (other.e23_ * self_.e412_), (other.e43_ * self_.e321_) + (other.e23_ * self_.e431_) - (other.e31_ * self_.e423_), -(other.e41_ * self_.e423_) - (other.e42_ * self_.e431_) - (other.e43_ * self_.e412_)) + (vec4<f32>(other.e1234_) * self_groups.group1_), 
        /* e41, e42, e43 */ ((vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_) + ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e41_, other.e42_, other.e43_, 0.0)), 
        /* e23, e31, e12 */ ((vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_) + ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e23_, other.e31_, other.e12_, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e1234_) * self_groups.group4_
    ));
}
fn multiVector_antiWedge_multiVector(self_: MultiVector, other: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((other.scalar * self_.e1234_) + (other.e1234_ * self_.scalar) + (other.e423_ * self_.e1_) + (other.e431_ * self_.e2_) + (other.e412_ * self_.e3_) + (other.e321_ * self_.e4_) - (other.e1_ * self_.e423_) - (other.e2_ * self_.e431_) - (other.e3_ * self_.e412_) - (other.e4_ * self_.e321_) - (other.e41_ * self_.e23_) - (other.e42_ * self_.e31_) - (other.e43_ * self_.e12_) - (other.e23_ * self_.e41_) - (other.e31_ * self_.e42_) - (other.e12_ * self_.e43_), other.e1234_ * self_.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((other.e41_ * self_.e321_) + (other.e31_ * self_.e412_) + (other.e412_ * self_.e31_) + (other.e321_ * self_.e41_) - (other.e12_ * self_.e431_) - (other.e431_ * self_.e12_), (other.e42_ * self_.e321_) + (other.e12_ * self_.e423_) + (other.e423_ * self_.e12_) + (other.e321_ * self_.e42_) - (other.e23_ * self_.e412_) - (other.e412_ * self_.e23_), (other.e43_ * self_.e321_) + (other.e23_ * self_.e431_) + (other.e431_ * self_.e23_) + (other.e321_ * self_.e43_) - (other.e31_ * self_.e423_) - (other.e423_ * self_.e31_), -(other.e41_ * self_.e423_) - (other.e42_ * self_.e431_) - (other.e43_ * self_.e412_) - (other.e423_ * self_.e41_) - (other.e431_ * self_.e42_) - (other.e412_ * self_.e43_)) + (vec4<f32>(other.e1234_) * self_groups.group1_) + (vec4<f32>(self_.e1234_) * other_groups.group1_), 
        /* e41, e42, e43 */ vec4<f32>((other.e431_ * self_.e412_) - (other.e412_ * self_.e431_), (other.e412_ * self_.e423_) - (other.e423_ * self_.e412_), (other.e423_ * self_.e431_) - (other.e431_ * self_.e423_), 0.0) + ((vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_) + ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group2_), 
        /* e23, e31, e12 */ ((vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_) + ((vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0)) + ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group3_) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0)), 
        /* e423, e431, e412, e321 */ (vec4<f32>(other.e1234_) * self_groups.group4_) + (vec4<f32>(self_.e1234_) * other_groups.group4_)
    ));
}
fn multiVector_antiWedge_origin(self_: MultiVector, other: Origin) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.e321_ * other.e4_, 1.0, 0.0, 0.0) * vec2<f32>(-1.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, self_.e1234_ * other.e4_) * vec4<f32>(0.0, 0.0, 0.0, 1.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn multiVector_antiWedge_plane(self_: MultiVector, other: Plane) -> MultiVector {
    let other_groups = plane_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((self_.e1_ * other.e423_) + (self_.e2_ * other.e431_) + (self_.e3_ * other.e412_) + (self_.e4_ * other.e321_), 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e41_ * other.e321_) + (self_.e31_ * other.e412_) - (self_.e12_ * other.e431_), (self_.e42_ * other.e321_) + (self_.e12_ * other.e423_) - (self_.e23_ * other.e412_), (self_.e43_ * other.e321_) + (self_.e23_ * other.e431_) - (self_.e31_ * other.e423_), -(self_.e41_ * other.e423_) - (self_.e42_ * other.e431_) - (self_.e43_ * other.e412_)), 
        /* e41, e42, e43 */ vec4<f32>((self_.e412_ * other.e431_) - (self_.e431_ * other.e412_), (self_.e423_ * other.e412_) - (self_.e412_ * other.e423_), (self_.e431_ * other.e423_) - (self_.e423_ * other.e431_), 0.0), 
        /* e23, e31, e12 */ ((vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0)) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e1234_) * other_groups.group0_
    ));
}
fn multiVector_antiWedge_point(self_: MultiVector, other: Point) -> MultiVector {
    let other_groups = point_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(-(self_.e423_ * other.e1_) - (self_.e431_ * other.e2_) - (self_.e412_ * other.e3_) - (self_.e321_ * other.e4_), 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e1234_) * other_groups.group0_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn multiVector_antiWedge_scalar(self_: MultiVector, other: Scalar) -> Scalar {
    return Scalar(self_.e1234_ * other.scalar);
}
fn origin_antiWedge_antiScalar(self_: Origin, other: AntiScalar) -> Origin {
    return Origin(other.e1234_ * self_.e4_);
}
fn origin_antiWedge_dualNum(self_: Origin, other: DualNum) -> Origin {
    return Origin(other.e1234_ * self_.e4_);
}
fn origin_antiWedge_flector(self_: Origin, other: Flector) -> Scalar {
    return Scalar(other.e321_ * self_.e4_);
}
fn origin_antiWedge_horizon(self_: Origin, other: Horizon) -> Scalar {
    return Scalar(other.e321_ * self_.e4_);
}
fn origin_antiWedge_motor(self_: Origin, other: Motor) -> Origin {
    return Origin(other.e1234_ * self_.e4_);
}
fn origin_antiWedge_multiVector(self_: Origin, other: MultiVector) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.e321_ * self_.e4_, 1.0, 0.0, 0.0) * vec2<f32>(1.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, other.e1234_ * self_.e4_) * vec4<f32>(0.0, 0.0, 0.0, 1.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn origin_antiWedge_plane(self_: Origin, other: Plane) -> Scalar {
    return Scalar(self_.e4_ * other.e321_);
}
fn plane_antiWedge_antiScalar(self_: Plane, other: AntiScalar) -> Plane {
    let self_groups = plane_grouped(self_);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(other.e1234_) * self_groups.group0_
    ));
}
fn plane_antiWedge_dualNum(self_: Plane, other: DualNum) -> Plane {
    let self_groups = plane_grouped(self_);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(other.e1234_) * self_groups.group0_
    ));
}
fn plane_antiWedge_flector(self_: Plane, other: Flector) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other.e431_ * self_.e412_) - (other.e412_ * self_.e431_), (other.e412_ * self_.e423_) - (other.e423_ * self_.e412_), (other.e423_ * self_.e431_) - (other.e431_ * self_.e423_), 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(other.e321_ * self_.e423_, other.e321_ * self_.e431_, other.e321_ * self_.e412_, -(other.e1_ * self_.e423_) - (other.e2_ * self_.e431_) - (other.e3_ * self_.e412_)) - (vec4<f32>(self_.e321_) * vec4<f32>(other.e423_, other.e431_, other.e412_, other.e4_))
    ));
}
fn plane_antiWedge_horizon(self_: Plane, other: Horizon) -> Line {
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ (vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0)
    ));
}
fn plane_antiWedge_line(self_: Plane, other: Line) -> Point {
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other.e41_ * self_.e321_) + (other.e31_ * self_.e412_) - (other.e12_ * self_.e431_), (other.e42_ * self_.e321_) + (other.e12_ * self_.e423_) - (other.e23_ * self_.e412_), (other.e43_ * self_.e321_) + (other.e23_ * self_.e431_) - (other.e31_ * self_.e423_), -(other.e41_ * self_.e423_) - (other.e42_ * self_.e431_) - (other.e43_ * self_.e412_))
    ));
}
fn plane_antiWedge_motor(self_: Plane, other: Motor) -> Flector {
    let self_groups = plane_grouped(self_);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other.e41_ * self_.e321_) + (other.e31_ * self_.e412_) - (other.e12_ * self_.e431_), (other.e42_ * self_.e321_) + (other.e12_ * self_.e423_) - (other.e23_ * self_.e412_), (other.e43_ * self_.e321_) + (other.e23_ * self_.e431_) - (other.e31_ * self_.e423_), -(other.e41_ * self_.e423_) - (other.e42_ * self_.e431_) - (other.e43_ * self_.e412_)), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e1234_) * self_groups.group0_
    ));
}
fn plane_antiWedge_multiVector(self_: Plane, other: MultiVector) -> MultiVector {
    let self_groups = plane_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(-(other.e1_ * self_.e423_) - (other.e2_ * self_.e431_) - (other.e3_ * self_.e412_) - (other.e4_ * self_.e321_), 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((other.e41_ * self_.e321_) + (other.e31_ * self_.e412_) - (other.e12_ * self_.e431_), (other.e42_ * self_.e321_) + (other.e12_ * self_.e423_) - (other.e23_ * self_.e412_), (other.e43_ * self_.e321_) + (other.e23_ * self_.e431_) - (other.e31_ * self_.e423_), -(other.e41_ * self_.e423_) - (other.e42_ * self_.e431_) - (other.e43_ * self_.e412_)), 
        /* e41, e42, e43 */ vec4<f32>((other.e431_ * self_.e412_) - (other.e412_ * self_.e431_), (other.e412_ * self_.e423_) - (other.e423_ * self_.e412_), (other.e423_ * self_.e431_) - (other.e431_ * self_.e423_), 0.0), 
        /* e23, e31, e12 */ ((vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0)) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e1234_) * self_groups.group0_
    ));
}
fn plane_antiWedge_origin(self_: Plane, other: Origin) -> Scalar {
    return Scalar(other.e4_ * self_.e321_ * -1.0);
}
fn plane_antiWedge_plane(self_: Plane, other: Plane) -> Line {
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>((other.e431_ * self_.e412_) - (other.e412_ * self_.e431_), (other.e412_ * self_.e423_) - (other.e423_ * self_.e412_), (other.e423_ * self_.e431_) - (other.e431_ * self_.e423_), 0.0), 
        /* e23, e31, e12 */ ((vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0)) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0))
    ));
}
fn plane_antiWedge_point(self_: Plane, other: Point) -> Scalar {
    return Scalar(-(self_.e423_ * other.e1_) - (self_.e431_ * other.e2_) - (self_.e412_ * other.e3_) - (self_.e321_ * other.e4_));
}
fn point_antiWedge_antiScalar(self_: Point, other: AntiScalar) -> Point {
    let self_groups = point_grouped(self_);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e1234_) * self_groups.group0_
    ));
}
fn point_antiWedge_dualNum(self_: Point, other: DualNum) -> Point {
    let self_groups = point_grouped(self_);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e1234_) * self_groups.group0_
    ));
}
fn point_antiWedge_flector(self_: Point, other: Flector) -> Scalar {
    return Scalar((other.e423_ * self_.e1_) + (other.e431_ * self_.e2_) + (other.e412_ * self_.e3_) + (other.e321_ * self_.e4_));
}
fn point_antiWedge_horizon(self_: Point, other: Horizon) -> Scalar {
    return Scalar(other.e321_ * self_.e4_);
}
fn point_antiWedge_motor(self_: Point, other: Motor) -> Point {
    let self_groups = point_grouped(self_);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e1234_) * self_groups.group0_
    ));
}
fn point_antiWedge_multiVector(self_: Point, other: MultiVector) -> MultiVector {
    let self_groups = point_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((other.e423_ * self_.e1_) + (other.e431_ * self_.e2_) + (other.e412_ * self_.e3_) + (other.e321_ * self_.e4_), 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(other.e1234_) * self_groups.group0_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn point_antiWedge_plane(self_: Point, other: Plane) -> Scalar {
    return Scalar((other.e423_ * self_.e1_) + (other.e431_ * self_.e2_) + (other.e412_ * self_.e3_) + (other.e321_ * self_.e4_));
}
fn scalar_antiWedge_antiScalar(self_: Scalar, other: AntiScalar) -> Scalar {
    return Scalar(other.e1234_ * self_.scalar);
}
fn scalar_antiWedge_dualNum(self_: Scalar, other: DualNum) -> Scalar {
    return Scalar(other.e1234_ * self_.scalar);
}
fn scalar_antiWedge_motor(self_: Scalar, other: Motor) -> Scalar {
    return Scalar(other.e1234_ * self_.scalar);
}
fn scalar_antiWedge_multiVector(self_: Scalar, other: MultiVector) -> Scalar {
    return Scalar(other.e1234_ * self_.scalar);
}
fn antiScalar_autoMorphism(self_: AntiScalar) -> AntiScalar {
    return self_;
}
fn dualNum_autoMorphism(self_: DualNum) -> DualNum {
    return self_;
}
fn flector_autoMorphism(self_: Flector) -> Flector {
    let self_groups = flector_grouped(self_);    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ self_groups.group0_ * vec4<f32>(-1.0), 
        /* e423, e431, e412, e321 */ self_groups.group1_ * vec4<f32>(-1.0)
    ));
}
fn horizon_autoMorphism(self_: Horizon) -> Horizon {
    return Horizon(self_.e321_ * -1.0);
}
fn line_autoMorphism(self_: Line) -> Line {
    return self_;
}
fn motor_autoMorphism(self_: Motor) -> Motor {
    return self_;
}
fn multiVector_autoMorphism(self_: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_groups.group0_, 
        /* e1, e2, e3, e4 */ self_groups.group1_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ self_groups.group2_, 
        /* e23, e31, e12 */ self_groups.group3_, 
        /* e423, e431, e412, e321 */ self_groups.group4_ * vec4<f32>(-1.0)
    ));
}
fn origin_autoMorphism(self_: Origin) -> Origin {
    return Origin(self_.e4_ * -1.0);
}
fn plane_autoMorphism(self_: Plane) -> Plane {
    let self_groups = plane_grouped(self_);    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ self_groups.group0_ * vec4<f32>(-1.0)
    ));
}
fn point_autoMorphism(self_: Point) -> Point {
    let self_groups = point_grouped(self_);    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ self_groups.group0_ * vec4<f32>(-1.0)
    ));
}
fn scalar_autoMorphism(self_: Scalar) -> Scalar {
    return self_;
}
fn antiScalar_conjugation(self_: AntiScalar) -> AntiScalar {
    return self_;
}
fn dualNum_conjugation(self_: DualNum) -> DualNum {
    return self_;
}
fn flector_conjugation(self_: Flector) -> Flector {
    let self_groups = flector_grouped(self_);    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ self_groups.group0_ * vec4<f32>(-1.0), 
        /* e423, e431, e412, e321 */ self_groups.group1_
    ));
}
fn horizon_conjugation(self_: Horizon) -> Horizon {
    return self_;
}
fn line_conjugation(self_: Line) -> Line {
    let self_groups = line_grouped(self_);    return line_degroup(LineGroups(
        /* e41, e42, e43 */ self_groups.group0_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ self_groups.group1_ * vec3<f32>(-1.0)
    ));
}
fn motor_conjugation(self_: Motor) -> Motor {
    let self_groups = motor_grouped(self_);    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ self_groups.group0_ * vec4<f32>(-1.0, -1.0, -1.0, 1.0), 
        /* e23, e31, e12, scalar */ self_groups.group1_ * vec4<f32>(-1.0, -1.0, -1.0, 1.0)
    ));
}
fn multiVector_conjugation(self_: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_groups.group0_, 
        /* e1, e2, e3, e4 */ self_groups.group1_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ self_groups.group2_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ self_groups.group3_ * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ self_groups.group4_
    ));
}
fn origin_conjugation(self_: Origin) -> Origin {
    return Origin(self_.e4_ * -1.0);
}
fn plane_conjugation(self_: Plane) -> Plane {
    return self_;
}
fn point_conjugation(self_: Point) -> Point {
    let self_groups = point_grouped(self_);    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ self_groups.group0_ * vec4<f32>(-1.0)
    ));
}
fn scalar_conjugation(self_: Scalar) -> Scalar {
    return self_;
}
fn antiScalar_constraintValid(self_: AntiScalar) -> AntiScalar {
    return self_;
}
fn origin_constraintValid(self_: Origin) -> Origin {
    return self_;
}
fn dualNum_constraintViolation(self_: DualNum) -> AntiScalar {
    let geometric_product: DualNum = dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(pow(self_.scalar, 2), self_.scalar * self_.e1234_, 0.0, 0.0) * vec2<f32>(1.0, 2.0)
    ));
    return AntiScalar(geometric_product.e1234_);
}
fn flector_constraintViolation(self_: Flector) -> DualNum {
    let self_groups = flector_grouped(self_);    let reverse: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ self_groups.group0_, 
        /* e423, e431, e412, e321 */ self_groups.group1_ * vec4<f32>(-1.0)
    ));
    let geometric_product: DualNum = dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, (reverse.e321_ * self_.e4_) - (reverse.e1_ * self_.e423_) - (reverse.e2_ * self_.e431_) - (reverse.e3_ * self_.e412_), 0.0, 0.0) + ((vec4<f32>(self_.e1_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(reverse.e1_, reverse.e423_, 0.0, 0.0)) + ((vec4<f32>(self_.e2_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(reverse.e2_, reverse.e431_, 0.0, 0.0)) + ((vec4<f32>(self_.e3_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(reverse.e3_, reverse.e412_, 0.0, 0.0)) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(reverse.e321_, reverse.e4_, 0.0, 0.0))
    ));
    let dot_product: Scalar = Scalar(pow(self_.e1_, 2) + pow(self_.e2_, 2) + pow(self_.e3_, 2) - pow(self_.e321_, 2));
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(geometric_product.scalar - dot_product.scalar, geometric_product.e1234_, 0.0, 0.0)
    ));
}
fn horizon_constraintViolation(self_: Horizon) -> Scalar {
    let reverse: Horizon = Horizon(self_.e321_ * -1.0);
    let geometric_product: Scalar = Scalar(reverse.e321_ * self_.e321_ * -1.0);
    let dot_product: Scalar = Scalar(pow(self_.e321_, 2) * -1.0);
    return Scalar(geometric_product.scalar - dot_product.scalar);
}
fn line_constraintViolation(self_: Line) -> DualNum {
    let self_groups = line_grouped(self_);    let reverse: Line = line_degroup(LineGroups(
        /* e41, e42, e43 */ self_groups.group0_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ self_groups.group1_ * vec3<f32>(-1.0)
    ));
    let geometric_product: DualNum = dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, -(reverse.e23_ * self_.e41_) - (reverse.e31_ * self_.e42_) - (reverse.e12_ * self_.e43_), 0.0, 0.0) - ((vec4<f32>(self_.e23_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(reverse.e23_, reverse.e41_, 0.0, 0.0)) - ((vec4<f32>(self_.e31_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(reverse.e31_, reverse.e42_, 0.0, 0.0)) - ((vec4<f32>(self_.e12_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(reverse.e12_, reverse.e43_, 0.0, 0.0))
    ));
    let dot_product: Scalar = Scalar(-pow(self_.e23_, 2) - pow(self_.e31_, 2) - pow(self_.e12_, 2));
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(geometric_product.scalar - dot_product.scalar, geometric_product.e1234_, 0.0, 0.0)
    ));
}
fn motor_constraintViolation(self_: Motor) -> DualNum {
    let self_groups = motor_grouped(self_);    let reverse: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ self_groups.group0_ * vec4<f32>(-1.0, -1.0, -1.0, 1.0), 
        /* e23, e31, e12, scalar */ self_groups.group1_ * vec4<f32>(-1.0, -1.0, -1.0, 1.0)
    ));
    let geometric_product: DualNum = dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, (reverse.scalar * self_.e1234_) - (reverse.e23_ * self_.e41_) - (reverse.e31_ * self_.e42_) - (reverse.e12_ * self_.e43_), 0.0, 0.0) + ((vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(reverse.scalar, reverse.e1234_, 0.0, 0.0)) - ((vec4<f32>(self_.e23_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(reverse.e23_, reverse.e41_, 0.0, 0.0)) - ((vec4<f32>(self_.e31_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(reverse.e31_, reverse.e42_, 0.0, 0.0)) - ((vec4<f32>(self_.e12_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(reverse.e12_, reverse.e43_, 0.0, 0.0))
    ));
    let dot_product: Scalar = Scalar(pow(self_.scalar, 2) - pow(self_.e23_, 2) - pow(self_.e31_, 2) - pow(self_.e12_, 2));
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(geometric_product.scalar - dot_product.scalar, geometric_product.e1234_, 0.0, 0.0)
    ));
}
fn multiVector_constraintViolation(self_: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);    let reverse_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ self_groups.group0_, 
        /* e1, e2, e3, e4 */ self_groups.group1_, 
        /* e41, e42, e43 */ self_groups.group2_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ self_groups.group3_ * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ self_groups.group4_ * vec4<f32>(-1.0)
    );
    let reverse: MultiVector = multiVector_degroup(reverse_groups);
    let geometric_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, (reverse.e1234_ * self_.scalar) + (reverse.e321_ * self_.e4_) - (reverse.e1_ * self_.e423_) - (reverse.e2_ * self_.e431_) - (reverse.e3_ * self_.e412_) - (reverse.e23_ * self_.e41_) - (reverse.e31_ * self_.e42_) - (reverse.e12_ * self_.e43_), 0.0, 0.0) + ((vec4<f32>(reverse.scalar) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_) + ((vec4<f32>(self_.e1_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(reverse.e1_, reverse.e423_, 0.0, 0.0)) + ((vec4<f32>(self_.e2_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(reverse.e2_, reverse.e431_, 0.0, 0.0)) + ((vec4<f32>(self_.e3_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(reverse.e3_, reverse.e412_, 0.0, 0.0)) - ((vec4<f32>(self_.e23_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(reverse.e23_, reverse.e41_, 0.0, 0.0)) - ((vec4<f32>(self_.e31_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(reverse.e31_, reverse.e42_, 0.0, 0.0)) - ((vec4<f32>(self_.e12_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(reverse.e12_, reverse.e43_, 0.0, 0.0)) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(reverse.e321_, reverse.e4_, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((reverse.e2_ * self_.e12_) + (reverse.e31_ * self_.e3_) + (reverse.e321_ * self_.e23_) - (reverse.e3_ * self_.e31_) - (reverse.e12_ * self_.e2_), (reverse.e3_ * self_.e23_) + (reverse.e12_ * self_.e1_) + (reverse.e321_ * self_.e31_) - (reverse.e1_ * self_.e12_) - (reverse.e23_ * self_.e3_), (reverse.e1_ * self_.e31_) + (reverse.e23_ * self_.e2_) + (reverse.e321_ * self_.e12_) - (reverse.e2_ * self_.e23_) - (reverse.e31_ * self_.e1_), (reverse.e1_ * self_.e41_) + (reverse.e2_ * self_.e42_) + (reverse.e3_ * self_.e43_) - (reverse.e41_ * self_.e1_) - (reverse.e42_ * self_.e2_) - (reverse.e43_ * self_.e3_) - (reverse.e23_ * self_.e423_) - (reverse.e31_ * self_.e431_) - (reverse.e12_ * self_.e412_) - (reverse.e423_ * self_.e23_) - (reverse.e431_ * self_.e31_) - (reverse.e412_ * self_.e12_) - (reverse.e321_ * self_.e1234_)) + (vec4<f32>(reverse.scalar) * self_groups.group1_) + (vec4<f32>(self_.scalar) * reverse_groups.group1_) + (vec4<f32>(self_.e321_) * vec4<f32>(reverse.e23_, reverse.e31_, reverse.e12_, reverse.e1234_)), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    );
    let geometric_product: MultiVector = multiVector_degroup(geometric_product_groups);
    let dot_product: Scalar = Scalar(pow(self_.scalar, 2) + pow(self_.e1_, 2) + pow(self_.e2_, 2) + pow(self_.e3_, 2) - pow(self_.e23_, 2) - pow(self_.e31_, 2) - pow(self_.e12_, 2) - pow(self_.e321_, 2));
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(geometric_product.scalar - dot_product.scalar, geometric_product.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ geometric_product_groups.group1_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn plane_constraintViolation(self_: Plane) -> Scalar {
    let self_groups = plane_grouped(self_);    let reverse: Plane = plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ self_groups.group0_ * vec4<f32>(-1.0)
    ));
    let geometric_product: Scalar = Scalar(reverse.e321_ * self_.e321_ * -1.0);
    let dot_product: Scalar = Scalar(pow(self_.e321_, 2) * -1.0);
    return Scalar(geometric_product.scalar - dot_product.scalar);
}
fn point_constraintViolation(self_: Point) -> Scalar {
    return Scalar(0.0);
}
fn scalar_constraintViolation(self_: Scalar) -> Scalar {
    return Scalar(0.0);
}
fn dualNum_dotProduct_dualNum(self_: DualNum, other: DualNum) -> Scalar {
    return Scalar(other.scalar * self_.scalar);
}
fn dualNum_dotProduct_motor(self_: DualNum, other: Motor) -> Scalar {
    return Scalar(self_.scalar * other.scalar);
}
fn dualNum_dotProduct_multiVector(self_: DualNum, other: MultiVector) -> Scalar {
    return Scalar(self_.scalar * other.scalar);
}
fn dualNum_dotProduct_scalar(self_: DualNum, other: Scalar) -> Scalar {
    return Scalar(self_.scalar * other.scalar);
}
fn flector_dotProduct_flector(self_: Flector, other: Flector) -> Scalar {
    return Scalar((other.e1_ * self_.e1_) + (other.e2_ * self_.e2_) + (other.e3_ * self_.e3_) - (other.e321_ * self_.e321_));
}
fn flector_dotProduct_horizon(self_: Flector, other: Horizon) -> Scalar {
    return Scalar(self_.e321_ * other.e321_ * -1.0);
}
fn flector_dotProduct_multiVector(self_: Flector, other: MultiVector) -> Scalar {
    return Scalar((self_.e1_ * other.e1_) + (self_.e2_ * other.e2_) + (self_.e3_ * other.e3_) - (self_.e321_ * other.e321_));
}
fn flector_dotProduct_plane(self_: Flector, other: Plane) -> Scalar {
    return Scalar(self_.e321_ * other.e321_ * -1.0);
}
fn flector_dotProduct_point(self_: Flector, other: Point) -> Scalar {
    return Scalar((self_.e1_ * other.e1_) + (self_.e2_ * other.e2_) + (self_.e3_ * other.e3_));
}
fn horizon_dotProduct_flector(self_: Horizon, other: Flector) -> Scalar {
    return Scalar(other.e321_ * self_.e321_ * -1.0);
}
fn horizon_dotProduct_horizon(self_: Horizon, other: Horizon) -> Scalar {
    return Scalar(other.e321_ * self_.e321_ * -1.0);
}
fn horizon_dotProduct_multiVector(self_: Horizon, other: MultiVector) -> Scalar {
    return Scalar(self_.e321_ * other.e321_ * -1.0);
}
fn horizon_dotProduct_plane(self_: Horizon, other: Plane) -> Scalar {
    return Scalar(self_.e321_ * other.e321_ * -1.0);
}
fn line_dotProduct_line(self_: Line, other: Line) -> Scalar {
    return Scalar(-(other.e23_ * self_.e23_) - (other.e31_ * self_.e31_) - (other.e12_ * self_.e12_));
}
fn line_dotProduct_motor(self_: Line, other: Motor) -> Scalar {
    return Scalar(-(self_.e23_ * other.e23_) - (self_.e31_ * other.e31_) - (self_.e12_ * other.e12_));
}
fn line_dotProduct_multiVector(self_: Line, other: MultiVector) -> Scalar {
    return Scalar(-(self_.e23_ * other.e23_) - (self_.e31_ * other.e31_) - (self_.e12_ * other.e12_));
}
fn motor_dotProduct_dualNum(self_: Motor, other: DualNum) -> Scalar {
    return Scalar(other.scalar * self_.scalar);
}
fn motor_dotProduct_line(self_: Motor, other: Line) -> Scalar {
    return Scalar(-(other.e23_ * self_.e23_) - (other.e31_ * self_.e31_) - (other.e12_ * self_.e12_));
}
fn motor_dotProduct_motor(self_: Motor, other: Motor) -> Scalar {
    return Scalar((other.scalar * self_.scalar) - (other.e23_ * self_.e23_) - (other.e31_ * self_.e31_) - (other.e12_ * self_.e12_));
}
fn motor_dotProduct_multiVector(self_: Motor, other: MultiVector) -> Scalar {
    return Scalar((self_.scalar * other.scalar) - (self_.e23_ * other.e23_) - (self_.e31_ * other.e31_) - (self_.e12_ * other.e12_));
}
fn motor_dotProduct_scalar(self_: Motor, other: Scalar) -> Scalar {
    return Scalar(self_.scalar * other.scalar);
}
fn multiVector_dotProduct_dualNum(self_: MultiVector, other: DualNum) -> Scalar {
    return Scalar(other.scalar * self_.scalar);
}
fn multiVector_dotProduct_flector(self_: MultiVector, other: Flector) -> Scalar {
    return Scalar((other.e1_ * self_.e1_) + (other.e2_ * self_.e2_) + (other.e3_ * self_.e3_) - (other.e321_ * self_.e321_));
}
fn multiVector_dotProduct_horizon(self_: MultiVector, other: Horizon) -> Scalar {
    return Scalar(other.e321_ * self_.e321_ * -1.0);
}
fn multiVector_dotProduct_line(self_: MultiVector, other: Line) -> Scalar {
    return Scalar(-(other.e23_ * self_.e23_) - (other.e31_ * self_.e31_) - (other.e12_ * self_.e12_));
}
fn multiVector_dotProduct_motor(self_: MultiVector, other: Motor) -> Scalar {
    return Scalar((other.scalar * self_.scalar) - (other.e23_ * self_.e23_) - (other.e31_ * self_.e31_) - (other.e12_ * self_.e12_));
}
fn multiVector_dotProduct_multiVector(self_: MultiVector, other: MultiVector) -> Scalar {
    return Scalar((other.scalar * self_.scalar) + (other.e1_ * self_.e1_) + (other.e2_ * self_.e2_) + (other.e3_ * self_.e3_) - (other.e23_ * self_.e23_) - (other.e31_ * self_.e31_) - (other.e12_ * self_.e12_) - (other.e321_ * self_.e321_));
}
fn multiVector_dotProduct_plane(self_: MultiVector, other: Plane) -> Scalar {
    return Scalar(self_.e321_ * other.e321_ * -1.0);
}
fn multiVector_dotProduct_point(self_: MultiVector, other: Point) -> Scalar {
    return Scalar((self_.e1_ * other.e1_) + (self_.e2_ * other.e2_) + (self_.e3_ * other.e3_));
}
fn multiVector_dotProduct_scalar(self_: MultiVector, other: Scalar) -> Scalar {
    return Scalar(self_.scalar * other.scalar);
}
fn plane_dotProduct_flector(self_: Plane, other: Flector) -> Scalar {
    return Scalar(other.e321_ * self_.e321_ * -1.0);
}
fn plane_dotProduct_horizon(self_: Plane, other: Horizon) -> Scalar {
    return Scalar(other.e321_ * self_.e321_ * -1.0);
}
fn plane_dotProduct_multiVector(self_: Plane, other: MultiVector) -> Scalar {
    return Scalar(other.e321_ * self_.e321_ * -1.0);
}
fn plane_dotProduct_plane(self_: Plane, other: Plane) -> Scalar {
    return Scalar(other.e321_ * self_.e321_ * -1.0);
}
fn point_dotProduct_flector(self_: Point, other: Flector) -> Scalar {
    return Scalar((other.e1_ * self_.e1_) + (other.e2_ * self_.e2_) + (other.e3_ * self_.e3_));
}
fn point_dotProduct_multiVector(self_: Point, other: MultiVector) -> Scalar {
    return Scalar((other.e1_ * self_.e1_) + (other.e2_ * self_.e2_) + (other.e3_ * self_.e3_));
}
fn point_dotProduct_point(self_: Point, other: Point) -> Scalar {
    return Scalar((other.e1_ * self_.e1_) + (other.e2_ * self_.e2_) + (other.e3_ * self_.e3_));
}
fn scalar_dotProduct_dualNum(self_: Scalar, other: DualNum) -> Scalar {
    return Scalar(other.scalar * self_.scalar);
}
fn scalar_dotProduct_motor(self_: Scalar, other: Motor) -> Scalar {
    return Scalar(other.scalar * self_.scalar);
}
fn scalar_dotProduct_multiVector(self_: Scalar, other: MultiVector) -> Scalar {
    return Scalar(other.scalar * self_.scalar);
}
fn scalar_dotProduct_scalar(self_: Scalar, other: Scalar) -> Scalar {
    return Scalar(other.scalar * self_.scalar);
}
fn antiScalar_doubleComplement(self_: AntiScalar) -> AntiScalar {
    return self_;
}
fn dualNum_doubleComplement(self_: DualNum) -> DualNum {
    return self_;
}
fn flector_doubleComplement(self_: Flector) -> Flector {
    let self_groups = flector_grouped(self_);    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ self_groups.group0_ * vec4<f32>(-1.0), 
        /* e423, e431, e412, e321 */ self_groups.group1_ * vec4<f32>(-1.0)
    ));
}
fn horizon_doubleComplement(self_: Horizon) -> Horizon {
    return Horizon(self_.e321_ * -1.0);
}
fn line_doubleComplement(self_: Line) -> Line {
    return self_;
}
fn motor_doubleComplement(self_: Motor) -> Motor {
    return self_;
}
fn multiVector_doubleComplement(self_: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_groups.group0_, 
        /* e1, e2, e3, e4 */ self_groups.group1_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ self_groups.group2_, 
        /* e23, e31, e12 */ self_groups.group3_, 
        /* e423, e431, e412, e321 */ self_groups.group4_ * vec4<f32>(-1.0)
    ));
}
fn origin_doubleComplement(self_: Origin) -> Origin {
    return Origin(self_.e4_ * -1.0);
}
fn plane_doubleComplement(self_: Plane) -> Plane {
    let self_groups = plane_grouped(self_);    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ self_groups.group0_ * vec4<f32>(-1.0)
    ));
}
fn point_doubleComplement(self_: Point) -> Point {
    let self_groups = point_grouped(self_);    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ self_groups.group0_ * vec4<f32>(-1.0)
    ));
}
fn scalar_doubleComplement(self_: Scalar) -> Scalar {
    return self_;
}
fn horizon_fix(self_: Horizon) -> Horizon {
    let reverse: Horizon = Horizon(self_.e321_ * -1.0);
    let geometric_product: Scalar = Scalar(reverse.e321_ * self_.e321_ * -1.0);
    let square_root: Scalar = Scalar(pow(geometric_product.scalar, 0.5));
    let dot_product: Scalar = Scalar(pow(square_root.scalar, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return Horizon(self_.e321_ * inverse.scalar);
}
fn plane_fix(self_: Plane) -> Plane {
    let self_groups = plane_grouped(self_);    let reverse: Plane = plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ self_groups.group0_ * vec4<f32>(-1.0)
    ));
    let geometric_product: Scalar = Scalar(reverse.e321_ * self_.e321_ * -1.0);
    let square_root: Scalar = Scalar(pow(geometric_product.scalar, 0.5));
    let dot_product: Scalar = Scalar(pow(square_root.scalar, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_groups.group0_
    ));
}
fn point_fix(self_: Point) -> Point {
    let self_groups = point_grouped(self_);    let geometric_product: Scalar = Scalar(pow(self_.e1_, 2) + pow(self_.e2_, 2) + pow(self_.e3_, 2));
    let square_root: Scalar = Scalar(pow(geometric_product.scalar, 0.5));
    let dot_product: Scalar = Scalar(pow(square_root.scalar, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_groups.group0_
    ));
}
fn scalar_fix(self_: Scalar) -> Scalar {
    let geometric_product: Scalar = Scalar(pow(self_.scalar, 2));
    let square_root: Scalar = Scalar(pow(geometric_product.scalar, 0.5));
    let dot_product: Scalar = Scalar(pow(square_root.scalar, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return Scalar(inverse.scalar * self_.scalar);
}
fn antiScalar_geometricAntiProduct_antiScalar(self_: AntiScalar, other: AntiScalar) -> AntiScalar {
    return AntiScalar(other.e1234_ * self_.e1234_);
}
fn antiScalar_geometricAntiProduct_dualNum(self_: AntiScalar, other: DualNum) -> DualNum {
    let other_groups = dualNum_grouped(other);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ (vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * other_groups.group0_
    ));
}
fn antiScalar_geometricAntiProduct_flector(self_: AntiScalar, other: Flector) -> Flector {
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e1234_) * other_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e1234_) * other_groups.group1_
    ));
}
fn antiScalar_geometricAntiProduct_horizon(self_: AntiScalar, other: Horizon) -> Horizon {
    return Horizon(self_.e1234_ * other.e321_);
}
fn antiScalar_geometricAntiProduct_line(self_: AntiScalar, other: Line) -> Line {
    let other_groups = line_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group1_
    ));
}
fn antiScalar_geometricAntiProduct_motor(self_: AntiScalar, other: Motor) -> Motor {
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.e1234_) * other_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.e1234_) * other_groups.group1_
    ));
}
fn antiScalar_geometricAntiProduct_multiVector(self_: AntiScalar, other: MultiVector) -> MultiVector {
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ (vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * other_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e1234_) * other_groups.group1_, 
        /* e41, e42, e43 */ (vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group2_, 
        /* e23, e31, e12 */ (vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e1234_) * other_groups.group4_
    ));
}
fn antiScalar_geometricAntiProduct_origin(self_: AntiScalar, other: Origin) -> Origin {
    return Origin(self_.e1234_ * other.e4_);
}
fn antiScalar_geometricAntiProduct_plane(self_: AntiScalar, other: Plane) -> Plane {
    let other_groups = plane_grouped(other);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e1234_) * other_groups.group0_
    ));
}
fn antiScalar_geometricAntiProduct_point(self_: AntiScalar, other: Point) -> Point {
    let other_groups = point_grouped(other);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e1234_) * other_groups.group0_
    ));
}
fn antiScalar_geometricAntiProduct_scalar(self_: AntiScalar, other: Scalar) -> Scalar {
    return Scalar(self_.e1234_ * other.scalar);
}
fn dualNum_geometricAntiProduct_antiScalar(self_: DualNum, other: AntiScalar) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ (vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_
    ));
}
fn dualNum_geometricAntiProduct_dualNum(self_: DualNum, other: DualNum) -> DualNum {
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>((other.scalar * self_.e1234_) + (other.e1234_ * self_.scalar), other.e1234_ * self_.e1234_, 0.0, 0.0)
    ));
}
fn dualNum_geometricAntiProduct_flector(self_: DualNum, other: Flector) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.scalar * other.e423_) + (self_.e1234_ * other.e1_), (self_.scalar * other.e431_) + (self_.e1234_ * other.e2_), (self_.scalar * other.e412_) + (self_.e1234_ * other.e3_), self_.e1234_ * other.e4_), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e1234_ * other.e423_, self_.e1234_ * other.e431_, self_.e1234_ * other.e412_, (self_.scalar * other.e4_) + (self_.e1234_ * other.e321_))
    ));
}
fn dualNum_geometricAntiProduct_horizon(self_: DualNum, other: Horizon) -> Horizon {
    return Horizon(self_.e1234_ * other.e321_);
}
fn dualNum_geometricAntiProduct_line(self_: DualNum, other: Line) -> Line {
    let other_groups = line_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group0_, 
        /* e23, e31, e12 */ ((vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group0_) + ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group1_)
    ));
}
fn dualNum_geometricAntiProduct_motor(self_: DualNum, other: Motor) -> Motor {
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.e1234_) * other_groups.group0_, 
        /* e23, e31, e12, scalar */ (vec4<f32>(self_.scalar) * other_groups.group0_) + (vec4<f32>(self_.e1234_) * other_groups.group1_)
    ));
}
fn dualNum_geometricAntiProduct_multiVector(self_: DualNum, other: MultiVector) -> MultiVector {
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((self_.scalar * other.e1234_) + (self_.e1234_ * other.scalar), self_.e1234_ * other.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.scalar * other.e423_) + (self_.e1234_ * other.e1_), (self_.scalar * other.e431_) + (self_.e1234_ * other.e2_), (self_.scalar * other.e412_) + (self_.e1234_ * other.e3_), self_.e1234_ * other.e4_), 
        /* e41, e42, e43 */ (vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group2_, 
        /* e23, e31, e12 */ ((vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group2_) + ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group3_), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e1234_ * other.e423_, self_.e1234_ * other.e431_, self_.e1234_ * other.e412_, (self_.scalar * other.e4_) + (self_.e1234_ * other.e321_))
    ));
}
fn dualNum_geometricAntiProduct_origin(self_: DualNum, other: Origin) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, self_.e1234_ * other.e4_) * vec4<f32>(0.0, 0.0, 0.0, 1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0, 1.0, 1.0, self_.scalar * other.e4_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)
    ));
}
fn dualNum_geometricAntiProduct_plane(self_: DualNum, other: Plane) -> Flector {
    let other_groups = plane_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.scalar * other.e423_, self_.scalar * other.e431_, self_.scalar * other.e412_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e1234_) * other_groups.group0_
    ));
}
fn dualNum_geometricAntiProduct_point(self_: DualNum, other: Point) -> Flector {
    let other_groups = point_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e1234_) * other_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0, 1.0, 1.0, self_.scalar * other.e4_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)
    ));
}
fn dualNum_geometricAntiProduct_scalar(self_: DualNum, other: Scalar) -> Scalar {
    return Scalar(self_.e1234_ * other.scalar);
}
fn flector_geometricAntiProduct_antiScalar(self_: Flector, other: AntiScalar) -> Flector {
    let self_groups = flector_grouped(self_);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e1234_) * self_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e1234_) * self_groups.group1_
    ));
}
fn flector_geometricAntiProduct_dualNum(self_: Flector, other: DualNum) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other.e1234_ * self_.e1_) - (other.scalar * self_.e423_), (other.e1234_ * self_.e2_) - (other.scalar * self_.e431_), (other.e1234_ * self_.e3_) - (other.scalar * self_.e412_), other.e1234_ * self_.e4_), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e1234_ * self_.e423_, other.e1234_ * self_.e431_, other.e1234_ * self_.e412_, (other.e1234_ * self_.e321_) - (other.scalar * self_.e4_))
    ));
}
fn flector_geometricAntiProduct_flector(self_: Flector, other: Flector) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other.e431_ * self_.e412_) - (other.e423_ * self_.e4_) - (other.e412_ * self_.e431_), (other.e412_ * self_.e423_) - (other.e423_ * self_.e412_) - (other.e431_ * self_.e4_), (other.e423_ * self_.e431_) - (other.e431_ * self_.e423_) - (other.e412_ * self_.e4_), (other.e423_ * self_.e423_) + (other.e431_ * self_.e431_) + (other.e412_ * self_.e412_)) - (vec4<f32>(other.e4_) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, self_.e4_)), 
        /* e23, e31, e12, scalar */ vec4<f32>((other.e3_ * self_.e431_) + (other.e431_ * self_.e3_) + (other.e321_ * self_.e423_) - (other.e2_ * self_.e412_) - (other.e423_ * self_.e321_) - (other.e412_ * self_.e2_), (other.e1_ * self_.e412_) + (other.e412_ * self_.e1_) + (other.e321_ * self_.e431_) - (other.e3_ * self_.e423_) - (other.e423_ * self_.e3_) - (other.e431_ * self_.e321_), (other.e2_ * self_.e423_) + (other.e423_ * self_.e2_) + (other.e321_ * self_.e412_) - (other.e1_ * self_.e431_) - (other.e431_ * self_.e1_) - (other.e412_ * self_.e321_), (other.e423_ * self_.e1_) + (other.e431_ * self_.e2_) + (other.e412_ * self_.e3_) - (other.e1_ * self_.e423_) - (other.e2_ * self_.e431_) - (other.e3_ * self_.e412_)) + (vec4<f32>(self_.e4_) * vec4<f32>(other.e1_, other.e2_, other.e3_, other.e321_)) - (vec4<f32>(other.e4_) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, self_.e321_))
    ));
}
fn flector_geometricAntiProduct_horizon(self_: Flector, other: Horizon) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(other.e321_) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, self_.e4_)
    ));
}
fn flector_geometricAntiProduct_line(self_: Flector, other: Line) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e2_ * other.e43_) + (self_.e412_ * other.e31_) + (self_.e321_ * other.e41_) - (self_.e3_ * other.e42_) - (self_.e4_ * other.e23_) - (self_.e431_ * other.e12_), (self_.e3_ * other.e41_) + (self_.e423_ * other.e12_) + (self_.e321_ * other.e42_) - (self_.e1_ * other.e43_) - (self_.e4_ * other.e31_) - (self_.e412_ * other.e23_), (self_.e1_ * other.e42_) + (self_.e431_ * other.e23_) + (self_.e321_ * other.e43_) - (self_.e2_ * other.e41_) - (self_.e4_ * other.e12_) - (self_.e423_ * other.e31_), -(self_.e423_ * other.e41_) - (self_.e431_ * other.e42_) - (self_.e412_ * other.e43_)), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e4_ * other.e41_) + (self_.e431_ * other.e43_) - (self_.e412_ * other.e42_), (self_.e4_ * other.e42_) + (self_.e412_ * other.e41_) - (self_.e423_ * other.e43_), (self_.e4_ * other.e43_) + (self_.e423_ * other.e42_) - (self_.e431_ * other.e41_), (self_.e423_ * other.e23_) + (self_.e431_ * other.e31_) + (self_.e412_ * other.e12_) - (self_.e1_ * other.e41_) - (self_.e2_ * other.e42_) - (self_.e3_ * other.e43_))
    ));
}
fn flector_geometricAntiProduct_motor(self_: Flector, other: Motor) -> Flector {
    let self_groups = flector_grouped(self_);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e2_ * other.e43_) + (self_.e412_ * other.e31_) + (self_.e321_ * other.e41_) - (self_.e3_ * other.e42_) - (self_.e4_ * other.e23_) - (self_.e423_ * other.scalar) - (self_.e431_ * other.e12_), (self_.e3_ * other.e41_) + (self_.e423_ * other.e12_) + (self_.e321_ * other.e42_) - (self_.e1_ * other.e43_) - (self_.e4_ * other.e31_) - (self_.e431_ * other.scalar) - (self_.e412_ * other.e23_), (self_.e1_ * other.e42_) + (self_.e431_ * other.e23_) + (self_.e321_ * other.e43_) - (self_.e2_ * other.e41_) - (self_.e4_ * other.e12_) - (self_.e423_ * other.e31_) - (self_.e412_ * other.scalar), -(self_.e423_ * other.e41_) - (self_.e431_ * other.e42_) - (self_.e412_ * other.e43_)) + (vec4<f32>(other.e1234_) * self_groups.group0_), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e4_ * other.e41_) + (self_.e431_ * other.e43_) - (self_.e412_ * other.e42_), (self_.e4_ * other.e42_) + (self_.e412_ * other.e41_) - (self_.e423_ * other.e43_), (self_.e4_ * other.e43_) + (self_.e423_ * other.e42_) - (self_.e431_ * other.e41_), (self_.e423_ * other.e23_) + (self_.e431_ * other.e31_) + (self_.e412_ * other.e12_) - (self_.e1_ * other.e41_) - (self_.e2_ * other.e42_) - (self_.e3_ * other.e43_) - (self_.e4_ * other.scalar)) + (vec4<f32>(other.e1234_) * self_groups.group1_)
    ));
}
fn flector_geometricAntiProduct_multiVector(self_: Flector, other: MultiVector) -> MultiVector {
    let self_groups = flector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((self_.e4_ * other.e321_) - (self_.e423_ * other.e1_) - (self_.e431_ * other.e2_) - (self_.e412_ * other.e3_), 0.0, 0.0, 0.0) + ((vec4<f32>(other.e423_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e1_, self_.e423_, 0.0, 0.0)) + ((vec4<f32>(other.e431_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e2_, self_.e431_, 0.0, 0.0)) + ((vec4<f32>(other.e412_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e3_, self_.e412_, 0.0, 0.0)) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e321_, self_.e4_, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e2_ * other.e43_) + (self_.e412_ * other.e31_) + (self_.e321_ * other.e41_) - (self_.e3_ * other.e42_) - (self_.e4_ * other.e23_) - (self_.e423_ * other.scalar) - (self_.e431_ * other.e12_), (self_.e3_ * other.e41_) + (self_.e423_ * other.e12_) + (self_.e321_ * other.e42_) - (self_.e1_ * other.e43_) - (self_.e4_ * other.e31_) - (self_.e431_ * other.scalar) - (self_.e412_ * other.e23_), (self_.e1_ * other.e42_) + (self_.e431_ * other.e23_) + (self_.e321_ * other.e43_) - (self_.e2_ * other.e41_) - (self_.e4_ * other.e12_) - (self_.e423_ * other.e31_) - (self_.e412_ * other.scalar), -(self_.e423_ * other.e41_) - (self_.e431_ * other.e42_) - (self_.e412_ * other.e43_)) + (vec4<f32>(other.e1234_) * self_groups.group0_), 
        /* e41, e42, e43 */ vec4<f32>((self_.e412_ * other.e431_) - (self_.e431_ * other.e412_), (self_.e423_ * other.e412_) - (self_.e412_ * other.e423_), (self_.e431_ * other.e423_) - (self_.e423_ * other.e431_), 0.0) - ((vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0)) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((self_.e3_ * other.e431_) + (self_.e431_ * other.e3_) - (self_.e2_ * other.e412_) - (self_.e412_ * other.e2_), (self_.e1_ * other.e412_) + (self_.e412_ * other.e1_) - (self_.e3_ * other.e423_) - (self_.e423_ * other.e3_), (self_.e2_ * other.e423_) + (self_.e423_ * other.e2_) - (self_.e1_ * other.e431_) - (self_.e431_ * other.e1_), 0.0) + ((vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0)) + ((vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0)) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0)) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e4_ * other.e41_) + (self_.e431_ * other.e43_) - (self_.e412_ * other.e42_), (self_.e4_ * other.e42_) + (self_.e412_ * other.e41_) - (self_.e423_ * other.e43_), (self_.e4_ * other.e43_) + (self_.e423_ * other.e42_) - (self_.e431_ * other.e41_), (self_.e423_ * other.e23_) + (self_.e431_ * other.e31_) + (self_.e412_ * other.e12_) - (self_.e1_ * other.e41_) - (self_.e2_ * other.e42_) - (self_.e3_ * other.e43_) - (self_.e4_ * other.scalar)) + (vec4<f32>(other.e1234_) * self_groups.group1_)
    ));
}
fn flector_geometricAntiProduct_origin(self_: Flector, other: Origin) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e4_) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, self_.e4_) * vec4<f32>(-1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(other.e4_) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, self_.e321_) * vec4<f32>(-1.0)
    ));
}
fn flector_geometricAntiProduct_plane(self_: Flector, other: Plane) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((self_.e412_ * other.e431_) - (self_.e4_ * other.e423_) - (self_.e431_ * other.e412_), (self_.e423_ * other.e412_) - (self_.e4_ * other.e431_) - (self_.e412_ * other.e423_), (self_.e431_ * other.e423_) - (self_.e4_ * other.e412_) - (self_.e423_ * other.e431_), (self_.e423_ * other.e423_) + (self_.e431_ * other.e431_) + (self_.e412_ * other.e412_)), 
        /* e23, e31, e12, scalar */ vec4<f32>((self_.e3_ * other.e431_) - (self_.e2_ * other.e412_) - (self_.e321_ * other.e423_), (self_.e1_ * other.e412_) - (self_.e3_ * other.e423_) - (self_.e321_ * other.e431_), (self_.e2_ * other.e423_) - (self_.e1_ * other.e431_) - (self_.e321_ * other.e412_), (self_.e1_ * other.e423_) + (self_.e2_ * other.e431_) + (self_.e3_ * other.e412_)) + (vec4<f32>(other.e321_) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, self_.e4_))
    ));
}
fn flector_geometricAntiProduct_point(self_: Flector, other: Point) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e4_) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, self_.e4_) * vec4<f32>(-1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>((self_.e4_ * other.e1_) + (self_.e431_ * other.e3_) - (self_.e412_ * other.e2_), (self_.e4_ * other.e2_) + (self_.e412_ * other.e1_) - (self_.e423_ * other.e3_), (self_.e4_ * other.e3_) + (self_.e423_ * other.e2_) - (self_.e431_ * other.e1_), -(self_.e423_ * other.e1_) - (self_.e431_ * other.e2_) - (self_.e412_ * other.e3_)) - (vec4<f32>(other.e4_) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, self_.e321_))
    ));
}
fn flector_geometricAntiProduct_scalar(self_: Flector, other: Scalar) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e423_ * other.scalar, self_.e431_ * other.scalar, self_.e412_ * other.scalar, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0, 1.0, 1.0, self_.e4_ * other.scalar) * vec4<f32>(0.0, 0.0, 0.0, -1.0)
    ));
}
fn horizon_geometricAntiProduct_antiScalar(self_: Horizon, other: AntiScalar) -> Horizon {
    return Horizon(other.e1234_ * self_.e321_);
}
fn horizon_geometricAntiProduct_dualNum(self_: Horizon, other: DualNum) -> Horizon {
    return Horizon(other.e1234_ * self_.e321_);
}
fn horizon_geometricAntiProduct_flector(self_: Horizon, other: Flector) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.e321_) * vec4<f32>(other.e423_, other.e431_, other.e412_, other.e4_) * vec4<f32>(-1.0)
    ));
}
fn horizon_geometricAntiProduct_line(self_: Horizon, other: Line) -> Point {
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e321_ * other.e41_, self_.e321_ * other.e42_, self_.e321_ * other.e43_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
}
fn horizon_geometricAntiProduct_motor(self_: Horizon, other: Motor) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e321_ * other.e41_, self_.e321_ * other.e42_, self_.e321_ * other.e43_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0, 1.0, 1.0, self_.e321_ * other.e1234_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)
    ));
}
fn horizon_geometricAntiProduct_multiVector(self_: Horizon, other: MultiVector) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.e321_ * other.e4_, 1.0, 0.0, 0.0) * vec2<f32>(-1.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e321_ * other.e41_, self_.e321_ * other.e42_, self_.e321_ * other.e43_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ (vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0) * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0, 1.0, 1.0, self_.e321_ * other.e1234_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)
    ));
}
fn horizon_geometricAntiProduct_origin(self_: Horizon, other: Origin) -> Scalar {
    return Scalar(self_.e321_ * other.e4_ * -1.0);
}
fn horizon_geometricAntiProduct_plane(self_: Horizon, other: Plane) -> Line {
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ (vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0) * vec3<f32>(-1.0)
    ));
}
fn horizon_geometricAntiProduct_point(self_: Horizon, other: Point) -> Scalar {
    return Scalar(self_.e321_ * other.e4_ * -1.0);
}
fn line_geometricAntiProduct_antiScalar(self_: Line, other: AntiScalar) -> Line {
    let self_groups = line_grouped(self_);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_
    ));
}
fn line_geometricAntiProduct_dualNum(self_: Line, other: DualNum) -> Line {
    let self_groups = line_grouped(self_);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_, 
        /* e23, e31, e12 */ ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_) + ((vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_)
    ));
}
fn line_geometricAntiProduct_flector(self_: Line, other: Flector) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other.e3_ * self_.e42_) + (other.e4_ * self_.e23_) + (other.e412_ * self_.e31_) + (other.e321_ * self_.e41_) - (other.e2_ * self_.e43_) - (other.e431_ * self_.e12_), (other.e1_ * self_.e43_) + (other.e4_ * self_.e31_) + (other.e423_ * self_.e12_) + (other.e321_ * self_.e42_) - (other.e3_ * self_.e41_) - (other.e412_ * self_.e23_), (other.e2_ * self_.e41_) + (other.e4_ * self_.e12_) + (other.e431_ * self_.e23_) + (other.e321_ * self_.e43_) - (other.e1_ * self_.e42_) - (other.e423_ * self_.e31_), -(other.e423_ * self_.e41_) - (other.e431_ * self_.e42_) - (other.e412_ * self_.e43_)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e4_ * self_.e41_) + (other.e412_ * self_.e42_) - (other.e431_ * self_.e43_), (other.e4_ * self_.e42_) + (other.e423_ * self_.e43_) - (other.e412_ * self_.e41_), (other.e4_ * self_.e43_) + (other.e431_ * self_.e41_) - (other.e423_ * self_.e42_), -(other.e1_ * self_.e41_) - (other.e2_ * self_.e42_) - (other.e3_ * self_.e43_) - (other.e423_ * self_.e23_) - (other.e431_ * self_.e31_) - (other.e412_ * self_.e12_))
    ));
}
fn line_geometricAntiProduct_horizon(self_: Line, other: Horizon) -> Point {
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e321_ * self_.e41_, other.e321_ * self_.e42_, other.e321_ * self_.e43_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
}
fn line_geometricAntiProduct_line(self_: Line, other: Line) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other.e43_ * self_.e42_) - (other.e42_ * self_.e43_), (other.e41_ * self_.e43_) - (other.e43_ * self_.e41_), (other.e42_ * self_.e41_) - (other.e41_ * self_.e42_), -(other.e41_ * self_.e41_) - (other.e42_ * self_.e42_) - (other.e43_ * self_.e43_)), 
        /* e23, e31, e12, scalar */ vec4<f32>((other.e43_ * self_.e31_) + (other.e12_ * self_.e42_) - (other.e42_ * self_.e12_) - (other.e31_ * self_.e43_), (other.e41_ * self_.e12_) + (other.e23_ * self_.e43_) - (other.e43_ * self_.e23_) - (other.e12_ * self_.e41_), (other.e42_ * self_.e23_) + (other.e31_ * self_.e41_) - (other.e41_ * self_.e31_) - (other.e23_ * self_.e42_), -(other.e41_ * self_.e23_) - (other.e42_ * self_.e31_) - (other.e43_ * self_.e12_) - (other.e23_ * self_.e41_) - (other.e31_ * self_.e42_) - (other.e12_ * self_.e43_))
    ));
}
fn line_geometricAntiProduct_motor(self_: Line, other: Motor) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((self_.e41_ * other.e1234_) + (self_.e42_ * other.e43_) - (self_.e43_ * other.e42_), (self_.e42_ * other.e1234_) + (self_.e43_ * other.e41_) - (self_.e41_ * other.e43_), (self_.e41_ * other.e42_) + (self_.e43_ * other.e1234_) - (self_.e42_ * other.e41_), -(self_.e41_ * other.e41_) - (self_.e42_ * other.e42_) - (self_.e43_ * other.e43_)), 
        /* e23, e31, e12, scalar */ vec4<f32>((self_.e41_ * other.scalar) + (self_.e42_ * other.e12_) + (self_.e23_ * other.e1234_) + (self_.e31_ * other.e43_) - (self_.e43_ * other.e31_) - (self_.e12_ * other.e42_), (self_.e42_ * other.scalar) + (self_.e43_ * other.e23_) + (self_.e31_ * other.e1234_) + (self_.e12_ * other.e41_) - (self_.e41_ * other.e12_) - (self_.e23_ * other.e43_), (self_.e41_ * other.e31_) + (self_.e43_ * other.scalar) + (self_.e23_ * other.e42_) + (self_.e12_ * other.e1234_) - (self_.e42_ * other.e23_) - (self_.e31_ * other.e41_), -(self_.e41_ * other.e23_) - (self_.e42_ * other.e31_) - (self_.e43_ * other.e12_) - (self_.e23_ * other.e41_) - (self_.e31_ * other.e42_) - (self_.e12_ * other.e43_))
    ));
}
fn line_geometricAntiProduct_multiVector(self_: Line, other: MultiVector) -> MultiVector {
    let self_groups = line_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(-(self_.e23_ * other.e41_) - (self_.e31_ * other.e42_) - (self_.e12_ * other.e43_), 0.0, 0.0, 0.0) - ((vec4<f32>(self_.e41_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e23_, other.e41_, 0.0, 0.0)) - ((vec4<f32>(self_.e42_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e31_, other.e42_, 0.0, 0.0)) - ((vec4<f32>(self_.e43_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e12_, other.e43_, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e41_ * other.e321_) + (self_.e42_ * other.e3_) + (self_.e23_ * other.e4_) + (self_.e31_ * other.e412_) - (self_.e43_ * other.e2_) - (self_.e12_ * other.e431_), (self_.e42_ * other.e321_) + (self_.e43_ * other.e1_) + (self_.e31_ * other.e4_) + (self_.e12_ * other.e423_) - (self_.e41_ * other.e3_) - (self_.e23_ * other.e412_), (self_.e41_ * other.e2_) + (self_.e43_ * other.e321_) + (self_.e23_ * other.e431_) + (self_.e12_ * other.e4_) - (self_.e42_ * other.e1_) - (self_.e31_ * other.e423_), -(self_.e41_ * other.e423_) - (self_.e42_ * other.e431_) - (self_.e43_ * other.e412_)), 
        /* e41, e42, e43 */ vec4<f32>((self_.e42_ * other.e43_) - (self_.e43_ * other.e42_), (self_.e43_ * other.e41_) - (self_.e41_ * other.e43_), (self_.e41_ * other.e42_) - (self_.e42_ * other.e41_), 0.0) + ((vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_), 
        /* e23, e31, e12 */ vec4<f32>((self_.e42_ * other.e12_) + (self_.e31_ * other.e43_) - (self_.e43_ * other.e31_) - (self_.e12_ * other.e42_), (self_.e43_ * other.e23_) + (self_.e12_ * other.e41_) - (self_.e41_ * other.e12_) - (self_.e23_ * other.e43_), (self_.e41_ * other.e31_) + (self_.e23_ * other.e42_) - (self_.e42_ * other.e23_) - (self_.e31_ * other.e41_), 0.0) + ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_) + ((vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e41_ * other.e4_) + (self_.e42_ * other.e412_) - (self_.e43_ * other.e431_), (self_.e42_ * other.e4_) + (self_.e43_ * other.e423_) - (self_.e41_ * other.e412_), (self_.e41_ * other.e431_) + (self_.e43_ * other.e4_) - (self_.e42_ * other.e423_), -(self_.e41_ * other.e1_) - (self_.e42_ * other.e2_) - (self_.e43_ * other.e3_) - (self_.e23_ * other.e423_) - (self_.e31_ * other.e431_) - (self_.e12_ * other.e412_))
    ));
}
fn line_geometricAntiProduct_origin(self_: Line, other: Origin) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e23_ * other.e4_, self_.e31_ * other.e4_, self_.e12_ * other.e4_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e41_ * other.e4_, self_.e42_ * other.e4_, self_.e43_ * other.e4_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
}
fn line_geometricAntiProduct_plane(self_: Line, other: Plane) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e41_ * other.e321_) + (self_.e31_ * other.e412_) - (self_.e12_ * other.e431_), (self_.e42_ * other.e321_) + (self_.e12_ * other.e423_) - (self_.e23_ * other.e412_), (self_.e43_ * other.e321_) + (self_.e23_ * other.e431_) - (self_.e31_ * other.e423_), -(self_.e41_ * other.e423_) - (self_.e42_ * other.e431_) - (self_.e43_ * other.e412_)), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e42_ * other.e412_) - (self_.e43_ * other.e431_), (self_.e43_ * other.e423_) - (self_.e41_ * other.e412_), (self_.e41_ * other.e431_) - (self_.e42_ * other.e423_), -(self_.e23_ * other.e423_) - (self_.e31_ * other.e431_) - (self_.e12_ * other.e412_))
    ));
}
fn line_geometricAntiProduct_point(self_: Line, other: Point) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e42_ * other.e3_) + (self_.e23_ * other.e4_) - (self_.e43_ * other.e2_), (self_.e43_ * other.e1_) + (self_.e31_ * other.e4_) - (self_.e41_ * other.e3_), (self_.e41_ * other.e2_) + (self_.e12_ * other.e4_) - (self_.e42_ * other.e1_), 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e41_ * other.e4_, self_.e42_ * other.e4_, self_.e43_ * other.e4_, -(self_.e41_ * other.e1_) - (self_.e42_ * other.e2_) - (self_.e43_ * other.e3_))
    ));
}
fn line_geometricAntiProduct_scalar(self_: Line, other: Scalar) -> Line {
    let self_groups = line_grouped(self_);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ (vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_
    ));
}
fn motor_geometricAntiProduct_antiScalar(self_: Motor, other: AntiScalar) -> Motor {
    let self_groups = motor_grouped(self_);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e1234_) * self_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(other.e1234_) * self_groups.group1_
    ));
}
fn motor_geometricAntiProduct_dualNum(self_: Motor, other: DualNum) -> Motor {
    let self_groups = motor_grouped(self_);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e1234_) * self_groups.group0_, 
        /* e23, e31, e12, scalar */ (vec4<f32>(other.scalar) * self_groups.group0_) + (vec4<f32>(other.e1234_) * self_groups.group1_)
    ));
}
fn motor_geometricAntiProduct_flector(self_: Motor, other: Flector) -> Flector {
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other.e3_ * self_.e42_) + (other.e4_ * self_.e23_) + (other.e423_ * self_.scalar) + (other.e412_ * self_.e31_) + (other.e321_ * self_.e41_) - (other.e2_ * self_.e43_) - (other.e431_ * self_.e12_), (other.e1_ * self_.e43_) + (other.e4_ * self_.e31_) + (other.e423_ * self_.e12_) + (other.e431_ * self_.scalar) + (other.e321_ * self_.e42_) - (other.e3_ * self_.e41_) - (other.e412_ * self_.e23_), (other.e2_ * self_.e41_) + (other.e4_ * self_.e12_) + (other.e431_ * self_.e23_) + (other.e412_ * self_.scalar) + (other.e321_ * self_.e43_) - (other.e1_ * self_.e42_) - (other.e423_ * self_.e31_), -(other.e423_ * self_.e41_) - (other.e431_ * self_.e42_) - (other.e412_ * self_.e43_)) + (vec4<f32>(self_.e1234_) * other_groups.group0_), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e412_ * self_.e42_) - (other.e431_ * self_.e43_), (other.e423_ * self_.e43_) - (other.e412_ * self_.e41_), (other.e431_ * self_.e41_) - (other.e423_ * self_.e42_), -(other.e1_ * self_.e41_) - (other.e2_ * self_.e42_) - (other.e3_ * self_.e43_) - (other.e423_ * self_.e23_) - (other.e431_ * self_.e31_) - (other.e412_ * self_.e12_)) + (vec4<f32>(other.e4_) * vec4<f32>(self_.e41_, self_.e42_, self_.e43_, self_.scalar)) + (vec4<f32>(self_.e1234_) * other_groups.group1_)
    ));
}
fn motor_geometricAntiProduct_horizon(self_: Motor, other: Horizon) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e321_ * self_.e41_, other.e321_ * self_.e42_, other.e321_ * self_.e43_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0, 1.0, 1.0, other.e321_ * self_.e1234_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)
    ));
}
fn motor_geometricAntiProduct_line(self_: Motor, other: Line) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other.e41_ * self_.e1234_) + (other.e43_ * self_.e42_) - (other.e42_ * self_.e43_), (other.e41_ * self_.e43_) + (other.e42_ * self_.e1234_) - (other.e43_ * self_.e41_), (other.e42_ * self_.e41_) + (other.e43_ * self_.e1234_) - (other.e41_ * self_.e42_), -(other.e41_ * self_.e41_) - (other.e42_ * self_.e42_) - (other.e43_ * self_.e43_)), 
        /* e23, e31, e12, scalar */ vec4<f32>((other.e41_ * self_.scalar) + (other.e43_ * self_.e31_) + (other.e23_ * self_.e1234_) + (other.e12_ * self_.e42_) - (other.e42_ * self_.e12_) - (other.e31_ * self_.e43_), (other.e41_ * self_.e12_) + (other.e42_ * self_.scalar) + (other.e23_ * self_.e43_) + (other.e31_ * self_.e1234_) - (other.e43_ * self_.e23_) - (other.e12_ * self_.e41_), (other.e42_ * self_.e23_) + (other.e43_ * self_.scalar) + (other.e31_ * self_.e41_) + (other.e12_ * self_.e1234_) - (other.e41_ * self_.e31_) - (other.e23_ * self_.e42_), -(other.e41_ * self_.e23_) - (other.e42_ * self_.e31_) - (other.e43_ * self_.e12_) - (other.e23_ * self_.e41_) - (other.e31_ * self_.e42_) - (other.e12_ * self_.e43_))
    ));
}
fn motor_geometricAntiProduct_motor(self_: Motor, other: Motor) -> Motor {
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other.e43_ * self_.e42_) + (other.e1234_ * self_.e41_) - (other.e42_ * self_.e43_), (other.e41_ * self_.e43_) + (other.e1234_ * self_.e42_) - (other.e43_ * self_.e41_), (other.e42_ * self_.e41_) + (other.e1234_ * self_.e43_) - (other.e41_ * self_.e42_), -(other.e41_ * self_.e41_) - (other.e42_ * self_.e42_) - (other.e43_ * self_.e43_)) + (vec4<f32>(self_.e1234_) * other_groups.group0_), 
        /* e23, e31, e12, scalar */ vec4<f32>((other.e43_ * self_.e31_) + (other.e1234_ * self_.e23_) + (other.e12_ * self_.e42_) + (other.scalar * self_.e41_) - (other.e42_ * self_.e12_) - (other.e31_ * self_.e43_), (other.e41_ * self_.e12_) + (other.e1234_ * self_.e31_) + (other.e23_ * self_.e43_) + (other.scalar * self_.e42_) - (other.e43_ * self_.e23_) - (other.e12_ * self_.e41_), (other.e42_ * self_.e23_) + (other.e1234_ * self_.e12_) + (other.e31_ * self_.e41_) + (other.scalar * self_.e43_) - (other.e41_ * self_.e31_) - (other.e23_ * self_.e42_), -(other.e41_ * self_.e23_) - (other.e42_ * self_.e31_) - (other.e43_ * self_.e12_) - (other.e23_ * self_.e41_) - (other.e31_ * self_.e42_) - (other.e12_ * self_.e43_)) + (vec4<f32>(self_.e1234_) * other_groups.group1_) + (vec4<f32>(self_.scalar) * other_groups.group0_)
    ));
}
fn motor_geometricAntiProduct_multiVector(self_: Motor, other: MultiVector) -> MultiVector {
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((self_.scalar * other.e1234_) - (self_.e23_ * other.e41_) - (self_.e31_ * other.e42_) - (self_.e12_ * other.e43_), 0.0, 0.0, 0.0) + ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * other_groups.group0_) - ((vec4<f32>(self_.e41_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e23_, other.e41_, 0.0, 0.0)) - ((vec4<f32>(self_.e42_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e31_, other.e42_, 0.0, 0.0)) - ((vec4<f32>(self_.e43_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e12_, other.e43_, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e41_ * other.e321_) + (self_.e42_ * other.e3_) + (self_.e23_ * other.e4_) + (self_.e31_ * other.e412_) + (self_.scalar * other.e423_) - (self_.e43_ * other.e2_) - (self_.e12_ * other.e431_), (self_.e42_ * other.e321_) + (self_.e43_ * other.e1_) + (self_.e31_ * other.e4_) + (self_.e12_ * other.e423_) + (self_.scalar * other.e431_) - (self_.e41_ * other.e3_) - (self_.e23_ * other.e412_), (self_.e41_ * other.e2_) + (self_.e43_ * other.e321_) + (self_.e23_ * other.e431_) + (self_.e12_ * other.e4_) + (self_.scalar * other.e412_) - (self_.e42_ * other.e1_) - (self_.e31_ * other.e423_), -(self_.e41_ * other.e423_) - (self_.e42_ * other.e431_) - (self_.e43_ * other.e412_)) + (vec4<f32>(self_.e1234_) * other_groups.group1_), 
        /* e41, e42, e43 */ vec4<f32>((self_.e42_ * other.e43_) - (self_.e43_ * other.e42_), (self_.e43_ * other.e41_) - (self_.e41_ * other.e43_), (self_.e41_ * other.e42_) - (self_.e42_ * other.e41_), 0.0) + ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group2_) + ((vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e41_, self_.e42_, self_.e43_, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((self_.e42_ * other.e12_) + (self_.e31_ * other.e43_) - (self_.e43_ * other.e31_) - (self_.e12_ * other.e42_), (self_.e43_ * other.e23_) + (self_.e12_ * other.e41_) - (self_.e41_ * other.e12_) - (self_.e23_ * other.e43_), (self_.e41_ * other.e31_) + (self_.e23_ * other.e42_) - (self_.e42_ * other.e23_) - (self_.e31_ * other.e41_), 0.0) + ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group3_) + ((vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group2_) + ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e41_, self_.e42_, self_.e43_, 0.0)) + ((vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e23_, self_.e31_, self_.e12_, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e42_ * other.e412_) - (self_.e43_ * other.e431_), (self_.e43_ * other.e423_) - (self_.e41_ * other.e412_), (self_.e41_ * other.e431_) - (self_.e42_ * other.e423_), -(self_.e41_ * other.e1_) - (self_.e42_ * other.e2_) - (self_.e43_ * other.e3_) - (self_.e23_ * other.e423_) - (self_.e31_ * other.e431_) - (self_.e12_ * other.e412_)) + (vec4<f32>(self_.e1234_) * other_groups.group4_) + (vec4<f32>(other.e4_) * vec4<f32>(self_.e41_, self_.e42_, self_.e43_, self_.scalar))
    ));
}
fn motor_geometricAntiProduct_origin(self_: Motor, other: Origin) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e4_) * vec4<f32>(self_.e23_, self_.e31_, self_.e12_, self_.e1234_), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e4_) * vec4<f32>(self_.e41_, self_.e42_, self_.e43_, self_.scalar)
    ));
}
fn motor_geometricAntiProduct_plane(self_: Motor, other: Plane) -> Flector {
    let other_groups = plane_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e41_ * other.e321_) + (self_.e31_ * other.e412_) + (self_.scalar * other.e423_) - (self_.e12_ * other.e431_), (self_.e42_ * other.e321_) + (self_.e12_ * other.e423_) + (self_.scalar * other.e431_) - (self_.e23_ * other.e412_), (self_.e43_ * other.e321_) + (self_.e23_ * other.e431_) + (self_.scalar * other.e412_) - (self_.e31_ * other.e423_), -(self_.e41_ * other.e423_) - (self_.e42_ * other.e431_) - (self_.e43_ * other.e412_)), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e42_ * other.e412_) - (self_.e43_ * other.e431_), (self_.e43_ * other.e423_) - (self_.e41_ * other.e412_), (self_.e41_ * other.e431_) - (self_.e42_ * other.e423_), -(self_.e23_ * other.e423_) - (self_.e31_ * other.e431_) - (self_.e12_ * other.e412_)) + (vec4<f32>(self_.e1234_) * other_groups.group0_)
    ));
}
fn motor_geometricAntiProduct_point(self_: Motor, other: Point) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e42_ * other.e3_) + (self_.e1234_ * other.e1_) + (self_.e23_ * other.e4_) - (self_.e43_ * other.e2_), (self_.e43_ * other.e1_) + (self_.e1234_ * other.e2_) + (self_.e31_ * other.e4_) - (self_.e41_ * other.e3_), (self_.e41_ * other.e2_) + (self_.e1234_ * other.e3_) + (self_.e12_ * other.e4_) - (self_.e42_ * other.e1_), self_.e1234_ * other.e4_), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e41_ * other.e4_, self_.e42_ * other.e4_, self_.e43_ * other.e4_, (self_.scalar * other.e4_) - (self_.e41_ * other.e1_) - (self_.e42_ * other.e2_) - (self_.e43_ * other.e3_))
    ));
}
fn motor_geometricAntiProduct_scalar(self_: Motor, other: Scalar) -> Motor {
    let self_groups = motor_grouped(self_);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(other.scalar) * self_groups.group0_
    ));
}
fn multiVector_geometricAntiProduct_antiScalar(self_: MultiVector, other: AntiScalar) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ (vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(other.e1234_) * self_groups.group1_, 
        /* e41, e42, e43 */ (vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_, 
        /* e23, e31, e12 */ (vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e1234_) * self_groups.group4_
    ));
}
fn multiVector_geometricAntiProduct_dualNum(self_: MultiVector, other: DualNum) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((other.scalar * self_.e1234_) + (other.e1234_ * self_.scalar), other.e1234_ * self_.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((other.e1234_ * self_.e1_) - (other.scalar * self_.e423_), (other.e1234_ * self_.e2_) - (other.scalar * self_.e431_), (other.e1234_ * self_.e3_) - (other.scalar * self_.e412_), other.e1234_ * self_.e4_), 
        /* e41, e42, e43 */ (vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_, 
        /* e23, e31, e12 */ ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_) + ((vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e1234_ * self_.e423_, other.e1234_ * self_.e431_, other.e1234_ * self_.e412_, (other.e1234_ * self_.e321_) - (other.scalar * self_.e4_))
    ));
}
fn multiVector_geometricAntiProduct_flector(self_: MultiVector, other: Flector) -> MultiVector {
    let other_groups = flector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((other.e321_ * self_.e4_) - (other.e1_ * self_.e423_) - (other.e2_ * self_.e431_) - (other.e3_ * self_.e412_), 0.0, 0.0, 0.0) + ((vec4<f32>(other.e423_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e1_, self_.e423_, 0.0, 0.0)) + ((vec4<f32>(other.e431_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e2_, self_.e431_, 0.0, 0.0)) + ((vec4<f32>(other.e412_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e3_, self_.e412_, 0.0, 0.0)) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e321_, self_.e4_, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((other.e3_ * self_.e42_) + (other.e4_ * self_.e23_) + (other.e423_ * self_.scalar) + (other.e412_ * self_.e31_) + (other.e321_ * self_.e41_) - (other.e2_ * self_.e43_) - (other.e431_ * self_.e12_), (other.e1_ * self_.e43_) + (other.e4_ * self_.e31_) + (other.e423_ * self_.e12_) + (other.e431_ * self_.scalar) + (other.e321_ * self_.e42_) - (other.e3_ * self_.e41_) - (other.e412_ * self_.e23_), (other.e2_ * self_.e41_) + (other.e4_ * self_.e12_) + (other.e431_ * self_.e23_) + (other.e412_ * self_.scalar) + (other.e321_ * self_.e43_) - (other.e1_ * self_.e42_) - (other.e423_ * self_.e31_), -(other.e423_ * self_.e41_) - (other.e431_ * self_.e42_) - (other.e412_ * self_.e43_)) + (vec4<f32>(self_.e1234_) * other_groups.group0_), 
        /* e41, e42, e43 */ vec4<f32>((other.e431_ * self_.e412_) - (other.e412_ * self_.e431_), (other.e412_ * self_.e423_) - (other.e423_ * self_.e412_), (other.e423_ * self_.e431_) - (other.e431_ * self_.e423_), 0.0) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0)) - ((vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((other.e3_ * self_.e431_) + (other.e431_ * self_.e3_) - (other.e2_ * self_.e412_) - (other.e412_ * self_.e2_), (other.e1_ * self_.e412_) + (other.e412_ * self_.e1_) - (other.e3_ * self_.e423_) - (other.e423_ * self_.e3_), (other.e2_ * self_.e423_) + (other.e423_ * self_.e2_) - (other.e1_ * self_.e431_) - (other.e431_ * self_.e1_), 0.0) + ((vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0)) + ((vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0)) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0)) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e412_ * self_.e42_) - (other.e431_ * self_.e43_), (other.e423_ * self_.e43_) - (other.e412_ * self_.e41_), (other.e431_ * self_.e41_) - (other.e423_ * self_.e42_), -(other.e1_ * self_.e41_) - (other.e2_ * self_.e42_) - (other.e3_ * self_.e43_) - (other.e423_ * self_.e23_) - (other.e431_ * self_.e31_) - (other.e412_ * self_.e12_)) + (vec4<f32>(other.e4_) * vec4<f32>(self_.e41_, self_.e42_, self_.e43_, self_.scalar)) + (vec4<f32>(self_.e1234_) * other_groups.group1_)
    ));
}
fn multiVector_geometricAntiProduct_horizon(self_: MultiVector, other: Horizon) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.e321_ * self_.e4_, 1.0, 0.0, 0.0) * vec2<f32>(1.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(other.e321_ * self_.e41_, other.e321_ * self_.e42_, other.e321_ * self_.e43_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ (vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0, 1.0, 1.0, other.e321_ * self_.e1234_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)
    ));
}
fn multiVector_geometricAntiProduct_line(self_: MultiVector, other: Line) -> MultiVector {
    let other_groups = line_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(-(other.e23_ * self_.e41_) - (other.e31_ * self_.e42_) - (other.e12_ * self_.e43_), 0.0, 0.0, 0.0) - ((vec4<f32>(other.e41_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e23_, self_.e41_, 0.0, 0.0)) - ((vec4<f32>(other.e42_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e31_, self_.e42_, 0.0, 0.0)) - ((vec4<f32>(other.e43_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e12_, self_.e43_, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((other.e41_ * self_.e321_) + (other.e43_ * self_.e2_) + (other.e31_ * self_.e412_) - (other.e42_ * self_.e3_) - (other.e23_ * self_.e4_) - (other.e12_ * self_.e431_), (other.e41_ * self_.e3_) + (other.e42_ * self_.e321_) + (other.e12_ * self_.e423_) - (other.e43_ * self_.e1_) - (other.e23_ * self_.e412_) - (other.e31_ * self_.e4_), (other.e42_ * self_.e1_) + (other.e43_ * self_.e321_) + (other.e23_ * self_.e431_) - (other.e41_ * self_.e2_) - (other.e31_ * self_.e423_) - (other.e12_ * self_.e4_), -(other.e41_ * self_.e423_) - (other.e42_ * self_.e431_) - (other.e43_ * self_.e412_)), 
        /* e41, e42, e43 */ vec4<f32>((other.e43_ * self_.e42_) - (other.e42_ * self_.e43_), (other.e41_ * self_.e43_) - (other.e43_ * self_.e41_), (other.e42_ * self_.e41_) - (other.e41_ * self_.e42_), 0.0) + ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group0_), 
        /* e23, e31, e12 */ vec4<f32>((other.e43_ * self_.e31_) + (other.e12_ * self_.e42_) - (other.e42_ * self_.e12_) - (other.e31_ * self_.e43_), (other.e41_ * self_.e12_) + (other.e23_ * self_.e43_) - (other.e43_ * self_.e23_) - (other.e12_ * self_.e41_), (other.e42_ * self_.e23_) + (other.e31_ * self_.e41_) - (other.e41_ * self_.e31_) - (other.e23_ * self_.e42_), 0.0) + ((vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group0_) + ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group1_), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e41_ * self_.e4_) + (other.e43_ * self_.e431_) - (other.e42_ * self_.e412_), (other.e41_ * self_.e412_) + (other.e42_ * self_.e4_) - (other.e43_ * self_.e423_), (other.e42_ * self_.e423_) + (other.e43_ * self_.e4_) - (other.e41_ * self_.e431_), (other.e23_ * self_.e423_) + (other.e31_ * self_.e431_) + (other.e12_ * self_.e412_) - (other.e41_ * self_.e1_) - (other.e42_ * self_.e2_) - (other.e43_ * self_.e3_))
    ));
}
fn multiVector_geometricAntiProduct_motor(self_: MultiVector, other: Motor) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((other.scalar * self_.e1234_) - (other.e23_ * self_.e41_) - (other.e31_ * self_.e42_) - (other.e12_ * self_.e43_), 0.0, 0.0, 0.0) + ((vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_) - ((vec4<f32>(other.e41_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e23_, self_.e41_, 0.0, 0.0)) - ((vec4<f32>(other.e42_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e31_, self_.e42_, 0.0, 0.0)) - ((vec4<f32>(other.e43_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e12_, self_.e43_, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((other.e41_ * self_.e321_) + (other.e43_ * self_.e2_) + (other.e31_ * self_.e412_) - (other.e42_ * self_.e3_) - (other.e23_ * self_.e4_) - (other.e12_ * self_.e431_) - (other.scalar * self_.e423_), (other.e41_ * self_.e3_) + (other.e42_ * self_.e321_) + (other.e12_ * self_.e423_) - (other.e43_ * self_.e1_) - (other.e23_ * self_.e412_) - (other.e31_ * self_.e4_) - (other.scalar * self_.e431_), (other.e42_ * self_.e1_) + (other.e43_ * self_.e321_) + (other.e23_ * self_.e431_) - (other.e41_ * self_.e2_) - (other.e31_ * self_.e423_) - (other.e12_ * self_.e4_) - (other.scalar * self_.e412_), -(other.e41_ * self_.e423_) - (other.e42_ * self_.e431_) - (other.e43_ * self_.e412_)) + (vec4<f32>(other.e1234_) * self_groups.group1_), 
        /* e41, e42, e43 */ vec4<f32>((other.e43_ * self_.e42_) - (other.e42_ * self_.e43_), (other.e41_ * self_.e43_) - (other.e43_ * self_.e41_), (other.e42_ * self_.e41_) - (other.e41_ * self_.e42_), 0.0) + ((vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_) + ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e41_, other.e42_, other.e43_, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((other.e43_ * self_.e31_) + (other.e12_ * self_.e42_) - (other.e42_ * self_.e12_) - (other.e31_ * self_.e43_), (other.e41_ * self_.e12_) + (other.e23_ * self_.e43_) - (other.e43_ * self_.e23_) - (other.e12_ * self_.e41_), (other.e42_ * self_.e23_) + (other.e31_ * self_.e41_) - (other.e41_ * self_.e31_) - (other.e23_ * self_.e42_), 0.0) + ((vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_) + ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_) + ((vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e41_, other.e42_, other.e43_, 0.0)) + ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e23_, other.e31_, other.e12_, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e41_ * self_.e4_) + (other.e43_ * self_.e431_) - (other.e42_ * self_.e412_), (other.e41_ * self_.e412_) + (other.e42_ * self_.e4_) - (other.e43_ * self_.e423_), (other.e42_ * self_.e423_) + (other.e43_ * self_.e4_) - (other.e41_ * self_.e431_), (other.e23_ * self_.e423_) + (other.e31_ * self_.e431_) + (other.e12_ * self_.e412_) - (other.e41_ * self_.e1_) - (other.e42_ * self_.e2_) - (other.e43_ * self_.e3_) - (other.scalar * self_.e4_)) + (vec4<f32>(other.e1234_) * self_groups.group4_)
    ));
}
fn multiVector_geometricAntiProduct_multiVector(self_: MultiVector, other: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((other.e1234_ * self_.scalar) + (other.e321_ * self_.e4_) - (other.e1_ * self_.e423_) - (other.e2_ * self_.e431_) - (other.e3_ * self_.e412_) - (other.e23_ * self_.e41_) - (other.e31_ * self_.e42_) - (other.e12_ * self_.e43_), 0.0, 0.0, 0.0) + ((vec4<f32>(other.e423_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e1_, self_.e423_, 0.0, 0.0)) + ((vec4<f32>(other.e431_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e2_, self_.e431_, 0.0, 0.0)) + ((vec4<f32>(other.e412_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e3_, self_.e412_, 0.0, 0.0)) + ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * other_groups.group0_) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e321_, self_.e4_, 0.0, 0.0)) - ((vec4<f32>(other.e41_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e23_, self_.e41_, 0.0, 0.0)) - ((vec4<f32>(other.e42_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e31_, self_.e42_, 0.0, 0.0)) - ((vec4<f32>(other.e43_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e12_, self_.e43_, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((other.e3_ * self_.e42_) + (other.e4_ * self_.e23_) + (other.e41_ * self_.e321_) + (other.e43_ * self_.e2_) + (other.e31_ * self_.e412_) + (other.e423_ * self_.scalar) + (other.e412_ * self_.e31_) + (other.e321_ * self_.e41_) - (other.scalar * self_.e423_) - (other.e2_ * self_.e43_) - (other.e42_ * self_.e3_) - (other.e23_ * self_.e4_) - (other.e12_ * self_.e431_) - (other.e431_ * self_.e12_), (other.e1_ * self_.e43_) + (other.e4_ * self_.e31_) + (other.e41_ * self_.e3_) + (other.e42_ * self_.e321_) + (other.e12_ * self_.e423_) + (other.e423_ * self_.e12_) + (other.e431_ * self_.scalar) + (other.e321_ * self_.e42_) - (other.scalar * self_.e431_) - (other.e3_ * self_.e41_) - (other.e43_ * self_.e1_) - (other.e23_ * self_.e412_) - (other.e31_ * self_.e4_) - (other.e412_ * self_.e23_), (other.e2_ * self_.e41_) + (other.e4_ * self_.e12_) + (other.e42_ * self_.e1_) + (other.e43_ * self_.e321_) + (other.e23_ * self_.e431_) + (other.e431_ * self_.e23_) + (other.e412_ * self_.scalar) + (other.e321_ * self_.e43_) - (other.scalar * self_.e412_) - (other.e1_ * self_.e42_) - (other.e41_ * self_.e2_) - (other.e31_ * self_.e423_) - (other.e12_ * self_.e4_) - (other.e423_ * self_.e31_), -(other.e41_ * self_.e423_) - (other.e42_ * self_.e431_) - (other.e43_ * self_.e412_) - (other.e423_ * self_.e41_) - (other.e431_ * self_.e42_) - (other.e412_ * self_.e43_)) + (vec4<f32>(other.e1234_) * self_groups.group1_) + (vec4<f32>(self_.e1234_) * other_groups.group1_), 
        /* e41, e42, e43 */ vec4<f32>((other.e43_ * self_.e42_) + (other.e431_ * self_.e412_) - (other.e42_ * self_.e43_) - (other.e412_ * self_.e431_), (other.e41_ * self_.e43_) + (other.e412_ * self_.e423_) - (other.e43_ * self_.e41_) - (other.e423_ * self_.e412_), (other.e42_ * self_.e41_) + (other.e423_ * self_.e431_) - (other.e41_ * self_.e42_) - (other.e431_ * self_.e423_), 0.0) + ((vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_) + ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group2_) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0)) - ((vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((other.e3_ * self_.e431_) + (other.e43_ * self_.e31_) + (other.e12_ * self_.e42_) + (other.e431_ * self_.e3_) - (other.e2_ * self_.e412_) - (other.e42_ * self_.e12_) - (other.e31_ * self_.e43_) - (other.e412_ * self_.e2_), (other.e1_ * self_.e412_) + (other.e41_ * self_.e12_) + (other.e23_ * self_.e43_) + (other.e412_ * self_.e1_) - (other.e3_ * self_.e423_) - (other.e43_ * self_.e23_) - (other.e12_ * self_.e41_) - (other.e423_ * self_.e3_), (other.e2_ * self_.e423_) + (other.e42_ * self_.e23_) + (other.e31_ * self_.e41_) + (other.e423_ * self_.e2_) - (other.e1_ * self_.e431_) - (other.e41_ * self_.e31_) - (other.e23_ * self_.e42_) - (other.e431_ * self_.e1_), 0.0) + ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_) + ((vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_) + ((vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0)) + ((vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group2_) + ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group3_) + ((vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0)) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0)) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e41_ * self_.e4_) + (other.e43_ * self_.e431_) + (other.e412_ * self_.e42_) - (other.e42_ * self_.e412_) - (other.e431_ * self_.e43_), (other.e41_ * self_.e412_) + (other.e42_ * self_.e4_) + (other.e423_ * self_.e43_) - (other.e43_ * self_.e423_) - (other.e412_ * self_.e41_), (other.e42_ * self_.e423_) + (other.e43_ * self_.e4_) + (other.e431_ * self_.e41_) - (other.e41_ * self_.e431_) - (other.e423_ * self_.e42_), (other.e23_ * self_.e423_) + (other.e31_ * self_.e431_) + (other.e12_ * self_.e412_) - (other.scalar * self_.e4_) - (other.e1_ * self_.e41_) - (other.e2_ * self_.e42_) - (other.e3_ * self_.e43_) - (other.e41_ * self_.e1_) - (other.e42_ * self_.e2_) - (other.e43_ * self_.e3_) - (other.e423_ * self_.e23_) - (other.e431_ * self_.e31_) - (other.e412_ * self_.e12_)) + (vec4<f32>(other.e1234_) * self_groups.group4_) + (vec4<f32>(other.e4_) * vec4<f32>(self_.e41_, self_.e42_, self_.e43_, self_.scalar)) + (vec4<f32>(self_.e1234_) * other_groups.group4_)
    ));
}
fn multiVector_geometricAntiProduct_origin(self_: MultiVector, other: Origin) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ (vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e321_, self_.e4_, 0.0, 0.0) * vec2<f32>(-1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(other.e4_) * vec4<f32>(self_.e23_, self_.e31_, self_.e12_, self_.e1234_), 
        /* e41, e42, e43 */ (vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ (vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0) * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e4_) * vec4<f32>(self_.e41_, self_.e42_, self_.e43_, self_.scalar)
    ));
}
fn multiVector_geometricAntiProduct_plane(self_: MultiVector, other: Plane) -> MultiVector {
    let other_groups = plane_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.e4_ * other.e321_, 0.0, 0.0, 0.0) + ((vec4<f32>(other.e423_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e1_, self_.e423_, 0.0, 0.0)) + ((vec4<f32>(other.e431_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e2_, self_.e431_, 0.0, 0.0)) + ((vec4<f32>(other.e412_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e3_, self_.e412_, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.scalar * other.e423_) + (self_.e41_ * other.e321_) + (self_.e31_ * other.e412_) - (self_.e12_ * other.e431_), (self_.scalar * other.e431_) + (self_.e42_ * other.e321_) + (self_.e12_ * other.e423_) - (self_.e23_ * other.e412_), (self_.scalar * other.e412_) + (self_.e43_ * other.e321_) + (self_.e23_ * other.e431_) - (self_.e31_ * other.e423_), -(self_.e41_ * other.e423_) - (self_.e42_ * other.e431_) - (self_.e43_ * other.e412_)), 
        /* e41, e42, e43 */ vec4<f32>((self_.e412_ * other.e431_) - (self_.e431_ * other.e412_), (self_.e423_ * other.e412_) - (self_.e412_ * other.e423_), (self_.e431_ * other.e423_) - (self_.e423_ * other.e431_), 0.0) - ((vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((self_.e3_ * other.e431_) - (self_.e2_ * other.e412_), (self_.e1_ * other.e412_) - (self_.e3_ * other.e423_), (self_.e2_ * other.e423_) - (self_.e1_ * other.e431_), 0.0) + ((vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0)) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e42_ * other.e412_) - (self_.e43_ * other.e431_), (self_.e43_ * other.e423_) - (self_.e41_ * other.e412_), (self_.e41_ * other.e431_) - (self_.e42_ * other.e423_), -(self_.e23_ * other.e423_) - (self_.e31_ * other.e431_) - (self_.e12_ * other.e412_)) + (vec4<f32>(self_.e1234_) * other_groups.group0_)
    ));
}
fn multiVector_geometricAntiProduct_point(self_: MultiVector, other: Point) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(-(self_.e423_ * other.e1_) - (self_.e431_ * other.e2_) - (self_.e412_ * other.e3_) - (self_.e321_ * other.e4_), self_.e4_ * other.e4_, 0.0, 0.0) * vec2<f32>(1.0, -1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e1234_ * other.e1_) + (self_.e42_ * other.e3_) + (self_.e23_ * other.e4_) - (self_.e43_ * other.e2_), (self_.e1234_ * other.e2_) + (self_.e43_ * other.e1_) + (self_.e31_ * other.e4_) - (self_.e41_ * other.e3_), (self_.e1234_ * other.e3_) + (self_.e41_ * other.e2_) + (self_.e12_ * other.e4_) - (self_.e42_ * other.e1_), self_.e1234_ * other.e4_), 
        /* e41, e42, e43 */ (vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>((self_.e431_ * other.e3_) - (self_.e412_ * other.e2_), (self_.e412_ * other.e1_) - (self_.e423_ * other.e3_), (self_.e423_ * other.e2_) - (self_.e431_ * other.e1_), 0.0) + ((vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0)) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e41_ * other.e4_, self_.e42_ * other.e4_, self_.e43_ * other.e4_, (self_.scalar * other.e4_) - (self_.e41_ * other.e1_) - (self_.e42_ * other.e2_) - (self_.e43_ * other.e3_))
    ));
}
fn multiVector_geometricAntiProduct_scalar(self_: MultiVector, other: Scalar) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.e1234_ * other.scalar, 1.0, 0.0, 0.0) * vec2<f32>(1.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e423_ * other.scalar, self_.e431_ * other.scalar, self_.e412_ * other.scalar, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ (vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_, 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0, 1.0, 1.0, self_.e4_ * other.scalar) * vec4<f32>(0.0, 0.0, 0.0, -1.0)
    ));
}
fn origin_geometricAntiProduct_antiScalar(self_: Origin, other: AntiScalar) -> Origin {
    return Origin(other.e1234_ * self_.e4_);
}
fn origin_geometricAntiProduct_dualNum(self_: Origin, other: DualNum) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, other.e1234_ * self_.e4_) * vec4<f32>(0.0, 0.0, 0.0, 1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0, 1.0, 1.0, other.scalar * self_.e4_) * vec4<f32>(0.0, 0.0, 0.0, -1.0)
    ));
}
fn origin_geometricAntiProduct_flector(self_: Origin, other: Flector) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.e4_) * vec4<f32>(other.e423_, other.e431_, other.e412_, other.e4_) * vec4<f32>(-1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.e4_) * vec4<f32>(other.e1_, other.e2_, other.e3_, other.e321_)
    ));
}
fn origin_geometricAntiProduct_horizon(self_: Origin, other: Horizon) -> Scalar {
    return Scalar(other.e321_ * self_.e4_);
}
fn origin_geometricAntiProduct_line(self_: Origin, other: Line) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e23_ * self_.e4_, other.e31_ * self_.e4_, other.e12_ * self_.e4_, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e41_ * self_.e4_, other.e42_ * self_.e4_, other.e43_ * self_.e4_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
}
fn origin_geometricAntiProduct_motor(self_: Origin, other: Motor) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e4_) * vec4<f32>(other.e23_, other.e31_, other.e12_, other.e1234_) * vec4<f32>(-1.0, -1.0, -1.0, 1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e4_) * vec4<f32>(other.e41_, other.e42_, other.e43_, other.scalar) * vec4<f32>(1.0, 1.0, 1.0, -1.0)
    ));
}
fn origin_geometricAntiProduct_multiVector(self_: Origin, other: MultiVector) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ (vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e321_, other.e4_, 0.0, 0.0) * vec2<f32>(1.0, -1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e4_) * vec4<f32>(other.e23_, other.e31_, other.e12_, other.e1234_) * vec4<f32>(-1.0, -1.0, -1.0, 1.0), 
        /* e41, e42, e43 */ (vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ (vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e4_) * vec4<f32>(other.e41_, other.e42_, other.e43_, other.scalar) * vec4<f32>(1.0, 1.0, 1.0, -1.0)
    ));
}
fn origin_geometricAntiProduct_origin(self_: Origin, other: Origin) -> AntiScalar {
    return AntiScalar(other.e4_ * self_.e4_ * -1.0);
}
fn origin_geometricAntiProduct_plane(self_: Origin, other: Plane) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.e4_ * other.e423_, self_.e4_ * other.e431_, self_.e4_ * other.e412_, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(1.0, 1.0, 1.0, self_.e4_ * other.e321_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)
    ));
}
fn origin_geometricAntiProduct_point(self_: Origin, other: Point) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(1.0, 1.0, 1.0, self_.e4_ * other.e4_) * vec4<f32>(0.0, 0.0, 0.0, -1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.e4_ * other.e1_, self_.e4_ * other.e2_, self_.e4_ * other.e3_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
}
fn origin_geometricAntiProduct_scalar(self_: Origin, other: Scalar) -> Horizon {
    return Horizon(self_.e4_ * other.scalar * -1.0);
}
fn plane_geometricAntiProduct_antiScalar(self_: Plane, other: AntiScalar) -> Plane {
    let self_groups = plane_grouped(self_);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(other.e1234_) * self_groups.group0_
    ));
}
fn plane_geometricAntiProduct_dualNum(self_: Plane, other: DualNum) -> Flector {
    let self_groups = plane_grouped(self_);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.scalar * self_.e423_, other.scalar * self_.e431_, other.scalar * self_.e412_, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e1234_) * self_groups.group0_
    ));
}
fn plane_geometricAntiProduct_flector(self_: Plane, other: Flector) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other.e431_ * self_.e412_) - (other.e4_ * self_.e423_) - (other.e412_ * self_.e431_), (other.e412_ * self_.e423_) - (other.e4_ * self_.e431_) - (other.e423_ * self_.e412_), (other.e423_ * self_.e431_) - (other.e4_ * self_.e412_) - (other.e431_ * self_.e423_), (other.e423_ * self_.e423_) + (other.e431_ * self_.e431_) + (other.e412_ * self_.e412_)), 
        /* e23, e31, e12, scalar */ vec4<f32>((other.e3_ * self_.e431_) + (other.e321_ * self_.e423_) - (other.e2_ * self_.e412_), (other.e1_ * self_.e412_) + (other.e321_ * self_.e431_) - (other.e3_ * self_.e423_), (other.e2_ * self_.e423_) + (other.e321_ * self_.e412_) - (other.e1_ * self_.e431_), -(other.e1_ * self_.e423_) - (other.e2_ * self_.e431_) - (other.e3_ * self_.e412_)) - (vec4<f32>(self_.e321_) * vec4<f32>(other.e423_, other.e431_, other.e412_, other.e4_))
    ));
}
fn plane_geometricAntiProduct_horizon(self_: Plane, other: Horizon) -> Line {
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ (vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0)
    ));
}
fn plane_geometricAntiProduct_line(self_: Plane, other: Line) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other.e41_ * self_.e321_) + (other.e31_ * self_.e412_) - (other.e12_ * self_.e431_), (other.e42_ * self_.e321_) + (other.e12_ * self_.e423_) - (other.e23_ * self_.e412_), (other.e43_ * self_.e321_) + (other.e23_ * self_.e431_) - (other.e31_ * self_.e423_), -(other.e41_ * self_.e423_) - (other.e42_ * self_.e431_) - (other.e43_ * self_.e412_)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e43_ * self_.e431_) - (other.e42_ * self_.e412_), (other.e41_ * self_.e412_) - (other.e43_ * self_.e423_), (other.e42_ * self_.e423_) - (other.e41_ * self_.e431_), (other.e23_ * self_.e423_) + (other.e31_ * self_.e431_) + (other.e12_ * self_.e412_))
    ));
}
fn plane_geometricAntiProduct_motor(self_: Plane, other: Motor) -> Flector {
    let self_groups = plane_grouped(self_);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other.e41_ * self_.e321_) + (other.e31_ * self_.e412_) - (other.e12_ * self_.e431_) - (other.scalar * self_.e423_), (other.e42_ * self_.e321_) + (other.e12_ * self_.e423_) - (other.e23_ * self_.e412_) - (other.scalar * self_.e431_), (other.e43_ * self_.e321_) + (other.e23_ * self_.e431_) - (other.e31_ * self_.e423_) - (other.scalar * self_.e412_), -(other.e41_ * self_.e423_) - (other.e42_ * self_.e431_) - (other.e43_ * self_.e412_)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e43_ * self_.e431_) - (other.e42_ * self_.e412_), (other.e41_ * self_.e412_) - (other.e43_ * self_.e423_), (other.e42_ * self_.e423_) - (other.e41_ * self_.e431_), (other.e23_ * self_.e423_) + (other.e31_ * self_.e431_) + (other.e12_ * self_.e412_)) + (vec4<f32>(other.e1234_) * self_groups.group0_)
    ));
}
fn plane_geometricAntiProduct_multiVector(self_: Plane, other: MultiVector) -> MultiVector {
    let self_groups = plane_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(-(other.e1_ * self_.e423_) - (other.e2_ * self_.e431_) - (other.e3_ * self_.e412_) - (other.e4_ * self_.e321_), (other.e423_ * self_.e423_) + (other.e431_ * self_.e431_) + (other.e412_ * self_.e412_), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((other.e41_ * self_.e321_) + (other.e31_ * self_.e412_) - (other.scalar * self_.e423_) - (other.e12_ * self_.e431_), (other.e42_ * self_.e321_) + (other.e12_ * self_.e423_) - (other.scalar * self_.e431_) - (other.e23_ * self_.e412_), (other.e43_ * self_.e321_) + (other.e23_ * self_.e431_) - (other.scalar * self_.e412_) - (other.e31_ * self_.e423_), -(other.e41_ * self_.e423_) - (other.e42_ * self_.e431_) - (other.e43_ * self_.e412_)), 
        /* e41, e42, e43 */ vec4<f32>((other.e431_ * self_.e412_) - (other.e412_ * self_.e431_), (other.e412_ * self_.e423_) - (other.e423_ * self_.e412_), (other.e423_ * self_.e431_) - (other.e431_ * self_.e423_), 0.0) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((other.e3_ * self_.e431_) - (other.e2_ * self_.e412_), (other.e1_ * self_.e412_) - (other.e3_ * self_.e423_), (other.e2_ * self_.e423_) - (other.e1_ * self_.e431_), 0.0) + ((vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0)) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e43_ * self_.e431_) - (other.e42_ * self_.e412_), (other.e41_ * self_.e412_) - (other.e43_ * self_.e423_), (other.e42_ * self_.e423_) - (other.e41_ * self_.e431_), (other.e23_ * self_.e423_) + (other.e31_ * self_.e431_) + (other.e12_ * self_.e412_)) + (vec4<f32>(other.e1234_) * self_groups.group0_)
    ));
}
fn plane_geometricAntiProduct_origin(self_: Plane, other: Origin) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e4_ * self_.e423_, other.e4_ * self_.e431_, other.e4_ * self_.e412_, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(1.0, 1.0, 1.0, other.e4_ * self_.e321_) * vec4<f32>(0.0, 0.0, 0.0, -1.0)
    ));
}
fn plane_geometricAntiProduct_plane(self_: Plane, other: Plane) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other.e431_ * self_.e412_) - (other.e412_ * self_.e431_), (other.e412_ * self_.e423_) - (other.e423_ * self_.e412_), (other.e423_ * self_.e431_) - (other.e431_ * self_.e423_), (other.e423_ * self_.e423_) + (other.e431_ * self_.e431_) + (other.e412_ * self_.e412_)), 
        /* e23, e31, e12, scalar */ vec4<f32>((other.e321_ * self_.e423_) - (other.e423_ * self_.e321_), (other.e321_ * self_.e431_) - (other.e431_ * self_.e321_), (other.e321_ * self_.e412_) - (other.e412_ * self_.e321_), 0.0)
    ));
}
fn plane_geometricAntiProduct_point(self_: Plane, other: Point) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.e423_ * other.e4_, self_.e431_ * other.e4_, self_.e412_ * other.e4_, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>((self_.e431_ * other.e3_) - (self_.e412_ * other.e2_), (self_.e412_ * other.e1_) - (self_.e423_ * other.e3_), (self_.e423_ * other.e2_) - (self_.e431_ * other.e1_), -(self_.e423_ * other.e1_) - (self_.e431_ * other.e2_) - (self_.e412_ * other.e3_) - (self_.e321_ * other.e4_))
    ));
}
fn plane_geometricAntiProduct_scalar(self_: Plane, other: Scalar) -> Point {
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e423_ * other.scalar, self_.e431_ * other.scalar, self_.e412_ * other.scalar, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0)
    ));
}
fn point_geometricAntiProduct_antiScalar(self_: Point, other: AntiScalar) -> Point {
    let self_groups = point_grouped(self_);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e1234_) * self_groups.group0_
    ));
}
fn point_geometricAntiProduct_dualNum(self_: Point, other: DualNum) -> Flector {
    let self_groups = point_grouped(self_);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e1234_) * self_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0, 1.0, 1.0, other.scalar * self_.e4_) * vec4<f32>(0.0, 0.0, 0.0, -1.0)
    ));
}
fn point_geometricAntiProduct_flector(self_: Point, other: Flector) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.e4_) * vec4<f32>(other.e423_, other.e431_, other.e412_, other.e4_) * vec4<f32>(-1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>((other.e431_ * self_.e3_) - (other.e4_ * self_.e1_) - (other.e412_ * self_.e2_), (other.e412_ * self_.e1_) - (other.e4_ * self_.e2_) - (other.e423_ * self_.e3_), (other.e423_ * self_.e2_) - (other.e4_ * self_.e3_) - (other.e431_ * self_.e1_), (other.e423_ * self_.e1_) + (other.e431_ * self_.e2_) + (other.e412_ * self_.e3_)) + (vec4<f32>(self_.e4_) * vec4<f32>(other.e1_, other.e2_, other.e3_, other.e321_))
    ));
}
fn point_geometricAntiProduct_horizon(self_: Point, other: Horizon) -> Scalar {
    return Scalar(other.e321_ * self_.e4_);
}
fn point_geometricAntiProduct_line(self_: Point, other: Line) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other.e43_ * self_.e2_) - (other.e42_ * self_.e3_) - (other.e23_ * self_.e4_), (other.e41_ * self_.e3_) - (other.e43_ * self_.e1_) - (other.e31_ * self_.e4_), (other.e42_ * self_.e1_) - (other.e41_ * self_.e2_) - (other.e12_ * self_.e4_), 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e41_ * self_.e4_, other.e42_ * self_.e4_, other.e43_ * self_.e4_, -(other.e41_ * self_.e1_) - (other.e42_ * self_.e2_) - (other.e43_ * self_.e3_))
    ));
}
fn point_geometricAntiProduct_motor(self_: Point, other: Motor) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other.e43_ * self_.e2_) + (other.e1234_ * self_.e1_) - (other.e42_ * self_.e3_) - (other.e23_ * self_.e4_), (other.e41_ * self_.e3_) + (other.e1234_ * self_.e2_) - (other.e43_ * self_.e1_) - (other.e31_ * self_.e4_), (other.e42_ * self_.e1_) + (other.e1234_ * self_.e3_) - (other.e41_ * self_.e2_) - (other.e12_ * self_.e4_), other.e1234_ * self_.e4_), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e41_ * self_.e4_, other.e42_ * self_.e4_, other.e43_ * self_.e4_, -(other.e41_ * self_.e1_) - (other.e42_ * self_.e2_) - (other.e43_ * self_.e3_) - (other.scalar * self_.e4_))
    ));
}
fn point_geometricAntiProduct_multiVector(self_: Point, other: MultiVector) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((other.e423_ * self_.e1_) + (other.e431_ * self_.e2_) + (other.e412_ * self_.e3_) + (other.e321_ * self_.e4_), other.e4_ * self_.e4_, 0.0, 0.0) * vec2<f32>(1.0, -1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((other.e1234_ * self_.e1_) + (other.e43_ * self_.e2_) - (other.e42_ * self_.e3_) - (other.e23_ * self_.e4_), (other.e1234_ * self_.e2_) + (other.e41_ * self_.e3_) - (other.e43_ * self_.e1_) - (other.e31_ * self_.e4_), (other.e1234_ * self_.e3_) + (other.e42_ * self_.e1_) - (other.e41_ * self_.e2_) - (other.e12_ * self_.e4_), other.e1234_ * self_.e4_), 
        /* e41, e42, e43 */ (vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>((other.e431_ * self_.e3_) - (other.e412_ * self_.e2_), (other.e412_ * self_.e1_) - (other.e423_ * self_.e3_), (other.e423_ * self_.e2_) - (other.e431_ * self_.e1_), 0.0) + ((vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0)) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e41_ * self_.e4_, other.e42_ * self_.e4_, other.e43_ * self_.e4_, -(other.scalar * self_.e4_) - (other.e41_ * self_.e1_) - (other.e42_ * self_.e2_) - (other.e43_ * self_.e3_))
    ));
}
fn point_geometricAntiProduct_origin(self_: Point, other: Origin) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(1.0, 1.0, 1.0, other.e4_ * self_.e4_) * vec4<f32>(0.0, 0.0, 0.0, -1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(other.e4_ * self_.e1_, other.e4_ * self_.e2_, other.e4_ * self_.e3_, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0)
    ));
}
fn point_geometricAntiProduct_plane(self_: Point, other: Plane) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e423_ * self_.e4_, other.e431_ * self_.e4_, other.e412_ * self_.e4_, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>((other.e431_ * self_.e3_) - (other.e412_ * self_.e2_), (other.e412_ * self_.e1_) - (other.e423_ * self_.e3_), (other.e423_ * self_.e2_) - (other.e431_ * self_.e1_), (other.e423_ * self_.e1_) + (other.e431_ * self_.e2_) + (other.e412_ * self_.e3_) + (other.e321_ * self_.e4_))
    ));
}
fn point_geometricAntiProduct_point(self_: Point, other: Point) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(1.0, 1.0, 1.0, other.e4_ * self_.e4_) * vec4<f32>(0.0, 0.0, 0.0, -1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>((other.e1_ * self_.e4_) - (other.e4_ * self_.e1_), (other.e2_ * self_.e4_) - (other.e4_ * self_.e2_), (other.e3_ * self_.e4_) - (other.e4_ * self_.e3_), 0.0)
    ));
}
fn point_geometricAntiProduct_scalar(self_: Point, other: Scalar) -> Horizon {
    return Horizon(self_.e4_ * other.scalar * -1.0);
}
fn scalar_geometricAntiProduct_antiScalar(self_: Scalar, other: AntiScalar) -> Scalar {
    return Scalar(other.e1234_ * self_.scalar);
}
fn scalar_geometricAntiProduct_dualNum(self_: Scalar, other: DualNum) -> Scalar {
    return Scalar(other.e1234_ * self_.scalar);
}
fn scalar_geometricAntiProduct_flector(self_: Scalar, other: Flector) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e423_ * self_.scalar, other.e431_ * self_.scalar, other.e412_ * self_.scalar, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0, 1.0, 1.0, other.e4_ * self_.scalar) * vec4<f32>(0.0, 0.0, 0.0, 1.0)
    ));
}
fn scalar_geometricAntiProduct_line(self_: Scalar, other: Line) -> Line {
    let other_groups = line_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ (vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group0_
    ));
}
fn scalar_geometricAntiProduct_motor(self_: Scalar, other: Motor) -> Motor {
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.scalar) * other_groups.group0_
    ));
}
fn scalar_geometricAntiProduct_multiVector(self_: Scalar, other: MultiVector) -> MultiVector {
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.e1234_ * self_.scalar, 1.0, 0.0, 0.0) * vec2<f32>(1.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(other.e423_ * self_.scalar, other.e431_ * self_.scalar, other.e412_ * self_.scalar, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ (vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group2_, 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0, 1.0, 1.0, other.e4_ * self_.scalar) * vec4<f32>(0.0, 0.0, 0.0, 1.0)
    ));
}
fn scalar_geometricAntiProduct_origin(self_: Scalar, other: Origin) -> Horizon {
    return Horizon(other.e4_ * self_.scalar);
}
fn scalar_geometricAntiProduct_plane(self_: Scalar, other: Plane) -> Point {
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e423_ * self_.scalar, other.e431_ * self_.scalar, other.e412_ * self_.scalar, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
}
fn scalar_geometricAntiProduct_point(self_: Scalar, other: Point) -> Horizon {
    return Horizon(other.e4_ * self_.scalar);
}
fn antiScalar_geometricAntiQuotient_antiScalar(self_: AntiScalar, other: AntiScalar) -> AntiScalar {
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return AntiScalar(anti_inverse.e1234_ * self_.e1234_);
}
fn antiScalar_geometricAntiQuotient_dualNum(self_: AntiScalar, other: DualNum) -> AntiScalar {
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return AntiScalar(anti_inverse.e1234_ * self_.e1234_);
}
fn antiScalar_geometricAntiQuotient_flector(self_: AntiScalar, other: Flector) -> AntiScalar {
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e423_, 2) + pow(other.e431_, 2) + pow(other.e412_, 2) - pow(other.e4_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return AntiScalar(anti_inverse.e1234_ * self_.e1234_);
}
fn antiScalar_geometricAntiQuotient_line(self_: AntiScalar, other: Line) -> AntiScalar {
    let anti_dot_product: AntiScalar = AntiScalar(-pow(other.e41_, 2) - pow(other.e42_, 2) - pow(other.e43_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return AntiScalar(anti_inverse.e1234_ * self_.e1234_);
}
fn antiScalar_geometricAntiQuotient_motor(self_: AntiScalar, other: Motor) -> AntiScalar {
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2) - pow(other.e41_, 2) - pow(other.e42_, 2) - pow(other.e43_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return AntiScalar(anti_inverse.e1234_ * self_.e1234_);
}
fn antiScalar_geometricAntiQuotient_multiVector(self_: AntiScalar, other: MultiVector) -> AntiScalar {
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2) + pow(other.e423_, 2) + pow(other.e431_, 2) + pow(other.e412_, 2) - pow(other.e4_, 2) - pow(other.e41_, 2) - pow(other.e42_, 2) - pow(other.e43_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return AntiScalar(anti_inverse.e1234_ * self_.e1234_);
}
fn antiScalar_geometricAntiQuotient_origin(self_: AntiScalar, other: Origin) -> AntiScalar {
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e4_, 2) * -1.0);
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return AntiScalar(anti_inverse.e1234_ * self_.e1234_);
}
fn antiScalar_geometricAntiQuotient_plane(self_: AntiScalar, other: Plane) -> AntiScalar {
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e423_, 2) + pow(other.e431_, 2) + pow(other.e412_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return AntiScalar(anti_inverse.e1234_ * self_.e1234_);
}
fn antiScalar_geometricAntiQuotient_point(self_: AntiScalar, other: Point) -> AntiScalar {
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e4_, 2) * -1.0);
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return AntiScalar(anti_inverse.e1234_ * self_.e1234_);
}
fn dualNum_geometricAntiQuotient_antiScalar(self_: DualNum, other: AntiScalar) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_
    ));
}
fn dualNum_geometricAntiQuotient_dualNum(self_: DualNum, other: DualNum) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_
    ));
}
fn dualNum_geometricAntiQuotient_flector(self_: DualNum, other: Flector) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e423_, 2) + pow(other.e431_, 2) + pow(other.e412_, 2) - pow(other.e4_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_
    ));
}
fn dualNum_geometricAntiQuotient_line(self_: DualNum, other: Line) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(-pow(other.e41_, 2) - pow(other.e42_, 2) - pow(other.e43_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_
    ));
}
fn dualNum_geometricAntiQuotient_motor(self_: DualNum, other: Motor) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2) - pow(other.e41_, 2) - pow(other.e42_, 2) - pow(other.e43_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_
    ));
}
fn dualNum_geometricAntiQuotient_multiVector(self_: DualNum, other: MultiVector) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2) + pow(other.e423_, 2) + pow(other.e431_, 2) + pow(other.e412_, 2) - pow(other.e4_, 2) - pow(other.e41_, 2) - pow(other.e42_, 2) - pow(other.e43_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_
    ));
}
fn dualNum_geometricAntiQuotient_origin(self_: DualNum, other: Origin) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e4_, 2) * -1.0);
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_
    ));
}
fn dualNum_geometricAntiQuotient_plane(self_: DualNum, other: Plane) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e423_, 2) + pow(other.e431_, 2) + pow(other.e412_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_
    ));
}
fn dualNum_geometricAntiQuotient_point(self_: DualNum, other: Point) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e4_, 2) * -1.0);
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_
    ));
}
fn flector_geometricAntiQuotient_antiScalar(self_: Flector, other: AntiScalar) -> Flector {
    let self_groups = flector_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group1_
    ));
}
fn flector_geometricAntiQuotient_dualNum(self_: Flector, other: DualNum) -> Flector {
    let self_groups = flector_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group1_
    ));
}
fn flector_geometricAntiQuotient_flector(self_: Flector, other: Flector) -> Flector {
    let self_groups = flector_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e423_, 2) + pow(other.e431_, 2) + pow(other.e412_, 2) - pow(other.e4_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group1_
    ));
}
fn flector_geometricAntiQuotient_line(self_: Flector, other: Line) -> Flector {
    let self_groups = flector_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(-pow(other.e41_, 2) - pow(other.e42_, 2) - pow(other.e43_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group1_
    ));
}
fn flector_geometricAntiQuotient_motor(self_: Flector, other: Motor) -> Flector {
    let self_groups = flector_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2) - pow(other.e41_, 2) - pow(other.e42_, 2) - pow(other.e43_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group1_
    ));
}
fn flector_geometricAntiQuotient_multiVector(self_: Flector, other: MultiVector) -> Flector {
    let self_groups = flector_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2) + pow(other.e423_, 2) + pow(other.e431_, 2) + pow(other.e412_, 2) - pow(other.e4_, 2) - pow(other.e41_, 2) - pow(other.e42_, 2) - pow(other.e43_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group1_
    ));
}
fn flector_geometricAntiQuotient_origin(self_: Flector, other: Origin) -> Flector {
    let self_groups = flector_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e4_, 2) * -1.0);
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group1_
    ));
}
fn flector_geometricAntiQuotient_plane(self_: Flector, other: Plane) -> Flector {
    let self_groups = flector_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e423_, 2) + pow(other.e431_, 2) + pow(other.e412_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group1_
    ));
}
fn flector_geometricAntiQuotient_point(self_: Flector, other: Point) -> Flector {
    let self_groups = flector_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e4_, 2) * -1.0);
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group1_
    ));
}
fn horizon_geometricAntiQuotient_antiScalar(self_: Horizon, other: AntiScalar) -> Horizon {
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return Horizon(anti_inverse.e1234_ * self_.e321_);
}
fn horizon_geometricAntiQuotient_dualNum(self_: Horizon, other: DualNum) -> Horizon {
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return Horizon(anti_inverse.e1234_ * self_.e321_);
}
fn horizon_geometricAntiQuotient_flector(self_: Horizon, other: Flector) -> Horizon {
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e423_, 2) + pow(other.e431_, 2) + pow(other.e412_, 2) - pow(other.e4_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return Horizon(anti_inverse.e1234_ * self_.e321_);
}
fn horizon_geometricAntiQuotient_line(self_: Horizon, other: Line) -> Horizon {
    let anti_dot_product: AntiScalar = AntiScalar(-pow(other.e41_, 2) - pow(other.e42_, 2) - pow(other.e43_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return Horizon(anti_inverse.e1234_ * self_.e321_);
}
fn horizon_geometricAntiQuotient_motor(self_: Horizon, other: Motor) -> Horizon {
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2) - pow(other.e41_, 2) - pow(other.e42_, 2) - pow(other.e43_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return Horizon(anti_inverse.e1234_ * self_.e321_);
}
fn horizon_geometricAntiQuotient_multiVector(self_: Horizon, other: MultiVector) -> Horizon {
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2) + pow(other.e423_, 2) + pow(other.e431_, 2) + pow(other.e412_, 2) - pow(other.e4_, 2) - pow(other.e41_, 2) - pow(other.e42_, 2) - pow(other.e43_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return Horizon(anti_inverse.e1234_ * self_.e321_);
}
fn horizon_geometricAntiQuotient_origin(self_: Horizon, other: Origin) -> Horizon {
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e4_, 2) * -1.0);
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return Horizon(anti_inverse.e1234_ * self_.e321_);
}
fn horizon_geometricAntiQuotient_plane(self_: Horizon, other: Plane) -> Horizon {
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e423_, 2) + pow(other.e431_, 2) + pow(other.e412_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return Horizon(anti_inverse.e1234_ * self_.e321_);
}
fn horizon_geometricAntiQuotient_point(self_: Horizon, other: Point) -> Horizon {
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e4_, 2) * -1.0);
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return Horizon(anti_inverse.e1234_ * self_.e321_);
}
fn line_geometricAntiQuotient_antiScalar(self_: Line, other: AntiScalar) -> Line {
    let self_groups = line_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_
    ));
}
fn line_geometricAntiQuotient_dualNum(self_: Line, other: DualNum) -> Line {
    let self_groups = line_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_
    ));
}
fn line_geometricAntiQuotient_flector(self_: Line, other: Flector) -> Line {
    let self_groups = line_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e423_, 2) + pow(other.e431_, 2) + pow(other.e412_, 2) - pow(other.e4_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_
    ));
}
fn line_geometricAntiQuotient_line(self_: Line, other: Line) -> Line {
    let self_groups = line_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(-pow(other.e41_, 2) - pow(other.e42_, 2) - pow(other.e43_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_
    ));
}
fn line_geometricAntiQuotient_motor(self_: Line, other: Motor) -> Line {
    let self_groups = line_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2) - pow(other.e41_, 2) - pow(other.e42_, 2) - pow(other.e43_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_
    ));
}
fn line_geometricAntiQuotient_multiVector(self_: Line, other: MultiVector) -> Line {
    let self_groups = line_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2) + pow(other.e423_, 2) + pow(other.e431_, 2) + pow(other.e412_, 2) - pow(other.e4_, 2) - pow(other.e41_, 2) - pow(other.e42_, 2) - pow(other.e43_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_
    ));
}
fn line_geometricAntiQuotient_origin(self_: Line, other: Origin) -> Line {
    let self_groups = line_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e4_, 2) * -1.0);
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_
    ));
}
fn line_geometricAntiQuotient_plane(self_: Line, other: Plane) -> Line {
    let self_groups = line_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e423_, 2) + pow(other.e431_, 2) + pow(other.e412_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_
    ));
}
fn line_geometricAntiQuotient_point(self_: Line, other: Point) -> Line {
    let self_groups = line_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e4_, 2) * -1.0);
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_
    ));
}
fn motor_geometricAntiQuotient_antiScalar(self_: Motor, other: AntiScalar) -> Motor {
    let self_groups = motor_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(anti_inverse.e1234_) * self_groups.group1_
    ));
}
fn motor_geometricAntiQuotient_dualNum(self_: Motor, other: DualNum) -> Motor {
    let self_groups = motor_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(anti_inverse.e1234_) * self_groups.group1_
    ));
}
fn motor_geometricAntiQuotient_flector(self_: Motor, other: Flector) -> Motor {
    let self_groups = motor_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e423_, 2) + pow(other.e431_, 2) + pow(other.e412_, 2) - pow(other.e4_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(anti_inverse.e1234_) * self_groups.group1_
    ));
}
fn motor_geometricAntiQuotient_line(self_: Motor, other: Line) -> Motor {
    let self_groups = motor_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(-pow(other.e41_, 2) - pow(other.e42_, 2) - pow(other.e43_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(anti_inverse.e1234_) * self_groups.group1_
    ));
}
fn motor_geometricAntiQuotient_motor(self_: Motor, other: Motor) -> Motor {
    let self_groups = motor_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2) - pow(other.e41_, 2) - pow(other.e42_, 2) - pow(other.e43_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(anti_inverse.e1234_) * self_groups.group1_
    ));
}
fn motor_geometricAntiQuotient_multiVector(self_: Motor, other: MultiVector) -> Motor {
    let self_groups = motor_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2) + pow(other.e423_, 2) + pow(other.e431_, 2) + pow(other.e412_, 2) - pow(other.e4_, 2) - pow(other.e41_, 2) - pow(other.e42_, 2) - pow(other.e43_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(anti_inverse.e1234_) * self_groups.group1_
    ));
}
fn motor_geometricAntiQuotient_origin(self_: Motor, other: Origin) -> Motor {
    let self_groups = motor_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e4_, 2) * -1.0);
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(anti_inverse.e1234_) * self_groups.group1_
    ));
}
fn motor_geometricAntiQuotient_plane(self_: Motor, other: Plane) -> Motor {
    let self_groups = motor_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e423_, 2) + pow(other.e431_, 2) + pow(other.e412_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(anti_inverse.e1234_) * self_groups.group1_
    ));
}
fn motor_geometricAntiQuotient_point(self_: Motor, other: Point) -> Motor {
    let self_groups = motor_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e4_, 2) * -1.0);
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(anti_inverse.e1234_) * self_groups.group1_
    ));
}
fn multiVector_geometricAntiQuotient_antiScalar(self_: MultiVector, other: AntiScalar) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group1_, 
        /* e41, e42, e43 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_, 
        /* e23, e31, e12 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group4_
    ));
}
fn multiVector_geometricAntiQuotient_dualNum(self_: MultiVector, other: DualNum) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group1_, 
        /* e41, e42, e43 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_, 
        /* e23, e31, e12 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group4_
    ));
}
fn multiVector_geometricAntiQuotient_flector(self_: MultiVector, other: Flector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e423_, 2) + pow(other.e431_, 2) + pow(other.e412_, 2) - pow(other.e4_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group1_, 
        /* e41, e42, e43 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_, 
        /* e23, e31, e12 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group4_
    ));
}
fn multiVector_geometricAntiQuotient_line(self_: MultiVector, other: Line) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(-pow(other.e41_, 2) - pow(other.e42_, 2) - pow(other.e43_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group1_, 
        /* e41, e42, e43 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_, 
        /* e23, e31, e12 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group4_
    ));
}
fn multiVector_geometricAntiQuotient_motor(self_: MultiVector, other: Motor) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2) - pow(other.e41_, 2) - pow(other.e42_, 2) - pow(other.e43_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group1_, 
        /* e41, e42, e43 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_, 
        /* e23, e31, e12 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group4_
    ));
}
fn multiVector_geometricAntiQuotient_multiVector(self_: MultiVector, other: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2) + pow(other.e423_, 2) + pow(other.e431_, 2) + pow(other.e412_, 2) - pow(other.e4_, 2) - pow(other.e41_, 2) - pow(other.e42_, 2) - pow(other.e43_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group1_, 
        /* e41, e42, e43 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_, 
        /* e23, e31, e12 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group4_
    ));
}
fn multiVector_geometricAntiQuotient_origin(self_: MultiVector, other: Origin) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e4_, 2) * -1.0);
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group1_, 
        /* e41, e42, e43 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_, 
        /* e23, e31, e12 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group4_
    ));
}
fn multiVector_geometricAntiQuotient_plane(self_: MultiVector, other: Plane) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e423_, 2) + pow(other.e431_, 2) + pow(other.e412_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group1_, 
        /* e41, e42, e43 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_, 
        /* e23, e31, e12 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group4_
    ));
}
fn multiVector_geometricAntiQuotient_point(self_: MultiVector, other: Point) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e4_, 2) * -1.0);
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group1_, 
        /* e41, e42, e43 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_, 
        /* e23, e31, e12 */ (vec4<f32>(anti_inverse.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group4_
    ));
}
fn origin_geometricAntiQuotient_antiScalar(self_: Origin, other: AntiScalar) -> Origin {
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return Origin(anti_inverse.e1234_ * self_.e4_);
}
fn origin_geometricAntiQuotient_dualNum(self_: Origin, other: DualNum) -> Origin {
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return Origin(anti_inverse.e1234_ * self_.e4_);
}
fn origin_geometricAntiQuotient_flector(self_: Origin, other: Flector) -> Origin {
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e423_, 2) + pow(other.e431_, 2) + pow(other.e412_, 2) - pow(other.e4_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return Origin(anti_inverse.e1234_ * self_.e4_);
}
fn origin_geometricAntiQuotient_line(self_: Origin, other: Line) -> Origin {
    let anti_dot_product: AntiScalar = AntiScalar(-pow(other.e41_, 2) - pow(other.e42_, 2) - pow(other.e43_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return Origin(anti_inverse.e1234_ * self_.e4_);
}
fn origin_geometricAntiQuotient_motor(self_: Origin, other: Motor) -> Origin {
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2) - pow(other.e41_, 2) - pow(other.e42_, 2) - pow(other.e43_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return Origin(anti_inverse.e1234_ * self_.e4_);
}
fn origin_geometricAntiQuotient_multiVector(self_: Origin, other: MultiVector) -> Origin {
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2) + pow(other.e423_, 2) + pow(other.e431_, 2) + pow(other.e412_, 2) - pow(other.e4_, 2) - pow(other.e41_, 2) - pow(other.e42_, 2) - pow(other.e43_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return Origin(anti_inverse.e1234_ * self_.e4_);
}
fn origin_geometricAntiQuotient_origin(self_: Origin, other: Origin) -> Origin {
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e4_, 2) * -1.0);
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return Origin(anti_inverse.e1234_ * self_.e4_);
}
fn origin_geometricAntiQuotient_plane(self_: Origin, other: Plane) -> Origin {
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e423_, 2) + pow(other.e431_, 2) + pow(other.e412_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return Origin(anti_inverse.e1234_ * self_.e4_);
}
fn origin_geometricAntiQuotient_point(self_: Origin, other: Point) -> Origin {
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e4_, 2) * -1.0);
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return Origin(anti_inverse.e1234_ * self_.e4_);
}
fn plane_geometricAntiQuotient_antiScalar(self_: Plane, other: AntiScalar) -> Plane {
    let self_groups = plane_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group0_
    ));
}
fn plane_geometricAntiQuotient_dualNum(self_: Plane, other: DualNum) -> Plane {
    let self_groups = plane_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group0_
    ));
}
fn plane_geometricAntiQuotient_flector(self_: Plane, other: Flector) -> Plane {
    let self_groups = plane_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e423_, 2) + pow(other.e431_, 2) + pow(other.e412_, 2) - pow(other.e4_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group0_
    ));
}
fn plane_geometricAntiQuotient_line(self_: Plane, other: Line) -> Plane {
    let self_groups = plane_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(-pow(other.e41_, 2) - pow(other.e42_, 2) - pow(other.e43_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group0_
    ));
}
fn plane_geometricAntiQuotient_motor(self_: Plane, other: Motor) -> Plane {
    let self_groups = plane_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2) - pow(other.e41_, 2) - pow(other.e42_, 2) - pow(other.e43_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group0_
    ));
}
fn plane_geometricAntiQuotient_multiVector(self_: Plane, other: MultiVector) -> Plane {
    let self_groups = plane_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2) + pow(other.e423_, 2) + pow(other.e431_, 2) + pow(other.e412_, 2) - pow(other.e4_, 2) - pow(other.e41_, 2) - pow(other.e42_, 2) - pow(other.e43_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group0_
    ));
}
fn plane_geometricAntiQuotient_origin(self_: Plane, other: Origin) -> Plane {
    let self_groups = plane_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e4_, 2) * -1.0);
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group0_
    ));
}
fn plane_geometricAntiQuotient_plane(self_: Plane, other: Plane) -> Plane {
    let self_groups = plane_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e423_, 2) + pow(other.e431_, 2) + pow(other.e412_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group0_
    ));
}
fn plane_geometricAntiQuotient_point(self_: Plane, other: Point) -> Plane {
    let self_groups = plane_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e4_, 2) * -1.0);
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group0_
    ));
}
fn point_geometricAntiQuotient_antiScalar(self_: Point, other: AntiScalar) -> Point {
    let self_groups = point_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group0_
    ));
}
fn point_geometricAntiQuotient_dualNum(self_: Point, other: DualNum) -> Point {
    let self_groups = point_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group0_
    ));
}
fn point_geometricAntiQuotient_flector(self_: Point, other: Flector) -> Point {
    let self_groups = point_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e423_, 2) + pow(other.e431_, 2) + pow(other.e412_, 2) - pow(other.e4_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group0_
    ));
}
fn point_geometricAntiQuotient_line(self_: Point, other: Line) -> Point {
    let self_groups = point_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(-pow(other.e41_, 2) - pow(other.e42_, 2) - pow(other.e43_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group0_
    ));
}
fn point_geometricAntiQuotient_motor(self_: Point, other: Motor) -> Point {
    let self_groups = point_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2) - pow(other.e41_, 2) - pow(other.e42_, 2) - pow(other.e43_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group0_
    ));
}
fn point_geometricAntiQuotient_multiVector(self_: Point, other: MultiVector) -> Point {
    let self_groups = point_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2) + pow(other.e423_, 2) + pow(other.e431_, 2) + pow(other.e412_, 2) - pow(other.e4_, 2) - pow(other.e41_, 2) - pow(other.e42_, 2) - pow(other.e43_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group0_
    ));
}
fn point_geometricAntiQuotient_origin(self_: Point, other: Origin) -> Point {
    let self_groups = point_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e4_, 2) * -1.0);
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group0_
    ));
}
fn point_geometricAntiQuotient_plane(self_: Point, other: Plane) -> Point {
    let self_groups = point_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e423_, 2) + pow(other.e431_, 2) + pow(other.e412_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group0_
    ));
}
fn point_geometricAntiQuotient_point(self_: Point, other: Point) -> Point {
    let self_groups = point_grouped(self_);
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e4_, 2) * -1.0);
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234_) * self_groups.group0_
    ));
}
fn scalar_geometricAntiQuotient_antiScalar(self_: Scalar, other: AntiScalar) -> Scalar {
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return Scalar(anti_inverse.e1234_ * self_.scalar);
}
fn scalar_geometricAntiQuotient_dualNum(self_: Scalar, other: DualNum) -> Scalar {
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return Scalar(anti_inverse.e1234_ * self_.scalar);
}
fn scalar_geometricAntiQuotient_flector(self_: Scalar, other: Flector) -> Scalar {
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e423_, 2) + pow(other.e431_, 2) + pow(other.e412_, 2) - pow(other.e4_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return Scalar(anti_inverse.e1234_ * self_.scalar);
}
fn scalar_geometricAntiQuotient_line(self_: Scalar, other: Line) -> Scalar {
    let anti_dot_product: AntiScalar = AntiScalar(-pow(other.e41_, 2) - pow(other.e42_, 2) - pow(other.e43_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return Scalar(anti_inverse.e1234_ * self_.scalar);
}
fn scalar_geometricAntiQuotient_motor(self_: Scalar, other: Motor) -> Scalar {
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2) - pow(other.e41_, 2) - pow(other.e42_, 2) - pow(other.e43_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return Scalar(anti_inverse.e1234_ * self_.scalar);
}
fn scalar_geometricAntiQuotient_multiVector(self_: Scalar, other: MultiVector) -> Scalar {
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e1234_, 2) + pow(other.e423_, 2) + pow(other.e431_, 2) + pow(other.e412_, 2) - pow(other.e4_, 2) - pow(other.e41_, 2) - pow(other.e42_, 2) - pow(other.e43_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return Scalar(anti_inverse.e1234_ * self_.scalar);
}
fn scalar_geometricAntiQuotient_origin(self_: Scalar, other: Origin) -> Scalar {
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e4_, 2) * -1.0);
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return Scalar(anti_inverse.e1234_ * self_.scalar);
}
fn scalar_geometricAntiQuotient_plane(self_: Scalar, other: Plane) -> Scalar {
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e423_, 2) + pow(other.e431_, 2) + pow(other.e412_, 2));
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return Scalar(anti_inverse.e1234_ * self_.scalar);
}
fn scalar_geometricAntiQuotient_point(self_: Scalar, other: Point) -> Scalar {
    let anti_dot_product: AntiScalar = AntiScalar(pow(other.e4_, 2) * -1.0);
    let anti_inverse: AntiScalar = AntiScalar(1.0/(anti_dot_product.e1234_));
    return Scalar(anti_inverse.e1234_ * self_.scalar);
}
fn antiScalar_geometricProduct_dualNum(self_: AntiScalar, other: DualNum) -> AntiScalar {
    return AntiScalar(self_.e1234_ * other.scalar);
}
fn antiScalar_geometricProduct_flector(self_: AntiScalar, other: Flector) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, self_.e1234_ * other.e321_) * vec4<f32>(0.0, 0.0, 0.0, -1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e1234_ * other.e1_, self_.e1234_ * other.e2_, self_.e1234_ * other.e3_, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0)
    ));
}
fn antiScalar_geometricProduct_horizon(self_: AntiScalar, other: Horizon) -> Origin {
    return Origin(self_.e1234_ * other.e321_ * -1.0);
}
fn antiScalar_geometricProduct_line(self_: AntiScalar, other: Line) -> Line {
    let other_groups = line_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group1_, 
        /* e23, e31, e12 */ vec4<f32>(0.0)
    ));
}
fn antiScalar_geometricProduct_motor(self_: AntiScalar, other: Motor) -> Motor {
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.e1234_) * other_groups.group1_, 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0)
    ));
}
fn antiScalar_geometricProduct_multiVector(self_: AntiScalar, other: MultiVector) -> MultiVector {
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(1.0, self_.e1234_ * other.scalar, 0.0, 0.0) * vec2<f32>(0.0, 1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, self_.e1234_ * other.e321_) * vec4<f32>(0.0, 0.0, 0.0, -1.0), 
        /* e41, e42, e43 */ (vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group3_, 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e1234_ * other.e1_, self_.e1234_ * other.e2_, self_.e1234_ * other.e3_, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0)
    ));
}
fn antiScalar_geometricProduct_plane(self_: AntiScalar, other: Plane) -> Origin {
    return Origin(self_.e1234_ * other.e321_ * -1.0);
}
fn antiScalar_geometricProduct_point(self_: AntiScalar, other: Point) -> Plane {
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e1234_ * other.e1_, self_.e1234_ * other.e2_, self_.e1234_ * other.e3_, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0)
    ));
}
fn antiScalar_geometricProduct_scalar(self_: AntiScalar, other: Scalar) -> AntiScalar {
    return AntiScalar(self_.e1234_ * other.scalar);
}
fn dualNum_geometricProduct_antiScalar(self_: DualNum, other: AntiScalar) -> AntiScalar {
    return AntiScalar(other.e1234_ * self_.scalar);
}
fn dualNum_geometricProduct_dualNum(self_: DualNum, other: DualNum) -> DualNum {
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar * self_.scalar, (other.scalar * self_.e1234_) + (other.e1234_ * self_.scalar), 0.0, 0.0)
    ));
}
fn dualNum_geometricProduct_flector(self_: DualNum, other: Flector) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.scalar * other.e1_, self_.scalar * other.e2_, self_.scalar * other.e3_, (self_.scalar * other.e4_) - (self_.e1234_ * other.e321_)), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.scalar * other.e423_) - (self_.e1234_ * other.e1_), (self_.scalar * other.e431_) - (self_.e1234_ * other.e2_), (self_.scalar * other.e412_) - (self_.e1234_ * other.e3_), self_.scalar * other.e321_)
    ));
}
fn dualNum_geometricProduct_horizon(self_: DualNum, other: Horizon) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, self_.e1234_ * other.e321_) * vec4<f32>(0.0, 0.0, 0.0, -1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0, 1.0, 1.0, self_.scalar * other.e321_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)
    ));
}
fn dualNum_geometricProduct_line(self_: DualNum, other: Line) -> Line {
    let other_groups = line_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ ((vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group0_) + ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group1_), 
        /* e23, e31, e12 */ (vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group1_
    ));
}
fn dualNum_geometricProduct_motor(self_: DualNum, other: Motor) -> Motor {
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ (vec4<f32>(self_.scalar) * other_groups.group0_) + (vec4<f32>(self_.e1234_) * other_groups.group1_), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.scalar) * other_groups.group1_
    ));
}
fn dualNum_geometricProduct_multiVector(self_: DualNum, other: MultiVector) -> MultiVector {
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar * other.scalar, (self_.scalar * other.e1234_) + (self_.e1234_ * other.scalar), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.scalar * other.e1_, self_.scalar * other.e2_, self_.scalar * other.e3_, (self_.scalar * other.e4_) - (self_.e1234_ * other.e321_)), 
        /* e41, e42, e43 */ ((vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group2_) + ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group3_), 
        /* e23, e31, e12 */ (vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.scalar * other.e423_) - (self_.e1234_ * other.e1_), (self_.scalar * other.e431_) - (self_.e1234_ * other.e2_), (self_.scalar * other.e412_) - (self_.e1234_ * other.e3_), self_.scalar * other.e321_)
    ));
}
fn dualNum_geometricProduct_origin(self_: DualNum, other: Origin) -> Origin {
    return Origin(self_.scalar * other.e4_);
}
fn dualNum_geometricProduct_plane(self_: DualNum, other: Plane) -> Flector {
    let other_groups = plane_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, self_.e1234_ * other.e321_) * vec4<f32>(0.0, 0.0, 0.0, -1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.scalar) * other_groups.group0_
    ));
}
fn dualNum_geometricProduct_point(self_: DualNum, other: Point) -> Flector {
    let other_groups = point_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.scalar) * other_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e1234_ * other.e1_, self_.e1234_ * other.e2_, self_.e1234_ * other.e3_, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0)
    ));
}
fn dualNum_geometricProduct_scalar(self_: DualNum, other: Scalar) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ (vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_
    ));
}
fn flector_geometricProduct_antiScalar(self_: Flector, other: AntiScalar) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, other.e1234_ * self_.e321_) * vec4<f32>(0.0, 0.0, 0.0, 1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e1234_ * self_.e1_, other.e1234_ * self_.e2_, other.e1234_ * self_.e3_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
}
fn flector_geometricProduct_dualNum(self_: Flector, other: DualNum) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.scalar * self_.e1_, other.scalar * self_.e2_, other.scalar * self_.e3_, (other.scalar * self_.e4_) + (other.e1234_ * self_.e321_)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.scalar * self_.e423_) + (other.e1234_ * self_.e1_), (other.scalar * self_.e431_) + (other.e1234_ * self_.e2_), (other.scalar * self_.e412_) + (other.e1234_ * self_.e3_), other.scalar * self_.e321_)
    ));
}
fn flector_geometricProduct_flector(self_: Flector, other: Flector) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other.e2_ * self_.e412_) + (other.e412_ * self_.e2_) + (other.e321_ * self_.e423_) - (other.e3_ * self_.e431_) - (other.e423_ * self_.e321_) - (other.e431_ * self_.e3_), (other.e3_ * self_.e423_) + (other.e423_ * self_.e3_) + (other.e321_ * self_.e431_) - (other.e1_ * self_.e412_) - (other.e431_ * self_.e321_) - (other.e412_ * self_.e1_), (other.e1_ * self_.e431_) + (other.e431_ * self_.e1_) + (other.e321_ * self_.e412_) - (other.e2_ * self_.e423_) - (other.e423_ * self_.e2_) - (other.e412_ * self_.e321_), (other.e423_ * self_.e1_) + (other.e431_ * self_.e2_) + (other.e412_ * self_.e3_) - (other.e1_ * self_.e423_) - (other.e2_ * self_.e431_) - (other.e3_ * self_.e412_)) + (vec4<f32>(self_.e4_) * vec4<f32>(other.e1_, other.e2_, other.e3_, other.e321_)) - (vec4<f32>(other.e4_) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, self_.e321_)), 
        /* e23, e31, e12, scalar */ vec4<f32>((other.e3_ * self_.e2_) - (other.e2_ * self_.e3_) - (other.e321_ * self_.e1_), (other.e1_ * self_.e3_) - (other.e3_ * self_.e1_) - (other.e321_ * self_.e2_), (other.e2_ * self_.e1_) - (other.e1_ * self_.e2_) - (other.e321_ * self_.e3_), (other.e1_ * self_.e1_) + (other.e2_ * self_.e2_) + (other.e3_ * self_.e3_)) - (vec4<f32>(self_.e321_) * vec4<f32>(other.e1_, other.e2_, other.e3_, other.e321_))
    ));
}
fn flector_geometricProduct_horizon(self_: Flector, other: Horizon) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e321_) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, self_.e4_), 
        /* e23, e31, e12, scalar */ vec4<f32>(other.e321_) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, self_.e321_) * vec4<f32>(-1.0)
    ));
}
fn flector_geometricProduct_line(self_: Flector, other: Line) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e3_ * other.e31_) + (self_.e321_ * other.e23_) - (self_.e2_ * other.e12_), (self_.e1_ * other.e12_) + (self_.e321_ * other.e31_) - (self_.e3_ * other.e23_), (self_.e2_ * other.e23_) + (self_.e321_ * other.e12_) - (self_.e1_ * other.e31_), -(self_.e1_ * other.e41_) - (self_.e2_ * other.e42_) - (self_.e3_ * other.e43_) - (self_.e423_ * other.e23_) - (self_.e431_ * other.e31_) - (self_.e412_ * other.e12_)), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e3_ * other.e42_) + (self_.e4_ * other.e23_) + (self_.e412_ * other.e31_) + (self_.e321_ * other.e41_) - (self_.e2_ * other.e43_) - (self_.e431_ * other.e12_), (self_.e1_ * other.e43_) + (self_.e4_ * other.e31_) + (self_.e423_ * other.e12_) + (self_.e321_ * other.e42_) - (self_.e3_ * other.e41_) - (self_.e412_ * other.e23_), (self_.e2_ * other.e41_) + (self_.e4_ * other.e12_) + (self_.e431_ * other.e23_) + (self_.e321_ * other.e43_) - (self_.e1_ * other.e42_) - (self_.e423_ * other.e31_), -(self_.e1_ * other.e23_) - (self_.e2_ * other.e31_) - (self_.e3_ * other.e12_))
    ));
}
fn flector_geometricProduct_motor(self_: Flector, other: Motor) -> Flector {
    let self_groups = flector_grouped(self_);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e3_ * other.e31_) - (self_.e2_ * other.e12_), (self_.e1_ * other.e12_) - (self_.e3_ * other.e23_), (self_.e2_ * other.e23_) - (self_.e1_ * other.e31_), -(self_.e1_ * other.e41_) - (self_.e2_ * other.e42_) - (self_.e3_ * other.e43_) - (self_.e423_ * other.e23_) - (self_.e431_ * other.e31_) - (self_.e412_ * other.e12_)) + (vec4<f32>(self_.e321_) * vec4<f32>(other.e23_, other.e31_, other.e12_, other.e1234_)) + (vec4<f32>(other.scalar) * self_groups.group0_), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e1_ * other.e1234_) + (self_.e3_ * other.e42_) + (self_.e4_ * other.e23_) + (self_.e412_ * other.e31_) + (self_.e321_ * other.e41_) - (self_.e2_ * other.e43_) - (self_.e431_ * other.e12_), (self_.e1_ * other.e43_) + (self_.e2_ * other.e1234_) + (self_.e4_ * other.e31_) + (self_.e423_ * other.e12_) + (self_.e321_ * other.e42_) - (self_.e3_ * other.e41_) - (self_.e412_ * other.e23_), (self_.e2_ * other.e41_) + (self_.e3_ * other.e1234_) + (self_.e4_ * other.e12_) + (self_.e431_ * other.e23_) + (self_.e321_ * other.e43_) - (self_.e1_ * other.e42_) - (self_.e423_ * other.e31_), -(self_.e1_ * other.e23_) - (self_.e2_ * other.e31_) - (self_.e3_ * other.e12_)) + (vec4<f32>(other.scalar) * self_groups.group1_)
    ));
}
fn flector_geometricProduct_multiVector(self_: Flector, other: MultiVector) -> MultiVector {
    let self_groups = flector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, (self_.e4_ * other.e321_) - (self_.e423_ * other.e1_) - (self_.e431_ * other.e2_) - (self_.e412_ * other.e3_), 0.0, 0.0) + ((vec4<f32>(self_.e1_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e1_, other.e423_, 0.0, 0.0)) + ((vec4<f32>(self_.e2_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e2_, other.e431_, 0.0, 0.0)) + ((vec4<f32>(self_.e3_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e3_, other.e412_, 0.0, 0.0)) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e321_, other.e4_, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e3_ * other.e31_) - (self_.e2_ * other.e12_), (self_.e1_ * other.e12_) - (self_.e3_ * other.e23_), (self_.e2_ * other.e23_) - (self_.e1_ * other.e31_), -(self_.e1_ * other.e41_) - (self_.e2_ * other.e42_) - (self_.e3_ * other.e43_) - (self_.e423_ * other.e23_) - (self_.e431_ * other.e31_) - (self_.e412_ * other.e12_)) + (vec4<f32>(self_.e321_) * vec4<f32>(other.e23_, other.e31_, other.e12_, other.e1234_)) + (vec4<f32>(other.scalar) * self_groups.group0_), 
        /* e41, e42, e43 */ vec4<f32>((self_.e2_ * other.e412_) + (self_.e412_ * other.e2_) - (self_.e3_ * other.e431_) - (self_.e431_ * other.e3_), (self_.e3_ * other.e423_) + (self_.e423_ * other.e3_) - (self_.e1_ * other.e412_) - (self_.e412_ * other.e1_), (self_.e1_ * other.e431_) + (self_.e431_ * other.e1_) - (self_.e2_ * other.e423_) - (self_.e423_ * other.e2_), 0.0) + ((vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0)) + ((vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0)) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0)) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((self_.e2_ * other.e3_) - (self_.e3_ * other.e2_), (self_.e3_ * other.e1_) - (self_.e1_ * other.e3_), (self_.e1_ * other.e2_) - (self_.e2_ * other.e1_), 0.0) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0)) - ((vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e1_ * other.e1234_) + (self_.e3_ * other.e42_) + (self_.e4_ * other.e23_) + (self_.e412_ * other.e31_) + (self_.e321_ * other.e41_) - (self_.e2_ * other.e43_) - (self_.e431_ * other.e12_), (self_.e1_ * other.e43_) + (self_.e2_ * other.e1234_) + (self_.e4_ * other.e31_) + (self_.e423_ * other.e12_) + (self_.e321_ * other.e42_) - (self_.e3_ * other.e41_) - (self_.e412_ * other.e23_), (self_.e2_ * other.e41_) + (self_.e3_ * other.e1234_) + (self_.e4_ * other.e12_) + (self_.e431_ * other.e23_) + (self_.e321_ * other.e43_) - (self_.e1_ * other.e42_) - (self_.e423_ * other.e31_), -(self_.e1_ * other.e23_) - (self_.e2_ * other.e31_) - (self_.e3_ * other.e12_)) + (vec4<f32>(other.scalar) * self_groups.group1_)
    ));
}
fn flector_geometricProduct_origin(self_: Flector, other: Origin) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e4_) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, self_.e321_) * vec4<f32>(-1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0)
    ));
}
fn flector_geometricProduct_plane(self_: Flector, other: Plane) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((self_.e2_ * other.e412_) - (self_.e3_ * other.e431_) - (self_.e321_ * other.e423_), (self_.e3_ * other.e423_) - (self_.e1_ * other.e412_) - (self_.e321_ * other.e431_), (self_.e1_ * other.e431_) - (self_.e2_ * other.e423_) - (self_.e321_ * other.e412_), (self_.e1_ * other.e423_) + (self_.e2_ * other.e431_) + (self_.e3_ * other.e412_)) + (vec4<f32>(other.e321_) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, self_.e4_)), 
        /* e23, e31, e12, scalar */ vec4<f32>(other.e321_) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, self_.e321_) * vec4<f32>(-1.0)
    ));
}
fn flector_geometricProduct_point(self_: Flector, other: Point) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((self_.e4_ * other.e1_) + (self_.e412_ * other.e2_) - (self_.e431_ * other.e3_), (self_.e4_ * other.e2_) + (self_.e423_ * other.e3_) - (self_.e412_ * other.e1_), (self_.e4_ * other.e3_) + (self_.e431_ * other.e1_) - (self_.e423_ * other.e2_), -(self_.e423_ * other.e1_) - (self_.e431_ * other.e2_) - (self_.e412_ * other.e3_)) - (vec4<f32>(other.e4_) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, self_.e321_)), 
        /* e23, e31, e12, scalar */ vec4<f32>((self_.e2_ * other.e3_) - (self_.e3_ * other.e2_) - (self_.e321_ * other.e1_), (self_.e3_ * other.e1_) - (self_.e1_ * other.e3_) - (self_.e321_ * other.e2_), (self_.e1_ * other.e2_) - (self_.e2_ * other.e1_) - (self_.e321_ * other.e3_), (self_.e1_ * other.e1_) + (self_.e2_ * other.e2_) + (self_.e3_ * other.e3_))
    ));
}
fn flector_geometricProduct_scalar(self_: Flector, other: Scalar) -> Flector {
    let self_groups = flector_grouped(self_);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.scalar) * self_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other.scalar) * self_groups.group1_
    ));
}
fn horizon_geometricProduct_antiScalar(self_: Horizon, other: AntiScalar) -> Origin {
    return Origin(other.e1234_ * self_.e321_);
}
fn horizon_geometricProduct_dualNum(self_: Horizon, other: DualNum) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, other.e1234_ * self_.e321_) * vec4<f32>(0.0, 0.0, 0.0, 1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0, 1.0, 1.0, other.scalar * self_.e321_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)
    ));
}
fn horizon_geometricProduct_flector(self_: Horizon, other: Flector) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.e321_) * vec4<f32>(other.e423_, other.e431_, other.e412_, other.e4_) * vec4<f32>(-1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.e321_) * vec4<f32>(other.e1_, other.e2_, other.e3_, other.e321_) * vec4<f32>(-1.0)
    ));
}
fn horizon_geometricProduct_horizon(self_: Horizon, other: Horizon) -> Scalar {
    return Scalar(other.e321_ * self_.e321_ * -1.0);
}
fn horizon_geometricProduct_line(self_: Horizon, other: Line) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e321_ * other.e23_, self_.e321_ * other.e31_, self_.e321_ * other.e12_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e321_ * other.e41_, self_.e321_ * other.e42_, self_.e321_ * other.e43_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
}
fn horizon_geometricProduct_motor(self_: Horizon, other: Motor) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e321_) * vec4<f32>(other.e23_, other.e31_, other.e12_, other.e1234_), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e321_) * vec4<f32>(other.e41_, other.e42_, other.e43_, other.scalar)
    ));
}
fn horizon_geometricProduct_multiVector(self_: Horizon, other: MultiVector) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ (vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e321_, other.e4_, 0.0, 0.0) * vec2<f32>(-1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e321_) * vec4<f32>(other.e23_, other.e31_, other.e12_, other.e1234_), 
        /* e41, e42, e43 */ (vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ (vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0) * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e321_) * vec4<f32>(other.e41_, other.e42_, other.e43_, other.scalar)
    ));
}
fn horizon_geometricProduct_origin(self_: Horizon, other: Origin) -> AntiScalar {
    return AntiScalar(self_.e321_ * other.e4_ * -1.0);
}
fn horizon_geometricProduct_plane(self_: Horizon, other: Plane) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.e321_ * other.e423_, self_.e321_ * other.e431_, self_.e321_ * other.e412_, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(1.0, 1.0, 1.0, self_.e321_ * other.e321_) * vec4<f32>(0.0, 0.0, 0.0, -1.0)
    ));
}
fn horizon_geometricProduct_point(self_: Horizon, other: Point) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(1.0, 1.0, 1.0, self_.e321_ * other.e4_) * vec4<f32>(0.0, 0.0, 0.0, -1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.e321_ * other.e1_, self_.e321_ * other.e2_, self_.e321_ * other.e3_, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0)
    ));
}
fn horizon_geometricProduct_scalar(self_: Horizon, other: Scalar) -> Horizon {
    return Horizon(self_.e321_ * other.scalar);
}
fn line_geometricProduct_antiScalar(self_: Line, other: AntiScalar) -> Line {
    let self_groups = line_grouped(self_);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_, 
        /* e23, e31, e12 */ vec4<f32>(0.0)
    ));
}
fn line_geometricProduct_dualNum(self_: Line, other: DualNum) -> Line {
    let self_groups = line_grouped(self_);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_) + ((vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_), 
        /* e23, e31, e12 */ (vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_
    ));
}
fn line_geometricProduct_flector(self_: Line, other: Flector) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other.e2_ * self_.e12_) + (other.e321_ * self_.e23_) - (other.e3_ * self_.e31_), (other.e3_ * self_.e23_) + (other.e321_ * self_.e31_) - (other.e1_ * self_.e12_), (other.e1_ * self_.e31_) + (other.e321_ * self_.e12_) - (other.e2_ * self_.e23_), (other.e1_ * self_.e41_) + (other.e2_ * self_.e42_) + (other.e3_ * self_.e43_) - (other.e423_ * self_.e23_) - (other.e431_ * self_.e31_) - (other.e412_ * self_.e12_)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e3_ * self_.e42_) + (other.e4_ * self_.e23_) + (other.e431_ * self_.e12_) - (other.e2_ * self_.e43_) - (other.e412_ * self_.e31_) - (other.e321_ * self_.e41_), (other.e1_ * self_.e43_) + (other.e4_ * self_.e31_) + (other.e412_ * self_.e23_) - (other.e3_ * self_.e41_) - (other.e423_ * self_.e12_) - (other.e321_ * self_.e42_), (other.e2_ * self_.e41_) + (other.e4_ * self_.e12_) + (other.e423_ * self_.e31_) - (other.e1_ * self_.e42_) - (other.e431_ * self_.e23_) - (other.e321_ * self_.e43_), -(other.e1_ * self_.e23_) - (other.e2_ * self_.e31_) - (other.e3_ * self_.e12_))
    ));
}
fn line_geometricProduct_horizon(self_: Line, other: Horizon) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e321_ * self_.e23_, other.e321_ * self_.e31_, other.e321_ * self_.e12_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e321_ * self_.e41_, other.e321_ * self_.e42_, other.e321_ * self_.e43_, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0)
    ));
}
fn line_geometricProduct_line(self_: Line, other: Line) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other.e42_ * self_.e12_) + (other.e31_ * self_.e43_) - (other.e43_ * self_.e31_) - (other.e12_ * self_.e42_), (other.e43_ * self_.e23_) + (other.e12_ * self_.e41_) - (other.e41_ * self_.e12_) - (other.e23_ * self_.e43_), (other.e41_ * self_.e31_) + (other.e23_ * self_.e42_) - (other.e42_ * self_.e23_) - (other.e31_ * self_.e41_), -(other.e41_ * self_.e23_) - (other.e42_ * self_.e31_) - (other.e43_ * self_.e12_) - (other.e23_ * self_.e41_) - (other.e31_ * self_.e42_) - (other.e12_ * self_.e43_)), 
        /* e23, e31, e12, scalar */ vec4<f32>((other.e31_ * self_.e12_) - (other.e12_ * self_.e31_), (other.e12_ * self_.e23_) - (other.e23_ * self_.e12_), (other.e23_ * self_.e31_) - (other.e31_ * self_.e23_), -(other.e23_ * self_.e23_) - (other.e31_ * self_.e31_) - (other.e12_ * self_.e12_))
    ));
}
fn line_geometricProduct_motor(self_: Line, other: Motor) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((self_.e41_ * other.scalar) + (self_.e43_ * other.e31_) + (self_.e23_ * other.e1234_) + (self_.e12_ * other.e42_) - (self_.e42_ * other.e12_) - (self_.e31_ * other.e43_), (self_.e41_ * other.e12_) + (self_.e42_ * other.scalar) + (self_.e23_ * other.e43_) + (self_.e31_ * other.e1234_) - (self_.e43_ * other.e23_) - (self_.e12_ * other.e41_), (self_.e42_ * other.e23_) + (self_.e43_ * other.scalar) + (self_.e31_ * other.e41_) + (self_.e12_ * other.e1234_) - (self_.e41_ * other.e31_) - (self_.e23_ * other.e42_), -(self_.e41_ * other.e23_) - (self_.e42_ * other.e31_) - (self_.e43_ * other.e12_) - (self_.e23_ * other.e41_) - (self_.e31_ * other.e42_) - (self_.e12_ * other.e43_)), 
        /* e23, e31, e12, scalar */ vec4<f32>((self_.e23_ * other.scalar) + (self_.e12_ * other.e31_) - (self_.e31_ * other.e12_), (self_.e23_ * other.e12_) + (self_.e31_ * other.scalar) - (self_.e12_ * other.e23_), (self_.e31_ * other.e23_) + (self_.e12_ * other.scalar) - (self_.e23_ * other.e31_), -(self_.e23_ * other.e23_) - (self_.e31_ * other.e31_) - (self_.e12_ * other.e12_))
    ));
}
fn line_geometricProduct_multiVector(self_: Line, other: MultiVector) -> MultiVector {
    let self_groups = line_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, -(self_.e23_ * other.e41_) - (self_.e31_ * other.e42_) - (self_.e12_ * other.e43_), 0.0, 0.0) - ((vec4<f32>(other.e23_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e23_, self_.e41_, 0.0, 0.0)) - ((vec4<f32>(other.e31_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e31_, self_.e42_, 0.0, 0.0)) - ((vec4<f32>(other.e12_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e12_, self_.e43_, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e23_ * other.e321_) + (self_.e12_ * other.e2_) - (self_.e31_ * other.e3_), (self_.e23_ * other.e3_) + (self_.e31_ * other.e321_) - (self_.e12_ * other.e1_), (self_.e31_ * other.e1_) + (self_.e12_ * other.e321_) - (self_.e23_ * other.e2_), (self_.e41_ * other.e1_) + (self_.e42_ * other.e2_) + (self_.e43_ * other.e3_) - (self_.e23_ * other.e423_) - (self_.e31_ * other.e431_) - (self_.e12_ * other.e412_)), 
        /* e41, e42, e43 */ vec4<f32>((self_.e43_ * other.e31_) + (self_.e12_ * other.e42_) - (self_.e42_ * other.e12_) - (self_.e31_ * other.e43_), (self_.e41_ * other.e12_) + (self_.e23_ * other.e43_) - (self_.e43_ * other.e23_) - (self_.e12_ * other.e41_), (self_.e42_ * other.e23_) + (self_.e31_ * other.e41_) - (self_.e41_ * other.e31_) - (self_.e23_ * other.e42_), 0.0) + ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_) + ((vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_), 
        /* e23, e31, e12 */ vec4<f32>((self_.e12_ * other.e31_) - (self_.e31_ * other.e12_), (self_.e23_ * other.e12_) - (self_.e12_ * other.e23_), (self_.e31_ * other.e23_) - (self_.e23_ * other.e31_), 0.0) + ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e42_ * other.e3_) + (self_.e23_ * other.e4_) + (self_.e12_ * other.e431_) - (self_.e41_ * other.e321_) - (self_.e43_ * other.e2_) - (self_.e31_ * other.e412_), (self_.e43_ * other.e1_) + (self_.e23_ * other.e412_) + (self_.e31_ * other.e4_) - (self_.e41_ * other.e3_) - (self_.e42_ * other.e321_) - (self_.e12_ * other.e423_), (self_.e41_ * other.e2_) + (self_.e31_ * other.e423_) + (self_.e12_ * other.e4_) - (self_.e42_ * other.e1_) - (self_.e43_ * other.e321_) - (self_.e23_ * other.e431_), -(self_.e23_ * other.e1_) - (self_.e31_ * other.e2_) - (self_.e12_ * other.e3_))
    ));
}
fn line_geometricProduct_origin(self_: Line, other: Origin) -> Plane {
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e23_ * other.e4_, self_.e31_ * other.e4_, self_.e12_ * other.e4_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
}
fn line_geometricProduct_plane(self_: Line, other: Plane) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e23_ * other.e321_, self_.e31_ * other.e321_, self_.e12_ * other.e321_, -(self_.e23_ * other.e423_) - (self_.e31_ * other.e431_) - (self_.e12_ * other.e412_)), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e12_ * other.e431_) - (self_.e41_ * other.e321_) - (self_.e31_ * other.e412_), (self_.e23_ * other.e412_) - (self_.e42_ * other.e321_) - (self_.e12_ * other.e423_), (self_.e31_ * other.e423_) - (self_.e43_ * other.e321_) - (self_.e23_ * other.e431_), 0.0)
    ));
}
fn line_geometricProduct_point(self_: Line, other: Point) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e12_ * other.e2_) - (self_.e31_ * other.e3_), (self_.e23_ * other.e3_) - (self_.e12_ * other.e1_), (self_.e31_ * other.e1_) - (self_.e23_ * other.e2_), (self_.e41_ * other.e1_) + (self_.e42_ * other.e2_) + (self_.e43_ * other.e3_)), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e42_ * other.e3_) + (self_.e23_ * other.e4_) - (self_.e43_ * other.e2_), (self_.e43_ * other.e1_) + (self_.e31_ * other.e4_) - (self_.e41_ * other.e3_), (self_.e41_ * other.e2_) + (self_.e12_ * other.e4_) - (self_.e42_ * other.e1_), -(self_.e23_ * other.e1_) - (self_.e31_ * other.e2_) - (self_.e12_ * other.e3_))
    ));
}
fn line_geometricProduct_scalar(self_: Line, other: Scalar) -> Line {
    let self_groups = line_grouped(self_);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_
    ));
}
fn motor_geometricProduct_antiScalar(self_: Motor, other: AntiScalar) -> Motor {
    let self_groups = motor_grouped(self_);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e1234_) * self_groups.group1_, 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0)
    ));
}
fn motor_geometricProduct_dualNum(self_: Motor, other: DualNum) -> Motor {
    let self_groups = motor_grouped(self_);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ (vec4<f32>(other.scalar) * self_groups.group0_) + (vec4<f32>(other.e1234_) * self_groups.group1_), 
        /* e23, e31, e12, scalar */ vec4<f32>(other.scalar) * self_groups.group1_
    ));
}
fn motor_geometricProduct_flector(self_: Motor, other: Flector) -> Flector {
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other.e2_ * self_.e12_) + (other.e321_ * self_.e23_) - (other.e3_ * self_.e31_), (other.e3_ * self_.e23_) + (other.e321_ * self_.e31_) - (other.e1_ * self_.e12_), (other.e1_ * self_.e31_) + (other.e321_ * self_.e12_) - (other.e2_ * self_.e23_), (other.e1_ * self_.e41_) + (other.e2_ * self_.e42_) + (other.e3_ * self_.e43_) - (other.e423_ * self_.e23_) - (other.e431_ * self_.e31_) - (other.e412_ * self_.e12_) - (other.e321_ * self_.e1234_)) + (vec4<f32>(self_.scalar) * other_groups.group0_), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e3_ * self_.e42_) + (other.e4_ * self_.e23_) + (other.e431_ * self_.e12_) - (other.e1_ * self_.e1234_) - (other.e2_ * self_.e43_) - (other.e412_ * self_.e31_) - (other.e321_ * self_.e41_), (other.e1_ * self_.e43_) + (other.e4_ * self_.e31_) + (other.e412_ * self_.e23_) - (other.e2_ * self_.e1234_) - (other.e3_ * self_.e41_) - (other.e423_ * self_.e12_) - (other.e321_ * self_.e42_), (other.e2_ * self_.e41_) + (other.e4_ * self_.e12_) + (other.e423_ * self_.e31_) - (other.e1_ * self_.e42_) - (other.e3_ * self_.e1234_) - (other.e431_ * self_.e23_) - (other.e321_ * self_.e43_), -(other.e1_ * self_.e23_) - (other.e2_ * self_.e31_) - (other.e3_ * self_.e12_)) + (vec4<f32>(self_.scalar) * other_groups.group1_)
    ));
}
fn motor_geometricProduct_horizon(self_: Motor, other: Horizon) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e321_) * vec4<f32>(self_.e23_, self_.e31_, self_.e12_, self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, -1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e321_) * vec4<f32>(self_.e41_, self_.e42_, self_.e43_, self_.scalar) * vec4<f32>(-1.0, -1.0, -1.0, 1.0)
    ));
}
fn motor_geometricProduct_line(self_: Motor, other: Line) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other.e41_ * self_.scalar) + (other.e42_ * self_.e12_) + (other.e23_ * self_.e1234_) + (other.e31_ * self_.e43_) - (other.e43_ * self_.e31_) - (other.e12_ * self_.e42_), (other.e42_ * self_.scalar) + (other.e43_ * self_.e23_) + (other.e31_ * self_.e1234_) + (other.e12_ * self_.e41_) - (other.e41_ * self_.e12_) - (other.e23_ * self_.e43_), (other.e41_ * self_.e31_) + (other.e43_ * self_.scalar) + (other.e23_ * self_.e42_) + (other.e12_ * self_.e1234_) - (other.e42_ * self_.e23_) - (other.e31_ * self_.e41_), -(other.e41_ * self_.e23_) - (other.e42_ * self_.e31_) - (other.e43_ * self_.e12_) - (other.e23_ * self_.e41_) - (other.e31_ * self_.e42_) - (other.e12_ * self_.e43_)), 
        /* e23, e31, e12, scalar */ vec4<f32>((other.e23_ * self_.scalar) + (other.e31_ * self_.e12_) - (other.e12_ * self_.e31_), (other.e31_ * self_.scalar) + (other.e12_ * self_.e23_) - (other.e23_ * self_.e12_), (other.e23_ * self_.e31_) + (other.e12_ * self_.scalar) - (other.e31_ * self_.e23_), -(other.e23_ * self_.e23_) - (other.e31_ * self_.e31_) - (other.e12_ * self_.e12_))
    ));
}
fn motor_geometricProduct_motor(self_: Motor, other: Motor) -> Motor {
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other.e42_ * self_.e12_) + (other.e1234_ * self_.e23_) + (other.e31_ * self_.e43_) + (other.scalar * self_.e41_) - (other.e43_ * self_.e31_) - (other.e12_ * self_.e42_), (other.e43_ * self_.e23_) + (other.e1234_ * self_.e31_) + (other.e12_ * self_.e41_) + (other.scalar * self_.e42_) - (other.e41_ * self_.e12_) - (other.e23_ * self_.e43_), (other.e41_ * self_.e31_) + (other.e1234_ * self_.e12_) + (other.e23_ * self_.e42_) + (other.scalar * self_.e43_) - (other.e42_ * self_.e23_) - (other.e31_ * self_.e41_), -(other.e41_ * self_.e23_) - (other.e42_ * self_.e31_) - (other.e43_ * self_.e12_) - (other.e23_ * self_.e41_) - (other.e31_ * self_.e42_) - (other.e12_ * self_.e43_)) + (vec4<f32>(self_.e1234_) * other_groups.group1_) + (vec4<f32>(self_.scalar) * other_groups.group0_), 
        /* e23, e31, e12, scalar */ vec4<f32>((other.e31_ * self_.e12_) + (other.scalar * self_.e23_) - (other.e12_ * self_.e31_), (other.e12_ * self_.e23_) + (other.scalar * self_.e31_) - (other.e23_ * self_.e12_), (other.e23_ * self_.e31_) + (other.scalar * self_.e12_) - (other.e31_ * self_.e23_), -(other.e23_ * self_.e23_) - (other.e31_ * self_.e31_) - (other.e12_ * self_.e12_)) + (vec4<f32>(self_.scalar) * other_groups.group1_)
    ));
}
fn motor_geometricProduct_multiVector(self_: Motor, other: MultiVector) -> MultiVector {
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, (self_.scalar * other.e1234_) - (self_.e23_ * other.e41_) - (self_.e31_ * other.e42_) - (self_.e12_ * other.e43_), 0.0, 0.0) + ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.scalar, self_.e1234_, 0.0, 0.0)) - ((vec4<f32>(other.e23_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e23_, self_.e41_, 0.0, 0.0)) - ((vec4<f32>(other.e31_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e31_, self_.e42_, 0.0, 0.0)) - ((vec4<f32>(other.e12_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e12_, self_.e43_, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e23_ * other.e321_) + (self_.e12_ * other.e2_) - (self_.e31_ * other.e3_), (self_.e23_ * other.e3_) + (self_.e31_ * other.e321_) - (self_.e12_ * other.e1_), (self_.e31_ * other.e1_) + (self_.e12_ * other.e321_) - (self_.e23_ * other.e2_), (self_.e41_ * other.e1_) + (self_.e42_ * other.e2_) + (self_.e43_ * other.e3_) - (self_.e1234_ * other.e321_) - (self_.e23_ * other.e423_) - (self_.e31_ * other.e431_) - (self_.e12_ * other.e412_)) + (vec4<f32>(self_.scalar) * other_groups.group1_), 
        /* e41, e42, e43 */ vec4<f32>((self_.e43_ * other.e31_) + (self_.e12_ * other.e42_) - (self_.e42_ * other.e12_) - (self_.e31_ * other.e43_), (self_.e41_ * other.e12_) + (self_.e23_ * other.e43_) - (self_.e43_ * other.e23_) - (self_.e12_ * other.e41_), (self_.e42_ * other.e23_) + (self_.e31_ * other.e41_) - (self_.e41_ * other.e31_) - (self_.e23_ * other.e42_), 0.0) + ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group3_) + ((vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group2_) + ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e41_, self_.e42_, self_.e43_, 0.0)) + ((vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e23_, self_.e31_, self_.e12_, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((self_.e12_ * other.e31_) - (self_.e31_ * other.e12_), (self_.e23_ * other.e12_) - (self_.e12_ * other.e23_), (self_.e31_ * other.e23_) - (self_.e23_ * other.e31_), 0.0) + ((vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group3_) + ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e23_, self_.e31_, self_.e12_, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e42_ * other.e3_) + (self_.e23_ * other.e4_) + (self_.e12_ * other.e431_) - (self_.e41_ * other.e321_) - (self_.e43_ * other.e2_) - (self_.e1234_ * other.e1_) - (self_.e31_ * other.e412_), (self_.e43_ * other.e1_) + (self_.e23_ * other.e412_) + (self_.e31_ * other.e4_) - (self_.e41_ * other.e3_) - (self_.e42_ * other.e321_) - (self_.e1234_ * other.e2_) - (self_.e12_ * other.e423_), (self_.e41_ * other.e2_) + (self_.e31_ * other.e423_) + (self_.e12_ * other.e4_) - (self_.e42_ * other.e1_) - (self_.e43_ * other.e321_) - (self_.e1234_ * other.e3_) - (self_.e23_ * other.e431_), -(self_.e23_ * other.e1_) - (self_.e31_ * other.e2_) - (self_.e12_ * other.e3_)) + (vec4<f32>(self_.scalar) * other_groups.group4_)
    ));
}
fn motor_geometricProduct_origin(self_: Motor, other: Origin) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, self_.scalar * other.e4_) * vec4<f32>(0.0, 0.0, 0.0, 1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e23_ * other.e4_, self_.e31_ * other.e4_, self_.e12_ * other.e4_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
}
fn motor_geometricProduct_plane(self_: Motor, other: Plane) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e23_ * other.e321_, self_.e31_ * other.e321_, self_.e12_ * other.e321_, -(self_.e1234_ * other.e321_) - (self_.e23_ * other.e423_) - (self_.e31_ * other.e431_) - (self_.e12_ * other.e412_)), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e12_ * other.e431_) + (self_.scalar * other.e423_) - (self_.e41_ * other.e321_) - (self_.e31_ * other.e412_), (self_.e23_ * other.e412_) + (self_.scalar * other.e431_) - (self_.e42_ * other.e321_) - (self_.e12_ * other.e423_), (self_.e31_ * other.e423_) + (self_.scalar * other.e412_) - (self_.e43_ * other.e321_) - (self_.e23_ * other.e431_), self_.scalar * other.e321_)
    ));
}
fn motor_geometricProduct_point(self_: Motor, other: Point) -> Flector {
    let other_groups = point_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e12_ * other.e2_) - (self_.e31_ * other.e3_), (self_.e23_ * other.e3_) - (self_.e12_ * other.e1_), (self_.e31_ * other.e1_) - (self_.e23_ * other.e2_), (self_.e41_ * other.e1_) + (self_.e42_ * other.e2_) + (self_.e43_ * other.e3_)) + (vec4<f32>(self_.scalar) * other_groups.group0_), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e42_ * other.e3_) + (self_.e23_ * other.e4_) - (self_.e43_ * other.e2_) - (self_.e1234_ * other.e1_), (self_.e43_ * other.e1_) + (self_.e31_ * other.e4_) - (self_.e41_ * other.e3_) - (self_.e1234_ * other.e2_), (self_.e41_ * other.e2_) + (self_.e12_ * other.e4_) - (self_.e42_ * other.e1_) - (self_.e1234_ * other.e3_), -(self_.e23_ * other.e1_) - (self_.e31_ * other.e2_) - (self_.e12_ * other.e3_))
    ));
}
fn motor_geometricProduct_scalar(self_: Motor, other: Scalar) -> Motor {
    let self_groups = motor_grouped(self_);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.scalar) * self_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(other.scalar) * self_groups.group1_
    ));
}
fn multiVector_geometricProduct_antiScalar(self_: MultiVector, other: AntiScalar) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(1.0, other.e1234_ * self_.scalar, 0.0, 0.0) * vec2<f32>(0.0, 1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, other.e1234_ * self_.e321_) * vec4<f32>(0.0, 0.0, 0.0, 1.0), 
        /* e41, e42, e43 */ (vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_, 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e1234_ * self_.e1_, other.e1234_ * self_.e2_, other.e1234_ * self_.e3_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
}
fn multiVector_geometricProduct_dualNum(self_: MultiVector, other: DualNum) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar * self_.scalar, (other.scalar * self_.e1234_) + (other.e1234_ * self_.scalar), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(other.scalar * self_.e1_, other.scalar * self_.e2_, other.scalar * self_.e3_, (other.scalar * self_.e4_) + (other.e1234_ * self_.e321_)), 
        /* e41, e42, e43 */ ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_) + ((vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_), 
        /* e23, e31, e12 */ (vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>((other.scalar * self_.e423_) + (other.e1234_ * self_.e1_), (other.scalar * self_.e431_) + (other.e1234_ * self_.e2_), (other.scalar * self_.e412_) + (other.e1234_ * self_.e3_), other.scalar * self_.e321_)
    ));
}
fn multiVector_geometricProduct_flector(self_: MultiVector, other: Flector) -> MultiVector {
    let other_groups = flector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, (other.e321_ * self_.e4_) - (other.e1_ * self_.e423_) - (other.e2_ * self_.e431_) - (other.e3_ * self_.e412_), 0.0, 0.0) + ((vec4<f32>(self_.e1_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e1_, other.e423_, 0.0, 0.0)) + ((vec4<f32>(self_.e2_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e2_, other.e431_, 0.0, 0.0)) + ((vec4<f32>(self_.e3_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e3_, other.e412_, 0.0, 0.0)) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e321_, other.e4_, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((other.e2_ * self_.e12_) + (other.e321_ * self_.e23_) - (other.e3_ * self_.e31_), (other.e3_ * self_.e23_) + (other.e321_ * self_.e31_) - (other.e1_ * self_.e12_), (other.e1_ * self_.e31_) + (other.e321_ * self_.e12_) - (other.e2_ * self_.e23_), (other.e1_ * self_.e41_) + (other.e2_ * self_.e42_) + (other.e3_ * self_.e43_) - (other.e423_ * self_.e23_) - (other.e431_ * self_.e31_) - (other.e412_ * self_.e12_) - (other.e321_ * self_.e1234_)) + (vec4<f32>(self_.scalar) * other_groups.group0_), 
        /* e41, e42, e43 */ vec4<f32>((other.e2_ * self_.e412_) + (other.e412_ * self_.e2_) - (other.e3_ * self_.e431_) - (other.e431_ * self_.e3_), (other.e3_ * self_.e423_) + (other.e423_ * self_.e3_) - (other.e1_ * self_.e412_) - (other.e412_ * self_.e1_), (other.e1_ * self_.e431_) + (other.e431_ * self_.e1_) - (other.e2_ * self_.e423_) - (other.e423_ * self_.e2_), 0.0) + ((vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0)) + ((vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0)) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0)) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((other.e3_ * self_.e2_) - (other.e2_ * self_.e3_), (other.e1_ * self_.e3_) - (other.e3_ * self_.e1_), (other.e2_ * self_.e1_) - (other.e1_ * self_.e2_), 0.0) - ((vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0)) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e3_ * self_.e42_) + (other.e4_ * self_.e23_) + (other.e431_ * self_.e12_) - (other.e1_ * self_.e1234_) - (other.e2_ * self_.e43_) - (other.e412_ * self_.e31_) - (other.e321_ * self_.e41_), (other.e1_ * self_.e43_) + (other.e4_ * self_.e31_) + (other.e412_ * self_.e23_) - (other.e2_ * self_.e1234_) - (other.e3_ * self_.e41_) - (other.e423_ * self_.e12_) - (other.e321_ * self_.e42_), (other.e2_ * self_.e41_) + (other.e4_ * self_.e12_) + (other.e423_ * self_.e31_) - (other.e1_ * self_.e42_) - (other.e3_ * self_.e1234_) - (other.e431_ * self_.e23_) - (other.e321_ * self_.e43_), -(other.e1_ * self_.e23_) - (other.e2_ * self_.e31_) - (other.e3_ * self_.e12_)) + (vec4<f32>(self_.scalar) * other_groups.group1_)
    ));
}
fn multiVector_geometricProduct_horizon(self_: MultiVector, other: Horizon) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ (vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e321_, self_.e4_, 0.0, 0.0) * vec2<f32>(-1.0, 1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(other.e321_) * vec4<f32>(self_.e23_, self_.e31_, self_.e12_, self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, -1.0), 
        /* e41, e42, e43 */ (vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0), 
        /* e23, e31, e12 */ (vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0) * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e321_) * vec4<f32>(self_.e41_, self_.e42_, self_.e43_, self_.scalar) * vec4<f32>(-1.0, -1.0, -1.0, 1.0)
    ));
}
fn multiVector_geometricProduct_line(self_: MultiVector, other: Line) -> MultiVector {
    let other_groups = line_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, -(other.e23_ * self_.e41_) - (other.e31_ * self_.e42_) - (other.e12_ * self_.e43_), 0.0, 0.0) - ((vec4<f32>(self_.e23_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e23_, other.e41_, 0.0, 0.0)) - ((vec4<f32>(self_.e31_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e31_, other.e42_, 0.0, 0.0)) - ((vec4<f32>(self_.e12_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e12_, other.e43_, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((other.e23_ * self_.e321_) + (other.e31_ * self_.e3_) - (other.e12_ * self_.e2_), (other.e31_ * self_.e321_) + (other.e12_ * self_.e1_) - (other.e23_ * self_.e3_), (other.e23_ * self_.e2_) + (other.e12_ * self_.e321_) - (other.e31_ * self_.e1_), -(other.e41_ * self_.e1_) - (other.e42_ * self_.e2_) - (other.e43_ * self_.e3_) - (other.e23_ * self_.e423_) - (other.e31_ * self_.e431_) - (other.e12_ * self_.e412_)), 
        /* e41, e42, e43 */ vec4<f32>((other.e42_ * self_.e12_) + (other.e31_ * self_.e43_) - (other.e43_ * self_.e31_) - (other.e12_ * self_.e42_), (other.e43_ * self_.e23_) + (other.e12_ * self_.e41_) - (other.e41_ * self_.e12_) - (other.e23_ * self_.e43_), (other.e41_ * self_.e31_) + (other.e23_ * self_.e42_) - (other.e42_ * self_.e23_) - (other.e31_ * self_.e41_), 0.0) + ((vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group0_) + ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group1_), 
        /* e23, e31, e12 */ vec4<f32>((other.e31_ * self_.e12_) - (other.e12_ * self_.e31_), (other.e12_ * self_.e23_) - (other.e23_ * self_.e12_), (other.e23_ * self_.e31_) - (other.e31_ * self_.e23_), 0.0) + ((vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group1_), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e41_ * self_.e321_) + (other.e42_ * self_.e3_) + (other.e23_ * self_.e4_) + (other.e31_ * self_.e412_) - (other.e43_ * self_.e2_) - (other.e12_ * self_.e431_), (other.e42_ * self_.e321_) + (other.e43_ * self_.e1_) + (other.e31_ * self_.e4_) + (other.e12_ * self_.e423_) - (other.e41_ * self_.e3_) - (other.e23_ * self_.e412_), (other.e41_ * self_.e2_) + (other.e43_ * self_.e321_) + (other.e23_ * self_.e431_) + (other.e12_ * self_.e4_) - (other.e42_ * self_.e1_) - (other.e31_ * self_.e423_), -(other.e23_ * self_.e1_) - (other.e31_ * self_.e2_) - (other.e12_ * self_.e3_))
    ));
}
fn multiVector_geometricProduct_motor(self_: MultiVector, other: Motor) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, (other.scalar * self_.e1234_) - (other.e23_ * self_.e41_) - (other.e31_ * self_.e42_) - (other.e12_ * self_.e43_), 0.0, 0.0) + ((vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.scalar, other.e1234_, 0.0, 0.0)) - ((vec4<f32>(self_.e23_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e23_, other.e41_, 0.0, 0.0)) - ((vec4<f32>(self_.e31_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e31_, other.e42_, 0.0, 0.0)) - ((vec4<f32>(self_.e12_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e12_, other.e43_, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((other.e31_ * self_.e3_) - (other.e12_ * self_.e2_), (other.e12_ * self_.e1_) - (other.e23_ * self_.e3_), (other.e23_ * self_.e2_) - (other.e31_ * self_.e1_), -(other.e41_ * self_.e1_) - (other.e42_ * self_.e2_) - (other.e43_ * self_.e3_) - (other.e23_ * self_.e423_) - (other.e31_ * self_.e431_) - (other.e12_ * self_.e412_)) + (vec4<f32>(other.scalar) * self_groups.group1_) + (vec4<f32>(self_.e321_) * vec4<f32>(other.e23_, other.e31_, other.e12_, other.e1234_)), 
        /* e41, e42, e43 */ vec4<f32>((other.e42_ * self_.e12_) + (other.e31_ * self_.e43_) - (other.e43_ * self_.e31_) - (other.e12_ * self_.e42_), (other.e43_ * self_.e23_) + (other.e12_ * self_.e41_) - (other.e41_ * self_.e12_) - (other.e23_ * self_.e43_), (other.e41_ * self_.e31_) + (other.e23_ * self_.e42_) - (other.e42_ * self_.e23_) - (other.e31_ * self_.e41_), 0.0) + ((vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_) + ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_) + ((vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e41_, other.e42_, other.e43_, 0.0)) + ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e23_, other.e31_, other.e12_, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((other.e31_ * self_.e12_) - (other.e12_ * self_.e31_), (other.e12_ * self_.e23_) - (other.e23_ * self_.e12_), (other.e23_ * self_.e31_) - (other.e31_ * self_.e23_), 0.0) + ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_) + ((vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e23_, other.e31_, other.e12_, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e42_ * self_.e3_) + (other.e1234_ * self_.e1_) + (other.e23_ * self_.e4_) + (other.e31_ * self_.e412_) + (other.scalar * self_.e423_) - (other.e43_ * self_.e2_) - (other.e12_ * self_.e431_), (other.e43_ * self_.e1_) + (other.e1234_ * self_.e2_) + (other.e31_ * self_.e4_) + (other.e12_ * self_.e423_) + (other.scalar * self_.e431_) - (other.e41_ * self_.e3_) - (other.e23_ * self_.e412_), (other.e41_ * self_.e2_) + (other.e1234_ * self_.e3_) + (other.e23_ * self_.e431_) + (other.e12_ * self_.e4_) + (other.scalar * self_.e412_) - (other.e42_ * self_.e1_) - (other.e31_ * self_.e423_), -(other.e23_ * self_.e1_) - (other.e31_ * self_.e2_) - (other.e12_ * self_.e3_)) + (vec4<f32>(self_.e321_) * vec4<f32>(other.e41_, other.e42_, other.e43_, other.scalar))
    ));
}
fn multiVector_geometricProduct_multiVector(self_: MultiVector, other: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, (other.e1234_ * self_.scalar) + (other.e321_ * self_.e4_) - (other.e1_ * self_.e423_) - (other.e2_ * self_.e431_) - (other.e3_ * self_.e412_) - (other.e23_ * self_.e41_) - (other.e31_ * self_.e42_) - (other.e12_ * self_.e43_), 0.0, 0.0) + ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_) + ((vec4<f32>(self_.e1_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e1_, other.e423_, 0.0, 0.0)) + ((vec4<f32>(self_.e2_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e2_, other.e431_, 0.0, 0.0)) + ((vec4<f32>(self_.e3_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e3_, other.e412_, 0.0, 0.0)) - ((vec4<f32>(self_.e23_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e23_, other.e41_, 0.0, 0.0)) - ((vec4<f32>(self_.e31_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e31_, other.e42_, 0.0, 0.0)) - ((vec4<f32>(self_.e12_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e12_, other.e43_, 0.0, 0.0)) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e321_, other.e4_, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((other.e2_ * self_.e12_) + (other.e31_ * self_.e3_) + (other.e321_ * self_.e23_) - (other.e3_ * self_.e31_) - (other.e12_ * self_.e2_), (other.e3_ * self_.e23_) + (other.e12_ * self_.e1_) + (other.e321_ * self_.e31_) - (other.e1_ * self_.e12_) - (other.e23_ * self_.e3_), (other.e1_ * self_.e31_) + (other.e23_ * self_.e2_) + (other.e321_ * self_.e12_) - (other.e2_ * self_.e23_) - (other.e31_ * self_.e1_), (other.e1_ * self_.e41_) + (other.e2_ * self_.e42_) + (other.e3_ * self_.e43_) - (other.e41_ * self_.e1_) - (other.e42_ * self_.e2_) - (other.e43_ * self_.e3_) - (other.e23_ * self_.e423_) - (other.e31_ * self_.e431_) - (other.e12_ * self_.e412_) - (other.e423_ * self_.e23_) - (other.e431_ * self_.e31_) - (other.e412_ * self_.e12_) - (other.e321_ * self_.e1234_)) + (vec4<f32>(other.scalar) * self_groups.group1_) + (vec4<f32>(self_.scalar) * other_groups.group1_) + (vec4<f32>(self_.e321_) * vec4<f32>(other.e23_, other.e31_, other.e12_, other.e1234_)), 
        /* e41, e42, e43 */ vec4<f32>((other.e2_ * self_.e412_) + (other.e42_ * self_.e12_) + (other.e31_ * self_.e43_) + (other.e412_ * self_.e2_) - (other.e3_ * self_.e431_) - (other.e43_ * self_.e31_) - (other.e12_ * self_.e42_) - (other.e431_ * self_.e3_), (other.e3_ * self_.e423_) + (other.e43_ * self_.e23_) + (other.e12_ * self_.e41_) + (other.e423_ * self_.e3_) - (other.e1_ * self_.e412_) - (other.e41_ * self_.e12_) - (other.e23_ * self_.e43_) - (other.e412_ * self_.e1_), (other.e1_ * self_.e431_) + (other.e41_ * self_.e31_) + (other.e23_ * self_.e42_) + (other.e431_ * self_.e1_) - (other.e2_ * self_.e423_) - (other.e42_ * self_.e23_) - (other.e31_ * self_.e41_) - (other.e423_ * self_.e2_), 0.0) + ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_) + ((vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_) + ((vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0)) + ((vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group2_) + ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group3_) + ((vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0)) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0)) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((other.e3_ * self_.e2_) + (other.e31_ * self_.e12_) - (other.e2_ * self_.e3_) - (other.e12_ * self_.e31_), (other.e1_ * self_.e3_) + (other.e12_ * self_.e23_) - (other.e3_ * self_.e1_) - (other.e23_ * self_.e12_), (other.e2_ * self_.e1_) + (other.e23_ * self_.e31_) - (other.e1_ * self_.e2_) - (other.e31_ * self_.e23_), 0.0) + ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_) + ((vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group3_) - ((vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0)) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e1234_ * self_.e1_) + (other.e3_ * self_.e42_) + (other.e4_ * self_.e23_) + (other.e41_ * self_.e321_) + (other.e42_ * self_.e3_) + (other.e23_ * self_.e4_) + (other.e31_ * self_.e412_) + (other.e431_ * self_.e12_) - (other.e1_ * self_.e1234_) - (other.e2_ * self_.e43_) - (other.e43_ * self_.e2_) - (other.e12_ * self_.e431_) - (other.e412_ * self_.e31_) - (other.e321_ * self_.e41_), (other.e1234_ * self_.e2_) + (other.e1_ * self_.e43_) + (other.e4_ * self_.e31_) + (other.e42_ * self_.e321_) + (other.e43_ * self_.e1_) + (other.e31_ * self_.e4_) + (other.e12_ * self_.e423_) + (other.e412_ * self_.e23_) - (other.e2_ * self_.e1234_) - (other.e3_ * self_.e41_) - (other.e41_ * self_.e3_) - (other.e23_ * self_.e412_) - (other.e423_ * self_.e12_) - (other.e321_ * self_.e42_), (other.e1234_ * self_.e3_) + (other.e2_ * self_.e41_) + (other.e4_ * self_.e12_) + (other.e41_ * self_.e2_) + (other.e43_ * self_.e321_) + (other.e23_ * self_.e431_) + (other.e12_ * self_.e4_) + (other.e423_ * self_.e31_) - (other.e1_ * self_.e42_) - (other.e3_ * self_.e1234_) - (other.e42_ * self_.e1_) - (other.e31_ * self_.e423_) - (other.e431_ * self_.e23_) - (other.e321_ * self_.e43_), -(other.e1_ * self_.e23_) - (other.e2_ * self_.e31_) - (other.e3_ * self_.e12_) - (other.e23_ * self_.e1_) - (other.e31_ * self_.e2_) - (other.e12_ * self_.e3_)) + (vec4<f32>(other.scalar) * self_groups.group4_) + (vec4<f32>(self_.scalar) * other_groups.group4_)
    ));
}
fn multiVector_geometricProduct_origin(self_: MultiVector, other: Origin) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(1.0, self_.e321_ * other.e4_, 0.0, 0.0) * vec2<f32>(0.0, -1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, self_.scalar * other.e4_) * vec4<f32>(0.0, 0.0, 0.0, 1.0), 
        /* e41, e42, e43 */ (vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e23_ * other.e4_, self_.e31_ * other.e4_, self_.e12_ * other.e4_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
}
fn multiVector_geometricProduct_plane(self_: MultiVector, other: Plane) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.e321_ * other.e321_, (self_.e1_ * other.e423_) + (self_.e2_ * other.e431_) + (self_.e3_ * other.e412_) + (self_.e4_ * other.e321_), 0.0, 0.0) * vec2<f32>(-1.0, 1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e23_ * other.e321_, self_.e31_ * other.e321_, self_.e12_ * other.e321_, -(self_.e1234_ * other.e321_) - (self_.e23_ * other.e423_) - (self_.e31_ * other.e431_) - (self_.e12_ * other.e412_)), 
        /* e41, e42, e43 */ vec4<f32>((self_.e2_ * other.e412_) - (self_.e3_ * other.e431_), (self_.e3_ * other.e423_) - (self_.e1_ * other.e412_), (self_.e1_ * other.e431_) - (self_.e2_ * other.e423_), 0.0) + ((vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0)) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0)), 
        /* e23, e31, e12 */ (vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0) * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.scalar * other.e423_) + (self_.e12_ * other.e431_) - (self_.e41_ * other.e321_) - (self_.e31_ * other.e412_), (self_.scalar * other.e431_) + (self_.e23_ * other.e412_) - (self_.e42_ * other.e321_) - (self_.e12_ * other.e423_), (self_.scalar * other.e412_) + (self_.e31_ * other.e423_) - (self_.e43_ * other.e321_) - (self_.e23_ * other.e431_), self_.scalar * other.e321_)
    ));
}
fn multiVector_geometricProduct_point(self_: MultiVector, other: Point) -> MultiVector {
    let other_groups = point_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((self_.e1_ * other.e1_) + (self_.e2_ * other.e2_) + (self_.e3_ * other.e3_), -(self_.e423_ * other.e1_) - (self_.e431_ * other.e2_) - (self_.e412_ * other.e3_) - (self_.e321_ * other.e4_), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e12_ * other.e2_) - (self_.e31_ * other.e3_), (self_.e23_ * other.e3_) - (self_.e12_ * other.e1_), (self_.e31_ * other.e1_) - (self_.e23_ * other.e2_), (self_.e41_ * other.e1_) + (self_.e42_ * other.e2_) + (self_.e43_ * other.e3_)) + (vec4<f32>(self_.scalar) * other_groups.group0_), 
        /* e41, e42, e43 */ vec4<f32>((self_.e412_ * other.e2_) - (self_.e431_ * other.e3_), (self_.e423_ * other.e3_) - (self_.e412_ * other.e1_), (self_.e431_ * other.e1_) - (self_.e423_ * other.e2_), 0.0) + ((vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0)) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((self_.e2_ * other.e3_) - (self_.e3_ * other.e2_), (self_.e3_ * other.e1_) - (self_.e1_ * other.e3_), (self_.e1_ * other.e2_) - (self_.e2_ * other.e1_), 0.0) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e42_ * other.e3_) + (self_.e23_ * other.e4_) - (self_.e1234_ * other.e1_) - (self_.e43_ * other.e2_), (self_.e43_ * other.e1_) + (self_.e31_ * other.e4_) - (self_.e1234_ * other.e2_) - (self_.e41_ * other.e3_), (self_.e41_ * other.e2_) + (self_.e12_ * other.e4_) - (self_.e1234_ * other.e3_) - (self_.e42_ * other.e1_), -(self_.e23_ * other.e1_) - (self_.e31_ * other.e2_) - (self_.e12_ * other.e3_))
    ));
}
fn multiVector_geometricProduct_scalar(self_: MultiVector, other: Scalar) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ (vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(other.scalar) * self_groups.group1_, 
        /* e41, e42, e43 */ (vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_, 
        /* e23, e31, e12 */ (vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other.scalar) * self_groups.group4_
    ));
}
fn origin_geometricProduct_dualNum(self_: Origin, other: DualNum) -> Origin {
    return Origin(other.scalar * self_.e4_);
}
fn origin_geometricProduct_flector(self_: Origin, other: Flector) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.e4_) * vec4<f32>(other.e1_, other.e2_, other.e3_, other.e321_), 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0)
    ));
}
fn origin_geometricProduct_horizon(self_: Origin, other: Horizon) -> AntiScalar {
    return AntiScalar(other.e321_ * self_.e4_);
}
fn origin_geometricProduct_line(self_: Origin, other: Line) -> Plane {
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(other.e23_ * self_.e4_, other.e31_ * self_.e4_, other.e12_ * self_.e4_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
}
fn origin_geometricProduct_motor(self_: Origin, other: Motor) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, other.scalar * self_.e4_) * vec4<f32>(0.0, 0.0, 0.0, 1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e23_ * self_.e4_, other.e31_ * self_.e4_, other.e12_ * self_.e4_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
}
fn origin_geometricProduct_multiVector(self_: Origin, other: MultiVector) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(1.0, other.e321_ * self_.e4_, 0.0, 0.0) * vec2<f32>(0.0, 1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, other.scalar * self_.e4_) * vec4<f32>(0.0, 0.0, 0.0, 1.0), 
        /* e41, e42, e43 */ (vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e23_ * self_.e4_, other.e31_ * self_.e4_, other.e12_ * self_.e4_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
}
fn origin_geometricProduct_plane(self_: Origin, other: Plane) -> AntiScalar {
    return AntiScalar(self_.e4_ * other.e321_);
}
fn origin_geometricProduct_point(self_: Origin, other: Point) -> Line {
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0)
    ));
}
fn origin_geometricProduct_scalar(self_: Origin, other: Scalar) -> Origin {
    return Origin(self_.e4_ * other.scalar);
}
fn plane_geometricProduct_antiScalar(self_: Plane, other: AntiScalar) -> Origin {
    return Origin(other.e1234_ * self_.e321_);
}
fn plane_geometricProduct_dualNum(self_: Plane, other: DualNum) -> Flector {
    let self_groups = plane_grouped(self_);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, other.e1234_ * self_.e321_) * vec4<f32>(0.0, 0.0, 0.0, 1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.scalar) * self_groups.group0_
    ));
}
fn plane_geometricProduct_flector(self_: Plane, other: Flector) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other.e2_ * self_.e412_) + (other.e321_ * self_.e423_) - (other.e3_ * self_.e431_), (other.e3_ * self_.e423_) + (other.e321_ * self_.e431_) - (other.e1_ * self_.e412_), (other.e1_ * self_.e431_) + (other.e321_ * self_.e412_) - (other.e2_ * self_.e423_), -(other.e1_ * self_.e423_) - (other.e2_ * self_.e431_) - (other.e3_ * self_.e412_)) - (vec4<f32>(self_.e321_) * vec4<f32>(other.e423_, other.e431_, other.e412_, other.e4_)), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.e321_) * vec4<f32>(other.e1_, other.e2_, other.e3_, other.e321_) * vec4<f32>(-1.0)
    ));
}
fn plane_geometricProduct_horizon(self_: Plane, other: Horizon) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e321_ * self_.e423_, other.e321_ * self_.e431_, other.e321_ * self_.e412_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(1.0, 1.0, 1.0, other.e321_ * self_.e321_) * vec4<f32>(0.0, 0.0, 0.0, -1.0)
    ));
}
fn plane_geometricProduct_line(self_: Plane, other: Line) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e23_ * self_.e321_, other.e31_ * self_.e321_, other.e12_ * self_.e321_, -(other.e23_ * self_.e423_) - (other.e31_ * self_.e431_) - (other.e12_ * self_.e412_)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e41_ * self_.e321_) + (other.e31_ * self_.e412_) - (other.e12_ * self_.e431_), (other.e42_ * self_.e321_) + (other.e12_ * self_.e423_) - (other.e23_ * self_.e412_), (other.e43_ * self_.e321_) + (other.e23_ * self_.e431_) - (other.e31_ * self_.e423_), 0.0)
    ));
}
fn plane_geometricProduct_motor(self_: Plane, other: Motor) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e23_ * self_.e321_, other.e31_ * self_.e321_, other.e12_ * self_.e321_, (other.e1234_ * self_.e321_) - (other.e23_ * self_.e423_) - (other.e31_ * self_.e431_) - (other.e12_ * self_.e412_)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e41_ * self_.e321_) + (other.e31_ * self_.e412_) + (other.scalar * self_.e423_) - (other.e12_ * self_.e431_), (other.e42_ * self_.e321_) + (other.e12_ * self_.e423_) + (other.scalar * self_.e431_) - (other.e23_ * self_.e412_), (other.e43_ * self_.e321_) + (other.e23_ * self_.e431_) + (other.scalar * self_.e412_) - (other.e31_ * self_.e423_), other.scalar * self_.e321_)
    ));
}
fn plane_geometricProduct_multiVector(self_: Plane, other: MultiVector) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.e321_ * self_.e321_, -(other.e1_ * self_.e423_) - (other.e2_ * self_.e431_) - (other.e3_ * self_.e412_) - (other.e4_ * self_.e321_), 0.0, 0.0) * vec2<f32>(-1.0, 1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(other.e23_ * self_.e321_, other.e31_ * self_.e321_, other.e12_ * self_.e321_, (other.e1234_ * self_.e321_) - (other.e23_ * self_.e423_) - (other.e31_ * self_.e431_) - (other.e12_ * self_.e412_)), 
        /* e41, e42, e43 */ vec4<f32>((other.e2_ * self_.e412_) - (other.e3_ * self_.e431_), (other.e3_ * self_.e423_) - (other.e1_ * self_.e412_), (other.e1_ * self_.e431_) - (other.e2_ * self_.e423_), 0.0) + ((vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0)) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0)), 
        /* e23, e31, e12 */ (vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0) * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.scalar * self_.e423_) + (other.e41_ * self_.e321_) + (other.e31_ * self_.e412_) - (other.e12_ * self_.e431_), (other.scalar * self_.e431_) + (other.e42_ * self_.e321_) + (other.e12_ * self_.e423_) - (other.e23_ * self_.e412_), (other.scalar * self_.e412_) + (other.e43_ * self_.e321_) + (other.e23_ * self_.e431_) - (other.e31_ * self_.e423_), other.scalar * self_.e321_)
    ));
}
fn plane_geometricProduct_origin(self_: Plane, other: Origin) -> AntiScalar {
    return AntiScalar(other.e4_ * self_.e321_ * -1.0);
}
fn plane_geometricProduct_plane(self_: Plane, other: Plane) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other.e321_ * self_.e423_) - (other.e423_ * self_.e321_), (other.e321_ * self_.e431_) - (other.e431_ * self_.e321_), (other.e321_ * self_.e412_) - (other.e412_ * self_.e321_), 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(1.0, 1.0, 1.0, other.e321_ * self_.e321_) * vec4<f32>(0.0, 0.0, 0.0, -1.0)
    ));
}
fn plane_geometricProduct_point(self_: Plane, other: Point) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((self_.e412_ * other.e2_) - (self_.e431_ * other.e3_), (self_.e423_ * other.e3_) - (self_.e412_ * other.e1_), (self_.e431_ * other.e1_) - (self_.e423_ * other.e2_), -(self_.e423_ * other.e1_) - (self_.e431_ * other.e2_) - (self_.e412_ * other.e3_) - (self_.e321_ * other.e4_)), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.e321_ * other.e1_, self_.e321_ * other.e2_, self_.e321_ * other.e3_, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0)
    ));
}
fn plane_geometricProduct_scalar(self_: Plane, other: Scalar) -> Plane {
    let self_groups = plane_grouped(self_);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(other.scalar) * self_groups.group0_
    ));
}
fn point_geometricProduct_antiScalar(self_: Point, other: AntiScalar) -> Plane {
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(other.e1234_ * self_.e1_, other.e1234_ * self_.e2_, other.e1234_ * self_.e3_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
}
fn point_geometricProduct_dualNum(self_: Point, other: DualNum) -> Flector {
    let self_groups = point_grouped(self_);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.scalar) * self_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e1234_ * self_.e1_, other.e1234_ * self_.e2_, other.e1234_ * self_.e3_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
}
fn point_geometricProduct_flector(self_: Point, other: Flector) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other.e412_ * self_.e2_) - (other.e4_ * self_.e1_) - (other.e431_ * self_.e3_), (other.e423_ * self_.e3_) - (other.e4_ * self_.e2_) - (other.e412_ * self_.e1_), (other.e431_ * self_.e1_) - (other.e4_ * self_.e3_) - (other.e423_ * self_.e2_), (other.e423_ * self_.e1_) + (other.e431_ * self_.e2_) + (other.e412_ * self_.e3_)) + (vec4<f32>(self_.e4_) * vec4<f32>(other.e1_, other.e2_, other.e3_, other.e321_)), 
        /* e23, e31, e12, scalar */ vec4<f32>((other.e3_ * self_.e2_) - (other.e2_ * self_.e3_) - (other.e321_ * self_.e1_), (other.e1_ * self_.e3_) - (other.e3_ * self_.e1_) - (other.e321_ * self_.e2_), (other.e2_ * self_.e1_) - (other.e1_ * self_.e2_) - (other.e321_ * self_.e3_), (other.e1_ * self_.e1_) + (other.e2_ * self_.e2_) + (other.e3_ * self_.e3_))
    ));
}
fn point_geometricProduct_horizon(self_: Point, other: Horizon) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(1.0, 1.0, 1.0, other.e321_ * self_.e4_) * vec4<f32>(0.0, 0.0, 0.0, 1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(other.e321_ * self_.e1_, other.e321_ * self_.e2_, other.e321_ * self_.e3_, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0)
    ));
}
fn point_geometricProduct_line(self_: Point, other: Line) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other.e31_ * self_.e3_) - (other.e12_ * self_.e2_), (other.e12_ * self_.e1_) - (other.e23_ * self_.e3_), (other.e23_ * self_.e2_) - (other.e31_ * self_.e1_), -(other.e41_ * self_.e1_) - (other.e42_ * self_.e2_) - (other.e43_ * self_.e3_)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e42_ * self_.e3_) + (other.e23_ * self_.e4_) - (other.e43_ * self_.e2_), (other.e43_ * self_.e1_) + (other.e31_ * self_.e4_) - (other.e41_ * self_.e3_), (other.e41_ * self_.e2_) + (other.e12_ * self_.e4_) - (other.e42_ * self_.e1_), -(other.e23_ * self_.e1_) - (other.e31_ * self_.e2_) - (other.e12_ * self_.e3_))
    ));
}
fn point_geometricProduct_motor(self_: Point, other: Motor) -> Flector {
    let self_groups = point_grouped(self_);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other.e31_ * self_.e3_) - (other.e12_ * self_.e2_), (other.e12_ * self_.e1_) - (other.e23_ * self_.e3_), (other.e23_ * self_.e2_) - (other.e31_ * self_.e1_), -(other.e41_ * self_.e1_) - (other.e42_ * self_.e2_) - (other.e43_ * self_.e3_)) + (vec4<f32>(other.scalar) * self_groups.group0_), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e42_ * self_.e3_) + (other.e1234_ * self_.e1_) + (other.e23_ * self_.e4_) - (other.e43_ * self_.e2_), (other.e43_ * self_.e1_) + (other.e1234_ * self_.e2_) + (other.e31_ * self_.e4_) - (other.e41_ * self_.e3_), (other.e41_ * self_.e2_) + (other.e1234_ * self_.e3_) + (other.e12_ * self_.e4_) - (other.e42_ * self_.e1_), -(other.e23_ * self_.e1_) - (other.e31_ * self_.e2_) - (other.e12_ * self_.e3_))
    ));
}
fn point_geometricProduct_multiVector(self_: Point, other: MultiVector) -> MultiVector {
    let self_groups = point_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, other.e321_ * self_.e4_, 0.0, 0.0) + ((vec4<f32>(self_.e1_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e1_, other.e423_, 0.0, 0.0)) + ((vec4<f32>(self_.e2_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e2_, other.e431_, 0.0, 0.0)) + ((vec4<f32>(self_.e3_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e3_, other.e412_, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((other.e31_ * self_.e3_) - (other.e12_ * self_.e2_), (other.e12_ * self_.e1_) - (other.e23_ * self_.e3_), (other.e23_ * self_.e2_) - (other.e31_ * self_.e1_), -(other.e41_ * self_.e1_) - (other.e42_ * self_.e2_) - (other.e43_ * self_.e3_)) + (vec4<f32>(other.scalar) * self_groups.group0_), 
        /* e41, e42, e43 */ vec4<f32>((other.e412_ * self_.e2_) - (other.e431_ * self_.e3_), (other.e423_ * self_.e3_) - (other.e412_ * self_.e1_), (other.e431_ * self_.e1_) - (other.e423_ * self_.e2_), 0.0) + ((vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0)) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((other.e3_ * self_.e2_) - (other.e2_ * self_.e3_), (other.e1_ * self_.e3_) - (other.e3_ * self_.e1_), (other.e2_ * self_.e1_) - (other.e1_ * self_.e2_), 0.0) - ((vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e1234_ * self_.e1_) + (other.e42_ * self_.e3_) + (other.e23_ * self_.e4_) - (other.e43_ * self_.e2_), (other.e1234_ * self_.e2_) + (other.e43_ * self_.e1_) + (other.e31_ * self_.e4_) - (other.e41_ * self_.e3_), (other.e1234_ * self_.e3_) + (other.e41_ * self_.e2_) + (other.e12_ * self_.e4_) - (other.e42_ * self_.e1_), -(other.e23_ * self_.e1_) - (other.e31_ * self_.e2_) - (other.e12_ * self_.e3_))
    ));
}
fn point_geometricProduct_origin(self_: Point, other: Origin) -> Line {
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0)
    ));
}
fn point_geometricProduct_plane(self_: Point, other: Plane) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other.e412_ * self_.e2_) - (other.e431_ * self_.e3_), (other.e423_ * self_.e3_) - (other.e412_ * self_.e1_), (other.e431_ * self_.e1_) - (other.e423_ * self_.e2_), (other.e423_ * self_.e1_) + (other.e431_ * self_.e2_) + (other.e412_ * self_.e3_) + (other.e321_ * self_.e4_)), 
        /* e23, e31, e12, scalar */ vec4<f32>(other.e321_ * self_.e1_, other.e321_ * self_.e2_, other.e321_ * self_.e3_, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0)
    ));
}
fn point_geometricProduct_point(self_: Point, other: Point) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other.e1_ * self_.e4_) - (other.e4_ * self_.e1_), (other.e2_ * self_.e4_) - (other.e4_ * self_.e2_), (other.e3_ * self_.e4_) - (other.e4_ * self_.e3_), 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>((other.e3_ * self_.e2_) - (other.e2_ * self_.e3_), (other.e1_ * self_.e3_) - (other.e3_ * self_.e1_), (other.e2_ * self_.e1_) - (other.e1_ * self_.e2_), (other.e1_ * self_.e1_) + (other.e2_ * self_.e2_) + (other.e3_ * self_.e3_))
    ));
}
fn point_geometricProduct_scalar(self_: Point, other: Scalar) -> Point {
    let self_groups = point_grouped(self_);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.scalar) * self_groups.group0_
    ));
}
fn scalar_geometricProduct_antiScalar(self_: Scalar, other: AntiScalar) -> AntiScalar {
    return AntiScalar(other.e1234_ * self_.scalar);
}
fn scalar_geometricProduct_dualNum(self_: Scalar, other: DualNum) -> DualNum {
    let other_groups = dualNum_grouped(other);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ (vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * other_groups.group0_
    ));
}
fn scalar_geometricProduct_flector(self_: Scalar, other: Flector) -> Flector {
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.scalar) * other_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.scalar) * other_groups.group1_
    ));
}
fn scalar_geometricProduct_horizon(self_: Scalar, other: Horizon) -> Horizon {
    return Horizon(other.e321_ * self_.scalar);
}
fn scalar_geometricProduct_line(self_: Scalar, other: Line) -> Line {
    let other_groups = line_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group1_
    ));
}
fn scalar_geometricProduct_motor(self_: Scalar, other: Motor) -> Motor {
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.scalar) * other_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.scalar) * other_groups.group1_
    ));
}
fn scalar_geometricProduct_multiVector(self_: Scalar, other: MultiVector) -> MultiVector {
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ (vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * other_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.scalar) * other_groups.group1_, 
        /* e41, e42, e43 */ (vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group2_, 
        /* e23, e31, e12 */ (vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.scalar) * other_groups.group4_
    ));
}
fn scalar_geometricProduct_origin(self_: Scalar, other: Origin) -> Origin {
    return Origin(other.e4_ * self_.scalar);
}
fn scalar_geometricProduct_plane(self_: Scalar, other: Plane) -> Plane {
    let other_groups = plane_grouped(other);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(self_.scalar) * other_groups.group0_
    ));
}
fn scalar_geometricProduct_point(self_: Scalar, other: Point) -> Point {
    let other_groups = point_grouped(other);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.scalar) * other_groups.group0_
    ));
}
fn scalar_geometricProduct_scalar(self_: Scalar, other: Scalar) -> Scalar {
    return Scalar(other.scalar * self_.scalar);
}
fn antiScalar_geometricQuotient_dualNum(self_: AntiScalar, other: DualNum) -> AntiScalar {
    let dot_product: Scalar = Scalar(pow(other.scalar, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return AntiScalar(self_.e1234_ * inverse.scalar);
}
fn antiScalar_geometricQuotient_flector(self_: AntiScalar, other: Flector) -> AntiScalar {
    let dot_product: Scalar = Scalar(pow(other.e1_, 2) + pow(other.e2_, 2) + pow(other.e3_, 2) - pow(other.e321_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return AntiScalar(self_.e1234_ * inverse.scalar);
}
fn antiScalar_geometricQuotient_horizon(self_: AntiScalar, other: Horizon) -> AntiScalar {
    let dot_product: Scalar = Scalar(pow(other.e321_, 2) * -1.0);
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return AntiScalar(self_.e1234_ * inverse.scalar);
}
fn antiScalar_geometricQuotient_line(self_: AntiScalar, other: Line) -> AntiScalar {
    let dot_product: Scalar = Scalar(-pow(other.e23_, 2) - pow(other.e31_, 2) - pow(other.e12_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return AntiScalar(self_.e1234_ * inverse.scalar);
}
fn antiScalar_geometricQuotient_motor(self_: AntiScalar, other: Motor) -> AntiScalar {
    let dot_product: Scalar = Scalar(pow(other.scalar, 2) - pow(other.e23_, 2) - pow(other.e31_, 2) - pow(other.e12_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return AntiScalar(self_.e1234_ * inverse.scalar);
}
fn antiScalar_geometricQuotient_multiVector(self_: AntiScalar, other: MultiVector) -> AntiScalar {
    let dot_product: Scalar = Scalar(pow(other.scalar, 2) + pow(other.e1_, 2) + pow(other.e2_, 2) + pow(other.e3_, 2) - pow(other.e23_, 2) - pow(other.e31_, 2) - pow(other.e12_, 2) - pow(other.e321_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return AntiScalar(self_.e1234_ * inverse.scalar);
}
fn antiScalar_geometricQuotient_plane(self_: AntiScalar, other: Plane) -> AntiScalar {
    let dot_product: Scalar = Scalar(pow(other.e321_, 2) * -1.0);
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return AntiScalar(self_.e1234_ * inverse.scalar);
}
fn antiScalar_geometricQuotient_point(self_: AntiScalar, other: Point) -> AntiScalar {
    let dot_product: Scalar = Scalar(pow(other.e1_, 2) + pow(other.e2_, 2) + pow(other.e3_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return AntiScalar(self_.e1234_ * inverse.scalar);
}
fn antiScalar_geometricQuotient_scalar(self_: AntiScalar, other: Scalar) -> AntiScalar {
    let dot_product: Scalar = Scalar(pow(other.scalar, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return AntiScalar(self_.e1234_ * inverse.scalar);
}
fn dualNum_geometricQuotient_dualNum(self_: DualNum, other: DualNum) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.scalar, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_
    ));
}
fn dualNum_geometricQuotient_flector(self_: DualNum, other: Flector) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.e1_, 2) + pow(other.e2_, 2) + pow(other.e3_, 2) - pow(other.e321_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_
    ));
}
fn dualNum_geometricQuotient_horizon(self_: DualNum, other: Horizon) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.e321_, 2) * -1.0);
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_
    ));
}
fn dualNum_geometricQuotient_line(self_: DualNum, other: Line) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let dot_product: Scalar = Scalar(-pow(other.e23_, 2) - pow(other.e31_, 2) - pow(other.e12_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_
    ));
}
fn dualNum_geometricQuotient_motor(self_: DualNum, other: Motor) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.scalar, 2) - pow(other.e23_, 2) - pow(other.e31_, 2) - pow(other.e12_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_
    ));
}
fn dualNum_geometricQuotient_multiVector(self_: DualNum, other: MultiVector) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.scalar, 2) + pow(other.e1_, 2) + pow(other.e2_, 2) + pow(other.e3_, 2) - pow(other.e23_, 2) - pow(other.e31_, 2) - pow(other.e12_, 2) - pow(other.e321_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_
    ));
}
fn dualNum_geometricQuotient_plane(self_: DualNum, other: Plane) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.e321_, 2) * -1.0);
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_
    ));
}
fn dualNum_geometricQuotient_point(self_: DualNum, other: Point) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.e1_, 2) + pow(other.e2_, 2) + pow(other.e3_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_
    ));
}
fn dualNum_geometricQuotient_scalar(self_: DualNum, other: Scalar) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.scalar, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_
    ));
}
fn flector_geometricQuotient_dualNum(self_: Flector, other: DualNum) -> Flector {
    let self_groups = flector_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.scalar, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_groups.group1_
    ));
}
fn flector_geometricQuotient_flector(self_: Flector, other: Flector) -> Flector {
    let self_groups = flector_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.e1_, 2) + pow(other.e2_, 2) + pow(other.e3_, 2) - pow(other.e321_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_groups.group1_
    ));
}
fn flector_geometricQuotient_horizon(self_: Flector, other: Horizon) -> Flector {
    let self_groups = flector_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.e321_, 2) * -1.0);
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_groups.group1_
    ));
}
fn flector_geometricQuotient_line(self_: Flector, other: Line) -> Flector {
    let self_groups = flector_grouped(self_);
    let dot_product: Scalar = Scalar(-pow(other.e23_, 2) - pow(other.e31_, 2) - pow(other.e12_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_groups.group1_
    ));
}
fn flector_geometricQuotient_motor(self_: Flector, other: Motor) -> Flector {
    let self_groups = flector_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.scalar, 2) - pow(other.e23_, 2) - pow(other.e31_, 2) - pow(other.e12_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_groups.group1_
    ));
}
fn flector_geometricQuotient_multiVector(self_: Flector, other: MultiVector) -> Flector {
    let self_groups = flector_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.scalar, 2) + pow(other.e1_, 2) + pow(other.e2_, 2) + pow(other.e3_, 2) - pow(other.e23_, 2) - pow(other.e31_, 2) - pow(other.e12_, 2) - pow(other.e321_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_groups.group1_
    ));
}
fn flector_geometricQuotient_plane(self_: Flector, other: Plane) -> Flector {
    let self_groups = flector_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.e321_, 2) * -1.0);
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_groups.group1_
    ));
}
fn flector_geometricQuotient_point(self_: Flector, other: Point) -> Flector {
    let self_groups = flector_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.e1_, 2) + pow(other.e2_, 2) + pow(other.e3_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_groups.group1_
    ));
}
fn flector_geometricQuotient_scalar(self_: Flector, other: Scalar) -> Flector {
    let self_groups = flector_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.scalar, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_groups.group1_
    ));
}
fn horizon_geometricQuotient_dualNum(self_: Horizon, other: DualNum) -> Horizon {
    let dot_product: Scalar = Scalar(pow(other.scalar, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return Horizon(self_.e321_ * inverse.scalar);
}
fn horizon_geometricQuotient_flector(self_: Horizon, other: Flector) -> Horizon {
    let dot_product: Scalar = Scalar(pow(other.e1_, 2) + pow(other.e2_, 2) + pow(other.e3_, 2) - pow(other.e321_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return Horizon(self_.e321_ * inverse.scalar);
}
fn horizon_geometricQuotient_horizon(self_: Horizon, other: Horizon) -> Horizon {
    let dot_product: Scalar = Scalar(pow(other.e321_, 2) * -1.0);
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return Horizon(self_.e321_ * inverse.scalar);
}
fn horizon_geometricQuotient_line(self_: Horizon, other: Line) -> Horizon {
    let dot_product: Scalar = Scalar(-pow(other.e23_, 2) - pow(other.e31_, 2) - pow(other.e12_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return Horizon(self_.e321_ * inverse.scalar);
}
fn horizon_geometricQuotient_motor(self_: Horizon, other: Motor) -> Horizon {
    let dot_product: Scalar = Scalar(pow(other.scalar, 2) - pow(other.e23_, 2) - pow(other.e31_, 2) - pow(other.e12_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return Horizon(self_.e321_ * inverse.scalar);
}
fn horizon_geometricQuotient_multiVector(self_: Horizon, other: MultiVector) -> Horizon {
    let dot_product: Scalar = Scalar(pow(other.scalar, 2) + pow(other.e1_, 2) + pow(other.e2_, 2) + pow(other.e3_, 2) - pow(other.e23_, 2) - pow(other.e31_, 2) - pow(other.e12_, 2) - pow(other.e321_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return Horizon(self_.e321_ * inverse.scalar);
}
fn horizon_geometricQuotient_plane(self_: Horizon, other: Plane) -> Horizon {
    let dot_product: Scalar = Scalar(pow(other.e321_, 2) * -1.0);
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return Horizon(self_.e321_ * inverse.scalar);
}
fn horizon_geometricQuotient_point(self_: Horizon, other: Point) -> Horizon {
    let dot_product: Scalar = Scalar(pow(other.e1_, 2) + pow(other.e2_, 2) + pow(other.e3_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return Horizon(self_.e321_ * inverse.scalar);
}
fn horizon_geometricQuotient_scalar(self_: Horizon, other: Scalar) -> Horizon {
    let dot_product: Scalar = Scalar(pow(other.scalar, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return Horizon(self_.e321_ * inverse.scalar);
}
fn line_geometricQuotient_dualNum(self_: Line, other: DualNum) -> Line {
    let self_groups = line_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.scalar, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_
    ));
}
fn line_geometricQuotient_flector(self_: Line, other: Flector) -> Line {
    let self_groups = line_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.e1_, 2) + pow(other.e2_, 2) + pow(other.e3_, 2) - pow(other.e321_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_
    ));
}
fn line_geometricQuotient_horizon(self_: Line, other: Horizon) -> Line {
    let self_groups = line_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.e321_, 2) * -1.0);
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_
    ));
}
fn line_geometricQuotient_line(self_: Line, other: Line) -> Line {
    let self_groups = line_grouped(self_);
    let dot_product: Scalar = Scalar(-pow(other.e23_, 2) - pow(other.e31_, 2) - pow(other.e12_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_
    ));
}
fn line_geometricQuotient_motor(self_: Line, other: Motor) -> Line {
    let self_groups = line_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.scalar, 2) - pow(other.e23_, 2) - pow(other.e31_, 2) - pow(other.e12_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_
    ));
}
fn line_geometricQuotient_multiVector(self_: Line, other: MultiVector) -> Line {
    let self_groups = line_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.scalar, 2) + pow(other.e1_, 2) + pow(other.e2_, 2) + pow(other.e3_, 2) - pow(other.e23_, 2) - pow(other.e31_, 2) - pow(other.e12_, 2) - pow(other.e321_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_
    ));
}
fn line_geometricQuotient_plane(self_: Line, other: Plane) -> Line {
    let self_groups = line_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.e321_, 2) * -1.0);
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_
    ));
}
fn line_geometricQuotient_point(self_: Line, other: Point) -> Line {
    let self_groups = line_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.e1_, 2) + pow(other.e2_, 2) + pow(other.e3_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_
    ));
}
fn line_geometricQuotient_scalar(self_: Line, other: Scalar) -> Line {
    let self_groups = line_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.scalar, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_
    ));
}
fn motor_geometricQuotient_dualNum(self_: Motor, other: DualNum) -> Motor {
    let self_groups = motor_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.scalar, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(inverse.scalar) * self_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(inverse.scalar) * self_groups.group1_
    ));
}
fn motor_geometricQuotient_flector(self_: Motor, other: Flector) -> Motor {
    let self_groups = motor_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.e1_, 2) + pow(other.e2_, 2) + pow(other.e3_, 2) - pow(other.e321_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(inverse.scalar) * self_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(inverse.scalar) * self_groups.group1_
    ));
}
fn motor_geometricQuotient_horizon(self_: Motor, other: Horizon) -> Motor {
    let self_groups = motor_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.e321_, 2) * -1.0);
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(inverse.scalar) * self_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(inverse.scalar) * self_groups.group1_
    ));
}
fn motor_geometricQuotient_line(self_: Motor, other: Line) -> Motor {
    let self_groups = motor_grouped(self_);
    let dot_product: Scalar = Scalar(-pow(other.e23_, 2) - pow(other.e31_, 2) - pow(other.e12_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(inverse.scalar) * self_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(inverse.scalar) * self_groups.group1_
    ));
}
fn motor_geometricQuotient_motor(self_: Motor, other: Motor) -> Motor {
    let self_groups = motor_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.scalar, 2) - pow(other.e23_, 2) - pow(other.e31_, 2) - pow(other.e12_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(inverse.scalar) * self_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(inverse.scalar) * self_groups.group1_
    ));
}
fn motor_geometricQuotient_multiVector(self_: Motor, other: MultiVector) -> Motor {
    let self_groups = motor_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.scalar, 2) + pow(other.e1_, 2) + pow(other.e2_, 2) + pow(other.e3_, 2) - pow(other.e23_, 2) - pow(other.e31_, 2) - pow(other.e12_, 2) - pow(other.e321_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(inverse.scalar) * self_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(inverse.scalar) * self_groups.group1_
    ));
}
fn motor_geometricQuotient_plane(self_: Motor, other: Plane) -> Motor {
    let self_groups = motor_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.e321_, 2) * -1.0);
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(inverse.scalar) * self_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(inverse.scalar) * self_groups.group1_
    ));
}
fn motor_geometricQuotient_point(self_: Motor, other: Point) -> Motor {
    let self_groups = motor_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.e1_, 2) + pow(other.e2_, 2) + pow(other.e3_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(inverse.scalar) * self_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(inverse.scalar) * self_groups.group1_
    ));
}
fn motor_geometricQuotient_scalar(self_: Motor, other: Scalar) -> Motor {
    let self_groups = motor_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.scalar, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(inverse.scalar) * self_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(inverse.scalar) * self_groups.group1_
    ));
}
fn multiVector_geometricQuotient_dualNum(self_: MultiVector, other: DualNum) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.scalar, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_groups.group1_, 
        /* e41, e42, e43 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_, 
        /* e23, e31, e12 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_groups.group4_
    ));
}
fn multiVector_geometricQuotient_flector(self_: MultiVector, other: Flector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.e1_, 2) + pow(other.e2_, 2) + pow(other.e3_, 2) - pow(other.e321_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_groups.group1_, 
        /* e41, e42, e43 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_, 
        /* e23, e31, e12 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_groups.group4_
    ));
}
fn multiVector_geometricQuotient_horizon(self_: MultiVector, other: Horizon) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.e321_, 2) * -1.0);
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_groups.group1_, 
        /* e41, e42, e43 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_, 
        /* e23, e31, e12 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_groups.group4_
    ));
}
fn multiVector_geometricQuotient_line(self_: MultiVector, other: Line) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let dot_product: Scalar = Scalar(-pow(other.e23_, 2) - pow(other.e31_, 2) - pow(other.e12_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_groups.group1_, 
        /* e41, e42, e43 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_, 
        /* e23, e31, e12 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_groups.group4_
    ));
}
fn multiVector_geometricQuotient_motor(self_: MultiVector, other: Motor) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.scalar, 2) - pow(other.e23_, 2) - pow(other.e31_, 2) - pow(other.e12_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_groups.group1_, 
        /* e41, e42, e43 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_, 
        /* e23, e31, e12 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_groups.group4_
    ));
}
fn multiVector_geometricQuotient_multiVector(self_: MultiVector, other: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.scalar, 2) + pow(other.e1_, 2) + pow(other.e2_, 2) + pow(other.e3_, 2) - pow(other.e23_, 2) - pow(other.e31_, 2) - pow(other.e12_, 2) - pow(other.e321_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_groups.group1_, 
        /* e41, e42, e43 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_, 
        /* e23, e31, e12 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_groups.group4_
    ));
}
fn multiVector_geometricQuotient_plane(self_: MultiVector, other: Plane) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.e321_, 2) * -1.0);
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_groups.group1_, 
        /* e41, e42, e43 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_, 
        /* e23, e31, e12 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_groups.group4_
    ));
}
fn multiVector_geometricQuotient_point(self_: MultiVector, other: Point) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.e1_, 2) + pow(other.e2_, 2) + pow(other.e3_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_groups.group1_, 
        /* e41, e42, e43 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_, 
        /* e23, e31, e12 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_groups.group4_
    ));
}
fn multiVector_geometricQuotient_scalar(self_: MultiVector, other: Scalar) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.scalar, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_groups.group1_, 
        /* e41, e42, e43 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_, 
        /* e23, e31, e12 */ (vec4<f32>(inverse.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_groups.group4_
    ));
}
fn origin_geometricQuotient_dualNum(self_: Origin, other: DualNum) -> Origin {
    let dot_product: Scalar = Scalar(pow(other.scalar, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return Origin(self_.e4_ * inverse.scalar);
}
fn origin_geometricQuotient_flector(self_: Origin, other: Flector) -> Origin {
    let dot_product: Scalar = Scalar(pow(other.e1_, 2) + pow(other.e2_, 2) + pow(other.e3_, 2) - pow(other.e321_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return Origin(self_.e4_ * inverse.scalar);
}
fn origin_geometricQuotient_horizon(self_: Origin, other: Horizon) -> Origin {
    let dot_product: Scalar = Scalar(pow(other.e321_, 2) * -1.0);
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return Origin(self_.e4_ * inverse.scalar);
}
fn origin_geometricQuotient_line(self_: Origin, other: Line) -> Origin {
    let dot_product: Scalar = Scalar(-pow(other.e23_, 2) - pow(other.e31_, 2) - pow(other.e12_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return Origin(self_.e4_ * inverse.scalar);
}
fn origin_geometricQuotient_motor(self_: Origin, other: Motor) -> Origin {
    let dot_product: Scalar = Scalar(pow(other.scalar, 2) - pow(other.e23_, 2) - pow(other.e31_, 2) - pow(other.e12_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return Origin(self_.e4_ * inverse.scalar);
}
fn origin_geometricQuotient_multiVector(self_: Origin, other: MultiVector) -> Origin {
    let dot_product: Scalar = Scalar(pow(other.scalar, 2) + pow(other.e1_, 2) + pow(other.e2_, 2) + pow(other.e3_, 2) - pow(other.e23_, 2) - pow(other.e31_, 2) - pow(other.e12_, 2) - pow(other.e321_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return Origin(self_.e4_ * inverse.scalar);
}
fn origin_geometricQuotient_plane(self_: Origin, other: Plane) -> Origin {
    let dot_product: Scalar = Scalar(pow(other.e321_, 2) * -1.0);
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return Origin(self_.e4_ * inverse.scalar);
}
fn origin_geometricQuotient_point(self_: Origin, other: Point) -> Origin {
    let dot_product: Scalar = Scalar(pow(other.e1_, 2) + pow(other.e2_, 2) + pow(other.e3_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return Origin(self_.e4_ * inverse.scalar);
}
fn origin_geometricQuotient_scalar(self_: Origin, other: Scalar) -> Origin {
    let dot_product: Scalar = Scalar(pow(other.scalar, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return Origin(self_.e4_ * inverse.scalar);
}
fn plane_geometricQuotient_dualNum(self_: Plane, other: DualNum) -> Plane {
    let self_groups = plane_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.scalar, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_groups.group0_
    ));
}
fn plane_geometricQuotient_flector(self_: Plane, other: Flector) -> Plane {
    let self_groups = plane_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.e1_, 2) + pow(other.e2_, 2) + pow(other.e3_, 2) - pow(other.e321_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_groups.group0_
    ));
}
fn plane_geometricQuotient_horizon(self_: Plane, other: Horizon) -> Plane {
    let self_groups = plane_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.e321_, 2) * -1.0);
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_groups.group0_
    ));
}
fn plane_geometricQuotient_line(self_: Plane, other: Line) -> Plane {
    let self_groups = plane_grouped(self_);
    let dot_product: Scalar = Scalar(-pow(other.e23_, 2) - pow(other.e31_, 2) - pow(other.e12_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_groups.group0_
    ));
}
fn plane_geometricQuotient_motor(self_: Plane, other: Motor) -> Plane {
    let self_groups = plane_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.scalar, 2) - pow(other.e23_, 2) - pow(other.e31_, 2) - pow(other.e12_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_groups.group0_
    ));
}
fn plane_geometricQuotient_multiVector(self_: Plane, other: MultiVector) -> Plane {
    let self_groups = plane_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.scalar, 2) + pow(other.e1_, 2) + pow(other.e2_, 2) + pow(other.e3_, 2) - pow(other.e23_, 2) - pow(other.e31_, 2) - pow(other.e12_, 2) - pow(other.e321_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_groups.group0_
    ));
}
fn plane_geometricQuotient_plane(self_: Plane, other: Plane) -> Plane {
    let self_groups = plane_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.e321_, 2) * -1.0);
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_groups.group0_
    ));
}
fn plane_geometricQuotient_point(self_: Plane, other: Point) -> Plane {
    let self_groups = plane_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.e1_, 2) + pow(other.e2_, 2) + pow(other.e3_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_groups.group0_
    ));
}
fn plane_geometricQuotient_scalar(self_: Plane, other: Scalar) -> Plane {
    let self_groups = plane_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.scalar, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_groups.group0_
    ));
}
fn point_geometricQuotient_dualNum(self_: Point, other: DualNum) -> Point {
    let self_groups = point_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.scalar, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_groups.group0_
    ));
}
fn point_geometricQuotient_flector(self_: Point, other: Flector) -> Point {
    let self_groups = point_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.e1_, 2) + pow(other.e2_, 2) + pow(other.e3_, 2) - pow(other.e321_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_groups.group0_
    ));
}
fn point_geometricQuotient_horizon(self_: Point, other: Horizon) -> Point {
    let self_groups = point_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.e321_, 2) * -1.0);
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_groups.group0_
    ));
}
fn point_geometricQuotient_line(self_: Point, other: Line) -> Point {
    let self_groups = point_grouped(self_);
    let dot_product: Scalar = Scalar(-pow(other.e23_, 2) - pow(other.e31_, 2) - pow(other.e12_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_groups.group0_
    ));
}
fn point_geometricQuotient_motor(self_: Point, other: Motor) -> Point {
    let self_groups = point_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.scalar, 2) - pow(other.e23_, 2) - pow(other.e31_, 2) - pow(other.e12_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_groups.group0_
    ));
}
fn point_geometricQuotient_multiVector(self_: Point, other: MultiVector) -> Point {
    let self_groups = point_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.scalar, 2) + pow(other.e1_, 2) + pow(other.e2_, 2) + pow(other.e3_, 2) - pow(other.e23_, 2) - pow(other.e31_, 2) - pow(other.e12_, 2) - pow(other.e321_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_groups.group0_
    ));
}
fn point_geometricQuotient_plane(self_: Point, other: Plane) -> Point {
    let self_groups = point_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.e321_, 2) * -1.0);
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_groups.group0_
    ));
}
fn point_geometricQuotient_point(self_: Point, other: Point) -> Point {
    let self_groups = point_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.e1_, 2) + pow(other.e2_, 2) + pow(other.e3_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_groups.group0_
    ));
}
fn point_geometricQuotient_scalar(self_: Point, other: Scalar) -> Point {
    let self_groups = point_grouped(self_);
    let dot_product: Scalar = Scalar(pow(other.scalar, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_groups.group0_
    ));
}
fn scalar_geometricQuotient_dualNum(self_: Scalar, other: DualNum) -> Scalar {
    let dot_product: Scalar = Scalar(pow(other.scalar, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return Scalar(inverse.scalar * self_.scalar);
}
fn scalar_geometricQuotient_flector(self_: Scalar, other: Flector) -> Scalar {
    let dot_product: Scalar = Scalar(pow(other.e1_, 2) + pow(other.e2_, 2) + pow(other.e3_, 2) - pow(other.e321_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return Scalar(inverse.scalar * self_.scalar);
}
fn scalar_geometricQuotient_horizon(self_: Scalar, other: Horizon) -> Scalar {
    let dot_product: Scalar = Scalar(pow(other.e321_, 2) * -1.0);
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return Scalar(inverse.scalar * self_.scalar);
}
fn scalar_geometricQuotient_line(self_: Scalar, other: Line) -> Scalar {
    let dot_product: Scalar = Scalar(-pow(other.e23_, 2) - pow(other.e31_, 2) - pow(other.e12_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return Scalar(inverse.scalar * self_.scalar);
}
fn scalar_geometricQuotient_motor(self_: Scalar, other: Motor) -> Scalar {
    let dot_product: Scalar = Scalar(pow(other.scalar, 2) - pow(other.e23_, 2) - pow(other.e31_, 2) - pow(other.e12_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return Scalar(inverse.scalar * self_.scalar);
}
fn scalar_geometricQuotient_multiVector(self_: Scalar, other: MultiVector) -> Scalar {
    let dot_product: Scalar = Scalar(pow(other.scalar, 2) + pow(other.e1_, 2) + pow(other.e2_, 2) + pow(other.e3_, 2) - pow(other.e23_, 2) - pow(other.e31_, 2) - pow(other.e12_, 2) - pow(other.e321_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return Scalar(inverse.scalar * self_.scalar);
}
fn scalar_geometricQuotient_plane(self_: Scalar, other: Plane) -> Scalar {
    let dot_product: Scalar = Scalar(pow(other.e321_, 2) * -1.0);
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return Scalar(inverse.scalar * self_.scalar);
}
fn scalar_geometricQuotient_point(self_: Scalar, other: Point) -> Scalar {
    let dot_product: Scalar = Scalar(pow(other.e1_, 2) + pow(other.e2_, 2) + pow(other.e3_, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return Scalar(inverse.scalar * self_.scalar);
}
fn scalar_geometricQuotient_scalar(self_: Scalar, other: Scalar) -> Scalar {
    let dot_product: Scalar = Scalar(pow(other.scalar, 2));
    let inverse: Scalar = Scalar(1.0/(dot_product.scalar));
    return Scalar(inverse.scalar * self_.scalar);
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
fn antiScalar_into_dualNum(self_: AntiScalar) -> DualNum {
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, self_.e1234_, 0.0, 0.0)
    ));
}
fn antiScalar_into_motor(self_: AntiScalar) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, self_.e1234_), 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0)
    ));
}
fn antiScalar_into_multiVector(self_: AntiScalar) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, self_.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn dualNum_into_motor(self_: DualNum) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, self_.e1234_), 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, self_.scalar)
    ));
}
fn dualNum_into_multiVector(self_: DualNum) -> MultiVector {
    let self_groups = dualNum_grouped(self_);    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn flector_into_multiVector(self_: Flector) -> MultiVector {
    let self_groups = flector_grouped(self_);    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ self_groups.group0_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ self_groups.group1_
    ));
}
fn horizon_into_flector(self_: Horizon) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.e321_)
    ));
}
fn horizon_into_multiVector(self_: Horizon) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.e321_)
    ));
}
fn horizon_into_plane(self_: Horizon) -> Plane {
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.e321_)
    ));
}
fn line_into_motor(self_: Line) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.e41_, self_.e42_, self_.e43_, 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.e23_, self_.e31_, self_.e12_, 0.0)
    ));
}
fn line_into_multiVector(self_: Line) -> MultiVector {
    let self_groups = line_grouped(self_);    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ self_groups.group0_, 
        /* e23, e31, e12 */ self_groups.group1_, 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn motor_into_multiVector(self_: Motor) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar, self_.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(self_.e41_, self_.e42_, self_.e43_, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(self_.e23_, self_.e31_, self_.e12_, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn origin_into_flector(self_: Origin) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.e4_), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn origin_into_multiVector(self_: Origin) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.e4_), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn origin_into_point(self_: Origin) -> Point {
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.e4_)
    ));
}
fn plane_into_flector(self_: Plane) -> Flector {
    let self_groups = plane_grouped(self_);    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ self_groups.group0_
    ));
}
fn plane_into_multiVector(self_: Plane) -> MultiVector {
    let self_groups = plane_grouped(self_);    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ self_groups.group0_
    ));
}
fn point_into_flector(self_: Point) -> Flector {
    let self_groups = point_grouped(self_);    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ self_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn point_into_multiVector(self_: Point) -> MultiVector {
    let self_groups = point_grouped(self_);    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ self_groups.group0_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn scalar_into_dualNum(self_: Scalar) -> DualNum {
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar, 0.0, 0.0, 0.0)
    ));
}
fn scalar_into_motor(self_: Scalar) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, self_.scalar)
    ));
}
fn scalar_into_multiVector(self_: Scalar) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn dualNum_inverse(self_: DualNum) -> Scalar {
    let dot_product: Scalar = Scalar(pow(self_.scalar, 2));
    return Scalar(1.0/(dot_product.scalar));
}
fn flector_inverse(self_: Flector) -> Scalar {
    let dot_product: Scalar = Scalar(pow(self_.e1_, 2) + pow(self_.e2_, 2) + pow(self_.e3_, 2) - pow(self_.e321_, 2));
    return Scalar(1.0/(dot_product.scalar));
}
fn horizon_inverse(self_: Horizon) -> Scalar {
    let dot_product: Scalar = Scalar(pow(self_.e321_, 2) * -1.0);
    return Scalar(1.0/(dot_product.scalar));
}
fn line_inverse(self_: Line) -> Scalar {
    let dot_product: Scalar = Scalar(-pow(self_.e23_, 2) - pow(self_.e31_, 2) - pow(self_.e12_, 2));
    return Scalar(1.0/(dot_product.scalar));
}
fn motor_inverse(self_: Motor) -> Scalar {
    let dot_product: Scalar = Scalar(pow(self_.scalar, 2) - pow(self_.e23_, 2) - pow(self_.e31_, 2) - pow(self_.e12_, 2));
    return Scalar(1.0/(dot_product.scalar));
}
fn multiVector_inverse(self_: MultiVector) -> Scalar {
    let dot_product: Scalar = Scalar(pow(self_.scalar, 2) + pow(self_.e1_, 2) + pow(self_.e2_, 2) + pow(self_.e3_, 2) - pow(self_.e23_, 2) - pow(self_.e31_, 2) - pow(self_.e12_, 2) - pow(self_.e321_, 2));
    return Scalar(1.0/(dot_product.scalar));
}
fn plane_inverse(self_: Plane) -> Scalar {
    let dot_product: Scalar = Scalar(pow(self_.e321_, 2) * -1.0);
    return Scalar(1.0/(dot_product.scalar));
}
fn point_inverse(self_: Point) -> Scalar {
    let dot_product: Scalar = Scalar(pow(self_.e1_, 2) + pow(self_.e2_, 2) + pow(self_.e3_, 2));
    return Scalar(1.0/(dot_product.scalar));
}
fn scalar_inverse(self_: Scalar) -> Scalar {
    let dot_product: Scalar = Scalar(pow(self_.scalar, 2));
    return Scalar(1.0/(dot_product.scalar));
}
fn antiScalar_leftComplement(self_: AntiScalar) -> Scalar {
    return Scalar(self_.e1234_);
}
fn dualNum_leftComplement(self_: DualNum) -> DualNum {
    let self_groups = dualNum_grouped(self_);    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ self_groups.group0_.yx
    ));
}
fn flector_leftComplement(self_: Flector) -> Flector {
    let self_groups = flector_grouped(self_);    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ self_groups.group1_, 
        /* e423, e431, e412, e321 */ self_groups.group0_ * vec4<f32>(-1.0)
    ));
}
fn horizon_leftComplement(self_: Horizon) -> Origin {
    return Origin(self_.e321_);
}
fn line_leftComplement(self_: Line) -> Line {
    let self_groups = line_grouped(self_);    return line_degroup(LineGroups(
        /* e41, e42, e43 */ self_groups.group1_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ self_groups.group0_ * vec3<f32>(-1.0)
    ));
}
fn motor_leftComplement(self_: Motor) -> Motor {
    let self_groups = motor_grouped(self_);    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ self_groups.group1_ * vec4<f32>(-1.0, -1.0, -1.0, 1.0), 
        /* e23, e31, e12, scalar */ self_groups.group0_ * vec4<f32>(-1.0, -1.0, -1.0, 1.0)
    ));
}
fn multiVector_leftComplement(self_: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_groups.group0_.yx, 
        /* e1, e2, e3, e4 */ self_groups.group4_, 
        /* e41, e42, e43 */ self_groups.group3_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ self_groups.group2_ * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ self_groups.group1_ * vec4<f32>(-1.0)
    ));
}
fn origin_leftComplement(self_: Origin) -> Horizon {
    return Horizon(self_.e4_ * -1.0);
}
fn plane_leftComplement(self_: Plane) -> Point {
    let self_groups = plane_grouped(self_);    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ self_groups.group0_
    ));
}
fn point_leftComplement(self_: Point) -> Plane {
    let self_groups = point_grouped(self_);    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ self_groups.group0_ * vec4<f32>(-1.0)
    ));
}
fn scalar_leftComplement(self_: Scalar) -> AntiScalar {
    return AntiScalar(self_.scalar);
}
fn antiScalar_mul_dualNum(self_: AntiScalar, other: DualNum) -> AntiScalar {
    return antiScalar_geometricProduct_dualNum(self_, other);
}
fn antiScalar_mul_flector(self_: AntiScalar, other: Flector) -> Flector {
    return antiScalar_geometricProduct_flector(self_, other);
}
fn antiScalar_mul_horizon(self_: AntiScalar, other: Horizon) -> Origin {
    return antiScalar_geometricProduct_horizon(self_, other);
}
fn antiScalar_mul_line(self_: AntiScalar, other: Line) -> Line {
    let other_groups = line_grouped(other);
    return antiScalar_geometricProduct_line(self_, other);
}
fn antiScalar_mul_motor(self_: AntiScalar, other: Motor) -> Motor {
    let other_groups = motor_grouped(other);
    return antiScalar_geometricProduct_motor(self_, other);
}
fn antiScalar_mul_multiVector(self_: AntiScalar, other: MultiVector) -> MultiVector {
    let other_groups = multiVector_grouped(other);
    return antiScalar_geometricProduct_multiVector(self_, other);
}
fn antiScalar_mul_plane(self_: AntiScalar, other: Plane) -> Origin {
    return antiScalar_geometricProduct_plane(self_, other);
}
fn antiScalar_mul_point(self_: AntiScalar, other: Point) -> Plane {
    return antiScalar_geometricProduct_point(self_, other);
}
fn antiScalar_mul_scalar(self_: AntiScalar, other: Scalar) -> AntiScalar {
    return antiScalar_geometricProduct_scalar(self_, other);
}
fn dualNum_mul_antiScalar(self_: DualNum, other: AntiScalar) -> AntiScalar {
    return dualNum_geometricProduct_antiScalar(self_, other);
}
fn dualNum_mul_dualNum(self_: DualNum, other: DualNum) -> DualNum {
    return dualNum_geometricProduct_dualNum(self_, other);
}
fn dualNum_mul_flector(self_: DualNum, other: Flector) -> Flector {
    return dualNum_geometricProduct_flector(self_, other);
}
fn dualNum_mul_horizon(self_: DualNum, other: Horizon) -> Flector {
    return dualNum_geometricProduct_horizon(self_, other);
}
fn dualNum_mul_line(self_: DualNum, other: Line) -> Line {
    let other_groups = line_grouped(other);
    return dualNum_geometricProduct_line(self_, other);
}
fn dualNum_mul_motor(self_: DualNum, other: Motor) -> Motor {
    let other_groups = motor_grouped(other);
    return dualNum_geometricProduct_motor(self_, other);
}
fn dualNum_mul_multiVector(self_: DualNum, other: MultiVector) -> MultiVector {
    let other_groups = multiVector_grouped(other);
    return dualNum_geometricProduct_multiVector(self_, other);
}
fn dualNum_mul_origin(self_: DualNum, other: Origin) -> Origin {
    return dualNum_geometricProduct_origin(self_, other);
}
fn dualNum_mul_plane(self_: DualNum, other: Plane) -> Flector {
    let other_groups = plane_grouped(other);
    return dualNum_geometricProduct_plane(self_, other);
}
fn dualNum_mul_point(self_: DualNum, other: Point) -> Flector {
    let other_groups = point_grouped(other);
    return dualNum_geometricProduct_point(self_, other);
}
fn dualNum_mul_scalar(self_: DualNum, other: Scalar) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    return dualNum_geometricProduct_scalar(self_, other);
}
fn flector_mul_antiScalar(self_: Flector, other: AntiScalar) -> Flector {
    return flector_geometricProduct_antiScalar(self_, other);
}
fn flector_mul_dualNum(self_: Flector, other: DualNum) -> Flector {
    return flector_geometricProduct_dualNum(self_, other);
}
fn flector_mul_flector(self_: Flector, other: Flector) -> Motor {
    return flector_geometricProduct_flector(self_, other);
}
fn flector_mul_horizon(self_: Flector, other: Horizon) -> Motor {
    return flector_geometricProduct_horizon(self_, other);
}
fn flector_mul_line(self_: Flector, other: Line) -> Flector {
    return flector_geometricProduct_line(self_, other);
}
fn flector_mul_motor(self_: Flector, other: Motor) -> Flector {
    let self_groups = flector_grouped(self_);
    return flector_geometricProduct_motor(self_, other);
}
fn flector_mul_multiVector(self_: Flector, other: MultiVector) -> MultiVector {
    let self_groups = flector_grouped(self_);
    return flector_geometricProduct_multiVector(self_, other);
}
fn flector_mul_origin(self_: Flector, other: Origin) -> Motor {
    return flector_geometricProduct_origin(self_, other);
}
fn flector_mul_plane(self_: Flector, other: Plane) -> Motor {
    return flector_geometricProduct_plane(self_, other);
}
fn flector_mul_point(self_: Flector, other: Point) -> Motor {
    return flector_geometricProduct_point(self_, other);
}
fn flector_mul_scalar(self_: Flector, other: Scalar) -> Flector {
    let self_groups = flector_grouped(self_);
    return flector_geometricProduct_scalar(self_, other);
}
fn horizon_mul_antiScalar(self_: Horizon, other: AntiScalar) -> Origin {
    return horizon_geometricProduct_antiScalar(self_, other);
}
fn horizon_mul_dualNum(self_: Horizon, other: DualNum) -> Flector {
    return horizon_geometricProduct_dualNum(self_, other);
}
fn horizon_mul_flector(self_: Horizon, other: Flector) -> Motor {
    return horizon_geometricProduct_flector(self_, other);
}
fn horizon_mul_horizon(self_: Horizon, other: Horizon) -> Scalar {
    return horizon_geometricProduct_horizon(self_, other);
}
fn horizon_mul_line(self_: Horizon, other: Line) -> Flector {
    return horizon_geometricProduct_line(self_, other);
}
fn horizon_mul_motor(self_: Horizon, other: Motor) -> Flector {
    return horizon_geometricProduct_motor(self_, other);
}
fn horizon_mul_multiVector(self_: Horizon, other: MultiVector) -> MultiVector {
    return horizon_geometricProduct_multiVector(self_, other);
}
fn horizon_mul_origin(self_: Horizon, other: Origin) -> AntiScalar {
    return horizon_geometricProduct_origin(self_, other);
}
fn horizon_mul_plane(self_: Horizon, other: Plane) -> Motor {
    return horizon_geometricProduct_plane(self_, other);
}
fn horizon_mul_point(self_: Horizon, other: Point) -> Motor {
    return horizon_geometricProduct_point(self_, other);
}
fn horizon_mul_scalar(self_: Horizon, other: Scalar) -> Horizon {
    return horizon_geometricProduct_scalar(self_, other);
}
fn line_mul_antiScalar(self_: Line, other: AntiScalar) -> Line {
    let self_groups = line_grouped(self_);
    return line_geometricProduct_antiScalar(self_, other);
}
fn line_mul_dualNum(self_: Line, other: DualNum) -> Line {
    let self_groups = line_grouped(self_);
    return line_geometricProduct_dualNum(self_, other);
}
fn line_mul_flector(self_: Line, other: Flector) -> Flector {
    return line_geometricProduct_flector(self_, other);
}
fn line_mul_horizon(self_: Line, other: Horizon) -> Flector {
    return line_geometricProduct_horizon(self_, other);
}
fn line_mul_line(self_: Line, other: Line) -> Motor {
    return line_geometricProduct_line(self_, other);
}
fn line_mul_motor(self_: Line, other: Motor) -> Motor {
    return line_geometricProduct_motor(self_, other);
}
fn line_mul_multiVector(self_: Line, other: MultiVector) -> MultiVector {
    let self_groups = line_grouped(self_);
    return line_geometricProduct_multiVector(self_, other);
}
fn line_mul_origin(self_: Line, other: Origin) -> Plane {
    return line_geometricProduct_origin(self_, other);
}
fn line_mul_plane(self_: Line, other: Plane) -> Flector {
    return line_geometricProduct_plane(self_, other);
}
fn line_mul_point(self_: Line, other: Point) -> Flector {
    return line_geometricProduct_point(self_, other);
}
fn line_mul_scalar(self_: Line, other: Scalar) -> Line {
    let self_groups = line_grouped(self_);
    return line_geometricProduct_scalar(self_, other);
}
fn motor_mul_antiScalar(self_: Motor, other: AntiScalar) -> Motor {
    let self_groups = motor_grouped(self_);
    return motor_geometricProduct_antiScalar(self_, other);
}
fn motor_mul_dualNum(self_: Motor, other: DualNum) -> Motor {
    let self_groups = motor_grouped(self_);
    return motor_geometricProduct_dualNum(self_, other);
}
fn motor_mul_flector(self_: Motor, other: Flector) -> Flector {
    let other_groups = flector_grouped(other);
    return motor_geometricProduct_flector(self_, other);
}
fn motor_mul_horizon(self_: Motor, other: Horizon) -> Flector {
    return motor_geometricProduct_horizon(self_, other);
}
fn motor_mul_line(self_: Motor, other: Line) -> Motor {
    return motor_geometricProduct_line(self_, other);
}
fn motor_mul_motor(self_: Motor, other: Motor) -> Motor {
    let other_groups = motor_grouped(other);
    return motor_geometricProduct_motor(self_, other);
}
fn motor_mul_multiVector(self_: Motor, other: MultiVector) -> MultiVector {
    let other_groups = multiVector_grouped(other);
    return motor_geometricProduct_multiVector(self_, other);
}
fn motor_mul_origin(self_: Motor, other: Origin) -> Flector {
    return motor_geometricProduct_origin(self_, other);
}
fn motor_mul_plane(self_: Motor, other: Plane) -> Flector {
    return motor_geometricProduct_plane(self_, other);
}
fn motor_mul_point(self_: Motor, other: Point) -> Flector {
    let other_groups = point_grouped(other);
    return motor_geometricProduct_point(self_, other);
}
fn motor_mul_scalar(self_: Motor, other: Scalar) -> Motor {
    let self_groups = motor_grouped(self_);
    return motor_geometricProduct_scalar(self_, other);
}
fn multiVector_mul_antiScalar(self_: MultiVector, other: AntiScalar) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    return multiVector_geometricProduct_antiScalar(self_, other);
}
fn multiVector_mul_dualNum(self_: MultiVector, other: DualNum) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    return multiVector_geometricProduct_dualNum(self_, other);
}
fn multiVector_mul_flector(self_: MultiVector, other: Flector) -> MultiVector {
    let other_groups = flector_grouped(other);
    return multiVector_geometricProduct_flector(self_, other);
}
fn multiVector_mul_horizon(self_: MultiVector, other: Horizon) -> MultiVector {
    return multiVector_geometricProduct_horizon(self_, other);
}
fn multiVector_mul_line(self_: MultiVector, other: Line) -> MultiVector {
    let other_groups = line_grouped(other);
    return multiVector_geometricProduct_line(self_, other);
}
fn multiVector_mul_motor(self_: MultiVector, other: Motor) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    return multiVector_geometricProduct_motor(self_, other);
}
fn multiVector_mul_multiVector(self_: MultiVector, other: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_geometricProduct_multiVector(self_, other);
}
fn multiVector_mul_origin(self_: MultiVector, other: Origin) -> MultiVector {
    return multiVector_geometricProduct_origin(self_, other);
}
fn multiVector_mul_plane(self_: MultiVector, other: Plane) -> MultiVector {
    return multiVector_geometricProduct_plane(self_, other);
}
fn multiVector_mul_point(self_: MultiVector, other: Point) -> MultiVector {
    let other_groups = point_grouped(other);
    return multiVector_geometricProduct_point(self_, other);
}
fn multiVector_mul_scalar(self_: MultiVector, other: Scalar) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    return multiVector_geometricProduct_scalar(self_, other);
}
fn origin_mul_dualNum(self_: Origin, other: DualNum) -> Origin {
    return origin_geometricProduct_dualNum(self_, other);
}
fn origin_mul_flector(self_: Origin, other: Flector) -> Motor {
    return origin_geometricProduct_flector(self_, other);
}
fn origin_mul_horizon(self_: Origin, other: Horizon) -> AntiScalar {
    return origin_geometricProduct_horizon(self_, other);
}
fn origin_mul_line(self_: Origin, other: Line) -> Plane {
    return origin_geometricProduct_line(self_, other);
}
fn origin_mul_motor(self_: Origin, other: Motor) -> Flector {
    return origin_geometricProduct_motor(self_, other);
}
fn origin_mul_multiVector(self_: Origin, other: MultiVector) -> MultiVector {
    return origin_geometricProduct_multiVector(self_, other);
}
fn origin_mul_plane(self_: Origin, other: Plane) -> AntiScalar {
    return origin_geometricProduct_plane(self_, other);
}
fn origin_mul_point(self_: Origin, other: Point) -> Line {
    return origin_geometricProduct_point(self_, other);
}
fn origin_mul_scalar(self_: Origin, other: Scalar) -> Origin {
    return origin_geometricProduct_scalar(self_, other);
}
fn plane_mul_antiScalar(self_: Plane, other: AntiScalar) -> Origin {
    return plane_geometricProduct_antiScalar(self_, other);
}
fn plane_mul_dualNum(self_: Plane, other: DualNum) -> Flector {
    let self_groups = plane_grouped(self_);
    return plane_geometricProduct_dualNum(self_, other);
}
fn plane_mul_flector(self_: Plane, other: Flector) -> Motor {
    return plane_geometricProduct_flector(self_, other);
}
fn plane_mul_horizon(self_: Plane, other: Horizon) -> Motor {
    return plane_geometricProduct_horizon(self_, other);
}
fn plane_mul_line(self_: Plane, other: Line) -> Flector {
    return plane_geometricProduct_line(self_, other);
}
fn plane_mul_motor(self_: Plane, other: Motor) -> Flector {
    return plane_geometricProduct_motor(self_, other);
}
fn plane_mul_multiVector(self_: Plane, other: MultiVector) -> MultiVector {
    return plane_geometricProduct_multiVector(self_, other);
}
fn plane_mul_origin(self_: Plane, other: Origin) -> AntiScalar {
    return plane_geometricProduct_origin(self_, other);
}
fn plane_mul_plane(self_: Plane, other: Plane) -> Motor {
    return plane_geometricProduct_plane(self_, other);
}
fn plane_mul_point(self_: Plane, other: Point) -> Motor {
    return plane_geometricProduct_point(self_, other);
}
fn plane_mul_scalar(self_: Plane, other: Scalar) -> Plane {
    let self_groups = plane_grouped(self_);
    return plane_geometricProduct_scalar(self_, other);
}
fn point_mul_antiScalar(self_: Point, other: AntiScalar) -> Plane {
    return point_geometricProduct_antiScalar(self_, other);
}
fn point_mul_dualNum(self_: Point, other: DualNum) -> Flector {
    let self_groups = point_grouped(self_);
    return point_geometricProduct_dualNum(self_, other);
}
fn point_mul_flector(self_: Point, other: Flector) -> Motor {
    return point_geometricProduct_flector(self_, other);
}
fn point_mul_horizon(self_: Point, other: Horizon) -> Motor {
    return point_geometricProduct_horizon(self_, other);
}
fn point_mul_line(self_: Point, other: Line) -> Flector {
    return point_geometricProduct_line(self_, other);
}
fn point_mul_motor(self_: Point, other: Motor) -> Flector {
    let self_groups = point_grouped(self_);
    return point_geometricProduct_motor(self_, other);
}
fn point_mul_multiVector(self_: Point, other: MultiVector) -> MultiVector {
    let self_groups = point_grouped(self_);
    return point_geometricProduct_multiVector(self_, other);
}
fn point_mul_origin(self_: Point, other: Origin) -> Line {
    return point_geometricProduct_origin(self_, other);
}
fn point_mul_plane(self_: Point, other: Plane) -> Motor {
    return point_geometricProduct_plane(self_, other);
}
fn point_mul_point(self_: Point, other: Point) -> Motor {
    return point_geometricProduct_point(self_, other);
}
fn point_mul_scalar(self_: Point, other: Scalar) -> Point {
    let self_groups = point_grouped(self_);
    return point_geometricProduct_scalar(self_, other);
}
fn scalar_mul_antiScalar(self_: Scalar, other: AntiScalar) -> AntiScalar {
    return scalar_geometricProduct_antiScalar(self_, other);
}
fn scalar_mul_dualNum(self_: Scalar, other: DualNum) -> DualNum {
    let other_groups = dualNum_grouped(other);
    return scalar_geometricProduct_dualNum(self_, other);
}
fn scalar_mul_flector(self_: Scalar, other: Flector) -> Flector {
    let other_groups = flector_grouped(other);
    return scalar_geometricProduct_flector(self_, other);
}
fn scalar_mul_horizon(self_: Scalar, other: Horizon) -> Horizon {
    return scalar_geometricProduct_horizon(self_, other);
}
fn scalar_mul_line(self_: Scalar, other: Line) -> Line {
    let other_groups = line_grouped(other);
    return scalar_geometricProduct_line(self_, other);
}
fn scalar_mul_motor(self_: Scalar, other: Motor) -> Motor {
    let other_groups = motor_grouped(other);
    return scalar_geometricProduct_motor(self_, other);
}
fn scalar_mul_multiVector(self_: Scalar, other: MultiVector) -> MultiVector {
    let other_groups = multiVector_grouped(other);
    return scalar_geometricProduct_multiVector(self_, other);
}
fn scalar_mul_origin(self_: Scalar, other: Origin) -> Origin {
    return scalar_geometricProduct_origin(self_, other);
}
fn scalar_mul_plane(self_: Scalar, other: Plane) -> Plane {
    let other_groups = plane_grouped(other);
    return scalar_geometricProduct_plane(self_, other);
}
fn scalar_mul_point(self_: Scalar, other: Point) -> Point {
    let other_groups = point_grouped(other);
    return scalar_geometricProduct_point(self_, other);
}
fn scalar_mul_scalar(self_: Scalar, other: Scalar) -> Scalar {
    return scalar_geometricProduct_scalar(self_, other);
}
fn antiScalar_neg(self_: AntiScalar) -> AntiScalar {
    return AntiScalar(self_.e1234_ * -1.0);
}
fn dualNum_neg(self_: DualNum) -> DualNum {
    let self_groups = dualNum_grouped(self_);    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ self_groups.group0_ * vec2<f32>(-1.0)
    ));
}
fn flector_neg(self_: Flector) -> Flector {
    let self_groups = flector_grouped(self_);    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ self_groups.group0_ * vec4<f32>(-1.0), 
        /* e423, e431, e412, e321 */ self_groups.group1_ * vec4<f32>(-1.0)
    ));
}
fn horizon_neg(self_: Horizon) -> Horizon {
    return Horizon(self_.e321_ * -1.0);
}
fn line_neg(self_: Line) -> Line {
    let self_groups = line_grouped(self_);    return line_degroup(LineGroups(
        /* e41, e42, e43 */ self_groups.group0_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ self_groups.group1_ * vec3<f32>(-1.0)
    ));
}
fn motor_neg(self_: Motor) -> Motor {
    let self_groups = motor_grouped(self_);    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ self_groups.group0_ * vec4<f32>(-1.0), 
        /* e23, e31, e12, scalar */ self_groups.group1_ * vec4<f32>(-1.0)
    ));
}
fn multiVector_neg(self_: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_groups.group0_ * vec2<f32>(-1.0), 
        /* e1, e2, e3, e4 */ self_groups.group1_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ self_groups.group2_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ self_groups.group3_ * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ self_groups.group4_ * vec4<f32>(-1.0)
    ));
}
fn origin_neg(self_: Origin) -> Origin {
    return Origin(self_.e4_ * -1.0);
}
fn plane_neg(self_: Plane) -> Plane {
    let self_groups = plane_grouped(self_);    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ self_groups.group0_ * vec4<f32>(-1.0)
    ));
}
fn point_neg(self_: Point) -> Point {
    let self_groups = point_grouped(self_);    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ self_groups.group0_ * vec4<f32>(-1.0)
    ));
}
fn scalar_neg(self_: Scalar) -> Scalar {
    return Scalar(self_.scalar * -1.0);
}
fn dualNum_one() -> DualNum {
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(1.0, 0.0, 0.0, 0.0)
    ));
}
fn motor_one() -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, 1.0)
    ));
}
fn multiVector_one() -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(1.0, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn scalar_one() -> Scalar {
    return Scalar(1.0);
}
fn antiScalar_reverse(self_: AntiScalar) -> AntiScalar {
    return self_;
}
fn dualNum_reverse(self_: DualNum) -> DualNum {
    return self_;
}
fn flector_reverse(self_: Flector) -> Flector {
    let self_groups = flector_grouped(self_);    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ self_groups.group0_, 
        /* e423, e431, e412, e321 */ self_groups.group1_ * vec4<f32>(-1.0)
    ));
}
fn horizon_reverse(self_: Horizon) -> Horizon {
    return Horizon(self_.e321_ * -1.0);
}
fn line_reverse(self_: Line) -> Line {
    let self_groups = line_grouped(self_);    return line_degroup(LineGroups(
        /* e41, e42, e43 */ self_groups.group0_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ self_groups.group1_ * vec3<f32>(-1.0)
    ));
}
fn motor_reverse(self_: Motor) -> Motor {
    let self_groups = motor_grouped(self_);    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ self_groups.group0_ * vec4<f32>(-1.0, -1.0, -1.0, 1.0), 
        /* e23, e31, e12, scalar */ self_groups.group1_ * vec4<f32>(-1.0, -1.0, -1.0, 1.0)
    ));
}
fn multiVector_reverse(self_: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_groups.group0_, 
        /* e1, e2, e3, e4 */ self_groups.group1_, 
        /* e41, e42, e43 */ self_groups.group2_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ self_groups.group3_ * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ self_groups.group4_ * vec4<f32>(-1.0)
    ));
}
fn origin_reverse(self_: Origin) -> Origin {
    return self_;
}
fn plane_reverse(self_: Plane) -> Plane {
    let self_groups = plane_grouped(self_);    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ self_groups.group0_ * vec4<f32>(-1.0)
    ));
}
fn point_reverse(self_: Point) -> Point {
    return self_;
}
fn scalar_reverse(self_: Scalar) -> Scalar {
    return self_;
}
fn dualNum_rightAntiDual(self_: DualNum) -> AntiScalar {
    return AntiScalar(self_.scalar);
}
fn flector_rightAntiDual(self_: Flector) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, self_.e321_) * vec4<f32>(0.0, 0.0, 0.0, -1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0)
    ));
}
fn horizon_rightAntiDual(self_: Horizon) -> Origin {
    return Origin(self_.e321_ * -1.0);
}
fn line_rightAntiDual(self_: Line) -> Line {
    let self_groups = line_grouped(self_);    return line_degroup(LineGroups(
        /* e41, e42, e43 */ self_groups.group1_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0)
    ));
}
fn motor_rightAntiDual(self_: Motor) -> Motor {
    let self_groups = motor_grouped(self_);    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ self_groups.group1_ * vec4<f32>(-1.0, -1.0, -1.0, 1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0)
    ));
}
fn multiVector_rightAntiDual(self_: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, self_.scalar, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, self_.e321_) * vec4<f32>(0.0, 0.0, 0.0, -1.0), 
        /* e41, e42, e43 */ self_groups.group3_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0)
    ));
}
fn plane_rightAntiDual(self_: Plane) -> Origin {
    return Origin(self_.e321_ * -1.0);
}
fn point_rightAntiDual(self_: Point) -> Plane {
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0)
    ));
}
fn scalar_rightAntiDual(self_: Scalar) -> AntiScalar {
    return AntiScalar(self_.scalar);
}
fn antiScalar_rightComplement(self_: AntiScalar) -> Scalar {
    return Scalar(self_.e1234_);
}
fn dualNum_rightComplement(self_: DualNum) -> DualNum {
    let self_groups = dualNum_grouped(self_);    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ self_groups.group0_.yx
    ));
}
fn flector_rightComplement(self_: Flector) -> Flector {
    let self_groups = flector_grouped(self_);    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ self_groups.group1_ * vec4<f32>(-1.0), 
        /* e423, e431, e412, e321 */ self_groups.group0_
    ));
}
fn horizon_rightComplement(self_: Horizon) -> Origin {
    return Origin(self_.e321_ * -1.0);
}
fn line_rightComplement(self_: Line) -> Line {
    let self_groups = line_grouped(self_);    return line_degroup(LineGroups(
        /* e41, e42, e43 */ self_groups.group1_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ self_groups.group0_ * vec3<f32>(-1.0)
    ));
}
fn motor_rightComplement(self_: Motor) -> Motor {
    let self_groups = motor_grouped(self_);    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ self_groups.group1_ * vec4<f32>(-1.0, -1.0, -1.0, 1.0), 
        /* e23, e31, e12, scalar */ self_groups.group0_ * vec4<f32>(-1.0, -1.0, -1.0, 1.0)
    ));
}
fn multiVector_rightComplement(self_: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_groups.group0_.yx, 
        /* e1, e2, e3, e4 */ self_groups.group4_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ self_groups.group3_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ self_groups.group2_ * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ self_groups.group1_
    ));
}
fn origin_rightComplement(self_: Origin) -> Horizon {
    return Horizon(self_.e4_);
}
fn plane_rightComplement(self_: Plane) -> Point {
    let self_groups = plane_grouped(self_);    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ self_groups.group0_ * vec4<f32>(-1.0)
    ));
}
fn point_rightComplement(self_: Point) -> Plane {
    let self_groups = point_grouped(self_);    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ self_groups.group0_
    ));
}
fn scalar_rightComplement(self_: Scalar) -> AntiScalar {
    return AntiScalar(self_.scalar);
}
fn dualNum_rightDual(self_: DualNum) -> AntiScalar {
    return AntiScalar(self_.scalar);
}
fn flector_rightDual(self_: Flector) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, self_.e321_) * vec4<f32>(0.0, 0.0, 0.0, -1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0)
    ));
}
fn horizon_rightDual(self_: Horizon) -> Origin {
    return Origin(self_.e321_ * -1.0);
}
fn line_rightDual(self_: Line) -> Line {
    let self_groups = line_grouped(self_);    return line_degroup(LineGroups(
        /* e41, e42, e43 */ self_groups.group1_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0)
    ));
}
fn motor_rightDual(self_: Motor) -> Motor {
    let self_groups = motor_grouped(self_);    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ self_groups.group1_ * vec4<f32>(-1.0, -1.0, -1.0, 1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0)
    ));
}
fn multiVector_rightDual(self_: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, self_.scalar, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, self_.e321_) * vec4<f32>(0.0, 0.0, 0.0, -1.0), 
        /* e41, e42, e43 */ self_groups.group3_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0)
    ));
}
fn plane_rightDual(self_: Plane) -> Origin {
    return Origin(self_.e321_ * -1.0);
}
fn point_rightDual(self_: Point) -> Plane {
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0)
    ));
}
fn scalar_rightDual(self_: Scalar) -> AntiScalar {
    return AntiScalar(self_.scalar);
}
fn antiScalar_sandwich_flector(self_: AntiScalar, other: Flector) -> Flector {
    let geometric_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, self_.e1234_ * other.e321_) * vec4<f32>(0.0, 0.0, 0.0, -1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e1234_ * other.e1_, self_.e1234_ * other.e2_, self_.e1234_ * other.e3_, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0)
    ));
    return flector_geometricProduct_antiScalar(geometric_product, antiScalar_reverse(self_));
}
fn antiScalar_sandwich_line(self_: AntiScalar, other: Line) -> Line {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = line_grouped(other);
    let geometric_product: Line = line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group1_, 
        /* e23, e31, e12 */ vec4<f32>(0.0)
    ));
    return line_geometricProduct_antiScalar(geometric_product, antiScalar_reverse(self_));
}
fn antiScalar_sandwich_motor(self_: AntiScalar, other: Motor) -> Motor {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = motor_grouped(other);
    let geometric_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.e1234_) * other_groups.group1_, 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0)
    ));
    return motor_geometricProduct_antiScalar(geometric_product, antiScalar_reverse(self_));
}
fn antiScalar_sandwich_multiVector(self_: AntiScalar, other: MultiVector) -> MultiVector {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let geometric_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(1.0, self_.e1234_ * other.scalar, 0.0, 0.0) * vec2<f32>(0.0, 1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, self_.e1234_ * other.e321_) * vec4<f32>(0.0, 0.0, 0.0, -1.0), 
        /* e41, e42, e43 */ (vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group3_, 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e1234_ * other.e1_, self_.e1234_ * other.e2_, self_.e1234_ * other.e3_, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0)
    ));
    return multiVector_geometricProduct_antiScalar(geometric_product, antiScalar_reverse(self_));
}
fn antiScalar_sandwich_point(self_: AntiScalar, other: Point) -> Origin {
    let geometric_product: Plane = plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e1234_ * other.e1_, self_.e1234_ * other.e2_, self_.e1234_ * other.e3_, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0)
    ));
    return plane_geometricProduct_antiScalar(geometric_product, antiScalar_reverse(self_));
}
fn dualNum_sandwich_antiScalar(self_: DualNum, other: AntiScalar) -> AntiScalar {
    let geometric_product: AntiScalar = AntiScalar(other.e1234_ * self_.scalar);
    return antiScalar_geometricProduct_dualNum(geometric_product, dualNum_reverse(self_));
}
fn dualNum_sandwich_dualNum(self_: DualNum, other: DualNum) -> DualNum {
    let geometric_product: DualNum = dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar * self_.scalar, (other.scalar * self_.e1234_) + (other.e1234_ * self_.scalar), 0.0, 0.0)
    ));
    return dualNum_geometricProduct_dualNum(geometric_product, dualNum_reverse(self_));
}
fn dualNum_sandwich_flector(self_: DualNum, other: Flector) -> Flector {
    let geometric_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.scalar * other.e1_, self_.scalar * other.e2_, self_.scalar * other.e3_, (self_.scalar * other.e4_) - (self_.e1234_ * other.e321_)), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.scalar * other.e423_) - (self_.e1234_ * other.e1_), (self_.scalar * other.e431_) - (self_.e1234_ * other.e2_), (self_.scalar * other.e412_) - (self_.e1234_ * other.e3_), self_.scalar * other.e321_)
    ));
    return flector_geometricProduct_dualNum(geometric_product, dualNum_reverse(self_));
}
fn dualNum_sandwich_horizon(self_: DualNum, other: Horizon) -> Flector {
    let geometric_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, self_.e1234_ * other.e321_) * vec4<f32>(0.0, 0.0, 0.0, -1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0, 1.0, 1.0, self_.scalar * other.e321_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)
    ));
    return flector_geometricProduct_dualNum(geometric_product, dualNum_reverse(self_));
}
fn dualNum_sandwich_line(self_: DualNum, other: Line) -> Line {
    let self_groups = dualNum_grouped(self_);
    let other_groups = line_grouped(other);
    let geometric_product: Line = line_degroup(LineGroups(
        /* e41, e42, e43 */ ((vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group0_) + ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group1_), 
        /* e23, e31, e12 */ (vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group1_
    ));
    return line_geometricProduct_dualNum(geometric_product, dualNum_reverse(self_));
}
fn dualNum_sandwich_motor(self_: DualNum, other: Motor) -> Motor {
    let self_groups = dualNum_grouped(self_);
    let other_groups = motor_grouped(other);
    let geometric_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ (vec4<f32>(self_.scalar) * other_groups.group0_) + (vec4<f32>(self_.e1234_) * other_groups.group1_), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.scalar) * other_groups.group1_
    ));
    return motor_geometricProduct_dualNum(geometric_product, dualNum_reverse(self_));
}
fn dualNum_sandwich_multiVector(self_: DualNum, other: MultiVector) -> MultiVector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let geometric_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar * other.scalar, (self_.scalar * other.e1234_) + (self_.e1234_ * other.scalar), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.scalar * other.e1_, self_.scalar * other.e2_, self_.scalar * other.e3_, (self_.scalar * other.e4_) - (self_.e1234_ * other.e321_)), 
        /* e41, e42, e43 */ ((vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group2_) + ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group3_), 
        /* e23, e31, e12 */ (vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.scalar * other.e423_) - (self_.e1234_ * other.e1_), (self_.scalar * other.e431_) - (self_.e1234_ * other.e2_), (self_.scalar * other.e412_) - (self_.e1234_ * other.e3_), self_.scalar * other.e321_)
    ));
    return multiVector_geometricProduct_dualNum(geometric_product, dualNum_reverse(self_));
}
fn dualNum_sandwich_origin(self_: DualNum, other: Origin) -> Origin {
    let geometric_product: Origin = Origin(self_.scalar * other.e4_);
    return origin_geometricProduct_dualNum(geometric_product, dualNum_reverse(self_));
}
fn dualNum_sandwich_plane(self_: DualNum, other: Plane) -> Flector {
    let other_groups = plane_grouped(other);
    let geometric_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, self_.e1234_ * other.e321_) * vec4<f32>(0.0, 0.0, 0.0, -1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.scalar) * other_groups.group0_
    ));
    return flector_geometricProduct_dualNum(geometric_product, dualNum_reverse(self_));
}
fn dualNum_sandwich_point(self_: DualNum, other: Point) -> Flector {
    let other_groups = point_grouped(other);
    let geometric_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.scalar) * other_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e1234_ * other.e1_, self_.e1234_ * other.e2_, self_.e1234_ * other.e3_, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0)
    ));
    return flector_geometricProduct_dualNum(geometric_product, dualNum_reverse(self_));
}
fn dualNum_sandwich_scalar(self_: DualNum, other: Scalar) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let geometric_product: DualNum = dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ (vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_
    ));
    return dualNum_geometricProduct_dualNum(geometric_product, dualNum_reverse(self_));
}
fn flector_sandwich_antiScalar(self_: Flector, other: AntiScalar) -> Motor {
    let self_groups = flector_grouped(self_);
    let geometric_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, other.e1234_ * self_.e321_) * vec4<f32>(0.0, 0.0, 0.0, 1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e1234_ * self_.e1_, other.e1234_ * self_.e2_, other.e1234_ * self_.e3_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
    return flector_geometricProduct_flector(geometric_product, flector_reverse(self_));
}
fn flector_sandwich_dualNum(self_: Flector, other: DualNum) -> Motor {
    let self_groups = flector_grouped(self_);
    let geometric_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.scalar * self_.e1_, other.scalar * self_.e2_, other.scalar * self_.e3_, (other.scalar * self_.e4_) + (other.e1234_ * self_.e321_)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.scalar * self_.e423_) + (other.e1234_ * self_.e1_), (other.scalar * self_.e431_) + (other.e1234_ * self_.e2_), (other.scalar * self_.e412_) + (other.e1234_ * self_.e3_), other.scalar * self_.e321_)
    ));
    return flector_geometricProduct_flector(geometric_product, flector_reverse(self_));
}
fn flector_sandwich_flector(self_: Flector, other: Flector) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = flector_grouped(other);
    let geometric_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other.e2_ * self_.e412_) + (other.e412_ * self_.e2_) + (other.e321_ * self_.e423_) - (other.e3_ * self_.e431_) - (other.e423_ * self_.e321_) - (other.e431_ * self_.e3_), (other.e3_ * self_.e423_) + (other.e423_ * self_.e3_) + (other.e321_ * self_.e431_) - (other.e1_ * self_.e412_) - (other.e431_ * self_.e321_) - (other.e412_ * self_.e1_), (other.e1_ * self_.e431_) + (other.e431_ * self_.e1_) + (other.e321_ * self_.e412_) - (other.e2_ * self_.e423_) - (other.e423_ * self_.e2_) - (other.e412_ * self_.e321_), (other.e423_ * self_.e1_) + (other.e431_ * self_.e2_) + (other.e412_ * self_.e3_) - (other.e1_ * self_.e423_) - (other.e2_ * self_.e431_) - (other.e3_ * self_.e412_)) + (vec4<f32>(self_.e4_) * vec4<f32>(other.e1_, other.e2_, other.e3_, other.e321_)) - (vec4<f32>(other.e4_) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, self_.e321_)), 
        /* e23, e31, e12, scalar */ vec4<f32>((other.e3_ * self_.e2_) - (other.e2_ * self_.e3_) - (other.e321_ * self_.e1_), (other.e1_ * self_.e3_) - (other.e3_ * self_.e1_) - (other.e321_ * self_.e2_), (other.e2_ * self_.e1_) - (other.e1_ * self_.e2_) - (other.e321_ * self_.e3_), (other.e1_ * self_.e1_) + (other.e2_ * self_.e2_) + (other.e3_ * self_.e3_)) - (vec4<f32>(self_.e321_) * vec4<f32>(other.e1_, other.e2_, other.e3_, other.e321_))
    ));
    return motor_geometricProduct_flector(geometric_product, flector_reverse(self_));
}
fn flector_sandwich_horizon(self_: Flector, other: Horizon) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = horizon_grouped(other);
    let geometric_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e321_) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, self_.e4_), 
        /* e23, e31, e12, scalar */ vec4<f32>(other.e321_) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, self_.e321_) * vec4<f32>(-1.0)
    ));
    return motor_geometricProduct_flector(geometric_product, flector_reverse(self_));
}
fn flector_sandwich_line(self_: Flector, other: Line) -> Motor {
    let self_groups = flector_grouped(self_);
    let geometric_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e3_ * other.e31_) + (self_.e321_ * other.e23_) - (self_.e2_ * other.e12_), (self_.e1_ * other.e12_) + (self_.e321_ * other.e31_) - (self_.e3_ * other.e23_), (self_.e2_ * other.e23_) + (self_.e321_ * other.e12_) - (self_.e1_ * other.e31_), -(self_.e1_ * other.e41_) - (self_.e2_ * other.e42_) - (self_.e3_ * other.e43_) - (self_.e423_ * other.e23_) - (self_.e431_ * other.e31_) - (self_.e412_ * other.e12_)), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e3_ * other.e42_) + (self_.e4_ * other.e23_) + (self_.e412_ * other.e31_) + (self_.e321_ * other.e41_) - (self_.e2_ * other.e43_) - (self_.e431_ * other.e12_), (self_.e1_ * other.e43_) + (self_.e4_ * other.e31_) + (self_.e423_ * other.e12_) + (self_.e321_ * other.e42_) - (self_.e3_ * other.e41_) - (self_.e412_ * other.e23_), (self_.e2_ * other.e41_) + (self_.e4_ * other.e12_) + (self_.e431_ * other.e23_) + (self_.e321_ * other.e43_) - (self_.e1_ * other.e42_) - (self_.e423_ * other.e31_), -(self_.e1_ * other.e23_) - (self_.e2_ * other.e31_) - (self_.e3_ * other.e12_))
    ));
    return flector_geometricProduct_flector(geometric_product, flector_reverse(self_));
}
fn flector_sandwich_motor(self_: Flector, other: Motor) -> Motor {
    let self_groups = flector_grouped(self_);
    let geometric_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e3_ * other.e31_) - (self_.e2_ * other.e12_), (self_.e1_ * other.e12_) - (self_.e3_ * other.e23_), (self_.e2_ * other.e23_) - (self_.e1_ * other.e31_), -(self_.e1_ * other.e41_) - (self_.e2_ * other.e42_) - (self_.e3_ * other.e43_) - (self_.e423_ * other.e23_) - (self_.e431_ * other.e31_) - (self_.e412_ * other.e12_)) + (vec4<f32>(self_.e321_) * vec4<f32>(other.e23_, other.e31_, other.e12_, other.e1234_)) + (vec4<f32>(other.scalar) * self_groups.group0_), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e1_ * other.e1234_) + (self_.e3_ * other.e42_) + (self_.e4_ * other.e23_) + (self_.e412_ * other.e31_) + (self_.e321_ * other.e41_) - (self_.e2_ * other.e43_) - (self_.e431_ * other.e12_), (self_.e1_ * other.e43_) + (self_.e2_ * other.e1234_) + (self_.e4_ * other.e31_) + (self_.e423_ * other.e12_) + (self_.e321_ * other.e42_) - (self_.e3_ * other.e41_) - (self_.e412_ * other.e23_), (self_.e2_ * other.e41_) + (self_.e3_ * other.e1234_) + (self_.e4_ * other.e12_) + (self_.e431_ * other.e23_) + (self_.e321_ * other.e43_) - (self_.e1_ * other.e42_) - (self_.e423_ * other.e31_), -(self_.e1_ * other.e23_) - (self_.e2_ * other.e31_) - (self_.e3_ * other.e12_)) + (vec4<f32>(other.scalar) * self_groups.group1_)
    ));
    return flector_geometricProduct_flector(geometric_product, flector_reverse(self_));
}
fn flector_sandwich_multiVector(self_: Flector, other: MultiVector) -> MultiVector {
    let self_groups = flector_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let geometric_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, (self_.e4_ * other.e321_) - (self_.e423_ * other.e1_) - (self_.e431_ * other.e2_) - (self_.e412_ * other.e3_), 0.0, 0.0) + ((vec4<f32>(self_.e1_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e1_, other.e423_, 0.0, 0.0)) + ((vec4<f32>(self_.e2_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e2_, other.e431_, 0.0, 0.0)) + ((vec4<f32>(self_.e3_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e3_, other.e412_, 0.0, 0.0)) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e321_, other.e4_, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e3_ * other.e31_) - (self_.e2_ * other.e12_), (self_.e1_ * other.e12_) - (self_.e3_ * other.e23_), (self_.e2_ * other.e23_) - (self_.e1_ * other.e31_), -(self_.e1_ * other.e41_) - (self_.e2_ * other.e42_) - (self_.e3_ * other.e43_) - (self_.e423_ * other.e23_) - (self_.e431_ * other.e31_) - (self_.e412_ * other.e12_)) + (vec4<f32>(self_.e321_) * vec4<f32>(other.e23_, other.e31_, other.e12_, other.e1234_)) + (vec4<f32>(other.scalar) * self_groups.group0_), 
        /* e41, e42, e43 */ vec4<f32>((self_.e2_ * other.e412_) + (self_.e412_ * other.e2_) - (self_.e3_ * other.e431_) - (self_.e431_ * other.e3_), (self_.e3_ * other.e423_) + (self_.e423_ * other.e3_) - (self_.e1_ * other.e412_) - (self_.e412_ * other.e1_), (self_.e1_ * other.e431_) + (self_.e431_ * other.e1_) - (self_.e2_ * other.e423_) - (self_.e423_ * other.e2_), 0.0) + ((vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0)) + ((vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0)) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0)) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((self_.e2_ * other.e3_) - (self_.e3_ * other.e2_), (self_.e3_ * other.e1_) - (self_.e1_ * other.e3_), (self_.e1_ * other.e2_) - (self_.e2_ * other.e1_), 0.0) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0)) - ((vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e1_ * other.e1234_) + (self_.e3_ * other.e42_) + (self_.e4_ * other.e23_) + (self_.e412_ * other.e31_) + (self_.e321_ * other.e41_) - (self_.e2_ * other.e43_) - (self_.e431_ * other.e12_), (self_.e1_ * other.e43_) + (self_.e2_ * other.e1234_) + (self_.e4_ * other.e31_) + (self_.e423_ * other.e12_) + (self_.e321_ * other.e42_) - (self_.e3_ * other.e41_) - (self_.e412_ * other.e23_), (self_.e2_ * other.e41_) + (self_.e3_ * other.e1234_) + (self_.e4_ * other.e12_) + (self_.e431_ * other.e23_) + (self_.e321_ * other.e43_) - (self_.e1_ * other.e42_) - (self_.e423_ * other.e31_), -(self_.e1_ * other.e23_) - (self_.e2_ * other.e31_) - (self_.e3_ * other.e12_)) + (vec4<f32>(other.scalar) * self_groups.group1_)
    ));
    return multiVector_geometricProduct_flector(geometric_product, flector_reverse(self_));
}
fn flector_sandwich_origin(self_: Flector, other: Origin) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = origin_grouped(other);
    let geometric_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e4_) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, self_.e321_) * vec4<f32>(-1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0)
    ));
    return motor_geometricProduct_flector(geometric_product, flector_reverse(self_));
}
fn flector_sandwich_plane(self_: Flector, other: Plane) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = plane_grouped(other);
    let geometric_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((self_.e2_ * other.e412_) - (self_.e3_ * other.e431_) - (self_.e321_ * other.e423_), (self_.e3_ * other.e423_) - (self_.e1_ * other.e412_) - (self_.e321_ * other.e431_), (self_.e1_ * other.e431_) - (self_.e2_ * other.e423_) - (self_.e321_ * other.e412_), (self_.e1_ * other.e423_) + (self_.e2_ * other.e431_) + (self_.e3_ * other.e412_)) + (vec4<f32>(other.e321_) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, self_.e4_)), 
        /* e23, e31, e12, scalar */ vec4<f32>(other.e321_) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, self_.e321_) * vec4<f32>(-1.0)
    ));
    return motor_geometricProduct_flector(geometric_product, flector_reverse(self_));
}
fn flector_sandwich_point(self_: Flector, other: Point) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = point_grouped(other);
    let geometric_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((self_.e4_ * other.e1_) + (self_.e412_ * other.e2_) - (self_.e431_ * other.e3_), (self_.e4_ * other.e2_) + (self_.e423_ * other.e3_) - (self_.e412_ * other.e1_), (self_.e4_ * other.e3_) + (self_.e431_ * other.e1_) - (self_.e423_ * other.e2_), -(self_.e423_ * other.e1_) - (self_.e431_ * other.e2_) - (self_.e412_ * other.e3_)) - (vec4<f32>(other.e4_) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, self_.e321_)), 
        /* e23, e31, e12, scalar */ vec4<f32>((self_.e2_ * other.e3_) - (self_.e3_ * other.e2_) - (self_.e321_ * other.e1_), (self_.e3_ * other.e1_) - (self_.e1_ * other.e3_) - (self_.e321_ * other.e2_), (self_.e1_ * other.e2_) - (self_.e2_ * other.e1_) - (self_.e321_ * other.e3_), (self_.e1_ * other.e1_) + (self_.e2_ * other.e2_) + (self_.e3_ * other.e3_))
    ));
    return motor_geometricProduct_flector(geometric_product, flector_reverse(self_));
}
fn flector_sandwich_scalar(self_: Flector, other: Scalar) -> Motor {
    let self_groups = flector_grouped(self_);
    let geometric_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.scalar) * self_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other.scalar) * self_groups.group1_
    ));
    return flector_geometricProduct_flector(geometric_product, flector_reverse(self_));
}
fn horizon_sandwich_antiScalar(self_: Horizon, other: AntiScalar) -> AntiScalar {
    let geometric_product: Origin = Origin(other.e1234_ * self_.e321_);
    return origin_geometricProduct_horizon(geometric_product, horizon_reverse(self_));
}
fn horizon_sandwich_dualNum(self_: Horizon, other: DualNum) -> Motor {
    let geometric_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, other.e1234_ * self_.e321_) * vec4<f32>(0.0, 0.0, 0.0, 1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0, 1.0, 1.0, other.scalar * self_.e321_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)
    ));
    return flector_geometricProduct_horizon(geometric_product, horizon_reverse(self_));
}
fn horizon_sandwich_flector(self_: Horizon, other: Flector) -> Flector {
    let geometric_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.e321_) * vec4<f32>(other.e423_, other.e431_, other.e412_, other.e4_) * vec4<f32>(-1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.e321_) * vec4<f32>(other.e1_, other.e2_, other.e3_, other.e321_) * vec4<f32>(-1.0)
    ));
    return motor_geometricProduct_horizon(geometric_product, horizon_reverse(self_));
}
fn horizon_sandwich_horizon(self_: Horizon, other: Horizon) -> Horizon {
    let geometric_product: Scalar = Scalar(other.e321_ * self_.e321_ * -1.0);
    return scalar_geometricProduct_horizon(geometric_product, horizon_reverse(self_));
}
fn horizon_sandwich_line(self_: Horizon, other: Line) -> Motor {
    let geometric_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e321_ * other.e23_, self_.e321_ * other.e31_, self_.e321_ * other.e12_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e321_ * other.e41_, self_.e321_ * other.e42_, self_.e321_ * other.e43_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
    return flector_geometricProduct_horizon(geometric_product, horizon_reverse(self_));
}
fn horizon_sandwich_motor(self_: Horizon, other: Motor) -> Motor {
    let geometric_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e321_) * vec4<f32>(other.e23_, other.e31_, other.e12_, other.e1234_), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e321_) * vec4<f32>(other.e41_, other.e42_, other.e43_, other.scalar)
    ));
    return flector_geometricProduct_horizon(geometric_product, horizon_reverse(self_));
}
fn horizon_sandwich_multiVector(self_: Horizon, other: MultiVector) -> MultiVector {
    let geometric_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ (vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e321_, other.e4_, 0.0, 0.0) * vec2<f32>(-1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e321_) * vec4<f32>(other.e23_, other.e31_, other.e12_, other.e1234_), 
        /* e41, e42, e43 */ (vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ (vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0) * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e321_) * vec4<f32>(other.e41_, other.e42_, other.e43_, other.scalar)
    ));
    return multiVector_geometricProduct_horizon(geometric_product, horizon_reverse(self_));
}
fn horizon_sandwich_origin(self_: Horizon, other: Origin) -> Origin {
    let geometric_product: AntiScalar = AntiScalar(self_.e321_ * other.e4_ * -1.0);
    return antiScalar_geometricProduct_horizon(geometric_product, horizon_reverse(self_));
}
fn horizon_sandwich_plane(self_: Horizon, other: Plane) -> Flector {
    let geometric_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.e321_ * other.e423_, self_.e321_ * other.e431_, self_.e321_ * other.e412_, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(1.0, 1.0, 1.0, self_.e321_ * other.e321_) * vec4<f32>(0.0, 0.0, 0.0, -1.0)
    ));
    return motor_geometricProduct_horizon(geometric_product, horizon_reverse(self_));
}
fn horizon_sandwich_point(self_: Horizon, other: Point) -> Flector {
    let geometric_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(1.0, 1.0, 1.0, self_.e321_ * other.e4_) * vec4<f32>(0.0, 0.0, 0.0, -1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.e321_ * other.e1_, self_.e321_ * other.e2_, self_.e321_ * other.e3_, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0)
    ));
    return motor_geometricProduct_horizon(geometric_product, horizon_reverse(self_));
}
fn horizon_sandwich_scalar(self_: Horizon, other: Scalar) -> Scalar {
    let geometric_product: Horizon = Horizon(self_.e321_ * other.scalar);
    return horizon_geometricProduct_horizon(geometric_product, horizon_reverse(self_));
}
fn line_sandwich_antiScalar(self_: Line, other: AntiScalar) -> Motor {
    let self_groups = line_grouped(self_);
    let geometric_product: Line = line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_, 
        /* e23, e31, e12 */ vec4<f32>(0.0)
    ));
    return line_geometricProduct_line(geometric_product, line_reverse(self_));
}
fn line_sandwich_dualNum(self_: Line, other: DualNum) -> Motor {
    let self_groups = line_grouped(self_);
    let geometric_product: Line = line_degroup(LineGroups(
        /* e41, e42, e43 */ ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_) + ((vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_), 
        /* e23, e31, e12 */ (vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_
    ));
    return line_geometricProduct_line(geometric_product, line_reverse(self_));
}
fn line_sandwich_flector(self_: Line, other: Flector) -> Flector {
    let self_groups = line_grouped(self_);
    let geometric_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other.e2_ * self_.e12_) + (other.e321_ * self_.e23_) - (other.e3_ * self_.e31_), (other.e3_ * self_.e23_) + (other.e321_ * self_.e31_) - (other.e1_ * self_.e12_), (other.e1_ * self_.e31_) + (other.e321_ * self_.e12_) - (other.e2_ * self_.e23_), (other.e1_ * self_.e41_) + (other.e2_ * self_.e42_) + (other.e3_ * self_.e43_) - (other.e423_ * self_.e23_) - (other.e431_ * self_.e31_) - (other.e412_ * self_.e12_)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e3_ * self_.e42_) + (other.e4_ * self_.e23_) + (other.e431_ * self_.e12_) - (other.e2_ * self_.e43_) - (other.e412_ * self_.e31_) - (other.e321_ * self_.e41_), (other.e1_ * self_.e43_) + (other.e4_ * self_.e31_) + (other.e412_ * self_.e23_) - (other.e3_ * self_.e41_) - (other.e423_ * self_.e12_) - (other.e321_ * self_.e42_), (other.e2_ * self_.e41_) + (other.e4_ * self_.e12_) + (other.e423_ * self_.e31_) - (other.e1_ * self_.e42_) - (other.e431_ * self_.e23_) - (other.e321_ * self_.e43_), -(other.e1_ * self_.e23_) - (other.e2_ * self_.e31_) - (other.e3_ * self_.e12_))
    ));
    return flector_geometricProduct_line(geometric_product, line_reverse(self_));
}
fn line_sandwich_horizon(self_: Line, other: Horizon) -> Flector {
    let self_groups = line_grouped(self_);
    let geometric_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e321_ * self_.e23_, other.e321_ * self_.e31_, other.e321_ * self_.e12_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e321_ * self_.e41_, other.e321_ * self_.e42_, other.e321_ * self_.e43_, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0)
    ));
    return flector_geometricProduct_line(geometric_product, line_reverse(self_));
}
fn line_sandwich_line(self_: Line, other: Line) -> Motor {
    let self_groups = line_grouped(self_);
    let geometric_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other.e42_ * self_.e12_) + (other.e31_ * self_.e43_) - (other.e43_ * self_.e31_) - (other.e12_ * self_.e42_), (other.e43_ * self_.e23_) + (other.e12_ * self_.e41_) - (other.e41_ * self_.e12_) - (other.e23_ * self_.e43_), (other.e41_ * self_.e31_) + (other.e23_ * self_.e42_) - (other.e42_ * self_.e23_) - (other.e31_ * self_.e41_), -(other.e41_ * self_.e23_) - (other.e42_ * self_.e31_) - (other.e43_ * self_.e12_) - (other.e23_ * self_.e41_) - (other.e31_ * self_.e42_) - (other.e12_ * self_.e43_)), 
        /* e23, e31, e12, scalar */ vec4<f32>((other.e31_ * self_.e12_) - (other.e12_ * self_.e31_), (other.e12_ * self_.e23_) - (other.e23_ * self_.e12_), (other.e23_ * self_.e31_) - (other.e31_ * self_.e23_), -(other.e23_ * self_.e23_) - (other.e31_ * self_.e31_) - (other.e12_ * self_.e12_))
    ));
    return motor_geometricProduct_line(geometric_product, line_reverse(self_));
}
fn line_sandwich_motor(self_: Line, other: Motor) -> Motor {
    let self_groups = line_grouped(self_);
    let geometric_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((self_.e41_ * other.scalar) + (self_.e43_ * other.e31_) + (self_.e23_ * other.e1234_) + (self_.e12_ * other.e42_) - (self_.e42_ * other.e12_) - (self_.e31_ * other.e43_), (self_.e41_ * other.e12_) + (self_.e42_ * other.scalar) + (self_.e23_ * other.e43_) + (self_.e31_ * other.e1234_) - (self_.e43_ * other.e23_) - (self_.e12_ * other.e41_), (self_.e42_ * other.e23_) + (self_.e43_ * other.scalar) + (self_.e31_ * other.e41_) + (self_.e12_ * other.e1234_) - (self_.e41_ * other.e31_) - (self_.e23_ * other.e42_), -(self_.e41_ * other.e23_) - (self_.e42_ * other.e31_) - (self_.e43_ * other.e12_) - (self_.e23_ * other.e41_) - (self_.e31_ * other.e42_) - (self_.e12_ * other.e43_)), 
        /* e23, e31, e12, scalar */ vec4<f32>((self_.e23_ * other.scalar) + (self_.e12_ * other.e31_) - (self_.e31_ * other.e12_), (self_.e23_ * other.e12_) + (self_.e31_ * other.scalar) - (self_.e12_ * other.e23_), (self_.e31_ * other.e23_) + (self_.e12_ * other.scalar) - (self_.e23_ * other.e31_), -(self_.e23_ * other.e23_) - (self_.e31_ * other.e31_) - (self_.e12_ * other.e12_))
    ));
    return motor_geometricProduct_line(geometric_product, line_reverse(self_));
}
fn line_sandwich_multiVector(self_: Line, other: MultiVector) -> MultiVector {
    let self_groups = line_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let geometric_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, -(self_.e23_ * other.e41_) - (self_.e31_ * other.e42_) - (self_.e12_ * other.e43_), 0.0, 0.0) - ((vec4<f32>(other.e23_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e23_, self_.e41_, 0.0, 0.0)) - ((vec4<f32>(other.e31_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e31_, self_.e42_, 0.0, 0.0)) - ((vec4<f32>(other.e12_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e12_, self_.e43_, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e23_ * other.e321_) + (self_.e12_ * other.e2_) - (self_.e31_ * other.e3_), (self_.e23_ * other.e3_) + (self_.e31_ * other.e321_) - (self_.e12_ * other.e1_), (self_.e31_ * other.e1_) + (self_.e12_ * other.e321_) - (self_.e23_ * other.e2_), (self_.e41_ * other.e1_) + (self_.e42_ * other.e2_) + (self_.e43_ * other.e3_) - (self_.e23_ * other.e423_) - (self_.e31_ * other.e431_) - (self_.e12_ * other.e412_)), 
        /* e41, e42, e43 */ vec4<f32>((self_.e43_ * other.e31_) + (self_.e12_ * other.e42_) - (self_.e42_ * other.e12_) - (self_.e31_ * other.e43_), (self_.e41_ * other.e12_) + (self_.e23_ * other.e43_) - (self_.e43_ * other.e23_) - (self_.e12_ * other.e41_), (self_.e42_ * other.e23_) + (self_.e31_ * other.e41_) - (self_.e41_ * other.e31_) - (self_.e23_ * other.e42_), 0.0) + ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_) + ((vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_), 
        /* e23, e31, e12 */ vec4<f32>((self_.e12_ * other.e31_) - (self_.e31_ * other.e12_), (self_.e23_ * other.e12_) - (self_.e12_ * other.e23_), (self_.e31_ * other.e23_) - (self_.e23_ * other.e31_), 0.0) + ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e42_ * other.e3_) + (self_.e23_ * other.e4_) + (self_.e12_ * other.e431_) - (self_.e41_ * other.e321_) - (self_.e43_ * other.e2_) - (self_.e31_ * other.e412_), (self_.e43_ * other.e1_) + (self_.e23_ * other.e412_) + (self_.e31_ * other.e4_) - (self_.e41_ * other.e3_) - (self_.e42_ * other.e321_) - (self_.e12_ * other.e423_), (self_.e41_ * other.e2_) + (self_.e31_ * other.e423_) + (self_.e12_ * other.e4_) - (self_.e42_ * other.e1_) - (self_.e43_ * other.e321_) - (self_.e23_ * other.e431_), -(self_.e23_ * other.e1_) - (self_.e31_ * other.e2_) - (self_.e12_ * other.e3_))
    ));
    return multiVector_geometricProduct_line(geometric_product, line_reverse(self_));
}
fn line_sandwich_origin(self_: Line, other: Origin) -> Flector {
    let self_groups = line_grouped(self_);
    let geometric_product: Plane = plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e23_ * other.e4_, self_.e31_ * other.e4_, self_.e12_ * other.e4_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
    return plane_geometricProduct_line(geometric_product, line_reverse(self_));
}
fn line_sandwich_plane(self_: Line, other: Plane) -> Flector {
    let self_groups = line_grouped(self_);
    let geometric_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e23_ * other.e321_, self_.e31_ * other.e321_, self_.e12_ * other.e321_, -(self_.e23_ * other.e423_) - (self_.e31_ * other.e431_) - (self_.e12_ * other.e412_)), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e12_ * other.e431_) - (self_.e41_ * other.e321_) - (self_.e31_ * other.e412_), (self_.e23_ * other.e412_) - (self_.e42_ * other.e321_) - (self_.e12_ * other.e423_), (self_.e31_ * other.e423_) - (self_.e43_ * other.e321_) - (self_.e23_ * other.e431_), 0.0)
    ));
    return flector_geometricProduct_line(geometric_product, line_reverse(self_));
}
fn line_sandwich_point(self_: Line, other: Point) -> Flector {
    let self_groups = line_grouped(self_);
    let geometric_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e12_ * other.e2_) - (self_.e31_ * other.e3_), (self_.e23_ * other.e3_) - (self_.e12_ * other.e1_), (self_.e31_ * other.e1_) - (self_.e23_ * other.e2_), (self_.e41_ * other.e1_) + (self_.e42_ * other.e2_) + (self_.e43_ * other.e3_)), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e42_ * other.e3_) + (self_.e23_ * other.e4_) - (self_.e43_ * other.e2_), (self_.e43_ * other.e1_) + (self_.e31_ * other.e4_) - (self_.e41_ * other.e3_), (self_.e41_ * other.e2_) + (self_.e12_ * other.e4_) - (self_.e42_ * other.e1_), -(self_.e23_ * other.e1_) - (self_.e31_ * other.e2_) - (self_.e12_ * other.e3_))
    ));
    return flector_geometricProduct_line(geometric_product, line_reverse(self_));
}
fn line_sandwich_scalar(self_: Line, other: Scalar) -> Motor {
    let self_groups = line_grouped(self_);
    let geometric_product: Line = line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_
    ));
    return line_geometricProduct_line(geometric_product, line_reverse(self_));
}
fn motor_sandwich_antiScalar(self_: Motor, other: AntiScalar) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    let geometric_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e1234_) * self_groups.group1_, 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0)
    ));
    return motor_geometricProduct_motor(geometric_product, motor_reverse(self_));
}
fn motor_sandwich_dualNum(self_: Motor, other: DualNum) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let geometric_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ (vec4<f32>(other.scalar) * self_groups.group0_) + (vec4<f32>(other.e1234_) * self_groups.group1_), 
        /* e23, e31, e12, scalar */ vec4<f32>(other.scalar) * self_groups.group1_
    ));
    return motor_geometricProduct_motor(geometric_product, motor_reverse(self_));
}
fn motor_sandwich_flector(self_: Motor, other: Flector) -> Flector {
    let self_groups = motor_grouped(self_);
    let other_groups = flector_grouped(other);
    let geometric_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other.e2_ * self_.e12_) + (other.e321_ * self_.e23_) - (other.e3_ * self_.e31_), (other.e3_ * self_.e23_) + (other.e321_ * self_.e31_) - (other.e1_ * self_.e12_), (other.e1_ * self_.e31_) + (other.e321_ * self_.e12_) - (other.e2_ * self_.e23_), (other.e1_ * self_.e41_) + (other.e2_ * self_.e42_) + (other.e3_ * self_.e43_) - (other.e423_ * self_.e23_) - (other.e431_ * self_.e31_) - (other.e412_ * self_.e12_) - (other.e321_ * self_.e1234_)) + (vec4<f32>(self_.scalar) * other_groups.group0_), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e3_ * self_.e42_) + (other.e4_ * self_.e23_) + (other.e431_ * self_.e12_) - (other.e1_ * self_.e1234_) - (other.e2_ * self_.e43_) - (other.e412_ * self_.e31_) - (other.e321_ * self_.e41_), (other.e1_ * self_.e43_) + (other.e4_ * self_.e31_) + (other.e412_ * self_.e23_) - (other.e2_ * self_.e1234_) - (other.e3_ * self_.e41_) - (other.e423_ * self_.e12_) - (other.e321_ * self_.e42_), (other.e2_ * self_.e41_) + (other.e4_ * self_.e12_) + (other.e423_ * self_.e31_) - (other.e1_ * self_.e42_) - (other.e3_ * self_.e1234_) - (other.e431_ * self_.e23_) - (other.e321_ * self_.e43_), -(other.e1_ * self_.e23_) - (other.e2_ * self_.e31_) - (other.e3_ * self_.e12_)) + (vec4<f32>(self_.scalar) * other_groups.group1_)
    ));
    return flector_geometricProduct_motor(geometric_product, motor_reverse(self_));
}
fn motor_sandwich_horizon(self_: Motor, other: Horizon) -> Flector {
    let self_groups = motor_grouped(self_);
    let geometric_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e321_) * vec4<f32>(self_.e23_, self_.e31_, self_.e12_, self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, -1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e321_) * vec4<f32>(self_.e41_, self_.e42_, self_.e43_, self_.scalar) * vec4<f32>(-1.0, -1.0, -1.0, 1.0)
    ));
    return flector_geometricProduct_motor(geometric_product, motor_reverse(self_));
}
fn motor_sandwich_line(self_: Motor, other: Line) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = line_grouped(other);
    let geometric_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other.e41_ * self_.scalar) + (other.e42_ * self_.e12_) + (other.e23_ * self_.e1234_) + (other.e31_ * self_.e43_) - (other.e43_ * self_.e31_) - (other.e12_ * self_.e42_), (other.e42_ * self_.scalar) + (other.e43_ * self_.e23_) + (other.e31_ * self_.e1234_) + (other.e12_ * self_.e41_) - (other.e41_ * self_.e12_) - (other.e23_ * self_.e43_), (other.e41_ * self_.e31_) + (other.e43_ * self_.scalar) + (other.e23_ * self_.e42_) + (other.e12_ * self_.e1234_) - (other.e42_ * self_.e23_) - (other.e31_ * self_.e41_), -(other.e41_ * self_.e23_) - (other.e42_ * self_.e31_) - (other.e43_ * self_.e12_) - (other.e23_ * self_.e41_) - (other.e31_ * self_.e42_) - (other.e12_ * self_.e43_)), 
        /* e23, e31, e12, scalar */ vec4<f32>((other.e23_ * self_.scalar) + (other.e31_ * self_.e12_) - (other.e12_ * self_.e31_), (other.e31_ * self_.scalar) + (other.e12_ * self_.e23_) - (other.e23_ * self_.e12_), (other.e23_ * self_.e31_) + (other.e12_ * self_.scalar) - (other.e31_ * self_.e23_), -(other.e23_ * self_.e23_) - (other.e31_ * self_.e31_) - (other.e12_ * self_.e12_))
    ));
    return motor_geometricProduct_motor(geometric_product, motor_reverse(self_));
}
fn motor_sandwich_motor(self_: Motor, other: Motor) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = motor_grouped(other);
    let geometric_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other.e42_ * self_.e12_) + (other.e1234_ * self_.e23_) + (other.e31_ * self_.e43_) + (other.scalar * self_.e41_) - (other.e43_ * self_.e31_) - (other.e12_ * self_.e42_), (other.e43_ * self_.e23_) + (other.e1234_ * self_.e31_) + (other.e12_ * self_.e41_) + (other.scalar * self_.e42_) - (other.e41_ * self_.e12_) - (other.e23_ * self_.e43_), (other.e41_ * self_.e31_) + (other.e1234_ * self_.e12_) + (other.e23_ * self_.e42_) + (other.scalar * self_.e43_) - (other.e42_ * self_.e23_) - (other.e31_ * self_.e41_), -(other.e41_ * self_.e23_) - (other.e42_ * self_.e31_) - (other.e43_ * self_.e12_) - (other.e23_ * self_.e41_) - (other.e31_ * self_.e42_) - (other.e12_ * self_.e43_)) + (vec4<f32>(self_.e1234_) * other_groups.group1_) + (vec4<f32>(self_.scalar) * other_groups.group0_), 
        /* e23, e31, e12, scalar */ vec4<f32>((other.e31_ * self_.e12_) + (other.scalar * self_.e23_) - (other.e12_ * self_.e31_), (other.e12_ * self_.e23_) + (other.scalar * self_.e31_) - (other.e23_ * self_.e12_), (other.e23_ * self_.e31_) + (other.scalar * self_.e12_) - (other.e31_ * self_.e23_), -(other.e23_ * self_.e23_) - (other.e31_ * self_.e31_) - (other.e12_ * self_.e12_)) + (vec4<f32>(self_.scalar) * other_groups.group1_)
    ));
    return motor_geometricProduct_motor(geometric_product, motor_reverse(self_));
}
fn motor_sandwich_multiVector(self_: Motor, other: MultiVector) -> MultiVector {
    let self_groups = motor_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let geometric_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, (self_.scalar * other.e1234_) - (self_.e23_ * other.e41_) - (self_.e31_ * other.e42_) - (self_.e12_ * other.e43_), 0.0, 0.0) + ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.scalar, self_.e1234_, 0.0, 0.0)) - ((vec4<f32>(other.e23_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e23_, self_.e41_, 0.0, 0.0)) - ((vec4<f32>(other.e31_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e31_, self_.e42_, 0.0, 0.0)) - ((vec4<f32>(other.e12_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e12_, self_.e43_, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e23_ * other.e321_) + (self_.e12_ * other.e2_) - (self_.e31_ * other.e3_), (self_.e23_ * other.e3_) + (self_.e31_ * other.e321_) - (self_.e12_ * other.e1_), (self_.e31_ * other.e1_) + (self_.e12_ * other.e321_) - (self_.e23_ * other.e2_), (self_.e41_ * other.e1_) + (self_.e42_ * other.e2_) + (self_.e43_ * other.e3_) - (self_.e1234_ * other.e321_) - (self_.e23_ * other.e423_) - (self_.e31_ * other.e431_) - (self_.e12_ * other.e412_)) + (vec4<f32>(self_.scalar) * other_groups.group1_), 
        /* e41, e42, e43 */ vec4<f32>((self_.e43_ * other.e31_) + (self_.e12_ * other.e42_) - (self_.e42_ * other.e12_) - (self_.e31_ * other.e43_), (self_.e41_ * other.e12_) + (self_.e23_ * other.e43_) - (self_.e43_ * other.e23_) - (self_.e12_ * other.e41_), (self_.e42_ * other.e23_) + (self_.e31_ * other.e41_) - (self_.e41_ * other.e31_) - (self_.e23_ * other.e42_), 0.0) + ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group3_) + ((vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group2_) + ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e41_, self_.e42_, self_.e43_, 0.0)) + ((vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e23_, self_.e31_, self_.e12_, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((self_.e12_ * other.e31_) - (self_.e31_ * other.e12_), (self_.e23_ * other.e12_) - (self_.e12_ * other.e23_), (self_.e31_ * other.e23_) - (self_.e23_ * other.e31_), 0.0) + ((vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group3_) + ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e23_, self_.e31_, self_.e12_, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e42_ * other.e3_) + (self_.e23_ * other.e4_) + (self_.e12_ * other.e431_) - (self_.e41_ * other.e321_) - (self_.e43_ * other.e2_) - (self_.e1234_ * other.e1_) - (self_.e31_ * other.e412_), (self_.e43_ * other.e1_) + (self_.e23_ * other.e412_) + (self_.e31_ * other.e4_) - (self_.e41_ * other.e3_) - (self_.e42_ * other.e321_) - (self_.e1234_ * other.e2_) - (self_.e12_ * other.e423_), (self_.e41_ * other.e2_) + (self_.e31_ * other.e423_) + (self_.e12_ * other.e4_) - (self_.e42_ * other.e1_) - (self_.e43_ * other.e321_) - (self_.e1234_ * other.e3_) - (self_.e23_ * other.e431_), -(self_.e23_ * other.e1_) - (self_.e31_ * other.e2_) - (self_.e12_ * other.e3_)) + (vec4<f32>(self_.scalar) * other_groups.group4_)
    ));
    return multiVector_geometricProduct_motor(geometric_product, motor_reverse(self_));
}
fn motor_sandwich_origin(self_: Motor, other: Origin) -> Flector {
    let self_groups = motor_grouped(self_);
    let geometric_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, self_.scalar * other.e4_) * vec4<f32>(0.0, 0.0, 0.0, 1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e23_ * other.e4_, self_.e31_ * other.e4_, self_.e12_ * other.e4_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
    return flector_geometricProduct_motor(geometric_product, motor_reverse(self_));
}
fn motor_sandwich_plane(self_: Motor, other: Plane) -> Flector {
    let self_groups = motor_grouped(self_);
    let geometric_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e23_ * other.e321_, self_.e31_ * other.e321_, self_.e12_ * other.e321_, -(self_.e1234_ * other.e321_) - (self_.e23_ * other.e423_) - (self_.e31_ * other.e431_) - (self_.e12_ * other.e412_)), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e12_ * other.e431_) + (self_.scalar * other.e423_) - (self_.e41_ * other.e321_) - (self_.e31_ * other.e412_), (self_.e23_ * other.e412_) + (self_.scalar * other.e431_) - (self_.e42_ * other.e321_) - (self_.e12_ * other.e423_), (self_.e31_ * other.e423_) + (self_.scalar * other.e412_) - (self_.e43_ * other.e321_) - (self_.e23_ * other.e431_), self_.scalar * other.e321_)
    ));
    return flector_geometricProduct_motor(geometric_product, motor_reverse(self_));
}
fn motor_sandwich_point(self_: Motor, other: Point) -> Flector {
    let self_groups = motor_grouped(self_);
    let other_groups = point_grouped(other);
    let geometric_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e12_ * other.e2_) - (self_.e31_ * other.e3_), (self_.e23_ * other.e3_) - (self_.e12_ * other.e1_), (self_.e31_ * other.e1_) - (self_.e23_ * other.e2_), (self_.e41_ * other.e1_) + (self_.e42_ * other.e2_) + (self_.e43_ * other.e3_)) + (vec4<f32>(self_.scalar) * other_groups.group0_), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e42_ * other.e3_) + (self_.e23_ * other.e4_) - (self_.e43_ * other.e2_) - (self_.e1234_ * other.e1_), (self_.e43_ * other.e1_) + (self_.e31_ * other.e4_) - (self_.e41_ * other.e3_) - (self_.e1234_ * other.e2_), (self_.e41_ * other.e2_) + (self_.e12_ * other.e4_) - (self_.e42_ * other.e1_) - (self_.e1234_ * other.e3_), -(self_.e23_ * other.e1_) - (self_.e31_ * other.e2_) - (self_.e12_ * other.e3_))
    ));
    return flector_geometricProduct_motor(geometric_product, motor_reverse(self_));
}
fn motor_sandwich_scalar(self_: Motor, other: Scalar) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = scalar_grouped(other);
    let geometric_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.scalar) * self_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(other.scalar) * self_groups.group1_
    ));
    return motor_geometricProduct_motor(geometric_product, motor_reverse(self_));
}
fn multiVector_sandwich_antiScalar(self_: MultiVector, other: AntiScalar) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    let geometric_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(1.0, other.e1234_ * self_.scalar, 0.0, 0.0) * vec2<f32>(0.0, 1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, other.e1234_ * self_.e321_) * vec4<f32>(0.0, 0.0, 0.0, 1.0), 
        /* e41, e42, e43 */ (vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_, 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e1234_ * self_.e1_, other.e1234_ * self_.e2_, other.e1234_ * self_.e3_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self_));
}
fn multiVector_sandwich_dualNum(self_: MultiVector, other: DualNum) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let geometric_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar * self_.scalar, (other.scalar * self_.e1234_) + (other.e1234_ * self_.scalar), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(other.scalar * self_.e1_, other.scalar * self_.e2_, other.scalar * self_.e3_, (other.scalar * self_.e4_) + (other.e1234_ * self_.e321_)), 
        /* e41, e42, e43 */ ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_) + ((vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_), 
        /* e23, e31, e12 */ (vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>((other.scalar * self_.e423_) + (other.e1234_ * self_.e1_), (other.scalar * self_.e431_) + (other.e1234_ * self_.e2_), (other.scalar * self_.e412_) + (other.e1234_ * self_.e3_), other.scalar * self_.e321_)
    ));
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self_));
}
fn multiVector_sandwich_flector(self_: MultiVector, other: Flector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = flector_grouped(other);
    let geometric_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, (other.e321_ * self_.e4_) - (other.e1_ * self_.e423_) - (other.e2_ * self_.e431_) - (other.e3_ * self_.e412_), 0.0, 0.0) + ((vec4<f32>(self_.e1_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e1_, other.e423_, 0.0, 0.0)) + ((vec4<f32>(self_.e2_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e2_, other.e431_, 0.0, 0.0)) + ((vec4<f32>(self_.e3_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e3_, other.e412_, 0.0, 0.0)) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e321_, other.e4_, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((other.e2_ * self_.e12_) + (other.e321_ * self_.e23_) - (other.e3_ * self_.e31_), (other.e3_ * self_.e23_) + (other.e321_ * self_.e31_) - (other.e1_ * self_.e12_), (other.e1_ * self_.e31_) + (other.e321_ * self_.e12_) - (other.e2_ * self_.e23_), (other.e1_ * self_.e41_) + (other.e2_ * self_.e42_) + (other.e3_ * self_.e43_) - (other.e423_ * self_.e23_) - (other.e431_ * self_.e31_) - (other.e412_ * self_.e12_) - (other.e321_ * self_.e1234_)) + (vec4<f32>(self_.scalar) * other_groups.group0_), 
        /* e41, e42, e43 */ vec4<f32>((other.e2_ * self_.e412_) + (other.e412_ * self_.e2_) - (other.e3_ * self_.e431_) - (other.e431_ * self_.e3_), (other.e3_ * self_.e423_) + (other.e423_ * self_.e3_) - (other.e1_ * self_.e412_) - (other.e412_ * self_.e1_), (other.e1_ * self_.e431_) + (other.e431_ * self_.e1_) - (other.e2_ * self_.e423_) - (other.e423_ * self_.e2_), 0.0) + ((vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0)) + ((vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0)) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0)) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((other.e3_ * self_.e2_) - (other.e2_ * self_.e3_), (other.e1_ * self_.e3_) - (other.e3_ * self_.e1_), (other.e2_ * self_.e1_) - (other.e1_ * self_.e2_), 0.0) - ((vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0)) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e3_ * self_.e42_) + (other.e4_ * self_.e23_) + (other.e431_ * self_.e12_) - (other.e1_ * self_.e1234_) - (other.e2_ * self_.e43_) - (other.e412_ * self_.e31_) - (other.e321_ * self_.e41_), (other.e1_ * self_.e43_) + (other.e4_ * self_.e31_) + (other.e412_ * self_.e23_) - (other.e2_ * self_.e1234_) - (other.e3_ * self_.e41_) - (other.e423_ * self_.e12_) - (other.e321_ * self_.e42_), (other.e2_ * self_.e41_) + (other.e4_ * self_.e12_) + (other.e423_ * self_.e31_) - (other.e1_ * self_.e42_) - (other.e3_ * self_.e1234_) - (other.e431_ * self_.e23_) - (other.e321_ * self_.e43_), -(other.e1_ * self_.e23_) - (other.e2_ * self_.e31_) - (other.e3_ * self_.e12_)) + (vec4<f32>(self_.scalar) * other_groups.group1_)
    ));
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self_));
}
fn multiVector_sandwich_horizon(self_: MultiVector, other: Horizon) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = horizon_grouped(other);
    let geometric_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ (vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e321_, self_.e4_, 0.0, 0.0) * vec2<f32>(-1.0, 1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(other.e321_) * vec4<f32>(self_.e23_, self_.e31_, self_.e12_, self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, -1.0), 
        /* e41, e42, e43 */ (vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0), 
        /* e23, e31, e12 */ (vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0) * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e321_) * vec4<f32>(self_.e41_, self_.e42_, self_.e43_, self_.scalar) * vec4<f32>(-1.0, -1.0, -1.0, 1.0)
    ));
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self_));
}
fn multiVector_sandwich_line(self_: MultiVector, other: Line) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = line_grouped(other);
    let geometric_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, -(other.e23_ * self_.e41_) - (other.e31_ * self_.e42_) - (other.e12_ * self_.e43_), 0.0, 0.0) - ((vec4<f32>(self_.e23_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e23_, other.e41_, 0.0, 0.0)) - ((vec4<f32>(self_.e31_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e31_, other.e42_, 0.0, 0.0)) - ((vec4<f32>(self_.e12_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e12_, other.e43_, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((other.e23_ * self_.e321_) + (other.e31_ * self_.e3_) - (other.e12_ * self_.e2_), (other.e31_ * self_.e321_) + (other.e12_ * self_.e1_) - (other.e23_ * self_.e3_), (other.e23_ * self_.e2_) + (other.e12_ * self_.e321_) - (other.e31_ * self_.e1_), -(other.e41_ * self_.e1_) - (other.e42_ * self_.e2_) - (other.e43_ * self_.e3_) - (other.e23_ * self_.e423_) - (other.e31_ * self_.e431_) - (other.e12_ * self_.e412_)), 
        /* e41, e42, e43 */ vec4<f32>((other.e42_ * self_.e12_) + (other.e31_ * self_.e43_) - (other.e43_ * self_.e31_) - (other.e12_ * self_.e42_), (other.e43_ * self_.e23_) + (other.e12_ * self_.e41_) - (other.e41_ * self_.e12_) - (other.e23_ * self_.e43_), (other.e41_ * self_.e31_) + (other.e23_ * self_.e42_) - (other.e42_ * self_.e23_) - (other.e31_ * self_.e41_), 0.0) + ((vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group0_) + ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group1_), 
        /* e23, e31, e12 */ vec4<f32>((other.e31_ * self_.e12_) - (other.e12_ * self_.e31_), (other.e12_ * self_.e23_) - (other.e23_ * self_.e12_), (other.e23_ * self_.e31_) - (other.e31_ * self_.e23_), 0.0) + ((vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group1_), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e41_ * self_.e321_) + (other.e42_ * self_.e3_) + (other.e23_ * self_.e4_) + (other.e31_ * self_.e412_) - (other.e43_ * self_.e2_) - (other.e12_ * self_.e431_), (other.e42_ * self_.e321_) + (other.e43_ * self_.e1_) + (other.e31_ * self_.e4_) + (other.e12_ * self_.e423_) - (other.e41_ * self_.e3_) - (other.e23_ * self_.e412_), (other.e41_ * self_.e2_) + (other.e43_ * self_.e321_) + (other.e23_ * self_.e431_) + (other.e12_ * self_.e4_) - (other.e42_ * self_.e1_) - (other.e31_ * self_.e423_), -(other.e23_ * self_.e1_) - (other.e31_ * self_.e2_) - (other.e12_ * self_.e3_))
    ));
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self_));
}
fn multiVector_sandwich_motor(self_: MultiVector, other: Motor) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = motor_grouped(other);
    let geometric_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, (other.scalar * self_.e1234_) - (other.e23_ * self_.e41_) - (other.e31_ * self_.e42_) - (other.e12_ * self_.e43_), 0.0, 0.0) + ((vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.scalar, other.e1234_, 0.0, 0.0)) - ((vec4<f32>(self_.e23_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e23_, other.e41_, 0.0, 0.0)) - ((vec4<f32>(self_.e31_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e31_, other.e42_, 0.0, 0.0)) - ((vec4<f32>(self_.e12_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e12_, other.e43_, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((other.e31_ * self_.e3_) - (other.e12_ * self_.e2_), (other.e12_ * self_.e1_) - (other.e23_ * self_.e3_), (other.e23_ * self_.e2_) - (other.e31_ * self_.e1_), -(other.e41_ * self_.e1_) - (other.e42_ * self_.e2_) - (other.e43_ * self_.e3_) - (other.e23_ * self_.e423_) - (other.e31_ * self_.e431_) - (other.e12_ * self_.e412_)) + (vec4<f32>(other.scalar) * self_groups.group1_) + (vec4<f32>(self_.e321_) * vec4<f32>(other.e23_, other.e31_, other.e12_, other.e1234_)), 
        /* e41, e42, e43 */ vec4<f32>((other.e42_ * self_.e12_) + (other.e31_ * self_.e43_) - (other.e43_ * self_.e31_) - (other.e12_ * self_.e42_), (other.e43_ * self_.e23_) + (other.e12_ * self_.e41_) - (other.e41_ * self_.e12_) - (other.e23_ * self_.e43_), (other.e41_ * self_.e31_) + (other.e23_ * self_.e42_) - (other.e42_ * self_.e23_) - (other.e31_ * self_.e41_), 0.0) + ((vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_) + ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_) + ((vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e41_, other.e42_, other.e43_, 0.0)) + ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e23_, other.e31_, other.e12_, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((other.e31_ * self_.e12_) - (other.e12_ * self_.e31_), (other.e12_ * self_.e23_) - (other.e23_ * self_.e12_), (other.e23_ * self_.e31_) - (other.e31_ * self_.e23_), 0.0) + ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_) + ((vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e23_, other.e31_, other.e12_, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e42_ * self_.e3_) + (other.e1234_ * self_.e1_) + (other.e23_ * self_.e4_) + (other.e31_ * self_.e412_) + (other.scalar * self_.e423_) - (other.e43_ * self_.e2_) - (other.e12_ * self_.e431_), (other.e43_ * self_.e1_) + (other.e1234_ * self_.e2_) + (other.e31_ * self_.e4_) + (other.e12_ * self_.e423_) + (other.scalar * self_.e431_) - (other.e41_ * self_.e3_) - (other.e23_ * self_.e412_), (other.e41_ * self_.e2_) + (other.e1234_ * self_.e3_) + (other.e23_ * self_.e431_) + (other.e12_ * self_.e4_) + (other.scalar * self_.e412_) - (other.e42_ * self_.e1_) - (other.e31_ * self_.e423_), -(other.e23_ * self_.e1_) - (other.e31_ * self_.e2_) - (other.e12_ * self_.e3_)) + (vec4<f32>(self_.e321_) * vec4<f32>(other.e41_, other.e42_, other.e43_, other.scalar))
    ));
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self_));
}
fn multiVector_sandwich_multiVector(self_: MultiVector, other: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let geometric_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, (other.e1234_ * self_.scalar) + (other.e321_ * self_.e4_) - (other.e1_ * self_.e423_) - (other.e2_ * self_.e431_) - (other.e3_ * self_.e412_) - (other.e23_ * self_.e41_) - (other.e31_ * self_.e42_) - (other.e12_ * self_.e43_), 0.0, 0.0) + ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_) + ((vec4<f32>(self_.e1_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e1_, other.e423_, 0.0, 0.0)) + ((vec4<f32>(self_.e2_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e2_, other.e431_, 0.0, 0.0)) + ((vec4<f32>(self_.e3_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e3_, other.e412_, 0.0, 0.0)) - ((vec4<f32>(self_.e23_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e23_, other.e41_, 0.0, 0.0)) - ((vec4<f32>(self_.e31_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e31_, other.e42_, 0.0, 0.0)) - ((vec4<f32>(self_.e12_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e12_, other.e43_, 0.0, 0.0)) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e321_, other.e4_, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((other.e2_ * self_.e12_) + (other.e31_ * self_.e3_) + (other.e321_ * self_.e23_) - (other.e3_ * self_.e31_) - (other.e12_ * self_.e2_), (other.e3_ * self_.e23_) + (other.e12_ * self_.e1_) + (other.e321_ * self_.e31_) - (other.e1_ * self_.e12_) - (other.e23_ * self_.e3_), (other.e1_ * self_.e31_) + (other.e23_ * self_.e2_) + (other.e321_ * self_.e12_) - (other.e2_ * self_.e23_) - (other.e31_ * self_.e1_), (other.e1_ * self_.e41_) + (other.e2_ * self_.e42_) + (other.e3_ * self_.e43_) - (other.e41_ * self_.e1_) - (other.e42_ * self_.e2_) - (other.e43_ * self_.e3_) - (other.e23_ * self_.e423_) - (other.e31_ * self_.e431_) - (other.e12_ * self_.e412_) - (other.e423_ * self_.e23_) - (other.e431_ * self_.e31_) - (other.e412_ * self_.e12_) - (other.e321_ * self_.e1234_)) + (vec4<f32>(other.scalar) * self_groups.group1_) + (vec4<f32>(self_.scalar) * other_groups.group1_) + (vec4<f32>(self_.e321_) * vec4<f32>(other.e23_, other.e31_, other.e12_, other.e1234_)), 
        /* e41, e42, e43 */ vec4<f32>((other.e2_ * self_.e412_) + (other.e42_ * self_.e12_) + (other.e31_ * self_.e43_) + (other.e412_ * self_.e2_) - (other.e3_ * self_.e431_) - (other.e43_ * self_.e31_) - (other.e12_ * self_.e42_) - (other.e431_ * self_.e3_), (other.e3_ * self_.e423_) + (other.e43_ * self_.e23_) + (other.e12_ * self_.e41_) + (other.e423_ * self_.e3_) - (other.e1_ * self_.e412_) - (other.e41_ * self_.e12_) - (other.e23_ * self_.e43_) - (other.e412_ * self_.e1_), (other.e1_ * self_.e431_) + (other.e41_ * self_.e31_) + (other.e23_ * self_.e42_) + (other.e431_ * self_.e1_) - (other.e2_ * self_.e423_) - (other.e42_ * self_.e23_) - (other.e31_ * self_.e41_) - (other.e423_ * self_.e2_), 0.0) + ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_) + ((vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_) + ((vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0)) + ((vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group2_) + ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group3_) + ((vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0)) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0)) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((other.e3_ * self_.e2_) + (other.e31_ * self_.e12_) - (other.e2_ * self_.e3_) - (other.e12_ * self_.e31_), (other.e1_ * self_.e3_) + (other.e12_ * self_.e23_) - (other.e3_ * self_.e1_) - (other.e23_ * self_.e12_), (other.e2_ * self_.e1_) + (other.e23_ * self_.e31_) - (other.e1_ * self_.e2_) - (other.e31_ * self_.e23_), 0.0) + ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_) + ((vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group3_) - ((vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0)) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e1234_ * self_.e1_) + (other.e3_ * self_.e42_) + (other.e4_ * self_.e23_) + (other.e41_ * self_.e321_) + (other.e42_ * self_.e3_) + (other.e23_ * self_.e4_) + (other.e31_ * self_.e412_) + (other.e431_ * self_.e12_) - (other.e1_ * self_.e1234_) - (other.e2_ * self_.e43_) - (other.e43_ * self_.e2_) - (other.e12_ * self_.e431_) - (other.e412_ * self_.e31_) - (other.e321_ * self_.e41_), (other.e1234_ * self_.e2_) + (other.e1_ * self_.e43_) + (other.e4_ * self_.e31_) + (other.e42_ * self_.e321_) + (other.e43_ * self_.e1_) + (other.e31_ * self_.e4_) + (other.e12_ * self_.e423_) + (other.e412_ * self_.e23_) - (other.e2_ * self_.e1234_) - (other.e3_ * self_.e41_) - (other.e41_ * self_.e3_) - (other.e23_ * self_.e412_) - (other.e423_ * self_.e12_) - (other.e321_ * self_.e42_), (other.e1234_ * self_.e3_) + (other.e2_ * self_.e41_) + (other.e4_ * self_.e12_) + (other.e41_ * self_.e2_) + (other.e43_ * self_.e321_) + (other.e23_ * self_.e431_) + (other.e12_ * self_.e4_) + (other.e423_ * self_.e31_) - (other.e1_ * self_.e42_) - (other.e3_ * self_.e1234_) - (other.e42_ * self_.e1_) - (other.e31_ * self_.e423_) - (other.e431_ * self_.e23_) - (other.e321_ * self_.e43_), -(other.e1_ * self_.e23_) - (other.e2_ * self_.e31_) - (other.e3_ * self_.e12_) - (other.e23_ * self_.e1_) - (other.e31_ * self_.e2_) - (other.e12_ * self_.e3_)) + (vec4<f32>(other.scalar) * self_groups.group4_) + (vec4<f32>(self_.scalar) * other_groups.group4_)
    ));
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self_));
}
fn multiVector_sandwich_origin(self_: MultiVector, other: Origin) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = origin_grouped(other);
    let geometric_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(1.0, self_.e321_ * other.e4_, 0.0, 0.0) * vec2<f32>(0.0, -1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, self_.scalar * other.e4_) * vec4<f32>(0.0, 0.0, 0.0, 1.0), 
        /* e41, e42, e43 */ (vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e23_ * other.e4_, self_.e31_ * other.e4_, self_.e12_ * other.e4_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self_));
}
fn multiVector_sandwich_plane(self_: MultiVector, other: Plane) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = plane_grouped(other);
    let geometric_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.e321_ * other.e321_, (self_.e1_ * other.e423_) + (self_.e2_ * other.e431_) + (self_.e3_ * other.e412_) + (self_.e4_ * other.e321_), 0.0, 0.0) * vec2<f32>(-1.0, 1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e23_ * other.e321_, self_.e31_ * other.e321_, self_.e12_ * other.e321_, -(self_.e1234_ * other.e321_) - (self_.e23_ * other.e423_) - (self_.e31_ * other.e431_) - (self_.e12_ * other.e412_)), 
        /* e41, e42, e43 */ vec4<f32>((self_.e2_ * other.e412_) - (self_.e3_ * other.e431_), (self_.e3_ * other.e423_) - (self_.e1_ * other.e412_), (self_.e1_ * other.e431_) - (self_.e2_ * other.e423_), 0.0) + ((vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0)) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0)), 
        /* e23, e31, e12 */ (vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0) * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.scalar * other.e423_) + (self_.e12_ * other.e431_) - (self_.e41_ * other.e321_) - (self_.e31_ * other.e412_), (self_.scalar * other.e431_) + (self_.e23_ * other.e412_) - (self_.e42_ * other.e321_) - (self_.e12_ * other.e423_), (self_.scalar * other.e412_) + (self_.e31_ * other.e423_) - (self_.e43_ * other.e321_) - (self_.e23_ * other.e431_), self_.scalar * other.e321_)
    ));
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self_));
}
fn multiVector_sandwich_point(self_: MultiVector, other: Point) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = point_grouped(other);
    let geometric_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((self_.e1_ * other.e1_) + (self_.e2_ * other.e2_) + (self_.e3_ * other.e3_), -(self_.e423_ * other.e1_) - (self_.e431_ * other.e2_) - (self_.e412_ * other.e3_) - (self_.e321_ * other.e4_), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e12_ * other.e2_) - (self_.e31_ * other.e3_), (self_.e23_ * other.e3_) - (self_.e12_ * other.e1_), (self_.e31_ * other.e1_) - (self_.e23_ * other.e2_), (self_.e41_ * other.e1_) + (self_.e42_ * other.e2_) + (self_.e43_ * other.e3_)) + (vec4<f32>(self_.scalar) * other_groups.group0_), 
        /* e41, e42, e43 */ vec4<f32>((self_.e412_ * other.e2_) - (self_.e431_ * other.e3_), (self_.e423_ * other.e3_) - (self_.e412_ * other.e1_), (self_.e431_ * other.e1_) - (self_.e423_ * other.e2_), 0.0) + ((vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0)) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((self_.e2_ * other.e3_) - (self_.e3_ * other.e2_), (self_.e3_ * other.e1_) - (self_.e1_ * other.e3_), (self_.e1_ * other.e2_) - (self_.e2_ * other.e1_), 0.0) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e42_ * other.e3_) + (self_.e23_ * other.e4_) - (self_.e1234_ * other.e1_) - (self_.e43_ * other.e2_), (self_.e43_ * other.e1_) + (self_.e31_ * other.e4_) - (self_.e1234_ * other.e2_) - (self_.e41_ * other.e3_), (self_.e41_ * other.e2_) + (self_.e12_ * other.e4_) - (self_.e1234_ * other.e3_) - (self_.e42_ * other.e1_), -(self_.e23_ * other.e1_) - (self_.e31_ * other.e2_) - (self_.e12_ * other.e3_))
    ));
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self_));
}
fn multiVector_sandwich_scalar(self_: MultiVector, other: Scalar) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = scalar_grouped(other);
    let geometric_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ (vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(other.scalar) * self_groups.group1_, 
        /* e41, e42, e43 */ (vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_, 
        /* e23, e31, e12 */ (vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other.scalar) * self_groups.group4_
    ));
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self_));
}
fn origin_sandwich_flector(self_: Origin, other: Flector) -> Flector {
    let geometric_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.e4_) * vec4<f32>(other.e1_, other.e2_, other.e3_, other.e321_), 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0)
    ));
    return motor_geometricProduct_origin(geometric_product, origin_reverse(self_));
}
fn origin_sandwich_line(self_: Origin, other: Line) -> AntiScalar {
    let geometric_product: Plane = plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(other.e23_ * self_.e4_, other.e31_ * self_.e4_, other.e12_ * self_.e4_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
    return plane_geometricProduct_origin(geometric_product, origin_reverse(self_));
}
fn origin_sandwich_motor(self_: Origin, other: Motor) -> Motor {
    let geometric_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, other.scalar * self_.e4_) * vec4<f32>(0.0, 0.0, 0.0, 1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e23_ * self_.e4_, other.e31_ * self_.e4_, other.e12_ * self_.e4_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
    return flector_geometricProduct_origin(geometric_product, origin_reverse(self_));
}
fn origin_sandwich_multiVector(self_: Origin, other: MultiVector) -> MultiVector {
    let geometric_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(1.0, other.e321_ * self_.e4_, 0.0, 0.0) * vec2<f32>(0.0, 1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, other.scalar * self_.e4_) * vec4<f32>(0.0, 0.0, 0.0, 1.0), 
        /* e41, e42, e43 */ (vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e23_ * self_.e4_, other.e31_ * self_.e4_, other.e12_ * self_.e4_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
    return multiVector_geometricProduct_origin(geometric_product, origin_reverse(self_));
}
fn origin_sandwich_point(self_: Origin, other: Point) -> Plane {
    let geometric_product: Line = line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0)
    ));
    return line_geometricProduct_origin(geometric_product, origin_reverse(self_));
}
fn plane_sandwich_antiScalar(self_: Plane, other: AntiScalar) -> AntiScalar {
    let self_groups = plane_grouped(self_);
    let geometric_product: Origin = Origin(other.e1234_ * self_.e321_);
    return origin_geometricProduct_plane(geometric_product, plane_reverse(self_));
}
fn plane_sandwich_dualNum(self_: Plane, other: DualNum) -> Motor {
    let self_groups = plane_grouped(self_);
    let geometric_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, other.e1234_ * self_.e321_) * vec4<f32>(0.0, 0.0, 0.0, 1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.scalar) * self_groups.group0_
    ));
    return flector_geometricProduct_plane(geometric_product, plane_reverse(self_));
}
fn plane_sandwich_flector(self_: Plane, other: Flector) -> Flector {
    let self_groups = plane_grouped(self_);
    let geometric_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other.e2_ * self_.e412_) + (other.e321_ * self_.e423_) - (other.e3_ * self_.e431_), (other.e3_ * self_.e423_) + (other.e321_ * self_.e431_) - (other.e1_ * self_.e412_), (other.e1_ * self_.e431_) + (other.e321_ * self_.e412_) - (other.e2_ * self_.e423_), -(other.e1_ * self_.e423_) - (other.e2_ * self_.e431_) - (other.e3_ * self_.e412_)) - (vec4<f32>(self_.e321_) * vec4<f32>(other.e423_, other.e431_, other.e412_, other.e4_)), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.e321_) * vec4<f32>(other.e1_, other.e2_, other.e3_, other.e321_) * vec4<f32>(-1.0)
    ));
    return motor_geometricProduct_plane(geometric_product, plane_reverse(self_));
}
fn plane_sandwich_horizon(self_: Plane, other: Horizon) -> Flector {
    let self_groups = plane_grouped(self_);
    let geometric_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e321_ * self_.e423_, other.e321_ * self_.e431_, other.e321_ * self_.e412_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(1.0, 1.0, 1.0, other.e321_ * self_.e321_) * vec4<f32>(0.0, 0.0, 0.0, -1.0)
    ));
    return motor_geometricProduct_plane(geometric_product, plane_reverse(self_));
}
fn plane_sandwich_line(self_: Plane, other: Line) -> Motor {
    let self_groups = plane_grouped(self_);
    let geometric_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e23_ * self_.e321_, other.e31_ * self_.e321_, other.e12_ * self_.e321_, -(other.e23_ * self_.e423_) - (other.e31_ * self_.e431_) - (other.e12_ * self_.e412_)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e41_ * self_.e321_) + (other.e31_ * self_.e412_) - (other.e12_ * self_.e431_), (other.e42_ * self_.e321_) + (other.e12_ * self_.e423_) - (other.e23_ * self_.e412_), (other.e43_ * self_.e321_) + (other.e23_ * self_.e431_) - (other.e31_ * self_.e423_), 0.0)
    ));
    return flector_geometricProduct_plane(geometric_product, plane_reverse(self_));
}
fn plane_sandwich_motor(self_: Plane, other: Motor) -> Motor {
    let self_groups = plane_grouped(self_);
    let geometric_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e23_ * self_.e321_, other.e31_ * self_.e321_, other.e12_ * self_.e321_, (other.e1234_ * self_.e321_) - (other.e23_ * self_.e423_) - (other.e31_ * self_.e431_) - (other.e12_ * self_.e412_)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e41_ * self_.e321_) + (other.e31_ * self_.e412_) + (other.scalar * self_.e423_) - (other.e12_ * self_.e431_), (other.e42_ * self_.e321_) + (other.e12_ * self_.e423_) + (other.scalar * self_.e431_) - (other.e23_ * self_.e412_), (other.e43_ * self_.e321_) + (other.e23_ * self_.e431_) + (other.scalar * self_.e412_) - (other.e31_ * self_.e423_), other.scalar * self_.e321_)
    ));
    return flector_geometricProduct_plane(geometric_product, plane_reverse(self_));
}
fn plane_sandwich_multiVector(self_: Plane, other: MultiVector) -> MultiVector {
    let self_groups = plane_grouped(self_);
    let geometric_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.e321_ * self_.e321_, -(other.e1_ * self_.e423_) - (other.e2_ * self_.e431_) - (other.e3_ * self_.e412_) - (other.e4_ * self_.e321_), 0.0, 0.0) * vec2<f32>(-1.0, 1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(other.e23_ * self_.e321_, other.e31_ * self_.e321_, other.e12_ * self_.e321_, (other.e1234_ * self_.e321_) - (other.e23_ * self_.e423_) - (other.e31_ * self_.e431_) - (other.e12_ * self_.e412_)), 
        /* e41, e42, e43 */ vec4<f32>((other.e2_ * self_.e412_) - (other.e3_ * self_.e431_), (other.e3_ * self_.e423_) - (other.e1_ * self_.e412_), (other.e1_ * self_.e431_) - (other.e2_ * self_.e423_), 0.0) + ((vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0)) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0)), 
        /* e23, e31, e12 */ (vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0) * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.scalar * self_.e423_) + (other.e41_ * self_.e321_) + (other.e31_ * self_.e412_) - (other.e12_ * self_.e431_), (other.scalar * self_.e431_) + (other.e42_ * self_.e321_) + (other.e12_ * self_.e423_) - (other.e23_ * self_.e412_), (other.scalar * self_.e412_) + (other.e43_ * self_.e321_) + (other.e23_ * self_.e431_) - (other.e31_ * self_.e423_), other.scalar * self_.e321_)
    ));
    return multiVector_geometricProduct_plane(geometric_product, plane_reverse(self_));
}
fn plane_sandwich_origin(self_: Plane, other: Origin) -> Origin {
    let self_groups = plane_grouped(self_);
    let geometric_product: AntiScalar = AntiScalar(other.e4_ * self_.e321_ * -1.0);
    return antiScalar_geometricProduct_plane(geometric_product, plane_reverse(self_));
}
fn plane_sandwich_plane(self_: Plane, other: Plane) -> Flector {
    let self_groups = plane_grouped(self_);
    let geometric_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other.e321_ * self_.e423_) - (other.e423_ * self_.e321_), (other.e321_ * self_.e431_) - (other.e431_ * self_.e321_), (other.e321_ * self_.e412_) - (other.e412_ * self_.e321_), 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(1.0, 1.0, 1.0, other.e321_ * self_.e321_) * vec4<f32>(0.0, 0.0, 0.0, -1.0)
    ));
    return motor_geometricProduct_plane(geometric_product, plane_reverse(self_));
}
fn plane_sandwich_point(self_: Plane, other: Point) -> Flector {
    let self_groups = plane_grouped(self_);
    let geometric_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((self_.e412_ * other.e2_) - (self_.e431_ * other.e3_), (self_.e423_ * other.e3_) - (self_.e412_ * other.e1_), (self_.e431_ * other.e1_) - (self_.e423_ * other.e2_), -(self_.e423_ * other.e1_) - (self_.e431_ * other.e2_) - (self_.e412_ * other.e3_) - (self_.e321_ * other.e4_)), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.e321_ * other.e1_, self_.e321_ * other.e2_, self_.e321_ * other.e3_, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0)
    ));
    return motor_geometricProduct_plane(geometric_product, plane_reverse(self_));
}
fn plane_sandwich_scalar(self_: Plane, other: Scalar) -> Motor {
    let self_groups = plane_grouped(self_);
    let geometric_product: Plane = plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(other.scalar) * self_groups.group0_
    ));
    return plane_geometricProduct_plane(geometric_product, plane_reverse(self_));
}
fn point_sandwich_antiScalar(self_: Point, other: AntiScalar) -> Motor {
    let geometric_product: Plane = plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(other.e1234_ * self_.e1_, other.e1234_ * self_.e2_, other.e1234_ * self_.e3_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
    return plane_geometricProduct_point(geometric_product, point_reverse(self_));
}
fn point_sandwich_dualNum(self_: Point, other: DualNum) -> Motor {
    let self_groups = point_grouped(self_);
    let geometric_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.scalar) * self_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e1234_ * self_.e1_, other.e1234_ * self_.e2_, other.e1234_ * self_.e3_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
    return flector_geometricProduct_point(geometric_product, point_reverse(self_));
}
fn point_sandwich_flector(self_: Point, other: Flector) -> Flector {
    let other_groups = flector_grouped(other);
    let geometric_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other.e412_ * self_.e2_) - (other.e4_ * self_.e1_) - (other.e431_ * self_.e3_), (other.e423_ * self_.e3_) - (other.e4_ * self_.e2_) - (other.e412_ * self_.e1_), (other.e431_ * self_.e1_) - (other.e4_ * self_.e3_) - (other.e423_ * self_.e2_), (other.e423_ * self_.e1_) + (other.e431_ * self_.e2_) + (other.e412_ * self_.e3_)) + (vec4<f32>(self_.e4_) * vec4<f32>(other.e1_, other.e2_, other.e3_, other.e321_)), 
        /* e23, e31, e12, scalar */ vec4<f32>((other.e3_ * self_.e2_) - (other.e2_ * self_.e3_) - (other.e321_ * self_.e1_), (other.e1_ * self_.e3_) - (other.e3_ * self_.e1_) - (other.e321_ * self_.e2_), (other.e2_ * self_.e1_) - (other.e1_ * self_.e2_) - (other.e321_ * self_.e3_), (other.e1_ * self_.e1_) + (other.e2_ * self_.e2_) + (other.e3_ * self_.e3_))
    ));
    return motor_geometricProduct_point(geometric_product, point_reverse(self_));
}
fn point_sandwich_horizon(self_: Point, other: Horizon) -> Flector {
    let other_groups = horizon_grouped(other);
    let geometric_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(1.0, 1.0, 1.0, other.e321_ * self_.e4_) * vec4<f32>(0.0, 0.0, 0.0, 1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(other.e321_ * self_.e1_, other.e321_ * self_.e2_, other.e321_ * self_.e3_, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0)
    ));
    return motor_geometricProduct_point(geometric_product, point_reverse(self_));
}
fn point_sandwich_line(self_: Point, other: Line) -> Motor {
    let geometric_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other.e31_ * self_.e3_) - (other.e12_ * self_.e2_), (other.e12_ * self_.e1_) - (other.e23_ * self_.e3_), (other.e23_ * self_.e2_) - (other.e31_ * self_.e1_), -(other.e41_ * self_.e1_) - (other.e42_ * self_.e2_) - (other.e43_ * self_.e3_)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e42_ * self_.e3_) + (other.e23_ * self_.e4_) - (other.e43_ * self_.e2_), (other.e43_ * self_.e1_) + (other.e31_ * self_.e4_) - (other.e41_ * self_.e3_), (other.e41_ * self_.e2_) + (other.e12_ * self_.e4_) - (other.e42_ * self_.e1_), -(other.e23_ * self_.e1_) - (other.e31_ * self_.e2_) - (other.e12_ * self_.e3_))
    ));
    return flector_geometricProduct_point(geometric_product, point_reverse(self_));
}
fn point_sandwich_motor(self_: Point, other: Motor) -> Motor {
    let self_groups = point_grouped(self_);
    let geometric_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other.e31_ * self_.e3_) - (other.e12_ * self_.e2_), (other.e12_ * self_.e1_) - (other.e23_ * self_.e3_), (other.e23_ * self_.e2_) - (other.e31_ * self_.e1_), -(other.e41_ * self_.e1_) - (other.e42_ * self_.e2_) - (other.e43_ * self_.e3_)) + (vec4<f32>(other.scalar) * self_groups.group0_), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e42_ * self_.e3_) + (other.e1234_ * self_.e1_) + (other.e23_ * self_.e4_) - (other.e43_ * self_.e2_), (other.e43_ * self_.e1_) + (other.e1234_ * self_.e2_) + (other.e31_ * self_.e4_) - (other.e41_ * self_.e3_), (other.e41_ * self_.e2_) + (other.e1234_ * self_.e3_) + (other.e12_ * self_.e4_) - (other.e42_ * self_.e1_), -(other.e23_ * self_.e1_) - (other.e31_ * self_.e2_) - (other.e12_ * self_.e3_))
    ));
    return flector_geometricProduct_point(geometric_product, point_reverse(self_));
}
fn point_sandwich_multiVector(self_: Point, other: MultiVector) -> MultiVector {
    let self_groups = point_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let geometric_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, other.e321_ * self_.e4_, 0.0, 0.0) + ((vec4<f32>(self_.e1_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e1_, other.e423_, 0.0, 0.0)) + ((vec4<f32>(self_.e2_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e2_, other.e431_, 0.0, 0.0)) + ((vec4<f32>(self_.e3_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e3_, other.e412_, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((other.e31_ * self_.e3_) - (other.e12_ * self_.e2_), (other.e12_ * self_.e1_) - (other.e23_ * self_.e3_), (other.e23_ * self_.e2_) - (other.e31_ * self_.e1_), -(other.e41_ * self_.e1_) - (other.e42_ * self_.e2_) - (other.e43_ * self_.e3_)) + (vec4<f32>(other.scalar) * self_groups.group0_), 
        /* e41, e42, e43 */ vec4<f32>((other.e412_ * self_.e2_) - (other.e431_ * self_.e3_), (other.e423_ * self_.e3_) - (other.e412_ * self_.e1_), (other.e431_ * self_.e1_) - (other.e423_ * self_.e2_), 0.0) + ((vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0)) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((other.e3_ * self_.e2_) - (other.e2_ * self_.e3_), (other.e1_ * self_.e3_) - (other.e3_ * self_.e1_), (other.e2_ * self_.e1_) - (other.e1_ * self_.e2_), 0.0) - ((vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e1234_ * self_.e1_) + (other.e42_ * self_.e3_) + (other.e23_ * self_.e4_) - (other.e43_ * self_.e2_), (other.e1234_ * self_.e2_) + (other.e43_ * self_.e1_) + (other.e31_ * self_.e4_) - (other.e41_ * self_.e3_), (other.e1234_ * self_.e3_) + (other.e41_ * self_.e2_) + (other.e12_ * self_.e4_) - (other.e42_ * self_.e1_), -(other.e23_ * self_.e1_) - (other.e31_ * self_.e2_) - (other.e12_ * self_.e3_))
    ));
    return multiVector_geometricProduct_point(geometric_product, point_reverse(self_));
}
fn point_sandwich_origin(self_: Point, other: Origin) -> Flector {
    let geometric_product: Line = line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0)
    ));
    return line_geometricProduct_point(geometric_product, point_reverse(self_));
}
fn point_sandwich_plane(self_: Point, other: Plane) -> Flector {
    let other_groups = plane_grouped(other);
    let geometric_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other.e412_ * self_.e2_) - (other.e431_ * self_.e3_), (other.e423_ * self_.e3_) - (other.e412_ * self_.e1_), (other.e431_ * self_.e1_) - (other.e423_ * self_.e2_), (other.e423_ * self_.e1_) + (other.e431_ * self_.e2_) + (other.e412_ * self_.e3_) + (other.e321_ * self_.e4_)), 
        /* e23, e31, e12, scalar */ vec4<f32>(other.e321_ * self_.e1_, other.e321_ * self_.e2_, other.e321_ * self_.e3_, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0)
    ));
    return motor_geometricProduct_point(geometric_product, point_reverse(self_));
}
fn point_sandwich_point(self_: Point, other: Point) -> Flector {
    let other_groups = point_grouped(other);
    let geometric_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other.e1_ * self_.e4_) - (other.e4_ * self_.e1_), (other.e2_ * self_.e4_) - (other.e4_ * self_.e2_), (other.e3_ * self_.e4_) - (other.e4_ * self_.e3_), 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>((other.e3_ * self_.e2_) - (other.e2_ * self_.e3_), (other.e1_ * self_.e3_) - (other.e3_ * self_.e1_), (other.e2_ * self_.e1_) - (other.e1_ * self_.e2_), (other.e1_ * self_.e1_) + (other.e2_ * self_.e2_) + (other.e3_ * self_.e3_))
    ));
    return motor_geometricProduct_point(geometric_product, point_reverse(self_));
}
fn point_sandwich_scalar(self_: Point, other: Scalar) -> Motor {
    let self_groups = point_grouped(self_);
    let geometric_product: Point = point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.scalar) * self_groups.group0_
    ));
    return point_geometricProduct_point(geometric_product, point_reverse(self_));
}
fn scalar_sandwich_antiScalar(self_: Scalar, other: AntiScalar) -> AntiScalar {
    let geometric_product: AntiScalar = AntiScalar(other.e1234_ * self_.scalar);
    return antiScalar_geometricProduct_scalar(geometric_product, scalar_reverse(self_));
}
fn scalar_sandwich_dualNum(self_: Scalar, other: DualNum) -> DualNum {
    let self_groups = scalar_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let geometric_product: DualNum = dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ (vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * other_groups.group0_
    ));
    return dualNum_geometricProduct_scalar(geometric_product, scalar_reverse(self_));
}
fn scalar_sandwich_flector(self_: Scalar, other: Flector) -> Flector {
    let self_groups = scalar_grouped(self_);
    let other_groups = flector_grouped(other);
    let geometric_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.scalar) * other_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.scalar) * other_groups.group1_
    ));
    return flector_geometricProduct_scalar(geometric_product, scalar_reverse(self_));
}
fn scalar_sandwich_horizon(self_: Scalar, other: Horizon) -> Horizon {
    let geometric_product: Horizon = Horizon(other.e321_ * self_.scalar);
    return horizon_geometricProduct_scalar(geometric_product, scalar_reverse(self_));
}
fn scalar_sandwich_line(self_: Scalar, other: Line) -> Line {
    let self_groups = scalar_grouped(self_);
    let other_groups = line_grouped(other);
    let geometric_product: Line = line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group1_
    ));
    return line_geometricProduct_scalar(geometric_product, scalar_reverse(self_));
}
fn scalar_sandwich_motor(self_: Scalar, other: Motor) -> Motor {
    let self_groups = scalar_grouped(self_);
    let other_groups = motor_grouped(other);
    let geometric_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.scalar) * other_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.scalar) * other_groups.group1_
    ));
    return motor_geometricProduct_scalar(geometric_product, scalar_reverse(self_));
}
fn scalar_sandwich_multiVector(self_: Scalar, other: MultiVector) -> MultiVector {
    let self_groups = scalar_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let geometric_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ (vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * other_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.scalar) * other_groups.group1_, 
        /* e41, e42, e43 */ (vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group2_, 
        /* e23, e31, e12 */ (vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.scalar) * other_groups.group4_
    ));
    return multiVector_geometricProduct_scalar(geometric_product, scalar_reverse(self_));
}
fn scalar_sandwich_origin(self_: Scalar, other: Origin) -> Origin {
    let geometric_product: Origin = Origin(other.e4_ * self_.scalar);
    return origin_geometricProduct_scalar(geometric_product, scalar_reverse(self_));
}
fn scalar_sandwich_plane(self_: Scalar, other: Plane) -> Plane {
    let self_groups = scalar_grouped(self_);
    let other_groups = plane_grouped(other);
    let geometric_product: Plane = plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(self_.scalar) * other_groups.group0_
    ));
    return plane_geometricProduct_scalar(geometric_product, scalar_reverse(self_));
}
fn scalar_sandwich_point(self_: Scalar, other: Point) -> Point {
    let self_groups = scalar_grouped(self_);
    let other_groups = point_grouped(other);
    let geometric_product: Point = point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.scalar) * other_groups.group0_
    ));
    return point_geometricProduct_scalar(geometric_product, scalar_reverse(self_));
}
fn scalar_sandwich_scalar(self_: Scalar, other: Scalar) -> Scalar {
    let geometric_product: Scalar = Scalar(other.scalar * self_.scalar);
    return scalar_geometricProduct_scalar(geometric_product, scalar_reverse(self_));
}
fn scalar_squareRoot(self_: Scalar) -> Scalar {
    return Scalar(pow(self_.scalar, 0.5));
}
fn antiScalar_sub_antiScalar(self_: AntiScalar, other: AntiScalar) -> AntiScalar {
    return AntiScalar(self_.e1234_ - other.e1234_);
}
fn antiScalar_sub_dualNum(self_: AntiScalar, other: DualNum) -> DualNum {
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar, self_.e1234_ - other.e1234_, 0.0, 0.0) * vec2<f32>(-1.0, 1.0)
    ));
}
fn antiScalar_sub_flector(self_: AntiScalar, other: Flector) -> MultiVector {
    let other_groups = flector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, self_.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ other_groups.group0_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ other_groups.group1_ * vec4<f32>(-1.0)
    ));
}
fn antiScalar_sub_horizon(self_: AntiScalar, other: Horizon) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, self_.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0, 1.0, 1.0, other.e321_) * vec4<f32>(0.0, 0.0, 0.0, -1.0)
    ));
}
fn antiScalar_sub_line(self_: AntiScalar, other: Line) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e41_, other.e42_, other.e43_, self_.e1234_) * vec4<f32>(-1.0, -1.0, -1.0, 1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(other.e23_, other.e31_, other.e12_, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0)
    ));
}
fn antiScalar_sub_motor(self_: AntiScalar, other: Motor) -> Motor {
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e41_, other.e42_, other.e43_, self_.e1234_ - other.e1234_) * vec4<f32>(-1.0, -1.0, -1.0, 1.0), 
        /* e23, e31, e12, scalar */ other_groups.group1_ * vec4<f32>(-1.0)
    ));
}
fn antiScalar_sub_multiVector(self_: AntiScalar, other: MultiVector) -> MultiVector {
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar, self_.e1234_ - other.e1234_, 0.0, 0.0) * vec2<f32>(-1.0, 1.0), 
        /* e1, e2, e3, e4 */ other_groups.group1_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ other_groups.group2_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ other_groups.group3_ * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ other_groups.group4_ * vec4<f32>(-1.0)
    ));
}
fn antiScalar_sub_origin(self_: AntiScalar, other: Origin) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, self_.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, other.e4_) * vec4<f32>(0.0, 0.0, 0.0, -1.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn antiScalar_sub_plane(self_: AntiScalar, other: Plane) -> MultiVector {
    let other_groups = plane_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, self_.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ other_groups.group0_ * vec4<f32>(-1.0)
    ));
}
fn antiScalar_sub_point(self_: AntiScalar, other: Point) -> MultiVector {
    let other_groups = point_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, self_.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ other_groups.group0_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn antiScalar_sub_scalar(self_: AntiScalar, other: Scalar) -> DualNum {
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar, self_.e1234_, 0.0, 0.0) * vec2<f32>(-1.0, 1.0)
    ));
}
fn dualNum_sub_antiScalar(self_: DualNum, other: AntiScalar) -> DualNum {
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar, self_.e1234_ - other.e1234_, 0.0, 0.0)
    ));
}
fn dualNum_sub_dualNum(self_: DualNum, other: DualNum) -> DualNum {
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar - other.scalar, self_.e1234_ - other.e1234_, 0.0, 0.0)
    ));
}
fn dualNum_sub_flector(self_: DualNum, other: Flector) -> MultiVector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = flector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_groups.group0_, 
        /* e1, e2, e3, e4 */ other_groups.group0_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ other_groups.group1_ * vec4<f32>(-1.0)
    ));
}
fn dualNum_sub_horizon(self_: DualNum, other: Horizon) -> MultiVector {
    let self_groups = dualNum_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0, 1.0, 1.0, other.e321_) * vec4<f32>(0.0, 0.0, 0.0, -1.0)
    ));
}
fn dualNum_sub_line(self_: DualNum, other: Line) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e41_, other.e42_, other.e43_, self_.e1234_) * vec4<f32>(-1.0, -1.0, -1.0, 1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(other.e23_, other.e31_, other.e12_, self_.scalar) * vec4<f32>(-1.0, -1.0, -1.0, 1.0)
    ));
}
fn dualNum_sub_motor(self_: DualNum, other: Motor) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e41_, other.e42_, other.e43_, self_.e1234_ - other.e1234_) * vec4<f32>(-1.0, -1.0, -1.0, 1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(other.e23_, other.e31_, other.e12_, self_.scalar - other.scalar) * vec4<f32>(-1.0, -1.0, -1.0, 1.0)
    ));
}
fn dualNum_sub_multiVector(self_: DualNum, other: MultiVector) -> MultiVector {
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar - other.scalar, self_.e1234_ - other.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ other_groups.group1_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ other_groups.group2_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ other_groups.group3_ * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ other_groups.group4_ * vec4<f32>(-1.0)
    ));
}
fn dualNum_sub_origin(self_: DualNum, other: Origin) -> MultiVector {
    let self_groups = dualNum_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, other.e4_) * vec4<f32>(0.0, 0.0, 0.0, -1.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn dualNum_sub_plane(self_: DualNum, other: Plane) -> MultiVector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = plane_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ other_groups.group0_ * vec4<f32>(-1.0)
    ));
}
fn dualNum_sub_point(self_: DualNum, other: Point) -> MultiVector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = point_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_groups.group0_, 
        /* e1, e2, e3, e4 */ other_groups.group0_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn dualNum_sub_scalar(self_: DualNum, other: Scalar) -> DualNum {
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar - other.scalar, self_.e1234_, 0.0, 0.0)
    ));
}
fn flector_sub_antiScalar(self_: Flector, other: AntiScalar) -> MultiVector {
    let self_groups = flector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(1.0, other.e1234_, 0.0, 0.0) * vec2<f32>(0.0, -1.0), 
        /* e1, e2, e3, e4 */ self_groups.group0_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ self_groups.group1_
    ));
}
fn flector_sub_dualNum(self_: Flector, other: DualNum) -> MultiVector {
    let self_groups = flector_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_ * vec2<f32>(-1.0), 
        /* e1, e2, e3, e4 */ self_groups.group0_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ self_groups.group1_
    ));
}
fn flector_sub_flector(self_: Flector, other: Flector) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e1_ - other.e1_, self_.e2_ - other.e2_, self_.e3_ - other.e3_, self_.e4_ - other.e4_), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e423_ - other.e423_, self_.e431_ - other.e431_, self_.e412_ - other.e412_, self_.e321_ - other.e321_)
    ));
}
fn flector_sub_horizon(self_: Flector, other: Horizon) -> Flector {
    let self_groups = flector_grouped(self_);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ self_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e423_, self_.e431_, self_.e412_, self_.e321_ - other.e321_)
    ));
}
fn flector_sub_line(self_: Flector, other: Line) -> MultiVector {
    let self_groups = flector_grouped(self_);
    let other_groups = line_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ self_groups.group0_, 
        /* e41, e42, e43 */ other_groups.group0_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ other_groups.group1_ * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ self_groups.group1_
    ));
}
fn flector_sub_motor(self_: Flector, other: Motor) -> MultiVector {
    let self_groups = flector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar, other.e1234_, 0.0, 0.0) * vec2<f32>(-1.0), 
        /* e1, e2, e3, e4 */ self_groups.group0_, 
        /* e41, e42, e43 */ vec4<f32>(other.e41_, other.e42_, other.e43_, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>(other.e23_, other.e31_, other.e12_, 0.0) * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ self_groups.group1_
    ));
}
fn flector_sub_multiVector(self_: Flector, other: MultiVector) -> MultiVector {
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_ * vec2<f32>(-1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e1_ - other.e1_, self_.e2_ - other.e2_, self_.e3_ - other.e3_, self_.e4_ - other.e4_), 
        /* e41, e42, e43 */ other_groups.group2_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ other_groups.group3_ * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e423_ - other.e423_, self_.e431_ - other.e431_, self_.e412_ - other.e412_, self_.e321_ - other.e321_)
    ));
}
fn flector_sub_origin(self_: Flector, other: Origin) -> Flector {
    let self_groups = flector_grouped(self_);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e1_, self_.e2_, self_.e3_, self_.e4_ - other.e4_), 
        /* e423, e431, e412, e321 */ self_groups.group1_
    ));
}
fn flector_sub_plane(self_: Flector, other: Plane) -> Flector {
    let self_groups = flector_grouped(self_);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ self_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e423_ - other.e423_, self_.e431_ - other.e431_, self_.e412_ - other.e412_, self_.e321_ - other.e321_)
    ));
}
fn flector_sub_point(self_: Flector, other: Point) -> Flector {
    let self_groups = flector_grouped(self_);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e1_ - other.e1_, self_.e2_ - other.e2_, self_.e3_ - other.e3_, self_.e4_ - other.e4_), 
        /* e423, e431, e412, e321 */ self_groups.group1_
    ));
}
fn flector_sub_scalar(self_: Flector, other: Scalar) -> MultiVector {
    let self_groups = flector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar, 1.0, 0.0, 0.0) * vec2<f32>(-1.0, 0.0), 
        /* e1, e2, e3, e4 */ self_groups.group0_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ self_groups.group1_
    ));
}
fn horizon_sub_antiScalar(self_: Horizon, other: AntiScalar) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(1.0, other.e1234_, 0.0, 0.0) * vec2<f32>(0.0, -1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.e321_)
    ));
}
fn horizon_sub_dualNum(self_: Horizon, other: DualNum) -> MultiVector {
    let other_groups = dualNum_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_ * vec2<f32>(-1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.e321_)
    ));
}
fn horizon_sub_flector(self_: Horizon, other: Flector) -> Flector {
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ other_groups.group0_ * vec4<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e423_, other.e431_, other.e412_, self_.e321_ - other.e321_) * vec4<f32>(-1.0, -1.0, -1.0, 1.0)
    ));
}
fn horizon_sub_horizon(self_: Horizon, other: Horizon) -> Horizon {
    return Horizon(self_.e321_ - other.e321_);
}
fn horizon_sub_line(self_: Horizon, other: Line) -> MultiVector {
    let other_groups = line_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ other_groups.group0_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ other_groups.group1_ * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.e321_)
    ));
}
fn horizon_sub_motor(self_: Horizon, other: Motor) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar, other.e1234_, 0.0, 0.0) * vec2<f32>(-1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(other.e41_, other.e42_, other.e43_, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>(other.e23_, other.e31_, other.e12_, 0.0) * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.e321_)
    ));
}
fn horizon_sub_multiVector(self_: Horizon, other: MultiVector) -> MultiVector {
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_ * vec2<f32>(-1.0), 
        /* e1, e2, e3, e4 */ other_groups.group1_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ other_groups.group2_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ other_groups.group3_ * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e423_, other.e431_, other.e412_, self_.e321_ - other.e321_) * vec4<f32>(-1.0, -1.0, -1.0, 1.0)
    ));
}
fn horizon_sub_origin(self_: Horizon, other: Origin) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, other.e4_) * vec4<f32>(0.0, 0.0, 0.0, -1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.e321_)
    ));
}
fn horizon_sub_plane(self_: Horizon, other: Plane) -> Plane {
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(other.e423_, other.e431_, other.e412_, self_.e321_ - other.e321_) * vec4<f32>(-1.0, -1.0, -1.0, 1.0)
    ));
}
fn horizon_sub_point(self_: Horizon, other: Point) -> Flector {
    let other_groups = point_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ other_groups.group0_ * vec4<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.e321_)
    ));
}
fn horizon_sub_scalar(self_: Horizon, other: Scalar) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar, 1.0, 0.0, 0.0) * vec2<f32>(-1.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.e321_)
    ));
}
fn line_sub_antiScalar(self_: Line, other: AntiScalar) -> Motor {
    let self_groups = line_grouped(self_);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_groups.group0_.xyz.xyz, other.e1234_ * -1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_groups.group1_.xyz.xyz, 0.0)
    ));
}
fn line_sub_dualNum(self_: Line, other: DualNum) -> Motor {
    let self_groups = line_grouped(self_);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_groups.group0_.xyz.xyz, other.e1234_ * -1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_groups.group1_.xyz.xyz, other.scalar * -1.0)
    ));
}
fn line_sub_flector(self_: Line, other: Flector) -> MultiVector {
    let self_groups = line_grouped(self_);
    let other_groups = flector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ other_groups.group0_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ self_groups.group0_, 
        /* e23, e31, e12 */ self_groups.group1_, 
        /* e423, e431, e412, e321 */ other_groups.group1_ * vec4<f32>(-1.0)
    ));
}
fn line_sub_horizon(self_: Line, other: Horizon) -> MultiVector {
    let self_groups = line_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ self_groups.group0_, 
        /* e23, e31, e12 */ self_groups.group1_, 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0, 1.0, 1.0, other.e321_) * vec4<f32>(0.0, 0.0, 0.0, -1.0)
    ));
}
fn line_sub_line(self_: Line, other: Line) -> Line {
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(self_.e41_ - other.e41_, self_.e42_ - other.e42_, self_.e43_ - other.e43_, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(self_.e23_ - other.e23_, self_.e31_ - other.e31_, self_.e12_ - other.e12_, 0.0)
    ));
}
fn line_sub_motor(self_: Line, other: Motor) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.e41_ - other.e41_, self_.e42_ - other.e42_, self_.e43_ - other.e43_, other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, -1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.e23_ - other.e23_, self_.e31_ - other.e31_, self_.e12_ - other.e12_, other.scalar) * vec4<f32>(1.0, 1.0, 1.0, -1.0)
    ));
}
fn line_sub_multiVector(self_: Line, other: MultiVector) -> MultiVector {
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_ * vec2<f32>(-1.0), 
        /* e1, e2, e3, e4 */ other_groups.group1_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ vec4<f32>(self_.e41_ - other.e41_, self_.e42_ - other.e42_, self_.e43_ - other.e43_, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(self_.e23_ - other.e23_, self_.e31_ - other.e31_, self_.e12_ - other.e12_, 0.0), 
        /* e423, e431, e412, e321 */ other_groups.group4_ * vec4<f32>(-1.0)
    ));
}
fn line_sub_origin(self_: Line, other: Origin) -> MultiVector {
    let self_groups = line_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, other.e4_) * vec4<f32>(0.0, 0.0, 0.0, -1.0), 
        /* e41, e42, e43 */ self_groups.group0_, 
        /* e23, e31, e12 */ self_groups.group1_, 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn line_sub_plane(self_: Line, other: Plane) -> MultiVector {
    let self_groups = line_grouped(self_);
    let other_groups = plane_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ self_groups.group0_, 
        /* e23, e31, e12 */ self_groups.group1_, 
        /* e423, e431, e412, e321 */ other_groups.group0_ * vec4<f32>(-1.0)
    ));
}
fn line_sub_point(self_: Line, other: Point) -> MultiVector {
    let self_groups = line_grouped(self_);
    let other_groups = point_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ other_groups.group0_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ self_groups.group0_, 
        /* e23, e31, e12 */ self_groups.group1_, 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn line_sub_scalar(self_: Line, other: Scalar) -> Motor {
    let self_groups = line_grouped(self_);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_groups.group0_.xyz.xyz, 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_groups.group1_.xyz.xyz, other.scalar * -1.0)
    ));
}
fn motor_sub_antiScalar(self_: Motor, other: AntiScalar) -> Motor {
    let self_groups = motor_grouped(self_);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.e41_, self_.e42_, self_.e43_, self_.e1234_ - other.e1234_), 
        /* e23, e31, e12, scalar */ self_groups.group1_
    ));
}
fn motor_sub_dualNum(self_: Motor, other: DualNum) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.e41_, self_.e42_, self_.e43_, self_.e1234_ - other.e1234_), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.e23_, self_.e31_, self_.e12_, self_.scalar - other.scalar)
    ));
}
fn motor_sub_flector(self_: Motor, other: Flector) -> MultiVector {
    let other_groups = flector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar, self_.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ other_groups.group0_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ vec4<f32>(self_.e41_, self_.e42_, self_.e43_, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(self_.e23_, self_.e31_, self_.e12_, 0.0), 
        /* e423, e431, e412, e321 */ other_groups.group1_ * vec4<f32>(-1.0)
    ));
}
fn motor_sub_horizon(self_: Motor, other: Horizon) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar, self_.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(self_.e41_, self_.e42_, self_.e43_, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(self_.e23_, self_.e31_, self_.e12_, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0, 1.0, 1.0, other.e321_) * vec4<f32>(0.0, 0.0, 0.0, -1.0)
    ));
}
fn motor_sub_line(self_: Motor, other: Line) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.e41_ - other.e41_, self_.e42_ - other.e42_, self_.e43_ - other.e43_, self_.e1234_), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.e23_ - other.e23_, self_.e31_ - other.e31_, self_.e12_ - other.e12_, self_.scalar)
    ));
}
fn motor_sub_motor(self_: Motor, other: Motor) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.e41_ - other.e41_, self_.e42_ - other.e42_, self_.e43_ - other.e43_, self_.e1234_ - other.e1234_), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.e23_ - other.e23_, self_.e31_ - other.e31_, self_.e12_ - other.e12_, self_.scalar - other.scalar)
    ));
}
fn motor_sub_multiVector(self_: Motor, other: MultiVector) -> MultiVector {
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar - other.scalar, self_.e1234_ - other.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ other_groups.group1_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ vec4<f32>(self_.e41_ - other.e41_, self_.e42_ - other.e42_, self_.e43_ - other.e43_, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(self_.e23_ - other.e23_, self_.e31_ - other.e31_, self_.e12_ - other.e12_, 0.0), 
        /* e423, e431, e412, e321 */ other_groups.group4_ * vec4<f32>(-1.0)
    ));
}
fn motor_sub_origin(self_: Motor, other: Origin) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar, self_.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, other.e4_) * vec4<f32>(0.0, 0.0, 0.0, -1.0), 
        /* e41, e42, e43 */ vec4<f32>(self_.e41_, self_.e42_, self_.e43_, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(self_.e23_, self_.e31_, self_.e12_, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn motor_sub_plane(self_: Motor, other: Plane) -> MultiVector {
    let other_groups = plane_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar, self_.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(self_.e41_, self_.e42_, self_.e43_, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(self_.e23_, self_.e31_, self_.e12_, 0.0), 
        /* e423, e431, e412, e321 */ other_groups.group0_ * vec4<f32>(-1.0)
    ));
}
fn motor_sub_point(self_: Motor, other: Point) -> MultiVector {
    let other_groups = point_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar, self_.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ other_groups.group0_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ vec4<f32>(self_.e41_, self_.e42_, self_.e43_, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(self_.e23_, self_.e31_, self_.e12_, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn motor_sub_scalar(self_: Motor, other: Scalar) -> Motor {
    let self_groups = motor_grouped(self_);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ self_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.e23_, self_.e31_, self_.e12_, self_.scalar - other.scalar)
    ));
}
fn multiVector_sub_antiScalar(self_: MultiVector, other: AntiScalar) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar, self_.e1234_ - other.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ self_groups.group1_, 
        /* e41, e42, e43 */ self_groups.group2_, 
        /* e23, e31, e12 */ self_groups.group3_, 
        /* e423, e431, e412, e321 */ self_groups.group4_
    ));
}
fn multiVector_sub_dualNum(self_: MultiVector, other: DualNum) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar - other.scalar, self_.e1234_ - other.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ self_groups.group1_, 
        /* e41, e42, e43 */ self_groups.group2_, 
        /* e23, e31, e12 */ self_groups.group3_, 
        /* e423, e431, e412, e321 */ self_groups.group4_
    ));
}
fn multiVector_sub_flector(self_: MultiVector, other: Flector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e1_ - other.e1_, self_.e2_ - other.e2_, self_.e3_ - other.e3_, self_.e4_ - other.e4_), 
        /* e41, e42, e43 */ self_groups.group2_, 
        /* e23, e31, e12 */ self_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e423_ - other.e423_, self_.e431_ - other.e431_, self_.e412_ - other.e412_, self_.e321_ - other.e321_)
    ));
}
fn multiVector_sub_horizon(self_: MultiVector, other: Horizon) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_groups.group0_, 
        /* e1, e2, e3, e4 */ self_groups.group1_, 
        /* e41, e42, e43 */ self_groups.group2_, 
        /* e23, e31, e12 */ self_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e423_, self_.e431_, self_.e412_, self_.e321_ - other.e321_)
    ));
}
fn multiVector_sub_line(self_: MultiVector, other: Line) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_groups.group0_, 
        /* e1, e2, e3, e4 */ self_groups.group1_, 
        /* e41, e42, e43 */ vec4<f32>(self_.e41_ - other.e41_, self_.e42_ - other.e42_, self_.e43_ - other.e43_, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(self_.e23_ - other.e23_, self_.e31_ - other.e31_, self_.e12_ - other.e12_, 0.0), 
        /* e423, e431, e412, e321 */ self_groups.group4_
    ));
}
fn multiVector_sub_motor(self_: MultiVector, other: Motor) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar - other.scalar, self_.e1234_ - other.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ self_groups.group1_, 
        /* e41, e42, e43 */ vec4<f32>(self_.e41_ - other.e41_, self_.e42_ - other.e42_, self_.e43_ - other.e43_, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(self_.e23_ - other.e23_, self_.e31_ - other.e31_, self_.e12_ - other.e12_, 0.0), 
        /* e423, e431, e412, e321 */ self_groups.group4_
    ));
}
fn multiVector_sub_multiVector(self_: MultiVector, other: MultiVector) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar - other.scalar, self_.e1234_ - other.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e1_ - other.e1_, self_.e2_ - other.e2_, self_.e3_ - other.e3_, self_.e4_ - other.e4_), 
        /* e41, e42, e43 */ vec4<f32>(self_.e41_ - other.e41_, self_.e42_ - other.e42_, self_.e43_ - other.e43_, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(self_.e23_ - other.e23_, self_.e31_ - other.e31_, self_.e12_ - other.e12_, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e423_ - other.e423_, self_.e431_ - other.e431_, self_.e412_ - other.e412_, self_.e321_ - other.e321_)
    ));
}
fn multiVector_sub_origin(self_: MultiVector, other: Origin) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e1_, self_.e2_, self_.e3_, self_.e4_ - other.e4_), 
        /* e41, e42, e43 */ self_groups.group2_, 
        /* e23, e31, e12 */ self_groups.group3_, 
        /* e423, e431, e412, e321 */ self_groups.group4_
    ));
}
fn multiVector_sub_plane(self_: MultiVector, other: Plane) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_groups.group0_, 
        /* e1, e2, e3, e4 */ self_groups.group1_, 
        /* e41, e42, e43 */ self_groups.group2_, 
        /* e23, e31, e12 */ self_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e423_ - other.e423_, self_.e431_ - other.e431_, self_.e412_ - other.e412_, self_.e321_ - other.e321_)
    ));
}
fn multiVector_sub_point(self_: MultiVector, other: Point) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e1_ - other.e1_, self_.e2_ - other.e2_, self_.e3_ - other.e3_, self_.e4_ - other.e4_), 
        /* e41, e42, e43 */ self_groups.group2_, 
        /* e23, e31, e12 */ self_groups.group3_, 
        /* e423, e431, e412, e321 */ self_groups.group4_
    ));
}
fn multiVector_sub_scalar(self_: MultiVector, other: Scalar) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar - other.scalar, self_.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ self_groups.group1_, 
        /* e41, e42, e43 */ self_groups.group2_, 
        /* e23, e31, e12 */ self_groups.group3_, 
        /* e423, e431, e412, e321 */ self_groups.group4_
    ));
}
fn origin_sub_antiScalar(self_: Origin, other: AntiScalar) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(1.0, other.e1234_, 0.0, 0.0) * vec2<f32>(0.0, -1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.e4_), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn origin_sub_dualNum(self_: Origin, other: DualNum) -> MultiVector {
    let other_groups = dualNum_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_ * vec2<f32>(-1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.e4_), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn origin_sub_flector(self_: Origin, other: Flector) -> Flector {
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e1_, other.e2_, other.e3_, self_.e4_ - other.e4_) * vec4<f32>(-1.0, -1.0, -1.0, 1.0), 
        /* e423, e431, e412, e321 */ other_groups.group1_ * vec4<f32>(-1.0)
    ));
}
fn origin_sub_horizon(self_: Origin, other: Horizon) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.e4_), 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0, 1.0, 1.0, other.e321_) * vec4<f32>(0.0, 0.0, 0.0, -1.0)
    ));
}
fn origin_sub_line(self_: Origin, other: Line) -> MultiVector {
    let other_groups = line_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.e4_), 
        /* e41, e42, e43 */ other_groups.group0_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ other_groups.group1_ * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn origin_sub_motor(self_: Origin, other: Motor) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar, other.e1234_, 0.0, 0.0) * vec2<f32>(-1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.e4_), 
        /* e41, e42, e43 */ vec4<f32>(other.e41_, other.e42_, other.e43_, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>(other.e23_, other.e31_, other.e12_, 0.0) * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn origin_sub_multiVector(self_: Origin, other: MultiVector) -> MultiVector {
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_ * vec2<f32>(-1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(other.e1_, other.e2_, other.e3_, self_.e4_ - other.e4_) * vec4<f32>(-1.0, -1.0, -1.0, 1.0), 
        /* e41, e42, e43 */ other_groups.group2_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ other_groups.group3_ * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ other_groups.group4_ * vec4<f32>(-1.0)
    ));
}
fn origin_sub_origin(self_: Origin, other: Origin) -> Origin {
    return Origin(self_.e4_ - other.e4_);
}
fn origin_sub_plane(self_: Origin, other: Plane) -> Flector {
    let other_groups = plane_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.e4_), 
        /* e423, e431, e412, e321 */ other_groups.group0_ * vec4<f32>(-1.0)
    ));
}
fn origin_sub_point(self_: Origin, other: Point) -> Point {
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e1_, other.e2_, other.e3_, self_.e4_ - other.e4_) * vec4<f32>(-1.0, -1.0, -1.0, 1.0)
    ));
}
fn origin_sub_scalar(self_: Origin, other: Scalar) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar, 1.0, 0.0, 0.0) * vec2<f32>(-1.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.e4_), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn plane_sub_antiScalar(self_: Plane, other: AntiScalar) -> MultiVector {
    let self_groups = plane_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(1.0, other.e1234_, 0.0, 0.0) * vec2<f32>(0.0, -1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ self_groups.group0_
    ));
}
fn plane_sub_dualNum(self_: Plane, other: DualNum) -> MultiVector {
    let self_groups = plane_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_ * vec2<f32>(-1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ self_groups.group0_
    ));
}
fn plane_sub_flector(self_: Plane, other: Flector) -> Flector {
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ other_groups.group0_ * vec4<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e423_ - other.e423_, self_.e431_ - other.e431_, self_.e412_ - other.e412_, self_.e321_ - other.e321_)
    ));
}
fn plane_sub_horizon(self_: Plane, other: Horizon) -> Plane {
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e423_, self_.e431_, self_.e412_, self_.e321_ - other.e321_)
    ));
}
fn plane_sub_line(self_: Plane, other: Line) -> MultiVector {
    let self_groups = plane_grouped(self_);
    let other_groups = line_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ other_groups.group0_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ other_groups.group1_ * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ self_groups.group0_
    ));
}
fn plane_sub_motor(self_: Plane, other: Motor) -> MultiVector {
    let self_groups = plane_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar, other.e1234_, 0.0, 0.0) * vec2<f32>(-1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(other.e41_, other.e42_, other.e43_, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>(other.e23_, other.e31_, other.e12_, 0.0) * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ self_groups.group0_
    ));
}
fn plane_sub_multiVector(self_: Plane, other: MultiVector) -> MultiVector {
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_ * vec2<f32>(-1.0), 
        /* e1, e2, e3, e4 */ other_groups.group1_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ other_groups.group2_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ other_groups.group3_ * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e423_ - other.e423_, self_.e431_ - other.e431_, self_.e412_ - other.e412_, self_.e321_ - other.e321_)
    ));
}
fn plane_sub_origin(self_: Plane, other: Origin) -> Flector {
    let self_groups = plane_grouped(self_);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, other.e4_) * vec4<f32>(0.0, 0.0, 0.0, -1.0), 
        /* e423, e431, e412, e321 */ self_groups.group0_
    ));
}
fn plane_sub_plane(self_: Plane, other: Plane) -> Plane {
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e423_ - other.e423_, self_.e431_ - other.e431_, self_.e412_ - other.e412_, self_.e321_ - other.e321_)
    ));
}
fn plane_sub_point(self_: Plane, other: Point) -> Flector {
    let self_groups = plane_grouped(self_);
    let other_groups = point_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ other_groups.group0_ * vec4<f32>(-1.0), 
        /* e423, e431, e412, e321 */ self_groups.group0_
    ));
}
fn plane_sub_scalar(self_: Plane, other: Scalar) -> MultiVector {
    let self_groups = plane_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar, 1.0, 0.0, 0.0) * vec2<f32>(-1.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ self_groups.group0_
    ));
}
fn point_sub_antiScalar(self_: Point, other: AntiScalar) -> MultiVector {
    let self_groups = point_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(1.0, other.e1234_, 0.0, 0.0) * vec2<f32>(0.0, -1.0), 
        /* e1, e2, e3, e4 */ self_groups.group0_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn point_sub_dualNum(self_: Point, other: DualNum) -> MultiVector {
    let self_groups = point_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_ * vec2<f32>(-1.0), 
        /* e1, e2, e3, e4 */ self_groups.group0_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn point_sub_flector(self_: Point, other: Flector) -> Flector {
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e1_ - other.e1_, self_.e2_ - other.e2_, self_.e3_ - other.e3_, self_.e4_ - other.e4_), 
        /* e423, e431, e412, e321 */ other_groups.group1_ * vec4<f32>(-1.0)
    ));
}
fn point_sub_horizon(self_: Point, other: Horizon) -> Flector {
    let self_groups = point_grouped(self_);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ self_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0, 1.0, 1.0, other.e321_) * vec4<f32>(0.0, 0.0, 0.0, -1.0)
    ));
}
fn point_sub_line(self_: Point, other: Line) -> MultiVector {
    let self_groups = point_grouped(self_);
    let other_groups = line_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ self_groups.group0_, 
        /* e41, e42, e43 */ other_groups.group0_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ other_groups.group1_ * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn point_sub_motor(self_: Point, other: Motor) -> MultiVector {
    let self_groups = point_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar, other.e1234_, 0.0, 0.0) * vec2<f32>(-1.0), 
        /* e1, e2, e3, e4 */ self_groups.group0_, 
        /* e41, e42, e43 */ vec4<f32>(other.e41_, other.e42_, other.e43_, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>(other.e23_, other.e31_, other.e12_, 0.0) * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn point_sub_multiVector(self_: Point, other: MultiVector) -> MultiVector {
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_ * vec2<f32>(-1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e1_ - other.e1_, self_.e2_ - other.e2_, self_.e3_ - other.e3_, self_.e4_ - other.e4_), 
        /* e41, e42, e43 */ other_groups.group2_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ other_groups.group3_ * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ other_groups.group4_ * vec4<f32>(-1.0)
    ));
}
fn point_sub_origin(self_: Point, other: Origin) -> Point {
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e1_, self_.e2_, self_.e3_, self_.e4_ - other.e4_)
    ));
}
fn point_sub_plane(self_: Point, other: Plane) -> Flector {
    let self_groups = point_grouped(self_);
    let other_groups = plane_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ self_groups.group0_, 
        /* e423, e431, e412, e321 */ other_groups.group0_ * vec4<f32>(-1.0)
    ));
}
fn point_sub_point(self_: Point, other: Point) -> Point {
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e1_ - other.e1_, self_.e2_ - other.e2_, self_.e3_ - other.e3_, self_.e4_ - other.e4_)
    ));
}
fn point_sub_scalar(self_: Point, other: Scalar) -> MultiVector {
    let self_groups = point_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar, 1.0, 0.0, 0.0) * vec2<f32>(-1.0, 0.0), 
        /* e1, e2, e3, e4 */ self_groups.group0_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn scalar_sub_antiScalar(self_: Scalar, other: AntiScalar) -> DualNum {
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar, other.e1234_, 0.0, 0.0) * vec2<f32>(1.0, -1.0)
    ));
}
fn scalar_sub_dualNum(self_: Scalar, other: DualNum) -> DualNum {
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar - other.scalar, other.e1234_, 0.0, 0.0) * vec2<f32>(1.0, -1.0)
    ));
}
fn scalar_sub_flector(self_: Scalar, other: Flector) -> MultiVector {
    let other_groups = flector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ other_groups.group0_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ other_groups.group1_ * vec4<f32>(-1.0)
    ));
}
fn scalar_sub_horizon(self_: Scalar, other: Horizon) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0, 1.0, 1.0, other.e321_) * vec4<f32>(0.0, 0.0, 0.0, -1.0)
    ));
}
fn scalar_sub_line(self_: Scalar, other: Line) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e41_, other.e42_, other.e43_, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(other.e23_, other.e31_, other.e12_, self_.scalar) * vec4<f32>(-1.0, -1.0, -1.0, 1.0)
    ));
}
fn scalar_sub_motor(self_: Scalar, other: Motor) -> Motor {
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ other_groups.group0_ * vec4<f32>(-1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(other.e23_, other.e31_, other.e12_, self_.scalar - other.scalar) * vec4<f32>(-1.0, -1.0, -1.0, 1.0)
    ));
}
fn scalar_sub_multiVector(self_: Scalar, other: MultiVector) -> MultiVector {
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar - other.scalar, other.e1234_, 0.0, 0.0) * vec2<f32>(1.0, -1.0), 
        /* e1, e2, e3, e4 */ other_groups.group1_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ other_groups.group2_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ other_groups.group3_ * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ other_groups.group4_ * vec4<f32>(-1.0)
    ));
}
fn scalar_sub_origin(self_: Scalar, other: Origin) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, other.e4_) * vec4<f32>(0.0, 0.0, 0.0, -1.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn scalar_sub_plane(self_: Scalar, other: Plane) -> MultiVector {
    let other_groups = plane_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ other_groups.group0_ * vec4<f32>(-1.0)
    ));
}
fn scalar_sub_point(self_: Scalar, other: Point) -> MultiVector {
    let other_groups = point_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ other_groups.group0_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn scalar_sub_scalar(self_: Scalar, other: Scalar) -> Scalar {
    return Scalar(self_.scalar - other.scalar);
}
fn antiScalar_unit() -> AntiScalar {
    return AntiScalar(1.0);
}
fn dualNum_unit() -> DualNum {
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ (vec4<f32>(1.0) * vec4<f32>(1.0, 1.0, 0.0, 0.0))
    ));
}
fn flector_unit() -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0)
    ));
}
fn horizon_unit() -> Horizon {
    return Horizon(1.0);
}
fn line_unit() -> Line {
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)), 
        /* e23, e31, e12 */ (vec4<f32>(1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0))
    ));
}
fn motor_unit() -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(1.0)
    ));
}
fn multiVector_unit() -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ (vec4<f32>(1.0) * vec4<f32>(1.0, 1.0, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>(1.0), 
        /* e41, e42, e43 */ (vec4<f32>(1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)), 
        /* e23, e31, e12 */ (vec4<f32>(1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0)
    ));
}
fn origin_unit() -> Origin {
    return Origin(1.0);
}
fn plane_unit() -> Plane {
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(1.0)
    ));
}
fn point_unit() -> Point {
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(1.0)
    ));
}
fn scalar_unit() -> Scalar {
    return Scalar(1.0);
}
fn antiScalar_wedge_dualNum(self_: AntiScalar, other: DualNum) -> AntiScalar {
    return AntiScalar(self_.e1234_ * other.scalar);
}
fn antiScalar_wedge_motor(self_: AntiScalar, other: Motor) -> AntiScalar {
    return AntiScalar(self_.e1234_ * other.scalar);
}
fn antiScalar_wedge_multiVector(self_: AntiScalar, other: MultiVector) -> AntiScalar {
    return AntiScalar(self_.e1234_ * other.scalar);
}
fn antiScalar_wedge_scalar(self_: AntiScalar, other: Scalar) -> AntiScalar {
    return AntiScalar(self_.e1234_ * other.scalar);
}
fn dualNum_wedge_antiScalar(self_: DualNum, other: AntiScalar) -> AntiScalar {
    return AntiScalar(other.e1234_ * self_.scalar);
}
fn dualNum_wedge_dualNum(self_: DualNum, other: DualNum) -> DualNum {
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar * self_.scalar, (other.scalar * self_.e1234_) + (other.e1234_ * self_.scalar), 0.0, 0.0)
    ));
}
fn dualNum_wedge_flector(self_: DualNum, other: Flector) -> Flector {
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.scalar) * other_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.scalar) * other_groups.group1_
    ));
}
fn dualNum_wedge_horizon(self_: DualNum, other: Horizon) -> Horizon {
    return Horizon(self_.scalar * other.e321_);
}
fn dualNum_wedge_line(self_: DualNum, other: Line) -> Line {
    let other_groups = line_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group1_
    ));
}
fn dualNum_wedge_motor(self_: DualNum, other: Motor) -> Motor {
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.scalar * other.e41_, self_.scalar * other.e42_, self_.scalar * other.e43_, (self_.scalar * other.e1234_) + (self_.e1234_ * other.scalar)), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.scalar) * other_groups.group1_
    ));
}
fn dualNum_wedge_multiVector(self_: DualNum, other: MultiVector) -> MultiVector {
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar * other.scalar, (self_.scalar * other.e1234_) + (self_.e1234_ * other.scalar), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.scalar) * other_groups.group1_, 
        /* e41, e42, e43 */ (vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group2_, 
        /* e23, e31, e12 */ (vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.scalar) * other_groups.group4_
    ));
}
fn dualNum_wedge_origin(self_: DualNum, other: Origin) -> Origin {
    return Origin(self_.scalar * other.e4_);
}
fn dualNum_wedge_plane(self_: DualNum, other: Plane) -> Plane {
    let other_groups = plane_grouped(other);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(self_.scalar) * other_groups.group0_
    ));
}
fn dualNum_wedge_point(self_: DualNum, other: Point) -> Point {
    let other_groups = point_grouped(other);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.scalar) * other_groups.group0_
    ));
}
fn dualNum_wedge_scalar(self_: DualNum, other: Scalar) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ (vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_
    ));
}
fn flector_wedge_dualNum(self_: Flector, other: DualNum) -> Flector {
    let self_groups = flector_grouped(self_);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.scalar) * self_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other.scalar) * self_groups.group1_
    ));
}
fn flector_wedge_flector(self_: Flector, other: Flector) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, (other.e423_ * self_.e1_) + (other.e431_ * self_.e2_) + (other.e412_ * self_.e3_) - (other.e1_ * self_.e423_) - (other.e2_ * self_.e431_) - (other.e3_ * self_.e412_)) + (vec4<f32>(self_.e4_) * vec4<f32>(other.e1_, other.e2_, other.e3_, other.e321_)) - (vec4<f32>(other.e4_) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, self_.e321_)), 
        /* e23, e31, e12, scalar */ vec4<f32>((other.e3_ * self_.e2_) - (other.e2_ * self_.e3_), (other.e1_ * self_.e3_) - (other.e3_ * self_.e1_), (other.e2_ * self_.e1_) - (other.e1_ * self_.e2_), 0.0)
    ));
}
fn flector_wedge_horizon(self_: Flector, other: Horizon) -> AntiScalar {
    return AntiScalar(self_.e4_ * other.e321_);
}
fn flector_wedge_line(self_: Flector, other: Line) -> Plane {
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e3_ * other.e42_) + (self_.e4_ * other.e23_) - (self_.e2_ * other.e43_), (self_.e1_ * other.e43_) + (self_.e4_ * other.e31_) - (self_.e3_ * other.e41_), (self_.e2_ * other.e41_) + (self_.e4_ * other.e12_) - (self_.e1_ * other.e42_), -(self_.e1_ * other.e23_) - (self_.e2_ * other.e31_) - (self_.e3_ * other.e12_))
    ));
}
fn flector_wedge_motor(self_: Flector, other: Motor) -> Flector {
    let self_groups = flector_grouped(self_);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.scalar) * self_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e3_ * other.e42_) + (self_.e4_ * other.e23_) - (self_.e2_ * other.e43_), (self_.e1_ * other.e43_) + (self_.e4_ * other.e31_) - (self_.e3_ * other.e41_), (self_.e2_ * other.e41_) + (self_.e4_ * other.e12_) - (self_.e1_ * other.e42_), -(self_.e1_ * other.e23_) - (self_.e2_ * other.e31_) - (self_.e3_ * other.e12_)) + (vec4<f32>(other.scalar) * self_groups.group1_)
    ));
}
fn flector_wedge_multiVector(self_: Flector, other: MultiVector) -> MultiVector {
    let self_groups = flector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, (self_.e1_ * other.e423_) + (self_.e2_ * other.e431_) + (self_.e3_ * other.e412_) + (self_.e4_ * other.e321_) - (self_.e423_ * other.e1_) - (self_.e431_ * other.e2_) - (self_.e412_ * other.e3_) - (self_.e321_ * other.e4_), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(other.scalar) * self_groups.group0_, 
        /* e41, e42, e43 */ ((vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0)) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((self_.e2_ * other.e3_) - (self_.e3_ * other.e2_), (self_.e3_ * other.e1_) - (self_.e1_ * other.e3_), (self_.e1_ * other.e2_) - (self_.e2_ * other.e1_), 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e3_ * other.e42_) + (self_.e4_ * other.e23_) - (self_.e2_ * other.e43_), (self_.e1_ * other.e43_) + (self_.e4_ * other.e31_) - (self_.e3_ * other.e41_), (self_.e2_ * other.e41_) + (self_.e4_ * other.e12_) - (self_.e1_ * other.e42_), -(self_.e1_ * other.e23_) - (self_.e2_ * other.e31_) - (self_.e3_ * other.e12_)) + (vec4<f32>(other.scalar) * self_groups.group1_)
    ));
}
fn flector_wedge_origin(self_: Flector, other: Origin) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e4_) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, self_.e321_) * vec4<f32>(-1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0)
    ));
}
fn flector_wedge_plane(self_: Flector, other: Plane) -> AntiScalar {
    return AntiScalar((self_.e1_ * other.e423_) + (self_.e2_ * other.e431_) + (self_.e3_ * other.e412_) + (self_.e4_ * other.e321_));
}
fn flector_wedge_point(self_: Flector, other: Point) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.e4_ * other.e1_, self_.e4_ * other.e2_, self_.e4_ * other.e3_, -(self_.e423_ * other.e1_) - (self_.e431_ * other.e2_) - (self_.e412_ * other.e3_)) - (vec4<f32>(other.e4_) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, self_.e321_)), 
        /* e23, e31, e12, scalar */ vec4<f32>((self_.e2_ * other.e3_) - (self_.e3_ * other.e2_), (self_.e3_ * other.e1_) - (self_.e1_ * other.e3_), (self_.e1_ * other.e2_) - (self_.e2_ * other.e1_), 0.0)
    ));
}
fn flector_wedge_scalar(self_: Flector, other: Scalar) -> Flector {
    let self_groups = flector_grouped(self_);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.scalar) * self_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other.scalar) * self_groups.group1_
    ));
}
fn horizon_wedge_dualNum(self_: Horizon, other: DualNum) -> Horizon {
    return Horizon(other.scalar * self_.e321_);
}
fn horizon_wedge_flector(self_: Horizon, other: Flector) -> AntiScalar {
    return AntiScalar(other.e4_ * self_.e321_ * -1.0);
}
fn horizon_wedge_motor(self_: Horizon, other: Motor) -> Horizon {
    return Horizon(self_.e321_ * other.scalar);
}
fn horizon_wedge_multiVector(self_: Horizon, other: MultiVector) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(1.0, self_.e321_ * other.e4_, 0.0, 0.0) * vec2<f32>(0.0, -1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0, 1.0, 1.0, self_.e321_ * other.scalar) * vec4<f32>(0.0, 0.0, 0.0, 1.0)
    ));
}
fn horizon_wedge_origin(self_: Horizon, other: Origin) -> AntiScalar {
    return AntiScalar(self_.e321_ * other.e4_ * -1.0);
}
fn horizon_wedge_point(self_: Horizon, other: Point) -> AntiScalar {
    return AntiScalar(self_.e321_ * other.e4_ * -1.0);
}
fn horizon_wedge_scalar(self_: Horizon, other: Scalar) -> Horizon {
    return Horizon(self_.e321_ * other.scalar);
}
fn line_wedge_dualNum(self_: Line, other: DualNum) -> Line {
    let self_groups = line_grouped(self_);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_
    ));
}
fn line_wedge_flector(self_: Line, other: Flector) -> Plane {
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>((other.e3_ * self_.e42_) + (other.e4_ * self_.e23_) - (other.e2_ * self_.e43_), (other.e1_ * self_.e43_) + (other.e4_ * self_.e31_) - (other.e3_ * self_.e41_), (other.e2_ * self_.e41_) + (other.e4_ * self_.e12_) - (other.e1_ * self_.e42_), -(other.e1_ * self_.e23_) - (other.e2_ * self_.e31_) - (other.e3_ * self_.e12_))
    ));
}
fn line_wedge_line(self_: Line, other: Line) -> AntiScalar {
    return AntiScalar(-(other.e41_ * self_.e23_) - (other.e42_ * self_.e31_) - (other.e43_ * self_.e12_) - (other.e23_ * self_.e41_) - (other.e31_ * self_.e42_) - (other.e12_ * self_.e43_));
}
fn line_wedge_motor(self_: Line, other: Motor) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.e41_ * other.scalar, self_.e42_ * other.scalar, self_.e43_ * other.scalar, -(self_.e41_ * other.e23_) - (self_.e42_ * other.e31_) - (self_.e43_ * other.e12_) - (self_.e23_ * other.e41_) - (self_.e31_ * other.e42_) - (self_.e12_ * other.e43_)), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.e23_ * other.scalar, self_.e31_ * other.scalar, self_.e12_ * other.scalar, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
}
fn line_wedge_multiVector(self_: Line, other: MultiVector) -> MultiVector {
    let self_groups = line_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, -(self_.e41_ * other.e23_) - (self_.e42_ * other.e31_) - (self_.e43_ * other.e12_) - (self_.e23_ * other.e41_) - (self_.e31_ * other.e42_) - (self_.e12_ * other.e43_), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ (vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_, 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e42_ * other.e3_) + (self_.e23_ * other.e4_) - (self_.e43_ * other.e2_), (self_.e43_ * other.e1_) + (self_.e31_ * other.e4_) - (self_.e41_ * other.e3_), (self_.e41_ * other.e2_) + (self_.e12_ * other.e4_) - (self_.e42_ * other.e1_), -(self_.e23_ * other.e1_) - (self_.e31_ * other.e2_) - (self_.e12_ * other.e3_))
    ));
}
fn line_wedge_origin(self_: Line, other: Origin) -> Plane {
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e23_ * other.e4_, self_.e31_ * other.e4_, self_.e12_ * other.e4_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
}
fn line_wedge_point(self_: Line, other: Point) -> Plane {
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e42_ * other.e3_) + (self_.e23_ * other.e4_) - (self_.e43_ * other.e2_), (self_.e43_ * other.e1_) + (self_.e31_ * other.e4_) - (self_.e41_ * other.e3_), (self_.e41_ * other.e2_) + (self_.e12_ * other.e4_) - (self_.e42_ * other.e1_), -(self_.e23_ * other.e1_) - (self_.e31_ * other.e2_) - (self_.e12_ * other.e3_))
    ));
}
fn line_wedge_scalar(self_: Line, other: Scalar) -> Line {
    let self_groups = line_grouped(self_);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_
    ));
}
fn motor_wedge_antiScalar(self_: Motor, other: AntiScalar) -> AntiScalar {
    return AntiScalar(other.e1234_ * self_.scalar);
}
fn motor_wedge_dualNum(self_: Motor, other: DualNum) -> Motor {
    let self_groups = motor_grouped(self_);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.scalar * self_.e41_, other.scalar * self_.e42_, other.scalar * self_.e43_, (other.scalar * self_.e1234_) + (other.e1234_ * self_.scalar)), 
        /* e23, e31, e12, scalar */ vec4<f32>(other.scalar) * self_groups.group1_
    ));
}
fn motor_wedge_flector(self_: Motor, other: Flector) -> Flector {
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.scalar) * other_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e3_ * self_.e42_) + (other.e4_ * self_.e23_) - (other.e2_ * self_.e43_), (other.e1_ * self_.e43_) + (other.e4_ * self_.e31_) - (other.e3_ * self_.e41_), (other.e2_ * self_.e41_) + (other.e4_ * self_.e12_) - (other.e1_ * self_.e42_), -(other.e1_ * self_.e23_) - (other.e2_ * self_.e31_) - (other.e3_ * self_.e12_)) + (vec4<f32>(self_.scalar) * other_groups.group1_)
    ));
}
fn motor_wedge_horizon(self_: Motor, other: Horizon) -> Horizon {
    return Horizon(other.e321_ * self_.scalar);
}
fn motor_wedge_line(self_: Motor, other: Line) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e41_ * self_.scalar, other.e42_ * self_.scalar, other.e43_ * self_.scalar, -(other.e41_ * self_.e23_) - (other.e42_ * self_.e31_) - (other.e43_ * self_.e12_) - (other.e23_ * self_.e41_) - (other.e31_ * self_.e42_) - (other.e12_ * self_.e43_)), 
        /* e23, e31, e12, scalar */ vec4<f32>(other.e23_ * self_.scalar, other.e31_ * self_.scalar, other.e12_ * self_.scalar, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
}
fn motor_wedge_motor(self_: Motor, other: Motor) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, -(other.e41_ * self_.e23_) - (other.e42_ * self_.e31_) - (other.e43_ * self_.e12_) - (other.e23_ * self_.e41_) - (other.e31_ * self_.e42_) - (other.e12_ * self_.e43_)) + (vec4<f32>(other.scalar) * self_groups.group0_) + (vec4<f32>(self_.scalar) * other_groups.group0_), 
        /* e23, e31, e12, scalar */ vec4<f32>((other.e23_ * self_.scalar) + (other.scalar * self_.e23_), (other.e31_ * self_.scalar) + (other.scalar * self_.e31_), (other.e12_ * self_.scalar) + (other.scalar * self_.e12_), other.scalar * self_.scalar)
    ));
}
fn motor_wedge_multiVector(self_: Motor, other: MultiVector) -> MultiVector {
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.scalar * other.scalar, (self_.e1234_ * other.scalar) + (self_.scalar * other.e1234_) - (self_.e41_ * other.e23_) - (self_.e42_ * other.e31_) - (self_.e43_ * other.e12_) - (self_.e23_ * other.e41_) - (self_.e31_ * other.e42_) - (self_.e12_ * other.e43_), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.scalar) * other_groups.group1_, 
        /* e41, e42, e43 */ ((vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group2_) + ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e41_, self_.e42_, self_.e43_, 0.0)), 
        /* e23, e31, e12 */ ((vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group3_) + ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e23_, self_.e31_, self_.e12_, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e42_ * other.e3_) + (self_.e23_ * other.e4_) - (self_.e43_ * other.e2_), (self_.e43_ * other.e1_) + (self_.e31_ * other.e4_) - (self_.e41_ * other.e3_), (self_.e41_ * other.e2_) + (self_.e12_ * other.e4_) - (self_.e42_ * other.e1_), -(self_.e23_ * other.e1_) - (self_.e31_ * other.e2_) - (self_.e12_ * other.e3_)) + (vec4<f32>(self_.scalar) * other_groups.group4_)
    ));
}
fn motor_wedge_origin(self_: Motor, other: Origin) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, self_.scalar * other.e4_) * vec4<f32>(0.0, 0.0, 0.0, 1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e23_ * other.e4_, self_.e31_ * other.e4_, self_.e12_ * other.e4_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
}
fn motor_wedge_plane(self_: Motor, other: Plane) -> Plane {
    let other_groups = plane_grouped(other);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(self_.scalar) * other_groups.group0_
    ));
}
fn motor_wedge_point(self_: Motor, other: Point) -> Flector {
    let other_groups = point_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.scalar) * other_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e42_ * other.e3_) + (self_.e23_ * other.e4_) - (self_.e43_ * other.e2_), (self_.e43_ * other.e1_) + (self_.e31_ * other.e4_) - (self_.e41_ * other.e3_), (self_.e41_ * other.e2_) + (self_.e12_ * other.e4_) - (self_.e42_ * other.e1_), -(self_.e23_ * other.e1_) - (self_.e31_ * other.e2_) - (self_.e12_ * other.e3_))
    ));
}
fn motor_wedge_scalar(self_: Motor, other: Scalar) -> Motor {
    let self_groups = motor_grouped(self_);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.scalar) * self_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(other.scalar) * self_groups.group1_
    ));
}
fn multiVector_wedge_antiScalar(self_: MultiVector, other: AntiScalar) -> AntiScalar {
    return AntiScalar(other.e1234_ * self_.scalar);
}
fn multiVector_wedge_dualNum(self_: MultiVector, other: DualNum) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar * self_.scalar, (other.scalar * self_.e1234_) + (other.e1234_ * self_.scalar), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(other.scalar) * self_groups.group1_, 
        /* e41, e42, e43 */ (vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_, 
        /* e23, e31, e12 */ (vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other.scalar) * self_groups.group4_
    ));
}
fn multiVector_wedge_flector(self_: MultiVector, other: Flector) -> MultiVector {
    let other_groups = flector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, (other.e423_ * self_.e1_) + (other.e431_ * self_.e2_) + (other.e412_ * self_.e3_) + (other.e321_ * self_.e4_) - (other.e1_ * self_.e423_) - (other.e2_ * self_.e431_) - (other.e3_ * self_.e412_) - (other.e4_ * self_.e321_), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.scalar) * other_groups.group0_, 
        /* e41, e42, e43 */ ((vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0)) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((other.e3_ * self_.e2_) - (other.e2_ * self_.e3_), (other.e1_ * self_.e3_) - (other.e3_ * self_.e1_), (other.e2_ * self_.e1_) - (other.e1_ * self_.e2_), 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e3_ * self_.e42_) + (other.e4_ * self_.e23_) - (other.e2_ * self_.e43_), (other.e1_ * self_.e43_) + (other.e4_ * self_.e31_) - (other.e3_ * self_.e41_), (other.e2_ * self_.e41_) + (other.e4_ * self_.e12_) - (other.e1_ * self_.e42_), -(other.e1_ * self_.e23_) - (other.e2_ * self_.e31_) - (other.e3_ * self_.e12_)) + (vec4<f32>(self_.scalar) * other_groups.group1_)
    ));
}
fn multiVector_wedge_horizon(self_: MultiVector, other: Horizon) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(1.0, other.e321_ * self_.e4_, 0.0, 0.0) * vec2<f32>(0.0, 1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0, 1.0, 1.0, other.e321_ * self_.scalar) * vec4<f32>(0.0, 0.0, 0.0, 1.0)
    ));
}
fn multiVector_wedge_line(self_: MultiVector, other: Line) -> MultiVector {
    let other_groups = line_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, -(other.e41_ * self_.e23_) - (other.e42_ * self_.e31_) - (other.e43_ * self_.e12_) - (other.e23_ * self_.e41_) - (other.e31_ * self_.e42_) - (other.e12_ * self_.e43_), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ (vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group1_, 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e42_ * self_.e3_) + (other.e23_ * self_.e4_) - (other.e43_ * self_.e2_), (other.e43_ * self_.e1_) + (other.e31_ * self_.e4_) - (other.e41_ * self_.e3_), (other.e41_ * self_.e2_) + (other.e12_ * self_.e4_) - (other.e42_ * self_.e1_), -(other.e23_ * self_.e1_) - (other.e31_ * self_.e2_) - (other.e12_ * self_.e3_))
    ));
}
fn multiVector_wedge_motor(self_: MultiVector, other: Motor) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar * self_.scalar, (other.e1234_ * self_.scalar) + (other.scalar * self_.e1234_) - (other.e41_ * self_.e23_) - (other.e42_ * self_.e31_) - (other.e43_ * self_.e12_) - (other.e23_ * self_.e41_) - (other.e31_ * self_.e42_) - (other.e12_ * self_.e43_), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(other.scalar) * self_groups.group1_, 
        /* e41, e42, e43 */ ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_) + ((vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e41_, other.e42_, other.e43_, 0.0)), 
        /* e23, e31, e12 */ ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_) + ((vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e23_, other.e31_, other.e12_, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e42_ * self_.e3_) + (other.e23_ * self_.e4_) - (other.e43_ * self_.e2_), (other.e43_ * self_.e1_) + (other.e31_ * self_.e4_) - (other.e41_ * self_.e3_), (other.e41_ * self_.e2_) + (other.e12_ * self_.e4_) - (other.e42_ * self_.e1_), -(other.e23_ * self_.e1_) - (other.e31_ * self_.e2_) - (other.e12_ * self_.e3_)) + (vec4<f32>(other.scalar) * self_groups.group4_)
    ));
}
fn multiVector_wedge_multiVector(self_: MultiVector, other: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar * self_.scalar, (other.scalar * self_.e1234_) + (other.e1234_ * self_.scalar) + (other.e423_ * self_.e1_) + (other.e431_ * self_.e2_) + (other.e412_ * self_.e3_) + (other.e321_ * self_.e4_) - (other.e1_ * self_.e423_) - (other.e2_ * self_.e431_) - (other.e3_ * self_.e412_) - (other.e4_ * self_.e321_) - (other.e41_ * self_.e23_) - (other.e42_ * self_.e31_) - (other.e43_ * self_.e12_) - (other.e23_ * self_.e41_) - (other.e31_ * self_.e42_) - (other.e12_ * self_.e43_), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ (vec4<f32>(other.scalar) * self_groups.group1_) + (vec4<f32>(self_.scalar) * other_groups.group1_), 
        /* e41, e42, e43 */ ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_) + ((vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group2_) + ((vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0)) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((other.e3_ * self_.e2_) - (other.e2_ * self_.e3_), (other.e1_ * self_.e3_) - (other.e3_ * self_.e1_), (other.e2_ * self_.e1_) - (other.e1_ * self_.e2_), 0.0) + ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_) + ((vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group3_), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e3_ * self_.e42_) + (other.e4_ * self_.e23_) + (other.e42_ * self_.e3_) + (other.e23_ * self_.e4_) - (other.e2_ * self_.e43_) - (other.e43_ * self_.e2_), (other.e1_ * self_.e43_) + (other.e4_ * self_.e31_) + (other.e43_ * self_.e1_) + (other.e31_ * self_.e4_) - (other.e3_ * self_.e41_) - (other.e41_ * self_.e3_), (other.e2_ * self_.e41_) + (other.e4_ * self_.e12_) + (other.e41_ * self_.e2_) + (other.e12_ * self_.e4_) - (other.e1_ * self_.e42_) - (other.e42_ * self_.e1_), -(other.e1_ * self_.e23_) - (other.e2_ * self_.e31_) - (other.e3_ * self_.e12_) - (other.e23_ * self_.e1_) - (other.e31_ * self_.e2_) - (other.e12_ * self_.e3_)) + (vec4<f32>(other.scalar) * self_groups.group4_) + (vec4<f32>(self_.scalar) * other_groups.group4_)
    ));
}
fn multiVector_wedge_origin(self_: MultiVector, other: Origin) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(1.0, self_.e321_ * other.e4_, 0.0, 0.0) * vec2<f32>(0.0, -1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, self_.scalar * other.e4_) * vec4<f32>(0.0, 0.0, 0.0, 1.0), 
        /* e41, e42, e43 */ (vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e23_ * other.e4_, self_.e31_ * other.e4_, self_.e12_ * other.e4_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
}
fn multiVector_wedge_plane(self_: MultiVector, other: Plane) -> MultiVector {
    let other_groups = plane_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, (self_.e1_ * other.e423_) + (self_.e2_ * other.e431_) + (self_.e3_ * other.e412_) + (self_.e4_ * other.e321_), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.scalar) * other_groups.group0_
    ));
}
fn multiVector_wedge_point(self_: MultiVector, other: Point) -> MultiVector {
    let other_groups = point_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, -(self_.e423_ * other.e1_) - (self_.e431_ * other.e2_) - (self_.e412_ * other.e3_) - (self_.e321_ * other.e4_), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.scalar) * other_groups.group0_, 
        /* e41, e42, e43 */ ((vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0)) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((self_.e2_ * other.e3_) - (self_.e3_ * other.e2_), (self_.e3_ * other.e1_) - (self_.e1_ * other.e3_), (self_.e1_ * other.e2_) - (self_.e2_ * other.e1_), 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e42_ * other.e3_) + (self_.e23_ * other.e4_) - (self_.e43_ * other.e2_), (self_.e43_ * other.e1_) + (self_.e31_ * other.e4_) - (self_.e41_ * other.e3_), (self_.e41_ * other.e2_) + (self_.e12_ * other.e4_) - (self_.e42_ * other.e1_), -(self_.e23_ * other.e1_) - (self_.e31_ * other.e2_) - (self_.e12_ * other.e3_))
    ));
}
fn multiVector_wedge_scalar(self_: MultiVector, other: Scalar) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ (vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(other.scalar) * self_groups.group1_, 
        /* e41, e42, e43 */ (vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_, 
        /* e23, e31, e12 */ (vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other.scalar) * self_groups.group4_
    ));
}
fn origin_wedge_dualNum(self_: Origin, other: DualNum) -> Origin {
    return Origin(other.scalar * self_.e4_);
}
fn origin_wedge_flector(self_: Origin, other: Flector) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.e4_) * vec4<f32>(other.e1_, other.e2_, other.e3_, other.e321_), 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0)
    ));
}
fn origin_wedge_horizon(self_: Origin, other: Horizon) -> AntiScalar {
    return AntiScalar(other.e321_ * self_.e4_);
}
fn origin_wedge_line(self_: Origin, other: Line) -> Plane {
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(other.e23_ * self_.e4_, other.e31_ * self_.e4_, other.e12_ * self_.e4_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
}
fn origin_wedge_motor(self_: Origin, other: Motor) -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, other.scalar * self_.e4_) * vec4<f32>(0.0, 0.0, 0.0, 1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e23_ * self_.e4_, other.e31_ * self_.e4_, other.e12_ * self_.e4_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
}
fn origin_wedge_multiVector(self_: Origin, other: MultiVector) -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(1.0, other.e321_ * self_.e4_, 0.0, 0.0) * vec2<f32>(0.0, 1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, other.scalar * self_.e4_) * vec4<f32>(0.0, 0.0, 0.0, 1.0), 
        /* e41, e42, e43 */ (vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e23_ * self_.e4_, other.e31_ * self_.e4_, other.e12_ * self_.e4_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
}
fn origin_wedge_plane(self_: Origin, other: Plane) -> AntiScalar {
    return AntiScalar(self_.e4_ * other.e321_);
}
fn origin_wedge_point(self_: Origin, other: Point) -> Line {
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0)
    ));
}
fn origin_wedge_scalar(self_: Origin, other: Scalar) -> Origin {
    return Origin(self_.e4_ * other.scalar);
}
fn plane_wedge_dualNum(self_: Plane, other: DualNum) -> Plane {
    let self_groups = plane_grouped(self_);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(other.scalar) * self_groups.group0_
    ));
}
fn plane_wedge_flector(self_: Plane, other: Flector) -> AntiScalar {
    return AntiScalar(-(other.e1_ * self_.e423_) - (other.e2_ * self_.e431_) - (other.e3_ * self_.e412_) - (other.e4_ * self_.e321_));
}
fn plane_wedge_motor(self_: Plane, other: Motor) -> Plane {
    let self_groups = plane_grouped(self_);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(other.scalar) * self_groups.group0_
    ));
}
fn plane_wedge_multiVector(self_: Plane, other: MultiVector) -> MultiVector {
    let self_groups = plane_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, -(other.e1_ * self_.e423_) - (other.e2_ * self_.e431_) - (other.e3_ * self_.e412_) - (other.e4_ * self_.e321_), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.scalar) * self_groups.group0_
    ));
}
fn plane_wedge_origin(self_: Plane, other: Origin) -> AntiScalar {
    return AntiScalar(other.e4_ * self_.e321_ * -1.0);
}
fn plane_wedge_point(self_: Plane, other: Point) -> AntiScalar {
    return AntiScalar(-(self_.e423_ * other.e1_) - (self_.e431_ * other.e2_) - (self_.e412_ * other.e3_) - (self_.e321_ * other.e4_));
}
fn plane_wedge_scalar(self_: Plane, other: Scalar) -> Plane {
    let self_groups = plane_grouped(self_);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(other.scalar) * self_groups.group0_
    ));
}
fn point_wedge_dualNum(self_: Point, other: DualNum) -> Point {
    let self_groups = point_grouped(self_);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.scalar) * self_groups.group0_
    ));
}
fn point_wedge_flector(self_: Point, other: Flector) -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other.e4_ * self_.e1_) * -1.0, (other.e4_ * self_.e2_) * -1.0, (other.e4_ * self_.e3_) * -1.0, (other.e423_ * self_.e1_) + (other.e431_ * self_.e2_) + (other.e412_ * self_.e3_)) + (vec4<f32>(self_.e4_) * vec4<f32>(other.e1_, other.e2_, other.e3_, other.e321_)), 
        /* e23, e31, e12, scalar */ vec4<f32>((other.e3_ * self_.e2_) - (other.e2_ * self_.e3_), (other.e1_ * self_.e3_) - (other.e3_ * self_.e1_), (other.e2_ * self_.e1_) - (other.e1_ * self_.e2_), 0.0)
    ));
}
fn point_wedge_horizon(self_: Point, other: Horizon) -> AntiScalar {
    return AntiScalar(other.e321_ * self_.e4_);
}
fn point_wedge_line(self_: Point, other: Line) -> Plane {
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>((other.e42_ * self_.e3_) + (other.e23_ * self_.e4_) - (other.e43_ * self_.e2_), (other.e43_ * self_.e1_) + (other.e31_ * self_.e4_) - (other.e41_ * self_.e3_), (other.e41_ * self_.e2_) + (other.e12_ * self_.e4_) - (other.e42_ * self_.e1_), -(other.e23_ * self_.e1_) - (other.e31_ * self_.e2_) - (other.e12_ * self_.e3_))
    ));
}
fn point_wedge_motor(self_: Point, other: Motor) -> Flector {
    let self_groups = point_grouped(self_);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.scalar) * self_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e42_ * self_.e3_) + (other.e23_ * self_.e4_) - (other.e43_ * self_.e2_), (other.e43_ * self_.e1_) + (other.e31_ * self_.e4_) - (other.e41_ * self_.e3_), (other.e41_ * self_.e2_) + (other.e12_ * self_.e4_) - (other.e42_ * self_.e1_), -(other.e23_ * self_.e1_) - (other.e31_ * self_.e2_) - (other.e12_ * self_.e3_))
    ));
}
fn point_wedge_multiVector(self_: Point, other: MultiVector) -> MultiVector {
    let self_groups = point_grouped(self_);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, (other.e423_ * self_.e1_) + (other.e431_ * self_.e2_) + (other.e412_ * self_.e3_) + (other.e321_ * self_.e4_), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(other.scalar) * self_groups.group0_, 
        /* e41, e42, e43 */ ((vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0)) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((other.e3_ * self_.e2_) - (other.e2_ * self_.e3_), (other.e1_ * self_.e3_) - (other.e3_ * self_.e1_), (other.e2_ * self_.e1_) - (other.e1_ * self_.e2_), 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e42_ * self_.e3_) + (other.e23_ * self_.e4_) - (other.e43_ * self_.e2_), (other.e43_ * self_.e1_) + (other.e31_ * self_.e4_) - (other.e41_ * self_.e3_), (other.e41_ * self_.e2_) + (other.e12_ * self_.e4_) - (other.e42_ * self_.e1_), -(other.e23_ * self_.e1_) - (other.e31_ * self_.e2_) - (other.e12_ * self_.e3_))
    ));
}
fn point_wedge_origin(self_: Point, other: Origin) -> Line {
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0)
    ));
}
fn point_wedge_plane(self_: Point, other: Plane) -> AntiScalar {
    return AntiScalar((other.e423_ * self_.e1_) + (other.e431_ * self_.e2_) + (other.e412_ * self_.e3_) + (other.e321_ * self_.e4_));
}
fn point_wedge_point(self_: Point, other: Point) -> Line {
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ ((vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0)) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((other.e3_ * self_.e2_) - (other.e2_ * self_.e3_), (other.e1_ * self_.e3_) - (other.e3_ * self_.e1_), (other.e2_ * self_.e1_) - (other.e1_ * self_.e2_), 0.0)
    ));
}
fn point_wedge_scalar(self_: Point, other: Scalar) -> Point {
    let self_groups = point_grouped(self_);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.scalar) * self_groups.group0_
    ));
}
fn scalar_wedge_antiScalar(self_: Scalar, other: AntiScalar) -> AntiScalar {
    return AntiScalar(other.e1234_ * self_.scalar);
}
fn scalar_wedge_dualNum(self_: Scalar, other: DualNum) -> DualNum {
    let other_groups = dualNum_grouped(other);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ (vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * other_groups.group0_
    ));
}
fn scalar_wedge_flector(self_: Scalar, other: Flector) -> Flector {
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.scalar) * other_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.scalar) * other_groups.group1_
    ));
}
fn scalar_wedge_horizon(self_: Scalar, other: Horizon) -> Horizon {
    return Horizon(other.e321_ * self_.scalar);
}
fn scalar_wedge_line(self_: Scalar, other: Line) -> Line {
    let other_groups = line_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group1_
    ));
}
fn scalar_wedge_motor(self_: Scalar, other: Motor) -> Motor {
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.scalar) * other_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.scalar) * other_groups.group1_
    ));
}
fn scalar_wedge_multiVector(self_: Scalar, other: MultiVector) -> MultiVector {
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ (vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * other_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.scalar) * other_groups.group1_, 
        /* e41, e42, e43 */ (vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group2_, 
        /* e23, e31, e12 */ (vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.scalar) * other_groups.group4_
    ));
}
fn scalar_wedge_origin(self_: Scalar, other: Origin) -> Origin {
    return Origin(other.e4_ * self_.scalar);
}
fn scalar_wedge_plane(self_: Scalar, other: Plane) -> Plane {
    let other_groups = plane_grouped(other);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(self_.scalar) * other_groups.group0_
    ));
}
fn scalar_wedge_point(self_: Scalar, other: Point) -> Point {
    let other_groups = point_grouped(other);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.scalar) * other_groups.group0_
    ));
}
fn scalar_wedge_scalar(self_: Scalar, other: Scalar) -> Scalar {
    return Scalar(other.scalar * self_.scalar);
}
fn antiScalar_zero() -> AntiScalar {
    return AntiScalar(0.0);
}
fn dualNum_zero() -> DualNum {
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(0.0)
    ));
}
fn flector_zero() -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn horizon_zero() -> Horizon {
    return Horizon(0.0);
}
fn line_zero() -> Line {
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0)
    ));
}
fn motor_zero() -> Motor {
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0)
    ));
}
fn multiVector_zero() -> MultiVector {
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn origin_zero() -> Origin {
    return Origin(0.0);
}
fn plane_zero() -> Plane {
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn point_zero() -> Point {
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0)
    ));
}
fn scalar_zero() -> Scalar {
    return Scalar(0.0);
}
fn antiScalar_antiSandwich_antiScalar(self_: AntiScalar, other: AntiScalar) -> AntiScalar {
    let geometric_anti_product: AntiScalar = AntiScalar(other.e1234_ * self_.e1234_);
    return antiScalar_geometricAntiProduct_antiScalar(geometric_anti_product, antiScalar_antiReverse(self_));
}
fn antiScalar_antiSandwich_dualNum(self_: AntiScalar, other: DualNum) -> DualNum {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let geometric_anti_product: DualNum = dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ (vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * other_groups.group0_
    ));
    return dualNum_geometricAntiProduct_antiScalar(geometric_anti_product, antiScalar_antiReverse(self_));
}
fn antiScalar_antiSandwich_flector(self_: AntiScalar, other: Flector) -> Flector {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = flector_grouped(other);
    let geometric_anti_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e1234_) * other_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e1234_) * other_groups.group1_
    ));
    return flector_geometricAntiProduct_antiScalar(geometric_anti_product, antiScalar_antiReverse(self_));
}
fn antiScalar_antiSandwich_horizon(self_: AntiScalar, other: Horizon) -> Horizon {
    let geometric_anti_product: Horizon = Horizon(self_.e1234_ * other.e321_);
    return horizon_geometricAntiProduct_antiScalar(geometric_anti_product, antiScalar_antiReverse(self_));
}
fn antiScalar_antiSandwich_line(self_: AntiScalar, other: Line) -> Line {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = line_grouped(other);
    let geometric_anti_product: Line = line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group1_
    ));
    return line_geometricAntiProduct_antiScalar(geometric_anti_product, antiScalar_antiReverse(self_));
}
fn antiScalar_antiSandwich_motor(self_: AntiScalar, other: Motor) -> Motor {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = motor_grouped(other);
    let geometric_anti_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.e1234_) * other_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.e1234_) * other_groups.group1_
    ));
    return motor_geometricAntiProduct_antiScalar(geometric_anti_product, antiScalar_antiReverse(self_));
}
fn antiScalar_antiSandwich_multiVector(self_: AntiScalar, other: MultiVector) -> MultiVector {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let geometric_anti_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ (vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * other_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e1234_) * other_groups.group1_, 
        /* e41, e42, e43 */ (vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group2_, 
        /* e23, e31, e12 */ (vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e1234_) * other_groups.group4_
    ));
    return multiVector_geometricAntiProduct_antiScalar(geometric_anti_product, antiScalar_antiReverse(self_));
}
fn antiScalar_antiSandwich_origin(self_: AntiScalar, other: Origin) -> Origin {
    let geometric_anti_product: Origin = Origin(self_.e1234_ * other.e4_);
    return origin_geometricAntiProduct_antiScalar(geometric_anti_product, antiScalar_antiReverse(self_));
}
fn antiScalar_antiSandwich_plane(self_: AntiScalar, other: Plane) -> Plane {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = plane_grouped(other);
    let geometric_anti_product: Plane = plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e1234_) * other_groups.group0_
    ));
    return plane_geometricAntiProduct_antiScalar(geometric_anti_product, antiScalar_antiReverse(self_));
}
fn antiScalar_antiSandwich_point(self_: AntiScalar, other: Point) -> Point {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = point_grouped(other);
    let geometric_anti_product: Point = point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e1234_) * other_groups.group0_
    ));
    return point_geometricAntiProduct_antiScalar(geometric_anti_product, antiScalar_antiReverse(self_));
}
fn antiScalar_antiSandwich_scalar(self_: AntiScalar, other: Scalar) -> Scalar {
    let geometric_anti_product: Scalar = Scalar(self_.e1234_ * other.scalar);
    return scalar_geometricAntiProduct_antiScalar(geometric_anti_product, antiScalar_antiReverse(self_));
}
fn dualNum_antiSandwich_antiScalar(self_: DualNum, other: AntiScalar) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let geometric_anti_product: DualNum = dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ (vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_
    ));
    return dualNum_geometricAntiProduct_dualNum(geometric_anti_product, dualNum_antiReverse(self_));
}
fn dualNum_antiSandwich_dualNum(self_: DualNum, other: DualNum) -> DualNum {
    let geometric_anti_product: DualNum = dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>((other.scalar * self_.e1234_) + (other.e1234_ * self_.scalar), other.e1234_ * self_.e1234_, 0.0, 0.0)
    ));
    return dualNum_geometricAntiProduct_dualNum(geometric_anti_product, dualNum_antiReverse(self_));
}
fn dualNum_antiSandwich_flector(self_: DualNum, other: Flector) -> Flector {
    let geometric_anti_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.scalar * other.e423_) + (self_.e1234_ * other.e1_), (self_.scalar * other.e431_) + (self_.e1234_ * other.e2_), (self_.scalar * other.e412_) + (self_.e1234_ * other.e3_), self_.e1234_ * other.e4_), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e1234_ * other.e423_, self_.e1234_ * other.e431_, self_.e1234_ * other.e412_, (self_.scalar * other.e4_) + (self_.e1234_ * other.e321_))
    ));
    return flector_geometricAntiProduct_dualNum(geometric_anti_product, dualNum_antiReverse(self_));
}
fn dualNum_antiSandwich_horizon(self_: DualNum, other: Horizon) -> Horizon {
    let geometric_anti_product: Horizon = Horizon(self_.e1234_ * other.e321_);
    return horizon_geometricAntiProduct_dualNum(geometric_anti_product, dualNum_antiReverse(self_));
}
fn dualNum_antiSandwich_line(self_: DualNum, other: Line) -> Line {
    let self_groups = dualNum_grouped(self_);
    let other_groups = line_grouped(other);
    let geometric_anti_product: Line = line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group0_, 
        /* e23, e31, e12 */ ((vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group0_) + ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group1_)
    ));
    return line_geometricAntiProduct_dualNum(geometric_anti_product, dualNum_antiReverse(self_));
}
fn dualNum_antiSandwich_motor(self_: DualNum, other: Motor) -> Motor {
    let self_groups = dualNum_grouped(self_);
    let other_groups = motor_grouped(other);
    let geometric_anti_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.e1234_) * other_groups.group0_, 
        /* e23, e31, e12, scalar */ (vec4<f32>(self_.scalar) * other_groups.group0_) + (vec4<f32>(self_.e1234_) * other_groups.group1_)
    ));
    return motor_geometricAntiProduct_dualNum(geometric_anti_product, dualNum_antiReverse(self_));
}
fn dualNum_antiSandwich_multiVector(self_: DualNum, other: MultiVector) -> MultiVector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let geometric_anti_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((self_.scalar * other.e1234_) + (self_.e1234_ * other.scalar), self_.e1234_ * other.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.scalar * other.e423_) + (self_.e1234_ * other.e1_), (self_.scalar * other.e431_) + (self_.e1234_ * other.e2_), (self_.scalar * other.e412_) + (self_.e1234_ * other.e3_), self_.e1234_ * other.e4_), 
        /* e41, e42, e43 */ (vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group2_, 
        /* e23, e31, e12 */ ((vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group2_) + ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group3_), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e1234_ * other.e423_, self_.e1234_ * other.e431_, self_.e1234_ * other.e412_, (self_.scalar * other.e4_) + (self_.e1234_ * other.e321_))
    ));
    return multiVector_geometricAntiProduct_dualNum(geometric_anti_product, dualNum_antiReverse(self_));
}
fn dualNum_antiSandwich_origin(self_: DualNum, other: Origin) -> Flector {
    let geometric_anti_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, self_.e1234_ * other.e4_) * vec4<f32>(0.0, 0.0, 0.0, 1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0, 1.0, 1.0, self_.scalar * other.e4_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)
    ));
    return flector_geometricAntiProduct_dualNum(geometric_anti_product, dualNum_antiReverse(self_));
}
fn dualNum_antiSandwich_plane(self_: DualNum, other: Plane) -> Flector {
    let other_groups = plane_grouped(other);
    let geometric_anti_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.scalar * other.e423_, self_.scalar * other.e431_, self_.scalar * other.e412_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e1234_) * other_groups.group0_
    ));
    return flector_geometricAntiProduct_dualNum(geometric_anti_product, dualNum_antiReverse(self_));
}
fn dualNum_antiSandwich_point(self_: DualNum, other: Point) -> Flector {
    let other_groups = point_grouped(other);
    let geometric_anti_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e1234_) * other_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0, 1.0, 1.0, self_.scalar * other.e4_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)
    ));
    return flector_geometricAntiProduct_dualNum(geometric_anti_product, dualNum_antiReverse(self_));
}
fn dualNum_antiSandwich_scalar(self_: DualNum, other: Scalar) -> Scalar {
    let geometric_anti_product: Scalar = Scalar(self_.e1234_ * other.scalar);
    return scalar_geometricAntiProduct_dualNum(geometric_anti_product, dualNum_antiReverse(self_));
}
fn flector_antiSandwich_antiScalar(self_: Flector, other: AntiScalar) -> Motor {
    let self_groups = flector_grouped(self_);
    let geometric_anti_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e1234_) * self_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e1234_) * self_groups.group1_
    ));
    return flector_geometricAntiProduct_flector(geometric_anti_product, flector_antiReverse(self_));
}
fn flector_antiSandwich_dualNum(self_: Flector, other: DualNum) -> Motor {
    let self_groups = flector_grouped(self_);
    let geometric_anti_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other.e1234_ * self_.e1_) - (other.scalar * self_.e423_), (other.e1234_ * self_.e2_) - (other.scalar * self_.e431_), (other.e1234_ * self_.e3_) - (other.scalar * self_.e412_), other.e1234_ * self_.e4_), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e1234_ * self_.e423_, other.e1234_ * self_.e431_, other.e1234_ * self_.e412_, (other.e1234_ * self_.e321_) - (other.scalar * self_.e4_))
    ));
    return flector_geometricAntiProduct_flector(geometric_anti_product, flector_antiReverse(self_));
}
fn flector_antiSandwich_flector(self_: Flector, other: Flector) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = flector_grouped(other);
    let geometric_anti_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other.e431_ * self_.e412_) - (other.e423_ * self_.e4_) - (other.e412_ * self_.e431_), (other.e412_ * self_.e423_) - (other.e423_ * self_.e412_) - (other.e431_ * self_.e4_), (other.e423_ * self_.e431_) - (other.e431_ * self_.e423_) - (other.e412_ * self_.e4_), (other.e423_ * self_.e423_) + (other.e431_ * self_.e431_) + (other.e412_ * self_.e412_)) - (vec4<f32>(other.e4_) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, self_.e4_)), 
        /* e23, e31, e12, scalar */ vec4<f32>((other.e3_ * self_.e431_) + (other.e431_ * self_.e3_) + (other.e321_ * self_.e423_) - (other.e2_ * self_.e412_) - (other.e423_ * self_.e321_) - (other.e412_ * self_.e2_), (other.e1_ * self_.e412_) + (other.e412_ * self_.e1_) + (other.e321_ * self_.e431_) - (other.e3_ * self_.e423_) - (other.e423_ * self_.e3_) - (other.e431_ * self_.e321_), (other.e2_ * self_.e423_) + (other.e423_ * self_.e2_) + (other.e321_ * self_.e412_) - (other.e1_ * self_.e431_) - (other.e431_ * self_.e1_) - (other.e412_ * self_.e321_), (other.e423_ * self_.e1_) + (other.e431_ * self_.e2_) + (other.e412_ * self_.e3_) - (other.e1_ * self_.e423_) - (other.e2_ * self_.e431_) - (other.e3_ * self_.e412_)) + (vec4<f32>(self_.e4_) * vec4<f32>(other.e1_, other.e2_, other.e3_, other.e321_)) - (vec4<f32>(other.e4_) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, self_.e321_))
    ));
    return motor_geometricAntiProduct_flector(geometric_anti_product, flector_antiReverse(self_));
}
fn flector_antiSandwich_horizon(self_: Flector, other: Horizon) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = horizon_grouped(other);
    let geometric_anti_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(other.e321_) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, self_.e4_)
    ));
    return motor_geometricAntiProduct_flector(geometric_anti_product, flector_antiReverse(self_));
}
fn flector_antiSandwich_line(self_: Flector, other: Line) -> Motor {
    let self_groups = flector_grouped(self_);
    let geometric_anti_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e2_ * other.e43_) + (self_.e412_ * other.e31_) + (self_.e321_ * other.e41_) - (self_.e3_ * other.e42_) - (self_.e4_ * other.e23_) - (self_.e431_ * other.e12_), (self_.e3_ * other.e41_) + (self_.e423_ * other.e12_) + (self_.e321_ * other.e42_) - (self_.e1_ * other.e43_) - (self_.e4_ * other.e31_) - (self_.e412_ * other.e23_), (self_.e1_ * other.e42_) + (self_.e431_ * other.e23_) + (self_.e321_ * other.e43_) - (self_.e2_ * other.e41_) - (self_.e4_ * other.e12_) - (self_.e423_ * other.e31_), -(self_.e423_ * other.e41_) - (self_.e431_ * other.e42_) - (self_.e412_ * other.e43_)), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e4_ * other.e41_) + (self_.e431_ * other.e43_) - (self_.e412_ * other.e42_), (self_.e4_ * other.e42_) + (self_.e412_ * other.e41_) - (self_.e423_ * other.e43_), (self_.e4_ * other.e43_) + (self_.e423_ * other.e42_) - (self_.e431_ * other.e41_), (self_.e423_ * other.e23_) + (self_.e431_ * other.e31_) + (self_.e412_ * other.e12_) - (self_.e1_ * other.e41_) - (self_.e2_ * other.e42_) - (self_.e3_ * other.e43_))
    ));
    return flector_geometricAntiProduct_flector(geometric_anti_product, flector_antiReverse(self_));
}
fn flector_antiSandwich_motor(self_: Flector, other: Motor) -> Motor {
    let self_groups = flector_grouped(self_);
    let geometric_anti_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e2_ * other.e43_) + (self_.e412_ * other.e31_) + (self_.e321_ * other.e41_) - (self_.e3_ * other.e42_) - (self_.e4_ * other.e23_) - (self_.e423_ * other.scalar) - (self_.e431_ * other.e12_), (self_.e3_ * other.e41_) + (self_.e423_ * other.e12_) + (self_.e321_ * other.e42_) - (self_.e1_ * other.e43_) - (self_.e4_ * other.e31_) - (self_.e431_ * other.scalar) - (self_.e412_ * other.e23_), (self_.e1_ * other.e42_) + (self_.e431_ * other.e23_) + (self_.e321_ * other.e43_) - (self_.e2_ * other.e41_) - (self_.e4_ * other.e12_) - (self_.e423_ * other.e31_) - (self_.e412_ * other.scalar), -(self_.e423_ * other.e41_) - (self_.e431_ * other.e42_) - (self_.e412_ * other.e43_)) + (vec4<f32>(other.e1234_) * self_groups.group0_), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e4_ * other.e41_) + (self_.e431_ * other.e43_) - (self_.e412_ * other.e42_), (self_.e4_ * other.e42_) + (self_.e412_ * other.e41_) - (self_.e423_ * other.e43_), (self_.e4_ * other.e43_) + (self_.e423_ * other.e42_) - (self_.e431_ * other.e41_), (self_.e423_ * other.e23_) + (self_.e431_ * other.e31_) + (self_.e412_ * other.e12_) - (self_.e1_ * other.e41_) - (self_.e2_ * other.e42_) - (self_.e3_ * other.e43_) - (self_.e4_ * other.scalar)) + (vec4<f32>(other.e1234_) * self_groups.group1_)
    ));
    return flector_geometricAntiProduct_flector(geometric_anti_product, flector_antiReverse(self_));
}
fn flector_antiSandwich_multiVector(self_: Flector, other: MultiVector) -> MultiVector {
    let self_groups = flector_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let geometric_anti_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((self_.e4_ * other.e321_) - (self_.e423_ * other.e1_) - (self_.e431_ * other.e2_) - (self_.e412_ * other.e3_), 0.0, 0.0, 0.0) + ((vec4<f32>(other.e423_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e1_, self_.e423_, 0.0, 0.0)) + ((vec4<f32>(other.e431_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e2_, self_.e431_, 0.0, 0.0)) + ((vec4<f32>(other.e412_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e3_, self_.e412_, 0.0, 0.0)) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e321_, self_.e4_, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e2_ * other.e43_) + (self_.e412_ * other.e31_) + (self_.e321_ * other.e41_) - (self_.e3_ * other.e42_) - (self_.e4_ * other.e23_) - (self_.e423_ * other.scalar) - (self_.e431_ * other.e12_), (self_.e3_ * other.e41_) + (self_.e423_ * other.e12_) + (self_.e321_ * other.e42_) - (self_.e1_ * other.e43_) - (self_.e4_ * other.e31_) - (self_.e431_ * other.scalar) - (self_.e412_ * other.e23_), (self_.e1_ * other.e42_) + (self_.e431_ * other.e23_) + (self_.e321_ * other.e43_) - (self_.e2_ * other.e41_) - (self_.e4_ * other.e12_) - (self_.e423_ * other.e31_) - (self_.e412_ * other.scalar), -(self_.e423_ * other.e41_) - (self_.e431_ * other.e42_) - (self_.e412_ * other.e43_)) + (vec4<f32>(other.e1234_) * self_groups.group0_), 
        /* e41, e42, e43 */ vec4<f32>((self_.e412_ * other.e431_) - (self_.e431_ * other.e412_), (self_.e423_ * other.e412_) - (self_.e412_ * other.e423_), (self_.e431_ * other.e423_) - (self_.e423_ * other.e431_), 0.0) - ((vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0)) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((self_.e3_ * other.e431_) + (self_.e431_ * other.e3_) - (self_.e2_ * other.e412_) - (self_.e412_ * other.e2_), (self_.e1_ * other.e412_) + (self_.e412_ * other.e1_) - (self_.e3_ * other.e423_) - (self_.e423_ * other.e3_), (self_.e2_ * other.e423_) + (self_.e423_ * other.e2_) - (self_.e1_ * other.e431_) - (self_.e431_ * other.e1_), 0.0) + ((vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0)) + ((vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0)) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0)) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e4_ * other.e41_) + (self_.e431_ * other.e43_) - (self_.e412_ * other.e42_), (self_.e4_ * other.e42_) + (self_.e412_ * other.e41_) - (self_.e423_ * other.e43_), (self_.e4_ * other.e43_) + (self_.e423_ * other.e42_) - (self_.e431_ * other.e41_), (self_.e423_ * other.e23_) + (self_.e431_ * other.e31_) + (self_.e412_ * other.e12_) - (self_.e1_ * other.e41_) - (self_.e2_ * other.e42_) - (self_.e3_ * other.e43_) - (self_.e4_ * other.scalar)) + (vec4<f32>(other.e1234_) * self_groups.group1_)
    ));
    return multiVector_geometricAntiProduct_flector(geometric_anti_product, flector_antiReverse(self_));
}
fn flector_antiSandwich_origin(self_: Flector, other: Origin) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = origin_grouped(other);
    let geometric_anti_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e4_) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, self_.e4_) * vec4<f32>(-1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(other.e4_) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, self_.e321_) * vec4<f32>(-1.0)
    ));
    return motor_geometricAntiProduct_flector(geometric_anti_product, flector_antiReverse(self_));
}
fn flector_antiSandwich_plane(self_: Flector, other: Plane) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = plane_grouped(other);
    let geometric_anti_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((self_.e412_ * other.e431_) - (self_.e4_ * other.e423_) - (self_.e431_ * other.e412_), (self_.e423_ * other.e412_) - (self_.e4_ * other.e431_) - (self_.e412_ * other.e423_), (self_.e431_ * other.e423_) - (self_.e4_ * other.e412_) - (self_.e423_ * other.e431_), (self_.e423_ * other.e423_) + (self_.e431_ * other.e431_) + (self_.e412_ * other.e412_)), 
        /* e23, e31, e12, scalar */ vec4<f32>((self_.e3_ * other.e431_) - (self_.e2_ * other.e412_) - (self_.e321_ * other.e423_), (self_.e1_ * other.e412_) - (self_.e3_ * other.e423_) - (self_.e321_ * other.e431_), (self_.e2_ * other.e423_) - (self_.e1_ * other.e431_) - (self_.e321_ * other.e412_), (self_.e1_ * other.e423_) + (self_.e2_ * other.e431_) + (self_.e3_ * other.e412_)) + (vec4<f32>(other.e321_) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, self_.e4_))
    ));
    return motor_geometricAntiProduct_flector(geometric_anti_product, flector_antiReverse(self_));
}
fn flector_antiSandwich_point(self_: Flector, other: Point) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = point_grouped(other);
    let geometric_anti_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e4_) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, self_.e4_) * vec4<f32>(-1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>((self_.e4_ * other.e1_) + (self_.e431_ * other.e3_) - (self_.e412_ * other.e2_), (self_.e4_ * other.e2_) + (self_.e412_ * other.e1_) - (self_.e423_ * other.e3_), (self_.e4_ * other.e3_) + (self_.e423_ * other.e2_) - (self_.e431_ * other.e1_), -(self_.e423_ * other.e1_) - (self_.e431_ * other.e2_) - (self_.e412_ * other.e3_)) - (vec4<f32>(other.e4_) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, self_.e321_))
    ));
    return motor_geometricAntiProduct_flector(geometric_anti_product, flector_antiReverse(self_));
}
fn flector_antiSandwich_scalar(self_: Flector, other: Scalar) -> Motor {
    let self_groups = flector_grouped(self_);
    let geometric_anti_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e423_ * other.scalar, self_.e431_ * other.scalar, self_.e412_ * other.scalar, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0, 1.0, 1.0, self_.e4_ * other.scalar) * vec4<f32>(0.0, 0.0, 0.0, -1.0)
    ));
    return flector_geometricAntiProduct_flector(geometric_anti_product, flector_antiReverse(self_));
}
fn horizon_antiSandwich_flector(self_: Horizon, other: Flector) -> Flector {
    let geometric_anti_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.e321_) * vec4<f32>(other.e423_, other.e431_, other.e412_, other.e4_) * vec4<f32>(-1.0)
    ));
    return motor_geometricAntiProduct_horizon(geometric_anti_product, horizon_antiReverse(self_));
}
fn horizon_antiSandwich_line(self_: Horizon, other: Line) -> Scalar {
    let geometric_anti_product: Point = point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e321_ * other.e41_, self_.e321_ * other.e42_, self_.e321_ * other.e43_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
    return point_geometricAntiProduct_horizon(geometric_anti_product, horizon_antiReverse(self_));
}
fn horizon_antiSandwich_motor(self_: Horizon, other: Motor) -> Motor {
    let geometric_anti_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e321_ * other.e41_, self_.e321_ * other.e42_, self_.e321_ * other.e43_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0, 1.0, 1.0, self_.e321_ * other.e1234_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)
    ));
    return flector_geometricAntiProduct_horizon(geometric_anti_product, horizon_antiReverse(self_));
}
fn horizon_antiSandwich_multiVector(self_: Horizon, other: MultiVector) -> MultiVector {
    let geometric_anti_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.e321_ * other.e4_, 1.0, 0.0, 0.0) * vec2<f32>(-1.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e321_ * other.e41_, self_.e321_ * other.e42_, self_.e321_ * other.e43_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ (vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0) * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0, 1.0, 1.0, self_.e321_ * other.e1234_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)
    ));
    return multiVector_geometricAntiProduct_horizon(geometric_anti_product, horizon_antiReverse(self_));
}
fn horizon_antiSandwich_plane(self_: Horizon, other: Plane) -> Point {
    let geometric_anti_product: Line = line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ (vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0) * vec3<f32>(-1.0)
    ));
    return line_geometricAntiProduct_horizon(geometric_anti_product, horizon_antiReverse(self_));
}
fn line_antiSandwich_antiScalar(self_: Line, other: AntiScalar) -> Motor {
    let self_groups = line_grouped(self_);
    let geometric_anti_product: Line = line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_
    ));
    return line_geometricAntiProduct_line(geometric_anti_product, line_antiReverse(self_));
}
fn line_antiSandwich_dualNum(self_: Line, other: DualNum) -> Motor {
    let self_groups = line_grouped(self_);
    let geometric_anti_product: Line = line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_, 
        /* e23, e31, e12 */ ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_) + ((vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_)
    ));
    return line_geometricAntiProduct_line(geometric_anti_product, line_antiReverse(self_));
}
fn line_antiSandwich_flector(self_: Line, other: Flector) -> Flector {
    let self_groups = line_grouped(self_);
    let geometric_anti_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other.e3_ * self_.e42_) + (other.e4_ * self_.e23_) + (other.e412_ * self_.e31_) + (other.e321_ * self_.e41_) - (other.e2_ * self_.e43_) - (other.e431_ * self_.e12_), (other.e1_ * self_.e43_) + (other.e4_ * self_.e31_) + (other.e423_ * self_.e12_) + (other.e321_ * self_.e42_) - (other.e3_ * self_.e41_) - (other.e412_ * self_.e23_), (other.e2_ * self_.e41_) + (other.e4_ * self_.e12_) + (other.e431_ * self_.e23_) + (other.e321_ * self_.e43_) - (other.e1_ * self_.e42_) - (other.e423_ * self_.e31_), -(other.e423_ * self_.e41_) - (other.e431_ * self_.e42_) - (other.e412_ * self_.e43_)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e4_ * self_.e41_) + (other.e412_ * self_.e42_) - (other.e431_ * self_.e43_), (other.e4_ * self_.e42_) + (other.e423_ * self_.e43_) - (other.e412_ * self_.e41_), (other.e4_ * self_.e43_) + (other.e431_ * self_.e41_) - (other.e423_ * self_.e42_), -(other.e1_ * self_.e41_) - (other.e2_ * self_.e42_) - (other.e3_ * self_.e43_) - (other.e423_ * self_.e23_) - (other.e431_ * self_.e31_) - (other.e412_ * self_.e12_))
    ));
    return flector_geometricAntiProduct_line(geometric_anti_product, line_antiReverse(self_));
}
fn line_antiSandwich_horizon(self_: Line, other: Horizon) -> Flector {
    let self_groups = line_grouped(self_);
    let geometric_anti_product: Point = point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e321_ * self_.e41_, other.e321_ * self_.e42_, other.e321_ * self_.e43_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
    return point_geometricAntiProduct_line(geometric_anti_product, line_antiReverse(self_));
}
fn line_antiSandwich_line(self_: Line, other: Line) -> Motor {
    let self_groups = line_grouped(self_);
    let geometric_anti_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other.e43_ * self_.e42_) - (other.e42_ * self_.e43_), (other.e41_ * self_.e43_) - (other.e43_ * self_.e41_), (other.e42_ * self_.e41_) - (other.e41_ * self_.e42_), -(other.e41_ * self_.e41_) - (other.e42_ * self_.e42_) - (other.e43_ * self_.e43_)), 
        /* e23, e31, e12, scalar */ vec4<f32>((other.e43_ * self_.e31_) + (other.e12_ * self_.e42_) - (other.e42_ * self_.e12_) - (other.e31_ * self_.e43_), (other.e41_ * self_.e12_) + (other.e23_ * self_.e43_) - (other.e43_ * self_.e23_) - (other.e12_ * self_.e41_), (other.e42_ * self_.e23_) + (other.e31_ * self_.e41_) - (other.e41_ * self_.e31_) - (other.e23_ * self_.e42_), -(other.e41_ * self_.e23_) - (other.e42_ * self_.e31_) - (other.e43_ * self_.e12_) - (other.e23_ * self_.e41_) - (other.e31_ * self_.e42_) - (other.e12_ * self_.e43_))
    ));
    return motor_geometricAntiProduct_line(geometric_anti_product, line_antiReverse(self_));
}
fn line_antiSandwich_motor(self_: Line, other: Motor) -> Motor {
    let self_groups = line_grouped(self_);
    let geometric_anti_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((self_.e41_ * other.e1234_) + (self_.e42_ * other.e43_) - (self_.e43_ * other.e42_), (self_.e42_ * other.e1234_) + (self_.e43_ * other.e41_) - (self_.e41_ * other.e43_), (self_.e41_ * other.e42_) + (self_.e43_ * other.e1234_) - (self_.e42_ * other.e41_), -(self_.e41_ * other.e41_) - (self_.e42_ * other.e42_) - (self_.e43_ * other.e43_)), 
        /* e23, e31, e12, scalar */ vec4<f32>((self_.e41_ * other.scalar) + (self_.e42_ * other.e12_) + (self_.e23_ * other.e1234_) + (self_.e31_ * other.e43_) - (self_.e43_ * other.e31_) - (self_.e12_ * other.e42_), (self_.e42_ * other.scalar) + (self_.e43_ * other.e23_) + (self_.e31_ * other.e1234_) + (self_.e12_ * other.e41_) - (self_.e41_ * other.e12_) - (self_.e23_ * other.e43_), (self_.e41_ * other.e31_) + (self_.e43_ * other.scalar) + (self_.e23_ * other.e42_) + (self_.e12_ * other.e1234_) - (self_.e42_ * other.e23_) - (self_.e31_ * other.e41_), -(self_.e41_ * other.e23_) - (self_.e42_ * other.e31_) - (self_.e43_ * other.e12_) - (self_.e23_ * other.e41_) - (self_.e31_ * other.e42_) - (self_.e12_ * other.e43_))
    ));
    return motor_geometricAntiProduct_line(geometric_anti_product, line_antiReverse(self_));
}
fn line_antiSandwich_multiVector(self_: Line, other: MultiVector) -> MultiVector {
    let self_groups = line_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let geometric_anti_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(-(self_.e23_ * other.e41_) - (self_.e31_ * other.e42_) - (self_.e12_ * other.e43_), 0.0, 0.0, 0.0) - ((vec4<f32>(self_.e41_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e23_, other.e41_, 0.0, 0.0)) - ((vec4<f32>(self_.e42_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e31_, other.e42_, 0.0, 0.0)) - ((vec4<f32>(self_.e43_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e12_, other.e43_, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e41_ * other.e321_) + (self_.e42_ * other.e3_) + (self_.e23_ * other.e4_) + (self_.e31_ * other.e412_) - (self_.e43_ * other.e2_) - (self_.e12_ * other.e431_), (self_.e42_ * other.e321_) + (self_.e43_ * other.e1_) + (self_.e31_ * other.e4_) + (self_.e12_ * other.e423_) - (self_.e41_ * other.e3_) - (self_.e23_ * other.e412_), (self_.e41_ * other.e2_) + (self_.e43_ * other.e321_) + (self_.e23_ * other.e431_) + (self_.e12_ * other.e4_) - (self_.e42_ * other.e1_) - (self_.e31_ * other.e423_), -(self_.e41_ * other.e423_) - (self_.e42_ * other.e431_) - (self_.e43_ * other.e412_)), 
        /* e41, e42, e43 */ vec4<f32>((self_.e42_ * other.e43_) - (self_.e43_ * other.e42_), (self_.e43_ * other.e41_) - (self_.e41_ * other.e43_), (self_.e41_ * other.e42_) - (self_.e42_ * other.e41_), 0.0) + ((vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_), 
        /* e23, e31, e12 */ vec4<f32>((self_.e42_ * other.e12_) + (self_.e31_ * other.e43_) - (self_.e43_ * other.e31_) - (self_.e12_ * other.e42_), (self_.e43_ * other.e23_) + (self_.e12_ * other.e41_) - (self_.e41_ * other.e12_) - (self_.e23_ * other.e43_), (self_.e41_ * other.e31_) + (self_.e23_ * other.e42_) - (self_.e42_ * other.e23_) - (self_.e31_ * other.e41_), 0.0) + ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_) + ((vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group1_), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e41_ * other.e4_) + (self_.e42_ * other.e412_) - (self_.e43_ * other.e431_), (self_.e42_ * other.e4_) + (self_.e43_ * other.e423_) - (self_.e41_ * other.e412_), (self_.e41_ * other.e431_) + (self_.e43_ * other.e4_) - (self_.e42_ * other.e423_), -(self_.e41_ * other.e1_) - (self_.e42_ * other.e2_) - (self_.e43_ * other.e3_) - (self_.e23_ * other.e423_) - (self_.e31_ * other.e431_) - (self_.e12_ * other.e412_))
    ));
    return multiVector_geometricAntiProduct_line(geometric_anti_product, line_antiReverse(self_));
}
fn line_antiSandwich_origin(self_: Line, other: Origin) -> Flector {
    let self_groups = line_grouped(self_);
    let geometric_anti_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e23_ * other.e4_, self_.e31_ * other.e4_, self_.e12_ * other.e4_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e41_ * other.e4_, self_.e42_ * other.e4_, self_.e43_ * other.e4_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
    return flector_geometricAntiProduct_line(geometric_anti_product, line_antiReverse(self_));
}
fn line_antiSandwich_plane(self_: Line, other: Plane) -> Flector {
    let self_groups = line_grouped(self_);
    let geometric_anti_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e41_ * other.e321_) + (self_.e31_ * other.e412_) - (self_.e12_ * other.e431_), (self_.e42_ * other.e321_) + (self_.e12_ * other.e423_) - (self_.e23_ * other.e412_), (self_.e43_ * other.e321_) + (self_.e23_ * other.e431_) - (self_.e31_ * other.e423_), -(self_.e41_ * other.e423_) - (self_.e42_ * other.e431_) - (self_.e43_ * other.e412_)), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e42_ * other.e412_) - (self_.e43_ * other.e431_), (self_.e43_ * other.e423_) - (self_.e41_ * other.e412_), (self_.e41_ * other.e431_) - (self_.e42_ * other.e423_), -(self_.e23_ * other.e423_) - (self_.e31_ * other.e431_) - (self_.e12_ * other.e412_))
    ));
    return flector_geometricAntiProduct_line(geometric_anti_product, line_antiReverse(self_));
}
fn line_antiSandwich_point(self_: Line, other: Point) -> Flector {
    let self_groups = line_grouped(self_);
    let geometric_anti_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e42_ * other.e3_) + (self_.e23_ * other.e4_) - (self_.e43_ * other.e2_), (self_.e43_ * other.e1_) + (self_.e31_ * other.e4_) - (self_.e41_ * other.e3_), (self_.e41_ * other.e2_) + (self_.e12_ * other.e4_) - (self_.e42_ * other.e1_), 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e41_ * other.e4_, self_.e42_ * other.e4_, self_.e43_ * other.e4_, -(self_.e41_ * other.e1_) - (self_.e42_ * other.e2_) - (self_.e43_ * other.e3_))
    ));
    return flector_geometricAntiProduct_line(geometric_anti_product, line_antiReverse(self_));
}
fn line_antiSandwich_scalar(self_: Line, other: Scalar) -> Motor {
    let self_groups = line_grouped(self_);
    let geometric_anti_product: Line = line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ (vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group0_
    ));
    return line_geometricAntiProduct_line(geometric_anti_product, line_antiReverse(self_));
}
fn motor_antiSandwich_antiScalar(self_: Motor, other: AntiScalar) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    let geometric_anti_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e1234_) * self_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(other.e1234_) * self_groups.group1_
    ));
    return motor_geometricAntiProduct_motor(geometric_anti_product, motor_antiReverse(self_));
}
fn motor_antiSandwich_dualNum(self_: Motor, other: DualNum) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let geometric_anti_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e1234_) * self_groups.group0_, 
        /* e23, e31, e12, scalar */ (vec4<f32>(other.scalar) * self_groups.group0_) + (vec4<f32>(other.e1234_) * self_groups.group1_)
    ));
    return motor_geometricAntiProduct_motor(geometric_anti_product, motor_antiReverse(self_));
}
fn motor_antiSandwich_flector(self_: Motor, other: Flector) -> Flector {
    let self_groups = motor_grouped(self_);
    let other_groups = flector_grouped(other);
    let geometric_anti_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other.e3_ * self_.e42_) + (other.e4_ * self_.e23_) + (other.e423_ * self_.scalar) + (other.e412_ * self_.e31_) + (other.e321_ * self_.e41_) - (other.e2_ * self_.e43_) - (other.e431_ * self_.e12_), (other.e1_ * self_.e43_) + (other.e4_ * self_.e31_) + (other.e423_ * self_.e12_) + (other.e431_ * self_.scalar) + (other.e321_ * self_.e42_) - (other.e3_ * self_.e41_) - (other.e412_ * self_.e23_), (other.e2_ * self_.e41_) + (other.e4_ * self_.e12_) + (other.e431_ * self_.e23_) + (other.e412_ * self_.scalar) + (other.e321_ * self_.e43_) - (other.e1_ * self_.e42_) - (other.e423_ * self_.e31_), -(other.e423_ * self_.e41_) - (other.e431_ * self_.e42_) - (other.e412_ * self_.e43_)) + (vec4<f32>(self_.e1234_) * other_groups.group0_), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e412_ * self_.e42_) - (other.e431_ * self_.e43_), (other.e423_ * self_.e43_) - (other.e412_ * self_.e41_), (other.e431_ * self_.e41_) - (other.e423_ * self_.e42_), -(other.e1_ * self_.e41_) - (other.e2_ * self_.e42_) - (other.e3_ * self_.e43_) - (other.e423_ * self_.e23_) - (other.e431_ * self_.e31_) - (other.e412_ * self_.e12_)) + (vec4<f32>(other.e4_) * vec4<f32>(self_.e41_, self_.e42_, self_.e43_, self_.scalar)) + (vec4<f32>(self_.e1234_) * other_groups.group1_)
    ));
    return flector_geometricAntiProduct_motor(geometric_anti_product, motor_antiReverse(self_));
}
fn motor_antiSandwich_horizon(self_: Motor, other: Horizon) -> Flector {
    let self_groups = motor_grouped(self_);
    let geometric_anti_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e321_ * self_.e41_, other.e321_ * self_.e42_, other.e321_ * self_.e43_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0, 1.0, 1.0, other.e321_ * self_.e1234_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)
    ));
    return flector_geometricAntiProduct_motor(geometric_anti_product, motor_antiReverse(self_));
}
fn motor_antiSandwich_line(self_: Motor, other: Line) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = line_grouped(other);
    let geometric_anti_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other.e41_ * self_.e1234_) + (other.e43_ * self_.e42_) - (other.e42_ * self_.e43_), (other.e41_ * self_.e43_) + (other.e42_ * self_.e1234_) - (other.e43_ * self_.e41_), (other.e42_ * self_.e41_) + (other.e43_ * self_.e1234_) - (other.e41_ * self_.e42_), -(other.e41_ * self_.e41_) - (other.e42_ * self_.e42_) - (other.e43_ * self_.e43_)), 
        /* e23, e31, e12, scalar */ vec4<f32>((other.e41_ * self_.scalar) + (other.e43_ * self_.e31_) + (other.e23_ * self_.e1234_) + (other.e12_ * self_.e42_) - (other.e42_ * self_.e12_) - (other.e31_ * self_.e43_), (other.e41_ * self_.e12_) + (other.e42_ * self_.scalar) + (other.e23_ * self_.e43_) + (other.e31_ * self_.e1234_) - (other.e43_ * self_.e23_) - (other.e12_ * self_.e41_), (other.e42_ * self_.e23_) + (other.e43_ * self_.scalar) + (other.e31_ * self_.e41_) + (other.e12_ * self_.e1234_) - (other.e41_ * self_.e31_) - (other.e23_ * self_.e42_), -(other.e41_ * self_.e23_) - (other.e42_ * self_.e31_) - (other.e43_ * self_.e12_) - (other.e23_ * self_.e41_) - (other.e31_ * self_.e42_) - (other.e12_ * self_.e43_))
    ));
    return motor_geometricAntiProduct_motor(geometric_anti_product, motor_antiReverse(self_));
}
fn motor_antiSandwich_motor(self_: Motor, other: Motor) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = motor_grouped(other);
    let geometric_anti_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other.e43_ * self_.e42_) + (other.e1234_ * self_.e41_) - (other.e42_ * self_.e43_), (other.e41_ * self_.e43_) + (other.e1234_ * self_.e42_) - (other.e43_ * self_.e41_), (other.e42_ * self_.e41_) + (other.e1234_ * self_.e43_) - (other.e41_ * self_.e42_), -(other.e41_ * self_.e41_) - (other.e42_ * self_.e42_) - (other.e43_ * self_.e43_)) + (vec4<f32>(self_.e1234_) * other_groups.group0_), 
        /* e23, e31, e12, scalar */ vec4<f32>((other.e43_ * self_.e31_) + (other.e1234_ * self_.e23_) + (other.e12_ * self_.e42_) + (other.scalar * self_.e41_) - (other.e42_ * self_.e12_) - (other.e31_ * self_.e43_), (other.e41_ * self_.e12_) + (other.e1234_ * self_.e31_) + (other.e23_ * self_.e43_) + (other.scalar * self_.e42_) - (other.e43_ * self_.e23_) - (other.e12_ * self_.e41_), (other.e42_ * self_.e23_) + (other.e1234_ * self_.e12_) + (other.e31_ * self_.e41_) + (other.scalar * self_.e43_) - (other.e41_ * self_.e31_) - (other.e23_ * self_.e42_), -(other.e41_ * self_.e23_) - (other.e42_ * self_.e31_) - (other.e43_ * self_.e12_) - (other.e23_ * self_.e41_) - (other.e31_ * self_.e42_) - (other.e12_ * self_.e43_)) + (vec4<f32>(self_.e1234_) * other_groups.group1_) + (vec4<f32>(self_.scalar) * other_groups.group0_)
    ));
    return motor_geometricAntiProduct_motor(geometric_anti_product, motor_antiReverse(self_));
}
fn motor_antiSandwich_multiVector(self_: Motor, other: MultiVector) -> MultiVector {
    let self_groups = motor_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let geometric_anti_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((self_.scalar * other.e1234_) - (self_.e23_ * other.e41_) - (self_.e31_ * other.e42_) - (self_.e12_ * other.e43_), 0.0, 0.0, 0.0) + ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * other_groups.group0_) - ((vec4<f32>(self_.e41_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e23_, other.e41_, 0.0, 0.0)) - ((vec4<f32>(self_.e42_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e31_, other.e42_, 0.0, 0.0)) - ((vec4<f32>(self_.e43_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e12_, other.e43_, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e41_ * other.e321_) + (self_.e42_ * other.e3_) + (self_.e23_ * other.e4_) + (self_.e31_ * other.e412_) + (self_.scalar * other.e423_) - (self_.e43_ * other.e2_) - (self_.e12_ * other.e431_), (self_.e42_ * other.e321_) + (self_.e43_ * other.e1_) + (self_.e31_ * other.e4_) + (self_.e12_ * other.e423_) + (self_.scalar * other.e431_) - (self_.e41_ * other.e3_) - (self_.e23_ * other.e412_), (self_.e41_ * other.e2_) + (self_.e43_ * other.e321_) + (self_.e23_ * other.e431_) + (self_.e12_ * other.e4_) + (self_.scalar * other.e412_) - (self_.e42_ * other.e1_) - (self_.e31_ * other.e423_), -(self_.e41_ * other.e423_) - (self_.e42_ * other.e431_) - (self_.e43_ * other.e412_)) + (vec4<f32>(self_.e1234_) * other_groups.group1_), 
        /* e41, e42, e43 */ vec4<f32>((self_.e42_ * other.e43_) - (self_.e43_ * other.e42_), (self_.e43_ * other.e41_) - (self_.e41_ * other.e43_), (self_.e41_ * other.e42_) - (self_.e42_ * other.e41_), 0.0) + ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group2_) + ((vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e41_, self_.e42_, self_.e43_, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((self_.e42_ * other.e12_) + (self_.e31_ * other.e43_) - (self_.e43_ * other.e31_) - (self_.e12_ * other.e42_), (self_.e43_ * other.e23_) + (self_.e12_ * other.e41_) - (self_.e41_ * other.e12_) - (self_.e23_ * other.e43_), (self_.e41_ * other.e31_) + (self_.e23_ * other.e42_) - (self_.e42_ * other.e23_) - (self_.e31_ * other.e41_), 0.0) + ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group3_) + ((vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group2_) + ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e41_, self_.e42_, self_.e43_, 0.0)) + ((vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e23_, self_.e31_, self_.e12_, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e42_ * other.e412_) - (self_.e43_ * other.e431_), (self_.e43_ * other.e423_) - (self_.e41_ * other.e412_), (self_.e41_ * other.e431_) - (self_.e42_ * other.e423_), -(self_.e41_ * other.e1_) - (self_.e42_ * other.e2_) - (self_.e43_ * other.e3_) - (self_.e23_ * other.e423_) - (self_.e31_ * other.e431_) - (self_.e12_ * other.e412_)) + (vec4<f32>(self_.e1234_) * other_groups.group4_) + (vec4<f32>(other.e4_) * vec4<f32>(self_.e41_, self_.e42_, self_.e43_, self_.scalar))
    ));
    return multiVector_geometricAntiProduct_motor(geometric_anti_product, motor_antiReverse(self_));
}
fn motor_antiSandwich_origin(self_: Motor, other: Origin) -> Flector {
    let self_groups = motor_grouped(self_);
    let geometric_anti_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e4_) * vec4<f32>(self_.e23_, self_.e31_, self_.e12_, self_.e1234_), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e4_) * vec4<f32>(self_.e41_, self_.e42_, self_.e43_, self_.scalar)
    ));
    return flector_geometricAntiProduct_motor(geometric_anti_product, motor_antiReverse(self_));
}
fn motor_antiSandwich_plane(self_: Motor, other: Plane) -> Flector {
    let self_groups = motor_grouped(self_);
    let other_groups = plane_grouped(other);
    let geometric_anti_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e41_ * other.e321_) + (self_.e31_ * other.e412_) + (self_.scalar * other.e423_) - (self_.e12_ * other.e431_), (self_.e42_ * other.e321_) + (self_.e12_ * other.e423_) + (self_.scalar * other.e431_) - (self_.e23_ * other.e412_), (self_.e43_ * other.e321_) + (self_.e23_ * other.e431_) + (self_.scalar * other.e412_) - (self_.e31_ * other.e423_), -(self_.e41_ * other.e423_) - (self_.e42_ * other.e431_) - (self_.e43_ * other.e412_)), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e42_ * other.e412_) - (self_.e43_ * other.e431_), (self_.e43_ * other.e423_) - (self_.e41_ * other.e412_), (self_.e41_ * other.e431_) - (self_.e42_ * other.e423_), -(self_.e23_ * other.e423_) - (self_.e31_ * other.e431_) - (self_.e12_ * other.e412_)) + (vec4<f32>(self_.e1234_) * other_groups.group0_)
    ));
    return flector_geometricAntiProduct_motor(geometric_anti_product, motor_antiReverse(self_));
}
fn motor_antiSandwich_point(self_: Motor, other: Point) -> Flector {
    let self_groups = motor_grouped(self_);
    let geometric_anti_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e42_ * other.e3_) + (self_.e1234_ * other.e1_) + (self_.e23_ * other.e4_) - (self_.e43_ * other.e2_), (self_.e43_ * other.e1_) + (self_.e1234_ * other.e2_) + (self_.e31_ * other.e4_) - (self_.e41_ * other.e3_), (self_.e41_ * other.e2_) + (self_.e1234_ * other.e3_) + (self_.e12_ * other.e4_) - (self_.e42_ * other.e1_), self_.e1234_ * other.e4_), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e41_ * other.e4_, self_.e42_ * other.e4_, self_.e43_ * other.e4_, (self_.scalar * other.e4_) - (self_.e41_ * other.e1_) - (self_.e42_ * other.e2_) - (self_.e43_ * other.e3_))
    ));
    return flector_geometricAntiProduct_motor(geometric_anti_product, motor_antiReverse(self_));
}
fn motor_antiSandwich_scalar(self_: Motor, other: Scalar) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = scalar_grouped(other);
    let geometric_anti_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(other.scalar) * self_groups.group0_
    ));
    return motor_geometricAntiProduct_motor(geometric_anti_product, motor_antiReverse(self_));
}
fn multiVector_antiSandwich_antiScalar(self_: MultiVector, other: AntiScalar) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    let geometric_anti_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ (vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(other.e1234_) * self_groups.group1_, 
        /* e41, e42, e43 */ (vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_, 
        /* e23, e31, e12 */ (vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e1234_) * self_groups.group4_
    ));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self_));
}
fn multiVector_antiSandwich_dualNum(self_: MultiVector, other: DualNum) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let geometric_anti_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((other.scalar * self_.e1234_) + (other.e1234_ * self_.scalar), other.e1234_ * self_.e1234_, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((other.e1234_ * self_.e1_) - (other.scalar * self_.e423_), (other.e1234_ * self_.e2_) - (other.scalar * self_.e431_), (other.e1234_ * self_.e3_) - (other.scalar * self_.e412_), other.e1234_ * self_.e4_), 
        /* e41, e42, e43 */ (vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_, 
        /* e23, e31, e12 */ ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_) + ((vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e1234_ * self_.e423_, other.e1234_ * self_.e431_, other.e1234_ * self_.e412_, (other.e1234_ * self_.e321_) - (other.scalar * self_.e4_))
    ));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self_));
}
fn multiVector_antiSandwich_flector(self_: MultiVector, other: Flector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = flector_grouped(other);
    let geometric_anti_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((other.e321_ * self_.e4_) - (other.e1_ * self_.e423_) - (other.e2_ * self_.e431_) - (other.e3_ * self_.e412_), 0.0, 0.0, 0.0) + ((vec4<f32>(other.e423_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e1_, self_.e423_, 0.0, 0.0)) + ((vec4<f32>(other.e431_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e2_, self_.e431_, 0.0, 0.0)) + ((vec4<f32>(other.e412_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e3_, self_.e412_, 0.0, 0.0)) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e321_, self_.e4_, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((other.e3_ * self_.e42_) + (other.e4_ * self_.e23_) + (other.e423_ * self_.scalar) + (other.e412_ * self_.e31_) + (other.e321_ * self_.e41_) - (other.e2_ * self_.e43_) - (other.e431_ * self_.e12_), (other.e1_ * self_.e43_) + (other.e4_ * self_.e31_) + (other.e423_ * self_.e12_) + (other.e431_ * self_.scalar) + (other.e321_ * self_.e42_) - (other.e3_ * self_.e41_) - (other.e412_ * self_.e23_), (other.e2_ * self_.e41_) + (other.e4_ * self_.e12_) + (other.e431_ * self_.e23_) + (other.e412_ * self_.scalar) + (other.e321_ * self_.e43_) - (other.e1_ * self_.e42_) - (other.e423_ * self_.e31_), -(other.e423_ * self_.e41_) - (other.e431_ * self_.e42_) - (other.e412_ * self_.e43_)) + (vec4<f32>(self_.e1234_) * other_groups.group0_), 
        /* e41, e42, e43 */ vec4<f32>((other.e431_ * self_.e412_) - (other.e412_ * self_.e431_), (other.e412_ * self_.e423_) - (other.e423_ * self_.e412_), (other.e423_ * self_.e431_) - (other.e431_ * self_.e423_), 0.0) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0)) - ((vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((other.e3_ * self_.e431_) + (other.e431_ * self_.e3_) - (other.e2_ * self_.e412_) - (other.e412_ * self_.e2_), (other.e1_ * self_.e412_) + (other.e412_ * self_.e1_) - (other.e3_ * self_.e423_) - (other.e423_ * self_.e3_), (other.e2_ * self_.e423_) + (other.e423_ * self_.e2_) - (other.e1_ * self_.e431_) - (other.e431_ * self_.e1_), 0.0) + ((vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0)) + ((vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0)) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0)) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e412_ * self_.e42_) - (other.e431_ * self_.e43_), (other.e423_ * self_.e43_) - (other.e412_ * self_.e41_), (other.e431_ * self_.e41_) - (other.e423_ * self_.e42_), -(other.e1_ * self_.e41_) - (other.e2_ * self_.e42_) - (other.e3_ * self_.e43_) - (other.e423_ * self_.e23_) - (other.e431_ * self_.e31_) - (other.e412_ * self_.e12_)) + (vec4<f32>(other.e4_) * vec4<f32>(self_.e41_, self_.e42_, self_.e43_, self_.scalar)) + (vec4<f32>(self_.e1234_) * other_groups.group1_)
    ));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self_));
}
fn multiVector_antiSandwich_horizon(self_: MultiVector, other: Horizon) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = horizon_grouped(other);
    let geometric_anti_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.e321_ * self_.e4_, 1.0, 0.0, 0.0) * vec2<f32>(1.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(other.e321_ * self_.e41_, other.e321_ * self_.e42_, other.e321_ * self_.e43_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ (vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0, 1.0, 1.0, other.e321_ * self_.e1234_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)
    ));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self_));
}
fn multiVector_antiSandwich_line(self_: MultiVector, other: Line) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = line_grouped(other);
    let geometric_anti_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(-(other.e23_ * self_.e41_) - (other.e31_ * self_.e42_) - (other.e12_ * self_.e43_), 0.0, 0.0, 0.0) - ((vec4<f32>(other.e41_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e23_, self_.e41_, 0.0, 0.0)) - ((vec4<f32>(other.e42_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e31_, self_.e42_, 0.0, 0.0)) - ((vec4<f32>(other.e43_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e12_, self_.e43_, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((other.e41_ * self_.e321_) + (other.e43_ * self_.e2_) + (other.e31_ * self_.e412_) - (other.e42_ * self_.e3_) - (other.e23_ * self_.e4_) - (other.e12_ * self_.e431_), (other.e41_ * self_.e3_) + (other.e42_ * self_.e321_) + (other.e12_ * self_.e423_) - (other.e43_ * self_.e1_) - (other.e23_ * self_.e412_) - (other.e31_ * self_.e4_), (other.e42_ * self_.e1_) + (other.e43_ * self_.e321_) + (other.e23_ * self_.e431_) - (other.e41_ * self_.e2_) - (other.e31_ * self_.e423_) - (other.e12_ * self_.e4_), -(other.e41_ * self_.e423_) - (other.e42_ * self_.e431_) - (other.e43_ * self_.e412_)), 
        /* e41, e42, e43 */ vec4<f32>((other.e43_ * self_.e42_) - (other.e42_ * self_.e43_), (other.e41_ * self_.e43_) - (other.e43_ * self_.e41_), (other.e42_ * self_.e41_) - (other.e41_ * self_.e42_), 0.0) + ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group0_), 
        /* e23, e31, e12 */ vec4<f32>((other.e43_ * self_.e31_) + (other.e12_ * self_.e42_) - (other.e42_ * self_.e12_) - (other.e31_ * self_.e43_), (other.e41_ * self_.e12_) + (other.e23_ * self_.e43_) - (other.e43_ * self_.e23_) - (other.e12_ * self_.e41_), (other.e42_ * self_.e23_) + (other.e31_ * self_.e41_) - (other.e41_ * self_.e31_) - (other.e23_ * self_.e42_), 0.0) + ((vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group0_) + ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group1_), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e41_ * self_.e4_) + (other.e43_ * self_.e431_) - (other.e42_ * self_.e412_), (other.e41_ * self_.e412_) + (other.e42_ * self_.e4_) - (other.e43_ * self_.e423_), (other.e42_ * self_.e423_) + (other.e43_ * self_.e4_) - (other.e41_ * self_.e431_), (other.e23_ * self_.e423_) + (other.e31_ * self_.e431_) + (other.e12_ * self_.e412_) - (other.e41_ * self_.e1_) - (other.e42_ * self_.e2_) - (other.e43_ * self_.e3_))
    ));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self_));
}
fn multiVector_antiSandwich_motor(self_: MultiVector, other: Motor) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = motor_grouped(other);
    let geometric_anti_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((other.scalar * self_.e1234_) - (other.e23_ * self_.e41_) - (other.e31_ * self_.e42_) - (other.e12_ * self_.e43_), 0.0, 0.0, 0.0) + ((vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * self_groups.group0_) - ((vec4<f32>(other.e41_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e23_, self_.e41_, 0.0, 0.0)) - ((vec4<f32>(other.e42_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e31_, self_.e42_, 0.0, 0.0)) - ((vec4<f32>(other.e43_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e12_, self_.e43_, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((other.e41_ * self_.e321_) + (other.e43_ * self_.e2_) + (other.e31_ * self_.e412_) - (other.e42_ * self_.e3_) - (other.e23_ * self_.e4_) - (other.e12_ * self_.e431_) - (other.scalar * self_.e423_), (other.e41_ * self_.e3_) + (other.e42_ * self_.e321_) + (other.e12_ * self_.e423_) - (other.e43_ * self_.e1_) - (other.e23_ * self_.e412_) - (other.e31_ * self_.e4_) - (other.scalar * self_.e431_), (other.e42_ * self_.e1_) + (other.e43_ * self_.e321_) + (other.e23_ * self_.e431_) - (other.e41_ * self_.e2_) - (other.e31_ * self_.e423_) - (other.e12_ * self_.e4_) - (other.scalar * self_.e412_), -(other.e41_ * self_.e423_) - (other.e42_ * self_.e431_) - (other.e43_ * self_.e412_)) + (vec4<f32>(other.e1234_) * self_groups.group1_), 
        /* e41, e42, e43 */ vec4<f32>((other.e43_ * self_.e42_) - (other.e42_ * self_.e43_), (other.e41_ * self_.e43_) - (other.e43_ * self_.e41_), (other.e42_ * self_.e41_) - (other.e41_ * self_.e42_), 0.0) + ((vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_) + ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e41_, other.e42_, other.e43_, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((other.e43_ * self_.e31_) + (other.e12_ * self_.e42_) - (other.e42_ * self_.e12_) - (other.e31_ * self_.e43_), (other.e41_ * self_.e12_) + (other.e23_ * self_.e43_) - (other.e43_ * self_.e23_) - (other.e12_ * self_.e41_), (other.e42_ * self_.e23_) + (other.e31_ * self_.e41_) - (other.e41_ * self_.e31_) - (other.e23_ * self_.e42_), 0.0) + ((vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_) + ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_) + ((vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e41_, other.e42_, other.e43_, 0.0)) + ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e23_, other.e31_, other.e12_, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e41_ * self_.e4_) + (other.e43_ * self_.e431_) - (other.e42_ * self_.e412_), (other.e41_ * self_.e412_) + (other.e42_ * self_.e4_) - (other.e43_ * self_.e423_), (other.e42_ * self_.e423_) + (other.e43_ * self_.e4_) - (other.e41_ * self_.e431_), (other.e23_ * self_.e423_) + (other.e31_ * self_.e431_) + (other.e12_ * self_.e412_) - (other.e41_ * self_.e1_) - (other.e42_ * self_.e2_) - (other.e43_ * self_.e3_) - (other.scalar * self_.e4_)) + (vec4<f32>(other.e1234_) * self_groups.group4_)
    ));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self_));
}
fn multiVector_antiSandwich_multiVector(self_: MultiVector, other: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let geometric_anti_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((other.e1234_ * self_.scalar) + (other.e321_ * self_.e4_) - (other.e1_ * self_.e423_) - (other.e2_ * self_.e431_) - (other.e3_ * self_.e412_) - (other.e23_ * self_.e41_) - (other.e31_ * self_.e42_) - (other.e12_ * self_.e43_), 0.0, 0.0, 0.0) + ((vec4<f32>(other.e423_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e1_, self_.e423_, 0.0, 0.0)) + ((vec4<f32>(other.e431_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e2_, self_.e431_, 0.0, 0.0)) + ((vec4<f32>(other.e412_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e3_, self_.e412_, 0.0, 0.0)) + ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * other_groups.group0_) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e321_, self_.e4_, 0.0, 0.0)) - ((vec4<f32>(other.e41_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e23_, self_.e41_, 0.0, 0.0)) - ((vec4<f32>(other.e42_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e31_, self_.e42_, 0.0, 0.0)) - ((vec4<f32>(other.e43_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e12_, self_.e43_, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((other.e3_ * self_.e42_) + (other.e4_ * self_.e23_) + (other.e41_ * self_.e321_) + (other.e43_ * self_.e2_) + (other.e31_ * self_.e412_) + (other.e423_ * self_.scalar) + (other.e412_ * self_.e31_) + (other.e321_ * self_.e41_) - (other.scalar * self_.e423_) - (other.e2_ * self_.e43_) - (other.e42_ * self_.e3_) - (other.e23_ * self_.e4_) - (other.e12_ * self_.e431_) - (other.e431_ * self_.e12_), (other.e1_ * self_.e43_) + (other.e4_ * self_.e31_) + (other.e41_ * self_.e3_) + (other.e42_ * self_.e321_) + (other.e12_ * self_.e423_) + (other.e423_ * self_.e12_) + (other.e431_ * self_.scalar) + (other.e321_ * self_.e42_) - (other.scalar * self_.e431_) - (other.e3_ * self_.e41_) - (other.e43_ * self_.e1_) - (other.e23_ * self_.e412_) - (other.e31_ * self_.e4_) - (other.e412_ * self_.e23_), (other.e2_ * self_.e41_) + (other.e4_ * self_.e12_) + (other.e42_ * self_.e1_) + (other.e43_ * self_.e321_) + (other.e23_ * self_.e431_) + (other.e431_ * self_.e23_) + (other.e412_ * self_.scalar) + (other.e321_ * self_.e43_) - (other.scalar * self_.e412_) - (other.e1_ * self_.e42_) - (other.e41_ * self_.e2_) - (other.e31_ * self_.e423_) - (other.e12_ * self_.e4_) - (other.e423_ * self_.e31_), -(other.e41_ * self_.e423_) - (other.e42_ * self_.e431_) - (other.e43_ * self_.e412_) - (other.e423_ * self_.e41_) - (other.e431_ * self_.e42_) - (other.e412_ * self_.e43_)) + (vec4<f32>(other.e1234_) * self_groups.group1_) + (vec4<f32>(self_.e1234_) * other_groups.group1_), 
        /* e41, e42, e43 */ vec4<f32>((other.e43_ * self_.e42_) + (other.e431_ * self_.e412_) - (other.e42_ * self_.e43_) - (other.e412_ * self_.e431_), (other.e41_ * self_.e43_) + (other.e412_ * self_.e423_) - (other.e43_ * self_.e41_) - (other.e423_ * self_.e412_), (other.e42_ * self_.e41_) + (other.e423_ * self_.e431_) - (other.e41_ * self_.e42_) - (other.e431_ * self_.e423_), 0.0) + ((vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_) + ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group2_) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0)) - ((vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((other.e3_ * self_.e431_) + (other.e43_ * self_.e31_) + (other.e12_ * self_.e42_) + (other.e431_ * self_.e3_) - (other.e2_ * self_.e412_) - (other.e42_ * self_.e12_) - (other.e31_ * self_.e43_) - (other.e412_ * self_.e2_), (other.e1_ * self_.e412_) + (other.e41_ * self_.e12_) + (other.e23_ * self_.e43_) + (other.e412_ * self_.e1_) - (other.e3_ * self_.e423_) - (other.e43_ * self_.e23_) - (other.e12_ * self_.e41_) - (other.e423_ * self_.e3_), (other.e2_ * self_.e423_) + (other.e42_ * self_.e23_) + (other.e31_ * self_.e41_) + (other.e423_ * self_.e2_) - (other.e1_ * self_.e431_) - (other.e41_ * self_.e31_) - (other.e23_ * self_.e42_) - (other.e431_ * self_.e1_), 0.0) + ((vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_) + ((vec4<f32>(other.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group3_) + ((vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0)) + ((vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group2_) + ((vec4<f32>(self_.e1234_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group3_) + ((vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0)) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0)) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e41_ * self_.e4_) + (other.e43_ * self_.e431_) + (other.e412_ * self_.e42_) - (other.e42_ * self_.e412_) - (other.e431_ * self_.e43_), (other.e41_ * self_.e412_) + (other.e42_ * self_.e4_) + (other.e423_ * self_.e43_) - (other.e43_ * self_.e423_) - (other.e412_ * self_.e41_), (other.e42_ * self_.e423_) + (other.e43_ * self_.e4_) + (other.e431_ * self_.e41_) - (other.e41_ * self_.e431_) - (other.e423_ * self_.e42_), (other.e23_ * self_.e423_) + (other.e31_ * self_.e431_) + (other.e12_ * self_.e412_) - (other.scalar * self_.e4_) - (other.e1_ * self_.e41_) - (other.e2_ * self_.e42_) - (other.e3_ * self_.e43_) - (other.e41_ * self_.e1_) - (other.e42_ * self_.e2_) - (other.e43_ * self_.e3_) - (other.e423_ * self_.e23_) - (other.e431_ * self_.e31_) - (other.e412_ * self_.e12_)) + (vec4<f32>(other.e1234_) * self_groups.group4_) + (vec4<f32>(other.e4_) * vec4<f32>(self_.e41_, self_.e42_, self_.e43_, self_.scalar)) + (vec4<f32>(self_.e1234_) * other_groups.group4_)
    ));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self_));
}
fn multiVector_antiSandwich_origin(self_: MultiVector, other: Origin) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = origin_grouped(other);
    let geometric_anti_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ (vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e321_, self_.e4_, 0.0, 0.0) * vec2<f32>(-1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(other.e4_) * vec4<f32>(self_.e23_, self_.e31_, self_.e12_, self_.e1234_), 
        /* e41, e42, e43 */ (vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ (vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0) * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e4_) * vec4<f32>(self_.e41_, self_.e42_, self_.e43_, self_.scalar)
    ));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self_));
}
fn multiVector_antiSandwich_plane(self_: MultiVector, other: Plane) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = plane_grouped(other);
    let geometric_anti_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.e4_ * other.e321_, 0.0, 0.0, 0.0) + ((vec4<f32>(other.e423_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e1_, self_.e423_, 0.0, 0.0)) + ((vec4<f32>(other.e431_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e2_, self_.e431_, 0.0, 0.0)) + ((vec4<f32>(other.e412_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(self_.e3_, self_.e412_, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.scalar * other.e423_) + (self_.e41_ * other.e321_) + (self_.e31_ * other.e412_) - (self_.e12_ * other.e431_), (self_.scalar * other.e431_) + (self_.e42_ * other.e321_) + (self_.e12_ * other.e423_) - (self_.e23_ * other.e412_), (self_.scalar * other.e412_) + (self_.e43_ * other.e321_) + (self_.e23_ * other.e431_) - (self_.e31_ * other.e423_), -(self_.e41_ * other.e423_) - (self_.e42_ * other.e431_) - (self_.e43_ * other.e412_)), 
        /* e41, e42, e43 */ vec4<f32>((self_.e412_ * other.e431_) - (self_.e431_ * other.e412_), (self_.e423_ * other.e412_) - (self_.e412_ * other.e423_), (self_.e431_ * other.e423_) - (self_.e423_ * other.e431_), 0.0) - ((vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((self_.e3_ * other.e431_) - (self_.e2_ * other.e412_), (self_.e1_ * other.e412_) - (self_.e3_ * other.e423_), (self_.e2_ * other.e423_) - (self_.e1_ * other.e431_), 0.0) + ((vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0)) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.e42_ * other.e412_) - (self_.e43_ * other.e431_), (self_.e43_ * other.e423_) - (self_.e41_ * other.e412_), (self_.e41_ * other.e431_) - (self_.e42_ * other.e423_), -(self_.e23_ * other.e423_) - (self_.e31_ * other.e431_) - (self_.e12_ * other.e412_)) + (vec4<f32>(self_.e1234_) * other_groups.group0_)
    ));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self_));
}
fn multiVector_antiSandwich_point(self_: MultiVector, other: Point) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = point_grouped(other);
    let geometric_anti_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(-(self_.e423_ * other.e1_) - (self_.e431_ * other.e2_) - (self_.e412_ * other.e3_) - (self_.e321_ * other.e4_), self_.e4_ * other.e4_, 0.0, 0.0) * vec2<f32>(1.0, -1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.e1234_ * other.e1_) + (self_.e42_ * other.e3_) + (self_.e23_ * other.e4_) - (self_.e43_ * other.e2_), (self_.e1234_ * other.e2_) + (self_.e43_ * other.e1_) + (self_.e31_ * other.e4_) - (self_.e41_ * other.e3_), (self_.e1234_ * other.e3_) + (self_.e41_ * other.e2_) + (self_.e12_ * other.e4_) - (self_.e42_ * other.e1_), self_.e1234_ * other.e4_), 
        /* e41, e42, e43 */ (vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>((self_.e431_ * other.e3_) - (self_.e412_ * other.e2_), (self_.e412_ * other.e1_) - (self_.e423_ * other.e3_), (self_.e423_ * other.e2_) - (self_.e431_ * other.e1_), 0.0) + ((vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0)) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e41_ * other.e4_, self_.e42_ * other.e4_, self_.e43_ * other.e4_, (self_.scalar * other.e4_) - (self_.e41_ * other.e1_) - (self_.e42_ * other.e2_) - (self_.e43_ * other.e3_))
    ));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self_));
}
fn multiVector_antiSandwich_scalar(self_: MultiVector, other: Scalar) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = scalar_grouped(other);
    let geometric_anti_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.e1234_ * other.scalar, 1.0, 0.0, 0.0) * vec2<f32>(1.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e423_ * other.scalar, self_.e431_ * other.scalar, self_.e412_ * other.scalar, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ (vec4<f32>(other.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * self_groups.group2_, 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0, 1.0, 1.0, self_.e4_ * other.scalar) * vec4<f32>(0.0, 0.0, 0.0, -1.0)
    ));
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self_));
}
fn origin_antiSandwich_antiScalar(self_: Origin, other: AntiScalar) -> AntiScalar {
    let geometric_anti_product: Origin = Origin(other.e1234_ * self_.e4_);
    return origin_geometricAntiProduct_origin(geometric_anti_product, origin_antiReverse(self_));
}
fn origin_antiSandwich_dualNum(self_: Origin, other: DualNum) -> Motor {
    let geometric_anti_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(1.0, 1.0, 1.0, other.e1234_ * self_.e4_) * vec4<f32>(0.0, 0.0, 0.0, 1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0, 1.0, 1.0, other.scalar * self_.e4_) * vec4<f32>(0.0, 0.0, 0.0, -1.0)
    ));
    return flector_geometricAntiProduct_origin(geometric_anti_product, origin_antiReverse(self_));
}
fn origin_antiSandwich_flector(self_: Origin, other: Flector) -> Flector {
    let geometric_anti_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.e4_) * vec4<f32>(other.e423_, other.e431_, other.e412_, other.e4_) * vec4<f32>(-1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.e4_) * vec4<f32>(other.e1_, other.e2_, other.e3_, other.e321_)
    ));
    return motor_geometricAntiProduct_origin(geometric_anti_product, origin_antiReverse(self_));
}
fn origin_antiSandwich_horizon(self_: Origin, other: Horizon) -> Horizon {
    let geometric_anti_product: Scalar = Scalar(other.e321_ * self_.e4_);
    return scalar_geometricAntiProduct_origin(geometric_anti_product, origin_antiReverse(self_));
}
fn origin_antiSandwich_line(self_: Origin, other: Line) -> Motor {
    let geometric_anti_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e23_ * self_.e4_, other.e31_ * self_.e4_, other.e12_ * self_.e4_, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e41_ * self_.e4_, other.e42_ * self_.e4_, other.e43_ * self_.e4_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
    return flector_geometricAntiProduct_origin(geometric_anti_product, origin_antiReverse(self_));
}
fn origin_antiSandwich_motor(self_: Origin, other: Motor) -> Motor {
    let geometric_anti_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e4_) * vec4<f32>(other.e23_, other.e31_, other.e12_, other.e1234_) * vec4<f32>(-1.0, -1.0, -1.0, 1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e4_) * vec4<f32>(other.e41_, other.e42_, other.e43_, other.scalar) * vec4<f32>(1.0, 1.0, 1.0, -1.0)
    ));
    return flector_geometricAntiProduct_origin(geometric_anti_product, origin_antiReverse(self_));
}
fn origin_antiSandwich_multiVector(self_: Origin, other: MultiVector) -> MultiVector {
    let geometric_anti_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ (vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) * vec4<f32>(other.e321_, other.e4_, 0.0, 0.0) * vec2<f32>(1.0, -1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e4_) * vec4<f32>(other.e23_, other.e31_, other.e12_, other.e1234_) * vec4<f32>(-1.0, -1.0, -1.0, 1.0), 
        /* e41, e42, e43 */ (vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ (vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.e4_) * vec4<f32>(other.e41_, other.e42_, other.e43_, other.scalar) * vec4<f32>(1.0, 1.0, 1.0, -1.0)
    ));
    return multiVector_geometricAntiProduct_origin(geometric_anti_product, origin_antiReverse(self_));
}
fn origin_antiSandwich_origin(self_: Origin, other: Origin) -> Origin {
    let geometric_anti_product: AntiScalar = AntiScalar(other.e4_ * self_.e4_ * -1.0);
    return antiScalar_geometricAntiProduct_origin(geometric_anti_product, origin_antiReverse(self_));
}
fn origin_antiSandwich_plane(self_: Origin, other: Plane) -> Flector {
    let geometric_anti_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.e4_ * other.e423_, self_.e4_ * other.e431_, self_.e4_ * other.e412_, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(1.0, 1.0, 1.0, self_.e4_ * other.e321_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)
    ));
    return motor_geometricAntiProduct_origin(geometric_anti_product, origin_antiReverse(self_));
}
fn origin_antiSandwich_point(self_: Origin, other: Point) -> Flector {
    let geometric_anti_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(1.0, 1.0, 1.0, self_.e4_ * other.e4_) * vec4<f32>(0.0, 0.0, 0.0, -1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.e4_ * other.e1_, self_.e4_ * other.e2_, self_.e4_ * other.e3_, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
    return motor_geometricAntiProduct_origin(geometric_anti_product, origin_antiReverse(self_));
}
fn origin_antiSandwich_scalar(self_: Origin, other: Scalar) -> Scalar {
    let geometric_anti_product: Horizon = Horizon(self_.e4_ * other.scalar * -1.0);
    return horizon_geometricAntiProduct_origin(geometric_anti_product, origin_antiReverse(self_));
}
fn plane_antiSandwich_antiScalar(self_: Plane, other: AntiScalar) -> Motor {
    let self_groups = plane_grouped(self_);
    let geometric_anti_product: Plane = plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(other.e1234_) * self_groups.group0_
    ));
    return plane_geometricAntiProduct_plane(geometric_anti_product, plane_antiReverse(self_));
}
fn plane_antiSandwich_dualNum(self_: Plane, other: DualNum) -> Motor {
    let self_groups = plane_grouped(self_);
    let geometric_anti_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.scalar * self_.e423_, other.scalar * self_.e431_, other.scalar * self_.e412_, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e1234_) * self_groups.group0_
    ));
    return flector_geometricAntiProduct_plane(geometric_anti_product, plane_antiReverse(self_));
}
fn plane_antiSandwich_flector(self_: Plane, other: Flector) -> Flector {
    let other_groups = flector_grouped(other);
    let geometric_anti_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other.e431_ * self_.e412_) - (other.e4_ * self_.e423_) - (other.e412_ * self_.e431_), (other.e412_ * self_.e423_) - (other.e4_ * self_.e431_) - (other.e423_ * self_.e412_), (other.e423_ * self_.e431_) - (other.e4_ * self_.e412_) - (other.e431_ * self_.e423_), (other.e423_ * self_.e423_) + (other.e431_ * self_.e431_) + (other.e412_ * self_.e412_)), 
        /* e23, e31, e12, scalar */ vec4<f32>((other.e3_ * self_.e431_) + (other.e321_ * self_.e423_) - (other.e2_ * self_.e412_), (other.e1_ * self_.e412_) + (other.e321_ * self_.e431_) - (other.e3_ * self_.e423_), (other.e2_ * self_.e423_) + (other.e321_ * self_.e412_) - (other.e1_ * self_.e431_), -(other.e1_ * self_.e423_) - (other.e2_ * self_.e431_) - (other.e3_ * self_.e412_)) - (vec4<f32>(self_.e321_) * vec4<f32>(other.e423_, other.e431_, other.e412_, other.e4_))
    ));
    return motor_geometricAntiProduct_plane(geometric_anti_product, plane_antiReverse(self_));
}
fn plane_antiSandwich_horizon(self_: Plane, other: Horizon) -> Flector {
    let geometric_anti_product: Line = line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ (vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0)
    ));
    return line_geometricAntiProduct_plane(geometric_anti_product, plane_antiReverse(self_));
}
fn plane_antiSandwich_line(self_: Plane, other: Line) -> Motor {
    let geometric_anti_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other.e41_ * self_.e321_) + (other.e31_ * self_.e412_) - (other.e12_ * self_.e431_), (other.e42_ * self_.e321_) + (other.e12_ * self_.e423_) - (other.e23_ * self_.e412_), (other.e43_ * self_.e321_) + (other.e23_ * self_.e431_) - (other.e31_ * self_.e423_), -(other.e41_ * self_.e423_) - (other.e42_ * self_.e431_) - (other.e43_ * self_.e412_)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e43_ * self_.e431_) - (other.e42_ * self_.e412_), (other.e41_ * self_.e412_) - (other.e43_ * self_.e423_), (other.e42_ * self_.e423_) - (other.e41_ * self_.e431_), (other.e23_ * self_.e423_) + (other.e31_ * self_.e431_) + (other.e12_ * self_.e412_))
    ));
    return flector_geometricAntiProduct_plane(geometric_anti_product, plane_antiReverse(self_));
}
fn plane_antiSandwich_motor(self_: Plane, other: Motor) -> Motor {
    let self_groups = plane_grouped(self_);
    let geometric_anti_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other.e41_ * self_.e321_) + (other.e31_ * self_.e412_) - (other.e12_ * self_.e431_) - (other.scalar * self_.e423_), (other.e42_ * self_.e321_) + (other.e12_ * self_.e423_) - (other.e23_ * self_.e412_) - (other.scalar * self_.e431_), (other.e43_ * self_.e321_) + (other.e23_ * self_.e431_) - (other.e31_ * self_.e423_) - (other.scalar * self_.e412_), -(other.e41_ * self_.e423_) - (other.e42_ * self_.e431_) - (other.e43_ * self_.e412_)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e43_ * self_.e431_) - (other.e42_ * self_.e412_), (other.e41_ * self_.e412_) - (other.e43_ * self_.e423_), (other.e42_ * self_.e423_) - (other.e41_ * self_.e431_), (other.e23_ * self_.e423_) + (other.e31_ * self_.e431_) + (other.e12_ * self_.e412_)) + (vec4<f32>(other.e1234_) * self_groups.group0_)
    ));
    return flector_geometricAntiProduct_plane(geometric_anti_product, plane_antiReverse(self_));
}
fn plane_antiSandwich_multiVector(self_: Plane, other: MultiVector) -> MultiVector {
    let self_groups = plane_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let geometric_anti_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(-(other.e1_ * self_.e423_) - (other.e2_ * self_.e431_) - (other.e3_ * self_.e412_) - (other.e4_ * self_.e321_), (other.e423_ * self_.e423_) + (other.e431_ * self_.e431_) + (other.e412_ * self_.e412_), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((other.e41_ * self_.e321_) + (other.e31_ * self_.e412_) - (other.scalar * self_.e423_) - (other.e12_ * self_.e431_), (other.e42_ * self_.e321_) + (other.e12_ * self_.e423_) - (other.scalar * self_.e431_) - (other.e23_ * self_.e412_), (other.e43_ * self_.e321_) + (other.e23_ * self_.e431_) - (other.scalar * self_.e412_) - (other.e31_ * self_.e423_), -(other.e41_ * self_.e423_) - (other.e42_ * self_.e431_) - (other.e43_ * self_.e412_)), 
        /* e41, e42, e43 */ vec4<f32>((other.e431_ * self_.e412_) - (other.e412_ * self_.e431_), (other.e412_ * self_.e423_) - (other.e423_ * self_.e412_), (other.e423_ * self_.e431_) - (other.e431_ * self_.e423_), 0.0) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((other.e3_ * self_.e431_) - (other.e2_ * self_.e412_), (other.e1_ * self_.e412_) - (other.e3_ * self_.e423_), (other.e2_ * self_.e423_) - (other.e1_ * self_.e431_), 0.0) + ((vec4<f32>(other.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e423_, self_.e431_, self_.e412_, 0.0)) - ((vec4<f32>(self_.e321_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other.e43_ * self_.e431_) - (other.e42_ * self_.e412_), (other.e41_ * self_.e412_) - (other.e43_ * self_.e423_), (other.e42_ * self_.e423_) - (other.e41_ * self_.e431_), (other.e23_ * self_.e423_) + (other.e31_ * self_.e431_) + (other.e12_ * self_.e412_)) + (vec4<f32>(other.e1234_) * self_groups.group0_)
    ));
    return multiVector_geometricAntiProduct_plane(geometric_anti_product, plane_antiReverse(self_));
}
fn plane_antiSandwich_origin(self_: Plane, other: Origin) -> Flector {
    let other_groups = origin_grouped(other);
    let geometric_anti_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e4_ * self_.e423_, other.e4_ * self_.e431_, other.e4_ * self_.e412_, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(1.0, 1.0, 1.0, other.e4_ * self_.e321_) * vec4<f32>(0.0, 0.0, 0.0, -1.0)
    ));
    return motor_geometricAntiProduct_plane(geometric_anti_product, plane_antiReverse(self_));
}
fn plane_antiSandwich_plane(self_: Plane, other: Plane) -> Flector {
    let other_groups = plane_grouped(other);
    let geometric_anti_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other.e431_ * self_.e412_) - (other.e412_ * self_.e431_), (other.e412_ * self_.e423_) - (other.e423_ * self_.e412_), (other.e423_ * self_.e431_) - (other.e431_ * self_.e423_), (other.e423_ * self_.e423_) + (other.e431_ * self_.e431_) + (other.e412_ * self_.e412_)), 
        /* e23, e31, e12, scalar */ vec4<f32>((other.e321_ * self_.e423_) - (other.e423_ * self_.e321_), (other.e321_ * self_.e431_) - (other.e431_ * self_.e321_), (other.e321_ * self_.e412_) - (other.e412_ * self_.e321_), 0.0)
    ));
    return motor_geometricAntiProduct_plane(geometric_anti_product, plane_antiReverse(self_));
}
fn plane_antiSandwich_point(self_: Plane, other: Point) -> Flector {
    let other_groups = point_grouped(other);
    let geometric_anti_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.e423_ * other.e4_, self_.e431_ * other.e4_, self_.e412_ * other.e4_, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>((self_.e431_ * other.e3_) - (self_.e412_ * other.e2_), (self_.e412_ * other.e1_) - (self_.e423_ * other.e3_), (self_.e423_ * other.e2_) - (self_.e431_ * other.e1_), -(self_.e423_ * other.e1_) - (self_.e431_ * other.e2_) - (self_.e412_ * other.e3_) - (self_.e321_ * other.e4_))
    ));
    return motor_geometricAntiProduct_plane(geometric_anti_product, plane_antiReverse(self_));
}
fn plane_antiSandwich_scalar(self_: Plane, other: Scalar) -> Motor {
    let geometric_anti_product: Point = point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.e423_ * other.scalar, self_.e431_ * other.scalar, self_.e412_ * other.scalar, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0)
    ));
    return point_geometricAntiProduct_plane(geometric_anti_product, plane_antiReverse(self_));
}
fn point_antiSandwich_antiScalar(self_: Point, other: AntiScalar) -> Motor {
    let self_groups = point_grouped(self_);
    let geometric_anti_product: Point = point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e1234_) * self_groups.group0_
    ));
    return point_geometricAntiProduct_point(geometric_anti_product, point_antiReverse(self_));
}
fn point_antiSandwich_dualNum(self_: Point, other: DualNum) -> Motor {
    let self_groups = point_grouped(self_);
    let geometric_anti_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e1234_) * self_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0, 1.0, 1.0, other.scalar * self_.e4_) * vec4<f32>(0.0, 0.0, 0.0, -1.0)
    ));
    return flector_geometricAntiProduct_point(geometric_anti_product, point_antiReverse(self_));
}
fn point_antiSandwich_flector(self_: Point, other: Flector) -> Flector {
    let self_groups = point_grouped(self_);
    let geometric_anti_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.e4_) * vec4<f32>(other.e423_, other.e431_, other.e412_, other.e4_) * vec4<f32>(-1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>((other.e431_ * self_.e3_) - (other.e4_ * self_.e1_) - (other.e412_ * self_.e2_), (other.e412_ * self_.e1_) - (other.e4_ * self_.e2_) - (other.e423_ * self_.e3_), (other.e423_ * self_.e2_) - (other.e4_ * self_.e3_) - (other.e431_ * self_.e1_), (other.e423_ * self_.e1_) + (other.e431_ * self_.e2_) + (other.e412_ * self_.e3_)) + (vec4<f32>(self_.e4_) * vec4<f32>(other.e1_, other.e2_, other.e3_, other.e321_))
    ));
    return motor_geometricAntiProduct_point(geometric_anti_product, point_antiReverse(self_));
}
fn point_antiSandwich_horizon(self_: Point, other: Horizon) -> Horizon {
    let self_groups = point_grouped(self_);
    let geometric_anti_product: Scalar = Scalar(other.e321_ * self_.e4_);
    return scalar_geometricAntiProduct_point(geometric_anti_product, point_antiReverse(self_));
}
fn point_antiSandwich_line(self_: Point, other: Line) -> Motor {
    let self_groups = point_grouped(self_);
    let geometric_anti_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other.e43_ * self_.e2_) - (other.e42_ * self_.e3_) - (other.e23_ * self_.e4_), (other.e41_ * self_.e3_) - (other.e43_ * self_.e1_) - (other.e31_ * self_.e4_), (other.e42_ * self_.e1_) - (other.e41_ * self_.e2_) - (other.e12_ * self_.e4_), 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e41_ * self_.e4_, other.e42_ * self_.e4_, other.e43_ * self_.e4_, -(other.e41_ * self_.e1_) - (other.e42_ * self_.e2_) - (other.e43_ * self_.e3_))
    ));
    return flector_geometricAntiProduct_point(geometric_anti_product, point_antiReverse(self_));
}
fn point_antiSandwich_motor(self_: Point, other: Motor) -> Motor {
    let self_groups = point_grouped(self_);
    let geometric_anti_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other.e43_ * self_.e2_) + (other.e1234_ * self_.e1_) - (other.e42_ * self_.e3_) - (other.e23_ * self_.e4_), (other.e41_ * self_.e3_) + (other.e1234_ * self_.e2_) - (other.e43_ * self_.e1_) - (other.e31_ * self_.e4_), (other.e42_ * self_.e1_) + (other.e1234_ * self_.e3_) - (other.e41_ * self_.e2_) - (other.e12_ * self_.e4_), other.e1234_ * self_.e4_), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e41_ * self_.e4_, other.e42_ * self_.e4_, other.e43_ * self_.e4_, -(other.e41_ * self_.e1_) - (other.e42_ * self_.e2_) - (other.e43_ * self_.e3_) - (other.scalar * self_.e4_))
    ));
    return flector_geometricAntiProduct_point(geometric_anti_product, point_antiReverse(self_));
}
fn point_antiSandwich_multiVector(self_: Point, other: MultiVector) -> MultiVector {
    let self_groups = point_grouped(self_);
    let geometric_anti_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((other.e423_ * self_.e1_) + (other.e431_ * self_.e2_) + (other.e412_ * self_.e3_) + (other.e321_ * self_.e4_), other.e4_ * self_.e4_, 0.0, 0.0) * vec2<f32>(1.0, -1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((other.e1234_ * self_.e1_) + (other.e43_ * self_.e2_) - (other.e42_ * self_.e3_) - (other.e23_ * self_.e4_), (other.e1234_ * self_.e2_) + (other.e41_ * self_.e3_) - (other.e43_ * self_.e1_) - (other.e31_ * self_.e4_), (other.e1234_ * self_.e3_) + (other.e42_ * self_.e1_) - (other.e41_ * self_.e2_) - (other.e12_ * self_.e4_), other.e1234_ * self_.e4_), 
        /* e41, e42, e43 */ (vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e423_, other.e431_, other.e412_, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>((other.e431_ * self_.e3_) - (other.e412_ * self_.e2_), (other.e412_ * self_.e1_) - (other.e423_ * self_.e3_), (other.e423_ * self_.e2_) - (other.e431_ * self_.e1_), 0.0) + ((vec4<f32>(self_.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(other.e1_, other.e2_, other.e3_, 0.0)) - ((vec4<f32>(other.e4_) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * vec4<f32>(self_.e1_, self_.e2_, self_.e3_, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e41_ * self_.e4_, other.e42_ * self_.e4_, other.e43_ * self_.e4_, -(other.scalar * self_.e4_) - (other.e41_ * self_.e1_) - (other.e42_ * self_.e2_) - (other.e43_ * self_.e3_))
    ));
    return multiVector_geometricAntiProduct_point(geometric_anti_product, point_antiReverse(self_));
}
fn point_antiSandwich_origin(self_: Point, other: Origin) -> Flector {
    let self_groups = point_grouped(self_);
    let geometric_anti_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(1.0, 1.0, 1.0, other.e4_ * self_.e4_) * vec4<f32>(0.0, 0.0, 0.0, -1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(other.e4_ * self_.e1_, other.e4_ * self_.e2_, other.e4_ * self_.e3_, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0)
    ));
    return motor_geometricAntiProduct_point(geometric_anti_product, point_antiReverse(self_));
}
fn point_antiSandwich_plane(self_: Point, other: Plane) -> Flector {
    let self_groups = point_grouped(self_);
    let geometric_anti_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e423_ * self_.e4_, other.e431_ * self_.e4_, other.e412_ * self_.e4_, 1.0) * vec4<f32>(-1.0, -1.0, -1.0, 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>((other.e431_ * self_.e3_) - (other.e412_ * self_.e2_), (other.e412_ * self_.e1_) - (other.e423_ * self_.e3_), (other.e423_ * self_.e2_) - (other.e431_ * self_.e1_), (other.e423_ * self_.e1_) + (other.e431_ * self_.e2_) + (other.e412_ * self_.e3_) + (other.e321_ * self_.e4_))
    ));
    return motor_geometricAntiProduct_point(geometric_anti_product, point_antiReverse(self_));
}
fn point_antiSandwich_point(self_: Point, other: Point) -> Flector {
    let self_groups = point_grouped(self_);
    let geometric_anti_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(1.0, 1.0, 1.0, other.e4_ * self_.e4_) * vec4<f32>(0.0, 0.0, 0.0, -1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>((other.e1_ * self_.e4_) - (other.e4_ * self_.e1_), (other.e2_ * self_.e4_) - (other.e4_ * self_.e2_), (other.e3_ * self_.e4_) - (other.e4_ * self_.e3_), 0.0)
    ));
    return motor_geometricAntiProduct_point(geometric_anti_product, point_antiReverse(self_));
}
fn point_antiSandwich_scalar(self_: Point, other: Scalar) -> Scalar {
    let self_groups = point_grouped(self_);
    let geometric_anti_product: Horizon = Horizon(self_.e4_ * other.scalar * -1.0);
    return horizon_geometricAntiProduct_point(geometric_anti_product, point_antiReverse(self_));
}
fn scalar_antiSandwich_flector(self_: Scalar, other: Flector) -> Flector {
    let geometric_anti_product: Flector = flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e423_ * self_.scalar, other.e431_ * self_.scalar, other.e412_ * self_.scalar, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0, 1.0, 1.0, other.e4_ * self_.scalar) * vec4<f32>(0.0, 0.0, 0.0, 1.0)
    ));
    return flector_geometricAntiProduct_scalar(geometric_anti_product, scalar_antiReverse(self_));
}
fn scalar_antiSandwich_line(self_: Scalar, other: Line) -> Line {
    let self_groups = scalar_grouped(self_);
    let other_groups = line_grouped(other);
    let geometric_anti_product: Line = line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ (vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group0_
    ));
    return line_geometricAntiProduct_scalar(geometric_anti_product, scalar_antiReverse(self_));
}
fn scalar_antiSandwich_motor(self_: Scalar, other: Motor) -> Motor {
    let self_groups = scalar_grouped(self_);
    let other_groups = motor_grouped(other);
    let geometric_anti_product: Motor = motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.scalar) * other_groups.group0_
    ));
    return motor_geometricAntiProduct_scalar(geometric_anti_product, scalar_antiReverse(self_));
}
fn scalar_antiSandwich_multiVector(self_: Scalar, other: MultiVector) -> MultiVector {
    let self_groups = scalar_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let geometric_anti_product: MultiVector = multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.e1234_ * self_.scalar, 1.0, 0.0, 0.0) * vec2<f32>(1.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(other.e423_ * self_.scalar, other.e431_ * self_.scalar, other.e412_ * self_.scalar, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ (vec4<f32>(self_.scalar) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) * other_groups.group2_, 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0, 1.0, 1.0, other.e4_ * self_.scalar) * vec4<f32>(0.0, 0.0, 0.0, 1.0)
    ));
    return multiVector_geometricAntiProduct_scalar(geometric_anti_product, scalar_antiReverse(self_));
}
fn scalar_antiSandwich_plane(self_: Scalar, other: Plane) -> Horizon {
    let geometric_anti_product: Point = point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e423_ * self_.scalar, other.e431_ * self_.scalar, other.e412_ * self_.scalar, 1.0) * vec4<f32>(1.0, 1.0, 1.0, 0.0)
    ));
    return point_geometricAntiProduct_scalar(geometric_anti_product, scalar_antiReverse(self_));
}
fn antiScalar_bitXor_dualNum(self_: AntiScalar, other: DualNum) -> AntiScalar {
    return antiScalar_wedge_dualNum(self_, other);
}
fn antiScalar_bitXor_motor(self_: AntiScalar, other: Motor) -> AntiScalar {
    return antiScalar_wedge_motor(self_, other);
}
fn antiScalar_bitXor_multiVector(self_: AntiScalar, other: MultiVector) -> AntiScalar {
    return antiScalar_wedge_multiVector(self_, other);
}
fn antiScalar_bitXor_scalar(self_: AntiScalar, other: Scalar) -> AntiScalar {
    return antiScalar_wedge_scalar(self_, other);
}
fn dualNum_bitXor_antiScalar(self_: DualNum, other: AntiScalar) -> AntiScalar {
    return dualNum_wedge_antiScalar(self_, other);
}
fn dualNum_bitXor_dualNum(self_: DualNum, other: DualNum) -> DualNum {
    return dualNum_wedge_dualNum(self_, other);
}
fn dualNum_bitXor_flector(self_: DualNum, other: Flector) -> Flector {
    let other_groups = flector_grouped(other);
    return dualNum_wedge_flector(self_, other);
}
fn dualNum_bitXor_horizon(self_: DualNum, other: Horizon) -> Horizon {
    return dualNum_wedge_horizon(self_, other);
}
fn dualNum_bitXor_line(self_: DualNum, other: Line) -> Line {
    let other_groups = line_grouped(other);
    return dualNum_wedge_line(self_, other);
}
fn dualNum_bitXor_motor(self_: DualNum, other: Motor) -> Motor {
    let other_groups = motor_grouped(other);
    return dualNum_wedge_motor(self_, other);
}
fn dualNum_bitXor_multiVector(self_: DualNum, other: MultiVector) -> MultiVector {
    let other_groups = multiVector_grouped(other);
    return dualNum_wedge_multiVector(self_, other);
}
fn dualNum_bitXor_origin(self_: DualNum, other: Origin) -> Origin {
    return dualNum_wedge_origin(self_, other);
}
fn dualNum_bitXor_plane(self_: DualNum, other: Plane) -> Plane {
    let other_groups = plane_grouped(other);
    return dualNum_wedge_plane(self_, other);
}
fn dualNum_bitXor_point(self_: DualNum, other: Point) -> Point {
    let other_groups = point_grouped(other);
    return dualNum_wedge_point(self_, other);
}
fn dualNum_bitXor_scalar(self_: DualNum, other: Scalar) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    return dualNum_wedge_scalar(self_, other);
}
fn flector_bitXor_dualNum(self_: Flector, other: DualNum) -> Flector {
    let self_groups = flector_grouped(self_);
    return flector_wedge_dualNum(self_, other);
}
fn flector_bitXor_flector(self_: Flector, other: Flector) -> Motor {
    return flector_wedge_flector(self_, other);
}
fn flector_bitXor_horizon(self_: Flector, other: Horizon) -> AntiScalar {
    return flector_wedge_horizon(self_, other);
}
fn flector_bitXor_line(self_: Flector, other: Line) -> Plane {
    return flector_wedge_line(self_, other);
}
fn flector_bitXor_motor(self_: Flector, other: Motor) -> Flector {
    let self_groups = flector_grouped(self_);
    return flector_wedge_motor(self_, other);
}
fn flector_bitXor_multiVector(self_: Flector, other: MultiVector) -> MultiVector {
    let self_groups = flector_grouped(self_);
    return flector_wedge_multiVector(self_, other);
}
fn flector_bitXor_origin(self_: Flector, other: Origin) -> Motor {
    return flector_wedge_origin(self_, other);
}
fn flector_bitXor_plane(self_: Flector, other: Plane) -> AntiScalar {
    return flector_wedge_plane(self_, other);
}
fn flector_bitXor_point(self_: Flector, other: Point) -> Motor {
    return flector_wedge_point(self_, other);
}
fn flector_bitXor_scalar(self_: Flector, other: Scalar) -> Flector {
    let self_groups = flector_grouped(self_);
    return flector_wedge_scalar(self_, other);
}
fn horizon_bitXor_dualNum(self_: Horizon, other: DualNum) -> Horizon {
    return horizon_wedge_dualNum(self_, other);
}
fn horizon_bitXor_flector(self_: Horizon, other: Flector) -> AntiScalar {
    return horizon_wedge_flector(self_, other);
}
fn horizon_bitXor_motor(self_: Horizon, other: Motor) -> Horizon {
    return horizon_wedge_motor(self_, other);
}
fn horizon_bitXor_multiVector(self_: Horizon, other: MultiVector) -> MultiVector {
    return horizon_wedge_multiVector(self_, other);
}
fn horizon_bitXor_origin(self_: Horizon, other: Origin) -> AntiScalar {
    return horizon_wedge_origin(self_, other);
}
fn horizon_bitXor_point(self_: Horizon, other: Point) -> AntiScalar {
    return horizon_wedge_point(self_, other);
}
fn horizon_bitXor_scalar(self_: Horizon, other: Scalar) -> Horizon {
    return horizon_wedge_scalar(self_, other);
}
fn line_bitXor_dualNum(self_: Line, other: DualNum) -> Line {
    let self_groups = line_grouped(self_);
    return line_wedge_dualNum(self_, other);
}
fn line_bitXor_flector(self_: Line, other: Flector) -> Plane {
    return line_wedge_flector(self_, other);
}
fn line_bitXor_line(self_: Line, other: Line) -> AntiScalar {
    return line_wedge_line(self_, other);
}
fn line_bitXor_motor(self_: Line, other: Motor) -> Motor {
    return line_wedge_motor(self_, other);
}
fn line_bitXor_multiVector(self_: Line, other: MultiVector) -> MultiVector {
    let self_groups = line_grouped(self_);
    return line_wedge_multiVector(self_, other);
}
fn line_bitXor_origin(self_: Line, other: Origin) -> Plane {
    return line_wedge_origin(self_, other);
}
fn line_bitXor_point(self_: Line, other: Point) -> Plane {
    return line_wedge_point(self_, other);
}
fn line_bitXor_scalar(self_: Line, other: Scalar) -> Line {
    let self_groups = line_grouped(self_);
    return line_wedge_scalar(self_, other);
}
fn motor_bitXor_antiScalar(self_: Motor, other: AntiScalar) -> AntiScalar {
    return motor_wedge_antiScalar(self_, other);
}
fn motor_bitXor_dualNum(self_: Motor, other: DualNum) -> Motor {
    let self_groups = motor_grouped(self_);
    return motor_wedge_dualNum(self_, other);
}
fn motor_bitXor_flector(self_: Motor, other: Flector) -> Flector {
    let other_groups = flector_grouped(other);
    return motor_wedge_flector(self_, other);
}
fn motor_bitXor_horizon(self_: Motor, other: Horizon) -> Horizon {
    return motor_wedge_horizon(self_, other);
}
fn motor_bitXor_line(self_: Motor, other: Line) -> Motor {
    return motor_wedge_line(self_, other);
}
fn motor_bitXor_motor(self_: Motor, other: Motor) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = motor_grouped(other);
    return motor_wedge_motor(self_, other);
}
fn motor_bitXor_multiVector(self_: Motor, other: MultiVector) -> MultiVector {
    let other_groups = multiVector_grouped(other);
    return motor_wedge_multiVector(self_, other);
}
fn motor_bitXor_origin(self_: Motor, other: Origin) -> Flector {
    return motor_wedge_origin(self_, other);
}
fn motor_bitXor_plane(self_: Motor, other: Plane) -> Plane {
    let other_groups = plane_grouped(other);
    return motor_wedge_plane(self_, other);
}
fn motor_bitXor_point(self_: Motor, other: Point) -> Flector {
    let other_groups = point_grouped(other);
    return motor_wedge_point(self_, other);
}
fn motor_bitXor_scalar(self_: Motor, other: Scalar) -> Motor {
    let self_groups = motor_grouped(self_);
    return motor_wedge_scalar(self_, other);
}
fn multiVector_bitXor_antiScalar(self_: MultiVector, other: AntiScalar) -> AntiScalar {
    return multiVector_wedge_antiScalar(self_, other);
}
fn multiVector_bitXor_dualNum(self_: MultiVector, other: DualNum) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    return multiVector_wedge_dualNum(self_, other);
}
fn multiVector_bitXor_flector(self_: MultiVector, other: Flector) -> MultiVector {
    let other_groups = flector_grouped(other);
    return multiVector_wedge_flector(self_, other);
}
fn multiVector_bitXor_horizon(self_: MultiVector, other: Horizon) -> MultiVector {
    return multiVector_wedge_horizon(self_, other);
}
fn multiVector_bitXor_line(self_: MultiVector, other: Line) -> MultiVector {
    let other_groups = line_grouped(other);
    return multiVector_wedge_line(self_, other);
}
fn multiVector_bitXor_motor(self_: MultiVector, other: Motor) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    return multiVector_wedge_motor(self_, other);
}
fn multiVector_bitXor_multiVector(self_: MultiVector, other: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_wedge_multiVector(self_, other);
}
fn multiVector_bitXor_origin(self_: MultiVector, other: Origin) -> MultiVector {
    return multiVector_wedge_origin(self_, other);
}
fn multiVector_bitXor_plane(self_: MultiVector, other: Plane) -> MultiVector {
    let other_groups = plane_grouped(other);
    return multiVector_wedge_plane(self_, other);
}
fn multiVector_bitXor_point(self_: MultiVector, other: Point) -> MultiVector {
    let other_groups = point_grouped(other);
    return multiVector_wedge_point(self_, other);
}
fn multiVector_bitXor_scalar(self_: MultiVector, other: Scalar) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    return multiVector_wedge_scalar(self_, other);
}
fn origin_bitXor_dualNum(self_: Origin, other: DualNum) -> Origin {
    return origin_wedge_dualNum(self_, other);
}
fn origin_bitXor_flector(self_: Origin, other: Flector) -> Motor {
    return origin_wedge_flector(self_, other);
}
fn origin_bitXor_horizon(self_: Origin, other: Horizon) -> AntiScalar {
    return origin_wedge_horizon(self_, other);
}
fn origin_bitXor_line(self_: Origin, other: Line) -> Plane {
    return origin_wedge_line(self_, other);
}
fn origin_bitXor_motor(self_: Origin, other: Motor) -> Flector {
    return origin_wedge_motor(self_, other);
}
fn origin_bitXor_multiVector(self_: Origin, other: MultiVector) -> MultiVector {
    return origin_wedge_multiVector(self_, other);
}
fn origin_bitXor_plane(self_: Origin, other: Plane) -> AntiScalar {
    return origin_wedge_plane(self_, other);
}
fn origin_bitXor_point(self_: Origin, other: Point) -> Line {
    return origin_wedge_point(self_, other);
}
fn origin_bitXor_scalar(self_: Origin, other: Scalar) -> Origin {
    return origin_wedge_scalar(self_, other);
}
fn plane_bitXor_dualNum(self_: Plane, other: DualNum) -> Plane {
    let self_groups = plane_grouped(self_);
    return plane_wedge_dualNum(self_, other);
}
fn plane_bitXor_flector(self_: Plane, other: Flector) -> AntiScalar {
    return plane_wedge_flector(self_, other);
}
fn plane_bitXor_motor(self_: Plane, other: Motor) -> Plane {
    let self_groups = plane_grouped(self_);
    return plane_wedge_motor(self_, other);
}
fn plane_bitXor_multiVector(self_: Plane, other: MultiVector) -> MultiVector {
    let self_groups = plane_grouped(self_);
    return plane_wedge_multiVector(self_, other);
}
fn plane_bitXor_origin(self_: Plane, other: Origin) -> AntiScalar {
    return plane_wedge_origin(self_, other);
}
fn plane_bitXor_point(self_: Plane, other: Point) -> AntiScalar {
    return plane_wedge_point(self_, other);
}
fn plane_bitXor_scalar(self_: Plane, other: Scalar) -> Plane {
    let self_groups = plane_grouped(self_);
    return plane_wedge_scalar(self_, other);
}
fn point_bitXor_dualNum(self_: Point, other: DualNum) -> Point {
    let self_groups = point_grouped(self_);
    return point_wedge_dualNum(self_, other);
}
fn point_bitXor_flector(self_: Point, other: Flector) -> Motor {
    return point_wedge_flector(self_, other);
}
fn point_bitXor_horizon(self_: Point, other: Horizon) -> AntiScalar {
    return point_wedge_horizon(self_, other);
}
fn point_bitXor_line(self_: Point, other: Line) -> Plane {
    return point_wedge_line(self_, other);
}
fn point_bitXor_motor(self_: Point, other: Motor) -> Flector {
    let self_groups = point_grouped(self_);
    return point_wedge_motor(self_, other);
}
fn point_bitXor_multiVector(self_: Point, other: MultiVector) -> MultiVector {
    let self_groups = point_grouped(self_);
    return point_wedge_multiVector(self_, other);
}
fn point_bitXor_origin(self_: Point, other: Origin) -> Line {
    return point_wedge_origin(self_, other);
}
fn point_bitXor_plane(self_: Point, other: Plane) -> AntiScalar {
    return point_wedge_plane(self_, other);
}
fn point_bitXor_point(self_: Point, other: Point) -> Line {
    return point_wedge_point(self_, other);
}
fn point_bitXor_scalar(self_: Point, other: Scalar) -> Point {
    let self_groups = point_grouped(self_);
    return point_wedge_scalar(self_, other);
}
fn scalar_bitXor_antiScalar(self_: Scalar, other: AntiScalar) -> AntiScalar {
    return scalar_wedge_antiScalar(self_, other);
}
fn scalar_bitXor_dualNum(self_: Scalar, other: DualNum) -> DualNum {
    let other_groups = dualNum_grouped(other);
    return scalar_wedge_dualNum(self_, other);
}
fn scalar_bitXor_flector(self_: Scalar, other: Flector) -> Flector {
    let other_groups = flector_grouped(other);
    return scalar_wedge_flector(self_, other);
}
fn scalar_bitXor_horizon(self_: Scalar, other: Horizon) -> Horizon {
    return scalar_wedge_horizon(self_, other);
}
fn scalar_bitXor_line(self_: Scalar, other: Line) -> Line {
    let other_groups = line_grouped(other);
    return scalar_wedge_line(self_, other);
}
fn scalar_bitXor_motor(self_: Scalar, other: Motor) -> Motor {
    let other_groups = motor_grouped(other);
    return scalar_wedge_motor(self_, other);
}
fn scalar_bitXor_multiVector(self_: Scalar, other: MultiVector) -> MultiVector {
    let other_groups = multiVector_grouped(other);
    return scalar_wedge_multiVector(self_, other);
}
fn scalar_bitXor_origin(self_: Scalar, other: Origin) -> Origin {
    return scalar_wedge_origin(self_, other);
}
fn scalar_bitXor_plane(self_: Scalar, other: Plane) -> Plane {
    let other_groups = plane_grouped(other);
    return scalar_wedge_plane(self_, other);
}
fn scalar_bitXor_point(self_: Scalar, other: Point) -> Point {
    let other_groups = point_grouped(other);
    return scalar_wedge_point(self_, other);
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
    let self_groups = line_grouped(self_);    return line_rightDual(self_);
}
fn motor_not(self_: Motor) -> Motor {
    let self_groups = motor_grouped(self_);    return motor_rightDual(self_);
}
fn multiVector_not(self_: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);    return multiVector_rightDual(self_);
}
fn plane_not(self_: Plane) -> Origin {
    return plane_rightDual(self_);
}
fn point_not(self_: Point) -> Plane {
    return point_rightDual(self_);
}
fn scalar_not(self_: Scalar) -> AntiScalar {
    return scalar_rightDual(self_);
}

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
    e1234: f32
}
struct AntiScalarGroups {
    // e1234, 0, 0, 0
    group0_: vec4<f32>
}
fn antiScalar_grouped(self_: AntiScalar) -> AntiScalarGroups {
    return AntiScalarGroups(
        vec4<f32>(self_.e1234, 0.0, 0.0, 0.0)
    );
}
fn antiScalar_degroup(self_: AntiScalarGroups) -> AntiScalar {
    return AntiScalar(
        self_.group0_.x
    );
}


struct DualNum {
    scalar: f32, e1234: f32
}
struct DualNumGroups {
    // scalar, e1234, 0, 0
    group0_: vec4<f32>
}
fn dualNum_grouped(self_: DualNum) -> DualNumGroups {
    return DualNumGroups(
        vec4<f32>(self_.scalar, self_.e1234, 0.0, 0.0)
    );
}
fn dualNum_degroup(self_: DualNumGroups) -> DualNum {
    return DualNum(
        self_.group0_.x, self_.group0_.y
    );
}


struct Flector {
    e1: f32, e2: f32, e3: f32, e4: f32,
    e423: f32, e431: f32, e412: f32, e321: f32
}
struct FlectorGroups {
    // e1, e2, e3, e4
    group0_: vec4<f32>,
    // e423, e431, e412, e321
    group1_: vec4<f32>
}
fn flector_grouped(self_: Flector) -> FlectorGroups {
    return FlectorGroups(
        vec4<f32>(self_.e1, self_.e2, self_.e3, self_.e4),
        vec4<f32>(self_.e423, self_.e431, self_.e412, self_.e321)
    );
}
fn flector_degroup(self_: FlectorGroups) -> Flector {
    return Flector(
        self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group0_.w,
        self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group1_.w
    );
}


struct Horizon {
    e321: f32
}
struct HorizonGroups {
    // e321, 0, 0, 0
    group0_: vec4<f32>
}
fn horizon_grouped(self_: Horizon) -> HorizonGroups {
    return HorizonGroups(
        vec4<f32>(self_.e321, 0.0, 0.0, 0.0)
    );
}
fn horizon_degroup(self_: HorizonGroups) -> Horizon {
    return Horizon(
        self_.group0_.x
    );
}


struct Line {
    e41: f32, e42: f32, e43: f32,
    e23: f32, e31: f32, e12: f32
}
struct LineGroups {
    // e41, e42, e43, 0
    group0_: vec4<f32>,
    // e23, e31, e12, 0
    group1_: vec4<f32>
}
fn line_grouped(self_: Line) -> LineGroups {
    return LineGroups(
        vec4<f32>(self_.e41, self_.e42, self_.e43, 0.0),
        vec4<f32>(self_.e23, self_.e31, self_.e12, 0.0)
    );
}
fn line_degroup(self_: LineGroups) -> Line {
    return Line(
        self_.group0_.x, self_.group0_.y, self_.group0_.z,
        self_.group1_.x, self_.group1_.y, self_.group1_.z
    );
}


struct Motor {
    e41: f32, e42: f32, e43: f32, e1234: f32,
    e23: f32, e31: f32, e12: f32, scalar: f32
}
struct MotorGroups {
    // e41, e42, e43, e1234
    group0_: vec4<f32>,
    // e23, e31, e12, scalar
    group1_: vec4<f32>
}
fn motor_grouped(self_: Motor) -> MotorGroups {
    return MotorGroups(
        vec4<f32>(self_.e41, self_.e42, self_.e43, self_.e1234),
        vec4<f32>(self_.e23, self_.e31, self_.e12, self_.scalar)
    );
}
fn motor_degroup(self_: MotorGroups) -> Motor {
    return Motor(
        self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group0_.w,
        self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group1_.w
    );
}


struct MultiVector {
    scalar: f32, e1234: f32,
    e1: f32, e2: f32, e3: f32, e4: f32,
    e41: f32, e42: f32, e43: f32,
    e23: f32, e31: f32, e12: f32,
    e423: f32, e431: f32, e412: f32, e321: f32
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
        vec4<f32>(self_.scalar, self_.e1234, 0.0, 0.0),
        vec4<f32>(self_.e1, self_.e2, self_.e3, self_.e4),
        vec4<f32>(self_.e41, self_.e42, self_.e43, 0.0),
        vec4<f32>(self_.e23, self_.e31, self_.e12, 0.0),
        vec4<f32>(self_.e423, self_.e431, self_.e412, self_.e321)
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
    e4: f32
}
struct OriginGroups {
    // e4, 0, 0, 0
    group0_: vec4<f32>
}
fn origin_grouped(self_: Origin) -> OriginGroups {
    return OriginGroups(
        vec4<f32>(self_.e4, 0.0, 0.0, 0.0)
    );
}
fn origin_degroup(self_: OriginGroups) -> Origin {
    return Origin(
        self_.group0_.x
    );
}


struct Plane {
    e423: f32, e431: f32, e412: f32, e321: f32
}
struct PlaneGroups {
    // e423, e431, e412, e321
    group0_: vec4<f32>
}
fn plane_grouped(self_: Plane) -> PlaneGroups {
    return PlaneGroups(
        vec4<f32>(self_.e423, self_.e431, self_.e412, self_.e321)
    );
}
fn plane_degroup(self_: PlaneGroups) -> Plane {
    return Plane(
        self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group0_.w
    );
}


struct Point {
    e1: f32, e2: f32, e3: f32, e4: f32
}
struct PointGroups {
    // e1, e2, e3, e4
    group0_: vec4<f32>
}
fn point_grouped(self_: Point) -> PointGroups {
    return PointGroups(
        vec4<f32>(self_.e1, self_.e2, self_.e3, self_.e4)
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
    let self_groups = antiScalar_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(other.e1234 + self.e1234, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_add_dualNum(self_: AntiScalar, other: DualNum) -> DualNum {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(other_groups.group0_.x, other_groups.group0_.y + self.e1234, 0.0, 0.0)
    ));
}
fn antiScalar_add_flector(self_: AntiScalar, other: Flector) -> MultiVector {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = flector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, self.e1234, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ other_groups.group0_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ other_groups.group1_
    ));
}
fn antiScalar_add_horizon(self_: AntiScalar, other: Horizon) -> MultiVector {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = horizon_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, self.e1234, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other.e321)
    ));
}
fn antiScalar_add_line(self_: AntiScalar, other: Line) -> Motor {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = line_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other_groups.group0_.xyz.xyz, self.e1234), 
        /* e23, e31, e12, scalar */ vec4<f32>(other_groups.group1_.xyz.xyz, 0.0)
    ));
}
fn antiScalar_add_motor(self_: AntiScalar, other: Motor) -> Motor {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, other_groups.group0_.w + self.e1234), 
        /* e23, e31, e12, scalar */ other_groups.group1_
    ));
}
fn antiScalar_add_multiVector(self_: AntiScalar, other: MultiVector) -> MultiVector {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other_groups.group0_.x, other_groups.group0_.y + self.e1234, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ other_groups.group1_, 
        /* e41, e42, e43 */ other_groups.group2_, 
        /* e23, e31, e12 */ other_groups.group3_, 
        /* e423, e431, e412, e321 */ other_groups.group4_
    ));
}
fn antiScalar_add_origin(self_: AntiScalar, other: Origin) -> MultiVector {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = origin_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, self.e1234, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other.e4), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn antiScalar_add_plane(self_: AntiScalar, other: Plane) -> MultiVector {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = plane_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, self.e1234, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ other_groups.group0_
    ));
}
fn antiScalar_add_point(self_: AntiScalar, other: Point) -> MultiVector {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = point_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, self.e1234, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ other_groups.group0_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn antiScalar_add_scalar(self_: AntiScalar, other: Scalar) -> DualNum {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = scalar_grouped(other);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar, self.e1234, 0.0, 0.0)
    ));
}
fn dualNum_add_antiScalar(self_: DualNum, other: AntiScalar) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(self_.group0_.x, self_.group0_.y + other.e1234, 0.0, 0.0)
    ));
}
fn dualNum_add_dualNum(self_: DualNum, other: DualNum) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ other_groups.group0_ + self_.group0_
    ));
}
fn dualNum_add_flector(self_: DualNum, other: Flector) -> MultiVector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = flector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_.group0_, 
        /* e1, e2, e3, e4 */ other_groups.group0_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ other_groups.group1_
    ));
}
fn dualNum_add_horizon(self_: DualNum, other: Horizon) -> MultiVector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = horizon_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other.e321)
    ));
}
fn dualNum_add_line(self_: DualNum, other: Line) -> Motor {
    let self_groups = dualNum_grouped(self_);
    let other_groups = line_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other_groups.group0_.xyz.xyz, self_.group0_.y), 
        /* e23, e31, e12, scalar */ vec4<f32>(other_groups.group1_.xyz.xyz, self_.group0_.x)
    ));
}
fn dualNum_add_motor(self_: DualNum, other: Motor) -> Motor {
    let self_groups = dualNum_grouped(self_);
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, self_.group0_.y + other_groups.group0_.w), 
        /* e23, e31, e12, scalar */ vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, self_.group0_.x + other_groups.group1_.w)
    ));
}
fn dualNum_add_multiVector(self_: DualNum, other: MultiVector) -> MultiVector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_.group0_ + other_groups.group0_, 
        /* e1, e2, e3, e4 */ other_groups.group1_, 
        /* e41, e42, e43 */ other_groups.group2_, 
        /* e23, e31, e12 */ other_groups.group3_, 
        /* e423, e431, e412, e321 */ other_groups.group4_
    ));
}
fn dualNum_add_origin(self_: DualNum, other: Origin) -> MultiVector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = origin_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other.e4), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn dualNum_add_plane(self_: DualNum, other: Plane) -> MultiVector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = plane_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_.group0_, 
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
        /* scalar, e1234 */ self_.group0_, 
        /* e1, e2, e3, e4 */ other_groups.group0_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn dualNum_add_scalar(self_: DualNum, other: Scalar) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let other_groups = scalar_grouped(other);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(self_.group0_.x + other.scalar, self_.group0_.y, 0.0, 0.0)
    ));
}
fn flector_add_antiScalar(self_: Flector, other: AntiScalar) -> MultiVector {
    let self_groups = flector_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, other.e1234, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ self_.group0_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ self_.group1_
    ));
}
fn flector_add_dualNum(self_: Flector, other: DualNum) -> MultiVector {
    let self_groups = flector_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_, 
        /* e1, e2, e3, e4 */ self_.group0_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ self_.group1_
    ));
}
fn flector_add_flector(self_: Flector, other: Flector) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ other_groups.group0_ + self_.group0_, 
        /* e423, e431, e412, e321 */ other_groups.group1_ + self_.group1_
    ));
}
fn flector_add_horizon(self_: Flector, other: Horizon) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = horizon_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ self_.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group1_.w + other.e321)
    ));
}
fn flector_add_line(self_: Flector, other: Line) -> MultiVector {
    let self_groups = flector_grouped(self_);
    let other_groups = line_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ self_.group0_, 
        /* e41, e42, e43 */ other_groups.group0_, 
        /* e23, e31, e12 */ other_groups.group1_, 
        /* e423, e431, e412, e321 */ self_.group1_
    ));
}
fn flector_add_motor(self_: Flector, other: Motor) -> MultiVector {
    let self_groups = flector_grouped(self_);
    let other_groups = motor_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other_groups.group1_.w, other_groups.group0_.w, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ self_.group0_, 
        /* e41, e42, e43 */ vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0), 
        /* e423, e431, e412, e321 */ self_.group1_
    ));
}
fn flector_add_multiVector(self_: Flector, other: MultiVector) -> MultiVector {
    let self_groups = flector_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_, 
        /* e1, e2, e3, e4 */ self_.group0_ + other_groups.group1_, 
        /* e41, e42, e43 */ other_groups.group2_, 
        /* e23, e31, e12 */ other_groups.group3_, 
        /* e423, e431, e412, e321 */ self_.group1_ + other_groups.group4_
    ));
}
fn flector_add_origin(self_: Flector, other: Origin) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = origin_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group0_.w + other.e4), 
        /* e423, e431, e412, e321 */ self_.group1_
    ));
}
fn flector_add_plane(self_: Flector, other: Plane) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = plane_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ self_.group0_, 
        /* e423, e431, e412, e321 */ self_.group1_ + other_groups.group0_
    ));
}
fn flector_add_point(self_: Flector, other: Point) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = point_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ self_.group0_ + other_groups.group0_, 
        /* e423, e431, e412, e321 */ self_.group1_
    ));
}
fn flector_add_scalar(self_: Flector, other: Scalar) -> MultiVector {
    let self_groups = flector_grouped(self_);
    let other_groups = scalar_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ self_.group0_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ self_.group1_
    ));
}
fn horizon_add_antiScalar(self_: Horizon, other: AntiScalar) -> MultiVector {
    let self_groups = horizon_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, other.e1234, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self.e321)
    ));
}
fn horizon_add_dualNum(self_: Horizon, other: DualNum) -> MultiVector {
    let self_groups = horizon_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self.e321)
    ));
}
fn horizon_add_flector(self_: Horizon, other: Flector) -> Flector {
    let self_groups = horizon_grouped(self_);
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ other_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, other_groups.group1_.w + self.e321)
    ));
}
fn horizon_add_horizon(self_: Horizon, other: Horizon) -> Horizon {
    let self_groups = horizon_grouped(self_);
    let other_groups = horizon_grouped(other);
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(other.e321 + self.e321, 0.0, 0.0, 0.0)
    ));
}
fn horizon_add_line(self_: Horizon, other: Line) -> MultiVector {
    let self_groups = horizon_grouped(self_);
    let other_groups = line_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ other_groups.group0_, 
        /* e23, e31, e12 */ other_groups.group1_, 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self.e321)
    ));
}
fn horizon_add_motor(self_: Horizon, other: Motor) -> MultiVector {
    let self_groups = horizon_grouped(self_);
    let other_groups = motor_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other_groups.group1_.w, other_groups.group0_.w, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self.e321)
    ));
}
fn horizon_add_multiVector(self_: Horizon, other: MultiVector) -> MultiVector {
    let self_groups = horizon_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_, 
        /* e1, e2, e3, e4 */ other_groups.group1_, 
        /* e41, e42, e43 */ other_groups.group2_, 
        /* e23, e31, e12 */ other_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group4_.x, other_groups.group4_.y, other_groups.group4_.z, other_groups.group4_.w + self.e321)
    ));
}
fn horizon_add_origin(self_: Horizon, other: Origin) -> Flector {
    let self_groups = horizon_grouped(self_);
    let other_groups = origin_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other.e4), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self.e321)
    ));
}
fn horizon_add_plane(self_: Horizon, other: Plane) -> Plane {
    let self_groups = horizon_grouped(self_);
    let other_groups = plane_grouped(other);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, other_groups.group0_.w + self.e321)
    ));
}
fn horizon_add_point(self_: Horizon, other: Point) -> Flector {
    let self_groups = horizon_grouped(self_);
    let other_groups = point_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ other_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self.e321)
    ));
}
fn horizon_add_scalar(self_: Horizon, other: Scalar) -> MultiVector {
    let self_groups = horizon_grouped(self_);
    let other_groups = scalar_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self.e321)
    ));
}
fn line_add_antiScalar(self_: Line, other: AntiScalar) -> Motor {
    let self_groups = line_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.group0_.xyz.xyz, other.e1234), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.group1_.xyz.xyz, 0.0)
    ));
}
fn line_add_dualNum(self_: Line, other: DualNum) -> Motor {
    let self_groups = line_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.group0_.xyz.xyz, other_groups.group0_.y), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.group1_.xyz.xyz, other_groups.group0_.x)
    ));
}
fn line_add_flector(self_: Line, other: Flector) -> MultiVector {
    let self_groups = line_grouped(self_);
    let other_groups = flector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ other_groups.group0_, 
        /* e41, e42, e43 */ self_.group0_, 
        /* e23, e31, e12 */ self_.group1_, 
        /* e423, e431, e412, e321 */ other_groups.group1_
    ));
}
fn line_add_horizon(self_: Line, other: Horizon) -> MultiVector {
    let self_groups = line_grouped(self_);
    let other_groups = horizon_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ self_.group0_, 
        /* e23, e31, e12 */ self_.group1_, 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other.e321)
    ));
}
fn line_add_line(self_: Line, other: Line) -> Line {
    let self_groups = line_grouped(self_);
    let other_groups = line_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ other_groups.group0_ + self_.group0_, 
        /* e23, e31, e12 */ other_groups.group1_ + self_.group1_
    ));
}
fn line_add_motor(self_: Line, other: Motor) -> Motor {
    let self_groups = line_grouped(self_);
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.group0_.x + other_groups.group0_.x, self_.group0_.y + other_groups.group0_.y, self_.group0_.z + other_groups.group0_.z, other_groups.group0_.w), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.group1_.x + other_groups.group1_.x, self_.group1_.y + other_groups.group1_.y, self_.group1_.z + other_groups.group1_.z, other_groups.group1_.w)
    ));
}
fn line_add_multiVector(self_: Line, other: MultiVector) -> MultiVector {
    let self_groups = line_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_, 
        /* e1, e2, e3, e4 */ other_groups.group1_, 
        /* e41, e42, e43 */ self_.group0_ + other_groups.group2_, 
        /* e23, e31, e12 */ self_.group1_ + other_groups.group3_, 
        /* e423, e431, e412, e321 */ other_groups.group4_
    ));
}
fn line_add_origin(self_: Line, other: Origin) -> MultiVector {
    let self_groups = line_grouped(self_);
    let other_groups = origin_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other.e4), 
        /* e41, e42, e43 */ self_.group0_, 
        /* e23, e31, e12 */ self_.group1_, 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn line_add_plane(self_: Line, other: Plane) -> MultiVector {
    let self_groups = line_grouped(self_);
    let other_groups = plane_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ self_.group0_, 
        /* e23, e31, e12 */ self_.group1_, 
        /* e423, e431, e412, e321 */ other_groups.group0_
    ));
}
fn line_add_point(self_: Line, other: Point) -> MultiVector {
    let self_groups = line_grouped(self_);
    let other_groups = point_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ other_groups.group0_, 
        /* e41, e42, e43 */ self_.group0_, 
        /* e23, e31, e12 */ self_.group1_, 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn line_add_scalar(self_: Line, other: Scalar) -> Motor {
    let self_groups = line_grouped(self_);
    let other_groups = scalar_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.group0_.xyz.xyz, 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.group1_.xyz.xyz, other.scalar)
    ));
}
fn motor_add_antiScalar(self_: Motor, other: AntiScalar) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group0_.w + other.e1234), 
        /* e23, e31, e12, scalar */ self_.group1_
    ));
}
fn motor_add_dualNum(self_: Motor, other: DualNum) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, other_groups.group0_.y + self_.group0_.w), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, other_groups.group0_.x + self_.group1_.w)
    ));
}
fn motor_add_flector(self_: Motor, other: Flector) -> MultiVector {
    let self_groups = motor_grouped(self_);
    let other_groups = flector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.group1_.w, self_.group0_.w, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ other_groups.group0_, 
        /* e41, e42, e43 */ vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0), 
        /* e423, e431, e412, e321 */ other_groups.group1_
    ));
}
fn motor_add_horizon(self_: Motor, other: Horizon) -> MultiVector {
    let self_groups = motor_grouped(self_);
    let other_groups = horizon_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.group1_.w, self_.group0_.w, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other.e321)
    ));
}
fn motor_add_line(self_: Motor, other: Line) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = line_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other_groups.group0_.x + self_.group0_.x, other_groups.group0_.y + self_.group0_.y, other_groups.group0_.z + self_.group0_.z, self_.group0_.w), 
        /* e23, e31, e12, scalar */ vec4<f32>(other_groups.group1_.x + self_.group1_.x, other_groups.group1_.y + self_.group1_.y, other_groups.group1_.z + self_.group1_.z, self_.group1_.w)
    ));
}
fn motor_add_motor(self_: Motor, other: Motor) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ other_groups.group0_ + self_.group0_, 
        /* e23, e31, e12, scalar */ other_groups.group1_ + self_.group1_
    ));
}
fn motor_add_multiVector(self_: Motor, other: MultiVector) -> MultiVector {
    let self_groups = motor_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.group1_.w, self_.group0_.w, 0.0, 0.0) + other_groups.group0_, 
        /* e1, e2, e3, e4 */ other_groups.group1_, 
        /* e41, e42, e43 */ vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0) + other_groups.group2_, 
        /* e23, e31, e12 */ vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0) + other_groups.group3_, 
        /* e423, e431, e412, e321 */ other_groups.group4_
    ));
}
fn motor_add_origin(self_: Motor, other: Origin) -> MultiVector {
    let self_groups = motor_grouped(self_);
    let other_groups = origin_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.group1_.w, self_.group0_.w, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other.e4), 
        /* e41, e42, e43 */ vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn motor_add_plane(self_: Motor, other: Plane) -> MultiVector {
    let self_groups = motor_grouped(self_);
    let other_groups = plane_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.group1_.w, self_.group0_.w, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0), 
        /* e423, e431, e412, e321 */ other_groups.group0_
    ));
}
fn motor_add_point(self_: Motor, other: Point) -> MultiVector {
    let self_groups = motor_grouped(self_);
    let other_groups = point_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.group1_.w, self_.group0_.w, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ other_groups.group0_, 
        /* e41, e42, e43 */ vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn motor_add_scalar(self_: Motor, other: Scalar) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = scalar_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ self_.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group1_.w + other.scalar)
    ));
}
fn multiVector_add_antiScalar(self_: MultiVector, other: AntiScalar) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.group0_.x, self_.group0_.y + other.e1234, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ self_.group1_, 
        /* e41, e42, e43 */ self_.group2_, 
        /* e23, e31, e12 */ self_.group3_, 
        /* e423, e431, e412, e321 */ self_.group4_
    ));
}
fn multiVector_add_dualNum(self_: MultiVector, other: DualNum) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_ + self_.group0_, 
        /* e1, e2, e3, e4 */ self_.group1_, 
        /* e41, e42, e43 */ self_.group2_, 
        /* e23, e31, e12 */ self_.group3_, 
        /* e423, e431, e412, e321 */ self_.group4_
    ));
}
fn multiVector_add_flector(self_: MultiVector, other: Flector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = flector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_.group0_, 
        /* e1, e2, e3, e4 */ other_groups.group0_ + self_.group1_, 
        /* e41, e42, e43 */ self_.group2_, 
        /* e23, e31, e12 */ self_.group3_, 
        /* e423, e431, e412, e321 */ other_groups.group1_ + self_.group4_
    ));
}
fn multiVector_add_horizon(self_: MultiVector, other: Horizon) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = horizon_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_.group0_, 
        /* e1, e2, e3, e4 */ self_.group1_, 
        /* e41, e42, e43 */ self_.group2_, 
        /* e23, e31, e12 */ self_.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group4_.x, self_.group4_.y, self_.group4_.z, self_.group4_.w + other.e321)
    ));
}
fn multiVector_add_line(self_: MultiVector, other: Line) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = line_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_.group0_, 
        /* e1, e2, e3, e4 */ self_.group1_, 
        /* e41, e42, e43 */ other_groups.group0_ + self_.group2_, 
        /* e23, e31, e12 */ other_groups.group1_ + self_.group3_, 
        /* e423, e431, e412, e321 */ self_.group4_
    ));
}
fn multiVector_add_motor(self_: MultiVector, other: Motor) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = motor_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other_groups.group1_.w, other_groups.group0_.w, 0.0, 0.0) + self_.group0_, 
        /* e1, e2, e3, e4 */ self_.group1_, 
        /* e41, e42, e43 */ vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0) + self_.group2_, 
        /* e23, e31, e12 */ vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0) + self_.group3_, 
        /* e423, e431, e412, e321 */ self_.group4_
    ));
}
fn multiVector_add_multiVector(self_: MultiVector, other: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_ + self_.group0_, 
        /* e1, e2, e3, e4 */ other_groups.group1_ + self_.group1_, 
        /* e41, e42, e43 */ other_groups.group2_ + self_.group2_, 
        /* e23, e31, e12 */ other_groups.group3_ + self_.group3_, 
        /* e423, e431, e412, e321 */ other_groups.group4_ + self_.group4_
    ));
}
fn multiVector_add_origin(self_: MultiVector, other: Origin) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = origin_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group1_.w + other.e4), 
        /* e41, e42, e43 */ self_.group2_, 
        /* e23, e31, e12 */ self_.group3_, 
        /* e423, e431, e412, e321 */ self_.group4_
    ));
}
fn multiVector_add_plane(self_: MultiVector, other: Plane) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = plane_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_.group0_, 
        /* e1, e2, e3, e4 */ self_.group1_, 
        /* e41, e42, e43 */ self_.group2_, 
        /* e23, e31, e12 */ self_.group3_, 
        /* e423, e431, e412, e321 */ self_.group4_ + other_groups.group0_
    ));
}
fn multiVector_add_point(self_: MultiVector, other: Point) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = point_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_.group0_, 
        /* e1, e2, e3, e4 */ self_.group1_ + other_groups.group0_, 
        /* e41, e42, e43 */ self_.group2_, 
        /* e23, e31, e12 */ self_.group3_, 
        /* e423, e431, e412, e321 */ self_.group4_
    ));
}
fn multiVector_add_scalar(self_: MultiVector, other: Scalar) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = scalar_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.group0_.x + other.scalar, self_.group0_.y, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ self_.group1_, 
        /* e41, e42, e43 */ self_.group2_, 
        /* e23, e31, e12 */ self_.group3_, 
        /* e423, e431, e412, e321 */ self_.group4_
    ));
}
fn origin_add_antiScalar(self_: Origin, other: AntiScalar) -> MultiVector {
    let self_groups = origin_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, other.e1234, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self.e4), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn origin_add_dualNum(self_: Origin, other: DualNum) -> MultiVector {
    let self_groups = origin_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self.e4), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn origin_add_flector(self_: Origin, other: Flector) -> Flector {
    let self_groups = origin_grouped(self_);
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, other_groups.group0_.w + self.e4), 
        /* e423, e431, e412, e321 */ other_groups.group1_
    ));
}
fn origin_add_horizon(self_: Origin, other: Horizon) -> Flector {
    let self_groups = origin_grouped(self_);
    let other_groups = horizon_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self.e4), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other.e321)
    ));
}
fn origin_add_line(self_: Origin, other: Line) -> MultiVector {
    let self_groups = origin_grouped(self_);
    let other_groups = line_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self.e4), 
        /* e41, e42, e43 */ other_groups.group0_, 
        /* e23, e31, e12 */ other_groups.group1_, 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn origin_add_motor(self_: Origin, other: Motor) -> MultiVector {
    let self_groups = origin_grouped(self_);
    let other_groups = motor_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other_groups.group1_.w, other_groups.group0_.w, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self.e4), 
        /* e41, e42, e43 */ vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn origin_add_multiVector(self_: Origin, other: MultiVector) -> MultiVector {
    let self_groups = origin_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, other_groups.group1_.w + self.e4), 
        /* e41, e42, e43 */ other_groups.group2_, 
        /* e23, e31, e12 */ other_groups.group3_, 
        /* e423, e431, e412, e321 */ other_groups.group4_
    ));
}
fn origin_add_origin(self_: Origin, other: Origin) -> Origin {
    let self_groups = origin_grouped(self_);
    let other_groups = origin_grouped(other);
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(other.e4 + self.e4, 0.0, 0.0, 0.0)
    ));
}
fn origin_add_plane(self_: Origin, other: Plane) -> Flector {
    let self_groups = origin_grouped(self_);
    let other_groups = plane_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self.e4), 
        /* e423, e431, e412, e321 */ other_groups.group0_
    ));
}
fn origin_add_point(self_: Origin, other: Point) -> Point {
    let self_groups = origin_grouped(self_);
    let other_groups = point_grouped(other);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, other_groups.group0_.w + self.e4)
    ));
}
fn origin_add_scalar(self_: Origin, other: Scalar) -> MultiVector {
    let self_groups = origin_grouped(self_);
    let other_groups = scalar_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self.e4), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn plane_add_antiScalar(self_: Plane, other: AntiScalar) -> MultiVector {
    let self_groups = plane_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, other.e1234, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ self_.group0_
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
        /* e423, e431, e412, e321 */ self_.group0_
    ));
}
fn plane_add_flector(self_: Plane, other: Flector) -> Flector {
    let self_groups = plane_grouped(self_);
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ other_groups.group0_, 
        /* e423, e431, e412, e321 */ other_groups.group1_ + self_.group0_
    ));
}
fn plane_add_horizon(self_: Plane, other: Horizon) -> Plane {
    let self_groups = plane_grouped(self_);
    let other_groups = horizon_grouped(other);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group0_.w + other.e321)
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
        /* e423, e431, e412, e321 */ self_.group0_
    ));
}
fn plane_add_motor(self_: Plane, other: Motor) -> MultiVector {
    let self_groups = plane_grouped(self_);
    let other_groups = motor_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other_groups.group1_.w, other_groups.group0_.w, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0), 
        /* e423, e431, e412, e321 */ self_.group0_
    ));
}
fn plane_add_multiVector(self_: Plane, other: MultiVector) -> MultiVector {
    let self_groups = plane_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_, 
        /* e1, e2, e3, e4 */ other_groups.group1_, 
        /* e41, e42, e43 */ other_groups.group2_, 
        /* e23, e31, e12 */ other_groups.group3_, 
        /* e423, e431, e412, e321 */ other_groups.group4_ + self_.group0_
    ));
}
fn plane_add_origin(self_: Plane, other: Origin) -> Flector {
    let self_groups = plane_grouped(self_);
    let other_groups = origin_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other.e4), 
        /* e423, e431, e412, e321 */ self_.group0_
    ));
}
fn plane_add_plane(self_: Plane, other: Plane) -> Plane {
    let self_groups = plane_grouped(self_);
    let other_groups = plane_grouped(other);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ other_groups.group0_ + self_.group0_
    ));
}
fn plane_add_point(self_: Plane, other: Point) -> Flector {
    let self_groups = plane_grouped(self_);
    let other_groups = point_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ other_groups.group0_, 
        /* e423, e431, e412, e321 */ self_.group0_
    ));
}
fn plane_add_scalar(self_: Plane, other: Scalar) -> MultiVector {
    let self_groups = plane_grouped(self_);
    let other_groups = scalar_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ self_.group0_
    ));
}
fn point_add_antiScalar(self_: Point, other: AntiScalar) -> MultiVector {
    let self_groups = point_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, other.e1234, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ self_.group0_, 
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
        /* e1, e2, e3, e4 */ self_.group0_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn point_add_flector(self_: Point, other: Flector) -> Flector {
    let self_groups = point_grouped(self_);
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ other_groups.group0_ + self_.group0_, 
        /* e423, e431, e412, e321 */ other_groups.group1_
    ));
}
fn point_add_horizon(self_: Point, other: Horizon) -> Flector {
    let self_groups = point_grouped(self_);
    let other_groups = horizon_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ self_.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other.e321)
    ));
}
fn point_add_line(self_: Point, other: Line) -> MultiVector {
    let self_groups = point_grouped(self_);
    let other_groups = line_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ self_.group0_, 
        /* e41, e42, e43 */ other_groups.group0_, 
        /* e23, e31, e12 */ other_groups.group1_, 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn point_add_motor(self_: Point, other: Motor) -> MultiVector {
    let self_groups = point_grouped(self_);
    let other_groups = motor_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other_groups.group1_.w, other_groups.group0_.w, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ self_.group0_, 
        /* e41, e42, e43 */ vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn point_add_multiVector(self_: Point, other: MultiVector) -> MultiVector {
    let self_groups = point_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_, 
        /* e1, e2, e3, e4 */ other_groups.group1_ + self_.group0_, 
        /* e41, e42, e43 */ other_groups.group2_, 
        /* e23, e31, e12 */ other_groups.group3_, 
        /* e423, e431, e412, e321 */ other_groups.group4_
    ));
}
fn point_add_origin(self_: Point, other: Origin) -> Point {
    let self_groups = point_grouped(self_);
    let other_groups = origin_grouped(other);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group0_.w + other.e4)
    ));
}
fn point_add_plane(self_: Point, other: Plane) -> Flector {
    let self_groups = point_grouped(self_);
    let other_groups = plane_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ self_.group0_, 
        /* e423, e431, e412, e321 */ other_groups.group0_
    ));
}
fn point_add_point(self_: Point, other: Point) -> Point {
    let self_groups = point_grouped(self_);
    let other_groups = point_grouped(other);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ other_groups.group0_ + self_.group0_
    ));
}
fn point_add_scalar(self_: Point, other: Scalar) -> MultiVector {
    let self_groups = point_grouped(self_);
    let other_groups = scalar_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ self_.group0_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn scalar_add_antiScalar(self_: Scalar, other: AntiScalar) -> DualNum {
    let self_groups = scalar_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(self.scalar, other.e1234, 0.0, 0.0)
    ));
}
fn scalar_add_dualNum(self_: Scalar, other: DualNum) -> DualNum {
    let self_groups = scalar_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(other_groups.group0_.x + self.scalar, other_groups.group0_.y, 0.0, 0.0)
    ));
}
fn scalar_add_flector(self_: Scalar, other: Flector) -> MultiVector {
    let self_groups = scalar_grouped(self_);
    let other_groups = flector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self.scalar, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ other_groups.group0_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ other_groups.group1_
    ));
}
fn scalar_add_horizon(self_: Scalar, other: Horizon) -> MultiVector {
    let self_groups = scalar_grouped(self_);
    let other_groups = horizon_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self.scalar, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other.e321)
    ));
}
fn scalar_add_line(self_: Scalar, other: Line) -> Motor {
    let self_groups = scalar_grouped(self_);
    let other_groups = line_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other_groups.group0_.xyz.xyz, 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(other_groups.group1_.xyz.xyz, self.scalar)
    ));
}
fn scalar_add_motor(self_: Scalar, other: Motor) -> Motor {
    let self_groups = scalar_grouped(self_);
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ other_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, other_groups.group1_.w + self.scalar)
    ));
}
fn scalar_add_multiVector(self_: Scalar, other: MultiVector) -> MultiVector {
    let self_groups = scalar_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other_groups.group0_.x + self.scalar, other_groups.group0_.y, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ other_groups.group1_, 
        /* e41, e42, e43 */ other_groups.group2_, 
        /* e23, e31, e12 */ other_groups.group3_, 
        /* e423, e431, e412, e321 */ other_groups.group4_
    ));
}
fn scalar_add_origin(self_: Scalar, other: Origin) -> MultiVector {
    let self_groups = scalar_grouped(self_);
    let other_groups = origin_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self.scalar, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other.e4), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn scalar_add_plane(self_: Scalar, other: Plane) -> MultiVector {
    let self_groups = scalar_grouped(self_);
    let other_groups = plane_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self.scalar, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ other_groups.group0_
    ));
}
fn scalar_add_point(self_: Scalar, other: Point) -> MultiVector {
    let self_groups = scalar_grouped(self_);
    let other_groups = point_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self.scalar, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ other_groups.group0_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn scalar_add_scalar(self_: Scalar, other: Scalar) -> Scalar {
    let self_groups = scalar_grouped(self_);
    let other_groups = scalar_grouped(other);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(other.scalar + self.scalar, 0.0, 0.0, 0.0)
    ));
}
fn horizon_antiConstraintValid(self_: Horizon) -> Horizon {
    let self_groups = horizon_grouped(self_);    return self;
}
fn scalar_antiConstraintValid(self_: Scalar) -> Scalar {
    let self_groups = scalar_grouped(self_);    return self;
}
fn antiScalar_antiConstraintViolation(self_: AntiScalar) -> Scalar {
    let self_groups = antiScalar_grouped(self_);    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(0.0, 0.0, 0.0, 0.0)
    ));
}
fn dualNum_antiConstraintViolation(self_: DualNum) -> Scalar {
    let self_groups = dualNum_grouped(self_);    let geometric_anti_product_groups: DualNumGroups = DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(self_.group0_.x * self_.group0_.y, pow(self_.group0_.y, 2), 0.0, 0.0) * vec2<f32>(2.0, 1.0)
    );
    let geometric_anti_product: DualNum = dualNum_degroup(geometric_anti_product_groups);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(geometric_anti_product_groups.group0_.x, 0.0, 0.0, 0.0)
    ));
}
fn flector_antiConstraintViolation(self_: Flector) -> DualNum {
    let self_groups = flector_grouped(self_);    let anti_reverse_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ self_.group0_ * vec4<f32>(-1.0), 
        /* e423, e431, e412, e321 */ self_.group1_
    );
    let anti_reverse: Flector = flector_degroup(anti_reverse_groups);
    let geometric_anti_product_groups: DualNumGroups = DualNumGroups(
        /* scalar, e1234 */ vec4<f32>((anti_reverse_groups.group1_.w * self_.group0_.w) - (anti_reverse_groups.group0_.x * self_.group1_.x) - (anti_reverse_groups.group0_.y * self_.group1_.y) - (anti_reverse_groups.group0_.z * self_.group1_.z), 0.0, 0.0, 0.0) + (vec4<f32>(vec2<f32>(anti_reverse_groups.group1_.x), 0.0, 0.0) * vec4<f32>(self_.group0_.x, self_.group1_.x, 0.0, 0.0)) + (vec4<f32>(vec2<f32>(anti_reverse_groups.group1_.y), 0.0, 0.0) * vec4<f32>(self_.group0_.y, self_.group1_.y, 0.0, 0.0)) + (vec4<f32>(vec2<f32>(anti_reverse_groups.group1_.z), 0.0, 0.0) * vec4<f32>(self_.group0_.z, self_.group1_.z, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(anti_reverse_groups.group0_.w), 0.0, 0.0) * vec4<f32>(self_.group1_.w, self_.group0_.w, 0.0, 0.0))
    );
    let geometric_anti_product: DualNum = dualNum_degroup(geometric_anti_product_groups);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(self_.group1_.x, 2) + pow(self_.group1_.y, 2) + pow(self_.group1_.z, 2) - pow(self_.group0_.w, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(geometric_anti_product_groups.group0_.x, geometric_anti_product_groups.group0_.y - anti_scalar_product.e1234, 0.0, 0.0)
    ));
}
fn line_antiConstraintViolation(self_: Line) -> DualNum {
    let self_groups = line_grouped(self_);    let anti_reverse_groups: LineGroups = LineGroups(
        /* e41, e42, e43 */ self_.group0_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ self_.group1_ * vec3<f32>(-1.0)
    );
    let anti_reverse: Line = line_degroup(anti_reverse_groups);
    let geometric_anti_product_groups: DualNumGroups = DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(-(anti_reverse_groups.group1_.x * self_.group0_.x) - (anti_reverse_groups.group1_.y * self_.group0_.y) - (anti_reverse_groups.group1_.z * self_.group0_.z), 0.0, 0.0, 0.0) - (vec4<f32>(vec2<f32>(anti_reverse_groups.group0_.x), 0.0, 0.0) * vec4<f32>(self_.group1_.x, self_.group0_.x, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(anti_reverse_groups.group0_.y), 0.0, 0.0) * vec4<f32>(self_.group1_.y, self_.group0_.y, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(anti_reverse_groups.group0_.z), 0.0, 0.0) * vec4<f32>(self_.group1_.z, self_.group0_.z, 0.0, 0.0))
    );
    let geometric_anti_product: DualNum = dualNum_degroup(geometric_anti_product_groups);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(-pow(self_.group0_.x, 2) - pow(self_.group0_.y, 2) - pow(self_.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(geometric_anti_product_groups.group0_.x, geometric_anti_product_groups.group0_.y - anti_scalar_product.e1234, 0.0, 0.0)
    ));
}
fn motor_antiConstraintViolation(self_: Motor) -> DualNum {
    let self_groups = motor_grouped(self_);    let anti_reverse_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.group0_.x * -1.0, self_.group0_.y * -1.0, self_.group0_.z * -1.0, self_.group0_.w), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.group1_.x * -1.0, self_.group1_.y * -1.0, self_.group1_.z * -1.0, self_.group1_.w)
    );
    let anti_reverse: Motor = motor_degroup(anti_reverse_groups);
    let geometric_anti_product_groups: DualNumGroups = DualNumGroups(
        /* scalar, e1234 */ vec4<f32>((anti_reverse_groups.group1_.w * self_.group0_.w) - (anti_reverse_groups.group1_.x * self_.group0_.x) - (anti_reverse_groups.group1_.y * self_.group0_.y) - (anti_reverse_groups.group1_.z * self_.group0_.z), 0.0, 0.0, 0.0) + (vec4<f32>(vec2<f32>(anti_reverse_groups.group0_.w), 0.0, 0.0) * vec4<f32>(self_.group1_.w, self_.group0_.w, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(anti_reverse_groups.group0_.x), 0.0, 0.0) * vec4<f32>(self_.group1_.x, self_.group0_.x, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(anti_reverse_groups.group0_.y), 0.0, 0.0) * vec4<f32>(self_.group1_.y, self_.group0_.y, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(anti_reverse_groups.group0_.z), 0.0, 0.0) * vec4<f32>(self_.group1_.z, self_.group0_.z, 0.0, 0.0))
    );
    let geometric_anti_product: DualNum = dualNum_degroup(geometric_anti_product_groups);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(self_.group0_.w, 2) - pow(self_.group0_.x, 2) - pow(self_.group0_.y, 2) - pow(self_.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(geometric_anti_product_groups.group0_.x, geometric_anti_product_groups.group0_.y - anti_scalar_product.e1234, 0.0, 0.0)
    ));
}
fn multiVector_antiConstraintViolation(self_: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);    let anti_reverse_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ self_.group0_, 
        /* e1, e2, e3, e4 */ self_.group1_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ self_.group2_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ self_.group3_ * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ self_.group4_
    );
    let anti_reverse: MultiVector = multiVector_degroup(anti_reverse_groups);
    let geometric_anti_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((anti_reverse_groups.group0_.y * self_.group0_.x) + (anti_reverse_groups.group4_.w * self_.group1_.w) - (anti_reverse_groups.group3_.x * self_.group2_.x) - (anti_reverse_groups.group3_.y * self_.group2_.y) - (anti_reverse_groups.group3_.z * self_.group2_.z) - (anti_reverse_groups.group1_.x * self_.group4_.x) - (anti_reverse_groups.group1_.y * self_.group4_.y) - (anti_reverse_groups.group1_.z * self_.group4_.z), 0.0, 0.0, 0.0) + (vec4<f32>(vec2<f32>(self_.group0_.y), 0.0, 0.0) * anti_reverse_groups.group0_) + (vec4<f32>(vec2<f32>(anti_reverse_groups.group4_.x), 0.0, 0.0) * vec4<f32>(self_.group1_.x, self_.group4_.x, 0.0, 0.0)) + (vec4<f32>(vec2<f32>(anti_reverse_groups.group4_.y), 0.0, 0.0) * vec4<f32>(self_.group1_.y, self_.group4_.y, 0.0, 0.0)) + (vec4<f32>(vec2<f32>(anti_reverse_groups.group4_.z), 0.0, 0.0) * vec4<f32>(self_.group1_.z, self_.group4_.z, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(anti_reverse_groups.group2_.x), 0.0, 0.0) * vec4<f32>(self_.group3_.x, self_.group2_.x, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(anti_reverse_groups.group2_.y), 0.0, 0.0) * vec4<f32>(self_.group3_.y, self_.group2_.y, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(anti_reverse_groups.group2_.z), 0.0, 0.0) * vec4<f32>(self_.group3_.z, self_.group2_.z, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(anti_reverse_groups.group1_.w), 0.0, 0.0) * vec4<f32>(self_.group4_.w, self_.group1_.w, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>((anti_reverse_groups.group2_.x * self_.group1_.w) + (self_.group2_.y * anti_reverse_groups.group4_.z) - (anti_reverse_groups.group2_.y * self_.group4_.z), (anti_reverse_groups.group2_.y * self_.group1_.w) + (self_.group2_.z * anti_reverse_groups.group4_.x) - (anti_reverse_groups.group2_.z * self_.group4_.x), (anti_reverse_groups.group2_.z * self_.group1_.w) + (self_.group2_.x * anti_reverse_groups.group4_.y) - (anti_reverse_groups.group2_.x * self_.group4_.y), (anti_reverse_groups.group3_.y * self_.group4_.y) + (anti_reverse_groups.group3_.z * self_.group4_.z) - (anti_reverse_groups.group0_.x * self_.group1_.w) - (anti_reverse_groups.group2_.x * self_.group1_.x) - (anti_reverse_groups.group2_.y * self_.group1_.y) - (anti_reverse_groups.group2_.z * self_.group1_.z) - (self_.group2_.x * anti_reverse_groups.group1_.x) - (self_.group2_.y * anti_reverse_groups.group1_.y) - (self_.group2_.z * anti_reverse_groups.group1_.z) - (self_.group3_.y * anti_reverse_groups.group4_.y) - (self_.group3_.z * anti_reverse_groups.group4_.z)) + (vec4<f32>(anti_reverse_groups.group0_.y) * self_.group4_) + (vec4<f32>(self_.group0_.y) * anti_reverse_groups.group4_) + (vec4<f32>(anti_reverse_groups.group1_.w) * vec4<f32>(self_.group2_.xyz.xyz, self_.group0_.x)) + (vec4<f32>(anti_reverse_groups.group2_.zxy.xyz, anti_reverse_groups.group3_.x) * self_.group4_.yzxx) - (vec4<f32>(self_.group2_.zxy.xyz, self_.group3_.x) * anti_reverse_groups.group4_.yzxx)
    );
    let geometric_anti_product: MultiVector = multiVector_degroup(geometric_anti_product_groups);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(self_.group0_.y, 2) + pow(self_.group4_.x, 2) + pow(self_.group4_.y, 2) + pow(self_.group4_.z, 2) - pow(self_.group2_.x, 2) - pow(self_.group2_.y, 2) - pow(self_.group2_.z, 2) - pow(self_.group1_.w, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(geometric_anti_product_groups.group0_.x, geometric_anti_product_groups.group0_.y - anti_scalar_product.e1234, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ geometric_anti_product_groups.group4_
    ));
}
fn origin_antiConstraintViolation(self_: Origin) -> AntiScalar {
    let self_groups = origin_grouped(self_);    let anti_reverse_groups: OriginGroups = OriginGroups(
        /* e4 */ vec4<f32>(self.e4 * -1.0, 0.0, 0.0, 0.0)
    );
    let anti_reverse: Origin = origin_degroup(anti_reverse_groups);
    let geometric_anti_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(anti_reverse.e4 * self.e4 * -1.0, 0.0, 0.0, 0.0)
    );
    let geometric_anti_product: AntiScalar = antiScalar_degroup(geometric_anti_product_groups);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(self.e4, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(geometric_anti_product.e1234 - anti_scalar_product.e1234, 0.0, 0.0, 0.0)
    ));
}
fn plane_antiConstraintViolation(self_: Plane) -> Scalar {
    let self_groups = plane_grouped(self_);    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(0.0, 0.0, 0.0, 0.0)
    ));
}
fn point_antiConstraintViolation(self_: Point) -> AntiScalar {
    let self_groups = point_grouped(self_);    let anti_reverse_groups: PointGroups = PointGroups(
        /* e1, e2, e3, e4 */ self_.group0_ * vec4<f32>(-1.0)
    );
    let anti_reverse: Point = point_degroup(anti_reverse_groups);
    let geometric_anti_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(anti_reverse_groups.group0_.w * self_.group0_.w * -1.0, 0.0, 0.0, 0.0)
    );
    let geometric_anti_product: AntiScalar = antiScalar_degroup(geometric_anti_product_groups);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(self_.group0_.w, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(geometric_anti_product.e1234 - anti_scalar_product.e1234, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_antiFix(self_: AntiScalar) -> AntiScalar {
    let self_groups = antiScalar_grouped(self_);    let anti_reverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(self.e1234, 0.0, 0.0, 0.0)
    );
    let anti_reverse: AntiScalar = antiScalar_degroup(anti_reverse_groups);
    let geometric_anti_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(anti_reverse.e1234 * self.e1234, 0.0, 0.0, 0.0)
    );
    let geometric_anti_product: AntiScalar = antiScalar_degroup(geometric_anti_product_groups);
    let anti_square_root_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(geometric_anti_product.e1234, 0.5), 0.0, 0.0, 0.0)
    );
    let anti_square_root: AntiScalar = antiScalar_degroup(anti_square_root_groups);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(anti_square_root.e1234, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(anti_inverse.e1234 * self.e1234, 0.0, 0.0, 0.0)
    ));
}
fn origin_antiFix(self_: Origin) -> Origin {
    let self_groups = origin_grouped(self_);    let anti_reverse_groups: OriginGroups = OriginGroups(
        /* e4 */ vec4<f32>(self.e4 * -1.0, 0.0, 0.0, 0.0)
    );
    let anti_reverse: Origin = origin_degroup(anti_reverse_groups);
    let geometric_anti_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(anti_reverse.e4 * self.e4 * -1.0, 0.0, 0.0, 0.0)
    );
    let geometric_anti_product: AntiScalar = antiScalar_degroup(geometric_anti_product_groups);
    let anti_square_root_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(geometric_anti_product.e1234, 0.5), 0.0, 0.0, 0.0)
    );
    let anti_square_root: AntiScalar = antiScalar_degroup(anti_square_root_groups);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(anti_square_root.e1234, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(anti_inverse.e1234 * self.e4, 0.0, 0.0, 0.0)
    ));
}
fn plane_antiFix(self_: Plane) -> Plane {
    let self_groups = plane_grouped(self_);    let geometric_anti_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(self_.group0_.x, 2) + pow(self_.group0_.y, 2) + pow(self_.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let geometric_anti_product: AntiScalar = antiScalar_degroup(geometric_anti_product_groups);
    let anti_square_root_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(geometric_anti_product.e1234, 0.5), 0.0, 0.0, 0.0)
    );
    let anti_square_root: AntiScalar = antiScalar_degroup(anti_square_root_groups);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(anti_square_root.e1234, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234) * self_.group0_
    ));
}
fn point_antiFix(self_: Point) -> Point {
    let self_groups = point_grouped(self_);    let anti_reverse_groups: PointGroups = PointGroups(
        /* e1, e2, e3, e4 */ self_.group0_ * vec4<f32>(-1.0)
    );
    let anti_reverse: Point = point_degroup(anti_reverse_groups);
    let geometric_anti_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(anti_reverse_groups.group0_.w * self_.group0_.w * -1.0, 0.0, 0.0, 0.0)
    );
    let geometric_anti_product: AntiScalar = antiScalar_degroup(geometric_anti_product_groups);
    let anti_square_root_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(geometric_anti_product.e1234, 0.5), 0.0, 0.0, 0.0)
    );
    let anti_square_root: AntiScalar = antiScalar_degroup(anti_square_root_groups);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(anti_square_root.e1234, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234) * self_.group0_
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
    let self_groups = antiScalar_grouped(self_);    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(self.e1234, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    ));
}
fn dualNum_antiInverse(self_: DualNum) -> AntiScalar {
    let self_groups = dualNum_grouped(self_);    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(self_.group0_.y, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    ));
}
fn flector_antiInverse(self_: Flector) -> AntiScalar {
    let self_groups = flector_grouped(self_);    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(self_.group1_.x, 2) + pow(self_.group1_.y, 2) + pow(self_.group1_.z, 2) - pow(self_.group0_.w, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    ));
}
fn line_antiInverse(self_: Line) -> AntiScalar {
    let self_groups = line_grouped(self_);    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(-pow(self_.group0_.x, 2) - pow(self_.group0_.y, 2) - pow(self_.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    ));
}
fn motor_antiInverse(self_: Motor) -> AntiScalar {
    let self_groups = motor_grouped(self_);    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(self_.group0_.w, 2) - pow(self_.group0_.x, 2) - pow(self_.group0_.y, 2) - pow(self_.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    ));
}
fn multiVector_antiInverse(self_: MultiVector) -> AntiScalar {
    let self_groups = multiVector_grouped(self_);    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(self_.group0_.y, 2) + pow(self_.group4_.x, 2) + pow(self_.group4_.y, 2) + pow(self_.group4_.z, 2) - pow(self_.group2_.x, 2) - pow(self_.group2_.y, 2) - pow(self_.group2_.z, 2) - pow(self_.group1_.w, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    ));
}
fn origin_antiInverse(self_: Origin) -> AntiScalar {
    let self_groups = origin_grouped(self_);    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(self.e4, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    ));
}
fn plane_antiInverse(self_: Plane) -> AntiScalar {
    let self_groups = plane_grouped(self_);    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(self_.group0_.x, 2) + pow(self_.group0_.y, 2) + pow(self_.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    ));
}
fn point_antiInverse(self_: Point) -> AntiScalar {
    let self_groups = point_grouped(self_);    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(self_.group0_.w, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_antiOne() -> AntiScalar {
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0, 0.0, 0.0, 0.0)
    ));
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
    let self_groups = antiScalar_grouped(self_);    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(self.e1234, 0.0, 0.0, 0.0)
    ));
}
fn dualNum_antiReverse(self_: DualNum) -> DualNum {
    let self_groups = dualNum_grouped(self_);    return self;
}
fn flector_antiReverse(self_: Flector) -> Flector {
    let self_groups = flector_grouped(self_);    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ self_.group0_ * vec4<f32>(-1.0), 
        /* e423, e431, e412, e321 */ self_.group1_
    ));
}
fn horizon_antiReverse(self_: Horizon) -> Horizon {
    let self_groups = horizon_grouped(self_);    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(self.e321, 0.0, 0.0, 0.0)
    ));
}
fn line_antiReverse(self_: Line) -> Line {
    let self_groups = line_grouped(self_);    return line_degroup(LineGroups(
        /* e41, e42, e43 */ self_.group0_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ self_.group1_ * vec3<f32>(-1.0)
    ));
}
fn motor_antiReverse(self_: Motor) -> Motor {
    let self_groups = motor_grouped(self_);    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.group0_.x * -1.0, self_.group0_.y * -1.0, self_.group0_.z * -1.0, self_.group0_.w), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.group1_.x * -1.0, self_.group1_.y * -1.0, self_.group1_.z * -1.0, self_.group1_.w)
    ));
}
fn multiVector_antiReverse(self_: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_.group0_, 
        /* e1, e2, e3, e4 */ self_.group1_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ self_.group2_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ self_.group3_ * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ self_.group4_
    ));
}
fn origin_antiReverse(self_: Origin) -> Origin {
    let self_groups = origin_grouped(self_);    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(self.e4 * -1.0, 0.0, 0.0, 0.0)
    ));
}
fn plane_antiReverse(self_: Plane) -> Plane {
    let self_groups = plane_grouped(self_);    return self;
}
fn point_antiReverse(self_: Point) -> Point {
    let self_groups = point_grouped(self_);    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ self_.group0_ * vec4<f32>(-1.0)
    ));
}
fn scalar_antiReverse(self_: Scalar) -> Scalar {
    let self_groups = scalar_grouped(self_);    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(self.scalar, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_antiSquareRoot(self_: AntiScalar) -> AntiScalar {
    let self_groups = antiScalar_grouped(self_);    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(self.e1234, 0.5), 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_antiWedge_antiScalar(self_: AntiScalar, other: AntiScalar) -> AntiScalar {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(other.e1234 * self.e1234, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_antiWedge_dualNum(self_: AntiScalar, other: DualNum) -> DualNum {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(self.e1234), 0.0, 0.0) * other_groups.group0_
    ));
}
fn antiScalar_antiWedge_flector(self_: AntiScalar, other: Flector) -> Flector {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self.e1234) * other_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self.e1234) * other_groups.group1_
    ));
}
fn antiScalar_antiWedge_horizon(self_: AntiScalar, other: Horizon) -> Horizon {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = horizon_grouped(other);
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(self.e1234 * other.e321, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_antiWedge_line(self_: AntiScalar, other: Line) -> Line {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = line_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(self.e1234), 0.0) * other_groups.group0_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(self.e1234), 0.0) * other_groups.group1_
    ));
}
fn antiScalar_antiWedge_motor(self_: AntiScalar, other: Motor) -> Motor {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self.e1234) * other_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(self.e1234) * other_groups.group1_
    ));
}
fn antiScalar_antiWedge_multiVector(self_: AntiScalar, other: MultiVector) -> MultiVector {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(self.e1234), 0.0, 0.0) * other_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(self.e1234) * other_groups.group1_, 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(self.e1234), 0.0) * other_groups.group2_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(self.e1234), 0.0) * other_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self.e1234) * other_groups.group4_
    ));
}
fn antiScalar_antiWedge_origin(self_: AntiScalar, other: Origin) -> Origin {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = origin_grouped(other);
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(self.e1234 * other.e4, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_antiWedge_plane(self_: AntiScalar, other: Plane) -> Plane {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = plane_grouped(other);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(self.e1234) * other_groups.group0_
    ));
}
fn antiScalar_antiWedge_point(self_: AntiScalar, other: Point) -> Point {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = point_grouped(other);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self.e1234) * other_groups.group0_
    ));
}
fn antiScalar_antiWedge_scalar(self_: AntiScalar, other: Scalar) -> Scalar {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = scalar_grouped(other);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(self.e1234 * other.scalar, 0.0, 0.0, 0.0)
    ));
}
fn dualNum_antiWedge_antiScalar(self_: DualNum, other: AntiScalar) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(other.e1234), 0.0, 0.0) * self_.group0_
    ));
}
fn dualNum_antiWedge_dualNum(self_: DualNum, other: DualNum) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>((other_groups.group0_.x * self_.group0_.y) + (other_groups.group0_.y * self_.group0_.x), other_groups.group0_.y * self_.group0_.y, 0.0, 0.0)
    ));
}
fn dualNum_antiWedge_flector(self_: DualNum, other: Flector) -> Flector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group0_.y) * other_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.y) * other_groups.group1_
    ));
}
fn dualNum_antiWedge_horizon(self_: DualNum, other: Horizon) -> Horizon {
    let self_groups = dualNum_grouped(self_);
    let other_groups = horizon_grouped(other);
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(self_.group0_.y * other.e321, 0.0, 0.0, 0.0)
    ));
}
fn dualNum_antiWedge_line(self_: DualNum, other: Line) -> Line {
    let self_groups = dualNum_grouped(self_);
    let other_groups = line_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(self_.group0_.y), 0.0) * other_groups.group0_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(self_.group0_.y), 0.0) * other_groups.group1_
    ));
}
fn dualNum_antiWedge_motor(self_: DualNum, other: Motor) -> Motor {
    let self_groups = dualNum_grouped(self_);
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.group0_.y) * other_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.group0_.y * other_groups.group1_.x, self_.group0_.y * other_groups.group1_.y, self_.group0_.y * other_groups.group1_.z, (self_.group0_.x * other_groups.group0_.w) + (self_.group0_.y * other_groups.group1_.w))
    ));
}
fn dualNum_antiWedge_multiVector(self_: DualNum, other: MultiVector) -> MultiVector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((self_.group0_.x * other_groups.group0_.y) + (self_.group0_.y * other_groups.group0_.x), self_.group0_.y * other_groups.group0_.y, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group0_.y) * other_groups.group1_, 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(self_.group0_.y), 0.0) * other_groups.group2_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(self_.group0_.y), 0.0) * other_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.y) * other_groups.group4_
    ));
}
fn dualNum_antiWedge_origin(self_: DualNum, other: Origin) -> Origin {
    let self_groups = dualNum_grouped(self_);
    let other_groups = origin_grouped(other);
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(self_.group0_.y * other.e4, 0.0, 0.0, 0.0)
    ));
}
fn dualNum_antiWedge_plane(self_: DualNum, other: Plane) -> Plane {
    let self_groups = dualNum_grouped(self_);
    let other_groups = plane_grouped(other);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.y) * other_groups.group0_
    ));
}
fn dualNum_antiWedge_point(self_: DualNum, other: Point) -> Point {
    let self_groups = dualNum_grouped(self_);
    let other_groups = point_grouped(other);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group0_.y) * other_groups.group0_
    ));
}
fn dualNum_antiWedge_scalar(self_: DualNum, other: Scalar) -> Scalar {
    let self_groups = dualNum_grouped(self_);
    let other_groups = scalar_grouped(other);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(self_.group0_.y * other.scalar, 0.0, 0.0, 0.0)
    ));
}
fn flector_antiWedge_antiScalar(self_: Flector, other: AntiScalar) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e1234) * self_.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e1234) * self_.group1_
    ));
}
fn flector_antiWedge_dualNum(self_: Flector, other: DualNum) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group0_.y) * self_.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group0_.y) * self_.group1_
    ));
}
fn flector_antiWedge_flector(self_: Flector, other: Flector) -> Motor {
    let self_groups = flector_grouped(self_);
    let other_groups = flector_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other_groups.group1_.y * self_.group1_.z) - (other_groups.group1_.z * self_.group1_.y), (other_groups.group1_.z * self_.group1_.x) - (other_groups.group1_.x * self_.group1_.z), (other_groups.group1_.x * self_.group1_.y) - (other_groups.group1_.y * self_.group1_.x), 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, (other_groups.group1_.y * self_.group0_.y) + (other_groups.group1_.z * self_.group0_.z) + (other_groups.group1_.w * self_.group0_.w) - (other_groups.group0_.y * self_.group1_.y) - (other_groups.group0_.z * self_.group1_.z) - (other_groups.group0_.w * self_.group1_.w)) + (vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group0_.x) * other_groups.group1_.wwwx) - (vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, other_groups.group0_.x) * self_.group1_.wwwx)
    ));
}
fn flector_antiWedge_horizon(self_: Flector, other: Horizon) -> Motor {
    let self_groups = flector_grouped(self_);
    let other_groups = horizon_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(other.e321) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group0_.w)
    ));
}
fn flector_antiWedge_line(self_: Flector, other: Line) -> Point {
    let self_groups = flector_grouped(self_);
    let other_groups = line_grouped(other);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group0_.x * self_.group1_.w) + (other_groups.group1_.y * self_.group1_.z), (other_groups.group0_.y * self_.group1_.w) + (other_groups.group1_.z * self_.group1_.x), (other_groups.group0_.z * self_.group1_.w) + (other_groups.group1_.x * self_.group1_.y), -(other_groups.group0_.y * self_.group1_.y) - (other_groups.group0_.z * self_.group1_.z)) - (vec4<f32>(other_groups.group1_.zxy.xyz, other_groups.group0_.x) * self_.group1_.yzxx)
    ));
}
fn flector_antiWedge_motor(self_: Flector, other: Motor) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = motor_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group1_.z * other_groups.group1_.y) + (self_.group1_.w * other_groups.group0_.x), (self_.group1_.x * other_groups.group1_.z) + (self_.group1_.w * other_groups.group0_.y), (self_.group1_.y * other_groups.group1_.x) + (self_.group1_.w * other_groups.group0_.z), -(self_.group1_.y * other_groups.group0_.y) - (self_.group1_.z * other_groups.group0_.z)) + (vec4<f32>(other_groups.group0_.w) * self_.group0_) - (vec4<f32>(other_groups.group1_.z, other_groups.group1_.x, other_groups.group1_.y, other_groups.group0_.x) * self_.group1_.yzxx), 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group0_.w) * self_.group1_
    ));
}
fn flector_antiWedge_multiVector(self_: Flector, other: MultiVector) -> MultiVector {
    let self_groups = flector_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((self_.group0_.x * other_groups.group4_.x) + (self_.group0_.y * other_groups.group4_.y) + (self_.group0_.z * other_groups.group4_.z) + (self_.group0_.w * other_groups.group4_.w) - (self_.group1_.x * other_groups.group1_.x) - (self_.group1_.y * other_groups.group1_.y) - (self_.group1_.z * other_groups.group1_.z) - (self_.group1_.w * other_groups.group1_.w), 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group2_.x * self_.group1_.w) + (other_groups.group3_.y * self_.group1_.z), (other_groups.group2_.y * self_.group1_.w) + (other_groups.group3_.z * self_.group1_.x), (other_groups.group2_.z * self_.group1_.w) + (other_groups.group3_.x * self_.group1_.y), -(other_groups.group2_.y * self_.group1_.y) - (other_groups.group2_.z * self_.group1_.z)) + (vec4<f32>(other_groups.group0_.y) * self_.group0_) - (vec4<f32>(other_groups.group3_.zxy.xyz, other_groups.group2_.x) * self_.group1_.yzxx), 
        /* e41, e42, e43 */ vec4<f32>((self_.group1_.z * other_groups.group4_.y) - (self_.group1_.y * other_groups.group4_.z), (self_.group1_.x * other_groups.group4_.z) - (self_.group1_.z * other_groups.group4_.x), (self_.group1_.y * other_groups.group4_.x) - (self_.group1_.x * other_groups.group4_.y), 0.0), 
        /* e23, e31, e12 */ (vec4<f32>(vec3<f32>(other_groups.group4_.w), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0)) - (vec4<f32>(vec3<f32>(self_.group1_.w), 0.0) * vec4<f32>(other_groups.group4_.x, other_groups.group4_.y, other_groups.group4_.z, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group0_.y) * self_.group1_
    ));
}
fn flector_antiWedge_origin(self_: Flector, other: Origin) -> Scalar {
    let self_groups = flector_grouped(self_);
    let other_groups = origin_grouped(other);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(self_.group1_.w * other.e4 * -1.0, 0.0, 0.0, 0.0)
    ));
}
fn flector_antiWedge_plane(self_: Flector, other: Plane) -> Motor {
    let self_groups = flector_grouped(self_);
    let other_groups = plane_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((self_.group1_.z * other_groups.group0_.y) - (self_.group1_.y * other_groups.group0_.z), (self_.group1_.x * other_groups.group0_.z) - (self_.group1_.z * other_groups.group0_.x), (self_.group1_.y * other_groups.group0_.x) - (self_.group1_.x * other_groups.group0_.y), 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>((self_.group1_.w * other_groups.group0_.x) * -1.0, (self_.group1_.w * other_groups.group0_.y) * -1.0, (self_.group1_.w * other_groups.group0_.z) * -1.0, (self_.group0_.y * other_groups.group0_.y) + (self_.group0_.z * other_groups.group0_.z) + (self_.group0_.w * other_groups.group0_.w)) + (vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group0_.x) * other_groups.group0_.wwwx)
    ));
}
fn flector_antiWedge_point(self_: Flector, other: Point) -> Scalar {
    let self_groups = flector_grouped(self_);
    let other_groups = point_grouped(other);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(-(self_.group1_.x * other_groups.group0_.x) - (self_.group1_.y * other_groups.group0_.y) - (self_.group1_.z * other_groups.group0_.z) - (self_.group1_.w * other_groups.group0_.w), 0.0, 0.0, 0.0)
    ));
}
fn horizon_antiWedge_antiScalar(self_: Horizon, other: AntiScalar) -> Horizon {
    let self_groups = horizon_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(other.e1234 * self.e321, 0.0, 0.0, 0.0)
    ));
}
fn horizon_antiWedge_dualNum(self_: Horizon, other: DualNum) -> Horizon {
    let self_groups = horizon_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(other_groups.group0_.y * self.e321, 0.0, 0.0, 0.0)
    ));
}
fn horizon_antiWedge_flector(self_: Horizon, other: Flector) -> Motor {
    let self_groups = horizon_grouped(self_);
    let other_groups = flector_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(self.e321) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, other_groups.group0_.w) * vec4<f32>(-1.0)
    ));
}
fn horizon_antiWedge_line(self_: Horizon, other: Line) -> Point {
    let self_groups = horizon_grouped(self_);
    let other_groups = line_grouped(other);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group0_.x * self.e321, other_groups.group0_.y * self.e321, other_groups.group0_.z * self.e321, 0.0)
    ));
}
fn horizon_antiWedge_motor(self_: Horizon, other: Motor) -> Flector {
    let self_groups = horizon_grouped(self_);
    let other_groups = motor_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group0_.x * self.e321, other_groups.group0_.y * self.e321, other_groups.group0_.z * self.e321, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group0_.w * self.e321)
    ));
}
fn horizon_antiWedge_multiVector(self_: Horizon, other: MultiVector) -> MultiVector {
    let self_groups = horizon_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other_groups.group1_.w * self.e321 * -1.0, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group2_.x * self.e321, other_groups.group2_.y * self.e321, other_groups.group2_.z * self.e321, 0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(self.e321), 0.0) * vec4<f32>(other_groups.group4_.x, other_groups.group4_.y, other_groups.group4_.z, 0.0) * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group0_.y * self.e321)
    ));
}
fn horizon_antiWedge_origin(self_: Horizon, other: Origin) -> Scalar {
    let self_groups = horizon_grouped(self_);
    let other_groups = origin_grouped(other);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(self.e321 * other.e4 * -1.0, 0.0, 0.0, 0.0)
    ));
}
fn horizon_antiWedge_plane(self_: Horizon, other: Plane) -> Line {
    let self_groups = horizon_grouped(self_);
    let other_groups = plane_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(self.e321), 0.0) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0) * vec3<f32>(-1.0)
    ));
}
fn horizon_antiWedge_point(self_: Horizon, other: Point) -> Scalar {
    let self_groups = horizon_grouped(self_);
    let other_groups = point_grouped(other);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(other_groups.group0_.w * self.e321 * -1.0, 0.0, 0.0, 0.0)
    ));
}
fn line_antiWedge_antiScalar(self_: Line, other: AntiScalar) -> Line {
    let self_groups = line_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(other.e1234), 0.0) * self_.group0_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(other.e1234), 0.0) * self_.group1_
    ));
}
fn line_antiWedge_dualNum(self_: Line, other: DualNum) -> Line {
    let self_groups = line_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(other_groups.group0_.y), 0.0) * self_.group0_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(other_groups.group0_.y), 0.0) * self_.group1_
    ));
}
fn line_antiWedge_flector(self_: Line, other: Flector) -> Point {
    let self_groups = line_grouped(self_);
    let other_groups = flector_grouped(other);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group0_.x * other_groups.group1_.w) + (self_.group1_.y * other_groups.group1_.z), (self_.group0_.y * other_groups.group1_.w) + (self_.group1_.z * other_groups.group1_.x), (self_.group0_.z * other_groups.group1_.w) + (self_.group1_.x * other_groups.group1_.y), -(self_.group0_.y * other_groups.group1_.y) - (self_.group0_.z * other_groups.group1_.z)) - (vec4<f32>(self_.group1_.zxy.xyz, self_.group0_.x) * other_groups.group1_.yzxx)
    ));
}
fn line_antiWedge_horizon(self_: Line, other: Horizon) -> Point {
    let self_groups = line_grouped(self_);
    let other_groups = horizon_grouped(other);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group0_.x * other.e321, self_.group0_.y * other.e321, self_.group0_.z * other.e321, 0.0)
    ));
}
fn line_antiWedge_line(self_: Line, other: Line) -> Scalar {
    let self_groups = line_grouped(self_);
    let other_groups = line_grouped(other);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(-(other_groups.group0_.x * self_.group1_.x) - (other_groups.group0_.y * self_.group1_.y) - (other_groups.group0_.z * self_.group1_.z) - (other_groups.group1_.x * self_.group0_.x) - (other_groups.group1_.y * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.z), 0.0, 0.0, 0.0)
    ));
}
fn line_antiWedge_motor(self_: Line, other: Motor) -> Motor {
    let self_groups = line_grouped(self_);
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.group0_.x * other_groups.group0_.w, self_.group0_.y * other_groups.group0_.w, self_.group0_.z * other_groups.group0_.w, 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.group1_.x * other_groups.group0_.w, self_.group1_.y * other_groups.group0_.w, self_.group1_.z * other_groups.group0_.w, -(self_.group0_.x * other_groups.group1_.x) - (self_.group0_.y * other_groups.group1_.y) - (self_.group0_.z * other_groups.group1_.z) - (self_.group1_.x * other_groups.group0_.x) - (self_.group1_.y * other_groups.group0_.y) - (self_.group1_.z * other_groups.group0_.z))
    ));
}
fn line_antiWedge_multiVector(self_: Line, other: MultiVector) -> MultiVector {
    let self_groups = line_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(-(self_.group0_.x * other_groups.group3_.x) - (self_.group0_.y * other_groups.group3_.y) - (self_.group0_.z * other_groups.group3_.z) - (self_.group1_.x * other_groups.group2_.x) - (self_.group1_.y * other_groups.group2_.y) - (self_.group1_.z * other_groups.group2_.z), 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group0_.x * other_groups.group4_.w) + (self_.group1_.y * other_groups.group4_.z), (self_.group0_.y * other_groups.group4_.w) + (self_.group1_.z * other_groups.group4_.x), (self_.group0_.z * other_groups.group4_.w) + (self_.group1_.x * other_groups.group4_.y), -(self_.group0_.y * other_groups.group4_.y) - (self_.group0_.z * other_groups.group4_.z)) - (vec4<f32>(self_.group1_.zxy.xyz, self_.group0_.x) * other_groups.group4_.yzxx), 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(other_groups.group0_.y), 0.0) * self_.group0_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(other_groups.group0_.y), 0.0) * self_.group1_, 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn line_antiWedge_plane(self_: Line, other: Plane) -> Point {
    let self_groups = line_grouped(self_);
    let other_groups = plane_grouped(other);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group0_.x * other_groups.group0_.w) + (self_.group1_.y * other_groups.group0_.z), (self_.group0_.y * other_groups.group0_.w) + (self_.group1_.z * other_groups.group0_.x), (self_.group0_.z * other_groups.group0_.w) + (self_.group1_.x * other_groups.group0_.y), -(self_.group0_.y * other_groups.group0_.y) - (self_.group0_.z * other_groups.group0_.z)) - (vec4<f32>(self_.group1_.zxy.xyz, self_.group0_.x) * other_groups.group0_.yzxx)
    ));
}
fn motor_antiWedge_antiScalar(self_: Motor, other: AntiScalar) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e1234) * self_.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(other.e1234) * self_.group1_
    ));
}
fn motor_antiWedge_dualNum(self_: Motor, other: DualNum) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other_groups.group0_.y) * self_.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(other_groups.group0_.y * self_.group1_.x, other_groups.group0_.y * self_.group1_.y, other_groups.group0_.y * self_.group1_.z, (other_groups.group0_.x * self_.group0_.w) + (other_groups.group0_.y * self_.group1_.w))
    ));
}
fn motor_antiWedge_flector(self_: Motor, other: Flector) -> Flector {
    let self_groups = motor_grouped(self_);
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group1_.z * self_.group1_.y) + (other_groups.group1_.w * self_.group0_.x), (other_groups.group1_.x * self_.group1_.z) + (other_groups.group1_.w * self_.group0_.y), (other_groups.group1_.y * self_.group1_.x) + (other_groups.group1_.w * self_.group0_.z), -(other_groups.group1_.y * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.z)) + (vec4<f32>(self_.group0_.w) * other_groups.group0_) - (vec4<f32>(self_.group1_.z, self_.group1_.x, self_.group1_.y, self_.group0_.x) * other_groups.group1_.yzxx), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.w) * other_groups.group1_
    ));
}
fn motor_antiWedge_horizon(self_: Motor, other: Horizon) -> Flector {
    let self_groups = motor_grouped(self_);
    let other_groups = horizon_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group0_.x * other.e321, self_.group0_.y * other.e321, self_.group0_.z * other.e321, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.group0_.w * other.e321)
    ));
}
fn motor_antiWedge_line(self_: Motor, other: Line) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = line_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other_groups.group0_.x * self_.group0_.w, other_groups.group0_.y * self_.group0_.w, other_groups.group0_.z * self_.group0_.w, 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(other_groups.group1_.x * self_.group0_.w, other_groups.group1_.y * self_.group0_.w, other_groups.group1_.z * self_.group0_.w, -(other_groups.group0_.x * self_.group1_.x) - (other_groups.group0_.y * self_.group1_.y) - (other_groups.group0_.z * self_.group1_.z) - (other_groups.group1_.x * self_.group0_.x) - (other_groups.group1_.y * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.z))
    ));
}
fn motor_antiWedge_motor(self_: Motor, other: Motor) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other_groups.group0_.x * self_.group0_.w) + (other_groups.group0_.w * self_.group0_.x), (other_groups.group0_.y * self_.group0_.w) + (other_groups.group0_.w * self_.group0_.y), (other_groups.group0_.z * self_.group0_.w) + (other_groups.group0_.w * self_.group0_.z), other_groups.group0_.w * self_.group0_.w), 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, -(other_groups.group0_.x * self_.group1_.x) - (other_groups.group0_.y * self_.group1_.y) - (other_groups.group0_.z * self_.group1_.z) - (other_groups.group1_.x * self_.group0_.x) - (other_groups.group1_.y * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.z)) + (vec4<f32>(other_groups.group0_.w) * self_.group1_) + (vec4<f32>(self_.group0_.w) * other_groups.group1_)
    ));
}
fn motor_antiWedge_multiVector(self_: Motor, other: MultiVector) -> MultiVector {
    let self_groups = motor_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((other_groups.group0_.x * self_.group0_.w) + (other_groups.group0_.y * self_.group1_.w) - (other_groups.group2_.x * self_.group1_.x) - (other_groups.group2_.y * self_.group1_.y) - (other_groups.group2_.z * self_.group1_.z) - (other_groups.group3_.x * self_.group0_.x) - (other_groups.group3_.y * self_.group0_.y) - (other_groups.group3_.z * self_.group0_.z), other_groups.group0_.y * self_.group0_.w, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group0_.w * other_groups.group1_.x) + (self_.group1_.y * other_groups.group4_.z), (self_.group0_.w * other_groups.group1_.y) + (self_.group1_.z * other_groups.group4_.x), (self_.group0_.w * other_groups.group1_.z) + (self_.group1_.x * other_groups.group4_.y), -(self_.group0_.y * other_groups.group4_.y) - (self_.group0_.z * other_groups.group4_.z)) + (vec4<f32>(other_groups.group4_.w, other_groups.group4_.w, other_groups.group4_.w, other_groups.group1_.w) * self_.group0_) - (vec4<f32>(self_.group1_.z, self_.group1_.x, self_.group1_.y, self_.group0_.x) * other_groups.group4_.yzxx), 
        /* e41, e42, e43 */ (vec4<f32>(vec3<f32>(other_groups.group0_.y), 0.0) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0)) + (vec4<f32>(vec3<f32>(self_.group0_.w), 0.0) * other_groups.group2_), 
        /* e23, e31, e12 */ (vec4<f32>(vec3<f32>(other_groups.group0_.y), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0)) + (vec4<f32>(vec3<f32>(self_.group0_.w), 0.0) * other_groups.group3_), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.w) * other_groups.group4_
    ));
}
fn motor_antiWedge_origin(self_: Motor, other: Origin) -> Origin {
    let self_groups = motor_grouped(self_);
    let other_groups = origin_grouped(other);
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(self_.group0_.w * other.e4, 0.0, 0.0, 0.0)
    ));
}
fn motor_antiWedge_plane(self_: Motor, other: Plane) -> Flector {
    let self_groups = motor_grouped(self_);
    let other_groups = plane_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group0_.x * other_groups.group0_.w) + (self_.group1_.y * other_groups.group0_.z), (self_.group0_.y * other_groups.group0_.w) + (self_.group1_.z * other_groups.group0_.x), (self_.group0_.z * other_groups.group0_.w) + (self_.group1_.x * other_groups.group0_.y), -(self_.group0_.y * other_groups.group0_.y) - (self_.group0_.z * other_groups.group0_.z)) - (vec4<f32>(self_.group1_.z, self_.group1_.x, self_.group1_.y, self_.group0_.x) * other_groups.group0_.yzxx), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.w) * other_groups.group0_
    ));
}
fn motor_antiWedge_point(self_: Motor, other: Point) -> Point {
    let self_groups = motor_grouped(self_);
    let other_groups = point_grouped(other);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group0_.w) * other_groups.group0_
    ));
}
fn motor_antiWedge_scalar(self_: Motor, other: Scalar) -> Scalar {
    let self_groups = motor_grouped(self_);
    let other_groups = scalar_grouped(other);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(self_.group0_.w * other.scalar, 0.0, 0.0, 0.0)
    ));
}
fn multiVector_antiWedge_antiScalar(self_: MultiVector, other: AntiScalar) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(other.e1234), 0.0, 0.0) * self_.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(other.e1234) * self_.group1_, 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(other.e1234), 0.0) * self_.group2_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(other.e1234), 0.0) * self_.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e1234) * self_.group4_
    ));
}
fn multiVector_antiWedge_dualNum(self_: MultiVector, other: DualNum) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((other_groups.group0_.x * self_.group0_.y) + (other_groups.group0_.y * self_.group0_.x), other_groups.group0_.y * self_.group0_.y, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group0_.y) * self_.group1_, 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(other_groups.group0_.y), 0.0) * self_.group2_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(other_groups.group0_.y), 0.0) * self_.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group0_.y) * self_.group4_
    ));
}
fn multiVector_antiWedge_flector(self_: MultiVector, other: Flector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = flector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((other_groups.group1_.x * self_.group1_.x) + (other_groups.group1_.y * self_.group1_.y) + (other_groups.group1_.z * self_.group1_.z) + (other_groups.group1_.w * self_.group1_.w) - (other_groups.group0_.x * self_.group4_.x) - (other_groups.group0_.y * self_.group4_.y) - (other_groups.group0_.z * self_.group4_.z) - (other_groups.group0_.w * self_.group4_.w), 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group2_.x * other_groups.group1_.w) + (self_.group3_.y * other_groups.group1_.z), (self_.group2_.y * other_groups.group1_.w) + (self_.group3_.z * other_groups.group1_.x), (self_.group2_.z * other_groups.group1_.w) + (self_.group3_.x * other_groups.group1_.y), -(self_.group2_.y * other_groups.group1_.y) - (self_.group2_.z * other_groups.group1_.z)) + (vec4<f32>(self_.group0_.y) * other_groups.group0_) - (vec4<f32>(self_.group3_.zxy.xyz, self_.group2_.x) * other_groups.group1_.yzxx), 
        /* e41, e42, e43 */ vec4<f32>((other_groups.group1_.y * self_.group4_.z) - (other_groups.group1_.z * self_.group4_.y), (other_groups.group1_.z * self_.group4_.x) - (other_groups.group1_.x * self_.group4_.z), (other_groups.group1_.x * self_.group4_.y) - (other_groups.group1_.y * self_.group4_.x), 0.0), 
        /* e23, e31, e12 */ (vec4<f32>(vec3<f32>(other_groups.group1_.w), 0.0) * vec4<f32>(self_.group4_.x, self_.group4_.y, self_.group4_.z, 0.0)) - (vec4<f32>(vec3<f32>(self_.group4_.w), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.y) * other_groups.group1_
    ));
}
fn multiVector_antiWedge_horizon(self_: MultiVector, other: Horizon) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = horizon_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.group1_.w * other.e321, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group2_.x * other.e321, self_.group2_.y * other.e321, self_.group2_.z * other.e321, 0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(other.e321), 0.0) * vec4<f32>(self_.group4_.x, self_.group4_.y, self_.group4_.z, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.group0_.y * other.e321)
    ));
}
fn multiVector_antiWedge_line(self_: MultiVector, other: Line) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = line_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(-(other_groups.group0_.x * self_.group3_.x) - (other_groups.group0_.y * self_.group3_.y) - (other_groups.group0_.z * self_.group3_.z) - (other_groups.group1_.x * self_.group2_.x) - (other_groups.group1_.y * self_.group2_.y) - (other_groups.group1_.z * self_.group2_.z), 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group0_.x * self_.group4_.w) + (other_groups.group1_.y * self_.group4_.z), (other_groups.group0_.y * self_.group4_.w) + (other_groups.group1_.z * self_.group4_.x), (other_groups.group0_.z * self_.group4_.w) + (other_groups.group1_.x * self_.group4_.y), -(other_groups.group0_.y * self_.group4_.y) - (other_groups.group0_.z * self_.group4_.z)) - (vec4<f32>(other_groups.group1_.zxy.xyz, other_groups.group0_.x) * self_.group4_.yzxx), 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(self_.group0_.y), 0.0) * other_groups.group0_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(self_.group0_.y), 0.0) * other_groups.group1_, 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn multiVector_antiWedge_motor(self_: MultiVector, other: Motor) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = motor_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((self_.group0_.x * other_groups.group0_.w) + (self_.group0_.y * other_groups.group1_.w) - (self_.group2_.x * other_groups.group1_.x) - (self_.group2_.y * other_groups.group1_.y) - (self_.group2_.z * other_groups.group1_.z) - (self_.group3_.x * other_groups.group0_.x) - (self_.group3_.y * other_groups.group0_.y) - (self_.group3_.z * other_groups.group0_.z), self_.group0_.y * other_groups.group0_.w, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group0_.w * self_.group1_.x) + (other_groups.group1_.y * self_.group4_.z), (other_groups.group0_.w * self_.group1_.y) + (other_groups.group1_.z * self_.group4_.x), (other_groups.group0_.w * self_.group1_.z) + (other_groups.group1_.x * self_.group4_.y), -(other_groups.group0_.y * self_.group4_.y) - (other_groups.group0_.z * self_.group4_.z)) + (vec4<f32>(self_.group4_.w, self_.group4_.w, self_.group4_.w, self_.group1_.w) * other_groups.group0_) - (vec4<f32>(other_groups.group1_.z, other_groups.group1_.x, other_groups.group1_.y, other_groups.group0_.x) * self_.group4_.yzxx), 
        /* e41, e42, e43 */ (vec4<f32>(vec3<f32>(self_.group0_.y), 0.0) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0)) + (vec4<f32>(vec3<f32>(other_groups.group0_.w), 0.0) * self_.group2_), 
        /* e23, e31, e12 */ (vec4<f32>(vec3<f32>(self_.group0_.y), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0)) + (vec4<f32>(vec3<f32>(other_groups.group0_.w), 0.0) * self_.group3_), 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group0_.w) * self_.group4_
    ));
}
fn multiVector_antiWedge_multiVector(self_: MultiVector, other: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((other_groups.group0_.x * self_.group0_.y) + (other_groups.group0_.y * self_.group0_.x) + (other_groups.group4_.x * self_.group1_.x) + (other_groups.group4_.y * self_.group1_.y) + (other_groups.group4_.z * self_.group1_.z) + (other_groups.group4_.w * self_.group1_.w) - (other_groups.group2_.x * self_.group3_.x) - (other_groups.group2_.y * self_.group3_.y) - (other_groups.group2_.z * self_.group3_.z) - (other_groups.group3_.x * self_.group2_.x) - (other_groups.group3_.y * self_.group2_.y) - (other_groups.group3_.z * self_.group2_.z) - (other_groups.group1_.x * self_.group4_.x) - (other_groups.group1_.y * self_.group4_.y) - (other_groups.group1_.z * self_.group4_.z) - (other_groups.group1_.w * self_.group4_.w), other_groups.group0_.y * self_.group0_.y, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group2_.x * self_.group4_.w) + (other_groups.group3_.y * self_.group4_.z) + (self_.group2_.x * other_groups.group4_.w) + (self_.group3_.y * other_groups.group4_.z), (other_groups.group2_.y * self_.group4_.w) + (other_groups.group3_.z * self_.group4_.x) + (self_.group2_.y * other_groups.group4_.w) + (self_.group3_.z * other_groups.group4_.x), (other_groups.group2_.z * self_.group4_.w) + (other_groups.group3_.x * self_.group4_.y) + (self_.group2_.z * other_groups.group4_.w) + (self_.group3_.x * other_groups.group4_.y), -(other_groups.group2_.y * self_.group4_.y) - (other_groups.group2_.z * self_.group4_.z) - (self_.group2_.y * other_groups.group4_.y) - (self_.group2_.z * other_groups.group4_.z)) + (vec4<f32>(other_groups.group0_.y) * self_.group1_) + (vec4<f32>(self_.group0_.y) * other_groups.group1_) - (vec4<f32>(other_groups.group3_.zxy.xyz, other_groups.group2_.x) * self_.group4_.yzxx) - (vec4<f32>(self_.group3_.zxy.xyz, self_.group2_.x) * other_groups.group4_.yzxx), 
        /* e41, e42, e43 */ vec4<f32>((other_groups.group4_.y * self_.group4_.z) - (other_groups.group4_.z * self_.group4_.y), (other_groups.group4_.z * self_.group4_.x) - (other_groups.group4_.x * self_.group4_.z), (other_groups.group4_.x * self_.group4_.y) - (other_groups.group4_.y * self_.group4_.x), 0.0) + (vec4<f32>(vec3<f32>(other_groups.group0_.y), 0.0) * self_.group2_) + (vec4<f32>(vec3<f32>(self_.group0_.y), 0.0) * other_groups.group2_), 
        /* e23, e31, e12 */ (vec4<f32>(vec3<f32>(other_groups.group0_.y), 0.0) * self_.group3_) + (vec4<f32>(vec3<f32>(self_.group0_.y), 0.0) * other_groups.group3_) + (vec4<f32>(vec3<f32>(other_groups.group4_.w), 0.0) * vec4<f32>(self_.group4_.x, self_.group4_.y, self_.group4_.z, 0.0)) - (vec4<f32>(vec3<f32>(self_.group4_.w), 0.0) * vec4<f32>(other_groups.group4_.x, other_groups.group4_.y, other_groups.group4_.z, 0.0)), 
        /* e423, e431, e412, e321 */ (vec4<f32>(other_groups.group0_.y) * self_.group4_) + (vec4<f32>(self_.group0_.y) * other_groups.group4_)
    ));
}
fn multiVector_antiWedge_origin(self_: MultiVector, other: Origin) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = origin_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.group4_.w * other.e4 * -1.0, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.group0_.y * other.e4), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn multiVector_antiWedge_plane(self_: MultiVector, other: Plane) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = plane_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((self_.group1_.x * other_groups.group0_.x) + (self_.group1_.y * other_groups.group0_.y) + (self_.group1_.z * other_groups.group0_.z) + (self_.group1_.w * other_groups.group0_.w), 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group2_.x * other_groups.group0_.w) + (self_.group3_.y * other_groups.group0_.z), (self_.group2_.y * other_groups.group0_.w) + (self_.group3_.z * other_groups.group0_.x), (self_.group2_.z * other_groups.group0_.w) + (self_.group3_.x * other_groups.group0_.y), -(self_.group2_.y * other_groups.group0_.y) - (self_.group2_.z * other_groups.group0_.z)) - (vec4<f32>(self_.group3_.zxy.xyz, self_.group2_.x) * other_groups.group0_.yzxx), 
        /* e41, e42, e43 */ vec4<f32>((self_.group4_.z * other_groups.group0_.y) - (self_.group4_.y * other_groups.group0_.z), (self_.group4_.x * other_groups.group0_.z) - (self_.group4_.z * other_groups.group0_.x), (self_.group4_.y * other_groups.group0_.x) - (self_.group4_.x * other_groups.group0_.y), 0.0), 
        /* e23, e31, e12 */ (vec4<f32>(vec3<f32>(other_groups.group0_.w), 0.0) * vec4<f32>(self_.group4_.x, self_.group4_.y, self_.group4_.z, 0.0)) - (vec4<f32>(vec3<f32>(self_.group4_.w), 0.0) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.y) * other_groups.group0_
    ));
}
fn multiVector_antiWedge_point(self_: MultiVector, other: Point) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = point_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(-(self_.group4_.x * other_groups.group0_.x) - (self_.group4_.y * other_groups.group0_.y) - (self_.group4_.z * other_groups.group0_.z) - (self_.group4_.w * other_groups.group0_.w), 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group0_.y) * other_groups.group0_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn multiVector_antiWedge_scalar(self_: MultiVector, other: Scalar) -> Scalar {
    let self_groups = multiVector_grouped(self_);
    let other_groups = scalar_grouped(other);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(self_.group0_.y * other.scalar, 0.0, 0.0, 0.0)
    ));
}
fn origin_antiWedge_antiScalar(self_: Origin, other: AntiScalar) -> Origin {
    let self_groups = origin_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(other.e1234 * self.e4, 0.0, 0.0, 0.0)
    ));
}
fn origin_antiWedge_dualNum(self_: Origin, other: DualNum) -> Origin {
    let self_groups = origin_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(other_groups.group0_.y * self.e4, 0.0, 0.0, 0.0)
    ));
}
fn origin_antiWedge_flector(self_: Origin, other: Flector) -> Scalar {
    let self_groups = origin_grouped(self_);
    let other_groups = flector_grouped(other);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(other_groups.group1_.w * self.e4, 0.0, 0.0, 0.0)
    ));
}
fn origin_antiWedge_horizon(self_: Origin, other: Horizon) -> Scalar {
    let self_groups = origin_grouped(self_);
    let other_groups = horizon_grouped(other);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(other.e321 * self.e4, 0.0, 0.0, 0.0)
    ));
}
fn origin_antiWedge_motor(self_: Origin, other: Motor) -> Origin {
    let self_groups = origin_grouped(self_);
    let other_groups = motor_grouped(other);
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(other_groups.group0_.w * self.e4, 0.0, 0.0, 0.0)
    ));
}
fn origin_antiWedge_multiVector(self_: Origin, other: MultiVector) -> MultiVector {
    let self_groups = origin_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other_groups.group4_.w * self.e4, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group0_.y * self.e4), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn origin_antiWedge_plane(self_: Origin, other: Plane) -> Scalar {
    let self_groups = origin_grouped(self_);
    let other_groups = plane_grouped(other);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(other_groups.group0_.w * self.e4, 0.0, 0.0, 0.0)
    ));
}
fn plane_antiWedge_antiScalar(self_: Plane, other: AntiScalar) -> Plane {
    let self_groups = plane_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(other.e1234) * self_.group0_
    ));
}
fn plane_antiWedge_dualNum(self_: Plane, other: DualNum) -> Plane {
    let self_groups = plane_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group0_.y) * self_.group0_
    ));
}
fn plane_antiWedge_flector(self_: Plane, other: Flector) -> Motor {
    let self_groups = plane_grouped(self_);
    let other_groups = flector_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other_groups.group1_.y * self_.group0_.z) - (other_groups.group1_.z * self_.group0_.y), (other_groups.group1_.z * self_.group0_.x) - (other_groups.group1_.x * self_.group0_.z), (other_groups.group1_.x * self_.group0_.y) - (other_groups.group1_.y * self_.group0_.x), 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(other_groups.group1_.w * self_.group0_.x, other_groups.group1_.w * self_.group0_.y, other_groups.group1_.w * self_.group0_.z, -(other_groups.group0_.y * self_.group0_.y) - (other_groups.group0_.z * self_.group0_.z) - (other_groups.group0_.w * self_.group0_.w)) - (vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, other_groups.group0_.x) * self_.group0_.wwwx)
    ));
}
fn plane_antiWedge_horizon(self_: Plane, other: Horizon) -> Line {
    let self_groups = plane_grouped(self_);
    let other_groups = horizon_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(other.e321), 0.0) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0)
    ));
}
fn plane_antiWedge_line(self_: Plane, other: Line) -> Point {
    let self_groups = plane_grouped(self_);
    let other_groups = line_grouped(other);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group0_.x * self_.group0_.w) + (other_groups.group1_.y * self_.group0_.z), (other_groups.group0_.y * self_.group0_.w) + (other_groups.group1_.z * self_.group0_.x), (other_groups.group0_.z * self_.group0_.w) + (other_groups.group1_.x * self_.group0_.y), -(other_groups.group0_.y * self_.group0_.y) - (other_groups.group0_.z * self_.group0_.z)) - (vec4<f32>(other_groups.group1_.zxy.xyz, other_groups.group0_.x) * self_.group0_.yzxx)
    ));
}
fn plane_antiWedge_motor(self_: Plane, other: Motor) -> Flector {
    let self_groups = plane_grouped(self_);
    let other_groups = motor_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group0_.x * self_.group0_.w) + (other_groups.group1_.y * self_.group0_.z), (other_groups.group0_.y * self_.group0_.w) + (other_groups.group1_.z * self_.group0_.x), (other_groups.group0_.z * self_.group0_.w) + (other_groups.group1_.x * self_.group0_.y), -(other_groups.group0_.y * self_.group0_.y) - (other_groups.group0_.z * self_.group0_.z)) - (vec4<f32>(other_groups.group1_.z, other_groups.group1_.x, other_groups.group1_.y, other_groups.group0_.x) * self_.group0_.yzxx), 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group0_.w) * self_.group0_
    ));
}
fn plane_antiWedge_multiVector(self_: Plane, other: MultiVector) -> MultiVector {
    let self_groups = plane_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(-(other_groups.group1_.x * self_.group0_.x) - (other_groups.group1_.y * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.z) - (other_groups.group1_.w * self_.group0_.w), 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group2_.x * self_.group0_.w) + (other_groups.group3_.y * self_.group0_.z), (other_groups.group2_.y * self_.group0_.w) + (other_groups.group3_.z * self_.group0_.x), (other_groups.group2_.z * self_.group0_.w) + (other_groups.group3_.x * self_.group0_.y), -(other_groups.group2_.y * self_.group0_.y) - (other_groups.group2_.z * self_.group0_.z)) - (vec4<f32>(other_groups.group3_.zxy.xyz, other_groups.group2_.x) * self_.group0_.yzxx), 
        /* e41, e42, e43 */ vec4<f32>((other_groups.group4_.y * self_.group0_.z) - (other_groups.group4_.z * self_.group0_.y), (other_groups.group4_.z * self_.group0_.x) - (other_groups.group4_.x * self_.group0_.z), (other_groups.group4_.x * self_.group0_.y) - (other_groups.group4_.y * self_.group0_.x), 0.0), 
        /* e23, e31, e12 */ (vec4<f32>(vec3<f32>(other_groups.group4_.w), 0.0) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0)) - (vec4<f32>(vec3<f32>(self_.group0_.w), 0.0) * vec4<f32>(other_groups.group4_.x, other_groups.group4_.y, other_groups.group4_.z, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group0_.y) * self_.group0_
    ));
}
fn plane_antiWedge_origin(self_: Plane, other: Origin) -> Scalar {
    let self_groups = plane_grouped(self_);
    let other_groups = origin_grouped(other);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(self_.group0_.w * other.e4 * -1.0, 0.0, 0.0, 0.0)
    ));
}
fn plane_antiWedge_plane(self_: Plane, other: Plane) -> Line {
    let self_groups = plane_grouped(self_);
    let other_groups = plane_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>((other_groups.group0_.y * self_.group0_.z) - (other_groups.group0_.z * self_.group0_.y), (other_groups.group0_.z * self_.group0_.x) - (other_groups.group0_.x * self_.group0_.z), (other_groups.group0_.x * self_.group0_.y) - (other_groups.group0_.y * self_.group0_.x), 0.0), 
        /* e23, e31, e12 */ (vec4<f32>(vec3<f32>(other_groups.group0_.w), 0.0) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0)) - (vec4<f32>(vec3<f32>(self_.group0_.w), 0.0) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0))
    ));
}
fn plane_antiWedge_point(self_: Plane, other: Point) -> Scalar {
    let self_groups = plane_grouped(self_);
    let other_groups = point_grouped(other);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(-(self_.group0_.x * other_groups.group0_.x) - (self_.group0_.y * other_groups.group0_.y) - (self_.group0_.z * other_groups.group0_.z) - (self_.group0_.w * other_groups.group0_.w), 0.0, 0.0, 0.0)
    ));
}
fn point_antiWedge_antiScalar(self_: Point, other: AntiScalar) -> Point {
    let self_groups = point_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e1234) * self_.group0_
    ));
}
fn point_antiWedge_dualNum(self_: Point, other: DualNum) -> Point {
    let self_groups = point_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group0_.y) * self_.group0_
    ));
}
fn point_antiWedge_flector(self_: Point, other: Flector) -> Scalar {
    let self_groups = point_grouped(self_);
    let other_groups = flector_grouped(other);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>((other_groups.group1_.x * self_.group0_.x) + (other_groups.group1_.y * self_.group0_.y) + (other_groups.group1_.z * self_.group0_.z) + (other_groups.group1_.w * self_.group0_.w), 0.0, 0.0, 0.0)
    ));
}
fn point_antiWedge_horizon(self_: Point, other: Horizon) -> Scalar {
    let self_groups = point_grouped(self_);
    let other_groups = horizon_grouped(other);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(self_.group0_.w * other.e321, 0.0, 0.0, 0.0)
    ));
}
fn point_antiWedge_motor(self_: Point, other: Motor) -> Point {
    let self_groups = point_grouped(self_);
    let other_groups = motor_grouped(other);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group0_.w) * self_.group0_
    ));
}
fn point_antiWedge_multiVector(self_: Point, other: MultiVector) -> MultiVector {
    let self_groups = point_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((other_groups.group4_.x * self_.group0_.x) + (other_groups.group4_.y * self_.group0_.y) + (other_groups.group4_.z * self_.group0_.z) + (other_groups.group4_.w * self_.group0_.w), 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group0_.y) * self_.group0_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn point_antiWedge_plane(self_: Point, other: Plane) -> Scalar {
    let self_groups = point_grouped(self_);
    let other_groups = plane_grouped(other);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>((other_groups.group0_.x * self_.group0_.x) + (other_groups.group0_.y * self_.group0_.y) + (other_groups.group0_.z * self_.group0_.z) + (other_groups.group0_.w * self_.group0_.w), 0.0, 0.0, 0.0)
    ));
}
fn scalar_antiWedge_antiScalar(self_: Scalar, other: AntiScalar) -> Scalar {
    let self_groups = scalar_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(other.e1234 * self.scalar, 0.0, 0.0, 0.0)
    ));
}
fn scalar_antiWedge_dualNum(self_: Scalar, other: DualNum) -> Scalar {
    let self_groups = scalar_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(other_groups.group0_.y * self.scalar, 0.0, 0.0, 0.0)
    ));
}
fn scalar_antiWedge_motor(self_: Scalar, other: Motor) -> Scalar {
    let self_groups = scalar_grouped(self_);
    let other_groups = motor_grouped(other);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(other_groups.group0_.w * self.scalar, 0.0, 0.0, 0.0)
    ));
}
fn scalar_antiWedge_multiVector(self_: Scalar, other: MultiVector) -> Scalar {
    let self_groups = scalar_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(other_groups.group0_.y * self.scalar, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_constraintValid(self_: AntiScalar) -> AntiScalar {
    let self_groups = antiScalar_grouped(self_);    return self;
}
fn origin_constraintValid(self_: Origin) -> Origin {
    let self_groups = origin_grouped(self_);    return self;
}
fn dualNum_constraintViolation(self_: DualNum) -> AntiScalar {
    let self_groups = dualNum_grouped(self_);    let geometric_product_groups: DualNumGroups = DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(pow(self_.group0_.x, 2), self_.group0_.x * self_.group0_.y, 0.0, 0.0) * vec2<f32>(1.0, 2.0)
    );
    let geometric_product: DualNum = dualNum_degroup(geometric_product_groups);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(geometric_product_groups.group0_.y, 0.0, 0.0, 0.0)
    ));
}
fn flector_constraintViolation(self_: Flector) -> DualNum {
    let self_groups = flector_grouped(self_);    let reverse_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ self_.group0_, 
        /* e423, e431, e412, e321 */ self_.group1_ * vec4<f32>(-1.0)
    );
    let reverse: Flector = flector_degroup(reverse_groups);
    let geometric_product_groups: DualNumGroups = DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, (reverse_groups.group1_.w * self_.group0_.w) - (reverse_groups.group0_.x * self_.group1_.x) - (reverse_groups.group0_.y * self_.group1_.y) - (reverse_groups.group0_.z * self_.group1_.z), 0.0, 0.0) + (vec4<f32>(vec2<f32>(self_.group0_.x), 0.0, 0.0) * vec4<f32>(reverse_groups.group0_.x, reverse_groups.group1_.x, 0.0, 0.0)) + (vec4<f32>(vec2<f32>(self_.group0_.y), 0.0, 0.0) * vec4<f32>(reverse_groups.group0_.y, reverse_groups.group1_.y, 0.0, 0.0)) + (vec4<f32>(vec2<f32>(self_.group0_.z), 0.0, 0.0) * vec4<f32>(reverse_groups.group0_.z, reverse_groups.group1_.z, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(self_.group1_.w), 0.0, 0.0) * vec4<f32>(reverse_groups.group1_.w, reverse_groups.group0_.w, 0.0, 0.0))
    );
    let geometric_product: DualNum = dualNum_degroup(geometric_product_groups);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(self_.group0_.x, 2) + pow(self_.group0_.y, 2) + pow(self_.group0_.z, 2) - pow(self_.group1_.w, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(geometric_product_groups.group0_.x - scalar_product.scalar, geometric_product_groups.group0_.y, 0.0, 0.0)
    ));
}
fn horizon_constraintViolation(self_: Horizon) -> Scalar {
    let self_groups = horizon_grouped(self_);    let reverse_groups: HorizonGroups = HorizonGroups(
        /* e321 */ vec4<f32>(self.e321 * -1.0, 0.0, 0.0, 0.0)
    );
    let reverse: Horizon = horizon_degroup(reverse_groups);
    let geometric_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(reverse.e321 * self.e321 * -1.0, 0.0, 0.0, 0.0)
    );
    let geometric_product: Scalar = scalar_degroup(geometric_product_groups);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(self.e321, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(geometric_product.scalar - scalar_product.scalar, 0.0, 0.0, 0.0)
    ));
}
fn line_constraintViolation(self_: Line) -> DualNum {
    let self_groups = line_grouped(self_);    let reverse_groups: LineGroups = LineGroups(
        /* e41, e42, e43 */ self_.group0_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ self_.group1_ * vec3<f32>(-1.0)
    );
    let reverse: Line = line_degroup(reverse_groups);
    let geometric_product_groups: DualNumGroups = DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, -(reverse_groups.group1_.x * self_.group0_.x) - (reverse_groups.group1_.y * self_.group0_.y) - (reverse_groups.group1_.z * self_.group0_.z), 0.0, 0.0) - (vec4<f32>(vec2<f32>(self_.group1_.x), 0.0, 0.0) * vec4<f32>(reverse_groups.group1_.x, reverse_groups.group0_.x, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(self_.group1_.y), 0.0, 0.0) * vec4<f32>(reverse_groups.group1_.y, reverse_groups.group0_.y, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(self_.group1_.z), 0.0, 0.0) * vec4<f32>(reverse_groups.group1_.z, reverse_groups.group0_.z, 0.0, 0.0))
    );
    let geometric_product: DualNum = dualNum_degroup(geometric_product_groups);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(-pow(self_.group1_.x, 2) - pow(self_.group1_.y, 2) - pow(self_.group1_.z, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(geometric_product_groups.group0_.x - scalar_product.scalar, geometric_product_groups.group0_.y, 0.0, 0.0)
    ));
}
fn motor_constraintViolation(self_: Motor) -> DualNum {
    let self_groups = motor_grouped(self_);    let reverse_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.group0_.x * -1.0, self_.group0_.y * -1.0, self_.group0_.z * -1.0, self_.group0_.w), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.group1_.x * -1.0, self_.group1_.y * -1.0, self_.group1_.z * -1.0, self_.group1_.w)
    );
    let reverse: Motor = motor_degroup(reverse_groups);
    let geometric_product_groups: DualNumGroups = DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, (reverse_groups.group1_.w * self_.group0_.w) - (reverse_groups.group1_.x * self_.group0_.x) - (reverse_groups.group1_.y * self_.group0_.y) - (reverse_groups.group1_.z * self_.group0_.z), 0.0, 0.0) + (vec4<f32>(vec2<f32>(self_.group1_.w), 0.0, 0.0) * vec4<f32>(reverse_groups.group1_.w, reverse_groups.group0_.w, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(self_.group1_.x), 0.0, 0.0) * vec4<f32>(reverse_groups.group1_.x, reverse_groups.group0_.x, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(self_.group1_.y), 0.0, 0.0) * vec4<f32>(reverse_groups.group1_.y, reverse_groups.group0_.y, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(self_.group1_.z), 0.0, 0.0) * vec4<f32>(reverse_groups.group1_.z, reverse_groups.group0_.z, 0.0, 0.0))
    );
    let geometric_product: DualNum = dualNum_degroup(geometric_product_groups);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(self_.group1_.w, 2) - pow(self_.group1_.x, 2) - pow(self_.group1_.y, 2) - pow(self_.group1_.z, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(geometric_product_groups.group0_.x - scalar_product.scalar, geometric_product_groups.group0_.y, 0.0, 0.0)
    ));
}
fn multiVector_constraintViolation(self_: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);    let reverse_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ self_.group0_, 
        /* e1, e2, e3, e4 */ self_.group1_, 
        /* e41, e42, e43 */ self_.group2_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ self_.group3_ * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ self_.group4_ * vec4<f32>(-1.0)
    );
    let reverse: MultiVector = multiVector_degroup(reverse_groups);
    let geometric_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, (reverse_groups.group0_.y * self_.group0_.x) + (reverse_groups.group4_.w * self_.group1_.w) - (reverse_groups.group3_.x * self_.group2_.x) - (reverse_groups.group3_.y * self_.group2_.y) - (reverse_groups.group3_.z * self_.group2_.z) - (reverse_groups.group1_.x * self_.group4_.x) - (reverse_groups.group1_.y * self_.group4_.y) - (reverse_groups.group1_.z * self_.group4_.z), 0.0, 0.0) + (vec4<f32>(vec2<f32>(reverse_groups.group0_.x), 0.0, 0.0) * self_.group0_) + (vec4<f32>(vec2<f32>(self_.group1_.x), 0.0, 0.0) * vec4<f32>(reverse_groups.group1_.x, reverse_groups.group4_.x, 0.0, 0.0)) + (vec4<f32>(vec2<f32>(self_.group1_.y), 0.0, 0.0) * vec4<f32>(reverse_groups.group1_.y, reverse_groups.group4_.y, 0.0, 0.0)) + (vec4<f32>(vec2<f32>(self_.group1_.z), 0.0, 0.0) * vec4<f32>(reverse_groups.group1_.z, reverse_groups.group4_.z, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(self_.group3_.x), 0.0, 0.0) * vec4<f32>(reverse_groups.group3_.x, reverse_groups.group2_.x, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(self_.group3_.y), 0.0, 0.0) * vec4<f32>(reverse_groups.group3_.y, reverse_groups.group2_.y, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(self_.group3_.z), 0.0, 0.0) * vec4<f32>(reverse_groups.group3_.z, reverse_groups.group2_.z, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(self_.group4_.w), 0.0, 0.0) * vec4<f32>(reverse_groups.group4_.w, reverse_groups.group1_.w, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((reverse_groups.group3_.y * self_.group1_.z) + (self_.group3_.x * reverse_groups.group4_.w) - (self_.group3_.y * reverse_groups.group1_.z), (reverse_groups.group3_.z * self_.group1_.x) + (self_.group3_.y * reverse_groups.group4_.w) - (self_.group3_.z * reverse_groups.group1_.x), (reverse_groups.group3_.x * self_.group1_.y) + (self_.group3_.z * reverse_groups.group4_.w) - (self_.group3_.x * reverse_groups.group1_.y), (self_.group2_.y * reverse_groups.group1_.y) + (self_.group2_.z * reverse_groups.group1_.z) - (self_.group0_.y * reverse_groups.group4_.w) - (reverse_groups.group2_.y * self_.group1_.y) - (reverse_groups.group2_.z * self_.group1_.z) - (reverse_groups.group3_.x * self_.group4_.x) - (reverse_groups.group3_.y * self_.group4_.y) - (reverse_groups.group3_.z * self_.group4_.z) - (self_.group3_.x * reverse_groups.group4_.x) - (self_.group3_.y * reverse_groups.group4_.y) - (self_.group3_.z * reverse_groups.group4_.z)) + (vec4<f32>(reverse_groups.group0_.x) * self_.group1_) + (vec4<f32>(self_.group0_.x) * reverse_groups.group1_) + (vec4<f32>(self_.group4_.w) * vec4<f32>(reverse_groups.group3_.xyz.xyz, reverse_groups.group0_.y)) + (vec4<f32>(self_.group3_.zxy.xyz, self_.group2_.x) * reverse_groups.group1_.yzxx) - (vec4<f32>(reverse_groups.group3_.zxy.xyz, reverse_groups.group2_.x) * self_.group1_.yzxx), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    );
    let geometric_product: MultiVector = multiVector_degroup(geometric_product_groups);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(self_.group0_.x, 2) + pow(self_.group1_.x, 2) + pow(self_.group1_.y, 2) + pow(self_.group1_.z, 2) - pow(self_.group3_.x, 2) - pow(self_.group3_.y, 2) - pow(self_.group3_.z, 2) - pow(self_.group4_.w, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(geometric_product_groups.group0_.x - scalar_product.scalar, geometric_product_groups.group0_.y, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ geometric_product_groups.group1_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn plane_constraintViolation(self_: Plane) -> Scalar {
    let self_groups = plane_grouped(self_);    let reverse_groups: PlaneGroups = PlaneGroups(
        /* e423, e431, e412, e321 */ self_.group0_ * vec4<f32>(-1.0)
    );
    let reverse: Plane = plane_degroup(reverse_groups);
    let geometric_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(reverse_groups.group0_.w * self_.group0_.w * -1.0, 0.0, 0.0, 0.0)
    );
    let geometric_product: Scalar = scalar_degroup(geometric_product_groups);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(self_.group0_.w, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(geometric_product.scalar - scalar_product.scalar, 0.0, 0.0, 0.0)
    ));
}
fn point_constraintViolation(self_: Point) -> Scalar {
    let self_groups = point_grouped(self_);    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(0.0, 0.0, 0.0, 0.0)
    ));
}
fn scalar_constraintViolation(self_: Scalar) -> Scalar {
    let self_groups = scalar_grouped(self_);    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(0.0, 0.0, 0.0, 0.0)
    ));
}
fn horizon_fix(self_: Horizon) -> Horizon {
    let self_groups = horizon_grouped(self_);    let reverse_groups: HorizonGroups = HorizonGroups(
        /* e321 */ vec4<f32>(self.e321 * -1.0, 0.0, 0.0, 0.0)
    );
    let reverse: Horizon = horizon_degroup(reverse_groups);
    let geometric_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(reverse.e321 * self.e321 * -1.0, 0.0, 0.0, 0.0)
    );
    let geometric_product: Scalar = scalar_degroup(geometric_product_groups);
    let square_root_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(geometric_product.scalar, 0.5), 0.0, 0.0, 0.0)
    );
    let square_root: Scalar = scalar_degroup(square_root_groups);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(square_root.scalar, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(self.e321 * inverse.scalar, 0.0, 0.0, 0.0)
    ));
}
fn plane_fix(self_: Plane) -> Plane {
    let self_groups = plane_grouped(self_);    let reverse_groups: PlaneGroups = PlaneGroups(
        /* e423, e431, e412, e321 */ self_.group0_ * vec4<f32>(-1.0)
    );
    let reverse: Plane = plane_degroup(reverse_groups);
    let geometric_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(reverse_groups.group0_.w * self_.group0_.w * -1.0, 0.0, 0.0, 0.0)
    );
    let geometric_product: Scalar = scalar_degroup(geometric_product_groups);
    let square_root_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(geometric_product.scalar, 0.5), 0.0, 0.0, 0.0)
    );
    let square_root: Scalar = scalar_degroup(square_root_groups);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(square_root.scalar, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_.group0_
    ));
}
fn point_fix(self_: Point) -> Point {
    let self_groups = point_grouped(self_);    let geometric_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(self_.group0_.x, 2) + pow(self_.group0_.y, 2) + pow(self_.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let geometric_product: Scalar = scalar_degroup(geometric_product_groups);
    let square_root_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(geometric_product.scalar, 0.5), 0.0, 0.0, 0.0)
    );
    let square_root: Scalar = scalar_degroup(square_root_groups);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(square_root.scalar, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_.group0_
    ));
}
fn scalar_fix(self_: Scalar) -> Scalar {
    let self_groups = scalar_grouped(self_);    let reverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(self.scalar, 0.0, 0.0, 0.0)
    );
    let reverse: Scalar = scalar_degroup(reverse_groups);
    let geometric_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(reverse.scalar * self.scalar, 0.0, 0.0, 0.0)
    );
    let geometric_product: Scalar = scalar_degroup(geometric_product_groups);
    let square_root_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(geometric_product.scalar, 0.5), 0.0, 0.0, 0.0)
    );
    let square_root: Scalar = scalar_degroup(square_root_groups);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(square_root.scalar, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(inverse.scalar * self.scalar, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_geometricAntiProduct_antiScalar(self_: AntiScalar, other: AntiScalar) -> AntiScalar {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(other.e1234 * self.e1234, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_geometricAntiProduct_dualNum(self_: AntiScalar, other: DualNum) -> DualNum {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(self.e1234), 0.0, 0.0) * other_groups.group0_
    ));
}
fn antiScalar_geometricAntiProduct_flector(self_: AntiScalar, other: Flector) -> Flector {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self.e1234) * other_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self.e1234) * other_groups.group1_
    ));
}
fn antiScalar_geometricAntiProduct_horizon(self_: AntiScalar, other: Horizon) -> Horizon {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = horizon_grouped(other);
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(self.e1234 * other.e321, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_geometricAntiProduct_line(self_: AntiScalar, other: Line) -> Line {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = line_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(self.e1234), 0.0) * other_groups.group0_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(self.e1234), 0.0) * other_groups.group1_
    ));
}
fn antiScalar_geometricAntiProduct_motor(self_: AntiScalar, other: Motor) -> Motor {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self.e1234) * other_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(self.e1234) * other_groups.group1_
    ));
}
fn antiScalar_geometricAntiProduct_multiVector(self_: AntiScalar, other: MultiVector) -> MultiVector {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(self.e1234), 0.0, 0.0) * other_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(self.e1234) * other_groups.group1_, 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(self.e1234), 0.0) * other_groups.group2_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(self.e1234), 0.0) * other_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self.e1234) * other_groups.group4_
    ));
}
fn antiScalar_geometricAntiProduct_origin(self_: AntiScalar, other: Origin) -> Origin {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = origin_grouped(other);
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(self.e1234 * other.e4, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_geometricAntiProduct_plane(self_: AntiScalar, other: Plane) -> Plane {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = plane_grouped(other);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(self.e1234) * other_groups.group0_
    ));
}
fn antiScalar_geometricAntiProduct_point(self_: AntiScalar, other: Point) -> Point {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = point_grouped(other);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self.e1234) * other_groups.group0_
    ));
}
fn antiScalar_geometricAntiProduct_scalar(self_: AntiScalar, other: Scalar) -> Scalar {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = scalar_grouped(other);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(self.e1234 * other.scalar, 0.0, 0.0, 0.0)
    ));
}
fn dualNum_geometricAntiProduct_antiScalar(self_: DualNum, other: AntiScalar) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(other.e1234), 0.0, 0.0) * self_.group0_
    ));
}
fn dualNum_geometricAntiProduct_dualNum(self_: DualNum, other: DualNum) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>((other_groups.group0_.x * self_.group0_.y) + (other_groups.group0_.y * self_.group0_.x), other_groups.group0_.y * self_.group0_.y, 0.0, 0.0)
    ));
}
fn dualNum_geometricAntiProduct_flector(self_: DualNum, other: Flector) -> Flector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group0_.x * other_groups.group1_.x) + (self_.group0_.y * other_groups.group0_.x), (self_.group0_.x * other_groups.group1_.y) + (self_.group0_.y * other_groups.group0_.y), (self_.group0_.x * other_groups.group1_.z) + (self_.group0_.y * other_groups.group0_.z), self_.group0_.y * other_groups.group0_.w), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.y * other_groups.group1_.x, self_.group0_.y * other_groups.group1_.y, self_.group0_.y * other_groups.group1_.z, (self_.group0_.x * other_groups.group0_.w) + (self_.group0_.y * other_groups.group1_.w))
    ));
}
fn dualNum_geometricAntiProduct_horizon(self_: DualNum, other: Horizon) -> Horizon {
    let self_groups = dualNum_grouped(self_);
    let other_groups = horizon_grouped(other);
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(self_.group0_.y * other.e321, 0.0, 0.0, 0.0)
    ));
}
fn dualNum_geometricAntiProduct_line(self_: DualNum, other: Line) -> Line {
    let self_groups = dualNum_grouped(self_);
    let other_groups = line_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(self_.group0_.y), 0.0) * other_groups.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(vec3<f32>(self_.group0_.x), 0.0) * other_groups.group0_) + (vec4<f32>(vec3<f32>(self_.group0_.y), 0.0) * other_groups.group1_)
    ));
}
fn dualNum_geometricAntiProduct_motor(self_: DualNum, other: Motor) -> Motor {
    let self_groups = dualNum_grouped(self_);
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.group0_.y) * other_groups.group0_, 
        /* e23, e31, e12, scalar */ (vec4<f32>(self_.group0_.x) * other_groups.group0_) + (vec4<f32>(self_.group0_.y) * other_groups.group1_)
    ));
}
fn dualNum_geometricAntiProduct_multiVector(self_: DualNum, other: MultiVector) -> MultiVector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((self_.group0_.x * other_groups.group0_.y) + (self_.group0_.y * other_groups.group0_.x), self_.group0_.y * other_groups.group0_.y, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group0_.x * other_groups.group4_.x) + (self_.group0_.y * other_groups.group1_.x), (self_.group0_.x * other_groups.group4_.y) + (self_.group0_.y * other_groups.group1_.y), (self_.group0_.x * other_groups.group4_.z) + (self_.group0_.y * other_groups.group1_.z), self_.group0_.y * other_groups.group1_.w), 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(self_.group0_.y), 0.0) * other_groups.group2_, 
        /* e23, e31, e12 */ (vec4<f32>(vec3<f32>(self_.group0_.x), 0.0) * other_groups.group2_) + (vec4<f32>(vec3<f32>(self_.group0_.y), 0.0) * other_groups.group3_), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.y * other_groups.group4_.x, self_.group0_.y * other_groups.group4_.y, self_.group0_.y * other_groups.group4_.z, (self_.group0_.x * other_groups.group1_.w) + (self_.group0_.y * other_groups.group4_.w))
    ));
}
fn dualNum_geometricAntiProduct_origin(self_: DualNum, other: Origin) -> Flector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = origin_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.group0_.y * other.e4), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.group0_.x * other.e4)
    ));
}
fn dualNum_geometricAntiProduct_plane(self_: DualNum, other: Plane) -> Flector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = plane_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group0_.x * other_groups.group0_.x, self_.group0_.x * other_groups.group0_.y, self_.group0_.x * other_groups.group0_.z, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.y) * other_groups.group0_
    ));
}
fn dualNum_geometricAntiProduct_point(self_: DualNum, other: Point) -> Flector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = point_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group0_.y) * other_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.group0_.x * other_groups.group0_.w)
    ));
}
fn dualNum_geometricAntiProduct_scalar(self_: DualNum, other: Scalar) -> Scalar {
    let self_groups = dualNum_grouped(self_);
    let other_groups = scalar_grouped(other);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(self_.group0_.y * other.scalar, 0.0, 0.0, 0.0)
    ));
}
fn flector_geometricAntiProduct_antiScalar(self_: Flector, other: AntiScalar) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e1234) * self_.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e1234) * self_.group1_
    ));
}
fn flector_geometricAntiProduct_dualNum(self_: Flector, other: DualNum) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group0_.y * self_.group0_.x) - (other_groups.group0_.x * self_.group1_.x), (other_groups.group0_.y * self_.group0_.y) - (other_groups.group0_.x * self_.group1_.y), (other_groups.group0_.y * self_.group0_.z) - (other_groups.group0_.x * self_.group1_.z), other_groups.group0_.y * self_.group0_.w), 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group0_.y * self_.group1_.x, other_groups.group0_.y * self_.group1_.y, other_groups.group0_.y * self_.group1_.z, (other_groups.group0_.y * self_.group1_.w) - (other_groups.group0_.x * self_.group0_.w))
    ));
}
fn flector_geometricAntiProduct_flector(self_: Flector, other: Flector) -> Motor {
    let self_groups = flector_grouped(self_);
    let other_groups = flector_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(-(other_groups.group1_.x * self_.group0_.w) - (other_groups.group1_.z * self_.group1_.y), -(other_groups.group1_.x * self_.group1_.z) - (other_groups.group1_.y * self_.group0_.w), -(other_groups.group1_.y * self_.group1_.x) - (other_groups.group1_.z * self_.group0_.w), (other_groups.group1_.y * self_.group1_.y) + (other_groups.group1_.z * self_.group1_.z)) + (other_groups.group1_.yzxx * self_.group1_.zxyx) - (vec4<f32>(other_groups.group0_.w) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group0_.w)), 
        /* e23, e31, e12, scalar */ vec4<f32>((other_groups.group0_.z * self_.group1_.y) - (other_groups.group1_.z * self_.group0_.y), (other_groups.group0_.x * self_.group1_.z) - (other_groups.group1_.x * self_.group0_.z), (other_groups.group0_.y * self_.group1_.x) - (other_groups.group1_.y * self_.group0_.x), (other_groups.group1_.w * self_.group0_.w) - (other_groups.group0_.w * self_.group1_.w)) + (vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, other_groups.group1_.x) * self_.group0_.wwwx) + (vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group0_.z) * other_groups.group1_.wwwz) + (other_groups.group1_.yzxy * self_.group0_.zxyy) - (vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, other_groups.group0_.z) * self_.group1_.wwwz) - (vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group1_.y) * other_groups.group0_.wwwy) - (other_groups.group0_.yzxx * self_.group1_.zxyx)
    ));
}
fn flector_geometricAntiProduct_horizon(self_: Flector, other: Horizon) -> Motor {
    let self_groups = flector_grouped(self_);
    let other_groups = horizon_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(other.e321) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group0_.w)
    ));
}
fn flector_geometricAntiProduct_line(self_: Flector, other: Line) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = line_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group0_.x * self_.group1_.w) + (other_groups.group0_.z * self_.group0_.y) + (other_groups.group1_.y * self_.group1_.z) - (other_groups.group0_.y * self_.group0_.z) - (other_groups.group1_.x * self_.group0_.w), (other_groups.group0_.x * self_.group0_.z) + (other_groups.group0_.y * self_.group1_.w) + (other_groups.group1_.z * self_.group1_.x) - (other_groups.group0_.z * self_.group0_.x) - (other_groups.group1_.y * self_.group0_.w), (other_groups.group0_.y * self_.group0_.x) + (other_groups.group0_.z * self_.group1_.w) + (other_groups.group1_.x * self_.group1_.y) - (other_groups.group0_.x * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.w), -(other_groups.group0_.y * self_.group1_.y) - (other_groups.group0_.z * self_.group1_.z)) - (vec4<f32>(other_groups.group1_.zxy.xyz, other_groups.group0_.x) * self_.group1_.yzxx), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.x * self_.group0_.w) - (other_groups.group0_.y * self_.group1_.z), (other_groups.group0_.y * self_.group0_.w) - (other_groups.group0_.z * self_.group1_.x), (other_groups.group0_.z * self_.group0_.w) - (other_groups.group0_.x * self_.group1_.y), (other_groups.group1_.y * self_.group1_.y) + (other_groups.group1_.z * self_.group1_.z) - (other_groups.group0_.x * self_.group0_.x) - (other_groups.group0_.y * self_.group0_.y) - (other_groups.group0_.z * self_.group0_.z)) + (vec4<f32>(other_groups.group0_.zxy.xyz, other_groups.group1_.x) * self_.group1_.yzxx)
    ));
}
fn flector_geometricAntiProduct_motor(self_: Flector, other: Motor) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = motor_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group0_.y * other_groups.group0_.z) + (self_.group1_.z * other_groups.group1_.y) + (self_.group1_.w * other_groups.group0_.x) - (self_.group0_.w * other_groups.group1_.x), (self_.group0_.z * other_groups.group0_.x) + (self_.group1_.x * other_groups.group1_.z) + (self_.group1_.w * other_groups.group0_.y) - (self_.group0_.w * other_groups.group1_.y), (self_.group0_.z * other_groups.group0_.w) + (self_.group1_.y * other_groups.group1_.x) + (self_.group1_.w * other_groups.group0_.z) - (self_.group0_.w * other_groups.group1_.z), 0.0) + (self_.group0_.xyxw * other_groups.group0_.wwyw) - (vec4<f32>(self_.group0_.z, self_.group0_.x, self_.group0_.y, self_.group1_.x) * other_groups.group0_.yzxx) - (vec4<f32>(other_groups.group1_.z, other_groups.group1_.x, other_groups.group1_.w, other_groups.group0_.z) * self_.group1_.yzzz) - (vec4<f32>(other_groups.group1_.w, other_groups.group1_.w, other_groups.group1_.y, other_groups.group0_.y) * self_.group1_.xyxy), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (self_.group1_.z * other_groups.group1_.z) - (self_.group0_.y * other_groups.group0_.y) - (self_.group0_.z * other_groups.group0_.z) - (self_.group0_.w * other_groups.group1_.w)) + (vec4<f32>(self_.group0_.w, self_.group0_.w, self_.group0_.w, self_.group1_.w) * other_groups.group0_) + (vec4<f32>(other_groups.group0_.z, other_groups.group0_.x, other_groups.group0_.w, other_groups.group1_.y) * self_.group1_.yzzy) + (vec4<f32>(other_groups.group0_.w, other_groups.group0_.w, other_groups.group0_.y, other_groups.group1_.x) * self_.group1_.xyxx) - (vec4<f32>(self_.group1_.z, self_.group1_.x, self_.group1_.y, self_.group0_.x) * other_groups.group0_.yzxx)
    ));
}
fn flector_geometricAntiProduct_multiVector(self_: Flector, other: MultiVector) -> MultiVector {
    let self_groups = flector_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((self_.group0_.w * other_groups.group4_.w) - (self_.group1_.x * other_groups.group1_.x) - (self_.group1_.y * other_groups.group1_.y) - (self_.group1_.z * other_groups.group1_.z), 0.0, 0.0, 0.0) + (vec4<f32>(vec2<f32>(other_groups.group4_.x), 0.0, 0.0) * vec4<f32>(self_.group0_.x, self_.group1_.x, 0.0, 0.0)) + (vec4<f32>(vec2<f32>(other_groups.group4_.y), 0.0, 0.0) * vec4<f32>(self_.group0_.y, self_.group1_.y, 0.0, 0.0)) + (vec4<f32>(vec2<f32>(other_groups.group4_.z), 0.0, 0.0) * vec4<f32>(self_.group0_.z, self_.group1_.z, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(other_groups.group1_.w), 0.0, 0.0) * vec4<f32>(self_.group1_.w, self_.group0_.w, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group2_.x * self_.group1_.w) + (other_groups.group2_.z * self_.group0_.y) + (other_groups.group3_.y * self_.group1_.z) - (other_groups.group2_.y * self_.group0_.z) - (other_groups.group3_.x * self_.group0_.w), (other_groups.group2_.x * self_.group0_.z) + (other_groups.group2_.y * self_.group1_.w) + (other_groups.group3_.z * self_.group1_.x) - (other_groups.group2_.z * self_.group0_.x) - (other_groups.group3_.y * self_.group0_.w), (other_groups.group2_.y * self_.group0_.x) + (other_groups.group2_.z * self_.group1_.w) + (other_groups.group3_.x * self_.group1_.y) - (other_groups.group2_.x * self_.group0_.y) - (other_groups.group3_.z * self_.group0_.w), (other_groups.group2_.z * self_.group1_.z) * -1.0) + (vec4<f32>(other_groups.group0_.y) * self_.group0_) - (vec4<f32>(other_groups.group0_.xx.xy, other_groups.group0_.x, other_groups.group2_.x) * self_.group1_.xyzx) - (vec4<f32>(other_groups.group3_.zxy.xyz, other_groups.group2_.y) * self_.group1_.yzxy), 
        /* e41, e42, e43 */ vec4<f32>((self_.group1_.z * other_groups.group4_.y) - (self_.group1_.y * other_groups.group4_.z), (self_.group1_.x * other_groups.group4_.z) - (self_.group1_.z * other_groups.group4_.x), (self_.group1_.y * other_groups.group4_.x) - (self_.group1_.x * other_groups.group4_.y), 0.0) - (vec4<f32>(vec3<f32>(self_.group0_.w), 0.0) * vec4<f32>(other_groups.group4_.x, other_groups.group4_.y, other_groups.group4_.z, 0.0)) - (vec4<f32>(vec3<f32>(other_groups.group1_.w), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((self_.group0_.z * other_groups.group4_.y) + (self_.group1_.y * other_groups.group1_.z) - (self_.group0_.y * other_groups.group4_.z) - (self_.group1_.z * other_groups.group1_.y), (self_.group0_.x * other_groups.group4_.z) + (self_.group1_.z * other_groups.group1_.x) - (self_.group0_.z * other_groups.group4_.x) - (self_.group1_.x * other_groups.group1_.z), (self_.group0_.y * other_groups.group4_.x) + (self_.group1_.x * other_groups.group1_.y) - (self_.group0_.x * other_groups.group4_.y) - (self_.group1_.y * other_groups.group1_.x), 0.0) + (vec4<f32>(vec3<f32>(self_.group0_.w), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0)) + (vec4<f32>(vec3<f32>(other_groups.group4_.w), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0)) - (vec4<f32>(vec3<f32>(self_.group1_.w), 0.0) * vec4<f32>(other_groups.group4_.x, other_groups.group4_.y, other_groups.group4_.z, 0.0)) - (vec4<f32>(vec3<f32>(other_groups.group1_.w), 0.0) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group2_.x * self_.group0_.w) - (other_groups.group2_.y * self_.group1_.z), (other_groups.group2_.y * self_.group0_.w) - (other_groups.group2_.z * self_.group1_.x), (other_groups.group2_.z * self_.group0_.w) - (other_groups.group2_.x * self_.group1_.y), (other_groups.group3_.y * self_.group1_.y) + (other_groups.group3_.z * self_.group1_.z) - (other_groups.group0_.x * self_.group0_.w) - (other_groups.group2_.x * self_.group0_.x) - (other_groups.group2_.y * self_.group0_.y) - (other_groups.group2_.z * self_.group0_.z)) + (vec4<f32>(other_groups.group0_.y) * self_.group1_) + (vec4<f32>(other_groups.group2_.zxy.xyz, other_groups.group3_.x) * self_.group1_.yzxx)
    ));
}
fn flector_geometricAntiProduct_origin(self_: Flector, other: Origin) -> Motor {
    let self_groups = flector_grouped(self_);
    let other_groups = origin_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e4) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group0_.w) * vec4<f32>(-1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(other.e4) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group1_.w) * vec4<f32>(-1.0)
    ));
}
fn flector_geometricAntiProduct_plane(self_: Flector, other: Plane) -> Motor {
    let self_groups = flector_grouped(self_);
    let other_groups = plane_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(-(self_.group0_.w * other_groups.group0_.x) - (self_.group1_.y * other_groups.group0_.z), -(self_.group0_.w * other_groups.group0_.y) - (self_.group1_.z * other_groups.group0_.x), -(self_.group0_.w * other_groups.group0_.z) - (self_.group1_.x * other_groups.group0_.y), (self_.group1_.y * other_groups.group0_.y) + (self_.group1_.z * other_groups.group0_.z)) + (self_.group1_.zxyx * other_groups.group0_.yzxx), 
        /* e23, e31, e12, scalar */ vec4<f32>(-(self_.group0_.y * other_groups.group0_.z) - (self_.group1_.w * other_groups.group0_.x), -(self_.group0_.z * other_groups.group0_.x) - (self_.group1_.w * other_groups.group0_.y), -(self_.group0_.x * other_groups.group0_.y) - (self_.group1_.w * other_groups.group0_.z), (self_.group0_.z * other_groups.group0_.z) + (self_.group0_.w * other_groups.group0_.w)) + (vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group0_.y) * other_groups.group0_.wwwy) + (self_.group0_.zxyx * other_groups.group0_.yzxx)
    ));
}
fn flector_geometricAntiProduct_point(self_: Flector, other: Point) -> Motor {
    let self_groups = flector_grouped(self_);
    let other_groups = point_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other_groups.group0_.w) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group0_.w) * vec4<f32>(-1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>((self_.group0_.w * other_groups.group0_.x) + (self_.group1_.y * other_groups.group0_.z), (self_.group0_.w * other_groups.group0_.y) + (self_.group1_.z * other_groups.group0_.x), (self_.group0_.w * other_groups.group0_.z) + (self_.group1_.x * other_groups.group0_.y), -(self_.group1_.z * other_groups.group0_.z) - (self_.group1_.w * other_groups.group0_.w)) - (vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group1_.x) * other_groups.group0_.wwwx) - (self_.group1_.zxyy * other_groups.group0_.yzxy)
    ));
}
fn flector_geometricAntiProduct_scalar(self_: Flector, other: Scalar) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = scalar_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group1_.x * other.scalar * -1.0, self_.group1_.y * other.scalar * -1.0, self_.group1_.z * other.scalar * -1.0, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.group0_.w * other.scalar * -1.0)
    ));
}
fn horizon_geometricAntiProduct_antiScalar(self_: Horizon, other: AntiScalar) -> Horizon {
    let self_groups = horizon_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(other.e1234 * self.e321, 0.0, 0.0, 0.0)
    ));
}
fn horizon_geometricAntiProduct_dualNum(self_: Horizon, other: DualNum) -> Horizon {
    let self_groups = horizon_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(other_groups.group0_.y * self.e321, 0.0, 0.0, 0.0)
    ));
}
fn horizon_geometricAntiProduct_flector(self_: Horizon, other: Flector) -> Motor {
    let self_groups = horizon_grouped(self_);
    let other_groups = flector_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(self.e321) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, other_groups.group0_.w) * vec4<f32>(-1.0)
    ));
}
fn horizon_geometricAntiProduct_line(self_: Horizon, other: Line) -> Point {
    let self_groups = horizon_grouped(self_);
    let other_groups = line_grouped(other);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group0_.x * self.e321, other_groups.group0_.y * self.e321, other_groups.group0_.z * self.e321, 0.0)
    ));
}
fn horizon_geometricAntiProduct_motor(self_: Horizon, other: Motor) -> Flector {
    let self_groups = horizon_grouped(self_);
    let other_groups = motor_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group0_.x * self.e321, other_groups.group0_.y * self.e321, other_groups.group0_.z * self.e321, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group0_.w * self.e321)
    ));
}
fn horizon_geometricAntiProduct_multiVector(self_: Horizon, other: MultiVector) -> MultiVector {
    let self_groups = horizon_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other_groups.group1_.w * self.e321 * -1.0, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group2_.x * self.e321, other_groups.group2_.y * self.e321, other_groups.group2_.z * self.e321, 0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(self.e321), 0.0) * vec4<f32>(other_groups.group4_.x, other_groups.group4_.y, other_groups.group4_.z, 0.0) * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group0_.y * self.e321)
    ));
}
fn horizon_geometricAntiProduct_origin(self_: Horizon, other: Origin) -> Scalar {
    let self_groups = horizon_grouped(self_);
    let other_groups = origin_grouped(other);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(self.e321 * other.e4 * -1.0, 0.0, 0.0, 0.0)
    ));
}
fn horizon_geometricAntiProduct_plane(self_: Horizon, other: Plane) -> Line {
    let self_groups = horizon_grouped(self_);
    let other_groups = plane_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(self.e321), 0.0) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0) * vec3<f32>(-1.0)
    ));
}
fn horizon_geometricAntiProduct_point(self_: Horizon, other: Point) -> Scalar {
    let self_groups = horizon_grouped(self_);
    let other_groups = point_grouped(other);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(other_groups.group0_.w * self.e321 * -1.0, 0.0, 0.0, 0.0)
    ));
}
fn line_geometricAntiProduct_antiScalar(self_: Line, other: AntiScalar) -> Line {
    let self_groups = line_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(other.e1234), 0.0) * self_.group0_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(other.e1234), 0.0) * self_.group1_
    ));
}
fn line_geometricAntiProduct_dualNum(self_: Line, other: DualNum) -> Line {
    let self_groups = line_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(other_groups.group0_.y), 0.0) * self_.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(vec3<f32>(other_groups.group0_.x), 0.0) * self_.group0_) + (vec4<f32>(vec3<f32>(other_groups.group0_.y), 0.0) * self_.group1_)
    ));
}
fn line_geometricAntiProduct_flector(self_: Line, other: Flector) -> Flector {
    let self_groups = line_grouped(self_);
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group0_.x * other_groups.group1_.w) + (self_.group0_.y * other_groups.group0_.z) + (self_.group1_.x * other_groups.group0_.w) + (self_.group1_.y * other_groups.group1_.z) - (self_.group0_.z * other_groups.group0_.y), (self_.group0_.y * other_groups.group1_.w) + (self_.group0_.z * other_groups.group0_.x) + (self_.group1_.y * other_groups.group0_.w) + (self_.group1_.z * other_groups.group1_.x) - (self_.group0_.x * other_groups.group0_.z), (self_.group0_.x * other_groups.group0_.y) + (self_.group0_.z * other_groups.group1_.w) + (self_.group1_.x * other_groups.group1_.y) + (self_.group1_.z * other_groups.group0_.w) - (self_.group0_.y * other_groups.group0_.x), -(self_.group0_.y * other_groups.group1_.y) - (self_.group0_.z * other_groups.group1_.z)) - (vec4<f32>(self_.group1_.zxy.xyz, self_.group0_.x) * other_groups.group1_.yzxx), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.group0_.x * other_groups.group0_.w) + (self_.group0_.y * other_groups.group1_.z), (self_.group0_.y * other_groups.group0_.w) + (self_.group0_.z * other_groups.group1_.x), (self_.group0_.x * other_groups.group1_.y) + (self_.group0_.z * other_groups.group0_.w), -(self_.group0_.x * other_groups.group0_.x) - (self_.group0_.y * other_groups.group0_.y) - (self_.group0_.z * other_groups.group0_.z) - (self_.group1_.y * other_groups.group1_.y) - (self_.group1_.z * other_groups.group1_.z)) - (vec4<f32>(self_.group0_.zxy.xyz, self_.group1_.x) * other_groups.group1_.yzxx)
    ));
}
fn line_geometricAntiProduct_horizon(self_: Line, other: Horizon) -> Point {
    let self_groups = line_grouped(self_);
    let other_groups = horizon_grouped(other);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group0_.x * other.e321, self_.group0_.y * other.e321, self_.group0_.z * other.e321, 0.0)
    ));
}
fn line_geometricAntiProduct_line(self_: Line, other: Line) -> Motor {
    let self_groups = line_grouped(self_);
    let other_groups = line_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other_groups.group0_.z * self_.group0_.y) - (other_groups.group0_.y * self_.group0_.z), (other_groups.group0_.x * self_.group0_.z) - (other_groups.group0_.z * self_.group0_.x), (other_groups.group0_.y * self_.group0_.x) - (other_groups.group0_.x * self_.group0_.y), -(other_groups.group0_.x * self_.group0_.x) - (other_groups.group0_.y * self_.group0_.y) - (other_groups.group0_.z * self_.group0_.z)), 
        /* e23, e31, e12, scalar */ vec4<f32>((other_groups.group0_.z * self_.group1_.y) + (other_groups.group1_.z * self_.group0_.y) - (other_groups.group0_.y * self_.group1_.z) - (other_groups.group1_.y * self_.group0_.z), (other_groups.group0_.x * self_.group1_.z) + (other_groups.group1_.x * self_.group0_.z) - (other_groups.group0_.z * self_.group1_.x) - (other_groups.group1_.z * self_.group0_.x), (other_groups.group0_.y * self_.group1_.x) + (other_groups.group1_.y * self_.group0_.x) - (other_groups.group0_.x * self_.group1_.y) - (other_groups.group1_.x * self_.group0_.y), -(other_groups.group0_.x * self_.group1_.x) - (other_groups.group0_.y * self_.group1_.y) - (other_groups.group0_.z * self_.group1_.z) - (other_groups.group1_.x * self_.group0_.x) - (other_groups.group1_.y * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.z))
    ));
}
fn line_geometricAntiProduct_motor(self_: Line, other: Motor) -> Motor {
    let self_groups = line_grouped(self_);
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((self_.group0_.x * other_groups.group0_.w) + (self_.group0_.y * other_groups.group0_.z), (self_.group0_.y * other_groups.group0_.w) + (self_.group0_.z * other_groups.group0_.x), (self_.group0_.x * other_groups.group0_.y) + (self_.group0_.z * other_groups.group0_.w), -(self_.group0_.y * other_groups.group0_.y) - (self_.group0_.z * other_groups.group0_.z)) - (vec4<f32>(self_.group0_.zxy.xyz, self_.group0_.x) * other_groups.group0_.yzxx), 
        /* e23, e31, e12, scalar */ vec4<f32>((self_.group0_.x * other_groups.group1_.w) + (self_.group0_.y * other_groups.group1_.z) + (self_.group1_.x * other_groups.group0_.w) + (self_.group1_.y * other_groups.group0_.z), (self_.group0_.y * other_groups.group1_.w) + (self_.group0_.z * other_groups.group1_.x) + (self_.group1_.y * other_groups.group0_.w) + (self_.group1_.z * other_groups.group0_.x), (self_.group0_.x * other_groups.group1_.y) + (self_.group0_.z * other_groups.group1_.w) + (self_.group1_.x * other_groups.group0_.y) + (self_.group1_.z * other_groups.group0_.w), -(self_.group0_.y * other_groups.group1_.y) - (self_.group0_.z * other_groups.group1_.z) - (self_.group1_.y * other_groups.group0_.y) - (self_.group1_.z * other_groups.group0_.z)) - (vec4<f32>(self_.group0_.zxy.xyz, self_.group0_.x) * other_groups.group1_.yzxx) - (vec4<f32>(self_.group1_.zxy.xyz, self_.group1_.x) * other_groups.group0_.yzxx)
    ));
}
fn line_geometricAntiProduct_multiVector(self_: Line, other: MultiVector) -> MultiVector {
    let self_groups = line_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(-(self_.group1_.x * other_groups.group2_.x) - (self_.group1_.y * other_groups.group2_.y) - (self_.group1_.z * other_groups.group2_.z), 0.0, 0.0, 0.0) - (vec4<f32>(vec2<f32>(self_.group0_.x), 0.0, 0.0) * vec4<f32>(other_groups.group3_.x, other_groups.group2_.x, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(self_.group0_.y), 0.0, 0.0) * vec4<f32>(other_groups.group3_.y, other_groups.group2_.y, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(self_.group0_.z), 0.0, 0.0) * vec4<f32>(other_groups.group3_.z, other_groups.group2_.z, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group0_.x * other_groups.group4_.w) + (self_.group0_.y * other_groups.group1_.z) + (self_.group1_.x * other_groups.group1_.w) + (self_.group1_.y * other_groups.group4_.z) - (self_.group0_.z * other_groups.group1_.y), (self_.group0_.y * other_groups.group4_.w) + (self_.group0_.z * other_groups.group1_.x) + (self_.group1_.y * other_groups.group1_.w) + (self_.group1_.z * other_groups.group4_.x) - (self_.group0_.x * other_groups.group1_.z), (self_.group0_.x * other_groups.group1_.y) + (self_.group0_.z * other_groups.group4_.w) + (self_.group1_.x * other_groups.group4_.y) + (self_.group1_.z * other_groups.group1_.w) - (self_.group0_.y * other_groups.group1_.x), -(self_.group0_.y * other_groups.group4_.y) - (self_.group0_.z * other_groups.group4_.z)) - (vec4<f32>(self_.group1_.zxy.xyz, self_.group0_.x) * other_groups.group4_.yzxx), 
        /* e41, e42, e43 */ (vec4<f32>(vec3<f32>(other_groups.group0_.y), 0.0) * self_.group0_) + (self_.group0_.yzx * other_groups.group2_.zxy) - (self_.group0_.zxy * other_groups.group2_.yzx), 
        /* e23, e31, e12 */ (vec4<f32>(vec3<f32>(other_groups.group0_.x), 0.0) * self_.group0_) + (vec4<f32>(vec3<f32>(other_groups.group0_.y), 0.0) * self_.group1_) + (self_.group0_.yzx * other_groups.group3_.zxy) + (self_.group1_.yzx * other_groups.group2_.zxy) - (self_.group0_.zxy * other_groups.group3_.yzx) - (self_.group1_.zxy * other_groups.group2_.yzx), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.group0_.x * other_groups.group1_.w) + (self_.group0_.y * other_groups.group4_.z), (self_.group0_.y * other_groups.group1_.w) + (self_.group0_.z * other_groups.group4_.x), (self_.group0_.x * other_groups.group4_.y) + (self_.group0_.z * other_groups.group1_.w), -(self_.group0_.x * other_groups.group1_.x) - (self_.group0_.y * other_groups.group1_.y) - (self_.group0_.z * other_groups.group1_.z) - (self_.group1_.y * other_groups.group4_.y) - (self_.group1_.z * other_groups.group4_.z)) - (vec4<f32>(self_.group0_.zxy.xyz, self_.group1_.x) * other_groups.group4_.yzxx)
    ));
}
fn line_geometricAntiProduct_origin(self_: Line, other: Origin) -> Flector {
    let self_groups = line_grouped(self_);
    let other_groups = origin_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group1_.x * other.e4, self_.group1_.y * other.e4, self_.group1_.z * other.e4, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.x * other.e4, self_.group0_.y * other.e4, self_.group0_.z * other.e4, 0.0)
    ));
}
fn line_geometricAntiProduct_plane(self_: Line, other: Plane) -> Flector {
    let self_groups = line_grouped(self_);
    let other_groups = plane_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group0_.x * other_groups.group0_.w) + (self_.group1_.y * other_groups.group0_.z), (self_.group0_.y * other_groups.group0_.w) + (self_.group1_.z * other_groups.group0_.x), (self_.group0_.z * other_groups.group0_.w) + (self_.group1_.x * other_groups.group0_.y), -(self_.group0_.y * other_groups.group0_.y) - (self_.group0_.z * other_groups.group0_.z)) - (vec4<f32>(self_.group1_.zxy.xyz, self_.group0_.x) * other_groups.group0_.yzxx), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.y * other_groups.group0_.z, self_.group0_.z * other_groups.group0_.x, self_.group0_.x * other_groups.group0_.y, -(self_.group1_.y * other_groups.group0_.y) - (self_.group1_.z * other_groups.group0_.z)) - (vec4<f32>(self_.group0_.zxy.xyz, self_.group1_.x) * other_groups.group0_.yzxx)
    ));
}
fn line_geometricAntiProduct_point(self_: Line, other: Point) -> Flector {
    let self_groups = line_grouped(self_);
    let other_groups = point_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group0_.y * other_groups.group0_.z) + (self_.group1_.x * other_groups.group0_.w) - (self_.group0_.z * other_groups.group0_.y), (self_.group0_.z * other_groups.group0_.x) + (self_.group1_.y * other_groups.group0_.w) - (self_.group0_.x * other_groups.group0_.z), (self_.group0_.x * other_groups.group0_.y) + (self_.group1_.z * other_groups.group0_.w) - (self_.group0_.y * other_groups.group0_.x), 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.x * other_groups.group0_.w, self_.group0_.y * other_groups.group0_.w, self_.group0_.z * other_groups.group0_.w, -(self_.group0_.x * other_groups.group0_.x) - (self_.group0_.y * other_groups.group0_.y) - (self_.group0_.z * other_groups.group0_.z))
    ));
}
fn line_geometricAntiProduct_scalar(self_: Line, other: Scalar) -> Line {
    let self_groups = line_grouped(self_);
    let other_groups = scalar_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(other.scalar), 0.0) * self_.group0_
    ));
}
fn motor_geometricAntiProduct_antiScalar(self_: Motor, other: AntiScalar) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e1234) * self_.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(other.e1234) * self_.group1_
    ));
}
fn motor_geometricAntiProduct_dualNum(self_: Motor, other: DualNum) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other_groups.group0_.y) * self_.group0_, 
        /* e23, e31, e12, scalar */ (vec4<f32>(other_groups.group0_.x) * self_.group0_) + (vec4<f32>(other_groups.group0_.y) * self_.group1_)
    ));
}
fn motor_geometricAntiProduct_flector(self_: Motor, other: Flector) -> Flector {
    let self_groups = motor_grouped(self_);
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group0_.z * self_.group0_.y) + (other_groups.group0_.w * self_.group1_.x) + (other_groups.group1_.x * self_.group1_.w) + (other_groups.group1_.z * self_.group1_.y) + (other_groups.group1_.w * self_.group0_.x), (other_groups.group0_.y * self_.group0_.w) + (other_groups.group0_.w * self_.group1_.y) + (other_groups.group1_.x * self_.group1_.z) + (other_groups.group1_.y * self_.group1_.w) + (other_groups.group1_.w * self_.group0_.y), (other_groups.group0_.z * self_.group0_.w) + (other_groups.group0_.w * self_.group1_.z) + (other_groups.group1_.y * self_.group1_.x) + (other_groups.group1_.z * self_.group1_.w) + (other_groups.group1_.w * self_.group0_.z), (other_groups.group1_.z * self_.group0_.z) * -1.0) + (other_groups.group0_.xxyw * self_.group0_.wzxw) - (vec4<f32>(other_groups.group0_.y, other_groups.group0_.z, other_groups.group0_.x, other_groups.group1_.x) * self_.group0_.zxyx) - (vec4<f32>(self_.group1_.z, self_.group1_.x, self_.group1_.y, self_.group0_.y) * other_groups.group1_.yzxy), 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group1_.z * self_.group0_.y, other_groups.group1_.y * self_.group0_.w, other_groups.group1_.z * self_.group0_.w, -(other_groups.group0_.y * self_.group0_.y) - (other_groups.group0_.z * self_.group0_.z) - (other_groups.group1_.x * self_.group1_.x) - (other_groups.group1_.y * self_.group1_.y) - (other_groups.group1_.z * self_.group1_.z)) + (vec4<f32>(other_groups.group0_.w) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group1_.w)) + (other_groups.group1_.xxyw * self_.group0_.wzxw) - (vec4<f32>(other_groups.group1_.y, other_groups.group1_.z, other_groups.group1_.x, other_groups.group0_.x) * self_.group0_.zxyx)
    ));
}
fn motor_geometricAntiProduct_horizon(self_: Motor, other: Horizon) -> Flector {
    let self_groups = motor_grouped(self_);
    let other_groups = horizon_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group0_.x * other.e321, self_.group0_.y * other.e321, self_.group0_.z * other.e321, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.group0_.w * other.e321)
    ));
}
fn motor_geometricAntiProduct_line(self_: Motor, other: Line) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = line_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other_groups.group0_.x * self_.group0_.w) + (other_groups.group0_.z * self_.group0_.y), (other_groups.group0_.x * self_.group0_.z) + (other_groups.group0_.y * self_.group0_.w), (other_groups.group0_.y * self_.group0_.x) + (other_groups.group0_.z * self_.group0_.w), -(other_groups.group0_.y * self_.group0_.y) - (other_groups.group0_.z * self_.group0_.z)) - (vec4<f32>(other_groups.group0_.yzx.xyz, other_groups.group0_.x) * self_.group0_.zxyx), 
        /* e23, e31, e12, scalar */ vec4<f32>((other_groups.group0_.x * self_.group1_.w) + (other_groups.group0_.z * self_.group1_.y) + (other_groups.group1_.x * self_.group0_.w) + (other_groups.group1_.z * self_.group0_.y), (other_groups.group0_.x * self_.group1_.z) + (other_groups.group0_.y * self_.group1_.w) + (other_groups.group1_.x * self_.group0_.z) + (other_groups.group1_.y * self_.group0_.w), (other_groups.group0_.y * self_.group1_.x) + (other_groups.group0_.z * self_.group1_.w) + (other_groups.group1_.y * self_.group0_.x) + (other_groups.group1_.z * self_.group0_.w), -(other_groups.group0_.y * self_.group1_.y) - (other_groups.group0_.z * self_.group1_.z) - (other_groups.group1_.y * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.z)) - (vec4<f32>(other_groups.group0_.yzx.xyz, other_groups.group0_.x) * self_.group1_.zxyx) - (vec4<f32>(other_groups.group1_.yzx.xyz, other_groups.group1_.x) * self_.group0_.zxyx)
    ));
}
fn motor_geometricAntiProduct_motor(self_: Motor, other: Motor) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other_groups.group0_.z * self_.group0_.y) + (other_groups.group0_.w * self_.group0_.x), (other_groups.group0_.y * self_.group0_.w) + (other_groups.group0_.w * self_.group0_.y), (other_groups.group0_.z * self_.group0_.w) + (other_groups.group0_.w * self_.group0_.z), -(other_groups.group0_.y * self_.group0_.y) - (other_groups.group0_.z * self_.group0_.z)) + (other_groups.group0_.xxyw * self_.group0_.wzxw) - (other_groups.group0_.yzxx * self_.group0_.zxyx), 
        /* e23, e31, e12, scalar */ vec4<f32>((other_groups.group0_.z * self_.group1_.y) + (other_groups.group0_.w * self_.group1_.x) + (other_groups.group1_.z * self_.group0_.y) + (other_groups.group1_.w * self_.group0_.x), (other_groups.group0_.y * self_.group1_.w) + (other_groups.group0_.w * self_.group1_.y) + (other_groups.group1_.y * self_.group0_.w) + (other_groups.group1_.w * self_.group0_.y), (other_groups.group0_.z * self_.group1_.w) + (other_groups.group0_.w * self_.group1_.z) + (other_groups.group1_.z * self_.group0_.w) + (other_groups.group1_.w * self_.group0_.z), -(other_groups.group0_.y * self_.group1_.y) - (other_groups.group0_.z * self_.group1_.z) - (other_groups.group1_.y * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.z)) + (other_groups.group0_.xxyw * self_.group1_.wzxw) + (other_groups.group1_.xxyw * self_.group0_.wzxw) - (other_groups.group0_.yzxx * self_.group1_.zxyx) - (other_groups.group1_.yzxx * self_.group0_.zxyx)
    ));
}
fn motor_geometricAntiProduct_multiVector(self_: Motor, other: MultiVector) -> MultiVector {
    let self_groups = motor_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((other_groups.group0_.y * self_.group1_.w) - (other_groups.group3_.x * self_.group0_.x) - (other_groups.group3_.y * self_.group0_.y) - (other_groups.group3_.z * self_.group0_.z), 0.0, 0.0, 0.0) + (vec4<f32>(vec2<f32>(self_.group0_.w), 0.0, 0.0) * other_groups.group0_) - (vec4<f32>(vec2<f32>(other_groups.group2_.x), 0.0, 0.0) * vec4<f32>(self_.group1_.x, self_.group0_.x, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(other_groups.group2_.y), 0.0, 0.0) * vec4<f32>(self_.group1_.y, self_.group0_.y, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(other_groups.group2_.z), 0.0, 0.0) * vec4<f32>(self_.group1_.z, self_.group0_.z, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group0_.y * other_groups.group1_.z) + (self_.group0_.w * other_groups.group1_.x) + (self_.group1_.x * other_groups.group1_.w) + (self_.group1_.y * other_groups.group4_.z) + (self_.group1_.w * other_groups.group4_.x), (self_.group0_.z * other_groups.group1_.x) + (self_.group0_.w * other_groups.group1_.y) + (self_.group1_.y * other_groups.group1_.w) + (self_.group1_.z * other_groups.group4_.x) + (self_.group1_.w * other_groups.group4_.y), (self_.group0_.z * other_groups.group4_.w) + (self_.group0_.w * other_groups.group1_.z) + (self_.group1_.x * other_groups.group4_.y) + (self_.group1_.z * other_groups.group1_.w) + (self_.group1_.w * other_groups.group4_.z), (self_.group0_.z * other_groups.group4_.z) * -1.0) + (vec4<f32>(other_groups.group4_.w, other_groups.group4_.w, other_groups.group1_.y, other_groups.group1_.w) * self_.group0_.xyxw) - (vec4<f32>(self_.group1_.z, self_.group1_.x, self_.group1_.y, self_.group0_.y) * other_groups.group4_.yzxy) - (vec4<f32>(other_groups.group1_.y, other_groups.group1_.z, other_groups.group1_.x, other_groups.group4_.x) * self_.group0_.zxyx), 
        /* e41, e42, e43 */ (vec4<f32>(vec3<f32>(other_groups.group0_.y), 0.0) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0)) + (vec4<f32>(self_.group0_.y, self_.group0_.w, self_.group0_.w, 0.0) * other_groups.group2_.zyz) + (vec4<f32>(self_.group0_.w, self_.group0_.z, self_.group0_.x, 0.0) * other_groups.group2_.xxy) - (vec4<f32>(self_.group0_.z, self_.group0_.x, self_.group0_.y, 0.0) * other_groups.group2_.yzx), 
        /* e23, e31, e12 */ (vec4<f32>(vec3<f32>(other_groups.group0_.x), 0.0) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0)) + (vec4<f32>(vec3<f32>(other_groups.group0_.y), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0)) + (vec4<f32>(self_.group0_.y, self_.group0_.w, self_.group0_.w, 0.0) * other_groups.group3_.zyz) + (vec4<f32>(self_.group0_.w, self_.group0_.z, self_.group0_.x, 0.0) * other_groups.group3_.xxy) + (vec4<f32>(self_.group1_.y, self_.group1_.w, self_.group1_.w, 0.0) * other_groups.group2_.zyz) + (vec4<f32>(self_.group1_.w, self_.group1_.z, self_.group1_.x, 0.0) * other_groups.group2_.xxy) - (vec4<f32>(self_.group0_.z, self_.group0_.x, self_.group0_.y, 0.0) * other_groups.group3_.yzx) - (vec4<f32>(self_.group1_.z, self_.group1_.x, self_.group1_.y, 0.0) * other_groups.group2_.yzx), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.group0_.y * other_groups.group4_.z) + (self_.group0_.w * other_groups.group4_.x), (self_.group0_.z * other_groups.group4_.x) + (self_.group0_.w * other_groups.group4_.y), (self_.group0_.z * other_groups.group1_.w) + (self_.group0_.w * other_groups.group4_.z), (self_.group1_.w * other_groups.group1_.w) - (self_.group0_.y * other_groups.group1_.y) - (self_.group0_.z * other_groups.group1_.z) - (self_.group1_.x * other_groups.group4_.x) - (self_.group1_.y * other_groups.group4_.y) - (self_.group1_.z * other_groups.group4_.z)) + (vec4<f32>(other_groups.group1_.w, other_groups.group1_.w, other_groups.group4_.y, other_groups.group4_.w) * self_.group0_.xyxw) - (vec4<f32>(other_groups.group4_.y, other_groups.group4_.z, other_groups.group4_.x, other_groups.group1_.x) * self_.group0_.zxyx)
    ));
}
fn motor_geometricAntiProduct_origin(self_: Motor, other: Origin) -> Flector {
    let self_groups = motor_grouped(self_);
    let other_groups = origin_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e4) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group0_.w), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e4) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group1_.w)
    ));
}
fn motor_geometricAntiProduct_plane(self_: Motor, other: Plane) -> Flector {
    let self_groups = motor_grouped(self_);
    let other_groups = plane_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group0_.x * other_groups.group0_.w) + (self_.group1_.y * other_groups.group0_.z) + (self_.group1_.w * other_groups.group0_.x), (self_.group0_.y * other_groups.group0_.w) + (self_.group1_.z * other_groups.group0_.x) + (self_.group1_.w * other_groups.group0_.y), (self_.group0_.z * other_groups.group0_.w) + (self_.group1_.x * other_groups.group0_.y) + (self_.group1_.w * other_groups.group0_.z), -(self_.group0_.y * other_groups.group0_.y) - (self_.group0_.z * other_groups.group0_.z)) - (vec4<f32>(self_.group1_.z, self_.group1_.x, self_.group1_.y, self_.group0_.x) * other_groups.group0_.yzxx), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.w * other_groups.group0_.x, self_.group0_.w * other_groups.group0_.y, self_.group0_.w * other_groups.group0_.z, -(self_.group1_.y * other_groups.group0_.y) - (self_.group1_.z * other_groups.group0_.z)) + (self_.group0_.yzxw * other_groups.group0_.zxyw) - (vec4<f32>(self_.group0_.z, self_.group0_.x, self_.group0_.y, self_.group1_.x) * other_groups.group0_.yzxx)
    ));
}
fn motor_geometricAntiProduct_point(self_: Motor, other: Point) -> Flector {
    let self_groups = motor_grouped(self_);
    let other_groups = point_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group0_.y * other_groups.group0_.z) + (self_.group0_.w * other_groups.group0_.x) + (self_.group1_.x * other_groups.group0_.w) - (self_.group0_.z * other_groups.group0_.y), (self_.group0_.z * other_groups.group0_.x) + (self_.group0_.w * other_groups.group0_.y) + (self_.group1_.y * other_groups.group0_.w) - (self_.group0_.x * other_groups.group0_.z), (self_.group0_.x * other_groups.group0_.y) + (self_.group0_.w * other_groups.group0_.z) + (self_.group1_.z * other_groups.group0_.w) - (self_.group0_.y * other_groups.group0_.x), self_.group0_.w * other_groups.group0_.w), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.x * other_groups.group0_.w, self_.group0_.y * other_groups.group0_.w, self_.group0_.z * other_groups.group0_.w, (self_.group1_.w * other_groups.group0_.w) - (self_.group0_.x * other_groups.group0_.x) - (self_.group0_.y * other_groups.group0_.y) - (self_.group0_.z * other_groups.group0_.z))
    ));
}
fn motor_geometricAntiProduct_scalar(self_: Motor, other: Scalar) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = scalar_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(other.scalar) * self_.group0_
    ));
}
fn multiVector_geometricAntiProduct_antiScalar(self_: MultiVector, other: AntiScalar) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(other.e1234), 0.0, 0.0) * self_.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(other.e1234) * self_.group1_, 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(other.e1234), 0.0) * self_.group2_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(other.e1234), 0.0) * self_.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e1234) * self_.group4_
    ));
}
fn multiVector_geometricAntiProduct_dualNum(self_: MultiVector, other: DualNum) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((other_groups.group0_.x * self_.group0_.y) + (other_groups.group0_.y * self_.group0_.x), other_groups.group0_.y * self_.group0_.y, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group0_.y * self_.group1_.x) - (other_groups.group0_.x * self_.group4_.x), (other_groups.group0_.y * self_.group1_.y) - (other_groups.group0_.x * self_.group4_.y), (other_groups.group0_.y * self_.group1_.z) - (other_groups.group0_.x * self_.group4_.z), other_groups.group0_.y * self_.group1_.w), 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(other_groups.group0_.y), 0.0) * self_.group2_, 
        /* e23, e31, e12 */ (vec4<f32>(vec3<f32>(other_groups.group0_.x), 0.0) * self_.group2_) + (vec4<f32>(vec3<f32>(other_groups.group0_.y), 0.0) * self_.group3_), 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group0_.y * self_.group4_.x, other_groups.group0_.y * self_.group4_.y, other_groups.group0_.y * self_.group4_.z, (other_groups.group0_.y * self_.group4_.w) - (other_groups.group0_.x * self_.group1_.w))
    ));
}
fn multiVector_geometricAntiProduct_flector(self_: MultiVector, other: Flector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = flector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((other_groups.group1_.w * self_.group1_.w) - (other_groups.group0_.x * self_.group4_.x) - (other_groups.group0_.y * self_.group4_.y) - (other_groups.group0_.z * self_.group4_.z), 0.0, 0.0, 0.0) + (vec4<f32>(vec2<f32>(other_groups.group1_.x), 0.0, 0.0) * vec4<f32>(self_.group1_.x, self_.group4_.x, 0.0, 0.0)) + (vec4<f32>(vec2<f32>(other_groups.group1_.y), 0.0, 0.0) * vec4<f32>(self_.group1_.y, self_.group4_.y, 0.0, 0.0)) + (vec4<f32>(vec2<f32>(other_groups.group1_.z), 0.0, 0.0) * vec4<f32>(self_.group1_.z, self_.group4_.z, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(other_groups.group0_.w), 0.0, 0.0) * vec4<f32>(self_.group4_.w, self_.group1_.w, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group0_.x * other_groups.group1_.x) + (self_.group2_.x * other_groups.group1_.w) + (self_.group2_.y * other_groups.group0_.z) + (self_.group3_.x * other_groups.group0_.w) + (self_.group3_.y * other_groups.group1_.z) - (self_.group2_.z * other_groups.group0_.y), (self_.group0_.x * other_groups.group1_.y) + (self_.group2_.y * other_groups.group1_.w) + (self_.group2_.z * other_groups.group0_.x) + (self_.group3_.y * other_groups.group0_.w) + (self_.group3_.z * other_groups.group1_.x) - (self_.group2_.x * other_groups.group0_.z), (self_.group0_.x * other_groups.group1_.z) + (self_.group2_.x * other_groups.group0_.y) + (self_.group2_.z * other_groups.group1_.w) + (self_.group3_.x * other_groups.group1_.y) + (self_.group3_.z * other_groups.group0_.w) - (self_.group2_.y * other_groups.group0_.x), -(self_.group2_.y * other_groups.group1_.y) - (self_.group2_.z * other_groups.group1_.z)) + (vec4<f32>(self_.group0_.y) * other_groups.group0_) - (vec4<f32>(self_.group3_.zxy.xyz, self_.group2_.x) * other_groups.group1_.yzxx), 
        /* e41, e42, e43 */ vec4<f32>((other_groups.group1_.y * self_.group4_.z) - (other_groups.group1_.z * self_.group4_.y), (other_groups.group1_.z * self_.group4_.x) - (other_groups.group1_.x * self_.group4_.z), (other_groups.group1_.x * self_.group4_.y) - (other_groups.group1_.y * self_.group4_.x), 0.0) - (vec4<f32>(vec3<f32>(other_groups.group0_.w), 0.0) * vec4<f32>(self_.group4_.x, self_.group4_.y, self_.group4_.z, 0.0)) - (vec4<f32>(vec3<f32>(self_.group1_.w), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((other_groups.group0_.z * self_.group4_.y) + (other_groups.group1_.y * self_.group1_.z) - (other_groups.group0_.y * self_.group4_.z) - (other_groups.group1_.z * self_.group1_.y), (other_groups.group0_.x * self_.group4_.z) + (other_groups.group1_.z * self_.group1_.x) - (other_groups.group0_.z * self_.group4_.x) - (other_groups.group1_.x * self_.group1_.z), (other_groups.group0_.y * self_.group4_.x) + (other_groups.group1_.x * self_.group1_.y) - (other_groups.group0_.x * self_.group4_.y) - (other_groups.group1_.y * self_.group1_.x), 0.0) + (vec4<f32>(vec3<f32>(other_groups.group1_.w), 0.0) * vec4<f32>(self_.group4_.x, self_.group4_.y, self_.group4_.z, 0.0)) + (vec4<f32>(vec3<f32>(self_.group1_.w), 0.0) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0)) - (vec4<f32>(vec3<f32>(other_groups.group0_.w), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0)) - (vec4<f32>(vec3<f32>(self_.group4_.w), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group2_.y * other_groups.group1_.z, self_.group2_.z * other_groups.group1_.x, self_.group2_.x * other_groups.group1_.y, -(self_.group2_.x * other_groups.group0_.x) - (self_.group2_.y * other_groups.group0_.y) - (self_.group2_.z * other_groups.group0_.z) - (self_.group3_.y * other_groups.group1_.y) - (self_.group3_.z * other_groups.group1_.z)) + (vec4<f32>(self_.group0_.y) * other_groups.group1_) + (vec4<f32>(other_groups.group0_.w) * vec4<f32>(self_.group2_.xyz.xyz, self_.group0_.x)) - (vec4<f32>(self_.group2_.zxy.xyz, self_.group3_.x) * other_groups.group1_.yzxx)
    ));
}
fn multiVector_geometricAntiProduct_horizon(self_: MultiVector, other: Horizon) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = horizon_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.group1_.w * other.e321, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group2_.x * other.e321, self_.group2_.y * other.e321, self_.group2_.z * other.e321, 0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(other.e321), 0.0) * vec4<f32>(self_.group4_.x, self_.group4_.y, self_.group4_.z, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.group0_.y * other.e321)
    ));
}
fn multiVector_geometricAntiProduct_line(self_: MultiVector, other: Line) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = line_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(-(other_groups.group1_.x * self_.group2_.x) - (other_groups.group1_.y * self_.group2_.y) - (other_groups.group1_.z * self_.group2_.z), 0.0, 0.0, 0.0) - (vec4<f32>(vec2<f32>(other_groups.group0_.x), 0.0, 0.0) * vec4<f32>(self_.group3_.x, self_.group2_.x, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(other_groups.group0_.y), 0.0, 0.0) * vec4<f32>(self_.group3_.y, self_.group2_.y, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(other_groups.group0_.z), 0.0, 0.0) * vec4<f32>(self_.group3_.z, self_.group2_.z, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group0_.x * self_.group4_.w) + (other_groups.group0_.z * self_.group1_.y) + (other_groups.group1_.y * self_.group4_.z) - (other_groups.group0_.y * self_.group1_.z) - (other_groups.group1_.x * self_.group1_.w), (other_groups.group0_.x * self_.group1_.z) + (other_groups.group0_.y * self_.group4_.w) + (other_groups.group1_.z * self_.group4_.x) - (other_groups.group0_.z * self_.group1_.x) - (other_groups.group1_.y * self_.group1_.w), (other_groups.group0_.y * self_.group1_.x) + (other_groups.group0_.z * self_.group4_.w) + (other_groups.group1_.x * self_.group4_.y) - (other_groups.group0_.x * self_.group1_.y) - (other_groups.group1_.z * self_.group1_.w), -(other_groups.group0_.y * self_.group4_.y) - (other_groups.group0_.z * self_.group4_.z)) - (vec4<f32>(other_groups.group1_.zxy.xyz, other_groups.group0_.x) * self_.group4_.yzxx), 
        /* e41, e42, e43 */ (vec4<f32>(vec3<f32>(self_.group0_.y), 0.0) * other_groups.group0_) + (other_groups.group0_.zxy * self_.group2_.yzx) - (other_groups.group0_.yzx * self_.group2_.zxy), 
        /* e23, e31, e12 */ (vec4<f32>(vec3<f32>(self_.group0_.x), 0.0) * other_groups.group0_) + (vec4<f32>(vec3<f32>(self_.group0_.y), 0.0) * other_groups.group1_) + (other_groups.group0_.zxy * self_.group3_.yzx) + (other_groups.group1_.zxy * self_.group2_.yzx) - (other_groups.group0_.yzx * self_.group3_.zxy) - (other_groups.group1_.yzx * self_.group2_.zxy), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.x * self_.group1_.w) - (other_groups.group0_.y * self_.group4_.z), (other_groups.group0_.y * self_.group1_.w) - (other_groups.group0_.z * self_.group4_.x), (other_groups.group0_.z * self_.group1_.w) - (other_groups.group0_.x * self_.group4_.y), (other_groups.group1_.y * self_.group4_.y) + (other_groups.group1_.z * self_.group4_.z) - (other_groups.group0_.x * self_.group1_.x) - (other_groups.group0_.y * self_.group1_.y) - (other_groups.group0_.z * self_.group1_.z)) + (vec4<f32>(other_groups.group0_.zxy.xyz, other_groups.group1_.x) * self_.group4_.yzxx)
    ));
}
fn multiVector_geometricAntiProduct_motor(self_: MultiVector, other: Motor) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = motor_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((self_.group0_.y * other_groups.group1_.w) - (self_.group3_.x * other_groups.group0_.x) - (self_.group3_.y * other_groups.group0_.y) - (self_.group3_.z * other_groups.group0_.z), 0.0, 0.0, 0.0) + (vec4<f32>(vec2<f32>(other_groups.group0_.w), 0.0, 0.0) * self_.group0_) - (vec4<f32>(vec2<f32>(self_.group2_.x), 0.0, 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group0_.x, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(self_.group2_.y), 0.0, 0.0) * vec4<f32>(other_groups.group1_.y, other_groups.group0_.y, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(self_.group2_.z), 0.0, 0.0) * vec4<f32>(other_groups.group1_.z, other_groups.group0_.z, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group0_.z * self_.group1_.y) + (other_groups.group0_.w * self_.group1_.x) + (other_groups.group1_.y * self_.group4_.z) - (other_groups.group1_.x * self_.group1_.w), (other_groups.group0_.y * self_.group4_.w) + (other_groups.group0_.w * self_.group1_.y) + (other_groups.group1_.z * self_.group4_.x) - (other_groups.group1_.y * self_.group1_.w), (other_groups.group0_.z * self_.group4_.w) + (other_groups.group0_.w * self_.group1_.z) + (other_groups.group1_.x * self_.group4_.y) - (other_groups.group1_.z * self_.group1_.w), 0.0) + (vec4<f32>(self_.group4_.w, self_.group1_.z, self_.group1_.x, self_.group1_.w) * other_groups.group0_.xxyw) - (vec4<f32>(other_groups.group1_.z, other_groups.group1_.x, other_groups.group1_.y, other_groups.group0_.y) * self_.group4_.yzxy) - (vec4<f32>(other_groups.group1_.w, other_groups.group1_.w, other_groups.group1_.w, other_groups.group0_.z) * self_.group4_.xyzz) - (vec4<f32>(self_.group1_.z, self_.group1_.x, self_.group1_.y, self_.group4_.x) * other_groups.group0_.yzxx), 
        /* e41, e42, e43 */ (vec4<f32>(vec3<f32>(self_.group0_.y), 0.0) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0)) + (vec4<f32>(other_groups.group0_.z, other_groups.group0_.x, other_groups.group0_.w, 0.0) * self_.group2_.yzz) + (vec4<f32>(other_groups.group0_.w, other_groups.group0_.w, other_groups.group0_.y, 0.0) * self_.group2_.xyx) - (vec4<f32>(other_groups.group0_.y, other_groups.group0_.z, other_groups.group0_.x, 0.0) * self_.group2_.zxy), 
        /* e23, e31, e12 */ (vec4<f32>(vec3<f32>(self_.group0_.x), 0.0) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0)) + (vec4<f32>(vec3<f32>(self_.group0_.y), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0)) + (vec4<f32>(other_groups.group0_.z, other_groups.group0_.x, other_groups.group0_.w, 0.0) * self_.group3_.yzz) + (vec4<f32>(other_groups.group0_.w, other_groups.group0_.w, other_groups.group0_.y, 0.0) * self_.group3_.xyx) + (vec4<f32>(other_groups.group1_.z, other_groups.group1_.x, other_groups.group1_.w, 0.0) * self_.group2_.yzz) + (vec4<f32>(other_groups.group1_.w, other_groups.group1_.w, other_groups.group1_.y, 0.0) * self_.group2_.xyx) - (vec4<f32>(other_groups.group0_.y, other_groups.group0_.z, other_groups.group0_.x, 0.0) * self_.group3_.zxy) - (vec4<f32>(other_groups.group1_.y, other_groups.group1_.z, other_groups.group1_.x, 0.0) * self_.group2_.zxy), 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group0_.w * self_.group4_.x, other_groups.group0_.y * self_.group1_.w, other_groups.group0_.z * self_.group1_.w, (other_groups.group1_.y * self_.group4_.y) + (other_groups.group1_.z * self_.group4_.z) - (other_groups.group0_.y * self_.group1_.y) - (other_groups.group0_.z * self_.group1_.z) - (other_groups.group1_.w * self_.group1_.w)) + (vec4<f32>(other_groups.group0_.z, other_groups.group0_.w, other_groups.group0_.w, other_groups.group1_.x) * self_.group4_.yyzx) + (vec4<f32>(self_.group1_.w, self_.group4_.z, self_.group4_.x, self_.group4_.w) * other_groups.group0_.xxyw) - (vec4<f32>(self_.group4_.z, self_.group4_.x, self_.group4_.y, self_.group1_.x) * other_groups.group0_.yzxx)
    ));
}
fn multiVector_geometricAntiProduct_multiVector(self_: MultiVector, other: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((other_groups.group0_.y * self_.group0_.x) + (other_groups.group4_.w * self_.group1_.w) - (other_groups.group3_.x * self_.group2_.x) - (other_groups.group3_.y * self_.group2_.y) - (other_groups.group3_.z * self_.group2_.z) - (other_groups.group1_.x * self_.group4_.x) - (other_groups.group1_.y * self_.group4_.y) - (other_groups.group1_.z * self_.group4_.z), 0.0, 0.0, 0.0) + (vec4<f32>(vec2<f32>(self_.group0_.y), 0.0, 0.0) * other_groups.group0_) + (vec4<f32>(vec2<f32>(other_groups.group4_.x), 0.0, 0.0) * vec4<f32>(self_.group1_.x, self_.group4_.x, 0.0, 0.0)) + (vec4<f32>(vec2<f32>(other_groups.group4_.y), 0.0, 0.0) * vec4<f32>(self_.group1_.y, self_.group4_.y, 0.0, 0.0)) + (vec4<f32>(vec2<f32>(other_groups.group4_.z), 0.0, 0.0) * vec4<f32>(self_.group1_.z, self_.group4_.z, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(other_groups.group2_.x), 0.0, 0.0) * vec4<f32>(self_.group3_.x, self_.group2_.x, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(other_groups.group2_.y), 0.0, 0.0) * vec4<f32>(self_.group3_.y, self_.group2_.y, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(other_groups.group2_.z), 0.0, 0.0) * vec4<f32>(self_.group3_.z, self_.group2_.z, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(other_groups.group1_.w), 0.0, 0.0) * vec4<f32>(self_.group4_.w, self_.group1_.w, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group0_.x * other_groups.group4_.x) + (other_groups.group2_.x * self_.group4_.w) + (other_groups.group2_.z * self_.group1_.y) + (other_groups.group3_.y * self_.group4_.z) + (self_.group2_.x * other_groups.group4_.w) + (self_.group2_.y * other_groups.group1_.z) + (self_.group3_.x * other_groups.group1_.w) + (self_.group3_.y * other_groups.group4_.z) - (other_groups.group2_.y * self_.group1_.z) - (other_groups.group3_.x * self_.group1_.w) - (self_.group2_.z * other_groups.group1_.y), (self_.group0_.x * other_groups.group4_.y) + (other_groups.group2_.x * self_.group1_.z) + (other_groups.group2_.y * self_.group4_.w) + (other_groups.group3_.z * self_.group4_.x) + (self_.group2_.y * other_groups.group4_.w) + (self_.group2_.z * other_groups.group1_.x) + (self_.group3_.y * other_groups.group1_.w) + (self_.group3_.z * other_groups.group4_.x) - (other_groups.group2_.z * self_.group1_.x) - (other_groups.group3_.y * self_.group1_.w) - (self_.group2_.x * other_groups.group1_.z), (self_.group0_.x * other_groups.group4_.z) + (other_groups.group2_.y * self_.group1_.x) + (other_groups.group2_.z * self_.group4_.w) + (other_groups.group3_.x * self_.group4_.y) + (self_.group2_.x * other_groups.group1_.y) + (self_.group2_.z * other_groups.group4_.w) + (self_.group3_.x * other_groups.group4_.y) + (self_.group3_.z * other_groups.group1_.w) - (other_groups.group2_.x * self_.group1_.y) - (other_groups.group3_.z * self_.group1_.w) - (self_.group2_.y * other_groups.group1_.x), -(other_groups.group2_.z * self_.group4_.z) - (self_.group2_.y * other_groups.group4_.y) - (self_.group2_.z * other_groups.group4_.z)) + (vec4<f32>(other_groups.group0_.y) * self_.group1_) + (vec4<f32>(self_.group0_.y) * other_groups.group1_) - (vec4<f32>(other_groups.group0_.xx.xy, other_groups.group0_.x, other_groups.group2_.x) * self_.group4_.xyzx) - (vec4<f32>(other_groups.group3_.zxy.xyz, other_groups.group2_.y) * self_.group4_.yzxy) - (vec4<f32>(self_.group3_.zxy.xyz, self_.group2_.x) * other_groups.group4_.yzxx), 
        /* e41, e42, e43 */ vec4<f32>((other_groups.group4_.y * self_.group4_.z) - (other_groups.group4_.z * self_.group4_.y), (other_groups.group4_.z * self_.group4_.x) - (other_groups.group4_.x * self_.group4_.z), (other_groups.group4_.x * self_.group4_.y) - (other_groups.group4_.y * self_.group4_.x), 0.0) + (vec4<f32>(vec3<f32>(other_groups.group0_.y), 0.0) * self_.group2_) + (vec4<f32>(vec3<f32>(self_.group0_.y), 0.0) * other_groups.group2_) + (other_groups.group2_.zxy * self_.group2_.yzx) - (vec4<f32>(vec3<f32>(other_groups.group1_.w), 0.0) * vec4<f32>(self_.group4_.x, self_.group4_.y, self_.group4_.z, 0.0)) - (vec4<f32>(vec3<f32>(self_.group1_.w), 0.0) * vec4<f32>(other_groups.group4_.x, other_groups.group4_.y, other_groups.group4_.z, 0.0)) - (other_groups.group2_.yzx * self_.group2_.zxy), 
        /* e23, e31, e12 */ vec4<f32>((other_groups.group1_.z * self_.group4_.y) + (other_groups.group4_.y * self_.group1_.z) - (other_groups.group1_.y * self_.group4_.z) - (other_groups.group4_.z * self_.group1_.y), (other_groups.group1_.x * self_.group4_.z) + (other_groups.group4_.z * self_.group1_.x) - (other_groups.group1_.z * self_.group4_.x) - (other_groups.group4_.x * self_.group1_.z), (other_groups.group1_.y * self_.group4_.x) + (other_groups.group4_.x * self_.group1_.y) - (other_groups.group1_.x * self_.group4_.y) - (other_groups.group4_.y * self_.group1_.x), 0.0) + (vec4<f32>(vec3<f32>(other_groups.group0_.x), 0.0) * self_.group2_) + (vec4<f32>(vec3<f32>(other_groups.group0_.y), 0.0) * self_.group3_) + (vec4<f32>(vec3<f32>(self_.group0_.x), 0.0) * other_groups.group2_) + (vec4<f32>(vec3<f32>(self_.group0_.y), 0.0) * other_groups.group3_) + (vec4<f32>(vec3<f32>(other_groups.group4_.w), 0.0) * vec4<f32>(self_.group4_.x, self_.group4_.y, self_.group4_.z, 0.0)) + (vec4<f32>(vec3<f32>(self_.group1_.w), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0)) + (other_groups.group2_.zxy * self_.group3_.yzx) + (other_groups.group3_.zxy * self_.group2_.yzx) - (vec4<f32>(vec3<f32>(other_groups.group1_.w), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0)) - (vec4<f32>(vec3<f32>(self_.group4_.w), 0.0) * vec4<f32>(other_groups.group4_.x, other_groups.group4_.y, other_groups.group4_.z, 0.0)) - (other_groups.group2_.yzx * self_.group3_.zxy) - (other_groups.group3_.yzx * self_.group2_.zxy), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group2_.x * self_.group1_.w) + (self_.group2_.y * other_groups.group4_.z) - (other_groups.group2_.y * self_.group4_.z), (other_groups.group2_.y * self_.group1_.w) + (self_.group2_.z * other_groups.group4_.x) - (other_groups.group2_.z * self_.group4_.x), (other_groups.group2_.z * self_.group1_.w) + (self_.group2_.x * other_groups.group4_.y) - (other_groups.group2_.x * self_.group4_.y), (other_groups.group3_.y * self_.group4_.y) + (other_groups.group3_.z * self_.group4_.z) - (other_groups.group0_.x * self_.group1_.w) - (other_groups.group2_.x * self_.group1_.x) - (other_groups.group2_.y * self_.group1_.y) - (other_groups.group2_.z * self_.group1_.z) - (self_.group2_.x * other_groups.group1_.x) - (self_.group2_.y * other_groups.group1_.y) - (self_.group2_.z * other_groups.group1_.z) - (self_.group3_.y * other_groups.group4_.y) - (self_.group3_.z * other_groups.group4_.z)) + (vec4<f32>(other_groups.group0_.y) * self_.group4_) + (vec4<f32>(self_.group0_.y) * other_groups.group4_) + (vec4<f32>(other_groups.group1_.w) * vec4<f32>(self_.group2_.xyz.xyz, self_.group0_.x)) + (vec4<f32>(other_groups.group2_.zxy.xyz, other_groups.group3_.x) * self_.group4_.yzxx) - (vec4<f32>(self_.group2_.zxy.xyz, self_.group3_.x) * other_groups.group4_.yzxx)
    ));
}
fn multiVector_geometricAntiProduct_origin(self_: MultiVector, other: Origin) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = origin_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(other.e4), 0.0, 0.0) * vec4<f32>(self_.group4_.w, self_.group1_.w, 0.0, 0.0) * vec2<f32>(-1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(other.e4) * vec4<f32>(self_.group3_.xyz.xyz, self_.group0_.y), 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(other.e4), 0.0) * vec4<f32>(self_.group4_.x, self_.group4_.y, self_.group4_.z, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(other.e4), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0) * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e4) * vec4<f32>(self_.group2_.xyz.xyz, self_.group0_.x)
    ));
}
fn multiVector_geometricAntiProduct_plane(self_: MultiVector, other: Plane) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = plane_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.group1_.w * other_groups.group0_.w, 0.0, 0.0, 0.0) + (vec4<f32>(vec2<f32>(other_groups.group0_.x), 0.0, 0.0) * vec4<f32>(self_.group1_.x, self_.group4_.x, 0.0, 0.0)) + (vec4<f32>(vec2<f32>(other_groups.group0_.y), 0.0, 0.0) * vec4<f32>(self_.group1_.y, self_.group4_.y, 0.0, 0.0)) + (vec4<f32>(vec2<f32>(other_groups.group0_.z), 0.0, 0.0) * vec4<f32>(self_.group1_.z, self_.group4_.z, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group0_.x * other_groups.group0_.x) + (self_.group2_.x * other_groups.group0_.w) + (self_.group3_.y * other_groups.group0_.z), (self_.group0_.x * other_groups.group0_.y) + (self_.group2_.y * other_groups.group0_.w) + (self_.group3_.z * other_groups.group0_.x), (self_.group0_.x * other_groups.group0_.z) + (self_.group2_.z * other_groups.group0_.w) + (self_.group3_.x * other_groups.group0_.y), -(self_.group2_.y * other_groups.group0_.y) - (self_.group2_.z * other_groups.group0_.z)) - (vec4<f32>(self_.group3_.zxy.xyz, self_.group2_.x) * other_groups.group0_.yzxx), 
        /* e41, e42, e43 */ vec4<f32>((self_.group4_.z * other_groups.group0_.y) - (self_.group4_.y * other_groups.group0_.z), (self_.group4_.x * other_groups.group0_.z) - (self_.group4_.z * other_groups.group0_.x), (self_.group4_.y * other_groups.group0_.x) - (self_.group4_.x * other_groups.group0_.y), 0.0) - (vec4<f32>(vec3<f32>(self_.group1_.w), 0.0) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((self_.group1_.z * other_groups.group0_.y) - (self_.group1_.y * other_groups.group0_.z), (self_.group1_.x * other_groups.group0_.z) - (self_.group1_.z * other_groups.group0_.x), (self_.group1_.y * other_groups.group0_.x) - (self_.group1_.x * other_groups.group0_.y), 0.0) + (vec4<f32>(vec3<f32>(other_groups.group0_.w), 0.0) * vec4<f32>(self_.group4_.x, self_.group4_.y, self_.group4_.z, 0.0)) - (vec4<f32>(vec3<f32>(self_.group4_.w), 0.0) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group2_.y * other_groups.group0_.z, self_.group2_.z * other_groups.group0_.x, self_.group2_.x * other_groups.group0_.y, -(self_.group3_.y * other_groups.group0_.y) - (self_.group3_.z * other_groups.group0_.z)) + (vec4<f32>(self_.group0_.y) * other_groups.group0_) - (vec4<f32>(self_.group2_.zxy.xyz, self_.group3_.x) * other_groups.group0_.yzxx)
    ));
}
fn multiVector_geometricAntiProduct_point(self_: MultiVector, other: Point) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = point_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(-(self_.group4_.x * other_groups.group0_.x) - (self_.group4_.y * other_groups.group0_.y) - (self_.group4_.z * other_groups.group0_.z) - (self_.group4_.w * other_groups.group0_.w), self_.group1_.w * other_groups.group0_.w * -1.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group0_.y * other_groups.group0_.x) + (self_.group2_.y * other_groups.group0_.z) + (self_.group3_.x * other_groups.group0_.w) - (self_.group2_.z * other_groups.group0_.y), (self_.group0_.y * other_groups.group0_.y) + (self_.group2_.z * other_groups.group0_.x) + (self_.group3_.y * other_groups.group0_.w) - (self_.group2_.x * other_groups.group0_.z), (self_.group0_.y * other_groups.group0_.z) + (self_.group2_.x * other_groups.group0_.y) + (self_.group3_.z * other_groups.group0_.w) - (self_.group2_.y * other_groups.group0_.x), self_.group0_.y * other_groups.group0_.w), 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(other_groups.group0_.w), 0.0) * vec4<f32>(self_.group4_.x, self_.group4_.y, self_.group4_.z, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>((self_.group4_.y * other_groups.group0_.z) - (self_.group4_.z * other_groups.group0_.y), (self_.group4_.z * other_groups.group0_.x) - (self_.group4_.x * other_groups.group0_.z), (self_.group4_.x * other_groups.group0_.y) - (self_.group4_.y * other_groups.group0_.x), 0.0) + (vec4<f32>(vec3<f32>(self_.group1_.w), 0.0) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0)) - (vec4<f32>(vec3<f32>(other_groups.group0_.w), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group2_.x * other_groups.group0_.w, self_.group2_.y * other_groups.group0_.w, self_.group2_.z * other_groups.group0_.w, (self_.group0_.x * other_groups.group0_.w) - (self_.group2_.x * other_groups.group0_.x) - (self_.group2_.y * other_groups.group0_.y) - (self_.group2_.z * other_groups.group0_.z))
    ));
}
fn multiVector_geometricAntiProduct_scalar(self_: MultiVector, other: Scalar) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = scalar_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.group0_.y * other.scalar, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group4_.x * other.scalar * -1.0, self_.group4_.y * other.scalar * -1.0, self_.group4_.z * other.scalar * -1.0, 0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(other.scalar), 0.0) * self_.group2_, 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.group1_.w * other.scalar * -1.0)
    ));
}
fn origin_geometricAntiProduct_antiScalar(self_: Origin, other: AntiScalar) -> Origin {
    let self_groups = origin_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(other.e1234 * self.e4, 0.0, 0.0, 0.0)
    ));
}
fn origin_geometricAntiProduct_dualNum(self_: Origin, other: DualNum) -> Flector {
    let self_groups = origin_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group0_.y * self.e4), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group0_.x * self.e4 * -1.0)
    ));
}
fn origin_geometricAntiProduct_flector(self_: Origin, other: Flector) -> Motor {
    let self_groups = origin_grouped(self_);
    let other_groups = flector_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self.e4) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, other_groups.group0_.w) * vec4<f32>(-1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(self.e4) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, other_groups.group1_.w)
    ));
}
fn origin_geometricAntiProduct_horizon(self_: Origin, other: Horizon) -> Scalar {
    let self_groups = origin_grouped(self_);
    let other_groups = horizon_grouped(other);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(other.e321 * self.e4, 0.0, 0.0, 0.0)
    ));
}
fn origin_geometricAntiProduct_line(self_: Origin, other: Line) -> Flector {
    let self_groups = origin_grouped(self_);
    let other_groups = line_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group1_.x * self.e4 * -1.0, other_groups.group1_.y * self.e4 * -1.0, other_groups.group1_.z * self.e4 * -1.0, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group0_.x * self.e4, other_groups.group0_.y * self.e4, other_groups.group0_.z * self.e4, 0.0)
    ));
}
fn origin_geometricAntiProduct_motor(self_: Origin, other: Motor) -> Flector {
    let self_groups = origin_grouped(self_);
    let other_groups = motor_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self.e4) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, other_groups.group0_.w) * vec4<f32>(-1.0, -1.0, -1.0, 1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self.e4) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, other_groups.group1_.w) * vec4<f32>(1.0, 1.0, 1.0, -1.0)
    ));
}
fn origin_geometricAntiProduct_multiVector(self_: Origin, other: MultiVector) -> MultiVector {
    let self_groups = origin_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(self.e4), 0.0, 0.0) * vec4<f32>(other_groups.group4_.w, other_groups.group1_.w, 0.0, 0.0) * vec2<f32>(1.0, -1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self.e4) * vec4<f32>(other_groups.group3_.xyz.xyz, other_groups.group0_.y) * vec4<f32>(-1.0, -1.0, -1.0, 1.0), 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(self.e4), 0.0) * vec4<f32>(other_groups.group4_.x, other_groups.group4_.y, other_groups.group4_.z, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(self.e4), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self.e4) * vec4<f32>(other_groups.group2_.xyz.xyz, other_groups.group0_.x) * vec4<f32>(1.0, 1.0, 1.0, -1.0)
    ));
}
fn origin_geometricAntiProduct_origin(self_: Origin, other: Origin) -> AntiScalar {
    let self_groups = origin_grouped(self_);
    let other_groups = origin_grouped(other);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(other.e4 * self.e4 * -1.0, 0.0, 0.0, 0.0)
    ));
}
fn origin_geometricAntiProduct_plane(self_: Origin, other: Plane) -> Motor {
    let self_groups = origin_grouped(self_);
    let other_groups = plane_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other_groups.group0_.x * self.e4 * -1.0, other_groups.group0_.y * self.e4 * -1.0, other_groups.group0_.z * self.e4 * -1.0, 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group0_.w * self.e4)
    ));
}
fn origin_geometricAntiProduct_point(self_: Origin, other: Point) -> Motor {
    let self_groups = origin_grouped(self_);
    let other_groups = point_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group0_.w * self.e4 * -1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(other_groups.group0_.x * self.e4, other_groups.group0_.y * self.e4, other_groups.group0_.z * self.e4, 0.0)
    ));
}
fn origin_geometricAntiProduct_scalar(self_: Origin, other: Scalar) -> Horizon {
    let self_groups = origin_grouped(self_);
    let other_groups = scalar_grouped(other);
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(self.e4 * other.scalar * -1.0, 0.0, 0.0, 0.0)
    ));
}
fn plane_geometricAntiProduct_antiScalar(self_: Plane, other: AntiScalar) -> Plane {
    let self_groups = plane_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(other.e1234) * self_.group0_
    ));
}
fn plane_geometricAntiProduct_dualNum(self_: Plane, other: DualNum) -> Flector {
    let self_groups = plane_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group0_.x * self_.group0_.x * -1.0, other_groups.group0_.x * self_.group0_.y * -1.0, other_groups.group0_.x * self_.group0_.z * -1.0, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group0_.y) * self_.group0_
    ));
}
fn plane_geometricAntiProduct_flector(self_: Plane, other: Flector) -> Motor {
    let self_groups = plane_grouped(self_);
    let other_groups = flector_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(-(other_groups.group0_.w * self_.group0_.x) - (other_groups.group1_.z * self_.group0_.y), -(other_groups.group0_.w * self_.group0_.y) - (other_groups.group1_.x * self_.group0_.z), -(other_groups.group0_.w * self_.group0_.z) - (other_groups.group1_.y * self_.group0_.x), (other_groups.group1_.y * self_.group0_.y) + (other_groups.group1_.z * self_.group0_.z)) + (other_groups.group1_.yzxx * self_.group0_.zxyx), 
        /* e23, e31, e12, scalar */ vec4<f32>((other_groups.group0_.z * self_.group0_.y) + (other_groups.group1_.w * self_.group0_.x), (other_groups.group0_.x * self_.group0_.z) + (other_groups.group1_.w * self_.group0_.y), (other_groups.group0_.y * self_.group0_.x) + (other_groups.group1_.w * self_.group0_.z), -(other_groups.group0_.z * self_.group0_.z) - (other_groups.group0_.w * self_.group0_.w)) - (vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, other_groups.group0_.y) * self_.group0_.wwwy) - (other_groups.group0_.yzxx * self_.group0_.zxyx)
    ));
}
fn plane_geometricAntiProduct_horizon(self_: Plane, other: Horizon) -> Line {
    let self_groups = plane_grouped(self_);
    let other_groups = horizon_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(other.e321), 0.0) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0)
    ));
}
fn plane_geometricAntiProduct_line(self_: Plane, other: Line) -> Flector {
    let self_groups = plane_grouped(self_);
    let other_groups = line_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group0_.x * self_.group0_.w) + (other_groups.group1_.y * self_.group0_.z), (other_groups.group0_.y * self_.group0_.w) + (other_groups.group1_.z * self_.group0_.x), (other_groups.group0_.z * self_.group0_.w) + (other_groups.group1_.x * self_.group0_.y), -(other_groups.group0_.y * self_.group0_.y) - (other_groups.group0_.z * self_.group0_.z)) - (vec4<f32>(other_groups.group1_.zxy.xyz, other_groups.group0_.x) * self_.group0_.yzxx), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.y * self_.group0_.z) * -1.0, (other_groups.group0_.z * self_.group0_.x) * -1.0, (other_groups.group0_.x * self_.group0_.y) * -1.0, (other_groups.group1_.y * self_.group0_.y) + (other_groups.group1_.z * self_.group0_.z)) + (vec4<f32>(other_groups.group0_.zxy.xyz, other_groups.group1_.x) * self_.group0_.yzxx)
    ));
}
fn plane_geometricAntiProduct_motor(self_: Plane, other: Motor) -> Flector {
    let self_groups = plane_grouped(self_);
    let other_groups = motor_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group0_.x * self_.group0_.w) + (other_groups.group1_.y * self_.group0_.z), (other_groups.group0_.y * self_.group0_.w) + (other_groups.group1_.z * self_.group0_.x), (other_groups.group0_.z * self_.group0_.w) + (other_groups.group1_.x * self_.group0_.y), (other_groups.group0_.z * self_.group0_.z) * -1.0) - (vec4<f32>(other_groups.group1_.z, other_groups.group1_.x, other_groups.group1_.y, other_groups.group0_.x) * self_.group0_.yzxx) - (vec4<f32>(other_groups.group1_.w, other_groups.group1_.w, other_groups.group1_.w, other_groups.group0_.y) * self_.group0_.xyzy), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.y * self_.group0_.z) * -1.0, (other_groups.group0_.z * self_.group0_.x) * -1.0, (other_groups.group0_.x * self_.group0_.y) * -1.0, (other_groups.group1_.y * self_.group0_.y) + (other_groups.group1_.z * self_.group0_.z)) + (vec4<f32>(other_groups.group0_.w, other_groups.group0_.w, other_groups.group0_.w, other_groups.group1_.x) * self_.group0_.xyzx) + (other_groups.group0_.zxyw * self_.group0_.yzxw)
    ));
}
fn plane_geometricAntiProduct_multiVector(self_: Plane, other: MultiVector) -> MultiVector {
    let self_groups = plane_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(-(other_groups.group1_.x * self_.group0_.x) - (other_groups.group1_.y * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.z) - (other_groups.group1_.w * self_.group0_.w), (other_groups.group4_.x * self_.group0_.x) + (other_groups.group4_.y * self_.group0_.y) + (other_groups.group4_.z * self_.group0_.z), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group2_.x * self_.group0_.w) + (other_groups.group3_.y * self_.group0_.z), (other_groups.group2_.y * self_.group0_.w) + (other_groups.group3_.z * self_.group0_.x), (other_groups.group2_.z * self_.group0_.w) + (other_groups.group3_.x * self_.group0_.y), (other_groups.group2_.z * self_.group0_.z) * -1.0) - (vec4<f32>(other_groups.group0_.xx.xy, other_groups.group0_.x, other_groups.group2_.x) * self_.group0_.xyzx) - (vec4<f32>(other_groups.group3_.zxy.xyz, other_groups.group2_.y) * self_.group0_.yzxy), 
        /* e41, e42, e43 */ vec4<f32>((other_groups.group4_.y * self_.group0_.z) - (other_groups.group4_.z * self_.group0_.y), (other_groups.group4_.z * self_.group0_.x) - (other_groups.group4_.x * self_.group0_.z), (other_groups.group4_.x * self_.group0_.y) - (other_groups.group4_.y * self_.group0_.x), 0.0) - (vec4<f32>(vec3<f32>(other_groups.group1_.w), 0.0) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((other_groups.group1_.z * self_.group0_.y) - (other_groups.group1_.y * self_.group0_.z), (other_groups.group1_.x * self_.group0_.z) - (other_groups.group1_.z * self_.group0_.x), (other_groups.group1_.y * self_.group0_.x) - (other_groups.group1_.x * self_.group0_.y), 0.0) + (vec4<f32>(vec3<f32>(other_groups.group4_.w), 0.0) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0)) - (vec4<f32>(vec3<f32>(self_.group0_.w), 0.0) * vec4<f32>(other_groups.group4_.x, other_groups.group4_.y, other_groups.group4_.z, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group2_.y * self_.group0_.z) * -1.0, (other_groups.group2_.z * self_.group0_.x) * -1.0, (other_groups.group2_.x * self_.group0_.y) * -1.0, (other_groups.group3_.y * self_.group0_.y) + (other_groups.group3_.z * self_.group0_.z)) + (vec4<f32>(other_groups.group0_.y) * self_.group0_) + (vec4<f32>(other_groups.group2_.zxy.xyz, other_groups.group3_.x) * self_.group0_.yzxx)
    ));
}
fn plane_geometricAntiProduct_origin(self_: Plane, other: Origin) -> Motor {
    let self_groups = plane_grouped(self_);
    let other_groups = origin_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.group0_.x * other.e4 * -1.0, self_.group0_.y * other.e4 * -1.0, self_.group0_.z * other.e4 * -1.0, 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, self_.group0_.w * other.e4 * -1.0)
    ));
}
fn plane_geometricAntiProduct_plane(self_: Plane, other: Plane) -> Motor {
    let self_groups = plane_grouped(self_);
    let other_groups = plane_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other_groups.group0_.z * self_.group0_.y) * -1.0, (other_groups.group0_.x * self_.group0_.z) * -1.0, (other_groups.group0_.y * self_.group0_.x) * -1.0, (other_groups.group0_.y * self_.group0_.y) + (other_groups.group0_.z * self_.group0_.z)) + (other_groups.group0_.yzxx * self_.group0_.zxyx), 
        /* e23, e31, e12, scalar */ vec4<f32>((other_groups.group0_.w * self_.group0_.x) - (other_groups.group0_.x * self_.group0_.w), (other_groups.group0_.w * self_.group0_.y) - (other_groups.group0_.y * self_.group0_.w), (other_groups.group0_.w * self_.group0_.z) - (other_groups.group0_.z * self_.group0_.w), 0.0)
    ));
}
fn plane_geometricAntiProduct_point(self_: Plane, other: Point) -> Motor {
    let self_groups = plane_grouped(self_);
    let other_groups = point_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.group0_.x * other_groups.group0_.w * -1.0, self_.group0_.y * other_groups.group0_.w * -1.0, self_.group0_.z * other_groups.group0_.w * -1.0, 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.group0_.y * other_groups.group0_.z, self_.group0_.z * other_groups.group0_.x, self_.group0_.x * other_groups.group0_.y, -(self_.group0_.y * other_groups.group0_.y) - (self_.group0_.z * other_groups.group0_.z) - (self_.group0_.w * other_groups.group0_.w)) - (self_.group0_.zxyx * other_groups.group0_.yzxx)
    ));
}
fn plane_geometricAntiProduct_scalar(self_: Plane, other: Scalar) -> Point {
    let self_groups = plane_grouped(self_);
    let other_groups = scalar_grouped(other);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group0_.x * other.scalar * -1.0, self_.group0_.y * other.scalar * -1.0, self_.group0_.z * other.scalar * -1.0, 0.0)
    ));
}
fn point_geometricAntiProduct_antiScalar(self_: Point, other: AntiScalar) -> Point {
    let self_groups = point_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e1234) * self_.group0_
    ));
}
fn point_geometricAntiProduct_dualNum(self_: Point, other: DualNum) -> Flector {
    let self_groups = point_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group0_.y) * self_.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group0_.x * self_.group0_.w * -1.0)
    ));
}
fn point_geometricAntiProduct_flector(self_: Point, other: Flector) -> Motor {
    let self_groups = point_grouped(self_);
    let other_groups = flector_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.group0_.w) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, other_groups.group0_.w) * vec4<f32>(-1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(-(other_groups.group0_.w * self_.group0_.x) - (other_groups.group1_.z * self_.group0_.y), -(other_groups.group0_.w * self_.group0_.y) - (other_groups.group1_.x * self_.group0_.z), -(other_groups.group0_.w * self_.group0_.z) - (other_groups.group1_.y * self_.group0_.x), (other_groups.group1_.z * self_.group0_.z) + (other_groups.group1_.w * self_.group0_.w)) + (vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, other_groups.group1_.x) * self_.group0_.wwwx) + (other_groups.group1_.yzxy * self_.group0_.zxyy)
    ));
}
fn point_geometricAntiProduct_horizon(self_: Point, other: Horizon) -> Scalar {
    let self_groups = point_grouped(self_);
    let other_groups = horizon_grouped(other);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(self_.group0_.w * other.e321, 0.0, 0.0, 0.0)
    ));
}
fn point_geometricAntiProduct_line(self_: Point, other: Line) -> Flector {
    let self_groups = point_grouped(self_);
    let other_groups = line_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group0_.z * self_.group0_.y) - (other_groups.group0_.y * self_.group0_.z) - (other_groups.group1_.x * self_.group0_.w), (other_groups.group0_.x * self_.group0_.z) - (other_groups.group0_.z * self_.group0_.x) - (other_groups.group1_.y * self_.group0_.w), (other_groups.group0_.y * self_.group0_.x) - (other_groups.group0_.x * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.w), 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group0_.x * self_.group0_.w, other_groups.group0_.y * self_.group0_.w, other_groups.group0_.z * self_.group0_.w, -(other_groups.group0_.x * self_.group0_.x) - (other_groups.group0_.y * self_.group0_.y) - (other_groups.group0_.z * self_.group0_.z))
    ));
}
fn point_geometricAntiProduct_motor(self_: Point, other: Motor) -> Flector {
    let self_groups = point_grouped(self_);
    let other_groups = motor_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group0_.z * self_.group0_.y) + (other_groups.group0_.w * self_.group0_.x) - (other_groups.group0_.y * self_.group0_.z) - (other_groups.group1_.x * self_.group0_.w), (other_groups.group0_.x * self_.group0_.z) + (other_groups.group0_.w * self_.group0_.y) - (other_groups.group0_.z * self_.group0_.x) - (other_groups.group1_.y * self_.group0_.w), (other_groups.group0_.y * self_.group0_.x) + (other_groups.group0_.w * self_.group0_.z) - (other_groups.group0_.x * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.w), other_groups.group0_.w * self_.group0_.w), 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group0_.x * self_.group0_.w, other_groups.group0_.y * self_.group0_.w, other_groups.group0_.z * self_.group0_.w, -(other_groups.group0_.x * self_.group0_.x) - (other_groups.group0_.y * self_.group0_.y) - (other_groups.group0_.z * self_.group0_.z) - (other_groups.group1_.w * self_.group0_.w))
    ));
}
fn point_geometricAntiProduct_multiVector(self_: Point, other: MultiVector) -> MultiVector {
    let self_groups = point_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((other_groups.group4_.x * self_.group0_.x) + (other_groups.group4_.y * self_.group0_.y) + (other_groups.group4_.z * self_.group0_.z) + (other_groups.group4_.w * self_.group0_.w), other_groups.group1_.w * self_.group0_.w * -1.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group0_.y * self_.group0_.x) + (other_groups.group2_.z * self_.group0_.y) - (other_groups.group2_.y * self_.group0_.z) - (other_groups.group3_.x * self_.group0_.w), (other_groups.group0_.y * self_.group0_.y) + (other_groups.group2_.x * self_.group0_.z) - (other_groups.group2_.z * self_.group0_.x) - (other_groups.group3_.y * self_.group0_.w), (other_groups.group0_.y * self_.group0_.z) + (other_groups.group2_.y * self_.group0_.x) - (other_groups.group2_.x * self_.group0_.y) - (other_groups.group3_.z * self_.group0_.w), other_groups.group0_.y * self_.group0_.w), 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(self_.group0_.w), 0.0) * vec4<f32>(other_groups.group4_.x, other_groups.group4_.y, other_groups.group4_.z, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>((other_groups.group4_.y * self_.group0_.z) - (other_groups.group4_.z * self_.group0_.y), (other_groups.group4_.z * self_.group0_.x) - (other_groups.group4_.x * self_.group0_.z), (other_groups.group4_.x * self_.group0_.y) - (other_groups.group4_.y * self_.group0_.x), 0.0) + (vec4<f32>(vec3<f32>(self_.group0_.w), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0)) - (vec4<f32>(vec3<f32>(other_groups.group1_.w), 0.0) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group2_.x * self_.group0_.w, other_groups.group2_.y * self_.group0_.w, other_groups.group2_.z * self_.group0_.w, -(other_groups.group0_.x * self_.group0_.w) - (other_groups.group2_.x * self_.group0_.x) - (other_groups.group2_.y * self_.group0_.y) - (other_groups.group2_.z * self_.group0_.z))
    ));
}
fn point_geometricAntiProduct_origin(self_: Point, other: Origin) -> Motor {
    let self_groups = point_grouped(self_);
    let other_groups = origin_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, self_.group0_.w * other.e4 * -1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.group0_.x * other.e4 * -1.0, self_.group0_.y * other.e4 * -1.0, self_.group0_.z * other.e4 * -1.0, 0.0)
    ));
}
fn point_geometricAntiProduct_plane(self_: Point, other: Plane) -> Motor {
    let self_groups = point_grouped(self_);
    let other_groups = plane_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other_groups.group0_.x * self_.group0_.w * -1.0, other_groups.group0_.y * self_.group0_.w * -1.0, other_groups.group0_.z * self_.group0_.w * -1.0, 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>((other_groups.group0_.z * self_.group0_.y) * -1.0, (other_groups.group0_.x * self_.group0_.z) * -1.0, (other_groups.group0_.y * self_.group0_.x) * -1.0, (other_groups.group0_.y * self_.group0_.y) + (other_groups.group0_.z * self_.group0_.z) + (other_groups.group0_.w * self_.group0_.w)) + (other_groups.group0_.yzxx * self_.group0_.zxyx)
    ));
}
fn point_geometricAntiProduct_point(self_: Point, other: Point) -> Motor {
    let self_groups = point_grouped(self_);
    let other_groups = point_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group0_.w * self_.group0_.w * -1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>((other_groups.group0_.x * self_.group0_.w) - (other_groups.group0_.w * self_.group0_.x), (other_groups.group0_.y * self_.group0_.w) - (other_groups.group0_.w * self_.group0_.y), (other_groups.group0_.z * self_.group0_.w) - (other_groups.group0_.w * self_.group0_.z), 0.0)
    ));
}
fn point_geometricAntiProduct_scalar(self_: Point, other: Scalar) -> Horizon {
    let self_groups = point_grouped(self_);
    let other_groups = scalar_grouped(other);
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(self_.group0_.w * other.scalar * -1.0, 0.0, 0.0, 0.0)
    ));
}
fn scalar_geometricAntiProduct_antiScalar(self_: Scalar, other: AntiScalar) -> Scalar {
    let self_groups = scalar_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(other.e1234 * self.scalar, 0.0, 0.0, 0.0)
    ));
}
fn scalar_geometricAntiProduct_dualNum(self_: Scalar, other: DualNum) -> Scalar {
    let self_groups = scalar_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(other_groups.group0_.y * self.scalar, 0.0, 0.0, 0.0)
    ));
}
fn scalar_geometricAntiProduct_flector(self_: Scalar, other: Flector) -> Flector {
    let self_groups = scalar_grouped(self_);
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group1_.x * self.scalar, other_groups.group1_.y * self.scalar, other_groups.group1_.z * self.scalar, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group0_.w * self.scalar)
    ));
}
fn scalar_geometricAntiProduct_line(self_: Scalar, other: Line) -> Line {
    let self_groups = scalar_grouped(self_);
    let other_groups = line_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(self.scalar), 0.0) * other_groups.group0_
    ));
}
fn scalar_geometricAntiProduct_motor(self_: Scalar, other: Motor) -> Motor {
    let self_groups = scalar_grouped(self_);
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(self.scalar) * other_groups.group0_
    ));
}
fn scalar_geometricAntiProduct_multiVector(self_: Scalar, other: MultiVector) -> MultiVector {
    let self_groups = scalar_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other_groups.group0_.y * self.scalar, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group4_.x * self.scalar, other_groups.group4_.y * self.scalar, other_groups.group4_.z * self.scalar, 0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(self.scalar), 0.0) * other_groups.group2_, 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group1_.w * self.scalar)
    ));
}
fn scalar_geometricAntiProduct_origin(self_: Scalar, other: Origin) -> Horizon {
    let self_groups = scalar_grouped(self_);
    let other_groups = origin_grouped(other);
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(other.e4 * self.scalar, 0.0, 0.0, 0.0)
    ));
}
fn scalar_geometricAntiProduct_plane(self_: Scalar, other: Plane) -> Point {
    let self_groups = scalar_grouped(self_);
    let other_groups = plane_grouped(other);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group0_.x * self.scalar, other_groups.group0_.y * self.scalar, other_groups.group0_.z * self.scalar, 0.0)
    ));
}
fn scalar_geometricAntiProduct_point(self_: Scalar, other: Point) -> Horizon {
    let self_groups = scalar_grouped(self_);
    let other_groups = point_grouped(other);
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(other_groups.group0_.w * self.scalar, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_geometricAntiQuotient_antiScalar(self_: AntiScalar, other: AntiScalar) -> AntiScalar {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other.e1234, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(anti_inverse.e1234 * self.e1234, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_geometricAntiQuotient_dualNum(self_: AntiScalar, other: DualNum) -> AntiScalar {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.y, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(anti_inverse.e1234 * self.e1234, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_geometricAntiQuotient_flector(self_: AntiScalar, other: Flector) -> AntiScalar {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = flector_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group1_.x, 2) + pow(other_groups.group1_.y, 2) + pow(other_groups.group1_.z, 2) - pow(other_groups.group0_.w, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(anti_inverse.e1234 * self.e1234, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_geometricAntiQuotient_line(self_: AntiScalar, other: Line) -> AntiScalar {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = line_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(-pow(other_groups.group0_.x, 2) - pow(other_groups.group0_.y, 2) - pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(anti_inverse.e1234 * self.e1234, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_geometricAntiQuotient_motor(self_: AntiScalar, other: Motor) -> AntiScalar {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = motor_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.w, 2) - pow(other_groups.group0_.x, 2) - pow(other_groups.group0_.y, 2) - pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(anti_inverse.e1234 * self.e1234, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_geometricAntiQuotient_multiVector(self_: AntiScalar, other: MultiVector) -> AntiScalar {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.y, 2) + pow(other_groups.group4_.x, 2) + pow(other_groups.group4_.y, 2) + pow(other_groups.group4_.z, 2) - pow(other_groups.group2_.x, 2) - pow(other_groups.group2_.y, 2) - pow(other_groups.group2_.z, 2) - pow(other_groups.group1_.w, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(anti_inverse.e1234 * self.e1234, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_geometricAntiQuotient_origin(self_: AntiScalar, other: Origin) -> AntiScalar {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = origin_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other.e4, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(anti_inverse.e1234 * self.e1234, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_geometricAntiQuotient_plane(self_: AntiScalar, other: Plane) -> AntiScalar {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = plane_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group0_.y, 2) + pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(anti_inverse.e1234 * self.e1234, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_geometricAntiQuotient_point(self_: AntiScalar, other: Point) -> AntiScalar {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = point_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.w, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(anti_inverse.e1234 * self.e1234, 0.0, 0.0, 0.0)
    ));
}
fn dualNum_geometricAntiQuotient_antiScalar(self_: DualNum, other: AntiScalar) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other.e1234, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(anti_inverse.e1234), 0.0, 0.0) * self_.group0_
    ));
}
fn dualNum_geometricAntiQuotient_dualNum(self_: DualNum, other: DualNum) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.y, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(anti_inverse.e1234), 0.0, 0.0) * self_.group0_
    ));
}
fn dualNum_geometricAntiQuotient_flector(self_: DualNum, other: Flector) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let other_groups = flector_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group1_.x, 2) + pow(other_groups.group1_.y, 2) + pow(other_groups.group1_.z, 2) - pow(other_groups.group0_.w, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(anti_inverse.e1234), 0.0, 0.0) * self_.group0_
    ));
}
fn dualNum_geometricAntiQuotient_line(self_: DualNum, other: Line) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let other_groups = line_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(-pow(other_groups.group0_.x, 2) - pow(other_groups.group0_.y, 2) - pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(anti_inverse.e1234), 0.0, 0.0) * self_.group0_
    ));
}
fn dualNum_geometricAntiQuotient_motor(self_: DualNum, other: Motor) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let other_groups = motor_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.w, 2) - pow(other_groups.group0_.x, 2) - pow(other_groups.group0_.y, 2) - pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(anti_inverse.e1234), 0.0, 0.0) * self_.group0_
    ));
}
fn dualNum_geometricAntiQuotient_multiVector(self_: DualNum, other: MultiVector) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.y, 2) + pow(other_groups.group4_.x, 2) + pow(other_groups.group4_.y, 2) + pow(other_groups.group4_.z, 2) - pow(other_groups.group2_.x, 2) - pow(other_groups.group2_.y, 2) - pow(other_groups.group2_.z, 2) - pow(other_groups.group1_.w, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(anti_inverse.e1234), 0.0, 0.0) * self_.group0_
    ));
}
fn dualNum_geometricAntiQuotient_origin(self_: DualNum, other: Origin) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let other_groups = origin_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other.e4, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(anti_inverse.e1234), 0.0, 0.0) * self_.group0_
    ));
}
fn dualNum_geometricAntiQuotient_plane(self_: DualNum, other: Plane) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let other_groups = plane_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group0_.y, 2) + pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(anti_inverse.e1234), 0.0, 0.0) * self_.group0_
    ));
}
fn dualNum_geometricAntiQuotient_point(self_: DualNum, other: Point) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let other_groups = point_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.w, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(anti_inverse.e1234), 0.0, 0.0) * self_.group0_
    ));
}
fn flector_geometricAntiQuotient_antiScalar(self_: Flector, other: AntiScalar) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other.e1234, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234) * self_.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234) * self_.group1_
    ));
}
fn flector_geometricAntiQuotient_dualNum(self_: Flector, other: DualNum) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.y, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234) * self_.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234) * self_.group1_
    ));
}
fn flector_geometricAntiQuotient_flector(self_: Flector, other: Flector) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = flector_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group1_.x, 2) + pow(other_groups.group1_.y, 2) + pow(other_groups.group1_.z, 2) - pow(other_groups.group0_.w, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234) * self_.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234) * self_.group1_
    ));
}
fn flector_geometricAntiQuotient_line(self_: Flector, other: Line) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = line_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(-pow(other_groups.group0_.x, 2) - pow(other_groups.group0_.y, 2) - pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234) * self_.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234) * self_.group1_
    ));
}
fn flector_geometricAntiQuotient_motor(self_: Flector, other: Motor) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = motor_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.w, 2) - pow(other_groups.group0_.x, 2) - pow(other_groups.group0_.y, 2) - pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234) * self_.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234) * self_.group1_
    ));
}
fn flector_geometricAntiQuotient_multiVector(self_: Flector, other: MultiVector) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.y, 2) + pow(other_groups.group4_.x, 2) + pow(other_groups.group4_.y, 2) + pow(other_groups.group4_.z, 2) - pow(other_groups.group2_.x, 2) - pow(other_groups.group2_.y, 2) - pow(other_groups.group2_.z, 2) - pow(other_groups.group1_.w, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234) * self_.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234) * self_.group1_
    ));
}
fn flector_geometricAntiQuotient_origin(self_: Flector, other: Origin) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = origin_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other.e4, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234) * self_.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234) * self_.group1_
    ));
}
fn flector_geometricAntiQuotient_plane(self_: Flector, other: Plane) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = plane_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group0_.y, 2) + pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234) * self_.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234) * self_.group1_
    ));
}
fn flector_geometricAntiQuotient_point(self_: Flector, other: Point) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = point_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.w, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234) * self_.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234) * self_.group1_
    ));
}
fn horizon_geometricAntiQuotient_antiScalar(self_: Horizon, other: AntiScalar) -> Horizon {
    let self_groups = horizon_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other.e1234, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(anti_inverse.e1234 * self.e321, 0.0, 0.0, 0.0)
    ));
}
fn horizon_geometricAntiQuotient_dualNum(self_: Horizon, other: DualNum) -> Horizon {
    let self_groups = horizon_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.y, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(anti_inverse.e1234 * self.e321, 0.0, 0.0, 0.0)
    ));
}
fn horizon_geometricAntiQuotient_flector(self_: Horizon, other: Flector) -> Horizon {
    let self_groups = horizon_grouped(self_);
    let other_groups = flector_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group1_.x, 2) + pow(other_groups.group1_.y, 2) + pow(other_groups.group1_.z, 2) - pow(other_groups.group0_.w, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(anti_inverse.e1234 * self.e321, 0.0, 0.0, 0.0)
    ));
}
fn horizon_geometricAntiQuotient_line(self_: Horizon, other: Line) -> Horizon {
    let self_groups = horizon_grouped(self_);
    let other_groups = line_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(-pow(other_groups.group0_.x, 2) - pow(other_groups.group0_.y, 2) - pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(anti_inverse.e1234 * self.e321, 0.0, 0.0, 0.0)
    ));
}
fn horizon_geometricAntiQuotient_motor(self_: Horizon, other: Motor) -> Horizon {
    let self_groups = horizon_grouped(self_);
    let other_groups = motor_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.w, 2) - pow(other_groups.group0_.x, 2) - pow(other_groups.group0_.y, 2) - pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(anti_inverse.e1234 * self.e321, 0.0, 0.0, 0.0)
    ));
}
fn horizon_geometricAntiQuotient_multiVector(self_: Horizon, other: MultiVector) -> Horizon {
    let self_groups = horizon_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.y, 2) + pow(other_groups.group4_.x, 2) + pow(other_groups.group4_.y, 2) + pow(other_groups.group4_.z, 2) - pow(other_groups.group2_.x, 2) - pow(other_groups.group2_.y, 2) - pow(other_groups.group2_.z, 2) - pow(other_groups.group1_.w, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(anti_inverse.e1234 * self.e321, 0.0, 0.0, 0.0)
    ));
}
fn horizon_geometricAntiQuotient_origin(self_: Horizon, other: Origin) -> Horizon {
    let self_groups = horizon_grouped(self_);
    let other_groups = origin_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other.e4, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(anti_inverse.e1234 * self.e321, 0.0, 0.0, 0.0)
    ));
}
fn horizon_geometricAntiQuotient_plane(self_: Horizon, other: Plane) -> Horizon {
    let self_groups = horizon_grouped(self_);
    let other_groups = plane_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group0_.y, 2) + pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(anti_inverse.e1234 * self.e321, 0.0, 0.0, 0.0)
    ));
}
fn horizon_geometricAntiQuotient_point(self_: Horizon, other: Point) -> Horizon {
    let self_groups = horizon_grouped(self_);
    let other_groups = point_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.w, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(anti_inverse.e1234 * self.e321, 0.0, 0.0, 0.0)
    ));
}
fn line_geometricAntiQuotient_antiScalar(self_: Line, other: AntiScalar) -> Line {
    let self_groups = line_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other.e1234, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(anti_inverse.e1234), 0.0) * self_.group0_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(anti_inverse.e1234), 0.0) * self_.group1_
    ));
}
fn line_geometricAntiQuotient_dualNum(self_: Line, other: DualNum) -> Line {
    let self_groups = line_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.y, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(anti_inverse.e1234), 0.0) * self_.group0_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(anti_inverse.e1234), 0.0) * self_.group1_
    ));
}
fn line_geometricAntiQuotient_flector(self_: Line, other: Flector) -> Line {
    let self_groups = line_grouped(self_);
    let other_groups = flector_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group1_.x, 2) + pow(other_groups.group1_.y, 2) + pow(other_groups.group1_.z, 2) - pow(other_groups.group0_.w, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(anti_inverse.e1234), 0.0) * self_.group0_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(anti_inverse.e1234), 0.0) * self_.group1_
    ));
}
fn line_geometricAntiQuotient_line(self_: Line, other: Line) -> Line {
    let self_groups = line_grouped(self_);
    let other_groups = line_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(-pow(other_groups.group0_.x, 2) - pow(other_groups.group0_.y, 2) - pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(anti_inverse.e1234), 0.0) * self_.group0_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(anti_inverse.e1234), 0.0) * self_.group1_
    ));
}
fn line_geometricAntiQuotient_motor(self_: Line, other: Motor) -> Line {
    let self_groups = line_grouped(self_);
    let other_groups = motor_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.w, 2) - pow(other_groups.group0_.x, 2) - pow(other_groups.group0_.y, 2) - pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(anti_inverse.e1234), 0.0) * self_.group0_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(anti_inverse.e1234), 0.0) * self_.group1_
    ));
}
fn line_geometricAntiQuotient_multiVector(self_: Line, other: MultiVector) -> Line {
    let self_groups = line_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.y, 2) + pow(other_groups.group4_.x, 2) + pow(other_groups.group4_.y, 2) + pow(other_groups.group4_.z, 2) - pow(other_groups.group2_.x, 2) - pow(other_groups.group2_.y, 2) - pow(other_groups.group2_.z, 2) - pow(other_groups.group1_.w, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(anti_inverse.e1234), 0.0) * self_.group0_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(anti_inverse.e1234), 0.0) * self_.group1_
    ));
}
fn line_geometricAntiQuotient_origin(self_: Line, other: Origin) -> Line {
    let self_groups = line_grouped(self_);
    let other_groups = origin_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other.e4, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(anti_inverse.e1234), 0.0) * self_.group0_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(anti_inverse.e1234), 0.0) * self_.group1_
    ));
}
fn line_geometricAntiQuotient_plane(self_: Line, other: Plane) -> Line {
    let self_groups = line_grouped(self_);
    let other_groups = plane_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group0_.y, 2) + pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(anti_inverse.e1234), 0.0) * self_.group0_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(anti_inverse.e1234), 0.0) * self_.group1_
    ));
}
fn line_geometricAntiQuotient_point(self_: Line, other: Point) -> Line {
    let self_groups = line_grouped(self_);
    let other_groups = point_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.w, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(anti_inverse.e1234), 0.0) * self_.group0_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(anti_inverse.e1234), 0.0) * self_.group1_
    ));
}
fn motor_geometricAntiQuotient_antiScalar(self_: Motor, other: AntiScalar) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other.e1234, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(anti_inverse.e1234) * self_.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(anti_inverse.e1234) * self_.group1_
    ));
}
fn motor_geometricAntiQuotient_dualNum(self_: Motor, other: DualNum) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.y, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(anti_inverse.e1234) * self_.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(anti_inverse.e1234) * self_.group1_
    ));
}
fn motor_geometricAntiQuotient_flector(self_: Motor, other: Flector) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = flector_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group1_.x, 2) + pow(other_groups.group1_.y, 2) + pow(other_groups.group1_.z, 2) - pow(other_groups.group0_.w, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(anti_inverse.e1234) * self_.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(anti_inverse.e1234) * self_.group1_
    ));
}
fn motor_geometricAntiQuotient_line(self_: Motor, other: Line) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = line_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(-pow(other_groups.group0_.x, 2) - pow(other_groups.group0_.y, 2) - pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(anti_inverse.e1234) * self_.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(anti_inverse.e1234) * self_.group1_
    ));
}
fn motor_geometricAntiQuotient_motor(self_: Motor, other: Motor) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = motor_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.w, 2) - pow(other_groups.group0_.x, 2) - pow(other_groups.group0_.y, 2) - pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(anti_inverse.e1234) * self_.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(anti_inverse.e1234) * self_.group1_
    ));
}
fn motor_geometricAntiQuotient_multiVector(self_: Motor, other: MultiVector) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.y, 2) + pow(other_groups.group4_.x, 2) + pow(other_groups.group4_.y, 2) + pow(other_groups.group4_.z, 2) - pow(other_groups.group2_.x, 2) - pow(other_groups.group2_.y, 2) - pow(other_groups.group2_.z, 2) - pow(other_groups.group1_.w, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(anti_inverse.e1234) * self_.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(anti_inverse.e1234) * self_.group1_
    ));
}
fn motor_geometricAntiQuotient_origin(self_: Motor, other: Origin) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = origin_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other.e4, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(anti_inverse.e1234) * self_.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(anti_inverse.e1234) * self_.group1_
    ));
}
fn motor_geometricAntiQuotient_plane(self_: Motor, other: Plane) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = plane_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group0_.y, 2) + pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(anti_inverse.e1234) * self_.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(anti_inverse.e1234) * self_.group1_
    ));
}
fn motor_geometricAntiQuotient_point(self_: Motor, other: Point) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = point_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.w, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(anti_inverse.e1234) * self_.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(anti_inverse.e1234) * self_.group1_
    ));
}
fn multiVector_geometricAntiQuotient_antiScalar(self_: MultiVector, other: AntiScalar) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other.e1234, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(anti_inverse.e1234), 0.0, 0.0) * self_.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234) * self_.group1_, 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(anti_inverse.e1234), 0.0) * self_.group2_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(anti_inverse.e1234), 0.0) * self_.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234) * self_.group4_
    ));
}
fn multiVector_geometricAntiQuotient_dualNum(self_: MultiVector, other: DualNum) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.y, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(anti_inverse.e1234), 0.0, 0.0) * self_.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234) * self_.group1_, 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(anti_inverse.e1234), 0.0) * self_.group2_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(anti_inverse.e1234), 0.0) * self_.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234) * self_.group4_
    ));
}
fn multiVector_geometricAntiQuotient_flector(self_: MultiVector, other: Flector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = flector_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group1_.x, 2) + pow(other_groups.group1_.y, 2) + pow(other_groups.group1_.z, 2) - pow(other_groups.group0_.w, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(anti_inverse.e1234), 0.0, 0.0) * self_.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234) * self_.group1_, 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(anti_inverse.e1234), 0.0) * self_.group2_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(anti_inverse.e1234), 0.0) * self_.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234) * self_.group4_
    ));
}
fn multiVector_geometricAntiQuotient_line(self_: MultiVector, other: Line) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = line_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(-pow(other_groups.group0_.x, 2) - pow(other_groups.group0_.y, 2) - pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(anti_inverse.e1234), 0.0, 0.0) * self_.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234) * self_.group1_, 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(anti_inverse.e1234), 0.0) * self_.group2_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(anti_inverse.e1234), 0.0) * self_.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234) * self_.group4_
    ));
}
fn multiVector_geometricAntiQuotient_motor(self_: MultiVector, other: Motor) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = motor_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.w, 2) - pow(other_groups.group0_.x, 2) - pow(other_groups.group0_.y, 2) - pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(anti_inverse.e1234), 0.0, 0.0) * self_.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234) * self_.group1_, 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(anti_inverse.e1234), 0.0) * self_.group2_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(anti_inverse.e1234), 0.0) * self_.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234) * self_.group4_
    ));
}
fn multiVector_geometricAntiQuotient_multiVector(self_: MultiVector, other: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.y, 2) + pow(other_groups.group4_.x, 2) + pow(other_groups.group4_.y, 2) + pow(other_groups.group4_.z, 2) - pow(other_groups.group2_.x, 2) - pow(other_groups.group2_.y, 2) - pow(other_groups.group2_.z, 2) - pow(other_groups.group1_.w, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(anti_inverse.e1234), 0.0, 0.0) * self_.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234) * self_.group1_, 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(anti_inverse.e1234), 0.0) * self_.group2_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(anti_inverse.e1234), 0.0) * self_.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234) * self_.group4_
    ));
}
fn multiVector_geometricAntiQuotient_origin(self_: MultiVector, other: Origin) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = origin_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other.e4, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(anti_inverse.e1234), 0.0, 0.0) * self_.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234) * self_.group1_, 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(anti_inverse.e1234), 0.0) * self_.group2_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(anti_inverse.e1234), 0.0) * self_.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234) * self_.group4_
    ));
}
fn multiVector_geometricAntiQuotient_plane(self_: MultiVector, other: Plane) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = plane_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group0_.y, 2) + pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(anti_inverse.e1234), 0.0, 0.0) * self_.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234) * self_.group1_, 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(anti_inverse.e1234), 0.0) * self_.group2_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(anti_inverse.e1234), 0.0) * self_.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234) * self_.group4_
    ));
}
fn multiVector_geometricAntiQuotient_point(self_: MultiVector, other: Point) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = point_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.w, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(anti_inverse.e1234), 0.0, 0.0) * self_.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234) * self_.group1_, 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(anti_inverse.e1234), 0.0) * self_.group2_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(anti_inverse.e1234), 0.0) * self_.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234) * self_.group4_
    ));
}
fn origin_geometricAntiQuotient_antiScalar(self_: Origin, other: AntiScalar) -> Origin {
    let self_groups = origin_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other.e1234, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(anti_inverse.e1234 * self.e4, 0.0, 0.0, 0.0)
    ));
}
fn origin_geometricAntiQuotient_dualNum(self_: Origin, other: DualNum) -> Origin {
    let self_groups = origin_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.y, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(anti_inverse.e1234 * self.e4, 0.0, 0.0, 0.0)
    ));
}
fn origin_geometricAntiQuotient_flector(self_: Origin, other: Flector) -> Origin {
    let self_groups = origin_grouped(self_);
    let other_groups = flector_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group1_.x, 2) + pow(other_groups.group1_.y, 2) + pow(other_groups.group1_.z, 2) - pow(other_groups.group0_.w, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(anti_inverse.e1234 * self.e4, 0.0, 0.0, 0.0)
    ));
}
fn origin_geometricAntiQuotient_line(self_: Origin, other: Line) -> Origin {
    let self_groups = origin_grouped(self_);
    let other_groups = line_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(-pow(other_groups.group0_.x, 2) - pow(other_groups.group0_.y, 2) - pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(anti_inverse.e1234 * self.e4, 0.0, 0.0, 0.0)
    ));
}
fn origin_geometricAntiQuotient_motor(self_: Origin, other: Motor) -> Origin {
    let self_groups = origin_grouped(self_);
    let other_groups = motor_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.w, 2) - pow(other_groups.group0_.x, 2) - pow(other_groups.group0_.y, 2) - pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(anti_inverse.e1234 * self.e4, 0.0, 0.0, 0.0)
    ));
}
fn origin_geometricAntiQuotient_multiVector(self_: Origin, other: MultiVector) -> Origin {
    let self_groups = origin_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.y, 2) + pow(other_groups.group4_.x, 2) + pow(other_groups.group4_.y, 2) + pow(other_groups.group4_.z, 2) - pow(other_groups.group2_.x, 2) - pow(other_groups.group2_.y, 2) - pow(other_groups.group2_.z, 2) - pow(other_groups.group1_.w, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(anti_inverse.e1234 * self.e4, 0.0, 0.0, 0.0)
    ));
}
fn origin_geometricAntiQuotient_origin(self_: Origin, other: Origin) -> Origin {
    let self_groups = origin_grouped(self_);
    let other_groups = origin_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other.e4, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(anti_inverse.e1234 * self.e4, 0.0, 0.0, 0.0)
    ));
}
fn origin_geometricAntiQuotient_plane(self_: Origin, other: Plane) -> Origin {
    let self_groups = origin_grouped(self_);
    let other_groups = plane_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group0_.y, 2) + pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(anti_inverse.e1234 * self.e4, 0.0, 0.0, 0.0)
    ));
}
fn origin_geometricAntiQuotient_point(self_: Origin, other: Point) -> Origin {
    let self_groups = origin_grouped(self_);
    let other_groups = point_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.w, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(anti_inverse.e1234 * self.e4, 0.0, 0.0, 0.0)
    ));
}
fn plane_geometricAntiQuotient_antiScalar(self_: Plane, other: AntiScalar) -> Plane {
    let self_groups = plane_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other.e1234, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234) * self_.group0_
    ));
}
fn plane_geometricAntiQuotient_dualNum(self_: Plane, other: DualNum) -> Plane {
    let self_groups = plane_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.y, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234) * self_.group0_
    ));
}
fn plane_geometricAntiQuotient_flector(self_: Plane, other: Flector) -> Plane {
    let self_groups = plane_grouped(self_);
    let other_groups = flector_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group1_.x, 2) + pow(other_groups.group1_.y, 2) + pow(other_groups.group1_.z, 2) - pow(other_groups.group0_.w, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234) * self_.group0_
    ));
}
fn plane_geometricAntiQuotient_line(self_: Plane, other: Line) -> Plane {
    let self_groups = plane_grouped(self_);
    let other_groups = line_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(-pow(other_groups.group0_.x, 2) - pow(other_groups.group0_.y, 2) - pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234) * self_.group0_
    ));
}
fn plane_geometricAntiQuotient_motor(self_: Plane, other: Motor) -> Plane {
    let self_groups = plane_grouped(self_);
    let other_groups = motor_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.w, 2) - pow(other_groups.group0_.x, 2) - pow(other_groups.group0_.y, 2) - pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234) * self_.group0_
    ));
}
fn plane_geometricAntiQuotient_multiVector(self_: Plane, other: MultiVector) -> Plane {
    let self_groups = plane_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.y, 2) + pow(other_groups.group4_.x, 2) + pow(other_groups.group4_.y, 2) + pow(other_groups.group4_.z, 2) - pow(other_groups.group2_.x, 2) - pow(other_groups.group2_.y, 2) - pow(other_groups.group2_.z, 2) - pow(other_groups.group1_.w, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234) * self_.group0_
    ));
}
fn plane_geometricAntiQuotient_origin(self_: Plane, other: Origin) -> Plane {
    let self_groups = plane_grouped(self_);
    let other_groups = origin_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other.e4, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234) * self_.group0_
    ));
}
fn plane_geometricAntiQuotient_plane(self_: Plane, other: Plane) -> Plane {
    let self_groups = plane_grouped(self_);
    let other_groups = plane_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group0_.y, 2) + pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234) * self_.group0_
    ));
}
fn plane_geometricAntiQuotient_point(self_: Plane, other: Point) -> Plane {
    let self_groups = plane_grouped(self_);
    let other_groups = point_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.w, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(anti_inverse.e1234) * self_.group0_
    ));
}
fn point_geometricAntiQuotient_antiScalar(self_: Point, other: AntiScalar) -> Point {
    let self_groups = point_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other.e1234, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234) * self_.group0_
    ));
}
fn point_geometricAntiQuotient_dualNum(self_: Point, other: DualNum) -> Point {
    let self_groups = point_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.y, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234) * self_.group0_
    ));
}
fn point_geometricAntiQuotient_flector(self_: Point, other: Flector) -> Point {
    let self_groups = point_grouped(self_);
    let other_groups = flector_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group1_.x, 2) + pow(other_groups.group1_.y, 2) + pow(other_groups.group1_.z, 2) - pow(other_groups.group0_.w, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234) * self_.group0_
    ));
}
fn point_geometricAntiQuotient_line(self_: Point, other: Line) -> Point {
    let self_groups = point_grouped(self_);
    let other_groups = line_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(-pow(other_groups.group0_.x, 2) - pow(other_groups.group0_.y, 2) - pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234) * self_.group0_
    ));
}
fn point_geometricAntiQuotient_motor(self_: Point, other: Motor) -> Point {
    let self_groups = point_grouped(self_);
    let other_groups = motor_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.w, 2) - pow(other_groups.group0_.x, 2) - pow(other_groups.group0_.y, 2) - pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234) * self_.group0_
    ));
}
fn point_geometricAntiQuotient_multiVector(self_: Point, other: MultiVector) -> Point {
    let self_groups = point_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.y, 2) + pow(other_groups.group4_.x, 2) + pow(other_groups.group4_.y, 2) + pow(other_groups.group4_.z, 2) - pow(other_groups.group2_.x, 2) - pow(other_groups.group2_.y, 2) - pow(other_groups.group2_.z, 2) - pow(other_groups.group1_.w, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234) * self_.group0_
    ));
}
fn point_geometricAntiQuotient_origin(self_: Point, other: Origin) -> Point {
    let self_groups = point_grouped(self_);
    let other_groups = origin_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other.e4, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234) * self_.group0_
    ));
}
fn point_geometricAntiQuotient_plane(self_: Point, other: Plane) -> Point {
    let self_groups = point_grouped(self_);
    let other_groups = plane_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group0_.y, 2) + pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234) * self_.group0_
    ));
}
fn point_geometricAntiQuotient_point(self_: Point, other: Point) -> Point {
    let self_groups = point_grouped(self_);
    let other_groups = point_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.w, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(anti_inverse.e1234) * self_.group0_
    ));
}
fn scalar_geometricAntiQuotient_antiScalar(self_: Scalar, other: AntiScalar) -> Scalar {
    let self_groups = scalar_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other.e1234, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(anti_inverse.e1234 * self.scalar, 0.0, 0.0, 0.0)
    ));
}
fn scalar_geometricAntiQuotient_dualNum(self_: Scalar, other: DualNum) -> Scalar {
    let self_groups = scalar_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.y, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(anti_inverse.e1234 * self.scalar, 0.0, 0.0, 0.0)
    ));
}
fn scalar_geometricAntiQuotient_flector(self_: Scalar, other: Flector) -> Scalar {
    let self_groups = scalar_grouped(self_);
    let other_groups = flector_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group1_.x, 2) + pow(other_groups.group1_.y, 2) + pow(other_groups.group1_.z, 2) - pow(other_groups.group0_.w, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(anti_inverse.e1234 * self.scalar, 0.0, 0.0, 0.0)
    ));
}
fn scalar_geometricAntiQuotient_line(self_: Scalar, other: Line) -> Scalar {
    let self_groups = scalar_grouped(self_);
    let other_groups = line_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(-pow(other_groups.group0_.x, 2) - pow(other_groups.group0_.y, 2) - pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(anti_inverse.e1234 * self.scalar, 0.0, 0.0, 0.0)
    ));
}
fn scalar_geometricAntiQuotient_motor(self_: Scalar, other: Motor) -> Scalar {
    let self_groups = scalar_grouped(self_);
    let other_groups = motor_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.w, 2) - pow(other_groups.group0_.x, 2) - pow(other_groups.group0_.y, 2) - pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(anti_inverse.e1234 * self.scalar, 0.0, 0.0, 0.0)
    ));
}
fn scalar_geometricAntiQuotient_multiVector(self_: Scalar, other: MultiVector) -> Scalar {
    let self_groups = scalar_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.y, 2) + pow(other_groups.group4_.x, 2) + pow(other_groups.group4_.y, 2) + pow(other_groups.group4_.z, 2) - pow(other_groups.group2_.x, 2) - pow(other_groups.group2_.y, 2) - pow(other_groups.group2_.z, 2) - pow(other_groups.group1_.w, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(anti_inverse.e1234 * self.scalar, 0.0, 0.0, 0.0)
    ));
}
fn scalar_geometricAntiQuotient_origin(self_: Scalar, other: Origin) -> Scalar {
    let self_groups = scalar_grouped(self_);
    let other_groups = origin_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other.e4, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(anti_inverse.e1234 * self.scalar, 0.0, 0.0, 0.0)
    ));
}
fn scalar_geometricAntiQuotient_plane(self_: Scalar, other: Plane) -> Scalar {
    let self_groups = scalar_grouped(self_);
    let other_groups = plane_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group0_.y, 2) + pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(anti_inverse.e1234 * self.scalar, 0.0, 0.0, 0.0)
    ));
}
fn scalar_geometricAntiQuotient_point(self_: Scalar, other: Point) -> Scalar {
    let self_groups = scalar_grouped(self_);
    let other_groups = point_grouped(other);
    let anti_scalar_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(pow(other_groups.group0_.w, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let anti_scalar_product: AntiScalar = antiScalar_degroup(anti_scalar_product_groups);
    let anti_inverse_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0/(anti_scalar_product.e1234), 0.0, 0.0, 0.0)
    );
    let anti_inverse: AntiScalar = antiScalar_degroup(anti_inverse_groups);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(anti_inverse.e1234 * self.scalar, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_geometricProduct_dualNum(self_: AntiScalar, other: DualNum) -> AntiScalar {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(other_groups.group0_.x * self.e1234, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_geometricProduct_flector(self_: AntiScalar, other: Flector) -> Flector {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group1_.w * self.e1234 * -1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group0_.x * self.e1234 * -1.0, other_groups.group0_.y * self.e1234 * -1.0, other_groups.group0_.z * self.e1234 * -1.0, 0.0)
    ));
}
fn antiScalar_geometricProduct_horizon(self_: AntiScalar, other: Horizon) -> Origin {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = horizon_grouped(other);
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(self.e1234 * other.e321 * -1.0, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_geometricProduct_line(self_: AntiScalar, other: Line) -> Line {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = line_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(self.e1234), 0.0) * other_groups.group1_, 
        /* e23, e31, e12 */ vec4<f32>(0.0)
    ));
}
fn antiScalar_geometricProduct_motor(self_: AntiScalar, other: Motor) -> Motor {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self.e1234) * other_groups.group1_, 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0)
    ));
}
fn antiScalar_geometricProduct_multiVector(self_: AntiScalar, other: MultiVector) -> MultiVector {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, other_groups.group0_.x * self.e1234, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group4_.w * self.e1234 * -1.0), 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(self.e1234), 0.0) * other_groups.group3_, 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group1_.x * self.e1234 * -1.0, other_groups.group1_.y * self.e1234 * -1.0, other_groups.group1_.z * self.e1234 * -1.0, 0.0)
    ));
}
fn antiScalar_geometricProduct_plane(self_: AntiScalar, other: Plane) -> Origin {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = plane_grouped(other);
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(other_groups.group0_.w * self.e1234 * -1.0, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_geometricProduct_point(self_: AntiScalar, other: Point) -> Plane {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = point_grouped(other);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group0_.x * self.e1234 * -1.0, other_groups.group0_.y * self.e1234 * -1.0, other_groups.group0_.z * self.e1234 * -1.0, 0.0)
    ));
}
fn antiScalar_geometricProduct_scalar(self_: AntiScalar, other: Scalar) -> AntiScalar {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = scalar_grouped(other);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(self.e1234 * other.scalar, 0.0, 0.0, 0.0)
    ));
}
fn dualNum_geometricProduct_antiScalar(self_: DualNum, other: AntiScalar) -> AntiScalar {
    let self_groups = dualNum_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(self_.group0_.x * other.e1234, 0.0, 0.0, 0.0)
    ));
}
fn dualNum_geometricProduct_dualNum(self_: DualNum, other: DualNum) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(other_groups.group0_.x * self_.group0_.x, (other_groups.group0_.x * self_.group0_.y) + (other_groups.group0_.y * self_.group0_.x), 0.0, 0.0)
    ));
}
fn dualNum_geometricProduct_flector(self_: DualNum, other: Flector) -> Flector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group0_.x * other_groups.group0_.x, self_.group0_.x * other_groups.group0_.y, self_.group0_.x * other_groups.group0_.z, (self_.group0_.x * other_groups.group0_.w) - (self_.group0_.y * other_groups.group1_.w)), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.group0_.x * other_groups.group1_.x) - (self_.group0_.y * other_groups.group0_.x), (self_.group0_.x * other_groups.group1_.y) - (self_.group0_.y * other_groups.group0_.y), (self_.group0_.x * other_groups.group1_.z) - (self_.group0_.y * other_groups.group0_.z), self_.group0_.x * other_groups.group1_.w)
    ));
}
fn dualNum_geometricProduct_horizon(self_: DualNum, other: Horizon) -> Flector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = horizon_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.group0_.y * other.e321 * -1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.group0_.x * other.e321)
    ));
}
fn dualNum_geometricProduct_line(self_: DualNum, other: Line) -> Line {
    let self_groups = dualNum_grouped(self_);
    let other_groups = line_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(vec3<f32>(self_.group0_.x), 0.0) * other_groups.group0_) + (vec4<f32>(vec3<f32>(self_.group0_.y), 0.0) * other_groups.group1_), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(self_.group0_.x), 0.0) * other_groups.group1_
    ));
}
fn dualNum_geometricProduct_motor(self_: DualNum, other: Motor) -> Motor {
    let self_groups = dualNum_grouped(self_);
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ (vec4<f32>(self_.group0_.x) * other_groups.group0_) + (vec4<f32>(self_.group0_.y) * other_groups.group1_), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.group0_.x) * other_groups.group1_
    ));
}
fn dualNum_geometricProduct_multiVector(self_: DualNum, other: MultiVector) -> MultiVector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.group0_.x * other_groups.group0_.x, (self_.group0_.x * other_groups.group0_.y) + (self_.group0_.y * other_groups.group0_.x), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group0_.x * other_groups.group1_.x, self_.group0_.x * other_groups.group1_.y, self_.group0_.x * other_groups.group1_.z, (self_.group0_.x * other_groups.group1_.w) - (self_.group0_.y * other_groups.group4_.w)), 
        /* e41, e42, e43 */ (vec4<f32>(vec3<f32>(self_.group0_.x), 0.0) * other_groups.group2_) + (vec4<f32>(vec3<f32>(self_.group0_.y), 0.0) * other_groups.group3_), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(self_.group0_.x), 0.0) * other_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.group0_.x * other_groups.group4_.x) - (self_.group0_.y * other_groups.group1_.x), (self_.group0_.x * other_groups.group4_.y) - (self_.group0_.y * other_groups.group1_.y), (self_.group0_.x * other_groups.group4_.z) - (self_.group0_.y * other_groups.group1_.z), self_.group0_.x * other_groups.group4_.w)
    ));
}
fn dualNum_geometricProduct_origin(self_: DualNum, other: Origin) -> Origin {
    let self_groups = dualNum_grouped(self_);
    let other_groups = origin_grouped(other);
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(self_.group0_.x * other.e4, 0.0, 0.0, 0.0)
    ));
}
fn dualNum_geometricProduct_plane(self_: DualNum, other: Plane) -> Flector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = plane_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.group0_.y * other_groups.group0_.w * -1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.x) * other_groups.group0_
    ));
}
fn dualNum_geometricProduct_point(self_: DualNum, other: Point) -> Flector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = point_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group0_.x) * other_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.y * other_groups.group0_.x * -1.0, self_.group0_.y * other_groups.group0_.y * -1.0, self_.group0_.y * other_groups.group0_.z * -1.0, 0.0)
    ));
}
fn dualNum_geometricProduct_scalar(self_: DualNum, other: Scalar) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let other_groups = scalar_grouped(other);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(other.scalar), 0.0, 0.0) * self_.group0_
    ));
}
fn flector_geometricProduct_antiScalar(self_: Flector, other: AntiScalar) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.group1_.w * other.e1234), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.x * other.e1234, self_.group0_.y * other.e1234, self_.group0_.z * other.e1234, 0.0)
    ));
}
fn flector_geometricProduct_dualNum(self_: Flector, other: DualNum) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group0_.x * self_.group0_.x, other_groups.group0_.x * self_.group0_.y, other_groups.group0_.x * self_.group0_.z, (other_groups.group0_.x * self_.group0_.w) + (other_groups.group0_.y * self_.group1_.w)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.x * self_.group1_.x) + (other_groups.group0_.y * self_.group0_.x), (other_groups.group0_.x * self_.group1_.y) + (other_groups.group0_.y * self_.group0_.y), (other_groups.group0_.x * self_.group1_.z) + (other_groups.group0_.y * self_.group0_.z), other_groups.group0_.x * self_.group1_.w)
    ));
}
fn flector_geometricProduct_flector(self_: Flector, other: Flector) -> Motor {
    let self_groups = flector_grouped(self_);
    let other_groups = flector_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other_groups.group0_.y * self_.group1_.z) - (other_groups.group1_.y * self_.group0_.z), (other_groups.group0_.z * self_.group1_.x) - (other_groups.group1_.z * self_.group0_.x), (other_groups.group0_.x * self_.group1_.y) - (other_groups.group1_.x * self_.group0_.y), (other_groups.group1_.w * self_.group0_.w) - (other_groups.group0_.w * self_.group1_.w)) + (vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, other_groups.group1_.x) * self_.group0_.wwwx) + (vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group0_.z) * other_groups.group1_.wwwz) + (other_groups.group1_.zxyy * self_.group0_.yzxy) - (vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, other_groups.group0_.z) * self_.group1_.wwwz) - (vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group1_.y) * other_groups.group0_.wwwy) - (other_groups.group0_.zxyx * self_.group1_.yzxx), 
        /* e23, e31, e12, scalar */ vec4<f32>(-(other_groups.group0_.y * self_.group0_.z) - (other_groups.group1_.w * self_.group0_.x), -(other_groups.group0_.z * self_.group0_.x) - (other_groups.group1_.w * self_.group0_.y), -(other_groups.group0_.x * self_.group0_.y) - (other_groups.group1_.w * self_.group0_.z), (other_groups.group0_.y * self_.group0_.y) + (other_groups.group0_.z * self_.group0_.z)) + (other_groups.group0_.zxyx * self_.group0_.yzxx) - (vec4<f32>(self_.group1_.w) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, other_groups.group1_.w))
    ));
}
fn flector_geometricProduct_horizon(self_: Flector, other: Horizon) -> Motor {
    let self_groups = flector_grouped(self_);
    let other_groups = horizon_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e321) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group0_.w), 
        /* e23, e31, e12, scalar */ vec4<f32>(other.e321) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group1_.w) * vec4<f32>(-1.0)
    ));
}
fn flector_geometricProduct_line(self_: Flector, other: Line) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = line_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group1_.x * self_.group1_.w) + (other_groups.group1_.y * self_.group0_.z), (other_groups.group1_.y * self_.group1_.w) + (other_groups.group1_.z * self_.group0_.x), (other_groups.group1_.x * self_.group0_.y) + (other_groups.group1_.z * self_.group1_.w), -(other_groups.group0_.y * self_.group0_.y) - (other_groups.group0_.z * self_.group0_.z) - (other_groups.group1_.x * self_.group1_.x) - (other_groups.group1_.y * self_.group1_.y) - (other_groups.group1_.z * self_.group1_.z)) - (vec4<f32>(other_groups.group1_.zxy.xyz, other_groups.group0_.x) * self_.group0_.yzxx), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.x * self_.group1_.w) + (other_groups.group0_.y * self_.group0_.z) + (other_groups.group1_.x * self_.group0_.w) + (other_groups.group1_.y * self_.group1_.z) - (other_groups.group1_.z * self_.group1_.y), (other_groups.group0_.y * self_.group1_.w) + (other_groups.group0_.z * self_.group0_.x) + (other_groups.group1_.y * self_.group0_.w) + (other_groups.group1_.z * self_.group1_.x) - (other_groups.group1_.x * self_.group1_.z), (other_groups.group0_.x * self_.group0_.y) + (other_groups.group0_.z * self_.group1_.w) + (other_groups.group1_.x * self_.group1_.y) + (other_groups.group1_.z * self_.group0_.w) - (other_groups.group1_.y * self_.group1_.x), -(other_groups.group1_.y * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.z)) - (vec4<f32>(other_groups.group0_.zxy.xyz, other_groups.group1_.x) * self_.group0_.yzxx)
    ));
}
fn flector_geometricProduct_motor(self_: Flector, other: Motor) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = motor_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group0_.z * other_groups.group1_.y, self_.group0_.y * other_groups.group1_.w, self_.group0_.z * other_groups.group1_.w, -(self_.group0_.y * other_groups.group0_.y) - (self_.group0_.z * other_groups.group0_.z) - (self_.group1_.x * other_groups.group1_.x) - (self_.group1_.y * other_groups.group1_.y) - (self_.group1_.z * other_groups.group1_.z)) + (vec4<f32>(self_.group1_.w) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, other_groups.group0_.w)) + (self_.group0_.xxyw * other_groups.group1_.wzxw) - (vec4<f32>(other_groups.group1_.z, other_groups.group1_.x, other_groups.group1_.y, other_groups.group0_.x) * self_.group0_.yzxx), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.group0_.x * other_groups.group0_.w) + (self_.group0_.z * other_groups.group0_.y) + (self_.group1_.x * other_groups.group1_.w) + (self_.group1_.z * other_groups.group1_.y) + (self_.group1_.w * other_groups.group0_.x), (self_.group0_.x * other_groups.group0_.z) + (self_.group0_.y * other_groups.group0_.w) + (self_.group1_.x * other_groups.group1_.z) + (self_.group1_.y * other_groups.group1_.w) + (self_.group1_.w * other_groups.group0_.y), (self_.group0_.y * other_groups.group0_.x) + (self_.group0_.z * other_groups.group0_.w) + (self_.group1_.y * other_groups.group1_.x) + (self_.group1_.z * other_groups.group1_.w) + (self_.group1_.w * other_groups.group0_.z), (self_.group0_.z * other_groups.group1_.z) * -1.0) + (vec4<f32>(self_.group0_.w, self_.group0_.w, self_.group0_.w, self_.group1_.w) * other_groups.group1_) - (vec4<f32>(self_.group1_.y, self_.group1_.z, self_.group1_.x, self_.group0_.y) * other_groups.group1_.zxyy) - (vec4<f32>(other_groups.group0_.z, other_groups.group0_.x, other_groups.group0_.y, other_groups.group1_.x) * self_.group0_.yzxx)
    ));
}
fn flector_geometricProduct_multiVector(self_: Flector, other: MultiVector) -> MultiVector {
    let self_groups = flector_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, (self_.group0_.w * other_groups.group4_.w) - (self_.group1_.x * other_groups.group1_.x) - (self_.group1_.y * other_groups.group1_.y) - (self_.group1_.z * other_groups.group1_.z), 0.0, 0.0) + (vec4<f32>(vec2<f32>(self_.group0_.x), 0.0, 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group4_.x, 0.0, 0.0)) + (vec4<f32>(vec2<f32>(self_.group0_.y), 0.0, 0.0) * vec4<f32>(other_groups.group1_.y, other_groups.group4_.y, 0.0, 0.0)) + (vec4<f32>(vec2<f32>(self_.group0_.z), 0.0, 0.0) * vec4<f32>(other_groups.group1_.z, other_groups.group4_.z, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(self_.group1_.w), 0.0, 0.0) * vec4<f32>(other_groups.group4_.w, other_groups.group1_.w, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group3_.y * self_.group0_.z, other_groups.group3_.z * self_.group0_.x, other_groups.group3_.x * self_.group0_.y, -(other_groups.group2_.y * self_.group0_.y) - (other_groups.group2_.z * self_.group0_.z) - (other_groups.group3_.x * self_.group1_.x) - (other_groups.group3_.y * self_.group1_.y) - (other_groups.group3_.z * self_.group1_.z)) + (vec4<f32>(other_groups.group0_.x) * self_.group0_) + (vec4<f32>(self_.group1_.w) * vec4<f32>(other_groups.group3_.xyz.xyz, other_groups.group0_.y)) - (vec4<f32>(other_groups.group3_.zxy.xyz, other_groups.group2_.x) * self_.group0_.yzxx), 
        /* e41, e42, e43 */ vec4<f32>((self_.group0_.y * other_groups.group4_.z) + (self_.group1_.z * other_groups.group1_.y) - (self_.group0_.z * other_groups.group4_.y) - (self_.group1_.y * other_groups.group1_.z), (self_.group0_.z * other_groups.group4_.x) + (self_.group1_.x * other_groups.group1_.z) - (self_.group0_.x * other_groups.group4_.z) - (self_.group1_.z * other_groups.group1_.x), (self_.group0_.x * other_groups.group4_.y) + (self_.group1_.y * other_groups.group1_.x) - (self_.group0_.y * other_groups.group4_.x) - (self_.group1_.x * other_groups.group1_.y), 0.0) + (vec4<f32>(vec3<f32>(self_.group0_.w), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0)) + (vec4<f32>(vec3<f32>(other_groups.group4_.w), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0)) - (vec4<f32>(vec3<f32>(self_.group1_.w), 0.0) * vec4<f32>(other_groups.group4_.x, other_groups.group4_.y, other_groups.group4_.z, 0.0)) - (vec4<f32>(vec3<f32>(other_groups.group1_.w), 0.0) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((self_.group0_.y * other_groups.group1_.z) - (self_.group0_.z * other_groups.group1_.y), (self_.group0_.z * other_groups.group1_.x) - (self_.group0_.x * other_groups.group1_.z), (self_.group0_.x * other_groups.group1_.y) - (self_.group0_.y * other_groups.group1_.x), 0.0) - (vec4<f32>(vec3<f32>(self_.group1_.w), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0)) - (vec4<f32>(vec3<f32>(other_groups.group4_.w), 0.0) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.y * self_.group0_.x) + (other_groups.group2_.x * self_.group1_.w) + (other_groups.group2_.y * self_.group0_.z) + (other_groups.group3_.x * self_.group0_.w) + (other_groups.group3_.y * self_.group1_.z) - (other_groups.group3_.z * self_.group1_.y), (other_groups.group0_.y * self_.group0_.y) + (other_groups.group2_.y * self_.group1_.w) + (other_groups.group2_.z * self_.group0_.x) + (other_groups.group3_.y * self_.group0_.w) + (other_groups.group3_.z * self_.group1_.x) - (other_groups.group3_.x * self_.group1_.z), (other_groups.group0_.y * self_.group0_.z) + (other_groups.group2_.x * self_.group0_.y) + (other_groups.group2_.z * self_.group1_.w) + (other_groups.group3_.x * self_.group1_.y) + (other_groups.group3_.z * self_.group0_.w) - (other_groups.group3_.y * self_.group1_.x), -(other_groups.group3_.y * self_.group0_.y) - (other_groups.group3_.z * self_.group0_.z)) + (vec4<f32>(other_groups.group0_.x) * self_.group1_) - (vec4<f32>(other_groups.group2_.zxy.xyz, other_groups.group3_.x) * self_.group0_.yzxx)
    ));
}
fn flector_geometricProduct_origin(self_: Flector, other: Origin) -> Motor {
    let self_groups = flector_grouped(self_);
    let other_groups = origin_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e4) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group1_.w) * vec4<f32>(-1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0)
    ));
}
fn flector_geometricProduct_plane(self_: Flector, other: Plane) -> Motor {
    let self_groups = flector_grouped(self_);
    let other_groups = plane_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(-(self_.group0_.z * other_groups.group0_.y) - (self_.group1_.w * other_groups.group0_.x), -(self_.group0_.x * other_groups.group0_.z) - (self_.group1_.w * other_groups.group0_.y), -(self_.group0_.y * other_groups.group0_.x) - (self_.group1_.w * other_groups.group0_.z), (self_.group0_.z * other_groups.group0_.z) + (self_.group0_.w * other_groups.group0_.w)) + (vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group0_.y) * other_groups.group0_.wwwy) + (self_.group0_.yzxx * other_groups.group0_.zxyx), 
        /* e23, e31, e12, scalar */ vec4<f32>(other_groups.group0_.w) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group1_.w) * vec4<f32>(-1.0)
    ));
}
fn flector_geometricProduct_point(self_: Flector, other: Point) -> Motor {
    let self_groups = flector_grouped(self_);
    let other_groups = point_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((self_.group0_.w * other_groups.group0_.x) + (self_.group1_.z * other_groups.group0_.y), (self_.group0_.w * other_groups.group0_.y) + (self_.group1_.x * other_groups.group0_.z), (self_.group0_.w * other_groups.group0_.z) + (self_.group1_.y * other_groups.group0_.x), -(self_.group1_.z * other_groups.group0_.z) - (self_.group1_.w * other_groups.group0_.w)) - (vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group1_.x) * other_groups.group0_.wwwx) - (self_.group1_.yzxy * other_groups.group0_.zxyy), 
        /* e23, e31, e12, scalar */ vec4<f32>(-(self_.group0_.z * other_groups.group0_.y) - (self_.group1_.w * other_groups.group0_.x), -(self_.group0_.x * other_groups.group0_.z) - (self_.group1_.w * other_groups.group0_.y), -(self_.group0_.y * other_groups.group0_.x) - (self_.group1_.w * other_groups.group0_.z), (self_.group0_.y * other_groups.group0_.y) + (self_.group0_.z * other_groups.group0_.z)) + (self_.group0_.yzxx * other_groups.group0_.zxyx)
    ));
}
fn flector_geometricProduct_scalar(self_: Flector, other: Scalar) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = scalar_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.scalar) * self_.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other.scalar) * self_.group1_
    ));
}
fn horizon_geometricProduct_antiScalar(self_: Horizon, other: AntiScalar) -> Origin {
    let self_groups = horizon_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(other.e1234 * self.e321, 0.0, 0.0, 0.0)
    ));
}
fn horizon_geometricProduct_dualNum(self_: Horizon, other: DualNum) -> Flector {
    let self_groups = horizon_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group0_.y * self.e321), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group0_.x * self.e321)
    ));
}
fn horizon_geometricProduct_flector(self_: Horizon, other: Flector) -> Motor {
    let self_groups = horizon_grouped(self_);
    let other_groups = flector_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self.e321) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, other_groups.group0_.w) * vec4<f32>(-1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(self.e321) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, other_groups.group1_.w) * vec4<f32>(-1.0)
    ));
}
fn horizon_geometricProduct_horizon(self_: Horizon, other: Horizon) -> Scalar {
    let self_groups = horizon_grouped(self_);
    let other_groups = horizon_grouped(other);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(other.e321 * self.e321 * -1.0, 0.0, 0.0, 0.0)
    ));
}
fn horizon_geometricProduct_line(self_: Horizon, other: Line) -> Flector {
    let self_groups = horizon_grouped(self_);
    let other_groups = line_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group1_.x * self.e321, other_groups.group1_.y * self.e321, other_groups.group1_.z * self.e321, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group0_.x * self.e321, other_groups.group0_.y * self.e321, other_groups.group0_.z * self.e321, 0.0)
    ));
}
fn horizon_geometricProduct_motor(self_: Horizon, other: Motor) -> Flector {
    let self_groups = horizon_grouped(self_);
    let other_groups = motor_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self.e321) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, other_groups.group0_.w), 
        /* e423, e431, e412, e321 */ vec4<f32>(self.e321) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, other_groups.group1_.w)
    ));
}
fn horizon_geometricProduct_multiVector(self_: Horizon, other: MultiVector) -> MultiVector {
    let self_groups = horizon_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(self.e321), 0.0, 0.0) * vec4<f32>(other_groups.group4_.w, other_groups.group1_.w, 0.0, 0.0) * vec2<f32>(-1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self.e321) * vec4<f32>(other_groups.group3_.xyz.xyz, other_groups.group0_.y), 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(self.e321), 0.0) * vec4<f32>(other_groups.group4_.x, other_groups.group4_.y, other_groups.group4_.z, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(self.e321), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0) * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self.e321) * vec4<f32>(other_groups.group2_.xyz.xyz, other_groups.group0_.x)
    ));
}
fn horizon_geometricProduct_origin(self_: Horizon, other: Origin) -> AntiScalar {
    let self_groups = horizon_grouped(self_);
    let other_groups = origin_grouped(other);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(self.e321 * other.e4 * -1.0, 0.0, 0.0, 0.0)
    ));
}
fn horizon_geometricProduct_plane(self_: Horizon, other: Plane) -> Motor {
    let self_groups = horizon_grouped(self_);
    let other_groups = plane_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other_groups.group0_.x * self.e321 * -1.0, other_groups.group0_.y * self.e321 * -1.0, other_groups.group0_.z * self.e321 * -1.0, 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group0_.w * self.e321 * -1.0)
    ));
}
fn horizon_geometricProduct_point(self_: Horizon, other: Point) -> Motor {
    let self_groups = horizon_grouped(self_);
    let other_groups = point_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group0_.w * self.e321 * -1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(other_groups.group0_.x * self.e321 * -1.0, other_groups.group0_.y * self.e321 * -1.0, other_groups.group0_.z * self.e321 * -1.0, 0.0)
    ));
}
fn horizon_geometricProduct_scalar(self_: Horizon, other: Scalar) -> Horizon {
    let self_groups = horizon_grouped(self_);
    let other_groups = scalar_grouped(other);
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(self.e321 * other.scalar, 0.0, 0.0, 0.0)
    ));
}
fn line_geometricProduct_antiScalar(self_: Line, other: AntiScalar) -> Line {
    let self_groups = line_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(other.e1234), 0.0) * self_.group1_, 
        /* e23, e31, e12 */ vec4<f32>(0.0)
    ));
}
fn line_geometricProduct_dualNum(self_: Line, other: DualNum) -> Line {
    let self_groups = line_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(vec3<f32>(other_groups.group0_.x), 0.0) * self_.group0_) + (vec4<f32>(vec3<f32>(other_groups.group0_.y), 0.0) * self_.group1_), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(other_groups.group0_.x), 0.0) * self_.group1_
    ));
}
fn line_geometricProduct_flector(self_: Line, other: Flector) -> Flector {
    let self_groups = line_grouped(self_);
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group1_.x * other_groups.group1_.w) - (self_.group1_.y * other_groups.group0_.z), (self_.group1_.y * other_groups.group1_.w) - (self_.group1_.z * other_groups.group0_.x), (self_.group1_.z * other_groups.group1_.w) - (self_.group1_.x * other_groups.group0_.y), (self_.group0_.y * other_groups.group0_.y) + (self_.group0_.z * other_groups.group0_.z) - (self_.group1_.x * other_groups.group1_.x) - (self_.group1_.y * other_groups.group1_.y) - (self_.group1_.z * other_groups.group1_.z)) + (vec4<f32>(self_.group1_.zxy.xyz, self_.group0_.x) * other_groups.group0_.yzxx), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.group0_.y * other_groups.group0_.z) + (self_.group1_.x * other_groups.group0_.w) + (self_.group1_.z * other_groups.group1_.y) - (self_.group0_.x * other_groups.group1_.w) - (self_.group1_.y * other_groups.group1_.z), (self_.group0_.z * other_groups.group0_.x) + (self_.group1_.x * other_groups.group1_.z) + (self_.group1_.y * other_groups.group0_.w) - (self_.group0_.y * other_groups.group1_.w) - (self_.group1_.z * other_groups.group1_.x), (self_.group0_.x * other_groups.group0_.y) + (self_.group1_.y * other_groups.group1_.x) + (self_.group1_.z * other_groups.group0_.w) - (self_.group0_.z * other_groups.group1_.w) - (self_.group1_.x * other_groups.group1_.y), -(self_.group1_.y * other_groups.group0_.y) - (self_.group1_.z * other_groups.group0_.z)) - (vec4<f32>(self_.group0_.zxy.xyz, self_.group1_.x) * other_groups.group0_.yzxx)
    ));
}
fn line_geometricProduct_horizon(self_: Line, other: Horizon) -> Flector {
    let self_groups = line_grouped(self_);
    let other_groups = horizon_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group1_.x * other.e321, self_.group1_.y * other.e321, self_.group1_.z * other.e321, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.x * other.e321 * -1.0, self_.group0_.y * other.e321 * -1.0, self_.group0_.z * other.e321 * -1.0, 0.0)
    ));
}
fn line_geometricProduct_line(self_: Line, other: Line) -> Motor {
    let self_groups = line_grouped(self_);
    let other_groups = line_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other_groups.group0_.y * self_.group1_.z) + (other_groups.group1_.y * self_.group0_.z) - (other_groups.group0_.z * self_.group1_.y) - (other_groups.group1_.z * self_.group0_.y), (other_groups.group0_.z * self_.group1_.x) + (other_groups.group1_.z * self_.group0_.x) - (other_groups.group0_.x * self_.group1_.z) - (other_groups.group1_.x * self_.group0_.z), (other_groups.group0_.x * self_.group1_.y) + (other_groups.group1_.x * self_.group0_.y) - (other_groups.group0_.y * self_.group1_.x) - (other_groups.group1_.y * self_.group0_.x), -(other_groups.group0_.x * self_.group1_.x) - (other_groups.group0_.y * self_.group1_.y) - (other_groups.group0_.z * self_.group1_.z) - (other_groups.group1_.x * self_.group0_.x) - (other_groups.group1_.y * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.z)), 
        /* e23, e31, e12, scalar */ vec4<f32>((other_groups.group1_.y * self_.group1_.z) - (other_groups.group1_.z * self_.group1_.y), (other_groups.group1_.z * self_.group1_.x) - (other_groups.group1_.x * self_.group1_.z), (other_groups.group1_.x * self_.group1_.y) - (other_groups.group1_.y * self_.group1_.x), -(other_groups.group1_.x * self_.group1_.x) - (other_groups.group1_.y * self_.group1_.y) - (other_groups.group1_.z * self_.group1_.z))
    ));
}
fn line_geometricProduct_motor(self_: Line, other: Motor) -> Motor {
    let self_groups = line_grouped(self_);
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((self_.group0_.x * other_groups.group1_.w) + (self_.group0_.z * other_groups.group1_.y) + (self_.group1_.x * other_groups.group0_.w) + (self_.group1_.z * other_groups.group0_.y), (self_.group0_.x * other_groups.group1_.z) + (self_.group0_.y * other_groups.group1_.w) + (self_.group1_.x * other_groups.group0_.z) + (self_.group1_.y * other_groups.group0_.w), (self_.group0_.y * other_groups.group1_.x) + (self_.group0_.z * other_groups.group1_.w) + (self_.group1_.y * other_groups.group0_.x) + (self_.group1_.z * other_groups.group0_.w), -(self_.group0_.y * other_groups.group1_.y) - (self_.group0_.z * other_groups.group1_.z) - (self_.group1_.y * other_groups.group0_.y) - (self_.group1_.z * other_groups.group0_.z)) - (vec4<f32>(self_.group0_.yzx.xyz, self_.group0_.x) * other_groups.group1_.zxyx) - (vec4<f32>(self_.group1_.yzx.xyz, self_.group1_.x) * other_groups.group0_.zxyx), 
        /* e23, e31, e12, scalar */ vec4<f32>((self_.group1_.x * other_groups.group1_.w) + (self_.group1_.z * other_groups.group1_.y), (self_.group1_.x * other_groups.group1_.z) + (self_.group1_.y * other_groups.group1_.w), (self_.group1_.y * other_groups.group1_.x) + (self_.group1_.z * other_groups.group1_.w), -(self_.group1_.y * other_groups.group1_.y) - (self_.group1_.z * other_groups.group1_.z)) - (vec4<f32>(self_.group1_.yzx.xyz, self_.group1_.x) * other_groups.group1_.zxyx)
    ));
}
fn line_geometricProduct_multiVector(self_: Line, other: MultiVector) -> MultiVector {
    let self_groups = line_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, -(self_.group1_.x * other_groups.group2_.x) - (self_.group1_.y * other_groups.group2_.y) - (self_.group1_.z * other_groups.group2_.z), 0.0, 0.0) - (vec4<f32>(vec2<f32>(other_groups.group3_.x), 0.0, 0.0) * vec4<f32>(self_.group1_.x, self_.group0_.x, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(other_groups.group3_.y), 0.0, 0.0) * vec4<f32>(self_.group1_.y, self_.group0_.y, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(other_groups.group3_.z), 0.0, 0.0) * vec4<f32>(self_.group1_.z, self_.group0_.z, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group1_.x * other_groups.group4_.w) - (self_.group1_.y * other_groups.group1_.z), (self_.group1_.y * other_groups.group4_.w) - (self_.group1_.z * other_groups.group1_.x), (self_.group1_.z * other_groups.group4_.w) - (self_.group1_.x * other_groups.group1_.y), (self_.group0_.y * other_groups.group1_.y) + (self_.group0_.z * other_groups.group1_.z) - (self_.group1_.x * other_groups.group4_.x) - (self_.group1_.y * other_groups.group4_.y) - (self_.group1_.z * other_groups.group4_.z)) + (vec4<f32>(self_.group1_.zxy.xyz, self_.group0_.x) * other_groups.group1_.yzxx), 
        /* e41, e42, e43 */ (vec4<f32>(vec3<f32>(other_groups.group0_.x), 0.0) * self_.group0_) + (vec4<f32>(vec3<f32>(other_groups.group0_.y), 0.0) * self_.group1_) + (self_.group0_.zxy * other_groups.group3_.yzx) + (self_.group1_.zxy * other_groups.group2_.yzx) - (self_.group0_.yzx * other_groups.group3_.zxy) - (self_.group1_.yzx * other_groups.group2_.zxy), 
        /* e23, e31, e12 */ (vec4<f32>(vec3<f32>(other_groups.group0_.x), 0.0) * self_.group1_) + (self_.group1_.zxy * other_groups.group3_.yzx) - (self_.group1_.yzx * other_groups.group3_.zxy), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.group0_.y * other_groups.group1_.z) + (self_.group1_.x * other_groups.group1_.w) + (self_.group1_.z * other_groups.group4_.y) - (self_.group0_.x * other_groups.group4_.w) - (self_.group1_.y * other_groups.group4_.z), (self_.group0_.z * other_groups.group1_.x) + (self_.group1_.x * other_groups.group4_.z) + (self_.group1_.y * other_groups.group1_.w) - (self_.group0_.y * other_groups.group4_.w) - (self_.group1_.z * other_groups.group4_.x), (self_.group0_.x * other_groups.group1_.y) + (self_.group1_.y * other_groups.group4_.x) + (self_.group1_.z * other_groups.group1_.w) - (self_.group0_.z * other_groups.group4_.w) - (self_.group1_.x * other_groups.group4_.y), -(self_.group1_.y * other_groups.group1_.y) - (self_.group1_.z * other_groups.group1_.z)) - (vec4<f32>(self_.group0_.zxy.xyz, self_.group1_.x) * other_groups.group1_.yzxx)
    ));
}
fn line_geometricProduct_origin(self_: Line, other: Origin) -> Plane {
    let self_groups = line_grouped(self_);
    let other_groups = origin_grouped(other);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group1_.x * other.e4, self_.group1_.y * other.e4, self_.group1_.z * other.e4, 0.0)
    ));
}
fn line_geometricProduct_plane(self_: Line, other: Plane) -> Flector {
    let self_groups = line_grouped(self_);
    let other_groups = plane_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group1_.x * other_groups.group0_.w, self_.group1_.y * other_groups.group0_.w, self_.group1_.z * other_groups.group0_.w, -(self_.group1_.x * other_groups.group0_.x) - (self_.group1_.y * other_groups.group0_.y) - (self_.group1_.z * other_groups.group0_.z)), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.group1_.z * other_groups.group0_.y) - (self_.group0_.x * other_groups.group0_.w) - (self_.group1_.y * other_groups.group0_.z), (self_.group1_.x * other_groups.group0_.z) - (self_.group0_.y * other_groups.group0_.w) - (self_.group1_.z * other_groups.group0_.x), (self_.group1_.y * other_groups.group0_.x) - (self_.group0_.z * other_groups.group0_.w) - (self_.group1_.x * other_groups.group0_.y), 0.0)
    ));
}
fn line_geometricProduct_point(self_: Line, other: Point) -> Flector {
    let self_groups = line_grouped(self_);
    let other_groups = point_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group1_.y * other_groups.group0_.z) * -1.0, (self_.group1_.z * other_groups.group0_.x) * -1.0, (self_.group1_.x * other_groups.group0_.y) * -1.0, (self_.group0_.y * other_groups.group0_.y) + (self_.group0_.z * other_groups.group0_.z)) + (vec4<f32>(self_.group1_.zxy.xyz, self_.group0_.x) * other_groups.group0_.yzxx), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.group0_.y * other_groups.group0_.z) + (self_.group1_.x * other_groups.group0_.w), (self_.group0_.z * other_groups.group0_.x) + (self_.group1_.y * other_groups.group0_.w), (self_.group0_.x * other_groups.group0_.y) + (self_.group1_.z * other_groups.group0_.w), -(self_.group1_.y * other_groups.group0_.y) - (self_.group1_.z * other_groups.group0_.z)) - (vec4<f32>(self_.group0_.zxy.xyz, self_.group1_.x) * other_groups.group0_.yzxx)
    ));
}
fn line_geometricProduct_scalar(self_: Line, other: Scalar) -> Line {
    let self_groups = line_grouped(self_);
    let other_groups = scalar_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(other.scalar), 0.0) * self_.group0_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(other.scalar), 0.0) * self_.group1_
    ));
}
fn motor_geometricProduct_antiScalar(self_: Motor, other: AntiScalar) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e1234) * self_.group1_, 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0)
    ));
}
fn motor_geometricProduct_dualNum(self_: Motor, other: DualNum) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ (vec4<f32>(other_groups.group0_.x) * self_.group0_) + (vec4<f32>(other_groups.group0_.y) * self_.group1_), 
        /* e23, e31, e12, scalar */ vec4<f32>(other_groups.group0_.x) * self_.group1_
    ));
}
fn motor_geometricProduct_flector(self_: Motor, other: Flector) -> Flector {
    let self_groups = motor_grouped(self_);
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other_groups.group0_.z * self_.group0_.z) - (other_groups.group1_.y * self_.group1_.y) - (other_groups.group1_.z * self_.group1_.z) - (other_groups.group1_.w * self_.group0_.w)) + (vec4<f32>(other_groups.group1_.w, other_groups.group1_.w, other_groups.group1_.w, other_groups.group0_.w) * self_.group1_) + (vec4<f32>(self_.group1_.z, self_.group1_.x, self_.group1_.w, self_.group0_.y) * other_groups.group0_.yzzy) + (vec4<f32>(self_.group1_.w, self_.group1_.w, self_.group1_.y, self_.group0_.x) * other_groups.group0_.xyxx) - (vec4<f32>(other_groups.group0_.z, other_groups.group0_.x, other_groups.group0_.y, other_groups.group1_.x) * self_.group1_.yzxx), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.z * self_.group0_.y) + (other_groups.group1_.x * self_.group1_.w) + (other_groups.group1_.y * self_.group1_.z) - (other_groups.group1_.w * self_.group0_.x), (other_groups.group0_.x * self_.group0_.z) + (other_groups.group1_.y * self_.group1_.w) + (other_groups.group1_.z * self_.group1_.x) - (other_groups.group1_.w * self_.group0_.y), (other_groups.group0_.y * self_.group0_.x) + (other_groups.group1_.x * self_.group1_.y) + (other_groups.group1_.z * self_.group1_.w) - (other_groups.group1_.w * self_.group0_.z), 0.0) + (vec4<f32>(other_groups.group0_.w, other_groups.group0_.w, other_groups.group0_.w, other_groups.group1_.w) * self_.group1_) - (vec4<f32>(other_groups.group1_.z, other_groups.group1_.x, other_groups.group1_.y, other_groups.group0_.z) * self_.group1_.yzxz) - (vec4<f32>(self_.group0_.z, self_.group0_.x, self_.group0_.w, self_.group1_.y) * other_groups.group0_.yzzy) - (vec4<f32>(self_.group0_.w, self_.group0_.w, self_.group0_.y, self_.group1_.x) * other_groups.group0_.xyxx)
    ));
}
fn motor_geometricProduct_horizon(self_: Motor, other: Horizon) -> Flector {
    let self_groups = motor_grouped(self_);
    let other_groups = horizon_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e321) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group0_.w) * vec4<f32>(1.0, 1.0, 1.0, -1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e321) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group1_.w) * vec4<f32>(-1.0, -1.0, -1.0, 1.0)
    ));
}
fn motor_geometricProduct_line(self_: Motor, other: Line) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = line_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other_groups.group0_.x * self_.group1_.w) + (other_groups.group0_.y * self_.group1_.z) + (other_groups.group1_.x * self_.group0_.w) + (other_groups.group1_.y * self_.group0_.z), (other_groups.group0_.y * self_.group1_.w) + (other_groups.group0_.z * self_.group1_.x) + (other_groups.group1_.y * self_.group0_.w) + (other_groups.group1_.z * self_.group0_.x), (other_groups.group0_.x * self_.group1_.y) + (other_groups.group0_.z * self_.group1_.w) + (other_groups.group1_.x * self_.group0_.y) + (other_groups.group1_.z * self_.group0_.w), -(other_groups.group0_.y * self_.group1_.y) - (other_groups.group0_.z * self_.group1_.z) - (other_groups.group1_.y * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.z)) - (vec4<f32>(other_groups.group0_.zxy.xyz, other_groups.group0_.x) * self_.group1_.yzxx) - (vec4<f32>(other_groups.group1_.zxy.xyz, other_groups.group1_.x) * self_.group0_.yzxx), 
        /* e23, e31, e12, scalar */ vec4<f32>((other_groups.group1_.x * self_.group1_.w) + (other_groups.group1_.y * self_.group1_.z), (other_groups.group1_.y * self_.group1_.w) + (other_groups.group1_.z * self_.group1_.x), (other_groups.group1_.x * self_.group1_.y) + (other_groups.group1_.z * self_.group1_.w), -(other_groups.group1_.y * self_.group1_.y) - (other_groups.group1_.z * self_.group1_.z)) - (vec4<f32>(other_groups.group1_.zxy.xyz, other_groups.group1_.x) * self_.group1_.yzxx)
    ));
}
fn motor_geometricProduct_motor(self_: Motor, other: Motor) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other_groups.group0_.y * self_.group1_.z) + (other_groups.group0_.w * self_.group1_.x) + (other_groups.group1_.y * self_.group0_.z) + (other_groups.group1_.w * self_.group0_.x), (other_groups.group0_.z * self_.group1_.x) + (other_groups.group0_.w * self_.group1_.y) + (other_groups.group1_.z * self_.group0_.x) + (other_groups.group1_.w * self_.group0_.y), (other_groups.group0_.z * self_.group1_.w) + (other_groups.group0_.w * self_.group1_.z) + (other_groups.group1_.z * self_.group0_.w) + (other_groups.group1_.w * self_.group0_.z), -(other_groups.group0_.y * self_.group1_.y) - (other_groups.group0_.z * self_.group1_.z) - (other_groups.group1_.y * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.z)) + (other_groups.group0_.xyxw * self_.group1_.wwyw) + (other_groups.group1_.xyxw * self_.group0_.wwyw) - (other_groups.group0_.zxyx * self_.group1_.yzxx) - (other_groups.group1_.zxyx * self_.group0_.yzxx), 
        /* e23, e31, e12, scalar */ vec4<f32>((other_groups.group1_.y * self_.group1_.z) + (other_groups.group1_.w * self_.group1_.x), (other_groups.group1_.z * self_.group1_.x) + (other_groups.group1_.w * self_.group1_.y), (other_groups.group1_.z * self_.group1_.w) + (other_groups.group1_.w * self_.group1_.z), -(other_groups.group1_.y * self_.group1_.y) - (other_groups.group1_.z * self_.group1_.z)) + (other_groups.group1_.xyxw * self_.group1_.wwyw) - (other_groups.group1_.zxyx * self_.group1_.yzxx)
    ));
}
fn motor_geometricProduct_multiVector(self_: Motor, other: MultiVector) -> MultiVector {
    let self_groups = motor_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, (other_groups.group0_.y * self_.group1_.w) - (other_groups.group3_.x * self_.group0_.x) - (other_groups.group3_.y * self_.group0_.y) - (other_groups.group3_.z * self_.group0_.z), 0.0, 0.0) + (vec4<f32>(vec2<f32>(other_groups.group0_.x), 0.0, 0.0) * vec4<f32>(self_.group1_.w, self_.group0_.w, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(self_.group1_.x), 0.0, 0.0) * vec4<f32>(other_groups.group3_.x, other_groups.group2_.x, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(self_.group1_.y), 0.0, 0.0) * vec4<f32>(other_groups.group3_.y, other_groups.group2_.y, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(self_.group1_.z), 0.0, 0.0) * vec4<f32>(other_groups.group3_.z, other_groups.group2_.z, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group1_.w * other_groups.group1_.x, self_.group1_.y * other_groups.group4_.w, self_.group1_.z * other_groups.group4_.w, (self_.group0_.y * other_groups.group1_.y) + (self_.group0_.z * other_groups.group1_.z) - (self_.group0_.w * other_groups.group4_.w) - (self_.group1_.y * other_groups.group4_.y) - (self_.group1_.z * other_groups.group4_.z)) + (vec4<f32>(self_.group1_.z, self_.group1_.w, self_.group1_.w, self_.group0_.x) * other_groups.group1_.yyzx) + (vec4<f32>(other_groups.group4_.w, other_groups.group1_.z, other_groups.group1_.x, other_groups.group1_.w) * self_.group1_.xxyw) - (vec4<f32>(other_groups.group1_.z, other_groups.group1_.x, other_groups.group1_.y, other_groups.group4_.x) * self_.group1_.yzxx), 
        /* e41, e42, e43 */ (vec4<f32>(vec3<f32>(other_groups.group0_.x), 0.0) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0)) + (vec4<f32>(vec3<f32>(other_groups.group0_.y), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0)) + (vec4<f32>(self_.group0_.z, self_.group0_.x, self_.group0_.w, 0.0) * other_groups.group3_.yzz) + (vec4<f32>(self_.group0_.w, self_.group0_.w, self_.group0_.y, 0.0) * other_groups.group3_.xyx) + (vec4<f32>(self_.group1_.z, self_.group1_.x, self_.group1_.w, 0.0) * other_groups.group2_.yzz) + (vec4<f32>(self_.group1_.w, self_.group1_.w, self_.group1_.y, 0.0) * other_groups.group2_.xyx) - (vec4<f32>(self_.group0_.y, self_.group0_.z, self_.group0_.x, 0.0) * other_groups.group3_.zxy) - (vec4<f32>(self_.group1_.y, self_.group1_.z, self_.group1_.x, 0.0) * other_groups.group2_.zxy), 
        /* e23, e31, e12 */ (vec4<f32>(vec3<f32>(other_groups.group0_.x), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0)) + (vec4<f32>(self_.group1_.z, self_.group1_.x, self_.group1_.w, 0.0) * other_groups.group3_.yzz) + (vec4<f32>(self_.group1_.w, self_.group1_.w, self_.group1_.y, 0.0) * other_groups.group3_.xyx) - (vec4<f32>(self_.group1_.y, self_.group1_.z, self_.group1_.x, 0.0) * other_groups.group3_.zxy), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.group0_.y * other_groups.group1_.z) + (self_.group1_.z * other_groups.group4_.y) + (self_.group1_.w * other_groups.group4_.x) - (self_.group0_.x * other_groups.group4_.w), (self_.group0_.z * other_groups.group1_.x) + (self_.group1_.y * other_groups.group1_.w) + (self_.group1_.w * other_groups.group4_.y) - (self_.group0_.y * other_groups.group4_.w), (self_.group0_.x * other_groups.group1_.y) + (self_.group1_.z * other_groups.group1_.w) + (self_.group1_.w * other_groups.group4_.z) - (self_.group0_.z * other_groups.group4_.w), 0.0) + (vec4<f32>(other_groups.group1_.w, other_groups.group4_.z, other_groups.group4_.x, other_groups.group4_.w) * self_.group1_.xxyw) - (vec4<f32>(self_.group0_.z, self_.group0_.x, self_.group0_.y, self_.group1_.x) * other_groups.group1_.yzxx) - (vec4<f32>(self_.group0_.w, self_.group0_.w, self_.group0_.w, self_.group1_.y) * other_groups.group1_.xyzy) - (vec4<f32>(other_groups.group4_.z, other_groups.group4_.x, other_groups.group4_.y, other_groups.group1_.z) * self_.group1_.yzxz)
    ));
}
fn motor_geometricProduct_origin(self_: Motor, other: Origin) -> Flector {
    let self_groups = motor_grouped(self_);
    let other_groups = origin_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.group1_.w * other.e4), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group1_.x * other.e4, self_.group1_.y * other.e4, self_.group1_.z * other.e4, 0.0)
    ));
}
fn motor_geometricProduct_plane(self_: Motor, other: Plane) -> Flector {
    let self_groups = motor_grouped(self_);
    let other_groups = plane_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group1_.x * other_groups.group0_.w, self_.group1_.y * other_groups.group0_.w, self_.group1_.z * other_groups.group0_.w, -(self_.group0_.w * other_groups.group0_.w) - (self_.group1_.x * other_groups.group0_.x) - (self_.group1_.y * other_groups.group0_.y) - (self_.group1_.z * other_groups.group0_.z)), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.group1_.z * other_groups.group0_.y) + (self_.group1_.w * other_groups.group0_.x) - (self_.group0_.x * other_groups.group0_.w) - (self_.group1_.y * other_groups.group0_.z), (self_.group1_.x * other_groups.group0_.z) + (self_.group1_.w * other_groups.group0_.y) - (self_.group0_.y * other_groups.group0_.w) - (self_.group1_.z * other_groups.group0_.x), (self_.group1_.y * other_groups.group0_.x) + (self_.group1_.w * other_groups.group0_.z) - (self_.group0_.z * other_groups.group0_.w) - (self_.group1_.x * other_groups.group0_.y), self_.group1_.w * other_groups.group0_.w)
    ));
}
fn motor_geometricProduct_point(self_: Motor, other: Point) -> Flector {
    let self_groups = motor_grouped(self_);
    let other_groups = point_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group1_.y * other_groups.group0_.z) * -1.0, (self_.group1_.z * other_groups.group0_.x) * -1.0, (self_.group1_.x * other_groups.group0_.y) * -1.0, (self_.group0_.z * other_groups.group0_.z) + (self_.group1_.w * other_groups.group0_.w)) + (vec4<f32>(self_.group1_.z, self_.group1_.x, self_.group1_.y, self_.group0_.x) * other_groups.group0_.yzxx) + (vec4<f32>(self_.group1_.w, self_.group1_.w, self_.group1_.w, self_.group0_.y) * other_groups.group0_.xyzy), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.group0_.y * other_groups.group0_.z) + (self_.group1_.x * other_groups.group0_.w), (self_.group0_.z * other_groups.group0_.x) + (self_.group1_.y * other_groups.group0_.w), (self_.group0_.x * other_groups.group0_.y) + (self_.group1_.z * other_groups.group0_.w), (self_.group1_.z * other_groups.group0_.z) * -1.0) - (vec4<f32>(self_.group0_.z, self_.group0_.x, self_.group0_.y, self_.group1_.x) * other_groups.group0_.yzxx) - (vec4<f32>(self_.group0_.w, self_.group0_.w, self_.group0_.w, self_.group1_.y) * other_groups.group0_.xyzy)
    ));
}
fn motor_geometricProduct_scalar(self_: Motor, other: Scalar) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = scalar_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.scalar) * self_.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(other.scalar) * self_.group1_
    ));
}
fn multiVector_geometricProduct_antiScalar(self_: MultiVector, other: AntiScalar) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, self_.group0_.x * other.e1234, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.group4_.w * other.e1234), 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(other.e1234), 0.0) * self_.group3_, 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group1_.x * other.e1234, self_.group1_.y * other.e1234, self_.group1_.z * other.e1234, 0.0)
    ));
}
fn multiVector_geometricProduct_dualNum(self_: MultiVector, other: DualNum) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other_groups.group0_.x * self_.group0_.x, (other_groups.group0_.x * self_.group0_.y) + (other_groups.group0_.y * self_.group0_.x), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group0_.x * self_.group1_.x, other_groups.group0_.x * self_.group1_.y, other_groups.group0_.x * self_.group1_.z, (other_groups.group0_.x * self_.group1_.w) + (other_groups.group0_.y * self_.group4_.w)), 
        /* e41, e42, e43 */ (vec4<f32>(vec3<f32>(other_groups.group0_.x), 0.0) * self_.group2_) + (vec4<f32>(vec3<f32>(other_groups.group0_.y), 0.0) * self_.group3_), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(other_groups.group0_.x), 0.0) * self_.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.x * self_.group4_.x) + (other_groups.group0_.y * self_.group1_.x), (other_groups.group0_.x * self_.group4_.y) + (other_groups.group0_.y * self_.group1_.y), (other_groups.group0_.x * self_.group4_.z) + (other_groups.group0_.y * self_.group1_.z), other_groups.group0_.x * self_.group4_.w)
    ));
}
fn multiVector_geometricProduct_flector(self_: MultiVector, other: Flector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = flector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, (other_groups.group1_.w * self_.group1_.w) - (other_groups.group0_.x * self_.group4_.x) - (other_groups.group0_.y * self_.group4_.y) - (other_groups.group0_.z * self_.group4_.z), 0.0, 0.0) + (vec4<f32>(vec2<f32>(self_.group1_.x), 0.0, 0.0) * vec4<f32>(other_groups.group0_.x, other_groups.group1_.x, 0.0, 0.0)) + (vec4<f32>(vec2<f32>(self_.group1_.y), 0.0, 0.0) * vec4<f32>(other_groups.group0_.y, other_groups.group1_.y, 0.0, 0.0)) + (vec4<f32>(vec2<f32>(self_.group1_.z), 0.0, 0.0) * vec4<f32>(other_groups.group0_.z, other_groups.group1_.z, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(self_.group4_.w), 0.0, 0.0) * vec4<f32>(other_groups.group1_.w, other_groups.group0_.w, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group3_.x * other_groups.group1_.w) - (self_.group3_.y * other_groups.group0_.z), (self_.group3_.y * other_groups.group1_.w) - (self_.group3_.z * other_groups.group0_.x), (self_.group3_.z * other_groups.group1_.w) - (self_.group3_.x * other_groups.group0_.y), (self_.group2_.y * other_groups.group0_.y) + (self_.group2_.z * other_groups.group0_.z) - (self_.group0_.y * other_groups.group1_.w) - (self_.group3_.x * other_groups.group1_.x) - (self_.group3_.y * other_groups.group1_.y) - (self_.group3_.z * other_groups.group1_.z)) + (vec4<f32>(self_.group0_.x) * other_groups.group0_) + (vec4<f32>(self_.group3_.zxy.xyz, self_.group2_.x) * other_groups.group0_.yzxx), 
        /* e41, e42, e43 */ vec4<f32>((other_groups.group0_.y * self_.group4_.z) + (other_groups.group1_.z * self_.group1_.y) - (other_groups.group0_.z * self_.group4_.y) - (other_groups.group1_.y * self_.group1_.z), (other_groups.group0_.z * self_.group4_.x) + (other_groups.group1_.x * self_.group1_.z) - (other_groups.group0_.x * self_.group4_.z) - (other_groups.group1_.z * self_.group1_.x), (other_groups.group0_.x * self_.group4_.y) + (other_groups.group1_.y * self_.group1_.x) - (other_groups.group0_.y * self_.group4_.x) - (other_groups.group1_.x * self_.group1_.y), 0.0) + (vec4<f32>(vec3<f32>(other_groups.group1_.w), 0.0) * vec4<f32>(self_.group4_.x, self_.group4_.y, self_.group4_.z, 0.0)) + (vec4<f32>(vec3<f32>(self_.group1_.w), 0.0) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0)) - (vec4<f32>(vec3<f32>(other_groups.group0_.w), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0)) - (vec4<f32>(vec3<f32>(self_.group4_.w), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((other_groups.group0_.z * self_.group1_.y) - (other_groups.group0_.y * self_.group1_.z), (other_groups.group0_.x * self_.group1_.z) - (other_groups.group0_.z * self_.group1_.x), (other_groups.group0_.y * self_.group1_.x) - (other_groups.group0_.x * self_.group1_.y), 0.0) - (vec4<f32>(vec3<f32>(other_groups.group1_.w), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0)) - (vec4<f32>(vec3<f32>(self_.group4_.w), 0.0) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.group2_.y * other_groups.group0_.z) + (self_.group3_.x * other_groups.group0_.w) + (self_.group3_.z * other_groups.group1_.y) - (self_.group2_.x * other_groups.group1_.w) - (self_.group3_.y * other_groups.group1_.z), (self_.group2_.z * other_groups.group0_.x) + (self_.group3_.x * other_groups.group1_.z) + (self_.group3_.y * other_groups.group0_.w) - (self_.group2_.y * other_groups.group1_.w) - (self_.group3_.z * other_groups.group1_.x), (self_.group2_.x * other_groups.group0_.y) + (self_.group3_.y * other_groups.group1_.x) + (self_.group3_.z * other_groups.group0_.w) - (self_.group2_.z * other_groups.group1_.w) - (self_.group3_.x * other_groups.group1_.y), (self_.group3_.z * other_groups.group0_.z) * -1.0) + (vec4<f32>(self_.group0_.x) * other_groups.group1_) - (vec4<f32>(self_.group0_.yy.xy, self_.group0_.y, self_.group3_.x) * other_groups.group0_.xyzx) - (vec4<f32>(self_.group2_.zxy.xyz, self_.group3_.y) * other_groups.group0_.yzxy)
    ));
}
fn multiVector_geometricProduct_horizon(self_: MultiVector, other: Horizon) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = horizon_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(other.e321), 0.0, 0.0) * vec4<f32>(self_.group4_.w, self_.group1_.w, 0.0, 0.0) * vec2<f32>(-1.0, 1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(other.e321) * vec4<f32>(self_.group3_.xyz.xyz, self_.group0_.y) * vec4<f32>(1.0, 1.0, 1.0, -1.0), 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(other.e321), 0.0) * vec4<f32>(self_.group4_.x, self_.group4_.y, self_.group4_.z, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(other.e321), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0) * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e321) * vec4<f32>(self_.group2_.xyz.xyz, self_.group0_.x) * vec4<f32>(-1.0, -1.0, -1.0, 1.0)
    ));
}
fn multiVector_geometricProduct_line(self_: MultiVector, other: Line) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = line_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, -(other_groups.group1_.x * self_.group2_.x) - (other_groups.group1_.y * self_.group2_.y) - (other_groups.group1_.z * self_.group2_.z), 0.0, 0.0) - (vec4<f32>(vec2<f32>(self_.group3_.x), 0.0, 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group0_.x, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(self_.group3_.y), 0.0, 0.0) * vec4<f32>(other_groups.group1_.y, other_groups.group0_.y, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(self_.group3_.z), 0.0, 0.0) * vec4<f32>(other_groups.group1_.z, other_groups.group0_.z, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group1_.x * self_.group4_.w) + (other_groups.group1_.y * self_.group1_.z), (other_groups.group1_.y * self_.group4_.w) + (other_groups.group1_.z * self_.group1_.x), (other_groups.group1_.x * self_.group1_.y) + (other_groups.group1_.z * self_.group4_.w), -(other_groups.group0_.y * self_.group1_.y) - (other_groups.group0_.z * self_.group1_.z) - (other_groups.group1_.x * self_.group4_.x) - (other_groups.group1_.y * self_.group4_.y) - (other_groups.group1_.z * self_.group4_.z)) - (vec4<f32>(other_groups.group1_.zxy.xyz, other_groups.group0_.x) * self_.group1_.yzxx), 
        /* e41, e42, e43 */ (vec4<f32>(vec3<f32>(self_.group0_.x), 0.0) * other_groups.group0_) + (vec4<f32>(vec3<f32>(self_.group0_.y), 0.0) * other_groups.group1_) + (other_groups.group0_.yzx * self_.group3_.zxy) + (other_groups.group1_.yzx * self_.group2_.zxy) - (other_groups.group0_.zxy * self_.group3_.yzx) - (other_groups.group1_.zxy * self_.group2_.yzx), 
        /* e23, e31, e12 */ (vec4<f32>(vec3<f32>(self_.group0_.x), 0.0) * other_groups.group1_) + (other_groups.group1_.yzx * self_.group3_.zxy) - (other_groups.group1_.zxy * self_.group3_.yzx), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.x * self_.group4_.w) + (other_groups.group0_.y * self_.group1_.z) + (other_groups.group1_.x * self_.group1_.w) + (other_groups.group1_.y * self_.group4_.z) - (other_groups.group1_.z * self_.group4_.y), (other_groups.group0_.y * self_.group4_.w) + (other_groups.group0_.z * self_.group1_.x) + (other_groups.group1_.y * self_.group1_.w) + (other_groups.group1_.z * self_.group4_.x) - (other_groups.group1_.x * self_.group4_.z), (other_groups.group0_.x * self_.group1_.y) + (other_groups.group0_.z * self_.group4_.w) + (other_groups.group1_.x * self_.group4_.y) + (other_groups.group1_.z * self_.group1_.w) - (other_groups.group1_.y * self_.group4_.x), -(other_groups.group1_.y * self_.group1_.y) - (other_groups.group1_.z * self_.group1_.z)) - (vec4<f32>(other_groups.group0_.zxy.xyz, other_groups.group1_.x) * self_.group1_.yzxx)
    ));
}
fn multiVector_geometricProduct_motor(self_: MultiVector, other: Motor) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = motor_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, (self_.group0_.y * other_groups.group1_.w) - (self_.group3_.x * other_groups.group0_.x) - (self_.group3_.y * other_groups.group0_.y) - (self_.group3_.z * other_groups.group0_.z), 0.0, 0.0) + (vec4<f32>(vec2<f32>(self_.group0_.x), 0.0, 0.0) * vec4<f32>(other_groups.group1_.w, other_groups.group0_.w, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(other_groups.group1_.x), 0.0, 0.0) * vec4<f32>(self_.group3_.x, self_.group2_.x, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(other_groups.group1_.y), 0.0, 0.0) * vec4<f32>(self_.group3_.y, self_.group2_.y, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(other_groups.group1_.z), 0.0, 0.0) * vec4<f32>(self_.group3_.z, self_.group2_.z, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group1_.y * self_.group1_.z) + (other_groups.group1_.w * self_.group1_.x), (other_groups.group1_.z * self_.group1_.x) + (other_groups.group1_.w * self_.group1_.y), (other_groups.group1_.z * self_.group4_.w) + (other_groups.group1_.w * self_.group1_.z), (other_groups.group0_.w * self_.group4_.w) - (other_groups.group0_.y * self_.group1_.y) - (other_groups.group0_.z * self_.group1_.z) - (other_groups.group1_.x * self_.group4_.x) - (other_groups.group1_.y * self_.group4_.y) - (other_groups.group1_.z * self_.group4_.z)) + (vec4<f32>(self_.group4_.w, self_.group4_.w, self_.group1_.y, self_.group1_.w) * other_groups.group1_.xyxw) - (vec4<f32>(other_groups.group1_.z, other_groups.group1_.x, other_groups.group1_.y, other_groups.group0_.x) * self_.group1_.yzxx), 
        /* e41, e42, e43 */ (vec4<f32>(vec3<f32>(self_.group0_.x), 0.0) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0)) + (vec4<f32>(vec3<f32>(self_.group0_.y), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0)) + (vec4<f32>(other_groups.group0_.y, other_groups.group0_.w, other_groups.group0_.w, 0.0) * self_.group3_.zyz) + (vec4<f32>(other_groups.group0_.w, other_groups.group0_.z, other_groups.group0_.x, 0.0) * self_.group3_.xxy) + (vec4<f32>(other_groups.group1_.y, other_groups.group1_.w, other_groups.group1_.w, 0.0) * self_.group2_.zyz) + (vec4<f32>(other_groups.group1_.w, other_groups.group1_.z, other_groups.group1_.x, 0.0) * self_.group2_.xxy) - (vec4<f32>(other_groups.group0_.z, other_groups.group0_.x, other_groups.group0_.y, 0.0) * self_.group3_.yzx) - (vec4<f32>(other_groups.group1_.z, other_groups.group1_.x, other_groups.group1_.y, 0.0) * self_.group2_.yzx), 
        /* e23, e31, e12 */ (vec4<f32>(vec3<f32>(self_.group0_.x), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0)) + (vec4<f32>(other_groups.group1_.y, other_groups.group1_.w, other_groups.group1_.w, 0.0) * self_.group3_.zyz) + (vec4<f32>(other_groups.group1_.w, other_groups.group1_.z, other_groups.group1_.x, 0.0) * self_.group3_.xxy) - (vec4<f32>(other_groups.group1_.z, other_groups.group1_.x, other_groups.group1_.y, 0.0) * self_.group3_.yzx), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.y * self_.group1_.z) + (other_groups.group0_.w * self_.group1_.x) + (other_groups.group1_.x * self_.group1_.w) + (other_groups.group1_.y * self_.group4_.z) + (other_groups.group1_.w * self_.group4_.x), (other_groups.group0_.z * self_.group1_.x) + (other_groups.group0_.w * self_.group1_.y) + (other_groups.group1_.y * self_.group1_.w) + (other_groups.group1_.z * self_.group4_.x) + (other_groups.group1_.w * self_.group4_.y), (other_groups.group0_.x * self_.group1_.y) + (other_groups.group0_.w * self_.group1_.z) + (other_groups.group1_.x * self_.group4_.y) + (other_groups.group1_.z * self_.group1_.w) + (other_groups.group1_.w * self_.group4_.z), (other_groups.group1_.z * self_.group1_.z) * -1.0) + (vec4<f32>(self_.group4_.w) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, other_groups.group1_.w)) - (vec4<f32>(other_groups.group0_.z, other_groups.group0_.x, other_groups.group0_.y, other_groups.group1_.x) * self_.group1_.yzxx) - (vec4<f32>(self_.group4_.y, self_.group4_.z, self_.group4_.x, self_.group1_.y) * other_groups.group1_.zxyy)
    ));
}
fn multiVector_geometricProduct_multiVector(self_: MultiVector, other: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, (other_groups.group0_.y * self_.group0_.x) + (other_groups.group4_.w * self_.group1_.w) - (other_groups.group3_.x * self_.group2_.x) - (other_groups.group3_.y * self_.group2_.y) - (other_groups.group3_.z * self_.group2_.z) - (other_groups.group1_.x * self_.group4_.x) - (other_groups.group1_.y * self_.group4_.y) - (other_groups.group1_.z * self_.group4_.z), 0.0, 0.0) + (vec4<f32>(vec2<f32>(other_groups.group0_.x), 0.0, 0.0) * self_.group0_) + (vec4<f32>(vec2<f32>(self_.group1_.x), 0.0, 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group4_.x, 0.0, 0.0)) + (vec4<f32>(vec2<f32>(self_.group1_.y), 0.0, 0.0) * vec4<f32>(other_groups.group1_.y, other_groups.group4_.y, 0.0, 0.0)) + (vec4<f32>(vec2<f32>(self_.group1_.z), 0.0, 0.0) * vec4<f32>(other_groups.group1_.z, other_groups.group4_.z, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(self_.group3_.x), 0.0, 0.0) * vec4<f32>(other_groups.group3_.x, other_groups.group2_.x, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(self_.group3_.y), 0.0, 0.0) * vec4<f32>(other_groups.group3_.y, other_groups.group2_.y, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(self_.group3_.z), 0.0, 0.0) * vec4<f32>(other_groups.group3_.z, other_groups.group2_.z, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(self_.group4_.w), 0.0, 0.0) * vec4<f32>(other_groups.group4_.w, other_groups.group1_.w, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group3_.y * self_.group1_.z) + (self_.group3_.x * other_groups.group4_.w) - (self_.group3_.y * other_groups.group1_.z), (other_groups.group3_.z * self_.group1_.x) + (self_.group3_.y * other_groups.group4_.w) - (self_.group3_.z * other_groups.group1_.x), (other_groups.group3_.x * self_.group1_.y) + (self_.group3_.z * other_groups.group4_.w) - (self_.group3_.x * other_groups.group1_.y), (self_.group2_.y * other_groups.group1_.y) + (self_.group2_.z * other_groups.group1_.z) - (self_.group0_.y * other_groups.group4_.w) - (other_groups.group2_.y * self_.group1_.y) - (other_groups.group2_.z * self_.group1_.z) - (other_groups.group3_.x * self_.group4_.x) - (other_groups.group3_.y * self_.group4_.y) - (other_groups.group3_.z * self_.group4_.z) - (self_.group3_.x * other_groups.group4_.x) - (self_.group3_.y * other_groups.group4_.y) - (self_.group3_.z * other_groups.group4_.z)) + (vec4<f32>(other_groups.group0_.x) * self_.group1_) + (vec4<f32>(self_.group0_.x) * other_groups.group1_) + (vec4<f32>(self_.group4_.w) * vec4<f32>(other_groups.group3_.xyz.xyz, other_groups.group0_.y)) + (vec4<f32>(self_.group3_.zxy.xyz, self_.group2_.x) * other_groups.group1_.yzxx) - (vec4<f32>(other_groups.group3_.zxy.xyz, other_groups.group2_.x) * self_.group1_.yzxx), 
        /* e41, e42, e43 */ vec4<f32>((other_groups.group1_.y * self_.group4_.z) + (other_groups.group4_.z * self_.group1_.y) - (other_groups.group1_.z * self_.group4_.y) - (other_groups.group4_.y * self_.group1_.z), (other_groups.group1_.z * self_.group4_.x) + (other_groups.group4_.x * self_.group1_.z) - (other_groups.group1_.x * self_.group4_.z) - (other_groups.group4_.z * self_.group1_.x), (other_groups.group1_.x * self_.group4_.y) + (other_groups.group4_.y * self_.group1_.x) - (other_groups.group1_.y * self_.group4_.x) - (other_groups.group4_.x * self_.group1_.y), 0.0) + (vec4<f32>(vec3<f32>(other_groups.group0_.x), 0.0) * self_.group2_) + (vec4<f32>(vec3<f32>(other_groups.group0_.y), 0.0) * self_.group3_) + (vec4<f32>(vec3<f32>(self_.group0_.x), 0.0) * other_groups.group2_) + (vec4<f32>(vec3<f32>(self_.group0_.y), 0.0) * other_groups.group3_) + (vec4<f32>(vec3<f32>(other_groups.group4_.w), 0.0) * vec4<f32>(self_.group4_.x, self_.group4_.y, self_.group4_.z, 0.0)) + (vec4<f32>(vec3<f32>(self_.group1_.w), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0)) + (other_groups.group2_.yzx * self_.group3_.zxy) + (other_groups.group3_.yzx * self_.group2_.zxy) - (vec4<f32>(vec3<f32>(other_groups.group1_.w), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0)) - (vec4<f32>(vec3<f32>(self_.group4_.w), 0.0) * vec4<f32>(other_groups.group4_.x, other_groups.group4_.y, other_groups.group4_.z, 0.0)) - (other_groups.group2_.zxy * self_.group3_.yzx) - (other_groups.group3_.zxy * self_.group2_.yzx), 
        /* e23, e31, e12 */ vec4<f32>((other_groups.group1_.z * self_.group1_.y) - (other_groups.group1_.y * self_.group1_.z), (other_groups.group1_.x * self_.group1_.z) - (other_groups.group1_.z * self_.group1_.x), (other_groups.group1_.y * self_.group1_.x) - (other_groups.group1_.x * self_.group1_.y), 0.0) + (vec4<f32>(vec3<f32>(other_groups.group0_.x), 0.0) * self_.group3_) + (vec4<f32>(vec3<f32>(self_.group0_.x), 0.0) * other_groups.group3_) + (other_groups.group3_.yzx * self_.group3_.zxy) - (vec4<f32>(vec3<f32>(other_groups.group4_.w), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0)) - (vec4<f32>(vec3<f32>(self_.group4_.w), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0)) - (other_groups.group3_.zxy * self_.group3_.yzx), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.y * self_.group1_.x) + (other_groups.group2_.x * self_.group4_.w) + (other_groups.group2_.y * self_.group1_.z) + (other_groups.group3_.x * self_.group1_.w) + (other_groups.group3_.y * self_.group4_.z) + (self_.group2_.y * other_groups.group1_.z) + (self_.group3_.x * other_groups.group1_.w) + (self_.group3_.z * other_groups.group4_.y) - (other_groups.group3_.z * self_.group4_.y) - (self_.group2_.x * other_groups.group4_.w) - (self_.group3_.y * other_groups.group4_.z), (other_groups.group0_.y * self_.group1_.y) + (other_groups.group2_.y * self_.group4_.w) + (other_groups.group2_.z * self_.group1_.x) + (other_groups.group3_.y * self_.group1_.w) + (other_groups.group3_.z * self_.group4_.x) + (self_.group2_.z * other_groups.group1_.x) + (self_.group3_.x * other_groups.group4_.z) + (self_.group3_.y * other_groups.group1_.w) - (other_groups.group3_.x * self_.group4_.z) - (self_.group2_.y * other_groups.group4_.w) - (self_.group3_.z * other_groups.group4_.x), (other_groups.group0_.y * self_.group1_.z) + (other_groups.group2_.x * self_.group1_.y) + (other_groups.group2_.z * self_.group4_.w) + (other_groups.group3_.x * self_.group4_.y) + (other_groups.group3_.z * self_.group1_.w) + (self_.group2_.x * other_groups.group1_.y) + (self_.group3_.y * other_groups.group4_.x) + (self_.group3_.z * other_groups.group1_.w) - (other_groups.group3_.y * self_.group4_.x) - (self_.group2_.z * other_groups.group4_.w) - (self_.group3_.x * other_groups.group4_.y), -(other_groups.group3_.y * self_.group1_.y) - (other_groups.group3_.z * self_.group1_.z) - (self_.group3_.z * other_groups.group1_.z)) + (vec4<f32>(other_groups.group0_.x) * self_.group4_) + (vec4<f32>(self_.group0_.x) * other_groups.group4_) - (vec4<f32>(self_.group0_.yy.xy, self_.group0_.y, self_.group3_.x) * other_groups.group1_.xyzx) - (vec4<f32>(other_groups.group2_.zxy.xyz, other_groups.group3_.x) * self_.group1_.yzxx) - (vec4<f32>(self_.group2_.zxy.xyz, self_.group3_.y) * other_groups.group1_.yzxy)
    ));
}
fn multiVector_geometricProduct_origin(self_: MultiVector, other: Origin) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = origin_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, self_.group4_.w * other.e4 * -1.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.group0_.x * other.e4), 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(other.e4), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group3_.x * other.e4, self_.group3_.y * other.e4, self_.group3_.z * other.e4, 0.0)
    ));
}
fn multiVector_geometricProduct_plane(self_: MultiVector, other: Plane) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = plane_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.group4_.w * other_groups.group0_.w * -1.0, (self_.group1_.x * other_groups.group0_.x) + (self_.group1_.y * other_groups.group0_.y) + (self_.group1_.z * other_groups.group0_.z) + (self_.group1_.w * other_groups.group0_.w), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group3_.x * other_groups.group0_.w, self_.group3_.y * other_groups.group0_.w, self_.group3_.z * other_groups.group0_.w, -(self_.group0_.y * other_groups.group0_.w) - (self_.group3_.x * other_groups.group0_.x) - (self_.group3_.y * other_groups.group0_.y) - (self_.group3_.z * other_groups.group0_.z)), 
        /* e41, e42, e43 */ vec4<f32>((self_.group1_.y * other_groups.group0_.z) - (self_.group1_.z * other_groups.group0_.y), (self_.group1_.z * other_groups.group0_.x) - (self_.group1_.x * other_groups.group0_.z), (self_.group1_.x * other_groups.group0_.y) - (self_.group1_.y * other_groups.group0_.x), 0.0) + (vec4<f32>(vec3<f32>(other_groups.group0_.w), 0.0) * vec4<f32>(self_.group4_.x, self_.group4_.y, self_.group4_.z, 0.0)) - (vec4<f32>(vec3<f32>(self_.group4_.w), 0.0) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(other_groups.group0_.w), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0) * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.group0_.x * other_groups.group0_.x) + (self_.group3_.z * other_groups.group0_.y) - (self_.group2_.x * other_groups.group0_.w) - (self_.group3_.y * other_groups.group0_.z), (self_.group0_.x * other_groups.group0_.y) + (self_.group3_.x * other_groups.group0_.z) - (self_.group2_.y * other_groups.group0_.w) - (self_.group3_.z * other_groups.group0_.x), (self_.group0_.x * other_groups.group0_.z) + (self_.group3_.y * other_groups.group0_.x) - (self_.group2_.z * other_groups.group0_.w) - (self_.group3_.x * other_groups.group0_.y), self_.group0_.x * other_groups.group0_.w)
    ));
}
fn multiVector_geometricProduct_point(self_: MultiVector, other: Point) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = point_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((self_.group1_.x * other_groups.group0_.x) + (self_.group1_.y * other_groups.group0_.y) + (self_.group1_.z * other_groups.group0_.z), -(self_.group4_.x * other_groups.group0_.x) - (self_.group4_.y * other_groups.group0_.y) - (self_.group4_.z * other_groups.group0_.z) - (self_.group4_.w * other_groups.group0_.w), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group3_.y * other_groups.group0_.z) * -1.0, (self_.group3_.z * other_groups.group0_.x) * -1.0, (self_.group3_.x * other_groups.group0_.y) * -1.0, (self_.group2_.y * other_groups.group0_.y) + (self_.group2_.z * other_groups.group0_.z)) + (vec4<f32>(self_.group0_.x) * other_groups.group0_) + (vec4<f32>(self_.group3_.zxy.xyz, self_.group2_.x) * other_groups.group0_.yzxx), 
        /* e41, e42, e43 */ vec4<f32>((self_.group4_.z * other_groups.group0_.y) - (self_.group4_.y * other_groups.group0_.z), (self_.group4_.x * other_groups.group0_.z) - (self_.group4_.z * other_groups.group0_.x), (self_.group4_.y * other_groups.group0_.x) - (self_.group4_.x * other_groups.group0_.y), 0.0) + (vec4<f32>(vec3<f32>(self_.group1_.w), 0.0) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0)) - (vec4<f32>(vec3<f32>(other_groups.group0_.w), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((self_.group1_.y * other_groups.group0_.z) - (self_.group1_.z * other_groups.group0_.y), (self_.group1_.z * other_groups.group0_.x) - (self_.group1_.x * other_groups.group0_.z), (self_.group1_.x * other_groups.group0_.y) - (self_.group1_.y * other_groups.group0_.x), 0.0) - (vec4<f32>(vec3<f32>(self_.group4_.w), 0.0) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.group2_.y * other_groups.group0_.z) + (self_.group3_.x * other_groups.group0_.w), (self_.group2_.z * other_groups.group0_.x) + (self_.group3_.y * other_groups.group0_.w), (self_.group2_.x * other_groups.group0_.y) + (self_.group3_.z * other_groups.group0_.w), (self_.group3_.z * other_groups.group0_.z) * -1.0) - (vec4<f32>(self_.group0_.yy.xy, self_.group0_.y, self_.group3_.x) * other_groups.group0_.xyzx) - (vec4<f32>(self_.group2_.zxy.xyz, self_.group3_.y) * other_groups.group0_.yzxy)
    ));
}
fn multiVector_geometricProduct_scalar(self_: MultiVector, other: Scalar) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = scalar_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(other.scalar), 0.0, 0.0) * self_.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(other.scalar) * self_.group1_, 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(other.scalar), 0.0) * self_.group2_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(other.scalar), 0.0) * self_.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other.scalar) * self_.group4_
    ));
}
fn origin_geometricProduct_dualNum(self_: Origin, other: DualNum) -> Origin {
    let self_groups = origin_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(other_groups.group0_.x * self.e4, 0.0, 0.0, 0.0)
    ));
}
fn origin_geometricProduct_flector(self_: Origin, other: Flector) -> Motor {
    let self_groups = origin_grouped(self_);
    let other_groups = flector_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self.e4) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, other_groups.group1_.w), 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0)
    ));
}
fn origin_geometricProduct_horizon(self_: Origin, other: Horizon) -> AntiScalar {
    let self_groups = origin_grouped(self_);
    let other_groups = horizon_grouped(other);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(other.e321 * self.e4, 0.0, 0.0, 0.0)
    ));
}
fn origin_geometricProduct_line(self_: Origin, other: Line) -> Plane {
    let self_groups = origin_grouped(self_);
    let other_groups = line_grouped(other);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group1_.x * self.e4, other_groups.group1_.y * self.e4, other_groups.group1_.z * self.e4, 0.0)
    ));
}
fn origin_geometricProduct_motor(self_: Origin, other: Motor) -> Flector {
    let self_groups = origin_grouped(self_);
    let other_groups = motor_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group1_.w * self.e4), 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group1_.x * self.e4, other_groups.group1_.y * self.e4, other_groups.group1_.z * self.e4, 0.0)
    ));
}
fn origin_geometricProduct_multiVector(self_: Origin, other: MultiVector) -> MultiVector {
    let self_groups = origin_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, other_groups.group4_.w * self.e4, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group0_.x * self.e4), 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(self.e4), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group3_.x * self.e4, other_groups.group3_.y * self.e4, other_groups.group3_.z * self.e4, 0.0)
    ));
}
fn origin_geometricProduct_plane(self_: Origin, other: Plane) -> AntiScalar {
    let self_groups = origin_grouped(self_);
    let other_groups = plane_grouped(other);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(other_groups.group0_.w * self.e4, 0.0, 0.0, 0.0)
    ));
}
fn origin_geometricProduct_point(self_: Origin, other: Point) -> Line {
    let self_groups = origin_grouped(self_);
    let other_groups = point_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(self.e4), 0.0) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0)
    ));
}
fn origin_geometricProduct_scalar(self_: Origin, other: Scalar) -> Origin {
    let self_groups = origin_grouped(self_);
    let other_groups = scalar_grouped(other);
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(self.e4 * other.scalar, 0.0, 0.0, 0.0)
    ));
}
fn plane_geometricProduct_antiScalar(self_: Plane, other: AntiScalar) -> Origin {
    let self_groups = plane_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(self_.group0_.w * other.e1234, 0.0, 0.0, 0.0)
    ));
}
fn plane_geometricProduct_dualNum(self_: Plane, other: DualNum) -> Flector {
    let self_groups = plane_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group0_.y * self_.group0_.w), 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group0_.x) * self_.group0_
    ));
}
fn plane_geometricProduct_flector(self_: Plane, other: Flector) -> Motor {
    let self_groups = plane_grouped(self_);
    let other_groups = flector_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other_groups.group0_.y * self_.group0_.z) + (other_groups.group1_.w * self_.group0_.x), (other_groups.group0_.z * self_.group0_.x) + (other_groups.group1_.w * self_.group0_.y), (other_groups.group0_.x * self_.group0_.y) + (other_groups.group1_.w * self_.group0_.z), -(other_groups.group0_.z * self_.group0_.z) - (other_groups.group0_.w * self_.group0_.w)) - (vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, other_groups.group0_.y) * self_.group0_.wwwy) - (other_groups.group0_.zxyx * self_.group0_.yzxx), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.group0_.w) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, other_groups.group1_.w) * vec4<f32>(-1.0)
    ));
}
fn plane_geometricProduct_horizon(self_: Plane, other: Horizon) -> Motor {
    let self_groups = plane_grouped(self_);
    let other_groups = horizon_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.group0_.x * other.e321, self_.group0_.y * other.e321, self_.group0_.z * other.e321, 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, self_.group0_.w * other.e321 * -1.0)
    ));
}
fn plane_geometricProduct_line(self_: Plane, other: Line) -> Flector {
    let self_groups = plane_grouped(self_);
    let other_groups = line_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group1_.x * self_.group0_.w, other_groups.group1_.y * self_.group0_.w, other_groups.group1_.z * self_.group0_.w, -(other_groups.group1_.x * self_.group0_.x) - (other_groups.group1_.y * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.z)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.x * self_.group0_.w) + (other_groups.group1_.y * self_.group0_.z) - (other_groups.group1_.z * self_.group0_.y), (other_groups.group0_.y * self_.group0_.w) + (other_groups.group1_.z * self_.group0_.x) - (other_groups.group1_.x * self_.group0_.z), (other_groups.group0_.z * self_.group0_.w) + (other_groups.group1_.x * self_.group0_.y) - (other_groups.group1_.y * self_.group0_.x), 0.0)
    ));
}
fn plane_geometricProduct_motor(self_: Plane, other: Motor) -> Flector {
    let self_groups = plane_grouped(self_);
    let other_groups = motor_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group1_.x * self_.group0_.w, other_groups.group1_.y * self_.group0_.w, other_groups.group1_.z * self_.group0_.w, (other_groups.group0_.w * self_.group0_.w) - (other_groups.group1_.x * self_.group0_.x) - (other_groups.group1_.y * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.z)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.x * self_.group0_.w) + (other_groups.group1_.y * self_.group0_.z) + (other_groups.group1_.w * self_.group0_.x) - (other_groups.group1_.z * self_.group0_.y), (other_groups.group0_.y * self_.group0_.w) + (other_groups.group1_.z * self_.group0_.x) + (other_groups.group1_.w * self_.group0_.y) - (other_groups.group1_.x * self_.group0_.z), (other_groups.group0_.z * self_.group0_.w) + (other_groups.group1_.x * self_.group0_.y) + (other_groups.group1_.w * self_.group0_.z) - (other_groups.group1_.y * self_.group0_.x), other_groups.group1_.w * self_.group0_.w)
    ));
}
fn plane_geometricProduct_multiVector(self_: Plane, other: MultiVector) -> MultiVector {
    let self_groups = plane_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other_groups.group4_.w * self_.group0_.w * -1.0, -(other_groups.group1_.x * self_.group0_.x) - (other_groups.group1_.y * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.z) - (other_groups.group1_.w * self_.group0_.w), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group3_.x * self_.group0_.w, other_groups.group3_.y * self_.group0_.w, other_groups.group3_.z * self_.group0_.w, (other_groups.group0_.y * self_.group0_.w) - (other_groups.group3_.x * self_.group0_.x) - (other_groups.group3_.y * self_.group0_.y) - (other_groups.group3_.z * self_.group0_.z)), 
        /* e41, e42, e43 */ vec4<f32>((other_groups.group1_.y * self_.group0_.z) - (other_groups.group1_.z * self_.group0_.y), (other_groups.group1_.z * self_.group0_.x) - (other_groups.group1_.x * self_.group0_.z), (other_groups.group1_.x * self_.group0_.y) - (other_groups.group1_.y * self_.group0_.x), 0.0) + (vec4<f32>(vec3<f32>(other_groups.group4_.w), 0.0) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0)) - (vec4<f32>(vec3<f32>(self_.group0_.w), 0.0) * vec4<f32>(other_groups.group4_.x, other_groups.group4_.y, other_groups.group4_.z, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(self_.group0_.w), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0) * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.x * self_.group0_.x) + (other_groups.group2_.x * self_.group0_.w) + (other_groups.group3_.y * self_.group0_.z) - (other_groups.group3_.z * self_.group0_.y), (other_groups.group0_.x * self_.group0_.y) + (other_groups.group2_.y * self_.group0_.w) + (other_groups.group3_.z * self_.group0_.x) - (other_groups.group3_.x * self_.group0_.z), (other_groups.group0_.x * self_.group0_.z) + (other_groups.group2_.z * self_.group0_.w) + (other_groups.group3_.x * self_.group0_.y) - (other_groups.group3_.y * self_.group0_.x), other_groups.group0_.x * self_.group0_.w)
    ));
}
fn plane_geometricProduct_origin(self_: Plane, other: Origin) -> AntiScalar {
    let self_groups = plane_grouped(self_);
    let other_groups = origin_grouped(other);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(self_.group0_.w * other.e4 * -1.0, 0.0, 0.0, 0.0)
    ));
}
fn plane_geometricProduct_plane(self_: Plane, other: Plane) -> Motor {
    let self_groups = plane_grouped(self_);
    let other_groups = plane_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other_groups.group0_.w * self_.group0_.x) - (other_groups.group0_.x * self_.group0_.w), (other_groups.group0_.w * self_.group0_.y) - (other_groups.group0_.y * self_.group0_.w), (other_groups.group0_.w * self_.group0_.z) - (other_groups.group0_.z * self_.group0_.w), 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group0_.w * self_.group0_.w * -1.0)
    ));
}
fn plane_geometricProduct_point(self_: Plane, other: Point) -> Motor {
    let self_groups = plane_grouped(self_);
    let other_groups = point_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.group0_.z * other_groups.group0_.y, self_.group0_.x * other_groups.group0_.z, self_.group0_.y * other_groups.group0_.x, -(self_.group0_.y * other_groups.group0_.y) - (self_.group0_.z * other_groups.group0_.z) - (self_.group0_.w * other_groups.group0_.w)) - (self_.group0_.yzxx * other_groups.group0_.zxyx), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.group0_.w * other_groups.group0_.x * -1.0, self_.group0_.w * other_groups.group0_.y * -1.0, self_.group0_.w * other_groups.group0_.z * -1.0, 0.0)
    ));
}
fn plane_geometricProduct_scalar(self_: Plane, other: Scalar) -> Plane {
    let self_groups = plane_grouped(self_);
    let other_groups = scalar_grouped(other);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(other.scalar) * self_.group0_
    ));
}
fn point_geometricProduct_antiScalar(self_: Point, other: AntiScalar) -> Plane {
    let self_groups = point_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.x * other.e1234, self_.group0_.y * other.e1234, self_.group0_.z * other.e1234, 0.0)
    ));
}
fn point_geometricProduct_dualNum(self_: Point, other: DualNum) -> Flector {
    let self_groups = point_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group0_.x) * self_.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group0_.y * self_.group0_.x, other_groups.group0_.y * self_.group0_.y, other_groups.group0_.y * self_.group0_.z, 0.0)
    ));
}
fn point_geometricProduct_flector(self_: Point, other: Flector) -> Motor {
    let self_groups = point_grouped(self_);
    let other_groups = flector_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(-(other_groups.group0_.w * self_.group0_.x) - (other_groups.group1_.y * self_.group0_.z), -(other_groups.group0_.w * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.x), -(other_groups.group0_.w * self_.group0_.z) - (other_groups.group1_.x * self_.group0_.y), (other_groups.group1_.z * self_.group0_.z) + (other_groups.group1_.w * self_.group0_.w)) + (vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, other_groups.group1_.x) * self_.group0_.wwwx) + (other_groups.group1_.zxyy * self_.group0_.yzxy), 
        /* e23, e31, e12, scalar */ vec4<f32>(-(other_groups.group0_.y * self_.group0_.z) - (other_groups.group1_.w * self_.group0_.x), -(other_groups.group0_.z * self_.group0_.x) - (other_groups.group1_.w * self_.group0_.y), -(other_groups.group0_.x * self_.group0_.y) - (other_groups.group1_.w * self_.group0_.z), (other_groups.group0_.y * self_.group0_.y) + (other_groups.group0_.z * self_.group0_.z)) + (other_groups.group0_.zxyx * self_.group0_.yzxx)
    ));
}
fn point_geometricProduct_horizon(self_: Point, other: Horizon) -> Motor {
    let self_groups = point_grouped(self_);
    let other_groups = horizon_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, self_.group0_.w * other.e321), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.group0_.x * other.e321 * -1.0, self_.group0_.y * other.e321 * -1.0, self_.group0_.z * other.e321 * -1.0, 0.0)
    ));
}
fn point_geometricProduct_line(self_: Point, other: Line) -> Flector {
    let self_groups = point_grouped(self_);
    let other_groups = line_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group1_.y * self_.group0_.z, other_groups.group1_.z * self_.group0_.x, other_groups.group1_.x * self_.group0_.y, -(other_groups.group0_.y * self_.group0_.y) - (other_groups.group0_.z * self_.group0_.z)) - (vec4<f32>(other_groups.group1_.zxy.xyz, other_groups.group0_.x) * self_.group0_.yzxx), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.y * self_.group0_.z) + (other_groups.group1_.x * self_.group0_.w), (other_groups.group0_.z * self_.group0_.x) + (other_groups.group1_.y * self_.group0_.w), (other_groups.group0_.x * self_.group0_.y) + (other_groups.group1_.z * self_.group0_.w), -(other_groups.group1_.y * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.z)) - (vec4<f32>(other_groups.group0_.zxy.xyz, other_groups.group1_.x) * self_.group0_.yzxx)
    ));
}
fn point_geometricProduct_motor(self_: Point, other: Motor) -> Flector {
    let self_groups = point_grouped(self_);
    let other_groups = motor_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group1_.w * self_.group0_.x, other_groups.group1_.w * self_.group0_.y, other_groups.group1_.w * self_.group0_.z, -(other_groups.group0_.y * self_.group0_.y) - (other_groups.group0_.z * self_.group0_.z)) + (other_groups.group1_.yzxw * self_.group0_.zxyw) - (vec4<f32>(other_groups.group1_.z, other_groups.group1_.x, other_groups.group1_.y, other_groups.group0_.x) * self_.group0_.yzxx), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.y * self_.group0_.z) + (other_groups.group0_.w * self_.group0_.x) + (other_groups.group1_.x * self_.group0_.w), (other_groups.group0_.z * self_.group0_.x) + (other_groups.group0_.w * self_.group0_.y) + (other_groups.group1_.y * self_.group0_.w), (other_groups.group0_.x * self_.group0_.y) + (other_groups.group0_.w * self_.group0_.z) + (other_groups.group1_.z * self_.group0_.w), -(other_groups.group1_.y * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.z)) - (vec4<f32>(other_groups.group0_.z, other_groups.group0_.x, other_groups.group0_.y, other_groups.group1_.x) * self_.group0_.yzxx)
    ));
}
fn point_geometricProduct_multiVector(self_: Point, other: MultiVector) -> MultiVector {
    let self_groups = point_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, other_groups.group4_.w * self_.group0_.w, 0.0, 0.0) + (vec4<f32>(vec2<f32>(self_.group0_.x), 0.0, 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group4_.x, 0.0, 0.0)) + (vec4<f32>(vec2<f32>(self_.group0_.y), 0.0, 0.0) * vec4<f32>(other_groups.group1_.y, other_groups.group4_.y, 0.0, 0.0)) + (vec4<f32>(vec2<f32>(self_.group0_.z), 0.0, 0.0) * vec4<f32>(other_groups.group1_.z, other_groups.group4_.z, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group3_.y * self_.group0_.z, other_groups.group3_.z * self_.group0_.x, other_groups.group3_.x * self_.group0_.y, -(other_groups.group2_.y * self_.group0_.y) - (other_groups.group2_.z * self_.group0_.z)) + (vec4<f32>(other_groups.group0_.x) * self_.group0_) - (vec4<f32>(other_groups.group3_.zxy.xyz, other_groups.group2_.x) * self_.group0_.yzxx), 
        /* e41, e42, e43 */ vec4<f32>((other_groups.group4_.z * self_.group0_.y) - (other_groups.group4_.y * self_.group0_.z), (other_groups.group4_.x * self_.group0_.z) - (other_groups.group4_.z * self_.group0_.x), (other_groups.group4_.y * self_.group0_.x) - (other_groups.group4_.x * self_.group0_.y), 0.0) + (vec4<f32>(vec3<f32>(self_.group0_.w), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0)) - (vec4<f32>(vec3<f32>(other_groups.group1_.w), 0.0) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((other_groups.group1_.z * self_.group0_.y) - (other_groups.group1_.y * self_.group0_.z), (other_groups.group1_.x * self_.group0_.z) - (other_groups.group1_.z * self_.group0_.x), (other_groups.group1_.y * self_.group0_.x) - (other_groups.group1_.x * self_.group0_.y), 0.0) - (vec4<f32>(vec3<f32>(other_groups.group4_.w), 0.0) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.y * self_.group0_.x) + (other_groups.group2_.y * self_.group0_.z) + (other_groups.group3_.x * self_.group0_.w), (other_groups.group0_.y * self_.group0_.y) + (other_groups.group2_.z * self_.group0_.x) + (other_groups.group3_.y * self_.group0_.w), (other_groups.group0_.y * self_.group0_.z) + (other_groups.group2_.x * self_.group0_.y) + (other_groups.group3_.z * self_.group0_.w), -(other_groups.group3_.y * self_.group0_.y) - (other_groups.group3_.z * self_.group0_.z)) - (vec4<f32>(other_groups.group2_.zxy.xyz, other_groups.group3_.x) * self_.group0_.yzxx)
    ));
}
fn point_geometricProduct_origin(self_: Point, other: Origin) -> Line {
    let self_groups = point_grouped(self_);
    let other_groups = origin_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(other.e4), 0.0) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0)
    ));
}
fn point_geometricProduct_plane(self_: Point, other: Plane) -> Motor {
    let self_groups = point_grouped(self_);
    let other_groups = plane_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other_groups.group0_.y * self_.group0_.z) * -1.0, (other_groups.group0_.z * self_.group0_.x) * -1.0, (other_groups.group0_.x * self_.group0_.y) * -1.0, (other_groups.group0_.y * self_.group0_.y) + (other_groups.group0_.z * self_.group0_.z) + (other_groups.group0_.w * self_.group0_.w)) + (other_groups.group0_.zxyx * self_.group0_.yzxx), 
        /* e23, e31, e12, scalar */ vec4<f32>(other_groups.group0_.w * self_.group0_.x * -1.0, other_groups.group0_.w * self_.group0_.y * -1.0, other_groups.group0_.w * self_.group0_.z * -1.0, 0.0)
    ));
}
fn point_geometricProduct_point(self_: Point, other: Point) -> Motor {
    let self_groups = point_grouped(self_);
    let other_groups = point_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other_groups.group0_.x * self_.group0_.w) - (other_groups.group0_.w * self_.group0_.x), (other_groups.group0_.y * self_.group0_.w) - (other_groups.group0_.w * self_.group0_.y), (other_groups.group0_.z * self_.group0_.w) - (other_groups.group0_.w * self_.group0_.z), 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>((other_groups.group0_.y * self_.group0_.z) * -1.0, (other_groups.group0_.z * self_.group0_.x) * -1.0, (other_groups.group0_.x * self_.group0_.y) * -1.0, (other_groups.group0_.y * self_.group0_.y) + (other_groups.group0_.z * self_.group0_.z)) + (other_groups.group0_.zxyx * self_.group0_.yzxx)
    ));
}
fn point_geometricProduct_scalar(self_: Point, other: Scalar) -> Point {
    let self_groups = point_grouped(self_);
    let other_groups = scalar_grouped(other);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.scalar) * self_.group0_
    ));
}
fn scalar_geometricProduct_antiScalar(self_: Scalar, other: AntiScalar) -> AntiScalar {
    let self_groups = scalar_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(other.e1234 * self.scalar, 0.0, 0.0, 0.0)
    ));
}
fn scalar_geometricProduct_dualNum(self_: Scalar, other: DualNum) -> DualNum {
    let self_groups = scalar_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(self.scalar), 0.0, 0.0) * other_groups.group0_
    ));
}
fn scalar_geometricProduct_flector(self_: Scalar, other: Flector) -> Flector {
    let self_groups = scalar_grouped(self_);
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self.scalar) * other_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self.scalar) * other_groups.group1_
    ));
}
fn scalar_geometricProduct_horizon(self_: Scalar, other: Horizon) -> Horizon {
    let self_groups = scalar_grouped(self_);
    let other_groups = horizon_grouped(other);
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(other.e321 * self.scalar, 0.0, 0.0, 0.0)
    ));
}
fn scalar_geometricProduct_line(self_: Scalar, other: Line) -> Line {
    let self_groups = scalar_grouped(self_);
    let other_groups = line_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(self.scalar), 0.0) * other_groups.group0_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(self.scalar), 0.0) * other_groups.group1_
    ));
}
fn scalar_geometricProduct_motor(self_: Scalar, other: Motor) -> Motor {
    let self_groups = scalar_grouped(self_);
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self.scalar) * other_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(self.scalar) * other_groups.group1_
    ));
}
fn scalar_geometricProduct_multiVector(self_: Scalar, other: MultiVector) -> MultiVector {
    let self_groups = scalar_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(self.scalar), 0.0, 0.0) * other_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(self.scalar) * other_groups.group1_, 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(self.scalar), 0.0) * other_groups.group2_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(self.scalar), 0.0) * other_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self.scalar) * other_groups.group4_
    ));
}
fn scalar_geometricProduct_origin(self_: Scalar, other: Origin) -> Origin {
    let self_groups = scalar_grouped(self_);
    let other_groups = origin_grouped(other);
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(other.e4 * self.scalar, 0.0, 0.0, 0.0)
    ));
}
fn scalar_geometricProduct_plane(self_: Scalar, other: Plane) -> Plane {
    let self_groups = scalar_grouped(self_);
    let other_groups = plane_grouped(other);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(self.scalar) * other_groups.group0_
    ));
}
fn scalar_geometricProduct_point(self_: Scalar, other: Point) -> Point {
    let self_groups = scalar_grouped(self_);
    let other_groups = point_grouped(other);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self.scalar) * other_groups.group0_
    ));
}
fn scalar_geometricProduct_scalar(self_: Scalar, other: Scalar) -> Scalar {
    let self_groups = scalar_grouped(self_);
    let other_groups = scalar_grouped(other);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(other.scalar * self.scalar, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_geometricQuotient_dualNum(self_: AntiScalar, other: DualNum) -> AntiScalar {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(self.e1234 * inverse.scalar, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_geometricQuotient_flector(self_: AntiScalar, other: Flector) -> AntiScalar {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = flector_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group0_.y, 2) + pow(other_groups.group0_.z, 2) - pow(other_groups.group1_.w, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(self.e1234 * inverse.scalar, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_geometricQuotient_horizon(self_: AntiScalar, other: Horizon) -> AntiScalar {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = horizon_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other.e321, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(self.e1234 * inverse.scalar, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_geometricQuotient_line(self_: AntiScalar, other: Line) -> AntiScalar {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = line_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(-pow(other_groups.group1_.x, 2) - pow(other_groups.group1_.y, 2) - pow(other_groups.group1_.z, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(self.e1234 * inverse.scalar, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_geometricQuotient_motor(self_: AntiScalar, other: Motor) -> AntiScalar {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = motor_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group1_.w, 2) - pow(other_groups.group1_.x, 2) - pow(other_groups.group1_.y, 2) - pow(other_groups.group1_.z, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(self.e1234 * inverse.scalar, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_geometricQuotient_multiVector(self_: AntiScalar, other: MultiVector) -> AntiScalar {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group1_.x, 2) + pow(other_groups.group1_.y, 2) + pow(other_groups.group1_.z, 2) - pow(other_groups.group3_.x, 2) - pow(other_groups.group3_.y, 2) - pow(other_groups.group3_.z, 2) - pow(other_groups.group4_.w, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(self.e1234 * inverse.scalar, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_geometricQuotient_plane(self_: AntiScalar, other: Plane) -> AntiScalar {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = plane_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.w, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(self.e1234 * inverse.scalar, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_geometricQuotient_point(self_: AntiScalar, other: Point) -> AntiScalar {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = point_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group0_.y, 2) + pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(self.e1234 * inverse.scalar, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_geometricQuotient_scalar(self_: AntiScalar, other: Scalar) -> AntiScalar {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = scalar_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other.scalar, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(self.e1234 * inverse.scalar, 0.0, 0.0, 0.0)
    ));
}
fn dualNum_geometricQuotient_dualNum(self_: DualNum, other: DualNum) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(inverse.scalar), 0.0, 0.0) * self_.group0_
    ));
}
fn dualNum_geometricQuotient_flector(self_: DualNum, other: Flector) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let other_groups = flector_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group0_.y, 2) + pow(other_groups.group0_.z, 2) - pow(other_groups.group1_.w, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(inverse.scalar), 0.0, 0.0) * self_.group0_
    ));
}
fn dualNum_geometricQuotient_horizon(self_: DualNum, other: Horizon) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let other_groups = horizon_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other.e321, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(inverse.scalar), 0.0, 0.0) * self_.group0_
    ));
}
fn dualNum_geometricQuotient_line(self_: DualNum, other: Line) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let other_groups = line_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(-pow(other_groups.group1_.x, 2) - pow(other_groups.group1_.y, 2) - pow(other_groups.group1_.z, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(inverse.scalar), 0.0, 0.0) * self_.group0_
    ));
}
fn dualNum_geometricQuotient_motor(self_: DualNum, other: Motor) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let other_groups = motor_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group1_.w, 2) - pow(other_groups.group1_.x, 2) - pow(other_groups.group1_.y, 2) - pow(other_groups.group1_.z, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(inverse.scalar), 0.0, 0.0) * self_.group0_
    ));
}
fn dualNum_geometricQuotient_multiVector(self_: DualNum, other: MultiVector) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group1_.x, 2) + pow(other_groups.group1_.y, 2) + pow(other_groups.group1_.z, 2) - pow(other_groups.group3_.x, 2) - pow(other_groups.group3_.y, 2) - pow(other_groups.group3_.z, 2) - pow(other_groups.group4_.w, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(inverse.scalar), 0.0, 0.0) * self_.group0_
    ));
}
fn dualNum_geometricQuotient_plane(self_: DualNum, other: Plane) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let other_groups = plane_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.w, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(inverse.scalar), 0.0, 0.0) * self_.group0_
    ));
}
fn dualNum_geometricQuotient_point(self_: DualNum, other: Point) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let other_groups = point_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group0_.y, 2) + pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(inverse.scalar), 0.0, 0.0) * self_.group0_
    ));
}
fn dualNum_geometricQuotient_scalar(self_: DualNum, other: Scalar) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let other_groups = scalar_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other.scalar, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(inverse.scalar), 0.0, 0.0) * self_.group0_
    ));
}
fn flector_geometricQuotient_dualNum(self_: Flector, other: DualNum) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_.group1_
    ));
}
fn flector_geometricQuotient_flector(self_: Flector, other: Flector) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = flector_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group0_.y, 2) + pow(other_groups.group0_.z, 2) - pow(other_groups.group1_.w, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_.group1_
    ));
}
fn flector_geometricQuotient_horizon(self_: Flector, other: Horizon) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = horizon_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other.e321, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_.group1_
    ));
}
fn flector_geometricQuotient_line(self_: Flector, other: Line) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = line_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(-pow(other_groups.group1_.x, 2) - pow(other_groups.group1_.y, 2) - pow(other_groups.group1_.z, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_.group1_
    ));
}
fn flector_geometricQuotient_motor(self_: Flector, other: Motor) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = motor_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group1_.w, 2) - pow(other_groups.group1_.x, 2) - pow(other_groups.group1_.y, 2) - pow(other_groups.group1_.z, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_.group1_
    ));
}
fn flector_geometricQuotient_multiVector(self_: Flector, other: MultiVector) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group1_.x, 2) + pow(other_groups.group1_.y, 2) + pow(other_groups.group1_.z, 2) - pow(other_groups.group3_.x, 2) - pow(other_groups.group3_.y, 2) - pow(other_groups.group3_.z, 2) - pow(other_groups.group4_.w, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_.group1_
    ));
}
fn flector_geometricQuotient_plane(self_: Flector, other: Plane) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = plane_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.w, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_.group1_
    ));
}
fn flector_geometricQuotient_point(self_: Flector, other: Point) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = point_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group0_.y, 2) + pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_.group1_
    ));
}
fn flector_geometricQuotient_scalar(self_: Flector, other: Scalar) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = scalar_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other.scalar, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_.group1_
    ));
}
fn horizon_geometricQuotient_dualNum(self_: Horizon, other: DualNum) -> Horizon {
    let self_groups = horizon_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(self.e321 * inverse.scalar, 0.0, 0.0, 0.0)
    ));
}
fn horizon_geometricQuotient_flector(self_: Horizon, other: Flector) -> Horizon {
    let self_groups = horizon_grouped(self_);
    let other_groups = flector_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group0_.y, 2) + pow(other_groups.group0_.z, 2) - pow(other_groups.group1_.w, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(self.e321 * inverse.scalar, 0.0, 0.0, 0.0)
    ));
}
fn horizon_geometricQuotient_horizon(self_: Horizon, other: Horizon) -> Horizon {
    let self_groups = horizon_grouped(self_);
    let other_groups = horizon_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other.e321, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(self.e321 * inverse.scalar, 0.0, 0.0, 0.0)
    ));
}
fn horizon_geometricQuotient_line(self_: Horizon, other: Line) -> Horizon {
    let self_groups = horizon_grouped(self_);
    let other_groups = line_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(-pow(other_groups.group1_.x, 2) - pow(other_groups.group1_.y, 2) - pow(other_groups.group1_.z, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(self.e321 * inverse.scalar, 0.0, 0.0, 0.0)
    ));
}
fn horizon_geometricQuotient_motor(self_: Horizon, other: Motor) -> Horizon {
    let self_groups = horizon_grouped(self_);
    let other_groups = motor_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group1_.w, 2) - pow(other_groups.group1_.x, 2) - pow(other_groups.group1_.y, 2) - pow(other_groups.group1_.z, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(self.e321 * inverse.scalar, 0.0, 0.0, 0.0)
    ));
}
fn horizon_geometricQuotient_multiVector(self_: Horizon, other: MultiVector) -> Horizon {
    let self_groups = horizon_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group1_.x, 2) + pow(other_groups.group1_.y, 2) + pow(other_groups.group1_.z, 2) - pow(other_groups.group3_.x, 2) - pow(other_groups.group3_.y, 2) - pow(other_groups.group3_.z, 2) - pow(other_groups.group4_.w, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(self.e321 * inverse.scalar, 0.0, 0.0, 0.0)
    ));
}
fn horizon_geometricQuotient_plane(self_: Horizon, other: Plane) -> Horizon {
    let self_groups = horizon_grouped(self_);
    let other_groups = plane_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.w, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(self.e321 * inverse.scalar, 0.0, 0.0, 0.0)
    ));
}
fn horizon_geometricQuotient_point(self_: Horizon, other: Point) -> Horizon {
    let self_groups = horizon_grouped(self_);
    let other_groups = point_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group0_.y, 2) + pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(self.e321 * inverse.scalar, 0.0, 0.0, 0.0)
    ));
}
fn horizon_geometricQuotient_scalar(self_: Horizon, other: Scalar) -> Horizon {
    let self_groups = horizon_grouped(self_);
    let other_groups = scalar_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other.scalar, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(self.e321 * inverse.scalar, 0.0, 0.0, 0.0)
    ));
}
fn line_geometricQuotient_dualNum(self_: Line, other: DualNum) -> Line {
    let self_groups = line_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(inverse.scalar), 0.0) * self_.group0_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(inverse.scalar), 0.0) * self_.group1_
    ));
}
fn line_geometricQuotient_flector(self_: Line, other: Flector) -> Line {
    let self_groups = line_grouped(self_);
    let other_groups = flector_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group0_.y, 2) + pow(other_groups.group0_.z, 2) - pow(other_groups.group1_.w, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(inverse.scalar), 0.0) * self_.group0_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(inverse.scalar), 0.0) * self_.group1_
    ));
}
fn line_geometricQuotient_horizon(self_: Line, other: Horizon) -> Line {
    let self_groups = line_grouped(self_);
    let other_groups = horizon_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other.e321, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(inverse.scalar), 0.0) * self_.group0_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(inverse.scalar), 0.0) * self_.group1_
    ));
}
fn line_geometricQuotient_line(self_: Line, other: Line) -> Line {
    let self_groups = line_grouped(self_);
    let other_groups = line_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(-pow(other_groups.group1_.x, 2) - pow(other_groups.group1_.y, 2) - pow(other_groups.group1_.z, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(inverse.scalar), 0.0) * self_.group0_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(inverse.scalar), 0.0) * self_.group1_
    ));
}
fn line_geometricQuotient_motor(self_: Line, other: Motor) -> Line {
    let self_groups = line_grouped(self_);
    let other_groups = motor_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group1_.w, 2) - pow(other_groups.group1_.x, 2) - pow(other_groups.group1_.y, 2) - pow(other_groups.group1_.z, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(inverse.scalar), 0.0) * self_.group0_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(inverse.scalar), 0.0) * self_.group1_
    ));
}
fn line_geometricQuotient_multiVector(self_: Line, other: MultiVector) -> Line {
    let self_groups = line_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group1_.x, 2) + pow(other_groups.group1_.y, 2) + pow(other_groups.group1_.z, 2) - pow(other_groups.group3_.x, 2) - pow(other_groups.group3_.y, 2) - pow(other_groups.group3_.z, 2) - pow(other_groups.group4_.w, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(inverse.scalar), 0.0) * self_.group0_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(inverse.scalar), 0.0) * self_.group1_
    ));
}
fn line_geometricQuotient_plane(self_: Line, other: Plane) -> Line {
    let self_groups = line_grouped(self_);
    let other_groups = plane_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.w, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(inverse.scalar), 0.0) * self_.group0_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(inverse.scalar), 0.0) * self_.group1_
    ));
}
fn line_geometricQuotient_point(self_: Line, other: Point) -> Line {
    let self_groups = line_grouped(self_);
    let other_groups = point_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group0_.y, 2) + pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(inverse.scalar), 0.0) * self_.group0_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(inverse.scalar), 0.0) * self_.group1_
    ));
}
fn line_geometricQuotient_scalar(self_: Line, other: Scalar) -> Line {
    let self_groups = line_grouped(self_);
    let other_groups = scalar_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other.scalar, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(inverse.scalar), 0.0) * self_.group0_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(inverse.scalar), 0.0) * self_.group1_
    ));
}
fn motor_geometricQuotient_dualNum(self_: Motor, other: DualNum) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(inverse.scalar) * self_.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(inverse.scalar) * self_.group1_
    ));
}
fn motor_geometricQuotient_flector(self_: Motor, other: Flector) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = flector_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group0_.y, 2) + pow(other_groups.group0_.z, 2) - pow(other_groups.group1_.w, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(inverse.scalar) * self_.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(inverse.scalar) * self_.group1_
    ));
}
fn motor_geometricQuotient_horizon(self_: Motor, other: Horizon) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = horizon_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other.e321, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(inverse.scalar) * self_.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(inverse.scalar) * self_.group1_
    ));
}
fn motor_geometricQuotient_line(self_: Motor, other: Line) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = line_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(-pow(other_groups.group1_.x, 2) - pow(other_groups.group1_.y, 2) - pow(other_groups.group1_.z, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(inverse.scalar) * self_.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(inverse.scalar) * self_.group1_
    ));
}
fn motor_geometricQuotient_motor(self_: Motor, other: Motor) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = motor_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group1_.w, 2) - pow(other_groups.group1_.x, 2) - pow(other_groups.group1_.y, 2) - pow(other_groups.group1_.z, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(inverse.scalar) * self_.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(inverse.scalar) * self_.group1_
    ));
}
fn motor_geometricQuotient_multiVector(self_: Motor, other: MultiVector) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group1_.x, 2) + pow(other_groups.group1_.y, 2) + pow(other_groups.group1_.z, 2) - pow(other_groups.group3_.x, 2) - pow(other_groups.group3_.y, 2) - pow(other_groups.group3_.z, 2) - pow(other_groups.group4_.w, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(inverse.scalar) * self_.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(inverse.scalar) * self_.group1_
    ));
}
fn motor_geometricQuotient_plane(self_: Motor, other: Plane) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = plane_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.w, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(inverse.scalar) * self_.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(inverse.scalar) * self_.group1_
    ));
}
fn motor_geometricQuotient_point(self_: Motor, other: Point) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = point_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group0_.y, 2) + pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(inverse.scalar) * self_.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(inverse.scalar) * self_.group1_
    ));
}
fn motor_geometricQuotient_scalar(self_: Motor, other: Scalar) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = scalar_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other.scalar, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(inverse.scalar) * self_.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(inverse.scalar) * self_.group1_
    ));
}
fn multiVector_geometricQuotient_dualNum(self_: MultiVector, other: DualNum) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(inverse.scalar), 0.0, 0.0) * self_.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_.group1_, 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(inverse.scalar), 0.0) * self_.group2_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(inverse.scalar), 0.0) * self_.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_.group4_
    ));
}
fn multiVector_geometricQuotient_flector(self_: MultiVector, other: Flector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = flector_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group0_.y, 2) + pow(other_groups.group0_.z, 2) - pow(other_groups.group1_.w, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(inverse.scalar), 0.0, 0.0) * self_.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_.group1_, 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(inverse.scalar), 0.0) * self_.group2_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(inverse.scalar), 0.0) * self_.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_.group4_
    ));
}
fn multiVector_geometricQuotient_horizon(self_: MultiVector, other: Horizon) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = horizon_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other.e321, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(inverse.scalar), 0.0, 0.0) * self_.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_.group1_, 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(inverse.scalar), 0.0) * self_.group2_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(inverse.scalar), 0.0) * self_.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_.group4_
    ));
}
fn multiVector_geometricQuotient_line(self_: MultiVector, other: Line) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = line_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(-pow(other_groups.group1_.x, 2) - pow(other_groups.group1_.y, 2) - pow(other_groups.group1_.z, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(inverse.scalar), 0.0, 0.0) * self_.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_.group1_, 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(inverse.scalar), 0.0) * self_.group2_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(inverse.scalar), 0.0) * self_.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_.group4_
    ));
}
fn multiVector_geometricQuotient_motor(self_: MultiVector, other: Motor) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = motor_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group1_.w, 2) - pow(other_groups.group1_.x, 2) - pow(other_groups.group1_.y, 2) - pow(other_groups.group1_.z, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(inverse.scalar), 0.0, 0.0) * self_.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_.group1_, 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(inverse.scalar), 0.0) * self_.group2_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(inverse.scalar), 0.0) * self_.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_.group4_
    ));
}
fn multiVector_geometricQuotient_multiVector(self_: MultiVector, other: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group1_.x, 2) + pow(other_groups.group1_.y, 2) + pow(other_groups.group1_.z, 2) - pow(other_groups.group3_.x, 2) - pow(other_groups.group3_.y, 2) - pow(other_groups.group3_.z, 2) - pow(other_groups.group4_.w, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(inverse.scalar), 0.0, 0.0) * self_.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_.group1_, 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(inverse.scalar), 0.0) * self_.group2_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(inverse.scalar), 0.0) * self_.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_.group4_
    ));
}
fn multiVector_geometricQuotient_plane(self_: MultiVector, other: Plane) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = plane_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.w, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(inverse.scalar), 0.0, 0.0) * self_.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_.group1_, 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(inverse.scalar), 0.0) * self_.group2_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(inverse.scalar), 0.0) * self_.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_.group4_
    ));
}
fn multiVector_geometricQuotient_point(self_: MultiVector, other: Point) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = point_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group0_.y, 2) + pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(inverse.scalar), 0.0, 0.0) * self_.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_.group1_, 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(inverse.scalar), 0.0) * self_.group2_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(inverse.scalar), 0.0) * self_.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_.group4_
    ));
}
fn multiVector_geometricQuotient_scalar(self_: MultiVector, other: Scalar) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = scalar_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other.scalar, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(inverse.scalar), 0.0, 0.0) * self_.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_.group1_, 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(inverse.scalar), 0.0) * self_.group2_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(inverse.scalar), 0.0) * self_.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_.group4_
    ));
}
fn origin_geometricQuotient_dualNum(self_: Origin, other: DualNum) -> Origin {
    let self_groups = origin_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(self.e4 * inverse.scalar, 0.0, 0.0, 0.0)
    ));
}
fn origin_geometricQuotient_flector(self_: Origin, other: Flector) -> Origin {
    let self_groups = origin_grouped(self_);
    let other_groups = flector_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group0_.y, 2) + pow(other_groups.group0_.z, 2) - pow(other_groups.group1_.w, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(self.e4 * inverse.scalar, 0.0, 0.0, 0.0)
    ));
}
fn origin_geometricQuotient_horizon(self_: Origin, other: Horizon) -> Origin {
    let self_groups = origin_grouped(self_);
    let other_groups = horizon_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other.e321, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(self.e4 * inverse.scalar, 0.0, 0.0, 0.0)
    ));
}
fn origin_geometricQuotient_line(self_: Origin, other: Line) -> Origin {
    let self_groups = origin_grouped(self_);
    let other_groups = line_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(-pow(other_groups.group1_.x, 2) - pow(other_groups.group1_.y, 2) - pow(other_groups.group1_.z, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(self.e4 * inverse.scalar, 0.0, 0.0, 0.0)
    ));
}
fn origin_geometricQuotient_motor(self_: Origin, other: Motor) -> Origin {
    let self_groups = origin_grouped(self_);
    let other_groups = motor_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group1_.w, 2) - pow(other_groups.group1_.x, 2) - pow(other_groups.group1_.y, 2) - pow(other_groups.group1_.z, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(self.e4 * inverse.scalar, 0.0, 0.0, 0.0)
    ));
}
fn origin_geometricQuotient_multiVector(self_: Origin, other: MultiVector) -> Origin {
    let self_groups = origin_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group1_.x, 2) + pow(other_groups.group1_.y, 2) + pow(other_groups.group1_.z, 2) - pow(other_groups.group3_.x, 2) - pow(other_groups.group3_.y, 2) - pow(other_groups.group3_.z, 2) - pow(other_groups.group4_.w, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(self.e4 * inverse.scalar, 0.0, 0.0, 0.0)
    ));
}
fn origin_geometricQuotient_plane(self_: Origin, other: Plane) -> Origin {
    let self_groups = origin_grouped(self_);
    let other_groups = plane_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.w, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(self.e4 * inverse.scalar, 0.0, 0.0, 0.0)
    ));
}
fn origin_geometricQuotient_point(self_: Origin, other: Point) -> Origin {
    let self_groups = origin_grouped(self_);
    let other_groups = point_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group0_.y, 2) + pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(self.e4 * inverse.scalar, 0.0, 0.0, 0.0)
    ));
}
fn origin_geometricQuotient_scalar(self_: Origin, other: Scalar) -> Origin {
    let self_groups = origin_grouped(self_);
    let other_groups = scalar_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other.scalar, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(self.e4 * inverse.scalar, 0.0, 0.0, 0.0)
    ));
}
fn plane_geometricQuotient_dualNum(self_: Plane, other: DualNum) -> Plane {
    let self_groups = plane_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_.group0_
    ));
}
fn plane_geometricQuotient_flector(self_: Plane, other: Flector) -> Plane {
    let self_groups = plane_grouped(self_);
    let other_groups = flector_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group0_.y, 2) + pow(other_groups.group0_.z, 2) - pow(other_groups.group1_.w, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_.group0_
    ));
}
fn plane_geometricQuotient_horizon(self_: Plane, other: Horizon) -> Plane {
    let self_groups = plane_grouped(self_);
    let other_groups = horizon_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other.e321, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_.group0_
    ));
}
fn plane_geometricQuotient_line(self_: Plane, other: Line) -> Plane {
    let self_groups = plane_grouped(self_);
    let other_groups = line_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(-pow(other_groups.group1_.x, 2) - pow(other_groups.group1_.y, 2) - pow(other_groups.group1_.z, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_.group0_
    ));
}
fn plane_geometricQuotient_motor(self_: Plane, other: Motor) -> Plane {
    let self_groups = plane_grouped(self_);
    let other_groups = motor_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group1_.w, 2) - pow(other_groups.group1_.x, 2) - pow(other_groups.group1_.y, 2) - pow(other_groups.group1_.z, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_.group0_
    ));
}
fn plane_geometricQuotient_multiVector(self_: Plane, other: MultiVector) -> Plane {
    let self_groups = plane_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group1_.x, 2) + pow(other_groups.group1_.y, 2) + pow(other_groups.group1_.z, 2) - pow(other_groups.group3_.x, 2) - pow(other_groups.group3_.y, 2) - pow(other_groups.group3_.z, 2) - pow(other_groups.group4_.w, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_.group0_
    ));
}
fn plane_geometricQuotient_plane(self_: Plane, other: Plane) -> Plane {
    let self_groups = plane_grouped(self_);
    let other_groups = plane_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.w, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_.group0_
    ));
}
fn plane_geometricQuotient_point(self_: Plane, other: Point) -> Plane {
    let self_groups = plane_grouped(self_);
    let other_groups = point_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group0_.y, 2) + pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_.group0_
    ));
}
fn plane_geometricQuotient_scalar(self_: Plane, other: Scalar) -> Plane {
    let self_groups = plane_grouped(self_);
    let other_groups = scalar_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other.scalar, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(inverse.scalar) * self_.group0_
    ));
}
fn point_geometricQuotient_dualNum(self_: Point, other: DualNum) -> Point {
    let self_groups = point_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_.group0_
    ));
}
fn point_geometricQuotient_flector(self_: Point, other: Flector) -> Point {
    let self_groups = point_grouped(self_);
    let other_groups = flector_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group0_.y, 2) + pow(other_groups.group0_.z, 2) - pow(other_groups.group1_.w, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_.group0_
    ));
}
fn point_geometricQuotient_horizon(self_: Point, other: Horizon) -> Point {
    let self_groups = point_grouped(self_);
    let other_groups = horizon_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other.e321, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_.group0_
    ));
}
fn point_geometricQuotient_line(self_: Point, other: Line) -> Point {
    let self_groups = point_grouped(self_);
    let other_groups = line_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(-pow(other_groups.group1_.x, 2) - pow(other_groups.group1_.y, 2) - pow(other_groups.group1_.z, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_.group0_
    ));
}
fn point_geometricQuotient_motor(self_: Point, other: Motor) -> Point {
    let self_groups = point_grouped(self_);
    let other_groups = motor_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group1_.w, 2) - pow(other_groups.group1_.x, 2) - pow(other_groups.group1_.y, 2) - pow(other_groups.group1_.z, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_.group0_
    ));
}
fn point_geometricQuotient_multiVector(self_: Point, other: MultiVector) -> Point {
    let self_groups = point_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group1_.x, 2) + pow(other_groups.group1_.y, 2) + pow(other_groups.group1_.z, 2) - pow(other_groups.group3_.x, 2) - pow(other_groups.group3_.y, 2) - pow(other_groups.group3_.z, 2) - pow(other_groups.group4_.w, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_.group0_
    ));
}
fn point_geometricQuotient_plane(self_: Point, other: Plane) -> Point {
    let self_groups = point_grouped(self_);
    let other_groups = plane_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.w, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_.group0_
    ));
}
fn point_geometricQuotient_point(self_: Point, other: Point) -> Point {
    let self_groups = point_grouped(self_);
    let other_groups = point_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group0_.y, 2) + pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_.group0_
    ));
}
fn point_geometricQuotient_scalar(self_: Point, other: Scalar) -> Point {
    let self_groups = point_grouped(self_);
    let other_groups = scalar_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other.scalar, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(inverse.scalar) * self_.group0_
    ));
}
fn scalar_geometricQuotient_dualNum(self_: Scalar, other: DualNum) -> Scalar {
    let self_groups = scalar_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(inverse.scalar * self.scalar, 0.0, 0.0, 0.0)
    ));
}
fn scalar_geometricQuotient_flector(self_: Scalar, other: Flector) -> Scalar {
    let self_groups = scalar_grouped(self_);
    let other_groups = flector_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group0_.y, 2) + pow(other_groups.group0_.z, 2) - pow(other_groups.group1_.w, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(inverse.scalar * self.scalar, 0.0, 0.0, 0.0)
    ));
}
fn scalar_geometricQuotient_horizon(self_: Scalar, other: Horizon) -> Scalar {
    let self_groups = scalar_grouped(self_);
    let other_groups = horizon_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other.e321, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(inverse.scalar * self.scalar, 0.0, 0.0, 0.0)
    ));
}
fn scalar_geometricQuotient_line(self_: Scalar, other: Line) -> Scalar {
    let self_groups = scalar_grouped(self_);
    let other_groups = line_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(-pow(other_groups.group1_.x, 2) - pow(other_groups.group1_.y, 2) - pow(other_groups.group1_.z, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(inverse.scalar * self.scalar, 0.0, 0.0, 0.0)
    ));
}
fn scalar_geometricQuotient_motor(self_: Scalar, other: Motor) -> Scalar {
    let self_groups = scalar_grouped(self_);
    let other_groups = motor_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group1_.w, 2) - pow(other_groups.group1_.x, 2) - pow(other_groups.group1_.y, 2) - pow(other_groups.group1_.z, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(inverse.scalar * self.scalar, 0.0, 0.0, 0.0)
    ));
}
fn scalar_geometricQuotient_multiVector(self_: Scalar, other: MultiVector) -> Scalar {
    let self_groups = scalar_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group1_.x, 2) + pow(other_groups.group1_.y, 2) + pow(other_groups.group1_.z, 2) - pow(other_groups.group3_.x, 2) - pow(other_groups.group3_.y, 2) - pow(other_groups.group3_.z, 2) - pow(other_groups.group4_.w, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(inverse.scalar * self.scalar, 0.0, 0.0, 0.0)
    ));
}
fn scalar_geometricQuotient_plane(self_: Scalar, other: Plane) -> Scalar {
    let self_groups = scalar_grouped(self_);
    let other_groups = plane_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.w, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(inverse.scalar * self.scalar, 0.0, 0.0, 0.0)
    ));
}
fn scalar_geometricQuotient_point(self_: Scalar, other: Point) -> Scalar {
    let self_groups = scalar_grouped(self_);
    let other_groups = point_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other_groups.group0_.x, 2) + pow(other_groups.group0_.y, 2) + pow(other_groups.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(inverse.scalar * self.scalar, 0.0, 0.0, 0.0)
    ));
}
fn scalar_geometricQuotient_scalar(self_: Scalar, other: Scalar) -> Scalar {
    let self_groups = scalar_grouped(self_);
    let other_groups = scalar_grouped(other);
    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(other.scalar, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    let inverse_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    );
    let inverse: Scalar = scalar_degroup(inverse_groups);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(inverse.scalar * self.scalar, 0.0, 0.0, 0.0)
    ));
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
    let self_groups = antiScalar_grouped(self_);    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, self.e1234, 0.0, 0.0)
    ));
}
fn antiScalar_into_motor(self_: AntiScalar) -> Motor {
    let self_groups = antiScalar_grouped(self_);    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, self.e1234), 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0)
    ));
}
fn antiScalar_into_multiVector(self_: AntiScalar) -> MultiVector {
    let self_groups = antiScalar_grouped(self_);    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, self.e1234, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn dualNum_into_motor(self_: DualNum) -> Motor {
    let self_groups = dualNum_grouped(self_);    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, self.e1234), 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, self.scalar)
    ));
}
fn dualNum_into_multiVector(self_: DualNum) -> MultiVector {
    let self_groups = dualNum_grouped(self_);    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self.scalar, self.e1234, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn flector_into_multiVector(self_: Flector) -> MultiVector {
    let self_groups = flector_grouped(self_);    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self.e1, self.e2, self.e3, self.e4), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self.e423, self.e431, self.e412, self.e321)
    ));
}
fn horizon_into_flector(self_: Horizon) -> Flector {
    let self_groups = horizon_grouped(self_);    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self.e321)
    ));
}
fn horizon_into_multiVector(self_: Horizon) -> MultiVector {
    let self_groups = horizon_grouped(self_);    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self.e321)
    ));
}
fn horizon_into_plane(self_: Horizon) -> Plane {
    let self_groups = horizon_grouped(self_);    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self.e321)
    ));
}
fn line_into_motor(self_: Line) -> Motor {
    let self_groups = line_grouped(self_);    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self.e41, self.e42, self.e43, 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(self.e23, self.e31, self.e12, 0.0)
    ));
}
fn line_into_multiVector(self_: Line) -> MultiVector {
    let self_groups = line_grouped(self_);    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(self.e41, self.e42, self.e43, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(self.e23, self.e31, self.e12, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn motor_into_multiVector(self_: Motor) -> MultiVector {
    let self_groups = motor_grouped(self_);    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self.scalar, self.e1234, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(self.e41, self.e42, self.e43, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(self.e23, self.e31, self.e12, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn origin_into_flector(self_: Origin) -> Flector {
    let self_groups = origin_grouped(self_);    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self.e4), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn origin_into_multiVector(self_: Origin) -> MultiVector {
    let self_groups = origin_grouped(self_);    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self.e4), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn origin_into_point(self_: Origin) -> Point {
    let self_groups = origin_grouped(self_);    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self.e4)
    ));
}
fn plane_into_flector(self_: Plane) -> Flector {
    let self_groups = plane_grouped(self_);    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self.e423, self.e431, self.e412, self.e321)
    ));
}
fn plane_into_multiVector(self_: Plane) -> MultiVector {
    let self_groups = plane_grouped(self_);    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self.e423, self.e431, self.e412, self.e321)
    ));
}
fn point_into_flector(self_: Point) -> Flector {
    let self_groups = point_grouped(self_);    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self.e1, self.e2, self.e3, self.e4), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn point_into_multiVector(self_: Point) -> MultiVector {
    let self_groups = point_grouped(self_);    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self.e1, self.e2, self.e3, self.e4), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn scalar_into_dualNum(self_: Scalar) -> DualNum {
    let self_groups = scalar_grouped(self_);    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(self.scalar, 0.0, 0.0, 0.0)
    ));
}
fn scalar_into_motor(self_: Scalar) -> Motor {
    let self_groups = scalar_grouped(self_);    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, self.scalar)
    ));
}
fn scalar_into_multiVector(self_: Scalar) -> MultiVector {
    let self_groups = scalar_grouped(self_);    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self.scalar, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn dualNum_inverse(self_: DualNum) -> Scalar {
    let self_groups = dualNum_grouped(self_);    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(self_.group0_.x, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    ));
}
fn flector_inverse(self_: Flector) -> Scalar {
    let self_groups = flector_grouped(self_);    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(self_.group0_.x, 2) + pow(self_.group0_.y, 2) + pow(self_.group0_.z, 2) - pow(self_.group1_.w, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    ));
}
fn horizon_inverse(self_: Horizon) -> Scalar {
    let self_groups = horizon_grouped(self_);    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(self.e321, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    ));
}
fn line_inverse(self_: Line) -> Scalar {
    let self_groups = line_grouped(self_);    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(-pow(self_.group1_.x, 2) - pow(self_.group1_.y, 2) - pow(self_.group1_.z, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    ));
}
fn motor_inverse(self_: Motor) -> Scalar {
    let self_groups = motor_grouped(self_);    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(self_.group1_.w, 2) - pow(self_.group1_.x, 2) - pow(self_.group1_.y, 2) - pow(self_.group1_.z, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    ));
}
fn multiVector_inverse(self_: MultiVector) -> Scalar {
    let self_groups = multiVector_grouped(self_);    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(self_.group0_.x, 2) + pow(self_.group1_.x, 2) + pow(self_.group1_.y, 2) + pow(self_.group1_.z, 2) - pow(self_.group3_.x, 2) - pow(self_.group3_.y, 2) - pow(self_.group3_.z, 2) - pow(self_.group4_.w, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    ));
}
fn plane_inverse(self_: Plane) -> Scalar {
    let self_groups = plane_grouped(self_);    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(self_.group0_.w, 2) * -1.0, 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    ));
}
fn point_inverse(self_: Point) -> Scalar {
    let self_groups = point_grouped(self_);    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(self_.group0_.x, 2) + pow(self_.group0_.y, 2) + pow(self_.group0_.z, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    ));
}
fn scalar_inverse(self_: Scalar) -> Scalar {
    let self_groups = scalar_grouped(self_);    let scalar_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(pow(self.scalar, 2), 0.0, 0.0, 0.0)
    );
    let scalar_product: Scalar = scalar_degroup(scalar_product_groups);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(1.0/(scalar_product.scalar), 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_mul_dualNum(self_: AntiScalar, other: DualNum) -> AntiScalar {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return antiScalar_geometricProduct_dualNum(self, other);
}
fn antiScalar_mul_flector(self_: AntiScalar, other: Flector) -> Flector {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = flector_grouped(other);
    return antiScalar_geometricProduct_flector(self, other);
}
fn antiScalar_mul_horizon(self_: AntiScalar, other: Horizon) -> Origin {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = horizon_grouped(other);
    return antiScalar_geometricProduct_horizon(self, other);
}
fn antiScalar_mul_line(self_: AntiScalar, other: Line) -> Line {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = line_grouped(other);
    return antiScalar_geometricProduct_line(self, other);
}
fn antiScalar_mul_motor(self_: AntiScalar, other: Motor) -> Motor {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = motor_grouped(other);
    return antiScalar_geometricProduct_motor(self, other);
}
fn antiScalar_mul_multiVector(self_: AntiScalar, other: MultiVector) -> MultiVector {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return antiScalar_geometricProduct_multiVector(self, other);
}
fn antiScalar_mul_plane(self_: AntiScalar, other: Plane) -> Origin {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = plane_grouped(other);
    return antiScalar_geometricProduct_plane(self, other);
}
fn antiScalar_mul_point(self_: AntiScalar, other: Point) -> Plane {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = point_grouped(other);
    return antiScalar_geometricProduct_point(self, other);
}
fn antiScalar_mul_scalar(self_: AntiScalar, other: Scalar) -> AntiScalar {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = scalar_grouped(other);
    return antiScalar_geometricProduct_scalar(self, other);
}
fn dualNum_mul_antiScalar(self_: DualNum, other: AntiScalar) -> AntiScalar {
    let self_groups = dualNum_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return dualNum_geometricProduct_antiScalar(self, other);
}
fn dualNum_mul_dualNum(self_: DualNum, other: DualNum) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return dualNum_geometricProduct_dualNum(self, other);
}
fn dualNum_mul_flector(self_: DualNum, other: Flector) -> Flector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = flector_grouped(other);
    return dualNum_geometricProduct_flector(self, other);
}
fn dualNum_mul_horizon(self_: DualNum, other: Horizon) -> Flector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = horizon_grouped(other);
    return dualNum_geometricProduct_horizon(self, other);
}
fn dualNum_mul_line(self_: DualNum, other: Line) -> Line {
    let self_groups = dualNum_grouped(self_);
    let other_groups = line_grouped(other);
    return dualNum_geometricProduct_line(self, other);
}
fn dualNum_mul_motor(self_: DualNum, other: Motor) -> Motor {
    let self_groups = dualNum_grouped(self_);
    let other_groups = motor_grouped(other);
    return dualNum_geometricProduct_motor(self, other);
}
fn dualNum_mul_multiVector(self_: DualNum, other: MultiVector) -> MultiVector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return dualNum_geometricProduct_multiVector(self, other);
}
fn dualNum_mul_origin(self_: DualNum, other: Origin) -> Origin {
    let self_groups = dualNum_grouped(self_);
    let other_groups = origin_grouped(other);
    return dualNum_geometricProduct_origin(self, other);
}
fn dualNum_mul_plane(self_: DualNum, other: Plane) -> Flector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = plane_grouped(other);
    return dualNum_geometricProduct_plane(self, other);
}
fn dualNum_mul_point(self_: DualNum, other: Point) -> Flector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = point_grouped(other);
    return dualNum_geometricProduct_point(self, other);
}
fn dualNum_mul_scalar(self_: DualNum, other: Scalar) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let other_groups = scalar_grouped(other);
    return dualNum_geometricProduct_scalar(self, other);
}
fn flector_mul_antiScalar(self_: Flector, other: AntiScalar) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return flector_geometricProduct_antiScalar(self, other);
}
fn flector_mul_dualNum(self_: Flector, other: DualNum) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return flector_geometricProduct_dualNum(self, other);
}
fn flector_mul_flector(self_: Flector, other: Flector) -> Motor {
    let self_groups = flector_grouped(self_);
    let other_groups = flector_grouped(other);
    return flector_geometricProduct_flector(self, other);
}
fn flector_mul_horizon(self_: Flector, other: Horizon) -> Motor {
    let self_groups = flector_grouped(self_);
    let other_groups = horizon_grouped(other);
    return flector_geometricProduct_horizon(self, other);
}
fn flector_mul_line(self_: Flector, other: Line) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = line_grouped(other);
    return flector_geometricProduct_line(self, other);
}
fn flector_mul_motor(self_: Flector, other: Motor) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = motor_grouped(other);
    return flector_geometricProduct_motor(self, other);
}
fn flector_mul_multiVector(self_: Flector, other: MultiVector) -> MultiVector {
    let self_groups = flector_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return flector_geometricProduct_multiVector(self, other);
}
fn flector_mul_origin(self_: Flector, other: Origin) -> Motor {
    let self_groups = flector_grouped(self_);
    let other_groups = origin_grouped(other);
    return flector_geometricProduct_origin(self, other);
}
fn flector_mul_plane(self_: Flector, other: Plane) -> Motor {
    let self_groups = flector_grouped(self_);
    let other_groups = plane_grouped(other);
    return flector_geometricProduct_plane(self, other);
}
fn flector_mul_point(self_: Flector, other: Point) -> Motor {
    let self_groups = flector_grouped(self_);
    let other_groups = point_grouped(other);
    return flector_geometricProduct_point(self, other);
}
fn flector_mul_scalar(self_: Flector, other: Scalar) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = scalar_grouped(other);
    return flector_geometricProduct_scalar(self, other);
}
fn horizon_mul_antiScalar(self_: Horizon, other: AntiScalar) -> Origin {
    let self_groups = horizon_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return horizon_geometricProduct_antiScalar(self, other);
}
fn horizon_mul_dualNum(self_: Horizon, other: DualNum) -> Flector {
    let self_groups = horizon_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return horizon_geometricProduct_dualNum(self, other);
}
fn horizon_mul_flector(self_: Horizon, other: Flector) -> Motor {
    let self_groups = horizon_grouped(self_);
    let other_groups = flector_grouped(other);
    return horizon_geometricProduct_flector(self, other);
}
fn horizon_mul_horizon(self_: Horizon, other: Horizon) -> Scalar {
    let self_groups = horizon_grouped(self_);
    let other_groups = horizon_grouped(other);
    return horizon_geometricProduct_horizon(self, other);
}
fn horizon_mul_line(self_: Horizon, other: Line) -> Flector {
    let self_groups = horizon_grouped(self_);
    let other_groups = line_grouped(other);
    return horizon_geometricProduct_line(self, other);
}
fn horizon_mul_motor(self_: Horizon, other: Motor) -> Flector {
    let self_groups = horizon_grouped(self_);
    let other_groups = motor_grouped(other);
    return horizon_geometricProduct_motor(self, other);
}
fn horizon_mul_multiVector(self_: Horizon, other: MultiVector) -> MultiVector {
    let self_groups = horizon_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return horizon_geometricProduct_multiVector(self, other);
}
fn horizon_mul_origin(self_: Horizon, other: Origin) -> AntiScalar {
    let self_groups = horizon_grouped(self_);
    let other_groups = origin_grouped(other);
    return horizon_geometricProduct_origin(self, other);
}
fn horizon_mul_plane(self_: Horizon, other: Plane) -> Motor {
    let self_groups = horizon_grouped(self_);
    let other_groups = plane_grouped(other);
    return horizon_geometricProduct_plane(self, other);
}
fn horizon_mul_point(self_: Horizon, other: Point) -> Motor {
    let self_groups = horizon_grouped(self_);
    let other_groups = point_grouped(other);
    return horizon_geometricProduct_point(self, other);
}
fn horizon_mul_scalar(self_: Horizon, other: Scalar) -> Horizon {
    let self_groups = horizon_grouped(self_);
    let other_groups = scalar_grouped(other);
    return horizon_geometricProduct_scalar(self, other);
}
fn line_mul_antiScalar(self_: Line, other: AntiScalar) -> Line {
    let self_groups = line_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return line_geometricProduct_antiScalar(self, other);
}
fn line_mul_dualNum(self_: Line, other: DualNum) -> Line {
    let self_groups = line_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return line_geometricProduct_dualNum(self, other);
}
fn line_mul_flector(self_: Line, other: Flector) -> Flector {
    let self_groups = line_grouped(self_);
    let other_groups = flector_grouped(other);
    return line_geometricProduct_flector(self, other);
}
fn line_mul_horizon(self_: Line, other: Horizon) -> Flector {
    let self_groups = line_grouped(self_);
    let other_groups = horizon_grouped(other);
    return line_geometricProduct_horizon(self, other);
}
fn line_mul_line(self_: Line, other: Line) -> Motor {
    let self_groups = line_grouped(self_);
    let other_groups = line_grouped(other);
    return line_geometricProduct_line(self, other);
}
fn line_mul_motor(self_: Line, other: Motor) -> Motor {
    let self_groups = line_grouped(self_);
    let other_groups = motor_grouped(other);
    return line_geometricProduct_motor(self, other);
}
fn line_mul_multiVector(self_: Line, other: MultiVector) -> MultiVector {
    let self_groups = line_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return line_geometricProduct_multiVector(self, other);
}
fn line_mul_origin(self_: Line, other: Origin) -> Plane {
    let self_groups = line_grouped(self_);
    let other_groups = origin_grouped(other);
    return line_geometricProduct_origin(self, other);
}
fn line_mul_plane(self_: Line, other: Plane) -> Flector {
    let self_groups = line_grouped(self_);
    let other_groups = plane_grouped(other);
    return line_geometricProduct_plane(self, other);
}
fn line_mul_point(self_: Line, other: Point) -> Flector {
    let self_groups = line_grouped(self_);
    let other_groups = point_grouped(other);
    return line_geometricProduct_point(self, other);
}
fn line_mul_scalar(self_: Line, other: Scalar) -> Line {
    let self_groups = line_grouped(self_);
    let other_groups = scalar_grouped(other);
    return line_geometricProduct_scalar(self, other);
}
fn motor_mul_antiScalar(self_: Motor, other: AntiScalar) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return motor_geometricProduct_antiScalar(self, other);
}
fn motor_mul_dualNum(self_: Motor, other: DualNum) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return motor_geometricProduct_dualNum(self, other);
}
fn motor_mul_flector(self_: Motor, other: Flector) -> Flector {
    let self_groups = motor_grouped(self_);
    let other_groups = flector_grouped(other);
    return motor_geometricProduct_flector(self, other);
}
fn motor_mul_horizon(self_: Motor, other: Horizon) -> Flector {
    let self_groups = motor_grouped(self_);
    let other_groups = horizon_grouped(other);
    return motor_geometricProduct_horizon(self, other);
}
fn motor_mul_line(self_: Motor, other: Line) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = line_grouped(other);
    return motor_geometricProduct_line(self, other);
}
fn motor_mul_motor(self_: Motor, other: Motor) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = motor_grouped(other);
    return motor_geometricProduct_motor(self, other);
}
fn motor_mul_multiVector(self_: Motor, other: MultiVector) -> MultiVector {
    let self_groups = motor_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return motor_geometricProduct_multiVector(self, other);
}
fn motor_mul_origin(self_: Motor, other: Origin) -> Flector {
    let self_groups = motor_grouped(self_);
    let other_groups = origin_grouped(other);
    return motor_geometricProduct_origin(self, other);
}
fn motor_mul_plane(self_: Motor, other: Plane) -> Flector {
    let self_groups = motor_grouped(self_);
    let other_groups = plane_grouped(other);
    return motor_geometricProduct_plane(self, other);
}
fn motor_mul_point(self_: Motor, other: Point) -> Flector {
    let self_groups = motor_grouped(self_);
    let other_groups = point_grouped(other);
    return motor_geometricProduct_point(self, other);
}
fn motor_mul_scalar(self_: Motor, other: Scalar) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = scalar_grouped(other);
    return motor_geometricProduct_scalar(self, other);
}
fn multiVector_mul_antiScalar(self_: MultiVector, other: AntiScalar) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return multiVector_geometricProduct_antiScalar(self, other);
}
fn multiVector_mul_dualNum(self_: MultiVector, other: DualNum) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return multiVector_geometricProduct_dualNum(self, other);
}
fn multiVector_mul_flector(self_: MultiVector, other: Flector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = flector_grouped(other);
    return multiVector_geometricProduct_flector(self, other);
}
fn multiVector_mul_horizon(self_: MultiVector, other: Horizon) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = horizon_grouped(other);
    return multiVector_geometricProduct_horizon(self, other);
}
fn multiVector_mul_line(self_: MultiVector, other: Line) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = line_grouped(other);
    return multiVector_geometricProduct_line(self, other);
}
fn multiVector_mul_motor(self_: MultiVector, other: Motor) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = motor_grouped(other);
    return multiVector_geometricProduct_motor(self, other);
}
fn multiVector_mul_multiVector(self_: MultiVector, other: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_geometricProduct_multiVector(self, other);
}
fn multiVector_mul_origin(self_: MultiVector, other: Origin) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = origin_grouped(other);
    return multiVector_geometricProduct_origin(self, other);
}
fn multiVector_mul_plane(self_: MultiVector, other: Plane) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = plane_grouped(other);
    return multiVector_geometricProduct_plane(self, other);
}
fn multiVector_mul_point(self_: MultiVector, other: Point) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = point_grouped(other);
    return multiVector_geometricProduct_point(self, other);
}
fn multiVector_mul_scalar(self_: MultiVector, other: Scalar) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = scalar_grouped(other);
    return multiVector_geometricProduct_scalar(self, other);
}
fn origin_mul_dualNum(self_: Origin, other: DualNum) -> Origin {
    let self_groups = origin_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return origin_geometricProduct_dualNum(self, other);
}
fn origin_mul_flector(self_: Origin, other: Flector) -> Motor {
    let self_groups = origin_grouped(self_);
    let other_groups = flector_grouped(other);
    return origin_geometricProduct_flector(self, other);
}
fn origin_mul_horizon(self_: Origin, other: Horizon) -> AntiScalar {
    let self_groups = origin_grouped(self_);
    let other_groups = horizon_grouped(other);
    return origin_geometricProduct_horizon(self, other);
}
fn origin_mul_line(self_: Origin, other: Line) -> Plane {
    let self_groups = origin_grouped(self_);
    let other_groups = line_grouped(other);
    return origin_geometricProduct_line(self, other);
}
fn origin_mul_motor(self_: Origin, other: Motor) -> Flector {
    let self_groups = origin_grouped(self_);
    let other_groups = motor_grouped(other);
    return origin_geometricProduct_motor(self, other);
}
fn origin_mul_multiVector(self_: Origin, other: MultiVector) -> MultiVector {
    let self_groups = origin_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return origin_geometricProduct_multiVector(self, other);
}
fn origin_mul_plane(self_: Origin, other: Plane) -> AntiScalar {
    let self_groups = origin_grouped(self_);
    let other_groups = plane_grouped(other);
    return origin_geometricProduct_plane(self, other);
}
fn origin_mul_point(self_: Origin, other: Point) -> Line {
    let self_groups = origin_grouped(self_);
    let other_groups = point_grouped(other);
    return origin_geometricProduct_point(self, other);
}
fn origin_mul_scalar(self_: Origin, other: Scalar) -> Origin {
    let self_groups = origin_grouped(self_);
    let other_groups = scalar_grouped(other);
    return origin_geometricProduct_scalar(self, other);
}
fn plane_mul_antiScalar(self_: Plane, other: AntiScalar) -> Origin {
    let self_groups = plane_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return plane_geometricProduct_antiScalar(self, other);
}
fn plane_mul_dualNum(self_: Plane, other: DualNum) -> Flector {
    let self_groups = plane_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return plane_geometricProduct_dualNum(self, other);
}
fn plane_mul_flector(self_: Plane, other: Flector) -> Motor {
    let self_groups = plane_grouped(self_);
    let other_groups = flector_grouped(other);
    return plane_geometricProduct_flector(self, other);
}
fn plane_mul_horizon(self_: Plane, other: Horizon) -> Motor {
    let self_groups = plane_grouped(self_);
    let other_groups = horizon_grouped(other);
    return plane_geometricProduct_horizon(self, other);
}
fn plane_mul_line(self_: Plane, other: Line) -> Flector {
    let self_groups = plane_grouped(self_);
    let other_groups = line_grouped(other);
    return plane_geometricProduct_line(self, other);
}
fn plane_mul_motor(self_: Plane, other: Motor) -> Flector {
    let self_groups = plane_grouped(self_);
    let other_groups = motor_grouped(other);
    return plane_geometricProduct_motor(self, other);
}
fn plane_mul_multiVector(self_: Plane, other: MultiVector) -> MultiVector {
    let self_groups = plane_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return plane_geometricProduct_multiVector(self, other);
}
fn plane_mul_origin(self_: Plane, other: Origin) -> AntiScalar {
    let self_groups = plane_grouped(self_);
    let other_groups = origin_grouped(other);
    return plane_geometricProduct_origin(self, other);
}
fn plane_mul_plane(self_: Plane, other: Plane) -> Motor {
    let self_groups = plane_grouped(self_);
    let other_groups = plane_grouped(other);
    return plane_geometricProduct_plane(self, other);
}
fn plane_mul_point(self_: Plane, other: Point) -> Motor {
    let self_groups = plane_grouped(self_);
    let other_groups = point_grouped(other);
    return plane_geometricProduct_point(self, other);
}
fn plane_mul_scalar(self_: Plane, other: Scalar) -> Plane {
    let self_groups = plane_grouped(self_);
    let other_groups = scalar_grouped(other);
    return plane_geometricProduct_scalar(self, other);
}
fn point_mul_antiScalar(self_: Point, other: AntiScalar) -> Plane {
    let self_groups = point_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return point_geometricProduct_antiScalar(self, other);
}
fn point_mul_dualNum(self_: Point, other: DualNum) -> Flector {
    let self_groups = point_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return point_geometricProduct_dualNum(self, other);
}
fn point_mul_flector(self_: Point, other: Flector) -> Motor {
    let self_groups = point_grouped(self_);
    let other_groups = flector_grouped(other);
    return point_geometricProduct_flector(self, other);
}
fn point_mul_horizon(self_: Point, other: Horizon) -> Motor {
    let self_groups = point_grouped(self_);
    let other_groups = horizon_grouped(other);
    return point_geometricProduct_horizon(self, other);
}
fn point_mul_line(self_: Point, other: Line) -> Flector {
    let self_groups = point_grouped(self_);
    let other_groups = line_grouped(other);
    return point_geometricProduct_line(self, other);
}
fn point_mul_motor(self_: Point, other: Motor) -> Flector {
    let self_groups = point_grouped(self_);
    let other_groups = motor_grouped(other);
    return point_geometricProduct_motor(self, other);
}
fn point_mul_multiVector(self_: Point, other: MultiVector) -> MultiVector {
    let self_groups = point_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return point_geometricProduct_multiVector(self, other);
}
fn point_mul_origin(self_: Point, other: Origin) -> Line {
    let self_groups = point_grouped(self_);
    let other_groups = origin_grouped(other);
    return point_geometricProduct_origin(self, other);
}
fn point_mul_plane(self_: Point, other: Plane) -> Motor {
    let self_groups = point_grouped(self_);
    let other_groups = plane_grouped(other);
    return point_geometricProduct_plane(self, other);
}
fn point_mul_point(self_: Point, other: Point) -> Motor {
    let self_groups = point_grouped(self_);
    let other_groups = point_grouped(other);
    return point_geometricProduct_point(self, other);
}
fn point_mul_scalar(self_: Point, other: Scalar) -> Point {
    let self_groups = point_grouped(self_);
    let other_groups = scalar_grouped(other);
    return point_geometricProduct_scalar(self, other);
}
fn scalar_mul_antiScalar(self_: Scalar, other: AntiScalar) -> AntiScalar {
    let self_groups = scalar_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return scalar_geometricProduct_antiScalar(self, other);
}
fn scalar_mul_dualNum(self_: Scalar, other: DualNum) -> DualNum {
    let self_groups = scalar_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return scalar_geometricProduct_dualNum(self, other);
}
fn scalar_mul_flector(self_: Scalar, other: Flector) -> Flector {
    let self_groups = scalar_grouped(self_);
    let other_groups = flector_grouped(other);
    return scalar_geometricProduct_flector(self, other);
}
fn scalar_mul_horizon(self_: Scalar, other: Horizon) -> Horizon {
    let self_groups = scalar_grouped(self_);
    let other_groups = horizon_grouped(other);
    return scalar_geometricProduct_horizon(self, other);
}
fn scalar_mul_line(self_: Scalar, other: Line) -> Line {
    let self_groups = scalar_grouped(self_);
    let other_groups = line_grouped(other);
    return scalar_geometricProduct_line(self, other);
}
fn scalar_mul_motor(self_: Scalar, other: Motor) -> Motor {
    let self_groups = scalar_grouped(self_);
    let other_groups = motor_grouped(other);
    return scalar_geometricProduct_motor(self, other);
}
fn scalar_mul_multiVector(self_: Scalar, other: MultiVector) -> MultiVector {
    let self_groups = scalar_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return scalar_geometricProduct_multiVector(self, other);
}
fn scalar_mul_origin(self_: Scalar, other: Origin) -> Origin {
    let self_groups = scalar_grouped(self_);
    let other_groups = origin_grouped(other);
    return scalar_geometricProduct_origin(self, other);
}
fn scalar_mul_plane(self_: Scalar, other: Plane) -> Plane {
    let self_groups = scalar_grouped(self_);
    let other_groups = plane_grouped(other);
    return scalar_geometricProduct_plane(self, other);
}
fn scalar_mul_point(self_: Scalar, other: Point) -> Point {
    let self_groups = scalar_grouped(self_);
    let other_groups = point_grouped(other);
    return scalar_geometricProduct_point(self, other);
}
fn scalar_mul_scalar(self_: Scalar, other: Scalar) -> Scalar {
    let self_groups = scalar_grouped(self_);
    let other_groups = scalar_grouped(other);
    return scalar_geometricProduct_scalar(self, other);
}
fn antiScalar_neg(self_: AntiScalar) -> AntiScalar {
    let self_groups = antiScalar_grouped(self_);    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(self.e1234 * -1.0, 0.0, 0.0, 0.0)
    ));
}
fn dualNum_neg(self_: DualNum) -> DualNum {
    let self_groups = dualNum_grouped(self_);    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ self_.group0_ * vec2<f32>(-1.0)
    ));
}
fn flector_neg(self_: Flector) -> Flector {
    let self_groups = flector_grouped(self_);    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ self_.group0_ * vec4<f32>(-1.0), 
        /* e423, e431, e412, e321 */ self_.group1_ * vec4<f32>(-1.0)
    ));
}
fn horizon_neg(self_: Horizon) -> Horizon {
    let self_groups = horizon_grouped(self_);    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(self.e321 * -1.0, 0.0, 0.0, 0.0)
    ));
}
fn line_neg(self_: Line) -> Line {
    let self_groups = line_grouped(self_);    return line_degroup(LineGroups(
        /* e41, e42, e43 */ self_.group0_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ self_.group1_ * vec3<f32>(-1.0)
    ));
}
fn motor_neg(self_: Motor) -> Motor {
    let self_groups = motor_grouped(self_);    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ self_.group0_ * vec4<f32>(-1.0), 
        /* e23, e31, e12, scalar */ self_.group1_ * vec4<f32>(-1.0)
    ));
}
fn multiVector_neg(self_: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_.group0_ * vec2<f32>(-1.0), 
        /* e1, e2, e3, e4 */ self_.group1_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ self_.group2_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ self_.group3_ * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ self_.group4_ * vec4<f32>(-1.0)
    ));
}
fn origin_neg(self_: Origin) -> Origin {
    let self_groups = origin_grouped(self_);    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(self.e4 * -1.0, 0.0, 0.0, 0.0)
    ));
}
fn plane_neg(self_: Plane) -> Plane {
    let self_groups = plane_grouped(self_);    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ self_.group0_ * vec4<f32>(-1.0)
    ));
}
fn point_neg(self_: Point) -> Point {
    let self_groups = point_grouped(self_);    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ self_.group0_ * vec4<f32>(-1.0)
    ));
}
fn scalar_neg(self_: Scalar) -> Scalar {
    let self_groups = scalar_grouped(self_);    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(self.scalar * -1.0, 0.0, 0.0, 0.0)
    ));
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
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(1.0, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_reverse(self_: AntiScalar) -> AntiScalar {
    let self_groups = antiScalar_grouped(self_);    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(self.e1234, 0.0, 0.0, 0.0)
    ));
}
fn dualNum_reverse(self_: DualNum) -> DualNum {
    let self_groups = dualNum_grouped(self_);    return self;
}
fn flector_reverse(self_: Flector) -> Flector {
    let self_groups = flector_grouped(self_);    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ self_.group0_, 
        /* e423, e431, e412, e321 */ self_.group1_ * vec4<f32>(-1.0)
    ));
}
fn horizon_reverse(self_: Horizon) -> Horizon {
    let self_groups = horizon_grouped(self_);    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(self.e321 * -1.0, 0.0, 0.0, 0.0)
    ));
}
fn line_reverse(self_: Line) -> Line {
    let self_groups = line_grouped(self_);    return line_degroup(LineGroups(
        /* e41, e42, e43 */ self_.group0_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ self_.group1_ * vec3<f32>(-1.0)
    ));
}
fn motor_reverse(self_: Motor) -> Motor {
    let self_groups = motor_grouped(self_);    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.group0_.x * -1.0, self_.group0_.y * -1.0, self_.group0_.z * -1.0, self_.group0_.w), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.group1_.x * -1.0, self_.group1_.y * -1.0, self_.group1_.z * -1.0, self_.group1_.w)
    ));
}
fn multiVector_reverse(self_: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_.group0_, 
        /* e1, e2, e3, e4 */ self_.group1_, 
        /* e41, e42, e43 */ self_.group2_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ self_.group3_ * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ self_.group4_ * vec4<f32>(-1.0)
    ));
}
fn origin_reverse(self_: Origin) -> Origin {
    let self_groups = origin_grouped(self_);    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(self.e4, 0.0, 0.0, 0.0)
    ));
}
fn plane_reverse(self_: Plane) -> Plane {
    let self_groups = plane_grouped(self_);    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ self_.group0_ * vec4<f32>(-1.0)
    ));
}
fn point_reverse(self_: Point) -> Point {
    let self_groups = point_grouped(self_);    return self;
}
fn scalar_reverse(self_: Scalar) -> Scalar {
    let self_groups = scalar_grouped(self_);    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(self.scalar, 0.0, 0.0, 0.0)
    ));
}
fn dualNum_rightAntiDual(self_: DualNum) -> AntiScalar {
    let self_groups = dualNum_grouped(self_);    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(self_.group0_.x, 0.0, 0.0, 0.0)
    ));
}
fn flector_rightAntiDual(self_: Flector) -> Flector {
    let self_groups = flector_grouped(self_);    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.group1_.w * -1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0)
    ));
}
fn horizon_rightAntiDual(self_: Horizon) -> Origin {
    let self_groups = horizon_grouped(self_);    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(self.e321 * -1.0, 0.0, 0.0, 0.0)
    ));
}
fn line_rightAntiDual(self_: Line) -> Line {
    let self_groups = line_grouped(self_);    return line_degroup(LineGroups(
        /* e41, e42, e43 */ self_.group1_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0)
    ));
}
fn motor_rightAntiDual(self_: Motor) -> Motor {
    let self_groups = motor_grouped(self_);    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.group1_.x * -1.0, self_.group1_.y * -1.0, self_.group1_.z * -1.0, self_.group1_.w), 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0)
    ));
}
fn multiVector_rightAntiDual(self_: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, self_.group0_.x, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.group4_.w * -1.0), 
        /* e41, e42, e43 */ self_.group3_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0)
    ));
}
fn plane_rightAntiDual(self_: Plane) -> Origin {
    let self_groups = plane_grouped(self_);    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(self_.group0_.w * -1.0, 0.0, 0.0, 0.0)
    ));
}
fn point_rightAntiDual(self_: Point) -> Plane {
    let self_groups = point_grouped(self_);    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0)
    ));
}
fn scalar_rightAntiDual(self_: Scalar) -> AntiScalar {
    let self_groups = scalar_grouped(self_);    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(self.scalar, 0.0, 0.0, 0.0)
    ));
}
fn dualNum_rightDual(self_: DualNum) -> AntiScalar {
    let self_groups = dualNum_grouped(self_);    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(self_.group0_.x, 0.0, 0.0, 0.0)
    ));
}
fn flector_rightDual(self_: Flector) -> Flector {
    let self_groups = flector_grouped(self_);    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.group1_.w * -1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0)
    ));
}
fn horizon_rightDual(self_: Horizon) -> Origin {
    let self_groups = horizon_grouped(self_);    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(self.e321 * -1.0, 0.0, 0.0, 0.0)
    ));
}
fn line_rightDual(self_: Line) -> Line {
    let self_groups = line_grouped(self_);    return line_degroup(LineGroups(
        /* e41, e42, e43 */ self_.group1_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0)
    ));
}
fn motor_rightDual(self_: Motor) -> Motor {
    let self_groups = motor_grouped(self_);    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.group1_.x * -1.0, self_.group1_.y * -1.0, self_.group1_.z * -1.0, self_.group1_.w), 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0)
    ));
}
fn multiVector_rightDual(self_: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, self_.group0_.x, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.group4_.w * -1.0), 
        /* e41, e42, e43 */ self_.group3_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0)
    ));
}
fn plane_rightDual(self_: Plane) -> Origin {
    let self_groups = plane_grouped(self_);    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(self_.group0_.w * -1.0, 0.0, 0.0, 0.0)
    ));
}
fn point_rightDual(self_: Point) -> Plane {
    let self_groups = point_grouped(self_);    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0)
    ));
}
fn scalar_rightDual(self_: Scalar) -> AntiScalar {
    let self_groups = scalar_grouped(self_);    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(self.scalar, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_sandwich_flector(self_: AntiScalar, other: Flector) -> Flector {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = flector_grouped(other);
    let geometric_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group1_.w * self.e1234 * -1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group0_.x * self.e1234 * -1.0, other_groups.group0_.y * self.e1234 * -1.0, other_groups.group0_.z * self.e1234 * -1.0, 0.0)
    );
    let geometric_product: Flector = flector_degroup(geometric_product_groups);
    return flector_geometricProduct_antiScalar(geometric_product, antiScalar_reverse(self));
}
fn antiScalar_sandwich_line(self_: AntiScalar, other: Line) -> Line {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = line_grouped(other);
    let geometric_product_groups: LineGroups = LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(self.e1234), 0.0) * other_groups.group1_, 
        /* e23, e31, e12 */ vec4<f32>(0.0)
    );
    let geometric_product: Line = line_degroup(geometric_product_groups);
    return line_geometricProduct_antiScalar(geometric_product, antiScalar_reverse(self));
}
fn antiScalar_sandwich_motor(self_: AntiScalar, other: Motor) -> Motor {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = motor_grouped(other);
    let geometric_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self.e1234) * other_groups.group1_, 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0)
    );
    let geometric_product: Motor = motor_degroup(geometric_product_groups);
    return motor_geometricProduct_antiScalar(geometric_product, antiScalar_reverse(self));
}
fn antiScalar_sandwich_multiVector(self_: AntiScalar, other: MultiVector) -> MultiVector {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let geometric_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, other_groups.group0_.x * self.e1234, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group4_.w * self.e1234 * -1.0), 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(self.e1234), 0.0) * other_groups.group3_, 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group1_.x * self.e1234 * -1.0, other_groups.group1_.y * self.e1234 * -1.0, other_groups.group1_.z * self.e1234 * -1.0, 0.0)
    );
    let geometric_product: MultiVector = multiVector_degroup(geometric_product_groups);
    return multiVector_geometricProduct_antiScalar(geometric_product, antiScalar_reverse(self));
}
fn antiScalar_sandwich_point(self_: AntiScalar, other: Point) -> Origin {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = point_grouped(other);
    let geometric_product_groups: PlaneGroups = PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group0_.x * self.e1234 * -1.0, other_groups.group0_.y * self.e1234 * -1.0, other_groups.group0_.z * self.e1234 * -1.0, 0.0)
    );
    let geometric_product: Plane = plane_degroup(geometric_product_groups);
    return plane_geometricProduct_antiScalar(geometric_product, antiScalar_reverse(self));
}
fn dualNum_sandwich_antiScalar(self_: DualNum, other: AntiScalar) -> AntiScalar {
    let self_groups = dualNum_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    let geometric_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(self_.group0_.x * other.e1234, 0.0, 0.0, 0.0)
    );
    let geometric_product: AntiScalar = antiScalar_degroup(geometric_product_groups);
    return antiScalar_geometricProduct_dualNum(geometric_product, dualNum_reverse(self));
}
fn dualNum_sandwich_dualNum(self_: DualNum, other: DualNum) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let geometric_product_groups: DualNumGroups = DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(other_groups.group0_.x * self_.group0_.x, (other_groups.group0_.x * self_.group0_.y) + (other_groups.group0_.y * self_.group0_.x), 0.0, 0.0)
    );
    let geometric_product: DualNum = dualNum_degroup(geometric_product_groups);
    return dualNum_geometricProduct_dualNum(geometric_product, dualNum_reverse(self));
}
fn dualNum_sandwich_flector(self_: DualNum, other: Flector) -> Flector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = flector_grouped(other);
    let geometric_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group0_.x * other_groups.group0_.x, self_.group0_.x * other_groups.group0_.y, self_.group0_.x * other_groups.group0_.z, (self_.group0_.x * other_groups.group0_.w) - (self_.group0_.y * other_groups.group1_.w)), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.group0_.x * other_groups.group1_.x) - (self_.group0_.y * other_groups.group0_.x), (self_.group0_.x * other_groups.group1_.y) - (self_.group0_.y * other_groups.group0_.y), (self_.group0_.x * other_groups.group1_.z) - (self_.group0_.y * other_groups.group0_.z), self_.group0_.x * other_groups.group1_.w)
    );
    let geometric_product: Flector = flector_degroup(geometric_product_groups);
    return flector_geometricProduct_dualNum(geometric_product, dualNum_reverse(self));
}
fn dualNum_sandwich_horizon(self_: DualNum, other: Horizon) -> Flector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = horizon_grouped(other);
    let geometric_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.group0_.y * other.e321 * -1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.group0_.x * other.e321)
    );
    let geometric_product: Flector = flector_degroup(geometric_product_groups);
    return flector_geometricProduct_dualNum(geometric_product, dualNum_reverse(self));
}
fn dualNum_sandwich_line(self_: DualNum, other: Line) -> Line {
    let self_groups = dualNum_grouped(self_);
    let other_groups = line_grouped(other);
    let geometric_product_groups: LineGroups = LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(vec3<f32>(self_.group0_.x), 0.0) * other_groups.group0_) + (vec4<f32>(vec3<f32>(self_.group0_.y), 0.0) * other_groups.group1_), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(self_.group0_.x), 0.0) * other_groups.group1_
    );
    let geometric_product: Line = line_degroup(geometric_product_groups);
    return line_geometricProduct_dualNum(geometric_product, dualNum_reverse(self));
}
fn dualNum_sandwich_motor(self_: DualNum, other: Motor) -> Motor {
    let self_groups = dualNum_grouped(self_);
    let other_groups = motor_grouped(other);
    let geometric_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ (vec4<f32>(self_.group0_.x) * other_groups.group0_) + (vec4<f32>(self_.group0_.y) * other_groups.group1_), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.group0_.x) * other_groups.group1_
    );
    let geometric_product: Motor = motor_degroup(geometric_product_groups);
    return motor_geometricProduct_dualNum(geometric_product, dualNum_reverse(self));
}
fn dualNum_sandwich_multiVector(self_: DualNum, other: MultiVector) -> MultiVector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let geometric_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.group0_.x * other_groups.group0_.x, (self_.group0_.x * other_groups.group0_.y) + (self_.group0_.y * other_groups.group0_.x), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group0_.x * other_groups.group1_.x, self_.group0_.x * other_groups.group1_.y, self_.group0_.x * other_groups.group1_.z, (self_.group0_.x * other_groups.group1_.w) - (self_.group0_.y * other_groups.group4_.w)), 
        /* e41, e42, e43 */ (vec4<f32>(vec3<f32>(self_.group0_.x), 0.0) * other_groups.group2_) + (vec4<f32>(vec3<f32>(self_.group0_.y), 0.0) * other_groups.group3_), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(self_.group0_.x), 0.0) * other_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.group0_.x * other_groups.group4_.x) - (self_.group0_.y * other_groups.group1_.x), (self_.group0_.x * other_groups.group4_.y) - (self_.group0_.y * other_groups.group1_.y), (self_.group0_.x * other_groups.group4_.z) - (self_.group0_.y * other_groups.group1_.z), self_.group0_.x * other_groups.group4_.w)
    );
    let geometric_product: MultiVector = multiVector_degroup(geometric_product_groups);
    return multiVector_geometricProduct_dualNum(geometric_product, dualNum_reverse(self));
}
fn dualNum_sandwich_origin(self_: DualNum, other: Origin) -> Origin {
    let self_groups = dualNum_grouped(self_);
    let other_groups = origin_grouped(other);
    let geometric_product_groups: OriginGroups = OriginGroups(
        /* e4 */ vec4<f32>(self_.group0_.x * other.e4, 0.0, 0.0, 0.0)
    );
    let geometric_product: Origin = origin_degroup(geometric_product_groups);
    return origin_geometricProduct_dualNum(geometric_product, dualNum_reverse(self));
}
fn dualNum_sandwich_plane(self_: DualNum, other: Plane) -> Flector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = plane_grouped(other);
    let geometric_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.group0_.y * other_groups.group0_.w * -1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.x) * other_groups.group0_
    );
    let geometric_product: Flector = flector_degroup(geometric_product_groups);
    return flector_geometricProduct_dualNum(geometric_product, dualNum_reverse(self));
}
fn dualNum_sandwich_point(self_: DualNum, other: Point) -> Flector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = point_grouped(other);
    let geometric_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group0_.x) * other_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.y * other_groups.group0_.x * -1.0, self_.group0_.y * other_groups.group0_.y * -1.0, self_.group0_.y * other_groups.group0_.z * -1.0, 0.0)
    );
    let geometric_product: Flector = flector_degroup(geometric_product_groups);
    return flector_geometricProduct_dualNum(geometric_product, dualNum_reverse(self));
}
fn dualNum_sandwich_scalar(self_: DualNum, other: Scalar) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let other_groups = scalar_grouped(other);
    let geometric_product_groups: DualNumGroups = DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(other.scalar), 0.0, 0.0) * self_.group0_
    );
    let geometric_product: DualNum = dualNum_degroup(geometric_product_groups);
    return dualNum_geometricProduct_dualNum(geometric_product, dualNum_reverse(self));
}
fn flector_sandwich_antiScalar(self_: Flector, other: AntiScalar) -> Motor {
    let self_groups = flector_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    let geometric_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.group1_.w * other.e1234), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.x * other.e1234, self_.group0_.y * other.e1234, self_.group0_.z * other.e1234, 0.0)
    );
    let geometric_product: Flector = flector_degroup(geometric_product_groups);
    return flector_geometricProduct_flector(geometric_product, flector_reverse(self));
}
fn flector_sandwich_dualNum(self_: Flector, other: DualNum) -> Motor {
    let self_groups = flector_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let geometric_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group0_.x * self_.group0_.x, other_groups.group0_.x * self_.group0_.y, other_groups.group0_.x * self_.group0_.z, (other_groups.group0_.x * self_.group0_.w) + (other_groups.group0_.y * self_.group1_.w)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.x * self_.group1_.x) + (other_groups.group0_.y * self_.group0_.x), (other_groups.group0_.x * self_.group1_.y) + (other_groups.group0_.y * self_.group0_.y), (other_groups.group0_.x * self_.group1_.z) + (other_groups.group0_.y * self_.group0_.z), other_groups.group0_.x * self_.group1_.w)
    );
    let geometric_product: Flector = flector_degroup(geometric_product_groups);
    return flector_geometricProduct_flector(geometric_product, flector_reverse(self));
}
fn flector_sandwich_flector(self_: Flector, other: Flector) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = flector_grouped(other);
    let geometric_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other_groups.group0_.y * self_.group1_.z) - (other_groups.group1_.y * self_.group0_.z), (other_groups.group0_.z * self_.group1_.x) - (other_groups.group1_.z * self_.group0_.x), (other_groups.group0_.x * self_.group1_.y) - (other_groups.group1_.x * self_.group0_.y), (other_groups.group1_.w * self_.group0_.w) - (other_groups.group0_.w * self_.group1_.w)) + (vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, other_groups.group1_.x) * self_.group0_.wwwx) + (vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group0_.z) * other_groups.group1_.wwwz) + (other_groups.group1_.zxyy * self_.group0_.yzxy) - (vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, other_groups.group0_.z) * self_.group1_.wwwz) - (vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group1_.y) * other_groups.group0_.wwwy) - (other_groups.group0_.zxyx * self_.group1_.yzxx), 
        /* e23, e31, e12, scalar */ vec4<f32>(-(other_groups.group0_.y * self_.group0_.z) - (other_groups.group1_.w * self_.group0_.x), -(other_groups.group0_.z * self_.group0_.x) - (other_groups.group1_.w * self_.group0_.y), -(other_groups.group0_.x * self_.group0_.y) - (other_groups.group1_.w * self_.group0_.z), (other_groups.group0_.y * self_.group0_.y) + (other_groups.group0_.z * self_.group0_.z)) + (other_groups.group0_.zxyx * self_.group0_.yzxx) - (vec4<f32>(self_.group1_.w) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, other_groups.group1_.w))
    );
    let geometric_product: Motor = motor_degroup(geometric_product_groups);
    return motor_geometricProduct_flector(geometric_product, flector_reverse(self));
}
fn flector_sandwich_horizon(self_: Flector, other: Horizon) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = horizon_grouped(other);
    let geometric_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e321) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group0_.w), 
        /* e23, e31, e12, scalar */ vec4<f32>(other.e321) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group1_.w) * vec4<f32>(-1.0)
    );
    let geometric_product: Motor = motor_degroup(geometric_product_groups);
    return motor_geometricProduct_flector(geometric_product, flector_reverse(self));
}
fn flector_sandwich_line(self_: Flector, other: Line) -> Motor {
    let self_groups = flector_grouped(self_);
    let other_groups = line_grouped(other);
    let geometric_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group1_.x * self_.group1_.w) + (other_groups.group1_.y * self_.group0_.z), (other_groups.group1_.y * self_.group1_.w) + (other_groups.group1_.z * self_.group0_.x), (other_groups.group1_.x * self_.group0_.y) + (other_groups.group1_.z * self_.group1_.w), -(other_groups.group0_.y * self_.group0_.y) - (other_groups.group0_.z * self_.group0_.z) - (other_groups.group1_.x * self_.group1_.x) - (other_groups.group1_.y * self_.group1_.y) - (other_groups.group1_.z * self_.group1_.z)) - (vec4<f32>(other_groups.group1_.zxy.xyz, other_groups.group0_.x) * self_.group0_.yzxx), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.x * self_.group1_.w) + (other_groups.group0_.y * self_.group0_.z) + (other_groups.group1_.x * self_.group0_.w) + (other_groups.group1_.y * self_.group1_.z) - (other_groups.group1_.z * self_.group1_.y), (other_groups.group0_.y * self_.group1_.w) + (other_groups.group0_.z * self_.group0_.x) + (other_groups.group1_.y * self_.group0_.w) + (other_groups.group1_.z * self_.group1_.x) - (other_groups.group1_.x * self_.group1_.z), (other_groups.group0_.x * self_.group0_.y) + (other_groups.group0_.z * self_.group1_.w) + (other_groups.group1_.x * self_.group1_.y) + (other_groups.group1_.z * self_.group0_.w) - (other_groups.group1_.y * self_.group1_.x), -(other_groups.group1_.y * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.z)) - (vec4<f32>(other_groups.group0_.zxy.xyz, other_groups.group1_.x) * self_.group0_.yzxx)
    );
    let geometric_product: Flector = flector_degroup(geometric_product_groups);
    return flector_geometricProduct_flector(geometric_product, flector_reverse(self));
}
fn flector_sandwich_motor(self_: Flector, other: Motor) -> Motor {
    let self_groups = flector_grouped(self_);
    let other_groups = motor_grouped(other);
    let geometric_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group0_.z * other_groups.group1_.y, self_.group0_.y * other_groups.group1_.w, self_.group0_.z * other_groups.group1_.w, -(self_.group0_.y * other_groups.group0_.y) - (self_.group0_.z * other_groups.group0_.z) - (self_.group1_.x * other_groups.group1_.x) - (self_.group1_.y * other_groups.group1_.y) - (self_.group1_.z * other_groups.group1_.z)) + (vec4<f32>(self_.group1_.w) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, other_groups.group0_.w)) + (self_.group0_.xxyw * other_groups.group1_.wzxw) - (vec4<f32>(other_groups.group1_.z, other_groups.group1_.x, other_groups.group1_.y, other_groups.group0_.x) * self_.group0_.yzxx), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.group0_.x * other_groups.group0_.w) + (self_.group0_.z * other_groups.group0_.y) + (self_.group1_.x * other_groups.group1_.w) + (self_.group1_.z * other_groups.group1_.y) + (self_.group1_.w * other_groups.group0_.x), (self_.group0_.x * other_groups.group0_.z) + (self_.group0_.y * other_groups.group0_.w) + (self_.group1_.x * other_groups.group1_.z) + (self_.group1_.y * other_groups.group1_.w) + (self_.group1_.w * other_groups.group0_.y), (self_.group0_.y * other_groups.group0_.x) + (self_.group0_.z * other_groups.group0_.w) + (self_.group1_.y * other_groups.group1_.x) + (self_.group1_.z * other_groups.group1_.w) + (self_.group1_.w * other_groups.group0_.z), (self_.group0_.z * other_groups.group1_.z) * -1.0) + (vec4<f32>(self_.group0_.w, self_.group0_.w, self_.group0_.w, self_.group1_.w) * other_groups.group1_) - (vec4<f32>(self_.group1_.y, self_.group1_.z, self_.group1_.x, self_.group0_.y) * other_groups.group1_.zxyy) - (vec4<f32>(other_groups.group0_.z, other_groups.group0_.x, other_groups.group0_.y, other_groups.group1_.x) * self_.group0_.yzxx)
    );
    let geometric_product: Flector = flector_degroup(geometric_product_groups);
    return flector_geometricProduct_flector(geometric_product, flector_reverse(self));
}
fn flector_sandwich_multiVector(self_: Flector, other: MultiVector) -> MultiVector {
    let self_groups = flector_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let geometric_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, (self_.group0_.w * other_groups.group4_.w) - (self_.group1_.x * other_groups.group1_.x) - (self_.group1_.y * other_groups.group1_.y) - (self_.group1_.z * other_groups.group1_.z), 0.0, 0.0) + (vec4<f32>(vec2<f32>(self_.group0_.x), 0.0, 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group4_.x, 0.0, 0.0)) + (vec4<f32>(vec2<f32>(self_.group0_.y), 0.0, 0.0) * vec4<f32>(other_groups.group1_.y, other_groups.group4_.y, 0.0, 0.0)) + (vec4<f32>(vec2<f32>(self_.group0_.z), 0.0, 0.0) * vec4<f32>(other_groups.group1_.z, other_groups.group4_.z, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(self_.group1_.w), 0.0, 0.0) * vec4<f32>(other_groups.group4_.w, other_groups.group1_.w, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group3_.y * self_.group0_.z, other_groups.group3_.z * self_.group0_.x, other_groups.group3_.x * self_.group0_.y, -(other_groups.group2_.y * self_.group0_.y) - (other_groups.group2_.z * self_.group0_.z) - (other_groups.group3_.x * self_.group1_.x) - (other_groups.group3_.y * self_.group1_.y) - (other_groups.group3_.z * self_.group1_.z)) + (vec4<f32>(other_groups.group0_.x) * self_.group0_) + (vec4<f32>(self_.group1_.w) * vec4<f32>(other_groups.group3_.xyz.xyz, other_groups.group0_.y)) - (vec4<f32>(other_groups.group3_.zxy.xyz, other_groups.group2_.x) * self_.group0_.yzxx), 
        /* e41, e42, e43 */ vec4<f32>((self_.group0_.y * other_groups.group4_.z) + (self_.group1_.z * other_groups.group1_.y) - (self_.group0_.z * other_groups.group4_.y) - (self_.group1_.y * other_groups.group1_.z), (self_.group0_.z * other_groups.group4_.x) + (self_.group1_.x * other_groups.group1_.z) - (self_.group0_.x * other_groups.group4_.z) - (self_.group1_.z * other_groups.group1_.x), (self_.group0_.x * other_groups.group4_.y) + (self_.group1_.y * other_groups.group1_.x) - (self_.group0_.y * other_groups.group4_.x) - (self_.group1_.x * other_groups.group1_.y), 0.0) + (vec4<f32>(vec3<f32>(self_.group0_.w), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0)) + (vec4<f32>(vec3<f32>(other_groups.group4_.w), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0)) - (vec4<f32>(vec3<f32>(self_.group1_.w), 0.0) * vec4<f32>(other_groups.group4_.x, other_groups.group4_.y, other_groups.group4_.z, 0.0)) - (vec4<f32>(vec3<f32>(other_groups.group1_.w), 0.0) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((self_.group0_.y * other_groups.group1_.z) - (self_.group0_.z * other_groups.group1_.y), (self_.group0_.z * other_groups.group1_.x) - (self_.group0_.x * other_groups.group1_.z), (self_.group0_.x * other_groups.group1_.y) - (self_.group0_.y * other_groups.group1_.x), 0.0) - (vec4<f32>(vec3<f32>(self_.group1_.w), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0)) - (vec4<f32>(vec3<f32>(other_groups.group4_.w), 0.0) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.y * self_.group0_.x) + (other_groups.group2_.x * self_.group1_.w) + (other_groups.group2_.y * self_.group0_.z) + (other_groups.group3_.x * self_.group0_.w) + (other_groups.group3_.y * self_.group1_.z) - (other_groups.group3_.z * self_.group1_.y), (other_groups.group0_.y * self_.group0_.y) + (other_groups.group2_.y * self_.group1_.w) + (other_groups.group2_.z * self_.group0_.x) + (other_groups.group3_.y * self_.group0_.w) + (other_groups.group3_.z * self_.group1_.x) - (other_groups.group3_.x * self_.group1_.z), (other_groups.group0_.y * self_.group0_.z) + (other_groups.group2_.x * self_.group0_.y) + (other_groups.group2_.z * self_.group1_.w) + (other_groups.group3_.x * self_.group1_.y) + (other_groups.group3_.z * self_.group0_.w) - (other_groups.group3_.y * self_.group1_.x), -(other_groups.group3_.y * self_.group0_.y) - (other_groups.group3_.z * self_.group0_.z)) + (vec4<f32>(other_groups.group0_.x) * self_.group1_) - (vec4<f32>(other_groups.group2_.zxy.xyz, other_groups.group3_.x) * self_.group0_.yzxx)
    );
    let geometric_product: MultiVector = multiVector_degroup(geometric_product_groups);
    return multiVector_geometricProduct_flector(geometric_product, flector_reverse(self));
}
fn flector_sandwich_origin(self_: Flector, other: Origin) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = origin_grouped(other);
    let geometric_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e4) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group1_.w) * vec4<f32>(-1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0)
    );
    let geometric_product: Motor = motor_degroup(geometric_product_groups);
    return motor_geometricProduct_flector(geometric_product, flector_reverse(self));
}
fn flector_sandwich_plane(self_: Flector, other: Plane) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = plane_grouped(other);
    let geometric_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(-(self_.group0_.z * other_groups.group0_.y) - (self_.group1_.w * other_groups.group0_.x), -(self_.group0_.x * other_groups.group0_.z) - (self_.group1_.w * other_groups.group0_.y), -(self_.group0_.y * other_groups.group0_.x) - (self_.group1_.w * other_groups.group0_.z), (self_.group0_.z * other_groups.group0_.z) + (self_.group0_.w * other_groups.group0_.w)) + (vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group0_.y) * other_groups.group0_.wwwy) + (self_.group0_.yzxx * other_groups.group0_.zxyx), 
        /* e23, e31, e12, scalar */ vec4<f32>(other_groups.group0_.w) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group1_.w) * vec4<f32>(-1.0)
    );
    let geometric_product: Motor = motor_degroup(geometric_product_groups);
    return motor_geometricProduct_flector(geometric_product, flector_reverse(self));
}
fn flector_sandwich_point(self_: Flector, other: Point) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = point_grouped(other);
    let geometric_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((self_.group0_.w * other_groups.group0_.x) + (self_.group1_.z * other_groups.group0_.y), (self_.group0_.w * other_groups.group0_.y) + (self_.group1_.x * other_groups.group0_.z), (self_.group0_.w * other_groups.group0_.z) + (self_.group1_.y * other_groups.group0_.x), -(self_.group1_.z * other_groups.group0_.z) - (self_.group1_.w * other_groups.group0_.w)) - (vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group1_.x) * other_groups.group0_.wwwx) - (self_.group1_.yzxy * other_groups.group0_.zxyy), 
        /* e23, e31, e12, scalar */ vec4<f32>(-(self_.group0_.z * other_groups.group0_.y) - (self_.group1_.w * other_groups.group0_.x), -(self_.group0_.x * other_groups.group0_.z) - (self_.group1_.w * other_groups.group0_.y), -(self_.group0_.y * other_groups.group0_.x) - (self_.group1_.w * other_groups.group0_.z), (self_.group0_.y * other_groups.group0_.y) + (self_.group0_.z * other_groups.group0_.z)) + (self_.group0_.yzxx * other_groups.group0_.zxyx)
    );
    let geometric_product: Motor = motor_degroup(geometric_product_groups);
    return motor_geometricProduct_flector(geometric_product, flector_reverse(self));
}
fn flector_sandwich_scalar(self_: Flector, other: Scalar) -> Motor {
    let self_groups = flector_grouped(self_);
    let other_groups = scalar_grouped(other);
    let geometric_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.scalar) * self_.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other.scalar) * self_.group1_
    );
    let geometric_product: Flector = flector_degroup(geometric_product_groups);
    return flector_geometricProduct_flector(geometric_product, flector_reverse(self));
}
fn horizon_sandwich_antiScalar(self_: Horizon, other: AntiScalar) -> AntiScalar {
    let self_groups = horizon_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    let geometric_product_groups: OriginGroups = OriginGroups(
        /* e4 */ vec4<f32>(other.e1234 * self.e321, 0.0, 0.0, 0.0)
    );
    let geometric_product: Origin = origin_degroup(geometric_product_groups);
    return origin_geometricProduct_horizon(geometric_product, horizon_reverse(self));
}
fn horizon_sandwich_dualNum(self_: Horizon, other: DualNum) -> Motor {
    let self_groups = horizon_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let geometric_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group0_.y * self.e321), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group0_.x * self.e321)
    );
    let geometric_product: Flector = flector_degroup(geometric_product_groups);
    return flector_geometricProduct_horizon(geometric_product, horizon_reverse(self));
}
fn horizon_sandwich_flector(self_: Horizon, other: Flector) -> Flector {
    let self_groups = horizon_grouped(self_);
    let other_groups = flector_grouped(other);
    let geometric_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self.e321) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, other_groups.group0_.w) * vec4<f32>(-1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(self.e321) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, other_groups.group1_.w) * vec4<f32>(-1.0)
    );
    let geometric_product: Motor = motor_degroup(geometric_product_groups);
    return motor_geometricProduct_horizon(geometric_product, horizon_reverse(self));
}
fn horizon_sandwich_horizon(self_: Horizon, other: Horizon) -> Horizon {
    let self_groups = horizon_grouped(self_);
    let other_groups = horizon_grouped(other);
    let geometric_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(other.e321 * self.e321 * -1.0, 0.0, 0.0, 0.0)
    );
    let geometric_product: Scalar = scalar_degroup(geometric_product_groups);
    return scalar_geometricProduct_horizon(geometric_product, horizon_reverse(self));
}
fn horizon_sandwich_line(self_: Horizon, other: Line) -> Motor {
    let self_groups = horizon_grouped(self_);
    let other_groups = line_grouped(other);
    let geometric_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group1_.x * self.e321, other_groups.group1_.y * self.e321, other_groups.group1_.z * self.e321, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group0_.x * self.e321, other_groups.group0_.y * self.e321, other_groups.group0_.z * self.e321, 0.0)
    );
    let geometric_product: Flector = flector_degroup(geometric_product_groups);
    return flector_geometricProduct_horizon(geometric_product, horizon_reverse(self));
}
fn horizon_sandwich_motor(self_: Horizon, other: Motor) -> Motor {
    let self_groups = horizon_grouped(self_);
    let other_groups = motor_grouped(other);
    let geometric_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self.e321) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, other_groups.group0_.w), 
        /* e423, e431, e412, e321 */ vec4<f32>(self.e321) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, other_groups.group1_.w)
    );
    let geometric_product: Flector = flector_degroup(geometric_product_groups);
    return flector_geometricProduct_horizon(geometric_product, horizon_reverse(self));
}
fn horizon_sandwich_multiVector(self_: Horizon, other: MultiVector) -> MultiVector {
    let self_groups = horizon_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let geometric_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(self.e321), 0.0, 0.0) * vec4<f32>(other_groups.group4_.w, other_groups.group1_.w, 0.0, 0.0) * vec2<f32>(-1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self.e321) * vec4<f32>(other_groups.group3_.xyz.xyz, other_groups.group0_.y), 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(self.e321), 0.0) * vec4<f32>(other_groups.group4_.x, other_groups.group4_.y, other_groups.group4_.z, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(self.e321), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0) * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self.e321) * vec4<f32>(other_groups.group2_.xyz.xyz, other_groups.group0_.x)
    );
    let geometric_product: MultiVector = multiVector_degroup(geometric_product_groups);
    return multiVector_geometricProduct_horizon(geometric_product, horizon_reverse(self));
}
fn horizon_sandwich_origin(self_: Horizon, other: Origin) -> Origin {
    let self_groups = horizon_grouped(self_);
    let other_groups = origin_grouped(other);
    let geometric_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(self.e321 * other.e4 * -1.0, 0.0, 0.0, 0.0)
    );
    let geometric_product: AntiScalar = antiScalar_degroup(geometric_product_groups);
    return antiScalar_geometricProduct_horizon(geometric_product, horizon_reverse(self));
}
fn horizon_sandwich_plane(self_: Horizon, other: Plane) -> Flector {
    let self_groups = horizon_grouped(self_);
    let other_groups = plane_grouped(other);
    let geometric_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other_groups.group0_.x * self.e321 * -1.0, other_groups.group0_.y * self.e321 * -1.0, other_groups.group0_.z * self.e321 * -1.0, 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group0_.w * self.e321 * -1.0)
    );
    let geometric_product: Motor = motor_degroup(geometric_product_groups);
    return motor_geometricProduct_horizon(geometric_product, horizon_reverse(self));
}
fn horizon_sandwich_point(self_: Horizon, other: Point) -> Flector {
    let self_groups = horizon_grouped(self_);
    let other_groups = point_grouped(other);
    let geometric_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group0_.w * self.e321 * -1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(other_groups.group0_.x * self.e321 * -1.0, other_groups.group0_.y * self.e321 * -1.0, other_groups.group0_.z * self.e321 * -1.0, 0.0)
    );
    let geometric_product: Motor = motor_degroup(geometric_product_groups);
    return motor_geometricProduct_horizon(geometric_product, horizon_reverse(self));
}
fn horizon_sandwich_scalar(self_: Horizon, other: Scalar) -> Scalar {
    let self_groups = horizon_grouped(self_);
    let other_groups = scalar_grouped(other);
    let geometric_product_groups: HorizonGroups = HorizonGroups(
        /* e321 */ vec4<f32>(self.e321 * other.scalar, 0.0, 0.0, 0.0)
    );
    let geometric_product: Horizon = horizon_degroup(geometric_product_groups);
    return horizon_geometricProduct_horizon(geometric_product, horizon_reverse(self));
}
fn line_sandwich_antiScalar(self_: Line, other: AntiScalar) -> Motor {
    let self_groups = line_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    let geometric_product_groups: LineGroups = LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(other.e1234), 0.0) * self_.group1_, 
        /* e23, e31, e12 */ vec4<f32>(0.0)
    );
    let geometric_product: Line = line_degroup(geometric_product_groups);
    return line_geometricProduct_line(geometric_product, line_reverse(self));
}
fn line_sandwich_dualNum(self_: Line, other: DualNum) -> Motor {
    let self_groups = line_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let geometric_product_groups: LineGroups = LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(vec3<f32>(other_groups.group0_.x), 0.0) * self_.group0_) + (vec4<f32>(vec3<f32>(other_groups.group0_.y), 0.0) * self_.group1_), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(other_groups.group0_.x), 0.0) * self_.group1_
    );
    let geometric_product: Line = line_degroup(geometric_product_groups);
    return line_geometricProduct_line(geometric_product, line_reverse(self));
}
fn line_sandwich_flector(self_: Line, other: Flector) -> Flector {
    let self_groups = line_grouped(self_);
    let other_groups = flector_grouped(other);
    let geometric_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group1_.x * other_groups.group1_.w) - (self_.group1_.y * other_groups.group0_.z), (self_.group1_.y * other_groups.group1_.w) - (self_.group1_.z * other_groups.group0_.x), (self_.group1_.z * other_groups.group1_.w) - (self_.group1_.x * other_groups.group0_.y), (self_.group0_.y * other_groups.group0_.y) + (self_.group0_.z * other_groups.group0_.z) - (self_.group1_.x * other_groups.group1_.x) - (self_.group1_.y * other_groups.group1_.y) - (self_.group1_.z * other_groups.group1_.z)) + (vec4<f32>(self_.group1_.zxy.xyz, self_.group0_.x) * other_groups.group0_.yzxx), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.group0_.y * other_groups.group0_.z) + (self_.group1_.x * other_groups.group0_.w) + (self_.group1_.z * other_groups.group1_.y) - (self_.group0_.x * other_groups.group1_.w) - (self_.group1_.y * other_groups.group1_.z), (self_.group0_.z * other_groups.group0_.x) + (self_.group1_.x * other_groups.group1_.z) + (self_.group1_.y * other_groups.group0_.w) - (self_.group0_.y * other_groups.group1_.w) - (self_.group1_.z * other_groups.group1_.x), (self_.group0_.x * other_groups.group0_.y) + (self_.group1_.y * other_groups.group1_.x) + (self_.group1_.z * other_groups.group0_.w) - (self_.group0_.z * other_groups.group1_.w) - (self_.group1_.x * other_groups.group1_.y), -(self_.group1_.y * other_groups.group0_.y) - (self_.group1_.z * other_groups.group0_.z)) - (vec4<f32>(self_.group0_.zxy.xyz, self_.group1_.x) * other_groups.group0_.yzxx)
    );
    let geometric_product: Flector = flector_degroup(geometric_product_groups);
    return flector_geometricProduct_line(geometric_product, line_reverse(self));
}
fn line_sandwich_horizon(self_: Line, other: Horizon) -> Flector {
    let self_groups = line_grouped(self_);
    let other_groups = horizon_grouped(other);
    let geometric_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group1_.x * other.e321, self_.group1_.y * other.e321, self_.group1_.z * other.e321, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.x * other.e321 * -1.0, self_.group0_.y * other.e321 * -1.0, self_.group0_.z * other.e321 * -1.0, 0.0)
    );
    let geometric_product: Flector = flector_degroup(geometric_product_groups);
    return flector_geometricProduct_line(geometric_product, line_reverse(self));
}
fn line_sandwich_line(self_: Line, other: Line) -> Motor {
    let self_groups = line_grouped(self_);
    let other_groups = line_grouped(other);
    let geometric_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other_groups.group0_.y * self_.group1_.z) + (other_groups.group1_.y * self_.group0_.z) - (other_groups.group0_.z * self_.group1_.y) - (other_groups.group1_.z * self_.group0_.y), (other_groups.group0_.z * self_.group1_.x) + (other_groups.group1_.z * self_.group0_.x) - (other_groups.group0_.x * self_.group1_.z) - (other_groups.group1_.x * self_.group0_.z), (other_groups.group0_.x * self_.group1_.y) + (other_groups.group1_.x * self_.group0_.y) - (other_groups.group0_.y * self_.group1_.x) - (other_groups.group1_.y * self_.group0_.x), -(other_groups.group0_.x * self_.group1_.x) - (other_groups.group0_.y * self_.group1_.y) - (other_groups.group0_.z * self_.group1_.z) - (other_groups.group1_.x * self_.group0_.x) - (other_groups.group1_.y * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.z)), 
        /* e23, e31, e12, scalar */ vec4<f32>((other_groups.group1_.y * self_.group1_.z) - (other_groups.group1_.z * self_.group1_.y), (other_groups.group1_.z * self_.group1_.x) - (other_groups.group1_.x * self_.group1_.z), (other_groups.group1_.x * self_.group1_.y) - (other_groups.group1_.y * self_.group1_.x), -(other_groups.group1_.x * self_.group1_.x) - (other_groups.group1_.y * self_.group1_.y) - (other_groups.group1_.z * self_.group1_.z))
    );
    let geometric_product: Motor = motor_degroup(geometric_product_groups);
    return motor_geometricProduct_line(geometric_product, line_reverse(self));
}
fn line_sandwich_motor(self_: Line, other: Motor) -> Motor {
    let self_groups = line_grouped(self_);
    let other_groups = motor_grouped(other);
    let geometric_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((self_.group0_.x * other_groups.group1_.w) + (self_.group0_.z * other_groups.group1_.y) + (self_.group1_.x * other_groups.group0_.w) + (self_.group1_.z * other_groups.group0_.y), (self_.group0_.x * other_groups.group1_.z) + (self_.group0_.y * other_groups.group1_.w) + (self_.group1_.x * other_groups.group0_.z) + (self_.group1_.y * other_groups.group0_.w), (self_.group0_.y * other_groups.group1_.x) + (self_.group0_.z * other_groups.group1_.w) + (self_.group1_.y * other_groups.group0_.x) + (self_.group1_.z * other_groups.group0_.w), -(self_.group0_.y * other_groups.group1_.y) - (self_.group0_.z * other_groups.group1_.z) - (self_.group1_.y * other_groups.group0_.y) - (self_.group1_.z * other_groups.group0_.z)) - (vec4<f32>(self_.group0_.yzx.xyz, self_.group0_.x) * other_groups.group1_.zxyx) - (vec4<f32>(self_.group1_.yzx.xyz, self_.group1_.x) * other_groups.group0_.zxyx), 
        /* e23, e31, e12, scalar */ vec4<f32>((self_.group1_.x * other_groups.group1_.w) + (self_.group1_.z * other_groups.group1_.y), (self_.group1_.x * other_groups.group1_.z) + (self_.group1_.y * other_groups.group1_.w), (self_.group1_.y * other_groups.group1_.x) + (self_.group1_.z * other_groups.group1_.w), -(self_.group1_.y * other_groups.group1_.y) - (self_.group1_.z * other_groups.group1_.z)) - (vec4<f32>(self_.group1_.yzx.xyz, self_.group1_.x) * other_groups.group1_.zxyx)
    );
    let geometric_product: Motor = motor_degroup(geometric_product_groups);
    return motor_geometricProduct_line(geometric_product, line_reverse(self));
}
fn line_sandwich_multiVector(self_: Line, other: MultiVector) -> MultiVector {
    let self_groups = line_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let geometric_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, -(self_.group1_.x * other_groups.group2_.x) - (self_.group1_.y * other_groups.group2_.y) - (self_.group1_.z * other_groups.group2_.z), 0.0, 0.0) - (vec4<f32>(vec2<f32>(other_groups.group3_.x), 0.0, 0.0) * vec4<f32>(self_.group1_.x, self_.group0_.x, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(other_groups.group3_.y), 0.0, 0.0) * vec4<f32>(self_.group1_.y, self_.group0_.y, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(other_groups.group3_.z), 0.0, 0.0) * vec4<f32>(self_.group1_.z, self_.group0_.z, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group1_.x * other_groups.group4_.w) - (self_.group1_.y * other_groups.group1_.z), (self_.group1_.y * other_groups.group4_.w) - (self_.group1_.z * other_groups.group1_.x), (self_.group1_.z * other_groups.group4_.w) - (self_.group1_.x * other_groups.group1_.y), (self_.group0_.y * other_groups.group1_.y) + (self_.group0_.z * other_groups.group1_.z) - (self_.group1_.x * other_groups.group4_.x) - (self_.group1_.y * other_groups.group4_.y) - (self_.group1_.z * other_groups.group4_.z)) + (vec4<f32>(self_.group1_.zxy.xyz, self_.group0_.x) * other_groups.group1_.yzxx), 
        /* e41, e42, e43 */ (vec4<f32>(vec3<f32>(other_groups.group0_.x), 0.0) * self_.group0_) + (vec4<f32>(vec3<f32>(other_groups.group0_.y), 0.0) * self_.group1_) + (self_.group0_.zxy * other_groups.group3_.yzx) + (self_.group1_.zxy * other_groups.group2_.yzx) - (self_.group0_.yzx * other_groups.group3_.zxy) - (self_.group1_.yzx * other_groups.group2_.zxy), 
        /* e23, e31, e12 */ (vec4<f32>(vec3<f32>(other_groups.group0_.x), 0.0) * self_.group1_) + (self_.group1_.zxy * other_groups.group3_.yzx) - (self_.group1_.yzx * other_groups.group3_.zxy), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.group0_.y * other_groups.group1_.z) + (self_.group1_.x * other_groups.group1_.w) + (self_.group1_.z * other_groups.group4_.y) - (self_.group0_.x * other_groups.group4_.w) - (self_.group1_.y * other_groups.group4_.z), (self_.group0_.z * other_groups.group1_.x) + (self_.group1_.x * other_groups.group4_.z) + (self_.group1_.y * other_groups.group1_.w) - (self_.group0_.y * other_groups.group4_.w) - (self_.group1_.z * other_groups.group4_.x), (self_.group0_.x * other_groups.group1_.y) + (self_.group1_.y * other_groups.group4_.x) + (self_.group1_.z * other_groups.group1_.w) - (self_.group0_.z * other_groups.group4_.w) - (self_.group1_.x * other_groups.group4_.y), -(self_.group1_.y * other_groups.group1_.y) - (self_.group1_.z * other_groups.group1_.z)) - (vec4<f32>(self_.group0_.zxy.xyz, self_.group1_.x) * other_groups.group1_.yzxx)
    );
    let geometric_product: MultiVector = multiVector_degroup(geometric_product_groups);
    return multiVector_geometricProduct_line(geometric_product, line_reverse(self));
}
fn line_sandwich_origin(self_: Line, other: Origin) -> Flector {
    let self_groups = line_grouped(self_);
    let other_groups = origin_grouped(other);
    let geometric_product_groups: PlaneGroups = PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group1_.x * other.e4, self_.group1_.y * other.e4, self_.group1_.z * other.e4, 0.0)
    );
    let geometric_product: Plane = plane_degroup(geometric_product_groups);
    return plane_geometricProduct_line(geometric_product, line_reverse(self));
}
fn line_sandwich_plane(self_: Line, other: Plane) -> Flector {
    let self_groups = line_grouped(self_);
    let other_groups = plane_grouped(other);
    let geometric_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group1_.x * other_groups.group0_.w, self_.group1_.y * other_groups.group0_.w, self_.group1_.z * other_groups.group0_.w, -(self_.group1_.x * other_groups.group0_.x) - (self_.group1_.y * other_groups.group0_.y) - (self_.group1_.z * other_groups.group0_.z)), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.group1_.z * other_groups.group0_.y) - (self_.group0_.x * other_groups.group0_.w) - (self_.group1_.y * other_groups.group0_.z), (self_.group1_.x * other_groups.group0_.z) - (self_.group0_.y * other_groups.group0_.w) - (self_.group1_.z * other_groups.group0_.x), (self_.group1_.y * other_groups.group0_.x) - (self_.group0_.z * other_groups.group0_.w) - (self_.group1_.x * other_groups.group0_.y), 0.0)
    );
    let geometric_product: Flector = flector_degroup(geometric_product_groups);
    return flector_geometricProduct_line(geometric_product, line_reverse(self));
}
fn line_sandwich_point(self_: Line, other: Point) -> Flector {
    let self_groups = line_grouped(self_);
    let other_groups = point_grouped(other);
    let geometric_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group1_.y * other_groups.group0_.z) * -1.0, (self_.group1_.z * other_groups.group0_.x) * -1.0, (self_.group1_.x * other_groups.group0_.y) * -1.0, (self_.group0_.y * other_groups.group0_.y) + (self_.group0_.z * other_groups.group0_.z)) + (vec4<f32>(self_.group1_.zxy.xyz, self_.group0_.x) * other_groups.group0_.yzxx), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.group0_.y * other_groups.group0_.z) + (self_.group1_.x * other_groups.group0_.w), (self_.group0_.z * other_groups.group0_.x) + (self_.group1_.y * other_groups.group0_.w), (self_.group0_.x * other_groups.group0_.y) + (self_.group1_.z * other_groups.group0_.w), -(self_.group1_.y * other_groups.group0_.y) - (self_.group1_.z * other_groups.group0_.z)) - (vec4<f32>(self_.group0_.zxy.xyz, self_.group1_.x) * other_groups.group0_.yzxx)
    );
    let geometric_product: Flector = flector_degroup(geometric_product_groups);
    return flector_geometricProduct_line(geometric_product, line_reverse(self));
}
fn line_sandwich_scalar(self_: Line, other: Scalar) -> Motor {
    let self_groups = line_grouped(self_);
    let other_groups = scalar_grouped(other);
    let geometric_product_groups: LineGroups = LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(other.scalar), 0.0) * self_.group0_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(other.scalar), 0.0) * self_.group1_
    );
    let geometric_product: Line = line_degroup(geometric_product_groups);
    return line_geometricProduct_line(geometric_product, line_reverse(self));
}
fn motor_sandwich_antiScalar(self_: Motor, other: AntiScalar) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    let geometric_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e1234) * self_.group1_, 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0)
    );
    let geometric_product: Motor = motor_degroup(geometric_product_groups);
    return motor_geometricProduct_motor(geometric_product, motor_reverse(self));
}
fn motor_sandwich_dualNum(self_: Motor, other: DualNum) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let geometric_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ (vec4<f32>(other_groups.group0_.x) * self_.group0_) + (vec4<f32>(other_groups.group0_.y) * self_.group1_), 
        /* e23, e31, e12, scalar */ vec4<f32>(other_groups.group0_.x) * self_.group1_
    );
    let geometric_product: Motor = motor_degroup(geometric_product_groups);
    return motor_geometricProduct_motor(geometric_product, motor_reverse(self));
}
fn motor_sandwich_flector(self_: Motor, other: Flector) -> Flector {
    let self_groups = motor_grouped(self_);
    let other_groups = flector_grouped(other);
    let geometric_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, (other_groups.group0_.z * self_.group0_.z) - (other_groups.group1_.y * self_.group1_.y) - (other_groups.group1_.z * self_.group1_.z) - (other_groups.group1_.w * self_.group0_.w)) + (vec4<f32>(other_groups.group1_.w, other_groups.group1_.w, other_groups.group1_.w, other_groups.group0_.w) * self_.group1_) + (vec4<f32>(self_.group1_.z, self_.group1_.x, self_.group1_.w, self_.group0_.y) * other_groups.group0_.yzzy) + (vec4<f32>(self_.group1_.w, self_.group1_.w, self_.group1_.y, self_.group0_.x) * other_groups.group0_.xyxx) - (vec4<f32>(other_groups.group0_.z, other_groups.group0_.x, other_groups.group0_.y, other_groups.group1_.x) * self_.group1_.yzxx), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.z * self_.group0_.y) + (other_groups.group1_.x * self_.group1_.w) + (other_groups.group1_.y * self_.group1_.z) - (other_groups.group1_.w * self_.group0_.x), (other_groups.group0_.x * self_.group0_.z) + (other_groups.group1_.y * self_.group1_.w) + (other_groups.group1_.z * self_.group1_.x) - (other_groups.group1_.w * self_.group0_.y), (other_groups.group0_.y * self_.group0_.x) + (other_groups.group1_.x * self_.group1_.y) + (other_groups.group1_.z * self_.group1_.w) - (other_groups.group1_.w * self_.group0_.z), 0.0) + (vec4<f32>(other_groups.group0_.w, other_groups.group0_.w, other_groups.group0_.w, other_groups.group1_.w) * self_.group1_) - (vec4<f32>(other_groups.group1_.z, other_groups.group1_.x, other_groups.group1_.y, other_groups.group0_.z) * self_.group1_.yzxz) - (vec4<f32>(self_.group0_.z, self_.group0_.x, self_.group0_.w, self_.group1_.y) * other_groups.group0_.yzzy) - (vec4<f32>(self_.group0_.w, self_.group0_.w, self_.group0_.y, self_.group1_.x) * other_groups.group0_.xyxx)
    );
    let geometric_product: Flector = flector_degroup(geometric_product_groups);
    return flector_geometricProduct_motor(geometric_product, motor_reverse(self));
}
fn motor_sandwich_horizon(self_: Motor, other: Horizon) -> Flector {
    let self_groups = motor_grouped(self_);
    let other_groups = horizon_grouped(other);
    let geometric_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e321) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group0_.w) * vec4<f32>(1.0, 1.0, 1.0, -1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e321) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group1_.w) * vec4<f32>(-1.0, -1.0, -1.0, 1.0)
    );
    let geometric_product: Flector = flector_degroup(geometric_product_groups);
    return flector_geometricProduct_motor(geometric_product, motor_reverse(self));
}
fn motor_sandwich_line(self_: Motor, other: Line) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = line_grouped(other);
    let geometric_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other_groups.group0_.x * self_.group1_.w) + (other_groups.group0_.y * self_.group1_.z) + (other_groups.group1_.x * self_.group0_.w) + (other_groups.group1_.y * self_.group0_.z), (other_groups.group0_.y * self_.group1_.w) + (other_groups.group0_.z * self_.group1_.x) + (other_groups.group1_.y * self_.group0_.w) + (other_groups.group1_.z * self_.group0_.x), (other_groups.group0_.x * self_.group1_.y) + (other_groups.group0_.z * self_.group1_.w) + (other_groups.group1_.x * self_.group0_.y) + (other_groups.group1_.z * self_.group0_.w), -(other_groups.group0_.y * self_.group1_.y) - (other_groups.group0_.z * self_.group1_.z) - (other_groups.group1_.y * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.z)) - (vec4<f32>(other_groups.group0_.zxy.xyz, other_groups.group0_.x) * self_.group1_.yzxx) - (vec4<f32>(other_groups.group1_.zxy.xyz, other_groups.group1_.x) * self_.group0_.yzxx), 
        /* e23, e31, e12, scalar */ vec4<f32>((other_groups.group1_.x * self_.group1_.w) + (other_groups.group1_.y * self_.group1_.z), (other_groups.group1_.y * self_.group1_.w) + (other_groups.group1_.z * self_.group1_.x), (other_groups.group1_.x * self_.group1_.y) + (other_groups.group1_.z * self_.group1_.w), -(other_groups.group1_.y * self_.group1_.y) - (other_groups.group1_.z * self_.group1_.z)) - (vec4<f32>(other_groups.group1_.zxy.xyz, other_groups.group1_.x) * self_.group1_.yzxx)
    );
    let geometric_product: Motor = motor_degroup(geometric_product_groups);
    return motor_geometricProduct_motor(geometric_product, motor_reverse(self));
}
fn motor_sandwich_motor(self_: Motor, other: Motor) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = motor_grouped(other);
    let geometric_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other_groups.group0_.y * self_.group1_.z) + (other_groups.group0_.w * self_.group1_.x) + (other_groups.group1_.y * self_.group0_.z) + (other_groups.group1_.w * self_.group0_.x), (other_groups.group0_.z * self_.group1_.x) + (other_groups.group0_.w * self_.group1_.y) + (other_groups.group1_.z * self_.group0_.x) + (other_groups.group1_.w * self_.group0_.y), (other_groups.group0_.z * self_.group1_.w) + (other_groups.group0_.w * self_.group1_.z) + (other_groups.group1_.z * self_.group0_.w) + (other_groups.group1_.w * self_.group0_.z), -(other_groups.group0_.y * self_.group1_.y) - (other_groups.group0_.z * self_.group1_.z) - (other_groups.group1_.y * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.z)) + (other_groups.group0_.xyxw * self_.group1_.wwyw) + (other_groups.group1_.xyxw * self_.group0_.wwyw) - (other_groups.group0_.zxyx * self_.group1_.yzxx) - (other_groups.group1_.zxyx * self_.group0_.yzxx), 
        /* e23, e31, e12, scalar */ vec4<f32>((other_groups.group1_.y * self_.group1_.z) + (other_groups.group1_.w * self_.group1_.x), (other_groups.group1_.z * self_.group1_.x) + (other_groups.group1_.w * self_.group1_.y), (other_groups.group1_.z * self_.group1_.w) + (other_groups.group1_.w * self_.group1_.z), -(other_groups.group1_.y * self_.group1_.y) - (other_groups.group1_.z * self_.group1_.z)) + (other_groups.group1_.xyxw * self_.group1_.wwyw) - (other_groups.group1_.zxyx * self_.group1_.yzxx)
    );
    let geometric_product: Motor = motor_degroup(geometric_product_groups);
    return motor_geometricProduct_motor(geometric_product, motor_reverse(self));
}
fn motor_sandwich_multiVector(self_: Motor, other: MultiVector) -> MultiVector {
    let self_groups = motor_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let geometric_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, (other_groups.group0_.y * self_.group1_.w) - (other_groups.group3_.x * self_.group0_.x) - (other_groups.group3_.y * self_.group0_.y) - (other_groups.group3_.z * self_.group0_.z), 0.0, 0.0) + (vec4<f32>(vec2<f32>(other_groups.group0_.x), 0.0, 0.0) * vec4<f32>(self_.group1_.w, self_.group0_.w, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(self_.group1_.x), 0.0, 0.0) * vec4<f32>(other_groups.group3_.x, other_groups.group2_.x, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(self_.group1_.y), 0.0, 0.0) * vec4<f32>(other_groups.group3_.y, other_groups.group2_.y, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(self_.group1_.z), 0.0, 0.0) * vec4<f32>(other_groups.group3_.z, other_groups.group2_.z, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group1_.w * other_groups.group1_.x, self_.group1_.y * other_groups.group4_.w, self_.group1_.z * other_groups.group4_.w, (self_.group0_.y * other_groups.group1_.y) + (self_.group0_.z * other_groups.group1_.z) - (self_.group0_.w * other_groups.group4_.w) - (self_.group1_.y * other_groups.group4_.y) - (self_.group1_.z * other_groups.group4_.z)) + (vec4<f32>(self_.group1_.z, self_.group1_.w, self_.group1_.w, self_.group0_.x) * other_groups.group1_.yyzx) + (vec4<f32>(other_groups.group4_.w, other_groups.group1_.z, other_groups.group1_.x, other_groups.group1_.w) * self_.group1_.xxyw) - (vec4<f32>(other_groups.group1_.z, other_groups.group1_.x, other_groups.group1_.y, other_groups.group4_.x) * self_.group1_.yzxx), 
        /* e41, e42, e43 */ (vec4<f32>(vec3<f32>(other_groups.group0_.x), 0.0) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0)) + (vec4<f32>(vec3<f32>(other_groups.group0_.y), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0)) + (vec4<f32>(self_.group0_.z, self_.group0_.x, self_.group0_.w, 0.0) * other_groups.group3_.yzz) + (vec4<f32>(self_.group0_.w, self_.group0_.w, self_.group0_.y, 0.0) * other_groups.group3_.xyx) + (vec4<f32>(self_.group1_.z, self_.group1_.x, self_.group1_.w, 0.0) * other_groups.group2_.yzz) + (vec4<f32>(self_.group1_.w, self_.group1_.w, self_.group1_.y, 0.0) * other_groups.group2_.xyx) - (vec4<f32>(self_.group0_.y, self_.group0_.z, self_.group0_.x, 0.0) * other_groups.group3_.zxy) - (vec4<f32>(self_.group1_.y, self_.group1_.z, self_.group1_.x, 0.0) * other_groups.group2_.zxy), 
        /* e23, e31, e12 */ (vec4<f32>(vec3<f32>(other_groups.group0_.x), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0)) + (vec4<f32>(self_.group1_.z, self_.group1_.x, self_.group1_.w, 0.0) * other_groups.group3_.yzz) + (vec4<f32>(self_.group1_.w, self_.group1_.w, self_.group1_.y, 0.0) * other_groups.group3_.xyx) - (vec4<f32>(self_.group1_.y, self_.group1_.z, self_.group1_.x, 0.0) * other_groups.group3_.zxy), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.group0_.y * other_groups.group1_.z) + (self_.group1_.z * other_groups.group4_.y) + (self_.group1_.w * other_groups.group4_.x) - (self_.group0_.x * other_groups.group4_.w), (self_.group0_.z * other_groups.group1_.x) + (self_.group1_.y * other_groups.group1_.w) + (self_.group1_.w * other_groups.group4_.y) - (self_.group0_.y * other_groups.group4_.w), (self_.group0_.x * other_groups.group1_.y) + (self_.group1_.z * other_groups.group1_.w) + (self_.group1_.w * other_groups.group4_.z) - (self_.group0_.z * other_groups.group4_.w), 0.0) + (vec4<f32>(other_groups.group1_.w, other_groups.group4_.z, other_groups.group4_.x, other_groups.group4_.w) * self_.group1_.xxyw) - (vec4<f32>(self_.group0_.z, self_.group0_.x, self_.group0_.y, self_.group1_.x) * other_groups.group1_.yzxx) - (vec4<f32>(self_.group0_.w, self_.group0_.w, self_.group0_.w, self_.group1_.y) * other_groups.group1_.xyzy) - (vec4<f32>(other_groups.group4_.z, other_groups.group4_.x, other_groups.group4_.y, other_groups.group1_.z) * self_.group1_.yzxz)
    );
    let geometric_product: MultiVector = multiVector_degroup(geometric_product_groups);
    return multiVector_geometricProduct_motor(geometric_product, motor_reverse(self));
}
fn motor_sandwich_origin(self_: Motor, other: Origin) -> Flector {
    let self_groups = motor_grouped(self_);
    let other_groups = origin_grouped(other);
    let geometric_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.group1_.w * other.e4), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group1_.x * other.e4, self_.group1_.y * other.e4, self_.group1_.z * other.e4, 0.0)
    );
    let geometric_product: Flector = flector_degroup(geometric_product_groups);
    return flector_geometricProduct_motor(geometric_product, motor_reverse(self));
}
fn motor_sandwich_plane(self_: Motor, other: Plane) -> Flector {
    let self_groups = motor_grouped(self_);
    let other_groups = plane_grouped(other);
    let geometric_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group1_.x * other_groups.group0_.w, self_.group1_.y * other_groups.group0_.w, self_.group1_.z * other_groups.group0_.w, -(self_.group0_.w * other_groups.group0_.w) - (self_.group1_.x * other_groups.group0_.x) - (self_.group1_.y * other_groups.group0_.y) - (self_.group1_.z * other_groups.group0_.z)), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.group1_.z * other_groups.group0_.y) + (self_.group1_.w * other_groups.group0_.x) - (self_.group0_.x * other_groups.group0_.w) - (self_.group1_.y * other_groups.group0_.z), (self_.group1_.x * other_groups.group0_.z) + (self_.group1_.w * other_groups.group0_.y) - (self_.group0_.y * other_groups.group0_.w) - (self_.group1_.z * other_groups.group0_.x), (self_.group1_.y * other_groups.group0_.x) + (self_.group1_.w * other_groups.group0_.z) - (self_.group0_.z * other_groups.group0_.w) - (self_.group1_.x * other_groups.group0_.y), self_.group1_.w * other_groups.group0_.w)
    );
    let geometric_product: Flector = flector_degroup(geometric_product_groups);
    return flector_geometricProduct_motor(geometric_product, motor_reverse(self));
}
fn motor_sandwich_point(self_: Motor, other: Point) -> Flector {
    let self_groups = motor_grouped(self_);
    let other_groups = point_grouped(other);
    let geometric_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group1_.y * other_groups.group0_.z) * -1.0, (self_.group1_.z * other_groups.group0_.x) * -1.0, (self_.group1_.x * other_groups.group0_.y) * -1.0, (self_.group0_.z * other_groups.group0_.z) + (self_.group1_.w * other_groups.group0_.w)) + (vec4<f32>(self_.group1_.z, self_.group1_.x, self_.group1_.y, self_.group0_.x) * other_groups.group0_.yzxx) + (vec4<f32>(self_.group1_.w, self_.group1_.w, self_.group1_.w, self_.group0_.y) * other_groups.group0_.xyzy), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.group0_.y * other_groups.group0_.z) + (self_.group1_.x * other_groups.group0_.w), (self_.group0_.z * other_groups.group0_.x) + (self_.group1_.y * other_groups.group0_.w), (self_.group0_.x * other_groups.group0_.y) + (self_.group1_.z * other_groups.group0_.w), (self_.group1_.z * other_groups.group0_.z) * -1.0) - (vec4<f32>(self_.group0_.z, self_.group0_.x, self_.group0_.y, self_.group1_.x) * other_groups.group0_.yzxx) - (vec4<f32>(self_.group0_.w, self_.group0_.w, self_.group0_.w, self_.group1_.y) * other_groups.group0_.xyzy)
    );
    let geometric_product: Flector = flector_degroup(geometric_product_groups);
    return flector_geometricProduct_motor(geometric_product, motor_reverse(self));
}
fn motor_sandwich_scalar(self_: Motor, other: Scalar) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = scalar_grouped(other);
    let geometric_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.scalar) * self_.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(other.scalar) * self_.group1_
    );
    let geometric_product: Motor = motor_degroup(geometric_product_groups);
    return motor_geometricProduct_motor(geometric_product, motor_reverse(self));
}
fn multiVector_sandwich_antiScalar(self_: MultiVector, other: AntiScalar) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    let geometric_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, self_.group0_.x * other.e1234, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.group4_.w * other.e1234), 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(other.e1234), 0.0) * self_.group3_, 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group1_.x * other.e1234, self_.group1_.y * other.e1234, self_.group1_.z * other.e1234, 0.0)
    );
    let geometric_product: MultiVector = multiVector_degroup(geometric_product_groups);
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self));
}
fn multiVector_sandwich_dualNum(self_: MultiVector, other: DualNum) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let geometric_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other_groups.group0_.x * self_.group0_.x, (other_groups.group0_.x * self_.group0_.y) + (other_groups.group0_.y * self_.group0_.x), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group0_.x * self_.group1_.x, other_groups.group0_.x * self_.group1_.y, other_groups.group0_.x * self_.group1_.z, (other_groups.group0_.x * self_.group1_.w) + (other_groups.group0_.y * self_.group4_.w)), 
        /* e41, e42, e43 */ (vec4<f32>(vec3<f32>(other_groups.group0_.x), 0.0) * self_.group2_) + (vec4<f32>(vec3<f32>(other_groups.group0_.y), 0.0) * self_.group3_), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(other_groups.group0_.x), 0.0) * self_.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.x * self_.group4_.x) + (other_groups.group0_.y * self_.group1_.x), (other_groups.group0_.x * self_.group4_.y) + (other_groups.group0_.y * self_.group1_.y), (other_groups.group0_.x * self_.group4_.z) + (other_groups.group0_.y * self_.group1_.z), other_groups.group0_.x * self_.group4_.w)
    );
    let geometric_product: MultiVector = multiVector_degroup(geometric_product_groups);
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self));
}
fn multiVector_sandwich_flector(self_: MultiVector, other: Flector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = flector_grouped(other);
    let geometric_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, (other_groups.group1_.w * self_.group1_.w) - (other_groups.group0_.x * self_.group4_.x) - (other_groups.group0_.y * self_.group4_.y) - (other_groups.group0_.z * self_.group4_.z), 0.0, 0.0) + (vec4<f32>(vec2<f32>(self_.group1_.x), 0.0, 0.0) * vec4<f32>(other_groups.group0_.x, other_groups.group1_.x, 0.0, 0.0)) + (vec4<f32>(vec2<f32>(self_.group1_.y), 0.0, 0.0) * vec4<f32>(other_groups.group0_.y, other_groups.group1_.y, 0.0, 0.0)) + (vec4<f32>(vec2<f32>(self_.group1_.z), 0.0, 0.0) * vec4<f32>(other_groups.group0_.z, other_groups.group1_.z, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(self_.group4_.w), 0.0, 0.0) * vec4<f32>(other_groups.group1_.w, other_groups.group0_.w, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group3_.x * other_groups.group1_.w) - (self_.group3_.y * other_groups.group0_.z), (self_.group3_.y * other_groups.group1_.w) - (self_.group3_.z * other_groups.group0_.x), (self_.group3_.z * other_groups.group1_.w) - (self_.group3_.x * other_groups.group0_.y), (self_.group2_.y * other_groups.group0_.y) + (self_.group2_.z * other_groups.group0_.z) - (self_.group0_.y * other_groups.group1_.w) - (self_.group3_.x * other_groups.group1_.x) - (self_.group3_.y * other_groups.group1_.y) - (self_.group3_.z * other_groups.group1_.z)) + (vec4<f32>(self_.group0_.x) * other_groups.group0_) + (vec4<f32>(self_.group3_.zxy.xyz, self_.group2_.x) * other_groups.group0_.yzxx), 
        /* e41, e42, e43 */ vec4<f32>((other_groups.group0_.y * self_.group4_.z) + (other_groups.group1_.z * self_.group1_.y) - (other_groups.group0_.z * self_.group4_.y) - (other_groups.group1_.y * self_.group1_.z), (other_groups.group0_.z * self_.group4_.x) + (other_groups.group1_.x * self_.group1_.z) - (other_groups.group0_.x * self_.group4_.z) - (other_groups.group1_.z * self_.group1_.x), (other_groups.group0_.x * self_.group4_.y) + (other_groups.group1_.y * self_.group1_.x) - (other_groups.group0_.y * self_.group4_.x) - (other_groups.group1_.x * self_.group1_.y), 0.0) + (vec4<f32>(vec3<f32>(other_groups.group1_.w), 0.0) * vec4<f32>(self_.group4_.x, self_.group4_.y, self_.group4_.z, 0.0)) + (vec4<f32>(vec3<f32>(self_.group1_.w), 0.0) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0)) - (vec4<f32>(vec3<f32>(other_groups.group0_.w), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0)) - (vec4<f32>(vec3<f32>(self_.group4_.w), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((other_groups.group0_.z * self_.group1_.y) - (other_groups.group0_.y * self_.group1_.z), (other_groups.group0_.x * self_.group1_.z) - (other_groups.group0_.z * self_.group1_.x), (other_groups.group0_.y * self_.group1_.x) - (other_groups.group0_.x * self_.group1_.y), 0.0) - (vec4<f32>(vec3<f32>(other_groups.group1_.w), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0)) - (vec4<f32>(vec3<f32>(self_.group4_.w), 0.0) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.group2_.y * other_groups.group0_.z) + (self_.group3_.x * other_groups.group0_.w) + (self_.group3_.z * other_groups.group1_.y) - (self_.group2_.x * other_groups.group1_.w) - (self_.group3_.y * other_groups.group1_.z), (self_.group2_.z * other_groups.group0_.x) + (self_.group3_.x * other_groups.group1_.z) + (self_.group3_.y * other_groups.group0_.w) - (self_.group2_.y * other_groups.group1_.w) - (self_.group3_.z * other_groups.group1_.x), (self_.group2_.x * other_groups.group0_.y) + (self_.group3_.y * other_groups.group1_.x) + (self_.group3_.z * other_groups.group0_.w) - (self_.group2_.z * other_groups.group1_.w) - (self_.group3_.x * other_groups.group1_.y), (self_.group3_.z * other_groups.group0_.z) * -1.0) + (vec4<f32>(self_.group0_.x) * other_groups.group1_) - (vec4<f32>(self_.group0_.yy.xy, self_.group0_.y, self_.group3_.x) * other_groups.group0_.xyzx) - (vec4<f32>(self_.group2_.zxy.xyz, self_.group3_.y) * other_groups.group0_.yzxy)
    );
    let geometric_product: MultiVector = multiVector_degroup(geometric_product_groups);
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self));
}
fn multiVector_sandwich_horizon(self_: MultiVector, other: Horizon) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = horizon_grouped(other);
    let geometric_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(other.e321), 0.0, 0.0) * vec4<f32>(self_.group4_.w, self_.group1_.w, 0.0, 0.0) * vec2<f32>(-1.0, 1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(other.e321) * vec4<f32>(self_.group3_.xyz.xyz, self_.group0_.y) * vec4<f32>(1.0, 1.0, 1.0, -1.0), 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(other.e321), 0.0) * vec4<f32>(self_.group4_.x, self_.group4_.y, self_.group4_.z, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(other.e321), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0) * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e321) * vec4<f32>(self_.group2_.xyz.xyz, self_.group0_.x) * vec4<f32>(-1.0, -1.0, -1.0, 1.0)
    );
    let geometric_product: MultiVector = multiVector_degroup(geometric_product_groups);
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self));
}
fn multiVector_sandwich_line(self_: MultiVector, other: Line) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = line_grouped(other);
    let geometric_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, -(other_groups.group1_.x * self_.group2_.x) - (other_groups.group1_.y * self_.group2_.y) - (other_groups.group1_.z * self_.group2_.z), 0.0, 0.0) - (vec4<f32>(vec2<f32>(self_.group3_.x), 0.0, 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group0_.x, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(self_.group3_.y), 0.0, 0.0) * vec4<f32>(other_groups.group1_.y, other_groups.group0_.y, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(self_.group3_.z), 0.0, 0.0) * vec4<f32>(other_groups.group1_.z, other_groups.group0_.z, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group1_.x * self_.group4_.w) + (other_groups.group1_.y * self_.group1_.z), (other_groups.group1_.y * self_.group4_.w) + (other_groups.group1_.z * self_.group1_.x), (other_groups.group1_.x * self_.group1_.y) + (other_groups.group1_.z * self_.group4_.w), -(other_groups.group0_.y * self_.group1_.y) - (other_groups.group0_.z * self_.group1_.z) - (other_groups.group1_.x * self_.group4_.x) - (other_groups.group1_.y * self_.group4_.y) - (other_groups.group1_.z * self_.group4_.z)) - (vec4<f32>(other_groups.group1_.zxy.xyz, other_groups.group0_.x) * self_.group1_.yzxx), 
        /* e41, e42, e43 */ (vec4<f32>(vec3<f32>(self_.group0_.x), 0.0) * other_groups.group0_) + (vec4<f32>(vec3<f32>(self_.group0_.y), 0.0) * other_groups.group1_) + (other_groups.group0_.yzx * self_.group3_.zxy) + (other_groups.group1_.yzx * self_.group2_.zxy) - (other_groups.group0_.zxy * self_.group3_.yzx) - (other_groups.group1_.zxy * self_.group2_.yzx), 
        /* e23, e31, e12 */ (vec4<f32>(vec3<f32>(self_.group0_.x), 0.0) * other_groups.group1_) + (other_groups.group1_.yzx * self_.group3_.zxy) - (other_groups.group1_.zxy * self_.group3_.yzx), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.x * self_.group4_.w) + (other_groups.group0_.y * self_.group1_.z) + (other_groups.group1_.x * self_.group1_.w) + (other_groups.group1_.y * self_.group4_.z) - (other_groups.group1_.z * self_.group4_.y), (other_groups.group0_.y * self_.group4_.w) + (other_groups.group0_.z * self_.group1_.x) + (other_groups.group1_.y * self_.group1_.w) + (other_groups.group1_.z * self_.group4_.x) - (other_groups.group1_.x * self_.group4_.z), (other_groups.group0_.x * self_.group1_.y) + (other_groups.group0_.z * self_.group4_.w) + (other_groups.group1_.x * self_.group4_.y) + (other_groups.group1_.z * self_.group1_.w) - (other_groups.group1_.y * self_.group4_.x), -(other_groups.group1_.y * self_.group1_.y) - (other_groups.group1_.z * self_.group1_.z)) - (vec4<f32>(other_groups.group0_.zxy.xyz, other_groups.group1_.x) * self_.group1_.yzxx)
    );
    let geometric_product: MultiVector = multiVector_degroup(geometric_product_groups);
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self));
}
fn multiVector_sandwich_motor(self_: MultiVector, other: Motor) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = motor_grouped(other);
    let geometric_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, (self_.group0_.y * other_groups.group1_.w) - (self_.group3_.x * other_groups.group0_.x) - (self_.group3_.y * other_groups.group0_.y) - (self_.group3_.z * other_groups.group0_.z), 0.0, 0.0) + (vec4<f32>(vec2<f32>(self_.group0_.x), 0.0, 0.0) * vec4<f32>(other_groups.group1_.w, other_groups.group0_.w, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(other_groups.group1_.x), 0.0, 0.0) * vec4<f32>(self_.group3_.x, self_.group2_.x, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(other_groups.group1_.y), 0.0, 0.0) * vec4<f32>(self_.group3_.y, self_.group2_.y, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(other_groups.group1_.z), 0.0, 0.0) * vec4<f32>(self_.group3_.z, self_.group2_.z, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group1_.y * self_.group1_.z) + (other_groups.group1_.w * self_.group1_.x), (other_groups.group1_.z * self_.group1_.x) + (other_groups.group1_.w * self_.group1_.y), (other_groups.group1_.z * self_.group4_.w) + (other_groups.group1_.w * self_.group1_.z), (other_groups.group0_.w * self_.group4_.w) - (other_groups.group0_.y * self_.group1_.y) - (other_groups.group0_.z * self_.group1_.z) - (other_groups.group1_.x * self_.group4_.x) - (other_groups.group1_.y * self_.group4_.y) - (other_groups.group1_.z * self_.group4_.z)) + (vec4<f32>(self_.group4_.w, self_.group4_.w, self_.group1_.y, self_.group1_.w) * other_groups.group1_.xyxw) - (vec4<f32>(other_groups.group1_.z, other_groups.group1_.x, other_groups.group1_.y, other_groups.group0_.x) * self_.group1_.yzxx), 
        /* e41, e42, e43 */ (vec4<f32>(vec3<f32>(self_.group0_.x), 0.0) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0)) + (vec4<f32>(vec3<f32>(self_.group0_.y), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0)) + (vec4<f32>(other_groups.group0_.y, other_groups.group0_.w, other_groups.group0_.w, 0.0) * self_.group3_.zyz) + (vec4<f32>(other_groups.group0_.w, other_groups.group0_.z, other_groups.group0_.x, 0.0) * self_.group3_.xxy) + (vec4<f32>(other_groups.group1_.y, other_groups.group1_.w, other_groups.group1_.w, 0.0) * self_.group2_.zyz) + (vec4<f32>(other_groups.group1_.w, other_groups.group1_.z, other_groups.group1_.x, 0.0) * self_.group2_.xxy) - (vec4<f32>(other_groups.group0_.z, other_groups.group0_.x, other_groups.group0_.y, 0.0) * self_.group3_.yzx) - (vec4<f32>(other_groups.group1_.z, other_groups.group1_.x, other_groups.group1_.y, 0.0) * self_.group2_.yzx), 
        /* e23, e31, e12 */ (vec4<f32>(vec3<f32>(self_.group0_.x), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0)) + (vec4<f32>(other_groups.group1_.y, other_groups.group1_.w, other_groups.group1_.w, 0.0) * self_.group3_.zyz) + (vec4<f32>(other_groups.group1_.w, other_groups.group1_.z, other_groups.group1_.x, 0.0) * self_.group3_.xxy) - (vec4<f32>(other_groups.group1_.z, other_groups.group1_.x, other_groups.group1_.y, 0.0) * self_.group3_.yzx), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.y * self_.group1_.z) + (other_groups.group0_.w * self_.group1_.x) + (other_groups.group1_.x * self_.group1_.w) + (other_groups.group1_.y * self_.group4_.z) + (other_groups.group1_.w * self_.group4_.x), (other_groups.group0_.z * self_.group1_.x) + (other_groups.group0_.w * self_.group1_.y) + (other_groups.group1_.y * self_.group1_.w) + (other_groups.group1_.z * self_.group4_.x) + (other_groups.group1_.w * self_.group4_.y), (other_groups.group0_.x * self_.group1_.y) + (other_groups.group0_.w * self_.group1_.z) + (other_groups.group1_.x * self_.group4_.y) + (other_groups.group1_.z * self_.group1_.w) + (other_groups.group1_.w * self_.group4_.z), (other_groups.group1_.z * self_.group1_.z) * -1.0) + (vec4<f32>(self_.group4_.w) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, other_groups.group1_.w)) - (vec4<f32>(other_groups.group0_.z, other_groups.group0_.x, other_groups.group0_.y, other_groups.group1_.x) * self_.group1_.yzxx) - (vec4<f32>(self_.group4_.y, self_.group4_.z, self_.group4_.x, self_.group1_.y) * other_groups.group1_.zxyy)
    );
    let geometric_product: MultiVector = multiVector_degroup(geometric_product_groups);
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self));
}
fn multiVector_sandwich_multiVector(self_: MultiVector, other: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let geometric_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, (other_groups.group0_.y * self_.group0_.x) + (other_groups.group4_.w * self_.group1_.w) - (other_groups.group3_.x * self_.group2_.x) - (other_groups.group3_.y * self_.group2_.y) - (other_groups.group3_.z * self_.group2_.z) - (other_groups.group1_.x * self_.group4_.x) - (other_groups.group1_.y * self_.group4_.y) - (other_groups.group1_.z * self_.group4_.z), 0.0, 0.0) + (vec4<f32>(vec2<f32>(other_groups.group0_.x), 0.0, 0.0) * self_.group0_) + (vec4<f32>(vec2<f32>(self_.group1_.x), 0.0, 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group4_.x, 0.0, 0.0)) + (vec4<f32>(vec2<f32>(self_.group1_.y), 0.0, 0.0) * vec4<f32>(other_groups.group1_.y, other_groups.group4_.y, 0.0, 0.0)) + (vec4<f32>(vec2<f32>(self_.group1_.z), 0.0, 0.0) * vec4<f32>(other_groups.group1_.z, other_groups.group4_.z, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(self_.group3_.x), 0.0, 0.0) * vec4<f32>(other_groups.group3_.x, other_groups.group2_.x, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(self_.group3_.y), 0.0, 0.0) * vec4<f32>(other_groups.group3_.y, other_groups.group2_.y, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(self_.group3_.z), 0.0, 0.0) * vec4<f32>(other_groups.group3_.z, other_groups.group2_.z, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(self_.group4_.w), 0.0, 0.0) * vec4<f32>(other_groups.group4_.w, other_groups.group1_.w, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group3_.y * self_.group1_.z) + (self_.group3_.x * other_groups.group4_.w) - (self_.group3_.y * other_groups.group1_.z), (other_groups.group3_.z * self_.group1_.x) + (self_.group3_.y * other_groups.group4_.w) - (self_.group3_.z * other_groups.group1_.x), (other_groups.group3_.x * self_.group1_.y) + (self_.group3_.z * other_groups.group4_.w) - (self_.group3_.x * other_groups.group1_.y), (self_.group2_.y * other_groups.group1_.y) + (self_.group2_.z * other_groups.group1_.z) - (self_.group0_.y * other_groups.group4_.w) - (other_groups.group2_.y * self_.group1_.y) - (other_groups.group2_.z * self_.group1_.z) - (other_groups.group3_.x * self_.group4_.x) - (other_groups.group3_.y * self_.group4_.y) - (other_groups.group3_.z * self_.group4_.z) - (self_.group3_.x * other_groups.group4_.x) - (self_.group3_.y * other_groups.group4_.y) - (self_.group3_.z * other_groups.group4_.z)) + (vec4<f32>(other_groups.group0_.x) * self_.group1_) + (vec4<f32>(self_.group0_.x) * other_groups.group1_) + (vec4<f32>(self_.group4_.w) * vec4<f32>(other_groups.group3_.xyz.xyz, other_groups.group0_.y)) + (vec4<f32>(self_.group3_.zxy.xyz, self_.group2_.x) * other_groups.group1_.yzxx) - (vec4<f32>(other_groups.group3_.zxy.xyz, other_groups.group2_.x) * self_.group1_.yzxx), 
        /* e41, e42, e43 */ vec4<f32>((other_groups.group1_.y * self_.group4_.z) + (other_groups.group4_.z * self_.group1_.y) - (other_groups.group1_.z * self_.group4_.y) - (other_groups.group4_.y * self_.group1_.z), (other_groups.group1_.z * self_.group4_.x) + (other_groups.group4_.x * self_.group1_.z) - (other_groups.group1_.x * self_.group4_.z) - (other_groups.group4_.z * self_.group1_.x), (other_groups.group1_.x * self_.group4_.y) + (other_groups.group4_.y * self_.group1_.x) - (other_groups.group1_.y * self_.group4_.x) - (other_groups.group4_.x * self_.group1_.y), 0.0) + (vec4<f32>(vec3<f32>(other_groups.group0_.x), 0.0) * self_.group2_) + (vec4<f32>(vec3<f32>(other_groups.group0_.y), 0.0) * self_.group3_) + (vec4<f32>(vec3<f32>(self_.group0_.x), 0.0) * other_groups.group2_) + (vec4<f32>(vec3<f32>(self_.group0_.y), 0.0) * other_groups.group3_) + (vec4<f32>(vec3<f32>(other_groups.group4_.w), 0.0) * vec4<f32>(self_.group4_.x, self_.group4_.y, self_.group4_.z, 0.0)) + (vec4<f32>(vec3<f32>(self_.group1_.w), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0)) + (other_groups.group2_.yzx * self_.group3_.zxy) + (other_groups.group3_.yzx * self_.group2_.zxy) - (vec4<f32>(vec3<f32>(other_groups.group1_.w), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0)) - (vec4<f32>(vec3<f32>(self_.group4_.w), 0.0) * vec4<f32>(other_groups.group4_.x, other_groups.group4_.y, other_groups.group4_.z, 0.0)) - (other_groups.group2_.zxy * self_.group3_.yzx) - (other_groups.group3_.zxy * self_.group2_.yzx), 
        /* e23, e31, e12 */ vec4<f32>((other_groups.group1_.z * self_.group1_.y) - (other_groups.group1_.y * self_.group1_.z), (other_groups.group1_.x * self_.group1_.z) - (other_groups.group1_.z * self_.group1_.x), (other_groups.group1_.y * self_.group1_.x) - (other_groups.group1_.x * self_.group1_.y), 0.0) + (vec4<f32>(vec3<f32>(other_groups.group0_.x), 0.0) * self_.group3_) + (vec4<f32>(vec3<f32>(self_.group0_.x), 0.0) * other_groups.group3_) + (other_groups.group3_.yzx * self_.group3_.zxy) - (vec4<f32>(vec3<f32>(other_groups.group4_.w), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0)) - (vec4<f32>(vec3<f32>(self_.group4_.w), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0)) - (other_groups.group3_.zxy * self_.group3_.yzx), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.y * self_.group1_.x) + (other_groups.group2_.x * self_.group4_.w) + (other_groups.group2_.y * self_.group1_.z) + (other_groups.group3_.x * self_.group1_.w) + (other_groups.group3_.y * self_.group4_.z) + (self_.group2_.y * other_groups.group1_.z) + (self_.group3_.x * other_groups.group1_.w) + (self_.group3_.z * other_groups.group4_.y) - (other_groups.group3_.z * self_.group4_.y) - (self_.group2_.x * other_groups.group4_.w) - (self_.group3_.y * other_groups.group4_.z), (other_groups.group0_.y * self_.group1_.y) + (other_groups.group2_.y * self_.group4_.w) + (other_groups.group2_.z * self_.group1_.x) + (other_groups.group3_.y * self_.group1_.w) + (other_groups.group3_.z * self_.group4_.x) + (self_.group2_.z * other_groups.group1_.x) + (self_.group3_.x * other_groups.group4_.z) + (self_.group3_.y * other_groups.group1_.w) - (other_groups.group3_.x * self_.group4_.z) - (self_.group2_.y * other_groups.group4_.w) - (self_.group3_.z * other_groups.group4_.x), (other_groups.group0_.y * self_.group1_.z) + (other_groups.group2_.x * self_.group1_.y) + (other_groups.group2_.z * self_.group4_.w) + (other_groups.group3_.x * self_.group4_.y) + (other_groups.group3_.z * self_.group1_.w) + (self_.group2_.x * other_groups.group1_.y) + (self_.group3_.y * other_groups.group4_.x) + (self_.group3_.z * other_groups.group1_.w) - (other_groups.group3_.y * self_.group4_.x) - (self_.group2_.z * other_groups.group4_.w) - (self_.group3_.x * other_groups.group4_.y), -(other_groups.group3_.y * self_.group1_.y) - (other_groups.group3_.z * self_.group1_.z) - (self_.group3_.z * other_groups.group1_.z)) + (vec4<f32>(other_groups.group0_.x) * self_.group4_) + (vec4<f32>(self_.group0_.x) * other_groups.group4_) - (vec4<f32>(self_.group0_.yy.xy, self_.group0_.y, self_.group3_.x) * other_groups.group1_.xyzx) - (vec4<f32>(other_groups.group2_.zxy.xyz, other_groups.group3_.x) * self_.group1_.yzxx) - (vec4<f32>(self_.group2_.zxy.xyz, self_.group3_.y) * other_groups.group1_.yzxy)
    );
    let geometric_product: MultiVector = multiVector_degroup(geometric_product_groups);
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self));
}
fn multiVector_sandwich_origin(self_: MultiVector, other: Origin) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = origin_grouped(other);
    let geometric_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, self_.group4_.w * other.e4 * -1.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.group0_.x * other.e4), 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(other.e4), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group3_.x * other.e4, self_.group3_.y * other.e4, self_.group3_.z * other.e4, 0.0)
    );
    let geometric_product: MultiVector = multiVector_degroup(geometric_product_groups);
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self));
}
fn multiVector_sandwich_plane(self_: MultiVector, other: Plane) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = plane_grouped(other);
    let geometric_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.group4_.w * other_groups.group0_.w * -1.0, (self_.group1_.x * other_groups.group0_.x) + (self_.group1_.y * other_groups.group0_.y) + (self_.group1_.z * other_groups.group0_.z) + (self_.group1_.w * other_groups.group0_.w), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group3_.x * other_groups.group0_.w, self_.group3_.y * other_groups.group0_.w, self_.group3_.z * other_groups.group0_.w, -(self_.group0_.y * other_groups.group0_.w) - (self_.group3_.x * other_groups.group0_.x) - (self_.group3_.y * other_groups.group0_.y) - (self_.group3_.z * other_groups.group0_.z)), 
        /* e41, e42, e43 */ vec4<f32>((self_.group1_.y * other_groups.group0_.z) - (self_.group1_.z * other_groups.group0_.y), (self_.group1_.z * other_groups.group0_.x) - (self_.group1_.x * other_groups.group0_.z), (self_.group1_.x * other_groups.group0_.y) - (self_.group1_.y * other_groups.group0_.x), 0.0) + (vec4<f32>(vec3<f32>(other_groups.group0_.w), 0.0) * vec4<f32>(self_.group4_.x, self_.group4_.y, self_.group4_.z, 0.0)) - (vec4<f32>(vec3<f32>(self_.group4_.w), 0.0) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(other_groups.group0_.w), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0) * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.group0_.x * other_groups.group0_.x) + (self_.group3_.z * other_groups.group0_.y) - (self_.group2_.x * other_groups.group0_.w) - (self_.group3_.y * other_groups.group0_.z), (self_.group0_.x * other_groups.group0_.y) + (self_.group3_.x * other_groups.group0_.z) - (self_.group2_.y * other_groups.group0_.w) - (self_.group3_.z * other_groups.group0_.x), (self_.group0_.x * other_groups.group0_.z) + (self_.group3_.y * other_groups.group0_.x) - (self_.group2_.z * other_groups.group0_.w) - (self_.group3_.x * other_groups.group0_.y), self_.group0_.x * other_groups.group0_.w)
    );
    let geometric_product: MultiVector = multiVector_degroup(geometric_product_groups);
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self));
}
fn multiVector_sandwich_point(self_: MultiVector, other: Point) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = point_grouped(other);
    let geometric_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((self_.group1_.x * other_groups.group0_.x) + (self_.group1_.y * other_groups.group0_.y) + (self_.group1_.z * other_groups.group0_.z), -(self_.group4_.x * other_groups.group0_.x) - (self_.group4_.y * other_groups.group0_.y) - (self_.group4_.z * other_groups.group0_.z) - (self_.group4_.w * other_groups.group0_.w), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group3_.y * other_groups.group0_.z) * -1.0, (self_.group3_.z * other_groups.group0_.x) * -1.0, (self_.group3_.x * other_groups.group0_.y) * -1.0, (self_.group2_.y * other_groups.group0_.y) + (self_.group2_.z * other_groups.group0_.z)) + (vec4<f32>(self_.group0_.x) * other_groups.group0_) + (vec4<f32>(self_.group3_.zxy.xyz, self_.group2_.x) * other_groups.group0_.yzxx), 
        /* e41, e42, e43 */ vec4<f32>((self_.group4_.z * other_groups.group0_.y) - (self_.group4_.y * other_groups.group0_.z), (self_.group4_.x * other_groups.group0_.z) - (self_.group4_.z * other_groups.group0_.x), (self_.group4_.y * other_groups.group0_.x) - (self_.group4_.x * other_groups.group0_.y), 0.0) + (vec4<f32>(vec3<f32>(self_.group1_.w), 0.0) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0)) - (vec4<f32>(vec3<f32>(other_groups.group0_.w), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((self_.group1_.y * other_groups.group0_.z) - (self_.group1_.z * other_groups.group0_.y), (self_.group1_.z * other_groups.group0_.x) - (self_.group1_.x * other_groups.group0_.z), (self_.group1_.x * other_groups.group0_.y) - (self_.group1_.y * other_groups.group0_.x), 0.0) - (vec4<f32>(vec3<f32>(self_.group4_.w), 0.0) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.group2_.y * other_groups.group0_.z) + (self_.group3_.x * other_groups.group0_.w), (self_.group2_.z * other_groups.group0_.x) + (self_.group3_.y * other_groups.group0_.w), (self_.group2_.x * other_groups.group0_.y) + (self_.group3_.z * other_groups.group0_.w), (self_.group3_.z * other_groups.group0_.z) * -1.0) - (vec4<f32>(self_.group0_.yy.xy, self_.group0_.y, self_.group3_.x) * other_groups.group0_.xyzx) - (vec4<f32>(self_.group2_.zxy.xyz, self_.group3_.y) * other_groups.group0_.yzxy)
    );
    let geometric_product: MultiVector = multiVector_degroup(geometric_product_groups);
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self));
}
fn multiVector_sandwich_scalar(self_: MultiVector, other: Scalar) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = scalar_grouped(other);
    let geometric_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(other.scalar), 0.0, 0.0) * self_.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(other.scalar) * self_.group1_, 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(other.scalar), 0.0) * self_.group2_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(other.scalar), 0.0) * self_.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other.scalar) * self_.group4_
    );
    let geometric_product: MultiVector = multiVector_degroup(geometric_product_groups);
    return multiVector_geometricProduct_multiVector(geometric_product, multiVector_reverse(self));
}
fn origin_sandwich_flector(self_: Origin, other: Flector) -> Flector {
    let self_groups = origin_grouped(self_);
    let other_groups = flector_grouped(other);
    let geometric_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self.e4) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, other_groups.group1_.w), 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0)
    );
    let geometric_product: Motor = motor_degroup(geometric_product_groups);
    return motor_geometricProduct_origin(geometric_product, origin_reverse(self));
}
fn origin_sandwich_line(self_: Origin, other: Line) -> AntiScalar {
    let self_groups = origin_grouped(self_);
    let other_groups = line_grouped(other);
    let geometric_product_groups: PlaneGroups = PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group1_.x * self.e4, other_groups.group1_.y * self.e4, other_groups.group1_.z * self.e4, 0.0)
    );
    let geometric_product: Plane = plane_degroup(geometric_product_groups);
    return plane_geometricProduct_origin(geometric_product, origin_reverse(self));
}
fn origin_sandwich_motor(self_: Origin, other: Motor) -> Motor {
    let self_groups = origin_grouped(self_);
    let other_groups = motor_grouped(other);
    let geometric_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group1_.w * self.e4), 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group1_.x * self.e4, other_groups.group1_.y * self.e4, other_groups.group1_.z * self.e4, 0.0)
    );
    let geometric_product: Flector = flector_degroup(geometric_product_groups);
    return flector_geometricProduct_origin(geometric_product, origin_reverse(self));
}
fn origin_sandwich_multiVector(self_: Origin, other: MultiVector) -> MultiVector {
    let self_groups = origin_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let geometric_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, other_groups.group4_.w * self.e4, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group0_.x * self.e4), 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(self.e4), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group3_.x * self.e4, other_groups.group3_.y * self.e4, other_groups.group3_.z * self.e4, 0.0)
    );
    let geometric_product: MultiVector = multiVector_degroup(geometric_product_groups);
    return multiVector_geometricProduct_origin(geometric_product, origin_reverse(self));
}
fn origin_sandwich_point(self_: Origin, other: Point) -> Plane {
    let self_groups = origin_grouped(self_);
    let other_groups = point_grouped(other);
    let geometric_product_groups: LineGroups = LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(self.e4), 0.0) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0)
    );
    let geometric_product: Line = line_degroup(geometric_product_groups);
    return line_geometricProduct_origin(geometric_product, origin_reverse(self));
}
fn plane_sandwich_antiScalar(self_: Plane, other: AntiScalar) -> AntiScalar {
    let self_groups = plane_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    let geometric_product_groups: OriginGroups = OriginGroups(
        /* e4 */ vec4<f32>(self_.group0_.w * other.e1234, 0.0, 0.0, 0.0)
    );
    let geometric_product: Origin = origin_degroup(geometric_product_groups);
    return origin_geometricProduct_plane(geometric_product, plane_reverse(self));
}
fn plane_sandwich_dualNum(self_: Plane, other: DualNum) -> Motor {
    let self_groups = plane_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let geometric_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group0_.y * self_.group0_.w), 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group0_.x) * self_.group0_
    );
    let geometric_product: Flector = flector_degroup(geometric_product_groups);
    return flector_geometricProduct_plane(geometric_product, plane_reverse(self));
}
fn plane_sandwich_flector(self_: Plane, other: Flector) -> Flector {
    let self_groups = plane_grouped(self_);
    let other_groups = flector_grouped(other);
    let geometric_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other_groups.group0_.y * self_.group0_.z) + (other_groups.group1_.w * self_.group0_.x), (other_groups.group0_.z * self_.group0_.x) + (other_groups.group1_.w * self_.group0_.y), (other_groups.group0_.x * self_.group0_.y) + (other_groups.group1_.w * self_.group0_.z), -(other_groups.group0_.z * self_.group0_.z) - (other_groups.group0_.w * self_.group0_.w)) - (vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, other_groups.group0_.y) * self_.group0_.wwwy) - (other_groups.group0_.zxyx * self_.group0_.yzxx), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.group0_.w) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, other_groups.group1_.w) * vec4<f32>(-1.0)
    );
    let geometric_product: Motor = motor_degroup(geometric_product_groups);
    return motor_geometricProduct_plane(geometric_product, plane_reverse(self));
}
fn plane_sandwich_horizon(self_: Plane, other: Horizon) -> Flector {
    let self_groups = plane_grouped(self_);
    let other_groups = horizon_grouped(other);
    let geometric_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.group0_.x * other.e321, self_.group0_.y * other.e321, self_.group0_.z * other.e321, 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, self_.group0_.w * other.e321 * -1.0)
    );
    let geometric_product: Motor = motor_degroup(geometric_product_groups);
    return motor_geometricProduct_plane(geometric_product, plane_reverse(self));
}
fn plane_sandwich_line(self_: Plane, other: Line) -> Motor {
    let self_groups = plane_grouped(self_);
    let other_groups = line_grouped(other);
    let geometric_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group1_.x * self_.group0_.w, other_groups.group1_.y * self_.group0_.w, other_groups.group1_.z * self_.group0_.w, -(other_groups.group1_.x * self_.group0_.x) - (other_groups.group1_.y * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.z)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.x * self_.group0_.w) + (other_groups.group1_.y * self_.group0_.z) - (other_groups.group1_.z * self_.group0_.y), (other_groups.group0_.y * self_.group0_.w) + (other_groups.group1_.z * self_.group0_.x) - (other_groups.group1_.x * self_.group0_.z), (other_groups.group0_.z * self_.group0_.w) + (other_groups.group1_.x * self_.group0_.y) - (other_groups.group1_.y * self_.group0_.x), 0.0)
    );
    let geometric_product: Flector = flector_degroup(geometric_product_groups);
    return flector_geometricProduct_plane(geometric_product, plane_reverse(self));
}
fn plane_sandwich_motor(self_: Plane, other: Motor) -> Motor {
    let self_groups = plane_grouped(self_);
    let other_groups = motor_grouped(other);
    let geometric_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group1_.x * self_.group0_.w, other_groups.group1_.y * self_.group0_.w, other_groups.group1_.z * self_.group0_.w, (other_groups.group0_.w * self_.group0_.w) - (other_groups.group1_.x * self_.group0_.x) - (other_groups.group1_.y * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.z)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.x * self_.group0_.w) + (other_groups.group1_.y * self_.group0_.z) + (other_groups.group1_.w * self_.group0_.x) - (other_groups.group1_.z * self_.group0_.y), (other_groups.group0_.y * self_.group0_.w) + (other_groups.group1_.z * self_.group0_.x) + (other_groups.group1_.w * self_.group0_.y) - (other_groups.group1_.x * self_.group0_.z), (other_groups.group0_.z * self_.group0_.w) + (other_groups.group1_.x * self_.group0_.y) + (other_groups.group1_.w * self_.group0_.z) - (other_groups.group1_.y * self_.group0_.x), other_groups.group1_.w * self_.group0_.w)
    );
    let geometric_product: Flector = flector_degroup(geometric_product_groups);
    return flector_geometricProduct_plane(geometric_product, plane_reverse(self));
}
fn plane_sandwich_multiVector(self_: Plane, other: MultiVector) -> MultiVector {
    let self_groups = plane_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let geometric_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other_groups.group4_.w * self_.group0_.w * -1.0, -(other_groups.group1_.x * self_.group0_.x) - (other_groups.group1_.y * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.z) - (other_groups.group1_.w * self_.group0_.w), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group3_.x * self_.group0_.w, other_groups.group3_.y * self_.group0_.w, other_groups.group3_.z * self_.group0_.w, (other_groups.group0_.y * self_.group0_.w) - (other_groups.group3_.x * self_.group0_.x) - (other_groups.group3_.y * self_.group0_.y) - (other_groups.group3_.z * self_.group0_.z)), 
        /* e41, e42, e43 */ vec4<f32>((other_groups.group1_.y * self_.group0_.z) - (other_groups.group1_.z * self_.group0_.y), (other_groups.group1_.z * self_.group0_.x) - (other_groups.group1_.x * self_.group0_.z), (other_groups.group1_.x * self_.group0_.y) - (other_groups.group1_.y * self_.group0_.x), 0.0) + (vec4<f32>(vec3<f32>(other_groups.group4_.w), 0.0) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0)) - (vec4<f32>(vec3<f32>(self_.group0_.w), 0.0) * vec4<f32>(other_groups.group4_.x, other_groups.group4_.y, other_groups.group4_.z, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(self_.group0_.w), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0) * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.x * self_.group0_.x) + (other_groups.group2_.x * self_.group0_.w) + (other_groups.group3_.y * self_.group0_.z) - (other_groups.group3_.z * self_.group0_.y), (other_groups.group0_.x * self_.group0_.y) + (other_groups.group2_.y * self_.group0_.w) + (other_groups.group3_.z * self_.group0_.x) - (other_groups.group3_.x * self_.group0_.z), (other_groups.group0_.x * self_.group0_.z) + (other_groups.group2_.z * self_.group0_.w) + (other_groups.group3_.x * self_.group0_.y) - (other_groups.group3_.y * self_.group0_.x), other_groups.group0_.x * self_.group0_.w)
    );
    let geometric_product: MultiVector = multiVector_degroup(geometric_product_groups);
    return multiVector_geometricProduct_plane(geometric_product, plane_reverse(self));
}
fn plane_sandwich_origin(self_: Plane, other: Origin) -> Origin {
    let self_groups = plane_grouped(self_);
    let other_groups = origin_grouped(other);
    let geometric_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(self_.group0_.w * other.e4 * -1.0, 0.0, 0.0, 0.0)
    );
    let geometric_product: AntiScalar = antiScalar_degroup(geometric_product_groups);
    return antiScalar_geometricProduct_plane(geometric_product, plane_reverse(self));
}
fn plane_sandwich_plane(self_: Plane, other: Plane) -> Flector {
    let self_groups = plane_grouped(self_);
    let other_groups = plane_grouped(other);
    let geometric_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other_groups.group0_.w * self_.group0_.x) - (other_groups.group0_.x * self_.group0_.w), (other_groups.group0_.w * self_.group0_.y) - (other_groups.group0_.y * self_.group0_.w), (other_groups.group0_.w * self_.group0_.z) - (other_groups.group0_.z * self_.group0_.w), 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group0_.w * self_.group0_.w * -1.0)
    );
    let geometric_product: Motor = motor_degroup(geometric_product_groups);
    return motor_geometricProduct_plane(geometric_product, plane_reverse(self));
}
fn plane_sandwich_point(self_: Plane, other: Point) -> Flector {
    let self_groups = plane_grouped(self_);
    let other_groups = point_grouped(other);
    let geometric_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.group0_.z * other_groups.group0_.y, self_.group0_.x * other_groups.group0_.z, self_.group0_.y * other_groups.group0_.x, -(self_.group0_.y * other_groups.group0_.y) - (self_.group0_.z * other_groups.group0_.z) - (self_.group0_.w * other_groups.group0_.w)) - (self_.group0_.yzxx * other_groups.group0_.zxyx), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.group0_.w * other_groups.group0_.x * -1.0, self_.group0_.w * other_groups.group0_.y * -1.0, self_.group0_.w * other_groups.group0_.z * -1.0, 0.0)
    );
    let geometric_product: Motor = motor_degroup(geometric_product_groups);
    return motor_geometricProduct_plane(geometric_product, plane_reverse(self));
}
fn plane_sandwich_scalar(self_: Plane, other: Scalar) -> Motor {
    let self_groups = plane_grouped(self_);
    let other_groups = scalar_grouped(other);
    let geometric_product_groups: PlaneGroups = PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(other.scalar) * self_.group0_
    );
    let geometric_product: Plane = plane_degroup(geometric_product_groups);
    return plane_geometricProduct_plane(geometric_product, plane_reverse(self));
}
fn point_sandwich_antiScalar(self_: Point, other: AntiScalar) -> Motor {
    let self_groups = point_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    let geometric_product_groups: PlaneGroups = PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.x * other.e1234, self_.group0_.y * other.e1234, self_.group0_.z * other.e1234, 0.0)
    );
    let geometric_product: Plane = plane_degroup(geometric_product_groups);
    return plane_geometricProduct_point(geometric_product, point_reverse(self));
}
fn point_sandwich_dualNum(self_: Point, other: DualNum) -> Motor {
    let self_groups = point_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let geometric_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group0_.x) * self_.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group0_.y * self_.group0_.x, other_groups.group0_.y * self_.group0_.y, other_groups.group0_.y * self_.group0_.z, 0.0)
    );
    let geometric_product: Flector = flector_degroup(geometric_product_groups);
    return flector_geometricProduct_point(geometric_product, point_reverse(self));
}
fn point_sandwich_flector(self_: Point, other: Flector) -> Flector {
    let self_groups = point_grouped(self_);
    let other_groups = flector_grouped(other);
    let geometric_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(-(other_groups.group0_.w * self_.group0_.x) - (other_groups.group1_.y * self_.group0_.z), -(other_groups.group0_.w * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.x), -(other_groups.group0_.w * self_.group0_.z) - (other_groups.group1_.x * self_.group0_.y), (other_groups.group1_.z * self_.group0_.z) + (other_groups.group1_.w * self_.group0_.w)) + (vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, other_groups.group1_.x) * self_.group0_.wwwx) + (other_groups.group1_.zxyy * self_.group0_.yzxy), 
        /* e23, e31, e12, scalar */ vec4<f32>(-(other_groups.group0_.y * self_.group0_.z) - (other_groups.group1_.w * self_.group0_.x), -(other_groups.group0_.z * self_.group0_.x) - (other_groups.group1_.w * self_.group0_.y), -(other_groups.group0_.x * self_.group0_.y) - (other_groups.group1_.w * self_.group0_.z), (other_groups.group0_.y * self_.group0_.y) + (other_groups.group0_.z * self_.group0_.z)) + (other_groups.group0_.zxyx * self_.group0_.yzxx)
    );
    let geometric_product: Motor = motor_degroup(geometric_product_groups);
    return motor_geometricProduct_point(geometric_product, point_reverse(self));
}
fn point_sandwich_horizon(self_: Point, other: Horizon) -> Flector {
    let self_groups = point_grouped(self_);
    let other_groups = horizon_grouped(other);
    let geometric_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, self_.group0_.w * other.e321), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.group0_.x * other.e321 * -1.0, self_.group0_.y * other.e321 * -1.0, self_.group0_.z * other.e321 * -1.0, 0.0)
    );
    let geometric_product: Motor = motor_degroup(geometric_product_groups);
    return motor_geometricProduct_point(geometric_product, point_reverse(self));
}
fn point_sandwich_line(self_: Point, other: Line) -> Motor {
    let self_groups = point_grouped(self_);
    let other_groups = line_grouped(other);
    let geometric_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group1_.y * self_.group0_.z, other_groups.group1_.z * self_.group0_.x, other_groups.group1_.x * self_.group0_.y, -(other_groups.group0_.y * self_.group0_.y) - (other_groups.group0_.z * self_.group0_.z)) - (vec4<f32>(other_groups.group1_.zxy.xyz, other_groups.group0_.x) * self_.group0_.yzxx), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.y * self_.group0_.z) + (other_groups.group1_.x * self_.group0_.w), (other_groups.group0_.z * self_.group0_.x) + (other_groups.group1_.y * self_.group0_.w), (other_groups.group0_.x * self_.group0_.y) + (other_groups.group1_.z * self_.group0_.w), -(other_groups.group1_.y * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.z)) - (vec4<f32>(other_groups.group0_.zxy.xyz, other_groups.group1_.x) * self_.group0_.yzxx)
    );
    let geometric_product: Flector = flector_degroup(geometric_product_groups);
    return flector_geometricProduct_point(geometric_product, point_reverse(self));
}
fn point_sandwich_motor(self_: Point, other: Motor) -> Motor {
    let self_groups = point_grouped(self_);
    let other_groups = motor_grouped(other);
    let geometric_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group1_.w * self_.group0_.x, other_groups.group1_.w * self_.group0_.y, other_groups.group1_.w * self_.group0_.z, -(other_groups.group0_.y * self_.group0_.y) - (other_groups.group0_.z * self_.group0_.z)) + (other_groups.group1_.yzxw * self_.group0_.zxyw) - (vec4<f32>(other_groups.group1_.z, other_groups.group1_.x, other_groups.group1_.y, other_groups.group0_.x) * self_.group0_.yzxx), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.y * self_.group0_.z) + (other_groups.group0_.w * self_.group0_.x) + (other_groups.group1_.x * self_.group0_.w), (other_groups.group0_.z * self_.group0_.x) + (other_groups.group0_.w * self_.group0_.y) + (other_groups.group1_.y * self_.group0_.w), (other_groups.group0_.x * self_.group0_.y) + (other_groups.group0_.w * self_.group0_.z) + (other_groups.group1_.z * self_.group0_.w), -(other_groups.group1_.y * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.z)) - (vec4<f32>(other_groups.group0_.z, other_groups.group0_.x, other_groups.group0_.y, other_groups.group1_.x) * self_.group0_.yzxx)
    );
    let geometric_product: Flector = flector_degroup(geometric_product_groups);
    return flector_geometricProduct_point(geometric_product, point_reverse(self));
}
fn point_sandwich_multiVector(self_: Point, other: MultiVector) -> MultiVector {
    let self_groups = point_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let geometric_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, other_groups.group4_.w * self_.group0_.w, 0.0, 0.0) + (vec4<f32>(vec2<f32>(self_.group0_.x), 0.0, 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group4_.x, 0.0, 0.0)) + (vec4<f32>(vec2<f32>(self_.group0_.y), 0.0, 0.0) * vec4<f32>(other_groups.group1_.y, other_groups.group4_.y, 0.0, 0.0)) + (vec4<f32>(vec2<f32>(self_.group0_.z), 0.0, 0.0) * vec4<f32>(other_groups.group1_.z, other_groups.group4_.z, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group3_.y * self_.group0_.z, other_groups.group3_.z * self_.group0_.x, other_groups.group3_.x * self_.group0_.y, -(other_groups.group2_.y * self_.group0_.y) - (other_groups.group2_.z * self_.group0_.z)) + (vec4<f32>(other_groups.group0_.x) * self_.group0_) - (vec4<f32>(other_groups.group3_.zxy.xyz, other_groups.group2_.x) * self_.group0_.yzxx), 
        /* e41, e42, e43 */ vec4<f32>((other_groups.group4_.z * self_.group0_.y) - (other_groups.group4_.y * self_.group0_.z), (other_groups.group4_.x * self_.group0_.z) - (other_groups.group4_.z * self_.group0_.x), (other_groups.group4_.y * self_.group0_.x) - (other_groups.group4_.x * self_.group0_.y), 0.0) + (vec4<f32>(vec3<f32>(self_.group0_.w), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0)) - (vec4<f32>(vec3<f32>(other_groups.group1_.w), 0.0) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((other_groups.group1_.z * self_.group0_.y) - (other_groups.group1_.y * self_.group0_.z), (other_groups.group1_.x * self_.group0_.z) - (other_groups.group1_.z * self_.group0_.x), (other_groups.group1_.y * self_.group0_.x) - (other_groups.group1_.x * self_.group0_.y), 0.0) - (vec4<f32>(vec3<f32>(other_groups.group4_.w), 0.0) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.y * self_.group0_.x) + (other_groups.group2_.y * self_.group0_.z) + (other_groups.group3_.x * self_.group0_.w), (other_groups.group0_.y * self_.group0_.y) + (other_groups.group2_.z * self_.group0_.x) + (other_groups.group3_.y * self_.group0_.w), (other_groups.group0_.y * self_.group0_.z) + (other_groups.group2_.x * self_.group0_.y) + (other_groups.group3_.z * self_.group0_.w), -(other_groups.group3_.y * self_.group0_.y) - (other_groups.group3_.z * self_.group0_.z)) - (vec4<f32>(other_groups.group2_.zxy.xyz, other_groups.group3_.x) * self_.group0_.yzxx)
    );
    let geometric_product: MultiVector = multiVector_degroup(geometric_product_groups);
    return multiVector_geometricProduct_point(geometric_product, point_reverse(self));
}
fn point_sandwich_origin(self_: Point, other: Origin) -> Flector {
    let self_groups = point_grouped(self_);
    let other_groups = origin_grouped(other);
    let geometric_product_groups: LineGroups = LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(other.e4), 0.0) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0)
    );
    let geometric_product: Line = line_degroup(geometric_product_groups);
    return line_geometricProduct_point(geometric_product, point_reverse(self));
}
fn point_sandwich_plane(self_: Point, other: Plane) -> Flector {
    let self_groups = point_grouped(self_);
    let other_groups = plane_grouped(other);
    let geometric_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other_groups.group0_.y * self_.group0_.z) * -1.0, (other_groups.group0_.z * self_.group0_.x) * -1.0, (other_groups.group0_.x * self_.group0_.y) * -1.0, (other_groups.group0_.y * self_.group0_.y) + (other_groups.group0_.z * self_.group0_.z) + (other_groups.group0_.w * self_.group0_.w)) + (other_groups.group0_.zxyx * self_.group0_.yzxx), 
        /* e23, e31, e12, scalar */ vec4<f32>(other_groups.group0_.w * self_.group0_.x * -1.0, other_groups.group0_.w * self_.group0_.y * -1.0, other_groups.group0_.w * self_.group0_.z * -1.0, 0.0)
    );
    let geometric_product: Motor = motor_degroup(geometric_product_groups);
    return motor_geometricProduct_point(geometric_product, point_reverse(self));
}
fn point_sandwich_point(self_: Point, other: Point) -> Flector {
    let self_groups = point_grouped(self_);
    let other_groups = point_grouped(other);
    let geometric_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other_groups.group0_.x * self_.group0_.w) - (other_groups.group0_.w * self_.group0_.x), (other_groups.group0_.y * self_.group0_.w) - (other_groups.group0_.w * self_.group0_.y), (other_groups.group0_.z * self_.group0_.w) - (other_groups.group0_.w * self_.group0_.z), 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>((other_groups.group0_.y * self_.group0_.z) * -1.0, (other_groups.group0_.z * self_.group0_.x) * -1.0, (other_groups.group0_.x * self_.group0_.y) * -1.0, (other_groups.group0_.y * self_.group0_.y) + (other_groups.group0_.z * self_.group0_.z)) + (other_groups.group0_.zxyx * self_.group0_.yzxx)
    );
    let geometric_product: Motor = motor_degroup(geometric_product_groups);
    return motor_geometricProduct_point(geometric_product, point_reverse(self));
}
fn point_sandwich_scalar(self_: Point, other: Scalar) -> Motor {
    let self_groups = point_grouped(self_);
    let other_groups = scalar_grouped(other);
    let geometric_product_groups: PointGroups = PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.scalar) * self_.group0_
    );
    let geometric_product: Point = point_degroup(geometric_product_groups);
    return point_geometricProduct_point(geometric_product, point_reverse(self));
}
fn scalar_sandwich_antiScalar(self_: Scalar, other: AntiScalar) -> AntiScalar {
    let self_groups = scalar_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    let geometric_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(other.e1234 * self.scalar, 0.0, 0.0, 0.0)
    );
    let geometric_product: AntiScalar = antiScalar_degroup(geometric_product_groups);
    return antiScalar_geometricProduct_scalar(geometric_product, scalar_reverse(self));
}
fn scalar_sandwich_dualNum(self_: Scalar, other: DualNum) -> DualNum {
    let self_groups = scalar_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let geometric_product_groups: DualNumGroups = DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(self.scalar), 0.0, 0.0) * other_groups.group0_
    );
    let geometric_product: DualNum = dualNum_degroup(geometric_product_groups);
    return dualNum_geometricProduct_scalar(geometric_product, scalar_reverse(self));
}
fn scalar_sandwich_flector(self_: Scalar, other: Flector) -> Flector {
    let self_groups = scalar_grouped(self_);
    let other_groups = flector_grouped(other);
    let geometric_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self.scalar) * other_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self.scalar) * other_groups.group1_
    );
    let geometric_product: Flector = flector_degroup(geometric_product_groups);
    return flector_geometricProduct_scalar(geometric_product, scalar_reverse(self));
}
fn scalar_sandwich_horizon(self_: Scalar, other: Horizon) -> Horizon {
    let self_groups = scalar_grouped(self_);
    let other_groups = horizon_grouped(other);
    let geometric_product_groups: HorizonGroups = HorizonGroups(
        /* e321 */ vec4<f32>(other.e321 * self.scalar, 0.0, 0.0, 0.0)
    );
    let geometric_product: Horizon = horizon_degroup(geometric_product_groups);
    return horizon_geometricProduct_scalar(geometric_product, scalar_reverse(self));
}
fn scalar_sandwich_line(self_: Scalar, other: Line) -> Line {
    let self_groups = scalar_grouped(self_);
    let other_groups = line_grouped(other);
    let geometric_product_groups: LineGroups = LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(self.scalar), 0.0) * other_groups.group0_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(self.scalar), 0.0) * other_groups.group1_
    );
    let geometric_product: Line = line_degroup(geometric_product_groups);
    return line_geometricProduct_scalar(geometric_product, scalar_reverse(self));
}
fn scalar_sandwich_motor(self_: Scalar, other: Motor) -> Motor {
    let self_groups = scalar_grouped(self_);
    let other_groups = motor_grouped(other);
    let geometric_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self.scalar) * other_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(self.scalar) * other_groups.group1_
    );
    let geometric_product: Motor = motor_degroup(geometric_product_groups);
    return motor_geometricProduct_scalar(geometric_product, scalar_reverse(self));
}
fn scalar_sandwich_multiVector(self_: Scalar, other: MultiVector) -> MultiVector {
    let self_groups = scalar_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let geometric_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(self.scalar), 0.0, 0.0) * other_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(self.scalar) * other_groups.group1_, 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(self.scalar), 0.0) * other_groups.group2_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(self.scalar), 0.0) * other_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self.scalar) * other_groups.group4_
    );
    let geometric_product: MultiVector = multiVector_degroup(geometric_product_groups);
    return multiVector_geometricProduct_scalar(geometric_product, scalar_reverse(self));
}
fn scalar_sandwich_origin(self_: Scalar, other: Origin) -> Origin {
    let self_groups = scalar_grouped(self_);
    let other_groups = origin_grouped(other);
    let geometric_product_groups: OriginGroups = OriginGroups(
        /* e4 */ vec4<f32>(other.e4 * self.scalar, 0.0, 0.0, 0.0)
    );
    let geometric_product: Origin = origin_degroup(geometric_product_groups);
    return origin_geometricProduct_scalar(geometric_product, scalar_reverse(self));
}
fn scalar_sandwich_plane(self_: Scalar, other: Plane) -> Plane {
    let self_groups = scalar_grouped(self_);
    let other_groups = plane_grouped(other);
    let geometric_product_groups: PlaneGroups = PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(self.scalar) * other_groups.group0_
    );
    let geometric_product: Plane = plane_degroup(geometric_product_groups);
    return plane_geometricProduct_scalar(geometric_product, scalar_reverse(self));
}
fn scalar_sandwich_point(self_: Scalar, other: Point) -> Point {
    let self_groups = scalar_grouped(self_);
    let other_groups = point_grouped(other);
    let geometric_product_groups: PointGroups = PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self.scalar) * other_groups.group0_
    );
    let geometric_product: Point = point_degroup(geometric_product_groups);
    return point_geometricProduct_scalar(geometric_product, scalar_reverse(self));
}
fn scalar_sandwich_scalar(self_: Scalar, other: Scalar) -> Scalar {
    let self_groups = scalar_grouped(self_);
    let other_groups = scalar_grouped(other);
    let geometric_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(other.scalar * self.scalar, 0.0, 0.0, 0.0)
    );
    let geometric_product: Scalar = scalar_degroup(geometric_product_groups);
    return scalar_geometricProduct_scalar(geometric_product, scalar_reverse(self));
}
fn scalar_squareRoot(self_: Scalar) -> Scalar {
    let self_groups = scalar_grouped(self_);    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(pow(self.scalar, 0.5), 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_sub_antiScalar(self_: AntiScalar, other: AntiScalar) -> AntiScalar {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(self.e1234 - other.e1234, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_sub_dualNum(self_: AntiScalar, other: DualNum) -> DualNum {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(other_groups.group0_.x * -1.0, self.e1234 - other_groups.group0_.y, 0.0, 0.0)
    ));
}
fn antiScalar_sub_flector(self_: AntiScalar, other: Flector) -> MultiVector {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = flector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, self.e1234, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ other_groups.group0_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ other_groups.group1_ * vec4<f32>(-1.0)
    ));
}
fn antiScalar_sub_horizon(self_: AntiScalar, other: Horizon) -> MultiVector {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = horizon_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, self.e1234, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other.e321 * -1.0)
    ));
}
fn antiScalar_sub_line(self_: AntiScalar, other: Line) -> Motor {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = line_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other_groups.group0_.x * -1.0, other_groups.group0_.y * -1.0, other_groups.group0_.z * -1.0, self.e1234), 
        /* e23, e31, e12, scalar */ vec4<f32>(other_groups.group1_.x * -1.0, other_groups.group1_.y * -1.0, other_groups.group1_.z * -1.0, 0.0)
    ));
}
fn antiScalar_sub_motor(self_: AntiScalar, other: Motor) -> Motor {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other_groups.group0_.x * -1.0, other_groups.group0_.y * -1.0, other_groups.group0_.z * -1.0, self.e1234 - other_groups.group0_.w), 
        /* e23, e31, e12, scalar */ other_groups.group1_ * vec4<f32>(-1.0)
    ));
}
fn antiScalar_sub_multiVector(self_: AntiScalar, other: MultiVector) -> MultiVector {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other_groups.group0_.x * -1.0, self.e1234 - other_groups.group0_.y, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ other_groups.group1_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ other_groups.group2_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ other_groups.group3_ * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ other_groups.group4_ * vec4<f32>(-1.0)
    ));
}
fn antiScalar_sub_origin(self_: AntiScalar, other: Origin) -> MultiVector {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = origin_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, self.e1234, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other.e4 * -1.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn antiScalar_sub_plane(self_: AntiScalar, other: Plane) -> MultiVector {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = plane_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, self.e1234, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ other_groups.group0_ * vec4<f32>(-1.0)
    ));
}
fn antiScalar_sub_point(self_: AntiScalar, other: Point) -> MultiVector {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = point_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, self.e1234, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ other_groups.group0_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn antiScalar_sub_scalar(self_: AntiScalar, other: Scalar) -> DualNum {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = scalar_grouped(other);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar * -1.0, self.e1234, 0.0, 0.0)
    ));
}
fn dualNum_sub_antiScalar(self_: DualNum, other: AntiScalar) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(self_.group0_.x, self_.group0_.y - other.e1234, 0.0, 0.0)
    ));
}
fn dualNum_sub_dualNum(self_: DualNum, other: DualNum) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ self_.group0_ - other_groups.group0_
    ));
}
fn dualNum_sub_flector(self_: DualNum, other: Flector) -> MultiVector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = flector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_.group0_, 
        /* e1, e2, e3, e4 */ other_groups.group0_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ other_groups.group1_ * vec4<f32>(-1.0)
    ));
}
fn dualNum_sub_horizon(self_: DualNum, other: Horizon) -> MultiVector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = horizon_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other.e321 * -1.0)
    ));
}
fn dualNum_sub_line(self_: DualNum, other: Line) -> Motor {
    let self_groups = dualNum_grouped(self_);
    let other_groups = line_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other_groups.group0_.x * -1.0, other_groups.group0_.y * -1.0, other_groups.group0_.z * -1.0, self_.group0_.y), 
        /* e23, e31, e12, scalar */ vec4<f32>(other_groups.group1_.x * -1.0, other_groups.group1_.y * -1.0, other_groups.group1_.z * -1.0, self_.group0_.x)
    ));
}
fn dualNum_sub_motor(self_: DualNum, other: Motor) -> Motor {
    let self_groups = dualNum_grouped(self_);
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other_groups.group0_.x * -1.0, other_groups.group0_.y * -1.0, other_groups.group0_.z * -1.0, self_.group0_.y - other_groups.group0_.w), 
        /* e23, e31, e12, scalar */ vec4<f32>(other_groups.group1_.x * -1.0, other_groups.group1_.y * -1.0, other_groups.group1_.z * -1.0, self_.group0_.x - other_groups.group1_.w)
    ));
}
fn dualNum_sub_multiVector(self_: DualNum, other: MultiVector) -> MultiVector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_.group0_ - other_groups.group0_, 
        /* e1, e2, e3, e4 */ other_groups.group1_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ other_groups.group2_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ other_groups.group3_ * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ other_groups.group4_ * vec4<f32>(-1.0)
    ));
}
fn dualNum_sub_origin(self_: DualNum, other: Origin) -> MultiVector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = origin_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other.e4 * -1.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn dualNum_sub_plane(self_: DualNum, other: Plane) -> MultiVector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = plane_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_.group0_, 
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
        /* scalar, e1234 */ self_.group0_, 
        /* e1, e2, e3, e4 */ other_groups.group0_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn dualNum_sub_scalar(self_: DualNum, other: Scalar) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let other_groups = scalar_grouped(other);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(self_.group0_.x - other.scalar, self_.group0_.y, 0.0, 0.0)
    ));
}
fn flector_sub_antiScalar(self_: Flector, other: AntiScalar) -> MultiVector {
    let self_groups = flector_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, other.e1234 * -1.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ self_.group0_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ self_.group1_
    ));
}
fn flector_sub_dualNum(self_: Flector, other: DualNum) -> MultiVector {
    let self_groups = flector_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_ * vec2<f32>(-1.0), 
        /* e1, e2, e3, e4 */ self_.group0_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ self_.group1_
    ));
}
fn flector_sub_flector(self_: Flector, other: Flector) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ self_.group0_ - other_groups.group0_, 
        /* e423, e431, e412, e321 */ self_.group1_ - other_groups.group1_
    ));
}
fn flector_sub_horizon(self_: Flector, other: Horizon) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = horizon_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ self_.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group1_.w - other.e321)
    ));
}
fn flector_sub_line(self_: Flector, other: Line) -> MultiVector {
    let self_groups = flector_grouped(self_);
    let other_groups = line_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ self_.group0_, 
        /* e41, e42, e43 */ other_groups.group0_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ other_groups.group1_ * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ self_.group1_
    ));
}
fn flector_sub_motor(self_: Flector, other: Motor) -> MultiVector {
    let self_groups = flector_grouped(self_);
    let other_groups = motor_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other_groups.group1_.w, other_groups.group0_.w, 0.0, 0.0) * vec2<f32>(-1.0), 
        /* e1, e2, e3, e4 */ self_.group0_, 
        /* e41, e42, e43 */ vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0) * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ self_.group1_
    ));
}
fn flector_sub_multiVector(self_: Flector, other: MultiVector) -> MultiVector {
    let self_groups = flector_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_ * vec2<f32>(-1.0), 
        /* e1, e2, e3, e4 */ self_.group0_ - other_groups.group1_, 
        /* e41, e42, e43 */ other_groups.group2_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ other_groups.group3_ * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ self_.group1_ - other_groups.group4_
    ));
}
fn flector_sub_origin(self_: Flector, other: Origin) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = origin_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group0_.w - other.e4), 
        /* e423, e431, e412, e321 */ self_.group1_
    ));
}
fn flector_sub_plane(self_: Flector, other: Plane) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = plane_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ self_.group0_, 
        /* e423, e431, e412, e321 */ self_.group1_ - other_groups.group0_
    ));
}
fn flector_sub_point(self_: Flector, other: Point) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = point_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ self_.group0_ - other_groups.group0_, 
        /* e423, e431, e412, e321 */ self_.group1_
    ));
}
fn flector_sub_scalar(self_: Flector, other: Scalar) -> MultiVector {
    let self_groups = flector_grouped(self_);
    let other_groups = scalar_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar * -1.0, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ self_.group0_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ self_.group1_
    ));
}
fn horizon_sub_antiScalar(self_: Horizon, other: AntiScalar) -> MultiVector {
    let self_groups = horizon_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, other.e1234 * -1.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self.e321)
    ));
}
fn horizon_sub_dualNum(self_: Horizon, other: DualNum) -> MultiVector {
    let self_groups = horizon_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_ * vec2<f32>(-1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self.e321)
    ));
}
fn horizon_sub_flector(self_: Horizon, other: Flector) -> Flector {
    let self_groups = horizon_grouped(self_);
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ other_groups.group0_ * vec4<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group1_.x * -1.0, other_groups.group1_.y * -1.0, other_groups.group1_.z * -1.0, self.e321 - other_groups.group1_.w)
    ));
}
fn horizon_sub_horizon(self_: Horizon, other: Horizon) -> Horizon {
    let self_groups = horizon_grouped(self_);
    let other_groups = horizon_grouped(other);
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(self.e321 - other.e321, 0.0, 0.0, 0.0)
    ));
}
fn horizon_sub_line(self_: Horizon, other: Line) -> MultiVector {
    let self_groups = horizon_grouped(self_);
    let other_groups = line_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ other_groups.group0_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ other_groups.group1_ * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self.e321)
    ));
}
fn horizon_sub_motor(self_: Horizon, other: Motor) -> MultiVector {
    let self_groups = horizon_grouped(self_);
    let other_groups = motor_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other_groups.group1_.w, other_groups.group0_.w, 0.0, 0.0) * vec2<f32>(-1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0) * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self.e321)
    ));
}
fn horizon_sub_multiVector(self_: Horizon, other: MultiVector) -> MultiVector {
    let self_groups = horizon_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_ * vec2<f32>(-1.0), 
        /* e1, e2, e3, e4 */ other_groups.group1_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ other_groups.group2_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ other_groups.group3_ * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group4_.x * -1.0, other_groups.group4_.y * -1.0, other_groups.group4_.z * -1.0, self.e321 - other_groups.group4_.w)
    ));
}
fn horizon_sub_origin(self_: Horizon, other: Origin) -> Flector {
    let self_groups = horizon_grouped(self_);
    let other_groups = origin_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other.e4 * -1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self.e321)
    ));
}
fn horizon_sub_plane(self_: Horizon, other: Plane) -> Plane {
    let self_groups = horizon_grouped(self_);
    let other_groups = plane_grouped(other);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group0_.x * -1.0, other_groups.group0_.y * -1.0, other_groups.group0_.z * -1.0, self.e321 - other_groups.group0_.w)
    ));
}
fn horizon_sub_point(self_: Horizon, other: Point) -> Flector {
    let self_groups = horizon_grouped(self_);
    let other_groups = point_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ other_groups.group0_ * vec4<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self.e321)
    ));
}
fn horizon_sub_scalar(self_: Horizon, other: Scalar) -> MultiVector {
    let self_groups = horizon_grouped(self_);
    let other_groups = scalar_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar * -1.0, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self.e321)
    ));
}
fn line_sub_antiScalar(self_: Line, other: AntiScalar) -> Motor {
    let self_groups = line_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.group0_.xyz.xyz, other.e1234 * -1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.group1_.xyz.xyz, 0.0)
    ));
}
fn line_sub_dualNum(self_: Line, other: DualNum) -> Motor {
    let self_groups = line_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.group0_.xyz.xyz, other_groups.group0_.y * -1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.group1_.xyz.xyz, other_groups.group0_.x * -1.0)
    ));
}
fn line_sub_flector(self_: Line, other: Flector) -> MultiVector {
    let self_groups = line_grouped(self_);
    let other_groups = flector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ other_groups.group0_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ self_.group0_, 
        /* e23, e31, e12 */ self_.group1_, 
        /* e423, e431, e412, e321 */ other_groups.group1_ * vec4<f32>(-1.0)
    ));
}
fn line_sub_horizon(self_: Line, other: Horizon) -> MultiVector {
    let self_groups = line_grouped(self_);
    let other_groups = horizon_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ self_.group0_, 
        /* e23, e31, e12 */ self_.group1_, 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other.e321 * -1.0)
    ));
}
fn line_sub_line(self_: Line, other: Line) -> Line {
    let self_groups = line_grouped(self_);
    let other_groups = line_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ self_.group0_ - other_groups.group0_, 
        /* e23, e31, e12 */ self_.group1_ - other_groups.group1_
    ));
}
fn line_sub_motor(self_: Line, other: Motor) -> Motor {
    let self_groups = line_grouped(self_);
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.group0_.x - other_groups.group0_.x, self_.group0_.y - other_groups.group0_.y, self_.group0_.z - other_groups.group0_.z, other_groups.group0_.w * -1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.group1_.x - other_groups.group1_.x, self_.group1_.y - other_groups.group1_.y, self_.group1_.z - other_groups.group1_.z, other_groups.group1_.w * -1.0)
    ));
}
fn line_sub_multiVector(self_: Line, other: MultiVector) -> MultiVector {
    let self_groups = line_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_ * vec2<f32>(-1.0), 
        /* e1, e2, e3, e4 */ other_groups.group1_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ self_.group0_ - other_groups.group2_, 
        /* e23, e31, e12 */ self_.group1_ - other_groups.group3_, 
        /* e423, e431, e412, e321 */ other_groups.group4_ * vec4<f32>(-1.0)
    ));
}
fn line_sub_origin(self_: Line, other: Origin) -> MultiVector {
    let self_groups = line_grouped(self_);
    let other_groups = origin_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other.e4 * -1.0), 
        /* e41, e42, e43 */ self_.group0_, 
        /* e23, e31, e12 */ self_.group1_, 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn line_sub_plane(self_: Line, other: Plane) -> MultiVector {
    let self_groups = line_grouped(self_);
    let other_groups = plane_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ self_.group0_, 
        /* e23, e31, e12 */ self_.group1_, 
        /* e423, e431, e412, e321 */ other_groups.group0_ * vec4<f32>(-1.0)
    ));
}
fn line_sub_point(self_: Line, other: Point) -> MultiVector {
    let self_groups = line_grouped(self_);
    let other_groups = point_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ other_groups.group0_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ self_.group0_, 
        /* e23, e31, e12 */ self_.group1_, 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn line_sub_scalar(self_: Line, other: Scalar) -> Motor {
    let self_groups = line_grouped(self_);
    let other_groups = scalar_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.group0_.xyz.xyz, 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.group1_.xyz.xyz, other.scalar * -1.0)
    ));
}
fn motor_sub_antiScalar(self_: Motor, other: AntiScalar) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group0_.w - other.e1234), 
        /* e23, e31, e12, scalar */ self_.group1_
    ));
}
fn motor_sub_dualNum(self_: Motor, other: DualNum) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group0_.w - other_groups.group0_.y), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group1_.w - other_groups.group0_.x)
    ));
}
fn motor_sub_flector(self_: Motor, other: Flector) -> MultiVector {
    let self_groups = motor_grouped(self_);
    let other_groups = flector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.group1_.w, self_.group0_.w, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ other_groups.group0_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0), 
        /* e423, e431, e412, e321 */ other_groups.group1_ * vec4<f32>(-1.0)
    ));
}
fn motor_sub_horizon(self_: Motor, other: Horizon) -> MultiVector {
    let self_groups = motor_grouped(self_);
    let other_groups = horizon_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.group1_.w, self_.group0_.w, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other.e321 * -1.0)
    ));
}
fn motor_sub_line(self_: Motor, other: Line) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = line_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.group0_.x - other_groups.group0_.x, self_.group0_.y - other_groups.group0_.y, self_.group0_.z - other_groups.group0_.z, self_.group0_.w), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.group1_.x - other_groups.group1_.x, self_.group1_.y - other_groups.group1_.y, self_.group1_.z - other_groups.group1_.z, self_.group1_.w)
    ));
}
fn motor_sub_motor(self_: Motor, other: Motor) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ self_.group0_ - other_groups.group0_, 
        /* e23, e31, e12, scalar */ self_.group1_ - other_groups.group1_
    ));
}
fn motor_sub_multiVector(self_: Motor, other: MultiVector) -> MultiVector {
    let self_groups = motor_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.group1_.w, self_.group0_.w, 0.0, 0.0) - other_groups.group0_, 
        /* e1, e2, e3, e4 */ other_groups.group1_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0) - other_groups.group2_, 
        /* e23, e31, e12 */ vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0) - other_groups.group3_, 
        /* e423, e431, e412, e321 */ other_groups.group4_ * vec4<f32>(-1.0)
    ));
}
fn motor_sub_origin(self_: Motor, other: Origin) -> MultiVector {
    let self_groups = motor_grouped(self_);
    let other_groups = origin_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.group1_.w, self_.group0_.w, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other.e4 * -1.0), 
        /* e41, e42, e43 */ vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn motor_sub_plane(self_: Motor, other: Plane) -> MultiVector {
    let self_groups = motor_grouped(self_);
    let other_groups = plane_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.group1_.w, self_.group0_.w, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0), 
        /* e423, e431, e412, e321 */ other_groups.group0_ * vec4<f32>(-1.0)
    ));
}
fn motor_sub_point(self_: Motor, other: Point) -> MultiVector {
    let self_groups = motor_grouped(self_);
    let other_groups = point_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.group1_.w, self_.group0_.w, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ other_groups.group0_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn motor_sub_scalar(self_: Motor, other: Scalar) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = scalar_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ self_.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group1_.w - other.scalar)
    ));
}
fn multiVector_sub_antiScalar(self_: MultiVector, other: AntiScalar) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.group0_.x, self_.group0_.y - other.e1234, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ self_.group1_, 
        /* e41, e42, e43 */ self_.group2_, 
        /* e23, e31, e12 */ self_.group3_, 
        /* e423, e431, e412, e321 */ self_.group4_
    ));
}
fn multiVector_sub_dualNum(self_: MultiVector, other: DualNum) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_.group0_ - other_groups.group0_, 
        /* e1, e2, e3, e4 */ self_.group1_, 
        /* e41, e42, e43 */ self_.group2_, 
        /* e23, e31, e12 */ self_.group3_, 
        /* e423, e431, e412, e321 */ self_.group4_
    ));
}
fn multiVector_sub_flector(self_: MultiVector, other: Flector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = flector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_.group0_, 
        /* e1, e2, e3, e4 */ self_.group1_ - other_groups.group0_, 
        /* e41, e42, e43 */ self_.group2_, 
        /* e23, e31, e12 */ self_.group3_, 
        /* e423, e431, e412, e321 */ self_.group4_ - other_groups.group1_
    ));
}
fn multiVector_sub_horizon(self_: MultiVector, other: Horizon) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = horizon_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_.group0_, 
        /* e1, e2, e3, e4 */ self_.group1_, 
        /* e41, e42, e43 */ self_.group2_, 
        /* e23, e31, e12 */ self_.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group4_.x, self_.group4_.y, self_.group4_.z, self_.group4_.w - other.e321)
    ));
}
fn multiVector_sub_line(self_: MultiVector, other: Line) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = line_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_.group0_, 
        /* e1, e2, e3, e4 */ self_.group1_, 
        /* e41, e42, e43 */ self_.group2_ - other_groups.group0_, 
        /* e23, e31, e12 */ self_.group3_ - other_groups.group1_, 
        /* e423, e431, e412, e321 */ self_.group4_
    ));
}
fn multiVector_sub_motor(self_: MultiVector, other: Motor) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = motor_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other_groups.group1_.w * -1.0, other_groups.group0_.w * -1.0, 0.0, 0.0) + self_.group0_, 
        /* e1, e2, e3, e4 */ self_.group1_, 
        /* e41, e42, e43 */ vec4<f32>(other_groups.group0_.x * -1.0, other_groups.group0_.y * -1.0, other_groups.group0_.z * -1.0, 0.0) + self_.group2_, 
        /* e23, e31, e12 */ vec4<f32>(other_groups.group1_.x * -1.0, other_groups.group1_.y * -1.0, other_groups.group1_.z * -1.0, 0.0) + self_.group3_, 
        /* e423, e431, e412, e321 */ self_.group4_
    ));
}
fn multiVector_sub_multiVector(self_: MultiVector, other: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_.group0_ - other_groups.group0_, 
        /* e1, e2, e3, e4 */ self_.group1_ - other_groups.group1_, 
        /* e41, e42, e43 */ self_.group2_ - other_groups.group2_, 
        /* e23, e31, e12 */ self_.group3_ - other_groups.group3_, 
        /* e423, e431, e412, e321 */ self_.group4_ - other_groups.group4_
    ));
}
fn multiVector_sub_origin(self_: MultiVector, other: Origin) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = origin_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group1_.w - other.e4), 
        /* e41, e42, e43 */ self_.group2_, 
        /* e23, e31, e12 */ self_.group3_, 
        /* e423, e431, e412, e321 */ self_.group4_
    ));
}
fn multiVector_sub_plane(self_: MultiVector, other: Plane) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = plane_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_.group0_, 
        /* e1, e2, e3, e4 */ self_.group1_, 
        /* e41, e42, e43 */ self_.group2_, 
        /* e23, e31, e12 */ self_.group3_, 
        /* e423, e431, e412, e321 */ self_.group4_ - other_groups.group0_
    ));
}
fn multiVector_sub_point(self_: MultiVector, other: Point) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = point_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ self_.group0_, 
        /* e1, e2, e3, e4 */ self_.group1_ - other_groups.group0_, 
        /* e41, e42, e43 */ self_.group2_, 
        /* e23, e31, e12 */ self_.group3_, 
        /* e423, e431, e412, e321 */ self_.group4_
    ));
}
fn multiVector_sub_scalar(self_: MultiVector, other: Scalar) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = scalar_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.group0_.x - other.scalar, self_.group0_.y, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ self_.group1_, 
        /* e41, e42, e43 */ self_.group2_, 
        /* e23, e31, e12 */ self_.group3_, 
        /* e423, e431, e412, e321 */ self_.group4_
    ));
}
fn origin_sub_antiScalar(self_: Origin, other: AntiScalar) -> MultiVector {
    let self_groups = origin_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, other.e1234 * -1.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self.e4), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn origin_sub_dualNum(self_: Origin, other: DualNum) -> MultiVector {
    let self_groups = origin_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_ * vec2<f32>(-1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self.e4), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn origin_sub_flector(self_: Origin, other: Flector) -> Flector {
    let self_groups = origin_grouped(self_);
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group0_.x * -1.0, other_groups.group0_.y * -1.0, other_groups.group0_.z * -1.0, self.e4 - other_groups.group0_.w), 
        /* e423, e431, e412, e321 */ other_groups.group1_ * vec4<f32>(-1.0)
    ));
}
fn origin_sub_horizon(self_: Origin, other: Horizon) -> Flector {
    let self_groups = origin_grouped(self_);
    let other_groups = horizon_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self.e4), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other.e321 * -1.0)
    ));
}
fn origin_sub_line(self_: Origin, other: Line) -> MultiVector {
    let self_groups = origin_grouped(self_);
    let other_groups = line_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self.e4), 
        /* e41, e42, e43 */ other_groups.group0_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ other_groups.group1_ * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn origin_sub_motor(self_: Origin, other: Motor) -> MultiVector {
    let self_groups = origin_grouped(self_);
    let other_groups = motor_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other_groups.group1_.w, other_groups.group0_.w, 0.0, 0.0) * vec2<f32>(-1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self.e4), 
        /* e41, e42, e43 */ vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0) * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn origin_sub_multiVector(self_: Origin, other: MultiVector) -> MultiVector {
    let self_groups = origin_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_ * vec2<f32>(-1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group1_.x * -1.0, other_groups.group1_.y * -1.0, other_groups.group1_.z * -1.0, self.e4 - other_groups.group1_.w), 
        /* e41, e42, e43 */ other_groups.group2_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ other_groups.group3_ * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ other_groups.group4_ * vec4<f32>(-1.0)
    ));
}
fn origin_sub_origin(self_: Origin, other: Origin) -> Origin {
    let self_groups = origin_grouped(self_);
    let other_groups = origin_grouped(other);
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(self.e4 - other.e4, 0.0, 0.0, 0.0)
    ));
}
fn origin_sub_plane(self_: Origin, other: Plane) -> Flector {
    let self_groups = origin_grouped(self_);
    let other_groups = plane_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self.e4), 
        /* e423, e431, e412, e321 */ other_groups.group0_ * vec4<f32>(-1.0)
    ));
}
fn origin_sub_point(self_: Origin, other: Point) -> Point {
    let self_groups = origin_grouped(self_);
    let other_groups = point_grouped(other);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group0_.x * -1.0, other_groups.group0_.y * -1.0, other_groups.group0_.z * -1.0, self.e4 - other_groups.group0_.w)
    ));
}
fn origin_sub_scalar(self_: Origin, other: Scalar) -> MultiVector {
    let self_groups = origin_grouped(self_);
    let other_groups = scalar_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar * -1.0, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self.e4), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn plane_sub_antiScalar(self_: Plane, other: AntiScalar) -> MultiVector {
    let self_groups = plane_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, other.e1234 * -1.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ self_.group0_
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
        /* e423, e431, e412, e321 */ self_.group0_
    ));
}
fn plane_sub_flector(self_: Plane, other: Flector) -> Flector {
    let self_groups = plane_grouped(self_);
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ other_groups.group0_ * vec4<f32>(-1.0), 
        /* e423, e431, e412, e321 */ self_.group0_ - other_groups.group1_
    ));
}
fn plane_sub_horizon(self_: Plane, other: Horizon) -> Plane {
    let self_groups = plane_grouped(self_);
    let other_groups = horizon_grouped(other);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group0_.w - other.e321)
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
        /* e423, e431, e412, e321 */ self_.group0_
    ));
}
fn plane_sub_motor(self_: Plane, other: Motor) -> MultiVector {
    let self_groups = plane_grouped(self_);
    let other_groups = motor_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other_groups.group1_.w, other_groups.group0_.w, 0.0, 0.0) * vec2<f32>(-1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0) * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ self_.group0_
    ));
}
fn plane_sub_multiVector(self_: Plane, other: MultiVector) -> MultiVector {
    let self_groups = plane_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_ * vec2<f32>(-1.0), 
        /* e1, e2, e3, e4 */ other_groups.group1_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ other_groups.group2_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ other_groups.group3_ * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ self_.group0_ - other_groups.group4_
    ));
}
fn plane_sub_origin(self_: Plane, other: Origin) -> Flector {
    let self_groups = plane_grouped(self_);
    let other_groups = origin_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other.e4 * -1.0), 
        /* e423, e431, e412, e321 */ self_.group0_
    ));
}
fn plane_sub_plane(self_: Plane, other: Plane) -> Plane {
    let self_groups = plane_grouped(self_);
    let other_groups = plane_grouped(other);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ self_.group0_ - other_groups.group0_
    ));
}
fn plane_sub_point(self_: Plane, other: Point) -> Flector {
    let self_groups = plane_grouped(self_);
    let other_groups = point_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ other_groups.group0_ * vec4<f32>(-1.0), 
        /* e423, e431, e412, e321 */ self_.group0_
    ));
}
fn plane_sub_scalar(self_: Plane, other: Scalar) -> MultiVector {
    let self_groups = plane_grouped(self_);
    let other_groups = scalar_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar * -1.0, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ self_.group0_
    ));
}
fn point_sub_antiScalar(self_: Point, other: AntiScalar) -> MultiVector {
    let self_groups = point_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, other.e1234 * -1.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ self_.group0_, 
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
        /* e1, e2, e3, e4 */ self_.group0_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn point_sub_flector(self_: Point, other: Flector) -> Flector {
    let self_groups = point_grouped(self_);
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ self_.group0_ - other_groups.group0_, 
        /* e423, e431, e412, e321 */ other_groups.group1_ * vec4<f32>(-1.0)
    ));
}
fn point_sub_horizon(self_: Point, other: Horizon) -> Flector {
    let self_groups = point_grouped(self_);
    let other_groups = horizon_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ self_.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other.e321 * -1.0)
    ));
}
fn point_sub_line(self_: Point, other: Line) -> MultiVector {
    let self_groups = point_grouped(self_);
    let other_groups = line_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0), 
        /* e1, e2, e3, e4 */ self_.group0_, 
        /* e41, e42, e43 */ other_groups.group0_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ other_groups.group1_ * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn point_sub_motor(self_: Point, other: Motor) -> MultiVector {
    let self_groups = point_grouped(self_);
    let other_groups = motor_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other_groups.group1_.w, other_groups.group0_.w, 0.0, 0.0) * vec2<f32>(-1.0), 
        /* e1, e2, e3, e4 */ self_.group0_, 
        /* e41, e42, e43 */ vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0) * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn point_sub_multiVector(self_: Point, other: MultiVector) -> MultiVector {
    let self_groups = point_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ other_groups.group0_ * vec2<f32>(-1.0), 
        /* e1, e2, e3, e4 */ self_.group0_ - other_groups.group1_, 
        /* e41, e42, e43 */ other_groups.group2_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ other_groups.group3_ * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ other_groups.group4_ * vec4<f32>(-1.0)
    ));
}
fn point_sub_origin(self_: Point, other: Origin) -> Point {
    let self_groups = point_grouped(self_);
    let other_groups = origin_grouped(other);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group0_.w - other.e4)
    ));
}
fn point_sub_plane(self_: Point, other: Plane) -> Flector {
    let self_groups = point_grouped(self_);
    let other_groups = plane_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ self_.group0_, 
        /* e423, e431, e412, e321 */ other_groups.group0_ * vec4<f32>(-1.0)
    ));
}
fn point_sub_point(self_: Point, other: Point) -> Point {
    let self_groups = point_grouped(self_);
    let other_groups = point_grouped(other);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ self_.group0_ - other_groups.group0_
    ));
}
fn point_sub_scalar(self_: Point, other: Scalar) -> MultiVector {
    let self_groups = point_grouped(self_);
    let other_groups = scalar_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other.scalar * -1.0, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ self_.group0_, 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn scalar_sub_antiScalar(self_: Scalar, other: AntiScalar) -> DualNum {
    let self_groups = scalar_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(self.scalar, other.e1234 * -1.0, 0.0, 0.0)
    ));
}
fn scalar_sub_dualNum(self_: Scalar, other: DualNum) -> DualNum {
    let self_groups = scalar_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(self.scalar - other_groups.group0_.x, other_groups.group0_.y * -1.0, 0.0, 0.0)
    ));
}
fn scalar_sub_flector(self_: Scalar, other: Flector) -> MultiVector {
    let self_groups = scalar_grouped(self_);
    let other_groups = flector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self.scalar, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ other_groups.group0_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ other_groups.group1_ * vec4<f32>(-1.0)
    ));
}
fn scalar_sub_horizon(self_: Scalar, other: Horizon) -> MultiVector {
    let self_groups = scalar_grouped(self_);
    let other_groups = horizon_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self.scalar, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other.e321 * -1.0)
    ));
}
fn scalar_sub_line(self_: Scalar, other: Line) -> Motor {
    let self_groups = scalar_grouped(self_);
    let other_groups = line_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other_groups.group0_.x * -1.0, other_groups.group0_.y * -1.0, other_groups.group0_.z * -1.0, 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(other_groups.group1_.x * -1.0, other_groups.group1_.y * -1.0, other_groups.group1_.z * -1.0, self.scalar)
    ));
}
fn scalar_sub_motor(self_: Scalar, other: Motor) -> Motor {
    let self_groups = scalar_grouped(self_);
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ other_groups.group0_ * vec4<f32>(-1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(other_groups.group1_.x * -1.0, other_groups.group1_.y * -1.0, other_groups.group1_.z * -1.0, self.scalar - other_groups.group1_.w)
    ));
}
fn scalar_sub_multiVector(self_: Scalar, other: MultiVector) -> MultiVector {
    let self_groups = scalar_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self.scalar - other_groups.group0_.x, other_groups.group0_.y * -1.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ other_groups.group1_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ other_groups.group2_ * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ other_groups.group3_ * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ other_groups.group4_ * vec4<f32>(-1.0)
    ));
}
fn scalar_sub_origin(self_: Scalar, other: Origin) -> MultiVector {
    let self_groups = scalar_grouped(self_);
    let other_groups = origin_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self.scalar, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other.e4 * -1.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn scalar_sub_plane(self_: Scalar, other: Plane) -> MultiVector {
    let self_groups = scalar_grouped(self_);
    let other_groups = plane_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self.scalar, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ other_groups.group0_ * vec4<f32>(-1.0)
    ));
}
fn scalar_sub_point(self_: Scalar, other: Point) -> MultiVector {
    let self_groups = scalar_grouped(self_);
    let other_groups = point_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self.scalar, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ other_groups.group0_ * vec4<f32>(-1.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0)
    ));
}
fn scalar_sub_scalar(self_: Scalar, other: Scalar) -> Scalar {
    let self_groups = scalar_grouped(self_);
    let other_groups = scalar_grouped(other);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(self.scalar - other.scalar, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_unit() -> AntiScalar {
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(1.0, 0.0, 0.0, 0.0)
    ));
}
fn dualNum_unit() -> DualNum {
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(1.0), 0.0, 0.0)
    ));
}
fn flector_unit() -> Flector {
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0)
    ));
}
fn horizon_unit() -> Horizon {
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(1.0, 0.0, 0.0, 0.0)
    ));
}
fn line_unit() -> Line {
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(1.0), 0.0), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(1.0), 0.0)
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
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(1.0), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(1.0), 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(1.0), 0.0), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(1.0), 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(1.0)
    ));
}
fn origin_unit() -> Origin {
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(1.0, 0.0, 0.0, 0.0)
    ));
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
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(1.0, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_wedge_dualNum(self_: AntiScalar, other: DualNum) -> AntiScalar {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(other_groups.group0_.x * self.e1234, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_wedge_motor(self_: AntiScalar, other: Motor) -> AntiScalar {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = motor_grouped(other);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(other_groups.group1_.w * self.e1234, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_wedge_multiVector(self_: AntiScalar, other: MultiVector) -> AntiScalar {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(other_groups.group0_.x * self.e1234, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_wedge_scalar(self_: AntiScalar, other: Scalar) -> AntiScalar {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = scalar_grouped(other);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(self.e1234 * other.scalar, 0.0, 0.0, 0.0)
    ));
}
fn dualNum_wedge_antiScalar(self_: DualNum, other: AntiScalar) -> AntiScalar {
    let self_groups = dualNum_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(self_.group0_.x * other.e1234, 0.0, 0.0, 0.0)
    ));
}
fn dualNum_wedge_dualNum(self_: DualNum, other: DualNum) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(other_groups.group0_.x * self_.group0_.x, (other_groups.group0_.x * self_.group0_.y) + (other_groups.group0_.y * self_.group0_.x), 0.0, 0.0)
    ));
}
fn dualNum_wedge_flector(self_: DualNum, other: Flector) -> Flector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group0_.x) * other_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.x) * other_groups.group1_
    ));
}
fn dualNum_wedge_horizon(self_: DualNum, other: Horizon) -> Horizon {
    let self_groups = dualNum_grouped(self_);
    let other_groups = horizon_grouped(other);
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(self_.group0_.x * other.e321, 0.0, 0.0, 0.0)
    ));
}
fn dualNum_wedge_line(self_: DualNum, other: Line) -> Line {
    let self_groups = dualNum_grouped(self_);
    let other_groups = line_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(self_.group0_.x), 0.0) * other_groups.group0_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(self_.group0_.x), 0.0) * other_groups.group1_
    ));
}
fn dualNum_wedge_motor(self_: DualNum, other: Motor) -> Motor {
    let self_groups = dualNum_grouped(self_);
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.group0_.x * other_groups.group0_.x, self_.group0_.x * other_groups.group0_.y, self_.group0_.x * other_groups.group0_.z, (self_.group0_.x * other_groups.group0_.w) + (self_.group0_.y * other_groups.group1_.w)), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.group0_.x) * other_groups.group1_
    ));
}
fn dualNum_wedge_multiVector(self_: DualNum, other: MultiVector) -> MultiVector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.group0_.x * other_groups.group0_.x, (self_.group0_.x * other_groups.group0_.y) + (self_.group0_.y * other_groups.group0_.x), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group0_.x) * other_groups.group1_, 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(self_.group0_.x), 0.0) * other_groups.group2_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(self_.group0_.x), 0.0) * other_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.x) * other_groups.group4_
    ));
}
fn dualNum_wedge_origin(self_: DualNum, other: Origin) -> Origin {
    let self_groups = dualNum_grouped(self_);
    let other_groups = origin_grouped(other);
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(self_.group0_.x * other.e4, 0.0, 0.0, 0.0)
    ));
}
fn dualNum_wedge_plane(self_: DualNum, other: Plane) -> Plane {
    let self_groups = dualNum_grouped(self_);
    let other_groups = plane_grouped(other);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.x) * other_groups.group0_
    ));
}
fn dualNum_wedge_point(self_: DualNum, other: Point) -> Point {
    let self_groups = dualNum_grouped(self_);
    let other_groups = point_grouped(other);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group0_.x) * other_groups.group0_
    ));
}
fn dualNum_wedge_scalar(self_: DualNum, other: Scalar) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let other_groups = scalar_grouped(other);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(other.scalar), 0.0, 0.0) * self_.group0_
    ));
}
fn flector_wedge_dualNum(self_: Flector, other: DualNum) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group0_.x) * self_.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group0_.x) * self_.group1_
    ));
}
fn flector_wedge_flector(self_: Flector, other: Flector) -> Motor {
    let self_groups = flector_grouped(self_);
    let other_groups = flector_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, (other_groups.group1_.y * self_.group0_.y) + (other_groups.group1_.z * self_.group0_.z) + (other_groups.group1_.w * self_.group0_.w) - (other_groups.group0_.y * self_.group1_.y) - (other_groups.group0_.z * self_.group1_.z) - (other_groups.group0_.w * self_.group1_.w)) + (vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, other_groups.group1_.x) * self_.group0_.wwwx) - (vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group1_.x) * other_groups.group0_.wwwx), 
        /* e23, e31, e12, scalar */ vec4<f32>((other_groups.group0_.z * self_.group0_.y) - (other_groups.group0_.y * self_.group0_.z), (other_groups.group0_.x * self_.group0_.z) - (other_groups.group0_.z * self_.group0_.x), (other_groups.group0_.y * self_.group0_.x) - (other_groups.group0_.x * self_.group0_.y), 0.0)
    ));
}
fn flector_wedge_horizon(self_: Flector, other: Horizon) -> AntiScalar {
    let self_groups = flector_grouped(self_);
    let other_groups = horizon_grouped(other);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(self_.group0_.w * other.e321, 0.0, 0.0, 0.0)
    ));
}
fn flector_wedge_line(self_: Flector, other: Line) -> Plane {
    let self_groups = flector_grouped(self_);
    let other_groups = line_grouped(other);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.y * self_.group0_.z) + (other_groups.group1_.x * self_.group0_.w), (other_groups.group0_.z * self_.group0_.x) + (other_groups.group1_.y * self_.group0_.w), (other_groups.group0_.x * self_.group0_.y) + (other_groups.group1_.z * self_.group0_.w), -(other_groups.group1_.y * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.z)) - (vec4<f32>(other_groups.group0_.zxy.xyz, other_groups.group1_.x) * self_.group0_.yzxx)
    ));
}
fn flector_wedge_motor(self_: Flector, other: Motor) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = motor_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group1_.w) * self_.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.group0_.z * other_groups.group0_.y) + (self_.group1_.x * other_groups.group1_.w), (self_.group0_.x * other_groups.group0_.z) + (self_.group1_.y * other_groups.group1_.w), (self_.group0_.y * other_groups.group0_.x) + (self_.group1_.z * other_groups.group1_.w), -(self_.group0_.y * other_groups.group1_.y) - (self_.group0_.z * other_groups.group1_.z)) + (vec4<f32>(self_.group0_.w, self_.group0_.w, self_.group0_.w, self_.group1_.w) * other_groups.group1_) - (vec4<f32>(other_groups.group0_.z, other_groups.group0_.x, other_groups.group0_.y, other_groups.group1_.x) * self_.group0_.yzxx)
    ));
}
fn flector_wedge_multiVector(self_: Flector, other: MultiVector) -> MultiVector {
    let self_groups = flector_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, (self_.group0_.x * other_groups.group4_.x) + (self_.group0_.y * other_groups.group4_.y) + (self_.group0_.z * other_groups.group4_.z) + (self_.group0_.w * other_groups.group4_.w) - (self_.group1_.x * other_groups.group1_.x) - (self_.group1_.y * other_groups.group1_.y) - (self_.group1_.z * other_groups.group1_.z) - (self_.group1_.w * other_groups.group1_.w), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group0_.x) * self_.group0_, 
        /* e41, e42, e43 */ (vec4<f32>(vec3<f32>(self_.group0_.w), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0)) - (vec4<f32>(vec3<f32>(other_groups.group1_.w), 0.0) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((self_.group0_.y * other_groups.group1_.z) - (self_.group0_.z * other_groups.group1_.y), (self_.group0_.z * other_groups.group1_.x) - (self_.group0_.x * other_groups.group1_.z), (self_.group0_.x * other_groups.group1_.y) - (self_.group0_.y * other_groups.group1_.x), 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group2_.y * self_.group0_.z) + (other_groups.group3_.x * self_.group0_.w), (other_groups.group2_.z * self_.group0_.x) + (other_groups.group3_.y * self_.group0_.w), (other_groups.group2_.x * self_.group0_.y) + (other_groups.group3_.z * self_.group0_.w), -(other_groups.group3_.y * self_.group0_.y) - (other_groups.group3_.z * self_.group0_.z)) + (vec4<f32>(other_groups.group0_.x) * self_.group1_) - (vec4<f32>(other_groups.group2_.zxy.xyz, other_groups.group3_.x) * self_.group0_.yzxx)
    ));
}
fn flector_wedge_origin(self_: Flector, other: Origin) -> Motor {
    let self_groups = flector_grouped(self_);
    let other_groups = origin_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e4) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group1_.w) * vec4<f32>(-1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0)
    ));
}
fn flector_wedge_plane(self_: Flector, other: Plane) -> AntiScalar {
    let self_groups = flector_grouped(self_);
    let other_groups = plane_grouped(other);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>((self_.group0_.x * other_groups.group0_.x) + (self_.group0_.y * other_groups.group0_.y) + (self_.group0_.z * other_groups.group0_.z) + (self_.group0_.w * other_groups.group0_.w), 0.0, 0.0, 0.0)
    ));
}
fn flector_wedge_point(self_: Flector, other: Point) -> Motor {
    let self_groups = flector_grouped(self_);
    let other_groups = point_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.group0_.w * other_groups.group0_.x, self_.group0_.w * other_groups.group0_.y, self_.group0_.w * other_groups.group0_.z, -(self_.group1_.y * other_groups.group0_.y) - (self_.group1_.z * other_groups.group0_.z) - (self_.group1_.w * other_groups.group0_.w)) - (vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group1_.x) * other_groups.group0_.wwwx), 
        /* e23, e31, e12, scalar */ vec4<f32>((self_.group0_.y * other_groups.group0_.z) - (self_.group0_.z * other_groups.group0_.y), (self_.group0_.z * other_groups.group0_.x) - (self_.group0_.x * other_groups.group0_.z), (self_.group0_.x * other_groups.group0_.y) - (self_.group0_.y * other_groups.group0_.x), 0.0)
    ));
}
fn flector_wedge_scalar(self_: Flector, other: Scalar) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = scalar_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.scalar) * self_.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other.scalar) * self_.group1_
    ));
}
fn horizon_wedge_dualNum(self_: Horizon, other: DualNum) -> Horizon {
    let self_groups = horizon_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(other_groups.group0_.x * self.e321, 0.0, 0.0, 0.0)
    ));
}
fn horizon_wedge_flector(self_: Horizon, other: Flector) -> AntiScalar {
    let self_groups = horizon_grouped(self_);
    let other_groups = flector_grouped(other);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(other_groups.group0_.w * self.e321 * -1.0, 0.0, 0.0, 0.0)
    ));
}
fn horizon_wedge_motor(self_: Horizon, other: Motor) -> Horizon {
    let self_groups = horizon_grouped(self_);
    let other_groups = motor_grouped(other);
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(other_groups.group1_.w * self.e321, 0.0, 0.0, 0.0)
    ));
}
fn horizon_wedge_multiVector(self_: Horizon, other: MultiVector) -> MultiVector {
    let self_groups = horizon_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, other_groups.group1_.w * self.e321 * -1.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group0_.x * self.e321)
    ));
}
fn horizon_wedge_origin(self_: Horizon, other: Origin) -> AntiScalar {
    let self_groups = horizon_grouped(self_);
    let other_groups = origin_grouped(other);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(self.e321 * other.e4 * -1.0, 0.0, 0.0, 0.0)
    ));
}
fn horizon_wedge_point(self_: Horizon, other: Point) -> AntiScalar {
    let self_groups = horizon_grouped(self_);
    let other_groups = point_grouped(other);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(other_groups.group0_.w * self.e321 * -1.0, 0.0, 0.0, 0.0)
    ));
}
fn horizon_wedge_scalar(self_: Horizon, other: Scalar) -> Horizon {
    let self_groups = horizon_grouped(self_);
    let other_groups = scalar_grouped(other);
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(self.e321 * other.scalar, 0.0, 0.0, 0.0)
    ));
}
fn line_wedge_dualNum(self_: Line, other: DualNum) -> Line {
    let self_groups = line_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(other_groups.group0_.x), 0.0) * self_.group0_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(other_groups.group0_.x), 0.0) * self_.group1_
    ));
}
fn line_wedge_flector(self_: Line, other: Flector) -> Plane {
    let self_groups = line_grouped(self_);
    let other_groups = flector_grouped(other);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>((self_.group0_.y * other_groups.group0_.z) + (self_.group1_.x * other_groups.group0_.w), (self_.group0_.z * other_groups.group0_.x) + (self_.group1_.y * other_groups.group0_.w), (self_.group0_.x * other_groups.group0_.y) + (self_.group1_.z * other_groups.group0_.w), -(self_.group1_.y * other_groups.group0_.y) - (self_.group1_.z * other_groups.group0_.z)) - (vec4<f32>(self_.group0_.zxy.xyz, self_.group1_.x) * other_groups.group0_.yzxx)
    ));
}
fn line_wedge_line(self_: Line, other: Line) -> AntiScalar {
    let self_groups = line_grouped(self_);
    let other_groups = line_grouped(other);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(-(other_groups.group0_.x * self_.group1_.x) - (other_groups.group0_.y * self_.group1_.y) - (other_groups.group0_.z * self_.group1_.z) - (other_groups.group1_.x * self_.group0_.x) - (other_groups.group1_.y * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.z), 0.0, 0.0, 0.0)
    ));
}
fn line_wedge_motor(self_: Line, other: Motor) -> Motor {
    let self_groups = line_grouped(self_);
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.group0_.x * other_groups.group1_.w, self_.group0_.y * other_groups.group1_.w, self_.group0_.z * other_groups.group1_.w, -(self_.group0_.x * other_groups.group1_.x) - (self_.group0_.y * other_groups.group1_.y) - (self_.group0_.z * other_groups.group1_.z) - (self_.group1_.x * other_groups.group0_.x) - (self_.group1_.y * other_groups.group0_.y) - (self_.group1_.z * other_groups.group0_.z)), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.group1_.x * other_groups.group1_.w, self_.group1_.y * other_groups.group1_.w, self_.group1_.z * other_groups.group1_.w, 0.0)
    ));
}
fn line_wedge_multiVector(self_: Line, other: MultiVector) -> MultiVector {
    let self_groups = line_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, -(self_.group0_.x * other_groups.group3_.x) - (self_.group0_.y * other_groups.group3_.y) - (self_.group0_.z * other_groups.group3_.z) - (self_.group1_.x * other_groups.group2_.x) - (self_.group1_.y * other_groups.group2_.y) - (self_.group1_.z * other_groups.group2_.z), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(other_groups.group0_.x), 0.0) * self_.group0_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(other_groups.group0_.x), 0.0) * self_.group1_, 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.group0_.y * other_groups.group1_.z) + (self_.group1_.x * other_groups.group1_.w), (self_.group0_.z * other_groups.group1_.x) + (self_.group1_.y * other_groups.group1_.w), (self_.group0_.x * other_groups.group1_.y) + (self_.group1_.z * other_groups.group1_.w), -(self_.group1_.y * other_groups.group1_.y) - (self_.group1_.z * other_groups.group1_.z)) - (vec4<f32>(self_.group0_.zxy.xyz, self_.group1_.x) * other_groups.group1_.yzxx)
    ));
}
fn line_wedge_origin(self_: Line, other: Origin) -> Plane {
    let self_groups = line_grouped(self_);
    let other_groups = origin_grouped(other);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group1_.x * other.e4, self_.group1_.y * other.e4, self_.group1_.z * other.e4, 0.0)
    ));
}
fn line_wedge_point(self_: Line, other: Point) -> Plane {
    let self_groups = line_grouped(self_);
    let other_groups = point_grouped(other);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>((self_.group0_.y * other_groups.group0_.z) + (self_.group1_.x * other_groups.group0_.w), (self_.group0_.z * other_groups.group0_.x) + (self_.group1_.y * other_groups.group0_.w), (self_.group0_.x * other_groups.group0_.y) + (self_.group1_.z * other_groups.group0_.w), -(self_.group1_.y * other_groups.group0_.y) - (self_.group1_.z * other_groups.group0_.z)) - (vec4<f32>(self_.group0_.zxy.xyz, self_.group1_.x) * other_groups.group0_.yzxx)
    ));
}
fn line_wedge_scalar(self_: Line, other: Scalar) -> Line {
    let self_groups = line_grouped(self_);
    let other_groups = scalar_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(other.scalar), 0.0) * self_.group0_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(other.scalar), 0.0) * self_.group1_
    ));
}
fn motor_wedge_antiScalar(self_: Motor, other: AntiScalar) -> AntiScalar {
    let self_groups = motor_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(self_.group1_.w * other.e1234, 0.0, 0.0, 0.0)
    ));
}
fn motor_wedge_dualNum(self_: Motor, other: DualNum) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other_groups.group0_.x * self_.group0_.x, other_groups.group0_.x * self_.group0_.y, other_groups.group0_.x * self_.group0_.z, (other_groups.group0_.x * self_.group0_.w) + (other_groups.group0_.y * self_.group1_.w)), 
        /* e23, e31, e12, scalar */ vec4<f32>(other_groups.group0_.x) * self_.group1_
    ));
}
fn motor_wedge_flector(self_: Motor, other: Flector) -> Flector {
    let self_groups = motor_grouped(self_);
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group1_.w) * other_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.z * self_.group0_.y) + (other_groups.group1_.x * self_.group1_.w), (other_groups.group0_.x * self_.group0_.z) + (other_groups.group1_.y * self_.group1_.w), (other_groups.group0_.y * self_.group0_.x) + (other_groups.group1_.z * self_.group1_.w), -(other_groups.group0_.y * self_.group1_.y) - (other_groups.group0_.z * self_.group1_.z)) + (vec4<f32>(other_groups.group0_.w, other_groups.group0_.w, other_groups.group0_.w, other_groups.group1_.w) * self_.group1_) - (vec4<f32>(self_.group0_.z, self_.group0_.x, self_.group0_.y, self_.group1_.x) * other_groups.group0_.yzxx)
    ));
}
fn motor_wedge_horizon(self_: Motor, other: Horizon) -> Horizon {
    let self_groups = motor_grouped(self_);
    let other_groups = horizon_grouped(other);
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(self_.group1_.w * other.e321, 0.0, 0.0, 0.0)
    ));
}
fn motor_wedge_line(self_: Motor, other: Line) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = line_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other_groups.group0_.x * self_.group1_.w, other_groups.group0_.y * self_.group1_.w, other_groups.group0_.z * self_.group1_.w, -(other_groups.group0_.x * self_.group1_.x) - (other_groups.group0_.y * self_.group1_.y) - (other_groups.group0_.z * self_.group1_.z) - (other_groups.group1_.x * self_.group0_.x) - (other_groups.group1_.y * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.z)), 
        /* e23, e31, e12, scalar */ vec4<f32>(other_groups.group1_.x * self_.group1_.w, other_groups.group1_.y * self_.group1_.w, other_groups.group1_.z * self_.group1_.w, 0.0)
    ));
}
fn motor_wedge_motor(self_: Motor, other: Motor) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, -(other_groups.group0_.x * self_.group1_.x) - (other_groups.group0_.y * self_.group1_.y) - (other_groups.group0_.z * self_.group1_.z) - (other_groups.group1_.x * self_.group0_.x) - (other_groups.group1_.y * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.z)) + (vec4<f32>(other_groups.group1_.w) * self_.group0_) + (vec4<f32>(self_.group1_.w) * other_groups.group0_), 
        /* e23, e31, e12, scalar */ vec4<f32>((other_groups.group1_.x * self_.group1_.w) + (other_groups.group1_.w * self_.group1_.x), (other_groups.group1_.y * self_.group1_.w) + (other_groups.group1_.w * self_.group1_.y), (other_groups.group1_.z * self_.group1_.w) + (other_groups.group1_.w * self_.group1_.z), other_groups.group1_.w * self_.group1_.w)
    ));
}
fn motor_wedge_multiVector(self_: Motor, other: MultiVector) -> MultiVector {
    let self_groups = motor_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other_groups.group0_.x * self_.group1_.w, (other_groups.group0_.x * self_.group0_.w) + (other_groups.group0_.y * self_.group1_.w) - (other_groups.group2_.x * self_.group1_.x) - (other_groups.group2_.y * self_.group1_.y) - (other_groups.group2_.z * self_.group1_.z) - (other_groups.group3_.x * self_.group0_.x) - (other_groups.group3_.y * self_.group0_.y) - (other_groups.group3_.z * self_.group0_.z), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group1_.w) * other_groups.group1_, 
        /* e41, e42, e43 */ (vec4<f32>(vec3<f32>(other_groups.group0_.x), 0.0) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0)) + (vec4<f32>(vec3<f32>(self_.group1_.w), 0.0) * other_groups.group2_), 
        /* e23, e31, e12 */ (vec4<f32>(vec3<f32>(other_groups.group0_.x), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0)) + (vec4<f32>(vec3<f32>(self_.group1_.w), 0.0) * other_groups.group3_), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.group0_.y * other_groups.group1_.z) + (self_.group1_.w * other_groups.group4_.x), (self_.group0_.z * other_groups.group1_.x) + (self_.group1_.w * other_groups.group4_.y), (self_.group0_.x * other_groups.group1_.y) + (self_.group1_.w * other_groups.group4_.z), -(self_.group1_.y * other_groups.group1_.y) - (self_.group1_.z * other_groups.group1_.z)) + (vec4<f32>(other_groups.group1_.w, other_groups.group1_.w, other_groups.group1_.w, other_groups.group4_.w) * self_.group1_) - (vec4<f32>(self_.group0_.z, self_.group0_.x, self_.group0_.y, self_.group1_.x) * other_groups.group1_.yzxx)
    ));
}
fn motor_wedge_origin(self_: Motor, other: Origin) -> Flector {
    let self_groups = motor_grouped(self_);
    let other_groups = origin_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.group1_.w * other.e4), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group1_.x * other.e4, self_.group1_.y * other.e4, self_.group1_.z * other.e4, 0.0)
    ));
}
fn motor_wedge_plane(self_: Motor, other: Plane) -> Plane {
    let self_groups = motor_grouped(self_);
    let other_groups = plane_grouped(other);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group1_.w) * other_groups.group0_
    ));
}
fn motor_wedge_point(self_: Motor, other: Point) -> Flector {
    let self_groups = motor_grouped(self_);
    let other_groups = point_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group1_.w) * other_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.group0_.y * other_groups.group0_.z) + (self_.group1_.x * other_groups.group0_.w), (self_.group0_.z * other_groups.group0_.x) + (self_.group1_.y * other_groups.group0_.w), (self_.group0_.x * other_groups.group0_.y) + (self_.group1_.z * other_groups.group0_.w), -(self_.group1_.y * other_groups.group0_.y) - (self_.group1_.z * other_groups.group0_.z)) - (vec4<f32>(self_.group0_.z, self_.group0_.x, self_.group0_.y, self_.group1_.x) * other_groups.group0_.yzxx)
    ));
}
fn motor_wedge_scalar(self_: Motor, other: Scalar) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = scalar_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.scalar) * self_.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(other.scalar) * self_.group1_
    ));
}
fn multiVector_wedge_antiScalar(self_: MultiVector, other: AntiScalar) -> AntiScalar {
    let self_groups = multiVector_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(self_.group0_.x * other.e1234, 0.0, 0.0, 0.0)
    ));
}
fn multiVector_wedge_dualNum(self_: MultiVector, other: DualNum) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other_groups.group0_.x * self_.group0_.x, (other_groups.group0_.x * self_.group0_.y) + (other_groups.group0_.y * self_.group0_.x), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group0_.x) * self_.group1_, 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(other_groups.group0_.x), 0.0) * self_.group2_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(other_groups.group0_.x), 0.0) * self_.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group0_.x) * self_.group4_
    ));
}
fn multiVector_wedge_flector(self_: MultiVector, other: Flector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = flector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, (other_groups.group1_.x * self_.group1_.x) + (other_groups.group1_.y * self_.group1_.y) + (other_groups.group1_.z * self_.group1_.z) + (other_groups.group1_.w * self_.group1_.w) - (other_groups.group0_.x * self_.group4_.x) - (other_groups.group0_.y * self_.group4_.y) - (other_groups.group0_.z * self_.group4_.z) - (other_groups.group0_.w * self_.group4_.w), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group0_.x) * other_groups.group0_, 
        /* e41, e42, e43 */ (vec4<f32>(vec3<f32>(self_.group1_.w), 0.0) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0)) - (vec4<f32>(vec3<f32>(other_groups.group0_.w), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((other_groups.group0_.z * self_.group1_.y) - (other_groups.group0_.y * self_.group1_.z), (other_groups.group0_.x * self_.group1_.z) - (other_groups.group0_.z * self_.group1_.x), (other_groups.group0_.y * self_.group1_.x) - (other_groups.group0_.x * self_.group1_.y), 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.group2_.y * other_groups.group0_.z) + (self_.group3_.x * other_groups.group0_.w), (self_.group2_.z * other_groups.group0_.x) + (self_.group3_.y * other_groups.group0_.w), (self_.group2_.x * other_groups.group0_.y) + (self_.group3_.z * other_groups.group0_.w), -(self_.group3_.y * other_groups.group0_.y) - (self_.group3_.z * other_groups.group0_.z)) + (vec4<f32>(self_.group0_.x) * other_groups.group1_) - (vec4<f32>(self_.group2_.zxy.xyz, self_.group3_.x) * other_groups.group0_.yzxx)
    ));
}
fn multiVector_wedge_horizon(self_: MultiVector, other: Horizon) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = horizon_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, self_.group1_.w * other.e321, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.group0_.x * other.e321)
    ));
}
fn multiVector_wedge_line(self_: MultiVector, other: Line) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = line_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, -(other_groups.group0_.x * self_.group3_.x) - (other_groups.group0_.y * self_.group3_.y) - (other_groups.group0_.z * self_.group3_.z) - (other_groups.group1_.x * self_.group2_.x) - (other_groups.group1_.y * self_.group2_.y) - (other_groups.group1_.z * self_.group2_.z), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(self_.group0_.x), 0.0) * other_groups.group0_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(self_.group0_.x), 0.0) * other_groups.group1_, 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.y * self_.group1_.z) + (other_groups.group1_.x * self_.group1_.w), (other_groups.group0_.z * self_.group1_.x) + (other_groups.group1_.y * self_.group1_.w), (other_groups.group0_.x * self_.group1_.y) + (other_groups.group1_.z * self_.group1_.w), -(other_groups.group1_.y * self_.group1_.y) - (other_groups.group1_.z * self_.group1_.z)) - (vec4<f32>(other_groups.group0_.zxy.xyz, other_groups.group1_.x) * self_.group1_.yzxx)
    ));
}
fn multiVector_wedge_motor(self_: MultiVector, other: Motor) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = motor_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.group0_.x * other_groups.group1_.w, (self_.group0_.x * other_groups.group0_.w) + (self_.group0_.y * other_groups.group1_.w) - (self_.group2_.x * other_groups.group1_.x) - (self_.group2_.y * other_groups.group1_.y) - (self_.group2_.z * other_groups.group1_.z) - (self_.group3_.x * other_groups.group0_.x) - (self_.group3_.y * other_groups.group0_.y) - (self_.group3_.z * other_groups.group0_.z), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group1_.w) * self_.group1_, 
        /* e41, e42, e43 */ (vec4<f32>(vec3<f32>(self_.group0_.x), 0.0) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0)) + (vec4<f32>(vec3<f32>(other_groups.group1_.w), 0.0) * self_.group2_), 
        /* e23, e31, e12 */ (vec4<f32>(vec3<f32>(self_.group0_.x), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0)) + (vec4<f32>(vec3<f32>(other_groups.group1_.w), 0.0) * self_.group3_), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.y * self_.group1_.z) + (other_groups.group1_.w * self_.group4_.x), (other_groups.group0_.z * self_.group1_.x) + (other_groups.group1_.w * self_.group4_.y), (other_groups.group0_.x * self_.group1_.y) + (other_groups.group1_.w * self_.group4_.z), -(other_groups.group1_.y * self_.group1_.y) - (other_groups.group1_.z * self_.group1_.z)) + (vec4<f32>(self_.group1_.w, self_.group1_.w, self_.group1_.w, self_.group4_.w) * other_groups.group1_) - (vec4<f32>(other_groups.group0_.z, other_groups.group0_.x, other_groups.group0_.y, other_groups.group1_.x) * self_.group1_.yzxx)
    ));
}
fn multiVector_wedge_multiVector(self_: MultiVector, other: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other_groups.group0_.x * self_.group0_.x, (other_groups.group0_.x * self_.group0_.y) + (other_groups.group0_.y * self_.group0_.x) + (other_groups.group4_.x * self_.group1_.x) + (other_groups.group4_.y * self_.group1_.y) + (other_groups.group4_.z * self_.group1_.z) + (other_groups.group4_.w * self_.group1_.w) - (other_groups.group2_.x * self_.group3_.x) - (other_groups.group2_.y * self_.group3_.y) - (other_groups.group2_.z * self_.group3_.z) - (other_groups.group3_.x * self_.group2_.x) - (other_groups.group3_.y * self_.group2_.y) - (other_groups.group3_.z * self_.group2_.z) - (other_groups.group1_.x * self_.group4_.x) - (other_groups.group1_.y * self_.group4_.y) - (other_groups.group1_.z * self_.group4_.z) - (other_groups.group1_.w * self_.group4_.w), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ (vec4<f32>(other_groups.group0_.x) * self_.group1_) + (vec4<f32>(self_.group0_.x) * other_groups.group1_), 
        /* e41, e42, e43 */ (vec4<f32>(vec3<f32>(other_groups.group0_.x), 0.0) * self_.group2_) + (vec4<f32>(vec3<f32>(self_.group0_.x), 0.0) * other_groups.group2_) + (vec4<f32>(vec3<f32>(self_.group1_.w), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0)) - (vec4<f32>(vec3<f32>(other_groups.group1_.w), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((other_groups.group1_.z * self_.group1_.y) - (other_groups.group1_.y * self_.group1_.z), (other_groups.group1_.x * self_.group1_.z) - (other_groups.group1_.z * self_.group1_.x), (other_groups.group1_.y * self_.group1_.x) - (other_groups.group1_.x * self_.group1_.y), 0.0) + (vec4<f32>(vec3<f32>(other_groups.group0_.x), 0.0) * self_.group3_) + (vec4<f32>(vec3<f32>(self_.group0_.x), 0.0) * other_groups.group3_), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group2_.y * self_.group1_.z) + (other_groups.group3_.x * self_.group1_.w) + (self_.group2_.y * other_groups.group1_.z) + (self_.group3_.x * other_groups.group1_.w), (other_groups.group2_.z * self_.group1_.x) + (other_groups.group3_.y * self_.group1_.w) + (self_.group2_.z * other_groups.group1_.x) + (self_.group3_.y * other_groups.group1_.w), (other_groups.group2_.x * self_.group1_.y) + (other_groups.group3_.z * self_.group1_.w) + (self_.group2_.x * other_groups.group1_.y) + (self_.group3_.z * other_groups.group1_.w), -(other_groups.group3_.y * self_.group1_.y) - (other_groups.group3_.z * self_.group1_.z) - (self_.group3_.y * other_groups.group1_.y) - (self_.group3_.z * other_groups.group1_.z)) + (vec4<f32>(other_groups.group0_.x) * self_.group4_) + (vec4<f32>(self_.group0_.x) * other_groups.group4_) - (vec4<f32>(other_groups.group2_.zxy.xyz, other_groups.group3_.x) * self_.group1_.yzxx) - (vec4<f32>(self_.group2_.zxy.xyz, self_.group3_.x) * other_groups.group1_.yzxx)
    ));
}
fn multiVector_wedge_origin(self_: MultiVector, other: Origin) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = origin_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, self_.group4_.w * other.e4 * -1.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.group0_.x * other.e4), 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(other.e4), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group3_.x * other.e4, self_.group3_.y * other.e4, self_.group3_.z * other.e4, 0.0)
    ));
}
fn multiVector_wedge_plane(self_: MultiVector, other: Plane) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = plane_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, (self_.group1_.x * other_groups.group0_.x) + (self_.group1_.y * other_groups.group0_.y) + (self_.group1_.z * other_groups.group0_.z) + (self_.group1_.w * other_groups.group0_.w), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.x) * other_groups.group0_
    ));
}
fn multiVector_wedge_point(self_: MultiVector, other: Point) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = point_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, -(self_.group4_.x * other_groups.group0_.x) - (self_.group4_.y * other_groups.group0_.y) - (self_.group4_.z * other_groups.group0_.z) - (self_.group4_.w * other_groups.group0_.w), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group0_.x) * other_groups.group0_, 
        /* e41, e42, e43 */ (vec4<f32>(vec3<f32>(self_.group1_.w), 0.0) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0)) - (vec4<f32>(vec3<f32>(other_groups.group0_.w), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((self_.group1_.y * other_groups.group0_.z) - (self_.group1_.z * other_groups.group0_.y), (self_.group1_.z * other_groups.group0_.x) - (self_.group1_.x * other_groups.group0_.z), (self_.group1_.x * other_groups.group0_.y) - (self_.group1_.y * other_groups.group0_.x), 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.group2_.y * other_groups.group0_.z) + (self_.group3_.x * other_groups.group0_.w), (self_.group2_.z * other_groups.group0_.x) + (self_.group3_.y * other_groups.group0_.w), (self_.group2_.x * other_groups.group0_.y) + (self_.group3_.z * other_groups.group0_.w), -(self_.group3_.y * other_groups.group0_.y) - (self_.group3_.z * other_groups.group0_.z)) - (vec4<f32>(self_.group2_.zxy.xyz, self_.group3_.x) * other_groups.group0_.yzxx)
    ));
}
fn multiVector_wedge_scalar(self_: MultiVector, other: Scalar) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = scalar_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(other.scalar), 0.0, 0.0) * self_.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(other.scalar) * self_.group1_, 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(other.scalar), 0.0) * self_.group2_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(other.scalar), 0.0) * self_.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other.scalar) * self_.group4_
    ));
}
fn origin_wedge_dualNum(self_: Origin, other: DualNum) -> Origin {
    let self_groups = origin_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(other_groups.group0_.x * self.e4, 0.0, 0.0, 0.0)
    ));
}
fn origin_wedge_flector(self_: Origin, other: Flector) -> Motor {
    let self_groups = origin_grouped(self_);
    let other_groups = flector_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self.e4) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, other_groups.group1_.w), 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0)
    ));
}
fn origin_wedge_horizon(self_: Origin, other: Horizon) -> AntiScalar {
    let self_groups = origin_grouped(self_);
    let other_groups = horizon_grouped(other);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(other.e321 * self.e4, 0.0, 0.0, 0.0)
    ));
}
fn origin_wedge_line(self_: Origin, other: Line) -> Plane {
    let self_groups = origin_grouped(self_);
    let other_groups = line_grouped(other);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group1_.x * self.e4, other_groups.group1_.y * self.e4, other_groups.group1_.z * self.e4, 0.0)
    ));
}
fn origin_wedge_motor(self_: Origin, other: Motor) -> Flector {
    let self_groups = origin_grouped(self_);
    let other_groups = motor_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group1_.w * self.e4), 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group1_.x * self.e4, other_groups.group1_.y * self.e4, other_groups.group1_.z * self.e4, 0.0)
    ));
}
fn origin_wedge_multiVector(self_: Origin, other: MultiVector) -> MultiVector {
    let self_groups = origin_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, other_groups.group4_.w * self.e4, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group0_.x * self.e4), 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(self.e4), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group3_.x * self.e4, other_groups.group3_.y * self.e4, other_groups.group3_.z * self.e4, 0.0)
    ));
}
fn origin_wedge_plane(self_: Origin, other: Plane) -> AntiScalar {
    let self_groups = origin_grouped(self_);
    let other_groups = plane_grouped(other);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(other_groups.group0_.w * self.e4, 0.0, 0.0, 0.0)
    ));
}
fn origin_wedge_point(self_: Origin, other: Point) -> Line {
    let self_groups = origin_grouped(self_);
    let other_groups = point_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(self.e4), 0.0) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0)
    ));
}
fn origin_wedge_scalar(self_: Origin, other: Scalar) -> Origin {
    let self_groups = origin_grouped(self_);
    let other_groups = scalar_grouped(other);
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(self.e4 * other.scalar, 0.0, 0.0, 0.0)
    ));
}
fn plane_wedge_dualNum(self_: Plane, other: DualNum) -> Plane {
    let self_groups = plane_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group0_.x) * self_.group0_
    ));
}
fn plane_wedge_flector(self_: Plane, other: Flector) -> AntiScalar {
    let self_groups = plane_grouped(self_);
    let other_groups = flector_grouped(other);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(-(other_groups.group0_.x * self_.group0_.x) - (other_groups.group0_.y * self_.group0_.y) - (other_groups.group0_.z * self_.group0_.z) - (other_groups.group0_.w * self_.group0_.w), 0.0, 0.0, 0.0)
    ));
}
fn plane_wedge_motor(self_: Plane, other: Motor) -> Plane {
    let self_groups = plane_grouped(self_);
    let other_groups = motor_grouped(other);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group1_.w) * self_.group0_
    ));
}
fn plane_wedge_multiVector(self_: Plane, other: MultiVector) -> MultiVector {
    let self_groups = plane_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, -(other_groups.group1_.x * self_.group0_.x) - (other_groups.group1_.y * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.z) - (other_groups.group1_.w * self_.group0_.w), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group0_.x) * self_.group0_
    ));
}
fn plane_wedge_origin(self_: Plane, other: Origin) -> AntiScalar {
    let self_groups = plane_grouped(self_);
    let other_groups = origin_grouped(other);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(self_.group0_.w * other.e4 * -1.0, 0.0, 0.0, 0.0)
    ));
}
fn plane_wedge_point(self_: Plane, other: Point) -> AntiScalar {
    let self_groups = plane_grouped(self_);
    let other_groups = point_grouped(other);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(-(self_.group0_.x * other_groups.group0_.x) - (self_.group0_.y * other_groups.group0_.y) - (self_.group0_.z * other_groups.group0_.z) - (self_.group0_.w * other_groups.group0_.w), 0.0, 0.0, 0.0)
    ));
}
fn plane_wedge_scalar(self_: Plane, other: Scalar) -> Plane {
    let self_groups = plane_grouped(self_);
    let other_groups = scalar_grouped(other);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(other.scalar) * self_.group0_
    ));
}
fn point_wedge_dualNum(self_: Point, other: DualNum) -> Point {
    let self_groups = point_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group0_.x) * self_.group0_
    ));
}
fn point_wedge_flector(self_: Point, other: Flector) -> Motor {
    let self_groups = point_grouped(self_);
    let other_groups = flector_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other_groups.group0_.w * self_.group0_.x) * -1.0, (other_groups.group0_.w * self_.group0_.y) * -1.0, (other_groups.group0_.w * self_.group0_.z) * -1.0, (other_groups.group1_.y * self_.group0_.y) + (other_groups.group1_.z * self_.group0_.z) + (other_groups.group1_.w * self_.group0_.w)) + (vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, other_groups.group1_.x) * self_.group0_.wwwx), 
        /* e23, e31, e12, scalar */ vec4<f32>((other_groups.group0_.z * self_.group0_.y) - (other_groups.group0_.y * self_.group0_.z), (other_groups.group0_.x * self_.group0_.z) - (other_groups.group0_.z * self_.group0_.x), (other_groups.group0_.y * self_.group0_.x) - (other_groups.group0_.x * self_.group0_.y), 0.0)
    ));
}
fn point_wedge_horizon(self_: Point, other: Horizon) -> AntiScalar {
    let self_groups = point_grouped(self_);
    let other_groups = horizon_grouped(other);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(self_.group0_.w * other.e321, 0.0, 0.0, 0.0)
    ));
}
fn point_wedge_line(self_: Point, other: Line) -> Plane {
    let self_groups = point_grouped(self_);
    let other_groups = line_grouped(other);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.y * self_.group0_.z) + (other_groups.group1_.x * self_.group0_.w), (other_groups.group0_.z * self_.group0_.x) + (other_groups.group1_.y * self_.group0_.w), (other_groups.group0_.x * self_.group0_.y) + (other_groups.group1_.z * self_.group0_.w), -(other_groups.group1_.y * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.z)) - (vec4<f32>(other_groups.group0_.zxy.xyz, other_groups.group1_.x) * self_.group0_.yzxx)
    ));
}
fn point_wedge_motor(self_: Point, other: Motor) -> Flector {
    let self_groups = point_grouped(self_);
    let other_groups = motor_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group1_.w) * self_.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.y * self_.group0_.z) + (other_groups.group1_.x * self_.group0_.w), (other_groups.group0_.z * self_.group0_.x) + (other_groups.group1_.y * self_.group0_.w), (other_groups.group0_.x * self_.group0_.y) + (other_groups.group1_.z * self_.group0_.w), -(other_groups.group1_.y * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.z)) - (vec4<f32>(other_groups.group0_.z, other_groups.group0_.x, other_groups.group0_.y, other_groups.group1_.x) * self_.group0_.yzxx)
    ));
}
fn point_wedge_multiVector(self_: Point, other: MultiVector) -> MultiVector {
    let self_groups = point_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(0.0, (other_groups.group4_.x * self_.group0_.x) + (other_groups.group4_.y * self_.group0_.y) + (other_groups.group4_.z * self_.group0_.z) + (other_groups.group4_.w * self_.group0_.w), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group0_.x) * self_.group0_, 
        /* e41, e42, e43 */ (vec4<f32>(vec3<f32>(self_.group0_.w), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0)) - (vec4<f32>(vec3<f32>(other_groups.group1_.w), 0.0) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((other_groups.group1_.z * self_.group0_.y) - (other_groups.group1_.y * self_.group0_.z), (other_groups.group1_.x * self_.group0_.z) - (other_groups.group1_.z * self_.group0_.x), (other_groups.group1_.y * self_.group0_.x) - (other_groups.group1_.x * self_.group0_.y), 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group2_.y * self_.group0_.z) + (other_groups.group3_.x * self_.group0_.w), (other_groups.group2_.z * self_.group0_.x) + (other_groups.group3_.y * self_.group0_.w), (other_groups.group2_.x * self_.group0_.y) + (other_groups.group3_.z * self_.group0_.w), -(other_groups.group3_.y * self_.group0_.y) - (other_groups.group3_.z * self_.group0_.z)) - (vec4<f32>(other_groups.group2_.zxy.xyz, other_groups.group3_.x) * self_.group0_.yzxx)
    ));
}
fn point_wedge_origin(self_: Point, other: Origin) -> Line {
    let self_groups = point_grouped(self_);
    let other_groups = origin_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(other.e4), 0.0) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>(0.0)
    ));
}
fn point_wedge_plane(self_: Point, other: Plane) -> AntiScalar {
    let self_groups = point_grouped(self_);
    let other_groups = plane_grouped(other);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>((other_groups.group0_.x * self_.group0_.x) + (other_groups.group0_.y * self_.group0_.y) + (other_groups.group0_.z * self_.group0_.z) + (other_groups.group0_.w * self_.group0_.w), 0.0, 0.0, 0.0)
    ));
}
fn point_wedge_point(self_: Point, other: Point) -> Line {
    let self_groups = point_grouped(self_);
    let other_groups = point_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ (vec4<f32>(vec3<f32>(self_.group0_.w), 0.0) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0)) - (vec4<f32>(vec3<f32>(other_groups.group0_.w), 0.0) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((other_groups.group0_.z * self_.group0_.y) - (other_groups.group0_.y * self_.group0_.z), (other_groups.group0_.x * self_.group0_.z) - (other_groups.group0_.z * self_.group0_.x), (other_groups.group0_.y * self_.group0_.x) - (other_groups.group0_.x * self_.group0_.y), 0.0)
    ));
}
fn point_wedge_scalar(self_: Point, other: Scalar) -> Point {
    let self_groups = point_grouped(self_);
    let other_groups = scalar_grouped(other);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.scalar) * self_.group0_
    ));
}
fn scalar_wedge_antiScalar(self_: Scalar, other: AntiScalar) -> AntiScalar {
    let self_groups = scalar_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(other.e1234 * self.scalar, 0.0, 0.0, 0.0)
    ));
}
fn scalar_wedge_dualNum(self_: Scalar, other: DualNum) -> DualNum {
    let self_groups = scalar_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return dualNum_degroup(DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(self.scalar), 0.0, 0.0) * other_groups.group0_
    ));
}
fn scalar_wedge_flector(self_: Scalar, other: Flector) -> Flector {
    let self_groups = scalar_grouped(self_);
    let other_groups = flector_grouped(other);
    return flector_degroup(FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self.scalar) * other_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self.scalar) * other_groups.group1_
    ));
}
fn scalar_wedge_horizon(self_: Scalar, other: Horizon) -> Horizon {
    let self_groups = scalar_grouped(self_);
    let other_groups = horizon_grouped(other);
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(other.e321 * self.scalar, 0.0, 0.0, 0.0)
    ));
}
fn scalar_wedge_line(self_: Scalar, other: Line) -> Line {
    let self_groups = scalar_grouped(self_);
    let other_groups = line_grouped(other);
    return line_degroup(LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(self.scalar), 0.0) * other_groups.group0_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(self.scalar), 0.0) * other_groups.group1_
    ));
}
fn scalar_wedge_motor(self_: Scalar, other: Motor) -> Motor {
    let self_groups = scalar_grouped(self_);
    let other_groups = motor_grouped(other);
    return motor_degroup(MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self.scalar) * other_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(self.scalar) * other_groups.group1_
    ));
}
fn scalar_wedge_multiVector(self_: Scalar, other: MultiVector) -> MultiVector {
    let self_groups = scalar_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_degroup(MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(self.scalar), 0.0, 0.0) * other_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(self.scalar) * other_groups.group1_, 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(self.scalar), 0.0) * other_groups.group2_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(self.scalar), 0.0) * other_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self.scalar) * other_groups.group4_
    ));
}
fn scalar_wedge_origin(self_: Scalar, other: Origin) -> Origin {
    let self_groups = scalar_grouped(self_);
    let other_groups = origin_grouped(other);
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(other.e4 * self.scalar, 0.0, 0.0, 0.0)
    ));
}
fn scalar_wedge_plane(self_: Scalar, other: Plane) -> Plane {
    let self_groups = scalar_grouped(self_);
    let other_groups = plane_grouped(other);
    return plane_degroup(PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(self.scalar) * other_groups.group0_
    ));
}
fn scalar_wedge_point(self_: Scalar, other: Point) -> Point {
    let self_groups = scalar_grouped(self_);
    let other_groups = point_grouped(other);
    return point_degroup(PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self.scalar) * other_groups.group0_
    ));
}
fn scalar_wedge_scalar(self_: Scalar, other: Scalar) -> Scalar {
    let self_groups = scalar_grouped(self_);
    let other_groups = scalar_grouped(other);
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(other.scalar * self.scalar, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_zero() -> AntiScalar {
    return antiScalar_degroup(AntiScalarGroups(
        /* e1234 */ vec4<f32>(0.0, 0.0, 0.0, 0.0)
    ));
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
    return horizon_degroup(HorizonGroups(
        /* e321 */ vec4<f32>(0.0, 0.0, 0.0, 0.0)
    ));
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
    return origin_degroup(OriginGroups(
        /* e4 */ vec4<f32>(0.0, 0.0, 0.0, 0.0)
    ));
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
    return scalar_degroup(ScalarGroups(
        /* scalar */ vec4<f32>(0.0, 0.0, 0.0, 0.0)
    ));
}
fn antiScalar_antiSandwich_antiScalar(self_: AntiScalar, other: AntiScalar) -> AntiScalar {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    let geometric_anti_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(other.e1234 * self.e1234, 0.0, 0.0, 0.0)
    );
    let geometric_anti_product: AntiScalar = antiScalar_degroup(geometric_anti_product_groups);
    return antiScalar_geometricAntiProduct_antiScalar(geometric_anti_product, antiScalar_antiReverse(self));
}
fn antiScalar_antiSandwich_dualNum(self_: AntiScalar, other: DualNum) -> DualNum {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let geometric_anti_product_groups: DualNumGroups = DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(self.e1234), 0.0, 0.0) * other_groups.group0_
    );
    let geometric_anti_product: DualNum = dualNum_degroup(geometric_anti_product_groups);
    return dualNum_geometricAntiProduct_antiScalar(geometric_anti_product, antiScalar_antiReverse(self));
}
fn antiScalar_antiSandwich_flector(self_: AntiScalar, other: Flector) -> Flector {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = flector_grouped(other);
    let geometric_anti_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self.e1234) * other_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self.e1234) * other_groups.group1_
    );
    let geometric_anti_product: Flector = flector_degroup(geometric_anti_product_groups);
    return flector_geometricAntiProduct_antiScalar(geometric_anti_product, antiScalar_antiReverse(self));
}
fn antiScalar_antiSandwich_horizon(self_: AntiScalar, other: Horizon) -> Horizon {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = horizon_grouped(other);
    let geometric_anti_product_groups: HorizonGroups = HorizonGroups(
        /* e321 */ vec4<f32>(self.e1234 * other.e321, 0.0, 0.0, 0.0)
    );
    let geometric_anti_product: Horizon = horizon_degroup(geometric_anti_product_groups);
    return horizon_geometricAntiProduct_antiScalar(geometric_anti_product, antiScalar_antiReverse(self));
}
fn antiScalar_antiSandwich_line(self_: AntiScalar, other: Line) -> Line {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = line_grouped(other);
    let geometric_anti_product_groups: LineGroups = LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(self.e1234), 0.0) * other_groups.group0_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(self.e1234), 0.0) * other_groups.group1_
    );
    let geometric_anti_product: Line = line_degroup(geometric_anti_product_groups);
    return line_geometricAntiProduct_antiScalar(geometric_anti_product, antiScalar_antiReverse(self));
}
fn antiScalar_antiSandwich_motor(self_: AntiScalar, other: Motor) -> Motor {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = motor_grouped(other);
    let geometric_anti_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self.e1234) * other_groups.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(self.e1234) * other_groups.group1_
    );
    let geometric_anti_product: Motor = motor_degroup(geometric_anti_product_groups);
    return motor_geometricAntiProduct_antiScalar(geometric_anti_product, antiScalar_antiReverse(self));
}
fn antiScalar_antiSandwich_multiVector(self_: AntiScalar, other: MultiVector) -> MultiVector {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let geometric_anti_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(self.e1234), 0.0, 0.0) * other_groups.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(self.e1234) * other_groups.group1_, 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(self.e1234), 0.0) * other_groups.group2_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(self.e1234), 0.0) * other_groups.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(self.e1234) * other_groups.group4_
    );
    let geometric_anti_product: MultiVector = multiVector_degroup(geometric_anti_product_groups);
    return multiVector_geometricAntiProduct_antiScalar(geometric_anti_product, antiScalar_antiReverse(self));
}
fn antiScalar_antiSandwich_origin(self_: AntiScalar, other: Origin) -> Origin {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = origin_grouped(other);
    let geometric_anti_product_groups: OriginGroups = OriginGroups(
        /* e4 */ vec4<f32>(self.e1234 * other.e4, 0.0, 0.0, 0.0)
    );
    let geometric_anti_product: Origin = origin_degroup(geometric_anti_product_groups);
    return origin_geometricAntiProduct_antiScalar(geometric_anti_product, antiScalar_antiReverse(self));
}
fn antiScalar_antiSandwich_plane(self_: AntiScalar, other: Plane) -> Plane {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = plane_grouped(other);
    let geometric_anti_product_groups: PlaneGroups = PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(self.e1234) * other_groups.group0_
    );
    let geometric_anti_product: Plane = plane_degroup(geometric_anti_product_groups);
    return plane_geometricAntiProduct_antiScalar(geometric_anti_product, antiScalar_antiReverse(self));
}
fn antiScalar_antiSandwich_point(self_: AntiScalar, other: Point) -> Point {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = point_grouped(other);
    let geometric_anti_product_groups: PointGroups = PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self.e1234) * other_groups.group0_
    );
    let geometric_anti_product: Point = point_degroup(geometric_anti_product_groups);
    return point_geometricAntiProduct_antiScalar(geometric_anti_product, antiScalar_antiReverse(self));
}
fn antiScalar_antiSandwich_scalar(self_: AntiScalar, other: Scalar) -> Scalar {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = scalar_grouped(other);
    let geometric_anti_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(self.e1234 * other.scalar, 0.0, 0.0, 0.0)
    );
    let geometric_anti_product: Scalar = scalar_degroup(geometric_anti_product_groups);
    return scalar_geometricAntiProduct_antiScalar(geometric_anti_product, antiScalar_antiReverse(self));
}
fn dualNum_antiSandwich_antiScalar(self_: DualNum, other: AntiScalar) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    let geometric_anti_product_groups: DualNumGroups = DualNumGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(other.e1234), 0.0, 0.0) * self_.group0_
    );
    let geometric_anti_product: DualNum = dualNum_degroup(geometric_anti_product_groups);
    return dualNum_geometricAntiProduct_dualNum(geometric_anti_product, dualNum_antiReverse(self));
}
fn dualNum_antiSandwich_dualNum(self_: DualNum, other: DualNum) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let geometric_anti_product_groups: DualNumGroups = DualNumGroups(
        /* scalar, e1234 */ vec4<f32>((other_groups.group0_.x * self_.group0_.y) + (other_groups.group0_.y * self_.group0_.x), other_groups.group0_.y * self_.group0_.y, 0.0, 0.0)
    );
    let geometric_anti_product: DualNum = dualNum_degroup(geometric_anti_product_groups);
    return dualNum_geometricAntiProduct_dualNum(geometric_anti_product, dualNum_antiReverse(self));
}
fn dualNum_antiSandwich_flector(self_: DualNum, other: Flector) -> Flector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = flector_grouped(other);
    let geometric_anti_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group0_.x * other_groups.group1_.x) + (self_.group0_.y * other_groups.group0_.x), (self_.group0_.x * other_groups.group1_.y) + (self_.group0_.y * other_groups.group0_.y), (self_.group0_.x * other_groups.group1_.z) + (self_.group0_.y * other_groups.group0_.z), self_.group0_.y * other_groups.group0_.w), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.y * other_groups.group1_.x, self_.group0_.y * other_groups.group1_.y, self_.group0_.y * other_groups.group1_.z, (self_.group0_.x * other_groups.group0_.w) + (self_.group0_.y * other_groups.group1_.w))
    );
    let geometric_anti_product: Flector = flector_degroup(geometric_anti_product_groups);
    return flector_geometricAntiProduct_dualNum(geometric_anti_product, dualNum_antiReverse(self));
}
fn dualNum_antiSandwich_horizon(self_: DualNum, other: Horizon) -> Horizon {
    let self_groups = dualNum_grouped(self_);
    let other_groups = horizon_grouped(other);
    let geometric_anti_product_groups: HorizonGroups = HorizonGroups(
        /* e321 */ vec4<f32>(self_.group0_.y * other.e321, 0.0, 0.0, 0.0)
    );
    let geometric_anti_product: Horizon = horizon_degroup(geometric_anti_product_groups);
    return horizon_geometricAntiProduct_dualNum(geometric_anti_product, dualNum_antiReverse(self));
}
fn dualNum_antiSandwich_line(self_: DualNum, other: Line) -> Line {
    let self_groups = dualNum_grouped(self_);
    let other_groups = line_grouped(other);
    let geometric_anti_product_groups: LineGroups = LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(self_.group0_.y), 0.0) * other_groups.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(vec3<f32>(self_.group0_.x), 0.0) * other_groups.group0_) + (vec4<f32>(vec3<f32>(self_.group0_.y), 0.0) * other_groups.group1_)
    );
    let geometric_anti_product: Line = line_degroup(geometric_anti_product_groups);
    return line_geometricAntiProduct_dualNum(geometric_anti_product, dualNum_antiReverse(self));
}
fn dualNum_antiSandwich_motor(self_: DualNum, other: Motor) -> Motor {
    let self_groups = dualNum_grouped(self_);
    let other_groups = motor_grouped(other);
    let geometric_anti_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.group0_.y) * other_groups.group0_, 
        /* e23, e31, e12, scalar */ (vec4<f32>(self_.group0_.x) * other_groups.group0_) + (vec4<f32>(self_.group0_.y) * other_groups.group1_)
    );
    let geometric_anti_product: Motor = motor_degroup(geometric_anti_product_groups);
    return motor_geometricAntiProduct_dualNum(geometric_anti_product, dualNum_antiReverse(self));
}
fn dualNum_antiSandwich_multiVector(self_: DualNum, other: MultiVector) -> MultiVector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let geometric_anti_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((self_.group0_.x * other_groups.group0_.y) + (self_.group0_.y * other_groups.group0_.x), self_.group0_.y * other_groups.group0_.y, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group0_.x * other_groups.group4_.x) + (self_.group0_.y * other_groups.group1_.x), (self_.group0_.x * other_groups.group4_.y) + (self_.group0_.y * other_groups.group1_.y), (self_.group0_.x * other_groups.group4_.z) + (self_.group0_.y * other_groups.group1_.z), self_.group0_.y * other_groups.group1_.w), 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(self_.group0_.y), 0.0) * other_groups.group2_, 
        /* e23, e31, e12 */ (vec4<f32>(vec3<f32>(self_.group0_.x), 0.0) * other_groups.group2_) + (vec4<f32>(vec3<f32>(self_.group0_.y), 0.0) * other_groups.group3_), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.y * other_groups.group4_.x, self_.group0_.y * other_groups.group4_.y, self_.group0_.y * other_groups.group4_.z, (self_.group0_.x * other_groups.group1_.w) + (self_.group0_.y * other_groups.group4_.w))
    );
    let geometric_anti_product: MultiVector = multiVector_degroup(geometric_anti_product_groups);
    return multiVector_geometricAntiProduct_dualNum(geometric_anti_product, dualNum_antiReverse(self));
}
fn dualNum_antiSandwich_origin(self_: DualNum, other: Origin) -> Flector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = origin_grouped(other);
    let geometric_anti_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, self_.group0_.y * other.e4), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.group0_.x * other.e4)
    );
    let geometric_anti_product: Flector = flector_degroup(geometric_anti_product_groups);
    return flector_geometricAntiProduct_dualNum(geometric_anti_product, dualNum_antiReverse(self));
}
fn dualNum_antiSandwich_plane(self_: DualNum, other: Plane) -> Flector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = plane_grouped(other);
    let geometric_anti_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group0_.x * other_groups.group0_.x, self_.group0_.x * other_groups.group0_.y, self_.group0_.x * other_groups.group0_.z, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.y) * other_groups.group0_
    );
    let geometric_anti_product: Flector = flector_degroup(geometric_anti_product_groups);
    return flector_geometricAntiProduct_dualNum(geometric_anti_product, dualNum_antiReverse(self));
}
fn dualNum_antiSandwich_point(self_: DualNum, other: Point) -> Flector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = point_grouped(other);
    let geometric_anti_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group0_.y) * other_groups.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.group0_.x * other_groups.group0_.w)
    );
    let geometric_anti_product: Flector = flector_degroup(geometric_anti_product_groups);
    return flector_geometricAntiProduct_dualNum(geometric_anti_product, dualNum_antiReverse(self));
}
fn dualNum_antiSandwich_scalar(self_: DualNum, other: Scalar) -> Scalar {
    let self_groups = dualNum_grouped(self_);
    let other_groups = scalar_grouped(other);
    let geometric_anti_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(self_.group0_.y * other.scalar, 0.0, 0.0, 0.0)
    );
    let geometric_anti_product: Scalar = scalar_degroup(geometric_anti_product_groups);
    return scalar_geometricAntiProduct_dualNum(geometric_anti_product, dualNum_antiReverse(self));
}
fn flector_antiSandwich_antiScalar(self_: Flector, other: AntiScalar) -> Motor {
    let self_groups = flector_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    let geometric_anti_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e1234) * self_.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e1234) * self_.group1_
    );
    let geometric_anti_product: Flector = flector_degroup(geometric_anti_product_groups);
    return flector_geometricAntiProduct_flector(geometric_anti_product, flector_antiReverse(self));
}
fn flector_antiSandwich_dualNum(self_: Flector, other: DualNum) -> Motor {
    let self_groups = flector_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let geometric_anti_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group0_.y * self_.group0_.x) - (other_groups.group0_.x * self_.group1_.x), (other_groups.group0_.y * self_.group0_.y) - (other_groups.group0_.x * self_.group1_.y), (other_groups.group0_.y * self_.group0_.z) - (other_groups.group0_.x * self_.group1_.z), other_groups.group0_.y * self_.group0_.w), 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group0_.y * self_.group1_.x, other_groups.group0_.y * self_.group1_.y, other_groups.group0_.y * self_.group1_.z, (other_groups.group0_.y * self_.group1_.w) - (other_groups.group0_.x * self_.group0_.w))
    );
    let geometric_anti_product: Flector = flector_degroup(geometric_anti_product_groups);
    return flector_geometricAntiProduct_flector(geometric_anti_product, flector_antiReverse(self));
}
fn flector_antiSandwich_flector(self_: Flector, other: Flector) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = flector_grouped(other);
    let geometric_anti_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(-(other_groups.group1_.x * self_.group0_.w) - (other_groups.group1_.z * self_.group1_.y), -(other_groups.group1_.x * self_.group1_.z) - (other_groups.group1_.y * self_.group0_.w), -(other_groups.group1_.y * self_.group1_.x) - (other_groups.group1_.z * self_.group0_.w), (other_groups.group1_.y * self_.group1_.y) + (other_groups.group1_.z * self_.group1_.z)) + (other_groups.group1_.yzxx * self_.group1_.zxyx) - (vec4<f32>(other_groups.group0_.w) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group0_.w)), 
        /* e23, e31, e12, scalar */ vec4<f32>((other_groups.group0_.z * self_.group1_.y) - (other_groups.group1_.z * self_.group0_.y), (other_groups.group0_.x * self_.group1_.z) - (other_groups.group1_.x * self_.group0_.z), (other_groups.group0_.y * self_.group1_.x) - (other_groups.group1_.y * self_.group0_.x), (other_groups.group1_.w * self_.group0_.w) - (other_groups.group0_.w * self_.group1_.w)) + (vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, other_groups.group1_.x) * self_.group0_.wwwx) + (vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group0_.z) * other_groups.group1_.wwwz) + (other_groups.group1_.yzxy * self_.group0_.zxyy) - (vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, other_groups.group0_.z) * self_.group1_.wwwz) - (vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group1_.y) * other_groups.group0_.wwwy) - (other_groups.group0_.yzxx * self_.group1_.zxyx)
    );
    let geometric_anti_product: Motor = motor_degroup(geometric_anti_product_groups);
    return motor_geometricAntiProduct_flector(geometric_anti_product, flector_antiReverse(self));
}
fn flector_antiSandwich_horizon(self_: Flector, other: Horizon) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = horizon_grouped(other);
    let geometric_anti_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(other.e321) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group0_.w)
    );
    let geometric_anti_product: Motor = motor_degroup(geometric_anti_product_groups);
    return motor_geometricAntiProduct_flector(geometric_anti_product, flector_antiReverse(self));
}
fn flector_antiSandwich_line(self_: Flector, other: Line) -> Motor {
    let self_groups = flector_grouped(self_);
    let other_groups = line_grouped(other);
    let geometric_anti_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group0_.x * self_.group1_.w) + (other_groups.group0_.z * self_.group0_.y) + (other_groups.group1_.y * self_.group1_.z) - (other_groups.group0_.y * self_.group0_.z) - (other_groups.group1_.x * self_.group0_.w), (other_groups.group0_.x * self_.group0_.z) + (other_groups.group0_.y * self_.group1_.w) + (other_groups.group1_.z * self_.group1_.x) - (other_groups.group0_.z * self_.group0_.x) - (other_groups.group1_.y * self_.group0_.w), (other_groups.group0_.y * self_.group0_.x) + (other_groups.group0_.z * self_.group1_.w) + (other_groups.group1_.x * self_.group1_.y) - (other_groups.group0_.x * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.w), -(other_groups.group0_.y * self_.group1_.y) - (other_groups.group0_.z * self_.group1_.z)) - (vec4<f32>(other_groups.group1_.zxy.xyz, other_groups.group0_.x) * self_.group1_.yzxx), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.x * self_.group0_.w) - (other_groups.group0_.y * self_.group1_.z), (other_groups.group0_.y * self_.group0_.w) - (other_groups.group0_.z * self_.group1_.x), (other_groups.group0_.z * self_.group0_.w) - (other_groups.group0_.x * self_.group1_.y), (other_groups.group1_.y * self_.group1_.y) + (other_groups.group1_.z * self_.group1_.z) - (other_groups.group0_.x * self_.group0_.x) - (other_groups.group0_.y * self_.group0_.y) - (other_groups.group0_.z * self_.group0_.z)) + (vec4<f32>(other_groups.group0_.zxy.xyz, other_groups.group1_.x) * self_.group1_.yzxx)
    );
    let geometric_anti_product: Flector = flector_degroup(geometric_anti_product_groups);
    return flector_geometricAntiProduct_flector(geometric_anti_product, flector_antiReverse(self));
}
fn flector_antiSandwich_motor(self_: Flector, other: Motor) -> Motor {
    let self_groups = flector_grouped(self_);
    let other_groups = motor_grouped(other);
    let geometric_anti_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group0_.y * other_groups.group0_.z) + (self_.group1_.z * other_groups.group1_.y) + (self_.group1_.w * other_groups.group0_.x) - (self_.group0_.w * other_groups.group1_.x), (self_.group0_.z * other_groups.group0_.x) + (self_.group1_.x * other_groups.group1_.z) + (self_.group1_.w * other_groups.group0_.y) - (self_.group0_.w * other_groups.group1_.y), (self_.group0_.z * other_groups.group0_.w) + (self_.group1_.y * other_groups.group1_.x) + (self_.group1_.w * other_groups.group0_.z) - (self_.group0_.w * other_groups.group1_.z), 0.0) + (self_.group0_.xyxw * other_groups.group0_.wwyw) - (vec4<f32>(self_.group0_.z, self_.group0_.x, self_.group0_.y, self_.group1_.x) * other_groups.group0_.yzxx) - (vec4<f32>(other_groups.group1_.z, other_groups.group1_.x, other_groups.group1_.w, other_groups.group0_.z) * self_.group1_.yzzz) - (vec4<f32>(other_groups.group1_.w, other_groups.group1_.w, other_groups.group1_.y, other_groups.group0_.y) * self_.group1_.xyxy), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, (self_.group1_.z * other_groups.group1_.z) - (self_.group0_.y * other_groups.group0_.y) - (self_.group0_.z * other_groups.group0_.z) - (self_.group0_.w * other_groups.group1_.w)) + (vec4<f32>(self_.group0_.w, self_.group0_.w, self_.group0_.w, self_.group1_.w) * other_groups.group0_) + (vec4<f32>(other_groups.group0_.z, other_groups.group0_.x, other_groups.group0_.w, other_groups.group1_.y) * self_.group1_.yzzy) + (vec4<f32>(other_groups.group0_.w, other_groups.group0_.w, other_groups.group0_.y, other_groups.group1_.x) * self_.group1_.xyxx) - (vec4<f32>(self_.group1_.z, self_.group1_.x, self_.group1_.y, self_.group0_.x) * other_groups.group0_.yzxx)
    );
    let geometric_anti_product: Flector = flector_degroup(geometric_anti_product_groups);
    return flector_geometricAntiProduct_flector(geometric_anti_product, flector_antiReverse(self));
}
fn flector_antiSandwich_multiVector(self_: Flector, other: MultiVector) -> MultiVector {
    let self_groups = flector_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let geometric_anti_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((self_.group0_.w * other_groups.group4_.w) - (self_.group1_.x * other_groups.group1_.x) - (self_.group1_.y * other_groups.group1_.y) - (self_.group1_.z * other_groups.group1_.z), 0.0, 0.0, 0.0) + (vec4<f32>(vec2<f32>(other_groups.group4_.x), 0.0, 0.0) * vec4<f32>(self_.group0_.x, self_.group1_.x, 0.0, 0.0)) + (vec4<f32>(vec2<f32>(other_groups.group4_.y), 0.0, 0.0) * vec4<f32>(self_.group0_.y, self_.group1_.y, 0.0, 0.0)) + (vec4<f32>(vec2<f32>(other_groups.group4_.z), 0.0, 0.0) * vec4<f32>(self_.group0_.z, self_.group1_.z, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(other_groups.group1_.w), 0.0, 0.0) * vec4<f32>(self_.group1_.w, self_.group0_.w, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group2_.x * self_.group1_.w) + (other_groups.group2_.z * self_.group0_.y) + (other_groups.group3_.y * self_.group1_.z) - (other_groups.group2_.y * self_.group0_.z) - (other_groups.group3_.x * self_.group0_.w), (other_groups.group2_.x * self_.group0_.z) + (other_groups.group2_.y * self_.group1_.w) + (other_groups.group3_.z * self_.group1_.x) - (other_groups.group2_.z * self_.group0_.x) - (other_groups.group3_.y * self_.group0_.w), (other_groups.group2_.y * self_.group0_.x) + (other_groups.group2_.z * self_.group1_.w) + (other_groups.group3_.x * self_.group1_.y) - (other_groups.group2_.x * self_.group0_.y) - (other_groups.group3_.z * self_.group0_.w), (other_groups.group2_.z * self_.group1_.z) * -1.0) + (vec4<f32>(other_groups.group0_.y) * self_.group0_) - (vec4<f32>(other_groups.group0_.xx.xy, other_groups.group0_.x, other_groups.group2_.x) * self_.group1_.xyzx) - (vec4<f32>(other_groups.group3_.zxy.xyz, other_groups.group2_.y) * self_.group1_.yzxy), 
        /* e41, e42, e43 */ vec4<f32>((self_.group1_.z * other_groups.group4_.y) - (self_.group1_.y * other_groups.group4_.z), (self_.group1_.x * other_groups.group4_.z) - (self_.group1_.z * other_groups.group4_.x), (self_.group1_.y * other_groups.group4_.x) - (self_.group1_.x * other_groups.group4_.y), 0.0) - (vec4<f32>(vec3<f32>(self_.group0_.w), 0.0) * vec4<f32>(other_groups.group4_.x, other_groups.group4_.y, other_groups.group4_.z, 0.0)) - (vec4<f32>(vec3<f32>(other_groups.group1_.w), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((self_.group0_.z * other_groups.group4_.y) + (self_.group1_.y * other_groups.group1_.z) - (self_.group0_.y * other_groups.group4_.z) - (self_.group1_.z * other_groups.group1_.y), (self_.group0_.x * other_groups.group4_.z) + (self_.group1_.z * other_groups.group1_.x) - (self_.group0_.z * other_groups.group4_.x) - (self_.group1_.x * other_groups.group1_.z), (self_.group0_.y * other_groups.group4_.x) + (self_.group1_.x * other_groups.group1_.y) - (self_.group0_.x * other_groups.group4_.y) - (self_.group1_.y * other_groups.group1_.x), 0.0) + (vec4<f32>(vec3<f32>(self_.group0_.w), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0)) + (vec4<f32>(vec3<f32>(other_groups.group4_.w), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0)) - (vec4<f32>(vec3<f32>(self_.group1_.w), 0.0) * vec4<f32>(other_groups.group4_.x, other_groups.group4_.y, other_groups.group4_.z, 0.0)) - (vec4<f32>(vec3<f32>(other_groups.group1_.w), 0.0) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group2_.x * self_.group0_.w) - (other_groups.group2_.y * self_.group1_.z), (other_groups.group2_.y * self_.group0_.w) - (other_groups.group2_.z * self_.group1_.x), (other_groups.group2_.z * self_.group0_.w) - (other_groups.group2_.x * self_.group1_.y), (other_groups.group3_.y * self_.group1_.y) + (other_groups.group3_.z * self_.group1_.z) - (other_groups.group0_.x * self_.group0_.w) - (other_groups.group2_.x * self_.group0_.x) - (other_groups.group2_.y * self_.group0_.y) - (other_groups.group2_.z * self_.group0_.z)) + (vec4<f32>(other_groups.group0_.y) * self_.group1_) + (vec4<f32>(other_groups.group2_.zxy.xyz, other_groups.group3_.x) * self_.group1_.yzxx)
    );
    let geometric_anti_product: MultiVector = multiVector_degroup(geometric_anti_product_groups);
    return multiVector_geometricAntiProduct_flector(geometric_anti_product, flector_antiReverse(self));
}
fn flector_antiSandwich_origin(self_: Flector, other: Origin) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = origin_grouped(other);
    let geometric_anti_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e4) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group0_.w) * vec4<f32>(-1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(other.e4) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group1_.w) * vec4<f32>(-1.0)
    );
    let geometric_anti_product: Motor = motor_degroup(geometric_anti_product_groups);
    return motor_geometricAntiProduct_flector(geometric_anti_product, flector_antiReverse(self));
}
fn flector_antiSandwich_plane(self_: Flector, other: Plane) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = plane_grouped(other);
    let geometric_anti_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(-(self_.group0_.w * other_groups.group0_.x) - (self_.group1_.y * other_groups.group0_.z), -(self_.group0_.w * other_groups.group0_.y) - (self_.group1_.z * other_groups.group0_.x), -(self_.group0_.w * other_groups.group0_.z) - (self_.group1_.x * other_groups.group0_.y), (self_.group1_.y * other_groups.group0_.y) + (self_.group1_.z * other_groups.group0_.z)) + (self_.group1_.zxyx * other_groups.group0_.yzxx), 
        /* e23, e31, e12, scalar */ vec4<f32>(-(self_.group0_.y * other_groups.group0_.z) - (self_.group1_.w * other_groups.group0_.x), -(self_.group0_.z * other_groups.group0_.x) - (self_.group1_.w * other_groups.group0_.y), -(self_.group0_.x * other_groups.group0_.y) - (self_.group1_.w * other_groups.group0_.z), (self_.group0_.z * other_groups.group0_.z) + (self_.group0_.w * other_groups.group0_.w)) + (vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group0_.y) * other_groups.group0_.wwwy) + (self_.group0_.zxyx * other_groups.group0_.yzxx)
    );
    let geometric_anti_product: Motor = motor_degroup(geometric_anti_product_groups);
    return motor_geometricAntiProduct_flector(geometric_anti_product, flector_antiReverse(self));
}
fn flector_antiSandwich_point(self_: Flector, other: Point) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = point_grouped(other);
    let geometric_anti_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other_groups.group0_.w) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group0_.w) * vec4<f32>(-1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>((self_.group0_.w * other_groups.group0_.x) + (self_.group1_.y * other_groups.group0_.z), (self_.group0_.w * other_groups.group0_.y) + (self_.group1_.z * other_groups.group0_.x), (self_.group0_.w * other_groups.group0_.z) + (self_.group1_.x * other_groups.group0_.y), -(self_.group1_.z * other_groups.group0_.z) - (self_.group1_.w * other_groups.group0_.w)) - (vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group1_.x) * other_groups.group0_.wwwx) - (self_.group1_.zxyy * other_groups.group0_.yzxy)
    );
    let geometric_anti_product: Motor = motor_degroup(geometric_anti_product_groups);
    return motor_geometricAntiProduct_flector(geometric_anti_product, flector_antiReverse(self));
}
fn flector_antiSandwich_scalar(self_: Flector, other: Scalar) -> Motor {
    let self_groups = flector_grouped(self_);
    let other_groups = scalar_grouped(other);
    let geometric_anti_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group1_.x * other.scalar * -1.0, self_.group1_.y * other.scalar * -1.0, self_.group1_.z * other.scalar * -1.0, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.group0_.w * other.scalar * -1.0)
    );
    let geometric_anti_product: Flector = flector_degroup(geometric_anti_product_groups);
    return flector_geometricAntiProduct_flector(geometric_anti_product, flector_antiReverse(self));
}
fn horizon_antiSandwich_flector(self_: Horizon, other: Flector) -> Flector {
    let self_groups = horizon_grouped(self_);
    let other_groups = flector_grouped(other);
    let geometric_anti_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(self.e321) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, other_groups.group0_.w) * vec4<f32>(-1.0)
    );
    let geometric_anti_product: Motor = motor_degroup(geometric_anti_product_groups);
    return motor_geometricAntiProduct_horizon(geometric_anti_product, horizon_antiReverse(self));
}
fn horizon_antiSandwich_line(self_: Horizon, other: Line) -> Scalar {
    let self_groups = horizon_grouped(self_);
    let other_groups = line_grouped(other);
    let geometric_anti_product_groups: PointGroups = PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group0_.x * self.e321, other_groups.group0_.y * self.e321, other_groups.group0_.z * self.e321, 0.0)
    );
    let geometric_anti_product: Point = point_degroup(geometric_anti_product_groups);
    return point_geometricAntiProduct_horizon(geometric_anti_product, horizon_antiReverse(self));
}
fn horizon_antiSandwich_motor(self_: Horizon, other: Motor) -> Motor {
    let self_groups = horizon_grouped(self_);
    let other_groups = motor_grouped(other);
    let geometric_anti_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group0_.x * self.e321, other_groups.group0_.y * self.e321, other_groups.group0_.z * self.e321, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group0_.w * self.e321)
    );
    let geometric_anti_product: Flector = flector_degroup(geometric_anti_product_groups);
    return flector_geometricAntiProduct_horizon(geometric_anti_product, horizon_antiReverse(self));
}
fn horizon_antiSandwich_multiVector(self_: Horizon, other: MultiVector) -> MultiVector {
    let self_groups = horizon_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let geometric_anti_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other_groups.group1_.w * self.e321 * -1.0, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group2_.x * self.e321, other_groups.group2_.y * self.e321, other_groups.group2_.z * self.e321, 0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(self.e321), 0.0) * vec4<f32>(other_groups.group4_.x, other_groups.group4_.y, other_groups.group4_.z, 0.0) * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group0_.y * self.e321)
    );
    let geometric_anti_product: MultiVector = multiVector_degroup(geometric_anti_product_groups);
    return multiVector_geometricAntiProduct_horizon(geometric_anti_product, horizon_antiReverse(self));
}
fn horizon_antiSandwich_plane(self_: Horizon, other: Plane) -> Point {
    let self_groups = horizon_grouped(self_);
    let other_groups = plane_grouped(other);
    let geometric_anti_product_groups: LineGroups = LineGroups(
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(self.e321), 0.0) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0) * vec3<f32>(-1.0)
    );
    let geometric_anti_product: Line = line_degroup(geometric_anti_product_groups);
    return line_geometricAntiProduct_horizon(geometric_anti_product, horizon_antiReverse(self));
}
fn line_antiSandwich_antiScalar(self_: Line, other: AntiScalar) -> Motor {
    let self_groups = line_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    let geometric_anti_product_groups: LineGroups = LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(other.e1234), 0.0) * self_.group0_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(other.e1234), 0.0) * self_.group1_
    );
    let geometric_anti_product: Line = line_degroup(geometric_anti_product_groups);
    return line_geometricAntiProduct_line(geometric_anti_product, line_antiReverse(self));
}
fn line_antiSandwich_dualNum(self_: Line, other: DualNum) -> Motor {
    let self_groups = line_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let geometric_anti_product_groups: LineGroups = LineGroups(
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(other_groups.group0_.y), 0.0) * self_.group0_, 
        /* e23, e31, e12 */ (vec4<f32>(vec3<f32>(other_groups.group0_.x), 0.0) * self_.group0_) + (vec4<f32>(vec3<f32>(other_groups.group0_.y), 0.0) * self_.group1_)
    );
    let geometric_anti_product: Line = line_degroup(geometric_anti_product_groups);
    return line_geometricAntiProduct_line(geometric_anti_product, line_antiReverse(self));
}
fn line_antiSandwich_flector(self_: Line, other: Flector) -> Flector {
    let self_groups = line_grouped(self_);
    let other_groups = flector_grouped(other);
    let geometric_anti_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group0_.x * other_groups.group1_.w) + (self_.group0_.y * other_groups.group0_.z) + (self_.group1_.x * other_groups.group0_.w) + (self_.group1_.y * other_groups.group1_.z) - (self_.group0_.z * other_groups.group0_.y), (self_.group0_.y * other_groups.group1_.w) + (self_.group0_.z * other_groups.group0_.x) + (self_.group1_.y * other_groups.group0_.w) + (self_.group1_.z * other_groups.group1_.x) - (self_.group0_.x * other_groups.group0_.z), (self_.group0_.x * other_groups.group0_.y) + (self_.group0_.z * other_groups.group1_.w) + (self_.group1_.x * other_groups.group1_.y) + (self_.group1_.z * other_groups.group0_.w) - (self_.group0_.y * other_groups.group0_.x), -(self_.group0_.y * other_groups.group1_.y) - (self_.group0_.z * other_groups.group1_.z)) - (vec4<f32>(self_.group1_.zxy.xyz, self_.group0_.x) * other_groups.group1_.yzxx), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.group0_.x * other_groups.group0_.w) + (self_.group0_.y * other_groups.group1_.z), (self_.group0_.y * other_groups.group0_.w) + (self_.group0_.z * other_groups.group1_.x), (self_.group0_.x * other_groups.group1_.y) + (self_.group0_.z * other_groups.group0_.w), -(self_.group0_.x * other_groups.group0_.x) - (self_.group0_.y * other_groups.group0_.y) - (self_.group0_.z * other_groups.group0_.z) - (self_.group1_.y * other_groups.group1_.y) - (self_.group1_.z * other_groups.group1_.z)) - (vec4<f32>(self_.group0_.zxy.xyz, self_.group1_.x) * other_groups.group1_.yzxx)
    );
    let geometric_anti_product: Flector = flector_degroup(geometric_anti_product_groups);
    return flector_geometricAntiProduct_line(geometric_anti_product, line_antiReverse(self));
}
fn line_antiSandwich_horizon(self_: Line, other: Horizon) -> Flector {
    let self_groups = line_grouped(self_);
    let other_groups = horizon_grouped(other);
    let geometric_anti_product_groups: PointGroups = PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group0_.x * other.e321, self_.group0_.y * other.e321, self_.group0_.z * other.e321, 0.0)
    );
    let geometric_anti_product: Point = point_degroup(geometric_anti_product_groups);
    return point_geometricAntiProduct_line(geometric_anti_product, line_antiReverse(self));
}
fn line_antiSandwich_line(self_: Line, other: Line) -> Motor {
    let self_groups = line_grouped(self_);
    let other_groups = line_grouped(other);
    let geometric_anti_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other_groups.group0_.z * self_.group0_.y) - (other_groups.group0_.y * self_.group0_.z), (other_groups.group0_.x * self_.group0_.z) - (other_groups.group0_.z * self_.group0_.x), (other_groups.group0_.y * self_.group0_.x) - (other_groups.group0_.x * self_.group0_.y), -(other_groups.group0_.x * self_.group0_.x) - (other_groups.group0_.y * self_.group0_.y) - (other_groups.group0_.z * self_.group0_.z)), 
        /* e23, e31, e12, scalar */ vec4<f32>((other_groups.group0_.z * self_.group1_.y) + (other_groups.group1_.z * self_.group0_.y) - (other_groups.group0_.y * self_.group1_.z) - (other_groups.group1_.y * self_.group0_.z), (other_groups.group0_.x * self_.group1_.z) + (other_groups.group1_.x * self_.group0_.z) - (other_groups.group0_.z * self_.group1_.x) - (other_groups.group1_.z * self_.group0_.x), (other_groups.group0_.y * self_.group1_.x) + (other_groups.group1_.y * self_.group0_.x) - (other_groups.group0_.x * self_.group1_.y) - (other_groups.group1_.x * self_.group0_.y), -(other_groups.group0_.x * self_.group1_.x) - (other_groups.group0_.y * self_.group1_.y) - (other_groups.group0_.z * self_.group1_.z) - (other_groups.group1_.x * self_.group0_.x) - (other_groups.group1_.y * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.z))
    );
    let geometric_anti_product: Motor = motor_degroup(geometric_anti_product_groups);
    return motor_geometricAntiProduct_line(geometric_anti_product, line_antiReverse(self));
}
fn line_antiSandwich_motor(self_: Line, other: Motor) -> Motor {
    let self_groups = line_grouped(self_);
    let other_groups = motor_grouped(other);
    let geometric_anti_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((self_.group0_.x * other_groups.group0_.w) + (self_.group0_.y * other_groups.group0_.z), (self_.group0_.y * other_groups.group0_.w) + (self_.group0_.z * other_groups.group0_.x), (self_.group0_.x * other_groups.group0_.y) + (self_.group0_.z * other_groups.group0_.w), -(self_.group0_.y * other_groups.group0_.y) - (self_.group0_.z * other_groups.group0_.z)) - (vec4<f32>(self_.group0_.zxy.xyz, self_.group0_.x) * other_groups.group0_.yzxx), 
        /* e23, e31, e12, scalar */ vec4<f32>((self_.group0_.x * other_groups.group1_.w) + (self_.group0_.y * other_groups.group1_.z) + (self_.group1_.x * other_groups.group0_.w) + (self_.group1_.y * other_groups.group0_.z), (self_.group0_.y * other_groups.group1_.w) + (self_.group0_.z * other_groups.group1_.x) + (self_.group1_.y * other_groups.group0_.w) + (self_.group1_.z * other_groups.group0_.x), (self_.group0_.x * other_groups.group1_.y) + (self_.group0_.z * other_groups.group1_.w) + (self_.group1_.x * other_groups.group0_.y) + (self_.group1_.z * other_groups.group0_.w), -(self_.group0_.y * other_groups.group1_.y) - (self_.group0_.z * other_groups.group1_.z) - (self_.group1_.y * other_groups.group0_.y) - (self_.group1_.z * other_groups.group0_.z)) - (vec4<f32>(self_.group0_.zxy.xyz, self_.group0_.x) * other_groups.group1_.yzxx) - (vec4<f32>(self_.group1_.zxy.xyz, self_.group1_.x) * other_groups.group0_.yzxx)
    );
    let geometric_anti_product: Motor = motor_degroup(geometric_anti_product_groups);
    return motor_geometricAntiProduct_line(geometric_anti_product, line_antiReverse(self));
}
fn line_antiSandwich_multiVector(self_: Line, other: MultiVector) -> MultiVector {
    let self_groups = line_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let geometric_anti_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(-(self_.group1_.x * other_groups.group2_.x) - (self_.group1_.y * other_groups.group2_.y) - (self_.group1_.z * other_groups.group2_.z), 0.0, 0.0, 0.0) - (vec4<f32>(vec2<f32>(self_.group0_.x), 0.0, 0.0) * vec4<f32>(other_groups.group3_.x, other_groups.group2_.x, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(self_.group0_.y), 0.0, 0.0) * vec4<f32>(other_groups.group3_.y, other_groups.group2_.y, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(self_.group0_.z), 0.0, 0.0) * vec4<f32>(other_groups.group3_.z, other_groups.group2_.z, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group0_.x * other_groups.group4_.w) + (self_.group0_.y * other_groups.group1_.z) + (self_.group1_.x * other_groups.group1_.w) + (self_.group1_.y * other_groups.group4_.z) - (self_.group0_.z * other_groups.group1_.y), (self_.group0_.y * other_groups.group4_.w) + (self_.group0_.z * other_groups.group1_.x) + (self_.group1_.y * other_groups.group1_.w) + (self_.group1_.z * other_groups.group4_.x) - (self_.group0_.x * other_groups.group1_.z), (self_.group0_.x * other_groups.group1_.y) + (self_.group0_.z * other_groups.group4_.w) + (self_.group1_.x * other_groups.group4_.y) + (self_.group1_.z * other_groups.group1_.w) - (self_.group0_.y * other_groups.group1_.x), -(self_.group0_.y * other_groups.group4_.y) - (self_.group0_.z * other_groups.group4_.z)) - (vec4<f32>(self_.group1_.zxy.xyz, self_.group0_.x) * other_groups.group4_.yzxx), 
        /* e41, e42, e43 */ (vec4<f32>(vec3<f32>(other_groups.group0_.y), 0.0) * self_.group0_) + (self_.group0_.yzx * other_groups.group2_.zxy) - (self_.group0_.zxy * other_groups.group2_.yzx), 
        /* e23, e31, e12 */ (vec4<f32>(vec3<f32>(other_groups.group0_.x), 0.0) * self_.group0_) + (vec4<f32>(vec3<f32>(other_groups.group0_.y), 0.0) * self_.group1_) + (self_.group0_.yzx * other_groups.group3_.zxy) + (self_.group1_.yzx * other_groups.group2_.zxy) - (self_.group0_.zxy * other_groups.group3_.yzx) - (self_.group1_.zxy * other_groups.group2_.yzx), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.group0_.x * other_groups.group1_.w) + (self_.group0_.y * other_groups.group4_.z), (self_.group0_.y * other_groups.group1_.w) + (self_.group0_.z * other_groups.group4_.x), (self_.group0_.x * other_groups.group4_.y) + (self_.group0_.z * other_groups.group1_.w), -(self_.group0_.x * other_groups.group1_.x) - (self_.group0_.y * other_groups.group1_.y) - (self_.group0_.z * other_groups.group1_.z) - (self_.group1_.y * other_groups.group4_.y) - (self_.group1_.z * other_groups.group4_.z)) - (vec4<f32>(self_.group0_.zxy.xyz, self_.group1_.x) * other_groups.group4_.yzxx)
    );
    let geometric_anti_product: MultiVector = multiVector_degroup(geometric_anti_product_groups);
    return multiVector_geometricAntiProduct_line(geometric_anti_product, line_antiReverse(self));
}
fn line_antiSandwich_origin(self_: Line, other: Origin) -> Flector {
    let self_groups = line_grouped(self_);
    let other_groups = origin_grouped(other);
    let geometric_anti_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group1_.x * other.e4, self_.group1_.y * other.e4, self_.group1_.z * other.e4, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.x * other.e4, self_.group0_.y * other.e4, self_.group0_.z * other.e4, 0.0)
    );
    let geometric_anti_product: Flector = flector_degroup(geometric_anti_product_groups);
    return flector_geometricAntiProduct_line(geometric_anti_product, line_antiReverse(self));
}
fn line_antiSandwich_plane(self_: Line, other: Plane) -> Flector {
    let self_groups = line_grouped(self_);
    let other_groups = plane_grouped(other);
    let geometric_anti_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group0_.x * other_groups.group0_.w) + (self_.group1_.y * other_groups.group0_.z), (self_.group0_.y * other_groups.group0_.w) + (self_.group1_.z * other_groups.group0_.x), (self_.group0_.z * other_groups.group0_.w) + (self_.group1_.x * other_groups.group0_.y), -(self_.group0_.y * other_groups.group0_.y) - (self_.group0_.z * other_groups.group0_.z)) - (vec4<f32>(self_.group1_.zxy.xyz, self_.group0_.x) * other_groups.group0_.yzxx), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.y * other_groups.group0_.z, self_.group0_.z * other_groups.group0_.x, self_.group0_.x * other_groups.group0_.y, -(self_.group1_.y * other_groups.group0_.y) - (self_.group1_.z * other_groups.group0_.z)) - (vec4<f32>(self_.group0_.zxy.xyz, self_.group1_.x) * other_groups.group0_.yzxx)
    );
    let geometric_anti_product: Flector = flector_degroup(geometric_anti_product_groups);
    return flector_geometricAntiProduct_line(geometric_anti_product, line_antiReverse(self));
}
fn line_antiSandwich_point(self_: Line, other: Point) -> Flector {
    let self_groups = line_grouped(self_);
    let other_groups = point_grouped(other);
    let geometric_anti_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group0_.y * other_groups.group0_.z) + (self_.group1_.x * other_groups.group0_.w) - (self_.group0_.z * other_groups.group0_.y), (self_.group0_.z * other_groups.group0_.x) + (self_.group1_.y * other_groups.group0_.w) - (self_.group0_.x * other_groups.group0_.z), (self_.group0_.x * other_groups.group0_.y) + (self_.group1_.z * other_groups.group0_.w) - (self_.group0_.y * other_groups.group0_.x), 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.x * other_groups.group0_.w, self_.group0_.y * other_groups.group0_.w, self_.group0_.z * other_groups.group0_.w, -(self_.group0_.x * other_groups.group0_.x) - (self_.group0_.y * other_groups.group0_.y) - (self_.group0_.z * other_groups.group0_.z))
    );
    let geometric_anti_product: Flector = flector_degroup(geometric_anti_product_groups);
    return flector_geometricAntiProduct_line(geometric_anti_product, line_antiReverse(self));
}
fn line_antiSandwich_scalar(self_: Line, other: Scalar) -> Motor {
    let self_groups = line_grouped(self_);
    let other_groups = scalar_grouped(other);
    let geometric_anti_product_groups: LineGroups = LineGroups(
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(other.scalar), 0.0) * self_.group0_
    );
    let geometric_anti_product: Line = line_degroup(geometric_anti_product_groups);
    return line_geometricAntiProduct_line(geometric_anti_product, line_antiReverse(self));
}
fn motor_antiSandwich_antiScalar(self_: Motor, other: AntiScalar) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    let geometric_anti_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other.e1234) * self_.group0_, 
        /* e23, e31, e12, scalar */ vec4<f32>(other.e1234) * self_.group1_
    );
    let geometric_anti_product: Motor = motor_degroup(geometric_anti_product_groups);
    return motor_geometricAntiProduct_motor(geometric_anti_product, motor_antiReverse(self));
}
fn motor_antiSandwich_dualNum(self_: Motor, other: DualNum) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let geometric_anti_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other_groups.group0_.y) * self_.group0_, 
        /* e23, e31, e12, scalar */ (vec4<f32>(other_groups.group0_.x) * self_.group0_) + (vec4<f32>(other_groups.group0_.y) * self_.group1_)
    );
    let geometric_anti_product: Motor = motor_degroup(geometric_anti_product_groups);
    return motor_geometricAntiProduct_motor(geometric_anti_product, motor_antiReverse(self));
}
fn motor_antiSandwich_flector(self_: Motor, other: Flector) -> Flector {
    let self_groups = motor_grouped(self_);
    let other_groups = flector_grouped(other);
    let geometric_anti_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group0_.z * self_.group0_.y) + (other_groups.group0_.w * self_.group1_.x) + (other_groups.group1_.x * self_.group1_.w) + (other_groups.group1_.z * self_.group1_.y) + (other_groups.group1_.w * self_.group0_.x), (other_groups.group0_.y * self_.group0_.w) + (other_groups.group0_.w * self_.group1_.y) + (other_groups.group1_.x * self_.group1_.z) + (other_groups.group1_.y * self_.group1_.w) + (other_groups.group1_.w * self_.group0_.y), (other_groups.group0_.z * self_.group0_.w) + (other_groups.group0_.w * self_.group1_.z) + (other_groups.group1_.y * self_.group1_.x) + (other_groups.group1_.z * self_.group1_.w) + (other_groups.group1_.w * self_.group0_.z), (other_groups.group1_.z * self_.group0_.z) * -1.0) + (other_groups.group0_.xxyw * self_.group0_.wzxw) - (vec4<f32>(other_groups.group0_.y, other_groups.group0_.z, other_groups.group0_.x, other_groups.group1_.x) * self_.group0_.zxyx) - (vec4<f32>(self_.group1_.z, self_.group1_.x, self_.group1_.y, self_.group0_.y) * other_groups.group1_.yzxy), 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group1_.z * self_.group0_.y, other_groups.group1_.y * self_.group0_.w, other_groups.group1_.z * self_.group0_.w, -(other_groups.group0_.y * self_.group0_.y) - (other_groups.group0_.z * self_.group0_.z) - (other_groups.group1_.x * self_.group1_.x) - (other_groups.group1_.y * self_.group1_.y) - (other_groups.group1_.z * self_.group1_.z)) + (vec4<f32>(other_groups.group0_.w) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group1_.w)) + (other_groups.group1_.xxyw * self_.group0_.wzxw) - (vec4<f32>(other_groups.group1_.y, other_groups.group1_.z, other_groups.group1_.x, other_groups.group0_.x) * self_.group0_.zxyx)
    );
    let geometric_anti_product: Flector = flector_degroup(geometric_anti_product_groups);
    return flector_geometricAntiProduct_motor(geometric_anti_product, motor_antiReverse(self));
}
fn motor_antiSandwich_horizon(self_: Motor, other: Horizon) -> Flector {
    let self_groups = motor_grouped(self_);
    let other_groups = horizon_grouped(other);
    let geometric_anti_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group0_.x * other.e321, self_.group0_.y * other.e321, self_.group0_.z * other.e321, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.group0_.w * other.e321)
    );
    let geometric_anti_product: Flector = flector_degroup(geometric_anti_product_groups);
    return flector_geometricAntiProduct_motor(geometric_anti_product, motor_antiReverse(self));
}
fn motor_antiSandwich_line(self_: Motor, other: Line) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = line_grouped(other);
    let geometric_anti_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other_groups.group0_.x * self_.group0_.w) + (other_groups.group0_.z * self_.group0_.y), (other_groups.group0_.x * self_.group0_.z) + (other_groups.group0_.y * self_.group0_.w), (other_groups.group0_.y * self_.group0_.x) + (other_groups.group0_.z * self_.group0_.w), -(other_groups.group0_.y * self_.group0_.y) - (other_groups.group0_.z * self_.group0_.z)) - (vec4<f32>(other_groups.group0_.yzx.xyz, other_groups.group0_.x) * self_.group0_.zxyx), 
        /* e23, e31, e12, scalar */ vec4<f32>((other_groups.group0_.x * self_.group1_.w) + (other_groups.group0_.z * self_.group1_.y) + (other_groups.group1_.x * self_.group0_.w) + (other_groups.group1_.z * self_.group0_.y), (other_groups.group0_.x * self_.group1_.z) + (other_groups.group0_.y * self_.group1_.w) + (other_groups.group1_.x * self_.group0_.z) + (other_groups.group1_.y * self_.group0_.w), (other_groups.group0_.y * self_.group1_.x) + (other_groups.group0_.z * self_.group1_.w) + (other_groups.group1_.y * self_.group0_.x) + (other_groups.group1_.z * self_.group0_.w), -(other_groups.group0_.y * self_.group1_.y) - (other_groups.group0_.z * self_.group1_.z) - (other_groups.group1_.y * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.z)) - (vec4<f32>(other_groups.group0_.yzx.xyz, other_groups.group0_.x) * self_.group1_.zxyx) - (vec4<f32>(other_groups.group1_.yzx.xyz, other_groups.group1_.x) * self_.group0_.zxyx)
    );
    let geometric_anti_product: Motor = motor_degroup(geometric_anti_product_groups);
    return motor_geometricAntiProduct_motor(geometric_anti_product, motor_antiReverse(self));
}
fn motor_antiSandwich_motor(self_: Motor, other: Motor) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = motor_grouped(other);
    let geometric_anti_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other_groups.group0_.z * self_.group0_.y) + (other_groups.group0_.w * self_.group0_.x), (other_groups.group0_.y * self_.group0_.w) + (other_groups.group0_.w * self_.group0_.y), (other_groups.group0_.z * self_.group0_.w) + (other_groups.group0_.w * self_.group0_.z), -(other_groups.group0_.y * self_.group0_.y) - (other_groups.group0_.z * self_.group0_.z)) + (other_groups.group0_.xxyw * self_.group0_.wzxw) - (other_groups.group0_.yzxx * self_.group0_.zxyx), 
        /* e23, e31, e12, scalar */ vec4<f32>((other_groups.group0_.z * self_.group1_.y) + (other_groups.group0_.w * self_.group1_.x) + (other_groups.group1_.z * self_.group0_.y) + (other_groups.group1_.w * self_.group0_.x), (other_groups.group0_.y * self_.group1_.w) + (other_groups.group0_.w * self_.group1_.y) + (other_groups.group1_.y * self_.group0_.w) + (other_groups.group1_.w * self_.group0_.y), (other_groups.group0_.z * self_.group1_.w) + (other_groups.group0_.w * self_.group1_.z) + (other_groups.group1_.z * self_.group0_.w) + (other_groups.group1_.w * self_.group0_.z), -(other_groups.group0_.y * self_.group1_.y) - (other_groups.group0_.z * self_.group1_.z) - (other_groups.group1_.y * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.z)) + (other_groups.group0_.xxyw * self_.group1_.wzxw) + (other_groups.group1_.xxyw * self_.group0_.wzxw) - (other_groups.group0_.yzxx * self_.group1_.zxyx) - (other_groups.group1_.yzxx * self_.group0_.zxyx)
    );
    let geometric_anti_product: Motor = motor_degroup(geometric_anti_product_groups);
    return motor_geometricAntiProduct_motor(geometric_anti_product, motor_antiReverse(self));
}
fn motor_antiSandwich_multiVector(self_: Motor, other: MultiVector) -> MultiVector {
    let self_groups = motor_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let geometric_anti_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((other_groups.group0_.y * self_.group1_.w) - (other_groups.group3_.x * self_.group0_.x) - (other_groups.group3_.y * self_.group0_.y) - (other_groups.group3_.z * self_.group0_.z), 0.0, 0.0, 0.0) + (vec4<f32>(vec2<f32>(self_.group0_.w), 0.0, 0.0) * other_groups.group0_) - (vec4<f32>(vec2<f32>(other_groups.group2_.x), 0.0, 0.0) * vec4<f32>(self_.group1_.x, self_.group0_.x, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(other_groups.group2_.y), 0.0, 0.0) * vec4<f32>(self_.group1_.y, self_.group0_.y, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(other_groups.group2_.z), 0.0, 0.0) * vec4<f32>(self_.group1_.z, self_.group0_.z, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group0_.y * other_groups.group1_.z) + (self_.group0_.w * other_groups.group1_.x) + (self_.group1_.x * other_groups.group1_.w) + (self_.group1_.y * other_groups.group4_.z) + (self_.group1_.w * other_groups.group4_.x), (self_.group0_.z * other_groups.group1_.x) + (self_.group0_.w * other_groups.group1_.y) + (self_.group1_.y * other_groups.group1_.w) + (self_.group1_.z * other_groups.group4_.x) + (self_.group1_.w * other_groups.group4_.y), (self_.group0_.z * other_groups.group4_.w) + (self_.group0_.w * other_groups.group1_.z) + (self_.group1_.x * other_groups.group4_.y) + (self_.group1_.z * other_groups.group1_.w) + (self_.group1_.w * other_groups.group4_.z), (self_.group0_.z * other_groups.group4_.z) * -1.0) + (vec4<f32>(other_groups.group4_.w, other_groups.group4_.w, other_groups.group1_.y, other_groups.group1_.w) * self_.group0_.xyxw) - (vec4<f32>(self_.group1_.z, self_.group1_.x, self_.group1_.y, self_.group0_.y) * other_groups.group4_.yzxy) - (vec4<f32>(other_groups.group1_.y, other_groups.group1_.z, other_groups.group1_.x, other_groups.group4_.x) * self_.group0_.zxyx), 
        /* e41, e42, e43 */ (vec4<f32>(vec3<f32>(other_groups.group0_.y), 0.0) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0)) + (vec4<f32>(self_.group0_.y, self_.group0_.w, self_.group0_.w, 0.0) * other_groups.group2_.zyz) + (vec4<f32>(self_.group0_.w, self_.group0_.z, self_.group0_.x, 0.0) * other_groups.group2_.xxy) - (vec4<f32>(self_.group0_.z, self_.group0_.x, self_.group0_.y, 0.0) * other_groups.group2_.yzx), 
        /* e23, e31, e12 */ (vec4<f32>(vec3<f32>(other_groups.group0_.x), 0.0) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0)) + (vec4<f32>(vec3<f32>(other_groups.group0_.y), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0)) + (vec4<f32>(self_.group0_.y, self_.group0_.w, self_.group0_.w, 0.0) * other_groups.group3_.zyz) + (vec4<f32>(self_.group0_.w, self_.group0_.z, self_.group0_.x, 0.0) * other_groups.group3_.xxy) + (vec4<f32>(self_.group1_.y, self_.group1_.w, self_.group1_.w, 0.0) * other_groups.group2_.zyz) + (vec4<f32>(self_.group1_.w, self_.group1_.z, self_.group1_.x, 0.0) * other_groups.group2_.xxy) - (vec4<f32>(self_.group0_.z, self_.group0_.x, self_.group0_.y, 0.0) * other_groups.group3_.yzx) - (vec4<f32>(self_.group1_.z, self_.group1_.x, self_.group1_.y, 0.0) * other_groups.group2_.yzx), 
        /* e423, e431, e412, e321 */ vec4<f32>((self_.group0_.y * other_groups.group4_.z) + (self_.group0_.w * other_groups.group4_.x), (self_.group0_.z * other_groups.group4_.x) + (self_.group0_.w * other_groups.group4_.y), (self_.group0_.z * other_groups.group1_.w) + (self_.group0_.w * other_groups.group4_.z), (self_.group1_.w * other_groups.group1_.w) - (self_.group0_.y * other_groups.group1_.y) - (self_.group0_.z * other_groups.group1_.z) - (self_.group1_.x * other_groups.group4_.x) - (self_.group1_.y * other_groups.group4_.y) - (self_.group1_.z * other_groups.group4_.z)) + (vec4<f32>(other_groups.group1_.w, other_groups.group1_.w, other_groups.group4_.y, other_groups.group4_.w) * self_.group0_.xyxw) - (vec4<f32>(other_groups.group4_.y, other_groups.group4_.z, other_groups.group4_.x, other_groups.group1_.x) * self_.group0_.zxyx)
    );
    let geometric_anti_product: MultiVector = multiVector_degroup(geometric_anti_product_groups);
    return multiVector_geometricAntiProduct_motor(geometric_anti_product, motor_antiReverse(self));
}
fn motor_antiSandwich_origin(self_: Motor, other: Origin) -> Flector {
    let self_groups = motor_grouped(self_);
    let other_groups = origin_grouped(other);
    let geometric_anti_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e4) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group0_.w), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e4) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group1_.w)
    );
    let geometric_anti_product: Flector = flector_degroup(geometric_anti_product_groups);
    return flector_geometricAntiProduct_motor(geometric_anti_product, motor_antiReverse(self));
}
fn motor_antiSandwich_plane(self_: Motor, other: Plane) -> Flector {
    let self_groups = motor_grouped(self_);
    let other_groups = plane_grouped(other);
    let geometric_anti_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group0_.x * other_groups.group0_.w) + (self_.group1_.y * other_groups.group0_.z) + (self_.group1_.w * other_groups.group0_.x), (self_.group0_.y * other_groups.group0_.w) + (self_.group1_.z * other_groups.group0_.x) + (self_.group1_.w * other_groups.group0_.y), (self_.group0_.z * other_groups.group0_.w) + (self_.group1_.x * other_groups.group0_.y) + (self_.group1_.w * other_groups.group0_.z), -(self_.group0_.y * other_groups.group0_.y) - (self_.group0_.z * other_groups.group0_.z)) - (vec4<f32>(self_.group1_.z, self_.group1_.x, self_.group1_.y, self_.group0_.x) * other_groups.group0_.yzxx), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.w * other_groups.group0_.x, self_.group0_.w * other_groups.group0_.y, self_.group0_.w * other_groups.group0_.z, -(self_.group1_.y * other_groups.group0_.y) - (self_.group1_.z * other_groups.group0_.z)) + (self_.group0_.yzxw * other_groups.group0_.zxyw) - (vec4<f32>(self_.group0_.z, self_.group0_.x, self_.group0_.y, self_.group1_.x) * other_groups.group0_.yzxx)
    );
    let geometric_anti_product: Flector = flector_degroup(geometric_anti_product_groups);
    return flector_geometricAntiProduct_motor(geometric_anti_product, motor_antiReverse(self));
}
fn motor_antiSandwich_point(self_: Motor, other: Point) -> Flector {
    let self_groups = motor_grouped(self_);
    let other_groups = point_grouped(other);
    let geometric_anti_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group0_.y * other_groups.group0_.z) + (self_.group0_.w * other_groups.group0_.x) + (self_.group1_.x * other_groups.group0_.w) - (self_.group0_.z * other_groups.group0_.y), (self_.group0_.z * other_groups.group0_.x) + (self_.group0_.w * other_groups.group0_.y) + (self_.group1_.y * other_groups.group0_.w) - (self_.group0_.x * other_groups.group0_.z), (self_.group0_.x * other_groups.group0_.y) + (self_.group0_.w * other_groups.group0_.z) + (self_.group1_.z * other_groups.group0_.w) - (self_.group0_.y * other_groups.group0_.x), self_.group0_.w * other_groups.group0_.w), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group0_.x * other_groups.group0_.w, self_.group0_.y * other_groups.group0_.w, self_.group0_.z * other_groups.group0_.w, (self_.group1_.w * other_groups.group0_.w) - (self_.group0_.x * other_groups.group0_.x) - (self_.group0_.y * other_groups.group0_.y) - (self_.group0_.z * other_groups.group0_.z))
    );
    let geometric_anti_product: Flector = flector_degroup(geometric_anti_product_groups);
    return flector_geometricAntiProduct_motor(geometric_anti_product, motor_antiReverse(self));
}
fn motor_antiSandwich_scalar(self_: Motor, other: Scalar) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = scalar_grouped(other);
    let geometric_anti_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(other.scalar) * self_.group0_
    );
    let geometric_anti_product: Motor = motor_degroup(geometric_anti_product_groups);
    return motor_geometricAntiProduct_motor(geometric_anti_product, motor_antiReverse(self));
}
fn multiVector_antiSandwich_antiScalar(self_: MultiVector, other: AntiScalar) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    let geometric_anti_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(other.e1234), 0.0, 0.0) * self_.group0_, 
        /* e1, e2, e3, e4 */ vec4<f32>(other.e1234) * self_.group1_, 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(other.e1234), 0.0) * self_.group2_, 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(other.e1234), 0.0) * self_.group3_, 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e1234) * self_.group4_
    );
    let geometric_anti_product: MultiVector = multiVector_degroup(geometric_anti_product_groups);
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self));
}
fn multiVector_antiSandwich_dualNum(self_: MultiVector, other: DualNum) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let geometric_anti_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((other_groups.group0_.x * self_.group0_.y) + (other_groups.group0_.y * self_.group0_.x), other_groups.group0_.y * self_.group0_.y, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group0_.y * self_.group1_.x) - (other_groups.group0_.x * self_.group4_.x), (other_groups.group0_.y * self_.group1_.y) - (other_groups.group0_.x * self_.group4_.y), (other_groups.group0_.y * self_.group1_.z) - (other_groups.group0_.x * self_.group4_.z), other_groups.group0_.y * self_.group1_.w), 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(other_groups.group0_.y), 0.0) * self_.group2_, 
        /* e23, e31, e12 */ (vec4<f32>(vec3<f32>(other_groups.group0_.x), 0.0) * self_.group2_) + (vec4<f32>(vec3<f32>(other_groups.group0_.y), 0.0) * self_.group3_), 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group0_.y * self_.group4_.x, other_groups.group0_.y * self_.group4_.y, other_groups.group0_.y * self_.group4_.z, (other_groups.group0_.y * self_.group4_.w) - (other_groups.group0_.x * self_.group1_.w))
    );
    let geometric_anti_product: MultiVector = multiVector_degroup(geometric_anti_product_groups);
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self));
}
fn multiVector_antiSandwich_flector(self_: MultiVector, other: Flector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = flector_grouped(other);
    let geometric_anti_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((other_groups.group1_.w * self_.group1_.w) - (other_groups.group0_.x * self_.group4_.x) - (other_groups.group0_.y * self_.group4_.y) - (other_groups.group0_.z * self_.group4_.z), 0.0, 0.0, 0.0) + (vec4<f32>(vec2<f32>(other_groups.group1_.x), 0.0, 0.0) * vec4<f32>(self_.group1_.x, self_.group4_.x, 0.0, 0.0)) + (vec4<f32>(vec2<f32>(other_groups.group1_.y), 0.0, 0.0) * vec4<f32>(self_.group1_.y, self_.group4_.y, 0.0, 0.0)) + (vec4<f32>(vec2<f32>(other_groups.group1_.z), 0.0, 0.0) * vec4<f32>(self_.group1_.z, self_.group4_.z, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(other_groups.group0_.w), 0.0, 0.0) * vec4<f32>(self_.group4_.w, self_.group1_.w, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group0_.x * other_groups.group1_.x) + (self_.group2_.x * other_groups.group1_.w) + (self_.group2_.y * other_groups.group0_.z) + (self_.group3_.x * other_groups.group0_.w) + (self_.group3_.y * other_groups.group1_.z) - (self_.group2_.z * other_groups.group0_.y), (self_.group0_.x * other_groups.group1_.y) + (self_.group2_.y * other_groups.group1_.w) + (self_.group2_.z * other_groups.group0_.x) + (self_.group3_.y * other_groups.group0_.w) + (self_.group3_.z * other_groups.group1_.x) - (self_.group2_.x * other_groups.group0_.z), (self_.group0_.x * other_groups.group1_.z) + (self_.group2_.x * other_groups.group0_.y) + (self_.group2_.z * other_groups.group1_.w) + (self_.group3_.x * other_groups.group1_.y) + (self_.group3_.z * other_groups.group0_.w) - (self_.group2_.y * other_groups.group0_.x), -(self_.group2_.y * other_groups.group1_.y) - (self_.group2_.z * other_groups.group1_.z)) + (vec4<f32>(self_.group0_.y) * other_groups.group0_) - (vec4<f32>(self_.group3_.zxy.xyz, self_.group2_.x) * other_groups.group1_.yzxx), 
        /* e41, e42, e43 */ vec4<f32>((other_groups.group1_.y * self_.group4_.z) - (other_groups.group1_.z * self_.group4_.y), (other_groups.group1_.z * self_.group4_.x) - (other_groups.group1_.x * self_.group4_.z), (other_groups.group1_.x * self_.group4_.y) - (other_groups.group1_.y * self_.group4_.x), 0.0) - (vec4<f32>(vec3<f32>(other_groups.group0_.w), 0.0) * vec4<f32>(self_.group4_.x, self_.group4_.y, self_.group4_.z, 0.0)) - (vec4<f32>(vec3<f32>(self_.group1_.w), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((other_groups.group0_.z * self_.group4_.y) + (other_groups.group1_.y * self_.group1_.z) - (other_groups.group0_.y * self_.group4_.z) - (other_groups.group1_.z * self_.group1_.y), (other_groups.group0_.x * self_.group4_.z) + (other_groups.group1_.z * self_.group1_.x) - (other_groups.group0_.z * self_.group4_.x) - (other_groups.group1_.x * self_.group1_.z), (other_groups.group0_.y * self_.group4_.x) + (other_groups.group1_.x * self_.group1_.y) - (other_groups.group0_.x * self_.group4_.y) - (other_groups.group1_.y * self_.group1_.x), 0.0) + (vec4<f32>(vec3<f32>(other_groups.group1_.w), 0.0) * vec4<f32>(self_.group4_.x, self_.group4_.y, self_.group4_.z, 0.0)) + (vec4<f32>(vec3<f32>(self_.group1_.w), 0.0) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0)) - (vec4<f32>(vec3<f32>(other_groups.group0_.w), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0)) - (vec4<f32>(vec3<f32>(self_.group4_.w), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group2_.y * other_groups.group1_.z, self_.group2_.z * other_groups.group1_.x, self_.group2_.x * other_groups.group1_.y, -(self_.group2_.x * other_groups.group0_.x) - (self_.group2_.y * other_groups.group0_.y) - (self_.group2_.z * other_groups.group0_.z) - (self_.group3_.y * other_groups.group1_.y) - (self_.group3_.z * other_groups.group1_.z)) + (vec4<f32>(self_.group0_.y) * other_groups.group1_) + (vec4<f32>(other_groups.group0_.w) * vec4<f32>(self_.group2_.xyz.xyz, self_.group0_.x)) - (vec4<f32>(self_.group2_.zxy.xyz, self_.group3_.x) * other_groups.group1_.yzxx)
    );
    let geometric_anti_product: MultiVector = multiVector_degroup(geometric_anti_product_groups);
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self));
}
fn multiVector_antiSandwich_horizon(self_: MultiVector, other: Horizon) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = horizon_grouped(other);
    let geometric_anti_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.group1_.w * other.e321, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group2_.x * other.e321, self_.group2_.y * other.e321, self_.group2_.z * other.e321, 0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(other.e321), 0.0) * vec4<f32>(self_.group4_.x, self_.group4_.y, self_.group4_.z, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.group0_.y * other.e321)
    );
    let geometric_anti_product: MultiVector = multiVector_degroup(geometric_anti_product_groups);
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self));
}
fn multiVector_antiSandwich_line(self_: MultiVector, other: Line) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = line_grouped(other);
    let geometric_anti_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(-(other_groups.group1_.x * self_.group2_.x) - (other_groups.group1_.y * self_.group2_.y) - (other_groups.group1_.z * self_.group2_.z), 0.0, 0.0, 0.0) - (vec4<f32>(vec2<f32>(other_groups.group0_.x), 0.0, 0.0) * vec4<f32>(self_.group3_.x, self_.group2_.x, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(other_groups.group0_.y), 0.0, 0.0) * vec4<f32>(self_.group3_.y, self_.group2_.y, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(other_groups.group0_.z), 0.0, 0.0) * vec4<f32>(self_.group3_.z, self_.group2_.z, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group0_.x * self_.group4_.w) + (other_groups.group0_.z * self_.group1_.y) + (other_groups.group1_.y * self_.group4_.z) - (other_groups.group0_.y * self_.group1_.z) - (other_groups.group1_.x * self_.group1_.w), (other_groups.group0_.x * self_.group1_.z) + (other_groups.group0_.y * self_.group4_.w) + (other_groups.group1_.z * self_.group4_.x) - (other_groups.group0_.z * self_.group1_.x) - (other_groups.group1_.y * self_.group1_.w), (other_groups.group0_.y * self_.group1_.x) + (other_groups.group0_.z * self_.group4_.w) + (other_groups.group1_.x * self_.group4_.y) - (other_groups.group0_.x * self_.group1_.y) - (other_groups.group1_.z * self_.group1_.w), -(other_groups.group0_.y * self_.group4_.y) - (other_groups.group0_.z * self_.group4_.z)) - (vec4<f32>(other_groups.group1_.zxy.xyz, other_groups.group0_.x) * self_.group4_.yzxx), 
        /* e41, e42, e43 */ (vec4<f32>(vec3<f32>(self_.group0_.y), 0.0) * other_groups.group0_) + (other_groups.group0_.zxy * self_.group2_.yzx) - (other_groups.group0_.yzx * self_.group2_.zxy), 
        /* e23, e31, e12 */ (vec4<f32>(vec3<f32>(self_.group0_.x), 0.0) * other_groups.group0_) + (vec4<f32>(vec3<f32>(self_.group0_.y), 0.0) * other_groups.group1_) + (other_groups.group0_.zxy * self_.group3_.yzx) + (other_groups.group1_.zxy * self_.group2_.yzx) - (other_groups.group0_.yzx * self_.group3_.zxy) - (other_groups.group1_.yzx * self_.group2_.zxy), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.x * self_.group1_.w) - (other_groups.group0_.y * self_.group4_.z), (other_groups.group0_.y * self_.group1_.w) - (other_groups.group0_.z * self_.group4_.x), (other_groups.group0_.z * self_.group1_.w) - (other_groups.group0_.x * self_.group4_.y), (other_groups.group1_.y * self_.group4_.y) + (other_groups.group1_.z * self_.group4_.z) - (other_groups.group0_.x * self_.group1_.x) - (other_groups.group0_.y * self_.group1_.y) - (other_groups.group0_.z * self_.group1_.z)) + (vec4<f32>(other_groups.group0_.zxy.xyz, other_groups.group1_.x) * self_.group4_.yzxx)
    );
    let geometric_anti_product: MultiVector = multiVector_degroup(geometric_anti_product_groups);
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self));
}
fn multiVector_antiSandwich_motor(self_: MultiVector, other: Motor) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = motor_grouped(other);
    let geometric_anti_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((self_.group0_.y * other_groups.group1_.w) - (self_.group3_.x * other_groups.group0_.x) - (self_.group3_.y * other_groups.group0_.y) - (self_.group3_.z * other_groups.group0_.z), 0.0, 0.0, 0.0) + (vec4<f32>(vec2<f32>(other_groups.group0_.w), 0.0, 0.0) * self_.group0_) - (vec4<f32>(vec2<f32>(self_.group2_.x), 0.0, 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group0_.x, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(self_.group2_.y), 0.0, 0.0) * vec4<f32>(other_groups.group1_.y, other_groups.group0_.y, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(self_.group2_.z), 0.0, 0.0) * vec4<f32>(other_groups.group1_.z, other_groups.group0_.z, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group0_.z * self_.group1_.y) + (other_groups.group0_.w * self_.group1_.x) + (other_groups.group1_.y * self_.group4_.z) - (other_groups.group1_.x * self_.group1_.w), (other_groups.group0_.y * self_.group4_.w) + (other_groups.group0_.w * self_.group1_.y) + (other_groups.group1_.z * self_.group4_.x) - (other_groups.group1_.y * self_.group1_.w), (other_groups.group0_.z * self_.group4_.w) + (other_groups.group0_.w * self_.group1_.z) + (other_groups.group1_.x * self_.group4_.y) - (other_groups.group1_.z * self_.group1_.w), 0.0) + (vec4<f32>(self_.group4_.w, self_.group1_.z, self_.group1_.x, self_.group1_.w) * other_groups.group0_.xxyw) - (vec4<f32>(other_groups.group1_.z, other_groups.group1_.x, other_groups.group1_.y, other_groups.group0_.y) * self_.group4_.yzxy) - (vec4<f32>(other_groups.group1_.w, other_groups.group1_.w, other_groups.group1_.w, other_groups.group0_.z) * self_.group4_.xyzz) - (vec4<f32>(self_.group1_.z, self_.group1_.x, self_.group1_.y, self_.group4_.x) * other_groups.group0_.yzxx), 
        /* e41, e42, e43 */ (vec4<f32>(vec3<f32>(self_.group0_.y), 0.0) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0)) + (vec4<f32>(other_groups.group0_.z, other_groups.group0_.x, other_groups.group0_.w, 0.0) * self_.group2_.yzz) + (vec4<f32>(other_groups.group0_.w, other_groups.group0_.w, other_groups.group0_.y, 0.0) * self_.group2_.xyx) - (vec4<f32>(other_groups.group0_.y, other_groups.group0_.z, other_groups.group0_.x, 0.0) * self_.group2_.zxy), 
        /* e23, e31, e12 */ (vec4<f32>(vec3<f32>(self_.group0_.x), 0.0) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0)) + (vec4<f32>(vec3<f32>(self_.group0_.y), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0)) + (vec4<f32>(other_groups.group0_.z, other_groups.group0_.x, other_groups.group0_.w, 0.0) * self_.group3_.yzz) + (vec4<f32>(other_groups.group0_.w, other_groups.group0_.w, other_groups.group0_.y, 0.0) * self_.group3_.xyx) + (vec4<f32>(other_groups.group1_.z, other_groups.group1_.x, other_groups.group1_.w, 0.0) * self_.group2_.yzz) + (vec4<f32>(other_groups.group1_.w, other_groups.group1_.w, other_groups.group1_.y, 0.0) * self_.group2_.xyx) - (vec4<f32>(other_groups.group0_.y, other_groups.group0_.z, other_groups.group0_.x, 0.0) * self_.group3_.zxy) - (vec4<f32>(other_groups.group1_.y, other_groups.group1_.z, other_groups.group1_.x, 0.0) * self_.group2_.zxy), 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group0_.w * self_.group4_.x, other_groups.group0_.y * self_.group1_.w, other_groups.group0_.z * self_.group1_.w, (other_groups.group1_.y * self_.group4_.y) + (other_groups.group1_.z * self_.group4_.z) - (other_groups.group0_.y * self_.group1_.y) - (other_groups.group0_.z * self_.group1_.z) - (other_groups.group1_.w * self_.group1_.w)) + (vec4<f32>(other_groups.group0_.z, other_groups.group0_.w, other_groups.group0_.w, other_groups.group1_.x) * self_.group4_.yyzx) + (vec4<f32>(self_.group1_.w, self_.group4_.z, self_.group4_.x, self_.group4_.w) * other_groups.group0_.xxyw) - (vec4<f32>(self_.group4_.z, self_.group4_.x, self_.group4_.y, self_.group1_.x) * other_groups.group0_.yzxx)
    );
    let geometric_anti_product: MultiVector = multiVector_degroup(geometric_anti_product_groups);
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self));
}
fn multiVector_antiSandwich_multiVector(self_: MultiVector, other: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let geometric_anti_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((other_groups.group0_.y * self_.group0_.x) + (other_groups.group4_.w * self_.group1_.w) - (other_groups.group3_.x * self_.group2_.x) - (other_groups.group3_.y * self_.group2_.y) - (other_groups.group3_.z * self_.group2_.z) - (other_groups.group1_.x * self_.group4_.x) - (other_groups.group1_.y * self_.group4_.y) - (other_groups.group1_.z * self_.group4_.z), 0.0, 0.0, 0.0) + (vec4<f32>(vec2<f32>(self_.group0_.y), 0.0, 0.0) * other_groups.group0_) + (vec4<f32>(vec2<f32>(other_groups.group4_.x), 0.0, 0.0) * vec4<f32>(self_.group1_.x, self_.group4_.x, 0.0, 0.0)) + (vec4<f32>(vec2<f32>(other_groups.group4_.y), 0.0, 0.0) * vec4<f32>(self_.group1_.y, self_.group4_.y, 0.0, 0.0)) + (vec4<f32>(vec2<f32>(other_groups.group4_.z), 0.0, 0.0) * vec4<f32>(self_.group1_.z, self_.group4_.z, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(other_groups.group2_.x), 0.0, 0.0) * vec4<f32>(self_.group3_.x, self_.group2_.x, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(other_groups.group2_.y), 0.0, 0.0) * vec4<f32>(self_.group3_.y, self_.group2_.y, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(other_groups.group2_.z), 0.0, 0.0) * vec4<f32>(self_.group3_.z, self_.group2_.z, 0.0, 0.0)) - (vec4<f32>(vec2<f32>(other_groups.group1_.w), 0.0, 0.0) * vec4<f32>(self_.group4_.w, self_.group1_.w, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group0_.x * other_groups.group4_.x) + (other_groups.group2_.x * self_.group4_.w) + (other_groups.group2_.z * self_.group1_.y) + (other_groups.group3_.y * self_.group4_.z) + (self_.group2_.x * other_groups.group4_.w) + (self_.group2_.y * other_groups.group1_.z) + (self_.group3_.x * other_groups.group1_.w) + (self_.group3_.y * other_groups.group4_.z) - (other_groups.group2_.y * self_.group1_.z) - (other_groups.group3_.x * self_.group1_.w) - (self_.group2_.z * other_groups.group1_.y), (self_.group0_.x * other_groups.group4_.y) + (other_groups.group2_.x * self_.group1_.z) + (other_groups.group2_.y * self_.group4_.w) + (other_groups.group3_.z * self_.group4_.x) + (self_.group2_.y * other_groups.group4_.w) + (self_.group2_.z * other_groups.group1_.x) + (self_.group3_.y * other_groups.group1_.w) + (self_.group3_.z * other_groups.group4_.x) - (other_groups.group2_.z * self_.group1_.x) - (other_groups.group3_.y * self_.group1_.w) - (self_.group2_.x * other_groups.group1_.z), (self_.group0_.x * other_groups.group4_.z) + (other_groups.group2_.y * self_.group1_.x) + (other_groups.group2_.z * self_.group4_.w) + (other_groups.group3_.x * self_.group4_.y) + (self_.group2_.x * other_groups.group1_.y) + (self_.group2_.z * other_groups.group4_.w) + (self_.group3_.x * other_groups.group4_.y) + (self_.group3_.z * other_groups.group1_.w) - (other_groups.group2_.x * self_.group1_.y) - (other_groups.group3_.z * self_.group1_.w) - (self_.group2_.y * other_groups.group1_.x), -(other_groups.group2_.z * self_.group4_.z) - (self_.group2_.y * other_groups.group4_.y) - (self_.group2_.z * other_groups.group4_.z)) + (vec4<f32>(other_groups.group0_.y) * self_.group1_) + (vec4<f32>(self_.group0_.y) * other_groups.group1_) - (vec4<f32>(other_groups.group0_.xx.xy, other_groups.group0_.x, other_groups.group2_.x) * self_.group4_.xyzx) - (vec4<f32>(other_groups.group3_.zxy.xyz, other_groups.group2_.y) * self_.group4_.yzxy) - (vec4<f32>(self_.group3_.zxy.xyz, self_.group2_.x) * other_groups.group4_.yzxx), 
        /* e41, e42, e43 */ vec4<f32>((other_groups.group4_.y * self_.group4_.z) - (other_groups.group4_.z * self_.group4_.y), (other_groups.group4_.z * self_.group4_.x) - (other_groups.group4_.x * self_.group4_.z), (other_groups.group4_.x * self_.group4_.y) - (other_groups.group4_.y * self_.group4_.x), 0.0) + (vec4<f32>(vec3<f32>(other_groups.group0_.y), 0.0) * self_.group2_) + (vec4<f32>(vec3<f32>(self_.group0_.y), 0.0) * other_groups.group2_) + (other_groups.group2_.zxy * self_.group2_.yzx) - (vec4<f32>(vec3<f32>(other_groups.group1_.w), 0.0) * vec4<f32>(self_.group4_.x, self_.group4_.y, self_.group4_.z, 0.0)) - (vec4<f32>(vec3<f32>(self_.group1_.w), 0.0) * vec4<f32>(other_groups.group4_.x, other_groups.group4_.y, other_groups.group4_.z, 0.0)) - (other_groups.group2_.yzx * self_.group2_.zxy), 
        /* e23, e31, e12 */ vec4<f32>((other_groups.group1_.z * self_.group4_.y) + (other_groups.group4_.y * self_.group1_.z) - (other_groups.group1_.y * self_.group4_.z) - (other_groups.group4_.z * self_.group1_.y), (other_groups.group1_.x * self_.group4_.z) + (other_groups.group4_.z * self_.group1_.x) - (other_groups.group1_.z * self_.group4_.x) - (other_groups.group4_.x * self_.group1_.z), (other_groups.group1_.y * self_.group4_.x) + (other_groups.group4_.x * self_.group1_.y) - (other_groups.group1_.x * self_.group4_.y) - (other_groups.group4_.y * self_.group1_.x), 0.0) + (vec4<f32>(vec3<f32>(other_groups.group0_.x), 0.0) * self_.group2_) + (vec4<f32>(vec3<f32>(other_groups.group0_.y), 0.0) * self_.group3_) + (vec4<f32>(vec3<f32>(self_.group0_.x), 0.0) * other_groups.group2_) + (vec4<f32>(vec3<f32>(self_.group0_.y), 0.0) * other_groups.group3_) + (vec4<f32>(vec3<f32>(other_groups.group4_.w), 0.0) * vec4<f32>(self_.group4_.x, self_.group4_.y, self_.group4_.z, 0.0)) + (vec4<f32>(vec3<f32>(self_.group1_.w), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0)) + (other_groups.group2_.zxy * self_.group3_.yzx) + (other_groups.group3_.zxy * self_.group2_.yzx) - (vec4<f32>(vec3<f32>(other_groups.group1_.w), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0)) - (vec4<f32>(vec3<f32>(self_.group4_.w), 0.0) * vec4<f32>(other_groups.group4_.x, other_groups.group4_.y, other_groups.group4_.z, 0.0)) - (other_groups.group2_.yzx * self_.group3_.zxy) - (other_groups.group3_.yzx * self_.group2_.zxy), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group2_.x * self_.group1_.w) + (self_.group2_.y * other_groups.group4_.z) - (other_groups.group2_.y * self_.group4_.z), (other_groups.group2_.y * self_.group1_.w) + (self_.group2_.z * other_groups.group4_.x) - (other_groups.group2_.z * self_.group4_.x), (other_groups.group2_.z * self_.group1_.w) + (self_.group2_.x * other_groups.group4_.y) - (other_groups.group2_.x * self_.group4_.y), (other_groups.group3_.y * self_.group4_.y) + (other_groups.group3_.z * self_.group4_.z) - (other_groups.group0_.x * self_.group1_.w) - (other_groups.group2_.x * self_.group1_.x) - (other_groups.group2_.y * self_.group1_.y) - (other_groups.group2_.z * self_.group1_.z) - (self_.group2_.x * other_groups.group1_.x) - (self_.group2_.y * other_groups.group1_.y) - (self_.group2_.z * other_groups.group1_.z) - (self_.group3_.y * other_groups.group4_.y) - (self_.group3_.z * other_groups.group4_.z)) + (vec4<f32>(other_groups.group0_.y) * self_.group4_) + (vec4<f32>(self_.group0_.y) * other_groups.group4_) + (vec4<f32>(other_groups.group1_.w) * vec4<f32>(self_.group2_.xyz.xyz, self_.group0_.x)) + (vec4<f32>(other_groups.group2_.zxy.xyz, other_groups.group3_.x) * self_.group4_.yzxx) - (vec4<f32>(self_.group2_.zxy.xyz, self_.group3_.x) * other_groups.group4_.yzxx)
    );
    let geometric_anti_product: MultiVector = multiVector_degroup(geometric_anti_product_groups);
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self));
}
fn multiVector_antiSandwich_origin(self_: MultiVector, other: Origin) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = origin_grouped(other);
    let geometric_anti_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(other.e4), 0.0, 0.0) * vec4<f32>(self_.group4_.w, self_.group1_.w, 0.0, 0.0) * vec2<f32>(-1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(other.e4) * vec4<f32>(self_.group3_.xyz.xyz, self_.group0_.y), 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(other.e4), 0.0) * vec4<f32>(self_.group4_.x, self_.group4_.y, self_.group4_.z, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(other.e4), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0) * vec3<f32>(-1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other.e4) * vec4<f32>(self_.group2_.xyz.xyz, self_.group0_.x)
    );
    let geometric_anti_product: MultiVector = multiVector_degroup(geometric_anti_product_groups);
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self));
}
fn multiVector_antiSandwich_plane(self_: MultiVector, other: Plane) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = plane_grouped(other);
    let geometric_anti_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.group1_.w * other_groups.group0_.w, 0.0, 0.0, 0.0) + (vec4<f32>(vec2<f32>(other_groups.group0_.x), 0.0, 0.0) * vec4<f32>(self_.group1_.x, self_.group4_.x, 0.0, 0.0)) + (vec4<f32>(vec2<f32>(other_groups.group0_.y), 0.0, 0.0) * vec4<f32>(self_.group1_.y, self_.group4_.y, 0.0, 0.0)) + (vec4<f32>(vec2<f32>(other_groups.group0_.z), 0.0, 0.0) * vec4<f32>(self_.group1_.z, self_.group4_.z, 0.0, 0.0)), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group0_.x * other_groups.group0_.x) + (self_.group2_.x * other_groups.group0_.w) + (self_.group3_.y * other_groups.group0_.z), (self_.group0_.x * other_groups.group0_.y) + (self_.group2_.y * other_groups.group0_.w) + (self_.group3_.z * other_groups.group0_.x), (self_.group0_.x * other_groups.group0_.z) + (self_.group2_.z * other_groups.group0_.w) + (self_.group3_.x * other_groups.group0_.y), -(self_.group2_.y * other_groups.group0_.y) - (self_.group2_.z * other_groups.group0_.z)) - (vec4<f32>(self_.group3_.zxy.xyz, self_.group2_.x) * other_groups.group0_.yzxx), 
        /* e41, e42, e43 */ vec4<f32>((self_.group4_.z * other_groups.group0_.y) - (self_.group4_.y * other_groups.group0_.z), (self_.group4_.x * other_groups.group0_.z) - (self_.group4_.z * other_groups.group0_.x), (self_.group4_.y * other_groups.group0_.x) - (self_.group4_.x * other_groups.group0_.y), 0.0) - (vec4<f32>(vec3<f32>(self_.group1_.w), 0.0) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((self_.group1_.z * other_groups.group0_.y) - (self_.group1_.y * other_groups.group0_.z), (self_.group1_.x * other_groups.group0_.z) - (self_.group1_.z * other_groups.group0_.x), (self_.group1_.y * other_groups.group0_.x) - (self_.group1_.x * other_groups.group0_.y), 0.0) + (vec4<f32>(vec3<f32>(other_groups.group0_.w), 0.0) * vec4<f32>(self_.group4_.x, self_.group4_.y, self_.group4_.z, 0.0)) - (vec4<f32>(vec3<f32>(self_.group4_.w), 0.0) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group2_.y * other_groups.group0_.z, self_.group2_.z * other_groups.group0_.x, self_.group2_.x * other_groups.group0_.y, -(self_.group3_.y * other_groups.group0_.y) - (self_.group3_.z * other_groups.group0_.z)) + (vec4<f32>(self_.group0_.y) * other_groups.group0_) - (vec4<f32>(self_.group2_.zxy.xyz, self_.group3_.x) * other_groups.group0_.yzxx)
    );
    let geometric_anti_product: MultiVector = multiVector_degroup(geometric_anti_product_groups);
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self));
}
fn multiVector_antiSandwich_point(self_: MultiVector, other: Point) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = point_grouped(other);
    let geometric_anti_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(-(self_.group4_.x * other_groups.group0_.x) - (self_.group4_.y * other_groups.group0_.y) - (self_.group4_.z * other_groups.group0_.z) - (self_.group4_.w * other_groups.group0_.w), self_.group1_.w * other_groups.group0_.w * -1.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((self_.group0_.y * other_groups.group0_.x) + (self_.group2_.y * other_groups.group0_.z) + (self_.group3_.x * other_groups.group0_.w) - (self_.group2_.z * other_groups.group0_.y), (self_.group0_.y * other_groups.group0_.y) + (self_.group2_.z * other_groups.group0_.x) + (self_.group3_.y * other_groups.group0_.w) - (self_.group2_.x * other_groups.group0_.z), (self_.group0_.y * other_groups.group0_.z) + (self_.group2_.x * other_groups.group0_.y) + (self_.group3_.z * other_groups.group0_.w) - (self_.group2_.y * other_groups.group0_.x), self_.group0_.y * other_groups.group0_.w), 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(other_groups.group0_.w), 0.0) * vec4<f32>(self_.group4_.x, self_.group4_.y, self_.group4_.z, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>((self_.group4_.y * other_groups.group0_.z) - (self_.group4_.z * other_groups.group0_.y), (self_.group4_.z * other_groups.group0_.x) - (self_.group4_.x * other_groups.group0_.z), (self_.group4_.x * other_groups.group0_.y) - (self_.group4_.y * other_groups.group0_.x), 0.0) + (vec4<f32>(vec3<f32>(self_.group1_.w), 0.0) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, 0.0)) - (vec4<f32>(vec3<f32>(other_groups.group0_.w), 0.0) * vec4<f32>(self_.group1_.x, self_.group1_.y, self_.group1_.z, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>(self_.group2_.x * other_groups.group0_.w, self_.group2_.y * other_groups.group0_.w, self_.group2_.z * other_groups.group0_.w, (self_.group0_.x * other_groups.group0_.w) - (self_.group2_.x * other_groups.group0_.x) - (self_.group2_.y * other_groups.group0_.y) - (self_.group2_.z * other_groups.group0_.z))
    );
    let geometric_anti_product: MultiVector = multiVector_degroup(geometric_anti_product_groups);
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self));
}
fn multiVector_antiSandwich_scalar(self_: MultiVector, other: Scalar) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = scalar_grouped(other);
    let geometric_anti_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(self_.group0_.y * other.scalar, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group4_.x * other.scalar * -1.0, self_.group4_.y * other.scalar * -1.0, self_.group4_.z * other.scalar * -1.0, 0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(other.scalar), 0.0) * self_.group2_, 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, self_.group1_.w * other.scalar * -1.0)
    );
    let geometric_anti_product: MultiVector = multiVector_degroup(geometric_anti_product_groups);
    return multiVector_geometricAntiProduct_multiVector(geometric_anti_product, multiVector_antiReverse(self));
}
fn origin_antiSandwich_antiScalar(self_: Origin, other: AntiScalar) -> AntiScalar {
    let self_groups = origin_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    let geometric_anti_product_groups: OriginGroups = OriginGroups(
        /* e4 */ vec4<f32>(other.e1234 * self.e4, 0.0, 0.0, 0.0)
    );
    let geometric_anti_product: Origin = origin_degroup(geometric_anti_product_groups);
    return origin_geometricAntiProduct_origin(geometric_anti_product, origin_antiReverse(self));
}
fn origin_antiSandwich_dualNum(self_: Origin, other: DualNum) -> Motor {
    let self_groups = origin_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let geometric_anti_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group0_.y * self.e4), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group0_.x * self.e4 * -1.0)
    );
    let geometric_anti_product: Flector = flector_degroup(geometric_anti_product_groups);
    return flector_geometricAntiProduct_origin(geometric_anti_product, origin_antiReverse(self));
}
fn origin_antiSandwich_flector(self_: Origin, other: Flector) -> Flector {
    let self_groups = origin_grouped(self_);
    let other_groups = flector_grouped(other);
    let geometric_anti_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self.e4) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, other_groups.group0_.w) * vec4<f32>(-1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(self.e4) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, other_groups.group1_.w)
    );
    let geometric_anti_product: Motor = motor_degroup(geometric_anti_product_groups);
    return motor_geometricAntiProduct_origin(geometric_anti_product, origin_antiReverse(self));
}
fn origin_antiSandwich_horizon(self_: Origin, other: Horizon) -> Horizon {
    let self_groups = origin_grouped(self_);
    let other_groups = horizon_grouped(other);
    let geometric_anti_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(other.e321 * self.e4, 0.0, 0.0, 0.0)
    );
    let geometric_anti_product: Scalar = scalar_degroup(geometric_anti_product_groups);
    return scalar_geometricAntiProduct_origin(geometric_anti_product, origin_antiReverse(self));
}
fn origin_antiSandwich_line(self_: Origin, other: Line) -> Motor {
    let self_groups = origin_grouped(self_);
    let other_groups = line_grouped(other);
    let geometric_anti_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group1_.x * self.e4 * -1.0, other_groups.group1_.y * self.e4 * -1.0, other_groups.group1_.z * self.e4 * -1.0, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group0_.x * self.e4, other_groups.group0_.y * self.e4, other_groups.group0_.z * self.e4, 0.0)
    );
    let geometric_anti_product: Flector = flector_degroup(geometric_anti_product_groups);
    return flector_geometricAntiProduct_origin(geometric_anti_product, origin_antiReverse(self));
}
fn origin_antiSandwich_motor(self_: Origin, other: Motor) -> Motor {
    let self_groups = origin_grouped(self_);
    let other_groups = motor_grouped(other);
    let geometric_anti_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self.e4) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, other_groups.group0_.w) * vec4<f32>(-1.0, -1.0, -1.0, 1.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self.e4) * vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, other_groups.group1_.w) * vec4<f32>(1.0, 1.0, 1.0, -1.0)
    );
    let geometric_anti_product: Flector = flector_degroup(geometric_anti_product_groups);
    return flector_geometricAntiProduct_origin(geometric_anti_product, origin_antiReverse(self));
}
fn origin_antiSandwich_multiVector(self_: Origin, other: MultiVector) -> MultiVector {
    let self_groups = origin_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let geometric_anti_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(vec2<f32>(self.e4), 0.0, 0.0) * vec4<f32>(other_groups.group4_.w, other_groups.group1_.w, 0.0, 0.0) * vec2<f32>(1.0, -1.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(self.e4) * vec4<f32>(other_groups.group3_.xyz.xyz, other_groups.group0_.y) * vec4<f32>(-1.0, -1.0, -1.0, 1.0), 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(self.e4), 0.0) * vec4<f32>(other_groups.group4_.x, other_groups.group4_.y, other_groups.group4_.z, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(self.e4), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(self.e4) * vec4<f32>(other_groups.group2_.xyz.xyz, other_groups.group0_.x) * vec4<f32>(1.0, 1.0, 1.0, -1.0)
    );
    let geometric_anti_product: MultiVector = multiVector_degroup(geometric_anti_product_groups);
    return multiVector_geometricAntiProduct_origin(geometric_anti_product, origin_antiReverse(self));
}
fn origin_antiSandwich_origin(self_: Origin, other: Origin) -> Origin {
    let self_groups = origin_grouped(self_);
    let other_groups = origin_grouped(other);
    let geometric_anti_product_groups: AntiScalarGroups = AntiScalarGroups(
        /* e1234 */ vec4<f32>(other.e4 * self.e4 * -1.0, 0.0, 0.0, 0.0)
    );
    let geometric_anti_product: AntiScalar = antiScalar_degroup(geometric_anti_product_groups);
    return antiScalar_geometricAntiProduct_origin(geometric_anti_product, origin_antiReverse(self));
}
fn origin_antiSandwich_plane(self_: Origin, other: Plane) -> Flector {
    let self_groups = origin_grouped(self_);
    let other_groups = plane_grouped(other);
    let geometric_anti_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other_groups.group0_.x * self.e4 * -1.0, other_groups.group0_.y * self.e4 * -1.0, other_groups.group0_.z * self.e4 * -1.0, 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group0_.w * self.e4)
    );
    let geometric_anti_product: Motor = motor_degroup(geometric_anti_product_groups);
    return motor_geometricAntiProduct_origin(geometric_anti_product, origin_antiReverse(self));
}
fn origin_antiSandwich_point(self_: Origin, other: Point) -> Flector {
    let self_groups = origin_grouped(self_);
    let other_groups = point_grouped(other);
    let geometric_anti_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group0_.w * self.e4 * -1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(other_groups.group0_.x * self.e4, other_groups.group0_.y * self.e4, other_groups.group0_.z * self.e4, 0.0)
    );
    let geometric_anti_product: Motor = motor_degroup(geometric_anti_product_groups);
    return motor_geometricAntiProduct_origin(geometric_anti_product, origin_antiReverse(self));
}
fn origin_antiSandwich_scalar(self_: Origin, other: Scalar) -> Scalar {
    let self_groups = origin_grouped(self_);
    let other_groups = scalar_grouped(other);
    let geometric_anti_product_groups: HorizonGroups = HorizonGroups(
        /* e321 */ vec4<f32>(self.e4 * other.scalar * -1.0, 0.0, 0.0, 0.0)
    );
    let geometric_anti_product: Horizon = horizon_degroup(geometric_anti_product_groups);
    return horizon_geometricAntiProduct_origin(geometric_anti_product, origin_antiReverse(self));
}
fn plane_antiSandwich_antiScalar(self_: Plane, other: AntiScalar) -> Motor {
    let self_groups = plane_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    let geometric_anti_product_groups: PlaneGroups = PlaneGroups(
        /* e423, e431, e412, e321 */ vec4<f32>(other.e1234) * self_.group0_
    );
    let geometric_anti_product: Plane = plane_degroup(geometric_anti_product_groups);
    return plane_geometricAntiProduct_plane(geometric_anti_product, plane_antiReverse(self));
}
fn plane_antiSandwich_dualNum(self_: Plane, other: DualNum) -> Motor {
    let self_groups = plane_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let geometric_anti_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group0_.x * self_.group0_.x * -1.0, other_groups.group0_.x * self_.group0_.y * -1.0, other_groups.group0_.x * self_.group0_.z * -1.0, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group0_.y) * self_.group0_
    );
    let geometric_anti_product: Flector = flector_degroup(geometric_anti_product_groups);
    return flector_geometricAntiProduct_plane(geometric_anti_product, plane_antiReverse(self));
}
fn plane_antiSandwich_flector(self_: Plane, other: Flector) -> Flector {
    let self_groups = plane_grouped(self_);
    let other_groups = flector_grouped(other);
    let geometric_anti_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(-(other_groups.group0_.w * self_.group0_.x) - (other_groups.group1_.z * self_.group0_.y), -(other_groups.group0_.w * self_.group0_.y) - (other_groups.group1_.x * self_.group0_.z), -(other_groups.group0_.w * self_.group0_.z) - (other_groups.group1_.y * self_.group0_.x), (other_groups.group1_.y * self_.group0_.y) + (other_groups.group1_.z * self_.group0_.z)) + (other_groups.group1_.yzxx * self_.group0_.zxyx), 
        /* e23, e31, e12, scalar */ vec4<f32>((other_groups.group0_.z * self_.group0_.y) + (other_groups.group1_.w * self_.group0_.x), (other_groups.group0_.x * self_.group0_.z) + (other_groups.group1_.w * self_.group0_.y), (other_groups.group0_.y * self_.group0_.x) + (other_groups.group1_.w * self_.group0_.z), -(other_groups.group0_.z * self_.group0_.z) - (other_groups.group0_.w * self_.group0_.w)) - (vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, other_groups.group0_.y) * self_.group0_.wwwy) - (other_groups.group0_.yzxx * self_.group0_.zxyx)
    );
    let geometric_anti_product: Motor = motor_degroup(geometric_anti_product_groups);
    return motor_geometricAntiProduct_plane(geometric_anti_product, plane_antiReverse(self));
}
fn plane_antiSandwich_horizon(self_: Plane, other: Horizon) -> Flector {
    let self_groups = plane_grouped(self_);
    let other_groups = horizon_grouped(other);
    let geometric_anti_product_groups: LineGroups = LineGroups(
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(other.e321), 0.0) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0)
    );
    let geometric_anti_product: Line = line_degroup(geometric_anti_product_groups);
    return line_geometricAntiProduct_plane(geometric_anti_product, plane_antiReverse(self));
}
fn plane_antiSandwich_line(self_: Plane, other: Line) -> Motor {
    let self_groups = plane_grouped(self_);
    let other_groups = line_grouped(other);
    let geometric_anti_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group0_.x * self_.group0_.w) + (other_groups.group1_.y * self_.group0_.z), (other_groups.group0_.y * self_.group0_.w) + (other_groups.group1_.z * self_.group0_.x), (other_groups.group0_.z * self_.group0_.w) + (other_groups.group1_.x * self_.group0_.y), -(other_groups.group0_.y * self_.group0_.y) - (other_groups.group0_.z * self_.group0_.z)) - (vec4<f32>(other_groups.group1_.zxy.xyz, other_groups.group0_.x) * self_.group0_.yzxx), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.y * self_.group0_.z) * -1.0, (other_groups.group0_.z * self_.group0_.x) * -1.0, (other_groups.group0_.x * self_.group0_.y) * -1.0, (other_groups.group1_.y * self_.group0_.y) + (other_groups.group1_.z * self_.group0_.z)) + (vec4<f32>(other_groups.group0_.zxy.xyz, other_groups.group1_.x) * self_.group0_.yzxx)
    );
    let geometric_anti_product: Flector = flector_degroup(geometric_anti_product_groups);
    return flector_geometricAntiProduct_plane(geometric_anti_product, plane_antiReverse(self));
}
fn plane_antiSandwich_motor(self_: Plane, other: Motor) -> Motor {
    let self_groups = plane_grouped(self_);
    let other_groups = motor_grouped(other);
    let geometric_anti_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group0_.x * self_.group0_.w) + (other_groups.group1_.y * self_.group0_.z), (other_groups.group0_.y * self_.group0_.w) + (other_groups.group1_.z * self_.group0_.x), (other_groups.group0_.z * self_.group0_.w) + (other_groups.group1_.x * self_.group0_.y), (other_groups.group0_.z * self_.group0_.z) * -1.0) - (vec4<f32>(other_groups.group1_.z, other_groups.group1_.x, other_groups.group1_.y, other_groups.group0_.x) * self_.group0_.yzxx) - (vec4<f32>(other_groups.group1_.w, other_groups.group1_.w, other_groups.group1_.w, other_groups.group0_.y) * self_.group0_.xyzy), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group0_.y * self_.group0_.z) * -1.0, (other_groups.group0_.z * self_.group0_.x) * -1.0, (other_groups.group0_.x * self_.group0_.y) * -1.0, (other_groups.group1_.y * self_.group0_.y) + (other_groups.group1_.z * self_.group0_.z)) + (vec4<f32>(other_groups.group0_.w, other_groups.group0_.w, other_groups.group0_.w, other_groups.group1_.x) * self_.group0_.xyzx) + (other_groups.group0_.zxyw * self_.group0_.yzxw)
    );
    let geometric_anti_product: Flector = flector_degroup(geometric_anti_product_groups);
    return flector_geometricAntiProduct_plane(geometric_anti_product, plane_antiReverse(self));
}
fn plane_antiSandwich_multiVector(self_: Plane, other: MultiVector) -> MultiVector {
    let self_groups = plane_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let geometric_anti_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(-(other_groups.group1_.x * self_.group0_.x) - (other_groups.group1_.y * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.z) - (other_groups.group1_.w * self_.group0_.w), (other_groups.group4_.x * self_.group0_.x) + (other_groups.group4_.y * self_.group0_.y) + (other_groups.group4_.z * self_.group0_.z), 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group2_.x * self_.group0_.w) + (other_groups.group3_.y * self_.group0_.z), (other_groups.group2_.y * self_.group0_.w) + (other_groups.group3_.z * self_.group0_.x), (other_groups.group2_.z * self_.group0_.w) + (other_groups.group3_.x * self_.group0_.y), (other_groups.group2_.z * self_.group0_.z) * -1.0) - (vec4<f32>(other_groups.group0_.xx.xy, other_groups.group0_.x, other_groups.group2_.x) * self_.group0_.xyzx) - (vec4<f32>(other_groups.group3_.zxy.xyz, other_groups.group2_.y) * self_.group0_.yzxy), 
        /* e41, e42, e43 */ vec4<f32>((other_groups.group4_.y * self_.group0_.z) - (other_groups.group4_.z * self_.group0_.y), (other_groups.group4_.z * self_.group0_.x) - (other_groups.group4_.x * self_.group0_.z), (other_groups.group4_.x * self_.group0_.y) - (other_groups.group4_.y * self_.group0_.x), 0.0) - (vec4<f32>(vec3<f32>(other_groups.group1_.w), 0.0) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0)), 
        /* e23, e31, e12 */ vec4<f32>((other_groups.group1_.z * self_.group0_.y) - (other_groups.group1_.y * self_.group0_.z), (other_groups.group1_.x * self_.group0_.z) - (other_groups.group1_.z * self_.group0_.x), (other_groups.group1_.y * self_.group0_.x) - (other_groups.group1_.x * self_.group0_.y), 0.0) + (vec4<f32>(vec3<f32>(other_groups.group4_.w), 0.0) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0)) - (vec4<f32>(vec3<f32>(self_.group0_.w), 0.0) * vec4<f32>(other_groups.group4_.x, other_groups.group4_.y, other_groups.group4_.z, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>((other_groups.group2_.y * self_.group0_.z) * -1.0, (other_groups.group2_.z * self_.group0_.x) * -1.0, (other_groups.group2_.x * self_.group0_.y) * -1.0, (other_groups.group3_.y * self_.group0_.y) + (other_groups.group3_.z * self_.group0_.z)) + (vec4<f32>(other_groups.group0_.y) * self_.group0_) + (vec4<f32>(other_groups.group2_.zxy.xyz, other_groups.group3_.x) * self_.group0_.yzxx)
    );
    let geometric_anti_product: MultiVector = multiVector_degroup(geometric_anti_product_groups);
    return multiVector_geometricAntiProduct_plane(geometric_anti_product, plane_antiReverse(self));
}
fn plane_antiSandwich_origin(self_: Plane, other: Origin) -> Flector {
    let self_groups = plane_grouped(self_);
    let other_groups = origin_grouped(other);
    let geometric_anti_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.group0_.x * other.e4 * -1.0, self_.group0_.y * other.e4 * -1.0, self_.group0_.z * other.e4 * -1.0, 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(0.0, 0.0, 0.0, self_.group0_.w * other.e4 * -1.0)
    );
    let geometric_anti_product: Motor = motor_degroup(geometric_anti_product_groups);
    return motor_geometricAntiProduct_plane(geometric_anti_product, plane_antiReverse(self));
}
fn plane_antiSandwich_plane(self_: Plane, other: Plane) -> Flector {
    let self_groups = plane_grouped(self_);
    let other_groups = plane_grouped(other);
    let geometric_anti_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>((other_groups.group0_.z * self_.group0_.y) * -1.0, (other_groups.group0_.x * self_.group0_.z) * -1.0, (other_groups.group0_.y * self_.group0_.x) * -1.0, (other_groups.group0_.y * self_.group0_.y) + (other_groups.group0_.z * self_.group0_.z)) + (other_groups.group0_.yzxx * self_.group0_.zxyx), 
        /* e23, e31, e12, scalar */ vec4<f32>((other_groups.group0_.w * self_.group0_.x) - (other_groups.group0_.x * self_.group0_.w), (other_groups.group0_.w * self_.group0_.y) - (other_groups.group0_.y * self_.group0_.w), (other_groups.group0_.w * self_.group0_.z) - (other_groups.group0_.z * self_.group0_.w), 0.0)
    );
    let geometric_anti_product: Motor = motor_degroup(geometric_anti_product_groups);
    return motor_geometricAntiProduct_plane(geometric_anti_product, plane_antiReverse(self));
}
fn plane_antiSandwich_point(self_: Plane, other: Point) -> Flector {
    let self_groups = plane_grouped(self_);
    let other_groups = point_grouped(other);
    let geometric_anti_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.group0_.x * other_groups.group0_.w * -1.0, self_.group0_.y * other_groups.group0_.w * -1.0, self_.group0_.z * other_groups.group0_.w * -1.0, 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.group0_.y * other_groups.group0_.z, self_.group0_.z * other_groups.group0_.x, self_.group0_.x * other_groups.group0_.y, -(self_.group0_.y * other_groups.group0_.y) - (self_.group0_.z * other_groups.group0_.z) - (self_.group0_.w * other_groups.group0_.w)) - (self_.group0_.zxyx * other_groups.group0_.yzxx)
    );
    let geometric_anti_product: Motor = motor_degroup(geometric_anti_product_groups);
    return motor_geometricAntiProduct_plane(geometric_anti_product, plane_antiReverse(self));
}
fn plane_antiSandwich_scalar(self_: Plane, other: Scalar) -> Motor {
    let self_groups = plane_grouped(self_);
    let other_groups = scalar_grouped(other);
    let geometric_anti_product_groups: PointGroups = PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(self_.group0_.x * other.scalar * -1.0, self_.group0_.y * other.scalar * -1.0, self_.group0_.z * other.scalar * -1.0, 0.0)
    );
    let geometric_anti_product: Point = point_degroup(geometric_anti_product_groups);
    return point_geometricAntiProduct_plane(geometric_anti_product, plane_antiReverse(self));
}
fn point_antiSandwich_antiScalar(self_: Point, other: AntiScalar) -> Motor {
    let self_groups = point_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    let geometric_anti_product_groups: PointGroups = PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other.e1234) * self_.group0_
    );
    let geometric_anti_product: Point = point_degroup(geometric_anti_product_groups);
    return point_geometricAntiProduct_point(geometric_anti_product, point_antiReverse(self));
}
fn point_antiSandwich_dualNum(self_: Point, other: DualNum) -> Motor {
    let self_groups = point_grouped(self_);
    let other_groups = dualNum_grouped(other);
    let geometric_anti_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group0_.y) * self_.group0_, 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group0_.x * self_.group0_.w * -1.0)
    );
    let geometric_anti_product: Flector = flector_degroup(geometric_anti_product_groups);
    return flector_geometricAntiProduct_point(geometric_anti_product, point_antiReverse(self));
}
fn point_antiSandwich_flector(self_: Point, other: Flector) -> Flector {
    let self_groups = point_grouped(self_);
    let other_groups = flector_grouped(other);
    let geometric_anti_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(self_.group0_.w) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, other_groups.group0_.w) * vec4<f32>(-1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(-(other_groups.group0_.w * self_.group0_.x) - (other_groups.group1_.z * self_.group0_.y), -(other_groups.group0_.w * self_.group0_.y) - (other_groups.group1_.x * self_.group0_.z), -(other_groups.group0_.w * self_.group0_.z) - (other_groups.group1_.y * self_.group0_.x), (other_groups.group1_.z * self_.group0_.z) + (other_groups.group1_.w * self_.group0_.w)) + (vec4<f32>(other_groups.group0_.x, other_groups.group0_.y, other_groups.group0_.z, other_groups.group1_.x) * self_.group0_.wwwx) + (other_groups.group1_.yzxy * self_.group0_.zxyy)
    );
    let geometric_anti_product: Motor = motor_degroup(geometric_anti_product_groups);
    return motor_geometricAntiProduct_point(geometric_anti_product, point_antiReverse(self));
}
fn point_antiSandwich_horizon(self_: Point, other: Horizon) -> Horizon {
    let self_groups = point_grouped(self_);
    let other_groups = horizon_grouped(other);
    let geometric_anti_product_groups: ScalarGroups = ScalarGroups(
        /* scalar */ vec4<f32>(self_.group0_.w * other.e321, 0.0, 0.0, 0.0)
    );
    let geometric_anti_product: Scalar = scalar_degroup(geometric_anti_product_groups);
    return scalar_geometricAntiProduct_point(geometric_anti_product, point_antiReverse(self));
}
fn point_antiSandwich_line(self_: Point, other: Line) -> Motor {
    let self_groups = point_grouped(self_);
    let other_groups = line_grouped(other);
    let geometric_anti_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group0_.z * self_.group0_.y) - (other_groups.group0_.y * self_.group0_.z) - (other_groups.group1_.x * self_.group0_.w), (other_groups.group0_.x * self_.group0_.z) - (other_groups.group0_.z * self_.group0_.x) - (other_groups.group1_.y * self_.group0_.w), (other_groups.group0_.y * self_.group0_.x) - (other_groups.group0_.x * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.w), 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group0_.x * self_.group0_.w, other_groups.group0_.y * self_.group0_.w, other_groups.group0_.z * self_.group0_.w, -(other_groups.group0_.x * self_.group0_.x) - (other_groups.group0_.y * self_.group0_.y) - (other_groups.group0_.z * self_.group0_.z))
    );
    let geometric_anti_product: Flector = flector_degroup(geometric_anti_product_groups);
    return flector_geometricAntiProduct_point(geometric_anti_product, point_antiReverse(self));
}
fn point_antiSandwich_motor(self_: Point, other: Motor) -> Motor {
    let self_groups = point_grouped(self_);
    let other_groups = motor_grouped(other);
    let geometric_anti_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group0_.z * self_.group0_.y) + (other_groups.group0_.w * self_.group0_.x) - (other_groups.group0_.y * self_.group0_.z) - (other_groups.group1_.x * self_.group0_.w), (other_groups.group0_.x * self_.group0_.z) + (other_groups.group0_.w * self_.group0_.y) - (other_groups.group0_.z * self_.group0_.x) - (other_groups.group1_.y * self_.group0_.w), (other_groups.group0_.y * self_.group0_.x) + (other_groups.group0_.w * self_.group0_.z) - (other_groups.group0_.x * self_.group0_.y) - (other_groups.group1_.z * self_.group0_.w), other_groups.group0_.w * self_.group0_.w), 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group0_.x * self_.group0_.w, other_groups.group0_.y * self_.group0_.w, other_groups.group0_.z * self_.group0_.w, -(other_groups.group0_.x * self_.group0_.x) - (other_groups.group0_.y * self_.group0_.y) - (other_groups.group0_.z * self_.group0_.z) - (other_groups.group1_.w * self_.group0_.w))
    );
    let geometric_anti_product: Flector = flector_degroup(geometric_anti_product_groups);
    return flector_geometricAntiProduct_point(geometric_anti_product, point_antiReverse(self));
}
fn point_antiSandwich_multiVector(self_: Point, other: MultiVector) -> MultiVector {
    let self_groups = point_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let geometric_anti_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>((other_groups.group4_.x * self_.group0_.x) + (other_groups.group4_.y * self_.group0_.y) + (other_groups.group4_.z * self_.group0_.z) + (other_groups.group4_.w * self_.group0_.w), other_groups.group1_.w * self_.group0_.w * -1.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>((other_groups.group0_.y * self_.group0_.x) + (other_groups.group2_.z * self_.group0_.y) - (other_groups.group2_.y * self_.group0_.z) - (other_groups.group3_.x * self_.group0_.w), (other_groups.group0_.y * self_.group0_.y) + (other_groups.group2_.x * self_.group0_.z) - (other_groups.group2_.z * self_.group0_.x) - (other_groups.group3_.y * self_.group0_.w), (other_groups.group0_.y * self_.group0_.z) + (other_groups.group2_.y * self_.group0_.x) - (other_groups.group2_.x * self_.group0_.y) - (other_groups.group3_.z * self_.group0_.w), other_groups.group0_.y * self_.group0_.w), 
        /* e41, e42, e43 */ vec4<f32>(vec3<f32>(self_.group0_.w), 0.0) * vec4<f32>(other_groups.group4_.x, other_groups.group4_.y, other_groups.group4_.z, 0.0) * vec3<f32>(-1.0), 
        /* e23, e31, e12 */ vec4<f32>((other_groups.group4_.y * self_.group0_.z) - (other_groups.group4_.z * self_.group0_.y), (other_groups.group4_.z * self_.group0_.x) - (other_groups.group4_.x * self_.group0_.z), (other_groups.group4_.x * self_.group0_.y) - (other_groups.group4_.y * self_.group0_.x), 0.0) + (vec4<f32>(vec3<f32>(self_.group0_.w), 0.0) * vec4<f32>(other_groups.group1_.x, other_groups.group1_.y, other_groups.group1_.z, 0.0)) - (vec4<f32>(vec3<f32>(other_groups.group1_.w), 0.0) * vec4<f32>(self_.group0_.x, self_.group0_.y, self_.group0_.z, 0.0)), 
        /* e423, e431, e412, e321 */ vec4<f32>(other_groups.group2_.x * self_.group0_.w, other_groups.group2_.y * self_.group0_.w, other_groups.group2_.z * self_.group0_.w, -(other_groups.group0_.x * self_.group0_.w) - (other_groups.group2_.x * self_.group0_.x) - (other_groups.group2_.y * self_.group0_.y) - (other_groups.group2_.z * self_.group0_.z))
    );
    let geometric_anti_product: MultiVector = multiVector_degroup(geometric_anti_product_groups);
    return multiVector_geometricAntiProduct_point(geometric_anti_product, point_antiReverse(self));
}
fn point_antiSandwich_origin(self_: Point, other: Origin) -> Flector {
    let self_groups = point_grouped(self_);
    let other_groups = origin_grouped(other);
    let geometric_anti_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, self_.group0_.w * other.e4 * -1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(self_.group0_.x * other.e4 * -1.0, self_.group0_.y * other.e4 * -1.0, self_.group0_.z * other.e4 * -1.0, 0.0)
    );
    let geometric_anti_product: Motor = motor_degroup(geometric_anti_product_groups);
    return motor_geometricAntiProduct_point(geometric_anti_product, point_antiReverse(self));
}
fn point_antiSandwich_plane(self_: Point, other: Plane) -> Flector {
    let self_groups = point_grouped(self_);
    let other_groups = plane_grouped(other);
    let geometric_anti_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(other_groups.group0_.x * self_.group0_.w * -1.0, other_groups.group0_.y * self_.group0_.w * -1.0, other_groups.group0_.z * self_.group0_.w * -1.0, 0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>((other_groups.group0_.z * self_.group0_.y) * -1.0, (other_groups.group0_.x * self_.group0_.z) * -1.0, (other_groups.group0_.y * self_.group0_.x) * -1.0, (other_groups.group0_.y * self_.group0_.y) + (other_groups.group0_.z * self_.group0_.z) + (other_groups.group0_.w * self_.group0_.w)) + (other_groups.group0_.yzxx * self_.group0_.zxyx)
    );
    let geometric_anti_product: Motor = motor_degroup(geometric_anti_product_groups);
    return motor_geometricAntiProduct_point(geometric_anti_product, point_antiReverse(self));
}
fn point_antiSandwich_point(self_: Point, other: Point) -> Flector {
    let self_groups = point_grouped(self_);
    let other_groups = point_grouped(other);
    let geometric_anti_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group0_.w * self_.group0_.w * -1.0), 
        /* e23, e31, e12, scalar */ vec4<f32>((other_groups.group0_.x * self_.group0_.w) - (other_groups.group0_.w * self_.group0_.x), (other_groups.group0_.y * self_.group0_.w) - (other_groups.group0_.w * self_.group0_.y), (other_groups.group0_.z * self_.group0_.w) - (other_groups.group0_.w * self_.group0_.z), 0.0)
    );
    let geometric_anti_product: Motor = motor_degroup(geometric_anti_product_groups);
    return motor_geometricAntiProduct_point(geometric_anti_product, point_antiReverse(self));
}
fn point_antiSandwich_scalar(self_: Point, other: Scalar) -> Scalar {
    let self_groups = point_grouped(self_);
    let other_groups = scalar_grouped(other);
    let geometric_anti_product_groups: HorizonGroups = HorizonGroups(
        /* e321 */ vec4<f32>(self_.group0_.w * other.scalar * -1.0, 0.0, 0.0, 0.0)
    );
    let geometric_anti_product: Horizon = horizon_degroup(geometric_anti_product_groups);
    return horizon_geometricAntiProduct_point(geometric_anti_product, point_antiReverse(self));
}
fn scalar_antiSandwich_flector(self_: Scalar, other: Flector) -> Flector {
    let self_groups = scalar_grouped(self_);
    let other_groups = flector_grouped(other);
    let geometric_anti_product_groups: FlectorGroups = FlectorGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group1_.x * self.scalar, other_groups.group1_.y * self.scalar, other_groups.group1_.z * self.scalar, 0.0), 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group0_.w * self.scalar)
    );
    let geometric_anti_product: Flector = flector_degroup(geometric_anti_product_groups);
    return flector_geometricAntiProduct_scalar(geometric_anti_product, scalar_antiReverse(self));
}
fn scalar_antiSandwich_line(self_: Scalar, other: Line) -> Line {
    let self_groups = scalar_grouped(self_);
    let other_groups = line_grouped(other);
    let geometric_anti_product_groups: LineGroups = LineGroups(
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(self.scalar), 0.0) * other_groups.group0_
    );
    let geometric_anti_product: Line = line_degroup(geometric_anti_product_groups);
    return line_geometricAntiProduct_scalar(geometric_anti_product, scalar_antiReverse(self));
}
fn scalar_antiSandwich_motor(self_: Scalar, other: Motor) -> Motor {
    let self_groups = scalar_grouped(self_);
    let other_groups = motor_grouped(other);
    let geometric_anti_product_groups: MotorGroups = MotorGroups(
        /* e41, e42, e43, e1234 */ vec4<f32>(0.0), 
        /* e23, e31, e12, scalar */ vec4<f32>(self.scalar) * other_groups.group0_
    );
    let geometric_anti_product: Motor = motor_degroup(geometric_anti_product_groups);
    return motor_geometricAntiProduct_scalar(geometric_anti_product, scalar_antiReverse(self));
}
fn scalar_antiSandwich_multiVector(self_: Scalar, other: MultiVector) -> MultiVector {
    let self_groups = scalar_grouped(self_);
    let other_groups = multiVector_grouped(other);
    let geometric_anti_product_groups: MultiVectorGroups = MultiVectorGroups(
        /* scalar, e1234 */ vec4<f32>(other_groups.group0_.y * self.scalar, 0.0, 0.0, 0.0), 
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group4_.x * self.scalar, other_groups.group4_.y * self.scalar, other_groups.group4_.z * self.scalar, 0.0), 
        /* e41, e42, e43 */ vec4<f32>(0.0), 
        /* e23, e31, e12 */ vec4<f32>(vec3<f32>(self.scalar), 0.0) * other_groups.group2_, 
        /* e423, e431, e412, e321 */ vec4<f32>(0.0, 0.0, 0.0, other_groups.group1_.w * self.scalar)
    );
    let geometric_anti_product: MultiVector = multiVector_degroup(geometric_anti_product_groups);
    return multiVector_geometricAntiProduct_scalar(geometric_anti_product, scalar_antiReverse(self));
}
fn scalar_antiSandwich_plane(self_: Scalar, other: Plane) -> Horizon {
    let self_groups = scalar_grouped(self_);
    let other_groups = plane_grouped(other);
    let geometric_anti_product_groups: PointGroups = PointGroups(
        /* e1, e2, e3, e4 */ vec4<f32>(other_groups.group0_.x * self.scalar, other_groups.group0_.y * self.scalar, other_groups.group0_.z * self.scalar, 0.0)
    );
    let geometric_anti_product: Point = point_degroup(geometric_anti_product_groups);
    return point_geometricAntiProduct_scalar(geometric_anti_product, scalar_antiReverse(self));
}
fn antiScalar_bitXor_dualNum(self_: AntiScalar, other: DualNum) -> AntiScalar {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return antiScalar_wedge_dualNum(self, other);
}
fn antiScalar_bitXor_motor(self_: AntiScalar, other: Motor) -> AntiScalar {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = motor_grouped(other);
    return antiScalar_wedge_motor(self, other);
}
fn antiScalar_bitXor_multiVector(self_: AntiScalar, other: MultiVector) -> AntiScalar {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return antiScalar_wedge_multiVector(self, other);
}
fn antiScalar_bitXor_scalar(self_: AntiScalar, other: Scalar) -> AntiScalar {
    let self_groups = antiScalar_grouped(self_);
    let other_groups = scalar_grouped(other);
    return antiScalar_wedge_scalar(self, other);
}
fn dualNum_bitXor_antiScalar(self_: DualNum, other: AntiScalar) -> AntiScalar {
    let self_groups = dualNum_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return dualNum_wedge_antiScalar(self, other);
}
fn dualNum_bitXor_dualNum(self_: DualNum, other: DualNum) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return dualNum_wedge_dualNum(self, other);
}
fn dualNum_bitXor_flector(self_: DualNum, other: Flector) -> Flector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = flector_grouped(other);
    return dualNum_wedge_flector(self, other);
}
fn dualNum_bitXor_horizon(self_: DualNum, other: Horizon) -> Horizon {
    let self_groups = dualNum_grouped(self_);
    let other_groups = horizon_grouped(other);
    return dualNum_wedge_horizon(self, other);
}
fn dualNum_bitXor_line(self_: DualNum, other: Line) -> Line {
    let self_groups = dualNum_grouped(self_);
    let other_groups = line_grouped(other);
    return dualNum_wedge_line(self, other);
}
fn dualNum_bitXor_motor(self_: DualNum, other: Motor) -> Motor {
    let self_groups = dualNum_grouped(self_);
    let other_groups = motor_grouped(other);
    return dualNum_wedge_motor(self, other);
}
fn dualNum_bitXor_multiVector(self_: DualNum, other: MultiVector) -> MultiVector {
    let self_groups = dualNum_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return dualNum_wedge_multiVector(self, other);
}
fn dualNum_bitXor_origin(self_: DualNum, other: Origin) -> Origin {
    let self_groups = dualNum_grouped(self_);
    let other_groups = origin_grouped(other);
    return dualNum_wedge_origin(self, other);
}
fn dualNum_bitXor_plane(self_: DualNum, other: Plane) -> Plane {
    let self_groups = dualNum_grouped(self_);
    let other_groups = plane_grouped(other);
    return dualNum_wedge_plane(self, other);
}
fn dualNum_bitXor_point(self_: DualNum, other: Point) -> Point {
    let self_groups = dualNum_grouped(self_);
    let other_groups = point_grouped(other);
    return dualNum_wedge_point(self, other);
}
fn dualNum_bitXor_scalar(self_: DualNum, other: Scalar) -> DualNum {
    let self_groups = dualNum_grouped(self_);
    let other_groups = scalar_grouped(other);
    return dualNum_wedge_scalar(self, other);
}
fn flector_bitXor_dualNum(self_: Flector, other: DualNum) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return flector_wedge_dualNum(self, other);
}
fn flector_bitXor_flector(self_: Flector, other: Flector) -> Motor {
    let self_groups = flector_grouped(self_);
    let other_groups = flector_grouped(other);
    return flector_wedge_flector(self, other);
}
fn flector_bitXor_horizon(self_: Flector, other: Horizon) -> AntiScalar {
    let self_groups = flector_grouped(self_);
    let other_groups = horizon_grouped(other);
    return flector_wedge_horizon(self, other);
}
fn flector_bitXor_line(self_: Flector, other: Line) -> Plane {
    let self_groups = flector_grouped(self_);
    let other_groups = line_grouped(other);
    return flector_wedge_line(self, other);
}
fn flector_bitXor_motor(self_: Flector, other: Motor) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = motor_grouped(other);
    return flector_wedge_motor(self, other);
}
fn flector_bitXor_multiVector(self_: Flector, other: MultiVector) -> MultiVector {
    let self_groups = flector_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return flector_wedge_multiVector(self, other);
}
fn flector_bitXor_origin(self_: Flector, other: Origin) -> Motor {
    let self_groups = flector_grouped(self_);
    let other_groups = origin_grouped(other);
    return flector_wedge_origin(self, other);
}
fn flector_bitXor_plane(self_: Flector, other: Plane) -> AntiScalar {
    let self_groups = flector_grouped(self_);
    let other_groups = plane_grouped(other);
    return flector_wedge_plane(self, other);
}
fn flector_bitXor_point(self_: Flector, other: Point) -> Motor {
    let self_groups = flector_grouped(self_);
    let other_groups = point_grouped(other);
    return flector_wedge_point(self, other);
}
fn flector_bitXor_scalar(self_: Flector, other: Scalar) -> Flector {
    let self_groups = flector_grouped(self_);
    let other_groups = scalar_grouped(other);
    return flector_wedge_scalar(self, other);
}
fn horizon_bitXor_dualNum(self_: Horizon, other: DualNum) -> Horizon {
    let self_groups = horizon_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return horizon_wedge_dualNum(self, other);
}
fn horizon_bitXor_flector(self_: Horizon, other: Flector) -> AntiScalar {
    let self_groups = horizon_grouped(self_);
    let other_groups = flector_grouped(other);
    return horizon_wedge_flector(self, other);
}
fn horizon_bitXor_motor(self_: Horizon, other: Motor) -> Horizon {
    let self_groups = horizon_grouped(self_);
    let other_groups = motor_grouped(other);
    return horizon_wedge_motor(self, other);
}
fn horizon_bitXor_multiVector(self_: Horizon, other: MultiVector) -> MultiVector {
    let self_groups = horizon_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return horizon_wedge_multiVector(self, other);
}
fn horizon_bitXor_origin(self_: Horizon, other: Origin) -> AntiScalar {
    let self_groups = horizon_grouped(self_);
    let other_groups = origin_grouped(other);
    return horizon_wedge_origin(self, other);
}
fn horizon_bitXor_point(self_: Horizon, other: Point) -> AntiScalar {
    let self_groups = horizon_grouped(self_);
    let other_groups = point_grouped(other);
    return horizon_wedge_point(self, other);
}
fn horizon_bitXor_scalar(self_: Horizon, other: Scalar) -> Horizon {
    let self_groups = horizon_grouped(self_);
    let other_groups = scalar_grouped(other);
    return horizon_wedge_scalar(self, other);
}
fn line_bitXor_dualNum(self_: Line, other: DualNum) -> Line {
    let self_groups = line_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return line_wedge_dualNum(self, other);
}
fn line_bitXor_flector(self_: Line, other: Flector) -> Plane {
    let self_groups = line_grouped(self_);
    let other_groups = flector_grouped(other);
    return line_wedge_flector(self, other);
}
fn line_bitXor_line(self_: Line, other: Line) -> AntiScalar {
    let self_groups = line_grouped(self_);
    let other_groups = line_grouped(other);
    return line_wedge_line(self, other);
}
fn line_bitXor_motor(self_: Line, other: Motor) -> Motor {
    let self_groups = line_grouped(self_);
    let other_groups = motor_grouped(other);
    return line_wedge_motor(self, other);
}
fn line_bitXor_multiVector(self_: Line, other: MultiVector) -> MultiVector {
    let self_groups = line_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return line_wedge_multiVector(self, other);
}
fn line_bitXor_origin(self_: Line, other: Origin) -> Plane {
    let self_groups = line_grouped(self_);
    let other_groups = origin_grouped(other);
    return line_wedge_origin(self, other);
}
fn line_bitXor_point(self_: Line, other: Point) -> Plane {
    let self_groups = line_grouped(self_);
    let other_groups = point_grouped(other);
    return line_wedge_point(self, other);
}
fn line_bitXor_scalar(self_: Line, other: Scalar) -> Line {
    let self_groups = line_grouped(self_);
    let other_groups = scalar_grouped(other);
    return line_wedge_scalar(self, other);
}
fn motor_bitXor_antiScalar(self_: Motor, other: AntiScalar) -> AntiScalar {
    let self_groups = motor_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return motor_wedge_antiScalar(self, other);
}
fn motor_bitXor_dualNum(self_: Motor, other: DualNum) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return motor_wedge_dualNum(self, other);
}
fn motor_bitXor_flector(self_: Motor, other: Flector) -> Flector {
    let self_groups = motor_grouped(self_);
    let other_groups = flector_grouped(other);
    return motor_wedge_flector(self, other);
}
fn motor_bitXor_horizon(self_: Motor, other: Horizon) -> Horizon {
    let self_groups = motor_grouped(self_);
    let other_groups = horizon_grouped(other);
    return motor_wedge_horizon(self, other);
}
fn motor_bitXor_line(self_: Motor, other: Line) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = line_grouped(other);
    return motor_wedge_line(self, other);
}
fn motor_bitXor_motor(self_: Motor, other: Motor) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = motor_grouped(other);
    return motor_wedge_motor(self, other);
}
fn motor_bitXor_multiVector(self_: Motor, other: MultiVector) -> MultiVector {
    let self_groups = motor_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return motor_wedge_multiVector(self, other);
}
fn motor_bitXor_origin(self_: Motor, other: Origin) -> Flector {
    let self_groups = motor_grouped(self_);
    let other_groups = origin_grouped(other);
    return motor_wedge_origin(self, other);
}
fn motor_bitXor_plane(self_: Motor, other: Plane) -> Plane {
    let self_groups = motor_grouped(self_);
    let other_groups = plane_grouped(other);
    return motor_wedge_plane(self, other);
}
fn motor_bitXor_point(self_: Motor, other: Point) -> Flector {
    let self_groups = motor_grouped(self_);
    let other_groups = point_grouped(other);
    return motor_wedge_point(self, other);
}
fn motor_bitXor_scalar(self_: Motor, other: Scalar) -> Motor {
    let self_groups = motor_grouped(self_);
    let other_groups = scalar_grouped(other);
    return motor_wedge_scalar(self, other);
}
fn multiVector_bitXor_antiScalar(self_: MultiVector, other: AntiScalar) -> AntiScalar {
    let self_groups = multiVector_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return multiVector_wedge_antiScalar(self, other);
}
fn multiVector_bitXor_dualNum(self_: MultiVector, other: DualNum) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return multiVector_wedge_dualNum(self, other);
}
fn multiVector_bitXor_flector(self_: MultiVector, other: Flector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = flector_grouped(other);
    return multiVector_wedge_flector(self, other);
}
fn multiVector_bitXor_horizon(self_: MultiVector, other: Horizon) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = horizon_grouped(other);
    return multiVector_wedge_horizon(self, other);
}
fn multiVector_bitXor_line(self_: MultiVector, other: Line) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = line_grouped(other);
    return multiVector_wedge_line(self, other);
}
fn multiVector_bitXor_motor(self_: MultiVector, other: Motor) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = motor_grouped(other);
    return multiVector_wedge_motor(self, other);
}
fn multiVector_bitXor_multiVector(self_: MultiVector, other: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return multiVector_wedge_multiVector(self, other);
}
fn multiVector_bitXor_origin(self_: MultiVector, other: Origin) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = origin_grouped(other);
    return multiVector_wedge_origin(self, other);
}
fn multiVector_bitXor_plane(self_: MultiVector, other: Plane) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = plane_grouped(other);
    return multiVector_wedge_plane(self, other);
}
fn multiVector_bitXor_point(self_: MultiVector, other: Point) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = point_grouped(other);
    return multiVector_wedge_point(self, other);
}
fn multiVector_bitXor_scalar(self_: MultiVector, other: Scalar) -> MultiVector {
    let self_groups = multiVector_grouped(self_);
    let other_groups = scalar_grouped(other);
    return multiVector_wedge_scalar(self, other);
}
fn origin_bitXor_dualNum(self_: Origin, other: DualNum) -> Origin {
    let self_groups = origin_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return origin_wedge_dualNum(self, other);
}
fn origin_bitXor_flector(self_: Origin, other: Flector) -> Motor {
    let self_groups = origin_grouped(self_);
    let other_groups = flector_grouped(other);
    return origin_wedge_flector(self, other);
}
fn origin_bitXor_horizon(self_: Origin, other: Horizon) -> AntiScalar {
    let self_groups = origin_grouped(self_);
    let other_groups = horizon_grouped(other);
    return origin_wedge_horizon(self, other);
}
fn origin_bitXor_line(self_: Origin, other: Line) -> Plane {
    let self_groups = origin_grouped(self_);
    let other_groups = line_grouped(other);
    return origin_wedge_line(self, other);
}
fn origin_bitXor_motor(self_: Origin, other: Motor) -> Flector {
    let self_groups = origin_grouped(self_);
    let other_groups = motor_grouped(other);
    return origin_wedge_motor(self, other);
}
fn origin_bitXor_multiVector(self_: Origin, other: MultiVector) -> MultiVector {
    let self_groups = origin_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return origin_wedge_multiVector(self, other);
}
fn origin_bitXor_plane(self_: Origin, other: Plane) -> AntiScalar {
    let self_groups = origin_grouped(self_);
    let other_groups = plane_grouped(other);
    return origin_wedge_plane(self, other);
}
fn origin_bitXor_point(self_: Origin, other: Point) -> Line {
    let self_groups = origin_grouped(self_);
    let other_groups = point_grouped(other);
    return origin_wedge_point(self, other);
}
fn origin_bitXor_scalar(self_: Origin, other: Scalar) -> Origin {
    let self_groups = origin_grouped(self_);
    let other_groups = scalar_grouped(other);
    return origin_wedge_scalar(self, other);
}
fn plane_bitXor_dualNum(self_: Plane, other: DualNum) -> Plane {
    let self_groups = plane_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return plane_wedge_dualNum(self, other);
}
fn plane_bitXor_flector(self_: Plane, other: Flector) -> AntiScalar {
    let self_groups = plane_grouped(self_);
    let other_groups = flector_grouped(other);
    return plane_wedge_flector(self, other);
}
fn plane_bitXor_motor(self_: Plane, other: Motor) -> Plane {
    let self_groups = plane_grouped(self_);
    let other_groups = motor_grouped(other);
    return plane_wedge_motor(self, other);
}
fn plane_bitXor_multiVector(self_: Plane, other: MultiVector) -> MultiVector {
    let self_groups = plane_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return plane_wedge_multiVector(self, other);
}
fn plane_bitXor_origin(self_: Plane, other: Origin) -> AntiScalar {
    let self_groups = plane_grouped(self_);
    let other_groups = origin_grouped(other);
    return plane_wedge_origin(self, other);
}
fn plane_bitXor_point(self_: Plane, other: Point) -> AntiScalar {
    let self_groups = plane_grouped(self_);
    let other_groups = point_grouped(other);
    return plane_wedge_point(self, other);
}
fn plane_bitXor_scalar(self_: Plane, other: Scalar) -> Plane {
    let self_groups = plane_grouped(self_);
    let other_groups = scalar_grouped(other);
    return plane_wedge_scalar(self, other);
}
fn point_bitXor_dualNum(self_: Point, other: DualNum) -> Point {
    let self_groups = point_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return point_wedge_dualNum(self, other);
}
fn point_bitXor_flector(self_: Point, other: Flector) -> Motor {
    let self_groups = point_grouped(self_);
    let other_groups = flector_grouped(other);
    return point_wedge_flector(self, other);
}
fn point_bitXor_horizon(self_: Point, other: Horizon) -> AntiScalar {
    let self_groups = point_grouped(self_);
    let other_groups = horizon_grouped(other);
    return point_wedge_horizon(self, other);
}
fn point_bitXor_line(self_: Point, other: Line) -> Plane {
    let self_groups = point_grouped(self_);
    let other_groups = line_grouped(other);
    return point_wedge_line(self, other);
}
fn point_bitXor_motor(self_: Point, other: Motor) -> Flector {
    let self_groups = point_grouped(self_);
    let other_groups = motor_grouped(other);
    return point_wedge_motor(self, other);
}
fn point_bitXor_multiVector(self_: Point, other: MultiVector) -> MultiVector {
    let self_groups = point_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return point_wedge_multiVector(self, other);
}
fn point_bitXor_origin(self_: Point, other: Origin) -> Line {
    let self_groups = point_grouped(self_);
    let other_groups = origin_grouped(other);
    return point_wedge_origin(self, other);
}
fn point_bitXor_plane(self_: Point, other: Plane) -> AntiScalar {
    let self_groups = point_grouped(self_);
    let other_groups = plane_grouped(other);
    return point_wedge_plane(self, other);
}
fn point_bitXor_point(self_: Point, other: Point) -> Line {
    let self_groups = point_grouped(self_);
    let other_groups = point_grouped(other);
    return point_wedge_point(self, other);
}
fn point_bitXor_scalar(self_: Point, other: Scalar) -> Point {
    let self_groups = point_grouped(self_);
    let other_groups = scalar_grouped(other);
    return point_wedge_scalar(self, other);
}
fn scalar_bitXor_antiScalar(self_: Scalar, other: AntiScalar) -> AntiScalar {
    let self_groups = scalar_grouped(self_);
    let other_groups = antiScalar_grouped(other);
    return scalar_wedge_antiScalar(self, other);
}
fn scalar_bitXor_dualNum(self_: Scalar, other: DualNum) -> DualNum {
    let self_groups = scalar_grouped(self_);
    let other_groups = dualNum_grouped(other);
    return scalar_wedge_dualNum(self, other);
}
fn scalar_bitXor_flector(self_: Scalar, other: Flector) -> Flector {
    let self_groups = scalar_grouped(self_);
    let other_groups = flector_grouped(other);
    return scalar_wedge_flector(self, other);
}
fn scalar_bitXor_horizon(self_: Scalar, other: Horizon) -> Horizon {
    let self_groups = scalar_grouped(self_);
    let other_groups = horizon_grouped(other);
    return scalar_wedge_horizon(self, other);
}
fn scalar_bitXor_line(self_: Scalar, other: Line) -> Line {
    let self_groups = scalar_grouped(self_);
    let other_groups = line_grouped(other);
    return scalar_wedge_line(self, other);
}
fn scalar_bitXor_motor(self_: Scalar, other: Motor) -> Motor {
    let self_groups = scalar_grouped(self_);
    let other_groups = motor_grouped(other);
    return scalar_wedge_motor(self, other);
}
fn scalar_bitXor_multiVector(self_: Scalar, other: MultiVector) -> MultiVector {
    let self_groups = scalar_grouped(self_);
    let other_groups = multiVector_grouped(other);
    return scalar_wedge_multiVector(self, other);
}
fn scalar_bitXor_origin(self_: Scalar, other: Origin) -> Origin {
    let self_groups = scalar_grouped(self_);
    let other_groups = origin_grouped(other);
    return scalar_wedge_origin(self, other);
}
fn scalar_bitXor_plane(self_: Scalar, other: Plane) -> Plane {
    let self_groups = scalar_grouped(self_);
    let other_groups = plane_grouped(other);
    return scalar_wedge_plane(self, other);
}
fn scalar_bitXor_point(self_: Scalar, other: Point) -> Point {
    let self_groups = scalar_grouped(self_);
    let other_groups = point_grouped(other);
    return scalar_wedge_point(self, other);
}
fn scalar_bitXor_scalar(self_: Scalar, other: Scalar) -> Scalar {
    let self_groups = scalar_grouped(self_);
    let other_groups = scalar_grouped(other);
    return scalar_wedge_scalar(self, other);
}
fn dualNum_not(self_: DualNum) -> AntiScalar {
    let self_groups = dualNum_grouped(self_);    return dualNum_rightDual(self);
}
fn flector_not(self_: Flector) -> Flector {
    let self_groups = flector_grouped(self_);    return flector_rightDual(self);
}
fn horizon_not(self_: Horizon) -> Origin {
    let self_groups = horizon_grouped(self_);    return horizon_rightDual(self);
}
fn line_not(self_: Line) -> Line {
    let self_groups = line_grouped(self_);    return line_rightDual(self);
}
fn motor_not(self_: Motor) -> Motor {
    let self_groups = motor_grouped(self_);    return motor_rightDual(self);
}
fn multiVector_not(self_: MultiVector) -> MultiVector {
    let self_groups = multiVector_grouped(self_);    return multiVector_rightDual(self);
}
fn plane_not(self_: Plane) -> Origin {
    let self_groups = plane_grouped(self_);    return plane_rightDual(self);
}
fn point_not(self_: Point) -> Plane {
    let self_groups = point_grouped(self_);    return point_rightDual(self);
}
fn scalar_not(self_: Scalar) -> AntiScalar {
    let self_groups = scalar_grouped(self_);    return scalar_rightDual(self);
}

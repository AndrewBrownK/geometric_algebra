#define_import_path cga3d_min

//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

struct Scalar {
    // 1
    float g0_;
};

struct AntiScalar {
    // e12345
    float g0_;
};

struct DualNum {
    // 1, e12345
    vec2 g0_;
};

struct FlatPoint {
    // e15, e25, e35, e45
    vec4 g0_;
};

struct Line {
    // -e145, -e245, -e345
    vec3 g0_;
    // e235, -e135, e125
    vec3 g1_;
};

struct Plane {
    // e2345, -e1345, e1245, -e1235
    vec4 g0_;
};

struct RoundPoint {
    // e1, e2, e3
    vec3 g0_;
    // e4, e5
    vec2 g1_;
};

struct Dipole {
    // -e14, -e24, -e34
    vec3 g0_;
    // e23, -e13, e12
    vec3 g1_;
    // e15, e25, e35, e45
    vec4 g2_;
};

struct Circle {
    // e234, -e134, e124, -e123
    vec4 g0_;
    // -e145, -e245, -e345
    vec3 g1_;
    // e235, -e135, e125
    vec3 g2_;
};

struct Sphere {
    // e2345, -e1345, e1245
    vec3 g0_;
    // e1234, -e1235
    vec2 g1_;
};

struct Motor {
    // -e145, -e245, -e345, e12345
    vec4 g0_;
    // e235, -e135, e125
    vec3 g1_;
};

struct Flector {
    // e15, e25, e35, e45
    vec4 g0_;
    // e2345, -e1345, e1245, -e1235
    vec4 g1_;
};

struct MultiVector {
    // 1, e12345
    vec2 g0_;
    // e1, e2, e3
    vec3 g1_;
    // e4, e5
    vec2 g2_;
    // -e14, -e24, -e34
    vec3 g3_;
    // e23, -e13, e12
    vec3 g4_;
    // e15, e25, e35, e45
    vec4 g5_;
    // e234, -e134, e124, -e123
    vec4 g6_;
    // -e145, -e245, -e345
    vec3 g7_;
    // e235, -e135, e125
    vec3 g8_;
    // e2345, -e1345, e1245
    vec3 g9_;
    // e1234, -e1235
    vec2 g10_;
};

AntiScalar antiScalar_one() {
    return AntiScalar(0.0);
}

Circle circle_one() {
    return Circle(vec4(0.0), vec3(0.0), vec3(0.0));
}

Dipole dipole_one() {
    return Dipole(vec3(0.0), vec3(0.0), vec4(0.0));
}

DualNum dualNum_one() {
    return DualNum(vec2(1.0, 0.0));
}

FlatPoint flatPoint_one() {
    return FlatPoint(vec4(0.0));
}

Flector flector_one() {
    return Flector(vec4(0.0), vec4(0.0));
}

Line line_one() {
    return Line(vec3(0.0), vec3(0.0));
}

Motor motor_one() {
    return Motor(vec4(0.0), vec3(0.0));
}

MultiVector multiVector_one() {
    return MultiVector(vec2(1.0, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

Plane plane_one() {
    return Plane(vec4(0.0));
}

RoundPoint roundPoint_one() {
    return RoundPoint(vec3(0.0), vec2(0.0));
}

Scalar scalar_one() {
    return Scalar(1.0);
}

Sphere sphere_one() {
    return Sphere(vec3(0.0), vec2(0.0));
}

AntiScalar antiScalar_unit() {
    return AntiScalar(1.0);
}

Circle circle_unit() {
    return Circle(vec4(1.0), vec3(1.0), vec3(1.0));
}

Dipole dipole_unit() {
    return Dipole(vec3(1.0), vec3(1.0), vec4(1.0));
}

DualNum dualNum_unit() {
    return DualNum(vec2(1.0));
}

FlatPoint flatPoint_unit() {
    return FlatPoint(vec4(1.0));
}

Flector flector_unit() {
    return Flector(vec4(1.0), vec4(1.0));
}

Line line_unit() {
    return Line(vec3(1.0), vec3(1.0));
}

Motor motor_unit() {
    return Motor(vec4(1.0), vec3(1.0));
}

MultiVector multiVector_unit() {
    return MultiVector(vec2(1.0), vec3(1.0), vec2(1.0), vec3(1.0), vec3(1.0), vec4(1.0), vec4(1.0), vec3(1.0), vec3(1.0), vec3(1.0), vec2(1.0));
}

Plane plane_unit() {
    return Plane(vec4(1.0));
}

RoundPoint roundPoint_unit() {
    return RoundPoint(vec3(1.0), vec2(1.0));
}

Scalar scalar_unit() {
    return Scalar(1.0);
}

Sphere sphere_unit() {
    return Sphere(vec3(1.0), vec2(1.0));
}

AntiScalar antiScalar_zero() {
    return AntiScalar(0.0);
}

Circle circle_zero() {
    return Circle(vec4(0.0), vec3(0.0), vec3(0.0));
}

Dipole dipole_zero() {
    return Dipole(vec3(0.0), vec3(0.0), vec4(0.0));
}

DualNum dualNum_zero() {
    return DualNum(vec2(0.0));
}

FlatPoint flatPoint_zero() {
    return FlatPoint(vec4(0.0));
}

Flector flector_zero() {
    return Flector(vec4(0.0), vec4(0.0));
}

Line line_zero() {
    return Line(vec3(0.0), vec3(0.0));
}

Motor motor_zero() {
    return Motor(vec4(0.0), vec3(0.0));
}

MultiVector multiVector_zero() {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

Plane plane_zero() {
    return Plane(vec4(0.0));
}

RoundPoint roundPoint_zero() {
    return RoundPoint(vec3(0.0), vec2(0.0));
}

Scalar scalar_zero() {
    return Scalar(0.0);
}

Sphere sphere_zero() {
    return Sphere(vec3(0.0), vec2(0.0));
}

AntiScalar antiScalar_neg(AntiScalar self) {
    return AntiScalar(-self.g0_);
}

Circle circle_neg(Circle self) {
    return Circle(self.g0_ * vec4(-1.0), self.g1_ * vec3(-1.0), self.g2_ * vec3(-1.0));
}

Dipole dipole_neg(Dipole self) {
    return Dipole(self.g0_ * vec3(-1.0), self.g1_ * vec3(-1.0), self.g2_ * vec4(-1.0));
}

DualNum dualNum_neg(DualNum self) {
    return DualNum(self.g0_ * vec2(-1.0));
}

FlatPoint flatPoint_neg(FlatPoint self) {
    return FlatPoint(self.g0_ * vec4(-1.0));
}

Flector flector_neg(Flector self) {
    return Flector(self.g0_ * vec4(-1.0), self.g1_ * vec4(-1.0));
}

Line line_neg(Line self) {
    return Line(self.g0_ * vec3(-1.0), self.g1_ * vec3(-1.0));
}

Motor motor_neg(Motor self) {
    return Motor(self.g0_ * vec4(-1.0), self.g1_ * vec3(-1.0));
}

MultiVector multiVector_neg(MultiVector self) {
    return MultiVector(self.g0_ * vec2(-1.0), self.g1_ * vec3(-1.0), self.g2_ * vec2(-1.0), self.g3_ * vec3(-1.0), self.g4_ * vec3(-1.0), self.g5_ * vec4(-1.0), self.g6_ * vec4(-1.0), self.g7_ * vec3(-1.0), self.g8_ * vec3(-1.0), self.g9_ * vec3(-1.0), self.g10_ * vec2(-1.0));
}

Plane plane_neg(Plane self) {
    return Plane(self.g0_ * vec4(-1.0));
}

RoundPoint roundPoint_neg(RoundPoint self) {
    return RoundPoint(self.g0_ * vec3(-1.0), self.g1_ * vec2(-1.0));
}

Scalar scalar_neg(Scalar self) {
    return Scalar(-self.g0_);
}

Sphere sphere_neg(Sphere self) {
    return Sphere(self.g0_ * vec3(-1.0), self.g1_ * vec2(-1.0));
}

AntiScalar antiScalar_add_antiScalar(AntiScalar self, AntiScalar other) {
    return AntiScalar(self.g0_ + other.g0_);
}

MultiVector antiScalar_add_circle(AntiScalar self, Circle other) {
    return MultiVector(vec2(0.0, self.g0_), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), other.g0_, other.g1_, other.g2_, vec3(0.0), vec2(0.0));
}

MultiVector antiScalar_add_dipole(AntiScalar self, Dipole other) {
    return MultiVector(vec2(0.0, self.g0_), vec3(0.0), vec2(0.0), other.g0_, other.g1_, other.g2_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

DualNum antiScalar_add_dualNum(AntiScalar self, DualNum other) {
    return DualNum(vec2(0.0, self.g0_) + other.g0_);
}

MultiVector antiScalar_add_flatPoint(AntiScalar self, FlatPoint other) {
    return MultiVector(vec2(0.0, self.g0_), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), other.g0_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector antiScalar_add_flector(AntiScalar self, Flector other) {
    return MultiVector(vec2(0.0, self.g0_), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), other.g0_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec2(0.0, other.g1_.w));
}

Motor antiScalar_add_line(AntiScalar self, Line other) {
    return Motor(vec4(0.0, 0.0, 0.0, self.g0_) + vec4(other.g0_.x, other.g0_.y, other.g0_.z, 0.0), other.g1_);
}

Motor antiScalar_add_motor(AntiScalar self, Motor other) {
    return Motor(vec4(0.0, 0.0, 0.0, self.g0_) + other.g0_, other.g1_);
}

MultiVector antiScalar_add_multiVector(AntiScalar self, MultiVector other) {
    return MultiVector(vec2(0.0, self.g0_) + other.g0_, other.g1_, other.g2_, other.g3_, other.g4_, other.g5_, other.g6_, other.g7_, other.g8_, other.g9_, other.g10_);
}

MultiVector antiScalar_add_plane(AntiScalar self, Plane other) {
    return MultiVector(vec2(0.0, self.g0_), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(0.0, other.g0_.w));
}

MultiVector antiScalar_add_roundPoint(AntiScalar self, RoundPoint other) {
    return MultiVector(vec2(0.0, self.g0_), other.g0_, other.g1_, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

DualNum antiScalar_add_scalar(AntiScalar self, Scalar other) {
    return DualNum(vec2(0.0, self.g0_) + vec2(other.g0_, 0.0));
}

MultiVector antiScalar_add_sphere(AntiScalar self, Sphere other) {
    return MultiVector(vec2(0.0, self.g0_), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), other.g0_, other.g1_);
}

MultiVector circle_add_antiScalar(Circle self, AntiScalar other) {
    return MultiVector(vec2(0.0, other.g0_), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), self.g0_, self.g1_, self.g2_, vec3(0.0), vec2(0.0));
}

Circle circle_add_circle(Circle self, Circle other) {
    return Circle(self.g0_ + other.g0_, self.g1_ + other.g1_, self.g2_ + other.g2_);
}

MultiVector circle_add_dipole(Circle self, Dipole other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), other.g0_, other.g1_, other.g2_, self.g0_, self.g1_, self.g2_, vec3(0.0), vec2(0.0));
}

MultiVector circle_add_dualNum(Circle self, DualNum other) {
    return MultiVector(other.g0_, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), self.g0_, self.g1_, self.g2_, vec3(0.0), vec2(0.0));
}

MultiVector circle_add_flatPoint(Circle self, FlatPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), other.g0_, self.g0_, self.g1_, self.g2_, vec3(0.0), vec2(0.0));
}

MultiVector circle_add_flector(Circle self, Flector other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), other.g0_, self.g0_, self.g1_, self.g2_, vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec2(0.0, other.g1_.w));
}

Circle circle_add_line(Circle self, Line other) {
    return Circle(self.g0_, self.g1_ + other.g0_, self.g2_ + other.g1_);
}

MultiVector circle_add_motor(Circle self, Motor other) {
    return MultiVector(vec2(0.0, other.g0_.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), self.g0_, self.g1_ + vec3(other.g0_.x, other.g0_.y, other.g0_.z), self.g2_ + other.g1_, vec3(0.0), vec2(0.0));
}

MultiVector circle_add_multiVector(Circle self, MultiVector other) {
    return MultiVector(other.g0_, other.g1_, other.g2_, other.g3_, other.g4_, other.g5_, self.g0_ + other.g6_, self.g1_ + other.g7_, self.g2_ + other.g8_, other.g9_, other.g10_);
}

MultiVector circle_add_plane(Circle self, Plane other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), self.g0_, self.g1_, self.g2_, vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(0.0, other.g0_.w));
}

MultiVector circle_add_roundPoint(Circle self, RoundPoint other) {
    return MultiVector(vec2(0.0), other.g0_, other.g1_, vec3(0.0), vec3(0.0), vec4(0.0), self.g0_, self.g1_, self.g2_, vec3(0.0), vec2(0.0));
}

MultiVector circle_add_scalar(Circle self, Scalar other) {
    return MultiVector(vec2(other.g0_, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), self.g0_, self.g1_, self.g2_, vec3(0.0), vec2(0.0));
}

MultiVector circle_add_sphere(Circle self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), self.g0_, self.g1_, self.g2_, other.g0_, other.g1_);
}

MultiVector dipole_add_antiScalar(Dipole self, AntiScalar other) {
    return MultiVector(vec2(0.0, other.g0_), vec3(0.0), vec2(0.0), self.g0_, self.g1_, self.g2_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector dipole_add_circle(Dipole self, Circle other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), self.g0_, self.g1_, self.g2_, other.g0_, other.g1_, other.g2_, vec3(0.0), vec2(0.0));
}

Dipole dipole_add_dipole(Dipole self, Dipole other) {
    return Dipole(self.g0_ + other.g0_, self.g1_ + other.g1_, self.g2_ + other.g2_);
}

MultiVector dipole_add_dualNum(Dipole self, DualNum other) {
    return MultiVector(other.g0_, vec3(0.0), vec2(0.0), self.g0_, self.g1_, self.g2_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

Dipole dipole_add_flatPoint(Dipole self, FlatPoint other) {
    return Dipole(self.g0_, self.g1_, self.g2_ + other.g0_);
}

MultiVector dipole_add_flector(Dipole self, Flector other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), self.g0_, self.g1_, self.g2_ + other.g0_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec2(0.0, other.g1_.w));
}

MultiVector dipole_add_line(Dipole self, Line other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), self.g0_, self.g1_, self.g2_, vec4(0.0), other.g0_, other.g1_, vec3(0.0), vec2(0.0));
}

MultiVector dipole_add_motor(Dipole self, Motor other) {
    return MultiVector(vec2(0.0, other.g0_.w), vec3(0.0), vec2(0.0), self.g0_, self.g1_, self.g2_, vec4(0.0), vec3(other.g0_.x, other.g0_.y, other.g0_.z), other.g1_, vec3(0.0), vec2(0.0));
}

MultiVector dipole_add_multiVector(Dipole self, MultiVector other) {
    return MultiVector(other.g0_, other.g1_, other.g2_, self.g0_ + other.g3_, self.g1_ + other.g4_, self.g2_ + other.g5_, other.g6_, other.g7_, other.g8_, other.g9_, other.g10_);
}

MultiVector dipole_add_plane(Dipole self, Plane other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), self.g0_, self.g1_, self.g2_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(0.0, other.g0_.w));
}

MultiVector dipole_add_roundPoint(Dipole self, RoundPoint other) {
    return MultiVector(vec2(0.0), other.g0_, other.g1_, self.g0_, self.g1_, self.g2_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector dipole_add_scalar(Dipole self, Scalar other) {
    return MultiVector(vec2(other.g0_, 0.0), vec3(0.0), vec2(0.0), self.g0_, self.g1_, self.g2_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector dipole_add_sphere(Dipole self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), self.g0_, self.g1_, self.g2_, vec4(0.0), vec3(0.0), vec3(0.0), other.g0_, other.g1_);
}

DualNum dualNum_add_antiScalar(DualNum self, AntiScalar other) {
    return DualNum(self.g0_ + vec2(0.0, other.g0_));
}

MultiVector dualNum_add_circle(DualNum self, Circle other) {
    return MultiVector(self.g0_, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), other.g0_, other.g1_, other.g2_, vec3(0.0), vec2(0.0));
}

MultiVector dualNum_add_dipole(DualNum self, Dipole other) {
    return MultiVector(self.g0_, vec3(0.0), vec2(0.0), other.g0_, other.g1_, other.g2_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

DualNum dualNum_add_dualNum(DualNum self, DualNum other) {
    return DualNum(self.g0_ + other.g0_);
}

MultiVector dualNum_add_flatPoint(DualNum self, FlatPoint other) {
    return MultiVector(self.g0_, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), other.g0_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector dualNum_add_flector(DualNum self, Flector other) {
    return MultiVector(self.g0_, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), other.g0_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec2(0.0, other.g1_.w));
}

MultiVector dualNum_add_line(DualNum self, Line other) {
    return MultiVector(self.g0_, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), other.g0_, other.g1_, vec3(0.0), vec2(0.0));
}

MultiVector dualNum_add_motor(DualNum self, Motor other) {
    return MultiVector(self.g0_ + vec2(0.0, other.g0_.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(other.g0_.x, other.g0_.y, other.g0_.z), other.g1_, vec3(0.0), vec2(0.0));
}

MultiVector dualNum_add_multiVector(DualNum self, MultiVector other) {
    return MultiVector(self.g0_ + other.g0_, other.g1_, other.g2_, other.g3_, other.g4_, other.g5_, other.g6_, other.g7_, other.g8_, other.g9_, other.g10_);
}

MultiVector dualNum_add_plane(DualNum self, Plane other) {
    return MultiVector(self.g0_, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(0.0, other.g0_.w));
}

MultiVector dualNum_add_roundPoint(DualNum self, RoundPoint other) {
    return MultiVector(self.g0_, other.g0_, other.g1_, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

DualNum dualNum_add_scalar(DualNum self, Scalar other) {
    return DualNum(self.g0_ + vec2(other.g0_, 0.0));
}

MultiVector dualNum_add_sphere(DualNum self, Sphere other) {
    return MultiVector(self.g0_, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), other.g0_, other.g1_);
}

MultiVector flatPoint_add_antiScalar(FlatPoint self, AntiScalar other) {
    return MultiVector(vec2(0.0, other.g0_), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector flatPoint_add_circle(FlatPoint self, Circle other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0_, other.g0_, other.g1_, other.g2_, vec3(0.0), vec2(0.0));
}

Dipole flatPoint_add_dipole(FlatPoint self, Dipole other) {
    return Dipole(other.g0_, other.g1_, self.g0_ + other.g2_);
}

MultiVector flatPoint_add_dualNum(FlatPoint self, DualNum other) {
    return MultiVector(other.g0_, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

FlatPoint flatPoint_add_flatPoint(FlatPoint self, FlatPoint other) {
    return FlatPoint(self.g0_ + other.g0_);
}

Flector flatPoint_add_flector(FlatPoint self, Flector other) {
    return Flector(self.g0_ + other.g0_, other.g1_);
}

MultiVector flatPoint_add_line(FlatPoint self, Line other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0_, vec4(0.0), other.g0_, other.g1_, vec3(0.0), vec2(0.0));
}

MultiVector flatPoint_add_motor(FlatPoint self, Motor other) {
    return MultiVector(vec2(0.0, other.g0_.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0_, vec4(0.0), vec3(other.g0_.x, other.g0_.y, other.g0_.z), other.g1_, vec3(0.0), vec2(0.0));
}

MultiVector flatPoint_add_multiVector(FlatPoint self, MultiVector other) {
    return MultiVector(other.g0_, other.g1_, other.g2_, other.g3_, other.g4_, self.g0_ + other.g5_, other.g6_, other.g7_, other.g8_, other.g9_, other.g10_);
}

Flector flatPoint_add_plane(FlatPoint self, Plane other) {
    return Flector(self.g0_, other.g0_);
}

MultiVector flatPoint_add_roundPoint(FlatPoint self, RoundPoint other) {
    return MultiVector(vec2(0.0), other.g0_, other.g1_, vec3(0.0), vec3(0.0), self.g0_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector flatPoint_add_scalar(FlatPoint self, Scalar other) {
    return MultiVector(vec2(other.g0_, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector flatPoint_add_sphere(FlatPoint self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0_, vec4(0.0), vec3(0.0), vec3(0.0), other.g0_, other.g1_);
}

MultiVector flector_add_antiScalar(Flector self, AntiScalar other) {
    return MultiVector(vec2(0.0, other.g0_), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g1_.x, self.g1_.y, self.g1_.z), vec2(0.0, self.g1_.w));
}

MultiVector flector_add_circle(Flector self, Circle other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0_, other.g0_, other.g1_, other.g2_, vec3(self.g1_.x, self.g1_.y, self.g1_.z), vec2(0.0, self.g1_.w));
}

MultiVector flector_add_dipole(Flector self, Dipole other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), other.g0_, other.g1_, self.g0_ + other.g2_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g1_.x, self.g1_.y, self.g1_.z), vec2(0.0, self.g1_.w));
}

MultiVector flector_add_dualNum(Flector self, DualNum other) {
    return MultiVector(other.g0_, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g1_.x, self.g1_.y, self.g1_.z), vec2(0.0, self.g1_.w));
}

Flector flector_add_flatPoint(Flector self, FlatPoint other) {
    return Flector(self.g0_ + other.g0_, self.g1_);
}

Flector flector_add_flector(Flector self, Flector other) {
    return Flector(self.g0_ + other.g0_, self.g1_ + other.g1_);
}

MultiVector flector_add_line(Flector self, Line other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0_, vec4(0.0), other.g0_, other.g1_, vec3(self.g1_.x, self.g1_.y, self.g1_.z), vec2(0.0, self.g1_.w));
}

MultiVector flector_add_motor(Flector self, Motor other) {
    return MultiVector(vec2(0.0, other.g0_.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0_, vec4(0.0), vec3(other.g0_.x, other.g0_.y, other.g0_.z), other.g1_, vec3(self.g1_.x, self.g1_.y, self.g1_.z), vec2(0.0, self.g1_.w));
}

MultiVector flector_add_multiVector(Flector self, MultiVector other) {
    return MultiVector(other.g0_, other.g1_, other.g2_, other.g3_, other.g4_, self.g0_ + other.g5_, other.g6_, other.g7_, other.g8_, vec3(self.g1_.x, self.g1_.y, self.g1_.z) + other.g9_, vec2(0.0, self.g1_.w) + other.g10_);
}

Flector flector_add_plane(Flector self, Plane other) {
    return Flector(self.g0_, self.g1_ + other.g0_);
}

MultiVector flector_add_roundPoint(Flector self, RoundPoint other) {
    return MultiVector(vec2(0.0), other.g0_, other.g1_, vec3(0.0), vec3(0.0), self.g0_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g1_.x, self.g1_.y, self.g1_.z), vec2(0.0, self.g1_.w));
}

MultiVector flector_add_scalar(Flector self, Scalar other) {
    return MultiVector(vec2(other.g0_, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g1_.x, self.g1_.y, self.g1_.z), vec2(0.0, self.g1_.w));
}

MultiVector flector_add_sphere(Flector self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g1_.x, self.g1_.y, self.g1_.z) + other.g0_, vec2(0.0, self.g1_.w) + other.g1_);
}

Motor line_add_antiScalar(Line self, AntiScalar other) {
    return Motor(vec4(self.g0_.x, self.g0_.y, self.g0_.z, 0.0) + vec4(0.0, 0.0, 0.0, other.g0_), self.g1_);
}

Circle line_add_circle(Line self, Circle other) {
    return Circle(other.g0_, self.g0_ + other.g1_, self.g1_ + other.g2_);
}

MultiVector line_add_dipole(Line self, Dipole other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), other.g0_, other.g1_, other.g2_, vec4(0.0), self.g0_, self.g1_, vec3(0.0), vec2(0.0));
}

MultiVector line_add_dualNum(Line self, DualNum other) {
    return MultiVector(other.g0_, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), self.g0_, self.g1_, vec3(0.0), vec2(0.0));
}

MultiVector line_add_flatPoint(Line self, FlatPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), other.g0_, vec4(0.0), self.g0_, self.g1_, vec3(0.0), vec2(0.0));
}

MultiVector line_add_flector(Line self, Flector other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), other.g0_, vec4(0.0), self.g0_, self.g1_, vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec2(0.0, other.g1_.w));
}

Line line_add_line(Line self, Line other) {
    return Line(self.g0_ + other.g0_, self.g1_ + other.g1_);
}

Motor line_add_motor(Line self, Motor other) {
    return Motor(vec4(self.g0_.x, self.g0_.y, self.g0_.z, 0.0) + other.g0_, self.g1_ + other.g1_);
}

MultiVector line_add_multiVector(Line self, MultiVector other) {
    return MultiVector(other.g0_, other.g1_, other.g2_, other.g3_, other.g4_, other.g5_, other.g6_, self.g0_ + other.g7_, self.g1_ + other.g8_, other.g9_, other.g10_);
}

MultiVector line_add_plane(Line self, Plane other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), self.g0_, self.g1_, vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(0.0, other.g0_.w));
}

MultiVector line_add_roundPoint(Line self, RoundPoint other) {
    return MultiVector(vec2(0.0), other.g0_, other.g1_, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), self.g0_, self.g1_, vec3(0.0), vec2(0.0));
}

MultiVector line_add_scalar(Line self, Scalar other) {
    return MultiVector(vec2(other.g0_, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), self.g0_, self.g1_, vec3(0.0), vec2(0.0));
}

MultiVector line_add_sphere(Line self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), self.g0_, self.g1_, other.g0_, other.g1_);
}

Motor motor_add_antiScalar(Motor self, AntiScalar other) {
    return Motor(self.g0_ + vec4(0.0, 0.0, 0.0, other.g0_), self.g1_);
}

MultiVector motor_add_circle(Motor self, Circle other) {
    return MultiVector(vec2(0.0, self.g0_.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), other.g0_, vec3(self.g0_.x, self.g0_.y, self.g0_.z) + other.g1_, self.g1_ + other.g2_, vec3(0.0), vec2(0.0));
}

MultiVector motor_add_dipole(Motor self, Dipole other) {
    return MultiVector(vec2(0.0, self.g0_.w), vec3(0.0), vec2(0.0), other.g0_, other.g1_, other.g2_, vec4(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z), self.g1_, vec3(0.0), vec2(0.0));
}

MultiVector motor_add_dualNum(Motor self, DualNum other) {
    return MultiVector(vec2(0.0, self.g0_.w) + other.g0_, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z), self.g1_, vec3(0.0), vec2(0.0));
}

MultiVector motor_add_flatPoint(Motor self, FlatPoint other) {
    return MultiVector(vec2(0.0, self.g0_.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), other.g0_, vec4(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z), self.g1_, vec3(0.0), vec2(0.0));
}

MultiVector motor_add_flector(Motor self, Flector other) {
    return MultiVector(vec2(0.0, self.g0_.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), other.g0_, vec4(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z), self.g1_, vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec2(0.0, other.g1_.w));
}

Motor motor_add_line(Motor self, Line other) {
    return Motor(self.g0_ + vec4(other.g0_.x, other.g0_.y, other.g0_.z, 0.0), self.g1_ + other.g1_);
}

Motor motor_add_motor(Motor self, Motor other) {
    return Motor(self.g0_ + other.g0_, self.g1_ + other.g1_);
}

MultiVector motor_add_multiVector(Motor self, MultiVector other) {
    return MultiVector(vec2(0.0, self.g0_.w) + other.g0_, other.g1_, other.g2_, other.g3_, other.g4_, other.g5_, other.g6_, vec3(self.g0_.x, self.g0_.y, self.g0_.z) + other.g7_, self.g1_ + other.g8_, other.g9_, other.g10_);
}

MultiVector motor_add_plane(Motor self, Plane other) {
    return MultiVector(vec2(0.0, self.g0_.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z), self.g1_, vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(0.0, other.g0_.w));
}

MultiVector motor_add_roundPoint(Motor self, RoundPoint other) {
    return MultiVector(vec2(0.0, self.g0_.w), other.g0_, other.g1_, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z), self.g1_, vec3(0.0), vec2(0.0));
}

MultiVector motor_add_scalar(Motor self, Scalar other) {
    return MultiVector(vec2(0.0, self.g0_.w) + vec2(other.g0_, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z), self.g1_, vec3(0.0), vec2(0.0));
}

MultiVector motor_add_sphere(Motor self, Sphere other) {
    return MultiVector(vec2(0.0, self.g0_.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z), self.g1_, other.g0_, other.g1_);
}

MultiVector multiVector_add_antiScalar(MultiVector self, AntiScalar other) {
    return MultiVector(self.g0_ + vec2(0.0, other.g0_), self.g1_, self.g2_, self.g3_, self.g4_, self.g5_, self.g6_, self.g7_, self.g8_, self.g9_, self.g10_);
}

MultiVector multiVector_add_circle(MultiVector self, Circle other) {
    return MultiVector(self.g0_, self.g1_, self.g2_, self.g3_, self.g4_, self.g5_, self.g6_ + other.g0_, self.g7_ + other.g1_, self.g8_ + other.g2_, self.g9_, self.g10_);
}

MultiVector multiVector_add_dipole(MultiVector self, Dipole other) {
    return MultiVector(self.g0_, self.g1_, self.g2_, self.g3_ + other.g0_, self.g4_ + other.g1_, self.g5_ + other.g2_, self.g6_, self.g7_, self.g8_, self.g9_, self.g10_);
}

MultiVector multiVector_add_dualNum(MultiVector self, DualNum other) {
    return MultiVector(self.g0_ + other.g0_, self.g1_, self.g2_, self.g3_, self.g4_, self.g5_, self.g6_, self.g7_, self.g8_, self.g9_, self.g10_);
}

MultiVector multiVector_add_flatPoint(MultiVector self, FlatPoint other) {
    return MultiVector(self.g0_, self.g1_, self.g2_, self.g3_, self.g4_, self.g5_ + other.g0_, self.g6_, self.g7_, self.g8_, self.g9_, self.g10_);
}

MultiVector multiVector_add_flector(MultiVector self, Flector other) {
    return MultiVector(self.g0_, self.g1_, self.g2_, self.g3_, self.g4_, self.g5_ + other.g0_, self.g6_, self.g7_, self.g8_, self.g9_ + vec3(other.g1_.x, other.g1_.y, other.g1_.z), self.g10_ + vec2(0.0, other.g1_.w));
}

MultiVector multiVector_add_line(MultiVector self, Line other) {
    return MultiVector(self.g0_, self.g1_, self.g2_, self.g3_, self.g4_, self.g5_, self.g6_, self.g7_ + other.g0_, self.g8_ + other.g1_, self.g9_, self.g10_);
}

MultiVector multiVector_add_motor(MultiVector self, Motor other) {
    return MultiVector(self.g0_ + vec2(0.0, other.g0_.w), self.g1_, self.g2_, self.g3_, self.g4_, self.g5_, self.g6_, self.g7_ + vec3(other.g0_.x, other.g0_.y, other.g0_.z), self.g8_ + other.g1_, self.g9_, self.g10_);
}

MultiVector multiVector_add_multiVector(MultiVector self, MultiVector other) {
    return MultiVector(self.g0_ + other.g0_, self.g1_ + other.g1_, self.g2_ + other.g2_, self.g3_ + other.g3_, self.g4_ + other.g4_, self.g5_ + other.g5_, self.g6_ + other.g6_, self.g7_ + other.g7_, self.g8_ + other.g8_, self.g9_ + other.g9_, self.g10_ + other.g10_);
}

MultiVector multiVector_add_plane(MultiVector self, Plane other) {
    return MultiVector(self.g0_, self.g1_, self.g2_, self.g3_, self.g4_, self.g5_, self.g6_, self.g7_, self.g8_, self.g9_ + vec3(other.g0_.x, other.g0_.y, other.g0_.z), self.g10_ + vec2(0.0, other.g0_.w));
}

MultiVector multiVector_add_roundPoint(MultiVector self, RoundPoint other) {
    return MultiVector(self.g0_, self.g1_ + other.g0_, self.g2_ + other.g1_, self.g3_, self.g4_, self.g5_, self.g6_, self.g7_, self.g8_, self.g9_, self.g10_);
}

MultiVector multiVector_add_scalar(MultiVector self, Scalar other) {
    return MultiVector(self.g0_ + vec2(other.g0_, 0.0), self.g1_, self.g2_, self.g3_, self.g4_, self.g5_, self.g6_, self.g7_, self.g8_, self.g9_, self.g10_);
}

MultiVector multiVector_add_sphere(MultiVector self, Sphere other) {
    return MultiVector(self.g0_, self.g1_, self.g2_, self.g3_, self.g4_, self.g5_, self.g6_, self.g7_, self.g8_, self.g9_ + other.g0_, self.g10_ + other.g1_);
}

MultiVector plane_add_antiScalar(Plane self, AntiScalar other) {
    return MultiVector(vec2(0.0, other.g0_), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z), vec2(0.0, self.g0_.w));
}

MultiVector plane_add_circle(Plane self, Circle other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), other.g0_, other.g1_, other.g2_, vec3(self.g0_.x, self.g0_.y, self.g0_.z), vec2(0.0, self.g0_.w));
}

MultiVector plane_add_dipole(Plane self, Dipole other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), other.g0_, other.g1_, other.g2_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z), vec2(0.0, self.g0_.w));
}

MultiVector plane_add_dualNum(Plane self, DualNum other) {
    return MultiVector(other.g0_, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z), vec2(0.0, self.g0_.w));
}

Flector plane_add_flatPoint(Plane self, FlatPoint other) {
    return Flector(other.g0_, self.g0_);
}

Flector plane_add_flector(Plane self, Flector other) {
    return Flector(other.g0_, self.g0_ + other.g1_);
}

MultiVector plane_add_line(Plane self, Line other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), other.g0_, other.g1_, vec3(self.g0_.x, self.g0_.y, self.g0_.z), vec2(0.0, self.g0_.w));
}

MultiVector plane_add_motor(Plane self, Motor other) {
    return MultiVector(vec2(0.0, other.g0_.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(other.g0_.x, other.g0_.y, other.g0_.z), other.g1_, vec3(self.g0_.x, self.g0_.y, self.g0_.z), vec2(0.0, self.g0_.w));
}

MultiVector plane_add_multiVector(Plane self, MultiVector other) {
    return MultiVector(other.g0_, other.g1_, other.g2_, other.g3_, other.g4_, other.g5_, other.g6_, other.g7_, other.g8_, vec3(self.g0_.x, self.g0_.y, self.g0_.z) + other.g9_, vec2(0.0, self.g0_.w) + other.g10_);
}

Plane plane_add_plane(Plane self, Plane other) {
    return Plane(self.g0_ + other.g0_);
}

MultiVector plane_add_roundPoint(Plane self, RoundPoint other) {
    return MultiVector(vec2(0.0), other.g0_, other.g1_, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z), vec2(0.0, self.g0_.w));
}

MultiVector plane_add_scalar(Plane self, Scalar other) {
    return MultiVector(vec2(other.g0_, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z), vec2(0.0, self.g0_.w));
}

Sphere plane_add_sphere(Plane self, Sphere other) {
    return Sphere(vec3(self.g0_.x, self.g0_.y, self.g0_.z) + other.g0_, vec2(0.0, self.g0_.w) + other.g1_);
}

MultiVector roundPoint_add_antiScalar(RoundPoint self, AntiScalar other) {
    return MultiVector(vec2(0.0, other.g0_), self.g0_, self.g1_, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector roundPoint_add_circle(RoundPoint self, Circle other) {
    return MultiVector(vec2(0.0), self.g0_, self.g1_, vec3(0.0), vec3(0.0), vec4(0.0), other.g0_, other.g1_, other.g2_, vec3(0.0), vec2(0.0));
}

MultiVector roundPoint_add_dipole(RoundPoint self, Dipole other) {
    return MultiVector(vec2(0.0), self.g0_, self.g1_, other.g0_, other.g1_, other.g2_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector roundPoint_add_dualNum(RoundPoint self, DualNum other) {
    return MultiVector(other.g0_, self.g0_, self.g1_, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector roundPoint_add_flatPoint(RoundPoint self, FlatPoint other) {
    return MultiVector(vec2(0.0), self.g0_, self.g1_, vec3(0.0), vec3(0.0), other.g0_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector roundPoint_add_flector(RoundPoint self, Flector other) {
    return MultiVector(vec2(0.0), self.g0_, self.g1_, vec3(0.0), vec3(0.0), other.g0_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec2(0.0, other.g1_.w));
}

MultiVector roundPoint_add_line(RoundPoint self, Line other) {
    return MultiVector(vec2(0.0), self.g0_, self.g1_, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), other.g0_, other.g1_, vec3(0.0), vec2(0.0));
}

MultiVector roundPoint_add_motor(RoundPoint self, Motor other) {
    return MultiVector(vec2(0.0, other.g0_.w), self.g0_, self.g1_, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(other.g0_.x, other.g0_.y, other.g0_.z), other.g1_, vec3(0.0), vec2(0.0));
}

MultiVector roundPoint_add_multiVector(RoundPoint self, MultiVector other) {
    return MultiVector(other.g0_, self.g0_ + other.g1_, self.g1_ + other.g2_, other.g3_, other.g4_, other.g5_, other.g6_, other.g7_, other.g8_, other.g9_, other.g10_);
}

MultiVector roundPoint_add_plane(RoundPoint self, Plane other) {
    return MultiVector(vec2(0.0), self.g0_, self.g1_, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(0.0, other.g0_.w));
}

RoundPoint roundPoint_add_roundPoint(RoundPoint self, RoundPoint other) {
    return RoundPoint(self.g0_ + other.g0_, self.g1_ + other.g1_);
}

MultiVector roundPoint_add_scalar(RoundPoint self, Scalar other) {
    return MultiVector(vec2(other.g0_, 0.0), self.g0_, self.g1_, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector roundPoint_add_sphere(RoundPoint self, Sphere other) {
    return MultiVector(vec2(0.0), self.g0_, self.g1_, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), other.g0_, other.g1_);
}

DualNum scalar_add_antiScalar(Scalar self, AntiScalar other) {
    return DualNum(vec2(self.g0_, 0.0) + vec2(0.0, other.g0_));
}

MultiVector scalar_add_circle(Scalar self, Circle other) {
    return MultiVector(vec2(self.g0_, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), other.g0_, other.g1_, other.g2_, vec3(0.0), vec2(0.0));
}

MultiVector scalar_add_dipole(Scalar self, Dipole other) {
    return MultiVector(vec2(self.g0_, 0.0), vec3(0.0), vec2(0.0), other.g0_, other.g1_, other.g2_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

DualNum scalar_add_dualNum(Scalar self, DualNum other) {
    return DualNum(vec2(self.g0_, 0.0) + other.g0_);
}

MultiVector scalar_add_flatPoint(Scalar self, FlatPoint other) {
    return MultiVector(vec2(self.g0_, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), other.g0_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector scalar_add_flector(Scalar self, Flector other) {
    return MultiVector(vec2(self.g0_, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), other.g0_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec2(0.0, other.g1_.w));
}

MultiVector scalar_add_line(Scalar self, Line other) {
    return MultiVector(vec2(self.g0_, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), other.g0_, other.g1_, vec3(0.0), vec2(0.0));
}

MultiVector scalar_add_motor(Scalar self, Motor other) {
    return MultiVector(vec2(self.g0_, 0.0) + vec2(0.0, other.g0_.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(other.g0_.x, other.g0_.y, other.g0_.z), other.g1_, vec3(0.0), vec2(0.0));
}

MultiVector scalar_add_multiVector(Scalar self, MultiVector other) {
    return MultiVector(vec2(self.g0_, 0.0) + other.g0_, other.g1_, other.g2_, other.g3_, other.g4_, other.g5_, other.g6_, other.g7_, other.g8_, other.g9_, other.g10_);
}

MultiVector scalar_add_plane(Scalar self, Plane other) {
    return MultiVector(vec2(self.g0_, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(0.0, other.g0_.w));
}

MultiVector scalar_add_roundPoint(Scalar self, RoundPoint other) {
    return MultiVector(vec2(self.g0_, 0.0), other.g0_, other.g1_, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

Scalar scalar_add_scalar(Scalar self, Scalar other) {
    return Scalar(self.g0_ + other.g0_);
}

MultiVector scalar_add_sphere(Scalar self, Sphere other) {
    return MultiVector(vec2(self.g0_, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), other.g0_, other.g1_);
}

MultiVector sphere_add_antiScalar(Sphere self, AntiScalar other) {
    return MultiVector(vec2(0.0, other.g0_), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), self.g0_, self.g1_);
}

MultiVector sphere_add_circle(Sphere self, Circle other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), other.g0_, other.g1_, other.g2_, self.g0_, self.g1_);
}

MultiVector sphere_add_dipole(Sphere self, Dipole other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), other.g0_, other.g1_, other.g2_, vec4(0.0), vec3(0.0), vec3(0.0), self.g0_, self.g1_);
}

MultiVector sphere_add_dualNum(Sphere self, DualNum other) {
    return MultiVector(other.g0_, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), self.g0_, self.g1_);
}

MultiVector sphere_add_flatPoint(Sphere self, FlatPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), other.g0_, vec4(0.0), vec3(0.0), vec3(0.0), self.g0_, self.g1_);
}

MultiVector sphere_add_flector(Sphere self, Flector other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), other.g0_, vec4(0.0), vec3(0.0), vec3(0.0), self.g0_ + vec3(other.g1_.x, other.g1_.y, other.g1_.z), self.g1_ + vec2(0.0, other.g1_.w));
}

MultiVector sphere_add_line(Sphere self, Line other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), other.g0_, other.g1_, self.g0_, self.g1_);
}

MultiVector sphere_add_motor(Sphere self, Motor other) {
    return MultiVector(vec2(0.0, other.g0_.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(other.g0_.x, other.g0_.y, other.g0_.z), other.g1_, self.g0_, self.g1_);
}

MultiVector sphere_add_multiVector(Sphere self, MultiVector other) {
    return MultiVector(other.g0_, other.g1_, other.g2_, other.g3_, other.g4_, other.g5_, other.g6_, other.g7_, other.g8_, self.g0_ + other.g9_, self.g1_ + other.g10_);
}

Sphere sphere_add_plane(Sphere self, Plane other) {
    return Sphere(self.g0_ + vec3(other.g0_.x, other.g0_.y, other.g0_.z), self.g1_ + vec2(0.0, other.g0_.w));
}

MultiVector sphere_add_roundPoint(Sphere self, RoundPoint other) {
    return MultiVector(vec2(0.0), other.g0_, other.g1_, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), self.g0_, self.g1_);
}

MultiVector sphere_add_scalar(Sphere self, Scalar other) {
    return MultiVector(vec2(other.g0_, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), self.g0_, self.g1_);
}

Sphere sphere_add_sphere(Sphere self, Sphere other) {
    return Sphere(self.g0_ + other.g0_, self.g1_ + other.g1_);
}

AntiScalar antiScalar_div_antiScalar(AntiScalar self, AntiScalar other) {
    return AntiScalar(self.g0_ / other.g0_);
}

Circle circle_div_circle(Circle self, Circle other) {
    return Circle(self.g0_ / other.g0_, self.g1_ / other.g1_, self.g2_ / other.g2_);
}

Dipole dipole_div_dipole(Dipole self, Dipole other) {
    return Dipole(self.g0_ / other.g0_, self.g1_ / other.g1_, self.g2_ / other.g2_);
}

DualNum dualNum_div_dualNum(DualNum self, DualNum other) {
    return DualNum(self.g0_ / other.g0_);
}

FlatPoint flatPoint_div_flatPoint(FlatPoint self, FlatPoint other) {
    return FlatPoint(self.g0_ / other.g0_);
}

Flector flector_div_flector(Flector self, Flector other) {
    return Flector(self.g0_ / other.g0_, self.g1_ / other.g1_);
}

Line line_div_line(Line self, Line other) {
    return Line(self.g0_ / other.g0_, self.g1_ / other.g1_);
}

Motor motor_div_motor(Motor self, Motor other) {
    return Motor(self.g0_ / other.g0_, self.g1_ / other.g1_);
}

MultiVector multiVector_div_multiVector(MultiVector self, MultiVector other) {
    return MultiVector(self.g0_ / other.g0_, self.g1_ / other.g1_, self.g2_ / other.g2_, self.g3_ / other.g3_, self.g4_ / other.g4_, self.g5_ / other.g5_, self.g6_ / other.g6_, self.g7_ / other.g7_, self.g8_ / other.g8_, self.g9_ / other.g9_, self.g10_ / other.g10_);
}

Plane plane_div_plane(Plane self, Plane other) {
    return Plane(self.g0_ / other.g0_);
}

RoundPoint roundPoint_div_roundPoint(RoundPoint self, RoundPoint other) {
    return RoundPoint(self.g0_ / other.g0_, self.g1_ / other.g1_);
}

Scalar scalar_div_scalar(Scalar self, Scalar other) {
    return Scalar(self.g0_ / other.g0_);
}

Sphere sphere_div_sphere(Sphere self, Sphere other) {
    return Sphere(self.g0_ / other.g0_, self.g1_ / other.g1_);
}

Line circle_into_line(Circle self) {
    return Line(self.g1_, self.g2_);
}

FlatPoint dipole_into_flatPoint(Dipole self) {
    return FlatPoint(self.g2_);
}

AntiScalar dualNum_into_antiScalar(DualNum self) {
    return AntiScalar(self.g0_.y);
}

Scalar dualNum_into_scalar(DualNum self) {
    return Scalar(self.g0_.x);
}

FlatPoint flector_into_flatPoint(Flector self) {
    return FlatPoint(self.g0_);
}

Plane flector_into_plane(Flector self) {
    return Plane(self.g1_);
}

AntiScalar motor_into_antiScalar(Motor self) {
    return AntiScalar(self.g0_.w);
}

Line motor_into_line(Motor self) {
    return Line(vec3(self.g0_.x, self.g0_.y, self.g0_.z), self.g1_);
}

AntiScalar multiVector_into_antiScalar(MultiVector self) {
    return AntiScalar(self.g0_.y);
}

Circle multiVector_into_circle(MultiVector self) {
    return Circle(self.g6_, self.g7_, self.g8_);
}

Dipole multiVector_into_dipole(MultiVector self) {
    return Dipole(self.g3_, self.g4_, self.g5_);
}

DualNum multiVector_into_dualNum(MultiVector self) {
    return DualNum(self.g0_);
}

FlatPoint multiVector_into_flatPoint(MultiVector self) {
    return FlatPoint(self.g5_);
}

Flector multiVector_into_flector(MultiVector self) {
    return Flector(self.g5_, vec4(self.g9_.x, self.g9_.y, self.g9_.z, self.g10_.y));
}

Line multiVector_into_line(MultiVector self) {
    return Line(self.g7_, self.g8_);
}

Motor multiVector_into_motor(MultiVector self) {
    return Motor(vec4(self.g7_.x, self.g7_.y, self.g7_.z, self.g0_.y), self.g8_);
}

Plane multiVector_into_plane(MultiVector self) {
    return Plane(vec4(self.g9_.x, self.g9_.y, self.g9_.z, self.g10_.y));
}

RoundPoint multiVector_into_roundPoint(MultiVector self) {
    return RoundPoint(self.g1_, self.g2_);
}

Scalar multiVector_into_scalar(MultiVector self) {
    return Scalar(self.g0_.x);
}

Sphere multiVector_into_sphere(MultiVector self) {
    return Sphere(self.g9_, self.g10_);
}

Plane sphere_into_plane(Sphere self) {
    return Plane(vec4(self.g0_.x, self.g0_.y, self.g0_.z, self.g1_.y));
}

AntiScalar antiScalar_mul_antiScalar(AntiScalar self, AntiScalar other) {
    return AntiScalar(self.g0_ * other.g0_);
}

Circle circle_mul_circle(Circle self, Circle other) {
    return Circle(self.g0_ * other.g0_, self.g1_ * other.g1_, self.g2_ * other.g2_);
}

Dipole dipole_mul_dipole(Dipole self, Dipole other) {
    return Dipole(self.g0_ * other.g0_, self.g1_ * other.g1_, self.g2_ * other.g2_);
}

DualNum dualNum_mul_dualNum(DualNum self, DualNum other) {
    return DualNum(self.g0_ * other.g0_);
}

FlatPoint flatPoint_mul_flatPoint(FlatPoint self, FlatPoint other) {
    return FlatPoint(self.g0_ * other.g0_);
}

Flector flector_mul_flector(Flector self, Flector other) {
    return Flector(self.g0_ * other.g0_, self.g1_ * other.g1_);
}

Line line_mul_line(Line self, Line other) {
    return Line(self.g0_ * other.g0_, self.g1_ * other.g1_);
}

Motor motor_mul_motor(Motor self, Motor other) {
    return Motor(self.g0_ * other.g0_, self.g1_ * other.g1_);
}

MultiVector multiVector_mul_multiVector(MultiVector self, MultiVector other) {
    return MultiVector(self.g0_ * other.g0_, self.g1_ * other.g1_, self.g2_ * other.g2_, self.g3_ * other.g3_, self.g4_ * other.g4_, self.g5_ * other.g5_, self.g6_ * other.g6_, self.g7_ * other.g7_, self.g8_ * other.g8_, self.g9_ * other.g9_, self.g10_ * other.g10_);
}

Plane plane_mul_plane(Plane self, Plane other) {
    return Plane(self.g0_ * other.g0_);
}

RoundPoint roundPoint_mul_roundPoint(RoundPoint self, RoundPoint other) {
    return RoundPoint(self.g0_ * other.g0_, self.g1_ * other.g1_);
}

Scalar scalar_mul_scalar(Scalar self, Scalar other) {
    return Scalar(self.g0_ * other.g0_);
}

Sphere sphere_mul_sphere(Sphere self, Sphere other) {
    return Sphere(self.g0_ * other.g0_, self.g1_ * other.g1_);
}

AntiScalar antiScalar_sub_antiScalar(AntiScalar self, AntiScalar other) {
    return AntiScalar(self.g0_ - other.g0_);
}

MultiVector antiScalar_sub_circle(AntiScalar self, Circle other) {
    return MultiVector(vec2(0.0, self.g0_), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0) - other.g0_, vec3(0.0) - other.g1_, vec3(0.0) - other.g2_, vec3(0.0), vec2(0.0));
}

MultiVector antiScalar_sub_dipole(AntiScalar self, Dipole other) {
    return MultiVector(vec2(0.0, self.g0_), vec3(0.0), vec2(0.0), vec3(0.0) - other.g0_, vec3(0.0) - other.g1_, vec4(0.0) - other.g2_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

DualNum antiScalar_sub_dualNum(AntiScalar self, DualNum other) {
    return DualNum(vec2(0.0, self.g0_) - other.g0_);
}

MultiVector antiScalar_sub_flatPoint(AntiScalar self, FlatPoint other) {
    return MultiVector(vec2(0.0, self.g0_), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0) - other.g0_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector antiScalar_sub_flector(AntiScalar self, Flector other) {
    return MultiVector(vec2(0.0, self.g0_), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0) - other.g0_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec2(0.0) - vec2(0.0, other.g1_.w));
}

Motor antiScalar_sub_line(AntiScalar self, Line other) {
    return Motor(vec4(0.0, 0.0, 0.0, self.g0_) - vec4(other.g0_.x, other.g0_.y, other.g0_.z, 0.0), vec3(0.0) - other.g1_);
}

Motor antiScalar_sub_motor(AntiScalar self, Motor other) {
    return Motor(vec4(0.0, 0.0, 0.0, self.g0_) - other.g0_, vec3(0.0) - other.g1_);
}

MultiVector antiScalar_sub_multiVector(AntiScalar self, MultiVector other) {
    return MultiVector(vec2(0.0, self.g0_) - other.g0_, vec3(0.0) - other.g1_, vec2(0.0) - other.g2_, vec3(0.0) - other.g3_, vec3(0.0) - other.g4_, vec4(0.0) - other.g5_, vec4(0.0) - other.g6_, vec3(0.0) - other.g7_, vec3(0.0) - other.g8_, vec3(0.0) - other.g9_, vec2(0.0) - other.g10_);
}

MultiVector antiScalar_sub_plane(AntiScalar self, Plane other) {
    return MultiVector(vec2(0.0, self.g0_), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(0.0) - vec2(0.0, other.g0_.w));
}

MultiVector antiScalar_sub_roundPoint(AntiScalar self, RoundPoint other) {
    return MultiVector(vec2(0.0, self.g0_), vec3(0.0) - other.g0_, vec2(0.0) - other.g1_, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

DualNum antiScalar_sub_scalar(AntiScalar self, Scalar other) {
    return DualNum(vec2(0.0, self.g0_) - vec2(other.g0_, 0.0));
}

MultiVector antiScalar_sub_sphere(AntiScalar self, Sphere other) {
    return MultiVector(vec2(0.0, self.g0_), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - other.g0_, vec2(0.0) - other.g1_);
}

MultiVector circle_sub_antiScalar(Circle self, AntiScalar other) {
    return MultiVector(vec2(0.0) - vec2(0.0, other.g0_), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), self.g0_, self.g1_, self.g2_, vec3(0.0), vec2(0.0));
}

Circle circle_sub_circle(Circle self, Circle other) {
    return Circle(self.g0_ - other.g0_, self.g1_ - other.g1_, self.g2_ - other.g2_);
}

MultiVector circle_sub_dipole(Circle self, Dipole other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0) - other.g0_, vec3(0.0) - other.g1_, vec4(0.0) - other.g2_, self.g0_, self.g1_, self.g2_, vec3(0.0), vec2(0.0));
}

MultiVector circle_sub_dualNum(Circle self, DualNum other) {
    return MultiVector(vec2(0.0) - other.g0_, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), self.g0_, self.g1_, self.g2_, vec3(0.0), vec2(0.0));
}

MultiVector circle_sub_flatPoint(Circle self, FlatPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0) - other.g0_, self.g0_, self.g1_, self.g2_, vec3(0.0), vec2(0.0));
}

MultiVector circle_sub_flector(Circle self, Flector other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0) - other.g0_, self.g0_, self.g1_, self.g2_, vec3(0.0) - vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec2(0.0) - vec2(0.0, other.g1_.w));
}

Circle circle_sub_line(Circle self, Line other) {
    return Circle(self.g0_, self.g1_ - other.g0_, self.g2_ - other.g1_);
}

MultiVector circle_sub_motor(Circle self, Motor other) {
    return MultiVector(vec2(0.0) - vec2(0.0, other.g0_.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), self.g0_, self.g1_ - vec3(other.g0_.x, other.g0_.y, other.g0_.z), self.g2_ - other.g1_, vec3(0.0), vec2(0.0));
}

MultiVector circle_sub_multiVector(Circle self, MultiVector other) {
    return MultiVector(vec2(0.0) - other.g0_, vec3(0.0) - other.g1_, vec2(0.0) - other.g2_, vec3(0.0) - other.g3_, vec3(0.0) - other.g4_, vec4(0.0) - other.g5_, self.g0_ - other.g6_, self.g1_ - other.g7_, self.g2_ - other.g8_, vec3(0.0) - other.g9_, vec2(0.0) - other.g10_);
}

MultiVector circle_sub_plane(Circle self, Plane other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), self.g0_, self.g1_, self.g2_, vec3(0.0) - vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(0.0) - vec2(0.0, other.g0_.w));
}

MultiVector circle_sub_roundPoint(Circle self, RoundPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0) - other.g0_, vec2(0.0) - other.g1_, vec3(0.0), vec3(0.0), vec4(0.0), self.g0_, self.g1_, self.g2_, vec3(0.0), vec2(0.0));
}

MultiVector circle_sub_scalar(Circle self, Scalar other) {
    return MultiVector(vec2(0.0) - vec2(other.g0_, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), self.g0_, self.g1_, self.g2_, vec3(0.0), vec2(0.0));
}

MultiVector circle_sub_sphere(Circle self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), self.g0_, self.g1_, self.g2_, vec3(0.0) - other.g0_, vec2(0.0) - other.g1_);
}

MultiVector dipole_sub_antiScalar(Dipole self, AntiScalar other) {
    return MultiVector(vec2(0.0) - vec2(0.0, other.g0_), vec3(0.0), vec2(0.0), self.g0_, self.g1_, self.g2_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector dipole_sub_circle(Dipole self, Circle other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), self.g0_, self.g1_, self.g2_, vec4(0.0) - other.g0_, vec3(0.0) - other.g1_, vec3(0.0) - other.g2_, vec3(0.0), vec2(0.0));
}

Dipole dipole_sub_dipole(Dipole self, Dipole other) {
    return Dipole(self.g0_ - other.g0_, self.g1_ - other.g1_, self.g2_ - other.g2_);
}

MultiVector dipole_sub_dualNum(Dipole self, DualNum other) {
    return MultiVector(vec2(0.0) - other.g0_, vec3(0.0), vec2(0.0), self.g0_, self.g1_, self.g2_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

Dipole dipole_sub_flatPoint(Dipole self, FlatPoint other) {
    return Dipole(self.g0_, self.g1_, self.g2_ - other.g0_);
}

MultiVector dipole_sub_flector(Dipole self, Flector other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), self.g0_, self.g1_, self.g2_ - other.g0_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec2(0.0) - vec2(0.0, other.g1_.w));
}

MultiVector dipole_sub_line(Dipole self, Line other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), self.g0_, self.g1_, self.g2_, vec4(0.0), vec3(0.0) - other.g0_, vec3(0.0) - other.g1_, vec3(0.0), vec2(0.0));
}

MultiVector dipole_sub_motor(Dipole self, Motor other) {
    return MultiVector(vec2(0.0) - vec2(0.0, other.g0_.w), vec3(0.0), vec2(0.0), self.g0_, self.g1_, self.g2_, vec4(0.0), vec3(0.0) - vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(0.0) - other.g1_, vec3(0.0), vec2(0.0));
}

MultiVector dipole_sub_multiVector(Dipole self, MultiVector other) {
    return MultiVector(vec2(0.0) - other.g0_, vec3(0.0) - other.g1_, vec2(0.0) - other.g2_, self.g0_ - other.g3_, self.g1_ - other.g4_, self.g2_ - other.g5_, vec4(0.0) - other.g6_, vec3(0.0) - other.g7_, vec3(0.0) - other.g8_, vec3(0.0) - other.g9_, vec2(0.0) - other.g10_);
}

MultiVector dipole_sub_plane(Dipole self, Plane other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), self.g0_, self.g1_, self.g2_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(0.0) - vec2(0.0, other.g0_.w));
}

MultiVector dipole_sub_roundPoint(Dipole self, RoundPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0) - other.g0_, vec2(0.0) - other.g1_, self.g0_, self.g1_, self.g2_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector dipole_sub_scalar(Dipole self, Scalar other) {
    return MultiVector(vec2(0.0) - vec2(other.g0_, 0.0), vec3(0.0), vec2(0.0), self.g0_, self.g1_, self.g2_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector dipole_sub_sphere(Dipole self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), self.g0_, self.g1_, self.g2_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - other.g0_, vec2(0.0) - other.g1_);
}

DualNum dualNum_sub_antiScalar(DualNum self, AntiScalar other) {
    return DualNum(self.g0_ - vec2(0.0, other.g0_));
}

MultiVector dualNum_sub_circle(DualNum self, Circle other) {
    return MultiVector(self.g0_, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0) - other.g0_, vec3(0.0) - other.g1_, vec3(0.0) - other.g2_, vec3(0.0), vec2(0.0));
}

MultiVector dualNum_sub_dipole(DualNum self, Dipole other) {
    return MultiVector(self.g0_, vec3(0.0), vec2(0.0), vec3(0.0) - other.g0_, vec3(0.0) - other.g1_, vec4(0.0) - other.g2_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

DualNum dualNum_sub_dualNum(DualNum self, DualNum other) {
    return DualNum(self.g0_ - other.g0_);
}

MultiVector dualNum_sub_flatPoint(DualNum self, FlatPoint other) {
    return MultiVector(self.g0_, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0) - other.g0_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector dualNum_sub_flector(DualNum self, Flector other) {
    return MultiVector(self.g0_, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0) - other.g0_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec2(0.0) - vec2(0.0, other.g1_.w));
}

MultiVector dualNum_sub_line(DualNum self, Line other) {
    return MultiVector(self.g0_, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0) - other.g0_, vec3(0.0) - other.g1_, vec3(0.0), vec2(0.0));
}

MultiVector dualNum_sub_motor(DualNum self, Motor other) {
    return MultiVector(self.g0_ - vec2(0.0, other.g0_.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0) - vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(0.0) - other.g1_, vec3(0.0), vec2(0.0));
}

MultiVector dualNum_sub_multiVector(DualNum self, MultiVector other) {
    return MultiVector(self.g0_ - other.g0_, vec3(0.0) - other.g1_, vec2(0.0) - other.g2_, vec3(0.0) - other.g3_, vec3(0.0) - other.g4_, vec4(0.0) - other.g5_, vec4(0.0) - other.g6_, vec3(0.0) - other.g7_, vec3(0.0) - other.g8_, vec3(0.0) - other.g9_, vec2(0.0) - other.g10_);
}

MultiVector dualNum_sub_plane(DualNum self, Plane other) {
    return MultiVector(self.g0_, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(0.0) - vec2(0.0, other.g0_.w));
}

MultiVector dualNum_sub_roundPoint(DualNum self, RoundPoint other) {
    return MultiVector(self.g0_, vec3(0.0) - other.g0_, vec2(0.0) - other.g1_, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

DualNum dualNum_sub_scalar(DualNum self, Scalar other) {
    return DualNum(self.g0_ - vec2(other.g0_, 0.0));
}

MultiVector dualNum_sub_sphere(DualNum self, Sphere other) {
    return MultiVector(self.g0_, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - other.g0_, vec2(0.0) - other.g1_);
}

MultiVector flatPoint_sub_antiScalar(FlatPoint self, AntiScalar other) {
    return MultiVector(vec2(0.0) - vec2(0.0, other.g0_), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector flatPoint_sub_circle(FlatPoint self, Circle other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0_, vec4(0.0) - other.g0_, vec3(0.0) - other.g1_, vec3(0.0) - other.g2_, vec3(0.0), vec2(0.0));
}

Dipole flatPoint_sub_dipole(FlatPoint self, Dipole other) {
    return Dipole(vec3(0.0) - other.g0_, vec3(0.0) - other.g1_, self.g0_ - other.g2_);
}

MultiVector flatPoint_sub_dualNum(FlatPoint self, DualNum other) {
    return MultiVector(vec2(0.0) - other.g0_, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

FlatPoint flatPoint_sub_flatPoint(FlatPoint self, FlatPoint other) {
    return FlatPoint(self.g0_ - other.g0_);
}

Flector flatPoint_sub_flector(FlatPoint self, Flector other) {
    return Flector(self.g0_ - other.g0_, vec4(0.0) - other.g1_);
}

MultiVector flatPoint_sub_line(FlatPoint self, Line other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0_, vec4(0.0), vec3(0.0) - other.g0_, vec3(0.0) - other.g1_, vec3(0.0), vec2(0.0));
}

MultiVector flatPoint_sub_motor(FlatPoint self, Motor other) {
    return MultiVector(vec2(0.0) - vec2(0.0, other.g0_.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0_, vec4(0.0), vec3(0.0) - vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(0.0) - other.g1_, vec3(0.0), vec2(0.0));
}

MultiVector flatPoint_sub_multiVector(FlatPoint self, MultiVector other) {
    return MultiVector(vec2(0.0) - other.g0_, vec3(0.0) - other.g1_, vec2(0.0) - other.g2_, vec3(0.0) - other.g3_, vec3(0.0) - other.g4_, self.g0_ - other.g5_, vec4(0.0) - other.g6_, vec3(0.0) - other.g7_, vec3(0.0) - other.g8_, vec3(0.0) - other.g9_, vec2(0.0) - other.g10_);
}

Flector flatPoint_sub_plane(FlatPoint self, Plane other) {
    return Flector(self.g0_, vec4(0.0) - other.g0_);
}

MultiVector flatPoint_sub_roundPoint(FlatPoint self, RoundPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0) - other.g0_, vec2(0.0) - other.g1_, vec3(0.0), vec3(0.0), self.g0_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector flatPoint_sub_scalar(FlatPoint self, Scalar other) {
    return MultiVector(vec2(0.0) - vec2(other.g0_, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector flatPoint_sub_sphere(FlatPoint self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - other.g0_, vec2(0.0) - other.g1_);
}

MultiVector flector_sub_antiScalar(Flector self, AntiScalar other) {
    return MultiVector(vec2(0.0) - vec2(0.0, other.g0_), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g1_.x, self.g1_.y, self.g1_.z), vec2(0.0, self.g1_.w));
}

MultiVector flector_sub_circle(Flector self, Circle other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0_, vec4(0.0) - other.g0_, vec3(0.0) - other.g1_, vec3(0.0) - other.g2_, vec3(self.g1_.x, self.g1_.y, self.g1_.z), vec2(0.0, self.g1_.w));
}

MultiVector flector_sub_dipole(Flector self, Dipole other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0) - other.g0_, vec3(0.0) - other.g1_, self.g0_ - other.g2_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g1_.x, self.g1_.y, self.g1_.z), vec2(0.0, self.g1_.w));
}

MultiVector flector_sub_dualNum(Flector self, DualNum other) {
    return MultiVector(vec2(0.0) - other.g0_, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g1_.x, self.g1_.y, self.g1_.z), vec2(0.0, self.g1_.w));
}

Flector flector_sub_flatPoint(Flector self, FlatPoint other) {
    return Flector(self.g0_ - other.g0_, self.g1_);
}

Flector flector_sub_flector(Flector self, Flector other) {
    return Flector(self.g0_ - other.g0_, self.g1_ - other.g1_);
}

MultiVector flector_sub_line(Flector self, Line other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0_, vec4(0.0), vec3(0.0) - other.g0_, vec3(0.0) - other.g1_, vec3(self.g1_.x, self.g1_.y, self.g1_.z), vec2(0.0, self.g1_.w));
}

MultiVector flector_sub_motor(Flector self, Motor other) {
    return MultiVector(vec2(0.0) - vec2(0.0, other.g0_.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0_, vec4(0.0), vec3(0.0) - vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(0.0) - other.g1_, vec3(self.g1_.x, self.g1_.y, self.g1_.z), vec2(0.0, self.g1_.w));
}

MultiVector flector_sub_multiVector(Flector self, MultiVector other) {
    return MultiVector(vec2(0.0) - other.g0_, vec3(0.0) - other.g1_, vec2(0.0) - other.g2_, vec3(0.0) - other.g3_, vec3(0.0) - other.g4_, self.g0_ - other.g5_, vec4(0.0) - other.g6_, vec3(0.0) - other.g7_, vec3(0.0) - other.g8_, vec3(self.g1_.x, self.g1_.y, self.g1_.z) - other.g9_, vec2(0.0, self.g1_.w) - other.g10_);
}

Flector flector_sub_plane(Flector self, Plane other) {
    return Flector(self.g0_, self.g1_ - other.g0_);
}

MultiVector flector_sub_roundPoint(Flector self, RoundPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0) - other.g0_, vec2(0.0) - other.g1_, vec3(0.0), vec3(0.0), self.g0_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g1_.x, self.g1_.y, self.g1_.z), vec2(0.0, self.g1_.w));
}

MultiVector flector_sub_scalar(Flector self, Scalar other) {
    return MultiVector(vec2(0.0) - vec2(other.g0_, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g1_.x, self.g1_.y, self.g1_.z), vec2(0.0, self.g1_.w));
}

MultiVector flector_sub_sphere(Flector self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g1_.x, self.g1_.y, self.g1_.z) - other.g0_, vec2(0.0, self.g1_.w) - other.g1_);
}

Motor line_sub_antiScalar(Line self, AntiScalar other) {
    return Motor(vec4(self.g0_.x, self.g0_.y, self.g0_.z, 0.0) - vec4(0.0, 0.0, 0.0, other.g0_), self.g1_);
}

Circle line_sub_circle(Line self, Circle other) {
    return Circle(vec4(0.0) - other.g0_, self.g0_ - other.g1_, self.g1_ - other.g2_);
}

MultiVector line_sub_dipole(Line self, Dipole other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0) - other.g0_, vec3(0.0) - other.g1_, vec4(0.0) - other.g2_, vec4(0.0), self.g0_, self.g1_, vec3(0.0), vec2(0.0));
}

MultiVector line_sub_dualNum(Line self, DualNum other) {
    return MultiVector(vec2(0.0) - other.g0_, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), self.g0_, self.g1_, vec3(0.0), vec2(0.0));
}

MultiVector line_sub_flatPoint(Line self, FlatPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0) - other.g0_, vec4(0.0), self.g0_, self.g1_, vec3(0.0), vec2(0.0));
}

MultiVector line_sub_flector(Line self, Flector other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0) - other.g0_, vec4(0.0), self.g0_, self.g1_, vec3(0.0) - vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec2(0.0) - vec2(0.0, other.g1_.w));
}

Line line_sub_line(Line self, Line other) {
    return Line(self.g0_ - other.g0_, self.g1_ - other.g1_);
}

Motor line_sub_motor(Line self, Motor other) {
    return Motor(vec4(self.g0_.x, self.g0_.y, self.g0_.z, 0.0) - other.g0_, self.g1_ - other.g1_);
}

MultiVector line_sub_multiVector(Line self, MultiVector other) {
    return MultiVector(vec2(0.0) - other.g0_, vec3(0.0) - other.g1_, vec2(0.0) - other.g2_, vec3(0.0) - other.g3_, vec3(0.0) - other.g4_, vec4(0.0) - other.g5_, vec4(0.0) - other.g6_, self.g0_ - other.g7_, self.g1_ - other.g8_, vec3(0.0) - other.g9_, vec2(0.0) - other.g10_);
}

MultiVector line_sub_plane(Line self, Plane other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), self.g0_, self.g1_, vec3(0.0) - vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(0.0) - vec2(0.0, other.g0_.w));
}

MultiVector line_sub_roundPoint(Line self, RoundPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0) - other.g0_, vec2(0.0) - other.g1_, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), self.g0_, self.g1_, vec3(0.0), vec2(0.0));
}

MultiVector line_sub_scalar(Line self, Scalar other) {
    return MultiVector(vec2(0.0) - vec2(other.g0_, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), self.g0_, self.g1_, vec3(0.0), vec2(0.0));
}

MultiVector line_sub_sphere(Line self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), self.g0_, self.g1_, vec3(0.0) - other.g0_, vec2(0.0) - other.g1_);
}

Motor motor_sub_antiScalar(Motor self, AntiScalar other) {
    return Motor(self.g0_ - vec4(0.0, 0.0, 0.0, other.g0_), self.g1_);
}

MultiVector motor_sub_circle(Motor self, Circle other) {
    return MultiVector(vec2(0.0, self.g0_.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0) - other.g0_, vec3(self.g0_.x, self.g0_.y, self.g0_.z) - other.g1_, self.g1_ - other.g2_, vec3(0.0), vec2(0.0));
}

MultiVector motor_sub_dipole(Motor self, Dipole other) {
    return MultiVector(vec2(0.0, self.g0_.w), vec3(0.0), vec2(0.0), vec3(0.0) - other.g0_, vec3(0.0) - other.g1_, vec4(0.0) - other.g2_, vec4(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z), self.g1_, vec3(0.0), vec2(0.0));
}

MultiVector motor_sub_dualNum(Motor self, DualNum other) {
    return MultiVector(vec2(0.0, self.g0_.w) - other.g0_, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z), self.g1_, vec3(0.0), vec2(0.0));
}

MultiVector motor_sub_flatPoint(Motor self, FlatPoint other) {
    return MultiVector(vec2(0.0, self.g0_.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0) - other.g0_, vec4(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z), self.g1_, vec3(0.0), vec2(0.0));
}

MultiVector motor_sub_flector(Motor self, Flector other) {
    return MultiVector(vec2(0.0, self.g0_.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0) - other.g0_, vec4(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z), self.g1_, vec3(0.0) - vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec2(0.0) - vec2(0.0, other.g1_.w));
}

Motor motor_sub_line(Motor self, Line other) {
    return Motor(self.g0_ - vec4(other.g0_.x, other.g0_.y, other.g0_.z, 0.0), self.g1_ - other.g1_);
}

Motor motor_sub_motor(Motor self, Motor other) {
    return Motor(self.g0_ - other.g0_, self.g1_ - other.g1_);
}

MultiVector motor_sub_multiVector(Motor self, MultiVector other) {
    return MultiVector(vec2(0.0, self.g0_.w) - other.g0_, vec3(0.0) - other.g1_, vec2(0.0) - other.g2_, vec3(0.0) - other.g3_, vec3(0.0) - other.g4_, vec4(0.0) - other.g5_, vec4(0.0) - other.g6_, vec3(self.g0_.x, self.g0_.y, self.g0_.z) - other.g7_, self.g1_ - other.g8_, vec3(0.0) - other.g9_, vec2(0.0) - other.g10_);
}

MultiVector motor_sub_plane(Motor self, Plane other) {
    return MultiVector(vec2(0.0, self.g0_.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z), self.g1_, vec3(0.0) - vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(0.0) - vec2(0.0, other.g0_.w));
}

MultiVector motor_sub_roundPoint(Motor self, RoundPoint other) {
    return MultiVector(vec2(0.0, self.g0_.w), vec3(0.0) - other.g0_, vec2(0.0) - other.g1_, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z), self.g1_, vec3(0.0), vec2(0.0));
}

MultiVector motor_sub_scalar(Motor self, Scalar other) {
    return MultiVector(vec2(0.0, self.g0_.w) - vec2(other.g0_, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z), self.g1_, vec3(0.0), vec2(0.0));
}

MultiVector motor_sub_sphere(Motor self, Sphere other) {
    return MultiVector(vec2(0.0, self.g0_.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z), self.g1_, vec3(0.0) - other.g0_, vec2(0.0) - other.g1_);
}

MultiVector multiVector_sub_antiScalar(MultiVector self, AntiScalar other) {
    return MultiVector(self.g0_ - vec2(0.0, other.g0_), self.g1_, self.g2_, self.g3_, self.g4_, self.g5_, self.g6_, self.g7_, self.g8_, self.g9_, self.g10_);
}

MultiVector multiVector_sub_circle(MultiVector self, Circle other) {
    return MultiVector(self.g0_, self.g1_, self.g2_, self.g3_, self.g4_, self.g5_, self.g6_ - other.g0_, self.g7_ - other.g1_, self.g8_ - other.g2_, self.g9_, self.g10_);
}

MultiVector multiVector_sub_dipole(MultiVector self, Dipole other) {
    return MultiVector(self.g0_, self.g1_, self.g2_, self.g3_ - other.g0_, self.g4_ - other.g1_, self.g5_ - other.g2_, self.g6_, self.g7_, self.g8_, self.g9_, self.g10_);
}

MultiVector multiVector_sub_dualNum(MultiVector self, DualNum other) {
    return MultiVector(self.g0_ - other.g0_, self.g1_, self.g2_, self.g3_, self.g4_, self.g5_, self.g6_, self.g7_, self.g8_, self.g9_, self.g10_);
}

MultiVector multiVector_sub_flatPoint(MultiVector self, FlatPoint other) {
    return MultiVector(self.g0_, self.g1_, self.g2_, self.g3_, self.g4_, self.g5_ - other.g0_, self.g6_, self.g7_, self.g8_, self.g9_, self.g10_);
}

MultiVector multiVector_sub_flector(MultiVector self, Flector other) {
    return MultiVector(self.g0_, self.g1_, self.g2_, self.g3_, self.g4_, self.g5_ - other.g0_, self.g6_, self.g7_, self.g8_, self.g9_ - vec3(other.g1_.x, other.g1_.y, other.g1_.z), self.g10_ - vec2(0.0, other.g1_.w));
}

MultiVector multiVector_sub_line(MultiVector self, Line other) {
    return MultiVector(self.g0_, self.g1_, self.g2_, self.g3_, self.g4_, self.g5_, self.g6_, self.g7_ - other.g0_, self.g8_ - other.g1_, self.g9_, self.g10_);
}

MultiVector multiVector_sub_motor(MultiVector self, Motor other) {
    return MultiVector(self.g0_ - vec2(0.0, other.g0_.w), self.g1_, self.g2_, self.g3_, self.g4_, self.g5_, self.g6_, self.g7_ - vec3(other.g0_.x, other.g0_.y, other.g0_.z), self.g8_ - other.g1_, self.g9_, self.g10_);
}

MultiVector multiVector_sub_multiVector(MultiVector self, MultiVector other) {
    return MultiVector(self.g0_ - other.g0_, self.g1_ - other.g1_, self.g2_ - other.g2_, self.g3_ - other.g3_, self.g4_ - other.g4_, self.g5_ - other.g5_, self.g6_ - other.g6_, self.g7_ - other.g7_, self.g8_ - other.g8_, self.g9_ - other.g9_, self.g10_ - other.g10_);
}

MultiVector multiVector_sub_plane(MultiVector self, Plane other) {
    return MultiVector(self.g0_, self.g1_, self.g2_, self.g3_, self.g4_, self.g5_, self.g6_, self.g7_, self.g8_, self.g9_ - vec3(other.g0_.x, other.g0_.y, other.g0_.z), self.g10_ - vec2(0.0, other.g0_.w));
}

MultiVector multiVector_sub_roundPoint(MultiVector self, RoundPoint other) {
    return MultiVector(self.g0_, self.g1_ - other.g0_, self.g2_ - other.g1_, self.g3_, self.g4_, self.g5_, self.g6_, self.g7_, self.g8_, self.g9_, self.g10_);
}

MultiVector multiVector_sub_scalar(MultiVector self, Scalar other) {
    return MultiVector(self.g0_ - vec2(other.g0_, 0.0), self.g1_, self.g2_, self.g3_, self.g4_, self.g5_, self.g6_, self.g7_, self.g8_, self.g9_, self.g10_);
}

MultiVector multiVector_sub_sphere(MultiVector self, Sphere other) {
    return MultiVector(self.g0_, self.g1_, self.g2_, self.g3_, self.g4_, self.g5_, self.g6_, self.g7_, self.g8_, self.g9_ - other.g0_, self.g10_ - other.g1_);
}

MultiVector plane_sub_antiScalar(Plane self, AntiScalar other) {
    return MultiVector(vec2(0.0) - vec2(0.0, other.g0_), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z), vec2(0.0, self.g0_.w));
}

MultiVector plane_sub_circle(Plane self, Circle other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0) - other.g0_, vec3(0.0) - other.g1_, vec3(0.0) - other.g2_, vec3(self.g0_.x, self.g0_.y, self.g0_.z), vec2(0.0, self.g0_.w));
}

MultiVector plane_sub_dipole(Plane self, Dipole other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0) - other.g0_, vec3(0.0) - other.g1_, vec4(0.0) - other.g2_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z), vec2(0.0, self.g0_.w));
}

MultiVector plane_sub_dualNum(Plane self, DualNum other) {
    return MultiVector(vec2(0.0) - other.g0_, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z), vec2(0.0, self.g0_.w));
}

Flector plane_sub_flatPoint(Plane self, FlatPoint other) {
    return Flector(vec4(0.0) - other.g0_, self.g0_);
}

Flector plane_sub_flector(Plane self, Flector other) {
    return Flector(vec4(0.0) - other.g0_, self.g0_ - other.g1_);
}

MultiVector plane_sub_line(Plane self, Line other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0) - other.g0_, vec3(0.0) - other.g1_, vec3(self.g0_.x, self.g0_.y, self.g0_.z), vec2(0.0, self.g0_.w));
}

MultiVector plane_sub_motor(Plane self, Motor other) {
    return MultiVector(vec2(0.0) - vec2(0.0, other.g0_.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0) - vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(0.0) - other.g1_, vec3(self.g0_.x, self.g0_.y, self.g0_.z), vec2(0.0, self.g0_.w));
}

MultiVector plane_sub_multiVector(Plane self, MultiVector other) {
    return MultiVector(vec2(0.0) - other.g0_, vec3(0.0) - other.g1_, vec2(0.0) - other.g2_, vec3(0.0) - other.g3_, vec3(0.0) - other.g4_, vec4(0.0) - other.g5_, vec4(0.0) - other.g6_, vec3(0.0) - other.g7_, vec3(0.0) - other.g8_, vec3(self.g0_.x, self.g0_.y, self.g0_.z) - other.g9_, vec2(0.0, self.g0_.w) - other.g10_);
}

Plane plane_sub_plane(Plane self, Plane other) {
    return Plane(self.g0_ - other.g0_);
}

MultiVector plane_sub_roundPoint(Plane self, RoundPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0) - other.g0_, vec2(0.0) - other.g1_, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z), vec2(0.0, self.g0_.w));
}

MultiVector plane_sub_scalar(Plane self, Scalar other) {
    return MultiVector(vec2(0.0) - vec2(other.g0_, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z), vec2(0.0, self.g0_.w));
}

Sphere plane_sub_sphere(Plane self, Sphere other) {
    return Sphere(vec3(self.g0_.x, self.g0_.y, self.g0_.z) - other.g0_, vec2(0.0, self.g0_.w) - other.g1_);
}

MultiVector roundPoint_sub_antiScalar(RoundPoint self, AntiScalar other) {
    return MultiVector(vec2(0.0) - vec2(0.0, other.g0_), self.g0_, self.g1_, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector roundPoint_sub_circle(RoundPoint self, Circle other) {
    return MultiVector(vec2(0.0), self.g0_, self.g1_, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0) - other.g0_, vec3(0.0) - other.g1_, vec3(0.0) - other.g2_, vec3(0.0), vec2(0.0));
}

MultiVector roundPoint_sub_dipole(RoundPoint self, Dipole other) {
    return MultiVector(vec2(0.0), self.g0_, self.g1_, vec3(0.0) - other.g0_, vec3(0.0) - other.g1_, vec4(0.0) - other.g2_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector roundPoint_sub_dualNum(RoundPoint self, DualNum other) {
    return MultiVector(vec2(0.0) - other.g0_, self.g0_, self.g1_, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector roundPoint_sub_flatPoint(RoundPoint self, FlatPoint other) {
    return MultiVector(vec2(0.0), self.g0_, self.g1_, vec3(0.0), vec3(0.0), vec4(0.0) - other.g0_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector roundPoint_sub_flector(RoundPoint self, Flector other) {
    return MultiVector(vec2(0.0), self.g0_, self.g1_, vec3(0.0), vec3(0.0), vec4(0.0) - other.g0_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec2(0.0) - vec2(0.0, other.g1_.w));
}

MultiVector roundPoint_sub_line(RoundPoint self, Line other) {
    return MultiVector(vec2(0.0), self.g0_, self.g1_, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0) - other.g0_, vec3(0.0) - other.g1_, vec3(0.0), vec2(0.0));
}

MultiVector roundPoint_sub_motor(RoundPoint self, Motor other) {
    return MultiVector(vec2(0.0) - vec2(0.0, other.g0_.w), self.g0_, self.g1_, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0) - vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(0.0) - other.g1_, vec3(0.0), vec2(0.0));
}

MultiVector roundPoint_sub_multiVector(RoundPoint self, MultiVector other) {
    return MultiVector(vec2(0.0) - other.g0_, self.g0_ - other.g1_, self.g1_ - other.g2_, vec3(0.0) - other.g3_, vec3(0.0) - other.g4_, vec4(0.0) - other.g5_, vec4(0.0) - other.g6_, vec3(0.0) - other.g7_, vec3(0.0) - other.g8_, vec3(0.0) - other.g9_, vec2(0.0) - other.g10_);
}

MultiVector roundPoint_sub_plane(RoundPoint self, Plane other) {
    return MultiVector(vec2(0.0), self.g0_, self.g1_, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(0.0) - vec2(0.0, other.g0_.w));
}

RoundPoint roundPoint_sub_roundPoint(RoundPoint self, RoundPoint other) {
    return RoundPoint(self.g0_ - other.g0_, self.g1_ - other.g1_);
}

MultiVector roundPoint_sub_scalar(RoundPoint self, Scalar other) {
    return MultiVector(vec2(0.0) - vec2(other.g0_, 0.0), self.g0_, self.g1_, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector roundPoint_sub_sphere(RoundPoint self, Sphere other) {
    return MultiVector(vec2(0.0), self.g0_, self.g1_, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - other.g0_, vec2(0.0) - other.g1_);
}

DualNum scalar_sub_antiScalar(Scalar self, AntiScalar other) {
    return DualNum(vec2(self.g0_, 0.0) - vec2(0.0, other.g0_));
}

MultiVector scalar_sub_circle(Scalar self, Circle other) {
    return MultiVector(vec2(self.g0_, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0) - other.g0_, vec3(0.0) - other.g1_, vec3(0.0) - other.g2_, vec3(0.0), vec2(0.0));
}

MultiVector scalar_sub_dipole(Scalar self, Dipole other) {
    return MultiVector(vec2(self.g0_, 0.0), vec3(0.0), vec2(0.0), vec3(0.0) - other.g0_, vec3(0.0) - other.g1_, vec4(0.0) - other.g2_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

DualNum scalar_sub_dualNum(Scalar self, DualNum other) {
    return DualNum(vec2(self.g0_, 0.0) - other.g0_);
}

MultiVector scalar_sub_flatPoint(Scalar self, FlatPoint other) {
    return MultiVector(vec2(self.g0_, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0) - other.g0_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector scalar_sub_flector(Scalar self, Flector other) {
    return MultiVector(vec2(self.g0_, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0) - other.g0_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec2(0.0) - vec2(0.0, other.g1_.w));
}

MultiVector scalar_sub_line(Scalar self, Line other) {
    return MultiVector(vec2(self.g0_, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0) - other.g0_, vec3(0.0) - other.g1_, vec3(0.0), vec2(0.0));
}

MultiVector scalar_sub_motor(Scalar self, Motor other) {
    return MultiVector(vec2(self.g0_, 0.0) - vec2(0.0, other.g0_.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0) - vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(0.0) - other.g1_, vec3(0.0), vec2(0.0));
}

MultiVector scalar_sub_multiVector(Scalar self, MultiVector other) {
    return MultiVector(vec2(self.g0_, 0.0) - other.g0_, vec3(0.0) - other.g1_, vec2(0.0) - other.g2_, vec3(0.0) - other.g3_, vec3(0.0) - other.g4_, vec4(0.0) - other.g5_, vec4(0.0) - other.g6_, vec3(0.0) - other.g7_, vec3(0.0) - other.g8_, vec3(0.0) - other.g9_, vec2(0.0) - other.g10_);
}

MultiVector scalar_sub_plane(Scalar self, Plane other) {
    return MultiVector(vec2(self.g0_, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(0.0) - vec2(0.0, other.g0_.w));
}

MultiVector scalar_sub_roundPoint(Scalar self, RoundPoint other) {
    return MultiVector(vec2(self.g0_, 0.0), vec3(0.0) - other.g0_, vec2(0.0) - other.g1_, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

Scalar scalar_sub_scalar(Scalar self, Scalar other) {
    return Scalar(self.g0_ - other.g0_);
}

MultiVector scalar_sub_sphere(Scalar self, Sphere other) {
    return MultiVector(vec2(self.g0_, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - other.g0_, vec2(0.0) - other.g1_);
}

MultiVector sphere_sub_antiScalar(Sphere self, AntiScalar other) {
    return MultiVector(vec2(0.0) - vec2(0.0, other.g0_), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), self.g0_, self.g1_);
}

MultiVector sphere_sub_circle(Sphere self, Circle other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0) - other.g0_, vec3(0.0) - other.g1_, vec3(0.0) - other.g2_, self.g0_, self.g1_);
}

MultiVector sphere_sub_dipole(Sphere self, Dipole other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0) - other.g0_, vec3(0.0) - other.g1_, vec4(0.0) - other.g2_, vec4(0.0), vec3(0.0), vec3(0.0), self.g0_, self.g1_);
}

MultiVector sphere_sub_dualNum(Sphere self, DualNum other) {
    return MultiVector(vec2(0.0) - other.g0_, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), self.g0_, self.g1_);
}

MultiVector sphere_sub_flatPoint(Sphere self, FlatPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0) - other.g0_, vec4(0.0), vec3(0.0), vec3(0.0), self.g0_, self.g1_);
}

MultiVector sphere_sub_flector(Sphere self, Flector other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0) - other.g0_, vec4(0.0), vec3(0.0), vec3(0.0), self.g0_ - vec3(other.g1_.x, other.g1_.y, other.g1_.z), self.g1_ - vec2(0.0, other.g1_.w));
}

MultiVector sphere_sub_line(Sphere self, Line other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0) - other.g0_, vec3(0.0) - other.g1_, self.g0_, self.g1_);
}

MultiVector sphere_sub_motor(Sphere self, Motor other) {
    return MultiVector(vec2(0.0) - vec2(0.0, other.g0_.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0) - vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(0.0) - other.g1_, self.g0_, self.g1_);
}

MultiVector sphere_sub_multiVector(Sphere self, MultiVector other) {
    return MultiVector(vec2(0.0) - other.g0_, vec3(0.0) - other.g1_, vec2(0.0) - other.g2_, vec3(0.0) - other.g3_, vec3(0.0) - other.g4_, vec4(0.0) - other.g5_, vec4(0.0) - other.g6_, vec3(0.0) - other.g7_, vec3(0.0) - other.g8_, self.g0_ - other.g9_, self.g1_ - other.g10_);
}

Sphere sphere_sub_plane(Sphere self, Plane other) {
    return Sphere(self.g0_ - vec3(other.g0_.x, other.g0_.y, other.g0_.z), self.g1_ - vec2(0.0, other.g0_.w));
}

MultiVector sphere_sub_roundPoint(Sphere self, RoundPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0) - other.g0_, vec2(0.0) - other.g1_, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), self.g0_, self.g1_);
}

MultiVector sphere_sub_scalar(Sphere self, Scalar other) {
    return MultiVector(vec2(0.0) - vec2(other.g0_, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), self.g0_, self.g1_);
}

Sphere sphere_sub_sphere(Sphere self, Sphere other) {
    return Sphere(self.g0_ - other.g0_, self.g1_ - other.g1_);
}

AntiScalar antiScalar_antiWedgeDot_antiScalar(AntiScalar self, AntiScalar other) {
    return AntiScalar(self.g0_ * other.g0_);
}

Circle antiScalar_antiWedgeDot_circle(AntiScalar self, Circle other) {
    return Circle(vec4(self.g0_) * other.g0_, vec3(self.g0_) * other.g1_, vec3(self.g0_) * other.g2_);
}

Dipole antiScalar_antiWedgeDot_dipole(AntiScalar self, Dipole other) {
    return Dipole(vec3(self.g0_) * other.g0_, vec3(self.g0_) * other.g1_, vec4(self.g0_) * other.g2_);
}

DualNum antiScalar_antiWedgeDot_dualNum(AntiScalar self, DualNum other) {
    return DualNum(vec2(self.g0_) * other.g0_);
}

FlatPoint antiScalar_antiWedgeDot_flatPoint(AntiScalar self, FlatPoint other) {
    return FlatPoint(vec4(self.g0_) * other.g0_);
}

Flector antiScalar_antiWedgeDot_flector(AntiScalar self, Flector other) {
    return Flector(vec4(self.g0_) * other.g0_, vec4(self.g0_) * other.g1_);
}

Line antiScalar_antiWedgeDot_line(AntiScalar self, Line other) {
    return Line(vec3(self.g0_) * other.g0_, vec3(self.g0_) * other.g1_);
}

Motor antiScalar_antiWedgeDot_motor(AntiScalar self, Motor other) {
    return Motor(vec4(self.g0_) * other.g0_, vec3(self.g0_) * other.g1_);
}

MultiVector antiScalar_antiWedgeDot_multiVector(AntiScalar self, MultiVector other) {
    return MultiVector(vec2(self.g0_) * other.g0_, vec3(self.g0_) * other.g1_, vec2(self.g0_) * other.g2_, vec3(self.g0_) * other.g3_, vec3(self.g0_) * other.g4_, vec4(self.g0_) * other.g5_, vec4(self.g0_) * other.g6_, vec3(self.g0_) * other.g7_, vec3(self.g0_) * other.g8_, vec3(self.g0_) * other.g9_, vec2(self.g0_) * other.g10_);
}

Plane antiScalar_antiWedgeDot_plane(AntiScalar self, Plane other) {
    return Plane(vec4(self.g0_) * other.g0_);
}

RoundPoint antiScalar_antiWedgeDot_roundPoint(AntiScalar self, RoundPoint other) {
    return RoundPoint(vec3(self.g0_) * other.g0_, vec2(self.g0_) * other.g1_);
}

Scalar antiScalar_antiWedgeDot_scalar(AntiScalar self, Scalar other) {
    return Scalar(self.g0_ * other.g0_);
}

Sphere antiScalar_antiWedgeDot_sphere(AntiScalar self, Sphere other) {
    return Sphere(vec3(self.g0_) * other.g0_, vec2(self.g0_) * other.g1_);
}

Circle circle_antiWedgeDot_antiScalar(Circle self, AntiScalar other) {
    return Circle(self.g0_ * vec4(other.g0_), self.g1_ * vec3(other.g0_), self.g2_ * vec3(other.g0_));
}

MultiVector circle_antiWedgeDot_circle(Circle self, Circle other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, -other.g2_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g2_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g2_.z) + vec2(self.g0_.w) * vec2(0.0, other.g0_.w) + vec2(self.g1_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g2_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g2_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g2_.z) * vec2(0.0, -other.g0_.z), vec3(self.g0_.x) * vec3(0.0, other.g2_.z, -other.g2_.y) + vec3(self.g0_.y) * vec3(-other.g2_.z, 0.0, other.g2_.x) + vec3(self.g0_.z) * vec3(other.g2_.y, -other.g2_.x, 0.0) + vec3(self.g0_.w) * other.g1_ + self.g1_ * vec3(other.g0_.w) + vec3(self.g2_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g2_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g2_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec2(self.g0_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g1_.z, 0.0) - vec2(self.g1_.x) * vec2(other.g0_.x, other.g2_.x) - vec2(self.g1_.y) * vec2(other.g0_.y, other.g2_.y) - vec2(self.g1_.z) * vec2(other.g0_.z, other.g2_.z) + vec2(self.g2_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g2_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g2_.z) * vec2(0.0, -other.g1_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(other.g0_.w, -other.g1_.z, other.g1_.y, other.g2_.x) + vec4(self.g0_.y) * vec4(other.g1_.z, other.g0_.w, -other.g1_.x, other.g2_.y) + vec4(self.g0_.z) * vec4(-other.g1_.y, other.g1_.x, other.g0_.w, other.g2_.z) + vec4(self.g0_.w) * vec4(-other.g0_.x, -other.g0_.y, -other.g0_.z, 0.0) + vec4(self.g1_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g1_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g1_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0) + vec4(self.g2_.x) * vec4(0.0, 0.0, 0.0, -other.g0_.x) + vec4(self.g2_.y) * vec4(0.0, 0.0, 0.0, -other.g0_.y) + vec4(self.g2_.z) * vec4(0.0, 0.0, 0.0, -other.g0_.z), vec3(self.g0_.x) * vec3(0.0, -other.g2_.z, other.g2_.y) + vec3(self.g0_.y) * vec3(other.g2_.z, 0.0, -other.g2_.x) + vec3(self.g0_.z) * vec3(-other.g2_.y, other.g2_.x, 0.0) + vec3(self.g1_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g1_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g1_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g2_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g2_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g2_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(self.g0_.w) * other.g2_ + vec3(self.g1_.x) * vec3(0.0, -other.g2_.z, other.g2_.y) + vec3(self.g1_.y) * vec3(other.g2_.z, 0.0, -other.g2_.x) + vec3(self.g1_.z) * vec3(-other.g2_.y, other.g2_.x, 0.0) + vec3(self.g2_.x) * vec3(-other.g0_.w, -other.g1_.z, other.g1_.y) + vec3(self.g2_.y) * vec3(other.g1_.z, -other.g0_.w, -other.g1_.x) + vec3(self.g2_.z) * vec3(-other.g1_.y, other.g1_.x, -other.g0_.w), vec3(0.0), vec2(0.0));
}

MultiVector circle_antiWedgeDot_dipole(Circle self, Dipole other) {
    return MultiVector(vec2(self.g0_.x) * vec2(-other.g2_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g2_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g2_.z, 0.0) + vec2(self.g0_.w) * vec2(-other.g2_.w, 0.0) + vec2(self.g1_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g1_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g1_.z) * vec2(-other.g1_.z, 0.0) + vec2(self.g2_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g2_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g2_.z) * vec2(-other.g0_.z, 0.0), vec3(0.0), vec2(0.0), vec3(self.g0_.x) * vec3(-other.g2_.w, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, -other.g2_.w, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, -other.g2_.w) - vec3(self.g0_.w) * other.g0_ + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(self.g0_.x) * vec3(0.0, -other.g2_.z, other.g2_.y) + vec3(self.g0_.y) * vec3(other.g2_.z, 0.0, -other.g2_.x) + vec3(self.g0_.z) * vec3(-other.g2_.y, other.g2_.x, 0.0) + vec3(self.g1_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g1_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g1_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g2_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g2_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g2_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, -other.g2_.x) + vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, -other.g2_.y) + self.g0_.wwwz * other.g2_.xyzz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g1_.x) * vec4(0.0, -other.g2_.z, other.g2_.y, 0.0) + vec4(self.g1_.y) * vec4(other.g2_.z, 0.0, -other.g2_.x, 0.0) + vec4(self.g1_.z) * vec4(-other.g2_.y, other.g2_.x, 0.0, 0.0) + vec4(self.g2_.x) * vec4(other.g2_.w, -other.g1_.z, other.g1_.y, other.g0_.x) + vec4(self.g2_.y) * vec4(other.g1_.z, other.g2_.w, -other.g1_.x, other.g0_.y) + vec4(self.g2_.z) * vec4(-other.g1_.y, other.g1_.x, other.g2_.w, other.g0_.z), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g2_.z, other.g2_.y) + vec3(self.g0_.y) * vec3(other.g2_.z, 0.0, -other.g2_.x) + vec3(self.g0_.z) * vec3(-other.g2_.y, other.g2_.x, 0.0) - vec3(self.g0_.w) * other.g1_ + self.g1_ * vec3(other.g2_.w) + vec3(self.g2_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g2_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g2_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec2(self.g0_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g1_.z, 0.0) - vec2(self.g1_.x) * vec2(other.g0_.x, other.g2_.x) - vec2(self.g1_.y) * vec2(other.g0_.y, other.g2_.y) - vec2(self.g1_.z) * vec2(other.g0_.z, other.g2_.z) + vec2(self.g2_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g2_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g2_.z) * vec2(0.0, -other.g1_.z));
}

MultiVector circle_antiWedgeDot_dualNum(Circle self, DualNum other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_.x), self.g1_ * vec3(other.g0_.x), vec4(self.g0_.w) * vec4(0.0, 0.0, 0.0, -other.g0_.x) + vec4(self.g2_.x, self.g2_.y, self.g2_.z, self.g2_.x) * vec4(other.g0_.x, other.g0_.x, other.g0_.x, 0.0), self.g0_ * vec4(other.g0_.y), self.g1_ * vec3(other.g0_.y), self.g2_ * vec3(other.g0_.y), vec3(0.0), vec2(0.0));
}

MultiVector circle_antiWedgeDot_flatPoint(Circle self, FlatPoint other) {
    return MultiVector(vec2(self.g0_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g0_.w) * vec2(-other.g0_.w, 0.0), vec3(0.0), vec2(0.0), vec3(0.0) - vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_.w), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, -other.g0_.x) + vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, -other.g0_.y) + self.g0_.wwwz * other.g0_.xyzz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g1_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g1_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g1_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0) + vec4(self.g2_.x, self.g2_.y, self.g2_.z, self.g2_.x) * vec4(other.g0_.w, other.g0_.w, other.g0_.w, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) + self.g1_ * vec3(other.g0_.w), vec2(self.g1_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g0_.z));
}

MultiVector circle_antiWedgeDot_flector(Circle self, Flector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g0_.w) * vec2(-other.g0_.w, 0.0), vec3(0.0), vec2(0.0), vec3(self.g0_.x) * vec3(-other.g0_.w, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, -other.g0_.w, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, -other.g0_.w), vec3(self.g0_.x) * vec3(other.g1_.w, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, other.g1_.w, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, other.g1_.w) - vec3(self.g0_.w) * vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, -other.g0_.x) + vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, -other.g0_.y) + self.g0_.wwwz * other.g0_.xyzz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g1_.x) * vec4(other.g1_.w, -other.g0_.z, other.g0_.y, -other.g1_.x) + vec4(self.g1_.y) * vec4(other.g0_.z, other.g1_.w, -other.g0_.x, -other.g1_.y) + vec4(self.g1_.z) * vec4(-other.g0_.y, other.g0_.x, other.g1_.w, -other.g1_.z) + vec4(self.g2_.x) * vec4(other.g0_.w, -other.g1_.z, other.g1_.y, 0.0) + vec4(self.g2_.y) * vec4(other.g1_.z, other.g0_.w, -other.g1_.x, 0.0) + vec4(self.g2_.z) * vec4(-other.g1_.y, other.g1_.x, other.g0_.w, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x) * vec3(other.g1_.w, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, other.g1_.w, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, other.g1_.w) + vec3(self.g1_.x) * vec3(other.g0_.w, -other.g1_.z, other.g1_.y) + vec3(self.g1_.y) * vec3(other.g1_.z, other.g0_.w, -other.g1_.x) + vec3(self.g1_.z) * vec3(-other.g1_.y, other.g1_.x, other.g0_.w), vec2(self.g0_.x) * vec2(other.g1_.x, 0.0) + vec2(self.g0_.y) * vec2(other.g1_.y, 0.0) + vec2(self.g0_.z, self.g0_.w) * vec2(other.g1_.z, other.g1_.w) + vec2(self.g1_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g2_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g2_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g2_.z) * vec2(0.0, -other.g1_.z));
}

MultiVector circle_antiWedgeDot_line(Circle self, Line other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g1_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g0_.z), vec3(self.g0_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g0_.w) * other.g0_, vec2(self.g0_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g1_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g2_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g2_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g2_.z) * vec2(0.0, -other.g0_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, other.g1_.x) + vec4(self.g0_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, other.g1_.y) + vec4(self.g0_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, other.g1_.z), vec3(self.g0_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(self.g0_.w) * other.g1_ + vec3(self.g1_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g1_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g1_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g2_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g2_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g2_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(0.0), vec2(0.0));
}

MultiVector circle_antiWedgeDot_motor(Circle self, Motor other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g1_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g0_.z), vec3(self.g0_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g0_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g0_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g1_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g2_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g2_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g2_.z) * vec2(0.0, -other.g0_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(other.g0_.w, -other.g0_.z, other.g0_.y, other.g1_.x) + vec4(self.g0_.y) * vec4(other.g0_.z, other.g0_.w, -other.g0_.x, other.g1_.y) + vec4(self.g0_.z) * vec4(-other.g0_.y, other.g0_.x, other.g0_.w, other.g1_.z) + vec4(self.g0_.w) * vec4(0.0, 0.0, 0.0, other.g0_.w), vec3(self.g0_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g1_.x) * vec3(other.g0_.w, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, other.g0_.w, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, other.g0_.w), vec3(self.g0_.w) * other.g1_ + vec3(self.g1_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g1_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g1_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g2_.x) * vec3(other.g0_.w, -other.g0_.z, other.g0_.y) + vec3(self.g2_.y) * vec3(other.g0_.z, other.g0_.w, -other.g0_.x) + vec3(self.g2_.z) * vec3(-other.g0_.y, other.g0_.x, other.g0_.w), vec3(0.0), vec2(0.0));
}

MultiVector circle_antiWedgeDot_multiVector(Circle self, MultiVector other) {
    return MultiVector(vec2(0.0) - vec2(self.g0_.x) * vec2(other.g5_.x, other.g8_.x) - vec2(self.g0_.y) * vec2(other.g5_.y, other.g8_.y) - vec2(self.g0_.z) * vec2(other.g5_.z, other.g8_.z) + vec2(self.g0_.w) * vec2(-other.g5_.w, other.g6_.w) - vec2(self.g1_.x) * vec2(other.g4_.x, other.g7_.x) - vec2(self.g1_.y) * vec2(other.g4_.y, other.g7_.y) - vec2(self.g1_.z) * vec2(other.g4_.z, other.g7_.z) - vec2(self.g2_.x) * vec2(other.g3_.x, other.g6_.x) - vec2(self.g2_.y) * vec2(other.g3_.y, other.g6_.y) - vec2(self.g2_.z) * vec2(other.g3_.z, other.g6_.z), vec3(self.g0_.x) * vec3(-other.g2_.y, other.g8_.z, -other.g8_.y) + vec3(self.g0_.y) * vec3(-other.g8_.z, -other.g2_.y, other.g8_.x) + vec3(self.g0_.z) * vec3(other.g8_.y, -other.g8_.x, -other.g2_.y) + vec3(self.g0_.w) * other.g7_ + vec3(self.g1_.x) * vec3(other.g6_.w, -other.g1_.z, other.g1_.y) + vec3(self.g1_.y) * vec3(other.g1_.z, other.g6_.w, -other.g1_.x) + vec3(self.g1_.z) * vec3(-other.g1_.y, other.g1_.x, other.g6_.w) + vec3(self.g2_.x) * vec3(other.g2_.x, -other.g6_.z, other.g6_.y) + vec3(self.g2_.y) * vec3(other.g6_.z, other.g2_.x, -other.g6_.x) + vec3(self.g2_.z) * vec3(-other.g6_.y, other.g6_.x, other.g2_.x), vec2(self.g0_.x) * vec2(-other.g7_.x, 0.0) + vec2(self.g0_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g7_.y, 0.0) + vec2(self.g0_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g7_.z, 0.0) + vec2(self.g0_.z) * vec2(-other.g1_.z, 0.0) + vec2(self.g0_.w) * other.g2_ * vec2(-1.0, 1.0) - vec2(self.g1_.x) * vec2(other.g6_.x, other.g8_.x) - vec2(self.g1_.y) * vec2(other.g6_.y, other.g8_.y) - vec2(self.g1_.z) * vec2(other.g6_.z, other.g8_.z) + vec2(self.g2_.x) * vec2(0.0, -other.g7_.x) + vec2(self.g2_.x) * vec2(0.0, other.g1_.x) + vec2(self.g2_.y) * vec2(0.0, -other.g7_.y) + vec2(self.g2_.y) * vec2(0.0, other.g1_.y) + vec2(self.g2_.z) * vec2(0.0, -other.g7_.z) + vec2(self.g2_.z) * vec2(0.0, other.g1_.z), vec3(self.g0_.x) * vec3(-other.g5_.w, other.g9_.z, -other.g9_.y) + vec3(self.g0_.x) * vec3(other.g0_.x, -other.g4_.z, other.g4_.y) + vec3(self.g0_.y) * vec3(-other.g9_.z, -other.g5_.w, other.g9_.x) + vec3(self.g0_.y) * vec3(other.g4_.z, other.g0_.x, -other.g4_.x) + vec3(self.g0_.z) * vec3(other.g9_.y, -other.g9_.x, -other.g5_.w) + vec3(self.g0_.z) * vec3(-other.g4_.y, other.g4_.x, other.g0_.x) - vec3(self.g0_.w) * other.g3_ + vec3(self.g1_.x) * vec3(other.g10_.x, -other.g3_.z, other.g3_.y) + vec3(self.g1_.y) * vec3(other.g3_.z, other.g10_.x, -other.g3_.x) + vec3(self.g1_.z) * vec3(-other.g3_.y, other.g3_.x, other.g10_.x), vec3(self.g0_.x) * vec3(other.g10_.y, -other.g5_.z, other.g5_.y) + vec3(self.g0_.y) * vec3(other.g5_.z, other.g10_.y, -other.g5_.x) + vec3(self.g0_.z) * vec3(-other.g5_.y, other.g5_.x, other.g10_.y) - vec3(self.g0_.w) * other.g9_ + vec3(self.g1_.x) * vec3(other.g0_.x, -other.g4_.z, other.g4_.y) + vec3(self.g1_.y) * vec3(other.g4_.z, other.g0_.x, -other.g4_.x) + vec3(self.g1_.z) * vec3(-other.g4_.y, other.g4_.x, other.g0_.x) + vec3(self.g2_.x) * vec3(other.g10_.x, -other.g3_.z, other.g3_.y) + vec3(self.g2_.y) * vec3(other.g3_.z, other.g10_.x, -other.g3_.x) + vec3(self.g2_.z) * vec3(-other.g3_.y, other.g3_.x, other.g10_.x), vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, -other.g5_.x) + vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, -other.g5_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, 0.0, -other.g5_.z) + vec4(self.g0_.w) * vec4(other.g5_.x, other.g5_.y, other.g5_.z, -other.g0_.x) + vec4(self.g1_.x) * vec4(other.g10_.y, -other.g5_.z, other.g5_.y, -other.g9_.x) + vec4(self.g1_.y) * vec4(other.g5_.z, other.g10_.y, -other.g5_.x, -other.g9_.y) + vec4(self.g1_.z) * vec4(-other.g5_.y, other.g5_.x, other.g10_.y, -other.g9_.z) + vec4(self.g2_.x) * vec4(other.g5_.w, -other.g9_.z, other.g9_.y, other.g3_.x) + vec4(self.g2_.x) * vec4(other.g0_.x, -other.g4_.z, other.g4_.y, 0.0) + vec4(self.g2_.y) * vec4(other.g9_.z, other.g5_.w, -other.g9_.x, other.g3_.y) + vec4(self.g2_.y) * vec4(other.g4_.z, other.g0_.x, -other.g4_.x, 0.0) + vec4(self.g2_.z) * vec4(-other.g9_.y, other.g9_.x, other.g5_.w, other.g3_.z) + vec4(self.g2_.z) * vec4(-other.g4_.y, other.g4_.x, other.g0_.x, 0.0), vec4(self.g0_.x) * vec4(other.g6_.w, -other.g7_.z, other.g7_.y, other.g8_.x) + vec4(self.g0_.x) * vec4(other.g0_.y, -other.g1_.z, other.g1_.y, 0.0) + vec4(self.g0_.y) * vec4(other.g7_.z, other.g6_.w, -other.g7_.x, other.g8_.y) + vec4(self.g0_.y) * vec4(other.g1_.z, other.g0_.y, -other.g1_.x, 0.0) + vec4(self.g0_.z) * vec4(-other.g7_.y, other.g7_.x, other.g6_.w, other.g8_.z) + vec4(self.g0_.z) * vec4(-other.g1_.y, other.g1_.x, other.g0_.y, 0.0) + vec4(self.g0_.w) * vec4(-other.g6_.x, -other.g6_.y, -other.g6_.z, other.g0_.y) + vec4(self.g1_.x) * vec4(other.g2_.x, -other.g6_.z, other.g6_.y, -other.g1_.x) + vec4(self.g1_.y) * vec4(other.g6_.z, other.g2_.x, -other.g6_.x, -other.g1_.y) + vec4(self.g1_.z) * vec4(-other.g6_.y, other.g6_.x, other.g2_.x, -other.g1_.z) + vec4(self.g2_.x) * vec4(0.0, 0.0, 0.0, -other.g6_.x) + vec4(self.g2_.y) * vec4(0.0, 0.0, 0.0, -other.g6_.y) + vec4(self.g2_.z) * vec4(0.0, 0.0, 0.0, -other.g6_.z), vec3(self.g0_.x) * vec3(other.g2_.y, -other.g8_.z, other.g8_.y) + vec3(self.g0_.y) * vec3(other.g8_.z, other.g2_.y, -other.g8_.x) + vec3(self.g0_.z) * vec3(-other.g8_.y, other.g8_.x, other.g2_.y) + vec3(self.g0_.w) * other.g1_ + vec3(self.g1_.x) * vec3(other.g0_.y, -other.g7_.z, other.g7_.y) + vec3(self.g1_.y) * vec3(other.g7_.z, other.g0_.y, -other.g7_.x) + vec3(self.g1_.z) * vec3(-other.g7_.y, other.g7_.x, other.g0_.y) + vec3(self.g2_.x) * vec3(other.g2_.x, -other.g6_.z, other.g6_.y) + vec3(self.g2_.y) * vec3(other.g6_.z, other.g2_.x, -other.g6_.x) + vec3(self.g2_.z) * vec3(-other.g6_.y, other.g6_.x, other.g2_.x), vec3(self.g0_.w) * other.g8_ + vec3(self.g1_.x) * vec3(other.g2_.y, -other.g8_.z, other.g8_.y) + vec3(self.g1_.y) * vec3(other.g8_.z, other.g2_.y, -other.g8_.x) + vec3(self.g1_.z) * vec3(-other.g8_.y, other.g8_.x, other.g2_.y) + vec3(self.g2_.x) * vec3(-other.g6_.w, -other.g7_.z, other.g7_.y) + vec3(self.g2_.x) * vec3(other.g0_.y, other.g1_.z, -other.g1_.y) + vec3(self.g2_.y) * vec3(other.g7_.z, -other.g6_.w, -other.g7_.x) + vec3(self.g2_.y) * vec3(-other.g1_.z, other.g0_.y, other.g1_.x) + vec3(self.g2_.z) * vec3(-other.g7_.y, other.g7_.x, -other.g6_.w) + vec3(self.g2_.z) * vec3(other.g1_.y, -other.g1_.x, other.g0_.y), vec3(self.g0_.x) * vec3(other.g10_.y, -other.g5_.z, other.g5_.y) + vec3(self.g0_.y) * vec3(other.g5_.z, other.g10_.y, -other.g5_.x) + vec3(self.g0_.z) * vec3(-other.g5_.y, other.g5_.x, other.g10_.y) - vec3(self.g0_.w) * other.g4_ + vec3(self.g1_.x) * vec3(other.g5_.w, -other.g9_.z, other.g9_.y) + vec3(self.g1_.y) * vec3(other.g9_.z, other.g5_.w, -other.g9_.x) + vec3(self.g1_.z) * vec3(-other.g9_.y, other.g9_.x, other.g5_.w) + vec3(self.g2_.x) * vec3(-other.g10_.x, other.g3_.z, -other.g3_.y) + vec3(self.g2_.y) * vec3(-other.g3_.z, -other.g10_.x, other.g3_.x) + vec3(self.g2_.z) * vec3(other.g3_.y, -other.g3_.x, -other.g10_.x), vec2(self.g0_.x) * vec2(other.g9_.x, 0.0) + vec2(self.g0_.x) * vec2(-other.g4_.x, 0.0) + vec2(self.g0_.y) * vec2(other.g9_.y, 0.0) + vec2(self.g0_.y) * vec2(-other.g4_.y, 0.0) + vec2(self.g0_.z) * vec2(other.g9_.z, 0.0) + vec2(self.g0_.z) * vec2(-other.g4_.z, 0.0) + vec2(self.g0_.w) * other.g10_ * vec2(-1.0, 1.0) - vec2(self.g1_.x) * vec2(other.g3_.x, other.g5_.x) - vec2(self.g1_.y) * vec2(other.g3_.y, other.g5_.y) - vec2(self.g1_.z) * vec2(other.g3_.z, other.g5_.z) + vec2(self.g2_.x) * vec2(0.0, -other.g9_.x) + vec2(self.g2_.x) * vec2(0.0, -other.g4_.x) + vec2(self.g2_.y) * vec2(0.0, -other.g9_.y) + vec2(self.g2_.y) * vec2(0.0, -other.g4_.y) + vec2(self.g2_.z) * vec2(0.0, -other.g9_.z) + vec2(self.g2_.z) * vec2(0.0, -other.g4_.z));
}

MultiVector circle_antiWedgeDot_plane(Circle self, Plane other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_.w) - vec3(self.g0_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec4(self.g1_.x) * vec4(other.g0_.w, 0.0, 0.0, -other.g0_.x) + vec4(self.g1_.y) * vec4(0.0, other.g0_.w, 0.0, -other.g0_.y) + vec4(self.g1_.z) * vec4(0.0, 0.0, other.g0_.w, -other.g0_.z) + vec4(self.g2_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g2_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g2_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_.w) + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec2(self.g0_.x) * vec2(other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(other.g0_.y, 0.0) + vec2(self.g0_.z, self.g0_.w) * vec2(other.g0_.z, other.g0_.w) + vec2(self.g2_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g2_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g2_.z) * vec2(0.0, -other.g0_.z));
}

MultiVector circle_antiWedgeDot_roundPoint(Circle self, RoundPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0) - vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g1_.y) + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) + self.g2_ * vec3(other.g1_.x), vec2(self.g0_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g0_.w) * other.g1_ * vec2(-1.0, 1.0) + vec2(self.g2_.x) * vec2(0.0, other.g0_.x) + vec2(self.g2_.y) * vec2(0.0, other.g0_.y) + vec2(self.g2_.z) * vec2(0.0, other.g0_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g0_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g0_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0) + vec4(self.g1_.x) * vec4(other.g1_.x, 0.0, 0.0, -other.g0_.x) + vec4(self.g1_.y) * vec4(0.0, other.g1_.x, 0.0, -other.g0_.y) + vec4(self.g1_.z) * vec4(0.0, 0.0, other.g1_.x, -other.g0_.z), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g1_.y) + vec3(self.g0_.w) * other.g0_ + self.g2_ * vec3(other.g1_.x), self.g1_ * vec3(other.g1_.y) + vec3(self.g2_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g2_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g2_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec3(0.0), vec2(0.0));
}

Dipole circle_antiWedgeDot_scalar(Circle self, Scalar other) {
    return Dipole(vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_), self.g1_ * vec3(other.g0_), vec4(self.g0_.w) * vec4(0.0, 0.0, 0.0, -other.g0_) + vec4(self.g2_.x, self.g2_.y, self.g2_.z, self.g2_.x) * vec4(other.g0_, other.g0_, other.g0_, 0.0));
}

MultiVector circle_antiWedgeDot_sphere(Circle self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + self.g1_ * vec3(other.g1_.x), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g1_.y) - vec3(self.g0_.w) * other.g0_ + self.g2_ * vec3(other.g1_.x), vec4(self.g1_.x) * vec4(other.g1_.y, 0.0, 0.0, -other.g0_.x) + vec4(self.g1_.y) * vec4(0.0, other.g1_.y, 0.0, -other.g0_.y) + vec4(self.g1_.z) * vec4(0.0, 0.0, other.g1_.y, -other.g0_.z) + vec4(self.g2_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g2_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g2_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g1_.y) + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) - self.g2_ * vec3(other.g1_.x), vec2(self.g0_.x) * vec2(other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(other.g0_.z, 0.0) + vec2(self.g0_.w) * other.g1_ * vec2(-1.0, 1.0) + vec2(self.g2_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g2_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g2_.z) * vec2(0.0, -other.g0_.z));
}

Dipole dipole_antiWedgeDot_antiScalar(Dipole self, AntiScalar other) {
    return Dipole(self.g0_ * vec3(other.g0_), self.g1_ * vec3(other.g0_), self.g2_ * vec4(other.g0_));
}

MultiVector dipole_antiWedgeDot_circle(Dipole self, Circle other) {
    return MultiVector(vec2(self.g0_.x) * vec2(-other.g2_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g2_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g2_.z, 0.0) + vec2(self.g1_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g1_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g1_.z) * vec2(-other.g1_.z, 0.0) + vec2(self.g2_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g2_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g2_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g2_.w) * vec2(-other.g0_.w, 0.0), vec3(0.0), vec2(0.0), vec3(self.g0_.x) * vec3(other.g0_.w, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, other.g0_.w, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, other.g0_.w) + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) + vec3(self.g2_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(self.g0_.x) * vec3(0.0, -other.g2_.z, other.g2_.y) + vec3(self.g0_.y) * vec3(other.g2_.z, 0.0, -other.g2_.x) + vec3(self.g0_.z) * vec3(-other.g2_.y, other.g2_.x, 0.0) + vec3(self.g1_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g1_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g1_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g2_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g2_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g2_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, -other.g2_.x) + vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, -other.g2_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, 0.0, -other.g2_.z) + vec4(self.g1_.x) * vec4(0.0, -other.g2_.z, other.g2_.y, 0.0) + vec4(self.g1_.y) * vec4(other.g2_.z, 0.0, -other.g2_.x, 0.0) + vec4(self.g1_.z) * vec4(-other.g2_.y, other.g2_.x, 0.0, 0.0) + vec4(self.g2_.x) * vec4(-other.g0_.w, -other.g1_.z, other.g1_.y, other.g0_.x) + vec4(self.g2_.y) * vec4(other.g1_.z, -other.g0_.w, -other.g1_.x, other.g0_.y) + vec4(self.g2_.z) * vec4(-other.g1_.y, other.g1_.x, -other.g0_.w, other.g0_.z) + vec4(self.g2_.w) * vec4(-other.g2_.x, -other.g2_.y, -other.g2_.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g2_.z, other.g2_.y) + vec3(self.g0_.y) * vec3(other.g2_.z, 0.0, -other.g2_.x) + vec3(self.g0_.z) * vec3(-other.g2_.y, other.g2_.x, 0.0) - self.g1_ * vec3(other.g0_.w) + vec3(self.g2_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g2_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g2_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + vec3(self.g2_.w) * other.g1_, vec2(self.g0_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g1_.z, 0.0) - vec2(self.g1_.x) * vec2(other.g0_.x, other.g2_.x) - vec2(self.g1_.y) * vec2(other.g0_.y, other.g2_.y) - vec2(self.g1_.z) * vec2(other.g0_.z, other.g2_.z) + vec2(self.g2_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g2_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g2_.z) * vec2(0.0, -other.g1_.z));
}

MultiVector dipole_antiWedgeDot_dipole(Dipole self, Dipole other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, other.g2_.x) + vec2(self.g0_.y) * vec2(0.0, other.g2_.y) + vec2(self.g0_.z) * vec2(0.0, other.g2_.z) + vec2(self.g1_.x) * vec2(0.0, other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, other.g1_.z) + vec2(self.g2_.x) * vec2(0.0, other.g0_.x) + vec2(self.g2_.y) * vec2(0.0, other.g0_.y) + vec2(self.g2_.z) * vec2(0.0, other.g0_.z) + vec2(self.g2_.w) * vec2(0.0, -other.g2_.w), vec3(self.g0_.x) * vec3(0.0, -other.g2_.z, other.g2_.y) + vec3(self.g0_.y) * vec3(other.g2_.z, 0.0, -other.g2_.x) + vec3(self.g0_.z) * vec3(-other.g2_.y, other.g2_.x, 0.0) + self.g1_ * vec3(other.g2_.w) + vec3(self.g2_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g2_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g2_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + vec3(self.g2_.w) * other.g1_, vec2(self.g0_.x) * vec2(other.g1_.x, 0.0) + vec2(self.g0_.y) * vec2(other.g1_.y, 0.0) + vec2(self.g0_.z) * vec2(other.g1_.z, 0.0) + vec2(self.g1_.x) * vec2(other.g0_.x, other.g2_.x) + vec2(self.g1_.y) * vec2(other.g0_.y, other.g2_.y) + vec2(self.g1_.z) * vec2(other.g0_.z, other.g2_.z) + vec2(self.g2_.x) * vec2(0.0, other.g1_.x) + vec2(self.g2_.y) * vec2(0.0, other.g1_.y) + vec2(self.g2_.z) * vec2(0.0, other.g1_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(other.g2_.w, other.g1_.z, -other.g1_.y, -other.g2_.x) + vec4(self.g0_.y) * vec4(-other.g1_.z, other.g2_.w, other.g1_.x, -other.g2_.y) + vec4(self.g0_.z) * vec4(other.g1_.y, -other.g1_.x, other.g2_.w, -other.g2_.z) + vec4(self.g1_.x) * vec4(0.0, other.g0_.z, -other.g0_.y, 0.0) + vec4(self.g1_.y) * vec4(-other.g0_.z, 0.0, other.g0_.x, 0.0) + vec4(self.g1_.z) * vec4(other.g0_.y, -other.g0_.x, 0.0, 0.0) + vec4(self.g2_.x) * vec4(0.0, 0.0, 0.0, other.g0_.x) + vec4(self.g2_.y) * vec4(0.0, 0.0, 0.0, other.g0_.y) + self.g2_.wwwz * vec4(-other.g0_.x, -other.g0_.y, -other.g0_.z, other.g0_.z), vec3(self.g0_.x) * vec3(0.0, other.g2_.z, -other.g2_.y) + vec3(self.g0_.y) * vec3(-other.g2_.z, 0.0, other.g2_.x) + vec3(self.g0_.z) * vec3(other.g2_.y, -other.g2_.x, 0.0) + vec3(self.g1_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g1_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g1_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g2_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g2_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g2_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec3(self.g1_.x) * vec3(0.0, other.g2_.z, -other.g2_.y) + vec3(self.g1_.y) * vec3(-other.g2_.z, 0.0, other.g2_.x) + vec3(self.g1_.z) * vec3(other.g2_.y, -other.g2_.x, 0.0) + vec3(self.g2_.x) * vec3(-other.g2_.w, other.g1_.z, -other.g1_.y) + vec3(self.g2_.y) * vec3(-other.g1_.z, -other.g2_.w, other.g1_.x) + vec3(self.g2_.z) * vec3(other.g1_.y, -other.g1_.x, -other.g2_.w) + vec3(self.g2_.w) * vec3(other.g2_.x, other.g2_.y, other.g2_.z), vec3(0.0), vec2(0.0));
}

MultiVector dipole_antiWedgeDot_dualNum(Dipole self, DualNum other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), self.g0_ * vec3(other.g0_.y), self.g1_ * vec3(other.g0_.y), self.g2_ * vec4(other.g0_.y), vec4(self.g0_.x, self.g0_.y, self.g0_.z, self.g0_.x) * vec4(-other.g0_.x, -other.g0_.x, -other.g0_.x, 0.0) + vec4(self.g2_.w) * vec4(0.0, 0.0, 0.0, other.g0_.x), vec3(0.0) - self.g1_ * vec3(other.g0_.x), vec3(0.0) - vec3(self.g2_.x, self.g2_.y, self.g2_.z) * vec3(other.g0_.x), vec3(0.0), vec2(0.0));
}

MultiVector dipole_antiWedgeDot_flatPoint(Dipole self, FlatPoint other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, other.g0_.z) + vec2(self.g2_.w) * vec2(0.0, -other.g0_.w), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) + self.g1_ * vec3(other.g0_.w), vec2(self.g1_.x) * vec2(0.0, other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, other.g0_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(other.g0_.w, 0.0, 0.0, -other.g0_.x) + vec4(self.g0_.y) * vec4(0.0, other.g0_.w, 0.0, -other.g0_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, other.g0_.w, -other.g0_.z), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec3(self.g1_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g1_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g1_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) - vec3(self.g2_.x, self.g2_.y, self.g2_.z) * vec3(other.g0_.w) + vec3(self.g2_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(0.0), vec2(0.0));
}

MultiVector dipole_antiWedgeDot_flector(Dipole self, Flector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, other.g0_.z) + vec2(self.g2_.w) * vec2(0.0, -other.g0_.w), vec3(self.g0_.x) * vec3(other.g1_.w, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, other.g1_.w, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, other.g1_.w) + vec3(self.g1_.x) * vec3(other.g0_.w, -other.g1_.z, other.g1_.y) + vec3(self.g1_.y) * vec3(other.g1_.z, other.g0_.w, -other.g1_.x) + vec3(self.g1_.z) * vec3(-other.g1_.y, other.g1_.x, other.g0_.w), vec2(self.g0_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g1_.z, 0.0) + vec2(self.g1_.x) * vec2(0.0, other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, other.g0_.z) + vec2(self.g2_.x) * vec2(0.0, other.g1_.x) + vec2(self.g2_.y) * vec2(0.0, other.g1_.y) + vec2(self.g2_.z) * vec2(0.0, other.g1_.z) + vec2(self.g2_.w) * vec2(0.0, other.g1_.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(other.g0_.w, -other.g1_.z, other.g1_.y, -other.g0_.x) + vec4(self.g0_.y) * vec4(other.g1_.z, other.g0_.w, -other.g1_.x, -other.g0_.y) + vec4(self.g0_.z) * vec4(-other.g1_.y, other.g1_.x, other.g0_.w, -other.g0_.z) + vec4(self.g1_.x) * vec4(0.0, 0.0, 0.0, -other.g1_.x) + vec4(self.g1_.y) * vec4(0.0, 0.0, 0.0, -other.g1_.y) + vec4(self.g1_.z) * vec4(0.0, 0.0, 0.0, -other.g1_.z), vec3(self.g0_.x) * vec3(-other.g1_.w, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, -other.g1_.w, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, -other.g1_.w) - vec3(self.g2_.w) * vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec3(self.g1_.x) * vec3(-other.g1_.w, other.g0_.z, -other.g0_.y) + vec3(self.g1_.y) * vec3(-other.g0_.z, -other.g1_.w, other.g0_.x) + vec3(self.g1_.z) * vec3(other.g0_.y, -other.g0_.x, -other.g1_.w) + vec3(self.g2_.x) * vec3(-other.g0_.w, other.g1_.z, -other.g1_.y) + vec3(self.g2_.y) * vec3(-other.g1_.z, -other.g0_.w, other.g1_.x) + vec3(self.g2_.z) * vec3(other.g1_.y, -other.g1_.x, -other.g0_.w) + vec3(self.g2_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(0.0), vec2(0.0));
}

MultiVector dipole_antiWedgeDot_line(Dipole self, Line other) {
    return MultiVector(vec2(self.g0_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g1_.z, 0.0) + vec2(self.g1_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g1_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g1_.z) * vec2(-other.g0_.z, 0.0), vec3(0.0), vec2(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(self.g0_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, -other.g1_.x) + vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, -other.g1_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, 0.0, -other.g1_.z) + vec4(self.g1_.x) * vec4(0.0, -other.g1_.z, other.g1_.y, 0.0) + vec4(self.g1_.y) * vec4(other.g1_.z, 0.0, -other.g1_.x, 0.0) + vec4(self.g1_.z) * vec4(-other.g1_.y, other.g1_.x, 0.0, 0.0) + vec4(self.g2_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g2_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g2_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0) + vec4(self.g2_.w) * vec4(-other.g1_.x, -other.g1_.y, -other.g1_.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g2_.w) * other.g0_, vec2(self.g0_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g1_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g2_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g2_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g2_.z) * vec2(0.0, -other.g0_.z));
}

MultiVector dipole_antiWedgeDot_motor(Dipole self, Motor other) {
    return MultiVector(vec2(self.g0_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g1_.z, 0.0) + vec2(self.g1_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g1_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g1_.z) * vec2(-other.g0_.z, 0.0), vec3(0.0), vec2(0.0), vec3(self.g0_.x) * vec3(other.g0_.w, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, other.g0_.w, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, other.g0_.w), vec3(self.g0_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g1_.x) * vec3(other.g0_.w, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, other.g0_.w, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, other.g0_.w), vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, -other.g1_.x) + vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, -other.g1_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, 0.0, -other.g1_.z) + vec4(self.g1_.x) * vec4(0.0, -other.g1_.z, other.g1_.y, 0.0) + vec4(self.g1_.y) * vec4(other.g1_.z, 0.0, -other.g1_.x, 0.0) + vec4(self.g1_.z) * vec4(-other.g1_.y, other.g1_.x, 0.0, 0.0) + vec4(self.g2_.x) * vec4(other.g0_.w, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g2_.y) * vec4(other.g0_.z, other.g0_.w, -other.g0_.x, 0.0) + vec4(self.g2_.z) * vec4(-other.g0_.y, other.g0_.x, other.g0_.w, 0.0) + vec4(self.g2_.w) * vec4(-other.g1_.x, -other.g1_.y, -other.g1_.z, other.g0_.w), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g2_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g0_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g1_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g2_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g2_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g2_.z) * vec2(0.0, -other.g0_.z));
}

MultiVector dipole_antiWedgeDot_multiVector(Dipole self, MultiVector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(-other.g8_.x, other.g5_.x) + vec2(self.g0_.y) * vec2(-other.g8_.y, other.g5_.y) + vec2(self.g0_.z) * vec2(-other.g8_.z, other.g5_.z) + vec2(self.g1_.x) * vec2(-other.g7_.x, other.g4_.x) + vec2(self.g1_.y) * vec2(-other.g7_.y, other.g4_.y) + vec2(self.g1_.z) * vec2(-other.g7_.z, other.g4_.z) + vec2(self.g2_.x) * vec2(-other.g6_.x, other.g3_.x) + vec2(self.g2_.y) * vec2(-other.g6_.y, other.g3_.y) + vec2(self.g2_.z) * vec2(-other.g6_.z, other.g3_.z) - vec2(self.g2_.w) * vec2(other.g6_.w, other.g5_.w), vec3(self.g0_.x) * vec3(other.g10_.y, -other.g5_.z, other.g5_.y) + vec3(self.g0_.y) * vec3(other.g5_.z, other.g10_.y, -other.g5_.x) + vec3(self.g0_.z) * vec3(-other.g5_.y, other.g5_.x, other.g10_.y) + vec3(self.g1_.x) * vec3(other.g5_.w, -other.g9_.z, other.g9_.y) + vec3(self.g1_.y) * vec3(other.g9_.z, other.g5_.w, -other.g9_.x) + vec3(self.g1_.z) * vec3(-other.g9_.y, other.g9_.x, other.g5_.w) + vec3(self.g2_.x) * vec3(-other.g10_.x, other.g3_.z, -other.g3_.y) + vec3(self.g2_.y) * vec3(-other.g3_.z, -other.g10_.x, other.g3_.x) + vec3(self.g2_.z) * vec3(other.g3_.y, -other.g3_.x, -other.g10_.x) + vec3(self.g2_.w) * other.g4_, vec2(self.g0_.x) * vec2(-other.g9_.x, 0.0) + vec2(self.g0_.x) * vec2(other.g4_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g9_.y, 0.0) + vec2(self.g0_.y) * vec2(other.g4_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g9_.z, 0.0) + vec2(self.g0_.z) * vec2(other.g4_.z, 0.0) + vec2(self.g1_.x) * vec2(other.g3_.x, other.g5_.x) + vec2(self.g1_.y) * vec2(other.g3_.y, other.g5_.y) + vec2(self.g1_.z) * vec2(other.g3_.z, other.g5_.z) + vec2(self.g2_.x) * vec2(0.0, other.g9_.x) + vec2(self.g2_.x) * vec2(0.0, other.g4_.x) + vec2(self.g2_.y) * vec2(0.0, other.g9_.y) + vec2(self.g2_.y) * vec2(0.0, other.g4_.y) + vec2(self.g2_.z) * vec2(0.0, other.g9_.z) + vec2(self.g2_.z) * vec2(0.0, other.g4_.z) + vec2(self.g2_.w) * other.g10_ * vec2(-1.0, 1.0), vec3(self.g0_.x) * vec3(other.g6_.w, -other.g7_.z, other.g7_.y) + vec3(self.g0_.x) * vec3(other.g0_.y, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g7_.z, other.g6_.w, -other.g7_.x) + vec3(self.g0_.y) * vec3(other.g1_.z, other.g0_.y, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g7_.y, other.g7_.x, other.g6_.w) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, other.g0_.y) + vec3(self.g1_.x) * vec3(other.g2_.x, -other.g6_.z, other.g6_.y) + vec3(self.g1_.y) * vec3(other.g6_.z, other.g2_.x, -other.g6_.x) + vec3(self.g1_.z) * vec3(-other.g6_.y, other.g6_.x, other.g2_.x) + vec3(self.g2_.w) * vec3(other.g6_.x, other.g6_.y, other.g6_.z), vec3(self.g0_.x) * vec3(other.g2_.y, -other.g8_.z, other.g8_.y) + vec3(self.g0_.y) * vec3(other.g8_.z, other.g2_.y, -other.g8_.x) + vec3(self.g0_.z) * vec3(-other.g8_.y, other.g8_.x, other.g2_.y) + vec3(self.g1_.x) * vec3(other.g0_.y, -other.g7_.z, other.g7_.y) + vec3(self.g1_.y) * vec3(other.g7_.z, other.g0_.y, -other.g7_.x) + vec3(self.g1_.z) * vec3(-other.g7_.y, other.g7_.x, other.g0_.y) + vec3(self.g2_.x) * vec3(other.g2_.x, -other.g6_.z, other.g6_.y) + vec3(self.g2_.y) * vec3(other.g6_.z, other.g2_.x, -other.g6_.x) + vec3(self.g2_.z) * vec3(-other.g6_.y, other.g6_.x, other.g2_.x) - vec3(self.g2_.w) * other.g1_, vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, -other.g8_.x) + vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, -other.g8_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, 0.0, -other.g8_.z) + vec4(self.g1_.x) * vec4(other.g2_.y, -other.g8_.z, other.g8_.y, other.g1_.x) + vec4(self.g1_.y) * vec4(other.g8_.z, other.g2_.y, -other.g8_.x, other.g1_.y) + vec4(self.g1_.z) * vec4(-other.g8_.y, other.g8_.x, other.g2_.y, other.g1_.z) + vec4(self.g2_.x) * vec4(-other.g6_.w, -other.g7_.z, other.g7_.y, other.g6_.x) + vec4(self.g2_.x) * vec4(other.g0_.y, other.g1_.z, -other.g1_.y, 0.0) + vec4(self.g2_.y) * vec4(other.g7_.z, -other.g6_.w, -other.g7_.x, other.g6_.y) + vec4(self.g2_.y) * vec4(-other.g1_.z, other.g0_.y, other.g1_.x, 0.0) + vec4(self.g2_.z) * vec4(-other.g7_.y, other.g7_.x, -other.g6_.w, other.g6_.z) + vec4(self.g2_.z) * vec4(other.g1_.y, -other.g1_.x, other.g0_.y, 0.0) + vec4(self.g2_.w) * vec4(-other.g8_.x, -other.g8_.y, -other.g8_.z, other.g0_.y), vec4(self.g0_.x) * vec4(other.g5_.w, -other.g9_.z, other.g9_.y, -other.g5_.x) + vec4(self.g0_.x) * vec4(-other.g0_.x, other.g4_.z, -other.g4_.y, 0.0) + vec4(self.g0_.y) * vec4(other.g9_.z, other.g5_.w, -other.g9_.x, -other.g5_.y) + vec4(self.g0_.y) * vec4(-other.g4_.z, -other.g0_.x, other.g4_.x, 0.0) + vec4(self.g0_.z) * vec4(-other.g9_.y, other.g9_.x, other.g5_.w, -other.g5_.z) + vec4(self.g0_.z) * vec4(other.g4_.y, -other.g4_.x, -other.g0_.x, 0.0) + vec4(self.g1_.x) * vec4(-other.g10_.x, other.g3_.z, -other.g3_.y, -other.g9_.x) + vec4(self.g1_.y) * vec4(-other.g3_.z, -other.g10_.x, other.g3_.x, -other.g9_.y) + vec4(self.g1_.z) * vec4(other.g3_.y, -other.g3_.x, -other.g10_.x, -other.g9_.z) + vec4(self.g2_.x) * vec4(0.0, 0.0, 0.0, other.g3_.x) + vec4(self.g2_.y) * vec4(0.0, 0.0, 0.0, other.g3_.y) + vec4(self.g2_.z) * vec4(0.0, 0.0, 0.0, other.g3_.z) + vec4(self.g2_.w) * vec4(-other.g3_.x, -other.g3_.y, -other.g3_.z, other.g0_.x), vec3(self.g0_.x) * vec3(-other.g10_.y, other.g5_.z, -other.g5_.y) + vec3(self.g0_.y) * vec3(-other.g5_.z, -other.g10_.y, other.g5_.x) + vec3(self.g0_.z) * vec3(other.g5_.y, -other.g5_.x, -other.g10_.y) + vec3(self.g1_.x) * vec3(-other.g0_.x, other.g4_.z, -other.g4_.y) + vec3(self.g1_.y) * vec3(-other.g4_.z, -other.g0_.x, other.g4_.x) + vec3(self.g1_.z) * vec3(other.g4_.y, -other.g4_.x, -other.g0_.x) + vec3(self.g2_.x) * vec3(-other.g10_.x, other.g3_.z, -other.g3_.y) + vec3(self.g2_.y) * vec3(-other.g3_.z, -other.g10_.x, other.g3_.x) + vec3(self.g2_.z) * vec3(other.g3_.y, -other.g3_.x, -other.g10_.x) - vec3(self.g2_.w) * other.g9_, vec3(self.g1_.x) * vec3(-other.g10_.y, other.g5_.z, -other.g5_.y) + vec3(self.g1_.y) * vec3(-other.g5_.z, -other.g10_.y, other.g5_.x) + vec3(self.g1_.z) * vec3(other.g5_.y, -other.g5_.x, -other.g10_.y) + vec3(self.g2_.x) * vec3(-other.g5_.w, other.g9_.z, -other.g9_.y) + vec3(self.g2_.x) * vec3(-other.g0_.x, other.g4_.z, -other.g4_.y) + vec3(self.g2_.y) * vec3(-other.g9_.z, -other.g5_.w, other.g9_.x) + vec3(self.g2_.y) * vec3(-other.g4_.z, -other.g0_.x, other.g4_.x) + vec3(self.g2_.z) * vec3(other.g9_.y, -other.g9_.x, -other.g5_.w) + vec3(self.g2_.z) * vec3(other.g4_.y, -other.g4_.x, -other.g0_.x) + vec3(self.g2_.w) * vec3(other.g5_.x, other.g5_.y, other.g5_.z), vec3(self.g0_.x) * vec3(other.g2_.y, -other.g8_.z, other.g8_.y) + vec3(self.g0_.y) * vec3(other.g8_.z, other.g2_.y, -other.g8_.x) + vec3(self.g0_.z) * vec3(-other.g8_.y, other.g8_.x, other.g2_.y) + vec3(self.g1_.x) * vec3(-other.g6_.w, other.g1_.z, -other.g1_.y) + vec3(self.g1_.y) * vec3(-other.g1_.z, -other.g6_.w, other.g1_.x) + vec3(self.g1_.z) * vec3(other.g1_.y, -other.g1_.x, -other.g6_.w) + vec3(self.g2_.x) * vec3(-other.g2_.x, other.g6_.z, -other.g6_.y) + vec3(self.g2_.y) * vec3(-other.g6_.z, -other.g2_.x, other.g6_.x) + vec3(self.g2_.z) * vec3(other.g6_.y, -other.g6_.x, -other.g2_.x) + vec3(self.g2_.w) * other.g7_, vec2(self.g0_.x) * vec2(-other.g7_.x, 0.0) + vec2(self.g0_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g7_.y, 0.0) + vec2(self.g0_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g7_.z, 0.0) + vec2(self.g0_.z) * vec2(-other.g1_.z, 0.0) - vec2(self.g1_.x) * vec2(other.g6_.x, other.g8_.x) - vec2(self.g1_.y) * vec2(other.g6_.y, other.g8_.y) - vec2(self.g1_.z) * vec2(other.g6_.z, other.g8_.z) + vec2(self.g2_.x) * vec2(0.0, -other.g7_.x) + vec2(self.g2_.x) * vec2(0.0, other.g1_.x) + vec2(self.g2_.y) * vec2(0.0, -other.g7_.y) + vec2(self.g2_.y) * vec2(0.0, other.g1_.y) + vec2(self.g2_.z) * vec2(0.0, -other.g7_.z) + vec2(self.g2_.z) * vec2(0.0, other.g1_.z) + vec2(self.g2_.w) * other.g2_ * vec2(1.0, -1.0));
}

MultiVector dipole_antiWedgeDot_plane(Dipole self, Plane other) {
    return MultiVector(vec2(0.0), self.g0_ * vec3(other.g0_.w) + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec2(self.g0_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g2_.x) * vec2(0.0, other.g0_.x) + vec2(self.g2_.y) * vec2(0.0, other.g0_.y) + vec2(self.g2_.z) * vec2(0.0, other.g0_.z) + vec2(self.g2_.w) * vec2(0.0, other.g0_.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g0_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g0_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0) + vec4(self.g1_.x) * vec4(0.0, 0.0, 0.0, -other.g0_.x) + vec4(self.g1_.y) * vec4(0.0, 0.0, 0.0, -other.g0_.y) + vec4(self.g1_.z) * vec4(0.0, 0.0, 0.0, -other.g0_.z), vec3(0.0) - self.g0_ * vec3(other.g0_.w) - vec3(self.g2_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(0.0) - self.g1_ * vec3(other.g0_.w) + vec3(self.g2_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g2_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g2_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec3(0.0), vec2(0.0));
}

MultiVector dipole_antiWedgeDot_roundPoint(Dipole self, RoundPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) + self.g1_ * vec3(other.g1_.x), self.g0_ * vec3(other.g1_.y) + vec3(self.g2_.x, self.g2_.y, self.g2_.z) * vec3(other.g1_.x) - vec3(self.g2_.w) * other.g0_, vec4(self.g1_.x) * vec4(other.g1_.y, 0.0, 0.0, other.g0_.x) + vec4(self.g1_.y) * vec4(0.0, other.g1_.y, 0.0, other.g0_.y) + vec4(self.g1_.z) * vec4(0.0, 0.0, other.g1_.y, other.g0_.z) + vec4(self.g2_.x) * vec4(0.0, other.g0_.z, -other.g0_.y, 0.0) + vec4(self.g2_.y) * vec4(-other.g0_.z, 0.0, other.g0_.x, 0.0) + vec4(self.g2_.z) * vec4(other.g0_.y, -other.g0_.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), self.g0_ * vec3(other.g1_.y) + vec3(self.g1_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g1_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g1_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) - vec3(self.g2_.x, self.g2_.y, self.g2_.z) * vec3(other.g1_.x), vec2(self.g0_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g2_.x) * vec2(0.0, other.g0_.x) + vec2(self.g2_.y) * vec2(0.0, other.g0_.y) + vec2(self.g2_.z) * vec2(0.0, other.g0_.z) + vec2(self.g2_.w) * other.g1_ * vec2(1.0, -1.0));
}

Circle dipole_antiWedgeDot_scalar(Dipole self, Scalar other) {
    return Circle(vec4(self.g0_.x, self.g0_.y, self.g0_.z, self.g0_.x) * vec4(-other.g0_, -other.g0_, -other.g0_, 0.0) + vec4(self.g2_.w) * vec4(0.0, 0.0, 0.0, other.g0_), vec3(0.0) - self.g1_ * vec3(other.g0_), vec3(0.0) - vec3(self.g2_.x, self.g2_.y, self.g2_.z) * vec3(other.g0_));
}

MultiVector dipole_antiWedgeDot_sphere(Dipole self, Sphere other) {
    return MultiVector(vec2(0.0), self.g0_ * vec3(other.g1_.y) + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) - vec3(self.g2_.x, self.g2_.y, self.g2_.z) * vec3(other.g1_.x), vec2(self.g0_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g2_.x) * vec2(0.0, other.g0_.x) + vec2(self.g2_.y) * vec2(0.0, other.g0_.y) + vec2(self.g2_.z) * vec2(0.0, other.g0_.z) + vec2(self.g2_.w) * other.g1_ * vec2(-1.0, 1.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g0_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g0_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0) + vec4(self.g1_.x) * vec4(-other.g1_.x, 0.0, 0.0, -other.g0_.x) + vec4(self.g1_.y) * vec4(0.0, -other.g1_.x, 0.0, -other.g0_.y) + vec4(self.g1_.z) * vec4(0.0, 0.0, -other.g1_.x, -other.g0_.z), vec3(0.0) - self.g0_ * vec3(other.g1_.y) - vec3(self.g2_.x, self.g2_.y, self.g2_.z) * vec3(other.g1_.x) - vec3(self.g2_.w) * other.g0_, vec3(0.0) - self.g1_ * vec3(other.g1_.y) + vec3(self.g2_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g2_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g2_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec3(0.0), vec2(0.0));
}

DualNum dualNum_antiWedgeDot_antiScalar(DualNum self, AntiScalar other) {
    return DualNum(self.g0_ * vec2(other.g0_));
}

MultiVector dualNum_antiWedgeDot_circle(DualNum self, Circle other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g0_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(self.g0_.x) * other.g1_, vec4(self.g0_.x) * vec4(other.g2_.x, other.g2_.y, other.g2_.z, -other.g0_.w), vec4(self.g0_.y) * other.g0_, vec3(self.g0_.y) * other.g1_, vec3(self.g0_.y) * other.g2_, vec3(0.0), vec2(0.0));
}

MultiVector dualNum_antiWedgeDot_dipole(DualNum self, Dipole other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g0_.y) * other.g0_, vec3(self.g0_.y) * other.g1_, vec4(self.g0_.y) * other.g2_, vec4(self.g0_.x) * vec4(-other.g0_.x, -other.g0_.y, -other.g0_.z, other.g2_.w), vec3(0.0) - vec3(self.g0_.x) * other.g1_, vec3(0.0) - vec3(self.g0_.x) * vec3(other.g2_.x, other.g2_.y, other.g2_.z), vec3(0.0), vec2(0.0));
}

DualNum dualNum_antiWedgeDot_dualNum(DualNum self, DualNum other) {
    return DualNum(vec2(self.g0_.x) * other.g0_.yx * vec2(1.0, -1.0) + vec2(self.g0_.y) * other.g0_);
}

MultiVector dualNum_antiWedgeDot_flatPoint(DualNum self, FlatPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(self.g0_.y) * other.g0_, vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, other.g0_.w), vec3(0.0), vec3(0.0) - vec3(self.g0_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(0.0), vec2(0.0));
}

MultiVector dualNum_antiWedgeDot_flector(DualNum self, Flector other) {
    return MultiVector(vec2(0.0), vec3(self.g0_.x) * vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec2(self.g0_.x) * vec2(0.0, -other.g1_.w), vec3(0.0), vec3(0.0), vec4(self.g0_.y) * other.g0_, vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, other.g0_.w), vec3(0.0), vec3(0.0) - vec3(self.g0_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(self.g0_.y) * vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec2(self.g0_.y) * vec2(0.0, other.g1_.w));
}

MultiVector dualNum_antiWedgeDot_line(DualNum self, Line other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(self.g0_.x) * other.g0_, vec4(self.g0_.x) * vec4(other.g1_.x, other.g1_.y, other.g1_.z, 0.0), vec4(0.0), vec3(self.g0_.y) * other.g0_, vec3(self.g0_.y) * other.g1_, vec3(0.0), vec2(0.0));
}

MultiVector dualNum_antiWedgeDot_motor(DualNum self, Motor other) {
    return MultiVector(self.g0_ * vec2(other.g0_.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(self.g0_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec4(self.g0_.x) * vec4(other.g1_.x, other.g1_.y, other.g1_.z, 0.0), vec4(0.0), vec3(self.g0_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(self.g0_.y) * other.g1_, vec3(0.0), vec2(0.0));
}

MultiVector dualNum_antiWedgeDot_multiVector(DualNum self, MultiVector other) {
    return MultiVector(vec2(self.g0_.x) * other.g0_.yx * vec2(1.0, -1.0) + vec2(self.g0_.y) * other.g0_, vec3(self.g0_.x) * other.g9_ + vec3(self.g0_.y) * other.g1_, vec2(0.0) - vec2(self.g0_.x) * other.g10_ + vec2(self.g0_.y) * other.g2_, vec3(self.g0_.x) * vec3(other.g6_.x, other.g6_.y, other.g6_.z) + vec3(self.g0_.y) * other.g3_, vec3(self.g0_.x) * other.g7_ + vec3(self.g0_.y) * other.g4_, vec4(self.g0_.x) * vec4(other.g8_.x, other.g8_.y, other.g8_.z, -other.g6_.w) + vec4(self.g0_.y) * other.g5_, vec4(self.g0_.x) * vec4(-other.g3_.x, -other.g3_.y, -other.g3_.z, other.g5_.w) + vec4(self.g0_.y) * other.g6_, vec3(0.0) - vec3(self.g0_.x) * other.g4_ + vec3(self.g0_.y) * other.g7_, vec3(0.0) - vec3(self.g0_.x) * vec3(other.g5_.x, other.g5_.y, other.g5_.z) + vec3(self.g0_.y) * other.g8_, vec3(0.0) - vec3(self.g0_.x) * other.g1_ + vec3(self.g0_.y) * other.g9_, vec2(self.g0_.x) * other.g2_ + vec2(self.g0_.y) * other.g10_);
}

MultiVector dualNum_antiWedgeDot_plane(DualNum self, Plane other) {
    return MultiVector(vec2(0.0), vec3(self.g0_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g0_.x) * vec2(0.0, -other.g0_.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g0_.y) * vec2(0.0, other.g0_.w));
}

MultiVector dualNum_antiWedgeDot_roundPoint(DualNum self, RoundPoint other) {
    return MultiVector(vec2(0.0), vec3(self.g0_.y) * other.g0_, vec2(self.g0_.y) * other.g1_, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - vec3(self.g0_.x) * other.g0_, vec2(self.g0_.x) * other.g1_);
}

DualNum dualNum_antiWedgeDot_scalar(DualNum self, Scalar other) {
    return DualNum(self.g0_.yx * vec2(other.g0_));
}

MultiVector dualNum_antiWedgeDot_sphere(DualNum self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(self.g0_.x) * other.g0_, vec2(0.0) - vec2(self.g0_.x) * other.g1_, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.y) * other.g0_, vec2(self.g0_.y) * other.g1_);
}

FlatPoint flatPoint_antiWedgeDot_antiScalar(FlatPoint self, AntiScalar other) {
    return FlatPoint(self.g0_ * vec4(other.g0_));
}

MultiVector flatPoint_antiWedgeDot_circle(FlatPoint self, Circle other) {
    return MultiVector(vec2(self.g0_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g0_.w) * vec2(-other.g0_.w, 0.0), vec3(0.0), vec2(0.0), vec3(self.g0_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec4(self.g0_.x) * vec4(-other.g0_.w, -other.g1_.z, other.g1_.y, other.g0_.x) + vec4(self.g0_.y) * vec4(other.g1_.z, -other.g0_.w, -other.g1_.x, other.g0_.y) + vec4(self.g0_.z) * vec4(-other.g1_.y, other.g1_.x, -other.g0_.w, other.g0_.z) + vec4(self.g0_.w) * vec4(-other.g2_.x, -other.g2_.y, -other.g2_.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + vec3(self.g0_.w) * other.g1_, vec2(self.g0_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g1_.z));
}

MultiVector flatPoint_antiWedgeDot_dipole(FlatPoint self, Dipole other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, other.g0_.z) + vec2(self.g0_.w) * vec2(0.0, -other.g2_.w), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + vec3(self.g0_.w) * other.g1_, vec2(self.g0_.x) * vec2(0.0, other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, other.g1_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, other.g0_.x) + vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, other.g0_.y) + self.g0_.wwwz * vec4(-other.g0_.x, -other.g0_.y, -other.g0_.z, other.g0_.z), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec3(self.g0_.x) * vec3(-other.g2_.w, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, -other.g2_.w, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, -other.g2_.w) + vec3(self.g0_.w) * vec3(other.g2_.x, other.g2_.y, other.g2_.z), vec3(0.0), vec2(0.0));
}

MultiVector flatPoint_antiWedgeDot_dualNum(FlatPoint self, DualNum other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0_ * vec4(other.g0_.y), vec4(self.g0_.w) * vec4(0.0, 0.0, 0.0, other.g0_.x), vec3(0.0), vec3(0.0) - vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_.x), vec3(0.0), vec2(0.0));
}

Motor flatPoint_antiWedgeDot_flatPoint(FlatPoint self, FlatPoint other) {
    return Motor(vec4(self.g0_.w) * vec4(0.0, 0.0, 0.0, -other.g0_.w), vec3(0.0) - vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_.w) + vec3(self.g0_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z));
}

MultiVector flatPoint_antiWedgeDot_flector(FlatPoint self, Flector other) {
    return MultiVector(vec2(self.g0_.w) * vec2(0.0, -other.g0_.w), vec3(0.0), vec2(self.g0_.x) * vec2(0.0, other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, other.g1_.z) + vec2(self.g0_.w) * vec2(0.0, other.g1_.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0) - vec3(self.g0_.w) * vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec3(self.g0_.x) * vec3(-other.g0_.w, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, -other.g0_.w, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, -other.g0_.w) + vec3(self.g0_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(0.0), vec2(0.0));
}

Flector flatPoint_antiWedgeDot_line(FlatPoint self, Line other) {
    return Flector(vec4(self.g0_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g0_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g0_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0) + vec4(self.g0_.w) * vec4(-other.g1_.x, -other.g1_.y, -other.g1_.z, 0.0), vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, -other.g0_.x) + vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, -other.g0_.y) + self.g0_.wwwz * vec4(other.g0_.x, other.g0_.y, other.g0_.z, -other.g0_.z));
}

Flector flatPoint_antiWedgeDot_motor(FlatPoint self, Motor other) {
    return Flector(vec4(self.g0_.x) * vec4(other.g0_.w, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g0_.y) * vec4(other.g0_.z, other.g0_.w, -other.g0_.x, 0.0) + vec4(self.g0_.z) * vec4(-other.g0_.y, other.g0_.x, other.g0_.w, 0.0) + vec4(self.g0_.w) * vec4(-other.g1_.x, -other.g1_.y, -other.g1_.z, other.g0_.w), vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, -other.g0_.x) + vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, -other.g0_.y) + self.g0_.wwwz * other.g0_.xyzz * vec4(1.0, 1.0, 1.0, -1.0));
}

MultiVector flatPoint_antiWedgeDot_multiVector(FlatPoint self, MultiVector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(-other.g6_.x, other.g3_.x) + vec2(self.g0_.y) * vec2(-other.g6_.y, other.g3_.y) + vec2(self.g0_.z) * vec2(-other.g6_.z, other.g3_.z) - vec2(self.g0_.w) * vec2(other.g6_.w, other.g5_.w), vec3(self.g0_.x) * vec3(-other.g10_.x, other.g3_.z, -other.g3_.y) + vec3(self.g0_.y) * vec3(-other.g3_.z, -other.g10_.x, other.g3_.x) + vec3(self.g0_.z) * vec3(other.g3_.y, -other.g3_.x, -other.g10_.x) + vec3(self.g0_.w) * other.g4_, vec2(self.g0_.x) * vec2(0.0, other.g9_.x) + vec2(self.g0_.x) * vec2(0.0, other.g4_.x) + vec2(self.g0_.y) * vec2(0.0, other.g9_.y) + vec2(self.g0_.y) * vec2(0.0, other.g4_.y) + vec2(self.g0_.z) * vec2(0.0, other.g9_.z) + vec2(self.g0_.z) * vec2(0.0, other.g4_.z) + vec2(self.g0_.w) * other.g10_ * vec2(-1.0, 1.0), vec3(self.g0_.w) * vec3(other.g6_.x, other.g6_.y, other.g6_.z), vec3(self.g0_.x) * vec3(other.g2_.x, -other.g6_.z, other.g6_.y) + vec3(self.g0_.y) * vec3(other.g6_.z, other.g2_.x, -other.g6_.x) + vec3(self.g0_.z) * vec3(-other.g6_.y, other.g6_.x, other.g2_.x) - vec3(self.g0_.w) * other.g1_, vec4(self.g0_.x) * vec4(-other.g6_.w, -other.g7_.z, other.g7_.y, other.g6_.x) + vec4(self.g0_.x) * vec4(other.g0_.y, other.g1_.z, -other.g1_.y, 0.0) + vec4(self.g0_.y) * vec4(other.g7_.z, -other.g6_.w, -other.g7_.x, other.g6_.y) + vec4(self.g0_.y) * vec4(-other.g1_.z, other.g0_.y, other.g1_.x, 0.0) + vec4(self.g0_.z) * vec4(-other.g7_.y, other.g7_.x, -other.g6_.w, other.g6_.z) + vec4(self.g0_.z) * vec4(other.g1_.y, -other.g1_.x, other.g0_.y, 0.0) + vec4(self.g0_.w) * vec4(-other.g8_.x, -other.g8_.y, -other.g8_.z, other.g0_.y), vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, other.g3_.x) + vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, other.g3_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, 0.0, other.g3_.z) + vec4(self.g0_.w) * vec4(-other.g3_.x, -other.g3_.y, -other.g3_.z, other.g0_.x), vec3(self.g0_.x) * vec3(-other.g10_.x, other.g3_.z, -other.g3_.y) + vec3(self.g0_.y) * vec3(-other.g3_.z, -other.g10_.x, other.g3_.x) + vec3(self.g0_.z) * vec3(other.g3_.y, -other.g3_.x, -other.g10_.x) - vec3(self.g0_.w) * other.g9_, vec3(self.g0_.x) * vec3(-other.g5_.w, other.g9_.z, -other.g9_.y) + vec3(self.g0_.x) * vec3(-other.g0_.x, other.g4_.z, -other.g4_.y) + vec3(self.g0_.y) * vec3(-other.g9_.z, -other.g5_.w, other.g9_.x) + vec3(self.g0_.y) * vec3(-other.g4_.z, -other.g0_.x, other.g4_.x) + vec3(self.g0_.z) * vec3(other.g9_.y, -other.g9_.x, -other.g5_.w) + vec3(self.g0_.z) * vec3(other.g4_.y, -other.g4_.x, -other.g0_.x) + vec3(self.g0_.w) * vec3(other.g5_.x, other.g5_.y, other.g5_.z), vec3(self.g0_.x) * vec3(-other.g2_.x, other.g6_.z, -other.g6_.y) + vec3(self.g0_.y) * vec3(-other.g6_.z, -other.g2_.x, other.g6_.x) + vec3(self.g0_.z) * vec3(other.g6_.y, -other.g6_.x, -other.g2_.x) + vec3(self.g0_.w) * other.g7_, vec2(self.g0_.x) * vec2(0.0, -other.g7_.x) + vec2(self.g0_.x) * vec2(0.0, other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g7_.y) + vec2(self.g0_.y) * vec2(0.0, other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g7_.z) + vec2(self.g0_.z) * vec2(0.0, other.g1_.z) + vec2(self.g0_.w) * other.g2_ * vec2(1.0, -1.0));
}

MultiVector flatPoint_antiWedgeDot_plane(FlatPoint self, Plane other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(self.g0_.x) * vec2(0.0, other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, other.g0_.z) + vec2(self.g0_.w) * vec2(0.0, other.g0_.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0) - vec3(self.g0_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec3(0.0), vec2(0.0));
}

MultiVector flatPoint_antiWedgeDot_roundPoint(FlatPoint self, RoundPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g1_.x) - vec3(self.g0_.w) * other.g0_, vec4(self.g0_.x) * vec4(0.0, other.g0_.z, -other.g0_.y, 0.0) + vec4(self.g0_.y) * vec4(-other.g0_.z, 0.0, other.g0_.x, 0.0) + vec4(self.g0_.z) * vec4(other.g0_.y, -other.g0_.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g1_.x), vec2(self.g0_.x) * vec2(0.0, other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, other.g0_.z) + vec2(self.g0_.w) * other.g1_ * vec2(1.0, -1.0));
}

Circle flatPoint_antiWedgeDot_scalar(FlatPoint self, Scalar other) {
    return Circle(vec4(self.g0_.w) * vec4(0.0, 0.0, 0.0, other.g0_), vec3(0.0), vec3(0.0) - vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_));
}

MultiVector flatPoint_antiWedgeDot_sphere(FlatPoint self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(0.0) - vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g1_.x), vec2(self.g0_.x) * vec2(0.0, other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, other.g0_.z) + vec2(self.g0_.w) * other.g1_ * vec2(-1.0, 1.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0) - vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g1_.x) - vec3(self.g0_.w) * other.g0_, vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec3(0.0), vec2(0.0));
}

Flector flector_antiWedgeDot_antiScalar(Flector self, AntiScalar other) {
    return Flector(self.g0_ * vec4(other.g0_), self.g1_ * vec4(other.g0_));
}

MultiVector flector_antiWedgeDot_circle(Flector self, Circle other) {
    return MultiVector(vec2(self.g0_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g0_.w) * vec2(-other.g0_.w, 0.0), vec3(0.0), vec2(0.0), vec3(self.g0_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) - vec3(self.g1_.x, self.g1_.y, self.g1_.z) * vec3(other.g0_.w) + vec3(self.g1_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec4(self.g0_.x) * vec4(-other.g0_.w, -other.g1_.z, other.g1_.y, other.g0_.x) + vec4(self.g0_.y) * vec4(other.g1_.z, -other.g0_.w, -other.g1_.x, other.g0_.y) + vec4(self.g0_.z) * vec4(-other.g1_.y, other.g1_.x, -other.g0_.w, other.g0_.z) + vec4(self.g0_.w) * vec4(-other.g2_.x, -other.g2_.y, -other.g2_.z, 0.0) + vec4(self.g1_.x) * vec4(0.0, other.g2_.z, -other.g2_.y, -other.g1_.x) + vec4(self.g1_.y) * vec4(-other.g2_.z, 0.0, other.g2_.x, -other.g1_.y) + vec4(self.g1_.z) * vec4(other.g2_.y, -other.g2_.x, 0.0, -other.g1_.z) + vec4(self.g1_.w) * vec4(other.g1_.x, other.g1_.y, other.g1_.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + vec3(self.g0_.w) * other.g1_ + vec3(self.g1_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g1_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g1_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) - vec3(self.g1_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g0_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g1_.x) * vec2(-other.g0_.x, other.g2_.x) + vec2(self.g1_.y) * vec2(-other.g0_.y, other.g2_.y) + vec2(self.g1_.z) * vec2(-other.g0_.z, other.g2_.z) + vec2(self.g1_.w) * vec2(0.0, -other.g0_.w));
}

MultiVector flector_antiWedgeDot_dipole(Flector self, Dipole other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, other.g0_.z) + vec2(self.g0_.w) * vec2(0.0, -other.g2_.w), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + vec3(self.g0_.w) * other.g1_ + vec3(self.g1_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g1_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g1_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) - vec3(self.g1_.w) * other.g0_, vec2(self.g0_.x) * vec2(0.0, other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, other.g1_.z) + vec2(self.g1_.x) * vec2(other.g0_.x, -other.g2_.x) + vec2(self.g1_.y) * vec2(other.g0_.y, -other.g2_.y) + vec2(self.g1_.z) * vec2(other.g0_.z, -other.g2_.z) + vec2(self.g1_.w) * vec2(0.0, -other.g2_.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, other.g0_.x) + vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, other.g0_.y) + self.g0_.wwwz * vec4(-other.g0_.x, -other.g0_.y, -other.g0_.z, other.g0_.z) + vec4(self.g1_.x) * vec4(0.0, other.g0_.z, -other.g0_.y, -other.g1_.x) + vec4(self.g1_.y) * vec4(-other.g0_.z, 0.0, other.g0_.x, -other.g1_.y) + vec4(self.g1_.z) * vec4(other.g0_.y, -other.g0_.x, 0.0, -other.g1_.z), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) - vec3(self.g1_.x, self.g1_.y, self.g1_.z) * vec3(other.g2_.w) - vec3(self.g1_.w) * other.g0_, vec3(self.g0_.x) * vec3(-other.g2_.w, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, -other.g2_.w, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, -other.g2_.w) + vec3(self.g0_.w) * vec3(other.g2_.x, other.g2_.y, other.g2_.z) + vec3(self.g1_.x) * vec3(0.0, -other.g2_.z, other.g2_.y) + vec3(self.g1_.y) * vec3(other.g2_.z, 0.0, -other.g2_.x) + vec3(self.g1_.z) * vec3(-other.g2_.y, other.g2_.x, 0.0) - vec3(self.g1_.w) * other.g1_, vec3(0.0), vec2(0.0));
}

MultiVector flector_antiWedgeDot_dualNum(Flector self, DualNum other) {
    return MultiVector(vec2(0.0), vec3(self.g1_.x, self.g1_.y, self.g1_.z) * vec3(other.g0_.x), vec2(self.g1_.w) * vec2(0.0, -other.g0_.x), vec3(0.0), vec3(0.0), self.g0_ * vec4(other.g0_.y), vec4(self.g0_.w) * vec4(0.0, 0.0, 0.0, other.g0_.x), vec3(0.0), vec3(0.0) - vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_.x), vec3(self.g1_.x, self.g1_.y, self.g1_.z) * vec3(other.g0_.y), vec2(self.g1_.w) * vec2(0.0, other.g0_.y));
}

MultiVector flector_antiWedgeDot_flatPoint(Flector self, FlatPoint other) {
    return MultiVector(vec2(self.g0_.w) * vec2(0.0, -other.g0_.w), vec3(0.0), vec2(self.g1_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g1_.w) * vec2(0.0, -other.g0_.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0) - vec3(self.g1_.x, self.g1_.y, self.g1_.z) * vec3(other.g0_.w), vec3(0.0) - vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_.w) + vec3(self.g0_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(0.0), vec2(0.0));
}

MultiVector flector_antiWedgeDot_flector(Flector self, Flector other) {
    return MultiVector(vec2(self.g0_.w) * vec2(0.0, -other.g0_.w) + vec2(self.g1_.x) * vec2(0.0, other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, other.g1_.z), vec3(0.0), vec2(self.g0_.x) * vec2(0.0, other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, other.g1_.z) + vec2(self.g0_.w) * vec2(0.0, other.g1_.w) + vec2(self.g1_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g1_.w) * vec2(0.0, -other.g0_.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0) - vec3(self.g0_.w) * vec3(other.g1_.x, other.g1_.y, other.g1_.z) + vec3(self.g1_.x) * vec3(-other.g0_.w, other.g1_.z, -other.g1_.y) + vec3(self.g1_.y) * vec3(-other.g1_.z, -other.g0_.w, other.g1_.x) + vec3(self.g1_.z) * vec3(other.g1_.y, -other.g1_.x, -other.g0_.w), vec3(self.g0_.x) * vec3(-other.g0_.w, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, -other.g0_.w, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, -other.g0_.w) + vec3(self.g0_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g1_.x) * vec3(other.g1_.w, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, other.g1_.w, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, other.g1_.w) - vec3(self.g1_.w) * vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec3(0.0), vec2(0.0));
}

Flector flector_antiWedgeDot_line(Flector self, Line other) {
    return Flector(vec4(self.g0_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g0_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g0_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0) + vec4(self.g0_.w) * vec4(-other.g1_.x, -other.g1_.y, -other.g1_.z, 0.0) + vec4(self.g1_.x) * vec4(0.0, other.g1_.z, -other.g1_.y, -other.g0_.x) + vec4(self.g1_.y) * vec4(-other.g1_.z, 0.0, other.g1_.x, -other.g0_.y) + vec4(self.g1_.z) * vec4(other.g1_.y, -other.g1_.x, 0.0, -other.g0_.z) + vec4(self.g1_.w) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, 0.0), vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, -other.g0_.x) + vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, -other.g0_.y) + self.g0_.wwwz * vec4(other.g0_.x, other.g0_.y, other.g0_.z, -other.g0_.z) + vec4(self.g1_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, other.g1_.x) + vec4(self.g1_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, other.g1_.y) + vec4(self.g1_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, other.g1_.z));
}

Flector flector_antiWedgeDot_motor(Flector self, Motor other) {
    return Flector(vec4(self.g0_.x) * vec4(other.g0_.w, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g0_.y) * vec4(other.g0_.z, other.g0_.w, -other.g0_.x, 0.0) + vec4(self.g0_.z) * vec4(-other.g0_.y, other.g0_.x, other.g0_.w, 0.0) + vec4(self.g0_.w) * vec4(-other.g1_.x, -other.g1_.y, -other.g1_.z, other.g0_.w) + vec4(self.g1_.x) * vec4(0.0, other.g1_.z, -other.g1_.y, -other.g0_.x) + vec4(self.g1_.y) * vec4(-other.g1_.z, 0.0, other.g1_.x, -other.g0_.y) + vec4(self.g1_.z) * vec4(other.g1_.y, -other.g1_.x, 0.0, -other.g0_.z) + vec4(self.g1_.w) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, 0.0), vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, -other.g0_.x) + vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, -other.g0_.y) + self.g0_.wwwz * other.g0_.xyzz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g1_.x) * vec4(other.g0_.w, -other.g0_.z, other.g0_.y, other.g1_.x) + vec4(self.g1_.y) * vec4(other.g0_.z, other.g0_.w, -other.g0_.x, other.g1_.y) + vec4(self.g1_.z) * vec4(-other.g0_.y, other.g0_.x, other.g0_.w, other.g1_.z) + vec4(self.g1_.w) * vec4(0.0, 0.0, 0.0, other.g0_.w));
}

MultiVector flector_antiWedgeDot_multiVector(Flector self, MultiVector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(-other.g6_.x, other.g3_.x) + vec2(self.g0_.y) * vec2(-other.g6_.y, other.g3_.y) + vec2(self.g0_.z) * vec2(-other.g6_.z, other.g3_.z) - vec2(self.g0_.w) * vec2(other.g6_.w, other.g5_.w) + vec2(self.g1_.x) * vec2(other.g1_.x, other.g9_.x) + vec2(self.g1_.y) * vec2(other.g1_.y, other.g9_.y) + vec2(self.g1_.z) * vec2(other.g1_.z, other.g9_.z) + vec2(self.g1_.w) * vec2(other.g2_.x, -other.g10_.x), vec3(self.g0_.x) * vec3(-other.g10_.x, other.g3_.z, -other.g3_.y) + vec3(self.g0_.y) * vec3(-other.g3_.z, -other.g10_.x, other.g3_.x) + vec3(self.g0_.z) * vec3(other.g3_.y, -other.g3_.x, -other.g10_.x) + vec3(self.g0_.w) * other.g4_ + vec3(self.g1_.x) * vec3(other.g0_.x, -other.g4_.z, other.g4_.y) + vec3(self.g1_.y) * vec3(other.g4_.z, other.g0_.x, -other.g4_.x) + vec3(self.g1_.z) * vec3(-other.g4_.y, other.g4_.x, other.g0_.x) - vec3(self.g1_.w) * other.g3_, vec2(self.g0_.x) * vec2(0.0, other.g9_.x) + vec2(self.g0_.x) * vec2(0.0, other.g4_.x) + vec2(self.g0_.y) * vec2(0.0, other.g9_.y) + vec2(self.g0_.y) * vec2(0.0, other.g4_.y) + vec2(self.g0_.z) * vec2(0.0, other.g9_.z) + vec2(self.g0_.z) * vec2(0.0, other.g4_.z) + vec2(self.g0_.w) * other.g10_ * vec2(-1.0, 1.0) + vec2(self.g1_.x) * vec2(other.g3_.x, -other.g5_.x) + vec2(self.g1_.y) * vec2(other.g3_.y, -other.g5_.y) + vec2(self.g1_.z) * vec2(other.g3_.z, -other.g5_.z) + vec2(self.g1_.w) * vec2(0.0, -other.g5_.w) + vec2(self.g1_.w) * vec2(0.0, -other.g0_.x), vec3(self.g0_.w) * vec3(other.g6_.x, other.g6_.y, other.g6_.z) + vec3(self.g1_.x) * vec3(other.g2_.x, -other.g6_.z, other.g6_.y) + vec3(self.g1_.y) * vec3(other.g6_.z, other.g2_.x, -other.g6_.x) + vec3(self.g1_.z) * vec3(-other.g6_.y, other.g6_.x, other.g2_.x), vec3(self.g0_.x) * vec3(other.g2_.x, -other.g6_.z, other.g6_.y) + vec3(self.g0_.y) * vec3(other.g6_.z, other.g2_.x, -other.g6_.x) + vec3(self.g0_.z) * vec3(-other.g6_.y, other.g6_.x, other.g2_.x) - vec3(self.g0_.w) * other.g1_ + vec3(self.g1_.x) * vec3(-other.g6_.w, other.g1_.z, -other.g1_.y) + vec3(self.g1_.y) * vec3(-other.g1_.z, -other.g6_.w, other.g1_.x) + vec3(self.g1_.z) * vec3(other.g1_.y, -other.g1_.x, -other.g6_.w) + vec3(self.g1_.w) * vec3(other.g6_.x, other.g6_.y, other.g6_.z), vec4(self.g0_.x) * vec4(-other.g6_.w, -other.g7_.z, other.g7_.y, other.g6_.x) + vec4(self.g0_.x) * vec4(other.g0_.y, other.g1_.z, -other.g1_.y, 0.0) + vec4(self.g0_.y) * vec4(other.g7_.z, -other.g6_.w, -other.g7_.x, other.g6_.y) + vec4(self.g0_.y) * vec4(-other.g1_.z, other.g0_.y, other.g1_.x, 0.0) + vec4(self.g0_.z) * vec4(-other.g7_.y, other.g7_.x, -other.g6_.w, other.g6_.z) + vec4(self.g0_.z) * vec4(other.g1_.y, -other.g1_.x, other.g0_.y, 0.0) + vec4(self.g0_.w) * vec4(-other.g8_.x, -other.g8_.y, -other.g8_.z, other.g0_.y) + vec4(self.g1_.x) * vec4(-other.g2_.y, other.g8_.z, -other.g8_.y, -other.g7_.x) + vec4(self.g1_.y) * vec4(-other.g8_.z, -other.g2_.y, other.g8_.x, -other.g7_.y) + vec4(self.g1_.z) * vec4(other.g8_.y, -other.g8_.x, -other.g2_.y, -other.g7_.z) + vec4(self.g1_.w) * vec4(other.g7_.x, other.g7_.y, other.g7_.z, -other.g2_.x) + vec4(self.g1_.w) * vec4(-other.g1_.x, -other.g1_.y, -other.g1_.z, 0.0), vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, other.g3_.x) + vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, other.g3_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, 0.0, other.g3_.z) + vec4(self.g0_.w) * vec4(-other.g3_.x, -other.g3_.y, -other.g3_.z, other.g0_.x) + vec4(self.g1_.x) * vec4(-other.g10_.x, other.g3_.z, -other.g3_.y, -other.g4_.x) + vec4(self.g1_.y) * vec4(-other.g3_.z, -other.g10_.x, other.g3_.x, -other.g4_.y) + vec4(self.g1_.z) * vec4(other.g3_.y, -other.g3_.x, -other.g10_.x, -other.g4_.z) + vec4(self.g1_.w) * vec4(0.0, 0.0, 0.0, -other.g10_.x), vec3(self.g0_.x) * vec3(-other.g10_.x, other.g3_.z, -other.g3_.y) + vec3(self.g0_.y) * vec3(-other.g3_.z, -other.g10_.x, other.g3_.x) + vec3(self.g0_.z) * vec3(other.g3_.y, -other.g3_.x, -other.g10_.x) - vec3(self.g0_.w) * other.g9_ + vec3(self.g1_.x) * vec3(-other.g5_.w, other.g9_.z, -other.g9_.y) + vec3(self.g1_.y) * vec3(-other.g9_.z, -other.g5_.w, other.g9_.x) + vec3(self.g1_.z) * vec3(other.g9_.y, -other.g9_.x, -other.g5_.w) - vec3(self.g1_.w) * other.g3_, vec3(self.g0_.x) * vec3(-other.g5_.w, other.g9_.z, -other.g9_.y) + vec3(self.g0_.x) * vec3(-other.g0_.x, other.g4_.z, -other.g4_.y) + vec3(self.g0_.y) * vec3(-other.g9_.z, -other.g5_.w, other.g9_.x) + vec3(self.g0_.y) * vec3(-other.g4_.z, -other.g0_.x, other.g4_.x) + vec3(self.g0_.z) * vec3(other.g9_.y, -other.g9_.x, -other.g5_.w) + vec3(self.g0_.z) * vec3(other.g4_.y, -other.g4_.x, -other.g0_.x) + vec3(self.g0_.w) * vec3(other.g5_.x, other.g5_.y, other.g5_.z) + vec3(self.g1_.x) * vec3(other.g10_.y, -other.g5_.z, other.g5_.y) + vec3(self.g1_.y) * vec3(other.g5_.z, other.g10_.y, -other.g5_.x) + vec3(self.g1_.z) * vec3(-other.g5_.y, other.g5_.x, other.g10_.y) - vec3(self.g1_.w) * other.g9_ - vec3(self.g1_.w) * other.g4_, vec3(self.g0_.x) * vec3(-other.g2_.x, other.g6_.z, -other.g6_.y) + vec3(self.g0_.y) * vec3(-other.g6_.z, -other.g2_.x, other.g6_.x) + vec3(self.g0_.z) * vec3(other.g6_.y, -other.g6_.x, -other.g2_.x) + vec3(self.g0_.w) * other.g7_ + vec3(self.g1_.x) * vec3(other.g0_.y, -other.g7_.z, other.g7_.y) + vec3(self.g1_.y) * vec3(other.g7_.z, other.g0_.y, -other.g7_.x) + vec3(self.g1_.z) * vec3(-other.g7_.y, other.g7_.x, other.g0_.y) - vec3(self.g1_.w) * vec3(other.g6_.x, other.g6_.y, other.g6_.z), vec2(self.g0_.x) * vec2(0.0, -other.g7_.x) + vec2(self.g0_.x) * vec2(0.0, other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g7_.y) + vec2(self.g0_.y) * vec2(0.0, other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g7_.z) + vec2(self.g0_.z) * vec2(0.0, other.g1_.z) + vec2(self.g0_.w) * other.g2_ * vec2(1.0, -1.0) + vec2(self.g1_.x) * vec2(-other.g6_.x, other.g8_.x) + vec2(self.g1_.y) * vec2(-other.g6_.y, other.g8_.y) + vec2(self.g1_.z) * vec2(-other.g6_.z, other.g8_.z) + vec2(self.g1_.w) * vec2(0.0, -other.g6_.w) + vec2(self.g1_.w) * vec2(0.0, other.g0_.y));
}

MultiVector flector_antiWedgeDot_plane(Flector self, Plane other) {
    return MultiVector(vec2(self.g1_.x) * vec2(0.0, other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, other.g0_.z), vec3(0.0), vec2(self.g0_.x) * vec2(0.0, other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, other.g0_.z) + vec2(self.g0_.w) * vec2(0.0, other.g0_.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0) - vec3(self.g0_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g1_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g1_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g1_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + vec3(self.g1_.x, self.g1_.y, self.g1_.z) * vec3(other.g0_.w) - vec3(self.g1_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(0.0), vec2(0.0));
}

MultiVector flector_antiWedgeDot_roundPoint(Flector self, RoundPoint other) {
    return MultiVector(vec2(self.g1_.x) * vec2(other.g0_.x, 0.0) + vec2(self.g1_.y) * vec2(other.g0_.y, 0.0) + vec2(self.g1_.z) * vec2(other.g0_.z, 0.0) + vec2(self.g1_.w) * vec2(other.g1_.x, 0.0), vec3(0.0), vec2(0.0), vec3(self.g1_.x, self.g1_.y, self.g1_.z) * vec3(other.g1_.x), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g1_.x) - vec3(self.g0_.w) * other.g0_ + vec3(self.g1_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g1_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g1_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec4(self.g0_.x) * vec4(0.0, other.g0_.z, -other.g0_.y, 0.0) + vec4(self.g0_.y) * vec4(-other.g0_.z, 0.0, other.g0_.x, 0.0) + vec4(self.g0_.z) * vec4(other.g0_.y, -other.g0_.x, 0.0, 0.0) + self.g1_.xyzx * vec4(-other.g1_.y, -other.g1_.y, -other.g1_.y, 0.0) - vec4(self.g1_.w) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.x), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g1_.x), vec2(self.g0_.x) * vec2(0.0, other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, other.g0_.z) + vec2(self.g0_.w) * other.g1_ * vec2(1.0, -1.0));
}

MultiVector flector_antiWedgeDot_scalar(Flector self, Scalar other) {
    return MultiVector(vec2(0.0), vec3(self.g1_.x, self.g1_.y, self.g1_.z) * vec3(other.g0_), vec2(self.g1_.w) * vec2(0.0, -other.g0_), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.w) * vec4(0.0, 0.0, 0.0, other.g0_), vec3(0.0), vec3(0.0) - vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_), vec3(0.0), vec2(0.0));
}

MultiVector flector_antiWedgeDot_sphere(Flector self, Sphere other) {
    return MultiVector(vec2(self.g1_.x) * vec2(0.0, other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, other.g0_.z) + vec2(self.g1_.w) * vec2(0.0, -other.g1_.x), vec3(0.0) - vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g1_.x), vec2(self.g0_.x) * vec2(0.0, other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, other.g0_.z) + vec2(self.g0_.w) * other.g1_ * vec2(-1.0, 1.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0) - self.g1_ * vec4(other.g1_.x), vec3(0.0) - vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g1_.x) - vec3(self.g0_.w) * other.g0_ + vec3(self.g1_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g1_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g1_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + vec3(self.g1_.x, self.g1_.y, self.g1_.z) * vec3(other.g1_.y) - vec3(self.g1_.w) * other.g0_, vec3(0.0), vec2(0.0));
}

Line line_antiWedgeDot_antiScalar(Line self, AntiScalar other) {
    return Line(self.g0_ * vec3(other.g0_), self.g1_ * vec3(other.g0_));
}

MultiVector line_antiWedgeDot_circle(Line self, Circle other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g1_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g0_.z), self.g0_ * vec3(other.g0_.w) + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec2(0.0) - vec2(self.g0_.x) * vec2(other.g0_.x, other.g2_.x) - vec2(self.g0_.y) * vec2(other.g0_.y, other.g2_.y) - vec2(self.g0_.z) * vec2(other.g0_.z, other.g2_.z) + vec2(self.g1_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g1_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g0_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g0_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0) + vec4(self.g1_.x) * vec4(0.0, 0.0, 0.0, -other.g0_.x) + vec4(self.g1_.y) * vec4(0.0, 0.0, 0.0, -other.g0_.y) + vec4(self.g1_.z) * vec4(0.0, 0.0, 0.0, -other.g0_.z), vec3(self.g0_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(self.g0_.x) * vec3(0.0, -other.g2_.z, other.g2_.y) + vec3(self.g0_.y) * vec3(other.g2_.z, 0.0, -other.g2_.x) + vec3(self.g0_.z) * vec3(-other.g2_.y, other.g2_.x, 0.0) + vec3(self.g1_.x) * vec3(-other.g0_.w, -other.g1_.z, other.g1_.y) + vec3(self.g1_.y) * vec3(other.g1_.z, -other.g0_.w, -other.g1_.x) + vec3(self.g1_.z) * vec3(-other.g1_.y, other.g1_.x, -other.g0_.w), vec3(0.0), vec2(0.0));
}

MultiVector line_antiWedgeDot_dipole(Line self, Dipole other) {
    return MultiVector(vec2(self.g0_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g1_.z, 0.0) + vec2(self.g1_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g1_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g1_.z) * vec2(-other.g0_.z, 0.0), vec3(0.0), vec2(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(self.g0_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec4(self.g0_.x) * vec4(0.0, -other.g2_.z, other.g2_.y, 0.0) + vec4(self.g0_.y) * vec4(other.g2_.z, 0.0, -other.g2_.x, 0.0) + vec4(self.g0_.z) * vec4(-other.g2_.y, other.g2_.x, 0.0, 0.0) + vec4(self.g1_.x) * vec4(other.g2_.w, -other.g1_.z, other.g1_.y, other.g0_.x) + vec4(self.g1_.y) * vec4(other.g1_.z, other.g2_.w, -other.g1_.x, other.g0_.y) + vec4(self.g1_.z) * vec4(-other.g1_.y, other.g1_.x, other.g2_.w, other.g0_.z), vec4(0.0), vec3(0.0), vec3(0.0), self.g0_ * vec3(other.g2_.w) + vec3(self.g1_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g1_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g1_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec2(0.0) - vec2(self.g0_.x) * vec2(other.g0_.x, other.g2_.x) - vec2(self.g0_.y) * vec2(other.g0_.y, other.g2_.y) - vec2(self.g0_.z) * vec2(other.g0_.z, other.g2_.z) + vec2(self.g1_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g1_.z));
}

MultiVector line_antiWedgeDot_dualNum(Line self, DualNum other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), self.g0_ * vec3(other.g0_.x), vec4(self.g1_.x, self.g1_.y, self.g1_.z, self.g1_.x) * vec4(other.g0_.x, other.g0_.x, other.g0_.x, 0.0), vec4(0.0), self.g0_ * vec3(other.g0_.y), self.g1_ * vec3(other.g0_.y), vec3(0.0), vec2(0.0));
}

Flector line_antiWedgeDot_flatPoint(Line self, FlatPoint other) {
    return Flector(vec4(self.g0_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g0_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g0_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0) + vec4(self.g1_.x, self.g1_.y, self.g1_.z, self.g1_.x) * vec4(other.g0_.w, other.g0_.w, other.g0_.w, 0.0), vec4(self.g0_.x) * vec4(other.g0_.w, 0.0, 0.0, -other.g0_.x) + vec4(self.g0_.y) * vec4(0.0, other.g0_.w, 0.0, -other.g0_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, other.g0_.w, -other.g0_.z));
}

Flector line_antiWedgeDot_flector(Line self, Flector other) {
    return Flector(vec4(self.g0_.x) * vec4(other.g1_.w, -other.g0_.z, other.g0_.y, -other.g1_.x) + vec4(self.g0_.y) * vec4(other.g0_.z, other.g1_.w, -other.g0_.x, -other.g1_.y) + vec4(self.g0_.z) * vec4(-other.g0_.y, other.g0_.x, other.g1_.w, -other.g1_.z) + vec4(self.g1_.x) * vec4(other.g0_.w, -other.g1_.z, other.g1_.y, 0.0) + vec4(self.g1_.y) * vec4(other.g1_.z, other.g0_.w, -other.g1_.x, 0.0) + vec4(self.g1_.z) * vec4(-other.g1_.y, other.g1_.x, other.g0_.w, 0.0), vec4(self.g0_.x) * vec4(other.g0_.w, -other.g1_.z, other.g1_.y, -other.g0_.x) + vec4(self.g0_.y) * vec4(other.g1_.z, other.g0_.w, -other.g1_.x, -other.g0_.y) + vec4(self.g0_.z) * vec4(-other.g1_.y, other.g1_.x, other.g0_.w, -other.g0_.z) + vec4(self.g1_.x) * vec4(0.0, 0.0, 0.0, -other.g1_.x) + vec4(self.g1_.y) * vec4(0.0, 0.0, 0.0, -other.g1_.y) + vec4(self.g1_.z) * vec4(0.0, 0.0, 0.0, -other.g1_.z));
}

MultiVector line_antiWedgeDot_line(Line self, Line other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g0_.z), vec3(0.0), vec2(self.g0_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g1_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g0_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(self.g0_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(0.0), vec2(0.0));
}

MultiVector line_antiWedgeDot_motor(Line self, Motor other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g0_.z), vec3(0.0), vec2(self.g0_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g1_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g0_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0_.x) * vec3(other.g0_.w, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, other.g0_.w, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, other.g0_.w), vec3(self.g0_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g1_.x) * vec3(other.g0_.w, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, other.g0_.w, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, other.g0_.w), vec3(0.0), vec2(0.0));
}

MultiVector line_antiWedgeDot_multiVector(Line self, MultiVector other) {
    return MultiVector(vec2(0.0) - vec2(self.g0_.x) * vec2(other.g4_.x, other.g7_.x) - vec2(self.g0_.y) * vec2(other.g4_.y, other.g7_.y) - vec2(self.g0_.z) * vec2(other.g4_.z, other.g7_.z) - vec2(self.g1_.x) * vec2(other.g3_.x, other.g6_.x) - vec2(self.g1_.y) * vec2(other.g3_.y, other.g6_.y) - vec2(self.g1_.z) * vec2(other.g3_.z, other.g6_.z), vec3(self.g0_.x) * vec3(other.g6_.w, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, other.g6_.w, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, other.g6_.w) + vec3(self.g1_.x) * vec3(other.g2_.x, -other.g6_.z, other.g6_.y) + vec3(self.g1_.y) * vec3(other.g6_.z, other.g2_.x, -other.g6_.x) + vec3(self.g1_.z) * vec3(-other.g6_.y, other.g6_.x, other.g2_.x), vec2(0.0) - vec2(self.g0_.x) * vec2(other.g6_.x, other.g8_.x) - vec2(self.g0_.y) * vec2(other.g6_.y, other.g8_.y) - vec2(self.g0_.z) * vec2(other.g6_.z, other.g8_.z) + vec2(self.g1_.x) * vec2(0.0, -other.g7_.x) + vec2(self.g1_.x) * vec2(0.0, other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g7_.y) + vec2(self.g1_.y) * vec2(0.0, other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g7_.z) + vec2(self.g1_.z) * vec2(0.0, other.g1_.z), vec3(self.g0_.x) * vec3(other.g10_.x, -other.g3_.z, other.g3_.y) + vec3(self.g0_.y) * vec3(other.g3_.z, other.g10_.x, -other.g3_.x) + vec3(self.g0_.z) * vec3(-other.g3_.y, other.g3_.x, other.g10_.x), vec3(self.g0_.x) * vec3(other.g0_.x, -other.g4_.z, other.g4_.y) + vec3(self.g0_.y) * vec3(other.g4_.z, other.g0_.x, -other.g4_.x) + vec3(self.g0_.z) * vec3(-other.g4_.y, other.g4_.x, other.g0_.x) + vec3(self.g1_.x) * vec3(other.g10_.x, -other.g3_.z, other.g3_.y) + vec3(self.g1_.y) * vec3(other.g3_.z, other.g10_.x, -other.g3_.x) + vec3(self.g1_.z) * vec3(-other.g3_.y, other.g3_.x, other.g10_.x), vec4(self.g0_.x) * vec4(other.g10_.y, -other.g5_.z, other.g5_.y, -other.g9_.x) + vec4(self.g0_.y) * vec4(other.g5_.z, other.g10_.y, -other.g5_.x, -other.g9_.y) + vec4(self.g0_.z) * vec4(-other.g5_.y, other.g5_.x, other.g10_.y, -other.g9_.z) + vec4(self.g1_.x) * vec4(other.g5_.w, -other.g9_.z, other.g9_.y, other.g3_.x) + vec4(self.g1_.x) * vec4(other.g0_.x, -other.g4_.z, other.g4_.y, 0.0) + vec4(self.g1_.y) * vec4(other.g9_.z, other.g5_.w, -other.g9_.x, other.g3_.y) + vec4(self.g1_.y) * vec4(other.g4_.z, other.g0_.x, -other.g4_.x, 0.0) + vec4(self.g1_.z) * vec4(-other.g9_.y, other.g9_.x, other.g5_.w, other.g3_.z) + vec4(self.g1_.z) * vec4(-other.g4_.y, other.g4_.x, other.g0_.x, 0.0), vec4(self.g0_.x) * vec4(other.g2_.x, -other.g6_.z, other.g6_.y, -other.g1_.x) + vec4(self.g0_.y) * vec4(other.g6_.z, other.g2_.x, -other.g6_.x, -other.g1_.y) + vec4(self.g0_.z) * vec4(-other.g6_.y, other.g6_.x, other.g2_.x, -other.g1_.z) + vec4(self.g1_.x) * vec4(0.0, 0.0, 0.0, -other.g6_.x) + vec4(self.g1_.y) * vec4(0.0, 0.0, 0.0, -other.g6_.y) + vec4(self.g1_.z) * vec4(0.0, 0.0, 0.0, -other.g6_.z), vec3(self.g0_.x) * vec3(other.g0_.y, -other.g7_.z, other.g7_.y) + vec3(self.g0_.y) * vec3(other.g7_.z, other.g0_.y, -other.g7_.x) + vec3(self.g0_.z) * vec3(-other.g7_.y, other.g7_.x, other.g0_.y) + vec3(self.g1_.x) * vec3(other.g2_.x, -other.g6_.z, other.g6_.y) + vec3(self.g1_.y) * vec3(other.g6_.z, other.g2_.x, -other.g6_.x) + vec3(self.g1_.z) * vec3(-other.g6_.y, other.g6_.x, other.g2_.x), vec3(self.g0_.x) * vec3(other.g2_.y, -other.g8_.z, other.g8_.y) + vec3(self.g0_.y) * vec3(other.g8_.z, other.g2_.y, -other.g8_.x) + vec3(self.g0_.z) * vec3(-other.g8_.y, other.g8_.x, other.g2_.y) + vec3(self.g1_.x) * vec3(-other.g6_.w, -other.g7_.z, other.g7_.y) + vec3(self.g1_.x) * vec3(other.g0_.y, other.g1_.z, -other.g1_.y) + vec3(self.g1_.y) * vec3(other.g7_.z, -other.g6_.w, -other.g7_.x) + vec3(self.g1_.y) * vec3(-other.g1_.z, other.g0_.y, other.g1_.x) + vec3(self.g1_.z) * vec3(-other.g7_.y, other.g7_.x, -other.g6_.w) + vec3(self.g1_.z) * vec3(other.g1_.y, -other.g1_.x, other.g0_.y), vec3(self.g0_.x) * vec3(other.g5_.w, -other.g9_.z, other.g9_.y) + vec3(self.g0_.y) * vec3(other.g9_.z, other.g5_.w, -other.g9_.x) + vec3(self.g0_.z) * vec3(-other.g9_.y, other.g9_.x, other.g5_.w) + vec3(self.g1_.x) * vec3(-other.g10_.x, other.g3_.z, -other.g3_.y) + vec3(self.g1_.y) * vec3(-other.g3_.z, -other.g10_.x, other.g3_.x) + vec3(self.g1_.z) * vec3(other.g3_.y, -other.g3_.x, -other.g10_.x), vec2(0.0) - vec2(self.g0_.x) * vec2(other.g3_.x, other.g5_.x) - vec2(self.g0_.y) * vec2(other.g3_.y, other.g5_.y) - vec2(self.g0_.z) * vec2(other.g3_.z, other.g5_.z) + vec2(self.g1_.x) * vec2(0.0, -other.g9_.x) + vec2(self.g1_.x) * vec2(0.0, -other.g4_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g9_.y) + vec2(self.g1_.y) * vec2(0.0, -other.g4_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g9_.z) + vec2(self.g1_.z) * vec2(0.0, -other.g4_.z));
}

Flector line_antiWedgeDot_plane(Line self, Plane other) {
    return Flector(vec4(self.g0_.x) * vec4(other.g0_.w, 0.0, 0.0, -other.g0_.x) + vec4(self.g0_.y) * vec4(0.0, other.g0_.w, 0.0, -other.g0_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, other.g0_.w, -other.g0_.z) + vec4(self.g1_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g1_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g1_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0), vec4(self.g0_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g0_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g0_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0) + vec4(self.g1_.x) * vec4(0.0, 0.0, 0.0, -other.g0_.x) + vec4(self.g1_.y) * vec4(0.0, 0.0, 0.0, -other.g0_.y) + vec4(self.g1_.z) * vec4(0.0, 0.0, 0.0, -other.g0_.z));
}

MultiVector line_antiWedgeDot_roundPoint(Line self, RoundPoint other) {
    return MultiVector(vec2(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) + self.g1_ * vec3(other.g1_.x), vec2(self.g1_.x) * vec2(0.0, other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, other.g0_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(other.g1_.x, 0.0, 0.0, -other.g0_.x) + vec4(self.g0_.y) * vec4(0.0, other.g1_.x, 0.0, -other.g0_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, other.g1_.x, -other.g0_.z), self.g1_ * vec3(other.g1_.x), self.g0_ * vec3(other.g1_.y) + vec3(self.g1_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g1_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g1_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec3(0.0), vec2(0.0));
}

Dipole line_antiWedgeDot_scalar(Line self, Scalar other) {
    return Dipole(vec3(0.0), self.g0_ * vec3(other.g0_), vec4(self.g1_.x, self.g1_.y, self.g1_.z, self.g1_.x) * vec4(other.g0_, other.g0_, other.g0_, 0.0));
}

MultiVector line_antiWedgeDot_sphere(Line self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), self.g0_ * vec3(other.g1_.x), self.g1_ * vec3(other.g1_.x), vec4(self.g0_.x) * vec4(other.g1_.y, 0.0, 0.0, -other.g0_.x) + vec4(self.g0_.y) * vec4(0.0, other.g1_.y, 0.0, -other.g0_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, other.g1_.y, -other.g0_.z) + vec4(self.g1_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g1_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g1_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) - self.g1_ * vec3(other.g1_.x), vec2(self.g1_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g0_.z));
}

Motor motor_antiWedgeDot_antiScalar(Motor self, AntiScalar other) {
    return Motor(self.g0_ * vec4(other.g0_), self.g1_ * vec3(other.g0_));
}

MultiVector motor_antiWedgeDot_circle(Motor self, Circle other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g1_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g0_.z), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_.w) + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec2(0.0) - vec2(self.g0_.x) * vec2(other.g0_.x, other.g2_.x) - vec2(self.g0_.y) * vec2(other.g0_.y, other.g2_.y) - vec2(self.g0_.z) * vec2(other.g0_.z, other.g2_.z) + vec2(self.g1_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g1_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g0_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g0_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0) + vec4(self.g0_.w) * other.g0_ + vec4(self.g1_.x) * vec4(0.0, 0.0, 0.0, -other.g0_.x) + vec4(self.g1_.y) * vec4(0.0, 0.0, 0.0, -other.g0_.y) + vec4(self.g1_.z) * vec4(0.0, 0.0, 0.0, -other.g0_.z), vec3(self.g0_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g0_.w) * other.g1_ + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(self.g0_.x) * vec3(0.0, -other.g2_.z, other.g2_.y) + vec3(self.g0_.y) * vec3(other.g2_.z, 0.0, -other.g2_.x) + vec3(self.g0_.z) * vec3(-other.g2_.y, other.g2_.x, 0.0) + vec3(self.g0_.w) * other.g2_ + vec3(self.g1_.x) * vec3(-other.g0_.w, -other.g1_.z, other.g1_.y) + vec3(self.g1_.y) * vec3(other.g1_.z, -other.g0_.w, -other.g1_.x) + vec3(self.g1_.z) * vec3(-other.g1_.y, other.g1_.x, -other.g0_.w), vec3(0.0), vec2(0.0));
}

MultiVector motor_antiWedgeDot_dipole(Motor self, Dipole other) {
    return MultiVector(vec2(self.g0_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g1_.z, 0.0) + vec2(self.g1_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g1_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g1_.z) * vec2(-other.g0_.z, 0.0), vec3(0.0), vec2(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) + vec3(self.g0_.w) * other.g0_, vec3(self.g0_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g0_.w) * other.g1_ + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec4(self.g0_.x) * vec4(0.0, -other.g2_.z, other.g2_.y, 0.0) + vec4(self.g0_.y) * vec4(other.g2_.z, 0.0, -other.g2_.x, 0.0) + vec4(self.g0_.z) * vec4(-other.g2_.y, other.g2_.x, 0.0, 0.0) + vec4(self.g0_.w) * other.g2_ + vec4(self.g1_.x) * vec4(other.g2_.w, -other.g1_.z, other.g1_.y, other.g0_.x) + vec4(self.g1_.y) * vec4(other.g1_.z, other.g2_.w, -other.g1_.x, other.g0_.y) + vec4(self.g1_.z) * vec4(-other.g1_.y, other.g1_.x, other.g2_.w, other.g0_.z), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g2_.w) + vec3(self.g1_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g1_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g1_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec2(0.0) - vec2(self.g0_.x) * vec2(other.g0_.x, other.g2_.x) - vec2(self.g0_.y) * vec2(other.g0_.y, other.g2_.y) - vec2(self.g0_.z) * vec2(other.g0_.z, other.g2_.z) + vec2(self.g1_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g1_.z));
}

MultiVector motor_antiWedgeDot_dualNum(Motor self, DualNum other) {
    return MultiVector(vec2(self.g0_.w) * other.g0_, vec3(0.0), vec2(0.0), vec3(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_.x), vec4(self.g1_.x, self.g1_.y, self.g1_.z, self.g1_.x) * vec4(other.g0_.x, other.g0_.x, other.g0_.x, 0.0), vec4(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_.y), self.g1_ * vec3(other.g0_.y), vec3(0.0), vec2(0.0));
}

Flector motor_antiWedgeDot_flatPoint(Motor self, FlatPoint other) {
    return Flector(vec4(self.g0_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g0_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g0_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0) + vec4(self.g0_.w) * other.g0_ + vec4(self.g1_.x, self.g1_.y, self.g1_.z, self.g1_.x) * vec4(other.g0_.w, other.g0_.w, other.g0_.w, 0.0), vec4(self.g0_.x) * vec4(other.g0_.w, 0.0, 0.0, -other.g0_.x) + vec4(self.g0_.y) * vec4(0.0, other.g0_.w, 0.0, -other.g0_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, other.g0_.w, -other.g0_.z));
}

Flector motor_antiWedgeDot_flector(Motor self, Flector other) {
    return Flector(vec4(self.g0_.x) * vec4(other.g1_.w, -other.g0_.z, other.g0_.y, -other.g1_.x) + vec4(self.g0_.y) * vec4(other.g0_.z, other.g1_.w, -other.g0_.x, -other.g1_.y) + vec4(self.g0_.z) * vec4(-other.g0_.y, other.g0_.x, other.g1_.w, -other.g1_.z) + vec4(self.g0_.w) * other.g0_ + vec4(self.g1_.x) * vec4(other.g0_.w, -other.g1_.z, other.g1_.y, 0.0) + vec4(self.g1_.y) * vec4(other.g1_.z, other.g0_.w, -other.g1_.x, 0.0) + vec4(self.g1_.z) * vec4(-other.g1_.y, other.g1_.x, other.g0_.w, 0.0), vec4(self.g0_.x) * vec4(other.g0_.w, -other.g1_.z, other.g1_.y, -other.g0_.x) + vec4(self.g0_.y) * vec4(other.g1_.z, other.g0_.w, -other.g1_.x, -other.g0_.y) + vec4(self.g0_.z) * vec4(-other.g1_.y, other.g1_.x, other.g0_.w, -other.g0_.z) + vec4(self.g0_.w) * other.g1_ + vec4(self.g1_.x) * vec4(0.0, 0.0, 0.0, -other.g1_.x) + vec4(self.g1_.y) * vec4(0.0, 0.0, 0.0, -other.g1_.y) + vec4(self.g1_.z) * vec4(0.0, 0.0, 0.0, -other.g1_.z));
}

MultiVector motor_antiWedgeDot_line(Motor self, Line other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g0_.z), vec3(0.0), vec2(self.g0_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g1_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g0_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) + vec3(self.g0_.w) * other.g0_, vec3(self.g0_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g0_.w) * other.g1_ + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(0.0), vec2(0.0));
}

MultiVector motor_antiWedgeDot_motor(Motor self, Motor other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g0_.w) * vec2(0.0, other.g0_.w), vec3(0.0), vec2(self.g0_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g1_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g0_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0_.x) * vec3(other.g0_.w, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, other.g0_.w, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, other.g0_.w) + vec3(self.g0_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(self.g0_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g0_.w) * other.g1_ + vec3(self.g1_.x) * vec3(other.g0_.w, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, other.g0_.w, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, other.g0_.w), vec3(0.0), vec2(0.0));
}

MultiVector motor_antiWedgeDot_multiVector(Motor self, MultiVector other) {
    return MultiVector(vec2(0.0) - vec2(self.g0_.x) * vec2(other.g4_.x, other.g7_.x) - vec2(self.g0_.y) * vec2(other.g4_.y, other.g7_.y) - vec2(self.g0_.z) * vec2(other.g4_.z, other.g7_.z) + vec2(self.g0_.w) * other.g0_ - vec2(self.g1_.x) * vec2(other.g3_.x, other.g6_.x) - vec2(self.g1_.y) * vec2(other.g3_.y, other.g6_.y) - vec2(self.g1_.z) * vec2(other.g3_.z, other.g6_.z), vec3(self.g0_.x) * vec3(other.g6_.w, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, other.g6_.w, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, other.g6_.w) + vec3(self.g0_.w) * other.g1_ + vec3(self.g1_.x) * vec3(other.g2_.x, -other.g6_.z, other.g6_.y) + vec3(self.g1_.y) * vec3(other.g6_.z, other.g2_.x, -other.g6_.x) + vec3(self.g1_.z) * vec3(-other.g6_.y, other.g6_.x, other.g2_.x), vec2(0.0) - vec2(self.g0_.x) * vec2(other.g6_.x, other.g8_.x) - vec2(self.g0_.y) * vec2(other.g6_.y, other.g8_.y) - vec2(self.g0_.z) * vec2(other.g6_.z, other.g8_.z) + vec2(self.g0_.w) * other.g2_ + vec2(self.g1_.x) * vec2(0.0, -other.g7_.x) + vec2(self.g1_.x) * vec2(0.0, other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g7_.y) + vec2(self.g1_.y) * vec2(0.0, other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g7_.z) + vec2(self.g1_.z) * vec2(0.0, other.g1_.z), vec3(self.g0_.x) * vec3(other.g10_.x, -other.g3_.z, other.g3_.y) + vec3(self.g0_.y) * vec3(other.g3_.z, other.g10_.x, -other.g3_.x) + vec3(self.g0_.z) * vec3(-other.g3_.y, other.g3_.x, other.g10_.x) + vec3(self.g0_.w) * other.g3_, vec3(self.g0_.x) * vec3(other.g0_.x, -other.g4_.z, other.g4_.y) + vec3(self.g0_.y) * vec3(other.g4_.z, other.g0_.x, -other.g4_.x) + vec3(self.g0_.z) * vec3(-other.g4_.y, other.g4_.x, other.g0_.x) + vec3(self.g0_.w) * other.g4_ + vec3(self.g1_.x) * vec3(other.g10_.x, -other.g3_.z, other.g3_.y) + vec3(self.g1_.y) * vec3(other.g3_.z, other.g10_.x, -other.g3_.x) + vec3(self.g1_.z) * vec3(-other.g3_.y, other.g3_.x, other.g10_.x), vec4(self.g0_.x) * vec4(other.g10_.y, -other.g5_.z, other.g5_.y, -other.g9_.x) + vec4(self.g0_.y) * vec4(other.g5_.z, other.g10_.y, -other.g5_.x, -other.g9_.y) + vec4(self.g0_.z) * vec4(-other.g5_.y, other.g5_.x, other.g10_.y, -other.g9_.z) + vec4(self.g0_.w) * other.g5_ + vec4(self.g1_.x) * vec4(other.g5_.w, -other.g9_.z, other.g9_.y, other.g3_.x) + vec4(self.g1_.x) * vec4(other.g0_.x, -other.g4_.z, other.g4_.y, 0.0) + vec4(self.g1_.y) * vec4(other.g9_.z, other.g5_.w, -other.g9_.x, other.g3_.y) + vec4(self.g1_.y) * vec4(other.g4_.z, other.g0_.x, -other.g4_.x, 0.0) + vec4(self.g1_.z) * vec4(-other.g9_.y, other.g9_.x, other.g5_.w, other.g3_.z) + vec4(self.g1_.z) * vec4(-other.g4_.y, other.g4_.x, other.g0_.x, 0.0), vec4(self.g0_.x) * vec4(other.g2_.x, -other.g6_.z, other.g6_.y, -other.g1_.x) + vec4(self.g0_.y) * vec4(other.g6_.z, other.g2_.x, -other.g6_.x, -other.g1_.y) + vec4(self.g0_.z) * vec4(-other.g6_.y, other.g6_.x, other.g2_.x, -other.g1_.z) + vec4(self.g0_.w) * other.g6_ + vec4(self.g1_.x) * vec4(0.0, 0.0, 0.0, -other.g6_.x) + vec4(self.g1_.y) * vec4(0.0, 0.0, 0.0, -other.g6_.y) + vec4(self.g1_.z) * vec4(0.0, 0.0, 0.0, -other.g6_.z), vec3(self.g0_.x) * vec3(other.g0_.y, -other.g7_.z, other.g7_.y) + vec3(self.g0_.y) * vec3(other.g7_.z, other.g0_.y, -other.g7_.x) + vec3(self.g0_.z) * vec3(-other.g7_.y, other.g7_.x, other.g0_.y) + vec3(self.g0_.w) * other.g7_ + vec3(self.g1_.x) * vec3(other.g2_.x, -other.g6_.z, other.g6_.y) + vec3(self.g1_.y) * vec3(other.g6_.z, other.g2_.x, -other.g6_.x) + vec3(self.g1_.z) * vec3(-other.g6_.y, other.g6_.x, other.g2_.x), vec3(self.g0_.x) * vec3(other.g2_.y, -other.g8_.z, other.g8_.y) + vec3(self.g0_.y) * vec3(other.g8_.z, other.g2_.y, -other.g8_.x) + vec3(self.g0_.z) * vec3(-other.g8_.y, other.g8_.x, other.g2_.y) + vec3(self.g0_.w) * other.g8_ + vec3(self.g1_.x) * vec3(-other.g6_.w, -other.g7_.z, other.g7_.y) + vec3(self.g1_.x) * vec3(other.g0_.y, other.g1_.z, -other.g1_.y) + vec3(self.g1_.y) * vec3(other.g7_.z, -other.g6_.w, -other.g7_.x) + vec3(self.g1_.y) * vec3(-other.g1_.z, other.g0_.y, other.g1_.x) + vec3(self.g1_.z) * vec3(-other.g7_.y, other.g7_.x, -other.g6_.w) + vec3(self.g1_.z) * vec3(other.g1_.y, -other.g1_.x, other.g0_.y), vec3(self.g0_.x) * vec3(other.g5_.w, -other.g9_.z, other.g9_.y) + vec3(self.g0_.y) * vec3(other.g9_.z, other.g5_.w, -other.g9_.x) + vec3(self.g0_.z) * vec3(-other.g9_.y, other.g9_.x, other.g5_.w) + vec3(self.g0_.w) * other.g9_ + vec3(self.g1_.x) * vec3(-other.g10_.x, other.g3_.z, -other.g3_.y) + vec3(self.g1_.y) * vec3(-other.g3_.z, -other.g10_.x, other.g3_.x) + vec3(self.g1_.z) * vec3(other.g3_.y, -other.g3_.x, -other.g10_.x), vec2(0.0) - vec2(self.g0_.x) * vec2(other.g3_.x, other.g5_.x) - vec2(self.g0_.y) * vec2(other.g3_.y, other.g5_.y) - vec2(self.g0_.z) * vec2(other.g3_.z, other.g5_.z) + vec2(self.g0_.w) * other.g10_ + vec2(self.g1_.x) * vec2(0.0, -other.g9_.x) + vec2(self.g1_.x) * vec2(0.0, -other.g4_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g9_.y) + vec2(self.g1_.y) * vec2(0.0, -other.g4_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g9_.z) + vec2(self.g1_.z) * vec2(0.0, -other.g4_.z));
}

Flector motor_antiWedgeDot_plane(Motor self, Plane other) {
    return Flector(vec4(self.g0_.x) * vec4(other.g0_.w, 0.0, 0.0, -other.g0_.x) + vec4(self.g0_.y) * vec4(0.0, other.g0_.w, 0.0, -other.g0_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, other.g0_.w, -other.g0_.z) + vec4(self.g1_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g1_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g1_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0), vec4(self.g0_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g0_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g0_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0) + vec4(self.g0_.w) * other.g0_ + vec4(self.g1_.x) * vec4(0.0, 0.0, 0.0, -other.g0_.x) + vec4(self.g1_.y) * vec4(0.0, 0.0, 0.0, -other.g0_.y) + vec4(self.g1_.z) * vec4(0.0, 0.0, 0.0, -other.g0_.z));
}

MultiVector motor_antiWedgeDot_roundPoint(Motor self, RoundPoint other) {
    return MultiVector(vec2(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) + vec3(self.g0_.w) * other.g0_ + self.g1_ * vec3(other.g1_.x), vec2(self.g0_.w) * other.g1_ + vec2(self.g1_.x) * vec2(0.0, other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, other.g0_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(other.g1_.x, 0.0, 0.0, -other.g0_.x) + vec4(self.g0_.y) * vec4(0.0, other.g1_.x, 0.0, -other.g0_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, other.g1_.x, -other.g0_.z), self.g1_ * vec3(other.g1_.x), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g1_.y) + vec3(self.g1_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g1_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g1_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec3(0.0), vec2(0.0));
}

MultiVector motor_antiWedgeDot_scalar(Motor self, Scalar other) {
    return MultiVector(vec2(self.g0_.w) * vec2(other.g0_, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_), vec4(self.g1_.x, self.g1_.y, self.g1_.z, self.g1_.x) * vec4(other.g0_, other.g0_, other.g0_, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector motor_antiWedgeDot_sphere(Motor self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g1_.x), self.g1_ * vec3(other.g1_.x), vec4(self.g0_.x) * vec4(other.g1_.y, 0.0, 0.0, -other.g0_.x) + vec4(self.g0_.y) * vec4(0.0, other.g1_.y, 0.0, -other.g0_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, other.g1_.y, -other.g0_.z) + vec4(self.g1_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g1_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g1_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) + vec3(self.g0_.w) * other.g0_ - self.g1_ * vec3(other.g1_.x), vec2(self.g0_.w) * other.g1_ + vec2(self.g1_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g0_.z));
}

MultiVector multiVector_antiWedgeDot_antiScalar(MultiVector self, AntiScalar other) {
    return MultiVector(self.g0_ * vec2(other.g0_), self.g1_ * vec3(other.g0_), self.g2_ * vec2(other.g0_), self.g3_ * vec3(other.g0_), self.g4_ * vec3(other.g0_), self.g5_ * vec4(other.g0_), self.g6_ * vec4(other.g0_), self.g7_ * vec3(other.g0_), self.g8_ * vec3(other.g0_), self.g9_ * vec3(other.g0_), self.g10_ * vec2(other.g0_));
}

MultiVector multiVector_antiWedgeDot_circle(MultiVector self, Circle other) {
    return MultiVector(vec2(self.g3_.x) * vec2(-other.g2_.x, 0.0) + vec2(self.g3_.y) * vec2(-other.g2_.y, 0.0) + vec2(self.g3_.z) * vec2(-other.g2_.z, 0.0) + vec2(self.g4_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g4_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g4_.z) * vec2(-other.g1_.z, 0.0) + vec2(self.g5_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g5_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g5_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g5_.w) * vec2(-other.g0_.w, 0.0) + vec2(self.g6_.x) * vec2(0.0, -other.g2_.x) + vec2(self.g6_.y) * vec2(0.0, -other.g2_.y) + vec2(self.g6_.z) * vec2(0.0, -other.g2_.z) + vec2(self.g6_.w) * vec2(0.0, other.g0_.w) + vec2(self.g7_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g7_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g7_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g8_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g8_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g8_.z) * vec2(0.0, -other.g0_.z), vec3(self.g1_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g1_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g1_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) - vec3(self.g2_.x) * other.g2_ + vec3(self.g2_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g6_.x) * vec3(0.0, other.g2_.z, -other.g2_.y) + vec3(self.g6_.y) * vec3(-other.g2_.z, 0.0, other.g2_.x) + vec3(self.g6_.z) * vec3(other.g2_.y, -other.g2_.x, 0.0) + vec3(self.g6_.w) * other.g1_ + self.g7_ * vec3(other.g0_.w) + vec3(self.g8_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g8_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g8_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec2(self.g1_.x) * vec2(other.g0_.x, -other.g2_.x) + vec2(self.g1_.y) * vec2(other.g0_.y, -other.g2_.y) + vec2(self.g1_.z) * vec2(other.g0_.z, -other.g2_.z) + self.g2_ * vec2(other.g0_.w) + vec2(self.g6_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g6_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g6_.z) * vec2(-other.g1_.z, 0.0) - vec2(self.g7_.x) * vec2(other.g0_.x, other.g2_.x) - vec2(self.g7_.y) * vec2(other.g0_.y, other.g2_.y) - vec2(self.g7_.z) * vec2(other.g0_.z, other.g2_.z) + vec2(self.g8_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g8_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g8_.z) * vec2(0.0, -other.g1_.z), vec3(self.g0_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g3_.x) * vec3(other.g0_.w, -other.g1_.z, other.g1_.y) + vec3(self.g3_.y) * vec3(other.g1_.z, other.g0_.w, -other.g1_.x) + vec3(self.g3_.z) * vec3(-other.g1_.y, other.g1_.x, other.g0_.w) + vec3(self.g4_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g4_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g4_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) + vec3(self.g5_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g9_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g9_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g9_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) + vec3(self.g10_.x) * other.g1_, vec3(self.g0_.x) * other.g1_ + vec3(self.g3_.x) * vec3(0.0, -other.g2_.z, other.g2_.y) + vec3(self.g3_.y) * vec3(other.g2_.z, 0.0, -other.g2_.x) + vec3(self.g3_.z) * vec3(-other.g2_.y, other.g2_.x, 0.0) + vec3(self.g4_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g4_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g4_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g5_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g5_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g5_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) - self.g9_ * vec3(other.g0_.w) + vec3(self.g10_.x) * other.g2_ + vec3(self.g10_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec4(self.g0_.x) * vec4(other.g2_.x, other.g2_.y, other.g2_.z, -other.g0_.w) + vec4(self.g3_.x) * vec4(0.0, 0.0, 0.0, -other.g2_.x) + vec4(self.g3_.y) * vec4(0.0, 0.0, 0.0, -other.g2_.y) + vec4(self.g3_.z) * vec4(0.0, 0.0, 0.0, -other.g2_.z) + vec4(self.g4_.x) * vec4(0.0, -other.g2_.z, other.g2_.y, 0.0) + vec4(self.g4_.y) * vec4(other.g2_.z, 0.0, -other.g2_.x, 0.0) + vec4(self.g4_.z) * vec4(-other.g2_.y, other.g2_.x, 0.0, 0.0) + vec4(self.g5_.x) * vec4(-other.g0_.w, -other.g1_.z, other.g1_.y, other.g0_.x) + vec4(self.g5_.y) * vec4(other.g1_.z, -other.g0_.w, -other.g1_.x, other.g0_.y) + vec4(self.g5_.z) * vec4(-other.g1_.y, other.g1_.x, -other.g0_.w, other.g0_.z) + vec4(self.g5_.w) * vec4(-other.g2_.x, -other.g2_.y, -other.g2_.z, 0.0) + vec4(self.g9_.x) * vec4(0.0, other.g2_.z, -other.g2_.y, -other.g1_.x) + vec4(self.g9_.y) * vec4(-other.g2_.z, 0.0, other.g2_.x, -other.g1_.y) + vec4(self.g9_.z) * vec4(other.g2_.y, -other.g2_.x, 0.0, -other.g1_.z) + vec4(self.g10_.y) * vec4(other.g1_.x, other.g1_.y, other.g1_.z, 0.0), vec4(self.g0_.y) * other.g0_ + vec4(self.g1_.x) * vec4(0.0, other.g0_.z, -other.g0_.y, -other.g1_.x) + vec4(self.g1_.y) * vec4(-other.g0_.z, 0.0, other.g0_.x, -other.g1_.y) + vec4(self.g1_.z) * vec4(other.g0_.y, -other.g0_.x, 0.0, -other.g1_.z) + vec4(self.g2_.x) * vec4(other.g1_.x, other.g1_.y, other.g1_.z, 0.0) + vec4(self.g6_.x) * vec4(other.g0_.w, -other.g1_.z, other.g1_.y, other.g2_.x) + vec4(self.g6_.y) * vec4(other.g1_.z, other.g0_.w, -other.g1_.x, other.g2_.y) + vec4(self.g6_.z) * vec4(-other.g1_.y, other.g1_.x, other.g0_.w, other.g2_.z) + vec4(self.g6_.w) * vec4(-other.g0_.x, -other.g0_.y, -other.g0_.z, 0.0) + vec4(self.g7_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g7_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g7_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0) + vec4(self.g8_.x) * vec4(0.0, 0.0, 0.0, -other.g0_.x) + vec4(self.g8_.y) * vec4(0.0, 0.0, 0.0, -other.g0_.y) + vec4(self.g8_.z) * vec4(0.0, 0.0, 0.0, -other.g0_.z), vec3(self.g0_.y) * other.g1_ + self.g1_ * vec3(other.g0_.w) + vec3(self.g2_.x) * other.g2_ + vec3(self.g2_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g6_.x) * vec3(0.0, -other.g2_.z, other.g2_.y) + vec3(self.g6_.y) * vec3(other.g2_.z, 0.0, -other.g2_.x) + vec3(self.g6_.z) * vec3(-other.g2_.y, other.g2_.x, 0.0) + vec3(self.g7_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g7_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g7_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g8_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g8_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g8_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(self.g0_.y) * other.g2_ + vec3(self.g1_.x) * vec3(0.0, -other.g2_.z, other.g2_.y) + vec3(self.g1_.y) * vec3(other.g2_.z, 0.0, -other.g2_.x) + vec3(self.g1_.z) * vec3(-other.g2_.y, other.g2_.x, 0.0) + vec3(self.g2_.y) * other.g1_ + vec3(self.g6_.w) * other.g2_ + vec3(self.g7_.x) * vec3(0.0, -other.g2_.z, other.g2_.y) + vec3(self.g7_.y) * vec3(other.g2_.z, 0.0, -other.g2_.x) + vec3(self.g7_.z) * vec3(-other.g2_.y, other.g2_.x, 0.0) + vec3(self.g8_.x) * vec3(-other.g0_.w, -other.g1_.z, other.g1_.y) + vec3(self.g8_.y) * vec3(other.g1_.z, -other.g0_.w, -other.g1_.x) + vec3(self.g8_.z) * vec3(-other.g1_.y, other.g1_.x, -other.g0_.w), vec3(self.g3_.x) * vec3(0.0, -other.g2_.z, other.g2_.y) + vec3(self.g3_.y) * vec3(other.g2_.z, 0.0, -other.g2_.x) + vec3(self.g3_.z) * vec3(-other.g2_.y, other.g2_.x, 0.0) - self.g4_ * vec3(other.g0_.w) + vec3(self.g5_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g5_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g5_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + vec3(self.g5_.w) * other.g1_ + vec3(self.g9_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g9_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g9_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g10_.x) * other.g2_ - vec3(self.g10_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g3_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g3_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g3_.z) * vec2(-other.g1_.z, 0.0) - vec2(self.g4_.x) * vec2(other.g0_.x, other.g2_.x) - vec2(self.g4_.y) * vec2(other.g0_.y, other.g2_.y) - vec2(self.g4_.z) * vec2(other.g0_.z, other.g2_.z) + vec2(self.g5_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g5_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g5_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g9_.x) * vec2(-other.g0_.x, other.g2_.x) + vec2(self.g9_.y) * vec2(-other.g0_.y, other.g2_.y) + vec2(self.g9_.z) * vec2(-other.g0_.z, other.g2_.z) + self.g10_ * vec2(other.g0_.w));
}

MultiVector multiVector_antiWedgeDot_dipole(MultiVector self, Dipole other) {
    return MultiVector(vec2(self.g3_.x) * vec2(0.0, other.g2_.x) + vec2(self.g3_.y) * vec2(0.0, other.g2_.y) + vec2(self.g3_.z) * vec2(0.0, other.g2_.z) + vec2(self.g4_.x) * vec2(0.0, other.g1_.x) + vec2(self.g4_.y) * vec2(0.0, other.g1_.y) + vec2(self.g4_.z) * vec2(0.0, other.g1_.z) + vec2(self.g5_.x) * vec2(0.0, other.g0_.x) + vec2(self.g5_.y) * vec2(0.0, other.g0_.y) + vec2(self.g5_.z) * vec2(0.0, other.g0_.z) + vec2(self.g5_.w) * vec2(0.0, -other.g2_.w) + vec2(self.g6_.x) * vec2(-other.g2_.x, 0.0) + vec2(self.g6_.y) * vec2(-other.g2_.y, 0.0) + vec2(self.g6_.z) * vec2(-other.g2_.z, 0.0) + vec2(self.g6_.w) * vec2(-other.g2_.w, 0.0) + vec2(self.g7_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g7_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g7_.z) * vec2(-other.g1_.z, 0.0) + vec2(self.g8_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g8_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g8_.z) * vec2(-other.g0_.z, 0.0), vec3(self.g3_.x) * vec3(0.0, -other.g2_.z, other.g2_.y) + vec3(self.g3_.y) * vec3(other.g2_.z, 0.0, -other.g2_.x) + vec3(self.g3_.z) * vec3(-other.g2_.y, other.g2_.x, 0.0) + self.g4_ * vec3(other.g2_.w) + vec3(self.g5_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g5_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g5_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + vec3(self.g5_.w) * other.g1_ + vec3(self.g9_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g9_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g9_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g10_.x) * vec3(other.g2_.x, other.g2_.y, other.g2_.z) - vec3(self.g10_.y) * other.g0_, vec2(self.g3_.x) * vec2(other.g1_.x, 0.0) + vec2(self.g3_.y) * vec2(other.g1_.y, 0.0) + vec2(self.g3_.z) * vec2(other.g1_.z, 0.0) + vec2(self.g4_.x) * vec2(other.g0_.x, other.g2_.x) + vec2(self.g4_.y) * vec2(other.g0_.y, other.g2_.y) + vec2(self.g4_.z) * vec2(other.g0_.z, other.g2_.z) + vec2(self.g5_.x) * vec2(0.0, other.g1_.x) + vec2(self.g5_.y) * vec2(0.0, other.g1_.y) + vec2(self.g5_.z) * vec2(0.0, other.g1_.z) + vec2(self.g9_.x) * vec2(other.g0_.x, -other.g2_.x) + vec2(self.g9_.y) * vec2(other.g0_.y, -other.g2_.y) + vec2(self.g9_.z) * vec2(other.g0_.z, -other.g2_.z) + self.g10_ * vec2(other.g2_.w), vec3(self.g0_.y) * other.g0_ + vec3(self.g1_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g1_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g1_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + vec3(self.g2_.x) * other.g1_ + vec3(self.g6_.x) * vec3(-other.g2_.w, -other.g1_.z, other.g1_.y) + vec3(self.g6_.y) * vec3(other.g1_.z, -other.g2_.w, -other.g1_.x) + vec3(self.g6_.z) * vec3(-other.g1_.y, other.g1_.x, -other.g2_.w) - vec3(self.g6_.w) * other.g0_ + vec3(self.g7_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g7_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g7_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(self.g0_.y) * other.g1_ - self.g1_ * vec3(other.g2_.w) + vec3(self.g2_.x) * vec3(other.g2_.x, other.g2_.y, other.g2_.z) + vec3(self.g2_.y) * other.g0_ + vec3(self.g6_.x) * vec3(0.0, -other.g2_.z, other.g2_.y) + vec3(self.g6_.y) * vec3(other.g2_.z, 0.0, -other.g2_.x) + vec3(self.g6_.z) * vec3(-other.g2_.y, other.g2_.x, 0.0) + vec3(self.g7_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g7_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g7_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g8_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g8_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g8_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec4(self.g0_.y) * other.g2_ + vec4(self.g1_.x) * vec4(0.0, -other.g2_.z, other.g2_.y, other.g1_.x) + vec4(self.g1_.y) * vec4(other.g2_.z, 0.0, -other.g2_.x, other.g1_.y) + vec4(self.g1_.z) * vec4(-other.g2_.y, other.g2_.x, 0.0, other.g1_.z) + vec4(self.g2_.y) * vec4(other.g1_.x, other.g1_.y, other.g1_.z, 0.0) + vec4(self.g6_.x) * vec4(0.0, 0.0, 0.0, -other.g2_.x) + vec4(self.g6_.y) * vec4(0.0, 0.0, 0.0, -other.g2_.y) + self.g6_.wwwz * other.g2_.xyzz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g7_.x) * vec4(0.0, -other.g2_.z, other.g2_.y, 0.0) + vec4(self.g7_.y) * vec4(other.g2_.z, 0.0, -other.g2_.x, 0.0) + vec4(self.g7_.z) * vec4(-other.g2_.y, other.g2_.x, 0.0, 0.0) + vec4(self.g8_.x) * vec4(other.g2_.w, -other.g1_.z, other.g1_.y, other.g0_.x) + vec4(self.g8_.y) * vec4(other.g1_.z, other.g2_.w, -other.g1_.x, other.g0_.y) + vec4(self.g8_.z) * vec4(-other.g1_.y, other.g1_.x, other.g2_.w, other.g0_.z), vec4(self.g0_.x) * vec4(-other.g0_.x, -other.g0_.y, -other.g0_.z, other.g2_.w) + vec4(self.g3_.x) * vec4(other.g2_.w, other.g1_.z, -other.g1_.y, -other.g2_.x) + vec4(self.g3_.y) * vec4(-other.g1_.z, other.g2_.w, other.g1_.x, -other.g2_.y) + vec4(self.g3_.z) * vec4(other.g1_.y, -other.g1_.x, other.g2_.w, -other.g2_.z) + vec4(self.g4_.x) * vec4(0.0, other.g0_.z, -other.g0_.y, 0.0) + vec4(self.g4_.y) * vec4(-other.g0_.z, 0.0, other.g0_.x, 0.0) + vec4(self.g4_.z) * vec4(other.g0_.y, -other.g0_.x, 0.0, 0.0) + vec4(self.g5_.x) * vec4(0.0, 0.0, 0.0, other.g0_.x) + vec4(self.g5_.y) * vec4(0.0, 0.0, 0.0, other.g0_.y) + self.g5_.wwwz * vec4(-other.g0_.x, -other.g0_.y, -other.g0_.z, other.g0_.z) + vec4(self.g9_.x) * vec4(0.0, other.g0_.z, -other.g0_.y, -other.g1_.x) + vec4(self.g9_.y) * vec4(-other.g0_.z, 0.0, other.g0_.x, -other.g1_.y) + vec4(self.g9_.z) * vec4(other.g0_.y, -other.g0_.x, 0.0, -other.g1_.z) + vec4(self.g10_.x) * vec4(-other.g1_.x, -other.g1_.y, -other.g1_.z, 0.0), vec3(0.0) - vec3(self.g0_.x) * other.g1_ + vec3(self.g3_.x) * vec3(0.0, other.g2_.z, -other.g2_.y) + vec3(self.g3_.y) * vec3(-other.g2_.z, 0.0, other.g2_.x) + vec3(self.g3_.z) * vec3(other.g2_.y, -other.g2_.x, 0.0) + vec3(self.g4_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g4_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g4_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g5_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g5_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g5_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) - self.g9_ * vec3(other.g2_.w) - vec3(self.g10_.x) * vec3(other.g2_.x, other.g2_.y, other.g2_.z) - vec3(self.g10_.y) * other.g0_, vec3(0.0) - vec3(self.g0_.x) * vec3(other.g2_.x, other.g2_.y, other.g2_.z) + vec3(self.g4_.x) * vec3(0.0, other.g2_.z, -other.g2_.y) + vec3(self.g4_.y) * vec3(-other.g2_.z, 0.0, other.g2_.x) + vec3(self.g4_.z) * vec3(other.g2_.y, -other.g2_.x, 0.0) + vec3(self.g5_.x) * vec3(-other.g2_.w, other.g1_.z, -other.g1_.y) + vec3(self.g5_.y) * vec3(-other.g1_.z, -other.g2_.w, other.g1_.x) + vec3(self.g5_.z) * vec3(other.g1_.y, -other.g1_.x, -other.g2_.w) + vec3(self.g5_.w) * vec3(other.g2_.x, other.g2_.y, other.g2_.z) + vec3(self.g9_.x) * vec3(0.0, -other.g2_.z, other.g2_.y) + vec3(self.g9_.y) * vec3(other.g2_.z, 0.0, -other.g2_.x) + vec3(self.g9_.z) * vec3(-other.g2_.y, other.g2_.x, 0.0) - vec3(self.g10_.y) * other.g1_, vec3(self.g1_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g1_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g1_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g2_.x) * vec3(other.g2_.x, other.g2_.y, other.g2_.z) - vec3(self.g2_.y) * other.g0_ + vec3(self.g6_.x) * vec3(0.0, -other.g2_.z, other.g2_.y) + vec3(self.g6_.y) * vec3(other.g2_.z, 0.0, -other.g2_.x) + vec3(self.g6_.z) * vec3(-other.g2_.y, other.g2_.x, 0.0) - vec3(self.g6_.w) * other.g1_ + self.g7_ * vec3(other.g2_.w) + vec3(self.g8_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g8_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g8_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec2(self.g1_.x) * vec2(other.g0_.x, -other.g2_.x) + vec2(self.g1_.y) * vec2(other.g0_.y, -other.g2_.y) + vec2(self.g1_.z) * vec2(other.g0_.z, -other.g2_.z) + self.g2_ * vec2(-other.g2_.w) + vec2(self.g6_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g6_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g6_.z) * vec2(-other.g1_.z, 0.0) - vec2(self.g7_.x) * vec2(other.g0_.x, other.g2_.x) - vec2(self.g7_.y) * vec2(other.g0_.y, other.g2_.y) - vec2(self.g7_.z) * vec2(other.g0_.z, other.g2_.z) + vec2(self.g8_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g8_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g8_.z) * vec2(0.0, -other.g1_.z));
}

MultiVector multiVector_antiWedgeDot_dualNum(MultiVector self, DualNum other) {
    return MultiVector(vec2(self.g0_.x) * other.g0_.yx * vec2(1.0, -1.0) + vec2(self.g0_.y) * other.g0_, self.g1_ * vec3(other.g0_.y) + self.g9_ * vec3(other.g0_.x), self.g2_ * vec2(other.g0_.y) - self.g10_ * vec2(other.g0_.x), self.g3_ * vec3(other.g0_.y) + vec3(self.g6_.x, self.g6_.y, self.g6_.z) * vec3(other.g0_.x), self.g4_ * vec3(other.g0_.y) + self.g7_ * vec3(other.g0_.x), self.g5_ * vec4(other.g0_.y) + vec4(self.g6_.w) * vec4(0.0, 0.0, 0.0, -other.g0_.x) + vec4(self.g8_.x, self.g8_.y, self.g8_.z, self.g8_.x) * vec4(other.g0_.x, other.g0_.x, other.g0_.x, 0.0), vec4(self.g3_.x, self.g3_.y, self.g3_.z, self.g3_.x) * vec4(-other.g0_.x, -other.g0_.x, -other.g0_.x, 0.0) + vec4(self.g5_.w) * vec4(0.0, 0.0, 0.0, other.g0_.x) + self.g6_ * vec4(other.g0_.y), vec3(0.0) - self.g4_ * vec3(other.g0_.x) + self.g7_ * vec3(other.g0_.y), vec3(0.0) - vec3(self.g5_.x, self.g5_.y, self.g5_.z) * vec3(other.g0_.x) + self.g8_ * vec3(other.g0_.y), vec3(0.0) - self.g1_ * vec3(other.g0_.x) + self.g9_ * vec3(other.g0_.y), self.g2_ * vec2(other.g0_.x) + self.g10_ * vec2(other.g0_.y));
}

MultiVector multiVector_antiWedgeDot_flatPoint(MultiVector self, FlatPoint other) {
    return MultiVector(vec2(self.g3_.x) * vec2(0.0, other.g0_.x) + vec2(self.g3_.y) * vec2(0.0, other.g0_.y) + vec2(self.g3_.z) * vec2(0.0, other.g0_.z) + vec2(self.g5_.w) * vec2(0.0, -other.g0_.w) + vec2(self.g6_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g6_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g6_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g6_.w) * vec2(-other.g0_.w, 0.0), vec3(self.g3_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g3_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g3_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) + self.g4_ * vec3(other.g0_.w) + vec3(self.g10_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g4_.x) * vec2(0.0, other.g0_.x) + vec2(self.g4_.y) * vec2(0.0, other.g0_.y) + vec2(self.g4_.z) * vec2(0.0, other.g0_.z) + vec2(self.g9_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g9_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g9_.z) * vec2(0.0, -other.g0_.z) + self.g10_ * vec2(other.g0_.w), vec3(0.0) - vec3(self.g6_.x, self.g6_.y, self.g6_.z) * vec3(other.g0_.w), vec3(0.0) - self.g1_ * vec3(other.g0_.w) + vec3(self.g2_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g6_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g6_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g6_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec4(self.g0_.y) * other.g0_ + vec4(self.g1_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g1_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g1_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0) + vec4(self.g6_.x) * vec4(0.0, 0.0, 0.0, -other.g0_.x) + vec4(self.g6_.y) * vec4(0.0, 0.0, 0.0, -other.g0_.y) + self.g6_.wwwz * other.g0_.xyzz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g7_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g7_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g7_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0) + vec4(self.g8_.x, self.g8_.y, self.g8_.z, self.g8_.x) * vec4(other.g0_.w, other.g0_.w, other.g0_.w, 0.0), vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, other.g0_.w) + vec4(self.g3_.x) * vec4(other.g0_.w, 0.0, 0.0, -other.g0_.x) + vec4(self.g3_.y) * vec4(0.0, other.g0_.w, 0.0, -other.g0_.y) + vec4(self.g3_.z) * vec4(0.0, 0.0, other.g0_.w, -other.g0_.z), vec3(self.g3_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g3_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g3_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) - self.g9_ * vec3(other.g0_.w) - vec3(self.g10_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(0.0) - vec3(self.g0_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g4_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g4_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g4_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) - vec3(self.g5_.x, self.g5_.y, self.g5_.z) * vec3(other.g0_.w) + vec3(self.g5_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g9_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g9_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g9_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(self.g2_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g6_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g6_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g6_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) + self.g7_ * vec3(other.g0_.w), vec2(self.g1_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g0_.z) + self.g2_ * vec2(-other.g0_.w) + vec2(self.g7_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g7_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g7_.z) * vec2(0.0, -other.g0_.z));
}

MultiVector multiVector_antiWedgeDot_flector(MultiVector self, Flector other) {
    return MultiVector(vec2(self.g1_.x) * vec2(other.g1_.x, 0.0) + vec2(self.g1_.y) * vec2(other.g1_.y, 0.0) + vec2(self.g1_.z) * vec2(other.g1_.z, 0.0) + vec2(self.g2_.x) * vec2(other.g1_.w, 0.0) + vec2(self.g3_.x) * vec2(0.0, other.g0_.x) + vec2(self.g3_.y) * vec2(0.0, other.g0_.y) + vec2(self.g3_.z) * vec2(0.0, other.g0_.z) + vec2(self.g5_.w) * vec2(0.0, -other.g0_.w) + vec2(self.g6_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g6_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g6_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g6_.w) * vec2(-other.g0_.w, 0.0) + vec2(self.g9_.x) * vec2(0.0, other.g1_.x) + vec2(self.g9_.y) * vec2(0.0, other.g1_.y) + vec2(self.g9_.z) * vec2(0.0, other.g1_.z) + vec2(self.g10_.x) * vec2(0.0, -other.g1_.w), vec3(self.g0_.x) * vec3(other.g1_.x, other.g1_.y, other.g1_.z) + vec3(self.g3_.x) * vec3(other.g1_.w, -other.g0_.z, other.g0_.y) + vec3(self.g3_.y) * vec3(other.g0_.z, other.g1_.w, -other.g0_.x) + vec3(self.g3_.z) * vec3(-other.g0_.y, other.g0_.x, other.g1_.w) + vec3(self.g4_.x) * vec3(other.g0_.w, -other.g1_.z, other.g1_.y) + vec3(self.g4_.y) * vec3(other.g1_.z, other.g0_.w, -other.g1_.x) + vec3(self.g4_.z) * vec3(-other.g1_.y, other.g1_.x, other.g0_.w) + vec3(self.g10_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g0_.x) * vec2(0.0, -other.g1_.w) + vec2(self.g3_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g3_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g3_.z) * vec2(-other.g1_.z, 0.0) + vec2(self.g4_.x) * vec2(0.0, other.g0_.x) + vec2(self.g4_.y) * vec2(0.0, other.g0_.y) + vec2(self.g4_.z) * vec2(0.0, other.g0_.z) + vec2(self.g5_.x) * vec2(0.0, other.g1_.x) + vec2(self.g5_.y) * vec2(0.0, other.g1_.y) + vec2(self.g5_.z) * vec2(0.0, other.g1_.z) + vec2(self.g5_.w) * vec2(0.0, other.g1_.w) + vec2(self.g9_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g9_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g9_.z) * vec2(0.0, -other.g0_.z) + self.g10_ * vec2(other.g0_.w), vec3(0.0) - vec3(self.g2_.x) * vec3(other.g1_.x, other.g1_.y, other.g1_.z) + vec3(self.g6_.x) * vec3(-other.g0_.w, other.g1_.z, -other.g1_.y) + vec3(self.g6_.y) * vec3(-other.g1_.z, -other.g0_.w, other.g1_.x) + vec3(self.g6_.z) * vec3(other.g1_.y, -other.g1_.x, -other.g0_.w), vec3(self.g1_.x) * vec3(-other.g0_.w, other.g1_.z, -other.g1_.y) + vec3(self.g1_.y) * vec3(-other.g1_.z, -other.g0_.w, other.g1_.x) + vec3(self.g1_.z) * vec3(other.g1_.y, -other.g1_.x, -other.g0_.w) + vec3(self.g2_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g6_.x) * vec3(other.g1_.w, -other.g0_.z, other.g0_.y) + vec3(self.g6_.y) * vec3(other.g0_.z, other.g1_.w, -other.g0_.x) + vec3(self.g6_.z) * vec3(-other.g0_.y, other.g0_.x, other.g1_.w) - vec3(self.g6_.w) * vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec4(self.g0_.y) * other.g0_ + vec4(self.g1_.x) * vec4(other.g1_.w, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g1_.y) * vec4(other.g0_.z, other.g1_.w, -other.g0_.x, 0.0) + vec4(self.g1_.z) * vec4(-other.g0_.y, other.g0_.x, other.g1_.w, 0.0) + vec4(self.g2_.y, self.g2_.y, self.g2_.y, self.g2_.x) * other.g1_ + vec4(self.g6_.x) * vec4(0.0, 0.0, 0.0, -other.g0_.x) + vec4(self.g6_.y) * vec4(0.0, 0.0, 0.0, -other.g0_.y) + self.g6_.wwwz * other.g0_.xyzz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g7_.x) * vec4(other.g1_.w, -other.g0_.z, other.g0_.y, -other.g1_.x) + vec4(self.g7_.y) * vec4(other.g0_.z, other.g1_.w, -other.g0_.x, -other.g1_.y) + vec4(self.g7_.z) * vec4(-other.g0_.y, other.g0_.x, other.g1_.w, -other.g1_.z) + vec4(self.g8_.x) * vec4(other.g0_.w, -other.g1_.z, other.g1_.y, 0.0) + vec4(self.g8_.y) * vec4(other.g1_.z, other.g0_.w, -other.g1_.x, 0.0) + vec4(self.g8_.z) * vec4(-other.g1_.y, other.g1_.x, other.g0_.w, 0.0), vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, other.g0_.w) + vec4(self.g3_.x) * vec4(other.g0_.w, -other.g1_.z, other.g1_.y, -other.g0_.x) + vec4(self.g3_.y) * vec4(other.g1_.z, other.g0_.w, -other.g1_.x, -other.g0_.y) + vec4(self.g3_.z) * vec4(-other.g1_.y, other.g1_.x, other.g0_.w, -other.g0_.z) + vec4(self.g4_.x) * vec4(0.0, 0.0, 0.0, -other.g1_.x) + vec4(self.g4_.y) * vec4(0.0, 0.0, 0.0, -other.g1_.y) + vec4(self.g4_.z) * vec4(0.0, 0.0, 0.0, -other.g1_.z) + vec4(self.g10_.x) * other.g1_, vec3(self.g3_.x) * vec3(-other.g1_.w, other.g0_.z, -other.g0_.y) + vec3(self.g3_.y) * vec3(-other.g0_.z, -other.g1_.w, other.g0_.x) + vec3(self.g3_.z) * vec3(other.g0_.y, -other.g0_.x, -other.g1_.w) - vec3(self.g5_.w) * vec3(other.g1_.x, other.g1_.y, other.g1_.z) + vec3(self.g9_.x) * vec3(-other.g0_.w, other.g1_.z, -other.g1_.y) + vec3(self.g9_.y) * vec3(-other.g1_.z, -other.g0_.w, other.g1_.x) + vec3(self.g9_.z) * vec3(other.g1_.y, -other.g1_.x, -other.g0_.w) - vec3(self.g10_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(0.0) - vec3(self.g0_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g4_.x) * vec3(-other.g1_.w, other.g0_.z, -other.g0_.y) + vec3(self.g4_.y) * vec3(-other.g0_.z, -other.g1_.w, other.g0_.x) + vec3(self.g4_.z) * vec3(other.g0_.y, -other.g0_.x, -other.g1_.w) + vec3(self.g5_.x) * vec3(-other.g0_.w, other.g1_.z, -other.g1_.y) + vec3(self.g5_.y) * vec3(-other.g1_.z, -other.g0_.w, other.g1_.x) + vec3(self.g5_.z) * vec3(other.g1_.y, -other.g1_.x, -other.g0_.w) + vec3(self.g5_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g9_.x) * vec3(other.g1_.w, -other.g0_.z, other.g0_.y) + vec3(self.g9_.y) * vec3(other.g0_.z, other.g1_.w, -other.g0_.x) + vec3(self.g9_.z) * vec3(-other.g0_.y, other.g0_.x, other.g1_.w) - vec3(self.g10_.y) * vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec3(self.g0_.y) * vec3(other.g1_.x, other.g1_.y, other.g1_.z) + vec3(self.g2_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g6_.x) * vec3(other.g1_.w, -other.g0_.z, other.g0_.y) + vec3(self.g6_.y) * vec3(other.g0_.z, other.g1_.w, -other.g0_.x) + vec3(self.g6_.z) * vec3(-other.g0_.y, other.g0_.x, other.g1_.w) + vec3(self.g7_.x) * vec3(other.g0_.w, -other.g1_.z, other.g1_.y) + vec3(self.g7_.y) * vec3(other.g1_.z, other.g0_.w, -other.g1_.x) + vec3(self.g7_.z) * vec3(-other.g1_.y, other.g1_.x, other.g0_.w), vec2(self.g0_.y) * vec2(0.0, other.g1_.w) + vec2(self.g1_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g0_.z) + self.g2_ * vec2(-other.g0_.w) + vec2(self.g6_.x) * vec2(other.g1_.x, 0.0) + vec2(self.g6_.y) * vec2(other.g1_.y, 0.0) + vec2(self.g6_.z, self.g6_.w) * vec2(other.g1_.z, other.g1_.w) + vec2(self.g7_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g7_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g7_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g8_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g8_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g8_.z) * vec2(0.0, -other.g1_.z));
}

MultiVector multiVector_antiWedgeDot_line(MultiVector self, Line other) {
    return MultiVector(vec2(self.g3_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g3_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g3_.z) * vec2(-other.g1_.z, 0.0) + vec2(self.g4_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g4_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g4_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g6_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g6_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g6_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g7_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g7_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g7_.z) * vec2(0.0, -other.g0_.z), vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) - vec3(self.g2_.x) * other.g1_ + vec3(self.g6_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g6_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g6_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g6_.w) * other.g0_, vec2(self.g1_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g6_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g6_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g6_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g7_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g7_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g7_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g8_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g8_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g8_.z) * vec2(0.0, -other.g0_.z), vec3(self.g3_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g3_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g3_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) + vec3(self.g10_.x) * other.g0_, vec3(self.g0_.x) * other.g0_ + vec3(self.g3_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g3_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g3_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g4_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g4_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g4_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) + vec3(self.g10_.x) * other.g1_, vec4(self.g0_.x) * vec4(other.g1_.x, other.g1_.y, other.g1_.z, 0.0) + vec4(self.g3_.x) * vec4(0.0, 0.0, 0.0, -other.g1_.x) + vec4(self.g3_.y) * vec4(0.0, 0.0, 0.0, -other.g1_.y) + vec4(self.g3_.z) * vec4(0.0, 0.0, 0.0, -other.g1_.z) + vec4(self.g4_.x) * vec4(0.0, -other.g1_.z, other.g1_.y, 0.0) + vec4(self.g4_.y) * vec4(other.g1_.z, 0.0, -other.g1_.x, 0.0) + vec4(self.g4_.z) * vec4(-other.g1_.y, other.g1_.x, 0.0, 0.0) + vec4(self.g5_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g5_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g5_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0) + vec4(self.g5_.w) * vec4(-other.g1_.x, -other.g1_.y, -other.g1_.z, 0.0) + vec4(self.g9_.x) * vec4(0.0, other.g1_.z, -other.g1_.y, -other.g0_.x) + vec4(self.g9_.y) * vec4(-other.g1_.z, 0.0, other.g1_.x, -other.g0_.y) + vec4(self.g9_.z) * vec4(other.g1_.y, -other.g1_.x, 0.0, -other.g0_.z) + vec4(self.g10_.y) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, 0.0), vec4(self.g1_.x) * vec4(0.0, 0.0, 0.0, -other.g0_.x) + vec4(self.g1_.y) * vec4(0.0, 0.0, 0.0, -other.g0_.y) + vec4(self.g1_.z) * vec4(0.0, 0.0, 0.0, -other.g0_.z) + vec4(self.g2_.x) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, 0.0) + vec4(self.g6_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, other.g1_.x) + vec4(self.g6_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, other.g1_.y) + vec4(self.g6_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, other.g1_.z), vec3(self.g0_.y) * other.g0_ + vec3(self.g2_.x) * other.g1_ + vec3(self.g6_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g6_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g6_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g7_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g7_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g7_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(self.g0_.y) * other.g1_ + vec3(self.g1_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g1_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g1_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g2_.y) * other.g0_ + vec3(self.g6_.w) * other.g1_ + vec3(self.g7_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g7_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g7_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g8_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g8_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g8_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(self.g3_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g3_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g3_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g5_.w) * other.g0_ + vec3(self.g9_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g9_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g9_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) + vec3(self.g10_.x) * other.g1_, vec2(self.g3_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g3_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g3_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g4_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g4_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g4_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g5_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g5_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g5_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g9_.x) * vec2(0.0, other.g1_.x) + vec2(self.g9_.y) * vec2(0.0, other.g1_.y) + vec2(self.g9_.z) * vec2(0.0, other.g1_.z));
}

MultiVector multiVector_antiWedgeDot_motor(MultiVector self, Motor other) {
    return MultiVector(self.g0_ * vec2(other.g0_.w) + vec2(self.g3_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g3_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g3_.z) * vec2(-other.g1_.z, 0.0) + vec2(self.g4_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g4_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g4_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g6_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g6_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g6_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g7_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g7_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g7_.z) * vec2(0.0, -other.g0_.z), vec3(self.g1_.x) * vec3(other.g0_.w, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, other.g0_.w, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, other.g0_.w) - vec3(self.g2_.x) * other.g1_ + vec3(self.g6_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g6_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g6_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g6_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g1_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g1_.z) + self.g2_ * vec2(other.g0_.w) + vec2(self.g6_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g6_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g6_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g7_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g7_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g7_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g8_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g8_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g8_.z) * vec2(0.0, -other.g0_.z), vec3(self.g3_.x) * vec3(other.g0_.w, -other.g0_.z, other.g0_.y) + vec3(self.g3_.y) * vec3(other.g0_.z, other.g0_.w, -other.g0_.x) + vec3(self.g3_.z) * vec3(-other.g0_.y, other.g0_.x, other.g0_.w) + vec3(self.g10_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(self.g0_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g3_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g3_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g3_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g4_.x) * vec3(other.g0_.w, -other.g0_.z, other.g0_.y) + vec3(self.g4_.y) * vec3(other.g0_.z, other.g0_.w, -other.g0_.x) + vec3(self.g4_.z) * vec3(-other.g0_.y, other.g0_.x, other.g0_.w) + vec3(self.g10_.x) * other.g1_, vec4(self.g0_.x) * vec4(other.g1_.x, other.g1_.y, other.g1_.z, 0.0) + vec4(self.g3_.x) * vec4(0.0, 0.0, 0.0, -other.g1_.x) + vec4(self.g3_.y) * vec4(0.0, 0.0, 0.0, -other.g1_.y) + vec4(self.g3_.z) * vec4(0.0, 0.0, 0.0, -other.g1_.z) + vec4(self.g4_.x) * vec4(0.0, -other.g1_.z, other.g1_.y, 0.0) + vec4(self.g4_.y) * vec4(other.g1_.z, 0.0, -other.g1_.x, 0.0) + vec4(self.g4_.z) * vec4(-other.g1_.y, other.g1_.x, 0.0, 0.0) + vec4(self.g5_.x) * vec4(other.g0_.w, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g5_.y) * vec4(other.g0_.z, other.g0_.w, -other.g0_.x, 0.0) + vec4(self.g5_.z) * vec4(-other.g0_.y, other.g0_.x, other.g0_.w, 0.0) + vec4(self.g5_.w) * vec4(-other.g1_.x, -other.g1_.y, -other.g1_.z, other.g0_.w) + vec4(self.g9_.x) * vec4(0.0, other.g1_.z, -other.g1_.y, -other.g0_.x) + vec4(self.g9_.y) * vec4(-other.g1_.z, 0.0, other.g1_.x, -other.g0_.y) + vec4(self.g9_.z) * vec4(other.g1_.y, -other.g1_.x, 0.0, -other.g0_.z) + vec4(self.g10_.y) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, 0.0), vec4(self.g1_.x) * vec4(0.0, 0.0, 0.0, -other.g0_.x) + vec4(self.g1_.y) * vec4(0.0, 0.0, 0.0, -other.g0_.y) + vec4(self.g1_.z) * vec4(0.0, 0.0, 0.0, -other.g0_.z) + vec4(self.g2_.x) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, 0.0) + vec4(self.g6_.x) * vec4(other.g0_.w, -other.g0_.z, other.g0_.y, other.g1_.x) + vec4(self.g6_.y) * vec4(other.g0_.z, other.g0_.w, -other.g0_.x, other.g1_.y) + vec4(self.g6_.z) * vec4(-other.g0_.y, other.g0_.x, other.g0_.w, other.g1_.z) + vec4(self.g6_.w) * vec4(0.0, 0.0, 0.0, other.g0_.w), vec3(self.g0_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g2_.x) * other.g1_ + vec3(self.g6_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g6_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g6_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g7_.x) * vec3(other.g0_.w, -other.g0_.z, other.g0_.y) + vec3(self.g7_.y) * vec3(other.g0_.z, other.g0_.w, -other.g0_.x) + vec3(self.g7_.z) * vec3(-other.g0_.y, other.g0_.x, other.g0_.w), vec3(self.g0_.y) * other.g1_ + vec3(self.g1_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g1_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g1_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g2_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g6_.w) * other.g1_ + vec3(self.g7_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g7_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g7_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g8_.x) * vec3(other.g0_.w, -other.g0_.z, other.g0_.y) + vec3(self.g8_.y) * vec3(other.g0_.z, other.g0_.w, -other.g0_.x) + vec3(self.g8_.z) * vec3(-other.g0_.y, other.g0_.x, other.g0_.w), vec3(self.g3_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g3_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g3_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g5_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g9_.x) * vec3(other.g0_.w, -other.g0_.z, other.g0_.y) + vec3(self.g9_.y) * vec3(other.g0_.z, other.g0_.w, -other.g0_.x) + vec3(self.g9_.z) * vec3(-other.g0_.y, other.g0_.x, other.g0_.w) + vec3(self.g10_.x) * other.g1_, vec2(self.g3_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g3_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g3_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g4_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g4_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g4_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g5_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g5_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g5_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g9_.x) * vec2(0.0, other.g1_.x) + vec2(self.g9_.y) * vec2(0.0, other.g1_.y) + vec2(self.g9_.z) * vec2(0.0, other.g1_.z) + self.g10_ * vec2(other.g0_.w));
}

MultiVector multiVector_antiWedgeDot_multiVector(MultiVector self, MultiVector other) {
    return MultiVector(vec2(self.g0_.x) * other.g0_.yx * vec2(1.0, -1.0) + vec2(self.g0_.y) * other.g0_ + vec2(self.g1_.x) * vec2(other.g9_.x, -other.g1_.x) + vec2(self.g1_.y) * vec2(other.g9_.y, -other.g1_.y) + vec2(self.g1_.z) * vec2(other.g9_.z, -other.g1_.z) + vec2(self.g2_.x) * vec2(other.g10_.y, other.g2_.y) + vec2(self.g2_.y) * vec2(other.g10_.x, other.g2_.x) + vec2(self.g3_.x) * vec2(-other.g8_.x, other.g5_.x) + vec2(self.g3_.y) * vec2(-other.g8_.y, other.g5_.y) + vec2(self.g3_.z) * vec2(-other.g8_.z, other.g5_.z) + vec2(self.g4_.x) * vec2(-other.g7_.x, other.g4_.x) + vec2(self.g4_.y) * vec2(-other.g7_.y, other.g4_.y) + vec2(self.g4_.z) * vec2(-other.g7_.z, other.g4_.z) + vec2(self.g5_.x) * vec2(-other.g6_.x, other.g3_.x) + vec2(self.g5_.y) * vec2(-other.g6_.y, other.g3_.y) + vec2(self.g5_.z) * vec2(-other.g6_.z, other.g3_.z) - vec2(self.g5_.w) * vec2(other.g6_.w, other.g5_.w) - vec2(self.g6_.x) * vec2(other.g5_.x, other.g8_.x) - vec2(self.g6_.y) * vec2(other.g5_.y, other.g8_.y) - vec2(self.g6_.z) * vec2(other.g5_.z, other.g8_.z) + vec2(self.g6_.w) * vec2(-other.g5_.w, other.g6_.w) - vec2(self.g7_.x) * vec2(other.g4_.x, other.g7_.x) - vec2(self.g7_.y) * vec2(other.g4_.y, other.g7_.y) - vec2(self.g7_.z) * vec2(other.g4_.z, other.g7_.z) - vec2(self.g8_.x) * vec2(other.g3_.x, other.g6_.x) - vec2(self.g8_.y) * vec2(other.g3_.y, other.g6_.y) - vec2(self.g8_.z) * vec2(other.g3_.z, other.g6_.z) + vec2(self.g9_.x) * vec2(other.g1_.x, other.g9_.x) + vec2(self.g9_.y) * vec2(other.g1_.y, other.g9_.y) + vec2(self.g9_.z) * vec2(other.g1_.z, other.g9_.z) + vec2(self.g10_.x) * vec2(other.g2_.y, -other.g10_.y) + vec2(self.g10_.y) * vec2(other.g2_.x, -other.g10_.x), vec3(self.g0_.x) * other.g9_ + vec3(self.g0_.y) * other.g1_ + vec3(self.g1_.x) * vec3(other.g0_.y, -other.g7_.z, other.g7_.y) + vec3(self.g1_.y) * vec3(other.g7_.z, other.g0_.y, -other.g7_.x) + vec3(self.g1_.z) * vec3(-other.g7_.y, other.g7_.x, other.g0_.y) - vec3(self.g2_.x) * other.g8_ + vec3(self.g2_.y) * vec3(other.g6_.x, other.g6_.y, other.g6_.z) + vec3(self.g3_.x) * vec3(other.g10_.y, -other.g5_.z, other.g5_.y) + vec3(self.g3_.y) * vec3(other.g5_.z, other.g10_.y, -other.g5_.x) + vec3(self.g3_.z) * vec3(-other.g5_.y, other.g5_.x, other.g10_.y) + vec3(self.g4_.x) * vec3(other.g5_.w, -other.g9_.z, other.g9_.y) + vec3(self.g4_.y) * vec3(other.g9_.z, other.g5_.w, -other.g9_.x) + vec3(self.g4_.z) * vec3(-other.g9_.y, other.g9_.x, other.g5_.w) + vec3(self.g5_.x) * vec3(-other.g10_.x, other.g3_.z, -other.g3_.y) + vec3(self.g5_.y) * vec3(-other.g3_.z, -other.g10_.x, other.g3_.x) + vec3(self.g5_.z) * vec3(other.g3_.y, -other.g3_.x, -other.g10_.x) + vec3(self.g5_.w) * other.g4_ + vec3(self.g6_.x) * vec3(-other.g2_.y, other.g8_.z, -other.g8_.y) + vec3(self.g6_.y) * vec3(-other.g8_.z, -other.g2_.y, other.g8_.x) + vec3(self.g6_.z) * vec3(other.g8_.y, -other.g8_.x, -other.g2_.y) + vec3(self.g6_.w) * other.g7_ + vec3(self.g7_.x) * vec3(other.g6_.w, -other.g1_.z, other.g1_.y) + vec3(self.g7_.y) * vec3(other.g1_.z, other.g6_.w, -other.g1_.x) + vec3(self.g7_.z) * vec3(-other.g1_.y, other.g1_.x, other.g6_.w) + vec3(self.g8_.x) * vec3(other.g2_.x, -other.g6_.z, other.g6_.y) + vec3(self.g8_.y) * vec3(other.g6_.z, other.g2_.x, -other.g6_.x) + vec3(self.g8_.z) * vec3(-other.g6_.y, other.g6_.x, other.g2_.x) + vec3(self.g9_.x) * vec3(other.g0_.x, -other.g4_.z, other.g4_.y) + vec3(self.g9_.y) * vec3(other.g4_.z, other.g0_.x, -other.g4_.x) + vec3(self.g9_.z) * vec3(-other.g4_.y, other.g4_.x, other.g0_.x) + vec3(self.g10_.x) * vec3(other.g5_.x, other.g5_.y, other.g5_.z) - vec3(self.g10_.y) * other.g3_, vec2(0.0) - vec2(self.g0_.x) * other.g10_ + vec2(self.g0_.y) * other.g2_ + vec2(self.g1_.x) * vec2(other.g6_.x, -other.g8_.x) + vec2(self.g1_.y) * vec2(other.g6_.y, -other.g8_.y) + vec2(self.g1_.z) * vec2(other.g6_.z, -other.g8_.z) + self.g2_ * vec2(other.g6_.w) + self.g2_ * vec2(other.g0_.y) + vec2(self.g3_.x) * vec2(-other.g9_.x, 0.0) + vec2(self.g3_.x) * vec2(other.g4_.x, 0.0) + vec2(self.g3_.y) * vec2(-other.g9_.y, 0.0) + vec2(self.g3_.y) * vec2(other.g4_.y, 0.0) + vec2(self.g3_.z) * vec2(-other.g9_.z, 0.0) + vec2(self.g3_.z) * vec2(other.g4_.z, 0.0) + vec2(self.g4_.x) * vec2(other.g3_.x, other.g5_.x) + vec2(self.g4_.y) * vec2(other.g3_.y, other.g5_.y) + vec2(self.g4_.z) * vec2(other.g3_.z, other.g5_.z) + vec2(self.g5_.x) * vec2(0.0, other.g9_.x) + vec2(self.g5_.x) * vec2(0.0, other.g4_.x) + vec2(self.g5_.y) * vec2(0.0, other.g9_.y) + vec2(self.g5_.y) * vec2(0.0, other.g4_.y) + vec2(self.g5_.z) * vec2(0.0, other.g9_.z) + vec2(self.g5_.z) * vec2(0.0, other.g4_.z) + vec2(self.g5_.w) * other.g10_ * vec2(-1.0, 1.0) + vec2(self.g6_.x) * vec2(-other.g7_.x, 0.0) + vec2(self.g6_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g6_.y) * vec2(-other.g7_.y, 0.0) + vec2(self.g6_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g6_.z) * vec2(-other.g7_.z, 0.0) + vec2(self.g6_.z) * vec2(-other.g1_.z, 0.0) + vec2(self.g6_.w) * other.g2_ * vec2(-1.0, 1.0) - vec2(self.g7_.x) * vec2(other.g6_.x, other.g8_.x) - vec2(self.g7_.y) * vec2(other.g6_.y, other.g8_.y) - vec2(self.g7_.z) * vec2(other.g6_.z, other.g8_.z) + vec2(self.g8_.x) * vec2(0.0, -other.g7_.x) + vec2(self.g8_.x) * vec2(0.0, other.g1_.x) + vec2(self.g8_.y) * vec2(0.0, -other.g7_.y) + vec2(self.g8_.y) * vec2(0.0, other.g1_.y) + vec2(self.g8_.z) * vec2(0.0, -other.g7_.z) + vec2(self.g8_.z) * vec2(0.0, other.g1_.z) + vec2(self.g9_.x) * vec2(other.g3_.x, -other.g5_.x) + vec2(self.g9_.y) * vec2(other.g3_.y, -other.g5_.y) + vec2(self.g9_.z) * vec2(other.g3_.z, -other.g5_.z) + self.g10_ * vec2(other.g5_.w) - self.g10_ * vec2(other.g0_.x), vec3(self.g0_.x) * vec3(other.g6_.x, other.g6_.y, other.g6_.z) + vec3(self.g0_.y) * other.g3_ + vec3(self.g1_.x) * vec3(-other.g10_.x, other.g3_.z, -other.g3_.y) + vec3(self.g1_.y) * vec3(-other.g3_.z, -other.g10_.x, other.g3_.x) + vec3(self.g1_.z) * vec3(other.g3_.y, -other.g3_.x, -other.g10_.x) - vec3(self.g2_.x) * other.g9_ + vec3(self.g2_.x) * other.g4_ + vec3(self.g3_.x) * vec3(other.g6_.w, -other.g7_.z, other.g7_.y) + vec3(self.g3_.x) * vec3(other.g0_.y, -other.g1_.z, other.g1_.y) + vec3(self.g3_.y) * vec3(other.g7_.z, other.g6_.w, -other.g7_.x) + vec3(self.g3_.y) * vec3(other.g1_.z, other.g0_.y, -other.g1_.x) + vec3(self.g3_.z) * vec3(-other.g7_.y, other.g7_.x, other.g6_.w) + vec3(self.g3_.z) * vec3(-other.g1_.y, other.g1_.x, other.g0_.y) + vec3(self.g4_.x) * vec3(other.g2_.x, -other.g6_.z, other.g6_.y) + vec3(self.g4_.y) * vec3(other.g6_.z, other.g2_.x, -other.g6_.x) + vec3(self.g4_.z) * vec3(-other.g6_.y, other.g6_.x, other.g2_.x) + vec3(self.g5_.w) * vec3(other.g6_.x, other.g6_.y, other.g6_.z) + vec3(self.g6_.x) * vec3(-other.g5_.w, other.g9_.z, -other.g9_.y) + vec3(self.g6_.x) * vec3(other.g0_.x, -other.g4_.z, other.g4_.y) + vec3(self.g6_.y) * vec3(-other.g9_.z, -other.g5_.w, other.g9_.x) + vec3(self.g6_.y) * vec3(other.g4_.z, other.g0_.x, -other.g4_.x) + vec3(self.g6_.z) * vec3(other.g9_.y, -other.g9_.x, -other.g5_.w) + vec3(self.g6_.z) * vec3(-other.g4_.y, other.g4_.x, other.g0_.x) - vec3(self.g6_.w) * other.g3_ + vec3(self.g7_.x) * vec3(other.g10_.x, -other.g3_.z, other.g3_.y) + vec3(self.g7_.y) * vec3(other.g3_.z, other.g10_.x, -other.g3_.x) + vec3(self.g7_.z) * vec3(-other.g3_.y, other.g3_.x, other.g10_.x) + vec3(self.g9_.x) * vec3(other.g2_.x, -other.g6_.z, other.g6_.y) + vec3(self.g9_.y) * vec3(other.g6_.z, other.g2_.x, -other.g6_.x) + vec3(self.g9_.z) * vec3(-other.g6_.y, other.g6_.x, other.g2_.x) + vec3(self.g10_.x) * other.g7_ + vec3(self.g10_.x) * other.g1_, vec3(self.g0_.x) * other.g7_ + vec3(self.g0_.y) * other.g4_ + vec3(self.g1_.x) * vec3(-other.g5_.w, other.g9_.z, -other.g9_.y) + vec3(self.g1_.y) * vec3(-other.g9_.z, -other.g5_.w, other.g9_.x) + vec3(self.g1_.z) * vec3(other.g9_.y, -other.g9_.x, -other.g5_.w) + vec3(self.g2_.x) * vec3(other.g5_.x, other.g5_.y, other.g5_.z) + vec3(self.g2_.y) * other.g3_ + vec3(self.g3_.x) * vec3(other.g2_.y, -other.g8_.z, other.g8_.y) + vec3(self.g3_.y) * vec3(other.g8_.z, other.g2_.y, -other.g8_.x) + vec3(self.g3_.z) * vec3(-other.g8_.y, other.g8_.x, other.g2_.y) + vec3(self.g4_.x) * vec3(other.g0_.y, -other.g7_.z, other.g7_.y) + vec3(self.g4_.y) * vec3(other.g7_.z, other.g0_.y, -other.g7_.x) + vec3(self.g4_.z) * vec3(-other.g7_.y, other.g7_.x, other.g0_.y) + vec3(self.g5_.x) * vec3(other.g2_.x, -other.g6_.z, other.g6_.y) + vec3(self.g5_.y) * vec3(other.g6_.z, other.g2_.x, -other.g6_.x) + vec3(self.g5_.z) * vec3(-other.g6_.y, other.g6_.x, other.g2_.x) - vec3(self.g5_.w) * other.g1_ + vec3(self.g6_.x) * vec3(other.g10_.y, -other.g5_.z, other.g5_.y) + vec3(self.g6_.y) * vec3(other.g5_.z, other.g10_.y, -other.g5_.x) + vec3(self.g6_.z) * vec3(-other.g5_.y, other.g5_.x, other.g10_.y) - vec3(self.g6_.w) * other.g9_ + vec3(self.g7_.x) * vec3(other.g0_.x, -other.g4_.z, other.g4_.y) + vec3(self.g7_.y) * vec3(other.g4_.z, other.g0_.x, -other.g4_.x) + vec3(self.g7_.z) * vec3(-other.g4_.y, other.g4_.x, other.g0_.x) + vec3(self.g8_.x) * vec3(other.g10_.x, -other.g3_.z, other.g3_.y) + vec3(self.g8_.y) * vec3(other.g3_.z, other.g10_.x, -other.g3_.x) + vec3(self.g8_.z) * vec3(-other.g3_.y, other.g3_.x, other.g10_.x) + vec3(self.g9_.x) * vec3(-other.g6_.w, other.g1_.z, -other.g1_.y) + vec3(self.g9_.y) * vec3(-other.g1_.z, -other.g6_.w, other.g1_.x) + vec3(self.g9_.z) * vec3(other.g1_.y, -other.g1_.x, -other.g6_.w) + vec3(self.g10_.x) * other.g8_ + vec3(self.g10_.y) * vec3(other.g6_.x, other.g6_.y, other.g6_.z), vec4(self.g0_.x) * vec4(other.g8_.x, other.g8_.y, other.g8_.z, -other.g6_.w) + vec4(self.g0_.y) * other.g5_ + vec4(self.g1_.x) * vec4(other.g10_.y, -other.g5_.z, other.g5_.y, other.g4_.x) + vec4(self.g1_.y) * vec4(other.g5_.z, other.g10_.y, -other.g5_.x, other.g4_.y) + vec4(self.g1_.z) * vec4(-other.g5_.y, other.g5_.x, other.g10_.y, other.g4_.z) + vec4(self.g2_.x) * vec4(0.0, 0.0, 0.0, other.g10_.y) + vec4(self.g2_.y) * vec4(other.g9_.x, other.g9_.y, other.g9_.z, -other.g10_.x) + vec4(self.g2_.y) * vec4(other.g4_.x, other.g4_.y, other.g4_.z, 0.0) + vec4(self.g3_.x) * vec4(0.0, 0.0, 0.0, -other.g8_.x) + vec4(self.g3_.y) * vec4(0.0, 0.0, 0.0, -other.g8_.y) + vec4(self.g3_.z) * vec4(0.0, 0.0, 0.0, -other.g8_.z) + vec4(self.g4_.x) * vec4(other.g2_.y, -other.g8_.z, other.g8_.y, other.g1_.x) + vec4(self.g4_.y) * vec4(other.g8_.z, other.g2_.y, -other.g8_.x, other.g1_.y) + vec4(self.g4_.z) * vec4(-other.g8_.y, other.g8_.x, other.g2_.y, other.g1_.z) + vec4(self.g5_.x) * vec4(-other.g6_.w, -other.g7_.z, other.g7_.y, other.g6_.x) + vec4(self.g5_.x) * vec4(other.g0_.y, other.g1_.z, -other.g1_.y, 0.0) + vec4(self.g5_.y) * vec4(other.g7_.z, -other.g6_.w, -other.g7_.x, other.g6_.y) + vec4(self.g5_.y) * vec4(-other.g1_.z, other.g0_.y, other.g1_.x, 0.0) + vec4(self.g5_.z) * vec4(-other.g7_.y, other.g7_.x, -other.g6_.w, other.g6_.z) + vec4(self.g5_.z) * vec4(other.g1_.y, -other.g1_.x, other.g0_.y, 0.0) + vec4(self.g5_.w) * vec4(-other.g8_.x, -other.g8_.y, -other.g8_.z, other.g0_.y) + vec4(self.g6_.x) * vec4(0.0, 0.0, 0.0, -other.g5_.x) + vec4(self.g6_.y) * vec4(0.0, 0.0, 0.0, -other.g5_.y) + vec4(self.g6_.z) * vec4(0.0, 0.0, 0.0, -other.g5_.z) + vec4(self.g6_.w) * vec4(other.g5_.x, other.g5_.y, other.g5_.z, -other.g0_.x) + vec4(self.g7_.x) * vec4(other.g10_.y, -other.g5_.z, other.g5_.y, -other.g9_.x) + vec4(self.g7_.y) * vec4(other.g5_.z, other.g10_.y, -other.g5_.x, -other.g9_.y) + vec4(self.g7_.z) * vec4(-other.g5_.y, other.g5_.x, other.g10_.y, -other.g9_.z) + vec4(self.g8_.x) * vec4(other.g5_.w, -other.g9_.z, other.g9_.y, other.g3_.x) + vec4(self.g8_.x) * vec4(other.g0_.x, -other.g4_.z, other.g4_.y, 0.0) + vec4(self.g8_.y) * vec4(other.g9_.z, other.g5_.w, -other.g9_.x, other.g3_.y) + vec4(self.g8_.y) * vec4(other.g4_.z, other.g0_.x, -other.g4_.x, 0.0) + vec4(self.g8_.z) * vec4(-other.g9_.y, other.g9_.x, other.g5_.w, other.g3_.z) + vec4(self.g8_.z) * vec4(-other.g4_.y, other.g4_.x, other.g0_.x, 0.0) + vec4(self.g9_.x) * vec4(-other.g2_.y, other.g8_.z, -other.g8_.y, -other.g7_.x) + vec4(self.g9_.y) * vec4(-other.g8_.z, -other.g2_.y, other.g8_.x, -other.g7_.y) + vec4(self.g9_.z) * vec4(other.g8_.y, -other.g8_.x, -other.g2_.y, -other.g7_.z) + vec4(self.g10_.x) * vec4(0.0, 0.0, 0.0, other.g2_.y) + vec4(self.g10_.y) * vec4(other.g7_.x, other.g7_.y, other.g7_.z, -other.g2_.x) + vec4(self.g10_.y) * vec4(-other.g1_.x, -other.g1_.y, -other.g1_.z, 0.0), vec4(self.g0_.x) * vec4(-other.g3_.x, -other.g3_.y, -other.g3_.z, other.g5_.w) + vec4(self.g0_.y) * other.g6_ + vec4(self.g1_.x) * vec4(-other.g2_.x, other.g6_.z, -other.g6_.y, -other.g7_.x) + vec4(self.g1_.y) * vec4(-other.g6_.z, -other.g2_.x, other.g6_.x, -other.g7_.y) + vec4(self.g1_.z) * vec4(other.g6_.y, -other.g6_.x, -other.g2_.x, -other.g7_.z) + vec4(self.g2_.x) * vec4(other.g7_.x, other.g7_.y, other.g7_.z, -other.g2_.y) + vec4(self.g2_.x) * vec4(other.g1_.x, other.g1_.y, other.g1_.z, 0.0) + vec4(self.g2_.y) * vec4(0.0, 0.0, 0.0, other.g2_.x) + vec4(self.g3_.x) * vec4(other.g5_.w, -other.g9_.z, other.g9_.y, -other.g5_.x) + vec4(self.g3_.x) * vec4(-other.g0_.x, other.g4_.z, -other.g4_.y, 0.0) + vec4(self.g3_.y) * vec4(other.g9_.z, other.g5_.w, -other.g9_.x, -other.g5_.y) + vec4(self.g3_.y) * vec4(-other.g4_.z, -other.g0_.x, other.g4_.x, 0.0) + vec4(self.g3_.z) * vec4(-other.g9_.y, other.g9_.x, other.g5_.w, -other.g5_.z) + vec4(self.g3_.z) * vec4(other.g4_.y, -other.g4_.x, -other.g0_.x, 0.0) + vec4(self.g4_.x) * vec4(-other.g10_.x, other.g3_.z, -other.g3_.y, -other.g9_.x) + vec4(self.g4_.y) * vec4(-other.g3_.z, -other.g10_.x, other.g3_.x, -other.g9_.y) + vec4(self.g4_.z) * vec4(other.g3_.y, -other.g3_.x, -other.g10_.x, -other.g9_.z) + vec4(self.g5_.x) * vec4(0.0, 0.0, 0.0, other.g3_.x) + vec4(self.g5_.y) * vec4(0.0, 0.0, 0.0, other.g3_.y) + vec4(self.g5_.z) * vec4(0.0, 0.0, 0.0, other.g3_.z) + vec4(self.g5_.w) * vec4(-other.g3_.x, -other.g3_.y, -other.g3_.z, other.g0_.x) + vec4(self.g6_.x) * vec4(other.g6_.w, -other.g7_.z, other.g7_.y, other.g8_.x) + vec4(self.g6_.x) * vec4(other.g0_.y, -other.g1_.z, other.g1_.y, 0.0) + vec4(self.g6_.y) * vec4(other.g7_.z, other.g6_.w, -other.g7_.x, other.g8_.y) + vec4(self.g6_.y) * vec4(other.g1_.z, other.g0_.y, -other.g1_.x, 0.0) + vec4(self.g6_.z) * vec4(-other.g7_.y, other.g7_.x, other.g6_.w, other.g8_.z) + vec4(self.g6_.z) * vec4(-other.g1_.y, other.g1_.x, other.g0_.y, 0.0) + vec4(self.g6_.w) * vec4(-other.g6_.x, -other.g6_.y, -other.g6_.z, other.g0_.y) + vec4(self.g7_.x) * vec4(other.g2_.x, -other.g6_.z, other.g6_.y, -other.g1_.x) + vec4(self.g7_.y) * vec4(other.g6_.z, other.g2_.x, -other.g6_.x, -other.g1_.y) + vec4(self.g7_.z) * vec4(-other.g6_.y, other.g6_.x, other.g2_.x, -other.g1_.z) + vec4(self.g8_.x) * vec4(0.0, 0.0, 0.0, -other.g6_.x) + vec4(self.g8_.y) * vec4(0.0, 0.0, 0.0, -other.g6_.y) + vec4(self.g8_.z) * vec4(0.0, 0.0, 0.0, -other.g6_.z) + vec4(self.g9_.x) * vec4(-other.g10_.x, other.g3_.z, -other.g3_.y, -other.g4_.x) + vec4(self.g9_.y) * vec4(-other.g3_.z, -other.g10_.x, other.g3_.x, -other.g4_.y) + vec4(self.g9_.z) * vec4(other.g3_.y, -other.g3_.x, -other.g10_.x, -other.g4_.z) + vec4(self.g10_.x) * vec4(other.g9_.x, other.g9_.y, other.g9_.z, other.g10_.y) + vec4(self.g10_.x) * vec4(-other.g4_.x, -other.g4_.y, -other.g4_.z, 0.0) + vec4(self.g10_.y) * vec4(0.0, 0.0, 0.0, -other.g10_.x), vec3(0.0) - vec3(self.g0_.x) * other.g4_ + vec3(self.g0_.y) * other.g7_ + vec3(self.g1_.x) * vec3(other.g6_.w, -other.g1_.z, other.g1_.y) + vec3(self.g1_.y) * vec3(other.g1_.z, other.g6_.w, -other.g1_.x) + vec3(self.g1_.z) * vec3(-other.g1_.y, other.g1_.x, other.g6_.w) + vec3(self.g2_.x) * other.g8_ + vec3(self.g2_.y) * vec3(other.g6_.x, other.g6_.y, other.g6_.z) + vec3(self.g3_.x) * vec3(-other.g10_.y, other.g5_.z, -other.g5_.y) + vec3(self.g3_.y) * vec3(-other.g5_.z, -other.g10_.y, other.g5_.x) + vec3(self.g3_.z) * vec3(other.g5_.y, -other.g5_.x, -other.g10_.y) + vec3(self.g4_.x) * vec3(-other.g0_.x, other.g4_.z, -other.g4_.y) + vec3(self.g4_.y) * vec3(-other.g4_.z, -other.g0_.x, other.g4_.x) + vec3(self.g4_.z) * vec3(other.g4_.y, -other.g4_.x, -other.g0_.x) + vec3(self.g5_.x) * vec3(-other.g10_.x, other.g3_.z, -other.g3_.y) + vec3(self.g5_.y) * vec3(-other.g3_.z, -other.g10_.x, other.g3_.x) + vec3(self.g5_.z) * vec3(other.g3_.y, -other.g3_.x, -other.g10_.x) - vec3(self.g5_.w) * other.g9_ + vec3(self.g6_.x) * vec3(other.g2_.y, -other.g8_.z, other.g8_.y) + vec3(self.g6_.y) * vec3(other.g8_.z, other.g2_.y, -other.g8_.x) + vec3(self.g6_.z) * vec3(-other.g8_.y, other.g8_.x, other.g2_.y) + vec3(self.g6_.w) * other.g1_ + vec3(self.g7_.x) * vec3(other.g0_.y, -other.g7_.z, other.g7_.y) + vec3(self.g7_.y) * vec3(other.g7_.z, other.g0_.y, -other.g7_.x) + vec3(self.g7_.z) * vec3(-other.g7_.y, other.g7_.x, other.g0_.y) + vec3(self.g8_.x) * vec3(other.g2_.x, -other.g6_.z, other.g6_.y) + vec3(self.g8_.y) * vec3(other.g6_.z, other.g2_.x, -other.g6_.x) + vec3(self.g8_.z) * vec3(-other.g6_.y, other.g6_.x, other.g2_.x) + vec3(self.g9_.x) * vec3(-other.g5_.w, other.g9_.z, -other.g9_.y) + vec3(self.g9_.y) * vec3(-other.g9_.z, -other.g5_.w, other.g9_.x) + vec3(self.g9_.z) * vec3(other.g9_.y, -other.g9_.x, -other.g5_.w) - vec3(self.g10_.x) * vec3(other.g5_.x, other.g5_.y, other.g5_.z) - vec3(self.g10_.y) * other.g3_, vec3(0.0) - vec3(self.g0_.x) * vec3(other.g5_.x, other.g5_.y, other.g5_.z) + vec3(self.g0_.y) * other.g8_ + vec3(self.g1_.x) * vec3(other.g2_.y, -other.g8_.z, other.g8_.y) + vec3(self.g1_.y) * vec3(other.g8_.z, other.g2_.y, -other.g8_.x) + vec3(self.g1_.z) * vec3(-other.g8_.y, other.g8_.x, other.g2_.y) + vec3(self.g2_.y) * other.g7_ - vec3(self.g2_.y) * other.g1_ + vec3(self.g4_.x) * vec3(-other.g10_.y, other.g5_.z, -other.g5_.y) + vec3(self.g4_.y) * vec3(-other.g5_.z, -other.g10_.y, other.g5_.x) + vec3(self.g4_.z) * vec3(other.g5_.y, -other.g5_.x, -other.g10_.y) + vec3(self.g5_.x) * vec3(-other.g5_.w, other.g9_.z, -other.g9_.y) + vec3(self.g5_.x) * vec3(-other.g0_.x, other.g4_.z, -other.g4_.y) + vec3(self.g5_.y) * vec3(-other.g9_.z, -other.g5_.w, other.g9_.x) + vec3(self.g5_.y) * vec3(-other.g4_.z, -other.g0_.x, other.g4_.x) + vec3(self.g5_.z) * vec3(other.g9_.y, -other.g9_.x, -other.g5_.w) + vec3(self.g5_.z) * vec3(other.g4_.y, -other.g4_.x, -other.g0_.x) + vec3(self.g5_.w) * vec3(other.g5_.x, other.g5_.y, other.g5_.z) + vec3(self.g6_.w) * other.g8_ + vec3(self.g7_.x) * vec3(other.g2_.y, -other.g8_.z, other.g8_.y) + vec3(self.g7_.y) * vec3(other.g8_.z, other.g2_.y, -other.g8_.x) + vec3(self.g7_.z) * vec3(-other.g8_.y, other.g8_.x, other.g2_.y) + vec3(self.g8_.x) * vec3(-other.g6_.w, -other.g7_.z, other.g7_.y) + vec3(self.g8_.x) * vec3(other.g0_.y, other.g1_.z, -other.g1_.y) + vec3(self.g8_.y) * vec3(other.g7_.z, -other.g6_.w, -other.g7_.x) + vec3(self.g8_.y) * vec3(-other.g1_.z, other.g0_.y, other.g1_.x) + vec3(self.g8_.z) * vec3(-other.g7_.y, other.g7_.x, -other.g6_.w) + vec3(self.g8_.z) * vec3(other.g1_.y, -other.g1_.x, other.g0_.y) + vec3(self.g9_.x) * vec3(other.g10_.y, -other.g5_.z, other.g5_.y) + vec3(self.g9_.y) * vec3(other.g5_.z, other.g10_.y, -other.g5_.x) + vec3(self.g9_.z) * vec3(-other.g5_.y, other.g5_.x, other.g10_.y) - vec3(self.g10_.y) * other.g9_ - vec3(self.g10_.y) * other.g4_, vec3(0.0) - vec3(self.g0_.x) * other.g1_ + vec3(self.g0_.y) * other.g9_ + vec3(self.g1_.x) * vec3(-other.g0_.x, other.g4_.z, -other.g4_.y) + vec3(self.g1_.y) * vec3(-other.g4_.z, -other.g0_.x, other.g4_.x) + vec3(self.g1_.z) * vec3(other.g4_.y, -other.g4_.x, -other.g0_.x) + vec3(self.g2_.x) * vec3(other.g5_.x, other.g5_.y, other.g5_.z) - vec3(self.g2_.y) * other.g3_ + vec3(self.g3_.x) * vec3(other.g2_.y, -other.g8_.z, other.g8_.y) + vec3(self.g3_.y) * vec3(other.g8_.z, other.g2_.y, -other.g8_.x) + vec3(self.g3_.z) * vec3(-other.g8_.y, other.g8_.x, other.g2_.y) + vec3(self.g4_.x) * vec3(-other.g6_.w, other.g1_.z, -other.g1_.y) + vec3(self.g4_.y) * vec3(-other.g1_.z, -other.g6_.w, other.g1_.x) + vec3(self.g4_.z) * vec3(other.g1_.y, -other.g1_.x, -other.g6_.w) + vec3(self.g5_.x) * vec3(-other.g2_.x, other.g6_.z, -other.g6_.y) + vec3(self.g5_.y) * vec3(-other.g6_.z, -other.g2_.x, other.g6_.x) + vec3(self.g5_.z) * vec3(other.g6_.y, -other.g6_.x, -other.g2_.x) + vec3(self.g5_.w) * other.g7_ + vec3(self.g6_.x) * vec3(other.g10_.y, -other.g5_.z, other.g5_.y) + vec3(self.g6_.y) * vec3(other.g5_.z, other.g10_.y, -other.g5_.x) + vec3(self.g6_.z) * vec3(-other.g5_.y, other.g5_.x, other.g10_.y) - vec3(self.g6_.w) * other.g4_ + vec3(self.g7_.x) * vec3(other.g5_.w, -other.g9_.z, other.g9_.y) + vec3(self.g7_.y) * vec3(other.g9_.z, other.g5_.w, -other.g9_.x) + vec3(self.g7_.z) * vec3(-other.g9_.y, other.g9_.x, other.g5_.w) + vec3(self.g8_.x) * vec3(-other.g10_.x, other.g3_.z, -other.g3_.y) + vec3(self.g8_.y) * vec3(-other.g3_.z, -other.g10_.x, other.g3_.x) + vec3(self.g8_.z) * vec3(other.g3_.y, -other.g3_.x, -other.g10_.x) + vec3(self.g9_.x) * vec3(other.g0_.y, -other.g7_.z, other.g7_.y) + vec3(self.g9_.y) * vec3(other.g7_.z, other.g0_.y, -other.g7_.x) + vec3(self.g9_.z) * vec3(-other.g7_.y, other.g7_.x, other.g0_.y) + vec3(self.g10_.x) * other.g8_ - vec3(self.g10_.y) * vec3(other.g6_.x, other.g6_.y, other.g6_.z), vec2(self.g0_.x) * other.g2_ + vec2(self.g0_.y) * other.g10_ + vec2(self.g1_.x) * vec2(other.g3_.x, -other.g5_.x) + vec2(self.g1_.y) * vec2(other.g3_.y, -other.g5_.y) + vec2(self.g1_.z) * vec2(other.g3_.z, -other.g5_.z) + self.g2_ * vec2(-other.g5_.w) + self.g2_ * vec2(other.g0_.x) + vec2(self.g3_.x) * vec2(-other.g7_.x, 0.0) + vec2(self.g3_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g3_.y) * vec2(-other.g7_.y, 0.0) + vec2(self.g3_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g3_.z) * vec2(-other.g7_.z, 0.0) + vec2(self.g3_.z) * vec2(-other.g1_.z, 0.0) - vec2(self.g4_.x) * vec2(other.g6_.x, other.g8_.x) - vec2(self.g4_.y) * vec2(other.g6_.y, other.g8_.y) - vec2(self.g4_.z) * vec2(other.g6_.z, other.g8_.z) + vec2(self.g5_.x) * vec2(0.0, -other.g7_.x) + vec2(self.g5_.x) * vec2(0.0, other.g1_.x) + vec2(self.g5_.y) * vec2(0.0, -other.g7_.y) + vec2(self.g5_.y) * vec2(0.0, other.g1_.y) + vec2(self.g5_.z) * vec2(0.0, -other.g7_.z) + vec2(self.g5_.z) * vec2(0.0, other.g1_.z) + vec2(self.g5_.w) * other.g2_ * vec2(1.0, -1.0) + vec2(self.g6_.x) * vec2(other.g9_.x, 0.0) + vec2(self.g6_.x) * vec2(-other.g4_.x, 0.0) + vec2(self.g6_.y) * vec2(other.g9_.y, 0.0) + vec2(self.g6_.y) * vec2(-other.g4_.y, 0.0) + vec2(self.g6_.z) * vec2(other.g9_.z, 0.0) + vec2(self.g6_.z) * vec2(-other.g4_.z, 0.0) + vec2(self.g6_.w) * other.g10_ * vec2(-1.0, 1.0) - vec2(self.g7_.x) * vec2(other.g3_.x, other.g5_.x) - vec2(self.g7_.y) * vec2(other.g3_.y, other.g5_.y) - vec2(self.g7_.z) * vec2(other.g3_.z, other.g5_.z) + vec2(self.g8_.x) * vec2(0.0, -other.g9_.x) + vec2(self.g8_.x) * vec2(0.0, -other.g4_.x) + vec2(self.g8_.y) * vec2(0.0, -other.g9_.y) + vec2(self.g8_.y) * vec2(0.0, -other.g4_.y) + vec2(self.g8_.z) * vec2(0.0, -other.g9_.z) + vec2(self.g8_.z) * vec2(0.0, -other.g4_.z) + vec2(self.g9_.x) * vec2(-other.g6_.x, other.g8_.x) + vec2(self.g9_.y) * vec2(-other.g6_.y, other.g8_.y) + vec2(self.g9_.z) * vec2(-other.g6_.z, other.g8_.z) + self.g10_ * vec2(other.g6_.w) + self.g10_ * vec2(other.g0_.y));
}

MultiVector multiVector_antiWedgeDot_plane(MultiVector self, Plane other) {
    return MultiVector(vec2(self.g1_.x) * vec2(other.g0_.x, 0.0) + vec2(self.g1_.y) * vec2(other.g0_.y, 0.0) + vec2(self.g1_.z) * vec2(other.g0_.z, 0.0) + vec2(self.g2_.x) * vec2(other.g0_.w, 0.0) + vec2(self.g9_.x) * vec2(0.0, other.g0_.x) + vec2(self.g9_.y) * vec2(0.0, other.g0_.y) + vec2(self.g9_.z) * vec2(0.0, other.g0_.z) + vec2(self.g10_.x) * vec2(0.0, -other.g0_.w), vec3(self.g0_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + self.g3_ * vec3(other.g0_.w) + vec3(self.g4_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g4_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g4_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec2(self.g0_.x) * vec2(0.0, -other.g0_.w) + vec2(self.g3_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g3_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g3_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g5_.x) * vec2(0.0, other.g0_.x) + vec2(self.g5_.y) * vec2(0.0, other.g0_.y) + vec2(self.g5_.z) * vec2(0.0, other.g0_.z) + vec2(self.g5_.w) * vec2(0.0, other.g0_.w), vec3(0.0) - vec3(self.g2_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g6_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g6_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g6_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec3(self.g1_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g1_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g1_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + vec3(self.g6_.x, self.g6_.y, self.g6_.z) * vec3(other.g0_.w) - vec3(self.g6_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec4(self.g1_.x, self.g1_.y, self.g1_.z, self.g1_.x) * vec4(other.g0_.w, other.g0_.w, other.g0_.w, 0.0) + vec4(self.g2_.y, self.g2_.y, self.g2_.y, self.g2_.x) * other.g0_ + vec4(self.g7_.x) * vec4(other.g0_.w, 0.0, 0.0, -other.g0_.x) + vec4(self.g7_.y) * vec4(0.0, other.g0_.w, 0.0, -other.g0_.y) + vec4(self.g7_.z) * vec4(0.0, 0.0, other.g0_.w, -other.g0_.z) + vec4(self.g8_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g8_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g8_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0), vec4(self.g3_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g3_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g3_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0) + vec4(self.g4_.x) * vec4(0.0, 0.0, 0.0, -other.g0_.x) + vec4(self.g4_.y) * vec4(0.0, 0.0, 0.0, -other.g0_.y) + vec4(self.g4_.z) * vec4(0.0, 0.0, 0.0, -other.g0_.z) + vec4(self.g10_.x) * other.g0_, vec3(0.0) - self.g3_ * vec3(other.g0_.w) - vec3(self.g5_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g9_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g9_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g9_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec3(0.0) - self.g4_ * vec3(other.g0_.w) + vec3(self.g5_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g5_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g5_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + self.g9_ * vec3(other.g0_.w) - vec3(self.g10_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(self.g0_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g6_.x, self.g6_.y, self.g6_.z) * vec3(other.g0_.w) + vec3(self.g7_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g7_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g7_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec2(self.g0_.y) * vec2(0.0, other.g0_.w) + vec2(self.g6_.x) * vec2(other.g0_.x, 0.0) + vec2(self.g6_.y) * vec2(other.g0_.y, 0.0) + vec2(self.g6_.z, self.g6_.w) * vec2(other.g0_.z, other.g0_.w) + vec2(self.g8_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g8_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g8_.z) * vec2(0.0, -other.g0_.z));
}

MultiVector multiVector_antiWedgeDot_roundPoint(MultiVector self, RoundPoint other) {
    return MultiVector(vec2(self.g1_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g2_.x) * vec2(0.0, other.g1_.y) + vec2(self.g2_.y) * vec2(0.0, other.g1_.x) + vec2(self.g9_.x) * vec2(other.g0_.x, 0.0) + vec2(self.g9_.y) * vec2(other.g0_.y, 0.0) + vec2(self.g9_.z) * vec2(other.g0_.z, 0.0) + vec2(self.g10_.x) * vec2(other.g1_.y, 0.0) + vec2(self.g10_.y) * vec2(other.g1_.x, 0.0), vec3(self.g0_.y) * other.g0_ - vec3(self.g6_.x, self.g6_.y, self.g6_.z) * vec3(other.g1_.y) + vec3(self.g7_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g7_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g7_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) + self.g8_ * vec3(other.g1_.x), vec2(self.g0_.y) * other.g1_ + vec2(self.g6_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g6_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g6_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g6_.w) * other.g1_ * vec2(-1.0, 1.0) + vec2(self.g8_.x) * vec2(0.0, other.g0_.x) + vec2(self.g8_.y) * vec2(0.0, other.g0_.y) + vec2(self.g8_.z) * vec2(0.0, other.g0_.z), vec3(self.g3_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g3_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g3_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) + self.g4_ * vec3(other.g1_.x) + self.g9_ * vec3(other.g1_.x) + vec3(self.g10_.x) * other.g0_, self.g3_ * vec3(other.g1_.y) + vec3(self.g5_.x, self.g5_.y, self.g5_.z) * vec3(other.g1_.x) - vec3(self.g5_.w) * other.g0_ + vec3(self.g9_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g9_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g9_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec4(self.g4_.x) * vec4(other.g1_.y, 0.0, 0.0, other.g0_.x) + vec4(self.g4_.y) * vec4(0.0, other.g1_.y, 0.0, other.g0_.y) + vec4(self.g4_.z) * vec4(0.0, 0.0, other.g1_.y, other.g0_.z) + vec4(self.g5_.x) * vec4(0.0, other.g0_.z, -other.g0_.y, 0.0) + vec4(self.g5_.y) * vec4(-other.g0_.z, 0.0, other.g0_.x, 0.0) + vec4(self.g5_.z) * vec4(other.g0_.y, -other.g0_.x, 0.0, 0.0) + vec4(self.g9_.x, self.g9_.y, self.g9_.z, self.g9_.x) * vec4(-other.g1_.y, -other.g1_.y, -other.g1_.y, 0.0) + vec4(self.g10_.x) * vec4(0.0, 0.0, 0.0, other.g1_.y) - vec4(self.g10_.y) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.x), vec4(self.g1_.x, self.g1_.y, self.g1_.z, self.g1_.x) * vec4(-other.g1_.x, -other.g1_.x, -other.g1_.x, 0.0) + vec4(self.g2_.x) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, -other.g1_.y) + vec4(self.g2_.y) * vec4(0.0, 0.0, 0.0, other.g1_.x) + vec4(self.g6_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g6_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g6_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0) + vec4(self.g7_.x) * vec4(other.g1_.x, 0.0, 0.0, -other.g0_.x) + vec4(self.g7_.y) * vec4(0.0, other.g1_.x, 0.0, -other.g0_.y) + vec4(self.g7_.z) * vec4(0.0, 0.0, other.g1_.x, -other.g0_.z), vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) + vec3(self.g6_.x, self.g6_.y, self.g6_.z) * vec3(other.g1_.y) + vec3(self.g6_.w) * other.g0_ + self.g8_ * vec3(other.g1_.x), self.g1_ * vec3(other.g1_.y) - vec3(self.g2_.y) * other.g0_ + self.g7_ * vec3(other.g1_.y) + vec3(self.g8_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g8_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g8_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec3(0.0) - vec3(self.g0_.x) * other.g0_ + self.g3_ * vec3(other.g1_.y) + vec3(self.g4_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g4_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g4_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) - vec3(self.g5_.x, self.g5_.y, self.g5_.z) * vec3(other.g1_.x), vec2(self.g0_.x) * other.g1_ + vec2(self.g3_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g3_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g3_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g5_.x) * vec2(0.0, other.g0_.x) + vec2(self.g5_.y) * vec2(0.0, other.g0_.y) + vec2(self.g5_.z) * vec2(0.0, other.g0_.z) + vec2(self.g5_.w) * other.g1_ * vec2(1.0, -1.0));
}

MultiVector multiVector_antiWedgeDot_scalar(MultiVector self, Scalar other) {
    return MultiVector(self.g0_.yx * vec2(other.g0_), self.g9_ * vec3(other.g0_), vec2(0.0) - self.g10_ * vec2(other.g0_), vec3(self.g6_.x, self.g6_.y, self.g6_.z) * vec3(other.g0_), self.g7_ * vec3(other.g0_), vec4(self.g6_.w) * vec4(0.0, 0.0, 0.0, -other.g0_) + vec4(self.g8_.x, self.g8_.y, self.g8_.z, self.g8_.x) * vec4(other.g0_, other.g0_, other.g0_, 0.0), vec4(self.g3_.x, self.g3_.y, self.g3_.z, self.g3_.x) * vec4(-other.g0_, -other.g0_, -other.g0_, 0.0) + vec4(self.g5_.w) * vec4(0.0, 0.0, 0.0, other.g0_), vec3(0.0) - self.g4_ * vec3(other.g0_), vec3(0.0) - vec3(self.g5_.x, self.g5_.y, self.g5_.z) * vec3(other.g0_), vec3(0.0) - self.g1_ * vec3(other.g0_), self.g2_ * vec2(other.g0_));
}

MultiVector multiVector_antiWedgeDot_sphere(MultiVector self, Sphere other) {
    return MultiVector(vec2(self.g1_.x) * vec2(other.g0_.x, 0.0) + vec2(self.g1_.y) * vec2(other.g0_.y, 0.0) + vec2(self.g1_.z) * vec2(other.g0_.z, 0.0) + vec2(self.g2_.x) * vec2(other.g1_.y, 0.0) + vec2(self.g2_.y) * vec2(other.g1_.x, 0.0) + vec2(self.g9_.x) * vec2(0.0, other.g0_.x) + vec2(self.g9_.y) * vec2(0.0, other.g0_.y) + vec2(self.g9_.z) * vec2(0.0, other.g0_.z) + vec2(self.g10_.x) * vec2(0.0, -other.g1_.y) + vec2(self.g10_.y) * vec2(0.0, -other.g1_.x), vec3(self.g0_.x) * other.g0_ + self.g3_ * vec3(other.g1_.y) + vec3(self.g4_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g4_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g4_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) - vec3(self.g5_.x, self.g5_.y, self.g5_.z) * vec3(other.g1_.x), vec2(0.0) - vec2(self.g0_.x) * other.g1_ + vec2(self.g3_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g3_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g3_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g5_.x) * vec2(0.0, other.g0_.x) + vec2(self.g5_.y) * vec2(0.0, other.g0_.y) + vec2(self.g5_.z) * vec2(0.0, other.g0_.z) + vec2(self.g5_.w) * other.g1_ * vec2(-1.0, 1.0), vec3(0.0) - self.g1_ * vec3(other.g1_.x) - vec3(self.g2_.x) * other.g0_ + vec3(self.g6_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g6_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g6_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + self.g7_ * vec3(other.g1_.x), vec3(self.g1_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g1_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g1_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + vec3(self.g6_.x, self.g6_.y, self.g6_.z) * vec3(other.g1_.y) - vec3(self.g6_.w) * other.g0_ + self.g8_ * vec3(other.g1_.x), vec4(self.g1_.x, self.g1_.y, self.g1_.z, self.g1_.x) * vec4(other.g1_.y, other.g1_.y, other.g1_.y, 0.0) + vec4(self.g2_.x) * vec4(0.0, 0.0, 0.0, other.g1_.y) + vec4(self.g2_.y) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, -other.g1_.x) + vec4(self.g7_.x) * vec4(other.g1_.y, 0.0, 0.0, -other.g0_.x) + vec4(self.g7_.y) * vec4(0.0, other.g1_.y, 0.0, -other.g0_.y) + vec4(self.g7_.z) * vec4(0.0, 0.0, other.g1_.y, -other.g0_.z) + vec4(self.g8_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g8_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g8_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0), vec4(self.g3_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g3_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g3_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0) + vec4(self.g4_.x) * vec4(-other.g1_.x, 0.0, 0.0, -other.g0_.x) + vec4(self.g4_.y) * vec4(0.0, -other.g1_.x, 0.0, -other.g0_.y) + vec4(self.g4_.z) * vec4(0.0, 0.0, -other.g1_.x, -other.g0_.z) + vec4(self.g9_.x, self.g9_.y, self.g9_.z, self.g9_.x) * vec4(-other.g1_.x, -other.g1_.x, -other.g1_.x, 0.0) + vec4(self.g10_.x) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.y) + vec4(self.g10_.y) * vec4(0.0, 0.0, 0.0, -other.g1_.x), vec3(0.0) - self.g3_ * vec3(other.g1_.y) - vec3(self.g5_.x, self.g5_.y, self.g5_.z) * vec3(other.g1_.x) - vec3(self.g5_.w) * other.g0_ + vec3(self.g9_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g9_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g9_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec3(0.0) - self.g4_ * vec3(other.g1_.y) + vec3(self.g5_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g5_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g5_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + self.g9_ * vec3(other.g1_.y) - vec3(self.g10_.y) * other.g0_, vec3(self.g0_.y) * other.g0_ + vec3(self.g6_.x, self.g6_.y, self.g6_.z) * vec3(other.g1_.y) + vec3(self.g7_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g7_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g7_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) - self.g8_ * vec3(other.g1_.x), vec2(self.g0_.y) * other.g1_ + vec2(self.g6_.x) * vec2(other.g0_.x, 0.0) + vec2(self.g6_.y) * vec2(other.g0_.y, 0.0) + vec2(self.g6_.z) * vec2(other.g0_.z, 0.0) + vec2(self.g6_.w) * other.g1_ * vec2(-1.0, 1.0) + vec2(self.g8_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g8_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g8_.z) * vec2(0.0, -other.g0_.z));
}

Plane plane_antiWedgeDot_antiScalar(Plane self, AntiScalar other) {
    return Plane(self.g0_ * vec4(other.g0_));
}

MultiVector plane_antiWedgeDot_circle(Plane self, Circle other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(0.0) - vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_.w) + vec3(self.g0_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec4(self.g0_.x) * vec4(0.0, other.g2_.z, -other.g2_.y, -other.g1_.x) + vec4(self.g0_.y) * vec4(-other.g2_.z, 0.0, other.g2_.x, -other.g1_.y) + vec4(self.g0_.z) * vec4(other.g2_.y, -other.g2_.x, 0.0, -other.g1_.z) + vec4(self.g0_.w) * vec4(other.g1_.x, other.g1_.y, other.g1_.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) - vec3(self.g0_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g0_.x) * vec2(-other.g0_.x, other.g2_.x) + vec2(self.g0_.y) * vec2(-other.g0_.y, other.g2_.y) + vec2(self.g0_.z) * vec2(-other.g0_.z, other.g2_.z) + vec2(self.g0_.w) * vec2(0.0, -other.g0_.w));
}

MultiVector plane_antiWedgeDot_dipole(Plane self, Dipole other) {
    return MultiVector(vec2(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) - vec3(self.g0_.w) * other.g0_, vec2(self.g0_.x) * vec2(other.g0_.x, -other.g2_.x) + vec2(self.g0_.y) * vec2(other.g0_.y, -other.g2_.y) + vec2(self.g0_.z) * vec2(other.g0_.z, -other.g2_.z) + vec2(self.g0_.w) * vec2(0.0, -other.g2_.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(0.0, other.g0_.z, -other.g0_.y, -other.g1_.x) + vec4(self.g0_.y) * vec4(-other.g0_.z, 0.0, other.g0_.x, -other.g1_.y) + vec4(self.g0_.z) * vec4(other.g0_.y, -other.g0_.x, 0.0, -other.g1_.z), vec3(0.0) - vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g2_.w) - vec3(self.g0_.w) * other.g0_, vec3(self.g0_.x) * vec3(0.0, -other.g2_.z, other.g2_.y) + vec3(self.g0_.y) * vec3(other.g2_.z, 0.0, -other.g2_.x) + vec3(self.g0_.z) * vec3(-other.g2_.y, other.g2_.x, 0.0) - vec3(self.g0_.w) * other.g1_, vec3(0.0), vec2(0.0));
}

MultiVector plane_antiWedgeDot_dualNum(Plane self, DualNum other) {
    return MultiVector(vec2(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_.x), vec2(self.g0_.w) * vec2(0.0, -other.g0_.x), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_.y), vec2(self.g0_.w) * vec2(0.0, other.g0_.y));
}

MultiVector plane_antiWedgeDot_flatPoint(Plane self, FlatPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(self.g0_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g0_.w) * vec2(0.0, -other.g0_.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0) - vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_.w), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(0.0), vec2(0.0));
}

MultiVector plane_antiWedgeDot_flector(Plane self, Flector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, other.g1_.z), vec3(0.0), vec2(self.g0_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g0_.w) * vec2(0.0, -other.g0_.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0_.x) * vec3(-other.g0_.w, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, -other.g0_.w, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, -other.g0_.w), vec3(self.g0_.x) * vec3(other.g1_.w, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, other.g1_.w, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, other.g1_.w) - vec3(self.g0_.w) * vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec3(0.0), vec2(0.0));
}

Flector plane_antiWedgeDot_line(Plane self, Line other) {
    return Flector(vec4(self.g0_.x) * vec4(0.0, other.g1_.z, -other.g1_.y, -other.g0_.x) + vec4(self.g0_.y) * vec4(-other.g1_.z, 0.0, other.g1_.x, -other.g0_.y) + vec4(self.g0_.z) * vec4(other.g1_.y, -other.g1_.x, 0.0, -other.g0_.z) + vec4(self.g0_.w) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, 0.0), vec4(self.g0_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, other.g1_.x) + vec4(self.g0_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, other.g1_.y) + vec4(self.g0_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, other.g1_.z));
}

Flector plane_antiWedgeDot_motor(Plane self, Motor other) {
    return Flector(vec4(self.g0_.x) * vec4(0.0, other.g1_.z, -other.g1_.y, -other.g0_.x) + vec4(self.g0_.y) * vec4(-other.g1_.z, 0.0, other.g1_.x, -other.g0_.y) + vec4(self.g0_.z) * vec4(other.g1_.y, -other.g1_.x, 0.0, -other.g0_.z) + vec4(self.g0_.w) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, 0.0), vec4(self.g0_.x) * vec4(other.g0_.w, -other.g0_.z, other.g0_.y, other.g1_.x) + vec4(self.g0_.y) * vec4(other.g0_.z, other.g0_.w, -other.g0_.x, other.g1_.y) + vec4(self.g0_.z) * vec4(-other.g0_.y, other.g0_.x, other.g0_.w, other.g1_.z) + vec4(self.g0_.w) * vec4(0.0, 0.0, 0.0, other.g0_.w));
}

MultiVector plane_antiWedgeDot_multiVector(Plane self, MultiVector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(other.g1_.x, other.g9_.x) + vec2(self.g0_.y) * vec2(other.g1_.y, other.g9_.y) + vec2(self.g0_.z) * vec2(other.g1_.z, other.g9_.z) + vec2(self.g0_.w) * vec2(other.g2_.x, -other.g10_.x), vec3(self.g0_.x) * vec3(other.g0_.x, -other.g4_.z, other.g4_.y) + vec3(self.g0_.y) * vec3(other.g4_.z, other.g0_.x, -other.g4_.x) + vec3(self.g0_.z) * vec3(-other.g4_.y, other.g4_.x, other.g0_.x) - vec3(self.g0_.w) * other.g3_, vec2(self.g0_.x) * vec2(other.g3_.x, -other.g5_.x) + vec2(self.g0_.y) * vec2(other.g3_.y, -other.g5_.y) + vec2(self.g0_.z) * vec2(other.g3_.z, -other.g5_.z) + vec2(self.g0_.w) * vec2(0.0, -other.g5_.w) + vec2(self.g0_.w) * vec2(0.0, -other.g0_.x), vec3(self.g0_.x) * vec3(other.g2_.x, -other.g6_.z, other.g6_.y) + vec3(self.g0_.y) * vec3(other.g6_.z, other.g2_.x, -other.g6_.x) + vec3(self.g0_.z) * vec3(-other.g6_.y, other.g6_.x, other.g2_.x), vec3(self.g0_.x) * vec3(-other.g6_.w, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, -other.g6_.w, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, -other.g6_.w) + vec3(self.g0_.w) * vec3(other.g6_.x, other.g6_.y, other.g6_.z), vec4(self.g0_.x) * vec4(-other.g2_.y, other.g8_.z, -other.g8_.y, -other.g7_.x) + vec4(self.g0_.y) * vec4(-other.g8_.z, -other.g2_.y, other.g8_.x, -other.g7_.y) + vec4(self.g0_.z) * vec4(other.g8_.y, -other.g8_.x, -other.g2_.y, -other.g7_.z) + vec4(self.g0_.w) * vec4(other.g7_.x, other.g7_.y, other.g7_.z, -other.g2_.x) + vec4(self.g0_.w) * vec4(-other.g1_.x, -other.g1_.y, -other.g1_.z, 0.0), vec4(self.g0_.x) * vec4(-other.g10_.x, other.g3_.z, -other.g3_.y, -other.g4_.x) + vec4(self.g0_.y) * vec4(-other.g3_.z, -other.g10_.x, other.g3_.x, -other.g4_.y) + vec4(self.g0_.z) * vec4(other.g3_.y, -other.g3_.x, -other.g10_.x, -other.g4_.z) + vec4(self.g0_.w) * vec4(0.0, 0.0, 0.0, -other.g10_.x), vec3(self.g0_.x) * vec3(-other.g5_.w, other.g9_.z, -other.g9_.y) + vec3(self.g0_.y) * vec3(-other.g9_.z, -other.g5_.w, other.g9_.x) + vec3(self.g0_.z) * vec3(other.g9_.y, -other.g9_.x, -other.g5_.w) - vec3(self.g0_.w) * other.g3_, vec3(self.g0_.x) * vec3(other.g10_.y, -other.g5_.z, other.g5_.y) + vec3(self.g0_.y) * vec3(other.g5_.z, other.g10_.y, -other.g5_.x) + vec3(self.g0_.z) * vec3(-other.g5_.y, other.g5_.x, other.g10_.y) - vec3(self.g0_.w) * other.g9_ - vec3(self.g0_.w) * other.g4_, vec3(self.g0_.x) * vec3(other.g0_.y, -other.g7_.z, other.g7_.y) + vec3(self.g0_.y) * vec3(other.g7_.z, other.g0_.y, -other.g7_.x) + vec3(self.g0_.z) * vec3(-other.g7_.y, other.g7_.x, other.g0_.y) - vec3(self.g0_.w) * vec3(other.g6_.x, other.g6_.y, other.g6_.z), vec2(self.g0_.x) * vec2(-other.g6_.x, other.g8_.x) + vec2(self.g0_.y) * vec2(-other.g6_.y, other.g8_.y) + vec2(self.g0_.z) * vec2(-other.g6_.z, other.g8_.z) + vec2(self.g0_.w) * vec2(0.0, -other.g6_.w) + vec2(self.g0_.w) * vec2(0.0, other.g0_.y));
}

Motor plane_antiWedgeDot_plane(Plane self, Plane other) {
    return Motor(vec4(self.g0_.x) * vec4(0.0, other.g0_.z, -other.g0_.y, other.g0_.x) + vec4(self.g0_.y) * vec4(-other.g0_.z, 0.0, other.g0_.x, other.g0_.y) + vec4(self.g0_.z) * vec4(other.g0_.y, -other.g0_.x, 0.0, other.g0_.z), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_.w) - vec3(self.g0_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z));
}

MultiVector plane_antiWedgeDot_roundPoint(Plane self, RoundPoint other) {
    return MultiVector(vec2(self.g0_.x) * vec2(other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(other.g0_.z, 0.0) + vec2(self.g0_.w) * vec2(other.g1_.x, 0.0), vec3(0.0), vec2(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g1_.x), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), self.g0_.xyzx * vec4(-other.g1_.y, -other.g1_.y, -other.g1_.y, 0.0) - vec4(self.g0_.w) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.x), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

RoundPoint plane_antiWedgeDot_scalar(Plane self, Scalar other) {
    return RoundPoint(vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_), vec2(self.g0_.w) * vec2(0.0, -other.g0_));
}

MultiVector plane_antiWedgeDot_sphere(Plane self, Sphere other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, other.g0_.z) + vec2(self.g0_.w) * vec2(0.0, -other.g1_.x), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0) - self.g0_ * vec4(other.g1_.x), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g1_.y) - vec3(self.g0_.w) * other.g0_, vec3(0.0), vec2(0.0));
}

RoundPoint roundPoint_antiWedgeDot_antiScalar(RoundPoint self, AntiScalar other) {
    return RoundPoint(self.g0_ * vec3(other.g0_), self.g1_ * vec2(other.g0_));
}

MultiVector roundPoint_antiWedgeDot_circle(RoundPoint self, Circle other) {
    return MultiVector(vec2(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) - vec3(self.g1_.x) * other.g2_ + vec3(self.g1_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g0_.x) * vec2(other.g0_.x, -other.g2_.x) + vec2(self.g0_.y) * vec2(other.g0_.y, -other.g2_.y) + vec2(self.g0_.z) * vec2(other.g0_.z, -other.g2_.z) + self.g1_ * vec2(other.g0_.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(0.0, other.g0_.z, -other.g0_.y, -other.g1_.x) + vec4(self.g0_.y) * vec4(-other.g0_.z, 0.0, other.g0_.x, -other.g1_.y) + vec4(self.g0_.z) * vec4(other.g0_.y, -other.g0_.x, 0.0, -other.g1_.z) + vec4(self.g1_.x) * vec4(other.g1_.x, other.g1_.y, other.g1_.z, 0.0), self.g0_ * vec3(other.g0_.w) + vec3(self.g1_.x) * other.g2_ + vec3(self.g1_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(self.g0_.x) * vec3(0.0, -other.g2_.z, other.g2_.y) + vec3(self.g0_.y) * vec3(other.g2_.z, 0.0, -other.g2_.x) + vec3(self.g0_.z) * vec3(-other.g2_.y, other.g2_.x, 0.0) + vec3(self.g1_.y) * other.g1_, vec3(0.0), vec2(0.0));
}

MultiVector roundPoint_antiWedgeDot_dipole(RoundPoint self, Dipole other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + vec3(self.g1_.x) * other.g1_, vec3(0.0) - self.g0_ * vec3(other.g2_.w) + vec3(self.g1_.x) * vec3(other.g2_.x, other.g2_.y, other.g2_.z) + vec3(self.g1_.y) * other.g0_, vec4(self.g0_.x) * vec4(0.0, -other.g2_.z, other.g2_.y, other.g1_.x) + vec4(self.g0_.y) * vec4(other.g2_.z, 0.0, -other.g2_.x, other.g1_.y) + vec4(self.g0_.z) * vec4(-other.g2_.y, other.g2_.x, 0.0, other.g1_.z) + vec4(self.g1_.y) * vec4(other.g1_.x, other.g1_.y, other.g1_.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g1_.x) * vec3(other.g2_.x, other.g2_.y, other.g2_.z) - vec3(self.g1_.y) * other.g0_, vec2(self.g0_.x) * vec2(other.g0_.x, -other.g2_.x) + vec2(self.g0_.y) * vec2(other.g0_.y, -other.g2_.y) + vec2(self.g0_.z) * vec2(other.g0_.z, -other.g2_.z) + self.g1_ * vec2(-other.g2_.w));
}

MultiVector roundPoint_antiWedgeDot_dualNum(RoundPoint self, DualNum other) {
    return MultiVector(vec2(0.0), self.g0_ * vec3(other.g0_.y), self.g1_ * vec2(other.g0_.y), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - self.g0_ * vec3(other.g0_.x), self.g1_ * vec2(other.g0_.x));
}

MultiVector roundPoint_antiWedgeDot_flatPoint(RoundPoint self, FlatPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0) - self.g0_ * vec3(other.g0_.w) + vec3(self.g1_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec4(self.g0_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g0_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g0_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g1_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g0_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g0_.z) + self.g1_ * vec2(-other.g0_.w));
}

MultiVector roundPoint_antiWedgeDot_flector(RoundPoint self, Flector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(other.g1_.x, 0.0) + vec2(self.g0_.y) * vec2(other.g1_.y, 0.0) + vec2(self.g0_.z) * vec2(other.g1_.z, 0.0) + vec2(self.g1_.x) * vec2(other.g1_.w, 0.0), vec3(0.0), vec2(0.0), vec3(0.0) - vec3(self.g1_.x) * vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec3(self.g0_.x) * vec3(-other.g0_.w, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, -other.g0_.w, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, -other.g0_.w) + vec3(self.g1_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec4(self.g0_.x) * vec4(other.g1_.w, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g0_.y) * vec4(other.g0_.z, other.g1_.w, -other.g0_.x, 0.0) + vec4(self.g0_.z) * vec4(-other.g0_.y, other.g0_.x, other.g1_.w, 0.0) + vec4(self.g1_.y, self.g1_.y, self.g1_.y, self.g1_.x) * other.g1_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g1_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g0_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g0_.z) + self.g1_ * vec2(-other.g0_.w));
}

MultiVector roundPoint_antiWedgeDot_line(RoundPoint self, Line other) {
    return MultiVector(vec2(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) - vec3(self.g1_.x) * other.g1_, vec2(self.g0_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g1_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, -other.g0_.x) + vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, -other.g0_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, 0.0, -other.g0_.z) + vec4(self.g1_.x) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, 0.0), vec3(self.g1_.x) * other.g1_, vec3(self.g0_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g1_.y) * other.g0_, vec3(0.0), vec2(0.0));
}

MultiVector roundPoint_antiWedgeDot_motor(RoundPoint self, Motor other) {
    return MultiVector(vec2(0.0), vec3(self.g0_.x) * vec3(other.g0_.w, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, other.g0_.w, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, other.g0_.w) - vec3(self.g1_.x) * other.g1_, vec2(self.g0_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g1_.z) + self.g1_ * vec2(other.g0_.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, -other.g0_.x) + vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, -other.g0_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, 0.0, -other.g0_.z) + vec4(self.g1_.x) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, 0.0), vec3(self.g1_.x) * other.g1_, vec3(self.g0_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g1_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(0.0), vec2(0.0));
}

MultiVector roundPoint_antiWedgeDot_multiVector(RoundPoint self, MultiVector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(other.g9_.x, -other.g1_.x) + vec2(self.g0_.y) * vec2(other.g9_.y, -other.g1_.y) + vec2(self.g0_.z) * vec2(other.g9_.z, -other.g1_.z) + vec2(self.g1_.x) * vec2(other.g10_.y, other.g2_.y) + vec2(self.g1_.y) * vec2(other.g10_.x, other.g2_.x), vec3(self.g0_.x) * vec3(other.g0_.y, -other.g7_.z, other.g7_.y) + vec3(self.g0_.y) * vec3(other.g7_.z, other.g0_.y, -other.g7_.x) + vec3(self.g0_.z) * vec3(-other.g7_.y, other.g7_.x, other.g0_.y) - vec3(self.g1_.x) * other.g8_ + vec3(self.g1_.y) * vec3(other.g6_.x, other.g6_.y, other.g6_.z), vec2(self.g0_.x) * vec2(other.g6_.x, -other.g8_.x) + vec2(self.g0_.y) * vec2(other.g6_.y, -other.g8_.y) + vec2(self.g0_.z) * vec2(other.g6_.z, -other.g8_.z) + self.g1_ * vec2(other.g6_.w) + self.g1_ * vec2(other.g0_.y), vec3(self.g0_.x) * vec3(-other.g10_.x, other.g3_.z, -other.g3_.y) + vec3(self.g0_.y) * vec3(-other.g3_.z, -other.g10_.x, other.g3_.x) + vec3(self.g0_.z) * vec3(other.g3_.y, -other.g3_.x, -other.g10_.x) - vec3(self.g1_.x) * other.g9_ + vec3(self.g1_.x) * other.g4_, vec3(self.g0_.x) * vec3(-other.g5_.w, other.g9_.z, -other.g9_.y) + vec3(self.g0_.y) * vec3(-other.g9_.z, -other.g5_.w, other.g9_.x) + vec3(self.g0_.z) * vec3(other.g9_.y, -other.g9_.x, -other.g5_.w) + vec3(self.g1_.x) * vec3(other.g5_.x, other.g5_.y, other.g5_.z) + vec3(self.g1_.y) * other.g3_, vec4(self.g0_.x) * vec4(other.g10_.y, -other.g5_.z, other.g5_.y, other.g4_.x) + vec4(self.g0_.y) * vec4(other.g5_.z, other.g10_.y, -other.g5_.x, other.g4_.y) + vec4(self.g0_.z) * vec4(-other.g5_.y, other.g5_.x, other.g10_.y, other.g4_.z) + vec4(self.g1_.x) * vec4(0.0, 0.0, 0.0, other.g10_.y) + vec4(self.g1_.y) * vec4(other.g9_.x, other.g9_.y, other.g9_.z, -other.g10_.x) + vec4(self.g1_.y) * vec4(other.g4_.x, other.g4_.y, other.g4_.z, 0.0), vec4(self.g0_.x) * vec4(-other.g2_.x, other.g6_.z, -other.g6_.y, -other.g7_.x) + vec4(self.g0_.y) * vec4(-other.g6_.z, -other.g2_.x, other.g6_.x, -other.g7_.y) + vec4(self.g0_.z) * vec4(other.g6_.y, -other.g6_.x, -other.g2_.x, -other.g7_.z) + vec4(self.g1_.x) * vec4(other.g7_.x, other.g7_.y, other.g7_.z, -other.g2_.y) + vec4(self.g1_.x) * vec4(other.g1_.x, other.g1_.y, other.g1_.z, 0.0) + vec4(self.g1_.y) * vec4(0.0, 0.0, 0.0, other.g2_.x), vec3(self.g0_.x) * vec3(other.g6_.w, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, other.g6_.w, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, other.g6_.w) + vec3(self.g1_.x) * other.g8_ + vec3(self.g1_.y) * vec3(other.g6_.x, other.g6_.y, other.g6_.z), vec3(self.g0_.x) * vec3(other.g2_.y, -other.g8_.z, other.g8_.y) + vec3(self.g0_.y) * vec3(other.g8_.z, other.g2_.y, -other.g8_.x) + vec3(self.g0_.z) * vec3(-other.g8_.y, other.g8_.x, other.g2_.y) + vec3(self.g1_.y) * other.g7_ - vec3(self.g1_.y) * other.g1_, vec3(self.g0_.x) * vec3(-other.g0_.x, other.g4_.z, -other.g4_.y) + vec3(self.g0_.y) * vec3(-other.g4_.z, -other.g0_.x, other.g4_.x) + vec3(self.g0_.z) * vec3(other.g4_.y, -other.g4_.x, -other.g0_.x) + vec3(self.g1_.x) * vec3(other.g5_.x, other.g5_.y, other.g5_.z) - vec3(self.g1_.y) * other.g3_, vec2(self.g0_.x) * vec2(other.g3_.x, -other.g5_.x) + vec2(self.g0_.y) * vec2(other.g3_.y, -other.g5_.y) + vec2(self.g0_.z) * vec2(other.g3_.z, -other.g5_.z) + self.g1_ * vec2(-other.g5_.w) + self.g1_ * vec2(other.g0_.x));
}

MultiVector roundPoint_antiWedgeDot_plane(RoundPoint self, Plane other) {
    return MultiVector(vec2(self.g0_.x) * vec2(other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(other.g0_.z, 0.0) + vec2(self.g1_.x) * vec2(other.g0_.w, 0.0), vec3(0.0), vec2(0.0), vec3(0.0) - vec3(self.g1_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec4(self.g0_.x, self.g0_.y, self.g0_.z, self.g0_.x) * vec4(other.g0_.w, other.g0_.w, other.g0_.w, 0.0) + vec4(self.g1_.y, self.g1_.y, self.g1_.y, self.g1_.x) * other.g0_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector roundPoint_antiWedgeDot_roundPoint(RoundPoint self, RoundPoint other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g1_.x) * vec2(0.0, other.g1_.y) + vec2(self.g1_.y) * vec2(0.0, other.g1_.x), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x, self.g0_.y, self.g0_.z, self.g0_.x) * vec4(-other.g1_.x, -other.g1_.x, -other.g1_.x, 0.0) + vec4(self.g1_.x) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, -other.g1_.y) + vec4(self.g1_.y) * vec4(0.0, 0.0, 0.0, other.g1_.x), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), self.g0_ * vec3(other.g1_.y) - vec3(self.g1_.y) * other.g0_, vec3(0.0), vec2(0.0));
}

Sphere roundPoint_antiWedgeDot_scalar(RoundPoint self, Scalar other) {
    return Sphere(vec3(0.0) - self.g0_ * vec3(other.g0_), self.g1_ * vec2(other.g0_));
}

MultiVector roundPoint_antiWedgeDot_sphere(RoundPoint self, Sphere other) {
    return MultiVector(vec2(self.g0_.x) * vec2(other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(other.g0_.z, 0.0) + vec2(self.g1_.x) * vec2(other.g1_.y, 0.0) + vec2(self.g1_.y) * vec2(other.g1_.x, 0.0), vec3(0.0), vec2(0.0), vec3(0.0) - self.g0_ * vec3(other.g1_.x) - vec3(self.g1_.x) * other.g0_, vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec4(self.g0_.x, self.g0_.y, self.g0_.z, self.g0_.x) * vec4(other.g1_.y, other.g1_.y, other.g1_.y, 0.0) + vec4(self.g1_.x) * vec4(0.0, 0.0, 0.0, other.g1_.y) + vec4(self.g1_.y) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, -other.g1_.x), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

Scalar scalar_antiWedgeDot_antiScalar(Scalar self, AntiScalar other) {
    return Scalar(self.g0_ * other.g0_);
}

Dipole scalar_antiWedgeDot_circle(Scalar self, Circle other) {
    return Dipole(vec3(self.g0_) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(self.g0_) * other.g1_, vec4(self.g0_) * vec4(other.g2_.x, other.g2_.y, other.g2_.z, -other.g0_.w));
}

Circle scalar_antiWedgeDot_dipole(Scalar self, Dipole other) {
    return Circle(vec4(self.g0_) * vec4(-other.g0_.x, -other.g0_.y, -other.g0_.z, other.g2_.w), vec3(0.0) - vec3(self.g0_) * other.g1_, vec3(0.0) - vec3(self.g0_) * vec3(other.g2_.x, other.g2_.y, other.g2_.z));
}

DualNum scalar_antiWedgeDot_dualNum(Scalar self, DualNum other) {
    return DualNum(vec2(self.g0_) * other.g0_.yx * vec2(1.0, -1.0));
}

Circle scalar_antiWedgeDot_flatPoint(Scalar self, FlatPoint other) {
    return Circle(vec4(self.g0_) * vec4(0.0, 0.0, 0.0, other.g0_.w), vec3(0.0), vec3(0.0) - vec3(self.g0_) * vec3(other.g0_.x, other.g0_.y, other.g0_.z));
}

MultiVector scalar_antiWedgeDot_flector(Scalar self, Flector other) {
    return MultiVector(vec2(0.0), vec3(self.g0_) * vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec2(self.g0_) * vec2(0.0, -other.g1_.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_) * vec4(0.0, 0.0, 0.0, other.g0_.w), vec3(0.0), vec3(0.0) - vec3(self.g0_) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(0.0), vec2(0.0));
}

Dipole scalar_antiWedgeDot_line(Scalar self, Line other) {
    return Dipole(vec3(0.0), vec3(self.g0_) * other.g0_, vec4(self.g0_) * vec4(other.g1_.x, other.g1_.y, other.g1_.z, 0.0));
}

MultiVector scalar_antiWedgeDot_motor(Scalar self, Motor other) {
    return MultiVector(vec2(self.g0_) * vec2(other.g0_.w, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(self.g0_) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec4(self.g0_) * vec4(other.g1_.x, other.g1_.y, other.g1_.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector scalar_antiWedgeDot_multiVector(Scalar self, MultiVector other) {
    return MultiVector(vec2(self.g0_) * other.g0_.yx * vec2(1.0, -1.0), vec3(self.g0_) * other.g9_, vec2(0.0) - vec2(self.g0_) * other.g10_, vec3(self.g0_) * vec3(other.g6_.x, other.g6_.y, other.g6_.z), vec3(self.g0_) * other.g7_, vec4(self.g0_) * vec4(other.g8_.x, other.g8_.y, other.g8_.z, -other.g6_.w), vec4(self.g0_) * vec4(-other.g3_.x, -other.g3_.y, -other.g3_.z, other.g5_.w), vec3(0.0) - vec3(self.g0_) * other.g4_, vec3(0.0) - vec3(self.g0_) * vec3(other.g5_.x, other.g5_.y, other.g5_.z), vec3(0.0) - vec3(self.g0_) * other.g1_, vec2(self.g0_) * other.g2_);
}

RoundPoint scalar_antiWedgeDot_plane(Scalar self, Plane other) {
    return RoundPoint(vec3(self.g0_) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g0_) * vec2(0.0, -other.g0_.w));
}

Sphere scalar_antiWedgeDot_roundPoint(Scalar self, RoundPoint other) {
    return Sphere(vec3(0.0) - vec3(self.g0_) * other.g0_, vec2(self.g0_) * other.g1_);
}

AntiScalar scalar_antiWedgeDot_scalar(Scalar self, Scalar other) {
    return AntiScalar(0.0 - self.g0_ * other.g0_);
}

RoundPoint scalar_antiWedgeDot_sphere(Scalar self, Sphere other) {
    return RoundPoint(vec3(self.g0_) * other.g0_, vec2(0.0) - vec2(self.g0_) * other.g1_);
}

Sphere sphere_antiWedgeDot_antiScalar(Sphere self, AntiScalar other) {
    return Sphere(self.g0_ * vec3(other.g0_), self.g1_ * vec2(other.g0_));
}

MultiVector sphere_antiWedgeDot_circle(Sphere self, Circle other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) + vec3(self.g1_.x) * other.g1_, vec3(0.0) - self.g0_ * vec3(other.g0_.w) + vec3(self.g1_.x) * other.g2_ + vec3(self.g1_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec4(self.g0_.x) * vec4(0.0, other.g2_.z, -other.g2_.y, -other.g1_.x) + vec4(self.g0_.y) * vec4(-other.g2_.z, 0.0, other.g2_.x, -other.g1_.y) + vec4(self.g0_.z) * vec4(other.g2_.y, -other.g2_.x, 0.0, -other.g1_.z) + vec4(self.g1_.y) * vec4(other.g1_.x, other.g1_.y, other.g1_.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g1_.x) * other.g2_ - vec3(self.g1_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g0_.x) * vec2(-other.g0_.x, other.g2_.x) + vec2(self.g0_.y) * vec2(-other.g0_.y, other.g2_.y) + vec2(self.g0_.z) * vec2(-other.g0_.z, other.g2_.z) + self.g1_ * vec2(other.g0_.w));
}

MultiVector sphere_antiWedgeDot_dipole(Sphere self, Dipole other) {
    return MultiVector(vec2(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g1_.x) * vec3(other.g2_.x, other.g2_.y, other.g2_.z) - vec3(self.g1_.y) * other.g0_, vec2(self.g0_.x) * vec2(other.g0_.x, -other.g2_.x) + vec2(self.g0_.y) * vec2(other.g0_.y, -other.g2_.y) + vec2(self.g0_.z) * vec2(other.g0_.z, -other.g2_.z) + self.g1_ * vec2(other.g2_.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(0.0, other.g0_.z, -other.g0_.y, -other.g1_.x) + vec4(self.g0_.y) * vec4(-other.g0_.z, 0.0, other.g0_.x, -other.g1_.y) + vec4(self.g0_.z) * vec4(other.g0_.y, -other.g0_.x, 0.0, -other.g1_.z) + vec4(self.g1_.x) * vec4(-other.g1_.x, -other.g1_.y, -other.g1_.z, 0.0), vec3(0.0) - self.g0_ * vec3(other.g2_.w) - vec3(self.g1_.x) * vec3(other.g2_.x, other.g2_.y, other.g2_.z) - vec3(self.g1_.y) * other.g0_, vec3(self.g0_.x) * vec3(0.0, -other.g2_.z, other.g2_.y) + vec3(self.g0_.y) * vec3(other.g2_.z, 0.0, -other.g2_.x) + vec3(self.g0_.z) * vec3(-other.g2_.y, other.g2_.x, 0.0) - vec3(self.g1_.y) * other.g1_, vec3(0.0), vec2(0.0));
}

MultiVector sphere_antiWedgeDot_dualNum(Sphere self, DualNum other) {
    return MultiVector(vec2(0.0), self.g0_ * vec3(other.g0_.x), vec2(0.0) - self.g1_ * vec2(other.g0_.x), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), self.g0_ * vec3(other.g0_.y), self.g1_ * vec2(other.g0_.y));
}

MultiVector sphere_antiWedgeDot_flatPoint(Sphere self, FlatPoint other) {
    return MultiVector(vec2(0.0), vec3(self.g1_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g0_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g0_.z) + self.g1_ * vec2(other.g0_.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0) - self.g0_ * vec3(other.g0_.w) - vec3(self.g1_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(0.0), vec2(0.0));
}

MultiVector sphere_antiWedgeDot_flector(Sphere self, Flector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, other.g1_.z) + vec2(self.g1_.x) * vec2(0.0, -other.g1_.w), vec3(self.g1_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g0_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g0_.z) + self.g1_ * vec2(other.g0_.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g1_.x) * other.g1_, vec3(self.g0_.x) * vec3(-other.g0_.w, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, -other.g0_.w, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, -other.g0_.w) - vec3(self.g1_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(self.g0_.x) * vec3(other.g1_.w, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, other.g1_.w, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, other.g1_.w) - vec3(self.g1_.y) * vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec3(0.0), vec2(0.0));
}

MultiVector sphere_antiWedgeDot_line(Sphere self, Line other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g1_.x) * other.g0_, vec3(self.g1_.x) * other.g1_, vec4(self.g0_.x) * vec4(0.0, other.g1_.z, -other.g1_.y, -other.g0_.x) + vec4(self.g0_.y) * vec4(-other.g1_.z, 0.0, other.g1_.x, -other.g0_.y) + vec4(self.g0_.z) * vec4(other.g1_.y, -other.g1_.x, 0.0, -other.g0_.z) + vec4(self.g1_.y) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) + vec3(self.g1_.x) * other.g1_, vec2(self.g0_.x) * vec2(0.0, other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, other.g1_.z));
}

MultiVector sphere_antiWedgeDot_motor(Sphere self, Motor other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g1_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(self.g1_.x) * other.g1_, vec4(self.g0_.x) * vec4(0.0, other.g1_.z, -other.g1_.y, -other.g0_.x) + vec4(self.g0_.y) * vec4(-other.g1_.z, 0.0, other.g1_.x, -other.g0_.y) + vec4(self.g0_.z) * vec4(other.g1_.y, -other.g1_.x, 0.0, -other.g0_.z) + vec4(self.g1_.y) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x) * vec3(other.g0_.w, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, other.g0_.w, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, other.g0_.w) + vec3(self.g1_.x) * other.g1_, vec2(self.g0_.x) * vec2(0.0, other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, other.g1_.z) + self.g1_ * vec2(other.g0_.w));
}

MultiVector sphere_antiWedgeDot_multiVector(Sphere self, MultiVector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(other.g1_.x, other.g9_.x) + vec2(self.g0_.y) * vec2(other.g1_.y, other.g9_.y) + vec2(self.g0_.z) * vec2(other.g1_.z, other.g9_.z) + vec2(self.g1_.x) * vec2(other.g2_.y, -other.g10_.y) + vec2(self.g1_.y) * vec2(other.g2_.x, -other.g10_.x), vec3(self.g0_.x) * vec3(other.g0_.x, -other.g4_.z, other.g4_.y) + vec3(self.g0_.y) * vec3(other.g4_.z, other.g0_.x, -other.g4_.x) + vec3(self.g0_.z) * vec3(-other.g4_.y, other.g4_.x, other.g0_.x) + vec3(self.g1_.x) * vec3(other.g5_.x, other.g5_.y, other.g5_.z) - vec3(self.g1_.y) * other.g3_, vec2(self.g0_.x) * vec2(other.g3_.x, -other.g5_.x) + vec2(self.g0_.y) * vec2(other.g3_.y, -other.g5_.y) + vec2(self.g0_.z) * vec2(other.g3_.z, -other.g5_.z) + self.g1_ * vec2(other.g5_.w) - self.g1_ * vec2(other.g0_.x), vec3(self.g0_.x) * vec3(other.g2_.x, -other.g6_.z, other.g6_.y) + vec3(self.g0_.y) * vec3(other.g6_.z, other.g2_.x, -other.g6_.x) + vec3(self.g0_.z) * vec3(-other.g6_.y, other.g6_.x, other.g2_.x) + vec3(self.g1_.x) * other.g7_ + vec3(self.g1_.x) * other.g1_, vec3(self.g0_.x) * vec3(-other.g6_.w, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, -other.g6_.w, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, -other.g6_.w) + vec3(self.g1_.x) * other.g8_ + vec3(self.g1_.y) * vec3(other.g6_.x, other.g6_.y, other.g6_.z), vec4(self.g0_.x) * vec4(-other.g2_.y, other.g8_.z, -other.g8_.y, -other.g7_.x) + vec4(self.g0_.y) * vec4(-other.g8_.z, -other.g2_.y, other.g8_.x, -other.g7_.y) + vec4(self.g0_.z) * vec4(other.g8_.y, -other.g8_.x, -other.g2_.y, -other.g7_.z) + vec4(self.g1_.x) * vec4(0.0, 0.0, 0.0, other.g2_.y) + vec4(self.g1_.y) * vec4(other.g7_.x, other.g7_.y, other.g7_.z, -other.g2_.x) + vec4(self.g1_.y) * vec4(-other.g1_.x, -other.g1_.y, -other.g1_.z, 0.0), vec4(self.g0_.x) * vec4(-other.g10_.x, other.g3_.z, -other.g3_.y, -other.g4_.x) + vec4(self.g0_.y) * vec4(-other.g3_.z, -other.g10_.x, other.g3_.x, -other.g4_.y) + vec4(self.g0_.z) * vec4(other.g3_.y, -other.g3_.x, -other.g10_.x, -other.g4_.z) + vec4(self.g1_.x) * vec4(other.g9_.x, other.g9_.y, other.g9_.z, other.g10_.y) + vec4(self.g1_.x) * vec4(-other.g4_.x, -other.g4_.y, -other.g4_.z, 0.0) + vec4(self.g1_.y) * vec4(0.0, 0.0, 0.0, -other.g10_.x), vec3(self.g0_.x) * vec3(-other.g5_.w, other.g9_.z, -other.g9_.y) + vec3(self.g0_.y) * vec3(-other.g9_.z, -other.g5_.w, other.g9_.x) + vec3(self.g0_.z) * vec3(other.g9_.y, -other.g9_.x, -other.g5_.w) - vec3(self.g1_.x) * vec3(other.g5_.x, other.g5_.y, other.g5_.z) - vec3(self.g1_.y) * other.g3_, vec3(self.g0_.x) * vec3(other.g10_.y, -other.g5_.z, other.g5_.y) + vec3(self.g0_.y) * vec3(other.g5_.z, other.g10_.y, -other.g5_.x) + vec3(self.g0_.z) * vec3(-other.g5_.y, other.g5_.x, other.g10_.y) - vec3(self.g1_.y) * other.g9_ - vec3(self.g1_.y) * other.g4_, vec3(self.g0_.x) * vec3(other.g0_.y, -other.g7_.z, other.g7_.y) + vec3(self.g0_.y) * vec3(other.g7_.z, other.g0_.y, -other.g7_.x) + vec3(self.g0_.z) * vec3(-other.g7_.y, other.g7_.x, other.g0_.y) + vec3(self.g1_.x) * other.g8_ - vec3(self.g1_.y) * vec3(other.g6_.x, other.g6_.y, other.g6_.z), vec2(self.g0_.x) * vec2(-other.g6_.x, other.g8_.x) + vec2(self.g0_.y) * vec2(-other.g6_.y, other.g8_.y) + vec2(self.g0_.z) * vec2(-other.g6_.z, other.g8_.z) + self.g1_ * vec2(other.g6_.w) + self.g1_ * vec2(other.g0_.y));
}

MultiVector sphere_antiWedgeDot_plane(Sphere self, Plane other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, other.g0_.z) + vec2(self.g1_.x) * vec2(0.0, -other.g0_.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g1_.x) * other.g0_, vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), self.g0_ * vec3(other.g0_.w) - vec3(self.g1_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(0.0), vec2(0.0));
}

MultiVector sphere_antiWedgeDot_roundPoint(Sphere self, RoundPoint other) {
    return MultiVector(vec2(self.g0_.x) * vec2(other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(other.g0_.z, 0.0) + vec2(self.g1_.x) * vec2(other.g1_.y, 0.0) + vec2(self.g1_.y) * vec2(other.g1_.x, 0.0), vec3(0.0), vec2(0.0), self.g0_ * vec3(other.g1_.x) + vec3(self.g1_.x) * other.g0_, vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec4(self.g0_.x, self.g0_.y, self.g0_.z, self.g0_.x) * vec4(-other.g1_.y, -other.g1_.y, -other.g1_.y, 0.0) + vec4(self.g1_.x) * vec4(0.0, 0.0, 0.0, other.g1_.y) - vec4(self.g1_.y) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.x), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

RoundPoint sphere_antiWedgeDot_scalar(Sphere self, Scalar other) {
    return RoundPoint(self.g0_ * vec3(other.g0_), vec2(0.0) - self.g1_ * vec2(other.g0_));
}

MultiVector sphere_antiWedgeDot_sphere(Sphere self, Sphere other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, other.g0_.z) + vec2(self.g1_.x) * vec2(0.0, -other.g1_.y) + vec2(self.g1_.y) * vec2(0.0, -other.g1_.x), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x, self.g0_.y, self.g0_.z, self.g0_.x) * vec4(-other.g1_.x, -other.g1_.x, -other.g1_.x, 0.0) + vec4(self.g1_.x) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.y) + vec4(self.g1_.y) * vec4(0.0, 0.0, 0.0, -other.g1_.x), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), self.g0_ * vec3(other.g1_.y) - vec3(self.g1_.y) * other.g0_, vec3(0.0), vec2(0.0));
}

Scalar antiScalar_wedgeDot_antiScalar(AntiScalar self, AntiScalar other) {
    return Scalar(0.0 - self.g0_ * other.g0_);
}

Dipole antiScalar_wedgeDot_circle(AntiScalar self, Circle other) {
    return Dipole(vec3(0.0) - vec3(self.g0_) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(0.0) - vec3(self.g0_) * other.g1_, vec4(self.g0_) * vec4(-other.g2_.x, -other.g2_.y, -other.g2_.z, other.g0_.w));
}

Circle antiScalar_wedgeDot_dipole(AntiScalar self, Dipole other) {
    return Circle(vec4(self.g0_) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, -other.g2_.w), vec3(self.g0_) * other.g1_, vec3(self.g0_) * vec3(other.g2_.x, other.g2_.y, other.g2_.z));
}

DualNum antiScalar_wedgeDot_dualNum(AntiScalar self, DualNum other) {
    return DualNum(vec2(self.g0_) * other.g0_.yx * vec2(-1.0, 1.0));
}

Circle antiScalar_wedgeDot_flatPoint(AntiScalar self, FlatPoint other) {
    return Circle(vec4(self.g0_) * vec4(0.0, 0.0, 0.0, -other.g0_.w), vec3(0.0), vec3(self.g0_) * vec3(other.g0_.x, other.g0_.y, other.g0_.z));
}

MultiVector antiScalar_wedgeDot_flector(AntiScalar self, Flector other) {
    return MultiVector(vec2(0.0), vec3(0.0) - vec3(self.g0_) * vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec2(self.g0_) * vec2(0.0, other.g1_.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_) * vec4(0.0, 0.0, 0.0, -other.g0_.w), vec3(0.0), vec3(self.g0_) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(0.0), vec2(0.0));
}

Dipole antiScalar_wedgeDot_line(AntiScalar self, Line other) {
    return Dipole(vec3(0.0), vec3(0.0) - vec3(self.g0_) * other.g0_, vec4(self.g0_) * vec4(-other.g1_.x, -other.g1_.y, -other.g1_.z, 0.0));
}

MultiVector antiScalar_wedgeDot_motor(AntiScalar self, Motor other) {
    return MultiVector(vec2(self.g0_) * vec2(-other.g0_.w, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0) - vec3(self.g0_) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec4(self.g0_) * vec4(-other.g1_.x, -other.g1_.y, -other.g1_.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector antiScalar_wedgeDot_multiVector(AntiScalar self, MultiVector other) {
    return MultiVector(vec2(self.g0_) * other.g0_.yx * vec2(-1.0, 1.0), vec3(0.0) - vec3(self.g0_) * other.g9_, vec2(self.g0_) * other.g10_, vec3(0.0) - vec3(self.g0_) * vec3(other.g6_.x, other.g6_.y, other.g6_.z), vec3(0.0) - vec3(self.g0_) * other.g7_, vec4(self.g0_) * vec4(-other.g8_.x, -other.g8_.y, -other.g8_.z, other.g6_.w), vec4(self.g0_) * vec4(other.g3_.x, other.g3_.y, other.g3_.z, -other.g5_.w), vec3(self.g0_) * other.g4_, vec3(self.g0_) * vec3(other.g5_.x, other.g5_.y, other.g5_.z), vec3(self.g0_) * other.g1_, vec2(0.0) - vec2(self.g0_) * other.g2_);
}

RoundPoint antiScalar_wedgeDot_plane(AntiScalar self, Plane other) {
    return RoundPoint(vec3(0.0) - vec3(self.g0_) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g0_) * vec2(0.0, other.g0_.w));
}

Sphere antiScalar_wedgeDot_roundPoint(AntiScalar self, RoundPoint other) {
    return Sphere(vec3(self.g0_) * other.g0_, vec2(0.0) - vec2(self.g0_) * other.g1_);
}

AntiScalar antiScalar_wedgeDot_scalar(AntiScalar self, Scalar other) {
    return AntiScalar(self.g0_ * other.g0_);
}

RoundPoint antiScalar_wedgeDot_sphere(AntiScalar self, Sphere other) {
    return RoundPoint(vec3(0.0) - vec3(self.g0_) * other.g0_, vec2(self.g0_) * other.g1_);
}

Dipole circle_wedgeDot_antiScalar(Circle self, AntiScalar other) {
    return Dipole(vec3(0.0) - vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_), vec3(0.0) - self.g1_ * vec3(other.g0_), vec4(self.g0_.w) * vec4(0.0, 0.0, 0.0, other.g0_) + vec4(self.g2_.x, self.g2_.y, self.g2_.z, self.g2_.x) * vec4(-other.g0_, -other.g0_, -other.g0_, 0.0));
}

MultiVector circle_wedgeDot_circle(Circle self, Circle other) {
    return MultiVector(vec2(self.g0_.x) * vec2(other.g2_.x, 0.0) + vec2(self.g0_.y) * vec2(other.g2_.y, 0.0) + vec2(self.g0_.z) * vec2(other.g2_.z, 0.0) + vec2(self.g0_.w) * vec2(-other.g0_.w, 0.0) + vec2(self.g1_.x) * vec2(other.g1_.x, 0.0) + vec2(self.g1_.y) * vec2(other.g1_.y, 0.0) + vec2(self.g1_.z) * vec2(other.g1_.z, 0.0) + vec2(self.g2_.x) * vec2(other.g0_.x, 0.0) + vec2(self.g2_.y) * vec2(other.g0_.y, 0.0) + vec2(self.g2_.z) * vec2(other.g0_.z, 0.0), vec3(0.0), vec2(0.0), vec3(self.g0_.x) * vec3(other.g0_.w, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, other.g0_.w, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, other.g0_.w) - vec3(self.g0_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(self.g0_.x) * vec3(0.0, -other.g2_.z, other.g2_.y) + vec3(self.g0_.y) * vec3(other.g2_.z, 0.0, -other.g2_.x) + vec3(self.g0_.z) * vec3(-other.g2_.y, other.g2_.x, 0.0) + vec3(self.g1_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g1_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g1_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g2_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g2_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g2_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, -other.g2_.x) + vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, -other.g2_.y) + self.g0_.wwwz * vec4(other.g2_.x, other.g2_.y, other.g2_.z, -other.g2_.z) + vec4(self.g1_.x) * vec4(0.0, -other.g2_.z, other.g2_.y, 0.0) + vec4(self.g1_.y) * vec4(other.g2_.z, 0.0, -other.g2_.x, 0.0) + vec4(self.g1_.z) * vec4(-other.g2_.y, other.g2_.x, 0.0, 0.0) + vec4(self.g2_.x) * vec4(-other.g0_.w, -other.g1_.z, other.g1_.y, other.g0_.x) + vec4(self.g2_.y) * vec4(other.g1_.z, -other.g0_.w, -other.g1_.x, other.g0_.y) + vec4(self.g2_.z) * vec4(-other.g1_.y, other.g1_.x, -other.g0_.w, other.g0_.z), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x) * vec3(0.0, other.g2_.z, -other.g2_.y) + vec3(self.g0_.y) * vec3(-other.g2_.z, 0.0, other.g2_.x) + vec3(self.g0_.z) * vec3(other.g2_.y, -other.g2_.x, 0.0) + vec3(self.g0_.w) * other.g1_ + self.g1_ * vec3(other.g0_.w) + vec3(self.g2_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g2_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g2_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec2(self.g0_.x) * vec2(other.g1_.x, 0.0) + vec2(self.g0_.y) * vec2(other.g1_.y, 0.0) + vec2(self.g0_.z) * vec2(other.g1_.z, 0.0) + vec2(self.g1_.x) * vec2(other.g0_.x, other.g2_.x) + vec2(self.g1_.y) * vec2(other.g0_.y, other.g2_.y) + vec2(self.g1_.z) * vec2(other.g0_.z, other.g2_.z) + vec2(self.g2_.x) * vec2(0.0, other.g1_.x) + vec2(self.g2_.y) * vec2(0.0, other.g1_.y) + vec2(self.g2_.z) * vec2(0.0, other.g1_.z));
}

MultiVector circle_wedgeDot_dipole(Circle self, Dipole other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, -other.g2_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g2_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g2_.z) + vec2(self.g0_.w) * vec2(0.0, -other.g2_.w) + vec2(self.g1_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g2_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g2_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g2_.z) * vec2(0.0, -other.g0_.z), vec3(self.g0_.x) * vec3(0.0, other.g2_.z, -other.g2_.y) + vec3(self.g0_.y) * vec3(-other.g2_.z, 0.0, other.g2_.x) + vec3(self.g0_.z) * vec3(other.g2_.y, -other.g2_.x, 0.0) + vec3(self.g0_.w) * other.g1_ - self.g1_ * vec3(other.g2_.w) + vec3(self.g2_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g2_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g2_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec2(self.g0_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g1_.z, 0.0) - vec2(self.g1_.x) * vec2(other.g0_.x, other.g2_.x) - vec2(self.g1_.y) * vec2(other.g0_.y, other.g2_.y) - vec2(self.g1_.z) * vec2(other.g0_.z, other.g2_.z) + vec2(self.g2_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g2_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g2_.z) * vec2(0.0, -other.g1_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(other.g2_.w, other.g1_.z, -other.g1_.y, -other.g2_.x) + vec4(self.g0_.y) * vec4(-other.g1_.z, other.g2_.w, other.g1_.x, -other.g2_.y) + vec4(self.g0_.z) * vec4(other.g1_.y, -other.g1_.x, other.g2_.w, -other.g2_.z) + vec4(self.g0_.w) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, 0.0) + vec4(self.g1_.x) * vec4(0.0, other.g0_.z, -other.g0_.y, 0.0) + vec4(self.g1_.y) * vec4(-other.g0_.z, 0.0, other.g0_.x, 0.0) + vec4(self.g1_.z) * vec4(other.g0_.y, -other.g0_.x, 0.0, 0.0) + vec4(self.g2_.x) * vec4(0.0, 0.0, 0.0, other.g0_.x) + vec4(self.g2_.y) * vec4(0.0, 0.0, 0.0, other.g0_.y) + vec4(self.g2_.z) * vec4(0.0, 0.0, 0.0, other.g0_.z), vec3(self.g0_.x) * vec3(0.0, other.g2_.z, -other.g2_.y) + vec3(self.g0_.y) * vec3(-other.g2_.z, 0.0, other.g2_.x) + vec3(self.g0_.z) * vec3(other.g2_.y, -other.g2_.x, 0.0) + vec3(self.g1_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g1_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g1_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g2_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g2_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g2_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec3(0.0) - vec3(self.g0_.w) * vec3(other.g2_.x, other.g2_.y, other.g2_.z) + vec3(self.g1_.x) * vec3(0.0, other.g2_.z, -other.g2_.y) + vec3(self.g1_.y) * vec3(-other.g2_.z, 0.0, other.g2_.x) + vec3(self.g1_.z) * vec3(other.g2_.y, -other.g2_.x, 0.0) + vec3(self.g2_.x) * vec3(-other.g2_.w, other.g1_.z, -other.g1_.y) + vec3(self.g2_.y) * vec3(-other.g1_.z, -other.g2_.w, other.g1_.x) + vec3(self.g2_.z) * vec3(other.g1_.y, -other.g1_.x, -other.g2_.w), vec3(0.0), vec2(0.0));
}

MultiVector circle_wedgeDot_dualNum(Circle self, DualNum other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0) - vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_.y), vec3(0.0) - self.g1_ * vec3(other.g0_.y), vec4(self.g0_.w) * vec4(0.0, 0.0, 0.0, other.g0_.y) + vec4(self.g2_.x, self.g2_.y, self.g2_.z, self.g2_.x) * vec4(-other.g0_.y, -other.g0_.y, -other.g0_.y, 0.0), self.g0_ * vec4(other.g0_.x), self.g1_ * vec3(other.g0_.x), self.g2_ * vec3(other.g0_.x), vec3(0.0), vec2(0.0));
}

MultiVector circle_wedgeDot_flatPoint(Circle self, FlatPoint other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g0_.w) * vec2(0.0, -other.g0_.w), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) - self.g1_ * vec3(other.g0_.w), vec2(self.g1_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g0_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(other.g0_.w, 0.0, 0.0, -other.g0_.x) + vec4(self.g0_.y) * vec4(0.0, other.g0_.w, 0.0, -other.g0_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, other.g0_.w, -other.g0_.z), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec3(0.0) - vec3(self.g0_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g1_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g1_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g1_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) - self.g2_ * vec3(other.g0_.w), vec3(0.0), vec2(0.0));
}

MultiVector circle_wedgeDot_flector(Circle self, Flector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g0_.w) * vec2(0.0, -other.g0_.w), vec3(self.g0_.x) * vec3(other.g1_.w, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, other.g1_.w, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, other.g1_.w) + vec3(self.g1_.x) * vec3(-other.g0_.w, -other.g1_.z, other.g1_.y) + vec3(self.g1_.y) * vec3(other.g1_.z, -other.g0_.w, -other.g1_.x) + vec3(self.g1_.z) * vec3(-other.g1_.y, other.g1_.x, -other.g0_.w), vec2(self.g0_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g1_.y, 0.0) - vec2(self.g0_.z, self.g0_.w) * vec2(other.g1_.z, other.g1_.w) + vec2(self.g1_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g2_.x) * vec2(0.0, other.g1_.x) + vec2(self.g2_.y) * vec2(0.0, other.g1_.y) + vec2(self.g2_.z) * vec2(0.0, other.g1_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(other.g0_.w, other.g1_.z, -other.g1_.y, -other.g0_.x) + vec4(self.g0_.y) * vec4(-other.g1_.z, other.g0_.w, other.g1_.x, -other.g0_.y) + vec4(self.g0_.z) * vec4(other.g1_.y, -other.g1_.x, other.g0_.w, -other.g0_.z) + vec4(self.g1_.x) * vec4(0.0, 0.0, 0.0, other.g1_.x) + vec4(self.g1_.y) * vec4(0.0, 0.0, 0.0, other.g1_.y) + vec4(self.g1_.z) * vec4(0.0, 0.0, 0.0, other.g1_.z), vec3(self.g0_.x) * vec3(other.g1_.w, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, other.g1_.w, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, other.g1_.w) - vec3(self.g0_.w) * vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec3(0.0) - vec3(self.g0_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g1_.x) * vec3(other.g1_.w, other.g0_.z, -other.g0_.y) + vec3(self.g1_.y) * vec3(-other.g0_.z, other.g1_.w, other.g0_.x) + vec3(self.g1_.z) * vec3(other.g0_.y, -other.g0_.x, other.g1_.w) + vec3(self.g2_.x) * vec3(-other.g0_.w, -other.g1_.z, other.g1_.y) + vec3(self.g2_.y) * vec3(other.g1_.z, -other.g0_.w, -other.g1_.x) + vec3(self.g2_.z) * vec3(-other.g1_.y, other.g1_.x, -other.g0_.w), vec3(0.0), vec2(0.0));
}

MultiVector circle_wedgeDot_line(Circle self, Line other) {
    return MultiVector(vec2(self.g0_.x) * vec2(other.g1_.x, 0.0) + vec2(self.g0_.y) * vec2(other.g1_.y, 0.0) + vec2(self.g0_.z) * vec2(other.g1_.z, 0.0) + vec2(self.g1_.x) * vec2(other.g0_.x, 0.0) + vec2(self.g1_.y) * vec2(other.g0_.y, 0.0) + vec2(self.g1_.z) * vec2(other.g0_.z, 0.0), vec3(0.0), vec2(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(self.g0_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, -other.g1_.x) + vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, -other.g1_.y) + self.g0_.wwwz * vec4(other.g1_.x, other.g1_.y, other.g1_.z, -other.g1_.z) + vec4(self.g1_.x) * vec4(0.0, -other.g1_.z, other.g1_.y, 0.0) + vec4(self.g1_.y) * vec4(other.g1_.z, 0.0, -other.g1_.x, 0.0) + vec4(self.g1_.z) * vec4(-other.g1_.y, other.g1_.x, 0.0, 0.0) + vec4(self.g2_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g2_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g2_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g0_.w) * other.g0_, vec2(self.g0_.x) * vec2(other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(other.g0_.z, 0.0) + vec2(self.g1_.x) * vec2(0.0, other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, other.g1_.z) + vec2(self.g2_.x) * vec2(0.0, other.g0_.x) + vec2(self.g2_.y) * vec2(0.0, other.g0_.y) + vec2(self.g2_.z) * vec2(0.0, other.g0_.z));
}

MultiVector circle_wedgeDot_motor(Circle self, Motor other) {
    return MultiVector(vec2(self.g0_.x) * vec2(other.g1_.x, 0.0) + vec2(self.g0_.y) * vec2(other.g1_.y, 0.0) + vec2(self.g0_.z) * vec2(other.g1_.z, 0.0) + vec2(self.g1_.x) * vec2(other.g0_.x, 0.0) + vec2(self.g1_.y) * vec2(other.g0_.y, 0.0) + vec2(self.g1_.z) * vec2(other.g0_.z, 0.0), vec3(0.0), vec2(0.0), vec3(self.g0_.x) * vec3(-other.g0_.w, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, -other.g0_.w, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, -other.g0_.w), vec3(self.g0_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g1_.x) * vec3(-other.g0_.w, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, -other.g0_.w, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, -other.g0_.w), vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, -other.g1_.x) + vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, -other.g1_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, 0.0, -other.g1_.z) + vec4(self.g0_.w) * vec4(other.g1_.x, other.g1_.y, other.g1_.z, other.g0_.w) + vec4(self.g1_.x) * vec4(0.0, -other.g1_.z, other.g1_.y, 0.0) + vec4(self.g1_.y) * vec4(other.g1_.z, 0.0, -other.g1_.x, 0.0) + vec4(self.g1_.z) * vec4(-other.g1_.y, other.g1_.x, 0.0, 0.0) + vec4(self.g2_.x) * vec4(-other.g0_.w, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g2_.y) * vec4(other.g0_.z, -other.g0_.w, -other.g0_.x, 0.0) + vec4(self.g2_.z) * vec4(-other.g0_.y, other.g0_.x, -other.g0_.w, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g0_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g0_.x) * vec2(other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(other.g0_.z, 0.0) + vec2(self.g1_.x) * vec2(0.0, other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, other.g1_.z) + vec2(self.g2_.x) * vec2(0.0, other.g0_.x) + vec2(self.g2_.y) * vec2(0.0, other.g0_.y) + vec2(self.g2_.z) * vec2(0.0, other.g0_.z));
}

MultiVector circle_wedgeDot_multiVector(Circle self, MultiVector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(other.g8_.x, -other.g5_.x) + vec2(self.g0_.y) * vec2(other.g8_.y, -other.g5_.y) + vec2(self.g0_.z) * vec2(other.g8_.z, -other.g5_.z) - vec2(self.g0_.w) * vec2(other.g6_.w, other.g5_.w) + vec2(self.g1_.x) * vec2(other.g7_.x, -other.g4_.x) + vec2(self.g1_.y) * vec2(other.g7_.y, -other.g4_.y) + vec2(self.g1_.z) * vec2(other.g7_.z, -other.g4_.z) + vec2(self.g2_.x) * vec2(other.g6_.x, -other.g3_.x) + vec2(self.g2_.y) * vec2(other.g6_.y, -other.g3_.y) + vec2(self.g2_.z) * vec2(other.g6_.z, -other.g3_.z), vec3(self.g0_.x) * vec3(other.g10_.y, other.g5_.z, -other.g5_.y) + vec3(self.g0_.y) * vec3(-other.g5_.z, other.g10_.y, other.g5_.x) + vec3(self.g0_.z) * vec3(other.g5_.y, -other.g5_.x, other.g10_.y) + vec3(self.g0_.w) * other.g4_ + vec3(self.g1_.x) * vec3(-other.g5_.w, -other.g9_.z, other.g9_.y) + vec3(self.g1_.y) * vec3(other.g9_.z, -other.g5_.w, -other.g9_.x) + vec3(self.g1_.z) * vec3(-other.g9_.y, other.g9_.x, -other.g5_.w) + vec3(self.g2_.x) * vec3(-other.g10_.x, -other.g3_.z, other.g3_.y) + vec3(self.g2_.y) * vec3(other.g3_.z, -other.g10_.x, -other.g3_.x) + vec3(self.g2_.z) * vec3(-other.g3_.y, other.g3_.x, -other.g10_.x), vec2(self.g0_.x) * vec2(-other.g9_.x, 0.0) + vec2(self.g0_.x) * vec2(-other.g4_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g9_.y, 0.0) + vec2(self.g0_.y) * vec2(-other.g4_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g9_.z, 0.0) + vec2(self.g0_.z) * vec2(-other.g4_.z, 0.0) + vec2(self.g0_.w) * other.g10_ * vec2(1.0, -1.0) - vec2(self.g1_.x) * vec2(other.g3_.x, other.g5_.x) - vec2(self.g1_.y) * vec2(other.g3_.y, other.g5_.y) - vec2(self.g1_.z) * vec2(other.g3_.z, other.g5_.z) + vec2(self.g2_.x) * vec2(0.0, other.g9_.x) + vec2(self.g2_.x) * vec2(0.0, -other.g4_.x) + vec2(self.g2_.y) * vec2(0.0, other.g9_.y) + vec2(self.g2_.y) * vec2(0.0, -other.g4_.y) + vec2(self.g2_.z) * vec2(0.0, other.g9_.z) + vec2(self.g2_.z) * vec2(0.0, -other.g4_.z), vec3(self.g0_.x) * vec3(other.g6_.w, -other.g7_.z, other.g7_.y) + vec3(self.g0_.x) * vec3(-other.g0_.y, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(other.g7_.z, other.g6_.w, -other.g7_.x) + vec3(self.g0_.y) * vec3(-other.g1_.z, -other.g0_.y, other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g7_.y, other.g7_.x, other.g6_.w) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, -other.g0_.y) - vec3(self.g0_.w) * vec3(other.g6_.x, other.g6_.y, other.g6_.z) + vec3(self.g1_.x) * vec3(-other.g2_.x, -other.g6_.z, other.g6_.y) + vec3(self.g1_.y) * vec3(other.g6_.z, -other.g2_.x, -other.g6_.x) + vec3(self.g1_.z) * vec3(-other.g6_.y, other.g6_.x, -other.g2_.x), vec3(self.g0_.x) * vec3(-other.g2_.y, -other.g8_.z, other.g8_.y) + vec3(self.g0_.y) * vec3(other.g8_.z, -other.g2_.y, -other.g8_.x) + vec3(self.g0_.z) * vec3(-other.g8_.y, other.g8_.x, -other.g2_.y) - vec3(self.g0_.w) * other.g1_ + vec3(self.g1_.x) * vec3(-other.g0_.y, -other.g7_.z, other.g7_.y) + vec3(self.g1_.y) * vec3(other.g7_.z, -other.g0_.y, -other.g7_.x) + vec3(self.g1_.z) * vec3(-other.g7_.y, other.g7_.x, -other.g0_.y) + vec3(self.g2_.x) * vec3(-other.g2_.x, -other.g6_.z, other.g6_.y) + vec3(self.g2_.y) * vec3(other.g6_.z, -other.g2_.x, -other.g6_.x) + vec3(self.g2_.z) * vec3(-other.g6_.y, other.g6_.x, -other.g2_.x), vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, -other.g8_.x) + vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, -other.g8_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, 0.0, -other.g8_.z) + vec4(self.g0_.w) * vec4(other.g8_.x, other.g8_.y, other.g8_.z, other.g0_.y) + vec4(self.g1_.x) * vec4(-other.g2_.y, -other.g8_.z, other.g8_.y, -other.g1_.x) + vec4(self.g1_.y) * vec4(other.g8_.z, -other.g2_.y, -other.g8_.x, -other.g1_.y) + vec4(self.g1_.z) * vec4(-other.g8_.y, other.g8_.x, -other.g2_.y, -other.g1_.z) + vec4(self.g2_.x) * vec4(-other.g6_.w, -other.g7_.z, other.g7_.y, other.g6_.x) + vec4(self.g2_.x) * vec4(-other.g0_.y, -other.g1_.z, other.g1_.y, 0.0) + vec4(self.g2_.y) * vec4(other.g7_.z, -other.g6_.w, -other.g7_.x, other.g6_.y) + vec4(self.g2_.y) * vec4(other.g1_.z, -other.g0_.y, -other.g1_.x, 0.0) + vec4(self.g2_.z) * vec4(-other.g7_.y, other.g7_.x, -other.g6_.w, other.g6_.z) + vec4(self.g2_.z) * vec4(-other.g1_.y, other.g1_.x, -other.g0_.y, 0.0), vec4(self.g0_.x) * vec4(other.g5_.w, other.g9_.z, -other.g9_.y, -other.g5_.x) + vec4(self.g0_.x) * vec4(other.g0_.x, other.g4_.z, -other.g4_.y, 0.0) + vec4(self.g0_.y) * vec4(-other.g9_.z, other.g5_.w, other.g9_.x, -other.g5_.y) + vec4(self.g0_.y) * vec4(-other.g4_.z, other.g0_.x, other.g4_.x, 0.0) + vec4(self.g0_.z) * vec4(other.g9_.y, -other.g9_.x, other.g5_.w, -other.g5_.z) + vec4(self.g0_.z) * vec4(other.g4_.y, -other.g4_.x, other.g0_.x, 0.0) + vec4(self.g0_.w) * vec4(other.g3_.x, other.g3_.y, other.g3_.z, other.g0_.x) + vec4(self.g1_.x) * vec4(other.g10_.x, other.g3_.z, -other.g3_.y, other.g9_.x) + vec4(self.g1_.y) * vec4(-other.g3_.z, other.g10_.x, other.g3_.x, other.g9_.y) + vec4(self.g1_.z) * vec4(other.g3_.y, -other.g3_.x, other.g10_.x, other.g9_.z) + vec4(self.g2_.x) * vec4(0.0, 0.0, 0.0, other.g3_.x) + vec4(self.g2_.y) * vec4(0.0, 0.0, 0.0, other.g3_.y) + vec4(self.g2_.z) * vec4(0.0, 0.0, 0.0, other.g3_.z), vec3(self.g0_.x) * vec3(other.g10_.y, other.g5_.z, -other.g5_.y) + vec3(self.g0_.y) * vec3(-other.g5_.z, other.g10_.y, other.g5_.x) + vec3(self.g0_.z) * vec3(other.g5_.y, -other.g5_.x, other.g10_.y) - vec3(self.g0_.w) * other.g9_ + vec3(self.g1_.x) * vec3(other.g0_.x, other.g4_.z, -other.g4_.y) + vec3(self.g1_.y) * vec3(-other.g4_.z, other.g0_.x, other.g4_.x) + vec3(self.g1_.z) * vec3(other.g4_.y, -other.g4_.x, other.g0_.x) + vec3(self.g2_.x) * vec3(other.g10_.x, other.g3_.z, -other.g3_.y) + vec3(self.g2_.y) * vec3(-other.g3_.z, other.g10_.x, other.g3_.x) + vec3(self.g2_.z) * vec3(other.g3_.y, -other.g3_.x, other.g10_.x), vec3(0.0) - vec3(self.g0_.w) * vec3(other.g5_.x, other.g5_.y, other.g5_.z) + vec3(self.g1_.x) * vec3(other.g10_.y, other.g5_.z, -other.g5_.y) + vec3(self.g1_.y) * vec3(-other.g5_.z, other.g10_.y, other.g5_.x) + vec3(self.g1_.z) * vec3(other.g5_.y, -other.g5_.x, other.g10_.y) + vec3(self.g2_.x) * vec3(-other.g5_.w, -other.g9_.z, other.g9_.y) + vec3(self.g2_.x) * vec3(other.g0_.x, other.g4_.z, -other.g4_.y) + vec3(self.g2_.y) * vec3(other.g9_.z, -other.g5_.w, -other.g9_.x) + vec3(self.g2_.y) * vec3(-other.g4_.z, other.g0_.x, other.g4_.x) + vec3(self.g2_.z) * vec3(-other.g9_.y, other.g9_.x, -other.g5_.w) + vec3(self.g2_.z) * vec3(other.g4_.y, -other.g4_.x, other.g0_.x), vec3(self.g0_.x) * vec3(other.g2_.y, other.g8_.z, -other.g8_.y) + vec3(self.g0_.y) * vec3(-other.g8_.z, other.g2_.y, other.g8_.x) + vec3(self.g0_.z) * vec3(other.g8_.y, -other.g8_.x, other.g2_.y) + vec3(self.g0_.w) * other.g7_ + vec3(self.g1_.x) * vec3(other.g6_.w, other.g1_.z, -other.g1_.y) + vec3(self.g1_.y) * vec3(-other.g1_.z, other.g6_.w, other.g1_.x) + vec3(self.g1_.z) * vec3(other.g1_.y, -other.g1_.x, other.g6_.w) + vec3(self.g2_.x) * vec3(-other.g2_.x, -other.g6_.z, other.g6_.y) + vec3(self.g2_.y) * vec3(other.g6_.z, -other.g2_.x, -other.g6_.x) + vec3(self.g2_.z) * vec3(-other.g6_.y, other.g6_.x, -other.g2_.x), vec2(self.g0_.x) * vec2(other.g7_.x, 0.0) + vec2(self.g0_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g0_.y) * vec2(other.g7_.y, 0.0) + vec2(self.g0_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g0_.z) * vec2(other.g7_.z, 0.0) + vec2(self.g0_.z) * vec2(-other.g1_.z, 0.0) + vec2(self.g0_.w) * other.g2_ * vec2(-1.0, 1.0) + vec2(self.g1_.x) * vec2(other.g6_.x, other.g8_.x) + vec2(self.g1_.y) * vec2(other.g6_.y, other.g8_.y) + vec2(self.g1_.z) * vec2(other.g6_.z, other.g8_.z) + vec2(self.g2_.x) * vec2(0.0, other.g7_.x) + vec2(self.g2_.x) * vec2(0.0, other.g1_.x) + vec2(self.g2_.y) * vec2(0.0, other.g7_.y) + vec2(self.g2_.y) * vec2(0.0, other.g1_.y) + vec2(self.g2_.z) * vec2(0.0, other.g7_.z) + vec2(self.g2_.z) * vec2(0.0, other.g1_.z));
}

MultiVector circle_wedgeDot_plane(Circle self, Plane other) {
    return MultiVector(vec2(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_.w) + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec2(self.g0_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g0_.y, 0.0) - vec2(self.g0_.z, self.g0_.w) * vec2(other.g0_.z, other.g0_.w) + vec2(self.g2_.x) * vec2(0.0, other.g0_.x) + vec2(self.g2_.y) * vec2(0.0, other.g0_.y) + vec2(self.g2_.z) * vec2(0.0, other.g0_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(0.0, other.g0_.z, -other.g0_.y, 0.0) + vec4(self.g0_.y) * vec4(-other.g0_.z, 0.0, other.g0_.x, 0.0) + vec4(self.g0_.z) * vec4(other.g0_.y, -other.g0_.x, 0.0, 0.0) + vec4(self.g1_.x) * vec4(0.0, 0.0, 0.0, other.g0_.x) + vec4(self.g1_.y) * vec4(0.0, 0.0, 0.0, other.g0_.y) + vec4(self.g1_.z) * vec4(0.0, 0.0, 0.0, other.g0_.z), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_.w) - vec3(self.g0_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), self.g1_ * vec3(other.g0_.w) + vec3(self.g2_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g2_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g2_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(0.0), vec2(0.0));
}

MultiVector circle_wedgeDot_roundPoint(Circle self, RoundPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) - self.g1_ * vec3(other.g1_.x), vec3(0.0) - vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g1_.y) - vec3(self.g0_.w) * other.g0_ - self.g2_ * vec3(other.g1_.x), vec4(self.g1_.x) * vec4(-other.g1_.y, 0.0, 0.0, -other.g0_.x) + vec4(self.g1_.y) * vec4(0.0, -other.g1_.y, 0.0, -other.g0_.y) + vec4(self.g1_.z) * vec4(0.0, 0.0, -other.g1_.y, -other.g0_.z) + vec4(self.g2_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g2_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g2_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g1_.y) + vec3(self.g1_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g1_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g1_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) - self.g2_ * vec3(other.g1_.x), vec2(self.g0_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g0_.w) * other.g1_ * vec2(-1.0, 1.0) + vec2(self.g2_.x) * vec2(0.0, other.g0_.x) + vec2(self.g2_.y) * vec2(0.0, other.g0_.y) + vec2(self.g2_.z) * vec2(0.0, other.g0_.z));
}

Circle circle_wedgeDot_scalar(Circle self, Scalar other) {
    return Circle(self.g0_ * vec4(other.g0_), self.g1_ * vec3(other.g0_), self.g2_ * vec3(other.g0_));
}

MultiVector circle_wedgeDot_sphere(Circle self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g1_.y) + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) - self.g2_ * vec3(other.g1_.x), vec2(self.g0_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g0_.w) * other.g1_ * vec2(1.0, -1.0) + vec2(self.g2_.x) * vec2(0.0, other.g0_.x) + vec2(self.g2_.y) * vec2(0.0, other.g0_.y) + vec2(self.g2_.z) * vec2(0.0, other.g0_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(0.0, other.g0_.z, -other.g0_.y, 0.0) + vec4(self.g0_.y) * vec4(-other.g0_.z, 0.0, other.g0_.x, 0.0) + vec4(self.g0_.z) * vec4(other.g0_.y, -other.g0_.x, 0.0, 0.0) + vec4(self.g1_.x) * vec4(other.g1_.x, 0.0, 0.0, other.g0_.x) + vec4(self.g1_.y) * vec4(0.0, other.g1_.x, 0.0, other.g0_.y) + vec4(self.g1_.z) * vec4(0.0, 0.0, other.g1_.x, other.g0_.z), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g1_.y) - vec3(self.g0_.w) * other.g0_ + self.g2_ * vec3(other.g1_.x), self.g1_ * vec3(other.g1_.y) + vec3(self.g2_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g2_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g2_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(0.0), vec2(0.0));
}

Circle dipole_wedgeDot_antiScalar(Dipole self, AntiScalar other) {
    return Circle(vec4(self.g0_.x, self.g0_.y, self.g0_.z, self.g0_.x) * vec4(other.g0_, other.g0_, other.g0_, 0.0) + vec4(self.g2_.w) * vec4(0.0, 0.0, 0.0, -other.g0_), self.g1_ * vec3(other.g0_), vec3(self.g2_.x, self.g2_.y, self.g2_.z) * vec3(other.g0_));
}

MultiVector dipole_wedgeDot_circle(Dipole self, Circle other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, -other.g2_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g2_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g2_.z) + vec2(self.g1_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g2_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g2_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g2_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g2_.w) * vec2(0.0, -other.g0_.w), vec3(self.g0_.x) * vec3(0.0, other.g2_.z, -other.g2_.y) + vec3(self.g0_.y) * vec3(-other.g2_.z, 0.0, other.g2_.x) + vec3(self.g0_.z) * vec3(other.g2_.y, -other.g2_.x, 0.0) + self.g1_ * vec3(other.g0_.w) + vec3(self.g2_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g2_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g2_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) - vec3(self.g2_.w) * other.g1_, vec2(self.g0_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g1_.z, 0.0) - vec2(self.g1_.x) * vec2(other.g0_.x, other.g2_.x) - vec2(self.g1_.y) * vec2(other.g0_.y, other.g2_.y) - vec2(self.g1_.z) * vec2(other.g0_.z, other.g2_.z) + vec2(self.g2_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g2_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g2_.z) * vec2(0.0, -other.g1_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(-other.g0_.w, other.g1_.z, -other.g1_.y, -other.g2_.x) + vec4(self.g0_.y) * vec4(-other.g1_.z, -other.g0_.w, other.g1_.x, -other.g2_.y) + vec4(self.g0_.z) * vec4(other.g1_.y, -other.g1_.x, -other.g0_.w, -other.g2_.z) + vec4(self.g1_.x) * vec4(0.0, other.g0_.z, -other.g0_.y, 0.0) + vec4(self.g1_.y) * vec4(-other.g0_.z, 0.0, other.g0_.x, 0.0) + vec4(self.g1_.z) * vec4(other.g0_.y, -other.g0_.x, 0.0, 0.0) + vec4(self.g2_.x) * vec4(0.0, 0.0, 0.0, other.g0_.x) + vec4(self.g2_.y) * vec4(0.0, 0.0, 0.0, other.g0_.y) + self.g2_.wwwz * other.g0_.xyzz * vec4(-1.0, -1.0, -1.0, 1.0), vec3(self.g0_.x) * vec3(0.0, other.g2_.z, -other.g2_.y) + vec3(self.g0_.y) * vec3(-other.g2_.z, 0.0, other.g2_.x) + vec3(self.g0_.z) * vec3(other.g2_.y, -other.g2_.x, 0.0) + vec3(self.g1_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g1_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g1_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g2_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g2_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g2_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec3(self.g1_.x) * vec3(0.0, other.g2_.z, -other.g2_.y) + vec3(self.g1_.y) * vec3(-other.g2_.z, 0.0, other.g2_.x) + vec3(self.g1_.z) * vec3(other.g2_.y, -other.g2_.x, 0.0) + vec3(self.g2_.x) * vec3(other.g0_.w, other.g1_.z, -other.g1_.y) + vec3(self.g2_.y) * vec3(-other.g1_.z, other.g0_.w, other.g1_.x) + vec3(self.g2_.z) * vec3(other.g1_.y, -other.g1_.x, other.g0_.w) + vec3(self.g2_.w) * other.g2_, vec3(0.0), vec2(0.0));
}

MultiVector dipole_wedgeDot_dipole(Dipole self, Dipole other) {
    return MultiVector(vec2(self.g0_.x) * vec2(-other.g2_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g2_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g2_.z, 0.0) + vec2(self.g1_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g1_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g1_.z) * vec2(-other.g1_.z, 0.0) + vec2(self.g2_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g2_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g2_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g2_.w) * vec2(other.g2_.w, 0.0), vec3(0.0), vec2(0.0), vec3(self.g0_.x) * vec3(other.g2_.w, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, other.g2_.w, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, other.g2_.w) + vec3(self.g1_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g1_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g1_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) - vec3(self.g2_.w) * other.g0_, vec3(self.g0_.x) * vec3(0.0, other.g2_.z, -other.g2_.y) + vec3(self.g0_.y) * vec3(-other.g2_.z, 0.0, other.g2_.x) + vec3(self.g0_.z) * vec3(other.g2_.y, -other.g2_.x, 0.0) + vec3(self.g1_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g1_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g1_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g2_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g2_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g2_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, other.g2_.x) + vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, other.g2_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, 0.0, other.g2_.z) + vec4(self.g1_.x) * vec4(0.0, other.g2_.z, -other.g2_.y, 0.0) + vec4(self.g1_.y) * vec4(-other.g2_.z, 0.0, other.g2_.x, 0.0) + vec4(self.g1_.z) * vec4(other.g2_.y, -other.g2_.x, 0.0, 0.0) + vec4(self.g2_.x) * vec4(-other.g2_.w, other.g1_.z, -other.g1_.y, -other.g0_.x) + vec4(self.g2_.y) * vec4(-other.g1_.z, -other.g2_.w, other.g1_.x, -other.g0_.y) + vec4(self.g2_.z) * vec4(other.g1_.y, -other.g1_.x, -other.g2_.w, -other.g0_.z) + vec4(self.g2_.w) * vec4(other.g2_.x, other.g2_.y, other.g2_.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g2_.z, other.g2_.y) + vec3(self.g0_.y) * vec3(other.g2_.z, 0.0, -other.g2_.x) + vec3(self.g0_.z) * vec3(-other.g2_.y, other.g2_.x, 0.0) + self.g1_ * vec3(other.g2_.w) + vec3(self.g2_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g2_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g2_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + vec3(self.g2_.w) * other.g1_, vec2(self.g0_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g1_.z, 0.0) - vec2(self.g1_.x) * vec2(other.g0_.x, other.g2_.x) - vec2(self.g1_.y) * vec2(other.g0_.y, other.g2_.y) - vec2(self.g1_.z) * vec2(other.g0_.z, other.g2_.z) + vec2(self.g2_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g2_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g2_.z) * vec2(0.0, -other.g1_.z));
}

MultiVector dipole_wedgeDot_dualNum(Dipole self, DualNum other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), self.g0_ * vec3(other.g0_.x), self.g1_ * vec3(other.g0_.x), self.g2_ * vec4(other.g0_.x), vec4(self.g0_.x, self.g0_.y, self.g0_.z, self.g0_.x) * vec4(other.g0_.y, other.g0_.y, other.g0_.y, 0.0) + vec4(self.g2_.w) * vec4(0.0, 0.0, 0.0, -other.g0_.y), self.g1_ * vec3(other.g0_.y), vec3(self.g2_.x, self.g2_.y, self.g2_.z) * vec3(other.g0_.y), vec3(0.0), vec2(0.0));
}

MultiVector dipole_wedgeDot_flatPoint(Dipole self, FlatPoint other) {
    return MultiVector(vec2(self.g0_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g2_.w) * vec2(other.g0_.w, 0.0), vec3(0.0), vec2(0.0), self.g0_ * vec3(other.g0_.w), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, other.g0_.x) + vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, other.g0_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, 0.0, other.g0_.z) + vec4(self.g1_.x) * vec4(0.0, other.g0_.z, -other.g0_.y, 0.0) + vec4(self.g1_.y) * vec4(-other.g0_.z, 0.0, other.g0_.x, 0.0) + vec4(self.g1_.z) * vec4(other.g0_.y, -other.g0_.x, 0.0, 0.0) + self.g2_.xyzx * vec4(-other.g0_.w, -other.g0_.w, -other.g0_.w, 0.0) + vec4(self.g2_.w) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) + self.g1_ * vec3(other.g0_.w), vec2(self.g1_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g0_.z));
}

MultiVector dipole_wedgeDot_flector(Dipole self, Flector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g2_.w) * vec2(other.g0_.w, 0.0), vec3(0.0), vec2(0.0), vec3(self.g0_.x) * vec3(other.g0_.w, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, other.g0_.w, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, other.g0_.w), vec3(self.g0_.x) * vec3(other.g1_.w, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, other.g1_.w, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, other.g1_.w) + vec3(self.g2_.w) * vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, other.g0_.x) + vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, other.g0_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, 0.0, other.g0_.z) + vec4(self.g1_.x) * vec4(other.g1_.w, other.g0_.z, -other.g0_.y, -other.g1_.x) + vec4(self.g1_.y) * vec4(-other.g0_.z, other.g1_.w, other.g0_.x, -other.g1_.y) + vec4(self.g1_.z) * vec4(other.g0_.y, -other.g0_.x, other.g1_.w, -other.g1_.z) + vec4(self.g2_.x) * vec4(-other.g0_.w, -other.g1_.z, other.g1_.y, 0.0) + vec4(self.g2_.y) * vec4(other.g1_.z, -other.g0_.w, -other.g1_.x, 0.0) + vec4(self.g2_.z) * vec4(-other.g1_.y, other.g1_.x, -other.g0_.w, 0.0) + vec4(self.g2_.w) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x) * vec3(-other.g1_.w, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, -other.g1_.w, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, -other.g1_.w) + vec3(self.g1_.x) * vec3(other.g0_.w, other.g1_.z, -other.g1_.y) + vec3(self.g1_.y) * vec3(-other.g1_.z, other.g0_.w, other.g1_.x) + vec3(self.g1_.z) * vec3(other.g1_.y, -other.g1_.x, other.g0_.w), vec2(self.g0_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g1_.z, 0.0) + vec2(self.g1_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g2_.x) * vec2(0.0, other.g1_.x) + vec2(self.g2_.y) * vec2(0.0, other.g1_.y) + vec2(self.g2_.z) * vec2(0.0, other.g1_.z) + vec2(self.g2_.w) * vec2(0.0, other.g1_.w));
}

MultiVector dipole_wedgeDot_line(Dipole self, Line other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g1_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g0_.z), vec3(self.g0_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) - vec3(self.g2_.w) * other.g0_, vec2(self.g0_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g1_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g2_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g2_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g2_.z) * vec2(0.0, -other.g0_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(0.0, other.g0_.z, -other.g0_.y, -other.g1_.x) + vec4(self.g0_.y) * vec4(-other.g0_.z, 0.0, other.g0_.x, -other.g1_.y) + vec4(self.g0_.z) * vec4(other.g0_.y, -other.g0_.x, 0.0, -other.g1_.z), vec3(self.g0_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g1_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g1_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g1_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec3(self.g1_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g1_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g1_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g2_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g2_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g2_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + vec3(self.g2_.w) * other.g1_, vec3(0.0), vec2(0.0));
}

MultiVector dipole_wedgeDot_motor(Dipole self, Motor other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g1_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g0_.z), vec3(self.g0_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) - vec3(self.g2_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g0_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g1_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g2_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g2_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g2_.z) * vec2(0.0, -other.g0_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(other.g0_.w, other.g0_.z, -other.g0_.y, -other.g1_.x) + vec4(self.g0_.y) * vec4(-other.g0_.z, other.g0_.w, other.g0_.x, -other.g1_.y) + vec4(self.g0_.z) * vec4(other.g0_.y, -other.g0_.x, other.g0_.w, -other.g1_.z) + vec4(self.g2_.w) * vec4(0.0, 0.0, 0.0, -other.g0_.w), vec3(self.g0_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g1_.x) * vec3(other.g0_.w, other.g0_.z, -other.g0_.y) + vec3(self.g1_.y) * vec3(-other.g0_.z, other.g0_.w, other.g0_.x) + vec3(self.g1_.z) * vec3(other.g0_.y, -other.g0_.x, other.g0_.w), vec3(self.g1_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g1_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g1_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g2_.x) * vec3(other.g0_.w, other.g0_.z, -other.g0_.y) + vec3(self.g2_.y) * vec3(-other.g0_.z, other.g0_.w, other.g0_.x) + vec3(self.g2_.z) * vec3(other.g0_.y, -other.g0_.x, other.g0_.w) + vec3(self.g2_.w) * other.g1_, vec3(0.0), vec2(0.0));
}

MultiVector dipole_wedgeDot_multiVector(Dipole self, MultiVector other) {
    return MultiVector(vec2(0.0) - vec2(self.g0_.x) * vec2(other.g5_.x, other.g8_.x) - vec2(self.g0_.y) * vec2(other.g5_.y, other.g8_.y) - vec2(self.g0_.z) * vec2(other.g5_.z, other.g8_.z) - vec2(self.g1_.x) * vec2(other.g4_.x, other.g7_.x) - vec2(self.g1_.y) * vec2(other.g4_.y, other.g7_.y) - vec2(self.g1_.z) * vec2(other.g4_.z, other.g7_.z) - vec2(self.g2_.x) * vec2(other.g3_.x, other.g6_.x) - vec2(self.g2_.y) * vec2(other.g3_.y, other.g6_.y) - vec2(self.g2_.z) * vec2(other.g3_.z, other.g6_.z) + vec2(self.g2_.w) * vec2(other.g5_.w, -other.g6_.w), vec3(self.g0_.x) * vec3(other.g2_.y, other.g8_.z, -other.g8_.y) + vec3(self.g0_.y) * vec3(-other.g8_.z, other.g2_.y, other.g8_.x) + vec3(self.g0_.z) * vec3(other.g8_.y, -other.g8_.x, other.g2_.y) + vec3(self.g1_.x) * vec3(other.g6_.w, other.g1_.z, -other.g1_.y) + vec3(self.g1_.y) * vec3(-other.g1_.z, other.g6_.w, other.g1_.x) + vec3(self.g1_.z) * vec3(other.g1_.y, -other.g1_.x, other.g6_.w) + vec3(self.g2_.x) * vec3(-other.g2_.x, -other.g6_.z, other.g6_.y) + vec3(self.g2_.y) * vec3(other.g6_.z, -other.g2_.x, -other.g6_.x) + vec3(self.g2_.z) * vec3(-other.g6_.y, other.g6_.x, -other.g2_.x) - vec3(self.g2_.w) * other.g7_, vec2(self.g0_.x) * vec2(-other.g7_.x, 0.0) + vec2(self.g0_.x) * vec2(other.g1_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g7_.y, 0.0) + vec2(self.g0_.y) * vec2(other.g1_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g7_.z, 0.0) + vec2(self.g0_.z) * vec2(other.g1_.z, 0.0) - vec2(self.g1_.x) * vec2(other.g6_.x, other.g8_.x) - vec2(self.g1_.y) * vec2(other.g6_.y, other.g8_.y) - vec2(self.g1_.z) * vec2(other.g6_.z, other.g8_.z) + vec2(self.g2_.x) * vec2(0.0, -other.g7_.x) + vec2(self.g2_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g2_.y) * vec2(0.0, -other.g7_.y) + vec2(self.g2_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g2_.z) * vec2(0.0, -other.g7_.z) + vec2(self.g2_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g2_.w) * other.g2_ * vec2(-1.0, 1.0), vec3(self.g0_.x) * vec3(other.g5_.w, other.g9_.z, -other.g9_.y) + vec3(self.g0_.x) * vec3(other.g0_.x, other.g4_.z, -other.g4_.y) + vec3(self.g0_.y) * vec3(-other.g9_.z, other.g5_.w, other.g9_.x) + vec3(self.g0_.y) * vec3(-other.g4_.z, other.g0_.x, other.g4_.x) + vec3(self.g0_.z) * vec3(other.g9_.y, -other.g9_.x, other.g5_.w) + vec3(self.g0_.z) * vec3(other.g4_.y, -other.g4_.x, other.g0_.x) + vec3(self.g1_.x) * vec3(other.g10_.x, other.g3_.z, -other.g3_.y) + vec3(self.g1_.y) * vec3(-other.g3_.z, other.g10_.x, other.g3_.x) + vec3(self.g1_.z) * vec3(other.g3_.y, -other.g3_.x, other.g10_.x) - vec3(self.g2_.w) * other.g3_, vec3(self.g0_.x) * vec3(other.g10_.y, other.g5_.z, -other.g5_.y) + vec3(self.g0_.y) * vec3(-other.g5_.z, other.g10_.y, other.g5_.x) + vec3(self.g0_.z) * vec3(other.g5_.y, -other.g5_.x, other.g10_.y) + vec3(self.g1_.x) * vec3(other.g0_.x, other.g4_.z, -other.g4_.y) + vec3(self.g1_.y) * vec3(-other.g4_.z, other.g0_.x, other.g4_.x) + vec3(self.g1_.z) * vec3(other.g4_.y, -other.g4_.x, other.g0_.x) + vec3(self.g2_.x) * vec3(other.g10_.x, other.g3_.z, -other.g3_.y) + vec3(self.g2_.y) * vec3(-other.g3_.z, other.g10_.x, other.g3_.x) + vec3(self.g2_.z) * vec3(other.g3_.y, -other.g3_.x, other.g10_.x) + vec3(self.g2_.w) * other.g9_, vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, other.g5_.x) + vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, other.g5_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, 0.0, other.g5_.z) + vec4(self.g1_.x) * vec4(other.g10_.y, other.g5_.z, -other.g5_.y, -other.g9_.x) + vec4(self.g1_.y) * vec4(-other.g5_.z, other.g10_.y, other.g5_.x, -other.g9_.y) + vec4(self.g1_.z) * vec4(other.g5_.y, -other.g5_.x, other.g10_.y, -other.g9_.z) + vec4(self.g2_.x) * vec4(-other.g5_.w, -other.g9_.z, other.g9_.y, -other.g3_.x) + vec4(self.g2_.x) * vec4(other.g0_.x, other.g4_.z, -other.g4_.y, 0.0) + vec4(self.g2_.y) * vec4(other.g9_.z, -other.g5_.w, -other.g9_.x, -other.g3_.y) + vec4(self.g2_.y) * vec4(-other.g4_.z, other.g0_.x, other.g4_.x, 0.0) + vec4(self.g2_.z) * vec4(-other.g9_.y, other.g9_.x, -other.g5_.w, -other.g3_.z) + vec4(self.g2_.z) * vec4(other.g4_.y, -other.g4_.x, other.g0_.x, 0.0) + vec4(self.g2_.w) * vec4(other.g5_.x, other.g5_.y, other.g5_.z, other.g0_.x), vec4(self.g0_.x) * vec4(-other.g6_.w, other.g7_.z, -other.g7_.y, -other.g8_.x) + vec4(self.g0_.x) * vec4(other.g0_.y, -other.g1_.z, other.g1_.y, 0.0) + vec4(self.g0_.y) * vec4(-other.g7_.z, -other.g6_.w, other.g7_.x, -other.g8_.y) + vec4(self.g0_.y) * vec4(other.g1_.z, other.g0_.y, -other.g1_.x, 0.0) + vec4(self.g0_.z) * vec4(other.g7_.y, -other.g7_.x, -other.g6_.w, -other.g8_.z) + vec4(self.g0_.z) * vec4(-other.g1_.y, other.g1_.x, other.g0_.y, 0.0) + vec4(self.g1_.x) * vec4(other.g2_.x, other.g6_.z, -other.g6_.y, -other.g1_.x) + vec4(self.g1_.y) * vec4(-other.g6_.z, other.g2_.x, other.g6_.x, -other.g1_.y) + vec4(self.g1_.z) * vec4(other.g6_.y, -other.g6_.x, other.g2_.x, -other.g1_.z) + vec4(self.g2_.x) * vec4(0.0, 0.0, 0.0, other.g6_.x) + vec4(self.g2_.y) * vec4(0.0, 0.0, 0.0, other.g6_.y) + vec4(self.g2_.z) * vec4(0.0, 0.0, 0.0, other.g6_.z) - vec4(self.g2_.w) * vec4(other.g6_.x, other.g6_.y, other.g6_.z, other.g0_.y), vec3(self.g0_.x) * vec3(other.g2_.y, other.g8_.z, -other.g8_.y) + vec3(self.g0_.y) * vec3(-other.g8_.z, other.g2_.y, other.g8_.x) + vec3(self.g0_.z) * vec3(other.g8_.y, -other.g8_.x, other.g2_.y) + vec3(self.g1_.x) * vec3(other.g0_.y, other.g7_.z, -other.g7_.y) + vec3(self.g1_.y) * vec3(-other.g7_.z, other.g0_.y, other.g7_.x) + vec3(self.g1_.z) * vec3(other.g7_.y, -other.g7_.x, other.g0_.y) + vec3(self.g2_.x) * vec3(other.g2_.x, other.g6_.z, -other.g6_.y) + vec3(self.g2_.y) * vec3(-other.g6_.z, other.g2_.x, other.g6_.x) + vec3(self.g2_.z) * vec3(other.g6_.y, -other.g6_.x, other.g2_.x) - vec3(self.g2_.w) * other.g1_, vec3(self.g1_.x) * vec3(other.g2_.y, other.g8_.z, -other.g8_.y) + vec3(self.g1_.y) * vec3(-other.g8_.z, other.g2_.y, other.g8_.x) + vec3(self.g1_.z) * vec3(other.g8_.y, -other.g8_.x, other.g2_.y) + vec3(self.g2_.x) * vec3(other.g6_.w, other.g7_.z, -other.g7_.y) + vec3(self.g2_.x) * vec3(other.g0_.y, other.g1_.z, -other.g1_.y) + vec3(self.g2_.y) * vec3(-other.g7_.z, other.g6_.w, other.g7_.x) + vec3(self.g2_.y) * vec3(-other.g1_.z, other.g0_.y, other.g1_.x) + vec3(self.g2_.z) * vec3(other.g7_.y, -other.g7_.x, other.g6_.w) + vec3(self.g2_.z) * vec3(other.g1_.y, -other.g1_.x, other.g0_.y) + vec3(self.g2_.w) * other.g8_, vec3(self.g0_.x) * vec3(-other.g10_.y, -other.g5_.z, other.g5_.y) + vec3(self.g0_.y) * vec3(other.g5_.z, -other.g10_.y, -other.g5_.x) + vec3(self.g0_.z) * vec3(-other.g5_.y, other.g5_.x, -other.g10_.y) + vec3(self.g1_.x) * vec3(other.g5_.w, other.g9_.z, -other.g9_.y) + vec3(self.g1_.y) * vec3(-other.g9_.z, other.g5_.w, other.g9_.x) + vec3(self.g1_.z) * vec3(other.g9_.y, -other.g9_.x, other.g5_.w) + vec3(self.g2_.x) * vec3(other.g10_.x, other.g3_.z, -other.g3_.y) + vec3(self.g2_.y) * vec3(-other.g3_.z, other.g10_.x, other.g3_.x) + vec3(self.g2_.z) * vec3(other.g3_.y, -other.g3_.x, other.g10_.x) + vec3(self.g2_.w) * other.g4_, vec2(self.g0_.x) * vec2(-other.g9_.x, 0.0) + vec2(self.g0_.x) * vec2(-other.g4_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g9_.y, 0.0) + vec2(self.g0_.y) * vec2(-other.g4_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g9_.z, 0.0) + vec2(self.g0_.z) * vec2(-other.g4_.z, 0.0) - vec2(self.g1_.x) * vec2(other.g3_.x, other.g5_.x) - vec2(self.g1_.y) * vec2(other.g3_.y, other.g5_.y) - vec2(self.g1_.z) * vec2(other.g3_.z, other.g5_.z) + vec2(self.g2_.x) * vec2(0.0, other.g9_.x) + vec2(self.g2_.x) * vec2(0.0, -other.g4_.x) + vec2(self.g2_.y) * vec2(0.0, other.g9_.y) + vec2(self.g2_.y) * vec2(0.0, -other.g4_.y) + vec2(self.g2_.z) * vec2(0.0, other.g9_.z) + vec2(self.g2_.z) * vec2(0.0, -other.g4_.z) + vec2(self.g2_.w) * other.g10_ * vec2(-1.0, 1.0));
}

MultiVector dipole_wedgeDot_plane(Dipole self, Plane other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), self.g0_ * vec3(other.g0_.w) + vec3(self.g2_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec4(self.g1_.x) * vec4(other.g0_.w, 0.0, 0.0, -other.g0_.x) + vec4(self.g1_.y) * vec4(0.0, other.g0_.w, 0.0, -other.g0_.y) + vec4(self.g1_.z) * vec4(0.0, 0.0, other.g0_.w, -other.g0_.z) + vec4(self.g2_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g2_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g2_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - self.g0_ * vec3(other.g0_.w) + vec3(self.g1_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g1_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g1_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec2(self.g0_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g2_.x) * vec2(0.0, other.g0_.x) + vec2(self.g2_.y) * vec2(0.0, other.g0_.y) + vec2(self.g2_.z) * vec2(0.0, other.g0_.z) + vec2(self.g2_.w) * vec2(0.0, other.g0_.w));
}

MultiVector dipole_wedgeDot_roundPoint(Dipole self, RoundPoint other) {
    return MultiVector(vec2(0.0), self.g0_ * vec3(other.g1_.y) + vec3(self.g1_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g1_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g1_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) - vec3(self.g2_.x, self.g2_.y, self.g2_.z) * vec3(other.g1_.x), vec2(self.g0_.x) * vec2(other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(other.g0_.z, 0.0) + vec2(self.g2_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g2_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g2_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g2_.w) * other.g1_ * vec2(-1.0, 1.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g0_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g0_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0) + vec4(self.g1_.x) * vec4(other.g1_.x, 0.0, 0.0, -other.g0_.x) + vec4(self.g1_.y) * vec4(0.0, other.g1_.x, 0.0, -other.g0_.y) + vec4(self.g1_.z) * vec4(0.0, 0.0, other.g1_.x, -other.g0_.z), self.g0_ * vec3(other.g1_.y) + vec3(self.g2_.x, self.g2_.y, self.g2_.z) * vec3(other.g1_.x) - vec3(self.g2_.w) * other.g0_, self.g1_ * vec3(other.g1_.y) + vec3(self.g2_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g2_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g2_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec3(0.0), vec2(0.0));
}

Dipole dipole_wedgeDot_scalar(Dipole self, Scalar other) {
    return Dipole(self.g0_ * vec3(other.g0_), self.g1_ * vec3(other.g0_), self.g2_ * vec4(other.g0_));
}

MultiVector dipole_wedgeDot_sphere(Dipole self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + self.g1_ * vec3(other.g1_.x), self.g0_ * vec3(other.g1_.y) + vec3(self.g2_.x, self.g2_.y, self.g2_.z) * vec3(other.g1_.x) + vec3(self.g2_.w) * other.g0_, vec4(self.g1_.x) * vec4(other.g1_.y, 0.0, 0.0, -other.g0_.x) + vec4(self.g1_.y) * vec4(0.0, other.g1_.y, 0.0, -other.g0_.y) + vec4(self.g1_.z) * vec4(0.0, 0.0, other.g1_.y, -other.g0_.z) + vec4(self.g2_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g2_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g2_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - self.g0_ * vec3(other.g1_.y) + vec3(self.g1_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g1_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g1_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + vec3(self.g2_.x, self.g2_.y, self.g2_.z) * vec3(other.g1_.x), vec2(self.g0_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g2_.x) * vec2(0.0, other.g0_.x) + vec2(self.g2_.y) * vec2(0.0, other.g0_.y) + vec2(self.g2_.z) * vec2(0.0, other.g0_.z) + vec2(self.g2_.w) * other.g1_ * vec2(-1.0, 1.0));
}

DualNum dualNum_wedgeDot_antiScalar(DualNum self, AntiScalar other) {
    return DualNum(self.g0_.yx * vec2(-other.g0_));
}

MultiVector dualNum_wedgeDot_circle(DualNum self, Circle other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0) - vec3(self.g0_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(0.0) - vec3(self.g0_.y) * other.g1_, vec4(self.g0_.y) * vec4(-other.g2_.x, -other.g2_.y, -other.g2_.z, other.g0_.w), vec4(self.g0_.x) * other.g0_, vec3(self.g0_.x) * other.g1_, vec3(self.g0_.x) * other.g2_, vec3(0.0), vec2(0.0));
}

MultiVector dualNum_wedgeDot_dipole(DualNum self, Dipole other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g0_.x) * other.g0_, vec3(self.g0_.x) * other.g1_, vec4(self.g0_.x) * other.g2_, vec4(self.g0_.y) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, -other.g2_.w), vec3(self.g0_.y) * other.g1_, vec3(self.g0_.y) * vec3(other.g2_.x, other.g2_.y, other.g2_.z), vec3(0.0), vec2(0.0));
}

DualNum dualNum_wedgeDot_dualNum(DualNum self, DualNum other) {
    return DualNum(vec2(self.g0_.x) * other.g0_ + vec2(self.g0_.y) * other.g0_.yx * vec2(-1.0, 1.0));
}

MultiVector dualNum_wedgeDot_flatPoint(DualNum self, FlatPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(self.g0_.x) * other.g0_, vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, -other.g0_.w), vec3(0.0), vec3(self.g0_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(0.0), vec2(0.0));
}

MultiVector dualNum_wedgeDot_flector(DualNum self, Flector other) {
    return MultiVector(vec2(0.0), vec3(0.0) - vec3(self.g0_.y) * vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec2(self.g0_.y) * vec2(0.0, other.g1_.w), vec3(0.0), vec3(0.0), vec4(self.g0_.x) * other.g0_, vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, -other.g0_.w), vec3(0.0), vec3(self.g0_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(self.g0_.x) * vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec2(self.g0_.x) * vec2(0.0, other.g1_.w));
}

MultiVector dualNum_wedgeDot_line(DualNum self, Line other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0) - vec3(self.g0_.y) * other.g0_, vec4(self.g0_.y) * vec4(-other.g1_.x, -other.g1_.y, -other.g1_.z, 0.0), vec4(0.0), vec3(self.g0_.x) * other.g0_, vec3(self.g0_.x) * other.g1_, vec3(0.0), vec2(0.0));
}

MultiVector dualNum_wedgeDot_motor(DualNum self, Motor other) {
    return MultiVector(self.g0_.yx * vec2(-other.g0_.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0) - vec3(self.g0_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec4(self.g0_.y) * vec4(-other.g1_.x, -other.g1_.y, -other.g1_.z, 0.0), vec4(0.0), vec3(self.g0_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(self.g0_.x) * other.g1_, vec3(0.0), vec2(0.0));
}

MultiVector dualNum_wedgeDot_multiVector(DualNum self, MultiVector other) {
    return MultiVector(vec2(self.g0_.x) * other.g0_ + vec2(self.g0_.y) * other.g0_.yx * vec2(-1.0, 1.0), vec3(self.g0_.x) * other.g1_ - vec3(self.g0_.y) * other.g9_, vec2(self.g0_.x) * other.g2_ + vec2(self.g0_.y) * other.g10_, vec3(self.g0_.x) * other.g3_ - vec3(self.g0_.y) * vec3(other.g6_.x, other.g6_.y, other.g6_.z), vec3(self.g0_.x) * other.g4_ - vec3(self.g0_.y) * other.g7_, vec4(self.g0_.x) * other.g5_ + vec4(self.g0_.y) * vec4(-other.g8_.x, -other.g8_.y, -other.g8_.z, other.g6_.w), vec4(self.g0_.x) * other.g6_ + vec4(self.g0_.y) * vec4(other.g3_.x, other.g3_.y, other.g3_.z, -other.g5_.w), vec3(self.g0_.x) * other.g7_ + vec3(self.g0_.y) * other.g4_, vec3(self.g0_.x) * other.g8_ + vec3(self.g0_.y) * vec3(other.g5_.x, other.g5_.y, other.g5_.z), vec3(self.g0_.x) * other.g9_ + vec3(self.g0_.y) * other.g1_, vec2(self.g0_.x) * other.g10_ - vec2(self.g0_.y) * other.g2_);
}

MultiVector dualNum_wedgeDot_plane(DualNum self, Plane other) {
    return MultiVector(vec2(0.0), vec3(0.0) - vec3(self.g0_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g0_.y) * vec2(0.0, other.g0_.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g0_.x) * vec2(0.0, other.g0_.w));
}

MultiVector dualNum_wedgeDot_roundPoint(DualNum self, RoundPoint other) {
    return MultiVector(vec2(0.0), vec3(self.g0_.x) * other.g0_, vec2(self.g0_.x) * other.g1_, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.y) * other.g0_, vec2(0.0) - vec2(self.g0_.y) * other.g1_);
}

DualNum dualNum_wedgeDot_scalar(DualNum self, Scalar other) {
    return DualNum(self.g0_ * vec2(other.g0_));
}

MultiVector dualNum_wedgeDot_sphere(DualNum self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(0.0) - vec3(self.g0_.y) * other.g0_, vec2(self.g0_.y) * other.g1_, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x) * other.g0_, vec2(self.g0_.x) * other.g1_);
}

Circle flatPoint_wedgeDot_antiScalar(FlatPoint self, AntiScalar other) {
    return Circle(vec4(self.g0_.w) * vec4(0.0, 0.0, 0.0, -other.g0_), vec3(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_));
}

MultiVector flatPoint_wedgeDot_circle(FlatPoint self, Circle other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g0_.w) * vec2(0.0, -other.g0_.w), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) - vec3(self.g0_.w) * other.g1_, vec2(self.g0_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g1_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, other.g0_.x) + vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, other.g0_.y) + self.g0_.wwwz * other.g0_.xyzz * vec4(-1.0, -1.0, -1.0, 1.0), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec3(self.g0_.x) * vec3(other.g0_.w, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, other.g0_.w, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, other.g0_.w) + vec3(self.g0_.w) * other.g2_, vec3(0.0), vec2(0.0));
}

MultiVector flatPoint_wedgeDot_dipole(FlatPoint self, Dipole other) {
    return MultiVector(vec2(self.g0_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g0_.w) * vec2(other.g2_.w, 0.0), vec3(0.0), vec2(0.0), vec3(0.0) - vec3(self.g0_.w) * other.g0_, vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec4(self.g0_.x) * vec4(-other.g2_.w, other.g1_.z, -other.g1_.y, -other.g0_.x) + vec4(self.g0_.y) * vec4(-other.g1_.z, -other.g2_.w, other.g1_.x, -other.g0_.y) + vec4(self.g0_.z) * vec4(other.g1_.y, -other.g1_.x, -other.g2_.w, -other.g0_.z) + vec4(self.g0_.w) * vec4(other.g2_.x, other.g2_.y, other.g2_.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + vec3(self.g0_.w) * other.g1_, vec2(self.g0_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g1_.z));
}

MultiVector flatPoint_wedgeDot_dualNum(FlatPoint self, DualNum other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0_ * vec4(other.g0_.x), vec4(self.g0_.w) * vec4(0.0, 0.0, 0.0, -other.g0_.y), vec3(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_.y), vec3(0.0), vec2(0.0));
}

MultiVector flatPoint_wedgeDot_flatPoint(FlatPoint self, FlatPoint other) {
    return MultiVector(vec2(self.g0_.w) * vec2(other.g0_.w, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0_.xyzx * vec4(-other.g0_.w, -other.g0_.w, -other.g0_.w, 0.0) + vec4(self.g0_.w) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector flatPoint_wedgeDot_flector(FlatPoint self, Flector other) {
    return MultiVector(vec2(self.g0_.w) * vec2(other.g0_.w, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(self.g0_.w) * vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec4(self.g0_.x) * vec4(-other.g0_.w, -other.g1_.z, other.g1_.y, 0.0) + vec4(self.g0_.y) * vec4(other.g1_.z, -other.g0_.w, -other.g1_.x, 0.0) + vec4(self.g0_.z) * vec4(-other.g1_.y, other.g1_.x, -other.g0_.w, 0.0) + vec4(self.g0_.w) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(self.g0_.x) * vec2(0.0, other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, other.g1_.z) + vec2(self.g0_.w) * vec2(0.0, other.g1_.w));
}

MultiVector flatPoint_wedgeDot_line(FlatPoint self, Line other) {
    return MultiVector(vec2(0.0), vec3(0.0) - vec3(self.g0_.w) * other.g0_, vec2(self.g0_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g0_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + vec3(self.g0_.w) * other.g1_, vec3(0.0), vec2(0.0));
}

MultiVector flatPoint_wedgeDot_motor(FlatPoint self, Motor other) {
    return MultiVector(vec2(0.0), vec3(0.0) - vec3(self.g0_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g0_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g0_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.w) * vec4(0.0, 0.0, 0.0, -other.g0_.w), vec3(0.0), vec3(self.g0_.x) * vec3(other.g0_.w, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, other.g0_.w, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, other.g0_.w) + vec3(self.g0_.w) * other.g1_, vec3(0.0), vec2(0.0));
}

MultiVector flatPoint_wedgeDot_multiVector(FlatPoint self, MultiVector other) {
    return MultiVector(vec2(0.0) - vec2(self.g0_.x) * vec2(other.g3_.x, other.g6_.x) - vec2(self.g0_.y) * vec2(other.g3_.y, other.g6_.y) - vec2(self.g0_.z) * vec2(other.g3_.z, other.g6_.z) + vec2(self.g0_.w) * vec2(other.g5_.w, -other.g6_.w), vec3(self.g0_.x) * vec3(-other.g2_.x, -other.g6_.z, other.g6_.y) + vec3(self.g0_.y) * vec3(other.g6_.z, -other.g2_.x, -other.g6_.x) + vec3(self.g0_.z) * vec3(-other.g6_.y, other.g6_.x, -other.g2_.x) - vec3(self.g0_.w) * other.g7_, vec2(self.g0_.x) * vec2(0.0, -other.g7_.x) + vec2(self.g0_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g7_.y) + vec2(self.g0_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g7_.z) + vec2(self.g0_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g0_.w) * other.g2_ * vec2(-1.0, 1.0), vec3(0.0) - vec3(self.g0_.w) * other.g3_, vec3(self.g0_.x) * vec3(other.g10_.x, other.g3_.z, -other.g3_.y) + vec3(self.g0_.y) * vec3(-other.g3_.z, other.g10_.x, other.g3_.x) + vec3(self.g0_.z) * vec3(other.g3_.y, -other.g3_.x, other.g10_.x) + vec3(self.g0_.w) * other.g9_, vec4(self.g0_.x) * vec4(-other.g5_.w, -other.g9_.z, other.g9_.y, -other.g3_.x) + vec4(self.g0_.x) * vec4(other.g0_.x, other.g4_.z, -other.g4_.y, 0.0) + vec4(self.g0_.y) * vec4(other.g9_.z, -other.g5_.w, -other.g9_.x, -other.g3_.y) + vec4(self.g0_.y) * vec4(-other.g4_.z, other.g0_.x, other.g4_.x, 0.0) + vec4(self.g0_.z) * vec4(-other.g9_.y, other.g9_.x, -other.g5_.w, -other.g3_.z) + vec4(self.g0_.z) * vec4(other.g4_.y, -other.g4_.x, other.g0_.x, 0.0) + vec4(self.g0_.w) * vec4(other.g5_.x, other.g5_.y, other.g5_.z, other.g0_.x), vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, other.g6_.x) + vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, other.g6_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, 0.0, other.g6_.z) - vec4(self.g0_.w) * vec4(other.g6_.x, other.g6_.y, other.g6_.z, other.g0_.y), vec3(self.g0_.x) * vec3(other.g2_.x, other.g6_.z, -other.g6_.y) + vec3(self.g0_.y) * vec3(-other.g6_.z, other.g2_.x, other.g6_.x) + vec3(self.g0_.z) * vec3(other.g6_.y, -other.g6_.x, other.g2_.x) - vec3(self.g0_.w) * other.g1_, vec3(self.g0_.x) * vec3(other.g6_.w, other.g7_.z, -other.g7_.y) + vec3(self.g0_.x) * vec3(other.g0_.y, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g7_.z, other.g6_.w, other.g7_.x) + vec3(self.g0_.y) * vec3(-other.g1_.z, other.g0_.y, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g7_.y, -other.g7_.x, other.g6_.w) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, other.g0_.y) + vec3(self.g0_.w) * other.g8_, vec3(self.g0_.x) * vec3(other.g10_.x, other.g3_.z, -other.g3_.y) + vec3(self.g0_.y) * vec3(-other.g3_.z, other.g10_.x, other.g3_.x) + vec3(self.g0_.z) * vec3(other.g3_.y, -other.g3_.x, other.g10_.x) + vec3(self.g0_.w) * other.g4_, vec2(self.g0_.x) * vec2(0.0, other.g9_.x) + vec2(self.g0_.x) * vec2(0.0, -other.g4_.x) + vec2(self.g0_.y) * vec2(0.0, other.g9_.y) + vec2(self.g0_.y) * vec2(0.0, -other.g4_.y) + vec2(self.g0_.z) * vec2(0.0, other.g9_.z) + vec2(self.g0_.z) * vec2(0.0, -other.g4_.z) + vec2(self.g0_.w) * other.g10_ * vec2(-1.0, 1.0));
}

MultiVector flatPoint_wedgeDot_plane(FlatPoint self, Plane other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(self.g0_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec4(self.g0_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g0_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g0_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(self.g0_.x) * vec2(0.0, other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, other.g0_.z) + vec2(self.g0_.w) * vec2(0.0, other.g0_.w));
}

MultiVector flatPoint_wedgeDot_roundPoint(FlatPoint self, RoundPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0) - vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g1_.x), vec2(self.g0_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g0_.w) * other.g1_ * vec2(-1.0, 1.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g1_.x) - vec3(self.g0_.w) * other.g0_, vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec3(0.0), vec2(0.0));
}

FlatPoint flatPoint_wedgeDot_scalar(FlatPoint self, Scalar other) {
    return FlatPoint(self.g0_ * vec4(other.g0_));
}

MultiVector flatPoint_wedgeDot_sphere(FlatPoint self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g1_.x) + vec3(self.g0_.w) * other.g0_, vec4(self.g0_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g0_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g0_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g1_.x), vec2(self.g0_.x) * vec2(0.0, other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, other.g0_.z) + vec2(self.g0_.w) * other.g1_ * vec2(-1.0, 1.0));
}

MultiVector flector_wedgeDot_antiScalar(Flector self, AntiScalar other) {
    return MultiVector(vec2(0.0), vec3(0.0) - vec3(self.g1_.x, self.g1_.y, self.g1_.z) * vec3(other.g0_), vec2(self.g1_.w) * vec2(0.0, other.g0_), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.w) * vec4(0.0, 0.0, 0.0, -other.g0_), vec3(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_), vec3(0.0), vec2(0.0));
}

MultiVector flector_wedgeDot_circle(Flector self, Circle other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g0_.w) * vec2(0.0, -other.g0_.w), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) - vec3(self.g0_.w) * other.g1_ + vec3(self.g1_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g1_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g1_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) - vec3(self.g1_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g0_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g1_.x) * vec2(other.g0_.x, -other.g2_.x) + vec2(self.g1_.y) * vec2(other.g0_.y, -other.g2_.y) + vec2(self.g1_.z) * vec2(other.g0_.z, -other.g2_.z) + vec2(self.g1_.w) * vec2(0.0, other.g0_.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, other.g0_.x) + vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, other.g0_.y) + self.g0_.wwwz * other.g0_.xyzz * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g1_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, other.g1_.x) + vec4(self.g1_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, other.g1_.y) + vec4(self.g1_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, other.g1_.z), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) - vec3(self.g1_.x, self.g1_.y, self.g1_.z) * vec3(other.g0_.w) + vec3(self.g1_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(self.g0_.x) * vec3(other.g0_.w, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, other.g0_.w, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, other.g0_.w) + vec3(self.g0_.w) * other.g2_ + vec3(self.g1_.x) * vec3(0.0, other.g2_.z, -other.g2_.y) + vec3(self.g1_.y) * vec3(-other.g2_.z, 0.0, other.g2_.x) + vec3(self.g1_.z) * vec3(other.g2_.y, -other.g2_.x, 0.0) + vec3(self.g1_.w) * other.g1_, vec3(0.0), vec2(0.0));
}

MultiVector flector_wedgeDot_dipole(Flector self, Dipole other) {
    return MultiVector(vec2(self.g0_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g0_.w) * vec2(other.g2_.w, 0.0), vec3(0.0), vec2(0.0), vec3(0.0) - vec3(self.g0_.w) * other.g0_ + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + vec3(self.g1_.x, self.g1_.y, self.g1_.z) * vec3(other.g2_.w) + vec3(self.g1_.w) * other.g0_, vec4(self.g0_.x) * vec4(-other.g2_.w, other.g1_.z, -other.g1_.y, -other.g0_.x) + vec4(self.g0_.y) * vec4(-other.g1_.z, -other.g2_.w, other.g1_.x, -other.g0_.y) + vec4(self.g0_.z) * vec4(other.g1_.y, -other.g1_.x, -other.g2_.w, -other.g0_.z) + vec4(self.g0_.w) * vec4(other.g2_.x, other.g2_.y, other.g2_.z, 0.0) + vec4(self.g1_.x) * vec4(0.0, other.g2_.z, -other.g2_.y, -other.g1_.x) + vec4(self.g1_.y) * vec4(-other.g2_.z, 0.0, other.g2_.x, -other.g1_.y) + vec4(self.g1_.z) * vec4(other.g2_.y, -other.g2_.x, 0.0, -other.g1_.z) + vec4(self.g1_.w) * vec4(other.g1_.x, other.g1_.y, other.g1_.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + vec3(self.g0_.w) * other.g1_ + vec3(self.g1_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g1_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g1_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g1_.w) * other.g0_, vec2(self.g0_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g1_.x) * vec2(other.g0_.x, -other.g2_.x) + vec2(self.g1_.y) * vec2(other.g0_.y, -other.g2_.y) + vec2(self.g1_.z) * vec2(other.g0_.z, -other.g2_.z) + vec2(self.g1_.w) * vec2(0.0, -other.g2_.w));
}

MultiVector flector_wedgeDot_dualNum(Flector self, DualNum other) {
    return MultiVector(vec2(0.0), vec3(0.0) - vec3(self.g1_.x, self.g1_.y, self.g1_.z) * vec3(other.g0_.y), vec2(self.g1_.w) * vec2(0.0, other.g0_.y), vec3(0.0), vec3(0.0), self.g0_ * vec4(other.g0_.x), vec4(self.g0_.w) * vec4(0.0, 0.0, 0.0, -other.g0_.y), vec3(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_.y), vec3(self.g1_.x, self.g1_.y, self.g1_.z) * vec3(other.g0_.x), vec2(self.g1_.w) * vec2(0.0, other.g0_.x));
}

MultiVector flector_wedgeDot_flatPoint(Flector self, FlatPoint other) {
    return MultiVector(vec2(self.g0_.w) * vec2(other.g0_.w, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(self.g1_.x, self.g1_.y, self.g1_.z) * vec3(other.g0_.w), self.g0_.xyzx * vec4(-other.g0_.w, -other.g0_.w, -other.g0_.w, 0.0) + vec4(self.g0_.w) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, 0.0) + vec4(self.g1_.x) * vec4(0.0, other.g0_.z, -other.g0_.y, 0.0) + vec4(self.g1_.y) * vec4(-other.g0_.z, 0.0, other.g0_.x, 0.0) + vec4(self.g1_.z) * vec4(other.g0_.y, -other.g0_.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(self.g1_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g1_.w) * vec2(0.0, -other.g0_.w));
}

MultiVector flector_wedgeDot_flector(Flector self, Flector other) {
    return MultiVector(vec2(self.g0_.w) * vec2(other.g0_.w, 0.0) + vec2(self.g1_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g1_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g1_.z) * vec2(-other.g1_.z, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(self.g0_.w) * vec3(other.g1_.x, other.g1_.y, other.g1_.z) + vec3(self.g1_.x) * vec3(other.g0_.w, other.g1_.z, -other.g1_.y) + vec3(self.g1_.y) * vec3(-other.g1_.z, other.g0_.w, other.g1_.x) + vec3(self.g1_.z) * vec3(other.g1_.y, -other.g1_.x, other.g0_.w), vec4(self.g0_.x) * vec4(-other.g0_.w, -other.g1_.z, other.g1_.y, 0.0) + vec4(self.g0_.y) * vec4(other.g1_.z, -other.g0_.w, -other.g1_.x, 0.0) + vec4(self.g0_.z) * vec4(-other.g1_.y, other.g1_.x, -other.g0_.w, 0.0) + vec4(self.g0_.w) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, 0.0) + vec4(self.g1_.x) * vec4(other.g1_.w, other.g0_.z, -other.g0_.y, 0.0) + vec4(self.g1_.y) * vec4(-other.g0_.z, other.g1_.w, other.g0_.x, 0.0) + vec4(self.g1_.z) * vec4(other.g0_.y, -other.g0_.x, other.g1_.w, 0.0) + vec4(self.g1_.w) * vec4(-other.g1_.x, -other.g1_.y, -other.g1_.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(self.g0_.x) * vec2(0.0, other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, other.g1_.z) + vec2(self.g0_.w) * vec2(0.0, other.g1_.w) + vec2(self.g1_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g1_.w) * vec2(0.0, -other.g0_.w));
}

MultiVector flector_wedgeDot_line(Flector self, Line other) {
    return MultiVector(vec2(0.0), vec3(0.0) - vec3(self.g0_.w) * other.g0_ + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec2(self.g0_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g1_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g1_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g1_.x) * vec4(0.0, 0.0, 0.0, other.g0_.x) + vec4(self.g1_.y) * vec4(0.0, 0.0, 0.0, other.g0_.y) + vec4(self.g1_.z) * vec4(0.0, 0.0, 0.0, other.g0_.z), vec3(0.0), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + vec3(self.g0_.w) * other.g1_ + vec3(self.g1_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g1_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g1_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g1_.w) * other.g0_, vec3(0.0), vec2(0.0));
}

MultiVector flector_wedgeDot_motor(Flector self, Motor other) {
    return MultiVector(vec2(0.0), vec3(0.0) - vec3(self.g0_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g1_.x) * vec3(-other.g0_.w, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, -other.g0_.w, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, -other.g0_.w), vec2(self.g0_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g1_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g1_.w) * vec2(0.0, other.g0_.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.w) * vec4(0.0, 0.0, 0.0, -other.g0_.w) + vec4(self.g1_.x) * vec4(0.0, 0.0, 0.0, other.g0_.x) + vec4(self.g1_.y) * vec4(0.0, 0.0, 0.0, other.g0_.y) + vec4(self.g1_.z) * vec4(0.0, 0.0, 0.0, other.g0_.z), vec3(0.0), vec3(self.g0_.x) * vec3(other.g0_.w, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, other.g0_.w, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, other.g0_.w) + vec3(self.g0_.w) * other.g1_ + vec3(self.g1_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g1_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g1_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g1_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(0.0), vec2(0.0));
}

MultiVector flector_wedgeDot_multiVector(Flector self, MultiVector other) {
    return MultiVector(vec2(0.0) - vec2(self.g0_.x) * vec2(other.g3_.x, other.g6_.x) - vec2(self.g0_.y) * vec2(other.g3_.y, other.g6_.y) - vec2(self.g0_.z) * vec2(other.g3_.z, other.g6_.z) + vec2(self.g0_.w) * vec2(other.g5_.w, -other.g6_.w) + vec2(self.g1_.x) * vec2(-other.g9_.x, other.g1_.x) + vec2(self.g1_.y) * vec2(-other.g9_.y, other.g1_.y) + vec2(self.g1_.z) * vec2(-other.g9_.z, other.g1_.z) + vec2(self.g1_.w) * vec2(other.g10_.x, other.g2_.x), vec3(self.g0_.x) * vec3(-other.g2_.x, -other.g6_.z, other.g6_.y) + vec3(self.g0_.y) * vec3(other.g6_.z, -other.g2_.x, -other.g6_.x) + vec3(self.g0_.z) * vec3(-other.g6_.y, other.g6_.x, -other.g2_.x) - vec3(self.g0_.w) * other.g7_ + vec3(self.g1_.x) * vec3(-other.g0_.y, -other.g7_.z, other.g7_.y) + vec3(self.g1_.y) * vec3(other.g7_.z, -other.g0_.y, -other.g7_.x) + vec3(self.g1_.z) * vec3(-other.g7_.y, other.g7_.x, -other.g0_.y) - vec3(self.g1_.w) * vec3(other.g6_.x, other.g6_.y, other.g6_.z), vec2(self.g0_.x) * vec2(0.0, -other.g7_.x) + vec2(self.g0_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g7_.y) + vec2(self.g0_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g7_.z) + vec2(self.g0_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g0_.w) * other.g2_ * vec2(-1.0, 1.0) + vec2(self.g1_.x) * vec2(other.g6_.x, -other.g8_.x) + vec2(self.g1_.y) * vec2(other.g6_.y, -other.g8_.y) + vec2(self.g1_.z) * vec2(other.g6_.z, -other.g8_.z) + vec2(self.g1_.w) * vec2(0.0, other.g6_.w) + vec2(self.g1_.w) * vec2(0.0, other.g0_.y), vec3(0.0) - vec3(self.g0_.w) * other.g3_ + vec3(self.g1_.x) * vec3(-other.g10_.x, -other.g3_.z, other.g3_.y) + vec3(self.g1_.y) * vec3(other.g3_.z, -other.g10_.x, -other.g3_.x) + vec3(self.g1_.z) * vec3(-other.g3_.y, other.g3_.x, -other.g10_.x), vec3(self.g0_.x) * vec3(other.g10_.x, other.g3_.z, -other.g3_.y) + vec3(self.g0_.y) * vec3(-other.g3_.z, other.g10_.x, other.g3_.x) + vec3(self.g0_.z) * vec3(other.g3_.y, -other.g3_.x, other.g10_.x) + vec3(self.g0_.w) * other.g9_ + vec3(self.g1_.x) * vec3(other.g5_.w, other.g9_.z, -other.g9_.y) + vec3(self.g1_.y) * vec3(-other.g9_.z, other.g5_.w, other.g9_.x) + vec3(self.g1_.z) * vec3(other.g9_.y, -other.g9_.x, other.g5_.w) + vec3(self.g1_.w) * other.g3_, vec4(self.g0_.x) * vec4(-other.g5_.w, -other.g9_.z, other.g9_.y, -other.g3_.x) + vec4(self.g0_.x) * vec4(other.g0_.x, other.g4_.z, -other.g4_.y, 0.0) + vec4(self.g0_.y) * vec4(other.g9_.z, -other.g5_.w, -other.g9_.x, -other.g3_.y) + vec4(self.g0_.y) * vec4(-other.g4_.z, other.g0_.x, other.g4_.x, 0.0) + vec4(self.g0_.z) * vec4(-other.g9_.y, other.g9_.x, -other.g5_.w, -other.g3_.z) + vec4(self.g0_.z) * vec4(other.g4_.y, -other.g4_.x, other.g0_.x, 0.0) + vec4(self.g0_.w) * vec4(other.g5_.x, other.g5_.y, other.g5_.z, other.g0_.x) + vec4(self.g1_.x) * vec4(other.g10_.y, other.g5_.z, -other.g5_.y, -other.g4_.x) + vec4(self.g1_.y) * vec4(-other.g5_.z, other.g10_.y, other.g5_.x, -other.g4_.y) + vec4(self.g1_.z) * vec4(other.g5_.y, -other.g5_.x, other.g10_.y, -other.g4_.z) + vec4(self.g1_.w) * vec4(-other.g9_.x, -other.g9_.y, -other.g9_.z, other.g10_.x) + vec4(self.g1_.w) * vec4(other.g4_.x, other.g4_.y, other.g4_.z, 0.0), vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, other.g6_.x) + vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, other.g6_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, 0.0, other.g6_.z) - vec4(self.g0_.w) * vec4(other.g6_.x, other.g6_.y, other.g6_.z, other.g0_.y) + vec4(self.g1_.x) * vec4(-other.g2_.x, -other.g6_.z, other.g6_.y, other.g7_.x) + vec4(self.g1_.y) * vec4(other.g6_.z, -other.g2_.x, -other.g6_.x, other.g7_.y) + vec4(self.g1_.z) * vec4(-other.g6_.y, other.g6_.x, -other.g2_.x, other.g7_.z) + vec4(self.g1_.w) * vec4(0.0, 0.0, 0.0, -other.g2_.x), vec3(self.g0_.x) * vec3(other.g2_.x, other.g6_.z, -other.g6_.y) + vec3(self.g0_.y) * vec3(-other.g6_.z, other.g2_.x, other.g6_.x) + vec3(self.g0_.z) * vec3(other.g6_.y, -other.g6_.x, other.g2_.x) - vec3(self.g0_.w) * other.g1_ + vec3(self.g1_.x) * vec3(-other.g6_.w, -other.g1_.z, other.g1_.y) + vec3(self.g1_.y) * vec3(other.g1_.z, -other.g6_.w, -other.g1_.x) + vec3(self.g1_.z) * vec3(-other.g1_.y, other.g1_.x, -other.g6_.w) + vec3(self.g1_.w) * vec3(other.g6_.x, other.g6_.y, other.g6_.z), vec3(self.g0_.x) * vec3(other.g6_.w, other.g7_.z, -other.g7_.y) + vec3(self.g0_.x) * vec3(other.g0_.y, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g7_.z, other.g6_.w, other.g7_.x) + vec3(self.g0_.y) * vec3(-other.g1_.z, other.g0_.y, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g7_.y, -other.g7_.x, other.g6_.w) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, other.g0_.y) + vec3(self.g0_.w) * other.g8_ + vec3(self.g1_.x) * vec3(other.g2_.y, other.g8_.z, -other.g8_.y) + vec3(self.g1_.y) * vec3(-other.g8_.z, other.g2_.y, other.g8_.x) + vec3(self.g1_.z) * vec3(other.g8_.y, -other.g8_.x, other.g2_.y) + vec3(self.g1_.w) * other.g7_ + vec3(self.g1_.w) * other.g1_, vec3(self.g0_.x) * vec3(other.g10_.x, other.g3_.z, -other.g3_.y) + vec3(self.g0_.y) * vec3(-other.g3_.z, other.g10_.x, other.g3_.x) + vec3(self.g0_.z) * vec3(other.g3_.y, -other.g3_.x, other.g10_.x) + vec3(self.g0_.w) * other.g4_ + vec3(self.g1_.x) * vec3(other.g0_.x, other.g4_.z, -other.g4_.y) + vec3(self.g1_.y) * vec3(-other.g4_.z, other.g0_.x, other.g4_.x) + vec3(self.g1_.z) * vec3(other.g4_.y, -other.g4_.x, other.g0_.x) + vec3(self.g1_.w) * other.g3_, vec2(self.g0_.x) * vec2(0.0, other.g9_.x) + vec2(self.g0_.x) * vec2(0.0, -other.g4_.x) + vec2(self.g0_.y) * vec2(0.0, other.g9_.y) + vec2(self.g0_.y) * vec2(0.0, -other.g4_.y) + vec2(self.g0_.z) * vec2(0.0, other.g9_.z) + vec2(self.g0_.z) * vec2(0.0, -other.g4_.z) + vec2(self.g0_.w) * other.g10_ * vec2(-1.0, 1.0) + vec2(self.g1_.x) * vec2(other.g3_.x, -other.g5_.x) + vec2(self.g1_.y) * vec2(other.g3_.y, -other.g5_.y) + vec2(self.g1_.z) * vec2(other.g3_.z, -other.g5_.z) + vec2(self.g1_.w) * vec2(0.0, -other.g5_.w) + vec2(self.g1_.w) * vec2(0.0, other.g0_.x));
}

MultiVector flector_wedgeDot_plane(Flector self, Plane other) {
    return MultiVector(vec2(self.g1_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g1_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g1_.z) * vec2(-other.g0_.z, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(self.g0_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g1_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g1_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g1_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec4(self.g0_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g0_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g0_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0) + self.g1_.xyzx * vec4(other.g0_.w, other.g0_.w, other.g0_.w, 0.0) + vec4(self.g1_.w) * vec4(-other.g0_.x, -other.g0_.y, -other.g0_.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(self.g0_.x) * vec2(0.0, other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, other.g0_.z) + vec2(self.g0_.w) * vec2(0.0, other.g0_.w));
}

MultiVector flector_wedgeDot_roundPoint(Flector self, RoundPoint other) {
    return MultiVector(vec2(self.g1_.x) * vec2(0.0, other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, other.g0_.z) + vec2(self.g1_.w) * vec2(0.0, other.g1_.x), vec3(0.0) - vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g1_.x), vec2(self.g0_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g0_.w) * other.g1_ * vec2(-1.0, 1.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0) - self.g1_ * vec4(other.g1_.x), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g1_.x) - vec3(self.g0_.w) * other.g0_ + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + vec3(self.g1_.x, self.g1_.y, self.g1_.z) * vec3(other.g1_.y) + vec3(self.g1_.w) * other.g0_, vec3(0.0), vec2(0.0));
}

Flector flector_wedgeDot_scalar(Flector self, Scalar other) {
    return Flector(self.g0_ * vec4(other.g0_), self.g1_ * vec4(other.g0_));
}

MultiVector flector_wedgeDot_sphere(Flector self, Sphere other) {
    return MultiVector(vec2(self.g1_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g1_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g1_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g1_.w) * vec2(other.g1_.x, 0.0), vec3(0.0), vec2(0.0), vec3(0.0) - vec3(self.g1_.x, self.g1_.y, self.g1_.z) * vec3(other.g1_.x), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g1_.x) + vec3(self.g0_.w) * other.g0_ + vec3(self.g1_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g1_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g1_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec4(self.g0_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g0_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g0_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0) + self.g1_.xyzx * vec4(other.g1_.y, other.g1_.y, other.g1_.y, 0.0) + vec4(self.g1_.w) * vec4(-other.g0_.x, -other.g0_.y, -other.g0_.z, other.g1_.x), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g1_.x), vec2(self.g0_.x) * vec2(0.0, other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, other.g0_.z) + vec2(self.g0_.w) * other.g1_ * vec2(-1.0, 1.0));
}

Dipole line_wedgeDot_antiScalar(Line self, AntiScalar other) {
    return Dipole(vec3(0.0), vec3(0.0) - self.g0_ * vec3(other.g0_), vec4(self.g1_.x, self.g1_.y, self.g1_.z, self.g1_.x) * vec4(-other.g0_, -other.g0_, -other.g0_, 0.0));
}

MultiVector line_wedgeDot_circle(Line self, Circle other) {
    return MultiVector(vec2(self.g0_.x) * vec2(other.g1_.x, 0.0) + vec2(self.g0_.y) * vec2(other.g1_.y, 0.0) + vec2(self.g0_.z) * vec2(other.g1_.z, 0.0) + vec2(self.g1_.x) * vec2(other.g0_.x, 0.0) + vec2(self.g1_.y) * vec2(other.g0_.y, 0.0) + vec2(self.g1_.z) * vec2(other.g0_.z, 0.0), vec3(0.0), vec2(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(self.g0_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec4(self.g0_.x) * vec4(0.0, -other.g2_.z, other.g2_.y, 0.0) + vec4(self.g0_.y) * vec4(other.g2_.z, 0.0, -other.g2_.x, 0.0) + vec4(self.g0_.z) * vec4(-other.g2_.y, other.g2_.x, 0.0, 0.0) + vec4(self.g1_.x) * vec4(-other.g0_.w, -other.g1_.z, other.g1_.y, other.g0_.x) + vec4(self.g1_.y) * vec4(other.g1_.z, -other.g0_.w, -other.g1_.x, other.g0_.y) + vec4(self.g1_.z) * vec4(-other.g1_.y, other.g1_.x, -other.g0_.w, other.g0_.z), vec4(0.0), vec3(0.0), vec3(0.0), self.g0_ * vec3(other.g0_.w) + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec2(self.g0_.x) * vec2(other.g0_.x, other.g2_.x) + vec2(self.g0_.y) * vec2(other.g0_.y, other.g2_.y) + vec2(self.g0_.z) * vec2(other.g0_.z, other.g2_.z) + vec2(self.g1_.x) * vec2(0.0, other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, other.g1_.z));
}

MultiVector line_wedgeDot_dipole(Line self, Dipole other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g1_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g0_.z), vec3(0.0) - self.g0_ * vec3(other.g2_.w) + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec2(0.0) - vec2(self.g0_.x) * vec2(other.g0_.x, other.g2_.x) - vec2(self.g0_.y) * vec2(other.g0_.y, other.g2_.y) - vec2(self.g0_.z) * vec2(other.g0_.z, other.g2_.z) + vec2(self.g1_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g1_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(0.0, other.g0_.z, -other.g0_.y, 0.0) + vec4(self.g0_.y) * vec4(-other.g0_.z, 0.0, other.g0_.x, 0.0) + vec4(self.g0_.z) * vec4(other.g0_.y, -other.g0_.x, 0.0, 0.0) + vec4(self.g1_.x) * vec4(0.0, 0.0, 0.0, other.g0_.x) + vec4(self.g1_.y) * vec4(0.0, 0.0, 0.0, other.g0_.y) + vec4(self.g1_.z) * vec4(0.0, 0.0, 0.0, other.g0_.z), vec3(self.g0_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g1_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g1_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g1_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec3(self.g0_.x) * vec3(0.0, other.g2_.z, -other.g2_.y) + vec3(self.g0_.y) * vec3(-other.g2_.z, 0.0, other.g2_.x) + vec3(self.g0_.z) * vec3(other.g2_.y, -other.g2_.x, 0.0) + vec3(self.g1_.x) * vec3(-other.g2_.w, other.g1_.z, -other.g1_.y) + vec3(self.g1_.y) * vec3(-other.g1_.z, -other.g2_.w, other.g1_.x) + vec3(self.g1_.z) * vec3(other.g1_.y, -other.g1_.x, -other.g2_.w), vec3(0.0), vec2(0.0));
}

MultiVector line_wedgeDot_dualNum(Line self, DualNum other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0) - self.g0_ * vec3(other.g0_.y), vec4(self.g1_.x, self.g1_.y, self.g1_.z, self.g1_.x) * vec4(-other.g0_.y, -other.g0_.y, -other.g0_.y, 0.0), vec4(0.0), self.g0_ * vec3(other.g0_.x), self.g1_ * vec3(other.g0_.x), vec3(0.0), vec2(0.0));
}

MultiVector line_wedgeDot_flatPoint(Line self, FlatPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0) - self.g0_ * vec3(other.g0_.w), vec2(self.g0_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g0_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) - self.g1_ * vec3(other.g0_.w), vec3(0.0), vec2(0.0));
}

MultiVector line_wedgeDot_flector(Line self, Flector other) {
    return MultiVector(vec2(0.0), vec3(self.g0_.x) * vec3(-other.g0_.w, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, -other.g0_.w, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, -other.g0_.w), vec2(self.g0_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g1_.x) * vec2(0.0, other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, other.g1_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, other.g1_.x) + vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, other.g1_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, 0.0, other.g1_.z), vec3(0.0), vec3(self.g0_.x) * vec3(other.g1_.w, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, other.g1_.w, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, other.g1_.w) + vec3(self.g1_.x) * vec3(-other.g0_.w, -other.g1_.z, other.g1_.y) + vec3(self.g1_.y) * vec3(other.g1_.z, -other.g0_.w, -other.g1_.x) + vec3(self.g1_.z) * vec3(-other.g1_.y, other.g1_.x, -other.g0_.w), vec3(0.0), vec2(0.0));
}

MultiVector line_wedgeDot_line(Line self, Line other) {
    return MultiVector(vec2(self.g0_.x) * vec2(other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(other.g0_.z, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec4(self.g0_.x) * vec4(0.0, -other.g1_.z, other.g1_.y, 0.0) + vec4(self.g0_.y) * vec4(other.g1_.z, 0.0, -other.g1_.x, 0.0) + vec4(self.g0_.z) * vec4(-other.g1_.y, other.g1_.x, 0.0, 0.0) + vec4(self.g1_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g1_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g1_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(self.g0_.x) * vec2(0.0, other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, other.g1_.z) + vec2(self.g1_.x) * vec2(0.0, other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, other.g0_.z));
}

MultiVector line_wedgeDot_motor(Line self, Motor other) {
    return MultiVector(vec2(self.g0_.x) * vec2(other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(other.g0_.z, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(self.g0_.x) * vec3(-other.g0_.w, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, -other.g0_.w, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, -other.g0_.w), vec4(self.g0_.x) * vec4(0.0, -other.g1_.z, other.g1_.y, 0.0) + vec4(self.g0_.y) * vec4(other.g1_.z, 0.0, -other.g1_.x, 0.0) + vec4(self.g0_.z) * vec4(-other.g1_.y, other.g1_.x, 0.0, 0.0) + vec4(self.g1_.x) * vec4(-other.g0_.w, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g1_.y) * vec4(other.g0_.z, -other.g0_.w, -other.g0_.x, 0.0) + vec4(self.g1_.z) * vec4(-other.g0_.y, other.g0_.x, -other.g0_.w, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(self.g0_.x) * vec2(0.0, other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, other.g1_.z) + vec2(self.g1_.x) * vec2(0.0, other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, other.g0_.z));
}

MultiVector line_wedgeDot_multiVector(Line self, MultiVector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(other.g7_.x, -other.g4_.x) + vec2(self.g0_.y) * vec2(other.g7_.y, -other.g4_.y) + vec2(self.g0_.z) * vec2(other.g7_.z, -other.g4_.z) + vec2(self.g1_.x) * vec2(other.g6_.x, -other.g3_.x) + vec2(self.g1_.y) * vec2(other.g6_.y, -other.g3_.y) + vec2(self.g1_.z) * vec2(other.g6_.z, -other.g3_.z), vec3(self.g0_.x) * vec3(-other.g5_.w, -other.g9_.z, other.g9_.y) + vec3(self.g0_.y) * vec3(other.g9_.z, -other.g5_.w, -other.g9_.x) + vec3(self.g0_.z) * vec3(-other.g9_.y, other.g9_.x, -other.g5_.w) + vec3(self.g1_.x) * vec3(-other.g10_.x, -other.g3_.z, other.g3_.y) + vec3(self.g1_.y) * vec3(other.g3_.z, -other.g10_.x, -other.g3_.x) + vec3(self.g1_.z) * vec3(-other.g3_.y, other.g3_.x, -other.g10_.x), vec2(0.0) - vec2(self.g0_.x) * vec2(other.g3_.x, other.g5_.x) - vec2(self.g0_.y) * vec2(other.g3_.y, other.g5_.y) - vec2(self.g0_.z) * vec2(other.g3_.z, other.g5_.z) + vec2(self.g1_.x) * vec2(0.0, other.g9_.x) + vec2(self.g1_.x) * vec2(0.0, -other.g4_.x) + vec2(self.g1_.y) * vec2(0.0, other.g9_.y) + vec2(self.g1_.y) * vec2(0.0, -other.g4_.y) + vec2(self.g1_.z) * vec2(0.0, other.g9_.z) + vec2(self.g1_.z) * vec2(0.0, -other.g4_.z), vec3(self.g0_.x) * vec3(-other.g2_.x, -other.g6_.z, other.g6_.y) + vec3(self.g0_.y) * vec3(other.g6_.z, -other.g2_.x, -other.g6_.x) + vec3(self.g0_.z) * vec3(-other.g6_.y, other.g6_.x, -other.g2_.x), vec3(self.g0_.x) * vec3(-other.g0_.y, -other.g7_.z, other.g7_.y) + vec3(self.g0_.y) * vec3(other.g7_.z, -other.g0_.y, -other.g7_.x) + vec3(self.g0_.z) * vec3(-other.g7_.y, other.g7_.x, -other.g0_.y) + vec3(self.g1_.x) * vec3(-other.g2_.x, -other.g6_.z, other.g6_.y) + vec3(self.g1_.y) * vec3(other.g6_.z, -other.g2_.x, -other.g6_.x) + vec3(self.g1_.z) * vec3(-other.g6_.y, other.g6_.x, -other.g2_.x), vec4(self.g0_.x) * vec4(-other.g2_.y, -other.g8_.z, other.g8_.y, -other.g1_.x) + vec4(self.g0_.y) * vec4(other.g8_.z, -other.g2_.y, -other.g8_.x, -other.g1_.y) + vec4(self.g0_.z) * vec4(-other.g8_.y, other.g8_.x, -other.g2_.y, -other.g1_.z) + vec4(self.g1_.x) * vec4(-other.g6_.w, -other.g7_.z, other.g7_.y, other.g6_.x) + vec4(self.g1_.x) * vec4(-other.g0_.y, -other.g1_.z, other.g1_.y, 0.0) + vec4(self.g1_.y) * vec4(other.g7_.z, -other.g6_.w, -other.g7_.x, other.g6_.y) + vec4(self.g1_.y) * vec4(other.g1_.z, -other.g0_.y, -other.g1_.x, 0.0) + vec4(self.g1_.z) * vec4(-other.g7_.y, other.g7_.x, -other.g6_.w, other.g6_.z) + vec4(self.g1_.z) * vec4(-other.g1_.y, other.g1_.x, -other.g0_.y, 0.0), vec4(self.g0_.x) * vec4(other.g10_.x, other.g3_.z, -other.g3_.y, other.g9_.x) + vec4(self.g0_.y) * vec4(-other.g3_.z, other.g10_.x, other.g3_.x, other.g9_.y) + vec4(self.g0_.z) * vec4(other.g3_.y, -other.g3_.x, other.g10_.x, other.g9_.z) + vec4(self.g1_.x) * vec4(0.0, 0.0, 0.0, other.g3_.x) + vec4(self.g1_.y) * vec4(0.0, 0.0, 0.0, other.g3_.y) + vec4(self.g1_.z) * vec4(0.0, 0.0, 0.0, other.g3_.z), vec3(self.g0_.x) * vec3(other.g0_.x, other.g4_.z, -other.g4_.y) + vec3(self.g0_.y) * vec3(-other.g4_.z, other.g0_.x, other.g4_.x) + vec3(self.g0_.z) * vec3(other.g4_.y, -other.g4_.x, other.g0_.x) + vec3(self.g1_.x) * vec3(other.g10_.x, other.g3_.z, -other.g3_.y) + vec3(self.g1_.y) * vec3(-other.g3_.z, other.g10_.x, other.g3_.x) + vec3(self.g1_.z) * vec3(other.g3_.y, -other.g3_.x, other.g10_.x), vec3(self.g0_.x) * vec3(other.g10_.y, other.g5_.z, -other.g5_.y) + vec3(self.g0_.y) * vec3(-other.g5_.z, other.g10_.y, other.g5_.x) + vec3(self.g0_.z) * vec3(other.g5_.y, -other.g5_.x, other.g10_.y) + vec3(self.g1_.x) * vec3(-other.g5_.w, -other.g9_.z, other.g9_.y) + vec3(self.g1_.x) * vec3(other.g0_.x, other.g4_.z, -other.g4_.y) + vec3(self.g1_.y) * vec3(other.g9_.z, -other.g5_.w, -other.g9_.x) + vec3(self.g1_.y) * vec3(-other.g4_.z, other.g0_.x, other.g4_.x) + vec3(self.g1_.z) * vec3(-other.g9_.y, other.g9_.x, -other.g5_.w) + vec3(self.g1_.z) * vec3(other.g4_.y, -other.g4_.x, other.g0_.x), vec3(self.g0_.x) * vec3(other.g6_.w, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, other.g6_.w, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, other.g6_.w) + vec3(self.g1_.x) * vec3(-other.g2_.x, -other.g6_.z, other.g6_.y) + vec3(self.g1_.y) * vec3(other.g6_.z, -other.g2_.x, -other.g6_.x) + vec3(self.g1_.z) * vec3(-other.g6_.y, other.g6_.x, -other.g2_.x), vec2(self.g0_.x) * vec2(other.g6_.x, other.g8_.x) + vec2(self.g0_.y) * vec2(other.g6_.y, other.g8_.y) + vec2(self.g0_.z) * vec2(other.g6_.z, other.g8_.z) + vec2(self.g1_.x) * vec2(0.0, other.g7_.x) + vec2(self.g1_.x) * vec2(0.0, other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, other.g7_.y) + vec2(self.g1_.y) * vec2(0.0, other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, other.g7_.z) + vec2(self.g1_.z) * vec2(0.0, other.g1_.z));
}

MultiVector line_wedgeDot_plane(Line self, Plane other) {
    return MultiVector(vec2(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec2(self.g1_.x) * vec2(0.0, other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, other.g0_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, other.g0_.x) + vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, other.g0_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, 0.0, other.g0_.z), vec3(0.0), self.g0_ * vec3(other.g0_.w) + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(0.0), vec2(0.0));
}

MultiVector line_wedgeDot_roundPoint(Line self, RoundPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0) - self.g0_ * vec3(other.g1_.x), vec3(0.0) - self.g1_ * vec3(other.g1_.x), vec4(self.g0_.x) * vec4(-other.g1_.y, 0.0, 0.0, -other.g0_.x) + vec4(self.g0_.y) * vec4(0.0, -other.g1_.y, 0.0, -other.g0_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, -other.g1_.y, -other.g0_.z) + vec4(self.g1_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g1_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g1_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) - self.g1_ * vec3(other.g1_.x), vec2(self.g1_.x) * vec2(0.0, other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, other.g0_.z));
}

Line line_wedgeDot_scalar(Line self, Scalar other) {
    return Line(self.g0_ * vec3(other.g0_), self.g1_ * vec3(other.g0_));
}

MultiVector line_wedgeDot_sphere(Line self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) - self.g1_ * vec3(other.g1_.x), vec2(self.g1_.x) * vec2(0.0, other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, other.g0_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(other.g1_.x, 0.0, 0.0, other.g0_.x) + vec4(self.g0_.y) * vec4(0.0, other.g1_.x, 0.0, other.g0_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, other.g1_.x, other.g0_.z), self.g1_ * vec3(other.g1_.x), self.g0_ * vec3(other.g1_.y) + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(0.0), vec2(0.0));
}

MultiVector motor_wedgeDot_antiScalar(Motor self, AntiScalar other) {
    return MultiVector(vec2(self.g0_.w) * vec2(-other.g0_, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0) - vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_), vec4(self.g1_.x, self.g1_.y, self.g1_.z, self.g1_.x) * vec4(-other.g0_, -other.g0_, -other.g0_, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector motor_wedgeDot_circle(Motor self, Circle other) {
    return MultiVector(vec2(self.g0_.x) * vec2(other.g1_.x, 0.0) + vec2(self.g0_.y) * vec2(other.g1_.y, 0.0) + vec2(self.g0_.z) * vec2(other.g1_.z, 0.0) + vec2(self.g1_.x) * vec2(other.g0_.x, 0.0) + vec2(self.g1_.y) * vec2(other.g0_.y, 0.0) + vec2(self.g1_.z) * vec2(other.g0_.z, 0.0), vec3(0.0), vec2(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) - vec3(self.g0_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(self.g0_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) - vec3(self.g0_.w) * other.g1_ + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec4(self.g0_.x) * vec4(0.0, -other.g2_.z, other.g2_.y, 0.0) + vec4(self.g0_.y) * vec4(other.g2_.z, 0.0, -other.g2_.x, 0.0) + vec4(self.g0_.z) * vec4(-other.g2_.y, other.g2_.x, 0.0, 0.0) + vec4(self.g0_.w) * vec4(-other.g2_.x, -other.g2_.y, -other.g2_.z, other.g0_.w) + vec4(self.g1_.x) * vec4(-other.g0_.w, -other.g1_.z, other.g1_.y, other.g0_.x) + vec4(self.g1_.y) * vec4(other.g1_.z, -other.g0_.w, -other.g1_.x, other.g0_.y) + vec4(self.g1_.z) * vec4(-other.g1_.y, other.g1_.x, -other.g0_.w, other.g0_.z), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_.w) + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec2(self.g0_.x) * vec2(other.g0_.x, other.g2_.x) + vec2(self.g0_.y) * vec2(other.g0_.y, other.g2_.y) + vec2(self.g0_.z) * vec2(other.g0_.z, other.g2_.z) + vec2(self.g1_.x) * vec2(0.0, other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, other.g1_.z));
}

MultiVector motor_wedgeDot_dipole(Motor self, Dipole other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g1_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g0_.z), vec3(0.0) - vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g2_.w) + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec2(0.0) - vec2(self.g0_.x) * vec2(other.g0_.x, other.g2_.x) - vec2(self.g0_.y) * vec2(other.g0_.y, other.g2_.y) - vec2(self.g0_.z) * vec2(other.g0_.z, other.g2_.z) + vec2(self.g1_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g1_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(0.0, other.g0_.z, -other.g0_.y, 0.0) + vec4(self.g0_.y) * vec4(-other.g0_.z, 0.0, other.g0_.x, 0.0) + vec4(self.g0_.z) * vec4(other.g0_.y, -other.g0_.x, 0.0, 0.0) + vec4(self.g0_.w) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, -other.g2_.w) + vec4(self.g1_.x) * vec4(0.0, 0.0, 0.0, other.g0_.x) + vec4(self.g1_.y) * vec4(0.0, 0.0, 0.0, other.g0_.y) + vec4(self.g1_.z) * vec4(0.0, 0.0, 0.0, other.g0_.z), vec3(self.g0_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g0_.w) * other.g1_ + vec3(self.g1_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g1_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g1_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec3(self.g0_.x) * vec3(0.0, other.g2_.z, -other.g2_.y) + vec3(self.g0_.y) * vec3(-other.g2_.z, 0.0, other.g2_.x) + vec3(self.g0_.z) * vec3(other.g2_.y, -other.g2_.x, 0.0) + vec3(self.g0_.w) * vec3(other.g2_.x, other.g2_.y, other.g2_.z) + vec3(self.g1_.x) * vec3(-other.g2_.w, other.g1_.z, -other.g1_.y) + vec3(self.g1_.y) * vec3(-other.g1_.z, -other.g2_.w, other.g1_.x) + vec3(self.g1_.z) * vec3(other.g1_.y, -other.g1_.x, -other.g2_.w), vec3(0.0), vec2(0.0));
}

MultiVector motor_wedgeDot_dualNum(Motor self, DualNum other) {
    return MultiVector(vec2(self.g0_.w) * other.g0_.yx * vec2(-1.0, 1.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0) - vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_.y), vec4(self.g1_.x, self.g1_.y, self.g1_.z, self.g1_.x) * vec4(-other.g0_.y, -other.g0_.y, -other.g0_.y, 0.0), vec4(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_.x), self.g1_ * vec3(other.g0_.x), vec3(0.0), vec2(0.0));
}

MultiVector motor_wedgeDot_flatPoint(Motor self, FlatPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0) - vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_.w), vec2(self.g0_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g0_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.w) * vec4(0.0, 0.0, 0.0, -other.g0_.w), vec3(0.0), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + vec3(self.g0_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) - self.g1_ * vec3(other.g0_.w), vec3(0.0), vec2(0.0));
}

MultiVector motor_wedgeDot_flector(Motor self, Flector other) {
    return MultiVector(vec2(0.0), vec3(self.g0_.x) * vec3(-other.g0_.w, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, -other.g0_.w, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, -other.g0_.w) - vec3(self.g0_.w) * vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec2(self.g0_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g0_.w) * vec2(0.0, other.g1_.w) + vec2(self.g1_.x) * vec2(0.0, other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, other.g1_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, other.g1_.x) + vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, other.g1_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, 0.0, other.g1_.z) + vec4(self.g0_.w) * vec4(0.0, 0.0, 0.0, -other.g0_.w), vec3(0.0), vec3(self.g0_.x) * vec3(other.g1_.w, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, other.g1_.w, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, other.g1_.w) + vec3(self.g0_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g1_.x) * vec3(-other.g0_.w, -other.g1_.z, other.g1_.y) + vec3(self.g1_.y) * vec3(other.g1_.z, -other.g0_.w, -other.g1_.x) + vec3(self.g1_.z) * vec3(-other.g1_.y, other.g1_.x, -other.g0_.w), vec3(0.0), vec2(0.0));
}

MultiVector motor_wedgeDot_line(Motor self, Line other) {
    return MultiVector(vec2(self.g0_.x) * vec2(other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(other.g0_.z, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) - vec3(self.g0_.w) * other.g0_, vec4(self.g0_.x) * vec4(0.0, -other.g1_.z, other.g1_.y, 0.0) + vec4(self.g0_.y) * vec4(other.g1_.z, 0.0, -other.g1_.x, 0.0) + vec4(self.g0_.z) * vec4(-other.g1_.y, other.g1_.x, 0.0, 0.0) + vec4(self.g0_.w) * vec4(-other.g1_.x, -other.g1_.y, -other.g1_.z, 0.0) + vec4(self.g1_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g1_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g1_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(self.g0_.x) * vec2(0.0, other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, other.g1_.z) + vec2(self.g1_.x) * vec2(0.0, other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, other.g0_.z));
}

MultiVector motor_wedgeDot_motor(Motor self, Motor other) {
    return MultiVector(vec2(self.g0_.x) * vec2(other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(other.g0_.z, 0.0) + vec2(self.g0_.w) * vec2(-other.g0_.w, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(self.g0_.x) * vec3(-other.g0_.w, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, -other.g0_.w, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, -other.g0_.w) - vec3(self.g0_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec4(self.g0_.x) * vec4(0.0, -other.g1_.z, other.g1_.y, 0.0) + vec4(self.g0_.y) * vec4(other.g1_.z, 0.0, -other.g1_.x, 0.0) + vec4(self.g0_.z) * vec4(-other.g1_.y, other.g1_.x, 0.0, 0.0) + vec4(self.g0_.w) * vec4(-other.g1_.x, -other.g1_.y, -other.g1_.z, 0.0) + vec4(self.g1_.x) * vec4(-other.g0_.w, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g1_.y) * vec4(other.g0_.z, -other.g0_.w, -other.g0_.x, 0.0) + vec4(self.g1_.z) * vec4(-other.g0_.y, other.g0_.x, -other.g0_.w, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(self.g0_.x) * vec2(0.0, other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, other.g1_.z) + vec2(self.g1_.x) * vec2(0.0, other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, other.g0_.z));
}

MultiVector motor_wedgeDot_multiVector(Motor self, MultiVector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(other.g7_.x, -other.g4_.x) + vec2(self.g0_.y) * vec2(other.g7_.y, -other.g4_.y) + vec2(self.g0_.z) * vec2(other.g7_.z, -other.g4_.z) + vec2(self.g0_.w) * other.g0_.yx * vec2(-1.0, 1.0) + vec2(self.g1_.x) * vec2(other.g6_.x, -other.g3_.x) + vec2(self.g1_.y) * vec2(other.g6_.y, -other.g3_.y) + vec2(self.g1_.z) * vec2(other.g6_.z, -other.g3_.z), vec3(self.g0_.x) * vec3(-other.g5_.w, -other.g9_.z, other.g9_.y) + vec3(self.g0_.y) * vec3(other.g9_.z, -other.g5_.w, -other.g9_.x) + vec3(self.g0_.z) * vec3(-other.g9_.y, other.g9_.x, -other.g5_.w) - vec3(self.g0_.w) * other.g9_ + vec3(self.g1_.x) * vec3(-other.g10_.x, -other.g3_.z, other.g3_.y) + vec3(self.g1_.y) * vec3(other.g3_.z, -other.g10_.x, -other.g3_.x) + vec3(self.g1_.z) * vec3(-other.g3_.y, other.g3_.x, -other.g10_.x), vec2(0.0) - vec2(self.g0_.x) * vec2(other.g3_.x, other.g5_.x) - vec2(self.g0_.y) * vec2(other.g3_.y, other.g5_.y) - vec2(self.g0_.z) * vec2(other.g3_.z, other.g5_.z) + vec2(self.g0_.w) * other.g10_ + vec2(self.g1_.x) * vec2(0.0, other.g9_.x) + vec2(self.g1_.x) * vec2(0.0, -other.g4_.x) + vec2(self.g1_.y) * vec2(0.0, other.g9_.y) + vec2(self.g1_.y) * vec2(0.0, -other.g4_.y) + vec2(self.g1_.z) * vec2(0.0, other.g9_.z) + vec2(self.g1_.z) * vec2(0.0, -other.g4_.z), vec3(self.g0_.x) * vec3(-other.g2_.x, -other.g6_.z, other.g6_.y) + vec3(self.g0_.y) * vec3(other.g6_.z, -other.g2_.x, -other.g6_.x) + vec3(self.g0_.z) * vec3(-other.g6_.y, other.g6_.x, -other.g2_.x) - vec3(self.g0_.w) * vec3(other.g6_.x, other.g6_.y, other.g6_.z), vec3(self.g0_.x) * vec3(-other.g0_.y, -other.g7_.z, other.g7_.y) + vec3(self.g0_.y) * vec3(other.g7_.z, -other.g0_.y, -other.g7_.x) + vec3(self.g0_.z) * vec3(-other.g7_.y, other.g7_.x, -other.g0_.y) - vec3(self.g0_.w) * other.g7_ + vec3(self.g1_.x) * vec3(-other.g2_.x, -other.g6_.z, other.g6_.y) + vec3(self.g1_.y) * vec3(other.g6_.z, -other.g2_.x, -other.g6_.x) + vec3(self.g1_.z) * vec3(-other.g6_.y, other.g6_.x, -other.g2_.x), vec4(self.g0_.x) * vec4(-other.g2_.y, -other.g8_.z, other.g8_.y, -other.g1_.x) + vec4(self.g0_.y) * vec4(other.g8_.z, -other.g2_.y, -other.g8_.x, -other.g1_.y) + vec4(self.g0_.z) * vec4(-other.g8_.y, other.g8_.x, -other.g2_.y, -other.g1_.z) + vec4(self.g0_.w) * vec4(-other.g8_.x, -other.g8_.y, -other.g8_.z, other.g6_.w) + vec4(self.g1_.x) * vec4(-other.g6_.w, -other.g7_.z, other.g7_.y, other.g6_.x) + vec4(self.g1_.x) * vec4(-other.g0_.y, -other.g1_.z, other.g1_.y, 0.0) + vec4(self.g1_.y) * vec4(other.g7_.z, -other.g6_.w, -other.g7_.x, other.g6_.y) + vec4(self.g1_.y) * vec4(other.g1_.z, -other.g0_.y, -other.g1_.x, 0.0) + vec4(self.g1_.z) * vec4(-other.g7_.y, other.g7_.x, -other.g6_.w, other.g6_.z) + vec4(self.g1_.z) * vec4(-other.g1_.y, other.g1_.x, -other.g0_.y, 0.0), vec4(self.g0_.x) * vec4(other.g10_.x, other.g3_.z, -other.g3_.y, other.g9_.x) + vec4(self.g0_.y) * vec4(-other.g3_.z, other.g10_.x, other.g3_.x, other.g9_.y) + vec4(self.g0_.z) * vec4(other.g3_.y, -other.g3_.x, other.g10_.x, other.g9_.z) + vec4(self.g0_.w) * vec4(other.g3_.x, other.g3_.y, other.g3_.z, -other.g5_.w) + vec4(self.g1_.x) * vec4(0.0, 0.0, 0.0, other.g3_.x) + vec4(self.g1_.y) * vec4(0.0, 0.0, 0.0, other.g3_.y) + vec4(self.g1_.z) * vec4(0.0, 0.0, 0.0, other.g3_.z), vec3(self.g0_.x) * vec3(other.g0_.x, other.g4_.z, -other.g4_.y) + vec3(self.g0_.y) * vec3(-other.g4_.z, other.g0_.x, other.g4_.x) + vec3(self.g0_.z) * vec3(other.g4_.y, -other.g4_.x, other.g0_.x) + vec3(self.g0_.w) * other.g4_ + vec3(self.g1_.x) * vec3(other.g10_.x, other.g3_.z, -other.g3_.y) + vec3(self.g1_.y) * vec3(-other.g3_.z, other.g10_.x, other.g3_.x) + vec3(self.g1_.z) * vec3(other.g3_.y, -other.g3_.x, other.g10_.x), vec3(self.g0_.x) * vec3(other.g10_.y, other.g5_.z, -other.g5_.y) + vec3(self.g0_.y) * vec3(-other.g5_.z, other.g10_.y, other.g5_.x) + vec3(self.g0_.z) * vec3(other.g5_.y, -other.g5_.x, other.g10_.y) + vec3(self.g0_.w) * vec3(other.g5_.x, other.g5_.y, other.g5_.z) + vec3(self.g1_.x) * vec3(-other.g5_.w, -other.g9_.z, other.g9_.y) + vec3(self.g1_.x) * vec3(other.g0_.x, other.g4_.z, -other.g4_.y) + vec3(self.g1_.y) * vec3(other.g9_.z, -other.g5_.w, -other.g9_.x) + vec3(self.g1_.y) * vec3(-other.g4_.z, other.g0_.x, other.g4_.x) + vec3(self.g1_.z) * vec3(-other.g9_.y, other.g9_.x, -other.g5_.w) + vec3(self.g1_.z) * vec3(other.g4_.y, -other.g4_.x, other.g0_.x), vec3(self.g0_.x) * vec3(other.g6_.w, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, other.g6_.w, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, other.g6_.w) + vec3(self.g0_.w) * other.g1_ + vec3(self.g1_.x) * vec3(-other.g2_.x, -other.g6_.z, other.g6_.y) + vec3(self.g1_.y) * vec3(other.g6_.z, -other.g2_.x, -other.g6_.x) + vec3(self.g1_.z) * vec3(-other.g6_.y, other.g6_.x, -other.g2_.x), vec2(self.g0_.x) * vec2(other.g6_.x, other.g8_.x) + vec2(self.g0_.y) * vec2(other.g6_.y, other.g8_.y) + vec2(self.g0_.z) * vec2(other.g6_.z, other.g8_.z) - vec2(self.g0_.w) * other.g2_ + vec2(self.g1_.x) * vec2(0.0, other.g7_.x) + vec2(self.g1_.x) * vec2(0.0, other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, other.g7_.y) + vec2(self.g1_.y) * vec2(0.0, other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, other.g7_.z) + vec2(self.g1_.z) * vec2(0.0, other.g1_.z));
}

MultiVector motor_wedgeDot_plane(Motor self, Plane other) {
    return MultiVector(vec2(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) - vec3(self.g0_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g0_.w) * vec2(0.0, other.g0_.w) + vec2(self.g1_.x) * vec2(0.0, other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, other.g0_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, other.g0_.x) + vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, other.g0_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, 0.0, other.g0_.z), vec3(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_.w) + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(0.0), vec2(0.0));
}

MultiVector motor_wedgeDot_roundPoint(Motor self, RoundPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0) - vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g1_.x), vec3(0.0) - self.g1_ * vec3(other.g1_.x), vec4(self.g0_.x) * vec4(-other.g1_.y, 0.0, 0.0, -other.g0_.x) + vec4(self.g0_.y) * vec4(0.0, -other.g1_.y, 0.0, -other.g0_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, -other.g1_.y, -other.g0_.z) + vec4(self.g1_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g1_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g1_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + vec3(self.g0_.w) * other.g0_ - self.g1_ * vec3(other.g1_.x), vec2(0.0) - vec2(self.g0_.w) * other.g1_ + vec2(self.g1_.x) * vec2(0.0, other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, other.g0_.z));
}

Motor motor_wedgeDot_scalar(Motor self, Scalar other) {
    return Motor(self.g0_ * vec4(other.g0_), self.g1_ * vec3(other.g0_));
}

MultiVector motor_wedgeDot_sphere(Motor self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) - vec3(self.g0_.w) * other.g0_ - self.g1_ * vec3(other.g1_.x), vec2(self.g0_.w) * other.g1_ + vec2(self.g1_.x) * vec2(0.0, other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, other.g0_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(other.g1_.x, 0.0, 0.0, other.g0_.x) + vec4(self.g0_.y) * vec4(0.0, other.g1_.x, 0.0, other.g0_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, other.g1_.x, other.g0_.z), self.g1_ * vec3(other.g1_.x), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g1_.y) + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(0.0), vec2(0.0));
}

MultiVector multiVector_wedgeDot_antiScalar(MultiVector self, AntiScalar other) {
    return MultiVector(self.g0_.yx * vec2(-other.g0_), vec3(0.0) - self.g9_ * vec3(other.g0_), self.g10_ * vec2(other.g0_), vec3(0.0) - vec3(self.g6_.x, self.g6_.y, self.g6_.z) * vec3(other.g0_), vec3(0.0) - self.g7_ * vec3(other.g0_), vec4(self.g6_.w) * vec4(0.0, 0.0, 0.0, other.g0_) + vec4(self.g8_.x, self.g8_.y, self.g8_.z, self.g8_.x) * vec4(-other.g0_, -other.g0_, -other.g0_, 0.0), vec4(self.g3_.x, self.g3_.y, self.g3_.z, self.g3_.x) * vec4(other.g0_, other.g0_, other.g0_, 0.0) + vec4(self.g5_.w) * vec4(0.0, 0.0, 0.0, -other.g0_), self.g4_ * vec3(other.g0_), vec3(self.g5_.x, self.g5_.y, self.g5_.z) * vec3(other.g0_), self.g1_ * vec3(other.g0_), vec2(0.0) - self.g2_ * vec2(other.g0_));
}

MultiVector multiVector_wedgeDot_circle(MultiVector self, Circle other) {
    return MultiVector(vec2(self.g3_.x) * vec2(0.0, -other.g2_.x) + vec2(self.g3_.y) * vec2(0.0, -other.g2_.y) + vec2(self.g3_.z) * vec2(0.0, -other.g2_.z) + vec2(self.g4_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g4_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g4_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g5_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g5_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g5_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g5_.w) * vec2(0.0, -other.g0_.w) + vec2(self.g6_.x) * vec2(other.g2_.x, 0.0) + vec2(self.g6_.y) * vec2(other.g2_.y, 0.0) + vec2(self.g6_.z) * vec2(other.g2_.z, 0.0) + vec2(self.g6_.w) * vec2(-other.g0_.w, 0.0) + vec2(self.g7_.x) * vec2(other.g1_.x, 0.0) + vec2(self.g7_.y) * vec2(other.g1_.y, 0.0) + vec2(self.g7_.z) * vec2(other.g1_.z, 0.0) + vec2(self.g8_.x) * vec2(other.g0_.x, 0.0) + vec2(self.g8_.y) * vec2(other.g0_.y, 0.0) + vec2(self.g8_.z) * vec2(other.g0_.z, 0.0), vec3(self.g3_.x) * vec3(0.0, other.g2_.z, -other.g2_.y) + vec3(self.g3_.y) * vec3(-other.g2_.z, 0.0, other.g2_.x) + vec3(self.g3_.z) * vec3(other.g2_.y, -other.g2_.x, 0.0) + self.g4_ * vec3(other.g0_.w) + vec3(self.g5_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g5_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g5_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) - vec3(self.g5_.w) * other.g1_ + vec3(self.g9_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g9_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g9_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g10_.x) * other.g2_ - vec3(self.g10_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g3_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g3_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g3_.z) * vec2(-other.g1_.z, 0.0) - vec2(self.g4_.x) * vec2(other.g0_.x, other.g2_.x) - vec2(self.g4_.y) * vec2(other.g0_.y, other.g2_.y) - vec2(self.g4_.z) * vec2(other.g0_.z, other.g2_.z) + vec2(self.g5_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g5_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g5_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g9_.x) * vec2(other.g0_.x, -other.g2_.x) + vec2(self.g9_.y) * vec2(other.g0_.y, -other.g2_.y) + vec2(self.g9_.z) * vec2(other.g0_.z, -other.g2_.z) + self.g10_ * vec2(-other.g0_.w), vec3(0.0) - vec3(self.g0_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) - vec3(self.g2_.x) * other.g1_ + vec3(self.g6_.x) * vec3(other.g0_.w, -other.g1_.z, other.g1_.y) + vec3(self.g6_.y) * vec3(other.g1_.z, other.g0_.w, -other.g1_.x) + vec3(self.g6_.z) * vec3(-other.g1_.y, other.g1_.x, other.g0_.w) - vec3(self.g6_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g7_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g7_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g7_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(0.0) - vec3(self.g0_.y) * other.g1_ - self.g1_ * vec3(other.g0_.w) - vec3(self.g2_.x) * other.g2_ - vec3(self.g2_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g6_.x) * vec3(0.0, -other.g2_.z, other.g2_.y) + vec3(self.g6_.y) * vec3(other.g2_.z, 0.0, -other.g2_.x) + vec3(self.g6_.z) * vec3(-other.g2_.y, other.g2_.x, 0.0) + vec3(self.g7_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g7_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g7_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g8_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g8_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g8_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec4(self.g0_.y) * vec4(-other.g2_.x, -other.g2_.y, -other.g2_.z, other.g0_.w) + vec4(self.g1_.x) * vec4(0.0, other.g2_.z, -other.g2_.y, -other.g1_.x) + vec4(self.g1_.y) * vec4(-other.g2_.z, 0.0, other.g2_.x, -other.g1_.y) + vec4(self.g1_.z) * vec4(other.g2_.y, -other.g2_.x, 0.0, -other.g1_.z) + vec4(self.g2_.y) * vec4(-other.g1_.x, -other.g1_.y, -other.g1_.z, 0.0) + vec4(self.g6_.x) * vec4(0.0, 0.0, 0.0, -other.g2_.x) + vec4(self.g6_.y) * vec4(0.0, 0.0, 0.0, -other.g2_.y) + self.g6_.wwwz * vec4(other.g2_.x, other.g2_.y, other.g2_.z, -other.g2_.z) + vec4(self.g7_.x) * vec4(0.0, -other.g2_.z, other.g2_.y, 0.0) + vec4(self.g7_.y) * vec4(other.g2_.z, 0.0, -other.g2_.x, 0.0) + vec4(self.g7_.z) * vec4(-other.g2_.y, other.g2_.x, 0.0, 0.0) + vec4(self.g8_.x) * vec4(-other.g0_.w, -other.g1_.z, other.g1_.y, other.g0_.x) + vec4(self.g8_.y) * vec4(other.g1_.z, -other.g0_.w, -other.g1_.x, other.g0_.y) + vec4(self.g8_.z) * vec4(-other.g1_.y, other.g1_.x, -other.g0_.w, other.g0_.z), vec4(self.g0_.x) * other.g0_ + vec4(self.g3_.x) * vec4(-other.g0_.w, other.g1_.z, -other.g1_.y, -other.g2_.x) + vec4(self.g3_.y) * vec4(-other.g1_.z, -other.g0_.w, other.g1_.x, -other.g2_.y) + vec4(self.g3_.z) * vec4(other.g1_.y, -other.g1_.x, -other.g0_.w, -other.g2_.z) + vec4(self.g4_.x) * vec4(0.0, other.g0_.z, -other.g0_.y, 0.0) + vec4(self.g4_.y) * vec4(-other.g0_.z, 0.0, other.g0_.x, 0.0) + vec4(self.g4_.z) * vec4(other.g0_.y, -other.g0_.x, 0.0, 0.0) + vec4(self.g5_.x) * vec4(0.0, 0.0, 0.0, other.g0_.x) + vec4(self.g5_.y) * vec4(0.0, 0.0, 0.0, other.g0_.y) + self.g5_.wwwz * other.g0_.xyzz * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g9_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, other.g1_.x) + vec4(self.g9_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, other.g1_.y) + vec4(self.g9_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, other.g1_.z) + vec4(self.g10_.x) * vec4(other.g1_.x, other.g1_.y, other.g1_.z, 0.0), vec3(self.g0_.x) * other.g1_ + vec3(self.g3_.x) * vec3(0.0, other.g2_.z, -other.g2_.y) + vec3(self.g3_.y) * vec3(-other.g2_.z, 0.0, other.g2_.x) + vec3(self.g3_.z) * vec3(other.g2_.y, -other.g2_.x, 0.0) + vec3(self.g4_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g4_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g4_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g5_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g5_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g5_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) - self.g9_ * vec3(other.g0_.w) + vec3(self.g10_.x) * other.g2_ + vec3(self.g10_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(self.g0_.x) * other.g2_ + vec3(self.g4_.x) * vec3(0.0, other.g2_.z, -other.g2_.y) + vec3(self.g4_.y) * vec3(-other.g2_.z, 0.0, other.g2_.x) + vec3(self.g4_.z) * vec3(other.g2_.y, -other.g2_.x, 0.0) + vec3(self.g5_.x) * vec3(other.g0_.w, other.g1_.z, -other.g1_.y) + vec3(self.g5_.y) * vec3(-other.g1_.z, other.g0_.w, other.g1_.x) + vec3(self.g5_.z) * vec3(other.g1_.y, -other.g1_.x, other.g0_.w) + vec3(self.g5_.w) * other.g2_ + vec3(self.g9_.x) * vec3(0.0, other.g2_.z, -other.g2_.y) + vec3(self.g9_.y) * vec3(-other.g2_.z, 0.0, other.g2_.x) + vec3(self.g9_.z) * vec3(other.g2_.y, -other.g2_.x, 0.0) + vec3(self.g10_.y) * other.g1_, vec3(self.g1_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g1_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g1_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g2_.x) * other.g2_ - vec3(self.g2_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g6_.x) * vec3(0.0, other.g2_.z, -other.g2_.y) + vec3(self.g6_.y) * vec3(-other.g2_.z, 0.0, other.g2_.x) + vec3(self.g6_.z) * vec3(other.g2_.y, -other.g2_.x, 0.0) + vec3(self.g6_.w) * other.g1_ + self.g7_ * vec3(other.g0_.w) + vec3(self.g8_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g8_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g8_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec2(self.g1_.x) * vec2(other.g0_.x, -other.g2_.x) + vec2(self.g1_.y) * vec2(other.g0_.y, -other.g2_.y) + vec2(self.g1_.z) * vec2(other.g0_.z, -other.g2_.z) + self.g2_ * vec2(other.g0_.w) + vec2(self.g6_.x) * vec2(other.g1_.x, 0.0) + vec2(self.g6_.y) * vec2(other.g1_.y, 0.0) + vec2(self.g6_.z) * vec2(other.g1_.z, 0.0) + vec2(self.g7_.x) * vec2(other.g0_.x, other.g2_.x) + vec2(self.g7_.y) * vec2(other.g0_.y, other.g2_.y) + vec2(self.g7_.z) * vec2(other.g0_.z, other.g2_.z) + vec2(self.g8_.x) * vec2(0.0, other.g1_.x) + vec2(self.g8_.y) * vec2(0.0, other.g1_.y) + vec2(self.g8_.z) * vec2(0.0, other.g1_.z));
}

MultiVector multiVector_wedgeDot_dipole(MultiVector self, Dipole other) {
    return MultiVector(vec2(self.g3_.x) * vec2(-other.g2_.x, 0.0) + vec2(self.g3_.y) * vec2(-other.g2_.y, 0.0) + vec2(self.g3_.z) * vec2(-other.g2_.z, 0.0) + vec2(self.g4_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g4_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g4_.z) * vec2(-other.g1_.z, 0.0) + vec2(self.g5_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g5_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g5_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g5_.w) * vec2(other.g2_.w, 0.0) + vec2(self.g6_.x) * vec2(0.0, -other.g2_.x) + vec2(self.g6_.y) * vec2(0.0, -other.g2_.y) + vec2(self.g6_.z) * vec2(0.0, -other.g2_.z) + vec2(self.g6_.w) * vec2(0.0, -other.g2_.w) + vec2(self.g7_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g7_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g7_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g8_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g8_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g8_.z) * vec2(0.0, -other.g0_.z), vec3(self.g1_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g1_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g1_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g2_.x) * vec3(other.g2_.x, other.g2_.y, other.g2_.z) - vec3(self.g2_.y) * other.g0_ + vec3(self.g6_.x) * vec3(0.0, other.g2_.z, -other.g2_.y) + vec3(self.g6_.y) * vec3(-other.g2_.z, 0.0, other.g2_.x) + vec3(self.g6_.z) * vec3(other.g2_.y, -other.g2_.x, 0.0) + vec3(self.g6_.w) * other.g1_ - self.g7_ * vec3(other.g2_.w) + vec3(self.g8_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g8_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g8_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec2(self.g1_.x) * vec2(-other.g0_.x, other.g2_.x) + vec2(self.g1_.y) * vec2(-other.g0_.y, other.g2_.y) + vec2(self.g1_.z) * vec2(-other.g0_.z, other.g2_.z) + self.g2_ * vec2(other.g2_.w) + vec2(self.g6_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g6_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g6_.z) * vec2(-other.g1_.z, 0.0) - vec2(self.g7_.x) * vec2(other.g0_.x, other.g2_.x) - vec2(self.g7_.y) * vec2(other.g0_.y, other.g2_.y) - vec2(self.g7_.z) * vec2(other.g0_.z, other.g2_.z) + vec2(self.g8_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g8_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g8_.z) * vec2(0.0, -other.g1_.z), vec3(self.g0_.x) * other.g0_ + vec3(self.g3_.x) * vec3(other.g2_.w, other.g1_.z, -other.g1_.y) + vec3(self.g3_.y) * vec3(-other.g1_.z, other.g2_.w, other.g1_.x) + vec3(self.g3_.z) * vec3(other.g1_.y, -other.g1_.x, other.g2_.w) + vec3(self.g4_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g4_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g4_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) - vec3(self.g5_.w) * other.g0_ + vec3(self.g9_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g9_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g9_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) + vec3(self.g10_.x) * other.g1_, vec3(self.g0_.x) * other.g1_ + vec3(self.g3_.x) * vec3(0.0, other.g2_.z, -other.g2_.y) + vec3(self.g3_.y) * vec3(-other.g2_.z, 0.0, other.g2_.x) + vec3(self.g3_.z) * vec3(other.g2_.y, -other.g2_.x, 0.0) + vec3(self.g4_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g4_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g4_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g5_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g5_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g5_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + self.g9_ * vec3(other.g2_.w) + vec3(self.g10_.x) * vec3(other.g2_.x, other.g2_.y, other.g2_.z) + vec3(self.g10_.y) * other.g0_, vec4(self.g0_.x) * other.g2_ + vec4(self.g3_.x) * vec4(0.0, 0.0, 0.0, other.g2_.x) + vec4(self.g3_.y) * vec4(0.0, 0.0, 0.0, other.g2_.y) + vec4(self.g3_.z) * vec4(0.0, 0.0, 0.0, other.g2_.z) + vec4(self.g4_.x) * vec4(0.0, other.g2_.z, -other.g2_.y, 0.0) + vec4(self.g4_.y) * vec4(-other.g2_.z, 0.0, other.g2_.x, 0.0) + vec4(self.g4_.z) * vec4(other.g2_.y, -other.g2_.x, 0.0, 0.0) + vec4(self.g5_.x) * vec4(-other.g2_.w, other.g1_.z, -other.g1_.y, -other.g0_.x) + vec4(self.g5_.y) * vec4(-other.g1_.z, -other.g2_.w, other.g1_.x, -other.g0_.y) + vec4(self.g5_.z) * vec4(other.g1_.y, -other.g1_.x, -other.g2_.w, -other.g0_.z) + vec4(self.g5_.w) * vec4(other.g2_.x, other.g2_.y, other.g2_.z, 0.0) + vec4(self.g9_.x) * vec4(0.0, other.g2_.z, -other.g2_.y, -other.g1_.x) + vec4(self.g9_.y) * vec4(-other.g2_.z, 0.0, other.g2_.x, -other.g1_.y) + vec4(self.g9_.z) * vec4(other.g2_.y, -other.g2_.x, 0.0, -other.g1_.z) + vec4(self.g10_.y) * vec4(other.g1_.x, other.g1_.y, other.g1_.z, 0.0), vec4(self.g0_.y) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, -other.g2_.w) + vec4(self.g1_.x) * vec4(0.0, other.g0_.z, -other.g0_.y, -other.g1_.x) + vec4(self.g1_.y) * vec4(-other.g0_.z, 0.0, other.g0_.x, -other.g1_.y) + vec4(self.g1_.z) * vec4(other.g0_.y, -other.g0_.x, 0.0, -other.g1_.z) + vec4(self.g2_.x) * vec4(other.g1_.x, other.g1_.y, other.g1_.z, 0.0) + vec4(self.g6_.x) * vec4(other.g2_.w, other.g1_.z, -other.g1_.y, -other.g2_.x) + vec4(self.g6_.y) * vec4(-other.g1_.z, other.g2_.w, other.g1_.x, -other.g2_.y) + vec4(self.g6_.z) * vec4(other.g1_.y, -other.g1_.x, other.g2_.w, -other.g2_.z) + vec4(self.g6_.w) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, 0.0) + vec4(self.g7_.x) * vec4(0.0, other.g0_.z, -other.g0_.y, 0.0) + vec4(self.g7_.y) * vec4(-other.g0_.z, 0.0, other.g0_.x, 0.0) + vec4(self.g7_.z) * vec4(other.g0_.y, -other.g0_.x, 0.0, 0.0) + vec4(self.g8_.x) * vec4(0.0, 0.0, 0.0, other.g0_.x) + vec4(self.g8_.y) * vec4(0.0, 0.0, 0.0, other.g0_.y) + vec4(self.g8_.z) * vec4(0.0, 0.0, 0.0, other.g0_.z), vec3(self.g0_.y) * other.g1_ - self.g1_ * vec3(other.g2_.w) + vec3(self.g2_.x) * vec3(other.g2_.x, other.g2_.y, other.g2_.z) + vec3(self.g2_.y) * other.g0_ + vec3(self.g6_.x) * vec3(0.0, other.g2_.z, -other.g2_.y) + vec3(self.g6_.y) * vec3(-other.g2_.z, 0.0, other.g2_.x) + vec3(self.g6_.z) * vec3(other.g2_.y, -other.g2_.x, 0.0) + vec3(self.g7_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g7_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g7_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g8_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g8_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g8_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec3(self.g0_.y) * vec3(other.g2_.x, other.g2_.y, other.g2_.z) + vec3(self.g1_.x) * vec3(0.0, -other.g2_.z, other.g2_.y) + vec3(self.g1_.y) * vec3(other.g2_.z, 0.0, -other.g2_.x) + vec3(self.g1_.z) * vec3(-other.g2_.y, other.g2_.x, 0.0) + vec3(self.g2_.y) * other.g1_ - vec3(self.g6_.w) * vec3(other.g2_.x, other.g2_.y, other.g2_.z) + vec3(self.g7_.x) * vec3(0.0, other.g2_.z, -other.g2_.y) + vec3(self.g7_.y) * vec3(-other.g2_.z, 0.0, other.g2_.x) + vec3(self.g7_.z) * vec3(other.g2_.y, -other.g2_.x, 0.0) + vec3(self.g8_.x) * vec3(-other.g2_.w, other.g1_.z, -other.g1_.y) + vec3(self.g8_.y) * vec3(-other.g1_.z, -other.g2_.w, other.g1_.x) + vec3(self.g8_.z) * vec3(other.g1_.y, -other.g1_.x, -other.g2_.w), vec3(self.g3_.x) * vec3(0.0, -other.g2_.z, other.g2_.y) + vec3(self.g3_.y) * vec3(other.g2_.z, 0.0, -other.g2_.x) + vec3(self.g3_.z) * vec3(-other.g2_.y, other.g2_.x, 0.0) + self.g4_ * vec3(other.g2_.w) + vec3(self.g5_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g5_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g5_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + vec3(self.g5_.w) * other.g1_ + vec3(self.g9_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g9_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g9_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) - vec3(self.g10_.x) * vec3(other.g2_.x, other.g2_.y, other.g2_.z) + vec3(self.g10_.y) * other.g0_, vec2(self.g3_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g3_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g3_.z) * vec2(-other.g1_.z, 0.0) - vec2(self.g4_.x) * vec2(other.g0_.x, other.g2_.x) - vec2(self.g4_.y) * vec2(other.g0_.y, other.g2_.y) - vec2(self.g4_.z) * vec2(other.g0_.z, other.g2_.z) + vec2(self.g5_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g5_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g5_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g9_.x) * vec2(other.g0_.x, -other.g2_.x) + vec2(self.g9_.y) * vec2(other.g0_.y, -other.g2_.y) + vec2(self.g9_.z) * vec2(other.g0_.z, -other.g2_.z) + self.g10_ * vec2(other.g2_.w));
}

MultiVector multiVector_wedgeDot_dualNum(MultiVector self, DualNum other) {
    return MultiVector(vec2(self.g0_.x) * other.g0_ + vec2(self.g0_.y) * other.g0_.yx * vec2(-1.0, 1.0), self.g1_ * vec3(other.g0_.x) - self.g9_ * vec3(other.g0_.y), self.g2_ * vec2(other.g0_.x) + self.g10_ * vec2(other.g0_.y), self.g3_ * vec3(other.g0_.x) - vec3(self.g6_.x, self.g6_.y, self.g6_.z) * vec3(other.g0_.y), self.g4_ * vec3(other.g0_.x) - self.g7_ * vec3(other.g0_.y), self.g5_ * vec4(other.g0_.x) + vec4(self.g6_.w) * vec4(0.0, 0.0, 0.0, other.g0_.y) + vec4(self.g8_.x, self.g8_.y, self.g8_.z, self.g8_.x) * vec4(-other.g0_.y, -other.g0_.y, -other.g0_.y, 0.0), vec4(self.g3_.x, self.g3_.y, self.g3_.z, self.g3_.x) * vec4(other.g0_.y, other.g0_.y, other.g0_.y, 0.0) + vec4(self.g5_.w) * vec4(0.0, 0.0, 0.0, -other.g0_.y) + self.g6_ * vec4(other.g0_.x), self.g4_ * vec3(other.g0_.y) + self.g7_ * vec3(other.g0_.x), vec3(self.g5_.x, self.g5_.y, self.g5_.z) * vec3(other.g0_.y) + self.g8_ * vec3(other.g0_.x), self.g1_ * vec3(other.g0_.y) + self.g9_ * vec3(other.g0_.x), vec2(0.0) - self.g2_ * vec2(other.g0_.y) + self.g10_ * vec2(other.g0_.x));
}

MultiVector multiVector_wedgeDot_flatPoint(MultiVector self, FlatPoint other) {
    return MultiVector(vec2(self.g3_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g3_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g3_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g5_.w) * vec2(other.g0_.w, 0.0) + vec2(self.g6_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g6_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g6_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g6_.w) * vec2(0.0, -other.g0_.w), vec3(self.g2_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g6_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g6_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g6_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) - self.g7_ * vec3(other.g0_.w), vec2(self.g1_.x) * vec2(0.0, other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, other.g0_.z) + self.g2_ * vec2(other.g0_.w) + vec2(self.g7_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g7_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g7_.z) * vec2(0.0, -other.g0_.z), self.g3_ * vec3(other.g0_.w), vec3(self.g3_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g3_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g3_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + self.g9_ * vec3(other.g0_.w) + vec3(self.g10_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec4(self.g0_.x) * other.g0_ + vec4(self.g3_.x) * vec4(0.0, 0.0, 0.0, other.g0_.x) + vec4(self.g3_.y) * vec4(0.0, 0.0, 0.0, other.g0_.y) + vec4(self.g3_.z) * vec4(0.0, 0.0, 0.0, other.g0_.z) + vec4(self.g4_.x) * vec4(0.0, other.g0_.z, -other.g0_.y, 0.0) + vec4(self.g4_.y) * vec4(-other.g0_.z, 0.0, other.g0_.x, 0.0) + vec4(self.g4_.z) * vec4(other.g0_.y, -other.g0_.x, 0.0, 0.0) + self.g5_.xyzx * vec4(-other.g0_.w, -other.g0_.w, -other.g0_.w, 0.0) + vec4(self.g5_.w) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, 0.0) + vec4(self.g9_.x) * vec4(0.0, other.g0_.z, -other.g0_.y, 0.0) + vec4(self.g9_.y) * vec4(-other.g0_.z, 0.0, other.g0_.x, 0.0) + vec4(self.g9_.z) * vec4(other.g0_.y, -other.g0_.x, 0.0, 0.0), vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, -other.g0_.w) + vec4(self.g6_.x) * vec4(other.g0_.w, 0.0, 0.0, -other.g0_.x) + vec4(self.g6_.y) * vec4(0.0, other.g0_.w, 0.0, -other.g0_.y) + vec4(self.g6_.z) * vec4(0.0, 0.0, other.g0_.w, -other.g0_.z), vec3(0.0) - self.g1_ * vec3(other.g0_.w) + vec3(self.g2_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g6_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g6_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g6_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec3(self.g0_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) - vec3(self.g6_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g7_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g7_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g7_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) - self.g8_ * vec3(other.g0_.w), vec3(self.g3_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g3_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g3_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) + self.g4_ * vec3(other.g0_.w) - vec3(self.g10_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g4_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g4_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g4_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g9_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g9_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g9_.z) * vec2(0.0, -other.g0_.z) + self.g10_ * vec2(other.g0_.w));
}

MultiVector multiVector_wedgeDot_flector(MultiVector self, Flector other) {
    return MultiVector(vec2(self.g1_.x) * vec2(0.0, other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, other.g1_.z) + vec2(self.g2_.x) * vec2(0.0, other.g1_.w) + vec2(self.g3_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g3_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g3_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g5_.w) * vec2(other.g0_.w, 0.0) + vec2(self.g6_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g6_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g6_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g6_.w) * vec2(0.0, -other.g0_.w) + vec2(self.g9_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g9_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g9_.z) * vec2(-other.g1_.z, 0.0) + vec2(self.g10_.x) * vec2(other.g1_.w, 0.0), vec3(0.0) - vec3(self.g0_.y) * vec3(other.g1_.x, other.g1_.y, other.g1_.z) + vec3(self.g2_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g6_.x) * vec3(other.g1_.w, other.g0_.z, -other.g0_.y) + vec3(self.g6_.y) * vec3(-other.g0_.z, other.g1_.w, other.g0_.x) + vec3(self.g6_.z) * vec3(other.g0_.y, -other.g0_.x, other.g1_.w) + vec3(self.g7_.x) * vec3(-other.g0_.w, -other.g1_.z, other.g1_.y) + vec3(self.g7_.y) * vec3(other.g1_.z, -other.g0_.w, -other.g1_.x) + vec3(self.g7_.z) * vec3(-other.g1_.y, other.g1_.x, -other.g0_.w), vec2(self.g0_.y) * vec2(0.0, other.g1_.w) + vec2(self.g1_.x) * vec2(0.0, other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, other.g0_.z) + self.g2_ * vec2(other.g0_.w) + vec2(self.g6_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g6_.y) * vec2(-other.g1_.y, 0.0) - vec2(self.g6_.z, self.g6_.w) * vec2(other.g1_.z, other.g1_.w) + vec2(self.g7_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g7_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g7_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g8_.x) * vec2(0.0, other.g1_.x) + vec2(self.g8_.y) * vec2(0.0, other.g1_.y) + vec2(self.g8_.z) * vec2(0.0, other.g1_.z), vec3(self.g3_.x) * vec3(other.g0_.w, other.g1_.z, -other.g1_.y) + vec3(self.g3_.y) * vec3(-other.g1_.z, other.g0_.w, other.g1_.x) + vec3(self.g3_.z) * vec3(other.g1_.y, -other.g1_.x, other.g0_.w) + vec3(self.g10_.x) * vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec3(self.g3_.x) * vec3(other.g1_.w, other.g0_.z, -other.g0_.y) + vec3(self.g3_.y) * vec3(-other.g0_.z, other.g1_.w, other.g0_.x) + vec3(self.g3_.z) * vec3(other.g0_.y, -other.g0_.x, other.g1_.w) + vec3(self.g5_.w) * vec3(other.g1_.x, other.g1_.y, other.g1_.z) + vec3(self.g9_.x) * vec3(other.g0_.w, other.g1_.z, -other.g1_.y) + vec3(self.g9_.y) * vec3(-other.g1_.z, other.g0_.w, other.g1_.x) + vec3(self.g9_.z) * vec3(other.g1_.y, -other.g1_.x, other.g0_.w) + vec3(self.g10_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec4(self.g0_.x) * other.g0_ + vec4(self.g3_.x) * vec4(0.0, 0.0, 0.0, other.g0_.x) + vec4(self.g3_.y) * vec4(0.0, 0.0, 0.0, other.g0_.y) + vec4(self.g3_.z) * vec4(0.0, 0.0, 0.0, other.g0_.z) + vec4(self.g4_.x) * vec4(other.g1_.w, other.g0_.z, -other.g0_.y, -other.g1_.x) + vec4(self.g4_.y) * vec4(-other.g0_.z, other.g1_.w, other.g0_.x, -other.g1_.y) + vec4(self.g4_.z) * vec4(other.g0_.y, -other.g0_.x, other.g1_.w, -other.g1_.z) + vec4(self.g5_.x) * vec4(-other.g0_.w, -other.g1_.z, other.g1_.y, 0.0) + vec4(self.g5_.y) * vec4(other.g1_.z, -other.g0_.w, -other.g1_.x, 0.0) + vec4(self.g5_.z) * vec4(-other.g1_.y, other.g1_.x, -other.g0_.w, 0.0) + vec4(self.g5_.w) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, 0.0) + vec4(self.g9_.x) * vec4(other.g1_.w, other.g0_.z, -other.g0_.y, 0.0) + vec4(self.g9_.y) * vec4(-other.g0_.z, other.g1_.w, other.g0_.x, 0.0) + vec4(self.g9_.z) * vec4(other.g0_.y, -other.g0_.x, other.g1_.w, 0.0) - vec4(self.g10_.y, self.g10_.y, self.g10_.y, self.g10_.x) * other.g1_, vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, -other.g0_.w) + vec4(self.g2_.x) * other.g1_ + vec4(self.g6_.x) * vec4(other.g0_.w, other.g1_.z, -other.g1_.y, -other.g0_.x) + vec4(self.g6_.y) * vec4(-other.g1_.z, other.g0_.w, other.g1_.x, -other.g0_.y) + vec4(self.g6_.z) * vec4(other.g1_.y, -other.g1_.x, other.g0_.w, -other.g0_.z) + vec4(self.g7_.x) * vec4(0.0, 0.0, 0.0, other.g1_.x) + vec4(self.g7_.y) * vec4(0.0, 0.0, 0.0, other.g1_.y) + vec4(self.g7_.z) * vec4(0.0, 0.0, 0.0, other.g1_.z), vec3(self.g1_.x) * vec3(-other.g0_.w, -other.g1_.z, other.g1_.y) + vec3(self.g1_.y) * vec3(other.g1_.z, -other.g0_.w, -other.g1_.x) + vec3(self.g1_.z) * vec3(-other.g1_.y, other.g1_.x, -other.g0_.w) + vec3(self.g2_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g6_.x) * vec3(other.g1_.w, other.g0_.z, -other.g0_.y) + vec3(self.g6_.y) * vec3(-other.g0_.z, other.g1_.w, other.g0_.x) + vec3(self.g6_.z) * vec3(other.g0_.y, -other.g0_.x, other.g1_.w) - vec3(self.g6_.w) * vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec3(self.g0_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g1_.x) * vec3(-other.g1_.w, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, -other.g1_.w, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, -other.g1_.w) - vec3(self.g2_.y) * vec3(other.g1_.x, other.g1_.y, other.g1_.z) - vec3(self.g6_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g7_.x) * vec3(other.g1_.w, other.g0_.z, -other.g0_.y) + vec3(self.g7_.y) * vec3(-other.g0_.z, other.g1_.w, other.g0_.x) + vec3(self.g7_.z) * vec3(other.g0_.y, -other.g0_.x, other.g1_.w) + vec3(self.g8_.x) * vec3(-other.g0_.w, -other.g1_.z, other.g1_.y) + vec3(self.g8_.y) * vec3(other.g1_.z, -other.g0_.w, -other.g1_.x) + vec3(self.g8_.z) * vec3(-other.g1_.y, other.g1_.x, -other.g0_.w), vec3(self.g0_.x) * vec3(other.g1_.x, other.g1_.y, other.g1_.z) + vec3(self.g3_.x) * vec3(-other.g1_.w, -other.g0_.z, other.g0_.y) + vec3(self.g3_.y) * vec3(other.g0_.z, -other.g1_.w, -other.g0_.x) + vec3(self.g3_.z) * vec3(-other.g0_.y, other.g0_.x, -other.g1_.w) + vec3(self.g4_.x) * vec3(other.g0_.w, other.g1_.z, -other.g1_.y) + vec3(self.g4_.y) * vec3(-other.g1_.z, other.g0_.w, other.g1_.x) + vec3(self.g4_.z) * vec3(other.g1_.y, -other.g1_.x, other.g0_.w) - vec3(self.g10_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g0_.x) * vec2(0.0, other.g1_.w) + vec2(self.g3_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g3_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g3_.z) * vec2(-other.g1_.z, 0.0) + vec2(self.g4_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g4_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g4_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g5_.x) * vec2(0.0, other.g1_.x) + vec2(self.g5_.y) * vec2(0.0, other.g1_.y) + vec2(self.g5_.z) * vec2(0.0, other.g1_.z) + vec2(self.g5_.w) * vec2(0.0, other.g1_.w) + vec2(self.g9_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g9_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g9_.z) * vec2(0.0, -other.g0_.z) + self.g10_ * vec2(other.g0_.w));
}

MultiVector multiVector_wedgeDot_line(MultiVector self, Line other) {
    return MultiVector(vec2(self.g3_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g3_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g3_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g4_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g4_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g4_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g6_.x) * vec2(other.g1_.x, 0.0) + vec2(self.g6_.y) * vec2(other.g1_.y, 0.0) + vec2(self.g6_.z) * vec2(other.g1_.z, 0.0) + vec2(self.g7_.x) * vec2(other.g0_.x, 0.0) + vec2(self.g7_.y) * vec2(other.g0_.y, 0.0) + vec2(self.g7_.z) * vec2(other.g0_.z, 0.0), vec3(self.g3_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g3_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g3_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) - vec3(self.g5_.w) * other.g0_ + vec3(self.g9_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g9_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g9_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) + vec3(self.g10_.x) * other.g1_, vec2(self.g3_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g3_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g3_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g4_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g4_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g4_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g5_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g5_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g5_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g9_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g9_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g9_.z) * vec2(0.0, -other.g1_.z), vec3(0.0) - vec3(self.g2_.x) * other.g0_ + vec3(self.g6_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g6_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g6_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(0.0) - vec3(self.g0_.y) * other.g0_ - vec3(self.g2_.x) * other.g1_ + vec3(self.g6_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g6_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g6_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g7_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g7_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g7_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec4(self.g0_.y) * vec4(-other.g1_.x, -other.g1_.y, -other.g1_.z, 0.0) + vec4(self.g1_.x) * vec4(0.0, other.g1_.z, -other.g1_.y, -other.g0_.x) + vec4(self.g1_.y) * vec4(-other.g1_.z, 0.0, other.g1_.x, -other.g0_.y) + vec4(self.g1_.z) * vec4(other.g1_.y, -other.g1_.x, 0.0, -other.g0_.z) + vec4(self.g2_.y) * vec4(-other.g0_.x, -other.g0_.y, -other.g0_.z, 0.0) + vec4(self.g6_.x) * vec4(0.0, 0.0, 0.0, -other.g1_.x) + vec4(self.g6_.y) * vec4(0.0, 0.0, 0.0, -other.g1_.y) + self.g6_.wwwz * vec4(other.g1_.x, other.g1_.y, other.g1_.z, -other.g1_.z) + vec4(self.g7_.x) * vec4(0.0, -other.g1_.z, other.g1_.y, 0.0) + vec4(self.g7_.y) * vec4(other.g1_.z, 0.0, -other.g1_.x, 0.0) + vec4(self.g7_.z) * vec4(-other.g1_.y, other.g1_.x, 0.0, 0.0) + vec4(self.g8_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g8_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g8_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0), vec4(self.g3_.x) * vec4(0.0, other.g0_.z, -other.g0_.y, -other.g1_.x) + vec4(self.g3_.y) * vec4(-other.g0_.z, 0.0, other.g0_.x, -other.g1_.y) + vec4(self.g3_.z) * vec4(other.g0_.y, -other.g0_.x, 0.0, -other.g1_.z) + vec4(self.g9_.x) * vec4(0.0, 0.0, 0.0, other.g0_.x) + vec4(self.g9_.y) * vec4(0.0, 0.0, 0.0, other.g0_.y) + vec4(self.g9_.z) * vec4(0.0, 0.0, 0.0, other.g0_.z) + vec4(self.g10_.x) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, 0.0), vec3(self.g0_.x) * other.g0_ + vec3(self.g3_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g3_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g3_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g4_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g4_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g4_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + vec3(self.g10_.x) * other.g1_, vec3(self.g0_.x) * other.g1_ + vec3(self.g4_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g4_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g4_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g5_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g5_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g5_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + vec3(self.g5_.w) * other.g1_ + vec3(self.g9_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g9_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g9_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g10_.y) * other.g0_, vec3(self.g1_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g1_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g1_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + vec3(self.g2_.x) * other.g1_ + vec3(self.g6_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g6_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g6_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g6_.w) * other.g0_, vec2(self.g1_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g6_.x) * vec2(other.g0_.x, 0.0) + vec2(self.g6_.y) * vec2(other.g0_.y, 0.0) + vec2(self.g6_.z) * vec2(other.g0_.z, 0.0) + vec2(self.g7_.x) * vec2(0.0, other.g1_.x) + vec2(self.g7_.y) * vec2(0.0, other.g1_.y) + vec2(self.g7_.z) * vec2(0.0, other.g1_.z) + vec2(self.g8_.x) * vec2(0.0, other.g0_.x) + vec2(self.g8_.y) * vec2(0.0, other.g0_.y) + vec2(self.g8_.z) * vec2(0.0, other.g0_.z));
}

MultiVector multiVector_wedgeDot_motor(MultiVector self, Motor other) {
    return MultiVector(self.g0_.yx * vec2(-other.g0_.w) + vec2(self.g3_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g3_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g3_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g4_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g4_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g4_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g6_.x) * vec2(other.g1_.x, 0.0) + vec2(self.g6_.y) * vec2(other.g1_.y, 0.0) + vec2(self.g6_.z) * vec2(other.g1_.z, 0.0) + vec2(self.g7_.x) * vec2(other.g0_.x, 0.0) + vec2(self.g7_.y) * vec2(other.g0_.y, 0.0) + vec2(self.g7_.z) * vec2(other.g0_.z, 0.0), vec3(self.g3_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g3_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g3_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) - vec3(self.g5_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g9_.x) * vec3(-other.g0_.w, -other.g0_.z, other.g0_.y) + vec3(self.g9_.y) * vec3(other.g0_.z, -other.g0_.w, -other.g0_.x) + vec3(self.g9_.z) * vec3(-other.g0_.y, other.g0_.x, -other.g0_.w) + vec3(self.g10_.x) * other.g1_, vec2(self.g3_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g3_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g3_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g4_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g4_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g4_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g5_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g5_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g5_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g9_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g9_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g9_.z) * vec2(0.0, -other.g1_.z) + self.g10_ * vec2(other.g0_.w), vec3(0.0) - vec3(self.g2_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g6_.x) * vec3(-other.g0_.w, -other.g0_.z, other.g0_.y) + vec3(self.g6_.y) * vec3(other.g0_.z, -other.g0_.w, -other.g0_.x) + vec3(self.g6_.z) * vec3(-other.g0_.y, other.g0_.x, -other.g0_.w), vec3(0.0) - vec3(self.g0_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) - vec3(self.g2_.x) * other.g1_ + vec3(self.g6_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g6_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g6_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g7_.x) * vec3(-other.g0_.w, -other.g0_.z, other.g0_.y) + vec3(self.g7_.y) * vec3(other.g0_.z, -other.g0_.w, -other.g0_.x) + vec3(self.g7_.z) * vec3(-other.g0_.y, other.g0_.x, -other.g0_.w), vec4(self.g0_.y) * vec4(-other.g1_.x, -other.g1_.y, -other.g1_.z, 0.0) + vec4(self.g1_.x) * vec4(0.0, other.g1_.z, -other.g1_.y, -other.g0_.x) + vec4(self.g1_.y) * vec4(-other.g1_.z, 0.0, other.g1_.x, -other.g0_.y) + vec4(self.g1_.z) * vec4(other.g1_.y, -other.g1_.x, 0.0, -other.g0_.z) + vec4(self.g2_.y) * vec4(-other.g0_.x, -other.g0_.y, -other.g0_.z, 0.0) + vec4(self.g6_.x) * vec4(0.0, 0.0, 0.0, -other.g1_.x) + vec4(self.g6_.y) * vec4(0.0, 0.0, 0.0, -other.g1_.y) + vec4(self.g6_.z) * vec4(0.0, 0.0, 0.0, -other.g1_.z) + vec4(self.g6_.w) * vec4(other.g1_.x, other.g1_.y, other.g1_.z, other.g0_.w) + vec4(self.g7_.x) * vec4(0.0, -other.g1_.z, other.g1_.y, 0.0) + vec4(self.g7_.y) * vec4(other.g1_.z, 0.0, -other.g1_.x, 0.0) + vec4(self.g7_.z) * vec4(-other.g1_.y, other.g1_.x, 0.0, 0.0) + vec4(self.g8_.x) * vec4(-other.g0_.w, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g8_.y) * vec4(other.g0_.z, -other.g0_.w, -other.g0_.x, 0.0) + vec4(self.g8_.z) * vec4(-other.g0_.y, other.g0_.x, -other.g0_.w, 0.0), vec4(self.g3_.x) * vec4(other.g0_.w, other.g0_.z, -other.g0_.y, -other.g1_.x) + vec4(self.g3_.y) * vec4(-other.g0_.z, other.g0_.w, other.g0_.x, -other.g1_.y) + vec4(self.g3_.z) * vec4(other.g0_.y, -other.g0_.x, other.g0_.w, -other.g1_.z) + vec4(self.g5_.w) * vec4(0.0, 0.0, 0.0, -other.g0_.w) + vec4(self.g9_.x) * vec4(0.0, 0.0, 0.0, other.g0_.x) + vec4(self.g9_.y) * vec4(0.0, 0.0, 0.0, other.g0_.y) + vec4(self.g9_.z) * vec4(0.0, 0.0, 0.0, other.g0_.z) + vec4(self.g10_.x) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, 0.0), vec3(self.g0_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g3_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g3_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g3_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g4_.x) * vec3(other.g0_.w, other.g0_.z, -other.g0_.y) + vec3(self.g4_.y) * vec3(-other.g0_.z, other.g0_.w, other.g0_.x) + vec3(self.g4_.z) * vec3(other.g0_.y, -other.g0_.x, other.g0_.w) + vec3(self.g10_.x) * other.g1_, vec3(self.g0_.x) * other.g1_ + vec3(self.g4_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g4_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g4_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g5_.x) * vec3(other.g0_.w, other.g0_.z, -other.g0_.y) + vec3(self.g5_.y) * vec3(-other.g0_.z, other.g0_.w, other.g0_.x) + vec3(self.g5_.z) * vec3(other.g0_.y, -other.g0_.x, other.g0_.w) + vec3(self.g5_.w) * other.g1_ + vec3(self.g9_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g9_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g9_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g10_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(self.g1_.x) * vec3(other.g0_.w, other.g0_.z, -other.g0_.y) + vec3(self.g1_.y) * vec3(-other.g0_.z, other.g0_.w, other.g0_.x) + vec3(self.g1_.z) * vec3(other.g0_.y, -other.g0_.x, other.g0_.w) + vec3(self.g2_.x) * other.g1_ + vec3(self.g6_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g6_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g6_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g6_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g1_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g1_.z) - self.g2_ * vec2(other.g0_.w) + vec2(self.g6_.x) * vec2(other.g0_.x, 0.0) + vec2(self.g6_.y) * vec2(other.g0_.y, 0.0) + vec2(self.g6_.z) * vec2(other.g0_.z, 0.0) + vec2(self.g7_.x) * vec2(0.0, other.g1_.x) + vec2(self.g7_.y) * vec2(0.0, other.g1_.y) + vec2(self.g7_.z) * vec2(0.0, other.g1_.z) + vec2(self.g8_.x) * vec2(0.0, other.g0_.x) + vec2(self.g8_.y) * vec2(0.0, other.g0_.y) + vec2(self.g8_.z) * vec2(0.0, other.g0_.z));
}

MultiVector multiVector_wedgeDot_multiVector(MultiVector self, MultiVector other) {
    return MultiVector(vec2(self.g0_.x) * other.g0_ + vec2(self.g0_.y) * other.g0_.yx * vec2(-1.0, 1.0) + vec2(self.g1_.x) * vec2(other.g1_.x, other.g9_.x) + vec2(self.g1_.y) * vec2(other.g1_.y, other.g9_.y) + vec2(self.g1_.z) * vec2(other.g1_.z, other.g9_.z) + vec2(self.g2_.x) * vec2(-other.g2_.y, other.g10_.y) + vec2(self.g2_.y) * vec2(-other.g2_.x, other.g10_.x) - vec2(self.g3_.x) * vec2(other.g5_.x, other.g8_.x) - vec2(self.g3_.y) * vec2(other.g5_.y, other.g8_.y) - vec2(self.g3_.z) * vec2(other.g5_.z, other.g8_.z) - vec2(self.g4_.x) * vec2(other.g4_.x, other.g7_.x) - vec2(self.g4_.y) * vec2(other.g4_.y, other.g7_.y) - vec2(self.g4_.z) * vec2(other.g4_.z, other.g7_.z) - vec2(self.g5_.x) * vec2(other.g3_.x, other.g6_.x) - vec2(self.g5_.y) * vec2(other.g3_.y, other.g6_.y) - vec2(self.g5_.z) * vec2(other.g3_.z, other.g6_.z) + vec2(self.g5_.w) * vec2(other.g5_.w, -other.g6_.w) + vec2(self.g6_.x) * vec2(other.g8_.x, -other.g5_.x) + vec2(self.g6_.y) * vec2(other.g8_.y, -other.g5_.y) + vec2(self.g6_.z) * vec2(other.g8_.z, -other.g5_.z) - vec2(self.g6_.w) * vec2(other.g6_.w, other.g5_.w) + vec2(self.g7_.x) * vec2(other.g7_.x, -other.g4_.x) + vec2(self.g7_.y) * vec2(other.g7_.y, -other.g4_.y) + vec2(self.g7_.z) * vec2(other.g7_.z, -other.g4_.z) + vec2(self.g8_.x) * vec2(other.g6_.x, -other.g3_.x) + vec2(self.g8_.y) * vec2(other.g6_.y, -other.g3_.y) + vec2(self.g8_.z) * vec2(other.g6_.z, -other.g3_.z) + vec2(self.g9_.x) * vec2(-other.g9_.x, other.g1_.x) + vec2(self.g9_.y) * vec2(-other.g9_.y, other.g1_.y) + vec2(self.g9_.z) * vec2(-other.g9_.z, other.g1_.z) + vec2(self.g10_.x) * vec2(other.g10_.y, other.g2_.y) + vec2(self.g10_.y) * vec2(other.g10_.x, other.g2_.x), vec3(self.g0_.x) * other.g1_ - vec3(self.g0_.y) * other.g9_ + vec3(self.g1_.x) * vec3(other.g0_.x, other.g4_.z, -other.g4_.y) + vec3(self.g1_.y) * vec3(-other.g4_.z, other.g0_.x, other.g4_.x) + vec3(self.g1_.z) * vec3(other.g4_.y, -other.g4_.x, other.g0_.x) + vec3(self.g2_.x) * vec3(other.g5_.x, other.g5_.y, other.g5_.z) - vec3(self.g2_.y) * other.g3_ + vec3(self.g3_.x) * vec3(other.g2_.y, other.g8_.z, -other.g8_.y) + vec3(self.g3_.y) * vec3(-other.g8_.z, other.g2_.y, other.g8_.x) + vec3(self.g3_.z) * vec3(other.g8_.y, -other.g8_.x, other.g2_.y) + vec3(self.g4_.x) * vec3(other.g6_.w, other.g1_.z, -other.g1_.y) + vec3(self.g4_.y) * vec3(-other.g1_.z, other.g6_.w, other.g1_.x) + vec3(self.g4_.z) * vec3(other.g1_.y, -other.g1_.x, other.g6_.w) + vec3(self.g5_.x) * vec3(-other.g2_.x, -other.g6_.z, other.g6_.y) + vec3(self.g5_.y) * vec3(other.g6_.z, -other.g2_.x, -other.g6_.x) + vec3(self.g5_.z) * vec3(-other.g6_.y, other.g6_.x, -other.g2_.x) - vec3(self.g5_.w) * other.g7_ + vec3(self.g6_.x) * vec3(other.g10_.y, other.g5_.z, -other.g5_.y) + vec3(self.g6_.y) * vec3(-other.g5_.z, other.g10_.y, other.g5_.x) + vec3(self.g6_.z) * vec3(other.g5_.y, -other.g5_.x, other.g10_.y) + vec3(self.g6_.w) * other.g4_ + vec3(self.g7_.x) * vec3(-other.g5_.w, -other.g9_.z, other.g9_.y) + vec3(self.g7_.y) * vec3(other.g9_.z, -other.g5_.w, -other.g9_.x) + vec3(self.g7_.z) * vec3(-other.g9_.y, other.g9_.x, -other.g5_.w) + vec3(self.g8_.x) * vec3(-other.g10_.x, -other.g3_.z, other.g3_.y) + vec3(self.g8_.y) * vec3(other.g3_.z, -other.g10_.x, -other.g3_.x) + vec3(self.g8_.z) * vec3(-other.g3_.y, other.g3_.x, -other.g10_.x) + vec3(self.g9_.x) * vec3(-other.g0_.y, -other.g7_.z, other.g7_.y) + vec3(self.g9_.y) * vec3(other.g7_.z, -other.g0_.y, -other.g7_.x) + vec3(self.g9_.z) * vec3(-other.g7_.y, other.g7_.x, -other.g0_.y) + vec3(self.g10_.x) * other.g8_ - vec3(self.g10_.y) * vec3(other.g6_.x, other.g6_.y, other.g6_.z), vec2(self.g0_.x) * other.g2_ + vec2(self.g0_.y) * other.g10_ + vec2(self.g1_.x) * vec2(-other.g3_.x, other.g5_.x) + vec2(self.g1_.y) * vec2(-other.g3_.y, other.g5_.y) + vec2(self.g1_.z) * vec2(-other.g3_.z, other.g5_.z) + self.g2_ * vec2(other.g5_.w) + self.g2_ * vec2(other.g0_.x) + vec2(self.g3_.x) * vec2(-other.g7_.x, 0.0) + vec2(self.g3_.x) * vec2(other.g1_.x, 0.0) + vec2(self.g3_.y) * vec2(-other.g7_.y, 0.0) + vec2(self.g3_.y) * vec2(other.g1_.y, 0.0) + vec2(self.g3_.z) * vec2(-other.g7_.z, 0.0) + vec2(self.g3_.z) * vec2(other.g1_.z, 0.0) - vec2(self.g4_.x) * vec2(other.g6_.x, other.g8_.x) - vec2(self.g4_.y) * vec2(other.g6_.y, other.g8_.y) - vec2(self.g4_.z) * vec2(other.g6_.z, other.g8_.z) + vec2(self.g5_.x) * vec2(0.0, -other.g7_.x) + vec2(self.g5_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g5_.y) * vec2(0.0, -other.g7_.y) + vec2(self.g5_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g5_.z) * vec2(0.0, -other.g7_.z) + vec2(self.g5_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g5_.w) * other.g2_ * vec2(-1.0, 1.0) + vec2(self.g6_.x) * vec2(-other.g9_.x, 0.0) + vec2(self.g6_.x) * vec2(-other.g4_.x, 0.0) + vec2(self.g6_.y) * vec2(-other.g9_.y, 0.0) + vec2(self.g6_.y) * vec2(-other.g4_.y, 0.0) + vec2(self.g6_.z) * vec2(-other.g9_.z, 0.0) + vec2(self.g6_.z) * vec2(-other.g4_.z, 0.0) + vec2(self.g6_.w) * other.g10_ * vec2(1.0, -1.0) - vec2(self.g7_.x) * vec2(other.g3_.x, other.g5_.x) - vec2(self.g7_.y) * vec2(other.g3_.y, other.g5_.y) - vec2(self.g7_.z) * vec2(other.g3_.z, other.g5_.z) + vec2(self.g8_.x) * vec2(0.0, other.g9_.x) + vec2(self.g8_.x) * vec2(0.0, -other.g4_.x) + vec2(self.g8_.y) * vec2(0.0, other.g9_.y) + vec2(self.g8_.y) * vec2(0.0, -other.g4_.y) + vec2(self.g8_.z) * vec2(0.0, other.g9_.z) + vec2(self.g8_.z) * vec2(0.0, -other.g4_.z) + vec2(self.g9_.x) * vec2(other.g6_.x, -other.g8_.x) + vec2(self.g9_.y) * vec2(other.g6_.y, -other.g8_.y) + vec2(self.g9_.z) * vec2(other.g6_.z, -other.g8_.z) + self.g10_ * vec2(-other.g6_.w) + self.g10_ * vec2(other.g0_.y), vec3(self.g0_.x) * other.g3_ - vec3(self.g0_.y) * vec3(other.g6_.x, other.g6_.y, other.g6_.z) + vec3(self.g1_.x) * vec3(-other.g2_.x, -other.g6_.z, other.g6_.y) + vec3(self.g1_.y) * vec3(other.g6_.z, -other.g2_.x, -other.g6_.x) + vec3(self.g1_.z) * vec3(-other.g6_.y, other.g6_.x, -other.g2_.x) - vec3(self.g2_.x) * other.g7_ + vec3(self.g2_.x) * other.g1_ + vec3(self.g3_.x) * vec3(other.g5_.w, other.g9_.z, -other.g9_.y) + vec3(self.g3_.x) * vec3(other.g0_.x, other.g4_.z, -other.g4_.y) + vec3(self.g3_.y) * vec3(-other.g9_.z, other.g5_.w, other.g9_.x) + vec3(self.g3_.y) * vec3(-other.g4_.z, other.g0_.x, other.g4_.x) + vec3(self.g3_.z) * vec3(other.g9_.y, -other.g9_.x, other.g5_.w) + vec3(self.g3_.z) * vec3(other.g4_.y, -other.g4_.x, other.g0_.x) + vec3(self.g4_.x) * vec3(other.g10_.x, other.g3_.z, -other.g3_.y) + vec3(self.g4_.y) * vec3(-other.g3_.z, other.g10_.x, other.g3_.x) + vec3(self.g4_.z) * vec3(other.g3_.y, -other.g3_.x, other.g10_.x) - vec3(self.g5_.w) * other.g3_ + vec3(self.g6_.x) * vec3(other.g6_.w, -other.g7_.z, other.g7_.y) + vec3(self.g6_.x) * vec3(-other.g0_.y, other.g1_.z, -other.g1_.y) + vec3(self.g6_.y) * vec3(other.g7_.z, other.g6_.w, -other.g7_.x) + vec3(self.g6_.y) * vec3(-other.g1_.z, -other.g0_.y, other.g1_.x) + vec3(self.g6_.z) * vec3(-other.g7_.y, other.g7_.x, other.g6_.w) + vec3(self.g6_.z) * vec3(other.g1_.y, -other.g1_.x, -other.g0_.y) - vec3(self.g6_.w) * vec3(other.g6_.x, other.g6_.y, other.g6_.z) + vec3(self.g7_.x) * vec3(-other.g2_.x, -other.g6_.z, other.g6_.y) + vec3(self.g7_.y) * vec3(other.g6_.z, -other.g2_.x, -other.g6_.x) + vec3(self.g7_.z) * vec3(-other.g6_.y, other.g6_.x, -other.g2_.x) + vec3(self.g9_.x) * vec3(-other.g10_.x, -other.g3_.z, other.g3_.y) + vec3(self.g9_.y) * vec3(other.g3_.z, -other.g10_.x, -other.g3_.x) + vec3(self.g9_.z) * vec3(-other.g3_.y, other.g3_.x, -other.g10_.x) + vec3(self.g10_.x) * other.g9_ + vec3(self.g10_.x) * other.g4_, vec3(self.g0_.x) * other.g4_ - vec3(self.g0_.y) * other.g7_ + vec3(self.g1_.x) * vec3(-other.g6_.w, -other.g1_.z, other.g1_.y) + vec3(self.g1_.y) * vec3(other.g1_.z, -other.g6_.w, -other.g1_.x) + vec3(self.g1_.z) * vec3(-other.g1_.y, other.g1_.x, -other.g6_.w) - vec3(self.g2_.x) * other.g8_ - vec3(self.g2_.y) * vec3(other.g6_.x, other.g6_.y, other.g6_.z) + vec3(self.g3_.x) * vec3(other.g10_.y, other.g5_.z, -other.g5_.y) + vec3(self.g3_.y) * vec3(-other.g5_.z, other.g10_.y, other.g5_.x) + vec3(self.g3_.z) * vec3(other.g5_.y, -other.g5_.x, other.g10_.y) + vec3(self.g4_.x) * vec3(other.g0_.x, other.g4_.z, -other.g4_.y) + vec3(self.g4_.y) * vec3(-other.g4_.z, other.g0_.x, other.g4_.x) + vec3(self.g4_.z) * vec3(other.g4_.y, -other.g4_.x, other.g0_.x) + vec3(self.g5_.x) * vec3(other.g10_.x, other.g3_.z, -other.g3_.y) + vec3(self.g5_.y) * vec3(-other.g3_.z, other.g10_.x, other.g3_.x) + vec3(self.g5_.z) * vec3(other.g3_.y, -other.g3_.x, other.g10_.x) + vec3(self.g5_.w) * other.g9_ + vec3(self.g6_.x) * vec3(-other.g2_.y, -other.g8_.z, other.g8_.y) + vec3(self.g6_.y) * vec3(other.g8_.z, -other.g2_.y, -other.g8_.x) + vec3(self.g6_.z) * vec3(-other.g8_.y, other.g8_.x, -other.g2_.y) - vec3(self.g6_.w) * other.g1_ + vec3(self.g7_.x) * vec3(-other.g0_.y, -other.g7_.z, other.g7_.y) + vec3(self.g7_.y) * vec3(other.g7_.z, -other.g0_.y, -other.g7_.x) + vec3(self.g7_.z) * vec3(-other.g7_.y, other.g7_.x, -other.g0_.y) + vec3(self.g8_.x) * vec3(-other.g2_.x, -other.g6_.z, other.g6_.y) + vec3(self.g8_.y) * vec3(other.g6_.z, -other.g2_.x, -other.g6_.x) + vec3(self.g8_.z) * vec3(-other.g6_.y, other.g6_.x, -other.g2_.x) + vec3(self.g9_.x) * vec3(other.g5_.w, other.g9_.z, -other.g9_.y) + vec3(self.g9_.y) * vec3(-other.g9_.z, other.g5_.w, other.g9_.x) + vec3(self.g9_.z) * vec3(other.g9_.y, -other.g9_.x, other.g5_.w) + vec3(self.g10_.x) * vec3(other.g5_.x, other.g5_.y, other.g5_.z) + vec3(self.g10_.y) * other.g3_, vec4(self.g0_.x) * other.g5_ + vec4(self.g0_.y) * vec4(-other.g8_.x, -other.g8_.y, -other.g8_.z, other.g6_.w) + vec4(self.g1_.x) * vec4(other.g2_.y, other.g8_.z, -other.g8_.y, -other.g7_.x) + vec4(self.g1_.y) * vec4(-other.g8_.z, other.g2_.y, other.g8_.x, -other.g7_.y) + vec4(self.g1_.z) * vec4(other.g8_.y, -other.g8_.x, other.g2_.y, -other.g7_.z) + vec4(self.g2_.x) * vec4(0.0, 0.0, 0.0, other.g2_.y) - vec4(self.g2_.y) * vec4(other.g7_.x, other.g7_.y, other.g7_.z, other.g2_.x) + vec4(self.g2_.y) * vec4(-other.g1_.x, -other.g1_.y, -other.g1_.z, 0.0) + vec4(self.g3_.x) * vec4(0.0, 0.0, 0.0, other.g5_.x) + vec4(self.g3_.y) * vec4(0.0, 0.0, 0.0, other.g5_.y) + vec4(self.g3_.z) * vec4(0.0, 0.0, 0.0, other.g5_.z) + vec4(self.g4_.x) * vec4(other.g10_.y, other.g5_.z, -other.g5_.y, -other.g9_.x) + vec4(self.g4_.y) * vec4(-other.g5_.z, other.g10_.y, other.g5_.x, -other.g9_.y) + vec4(self.g4_.z) * vec4(other.g5_.y, -other.g5_.x, other.g10_.y, -other.g9_.z) + vec4(self.g5_.x) * vec4(-other.g5_.w, -other.g9_.z, other.g9_.y, -other.g3_.x) + vec4(self.g5_.x) * vec4(other.g0_.x, other.g4_.z, -other.g4_.y, 0.0) + vec4(self.g5_.y) * vec4(other.g9_.z, -other.g5_.w, -other.g9_.x, -other.g3_.y) + vec4(self.g5_.y) * vec4(-other.g4_.z, other.g0_.x, other.g4_.x, 0.0) + vec4(self.g5_.z) * vec4(-other.g9_.y, other.g9_.x, -other.g5_.w, -other.g3_.z) + vec4(self.g5_.z) * vec4(other.g4_.y, -other.g4_.x, other.g0_.x, 0.0) + vec4(self.g5_.w) * vec4(other.g5_.x, other.g5_.y, other.g5_.z, other.g0_.x) + vec4(self.g6_.x) * vec4(0.0, 0.0, 0.0, -other.g8_.x) + vec4(self.g6_.y) * vec4(0.0, 0.0, 0.0, -other.g8_.y) + vec4(self.g6_.z) * vec4(0.0, 0.0, 0.0, -other.g8_.z) + vec4(self.g6_.w) * vec4(other.g8_.x, other.g8_.y, other.g8_.z, other.g0_.y) + vec4(self.g7_.x) * vec4(-other.g2_.y, -other.g8_.z, other.g8_.y, -other.g1_.x) + vec4(self.g7_.y) * vec4(other.g8_.z, -other.g2_.y, -other.g8_.x, -other.g1_.y) + vec4(self.g7_.z) * vec4(-other.g8_.y, other.g8_.x, -other.g2_.y, -other.g1_.z) + vec4(self.g8_.x) * vec4(-other.g6_.w, -other.g7_.z, other.g7_.y, other.g6_.x) + vec4(self.g8_.x) * vec4(-other.g0_.y, -other.g1_.z, other.g1_.y, 0.0) + vec4(self.g8_.y) * vec4(other.g7_.z, -other.g6_.w, -other.g7_.x, other.g6_.y) + vec4(self.g8_.y) * vec4(other.g1_.z, -other.g0_.y, -other.g1_.x, 0.0) + vec4(self.g8_.z) * vec4(-other.g7_.y, other.g7_.x, -other.g6_.w, other.g6_.z) + vec4(self.g8_.z) * vec4(-other.g1_.y, other.g1_.x, -other.g0_.y, 0.0) + vec4(self.g9_.x) * vec4(other.g10_.y, other.g5_.z, -other.g5_.y, -other.g4_.x) + vec4(self.g9_.y) * vec4(-other.g5_.z, other.g10_.y, other.g5_.x, -other.g4_.y) + vec4(self.g9_.z) * vec4(other.g5_.y, -other.g5_.x, other.g10_.y, -other.g4_.z) + vec4(self.g10_.x) * vec4(0.0, 0.0, 0.0, -other.g10_.y) + vec4(self.g10_.y) * vec4(-other.g9_.x, -other.g9_.y, -other.g9_.z, other.g10_.x) + vec4(self.g10_.y) * vec4(other.g4_.x, other.g4_.y, other.g4_.z, 0.0), vec4(self.g0_.x) * other.g6_ + vec4(self.g0_.y) * vec4(other.g3_.x, other.g3_.y, other.g3_.z, -other.g5_.w) + vec4(self.g1_.x) * vec4(other.g10_.x, other.g3_.z, -other.g3_.y, -other.g4_.x) + vec4(self.g1_.y) * vec4(-other.g3_.z, other.g10_.x, other.g3_.x, -other.g4_.y) + vec4(self.g1_.z) * vec4(other.g3_.y, -other.g3_.x, other.g10_.x, -other.g4_.z) + vec4(self.g2_.x) * vec4(other.g9_.x, other.g9_.y, other.g9_.z, other.g10_.y) + vec4(self.g2_.x) * vec4(other.g4_.x, other.g4_.y, other.g4_.z, 0.0) + vec4(self.g2_.y) * vec4(0.0, 0.0, 0.0, -other.g10_.x) + vec4(self.g3_.x) * vec4(-other.g6_.w, other.g7_.z, -other.g7_.y, -other.g8_.x) + vec4(self.g3_.x) * vec4(other.g0_.y, -other.g1_.z, other.g1_.y, 0.0) + vec4(self.g3_.y) * vec4(-other.g7_.z, -other.g6_.w, other.g7_.x, -other.g8_.y) + vec4(self.g3_.y) * vec4(other.g1_.z, other.g0_.y, -other.g1_.x, 0.0) + vec4(self.g3_.z) * vec4(other.g7_.y, -other.g7_.x, -other.g6_.w, -other.g8_.z) + vec4(self.g3_.z) * vec4(-other.g1_.y, other.g1_.x, other.g0_.y, 0.0) + vec4(self.g4_.x) * vec4(other.g2_.x, other.g6_.z, -other.g6_.y, -other.g1_.x) + vec4(self.g4_.y) * vec4(-other.g6_.z, other.g2_.x, other.g6_.x, -other.g1_.y) + vec4(self.g4_.z) * vec4(other.g6_.y, -other.g6_.x, other.g2_.x, -other.g1_.z) + vec4(self.g5_.x) * vec4(0.0, 0.0, 0.0, other.g6_.x) + vec4(self.g5_.y) * vec4(0.0, 0.0, 0.0, other.g6_.y) + vec4(self.g5_.z) * vec4(0.0, 0.0, 0.0, other.g6_.z) - vec4(self.g5_.w) * vec4(other.g6_.x, other.g6_.y, other.g6_.z, other.g0_.y) + vec4(self.g6_.x) * vec4(other.g5_.w, other.g9_.z, -other.g9_.y, -other.g5_.x) + vec4(self.g6_.x) * vec4(other.g0_.x, other.g4_.z, -other.g4_.y, 0.0) + vec4(self.g6_.y) * vec4(-other.g9_.z, other.g5_.w, other.g9_.x, -other.g5_.y) + vec4(self.g6_.y) * vec4(-other.g4_.z, other.g0_.x, other.g4_.x, 0.0) + vec4(self.g6_.z) * vec4(other.g9_.y, -other.g9_.x, other.g5_.w, -other.g5_.z) + vec4(self.g6_.z) * vec4(other.g4_.y, -other.g4_.x, other.g0_.x, 0.0) + vec4(self.g6_.w) * vec4(other.g3_.x, other.g3_.y, other.g3_.z, other.g0_.x) + vec4(self.g7_.x) * vec4(other.g10_.x, other.g3_.z, -other.g3_.y, other.g9_.x) + vec4(self.g7_.y) * vec4(-other.g3_.z, other.g10_.x, other.g3_.x, other.g9_.y) + vec4(self.g7_.z) * vec4(other.g3_.y, -other.g3_.x, other.g10_.x, other.g9_.z) + vec4(self.g8_.x) * vec4(0.0, 0.0, 0.0, other.g3_.x) + vec4(self.g8_.y) * vec4(0.0, 0.0, 0.0, other.g3_.y) + vec4(self.g8_.z) * vec4(0.0, 0.0, 0.0, other.g3_.z) + vec4(self.g9_.x) * vec4(-other.g2_.x, -other.g6_.z, other.g6_.y, other.g7_.x) + vec4(self.g9_.y) * vec4(other.g6_.z, -other.g2_.x, -other.g6_.x, other.g7_.y) + vec4(self.g9_.z) * vec4(-other.g6_.y, other.g6_.x, -other.g2_.x, other.g7_.z) + vec4(self.g10_.x) * vec4(other.g7_.x, other.g7_.y, other.g7_.z, other.g2_.y) + vec4(self.g10_.x) * vec4(-other.g1_.x, -other.g1_.y, -other.g1_.z, 0.0) + vec4(self.g10_.y) * vec4(0.0, 0.0, 0.0, -other.g2_.x), vec3(self.g0_.x) * other.g7_ + vec3(self.g0_.y) * other.g4_ + vec3(self.g1_.x) * vec3(-other.g5_.w, -other.g9_.z, other.g9_.y) + vec3(self.g1_.y) * vec3(other.g9_.z, -other.g5_.w, -other.g9_.x) + vec3(self.g1_.z) * vec3(-other.g9_.y, other.g9_.x, -other.g5_.w) + vec3(self.g2_.x) * vec3(other.g5_.x, other.g5_.y, other.g5_.z) + vec3(self.g2_.y) * other.g3_ + vec3(self.g3_.x) * vec3(other.g2_.y, other.g8_.z, -other.g8_.y) + vec3(self.g3_.y) * vec3(-other.g8_.z, other.g2_.y, other.g8_.x) + vec3(self.g3_.z) * vec3(other.g8_.y, -other.g8_.x, other.g2_.y) + vec3(self.g4_.x) * vec3(other.g0_.y, other.g7_.z, -other.g7_.y) + vec3(self.g4_.y) * vec3(-other.g7_.z, other.g0_.y, other.g7_.x) + vec3(self.g4_.z) * vec3(other.g7_.y, -other.g7_.x, other.g0_.y) + vec3(self.g5_.x) * vec3(other.g2_.x, other.g6_.z, -other.g6_.y) + vec3(self.g5_.y) * vec3(-other.g6_.z, other.g2_.x, other.g6_.x) + vec3(self.g5_.z) * vec3(other.g6_.y, -other.g6_.x, other.g2_.x) - vec3(self.g5_.w) * other.g1_ + vec3(self.g6_.x) * vec3(other.g10_.y, other.g5_.z, -other.g5_.y) + vec3(self.g6_.y) * vec3(-other.g5_.z, other.g10_.y, other.g5_.x) + vec3(self.g6_.z) * vec3(other.g5_.y, -other.g5_.x, other.g10_.y) - vec3(self.g6_.w) * other.g9_ + vec3(self.g7_.x) * vec3(other.g0_.x, other.g4_.z, -other.g4_.y) + vec3(self.g7_.y) * vec3(-other.g4_.z, other.g0_.x, other.g4_.x) + vec3(self.g7_.z) * vec3(other.g4_.y, -other.g4_.x, other.g0_.x) + vec3(self.g8_.x) * vec3(other.g10_.x, other.g3_.z, -other.g3_.y) + vec3(self.g8_.y) * vec3(-other.g3_.z, other.g10_.x, other.g3_.x) + vec3(self.g8_.z) * vec3(other.g3_.y, -other.g3_.x, other.g10_.x) + vec3(self.g9_.x) * vec3(-other.g6_.w, -other.g1_.z, other.g1_.y) + vec3(self.g9_.y) * vec3(other.g1_.z, -other.g6_.w, -other.g1_.x) + vec3(self.g9_.z) * vec3(-other.g1_.y, other.g1_.x, -other.g6_.w) + vec3(self.g10_.x) * other.g8_ + vec3(self.g10_.y) * vec3(other.g6_.x, other.g6_.y, other.g6_.z), vec3(self.g0_.x) * other.g8_ + vec3(self.g0_.y) * vec3(other.g5_.x, other.g5_.y, other.g5_.z) + vec3(self.g1_.x) * vec3(-other.g10_.y, -other.g5_.z, other.g5_.y) + vec3(self.g1_.y) * vec3(other.g5_.z, -other.g10_.y, -other.g5_.x) + vec3(self.g1_.z) * vec3(-other.g5_.y, other.g5_.x, -other.g10_.y) - vec3(self.g2_.y) * other.g9_ + vec3(self.g2_.y) * other.g4_ + vec3(self.g4_.x) * vec3(other.g2_.y, other.g8_.z, -other.g8_.y) + vec3(self.g4_.y) * vec3(-other.g8_.z, other.g2_.y, other.g8_.x) + vec3(self.g4_.z) * vec3(other.g8_.y, -other.g8_.x, other.g2_.y) + vec3(self.g5_.x) * vec3(other.g6_.w, other.g7_.z, -other.g7_.y) + vec3(self.g5_.x) * vec3(other.g0_.y, other.g1_.z, -other.g1_.y) + vec3(self.g5_.y) * vec3(-other.g7_.z, other.g6_.w, other.g7_.x) + vec3(self.g5_.y) * vec3(-other.g1_.z, other.g0_.y, other.g1_.x) + vec3(self.g5_.z) * vec3(other.g7_.y, -other.g7_.x, other.g6_.w) + vec3(self.g5_.z) * vec3(other.g1_.y, -other.g1_.x, other.g0_.y) + vec3(self.g5_.w) * other.g8_ - vec3(self.g6_.w) * vec3(other.g5_.x, other.g5_.y, other.g5_.z) + vec3(self.g7_.x) * vec3(other.g10_.y, other.g5_.z, -other.g5_.y) + vec3(self.g7_.y) * vec3(-other.g5_.z, other.g10_.y, other.g5_.x) + vec3(self.g7_.z) * vec3(other.g5_.y, -other.g5_.x, other.g10_.y) + vec3(self.g8_.x) * vec3(-other.g5_.w, -other.g9_.z, other.g9_.y) + vec3(self.g8_.x) * vec3(other.g0_.x, other.g4_.z, -other.g4_.y) + vec3(self.g8_.y) * vec3(other.g9_.z, -other.g5_.w, -other.g9_.x) + vec3(self.g8_.y) * vec3(-other.g4_.z, other.g0_.x, other.g4_.x) + vec3(self.g8_.z) * vec3(-other.g9_.y, other.g9_.x, -other.g5_.w) + vec3(self.g8_.z) * vec3(other.g4_.y, -other.g4_.x, other.g0_.x) + vec3(self.g9_.x) * vec3(other.g2_.y, other.g8_.z, -other.g8_.y) + vec3(self.g9_.y) * vec3(-other.g8_.z, other.g2_.y, other.g8_.x) + vec3(self.g9_.z) * vec3(other.g8_.y, -other.g8_.x, other.g2_.y) + vec3(self.g10_.y) * other.g7_ + vec3(self.g10_.y) * other.g1_, vec3(self.g0_.x) * other.g9_ + vec3(self.g0_.y) * other.g1_ + vec3(self.g1_.x) * vec3(other.g0_.y, other.g7_.z, -other.g7_.y) + vec3(self.g1_.y) * vec3(-other.g7_.z, other.g0_.y, other.g7_.x) + vec3(self.g1_.z) * vec3(other.g7_.y, -other.g7_.x, other.g0_.y) + vec3(self.g2_.x) * other.g8_ - vec3(self.g2_.y) * vec3(other.g6_.x, other.g6_.y, other.g6_.z) + vec3(self.g3_.x) * vec3(-other.g10_.y, -other.g5_.z, other.g5_.y) + vec3(self.g3_.y) * vec3(other.g5_.z, -other.g10_.y, -other.g5_.x) + vec3(self.g3_.z) * vec3(-other.g5_.y, other.g5_.x, -other.g10_.y) + vec3(self.g4_.x) * vec3(other.g5_.w, other.g9_.z, -other.g9_.y) + vec3(self.g4_.y) * vec3(-other.g9_.z, other.g5_.w, other.g9_.x) + vec3(self.g4_.z) * vec3(other.g9_.y, -other.g9_.x, other.g5_.w) + vec3(self.g5_.x) * vec3(other.g10_.x, other.g3_.z, -other.g3_.y) + vec3(self.g5_.y) * vec3(-other.g3_.z, other.g10_.x, other.g3_.x) + vec3(self.g5_.z) * vec3(other.g3_.y, -other.g3_.x, other.g10_.x) + vec3(self.g5_.w) * other.g4_ + vec3(self.g6_.x) * vec3(other.g2_.y, other.g8_.z, -other.g8_.y) + vec3(self.g6_.y) * vec3(-other.g8_.z, other.g2_.y, other.g8_.x) + vec3(self.g6_.z) * vec3(other.g8_.y, -other.g8_.x, other.g2_.y) + vec3(self.g6_.w) * other.g7_ + vec3(self.g7_.x) * vec3(other.g6_.w, other.g1_.z, -other.g1_.y) + vec3(self.g7_.y) * vec3(-other.g1_.z, other.g6_.w, other.g1_.x) + vec3(self.g7_.z) * vec3(other.g1_.y, -other.g1_.x, other.g6_.w) + vec3(self.g8_.x) * vec3(-other.g2_.x, -other.g6_.z, other.g6_.y) + vec3(self.g8_.y) * vec3(other.g6_.z, -other.g2_.x, -other.g6_.x) + vec3(self.g8_.z) * vec3(-other.g6_.y, other.g6_.x, -other.g2_.x) + vec3(self.g9_.x) * vec3(other.g0_.x, other.g4_.z, -other.g4_.y) + vec3(self.g9_.y) * vec3(-other.g4_.z, other.g0_.x, other.g4_.x) + vec3(self.g9_.z) * vec3(other.g4_.y, -other.g4_.x, other.g0_.x) - vec3(self.g10_.x) * vec3(other.g5_.x, other.g5_.y, other.g5_.z) + vec3(self.g10_.y) * other.g3_, vec2(self.g0_.x) * other.g10_ - vec2(self.g0_.y) * other.g2_ + vec2(self.g1_.x) * vec2(other.g6_.x, -other.g8_.x) + vec2(self.g1_.y) * vec2(other.g6_.y, -other.g8_.y) + vec2(self.g1_.z) * vec2(other.g6_.z, -other.g8_.z) + self.g2_ * vec2(other.g6_.w) - self.g2_ * vec2(other.g0_.y) + vec2(self.g3_.x) * vec2(-other.g9_.x, 0.0) + vec2(self.g3_.x) * vec2(-other.g4_.x, 0.0) + vec2(self.g3_.y) * vec2(-other.g9_.y, 0.0) + vec2(self.g3_.y) * vec2(-other.g4_.y, 0.0) + vec2(self.g3_.z) * vec2(-other.g9_.z, 0.0) + vec2(self.g3_.z) * vec2(-other.g4_.z, 0.0) - vec2(self.g4_.x) * vec2(other.g3_.x, other.g5_.x) - vec2(self.g4_.y) * vec2(other.g3_.y, other.g5_.y) - vec2(self.g4_.z) * vec2(other.g3_.z, other.g5_.z) + vec2(self.g5_.x) * vec2(0.0, other.g9_.x) + vec2(self.g5_.x) * vec2(0.0, -other.g4_.x) + vec2(self.g5_.y) * vec2(0.0, other.g9_.y) + vec2(self.g5_.y) * vec2(0.0, -other.g4_.y) + vec2(self.g5_.z) * vec2(0.0, other.g9_.z) + vec2(self.g5_.z) * vec2(0.0, -other.g4_.z) + vec2(self.g5_.w) * other.g10_ * vec2(-1.0, 1.0) + vec2(self.g6_.x) * vec2(other.g7_.x, 0.0) + vec2(self.g6_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g6_.y) * vec2(other.g7_.y, 0.0) + vec2(self.g6_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g6_.z) * vec2(other.g7_.z, 0.0) + vec2(self.g6_.z) * vec2(-other.g1_.z, 0.0) + vec2(self.g6_.w) * other.g2_ * vec2(-1.0, 1.0) + vec2(self.g7_.x) * vec2(other.g6_.x, other.g8_.x) + vec2(self.g7_.y) * vec2(other.g6_.y, other.g8_.y) + vec2(self.g7_.z) * vec2(other.g6_.z, other.g8_.z) + vec2(self.g8_.x) * vec2(0.0, other.g7_.x) + vec2(self.g8_.x) * vec2(0.0, other.g1_.x) + vec2(self.g8_.y) * vec2(0.0, other.g7_.y) + vec2(self.g8_.y) * vec2(0.0, other.g1_.y) + vec2(self.g8_.z) * vec2(0.0, other.g7_.z) + vec2(self.g8_.z) * vec2(0.0, other.g1_.z) + vec2(self.g9_.x) * vec2(other.g3_.x, -other.g5_.x) + vec2(self.g9_.y) * vec2(other.g3_.y, -other.g5_.y) + vec2(self.g9_.z) * vec2(other.g3_.z, -other.g5_.z) + self.g10_ * vec2(other.g5_.w) + self.g10_ * vec2(other.g0_.x));
}

MultiVector multiVector_wedgeDot_plane(MultiVector self, Plane other) {
    return MultiVector(vec2(self.g1_.x) * vec2(0.0, other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, other.g0_.z) + vec2(self.g2_.x) * vec2(0.0, other.g0_.w) + vec2(self.g9_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g9_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g9_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g10_.x) * vec2(other.g0_.w, 0.0), vec3(0.0) - vec3(self.g0_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g6_.x, self.g6_.y, self.g6_.z) * vec3(other.g0_.w) + vec3(self.g7_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g7_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g7_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec2(self.g0_.y) * vec2(0.0, other.g0_.w) + vec2(self.g6_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g6_.y) * vec2(-other.g0_.y, 0.0) - vec2(self.g6_.z, self.g6_.w) * vec2(other.g0_.z, other.g0_.w) + vec2(self.g8_.x) * vec2(0.0, other.g0_.x) + vec2(self.g8_.y) * vec2(0.0, other.g0_.y) + vec2(self.g8_.z) * vec2(0.0, other.g0_.z), vec3(self.g3_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g3_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g3_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + vec3(self.g10_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), self.g3_ * vec3(other.g0_.w) + vec3(self.g5_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + vec3(self.g9_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g9_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g9_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec4(self.g4_.x) * vec4(other.g0_.w, 0.0, 0.0, -other.g0_.x) + vec4(self.g4_.y) * vec4(0.0, other.g0_.w, 0.0, -other.g0_.y) + vec4(self.g4_.z) * vec4(0.0, 0.0, other.g0_.w, -other.g0_.z) + vec4(self.g5_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g5_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g5_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0) + vec4(self.g9_.x, self.g9_.y, self.g9_.z, self.g9_.x) * vec4(other.g0_.w, other.g0_.w, other.g0_.w, 0.0) - vec4(self.g10_.y, self.g10_.y, self.g10_.y, self.g10_.x) * other.g0_, vec4(self.g2_.x) * other.g0_ + vec4(self.g6_.x) * vec4(0.0, other.g0_.z, -other.g0_.y, 0.0) + vec4(self.g6_.y) * vec4(-other.g0_.z, 0.0, other.g0_.x, 0.0) + vec4(self.g6_.z) * vec4(other.g0_.y, -other.g0_.x, 0.0, 0.0) + vec4(self.g7_.x) * vec4(0.0, 0.0, 0.0, other.g0_.x) + vec4(self.g7_.y) * vec4(0.0, 0.0, 0.0, other.g0_.y) + vec4(self.g7_.z) * vec4(0.0, 0.0, 0.0, other.g0_.z), vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) + vec3(self.g6_.x, self.g6_.y, self.g6_.z) * vec3(other.g0_.w) - vec3(self.g6_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(0.0) - self.g1_ * vec3(other.g0_.w) - vec3(self.g2_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + self.g7_ * vec3(other.g0_.w) + vec3(self.g8_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g8_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g8_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(self.g0_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) - self.g3_ * vec3(other.g0_.w) + vec3(self.g4_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g4_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g4_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec2(self.g0_.x) * vec2(0.0, other.g0_.w) + vec2(self.g3_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g3_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g3_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g5_.x) * vec2(0.0, other.g0_.x) + vec2(self.g5_.y) * vec2(0.0, other.g0_.y) + vec2(self.g5_.z) * vec2(0.0, other.g0_.z) + vec2(self.g5_.w) * vec2(0.0, other.g0_.w));
}

MultiVector multiVector_wedgeDot_roundPoint(MultiVector self, RoundPoint other) {
    return MultiVector(vec2(self.g1_.x) * vec2(other.g0_.x, 0.0) + vec2(self.g1_.y) * vec2(other.g0_.y, 0.0) + vec2(self.g1_.z) * vec2(other.g0_.z, 0.0) + vec2(self.g2_.x) * vec2(-other.g1_.y, 0.0) + vec2(self.g2_.y) * vec2(-other.g1_.x, 0.0) + vec2(self.g9_.x) * vec2(0.0, other.g0_.x) + vec2(self.g9_.y) * vec2(0.0, other.g0_.y) + vec2(self.g9_.z) * vec2(0.0, other.g0_.z) + vec2(self.g10_.x) * vec2(0.0, other.g1_.y) + vec2(self.g10_.y) * vec2(0.0, other.g1_.x), vec3(self.g0_.x) * other.g0_ + self.g3_ * vec3(other.g1_.y) + vec3(self.g4_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g4_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g4_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) - vec3(self.g5_.x, self.g5_.y, self.g5_.z) * vec3(other.g1_.x), vec2(self.g0_.x) * other.g1_ + vec2(self.g3_.x) * vec2(other.g0_.x, 0.0) + vec2(self.g3_.y) * vec2(other.g0_.y, 0.0) + vec2(self.g3_.z) * vec2(other.g0_.z, 0.0) + vec2(self.g5_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g5_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g5_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g5_.w) * other.g1_ * vec2(-1.0, 1.0), vec3(0.0) - self.g1_ * vec3(other.g1_.x) + vec3(self.g2_.x) * other.g0_ + vec3(self.g6_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g6_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g6_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) - self.g7_ * vec3(other.g1_.x), vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) - vec3(self.g6_.x, self.g6_.y, self.g6_.z) * vec3(other.g1_.y) - vec3(self.g6_.w) * other.g0_ - self.g8_ * vec3(other.g1_.x), vec4(self.g1_.x, self.g1_.y, self.g1_.z, self.g1_.x) * vec4(other.g1_.y, other.g1_.y, other.g1_.y, 0.0) + vec4(self.g2_.x) * vec4(0.0, 0.0, 0.0, other.g1_.y) - vec4(self.g2_.y) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.x) + vec4(self.g7_.x) * vec4(-other.g1_.y, 0.0, 0.0, -other.g0_.x) + vec4(self.g7_.y) * vec4(0.0, -other.g1_.y, 0.0, -other.g0_.y) + vec4(self.g7_.z) * vec4(0.0, 0.0, -other.g1_.y, -other.g0_.z) + vec4(self.g8_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g8_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g8_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0), vec4(self.g3_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g3_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g3_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0) + vec4(self.g4_.x) * vec4(other.g1_.x, 0.0, 0.0, -other.g0_.x) + vec4(self.g4_.y) * vec4(0.0, other.g1_.x, 0.0, -other.g0_.y) + vec4(self.g4_.z) * vec4(0.0, 0.0, other.g1_.x, -other.g0_.z) + vec4(self.g9_.x, self.g9_.y, self.g9_.z, self.g9_.x) * vec4(-other.g1_.x, -other.g1_.x, -other.g1_.x, 0.0) + vec4(self.g10_.x) * vec4(-other.g0_.x, -other.g0_.y, -other.g0_.z, other.g1_.y) + vec4(self.g10_.y) * vec4(0.0, 0.0, 0.0, -other.g1_.x), self.g3_ * vec3(other.g1_.y) + vec3(self.g5_.x, self.g5_.y, self.g5_.z) * vec3(other.g1_.x) - vec3(self.g5_.w) * other.g0_ + vec3(self.g9_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g9_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g9_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), self.g4_ * vec3(other.g1_.y) + vec3(self.g5_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g5_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g5_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + self.g9_ * vec3(other.g1_.y) + vec3(self.g10_.y) * other.g0_, vec3(self.g0_.y) * other.g0_ + vec3(self.g6_.x, self.g6_.y, self.g6_.z) * vec3(other.g1_.y) + vec3(self.g7_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g7_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g7_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) - self.g8_ * vec3(other.g1_.x), vec2(0.0) - vec2(self.g0_.y) * other.g1_ + vec2(self.g6_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g6_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g6_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g6_.w) * other.g1_ * vec2(-1.0, 1.0) + vec2(self.g8_.x) * vec2(0.0, other.g0_.x) + vec2(self.g8_.y) * vec2(0.0, other.g0_.y) + vec2(self.g8_.z) * vec2(0.0, other.g0_.z));
}

MultiVector multiVector_wedgeDot_scalar(MultiVector self, Scalar other) {
    return MultiVector(self.g0_ * vec2(other.g0_), self.g1_ * vec3(other.g0_), self.g2_ * vec2(other.g0_), self.g3_ * vec3(other.g0_), self.g4_ * vec3(other.g0_), self.g5_ * vec4(other.g0_), self.g6_ * vec4(other.g0_), self.g7_ * vec3(other.g0_), self.g8_ * vec3(other.g0_), self.g9_ * vec3(other.g0_), self.g10_ * vec2(other.g0_));
}

MultiVector multiVector_wedgeDot_sphere(MultiVector self, Sphere other) {
    return MultiVector(vec2(self.g1_.x) * vec2(0.0, other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, other.g0_.z) + vec2(self.g2_.x) * vec2(0.0, other.g1_.y) + vec2(self.g2_.y) * vec2(0.0, other.g1_.x) + vec2(self.g9_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g9_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g9_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g10_.x) * vec2(other.g1_.y, 0.0) + vec2(self.g10_.y) * vec2(other.g1_.x, 0.0), vec3(0.0) - vec3(self.g0_.y) * other.g0_ + vec3(self.g6_.x, self.g6_.y, self.g6_.z) * vec3(other.g1_.y) + vec3(self.g7_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g7_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g7_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) - self.g8_ * vec3(other.g1_.x), vec2(self.g0_.y) * other.g1_ + vec2(self.g6_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g6_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g6_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g6_.w) * other.g1_ * vec2(1.0, -1.0) + vec2(self.g8_.x) * vec2(0.0, other.g0_.x) + vec2(self.g8_.y) * vec2(0.0, other.g0_.y) + vec2(self.g8_.z) * vec2(0.0, other.g0_.z), vec3(self.g3_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g3_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g3_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + self.g4_ * vec3(other.g1_.x) - self.g9_ * vec3(other.g1_.x) + vec3(self.g10_.x) * other.g0_, self.g3_ * vec3(other.g1_.y) + vec3(self.g5_.x, self.g5_.y, self.g5_.z) * vec3(other.g1_.x) + vec3(self.g5_.w) * other.g0_ + vec3(self.g9_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g9_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g9_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec4(self.g4_.x) * vec4(other.g1_.y, 0.0, 0.0, -other.g0_.x) + vec4(self.g4_.y) * vec4(0.0, other.g1_.y, 0.0, -other.g0_.y) + vec4(self.g4_.z) * vec4(0.0, 0.0, other.g1_.y, -other.g0_.z) + vec4(self.g5_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g5_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g5_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0) + vec4(self.g9_.x, self.g9_.y, self.g9_.z, self.g9_.x) * vec4(other.g1_.y, other.g1_.y, other.g1_.y, 0.0) + vec4(self.g10_.x) * vec4(0.0, 0.0, 0.0, -other.g1_.y) + vec4(self.g10_.y) * vec4(-other.g0_.x, -other.g0_.y, -other.g0_.z, other.g1_.x), vec4(self.g1_.x, self.g1_.y, self.g1_.z, self.g1_.x) * vec4(other.g1_.x, other.g1_.x, other.g1_.x, 0.0) + vec4(self.g2_.x) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.y) + vec4(self.g2_.y) * vec4(0.0, 0.0, 0.0, -other.g1_.x) + vec4(self.g6_.x) * vec4(0.0, other.g0_.z, -other.g0_.y, 0.0) + vec4(self.g6_.y) * vec4(-other.g0_.z, 0.0, other.g0_.x, 0.0) + vec4(self.g6_.z) * vec4(other.g0_.y, -other.g0_.x, 0.0, 0.0) + vec4(self.g7_.x) * vec4(other.g1_.x, 0.0, 0.0, other.g0_.x) + vec4(self.g7_.y) * vec4(0.0, other.g1_.x, 0.0, other.g0_.y) + vec4(self.g7_.z) * vec4(0.0, 0.0, other.g1_.x, other.g0_.z), vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) + vec3(self.g6_.x, self.g6_.y, self.g6_.z) * vec3(other.g1_.y) - vec3(self.g6_.w) * other.g0_ + self.g8_ * vec3(other.g1_.x), vec3(0.0) - self.g1_ * vec3(other.g1_.y) - vec3(self.g2_.y) * other.g0_ + self.g7_ * vec3(other.g1_.y) + vec3(self.g8_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g8_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g8_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(self.g0_.x) * other.g0_ - self.g3_ * vec3(other.g1_.y) + vec3(self.g4_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g4_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g4_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + vec3(self.g5_.x, self.g5_.y, self.g5_.z) * vec3(other.g1_.x), vec2(self.g0_.x) * other.g1_ + vec2(self.g3_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g3_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g3_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g5_.x) * vec2(0.0, other.g0_.x) + vec2(self.g5_.y) * vec2(0.0, other.g0_.y) + vec2(self.g5_.z) * vec2(0.0, other.g0_.z) + vec2(self.g5_.w) * other.g1_ * vec2(-1.0, 1.0));
}

RoundPoint plane_wedgeDot_antiScalar(Plane self, AntiScalar other) {
    return RoundPoint(vec3(0.0) - vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_), vec2(self.g0_.w) * vec2(0.0, other.g0_));
}

MultiVector plane_wedgeDot_circle(Plane self, Circle other) {
    return MultiVector(vec2(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) - vec3(self.g0_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g0_.x) * vec2(other.g0_.x, -other.g2_.x) + vec2(self.g0_.y) * vec2(other.g0_.y, -other.g2_.y) + vec2(self.g0_.z) * vec2(other.g0_.z, -other.g2_.z) + vec2(self.g0_.w) * vec2(0.0, other.g0_.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, other.g1_.x) + vec4(self.g0_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, other.g1_.y) + vec4(self.g0_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, other.g1_.z), vec3(0.0) - vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_.w) + vec3(self.g0_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(self.g0_.x) * vec3(0.0, other.g2_.z, -other.g2_.y) + vec3(self.g0_.y) * vec3(-other.g2_.z, 0.0, other.g2_.x) + vec3(self.g0_.z) * vec3(other.g2_.y, -other.g2_.x, 0.0) + vec3(self.g0_.w) * other.g1_, vec3(0.0), vec2(0.0));
}

MultiVector plane_wedgeDot_dipole(Plane self, Dipole other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g2_.w) + vec3(self.g0_.w) * other.g0_, vec4(self.g0_.x) * vec4(0.0, other.g2_.z, -other.g2_.y, -other.g1_.x) + vec4(self.g0_.y) * vec4(-other.g2_.z, 0.0, other.g2_.x, -other.g1_.y) + vec4(self.g0_.z) * vec4(other.g2_.y, -other.g2_.x, 0.0, -other.g1_.z) + vec4(self.g0_.w) * vec4(other.g1_.x, other.g1_.y, other.g1_.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g0_.w) * other.g0_, vec2(self.g0_.x) * vec2(other.g0_.x, -other.g2_.x) + vec2(self.g0_.y) * vec2(other.g0_.y, -other.g2_.y) + vec2(self.g0_.z) * vec2(other.g0_.z, -other.g2_.z) + vec2(self.g0_.w) * vec2(0.0, -other.g2_.w));
}

MultiVector plane_wedgeDot_dualNum(Plane self, DualNum other) {
    return MultiVector(vec2(0.0), vec3(0.0) - vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_.y), vec2(self.g0_.w) * vec2(0.0, other.g0_.y), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_.x), vec2(self.g0_.w) * vec2(0.0, other.g0_.x));
}

MultiVector plane_wedgeDot_flatPoint(Plane self, FlatPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_.w), vec4(self.g0_.x) * vec4(0.0, other.g0_.z, -other.g0_.y, 0.0) + vec4(self.g0_.y) * vec4(-other.g0_.z, 0.0, other.g0_.x, 0.0) + vec4(self.g0_.z) * vec4(other.g0_.y, -other.g0_.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(self.g0_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g0_.w) * vec2(0.0, -other.g0_.w));
}

MultiVector plane_wedgeDot_flector(Plane self, Flector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g1_.z, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(self.g0_.x) * vec3(other.g0_.w, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, other.g0_.w, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, other.g0_.w), vec4(self.g0_.x) * vec4(other.g1_.w, other.g0_.z, -other.g0_.y, 0.0) + vec4(self.g0_.y) * vec4(-other.g0_.z, other.g1_.w, other.g0_.x, 0.0) + vec4(self.g0_.z) * vec4(other.g0_.y, -other.g0_.x, other.g1_.w, 0.0) + vec4(self.g0_.w) * vec4(-other.g1_.x, -other.g1_.y, -other.g1_.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(self.g0_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g0_.w) * vec2(0.0, -other.g0_.w));
}

MultiVector plane_wedgeDot_line(Plane self, Line other) {
    return MultiVector(vec2(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec2(self.g0_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g1_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, other.g0_.x) + vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, other.g0_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, 0.0, other.g0_.z), vec3(0.0), vec3(self.g0_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g0_.w) * other.g0_, vec3(0.0), vec2(0.0));
}

MultiVector plane_wedgeDot_motor(Plane self, Motor other) {
    return MultiVector(vec2(0.0), vec3(self.g0_.x) * vec3(-other.g0_.w, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, -other.g0_.w, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, -other.g0_.w), vec2(self.g0_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g0_.w) * vec2(0.0, other.g0_.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, other.g0_.x) + vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, other.g0_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, 0.0, other.g0_.z), vec3(0.0), vec3(self.g0_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g0_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(0.0), vec2(0.0));
}

MultiVector plane_wedgeDot_multiVector(Plane self, MultiVector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(-other.g9_.x, other.g1_.x) + vec2(self.g0_.y) * vec2(-other.g9_.y, other.g1_.y) + vec2(self.g0_.z) * vec2(-other.g9_.z, other.g1_.z) + vec2(self.g0_.w) * vec2(other.g10_.x, other.g2_.x), vec3(self.g0_.x) * vec3(-other.g0_.y, -other.g7_.z, other.g7_.y) + vec3(self.g0_.y) * vec3(other.g7_.z, -other.g0_.y, -other.g7_.x) + vec3(self.g0_.z) * vec3(-other.g7_.y, other.g7_.x, -other.g0_.y) - vec3(self.g0_.w) * vec3(other.g6_.x, other.g6_.y, other.g6_.z), vec2(self.g0_.x) * vec2(other.g6_.x, -other.g8_.x) + vec2(self.g0_.y) * vec2(other.g6_.y, -other.g8_.y) + vec2(self.g0_.z) * vec2(other.g6_.z, -other.g8_.z) + vec2(self.g0_.w) * vec2(0.0, other.g6_.w) + vec2(self.g0_.w) * vec2(0.0, other.g0_.y), vec3(self.g0_.x) * vec3(-other.g10_.x, -other.g3_.z, other.g3_.y) + vec3(self.g0_.y) * vec3(other.g3_.z, -other.g10_.x, -other.g3_.x) + vec3(self.g0_.z) * vec3(-other.g3_.y, other.g3_.x, -other.g10_.x), vec3(self.g0_.x) * vec3(other.g5_.w, other.g9_.z, -other.g9_.y) + vec3(self.g0_.y) * vec3(-other.g9_.z, other.g5_.w, other.g9_.x) + vec3(self.g0_.z) * vec3(other.g9_.y, -other.g9_.x, other.g5_.w) + vec3(self.g0_.w) * other.g3_, vec4(self.g0_.x) * vec4(other.g10_.y, other.g5_.z, -other.g5_.y, -other.g4_.x) + vec4(self.g0_.y) * vec4(-other.g5_.z, other.g10_.y, other.g5_.x, -other.g4_.y) + vec4(self.g0_.z) * vec4(other.g5_.y, -other.g5_.x, other.g10_.y, -other.g4_.z) + vec4(self.g0_.w) * vec4(-other.g9_.x, -other.g9_.y, -other.g9_.z, other.g10_.x) + vec4(self.g0_.w) * vec4(other.g4_.x, other.g4_.y, other.g4_.z, 0.0), vec4(self.g0_.x) * vec4(-other.g2_.x, -other.g6_.z, other.g6_.y, other.g7_.x) + vec4(self.g0_.y) * vec4(other.g6_.z, -other.g2_.x, -other.g6_.x, other.g7_.y) + vec4(self.g0_.z) * vec4(-other.g6_.y, other.g6_.x, -other.g2_.x, other.g7_.z) + vec4(self.g0_.w) * vec4(0.0, 0.0, 0.0, -other.g2_.x), vec3(self.g0_.x) * vec3(-other.g6_.w, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, -other.g6_.w, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, -other.g6_.w) + vec3(self.g0_.w) * vec3(other.g6_.x, other.g6_.y, other.g6_.z), vec3(self.g0_.x) * vec3(other.g2_.y, other.g8_.z, -other.g8_.y) + vec3(self.g0_.y) * vec3(-other.g8_.z, other.g2_.y, other.g8_.x) + vec3(self.g0_.z) * vec3(other.g8_.y, -other.g8_.x, other.g2_.y) + vec3(self.g0_.w) * other.g7_ + vec3(self.g0_.w) * other.g1_, vec3(self.g0_.x) * vec3(other.g0_.x, other.g4_.z, -other.g4_.y) + vec3(self.g0_.y) * vec3(-other.g4_.z, other.g0_.x, other.g4_.x) + vec3(self.g0_.z) * vec3(other.g4_.y, -other.g4_.x, other.g0_.x) + vec3(self.g0_.w) * other.g3_, vec2(self.g0_.x) * vec2(other.g3_.x, -other.g5_.x) + vec2(self.g0_.y) * vec2(other.g3_.y, -other.g5_.y) + vec2(self.g0_.z) * vec2(other.g3_.z, -other.g5_.z) + vec2(self.g0_.w) * vec2(0.0, -other.g5_.w) + vec2(self.g0_.w) * vec2(0.0, other.g0_.x));
}

MultiVector plane_wedgeDot_plane(Plane self, Plane other) {
    return MultiVector(vec2(self.g0_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g0_.z, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), self.g0_.xyzx * vec4(other.g0_.w, other.g0_.w, other.g0_.w, 0.0) + vec4(self.g0_.w) * vec4(-other.g0_.x, -other.g0_.y, -other.g0_.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector plane_wedgeDot_roundPoint(Plane self, RoundPoint other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, other.g0_.z) + vec2(self.g0_.w) * vec2(0.0, other.g1_.x), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0) - self.g0_ * vec4(other.g1_.x), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g1_.y) + vec3(self.g0_.w) * other.g0_, vec3(0.0), vec2(0.0));
}

Plane plane_wedgeDot_scalar(Plane self, Scalar other) {
    return Plane(self.g0_ * vec4(other.g0_));
}

MultiVector plane_wedgeDot_sphere(Plane self, Sphere other) {
    return MultiVector(vec2(self.g0_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g0_.w) * vec2(other.g1_.x, 0.0), vec3(0.0), vec2(0.0), vec3(0.0) - vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g1_.x), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), self.g0_.xyzx * vec4(other.g1_.y, other.g1_.y, other.g1_.y, 0.0) + vec4(self.g0_.w) * vec4(-other.g0_.x, -other.g0_.y, -other.g0_.z, other.g1_.x), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

Sphere roundPoint_wedgeDot_antiScalar(RoundPoint self, AntiScalar other) {
    return Sphere(self.g0_ * vec3(other.g0_), vec2(0.0) - self.g1_ * vec2(other.g0_));
}

MultiVector roundPoint_wedgeDot_circle(RoundPoint self, Circle other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) - vec3(self.g1_.x) * other.g1_, vec3(0.0) - self.g0_ * vec3(other.g0_.w) - vec3(self.g1_.x) * other.g2_ - vec3(self.g1_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec4(self.g0_.x) * vec4(0.0, other.g2_.z, -other.g2_.y, -other.g1_.x) + vec4(self.g0_.y) * vec4(-other.g2_.z, 0.0, other.g2_.x, -other.g1_.y) + vec4(self.g0_.z) * vec4(other.g2_.y, -other.g2_.x, 0.0, -other.g1_.z) + vec4(self.g1_.y) * vec4(-other.g1_.x, -other.g1_.y, -other.g1_.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g1_.x) * other.g2_ - vec3(self.g1_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g0_.x) * vec2(other.g0_.x, -other.g2_.x) + vec2(self.g0_.y) * vec2(other.g0_.y, -other.g2_.y) + vec2(self.g0_.z) * vec2(other.g0_.z, -other.g2_.z) + self.g1_ * vec2(other.g0_.w));
}

MultiVector roundPoint_wedgeDot_dipole(RoundPoint self, Dipole other) {
    return MultiVector(vec2(0.0), vec3(self.g0_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g1_.x) * vec3(other.g2_.x, other.g2_.y, other.g2_.z) - vec3(self.g1_.y) * other.g0_, vec2(self.g0_.x) * vec2(-other.g0_.x, other.g2_.x) + vec2(self.g0_.y) * vec2(-other.g0_.y, other.g2_.y) + vec2(self.g0_.z) * vec2(-other.g0_.z, other.g2_.z) + self.g1_ * vec2(other.g2_.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(0.0, other.g0_.z, -other.g0_.y, -other.g1_.x) + vec4(self.g0_.y) * vec4(-other.g0_.z, 0.0, other.g0_.x, -other.g1_.y) + vec4(self.g0_.z) * vec4(other.g0_.y, -other.g0_.x, 0.0, -other.g1_.z) + vec4(self.g1_.x) * vec4(other.g1_.x, other.g1_.y, other.g1_.z, 0.0), vec3(0.0) - self.g0_ * vec3(other.g2_.w) + vec3(self.g1_.x) * vec3(other.g2_.x, other.g2_.y, other.g2_.z) + vec3(self.g1_.y) * other.g0_, vec3(self.g0_.x) * vec3(0.0, -other.g2_.z, other.g2_.y) + vec3(self.g0_.y) * vec3(other.g2_.z, 0.0, -other.g2_.x) + vec3(self.g0_.z) * vec3(-other.g2_.y, other.g2_.x, 0.0) + vec3(self.g1_.y) * other.g1_, vec3(0.0), vec2(0.0));
}

MultiVector roundPoint_wedgeDot_dualNum(RoundPoint self, DualNum other) {
    return MultiVector(vec2(0.0), self.g0_ * vec3(other.g0_.x), self.g1_ * vec2(other.g0_.x), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), self.g0_ * vec3(other.g0_.y), vec2(0.0) - self.g1_ * vec2(other.g0_.y));
}

MultiVector roundPoint_wedgeDot_flatPoint(RoundPoint self, FlatPoint other) {
    return MultiVector(vec2(0.0), vec3(self.g1_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g0_.x) * vec2(0.0, other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, other.g0_.z) + self.g1_ * vec2(other.g0_.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0) - self.g0_ * vec3(other.g0_.w) + vec3(self.g1_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(0.0), vec2(0.0));
}

MultiVector roundPoint_wedgeDot_flector(RoundPoint self, Flector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, other.g1_.z) + vec2(self.g1_.x) * vec2(0.0, other.g1_.w), vec3(self.g1_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g0_.x) * vec2(0.0, other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, other.g0_.z) + self.g1_ * vec2(other.g0_.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g1_.x) * other.g1_, vec3(self.g0_.x) * vec3(-other.g0_.w, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, -other.g0_.w, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, -other.g0_.w) + vec3(self.g1_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(self.g0_.x) * vec3(-other.g1_.w, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, -other.g1_.w, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, -other.g1_.w) - vec3(self.g1_.y) * vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec3(0.0), vec2(0.0));
}

MultiVector roundPoint_wedgeDot_line(RoundPoint self, Line other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0) - vec3(self.g1_.x) * other.g0_, vec3(0.0) - vec3(self.g1_.x) * other.g1_, vec4(self.g0_.x) * vec4(0.0, other.g1_.z, -other.g1_.y, -other.g0_.x) + vec4(self.g0_.y) * vec4(-other.g1_.z, 0.0, other.g1_.x, -other.g0_.y) + vec4(self.g0_.z) * vec4(other.g1_.y, -other.g1_.x, 0.0, -other.g0_.z) + vec4(self.g1_.y) * vec4(-other.g0_.x, -other.g0_.y, -other.g0_.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + vec3(self.g1_.x) * other.g1_, vec2(self.g0_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g1_.z));
}

MultiVector roundPoint_wedgeDot_motor(RoundPoint self, Motor other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0) - vec3(self.g1_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(0.0) - vec3(self.g1_.x) * other.g1_, vec4(self.g0_.x) * vec4(0.0, other.g1_.z, -other.g1_.y, -other.g0_.x) + vec4(self.g0_.y) * vec4(-other.g1_.z, 0.0, other.g1_.x, -other.g0_.y) + vec4(self.g0_.z) * vec4(other.g1_.y, -other.g1_.x, 0.0, -other.g0_.z) + vec4(self.g1_.y) * vec4(-other.g0_.x, -other.g0_.y, -other.g0_.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x) * vec3(other.g0_.w, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, other.g0_.w, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, other.g0_.w) + vec3(self.g1_.x) * other.g1_, vec2(self.g0_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g1_.z) - self.g1_ * vec2(other.g0_.w));
}

MultiVector roundPoint_wedgeDot_multiVector(RoundPoint self, MultiVector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(other.g1_.x, other.g9_.x) + vec2(self.g0_.y) * vec2(other.g1_.y, other.g9_.y) + vec2(self.g0_.z) * vec2(other.g1_.z, other.g9_.z) + vec2(self.g1_.x) * vec2(-other.g2_.y, other.g10_.y) + vec2(self.g1_.y) * vec2(-other.g2_.x, other.g10_.x), vec3(self.g0_.x) * vec3(other.g0_.x, other.g4_.z, -other.g4_.y) + vec3(self.g0_.y) * vec3(-other.g4_.z, other.g0_.x, other.g4_.x) + vec3(self.g0_.z) * vec3(other.g4_.y, -other.g4_.x, other.g0_.x) + vec3(self.g1_.x) * vec3(other.g5_.x, other.g5_.y, other.g5_.z) - vec3(self.g1_.y) * other.g3_, vec2(self.g0_.x) * vec2(-other.g3_.x, other.g5_.x) + vec2(self.g0_.y) * vec2(-other.g3_.y, other.g5_.y) + vec2(self.g0_.z) * vec2(-other.g3_.z, other.g5_.z) + self.g1_ * vec2(other.g5_.w) + self.g1_ * vec2(other.g0_.x), vec3(self.g0_.x) * vec3(-other.g2_.x, -other.g6_.z, other.g6_.y) + vec3(self.g0_.y) * vec3(other.g6_.z, -other.g2_.x, -other.g6_.x) + vec3(self.g0_.z) * vec3(-other.g6_.y, other.g6_.x, -other.g2_.x) - vec3(self.g1_.x) * other.g7_ + vec3(self.g1_.x) * other.g1_, vec3(self.g0_.x) * vec3(-other.g6_.w, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, -other.g6_.w, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, -other.g6_.w) - vec3(self.g1_.x) * other.g8_ - vec3(self.g1_.y) * vec3(other.g6_.x, other.g6_.y, other.g6_.z), vec4(self.g0_.x) * vec4(other.g2_.y, other.g8_.z, -other.g8_.y, -other.g7_.x) + vec4(self.g0_.y) * vec4(-other.g8_.z, other.g2_.y, other.g8_.x, -other.g7_.y) + vec4(self.g0_.z) * vec4(other.g8_.y, -other.g8_.x, other.g2_.y, -other.g7_.z) + vec4(self.g1_.x) * vec4(0.0, 0.0, 0.0, other.g2_.y) - vec4(self.g1_.y) * vec4(other.g7_.x, other.g7_.y, other.g7_.z, other.g2_.x) + vec4(self.g1_.y) * vec4(-other.g1_.x, -other.g1_.y, -other.g1_.z, 0.0), vec4(self.g0_.x) * vec4(other.g10_.x, other.g3_.z, -other.g3_.y, -other.g4_.x) + vec4(self.g0_.y) * vec4(-other.g3_.z, other.g10_.x, other.g3_.x, -other.g4_.y) + vec4(self.g0_.z) * vec4(other.g3_.y, -other.g3_.x, other.g10_.x, -other.g4_.z) + vec4(self.g1_.x) * vec4(other.g9_.x, other.g9_.y, other.g9_.z, other.g10_.y) + vec4(self.g1_.x) * vec4(other.g4_.x, other.g4_.y, other.g4_.z, 0.0) + vec4(self.g1_.y) * vec4(0.0, 0.0, 0.0, -other.g10_.x), vec3(self.g0_.x) * vec3(-other.g5_.w, -other.g9_.z, other.g9_.y) + vec3(self.g0_.y) * vec3(other.g9_.z, -other.g5_.w, -other.g9_.x) + vec3(self.g0_.z) * vec3(-other.g9_.y, other.g9_.x, -other.g5_.w) + vec3(self.g1_.x) * vec3(other.g5_.x, other.g5_.y, other.g5_.z) + vec3(self.g1_.y) * other.g3_, vec3(self.g0_.x) * vec3(-other.g10_.y, -other.g5_.z, other.g5_.y) + vec3(self.g0_.y) * vec3(other.g5_.z, -other.g10_.y, -other.g5_.x) + vec3(self.g0_.z) * vec3(-other.g5_.y, other.g5_.x, -other.g10_.y) - vec3(self.g1_.y) * other.g9_ + vec3(self.g1_.y) * other.g4_, vec3(self.g0_.x) * vec3(other.g0_.y, other.g7_.z, -other.g7_.y) + vec3(self.g0_.y) * vec3(-other.g7_.z, other.g0_.y, other.g7_.x) + vec3(self.g0_.z) * vec3(other.g7_.y, -other.g7_.x, other.g0_.y) + vec3(self.g1_.x) * other.g8_ - vec3(self.g1_.y) * vec3(other.g6_.x, other.g6_.y, other.g6_.z), vec2(self.g0_.x) * vec2(other.g6_.x, -other.g8_.x) + vec2(self.g0_.y) * vec2(other.g6_.y, -other.g8_.y) + vec2(self.g0_.z) * vec2(other.g6_.z, -other.g8_.z) + self.g1_ * vec2(other.g6_.w) - self.g1_ * vec2(other.g0_.y));
}

MultiVector roundPoint_wedgeDot_plane(RoundPoint self, Plane other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, other.g0_.z) + vec2(self.g1_.x) * vec2(0.0, other.g0_.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g1_.x) * other.g0_, vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(0.0) - self.g0_ * vec3(other.g0_.w) - vec3(self.g1_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(0.0), vec2(0.0));
}

MultiVector roundPoint_wedgeDot_roundPoint(RoundPoint self, RoundPoint other) {
    return MultiVector(vec2(self.g0_.x) * vec2(other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(other.g0_.z, 0.0) + vec2(self.g1_.x) * vec2(-other.g1_.y, 0.0) + vec2(self.g1_.y) * vec2(-other.g1_.x, 0.0), vec3(0.0), vec2(0.0), vec3(0.0) - self.g0_ * vec3(other.g1_.x) + vec3(self.g1_.x) * other.g0_, vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec4(self.g0_.x, self.g0_.y, self.g0_.z, self.g0_.x) * vec4(other.g1_.y, other.g1_.y, other.g1_.y, 0.0) + vec4(self.g1_.x) * vec4(0.0, 0.0, 0.0, other.g1_.y) - vec4(self.g1_.y) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.x), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

RoundPoint roundPoint_wedgeDot_scalar(RoundPoint self, Scalar other) {
    return RoundPoint(self.g0_ * vec3(other.g0_), self.g1_ * vec2(other.g0_));
}

MultiVector roundPoint_wedgeDot_sphere(RoundPoint self, Sphere other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, other.g0_.z) + vec2(self.g1_.x) * vec2(0.0, other.g1_.y) + vec2(self.g1_.y) * vec2(0.0, other.g1_.x), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x, self.g0_.y, self.g0_.z, self.g0_.x) * vec4(other.g1_.x, other.g1_.x, other.g1_.x, 0.0) + vec4(self.g1_.x) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.y) + vec4(self.g1_.y) * vec4(0.0, 0.0, 0.0, -other.g1_.x), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(0.0) - self.g0_ * vec3(other.g1_.y) - vec3(self.g1_.y) * other.g0_, vec3(0.0), vec2(0.0));
}

AntiScalar scalar_wedgeDot_antiScalar(Scalar self, AntiScalar other) {
    return AntiScalar(self.g0_ * other.g0_);
}

Circle scalar_wedgeDot_circle(Scalar self, Circle other) {
    return Circle(vec4(self.g0_) * other.g0_, vec3(self.g0_) * other.g1_, vec3(self.g0_) * other.g2_);
}

Dipole scalar_wedgeDot_dipole(Scalar self, Dipole other) {
    return Dipole(vec3(self.g0_) * other.g0_, vec3(self.g0_) * other.g1_, vec4(self.g0_) * other.g2_);
}

DualNum scalar_wedgeDot_dualNum(Scalar self, DualNum other) {
    return DualNum(vec2(self.g0_) * other.g0_);
}

FlatPoint scalar_wedgeDot_flatPoint(Scalar self, FlatPoint other) {
    return FlatPoint(vec4(self.g0_) * other.g0_);
}

Flector scalar_wedgeDot_flector(Scalar self, Flector other) {
    return Flector(vec4(self.g0_) * other.g0_, vec4(self.g0_) * other.g1_);
}

Line scalar_wedgeDot_line(Scalar self, Line other) {
    return Line(vec3(self.g0_) * other.g0_, vec3(self.g0_) * other.g1_);
}

Motor scalar_wedgeDot_motor(Scalar self, Motor other) {
    return Motor(vec4(self.g0_) * other.g0_, vec3(self.g0_) * other.g1_);
}

MultiVector scalar_wedgeDot_multiVector(Scalar self, MultiVector other) {
    return MultiVector(vec2(self.g0_) * other.g0_, vec3(self.g0_) * other.g1_, vec2(self.g0_) * other.g2_, vec3(self.g0_) * other.g3_, vec3(self.g0_) * other.g4_, vec4(self.g0_) * other.g5_, vec4(self.g0_) * other.g6_, vec3(self.g0_) * other.g7_, vec3(self.g0_) * other.g8_, vec3(self.g0_) * other.g9_, vec2(self.g0_) * other.g10_);
}

Plane scalar_wedgeDot_plane(Scalar self, Plane other) {
    return Plane(vec4(self.g0_) * other.g0_);
}

RoundPoint scalar_wedgeDot_roundPoint(Scalar self, RoundPoint other) {
    return RoundPoint(vec3(self.g0_) * other.g0_, vec2(self.g0_) * other.g1_);
}

Scalar scalar_wedgeDot_scalar(Scalar self, Scalar other) {
    return Scalar(self.g0_ * other.g0_);
}

Sphere scalar_wedgeDot_sphere(Scalar self, Sphere other) {
    return Sphere(vec3(self.g0_) * other.g0_, vec2(self.g0_) * other.g1_);
}

RoundPoint sphere_wedgeDot_antiScalar(Sphere self, AntiScalar other) {
    return RoundPoint(vec3(0.0) - self.g0_ * vec3(other.g0_), self.g1_ * vec2(other.g0_));
}

MultiVector sphere_wedgeDot_circle(Sphere self, Circle other) {
    return MultiVector(vec2(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g1_.x) * other.g2_ - vec3(self.g1_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g0_.x) * vec2(other.g0_.x, -other.g2_.x) + vec2(self.g0_.y) * vec2(other.g0_.y, -other.g2_.y) + vec2(self.g0_.z) * vec2(other.g0_.z, -other.g2_.z) + self.g1_ * vec2(-other.g0_.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, other.g1_.x) + vec4(self.g0_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, other.g1_.y) + vec4(self.g0_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, other.g1_.z) + vec4(self.g1_.x) * vec4(other.g1_.x, other.g1_.y, other.g1_.z, 0.0), vec3(0.0) - self.g0_ * vec3(other.g0_.w) + vec3(self.g1_.x) * other.g2_ + vec3(self.g1_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(self.g0_.x) * vec3(0.0, other.g2_.z, -other.g2_.y) + vec3(self.g0_.y) * vec3(-other.g2_.z, 0.0, other.g2_.x) + vec3(self.g0_.z) * vec3(other.g2_.y, -other.g2_.x, 0.0) + vec3(self.g1_.y) * other.g1_, vec3(0.0), vec2(0.0));
}

MultiVector sphere_wedgeDot_dipole(Sphere self, Dipole other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) + vec3(self.g1_.x) * other.g1_, self.g0_ * vec3(other.g2_.w) + vec3(self.g1_.x) * vec3(other.g2_.x, other.g2_.y, other.g2_.z) + vec3(self.g1_.y) * other.g0_, vec4(self.g0_.x) * vec4(0.0, other.g2_.z, -other.g2_.y, -other.g1_.x) + vec4(self.g0_.y) * vec4(-other.g2_.z, 0.0, other.g2_.x, -other.g1_.y) + vec4(self.g0_.z) * vec4(other.g2_.y, -other.g2_.x, 0.0, -other.g1_.z) + vec4(self.g1_.y) * vec4(other.g1_.x, other.g1_.y, other.g1_.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) - vec3(self.g1_.x) * vec3(other.g2_.x, other.g2_.y, other.g2_.z) + vec3(self.g1_.y) * other.g0_, vec2(self.g0_.x) * vec2(other.g0_.x, -other.g2_.x) + vec2(self.g0_.y) * vec2(other.g0_.y, -other.g2_.y) + vec2(self.g0_.z) * vec2(other.g0_.z, -other.g2_.z) + self.g1_ * vec2(other.g2_.w));
}

MultiVector sphere_wedgeDot_dualNum(Sphere self, DualNum other) {
    return MultiVector(vec2(0.0), vec3(0.0) - self.g0_ * vec3(other.g0_.y), self.g1_ * vec2(other.g0_.y), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), self.g0_ * vec3(other.g0_.x), self.g1_ * vec2(other.g0_.x));
}

MultiVector sphere_wedgeDot_flatPoint(Sphere self, FlatPoint other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(0.0), self.g0_ * vec3(other.g0_.w) + vec3(self.g1_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec4(self.g0_.x) * vec4(0.0, other.g0_.z, -other.g0_.y, 0.0) + vec4(self.g0_.y) * vec4(-other.g0_.z, 0.0, other.g0_.x, 0.0) + vec4(self.g0_.z) * vec4(other.g0_.y, -other.g0_.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - vec3(self.g1_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g0_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g0_.z) + self.g1_ * vec2(other.g0_.w));
}

MultiVector sphere_wedgeDot_flector(Sphere self, Flector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g1_.z, 0.0) + vec2(self.g1_.x) * vec2(other.g1_.w, 0.0), vec3(0.0), vec2(0.0), vec3(self.g1_.x) * vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec3(self.g0_.x) * vec3(other.g0_.w, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, other.g0_.w, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, other.g0_.w) + vec3(self.g1_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec4(self.g0_.x) * vec4(other.g1_.w, other.g0_.z, -other.g0_.y, 0.0) + vec4(self.g0_.y) * vec4(-other.g0_.z, other.g1_.w, other.g0_.x, 0.0) + vec4(self.g0_.z) * vec4(other.g0_.y, -other.g0_.x, other.g1_.w, 0.0) - vec4(self.g1_.y, self.g1_.y, self.g1_.y, self.g1_.x) * other.g1_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0) - vec3(self.g1_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g0_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g0_.z) + self.g1_ * vec2(other.g0_.w));
}

MultiVector sphere_wedgeDot_line(Sphere self, Line other) {
    return MultiVector(vec2(0.0), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) + vec3(self.g1_.x) * other.g1_, vec2(self.g0_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g1_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, other.g0_.x) + vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, other.g0_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, 0.0, other.g0_.z) + vec4(self.g1_.x) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, 0.0), vec3(self.g1_.x) * other.g1_, vec3(self.g0_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g1_.y) * other.g0_, vec3(0.0), vec2(0.0));
}

MultiVector sphere_wedgeDot_motor(Sphere self, Motor other) {
    return MultiVector(vec2(0.0), vec3(self.g0_.x) * vec3(-other.g0_.w, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, -other.g0_.w, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, -other.g0_.w) + vec3(self.g1_.x) * other.g1_, vec2(self.g0_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g1_.z) + self.g1_ * vec2(other.g0_.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * vec4(0.0, 0.0, 0.0, other.g0_.x) + vec4(self.g0_.y) * vec4(0.0, 0.0, 0.0, other.g0_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, 0.0, other.g0_.z) + vec4(self.g1_.x) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, 0.0), vec3(self.g1_.x) * other.g1_, vec3(self.g0_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g1_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(0.0), vec2(0.0));
}

MultiVector sphere_wedgeDot_multiVector(Sphere self, MultiVector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(-other.g9_.x, other.g1_.x) + vec2(self.g0_.y) * vec2(-other.g9_.y, other.g1_.y) + vec2(self.g0_.z) * vec2(-other.g9_.z, other.g1_.z) + vec2(self.g1_.x) * vec2(other.g10_.y, other.g2_.y) + vec2(self.g1_.y) * vec2(other.g10_.x, other.g2_.x), vec3(self.g0_.x) * vec3(-other.g0_.y, -other.g7_.z, other.g7_.y) + vec3(self.g0_.y) * vec3(other.g7_.z, -other.g0_.y, -other.g7_.x) + vec3(self.g0_.z) * vec3(-other.g7_.y, other.g7_.x, -other.g0_.y) + vec3(self.g1_.x) * other.g8_ - vec3(self.g1_.y) * vec3(other.g6_.x, other.g6_.y, other.g6_.z), vec2(self.g0_.x) * vec2(other.g6_.x, -other.g8_.x) + vec2(self.g0_.y) * vec2(other.g6_.y, -other.g8_.y) + vec2(self.g0_.z) * vec2(other.g6_.z, -other.g8_.z) + self.g1_ * vec2(-other.g6_.w) + self.g1_ * vec2(other.g0_.y), vec3(self.g0_.x) * vec3(-other.g10_.x, -other.g3_.z, other.g3_.y) + vec3(self.g0_.y) * vec3(other.g3_.z, -other.g10_.x, -other.g3_.x) + vec3(self.g0_.z) * vec3(-other.g3_.y, other.g3_.x, -other.g10_.x) + vec3(self.g1_.x) * other.g9_ + vec3(self.g1_.x) * other.g4_, vec3(self.g0_.x) * vec3(other.g5_.w, other.g9_.z, -other.g9_.y) + vec3(self.g0_.y) * vec3(-other.g9_.z, other.g5_.w, other.g9_.x) + vec3(self.g0_.z) * vec3(other.g9_.y, -other.g9_.x, other.g5_.w) + vec3(self.g1_.x) * vec3(other.g5_.x, other.g5_.y, other.g5_.z) + vec3(self.g1_.y) * other.g3_, vec4(self.g0_.x) * vec4(other.g10_.y, other.g5_.z, -other.g5_.y, -other.g4_.x) + vec4(self.g0_.y) * vec4(-other.g5_.z, other.g10_.y, other.g5_.x, -other.g4_.y) + vec4(self.g0_.z) * vec4(other.g5_.y, -other.g5_.x, other.g10_.y, -other.g4_.z) + vec4(self.g1_.x) * vec4(0.0, 0.0, 0.0, -other.g10_.y) + vec4(self.g1_.y) * vec4(-other.g9_.x, -other.g9_.y, -other.g9_.z, other.g10_.x) + vec4(self.g1_.y) * vec4(other.g4_.x, other.g4_.y, other.g4_.z, 0.0), vec4(self.g0_.x) * vec4(-other.g2_.x, -other.g6_.z, other.g6_.y, other.g7_.x) + vec4(self.g0_.y) * vec4(other.g6_.z, -other.g2_.x, -other.g6_.x, other.g7_.y) + vec4(self.g0_.z) * vec4(-other.g6_.y, other.g6_.x, -other.g2_.x, other.g7_.z) + vec4(self.g1_.x) * vec4(other.g7_.x, other.g7_.y, other.g7_.z, other.g2_.y) + vec4(self.g1_.x) * vec4(-other.g1_.x, -other.g1_.y, -other.g1_.z, 0.0) + vec4(self.g1_.y) * vec4(0.0, 0.0, 0.0, -other.g2_.x), vec3(self.g0_.x) * vec3(-other.g6_.w, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, -other.g6_.w, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, -other.g6_.w) + vec3(self.g1_.x) * other.g8_ + vec3(self.g1_.y) * vec3(other.g6_.x, other.g6_.y, other.g6_.z), vec3(self.g0_.x) * vec3(other.g2_.y, other.g8_.z, -other.g8_.y) + vec3(self.g0_.y) * vec3(-other.g8_.z, other.g2_.y, other.g8_.x) + vec3(self.g0_.z) * vec3(other.g8_.y, -other.g8_.x, other.g2_.y) + vec3(self.g1_.y) * other.g7_ + vec3(self.g1_.y) * other.g1_, vec3(self.g0_.x) * vec3(other.g0_.x, other.g4_.z, -other.g4_.y) + vec3(self.g0_.y) * vec3(-other.g4_.z, other.g0_.x, other.g4_.x) + vec3(self.g0_.z) * vec3(other.g4_.y, -other.g4_.x, other.g0_.x) - vec3(self.g1_.x) * vec3(other.g5_.x, other.g5_.y, other.g5_.z) + vec3(self.g1_.y) * other.g3_, vec2(self.g0_.x) * vec2(other.g3_.x, -other.g5_.x) + vec2(self.g0_.y) * vec2(other.g3_.y, -other.g5_.y) + vec2(self.g0_.z) * vec2(other.g3_.z, -other.g5_.z) + self.g1_ * vec2(other.g5_.w) + self.g1_ * vec2(other.g0_.x));
}

MultiVector sphere_wedgeDot_plane(Sphere self, Plane other) {
    return MultiVector(vec2(self.g0_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g1_.x) * vec2(other.g0_.w, 0.0), vec3(0.0), vec2(0.0), vec3(self.g1_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec4(self.g0_.x, self.g0_.y, self.g0_.z, self.g0_.x) * vec4(other.g0_.w, other.g0_.w, other.g0_.w, 0.0) - vec4(self.g1_.y, self.g1_.y, self.g1_.y, self.g1_.x) * other.g0_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector sphere_wedgeDot_roundPoint(Sphere self, RoundPoint other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, other.g0_.z) + vec2(self.g1_.x) * vec2(0.0, other.g1_.y) + vec2(self.g1_.y) * vec2(0.0, other.g1_.x), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x, self.g0_.y, self.g0_.z, self.g0_.x) * vec4(-other.g1_.x, -other.g1_.x, -other.g1_.x, 0.0) + vec4(self.g1_.x) * vec4(-other.g0_.x, -other.g0_.y, -other.g0_.z, other.g1_.y) + vec4(self.g1_.y) * vec4(0.0, 0.0, 0.0, -other.g1_.x), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), self.g0_ * vec3(other.g1_.y) + vec3(self.g1_.y) * other.g0_, vec3(0.0), vec2(0.0));
}

Sphere sphere_wedgeDot_scalar(Sphere self, Scalar other) {
    return Sphere(self.g0_ * vec3(other.g0_), self.g1_ * vec2(other.g0_));
}

MultiVector sphere_wedgeDot_sphere(Sphere self, Sphere other) {
    return MultiVector(vec2(self.g0_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g1_.x) * vec2(other.g1_.y, 0.0) + vec2(self.g1_.y) * vec2(other.g1_.x, 0.0), vec3(0.0), vec2(0.0), vec3(0.0) - self.g0_ * vec3(other.g1_.x) + vec3(self.g1_.x) * other.g0_, vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec4(self.g0_.x, self.g0_.y, self.g0_.z, self.g0_.x) * vec4(other.g1_.y, other.g1_.y, other.g1_.y, 0.0) + vec4(self.g1_.x) * vec4(0.0, 0.0, 0.0, -other.g1_.y) + vec4(self.g1_.y) * vec4(-other.g0_.x, -other.g0_.y, -other.g0_.z, other.g1_.x), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

AntiScalar antiScalar_antiWedge_antiScalar(AntiScalar self, AntiScalar other) {
    return AntiScalar(self.g0_ * other.g0_);
}

Circle antiScalar_antiWedge_circle(AntiScalar self, Circle other) {
    return Circle(vec4(self.g0_) * other.g0_, vec3(self.g0_) * other.g1_, vec3(self.g0_) * other.g2_);
}

Dipole antiScalar_antiWedge_dipole(AntiScalar self, Dipole other) {
    return Dipole(vec3(self.g0_) * other.g0_, vec3(self.g0_) * other.g1_, vec4(self.g0_) * other.g2_);
}

DualNum antiScalar_antiWedge_dualNum(AntiScalar self, DualNum other) {
    return DualNum(vec2(self.g0_) * other.g0_);
}

FlatPoint antiScalar_antiWedge_flatPoint(AntiScalar self, FlatPoint other) {
    return FlatPoint(vec4(self.g0_) * other.g0_);
}

Flector antiScalar_antiWedge_flector(AntiScalar self, Flector other) {
    return Flector(vec4(self.g0_) * other.g0_, vec4(self.g0_) * other.g1_);
}

Line antiScalar_antiWedge_line(AntiScalar self, Line other) {
    return Line(vec3(self.g0_) * other.g0_, vec3(self.g0_) * other.g1_);
}

Motor antiScalar_antiWedge_motor(AntiScalar self, Motor other) {
    return Motor(vec4(self.g0_) * other.g0_, vec3(self.g0_) * other.g1_);
}

MultiVector antiScalar_antiWedge_multiVector(AntiScalar self, MultiVector other) {
    return MultiVector(vec2(self.g0_) * other.g0_, vec3(self.g0_) * other.g1_, vec2(self.g0_) * other.g2_, vec3(self.g0_) * other.g3_, vec3(self.g0_) * other.g4_, vec4(self.g0_) * other.g5_, vec4(self.g0_) * other.g6_, vec3(self.g0_) * other.g7_, vec3(self.g0_) * other.g8_, vec3(self.g0_) * other.g9_, vec2(self.g0_) * other.g10_);
}

Plane antiScalar_antiWedge_plane(AntiScalar self, Plane other) {
    return Plane(vec4(self.g0_) * other.g0_);
}

RoundPoint antiScalar_antiWedge_roundPoint(AntiScalar self, RoundPoint other) {
    return RoundPoint(vec3(self.g0_) * other.g0_, vec2(self.g0_) * other.g1_);
}

Scalar antiScalar_antiWedge_scalar(AntiScalar self, Scalar other) {
    return Scalar(self.g0_ * other.g0_);
}

Sphere antiScalar_antiWedge_sphere(AntiScalar self, Sphere other) {
    return Sphere(vec3(self.g0_) * other.g0_, vec2(self.g0_) * other.g1_);
}

Circle circle_antiWedge_antiScalar(Circle self, AntiScalar other) {
    return Circle(self.g0_ * vec4(other.g0_), self.g1_ * vec3(other.g0_), self.g2_ * vec3(other.g0_));
}

RoundPoint circle_antiWedge_circle(Circle self, Circle other) {
    return RoundPoint(vec3(self.g0_.x) * vec3(0.0, other.g2_.z, -other.g2_.y) + vec3(self.g0_.y) * vec3(-other.g2_.z, 0.0, other.g2_.x) + vec3(self.g0_.z) * vec3(other.g2_.y, -other.g2_.x, 0.0) + vec3(self.g0_.w) * other.g1_ + self.g1_ * vec3(other.g0_.w) + vec3(self.g2_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g2_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g2_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec2(self.g0_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g1_.z, 0.0) - vec2(self.g1_.x) * vec2(other.g0_.x, other.g2_.x) - vec2(self.g1_.y) * vec2(other.g0_.y, other.g2_.y) - vec2(self.g1_.z) * vec2(other.g0_.z, other.g2_.z) + vec2(self.g2_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g2_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g2_.z) * vec2(0.0, -other.g1_.z));
}

Scalar circle_antiWedge_dipole(Circle self, Dipole other) {
    return Scalar(0.0 - self.g0_.x * other.g2_.x - self.g0_.y * other.g2_.y - self.g0_.z * other.g2_.z - self.g0_.w * other.g2_.w - self.g1_.x * other.g1_.x - self.g1_.y * other.g1_.y - self.g1_.z * other.g1_.z - self.g2_.x * other.g0_.x - self.g2_.y * other.g0_.y - self.g2_.z * other.g0_.z);
}

Circle circle_antiWedge_dualNum(Circle self, DualNum other) {
    return Circle(self.g0_ * vec4(other.g0_.y), self.g1_ * vec3(other.g0_.y), self.g2_ * vec3(other.g0_.y));
}

Scalar circle_antiWedge_flatPoint(Circle self, FlatPoint other) {
    return Scalar(0.0 - self.g0_.x * other.g0_.x - self.g0_.y * other.g0_.y - self.g0_.z * other.g0_.z - self.g0_.w * other.g0_.w);
}

MultiVector circle_antiWedge_flector(Circle self, Flector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g0_.w) * vec2(-other.g0_.w, 0.0), vec3(0.0), vec2(0.0), vec3(self.g0_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g1_.w) - vec3(self.g0_.w) * vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec4(self.g1_.x) * vec4(other.g1_.w, 0.0, 0.0, -other.g1_.x) + vec4(self.g1_.y) * vec4(0.0, other.g1_.w, 0.0, -other.g1_.y) + vec4(self.g1_.z) * vec4(0.0, 0.0, other.g1_.w, -other.g1_.z) + vec4(self.g2_.x) * vec4(0.0, -other.g1_.z, other.g1_.y, 0.0) + vec4(self.g2_.y) * vec4(other.g1_.z, 0.0, -other.g1_.x, 0.0) + vec4(self.g2_.z) * vec4(-other.g1_.y, other.g1_.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

RoundPoint circle_antiWedge_line(Circle self, Line other) {
    return RoundPoint(vec3(self.g0_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g0_.w) * other.g0_, vec2(self.g0_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g1_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g2_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g2_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g2_.z) * vec2(0.0, -other.g0_.z));
}

MultiVector circle_antiWedge_motor(Circle self, Motor other) {
    return MultiVector(vec2(0.0), vec3(self.g0_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g0_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g0_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g1_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g2_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g2_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g2_.z) * vec2(0.0, -other.g0_.z), vec3(0.0), vec3(0.0), vec4(0.0), self.g0_ * vec4(other.g0_.w), self.g1_ * vec3(other.g0_.w), self.g2_ * vec3(other.g0_.w), vec3(0.0), vec2(0.0));
}

MultiVector circle_antiWedge_multiVector(Circle self, MultiVector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(-other.g5_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g5_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g5_.z, 0.0) + vec2(self.g0_.w) * vec2(-other.g5_.w, 0.0) + vec2(self.g1_.x) * vec2(-other.g4_.x, 0.0) + vec2(self.g1_.y) * vec2(-other.g4_.y, 0.0) + vec2(self.g1_.z) * vec2(-other.g4_.z, 0.0) + vec2(self.g2_.x) * vec2(-other.g3_.x, 0.0) + vec2(self.g2_.y) * vec2(-other.g3_.y, 0.0) + vec2(self.g2_.z) * vec2(-other.g3_.z, 0.0), vec3(self.g0_.x) * vec3(0.0, other.g8_.z, -other.g8_.y) + vec3(self.g0_.y) * vec3(-other.g8_.z, 0.0, other.g8_.x) + vec3(self.g0_.z) * vec3(other.g8_.y, -other.g8_.x, 0.0) + vec3(self.g0_.w) * other.g7_ + self.g1_ * vec3(other.g6_.w) + vec3(self.g2_.x) * vec3(0.0, -other.g6_.z, other.g6_.y) + vec3(self.g2_.y) * vec3(other.g6_.z, 0.0, -other.g6_.x) + vec3(self.g2_.z) * vec3(-other.g6_.y, other.g6_.x, 0.0), vec2(self.g0_.x) * vec2(-other.g7_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g7_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g7_.z, 0.0) - vec2(self.g1_.x) * vec2(other.g6_.x, other.g8_.x) - vec2(self.g1_.y) * vec2(other.g6_.y, other.g8_.y) - vec2(self.g1_.z) * vec2(other.g6_.z, other.g8_.z) + vec2(self.g2_.x) * vec2(0.0, -other.g7_.x) + vec2(self.g2_.y) * vec2(0.0, -other.g7_.y) + vec2(self.g2_.z) * vec2(0.0, -other.g7_.z), vec3(self.g0_.x) * vec3(0.0, other.g9_.z, -other.g9_.y) + vec3(self.g0_.y) * vec3(-other.g9_.z, 0.0, other.g9_.x) + vec3(self.g0_.z) * vec3(other.g9_.y, -other.g9_.x, 0.0) + self.g1_ * vec3(other.g10_.x), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g10_.y) - vec3(self.g0_.w) * other.g9_ + self.g2_ * vec3(other.g10_.x), vec4(self.g1_.x) * vec4(other.g10_.y, 0.0, 0.0, -other.g9_.x) + vec4(self.g1_.y) * vec4(0.0, other.g10_.y, 0.0, -other.g9_.y) + vec4(self.g1_.z) * vec4(0.0, 0.0, other.g10_.y, -other.g9_.z) + vec4(self.g2_.x) * vec4(0.0, -other.g9_.z, other.g9_.y, 0.0) + vec4(self.g2_.y) * vec4(other.g9_.z, 0.0, -other.g9_.x, 0.0) + vec4(self.g2_.z) * vec4(-other.g9_.y, other.g9_.x, 0.0, 0.0), self.g0_ * vec4(other.g0_.y), self.g1_ * vec3(other.g0_.y), self.g2_ * vec3(other.g0_.y), vec3(0.0), vec2(0.0));
}

Dipole circle_antiWedge_plane(Circle self, Plane other) {
    return Dipole(vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_.w) - vec3(self.g0_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec4(self.g1_.x) * vec4(other.g0_.w, 0.0, 0.0, -other.g0_.x) + vec4(self.g1_.y) * vec4(0.0, other.g0_.w, 0.0, -other.g0_.y) + vec4(self.g1_.z) * vec4(0.0, 0.0, other.g0_.w, -other.g0_.z) + vec4(self.g2_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g2_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g2_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0));
}

Dipole circle_antiWedge_sphere(Circle self, Sphere other) {
    return Dipole(vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + self.g1_ * vec3(other.g1_.x), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g1_.y) - vec3(self.g0_.w) * other.g0_ + self.g2_ * vec3(other.g1_.x), vec4(self.g1_.x) * vec4(other.g1_.y, 0.0, 0.0, -other.g0_.x) + vec4(self.g1_.y) * vec4(0.0, other.g1_.y, 0.0, -other.g0_.y) + vec4(self.g1_.z) * vec4(0.0, 0.0, other.g1_.y, -other.g0_.z) + vec4(self.g2_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g2_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g2_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0));
}

Dipole dipole_antiWedge_antiScalar(Dipole self, AntiScalar other) {
    return Dipole(self.g0_ * vec3(other.g0_), self.g1_ * vec3(other.g0_), self.g2_ * vec4(other.g0_));
}

Scalar dipole_antiWedge_circle(Dipole self, Circle other) {
    return Scalar(0.0 - self.g0_.x * other.g2_.x - self.g0_.y * other.g2_.y - self.g0_.z * other.g2_.z - self.g1_.x * other.g1_.x - self.g1_.y * other.g1_.y - self.g1_.z * other.g1_.z - self.g2_.x * other.g0_.x - self.g2_.y * other.g0_.y - self.g2_.z * other.g0_.z - self.g2_.w * other.g0_.w);
}

Dipole dipole_antiWedge_dualNum(Dipole self, DualNum other) {
    return Dipole(self.g0_ * vec3(other.g0_.y), self.g1_ * vec3(other.g0_.y), self.g2_ * vec4(other.g0_.y));
}

RoundPoint dipole_antiWedge_flector(Dipole self, Flector other) {
    return RoundPoint(self.g0_ * vec3(other.g1_.w) + vec3(self.g1_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g1_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g1_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0), vec2(self.g0_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g1_.z, 0.0) + vec2(self.g2_.x) * vec2(0.0, other.g1_.x) + vec2(self.g2_.y) * vec2(0.0, other.g1_.y) + vec2(self.g2_.z) * vec2(0.0, other.g1_.z) + vec2(self.g2_.w) * vec2(0.0, other.g1_.w));
}

Scalar dipole_antiWedge_line(Dipole self, Line other) {
    return Scalar(0.0 - self.g0_.x * other.g1_.x - self.g0_.y * other.g1_.y - self.g0_.z * other.g1_.z - self.g1_.x * other.g0_.x - self.g1_.y * other.g0_.y - self.g1_.z * other.g0_.z);
}

MultiVector dipole_antiWedge_motor(Dipole self, Motor other) {
    return MultiVector(vec2(self.g0_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g1_.z, 0.0) + vec2(self.g1_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g1_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g1_.z) * vec2(-other.g0_.z, 0.0), vec3(0.0), vec2(0.0), self.g0_ * vec3(other.g0_.w), self.g1_ * vec3(other.g0_.w), self.g2_ * vec4(other.g0_.w), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector dipole_antiWedge_multiVector(Dipole self, MultiVector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(-other.g8_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g8_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g8_.z, 0.0) + vec2(self.g1_.x) * vec2(-other.g7_.x, 0.0) + vec2(self.g1_.y) * vec2(-other.g7_.y, 0.0) + vec2(self.g1_.z) * vec2(-other.g7_.z, 0.0) + vec2(self.g2_.x) * vec2(-other.g6_.x, 0.0) + vec2(self.g2_.y) * vec2(-other.g6_.y, 0.0) + vec2(self.g2_.z) * vec2(-other.g6_.z, 0.0) + vec2(self.g2_.w) * vec2(-other.g6_.w, 0.0), self.g0_ * vec3(other.g10_.y) + vec3(self.g1_.x) * vec3(0.0, -other.g9_.z, other.g9_.y) + vec3(self.g1_.y) * vec3(other.g9_.z, 0.0, -other.g9_.x) + vec3(self.g1_.z) * vec3(-other.g9_.y, other.g9_.x, 0.0) - vec3(self.g2_.x, self.g2_.y, self.g2_.z) * vec3(other.g10_.x), vec2(self.g0_.x) * vec2(-other.g9_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g9_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g9_.z, 0.0) + vec2(self.g2_.x) * vec2(0.0, other.g9_.x) + vec2(self.g2_.y) * vec2(0.0, other.g9_.y) + vec2(self.g2_.z) * vec2(0.0, other.g9_.z) + vec2(self.g2_.w) * other.g10_ * vec2(-1.0, 1.0), self.g0_ * vec3(other.g0_.y), self.g1_ * vec3(other.g0_.y), self.g2_ * vec4(other.g0_.y), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

RoundPoint dipole_antiWedge_plane(Dipole self, Plane other) {
    return RoundPoint(self.g0_ * vec3(other.g0_.w) + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec2(self.g0_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g2_.x) * vec2(0.0, other.g0_.x) + vec2(self.g2_.y) * vec2(0.0, other.g0_.y) + vec2(self.g2_.z) * vec2(0.0, other.g0_.z) + vec2(self.g2_.w) * vec2(0.0, other.g0_.w));
}

RoundPoint dipole_antiWedge_sphere(Dipole self, Sphere other) {
    return RoundPoint(self.g0_ * vec3(other.g1_.y) + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) - vec3(self.g2_.x, self.g2_.y, self.g2_.z) * vec3(other.g1_.x), vec2(self.g0_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g2_.x) * vec2(0.0, other.g0_.x) + vec2(self.g2_.y) * vec2(0.0, other.g0_.y) + vec2(self.g2_.z) * vec2(0.0, other.g0_.z) + vec2(self.g2_.w) * other.g1_ * vec2(-1.0, 1.0));
}

DualNum dualNum_antiWedge_antiScalar(DualNum self, AntiScalar other) {
    return DualNum(self.g0_ * vec2(other.g0_));
}

Circle dualNum_antiWedge_circle(DualNum self, Circle other) {
    return Circle(vec4(self.g0_.y) * other.g0_, vec3(self.g0_.y) * other.g1_, vec3(self.g0_.y) * other.g2_);
}

Dipole dualNum_antiWedge_dipole(DualNum self, Dipole other) {
    return Dipole(vec3(self.g0_.y) * other.g0_, vec3(self.g0_.y) * other.g1_, vec4(self.g0_.y) * other.g2_);
}

DualNum dualNum_antiWedge_dualNum(DualNum self, DualNum other) {
    return DualNum(vec2(self.g0_.x) * vec2(other.g0_.y, 0.0) + vec2(self.g0_.y) * other.g0_);
}

FlatPoint dualNum_antiWedge_flatPoint(DualNum self, FlatPoint other) {
    return FlatPoint(vec4(self.g0_.y) * other.g0_);
}

Flector dualNum_antiWedge_flector(DualNum self, Flector other) {
    return Flector(vec4(self.g0_.y) * other.g0_, vec4(self.g0_.y) * other.g1_);
}

Line dualNum_antiWedge_line(DualNum self, Line other) {
    return Line(vec3(self.g0_.y) * other.g0_, vec3(self.g0_.y) * other.g1_);
}

MultiVector dualNum_antiWedge_motor(DualNum self, Motor other) {
    return MultiVector(self.g0_ * vec2(other.g0_.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(self.g0_.y) * other.g1_, vec3(0.0), vec2(0.0));
}

MultiVector dualNum_antiWedge_multiVector(DualNum self, MultiVector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(other.g0_.y, 0.0) + vec2(self.g0_.y) * other.g0_, vec3(self.g0_.y) * other.g1_, vec2(self.g0_.y) * other.g2_, vec3(self.g0_.y) * other.g3_, vec3(self.g0_.y) * other.g4_, vec4(self.g0_.y) * other.g5_, vec4(self.g0_.y) * other.g6_, vec3(self.g0_.y) * other.g7_, vec3(self.g0_.y) * other.g8_, vec3(self.g0_.y) * other.g9_, vec2(self.g0_.y) * other.g10_);
}

Plane dualNum_antiWedge_plane(DualNum self, Plane other) {
    return Plane(vec4(self.g0_.y) * other.g0_);
}

RoundPoint dualNum_antiWedge_roundPoint(DualNum self, RoundPoint other) {
    return RoundPoint(vec3(self.g0_.y) * other.g0_, vec2(self.g0_.y) * other.g1_);
}

Scalar dualNum_antiWedge_scalar(DualNum self, Scalar other) {
    return Scalar(self.g0_.y * other.g0_);
}

Sphere dualNum_antiWedge_sphere(DualNum self, Sphere other) {
    return Sphere(vec3(self.g0_.y) * other.g0_, vec2(self.g0_.y) * other.g1_);
}

FlatPoint flatPoint_antiWedge_antiScalar(FlatPoint self, AntiScalar other) {
    return FlatPoint(self.g0_ * vec4(other.g0_));
}

Scalar flatPoint_antiWedge_circle(FlatPoint self, Circle other) {
    return Scalar(0.0 - self.g0_.x * other.g0_.x - self.g0_.y * other.g0_.y - self.g0_.z * other.g0_.z - self.g0_.w * other.g0_.w);
}

FlatPoint flatPoint_antiWedge_dualNum(FlatPoint self, DualNum other) {
    return FlatPoint(self.g0_ * vec4(other.g0_.y));
}

RoundPoint flatPoint_antiWedge_flector(FlatPoint self, Flector other) {
    return RoundPoint(vec3(0.0), vec2(self.g0_.x) * vec2(0.0, other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, other.g1_.z) + vec2(self.g0_.w) * vec2(0.0, other.g1_.w));
}

FlatPoint flatPoint_antiWedge_motor(FlatPoint self, Motor other) {
    return FlatPoint(self.g0_ * vec4(other.g0_.w));
}

MultiVector flatPoint_antiWedge_multiVector(FlatPoint self, MultiVector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(-other.g6_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g6_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g6_.z, 0.0) + vec2(self.g0_.w) * vec2(-other.g6_.w, 0.0), vec3(0.0) - vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g10_.x), vec2(self.g0_.x) * vec2(0.0, other.g9_.x) + vec2(self.g0_.y) * vec2(0.0, other.g9_.y) + vec2(self.g0_.z) * vec2(0.0, other.g9_.z) + vec2(self.g0_.w) * other.g10_ * vec2(-1.0, 1.0), vec3(0.0), vec3(0.0), self.g0_ * vec4(other.g0_.y), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

RoundPoint flatPoint_antiWedge_plane(FlatPoint self, Plane other) {
    return RoundPoint(vec3(0.0), vec2(self.g0_.x) * vec2(0.0, other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, other.g0_.z) + vec2(self.g0_.w) * vec2(0.0, other.g0_.w));
}

RoundPoint flatPoint_antiWedge_sphere(FlatPoint self, Sphere other) {
    return RoundPoint(vec3(0.0) - vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g1_.x), vec2(self.g0_.x) * vec2(0.0, other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, other.g0_.z) + vec2(self.g0_.w) * other.g1_ * vec2(-1.0, 1.0));
}

Flector flector_antiWedge_antiScalar(Flector self, AntiScalar other) {
    return Flector(self.g0_ * vec4(other.g0_), self.g1_ * vec4(other.g0_));
}

MultiVector flector_antiWedge_circle(Flector self, Circle other) {
    return MultiVector(vec2(self.g0_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g0_.w) * vec2(-other.g0_.w, 0.0), vec3(0.0), vec2(0.0), vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(0.0) - vec3(self.g1_.x, self.g1_.y, self.g1_.z) * vec3(other.g0_.w) + vec3(self.g1_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec4(self.g1_.x) * vec4(0.0, other.g2_.z, -other.g2_.y, -other.g1_.x) + vec4(self.g1_.y) * vec4(-other.g2_.z, 0.0, other.g2_.x, -other.g1_.y) + vec4(self.g1_.z) * vec4(other.g2_.y, -other.g2_.x, 0.0, -other.g1_.z) + vec4(self.g1_.w) * vec4(other.g1_.x, other.g1_.y, other.g1_.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

RoundPoint flector_antiWedge_dipole(Flector self, Dipole other) {
    return RoundPoint(vec3(self.g1_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g1_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g1_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) - vec3(self.g1_.w) * other.g0_, vec2(self.g1_.x) * vec2(other.g0_.x, -other.g2_.x) + vec2(self.g1_.y) * vec2(other.g0_.y, -other.g2_.y) + vec2(self.g1_.z) * vec2(other.g0_.z, -other.g2_.z) + vec2(self.g1_.w) * vec2(0.0, -other.g2_.w));
}

Flector flector_antiWedge_dualNum(Flector self, DualNum other) {
    return Flector(self.g0_ * vec4(other.g0_.y), self.g1_ * vec4(other.g0_.y));
}

RoundPoint flector_antiWedge_flatPoint(Flector self, FlatPoint other) {
    return RoundPoint(vec3(0.0), vec2(self.g1_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g1_.w) * vec2(0.0, -other.g0_.w));
}

MultiVector flector_antiWedge_flector(Flector self, Flector other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(self.g0_.x) * vec2(0.0, other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, other.g1_.z) + vec2(self.g0_.w) * vec2(0.0, other.g1_.w) + vec2(self.g1_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g1_.w) * vec2(0.0, -other.g0_.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g1_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g1_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g1_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0), vec3(self.g1_.x, self.g1_.y, self.g1_.z) * vec3(other.g1_.w) - vec3(self.g1_.w) * vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec3(0.0), vec2(0.0));
}

FlatPoint flector_antiWedge_line(Flector self, Line other) {
    return FlatPoint(vec4(self.g1_.x) * vec4(0.0, other.g1_.z, -other.g1_.y, -other.g0_.x) + vec4(self.g1_.y) * vec4(-other.g1_.z, 0.0, other.g1_.x, -other.g0_.y) + vec4(self.g1_.z) * vec4(other.g1_.y, -other.g1_.x, 0.0, -other.g0_.z) + vec4(self.g1_.w) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, 0.0));
}

Flector flector_antiWedge_motor(Flector self, Motor other) {
    return Flector(self.g0_ * vec4(other.g0_.w) + vec4(self.g1_.x) * vec4(0.0, other.g1_.z, -other.g1_.y, -other.g0_.x) + vec4(self.g1_.y) * vec4(-other.g1_.z, 0.0, other.g1_.x, -other.g0_.y) + vec4(self.g1_.z) * vec4(other.g1_.y, -other.g1_.x, 0.0, -other.g0_.z) + vec4(self.g1_.w) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, 0.0), self.g1_ * vec4(other.g0_.w));
}

MultiVector flector_antiWedge_multiVector(Flector self, MultiVector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(-other.g6_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g6_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g6_.z, 0.0) + vec2(self.g0_.w) * vec2(-other.g6_.w, 0.0) + vec2(self.g1_.x) * vec2(other.g1_.x, 0.0) + vec2(self.g1_.y) * vec2(other.g1_.y, 0.0) + vec2(self.g1_.z) * vec2(other.g1_.z, 0.0) + vec2(self.g1_.w) * vec2(other.g2_.x, 0.0), vec3(0.0) - vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g10_.x) + vec3(self.g1_.x) * vec3(0.0, -other.g4_.z, other.g4_.y) + vec3(self.g1_.y) * vec3(other.g4_.z, 0.0, -other.g4_.x) + vec3(self.g1_.z) * vec3(-other.g4_.y, other.g4_.x, 0.0) - vec3(self.g1_.w) * other.g3_, vec2(self.g0_.x) * vec2(0.0, other.g9_.x) + vec2(self.g0_.y) * vec2(0.0, other.g9_.y) + vec2(self.g0_.z) * vec2(0.0, other.g9_.z) + vec2(self.g0_.w) * other.g10_ * vec2(-1.0, 1.0) + vec2(self.g1_.x) * vec2(other.g3_.x, -other.g5_.x) + vec2(self.g1_.y) * vec2(other.g3_.y, -other.g5_.y) + vec2(self.g1_.z) * vec2(other.g3_.z, -other.g5_.z) + vec2(self.g1_.w) * vec2(0.0, -other.g5_.w), vec3(self.g1_.x) * vec3(0.0, -other.g6_.z, other.g6_.y) + vec3(self.g1_.y) * vec3(other.g6_.z, 0.0, -other.g6_.x) + vec3(self.g1_.z) * vec3(-other.g6_.y, other.g6_.x, 0.0), vec3(0.0) - vec3(self.g1_.x, self.g1_.y, self.g1_.z) * vec3(other.g6_.w) + vec3(self.g1_.w) * vec3(other.g6_.x, other.g6_.y, other.g6_.z), self.g0_ * vec4(other.g0_.y) + vec4(self.g1_.x) * vec4(0.0, other.g8_.z, -other.g8_.y, -other.g7_.x) + vec4(self.g1_.y) * vec4(-other.g8_.z, 0.0, other.g8_.x, -other.g7_.y) + vec4(self.g1_.z) * vec4(other.g8_.y, -other.g8_.x, 0.0, -other.g7_.z) + vec4(self.g1_.w) * vec4(other.g7_.x, other.g7_.y, other.g7_.z, 0.0), vec4(0.0) - self.g1_ * vec4(other.g10_.x), vec3(self.g1_.x) * vec3(0.0, other.g9_.z, -other.g9_.y) + vec3(self.g1_.y) * vec3(-other.g9_.z, 0.0, other.g9_.x) + vec3(self.g1_.z) * vec3(other.g9_.y, -other.g9_.x, 0.0), vec3(self.g1_.x, self.g1_.y, self.g1_.z) * vec3(other.g10_.y) - vec3(self.g1_.w) * other.g9_, vec3(self.g1_.x, self.g1_.y, self.g1_.z) * vec3(other.g0_.y), vec2(self.g1_.w) * vec2(0.0, other.g0_.y));
}

MultiVector flector_antiWedge_plane(Flector self, Plane other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(self.g0_.x) * vec2(0.0, other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, other.g0_.z) + vec2(self.g0_.w) * vec2(0.0, other.g0_.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g1_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g1_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g1_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec3(self.g1_.x, self.g1_.y, self.g1_.z) * vec3(other.g0_.w) - vec3(self.g1_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(0.0), vec2(0.0));
}

Scalar flector_antiWedge_roundPoint(Flector self, RoundPoint other) {
    return Scalar(self.g1_.x * other.g0_.x + self.g1_.y * other.g0_.y + self.g1_.z * other.g0_.z + self.g1_.w * other.g1_.x);
}

MultiVector flector_antiWedge_sphere(Flector self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(0.0) - vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g1_.x), vec2(self.g0_.x) * vec2(0.0, other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, other.g0_.z) + vec2(self.g0_.w) * other.g1_ * vec2(-1.0, 1.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0) - self.g1_ * vec4(other.g1_.x), vec3(self.g1_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g1_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g1_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec3(self.g1_.x, self.g1_.y, self.g1_.z) * vec3(other.g1_.y) - vec3(self.g1_.w) * other.g0_, vec3(0.0), vec2(0.0));
}

Line line_antiWedge_antiScalar(Line self, AntiScalar other) {
    return Line(self.g0_ * vec3(other.g0_), self.g1_ * vec3(other.g0_));
}

RoundPoint line_antiWedge_circle(Line self, Circle other) {
    return RoundPoint(self.g0_ * vec3(other.g0_.w) + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec2(0.0) - vec2(self.g0_.x) * vec2(other.g0_.x, other.g2_.x) - vec2(self.g0_.y) * vec2(other.g0_.y, other.g2_.y) - vec2(self.g0_.z) * vec2(other.g0_.z, other.g2_.z) + vec2(self.g1_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g1_.z));
}

Scalar line_antiWedge_dipole(Line self, Dipole other) {
    return Scalar(0.0 - self.g0_.x * other.g1_.x - self.g0_.y * other.g1_.y - self.g0_.z * other.g1_.z - self.g1_.x * other.g0_.x - self.g1_.y * other.g0_.y - self.g1_.z * other.g0_.z);
}

Line line_antiWedge_dualNum(Line self, DualNum other) {
    return Line(self.g0_ * vec3(other.g0_.y), self.g1_ * vec3(other.g0_.y));
}

FlatPoint line_antiWedge_flector(Line self, Flector other) {
    return FlatPoint(vec4(self.g0_.x) * vec4(other.g1_.w, 0.0, 0.0, -other.g1_.x) + vec4(self.g0_.y) * vec4(0.0, other.g1_.w, 0.0, -other.g1_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, other.g1_.w, -other.g1_.z) + vec4(self.g1_.x) * vec4(0.0, -other.g1_.z, other.g1_.y, 0.0) + vec4(self.g1_.y) * vec4(other.g1_.z, 0.0, -other.g1_.x, 0.0) + vec4(self.g1_.z) * vec4(-other.g1_.y, other.g1_.x, 0.0, 0.0));
}

RoundPoint line_antiWedge_line(Line self, Line other) {
    return RoundPoint(vec3(0.0), vec2(self.g0_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g1_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g0_.z));
}

MultiVector line_antiWedge_motor(Line self, Motor other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(self.g0_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g1_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g0_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), self.g0_ * vec3(other.g0_.w), self.g1_ * vec3(other.g0_.w), vec3(0.0), vec2(0.0));
}

MultiVector line_antiWedge_multiVector(Line self, MultiVector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(-other.g4_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g4_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g4_.z, 0.0) + vec2(self.g1_.x) * vec2(-other.g3_.x, 0.0) + vec2(self.g1_.y) * vec2(-other.g3_.y, 0.0) + vec2(self.g1_.z) * vec2(-other.g3_.z, 0.0), self.g0_ * vec3(other.g6_.w) + vec3(self.g1_.x) * vec3(0.0, -other.g6_.z, other.g6_.y) + vec3(self.g1_.y) * vec3(other.g6_.z, 0.0, -other.g6_.x) + vec3(self.g1_.z) * vec3(-other.g6_.y, other.g6_.x, 0.0), vec2(0.0) - vec2(self.g0_.x) * vec2(other.g6_.x, other.g8_.x) - vec2(self.g0_.y) * vec2(other.g6_.y, other.g8_.y) - vec2(self.g0_.z) * vec2(other.g6_.z, other.g8_.z) + vec2(self.g1_.x) * vec2(0.0, -other.g7_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g7_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g7_.z), self.g0_ * vec3(other.g10_.x), self.g1_ * vec3(other.g10_.x), vec4(self.g0_.x) * vec4(other.g10_.y, 0.0, 0.0, -other.g9_.x) + vec4(self.g0_.y) * vec4(0.0, other.g10_.y, 0.0, -other.g9_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, other.g10_.y, -other.g9_.z) + vec4(self.g1_.x) * vec4(0.0, -other.g9_.z, other.g9_.y, 0.0) + vec4(self.g1_.y) * vec4(other.g9_.z, 0.0, -other.g9_.x, 0.0) + vec4(self.g1_.z) * vec4(-other.g9_.y, other.g9_.x, 0.0, 0.0), vec4(0.0), self.g0_ * vec3(other.g0_.y), self.g1_ * vec3(other.g0_.y), vec3(0.0), vec2(0.0));
}

FlatPoint line_antiWedge_plane(Line self, Plane other) {
    return FlatPoint(vec4(self.g0_.x) * vec4(other.g0_.w, 0.0, 0.0, -other.g0_.x) + vec4(self.g0_.y) * vec4(0.0, other.g0_.w, 0.0, -other.g0_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, other.g0_.w, -other.g0_.z) + vec4(self.g1_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g1_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g1_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0));
}

Dipole line_antiWedge_sphere(Line self, Sphere other) {
    return Dipole(self.g0_ * vec3(other.g1_.x), self.g1_ * vec3(other.g1_.x), vec4(self.g0_.x) * vec4(other.g1_.y, 0.0, 0.0, -other.g0_.x) + vec4(self.g0_.y) * vec4(0.0, other.g1_.y, 0.0, -other.g0_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, other.g1_.y, -other.g0_.z) + vec4(self.g1_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g1_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g1_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0));
}

Motor motor_antiWedge_antiScalar(Motor self, AntiScalar other) {
    return Motor(self.g0_ * vec4(other.g0_), self.g1_ * vec3(other.g0_));
}

MultiVector motor_antiWedge_circle(Motor self, Circle other) {
    return MultiVector(vec2(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_.w) + vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec2(0.0) - vec2(self.g0_.x) * vec2(other.g0_.x, other.g2_.x) - vec2(self.g0_.y) * vec2(other.g0_.y, other.g2_.y) - vec2(self.g0_.z) * vec2(other.g0_.z, other.g2_.z) + vec2(self.g1_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g1_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.w) * other.g0_, vec3(self.g0_.w) * other.g1_, vec3(self.g0_.w) * other.g2_, vec3(0.0), vec2(0.0));
}

MultiVector motor_antiWedge_dipole(Motor self, Dipole other) {
    return MultiVector(vec2(self.g0_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g1_.z, 0.0) + vec2(self.g1_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g1_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g1_.z) * vec2(-other.g0_.z, 0.0), vec3(0.0), vec2(0.0), vec3(self.g0_.w) * other.g0_, vec3(self.g0_.w) * other.g1_, vec4(self.g0_.w) * other.g2_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector motor_antiWedge_dualNum(Motor self, DualNum other) {
    return MultiVector(vec2(self.g0_.w) * other.g0_, vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_.y), self.g1_ * vec3(other.g0_.y), vec3(0.0), vec2(0.0));
}

FlatPoint motor_antiWedge_flatPoint(Motor self, FlatPoint other) {
    return FlatPoint(vec4(self.g0_.w) * other.g0_);
}

Flector motor_antiWedge_flector(Motor self, Flector other) {
    return Flector(vec4(self.g0_.x) * vec4(other.g1_.w, 0.0, 0.0, -other.g1_.x) + vec4(self.g0_.y) * vec4(0.0, other.g1_.w, 0.0, -other.g1_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, other.g1_.w, -other.g1_.z) + vec4(self.g0_.w) * other.g0_ + vec4(self.g1_.x) * vec4(0.0, -other.g1_.z, other.g1_.y, 0.0) + vec4(self.g1_.y) * vec4(other.g1_.z, 0.0, -other.g1_.x, 0.0) + vec4(self.g1_.z) * vec4(-other.g1_.y, other.g1_.x, 0.0, 0.0), vec4(self.g0_.w) * other.g1_);
}

MultiVector motor_antiWedge_line(Motor self, Line other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(self.g0_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g1_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g0_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0_.w) * other.g0_, vec3(self.g0_.w) * other.g1_, vec3(0.0), vec2(0.0));
}

MultiVector motor_antiWedge_motor(Motor self, Motor other) {
    return MultiVector(vec2(self.g0_.w) * vec2(0.0, other.g0_.w), vec3(0.0), vec2(self.g0_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g1_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g0_.z), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_.w) + vec3(self.g0_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(self.g0_.w) * other.g1_ + self.g1_ * vec3(other.g0_.w), vec3(0.0), vec2(0.0));
}

MultiVector motor_antiWedge_multiVector(Motor self, MultiVector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(-other.g4_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g4_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g4_.z, 0.0) + vec2(self.g0_.w) * other.g0_ + vec2(self.g1_.x) * vec2(-other.g3_.x, 0.0) + vec2(self.g1_.y) * vec2(-other.g3_.y, 0.0) + vec2(self.g1_.z) * vec2(-other.g3_.z, 0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g6_.w) + vec3(self.g0_.w) * other.g1_ + vec3(self.g1_.x) * vec3(0.0, -other.g6_.z, other.g6_.y) + vec3(self.g1_.y) * vec3(other.g6_.z, 0.0, -other.g6_.x) + vec3(self.g1_.z) * vec3(-other.g6_.y, other.g6_.x, 0.0), vec2(0.0) - vec2(self.g0_.x) * vec2(other.g6_.x, other.g8_.x) - vec2(self.g0_.y) * vec2(other.g6_.y, other.g8_.y) - vec2(self.g0_.z) * vec2(other.g6_.z, other.g8_.z) + vec2(self.g0_.w) * other.g2_ + vec2(self.g1_.x) * vec2(0.0, -other.g7_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g7_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g7_.z), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g10_.x) + vec3(self.g0_.w) * other.g3_, vec3(self.g0_.w) * other.g4_ + self.g1_ * vec3(other.g10_.x), vec4(self.g0_.x) * vec4(other.g10_.y, 0.0, 0.0, -other.g9_.x) + vec4(self.g0_.y) * vec4(0.0, other.g10_.y, 0.0, -other.g9_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, other.g10_.y, -other.g9_.z) + vec4(self.g0_.w) * other.g5_ + vec4(self.g1_.x) * vec4(0.0, -other.g9_.z, other.g9_.y, 0.0) + vec4(self.g1_.y) * vec4(other.g9_.z, 0.0, -other.g9_.x, 0.0) + vec4(self.g1_.z) * vec4(-other.g9_.y, other.g9_.x, 0.0, 0.0), vec4(self.g0_.w) * other.g6_, vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_.y) + vec3(self.g0_.w) * other.g7_, vec3(self.g0_.w) * other.g8_ + self.g1_ * vec3(other.g0_.y), vec3(self.g0_.w) * other.g9_, vec2(self.g0_.w) * other.g10_);
}

Flector motor_antiWedge_plane(Motor self, Plane other) {
    return Flector(vec4(self.g0_.x) * vec4(other.g0_.w, 0.0, 0.0, -other.g0_.x) + vec4(self.g0_.y) * vec4(0.0, other.g0_.w, 0.0, -other.g0_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, other.g0_.w, -other.g0_.z) + vec4(self.g1_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g1_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g1_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0), vec4(self.g0_.w) * other.g0_);
}

RoundPoint motor_antiWedge_roundPoint(Motor self, RoundPoint other) {
    return RoundPoint(vec3(self.g0_.w) * other.g0_, vec2(self.g0_.w) * other.g1_);
}

Scalar motor_antiWedge_scalar(Motor self, Scalar other) {
    return Scalar(self.g0_.w * other.g0_);
}

MultiVector motor_antiWedge_sphere(Motor self, Sphere other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g1_.x), self.g1_ * vec3(other.g1_.x), vec4(self.g0_.x) * vec4(other.g1_.y, 0.0, 0.0, -other.g0_.x) + vec4(self.g0_.y) * vec4(0.0, other.g1_.y, 0.0, -other.g0_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, other.g1_.y, -other.g0_.z) + vec4(self.g1_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g1_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g1_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.w) * other.g0_, vec2(self.g0_.w) * other.g1_);
}

MultiVector multiVector_antiWedge_antiScalar(MultiVector self, AntiScalar other) {
    return MultiVector(self.g0_ * vec2(other.g0_), self.g1_ * vec3(other.g0_), self.g2_ * vec2(other.g0_), self.g3_ * vec3(other.g0_), self.g4_ * vec3(other.g0_), self.g5_ * vec4(other.g0_), self.g6_ * vec4(other.g0_), self.g7_ * vec3(other.g0_), self.g8_ * vec3(other.g0_), self.g9_ * vec3(other.g0_), self.g10_ * vec2(other.g0_));
}

MultiVector multiVector_antiWedge_circle(MultiVector self, Circle other) {
    return MultiVector(vec2(self.g3_.x) * vec2(-other.g2_.x, 0.0) + vec2(self.g3_.y) * vec2(-other.g2_.y, 0.0) + vec2(self.g3_.z) * vec2(-other.g2_.z, 0.0) + vec2(self.g4_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g4_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g4_.z) * vec2(-other.g1_.z, 0.0) + vec2(self.g5_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g5_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g5_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g5_.w) * vec2(-other.g0_.w, 0.0), vec3(self.g6_.x) * vec3(0.0, other.g2_.z, -other.g2_.y) + vec3(self.g6_.y) * vec3(-other.g2_.z, 0.0, other.g2_.x) + vec3(self.g6_.z) * vec3(other.g2_.y, -other.g2_.x, 0.0) + vec3(self.g6_.w) * other.g1_ + self.g7_ * vec3(other.g0_.w) + vec3(self.g8_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g8_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g8_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec2(self.g6_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g6_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g6_.z) * vec2(-other.g1_.z, 0.0) - vec2(self.g7_.x) * vec2(other.g0_.x, other.g2_.x) - vec2(self.g7_.y) * vec2(other.g0_.y, other.g2_.y) - vec2(self.g7_.z) * vec2(other.g0_.z, other.g2_.z) + vec2(self.g8_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g8_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g8_.z) * vec2(0.0, -other.g1_.z), vec3(self.g9_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g9_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g9_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) + vec3(self.g10_.x) * other.g1_, vec3(0.0) - self.g9_ * vec3(other.g0_.w) + vec3(self.g10_.x) * other.g2_ + vec3(self.g10_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec4(self.g9_.x) * vec4(0.0, other.g2_.z, -other.g2_.y, -other.g1_.x) + vec4(self.g9_.y) * vec4(-other.g2_.z, 0.0, other.g2_.x, -other.g1_.y) + vec4(self.g9_.z) * vec4(other.g2_.y, -other.g2_.x, 0.0, -other.g1_.z) + vec4(self.g10_.y) * vec4(other.g1_.x, other.g1_.y, other.g1_.z, 0.0), vec4(self.g0_.y) * other.g0_, vec3(self.g0_.y) * other.g1_, vec3(self.g0_.y) * other.g2_, vec3(0.0), vec2(0.0));
}

MultiVector multiVector_antiWedge_dipole(MultiVector self, Dipole other) {
    return MultiVector(vec2(self.g6_.x) * vec2(-other.g2_.x, 0.0) + vec2(self.g6_.y) * vec2(-other.g2_.y, 0.0) + vec2(self.g6_.z) * vec2(-other.g2_.z, 0.0) + vec2(self.g6_.w) * vec2(-other.g2_.w, 0.0) + vec2(self.g7_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g7_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g7_.z) * vec2(-other.g1_.z, 0.0) + vec2(self.g8_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g8_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g8_.z) * vec2(-other.g0_.z, 0.0), vec3(self.g9_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g9_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g9_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g10_.x) * vec3(other.g2_.x, other.g2_.y, other.g2_.z) - vec3(self.g10_.y) * other.g0_, vec2(self.g9_.x) * vec2(other.g0_.x, -other.g2_.x) + vec2(self.g9_.y) * vec2(other.g0_.y, -other.g2_.y) + vec2(self.g9_.z) * vec2(other.g0_.z, -other.g2_.z) + self.g10_ * vec2(other.g2_.w), vec3(self.g0_.y) * other.g0_, vec3(self.g0_.y) * other.g1_, vec4(self.g0_.y) * other.g2_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector multiVector_antiWedge_dualNum(MultiVector self, DualNum other) {
    return MultiVector(vec2(self.g0_.x) * vec2(other.g0_.y, 0.0) + vec2(self.g0_.y) * other.g0_, self.g1_ * vec3(other.g0_.y), self.g2_ * vec2(other.g0_.y), self.g3_ * vec3(other.g0_.y), self.g4_ * vec3(other.g0_.y), self.g5_ * vec4(other.g0_.y), self.g6_ * vec4(other.g0_.y), self.g7_ * vec3(other.g0_.y), self.g8_ * vec3(other.g0_.y), self.g9_ * vec3(other.g0_.y), self.g10_ * vec2(other.g0_.y));
}

MultiVector multiVector_antiWedge_flatPoint(MultiVector self, FlatPoint other) {
    return MultiVector(vec2(self.g6_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g6_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g6_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g6_.w) * vec2(-other.g0_.w, 0.0), vec3(self.g10_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g9_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g9_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g9_.z) * vec2(0.0, -other.g0_.z) + self.g10_ * vec2(other.g0_.w), vec3(0.0), vec3(0.0), vec4(self.g0_.y) * other.g0_, vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector multiVector_antiWedge_flector(MultiVector self, Flector other) {
    return MultiVector(vec2(self.g1_.x) * vec2(other.g1_.x, 0.0) + vec2(self.g1_.y) * vec2(other.g1_.y, 0.0) + vec2(self.g1_.z) * vec2(other.g1_.z, 0.0) + vec2(self.g2_.x) * vec2(other.g1_.w, 0.0) + vec2(self.g6_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g6_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g6_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g6_.w) * vec2(-other.g0_.w, 0.0), self.g3_ * vec3(other.g1_.w) + vec3(self.g4_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g4_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g4_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g10_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g3_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g3_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g3_.z) * vec2(-other.g1_.z, 0.0) + vec2(self.g5_.x) * vec2(0.0, other.g1_.x) + vec2(self.g5_.y) * vec2(0.0, other.g1_.y) + vec2(self.g5_.z) * vec2(0.0, other.g1_.z) + vec2(self.g5_.w) * vec2(0.0, other.g1_.w) + vec2(self.g9_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g9_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g9_.z) * vec2(0.0, -other.g0_.z) + self.g10_ * vec2(other.g0_.w), vec3(self.g6_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g6_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g6_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0), vec3(self.g6_.x, self.g6_.y, self.g6_.z) * vec3(other.g1_.w) - vec3(self.g6_.w) * vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec4(self.g0_.y) * other.g0_ + vec4(self.g7_.x) * vec4(other.g1_.w, 0.0, 0.0, -other.g1_.x) + vec4(self.g7_.y) * vec4(0.0, other.g1_.w, 0.0, -other.g1_.y) + vec4(self.g7_.z) * vec4(0.0, 0.0, other.g1_.w, -other.g1_.z) + vec4(self.g8_.x) * vec4(0.0, -other.g1_.z, other.g1_.y, 0.0) + vec4(self.g8_.y) * vec4(other.g1_.z, 0.0, -other.g1_.x, 0.0) + vec4(self.g8_.z) * vec4(-other.g1_.y, other.g1_.x, 0.0, 0.0), vec4(self.g10_.x) * other.g1_, vec3(self.g9_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g9_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g9_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0), self.g9_ * vec3(other.g1_.w) - vec3(self.g10_.y) * vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec3(self.g0_.y) * vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec2(self.g0_.y) * vec2(0.0, other.g1_.w));
}

MultiVector multiVector_antiWedge_line(MultiVector self, Line other) {
    return MultiVector(vec2(self.g3_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g3_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g3_.z) * vec2(-other.g1_.z, 0.0) + vec2(self.g4_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g4_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g4_.z) * vec2(-other.g0_.z, 0.0), vec3(self.g6_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g6_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g6_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g6_.w) * other.g0_, vec2(self.g6_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g6_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g6_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g7_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g7_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g7_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g8_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g8_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g8_.z) * vec2(0.0, -other.g0_.z), vec3(self.g10_.x) * other.g0_, vec3(self.g10_.x) * other.g1_, vec4(self.g9_.x) * vec4(0.0, other.g1_.z, -other.g1_.y, -other.g0_.x) + vec4(self.g9_.y) * vec4(-other.g1_.z, 0.0, other.g1_.x, -other.g0_.y) + vec4(self.g9_.z) * vec4(other.g1_.y, -other.g1_.x, 0.0, -other.g0_.z) + vec4(self.g10_.y) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, 0.0), vec4(0.0), vec3(self.g0_.y) * other.g0_, vec3(self.g0_.y) * other.g1_, vec3(0.0), vec2(0.0));
}

MultiVector multiVector_antiWedge_motor(MultiVector self, Motor other) {
    return MultiVector(self.g0_ * vec2(other.g0_.w) + vec2(self.g3_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g3_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g3_.z) * vec2(-other.g1_.z, 0.0) + vec2(self.g4_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g4_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g4_.z) * vec2(-other.g0_.z, 0.0), self.g1_ * vec3(other.g0_.w) + vec3(self.g6_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g6_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g6_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g6_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), self.g2_ * vec2(other.g0_.w) + vec2(self.g6_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g6_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g6_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g7_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g7_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g7_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g8_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g8_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g8_.z) * vec2(0.0, -other.g0_.z), self.g3_ * vec3(other.g0_.w) + vec3(self.g10_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), self.g4_ * vec3(other.g0_.w) + vec3(self.g10_.x) * other.g1_, self.g5_ * vec4(other.g0_.w) + vec4(self.g9_.x) * vec4(0.0, other.g1_.z, -other.g1_.y, -other.g0_.x) + vec4(self.g9_.y) * vec4(-other.g1_.z, 0.0, other.g1_.x, -other.g0_.y) + vec4(self.g9_.z) * vec4(other.g1_.y, -other.g1_.x, 0.0, -other.g0_.z) + vec4(self.g10_.y) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, 0.0), self.g6_ * vec4(other.g0_.w), vec3(self.g0_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z) + self.g7_ * vec3(other.g0_.w), vec3(self.g0_.y) * other.g1_ + self.g8_ * vec3(other.g0_.w), self.g9_ * vec3(other.g0_.w), self.g10_ * vec2(other.g0_.w));
}

MultiVector multiVector_antiWedge_multiVector(MultiVector self, MultiVector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(other.g0_.y, 0.0) + vec2(self.g0_.y) * other.g0_ + vec2(self.g1_.x) * vec2(other.g9_.x, 0.0) + vec2(self.g1_.y) * vec2(other.g9_.y, 0.0) + vec2(self.g1_.z) * vec2(other.g9_.z, 0.0) + vec2(self.g2_.x) * vec2(other.g10_.y, 0.0) + vec2(self.g2_.y) * vec2(other.g10_.x, 0.0) + vec2(self.g3_.x) * vec2(-other.g8_.x, 0.0) + vec2(self.g3_.y) * vec2(-other.g8_.y, 0.0) + vec2(self.g3_.z) * vec2(-other.g8_.z, 0.0) + vec2(self.g4_.x) * vec2(-other.g7_.x, 0.0) + vec2(self.g4_.y) * vec2(-other.g7_.y, 0.0) + vec2(self.g4_.z) * vec2(-other.g7_.z, 0.0) + vec2(self.g5_.x) * vec2(-other.g6_.x, 0.0) + vec2(self.g5_.y) * vec2(-other.g6_.y, 0.0) + vec2(self.g5_.z) * vec2(-other.g6_.z, 0.0) + vec2(self.g5_.w) * vec2(-other.g6_.w, 0.0) + vec2(self.g6_.x) * vec2(-other.g5_.x, 0.0) + vec2(self.g6_.y) * vec2(-other.g5_.y, 0.0) + vec2(self.g6_.z) * vec2(-other.g5_.z, 0.0) + vec2(self.g6_.w) * vec2(-other.g5_.w, 0.0) + vec2(self.g7_.x) * vec2(-other.g4_.x, 0.0) + vec2(self.g7_.y) * vec2(-other.g4_.y, 0.0) + vec2(self.g7_.z) * vec2(-other.g4_.z, 0.0) + vec2(self.g8_.x) * vec2(-other.g3_.x, 0.0) + vec2(self.g8_.y) * vec2(-other.g3_.y, 0.0) + vec2(self.g8_.z) * vec2(-other.g3_.z, 0.0) + vec2(self.g9_.x) * vec2(other.g1_.x, 0.0) + vec2(self.g9_.y) * vec2(other.g1_.y, 0.0) + vec2(self.g9_.z) * vec2(other.g1_.z, 0.0) + vec2(self.g10_.x) * vec2(other.g2_.y, 0.0) + vec2(self.g10_.y) * vec2(other.g2_.x, 0.0), vec3(self.g0_.y) * other.g1_ + self.g1_ * vec3(other.g0_.y) + self.g3_ * vec3(other.g10_.y) + vec3(self.g4_.x) * vec3(0.0, -other.g9_.z, other.g9_.y) + vec3(self.g4_.y) * vec3(other.g9_.z, 0.0, -other.g9_.x) + vec3(self.g4_.z) * vec3(-other.g9_.y, other.g9_.x, 0.0) - vec3(self.g5_.x, self.g5_.y, self.g5_.z) * vec3(other.g10_.x) + vec3(self.g6_.x) * vec3(0.0, other.g8_.z, -other.g8_.y) + vec3(self.g6_.y) * vec3(-other.g8_.z, 0.0, other.g8_.x) + vec3(self.g6_.z) * vec3(other.g8_.y, -other.g8_.x, 0.0) + vec3(self.g6_.w) * other.g7_ + self.g7_ * vec3(other.g6_.w) + vec3(self.g8_.x) * vec3(0.0, -other.g6_.z, other.g6_.y) + vec3(self.g8_.y) * vec3(other.g6_.z, 0.0, -other.g6_.x) + vec3(self.g8_.z) * vec3(-other.g6_.y, other.g6_.x, 0.0) + vec3(self.g9_.x) * vec3(0.0, -other.g4_.z, other.g4_.y) + vec3(self.g9_.y) * vec3(other.g4_.z, 0.0, -other.g4_.x) + vec3(self.g9_.z) * vec3(-other.g4_.y, other.g4_.x, 0.0) + vec3(self.g10_.x) * vec3(other.g5_.x, other.g5_.y, other.g5_.z) - vec3(self.g10_.y) * other.g3_, vec2(self.g0_.y) * other.g2_ + self.g2_ * vec2(other.g0_.y) + vec2(self.g3_.x) * vec2(-other.g9_.x, 0.0) + vec2(self.g3_.y) * vec2(-other.g9_.y, 0.0) + vec2(self.g3_.z) * vec2(-other.g9_.z, 0.0) + vec2(self.g5_.x) * vec2(0.0, other.g9_.x) + vec2(self.g5_.y) * vec2(0.0, other.g9_.y) + vec2(self.g5_.z) * vec2(0.0, other.g9_.z) + vec2(self.g5_.w) * other.g10_ * vec2(-1.0, 1.0) + vec2(self.g6_.x) * vec2(-other.g7_.x, 0.0) + vec2(self.g6_.y) * vec2(-other.g7_.y, 0.0) + vec2(self.g6_.z) * vec2(-other.g7_.z, 0.0) - vec2(self.g7_.x) * vec2(other.g6_.x, other.g8_.x) - vec2(self.g7_.y) * vec2(other.g6_.y, other.g8_.y) - vec2(self.g7_.z) * vec2(other.g6_.z, other.g8_.z) + vec2(self.g8_.x) * vec2(0.0, -other.g7_.x) + vec2(self.g8_.y) * vec2(0.0, -other.g7_.y) + vec2(self.g8_.z) * vec2(0.0, -other.g7_.z) + vec2(self.g9_.x) * vec2(other.g3_.x, -other.g5_.x) + vec2(self.g9_.y) * vec2(other.g3_.y, -other.g5_.y) + vec2(self.g9_.z) * vec2(other.g3_.z, -other.g5_.z) + self.g10_ * vec2(other.g5_.w), vec3(self.g0_.y) * other.g3_ + self.g3_ * vec3(other.g0_.y) + vec3(self.g6_.x) * vec3(0.0, other.g9_.z, -other.g9_.y) + vec3(self.g6_.y) * vec3(-other.g9_.z, 0.0, other.g9_.x) + vec3(self.g6_.z) * vec3(other.g9_.y, -other.g9_.x, 0.0) + self.g7_ * vec3(other.g10_.x) + vec3(self.g9_.x) * vec3(0.0, -other.g6_.z, other.g6_.y) + vec3(self.g9_.y) * vec3(other.g6_.z, 0.0, -other.g6_.x) + vec3(self.g9_.z) * vec3(-other.g6_.y, other.g6_.x, 0.0) + vec3(self.g10_.x) * other.g7_, vec3(self.g0_.y) * other.g4_ + self.g4_ * vec3(other.g0_.y) + vec3(self.g6_.x, self.g6_.y, self.g6_.z) * vec3(other.g10_.y) - vec3(self.g6_.w) * other.g9_ + self.g8_ * vec3(other.g10_.x) - self.g9_ * vec3(other.g6_.w) + vec3(self.g10_.x) * other.g8_ + vec3(self.g10_.y) * vec3(other.g6_.x, other.g6_.y, other.g6_.z), vec4(self.g0_.y) * other.g5_ + self.g5_ * vec4(other.g0_.y) + vec4(self.g7_.x) * vec4(other.g10_.y, 0.0, 0.0, -other.g9_.x) + vec4(self.g7_.y) * vec4(0.0, other.g10_.y, 0.0, -other.g9_.y) + vec4(self.g7_.z) * vec4(0.0, 0.0, other.g10_.y, -other.g9_.z) + vec4(self.g8_.x) * vec4(0.0, -other.g9_.z, other.g9_.y, 0.0) + vec4(self.g8_.y) * vec4(other.g9_.z, 0.0, -other.g9_.x, 0.0) + vec4(self.g8_.z) * vec4(-other.g9_.y, other.g9_.x, 0.0, 0.0) + vec4(self.g9_.x) * vec4(0.0, other.g8_.z, -other.g8_.y, -other.g7_.x) + vec4(self.g9_.y) * vec4(-other.g8_.z, 0.0, other.g8_.x, -other.g7_.y) + vec4(self.g9_.z) * vec4(other.g8_.y, -other.g8_.x, 0.0, -other.g7_.z) + vec4(self.g10_.y) * vec4(other.g7_.x, other.g7_.y, other.g7_.z, 0.0), vec4(self.g0_.y) * other.g6_ + self.g6_ * vec4(other.g0_.y) + vec4(self.g9_.x, self.g9_.y, self.g9_.z, self.g9_.x) * vec4(-other.g10_.x, -other.g10_.x, -other.g10_.x, 0.0) + vec4(self.g10_.x) * vec4(other.g9_.x, other.g9_.y, other.g9_.z, other.g10_.y) + vec4(self.g10_.y) * vec4(0.0, 0.0, 0.0, -other.g10_.x), vec3(self.g0_.y) * other.g7_ + self.g7_ * vec3(other.g0_.y) + vec3(self.g9_.x) * vec3(0.0, other.g9_.z, -other.g9_.y) + vec3(self.g9_.y) * vec3(-other.g9_.z, 0.0, other.g9_.x) + vec3(self.g9_.z) * vec3(other.g9_.y, -other.g9_.x, 0.0), vec3(self.g0_.y) * other.g8_ + self.g8_ * vec3(other.g0_.y) + self.g9_ * vec3(other.g10_.y) - vec3(self.g10_.y) * other.g9_, vec3(self.g0_.y) * other.g9_ + self.g9_ * vec3(other.g0_.y), vec2(self.g0_.y) * other.g10_ + self.g10_ * vec2(other.g0_.y));
}

MultiVector multiVector_antiWedge_plane(MultiVector self, Plane other) {
    return MultiVector(vec2(self.g1_.x) * vec2(other.g0_.x, 0.0) + vec2(self.g1_.y) * vec2(other.g0_.y, 0.0) + vec2(self.g1_.z) * vec2(other.g0_.z, 0.0) + vec2(self.g2_.x) * vec2(other.g0_.w, 0.0), self.g3_ * vec3(other.g0_.w) + vec3(self.g4_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g4_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g4_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec2(self.g3_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g3_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g3_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g5_.x) * vec2(0.0, other.g0_.x) + vec2(self.g5_.y) * vec2(0.0, other.g0_.y) + vec2(self.g5_.z) * vec2(0.0, other.g0_.z) + vec2(self.g5_.w) * vec2(0.0, other.g0_.w), vec3(self.g6_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g6_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g6_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec3(self.g6_.x, self.g6_.y, self.g6_.z) * vec3(other.g0_.w) - vec3(self.g6_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec4(self.g7_.x) * vec4(other.g0_.w, 0.0, 0.0, -other.g0_.x) + vec4(self.g7_.y) * vec4(0.0, other.g0_.w, 0.0, -other.g0_.y) + vec4(self.g7_.z) * vec4(0.0, 0.0, other.g0_.w, -other.g0_.z) + vec4(self.g8_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g8_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g8_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0), vec4(self.g10_.x) * other.g0_, vec3(self.g9_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g9_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g9_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), self.g9_ * vec3(other.g0_.w) - vec3(self.g10_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(self.g0_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g0_.y) * vec2(0.0, other.g0_.w));
}

MultiVector multiVector_antiWedge_roundPoint(MultiVector self, RoundPoint other) {
    return MultiVector(vec2(self.g9_.x) * vec2(other.g0_.x, 0.0) + vec2(self.g9_.y) * vec2(other.g0_.y, 0.0) + vec2(self.g9_.z) * vec2(other.g0_.z, 0.0) + vec2(self.g10_.x) * vec2(other.g1_.y, 0.0) + vec2(self.g10_.y) * vec2(other.g1_.x, 0.0), vec3(self.g0_.y) * other.g0_, vec2(self.g0_.y) * other.g1_, vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

Scalar multiVector_antiWedge_scalar(MultiVector self, Scalar other) {
    return Scalar(self.g0_.y * other.g0_);
}

MultiVector multiVector_antiWedge_sphere(MultiVector self, Sphere other) {
    return MultiVector(vec2(self.g1_.x) * vec2(other.g0_.x, 0.0) + vec2(self.g1_.y) * vec2(other.g0_.y, 0.0) + vec2(self.g1_.z) * vec2(other.g0_.z, 0.0) + vec2(self.g2_.x) * vec2(other.g1_.y, 0.0) + vec2(self.g2_.y) * vec2(other.g1_.x, 0.0), self.g3_ * vec3(other.g1_.y) + vec3(self.g4_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g4_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g4_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) - vec3(self.g5_.x, self.g5_.y, self.g5_.z) * vec3(other.g1_.x), vec2(self.g3_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g3_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g3_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g5_.x) * vec2(0.0, other.g0_.x) + vec2(self.g5_.y) * vec2(0.0, other.g0_.y) + vec2(self.g5_.z) * vec2(0.0, other.g0_.z) + vec2(self.g5_.w) * other.g1_ * vec2(-1.0, 1.0), vec3(self.g6_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g6_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g6_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + self.g7_ * vec3(other.g1_.x), vec3(self.g6_.x, self.g6_.y, self.g6_.z) * vec3(other.g1_.y) - vec3(self.g6_.w) * other.g0_ + self.g8_ * vec3(other.g1_.x), vec4(self.g7_.x) * vec4(other.g1_.y, 0.0, 0.0, -other.g0_.x) + vec4(self.g7_.y) * vec4(0.0, other.g1_.y, 0.0, -other.g0_.y) + vec4(self.g7_.z) * vec4(0.0, 0.0, other.g1_.y, -other.g0_.z) + vec4(self.g8_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g8_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g8_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0), vec4(self.g9_.x, self.g9_.y, self.g9_.z, self.g9_.x) * vec4(-other.g1_.x, -other.g1_.x, -other.g1_.x, 0.0) + vec4(self.g10_.x) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.y) + vec4(self.g10_.y) * vec4(0.0, 0.0, 0.0, -other.g1_.x), vec3(self.g9_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g9_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g9_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), self.g9_ * vec3(other.g1_.y) - vec3(self.g10_.y) * other.g0_, vec3(self.g0_.y) * other.g0_, vec2(self.g0_.y) * other.g1_);
}

Plane plane_antiWedge_antiScalar(Plane self, AntiScalar other) {
    return Plane(self.g0_ * vec4(other.g0_));
}

Dipole plane_antiWedge_circle(Plane self, Circle other) {
    return Dipole(vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(0.0) - vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_.w) + vec3(self.g0_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec4(self.g0_.x) * vec4(0.0, other.g2_.z, -other.g2_.y, -other.g1_.x) + vec4(self.g0_.y) * vec4(-other.g2_.z, 0.0, other.g2_.x, -other.g1_.y) + vec4(self.g0_.z) * vec4(other.g2_.y, -other.g2_.x, 0.0, -other.g1_.z) + vec4(self.g0_.w) * vec4(other.g1_.x, other.g1_.y, other.g1_.z, 0.0));
}

RoundPoint plane_antiWedge_dipole(Plane self, Dipole other) {
    return RoundPoint(vec3(self.g0_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) - vec3(self.g0_.w) * other.g0_, vec2(self.g0_.x) * vec2(other.g0_.x, -other.g2_.x) + vec2(self.g0_.y) * vec2(other.g0_.y, -other.g2_.y) + vec2(self.g0_.z) * vec2(other.g0_.z, -other.g2_.z) + vec2(self.g0_.w) * vec2(0.0, -other.g2_.w));
}

Plane plane_antiWedge_dualNum(Plane self, DualNum other) {
    return Plane(self.g0_ * vec4(other.g0_.y));
}

RoundPoint plane_antiWedge_flatPoint(Plane self, FlatPoint other) {
    return RoundPoint(vec3(0.0), vec2(self.g0_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g0_.w) * vec2(0.0, -other.g0_.w));
}

MultiVector plane_antiWedge_flector(Plane self, Flector other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(self.g0_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g0_.w) * vec2(0.0, -other.g0_.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g1_.w) - vec3(self.g0_.w) * vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec3(0.0), vec2(0.0));
}

FlatPoint plane_antiWedge_line(Plane self, Line other) {
    return FlatPoint(vec4(self.g0_.x) * vec4(0.0, other.g1_.z, -other.g1_.y, -other.g0_.x) + vec4(self.g0_.y) * vec4(-other.g1_.z, 0.0, other.g1_.x, -other.g0_.y) + vec4(self.g0_.z) * vec4(other.g1_.y, -other.g1_.x, 0.0, -other.g0_.z) + vec4(self.g0_.w) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, 0.0));
}

Flector plane_antiWedge_motor(Plane self, Motor other) {
    return Flector(vec4(self.g0_.x) * vec4(0.0, other.g1_.z, -other.g1_.y, -other.g0_.x) + vec4(self.g0_.y) * vec4(-other.g1_.z, 0.0, other.g1_.x, -other.g0_.y) + vec4(self.g0_.z) * vec4(other.g1_.y, -other.g1_.x, 0.0, -other.g0_.z) + vec4(self.g0_.w) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, 0.0), self.g0_ * vec4(other.g0_.w));
}

MultiVector plane_antiWedge_multiVector(Plane self, MultiVector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(other.g1_.x, 0.0) + vec2(self.g0_.y) * vec2(other.g1_.y, 0.0) + vec2(self.g0_.z) * vec2(other.g1_.z, 0.0) + vec2(self.g0_.w) * vec2(other.g2_.x, 0.0), vec3(self.g0_.x) * vec3(0.0, -other.g4_.z, other.g4_.y) + vec3(self.g0_.y) * vec3(other.g4_.z, 0.0, -other.g4_.x) + vec3(self.g0_.z) * vec3(-other.g4_.y, other.g4_.x, 0.0) - vec3(self.g0_.w) * other.g3_, vec2(self.g0_.x) * vec2(other.g3_.x, -other.g5_.x) + vec2(self.g0_.y) * vec2(other.g3_.y, -other.g5_.y) + vec2(self.g0_.z) * vec2(other.g3_.z, -other.g5_.z) + vec2(self.g0_.w) * vec2(0.0, -other.g5_.w), vec3(self.g0_.x) * vec3(0.0, -other.g6_.z, other.g6_.y) + vec3(self.g0_.y) * vec3(other.g6_.z, 0.0, -other.g6_.x) + vec3(self.g0_.z) * vec3(-other.g6_.y, other.g6_.x, 0.0), vec3(0.0) - vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g6_.w) + vec3(self.g0_.w) * vec3(other.g6_.x, other.g6_.y, other.g6_.z), vec4(self.g0_.x) * vec4(0.0, other.g8_.z, -other.g8_.y, -other.g7_.x) + vec4(self.g0_.y) * vec4(-other.g8_.z, 0.0, other.g8_.x, -other.g7_.y) + vec4(self.g0_.z) * vec4(other.g8_.y, -other.g8_.x, 0.0, -other.g7_.z) + vec4(self.g0_.w) * vec4(other.g7_.x, other.g7_.y, other.g7_.z, 0.0), vec4(0.0) - self.g0_ * vec4(other.g10_.x), vec3(self.g0_.x) * vec3(0.0, other.g9_.z, -other.g9_.y) + vec3(self.g0_.y) * vec3(-other.g9_.z, 0.0, other.g9_.x) + vec3(self.g0_.z) * vec3(other.g9_.y, -other.g9_.x, 0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g10_.y) - vec3(self.g0_.w) * other.g9_, vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_.y), vec2(self.g0_.w) * vec2(0.0, other.g0_.y));
}

Line plane_antiWedge_plane(Plane self, Plane other) {
    return Line(vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_.w) - vec3(self.g0_.w) * vec3(other.g0_.x, other.g0_.y, other.g0_.z));
}

Scalar plane_antiWedge_roundPoint(Plane self, RoundPoint other) {
    return Scalar(self.g0_.x * other.g0_.x + self.g0_.y * other.g0_.y + self.g0_.z * other.g0_.z + self.g0_.w * other.g1_.x);
}

Circle plane_antiWedge_sphere(Plane self, Sphere other) {
    return Circle(vec4(0.0) - self.g0_ * vec4(other.g1_.x), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g1_.y) - vec3(self.g0_.w) * other.g0_);
}

RoundPoint roundPoint_antiWedge_antiScalar(RoundPoint self, AntiScalar other) {
    return RoundPoint(self.g0_ * vec3(other.g0_), self.g1_ * vec2(other.g0_));
}

RoundPoint roundPoint_antiWedge_dualNum(RoundPoint self, DualNum other) {
    return RoundPoint(self.g0_ * vec3(other.g0_.y), self.g1_ * vec2(other.g0_.y));
}

Scalar roundPoint_antiWedge_flector(RoundPoint self, Flector other) {
    return Scalar(self.g0_.x * other.g1_.x + self.g0_.y * other.g1_.y + self.g0_.z * other.g1_.z + self.g1_.x * other.g1_.w);
}

RoundPoint roundPoint_antiWedge_motor(RoundPoint self, Motor other) {
    return RoundPoint(self.g0_ * vec3(other.g0_.w), self.g1_ * vec2(other.g0_.w));
}

MultiVector roundPoint_antiWedge_multiVector(RoundPoint self, MultiVector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(other.g9_.x, 0.0) + vec2(self.g0_.y) * vec2(other.g9_.y, 0.0) + vec2(self.g0_.z) * vec2(other.g9_.z, 0.0) + vec2(self.g1_.x) * vec2(other.g10_.y, 0.0) + vec2(self.g1_.y) * vec2(other.g10_.x, 0.0), self.g0_ * vec3(other.g0_.y), self.g1_ * vec2(other.g0_.y), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

Scalar roundPoint_antiWedge_plane(RoundPoint self, Plane other) {
    return Scalar(self.g0_.x * other.g0_.x + self.g0_.y * other.g0_.y + self.g0_.z * other.g0_.z + self.g1_.x * other.g0_.w);
}

Scalar roundPoint_antiWedge_sphere(RoundPoint self, Sphere other) {
    return Scalar(self.g0_.x * other.g0_.x + self.g0_.y * other.g0_.y + self.g0_.z * other.g0_.z + self.g1_.x * other.g1_.y + self.g1_.y * other.g1_.x);
}

Scalar scalar_antiWedge_antiScalar(Scalar self, AntiScalar other) {
    return Scalar(self.g0_ * other.g0_);
}

Scalar scalar_antiWedge_dualNum(Scalar self, DualNum other) {
    return Scalar(self.g0_ * other.g0_.y);
}

Scalar scalar_antiWedge_motor(Scalar self, Motor other) {
    return Scalar(self.g0_ * other.g0_.w);
}

Scalar scalar_antiWedge_multiVector(Scalar self, MultiVector other) {
    return Scalar(self.g0_ * other.g0_.y);
}

Sphere sphere_antiWedge_antiScalar(Sphere self, AntiScalar other) {
    return Sphere(self.g0_ * vec3(other.g0_), self.g1_ * vec2(other.g0_));
}

Dipole sphere_antiWedge_circle(Sphere self, Circle other) {
    return Dipole(vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) + vec3(self.g1_.x) * other.g1_, vec3(0.0) - self.g0_ * vec3(other.g0_.w) + vec3(self.g1_.x) * other.g2_ + vec3(self.g1_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec4(self.g0_.x) * vec4(0.0, other.g2_.z, -other.g2_.y, -other.g1_.x) + vec4(self.g0_.y) * vec4(-other.g2_.z, 0.0, other.g2_.x, -other.g1_.y) + vec4(self.g0_.z) * vec4(other.g2_.y, -other.g2_.x, 0.0, -other.g1_.z) + vec4(self.g1_.y) * vec4(other.g1_.x, other.g1_.y, other.g1_.z, 0.0));
}

RoundPoint sphere_antiWedge_dipole(Sphere self, Dipole other) {
    return RoundPoint(vec3(self.g0_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + vec3(self.g1_.x) * vec3(other.g2_.x, other.g2_.y, other.g2_.z) - vec3(self.g1_.y) * other.g0_, vec2(self.g0_.x) * vec2(other.g0_.x, -other.g2_.x) + vec2(self.g0_.y) * vec2(other.g0_.y, -other.g2_.y) + vec2(self.g0_.z) * vec2(other.g0_.z, -other.g2_.z) + self.g1_ * vec2(other.g2_.w));
}

Sphere sphere_antiWedge_dualNum(Sphere self, DualNum other) {
    return Sphere(self.g0_ * vec3(other.g0_.y), self.g1_ * vec2(other.g0_.y));
}

RoundPoint sphere_antiWedge_flatPoint(Sphere self, FlatPoint other) {
    return RoundPoint(vec3(self.g1_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g0_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g0_.z) + self.g1_ * vec2(other.g0_.w));
}

MultiVector sphere_antiWedge_flector(Sphere self, Flector other) {
    return MultiVector(vec2(0.0), vec3(self.g1_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g0_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g0_.z) + self.g1_ * vec2(other.g0_.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g1_.x) * other.g1_, vec3(self.g0_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0), self.g0_ * vec3(other.g1_.w) - vec3(self.g1_.y) * vec3(other.g1_.x, other.g1_.y, other.g1_.z), vec3(0.0), vec2(0.0));
}

Dipole sphere_antiWedge_line(Sphere self, Line other) {
    return Dipole(vec3(self.g1_.x) * other.g0_, vec3(self.g1_.x) * other.g1_, vec4(self.g0_.x) * vec4(0.0, other.g1_.z, -other.g1_.y, -other.g0_.x) + vec4(self.g0_.y) * vec4(-other.g1_.z, 0.0, other.g1_.x, -other.g0_.y) + vec4(self.g0_.z) * vec4(other.g1_.y, -other.g1_.x, 0.0, -other.g0_.z) + vec4(self.g1_.y) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, 0.0));
}

MultiVector sphere_antiWedge_motor(Sphere self, Motor other) {
    return MultiVector(vec2(0.0), vec3(0.0), vec2(0.0), vec3(self.g1_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(self.g1_.x) * other.g1_, vec4(self.g0_.x) * vec4(0.0, other.g1_.z, -other.g1_.y, -other.g0_.x) + vec4(self.g0_.y) * vec4(-other.g1_.z, 0.0, other.g1_.x, -other.g0_.y) + vec4(self.g0_.z) * vec4(other.g1_.y, -other.g1_.x, 0.0, -other.g0_.z) + vec4(self.g1_.y) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), self.g0_ * vec3(other.g0_.w), self.g1_ * vec2(other.g0_.w));
}

MultiVector sphere_antiWedge_multiVector(Sphere self, MultiVector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(other.g1_.x, 0.0) + vec2(self.g0_.y) * vec2(other.g1_.y, 0.0) + vec2(self.g0_.z) * vec2(other.g1_.z, 0.0) + vec2(self.g1_.x) * vec2(other.g2_.y, 0.0) + vec2(self.g1_.y) * vec2(other.g2_.x, 0.0), vec3(self.g0_.x) * vec3(0.0, -other.g4_.z, other.g4_.y) + vec3(self.g0_.y) * vec3(other.g4_.z, 0.0, -other.g4_.x) + vec3(self.g0_.z) * vec3(-other.g4_.y, other.g4_.x, 0.0) + vec3(self.g1_.x) * vec3(other.g5_.x, other.g5_.y, other.g5_.z) - vec3(self.g1_.y) * other.g3_, vec2(self.g0_.x) * vec2(other.g3_.x, -other.g5_.x) + vec2(self.g0_.y) * vec2(other.g3_.y, -other.g5_.y) + vec2(self.g0_.z) * vec2(other.g3_.z, -other.g5_.z) + self.g1_ * vec2(other.g5_.w), vec3(self.g0_.x) * vec3(0.0, -other.g6_.z, other.g6_.y) + vec3(self.g0_.y) * vec3(other.g6_.z, 0.0, -other.g6_.x) + vec3(self.g0_.z) * vec3(-other.g6_.y, other.g6_.x, 0.0) + vec3(self.g1_.x) * other.g7_, vec3(0.0) - self.g0_ * vec3(other.g6_.w) + vec3(self.g1_.x) * other.g8_ + vec3(self.g1_.y) * vec3(other.g6_.x, other.g6_.y, other.g6_.z), vec4(self.g0_.x) * vec4(0.0, other.g8_.z, -other.g8_.y, -other.g7_.x) + vec4(self.g0_.y) * vec4(-other.g8_.z, 0.0, other.g8_.x, -other.g7_.y) + vec4(self.g0_.z) * vec4(other.g8_.y, -other.g8_.x, 0.0, -other.g7_.z) + vec4(self.g1_.y) * vec4(other.g7_.x, other.g7_.y, other.g7_.z, 0.0), vec4(self.g0_.x, self.g0_.y, self.g0_.z, self.g0_.x) * vec4(-other.g10_.x, -other.g10_.x, -other.g10_.x, 0.0) + vec4(self.g1_.x) * vec4(other.g9_.x, other.g9_.y, other.g9_.z, other.g10_.y) + vec4(self.g1_.y) * vec4(0.0, 0.0, 0.0, -other.g10_.x), vec3(self.g0_.x) * vec3(0.0, other.g9_.z, -other.g9_.y) + vec3(self.g0_.y) * vec3(-other.g9_.z, 0.0, other.g9_.x) + vec3(self.g0_.z) * vec3(other.g9_.y, -other.g9_.x, 0.0), self.g0_ * vec3(other.g10_.y) - vec3(self.g1_.y) * other.g9_, self.g0_ * vec3(other.g0_.y), self.g1_ * vec2(other.g0_.y));
}

Circle sphere_antiWedge_plane(Sphere self, Plane other) {
    return Circle(vec4(self.g1_.x) * other.g0_, vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), self.g0_ * vec3(other.g0_.w) - vec3(self.g1_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z));
}

Scalar sphere_antiWedge_roundPoint(Sphere self, RoundPoint other) {
    return Scalar(self.g0_.x * other.g0_.x + self.g0_.y * other.g0_.y + self.g0_.z * other.g0_.z + self.g1_.x * other.g1_.y + self.g1_.y * other.g1_.x);
}

Circle sphere_antiWedge_sphere(Sphere self, Sphere other) {
    return Circle(vec4(self.g0_.x, self.g0_.y, self.g0_.z, self.g0_.x) * vec4(-other.g1_.x, -other.g1_.x, -other.g1_.x, 0.0) + vec4(self.g1_.x) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.y) + vec4(self.g1_.y) * vec4(0.0, 0.0, 0.0, -other.g1_.x), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), self.g0_ * vec3(other.g1_.y) - vec3(self.g1_.y) * other.g0_);
}

AntiScalar antiScalar_wedge_dualNum(AntiScalar self, DualNum other) {
    return AntiScalar(self.g0_ * other.g0_.x);
}

AntiScalar antiScalar_wedge_multiVector(AntiScalar self, MultiVector other) {
    return AntiScalar(self.g0_ * other.g0_.x);
}

AntiScalar antiScalar_wedge_scalar(AntiScalar self, Scalar other) {
    return AntiScalar(self.g0_ * other.g0_);
}

AntiScalar circle_wedge_dipole(Circle self, Dipole other) {
    return AntiScalar(0.0 - self.g0_.x * other.g2_.x - self.g0_.y * other.g2_.y - self.g0_.z * other.g2_.z - self.g0_.w * other.g2_.w - self.g1_.x * other.g1_.x - self.g1_.y * other.g1_.y - self.g1_.z * other.g1_.z - self.g2_.x * other.g0_.x - self.g2_.y * other.g0_.y - self.g2_.z * other.g0_.z);
}

Circle circle_wedge_dualNum(Circle self, DualNum other) {
    return Circle(self.g0_ * vec4(other.g0_.x), self.g1_ * vec3(other.g0_.x), self.g2_ * vec3(other.g0_.x));
}

AntiScalar circle_wedge_flatPoint(Circle self, FlatPoint other) {
    return AntiScalar(0.0 - self.g0_.x * other.g0_.x - self.g0_.y * other.g0_.y - self.g0_.z * other.g0_.z - self.g0_.w * other.g0_.w);
}

AntiScalar circle_wedge_flector(Circle self, Flector other) {
    return AntiScalar(0.0 - self.g0_.x * other.g0_.x - self.g0_.y * other.g0_.y - self.g0_.z * other.g0_.z - self.g0_.w * other.g0_.w);
}

MultiVector circle_wedge_multiVector(Circle self, MultiVector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, -other.g5_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g5_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g5_.z) + vec2(self.g0_.w) * vec2(0.0, -other.g5_.w) + vec2(self.g1_.x) * vec2(0.0, -other.g4_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g4_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g4_.z) + vec2(self.g2_.x) * vec2(0.0, -other.g3_.x) + vec2(self.g2_.y) * vec2(0.0, -other.g3_.y) + vec2(self.g2_.z) * vec2(0.0, -other.g3_.z), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), self.g0_ * vec4(other.g0_.x), self.g1_ * vec3(other.g0_.x), self.g2_ * vec3(other.g0_.x), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g2_.y) + vec3(self.g1_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g1_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g1_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) - self.g2_ * vec3(other.g2_.x), vec2(self.g0_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g1_.z, 0.0) + vec2(self.g0_.w) * other.g2_ * vec2(-1.0, 1.0) + vec2(self.g2_.x) * vec2(0.0, other.g1_.x) + vec2(self.g2_.y) * vec2(0.0, other.g1_.y) + vec2(self.g2_.z) * vec2(0.0, other.g1_.z));
}

Sphere circle_wedge_roundPoint(Circle self, RoundPoint other) {
    return Sphere(vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g1_.y) + vec3(self.g1_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g1_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g1_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) - self.g2_ * vec3(other.g1_.x), vec2(self.g0_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g0_.w) * other.g1_ * vec2(-1.0, 1.0) + vec2(self.g2_.x) * vec2(0.0, other.g0_.x) + vec2(self.g2_.y) * vec2(0.0, other.g0_.y) + vec2(self.g2_.z) * vec2(0.0, other.g0_.z));
}

Circle circle_wedge_scalar(Circle self, Scalar other) {
    return Circle(self.g0_ * vec4(other.g0_), self.g1_ * vec3(other.g0_), self.g2_ * vec3(other.g0_));
}

AntiScalar dipole_wedge_circle(Dipole self, Circle other) {
    return AntiScalar(0.0 - self.g0_.x * other.g2_.x - self.g0_.y * other.g2_.y - self.g0_.z * other.g2_.z - self.g1_.x * other.g1_.x - self.g1_.y * other.g1_.y - self.g1_.z * other.g1_.z - self.g2_.x * other.g0_.x - self.g2_.y * other.g0_.y - self.g2_.z * other.g0_.z - self.g2_.w * other.g0_.w);
}

Sphere dipole_wedge_dipole(Dipole self, Dipole other) {
    return Sphere(vec3(self.g0_.x) * vec3(0.0, -other.g2_.z, other.g2_.y) + vec3(self.g0_.y) * vec3(other.g2_.z, 0.0, -other.g2_.x) + vec3(self.g0_.z) * vec3(-other.g2_.y, other.g2_.x, 0.0) + self.g1_ * vec3(other.g2_.w) + vec3(self.g2_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g2_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g2_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + vec3(self.g2_.w) * other.g1_, vec2(self.g0_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g1_.z, 0.0) - vec2(self.g1_.x) * vec2(other.g0_.x, other.g2_.x) - vec2(self.g1_.y) * vec2(other.g0_.y, other.g2_.y) - vec2(self.g1_.z) * vec2(other.g0_.z, other.g2_.z) + vec2(self.g2_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g2_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g2_.z) * vec2(0.0, -other.g1_.z));
}

Dipole dipole_wedge_dualNum(Dipole self, DualNum other) {
    return Dipole(self.g0_ * vec3(other.g0_.x), self.g1_ * vec3(other.g0_.x), self.g2_ * vec4(other.g0_.x));
}

Plane dipole_wedge_flatPoint(Dipole self, FlatPoint other) {
    return Plane(vec4(self.g0_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g0_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g0_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0) + vec4(self.g1_.x) * vec4(other.g0_.w, 0.0, 0.0, -other.g0_.x) + vec4(self.g1_.y) * vec4(0.0, other.g0_.w, 0.0, -other.g0_.y) + vec4(self.g1_.z) * vec4(0.0, 0.0, other.g0_.w, -other.g0_.z));
}

Plane dipole_wedge_flector(Dipole self, Flector other) {
    return Plane(vec4(self.g0_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g0_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g0_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0) + vec4(self.g1_.x) * vec4(other.g0_.w, 0.0, 0.0, -other.g0_.x) + vec4(self.g1_.y) * vec4(0.0, other.g0_.w, 0.0, -other.g0_.y) + vec4(self.g1_.z) * vec4(0.0, 0.0, other.g0_.w, -other.g0_.z));
}

AntiScalar dipole_wedge_line(Dipole self, Line other) {
    return AntiScalar(0.0 - self.g0_.x * other.g1_.x - self.g0_.y * other.g1_.y - self.g0_.z * other.g1_.z - self.g1_.x * other.g0_.x - self.g1_.y * other.g0_.y - self.g1_.z * other.g0_.z);
}

AntiScalar dipole_wedge_motor(Dipole self, Motor other) {
    return AntiScalar(0.0 - self.g0_.x * other.g1_.x - self.g0_.y * other.g1_.y - self.g0_.z * other.g1_.z - self.g1_.x * other.g0_.x - self.g1_.y * other.g0_.y - self.g1_.z * other.g0_.z);
}

MultiVector dipole_wedge_multiVector(Dipole self, MultiVector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, -other.g8_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g8_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g8_.z) + vec2(self.g1_.x) * vec2(0.0, -other.g7_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g7_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g7_.z) + vec2(self.g2_.x) * vec2(0.0, -other.g6_.x) + vec2(self.g2_.y) * vec2(0.0, -other.g6_.y) + vec2(self.g2_.z) * vec2(0.0, -other.g6_.z) + vec2(self.g2_.w) * vec2(0.0, -other.g6_.w), vec3(0.0), vec2(0.0), self.g0_ * vec3(other.g0_.x), self.g1_ * vec3(other.g0_.x), self.g2_ * vec4(other.g0_.x), vec4(self.g0_.x) * vec4(0.0, -other.g1_.z, other.g1_.y, 0.0) + vec4(self.g0_.y) * vec4(other.g1_.z, 0.0, -other.g1_.x, 0.0) + vec4(self.g0_.z) * vec4(-other.g1_.y, other.g1_.x, 0.0, 0.0) + vec4(self.g1_.x) * vec4(other.g2_.x, 0.0, 0.0, -other.g1_.x) + vec4(self.g1_.y) * vec4(0.0, other.g2_.x, 0.0, -other.g1_.y) + vec4(self.g1_.z) * vec4(0.0, 0.0, other.g2_.x, -other.g1_.z), self.g0_ * vec3(other.g2_.y) + vec3(self.g2_.x, self.g2_.y, self.g2_.z) * vec3(other.g2_.x) - vec3(self.g2_.w) * other.g1_, self.g1_ * vec3(other.g2_.y) + vec3(self.g2_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g2_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g2_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0), vec3(self.g0_.x) * vec3(0.0, -other.g5_.z, other.g5_.y) + vec3(self.g0_.y) * vec3(other.g5_.z, 0.0, -other.g5_.x) + vec3(self.g0_.z) * vec3(-other.g5_.y, other.g5_.x, 0.0) + self.g1_ * vec3(other.g5_.w) + vec3(self.g2_.x) * vec3(0.0, other.g3_.z, -other.g3_.y) + vec3(self.g2_.y) * vec3(-other.g3_.z, 0.0, other.g3_.x) + vec3(self.g2_.z) * vec3(other.g3_.y, -other.g3_.x, 0.0) + vec3(self.g2_.w) * other.g4_, vec2(self.g0_.x) * vec2(-other.g4_.x, 0.0) + vec2(self.g0_.y) * vec2(-other.g4_.y, 0.0) + vec2(self.g0_.z) * vec2(-other.g4_.z, 0.0) - vec2(self.g1_.x) * vec2(other.g3_.x, other.g5_.x) - vec2(self.g1_.y) * vec2(other.g3_.y, other.g5_.y) - vec2(self.g1_.z) * vec2(other.g3_.z, other.g5_.z) + vec2(self.g2_.x) * vec2(0.0, -other.g4_.x) + vec2(self.g2_.y) * vec2(0.0, -other.g4_.y) + vec2(self.g2_.z) * vec2(0.0, -other.g4_.z));
}

Circle dipole_wedge_roundPoint(Dipole self, RoundPoint other) {
    return Circle(vec4(self.g0_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g0_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g0_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0) + vec4(self.g1_.x) * vec4(other.g1_.x, 0.0, 0.0, -other.g0_.x) + vec4(self.g1_.y) * vec4(0.0, other.g1_.x, 0.0, -other.g0_.y) + vec4(self.g1_.z) * vec4(0.0, 0.0, other.g1_.x, -other.g0_.z), self.g0_ * vec3(other.g1_.y) + vec3(self.g2_.x, self.g2_.y, self.g2_.z) * vec3(other.g1_.x) - vec3(self.g2_.w) * other.g0_, self.g1_ * vec3(other.g1_.y) + vec3(self.g2_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g2_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g2_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0));
}

Dipole dipole_wedge_scalar(Dipole self, Scalar other) {
    return Dipole(self.g0_ * vec3(other.g0_), self.g1_ * vec3(other.g0_), self.g2_ * vec4(other.g0_));
}

AntiScalar dualNum_wedge_antiScalar(DualNum self, AntiScalar other) {
    return AntiScalar(self.g0_.x * other.g0_);
}

Circle dualNum_wedge_circle(DualNum self, Circle other) {
    return Circle(vec4(self.g0_.x) * other.g0_, vec3(self.g0_.x) * other.g1_, vec3(self.g0_.x) * other.g2_);
}

Dipole dualNum_wedge_dipole(DualNum self, Dipole other) {
    return Dipole(vec3(self.g0_.x) * other.g0_, vec3(self.g0_.x) * other.g1_, vec4(self.g0_.x) * other.g2_);
}

DualNum dualNum_wedge_dualNum(DualNum self, DualNum other) {
    return DualNum(vec2(self.g0_.x) * other.g0_ + vec2(self.g0_.y) * vec2(0.0, other.g0_.x));
}

FlatPoint dualNum_wedge_flatPoint(DualNum self, FlatPoint other) {
    return FlatPoint(vec4(self.g0_.x) * other.g0_);
}

Flector dualNum_wedge_flector(DualNum self, Flector other) {
    return Flector(vec4(self.g0_.x) * other.g0_, vec4(self.g0_.x) * other.g1_);
}

Line dualNum_wedge_line(DualNum self, Line other) {
    return Line(vec3(self.g0_.x) * other.g0_, vec3(self.g0_.x) * other.g1_);
}

Motor dualNum_wedge_motor(DualNum self, Motor other) {
    return Motor(vec4(self.g0_.x) * other.g0_, vec3(self.g0_.x) * other.g1_);
}

MultiVector dualNum_wedge_multiVector(DualNum self, MultiVector other) {
    return MultiVector(vec2(self.g0_.x) * other.g0_ + vec2(self.g0_.y) * vec2(0.0, other.g0_.x), vec3(self.g0_.x) * other.g1_, vec2(self.g0_.x) * other.g2_, vec3(self.g0_.x) * other.g3_, vec3(self.g0_.x) * other.g4_, vec4(self.g0_.x) * other.g5_, vec4(self.g0_.x) * other.g6_, vec3(self.g0_.x) * other.g7_, vec3(self.g0_.x) * other.g8_, vec3(self.g0_.x) * other.g9_, vec2(self.g0_.x) * other.g10_);
}

Plane dualNum_wedge_plane(DualNum self, Plane other) {
    return Plane(vec4(self.g0_.x) * other.g0_);
}

RoundPoint dualNum_wedge_roundPoint(DualNum self, RoundPoint other) {
    return RoundPoint(vec3(self.g0_.x) * other.g0_, vec2(self.g0_.x) * other.g1_);
}

DualNum dualNum_wedge_scalar(DualNum self, Scalar other) {
    return DualNum(self.g0_ * vec2(other.g0_));
}

Sphere dualNum_wedge_sphere(DualNum self, Sphere other) {
    return Sphere(vec3(self.g0_.x) * other.g0_, vec2(self.g0_.x) * other.g1_);
}

AntiScalar flatPoint_wedge_circle(FlatPoint self, Circle other) {
    return AntiScalar(0.0 - self.g0_.x * other.g0_.x - self.g0_.y * other.g0_.y - self.g0_.z * other.g0_.z - self.g0_.w * other.g0_.w);
}

Plane flatPoint_wedge_dipole(FlatPoint self, Dipole other) {
    return Plane(vec4(self.g0_.x) * vec4(0.0, other.g0_.z, -other.g0_.y, -other.g1_.x) + vec4(self.g0_.y) * vec4(-other.g0_.z, 0.0, other.g0_.x, -other.g1_.y) + vec4(self.g0_.z) * vec4(other.g0_.y, -other.g0_.x, 0.0, -other.g1_.z) + vec4(self.g0_.w) * vec4(other.g1_.x, other.g1_.y, other.g1_.z, 0.0));
}

FlatPoint flatPoint_wedge_dualNum(FlatPoint self, DualNum other) {
    return FlatPoint(self.g0_ * vec4(other.g0_.x));
}

MultiVector flatPoint_wedge_multiVector(FlatPoint self, MultiVector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, -other.g6_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g6_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g6_.z) + vec2(self.g0_.w) * vec2(0.0, -other.g6_.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0_ * vec4(other.g0_.x), vec4(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g2_.x) - vec3(self.g0_.w) * other.g1_, vec3(self.g0_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0), vec3(self.g0_.x) * vec3(0.0, other.g3_.z, -other.g3_.y) + vec3(self.g0_.y) * vec3(-other.g3_.z, 0.0, other.g3_.x) + vec3(self.g0_.z) * vec3(other.g3_.y, -other.g3_.x, 0.0) + vec3(self.g0_.w) * other.g4_, vec2(self.g0_.x) * vec2(0.0, -other.g4_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g4_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g4_.z));
}

Line flatPoint_wedge_roundPoint(FlatPoint self, RoundPoint other) {
    return Line(vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g1_.x) - vec3(self.g0_.w) * other.g0_, vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0));
}

FlatPoint flatPoint_wedge_scalar(FlatPoint self, Scalar other) {
    return FlatPoint(self.g0_ * vec4(other.g0_));
}

AntiScalar flector_wedge_circle(Flector self, Circle other) {
    return AntiScalar(0.0 - self.g0_.x * other.g0_.x - self.g0_.y * other.g0_.y - self.g0_.z * other.g0_.z - self.g0_.w * other.g0_.w);
}

Plane flector_wedge_dipole(Flector self, Dipole other) {
    return Plane(vec4(self.g0_.x) * vec4(0.0, other.g0_.z, -other.g0_.y, -other.g1_.x) + vec4(self.g0_.y) * vec4(-other.g0_.z, 0.0, other.g0_.x, -other.g1_.y) + vec4(self.g0_.z) * vec4(other.g0_.y, -other.g0_.x, 0.0, -other.g1_.z) + vec4(self.g0_.w) * vec4(other.g1_.x, other.g1_.y, other.g1_.z, 0.0));
}

Flector flector_wedge_dualNum(Flector self, DualNum other) {
    return Flector(self.g0_ * vec4(other.g0_.x), self.g1_ * vec4(other.g0_.x));
}

MultiVector flector_wedge_multiVector(Flector self, MultiVector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, -other.g6_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g6_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g6_.z) + vec2(self.g0_.w) * vec2(0.0, -other.g6_.w) + vec2(self.g1_.x) * vec2(0.0, other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, other.g1_.z) + vec2(self.g1_.w) * vec2(0.0, other.g2_.x), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g0_ * vec4(other.g0_.x), vec4(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g2_.x) - vec3(self.g0_.w) * other.g1_, vec3(self.g0_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0), vec3(self.g0_.x) * vec3(0.0, other.g3_.z, -other.g3_.y) + vec3(self.g0_.y) * vec3(-other.g3_.z, 0.0, other.g3_.x) + vec3(self.g0_.z) * vec3(other.g3_.y, -other.g3_.x, 0.0) + vec3(self.g0_.w) * other.g4_ + vec3(self.g1_.x, self.g1_.y, self.g1_.z) * vec3(other.g0_.x), vec2(self.g0_.x) * vec2(0.0, -other.g4_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g4_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g4_.z) + vec2(self.g1_.w) * vec2(0.0, other.g0_.x));
}

Motor flector_wedge_roundPoint(Flector self, RoundPoint other) {
    return Motor(self.g0_.xyzx * vec4(other.g1_.x, other.g1_.x, other.g1_.x, 0.0) + vec4(self.g0_.w) * vec4(-other.g0_.x, -other.g0_.y, -other.g0_.z, 0.0) + vec4(self.g1_.x) * vec4(0.0, 0.0, 0.0, other.g0_.x) + vec4(self.g1_.y) * vec4(0.0, 0.0, 0.0, other.g0_.y) + vec4(self.g1_.z) * vec4(0.0, 0.0, 0.0, other.g0_.z) + vec4(self.g1_.w) * vec4(0.0, 0.0, 0.0, other.g1_.x), vec3(self.g0_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g0_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g0_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0));
}

Flector flector_wedge_scalar(Flector self, Scalar other) {
    return Flector(self.g0_ * vec4(other.g0_), self.g1_ * vec4(other.g0_));
}

AntiScalar line_wedge_dipole(Line self, Dipole other) {
    return AntiScalar(0.0 - self.g0_.x * other.g1_.x - self.g0_.y * other.g1_.y - self.g0_.z * other.g1_.z - self.g1_.x * other.g0_.x - self.g1_.y * other.g0_.y - self.g1_.z * other.g0_.z);
}

Line line_wedge_dualNum(Line self, DualNum other) {
    return Line(self.g0_ * vec3(other.g0_.x), self.g1_ * vec3(other.g0_.x));
}

MultiVector line_wedge_multiVector(Line self, MultiVector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, -other.g4_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g4_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g4_.z) + vec2(self.g1_.x) * vec2(0.0, -other.g3_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g3_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g3_.z), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), self.g0_ * vec3(other.g0_.x), self.g1_ * vec3(other.g0_.x), vec3(self.g0_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) - self.g1_ * vec3(other.g2_.x), vec2(self.g1_.x) * vec2(0.0, other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, other.g1_.z));
}

Plane line_wedge_roundPoint(Line self, RoundPoint other) {
    return Plane(vec4(self.g0_.x) * vec4(0.0, other.g0_.z, -other.g0_.y, 0.0) + vec4(self.g0_.y) * vec4(-other.g0_.z, 0.0, other.g0_.x, 0.0) + vec4(self.g0_.z) * vec4(other.g0_.y, -other.g0_.x, 0.0, 0.0) + vec4(self.g1_.x) * vec4(-other.g1_.x, 0.0, 0.0, other.g0_.x) + vec4(self.g1_.y) * vec4(0.0, -other.g1_.x, 0.0, other.g0_.y) + vec4(self.g1_.z) * vec4(0.0, 0.0, -other.g1_.x, other.g0_.z));
}

Line line_wedge_scalar(Line self, Scalar other) {
    return Line(self.g0_ * vec3(other.g0_), self.g1_ * vec3(other.g0_));
}

AntiScalar motor_wedge_dipole(Motor self, Dipole other) {
    return AntiScalar(0.0 - self.g0_.x * other.g1_.x - self.g0_.y * other.g1_.y - self.g0_.z * other.g1_.z - self.g1_.x * other.g0_.x - self.g1_.y * other.g0_.y - self.g1_.z * other.g0_.z);
}

Motor motor_wedge_dualNum(Motor self, DualNum other) {
    return Motor(self.g0_ * vec4(other.g0_.x), self.g1_ * vec3(other.g0_.x));
}

MultiVector motor_wedge_multiVector(Motor self, MultiVector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, -other.g4_.x) + vec2(self.g0_.y) * vec2(0.0, -other.g4_.y) + vec2(self.g0_.z) * vec2(0.0, -other.g4_.z) + vec2(self.g0_.w) * vec2(0.0, other.g0_.x) + vec2(self.g1_.x) * vec2(0.0, -other.g3_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g3_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g3_.z), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_.x), self.g1_ * vec3(other.g0_.x), vec3(self.g0_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) - self.g1_ * vec3(other.g2_.x), vec2(self.g1_.x) * vec2(0.0, other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, other.g1_.z));
}

Plane motor_wedge_roundPoint(Motor self, RoundPoint other) {
    return Plane(vec4(self.g0_.x) * vec4(0.0, other.g0_.z, -other.g0_.y, 0.0) + vec4(self.g0_.y) * vec4(-other.g0_.z, 0.0, other.g0_.x, 0.0) + vec4(self.g0_.z) * vec4(other.g0_.y, -other.g0_.x, 0.0, 0.0) + vec4(self.g1_.x) * vec4(-other.g1_.x, 0.0, 0.0, other.g0_.x) + vec4(self.g1_.y) * vec4(0.0, -other.g1_.x, 0.0, other.g0_.y) + vec4(self.g1_.z) * vec4(0.0, 0.0, -other.g1_.x, other.g0_.z));
}

Motor motor_wedge_scalar(Motor self, Scalar other) {
    return Motor(self.g0_ * vec4(other.g0_), self.g1_ * vec3(other.g0_));
}

AntiScalar multiVector_wedge_antiScalar(MultiVector self, AntiScalar other) {
    return AntiScalar(self.g0_.x * other.g0_);
}

MultiVector multiVector_wedge_circle(MultiVector self, Circle other) {
    return MultiVector(vec2(self.g3_.x) * vec2(0.0, -other.g2_.x) + vec2(self.g3_.y) * vec2(0.0, -other.g2_.y) + vec2(self.g3_.z) * vec2(0.0, -other.g2_.z) + vec2(self.g4_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g4_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g4_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g5_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g5_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g5_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g5_.w) * vec2(0.0, -other.g0_.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(self.g0_.x) * other.g0_, vec3(self.g0_.x) * other.g1_, vec3(self.g0_.x) * other.g2_, vec3(self.g1_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g1_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g1_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g2_.x) * other.g2_ - vec3(self.g2_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g1_.x) * vec2(other.g0_.x, -other.g2_.x) + vec2(self.g1_.y) * vec2(other.g0_.y, -other.g2_.y) + vec2(self.g1_.z) * vec2(other.g0_.z, -other.g2_.z) + self.g2_ * vec2(other.g0_.w));
}

MultiVector multiVector_wedge_dipole(MultiVector self, Dipole other) {
    return MultiVector(vec2(self.g6_.x) * vec2(0.0, -other.g2_.x) + vec2(self.g6_.y) * vec2(0.0, -other.g2_.y) + vec2(self.g6_.z) * vec2(0.0, -other.g2_.z) + vec2(self.g6_.w) * vec2(0.0, -other.g2_.w) + vec2(self.g7_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g7_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g7_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g8_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g8_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g8_.z) * vec2(0.0, -other.g0_.z), vec3(0.0), vec2(0.0), vec3(self.g0_.x) * other.g0_, vec3(self.g0_.x) * other.g1_, vec4(self.g0_.x) * other.g2_, vec4(self.g1_.x) * vec4(0.0, other.g0_.z, -other.g0_.y, -other.g1_.x) + vec4(self.g1_.y) * vec4(-other.g0_.z, 0.0, other.g0_.x, -other.g1_.y) + vec4(self.g1_.z) * vec4(other.g0_.y, -other.g0_.x, 0.0, -other.g1_.z) + vec4(self.g2_.x) * vec4(other.g1_.x, other.g1_.y, other.g1_.z, 0.0), vec3(0.0) - self.g1_ * vec3(other.g2_.w) + vec3(self.g2_.x) * vec3(other.g2_.x, other.g2_.y, other.g2_.z) + vec3(self.g2_.y) * other.g0_, vec3(self.g1_.x) * vec3(0.0, -other.g2_.z, other.g2_.y) + vec3(self.g1_.y) * vec3(other.g2_.z, 0.0, -other.g2_.x) + vec3(self.g1_.z) * vec3(-other.g2_.y, other.g2_.x, 0.0) + vec3(self.g2_.y) * other.g1_, vec3(self.g3_.x) * vec3(0.0, -other.g2_.z, other.g2_.y) + vec3(self.g3_.y) * vec3(other.g2_.z, 0.0, -other.g2_.x) + vec3(self.g3_.z) * vec3(-other.g2_.y, other.g2_.x, 0.0) + self.g4_ * vec3(other.g2_.w) + vec3(self.g5_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g5_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g5_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + vec3(self.g5_.w) * other.g1_, vec2(self.g3_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g3_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g3_.z) * vec2(-other.g1_.z, 0.0) - vec2(self.g4_.x) * vec2(other.g0_.x, other.g2_.x) - vec2(self.g4_.y) * vec2(other.g0_.y, other.g2_.y) - vec2(self.g4_.z) * vec2(other.g0_.z, other.g2_.z) + vec2(self.g5_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g5_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g5_.z) * vec2(0.0, -other.g1_.z));
}

MultiVector multiVector_wedge_dualNum(MultiVector self, DualNum other) {
    return MultiVector(vec2(self.g0_.x) * other.g0_ + vec2(self.g0_.y) * vec2(0.0, other.g0_.x), self.g1_ * vec3(other.g0_.x), self.g2_ * vec2(other.g0_.x), self.g3_ * vec3(other.g0_.x), self.g4_ * vec3(other.g0_.x), self.g5_ * vec4(other.g0_.x), self.g6_ * vec4(other.g0_.x), self.g7_ * vec3(other.g0_.x), self.g8_ * vec3(other.g0_.x), self.g9_ * vec3(other.g0_.x), self.g10_ * vec2(other.g0_.x));
}

MultiVector multiVector_wedge_flatPoint(MultiVector self, FlatPoint other) {
    return MultiVector(vec2(self.g6_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g6_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g6_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g6_.w) * vec2(0.0, -other.g0_.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(self.g0_.x) * other.g0_, vec4(0.0), vec3(0.0) - self.g1_ * vec3(other.g0_.w) + vec3(self.g2_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(self.g3_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g3_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g3_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) + self.g4_ * vec3(other.g0_.w), vec2(self.g4_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g4_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g4_.z) * vec2(0.0, -other.g0_.z));
}

MultiVector multiVector_wedge_flector(MultiVector self, Flector other) {
    return MultiVector(vec2(self.g1_.x) * vec2(0.0, other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, other.g1_.z) + vec2(self.g2_.x) * vec2(0.0, other.g1_.w) + vec2(self.g6_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g6_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g6_.z) * vec2(0.0, -other.g0_.z) + vec2(self.g6_.w) * vec2(0.0, -other.g0_.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(self.g0_.x) * other.g0_, vec4(0.0), vec3(0.0) - self.g1_ * vec3(other.g0_.w) + vec3(self.g2_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec3(self.g0_.x) * vec3(other.g1_.x, other.g1_.y, other.g1_.z) + vec3(self.g3_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g3_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g3_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0) + self.g4_ * vec3(other.g0_.w), vec2(self.g0_.x) * vec2(0.0, other.g1_.w) + vec2(self.g4_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g4_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g4_.z) * vec2(0.0, -other.g0_.z));
}

MultiVector multiVector_wedge_line(MultiVector self, Line other) {
    return MultiVector(vec2(self.g3_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g3_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g3_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g4_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g4_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g4_.z) * vec2(0.0, -other.g0_.z), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0_.x) * other.g0_, vec3(self.g0_.x) * other.g1_, vec3(self.g1_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g1_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g1_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + vec3(self.g2_.x) * other.g1_, vec2(self.g1_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g1_.z));
}

MultiVector multiVector_wedge_motor(MultiVector self, Motor other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, other.g0_.w) + vec2(self.g3_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g3_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g3_.z) * vec2(0.0, -other.g1_.z) + vec2(self.g4_.x) * vec2(0.0, -other.g0_.x) + vec2(self.g4_.y) * vec2(0.0, -other.g0_.y) + vec2(self.g4_.z) * vec2(0.0, -other.g0_.z), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(self.g0_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(self.g0_.x) * other.g1_, vec3(self.g1_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g1_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g1_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) + vec3(self.g2_.x) * other.g1_, vec2(self.g1_.x) * vec2(0.0, -other.g1_.x) + vec2(self.g1_.y) * vec2(0.0, -other.g1_.y) + vec2(self.g1_.z) * vec2(0.0, -other.g1_.z));
}

MultiVector multiVector_wedge_multiVector(MultiVector self, MultiVector other) {
    return MultiVector(vec2(self.g0_.x) * other.g0_ + vec2(self.g0_.y) * vec2(0.0, other.g0_.x) + vec2(self.g1_.x) * vec2(0.0, other.g9_.x) + vec2(self.g1_.y) * vec2(0.0, other.g9_.y) + vec2(self.g1_.z) * vec2(0.0, other.g9_.z) + vec2(self.g2_.x) * vec2(0.0, other.g10_.y) + vec2(self.g2_.y) * vec2(0.0, other.g10_.x) + vec2(self.g3_.x) * vec2(0.0, -other.g8_.x) + vec2(self.g3_.y) * vec2(0.0, -other.g8_.y) + vec2(self.g3_.z) * vec2(0.0, -other.g8_.z) + vec2(self.g4_.x) * vec2(0.0, -other.g7_.x) + vec2(self.g4_.y) * vec2(0.0, -other.g7_.y) + vec2(self.g4_.z) * vec2(0.0, -other.g7_.z) + vec2(self.g5_.x) * vec2(0.0, -other.g6_.x) + vec2(self.g5_.y) * vec2(0.0, -other.g6_.y) + vec2(self.g5_.z) * vec2(0.0, -other.g6_.z) + vec2(self.g5_.w) * vec2(0.0, -other.g6_.w) + vec2(self.g6_.x) * vec2(0.0, -other.g5_.x) + vec2(self.g6_.y) * vec2(0.0, -other.g5_.y) + vec2(self.g6_.z) * vec2(0.0, -other.g5_.z) + vec2(self.g6_.w) * vec2(0.0, -other.g5_.w) + vec2(self.g7_.x) * vec2(0.0, -other.g4_.x) + vec2(self.g7_.y) * vec2(0.0, -other.g4_.y) + vec2(self.g7_.z) * vec2(0.0, -other.g4_.z) + vec2(self.g8_.x) * vec2(0.0, -other.g3_.x) + vec2(self.g8_.y) * vec2(0.0, -other.g3_.y) + vec2(self.g8_.z) * vec2(0.0, -other.g3_.z) + vec2(self.g9_.x) * vec2(0.0, other.g1_.x) + vec2(self.g9_.y) * vec2(0.0, other.g1_.y) + vec2(self.g9_.z) * vec2(0.0, other.g1_.z) + vec2(self.g10_.x) * vec2(0.0, other.g2_.y) + vec2(self.g10_.y) * vec2(0.0, other.g2_.x), vec3(self.g0_.x) * other.g1_ + self.g1_ * vec3(other.g0_.x), vec2(self.g0_.x) * other.g2_ + self.g2_ * vec2(other.g0_.x), vec3(self.g0_.x) * other.g3_ - self.g1_ * vec3(other.g2_.x) + vec3(self.g2_.x) * other.g1_ + self.g3_ * vec3(other.g0_.x), vec3(self.g0_.x) * other.g4_ + vec3(self.g1_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g1_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g1_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0) + self.g4_ * vec3(other.g0_.x), vec4(self.g0_.x) * other.g5_ + vec4(self.g1_.x, self.g1_.y, self.g1_.z, self.g1_.x) * vec4(other.g2_.y, other.g2_.y, other.g2_.y, 0.0) + vec4(self.g2_.x) * vec4(0.0, 0.0, 0.0, other.g2_.y) - vec4(self.g2_.y) * vec4(other.g1_.x, other.g1_.y, other.g1_.z, other.g2_.x) + self.g5_ * vec4(other.g0_.x), vec4(self.g0_.x) * other.g6_ + vec4(self.g1_.x) * vec4(0.0, other.g3_.z, -other.g3_.y, -other.g4_.x) + vec4(self.g1_.y) * vec4(-other.g3_.z, 0.0, other.g3_.x, -other.g4_.y) + vec4(self.g1_.z) * vec4(other.g3_.y, -other.g3_.x, 0.0, -other.g4_.z) + vec4(self.g2_.x) * vec4(other.g4_.x, other.g4_.y, other.g4_.z, 0.0) + vec4(self.g3_.x) * vec4(0.0, -other.g1_.z, other.g1_.y, 0.0) + vec4(self.g3_.y) * vec4(other.g1_.z, 0.0, -other.g1_.x, 0.0) + vec4(self.g3_.z) * vec4(-other.g1_.y, other.g1_.x, 0.0, 0.0) + vec4(self.g4_.x) * vec4(other.g2_.x, 0.0, 0.0, -other.g1_.x) + vec4(self.g4_.y) * vec4(0.0, other.g2_.x, 0.0, -other.g1_.y) + vec4(self.g4_.z) * vec4(0.0, 0.0, other.g2_.x, -other.g1_.z) + self.g6_ * vec4(other.g0_.x), vec3(self.g0_.x) * other.g7_ - self.g1_ * vec3(other.g5_.w) + vec3(self.g2_.x) * vec3(other.g5_.x, other.g5_.y, other.g5_.z) + vec3(self.g2_.y) * other.g3_ + self.g3_ * vec3(other.g2_.y) + vec3(self.g5_.x, self.g5_.y, self.g5_.z) * vec3(other.g2_.x) - vec3(self.g5_.w) * other.g1_ + self.g7_ * vec3(other.g0_.x), vec3(self.g0_.x) * other.g8_ + vec3(self.g1_.x) * vec3(0.0, -other.g5_.z, other.g5_.y) + vec3(self.g1_.y) * vec3(other.g5_.z, 0.0, -other.g5_.x) + vec3(self.g1_.z) * vec3(-other.g5_.y, other.g5_.x, 0.0) + vec3(self.g2_.y) * other.g4_ + self.g4_ * vec3(other.g2_.y) + vec3(self.g5_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g5_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g5_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + self.g8_ * vec3(other.g0_.x), vec3(self.g0_.x) * other.g9_ + vec3(self.g1_.x) * vec3(0.0, other.g7_.z, -other.g7_.y) + vec3(self.g1_.y) * vec3(-other.g7_.z, 0.0, other.g7_.x) + vec3(self.g1_.z) * vec3(other.g7_.y, -other.g7_.x, 0.0) + vec3(self.g2_.x) * other.g8_ - vec3(self.g2_.y) * vec3(other.g6_.x, other.g6_.y, other.g6_.z) + vec3(self.g3_.x) * vec3(0.0, -other.g5_.z, other.g5_.y) + vec3(self.g3_.y) * vec3(other.g5_.z, 0.0, -other.g5_.x) + vec3(self.g3_.z) * vec3(-other.g5_.y, other.g5_.x, 0.0) + self.g4_ * vec3(other.g5_.w) + vec3(self.g5_.x) * vec3(0.0, other.g3_.z, -other.g3_.y) + vec3(self.g5_.y) * vec3(-other.g3_.z, 0.0, other.g3_.x) + vec3(self.g5_.z) * vec3(other.g3_.y, -other.g3_.x, 0.0) + vec3(self.g5_.w) * other.g4_ + vec3(self.g6_.x, self.g6_.y, self.g6_.z) * vec3(other.g2_.y) + vec3(self.g7_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g7_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g7_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) - self.g8_ * vec3(other.g2_.x) + self.g9_ * vec3(other.g0_.x), vec2(self.g0_.x) * other.g10_ + vec2(self.g1_.x) * vec2(other.g6_.x, -other.g8_.x) + vec2(self.g1_.y) * vec2(other.g6_.y, -other.g8_.y) + vec2(self.g1_.z) * vec2(other.g6_.z, -other.g8_.z) + self.g2_ * vec2(other.g6_.w) + vec2(self.g3_.x) * vec2(-other.g4_.x, 0.0) + vec2(self.g3_.y) * vec2(-other.g4_.y, 0.0) + vec2(self.g3_.z) * vec2(-other.g4_.z, 0.0) - vec2(self.g4_.x) * vec2(other.g3_.x, other.g5_.x) - vec2(self.g4_.y) * vec2(other.g3_.y, other.g5_.y) - vec2(self.g4_.z) * vec2(other.g3_.z, other.g5_.z) + vec2(self.g5_.x) * vec2(0.0, -other.g4_.x) + vec2(self.g5_.y) * vec2(0.0, -other.g4_.y) + vec2(self.g5_.z) * vec2(0.0, -other.g4_.z) + vec2(self.g6_.x) * vec2(-other.g1_.x, 0.0) + vec2(self.g6_.y) * vec2(-other.g1_.y, 0.0) + vec2(self.g6_.z) * vec2(-other.g1_.z, 0.0) + vec2(self.g6_.w) * other.g2_ * vec2(-1.0, 1.0) + vec2(self.g8_.x) * vec2(0.0, other.g1_.x) + vec2(self.g8_.y) * vec2(0.0, other.g1_.y) + vec2(self.g8_.z) * vec2(0.0, other.g1_.z) + self.g10_ * vec2(other.g0_.x));
}

MultiVector multiVector_wedge_plane(MultiVector self, Plane other) {
    return MultiVector(vec2(self.g1_.x) * vec2(0.0, other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, other.g0_.z) + vec2(self.g2_.x) * vec2(0.0, other.g0_.w), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g0_.x) * vec2(0.0, other.g0_.w));
}

MultiVector multiVector_wedge_roundPoint(MultiVector self, RoundPoint other) {
    return MultiVector(vec2(self.g9_.x) * vec2(0.0, other.g0_.x) + vec2(self.g9_.y) * vec2(0.0, other.g0_.y) + vec2(self.g9_.z) * vec2(0.0, other.g0_.z) + vec2(self.g10_.x) * vec2(0.0, other.g1_.y) + vec2(self.g10_.y) * vec2(0.0, other.g1_.x), vec3(self.g0_.x) * other.g0_, vec2(self.g0_.x) * other.g1_, vec3(0.0) - self.g1_ * vec3(other.g1_.x) + vec3(self.g2_.x) * other.g0_, vec3(self.g1_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g1_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g1_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec4(self.g1_.x, self.g1_.y, self.g1_.z, self.g1_.x) * vec4(other.g1_.y, other.g1_.y, other.g1_.y, 0.0) + vec4(self.g2_.x) * vec4(0.0, 0.0, 0.0, other.g1_.y) - vec4(self.g2_.y) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.x), vec4(self.g3_.x) * vec4(0.0, -other.g0_.z, other.g0_.y, 0.0) + vec4(self.g3_.y) * vec4(other.g0_.z, 0.0, -other.g0_.x, 0.0) + vec4(self.g3_.z) * vec4(-other.g0_.y, other.g0_.x, 0.0, 0.0) + vec4(self.g4_.x) * vec4(other.g1_.x, 0.0, 0.0, -other.g0_.x) + vec4(self.g4_.y) * vec4(0.0, other.g1_.x, 0.0, -other.g0_.y) + vec4(self.g4_.z) * vec4(0.0, 0.0, other.g1_.x, -other.g0_.z), self.g3_ * vec3(other.g1_.y) + vec3(self.g5_.x, self.g5_.y, self.g5_.z) * vec3(other.g1_.x) - vec3(self.g5_.w) * other.g0_, self.g4_ * vec3(other.g1_.y) + vec3(self.g5_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g5_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g5_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0), vec3(self.g6_.x, self.g6_.y, self.g6_.z) * vec3(other.g1_.y) + vec3(self.g7_.x) * vec3(0.0, other.g0_.z, -other.g0_.y) + vec3(self.g7_.y) * vec3(-other.g0_.z, 0.0, other.g0_.x) + vec3(self.g7_.z) * vec3(other.g0_.y, -other.g0_.x, 0.0) - self.g8_ * vec3(other.g1_.x), vec2(self.g6_.x) * vec2(-other.g0_.x, 0.0) + vec2(self.g6_.y) * vec2(-other.g0_.y, 0.0) + vec2(self.g6_.z) * vec2(-other.g0_.z, 0.0) + vec2(self.g6_.w) * other.g1_ * vec2(-1.0, 1.0) + vec2(self.g8_.x) * vec2(0.0, other.g0_.x) + vec2(self.g8_.y) * vec2(0.0, other.g0_.y) + vec2(self.g8_.z) * vec2(0.0, other.g0_.z));
}

MultiVector multiVector_wedge_scalar(MultiVector self, Scalar other) {
    return MultiVector(self.g0_ * vec2(other.g0_), self.g1_ * vec3(other.g0_), self.g2_ * vec2(other.g0_), self.g3_ * vec3(other.g0_), self.g4_ * vec3(other.g0_), self.g5_ * vec4(other.g0_), self.g6_ * vec4(other.g0_), self.g7_ * vec3(other.g0_), self.g8_ * vec3(other.g0_), self.g9_ * vec3(other.g0_), self.g10_ * vec2(other.g0_));
}

MultiVector multiVector_wedge_sphere(MultiVector self, Sphere other) {
    return MultiVector(vec2(self.g1_.x) * vec2(0.0, other.g0_.x) + vec2(self.g1_.y) * vec2(0.0, other.g0_.y) + vec2(self.g1_.z) * vec2(0.0, other.g0_.z) + vec2(self.g2_.x) * vec2(0.0, other.g1_.y) + vec2(self.g2_.y) * vec2(0.0, other.g1_.x), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x) * other.g0_, vec2(self.g0_.x) * other.g1_);
}

Plane plane_wedge_dualNum(Plane self, DualNum other) {
    return Plane(self.g0_ * vec4(other.g0_.x));
}

MultiVector plane_wedge_multiVector(Plane self, MultiVector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, other.g1_.z) + vec2(self.g0_.w) * vec2(0.0, other.g2_.x), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z) * vec3(other.g0_.x), vec2(self.g0_.w) * vec2(0.0, other.g0_.x));
}

AntiScalar plane_wedge_roundPoint(Plane self, RoundPoint other) {
    return AntiScalar(self.g0_.x * other.g0_.x + self.g0_.y * other.g0_.y + self.g0_.z * other.g0_.z + self.g0_.w * other.g1_.x);
}

Plane plane_wedge_scalar(Plane self, Scalar other) {
    return Plane(self.g0_ * vec4(other.g0_));
}

Sphere roundPoint_wedge_circle(RoundPoint self, Circle other) {
    return Sphere(vec3(self.g0_.x) * vec3(0.0, other.g1_.z, -other.g1_.y) + vec3(self.g0_.y) * vec3(-other.g1_.z, 0.0, other.g1_.x) + vec3(self.g0_.z) * vec3(other.g1_.y, -other.g1_.x, 0.0) + vec3(self.g1_.x) * other.g2_ - vec3(self.g1_.y) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec2(self.g0_.x) * vec2(other.g0_.x, -other.g2_.x) + vec2(self.g0_.y) * vec2(other.g0_.y, -other.g2_.y) + vec2(self.g0_.z) * vec2(other.g0_.z, -other.g2_.z) + self.g1_ * vec2(other.g0_.w));
}

Circle roundPoint_wedge_dipole(RoundPoint self, Dipole other) {
    return Circle(vec4(self.g0_.x) * vec4(0.0, other.g0_.z, -other.g0_.y, -other.g1_.x) + vec4(self.g0_.y) * vec4(-other.g0_.z, 0.0, other.g0_.x, -other.g1_.y) + vec4(self.g0_.z) * vec4(other.g0_.y, -other.g0_.x, 0.0, -other.g1_.z) + vec4(self.g1_.x) * vec4(other.g1_.x, other.g1_.y, other.g1_.z, 0.0), vec3(0.0) - self.g0_ * vec3(other.g2_.w) + vec3(self.g1_.x) * vec3(other.g2_.x, other.g2_.y, other.g2_.z) + vec3(self.g1_.y) * other.g0_, vec3(self.g0_.x) * vec3(0.0, -other.g2_.z, other.g2_.y) + vec3(self.g0_.y) * vec3(other.g2_.z, 0.0, -other.g2_.x) + vec3(self.g0_.z) * vec3(-other.g2_.y, other.g2_.x, 0.0) + vec3(self.g1_.y) * other.g1_);
}

RoundPoint roundPoint_wedge_dualNum(RoundPoint self, DualNum other) {
    return RoundPoint(self.g0_ * vec3(other.g0_.x), self.g1_ * vec2(other.g0_.x));
}

Line roundPoint_wedge_flatPoint(RoundPoint self, FlatPoint other) {
    return Line(vec3(0.0) - self.g0_ * vec3(other.g0_.w) + vec3(self.g1_.x) * vec3(other.g0_.x, other.g0_.y, other.g0_.z), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0));
}

Motor roundPoint_wedge_flector(RoundPoint self, Flector other) {
    return Motor(vec4(self.g0_.x) * vec4(-other.g0_.w, 0.0, 0.0, other.g1_.x) + vec4(self.g0_.y) * vec4(0.0, -other.g0_.w, 0.0, other.g1_.y) + vec4(self.g0_.z) * vec4(0.0, 0.0, -other.g0_.w, other.g1_.z) + vec4(self.g1_.x) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.w), vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0));
}

Plane roundPoint_wedge_line(RoundPoint self, Line other) {
    return Plane(vec4(self.g0_.x) * vec4(0.0, other.g0_.z, -other.g0_.y, -other.g1_.x) + vec4(self.g0_.y) * vec4(-other.g0_.z, 0.0, other.g0_.x, -other.g1_.y) + vec4(self.g0_.z) * vec4(other.g0_.y, -other.g0_.x, 0.0, -other.g1_.z) + vec4(self.g1_.x) * vec4(other.g1_.x, other.g1_.y, other.g1_.z, 0.0));
}

Plane roundPoint_wedge_motor(RoundPoint self, Motor other) {
    return Plane(vec4(self.g0_.x) * vec4(0.0, other.g0_.z, -other.g0_.y, -other.g1_.x) + vec4(self.g0_.y) * vec4(-other.g0_.z, 0.0, other.g0_.x, -other.g1_.y) + vec4(self.g0_.z) * vec4(other.g0_.y, -other.g0_.x, 0.0, -other.g1_.z) + vec4(self.g1_.x) * vec4(other.g1_.x, other.g1_.y, other.g1_.z, 0.0));
}

MultiVector roundPoint_wedge_multiVector(RoundPoint self, MultiVector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, other.g9_.x) + vec2(self.g0_.y) * vec2(0.0, other.g9_.y) + vec2(self.g0_.z) * vec2(0.0, other.g9_.z) + vec2(self.g1_.x) * vec2(0.0, other.g10_.y) + vec2(self.g1_.y) * vec2(0.0, other.g10_.x), self.g0_ * vec3(other.g0_.x), self.g1_ * vec2(other.g0_.x), vec3(0.0) - self.g0_ * vec3(other.g2_.x) + vec3(self.g1_.x) * other.g1_, vec3(self.g0_.x) * vec3(0.0, -other.g1_.z, other.g1_.y) + vec3(self.g0_.y) * vec3(other.g1_.z, 0.0, -other.g1_.x) + vec3(self.g0_.z) * vec3(-other.g1_.y, other.g1_.x, 0.0), vec4(self.g0_.x, self.g0_.y, self.g0_.z, self.g0_.x) * vec4(other.g2_.y, other.g2_.y, other.g2_.y, 0.0) + vec4(self.g1_.x) * vec4(0.0, 0.0, 0.0, other.g2_.y) - vec4(self.g1_.y) * vec4(other.g1_.x, other.g1_.y, other.g1_.z, other.g2_.x), vec4(self.g0_.x) * vec4(0.0, other.g3_.z, -other.g3_.y, -other.g4_.x) + vec4(self.g0_.y) * vec4(-other.g3_.z, 0.0, other.g3_.x, -other.g4_.y) + vec4(self.g0_.z) * vec4(other.g3_.y, -other.g3_.x, 0.0, -other.g4_.z) + vec4(self.g1_.x) * vec4(other.g4_.x, other.g4_.y, other.g4_.z, 0.0), vec3(0.0) - self.g0_ * vec3(other.g5_.w) + vec3(self.g1_.x) * vec3(other.g5_.x, other.g5_.y, other.g5_.z) + vec3(self.g1_.y) * other.g3_, vec3(self.g0_.x) * vec3(0.0, -other.g5_.z, other.g5_.y) + vec3(self.g0_.y) * vec3(other.g5_.z, 0.0, -other.g5_.x) + vec3(self.g0_.z) * vec3(-other.g5_.y, other.g5_.x, 0.0) + vec3(self.g1_.y) * other.g4_, vec3(self.g0_.x) * vec3(0.0, other.g7_.z, -other.g7_.y) + vec3(self.g0_.y) * vec3(-other.g7_.z, 0.0, other.g7_.x) + vec3(self.g0_.z) * vec3(other.g7_.y, -other.g7_.x, 0.0) + vec3(self.g1_.x) * other.g8_ - vec3(self.g1_.y) * vec3(other.g6_.x, other.g6_.y, other.g6_.z), vec2(self.g0_.x) * vec2(other.g6_.x, -other.g8_.x) + vec2(self.g0_.y) * vec2(other.g6_.y, -other.g8_.y) + vec2(self.g0_.z) * vec2(other.g6_.z, -other.g8_.z) + self.g1_ * vec2(other.g6_.w));
}

AntiScalar roundPoint_wedge_plane(RoundPoint self, Plane other) {
    return AntiScalar(self.g0_.x * other.g0_.x + self.g0_.y * other.g0_.y + self.g0_.z * other.g0_.z + self.g1_.x * other.g0_.w);
}

Dipole roundPoint_wedge_roundPoint(RoundPoint self, RoundPoint other) {
    return Dipole(vec3(0.0) - self.g0_ * vec3(other.g1_.x) + vec3(self.g1_.x) * other.g0_, vec3(self.g0_.x) * vec3(0.0, -other.g0_.z, other.g0_.y) + vec3(self.g0_.y) * vec3(other.g0_.z, 0.0, -other.g0_.x) + vec3(self.g0_.z) * vec3(-other.g0_.y, other.g0_.x, 0.0), vec4(self.g0_.x, self.g0_.y, self.g0_.z, self.g0_.x) * vec4(other.g1_.y, other.g1_.y, other.g1_.y, 0.0) + vec4(self.g1_.x) * vec4(0.0, 0.0, 0.0, other.g1_.y) - vec4(self.g1_.y) * vec4(other.g0_.x, other.g0_.y, other.g0_.z, other.g1_.x));
}

RoundPoint roundPoint_wedge_scalar(RoundPoint self, Scalar other) {
    return RoundPoint(self.g0_ * vec3(other.g0_), self.g1_ * vec2(other.g0_));
}

AntiScalar roundPoint_wedge_sphere(RoundPoint self, Sphere other) {
    return AntiScalar(self.g0_.x * other.g0_.x + self.g0_.y * other.g0_.y + self.g0_.z * other.g0_.z + self.g1_.x * other.g1_.y + self.g1_.y * other.g1_.x);
}

AntiScalar scalar_wedge_antiScalar(Scalar self, AntiScalar other) {
    return AntiScalar(self.g0_ * other.g0_);
}

Circle scalar_wedge_circle(Scalar self, Circle other) {
    return Circle(vec4(self.g0_) * other.g0_, vec3(self.g0_) * other.g1_, vec3(self.g0_) * other.g2_);
}

Dipole scalar_wedge_dipole(Scalar self, Dipole other) {
    return Dipole(vec3(self.g0_) * other.g0_, vec3(self.g0_) * other.g1_, vec4(self.g0_) * other.g2_);
}

DualNum scalar_wedge_dualNum(Scalar self, DualNum other) {
    return DualNum(vec2(self.g0_) * other.g0_);
}

FlatPoint scalar_wedge_flatPoint(Scalar self, FlatPoint other) {
    return FlatPoint(vec4(self.g0_) * other.g0_);
}

Flector scalar_wedge_flector(Scalar self, Flector other) {
    return Flector(vec4(self.g0_) * other.g0_, vec4(self.g0_) * other.g1_);
}

Line scalar_wedge_line(Scalar self, Line other) {
    return Line(vec3(self.g0_) * other.g0_, vec3(self.g0_) * other.g1_);
}

Motor scalar_wedge_motor(Scalar self, Motor other) {
    return Motor(vec4(self.g0_) * other.g0_, vec3(self.g0_) * other.g1_);
}

MultiVector scalar_wedge_multiVector(Scalar self, MultiVector other) {
    return MultiVector(vec2(self.g0_) * other.g0_, vec3(self.g0_) * other.g1_, vec2(self.g0_) * other.g2_, vec3(self.g0_) * other.g3_, vec3(self.g0_) * other.g4_, vec4(self.g0_) * other.g5_, vec4(self.g0_) * other.g6_, vec3(self.g0_) * other.g7_, vec3(self.g0_) * other.g8_, vec3(self.g0_) * other.g9_, vec2(self.g0_) * other.g10_);
}

Plane scalar_wedge_plane(Scalar self, Plane other) {
    return Plane(vec4(self.g0_) * other.g0_);
}

RoundPoint scalar_wedge_roundPoint(Scalar self, RoundPoint other) {
    return RoundPoint(vec3(self.g0_) * other.g0_, vec2(self.g0_) * other.g1_);
}

Scalar scalar_wedge_scalar(Scalar self, Scalar other) {
    return Scalar(self.g0_ * other.g0_);
}

Sphere scalar_wedge_sphere(Scalar self, Sphere other) {
    return Sphere(vec3(self.g0_) * other.g0_, vec2(self.g0_) * other.g1_);
}

Sphere sphere_wedge_dualNum(Sphere self, DualNum other) {
    return Sphere(self.g0_ * vec3(other.g0_.x), self.g1_ * vec2(other.g0_.x));
}

MultiVector sphere_wedge_multiVector(Sphere self, MultiVector other) {
    return MultiVector(vec2(self.g0_.x) * vec2(0.0, other.g1_.x) + vec2(self.g0_.y) * vec2(0.0, other.g1_.y) + vec2(self.g0_.z) * vec2(0.0, other.g1_.z) + vec2(self.g1_.x) * vec2(0.0, other.g2_.y) + vec2(self.g1_.y) * vec2(0.0, other.g2_.x), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), self.g0_ * vec3(other.g0_.x), self.g1_ * vec2(other.g0_.x));
}

AntiScalar sphere_wedge_roundPoint(Sphere self, RoundPoint other) {
    return AntiScalar(self.g0_.x * other.g0_.x + self.g0_.y * other.g0_.y + self.g0_.z * other.g0_.z + self.g1_.x * other.g1_.y + self.g1_.y * other.g1_.x);
}

Sphere sphere_wedge_scalar(Sphere self, Scalar other) {
    return Sphere(self.g0_ * vec3(other.g0_), self.g1_ * vec2(other.g0_));
}

AntiScalar antiScalar_antiDot_antiScalar(AntiScalar self, AntiScalar other) {
    return AntiScalar(self.g0_ * other.g0_);
}

AntiScalar antiScalar_antiDot_dualNum(AntiScalar self, DualNum other) {
    return AntiScalar(self.g0_ * other.g0_.y);
}

AntiScalar antiScalar_antiDot_motor(AntiScalar self, Motor other) {
    return AntiScalar(self.g0_ * other.g0_.w);
}

AntiScalar antiScalar_antiDot_multiVector(AntiScalar self, MultiVector other) {
    return AntiScalar(self.g0_ * other.g0_.y);
}

AntiScalar circle_antiDot_circle(Circle self, Circle other) {
    return AntiScalar(0.0 - self.g0_.x * other.g2_.x - self.g0_.y * other.g2_.y - self.g0_.z * other.g2_.z + self.g0_.w * other.g0_.w - self.g1_.x * other.g1_.x - self.g1_.y * other.g1_.y - self.g1_.z * other.g1_.z - self.g2_.x * other.g0_.x - self.g2_.y * other.g0_.y - self.g2_.z * other.g0_.z);
}

AntiScalar circle_antiDot_line(Circle self, Line other) {
    return AntiScalar(0.0 - self.g0_.x * other.g1_.x - self.g0_.y * other.g1_.y - self.g0_.z * other.g1_.z - self.g1_.x * other.g0_.x - self.g1_.y * other.g0_.y - self.g1_.z * other.g0_.z);
}

AntiScalar circle_antiDot_motor(Circle self, Motor other) {
    return AntiScalar(0.0 - self.g0_.x * other.g1_.x - self.g0_.y * other.g1_.y - self.g0_.z * other.g1_.z - self.g1_.x * other.g0_.x - self.g1_.y * other.g0_.y - self.g1_.z * other.g0_.z);
}

AntiScalar circle_antiDot_multiVector(Circle self, MultiVector other) {
    return AntiScalar(0.0 - self.g0_.x * other.g8_.x - self.g0_.y * other.g8_.y - self.g0_.z * other.g8_.z + self.g0_.w * other.g6_.w - self.g1_.x * other.g7_.x - self.g1_.y * other.g7_.y - self.g1_.z * other.g7_.z - self.g2_.x * other.g6_.x - self.g2_.y * other.g6_.y - self.g2_.z * other.g6_.z);
}

AntiScalar dipole_antiDot_dipole(Dipole self, Dipole other) {
    return AntiScalar(self.g0_.x * other.g2_.x + self.g0_.y * other.g2_.y + self.g0_.z * other.g2_.z + self.g1_.x * other.g1_.x + self.g1_.y * other.g1_.y + self.g1_.z * other.g1_.z + self.g2_.x * other.g0_.x + self.g2_.y * other.g0_.y + self.g2_.z * other.g0_.z - self.g2_.w * other.g2_.w);
}

AntiScalar dipole_antiDot_flatPoint(Dipole self, FlatPoint other) {
    return AntiScalar(self.g0_.x * other.g0_.x + self.g0_.y * other.g0_.y + self.g0_.z * other.g0_.z - self.g2_.w * other.g0_.w);
}

AntiScalar dipole_antiDot_flector(Dipole self, Flector other) {
    return AntiScalar(self.g0_.x * other.g0_.x + self.g0_.y * other.g0_.y + self.g0_.z * other.g0_.z - self.g2_.w * other.g0_.w);
}

AntiScalar dipole_antiDot_multiVector(Dipole self, MultiVector other) {
    return AntiScalar(self.g0_.x * other.g5_.x + self.g0_.y * other.g5_.y + self.g0_.z * other.g5_.z + self.g1_.x * other.g4_.x + self.g1_.y * other.g4_.y + self.g1_.z * other.g4_.z + self.g2_.x * other.g3_.x + self.g2_.y * other.g3_.y + self.g2_.z * other.g3_.z - self.g2_.w * other.g5_.w);
}

AntiScalar dualNum_antiDot_antiScalar(DualNum self, AntiScalar other) {
    return AntiScalar(self.g0_.y * other.g0_);
}

AntiScalar dualNum_antiDot_dualNum(DualNum self, DualNum other) {
    return AntiScalar(0.0 - self.g0_.x * other.g0_.x + self.g0_.y * other.g0_.y);
}

AntiScalar dualNum_antiDot_motor(DualNum self, Motor other) {
    return AntiScalar(self.g0_.y * other.g0_.w);
}

AntiScalar dualNum_antiDot_multiVector(DualNum self, MultiVector other) {
    return AntiScalar(0.0 - self.g0_.x * other.g0_.x + self.g0_.y * other.g0_.y);
}

AntiScalar dualNum_antiDot_scalar(DualNum self, Scalar other) {
    return AntiScalar(0.0 - self.g0_.x * other.g0_);
}

AntiScalar flatPoint_antiDot_dipole(FlatPoint self, Dipole other) {
    return AntiScalar(self.g0_.x * other.g0_.x + self.g0_.y * other.g0_.y + self.g0_.z * other.g0_.z - self.g0_.w * other.g2_.w);
}

AntiScalar flatPoint_antiDot_flatPoint(FlatPoint self, FlatPoint other) {
    return AntiScalar(0.0 - self.g0_.w * other.g0_.w);
}

AntiScalar flatPoint_antiDot_flector(FlatPoint self, Flector other) {
    return AntiScalar(0.0 - self.g0_.w * other.g0_.w);
}

AntiScalar flatPoint_antiDot_multiVector(FlatPoint self, MultiVector other) {
    return AntiScalar(self.g0_.x * other.g3_.x + self.g0_.y * other.g3_.y + self.g0_.z * other.g3_.z - self.g0_.w * other.g5_.w);
}

AntiScalar flector_antiDot_dipole(Flector self, Dipole other) {
    return AntiScalar(self.g0_.x * other.g0_.x + self.g0_.y * other.g0_.y + self.g0_.z * other.g0_.z - self.g0_.w * other.g2_.w);
}

AntiScalar flector_antiDot_flatPoint(Flector self, FlatPoint other) {
    return AntiScalar(0.0 - self.g0_.w * other.g0_.w);
}

AntiScalar flector_antiDot_flector(Flector self, Flector other) {
    return AntiScalar(0.0 - self.g0_.w * other.g0_.w + self.g1_.x * other.g1_.x + self.g1_.y * other.g1_.y + self.g1_.z * other.g1_.z);
}

AntiScalar flector_antiDot_multiVector(Flector self, MultiVector other) {
    return AntiScalar(self.g0_.x * other.g3_.x + self.g0_.y * other.g3_.y + self.g0_.z * other.g3_.z - self.g0_.w * other.g5_.w + self.g1_.x * other.g9_.x + self.g1_.y * other.g9_.y + self.g1_.z * other.g9_.z - self.g1_.w * other.g10_.x);
}

AntiScalar flector_antiDot_plane(Flector self, Plane other) {
    return AntiScalar(self.g1_.x * other.g0_.x + self.g1_.y * other.g0_.y + self.g1_.z * other.g0_.z);
}

AntiScalar flector_antiDot_sphere(Flector self, Sphere other) {
    return AntiScalar(self.g1_.x * other.g0_.x + self.g1_.y * other.g0_.y + self.g1_.z * other.g0_.z - self.g1_.w * other.g1_.x);
}

AntiScalar line_antiDot_circle(Line self, Circle other) {
    return AntiScalar(0.0 - self.g0_.x * other.g1_.x - self.g0_.y * other.g1_.y - self.g0_.z * other.g1_.z - self.g1_.x * other.g0_.x - self.g1_.y * other.g0_.y - self.g1_.z * other.g0_.z);
}

AntiScalar line_antiDot_line(Line self, Line other) {
    return AntiScalar(0.0 - self.g0_.x * other.g0_.x - self.g0_.y * other.g0_.y - self.g0_.z * other.g0_.z);
}

AntiScalar line_antiDot_motor(Line self, Motor other) {
    return AntiScalar(0.0 - self.g0_.x * other.g0_.x - self.g0_.y * other.g0_.y - self.g0_.z * other.g0_.z);
}

AntiScalar line_antiDot_multiVector(Line self, MultiVector other) {
    return AntiScalar(0.0 - self.g0_.x * other.g7_.x - self.g0_.y * other.g7_.y - self.g0_.z * other.g7_.z - self.g1_.x * other.g6_.x - self.g1_.y * other.g6_.y - self.g1_.z * other.g6_.z);
}

AntiScalar motor_antiDot_antiScalar(Motor self, AntiScalar other) {
    return AntiScalar(self.g0_.w * other.g0_);
}

AntiScalar motor_antiDot_circle(Motor self, Circle other) {
    return AntiScalar(0.0 - self.g0_.x * other.g1_.x - self.g0_.y * other.g1_.y - self.g0_.z * other.g1_.z - self.g1_.x * other.g0_.x - self.g1_.y * other.g0_.y - self.g1_.z * other.g0_.z);
}

AntiScalar motor_antiDot_dualNum(Motor self, DualNum other) {
    return AntiScalar(self.g0_.w * other.g0_.y);
}

AntiScalar motor_antiDot_line(Motor self, Line other) {
    return AntiScalar(0.0 - self.g0_.x * other.g0_.x - self.g0_.y * other.g0_.y - self.g0_.z * other.g0_.z);
}

AntiScalar motor_antiDot_motor(Motor self, Motor other) {
    return AntiScalar(0.0 - self.g0_.x * other.g0_.x - self.g0_.y * other.g0_.y - self.g0_.z * other.g0_.z + self.g0_.w * other.g0_.w);
}

AntiScalar motor_antiDot_multiVector(Motor self, MultiVector other) {
    return AntiScalar(0.0 - self.g0_.x * other.g7_.x - self.g0_.y * other.g7_.y - self.g0_.z * other.g7_.z + self.g0_.w * other.g0_.y - self.g1_.x * other.g6_.x - self.g1_.y * other.g6_.y - self.g1_.z * other.g6_.z);
}

AntiScalar multiVector_antiDot_antiScalar(MultiVector self, AntiScalar other) {
    return AntiScalar(self.g0_.y * other.g0_);
}

AntiScalar multiVector_antiDot_circle(MultiVector self, Circle other) {
    return AntiScalar(0.0 - self.g6_.x * other.g2_.x - self.g6_.y * other.g2_.y - self.g6_.z * other.g2_.z + self.g6_.w * other.g0_.w - self.g7_.x * other.g1_.x - self.g7_.y * other.g1_.y - self.g7_.z * other.g1_.z - self.g8_.x * other.g0_.x - self.g8_.y * other.g0_.y - self.g8_.z * other.g0_.z);
}

AntiScalar multiVector_antiDot_dipole(MultiVector self, Dipole other) {
    return AntiScalar(self.g3_.x * other.g2_.x + self.g3_.y * other.g2_.y + self.g3_.z * other.g2_.z + self.g4_.x * other.g1_.x + self.g4_.y * other.g1_.y + self.g4_.z * other.g1_.z + self.g5_.x * other.g0_.x + self.g5_.y * other.g0_.y + self.g5_.z * other.g0_.z - self.g5_.w * other.g2_.w);
}

AntiScalar multiVector_antiDot_dualNum(MultiVector self, DualNum other) {
    return AntiScalar(0.0 - self.g0_.x * other.g0_.x + self.g0_.y * other.g0_.y);
}

AntiScalar multiVector_antiDot_flatPoint(MultiVector self, FlatPoint other) {
    return AntiScalar(self.g3_.x * other.g0_.x + self.g3_.y * other.g0_.y + self.g3_.z * other.g0_.z - self.g5_.w * other.g0_.w);
}

AntiScalar multiVector_antiDot_flector(MultiVector self, Flector other) {
    return AntiScalar(self.g3_.x * other.g0_.x + self.g3_.y * other.g0_.y + self.g3_.z * other.g0_.z - self.g5_.w * other.g0_.w + self.g9_.x * other.g1_.x + self.g9_.y * other.g1_.y + self.g9_.z * other.g1_.z - self.g10_.x * other.g1_.w);
}

AntiScalar multiVector_antiDot_line(MultiVector self, Line other) {
    return AntiScalar(0.0 - self.g6_.x * other.g1_.x - self.g6_.y * other.g1_.y - self.g6_.z * other.g1_.z - self.g7_.x * other.g0_.x - self.g7_.y * other.g0_.y - self.g7_.z * other.g0_.z);
}

AntiScalar multiVector_antiDot_motor(MultiVector self, Motor other) {
    return AntiScalar(self.g0_.y * other.g0_.w - self.g6_.x * other.g1_.x - self.g6_.y * other.g1_.y - self.g6_.z * other.g1_.z - self.g7_.x * other.g0_.x - self.g7_.y * other.g0_.y - self.g7_.z * other.g0_.z);
}

AntiScalar multiVector_antiDot_multiVector(MultiVector self, MultiVector other) {
    return AntiScalar(0.0 - self.g0_.x * other.g0_.x + self.g0_.y * other.g0_.y - self.g1_.x * other.g1_.x - self.g1_.y * other.g1_.y - self.g1_.z * other.g1_.z + self.g2_.x * other.g2_.y + self.g2_.y * other.g2_.x + self.g3_.x * other.g5_.x + self.g3_.y * other.g5_.y + self.g3_.z * other.g5_.z + self.g4_.x * other.g4_.x + self.g4_.y * other.g4_.y + self.g4_.z * other.g4_.z + self.g5_.x * other.g3_.x + self.g5_.y * other.g3_.y + self.g5_.z * other.g3_.z - self.g5_.w * other.g5_.w - self.g6_.x * other.g8_.x - self.g6_.y * other.g8_.y - self.g6_.z * other.g8_.z + self.g6_.w * other.g6_.w - self.g7_.x * other.g7_.x - self.g7_.y * other.g7_.y - self.g7_.z * other.g7_.z - self.g8_.x * other.g6_.x - self.g8_.y * other.g6_.y - self.g8_.z * other.g6_.z + self.g9_.x * other.g9_.x + self.g9_.y * other.g9_.y + self.g9_.z * other.g9_.z - self.g10_.x * other.g10_.y - self.g10_.y * other.g10_.x);
}

AntiScalar multiVector_antiDot_plane(MultiVector self, Plane other) {
    return AntiScalar(self.g9_.x * other.g0_.x + self.g9_.y * other.g0_.y + self.g9_.z * other.g0_.z - self.g10_.x * other.g0_.w);
}

AntiScalar multiVector_antiDot_roundPoint(MultiVector self, RoundPoint other) {
    return AntiScalar(0.0 - self.g1_.x * other.g0_.x - self.g1_.y * other.g0_.y - self.g1_.z * other.g0_.z + self.g2_.x * other.g1_.y + self.g2_.y * other.g1_.x);
}

AntiScalar multiVector_antiDot_scalar(MultiVector self, Scalar other) {
    return AntiScalar(0.0 - self.g0_.x * other.g0_);
}

AntiScalar multiVector_antiDot_sphere(MultiVector self, Sphere other) {
    return AntiScalar(self.g9_.x * other.g0_.x + self.g9_.y * other.g0_.y + self.g9_.z * other.g0_.z - self.g10_.x * other.g1_.y - self.g10_.y * other.g1_.x);
}

AntiScalar plane_antiDot_flector(Plane self, Flector other) {
    return AntiScalar(self.g0_.x * other.g1_.x + self.g0_.y * other.g1_.y + self.g0_.z * other.g1_.z);
}

AntiScalar plane_antiDot_multiVector(Plane self, MultiVector other) {
    return AntiScalar(self.g0_.x * other.g9_.x + self.g0_.y * other.g9_.y + self.g0_.z * other.g9_.z - self.g0_.w * other.g10_.x);
}

AntiScalar plane_antiDot_plane(Plane self, Plane other) {
    return AntiScalar(self.g0_.x * other.g0_.x + self.g0_.y * other.g0_.y + self.g0_.z * other.g0_.z);
}

AntiScalar plane_antiDot_sphere(Plane self, Sphere other) {
    return AntiScalar(self.g0_.x * other.g0_.x + self.g0_.y * other.g0_.y + self.g0_.z * other.g0_.z - self.g0_.w * other.g1_.x);
}

AntiScalar roundPoint_antiDot_multiVector(RoundPoint self, MultiVector other) {
    return AntiScalar(0.0 - self.g0_.x * other.g1_.x - self.g0_.y * other.g1_.y - self.g0_.z * other.g1_.z + self.g1_.x * other.g2_.y + self.g1_.y * other.g2_.x);
}

AntiScalar roundPoint_antiDot_roundPoint(RoundPoint self, RoundPoint other) {
    return AntiScalar(0.0 - self.g0_.x * other.g0_.x - self.g0_.y * other.g0_.y - self.g0_.z * other.g0_.z + self.g1_.x * other.g1_.y + self.g1_.y * other.g1_.x);
}

AntiScalar scalar_antiDot_dualNum(Scalar self, DualNum other) {
    return AntiScalar(0.0 - self.g0_ * other.g0_.x);
}

AntiScalar scalar_antiDot_multiVector(Scalar self, MultiVector other) {
    return AntiScalar(0.0 - self.g0_ * other.g0_.x);
}

AntiScalar scalar_antiDot_scalar(Scalar self, Scalar other) {
    return AntiScalar(0.0 - self.g0_ * other.g0_);
}

AntiScalar sphere_antiDot_flector(Sphere self, Flector other) {
    return AntiScalar(self.g0_.x * other.g1_.x + self.g0_.y * other.g1_.y + self.g0_.z * other.g1_.z - self.g1_.x * other.g1_.w);
}

AntiScalar sphere_antiDot_multiVector(Sphere self, MultiVector other) {
    return AntiScalar(self.g0_.x * other.g9_.x + self.g0_.y * other.g9_.y + self.g0_.z * other.g9_.z - self.g1_.x * other.g10_.y - self.g1_.y * other.g10_.x);
}

AntiScalar sphere_antiDot_plane(Sphere self, Plane other) {
    return AntiScalar(self.g0_.x * other.g0_.x + self.g0_.y * other.g0_.y + self.g0_.z * other.g0_.z - self.g1_.x * other.g0_.w);
}

AntiScalar sphere_antiDot_sphere(Sphere self, Sphere other) {
    return AntiScalar(self.g0_.x * other.g0_.x + self.g0_.y * other.g0_.y + self.g0_.z * other.g0_.z - self.g1_.x * other.g1_.y - self.g1_.y * other.g1_.x);
}

Scalar antiScalar_dot_antiScalar(AntiScalar self, AntiScalar other) {
    return Scalar(0.0 - self.g0_ * other.g0_);
}

Scalar antiScalar_dot_dualNum(AntiScalar self, DualNum other) {
    return Scalar(0.0 - self.g0_ * other.g0_.y);
}

Scalar antiScalar_dot_motor(AntiScalar self, Motor other) {
    return Scalar(0.0 - self.g0_ * other.g0_.w);
}

Scalar antiScalar_dot_multiVector(AntiScalar self, MultiVector other) {
    return Scalar(0.0 - self.g0_ * other.g0_.y);
}

Scalar circle_dot_circle(Circle self, Circle other) {
    return Scalar(self.g0_.x * other.g2_.x + self.g0_.y * other.g2_.y + self.g0_.z * other.g2_.z - self.g0_.w * other.g0_.w + self.g1_.x * other.g1_.x + self.g1_.y * other.g1_.y + self.g1_.z * other.g1_.z + self.g2_.x * other.g0_.x + self.g2_.y * other.g0_.y + self.g2_.z * other.g0_.z);
}

Scalar circle_dot_line(Circle self, Line other) {
    return Scalar(self.g0_.x * other.g1_.x + self.g0_.y * other.g1_.y + self.g0_.z * other.g1_.z + self.g1_.x * other.g0_.x + self.g1_.y * other.g0_.y + self.g1_.z * other.g0_.z);
}

Scalar circle_dot_motor(Circle self, Motor other) {
    return Scalar(self.g0_.x * other.g1_.x + self.g0_.y * other.g1_.y + self.g0_.z * other.g1_.z + self.g1_.x * other.g0_.x + self.g1_.y * other.g0_.y + self.g1_.z * other.g0_.z);
}

Scalar circle_dot_multiVector(Circle self, MultiVector other) {
    return Scalar(self.g0_.x * other.g8_.x + self.g0_.y * other.g8_.y + self.g0_.z * other.g8_.z - self.g0_.w * other.g6_.w + self.g1_.x * other.g7_.x + self.g1_.y * other.g7_.y + self.g1_.z * other.g7_.z + self.g2_.x * other.g6_.x + self.g2_.y * other.g6_.y + self.g2_.z * other.g6_.z);
}

Scalar dipole_dot_dipole(Dipole self, Dipole other) {
    return Scalar(0.0 - self.g0_.x * other.g2_.x - self.g0_.y * other.g2_.y - self.g0_.z * other.g2_.z - self.g1_.x * other.g1_.x - self.g1_.y * other.g1_.y - self.g1_.z * other.g1_.z - self.g2_.x * other.g0_.x - self.g2_.y * other.g0_.y - self.g2_.z * other.g0_.z + self.g2_.w * other.g2_.w);
}

Scalar dipole_dot_flatPoint(Dipole self, FlatPoint other) {
    return Scalar(0.0 - self.g0_.x * other.g0_.x - self.g0_.y * other.g0_.y - self.g0_.z * other.g0_.z + self.g2_.w * other.g0_.w);
}

Scalar dipole_dot_flector(Dipole self, Flector other) {
    return Scalar(0.0 - self.g0_.x * other.g0_.x - self.g0_.y * other.g0_.y - self.g0_.z * other.g0_.z + self.g2_.w * other.g0_.w);
}

Scalar dipole_dot_multiVector(Dipole self, MultiVector other) {
    return Scalar(0.0 - self.g0_.x * other.g5_.x - self.g0_.y * other.g5_.y - self.g0_.z * other.g5_.z - self.g1_.x * other.g4_.x - self.g1_.y * other.g4_.y - self.g1_.z * other.g4_.z - self.g2_.x * other.g3_.x - self.g2_.y * other.g3_.y - self.g2_.z * other.g3_.z + self.g2_.w * other.g5_.w);
}

Scalar dualNum_dot_antiScalar(DualNum self, AntiScalar other) {
    return Scalar(0.0 - self.g0_.y * other.g0_);
}

Scalar dualNum_dot_dualNum(DualNum self, DualNum other) {
    return Scalar(self.g0_.x * other.g0_.x - self.g0_.y * other.g0_.y);
}

Scalar dualNum_dot_motor(DualNum self, Motor other) {
    return Scalar(0.0 - self.g0_.y * other.g0_.w);
}

Scalar dualNum_dot_multiVector(DualNum self, MultiVector other) {
    return Scalar(self.g0_.x * other.g0_.x - self.g0_.y * other.g0_.y);
}

Scalar dualNum_dot_scalar(DualNum self, Scalar other) {
    return Scalar(self.g0_.x * other.g0_);
}

Scalar flatPoint_dot_dipole(FlatPoint self, Dipole other) {
    return Scalar(0.0 - self.g0_.x * other.g0_.x - self.g0_.y * other.g0_.y - self.g0_.z * other.g0_.z + self.g0_.w * other.g2_.w);
}

Scalar flatPoint_dot_flatPoint(FlatPoint self, FlatPoint other) {
    return Scalar(self.g0_.w * other.g0_.w);
}

Scalar flatPoint_dot_flector(FlatPoint self, Flector other) {
    return Scalar(self.g0_.w * other.g0_.w);
}

Scalar flatPoint_dot_multiVector(FlatPoint self, MultiVector other) {
    return Scalar(0.0 - self.g0_.x * other.g3_.x - self.g0_.y * other.g3_.y - self.g0_.z * other.g3_.z + self.g0_.w * other.g5_.w);
}

Scalar flector_dot_dipole(Flector self, Dipole other) {
    return Scalar(0.0 - self.g0_.x * other.g0_.x - self.g0_.y * other.g0_.y - self.g0_.z * other.g0_.z + self.g0_.w * other.g2_.w);
}

Scalar flector_dot_flatPoint(Flector self, FlatPoint other) {
    return Scalar(self.g0_.w * other.g0_.w);
}

Scalar flector_dot_flector(Flector self, Flector other) {
    return Scalar(self.g0_.w * other.g0_.w - self.g1_.x * other.g1_.x - self.g1_.y * other.g1_.y - self.g1_.z * other.g1_.z);
}

Scalar flector_dot_multiVector(Flector self, MultiVector other) {
    return Scalar(0.0 - self.g0_.x * other.g3_.x - self.g0_.y * other.g3_.y - self.g0_.z * other.g3_.z + self.g0_.w * other.g5_.w - self.g1_.x * other.g9_.x - self.g1_.y * other.g9_.y - self.g1_.z * other.g9_.z + self.g1_.w * other.g10_.x);
}

Scalar flector_dot_plane(Flector self, Plane other) {
    return Scalar(0.0 - self.g1_.x * other.g0_.x - self.g1_.y * other.g0_.y - self.g1_.z * other.g0_.z);
}

Scalar flector_dot_sphere(Flector self, Sphere other) {
    return Scalar(0.0 - self.g1_.x * other.g0_.x - self.g1_.y * other.g0_.y - self.g1_.z * other.g0_.z + self.g1_.w * other.g1_.x);
}

Scalar line_dot_circle(Line self, Circle other) {
    return Scalar(self.g0_.x * other.g1_.x + self.g0_.y * other.g1_.y + self.g0_.z * other.g1_.z + self.g1_.x * other.g0_.x + self.g1_.y * other.g0_.y + self.g1_.z * other.g0_.z);
}

Scalar line_dot_line(Line self, Line other) {
    return Scalar(self.g0_.x * other.g0_.x + self.g0_.y * other.g0_.y + self.g0_.z * other.g0_.z);
}

Scalar line_dot_motor(Line self, Motor other) {
    return Scalar(self.g0_.x * other.g0_.x + self.g0_.y * other.g0_.y + self.g0_.z * other.g0_.z);
}

Scalar line_dot_multiVector(Line self, MultiVector other) {
    return Scalar(self.g0_.x * other.g7_.x + self.g0_.y * other.g7_.y + self.g0_.z * other.g7_.z + self.g1_.x * other.g6_.x + self.g1_.y * other.g6_.y + self.g1_.z * other.g6_.z);
}

Scalar motor_dot_antiScalar(Motor self, AntiScalar other) {
    return Scalar(0.0 - self.g0_.w * other.g0_);
}

Scalar motor_dot_circle(Motor self, Circle other) {
    return Scalar(self.g0_.x * other.g1_.x + self.g0_.y * other.g1_.y + self.g0_.z * other.g1_.z + self.g1_.x * other.g0_.x + self.g1_.y * other.g0_.y + self.g1_.z * other.g0_.z);
}

Scalar motor_dot_dualNum(Motor self, DualNum other) {
    return Scalar(0.0 - self.g0_.w * other.g0_.y);
}

Scalar motor_dot_line(Motor self, Line other) {
    return Scalar(self.g0_.x * other.g0_.x + self.g0_.y * other.g0_.y + self.g0_.z * other.g0_.z);
}

Scalar motor_dot_motor(Motor self, Motor other) {
    return Scalar(self.g0_.x * other.g0_.x + self.g0_.y * other.g0_.y + self.g0_.z * other.g0_.z - self.g0_.w * other.g0_.w);
}

Scalar motor_dot_multiVector(Motor self, MultiVector other) {
    return Scalar(self.g0_.x * other.g7_.x + self.g0_.y * other.g7_.y + self.g0_.z * other.g7_.z - self.g0_.w * other.g0_.y + self.g1_.x * other.g6_.x + self.g1_.y * other.g6_.y + self.g1_.z * other.g6_.z);
}

Scalar multiVector_dot_antiScalar(MultiVector self, AntiScalar other) {
    return Scalar(0.0 - self.g0_.y * other.g0_);
}

Scalar multiVector_dot_circle(MultiVector self, Circle other) {
    return Scalar(self.g6_.x * other.g2_.x + self.g6_.y * other.g2_.y + self.g6_.z * other.g2_.z - self.g6_.w * other.g0_.w + self.g7_.x * other.g1_.x + self.g7_.y * other.g1_.y + self.g7_.z * other.g1_.z + self.g8_.x * other.g0_.x + self.g8_.y * other.g0_.y + self.g8_.z * other.g0_.z);
}

Scalar multiVector_dot_dipole(MultiVector self, Dipole other) {
    return Scalar(0.0 - self.g3_.x * other.g2_.x - self.g3_.y * other.g2_.y - self.g3_.z * other.g2_.z - self.g4_.x * other.g1_.x - self.g4_.y * other.g1_.y - self.g4_.z * other.g1_.z - self.g5_.x * other.g0_.x - self.g5_.y * other.g0_.y - self.g5_.z * other.g0_.z + self.g5_.w * other.g2_.w);
}

Scalar multiVector_dot_dualNum(MultiVector self, DualNum other) {
    return Scalar(self.g0_.x * other.g0_.x - self.g0_.y * other.g0_.y);
}

Scalar multiVector_dot_flatPoint(MultiVector self, FlatPoint other) {
    return Scalar(0.0 - self.g3_.x * other.g0_.x - self.g3_.y * other.g0_.y - self.g3_.z * other.g0_.z + self.g5_.w * other.g0_.w);
}

Scalar multiVector_dot_flector(MultiVector self, Flector other) {
    return Scalar(0.0 - self.g3_.x * other.g0_.x - self.g3_.y * other.g0_.y - self.g3_.z * other.g0_.z + self.g5_.w * other.g0_.w - self.g9_.x * other.g1_.x - self.g9_.y * other.g1_.y - self.g9_.z * other.g1_.z + self.g10_.x * other.g1_.w);
}

Scalar multiVector_dot_line(MultiVector self, Line other) {
    return Scalar(self.g6_.x * other.g1_.x + self.g6_.y * other.g1_.y + self.g6_.z * other.g1_.z + self.g7_.x * other.g0_.x + self.g7_.y * other.g0_.y + self.g7_.z * other.g0_.z);
}

Scalar multiVector_dot_motor(MultiVector self, Motor other) {
    return Scalar(0.0 - self.g0_.y * other.g0_.w + self.g6_.x * other.g1_.x + self.g6_.y * other.g1_.y + self.g6_.z * other.g1_.z + self.g7_.x * other.g0_.x + self.g7_.y * other.g0_.y + self.g7_.z * other.g0_.z);
}

Scalar multiVector_dot_multiVector(MultiVector self, MultiVector other) {
    return Scalar(self.g0_.x * other.g0_.x - self.g0_.y * other.g0_.y + self.g1_.x * other.g1_.x + self.g1_.y * other.g1_.y + self.g1_.z * other.g1_.z - self.g2_.x * other.g2_.y - self.g2_.y * other.g2_.x - self.g3_.x * other.g5_.x - self.g3_.y * other.g5_.y - self.g3_.z * other.g5_.z - self.g4_.x * other.g4_.x - self.g4_.y * other.g4_.y - self.g4_.z * other.g4_.z - self.g5_.x * other.g3_.x - self.g5_.y * other.g3_.y - self.g5_.z * other.g3_.z + self.g5_.w * other.g5_.w + self.g6_.x * other.g8_.x + self.g6_.y * other.g8_.y + self.g6_.z * other.g8_.z - self.g6_.w * other.g6_.w + self.g7_.x * other.g7_.x + self.g7_.y * other.g7_.y + self.g7_.z * other.g7_.z + self.g8_.x * other.g6_.x + self.g8_.y * other.g6_.y + self.g8_.z * other.g6_.z - self.g9_.x * other.g9_.x - self.g9_.y * other.g9_.y - self.g9_.z * other.g9_.z + self.g10_.x * other.g10_.y + self.g10_.y * other.g10_.x);
}

Scalar multiVector_dot_plane(MultiVector self, Plane other) {
    return Scalar(0.0 - self.g9_.x * other.g0_.x - self.g9_.y * other.g0_.y - self.g9_.z * other.g0_.z + self.g10_.x * other.g0_.w);
}

Scalar multiVector_dot_roundPoint(MultiVector self, RoundPoint other) {
    return Scalar(self.g1_.x * other.g0_.x + self.g1_.y * other.g0_.y + self.g1_.z * other.g0_.z - self.g2_.x * other.g1_.y - self.g2_.y * other.g1_.x);
}

Scalar multiVector_dot_scalar(MultiVector self, Scalar other) {
    return Scalar(self.g0_.x * other.g0_);
}

Scalar multiVector_dot_sphere(MultiVector self, Sphere other) {
    return Scalar(0.0 - self.g9_.x * other.g0_.x - self.g9_.y * other.g0_.y - self.g9_.z * other.g0_.z + self.g10_.x * other.g1_.y + self.g10_.y * other.g1_.x);
}

Scalar plane_dot_flector(Plane self, Flector other) {
    return Scalar(0.0 - self.g0_.x * other.g1_.x - self.g0_.y * other.g1_.y - self.g0_.z * other.g1_.z);
}

Scalar plane_dot_multiVector(Plane self, MultiVector other) {
    return Scalar(0.0 - self.g0_.x * other.g9_.x - self.g0_.y * other.g9_.y - self.g0_.z * other.g9_.z + self.g0_.w * other.g10_.x);
}

Scalar plane_dot_plane(Plane self, Plane other) {
    return Scalar(0.0 - self.g0_.x * other.g0_.x - self.g0_.y * other.g0_.y - self.g0_.z * other.g0_.z);
}

Scalar plane_dot_sphere(Plane self, Sphere other) {
    return Scalar(0.0 - self.g0_.x * other.g0_.x - self.g0_.y * other.g0_.y - self.g0_.z * other.g0_.z + self.g0_.w * other.g1_.x);
}

Scalar roundPoint_dot_multiVector(RoundPoint self, MultiVector other) {
    return Scalar(self.g0_.x * other.g1_.x + self.g0_.y * other.g1_.y + self.g0_.z * other.g1_.z - self.g1_.x * other.g2_.y - self.g1_.y * other.g2_.x);
}

Scalar roundPoint_dot_roundPoint(RoundPoint self, RoundPoint other) {
    return Scalar(self.g0_.x * other.g0_.x + self.g0_.y * other.g0_.y + self.g0_.z * other.g0_.z - self.g1_.x * other.g1_.y - self.g1_.y * other.g1_.x);
}

Scalar scalar_dot_dualNum(Scalar self, DualNum other) {
    return Scalar(self.g0_ * other.g0_.x);
}

Scalar scalar_dot_multiVector(Scalar self, MultiVector other) {
    return Scalar(self.g0_ * other.g0_.x);
}

Scalar scalar_dot_scalar(Scalar self, Scalar other) {
    return Scalar(self.g0_ * other.g0_);
}

Scalar sphere_dot_flector(Sphere self, Flector other) {
    return Scalar(0.0 - self.g0_.x * other.g1_.x - self.g0_.y * other.g1_.y - self.g0_.z * other.g1_.z + self.g1_.x * other.g1_.w);
}

Scalar sphere_dot_multiVector(Sphere self, MultiVector other) {
    return Scalar(0.0 - self.g0_.x * other.g9_.x - self.g0_.y * other.g9_.y - self.g0_.z * other.g9_.z + self.g1_.x * other.g10_.y + self.g1_.y * other.g10_.x);
}

Scalar sphere_dot_plane(Sphere self, Plane other) {
    return Scalar(0.0 - self.g0_.x * other.g0_.x - self.g0_.y * other.g0_.y - self.g0_.z * other.g0_.z + self.g1_.x * other.g0_.w);
}

Scalar sphere_dot_sphere(Sphere self, Sphere other) {
    return Scalar(0.0 - self.g0_.x * other.g0_.x - self.g0_.y * other.g0_.y - self.g0_.z * other.g0_.z + self.g1_.x * other.g1_.y + self.g1_.y * other.g1_.x);
}

Circle circle_flatBulk(Circle self) {
    return Circle(vec4(0.0), vec3(0.0), self.g2_);
}

Dipole dipole_flatBulk(Dipole self) {
    return Dipole(vec3(0.0), vec3(0.0), self.g2_ * vec4(1.0, 1.0, 1.0, 0.0));
}

FlatPoint flatPoint_flatBulk(FlatPoint self) {
    return FlatPoint(self.g0_ * vec4(1.0, 1.0, 1.0, 0.0));
}

Flector flector_flatBulk(Flector self) {
    return Flector(self.g0_ * vec4(1.0, 1.0, 1.0, 0.0), self.g1_ * vec4(0.0, 0.0, 0.0, 1.0));
}

Line line_flatBulk(Line self) {
    return Line(vec3(0.0), self.g1_);
}

Motor motor_flatBulk(Motor self) {
    return Motor(vec4(0.0), self.g1_);
}

MultiVector multiVector_flatBulk(MultiVector self) {
    return MultiVector(vec2(0.0), vec3(0.0), self.g2_ * vec2(0.0, 1.0), vec3(0.0), vec3(0.0), self.g5_ * vec4(1.0, 1.0, 1.0, 0.0), vec4(0.0), vec3(0.0), self.g8_, vec3(0.0), self.g10_ * vec2(0.0, 1.0));
}

Plane plane_flatBulk(Plane self) {
    return Plane(self.g0_ * vec4(0.0, 0.0, 0.0, 1.0));
}

RoundPoint roundPoint_flatBulk(RoundPoint self) {
    return RoundPoint(vec3(0.0), self.g1_ * vec2(0.0, 1.0));
}

Sphere sphere_flatBulk(Sphere self) {
    return Sphere(vec3(0.0), self.g1_ * vec2(0.0, 1.0));
}

AntiScalar antiScalar_flatWeight(AntiScalar self) {
    return self;
}

Circle circle_flatWeight(Circle self) {
    return Circle(vec4(0.0), self.g1_, vec3(0.0));
}

Dipole dipole_flatWeight(Dipole self) {
    return Dipole(vec3(0.0), vec3(0.0), self.g2_ * vec4(0.0, 0.0, 0.0, 1.0));
}

DualNum dualNum_flatWeight(DualNum self) {
    return DualNum(self.g0_ * vec2(0.0, 1.0));
}

FlatPoint flatPoint_flatWeight(FlatPoint self) {
    return FlatPoint(self.g0_ * vec4(0.0, 0.0, 0.0, 1.0));
}

Flector flector_flatWeight(Flector self) {
    return Flector(self.g0_ * vec4(0.0, 0.0, 0.0, 1.0), self.g1_ * vec4(1.0, 1.0, 1.0, 0.0));
}

Line line_flatWeight(Line self) {
    return Line(self.g0_, vec3(0.0));
}

Motor motor_flatWeight(Motor self) {
    return Motor(self.g0_, vec3(0.0));
}

MultiVector multiVector_flatWeight(MultiVector self) {
    return MultiVector(self.g0_ * vec2(0.0, 1.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(0.0), self.g5_ * vec4(0.0, 0.0, 0.0, 1.0), vec4(0.0), self.g7_, vec3(0.0), self.g9_, vec2(0.0));
}

Plane plane_flatWeight(Plane self) {
    return Plane(self.g0_ * vec4(1.0, 1.0, 1.0, 0.0));
}

Sphere sphere_flatWeight(Sphere self) {
    return Sphere(self.g0_, vec2(0.0));
}

Circle circle_roundBulk(Circle self) {
    return Circle(self.g0_ * vec4(0.0, 0.0, 0.0, 1.0), vec3(0.0), vec3(0.0));
}

Dipole dipole_roundBulk(Dipole self) {
    return Dipole(vec3(0.0), self.g1_, vec4(0.0));
}

DualNum dualNum_roundBulk(DualNum self) {
    return DualNum(self.g0_ * vec2(1.0, 0.0));
}

MultiVector multiVector_roundBulk(MultiVector self) {
    return MultiVector(self.g0_ * vec2(1.0, 0.0), self.g1_, vec2(0.0), vec3(0.0), self.g4_, vec4(0.0), self.g6_ * vec4(0.0, 0.0, 0.0, 1.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

RoundPoint roundPoint_roundBulk(RoundPoint self) {
    return RoundPoint(self.g0_, vec2(0.0));
}

Scalar scalar_roundBulk(Scalar self) {
    return self;
}

Circle circle_roundWeight(Circle self) {
    return Circle(self.g0_ * vec4(1.0, 1.0, 1.0, 0.0), vec3(0.0), vec3(0.0));
}

Dipole dipole_roundWeight(Dipole self) {
    return Dipole(self.g0_, vec3(0.0), vec4(0.0));
}

MultiVector multiVector_roundWeight(MultiVector self) {
    return MultiVector(vec2(0.0), vec3(0.0), self.g2_ * vec2(1.0, 0.0), self.g3_, vec3(0.0), vec4(0.0), self.g6_ * vec4(1.0, 1.0, 1.0, 0.0), vec3(0.0), vec3(0.0), vec3(0.0), self.g10_ * vec2(1.0, 0.0));
}

RoundPoint roundPoint_roundWeight(RoundPoint self) {
    return RoundPoint(vec3(0.0), self.g1_ * vec2(1.0, 0.0));
}

Sphere sphere_roundWeight(Sphere self) {
    return Sphere(vec3(0.0), self.g1_ * vec2(1.0, 0.0));
}

Scalar antiScalar_antiDual(AntiScalar self) {
    return Scalar(self.g0_);
}

Dipole circle_antiDual(Circle self) {
    return Dipole(vec3(-self.g0_.x, self.g0_.y, self.g0_.z), self.g1_ * vec3(-1.0), vec4(-self.g2_.x, -self.g2_.y, -self.g2_.z, self.g0_.w));
}

Circle dipole_antiDual(Dipole self) {
    return Circle(vec4(self.g0_.x, self.g0_.y, self.g0_.z, -self.g2_.w), self.g1_, vec3(self.g2_.x, self.g2_.y, self.g2_.z));
}

DualNum dualNum_antiDual(DualNum self) {
    return DualNum(self.g0_.yx * vec2(1.0, -1.0));
}

Circle flatPoint_antiDual(FlatPoint self) {
    return Circle(vec4(0.0, 0.0, 0.0, -self.g0_.w), vec3(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z));
}

MultiVector flector_antiDual(Flector self) {
    return MultiVector(vec2(0.0), vec3(self.g1_.x, self.g1_.y, self.g1_.z), vec2(0.0, -self.g1_.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0, 0.0, 0.0, -self.g0_.w), vec3(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z), vec3(0.0), vec2(0.0));
}

Dipole line_antiDual(Line self) {
    return Dipole(vec3(0.0), self.g0_ * vec3(-1.0), vec4(-self.g1_.x, -self.g1_.y, -self.g1_.z, 0.0));
}

MultiVector motor_antiDual(Motor self) {
    return MultiVector(vec2(self.g0_.w, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(-self.g0_.x, self.g0_.y, self.g0_.z), vec4(-self.g1_.x, -self.g1_.y, -self.g1_.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector multiVector_antiDual(MultiVector self) {
    return MultiVector(self.g0_.yx * vec2(1.0, -1.0), self.g9_, self.g10_ * vec2(-1.0), vec3(-self.g6_.x, self.g6_.y, self.g6_.z), self.g7_ * vec3(-1.0), vec4(-self.g8_.x, -self.g8_.y, -self.g8_.z, self.g6_.w), vec4(self.g3_.x, self.g3_.y, self.g3_.z, -self.g5_.w), self.g4_, vec3(self.g5_.x, self.g5_.y, self.g5_.z), self.g1_ * vec3(-1.0), self.g2_);
}

RoundPoint plane_antiDual(Plane self) {
    return RoundPoint(vec3(self.g0_.x, self.g0_.y, self.g0_.z), vec2(0.0, -self.g0_.w));
}

Sphere roundPoint_antiDual(RoundPoint self) {
    return Sphere(self.g0_ * vec3(-1.0), self.g1_);
}

AntiScalar scalar_antiDual(Scalar self) {
    return AntiScalar(-self.g0_);
}

RoundPoint sphere_antiDual(Sphere self) {
    return RoundPoint(self.g0_, self.g1_ * vec2(-1.0));
}

AntiScalar antiScalar_antiReversal(AntiScalar self) {
    return self;
}

Circle circle_antiReversal(Circle self) {
    return Circle(self.g0_ * vec4(-1.0), self.g1_ * vec3(-1.0), self.g2_ * vec3(-1.0));
}

Dipole dipole_antiReversal(Dipole self) {
    return Dipole(self.g0_ * vec3(-1.0), self.g1_ * vec3(-1.0), self.g2_ * vec4(-1.0));
}

DualNum dualNum_antiReversal(DualNum self) {
    return self;
}

FlatPoint flatPoint_antiReversal(FlatPoint self) {
    return FlatPoint(self.g0_ * vec4(-1.0));
}

Flector flector_antiReversal(Flector self) {
    return Flector(self.g0_ * vec4(-1.0), self.g1_);
}

Line line_antiReversal(Line self) {
    return Line(self.g0_ * vec3(-1.0), self.g1_ * vec3(-1.0));
}

Motor motor_antiReversal(Motor self) {
    return Motor(self.g0_ * vec4(-1.0, -1.0, -1.0, 1.0), self.g1_ * vec3(-1.0));
}

MultiVector multiVector_antiReversal(MultiVector self) {
    return MultiVector(self.g0_, self.g1_, self.g2_, self.g3_ * vec3(-1.0), self.g4_ * vec3(-1.0), self.g5_ * vec4(-1.0), self.g6_ * vec4(-1.0), self.g7_ * vec3(-1.0), self.g8_ * vec3(-1.0), self.g9_, self.g10_);
}

Plane plane_antiReversal(Plane self) {
    return self;
}

RoundPoint roundPoint_antiReversal(RoundPoint self) {
    return self;
}

Scalar scalar_antiReversal(Scalar self) {
    return self;
}

Sphere sphere_antiReversal(Sphere self) {
    return self;
}

AntiScalar antiScalar_automorphism(AntiScalar self) {
    return AntiScalar(-self.g0_);
}

Circle circle_automorphism(Circle self) {
    return Circle(self.g0_ * vec4(-1.0), self.g1_ * vec3(-1.0), self.g2_ * vec3(-1.0));
}

Dipole dipole_automorphism(Dipole self) {
    return self;
}

DualNum dualNum_automorphism(DualNum self) {
    return DualNum(self.g0_ * vec2(1.0, -1.0));
}

FlatPoint flatPoint_automorphism(FlatPoint self) {
    return self;
}

Flector flector_automorphism(Flector self) {
    return self;
}

Line line_automorphism(Line self) {
    return Line(self.g0_ * vec3(-1.0), self.g1_ * vec3(-1.0));
}

Motor motor_automorphism(Motor self) {
    return Motor(self.g0_ * vec4(-1.0), self.g1_ * vec3(-1.0));
}

MultiVector multiVector_automorphism(MultiVector self) {
    return MultiVector(self.g0_ * vec2(1.0, -1.0), self.g1_ * vec3(-1.0), self.g2_ * vec2(-1.0), self.g3_, self.g4_, self.g5_, self.g6_ * vec4(-1.0), self.g7_ * vec3(-1.0), self.g8_ * vec3(-1.0), self.g9_, self.g10_);
}

Plane plane_automorphism(Plane self) {
    return self;
}

RoundPoint roundPoint_automorphism(RoundPoint self) {
    return RoundPoint(self.g0_ * vec3(-1.0), self.g1_ * vec2(-1.0));
}

Scalar scalar_automorphism(Scalar self) {
    return self;
}

Sphere sphere_automorphism(Sphere self) {
    return self;
}

Scalar antiScalar_complement(AntiScalar self) {
    return Scalar(self.g0_);
}

Dipole circle_complement(Circle self) {
    return Dipole(self.g2_ * vec3(-1.0), self.g1_ * vec3(-1.0), self.g0_ * vec4(-1.0));
}

Circle dipole_complement(Dipole self) {
    return Circle(self.g2_ * vec4(-1.0), self.g1_ * vec3(-1.0), self.g0_ * vec3(-1.0));
}

DualNum dualNum_complement(DualNum self) {
    return DualNum(self.g0_.yx);
}

Circle flatPoint_complement(FlatPoint self) {
    return Circle(self.g0_ * vec4(-1.0), vec3(0.0), vec3(0.0));
}

MultiVector flector_complement(Flector self) {
    return MultiVector(vec2(0.0), vec3(self.g1_.x, self.g1_.y, self.g1_.z), vec2(self.g1_.w, 0.0), vec3(0.0), vec3(0.0), vec4(0.0), self.g0_ * vec4(-1.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

Dipole line_complement(Line self) {
    return Dipole(self.g1_ * vec3(-1.0), self.g0_ * vec3(-1.0), vec4(0.0));
}

MultiVector motor_complement(Motor self) {
    return MultiVector(vec2(self.g0_.w, 0.0), vec3(0.0), vec2(0.0), self.g1_ * vec3(-1.0), vec3(-self.g0_.x, self.g0_.y, self.g0_.z), vec4(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector multiVector_complement(MultiVector self) {
    return MultiVector(self.g0_.yx, self.g9_, self.g10_.yx, self.g8_ * vec3(-1.0), self.g7_ * vec3(-1.0), self.g6_ * vec4(-1.0), self.g5_ * vec4(-1.0), self.g4_ * vec3(-1.0), self.g3_ * vec3(-1.0), self.g1_, self.g2_.yx);
}

RoundPoint plane_complement(Plane self) {
    return RoundPoint(vec3(self.g0_.x, self.g0_.y, self.g0_.z), vec2(self.g0_.w, 0.0));
}

Sphere roundPoint_complement(RoundPoint self) {
    return Sphere(self.g0_, self.g1_.yx);
}

AntiScalar scalar_complement(Scalar self) {
    return AntiScalar(self.g0_);
}

RoundPoint sphere_complement(Sphere self) {
    return RoundPoint(self.g0_, self.g1_.yx);
}

AntiScalar antiScalar_conformalConjugate(AntiScalar self) {
    return AntiScalar(-self.g0_);
}

Circle circle_conformalConjugate(Circle self) {
    return Circle(self.g0_, self.g1_ * vec3(-1.0), self.g2_ * vec3(-1.0));
}

Dipole dipole_conformalConjugate(Dipole self) {
    return Dipole(self.g0_, self.g1_, self.g2_ * vec4(-1.0));
}

DualNum dualNum_conformalConjugate(DualNum self) {
    return DualNum(self.g0_ * vec2(1.0, -1.0));
}

FlatPoint flatPoint_conformalConjugate(FlatPoint self) {
    return FlatPoint(self.g0_ * vec4(-1.0));
}

Flector flector_conformalConjugate(Flector self) {
    return Flector(self.g0_ * vec4(-1.0), self.g1_ * vec4(-1.0));
}

Line line_conformalConjugate(Line self) {
    return Line(self.g0_ * vec3(-1.0), self.g1_ * vec3(-1.0));
}

Motor motor_conformalConjugate(Motor self) {
    return Motor(self.g0_ * vec4(-1.0), self.g1_ * vec3(-1.0));
}

MultiVector multiVector_conformalConjugate(MultiVector self) {
    return MultiVector(self.g0_ * vec2(1.0, -1.0), self.g1_, self.g2_ * vec2(1.0, -1.0), self.g3_, self.g4_, self.g5_ * vec4(-1.0), self.g6_, self.g7_ * vec3(-1.0), self.g8_ * vec3(-1.0), self.g9_ * vec3(-1.0), self.g10_ * vec2(1.0, -1.0));
}

Plane plane_conformalConjugate(Plane self) {
    return Plane(self.g0_ * vec4(-1.0));
}

RoundPoint roundPoint_conformalConjugate(RoundPoint self) {
    return RoundPoint(self.g0_, self.g1_ * vec2(1.0, -1.0));
}

Scalar scalar_conformalConjugate(Scalar self) {
    return self;
}

Sphere sphere_conformalConjugate(Sphere self) {
    return Sphere(self.g0_ * vec3(-1.0), self.g1_ * vec2(1.0, -1.0));
}

AntiScalar antiScalar_conjugation(AntiScalar self) {
    return AntiScalar(-self.g0_);
}

Circle circle_conjugation(Circle self) {
    return self;
}

Dipole dipole_conjugation(Dipole self) {
    return Dipole(self.g0_ * vec3(-1.0), self.g1_ * vec3(-1.0), self.g2_ * vec4(-1.0));
}

DualNum dualNum_conjugation(DualNum self) {
    return DualNum(self.g0_ * vec2(1.0, -1.0));
}

FlatPoint flatPoint_conjugation(FlatPoint self) {
    return FlatPoint(self.g0_ * vec4(-1.0));
}

Flector flector_conjugation(Flector self) {
    return Flector(self.g0_ * vec4(-1.0), self.g1_);
}

Line line_conjugation(Line self) {
    return self;
}

Motor motor_conjugation(Motor self) {
    return Motor(self.g0_ * vec4(1.0, 1.0, 1.0, -1.0), self.g1_);
}

MultiVector multiVector_conjugation(MultiVector self) {
    return MultiVector(self.g0_ * vec2(1.0, -1.0), self.g1_ * vec3(-1.0), self.g2_ * vec2(-1.0), self.g3_ * vec3(-1.0), self.g4_ * vec3(-1.0), self.g5_ * vec4(-1.0), self.g6_, self.g7_, self.g8_, self.g9_, self.g10_);
}

Plane plane_conjugation(Plane self) {
    return self;
}

RoundPoint roundPoint_conjugation(RoundPoint self) {
    return RoundPoint(self.g0_ * vec3(-1.0), self.g1_ * vec2(-1.0));
}

Scalar scalar_conjugation(Scalar self) {
    return self;
}

Sphere sphere_conjugation(Sphere self) {
    return self;
}

AntiScalar antiScalar_doubleComplement(AntiScalar self) {
    return self;
}

Circle circle_doubleComplement(Circle self) {
    return self;
}

Dipole dipole_doubleComplement(Dipole self) {
    return self;
}

DualNum dualNum_doubleComplement(DualNum self) {
    return self;
}

FlatPoint flatPoint_doubleComplement(FlatPoint self) {
    return self;
}

Flector flector_doubleComplement(Flector self) {
    return self;
}

Line line_doubleComplement(Line self) {
    return self;
}

Motor motor_doubleComplement(Motor self) {
    return self;
}

MultiVector multiVector_doubleComplement(MultiVector self) {
    return self;
}

Plane plane_doubleComplement(Plane self) {
    return self;
}

RoundPoint roundPoint_doubleComplement(RoundPoint self) {
    return self;
}

Scalar scalar_doubleComplement(Scalar self) {
    return self;
}

Sphere sphere_doubleComplement(Sphere self) {
    return self;
}

Scalar antiScalar_dual(AntiScalar self) {
    return Scalar(-self.g0_);
}

Dipole circle_dual(Circle self) {
    return Dipole(vec3(self.g0_.x, self.g0_.y, self.g0_.z), self.g1_, vec4(self.g2_.x, self.g2_.y, self.g2_.z, -self.g0_.w));
}

Circle dipole_dual(Dipole self) {
    return Circle(vec4(-self.g0_.x, -self.g0_.y, -self.g0_.z, self.g2_.w), self.g1_ * vec3(-1.0), vec3(-self.g2_.x, self.g2_.y, self.g2_.z));
}

DualNum dualNum_dual(DualNum self) {
    return DualNum(self.g0_.yx * vec2(-1.0, 1.0));
}

Circle flatPoint_dual(FlatPoint self) {
    return Circle(vec4(0.0, 0.0, 0.0, self.g0_.w), vec3(0.0), vec3(-self.g0_.x, self.g0_.y, self.g0_.z));
}

MultiVector flector_dual(Flector self) {
    return MultiVector(vec2(0.0), vec3(-self.g1_.x, self.g1_.y, self.g1_.z), vec2(0.0, self.g1_.w), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0, 0.0, 0.0, self.g0_.w), vec3(0.0), vec3(-self.g0_.x, self.g0_.y, self.g0_.z), vec3(0.0), vec2(0.0));
}

Dipole line_dual(Line self) {
    return Dipole(vec3(0.0), self.g0_, vec4(self.g1_.x, self.g1_.y, self.g1_.z, 0.0));
}

MultiVector motor_dual(Motor self) {
    return MultiVector(vec2(-self.g0_.w, 0.0), vec3(0.0), vec2(0.0), vec3(0.0), vec3(self.g0_.x, self.g0_.y, self.g0_.z), vec4(self.g1_.x, self.g1_.y, self.g1_.z, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec2(0.0));
}

MultiVector multiVector_dual(MultiVector self) {
    return MultiVector(self.g0_.yx * vec2(-1.0, 1.0), self.g9_ * vec3(-1.0), self.g10_, vec3(self.g6_.x, self.g6_.y, self.g6_.z), self.g7_, vec4(self.g8_.x, self.g8_.y, self.g8_.z, -self.g6_.w), vec4(-self.g3_.x, -self.g3_.y, -self.g3_.z, self.g5_.w), self.g4_ * vec3(-1.0), vec3(-self.g5_.x, self.g5_.y, self.g5_.z), self.g1_, self.g2_ * vec2(-1.0));
}

RoundPoint plane_dual(Plane self) {
    return RoundPoint(vec3(-self.g0_.x, self.g0_.y, self.g0_.z), vec2(0.0, self.g0_.w));
}

Sphere roundPoint_dual(RoundPoint self) {
    return Sphere(self.g0_, self.g1_ * vec2(-1.0));
}

AntiScalar scalar_dual(Scalar self) {
    return AntiScalar(self.g0_);
}

RoundPoint sphere_dual(Sphere self) {
    return RoundPoint(self.g0_ * vec3(-1.0), self.g1_);
}

AntiScalar antiScalar_reversal(AntiScalar self) {
    return self;
}

Circle circle_reversal(Circle self) {
    return Circle(self.g0_ * vec4(-1.0), self.g1_ * vec3(-1.0), self.g2_ * vec3(-1.0));
}

Dipole dipole_reversal(Dipole self) {
    return Dipole(self.g0_ * vec3(-1.0), self.g1_ * vec3(-1.0), self.g2_ * vec4(-1.0));
}

DualNum dualNum_reversal(DualNum self) {
    return self;
}

FlatPoint flatPoint_reversal(FlatPoint self) {
    return FlatPoint(self.g0_ * vec4(-1.0));
}

Flector flector_reversal(Flector self) {
    return Flector(self.g0_ * vec4(-1.0), self.g1_);
}

Line line_reversal(Line self) {
    return Line(self.g0_ * vec3(-1.0), self.g1_ * vec3(-1.0));
}

Motor motor_reversal(Motor self) {
    return Motor(self.g0_ * vec4(-1.0, -1.0, -1.0, 1.0), self.g1_ * vec3(-1.0));
}

MultiVector multiVector_reversal(MultiVector self) {
    return MultiVector(self.g0_, self.g1_, self.g2_, self.g3_ * vec3(-1.0), self.g4_ * vec3(-1.0), self.g5_ * vec4(-1.0), self.g6_ * vec4(-1.0), self.g7_ * vec3(-1.0), self.g8_ * vec3(-1.0), self.g9_, self.g10_);
}

Plane plane_reversal(Plane self) {
    return self;
}

RoundPoint roundPoint_reversal(RoundPoint self) {
    return self;
}

Scalar scalar_reversal(Scalar self) {
    return self;
}

Sphere sphere_reversal(Sphere self) {
    return self;
}

Dipole circle_flatBulkDual(Circle self) {
    return circle_dual(circle_flatBulk(self));
}

Circle dipole_flatBulkDual(Dipole self) {
    return dipole_dual(dipole_flatBulk(self));
}

Circle flatPoint_flatBulkDual(FlatPoint self) {
    return flatPoint_dual(flatPoint_flatBulk(self));
}

MultiVector flector_flatBulkDual(Flector self) {
    return flector_dual(flector_flatBulk(self));
}

Dipole line_flatBulkDual(Line self) {
    return line_dual(line_flatBulk(self));
}

MultiVector motor_flatBulkDual(Motor self) {
    return motor_dual(motor_flatBulk(self));
}

MultiVector multiVector_flatBulkDual(MultiVector self) {
    return multiVector_dual(multiVector_flatBulk(self));
}

RoundPoint plane_flatBulkDual(Plane self) {
    return plane_dual(plane_flatBulk(self));
}

Sphere roundPoint_flatBulkDual(RoundPoint self) {
    return roundPoint_dual(roundPoint_flatBulk(self));
}

RoundPoint sphere_flatBulkDual(Sphere self) {
    return sphere_dual(sphere_flatBulk(self));
}

Scalar antiScalar_flatWeightDual(AntiScalar self) {
    return antiScalar_dual(antiScalar_flatWeight(self));
}

Dipole circle_flatWeightDual(Circle self) {
    return circle_dual(circle_flatWeight(self));
}

Circle dipole_flatWeightDual(Dipole self) {
    return dipole_dual(dipole_flatWeight(self));
}

DualNum dualNum_flatWeightDual(DualNum self) {
    return dualNum_dual(dualNum_flatWeight(self));
}

Circle flatPoint_flatWeightDual(FlatPoint self) {
    return flatPoint_dual(flatPoint_flatWeight(self));
}

MultiVector flector_flatWeightDual(Flector self) {
    return flector_dual(flector_flatWeight(self));
}

Dipole line_flatWeightDual(Line self) {
    return line_dual(line_flatWeight(self));
}

MultiVector motor_flatWeightDual(Motor self) {
    return motor_dual(motor_flatWeight(self));
}

MultiVector multiVector_flatWeightDual(MultiVector self) {
    return multiVector_dual(multiVector_flatWeight(self));
}

RoundPoint plane_flatWeightDual(Plane self) {
    return plane_dual(plane_flatWeight(self));
}

RoundPoint sphere_flatWeightDual(Sphere self) {
    return sphere_dual(sphere_flatWeight(self));
}

Dipole circle_roundBulkDual(Circle self) {
    return circle_dual(circle_roundBulk(self));
}

Circle dipole_roundBulkDual(Dipole self) {
    return dipole_dual(dipole_roundBulk(self));
}

DualNum dualNum_roundBulkDual(DualNum self) {
    return dualNum_dual(dualNum_roundBulk(self));
}

MultiVector multiVector_roundBulkDual(MultiVector self) {
    return multiVector_dual(multiVector_roundBulk(self));
}

Sphere roundPoint_roundBulkDual(RoundPoint self) {
    return roundPoint_dual(roundPoint_roundBulk(self));
}

AntiScalar scalar_roundBulkDual(Scalar self) {
    return scalar_dual(scalar_roundBulk(self));
}

Dipole circle_roundWeightDual(Circle self) {
    return circle_dual(circle_roundWeight(self));
}

Circle dipole_roundWeightDual(Dipole self) {
    return dipole_dual(dipole_roundWeight(self));
}

MultiVector multiVector_roundWeightDual(MultiVector self) {
    return multiVector_dual(multiVector_roundWeight(self));
}

Sphere roundPoint_roundWeightDual(RoundPoint self) {
    return roundPoint_dual(roundPoint_roundWeight(self));
}

RoundPoint sphere_roundWeightDual(Sphere self) {
    return sphere_dual(sphere_roundWeight(self));
}

int antiScalar_antiGrade() {
    return 0;
}

int circle_antiGrade() {
    return 2;
}

int dipole_antiGrade() {
    return 3;
}

int flatPoint_antiGrade() {
    return 3;
}

int line_antiGrade() {
    return 2;
}

int plane_antiGrade() {
    return 1;
}

int roundPoint_antiGrade() {
    return 4;
}

int scalar_antiGrade() {
    return 5;
}

int sphere_antiGrade() {
    return 1;
}

int antiScalar_grade() {
    return 5;
}

int circle_grade() {
    return 3;
}

int dipole_grade() {
    return 2;
}

int flatPoint_grade() {
    return 2;
}

int line_grade() {
    return 3;
}

int plane_grade() {
    return 4;
}

int roundPoint_grade() {
    return 1;
}

int scalar_grade() {
    return 0;
}

int sphere_grade() {
    return 4;
}

AntiScalar antiScalar_antiSqrt(AntiScalar self) {
    return AntiScalar(sqrt(self.g0_));
}

DualNum dualNum_antiSqrt(DualNum self) {
    float s = self.g0_.x;
    float t = self.g0_.y;
    float sqrt_t = sqrt(t);
    return DualNum(vec2(s / (2.0 * sqrt_t), sqrt_t));
}

Plane antiScalar_attitude(AntiScalar self) {
    return antiScalar_antiWedge_plane(self, Plane(vec4(0.0, 0.0, 0.0, 1.0)));
}

Dipole circle_attitude(Circle self) {
    return circle_antiWedge_plane(self, Plane(vec4(0.0, 0.0, 0.0, 1.0)));
}

RoundPoint dipole_attitude(Dipole self) {
    return dipole_antiWedge_plane(self, Plane(vec4(0.0, 0.0, 0.0, 1.0)));
}

Plane dualNum_attitude(DualNum self) {
    return dualNum_antiWedge_plane(self, Plane(vec4(0.0, 0.0, 0.0, 1.0)));
}

RoundPoint flatPoint_attitude(FlatPoint self) {
    return flatPoint_antiWedge_plane(self, Plane(vec4(0.0, 0.0, 0.0, 1.0)));
}

MultiVector flector_attitude(Flector self) {
    return flector_antiWedge_plane(self, Plane(vec4(0.0, 0.0, 0.0, 1.0)));
}

FlatPoint line_attitude(Line self) {
    return line_antiWedge_plane(self, Plane(vec4(0.0, 0.0, 0.0, 1.0)));
}

Flector motor_attitude(Motor self) {
    return motor_antiWedge_plane(self, Plane(vec4(0.0, 0.0, 0.0, 1.0)));
}

MultiVector multiVector_attitude(MultiVector self) {
    return multiVector_antiWedge_plane(self, Plane(vec4(0.0, 0.0, 0.0, 1.0)));
}

Line plane_attitude(Plane self) {
    return plane_antiWedge_plane(self, Plane(vec4(0.0, 0.0, 0.0, 1.0)));
}

Scalar roundPoint_attitude(RoundPoint self) {
    return roundPoint_antiWedge_plane(self, Plane(vec4(0.0, 0.0, 0.0, 1.0)));
}

Circle sphere_attitude(Sphere self) {
    return sphere_antiWedge_plane(self, Plane(vec4(0.0, 0.0, 0.0, 1.0)));
}

Sphere circle_carrier(Circle self) {
    return circle_wedge_roundPoint(self, RoundPoint(vec3(0.0), vec2(0.0, 1.0)));
}

Circle dipole_carrier(Dipole self) {
    return dipole_wedge_roundPoint(self, RoundPoint(vec3(0.0), vec2(0.0, 1.0)));
}

RoundPoint dualNum_carrier(DualNum self) {
    return dualNum_wedge_roundPoint(self, RoundPoint(vec3(0.0), vec2(0.0, 1.0)));
}

MultiVector multiVector_carrier(MultiVector self) {
    return multiVector_wedge_roundPoint(self, RoundPoint(vec3(0.0), vec2(0.0, 1.0)));
}

Dipole roundPoint_carrier(RoundPoint self) {
    return roundPoint_wedge_roundPoint(self, RoundPoint(vec3(0.0), vec2(0.0, 1.0)));
}

RoundPoint scalar_carrier(Scalar self) {
    return scalar_wedge_roundPoint(self, RoundPoint(vec3(0.0), vec2(0.0, 1.0)));
}

AntiScalar sphere_carrier(Sphere self) {
    return sphere_wedge_roundPoint(self, RoundPoint(vec3(0.0), vec2(0.0, 1.0)));
}

Circle circle_coCarrier(Circle self) {
    return dipole_wedge_roundPoint(circle_antiDual(self), RoundPoint(vec3(0.0), vec2(0.0, 1.0)));
}

Sphere dipole_coCarrier(Dipole self) {
    return circle_wedge_roundPoint(dipole_antiDual(self), RoundPoint(vec3(0.0), vec2(0.0, 1.0)));
}

RoundPoint dualNum_coCarrier(DualNum self) {
    return dualNum_wedge_roundPoint(dualNum_antiDual(self), RoundPoint(vec3(0.0), vec2(0.0, 1.0)));
}

MultiVector multiVector_coCarrier(MultiVector self) {
    return multiVector_wedge_roundPoint(multiVector_antiDual(self), RoundPoint(vec3(0.0), vec2(0.0, 1.0)));
}

AntiScalar roundPoint_coCarrier(RoundPoint self) {
    return sphere_wedge_roundPoint(roundPoint_antiDual(self), RoundPoint(vec3(0.0), vec2(0.0, 1.0)));
}

Dipole sphere_coCarrier(Sphere self) {
    return roundPoint_wedge_roundPoint(sphere_antiDual(self), RoundPoint(vec3(0.0), vec2(0.0, 1.0)));
}

DualNum dualNum_sqrt(DualNum self) {
    float s = self.g0_.x;
    float t = self.g0_.y;
    float sqrt_s = sqrt(s);
    return DualNum(vec2(sqrt_s, t / (2.0 * sqrt_s)));
}

Scalar scalar_sqrt(Scalar self) {
    return Scalar(sqrt(self.g0_));
}

RoundPoint circle_center(Circle self) {
    return circle_antiWedge_circle(circle_coCarrier(self), self);
}

RoundPoint dipole_center(Dipole self) {
    return sphere_antiWedge_dipole(dipole_coCarrier(self), self);
}

RoundPoint dualNum_center(DualNum self) {
    return roundPoint_antiWedge_dualNum(dualNum_coCarrier(self), self);
}

MultiVector multiVector_center(MultiVector self) {
    return multiVector_antiWedge_multiVector(multiVector_coCarrier(self), self);
}

RoundPoint roundPoint_center(RoundPoint self) {
    return antiScalar_antiWedge_roundPoint(roundPoint_coCarrier(self), self);
}

RoundPoint sphere_center(Sphere self) {
    return dipole_antiWedge_sphere(sphere_coCarrier(self), self);
}

Sphere circle_container(Circle self) {
    return circle_wedge_roundPoint(self, sphere_antiDual(circle_carrier(self)));
}

Sphere dipole_container(Dipole self) {
    return dipole_wedge_dipole(self, circle_antiDual(dipole_carrier(self)));
}

Sphere dualNum_container(DualNum self) {
    return dualNum_wedge_sphere(self, roundPoint_antiDual(dualNum_carrier(self)));
}

MultiVector multiVector_container(MultiVector self) {
    return multiVector_wedge_multiVector(self, multiVector_antiDual(multiVector_carrier(self)));
}

Sphere roundPoint_container(RoundPoint self) {
    return roundPoint_wedge_circle(self, dipole_antiDual(roundPoint_carrier(self)));
}

Sphere scalar_container(Scalar self) {
    return scalar_wedge_sphere(self, roundPoint_antiDual(scalar_carrier(self)));
}

Sphere sphere_container(Sphere self) {
    return sphere_wedge_scalar(self, antiScalar_antiDual(sphere_carrier(self)));
}

Circle circle_partner(Circle self) {
    return sphere_antiWedge_sphere(sphere_neg(dipole_container(circle_dual(self))), circle_carrier(self));
}

Dipole dipole_partner(Dipole self) {
    return sphere_antiWedge_circle(sphere_neg(circle_container(dipole_dual(self))), dipole_carrier(self));
}

Scalar dualNum_partner(DualNum self) {
    return sphere_antiWedge_roundPoint(sphere_neg(dualNum_container(dualNum_dual(self))), dualNum_carrier(self));
}

MultiVector multiVector_partner(MultiVector self) {
    return multiVector_antiWedge_multiVector(multiVector_neg(multiVector_container(multiVector_dual(self))), multiVector_carrier(self));
}

RoundPoint roundPoint_partner(RoundPoint self) {
    return sphere_antiWedge_dipole(sphere_neg(sphere_container(roundPoint_dual(self))), roundPoint_carrier(self));
}

Sphere sphere_partner(Sphere self) {
    return sphere_antiWedge_antiScalar(sphere_neg(roundPoint_container(sphere_dual(self))), sphere_carrier(self));
}

AntiScalar antiScalar_antiInverse(AntiScalar self) {
    return antiScalar_antiWedgeDot_antiScalar(self, antiScalar_div_antiScalar(antiScalar_unit(), antiScalar_antiDot_antiScalar(self, self)));
}

Circle circle_antiInverse(Circle self) {
    return circle_antiWedgeDot_antiScalar(self, antiScalar_div_antiScalar(antiScalar_unit(), circle_antiDot_circle(self, self)));
}

Dipole dipole_antiInverse(Dipole self) {
    return dipole_antiWedgeDot_antiScalar(self, antiScalar_div_antiScalar(antiScalar_unit(), dipole_antiDot_dipole(self, self)));
}

DualNum dualNum_antiInverse(DualNum self) {
    float s = self.g0_.x;
    float t = self.g0_.y;
    return DualNum(vec2(-1.0 * s / (t * t), 1.0 / t));
}

FlatPoint flatPoint_antiInverse(FlatPoint self) {
    return flatPoint_antiWedgeDot_antiScalar(self, antiScalar_div_antiScalar(antiScalar_unit(), flatPoint_antiDot_flatPoint(self, self)));
}

Flector flector_antiInverse(Flector self) {
    return flector_antiWedgeDot_antiScalar(self, antiScalar_div_antiScalar(antiScalar_unit(), flector_antiDot_flector(self, self)));
}

Line line_antiInverse(Line self) {
    return line_antiWedgeDot_antiScalar(self, antiScalar_div_antiScalar(antiScalar_unit(), line_antiDot_line(self, self)));
}

Motor motor_antiInverse(Motor self) {
    return motor_antiWedgeDot_antiScalar(self, antiScalar_div_antiScalar(antiScalar_unit(), motor_antiDot_motor(self, self)));
}

MultiVector multiVector_antiInverse(MultiVector self) {
    return multiVector_antiWedgeDot_antiScalar(self, antiScalar_div_antiScalar(antiScalar_unit(), multiVector_antiDot_multiVector(self, self)));
}

Plane plane_antiInverse(Plane self) {
    return plane_antiWedgeDot_antiScalar(self, antiScalar_div_antiScalar(antiScalar_unit(), plane_antiDot_plane(self, self)));
}

RoundPoint roundPoint_antiInverse(RoundPoint self) {
    return roundPoint_antiWedgeDot_antiScalar(self, antiScalar_div_antiScalar(antiScalar_unit(), roundPoint_antiDot_roundPoint(self, self)));
}

Scalar scalar_antiInverse(Scalar self) {
    return scalar_antiWedgeDot_antiScalar(self, antiScalar_div_antiScalar(antiScalar_unit(), scalar_antiDot_scalar(self, self)));
}

Sphere sphere_antiInverse(Sphere self) {
    return sphere_antiWedgeDot_antiScalar(self, antiScalar_div_antiScalar(antiScalar_unit(), sphere_antiDot_sphere(self, self)));
}

AntiScalar antiScalar_inverse(AntiScalar self) {
    return antiScalar_wedgeDot_scalar(self, scalar_div_scalar(scalar_unit(), antiScalar_dot_antiScalar(self, self)));
}

Circle circle_inverse(Circle self) {
    return circle_wedgeDot_scalar(self, scalar_div_scalar(scalar_unit(), circle_dot_circle(self, self)));
}

Dipole dipole_inverse(Dipole self) {
    return dipole_wedgeDot_scalar(self, scalar_div_scalar(scalar_unit(), dipole_dot_dipole(self, self)));
}

DualNum dualNum_inverse(DualNum self) {
    float s = self.g0_.x;
    float t = self.g0_.y;
    return DualNum(vec2(1.0 / s, -1.0 * t / (s * s)));
}

FlatPoint flatPoint_inverse(FlatPoint self) {
    return flatPoint_wedgeDot_scalar(self, scalar_div_scalar(scalar_unit(), flatPoint_dot_flatPoint(self, self)));
}

Flector flector_inverse(Flector self) {
    return flector_wedgeDot_scalar(self, scalar_div_scalar(scalar_unit(), flector_dot_flector(self, self)));
}

Line line_inverse(Line self) {
    return line_wedgeDot_scalar(self, scalar_div_scalar(scalar_unit(), line_dot_line(self, self)));
}

Motor motor_inverse(Motor self) {
    return motor_wedgeDot_scalar(self, scalar_div_scalar(scalar_unit(), motor_dot_motor(self, self)));
}

MultiVector multiVector_inverse(MultiVector self) {
    return multiVector_wedgeDot_scalar(self, scalar_div_scalar(scalar_unit(), multiVector_dot_multiVector(self, self)));
}

Plane plane_inverse(Plane self) {
    return plane_wedgeDot_scalar(self, scalar_div_scalar(scalar_unit(), plane_dot_plane(self, self)));
}

RoundPoint roundPoint_inverse(RoundPoint self) {
    return roundPoint_wedgeDot_scalar(self, scalar_div_scalar(scalar_unit(), roundPoint_dot_roundPoint(self, self)));
}

Scalar scalar_inverse(Scalar self) {
    return scalar_wedgeDot_scalar(self, scalar_div_scalar(scalar_unit(), scalar_dot_scalar(self, self)));
}

Sphere sphere_inverse(Sphere self) {
    return sphere_wedgeDot_scalar(self, scalar_div_scalar(scalar_unit(), sphere_dot_sphere(self, self)));
}

DualNum dualNum_antiCos(DualNum self) {
    float s = self.g0_.x;
    float t = self.g0_.y;
    return DualNum(vec2(-1.0 * s * sin(t), cos(t)));
}

AntiScalar antiScalar_antiCosh(AntiScalar self) {
    return AntiScalar(cosh(self.g0_));
}

DualNum dualNum_antiCosh(DualNum self) {
    float s = self.g0_.x;
    float t = self.g0_.y;
    return DualNum(vec2(s * sinh(t), cosh(t)));
}

AntiScalar antiScalar_antiExp(AntiScalar self) {
    return AntiScalar(exp(self.g0_));
}

DualNum dualNum_antiExp(DualNum self) {
    float s = self.g0_.x;
    float t = self.g0_.y;
    float exp_t = exp(t);
    return DualNum(vec2(s * exp_t, exp_t));
}

AntiScalar antiScalar_antiInverseSqrt(AntiScalar self) {
    return AntiScalar(1.0 / sqrt(self.g0_));
}

DualNum dualNum_antiInverseSqrt(DualNum self) {
    float s = self.g0_.x;
    float t = self.g0_.y;
    float sqrt_t = sqrt(t);
    return DualNum(vec2(-1.0 * s / (2.0 * t * sqrt_t), 1.0 / sqrt_t));
}

AntiScalar antiScalar_antiPow(AntiScalar self, float other) {
    return AntiScalar(pow(self.g0_, other));
}

DualNum dualNum_antiPow(DualNum self, float other) {
    float s = self.g0_.x;
    float t = self.g0_.y;
    return DualNum(vec2(other * pow(t, other - 1.0) * s, pow(t, other)));
}

DualNum dualNum_antiSin(DualNum self) {
    float s = self.g0_.x;
    float t = self.g0_.y;
    return DualNum(vec2(s * cos(t), sin(t)));
}

AntiScalar antiScalar_antiSinh(AntiScalar self) {
    return AntiScalar(sinh(self.g0_));
}

DualNum dualNum_antiSinh(DualNum self) {
    float s = self.g0_.x;
    float t = self.g0_.y;
    return DualNum(vec2(s * cosh(t), sinh(t)));
}

DualNum dualNum_antiTan(DualNum self) {
    float s = self.g0_.x;
    float t = self.g0_.y;
    float tan_t = tan(t);
    return DualNum(vec2(s * (1.0 + tan_t * tan_t), tan_t));
}

AntiScalar antiScalar_antiTanh(AntiScalar self) {
    return AntiScalar(tanh(self.g0_));
}

DualNum dualNum_antiTanh(DualNum self) {
    float s = self.g0_.x;
    float t = self.g0_.y;
    float tanh_t = tanh(t);
    return DualNum(vec2(s * (1.0 - tanh_t * tanh_t), tanh_t));
}

DualNum dualNum_cos(DualNum self) {
    float s = self.g0_.x;
    float t = self.g0_.y;
    return DualNum(vec2(cos(s), -1.0 * t * sin(s)));
}

DualNum dualNum_cosh(DualNum self) {
    float s = self.g0_.x;
    float t = self.g0_.y;
    return DualNum(vec2(cosh(s), t * sinh(s)));
}

Scalar scalar_cosh(Scalar self) {
    return Scalar(cosh(self.g0_));
}

DualNum dualNum_exp(DualNum self) {
    float s = self.g0_.x;
    float t = self.g0_.y;
    float exp_s = exp(s);
    return DualNum(vec2(exp_s, t * exp_s));
}

Scalar scalar_exp(Scalar self) {
    return Scalar(exp(self.g0_));
}

DualNum dualNum_inverseSqrt(DualNum self) {
    float s = self.g0_.x;
    float t = self.g0_.y;
    float sqrt_s = sqrt(s);
    return DualNum(vec2(1.0 / sqrt_s, -1.0 * t / (2.0 * s * sqrt_s)));
}

Scalar scalar_inverseSqrt(Scalar self) {
    return Scalar(1.0 / sqrt(self.g0_));
}

DualNum dualNum_pow(DualNum self, float other) {
    float s = self.g0_.x;
    float t = self.g0_.y;
    return DualNum(vec2(pow(s, other), other * pow(s, other - 1.0) * t));
}

Scalar scalar_pow(Scalar self, float other) {
    return Scalar(pow(self.g0_, other));
}

DualNum dualNum_sin(DualNum self) {
    float s = self.g0_.x;
    float t = self.g0_.y;
    return DualNum(vec2(sin(s), t * cos(s)));
}

DualNum dualNum_sinh(DualNum self) {
    float s = self.g0_.x;
    float t = self.g0_.y;
    return DualNum(vec2(sinh(s), t * cosh(s)));
}

Scalar scalar_sinh(Scalar self) {
    return Scalar(sinh(self.g0_));
}

DualNum dualNum_tan(DualNum self) {
    float s = self.g0_.x;
    float t = self.g0_.y;
    float tan_s = tan(s);
    return DualNum(vec2(tan_s, t * (1.0 + tan_s * tan_s)));
}

DualNum dualNum_tanh(DualNum self) {
    float s = self.g0_.x;
    float t = self.g0_.y;
    float tanh_s = tanh(s);
    return DualNum(vec2(tanh_s, t * (1.0 - tanh_s * tanh_s)));
}

Scalar scalar_tanh(Scalar self) {
    return Scalar(tanh(self.g0_));
}

Scalar circle_roundBulkNormSquared(Circle self) {
    Circle round_bulk_carrier = circle_roundBulk(self);
    return circle_dot_circle(round_bulk_carrier, round_bulk_carrier);
}

Scalar dipole_roundBulkNormSquared(Dipole self) {
    Dipole round_bulk_carrier = dipole_roundBulk(self);
    return dipole_dot_dipole(round_bulk_carrier, round_bulk_carrier);
}

Scalar dualNum_roundBulkNormSquared(DualNum self) {
    DualNum round_bulk_carrier = dualNum_roundBulk(self);
    return dualNum_dot_dualNum(round_bulk_carrier, round_bulk_carrier);
}

Scalar multiVector_roundBulkNormSquared(MultiVector self) {
    MultiVector round_bulk_carrier = multiVector_roundBulk(self);
    return multiVector_dot_multiVector(round_bulk_carrier, round_bulk_carrier);
}

Scalar roundPoint_roundBulkNormSquared(RoundPoint self) {
    RoundPoint round_bulk_carrier = roundPoint_roundBulk(self);
    return roundPoint_dot_roundPoint(round_bulk_carrier, round_bulk_carrier);
}

Scalar scalar_roundBulkNormSquared(Scalar self) {
    Scalar round_bulk_carrier = scalar_roundBulk(self);
    return scalar_dot_scalar(round_bulk_carrier, round_bulk_carrier);
}

Scalar circle_roundBulkNorm(Circle self) {
    return scalar_sqrt(circle_roundBulkNormSquared(self));
}

Scalar dipole_roundBulkNorm(Dipole self) {
    return scalar_sqrt(dipole_roundBulkNormSquared(self));
}

Scalar dualNum_roundBulkNorm(DualNum self) {
    return scalar_sqrt(dualNum_roundBulkNormSquared(self));
}

Scalar multiVector_roundBulkNorm(MultiVector self) {
    return scalar_sqrt(multiVector_roundBulkNormSquared(self));
}

Scalar roundPoint_roundBulkNorm(RoundPoint self) {
    return scalar_sqrt(roundPoint_roundBulkNormSquared(self));
}

Scalar scalar_roundBulkNorm(Scalar self) {
    return scalar_sqrt(scalar_roundBulkNormSquared(self));
}

AntiScalar circle_roundWeightNormSquared(Circle self) {
    Sphere round_weight_carrier = circle_wedge_roundPoint(circle_roundWeight(self), RoundPoint(vec3(0.0), vec2(0.0, 1.0)));
    return sphere_antiDot_sphere(round_weight_carrier, round_weight_carrier);
}

AntiScalar dipole_roundWeightNormSquared(Dipole self) {
    Circle round_weight_carrier = dipole_wedge_roundPoint(dipole_roundWeight(self), RoundPoint(vec3(0.0), vec2(0.0, 1.0)));
    return circle_antiDot_circle(round_weight_carrier, round_weight_carrier);
}

AntiScalar multiVector_roundWeightNormSquared(MultiVector self) {
    MultiVector round_weight_carrier = multiVector_wedge_roundPoint(multiVector_roundWeight(self), RoundPoint(vec3(0.0), vec2(0.0, 1.0)));
    return multiVector_antiDot_multiVector(round_weight_carrier, round_weight_carrier);
}

AntiScalar roundPoint_roundWeightNormSquared(RoundPoint self) {
    Dipole round_weight_carrier = roundPoint_wedge_roundPoint(roundPoint_roundWeight(self), RoundPoint(vec3(0.0), vec2(0.0, 1.0)));
    return dipole_antiDot_dipole(round_weight_carrier, round_weight_carrier);
}

AntiScalar sphere_roundWeightNormSquared(Sphere self) {
    AntiScalar round_weight_carrier = sphere_wedge_roundPoint(sphere_roundWeight(self), RoundPoint(vec3(0.0), vec2(0.0, 1.0)));
    return antiScalar_antiDot_antiScalar(round_weight_carrier, round_weight_carrier);
}

AntiScalar circle_roundWeightNorm(Circle self) {
    return antiScalar_antiSqrt(circle_roundWeightNormSquared(self));
}

AntiScalar dipole_roundWeightNorm(Dipole self) {
    return antiScalar_antiSqrt(dipole_roundWeightNormSquared(self));
}

AntiScalar multiVector_roundWeightNorm(MultiVector self) {
    return antiScalar_antiSqrt(multiVector_roundWeightNormSquared(self));
}

AntiScalar roundPoint_roundWeightNorm(RoundPoint self) {
    return antiScalar_antiSqrt(roundPoint_roundWeightNormSquared(self));
}

AntiScalar sphere_roundWeightNorm(Sphere self) {
    return antiScalar_antiSqrt(sphere_roundWeightNormSquared(self));
}

DualNum circle_roundNormSquared(Circle self) {
    return scalar_add_antiScalar(circle_roundBulkNormSquared(self), circle_roundWeightNormSquared(self));
}

DualNum dipole_roundNormSquared(Dipole self) {
    return scalar_add_antiScalar(dipole_roundBulkNormSquared(self), dipole_roundWeightNormSquared(self));
}

DualNum multiVector_roundNormSquared(MultiVector self) {
    return scalar_add_antiScalar(multiVector_roundBulkNormSquared(self), multiVector_roundWeightNormSquared(self));
}

DualNum roundPoint_roundNormSquared(RoundPoint self) {
    return scalar_add_antiScalar(roundPoint_roundBulkNormSquared(self), roundPoint_roundWeightNormSquared(self));
}

DualNum circle_roundNorm(Circle self) {
    return scalar_add_antiScalar(circle_roundBulkNorm(self), circle_roundWeightNorm(self));
}

DualNum dipole_roundNorm(Dipole self) {
    return scalar_add_antiScalar(dipole_roundBulkNorm(self), dipole_roundWeightNorm(self));
}

DualNum multiVector_roundNorm(MultiVector self) {
    return scalar_add_antiScalar(multiVector_roundBulkNorm(self), multiVector_roundWeightNorm(self));
}

DualNum roundPoint_roundNorm(RoundPoint self) {
    return scalar_add_antiScalar(roundPoint_roundBulkNorm(self), roundPoint_roundWeightNorm(self));
}

float circle_unitizedRoundNormSquared(Circle self) {
    return circle_roundBulkNormSquared(self).g0_ / circle_roundWeightNormSquared(self).g0_;
}

float dipole_unitizedRoundNormSquared(Dipole self) {
    return dipole_roundBulkNormSquared(self).g0_ / dipole_roundWeightNormSquared(self).g0_;
}

float multiVector_unitizedRoundNormSquared(MultiVector self) {
    return multiVector_roundBulkNormSquared(self).g0_ / multiVector_roundWeightNormSquared(self).g0_;
}

float roundPoint_unitizedRoundNormSquared(RoundPoint self) {
    return roundPoint_roundBulkNormSquared(self).g0_ / roundPoint_roundWeightNormSquared(self).g0_;
}

float circle_unitizedRoundNorm(Circle self) {
    return sqrt(circle_unitizedRoundNormSquared(self));
}

float dipole_unitizedRoundNorm(Dipole self) {
    return sqrt(dipole_unitizedRoundNormSquared(self));
}

float multiVector_unitizedRoundNorm(MultiVector self) {
    return sqrt(multiVector_unitizedRoundNormSquared(self));
}

float roundPoint_unitizedRoundNorm(RoundPoint self) {
    return sqrt(roundPoint_unitizedRoundNormSquared(self));
}

Scalar circle_flatBulkNormSquared(Circle self) {
    Sphere flat_bulk_thing = circle_wedge_roundPoint(circle_flatBulk(self), RoundPoint(vec3(0.0), vec2(1.0, 0.0)));
    return sphere_dot_sphere(flat_bulk_thing, flat_bulk_thing);
}

Scalar dipole_flatBulkNormSquared(Dipole self) {
    Circle flat_bulk_thing = dipole_wedge_roundPoint(dipole_flatBulk(self), RoundPoint(vec3(0.0), vec2(1.0, 0.0)));
    return circle_dot_circle(flat_bulk_thing, flat_bulk_thing);
}

Scalar flatPoint_flatBulkNormSquared(FlatPoint self) {
    Line flat_bulk_thing = flatPoint_wedge_roundPoint(flatPoint_flatBulk(self), RoundPoint(vec3(0.0), vec2(1.0, 0.0)));
    return line_dot_line(flat_bulk_thing, flat_bulk_thing);
}

Scalar flector_flatBulkNormSquared(Flector self) {
    Motor flat_bulk_thing = flector_wedge_roundPoint(flector_flatBulk(self), RoundPoint(vec3(0.0), vec2(1.0, 0.0)));
    return motor_dot_motor(flat_bulk_thing, flat_bulk_thing);
}

Scalar line_flatBulkNormSquared(Line self) {
    Plane flat_bulk_thing = line_wedge_roundPoint(line_flatBulk(self), RoundPoint(vec3(0.0), vec2(1.0, 0.0)));
    return plane_dot_plane(flat_bulk_thing, flat_bulk_thing);
}

Scalar motor_flatBulkNormSquared(Motor self) {
    Plane flat_bulk_thing = motor_wedge_roundPoint(motor_flatBulk(self), RoundPoint(vec3(0.0), vec2(1.0, 0.0)));
    return plane_dot_plane(flat_bulk_thing, flat_bulk_thing);
}

Scalar multiVector_flatBulkNormSquared(MultiVector self) {
    MultiVector flat_bulk_thing = multiVector_wedge_roundPoint(multiVector_flatBulk(self), RoundPoint(vec3(0.0), vec2(1.0, 0.0)));
    return multiVector_dot_multiVector(flat_bulk_thing, flat_bulk_thing);
}

Scalar plane_flatBulkNormSquared(Plane self) {
    AntiScalar flat_bulk_thing = plane_wedge_roundPoint(plane_flatBulk(self), RoundPoint(vec3(0.0), vec2(1.0, 0.0)));
    return antiScalar_dot_antiScalar(flat_bulk_thing, flat_bulk_thing);
}

Scalar roundPoint_flatBulkNormSquared(RoundPoint self) {
    Dipole flat_bulk_thing = roundPoint_wedge_roundPoint(roundPoint_flatBulk(self), RoundPoint(vec3(0.0), vec2(1.0, 0.0)));
    return dipole_dot_dipole(flat_bulk_thing, flat_bulk_thing);
}

Scalar sphere_flatBulkNormSquared(Sphere self) {
    AntiScalar flat_bulk_thing = sphere_wedge_roundPoint(sphere_flatBulk(self), RoundPoint(vec3(0.0), vec2(1.0, 0.0)));
    return antiScalar_dot_antiScalar(flat_bulk_thing, flat_bulk_thing);
}

Scalar circle_flatBulkNorm(Circle self) {
    return scalar_sqrt(circle_flatBulkNormSquared(self));
}

Scalar dipole_flatBulkNorm(Dipole self) {
    return scalar_sqrt(dipole_flatBulkNormSquared(self));
}

Scalar flatPoint_flatBulkNorm(FlatPoint self) {
    return scalar_sqrt(flatPoint_flatBulkNormSquared(self));
}

Scalar flector_flatBulkNorm(Flector self) {
    return scalar_sqrt(flector_flatBulkNormSquared(self));
}

Scalar line_flatBulkNorm(Line self) {
    return scalar_sqrt(line_flatBulkNormSquared(self));
}

Scalar motor_flatBulkNorm(Motor self) {
    return scalar_sqrt(motor_flatBulkNormSquared(self));
}

Scalar multiVector_flatBulkNorm(MultiVector self) {
    return scalar_sqrt(multiVector_flatBulkNormSquared(self));
}

Scalar plane_flatBulkNorm(Plane self) {
    return scalar_sqrt(plane_flatBulkNormSquared(self));
}

Scalar roundPoint_flatBulkNorm(RoundPoint self) {
    return scalar_sqrt(roundPoint_flatBulkNormSquared(self));
}

Scalar sphere_flatBulkNorm(Sphere self) {
    return scalar_sqrt(sphere_flatBulkNormSquared(self));
}

AntiScalar antiScalar_flatWeightNormSquared(AntiScalar self) {
    AntiScalar flat_weight = antiScalar_flatWeight(self);
    return antiScalar_antiDot_antiScalar(flat_weight, flat_weight);
}

AntiScalar circle_flatWeightNormSquared(Circle self) {
    Circle flat_weight = circle_flatWeight(self);
    return circle_antiDot_circle(flat_weight, flat_weight);
}

AntiScalar dipole_flatWeightNormSquared(Dipole self) {
    Dipole flat_weight = dipole_flatWeight(self);
    return dipole_antiDot_dipole(flat_weight, flat_weight);
}

AntiScalar dualNum_flatWeightNormSquared(DualNum self) {
    DualNum flat_weight = dualNum_flatWeight(self);
    return dualNum_antiDot_dualNum(flat_weight, flat_weight);
}

AntiScalar flatPoint_flatWeightNormSquared(FlatPoint self) {
    FlatPoint flat_weight = flatPoint_flatWeight(self);
    return flatPoint_antiDot_flatPoint(flat_weight, flat_weight);
}

AntiScalar flector_flatWeightNormSquared(Flector self) {
    Flector flat_weight = flector_flatWeight(self);
    return flector_antiDot_flector(flat_weight, flat_weight);
}

AntiScalar line_flatWeightNormSquared(Line self) {
    Line flat_weight = line_flatWeight(self);
    return line_antiDot_line(flat_weight, flat_weight);
}

AntiScalar motor_flatWeightNormSquared(Motor self) {
    Motor flat_weight = motor_flatWeight(self);
    return motor_antiDot_motor(flat_weight, flat_weight);
}

AntiScalar multiVector_flatWeightNormSquared(MultiVector self) {
    MultiVector flat_weight = multiVector_flatWeight(self);
    return multiVector_antiDot_multiVector(flat_weight, flat_weight);
}

AntiScalar plane_flatWeightNormSquared(Plane self) {
    Plane flat_weight = plane_flatWeight(self);
    return plane_antiDot_plane(flat_weight, flat_weight);
}

AntiScalar sphere_flatWeightNormSquared(Sphere self) {
    Sphere flat_weight = sphere_flatWeight(self);
    return sphere_antiDot_sphere(flat_weight, flat_weight);
}

AntiScalar antiScalar_flatWeightNorm(AntiScalar self) {
    return antiScalar_antiSqrt(antiScalar_flatWeightNormSquared(self));
}

AntiScalar circle_flatWeightNorm(Circle self) {
    return antiScalar_antiSqrt(circle_flatWeightNormSquared(self));
}

AntiScalar dipole_flatWeightNorm(Dipole self) {
    return antiScalar_antiSqrt(dipole_flatWeightNormSquared(self));
}

AntiScalar dualNum_flatWeightNorm(DualNum self) {
    return antiScalar_antiSqrt(dualNum_flatWeightNormSquared(self));
}

AntiScalar flatPoint_flatWeightNorm(FlatPoint self) {
    return antiScalar_antiSqrt(flatPoint_flatWeightNormSquared(self));
}

AntiScalar flector_flatWeightNorm(Flector self) {
    return antiScalar_antiSqrt(flector_flatWeightNormSquared(self));
}

AntiScalar line_flatWeightNorm(Line self) {
    return antiScalar_antiSqrt(line_flatWeightNormSquared(self));
}

AntiScalar motor_flatWeightNorm(Motor self) {
    return antiScalar_antiSqrt(motor_flatWeightNormSquared(self));
}

AntiScalar multiVector_flatWeightNorm(MultiVector self) {
    return antiScalar_antiSqrt(multiVector_flatWeightNormSquared(self));
}

AntiScalar plane_flatWeightNorm(Plane self) {
    return antiScalar_antiSqrt(plane_flatWeightNormSquared(self));
}

AntiScalar sphere_flatWeightNorm(Sphere self) {
    return antiScalar_antiSqrt(sphere_flatWeightNormSquared(self));
}

DualNum circle_flatNormSquared(Circle self) {
    return scalar_add_antiScalar(circle_flatBulkNormSquared(self), circle_flatWeightNormSquared(self));
}

DualNum dipole_flatNormSquared(Dipole self) {
    return scalar_add_antiScalar(dipole_flatBulkNormSquared(self), dipole_flatWeightNormSquared(self));
}

DualNum flatPoint_flatNormSquared(FlatPoint self) {
    return scalar_add_antiScalar(flatPoint_flatBulkNormSquared(self), flatPoint_flatWeightNormSquared(self));
}

DualNum flector_flatNormSquared(Flector self) {
    return scalar_add_antiScalar(flector_flatBulkNormSquared(self), flector_flatWeightNormSquared(self));
}

DualNum line_flatNormSquared(Line self) {
    return scalar_add_antiScalar(line_flatBulkNormSquared(self), line_flatWeightNormSquared(self));
}

DualNum motor_flatNormSquared(Motor self) {
    return scalar_add_antiScalar(motor_flatBulkNormSquared(self), motor_flatWeightNormSquared(self));
}

DualNum multiVector_flatNormSquared(MultiVector self) {
    return scalar_add_antiScalar(multiVector_flatBulkNormSquared(self), multiVector_flatWeightNormSquared(self));
}

DualNum plane_flatNormSquared(Plane self) {
    return scalar_add_antiScalar(plane_flatBulkNormSquared(self), plane_flatWeightNormSquared(self));
}

DualNum sphere_flatNormSquared(Sphere self) {
    return scalar_add_antiScalar(sphere_flatBulkNormSquared(self), sphere_flatWeightNormSquared(self));
}

DualNum circle_flatNorm(Circle self) {
    return scalar_add_antiScalar(circle_flatBulkNorm(self), circle_flatWeightNorm(self));
}

DualNum dipole_flatNorm(Dipole self) {
    return scalar_add_antiScalar(dipole_flatBulkNorm(self), dipole_flatWeightNorm(self));
}

DualNum flatPoint_flatNorm(FlatPoint self) {
    return scalar_add_antiScalar(flatPoint_flatBulkNorm(self), flatPoint_flatWeightNorm(self));
}

DualNum flector_flatNorm(Flector self) {
    return scalar_add_antiScalar(flector_flatBulkNorm(self), flector_flatWeightNorm(self));
}

DualNum line_flatNorm(Line self) {
    return scalar_add_antiScalar(line_flatBulkNorm(self), line_flatWeightNorm(self));
}

DualNum motor_flatNorm(Motor self) {
    return scalar_add_antiScalar(motor_flatBulkNorm(self), motor_flatWeightNorm(self));
}

DualNum multiVector_flatNorm(MultiVector self) {
    return scalar_add_antiScalar(multiVector_flatBulkNorm(self), multiVector_flatWeightNorm(self));
}

DualNum plane_flatNorm(Plane self) {
    return scalar_add_antiScalar(plane_flatBulkNorm(self), plane_flatWeightNorm(self));
}

DualNum sphere_flatNorm(Sphere self) {
    return scalar_add_antiScalar(sphere_flatBulkNorm(self), sphere_flatWeightNorm(self));
}

float circle_unitizedFlatNormSquared(Circle self) {
    return circle_flatBulkNormSquared(self).g0_ / circle_flatWeightNormSquared(self).g0_;
}

float dipole_unitizedFlatNormSquared(Dipole self) {
    return dipole_flatBulkNormSquared(self).g0_ / dipole_flatWeightNormSquared(self).g0_;
}

float flatPoint_unitizedFlatNormSquared(FlatPoint self) {
    return flatPoint_flatBulkNormSquared(self).g0_ / flatPoint_flatWeightNormSquared(self).g0_;
}

float flector_unitizedFlatNormSquared(Flector self) {
    return flector_flatBulkNormSquared(self).g0_ / flector_flatWeightNormSquared(self).g0_;
}

float line_unitizedFlatNormSquared(Line self) {
    return line_flatBulkNormSquared(self).g0_ / line_flatWeightNormSquared(self).g0_;
}

float motor_unitizedFlatNormSquared(Motor self) {
    return motor_flatBulkNormSquared(self).g0_ / motor_flatWeightNormSquared(self).g0_;
}

float multiVector_unitizedFlatNormSquared(MultiVector self) {
    return multiVector_flatBulkNormSquared(self).g0_ / multiVector_flatWeightNormSquared(self).g0_;
}

float plane_unitizedFlatNormSquared(Plane self) {
    return plane_flatBulkNormSquared(self).g0_ / plane_flatWeightNormSquared(self).g0_;
}

float sphere_unitizedFlatNormSquared(Sphere self) {
    return sphere_flatBulkNormSquared(self).g0_ / sphere_flatWeightNormSquared(self).g0_;
}

float circle_unitizedFlatNorm(Circle self) {
    return sqrt(circle_unitizedFlatNormSquared(self));
}

float dipole_unitizedFlatNorm(Dipole self) {
    return sqrt(dipole_unitizedFlatNormSquared(self));
}

float flatPoint_unitizedFlatNorm(FlatPoint self) {
    return sqrt(flatPoint_unitizedFlatNormSquared(self));
}

float flector_unitizedFlatNorm(Flector self) {
    return sqrt(flector_unitizedFlatNormSquared(self));
}

float line_unitizedFlatNorm(Line self) {
    return sqrt(line_unitizedFlatNormSquared(self));
}

float motor_unitizedFlatNorm(Motor self) {
    return sqrt(motor_unitizedFlatNormSquared(self));
}

float multiVector_unitizedFlatNorm(MultiVector self) {
    return sqrt(multiVector_unitizedFlatNormSquared(self));
}

float plane_unitizedFlatNorm(Plane self) {
    return sqrt(plane_unitizedFlatNormSquared(self));
}

float sphere_unitizedFlatNorm(Sphere self) {
    return sqrt(sphere_unitizedFlatNormSquared(self));
}

Scalar circle_centerNormSquared(Circle self) {
    return scalar_add_scalar(circle_roundBulkNormSquared(self), antiScalar_antiDual(circle_flatWeightNormSquared(self)));
}

Scalar dipole_centerNormSquared(Dipole self) {
    return scalar_add_scalar(dipole_roundBulkNormSquared(self), antiScalar_antiDual(dipole_flatWeightNormSquared(self)));
}

Scalar dualNum_centerNormSquared(DualNum self) {
    return scalar_add_scalar(dualNum_roundBulkNormSquared(self), antiScalar_antiDual(dualNum_flatWeightNormSquared(self)));
}

Scalar multiVector_centerNormSquared(MultiVector self) {
    return scalar_add_scalar(multiVector_roundBulkNormSquared(self), antiScalar_antiDual(multiVector_flatWeightNormSquared(self)));
}

Scalar circle_centerNorm(Circle self) {
    return scalar_sqrt(circle_centerNormSquared(self));
}

Scalar dipole_centerNorm(Dipole self) {
    return scalar_sqrt(dipole_centerNormSquared(self));
}

Scalar dualNum_centerNorm(DualNum self) {
    return scalar_sqrt(dualNum_centerNormSquared(self));
}

Scalar multiVector_centerNorm(MultiVector self) {
    return scalar_sqrt(multiVector_centerNormSquared(self));
}

float circle_unitizedCenterNormSquared(Circle self) {
    return circle_centerNormSquared(self).g0_ / circle_roundWeightNormSquared(self).g0_;
}

float dipole_unitizedCenterNormSquared(Dipole self) {
    return dipole_centerNormSquared(self).g0_ / dipole_roundWeightNormSquared(self).g0_;
}

float multiVector_unitizedCenterNormSquared(MultiVector self) {
    return multiVector_centerNormSquared(self).g0_ / multiVector_roundWeightNormSquared(self).g0_;
}

float circle_unitizedCenterNorm(Circle self) {
    return sqrt(circle_unitizedCenterNormSquared(self));
}

float dipole_unitizedCenterNorm(Dipole self) {
    return sqrt(dipole_unitizedCenterNormSquared(self));
}

float multiVector_unitizedCenterNorm(MultiVector self) {
    return sqrt(multiVector_unitizedCenterNormSquared(self));
}

Scalar circle_radiusNormSquared(Circle self) {
    return antiScalar_antiDual(circle_antiDot_circle(self, self));
}

Scalar dipole_radiusNormSquared(Dipole self) {
    return antiScalar_antiDual(dipole_antiDot_dipole(self, self));
}

Scalar dualNum_radiusNormSquared(DualNum self) {
    return antiScalar_antiDual(dualNum_antiDot_dualNum(self, self));
}

Scalar multiVector_radiusNormSquared(MultiVector self) {
    return antiScalar_antiDual(multiVector_antiDot_multiVector(self, self));
}

Scalar roundPoint_radiusNormSquared(RoundPoint self) {
    return antiScalar_antiDual(roundPoint_antiDot_roundPoint(self, self));
}

Scalar sphere_radiusNormSquared(Sphere self) {
    return antiScalar_antiDual(sphere_antiDot_sphere(self, self));
}

Scalar circle_radiusNorm(Circle self) {
    return scalar_sqrt(circle_radiusNormSquared(self));
}

Scalar dipole_radiusNorm(Dipole self) {
    return scalar_sqrt(dipole_radiusNormSquared(self));
}

Scalar dualNum_radiusNorm(DualNum self) {
    return scalar_sqrt(dualNum_radiusNormSquared(self));
}

Scalar multiVector_radiusNorm(MultiVector self) {
    return scalar_sqrt(multiVector_radiusNormSquared(self));
}

Scalar roundPoint_radiusNorm(RoundPoint self) {
    return scalar_sqrt(roundPoint_radiusNormSquared(self));
}

Scalar sphere_radiusNorm(Sphere self) {
    return scalar_sqrt(sphere_radiusNormSquared(self));
}

float circle_unitizedRadiusNormSquared(Circle self) {
    return circle_radiusNormSquared(self).g0_ / circle_roundWeightNormSquared(self).g0_;
}

float dipole_unitizedRadiusNormSquared(Dipole self) {
    return dipole_radiusNormSquared(self).g0_ / dipole_roundWeightNormSquared(self).g0_;
}

float multiVector_unitizedRadiusNormSquared(MultiVector self) {
    return multiVector_radiusNormSquared(self).g0_ / multiVector_roundWeightNormSquared(self).g0_;
}

float roundPoint_unitizedRadiusNormSquared(RoundPoint self) {
    return roundPoint_radiusNormSquared(self).g0_ / roundPoint_roundWeightNormSquared(self).g0_;
}

float sphere_unitizedRadiusNormSquared(Sphere self) {
    return sphere_radiusNormSquared(self).g0_ / sphere_roundWeightNormSquared(self).g0_;
}

float circle_unitizedRadiusNorm(Circle self) {
    return sqrt(circle_unitizedRadiusNormSquared(self));
}

float dipole_unitizedRadiusNorm(Dipole self) {
    return sqrt(dipole_unitizedRadiusNormSquared(self));
}

float multiVector_unitizedRadiusNorm(MultiVector self) {
    return sqrt(multiVector_unitizedRadiusNormSquared(self));
}

float roundPoint_unitizedRadiusNorm(RoundPoint self) {
    return sqrt(roundPoint_unitizedRadiusNormSquared(self));
}

float sphere_unitizedRadiusNorm(Sphere self) {
    return sqrt(sphere_unitizedRadiusNormSquared(self));
}

Circle circle_unitize(Circle self) {
    return circle_wedgeDot_scalar(self, Scalar(1.0 / circle_roundWeightNorm(self).g0_));
}

Dipole dipole_unitize(Dipole self) {
    return dipole_wedgeDot_scalar(self, Scalar(1.0 / dipole_roundWeightNorm(self).g0_));
}

MultiVector multiVector_unitize(MultiVector self) {
    return multiVector_wedgeDot_scalar(self, Scalar(1.0 / multiVector_roundWeightNorm(self).g0_));
}

RoundPoint roundPoint_unitize(RoundPoint self) {
    return roundPoint_wedgeDot_scalar(self, Scalar(1.0 / roundPoint_roundWeightNorm(self).g0_));
}

Sphere sphere_unitize(Sphere self) {
    return sphere_wedgeDot_scalar(self, Scalar(1.0 / sphere_roundWeightNorm(self).g0_));
}

Circle antiScalar_sandwich_circle(AntiScalar self, Circle other) {
    return circle_antiWedgeDot_antiScalar(antiScalar_antiWedgeDot_circle(self, other), antiScalar_antiReversal(self));
}

Dipole antiScalar_sandwich_dipole(AntiScalar self, Dipole other) {
    return dipole_antiWedgeDot_antiScalar(antiScalar_antiWedgeDot_dipole(self, other), antiScalar_antiReversal(self));
}

FlatPoint antiScalar_sandwich_flatPoint(AntiScalar self, FlatPoint other) {
    return flatPoint_antiWedgeDot_antiScalar(antiScalar_antiWedgeDot_flatPoint(self, other), antiScalar_antiReversal(self));
}

Flector antiScalar_sandwich_flector(AntiScalar self, Flector other) {
    return flector_antiWedgeDot_antiScalar(antiScalar_antiWedgeDot_flector(self, other), antiScalar_antiReversal(self));
}

Line antiScalar_sandwich_line(AntiScalar self, Line other) {
    return line_antiWedgeDot_antiScalar(antiScalar_antiWedgeDot_line(self, other), antiScalar_antiReversal(self));
}

Motor antiScalar_sandwich_motor(AntiScalar self, Motor other) {
    return motor_antiWedgeDot_antiScalar(antiScalar_antiWedgeDot_motor(self, other), antiScalar_antiReversal(self));
}

MultiVector antiScalar_sandwich_multiVector(AntiScalar self, MultiVector other) {
    return multiVector_antiWedgeDot_antiScalar(antiScalar_antiWedgeDot_multiVector(self, other), antiScalar_antiReversal(self));
}

Plane antiScalar_sandwich_plane(AntiScalar self, Plane other) {
    return plane_antiWedgeDot_antiScalar(antiScalar_antiWedgeDot_plane(self, other), antiScalar_antiReversal(self));
}

RoundPoint antiScalar_sandwich_roundPoint(AntiScalar self, RoundPoint other) {
    return roundPoint_antiWedgeDot_antiScalar(antiScalar_antiWedgeDot_roundPoint(self, other), antiScalar_antiReversal(self));
}

Sphere antiScalar_sandwich_sphere(AntiScalar self, Sphere other) {
    return sphere_antiWedgeDot_antiScalar(antiScalar_antiWedgeDot_sphere(self, other), antiScalar_antiReversal(self));
}

Circle circle_sandwich_circle(Circle self, Circle other) {
    return multiVector_into_circle(multiVector_antiWedgeDot_circle(circle_antiWedgeDot_circle(self, other), circle_antiReversal(self)));
}

Dipole circle_sandwich_dipole(Circle self, Dipole other) {
    return multiVector_into_dipole(multiVector_antiWedgeDot_circle(circle_antiWedgeDot_dipole(self, other), circle_antiReversal(self)));
}

FlatPoint circle_sandwich_flatPoint(Circle self, FlatPoint other) {
    return multiVector_into_flatPoint(multiVector_antiWedgeDot_circle(circle_antiWedgeDot_flatPoint(self, other), circle_antiReversal(self)));
}

Flector circle_sandwich_flector(Circle self, Flector other) {
    return multiVector_into_flector(multiVector_antiWedgeDot_circle(circle_antiWedgeDot_flector(self, other), circle_antiReversal(self)));
}

Line circle_sandwich_line(Circle self, Line other) {
    return multiVector_into_line(multiVector_antiWedgeDot_circle(circle_antiWedgeDot_line(self, other), circle_antiReversal(self)));
}

Motor circle_sandwich_motor(Circle self, Motor other) {
    return multiVector_into_motor(multiVector_antiWedgeDot_circle(circle_antiWedgeDot_motor(self, other), circle_antiReversal(self)));
}

MultiVector circle_sandwich_multiVector(Circle self, MultiVector other) {
    return multiVector_antiWedgeDot_circle(circle_antiWedgeDot_multiVector(self, other), circle_antiReversal(self));
}

Plane circle_sandwich_plane(Circle self, Plane other) {
    return multiVector_into_plane(multiVector_antiWedgeDot_circle(circle_antiWedgeDot_plane(self, other), circle_antiReversal(self)));
}

RoundPoint circle_sandwich_roundPoint(Circle self, RoundPoint other) {
    return multiVector_into_roundPoint(multiVector_antiWedgeDot_circle(circle_antiWedgeDot_roundPoint(self, other), circle_antiReversal(self)));
}

Sphere circle_sandwich_sphere(Circle self, Sphere other) {
    return multiVector_into_sphere(multiVector_antiWedgeDot_circle(circle_antiWedgeDot_sphere(self, other), circle_antiReversal(self)));
}

Circle dipole_sandwich_circle(Dipole self, Circle other) {
    return multiVector_into_circle(multiVector_antiWedgeDot_dipole(dipole_antiWedgeDot_circle(self, other), dipole_antiReversal(self)));
}

Dipole dipole_sandwich_dipole(Dipole self, Dipole other) {
    return multiVector_into_dipole(multiVector_antiWedgeDot_dipole(dipole_antiWedgeDot_dipole(self, other), dipole_antiReversal(self)));
}

FlatPoint dipole_sandwich_flatPoint(Dipole self, FlatPoint other) {
    return multiVector_into_flatPoint(multiVector_antiWedgeDot_dipole(dipole_antiWedgeDot_flatPoint(self, other), dipole_antiReversal(self)));
}

Flector dipole_sandwich_flector(Dipole self, Flector other) {
    return multiVector_into_flector(multiVector_antiWedgeDot_dipole(dipole_antiWedgeDot_flector(self, other), dipole_antiReversal(self)));
}

Line dipole_sandwich_line(Dipole self, Line other) {
    return multiVector_into_line(multiVector_antiWedgeDot_dipole(dipole_antiWedgeDot_line(self, other), dipole_antiReversal(self)));
}

Motor dipole_sandwich_motor(Dipole self, Motor other) {
    return multiVector_into_motor(multiVector_antiWedgeDot_dipole(dipole_antiWedgeDot_motor(self, other), dipole_antiReversal(self)));
}

MultiVector dipole_sandwich_multiVector(Dipole self, MultiVector other) {
    return multiVector_antiWedgeDot_dipole(dipole_antiWedgeDot_multiVector(self, other), dipole_antiReversal(self));
}

Plane dipole_sandwich_plane(Dipole self, Plane other) {
    return multiVector_into_plane(multiVector_antiWedgeDot_dipole(dipole_antiWedgeDot_plane(self, other), dipole_antiReversal(self)));
}

RoundPoint dipole_sandwich_roundPoint(Dipole self, RoundPoint other) {
    return multiVector_into_roundPoint(multiVector_antiWedgeDot_dipole(dipole_antiWedgeDot_roundPoint(self, other), dipole_antiReversal(self)));
}

Sphere dipole_sandwich_sphere(Dipole self, Sphere other) {
    return multiVector_into_sphere(multiVector_antiWedgeDot_dipole(dipole_antiWedgeDot_sphere(self, other), dipole_antiReversal(self)));
}

Circle dualNum_sandwich_circle(DualNum self, Circle other) {
    return multiVector_into_circle(multiVector_antiWedgeDot_dualNum(dualNum_antiWedgeDot_circle(self, other), dualNum_antiReversal(self)));
}

Dipole dualNum_sandwich_dipole(DualNum self, Dipole other) {
    return multiVector_into_dipole(multiVector_antiWedgeDot_dualNum(dualNum_antiWedgeDot_dipole(self, other), dualNum_antiReversal(self)));
}

FlatPoint dualNum_sandwich_flatPoint(DualNum self, FlatPoint other) {
    return multiVector_into_flatPoint(multiVector_antiWedgeDot_dualNum(dualNum_antiWedgeDot_flatPoint(self, other), dualNum_antiReversal(self)));
}

Flector dualNum_sandwich_flector(DualNum self, Flector other) {
    return multiVector_into_flector(multiVector_antiWedgeDot_dualNum(dualNum_antiWedgeDot_flector(self, other), dualNum_antiReversal(self)));
}

Line dualNum_sandwich_line(DualNum self, Line other) {
    return multiVector_into_line(multiVector_antiWedgeDot_dualNum(dualNum_antiWedgeDot_line(self, other), dualNum_antiReversal(self)));
}

Motor dualNum_sandwich_motor(DualNum self, Motor other) {
    return multiVector_into_motor(multiVector_antiWedgeDot_dualNum(dualNum_antiWedgeDot_motor(self, other), dualNum_antiReversal(self)));
}

MultiVector dualNum_sandwich_multiVector(DualNum self, MultiVector other) {
    return multiVector_antiWedgeDot_dualNum(dualNum_antiWedgeDot_multiVector(self, other), dualNum_antiReversal(self));
}

Plane dualNum_sandwich_plane(DualNum self, Plane other) {
    return multiVector_into_plane(multiVector_antiWedgeDot_dualNum(dualNum_antiWedgeDot_plane(self, other), dualNum_antiReversal(self)));
}

RoundPoint dualNum_sandwich_roundPoint(DualNum self, RoundPoint other) {
    return multiVector_into_roundPoint(multiVector_antiWedgeDot_dualNum(dualNum_antiWedgeDot_roundPoint(self, other), dualNum_antiReversal(self)));
}

Sphere dualNum_sandwich_sphere(DualNum self, Sphere other) {
    return multiVector_into_sphere(multiVector_antiWedgeDot_dualNum(dualNum_antiWedgeDot_sphere(self, other), dualNum_antiReversal(self)));
}

Circle flatPoint_sandwich_circle(FlatPoint self, Circle other) {
    return multiVector_into_circle(multiVector_antiWedgeDot_flatPoint(flatPoint_antiWedgeDot_circle(self, other), flatPoint_antiReversal(self)));
}

Dipole flatPoint_sandwich_dipole(FlatPoint self, Dipole other) {
    return multiVector_into_dipole(multiVector_antiWedgeDot_flatPoint(flatPoint_antiWedgeDot_dipole(self, other), flatPoint_antiReversal(self)));
}

FlatPoint flatPoint_sandwich_flatPoint(FlatPoint self, FlatPoint other) {
    return flector_into_flatPoint(motor_antiWedgeDot_flatPoint(flatPoint_antiWedgeDot_flatPoint(self, other), flatPoint_antiReversal(self)));
}

Flector flatPoint_sandwich_flector(FlatPoint self, Flector other) {
    return multiVector_into_flector(multiVector_antiWedgeDot_flatPoint(flatPoint_antiWedgeDot_flector(self, other), flatPoint_antiReversal(self)));
}

Line flatPoint_sandwich_line(FlatPoint self, Line other) {
    return multiVector_into_line(flector_antiWedgeDot_flatPoint(flatPoint_antiWedgeDot_line(self, other), flatPoint_antiReversal(self)));
}

Motor flatPoint_sandwich_motor(FlatPoint self, Motor other) {
    return multiVector_into_motor(flector_antiWedgeDot_flatPoint(flatPoint_antiWedgeDot_motor(self, other), flatPoint_antiReversal(self)));
}

MultiVector flatPoint_sandwich_multiVector(FlatPoint self, MultiVector other) {
    return multiVector_antiWedgeDot_flatPoint(flatPoint_antiWedgeDot_multiVector(self, other), flatPoint_antiReversal(self));
}

Plane flatPoint_sandwich_plane(FlatPoint self, Plane other) {
    return multiVector_into_plane(multiVector_antiWedgeDot_flatPoint(flatPoint_antiWedgeDot_plane(self, other), flatPoint_antiReversal(self)));
}

RoundPoint flatPoint_sandwich_roundPoint(FlatPoint self, RoundPoint other) {
    return multiVector_into_roundPoint(multiVector_antiWedgeDot_flatPoint(flatPoint_antiWedgeDot_roundPoint(self, other), flatPoint_antiReversal(self)));
}

Sphere flatPoint_sandwich_sphere(FlatPoint self, Sphere other) {
    return multiVector_into_sphere(multiVector_antiWedgeDot_flatPoint(flatPoint_antiWedgeDot_sphere(self, other), flatPoint_antiReversal(self)));
}

Circle flector_sandwich_circle(Flector self, Circle other) {
    return multiVector_into_circle(multiVector_antiWedgeDot_flector(flector_antiWedgeDot_circle(self, other), flector_antiReversal(self)));
}

Dipole flector_sandwich_dipole(Flector self, Dipole other) {
    return multiVector_into_dipole(multiVector_antiWedgeDot_flector(flector_antiWedgeDot_dipole(self, other), flector_antiReversal(self)));
}

FlatPoint flector_sandwich_flatPoint(Flector self, FlatPoint other) {
    return multiVector_into_flatPoint(multiVector_antiWedgeDot_flector(flector_antiWedgeDot_flatPoint(self, other), flector_antiReversal(self)));
}

Flector flector_sandwich_flector(Flector self, Flector other) {
    return multiVector_into_flector(multiVector_antiWedgeDot_flector(flector_antiWedgeDot_flector(self, other), flector_antiReversal(self)));
}

Line flector_sandwich_line(Flector self, Line other) {
    return multiVector_into_line(flector_antiWedgeDot_flector(flector_antiWedgeDot_line(self, other), flector_antiReversal(self)));
}

Motor flector_sandwich_motor(Flector self, Motor other) {
    return multiVector_into_motor(flector_antiWedgeDot_flector(flector_antiWedgeDot_motor(self, other), flector_antiReversal(self)));
}

MultiVector flector_sandwich_multiVector(Flector self, MultiVector other) {
    return multiVector_antiWedgeDot_flector(flector_antiWedgeDot_multiVector(self, other), flector_antiReversal(self));
}

Plane flector_sandwich_plane(Flector self, Plane other) {
    return multiVector_into_plane(multiVector_antiWedgeDot_flector(flector_antiWedgeDot_plane(self, other), flector_antiReversal(self)));
}

RoundPoint flector_sandwich_roundPoint(Flector self, RoundPoint other) {
    return multiVector_into_roundPoint(multiVector_antiWedgeDot_flector(flector_antiWedgeDot_roundPoint(self, other), flector_antiReversal(self)));
}

Sphere flector_sandwich_sphere(Flector self, Sphere other) {
    return multiVector_into_sphere(multiVector_antiWedgeDot_flector(flector_antiWedgeDot_sphere(self, other), flector_antiReversal(self)));
}

Circle line_sandwich_circle(Line self, Circle other) {
    return multiVector_into_circle(multiVector_antiWedgeDot_line(line_antiWedgeDot_circle(self, other), line_antiReversal(self)));
}

Dipole line_sandwich_dipole(Line self, Dipole other) {
    return multiVector_into_dipole(multiVector_antiWedgeDot_line(line_antiWedgeDot_dipole(self, other), line_antiReversal(self)));
}

FlatPoint line_sandwich_flatPoint(Line self, FlatPoint other) {
    return flector_into_flatPoint(flector_antiWedgeDot_line(line_antiWedgeDot_flatPoint(self, other), line_antiReversal(self)));
}

Flector line_sandwich_flector(Line self, Flector other) {
    return flector_antiWedgeDot_line(line_antiWedgeDot_flector(self, other), line_antiReversal(self));
}

Line line_sandwich_line(Line self, Line other) {
    return multiVector_into_line(multiVector_antiWedgeDot_line(line_antiWedgeDot_line(self, other), line_antiReversal(self)));
}

Motor line_sandwich_motor(Line self, Motor other) {
    return multiVector_into_motor(multiVector_antiWedgeDot_line(line_antiWedgeDot_motor(self, other), line_antiReversal(self)));
}

MultiVector line_sandwich_multiVector(Line self, MultiVector other) {
    return multiVector_antiWedgeDot_line(line_antiWedgeDot_multiVector(self, other), line_antiReversal(self));
}

Plane line_sandwich_plane(Line self, Plane other) {
    return flector_into_plane(flector_antiWedgeDot_line(line_antiWedgeDot_plane(self, other), line_antiReversal(self)));
}

RoundPoint line_sandwich_roundPoint(Line self, RoundPoint other) {
    return multiVector_into_roundPoint(multiVector_antiWedgeDot_line(line_antiWedgeDot_roundPoint(self, other), line_antiReversal(self)));
}

Sphere line_sandwich_sphere(Line self, Sphere other) {
    return multiVector_into_sphere(multiVector_antiWedgeDot_line(line_antiWedgeDot_sphere(self, other), line_antiReversal(self)));
}

Circle motor_sandwich_circle(Motor self, Circle other) {
    return multiVector_into_circle(multiVector_antiWedgeDot_motor(motor_antiWedgeDot_circle(self, other), motor_antiReversal(self)));
}

Dipole motor_sandwich_dipole(Motor self, Dipole other) {
    return multiVector_into_dipole(multiVector_antiWedgeDot_motor(motor_antiWedgeDot_dipole(self, other), motor_antiReversal(self)));
}

FlatPoint motor_sandwich_flatPoint(Motor self, FlatPoint other) {
    return flector_into_flatPoint(flector_antiWedgeDot_motor(motor_antiWedgeDot_flatPoint(self, other), motor_antiReversal(self)));
}

Flector motor_sandwich_flector(Motor self, Flector other) {
    return flector_antiWedgeDot_motor(motor_antiWedgeDot_flector(self, other), motor_antiReversal(self));
}

Line motor_sandwich_line(Motor self, Line other) {
    return multiVector_into_line(multiVector_antiWedgeDot_motor(motor_antiWedgeDot_line(self, other), motor_antiReversal(self)));
}

Motor motor_sandwich_motor(Motor self, Motor other) {
    return multiVector_into_motor(multiVector_antiWedgeDot_motor(motor_antiWedgeDot_motor(self, other), motor_antiReversal(self)));
}

MultiVector motor_sandwich_multiVector(Motor self, MultiVector other) {
    return multiVector_antiWedgeDot_motor(motor_antiWedgeDot_multiVector(self, other), motor_antiReversal(self));
}

Plane motor_sandwich_plane(Motor self, Plane other) {
    return flector_into_plane(flector_antiWedgeDot_motor(motor_antiWedgeDot_plane(self, other), motor_antiReversal(self)));
}

RoundPoint motor_sandwich_roundPoint(Motor self, RoundPoint other) {
    return multiVector_into_roundPoint(multiVector_antiWedgeDot_motor(motor_antiWedgeDot_roundPoint(self, other), motor_antiReversal(self)));
}

Sphere motor_sandwich_sphere(Motor self, Sphere other) {
    return multiVector_into_sphere(multiVector_antiWedgeDot_motor(motor_antiWedgeDot_sphere(self, other), motor_antiReversal(self)));
}

Circle multiVector_sandwich_circle(MultiVector self, Circle other) {
    return multiVector_into_circle(multiVector_antiWedgeDot_multiVector(multiVector_antiWedgeDot_circle(self, other), multiVector_antiReversal(self)));
}

Dipole multiVector_sandwich_dipole(MultiVector self, Dipole other) {
    return multiVector_into_dipole(multiVector_antiWedgeDot_multiVector(multiVector_antiWedgeDot_dipole(self, other), multiVector_antiReversal(self)));
}

FlatPoint multiVector_sandwich_flatPoint(MultiVector self, FlatPoint other) {
    return multiVector_into_flatPoint(multiVector_antiWedgeDot_multiVector(multiVector_antiWedgeDot_flatPoint(self, other), multiVector_antiReversal(self)));
}

Flector multiVector_sandwich_flector(MultiVector self, Flector other) {
    return multiVector_into_flector(multiVector_antiWedgeDot_multiVector(multiVector_antiWedgeDot_flector(self, other), multiVector_antiReversal(self)));
}

Line multiVector_sandwich_line(MultiVector self, Line other) {
    return multiVector_into_line(multiVector_antiWedgeDot_multiVector(multiVector_antiWedgeDot_line(self, other), multiVector_antiReversal(self)));
}

Motor multiVector_sandwich_motor(MultiVector self, Motor other) {
    return multiVector_into_motor(multiVector_antiWedgeDot_multiVector(multiVector_antiWedgeDot_motor(self, other), multiVector_antiReversal(self)));
}

MultiVector multiVector_sandwich_multiVector(MultiVector self, MultiVector other) {
    return multiVector_antiWedgeDot_multiVector(multiVector_antiWedgeDot_multiVector(self, other), multiVector_antiReversal(self));
}

Plane multiVector_sandwich_plane(MultiVector self, Plane other) {
    return multiVector_into_plane(multiVector_antiWedgeDot_multiVector(multiVector_antiWedgeDot_plane(self, other), multiVector_antiReversal(self)));
}

RoundPoint multiVector_sandwich_roundPoint(MultiVector self, RoundPoint other) {
    return multiVector_into_roundPoint(multiVector_antiWedgeDot_multiVector(multiVector_antiWedgeDot_roundPoint(self, other), multiVector_antiReversal(self)));
}

Sphere multiVector_sandwich_sphere(MultiVector self, Sphere other) {
    return multiVector_into_sphere(multiVector_antiWedgeDot_multiVector(multiVector_antiWedgeDot_sphere(self, other), multiVector_antiReversal(self)));
}

Circle plane_sandwich_circle(Plane self, Circle other) {
    return multiVector_into_circle(multiVector_antiWedgeDot_plane(plane_antiWedgeDot_circle(self, other), plane_antiReversal(self)));
}

Dipole plane_sandwich_dipole(Plane self, Dipole other) {
    return multiVector_into_dipole(multiVector_antiWedgeDot_plane(plane_antiWedgeDot_dipole(self, other), plane_antiReversal(self)));
}

FlatPoint plane_sandwich_flatPoint(Plane self, FlatPoint other) {
    return multiVector_into_flatPoint(multiVector_antiWedgeDot_plane(plane_antiWedgeDot_flatPoint(self, other), plane_antiReversal(self)));
}

Flector plane_sandwich_flector(Plane self, Flector other) {
    return multiVector_into_flector(multiVector_antiWedgeDot_plane(plane_antiWedgeDot_flector(self, other), plane_antiReversal(self)));
}

Line plane_sandwich_line(Plane self, Line other) {
    return multiVector_into_line(flector_antiWedgeDot_plane(plane_antiWedgeDot_line(self, other), plane_antiReversal(self)));
}

Motor plane_sandwich_motor(Plane self, Motor other) {
    return multiVector_into_motor(flector_antiWedgeDot_plane(plane_antiWedgeDot_motor(self, other), plane_antiReversal(self)));
}

MultiVector plane_sandwich_multiVector(Plane self, MultiVector other) {
    return multiVector_antiWedgeDot_plane(plane_antiWedgeDot_multiVector(self, other), plane_antiReversal(self));
}

Plane plane_sandwich_plane(Plane self, Plane other) {
    return flector_into_plane(motor_antiWedgeDot_plane(plane_antiWedgeDot_plane(self, other), plane_antiReversal(self)));
}

RoundPoint plane_sandwich_roundPoint(Plane self, RoundPoint other) {
    return multiVector_into_roundPoint(multiVector_antiWedgeDot_plane(plane_antiWedgeDot_roundPoint(self, other), plane_antiReversal(self)));
}

Sphere plane_sandwich_sphere(Plane self, Sphere other) {
    return multiVector_into_sphere(multiVector_antiWedgeDot_plane(plane_antiWedgeDot_sphere(self, other), plane_antiReversal(self)));
}

Circle roundPoint_sandwich_circle(RoundPoint self, Circle other) {
    return multiVector_into_circle(multiVector_antiWedgeDot_roundPoint(roundPoint_antiWedgeDot_circle(self, other), roundPoint_antiReversal(self)));
}

Dipole roundPoint_sandwich_dipole(RoundPoint self, Dipole other) {
    return multiVector_into_dipole(multiVector_antiWedgeDot_roundPoint(roundPoint_antiWedgeDot_dipole(self, other), roundPoint_antiReversal(self)));
}

FlatPoint roundPoint_sandwich_flatPoint(RoundPoint self, FlatPoint other) {
    return multiVector_into_flatPoint(multiVector_antiWedgeDot_roundPoint(roundPoint_antiWedgeDot_flatPoint(self, other), roundPoint_antiReversal(self)));
}

Flector roundPoint_sandwich_flector(RoundPoint self, Flector other) {
    return multiVector_into_flector(multiVector_antiWedgeDot_roundPoint(roundPoint_antiWedgeDot_flector(self, other), roundPoint_antiReversal(self)));
}

Line roundPoint_sandwich_line(RoundPoint self, Line other) {
    return multiVector_into_line(multiVector_antiWedgeDot_roundPoint(roundPoint_antiWedgeDot_line(self, other), roundPoint_antiReversal(self)));
}

Motor roundPoint_sandwich_motor(RoundPoint self, Motor other) {
    return multiVector_into_motor(multiVector_antiWedgeDot_roundPoint(roundPoint_antiWedgeDot_motor(self, other), roundPoint_antiReversal(self)));
}

MultiVector roundPoint_sandwich_multiVector(RoundPoint self, MultiVector other) {
    return multiVector_antiWedgeDot_roundPoint(roundPoint_antiWedgeDot_multiVector(self, other), roundPoint_antiReversal(self));
}

Plane roundPoint_sandwich_plane(RoundPoint self, Plane other) {
    return multiVector_into_plane(multiVector_antiWedgeDot_roundPoint(roundPoint_antiWedgeDot_plane(self, other), roundPoint_antiReversal(self)));
}

RoundPoint roundPoint_sandwich_roundPoint(RoundPoint self, RoundPoint other) {
    return multiVector_into_roundPoint(multiVector_antiWedgeDot_roundPoint(roundPoint_antiWedgeDot_roundPoint(self, other), roundPoint_antiReversal(self)));
}

Sphere roundPoint_sandwich_sphere(RoundPoint self, Sphere other) {
    return multiVector_into_sphere(multiVector_antiWedgeDot_roundPoint(roundPoint_antiWedgeDot_sphere(self, other), roundPoint_antiReversal(self)));
}

Circle scalar_sandwich_circle(Scalar self, Circle other) {
    return dipole_antiWedgeDot_scalar(scalar_antiWedgeDot_circle(self, other), scalar_antiReversal(self));
}

Dipole scalar_sandwich_dipole(Scalar self, Dipole other) {
    return circle_antiWedgeDot_scalar(scalar_antiWedgeDot_dipole(self, other), scalar_antiReversal(self));
}

FlatPoint scalar_sandwich_flatPoint(Scalar self, FlatPoint other) {
    return dipole_into_flatPoint(circle_antiWedgeDot_scalar(scalar_antiWedgeDot_flatPoint(self, other), scalar_antiReversal(self)));
}

Flector scalar_sandwich_flector(Scalar self, Flector other) {
    return multiVector_into_flector(multiVector_antiWedgeDot_scalar(scalar_antiWedgeDot_flector(self, other), scalar_antiReversal(self)));
}

Line scalar_sandwich_line(Scalar self, Line other) {
    return circle_into_line(dipole_antiWedgeDot_scalar(scalar_antiWedgeDot_line(self, other), scalar_antiReversal(self)));
}

Motor scalar_sandwich_motor(Scalar self, Motor other) {
    return multiVector_into_motor(multiVector_antiWedgeDot_scalar(scalar_antiWedgeDot_motor(self, other), scalar_antiReversal(self)));
}

MultiVector scalar_sandwich_multiVector(Scalar self, MultiVector other) {
    return multiVector_antiWedgeDot_scalar(scalar_antiWedgeDot_multiVector(self, other), scalar_antiReversal(self));
}

Plane scalar_sandwich_plane(Scalar self, Plane other) {
    return sphere_into_plane(roundPoint_antiWedgeDot_scalar(scalar_antiWedgeDot_plane(self, other), scalar_antiReversal(self)));
}

RoundPoint scalar_sandwich_roundPoint(Scalar self, RoundPoint other) {
    return sphere_antiWedgeDot_scalar(scalar_antiWedgeDot_roundPoint(self, other), scalar_antiReversal(self));
}

Sphere scalar_sandwich_sphere(Scalar self, Sphere other) {
    return roundPoint_antiWedgeDot_scalar(scalar_antiWedgeDot_sphere(self, other), scalar_antiReversal(self));
}

Circle sphere_sandwich_circle(Sphere self, Circle other) {
    return multiVector_into_circle(multiVector_antiWedgeDot_sphere(sphere_antiWedgeDot_circle(self, other), sphere_antiReversal(self)));
}

Dipole sphere_sandwich_dipole(Sphere self, Dipole other) {
    return multiVector_into_dipole(multiVector_antiWedgeDot_sphere(sphere_antiWedgeDot_dipole(self, other), sphere_antiReversal(self)));
}

FlatPoint sphere_sandwich_flatPoint(Sphere self, FlatPoint other) {
    return multiVector_into_flatPoint(multiVector_antiWedgeDot_sphere(sphere_antiWedgeDot_flatPoint(self, other), sphere_antiReversal(self)));
}

Flector sphere_sandwich_flector(Sphere self, Flector other) {
    return multiVector_into_flector(multiVector_antiWedgeDot_sphere(sphere_antiWedgeDot_flector(self, other), sphere_antiReversal(self)));
}

Line sphere_sandwich_line(Sphere self, Line other) {
    return multiVector_into_line(multiVector_antiWedgeDot_sphere(sphere_antiWedgeDot_line(self, other), sphere_antiReversal(self)));
}

Motor sphere_sandwich_motor(Sphere self, Motor other) {
    return multiVector_into_motor(multiVector_antiWedgeDot_sphere(sphere_antiWedgeDot_motor(self, other), sphere_antiReversal(self)));
}

MultiVector sphere_sandwich_multiVector(Sphere self, MultiVector other) {
    return multiVector_antiWedgeDot_sphere(sphere_antiWedgeDot_multiVector(self, other), sphere_antiReversal(self));
}

Plane sphere_sandwich_plane(Sphere self, Plane other) {
    return multiVector_into_plane(multiVector_antiWedgeDot_sphere(sphere_antiWedgeDot_plane(self, other), sphere_antiReversal(self)));
}

RoundPoint sphere_sandwich_roundPoint(Sphere self, RoundPoint other) {
    return multiVector_into_roundPoint(multiVector_antiWedgeDot_sphere(sphere_antiWedgeDot_roundPoint(self, other), sphere_antiReversal(self)));
}

Sphere sphere_sandwich_sphere(Sphere self, Sphere other) {
    return multiVector_into_sphere(multiVector_antiWedgeDot_sphere(sphere_antiWedgeDot_sphere(self, other), sphere_antiReversal(self)));
}

AntiScalar antiScalar_geometricAntiQuotient_antiScalar(AntiScalar self, AntiScalar other) {
    return antiScalar_antiWedgeDot_antiScalar(self, antiScalar_antiInverse(other));
}

Circle antiScalar_geometricAntiQuotient_circle(AntiScalar self, Circle other) {
    return antiScalar_antiWedgeDot_circle(self, circle_antiInverse(other));
}

Dipole antiScalar_geometricAntiQuotient_dipole(AntiScalar self, Dipole other) {
    return antiScalar_antiWedgeDot_dipole(self, dipole_antiInverse(other));
}

DualNum antiScalar_geometricAntiQuotient_dualNum(AntiScalar self, DualNum other) {
    return antiScalar_antiWedgeDot_dualNum(self, dualNum_antiInverse(other));
}

FlatPoint antiScalar_geometricAntiQuotient_flatPoint(AntiScalar self, FlatPoint other) {
    return antiScalar_antiWedgeDot_flatPoint(self, flatPoint_antiInverse(other));
}

Flector antiScalar_geometricAntiQuotient_flector(AntiScalar self, Flector other) {
    return antiScalar_antiWedgeDot_flector(self, flector_antiInverse(other));
}

Line antiScalar_geometricAntiQuotient_line(AntiScalar self, Line other) {
    return antiScalar_antiWedgeDot_line(self, line_antiInverse(other));
}

Motor antiScalar_geometricAntiQuotient_motor(AntiScalar self, Motor other) {
    return antiScalar_antiWedgeDot_motor(self, motor_antiInverse(other));
}

MultiVector antiScalar_geometricAntiQuotient_multiVector(AntiScalar self, MultiVector other) {
    return antiScalar_antiWedgeDot_multiVector(self, multiVector_antiInverse(other));
}

Plane antiScalar_geometricAntiQuotient_plane(AntiScalar self, Plane other) {
    return antiScalar_antiWedgeDot_plane(self, plane_antiInverse(other));
}

RoundPoint antiScalar_geometricAntiQuotient_roundPoint(AntiScalar self, RoundPoint other) {
    return antiScalar_antiWedgeDot_roundPoint(self, roundPoint_antiInverse(other));
}

Scalar antiScalar_geometricAntiQuotient_scalar(AntiScalar self, Scalar other) {
    return antiScalar_antiWedgeDot_scalar(self, scalar_antiInverse(other));
}

Sphere antiScalar_geometricAntiQuotient_sphere(AntiScalar self, Sphere other) {
    return antiScalar_antiWedgeDot_sphere(self, sphere_antiInverse(other));
}

Circle circle_geometricAntiQuotient_antiScalar(Circle self, AntiScalar other) {
    return circle_antiWedgeDot_antiScalar(self, antiScalar_antiInverse(other));
}

MultiVector circle_geometricAntiQuotient_circle(Circle self, Circle other) {
    return circle_antiWedgeDot_circle(self, circle_antiInverse(other));
}

MultiVector circle_geometricAntiQuotient_dipole(Circle self, Dipole other) {
    return circle_antiWedgeDot_dipole(self, dipole_antiInverse(other));
}

MultiVector circle_geometricAntiQuotient_dualNum(Circle self, DualNum other) {
    return circle_antiWedgeDot_dualNum(self, dualNum_antiInverse(other));
}

MultiVector circle_geometricAntiQuotient_flatPoint(Circle self, FlatPoint other) {
    return circle_antiWedgeDot_flatPoint(self, flatPoint_antiInverse(other));
}

MultiVector circle_geometricAntiQuotient_flector(Circle self, Flector other) {
    return circle_antiWedgeDot_flector(self, flector_antiInverse(other));
}

MultiVector circle_geometricAntiQuotient_line(Circle self, Line other) {
    return circle_antiWedgeDot_line(self, line_antiInverse(other));
}

MultiVector circle_geometricAntiQuotient_motor(Circle self, Motor other) {
    return circle_antiWedgeDot_motor(self, motor_antiInverse(other));
}

MultiVector circle_geometricAntiQuotient_multiVector(Circle self, MultiVector other) {
    return circle_antiWedgeDot_multiVector(self, multiVector_antiInverse(other));
}

MultiVector circle_geometricAntiQuotient_plane(Circle self, Plane other) {
    return circle_antiWedgeDot_plane(self, plane_antiInverse(other));
}

MultiVector circle_geometricAntiQuotient_roundPoint(Circle self, RoundPoint other) {
    return circle_antiWedgeDot_roundPoint(self, roundPoint_antiInverse(other));
}

Dipole circle_geometricAntiQuotient_scalar(Circle self, Scalar other) {
    return circle_antiWedgeDot_scalar(self, scalar_antiInverse(other));
}

MultiVector circle_geometricAntiQuotient_sphere(Circle self, Sphere other) {
    return circle_antiWedgeDot_sphere(self, sphere_antiInverse(other));
}

Dipole dipole_geometricAntiQuotient_antiScalar(Dipole self, AntiScalar other) {
    return dipole_antiWedgeDot_antiScalar(self, antiScalar_antiInverse(other));
}

MultiVector dipole_geometricAntiQuotient_circle(Dipole self, Circle other) {
    return dipole_antiWedgeDot_circle(self, circle_antiInverse(other));
}

MultiVector dipole_geometricAntiQuotient_dipole(Dipole self, Dipole other) {
    return dipole_antiWedgeDot_dipole(self, dipole_antiInverse(other));
}

MultiVector dipole_geometricAntiQuotient_dualNum(Dipole self, DualNum other) {
    return dipole_antiWedgeDot_dualNum(self, dualNum_antiInverse(other));
}

MultiVector dipole_geometricAntiQuotient_flatPoint(Dipole self, FlatPoint other) {
    return dipole_antiWedgeDot_flatPoint(self, flatPoint_antiInverse(other));
}

MultiVector dipole_geometricAntiQuotient_flector(Dipole self, Flector other) {
    return dipole_antiWedgeDot_flector(self, flector_antiInverse(other));
}

MultiVector dipole_geometricAntiQuotient_line(Dipole self, Line other) {
    return dipole_antiWedgeDot_line(self, line_antiInverse(other));
}

MultiVector dipole_geometricAntiQuotient_motor(Dipole self, Motor other) {
    return dipole_antiWedgeDot_motor(self, motor_antiInverse(other));
}

MultiVector dipole_geometricAntiQuotient_multiVector(Dipole self, MultiVector other) {
    return dipole_antiWedgeDot_multiVector(self, multiVector_antiInverse(other));
}

MultiVector dipole_geometricAntiQuotient_plane(Dipole self, Plane other) {
    return dipole_antiWedgeDot_plane(self, plane_antiInverse(other));
}

MultiVector dipole_geometricAntiQuotient_roundPoint(Dipole self, RoundPoint other) {
    return dipole_antiWedgeDot_roundPoint(self, roundPoint_antiInverse(other));
}

Circle dipole_geometricAntiQuotient_scalar(Dipole self, Scalar other) {
    return dipole_antiWedgeDot_scalar(self, scalar_antiInverse(other));
}

MultiVector dipole_geometricAntiQuotient_sphere(Dipole self, Sphere other) {
    return dipole_antiWedgeDot_sphere(self, sphere_antiInverse(other));
}

DualNum dualNum_geometricAntiQuotient_antiScalar(DualNum self, AntiScalar other) {
    return dualNum_antiWedgeDot_antiScalar(self, antiScalar_antiInverse(other));
}

MultiVector dualNum_geometricAntiQuotient_circle(DualNum self, Circle other) {
    return dualNum_antiWedgeDot_circle(self, circle_antiInverse(other));
}

MultiVector dualNum_geometricAntiQuotient_dipole(DualNum self, Dipole other) {
    return dualNum_antiWedgeDot_dipole(self, dipole_antiInverse(other));
}

DualNum dualNum_geometricAntiQuotient_dualNum(DualNum self, DualNum other) {
    return dualNum_antiWedgeDot_dualNum(self, dualNum_antiInverse(other));
}

MultiVector dualNum_geometricAntiQuotient_flatPoint(DualNum self, FlatPoint other) {
    return dualNum_antiWedgeDot_flatPoint(self, flatPoint_antiInverse(other));
}

MultiVector dualNum_geometricAntiQuotient_flector(DualNum self, Flector other) {
    return dualNum_antiWedgeDot_flector(self, flector_antiInverse(other));
}

MultiVector dualNum_geometricAntiQuotient_line(DualNum self, Line other) {
    return dualNum_antiWedgeDot_line(self, line_antiInverse(other));
}

MultiVector dualNum_geometricAntiQuotient_motor(DualNum self, Motor other) {
    return dualNum_antiWedgeDot_motor(self, motor_antiInverse(other));
}

MultiVector dualNum_geometricAntiQuotient_multiVector(DualNum self, MultiVector other) {
    return dualNum_antiWedgeDot_multiVector(self, multiVector_antiInverse(other));
}

MultiVector dualNum_geometricAntiQuotient_plane(DualNum self, Plane other) {
    return dualNum_antiWedgeDot_plane(self, plane_antiInverse(other));
}

MultiVector dualNum_geometricAntiQuotient_roundPoint(DualNum self, RoundPoint other) {
    return dualNum_antiWedgeDot_roundPoint(self, roundPoint_antiInverse(other));
}

DualNum dualNum_geometricAntiQuotient_scalar(DualNum self, Scalar other) {
    return dualNum_antiWedgeDot_scalar(self, scalar_antiInverse(other));
}

MultiVector dualNum_geometricAntiQuotient_sphere(DualNum self, Sphere other) {
    return dualNum_antiWedgeDot_sphere(self, sphere_antiInverse(other));
}

FlatPoint flatPoint_geometricAntiQuotient_antiScalar(FlatPoint self, AntiScalar other) {
    return flatPoint_antiWedgeDot_antiScalar(self, antiScalar_antiInverse(other));
}

MultiVector flatPoint_geometricAntiQuotient_circle(FlatPoint self, Circle other) {
    return flatPoint_antiWedgeDot_circle(self, circle_antiInverse(other));
}

MultiVector flatPoint_geometricAntiQuotient_dipole(FlatPoint self, Dipole other) {
    return flatPoint_antiWedgeDot_dipole(self, dipole_antiInverse(other));
}

MultiVector flatPoint_geometricAntiQuotient_dualNum(FlatPoint self, DualNum other) {
    return flatPoint_antiWedgeDot_dualNum(self, dualNum_antiInverse(other));
}

Motor flatPoint_geometricAntiQuotient_flatPoint(FlatPoint self, FlatPoint other) {
    return flatPoint_antiWedgeDot_flatPoint(self, flatPoint_antiInverse(other));
}

MultiVector flatPoint_geometricAntiQuotient_flector(FlatPoint self, Flector other) {
    return flatPoint_antiWedgeDot_flector(self, flector_antiInverse(other));
}

Flector flatPoint_geometricAntiQuotient_line(FlatPoint self, Line other) {
    return flatPoint_antiWedgeDot_line(self, line_antiInverse(other));
}

Flector flatPoint_geometricAntiQuotient_motor(FlatPoint self, Motor other) {
    return flatPoint_antiWedgeDot_motor(self, motor_antiInverse(other));
}

MultiVector flatPoint_geometricAntiQuotient_multiVector(FlatPoint self, MultiVector other) {
    return flatPoint_antiWedgeDot_multiVector(self, multiVector_antiInverse(other));
}

MultiVector flatPoint_geometricAntiQuotient_plane(FlatPoint self, Plane other) {
    return flatPoint_antiWedgeDot_plane(self, plane_antiInverse(other));
}

MultiVector flatPoint_geometricAntiQuotient_roundPoint(FlatPoint self, RoundPoint other) {
    return flatPoint_antiWedgeDot_roundPoint(self, roundPoint_antiInverse(other));
}

Circle flatPoint_geometricAntiQuotient_scalar(FlatPoint self, Scalar other) {
    return flatPoint_antiWedgeDot_scalar(self, scalar_antiInverse(other));
}

MultiVector flatPoint_geometricAntiQuotient_sphere(FlatPoint self, Sphere other) {
    return flatPoint_antiWedgeDot_sphere(self, sphere_antiInverse(other));
}

Flector flector_geometricAntiQuotient_antiScalar(Flector self, AntiScalar other) {
    return flector_antiWedgeDot_antiScalar(self, antiScalar_antiInverse(other));
}

MultiVector flector_geometricAntiQuotient_circle(Flector self, Circle other) {
    return flector_antiWedgeDot_circle(self, circle_antiInverse(other));
}

MultiVector flector_geometricAntiQuotient_dipole(Flector self, Dipole other) {
    return flector_antiWedgeDot_dipole(self, dipole_antiInverse(other));
}

MultiVector flector_geometricAntiQuotient_dualNum(Flector self, DualNum other) {
    return flector_antiWedgeDot_dualNum(self, dualNum_antiInverse(other));
}

MultiVector flector_geometricAntiQuotient_flatPoint(Flector self, FlatPoint other) {
    return flector_antiWedgeDot_flatPoint(self, flatPoint_antiInverse(other));
}

MultiVector flector_geometricAntiQuotient_flector(Flector self, Flector other) {
    return flector_antiWedgeDot_flector(self, flector_antiInverse(other));
}

Flector flector_geometricAntiQuotient_line(Flector self, Line other) {
    return flector_antiWedgeDot_line(self, line_antiInverse(other));
}

Flector flector_geometricAntiQuotient_motor(Flector self, Motor other) {
    return flector_antiWedgeDot_motor(self, motor_antiInverse(other));
}

MultiVector flector_geometricAntiQuotient_multiVector(Flector self, MultiVector other) {
    return flector_antiWedgeDot_multiVector(self, multiVector_antiInverse(other));
}

MultiVector flector_geometricAntiQuotient_plane(Flector self, Plane other) {
    return flector_antiWedgeDot_plane(self, plane_antiInverse(other));
}

MultiVector flector_geometricAntiQuotient_roundPoint(Flector self, RoundPoint other) {
    return flector_antiWedgeDot_roundPoint(self, roundPoint_antiInverse(other));
}

MultiVector flector_geometricAntiQuotient_scalar(Flector self, Scalar other) {
    return flector_antiWedgeDot_scalar(self, scalar_antiInverse(other));
}

MultiVector flector_geometricAntiQuotient_sphere(Flector self, Sphere other) {
    return flector_antiWedgeDot_sphere(self, sphere_antiInverse(other));
}

Line line_geometricAntiQuotient_antiScalar(Line self, AntiScalar other) {
    return line_antiWedgeDot_antiScalar(self, antiScalar_antiInverse(other));
}

MultiVector line_geometricAntiQuotient_circle(Line self, Circle other) {
    return line_antiWedgeDot_circle(self, circle_antiInverse(other));
}

MultiVector line_geometricAntiQuotient_dipole(Line self, Dipole other) {
    return line_antiWedgeDot_dipole(self, dipole_antiInverse(other));
}

MultiVector line_geometricAntiQuotient_dualNum(Line self, DualNum other) {
    return line_antiWedgeDot_dualNum(self, dualNum_antiInverse(other));
}

Flector line_geometricAntiQuotient_flatPoint(Line self, FlatPoint other) {
    return line_antiWedgeDot_flatPoint(self, flatPoint_antiInverse(other));
}

Flector line_geometricAntiQuotient_flector(Line self, Flector other) {
    return line_antiWedgeDot_flector(self, flector_antiInverse(other));
}

MultiVector line_geometricAntiQuotient_line(Line self, Line other) {
    return line_antiWedgeDot_line(self, line_antiInverse(other));
}

MultiVector line_geometricAntiQuotient_motor(Line self, Motor other) {
    return line_antiWedgeDot_motor(self, motor_antiInverse(other));
}

MultiVector line_geometricAntiQuotient_multiVector(Line self, MultiVector other) {
    return line_antiWedgeDot_multiVector(self, multiVector_antiInverse(other));
}

Flector line_geometricAntiQuotient_plane(Line self, Plane other) {
    return line_antiWedgeDot_plane(self, plane_antiInverse(other));
}

MultiVector line_geometricAntiQuotient_roundPoint(Line self, RoundPoint other) {
    return line_antiWedgeDot_roundPoint(self, roundPoint_antiInverse(other));
}

Dipole line_geometricAntiQuotient_scalar(Line self, Scalar other) {
    return line_antiWedgeDot_scalar(self, scalar_antiInverse(other));
}

MultiVector line_geometricAntiQuotient_sphere(Line self, Sphere other) {
    return line_antiWedgeDot_sphere(self, sphere_antiInverse(other));
}

Motor motor_geometricAntiQuotient_antiScalar(Motor self, AntiScalar other) {
    return motor_antiWedgeDot_antiScalar(self, antiScalar_antiInverse(other));
}

MultiVector motor_geometricAntiQuotient_circle(Motor self, Circle other) {
    return motor_antiWedgeDot_circle(self, circle_antiInverse(other));
}

MultiVector motor_geometricAntiQuotient_dipole(Motor self, Dipole other) {
    return motor_antiWedgeDot_dipole(self, dipole_antiInverse(other));
}

MultiVector motor_geometricAntiQuotient_dualNum(Motor self, DualNum other) {
    return motor_antiWedgeDot_dualNum(self, dualNum_antiInverse(other));
}

Flector motor_geometricAntiQuotient_flatPoint(Motor self, FlatPoint other) {
    return motor_antiWedgeDot_flatPoint(self, flatPoint_antiInverse(other));
}

Flector motor_geometricAntiQuotient_flector(Motor self, Flector other) {
    return motor_antiWedgeDot_flector(self, flector_antiInverse(other));
}

MultiVector motor_geometricAntiQuotient_line(Motor self, Line other) {
    return motor_antiWedgeDot_line(self, line_antiInverse(other));
}

MultiVector motor_geometricAntiQuotient_motor(Motor self, Motor other) {
    return motor_antiWedgeDot_motor(self, motor_antiInverse(other));
}

MultiVector motor_geometricAntiQuotient_multiVector(Motor self, MultiVector other) {
    return motor_antiWedgeDot_multiVector(self, multiVector_antiInverse(other));
}

Flector motor_geometricAntiQuotient_plane(Motor self, Plane other) {
    return motor_antiWedgeDot_plane(self, plane_antiInverse(other));
}

MultiVector motor_geometricAntiQuotient_roundPoint(Motor self, RoundPoint other) {
    return motor_antiWedgeDot_roundPoint(self, roundPoint_antiInverse(other));
}

MultiVector motor_geometricAntiQuotient_scalar(Motor self, Scalar other) {
    return motor_antiWedgeDot_scalar(self, scalar_antiInverse(other));
}

MultiVector motor_geometricAntiQuotient_sphere(Motor self, Sphere other) {
    return motor_antiWedgeDot_sphere(self, sphere_antiInverse(other));
}

MultiVector multiVector_geometricAntiQuotient_antiScalar(MultiVector self, AntiScalar other) {
    return multiVector_antiWedgeDot_antiScalar(self, antiScalar_antiInverse(other));
}

MultiVector multiVector_geometricAntiQuotient_circle(MultiVector self, Circle other) {
    return multiVector_antiWedgeDot_circle(self, circle_antiInverse(other));
}

MultiVector multiVector_geometricAntiQuotient_dipole(MultiVector self, Dipole other) {
    return multiVector_antiWedgeDot_dipole(self, dipole_antiInverse(other));
}

MultiVector multiVector_geometricAntiQuotient_dualNum(MultiVector self, DualNum other) {
    return multiVector_antiWedgeDot_dualNum(self, dualNum_antiInverse(other));
}

MultiVector multiVector_geometricAntiQuotient_flatPoint(MultiVector self, FlatPoint other) {
    return multiVector_antiWedgeDot_flatPoint(self, flatPoint_antiInverse(other));
}

MultiVector multiVector_geometricAntiQuotient_flector(MultiVector self, Flector other) {
    return multiVector_antiWedgeDot_flector(self, flector_antiInverse(other));
}

MultiVector multiVector_geometricAntiQuotient_line(MultiVector self, Line other) {
    return multiVector_antiWedgeDot_line(self, line_antiInverse(other));
}

MultiVector multiVector_geometricAntiQuotient_motor(MultiVector self, Motor other) {
    return multiVector_antiWedgeDot_motor(self, motor_antiInverse(other));
}

MultiVector multiVector_geometricAntiQuotient_multiVector(MultiVector self, MultiVector other) {
    return multiVector_antiWedgeDot_multiVector(self, multiVector_antiInverse(other));
}

MultiVector multiVector_geometricAntiQuotient_plane(MultiVector self, Plane other) {
    return multiVector_antiWedgeDot_plane(self, plane_antiInverse(other));
}

MultiVector multiVector_geometricAntiQuotient_roundPoint(MultiVector self, RoundPoint other) {
    return multiVector_antiWedgeDot_roundPoint(self, roundPoint_antiInverse(other));
}

MultiVector multiVector_geometricAntiQuotient_scalar(MultiVector self, Scalar other) {
    return multiVector_antiWedgeDot_scalar(self, scalar_antiInverse(other));
}

MultiVector multiVector_geometricAntiQuotient_sphere(MultiVector self, Sphere other) {
    return multiVector_antiWedgeDot_sphere(self, sphere_antiInverse(other));
}

Plane plane_geometricAntiQuotient_antiScalar(Plane self, AntiScalar other) {
    return plane_antiWedgeDot_antiScalar(self, antiScalar_antiInverse(other));
}

MultiVector plane_geometricAntiQuotient_circle(Plane self, Circle other) {
    return plane_antiWedgeDot_circle(self, circle_antiInverse(other));
}

MultiVector plane_geometricAntiQuotient_dipole(Plane self, Dipole other) {
    return plane_antiWedgeDot_dipole(self, dipole_antiInverse(other));
}

MultiVector plane_geometricAntiQuotient_dualNum(Plane self, DualNum other) {
    return plane_antiWedgeDot_dualNum(self, dualNum_antiInverse(other));
}

MultiVector plane_geometricAntiQuotient_flatPoint(Plane self, FlatPoint other) {
    return plane_antiWedgeDot_flatPoint(self, flatPoint_antiInverse(other));
}

MultiVector plane_geometricAntiQuotient_flector(Plane self, Flector other) {
    return plane_antiWedgeDot_flector(self, flector_antiInverse(other));
}

Flector plane_geometricAntiQuotient_line(Plane self, Line other) {
    return plane_antiWedgeDot_line(self, line_antiInverse(other));
}

Flector plane_geometricAntiQuotient_motor(Plane self, Motor other) {
    return plane_antiWedgeDot_motor(self, motor_antiInverse(other));
}

MultiVector plane_geometricAntiQuotient_multiVector(Plane self, MultiVector other) {
    return plane_antiWedgeDot_multiVector(self, multiVector_antiInverse(other));
}

Motor plane_geometricAntiQuotient_plane(Plane self, Plane other) {
    return plane_antiWedgeDot_plane(self, plane_antiInverse(other));
}

MultiVector plane_geometricAntiQuotient_roundPoint(Plane self, RoundPoint other) {
    return plane_antiWedgeDot_roundPoint(self, roundPoint_antiInverse(other));
}

RoundPoint plane_geometricAntiQuotient_scalar(Plane self, Scalar other) {
    return plane_antiWedgeDot_scalar(self, scalar_antiInverse(other));
}

MultiVector plane_geometricAntiQuotient_sphere(Plane self, Sphere other) {
    return plane_antiWedgeDot_sphere(self, sphere_antiInverse(other));
}

RoundPoint roundPoint_geometricAntiQuotient_antiScalar(RoundPoint self, AntiScalar other) {
    return roundPoint_antiWedgeDot_antiScalar(self, antiScalar_antiInverse(other));
}

MultiVector roundPoint_geometricAntiQuotient_circle(RoundPoint self, Circle other) {
    return roundPoint_antiWedgeDot_circle(self, circle_antiInverse(other));
}

MultiVector roundPoint_geometricAntiQuotient_dipole(RoundPoint self, Dipole other) {
    return roundPoint_antiWedgeDot_dipole(self, dipole_antiInverse(other));
}

MultiVector roundPoint_geometricAntiQuotient_dualNum(RoundPoint self, DualNum other) {
    return roundPoint_antiWedgeDot_dualNum(self, dualNum_antiInverse(other));
}

MultiVector roundPoint_geometricAntiQuotient_flatPoint(RoundPoint self, FlatPoint other) {
    return roundPoint_antiWedgeDot_flatPoint(self, flatPoint_antiInverse(other));
}

MultiVector roundPoint_geometricAntiQuotient_flector(RoundPoint self, Flector other) {
    return roundPoint_antiWedgeDot_flector(self, flector_antiInverse(other));
}

MultiVector roundPoint_geometricAntiQuotient_line(RoundPoint self, Line other) {
    return roundPoint_antiWedgeDot_line(self, line_antiInverse(other));
}

MultiVector roundPoint_geometricAntiQuotient_motor(RoundPoint self, Motor other) {
    return roundPoint_antiWedgeDot_motor(self, motor_antiInverse(other));
}

MultiVector roundPoint_geometricAntiQuotient_multiVector(RoundPoint self, MultiVector other) {
    return roundPoint_antiWedgeDot_multiVector(self, multiVector_antiInverse(other));
}

MultiVector roundPoint_geometricAntiQuotient_plane(RoundPoint self, Plane other) {
    return roundPoint_antiWedgeDot_plane(self, plane_antiInverse(other));
}

MultiVector roundPoint_geometricAntiQuotient_roundPoint(RoundPoint self, RoundPoint other) {
    return roundPoint_antiWedgeDot_roundPoint(self, roundPoint_antiInverse(other));
}

Sphere roundPoint_geometricAntiQuotient_scalar(RoundPoint self, Scalar other) {
    return roundPoint_antiWedgeDot_scalar(self, scalar_antiInverse(other));
}

MultiVector roundPoint_geometricAntiQuotient_sphere(RoundPoint self, Sphere other) {
    return roundPoint_antiWedgeDot_sphere(self, sphere_antiInverse(other));
}

Scalar scalar_geometricAntiQuotient_antiScalar(Scalar self, AntiScalar other) {
    return scalar_antiWedgeDot_antiScalar(self, antiScalar_antiInverse(other));
}

Dipole scalar_geometricAntiQuotient_circle(Scalar self, Circle other) {
    return scalar_antiWedgeDot_circle(self, circle_antiInverse(other));
}

Circle scalar_geometricAntiQuotient_dipole(Scalar self, Dipole other) {
    return scalar_antiWedgeDot_dipole(self, dipole_antiInverse(other));
}

DualNum scalar_geometricAntiQuotient_dualNum(Scalar self, DualNum other) {
    return scalar_antiWedgeDot_dualNum(self, dualNum_antiInverse(other));
}

Circle scalar_geometricAntiQuotient_flatPoint(Scalar self, FlatPoint other) {
    return scalar_antiWedgeDot_flatPoint(self, flatPoint_antiInverse(other));
}

MultiVector scalar_geometricAntiQuotient_flector(Scalar self, Flector other) {
    return scalar_antiWedgeDot_flector(self, flector_antiInverse(other));
}

Dipole scalar_geometricAntiQuotient_line(Scalar self, Line other) {
    return scalar_antiWedgeDot_line(self, line_antiInverse(other));
}

MultiVector scalar_geometricAntiQuotient_motor(Scalar self, Motor other) {
    return scalar_antiWedgeDot_motor(self, motor_antiInverse(other));
}

MultiVector scalar_geometricAntiQuotient_multiVector(Scalar self, MultiVector other) {
    return scalar_antiWedgeDot_multiVector(self, multiVector_antiInverse(other));
}

RoundPoint scalar_geometricAntiQuotient_plane(Scalar self, Plane other) {
    return scalar_antiWedgeDot_plane(self, plane_antiInverse(other));
}

Sphere scalar_geometricAntiQuotient_roundPoint(Scalar self, RoundPoint other) {
    return scalar_antiWedgeDot_roundPoint(self, roundPoint_antiInverse(other));
}

AntiScalar scalar_geometricAntiQuotient_scalar(Scalar self, Scalar other) {
    return scalar_antiWedgeDot_scalar(self, scalar_antiInverse(other));
}

RoundPoint scalar_geometricAntiQuotient_sphere(Scalar self, Sphere other) {
    return scalar_antiWedgeDot_sphere(self, sphere_antiInverse(other));
}

Sphere sphere_geometricAntiQuotient_antiScalar(Sphere self, AntiScalar other) {
    return sphere_antiWedgeDot_antiScalar(self, antiScalar_antiInverse(other));
}

MultiVector sphere_geometricAntiQuotient_circle(Sphere self, Circle other) {
    return sphere_antiWedgeDot_circle(self, circle_antiInverse(other));
}

MultiVector sphere_geometricAntiQuotient_dipole(Sphere self, Dipole other) {
    return sphere_antiWedgeDot_dipole(self, dipole_antiInverse(other));
}

MultiVector sphere_geometricAntiQuotient_dualNum(Sphere self, DualNum other) {
    return sphere_antiWedgeDot_dualNum(self, dualNum_antiInverse(other));
}

MultiVector sphere_geometricAntiQuotient_flatPoint(Sphere self, FlatPoint other) {
    return sphere_antiWedgeDot_flatPoint(self, flatPoint_antiInverse(other));
}

MultiVector sphere_geometricAntiQuotient_flector(Sphere self, Flector other) {
    return sphere_antiWedgeDot_flector(self, flector_antiInverse(other));
}

MultiVector sphere_geometricAntiQuotient_line(Sphere self, Line other) {
    return sphere_antiWedgeDot_line(self, line_antiInverse(other));
}

MultiVector sphere_geometricAntiQuotient_motor(Sphere self, Motor other) {
    return sphere_antiWedgeDot_motor(self, motor_antiInverse(other));
}

MultiVector sphere_geometricAntiQuotient_multiVector(Sphere self, MultiVector other) {
    return sphere_antiWedgeDot_multiVector(self, multiVector_antiInverse(other));
}

MultiVector sphere_geometricAntiQuotient_plane(Sphere self, Plane other) {
    return sphere_antiWedgeDot_plane(self, plane_antiInverse(other));
}

MultiVector sphere_geometricAntiQuotient_roundPoint(Sphere self, RoundPoint other) {
    return sphere_antiWedgeDot_roundPoint(self, roundPoint_antiInverse(other));
}

RoundPoint sphere_geometricAntiQuotient_scalar(Sphere self, Scalar other) {
    return sphere_antiWedgeDot_scalar(self, scalar_antiInverse(other));
}

MultiVector sphere_geometricAntiQuotient_sphere(Sphere self, Sphere other) {
    return sphere_antiWedgeDot_sphere(self, sphere_antiInverse(other));
}

Scalar antiScalar_geometricQuotient_antiScalar(AntiScalar self, AntiScalar other) {
    return antiScalar_wedgeDot_antiScalar(self, antiScalar_inverse(other));
}

Dipole antiScalar_geometricQuotient_circle(AntiScalar self, Circle other) {
    return antiScalar_wedgeDot_circle(self, circle_inverse(other));
}

Circle antiScalar_geometricQuotient_dipole(AntiScalar self, Dipole other) {
    return antiScalar_wedgeDot_dipole(self, dipole_inverse(other));
}

DualNum antiScalar_geometricQuotient_dualNum(AntiScalar self, DualNum other) {
    return antiScalar_wedgeDot_dualNum(self, dualNum_inverse(other));
}

Circle antiScalar_geometricQuotient_flatPoint(AntiScalar self, FlatPoint other) {
    return antiScalar_wedgeDot_flatPoint(self, flatPoint_inverse(other));
}

MultiVector antiScalar_geometricQuotient_flector(AntiScalar self, Flector other) {
    return antiScalar_wedgeDot_flector(self, flector_inverse(other));
}

Dipole antiScalar_geometricQuotient_line(AntiScalar self, Line other) {
    return antiScalar_wedgeDot_line(self, line_inverse(other));
}

MultiVector antiScalar_geometricQuotient_motor(AntiScalar self, Motor other) {
    return antiScalar_wedgeDot_motor(self, motor_inverse(other));
}

MultiVector antiScalar_geometricQuotient_multiVector(AntiScalar self, MultiVector other) {
    return antiScalar_wedgeDot_multiVector(self, multiVector_inverse(other));
}

RoundPoint antiScalar_geometricQuotient_plane(AntiScalar self, Plane other) {
    return antiScalar_wedgeDot_plane(self, plane_inverse(other));
}

Sphere antiScalar_geometricQuotient_roundPoint(AntiScalar self, RoundPoint other) {
    return antiScalar_wedgeDot_roundPoint(self, roundPoint_inverse(other));
}

AntiScalar antiScalar_geometricQuotient_scalar(AntiScalar self, Scalar other) {
    return antiScalar_wedgeDot_scalar(self, scalar_inverse(other));
}

RoundPoint antiScalar_geometricQuotient_sphere(AntiScalar self, Sphere other) {
    return antiScalar_wedgeDot_sphere(self, sphere_inverse(other));
}

Dipole circle_geometricQuotient_antiScalar(Circle self, AntiScalar other) {
    return circle_wedgeDot_antiScalar(self, antiScalar_inverse(other));
}

MultiVector circle_geometricQuotient_circle(Circle self, Circle other) {
    return circle_wedgeDot_circle(self, circle_inverse(other));
}

MultiVector circle_geometricQuotient_dipole(Circle self, Dipole other) {
    return circle_wedgeDot_dipole(self, dipole_inverse(other));
}

MultiVector circle_geometricQuotient_dualNum(Circle self, DualNum other) {
    return circle_wedgeDot_dualNum(self, dualNum_inverse(other));
}

MultiVector circle_geometricQuotient_flatPoint(Circle self, FlatPoint other) {
    return circle_wedgeDot_flatPoint(self, flatPoint_inverse(other));
}

MultiVector circle_geometricQuotient_flector(Circle self, Flector other) {
    return circle_wedgeDot_flector(self, flector_inverse(other));
}

MultiVector circle_geometricQuotient_line(Circle self, Line other) {
    return circle_wedgeDot_line(self, line_inverse(other));
}

MultiVector circle_geometricQuotient_motor(Circle self, Motor other) {
    return circle_wedgeDot_motor(self, motor_inverse(other));
}

MultiVector circle_geometricQuotient_multiVector(Circle self, MultiVector other) {
    return circle_wedgeDot_multiVector(self, multiVector_inverse(other));
}

MultiVector circle_geometricQuotient_plane(Circle self, Plane other) {
    return circle_wedgeDot_plane(self, plane_inverse(other));
}

MultiVector circle_geometricQuotient_roundPoint(Circle self, RoundPoint other) {
    return circle_wedgeDot_roundPoint(self, roundPoint_inverse(other));
}

Circle circle_geometricQuotient_scalar(Circle self, Scalar other) {
    return circle_wedgeDot_scalar(self, scalar_inverse(other));
}

MultiVector circle_geometricQuotient_sphere(Circle self, Sphere other) {
    return circle_wedgeDot_sphere(self, sphere_inverse(other));
}

Circle dipole_geometricQuotient_antiScalar(Dipole self, AntiScalar other) {
    return dipole_wedgeDot_antiScalar(self, antiScalar_inverse(other));
}

MultiVector dipole_geometricQuotient_circle(Dipole self, Circle other) {
    return dipole_wedgeDot_circle(self, circle_inverse(other));
}

MultiVector dipole_geometricQuotient_dipole(Dipole self, Dipole other) {
    return dipole_wedgeDot_dipole(self, dipole_inverse(other));
}

MultiVector dipole_geometricQuotient_dualNum(Dipole self, DualNum other) {
    return dipole_wedgeDot_dualNum(self, dualNum_inverse(other));
}

MultiVector dipole_geometricQuotient_flatPoint(Dipole self, FlatPoint other) {
    return dipole_wedgeDot_flatPoint(self, flatPoint_inverse(other));
}

MultiVector dipole_geometricQuotient_flector(Dipole self, Flector other) {
    return dipole_wedgeDot_flector(self, flector_inverse(other));
}

MultiVector dipole_geometricQuotient_line(Dipole self, Line other) {
    return dipole_wedgeDot_line(self, line_inverse(other));
}

MultiVector dipole_geometricQuotient_motor(Dipole self, Motor other) {
    return dipole_wedgeDot_motor(self, motor_inverse(other));
}

MultiVector dipole_geometricQuotient_multiVector(Dipole self, MultiVector other) {
    return dipole_wedgeDot_multiVector(self, multiVector_inverse(other));
}

MultiVector dipole_geometricQuotient_plane(Dipole self, Plane other) {
    return dipole_wedgeDot_plane(self, plane_inverse(other));
}

MultiVector dipole_geometricQuotient_roundPoint(Dipole self, RoundPoint other) {
    return dipole_wedgeDot_roundPoint(self, roundPoint_inverse(other));
}

Dipole dipole_geometricQuotient_scalar(Dipole self, Scalar other) {
    return dipole_wedgeDot_scalar(self, scalar_inverse(other));
}

MultiVector dipole_geometricQuotient_sphere(Dipole self, Sphere other) {
    return dipole_wedgeDot_sphere(self, sphere_inverse(other));
}

DualNum dualNum_geometricQuotient_antiScalar(DualNum self, AntiScalar other) {
    return dualNum_wedgeDot_antiScalar(self, antiScalar_inverse(other));
}

MultiVector dualNum_geometricQuotient_circle(DualNum self, Circle other) {
    return dualNum_wedgeDot_circle(self, circle_inverse(other));
}

MultiVector dualNum_geometricQuotient_dipole(DualNum self, Dipole other) {
    return dualNum_wedgeDot_dipole(self, dipole_inverse(other));
}

DualNum dualNum_geometricQuotient_dualNum(DualNum self, DualNum other) {
    return dualNum_wedgeDot_dualNum(self, dualNum_inverse(other));
}

MultiVector dualNum_geometricQuotient_flatPoint(DualNum self, FlatPoint other) {
    return dualNum_wedgeDot_flatPoint(self, flatPoint_inverse(other));
}

MultiVector dualNum_geometricQuotient_flector(DualNum self, Flector other) {
    return dualNum_wedgeDot_flector(self, flector_inverse(other));
}

MultiVector dualNum_geometricQuotient_line(DualNum self, Line other) {
    return dualNum_wedgeDot_line(self, line_inverse(other));
}

MultiVector dualNum_geometricQuotient_motor(DualNum self, Motor other) {
    return dualNum_wedgeDot_motor(self, motor_inverse(other));
}

MultiVector dualNum_geometricQuotient_multiVector(DualNum self, MultiVector other) {
    return dualNum_wedgeDot_multiVector(self, multiVector_inverse(other));
}

MultiVector dualNum_geometricQuotient_plane(DualNum self, Plane other) {
    return dualNum_wedgeDot_plane(self, plane_inverse(other));
}

MultiVector dualNum_geometricQuotient_roundPoint(DualNum self, RoundPoint other) {
    return dualNum_wedgeDot_roundPoint(self, roundPoint_inverse(other));
}

DualNum dualNum_geometricQuotient_scalar(DualNum self, Scalar other) {
    return dualNum_wedgeDot_scalar(self, scalar_inverse(other));
}

MultiVector dualNum_geometricQuotient_sphere(DualNum self, Sphere other) {
    return dualNum_wedgeDot_sphere(self, sphere_inverse(other));
}

Circle flatPoint_geometricQuotient_antiScalar(FlatPoint self, AntiScalar other) {
    return flatPoint_wedgeDot_antiScalar(self, antiScalar_inverse(other));
}

MultiVector flatPoint_geometricQuotient_circle(FlatPoint self, Circle other) {
    return flatPoint_wedgeDot_circle(self, circle_inverse(other));
}

MultiVector flatPoint_geometricQuotient_dipole(FlatPoint self, Dipole other) {
    return flatPoint_wedgeDot_dipole(self, dipole_inverse(other));
}

MultiVector flatPoint_geometricQuotient_dualNum(FlatPoint self, DualNum other) {
    return flatPoint_wedgeDot_dualNum(self, dualNum_inverse(other));
}

MultiVector flatPoint_geometricQuotient_flatPoint(FlatPoint self, FlatPoint other) {
    return flatPoint_wedgeDot_flatPoint(self, flatPoint_inverse(other));
}

MultiVector flatPoint_geometricQuotient_flector(FlatPoint self, Flector other) {
    return flatPoint_wedgeDot_flector(self, flector_inverse(other));
}

MultiVector flatPoint_geometricQuotient_line(FlatPoint self, Line other) {
    return flatPoint_wedgeDot_line(self, line_inverse(other));
}

MultiVector flatPoint_geometricQuotient_motor(FlatPoint self, Motor other) {
    return flatPoint_wedgeDot_motor(self, motor_inverse(other));
}

MultiVector flatPoint_geometricQuotient_multiVector(FlatPoint self, MultiVector other) {
    return flatPoint_wedgeDot_multiVector(self, multiVector_inverse(other));
}

MultiVector flatPoint_geometricQuotient_plane(FlatPoint self, Plane other) {
    return flatPoint_wedgeDot_plane(self, plane_inverse(other));
}

MultiVector flatPoint_geometricQuotient_roundPoint(FlatPoint self, RoundPoint other) {
    return flatPoint_wedgeDot_roundPoint(self, roundPoint_inverse(other));
}

FlatPoint flatPoint_geometricQuotient_scalar(FlatPoint self, Scalar other) {
    return flatPoint_wedgeDot_scalar(self, scalar_inverse(other));
}

MultiVector flatPoint_geometricQuotient_sphere(FlatPoint self, Sphere other) {
    return flatPoint_wedgeDot_sphere(self, sphere_inverse(other));
}

MultiVector flector_geometricQuotient_antiScalar(Flector self, AntiScalar other) {
    return flector_wedgeDot_antiScalar(self, antiScalar_inverse(other));
}

MultiVector flector_geometricQuotient_circle(Flector self, Circle other) {
    return flector_wedgeDot_circle(self, circle_inverse(other));
}

MultiVector flector_geometricQuotient_dipole(Flector self, Dipole other) {
    return flector_wedgeDot_dipole(self, dipole_inverse(other));
}

MultiVector flector_geometricQuotient_dualNum(Flector self, DualNum other) {
    return flector_wedgeDot_dualNum(self, dualNum_inverse(other));
}

MultiVector flector_geometricQuotient_flatPoint(Flector self, FlatPoint other) {
    return flector_wedgeDot_flatPoint(self, flatPoint_inverse(other));
}

MultiVector flector_geometricQuotient_flector(Flector self, Flector other) {
    return flector_wedgeDot_flector(self, flector_inverse(other));
}

MultiVector flector_geometricQuotient_line(Flector self, Line other) {
    return flector_wedgeDot_line(self, line_inverse(other));
}

MultiVector flector_geometricQuotient_motor(Flector self, Motor other) {
    return flector_wedgeDot_motor(self, motor_inverse(other));
}

MultiVector flector_geometricQuotient_multiVector(Flector self, MultiVector other) {
    return flector_wedgeDot_multiVector(self, multiVector_inverse(other));
}

MultiVector flector_geometricQuotient_plane(Flector self, Plane other) {
    return flector_wedgeDot_plane(self, plane_inverse(other));
}

MultiVector flector_geometricQuotient_roundPoint(Flector self, RoundPoint other) {
    return flector_wedgeDot_roundPoint(self, roundPoint_inverse(other));
}

Flector flector_geometricQuotient_scalar(Flector self, Scalar other) {
    return flector_wedgeDot_scalar(self, scalar_inverse(other));
}

MultiVector flector_geometricQuotient_sphere(Flector self, Sphere other) {
    return flector_wedgeDot_sphere(self, sphere_inverse(other));
}

Dipole line_geometricQuotient_antiScalar(Line self, AntiScalar other) {
    return line_wedgeDot_antiScalar(self, antiScalar_inverse(other));
}

MultiVector line_geometricQuotient_circle(Line self, Circle other) {
    return line_wedgeDot_circle(self, circle_inverse(other));
}

MultiVector line_geometricQuotient_dipole(Line self, Dipole other) {
    return line_wedgeDot_dipole(self, dipole_inverse(other));
}

MultiVector line_geometricQuotient_dualNum(Line self, DualNum other) {
    return line_wedgeDot_dualNum(self, dualNum_inverse(other));
}

MultiVector line_geometricQuotient_flatPoint(Line self, FlatPoint other) {
    return line_wedgeDot_flatPoint(self, flatPoint_inverse(other));
}

MultiVector line_geometricQuotient_flector(Line self, Flector other) {
    return line_wedgeDot_flector(self, flector_inverse(other));
}

MultiVector line_geometricQuotient_line(Line self, Line other) {
    return line_wedgeDot_line(self, line_inverse(other));
}

MultiVector line_geometricQuotient_motor(Line self, Motor other) {
    return line_wedgeDot_motor(self, motor_inverse(other));
}

MultiVector line_geometricQuotient_multiVector(Line self, MultiVector other) {
    return line_wedgeDot_multiVector(self, multiVector_inverse(other));
}

MultiVector line_geometricQuotient_plane(Line self, Plane other) {
    return line_wedgeDot_plane(self, plane_inverse(other));
}

MultiVector line_geometricQuotient_roundPoint(Line self, RoundPoint other) {
    return line_wedgeDot_roundPoint(self, roundPoint_inverse(other));
}

Line line_geometricQuotient_scalar(Line self, Scalar other) {
    return line_wedgeDot_scalar(self, scalar_inverse(other));
}

MultiVector line_geometricQuotient_sphere(Line self, Sphere other) {
    return line_wedgeDot_sphere(self, sphere_inverse(other));
}

MultiVector motor_geometricQuotient_antiScalar(Motor self, AntiScalar other) {
    return motor_wedgeDot_antiScalar(self, antiScalar_inverse(other));
}

MultiVector motor_geometricQuotient_circle(Motor self, Circle other) {
    return motor_wedgeDot_circle(self, circle_inverse(other));
}

MultiVector motor_geometricQuotient_dipole(Motor self, Dipole other) {
    return motor_wedgeDot_dipole(self, dipole_inverse(other));
}

MultiVector motor_geometricQuotient_dualNum(Motor self, DualNum other) {
    return motor_wedgeDot_dualNum(self, dualNum_inverse(other));
}

MultiVector motor_geometricQuotient_flatPoint(Motor self, FlatPoint other) {
    return motor_wedgeDot_flatPoint(self, flatPoint_inverse(other));
}

MultiVector motor_geometricQuotient_flector(Motor self, Flector other) {
    return motor_wedgeDot_flector(self, flector_inverse(other));
}

MultiVector motor_geometricQuotient_line(Motor self, Line other) {
    return motor_wedgeDot_line(self, line_inverse(other));
}

MultiVector motor_geometricQuotient_motor(Motor self, Motor other) {
    return motor_wedgeDot_motor(self, motor_inverse(other));
}

MultiVector motor_geometricQuotient_multiVector(Motor self, MultiVector other) {
    return motor_wedgeDot_multiVector(self, multiVector_inverse(other));
}

MultiVector motor_geometricQuotient_plane(Motor self, Plane other) {
    return motor_wedgeDot_plane(self, plane_inverse(other));
}

MultiVector motor_geometricQuotient_roundPoint(Motor self, RoundPoint other) {
    return motor_wedgeDot_roundPoint(self, roundPoint_inverse(other));
}

Motor motor_geometricQuotient_scalar(Motor self, Scalar other) {
    return motor_wedgeDot_scalar(self, scalar_inverse(other));
}

MultiVector motor_geometricQuotient_sphere(Motor self, Sphere other) {
    return motor_wedgeDot_sphere(self, sphere_inverse(other));
}

MultiVector multiVector_geometricQuotient_antiScalar(MultiVector self, AntiScalar other) {
    return multiVector_wedgeDot_antiScalar(self, antiScalar_inverse(other));
}

MultiVector multiVector_geometricQuotient_circle(MultiVector self, Circle other) {
    return multiVector_wedgeDot_circle(self, circle_inverse(other));
}

MultiVector multiVector_geometricQuotient_dipole(MultiVector self, Dipole other) {
    return multiVector_wedgeDot_dipole(self, dipole_inverse(other));
}

MultiVector multiVector_geometricQuotient_dualNum(MultiVector self, DualNum other) {
    return multiVector_wedgeDot_dualNum(self, dualNum_inverse(other));
}

MultiVector multiVector_geometricQuotient_flatPoint(MultiVector self, FlatPoint other) {
    return multiVector_wedgeDot_flatPoint(self, flatPoint_inverse(other));
}

MultiVector multiVector_geometricQuotient_flector(MultiVector self, Flector other) {
    return multiVector_wedgeDot_flector(self, flector_inverse(other));
}

MultiVector multiVector_geometricQuotient_line(MultiVector self, Line other) {
    return multiVector_wedgeDot_line(self, line_inverse(other));
}

MultiVector multiVector_geometricQuotient_motor(MultiVector self, Motor other) {
    return multiVector_wedgeDot_motor(self, motor_inverse(other));
}

MultiVector multiVector_geometricQuotient_multiVector(MultiVector self, MultiVector other) {
    return multiVector_wedgeDot_multiVector(self, multiVector_inverse(other));
}

MultiVector multiVector_geometricQuotient_plane(MultiVector self, Plane other) {
    return multiVector_wedgeDot_plane(self, plane_inverse(other));
}

MultiVector multiVector_geometricQuotient_roundPoint(MultiVector self, RoundPoint other) {
    return multiVector_wedgeDot_roundPoint(self, roundPoint_inverse(other));
}

MultiVector multiVector_geometricQuotient_scalar(MultiVector self, Scalar other) {
    return multiVector_wedgeDot_scalar(self, scalar_inverse(other));
}

MultiVector multiVector_geometricQuotient_sphere(MultiVector self, Sphere other) {
    return multiVector_wedgeDot_sphere(self, sphere_inverse(other));
}

RoundPoint plane_geometricQuotient_antiScalar(Plane self, AntiScalar other) {
    return plane_wedgeDot_antiScalar(self, antiScalar_inverse(other));
}

MultiVector plane_geometricQuotient_circle(Plane self, Circle other) {
    return plane_wedgeDot_circle(self, circle_inverse(other));
}

MultiVector plane_geometricQuotient_dipole(Plane self, Dipole other) {
    return plane_wedgeDot_dipole(self, dipole_inverse(other));
}

MultiVector plane_geometricQuotient_dualNum(Plane self, DualNum other) {
    return plane_wedgeDot_dualNum(self, dualNum_inverse(other));
}

MultiVector plane_geometricQuotient_flatPoint(Plane self, FlatPoint other) {
    return plane_wedgeDot_flatPoint(self, flatPoint_inverse(other));
}

MultiVector plane_geometricQuotient_flector(Plane self, Flector other) {
    return plane_wedgeDot_flector(self, flector_inverse(other));
}

MultiVector plane_geometricQuotient_line(Plane self, Line other) {
    return plane_wedgeDot_line(self, line_inverse(other));
}

MultiVector plane_geometricQuotient_motor(Plane self, Motor other) {
    return plane_wedgeDot_motor(self, motor_inverse(other));
}

MultiVector plane_geometricQuotient_multiVector(Plane self, MultiVector other) {
    return plane_wedgeDot_multiVector(self, multiVector_inverse(other));
}

MultiVector plane_geometricQuotient_plane(Plane self, Plane other) {
    return plane_wedgeDot_plane(self, plane_inverse(other));
}

MultiVector plane_geometricQuotient_roundPoint(Plane self, RoundPoint other) {
    return plane_wedgeDot_roundPoint(self, roundPoint_inverse(other));
}

Plane plane_geometricQuotient_scalar(Plane self, Scalar other) {
    return plane_wedgeDot_scalar(self, scalar_inverse(other));
}

MultiVector plane_geometricQuotient_sphere(Plane self, Sphere other) {
    return plane_wedgeDot_sphere(self, sphere_inverse(other));
}

Sphere roundPoint_geometricQuotient_antiScalar(RoundPoint self, AntiScalar other) {
    return roundPoint_wedgeDot_antiScalar(self, antiScalar_inverse(other));
}

MultiVector roundPoint_geometricQuotient_circle(RoundPoint self, Circle other) {
    return roundPoint_wedgeDot_circle(self, circle_inverse(other));
}

MultiVector roundPoint_geometricQuotient_dipole(RoundPoint self, Dipole other) {
    return roundPoint_wedgeDot_dipole(self, dipole_inverse(other));
}

MultiVector roundPoint_geometricQuotient_dualNum(RoundPoint self, DualNum other) {
    return roundPoint_wedgeDot_dualNum(self, dualNum_inverse(other));
}

MultiVector roundPoint_geometricQuotient_flatPoint(RoundPoint self, FlatPoint other) {
    return roundPoint_wedgeDot_flatPoint(self, flatPoint_inverse(other));
}

MultiVector roundPoint_geometricQuotient_flector(RoundPoint self, Flector other) {
    return roundPoint_wedgeDot_flector(self, flector_inverse(other));
}

MultiVector roundPoint_geometricQuotient_line(RoundPoint self, Line other) {
    return roundPoint_wedgeDot_line(self, line_inverse(other));
}

MultiVector roundPoint_geometricQuotient_motor(RoundPoint self, Motor other) {
    return roundPoint_wedgeDot_motor(self, motor_inverse(other));
}

MultiVector roundPoint_geometricQuotient_multiVector(RoundPoint self, MultiVector other) {
    return roundPoint_wedgeDot_multiVector(self, multiVector_inverse(other));
}

MultiVector roundPoint_geometricQuotient_plane(RoundPoint self, Plane other) {
    return roundPoint_wedgeDot_plane(self, plane_inverse(other));
}

MultiVector roundPoint_geometricQuotient_roundPoint(RoundPoint self, RoundPoint other) {
    return roundPoint_wedgeDot_roundPoint(self, roundPoint_inverse(other));
}

RoundPoint roundPoint_geometricQuotient_scalar(RoundPoint self, Scalar other) {
    return roundPoint_wedgeDot_scalar(self, scalar_inverse(other));
}

MultiVector roundPoint_geometricQuotient_sphere(RoundPoint self, Sphere other) {
    return roundPoint_wedgeDot_sphere(self, sphere_inverse(other));
}

AntiScalar scalar_geometricQuotient_antiScalar(Scalar self, AntiScalar other) {
    return scalar_wedgeDot_antiScalar(self, antiScalar_inverse(other));
}

Circle scalar_geometricQuotient_circle(Scalar self, Circle other) {
    return scalar_wedgeDot_circle(self, circle_inverse(other));
}

Dipole scalar_geometricQuotient_dipole(Scalar self, Dipole other) {
    return scalar_wedgeDot_dipole(self, dipole_inverse(other));
}

DualNum scalar_geometricQuotient_dualNum(Scalar self, DualNum other) {
    return scalar_wedgeDot_dualNum(self, dualNum_inverse(other));
}

FlatPoint scalar_geometricQuotient_flatPoint(Scalar self, FlatPoint other) {
    return scalar_wedgeDot_flatPoint(self, flatPoint_inverse(other));
}

Flector scalar_geometricQuotient_flector(Scalar self, Flector other) {
    return scalar_wedgeDot_flector(self, flector_inverse(other));
}

Line scalar_geometricQuotient_line(Scalar self, Line other) {
    return scalar_wedgeDot_line(self, line_inverse(other));
}

Motor scalar_geometricQuotient_motor(Scalar self, Motor other) {
    return scalar_wedgeDot_motor(self, motor_inverse(other));
}

MultiVector scalar_geometricQuotient_multiVector(Scalar self, MultiVector other) {
    return scalar_wedgeDot_multiVector(self, multiVector_inverse(other));
}

Plane scalar_geometricQuotient_plane(Scalar self, Plane other) {
    return scalar_wedgeDot_plane(self, plane_inverse(other));
}

RoundPoint scalar_geometricQuotient_roundPoint(Scalar self, RoundPoint other) {
    return scalar_wedgeDot_roundPoint(self, roundPoint_inverse(other));
}

Scalar scalar_geometricQuotient_scalar(Scalar self, Scalar other) {
    return scalar_wedgeDot_scalar(self, scalar_inverse(other));
}

Sphere scalar_geometricQuotient_sphere(Scalar self, Sphere other) {
    return scalar_wedgeDot_sphere(self, sphere_inverse(other));
}

RoundPoint sphere_geometricQuotient_antiScalar(Sphere self, AntiScalar other) {
    return sphere_wedgeDot_antiScalar(self, antiScalar_inverse(other));
}

MultiVector sphere_geometricQuotient_circle(Sphere self, Circle other) {
    return sphere_wedgeDot_circle(self, circle_inverse(other));
}

MultiVector sphere_geometricQuotient_dipole(Sphere self, Dipole other) {
    return sphere_wedgeDot_dipole(self, dipole_inverse(other));
}

MultiVector sphere_geometricQuotient_dualNum(Sphere self, DualNum other) {
    return sphere_wedgeDot_dualNum(self, dualNum_inverse(other));
}

MultiVector sphere_geometricQuotient_flatPoint(Sphere self, FlatPoint other) {
    return sphere_wedgeDot_flatPoint(self, flatPoint_inverse(other));
}

MultiVector sphere_geometricQuotient_flector(Sphere self, Flector other) {
    return sphere_wedgeDot_flector(self, flector_inverse(other));
}

MultiVector sphere_geometricQuotient_line(Sphere self, Line other) {
    return sphere_wedgeDot_line(self, line_inverse(other));
}

MultiVector sphere_geometricQuotient_motor(Sphere self, Motor other) {
    return sphere_wedgeDot_motor(self, motor_inverse(other));
}

MultiVector sphere_geometricQuotient_multiVector(Sphere self, MultiVector other) {
    return sphere_wedgeDot_multiVector(self, multiVector_inverse(other));
}

MultiVector sphere_geometricQuotient_plane(Sphere self, Plane other) {
    return sphere_wedgeDot_plane(self, plane_inverse(other));
}

MultiVector sphere_geometricQuotient_roundPoint(Sphere self, RoundPoint other) {
    return sphere_wedgeDot_roundPoint(self, roundPoint_inverse(other));
}

Sphere sphere_geometricQuotient_scalar(Sphere self, Scalar other) {
    return sphere_wedgeDot_scalar(self, scalar_inverse(other));
}

MultiVector sphere_geometricQuotient_sphere(Sphere self, Sphere other) {
    return sphere_wedgeDot_sphere(self, sphere_inverse(other));
}

Scalar circle_contraction_circle(Circle self, Circle other) {
    return circle_antiWedge_dipole(self, circle_antiDual(other));
}

RoundPoint circle_contraction_dipole(Circle self, Dipole other) {
    return circle_antiWedge_circle(self, dipole_antiDual(other));
}

RoundPoint circle_contraction_flatPoint(Circle self, FlatPoint other) {
    return circle_antiWedge_circle(self, flatPoint_antiDual(other));
}

MultiVector circle_contraction_flector(Circle self, Flector other) {
    return circle_antiWedge_multiVector(self, flector_antiDual(other));
}

Scalar circle_contraction_line(Circle self, Line other) {
    return circle_antiWedge_dipole(self, line_antiDual(other));
}

MultiVector circle_contraction_motor(Circle self, Motor other) {
    return circle_antiWedge_multiVector(self, motor_antiDual(other));
}

MultiVector circle_contraction_multiVector(Circle self, MultiVector other) {
    return circle_antiWedge_multiVector(self, multiVector_antiDual(other));
}

Dipole circle_contraction_roundPoint(Circle self, RoundPoint other) {
    return circle_antiWedge_sphere(self, roundPoint_antiDual(other));
}

Scalar dipole_contraction_dipole(Dipole self, Dipole other) {
    return dipole_antiWedge_circle(self, dipole_antiDual(other));
}

Scalar dipole_contraction_flatPoint(Dipole self, FlatPoint other) {
    return dipole_antiWedge_circle(self, flatPoint_antiDual(other));
}

MultiVector dipole_contraction_flector(Dipole self, Flector other) {
    return dipole_antiWedge_multiVector(self, flector_antiDual(other));
}

MultiVector dipole_contraction_motor(Dipole self, Motor other) {
    return dipole_antiWedge_multiVector(self, motor_antiDual(other));
}

MultiVector dipole_contraction_multiVector(Dipole self, MultiVector other) {
    return dipole_antiWedge_multiVector(self, multiVector_antiDual(other));
}

RoundPoint dipole_contraction_roundPoint(Dipole self, RoundPoint other) {
    return dipole_antiWedge_sphere(self, roundPoint_antiDual(other));
}

Scalar flatPoint_contraction_dipole(FlatPoint self, Dipole other) {
    return flatPoint_antiWedge_circle(self, dipole_antiDual(other));
}

Scalar flatPoint_contraction_flatPoint(FlatPoint self, FlatPoint other) {
    return flatPoint_antiWedge_circle(self, flatPoint_antiDual(other));
}

MultiVector flatPoint_contraction_flector(FlatPoint self, Flector other) {
    return flatPoint_antiWedge_multiVector(self, flector_antiDual(other));
}

MultiVector flatPoint_contraction_motor(FlatPoint self, Motor other) {
    return flatPoint_antiWedge_multiVector(self, motor_antiDual(other));
}

MultiVector flatPoint_contraction_multiVector(FlatPoint self, MultiVector other) {
    return flatPoint_antiWedge_multiVector(self, multiVector_antiDual(other));
}

RoundPoint flatPoint_contraction_roundPoint(FlatPoint self, RoundPoint other) {
    return flatPoint_antiWedge_sphere(self, roundPoint_antiDual(other));
}

RoundPoint flector_contraction_circle(Flector self, Circle other) {
    return flector_antiWedge_dipole(self, circle_antiDual(other));
}

MultiVector flector_contraction_dipole(Flector self, Dipole other) {
    return flector_antiWedge_circle(self, dipole_antiDual(other));
}

MultiVector flector_contraction_flatPoint(Flector self, FlatPoint other) {
    return flector_antiWedge_circle(self, flatPoint_antiDual(other));
}

MultiVector flector_contraction_flector(Flector self, Flector other) {
    return flector_antiWedge_multiVector(self, flector_antiDual(other));
}

RoundPoint flector_contraction_line(Flector self, Line other) {
    return flector_antiWedge_dipole(self, line_antiDual(other));
}

MultiVector flector_contraction_motor(Flector self, Motor other) {
    return flector_antiWedge_multiVector(self, motor_antiDual(other));
}

MultiVector flector_contraction_multiVector(Flector self, MultiVector other) {
    return flector_antiWedge_multiVector(self, multiVector_antiDual(other));
}

Scalar flector_contraction_plane(Flector self, Plane other) {
    return flector_antiWedge_roundPoint(self, plane_antiDual(other));
}

MultiVector flector_contraction_roundPoint(Flector self, RoundPoint other) {
    return flector_antiWedge_sphere(self, roundPoint_antiDual(other));
}

Scalar flector_contraction_sphere(Flector self, Sphere other) {
    return flector_antiWedge_roundPoint(self, sphere_antiDual(other));
}

Scalar line_contraction_circle(Line self, Circle other) {
    return line_antiWedge_dipole(self, circle_antiDual(other));
}

RoundPoint line_contraction_dipole(Line self, Dipole other) {
    return line_antiWedge_circle(self, dipole_antiDual(other));
}

RoundPoint line_contraction_flatPoint(Line self, FlatPoint other) {
    return line_antiWedge_circle(self, flatPoint_antiDual(other));
}

MultiVector line_contraction_flector(Line self, Flector other) {
    return line_antiWedge_multiVector(self, flector_antiDual(other));
}

Scalar line_contraction_line(Line self, Line other) {
    return line_antiWedge_dipole(self, line_antiDual(other));
}

MultiVector line_contraction_motor(Line self, Motor other) {
    return line_antiWedge_multiVector(self, motor_antiDual(other));
}

MultiVector line_contraction_multiVector(Line self, MultiVector other) {
    return line_antiWedge_multiVector(self, multiVector_antiDual(other));
}

Dipole line_contraction_roundPoint(Line self, RoundPoint other) {
    return line_antiWedge_sphere(self, roundPoint_antiDual(other));
}

MultiVector motor_contraction_circle(Motor self, Circle other) {
    return motor_antiWedge_dipole(self, circle_antiDual(other));
}

MultiVector motor_contraction_dipole(Motor self, Dipole other) {
    return motor_antiWedge_circle(self, dipole_antiDual(other));
}

MultiVector motor_contraction_flatPoint(Motor self, FlatPoint other) {
    return motor_antiWedge_circle(self, flatPoint_antiDual(other));
}

MultiVector motor_contraction_flector(Motor self, Flector other) {
    return motor_antiWedge_multiVector(self, flector_antiDual(other));
}

MultiVector motor_contraction_line(Motor self, Line other) {
    return motor_antiWedge_dipole(self, line_antiDual(other));
}

MultiVector motor_contraction_motor(Motor self, Motor other) {
    return motor_antiWedge_multiVector(self, motor_antiDual(other));
}

MultiVector motor_contraction_multiVector(Motor self, MultiVector other) {
    return motor_antiWedge_multiVector(self, multiVector_antiDual(other));
}

RoundPoint motor_contraction_plane(Motor self, Plane other) {
    return motor_antiWedge_roundPoint(self, plane_antiDual(other));
}

MultiVector motor_contraction_roundPoint(Motor self, RoundPoint other) {
    return motor_antiWedge_sphere(self, roundPoint_antiDual(other));
}

RoundPoint motor_contraction_sphere(Motor self, Sphere other) {
    return motor_antiWedge_roundPoint(self, sphere_antiDual(other));
}

MultiVector multiVector_contraction_circle(MultiVector self, Circle other) {
    return multiVector_antiWedge_dipole(self, circle_antiDual(other));
}

MultiVector multiVector_contraction_dipole(MultiVector self, Dipole other) {
    return multiVector_antiWedge_circle(self, dipole_antiDual(other));
}

MultiVector multiVector_contraction_flatPoint(MultiVector self, FlatPoint other) {
    return multiVector_antiWedge_circle(self, flatPoint_antiDual(other));
}

MultiVector multiVector_contraction_flector(MultiVector self, Flector other) {
    return multiVector_antiWedge_multiVector(self, flector_antiDual(other));
}

MultiVector multiVector_contraction_line(MultiVector self, Line other) {
    return multiVector_antiWedge_dipole(self, line_antiDual(other));
}

MultiVector multiVector_contraction_motor(MultiVector self, Motor other) {
    return multiVector_antiWedge_multiVector(self, motor_antiDual(other));
}

MultiVector multiVector_contraction_multiVector(MultiVector self, MultiVector other) {
    return multiVector_antiWedge_multiVector(self, multiVector_antiDual(other));
}

MultiVector multiVector_contraction_plane(MultiVector self, Plane other) {
    return multiVector_antiWedge_roundPoint(self, plane_antiDual(other));
}

MultiVector multiVector_contraction_roundPoint(MultiVector self, RoundPoint other) {
    return multiVector_antiWedge_sphere(self, roundPoint_antiDual(other));
}

MultiVector multiVector_contraction_sphere(MultiVector self, Sphere other) {
    return multiVector_antiWedge_roundPoint(self, sphere_antiDual(other));
}

RoundPoint plane_contraction_circle(Plane self, Circle other) {
    return plane_antiWedge_dipole(self, circle_antiDual(other));
}

Dipole plane_contraction_dipole(Plane self, Dipole other) {
    return plane_antiWedge_circle(self, dipole_antiDual(other));
}

Dipole plane_contraction_flatPoint(Plane self, FlatPoint other) {
    return plane_antiWedge_circle(self, flatPoint_antiDual(other));
}

MultiVector plane_contraction_flector(Plane self, Flector other) {
    return plane_antiWedge_multiVector(self, flector_antiDual(other));
}

RoundPoint plane_contraction_line(Plane self, Line other) {
    return plane_antiWedge_dipole(self, line_antiDual(other));
}

MultiVector plane_contraction_motor(Plane self, Motor other) {
    return plane_antiWedge_multiVector(self, motor_antiDual(other));
}

MultiVector plane_contraction_multiVector(Plane self, MultiVector other) {
    return plane_antiWedge_multiVector(self, multiVector_antiDual(other));
}

Scalar plane_contraction_plane(Plane self, Plane other) {
    return plane_antiWedge_roundPoint(self, plane_antiDual(other));
}

Circle plane_contraction_roundPoint(Plane self, RoundPoint other) {
    return plane_antiWedge_sphere(self, roundPoint_antiDual(other));
}

Scalar plane_contraction_sphere(Plane self, Sphere other) {
    return plane_antiWedge_roundPoint(self, sphere_antiDual(other));
}

MultiVector roundPoint_contraction_flector(RoundPoint self, Flector other) {
    return roundPoint_antiWedge_multiVector(self, flector_antiDual(other));
}

MultiVector roundPoint_contraction_motor(RoundPoint self, Motor other) {
    return roundPoint_antiWedge_multiVector(self, motor_antiDual(other));
}

MultiVector roundPoint_contraction_multiVector(RoundPoint self, MultiVector other) {
    return roundPoint_antiWedge_multiVector(self, multiVector_antiDual(other));
}

Scalar roundPoint_contraction_roundPoint(RoundPoint self, RoundPoint other) {
    return roundPoint_antiWedge_sphere(self, roundPoint_antiDual(other));
}

RoundPoint sphere_contraction_circle(Sphere self, Circle other) {
    return sphere_antiWedge_dipole(self, circle_antiDual(other));
}

Dipole sphere_contraction_dipole(Sphere self, Dipole other) {
    return sphere_antiWedge_circle(self, dipole_antiDual(other));
}

Dipole sphere_contraction_flatPoint(Sphere self, FlatPoint other) {
    return sphere_antiWedge_circle(self, flatPoint_antiDual(other));
}

MultiVector sphere_contraction_flector(Sphere self, Flector other) {
    return sphere_antiWedge_multiVector(self, flector_antiDual(other));
}

RoundPoint sphere_contraction_line(Sphere self, Line other) {
    return sphere_antiWedge_dipole(self, line_antiDual(other));
}

MultiVector sphere_contraction_motor(Sphere self, Motor other) {
    return sphere_antiWedge_multiVector(self, motor_antiDual(other));
}

MultiVector sphere_contraction_multiVector(Sphere self, MultiVector other) {
    return sphere_antiWedge_multiVector(self, multiVector_antiDual(other));
}

Scalar sphere_contraction_plane(Sphere self, Plane other) {
    return sphere_antiWedge_roundPoint(self, plane_antiDual(other));
}

Circle sphere_contraction_roundPoint(Sphere self, RoundPoint other) {
    return sphere_antiWedge_sphere(self, roundPoint_antiDual(other));
}

Scalar sphere_contraction_sphere(Sphere self, Sphere other) {
    return sphere_antiWedge_roundPoint(self, sphere_antiDual(other));
}

AntiScalar circle_expansion_circle(Circle self, Circle other) {
    return circle_wedge_dipole(self, circle_antiDual(other));
}

MultiVector circle_expansion_flector(Circle self, Flector other) {
    return circle_wedge_multiVector(self, flector_antiDual(other));
}

AntiScalar circle_expansion_line(Circle self, Line other) {
    return circle_wedge_dipole(self, line_antiDual(other));
}

MultiVector circle_expansion_motor(Circle self, Motor other) {
    return circle_wedge_multiVector(self, motor_antiDual(other));
}

MultiVector circle_expansion_multiVector(Circle self, MultiVector other) {
    return circle_wedge_multiVector(self, multiVector_antiDual(other));
}

Sphere circle_expansion_plane(Circle self, Plane other) {
    return circle_wedge_roundPoint(self, plane_antiDual(other));
}

Sphere circle_expansion_sphere(Circle self, Sphere other) {
    return circle_wedge_roundPoint(self, sphere_antiDual(other));
}

Sphere dipole_expansion_circle(Dipole self, Circle other) {
    return dipole_wedge_dipole(self, circle_antiDual(other));
}

AntiScalar dipole_expansion_dipole(Dipole self, Dipole other) {
    return dipole_wedge_circle(self, dipole_antiDual(other));
}

AntiScalar dipole_expansion_flatPoint(Dipole self, FlatPoint other) {
    return dipole_wedge_circle(self, flatPoint_antiDual(other));
}

MultiVector dipole_expansion_flector(Dipole self, Flector other) {
    return dipole_wedge_multiVector(self, flector_antiDual(other));
}

Sphere dipole_expansion_line(Dipole self, Line other) {
    return dipole_wedge_dipole(self, line_antiDual(other));
}

MultiVector dipole_expansion_motor(Dipole self, Motor other) {
    return dipole_wedge_multiVector(self, motor_antiDual(other));
}

MultiVector dipole_expansion_multiVector(Dipole self, MultiVector other) {
    return dipole_wedge_multiVector(self, multiVector_antiDual(other));
}

Circle dipole_expansion_plane(Dipole self, Plane other) {
    return dipole_wedge_roundPoint(self, plane_antiDual(other));
}

Circle dipole_expansion_sphere(Dipole self, Sphere other) {
    return dipole_wedge_roundPoint(self, sphere_antiDual(other));
}

Plane flatPoint_expansion_circle(FlatPoint self, Circle other) {
    return flatPoint_wedge_dipole(self, circle_antiDual(other));
}

AntiScalar flatPoint_expansion_dipole(FlatPoint self, Dipole other) {
    return flatPoint_wedge_circle(self, dipole_antiDual(other));
}

AntiScalar flatPoint_expansion_flatPoint(FlatPoint self, FlatPoint other) {
    return flatPoint_wedge_circle(self, flatPoint_antiDual(other));
}

MultiVector flatPoint_expansion_flector(FlatPoint self, Flector other) {
    return flatPoint_wedge_multiVector(self, flector_antiDual(other));
}

Plane flatPoint_expansion_line(FlatPoint self, Line other) {
    return flatPoint_wedge_dipole(self, line_antiDual(other));
}

MultiVector flatPoint_expansion_motor(FlatPoint self, Motor other) {
    return flatPoint_wedge_multiVector(self, motor_antiDual(other));
}

MultiVector flatPoint_expansion_multiVector(FlatPoint self, MultiVector other) {
    return flatPoint_wedge_multiVector(self, multiVector_antiDual(other));
}

Line flatPoint_expansion_plane(FlatPoint self, Plane other) {
    return flatPoint_wedge_roundPoint(self, plane_antiDual(other));
}

Line flatPoint_expansion_sphere(FlatPoint self, Sphere other) {
    return flatPoint_wedge_roundPoint(self, sphere_antiDual(other));
}

Plane flector_expansion_circle(Flector self, Circle other) {
    return flector_wedge_dipole(self, circle_antiDual(other));
}

AntiScalar flector_expansion_dipole(Flector self, Dipole other) {
    return flector_wedge_circle(self, dipole_antiDual(other));
}

AntiScalar flector_expansion_flatPoint(Flector self, FlatPoint other) {
    return flector_wedge_circle(self, flatPoint_antiDual(other));
}

MultiVector flector_expansion_flector(Flector self, Flector other) {
    return flector_wedge_multiVector(self, flector_antiDual(other));
}

Plane flector_expansion_line(Flector self, Line other) {
    return flector_wedge_dipole(self, line_antiDual(other));
}

MultiVector flector_expansion_motor(Flector self, Motor other) {
    return flector_wedge_multiVector(self, motor_antiDual(other));
}

MultiVector flector_expansion_multiVector(Flector self, MultiVector other) {
    return flector_wedge_multiVector(self, multiVector_antiDual(other));
}

Motor flector_expansion_plane(Flector self, Plane other) {
    return flector_wedge_roundPoint(self, plane_antiDual(other));
}

Motor flector_expansion_sphere(Flector self, Sphere other) {
    return flector_wedge_roundPoint(self, sphere_antiDual(other));
}

AntiScalar line_expansion_circle(Line self, Circle other) {
    return line_wedge_dipole(self, circle_antiDual(other));
}

MultiVector line_expansion_flector(Line self, Flector other) {
    return line_wedge_multiVector(self, flector_antiDual(other));
}

AntiScalar line_expansion_line(Line self, Line other) {
    return line_wedge_dipole(self, line_antiDual(other));
}

MultiVector line_expansion_motor(Line self, Motor other) {
    return line_wedge_multiVector(self, motor_antiDual(other));
}

MultiVector line_expansion_multiVector(Line self, MultiVector other) {
    return line_wedge_multiVector(self, multiVector_antiDual(other));
}

Plane line_expansion_plane(Line self, Plane other) {
    return line_wedge_roundPoint(self, plane_antiDual(other));
}

Plane line_expansion_sphere(Line self, Sphere other) {
    return line_wedge_roundPoint(self, sphere_antiDual(other));
}

AntiScalar motor_expansion_circle(Motor self, Circle other) {
    return motor_wedge_dipole(self, circle_antiDual(other));
}

MultiVector motor_expansion_flector(Motor self, Flector other) {
    return motor_wedge_multiVector(self, flector_antiDual(other));
}

AntiScalar motor_expansion_line(Motor self, Line other) {
    return motor_wedge_dipole(self, line_antiDual(other));
}

MultiVector motor_expansion_motor(Motor self, Motor other) {
    return motor_wedge_multiVector(self, motor_antiDual(other));
}

MultiVector motor_expansion_multiVector(Motor self, MultiVector other) {
    return motor_wedge_multiVector(self, multiVector_antiDual(other));
}

Plane motor_expansion_plane(Motor self, Plane other) {
    return motor_wedge_roundPoint(self, plane_antiDual(other));
}

Plane motor_expansion_sphere(Motor self, Sphere other) {
    return motor_wedge_roundPoint(self, sphere_antiDual(other));
}

MultiVector multiVector_expansion_circle(MultiVector self, Circle other) {
    return multiVector_wedge_dipole(self, circle_antiDual(other));
}

MultiVector multiVector_expansion_dipole(MultiVector self, Dipole other) {
    return multiVector_wedge_circle(self, dipole_antiDual(other));
}

MultiVector multiVector_expansion_flatPoint(MultiVector self, FlatPoint other) {
    return multiVector_wedge_circle(self, flatPoint_antiDual(other));
}

MultiVector multiVector_expansion_flector(MultiVector self, Flector other) {
    return multiVector_wedge_multiVector(self, flector_antiDual(other));
}

MultiVector multiVector_expansion_line(MultiVector self, Line other) {
    return multiVector_wedge_dipole(self, line_antiDual(other));
}

MultiVector multiVector_expansion_motor(MultiVector self, Motor other) {
    return multiVector_wedge_multiVector(self, motor_antiDual(other));
}

MultiVector multiVector_expansion_multiVector(MultiVector self, MultiVector other) {
    return multiVector_wedge_multiVector(self, multiVector_antiDual(other));
}

MultiVector multiVector_expansion_plane(MultiVector self, Plane other) {
    return multiVector_wedge_roundPoint(self, plane_antiDual(other));
}

MultiVector multiVector_expansion_roundPoint(MultiVector self, RoundPoint other) {
    return multiVector_wedge_sphere(self, roundPoint_antiDual(other));
}

MultiVector multiVector_expansion_sphere(MultiVector self, Sphere other) {
    return multiVector_wedge_roundPoint(self, sphere_antiDual(other));
}

MultiVector plane_expansion_flector(Plane self, Flector other) {
    return plane_wedge_multiVector(self, flector_antiDual(other));
}

MultiVector plane_expansion_motor(Plane self, Motor other) {
    return plane_wedge_multiVector(self, motor_antiDual(other));
}

MultiVector plane_expansion_multiVector(Plane self, MultiVector other) {
    return plane_wedge_multiVector(self, multiVector_antiDual(other));
}

AntiScalar plane_expansion_plane(Plane self, Plane other) {
    return plane_wedge_roundPoint(self, plane_antiDual(other));
}

AntiScalar plane_expansion_sphere(Plane self, Sphere other) {
    return plane_wedge_roundPoint(self, sphere_antiDual(other));
}

Circle roundPoint_expansion_circle(RoundPoint self, Circle other) {
    return roundPoint_wedge_dipole(self, circle_antiDual(other));
}

Sphere roundPoint_expansion_dipole(RoundPoint self, Dipole other) {
    return roundPoint_wedge_circle(self, dipole_antiDual(other));
}

Sphere roundPoint_expansion_flatPoint(RoundPoint self, FlatPoint other) {
    return roundPoint_wedge_circle(self, flatPoint_antiDual(other));
}

MultiVector roundPoint_expansion_flector(RoundPoint self, Flector other) {
    return roundPoint_wedge_multiVector(self, flector_antiDual(other));
}

Circle roundPoint_expansion_line(RoundPoint self, Line other) {
    return roundPoint_wedge_dipole(self, line_antiDual(other));
}

MultiVector roundPoint_expansion_motor(RoundPoint self, Motor other) {
    return roundPoint_wedge_multiVector(self, motor_antiDual(other));
}

MultiVector roundPoint_expansion_multiVector(RoundPoint self, MultiVector other) {
    return roundPoint_wedge_multiVector(self, multiVector_antiDual(other));
}

Dipole roundPoint_expansion_plane(RoundPoint self, Plane other) {
    return roundPoint_wedge_roundPoint(self, plane_antiDual(other));
}

AntiScalar roundPoint_expansion_roundPoint(RoundPoint self, RoundPoint other) {
    return roundPoint_wedge_sphere(self, roundPoint_antiDual(other));
}

Dipole roundPoint_expansion_sphere(RoundPoint self, Sphere other) {
    return roundPoint_wedge_roundPoint(self, sphere_antiDual(other));
}

MultiVector sphere_expansion_flector(Sphere self, Flector other) {
    return sphere_wedge_multiVector(self, flector_antiDual(other));
}

MultiVector sphere_expansion_motor(Sphere self, Motor other) {
    return sphere_wedge_multiVector(self, motor_antiDual(other));
}

MultiVector sphere_expansion_multiVector(Sphere self, MultiVector other) {
    return sphere_wedge_multiVector(self, multiVector_antiDual(other));
}

AntiScalar sphere_expansion_plane(Sphere self, Plane other) {
    return sphere_wedge_roundPoint(self, plane_antiDual(other));
}

AntiScalar sphere_expansion_sphere(Sphere self, Sphere other) {
    return sphere_wedge_roundPoint(self, sphere_antiDual(other));
}

Circle circle_antiProjectOrthogonallyOnto_circle(Circle self, Circle other) {
    return circle_wedge_scalar(other, circle_antiWedge_dipole(self, circle_antiDual(other)));
}

Circle circle_antiProjectOrthogonallyOnto_dipole(Circle self, Dipole other) {
    return dipole_wedge_roundPoint(other, circle_antiWedge_circle(self, dipole_antiDual(other)));
}

Line circle_antiProjectOrthogonallyOnto_flatPoint(Circle self, FlatPoint other) {
    return flatPoint_wedge_roundPoint(other, circle_antiWedge_circle(self, flatPoint_antiDual(other)));
}

MultiVector circle_antiProjectOrthogonallyOnto_flector(Circle self, Flector other) {
    return flector_wedge_multiVector(other, circle_antiWedge_multiVector(self, flector_antiDual(other)));
}

Line circle_antiProjectOrthogonallyOnto_line(Circle self, Line other) {
    return line_wedge_scalar(other, circle_antiWedge_dipole(self, line_antiDual(other)));
}

MultiVector circle_antiProjectOrthogonallyOnto_motor(Circle self, Motor other) {
    return motor_wedge_multiVector(other, circle_antiWedge_multiVector(self, motor_antiDual(other)));
}

MultiVector circle_antiProjectOrthogonallyOnto_multiVector(Circle self, MultiVector other) {
    return multiVector_wedge_multiVector(other, circle_antiWedge_multiVector(self, multiVector_antiDual(other)));
}

Circle circle_antiProjectOrthogonallyOnto_roundPoint(Circle self, RoundPoint other) {
    return roundPoint_wedge_dipole(other, circle_antiWedge_sphere(self, roundPoint_antiDual(other)));
}

Dipole dipole_antiProjectOrthogonallyOnto_dipole(Dipole self, Dipole other) {
    return dipole_wedge_scalar(other, dipole_antiWedge_circle(self, dipole_antiDual(other)));
}

FlatPoint dipole_antiProjectOrthogonallyOnto_flatPoint(Dipole self, FlatPoint other) {
    return flatPoint_wedge_scalar(other, dipole_antiWedge_circle(self, flatPoint_antiDual(other)));
}

MultiVector dipole_antiProjectOrthogonallyOnto_flector(Dipole self, Flector other) {
    return flector_wedge_multiVector(other, dipole_antiWedge_multiVector(self, flector_antiDual(other)));
}

MultiVector dipole_antiProjectOrthogonallyOnto_motor(Dipole self, Motor other) {
    return motor_wedge_multiVector(other, dipole_antiWedge_multiVector(self, motor_antiDual(other)));
}

MultiVector dipole_antiProjectOrthogonallyOnto_multiVector(Dipole self, MultiVector other) {
    return multiVector_wedge_multiVector(other, dipole_antiWedge_multiVector(self, multiVector_antiDual(other)));
}

Dipole dipole_antiProjectOrthogonallyOnto_roundPoint(Dipole self, RoundPoint other) {
    return roundPoint_wedge_roundPoint(other, dipole_antiWedge_sphere(self, roundPoint_antiDual(other)));
}

Dipole flatPoint_antiProjectOrthogonallyOnto_dipole(FlatPoint self, Dipole other) {
    return dipole_wedge_scalar(other, flatPoint_antiWedge_circle(self, dipole_antiDual(other)));
}

FlatPoint flatPoint_antiProjectOrthogonallyOnto_flatPoint(FlatPoint self, FlatPoint other) {
    return flatPoint_wedge_scalar(other, flatPoint_antiWedge_circle(self, flatPoint_antiDual(other)));
}

MultiVector flatPoint_antiProjectOrthogonallyOnto_flector(FlatPoint self, Flector other) {
    return flector_wedge_multiVector(other, flatPoint_antiWedge_multiVector(self, flector_antiDual(other)));
}

MultiVector flatPoint_antiProjectOrthogonallyOnto_motor(FlatPoint self, Motor other) {
    return motor_wedge_multiVector(other, flatPoint_antiWedge_multiVector(self, motor_antiDual(other)));
}

MultiVector flatPoint_antiProjectOrthogonallyOnto_multiVector(FlatPoint self, MultiVector other) {
    return multiVector_wedge_multiVector(other, flatPoint_antiWedge_multiVector(self, multiVector_antiDual(other)));
}

Dipole flatPoint_antiProjectOrthogonallyOnto_roundPoint(FlatPoint self, RoundPoint other) {
    return roundPoint_wedge_roundPoint(other, flatPoint_antiWedge_sphere(self, roundPoint_antiDual(other)));
}

Sphere flector_antiProjectOrthogonallyOnto_circle(Flector self, Circle other) {
    return circle_wedge_roundPoint(other, flector_antiWedge_dipole(self, circle_antiDual(other)));
}

MultiVector flector_antiProjectOrthogonallyOnto_dipole(Flector self, Dipole other) {
    return dipole_wedge_multiVector(other, flector_antiWedge_circle(self, dipole_antiDual(other)));
}

MultiVector flector_antiProjectOrthogonallyOnto_flatPoint(Flector self, FlatPoint other) {
    return flatPoint_wedge_multiVector(other, flector_antiWedge_circle(self, flatPoint_antiDual(other)));
}

MultiVector flector_antiProjectOrthogonallyOnto_flector(Flector self, Flector other) {
    return flector_wedge_multiVector(other, flector_antiWedge_multiVector(self, flector_antiDual(other)));
}

Plane flector_antiProjectOrthogonallyOnto_line(Flector self, Line other) {
    return line_wedge_roundPoint(other, flector_antiWedge_dipole(self, line_antiDual(other)));
}

MultiVector flector_antiProjectOrthogonallyOnto_motor(Flector self, Motor other) {
    return motor_wedge_multiVector(other, flector_antiWedge_multiVector(self, motor_antiDual(other)));
}

MultiVector flector_antiProjectOrthogonallyOnto_multiVector(Flector self, MultiVector other) {
    return multiVector_wedge_multiVector(other, flector_antiWedge_multiVector(self, multiVector_antiDual(other)));
}

Plane flector_antiProjectOrthogonallyOnto_plane(Flector self, Plane other) {
    return plane_wedge_scalar(other, flector_antiWedge_roundPoint(self, plane_antiDual(other)));
}

MultiVector flector_antiProjectOrthogonallyOnto_roundPoint(Flector self, RoundPoint other) {
    return roundPoint_wedge_multiVector(other, flector_antiWedge_sphere(self, roundPoint_antiDual(other)));
}

Sphere flector_antiProjectOrthogonallyOnto_sphere(Flector self, Sphere other) {
    return sphere_wedge_scalar(other, flector_antiWedge_roundPoint(self, sphere_antiDual(other)));
}

Circle line_antiProjectOrthogonallyOnto_circle(Line self, Circle other) {
    return circle_wedge_scalar(other, line_antiWedge_dipole(self, circle_antiDual(other)));
}

Circle line_antiProjectOrthogonallyOnto_dipole(Line self, Dipole other) {
    return dipole_wedge_roundPoint(other, line_antiWedge_circle(self, dipole_antiDual(other)));
}

Line line_antiProjectOrthogonallyOnto_flatPoint(Line self, FlatPoint other) {
    return flatPoint_wedge_roundPoint(other, line_antiWedge_circle(self, flatPoint_antiDual(other)));
}

MultiVector line_antiProjectOrthogonallyOnto_flector(Line self, Flector other) {
    return flector_wedge_multiVector(other, line_antiWedge_multiVector(self, flector_antiDual(other)));
}

Line line_antiProjectOrthogonallyOnto_line(Line self, Line other) {
    return line_wedge_scalar(other, line_antiWedge_dipole(self, line_antiDual(other)));
}

MultiVector line_antiProjectOrthogonallyOnto_motor(Line self, Motor other) {
    return motor_wedge_multiVector(other, line_antiWedge_multiVector(self, motor_antiDual(other)));
}

MultiVector line_antiProjectOrthogonallyOnto_multiVector(Line self, MultiVector other) {
    return multiVector_wedge_multiVector(other, line_antiWedge_multiVector(self, multiVector_antiDual(other)));
}

Circle line_antiProjectOrthogonallyOnto_roundPoint(Line self, RoundPoint other) {
    return roundPoint_wedge_dipole(other, line_antiWedge_sphere(self, roundPoint_antiDual(other)));
}

MultiVector motor_antiProjectOrthogonallyOnto_circle(Motor self, Circle other) {
    return circle_wedge_multiVector(other, motor_antiWedge_dipole(self, circle_antiDual(other)));
}

MultiVector motor_antiProjectOrthogonallyOnto_dipole(Motor self, Dipole other) {
    return dipole_wedge_multiVector(other, motor_antiWedge_circle(self, dipole_antiDual(other)));
}

MultiVector motor_antiProjectOrthogonallyOnto_flatPoint(Motor self, FlatPoint other) {
    return flatPoint_wedge_multiVector(other, motor_antiWedge_circle(self, flatPoint_antiDual(other)));
}

MultiVector motor_antiProjectOrthogonallyOnto_flector(Motor self, Flector other) {
    return flector_wedge_multiVector(other, motor_antiWedge_multiVector(self, flector_antiDual(other)));
}

MultiVector motor_antiProjectOrthogonallyOnto_line(Motor self, Line other) {
    return line_wedge_multiVector(other, motor_antiWedge_dipole(self, line_antiDual(other)));
}

MultiVector motor_antiProjectOrthogonallyOnto_motor(Motor self, Motor other) {
    return motor_wedge_multiVector(other, motor_antiWedge_multiVector(self, motor_antiDual(other)));
}

MultiVector motor_antiProjectOrthogonallyOnto_multiVector(Motor self, MultiVector other) {
    return multiVector_wedge_multiVector(other, motor_antiWedge_multiVector(self, multiVector_antiDual(other)));
}

AntiScalar motor_antiProjectOrthogonallyOnto_plane(Motor self, Plane other) {
    return plane_wedge_roundPoint(other, motor_antiWedge_roundPoint(self, plane_antiDual(other)));
}

MultiVector motor_antiProjectOrthogonallyOnto_roundPoint(Motor self, RoundPoint other) {
    return roundPoint_wedge_multiVector(other, motor_antiWedge_sphere(self, roundPoint_antiDual(other)));
}

AntiScalar motor_antiProjectOrthogonallyOnto_sphere(Motor self, Sphere other) {
    return sphere_wedge_roundPoint(other, motor_antiWedge_roundPoint(self, sphere_antiDual(other)));
}

MultiVector multiVector_antiProjectOrthogonallyOnto_circle(MultiVector self, Circle other) {
    return circle_wedge_multiVector(other, multiVector_antiWedge_dipole(self, circle_antiDual(other)));
}

MultiVector multiVector_antiProjectOrthogonallyOnto_dipole(MultiVector self, Dipole other) {
    return dipole_wedge_multiVector(other, multiVector_antiWedge_circle(self, dipole_antiDual(other)));
}

MultiVector multiVector_antiProjectOrthogonallyOnto_flatPoint(MultiVector self, FlatPoint other) {
    return flatPoint_wedge_multiVector(other, multiVector_antiWedge_circle(self, flatPoint_antiDual(other)));
}

MultiVector multiVector_antiProjectOrthogonallyOnto_flector(MultiVector self, Flector other) {
    return flector_wedge_multiVector(other, multiVector_antiWedge_multiVector(self, flector_antiDual(other)));
}

MultiVector multiVector_antiProjectOrthogonallyOnto_line(MultiVector self, Line other) {
    return line_wedge_multiVector(other, multiVector_antiWedge_dipole(self, line_antiDual(other)));
}

MultiVector multiVector_antiProjectOrthogonallyOnto_motor(MultiVector self, Motor other) {
    return motor_wedge_multiVector(other, multiVector_antiWedge_multiVector(self, motor_antiDual(other)));
}

MultiVector multiVector_antiProjectOrthogonallyOnto_multiVector(MultiVector self, MultiVector other) {
    return multiVector_wedge_multiVector(other, multiVector_antiWedge_multiVector(self, multiVector_antiDual(other)));
}

MultiVector multiVector_antiProjectOrthogonallyOnto_plane(MultiVector self, Plane other) {
    return plane_wedge_multiVector(other, multiVector_antiWedge_roundPoint(self, plane_antiDual(other)));
}

MultiVector multiVector_antiProjectOrthogonallyOnto_roundPoint(MultiVector self, RoundPoint other) {
    return roundPoint_wedge_multiVector(other, multiVector_antiWedge_sphere(self, roundPoint_antiDual(other)));
}

MultiVector multiVector_antiProjectOrthogonallyOnto_sphere(MultiVector self, Sphere other) {
    return sphere_wedge_multiVector(other, multiVector_antiWedge_roundPoint(self, sphere_antiDual(other)));
}

Sphere plane_antiProjectOrthogonallyOnto_circle(Plane self, Circle other) {
    return circle_wedge_roundPoint(other, plane_antiWedge_dipole(self, circle_antiDual(other)));
}

Sphere plane_antiProjectOrthogonallyOnto_dipole(Plane self, Dipole other) {
    return dipole_wedge_dipole(other, plane_antiWedge_circle(self, dipole_antiDual(other)));
}

Plane plane_antiProjectOrthogonallyOnto_flatPoint(Plane self, FlatPoint other) {
    return flatPoint_wedge_dipole(other, plane_antiWedge_circle(self, flatPoint_antiDual(other)));
}

MultiVector plane_antiProjectOrthogonallyOnto_flector(Plane self, Flector other) {
    return flector_wedge_multiVector(other, plane_antiWedge_multiVector(self, flector_antiDual(other)));
}

Plane plane_antiProjectOrthogonallyOnto_line(Plane self, Line other) {
    return line_wedge_roundPoint(other, plane_antiWedge_dipole(self, line_antiDual(other)));
}

MultiVector plane_antiProjectOrthogonallyOnto_motor(Plane self, Motor other) {
    return motor_wedge_multiVector(other, plane_antiWedge_multiVector(self, motor_antiDual(other)));
}

MultiVector plane_antiProjectOrthogonallyOnto_multiVector(Plane self, MultiVector other) {
    return multiVector_wedge_multiVector(other, plane_antiWedge_multiVector(self, multiVector_antiDual(other)));
}

Plane plane_antiProjectOrthogonallyOnto_plane(Plane self, Plane other) {
    return plane_wedge_scalar(other, plane_antiWedge_roundPoint(self, plane_antiDual(other)));
}

Sphere plane_antiProjectOrthogonallyOnto_roundPoint(Plane self, RoundPoint other) {
    return roundPoint_wedge_circle(other, plane_antiWedge_sphere(self, roundPoint_antiDual(other)));
}

Sphere plane_antiProjectOrthogonallyOnto_sphere(Plane self, Sphere other) {
    return sphere_wedge_scalar(other, plane_antiWedge_roundPoint(self, sphere_antiDual(other)));
}

MultiVector roundPoint_antiProjectOrthogonallyOnto_flector(RoundPoint self, Flector other) {
    return flector_wedge_multiVector(other, roundPoint_antiWedge_multiVector(self, flector_antiDual(other)));
}

MultiVector roundPoint_antiProjectOrthogonallyOnto_motor(RoundPoint self, Motor other) {
    return motor_wedge_multiVector(other, roundPoint_antiWedge_multiVector(self, motor_antiDual(other)));
}

MultiVector roundPoint_antiProjectOrthogonallyOnto_multiVector(RoundPoint self, MultiVector other) {
    return multiVector_wedge_multiVector(other, roundPoint_antiWedge_multiVector(self, multiVector_antiDual(other)));
}

RoundPoint roundPoint_antiProjectOrthogonallyOnto_roundPoint(RoundPoint self, RoundPoint other) {
    return roundPoint_wedge_scalar(other, roundPoint_antiWedge_sphere(self, roundPoint_antiDual(other)));
}

Sphere sphere_antiProjectOrthogonallyOnto_circle(Sphere self, Circle other) {
    return circle_wedge_roundPoint(other, sphere_antiWedge_dipole(self, circle_antiDual(other)));
}

Sphere sphere_antiProjectOrthogonallyOnto_dipole(Sphere self, Dipole other) {
    return dipole_wedge_dipole(other, sphere_antiWedge_circle(self, dipole_antiDual(other)));
}

Plane sphere_antiProjectOrthogonallyOnto_flatPoint(Sphere self, FlatPoint other) {
    return flatPoint_wedge_dipole(other, sphere_antiWedge_circle(self, flatPoint_antiDual(other)));
}

MultiVector sphere_antiProjectOrthogonallyOnto_flector(Sphere self, Flector other) {
    return flector_wedge_multiVector(other, sphere_antiWedge_multiVector(self, flector_antiDual(other)));
}

Plane sphere_antiProjectOrthogonallyOnto_line(Sphere self, Line other) {
    return line_wedge_roundPoint(other, sphere_antiWedge_dipole(self, line_antiDual(other)));
}

MultiVector sphere_antiProjectOrthogonallyOnto_motor(Sphere self, Motor other) {
    return motor_wedge_multiVector(other, sphere_antiWedge_multiVector(self, motor_antiDual(other)));
}

MultiVector sphere_antiProjectOrthogonallyOnto_multiVector(Sphere self, MultiVector other) {
    return multiVector_wedge_multiVector(other, sphere_antiWedge_multiVector(self, multiVector_antiDual(other)));
}

Plane sphere_antiProjectOrthogonallyOnto_plane(Sphere self, Plane other) {
    return plane_wedge_scalar(other, sphere_antiWedge_roundPoint(self, plane_antiDual(other)));
}

Sphere sphere_antiProjectOrthogonallyOnto_roundPoint(Sphere self, RoundPoint other) {
    return roundPoint_wedge_circle(other, sphere_antiWedge_sphere(self, roundPoint_antiDual(other)));
}

Sphere sphere_antiProjectOrthogonallyOnto_sphere(Sphere self, Sphere other) {
    return sphere_wedge_scalar(other, sphere_antiWedge_roundPoint(self, sphere_antiDual(other)));
}

Circle circle_antiProjectViaHorizonOnto_circle(Circle self, Circle other) {
    return circle_wedge_scalar(other, circle_antiWedge_dipole(self, circle_dual(other)));
}

Circle circle_antiProjectViaHorizonOnto_dipole(Circle self, Dipole other) {
    return dipole_wedge_roundPoint(other, circle_antiWedge_circle(self, dipole_dual(other)));
}

Line circle_antiProjectViaHorizonOnto_flatPoint(Circle self, FlatPoint other) {
    return flatPoint_wedge_roundPoint(other, circle_antiWedge_circle(self, flatPoint_dual(other)));
}

MultiVector circle_antiProjectViaHorizonOnto_flector(Circle self, Flector other) {
    return flector_wedge_multiVector(other, circle_antiWedge_multiVector(self, flector_dual(other)));
}

Line circle_antiProjectViaHorizonOnto_line(Circle self, Line other) {
    return line_wedge_scalar(other, circle_antiWedge_dipole(self, line_dual(other)));
}

MultiVector circle_antiProjectViaHorizonOnto_motor(Circle self, Motor other) {
    return motor_wedge_multiVector(other, circle_antiWedge_multiVector(self, motor_dual(other)));
}

MultiVector circle_antiProjectViaHorizonOnto_multiVector(Circle self, MultiVector other) {
    return multiVector_wedge_multiVector(other, circle_antiWedge_multiVector(self, multiVector_dual(other)));
}

Circle circle_antiProjectViaHorizonOnto_roundPoint(Circle self, RoundPoint other) {
    return roundPoint_wedge_dipole(other, circle_antiWedge_sphere(self, roundPoint_dual(other)));
}

Dipole dipole_antiProjectViaHorizonOnto_dipole(Dipole self, Dipole other) {
    return dipole_wedge_scalar(other, dipole_antiWedge_circle(self, dipole_dual(other)));
}

FlatPoint dipole_antiProjectViaHorizonOnto_flatPoint(Dipole self, FlatPoint other) {
    return flatPoint_wedge_scalar(other, dipole_antiWedge_circle(self, flatPoint_dual(other)));
}

MultiVector dipole_antiProjectViaHorizonOnto_flector(Dipole self, Flector other) {
    return flector_wedge_multiVector(other, dipole_antiWedge_multiVector(self, flector_dual(other)));
}

MultiVector dipole_antiProjectViaHorizonOnto_motor(Dipole self, Motor other) {
    return motor_wedge_multiVector(other, dipole_antiWedge_multiVector(self, motor_dual(other)));
}

MultiVector dipole_antiProjectViaHorizonOnto_multiVector(Dipole self, MultiVector other) {
    return multiVector_wedge_multiVector(other, dipole_antiWedge_multiVector(self, multiVector_dual(other)));
}

Dipole dipole_antiProjectViaHorizonOnto_roundPoint(Dipole self, RoundPoint other) {
    return roundPoint_wedge_roundPoint(other, dipole_antiWedge_sphere(self, roundPoint_dual(other)));
}

Dipole flatPoint_antiProjectViaHorizonOnto_dipole(FlatPoint self, Dipole other) {
    return dipole_wedge_scalar(other, flatPoint_antiWedge_circle(self, dipole_dual(other)));
}

FlatPoint flatPoint_antiProjectViaHorizonOnto_flatPoint(FlatPoint self, FlatPoint other) {
    return flatPoint_wedge_scalar(other, flatPoint_antiWedge_circle(self, flatPoint_dual(other)));
}

MultiVector flatPoint_antiProjectViaHorizonOnto_flector(FlatPoint self, Flector other) {
    return flector_wedge_multiVector(other, flatPoint_antiWedge_multiVector(self, flector_dual(other)));
}

MultiVector flatPoint_antiProjectViaHorizonOnto_motor(FlatPoint self, Motor other) {
    return motor_wedge_multiVector(other, flatPoint_antiWedge_multiVector(self, motor_dual(other)));
}

MultiVector flatPoint_antiProjectViaHorizonOnto_multiVector(FlatPoint self, MultiVector other) {
    return multiVector_wedge_multiVector(other, flatPoint_antiWedge_multiVector(self, multiVector_dual(other)));
}

Dipole flatPoint_antiProjectViaHorizonOnto_roundPoint(FlatPoint self, RoundPoint other) {
    return roundPoint_wedge_roundPoint(other, flatPoint_antiWedge_sphere(self, roundPoint_dual(other)));
}

Sphere flector_antiProjectViaHorizonOnto_circle(Flector self, Circle other) {
    return circle_wedge_roundPoint(other, flector_antiWedge_dipole(self, circle_dual(other)));
}

MultiVector flector_antiProjectViaHorizonOnto_dipole(Flector self, Dipole other) {
    return dipole_wedge_multiVector(other, flector_antiWedge_circle(self, dipole_dual(other)));
}

MultiVector flector_antiProjectViaHorizonOnto_flatPoint(Flector self, FlatPoint other) {
    return flatPoint_wedge_multiVector(other, flector_antiWedge_circle(self, flatPoint_dual(other)));
}

MultiVector flector_antiProjectViaHorizonOnto_flector(Flector self, Flector other) {
    return flector_wedge_multiVector(other, flector_antiWedge_multiVector(self, flector_dual(other)));
}

Plane flector_antiProjectViaHorizonOnto_line(Flector self, Line other) {
    return line_wedge_roundPoint(other, flector_antiWedge_dipole(self, line_dual(other)));
}

MultiVector flector_antiProjectViaHorizonOnto_motor(Flector self, Motor other) {
    return motor_wedge_multiVector(other, flector_antiWedge_multiVector(self, motor_dual(other)));
}

MultiVector flector_antiProjectViaHorizonOnto_multiVector(Flector self, MultiVector other) {
    return multiVector_wedge_multiVector(other, flector_antiWedge_multiVector(self, multiVector_dual(other)));
}

Plane flector_antiProjectViaHorizonOnto_plane(Flector self, Plane other) {
    return plane_wedge_scalar(other, flector_antiWedge_roundPoint(self, plane_dual(other)));
}

MultiVector flector_antiProjectViaHorizonOnto_roundPoint(Flector self, RoundPoint other) {
    return roundPoint_wedge_multiVector(other, flector_antiWedge_sphere(self, roundPoint_dual(other)));
}

Sphere flector_antiProjectViaHorizonOnto_sphere(Flector self, Sphere other) {
    return sphere_wedge_scalar(other, flector_antiWedge_roundPoint(self, sphere_dual(other)));
}

Circle line_antiProjectViaHorizonOnto_circle(Line self, Circle other) {
    return circle_wedge_scalar(other, line_antiWedge_dipole(self, circle_dual(other)));
}

Circle line_antiProjectViaHorizonOnto_dipole(Line self, Dipole other) {
    return dipole_wedge_roundPoint(other, line_antiWedge_circle(self, dipole_dual(other)));
}

Line line_antiProjectViaHorizonOnto_flatPoint(Line self, FlatPoint other) {
    return flatPoint_wedge_roundPoint(other, line_antiWedge_circle(self, flatPoint_dual(other)));
}

MultiVector line_antiProjectViaHorizonOnto_flector(Line self, Flector other) {
    return flector_wedge_multiVector(other, line_antiWedge_multiVector(self, flector_dual(other)));
}

Line line_antiProjectViaHorizonOnto_line(Line self, Line other) {
    return line_wedge_scalar(other, line_antiWedge_dipole(self, line_dual(other)));
}

MultiVector line_antiProjectViaHorizonOnto_motor(Line self, Motor other) {
    return motor_wedge_multiVector(other, line_antiWedge_multiVector(self, motor_dual(other)));
}

MultiVector line_antiProjectViaHorizonOnto_multiVector(Line self, MultiVector other) {
    return multiVector_wedge_multiVector(other, line_antiWedge_multiVector(self, multiVector_dual(other)));
}

Circle line_antiProjectViaHorizonOnto_roundPoint(Line self, RoundPoint other) {
    return roundPoint_wedge_dipole(other, line_antiWedge_sphere(self, roundPoint_dual(other)));
}

MultiVector motor_antiProjectViaHorizonOnto_circle(Motor self, Circle other) {
    return circle_wedge_multiVector(other, motor_antiWedge_dipole(self, circle_dual(other)));
}

MultiVector motor_antiProjectViaHorizonOnto_dipole(Motor self, Dipole other) {
    return dipole_wedge_multiVector(other, motor_antiWedge_circle(self, dipole_dual(other)));
}

MultiVector motor_antiProjectViaHorizonOnto_flatPoint(Motor self, FlatPoint other) {
    return flatPoint_wedge_multiVector(other, motor_antiWedge_circle(self, flatPoint_dual(other)));
}

MultiVector motor_antiProjectViaHorizonOnto_flector(Motor self, Flector other) {
    return flector_wedge_multiVector(other, motor_antiWedge_multiVector(self, flector_dual(other)));
}

MultiVector motor_antiProjectViaHorizonOnto_line(Motor self, Line other) {
    return line_wedge_multiVector(other, motor_antiWedge_dipole(self, line_dual(other)));
}

MultiVector motor_antiProjectViaHorizonOnto_motor(Motor self, Motor other) {
    return motor_wedge_multiVector(other, motor_antiWedge_multiVector(self, motor_dual(other)));
}

MultiVector motor_antiProjectViaHorizonOnto_multiVector(Motor self, MultiVector other) {
    return multiVector_wedge_multiVector(other, motor_antiWedge_multiVector(self, multiVector_dual(other)));
}

AntiScalar motor_antiProjectViaHorizonOnto_plane(Motor self, Plane other) {
    return plane_wedge_roundPoint(other, motor_antiWedge_roundPoint(self, plane_dual(other)));
}

MultiVector motor_antiProjectViaHorizonOnto_roundPoint(Motor self, RoundPoint other) {
    return roundPoint_wedge_multiVector(other, motor_antiWedge_sphere(self, roundPoint_dual(other)));
}

AntiScalar motor_antiProjectViaHorizonOnto_sphere(Motor self, Sphere other) {
    return sphere_wedge_roundPoint(other, motor_antiWedge_roundPoint(self, sphere_dual(other)));
}

MultiVector multiVector_antiProjectViaHorizonOnto_circle(MultiVector self, Circle other) {
    return circle_wedge_multiVector(other, multiVector_antiWedge_dipole(self, circle_dual(other)));
}

MultiVector multiVector_antiProjectViaHorizonOnto_dipole(MultiVector self, Dipole other) {
    return dipole_wedge_multiVector(other, multiVector_antiWedge_circle(self, dipole_dual(other)));
}

MultiVector multiVector_antiProjectViaHorizonOnto_flatPoint(MultiVector self, FlatPoint other) {
    return flatPoint_wedge_multiVector(other, multiVector_antiWedge_circle(self, flatPoint_dual(other)));
}

MultiVector multiVector_antiProjectViaHorizonOnto_flector(MultiVector self, Flector other) {
    return flector_wedge_multiVector(other, multiVector_antiWedge_multiVector(self, flector_dual(other)));
}

MultiVector multiVector_antiProjectViaHorizonOnto_line(MultiVector self, Line other) {
    return line_wedge_multiVector(other, multiVector_antiWedge_dipole(self, line_dual(other)));
}

MultiVector multiVector_antiProjectViaHorizonOnto_motor(MultiVector self, Motor other) {
    return motor_wedge_multiVector(other, multiVector_antiWedge_multiVector(self, motor_dual(other)));
}

MultiVector multiVector_antiProjectViaHorizonOnto_multiVector(MultiVector self, MultiVector other) {
    return multiVector_wedge_multiVector(other, multiVector_antiWedge_multiVector(self, multiVector_dual(other)));
}

MultiVector multiVector_antiProjectViaHorizonOnto_plane(MultiVector self, Plane other) {
    return plane_wedge_multiVector(other, multiVector_antiWedge_roundPoint(self, plane_dual(other)));
}

MultiVector multiVector_antiProjectViaHorizonOnto_roundPoint(MultiVector self, RoundPoint other) {
    return roundPoint_wedge_multiVector(other, multiVector_antiWedge_sphere(self, roundPoint_dual(other)));
}

MultiVector multiVector_antiProjectViaHorizonOnto_sphere(MultiVector self, Sphere other) {
    return sphere_wedge_multiVector(other, multiVector_antiWedge_roundPoint(self, sphere_dual(other)));
}

Sphere plane_antiProjectViaHorizonOnto_circle(Plane self, Circle other) {
    return circle_wedge_roundPoint(other, plane_antiWedge_dipole(self, circle_dual(other)));
}

Sphere plane_antiProjectViaHorizonOnto_dipole(Plane self, Dipole other) {
    return dipole_wedge_dipole(other, plane_antiWedge_circle(self, dipole_dual(other)));
}

Plane plane_antiProjectViaHorizonOnto_flatPoint(Plane self, FlatPoint other) {
    return flatPoint_wedge_dipole(other, plane_antiWedge_circle(self, flatPoint_dual(other)));
}

MultiVector plane_antiProjectViaHorizonOnto_flector(Plane self, Flector other) {
    return flector_wedge_multiVector(other, plane_antiWedge_multiVector(self, flector_dual(other)));
}

Plane plane_antiProjectViaHorizonOnto_line(Plane self, Line other) {
    return line_wedge_roundPoint(other, plane_antiWedge_dipole(self, line_dual(other)));
}

MultiVector plane_antiProjectViaHorizonOnto_motor(Plane self, Motor other) {
    return motor_wedge_multiVector(other, plane_antiWedge_multiVector(self, motor_dual(other)));
}

MultiVector plane_antiProjectViaHorizonOnto_multiVector(Plane self, MultiVector other) {
    return multiVector_wedge_multiVector(other, plane_antiWedge_multiVector(self, multiVector_dual(other)));
}

Plane plane_antiProjectViaHorizonOnto_plane(Plane self, Plane other) {
    return plane_wedge_scalar(other, plane_antiWedge_roundPoint(self, plane_dual(other)));
}

Sphere plane_antiProjectViaHorizonOnto_roundPoint(Plane self, RoundPoint other) {
    return roundPoint_wedge_circle(other, plane_antiWedge_sphere(self, roundPoint_dual(other)));
}

Sphere plane_antiProjectViaHorizonOnto_sphere(Plane self, Sphere other) {
    return sphere_wedge_scalar(other, plane_antiWedge_roundPoint(self, sphere_dual(other)));
}

MultiVector roundPoint_antiProjectViaHorizonOnto_flector(RoundPoint self, Flector other) {
    return flector_wedge_multiVector(other, roundPoint_antiWedge_multiVector(self, flector_dual(other)));
}

MultiVector roundPoint_antiProjectViaHorizonOnto_motor(RoundPoint self, Motor other) {
    return motor_wedge_multiVector(other, roundPoint_antiWedge_multiVector(self, motor_dual(other)));
}

MultiVector roundPoint_antiProjectViaHorizonOnto_multiVector(RoundPoint self, MultiVector other) {
    return multiVector_wedge_multiVector(other, roundPoint_antiWedge_multiVector(self, multiVector_dual(other)));
}

RoundPoint roundPoint_antiProjectViaHorizonOnto_roundPoint(RoundPoint self, RoundPoint other) {
    return roundPoint_wedge_scalar(other, roundPoint_antiWedge_sphere(self, roundPoint_dual(other)));
}

Sphere sphere_antiProjectViaHorizonOnto_circle(Sphere self, Circle other) {
    return circle_wedge_roundPoint(other, sphere_antiWedge_dipole(self, circle_dual(other)));
}

Sphere sphere_antiProjectViaHorizonOnto_dipole(Sphere self, Dipole other) {
    return dipole_wedge_dipole(other, sphere_antiWedge_circle(self, dipole_dual(other)));
}

Plane sphere_antiProjectViaHorizonOnto_flatPoint(Sphere self, FlatPoint other) {
    return flatPoint_wedge_dipole(other, sphere_antiWedge_circle(self, flatPoint_dual(other)));
}

MultiVector sphere_antiProjectViaHorizonOnto_flector(Sphere self, Flector other) {
    return flector_wedge_multiVector(other, sphere_antiWedge_multiVector(self, flector_dual(other)));
}

Plane sphere_antiProjectViaHorizonOnto_line(Sphere self, Line other) {
    return line_wedge_roundPoint(other, sphere_antiWedge_dipole(self, line_dual(other)));
}

MultiVector sphere_antiProjectViaHorizonOnto_motor(Sphere self, Motor other) {
    return motor_wedge_multiVector(other, sphere_antiWedge_multiVector(self, motor_dual(other)));
}

MultiVector sphere_antiProjectViaHorizonOnto_multiVector(Sphere self, MultiVector other) {
    return multiVector_wedge_multiVector(other, sphere_antiWedge_multiVector(self, multiVector_dual(other)));
}

Plane sphere_antiProjectViaHorizonOnto_plane(Sphere self, Plane other) {
    return plane_wedge_scalar(other, sphere_antiWedge_roundPoint(self, plane_dual(other)));
}

Sphere sphere_antiProjectViaHorizonOnto_roundPoint(Sphere self, RoundPoint other) {
    return roundPoint_wedge_circle(other, sphere_antiWedge_sphere(self, roundPoint_dual(other)));
}

Sphere sphere_antiProjectViaHorizonOnto_sphere(Sphere self, Sphere other) {
    return sphere_wedge_scalar(other, sphere_antiWedge_roundPoint(self, sphere_dual(other)));
}

Circle circle_projectOrthogonallyOnto_circle(Circle self, Circle other) {
    return circle_antiWedge_antiScalar(other, circle_wedge_dipole(self, circle_antiDual(other)));
}

MultiVector circle_projectOrthogonallyOnto_flector(Circle self, Flector other) {
    return flector_antiWedge_multiVector(other, circle_wedge_multiVector(self, flector_antiDual(other)));
}

Line circle_projectOrthogonallyOnto_line(Circle self, Line other) {
    return line_antiWedge_antiScalar(other, circle_wedge_dipole(self, line_antiDual(other)));
}

MultiVector circle_projectOrthogonallyOnto_motor(Circle self, Motor other) {
    return motor_antiWedge_multiVector(other, circle_wedge_multiVector(self, motor_antiDual(other)));
}

MultiVector circle_projectOrthogonallyOnto_multiVector(Circle self, MultiVector other) {
    return multiVector_antiWedge_multiVector(other, circle_wedge_multiVector(self, multiVector_antiDual(other)));
}

Circle circle_projectOrthogonallyOnto_plane(Circle self, Plane other) {
    return plane_antiWedge_sphere(other, circle_wedge_roundPoint(self, plane_antiDual(other)));
}

Circle circle_projectOrthogonallyOnto_sphere(Circle self, Sphere other) {
    return sphere_antiWedge_sphere(other, circle_wedge_roundPoint(self, sphere_antiDual(other)));
}

Dipole dipole_projectOrthogonallyOnto_circle(Dipole self, Circle other) {
    return circle_antiWedge_sphere(other, dipole_wedge_dipole(self, circle_antiDual(other)));
}

Dipole dipole_projectOrthogonallyOnto_dipole(Dipole self, Dipole other) {
    return dipole_antiWedge_antiScalar(other, dipole_wedge_circle(self, dipole_antiDual(other)));
}

FlatPoint dipole_projectOrthogonallyOnto_flatPoint(Dipole self, FlatPoint other) {
    return flatPoint_antiWedge_antiScalar(other, dipole_wedge_circle(self, flatPoint_antiDual(other)));
}

MultiVector dipole_projectOrthogonallyOnto_flector(Dipole self, Flector other) {
    return flector_antiWedge_multiVector(other, dipole_wedge_multiVector(self, flector_antiDual(other)));
}

Dipole dipole_projectOrthogonallyOnto_line(Dipole self, Line other) {
    return line_antiWedge_sphere(other, dipole_wedge_dipole(self, line_antiDual(other)));
}

MultiVector dipole_projectOrthogonallyOnto_motor(Dipole self, Motor other) {
    return motor_antiWedge_multiVector(other, dipole_wedge_multiVector(self, motor_antiDual(other)));
}

MultiVector dipole_projectOrthogonallyOnto_multiVector(Dipole self, MultiVector other) {
    return multiVector_antiWedge_multiVector(other, dipole_wedge_multiVector(self, multiVector_antiDual(other)));
}

Dipole dipole_projectOrthogonallyOnto_plane(Dipole self, Plane other) {
    return plane_antiWedge_circle(other, dipole_wedge_roundPoint(self, plane_antiDual(other)));
}

Dipole dipole_projectOrthogonallyOnto_sphere(Dipole self, Sphere other) {
    return sphere_antiWedge_circle(other, dipole_wedge_roundPoint(self, sphere_antiDual(other)));
}

Dipole flatPoint_projectOrthogonallyOnto_circle(FlatPoint self, Circle other) {
    return circle_antiWedge_plane(other, flatPoint_wedge_dipole(self, circle_antiDual(other)));
}

Dipole flatPoint_projectOrthogonallyOnto_dipole(FlatPoint self, Dipole other) {
    return dipole_antiWedge_antiScalar(other, flatPoint_wedge_circle(self, dipole_antiDual(other)));
}

FlatPoint flatPoint_projectOrthogonallyOnto_flatPoint(FlatPoint self, FlatPoint other) {
    return flatPoint_antiWedge_antiScalar(other, flatPoint_wedge_circle(self, flatPoint_antiDual(other)));
}

MultiVector flatPoint_projectOrthogonallyOnto_flector(FlatPoint self, Flector other) {
    return flector_antiWedge_multiVector(other, flatPoint_wedge_multiVector(self, flector_antiDual(other)));
}

FlatPoint flatPoint_projectOrthogonallyOnto_line(FlatPoint self, Line other) {
    return line_antiWedge_plane(other, flatPoint_wedge_dipole(self, line_antiDual(other)));
}

MultiVector flatPoint_projectOrthogonallyOnto_motor(FlatPoint self, Motor other) {
    return motor_antiWedge_multiVector(other, flatPoint_wedge_multiVector(self, motor_antiDual(other)));
}

MultiVector flatPoint_projectOrthogonallyOnto_multiVector(FlatPoint self, MultiVector other) {
    return multiVector_antiWedge_multiVector(other, flatPoint_wedge_multiVector(self, multiVector_antiDual(other)));
}

FlatPoint flatPoint_projectOrthogonallyOnto_plane(FlatPoint self, Plane other) {
    return plane_antiWedge_line(other, flatPoint_wedge_roundPoint(self, plane_antiDual(other)));
}

Dipole flatPoint_projectOrthogonallyOnto_sphere(FlatPoint self, Sphere other) {
    return sphere_antiWedge_line(other, flatPoint_wedge_roundPoint(self, sphere_antiDual(other)));
}

Dipole flector_projectOrthogonallyOnto_circle(Flector self, Circle other) {
    return circle_antiWedge_plane(other, flector_wedge_dipole(self, circle_antiDual(other)));
}

Dipole flector_projectOrthogonallyOnto_dipole(Flector self, Dipole other) {
    return dipole_antiWedge_antiScalar(other, flector_wedge_circle(self, dipole_antiDual(other)));
}

FlatPoint flector_projectOrthogonallyOnto_flatPoint(Flector self, FlatPoint other) {
    return flatPoint_antiWedge_antiScalar(other, flector_wedge_circle(self, flatPoint_antiDual(other)));
}

MultiVector flector_projectOrthogonallyOnto_flector(Flector self, Flector other) {
    return flector_antiWedge_multiVector(other, flector_wedge_multiVector(self, flector_antiDual(other)));
}

FlatPoint flector_projectOrthogonallyOnto_line(Flector self, Line other) {
    return line_antiWedge_plane(other, flector_wedge_dipole(self, line_antiDual(other)));
}

MultiVector flector_projectOrthogonallyOnto_motor(Flector self, Motor other) {
    return motor_antiWedge_multiVector(other, flector_wedge_multiVector(self, motor_antiDual(other)));
}

MultiVector flector_projectOrthogonallyOnto_multiVector(Flector self, MultiVector other) {
    return multiVector_antiWedge_multiVector(other, flector_wedge_multiVector(self, multiVector_antiDual(other)));
}

Flector flector_projectOrthogonallyOnto_plane(Flector self, Plane other) {
    return plane_antiWedge_motor(other, flector_wedge_roundPoint(self, plane_antiDual(other)));
}

MultiVector flector_projectOrthogonallyOnto_sphere(Flector self, Sphere other) {
    return sphere_antiWedge_motor(other, flector_wedge_roundPoint(self, sphere_antiDual(other)));
}

Circle line_projectOrthogonallyOnto_circle(Line self, Circle other) {
    return circle_antiWedge_antiScalar(other, line_wedge_dipole(self, circle_antiDual(other)));
}

MultiVector line_projectOrthogonallyOnto_flector(Line self, Flector other) {
    return flector_antiWedge_multiVector(other, line_wedge_multiVector(self, flector_antiDual(other)));
}

Line line_projectOrthogonallyOnto_line(Line self, Line other) {
    return line_antiWedge_antiScalar(other, line_wedge_dipole(self, line_antiDual(other)));
}

MultiVector line_projectOrthogonallyOnto_motor(Line self, Motor other) {
    return motor_antiWedge_multiVector(other, line_wedge_multiVector(self, motor_antiDual(other)));
}

MultiVector line_projectOrthogonallyOnto_multiVector(Line self, MultiVector other) {
    return multiVector_antiWedge_multiVector(other, line_wedge_multiVector(self, multiVector_antiDual(other)));
}

Line line_projectOrthogonallyOnto_plane(Line self, Plane other) {
    return plane_antiWedge_plane(other, line_wedge_roundPoint(self, plane_antiDual(other)));
}

Circle line_projectOrthogonallyOnto_sphere(Line self, Sphere other) {
    return sphere_antiWedge_plane(other, line_wedge_roundPoint(self, sphere_antiDual(other)));
}

Circle motor_projectOrthogonallyOnto_circle(Motor self, Circle other) {
    return circle_antiWedge_antiScalar(other, motor_wedge_dipole(self, circle_antiDual(other)));
}

MultiVector motor_projectOrthogonallyOnto_flector(Motor self, Flector other) {
    return flector_antiWedge_multiVector(other, motor_wedge_multiVector(self, flector_antiDual(other)));
}

Line motor_projectOrthogonallyOnto_line(Motor self, Line other) {
    return line_antiWedge_antiScalar(other, motor_wedge_dipole(self, line_antiDual(other)));
}

MultiVector motor_projectOrthogonallyOnto_motor(Motor self, Motor other) {
    return motor_antiWedge_multiVector(other, motor_wedge_multiVector(self, motor_antiDual(other)));
}

MultiVector motor_projectOrthogonallyOnto_multiVector(Motor self, MultiVector other) {
    return multiVector_antiWedge_multiVector(other, motor_wedge_multiVector(self, multiVector_antiDual(other)));
}

Line motor_projectOrthogonallyOnto_plane(Motor self, Plane other) {
    return plane_antiWedge_plane(other, motor_wedge_roundPoint(self, plane_antiDual(other)));
}

Circle motor_projectOrthogonallyOnto_sphere(Motor self, Sphere other) {
    return sphere_antiWedge_plane(other, motor_wedge_roundPoint(self, sphere_antiDual(other)));
}

MultiVector multiVector_projectOrthogonallyOnto_circle(MultiVector self, Circle other) {
    return circle_antiWedge_multiVector(other, multiVector_wedge_dipole(self, circle_antiDual(other)));
}

MultiVector multiVector_projectOrthogonallyOnto_dipole(MultiVector self, Dipole other) {
    return dipole_antiWedge_multiVector(other, multiVector_wedge_circle(self, dipole_antiDual(other)));
}

MultiVector multiVector_projectOrthogonallyOnto_flatPoint(MultiVector self, FlatPoint other) {
    return flatPoint_antiWedge_multiVector(other, multiVector_wedge_circle(self, flatPoint_antiDual(other)));
}

MultiVector multiVector_projectOrthogonallyOnto_flector(MultiVector self, Flector other) {
    return flector_antiWedge_multiVector(other, multiVector_wedge_multiVector(self, flector_antiDual(other)));
}

MultiVector multiVector_projectOrthogonallyOnto_line(MultiVector self, Line other) {
    return line_antiWedge_multiVector(other, multiVector_wedge_dipole(self, line_antiDual(other)));
}

MultiVector multiVector_projectOrthogonallyOnto_motor(MultiVector self, Motor other) {
    return motor_antiWedge_multiVector(other, multiVector_wedge_multiVector(self, motor_antiDual(other)));
}

MultiVector multiVector_projectOrthogonallyOnto_multiVector(MultiVector self, MultiVector other) {
    return multiVector_antiWedge_multiVector(other, multiVector_wedge_multiVector(self, multiVector_antiDual(other)));
}

MultiVector multiVector_projectOrthogonallyOnto_plane(MultiVector self, Plane other) {
    return plane_antiWedge_multiVector(other, multiVector_wedge_roundPoint(self, plane_antiDual(other)));
}

MultiVector multiVector_projectOrthogonallyOnto_roundPoint(MultiVector self, RoundPoint other) {
    return roundPoint_antiWedge_multiVector(other, multiVector_wedge_sphere(self, roundPoint_antiDual(other)));
}

MultiVector multiVector_projectOrthogonallyOnto_sphere(MultiVector self, Sphere other) {
    return sphere_antiWedge_multiVector(other, multiVector_wedge_roundPoint(self, sphere_antiDual(other)));
}

MultiVector plane_projectOrthogonallyOnto_flector(Plane self, Flector other) {
    return flector_antiWedge_multiVector(other, plane_wedge_multiVector(self, flector_antiDual(other)));
}

MultiVector plane_projectOrthogonallyOnto_motor(Plane self, Motor other) {
    return motor_antiWedge_multiVector(other, plane_wedge_multiVector(self, motor_antiDual(other)));
}

MultiVector plane_projectOrthogonallyOnto_multiVector(Plane self, MultiVector other) {
    return multiVector_antiWedge_multiVector(other, plane_wedge_multiVector(self, multiVector_antiDual(other)));
}

Plane plane_projectOrthogonallyOnto_plane(Plane self, Plane other) {
    return plane_antiWedge_antiScalar(other, plane_wedge_roundPoint(self, plane_antiDual(other)));
}

Sphere plane_projectOrthogonallyOnto_sphere(Plane self, Sphere other) {
    return sphere_antiWedge_antiScalar(other, plane_wedge_roundPoint(self, sphere_antiDual(other)));
}

RoundPoint roundPoint_projectOrthogonallyOnto_circle(RoundPoint self, Circle other) {
    return circle_antiWedge_circle(other, roundPoint_wedge_dipole(self, circle_antiDual(other)));
}

RoundPoint roundPoint_projectOrthogonallyOnto_dipole(RoundPoint self, Dipole other) {
    return dipole_antiWedge_sphere(other, roundPoint_wedge_circle(self, dipole_antiDual(other)));
}

RoundPoint roundPoint_projectOrthogonallyOnto_flatPoint(RoundPoint self, FlatPoint other) {
    return flatPoint_antiWedge_sphere(other, roundPoint_wedge_circle(self, flatPoint_antiDual(other)));
}

MultiVector roundPoint_projectOrthogonallyOnto_flector(RoundPoint self, Flector other) {
    return flector_antiWedge_multiVector(other, roundPoint_wedge_multiVector(self, flector_antiDual(other)));
}

RoundPoint roundPoint_projectOrthogonallyOnto_line(RoundPoint self, Line other) {
    return line_antiWedge_circle(other, roundPoint_wedge_dipole(self, line_antiDual(other)));
}

MultiVector roundPoint_projectOrthogonallyOnto_motor(RoundPoint self, Motor other) {
    return motor_antiWedge_multiVector(other, roundPoint_wedge_multiVector(self, motor_antiDual(other)));
}

MultiVector roundPoint_projectOrthogonallyOnto_multiVector(RoundPoint self, MultiVector other) {
    return multiVector_antiWedge_multiVector(other, roundPoint_wedge_multiVector(self, multiVector_antiDual(other)));
}

RoundPoint roundPoint_projectOrthogonallyOnto_plane(RoundPoint self, Plane other) {
    return plane_antiWedge_dipole(other, roundPoint_wedge_roundPoint(self, plane_antiDual(other)));
}

RoundPoint roundPoint_projectOrthogonallyOnto_roundPoint(RoundPoint self, RoundPoint other) {
    return roundPoint_antiWedge_antiScalar(other, roundPoint_wedge_sphere(self, roundPoint_antiDual(other)));
}

RoundPoint roundPoint_projectOrthogonallyOnto_sphere(RoundPoint self, Sphere other) {
    return sphere_antiWedge_dipole(other, roundPoint_wedge_roundPoint(self, sphere_antiDual(other)));
}

MultiVector sphere_projectOrthogonallyOnto_flector(Sphere self, Flector other) {
    return flector_antiWedge_multiVector(other, sphere_wedge_multiVector(self, flector_antiDual(other)));
}

MultiVector sphere_projectOrthogonallyOnto_motor(Sphere self, Motor other) {
    return motor_antiWedge_multiVector(other, sphere_wedge_multiVector(self, motor_antiDual(other)));
}

MultiVector sphere_projectOrthogonallyOnto_multiVector(Sphere self, MultiVector other) {
    return multiVector_antiWedge_multiVector(other, sphere_wedge_multiVector(self, multiVector_antiDual(other)));
}

Plane sphere_projectOrthogonallyOnto_plane(Sphere self, Plane other) {
    return plane_antiWedge_antiScalar(other, sphere_wedge_roundPoint(self, plane_antiDual(other)));
}

Sphere sphere_projectOrthogonallyOnto_sphere(Sphere self, Sphere other) {
    return sphere_antiWedge_antiScalar(other, sphere_wedge_roundPoint(self, sphere_antiDual(other)));
}

Circle circle_projectViaOriginOnto_circle(Circle self, Circle other) {
    return circle_antiWedge_antiScalar(other, circle_wedge_dipole(self, circle_dual(other)));
}

MultiVector circle_projectViaOriginOnto_flector(Circle self, Flector other) {
    return flector_antiWedge_multiVector(other, circle_wedge_multiVector(self, flector_dual(other)));
}

Line circle_projectViaOriginOnto_line(Circle self, Line other) {
    return line_antiWedge_antiScalar(other, circle_wedge_dipole(self, line_dual(other)));
}

MultiVector circle_projectViaOriginOnto_motor(Circle self, Motor other) {
    return motor_antiWedge_multiVector(other, circle_wedge_multiVector(self, motor_dual(other)));
}

MultiVector circle_projectViaOriginOnto_multiVector(Circle self, MultiVector other) {
    return multiVector_antiWedge_multiVector(other, circle_wedge_multiVector(self, multiVector_dual(other)));
}

Circle circle_projectViaOriginOnto_plane(Circle self, Plane other) {
    return plane_antiWedge_sphere(other, circle_wedge_roundPoint(self, plane_dual(other)));
}

Circle circle_projectViaOriginOnto_sphere(Circle self, Sphere other) {
    return sphere_antiWedge_sphere(other, circle_wedge_roundPoint(self, sphere_dual(other)));
}

Dipole dipole_projectViaOriginOnto_circle(Dipole self, Circle other) {
    return circle_antiWedge_sphere(other, dipole_wedge_dipole(self, circle_dual(other)));
}

Dipole dipole_projectViaOriginOnto_dipole(Dipole self, Dipole other) {
    return dipole_antiWedge_antiScalar(other, dipole_wedge_circle(self, dipole_dual(other)));
}

FlatPoint dipole_projectViaOriginOnto_flatPoint(Dipole self, FlatPoint other) {
    return flatPoint_antiWedge_antiScalar(other, dipole_wedge_circle(self, flatPoint_dual(other)));
}

MultiVector dipole_projectViaOriginOnto_flector(Dipole self, Flector other) {
    return flector_antiWedge_multiVector(other, dipole_wedge_multiVector(self, flector_dual(other)));
}

Dipole dipole_projectViaOriginOnto_line(Dipole self, Line other) {
    return line_antiWedge_sphere(other, dipole_wedge_dipole(self, line_dual(other)));
}

MultiVector dipole_projectViaOriginOnto_motor(Dipole self, Motor other) {
    return motor_antiWedge_multiVector(other, dipole_wedge_multiVector(self, motor_dual(other)));
}

MultiVector dipole_projectViaOriginOnto_multiVector(Dipole self, MultiVector other) {
    return multiVector_antiWedge_multiVector(other, dipole_wedge_multiVector(self, multiVector_dual(other)));
}

Dipole dipole_projectViaOriginOnto_plane(Dipole self, Plane other) {
    return plane_antiWedge_circle(other, dipole_wedge_roundPoint(self, plane_dual(other)));
}

Dipole dipole_projectViaOriginOnto_sphere(Dipole self, Sphere other) {
    return sphere_antiWedge_circle(other, dipole_wedge_roundPoint(self, sphere_dual(other)));
}

Dipole flatPoint_projectViaOriginOnto_circle(FlatPoint self, Circle other) {
    return circle_antiWedge_plane(other, flatPoint_wedge_dipole(self, circle_dual(other)));
}

Dipole flatPoint_projectViaOriginOnto_dipole(FlatPoint self, Dipole other) {
    return dipole_antiWedge_antiScalar(other, flatPoint_wedge_circle(self, dipole_dual(other)));
}

FlatPoint flatPoint_projectViaOriginOnto_flatPoint(FlatPoint self, FlatPoint other) {
    return flatPoint_antiWedge_antiScalar(other, flatPoint_wedge_circle(self, flatPoint_dual(other)));
}

MultiVector flatPoint_projectViaOriginOnto_flector(FlatPoint self, Flector other) {
    return flector_antiWedge_multiVector(other, flatPoint_wedge_multiVector(self, flector_dual(other)));
}

FlatPoint flatPoint_projectViaOriginOnto_line(FlatPoint self, Line other) {
    return line_antiWedge_plane(other, flatPoint_wedge_dipole(self, line_dual(other)));
}

MultiVector flatPoint_projectViaOriginOnto_motor(FlatPoint self, Motor other) {
    return motor_antiWedge_multiVector(other, flatPoint_wedge_multiVector(self, motor_dual(other)));
}

MultiVector flatPoint_projectViaOriginOnto_multiVector(FlatPoint self, MultiVector other) {
    return multiVector_antiWedge_multiVector(other, flatPoint_wedge_multiVector(self, multiVector_dual(other)));
}

FlatPoint flatPoint_projectViaOriginOnto_plane(FlatPoint self, Plane other) {
    return plane_antiWedge_line(other, flatPoint_wedge_roundPoint(self, plane_dual(other)));
}

Dipole flatPoint_projectViaOriginOnto_sphere(FlatPoint self, Sphere other) {
    return sphere_antiWedge_line(other, flatPoint_wedge_roundPoint(self, sphere_dual(other)));
}

Dipole flector_projectViaOriginOnto_circle(Flector self, Circle other) {
    return circle_antiWedge_plane(other, flector_wedge_dipole(self, circle_dual(other)));
}

Dipole flector_projectViaOriginOnto_dipole(Flector self, Dipole other) {
    return dipole_antiWedge_antiScalar(other, flector_wedge_circle(self, dipole_dual(other)));
}

FlatPoint flector_projectViaOriginOnto_flatPoint(Flector self, FlatPoint other) {
    return flatPoint_antiWedge_antiScalar(other, flector_wedge_circle(self, flatPoint_dual(other)));
}

MultiVector flector_projectViaOriginOnto_flector(Flector self, Flector other) {
    return flector_antiWedge_multiVector(other, flector_wedge_multiVector(self, flector_dual(other)));
}

FlatPoint flector_projectViaOriginOnto_line(Flector self, Line other) {
    return line_antiWedge_plane(other, flector_wedge_dipole(self, line_dual(other)));
}

MultiVector flector_projectViaOriginOnto_motor(Flector self, Motor other) {
    return motor_antiWedge_multiVector(other, flector_wedge_multiVector(self, motor_dual(other)));
}

MultiVector flector_projectViaOriginOnto_multiVector(Flector self, MultiVector other) {
    return multiVector_antiWedge_multiVector(other, flector_wedge_multiVector(self, multiVector_dual(other)));
}

Flector flector_projectViaOriginOnto_plane(Flector self, Plane other) {
    return plane_antiWedge_motor(other, flector_wedge_roundPoint(self, plane_dual(other)));
}

MultiVector flector_projectViaOriginOnto_sphere(Flector self, Sphere other) {
    return sphere_antiWedge_motor(other, flector_wedge_roundPoint(self, sphere_dual(other)));
}

Circle line_projectViaOriginOnto_circle(Line self, Circle other) {
    return circle_antiWedge_antiScalar(other, line_wedge_dipole(self, circle_dual(other)));
}

MultiVector line_projectViaOriginOnto_flector(Line self, Flector other) {
    return flector_antiWedge_multiVector(other, line_wedge_multiVector(self, flector_dual(other)));
}

Line line_projectViaOriginOnto_line(Line self, Line other) {
    return line_antiWedge_antiScalar(other, line_wedge_dipole(self, line_dual(other)));
}

MultiVector line_projectViaOriginOnto_motor(Line self, Motor other) {
    return motor_antiWedge_multiVector(other, line_wedge_multiVector(self, motor_dual(other)));
}

MultiVector line_projectViaOriginOnto_multiVector(Line self, MultiVector other) {
    return multiVector_antiWedge_multiVector(other, line_wedge_multiVector(self, multiVector_dual(other)));
}

Line line_projectViaOriginOnto_plane(Line self, Plane other) {
    return plane_antiWedge_plane(other, line_wedge_roundPoint(self, plane_dual(other)));
}

Circle line_projectViaOriginOnto_sphere(Line self, Sphere other) {
    return sphere_antiWedge_plane(other, line_wedge_roundPoint(self, sphere_dual(other)));
}

Circle motor_projectViaOriginOnto_circle(Motor self, Circle other) {
    return circle_antiWedge_antiScalar(other, motor_wedge_dipole(self, circle_dual(other)));
}

MultiVector motor_projectViaOriginOnto_flector(Motor self, Flector other) {
    return flector_antiWedge_multiVector(other, motor_wedge_multiVector(self, flector_dual(other)));
}

Line motor_projectViaOriginOnto_line(Motor self, Line other) {
    return line_antiWedge_antiScalar(other, motor_wedge_dipole(self, line_dual(other)));
}

MultiVector motor_projectViaOriginOnto_motor(Motor self, Motor other) {
    return motor_antiWedge_multiVector(other, motor_wedge_multiVector(self, motor_dual(other)));
}

MultiVector motor_projectViaOriginOnto_multiVector(Motor self, MultiVector other) {
    return multiVector_antiWedge_multiVector(other, motor_wedge_multiVector(self, multiVector_dual(other)));
}

Line motor_projectViaOriginOnto_plane(Motor self, Plane other) {
    return plane_antiWedge_plane(other, motor_wedge_roundPoint(self, plane_dual(other)));
}

Circle motor_projectViaOriginOnto_sphere(Motor self, Sphere other) {
    return sphere_antiWedge_plane(other, motor_wedge_roundPoint(self, sphere_dual(other)));
}

MultiVector multiVector_projectViaOriginOnto_circle(MultiVector self, Circle other) {
    return circle_antiWedge_multiVector(other, multiVector_wedge_dipole(self, circle_dual(other)));
}

MultiVector multiVector_projectViaOriginOnto_dipole(MultiVector self, Dipole other) {
    return dipole_antiWedge_multiVector(other, multiVector_wedge_circle(self, dipole_dual(other)));
}

MultiVector multiVector_projectViaOriginOnto_flatPoint(MultiVector self, FlatPoint other) {
    return flatPoint_antiWedge_multiVector(other, multiVector_wedge_circle(self, flatPoint_dual(other)));
}

MultiVector multiVector_projectViaOriginOnto_flector(MultiVector self, Flector other) {
    return flector_antiWedge_multiVector(other, multiVector_wedge_multiVector(self, flector_dual(other)));
}

MultiVector multiVector_projectViaOriginOnto_line(MultiVector self, Line other) {
    return line_antiWedge_multiVector(other, multiVector_wedge_dipole(self, line_dual(other)));
}

MultiVector multiVector_projectViaOriginOnto_motor(MultiVector self, Motor other) {
    return motor_antiWedge_multiVector(other, multiVector_wedge_multiVector(self, motor_dual(other)));
}

MultiVector multiVector_projectViaOriginOnto_multiVector(MultiVector self, MultiVector other) {
    return multiVector_antiWedge_multiVector(other, multiVector_wedge_multiVector(self, multiVector_dual(other)));
}

MultiVector multiVector_projectViaOriginOnto_plane(MultiVector self, Plane other) {
    return plane_antiWedge_multiVector(other, multiVector_wedge_roundPoint(self, plane_dual(other)));
}

MultiVector multiVector_projectViaOriginOnto_roundPoint(MultiVector self, RoundPoint other) {
    return roundPoint_antiWedge_multiVector(other, multiVector_wedge_sphere(self, roundPoint_dual(other)));
}

MultiVector multiVector_projectViaOriginOnto_sphere(MultiVector self, Sphere other) {
    return sphere_antiWedge_multiVector(other, multiVector_wedge_roundPoint(self, sphere_dual(other)));
}

MultiVector plane_projectViaOriginOnto_flector(Plane self, Flector other) {
    return flector_antiWedge_multiVector(other, plane_wedge_multiVector(self, flector_dual(other)));
}

MultiVector plane_projectViaOriginOnto_motor(Plane self, Motor other) {
    return motor_antiWedge_multiVector(other, plane_wedge_multiVector(self, motor_dual(other)));
}

MultiVector plane_projectViaOriginOnto_multiVector(Plane self, MultiVector other) {
    return multiVector_antiWedge_multiVector(other, plane_wedge_multiVector(self, multiVector_dual(other)));
}

Plane plane_projectViaOriginOnto_plane(Plane self, Plane other) {
    return plane_antiWedge_antiScalar(other, plane_wedge_roundPoint(self, plane_dual(other)));
}

Sphere plane_projectViaOriginOnto_sphere(Plane self, Sphere other) {
    return sphere_antiWedge_antiScalar(other, plane_wedge_roundPoint(self, sphere_dual(other)));
}

RoundPoint roundPoint_projectViaOriginOnto_circle(RoundPoint self, Circle other) {
    return circle_antiWedge_circle(other, roundPoint_wedge_dipole(self, circle_dual(other)));
}

RoundPoint roundPoint_projectViaOriginOnto_dipole(RoundPoint self, Dipole other) {
    return dipole_antiWedge_sphere(other, roundPoint_wedge_circle(self, dipole_dual(other)));
}

RoundPoint roundPoint_projectViaOriginOnto_flatPoint(RoundPoint self, FlatPoint other) {
    return flatPoint_antiWedge_sphere(other, roundPoint_wedge_circle(self, flatPoint_dual(other)));
}

MultiVector roundPoint_projectViaOriginOnto_flector(RoundPoint self, Flector other) {
    return flector_antiWedge_multiVector(other, roundPoint_wedge_multiVector(self, flector_dual(other)));
}

RoundPoint roundPoint_projectViaOriginOnto_line(RoundPoint self, Line other) {
    return line_antiWedge_circle(other, roundPoint_wedge_dipole(self, line_dual(other)));
}

MultiVector roundPoint_projectViaOriginOnto_motor(RoundPoint self, Motor other) {
    return motor_antiWedge_multiVector(other, roundPoint_wedge_multiVector(self, motor_dual(other)));
}

MultiVector roundPoint_projectViaOriginOnto_multiVector(RoundPoint self, MultiVector other) {
    return multiVector_antiWedge_multiVector(other, roundPoint_wedge_multiVector(self, multiVector_dual(other)));
}

RoundPoint roundPoint_projectViaOriginOnto_plane(RoundPoint self, Plane other) {
    return plane_antiWedge_dipole(other, roundPoint_wedge_roundPoint(self, plane_dual(other)));
}

RoundPoint roundPoint_projectViaOriginOnto_roundPoint(RoundPoint self, RoundPoint other) {
    return roundPoint_antiWedge_antiScalar(other, roundPoint_wedge_sphere(self, roundPoint_dual(other)));
}

RoundPoint roundPoint_projectViaOriginOnto_sphere(RoundPoint self, Sphere other) {
    return sphere_antiWedge_dipole(other, roundPoint_wedge_roundPoint(self, sphere_dual(other)));
}

MultiVector sphere_projectViaOriginOnto_flector(Sphere self, Flector other) {
    return flector_antiWedge_multiVector(other, sphere_wedge_multiVector(self, flector_dual(other)));
}

MultiVector sphere_projectViaOriginOnto_motor(Sphere self, Motor other) {
    return motor_antiWedge_multiVector(other, sphere_wedge_multiVector(self, motor_dual(other)));
}

MultiVector sphere_projectViaOriginOnto_multiVector(Sphere self, MultiVector other) {
    return multiVector_antiWedge_multiVector(other, sphere_wedge_multiVector(self, multiVector_dual(other)));
}

Plane sphere_projectViaOriginOnto_plane(Sphere self, Plane other) {
    return plane_antiWedge_antiScalar(other, sphere_wedge_roundPoint(self, plane_dual(other)));
}

Sphere sphere_projectViaOriginOnto_sphere(Sphere self, Sphere other) {
    return sphere_antiWedge_antiScalar(other, sphere_wedge_roundPoint(self, sphere_dual(other)));
}

Circle circle_antiRejectOrthogonallyFrom_dipole(Circle self, Dipole other) {
    return antiScalar_antiWedge_circle(circle_wedge_dipole(self, other), dipole_antiDual(other));
}

Circle circle_antiRejectOrthogonallyFrom_flatPoint(Circle self, FlatPoint other) {
    return antiScalar_antiWedge_circle(circle_wedge_flatPoint(self, other), flatPoint_antiDual(other));
}

MultiVector circle_antiRejectOrthogonallyFrom_flector(Circle self, Flector other) {
    return antiScalar_antiWedge_multiVector(circle_wedge_flector(self, other), flector_antiDual(other));
}

MultiVector circle_antiRejectOrthogonallyFrom_multiVector(Circle self, MultiVector other) {
    return multiVector_antiWedge_multiVector(circle_wedge_multiVector(self, other), multiVector_antiDual(other));
}

Circle circle_antiRejectOrthogonallyFrom_roundPoint(Circle self, RoundPoint other) {
    return sphere_antiWedge_sphere(circle_wedge_roundPoint(self, other), roundPoint_antiDual(other));
}

Dipole dipole_antiRejectOrthogonallyFrom_circle(Dipole self, Circle other) {
    return antiScalar_antiWedge_dipole(dipole_wedge_circle(self, other), circle_antiDual(other));
}

Dipole dipole_antiRejectOrthogonallyFrom_dipole(Dipole self, Dipole other) {
    return sphere_antiWedge_circle(dipole_wedge_dipole(self, other), dipole_antiDual(other));
}

Dipole dipole_antiRejectOrthogonallyFrom_flatPoint(Dipole self, FlatPoint other) {
    return plane_antiWedge_circle(dipole_wedge_flatPoint(self, other), flatPoint_antiDual(other));
}

MultiVector dipole_antiRejectOrthogonallyFrom_flector(Dipole self, Flector other) {
    return plane_antiWedge_multiVector(dipole_wedge_flector(self, other), flector_antiDual(other));
}

Dipole dipole_antiRejectOrthogonallyFrom_line(Dipole self, Line other) {
    return antiScalar_antiWedge_dipole(dipole_wedge_line(self, other), line_antiDual(other));
}

MultiVector dipole_antiRejectOrthogonallyFrom_motor(Dipole self, Motor other) {
    return antiScalar_antiWedge_multiVector(dipole_wedge_motor(self, other), motor_antiDual(other));
}

MultiVector dipole_antiRejectOrthogonallyFrom_multiVector(Dipole self, MultiVector other) {
    return multiVector_antiWedge_multiVector(dipole_wedge_multiVector(self, other), multiVector_antiDual(other));
}

Dipole dipole_antiRejectOrthogonallyFrom_roundPoint(Dipole self, RoundPoint other) {
    return circle_antiWedge_sphere(dipole_wedge_roundPoint(self, other), roundPoint_antiDual(other));
}

Dipole flatPoint_antiRejectOrthogonallyFrom_circle(FlatPoint self, Circle other) {
    return antiScalar_antiWedge_dipole(flatPoint_wedge_circle(self, other), circle_antiDual(other));
}

Dipole flatPoint_antiRejectOrthogonallyFrom_dipole(FlatPoint self, Dipole other) {
    return plane_antiWedge_circle(flatPoint_wedge_dipole(self, other), dipole_antiDual(other));
}

MultiVector flatPoint_antiRejectOrthogonallyFrom_multiVector(FlatPoint self, MultiVector other) {
    return multiVector_antiWedge_multiVector(flatPoint_wedge_multiVector(self, other), multiVector_antiDual(other));
}

Dipole flatPoint_antiRejectOrthogonallyFrom_roundPoint(FlatPoint self, RoundPoint other) {
    return line_antiWedge_sphere(flatPoint_wedge_roundPoint(self, other), roundPoint_antiDual(other));
}

Dipole flector_antiRejectOrthogonallyFrom_circle(Flector self, Circle other) {
    return antiScalar_antiWedge_dipole(flector_wedge_circle(self, other), circle_antiDual(other));
}

Dipole flector_antiRejectOrthogonallyFrom_dipole(Flector self, Dipole other) {
    return plane_antiWedge_circle(flector_wedge_dipole(self, other), dipole_antiDual(other));
}

MultiVector flector_antiRejectOrthogonallyFrom_multiVector(Flector self, MultiVector other) {
    return multiVector_antiWedge_multiVector(flector_wedge_multiVector(self, other), multiVector_antiDual(other));
}

MultiVector flector_antiRejectOrthogonallyFrom_roundPoint(Flector self, RoundPoint other) {
    return motor_antiWedge_sphere(flector_wedge_roundPoint(self, other), roundPoint_antiDual(other));
}

Circle line_antiRejectOrthogonallyFrom_dipole(Line self, Dipole other) {
    return antiScalar_antiWedge_circle(line_wedge_dipole(self, other), dipole_antiDual(other));
}

MultiVector line_antiRejectOrthogonallyFrom_multiVector(Line self, MultiVector other) {
    return multiVector_antiWedge_multiVector(line_wedge_multiVector(self, other), multiVector_antiDual(other));
}

Circle line_antiRejectOrthogonallyFrom_roundPoint(Line self, RoundPoint other) {
    return plane_antiWedge_sphere(line_wedge_roundPoint(self, other), roundPoint_antiDual(other));
}

Circle motor_antiRejectOrthogonallyFrom_dipole(Motor self, Dipole other) {
    return antiScalar_antiWedge_circle(motor_wedge_dipole(self, other), dipole_antiDual(other));
}

MultiVector motor_antiRejectOrthogonallyFrom_multiVector(Motor self, MultiVector other) {
    return multiVector_antiWedge_multiVector(motor_wedge_multiVector(self, other), multiVector_antiDual(other));
}

Circle motor_antiRejectOrthogonallyFrom_roundPoint(Motor self, RoundPoint other) {
    return plane_antiWedge_sphere(motor_wedge_roundPoint(self, other), roundPoint_antiDual(other));
}

MultiVector multiVector_antiRejectOrthogonallyFrom_circle(MultiVector self, Circle other) {
    return multiVector_antiWedge_dipole(multiVector_wedge_circle(self, other), circle_antiDual(other));
}

MultiVector multiVector_antiRejectOrthogonallyFrom_dipole(MultiVector self, Dipole other) {
    return multiVector_antiWedge_circle(multiVector_wedge_dipole(self, other), dipole_antiDual(other));
}

MultiVector multiVector_antiRejectOrthogonallyFrom_flatPoint(MultiVector self, FlatPoint other) {
    return multiVector_antiWedge_circle(multiVector_wedge_flatPoint(self, other), flatPoint_antiDual(other));
}

MultiVector multiVector_antiRejectOrthogonallyFrom_flector(MultiVector self, Flector other) {
    return multiVector_antiWedge_multiVector(multiVector_wedge_flector(self, other), flector_antiDual(other));
}

MultiVector multiVector_antiRejectOrthogonallyFrom_line(MultiVector self, Line other) {
    return multiVector_antiWedge_dipole(multiVector_wedge_line(self, other), line_antiDual(other));
}

MultiVector multiVector_antiRejectOrthogonallyFrom_motor(MultiVector self, Motor other) {
    return multiVector_antiWedge_multiVector(multiVector_wedge_motor(self, other), motor_antiDual(other));
}

MultiVector multiVector_antiRejectOrthogonallyFrom_multiVector(MultiVector self, MultiVector other) {
    return multiVector_antiWedge_multiVector(multiVector_wedge_multiVector(self, other), multiVector_antiDual(other));
}

MultiVector multiVector_antiRejectOrthogonallyFrom_plane(MultiVector self, Plane other) {
    return multiVector_antiWedge_roundPoint(multiVector_wedge_plane(self, other), plane_antiDual(other));
}

MultiVector multiVector_antiRejectOrthogonallyFrom_roundPoint(MultiVector self, RoundPoint other) {
    return multiVector_antiWedge_sphere(multiVector_wedge_roundPoint(self, other), roundPoint_antiDual(other));
}

MultiVector multiVector_antiRejectOrthogonallyFrom_sphere(MultiVector self, Sphere other) {
    return multiVector_antiWedge_roundPoint(multiVector_wedge_sphere(self, other), sphere_antiDual(other));
}

MultiVector plane_antiRejectOrthogonallyFrom_multiVector(Plane self, MultiVector other) {
    return multiVector_antiWedge_multiVector(plane_wedge_multiVector(self, other), multiVector_antiDual(other));
}

Sphere plane_antiRejectOrthogonallyFrom_roundPoint(Plane self, RoundPoint other) {
    return antiScalar_antiWedge_sphere(plane_wedge_roundPoint(self, other), roundPoint_antiDual(other));
}

RoundPoint roundPoint_antiRejectOrthogonallyFrom_circle(RoundPoint self, Circle other) {
    return sphere_antiWedge_dipole(roundPoint_wedge_circle(self, other), circle_antiDual(other));
}

RoundPoint roundPoint_antiRejectOrthogonallyFrom_dipole(RoundPoint self, Dipole other) {
    return circle_antiWedge_circle(roundPoint_wedge_dipole(self, other), dipole_antiDual(other));
}

RoundPoint roundPoint_antiRejectOrthogonallyFrom_flatPoint(RoundPoint self, FlatPoint other) {
    return line_antiWedge_circle(roundPoint_wedge_flatPoint(self, other), flatPoint_antiDual(other));
}

MultiVector roundPoint_antiRejectOrthogonallyFrom_flector(RoundPoint self, Flector other) {
    return motor_antiWedge_multiVector(roundPoint_wedge_flector(self, other), flector_antiDual(other));
}

RoundPoint roundPoint_antiRejectOrthogonallyFrom_line(RoundPoint self, Line other) {
    return plane_antiWedge_dipole(roundPoint_wedge_line(self, other), line_antiDual(other));
}

MultiVector roundPoint_antiRejectOrthogonallyFrom_motor(RoundPoint self, Motor other) {
    return plane_antiWedge_multiVector(roundPoint_wedge_motor(self, other), motor_antiDual(other));
}

MultiVector roundPoint_antiRejectOrthogonallyFrom_multiVector(RoundPoint self, MultiVector other) {
    return multiVector_antiWedge_multiVector(roundPoint_wedge_multiVector(self, other), multiVector_antiDual(other));
}

RoundPoint roundPoint_antiRejectOrthogonallyFrom_plane(RoundPoint self, Plane other) {
    return antiScalar_antiWedge_roundPoint(roundPoint_wedge_plane(self, other), plane_antiDual(other));
}

RoundPoint roundPoint_antiRejectOrthogonallyFrom_roundPoint(RoundPoint self, RoundPoint other) {
    return dipole_antiWedge_sphere(roundPoint_wedge_roundPoint(self, other), roundPoint_antiDual(other));
}

RoundPoint roundPoint_antiRejectOrthogonallyFrom_sphere(RoundPoint self, Sphere other) {
    return antiScalar_antiWedge_roundPoint(roundPoint_wedge_sphere(self, other), sphere_antiDual(other));
}

MultiVector sphere_antiRejectOrthogonallyFrom_multiVector(Sphere self, MultiVector other) {
    return multiVector_antiWedge_multiVector(sphere_wedge_multiVector(self, other), multiVector_antiDual(other));
}

Sphere sphere_antiRejectOrthogonallyFrom_roundPoint(Sphere self, RoundPoint other) {
    return antiScalar_antiWedge_sphere(sphere_wedge_roundPoint(self, other), roundPoint_antiDual(other));
}

Circle circle_antiRejectViaHorizonFrom_dipole(Circle self, Dipole other) {
    return antiScalar_antiWedge_circle(circle_wedge_dipole(self, other), dipole_dual(other));
}

Circle circle_antiRejectViaHorizonFrom_flatPoint(Circle self, FlatPoint other) {
    return antiScalar_antiWedge_circle(circle_wedge_flatPoint(self, other), flatPoint_dual(other));
}

MultiVector circle_antiRejectViaHorizonFrom_flector(Circle self, Flector other) {
    return antiScalar_antiWedge_multiVector(circle_wedge_flector(self, other), flector_dual(other));
}

MultiVector circle_antiRejectViaHorizonFrom_multiVector(Circle self, MultiVector other) {
    return multiVector_antiWedge_multiVector(circle_wedge_multiVector(self, other), multiVector_dual(other));
}

Circle circle_antiRejectViaHorizonFrom_roundPoint(Circle self, RoundPoint other) {
    return sphere_antiWedge_sphere(circle_wedge_roundPoint(self, other), roundPoint_dual(other));
}

Dipole dipole_antiRejectViaHorizonFrom_circle(Dipole self, Circle other) {
    return antiScalar_antiWedge_dipole(dipole_wedge_circle(self, other), circle_dual(other));
}

Dipole dipole_antiRejectViaHorizonFrom_dipole(Dipole self, Dipole other) {
    return sphere_antiWedge_circle(dipole_wedge_dipole(self, other), dipole_dual(other));
}

Dipole dipole_antiRejectViaHorizonFrom_flatPoint(Dipole self, FlatPoint other) {
    return plane_antiWedge_circle(dipole_wedge_flatPoint(self, other), flatPoint_dual(other));
}

MultiVector dipole_antiRejectViaHorizonFrom_flector(Dipole self, Flector other) {
    return plane_antiWedge_multiVector(dipole_wedge_flector(self, other), flector_dual(other));
}

Dipole dipole_antiRejectViaHorizonFrom_line(Dipole self, Line other) {
    return antiScalar_antiWedge_dipole(dipole_wedge_line(self, other), line_dual(other));
}

MultiVector dipole_antiRejectViaHorizonFrom_motor(Dipole self, Motor other) {
    return antiScalar_antiWedge_multiVector(dipole_wedge_motor(self, other), motor_dual(other));
}

MultiVector dipole_antiRejectViaHorizonFrom_multiVector(Dipole self, MultiVector other) {
    return multiVector_antiWedge_multiVector(dipole_wedge_multiVector(self, other), multiVector_dual(other));
}

Dipole dipole_antiRejectViaHorizonFrom_roundPoint(Dipole self, RoundPoint other) {
    return circle_antiWedge_sphere(dipole_wedge_roundPoint(self, other), roundPoint_dual(other));
}

Dipole flatPoint_antiRejectViaHorizonFrom_circle(FlatPoint self, Circle other) {
    return antiScalar_antiWedge_dipole(flatPoint_wedge_circle(self, other), circle_dual(other));
}

Dipole flatPoint_antiRejectViaHorizonFrom_dipole(FlatPoint self, Dipole other) {
    return plane_antiWedge_circle(flatPoint_wedge_dipole(self, other), dipole_dual(other));
}

MultiVector flatPoint_antiRejectViaHorizonFrom_multiVector(FlatPoint self, MultiVector other) {
    return multiVector_antiWedge_multiVector(flatPoint_wedge_multiVector(self, other), multiVector_dual(other));
}

Dipole flatPoint_antiRejectViaHorizonFrom_roundPoint(FlatPoint self, RoundPoint other) {
    return line_antiWedge_sphere(flatPoint_wedge_roundPoint(self, other), roundPoint_dual(other));
}

Dipole flector_antiRejectViaHorizonFrom_circle(Flector self, Circle other) {
    return antiScalar_antiWedge_dipole(flector_wedge_circle(self, other), circle_dual(other));
}

Dipole flector_antiRejectViaHorizonFrom_dipole(Flector self, Dipole other) {
    return plane_antiWedge_circle(flector_wedge_dipole(self, other), dipole_dual(other));
}

MultiVector flector_antiRejectViaHorizonFrom_multiVector(Flector self, MultiVector other) {
    return multiVector_antiWedge_multiVector(flector_wedge_multiVector(self, other), multiVector_dual(other));
}

MultiVector flector_antiRejectViaHorizonFrom_roundPoint(Flector self, RoundPoint other) {
    return motor_antiWedge_sphere(flector_wedge_roundPoint(self, other), roundPoint_dual(other));
}

Circle line_antiRejectViaHorizonFrom_dipole(Line self, Dipole other) {
    return antiScalar_antiWedge_circle(line_wedge_dipole(self, other), dipole_dual(other));
}

MultiVector line_antiRejectViaHorizonFrom_multiVector(Line self, MultiVector other) {
    return multiVector_antiWedge_multiVector(line_wedge_multiVector(self, other), multiVector_dual(other));
}

Circle line_antiRejectViaHorizonFrom_roundPoint(Line self, RoundPoint other) {
    return plane_antiWedge_sphere(line_wedge_roundPoint(self, other), roundPoint_dual(other));
}

Circle motor_antiRejectViaHorizonFrom_dipole(Motor self, Dipole other) {
    return antiScalar_antiWedge_circle(motor_wedge_dipole(self, other), dipole_dual(other));
}

MultiVector motor_antiRejectViaHorizonFrom_multiVector(Motor self, MultiVector other) {
    return multiVector_antiWedge_multiVector(motor_wedge_multiVector(self, other), multiVector_dual(other));
}

Circle motor_antiRejectViaHorizonFrom_roundPoint(Motor self, RoundPoint other) {
    return plane_antiWedge_sphere(motor_wedge_roundPoint(self, other), roundPoint_dual(other));
}

MultiVector multiVector_antiRejectViaHorizonFrom_circle(MultiVector self, Circle other) {
    return multiVector_antiWedge_dipole(multiVector_wedge_circle(self, other), circle_dual(other));
}

MultiVector multiVector_antiRejectViaHorizonFrom_dipole(MultiVector self, Dipole other) {
    return multiVector_antiWedge_circle(multiVector_wedge_dipole(self, other), dipole_dual(other));
}

MultiVector multiVector_antiRejectViaHorizonFrom_flatPoint(MultiVector self, FlatPoint other) {
    return multiVector_antiWedge_circle(multiVector_wedge_flatPoint(self, other), flatPoint_dual(other));
}

MultiVector multiVector_antiRejectViaHorizonFrom_flector(MultiVector self, Flector other) {
    return multiVector_antiWedge_multiVector(multiVector_wedge_flector(self, other), flector_dual(other));
}

MultiVector multiVector_antiRejectViaHorizonFrom_line(MultiVector self, Line other) {
    return multiVector_antiWedge_dipole(multiVector_wedge_line(self, other), line_dual(other));
}

MultiVector multiVector_antiRejectViaHorizonFrom_motor(MultiVector self, Motor other) {
    return multiVector_antiWedge_multiVector(multiVector_wedge_motor(self, other), motor_dual(other));
}

MultiVector multiVector_antiRejectViaHorizonFrom_multiVector(MultiVector self, MultiVector other) {
    return multiVector_antiWedge_multiVector(multiVector_wedge_multiVector(self, other), multiVector_dual(other));
}

MultiVector multiVector_antiRejectViaHorizonFrom_plane(MultiVector self, Plane other) {
    return multiVector_antiWedge_roundPoint(multiVector_wedge_plane(self, other), plane_dual(other));
}

MultiVector multiVector_antiRejectViaHorizonFrom_roundPoint(MultiVector self, RoundPoint other) {
    return multiVector_antiWedge_sphere(multiVector_wedge_roundPoint(self, other), roundPoint_dual(other));
}

MultiVector multiVector_antiRejectViaHorizonFrom_sphere(MultiVector self, Sphere other) {
    return multiVector_antiWedge_roundPoint(multiVector_wedge_sphere(self, other), sphere_dual(other));
}

MultiVector plane_antiRejectViaHorizonFrom_multiVector(Plane self, MultiVector other) {
    return multiVector_antiWedge_multiVector(plane_wedge_multiVector(self, other), multiVector_dual(other));
}

Sphere plane_antiRejectViaHorizonFrom_roundPoint(Plane self, RoundPoint other) {
    return antiScalar_antiWedge_sphere(plane_wedge_roundPoint(self, other), roundPoint_dual(other));
}

RoundPoint roundPoint_antiRejectViaHorizonFrom_circle(RoundPoint self, Circle other) {
    return sphere_antiWedge_dipole(roundPoint_wedge_circle(self, other), circle_dual(other));
}

RoundPoint roundPoint_antiRejectViaHorizonFrom_dipole(RoundPoint self, Dipole other) {
    return circle_antiWedge_circle(roundPoint_wedge_dipole(self, other), dipole_dual(other));
}

RoundPoint roundPoint_antiRejectViaHorizonFrom_flatPoint(RoundPoint self, FlatPoint other) {
    return line_antiWedge_circle(roundPoint_wedge_flatPoint(self, other), flatPoint_dual(other));
}

MultiVector roundPoint_antiRejectViaHorizonFrom_flector(RoundPoint self, Flector other) {
    return motor_antiWedge_multiVector(roundPoint_wedge_flector(self, other), flector_dual(other));
}

RoundPoint roundPoint_antiRejectViaHorizonFrom_line(RoundPoint self, Line other) {
    return plane_antiWedge_dipole(roundPoint_wedge_line(self, other), line_dual(other));
}

MultiVector roundPoint_antiRejectViaHorizonFrom_motor(RoundPoint self, Motor other) {
    return plane_antiWedge_multiVector(roundPoint_wedge_motor(self, other), motor_dual(other));
}

MultiVector roundPoint_antiRejectViaHorizonFrom_multiVector(RoundPoint self, MultiVector other) {
    return multiVector_antiWedge_multiVector(roundPoint_wedge_multiVector(self, other), multiVector_dual(other));
}

RoundPoint roundPoint_antiRejectViaHorizonFrom_plane(RoundPoint self, Plane other) {
    return antiScalar_antiWedge_roundPoint(roundPoint_wedge_plane(self, other), plane_dual(other));
}

RoundPoint roundPoint_antiRejectViaHorizonFrom_roundPoint(RoundPoint self, RoundPoint other) {
    return dipole_antiWedge_sphere(roundPoint_wedge_roundPoint(self, other), roundPoint_dual(other));
}

RoundPoint roundPoint_antiRejectViaHorizonFrom_sphere(RoundPoint self, Sphere other) {
    return antiScalar_antiWedge_roundPoint(roundPoint_wedge_sphere(self, other), sphere_dual(other));
}

MultiVector sphere_antiRejectViaHorizonFrom_multiVector(Sphere self, MultiVector other) {
    return multiVector_antiWedge_multiVector(sphere_wedge_multiVector(self, other), multiVector_dual(other));
}

Sphere sphere_antiRejectViaHorizonFrom_roundPoint(Sphere self, RoundPoint other) {
    return antiScalar_antiWedge_sphere(sphere_wedge_roundPoint(self, other), roundPoint_dual(other));
}

Circle circle_rejectOrthogonallyFrom_circle(Circle self, Circle other) {
    return roundPoint_wedge_dipole(circle_antiWedge_circle(self, other), circle_antiDual(other));
}

Circle circle_rejectOrthogonallyFrom_dipole(Circle self, Dipole other) {
    return scalar_wedge_circle(circle_antiWedge_dipole(self, other), dipole_antiDual(other));
}

Circle circle_rejectOrthogonallyFrom_flatPoint(Circle self, FlatPoint other) {
    return scalar_wedge_circle(circle_antiWedge_flatPoint(self, other), flatPoint_antiDual(other));
}

MultiVector circle_rejectOrthogonallyFrom_flector(Circle self, Flector other) {
    return multiVector_wedge_multiVector(circle_antiWedge_flector(self, other), flector_antiDual(other));
}

Circle circle_rejectOrthogonallyFrom_line(Circle self, Line other) {
    return roundPoint_wedge_dipole(circle_antiWedge_line(self, other), line_antiDual(other));
}

MultiVector circle_rejectOrthogonallyFrom_motor(Circle self, Motor other) {
    return multiVector_wedge_multiVector(circle_antiWedge_motor(self, other), motor_antiDual(other));
}

MultiVector circle_rejectOrthogonallyFrom_multiVector(Circle self, MultiVector other) {
    return multiVector_wedge_multiVector(circle_antiWedge_multiVector(self, other), multiVector_antiDual(other));
}

Circle circle_rejectOrthogonallyFrom_plane(Circle self, Plane other) {
    return dipole_wedge_roundPoint(circle_antiWedge_plane(self, other), plane_antiDual(other));
}

Circle circle_rejectOrthogonallyFrom_sphere(Circle self, Sphere other) {
    return dipole_wedge_roundPoint(circle_antiWedge_sphere(self, other), sphere_antiDual(other));
}

Dipole dipole_rejectOrthogonallyFrom_circle(Dipole self, Circle other) {
    return scalar_wedge_dipole(dipole_antiWedge_circle(self, other), circle_antiDual(other));
}

MultiVector dipole_rejectOrthogonallyFrom_flector(Dipole self, Flector other) {
    return roundPoint_wedge_multiVector(dipole_antiWedge_flector(self, other), flector_antiDual(other));
}

Dipole dipole_rejectOrthogonallyFrom_line(Dipole self, Line other) {
    return scalar_wedge_dipole(dipole_antiWedge_line(self, other), line_antiDual(other));
}

MultiVector dipole_rejectOrthogonallyFrom_motor(Dipole self, Motor other) {
    return multiVector_wedge_multiVector(dipole_antiWedge_motor(self, other), motor_antiDual(other));
}

MultiVector dipole_rejectOrthogonallyFrom_multiVector(Dipole self, MultiVector other) {
    return multiVector_wedge_multiVector(dipole_antiWedge_multiVector(self, other), multiVector_antiDual(other));
}

Dipole dipole_rejectOrthogonallyFrom_plane(Dipole self, Plane other) {
    return roundPoint_wedge_roundPoint(dipole_antiWedge_plane(self, other), plane_antiDual(other));
}

Dipole dipole_rejectOrthogonallyFrom_sphere(Dipole self, Sphere other) {
    return roundPoint_wedge_roundPoint(dipole_antiWedge_sphere(self, other), sphere_antiDual(other));
}

Dipole flatPoint_rejectOrthogonallyFrom_circle(FlatPoint self, Circle other) {
    return scalar_wedge_dipole(flatPoint_antiWedge_circle(self, other), circle_antiDual(other));
}

MultiVector flatPoint_rejectOrthogonallyFrom_flector(FlatPoint self, Flector other) {
    return roundPoint_wedge_multiVector(flatPoint_antiWedge_flector(self, other), flector_antiDual(other));
}

MultiVector flatPoint_rejectOrthogonallyFrom_motor(FlatPoint self, Motor other) {
    return flatPoint_wedge_multiVector(flatPoint_antiWedge_motor(self, other), motor_antiDual(other));
}

MultiVector flatPoint_rejectOrthogonallyFrom_multiVector(FlatPoint self, MultiVector other) {
    return multiVector_wedge_multiVector(flatPoint_antiWedge_multiVector(self, other), multiVector_antiDual(other));
}

Dipole flatPoint_rejectOrthogonallyFrom_plane(FlatPoint self, Plane other) {
    return roundPoint_wedge_roundPoint(flatPoint_antiWedge_plane(self, other), plane_antiDual(other));
}

Dipole flatPoint_rejectOrthogonallyFrom_sphere(FlatPoint self, Sphere other) {
    return roundPoint_wedge_roundPoint(flatPoint_antiWedge_sphere(self, other), sphere_antiDual(other));
}

MultiVector flector_rejectOrthogonallyFrom_circle(Flector self, Circle other) {
    return multiVector_wedge_dipole(flector_antiWedge_circle(self, other), circle_antiDual(other));
}

Sphere flector_rejectOrthogonallyFrom_dipole(Flector self, Dipole other) {
    return roundPoint_wedge_circle(flector_antiWedge_dipole(self, other), dipole_antiDual(other));
}

Sphere flector_rejectOrthogonallyFrom_flatPoint(Flector self, FlatPoint other) {
    return roundPoint_wedge_circle(flector_antiWedge_flatPoint(self, other), flatPoint_antiDual(other));
}

MultiVector flector_rejectOrthogonallyFrom_flector(Flector self, Flector other) {
    return multiVector_wedge_multiVector(flector_antiWedge_flector(self, other), flector_antiDual(other));
}

Plane flector_rejectOrthogonallyFrom_line(Flector self, Line other) {
    return flatPoint_wedge_dipole(flector_antiWedge_line(self, other), line_antiDual(other));
}

MultiVector flector_rejectOrthogonallyFrom_motor(Flector self, Motor other) {
    return flector_wedge_multiVector(flector_antiWedge_motor(self, other), motor_antiDual(other));
}

MultiVector flector_rejectOrthogonallyFrom_multiVector(Flector self, MultiVector other) {
    return multiVector_wedge_multiVector(flector_antiWedge_multiVector(self, other), multiVector_antiDual(other));
}

MultiVector flector_rejectOrthogonallyFrom_plane(Flector self, Plane other) {
    return multiVector_wedge_roundPoint(flector_antiWedge_plane(self, other), plane_antiDual(other));
}

Sphere flector_rejectOrthogonallyFrom_roundPoint(Flector self, RoundPoint other) {
    return scalar_wedge_sphere(flector_antiWedge_roundPoint(self, other), roundPoint_antiDual(other));
}

MultiVector flector_rejectOrthogonallyFrom_sphere(Flector self, Sphere other) {
    return multiVector_wedge_roundPoint(flector_antiWedge_sphere(self, other), sphere_antiDual(other));
}

Circle line_rejectOrthogonallyFrom_circle(Line self, Circle other) {
    return roundPoint_wedge_dipole(line_antiWedge_circle(self, other), circle_antiDual(other));
}

Circle line_rejectOrthogonallyFrom_dipole(Line self, Dipole other) {
    return scalar_wedge_circle(line_antiWedge_dipole(self, other), dipole_antiDual(other));
}

MultiVector line_rejectOrthogonallyFrom_flector(Line self, Flector other) {
    return flatPoint_wedge_multiVector(line_antiWedge_flector(self, other), flector_antiDual(other));
}

Circle line_rejectOrthogonallyFrom_line(Line self, Line other) {
    return roundPoint_wedge_dipole(line_antiWedge_line(self, other), line_antiDual(other));
}

MultiVector line_rejectOrthogonallyFrom_motor(Line self, Motor other) {
    return multiVector_wedge_multiVector(line_antiWedge_motor(self, other), motor_antiDual(other));
}

MultiVector line_rejectOrthogonallyFrom_multiVector(Line self, MultiVector other) {
    return multiVector_wedge_multiVector(line_antiWedge_multiVector(self, other), multiVector_antiDual(other));
}

Line line_rejectOrthogonallyFrom_plane(Line self, Plane other) {
    return flatPoint_wedge_roundPoint(line_antiWedge_plane(self, other), plane_antiDual(other));
}

Circle line_rejectOrthogonallyFrom_sphere(Line self, Sphere other) {
    return dipole_wedge_roundPoint(line_antiWedge_sphere(self, other), sphere_antiDual(other));
}

MultiVector motor_rejectOrthogonallyFrom_circle(Motor self, Circle other) {
    return multiVector_wedge_dipole(motor_antiWedge_circle(self, other), circle_antiDual(other));
}

MultiVector motor_rejectOrthogonallyFrom_dipole(Motor self, Dipole other) {
    return multiVector_wedge_circle(motor_antiWedge_dipole(self, other), dipole_antiDual(other));
}

AntiScalar motor_rejectOrthogonallyFrom_flatPoint(Motor self, FlatPoint other) {
    return flatPoint_wedge_circle(motor_antiWedge_flatPoint(self, other), flatPoint_antiDual(other));
}

MultiVector motor_rejectOrthogonallyFrom_flector(Motor self, Flector other) {
    return flector_wedge_multiVector(motor_antiWedge_flector(self, other), flector_antiDual(other));
}

MultiVector motor_rejectOrthogonallyFrom_line(Motor self, Line other) {
    return multiVector_wedge_dipole(motor_antiWedge_line(self, other), line_antiDual(other));
}

MultiVector motor_rejectOrthogonallyFrom_motor(Motor self, Motor other) {
    return multiVector_wedge_multiVector(motor_antiWedge_motor(self, other), motor_antiDual(other));
}

MultiVector motor_rejectOrthogonallyFrom_multiVector(Motor self, MultiVector other) {
    return multiVector_wedge_multiVector(motor_antiWedge_multiVector(self, other), multiVector_antiDual(other));
}

Motor motor_rejectOrthogonallyFrom_plane(Motor self, Plane other) {
    return flector_wedge_roundPoint(motor_antiWedge_plane(self, other), plane_antiDual(other));
}

AntiScalar motor_rejectOrthogonallyFrom_roundPoint(Motor self, RoundPoint other) {
    return roundPoint_wedge_sphere(motor_antiWedge_roundPoint(self, other), roundPoint_antiDual(other));
}

MultiVector motor_rejectOrthogonallyFrom_sphere(Motor self, Sphere other) {
    return multiVector_wedge_roundPoint(motor_antiWedge_sphere(self, other), sphere_antiDual(other));
}

MultiVector multiVector_rejectOrthogonallyFrom_circle(MultiVector self, Circle other) {
    return multiVector_wedge_dipole(multiVector_antiWedge_circle(self, other), circle_antiDual(other));
}

MultiVector multiVector_rejectOrthogonallyFrom_dipole(MultiVector self, Dipole other) {
    return multiVector_wedge_circle(multiVector_antiWedge_dipole(self, other), dipole_antiDual(other));
}

MultiVector multiVector_rejectOrthogonallyFrom_flatPoint(MultiVector self, FlatPoint other) {
    return multiVector_wedge_circle(multiVector_antiWedge_flatPoint(self, other), flatPoint_antiDual(other));
}

MultiVector multiVector_rejectOrthogonallyFrom_flector(MultiVector self, Flector other) {
    return multiVector_wedge_multiVector(multiVector_antiWedge_flector(self, other), flector_antiDual(other));
}

MultiVector multiVector_rejectOrthogonallyFrom_line(MultiVector self, Line other) {
    return multiVector_wedge_dipole(multiVector_antiWedge_line(self, other), line_antiDual(other));
}

MultiVector multiVector_rejectOrthogonallyFrom_motor(MultiVector self, Motor other) {
    return multiVector_wedge_multiVector(multiVector_antiWedge_motor(self, other), motor_antiDual(other));
}

MultiVector multiVector_rejectOrthogonallyFrom_multiVector(MultiVector self, MultiVector other) {
    return multiVector_wedge_multiVector(multiVector_antiWedge_multiVector(self, other), multiVector_antiDual(other));
}

MultiVector multiVector_rejectOrthogonallyFrom_plane(MultiVector self, Plane other) {
    return multiVector_wedge_roundPoint(multiVector_antiWedge_plane(self, other), plane_antiDual(other));
}

MultiVector multiVector_rejectOrthogonallyFrom_roundPoint(MultiVector self, RoundPoint other) {
    return multiVector_wedge_sphere(multiVector_antiWedge_roundPoint(self, other), roundPoint_antiDual(other));
}

MultiVector multiVector_rejectOrthogonallyFrom_sphere(MultiVector self, Sphere other) {
    return multiVector_wedge_roundPoint(multiVector_antiWedge_sphere(self, other), sphere_antiDual(other));
}

Sphere plane_rejectOrthogonallyFrom_circle(Plane self, Circle other) {
    return dipole_wedge_dipole(plane_antiWedge_circle(self, other), circle_antiDual(other));
}

Sphere plane_rejectOrthogonallyFrom_dipole(Plane self, Dipole other) {
    return roundPoint_wedge_circle(plane_antiWedge_dipole(self, other), dipole_antiDual(other));
}

Sphere plane_rejectOrthogonallyFrom_flatPoint(Plane self, FlatPoint other) {
    return roundPoint_wedge_circle(plane_antiWedge_flatPoint(self, other), flatPoint_antiDual(other));
}

MultiVector plane_rejectOrthogonallyFrom_flector(Plane self, Flector other) {
    return multiVector_wedge_multiVector(plane_antiWedge_flector(self, other), flector_antiDual(other));
}

Plane plane_rejectOrthogonallyFrom_line(Plane self, Line other) {
    return flatPoint_wedge_dipole(plane_antiWedge_line(self, other), line_antiDual(other));
}

MultiVector plane_rejectOrthogonallyFrom_motor(Plane self, Motor other) {
    return flector_wedge_multiVector(plane_antiWedge_motor(self, other), motor_antiDual(other));
}

MultiVector plane_rejectOrthogonallyFrom_multiVector(Plane self, MultiVector other) {
    return multiVector_wedge_multiVector(plane_antiWedge_multiVector(self, other), multiVector_antiDual(other));
}

Plane plane_rejectOrthogonallyFrom_plane(Plane self, Plane other) {
    return line_wedge_roundPoint(plane_antiWedge_plane(self, other), plane_antiDual(other));
}

Sphere plane_rejectOrthogonallyFrom_roundPoint(Plane self, RoundPoint other) {
    return scalar_wedge_sphere(plane_antiWedge_roundPoint(self, other), roundPoint_antiDual(other));
}

Sphere plane_rejectOrthogonallyFrom_sphere(Plane self, Sphere other) {
    return circle_wedge_roundPoint(plane_antiWedge_sphere(self, other), sphere_antiDual(other));
}

MultiVector roundPoint_rejectOrthogonallyFrom_flector(RoundPoint self, Flector other) {
    return scalar_wedge_multiVector(roundPoint_antiWedge_flector(self, other), flector_antiDual(other));
}

MultiVector roundPoint_rejectOrthogonallyFrom_motor(RoundPoint self, Motor other) {
    return roundPoint_wedge_multiVector(roundPoint_antiWedge_motor(self, other), motor_antiDual(other));
}

MultiVector roundPoint_rejectOrthogonallyFrom_multiVector(RoundPoint self, MultiVector other) {
    return multiVector_wedge_multiVector(roundPoint_antiWedge_multiVector(self, other), multiVector_antiDual(other));
}

RoundPoint roundPoint_rejectOrthogonallyFrom_plane(RoundPoint self, Plane other) {
    return scalar_wedge_roundPoint(roundPoint_antiWedge_plane(self, other), plane_antiDual(other));
}

RoundPoint roundPoint_rejectOrthogonallyFrom_sphere(RoundPoint self, Sphere other) {
    return scalar_wedge_roundPoint(roundPoint_antiWedge_sphere(self, other), sphere_antiDual(other));
}

Sphere sphere_rejectOrthogonallyFrom_circle(Sphere self, Circle other) {
    return dipole_wedge_dipole(sphere_antiWedge_circle(self, other), circle_antiDual(other));
}

Sphere sphere_rejectOrthogonallyFrom_dipole(Sphere self, Dipole other) {
    return roundPoint_wedge_circle(sphere_antiWedge_dipole(self, other), dipole_antiDual(other));
}

Sphere sphere_rejectOrthogonallyFrom_flatPoint(Sphere self, FlatPoint other) {
    return roundPoint_wedge_circle(sphere_antiWedge_flatPoint(self, other), flatPoint_antiDual(other));
}

MultiVector sphere_rejectOrthogonallyFrom_flector(Sphere self, Flector other) {
    return multiVector_wedge_multiVector(sphere_antiWedge_flector(self, other), flector_antiDual(other));
}

Sphere sphere_rejectOrthogonallyFrom_line(Sphere self, Line other) {
    return dipole_wedge_dipole(sphere_antiWedge_line(self, other), line_antiDual(other));
}

MultiVector sphere_rejectOrthogonallyFrom_motor(Sphere self, Motor other) {
    return multiVector_wedge_multiVector(sphere_antiWedge_motor(self, other), motor_antiDual(other));
}

MultiVector sphere_rejectOrthogonallyFrom_multiVector(Sphere self, MultiVector other) {
    return multiVector_wedge_multiVector(sphere_antiWedge_multiVector(self, other), multiVector_antiDual(other));
}

Sphere sphere_rejectOrthogonallyFrom_plane(Sphere self, Plane other) {
    return circle_wedge_roundPoint(sphere_antiWedge_plane(self, other), plane_antiDual(other));
}

Sphere sphere_rejectOrthogonallyFrom_roundPoint(Sphere self, RoundPoint other) {
    return scalar_wedge_sphere(sphere_antiWedge_roundPoint(self, other), roundPoint_antiDual(other));
}

Sphere sphere_rejectOrthogonallyFrom_sphere(Sphere self, Sphere other) {
    return circle_wedge_roundPoint(sphere_antiWedge_sphere(self, other), sphere_antiDual(other));
}

Circle circle_rejectViaOriginFrom_circle(Circle self, Circle other) {
    return roundPoint_wedge_dipole(circle_antiWedge_circle(self, other), circle_dual(other));
}

Circle circle_rejectViaOriginFrom_dipole(Circle self, Dipole other) {
    return scalar_wedge_circle(circle_antiWedge_dipole(self, other), dipole_dual(other));
}

Circle circle_rejectViaOriginFrom_flatPoint(Circle self, FlatPoint other) {
    return scalar_wedge_circle(circle_antiWedge_flatPoint(self, other), flatPoint_dual(other));
}

MultiVector circle_rejectViaOriginFrom_flector(Circle self, Flector other) {
    return multiVector_wedge_multiVector(circle_antiWedge_flector(self, other), flector_dual(other));
}

Circle circle_rejectViaOriginFrom_line(Circle self, Line other) {
    return roundPoint_wedge_dipole(circle_antiWedge_line(self, other), line_dual(other));
}

MultiVector circle_rejectViaOriginFrom_motor(Circle self, Motor other) {
    return multiVector_wedge_multiVector(circle_antiWedge_motor(self, other), motor_dual(other));
}

MultiVector circle_rejectViaOriginFrom_multiVector(Circle self, MultiVector other) {
    return multiVector_wedge_multiVector(circle_antiWedge_multiVector(self, other), multiVector_dual(other));
}

Circle circle_rejectViaOriginFrom_plane(Circle self, Plane other) {
    return dipole_wedge_roundPoint(circle_antiWedge_plane(self, other), plane_dual(other));
}

Circle circle_rejectViaOriginFrom_sphere(Circle self, Sphere other) {
    return dipole_wedge_roundPoint(circle_antiWedge_sphere(self, other), sphere_dual(other));
}

Dipole dipole_rejectViaOriginFrom_circle(Dipole self, Circle other) {
    return scalar_wedge_dipole(dipole_antiWedge_circle(self, other), circle_dual(other));
}

MultiVector dipole_rejectViaOriginFrom_flector(Dipole self, Flector other) {
    return roundPoint_wedge_multiVector(dipole_antiWedge_flector(self, other), flector_dual(other));
}

Dipole dipole_rejectViaOriginFrom_line(Dipole self, Line other) {
    return scalar_wedge_dipole(dipole_antiWedge_line(self, other), line_dual(other));
}

MultiVector dipole_rejectViaOriginFrom_motor(Dipole self, Motor other) {
    return multiVector_wedge_multiVector(dipole_antiWedge_motor(self, other), motor_dual(other));
}

MultiVector dipole_rejectViaOriginFrom_multiVector(Dipole self, MultiVector other) {
    return multiVector_wedge_multiVector(dipole_antiWedge_multiVector(self, other), multiVector_dual(other));
}

Dipole dipole_rejectViaOriginFrom_plane(Dipole self, Plane other) {
    return roundPoint_wedge_roundPoint(dipole_antiWedge_plane(self, other), plane_dual(other));
}

Dipole dipole_rejectViaOriginFrom_sphere(Dipole self, Sphere other) {
    return roundPoint_wedge_roundPoint(dipole_antiWedge_sphere(self, other), sphere_dual(other));
}

Dipole flatPoint_rejectViaOriginFrom_circle(FlatPoint self, Circle other) {
    return scalar_wedge_dipole(flatPoint_antiWedge_circle(self, other), circle_dual(other));
}

MultiVector flatPoint_rejectViaOriginFrom_flector(FlatPoint self, Flector other) {
    return roundPoint_wedge_multiVector(flatPoint_antiWedge_flector(self, other), flector_dual(other));
}

MultiVector flatPoint_rejectViaOriginFrom_motor(FlatPoint self, Motor other) {
    return flatPoint_wedge_multiVector(flatPoint_antiWedge_motor(self, other), motor_dual(other));
}

MultiVector flatPoint_rejectViaOriginFrom_multiVector(FlatPoint self, MultiVector other) {
    return multiVector_wedge_multiVector(flatPoint_antiWedge_multiVector(self, other), multiVector_dual(other));
}

Dipole flatPoint_rejectViaOriginFrom_plane(FlatPoint self, Plane other) {
    return roundPoint_wedge_roundPoint(flatPoint_antiWedge_plane(self, other), plane_dual(other));
}

Dipole flatPoint_rejectViaOriginFrom_sphere(FlatPoint self, Sphere other) {
    return roundPoint_wedge_roundPoint(flatPoint_antiWedge_sphere(self, other), sphere_dual(other));
}

MultiVector flector_rejectViaOriginFrom_circle(Flector self, Circle other) {
    return multiVector_wedge_dipole(flector_antiWedge_circle(self, other), circle_dual(other));
}

Sphere flector_rejectViaOriginFrom_dipole(Flector self, Dipole other) {
    return roundPoint_wedge_circle(flector_antiWedge_dipole(self, other), dipole_dual(other));
}

Sphere flector_rejectViaOriginFrom_flatPoint(Flector self, FlatPoint other) {
    return roundPoint_wedge_circle(flector_antiWedge_flatPoint(self, other), flatPoint_dual(other));
}

MultiVector flector_rejectViaOriginFrom_flector(Flector self, Flector other) {
    return multiVector_wedge_multiVector(flector_antiWedge_flector(self, other), flector_dual(other));
}

Plane flector_rejectViaOriginFrom_line(Flector self, Line other) {
    return flatPoint_wedge_dipole(flector_antiWedge_line(self, other), line_dual(other));
}

MultiVector flector_rejectViaOriginFrom_motor(Flector self, Motor other) {
    return flector_wedge_multiVector(flector_antiWedge_motor(self, other), motor_dual(other));
}

MultiVector flector_rejectViaOriginFrom_multiVector(Flector self, MultiVector other) {
    return multiVector_wedge_multiVector(flector_antiWedge_multiVector(self, other), multiVector_dual(other));
}

MultiVector flector_rejectViaOriginFrom_plane(Flector self, Plane other) {
    return multiVector_wedge_roundPoint(flector_antiWedge_plane(self, other), plane_dual(other));
}

Sphere flector_rejectViaOriginFrom_roundPoint(Flector self, RoundPoint other) {
    return scalar_wedge_sphere(flector_antiWedge_roundPoint(self, other), roundPoint_dual(other));
}

MultiVector flector_rejectViaOriginFrom_sphere(Flector self, Sphere other) {
    return multiVector_wedge_roundPoint(flector_antiWedge_sphere(self, other), sphere_dual(other));
}

Circle line_rejectViaOriginFrom_circle(Line self, Circle other) {
    return roundPoint_wedge_dipole(line_antiWedge_circle(self, other), circle_dual(other));
}

Circle line_rejectViaOriginFrom_dipole(Line self, Dipole other) {
    return scalar_wedge_circle(line_antiWedge_dipole(self, other), dipole_dual(other));
}

MultiVector line_rejectViaOriginFrom_flector(Line self, Flector other) {
    return flatPoint_wedge_multiVector(line_antiWedge_flector(self, other), flector_dual(other));
}

Circle line_rejectViaOriginFrom_line(Line self, Line other) {
    return roundPoint_wedge_dipole(line_antiWedge_line(self, other), line_dual(other));
}

MultiVector line_rejectViaOriginFrom_motor(Line self, Motor other) {
    return multiVector_wedge_multiVector(line_antiWedge_motor(self, other), motor_dual(other));
}

MultiVector line_rejectViaOriginFrom_multiVector(Line self, MultiVector other) {
    return multiVector_wedge_multiVector(line_antiWedge_multiVector(self, other), multiVector_dual(other));
}

Line line_rejectViaOriginFrom_plane(Line self, Plane other) {
    return flatPoint_wedge_roundPoint(line_antiWedge_plane(self, other), plane_dual(other));
}

Circle line_rejectViaOriginFrom_sphere(Line self, Sphere other) {
    return dipole_wedge_roundPoint(line_antiWedge_sphere(self, other), sphere_dual(other));
}

MultiVector motor_rejectViaOriginFrom_circle(Motor self, Circle other) {
    return multiVector_wedge_dipole(motor_antiWedge_circle(self, other), circle_dual(other));
}

MultiVector motor_rejectViaOriginFrom_dipole(Motor self, Dipole other) {
    return multiVector_wedge_circle(motor_antiWedge_dipole(self, other), dipole_dual(other));
}

AntiScalar motor_rejectViaOriginFrom_flatPoint(Motor self, FlatPoint other) {
    return flatPoint_wedge_circle(motor_antiWedge_flatPoint(self, other), flatPoint_dual(other));
}

MultiVector motor_rejectViaOriginFrom_flector(Motor self, Flector other) {
    return flector_wedge_multiVector(motor_antiWedge_flector(self, other), flector_dual(other));
}

MultiVector motor_rejectViaOriginFrom_line(Motor self, Line other) {
    return multiVector_wedge_dipole(motor_antiWedge_line(self, other), line_dual(other));
}

MultiVector motor_rejectViaOriginFrom_motor(Motor self, Motor other) {
    return multiVector_wedge_multiVector(motor_antiWedge_motor(self, other), motor_dual(other));
}

MultiVector motor_rejectViaOriginFrom_multiVector(Motor self, MultiVector other) {
    return multiVector_wedge_multiVector(motor_antiWedge_multiVector(self, other), multiVector_dual(other));
}

Motor motor_rejectViaOriginFrom_plane(Motor self, Plane other) {
    return flector_wedge_roundPoint(motor_antiWedge_plane(self, other), plane_dual(other));
}

AntiScalar motor_rejectViaOriginFrom_roundPoint(Motor self, RoundPoint other) {
    return roundPoint_wedge_sphere(motor_antiWedge_roundPoint(self, other), roundPoint_dual(other));
}

MultiVector motor_rejectViaOriginFrom_sphere(Motor self, Sphere other) {
    return multiVector_wedge_roundPoint(motor_antiWedge_sphere(self, other), sphere_dual(other));
}

MultiVector multiVector_rejectViaOriginFrom_circle(MultiVector self, Circle other) {
    return multiVector_wedge_dipole(multiVector_antiWedge_circle(self, other), circle_dual(other));
}

MultiVector multiVector_rejectViaOriginFrom_dipole(MultiVector self, Dipole other) {
    return multiVector_wedge_circle(multiVector_antiWedge_dipole(self, other), dipole_dual(other));
}

MultiVector multiVector_rejectViaOriginFrom_flatPoint(MultiVector self, FlatPoint other) {
    return multiVector_wedge_circle(multiVector_antiWedge_flatPoint(self, other), flatPoint_dual(other));
}

MultiVector multiVector_rejectViaOriginFrom_flector(MultiVector self, Flector other) {
    return multiVector_wedge_multiVector(multiVector_antiWedge_flector(self, other), flector_dual(other));
}

MultiVector multiVector_rejectViaOriginFrom_line(MultiVector self, Line other) {
    return multiVector_wedge_dipole(multiVector_antiWedge_line(self, other), line_dual(other));
}

MultiVector multiVector_rejectViaOriginFrom_motor(MultiVector self, Motor other) {
    return multiVector_wedge_multiVector(multiVector_antiWedge_motor(self, other), motor_dual(other));
}

MultiVector multiVector_rejectViaOriginFrom_multiVector(MultiVector self, MultiVector other) {
    return multiVector_wedge_multiVector(multiVector_antiWedge_multiVector(self, other), multiVector_dual(other));
}

MultiVector multiVector_rejectViaOriginFrom_plane(MultiVector self, Plane other) {
    return multiVector_wedge_roundPoint(multiVector_antiWedge_plane(self, other), plane_dual(other));
}

MultiVector multiVector_rejectViaOriginFrom_roundPoint(MultiVector self, RoundPoint other) {
    return multiVector_wedge_sphere(multiVector_antiWedge_roundPoint(self, other), roundPoint_dual(other));
}

MultiVector multiVector_rejectViaOriginFrom_sphere(MultiVector self, Sphere other) {
    return multiVector_wedge_roundPoint(multiVector_antiWedge_sphere(self, other), sphere_dual(other));
}

Sphere plane_rejectViaOriginFrom_circle(Plane self, Circle other) {
    return dipole_wedge_dipole(plane_antiWedge_circle(self, other), circle_dual(other));
}

Sphere plane_rejectViaOriginFrom_dipole(Plane self, Dipole other) {
    return roundPoint_wedge_circle(plane_antiWedge_dipole(self, other), dipole_dual(other));
}

Sphere plane_rejectViaOriginFrom_flatPoint(Plane self, FlatPoint other) {
    return roundPoint_wedge_circle(plane_antiWedge_flatPoint(self, other), flatPoint_dual(other));
}

MultiVector plane_rejectViaOriginFrom_flector(Plane self, Flector other) {
    return multiVector_wedge_multiVector(plane_antiWedge_flector(self, other), flector_dual(other));
}

Plane plane_rejectViaOriginFrom_line(Plane self, Line other) {
    return flatPoint_wedge_dipole(plane_antiWedge_line(self, other), line_dual(other));
}

MultiVector plane_rejectViaOriginFrom_motor(Plane self, Motor other) {
    return flector_wedge_multiVector(plane_antiWedge_motor(self, other), motor_dual(other));
}

MultiVector plane_rejectViaOriginFrom_multiVector(Plane self, MultiVector other) {
    return multiVector_wedge_multiVector(plane_antiWedge_multiVector(self, other), multiVector_dual(other));
}

Plane plane_rejectViaOriginFrom_plane(Plane self, Plane other) {
    return line_wedge_roundPoint(plane_antiWedge_plane(self, other), plane_dual(other));
}

Sphere plane_rejectViaOriginFrom_roundPoint(Plane self, RoundPoint other) {
    return scalar_wedge_sphere(plane_antiWedge_roundPoint(self, other), roundPoint_dual(other));
}

Sphere plane_rejectViaOriginFrom_sphere(Plane self, Sphere other) {
    return circle_wedge_roundPoint(plane_antiWedge_sphere(self, other), sphere_dual(other));
}

MultiVector roundPoint_rejectViaOriginFrom_flector(RoundPoint self, Flector other) {
    return scalar_wedge_multiVector(roundPoint_antiWedge_flector(self, other), flector_dual(other));
}

MultiVector roundPoint_rejectViaOriginFrom_motor(RoundPoint self, Motor other) {
    return roundPoint_wedge_multiVector(roundPoint_antiWedge_motor(self, other), motor_dual(other));
}

MultiVector roundPoint_rejectViaOriginFrom_multiVector(RoundPoint self, MultiVector other) {
    return multiVector_wedge_multiVector(roundPoint_antiWedge_multiVector(self, other), multiVector_dual(other));
}

RoundPoint roundPoint_rejectViaOriginFrom_plane(RoundPoint self, Plane other) {
    return scalar_wedge_roundPoint(roundPoint_antiWedge_plane(self, other), plane_dual(other));
}

RoundPoint roundPoint_rejectViaOriginFrom_sphere(RoundPoint self, Sphere other) {
    return scalar_wedge_roundPoint(roundPoint_antiWedge_sphere(self, other), sphere_dual(other));
}

Sphere sphere_rejectViaOriginFrom_circle(Sphere self, Circle other) {
    return dipole_wedge_dipole(sphere_antiWedge_circle(self, other), circle_dual(other));
}

Sphere sphere_rejectViaOriginFrom_dipole(Sphere self, Dipole other) {
    return roundPoint_wedge_circle(sphere_antiWedge_dipole(self, other), dipole_dual(other));
}

Sphere sphere_rejectViaOriginFrom_flatPoint(Sphere self, FlatPoint other) {
    return roundPoint_wedge_circle(sphere_antiWedge_flatPoint(self, other), flatPoint_dual(other));
}

MultiVector sphere_rejectViaOriginFrom_flector(Sphere self, Flector other) {
    return multiVector_wedge_multiVector(sphere_antiWedge_flector(self, other), flector_dual(other));
}

Sphere sphere_rejectViaOriginFrom_line(Sphere self, Line other) {
    return dipole_wedge_dipole(sphere_antiWedge_line(self, other), line_dual(other));
}

MultiVector sphere_rejectViaOriginFrom_motor(Sphere self, Motor other) {
    return multiVector_wedge_multiVector(sphere_antiWedge_motor(self, other), motor_dual(other));
}

MultiVector sphere_rejectViaOriginFrom_multiVector(Sphere self, MultiVector other) {
    return multiVector_wedge_multiVector(sphere_antiWedge_multiVector(self, other), multiVector_dual(other));
}

Sphere sphere_rejectViaOriginFrom_plane(Sphere self, Plane other) {
    return circle_wedge_roundPoint(sphere_antiWedge_plane(self, other), plane_dual(other));
}

Sphere sphere_rejectViaOriginFrom_roundPoint(Sphere self, RoundPoint other) {
    return scalar_wedge_sphere(sphere_antiWedge_roundPoint(self, other), roundPoint_dual(other));
}

Sphere sphere_rejectViaOriginFrom_sphere(Sphere self, Sphere other) {
    return circle_wedge_roundPoint(sphere_antiWedge_sphere(self, other), sphere_dual(other));
}

Sphere circle_antiSupport(Circle self) {
    return circle_wedge_roundPoint(self, sphere_antiWedge_dipole(roundPoint_complement(RoundPoint(vec3(0.0), vec2(1.0, 0.0))), circle_dual(self)));
}

Sphere dipole_antiSupport(Dipole self) {
    return dipole_wedge_dipole(self, sphere_antiWedge_circle(roundPoint_complement(RoundPoint(vec3(0.0), vec2(1.0, 0.0))), dipole_dual(self)));
}

Sphere dualNum_antiSupport(DualNum self) {
    return dualNum_wedge_sphere(self, sphere_antiWedge_dualNum(roundPoint_complement(RoundPoint(vec3(0.0), vec2(1.0, 0.0))), dualNum_dual(self)));
}

Plane flatPoint_antiSupport(FlatPoint self) {
    return flatPoint_wedge_dipole(self, sphere_antiWedge_circle(roundPoint_complement(RoundPoint(vec3(0.0), vec2(1.0, 0.0))), flatPoint_dual(self)));
}

MultiVector flector_antiSupport(Flector self) {
    return flector_wedge_multiVector(self, sphere_antiWedge_multiVector(roundPoint_complement(RoundPoint(vec3(0.0), vec2(1.0, 0.0))), flector_dual(self)));
}

Plane line_antiSupport(Line self) {
    return line_wedge_roundPoint(self, sphere_antiWedge_dipole(roundPoint_complement(RoundPoint(vec3(0.0), vec2(1.0, 0.0))), line_dual(self)));
}

MultiVector motor_antiSupport(Motor self) {
    return motor_wedge_multiVector(self, sphere_antiWedge_multiVector(roundPoint_complement(RoundPoint(vec3(0.0), vec2(1.0, 0.0))), motor_dual(self)));
}

MultiVector multiVector_antiSupport(MultiVector self) {
    return multiVector_wedge_multiVector(self, sphere_antiWedge_multiVector(roundPoint_complement(RoundPoint(vec3(0.0), vec2(1.0, 0.0))), multiVector_dual(self)));
}

Plane plane_antiSupport(Plane self) {
    return plane_wedge_scalar(self, sphere_antiWedge_roundPoint(roundPoint_complement(RoundPoint(vec3(0.0), vec2(1.0, 0.0))), plane_dual(self)));
}

Sphere roundPoint_antiSupport(RoundPoint self) {
    return roundPoint_wedge_circle(self, sphere_antiWedge_sphere(roundPoint_complement(RoundPoint(vec3(0.0), vec2(1.0, 0.0))), roundPoint_dual(self)));
}

Sphere scalar_antiSupport(Scalar self) {
    return scalar_wedge_sphere(self, sphere_antiWedge_antiScalar(roundPoint_complement(RoundPoint(vec3(0.0), vec2(1.0, 0.0))), scalar_dual(self)));
}

Sphere sphere_antiSupport(Sphere self) {
    return sphere_wedge_scalar(self, sphere_antiWedge_roundPoint(roundPoint_complement(RoundPoint(vec3(0.0), vec2(1.0, 0.0))), sphere_dual(self)));
}

RoundPoint antiScalar_support(AntiScalar self) {
    return antiScalar_antiWedge_roundPoint(self, roundPoint_wedge_scalar(RoundPoint(vec3(0.0), vec2(1.0, 0.0)), antiScalar_antiDual(self)));
}

RoundPoint circle_support(Circle self) {
    return circle_antiWedge_circle(self, roundPoint_wedge_dipole(RoundPoint(vec3(0.0), vec2(1.0, 0.0)), circle_antiDual(self)));
}

RoundPoint dipole_support(Dipole self) {
    return dipole_antiWedge_sphere(self, roundPoint_wedge_circle(RoundPoint(vec3(0.0), vec2(1.0, 0.0)), dipole_antiDual(self)));
}

RoundPoint dualNum_support(DualNum self) {
    return dualNum_antiWedge_roundPoint(self, roundPoint_wedge_dualNum(RoundPoint(vec3(0.0), vec2(1.0, 0.0)), dualNum_antiDual(self)));
}

RoundPoint flatPoint_support(FlatPoint self) {
    return flatPoint_antiWedge_sphere(self, roundPoint_wedge_circle(RoundPoint(vec3(0.0), vec2(1.0, 0.0)), flatPoint_antiDual(self)));
}

MultiVector flector_support(Flector self) {
    return flector_antiWedge_multiVector(self, roundPoint_wedge_multiVector(RoundPoint(vec3(0.0), vec2(1.0, 0.0)), flector_antiDual(self)));
}

RoundPoint line_support(Line self) {
    return line_antiWedge_circle(self, roundPoint_wedge_dipole(RoundPoint(vec3(0.0), vec2(1.0, 0.0)), line_antiDual(self)));
}

MultiVector motor_support(Motor self) {
    return motor_antiWedge_multiVector(self, roundPoint_wedge_multiVector(RoundPoint(vec3(0.0), vec2(1.0, 0.0)), motor_antiDual(self)));
}

MultiVector multiVector_support(MultiVector self) {
    return multiVector_antiWedge_multiVector(self, roundPoint_wedge_multiVector(RoundPoint(vec3(0.0), vec2(1.0, 0.0)), multiVector_antiDual(self)));
}

RoundPoint plane_support(Plane self) {
    return plane_antiWedge_dipole(self, roundPoint_wedge_roundPoint(RoundPoint(vec3(0.0), vec2(1.0, 0.0)), plane_antiDual(self)));
}

RoundPoint roundPoint_support(RoundPoint self) {
    return roundPoint_antiWedge_antiScalar(self, roundPoint_wedge_sphere(RoundPoint(vec3(0.0), vec2(1.0, 0.0)), roundPoint_antiDual(self)));
}

RoundPoint sphere_support(Sphere self) {
    return sphere_antiWedge_dipole(self, roundPoint_wedge_roundPoint(RoundPoint(vec3(0.0), vec2(1.0, 0.0)), sphere_antiDual(self)));
}


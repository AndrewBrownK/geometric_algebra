struct Scalar {
    // 1
    float g0;
};

struct AntiScalar {
    // e1234
    float g0;
};

struct Magnitude {
    // 1, e1234
    vec2 g0;
};

struct Point {
    // e1, e2, e3, e4
    vec4 g0;
};

struct Origin {
    // e4
    float g0;
};

struct PointAtInfinity {
    // e1, e2, e3
    vec3 g0;
};

struct Line {
    // -e14, -e24, -e34
    vec3 g0;
    // e23, -e13, e12
    vec3 g1;
};

struct LineAtOrigin {
    // -e14, -e24, -e34
    vec3 g0;
};

struct LineAtInfinity {
    // e23, -e13, e12
    vec3 g0;
};

struct Plane {
    // e234, -e134, e124, -e123
    vec4 g0;
};

struct PlaneAtOrigin {
    // e234, -e134, e124
    vec3 g0;
};

struct Horizon {
    // -e123
    float g0;
};

struct Motor {
    // -e14, -e24, -e34, e1234
    vec4 g0;
    // e23, -e13, e12
    vec3 g1;
};

struct Rotor {
    // -e14, -e24, -e34, e1234
    vec4 g0;
};

struct Translator {
    // e23, -e13, e12, e1234
    vec4 g0;
};

struct Flector {
    // e1, e2, e3, e4
    vec4 g0;
    // e234, -e134, e124, -e123
    vec4 g1;
};

struct MultiVector {
    // 1, e1234
    vec2 g0;
    // e1, e2, e3, e4
    vec4 g1;
    // -e14, -e24, -e34
    vec3 g2;
    // e23, -e13, e12
    vec3 g3;
    // e234, -e134, e124, -e123
    vec4 g4;
};

AntiScalar anti_scalar_one() {
    return AntiScalar(0.0);
}

Flector flector_one() {
    return Flector(vec4(0.0), vec4(0.0));
}

Horizon horizon_one() {
    return Horizon(0.0);
}

Line line_one() {
    return Line(vec3(0.0), vec3(0.0));
}

LineAtInfinity line_at_infinity_one() {
    return LineAtInfinity(vec3(0.0));
}

LineAtOrigin line_at_origin_one() {
    return LineAtOrigin(vec3(0.0));
}

Magnitude magnitude_one() {
    return Magnitude(vec2(1.0, 0.0));
}

Motor motor_one() {
    return Motor(vec4(0.0), vec3(0.0));
}

MultiVector multi_vector_one() {
    return MultiVector(vec2(1.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec4(0.0));
}

Origin origin_one() {
    return Origin(0.0);
}

Plane plane_one() {
    return Plane(vec4(0.0));
}

PlaneAtOrigin plane_at_origin_one() {
    return PlaneAtOrigin(vec3(0.0));
}

Point point_one() {
    return Point(vec4(0.0));
}

PointAtInfinity point_at_infinity_one() {
    return PointAtInfinity(vec3(0.0));
}

Rotor rotor_one() {
    return Rotor(vec4(0.0));
}

Scalar scalar_one() {
    return Scalar(1.0);
}

Translator translator_one() {
    return Translator(vec4(0.0));
}

AntiScalar anti_scalar_zero() {
    return AntiScalar(0.0);
}

Flector flector_zero() {
    return Flector(vec4(0.0), vec4(0.0));
}

Horizon horizon_zero() {
    return Horizon(0.0);
}

Line line_zero() {
    return Line(vec3(0.0), vec3(0.0));
}

LineAtInfinity line_at_infinity_zero() {
    return LineAtInfinity(vec3(0.0));
}

LineAtOrigin line_at_origin_zero() {
    return LineAtOrigin(vec3(0.0));
}

Magnitude magnitude_zero() {
    return Magnitude(vec2(0.0));
}

Motor motor_zero() {
    return Motor(vec4(0.0), vec3(0.0));
}

MultiVector multi_vector_zero() {
    return MultiVector(vec2(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec4(0.0));
}

Origin origin_zero() {
    return Origin(0.0);
}

Plane plane_zero() {
    return Plane(vec4(0.0));
}

PlaneAtOrigin plane_at_origin_zero() {
    return PlaneAtOrigin(vec3(0.0));
}

Point point_zero() {
    return Point(vec4(0.0));
}

PointAtInfinity point_at_infinity_zero() {
    return PointAtInfinity(vec3(0.0));
}

Rotor rotor_zero() {
    return Rotor(vec4(0.0));
}

Scalar scalar_zero() {
    return Scalar(0.0);
}

Translator translator_zero() {
    return Translator(vec4(0.0));
}

AntiScalar anti_scalar_neg(AntiScalar self) {
    return AntiScalar(self.g0 * -1.0);
}

Flector flector_neg(Flector self) {
    return Flector(self.g0 * vec4(-1.0), self.g1 * vec4(1.0, -1.0, 1.0, -1.0));
}

Horizon horizon_neg(Horizon self) {
    return Horizon(self.g0 * -1.0);
}

Line line_neg(Line self) {
    return Line(self.g0 * vec3(-1.0, 1.0, -1.0), self.g1 * vec3(-1.0));
}

LineAtInfinity line_at_infinity_neg(LineAtInfinity self) {
    return LineAtInfinity(self.g0 * vec3(-1.0));
}

LineAtOrigin line_at_origin_neg(LineAtOrigin self) {
    return LineAtOrigin(self.g0 * vec3(-1.0, 1.0, -1.0));
}

Magnitude magnitude_neg(Magnitude self) {
    return Magnitude(self.g0 * vec2(-1.0));
}

Motor motor_neg(Motor self) {
    return Motor(self.g0 * vec4(-1.0, 1.0, -1.0, -1.0), self.g1 * vec3(-1.0));
}

MultiVector multi_vector_neg(MultiVector self) {
    return MultiVector(self.g0 * vec2(-1.0), self.g1 * vec4(-1.0), self.g2 * vec3(-1.0, 1.0, -1.0), self.g3 * vec3(-1.0), self.g4 * vec4(1.0, -1.0, 1.0, -1.0));
}

Origin origin_neg(Origin self) {
    return Origin(self.g0 * -1.0);
}

Plane plane_neg(Plane self) {
    return Plane(self.g0 * vec4(1.0, -1.0, 1.0, -1.0));
}

PlaneAtOrigin plane_at_origin_neg(PlaneAtOrigin self) {
    return PlaneAtOrigin(self.g0 * vec3(1.0, -1.0, 1.0));
}

Point point_neg(Point self) {
    return Point(self.g0 * vec4(-1.0));
}

PointAtInfinity point_at_infinity_neg(PointAtInfinity self) {
    return PointAtInfinity(self.g0 * vec3(-1.0));
}

Rotor rotor_neg(Rotor self) {
    return Rotor(self.g0 * vec4(-1.0, 1.0, -1.0, -1.0));
}

Scalar scalar_neg(Scalar self) {
    return Scalar(self.g0 * -1.0);
}

Translator translator_neg(Translator self) {
    return Translator(self.g0 * vec4(-1.0));
}

AntiScalar anti_scalar_anti_scalar_add(AntiScalar self, AntiScalar other) {
    return AntiScalar(self.g0 + other.g0);
}

Motor anti_scalar_line_add(AntiScalar self, Line other) {
    return Motor(vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0), other.g1);
}

Translator anti_scalar_line_at_infinity_add(AntiScalar self, LineAtInfinity other) {
    return Translator(vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Rotor anti_scalar_line_at_origin_add(AntiScalar self, LineAtOrigin other) {
    return Rotor(vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Magnitude anti_scalar_magnitude_add(AntiScalar self, Magnitude other) {
    return Magnitude(vec2(self.g0) * vec2(0.0, 1.0) + other.g0);
}

Motor anti_scalar_motor_add(AntiScalar self, Motor other) {
    return Motor(vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) + other.g0, other.g1);
}

MultiVector anti_scalar_multi_vector_add(AntiScalar self, MultiVector other) {
    return MultiVector(vec2(self.g0) * vec2(0.0, 1.0) + other.g0, other.g1, other.g2, other.g3, other.g4);
}

Rotor anti_scalar_rotor_add(AntiScalar self, Rotor other) {
    return Rotor(vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) + other.g0);
}

Magnitude anti_scalar_scalar_add(AntiScalar self, Scalar other) {
    return Magnitude(vec2(self.g0) * vec2(0.0, 1.0) + vec2(other.g0) * vec2(1.0, 0.0));
}

Translator anti_scalar_translator_add(AntiScalar self, Translator other) {
    return Translator(vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) + other.g0);
}

Flector flector_flector_add(Flector self, Flector other) {
    return Flector(self.g0 + other.g0, self.g1 + other.g1);
}

Flector flector_horizon_add(Flector self, Horizon other) {
    return Flector(self.g0, self.g1 + vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0));
}

MultiVector flector_multi_vector_add(Flector self, MultiVector other) {
    return MultiVector(other.g0, self.g0 + other.g1, other.g2, other.g3, self.g1 + other.g4);
}

Flector flector_origin_add(Flector self, Origin other) {
    return Flector(self.g0 + vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0), self.g1);
}

Flector flector_plane_add(Flector self, Plane other) {
    return Flector(self.g0, self.g1 + other.g0);
}

Flector flector_plane_at_origin_add(Flector self, PlaneAtOrigin other) {
    return Flector(self.g0, self.g1 + vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Flector flector_point_add(Flector self, Point other) {
    return Flector(self.g0 + other.g0, self.g1);
}

Flector flector_point_at_infinity_add(Flector self, PointAtInfinity other) {
    return Flector(self.g0 + vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0), self.g1);
}

Flector horizon_flector_add(Horizon self, Flector other) {
    return Flector(other.g0, vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) + other.g1);
}

Horizon horizon_horizon_add(Horizon self, Horizon other) {
    return Horizon(self.g0 + other.g0);
}

MultiVector horizon_multi_vector_add(Horizon self, MultiVector other) {
    return MultiVector(other.g0, other.g1, other.g2, other.g3, vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) + other.g4);
}

Plane horizon_plane_add(Horizon self, Plane other) {
    return Plane(vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) + other.g0);
}

Plane horizon_plane_at_origin_add(Horizon self, PlaneAtOrigin other) {
    return Plane(vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Motor line_anti_scalar_add(Line self, AntiScalar other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0), self.g1);
}

Line line_line_add(Line self, Line other) {
    return Line(self.g0 + other.g0, self.g1 + other.g1);
}

Line line_line_at_infinity_add(Line self, LineAtInfinity other) {
    return Line(self.g0, self.g1 + other.g0);
}

Line line_line_at_origin_add(Line self, LineAtOrigin other) {
    return Line(self.g0 + other.g0, self.g1);
}

Motor line_motor_add(Line self, Motor other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + other.g0, self.g1 + other.g1);
}

MultiVector line_multi_vector_add(Line self, MultiVector other) {
    return MultiVector(other.g0, other.g1, self.g0 + other.g2, self.g1 + other.g3, other.g4);
}

Motor line_rotor_add(Line self, Rotor other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + other.g0, self.g1);
}

Motor line_translator_add(Line self, Translator other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0), self.g1 + vec3(other.g0.x, other.g0.y, other.g0.z));
}

Translator line_at_infinity_anti_scalar_add(LineAtInfinity self, AntiScalar other) {
    return Translator(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0));
}

Line line_at_infinity_line_add(LineAtInfinity self, Line other) {
    return Line(other.g0, self.g0 + other.g1);
}

LineAtInfinity line_at_infinity_line_at_infinity_add(LineAtInfinity self, LineAtInfinity other) {
    return LineAtInfinity(self.g0 + other.g0);
}

Line line_at_infinity_line_at_origin_add(LineAtInfinity self, LineAtOrigin other) {
    return Line(other.g0, self.g0);
}

Motor line_at_infinity_motor_add(LineAtInfinity self, Motor other) {
    return Motor(other.g0, self.g0 + other.g1);
}

MultiVector line_at_infinity_multi_vector_add(LineAtInfinity self, MultiVector other) {
    return MultiVector(other.g0, other.g1, other.g2, self.g0 + other.g3, other.g4);
}

Motor line_at_infinity_rotor_add(LineAtInfinity self, Rotor other) {
    return Motor(other.g0, self.g0);
}

Translator line_at_infinity_translator_add(LineAtInfinity self, Translator other) {
    return Translator(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + other.g0);
}

Rotor line_at_origin_anti_scalar_add(LineAtOrigin self, AntiScalar other) {
    return Rotor(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0));
}

Line line_at_origin_line_add(LineAtOrigin self, Line other) {
    return Line(self.g0 + other.g0, other.g1);
}

Line line_at_origin_line_at_infinity_add(LineAtOrigin self, LineAtInfinity other) {
    return Line(self.g0, other.g0);
}

LineAtOrigin line_at_origin_line_at_origin_add(LineAtOrigin self, LineAtOrigin other) {
    return LineAtOrigin(self.g0 + other.g0);
}

Motor line_at_origin_motor_add(LineAtOrigin self, Motor other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + other.g0, other.g1);
}

MultiVector line_at_origin_multi_vector_add(LineAtOrigin self, MultiVector other) {
    return MultiVector(other.g0, other.g1, self.g0 + other.g2, other.g3, other.g4);
}

Rotor line_at_origin_rotor_add(LineAtOrigin self, Rotor other) {
    return Rotor(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + other.g0);
}

Motor line_at_origin_translator_add(LineAtOrigin self, Translator other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0), vec3(other.g0.x, other.g0.y, other.g0.z));
}

Magnitude magnitude_anti_scalar_add(Magnitude self, AntiScalar other) {
    return Magnitude(self.g0 + vec2(other.g0) * vec2(0.0, 1.0));
}

Magnitude magnitude_magnitude_add(Magnitude self, Magnitude other) {
    return Magnitude(self.g0 + other.g0);
}

MultiVector magnitude_multi_vector_add(Magnitude self, MultiVector other) {
    return MultiVector(self.g0 + other.g0, other.g1, other.g2, other.g3, other.g4);
}

Magnitude magnitude_scalar_add(Magnitude self, Scalar other) {
    return Magnitude(self.g0 + vec2(other.g0) * vec2(1.0, 0.0));
}

Motor motor_anti_scalar_add(Motor self, AntiScalar other) {
    return Motor(self.g0 + vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0), self.g1);
}

Motor motor_line_add(Motor self, Line other) {
    return Motor(self.g0 + vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0), self.g1 + other.g1);
}

Motor motor_line_at_infinity_add(Motor self, LineAtInfinity other) {
    return Motor(self.g0, self.g1 + other.g0);
}

Motor motor_line_at_origin_add(Motor self, LineAtOrigin other) {
    return Motor(self.g0 + vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0), self.g1);
}

Motor motor_motor_add(Motor self, Motor other) {
    return Motor(self.g0 + other.g0, self.g1 + other.g1);
}

MultiVector motor_multi_vector_add(Motor self, MultiVector other) {
    return MultiVector(vec2(self.g0.x, self.g0.w) * vec2(0.0, 1.0) + other.g0, other.g1, vec3(self.g0.x, self.g0.y, self.g0.z) + other.g2, self.g1 + other.g3, other.g4);
}

Motor motor_rotor_add(Motor self, Rotor other) {
    return Motor(self.g0 + other.g0, self.g1);
}

Motor motor_translator_add(Motor self, Translator other) {
    return Motor(self.g0 + other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0), self.g1 + vec3(other.g0.x, other.g0.y, other.g0.z));
}

MultiVector multi_vector_anti_scalar_add(MultiVector self, AntiScalar other) {
    return MultiVector(self.g0 + vec2(other.g0) * vec2(0.0, 1.0), self.g1, self.g2, self.g3, self.g4);
}

MultiVector multi_vector_flector_add(MultiVector self, Flector other) {
    return MultiVector(self.g0, self.g1 + other.g0, self.g2, self.g3, self.g4 + other.g1);
}

MultiVector multi_vector_horizon_add(MultiVector self, Horizon other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3, self.g4 + vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0));
}

MultiVector multi_vector_line_add(MultiVector self, Line other) {
    return MultiVector(self.g0, self.g1, self.g2 + other.g0, self.g3 + other.g1, self.g4);
}

MultiVector multi_vector_line_at_infinity_add(MultiVector self, LineAtInfinity other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3 + other.g0, self.g4);
}

MultiVector multi_vector_line_at_origin_add(MultiVector self, LineAtOrigin other) {
    return MultiVector(self.g0, self.g1, self.g2 + other.g0, self.g3, self.g4);
}

MultiVector multi_vector_magnitude_add(MultiVector self, Magnitude other) {
    return MultiVector(self.g0 + other.g0, self.g1, self.g2, self.g3, self.g4);
}

MultiVector multi_vector_motor_add(MultiVector self, Motor other) {
    return MultiVector(self.g0 + vec2(other.g0.x, other.g0.w) * vec2(0.0, 1.0), self.g1, self.g2 + vec3(other.g0.x, other.g0.y, other.g0.z), self.g3 + other.g1, self.g4);
}

MultiVector multi_vector_multi_vector_add(MultiVector self, MultiVector other) {
    return MultiVector(self.g0 + other.g0, self.g1 + other.g1, self.g2 + other.g2, self.g3 + other.g3, self.g4 + other.g4);
}

MultiVector multi_vector_origin_add(MultiVector self, Origin other) {
    return MultiVector(self.g0, self.g1 + vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0), self.g2, self.g3, self.g4);
}

MultiVector multi_vector_plane_add(MultiVector self, Plane other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3, self.g4 + other.g0);
}

MultiVector multi_vector_plane_at_origin_add(MultiVector self, PlaneAtOrigin other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3, self.g4 + vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

MultiVector multi_vector_point_add(MultiVector self, Point other) {
    return MultiVector(self.g0, self.g1 + other.g0, self.g2, self.g3, self.g4);
}

MultiVector multi_vector_point_at_infinity_add(MultiVector self, PointAtInfinity other) {
    return MultiVector(self.g0, self.g1 + vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0), self.g2, self.g3, self.g4);
}

MultiVector multi_vector_rotor_add(MultiVector self, Rotor other) {
    return MultiVector(self.g0 + vec2(other.g0.x, other.g0.w) * vec2(0.0, 1.0), self.g1, self.g2 + vec3(other.g0.x, other.g0.y, other.g0.z), self.g3, self.g4);
}

MultiVector multi_vector_scalar_add(MultiVector self, Scalar other) {
    return MultiVector(self.g0 + vec2(other.g0) * vec2(1.0, 0.0), self.g1, self.g2, self.g3, self.g4);
}

MultiVector multi_vector_translator_add(MultiVector self, Translator other) {
    return MultiVector(self.g0 + vec2(other.g0.x, other.g0.w) * vec2(0.0, 1.0), self.g1, self.g2, self.g3 + vec3(other.g0.x, other.g0.y, other.g0.z), self.g4);
}

Flector origin_flector_add(Origin self, Flector other) {
    return Flector(vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) + other.g0, other.g1);
}

MultiVector origin_multi_vector_add(Origin self, MultiVector other) {
    return MultiVector(other.g0, vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) + other.g1, other.g2, other.g3, other.g4);
}

Origin origin_origin_add(Origin self, Origin other) {
    return Origin(self.g0 + other.g0);
}

Point origin_point_add(Origin self, Point other) {
    return Point(vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) + other.g0);
}

Point origin_point_at_infinity_add(Origin self, PointAtInfinity other) {
    return Point(vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Flector plane_flector_add(Plane self, Flector other) {
    return Flector(other.g0, self.g0 + other.g1);
}

Plane plane_horizon_add(Plane self, Horizon other) {
    return Plane(self.g0 + vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0));
}

MultiVector plane_multi_vector_add(Plane self, MultiVector other) {
    return MultiVector(other.g0, other.g1, other.g2, other.g3, self.g0 + other.g4);
}

Plane plane_plane_add(Plane self, Plane other) {
    return Plane(self.g0 + other.g0);
}

Plane plane_plane_at_origin_add(Plane self, PlaneAtOrigin other) {
    return Plane(self.g0 + vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Flector plane_point_add(Plane self, Point other) {
    return Flector(other.g0, self.g0);
}

Flector plane_at_origin_flector_add(PlaneAtOrigin self, Flector other) {
    return Flector(other.g0, vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + other.g1);
}

Plane plane_at_origin_horizon_add(PlaneAtOrigin self, Horizon other) {
    return Plane(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0));
}

MultiVector plane_at_origin_multi_vector_add(PlaneAtOrigin self, MultiVector other) {
    return MultiVector(other.g0, other.g1, other.g2, other.g3, vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + other.g4);
}

Plane plane_at_origin_plane_add(PlaneAtOrigin self, Plane other) {
    return Plane(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + other.g0);
}

PlaneAtOrigin plane_at_origin_plane_at_origin_add(PlaneAtOrigin self, PlaneAtOrigin other) {
    return PlaneAtOrigin(self.g0 + other.g0);
}

Flector point_flector_add(Point self, Flector other) {
    return Flector(self.g0 + other.g0, other.g1);
}

MultiVector point_multi_vector_add(Point self, MultiVector other) {
    return MultiVector(other.g0, self.g0 + other.g1, other.g2, other.g3, other.g4);
}

Point point_origin_add(Point self, Origin other) {
    return Point(self.g0 + vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0));
}

Flector point_plane_add(Point self, Plane other) {
    return Flector(self.g0, other.g0);
}

Point point_point_add(Point self, Point other) {
    return Point(self.g0 + other.g0);
}

Point point_point_at_infinity_add(Point self, PointAtInfinity other) {
    return Point(self.g0 + vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Flector point_at_infinity_flector_add(PointAtInfinity self, Flector other) {
    return Flector(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + other.g0, other.g1);
}

MultiVector point_at_infinity_multi_vector_add(PointAtInfinity self, MultiVector other) {
    return MultiVector(other.g0, vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + other.g1, other.g2, other.g3, other.g4);
}

Point point_at_infinity_origin_add(PointAtInfinity self, Origin other) {
    return Point(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0));
}

Point point_at_infinity_point_add(PointAtInfinity self, Point other) {
    return Point(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + other.g0);
}

PointAtInfinity point_at_infinity_point_at_infinity_add(PointAtInfinity self, PointAtInfinity other) {
    return PointAtInfinity(self.g0 + other.g0);
}

Rotor rotor_anti_scalar_add(Rotor self, AntiScalar other) {
    return Rotor(self.g0 + vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0));
}

Motor rotor_line_add(Rotor self, Line other) {
    return Motor(self.g0 + vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0), other.g1);
}

Motor rotor_line_at_infinity_add(Rotor self, LineAtInfinity other) {
    return Motor(self.g0, other.g0);
}

Rotor rotor_line_at_origin_add(Rotor self, LineAtOrigin other) {
    return Rotor(self.g0 + vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Motor rotor_motor_add(Rotor self, Motor other) {
    return Motor(self.g0 + other.g0, other.g1);
}

MultiVector rotor_multi_vector_add(Rotor self, MultiVector other) {
    return MultiVector(vec2(self.g0.x, self.g0.w) * vec2(0.0, 1.0) + other.g0, other.g1, vec3(self.g0.x, self.g0.y, self.g0.z) + other.g2, other.g3, other.g4);
}

Rotor rotor_rotor_add(Rotor self, Rotor other) {
    return Rotor(self.g0 + other.g0);
}

Motor rotor_translator_add(Rotor self, Translator other) {
    return Motor(self.g0 + other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0), vec3(other.g0.x, other.g0.y, other.g0.z));
}

Magnitude scalar_anti_scalar_add(Scalar self, AntiScalar other) {
    return Magnitude(vec2(self.g0) * vec2(1.0, 0.0) + vec2(other.g0) * vec2(0.0, 1.0));
}

Magnitude scalar_magnitude_add(Scalar self, Magnitude other) {
    return Magnitude(vec2(self.g0) * vec2(1.0, 0.0) + other.g0);
}

MultiVector scalar_multi_vector_add(Scalar self, MultiVector other) {
    return MultiVector(vec2(self.g0) * vec2(1.0, 0.0) + other.g0, other.g1, other.g2, other.g3, other.g4);
}

Scalar scalar_scalar_add(Scalar self, Scalar other) {
    return Scalar(self.g0 + other.g0);
}

Translator translator_anti_scalar_add(Translator self, AntiScalar other) {
    return Translator(self.g0 + vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0));
}

Motor translator_line_add(Translator self, Line other) {
    return Motor(self.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0) + vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0), vec3(self.g0.x, self.g0.y, self.g0.z) + other.g1);
}

Translator translator_line_at_infinity_add(Translator self, LineAtInfinity other) {
    return Translator(self.g0 + vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Motor translator_line_at_origin_add(Translator self, LineAtOrigin other) {
    return Motor(self.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0) + vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0), vec3(self.g0.x, self.g0.y, self.g0.z));
}

Motor translator_motor_add(Translator self, Motor other) {
    return Motor(self.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0) + other.g0, vec3(self.g0.x, self.g0.y, self.g0.z) + other.g1);
}

MultiVector translator_multi_vector_add(Translator self, MultiVector other) {
    return MultiVector(vec2(self.g0.x, self.g0.w) * vec2(0.0, 1.0) + other.g0, other.g1, other.g2, vec3(self.g0.x, self.g0.y, self.g0.z) + other.g3, other.g4);
}

Motor translator_rotor_add(Translator self, Rotor other) {
    return Motor(self.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0) + other.g0, vec3(self.g0.x, self.g0.y, self.g0.z));
}

Translator translator_translator_add(Translator self, Translator other) {
    return Translator(self.g0 + other.g0);
}

AntiScalar anti_scalar_anti_scalar_div(AntiScalar self, AntiScalar other) {
    return AntiScalar(self.g0 * 1.0 / other.g0 * 1.0);
}

Flector flector_flector_div(Flector self, Flector other) {
    return Flector(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.w) * vec4(1.0, 1.0, 1.0, 1.0), vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.w) * vec4(1.0, 1.0, 1.0, 1.0));
}

Horizon horizon_horizon_div(Horizon self, Horizon other) {
    return Horizon(self.g0 * 1.0 / other.g0 * 1.0);
}

Line line_line_div(Line self, Line other) {
    return Line(vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(1.0, 1.0, 1.0) / vec3(other.g0.x, other.g0.y, other.g0.z) * vec3(1.0, 1.0, 1.0), vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(1.0, 1.0, 1.0) / vec3(other.g1.x, other.g1.y, other.g1.z) * vec3(1.0, 1.0, 1.0));
}

LineAtInfinity line_at_infinity_line_at_infinity_div(LineAtInfinity self, LineAtInfinity other) {
    return LineAtInfinity(vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(1.0, 1.0, 1.0) / vec3(other.g0.x, other.g0.y, other.g0.z) * vec3(1.0, 1.0, 1.0));
}

LineAtOrigin line_at_origin_line_at_origin_div(LineAtOrigin self, LineAtOrigin other) {
    return LineAtOrigin(vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(1.0, 1.0, 1.0) / vec3(other.g0.x, other.g0.y, other.g0.z) * vec3(1.0, 1.0, 1.0));
}

Magnitude magnitude_magnitude_div(Magnitude self, Magnitude other) {
    return Magnitude(vec2(self.g0.x, self.g0.y) * vec2(1.0, 1.0) / vec2(other.g0.x, other.g0.y) * vec2(1.0, 1.0));
}

Motor motor_motor_div(Motor self, Motor other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.w) * vec4(1.0, 1.0, 1.0, 1.0), vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(1.0, 1.0, 1.0) / vec3(other.g1.x, other.g1.y, other.g1.z) * vec3(1.0, 1.0, 1.0));
}

MultiVector multi_vector_multi_vector_div(MultiVector self, MultiVector other) {
    return MultiVector(vec2(self.g0.x, self.g0.y) * vec2(1.0, 1.0) / vec2(other.g0.x, other.g0.y) * vec2(1.0, 1.0), vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.w) * vec4(1.0, 1.0, 1.0, 1.0), vec3(self.g2.x, self.g2.y, self.g2.z) * vec3(1.0, 1.0, 1.0) / vec3(other.g2.x, other.g2.y, other.g2.z) * vec3(1.0, 1.0, 1.0), vec3(self.g3.x, self.g3.y, self.g3.z) * vec3(1.0, 1.0, 1.0) / vec3(other.g3.x, other.g3.y, other.g3.z) * vec3(1.0, 1.0, 1.0), vec4(self.g4.x, self.g4.y, self.g4.z, self.g4.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g4.x, other.g4.y, other.g4.z, other.g4.w) * vec4(1.0, 1.0, 1.0, 1.0));
}

Origin origin_origin_div(Origin self, Origin other) {
    return Origin(self.g0 * 1.0 / other.g0 * 1.0);
}

Plane plane_plane_div(Plane self, Plane other) {
    return Plane(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.w) * vec4(1.0, 1.0, 1.0, 1.0));
}

PlaneAtOrigin plane_at_origin_plane_at_origin_div(PlaneAtOrigin self, PlaneAtOrigin other) {
    return PlaneAtOrigin(vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(1.0, 1.0, 1.0) / vec3(other.g0.x, other.g0.y, other.g0.z) * vec3(1.0, 1.0, 1.0));
}

Point point_point_div(Point self, Point other) {
    return Point(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.w) * vec4(1.0, 1.0, 1.0, 1.0));
}

PointAtInfinity point_at_infinity_point_at_infinity_div(PointAtInfinity self, PointAtInfinity other) {
    return PointAtInfinity(vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(1.0, 1.0, 1.0) / vec3(other.g0.x, other.g0.y, other.g0.z) * vec3(1.0, 1.0, 1.0));
}

Rotor rotor_rotor_div(Rotor self, Rotor other) {
    return Rotor(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.w) * vec4(1.0, 1.0, 1.0, 1.0));
}

Scalar scalar_scalar_div(Scalar self, Scalar other) {
    return Scalar(self.g0 * 1.0 / other.g0 * 1.0);
}

Translator translator_translator_div(Translator self, Translator other) {
    return Translator(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.w) * vec4(1.0, 1.0, 1.0, 1.0));
}

Horizon flector_horizon_into(Flector self) {
    return Horizon(self.g1.w);
}

Origin flector_origin_into(Flector self) {
    return Origin(self.g0.w);
}

Plane flector_plane_into(Flector self) {
    return Plane(self.g1);
}

PlaneAtOrigin flector_plane_at_origin_into(Flector self) {
    return PlaneAtOrigin(vec3(self.g1.x, self.g1.y, self.g1.z));
}

Point flector_point_into(Flector self) {
    return Point(self.g0);
}

PointAtInfinity flector_point_at_infinity_into(Flector self) {
    return PointAtInfinity(vec3(self.g0.x, self.g0.y, self.g0.z));
}

LineAtInfinity line_line_at_infinity_into(Line self) {
    return LineAtInfinity(self.g1);
}

LineAtOrigin line_line_at_origin_into(Line self) {
    return LineAtOrigin(self.g0);
}

AntiScalar magnitude_anti_scalar_into(Magnitude self) {
    return AntiScalar(self.g0.y);
}

Scalar magnitude_scalar_into(Magnitude self) {
    return Scalar(self.g0.x);
}

AntiScalar motor_anti_scalar_into(Motor self) {
    return AntiScalar(self.g0.w);
}

Line motor_line_into(Motor self) {
    return Line(vec3(self.g0.x, self.g0.y, self.g0.z), self.g1);
}

LineAtInfinity motor_line_at_infinity_into(Motor self) {
    return LineAtInfinity(self.g1);
}

LineAtOrigin motor_line_at_origin_into(Motor self) {
    return LineAtOrigin(vec3(self.g0.x, self.g0.y, self.g0.z));
}

Rotor motor_rotor_into(Motor self) {
    return Rotor(self.g0);
}

Translator motor_translator_into(Motor self) {
    return Translator(vec4(self.g1.x, self.g1.y, self.g1.z, self.g0.w));
}

AntiScalar multi_vector_anti_scalar_into(MultiVector self) {
    return AntiScalar(self.g0.y);
}

Flector multi_vector_flector_into(MultiVector self) {
    return Flector(self.g1, self.g4);
}

Horizon multi_vector_horizon_into(MultiVector self) {
    return Horizon(self.g4.w);
}

Line multi_vector_line_into(MultiVector self) {
    return Line(self.g2, self.g3);
}

LineAtInfinity multi_vector_line_at_infinity_into(MultiVector self) {
    return LineAtInfinity(self.g3);
}

LineAtOrigin multi_vector_line_at_origin_into(MultiVector self) {
    return LineAtOrigin(self.g2);
}

Magnitude multi_vector_magnitude_into(MultiVector self) {
    return Magnitude(self.g0);
}

Motor multi_vector_motor_into(MultiVector self) {
    return Motor(vec4(self.g2.x, self.g2.y, self.g2.z, self.g0.y), self.g3);
}

Origin multi_vector_origin_into(MultiVector self) {
    return Origin(self.g1.w);
}

Plane multi_vector_plane_into(MultiVector self) {
    return Plane(self.g4);
}

PlaneAtOrigin multi_vector_plane_at_origin_into(MultiVector self) {
    return PlaneAtOrigin(vec3(self.g4.x, self.g4.y, self.g4.z));
}

Point multi_vector_point_into(MultiVector self) {
    return Point(self.g1);
}

PointAtInfinity multi_vector_point_at_infinity_into(MultiVector self) {
    return PointAtInfinity(vec3(self.g1.x, self.g1.y, self.g1.z));
}

Rotor multi_vector_rotor_into(MultiVector self) {
    return Rotor(vec4(self.g2.x, self.g2.y, self.g2.z, self.g0.y));
}

Scalar multi_vector_scalar_into(MultiVector self) {
    return Scalar(self.g0.x);
}

Translator multi_vector_translator_into(MultiVector self) {
    return Translator(vec4(self.g3.x, self.g3.y, self.g3.z, self.g0.y));
}

Horizon plane_horizon_into(Plane self) {
    return Horizon(self.g0.w);
}

PlaneAtOrigin plane_plane_at_origin_into(Plane self) {
    return PlaneAtOrigin(vec3(self.g0.x, self.g0.y, self.g0.z));
}

Origin point_origin_into(Point self) {
    return Origin(self.g0.w);
}

PointAtInfinity point_point_at_infinity_into(Point self) {
    return PointAtInfinity(vec3(self.g0.x, self.g0.y, self.g0.z));
}

AntiScalar rotor_anti_scalar_into(Rotor self) {
    return AntiScalar(self.g0.w);
}

LineAtOrigin rotor_line_at_origin_into(Rotor self) {
    return LineAtOrigin(vec3(self.g0.x, self.g0.y, self.g0.z));
}

AntiScalar translator_anti_scalar_into(Translator self) {
    return AntiScalar(self.g0.w);
}

LineAtInfinity translator_line_at_infinity_into(Translator self) {
    return LineAtInfinity(vec3(self.g0.x, self.g0.y, self.g0.z));
}

AntiScalar anti_scalar_anti_scalar_mul(AntiScalar self, AntiScalar other) {
    return AntiScalar(self.g0 * other.g0);
}

Flector flector_flector_mul(Flector self, Flector other) {
    return Flector(self.g0 * other.g0, self.g1 * other.g1);
}

Horizon horizon_horizon_mul(Horizon self, Horizon other) {
    return Horizon(self.g0 * other.g0);
}

Line line_line_mul(Line self, Line other) {
    return Line(self.g0 * other.g0, self.g1 * other.g1);
}

LineAtInfinity line_at_infinity_line_at_infinity_mul(LineAtInfinity self, LineAtInfinity other) {
    return LineAtInfinity(self.g0 * other.g0);
}

LineAtOrigin line_at_origin_line_at_origin_mul(LineAtOrigin self, LineAtOrigin other) {
    return LineAtOrigin(self.g0 * other.g0);
}

Magnitude magnitude_magnitude_mul(Magnitude self, Magnitude other) {
    return Magnitude(self.g0 * other.g0);
}

Motor motor_motor_mul(Motor self, Motor other) {
    return Motor(self.g0 * other.g0, self.g1 * other.g1);
}

MultiVector multi_vector_multi_vector_mul(MultiVector self, MultiVector other) {
    return MultiVector(self.g0 * other.g0, self.g1 * other.g1, self.g2 * other.g2, self.g3 * other.g3, self.g4 * other.g4);
}

Origin origin_origin_mul(Origin self, Origin other) {
    return Origin(self.g0 * other.g0);
}

Plane plane_plane_mul(Plane self, Plane other) {
    return Plane(self.g0 * other.g0);
}

PlaneAtOrigin plane_at_origin_plane_at_origin_mul(PlaneAtOrigin self, PlaneAtOrigin other) {
    return PlaneAtOrigin(self.g0 * other.g0);
}

Point point_point_mul(Point self, Point other) {
    return Point(self.g0 * other.g0);
}

PointAtInfinity point_at_infinity_point_at_infinity_mul(PointAtInfinity self, PointAtInfinity other) {
    return PointAtInfinity(self.g0 * other.g0);
}

Rotor rotor_rotor_mul(Rotor self, Rotor other) {
    return Rotor(self.g0 * other.g0);
}

Scalar scalar_scalar_mul(Scalar self, Scalar other) {
    return Scalar(self.g0 * other.g0);
}

Translator translator_translator_mul(Translator self, Translator other) {
    return Translator(self.g0 * other.g0);
}

AntiScalar anti_scalar_anti_scalar_sub(AntiScalar self, AntiScalar other) {
    return AntiScalar(self.g0 - other.g0);
}

Motor anti_scalar_line_sub(AntiScalar self, Line other) {
    return Motor(vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) - vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0), vec3(0.0) - other.g1);
}

Translator anti_scalar_line_at_infinity_sub(AntiScalar self, LineAtInfinity other) {
    return Translator(vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) - vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Rotor anti_scalar_line_at_origin_sub(AntiScalar self, LineAtOrigin other) {
    return Rotor(vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) - vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Magnitude anti_scalar_magnitude_sub(AntiScalar self, Magnitude other) {
    return Magnitude(vec2(self.g0) * vec2(0.0, 1.0) - other.g0);
}

Motor anti_scalar_motor_sub(AntiScalar self, Motor other) {
    return Motor(vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) - other.g0, vec3(0.0) - other.g1);
}

MultiVector anti_scalar_multi_vector_sub(AntiScalar self, MultiVector other) {
    return MultiVector(vec2(self.g0) * vec2(0.0, 1.0) - other.g0, vec4(0.0) - other.g1, vec3(0.0) - other.g2, vec3(0.0) - other.g3, vec4(0.0) - other.g4);
}

Rotor anti_scalar_rotor_sub(AntiScalar self, Rotor other) {
    return Rotor(vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) - other.g0);
}

Magnitude anti_scalar_scalar_sub(AntiScalar self, Scalar other) {
    return Magnitude(vec2(self.g0) * vec2(0.0, 1.0) - vec2(other.g0) * vec2(1.0, 0.0));
}

Translator anti_scalar_translator_sub(AntiScalar self, Translator other) {
    return Translator(vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) - other.g0);
}

Flector flector_flector_sub(Flector self, Flector other) {
    return Flector(self.g0 - other.g0, self.g1 - other.g1);
}

Flector flector_horizon_sub(Flector self, Horizon other) {
    return Flector(self.g0, self.g1 - vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0));
}

MultiVector flector_multi_vector_sub(Flector self, MultiVector other) {
    return MultiVector(vec2(0.0) - other.g0, self.g0 - other.g1, vec3(0.0) - other.g2, vec3(0.0) - other.g3, self.g1 - other.g4);
}

Flector flector_origin_sub(Flector self, Origin other) {
    return Flector(self.g0 - vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0), self.g1);
}

Flector flector_plane_sub(Flector self, Plane other) {
    return Flector(self.g0, self.g1 - other.g0);
}

Flector flector_plane_at_origin_sub(Flector self, PlaneAtOrigin other) {
    return Flector(self.g0, self.g1 - vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Flector flector_point_sub(Flector self, Point other) {
    return Flector(self.g0 - other.g0, self.g1);
}

Flector flector_point_at_infinity_sub(Flector self, PointAtInfinity other) {
    return Flector(self.g0 - vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0), self.g1);
}

Flector horizon_flector_sub(Horizon self, Flector other) {
    return Flector(vec4(0.0) - other.g0, vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) - other.g1);
}

Horizon horizon_horizon_sub(Horizon self, Horizon other) {
    return Horizon(self.g0 - other.g0);
}

MultiVector horizon_multi_vector_sub(Horizon self, MultiVector other) {
    return MultiVector(vec2(0.0) - other.g0, vec4(0.0) - other.g1, vec3(0.0) - other.g2, vec3(0.0) - other.g3, vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) - other.g4);
}

Plane horizon_plane_sub(Horizon self, Plane other) {
    return Plane(vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) - other.g0);
}

Plane horizon_plane_at_origin_sub(Horizon self, PlaneAtOrigin other) {
    return Plane(vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) - vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Motor line_anti_scalar_sub(Line self, AntiScalar other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) - vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0), self.g1);
}

Line line_line_sub(Line self, Line other) {
    return Line(self.g0 - other.g0, self.g1 - other.g1);
}

Line line_line_at_infinity_sub(Line self, LineAtInfinity other) {
    return Line(self.g0, self.g1 - other.g0);
}

Line line_line_at_origin_sub(Line self, LineAtOrigin other) {
    return Line(self.g0 - other.g0, self.g1);
}

Motor line_motor_sub(Line self, Motor other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) - other.g0, self.g1 - other.g1);
}

MultiVector line_multi_vector_sub(Line self, MultiVector other) {
    return MultiVector(vec2(0.0) - other.g0, vec4(0.0) - other.g1, self.g0 - other.g2, self.g1 - other.g3, vec4(0.0) - other.g4);
}

Motor line_rotor_sub(Line self, Rotor other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) - other.g0, self.g1);
}

Motor line_translator_sub(Line self, Translator other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) - other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0), self.g1 - vec3(other.g0.x, other.g0.y, other.g0.z));
}

Translator line_at_infinity_anti_scalar_sub(LineAtInfinity self, AntiScalar other) {
    return Translator(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) - vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0));
}

Line line_at_infinity_line_sub(LineAtInfinity self, Line other) {
    return Line(vec3(0.0) - other.g0, self.g0 - other.g1);
}

LineAtInfinity line_at_infinity_line_at_infinity_sub(LineAtInfinity self, LineAtInfinity other) {
    return LineAtInfinity(self.g0 - other.g0);
}

Line line_at_infinity_line_at_origin_sub(LineAtInfinity self, LineAtOrigin other) {
    return Line(vec3(0.0) - other.g0, self.g0);
}

Motor line_at_infinity_motor_sub(LineAtInfinity self, Motor other) {
    return Motor(vec4(0.0) - other.g0, self.g0 - other.g1);
}

MultiVector line_at_infinity_multi_vector_sub(LineAtInfinity self, MultiVector other) {
    return MultiVector(vec2(0.0) - other.g0, vec4(0.0) - other.g1, vec3(0.0) - other.g2, self.g0 - other.g3, vec4(0.0) - other.g4);
}

Motor line_at_infinity_rotor_sub(LineAtInfinity self, Rotor other) {
    return Motor(vec4(0.0) - other.g0, self.g0);
}

Translator line_at_infinity_translator_sub(LineAtInfinity self, Translator other) {
    return Translator(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) - other.g0);
}

Rotor line_at_origin_anti_scalar_sub(LineAtOrigin self, AntiScalar other) {
    return Rotor(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) - vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0));
}

Line line_at_origin_line_sub(LineAtOrigin self, Line other) {
    return Line(self.g0 - other.g0, vec3(0.0) - other.g1);
}

Line line_at_origin_line_at_infinity_sub(LineAtOrigin self, LineAtInfinity other) {
    return Line(self.g0, vec3(0.0) - other.g0);
}

LineAtOrigin line_at_origin_line_at_origin_sub(LineAtOrigin self, LineAtOrigin other) {
    return LineAtOrigin(self.g0 - other.g0);
}

Motor line_at_origin_motor_sub(LineAtOrigin self, Motor other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) - other.g0, vec3(0.0) - other.g1);
}

MultiVector line_at_origin_multi_vector_sub(LineAtOrigin self, MultiVector other) {
    return MultiVector(vec2(0.0) - other.g0, vec4(0.0) - other.g1, self.g0 - other.g2, vec3(0.0) - other.g3, vec4(0.0) - other.g4);
}

Rotor line_at_origin_rotor_sub(LineAtOrigin self, Rotor other) {
    return Rotor(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) - other.g0);
}

Motor line_at_origin_translator_sub(LineAtOrigin self, Translator other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) - other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0), vec3(0.0) - vec3(other.g0.x, other.g0.y, other.g0.z));
}

Magnitude magnitude_anti_scalar_sub(Magnitude self, AntiScalar other) {
    return Magnitude(self.g0 - vec2(other.g0) * vec2(0.0, 1.0));
}

Magnitude magnitude_magnitude_sub(Magnitude self, Magnitude other) {
    return Magnitude(self.g0 - other.g0);
}

MultiVector magnitude_multi_vector_sub(Magnitude self, MultiVector other) {
    return MultiVector(self.g0 - other.g0, vec4(0.0) - other.g1, vec3(0.0) - other.g2, vec3(0.0) - other.g3, vec4(0.0) - other.g4);
}

Magnitude magnitude_scalar_sub(Magnitude self, Scalar other) {
    return Magnitude(self.g0 - vec2(other.g0) * vec2(1.0, 0.0));
}

Motor motor_anti_scalar_sub(Motor self, AntiScalar other) {
    return Motor(self.g0 - vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0), self.g1);
}

Motor motor_line_sub(Motor self, Line other) {
    return Motor(self.g0 - vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0), self.g1 - other.g1);
}

Motor motor_line_at_infinity_sub(Motor self, LineAtInfinity other) {
    return Motor(self.g0, self.g1 - other.g0);
}

Motor motor_line_at_origin_sub(Motor self, LineAtOrigin other) {
    return Motor(self.g0 - vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0), self.g1);
}

Motor motor_motor_sub(Motor self, Motor other) {
    return Motor(self.g0 - other.g0, self.g1 - other.g1);
}

MultiVector motor_multi_vector_sub(Motor self, MultiVector other) {
    return MultiVector(vec2(self.g0.x, self.g0.w) * vec2(0.0, 1.0) - other.g0, vec4(0.0) - other.g1, vec3(self.g0.x, self.g0.y, self.g0.z) - other.g2, self.g1 - other.g3, vec4(0.0) - other.g4);
}

Motor motor_rotor_sub(Motor self, Rotor other) {
    return Motor(self.g0 - other.g0, self.g1);
}

Motor motor_translator_sub(Motor self, Translator other) {
    return Motor(self.g0 - other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0), self.g1 - vec3(other.g0.x, other.g0.y, other.g0.z));
}

MultiVector multi_vector_anti_scalar_sub(MultiVector self, AntiScalar other) {
    return MultiVector(self.g0 - vec2(other.g0) * vec2(0.0, 1.0), self.g1, self.g2, self.g3, self.g4);
}

MultiVector multi_vector_flector_sub(MultiVector self, Flector other) {
    return MultiVector(self.g0, self.g1 - other.g0, self.g2, self.g3, self.g4 - other.g1);
}

MultiVector multi_vector_horizon_sub(MultiVector self, Horizon other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3, self.g4 - vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0));
}

MultiVector multi_vector_line_sub(MultiVector self, Line other) {
    return MultiVector(self.g0, self.g1, self.g2 - other.g0, self.g3 - other.g1, self.g4);
}

MultiVector multi_vector_line_at_infinity_sub(MultiVector self, LineAtInfinity other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3 - other.g0, self.g4);
}

MultiVector multi_vector_line_at_origin_sub(MultiVector self, LineAtOrigin other) {
    return MultiVector(self.g0, self.g1, self.g2 - other.g0, self.g3, self.g4);
}

MultiVector multi_vector_magnitude_sub(MultiVector self, Magnitude other) {
    return MultiVector(self.g0 - other.g0, self.g1, self.g2, self.g3, self.g4);
}

MultiVector multi_vector_motor_sub(MultiVector self, Motor other) {
    return MultiVector(self.g0 - vec2(other.g0.x, other.g0.w) * vec2(0.0, 1.0), self.g1, self.g2 - vec3(other.g0.x, other.g0.y, other.g0.z), self.g3 - other.g1, self.g4);
}

MultiVector multi_vector_multi_vector_sub(MultiVector self, MultiVector other) {
    return MultiVector(self.g0 - other.g0, self.g1 - other.g1, self.g2 - other.g2, self.g3 - other.g3, self.g4 - other.g4);
}

MultiVector multi_vector_origin_sub(MultiVector self, Origin other) {
    return MultiVector(self.g0, self.g1 - vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0), self.g2, self.g3, self.g4);
}

MultiVector multi_vector_plane_sub(MultiVector self, Plane other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3, self.g4 - other.g0);
}

MultiVector multi_vector_plane_at_origin_sub(MultiVector self, PlaneAtOrigin other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3, self.g4 - vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

MultiVector multi_vector_point_sub(MultiVector self, Point other) {
    return MultiVector(self.g0, self.g1 - other.g0, self.g2, self.g3, self.g4);
}

MultiVector multi_vector_point_at_infinity_sub(MultiVector self, PointAtInfinity other) {
    return MultiVector(self.g0, self.g1 - vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0), self.g2, self.g3, self.g4);
}

MultiVector multi_vector_rotor_sub(MultiVector self, Rotor other) {
    return MultiVector(self.g0 - vec2(other.g0.x, other.g0.w) * vec2(0.0, 1.0), self.g1, self.g2 - vec3(other.g0.x, other.g0.y, other.g0.z), self.g3, self.g4);
}

MultiVector multi_vector_scalar_sub(MultiVector self, Scalar other) {
    return MultiVector(self.g0 - vec2(other.g0) * vec2(1.0, 0.0), self.g1, self.g2, self.g3, self.g4);
}

MultiVector multi_vector_translator_sub(MultiVector self, Translator other) {
    return MultiVector(self.g0 - vec2(other.g0.x, other.g0.w) * vec2(0.0, 1.0), self.g1, self.g2, self.g3 - vec3(other.g0.x, other.g0.y, other.g0.z), self.g4);
}

Flector origin_flector_sub(Origin self, Flector other) {
    return Flector(vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) - other.g0, vec4(0.0) - other.g1);
}

MultiVector origin_multi_vector_sub(Origin self, MultiVector other) {
    return MultiVector(vec2(0.0) - other.g0, vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) - other.g1, vec3(0.0) - other.g2, vec3(0.0) - other.g3, vec4(0.0) - other.g4);
}

Origin origin_origin_sub(Origin self, Origin other) {
    return Origin(self.g0 - other.g0);
}

Point origin_point_sub(Origin self, Point other) {
    return Point(vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) - other.g0);
}

Point origin_point_at_infinity_sub(Origin self, PointAtInfinity other) {
    return Point(vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) - vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Flector plane_flector_sub(Plane self, Flector other) {
    return Flector(vec4(0.0) - other.g0, self.g0 - other.g1);
}

Plane plane_horizon_sub(Plane self, Horizon other) {
    return Plane(self.g0 - vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0));
}

MultiVector plane_multi_vector_sub(Plane self, MultiVector other) {
    return MultiVector(vec2(0.0) - other.g0, vec4(0.0) - other.g1, vec3(0.0) - other.g2, vec3(0.0) - other.g3, self.g0 - other.g4);
}

Plane plane_plane_sub(Plane self, Plane other) {
    return Plane(self.g0 - other.g0);
}

Plane plane_plane_at_origin_sub(Plane self, PlaneAtOrigin other) {
    return Plane(self.g0 - vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Flector plane_point_sub(Plane self, Point other) {
    return Flector(vec4(0.0) - other.g0, self.g0);
}

Flector plane_at_origin_flector_sub(PlaneAtOrigin self, Flector other) {
    return Flector(vec4(0.0) - other.g0, vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) - other.g1);
}

Plane plane_at_origin_horizon_sub(PlaneAtOrigin self, Horizon other) {
    return Plane(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) - vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0));
}

MultiVector plane_at_origin_multi_vector_sub(PlaneAtOrigin self, MultiVector other) {
    return MultiVector(vec2(0.0) - other.g0, vec4(0.0) - other.g1, vec3(0.0) - other.g2, vec3(0.0) - other.g3, vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) - other.g4);
}

Plane plane_at_origin_plane_sub(PlaneAtOrigin self, Plane other) {
    return Plane(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) - other.g0);
}

PlaneAtOrigin plane_at_origin_plane_at_origin_sub(PlaneAtOrigin self, PlaneAtOrigin other) {
    return PlaneAtOrigin(self.g0 - other.g0);
}

Flector point_flector_sub(Point self, Flector other) {
    return Flector(self.g0 - other.g0, vec4(0.0) - other.g1);
}

MultiVector point_multi_vector_sub(Point self, MultiVector other) {
    return MultiVector(vec2(0.0) - other.g0, self.g0 - other.g1, vec3(0.0) - other.g2, vec3(0.0) - other.g3, vec4(0.0) - other.g4);
}

Point point_origin_sub(Point self, Origin other) {
    return Point(self.g0 - vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0));
}

Flector point_plane_sub(Point self, Plane other) {
    return Flector(self.g0, vec4(0.0) - other.g0);
}

Point point_point_sub(Point self, Point other) {
    return Point(self.g0 - other.g0);
}

Point point_point_at_infinity_sub(Point self, PointAtInfinity other) {
    return Point(self.g0 - vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Flector point_at_infinity_flector_sub(PointAtInfinity self, Flector other) {
    return Flector(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) - other.g0, vec4(0.0) - other.g1);
}

MultiVector point_at_infinity_multi_vector_sub(PointAtInfinity self, MultiVector other) {
    return MultiVector(vec2(0.0) - other.g0, vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) - other.g1, vec3(0.0) - other.g2, vec3(0.0) - other.g3, vec4(0.0) - other.g4);
}

Point point_at_infinity_origin_sub(PointAtInfinity self, Origin other) {
    return Point(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) - vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0));
}

Point point_at_infinity_point_sub(PointAtInfinity self, Point other) {
    return Point(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) - other.g0);
}

PointAtInfinity point_at_infinity_point_at_infinity_sub(PointAtInfinity self, PointAtInfinity other) {
    return PointAtInfinity(self.g0 - other.g0);
}

Rotor rotor_anti_scalar_sub(Rotor self, AntiScalar other) {
    return Rotor(self.g0 - vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0));
}

Motor rotor_line_sub(Rotor self, Line other) {
    return Motor(self.g0 - vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0), vec3(0.0) - other.g1);
}

Motor rotor_line_at_infinity_sub(Rotor self, LineAtInfinity other) {
    return Motor(self.g0, vec3(0.0) - other.g0);
}

Rotor rotor_line_at_origin_sub(Rotor self, LineAtOrigin other) {
    return Rotor(self.g0 - vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Motor rotor_motor_sub(Rotor self, Motor other) {
    return Motor(self.g0 - other.g0, vec3(0.0) - other.g1);
}

MultiVector rotor_multi_vector_sub(Rotor self, MultiVector other) {
    return MultiVector(vec2(self.g0.x, self.g0.w) * vec2(0.0, 1.0) - other.g0, vec4(0.0) - other.g1, vec3(self.g0.x, self.g0.y, self.g0.z) - other.g2, vec3(0.0) - other.g3, vec4(0.0) - other.g4);
}

Rotor rotor_rotor_sub(Rotor self, Rotor other) {
    return Rotor(self.g0 - other.g0);
}

Motor rotor_translator_sub(Rotor self, Translator other) {
    return Motor(self.g0 - other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0), vec3(0.0) - vec3(other.g0.x, other.g0.y, other.g0.z));
}

Magnitude scalar_anti_scalar_sub(Scalar self, AntiScalar other) {
    return Magnitude(vec2(self.g0) * vec2(1.0, 0.0) - vec2(other.g0) * vec2(0.0, 1.0));
}

Magnitude scalar_magnitude_sub(Scalar self, Magnitude other) {
    return Magnitude(vec2(self.g0) * vec2(1.0, 0.0) - other.g0);
}

MultiVector scalar_multi_vector_sub(Scalar self, MultiVector other) {
    return MultiVector(vec2(self.g0) * vec2(1.0, 0.0) - other.g0, vec4(0.0) - other.g1, vec3(0.0) - other.g2, vec3(0.0) - other.g3, vec4(0.0) - other.g4);
}

Scalar scalar_scalar_sub(Scalar self, Scalar other) {
    return Scalar(self.g0 - other.g0);
}

Translator translator_anti_scalar_sub(Translator self, AntiScalar other) {
    return Translator(self.g0 - vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0));
}

Motor translator_line_sub(Translator self, Line other) {
    return Motor(self.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0) - vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0), vec3(self.g0.x, self.g0.y, self.g0.z) - other.g1);
}

Translator translator_line_at_infinity_sub(Translator self, LineAtInfinity other) {
    return Translator(self.g0 - vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Motor translator_line_at_origin_sub(Translator self, LineAtOrigin other) {
    return Motor(self.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0) - vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0), vec3(self.g0.x, self.g0.y, self.g0.z));
}

Motor translator_motor_sub(Translator self, Motor other) {
    return Motor(self.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0) - other.g0, vec3(self.g0.x, self.g0.y, self.g0.z) - other.g1);
}

MultiVector translator_multi_vector_sub(Translator self, MultiVector other) {
    return MultiVector(vec2(self.g0.x, self.g0.w) * vec2(0.0, 1.0) - other.g0, vec4(0.0) - other.g1, vec3(0.0) - other.g2, vec3(self.g0.x, self.g0.y, self.g0.z) - other.g3, vec4(0.0) - other.g4);
}

Motor translator_rotor_sub(Translator self, Rotor other) {
    return Motor(self.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0) - other.g0, vec3(self.g0.x, self.g0.y, self.g0.z));
}

Translator translator_translator_sub(Translator self, Translator other) {
    return Translator(self.g0 - other.g0);
}


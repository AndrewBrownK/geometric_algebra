struct Scalar {
    // 1
    float g0;
};

struct AntiScalar {
    // e0123
    float g0;
};

struct HomogeneousMagnitude {
    // 1, e0123
    vec2 g0;
};

struct Point {
    // e0, e1, e2, e3
    vec4 g0;
};

struct Line {
    // -e03, -e13, -e23
    vec3 g0;
    // e12, -e02, e01
    vec3 g1;
};

struct Plane {
    // e123, -e023, e013, -e012
    vec4 g0;
};

struct Motor {
    // -e03, -e13, -e23, e0123
    vec4 g0;
    // e12, -e02, e01
    vec3 g1;
};

struct Rotor {
    // -e03, -e13, -e23, e0123
    vec4 g0;
};

struct Translator {
    // e12, -e02, e01, e0123
    vec4 g0;
};

struct Flector {
    // e0, e1, e2, e3
    vec4 g0;
    // e123, -e023, e013, -e012
    vec4 g1;
};

struct MultiVector {
    // 1, e0123
    vec2 g0;
    // e0, e1, e2, e3
    vec4 g1;
    // -e03, -e13, -e23
    vec3 g2;
    // e12, -e02, e01
    vec3 g3;
    // e123, -e023, e013, -e012
    vec4 g4;
};

Scalar scalar_zero() {
    return Scalar(0.0);
}

Scalar scalar_one() {
    return Scalar(1.0);
}

int scalar_grade(Scalar self) {
    return 0;
}

int scalar_anti_grade(Scalar self) {
    return 4;
}

Scalar scalar_neg(Scalar self) {
    return Scalar(self.g0 * -1.0);
}

Scalar scalar_automorphism(Scalar self) {
    return Scalar(self.g0);
}

Scalar scalar_reversal(Scalar self) {
    return Scalar(self.g0);
}

Scalar scalar_conjugation(Scalar self) {
    return Scalar(self.g0);
}

AntiScalar scalar_dual(Scalar self) {
    return AntiScalar(self.g0);
}

Scalar scalar_anti_reversal(Scalar self) {
    return Scalar(self.g0);
}

AntiScalar scalar_right_complement(Scalar self) {
    return AntiScalar(self.g0);
}

AntiScalar scalar_left_complement(Scalar self) {
    return AntiScalar(self.g0);
}

Scalar scalar_double_complement(Scalar self) {
    return Scalar(self.g0);
}

AntiScalar anti_scalar_zero() {
    return AntiScalar(0.0);
}

AntiScalar anti_scalar_one() {
    return AntiScalar(0.0);
}

int anti_scalar_grade(AntiScalar self) {
    return 4;
}

int anti_scalar_anti_grade(AntiScalar self) {
    return 0;
}

AntiScalar anti_scalar_neg(AntiScalar self) {
    return AntiScalar(self.g0 * -1.0);
}

AntiScalar anti_scalar_automorphism(AntiScalar self) {
    return AntiScalar(self.g0);
}

AntiScalar anti_scalar_reversal(AntiScalar self) {
    return AntiScalar(self.g0);
}

AntiScalar anti_scalar_conjugation(AntiScalar self) {
    return AntiScalar(self.g0);
}

Scalar anti_scalar_dual(AntiScalar self) {
    return Scalar(self.g0);
}

AntiScalar anti_scalar_anti_reversal(AntiScalar self) {
    return AntiScalar(self.g0);
}

Scalar anti_scalar_right_complement(AntiScalar self) {
    return Scalar(self.g0);
}

Scalar anti_scalar_left_complement(AntiScalar self) {
    return Scalar(self.g0);
}

AntiScalar anti_scalar_double_complement(AntiScalar self) {
    return AntiScalar(self.g0);
}

HomogeneousMagnitude homogeneous_magnitude_zero() {
    return HomogeneousMagnitude(vec2(0.0));
}

HomogeneousMagnitude homogeneous_magnitude_one() {
    return HomogeneousMagnitude(vec2(1.0, 0.0));
}

HomogeneousMagnitude homogeneous_magnitude_neg(HomogeneousMagnitude self) {
    return HomogeneousMagnitude(self.g0 * vec2(-1.0));
}

HomogeneousMagnitude homogeneous_magnitude_automorphism(HomogeneousMagnitude self) {
    return HomogeneousMagnitude(self.g0);
}

HomogeneousMagnitude homogeneous_magnitude_reversal(HomogeneousMagnitude self) {
    return HomogeneousMagnitude(self.g0);
}

HomogeneousMagnitude homogeneous_magnitude_conjugation(HomogeneousMagnitude self) {
    return HomogeneousMagnitude(self.g0);
}

HomogeneousMagnitude homogeneous_magnitude_dual(HomogeneousMagnitude self) {
    return HomogeneousMagnitude(self.g0.yx);
}

HomogeneousMagnitude homogeneous_magnitude_anti_reversal(HomogeneousMagnitude self) {
    return HomogeneousMagnitude(self.g0);
}

HomogeneousMagnitude homogeneous_magnitude_right_complement(HomogeneousMagnitude self) {
    return HomogeneousMagnitude(self.g0.yx);
}

HomogeneousMagnitude homogeneous_magnitude_left_complement(HomogeneousMagnitude self) {
    return HomogeneousMagnitude(self.g0.yx);
}

HomogeneousMagnitude homogeneous_magnitude_double_complement(HomogeneousMagnitude self) {
    return HomogeneousMagnitude(self.g0);
}

Point point_zero() {
    return Point(vec4(0.0));
}

Point point_one() {
    return Point(vec4(0.0));
}

int point_grade(Point self) {
    return 1;
}

int point_anti_grade(Point self) {
    return 3;
}

Point point_neg(Point self) {
    return Point(self.g0 * vec4(-1.0));
}

Point point_automorphism(Point self) {
    return Point(self.g0 * vec4(-1.0));
}

Point point_reversal(Point self) {
    return Point(self.g0);
}

Point point_conjugation(Point self) {
    return Point(self.g0 * vec4(-1.0));
}

Plane point_dual(Point self) {
    return Plane(self.g0);
}

Point point_anti_reversal(Point self) {
    return Point(self.g0 * vec4(-1.0));
}

Plane point_right_complement(Point self) {
    return Plane(self.g0);
}

Plane point_left_complement(Point self) {
    return Plane(self.g0 * vec4(-1.0));
}

Point point_double_complement(Point self) {
    return Point(self.g0 * vec4(-1.0));
}

Line line_zero() {
    return Line(vec3(0.0), vec3(0.0));
}

Line line_one() {
    return Line(vec3(0.0), vec3(0.0));
}

int line_grade(Line self) {
    return 2;
}

int line_anti_grade(Line self) {
    return 2;
}

Line line_neg(Line self) {
    return Line(self.g0 * vec3(-1.0), self.g1 * vec3(-1.0));
}

Line line_automorphism(Line self) {
    return Line(self.g0, self.g1);
}

Line line_reversal(Line self) {
    return Line(self.g0 * vec3(-1.0), self.g1 * vec3(-1.0));
}

Line line_conjugation(Line self) {
    return Line(self.g0 * vec3(-1.0), self.g1 * vec3(-1.0));
}

Line line_dual(Line self) {
    return Line(self.g1 * vec3(-1.0), self.g0 * vec3(-1.0));
}

Line line_anti_reversal(Line self) {
    return Line(self.g0 * vec3(-1.0), self.g1 * vec3(-1.0));
}

Line line_right_complement(Line self) {
    return Line(self.g1 * vec3(-1.0), self.g0 * vec3(-1.0));
}

Line line_left_complement(Line self) {
    return Line(self.g1 * vec3(-1.0), self.g0 * vec3(-1.0));
}

Line line_double_complement(Line self) {
    return Line(self.g0, self.g1);
}

Plane plane_zero() {
    return Plane(vec4(0.0));
}

Plane plane_one() {
    return Plane(vec4(0.0));
}

int plane_grade(Plane self) {
    return 3;
}

int plane_anti_grade(Plane self) {
    return 1;
}

Plane plane_neg(Plane self) {
    return Plane(self.g0 * vec4(-1.0));
}

Plane plane_automorphism(Plane self) {
    return Plane(self.g0 * vec4(-1.0));
}

Plane plane_reversal(Plane self) {
    return Plane(self.g0 * vec4(-1.0));
}

Plane plane_conjugation(Plane self) {
    return Plane(self.g0);
}

Point plane_dual(Plane self) {
    return Point(self.g0 * vec4(-1.0));
}

Plane plane_anti_reversal(Plane self) {
    return Plane(self.g0);
}

Point plane_right_complement(Plane self) {
    return Point(self.g0 * vec4(-1.0));
}

Point plane_left_complement(Plane self) {
    return Point(self.g0);
}

Plane plane_double_complement(Plane self) {
    return Plane(self.g0 * vec4(-1.0));
}

Motor motor_zero() {
    return Motor(vec4(0.0), vec3(0.0));
}

Motor motor_one() {
    return Motor(vec4(0.0), vec3(0.0));
}

Motor motor_neg(Motor self) {
    return Motor(self.g0 * vec4(-1.0), self.g1 * vec3(-1.0));
}

Motor motor_automorphism(Motor self) {
    return Motor(self.g0, self.g1);
}

Motor motor_reversal(Motor self) {
    return Motor(self.g0 * vec4(-1.0, -1.0, -1.0, 1.0), self.g1 * vec3(-1.0));
}

Motor motor_conjugation(Motor self) {
    return Motor(self.g0 * vec4(-1.0, -1.0, -1.0, 1.0), self.g1 * vec3(-1.0));
}

Motor motor_anti_reversal(Motor self) {
    return Motor(self.g0 * vec4(-1.0, -1.0, -1.0, 1.0), self.g1 * vec3(-1.0));
}

Motor motor_double_complement(Motor self) {
    return Motor(self.g0, self.g1);
}

Rotor rotor_zero() {
    return Rotor(vec4(0.0));
}

Rotor rotor_one() {
    return Rotor(vec4(0.0));
}

Rotor rotor_neg(Rotor self) {
    return Rotor(self.g0 * vec4(-1.0));
}

Rotor rotor_automorphism(Rotor self) {
    return Rotor(self.g0);
}

Rotor rotor_reversal(Rotor self) {
    return Rotor(self.g0 * vec4(-1.0, -1.0, -1.0, 1.0));
}

Rotor rotor_conjugation(Rotor self) {
    return Rotor(self.g0 * vec4(-1.0, -1.0, -1.0, 1.0));
}

Rotor rotor_anti_reversal(Rotor self) {
    return Rotor(self.g0 * vec4(-1.0, -1.0, -1.0, 1.0));
}

Rotor rotor_double_complement(Rotor self) {
    return Rotor(self.g0);
}

Translator translator_zero() {
    return Translator(vec4(0.0));
}

Translator translator_one() {
    return Translator(vec4(0.0));
}

Translator translator_neg(Translator self) {
    return Translator(self.g0 * vec4(-1.0));
}

Translator translator_automorphism(Translator self) {
    return Translator(self.g0);
}

Translator translator_reversal(Translator self) {
    return Translator(self.g0 * vec4(-1.0, -1.0, -1.0, 1.0));
}

Translator translator_conjugation(Translator self) {
    return Translator(self.g0 * vec4(-1.0, -1.0, -1.0, 1.0));
}

Translator translator_anti_reversal(Translator self) {
    return Translator(self.g0 * vec4(-1.0, -1.0, -1.0, 1.0));
}

Translator translator_double_complement(Translator self) {
    return Translator(self.g0);
}

Flector flector_zero() {
    return Flector(vec4(0.0), vec4(0.0));
}

Flector flector_one() {
    return Flector(vec4(0.0), vec4(0.0));
}

Flector flector_neg(Flector self) {
    return Flector(self.g0 * vec4(-1.0), self.g1 * vec4(-1.0));
}

Flector flector_automorphism(Flector self) {
    return Flector(self.g0 * vec4(-1.0), self.g1 * vec4(-1.0));
}

Flector flector_reversal(Flector self) {
    return Flector(self.g0, self.g1 * vec4(-1.0));
}

Flector flector_conjugation(Flector self) {
    return Flector(self.g0 * vec4(-1.0), self.g1);
}

Flector flector_dual(Flector self) {
    return Flector(self.g1 * vec4(-1.0), self.g0);
}

Flector flector_anti_reversal(Flector self) {
    return Flector(self.g0 * vec4(-1.0), self.g1);
}

Flector flector_right_complement(Flector self) {
    return Flector(self.g1 * vec4(-1.0), self.g0);
}

Flector flector_left_complement(Flector self) {
    return Flector(self.g1, self.g0 * vec4(-1.0));
}

Flector flector_double_complement(Flector self) {
    return Flector(self.g0 * vec4(-1.0), self.g1 * vec4(-1.0));
}

MultiVector multi_vector_zero() {
    return MultiVector(vec2(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec4(0.0));
}

MultiVector multi_vector_one() {
    return MultiVector(vec2(1.0, 0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec4(0.0));
}

MultiVector multi_vector_neg(MultiVector self) {
    return MultiVector(self.g0 * vec2(-1.0), self.g1 * vec4(-1.0), self.g2 * vec3(-1.0), self.g3 * vec3(-1.0), self.g4 * vec4(-1.0));
}

MultiVector multi_vector_automorphism(MultiVector self) {
    return MultiVector(self.g0, self.g1 * vec4(-1.0), self.g2, self.g3, self.g4 * vec4(-1.0));
}

MultiVector multi_vector_reversal(MultiVector self) {
    return MultiVector(self.g0, self.g1, self.g2 * vec3(-1.0), self.g3 * vec3(-1.0), self.g4 * vec4(-1.0));
}

MultiVector multi_vector_conjugation(MultiVector self) {
    return MultiVector(self.g0, self.g1 * vec4(-1.0), self.g2 * vec3(-1.0), self.g3 * vec3(-1.0), self.g4);
}

MultiVector multi_vector_dual(MultiVector self) {
    return MultiVector(self.g0.yx, self.g4 * vec4(-1.0), self.g3 * vec3(-1.0), self.g2 * vec3(-1.0), self.g1);
}

MultiVector multi_vector_anti_reversal(MultiVector self) {
    return MultiVector(self.g0, self.g1 * vec4(-1.0), self.g2 * vec3(-1.0), self.g3 * vec3(-1.0), self.g4);
}

MultiVector multi_vector_right_complement(MultiVector self) {
    return MultiVector(self.g0.yx, self.g4 * vec4(-1.0), self.g3 * vec3(-1.0), self.g2 * vec3(-1.0), self.g1);
}

MultiVector multi_vector_left_complement(MultiVector self) {
    return MultiVector(self.g0.yx, self.g4, self.g3 * vec3(-1.0), self.g2 * vec3(-1.0), self.g1 * vec4(-1.0));
}

MultiVector multi_vector_double_complement(MultiVector self) {
    return MultiVector(self.g0, self.g1 * vec4(-1.0), self.g2, self.g3, self.g4 * vec4(-1.0));
}

Scalar scalar_scalar_add(Scalar self, Scalar other) {
    return Scalar(self.g0 + other.g0);
}

Scalar scalar_scalar_sub(Scalar self, Scalar other) {
    return Scalar(self.g0 - other.g0);
}

Scalar scalar_scalar_mul(Scalar self, Scalar other) {
    return Scalar(self.g0 * other.g0);
}

Scalar scalar_scalar_div(Scalar self, Scalar other) {
    return Scalar(self.g0 * 1.0 / other.g0 * 1.0);
}

Scalar scalar_scalar_geometric_product(Scalar self, Scalar other) {
    return Scalar(self.g0 * other.g0);
}

Scalar scalar_scalar_outer_product(Scalar self, Scalar other) {
    return Scalar(self.g0 * other.g0);
}

Scalar scalar_scalar_wedge(Scalar self, Scalar other) {
    return Scalar(self.g0 * other.g0);
}

Scalar scalar_scalar_join(Scalar self, Scalar other) {
    return Scalar(self.g0 * other.g0);
}

Scalar scalar_scalar_inner_product(Scalar self, Scalar other) {
    return Scalar(self.g0 * other.g0);
}

Scalar scalar_scalar_left_contraction(Scalar self, Scalar other) {
    return Scalar(self.g0 * other.g0);
}

Scalar scalar_scalar_right_contraction(Scalar self, Scalar other) {
    return Scalar(self.g0 * other.g0);
}

Scalar scalar_scalar_scalar_product(Scalar self, Scalar other) {
    return Scalar(self.g0 * other.g0);
}

Scalar scalar_scalar_dot(Scalar self, Scalar other) {
    return Scalar(self.g0 * other.g0);
}

HomogeneousMagnitude scalar_anti_scalar_add(Scalar self, AntiScalar other) {
    return HomogeneousMagnitude(vec2(self.g0) * vec2(1.0, 0.0) + vec2(other.g0) * vec2(0.0, 1.0));
}

HomogeneousMagnitude scalar_anti_scalar_sub(Scalar self, AntiScalar other) {
    return HomogeneousMagnitude(vec2(self.g0) * vec2(1.0, 0.0) - vec2(other.g0) * vec2(0.0, 1.0));
}

AntiScalar scalar_anti_scalar_geometric_product(Scalar self, AntiScalar other) {
    return AntiScalar(self.g0 * other.g0);
}

Scalar scalar_anti_scalar_geometric_anti_product(Scalar self, AntiScalar other) {
    return Scalar(self.g0 * other.g0);
}

Scalar scalar_anti_scalar_regressive_product(Scalar self, AntiScalar other) {
    return Scalar(self.g0 * other.g0);
}

Scalar scalar_anti_scalar_anti_wedge(Scalar self, AntiScalar other) {
    return Scalar(self.g0 * other.g0);
}

Scalar scalar_anti_scalar_meet(Scalar self, AntiScalar other) {
    return Scalar(self.g0 * other.g0);
}

AntiScalar scalar_anti_scalar_outer_product(Scalar self, AntiScalar other) {
    return AntiScalar(self.g0 * other.g0);
}

AntiScalar scalar_anti_scalar_wedge(Scalar self, AntiScalar other) {
    return AntiScalar(self.g0 * other.g0);
}

AntiScalar scalar_anti_scalar_join(Scalar self, AntiScalar other) {
    return AntiScalar(self.g0 * other.g0);
}

AntiScalar scalar_anti_scalar_inner_product(Scalar self, AntiScalar other) {
    return AntiScalar(self.g0 * other.g0);
}

Scalar scalar_anti_scalar_inner_anti_product(Scalar self, AntiScalar other) {
    return Scalar(self.g0 * other.g0);
}

AntiScalar scalar_anti_scalar_left_contraction(Scalar self, AntiScalar other) {
    return AntiScalar(self.g0 * other.g0);
}

Scalar scalar_anti_scalar_right_anti_contraction(Scalar self, AntiScalar other) {
    return Scalar(self.g0 * other.g0);
}

HomogeneousMagnitude scalar_homogeneous_magnitude_add(Scalar self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(vec2(self.g0) * vec2(1.0, 0.0) + other.g0);
}

HomogeneousMagnitude scalar_homogeneous_magnitude_sub(Scalar self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(vec2(self.g0) * vec2(1.0, 0.0) - other.g0);
}

HomogeneousMagnitude scalar_homogeneous_magnitude_geometric_product(Scalar self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(vec2(self.g0) * other.g0);
}

Scalar scalar_homogeneous_magnitude_geometric_anti_product(Scalar self, HomogeneousMagnitude other) {
    return Scalar(self.g0 * other.g0.y);
}

Scalar scalar_homogeneous_magnitude_regressive_product(Scalar self, HomogeneousMagnitude other) {
    return Scalar(self.g0 * other.g0.y);
}

Scalar scalar_homogeneous_magnitude_anti_wedge(Scalar self, HomogeneousMagnitude other) {
    return Scalar(self.g0 * other.g0.y);
}

Scalar scalar_homogeneous_magnitude_meet(Scalar self, HomogeneousMagnitude other) {
    return Scalar(self.g0 * other.g0.y);
}

HomogeneousMagnitude scalar_homogeneous_magnitude_outer_product(Scalar self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(vec2(self.g0) * other.g0);
}

HomogeneousMagnitude scalar_homogeneous_magnitude_wedge(Scalar self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(vec2(self.g0) * other.g0);
}

HomogeneousMagnitude scalar_homogeneous_magnitude_join(Scalar self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(vec2(self.g0) * other.g0);
}

HomogeneousMagnitude scalar_homogeneous_magnitude_inner_product(Scalar self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(vec2(self.g0) * other.g0);
}

Scalar scalar_homogeneous_magnitude_inner_anti_product(Scalar self, HomogeneousMagnitude other) {
    return Scalar(self.g0 * other.g0.y);
}

HomogeneousMagnitude scalar_homogeneous_magnitude_left_contraction(Scalar self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(vec2(self.g0) * other.g0);
}

Scalar scalar_homogeneous_magnitude_right_contraction(Scalar self, HomogeneousMagnitude other) {
    return Scalar(self.g0 * other.g0.x);
}

Scalar scalar_homogeneous_magnitude_right_anti_contraction(Scalar self, HomogeneousMagnitude other) {
    return Scalar(self.g0 * other.g0.y);
}

Scalar scalar_homogeneous_magnitude_scalar_product(Scalar self, HomogeneousMagnitude other) {
    return Scalar(self.g0 * other.g0.x);
}

Scalar scalar_homogeneous_magnitude_dot(Scalar self, HomogeneousMagnitude other) {
    return Scalar(self.g0 * other.g0.x);
}

Point scalar_point_geometric_product(Scalar self, Point other) {
    return Point(vec4(self.g0) * other.g0);
}

Plane scalar_point_geometric_anti_product(Scalar self, Point other) {
    return Plane(vec4(self.g0) * other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0));
}

Point scalar_point_outer_product(Scalar self, Point other) {
    return Point(vec4(self.g0) * other.g0);
}

Point scalar_point_wedge(Scalar self, Point other) {
    return Point(vec4(self.g0) * other.g0);
}

Point scalar_point_join(Scalar self, Point other) {
    return Point(vec4(self.g0) * other.g0);
}

Point scalar_point_inner_product(Scalar self, Point other) {
    return Point(vec4(self.g0) * other.g0);
}

Plane scalar_point_inner_anti_product(Scalar self, Point other) {
    return Plane(vec4(self.g0) * other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0));
}

Point scalar_point_left_contraction(Scalar self, Point other) {
    return Point(vec4(self.g0) * other.g0);
}

Plane scalar_point_right_anti_contraction(Scalar self, Point other) {
    return Plane(vec4(self.g0) * other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0));
}

Line scalar_line_geometric_product(Scalar self, Line other) {
    return Line(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Translator scalar_line_geometric_anti_product(Scalar self, Line other) {
    return Translator(vec4(self.g0) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Line scalar_line_outer_product(Scalar self, Line other) {
    return Line(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Line scalar_line_wedge(Scalar self, Line other) {
    return Line(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Line scalar_line_join(Scalar self, Line other) {
    return Line(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Line scalar_line_inner_product(Scalar self, Line other) {
    return Line(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Translator scalar_line_inner_anti_product(Scalar self, Line other) {
    return Translator(vec4(self.g0) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Line scalar_line_left_contraction(Scalar self, Line other) {
    return Line(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Translator scalar_line_right_anti_contraction(Scalar self, Line other) {
    return Translator(vec4(self.g0) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Plane scalar_plane_geometric_product(Scalar self, Plane other) {
    return Plane(vec4(self.g0) * other.g0);
}

Point scalar_plane_geometric_anti_product(Scalar self, Plane other) {
    return Point(vec4(self.g0) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0));
}

Plane scalar_plane_outer_product(Scalar self, Plane other) {
    return Plane(vec4(self.g0) * other.g0);
}

Plane scalar_plane_wedge(Scalar self, Plane other) {
    return Plane(vec4(self.g0) * other.g0);
}

Plane scalar_plane_join(Scalar self, Plane other) {
    return Plane(vec4(self.g0) * other.g0);
}

Plane scalar_plane_inner_product(Scalar self, Plane other) {
    return Plane(vec4(self.g0) * other.g0);
}

Point scalar_plane_inner_anti_product(Scalar self, Plane other) {
    return Point(vec4(self.g0) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0));
}

Plane scalar_plane_left_contraction(Scalar self, Plane other) {
    return Plane(vec4(self.g0) * other.g0);
}

Point scalar_plane_right_anti_contraction(Scalar self, Plane other) {
    return Point(vec4(self.g0) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0));
}

Motor scalar_motor_geometric_product(Scalar self, Motor other) {
    return Motor(vec4(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

MultiVector scalar_motor_geometric_anti_product(Scalar self, Motor other) {
    return MultiVector(vec2(self.g0) * vec2(other.g0.w, other.g0.x) * vec2(1.0, 0.0), vec4(0.0), vec3(0.0), vec3(self.g0) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(0.0));
}

Scalar scalar_motor_regressive_product(Scalar self, Motor other) {
    return Scalar(self.g0 * other.g0.w);
}

Scalar scalar_motor_anti_wedge(Scalar self, Motor other) {
    return Scalar(self.g0 * other.g0.w);
}

Scalar scalar_motor_meet(Scalar self, Motor other) {
    return Scalar(self.g0 * other.g0.w);
}

Motor scalar_motor_outer_product(Scalar self, Motor other) {
    return Motor(vec4(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Motor scalar_motor_wedge(Scalar self, Motor other) {
    return Motor(vec4(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Motor scalar_motor_join(Scalar self, Motor other) {
    return Motor(vec4(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Motor scalar_motor_inner_product(Scalar self, Motor other) {
    return Motor(vec4(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

MultiVector scalar_motor_inner_anti_product(Scalar self, Motor other) {
    return MultiVector(vec2(self.g0) * vec2(other.g0.w, other.g0.x) * vec2(1.0, 0.0), vec4(0.0), vec3(0.0), vec3(self.g0) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(0.0));
}

Motor scalar_motor_left_contraction(Scalar self, Motor other) {
    return Motor(vec4(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

MultiVector scalar_motor_right_anti_contraction(Scalar self, Motor other) {
    return MultiVector(vec2(self.g0) * vec2(other.g0.w, other.g0.x) * vec2(1.0, 0.0), vec4(0.0), vec3(0.0), vec3(self.g0) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(0.0));
}

Rotor scalar_rotor_geometric_product(Scalar self, Rotor other) {
    return Rotor(vec4(self.g0) * other.g0);
}

MultiVector scalar_rotor_geometric_anti_product(Scalar self, Rotor other) {
    return MultiVector(vec2(self.g0) * vec2(other.g0.w, other.g0.x) * vec2(1.0, 0.0), vec4(0.0), vec3(0.0), vec3(self.g0) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(0.0));
}

Scalar scalar_rotor_regressive_product(Scalar self, Rotor other) {
    return Scalar(self.g0 * other.g0.w);
}

Scalar scalar_rotor_anti_wedge(Scalar self, Rotor other) {
    return Scalar(self.g0 * other.g0.w);
}

Scalar scalar_rotor_meet(Scalar self, Rotor other) {
    return Scalar(self.g0 * other.g0.w);
}

Rotor scalar_rotor_outer_product(Scalar self, Rotor other) {
    return Rotor(vec4(self.g0) * other.g0);
}

Rotor scalar_rotor_wedge(Scalar self, Rotor other) {
    return Rotor(vec4(self.g0) * other.g0);
}

Rotor scalar_rotor_join(Scalar self, Rotor other) {
    return Rotor(vec4(self.g0) * other.g0);
}

Rotor scalar_rotor_inner_product(Scalar self, Rotor other) {
    return Rotor(vec4(self.g0) * other.g0);
}

MultiVector scalar_rotor_inner_anti_product(Scalar self, Rotor other) {
    return MultiVector(vec2(self.g0) * vec2(other.g0.w, other.g0.x) * vec2(1.0, 0.0), vec4(0.0), vec3(0.0), vec3(self.g0) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(0.0));
}

Rotor scalar_rotor_left_contraction(Scalar self, Rotor other) {
    return Rotor(vec4(self.g0) * other.g0);
}

MultiVector scalar_rotor_right_anti_contraction(Scalar self, Rotor other) {
    return MultiVector(vec2(self.g0) * vec2(other.g0.w, other.g0.x) * vec2(1.0, 0.0), vec4(0.0), vec3(0.0), vec3(self.g0) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(0.0));
}

Translator scalar_translator_geometric_product(Scalar self, Translator other) {
    return Translator(vec4(self.g0) * other.g0);
}

Scalar scalar_translator_geometric_anti_product(Scalar self, Translator other) {
    return Scalar(self.g0 * other.g0.w);
}

Scalar scalar_translator_regressive_product(Scalar self, Translator other) {
    return Scalar(self.g0 * other.g0.w);
}

Scalar scalar_translator_anti_wedge(Scalar self, Translator other) {
    return Scalar(self.g0 * other.g0.w);
}

Scalar scalar_translator_meet(Scalar self, Translator other) {
    return Scalar(self.g0 * other.g0.w);
}

Translator scalar_translator_outer_product(Scalar self, Translator other) {
    return Translator(vec4(self.g0) * other.g0);
}

Translator scalar_translator_wedge(Scalar self, Translator other) {
    return Translator(vec4(self.g0) * other.g0);
}

Translator scalar_translator_join(Scalar self, Translator other) {
    return Translator(vec4(self.g0) * other.g0);
}

Translator scalar_translator_inner_product(Scalar self, Translator other) {
    return Translator(vec4(self.g0) * other.g0);
}

Scalar scalar_translator_inner_anti_product(Scalar self, Translator other) {
    return Scalar(self.g0 * other.g0.w);
}

Translator scalar_translator_left_contraction(Scalar self, Translator other) {
    return Translator(vec4(self.g0) * other.g0);
}

Scalar scalar_translator_right_anti_contraction(Scalar self, Translator other) {
    return Scalar(self.g0 * other.g0.w);
}

Flector scalar_flector_geometric_product(Scalar self, Flector other) {
    return Flector(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

Flector scalar_flector_geometric_anti_product(Scalar self, Flector other) {
    return Flector(vec4(self.g0) * other.g1.xyzx * vec4(1.0, 1.0, 1.0, 0.0), vec4(self.g0) * other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0));
}

Flector scalar_flector_outer_product(Scalar self, Flector other) {
    return Flector(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

Flector scalar_flector_wedge(Scalar self, Flector other) {
    return Flector(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

Flector scalar_flector_join(Scalar self, Flector other) {
    return Flector(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

Flector scalar_flector_inner_product(Scalar self, Flector other) {
    return Flector(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

Flector scalar_flector_inner_anti_product(Scalar self, Flector other) {
    return Flector(vec4(self.g0) * other.g1.xyzx * vec4(1.0, 1.0, 1.0, 0.0), vec4(self.g0) * other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0));
}

Flector scalar_flector_left_contraction(Scalar self, Flector other) {
    return Flector(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

Flector scalar_flector_right_anti_contraction(Scalar self, Flector other) {
    return Flector(vec4(self.g0) * other.g1.xyzx * vec4(1.0, 1.0, 1.0, 0.0), vec4(self.g0) * other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0));
}

MultiVector scalar_multi_vector_add(Scalar self, MultiVector other) {
    return MultiVector(vec2(self.g0) * vec2(1.0, 0.0) + other.g0, other.g1, other.g2, other.g3, other.g4);
}

MultiVector scalar_multi_vector_sub(Scalar self, MultiVector other) {
    return MultiVector(vec2(self.g0) * vec2(1.0, 0.0) - other.g0, vec4(0.0) - other.g1, vec3(0.0) - other.g2, vec3(0.0) - other.g3, vec4(0.0) - other.g4);
}

MultiVector scalar_multi_vector_geometric_product(Scalar self, MultiVector other) {
    return MultiVector(vec2(self.g0) * other.g0, vec4(self.g0) * other.g1, vec3(self.g0) * other.g2, vec3(self.g0) * other.g3, vec4(self.g0) * other.g4);
}

MultiVector scalar_multi_vector_geometric_anti_product(Scalar self, MultiVector other) {
    return MultiVector(vec2(self.g0) * other.g0.yx * vec2(1.0, 0.0), vec4(self.g0) * other.g4.xyzx * vec4(1.0, 1.0, 1.0, 0.0), vec3(0.0), vec3(self.g0) * other.g2, vec4(self.g0) * other.g1.xxxw * vec4(0.0, 0.0, 0.0, 1.0));
}

Scalar scalar_multi_vector_regressive_product(Scalar self, MultiVector other) {
    return Scalar(self.g0 * other.g0.y);
}

Scalar scalar_multi_vector_anti_wedge(Scalar self, MultiVector other) {
    return Scalar(self.g0 * other.g0.y);
}

Scalar scalar_multi_vector_meet(Scalar self, MultiVector other) {
    return Scalar(self.g0 * other.g0.y);
}

MultiVector scalar_multi_vector_outer_product(Scalar self, MultiVector other) {
    return MultiVector(vec2(self.g0) * other.g0, vec4(self.g0) * other.g1, vec3(self.g0) * other.g2, vec3(self.g0) * other.g3, vec4(self.g0) * other.g4);
}

MultiVector scalar_multi_vector_wedge(Scalar self, MultiVector other) {
    return MultiVector(vec2(self.g0) * other.g0, vec4(self.g0) * other.g1, vec3(self.g0) * other.g2, vec3(self.g0) * other.g3, vec4(self.g0) * other.g4);
}

MultiVector scalar_multi_vector_join(Scalar self, MultiVector other) {
    return MultiVector(vec2(self.g0) * other.g0, vec4(self.g0) * other.g1, vec3(self.g0) * other.g2, vec3(self.g0) * other.g3, vec4(self.g0) * other.g4);
}

MultiVector scalar_multi_vector_inner_product(Scalar self, MultiVector other) {
    return MultiVector(vec2(self.g0) * other.g0, vec4(self.g0) * other.g1, vec3(self.g0) * other.g2, vec3(self.g0) * other.g3, vec4(self.g0) * other.g4);
}

MultiVector scalar_multi_vector_inner_anti_product(Scalar self, MultiVector other) {
    return MultiVector(vec2(self.g0) * other.g0.yx * vec2(1.0, 0.0), vec4(self.g0) * other.g4.xyzx * vec4(1.0, 1.0, 1.0, 0.0), vec3(0.0), vec3(self.g0) * other.g2, vec4(self.g0) * other.g1.xxxw * vec4(0.0, 0.0, 0.0, 1.0));
}

MultiVector scalar_multi_vector_left_contraction(Scalar self, MultiVector other) {
    return MultiVector(vec2(self.g0) * other.g0, vec4(self.g0) * other.g1, vec3(self.g0) * other.g2, vec3(self.g0) * other.g3, vec4(self.g0) * other.g4);
}

Scalar scalar_multi_vector_right_contraction(Scalar self, MultiVector other) {
    return Scalar(self.g0 * other.g0.x);
}

MultiVector scalar_multi_vector_right_anti_contraction(Scalar self, MultiVector other) {
    return MultiVector(vec2(self.g0) * other.g0.yx * vec2(1.0, 0.0), vec4(self.g0) * other.g4.xyzx * vec4(1.0, 1.0, 1.0, 0.0), vec3(0.0), vec3(self.g0) * other.g2, vec4(self.g0) * other.g1.xxxw * vec4(0.0, 0.0, 0.0, 1.0));
}

Scalar scalar_multi_vector_scalar_product(Scalar self, MultiVector other) {
    return Scalar(self.g0 * other.g0.x);
}

Scalar scalar_multi_vector_dot(Scalar self, MultiVector other) {
    return Scalar(self.g0 * other.g0.x);
}

HomogeneousMagnitude anti_scalar_scalar_add(AntiScalar self, Scalar other) {
    return HomogeneousMagnitude(vec2(self.g0) * vec2(0.0, 1.0) + vec2(other.g0) * vec2(1.0, 0.0));
}

HomogeneousMagnitude anti_scalar_scalar_sub(AntiScalar self, Scalar other) {
    return HomogeneousMagnitude(vec2(self.g0) * vec2(0.0, 1.0) - vec2(other.g0) * vec2(1.0, 0.0));
}

AntiScalar anti_scalar_scalar_geometric_product(AntiScalar self, Scalar other) {
    return AntiScalar(self.g0 * other.g0);
}

Scalar anti_scalar_scalar_geometric_anti_product(AntiScalar self, Scalar other) {
    return Scalar(self.g0 * other.g0);
}

Scalar anti_scalar_scalar_regressive_product(AntiScalar self, Scalar other) {
    return Scalar(self.g0 * other.g0);
}

Scalar anti_scalar_scalar_anti_wedge(AntiScalar self, Scalar other) {
    return Scalar(self.g0 * other.g0);
}

Scalar anti_scalar_scalar_meet(AntiScalar self, Scalar other) {
    return Scalar(self.g0 * other.g0);
}

AntiScalar anti_scalar_scalar_outer_product(AntiScalar self, Scalar other) {
    return AntiScalar(self.g0 * other.g0);
}

AntiScalar anti_scalar_scalar_wedge(AntiScalar self, Scalar other) {
    return AntiScalar(self.g0 * other.g0);
}

AntiScalar anti_scalar_scalar_join(AntiScalar self, Scalar other) {
    return AntiScalar(self.g0 * other.g0);
}

AntiScalar anti_scalar_scalar_inner_product(AntiScalar self, Scalar other) {
    return AntiScalar(self.g0 * other.g0);
}

Scalar anti_scalar_scalar_inner_anti_product(AntiScalar self, Scalar other) {
    return Scalar(self.g0 * other.g0);
}

AntiScalar anti_scalar_scalar_right_contraction(AntiScalar self, Scalar other) {
    return AntiScalar(self.g0 * other.g0);
}

Scalar anti_scalar_scalar_left_anti_contraction(AntiScalar self, Scalar other) {
    return Scalar(self.g0 * other.g0);
}

AntiScalar anti_scalar_anti_scalar_add(AntiScalar self, AntiScalar other) {
    return AntiScalar(self.g0 + other.g0);
}

AntiScalar anti_scalar_anti_scalar_sub(AntiScalar self, AntiScalar other) {
    return AntiScalar(self.g0 - other.g0);
}

AntiScalar anti_scalar_anti_scalar_mul(AntiScalar self, AntiScalar other) {
    return AntiScalar(self.g0 * other.g0);
}

AntiScalar anti_scalar_anti_scalar_div(AntiScalar self, AntiScalar other) {
    return AntiScalar(self.g0 * 1.0 / other.g0 * 1.0);
}

AntiScalar anti_scalar_anti_scalar_geometric_anti_product(AntiScalar self, AntiScalar other) {
    return AntiScalar(self.g0 * other.g0);
}

AntiScalar anti_scalar_anti_scalar_regressive_product(AntiScalar self, AntiScalar other) {
    return AntiScalar(self.g0 * other.g0);
}

AntiScalar anti_scalar_anti_scalar_anti_wedge(AntiScalar self, AntiScalar other) {
    return AntiScalar(self.g0 * other.g0);
}

AntiScalar anti_scalar_anti_scalar_meet(AntiScalar self, AntiScalar other) {
    return AntiScalar(self.g0 * other.g0);
}

AntiScalar anti_scalar_anti_scalar_inner_anti_product(AntiScalar self, AntiScalar other) {
    return AntiScalar(self.g0 * other.g0);
}

AntiScalar anti_scalar_anti_scalar_left_anti_contraction(AntiScalar self, AntiScalar other) {
    return AntiScalar(self.g0 * other.g0);
}

AntiScalar anti_scalar_anti_scalar_right_anti_contraction(AntiScalar self, AntiScalar other) {
    return AntiScalar(self.g0 * other.g0);
}

AntiScalar anti_scalar_anti_scalar_anti_scalar_product(AntiScalar self, AntiScalar other) {
    return AntiScalar(self.g0 * other.g0);
}

AntiScalar anti_scalar_anti_scalar_anti_dot(AntiScalar self, AntiScalar other) {
    return AntiScalar(self.g0 * other.g0);
}

HomogeneousMagnitude anti_scalar_homogeneous_magnitude_add(AntiScalar self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(vec2(self.g0) * vec2(0.0, 1.0) + other.g0);
}

HomogeneousMagnitude anti_scalar_homogeneous_magnitude_sub(AntiScalar self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(vec2(self.g0) * vec2(0.0, 1.0) - other.g0);
}

AntiScalar anti_scalar_homogeneous_magnitude_geometric_product(AntiScalar self, HomogeneousMagnitude other) {
    return AntiScalar(self.g0 * other.g0.x);
}

HomogeneousMagnitude anti_scalar_homogeneous_magnitude_geometric_anti_product(AntiScalar self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(vec2(self.g0) * other.g0);
}

HomogeneousMagnitude anti_scalar_homogeneous_magnitude_regressive_product(AntiScalar self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(vec2(self.g0) * other.g0);
}

HomogeneousMagnitude anti_scalar_homogeneous_magnitude_anti_wedge(AntiScalar self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(vec2(self.g0) * other.g0);
}

HomogeneousMagnitude anti_scalar_homogeneous_magnitude_meet(AntiScalar self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(vec2(self.g0) * other.g0);
}

AntiScalar anti_scalar_homogeneous_magnitude_outer_product(AntiScalar self, HomogeneousMagnitude other) {
    return AntiScalar(self.g0 * other.g0.x);
}

AntiScalar anti_scalar_homogeneous_magnitude_wedge(AntiScalar self, HomogeneousMagnitude other) {
    return AntiScalar(self.g0 * other.g0.x);
}

AntiScalar anti_scalar_homogeneous_magnitude_join(AntiScalar self, HomogeneousMagnitude other) {
    return AntiScalar(self.g0 * other.g0.x);
}

AntiScalar anti_scalar_homogeneous_magnitude_inner_product(AntiScalar self, HomogeneousMagnitude other) {
    return AntiScalar(self.g0 * other.g0.x);
}

HomogeneousMagnitude anti_scalar_homogeneous_magnitude_inner_anti_product(AntiScalar self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(vec2(self.g0) * other.g0);
}

AntiScalar anti_scalar_homogeneous_magnitude_right_contraction(AntiScalar self, HomogeneousMagnitude other) {
    return AntiScalar(self.g0 * other.g0.x);
}

HomogeneousMagnitude anti_scalar_homogeneous_magnitude_left_anti_contraction(AntiScalar self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(vec2(self.g0) * other.g0);
}

AntiScalar anti_scalar_homogeneous_magnitude_right_anti_contraction(AntiScalar self, HomogeneousMagnitude other) {
    return AntiScalar(self.g0 * other.g0.y);
}

AntiScalar anti_scalar_homogeneous_magnitude_anti_scalar_product(AntiScalar self, HomogeneousMagnitude other) {
    return AntiScalar(self.g0 * other.g0.y);
}

AntiScalar anti_scalar_homogeneous_magnitude_anti_dot(AntiScalar self, HomogeneousMagnitude other) {
    return AntiScalar(self.g0 * other.g0.y);
}

Plane anti_scalar_point_geometric_product(AntiScalar self, Point other) {
    return Plane(vec4(self.g0) * other.g0.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

Point anti_scalar_point_geometric_anti_product(AntiScalar self, Point other) {
    return Point(vec4(self.g0) * other.g0);
}

Point anti_scalar_point_regressive_product(AntiScalar self, Point other) {
    return Point(vec4(self.g0) * other.g0);
}

Point anti_scalar_point_anti_wedge(AntiScalar self, Point other) {
    return Point(vec4(self.g0) * other.g0);
}

Point anti_scalar_point_meet(AntiScalar self, Point other) {
    return Point(vec4(self.g0) * other.g0);
}

Plane anti_scalar_point_inner_product(AntiScalar self, Point other) {
    return Plane(vec4(self.g0) * other.g0.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

Point anti_scalar_point_inner_anti_product(AntiScalar self, Point other) {
    return Point(vec4(self.g0) * other.g0);
}

Plane anti_scalar_point_right_contraction(AntiScalar self, Point other) {
    return Plane(vec4(self.g0) * other.g0.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

Point anti_scalar_point_left_anti_contraction(AntiScalar self, Point other) {
    return Point(vec4(self.g0) * other.g0);
}

Motor anti_scalar_line_add(AntiScalar self, Line other) {
    return Motor(vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0), other.g1);
}

Motor anti_scalar_line_sub(AntiScalar self, Line other) {
    return Motor(vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) - vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0), vec3(0.0) - other.g1);
}

Rotor anti_scalar_line_geometric_product(AntiScalar self, Line other) {
    return Rotor(vec4(self.g0) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Line anti_scalar_line_geometric_anti_product(AntiScalar self, Line other) {
    return Line(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Line anti_scalar_line_regressive_product(AntiScalar self, Line other) {
    return Line(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Line anti_scalar_line_anti_wedge(AntiScalar self, Line other) {
    return Line(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Line anti_scalar_line_meet(AntiScalar self, Line other) {
    return Line(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Rotor anti_scalar_line_inner_product(AntiScalar self, Line other) {
    return Rotor(vec4(self.g0) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Line anti_scalar_line_inner_anti_product(AntiScalar self, Line other) {
    return Line(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Rotor anti_scalar_line_right_contraction(AntiScalar self, Line other) {
    return Rotor(vec4(self.g0) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Line anti_scalar_line_left_anti_contraction(AntiScalar self, Line other) {
    return Line(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Point anti_scalar_plane_geometric_product(AntiScalar self, Plane other) {
    return Point(vec4(self.g0) * other.g0.xxxw * vec4(0.0, 0.0, 0.0, -1.0));
}

Plane anti_scalar_plane_geometric_anti_product(AntiScalar self, Plane other) {
    return Plane(vec4(self.g0) * other.g0);
}

Plane anti_scalar_plane_regressive_product(AntiScalar self, Plane other) {
    return Plane(vec4(self.g0) * other.g0);
}

Plane anti_scalar_plane_anti_wedge(AntiScalar self, Plane other) {
    return Plane(vec4(self.g0) * other.g0);
}

Plane anti_scalar_plane_meet(AntiScalar self, Plane other) {
    return Plane(vec4(self.g0) * other.g0);
}

Point anti_scalar_plane_inner_product(AntiScalar self, Plane other) {
    return Point(vec4(self.g0) * other.g0.xxxw * vec4(0.0, 0.0, 0.0, -1.0));
}

Plane anti_scalar_plane_inner_anti_product(AntiScalar self, Plane other) {
    return Plane(vec4(self.g0) * other.g0);
}

Point anti_scalar_plane_right_contraction(AntiScalar self, Plane other) {
    return Point(vec4(self.g0) * other.g0.xxxw * vec4(0.0, 0.0, 0.0, -1.0));
}

Plane anti_scalar_plane_left_anti_contraction(AntiScalar self, Plane other) {
    return Plane(vec4(self.g0) * other.g0);
}

Motor anti_scalar_motor_add(AntiScalar self, Motor other) {
    return Motor(vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) + other.g0, other.g1);
}

Motor anti_scalar_motor_sub(AntiScalar self, Motor other) {
    return Motor(vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) - other.g0, vec3(0.0) - other.g1);
}

Rotor anti_scalar_motor_geometric_product(AntiScalar self, Motor other) {
    return Rotor(vec4(self.g0) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Motor anti_scalar_motor_geometric_anti_product(AntiScalar self, Motor other) {
    return Motor(vec4(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Motor anti_scalar_motor_regressive_product(AntiScalar self, Motor other) {
    return Motor(vec4(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Motor anti_scalar_motor_anti_wedge(AntiScalar self, Motor other) {
    return Motor(vec4(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Motor anti_scalar_motor_meet(AntiScalar self, Motor other) {
    return Motor(vec4(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Rotor anti_scalar_motor_inner_product(AntiScalar self, Motor other) {
    return Rotor(vec4(self.g0) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Motor anti_scalar_motor_inner_anti_product(AntiScalar self, Motor other) {
    return Motor(vec4(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Rotor anti_scalar_motor_right_contraction(AntiScalar self, Motor other) {
    return Rotor(vec4(self.g0) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Motor anti_scalar_motor_left_anti_contraction(AntiScalar self, Motor other) {
    return Motor(vec4(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

AntiScalar anti_scalar_motor_right_anti_contraction(AntiScalar self, Motor other) {
    return AntiScalar(self.g0 * other.g0.w);
}

AntiScalar anti_scalar_motor_anti_scalar_product(AntiScalar self, Motor other) {
    return AntiScalar(self.g0 * other.g0.w);
}

AntiScalar anti_scalar_motor_anti_dot(AntiScalar self, Motor other) {
    return AntiScalar(self.g0 * other.g0.w);
}

Rotor anti_scalar_rotor_add(AntiScalar self, Rotor other) {
    return Rotor(vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) + other.g0);
}

Rotor anti_scalar_rotor_sub(AntiScalar self, Rotor other) {
    return Rotor(vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) - other.g0);
}

Rotor anti_scalar_rotor_geometric_anti_product(AntiScalar self, Rotor other) {
    return Rotor(vec4(self.g0) * other.g0);
}

Rotor anti_scalar_rotor_regressive_product(AntiScalar self, Rotor other) {
    return Rotor(vec4(self.g0) * other.g0);
}

Rotor anti_scalar_rotor_anti_wedge(AntiScalar self, Rotor other) {
    return Rotor(vec4(self.g0) * other.g0);
}

Rotor anti_scalar_rotor_meet(AntiScalar self, Rotor other) {
    return Rotor(vec4(self.g0) * other.g0);
}

Rotor anti_scalar_rotor_inner_anti_product(AntiScalar self, Rotor other) {
    return Rotor(vec4(self.g0) * other.g0);
}

Rotor anti_scalar_rotor_left_anti_contraction(AntiScalar self, Rotor other) {
    return Rotor(vec4(self.g0) * other.g0);
}

AntiScalar anti_scalar_rotor_right_anti_contraction(AntiScalar self, Rotor other) {
    return AntiScalar(self.g0 * other.g0.w);
}

AntiScalar anti_scalar_rotor_anti_scalar_product(AntiScalar self, Rotor other) {
    return AntiScalar(self.g0 * other.g0.w);
}

AntiScalar anti_scalar_rotor_anti_dot(AntiScalar self, Rotor other) {
    return AntiScalar(self.g0 * other.g0.w);
}

Translator anti_scalar_translator_add(AntiScalar self, Translator other) {
    return Translator(vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) + other.g0);
}

Translator anti_scalar_translator_sub(AntiScalar self, Translator other) {
    return Translator(vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) - other.g0);
}

Rotor anti_scalar_translator_geometric_product(AntiScalar self, Translator other) {
    return Rotor(vec4(self.g0) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0));
}

Translator anti_scalar_translator_geometric_anti_product(AntiScalar self, Translator other) {
    return Translator(vec4(self.g0) * other.g0);
}

Translator anti_scalar_translator_regressive_product(AntiScalar self, Translator other) {
    return Translator(vec4(self.g0) * other.g0);
}

Translator anti_scalar_translator_anti_wedge(AntiScalar self, Translator other) {
    return Translator(vec4(self.g0) * other.g0);
}

Translator anti_scalar_translator_meet(AntiScalar self, Translator other) {
    return Translator(vec4(self.g0) * other.g0);
}

Rotor anti_scalar_translator_inner_product(AntiScalar self, Translator other) {
    return Rotor(vec4(self.g0) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0));
}

Translator anti_scalar_translator_inner_anti_product(AntiScalar self, Translator other) {
    return Translator(vec4(self.g0) * other.g0);
}

Rotor anti_scalar_translator_right_contraction(AntiScalar self, Translator other) {
    return Rotor(vec4(self.g0) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0));
}

Translator anti_scalar_translator_left_anti_contraction(AntiScalar self, Translator other) {
    return Translator(vec4(self.g0) * other.g0);
}

AntiScalar anti_scalar_translator_right_anti_contraction(AntiScalar self, Translator other) {
    return AntiScalar(self.g0 * other.g0.w);
}

AntiScalar anti_scalar_translator_anti_scalar_product(AntiScalar self, Translator other) {
    return AntiScalar(self.g0 * other.g0.w);
}

AntiScalar anti_scalar_translator_anti_dot(AntiScalar self, Translator other) {
    return AntiScalar(self.g0 * other.g0.w);
}

Flector anti_scalar_flector_geometric_product(AntiScalar self, Flector other) {
    return Flector(vec4(self.g0) * other.g1.xxxw * vec4(0.0, 0.0, 0.0, -1.0), vec4(self.g0) * other.g0.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

Flector anti_scalar_flector_geometric_anti_product(AntiScalar self, Flector other) {
    return Flector(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

Flector anti_scalar_flector_regressive_product(AntiScalar self, Flector other) {
    return Flector(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

Flector anti_scalar_flector_anti_wedge(AntiScalar self, Flector other) {
    return Flector(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

Flector anti_scalar_flector_meet(AntiScalar self, Flector other) {
    return Flector(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

Flector anti_scalar_flector_inner_product(AntiScalar self, Flector other) {
    return Flector(vec4(self.g0) * other.g1.xxxw * vec4(0.0, 0.0, 0.0, -1.0), vec4(self.g0) * other.g0.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

Flector anti_scalar_flector_inner_anti_product(AntiScalar self, Flector other) {
    return Flector(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

Flector anti_scalar_flector_right_contraction(AntiScalar self, Flector other) {
    return Flector(vec4(self.g0) * other.g1.xxxw * vec4(0.0, 0.0, 0.0, -1.0), vec4(self.g0) * other.g0.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

Flector anti_scalar_flector_left_anti_contraction(AntiScalar self, Flector other) {
    return Flector(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

MultiVector anti_scalar_multi_vector_add(AntiScalar self, MultiVector other) {
    return MultiVector(vec2(self.g0) * vec2(0.0, 1.0) + other.g0, other.g1, other.g2, other.g3, other.g4);
}

MultiVector anti_scalar_multi_vector_sub(AntiScalar self, MultiVector other) {
    return MultiVector(vec2(self.g0) * vec2(0.0, 1.0) - other.g0, vec4(0.0) - other.g1, vec3(0.0) - other.g2, vec3(0.0) - other.g3, vec4(0.0) - other.g4);
}

MultiVector anti_scalar_multi_vector_geometric_product(AntiScalar self, MultiVector other) {
    return MultiVector(vec2(self.g0) * vec2(other.g0.x) * vec2(0.0, 1.0), vec4(self.g0) * other.g4.xxxw * vec4(0.0, 0.0, 0.0, -1.0), vec3(self.g0) * other.g3, vec3(0.0), vec4(self.g0) * other.g1.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

MultiVector anti_scalar_multi_vector_geometric_anti_product(AntiScalar self, MultiVector other) {
    return MultiVector(vec2(self.g0) * other.g0, vec4(self.g0) * other.g1, vec3(self.g0) * other.g2, vec3(self.g0) * other.g3, vec4(self.g0) * other.g4);
}

MultiVector anti_scalar_multi_vector_regressive_product(AntiScalar self, MultiVector other) {
    return MultiVector(vec2(self.g0) * other.g0, vec4(self.g0) * other.g1, vec3(self.g0) * other.g2, vec3(self.g0) * other.g3, vec4(self.g0) * other.g4);
}

MultiVector anti_scalar_multi_vector_anti_wedge(AntiScalar self, MultiVector other) {
    return MultiVector(vec2(self.g0) * other.g0, vec4(self.g0) * other.g1, vec3(self.g0) * other.g2, vec3(self.g0) * other.g3, vec4(self.g0) * other.g4);
}

MultiVector anti_scalar_multi_vector_meet(AntiScalar self, MultiVector other) {
    return MultiVector(vec2(self.g0) * other.g0, vec4(self.g0) * other.g1, vec3(self.g0) * other.g2, vec3(self.g0) * other.g3, vec4(self.g0) * other.g4);
}

AntiScalar anti_scalar_multi_vector_outer_product(AntiScalar self, MultiVector other) {
    return AntiScalar(self.g0 * other.g0.x);
}

AntiScalar anti_scalar_multi_vector_wedge(AntiScalar self, MultiVector other) {
    return AntiScalar(self.g0 * other.g0.x);
}

AntiScalar anti_scalar_multi_vector_join(AntiScalar self, MultiVector other) {
    return AntiScalar(self.g0 * other.g0.x);
}

MultiVector anti_scalar_multi_vector_inner_product(AntiScalar self, MultiVector other) {
    return MultiVector(vec2(self.g0) * vec2(other.g0.x) * vec2(0.0, 1.0), vec4(self.g0) * other.g4.xxxw * vec4(0.0, 0.0, 0.0, -1.0), vec3(self.g0) * other.g3, vec3(0.0), vec4(self.g0) * other.g1.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

MultiVector anti_scalar_multi_vector_inner_anti_product(AntiScalar self, MultiVector other) {
    return MultiVector(vec2(self.g0) * other.g0, vec4(self.g0) * other.g1, vec3(self.g0) * other.g2, vec3(self.g0) * other.g3, vec4(self.g0) * other.g4);
}

MultiVector anti_scalar_multi_vector_right_contraction(AntiScalar self, MultiVector other) {
    return MultiVector(vec2(self.g0) * vec2(other.g0.x) * vec2(0.0, 1.0), vec4(self.g0) * other.g4.xxxw * vec4(0.0, 0.0, 0.0, -1.0), vec3(self.g0) * other.g3, vec3(0.0), vec4(self.g0) * other.g1.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

MultiVector anti_scalar_multi_vector_left_anti_contraction(AntiScalar self, MultiVector other) {
    return MultiVector(vec2(self.g0) * other.g0, vec4(self.g0) * other.g1, vec3(self.g0) * other.g2, vec3(self.g0) * other.g3, vec4(self.g0) * other.g4);
}

AntiScalar anti_scalar_multi_vector_right_anti_contraction(AntiScalar self, MultiVector other) {
    return AntiScalar(self.g0 * other.g0.y);
}

AntiScalar anti_scalar_multi_vector_anti_scalar_product(AntiScalar self, MultiVector other) {
    return AntiScalar(self.g0 * other.g0.y);
}

AntiScalar anti_scalar_multi_vector_anti_dot(AntiScalar self, MultiVector other) {
    return AntiScalar(self.g0 * other.g0.y);
}

Scalar homogeneous_magnitude_scalar_into(HomogeneousMagnitude self) {
    return Scalar(self.g0.x);
}

HomogeneousMagnitude homogeneous_magnitude_scalar_add(HomogeneousMagnitude self, Scalar other) {
    return HomogeneousMagnitude(self.g0 + vec2(other.g0) * vec2(1.0, 0.0));
}

HomogeneousMagnitude homogeneous_magnitude_scalar_sub(HomogeneousMagnitude self, Scalar other) {
    return HomogeneousMagnitude(self.g0 - vec2(other.g0) * vec2(1.0, 0.0));
}

HomogeneousMagnitude homogeneous_magnitude_scalar_geometric_product(HomogeneousMagnitude self, Scalar other) {
    return HomogeneousMagnitude(self.g0 * vec2(other.g0));
}

Scalar homogeneous_magnitude_scalar_geometric_anti_product(HomogeneousMagnitude self, Scalar other) {
    return Scalar(self.g0.y * other.g0);
}

Scalar homogeneous_magnitude_scalar_regressive_product(HomogeneousMagnitude self, Scalar other) {
    return Scalar(self.g0.y * other.g0);
}

Scalar homogeneous_magnitude_scalar_anti_wedge(HomogeneousMagnitude self, Scalar other) {
    return Scalar(self.g0.y * other.g0);
}

Scalar homogeneous_magnitude_scalar_meet(HomogeneousMagnitude self, Scalar other) {
    return Scalar(self.g0.y * other.g0);
}

HomogeneousMagnitude homogeneous_magnitude_scalar_outer_product(HomogeneousMagnitude self, Scalar other) {
    return HomogeneousMagnitude(self.g0 * vec2(other.g0));
}

HomogeneousMagnitude homogeneous_magnitude_scalar_wedge(HomogeneousMagnitude self, Scalar other) {
    return HomogeneousMagnitude(self.g0 * vec2(other.g0));
}

HomogeneousMagnitude homogeneous_magnitude_scalar_join(HomogeneousMagnitude self, Scalar other) {
    return HomogeneousMagnitude(self.g0 * vec2(other.g0));
}

HomogeneousMagnitude homogeneous_magnitude_scalar_inner_product(HomogeneousMagnitude self, Scalar other) {
    return HomogeneousMagnitude(self.g0 * vec2(other.g0));
}

Scalar homogeneous_magnitude_scalar_inner_anti_product(HomogeneousMagnitude self, Scalar other) {
    return Scalar(self.g0.y * other.g0);
}

Scalar homogeneous_magnitude_scalar_left_contraction(HomogeneousMagnitude self, Scalar other) {
    return Scalar(self.g0.x * other.g0);
}

HomogeneousMagnitude homogeneous_magnitude_scalar_right_contraction(HomogeneousMagnitude self, Scalar other) {
    return HomogeneousMagnitude(self.g0 * vec2(other.g0));
}

Scalar homogeneous_magnitude_scalar_left_anti_contraction(HomogeneousMagnitude self, Scalar other) {
    return Scalar(self.g0.y * other.g0);
}

Scalar homogeneous_magnitude_scalar_scalar_product(HomogeneousMagnitude self, Scalar other) {
    return Scalar(self.g0.x * other.g0);
}

Scalar homogeneous_magnitude_scalar_dot(HomogeneousMagnitude self, Scalar other) {
    return Scalar(self.g0.x * other.g0);
}

AntiScalar homogeneous_magnitude_anti_scalar_into(HomogeneousMagnitude self) {
    return AntiScalar(self.g0.y);
}

HomogeneousMagnitude homogeneous_magnitude_anti_scalar_add(HomogeneousMagnitude self, AntiScalar other) {
    return HomogeneousMagnitude(self.g0 + vec2(other.g0) * vec2(0.0, 1.0));
}

HomogeneousMagnitude homogeneous_magnitude_anti_scalar_sub(HomogeneousMagnitude self, AntiScalar other) {
    return HomogeneousMagnitude(self.g0 - vec2(other.g0) * vec2(0.0, 1.0));
}

AntiScalar homogeneous_magnitude_anti_scalar_geometric_product(HomogeneousMagnitude self, AntiScalar other) {
    return AntiScalar(self.g0.x * other.g0);
}

HomogeneousMagnitude homogeneous_magnitude_anti_scalar_geometric_anti_product(HomogeneousMagnitude self, AntiScalar other) {
    return HomogeneousMagnitude(self.g0 * vec2(other.g0));
}

HomogeneousMagnitude homogeneous_magnitude_anti_scalar_regressive_product(HomogeneousMagnitude self, AntiScalar other) {
    return HomogeneousMagnitude(self.g0 * vec2(other.g0));
}

HomogeneousMagnitude homogeneous_magnitude_anti_scalar_anti_wedge(HomogeneousMagnitude self, AntiScalar other) {
    return HomogeneousMagnitude(self.g0 * vec2(other.g0));
}

HomogeneousMagnitude homogeneous_magnitude_anti_scalar_meet(HomogeneousMagnitude self, AntiScalar other) {
    return HomogeneousMagnitude(self.g0 * vec2(other.g0));
}

AntiScalar homogeneous_magnitude_anti_scalar_outer_product(HomogeneousMagnitude self, AntiScalar other) {
    return AntiScalar(self.g0.x * other.g0);
}

AntiScalar homogeneous_magnitude_anti_scalar_wedge(HomogeneousMagnitude self, AntiScalar other) {
    return AntiScalar(self.g0.x * other.g0);
}

AntiScalar homogeneous_magnitude_anti_scalar_join(HomogeneousMagnitude self, AntiScalar other) {
    return AntiScalar(self.g0.x * other.g0);
}

AntiScalar homogeneous_magnitude_anti_scalar_inner_product(HomogeneousMagnitude self, AntiScalar other) {
    return AntiScalar(self.g0.x * other.g0);
}

HomogeneousMagnitude homogeneous_magnitude_anti_scalar_inner_anti_product(HomogeneousMagnitude self, AntiScalar other) {
    return HomogeneousMagnitude(self.g0 * vec2(other.g0));
}

AntiScalar homogeneous_magnitude_anti_scalar_left_contraction(HomogeneousMagnitude self, AntiScalar other) {
    return AntiScalar(self.g0.x * other.g0);
}

AntiScalar homogeneous_magnitude_anti_scalar_left_anti_contraction(HomogeneousMagnitude self, AntiScalar other) {
    return AntiScalar(self.g0.y * other.g0);
}

HomogeneousMagnitude homogeneous_magnitude_anti_scalar_right_anti_contraction(HomogeneousMagnitude self, AntiScalar other) {
    return HomogeneousMagnitude(self.g0 * vec2(other.g0));
}

AntiScalar homogeneous_magnitude_anti_scalar_anti_scalar_product(HomogeneousMagnitude self, AntiScalar other) {
    return AntiScalar(self.g0.y * other.g0);
}

AntiScalar homogeneous_magnitude_anti_scalar_anti_dot(HomogeneousMagnitude self, AntiScalar other) {
    return AntiScalar(self.g0.y * other.g0);
}

HomogeneousMagnitude homogeneous_magnitude_homogeneous_magnitude_add(HomogeneousMagnitude self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(self.g0 + other.g0);
}

HomogeneousMagnitude homogeneous_magnitude_homogeneous_magnitude_sub(HomogeneousMagnitude self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(self.g0 - other.g0);
}

HomogeneousMagnitude homogeneous_magnitude_homogeneous_magnitude_mul(HomogeneousMagnitude self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(self.g0 * other.g0);
}

HomogeneousMagnitude homogeneous_magnitude_homogeneous_magnitude_div(HomogeneousMagnitude self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(vec2(self.g0.x, self.g0.y) * vec2(1.0, 1.0) / vec2(other.g0.x, other.g0.y) * vec2(1.0, 1.0));
}

HomogeneousMagnitude homogeneous_magnitude_homogeneous_magnitude_geometric_product(HomogeneousMagnitude self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(vec2(self.g0.x) * other.g0 + self.g0 * vec2(other.g0.x) * vec2(0.0, 1.0));
}

HomogeneousMagnitude homogeneous_magnitude_homogeneous_magnitude_geometric_anti_product(HomogeneousMagnitude self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(vec2(self.g0.y) * other.g0 + vec2(self.g0.x) * other.g0.yx * vec2(1.0, 0.0));
}

HomogeneousMagnitude homogeneous_magnitude_homogeneous_magnitude_regressive_product(HomogeneousMagnitude self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(vec2(self.g0.y) * other.g0 + vec2(self.g0.x) * other.g0.yx * vec2(1.0, 0.0));
}

HomogeneousMagnitude homogeneous_magnitude_homogeneous_magnitude_anti_wedge(HomogeneousMagnitude self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(vec2(self.g0.y) * other.g0 + vec2(self.g0.x) * other.g0.yx * vec2(1.0, 0.0));
}

HomogeneousMagnitude homogeneous_magnitude_homogeneous_magnitude_meet(HomogeneousMagnitude self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(vec2(self.g0.y) * other.g0 + vec2(self.g0.x) * other.g0.yx * vec2(1.0, 0.0));
}

HomogeneousMagnitude homogeneous_magnitude_homogeneous_magnitude_outer_product(HomogeneousMagnitude self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(vec2(self.g0.x) * other.g0 + self.g0 * vec2(other.g0.x) * vec2(0.0, 1.0));
}

HomogeneousMagnitude homogeneous_magnitude_homogeneous_magnitude_wedge(HomogeneousMagnitude self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(vec2(self.g0.x) * other.g0 + self.g0 * vec2(other.g0.x) * vec2(0.0, 1.0));
}

HomogeneousMagnitude homogeneous_magnitude_homogeneous_magnitude_join(HomogeneousMagnitude self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(vec2(self.g0.x) * other.g0 + self.g0 * vec2(other.g0.x) * vec2(0.0, 1.0));
}

HomogeneousMagnitude homogeneous_magnitude_homogeneous_magnitude_inner_product(HomogeneousMagnitude self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(vec2(self.g0.x) * other.g0 + self.g0 * vec2(other.g0.x) * vec2(0.0, 1.0));
}

HomogeneousMagnitude homogeneous_magnitude_homogeneous_magnitude_inner_anti_product(HomogeneousMagnitude self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(vec2(self.g0.y) * other.g0 + vec2(self.g0.x) * other.g0.yx * vec2(1.0, 0.0));
}

HomogeneousMagnitude homogeneous_magnitude_homogeneous_magnitude_left_contraction(HomogeneousMagnitude self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(vec2(self.g0.x) * other.g0);
}

HomogeneousMagnitude homogeneous_magnitude_homogeneous_magnitude_right_contraction(HomogeneousMagnitude self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(self.g0 * vec2(other.g0.x));
}

HomogeneousMagnitude homogeneous_magnitude_homogeneous_magnitude_left_anti_contraction(HomogeneousMagnitude self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(vec2(self.g0.y) * other.g0);
}

HomogeneousMagnitude homogeneous_magnitude_homogeneous_magnitude_right_anti_contraction(HomogeneousMagnitude self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(self.g0 * vec2(other.g0.y));
}

Scalar homogeneous_magnitude_homogeneous_magnitude_scalar_product(HomogeneousMagnitude self, HomogeneousMagnitude other) {
    return Scalar(self.g0.x * other.g0.x);
}

Scalar homogeneous_magnitude_homogeneous_magnitude_dot(HomogeneousMagnitude self, HomogeneousMagnitude other) {
    return Scalar(self.g0.x * other.g0.x);
}

AntiScalar homogeneous_magnitude_homogeneous_magnitude_anti_scalar_product(HomogeneousMagnitude self, HomogeneousMagnitude other) {
    return AntiScalar(self.g0.y * other.g0.y);
}

AntiScalar homogeneous_magnitude_homogeneous_magnitude_anti_dot(HomogeneousMagnitude self, HomogeneousMagnitude other) {
    return AntiScalar(self.g0.y * other.g0.y);
}

Flector homogeneous_magnitude_point_geometric_product(HomogeneousMagnitude self, Point other) {
    return Flector(vec4(self.g0.x) * other.g0, vec4(self.g0.y, self.g0.y, self.g0.y, self.g0.x) * other.g0.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

Flector homogeneous_magnitude_point_geometric_anti_product(HomogeneousMagnitude self, Point other) {
    return Flector(vec4(self.g0.y) * other.g0, vec4(self.g0.x) * other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0));
}

Point homogeneous_magnitude_point_regressive_product(HomogeneousMagnitude self, Point other) {
    return Point(vec4(self.g0.y) * other.g0);
}

Point homogeneous_magnitude_point_anti_wedge(HomogeneousMagnitude self, Point other) {
    return Point(vec4(self.g0.y) * other.g0);
}

Point homogeneous_magnitude_point_meet(HomogeneousMagnitude self, Point other) {
    return Point(vec4(self.g0.y) * other.g0);
}

Point homogeneous_magnitude_point_outer_product(HomogeneousMagnitude self, Point other) {
    return Point(vec4(self.g0.x) * other.g0);
}

Point homogeneous_magnitude_point_wedge(HomogeneousMagnitude self, Point other) {
    return Point(vec4(self.g0.x) * other.g0);
}

Point homogeneous_magnitude_point_join(HomogeneousMagnitude self, Point other) {
    return Point(vec4(self.g0.x) * other.g0);
}

Flector homogeneous_magnitude_point_inner_product(HomogeneousMagnitude self, Point other) {
    return Flector(vec4(self.g0.x) * other.g0, vec4(self.g0.y, self.g0.y, self.g0.y, self.g0.x) * other.g0.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

Flector homogeneous_magnitude_point_inner_anti_product(HomogeneousMagnitude self, Point other) {
    return Flector(vec4(self.g0.y) * other.g0, vec4(self.g0.x) * other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0));
}

Point homogeneous_magnitude_point_left_contraction(HomogeneousMagnitude self, Point other) {
    return Point(vec4(self.g0.x) * other.g0);
}

Plane homogeneous_magnitude_point_right_contraction(HomogeneousMagnitude self, Point other) {
    return Plane(vec4(self.g0.y, self.g0.y, self.g0.y, self.g0.x) * other.g0.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

Point homogeneous_magnitude_point_left_anti_contraction(HomogeneousMagnitude self, Point other) {
    return Point(vec4(self.g0.y) * other.g0);
}

Plane homogeneous_magnitude_point_right_anti_contraction(HomogeneousMagnitude self, Point other) {
    return Plane(vec4(self.g0.x) * other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0));
}

Line homogeneous_magnitude_line_geometric_product(HomogeneousMagnitude self, Line other) {
    return Line(vec3(self.g0.x) * other.g0 + vec3(self.g0.y) * other.g1, vec3(self.g0.x) * other.g1);
}

Line homogeneous_magnitude_line_geometric_anti_product(HomogeneousMagnitude self, Line other) {
    return Line(vec3(self.g0.y) * other.g0, vec3(self.g0.x) * other.g0 + vec3(self.g0.y) * other.g1);
}

Line homogeneous_magnitude_line_regressive_product(HomogeneousMagnitude self, Line other) {
    return Line(vec3(self.g0.y) * other.g0, vec3(self.g0.y) * other.g1);
}

Line homogeneous_magnitude_line_anti_wedge(HomogeneousMagnitude self, Line other) {
    return Line(vec3(self.g0.y) * other.g0, vec3(self.g0.y) * other.g1);
}

Line homogeneous_magnitude_line_meet(HomogeneousMagnitude self, Line other) {
    return Line(vec3(self.g0.y) * other.g0, vec3(self.g0.y) * other.g1);
}

Line homogeneous_magnitude_line_outer_product(HomogeneousMagnitude self, Line other) {
    return Line(vec3(self.g0.x) * other.g0, vec3(self.g0.x) * other.g1);
}

Line homogeneous_magnitude_line_wedge(HomogeneousMagnitude self, Line other) {
    return Line(vec3(self.g0.x) * other.g0, vec3(self.g0.x) * other.g1);
}

Line homogeneous_magnitude_line_join(HomogeneousMagnitude self, Line other) {
    return Line(vec3(self.g0.x) * other.g0, vec3(self.g0.x) * other.g1);
}

Line homogeneous_magnitude_line_inner_product(HomogeneousMagnitude self, Line other) {
    return Line(vec3(self.g0.x) * other.g0 + vec3(self.g0.y) * other.g1, vec3(self.g0.x) * other.g1);
}

Line homogeneous_magnitude_line_inner_anti_product(HomogeneousMagnitude self, Line other) {
    return Line(vec3(self.g0.y) * other.g0, vec3(self.g0.x) * other.g0 + vec3(self.g0.y) * other.g1);
}

Line homogeneous_magnitude_line_left_contraction(HomogeneousMagnitude self, Line other) {
    return Line(vec3(self.g0.x) * other.g0, vec3(self.g0.x) * other.g1);
}

Rotor homogeneous_magnitude_line_right_contraction(HomogeneousMagnitude self, Line other) {
    return Rotor(vec4(self.g0.y, self.g0.y, self.g0.y, self.g0.x) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Line homogeneous_magnitude_line_left_anti_contraction(HomogeneousMagnitude self, Line other) {
    return Line(vec3(self.g0.y) * other.g0, vec3(self.g0.y) * other.g1);
}

Translator homogeneous_magnitude_line_right_anti_contraction(HomogeneousMagnitude self, Line other) {
    return Translator(vec4(self.g0.x) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Flector homogeneous_magnitude_plane_geometric_product(HomogeneousMagnitude self, Plane other) {
    return Flector(vec4(self.g0.x, self.g0.x, self.g0.x, self.g0.y) * other.g0.xxxw * vec4(0.0, 0.0, 0.0, -1.0), vec4(self.g0.x) * other.g0);
}

Flector homogeneous_magnitude_plane_geometric_anti_product(HomogeneousMagnitude self, Plane other) {
    return Flector(vec4(self.g0.x) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0), vec4(self.g0.y) * other.g0);
}

Plane homogeneous_magnitude_plane_regressive_product(HomogeneousMagnitude self, Plane other) {
    return Plane(vec4(self.g0.y) * other.g0);
}

Plane homogeneous_magnitude_plane_anti_wedge(HomogeneousMagnitude self, Plane other) {
    return Plane(vec4(self.g0.y) * other.g0);
}

Plane homogeneous_magnitude_plane_meet(HomogeneousMagnitude self, Plane other) {
    return Plane(vec4(self.g0.y) * other.g0);
}

Plane homogeneous_magnitude_plane_outer_product(HomogeneousMagnitude self, Plane other) {
    return Plane(vec4(self.g0.x) * other.g0);
}

Plane homogeneous_magnitude_plane_wedge(HomogeneousMagnitude self, Plane other) {
    return Plane(vec4(self.g0.x) * other.g0);
}

Plane homogeneous_magnitude_plane_join(HomogeneousMagnitude self, Plane other) {
    return Plane(vec4(self.g0.x) * other.g0);
}

Flector homogeneous_magnitude_plane_inner_product(HomogeneousMagnitude self, Plane other) {
    return Flector(vec4(self.g0.x, self.g0.x, self.g0.x, self.g0.y) * other.g0.xxxw * vec4(0.0, 0.0, 0.0, -1.0), vec4(self.g0.x) * other.g0);
}

Flector homogeneous_magnitude_plane_inner_anti_product(HomogeneousMagnitude self, Plane other) {
    return Flector(vec4(self.g0.x) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0), vec4(self.g0.y) * other.g0);
}

Plane homogeneous_magnitude_plane_left_contraction(HomogeneousMagnitude self, Plane other) {
    return Plane(vec4(self.g0.x) * other.g0);
}

Point homogeneous_magnitude_plane_right_contraction(HomogeneousMagnitude self, Plane other) {
    return Point(vec4(self.g0.x, self.g0.x, self.g0.x, self.g0.y) * other.g0.xxxw * vec4(0.0, 0.0, 0.0, -1.0));
}

Plane homogeneous_magnitude_plane_left_anti_contraction(HomogeneousMagnitude self, Plane other) {
    return Plane(vec4(self.g0.y) * other.g0);
}

Point homogeneous_magnitude_plane_right_anti_contraction(HomogeneousMagnitude self, Plane other) {
    return Point(vec4(self.g0.x) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0));
}

Motor homogeneous_magnitude_motor_geometric_product(HomogeneousMagnitude self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0 + vec4(self.g0.y, self.g0.y, self.g0.y, self.g0.x) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0), vec3(self.g0.x) * other.g1);
}

MultiVector homogeneous_magnitude_motor_geometric_anti_product(HomogeneousMagnitude self, Motor other) {
    return MultiVector(self.g0 * vec2(other.g0.w), vec4(0.0), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g0.y) * other.g1, vec4(0.0));
}

MultiVector homogeneous_magnitude_motor_regressive_product(HomogeneousMagnitude self, Motor other) {
    return MultiVector(self.g0 * vec2(other.g0.w), vec4(0.0), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.y) * other.g1, vec4(0.0));
}

MultiVector homogeneous_magnitude_motor_anti_wedge(HomogeneousMagnitude self, Motor other) {
    return MultiVector(self.g0 * vec2(other.g0.w), vec4(0.0), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.y) * other.g1, vec4(0.0));
}

MultiVector homogeneous_magnitude_motor_meet(HomogeneousMagnitude self, Motor other) {
    return MultiVector(self.g0 * vec2(other.g0.w), vec4(0.0), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.y) * other.g1, vec4(0.0));
}

Motor homogeneous_magnitude_motor_outer_product(HomogeneousMagnitude self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0, vec3(self.g0.x) * other.g1);
}

Motor homogeneous_magnitude_motor_wedge(HomogeneousMagnitude self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0, vec3(self.g0.x) * other.g1);
}

Motor homogeneous_magnitude_motor_join(HomogeneousMagnitude self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0, vec3(self.g0.x) * other.g1);
}

Motor homogeneous_magnitude_motor_inner_product(HomogeneousMagnitude self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0 + vec4(self.g0.y, self.g0.y, self.g0.y, self.g0.x) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0), vec3(self.g0.x) * other.g1);
}

MultiVector homogeneous_magnitude_motor_inner_anti_product(HomogeneousMagnitude self, Motor other) {
    return MultiVector(self.g0 * vec2(other.g0.w), vec4(0.0), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g0.y) * other.g1, vec4(0.0));
}

Motor homogeneous_magnitude_motor_left_contraction(HomogeneousMagnitude self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0, vec3(self.g0.x) * other.g1);
}

Rotor homogeneous_magnitude_motor_right_contraction(HomogeneousMagnitude self, Motor other) {
    return Rotor(vec4(self.g0.y, self.g0.y, self.g0.y, self.g0.x) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Motor homogeneous_magnitude_motor_left_anti_contraction(HomogeneousMagnitude self, Motor other) {
    return Motor(vec4(self.g0.y) * other.g0, vec3(self.g0.y) * other.g1);
}

MultiVector homogeneous_magnitude_motor_right_anti_contraction(HomogeneousMagnitude self, Motor other) {
    return MultiVector(self.g0 * vec2(other.g0.w), vec4(0.0), vec3(0.0), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(0.0));
}

AntiScalar homogeneous_magnitude_motor_anti_scalar_product(HomogeneousMagnitude self, Motor other) {
    return AntiScalar(self.g0.y * other.g0.w);
}

AntiScalar homogeneous_magnitude_motor_anti_dot(HomogeneousMagnitude self, Motor other) {
    return AntiScalar(self.g0.y * other.g0.w);
}

Rotor homogeneous_magnitude_rotor_geometric_product(HomogeneousMagnitude self, Rotor other) {
    return Rotor(vec4(self.g0.x) * other.g0);
}

MultiVector homogeneous_magnitude_rotor_geometric_anti_product(HomogeneousMagnitude self, Rotor other) {
    return MultiVector(self.g0 * vec2(other.g0.w), vec4(0.0), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(0.0));
}

MultiVector homogeneous_magnitude_rotor_regressive_product(HomogeneousMagnitude self, Rotor other) {
    return MultiVector(self.g0 * vec2(other.g0.w), vec4(0.0), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(0.0), vec4(0.0));
}

MultiVector homogeneous_magnitude_rotor_anti_wedge(HomogeneousMagnitude self, Rotor other) {
    return MultiVector(self.g0 * vec2(other.g0.w), vec4(0.0), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(0.0), vec4(0.0));
}

MultiVector homogeneous_magnitude_rotor_meet(HomogeneousMagnitude self, Rotor other) {
    return MultiVector(self.g0 * vec2(other.g0.w), vec4(0.0), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(0.0), vec4(0.0));
}

Rotor homogeneous_magnitude_rotor_outer_product(HomogeneousMagnitude self, Rotor other) {
    return Rotor(vec4(self.g0.x) * other.g0);
}

Rotor homogeneous_magnitude_rotor_wedge(HomogeneousMagnitude self, Rotor other) {
    return Rotor(vec4(self.g0.x) * other.g0);
}

Rotor homogeneous_magnitude_rotor_join(HomogeneousMagnitude self, Rotor other) {
    return Rotor(vec4(self.g0.x) * other.g0);
}

Rotor homogeneous_magnitude_rotor_inner_product(HomogeneousMagnitude self, Rotor other) {
    return Rotor(vec4(self.g0.x) * other.g0);
}

MultiVector homogeneous_magnitude_rotor_inner_anti_product(HomogeneousMagnitude self, Rotor other) {
    return MultiVector(self.g0 * vec2(other.g0.w), vec4(0.0), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(0.0));
}

Rotor homogeneous_magnitude_rotor_left_contraction(HomogeneousMagnitude self, Rotor other) {
    return Rotor(vec4(self.g0.x) * other.g0);
}

Rotor homogeneous_magnitude_rotor_left_anti_contraction(HomogeneousMagnitude self, Rotor other) {
    return Rotor(vec4(self.g0.y) * other.g0);
}

MultiVector homogeneous_magnitude_rotor_right_anti_contraction(HomogeneousMagnitude self, Rotor other) {
    return MultiVector(self.g0 * vec2(other.g0.w), vec4(0.0), vec3(0.0), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(0.0));
}

AntiScalar homogeneous_magnitude_rotor_anti_scalar_product(HomogeneousMagnitude self, Rotor other) {
    return AntiScalar(self.g0.y * other.g0.w);
}

AntiScalar homogeneous_magnitude_rotor_anti_dot(HomogeneousMagnitude self, Rotor other) {
    return AntiScalar(self.g0.y * other.g0.w);
}

Motor homogeneous_magnitude_translator_geometric_product(HomogeneousMagnitude self, Translator other) {
    return Motor(vec4(self.g0.y, self.g0.y, self.g0.y, self.g0.x) * other.g0, vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z));
}

MultiVector homogeneous_magnitude_translator_geometric_anti_product(HomogeneousMagnitude self, Translator other) {
    return MultiVector(self.g0 * vec2(other.g0.w), vec4(0.0), vec3(0.0), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(0.0));
}

MultiVector homogeneous_magnitude_translator_regressive_product(HomogeneousMagnitude self, Translator other) {
    return MultiVector(self.g0 * vec2(other.g0.w), vec4(0.0), vec3(0.0), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(0.0));
}

MultiVector homogeneous_magnitude_translator_anti_wedge(HomogeneousMagnitude self, Translator other) {
    return MultiVector(self.g0 * vec2(other.g0.w), vec4(0.0), vec3(0.0), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(0.0));
}

MultiVector homogeneous_magnitude_translator_meet(HomogeneousMagnitude self, Translator other) {
    return MultiVector(self.g0 * vec2(other.g0.w), vec4(0.0), vec3(0.0), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(0.0));
}

Translator homogeneous_magnitude_translator_outer_product(HomogeneousMagnitude self, Translator other) {
    return Translator(vec4(self.g0.x) * other.g0);
}

Translator homogeneous_magnitude_translator_wedge(HomogeneousMagnitude self, Translator other) {
    return Translator(vec4(self.g0.x) * other.g0);
}

Translator homogeneous_magnitude_translator_join(HomogeneousMagnitude self, Translator other) {
    return Translator(vec4(self.g0.x) * other.g0);
}

Motor homogeneous_magnitude_translator_inner_product(HomogeneousMagnitude self, Translator other) {
    return Motor(vec4(self.g0.y, self.g0.y, self.g0.y, self.g0.x) * other.g0, vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z));
}

MultiVector homogeneous_magnitude_translator_inner_anti_product(HomogeneousMagnitude self, Translator other) {
    return MultiVector(self.g0 * vec2(other.g0.w), vec4(0.0), vec3(0.0), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(0.0));
}

Translator homogeneous_magnitude_translator_left_contraction(HomogeneousMagnitude self, Translator other) {
    return Translator(vec4(self.g0.x) * other.g0);
}

Rotor homogeneous_magnitude_translator_right_contraction(HomogeneousMagnitude self, Translator other) {
    return Rotor(vec4(self.g0.y, self.g0.y, self.g0.y, self.g0.x) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0));
}

Translator homogeneous_magnitude_translator_left_anti_contraction(HomogeneousMagnitude self, Translator other) {
    return Translator(vec4(self.g0.y) * other.g0);
}

HomogeneousMagnitude homogeneous_magnitude_translator_right_anti_contraction(HomogeneousMagnitude self, Translator other) {
    return HomogeneousMagnitude(self.g0 * vec2(other.g0.w));
}

AntiScalar homogeneous_magnitude_translator_anti_scalar_product(HomogeneousMagnitude self, Translator other) {
    return AntiScalar(self.g0.y * other.g0.w);
}

AntiScalar homogeneous_magnitude_translator_anti_dot(HomogeneousMagnitude self, Translator other) {
    return AntiScalar(self.g0.y * other.g0.w);
}

Flector homogeneous_magnitude_flector_geometric_product(HomogeneousMagnitude self, Flector other) {
    return Flector(vec4(self.g0.x) * other.g0 + vec4(self.g0.x, self.g0.x, self.g0.x, self.g0.y) * other.g1.xxxw * vec4(0.0, 0.0, 0.0, -1.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.y, self.g0.y, self.g0.y, self.g0.x) * other.g0.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

Flector homogeneous_magnitude_flector_geometric_anti_product(HomogeneousMagnitude self, Flector other) {
    return Flector(vec4(self.g0.y) * other.g0 + vec4(self.g0.x) * other.g1.xyzx * vec4(1.0, 1.0, 1.0, 0.0), vec4(self.g0.y) * other.g1 + vec4(self.g0.x) * other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0));
}

Flector homogeneous_magnitude_flector_regressive_product(HomogeneousMagnitude self, Flector other) {
    return Flector(vec4(self.g0.y) * other.g0, vec4(self.g0.y) * other.g1);
}

Flector homogeneous_magnitude_flector_anti_wedge(HomogeneousMagnitude self, Flector other) {
    return Flector(vec4(self.g0.y) * other.g0, vec4(self.g0.y) * other.g1);
}

Flector homogeneous_magnitude_flector_meet(HomogeneousMagnitude self, Flector other) {
    return Flector(vec4(self.g0.y) * other.g0, vec4(self.g0.y) * other.g1);
}

Flector homogeneous_magnitude_flector_outer_product(HomogeneousMagnitude self, Flector other) {
    return Flector(vec4(self.g0.x) * other.g0, vec4(self.g0.x) * other.g1);
}

Flector homogeneous_magnitude_flector_wedge(HomogeneousMagnitude self, Flector other) {
    return Flector(vec4(self.g0.x) * other.g0, vec4(self.g0.x) * other.g1);
}

Flector homogeneous_magnitude_flector_join(HomogeneousMagnitude self, Flector other) {
    return Flector(vec4(self.g0.x) * other.g0, vec4(self.g0.x) * other.g1);
}

Flector homogeneous_magnitude_flector_inner_product(HomogeneousMagnitude self, Flector other) {
    return Flector(vec4(self.g0.x) * other.g0 + vec4(self.g0.x, self.g0.x, self.g0.x, self.g0.y) * other.g1.xxxw * vec4(0.0, 0.0, 0.0, -1.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.y, self.g0.y, self.g0.y, self.g0.x) * other.g0.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

Flector homogeneous_magnitude_flector_inner_anti_product(HomogeneousMagnitude self, Flector other) {
    return Flector(vec4(self.g0.y) * other.g0 + vec4(self.g0.x) * other.g1.xyzx * vec4(1.0, 1.0, 1.0, 0.0), vec4(self.g0.y) * other.g1 + vec4(self.g0.x) * other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0));
}

Flector homogeneous_magnitude_flector_left_contraction(HomogeneousMagnitude self, Flector other) {
    return Flector(vec4(self.g0.x) * other.g0, vec4(self.g0.x) * other.g1);
}

Flector homogeneous_magnitude_flector_right_contraction(HomogeneousMagnitude self, Flector other) {
    return Flector(vec4(self.g0.x, self.g0.x, self.g0.x, self.g0.y) * other.g1.xxxw * vec4(0.0, 0.0, 0.0, -1.0), vec4(self.g0.y, self.g0.y, self.g0.y, self.g0.x) * other.g0.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

Flector homogeneous_magnitude_flector_left_anti_contraction(HomogeneousMagnitude self, Flector other) {
    return Flector(vec4(self.g0.y) * other.g0, vec4(self.g0.y) * other.g1);
}

Flector homogeneous_magnitude_flector_right_anti_contraction(HomogeneousMagnitude self, Flector other) {
    return Flector(vec4(self.g0.x) * other.g1.xyzx * vec4(1.0, 1.0, 1.0, 0.0), vec4(self.g0.x) * other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0));
}

MultiVector homogeneous_magnitude_multi_vector_add(HomogeneousMagnitude self, MultiVector other) {
    return MultiVector(self.g0 + other.g0, other.g1, other.g2, other.g3, other.g4);
}

MultiVector homogeneous_magnitude_multi_vector_sub(HomogeneousMagnitude self, MultiVector other) {
    return MultiVector(self.g0 - other.g0, vec4(0.0) - other.g1, vec3(0.0) - other.g2, vec3(0.0) - other.g3, vec4(0.0) - other.g4);
}

MultiVector homogeneous_magnitude_multi_vector_geometric_product(HomogeneousMagnitude self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * other.g0 + self.g0 * vec2(other.g0.x) * vec2(0.0, 1.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.x, self.g0.x, self.g0.x, self.g0.y) * other.g4.xxxw * vec4(0.0, 0.0, 0.0, -1.0), vec3(self.g0.x) * other.g2 + vec3(self.g0.y) * other.g3, vec3(self.g0.x) * other.g3, vec4(self.g0.x) * other.g4 + vec4(self.g0.y, self.g0.y, self.g0.y, self.g0.x) * other.g1.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

MultiVector homogeneous_magnitude_multi_vector_geometric_anti_product(HomogeneousMagnitude self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * other.g0 + vec2(self.g0.x) * other.g0.yx * vec2(1.0, 0.0), vec4(self.g0.y) * other.g1 + vec4(self.g0.x) * other.g4.xyzx * vec4(1.0, 1.0, 1.0, 0.0), vec3(self.g0.y) * other.g2, vec3(self.g0.x) * other.g2 + vec3(self.g0.y) * other.g3, vec4(self.g0.y) * other.g4 + vec4(self.g0.x) * other.g1.xxxw * vec4(0.0, 0.0, 0.0, 1.0));
}

MultiVector homogeneous_magnitude_multi_vector_regressive_product(HomogeneousMagnitude self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * other.g0 + vec2(self.g0.x) * other.g0.yx * vec2(1.0, 0.0), vec4(self.g0.y) * other.g1, vec3(self.g0.y) * other.g2, vec3(self.g0.y) * other.g3, vec4(self.g0.y) * other.g4);
}

MultiVector homogeneous_magnitude_multi_vector_anti_wedge(HomogeneousMagnitude self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * other.g0 + vec2(self.g0.x) * other.g0.yx * vec2(1.0, 0.0), vec4(self.g0.y) * other.g1, vec3(self.g0.y) * other.g2, vec3(self.g0.y) * other.g3, vec4(self.g0.y) * other.g4);
}

MultiVector homogeneous_magnitude_multi_vector_meet(HomogeneousMagnitude self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * other.g0 + vec2(self.g0.x) * other.g0.yx * vec2(1.0, 0.0), vec4(self.g0.y) * other.g1, vec3(self.g0.y) * other.g2, vec3(self.g0.y) * other.g3, vec4(self.g0.y) * other.g4);
}

MultiVector homogeneous_magnitude_multi_vector_outer_product(HomogeneousMagnitude self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * other.g0 + self.g0 * vec2(other.g0.x) * vec2(0.0, 1.0), vec4(self.g0.x) * other.g1, vec3(self.g0.x) * other.g2, vec3(self.g0.x) * other.g3, vec4(self.g0.x) * other.g4);
}

MultiVector homogeneous_magnitude_multi_vector_wedge(HomogeneousMagnitude self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * other.g0 + self.g0 * vec2(other.g0.x) * vec2(0.0, 1.0), vec4(self.g0.x) * other.g1, vec3(self.g0.x) * other.g2, vec3(self.g0.x) * other.g3, vec4(self.g0.x) * other.g4);
}

MultiVector homogeneous_magnitude_multi_vector_join(HomogeneousMagnitude self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * other.g0 + self.g0 * vec2(other.g0.x) * vec2(0.0, 1.0), vec4(self.g0.x) * other.g1, vec3(self.g0.x) * other.g2, vec3(self.g0.x) * other.g3, vec4(self.g0.x) * other.g4);
}

MultiVector homogeneous_magnitude_multi_vector_inner_product(HomogeneousMagnitude self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * other.g0 + self.g0 * vec2(other.g0.x) * vec2(0.0, 1.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.x, self.g0.x, self.g0.x, self.g0.y) * other.g4.xxxw * vec4(0.0, 0.0, 0.0, -1.0), vec3(self.g0.x) * other.g2 + vec3(self.g0.y) * other.g3, vec3(self.g0.x) * other.g3, vec4(self.g0.x) * other.g4 + vec4(self.g0.y, self.g0.y, self.g0.y, self.g0.x) * other.g1.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

MultiVector homogeneous_magnitude_multi_vector_inner_anti_product(HomogeneousMagnitude self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * other.g0 + vec2(self.g0.x) * other.g0.yx * vec2(1.0, 0.0), vec4(self.g0.y) * other.g1 + vec4(self.g0.x) * other.g4.xyzx * vec4(1.0, 1.0, 1.0, 0.0), vec3(self.g0.y) * other.g2, vec3(self.g0.x) * other.g2 + vec3(self.g0.y) * other.g3, vec4(self.g0.y) * other.g4 + vec4(self.g0.x) * other.g1.xxxw * vec4(0.0, 0.0, 0.0, 1.0));
}

MultiVector homogeneous_magnitude_multi_vector_left_contraction(HomogeneousMagnitude self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * other.g0, vec4(self.g0.x) * other.g1, vec3(self.g0.x) * other.g2, vec3(self.g0.x) * other.g3, vec4(self.g0.x) * other.g4);
}

MultiVector homogeneous_magnitude_multi_vector_right_contraction(HomogeneousMagnitude self, MultiVector other) {
    return MultiVector(self.g0 * vec2(other.g0.x), vec4(self.g0.x, self.g0.x, self.g0.x, self.g0.y) * other.g4.xxxw * vec4(0.0, 0.0, 0.0, -1.0), vec3(self.g0.y) * other.g3, vec3(0.0), vec4(self.g0.y, self.g0.y, self.g0.y, self.g0.x) * other.g1.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

MultiVector homogeneous_magnitude_multi_vector_left_anti_contraction(HomogeneousMagnitude self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * other.g0, vec4(self.g0.y) * other.g1, vec3(self.g0.y) * other.g2, vec3(self.g0.y) * other.g3, vec4(self.g0.y) * other.g4);
}

MultiVector homogeneous_magnitude_multi_vector_right_anti_contraction(HomogeneousMagnitude self, MultiVector other) {
    return MultiVector(self.g0 * vec2(other.g0.y), vec4(self.g0.x) * other.g4.xyzx * vec4(1.0, 1.0, 1.0, 0.0), vec3(0.0), vec3(self.g0.x) * other.g2, vec4(self.g0.x) * other.g1.xxxw * vec4(0.0, 0.0, 0.0, 1.0));
}

Scalar homogeneous_magnitude_multi_vector_scalar_product(HomogeneousMagnitude self, MultiVector other) {
    return Scalar(self.g0.x * other.g0.x);
}

Scalar homogeneous_magnitude_multi_vector_dot(HomogeneousMagnitude self, MultiVector other) {
    return Scalar(self.g0.x * other.g0.x);
}

AntiScalar homogeneous_magnitude_multi_vector_anti_scalar_product(HomogeneousMagnitude self, MultiVector other) {
    return AntiScalar(self.g0.y * other.g0.y);
}

AntiScalar homogeneous_magnitude_multi_vector_anti_dot(HomogeneousMagnitude self, MultiVector other) {
    return AntiScalar(self.g0.y * other.g0.y);
}

Point point_scalar_geometric_product(Point self, Scalar other) {
    return Point(self.g0 * vec4(other.g0));
}

Plane point_scalar_geometric_anti_product(Point self, Scalar other) {
    return Plane(self.g0.xxxw * vec4(other.g0) * vec4(0.0, 0.0, 0.0, -1.0));
}

Point point_scalar_outer_product(Point self, Scalar other) {
    return Point(self.g0 * vec4(other.g0));
}

Point point_scalar_wedge(Point self, Scalar other) {
    return Point(self.g0 * vec4(other.g0));
}

Point point_scalar_join(Point self, Scalar other) {
    return Point(self.g0 * vec4(other.g0));
}

Point point_scalar_inner_product(Point self, Scalar other) {
    return Point(self.g0 * vec4(other.g0));
}

Plane point_scalar_inner_anti_product(Point self, Scalar other) {
    return Plane(self.g0.xxxw * vec4(other.g0) * vec4(0.0, 0.0, 0.0, -1.0));
}

Point point_scalar_right_contraction(Point self, Scalar other) {
    return Point(self.g0 * vec4(other.g0));
}

Plane point_scalar_left_anti_contraction(Point self, Scalar other) {
    return Plane(self.g0.xxxw * vec4(other.g0) * vec4(0.0, 0.0, 0.0, -1.0));
}

Plane point_anti_scalar_geometric_product(Point self, AntiScalar other) {
    return Plane(self.g0.xyzx * vec4(other.g0) * vec4(1.0, 1.0, 1.0, 0.0));
}

Point point_anti_scalar_geometric_anti_product(Point self, AntiScalar other) {
    return Point(self.g0 * vec4(other.g0));
}

Point point_anti_scalar_regressive_product(Point self, AntiScalar other) {
    return Point(self.g0 * vec4(other.g0));
}

Point point_anti_scalar_anti_wedge(Point self, AntiScalar other) {
    return Point(self.g0 * vec4(other.g0));
}

Point point_anti_scalar_meet(Point self, AntiScalar other) {
    return Point(self.g0 * vec4(other.g0));
}

Plane point_anti_scalar_inner_product(Point self, AntiScalar other) {
    return Plane(self.g0.xyzx * vec4(other.g0) * vec4(1.0, 1.0, 1.0, 0.0));
}

Point point_anti_scalar_inner_anti_product(Point self, AntiScalar other) {
    return Point(self.g0 * vec4(other.g0));
}

Plane point_anti_scalar_left_contraction(Point self, AntiScalar other) {
    return Plane(self.g0.xyzx * vec4(other.g0) * vec4(1.0, 1.0, 1.0, 0.0));
}

Point point_anti_scalar_right_anti_contraction(Point self, AntiScalar other) {
    return Point(self.g0 * vec4(other.g0));
}

Flector point_homogeneous_magnitude_geometric_product(Point self, HomogeneousMagnitude other) {
    return Flector(self.g0 * vec4(other.g0.x), self.g0.xyzx * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Flector point_homogeneous_magnitude_geometric_anti_product(Point self, HomogeneousMagnitude other) {
    return Flector(self.g0 * vec4(other.g0.y), self.g0.xxxw * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

Point point_homogeneous_magnitude_regressive_product(Point self, HomogeneousMagnitude other) {
    return Point(self.g0 * vec4(other.g0.y));
}

Point point_homogeneous_magnitude_anti_wedge(Point self, HomogeneousMagnitude other) {
    return Point(self.g0 * vec4(other.g0.y));
}

Point point_homogeneous_magnitude_meet(Point self, HomogeneousMagnitude other) {
    return Point(self.g0 * vec4(other.g0.y));
}

Point point_homogeneous_magnitude_outer_product(Point self, HomogeneousMagnitude other) {
    return Point(self.g0 * vec4(other.g0.x));
}

Point point_homogeneous_magnitude_wedge(Point self, HomogeneousMagnitude other) {
    return Point(self.g0 * vec4(other.g0.x));
}

Point point_homogeneous_magnitude_join(Point self, HomogeneousMagnitude other) {
    return Point(self.g0 * vec4(other.g0.x));
}

Flector point_homogeneous_magnitude_inner_product(Point self, HomogeneousMagnitude other) {
    return Flector(self.g0 * vec4(other.g0.x), self.g0.xyzx * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Flector point_homogeneous_magnitude_inner_anti_product(Point self, HomogeneousMagnitude other) {
    return Flector(self.g0 * vec4(other.g0.y), self.g0.xxxw * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

Plane point_homogeneous_magnitude_left_contraction(Point self, HomogeneousMagnitude other) {
    return Plane(self.g0.xyzx * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Point point_homogeneous_magnitude_right_contraction(Point self, HomogeneousMagnitude other) {
    return Point(self.g0 * vec4(other.g0.x));
}

Plane point_homogeneous_magnitude_left_anti_contraction(Point self, HomogeneousMagnitude other) {
    return Plane(self.g0.xxxw * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

Point point_homogeneous_magnitude_right_anti_contraction(Point self, HomogeneousMagnitude other) {
    return Point(self.g0 * vec4(other.g0.y));
}

Point point_point_add(Point self, Point other) {
    return Point(self.g0 + other.g0);
}

Point point_point_sub(Point self, Point other) {
    return Point(self.g0 - other.g0);
}

Point point_point_mul(Point self, Point other) {
    return Point(self.g0 * other.g0);
}

Point point_point_div(Point self, Point other) {
    return Point(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.w) * vec4(1.0, 1.0, 1.0, 1.0));
}

MultiVector point_point_geometric_product(Point self, Point other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(1.0, 0.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(1.0, 0.0), vec4(0.0), vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w) * vec3(-1.0), vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0), vec4(0.0));
}

Translator point_point_geometric_anti_product(Point self, Point other) {
    return Translator(vec4(self.g0.w) * other.g0 * vec4(1.0, 1.0, 1.0, -1.0) + self.g0.xyzx * other.g0.wwwx * vec4(-1.0, -1.0, -1.0, 0.0));
}

Line point_point_outer_product(Point self, Point other) {
    return Line(vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w) * vec3(-1.0), vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0));
}

Line point_point_wedge(Point self, Point other) {
    return Line(vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w) * vec3(-1.0), vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0));
}

Line point_point_join(Point self, Point other) {
    return Line(vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w) * vec3(-1.0), vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0));
}

Scalar point_point_inner_product(Point self, Point other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

AntiScalar point_point_inner_anti_product(Point self, Point other) {
    return AntiScalar(0.0 - self.g0.w * other.g0.w);
}

Scalar point_point_left_contraction(Point self, Point other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

Scalar point_point_right_contraction(Point self, Point other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

AntiScalar point_point_left_anti_contraction(Point self, Point other) {
    return AntiScalar(0.0 - self.g0.w * other.g0.w);
}

AntiScalar point_point_right_anti_contraction(Point self, Point other) {
    return AntiScalar(0.0 - self.g0.w * other.g0.w);
}

Scalar point_point_scalar_product(Point self, Point other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

Scalar point_point_dot(Point self, Point other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

AntiScalar point_point_anti_scalar_product(Point self, Point other) {
    return AntiScalar(0.0 - self.g0.w * other.g0.w);
}

AntiScalar point_point_anti_dot(Point self, Point other) {
    return AntiScalar(0.0 - self.g0.w * other.g0.w);
}

Flector point_line_geometric_product(Point self, Line other) {
    return Flector(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0), vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

Flector point_line_geometric_anti_product(Point self, Line other) {
    return Flector(vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g0.z) * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.y) * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(-1.0, -1.0, -1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g0.x) * vec4(0.0, -1.0, 1.0, 0.0), vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, -1.0));
}

Plane point_line_outer_product(Point self, Line other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

Plane point_line_wedge(Point self, Line other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

Plane point_line_join(Point self, Line other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

Point point_line_inner_product(Point self, Line other) {
    return Point(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

Plane point_line_inner_anti_product(Point self, Line other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, -1.0));
}

Point point_line_left_contraction(Point self, Line other) {
    return Point(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

Plane point_line_right_anti_contraction(Point self, Line other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, -1.0));
}

Flector point_plane_add(Point self, Plane other) {
    return Flector(self.g0, other.g0);
}

Flector point_plane_sub(Point self, Plane other) {
    return Flector(self.g0, vec4(0.0) - other.g0);
}

Motor point_plane_geometric_product(Point self, Plane other) {
    return Motor(vec4(self.g0.y) * other.g0.zzxy * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g0.z) * other.g0.yxyz * vec4(-1.0, 1.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, 1.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w) * vec3(-1.0));
}

MultiVector point_plane_geometric_anti_product(Point self, Plane other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(1.0, 0.0) + vec2(self.g0.w) * vec2(other.g0.w) * vec2(1.0, 0.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(1.0, 0.0), vec4(0.0), vec3(0.0) - vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0), vec4(0.0));
}

Scalar point_plane_regressive_product(Point self, Plane other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z + self.g0.w * other.g0.w);
}

Scalar point_plane_anti_wedge(Point self, Plane other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z + self.g0.w * other.g0.w);
}

Scalar point_plane_meet(Point self, Plane other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z + self.g0.w * other.g0.w);
}

AntiScalar point_plane_outer_product(Point self, Plane other) {
    return AntiScalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z + self.g0.w * other.g0.w);
}

AntiScalar point_plane_wedge(Point self, Plane other) {
    return AntiScalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z + self.g0.w * other.g0.w);
}

AntiScalar point_plane_join(Point self, Plane other) {
    return AntiScalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z + self.g0.w * other.g0.w);
}

Line point_plane_inner_product(Point self, Plane other) {
    return Line(vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w) * vec3(-1.0));
}

Line point_plane_inner_anti_product(Point self, Plane other) {
    return Line(vec3(0.0) - vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0));
}

Line point_plane_left_contraction(Point self, Plane other) {
    return Line(vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w) * vec3(-1.0));
}

Line point_plane_right_anti_contraction(Point self, Plane other) {
    return Line(vec3(0.0) - vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0));
}

Flector point_motor_geometric_product(Point self, Motor other) {
    return Flector(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0), vec4(self.g0.x) * vec4(other.g0.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.y) * vec4(other.g0.z, other.g0.w, other.g0.x, other.g1.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.w, other.g1.z) * vec4(1.0, -1.0, 1.0, -1.0) + self.g0.wwwx * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Flector point_motor_geometric_anti_product(Point self, Motor other) {
    return Flector(vec4(self.g0.y) * other.g0.zwxz * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxwy * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g0.w) * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g0.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, 0.0), vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

Point point_motor_regressive_product(Point self, Motor other) {
    return Point(self.g0 * vec4(other.g0.w));
}

Point point_motor_anti_wedge(Point self, Motor other) {
    return Point(self.g0 * vec4(other.g0.w));
}

Point point_motor_meet(Point self, Motor other) {
    return Point(self.g0 * vec4(other.g0.w));
}

Plane point_motor_outer_product(Point self, Motor other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

Plane point_motor_wedge(Point self, Motor other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

Plane point_motor_join(Point self, Motor other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

Flector point_motor_inner_product(Point self, Motor other) {
    return Flector(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0), self.g0.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

Flector point_motor_inner_anti_product(Point self, Motor other) {
    return Flector(self.g0 * vec4(other.g0.w), vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

Flector point_motor_left_contraction(Point self, Motor other) {
    return Flector(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0), self.g0.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

Flector point_motor_right_anti_contraction(Point self, Motor other) {
    return Flector(self.g0 * vec4(other.g0.w), vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

Flector point_rotor_geometric_product(Point self, Rotor other) {
    return Flector(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0), vec4(self.g0.y) * other.g0.zwxz * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g0.z) * other.g0.yxwy * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g0.x) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 0.0));
}

Flector point_rotor_geometric_anti_product(Point self, Rotor other) {
    return Flector(vec4(self.g0.y) * other.g0.zwxz * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxwy * vec4(-1.0, 1.0, 1.0, 0.0) + self.g0.xxxw * other.g0.wzyw * vec4(1.0, -1.0, 1.0, 1.0), vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

Point point_rotor_regressive_product(Point self, Rotor other) {
    return Point(self.g0 * vec4(other.g0.w));
}

Point point_rotor_anti_wedge(Point self, Rotor other) {
    return Point(self.g0 * vec4(other.g0.w));
}

Point point_rotor_meet(Point self, Rotor other) {
    return Point(self.g0 * vec4(other.g0.w));
}

Plane point_rotor_outer_product(Point self, Rotor other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, 1.0, -1.0, 0.0));
}

Plane point_rotor_wedge(Point self, Rotor other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, 1.0, -1.0, 0.0));
}

Plane point_rotor_join(Point self, Rotor other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, 1.0, -1.0, 0.0));
}

Flector point_rotor_inner_product(Point self, Rotor other) {
    return Flector(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0), self.g0.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

Flector point_rotor_inner_anti_product(Point self, Rotor other) {
    return Flector(self.g0 * vec4(other.g0.w), vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

Flector point_rotor_left_contraction(Point self, Rotor other) {
    return Flector(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0), self.g0.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

Flector point_rotor_right_anti_contraction(Point self, Rotor other) {
    return Flector(self.g0 * vec4(other.g0.w), vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

Flector point_translator_geometric_product(Point self, Translator other) {
    return Flector(vec4(self.g0.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, 1.0, -1.0, 0.0), vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Point point_translator_geometric_anti_product(Point self, Translator other) {
    return Point(vec4(self.g0.w) * other.g0 * vec4(-1.0, -1.0, -1.0, 1.0) + self.g0.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

Point point_translator_regressive_product(Point self, Translator other) {
    return Point(self.g0 * vec4(other.g0.w));
}

Point point_translator_anti_wedge(Point self, Translator other) {
    return Point(self.g0 * vec4(other.g0.w));
}

Point point_translator_meet(Point self, Translator other) {
    return Point(self.g0 * vec4(other.g0.w));
}

Plane point_translator_outer_product(Point self, Translator other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

Plane point_translator_wedge(Point self, Translator other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

Plane point_translator_join(Point self, Translator other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

Flector point_translator_inner_product(Point self, Translator other) {
    return Flector(vec4(self.g0.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, 1.0, -1.0, 0.0), self.g0.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

Point point_translator_inner_anti_product(Point self, Translator other) {
    return Point(self.g0 * vec4(other.g0.w));
}

Flector point_translator_left_contraction(Point self, Translator other) {
    return Flector(vec4(self.g0.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, 1.0, -1.0, 0.0), self.g0.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

Point point_translator_right_anti_contraction(Point self, Translator other) {
    return Point(self.g0 * vec4(other.g0.w));
}

Flector point_flector_add(Point self, Flector other) {
    return Flector(self.g0 + other.g0, other.g1);
}

Flector point_flector_sub(Point self, Flector other) {
    return Flector(self.g0 - other.g0, vec4(0.0) - other.g1);
}

MultiVector point_flector_geometric_product(Point self, Flector other) {
    return MultiVector(vec2(self.g0.x) * vec2(other.g0.x, other.g1.x) + vec2(self.g0.y) * vec2(other.g0.y, other.g1.y) + vec2(self.g0.z) * vec2(other.g0.z, other.g1.z) + vec2(self.g0.x, self.g0.w) * vec2(other.g1.x, other.g1.w) * vec2(0.0, 1.0), vec4(0.0), vec3(self.g0.x) * vec3(other.g0.w, other.g1.z, other.g1.y) * vec3(-1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g1.z, other.g0.w, other.g1.x) * vec3(1.0, -1.0, -1.0) + vec3(self.g0.z) * vec3(other.g1.y, other.g1.x, other.g0.w) * vec3(-1.0, 1.0, -1.0) + vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.x) * vec3(other.g1.w, other.g0.z, other.g0.y) * vec3(-1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g0.z, other.g1.w, other.g0.x) * vec3(1.0, -1.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g1.w) * vec3(-1.0, 1.0, -1.0), vec4(0.0));
}

MultiVector point_flector_geometric_anti_product(Point self, Flector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(1.0, 0.0) + vec2(self.g0.w) * vec2(other.g1.w, other.g0.w) * vec2(1.0, -1.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(1.0, 0.0), vec4(0.0), vec3(0.0) - vec3(self.g0.w) * vec3(other.g1.x, other.g1.y, other.g1.z), vec3(self.g0.x) * vec3(other.g0.w, other.g1.z, other.g1.y) * vec3(-1.0, 1.0, -1.0) + vec3(self.g0.y) * vec3(other.g1.z, other.g0.w, other.g1.x) * vec3(-1.0, -1.0, 1.0) + vec3(self.g0.z) * vec3(other.g1.y, other.g1.x, other.g0.w) * vec3(1.0, -1.0, -1.0) + vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(0.0));
}

Scalar point_flector_regressive_product(Point self, Flector other) {
    return Scalar(self.g0.x * other.g1.x + self.g0.y * other.g1.y + self.g0.z * other.g1.z + self.g0.w * other.g1.w);
}

Scalar point_flector_anti_wedge(Point self, Flector other) {
    return Scalar(self.g0.x * other.g1.x + self.g0.y * other.g1.y + self.g0.z * other.g1.z + self.g0.w * other.g1.w);
}

Scalar point_flector_meet(Point self, Flector other) {
    return Scalar(self.g0.x * other.g1.x + self.g0.y * other.g1.y + self.g0.z * other.g1.z + self.g0.w * other.g1.w);
}

Motor point_flector_outer_product(Point self, Flector other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.w, other.g0.w, other.g0.w, other.g1.y) * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.w, other.g0.w, other.g0.w, other.g1.z) * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.w) + vec4(self.g0.x) * vec4(other.g0.w, other.g0.x, other.g0.x, other.g1.x) * vec4(-1.0, 0.0, 0.0, 1.0), vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0));
}

Motor point_flector_wedge(Point self, Flector other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.w, other.g0.w, other.g0.w, other.g1.y) * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.w, other.g0.w, other.g0.w, other.g1.z) * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.w) + vec4(self.g0.x) * vec4(other.g0.w, other.g0.x, other.g0.x, other.g1.x) * vec4(-1.0, 0.0, 0.0, 1.0), vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0));
}

Motor point_flector_join(Point self, Flector other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.w, other.g0.w, other.g0.w, other.g1.y) * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.w, other.g0.w, other.g0.w, other.g1.z) * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.w) + vec4(self.g0.x) * vec4(other.g0.w, other.g0.x, other.g0.x, other.g1.x) * vec4(-1.0, 0.0, 0.0, 1.0), vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0));
}

MultiVector point_flector_inner_product(Point self, Flector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(1.0, 0.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(1.0, 0.0), vec4(0.0), vec3(self.g0.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g1.x, other.g1.z, other.g1.y) * vec3(0.0, -1.0, 1.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.w) * vec3(-1.0), vec4(0.0));
}

Motor point_flector_inner_anti_product(Point self, Flector other) {
    return Motor(vec4(0.0) - vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g0.w), vec3(self.g0.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * vec3(other.g1.x, other.g1.z, other.g1.y) * vec3(0.0, 1.0, -1.0));
}

MultiVector point_flector_left_contraction(Point self, Flector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(1.0, 0.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(1.0, 0.0), vec4(0.0), vec3(self.g0.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g1.x, other.g1.z, other.g1.y) * vec3(0.0, -1.0, 1.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.w) * vec3(-1.0), vec4(0.0));
}

Scalar point_flector_right_contraction(Point self, Flector other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

AntiScalar point_flector_left_anti_contraction(Point self, Flector other) {
    return AntiScalar(0.0 - self.g0.w * other.g0.w);
}

Motor point_flector_right_anti_contraction(Point self, Flector other) {
    return Motor(vec4(0.0) - vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g0.w), vec3(self.g0.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * vec3(other.g1.x, other.g1.z, other.g1.y) * vec3(0.0, 1.0, -1.0));
}

Scalar point_flector_scalar_product(Point self, Flector other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

Scalar point_flector_dot(Point self, Flector other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

AntiScalar point_flector_anti_scalar_product(Point self, Flector other) {
    return AntiScalar(0.0 - self.g0.w * other.g0.w);
}

AntiScalar point_flector_anti_dot(Point self, Flector other) {
    return AntiScalar(0.0 - self.g0.w * other.g0.w);
}

MultiVector point_multi_vector_add(Point self, MultiVector other) {
    return MultiVector(other.g0, self.g0 + other.g1, other.g2, other.g3, other.g4);
}

MultiVector point_multi_vector_sub(Point self, MultiVector other) {
    return MultiVector(vec2(0.0) - other.g0, self.g0 - other.g1, vec3(0.0) - other.g2, vec3(0.0) - other.g3, vec4(0.0) - other.g4);
}

MultiVector point_multi_vector_geometric_product(Point self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * vec2(other.g1.x, other.g4.x) + vec2(self.g0.y) * vec2(other.g1.y, other.g4.y) + vec2(self.g0.z) * vec2(other.g1.z, other.g4.z) + vec2(self.g0.x, self.g0.w) * vec2(other.g4.x, other.g4.w) * vec2(0.0, 1.0), vec4(self.g0.x) * vec4(other.g0.x, other.g3.z, other.g3.y, other.g2.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.y) * vec4(other.g3.z, other.g0.x, other.g3.x, other.g2.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g3.y, other.g3.x, other.g0.x, other.g2.z) * vec4(1.0, -1.0, 1.0, -1.0) + self.g0.xxxw * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0), vec3(self.g0.x) * vec3(other.g1.w, other.g4.z, other.g4.y) * vec3(-1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g4.z, other.g1.w, other.g4.x) * vec3(1.0, -1.0, -1.0) + vec3(self.g0.z) * vec3(other.g4.y, other.g4.x, other.g1.w) * vec3(-1.0, 1.0, -1.0) + vec3(self.g0.w) * vec3(other.g1.x, other.g1.y, other.g1.z), vec3(self.g0.x) * vec3(other.g4.w, other.g1.z, other.g1.y) * vec3(-1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g1.z, other.g4.w, other.g1.x) * vec3(1.0, -1.0, -1.0) + vec3(self.g0.z) * vec3(other.g1.y, other.g1.x, other.g4.w) * vec3(-1.0, 1.0, -1.0), vec4(self.g0.x) * vec4(other.g0.y, other.g2.z, other.g2.y, other.g3.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.y) * vec4(other.g2.z, other.g0.y, other.g2.x, other.g3.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g2.y, other.g2.x, other.g0.y, other.g3.z) * vec4(1.0, -1.0, 1.0, -1.0) + self.g0.wwwx * vec4(other.g3.x, other.g3.y, other.g3.z, other.g3.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

MultiVector point_multi_vector_geometric_anti_product(Point self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g4.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g4.z) * vec2(1.0, 0.0) + vec2(self.g0.w) * vec2(other.g4.w, other.g1.w) * vec2(1.0, -1.0) + vec2(self.g0.x) * vec2(other.g4.x) * vec2(1.0, 0.0), vec4(self.g0.y) * vec4(other.g2.z, other.g0.y, other.g2.x, other.g2.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g0.z) * vec4(other.g2.y, other.g2.x, other.g0.y, other.g2.y) * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g0.w) * vec4(other.g3.x, other.g3.y, other.g3.z, other.g0.y) * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.y, other.g2.z, other.g2.y, other.g0.x) * vec4(1.0, -1.0, 1.0, 0.0), vec3(0.0) - vec3(self.g0.w) * vec3(other.g4.x, other.g4.y, other.g4.z), vec3(self.g0.x) * vec3(other.g1.w, other.g4.z, other.g4.y) * vec3(-1.0, 1.0, -1.0) + vec3(self.g0.y) * vec3(other.g4.z, other.g1.w, other.g4.x) * vec3(-1.0, -1.0, 1.0) + vec3(self.g0.z) * vec3(other.g4.y, other.g4.x, other.g1.w) * vec3(1.0, -1.0, -1.0) + vec3(self.g0.w) * vec3(other.g1.x, other.g1.y, other.g1.z), vec4(self.g0.y) * vec4(other.g2.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g2.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g2.x, other.g2.y, other.g2.z, other.g0.x) * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.x) * vec4(other.g2.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

MultiVector point_multi_vector_regressive_product(Point self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g4.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g4.z) * vec2(1.0, 0.0) + vec2(self.g0.w) * vec2(other.g4.w) * vec2(1.0, 0.0) + vec2(self.g0.x) * vec2(other.g4.x) * vec2(1.0, 0.0), self.g0 * vec4(other.g0.y), vec3(0.0), vec3(0.0), vec4(0.0));
}

MultiVector point_multi_vector_anti_wedge(Point self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g4.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g4.z) * vec2(1.0, 0.0) + vec2(self.g0.w) * vec2(other.g4.w) * vec2(1.0, 0.0) + vec2(self.g0.x) * vec2(other.g4.x) * vec2(1.0, 0.0), self.g0 * vec4(other.g0.y), vec3(0.0), vec3(0.0), vec4(0.0));
}

MultiVector point_multi_vector_meet(Point self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g4.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g4.z) * vec2(1.0, 0.0) + vec2(self.g0.w) * vec2(other.g4.w) * vec2(1.0, 0.0) + vec2(self.g0.x) * vec2(other.g4.x) * vec2(1.0, 0.0), self.g0 * vec4(other.g0.y), vec3(0.0), vec3(0.0), vec4(0.0));
}

MultiVector point_multi_vector_outer_product(Point self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g4.y) * vec2(0.0, 1.0) + vec2(self.g0.z) * vec2(other.g4.z) * vec2(0.0, 1.0) + vec2(self.g0.w) * vec2(other.g4.w) * vec2(0.0, 1.0) + vec2(self.g0.x) * vec2(other.g4.x) * vec2(0.0, 1.0), self.g0 * vec4(other.g0.x), vec3(self.g0.w) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.w) * vec3(-1.0), vec3(self.g0.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g1.x, other.g1.z, other.g1.y) * vec3(0.0, -1.0, 1.0), vec4(self.g0.y) * vec4(other.g2.z, other.g2.z, other.g2.x, other.g3.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g2.y, other.g2.x, other.g2.y, other.g3.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g3.x, other.g3.y, other.g3.z, other.g3.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g2.x, other.g2.z, other.g2.y, other.g3.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

MultiVector point_multi_vector_wedge(Point self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g4.y) * vec2(0.0, 1.0) + vec2(self.g0.z) * vec2(other.g4.z) * vec2(0.0, 1.0) + vec2(self.g0.w) * vec2(other.g4.w) * vec2(0.0, 1.0) + vec2(self.g0.x) * vec2(other.g4.x) * vec2(0.0, 1.0), self.g0 * vec4(other.g0.x), vec3(self.g0.w) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.w) * vec3(-1.0), vec3(self.g0.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g1.x, other.g1.z, other.g1.y) * vec3(0.0, -1.0, 1.0), vec4(self.g0.y) * vec4(other.g2.z, other.g2.z, other.g2.x, other.g3.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g2.y, other.g2.x, other.g2.y, other.g3.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g3.x, other.g3.y, other.g3.z, other.g3.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g2.x, other.g2.z, other.g2.y, other.g3.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

MultiVector point_multi_vector_join(Point self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g4.y) * vec2(0.0, 1.0) + vec2(self.g0.z) * vec2(other.g4.z) * vec2(0.0, 1.0) + vec2(self.g0.w) * vec2(other.g4.w) * vec2(0.0, 1.0) + vec2(self.g0.x) * vec2(other.g4.x) * vec2(0.0, 1.0), self.g0 * vec4(other.g0.x), vec3(self.g0.w) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.w) * vec3(-1.0), vec3(self.g0.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g1.x, other.g1.z, other.g1.y) * vec3(0.0, -1.0, 1.0), vec4(self.g0.y) * vec4(other.g2.z, other.g2.z, other.g2.x, other.g3.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g2.y, other.g2.x, other.g2.y, other.g3.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g3.x, other.g3.y, other.g3.z, other.g3.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g2.x, other.g2.z, other.g2.y, other.g3.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

MultiVector point_multi_vector_inner_product(Point self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(1.0, 0.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(1.0, 0.0), vec4(self.g0.x) * vec4(other.g0.x, other.g3.z, other.g3.y, other.g2.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.y) * vec4(other.g3.z, other.g0.x, other.g3.x, other.g2.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g3.y, other.g3.x, other.g0.x, other.g2.z) * vec4(1.0, -1.0, 1.0, -1.0) + self.g0.xxxw * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0), vec3(self.g0.y) * vec3(other.g4.z, other.g4.z, other.g4.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g4.y, other.g4.x, other.g4.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g4.x, other.g4.z, other.g4.y) * vec3(0.0, -1.0, 1.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g4.w) * vec3(-1.0), self.g0.xyzx * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

MultiVector point_multi_vector_inner_anti_product(Point self, MultiVector other) {
    return MultiVector(vec2(self.g0.x, self.g0.w) * vec2(other.g1.x, other.g1.w) * vec2(0.0, -1.0), self.g0 * vec4(other.g0.y), vec3(0.0) - vec3(self.g0.w) * vec3(other.g4.x, other.g4.y, other.g4.z), vec3(self.g0.y) * vec3(other.g4.z, other.g4.z, other.g4.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * vec3(other.g4.y, other.g4.x, other.g4.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * vec3(other.g4.x, other.g4.z, other.g4.y) * vec3(0.0, 1.0, -1.0), vec4(self.g0.y) * vec4(other.g2.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g2.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g2.x, other.g2.y, other.g2.z, other.g0.x) * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.x) * vec4(other.g2.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

MultiVector point_multi_vector_left_contraction(Point self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(1.0, 0.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(1.0, 0.0), vec4(self.g0.y) * vec4(other.g3.z, other.g3.z, other.g3.x, other.g2.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g3.y, other.g3.x, other.g3.y, other.g2.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.x) * vec4(other.g3.x, other.g3.z, other.g3.y, other.g2.x) * vec4(0.0, 1.0, -1.0, -1.0), vec3(self.g0.y) * vec3(other.g4.z, other.g4.z, other.g4.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g4.y, other.g4.x, other.g4.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g4.x, other.g4.z, other.g4.y) * vec3(0.0, -1.0, 1.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g4.w) * vec3(-1.0), self.g0.xyzx * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

MultiVector point_multi_vector_right_contraction(Point self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(1.0, 0.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(1.0, 0.0), self.g0 * vec4(other.g0.x), vec3(0.0), vec3(0.0), vec4(0.0));
}

MultiVector point_multi_vector_left_anti_contraction(Point self, MultiVector other) {
    return MultiVector(vec2(self.g0.x, self.g0.w) * vec2(other.g1.x, other.g1.w) * vec2(0.0, -1.0), vec4(0.0), vec3(0.0), vec3(0.0), self.g0.xxxw * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

MultiVector point_multi_vector_right_anti_contraction(Point self, MultiVector other) {
    return MultiVector(vec2(self.g0.x, self.g0.w) * vec2(other.g1.x, other.g1.w) * vec2(0.0, -1.0), self.g0 * vec4(other.g0.y), vec3(0.0) - vec3(self.g0.w) * vec3(other.g4.x, other.g4.y, other.g4.z), vec3(self.g0.y) * vec3(other.g4.z, other.g4.z, other.g4.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * vec3(other.g4.y, other.g4.x, other.g4.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * vec3(other.g4.x, other.g4.z, other.g4.y) * vec3(0.0, 1.0, -1.0), vec4(self.g0.y) * vec4(other.g2.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g2.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * vec4(other.g2.x, other.g2.y, other.g2.z, other.g2.x) * vec4(1.0, 1.0, 1.0, -1.0));
}

Scalar point_multi_vector_scalar_product(Point self, MultiVector other) {
    return Scalar(self.g0.x * other.g1.x + self.g0.y * other.g1.y + self.g0.z * other.g1.z);
}

Scalar point_multi_vector_dot(Point self, MultiVector other) {
    return Scalar(self.g0.x * other.g1.x + self.g0.y * other.g1.y + self.g0.z * other.g1.z);
}

AntiScalar point_multi_vector_anti_scalar_product(Point self, MultiVector other) {
    return AntiScalar(0.0 - self.g0.w * other.g1.w);
}

AntiScalar point_multi_vector_anti_dot(Point self, MultiVector other) {
    return AntiScalar(0.0 - self.g0.w * other.g1.w);
}

Line line_scalar_geometric_product(Line self, Scalar other) {
    return Line(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0));
}

Translator line_scalar_geometric_anti_product(Line self, Scalar other) {
    return Translator(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(other.g0) * vec4(1.0, 1.0, 1.0, 0.0));
}

Line line_scalar_outer_product(Line self, Scalar other) {
    return Line(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0));
}

Line line_scalar_wedge(Line self, Scalar other) {
    return Line(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0));
}

Line line_scalar_join(Line self, Scalar other) {
    return Line(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0));
}

Line line_scalar_inner_product(Line self, Scalar other) {
    return Line(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0));
}

Translator line_scalar_inner_anti_product(Line self, Scalar other) {
    return Translator(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(other.g0) * vec4(1.0, 1.0, 1.0, 0.0));
}

Line line_scalar_right_contraction(Line self, Scalar other) {
    return Line(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0));
}

Translator line_scalar_left_anti_contraction(Line self, Scalar other) {
    return Translator(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(other.g0) * vec4(1.0, 1.0, 1.0, 0.0));
}

Motor line_anti_scalar_add(Line self, AntiScalar other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0), self.g1);
}

Motor line_anti_scalar_sub(Line self, AntiScalar other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) - vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0), self.g1);
}

Rotor line_anti_scalar_geometric_product(Line self, AntiScalar other) {
    return Rotor(vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.x) * vec4(other.g0) * vec4(1.0, 1.0, 1.0, 0.0));
}

Line line_anti_scalar_geometric_anti_product(Line self, AntiScalar other) {
    return Line(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0));
}

Line line_anti_scalar_regressive_product(Line self, AntiScalar other) {
    return Line(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0));
}

Line line_anti_scalar_anti_wedge(Line self, AntiScalar other) {
    return Line(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0));
}

Line line_anti_scalar_meet(Line self, AntiScalar other) {
    return Line(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0));
}

Rotor line_anti_scalar_inner_product(Line self, AntiScalar other) {
    return Rotor(vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.x) * vec4(other.g0) * vec4(1.0, 1.0, 1.0, 0.0));
}

Line line_anti_scalar_inner_anti_product(Line self, AntiScalar other) {
    return Line(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0));
}

Rotor line_anti_scalar_left_contraction(Line self, AntiScalar other) {
    return Rotor(vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.x) * vec4(other.g0) * vec4(1.0, 1.0, 1.0, 0.0));
}

Line line_anti_scalar_right_anti_contraction(Line self, AntiScalar other) {
    return Line(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0));
}

Line line_homogeneous_magnitude_geometric_product(Line self, HomogeneousMagnitude other) {
    return Line(vec3(self.g1.x) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g1.y) * vec3(other.g0.y) * vec3(0.0, 1.0, 0.0) + vec3(self.g1.z) * vec3(other.g0.y) * vec3(0.0, 0.0, 1.0) + self.g0 * vec3(other.g0.x), self.g1 * vec3(other.g0.x));
}

Line line_homogeneous_magnitude_geometric_anti_product(Line self, HomogeneousMagnitude other) {
    return Line(self.g0 * vec3(other.g0.y), vec3(self.g1.x) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g1.y) * vec3(other.g0.y) * vec3(0.0, 1.0, 0.0) + vec3(self.g1.z) * vec3(other.g0.y) * vec3(0.0, 0.0, 1.0) + self.g0 * vec3(other.g0.x));
}

Line line_homogeneous_magnitude_regressive_product(Line self, HomogeneousMagnitude other) {
    return Line(self.g0 * vec3(other.g0.y), self.g1 * vec3(other.g0.y));
}

Line line_homogeneous_magnitude_anti_wedge(Line self, HomogeneousMagnitude other) {
    return Line(self.g0 * vec3(other.g0.y), self.g1 * vec3(other.g0.y));
}

Line line_homogeneous_magnitude_meet(Line self, HomogeneousMagnitude other) {
    return Line(self.g0 * vec3(other.g0.y), self.g1 * vec3(other.g0.y));
}

Line line_homogeneous_magnitude_outer_product(Line self, HomogeneousMagnitude other) {
    return Line(self.g0 * vec3(other.g0.x), self.g1 * vec3(other.g0.x));
}

Line line_homogeneous_magnitude_wedge(Line self, HomogeneousMagnitude other) {
    return Line(self.g0 * vec3(other.g0.x), self.g1 * vec3(other.g0.x));
}

Line line_homogeneous_magnitude_join(Line self, HomogeneousMagnitude other) {
    return Line(self.g0 * vec3(other.g0.x), self.g1 * vec3(other.g0.x));
}

Line line_homogeneous_magnitude_inner_product(Line self, HomogeneousMagnitude other) {
    return Line(vec3(self.g1.x) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g1.y) * vec3(other.g0.y) * vec3(0.0, 1.0, 0.0) + vec3(self.g1.z) * vec3(other.g0.y) * vec3(0.0, 0.0, 1.0) + self.g0 * vec3(other.g0.x), self.g1 * vec3(other.g0.x));
}

Line line_homogeneous_magnitude_inner_anti_product(Line self, HomogeneousMagnitude other) {
    return Line(self.g0 * vec3(other.g0.y), vec3(self.g1.x) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g1.y) * vec3(other.g0.y) * vec3(0.0, 1.0, 0.0) + vec3(self.g1.z) * vec3(other.g0.y) * vec3(0.0, 0.0, 1.0) + self.g0 * vec3(other.g0.x));
}

Rotor line_homogeneous_magnitude_left_contraction(Line self, HomogeneousMagnitude other) {
    return Rotor(vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.x) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Line line_homogeneous_magnitude_right_contraction(Line self, HomogeneousMagnitude other) {
    return Line(self.g0 * vec3(other.g0.x), self.g1 * vec3(other.g0.x));
}

Translator line_homogeneous_magnitude_left_anti_contraction(Line self, HomogeneousMagnitude other) {
    return Translator(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Line line_homogeneous_magnitude_right_anti_contraction(Line self, HomogeneousMagnitude other) {
    return Line(self.g0 * vec3(other.g0.y), self.g1 * vec3(other.g0.y));
}

Flector line_point_geometric_product(Line self, Point other) {
    return Flector(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g0.x, self.g1.x, self.g1.x, self.g0.x) * other.g0.xzyx * vec4(0.0, 1.0, -1.0, 1.0), vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.x, self.g0.x, self.g0.x, self.g1.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0));
}

Flector line_point_geometric_anti_product(Line self, Point other) {
    return Flector(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.w) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.w) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.x, self.g0.x, self.g0.x, self.g0.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, 0.0), vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Plane line_point_outer_product(Line self, Point other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.x, self.g0.x, self.g0.x, self.g1.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0));
}

Plane line_point_wedge(Line self, Point other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.x, self.g0.x, self.g0.x, self.g1.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0));
}

Plane line_point_join(Line self, Point other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.x, self.g0.x, self.g0.x, self.g1.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0));
}

Point line_point_inner_product(Line self, Point other) {
    return Point(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g0.x, self.g1.x, self.g1.x, self.g0.x) * other.g0.xzyx * vec4(0.0, 1.0, -1.0, 1.0));
}

Plane line_point_inner_anti_product(Line self, Point other) {
    return Plane(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Point line_point_right_contraction(Line self, Point other) {
    return Point(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g0.x, self.g1.x, self.g1.x, self.g0.x) * other.g0.xzyx * vec4(0.0, 1.0, -1.0, 1.0));
}

Plane line_point_left_anti_contraction(Line self, Point other) {
    return Plane(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Line line_line_add(Line self, Line other) {
    return Line(self.g0 + other.g0, self.g1 + other.g1);
}

Line line_line_sub(Line self, Line other) {
    return Line(self.g0 - other.g0, self.g1 - other.g1);
}

Line line_line_mul(Line self, Line other) {
    return Line(self.g0 * other.g0, self.g1 * other.g1);
}

Line line_line_div(Line self, Line other) {
    return Line(vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(1.0, 1.0, 1.0) / vec3(other.g0.x, other.g0.y, other.g0.z) * vec3(1.0, 1.0, 1.0), vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(1.0, 1.0, 1.0) / vec3(other.g1.x, other.g1.y, other.g1.z) * vec3(1.0, 1.0, 1.0));
}

MultiVector line_line_geometric_product(Line self, Line other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(0.0, -1.0) - vec2(self.g1.x) * vec2(other.g1.x, other.g0.x) - vec2(self.g1.y) * vec2(other.g1.y, other.g0.y) - vec2(self.g1.z) * vec2(other.g1.z, other.g0.z) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(0.0, -1.0), vec4(0.0), vec3(self.g0.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g1.x) * other.g0.zzy * vec3(0.0, 1.0, -1.0) + vec3(self.g1.y) * other.g0.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * other.g0.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * other.g1.xzy * vec3(0.0, 1.0, -1.0), vec3(self.g1.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g1.x) * other.g1.xzy * vec3(0.0, 1.0, -1.0), vec4(0.0));
}

MultiVector line_line_geometric_anti_product(Line self, Line other) {
    return MultiVector(vec2(0.0) - vec2(self.g0.x) * vec2(other.g1.x, other.g0.x) - vec2(self.g0.y) * vec2(other.g1.y, other.g0.y) - vec2(self.g0.z) * vec2(other.g1.z, other.g0.z) + vec2(self.g1.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.y) * other.g0.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * other.g0.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * other.g0.xzy * vec3(0.0, -1.0, 1.0), vec3(self.g0.y) * other.g1.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * other.g1.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.x) * other.g0.zzy * vec3(0.0, -1.0, 1.0) + vec3(self.g1.y) * other.g0.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * other.g0.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * other.g1.xzy * vec3(0.0, -1.0, 1.0), vec4(0.0));
}

Scalar line_line_regressive_product(Line self, Line other) {
    return Scalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

Scalar line_line_anti_wedge(Line self, Line other) {
    return Scalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

Scalar line_line_meet(Line self, Line other) {
    return Scalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

AntiScalar line_line_outer_product(Line self, Line other) {
    return AntiScalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

AntiScalar line_line_wedge(Line self, Line other) {
    return AntiScalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

AntiScalar line_line_join(Line self, Line other) {
    return AntiScalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

Scalar line_line_inner_product(Line self, Line other) {
    return Scalar(0.0 - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z);
}

AntiScalar line_line_inner_anti_product(Line self, Line other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

Scalar line_line_left_contraction(Line self, Line other) {
    return Scalar(0.0 - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z);
}

Scalar line_line_right_contraction(Line self, Line other) {
    return Scalar(0.0 - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z);
}

AntiScalar line_line_left_anti_contraction(Line self, Line other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

AntiScalar line_line_right_anti_contraction(Line self, Line other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

Scalar line_line_scalar_product(Line self, Line other) {
    return Scalar(0.0 - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z);
}

Scalar line_line_dot(Line self, Line other) {
    return Scalar(0.0 - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z);
}

AntiScalar line_line_anti_scalar_product(Line self, Line other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

AntiScalar line_line_anti_dot(Line self, Line other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

Flector line_plane_geometric_product(Line self, Plane other) {
    return Flector(vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0), vec4(self.g1.x) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * other.g0.wwwx * vec4(-1.0, -1.0, -1.0, 0.0));
}

Flector line_plane_geometric_anti_product(Line self, Plane other) {
    return Flector(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x, self.g1.x, self.g1.x, self.g0.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0), vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x, self.g0.x, self.g0.x, self.g1.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, -1.0));
}

Point line_plane_regressive_product(Line self, Plane other) {
    return Point(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x, self.g1.x, self.g1.x, self.g0.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0));
}

Point line_plane_anti_wedge(Line self, Plane other) {
    return Point(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x, self.g1.x, self.g1.x, self.g0.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0));
}

Point line_plane_meet(Line self, Plane other) {
    return Point(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x, self.g1.x, self.g1.x, self.g0.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0));
}

Point line_plane_inner_product(Line self, Plane other) {
    return Point(vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Plane line_plane_inner_anti_product(Line self, Plane other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x, self.g0.x, self.g0.x, self.g1.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, -1.0));
}

Point line_plane_left_contraction(Line self, Plane other) {
    return Point(vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Plane line_plane_right_anti_contraction(Line self, Plane other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x, self.g0.x, self.g0.x, self.g1.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, -1.0));
}

Motor line_motor_add(Line self, Motor other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + other.g0, self.g1 + other.g1);
}

Motor line_motor_sub(Line self, Motor other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) - other.g0, self.g1 - other.g1);
}

MultiVector line_motor_geometric_product(Line self, Motor other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(0.0, -1.0) - vec2(self.g1.x) * vec2(other.g1.x, other.g0.x) - vec2(self.g1.y) * vec2(other.g1.y, other.g0.y) - vec2(self.g1.z) * vec2(other.g1.z, other.g0.z) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(0.0, -1.0), vec4(0.0), vec3(self.g0.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g1.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.y) * vec3(other.g0.z, other.g0.w, other.g0.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.w) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.x) * other.g1.xzy * vec3(0.0, 1.0, -1.0), vec3(self.g1.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g1.x) * other.g1.xzy * vec3(0.0, 1.0, -1.0), vec4(0.0));
}

MultiVector line_motor_geometric_anti_product(Line self, Motor other) {
    return MultiVector(vec2(0.0) - vec2(self.g0.x) * vec2(other.g1.x, other.g0.x) - vec2(self.g0.y) * vec2(other.g1.y, other.g0.y) - vec2(self.g0.z) * vec2(other.g1.z, other.g0.z) + vec2(self.g1.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g0.z, other.g0.w, other.g0.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.w) * vec3(-1.0, 1.0, 1.0), vec3(self.g0.y) * other.g1.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * other.g1.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g0.z, other.g0.w, other.g0.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.w) * vec3(-1.0, 1.0, 1.0) + vec3(self.g0.x) * other.g1.xzy * vec3(0.0, -1.0, 1.0), vec4(0.0));
}

MultiVector line_motor_regressive_product(Line self, Motor other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g1.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(-1.0, 0.0), vec4(0.0), self.g0 * vec3(other.g0.w), self.g1 * vec3(other.g0.w), vec4(0.0));
}

MultiVector line_motor_anti_wedge(Line self, Motor other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g1.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(-1.0, 0.0), vec4(0.0), self.g0 * vec3(other.g0.w), self.g1 * vec3(other.g0.w), vec4(0.0));
}

MultiVector line_motor_meet(Line self, Motor other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g1.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(-1.0, 0.0), vec4(0.0), self.g0 * vec3(other.g0.w), self.g1 * vec3(other.g0.w), vec4(0.0));
}

AntiScalar line_motor_outer_product(Line self, Motor other) {
    return AntiScalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

AntiScalar line_motor_wedge(Line self, Motor other) {
    return AntiScalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

AntiScalar line_motor_join(Line self, Motor other) {
    return AntiScalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

MultiVector line_motor_inner_product(Line self, Motor other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g1.x) * vec2(-1.0, 0.0), vec4(0.0), self.g1 * vec3(other.g0.w), vec3(0.0), vec4(0.0));
}

Motor line_motor_inner_anti_product(Line self, Motor other) {
    return Motor(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0), self.g1 * vec3(other.g0.w));
}

MultiVector line_motor_left_contraction(Line self, Motor other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g1.x) * vec2(-1.0, 0.0), vec4(0.0), self.g1 * vec3(other.g0.w), vec3(0.0), vec4(0.0));
}

Scalar line_motor_right_contraction(Line self, Motor other) {
    return Scalar(0.0 - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z);
}

AntiScalar line_motor_left_anti_contraction(Line self, Motor other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

Motor line_motor_right_anti_contraction(Line self, Motor other) {
    return Motor(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0), self.g1 * vec3(other.g0.w));
}

Scalar line_motor_scalar_product(Line self, Motor other) {
    return Scalar(0.0 - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z);
}

Scalar line_motor_dot(Line self, Motor other) {
    return Scalar(0.0 - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z);
}

AntiScalar line_motor_anti_scalar_product(Line self, Motor other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

AntiScalar line_motor_anti_dot(Line self, Motor other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

Motor line_rotor_add(Line self, Rotor other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + other.g0, self.g1);
}

Motor line_rotor_sub(Line self, Rotor other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) - other.g0, self.g1);
}

Rotor line_rotor_geometric_product(Line self, Rotor other) {
    return Rotor(vec4(self.g1.x) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * other.g0.zwxy * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * other.g0.yxwz * vec4(1.0, -1.0, 1.0, -1.0));
}

MultiVector line_rotor_geometric_anti_product(Line self, Rotor other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g1.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g1.x, self.g0.x) * vec2(other.g0.x) * vec2(-1.0), vec4(0.0), vec3(self.g0.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g0.z, other.g0.w, other.g0.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.w) * vec3(-1.0, 1.0, 1.0), vec3(self.g1.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g0.z, other.g0.w, other.g0.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.w) * vec3(-1.0, 1.0, 1.0), vec4(0.0));
}

MultiVector line_rotor_regressive_product(Line self, Rotor other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(-1.0, 0.0), vec4(0.0), self.g0 * vec3(other.g0.w), self.g1 * vec3(other.g0.w), vec4(0.0));
}

MultiVector line_rotor_anti_wedge(Line self, Rotor other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(-1.0, 0.0), vec4(0.0), self.g0 * vec3(other.g0.w), self.g1 * vec3(other.g0.w), vec4(0.0));
}

MultiVector line_rotor_meet(Line self, Rotor other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(-1.0, 0.0), vec4(0.0), self.g0 * vec3(other.g0.w), self.g1 * vec3(other.g0.w), vec4(0.0));
}

AntiScalar line_rotor_outer_product(Line self, Rotor other) {
    return AntiScalar(0.0 - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

AntiScalar line_rotor_wedge(Line self, Rotor other) {
    return AntiScalar(0.0 - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

AntiScalar line_rotor_join(Line self, Rotor other) {
    return AntiScalar(0.0 - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

Rotor line_rotor_inner_product(Line self, Rotor other) {
    return Rotor(vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.x) * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

Motor line_rotor_inner_anti_product(Line self, Rotor other) {
    return Motor(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0), self.g1 * vec3(other.g0.w));
}

Rotor line_rotor_left_contraction(Line self, Rotor other) {
    return Rotor(vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.x) * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

AntiScalar line_rotor_left_anti_contraction(Line self, Rotor other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

Motor line_rotor_right_anti_contraction(Line self, Rotor other) {
    return Motor(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0), self.g1 * vec3(other.g0.w));
}

AntiScalar line_rotor_anti_scalar_product(Line self, Rotor other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

AntiScalar line_rotor_anti_dot(Line self, Rotor other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

Motor line_translator_add(Line self, Translator other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0), self.g1 + vec3(other.g0.x, other.g0.y, other.g0.z));
}

Motor line_translator_sub(Line self, Translator other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) - other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0), self.g1 - vec3(other.g0.x, other.g0.y, other.g0.z));
}

MultiVector line_translator_geometric_product(Line self, Translator other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g1.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g1.x, self.g0.x) * vec2(other.g0.x) * vec2(-1.0), vec4(0.0), vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g1.y) * vec3(other.g0.w) * vec3(0.0, 1.0, 0.0) + vec3(self.g1.z) * vec3(other.g0.w) * vec3(0.0, 0.0, 1.0) + vec3(self.g1.x, self.g0.x, self.g0.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, 1.0, -1.0), vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g1.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0), vec4(0.0));
}

MultiVector line_translator_geometric_anti_product(Line self, Translator other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(-1.0, 0.0), vec4(0.0), self.g0 * vec3(other.g0.w), vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.y) * vec3(other.g0.w) * vec3(0.0, 1.0, 0.0) + vec3(self.g1.z) * vec3(other.g0.w) * vec3(0.0, 0.0, 1.0) + vec3(self.g1.x, self.g0.x, self.g0.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, -1.0, 1.0), vec4(0.0));
}

MultiVector line_translator_regressive_product(Line self, Translator other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(-1.0, 0.0), vec4(0.0), self.g0 * vec3(other.g0.w), self.g1 * vec3(other.g0.w), vec4(0.0));
}

MultiVector line_translator_anti_wedge(Line self, Translator other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(-1.0, 0.0), vec4(0.0), self.g0 * vec3(other.g0.w), self.g1 * vec3(other.g0.w), vec4(0.0));
}

MultiVector line_translator_meet(Line self, Translator other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(-1.0, 0.0), vec4(0.0), self.g0 * vec3(other.g0.w), self.g1 * vec3(other.g0.w), vec4(0.0));
}

AntiScalar line_translator_outer_product(Line self, Translator other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

AntiScalar line_translator_wedge(Line self, Translator other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

AntiScalar line_translator_join(Line self, Translator other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

MultiVector line_translator_inner_product(Line self, Translator other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(-1.0, 0.0), vec4(0.0), self.g1 * vec3(other.g0.w), vec3(0.0), vec4(0.0));
}

Line line_translator_inner_anti_product(Line self, Translator other) {
    return Line(self.g0 * vec3(other.g0.w), self.g1 * vec3(other.g0.w));
}

MultiVector line_translator_left_contraction(Line self, Translator other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(-1.0, 0.0), vec4(0.0), self.g1 * vec3(other.g0.w), vec3(0.0), vec4(0.0));
}

Scalar line_translator_right_contraction(Line self, Translator other) {
    return Scalar(0.0 - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

Line line_translator_right_anti_contraction(Line self, Translator other) {
    return Line(self.g0 * vec3(other.g0.w), self.g1 * vec3(other.g0.w));
}

Scalar line_translator_scalar_product(Line self, Translator other) {
    return Scalar(0.0 - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

Scalar line_translator_dot(Line self, Translator other) {
    return Scalar(0.0 - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

Flector line_flector_geometric_product(Line self, Flector other) {
    return Flector(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.z, other.g1.w, other.g0.x, other.g1.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g1.w, other.g1.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0), vec4(self.g0.y) * vec4(other.g0.z, other.g1.w, other.g0.x, other.g0.z) * vec4(1.0, -1.0, -1.0, 0.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g1.w, other.g0.y) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.x) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4(-1.0, -1.0, 1.0, 0.0));
}

Flector line_flector_geometric_anti_product(Line self, Flector other) {
    return Flector(vec4(self.g0.x) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g0.z, other.g1.w, other.g0.x, other.g1.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g1.w, other.g1.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.y) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g1.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g1.y) * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, -1.0, 1.0, 0.0), vec4(self.g0.x) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

Point line_flector_regressive_product(Line self, Flector other) {
    return Point(vec4(self.g0.y) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.y) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g1.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x, self.g1.x, self.g1.x, self.g0.x) * other.g1.wzyx * vec4(1.0, -1.0, 1.0, -1.0));
}

Point line_flector_anti_wedge(Line self, Flector other) {
    return Point(vec4(self.g0.y) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.y) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g1.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x, self.g1.x, self.g1.x, self.g0.x) * other.g1.wzyx * vec4(1.0, -1.0, 1.0, -1.0));
}

Point line_flector_meet(Line self, Flector other) {
    return Point(vec4(self.g0.y) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.y) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g1.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x, self.g1.x, self.g1.x, self.g0.x) * other.g1.wzyx * vec4(1.0, -1.0, 1.0, -1.0));
}

Plane line_flector_outer_product(Line self, Flector other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.x, self.g0.x, self.g0.x, self.g1.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0));
}

Plane line_flector_wedge(Line self, Flector other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.x, self.g0.x, self.g0.x, self.g1.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0));
}

Plane line_flector_join(Line self, Flector other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.x, self.g0.x, self.g0.x, self.g1.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0));
}

Point line_flector_inner_product(Line self, Flector other) {
    return Point(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.z, other.g1.w, other.g0.x, other.g1.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g1.w, other.g1.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0));
}

Plane line_flector_inner_anti_product(Line self, Flector other) {
    return Plane(vec4(self.g0.x) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

Point line_flector_left_contraction(Line self, Flector other) {
    return Point(vec4(self.g1.y) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.x) * other.g1.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Point line_flector_right_contraction(Line self, Flector other) {
    return Point(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g0.x, self.g1.x, self.g1.x, self.g0.x) * other.g0.xzyx * vec4(0.0, 1.0, -1.0, 1.0));
}

Plane line_flector_left_anti_contraction(Line self, Flector other) {
    return Plane(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Plane line_flector_right_anti_contraction(Line self, Flector other) {
    return Plane(vec4(self.g0.y) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g1.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x, self.g0.x, self.g0.x, self.g1.x) * other.g1.xzyx * vec4(0.0, -1.0, 1.0, -1.0));
}

MultiVector line_multi_vector_add(Line self, MultiVector other) {
    return MultiVector(other.g0, other.g1, self.g0 + other.g2, self.g1 + other.g3, other.g4);
}

MultiVector line_multi_vector_sub(Line self, MultiVector other) {
    return MultiVector(vec2(0.0) - other.g0, vec4(0.0) - other.g1, self.g0 - other.g2, self.g1 - other.g3, vec4(0.0) - other.g4);
}

MultiVector line_multi_vector_geometric_product(Line self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g3.y) * vec2(0.0, -1.0) + vec2(self.g0.z) * vec2(other.g3.z) * vec2(0.0, -1.0) - vec2(self.g1.x) * vec2(other.g3.x, other.g2.x) - vec2(self.g1.y) * vec2(other.g3.y, other.g2.y) - vec2(self.g1.z) * vec2(other.g3.z, other.g2.z) + vec2(self.g0.x) * vec2(other.g3.x) * vec2(0.0, -1.0), vec4(self.g0.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g4.w, other.g1.z, other.g1.y, other.g4.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g1.z, other.g4.w, other.g1.x, other.g4.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g4.w, other.g4.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, 1.0), vec3(self.g0.x) * vec3(other.g0.x, other.g3.z, other.g3.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.y) * vec3(other.g3.z, other.g0.x, other.g3.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g0.z) * vec3(other.g3.y, other.g3.x, other.g0.x) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.x) * vec3(other.g0.y, other.g2.z, other.g2.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.y) * vec3(other.g2.z, other.g0.y, other.g2.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g1.z) * vec3(other.g2.y, other.g2.x, other.g0.y) * vec3(1.0, -1.0, 1.0), vec3(self.g1.x) * vec3(other.g0.x, other.g3.z, other.g3.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.y) * vec3(other.g3.z, other.g0.x, other.g3.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g1.z) * vec3(other.g3.y, other.g3.x, other.g0.x) * vec3(1.0, -1.0, 1.0), vec4(self.g0.y) * vec4(other.g1.z, other.g4.w, other.g1.x, other.g1.z) * vec4(1.0, -1.0, -1.0, 0.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g4.w, other.g1.y) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g1.x) * vec4(other.g1.w, other.g4.z, other.g4.y, other.g1.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g4.z, other.g1.w, other.g4.x, other.g1.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g4.y, other.g4.x, other.g1.w, other.g1.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.x) * vec4(other.g4.w, other.g1.z, other.g1.y, other.g4.x) * vec4(-1.0, -1.0, 1.0, 0.0));
}

MultiVector line_multi_vector_geometric_anti_product(Line self, MultiVector other) {
    return MultiVector(vec2(0.0) - vec2(self.g0.x) * vec2(other.g3.x, other.g2.x) - vec2(self.g0.y) * vec2(other.g3.y, other.g2.y) - vec2(self.g0.z) * vec2(other.g3.z, other.g2.z) + vec2(self.g1.y) * vec2(other.g2.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g2.z) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g2.x) * vec2(-1.0, 0.0), vec4(self.g0.x) * vec4(other.g4.w, other.g1.z, other.g1.y, other.g4.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g1.z, other.g4.w, other.g1.x, other.g4.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g4.w, other.g4.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.y) * vec4(other.g4.z, other.g1.w, other.g4.x, other.g4.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g1.z) * vec4(other.g4.y, other.g4.x, other.g1.w, other.g4.y) * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g1.w, other.g4.z, other.g4.y, other.g1.x) * vec4(1.0, -1.0, 1.0, 0.0), vec3(self.g0.x) * vec3(other.g0.y, other.g2.z, other.g2.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g2.z, other.g0.y, other.g2.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g2.y, other.g2.x, other.g0.y) * vec3(-1.0, 1.0, 1.0), vec3(self.g0.x) * vec3(other.g0.x, other.g3.z, other.g3.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g3.z, other.g0.x, other.g3.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g3.y, other.g3.x, other.g0.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g1.x) * vec3(other.g0.y, other.g2.z, other.g2.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g2.z, other.g0.y, other.g2.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.z) * vec3(other.g2.y, other.g2.x, other.g0.y) * vec3(-1.0, 1.0, 1.0), vec4(self.g0.x) * vec4(other.g1.w, other.g4.z, other.g4.y, other.g1.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g4.z, other.g1.w, other.g4.x, other.g1.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g4.y, other.g4.x, other.g1.w, other.g1.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.y) * vec4(other.g4.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g4.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g4.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

MultiVector line_multi_vector_regressive_product(Line self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g3.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g3.z) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g2.x) * vec2(-1.0, 0.0) + vec2(self.g1.y) * vec2(other.g2.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g2.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g3.x) * vec2(-1.0, 0.0), vec4(self.g0.y) * other.g4.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g4.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.y) * other.g4.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g4.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x, self.g1.x, self.g1.x, self.g0.x) * other.g4.wzyx * vec4(1.0, -1.0, 1.0, -1.0), self.g0 * vec3(other.g0.y), self.g1 * vec3(other.g0.y), vec4(0.0));
}

MultiVector line_multi_vector_anti_wedge(Line self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g3.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g3.z) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g2.x) * vec2(-1.0, 0.0) + vec2(self.g1.y) * vec2(other.g2.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g2.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g3.x) * vec2(-1.0, 0.0), vec4(self.g0.y) * other.g4.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g4.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.y) * other.g4.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g4.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x, self.g1.x, self.g1.x, self.g0.x) * other.g4.wzyx * vec4(1.0, -1.0, 1.0, -1.0), self.g0 * vec3(other.g0.y), self.g1 * vec3(other.g0.y), vec4(0.0));
}

MultiVector line_multi_vector_meet(Line self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g3.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g3.z) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g2.x) * vec2(-1.0, 0.0) + vec2(self.g1.y) * vec2(other.g2.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g2.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g3.x) * vec2(-1.0, 0.0), vec4(self.g0.y) * other.g4.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g4.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.y) * other.g4.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g4.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x, self.g1.x, self.g1.x, self.g0.x) * other.g4.wzyx * vec4(1.0, -1.0, 1.0, -1.0), self.g0 * vec3(other.g0.y), self.g1 * vec3(other.g0.y), vec4(0.0));
}

MultiVector line_multi_vector_outer_product(Line self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g3.y) * vec2(0.0, -1.0) + vec2(self.g0.z) * vec2(other.g3.z) * vec2(0.0, -1.0) + vec2(self.g1.x) * vec2(other.g2.x) * vec2(0.0, -1.0) + vec2(self.g1.y) * vec2(other.g2.y) * vec2(0.0, -1.0) + vec2(self.g1.z) * vec2(other.g2.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g3.x) * vec2(0.0, -1.0), vec4(0.0), self.g0 * vec3(other.g0.x), self.g1 * vec3(other.g0.x), vec4(self.g0.y) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g1.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.y) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.x, self.g0.x, self.g0.x, self.g1.x) * other.g1.wzyx * vec4(1.0, -1.0, 1.0, -1.0));
}

MultiVector line_multi_vector_wedge(Line self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g3.y) * vec2(0.0, -1.0) + vec2(self.g0.z) * vec2(other.g3.z) * vec2(0.0, -1.0) + vec2(self.g1.x) * vec2(other.g2.x) * vec2(0.0, -1.0) + vec2(self.g1.y) * vec2(other.g2.y) * vec2(0.0, -1.0) + vec2(self.g1.z) * vec2(other.g2.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g3.x) * vec2(0.0, -1.0), vec4(0.0), self.g0 * vec3(other.g0.x), self.g1 * vec3(other.g0.x), vec4(self.g0.y) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g1.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.y) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.x, self.g0.x, self.g0.x, self.g1.x) * other.g1.wzyx * vec4(1.0, -1.0, 1.0, -1.0));
}

MultiVector line_multi_vector_join(Line self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g3.y) * vec2(0.0, -1.0) + vec2(self.g0.z) * vec2(other.g3.z) * vec2(0.0, -1.0) + vec2(self.g1.x) * vec2(other.g2.x) * vec2(0.0, -1.0) + vec2(self.g1.y) * vec2(other.g2.y) * vec2(0.0, -1.0) + vec2(self.g1.z) * vec2(other.g2.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g3.x) * vec2(0.0, -1.0), vec4(0.0), self.g0 * vec3(other.g0.x), self.g1 * vec3(other.g0.x), vec4(self.g0.y) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g1.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.y) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.x, self.g0.x, self.g0.x, self.g1.x) * other.g1.wzyx * vec4(1.0, -1.0, 1.0, -1.0));
}

MultiVector line_multi_vector_inner_product(Line self, MultiVector other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g3.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g3.z) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g3.x) * vec2(-1.0, 0.0), vec4(self.g0.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g4.w, other.g1.z, other.g1.y, other.g4.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g1.z, other.g4.w, other.g1.x, other.g4.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g4.w, other.g4.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, 1.0), vec3(self.g1.x) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g1.y) * vec3(other.g0.y) * vec3(0.0, 1.0, 0.0) + vec3(self.g1.z) * vec3(other.g0.y) * vec3(0.0, 0.0, 1.0) + self.g0 * vec3(other.g0.x), self.g1 * vec3(other.g0.x), vec4(0.0));
}

MultiVector line_multi_vector_inner_anti_product(Line self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g2.y) * vec2(0.0, -1.0) + vec2(self.g0.z) * vec2(other.g2.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g2.x) * vec2(0.0, -1.0), vec4(0.0), self.g0 * vec3(other.g0.y), vec3(self.g1.x) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g1.y) * vec3(other.g0.y) * vec3(0.0, 1.0, 0.0) + vec3(self.g1.z) * vec3(other.g0.y) * vec3(0.0, 0.0, 1.0) + self.g0 * vec3(other.g0.x), vec4(self.g0.x) * vec4(other.g1.w, other.g4.z, other.g4.y, other.g1.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g4.z, other.g1.w, other.g4.x, other.g1.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g4.y, other.g4.x, other.g1.w, other.g1.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.y) * vec4(other.g4.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g4.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g4.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

MultiVector line_multi_vector_left_contraction(Line self, MultiVector other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g3.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g3.z) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g3.x) * vec2(-1.0, 0.0), vec4(self.g1.y) * other.g4.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g4.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.x) * other.g4.wxxx * vec4(1.0, 0.0, 0.0, -1.0), self.g1 * vec3(other.g0.y), vec3(0.0), vec4(0.0));
}

MultiVector line_multi_vector_right_contraction(Line self, MultiVector other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g3.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g3.z) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g3.x) * vec2(-1.0, 0.0), vec4(self.g0.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.y) * other.g1.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g1.z) * other.g1.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g0.x, self.g1.x, self.g1.x, self.g0.x) * other.g1.xzyx * vec4(0.0, 1.0, -1.0, 1.0), self.g0 * vec3(other.g0.x), self.g1 * vec3(other.g0.x), vec4(0.0));
}

MultiVector line_multi_vector_left_anti_contraction(Line self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g2.y) * vec2(0.0, -1.0) + vec2(self.g0.z) * vec2(other.g2.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g2.x) * vec2(0.0, -1.0), vec4(0.0), vec3(0.0), self.g0 * vec3(other.g0.x), vec4(self.g0.y) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g1.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

MultiVector line_multi_vector_right_anti_contraction(Line self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g2.y) * vec2(0.0, -1.0) + vec2(self.g0.z) * vec2(other.g2.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g2.x) * vec2(0.0, -1.0), vec4(0.0), self.g0 * vec3(other.g0.y), self.g1 * vec3(other.g0.y), vec4(self.g0.y) * other.g4.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g4.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g4.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g4.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x, self.g0.x, self.g0.x, self.g1.x) * other.g4.xzyx * vec4(0.0, -1.0, 1.0, -1.0));
}

Scalar line_multi_vector_scalar_product(Line self, MultiVector other) {
    return Scalar(0.0 - self.g1.x * other.g3.x - self.g1.y * other.g3.y - self.g1.z * other.g3.z);
}

Scalar line_multi_vector_dot(Line self, MultiVector other) {
    return Scalar(0.0 - self.g1.x * other.g3.x - self.g1.y * other.g3.y - self.g1.z * other.g3.z);
}

AntiScalar line_multi_vector_anti_scalar_product(Line self, MultiVector other) {
    return AntiScalar(0.0 - self.g0.x * other.g2.x - self.g0.y * other.g2.y - self.g0.z * other.g2.z);
}

AntiScalar line_multi_vector_anti_dot(Line self, MultiVector other) {
    return AntiScalar(0.0 - self.g0.x * other.g2.x - self.g0.y * other.g2.y - self.g0.z * other.g2.z);
}

Plane plane_scalar_geometric_product(Plane self, Scalar other) {
    return Plane(self.g0 * vec4(other.g0));
}

Point plane_scalar_geometric_anti_product(Plane self, Scalar other) {
    return Point(self.g0.xyzx * vec4(other.g0) * vec4(-1.0, -1.0, -1.0, 0.0));
}

Plane plane_scalar_outer_product(Plane self, Scalar other) {
    return Plane(self.g0 * vec4(other.g0));
}

Plane plane_scalar_wedge(Plane self, Scalar other) {
    return Plane(self.g0 * vec4(other.g0));
}

Plane plane_scalar_join(Plane self, Scalar other) {
    return Plane(self.g0 * vec4(other.g0));
}

Plane plane_scalar_inner_product(Plane self, Scalar other) {
    return Plane(self.g0 * vec4(other.g0));
}

Point plane_scalar_inner_anti_product(Plane self, Scalar other) {
    return Point(self.g0.xyzx * vec4(other.g0) * vec4(-1.0, -1.0, -1.0, 0.0));
}

Plane plane_scalar_right_contraction(Plane self, Scalar other) {
    return Plane(self.g0 * vec4(other.g0));
}

Point plane_scalar_left_anti_contraction(Plane self, Scalar other) {
    return Point(self.g0.xyzx * vec4(other.g0) * vec4(-1.0, -1.0, -1.0, 0.0));
}

Point plane_anti_scalar_geometric_product(Plane self, AntiScalar other) {
    return Point(self.g0.xxxw * vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0));
}

Plane plane_anti_scalar_geometric_anti_product(Plane self, AntiScalar other) {
    return Plane(self.g0 * vec4(other.g0));
}

Plane plane_anti_scalar_regressive_product(Plane self, AntiScalar other) {
    return Plane(self.g0 * vec4(other.g0));
}

Plane plane_anti_scalar_anti_wedge(Plane self, AntiScalar other) {
    return Plane(self.g0 * vec4(other.g0));
}

Plane plane_anti_scalar_meet(Plane self, AntiScalar other) {
    return Plane(self.g0 * vec4(other.g0));
}

Point plane_anti_scalar_inner_product(Plane self, AntiScalar other) {
    return Point(self.g0.xxxw * vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0));
}

Plane plane_anti_scalar_inner_anti_product(Plane self, AntiScalar other) {
    return Plane(self.g0 * vec4(other.g0));
}

Point plane_anti_scalar_left_contraction(Plane self, AntiScalar other) {
    return Point(self.g0.xxxw * vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0));
}

Plane plane_anti_scalar_right_anti_contraction(Plane self, AntiScalar other) {
    return Plane(self.g0 * vec4(other.g0));
}

Flector plane_homogeneous_magnitude_geometric_product(Plane self, HomogeneousMagnitude other) {
    return Flector(self.g0.xxxw * vec4(other.g0.x, other.g0.x, other.g0.x, other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0), self.g0 * vec4(other.g0.x));
}

Flector plane_homogeneous_magnitude_geometric_anti_product(Plane self, HomogeneousMagnitude other) {
    return Flector(self.g0.xyzx * vec4(other.g0.x) * vec4(-1.0, -1.0, -1.0, 0.0), self.g0 * vec4(other.g0.y));
}

Plane plane_homogeneous_magnitude_regressive_product(Plane self, HomogeneousMagnitude other) {
    return Plane(self.g0 * vec4(other.g0.y));
}

Plane plane_homogeneous_magnitude_anti_wedge(Plane self, HomogeneousMagnitude other) {
    return Plane(self.g0 * vec4(other.g0.y));
}

Plane plane_homogeneous_magnitude_meet(Plane self, HomogeneousMagnitude other) {
    return Plane(self.g0 * vec4(other.g0.y));
}

Plane plane_homogeneous_magnitude_outer_product(Plane self, HomogeneousMagnitude other) {
    return Plane(self.g0 * vec4(other.g0.x));
}

Plane plane_homogeneous_magnitude_wedge(Plane self, HomogeneousMagnitude other) {
    return Plane(self.g0 * vec4(other.g0.x));
}

Plane plane_homogeneous_magnitude_join(Plane self, HomogeneousMagnitude other) {
    return Plane(self.g0 * vec4(other.g0.x));
}

Flector plane_homogeneous_magnitude_inner_product(Plane self, HomogeneousMagnitude other) {
    return Flector(self.g0.xxxw * vec4(other.g0.x, other.g0.x, other.g0.x, other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0), self.g0 * vec4(other.g0.x));
}

Flector plane_homogeneous_magnitude_inner_anti_product(Plane self, HomogeneousMagnitude other) {
    return Flector(self.g0.xyzx * vec4(other.g0.x) * vec4(-1.0, -1.0, -1.0, 0.0), self.g0 * vec4(other.g0.y));
}

Point plane_homogeneous_magnitude_left_contraction(Plane self, HomogeneousMagnitude other) {
    return Point(self.g0.xxxw * vec4(other.g0.x, other.g0.x, other.g0.x, other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0));
}

Plane plane_homogeneous_magnitude_right_contraction(Plane self, HomogeneousMagnitude other) {
    return Plane(self.g0 * vec4(other.g0.x));
}

Point plane_homogeneous_magnitude_left_anti_contraction(Plane self, HomogeneousMagnitude other) {
    return Point(self.g0.xyzx * vec4(other.g0.x) * vec4(-1.0, -1.0, -1.0, 0.0));
}

Plane plane_homogeneous_magnitude_right_anti_contraction(Plane self, HomogeneousMagnitude other) {
    return Plane(self.g0 * vec4(other.g0.y));
}

Flector plane_point_add(Plane self, Point other) {
    return Flector(other.g0, self.g0);
}

Flector plane_point_sub(Plane self, Point other) {
    return Flector(vec4(0.0) - other.g0, self.g0);
}

Motor plane_point_geometric_product(Plane self, Point other) {
    return Motor(vec4(self.g0.y) * other.g0.zzxy * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.yxyz * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, 1.0, -1.0, -1.0), vec3(0.0) - vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z));
}

MultiVector plane_point_geometric_anti_product(Plane self, Point other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.w) * vec2(other.g0.w) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w) * vec3(-1.0), vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0), vec4(0.0));
}

Scalar plane_point_regressive_product(Plane self, Point other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

Scalar plane_point_anti_wedge(Plane self, Point other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

Scalar plane_point_meet(Plane self, Point other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

AntiScalar plane_point_outer_product(Plane self, Point other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

AntiScalar plane_point_wedge(Plane self, Point other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

AntiScalar plane_point_join(Plane self, Point other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

Line plane_point_inner_product(Plane self, Point other) {
    return Line(vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z));
}

Line plane_point_inner_anti_product(Plane self, Point other) {
    return Line(vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w) * vec3(-1.0), vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0));
}

Line plane_point_right_contraction(Plane self, Point other) {
    return Line(vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z));
}

Line plane_point_left_anti_contraction(Plane self, Point other) {
    return Line(vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w) * vec3(-1.0), vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0));
}

Flector plane_line_geometric_product(Plane self, Line other) {
    return Flector(vec4(self.g0.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, -1.0), vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.z) * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.y) * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g1.x) * vec4(0.0, 1.0, -1.0, 0.0));
}

Flector plane_line_geometric_anti_product(Plane self, Line other) {
    return Flector(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0), vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(-1.0, 1.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, -1.0, 1.0, 1.0));
}

Point plane_line_regressive_product(Plane self, Line other) {
    return Point(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

Point plane_line_anti_wedge(Plane self, Line other) {
    return Point(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

Point plane_line_meet(Plane self, Line other) {
    return Point(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

Point plane_line_inner_product(Plane self, Line other) {
    return Point(vec4(self.g0.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, -1.0));
}

Plane plane_line_inner_anti_product(Plane self, Line other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(-1.0, 1.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, -1.0, 1.0, 1.0));
}

Point plane_line_right_contraction(Plane self, Line other) {
    return Point(vec4(self.g0.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, -1.0));
}

Plane plane_line_left_anti_contraction(Plane self, Line other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(-1.0, 1.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, -1.0, 1.0, 1.0));
}

Plane plane_plane_add(Plane self, Plane other) {
    return Plane(self.g0 + other.g0);
}

Plane plane_plane_sub(Plane self, Plane other) {
    return Plane(self.g0 - other.g0);
}

Plane plane_plane_mul(Plane self, Plane other) {
    return Plane(self.g0 * other.g0);
}

Plane plane_plane_div(Plane self, Plane other) {
    return Plane(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.w) * vec4(1.0, 1.0, 1.0, 1.0));
}

MultiVector plane_plane_geometric_product(Plane self, Plane other) {
    return MultiVector(vec2(self.g0.w, self.g0.x) * vec2(other.g0.w, other.g0.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(0.0) - vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w), vec3(0.0), vec4(0.0));
}

Motor plane_plane_geometric_anti_product(Plane self, Plane other) {
    return Motor(vec4(self.g0.y) * other.g0.zzxy * vec4(-1.0, 0.0, 1.0, 1.0) + vec4(self.g0.z) * other.g0.yxyz * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, 1.0, -1.0, 1.0), vec3(0.0) - vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w));
}

Line plane_plane_regressive_product(Plane self, Plane other) {
    return Line(vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w));
}

Line plane_plane_anti_wedge(Plane self, Plane other) {
    return Line(vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w));
}

Line plane_plane_meet(Plane self, Plane other) {
    return Line(vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w));
}

Scalar plane_plane_inner_product(Plane self, Plane other) {
    return Scalar(0.0 - self.g0.w * other.g0.w);
}

AntiScalar plane_plane_inner_anti_product(Plane self, Plane other) {
    return AntiScalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

Scalar plane_plane_left_contraction(Plane self, Plane other) {
    return Scalar(0.0 - self.g0.w * other.g0.w);
}

Scalar plane_plane_right_contraction(Plane self, Plane other) {
    return Scalar(0.0 - self.g0.w * other.g0.w);
}

AntiScalar plane_plane_left_anti_contraction(Plane self, Plane other) {
    return AntiScalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

AntiScalar plane_plane_right_anti_contraction(Plane self, Plane other) {
    return AntiScalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

Scalar plane_plane_scalar_product(Plane self, Plane other) {
    return Scalar(0.0 - self.g0.w * other.g0.w);
}

Scalar plane_plane_dot(Plane self, Plane other) {
    return Scalar(0.0 - self.g0.w * other.g0.w);
}

AntiScalar plane_plane_anti_scalar_product(Plane self, Plane other) {
    return AntiScalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

AntiScalar plane_plane_anti_dot(Plane self, Plane other) {
    return AntiScalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

Flector plane_motor_geometric_product(Plane self, Motor other) {
    return Flector(vec4(self.g0.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g0.w) + vec4(self.g0.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0), vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.z) * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.y) * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g0.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g1.x) * vec4(0.0, 1.0, -1.0, 0.0));
}

Flector plane_motor_geometric_anti_product(Plane self, Motor other) {
    return Flector(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0), vec4(self.g0.x) * vec4(other.g0.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g0.y) * vec4(other.g0.z, other.g0.w, other.g0.x, other.g1.y) * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.w, other.g1.z) * vec4(-1.0, 1.0, 1.0, 1.0) + self.g0.xxxw * other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0));
}

Flector plane_motor_regressive_product(Plane self, Motor other) {
    return Flector(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0), self.g0 * vec4(other.g0.w));
}

Flector plane_motor_anti_wedge(Plane self, Motor other) {
    return Flector(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0), self.g0 * vec4(other.g0.w));
}

Flector plane_motor_meet(Plane self, Motor other) {
    return Flector(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0), self.g0 * vec4(other.g0.w));
}

Point plane_motor_inner_product(Plane self, Motor other) {
    return Point(vec4(self.g0.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g0.w) + vec4(self.g0.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

Plane plane_motor_inner_anti_product(Plane self, Motor other) {
    return Plane(vec4(self.g0.x) * vec4(other.g0.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g0.y) * vec4(other.g0.z, other.g0.w, other.g0.x, other.g1.y) * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.w, other.g1.z) * vec4(-1.0, 1.0, 1.0, 1.0) + self.g0.xxxw * other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0));
}

Point plane_motor_left_contraction(Plane self, Motor other) {
    return Point(self.g0.xxxw * other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0));
}

Point plane_motor_right_contraction(Plane self, Motor other) {
    return Point(vec4(self.g0.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, -1.0));
}

Plane plane_motor_left_anti_contraction(Plane self, Motor other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(-1.0, 1.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, -1.0, 1.0, 1.0));
}

Plane plane_motor_right_anti_contraction(Plane self, Motor other) {
    return Plane(self.g0 * vec4(other.g0.w));
}

Flector plane_rotor_geometric_product(Plane self, Rotor other) {
    return Flector(self.g0.xxxw * other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0), self.g0.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0));
}

Flector plane_rotor_geometric_anti_product(Plane self, Rotor other) {
    return Flector(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0), vec4(self.g0.y) * other.g0.zwxz * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxwy * vec4(-1.0, 1.0, 1.0, 0.0) + self.g0.xxxw * other.g0.wzyw * vec4(1.0, -1.0, 1.0, 1.0));
}

Flector plane_rotor_regressive_product(Plane self, Rotor other) {
    return Flector(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0), self.g0 * vec4(other.g0.w));
}

Flector plane_rotor_anti_wedge(Plane self, Rotor other) {
    return Flector(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0), self.g0 * vec4(other.g0.w));
}

Flector plane_rotor_meet(Plane self, Rotor other) {
    return Flector(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0), self.g0 * vec4(other.g0.w));
}

Point plane_rotor_inner_product(Plane self, Rotor other) {
    return Point(self.g0.xxxw * other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0));
}

Plane plane_rotor_inner_anti_product(Plane self, Rotor other) {
    return Plane(vec4(self.g0.y) * other.g0.zwxz * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxwy * vec4(-1.0, 1.0, 1.0, 0.0) + self.g0.xxxw * other.g0.wzyw * vec4(1.0, -1.0, 1.0, 1.0));
}

Point plane_rotor_left_contraction(Plane self, Rotor other) {
    return Point(self.g0.xxxw * other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0));
}

Plane plane_rotor_left_anti_contraction(Plane self, Rotor other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, 0.0));
}

Plane plane_rotor_right_anti_contraction(Plane self, Rotor other) {
    return Plane(self.g0 * vec4(other.g0.w));
}

Flector plane_translator_geometric_product(Plane self, Translator other) {
    return Flector(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.w) * other.g0 + vec4(self.g0.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0), vec4(self.g0.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, 1.0, -1.0, 0.0));
}

Flector plane_translator_geometric_anti_product(Plane self, Translator other) {
    return Flector(vec4(self.g0.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, 1.0, -1.0, 0.0), vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, 1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, 1.0));
}

Flector plane_translator_regressive_product(Plane self, Translator other) {
    return Flector(vec4(self.g0.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, 1.0, -1.0, 0.0), self.g0 * vec4(other.g0.w));
}

Flector plane_translator_anti_wedge(Plane self, Translator other) {
    return Flector(vec4(self.g0.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, 1.0, -1.0, 0.0), self.g0 * vec4(other.g0.w));
}

Flector plane_translator_meet(Plane self, Translator other) {
    return Flector(vec4(self.g0.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, 1.0, -1.0, 0.0), self.g0 * vec4(other.g0.w));
}

Point plane_translator_inner_product(Plane self, Translator other) {
    return Point(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.w) * other.g0 + vec4(self.g0.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

Plane plane_translator_inner_anti_product(Plane self, Translator other) {
    return Plane(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, 1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, 1.0));
}

Point plane_translator_left_contraction(Plane self, Translator other) {
    return Point(self.g0.xxxw * other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0));
}

Point plane_translator_right_contraction(Plane self, Translator other) {
    return Point(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

Plane plane_translator_left_anti_contraction(Plane self, Translator other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0));
}

Plane plane_translator_right_anti_contraction(Plane self, Translator other) {
    return Plane(self.g0 * vec4(other.g0.w));
}

Flector plane_flector_add(Plane self, Flector other) {
    return Flector(other.g0, self.g0 + other.g1);
}

Flector plane_flector_sub(Plane self, Flector other) {
    return Flector(vec4(0.0) - other.g0, self.g0 - other.g1);
}

MultiVector plane_flector_geometric_product(Plane self, Flector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(0.0, -1.0) - vec2(self.g0.w) * vec2(other.g1.w, other.g0.w) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(0.0, -1.0), vec4(0.0), vec3(self.g0.x) * vec3(other.g1.w, other.g0.z, other.g0.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.y) * vec3(other.g0.z, other.g1.w, other.g0.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g1.w) * vec3(1.0, -1.0, 1.0) - vec3(self.g0.w) * vec3(other.g1.x, other.g1.y, other.g1.z), vec3(0.0) - vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(0.0));
}

MultiVector plane_flector_geometric_anti_product(Plane self, Flector other) {
    return MultiVector(vec2(self.g0.x) * vec2(other.g0.x, other.g1.x) * vec2(-1.0, 1.0) + vec2(self.g0.y) * vec2(other.g0.y, other.g1.y) * vec2(-1.0, 1.0) + vec2(self.g0.z) * vec2(other.g0.z, other.g1.z) * vec2(-1.0, 1.0) + vec2(self.g0.w, self.g0.x) * vec2(other.g0.w, other.g0.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.x) * vec3(other.g0.w, other.g1.z, other.g1.y) * vec3(-1.0, 1.0, -1.0) + vec3(self.g0.y) * vec3(other.g1.z, other.g0.w, other.g1.x) * vec3(-1.0, -1.0, 1.0) + vec3(self.g0.z) * vec3(other.g1.y, other.g1.x, other.g0.w) * vec3(1.0, -1.0, -1.0), vec3(self.g0.x) * vec3(other.g1.w, other.g0.z, other.g0.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g0.z, other.g1.w, other.g0.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g1.w) * vec3(-1.0, 1.0, 1.0) - vec3(self.g0.w) * vec3(other.g1.x, other.g1.y, other.g1.z), vec4(0.0));
}

MultiVector plane_flector_regressive_product(Plane self, Flector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.w) * vec2(other.g0.w) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * vec3(other.g1.x, other.g1.z, other.g1.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g0.w) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.w), vec4(0.0));
}

MultiVector plane_flector_anti_wedge(Plane self, Flector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.w) * vec2(other.g0.w) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * vec3(other.g1.x, other.g1.z, other.g1.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g0.w) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.w), vec4(0.0));
}

MultiVector plane_flector_meet(Plane self, Flector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.w) * vec2(other.g0.w) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * vec3(other.g1.x, other.g1.z, other.g1.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g0.w) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.w), vec4(0.0));
}

AntiScalar plane_flector_outer_product(Plane self, Flector other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

AntiScalar plane_flector_wedge(Plane self, Flector other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

AntiScalar plane_flector_join(Plane self, Flector other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

MultiVector plane_flector_inner_product(Plane self, Flector other) {
    return MultiVector(vec2(self.g0.w, self.g0.x) * vec2(other.g1.w, other.g1.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(0.0));
}

Motor plane_flector_inner_anti_product(Plane self, Flector other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.w, other.g0.w, other.g0.w, other.g1.y) * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.w, other.g0.w, other.g0.w, other.g1.z) * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.w, other.g0.x, other.g0.x, other.g1.x) * vec4(-1.0, 0.0, 0.0, 1.0), vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0));
}

Scalar plane_flector_left_contraction(Plane self, Flector other) {
    return Scalar(0.0 - self.g0.w * other.g1.w);
}

MultiVector plane_flector_right_contraction(Plane self, Flector other) {
    return MultiVector(vec2(self.g0.w, self.g0.x) * vec2(other.g1.w, other.g1.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(0.0));
}

Motor plane_flector_left_anti_contraction(Plane self, Flector other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.w, other.g0.w, other.g0.w, other.g1.y) * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.w, other.g0.w, other.g0.w, other.g1.z) * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.w, other.g0.x, other.g0.x, other.g1.x) * vec4(-1.0, 0.0, 0.0, 1.0), vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0));
}

AntiScalar plane_flector_right_anti_contraction(Plane self, Flector other) {
    return AntiScalar(self.g0.x * other.g1.x + self.g0.y * other.g1.y + self.g0.z * other.g1.z);
}

Scalar plane_flector_scalar_product(Plane self, Flector other) {
    return Scalar(0.0 - self.g0.w * other.g1.w);
}

Scalar plane_flector_dot(Plane self, Flector other) {
    return Scalar(0.0 - self.g0.w * other.g1.w);
}

AntiScalar plane_flector_anti_scalar_product(Plane self, Flector other) {
    return AntiScalar(self.g0.x * other.g1.x + self.g0.y * other.g1.y + self.g0.z * other.g1.z);
}

AntiScalar plane_flector_anti_dot(Plane self, Flector other) {
    return AntiScalar(self.g0.x * other.g1.x + self.g0.y * other.g1.y + self.g0.z * other.g1.z);
}

MultiVector plane_multi_vector_add(Plane self, MultiVector other) {
    return MultiVector(other.g0, other.g1, other.g2, other.g3, self.g0 + other.g4);
}

MultiVector plane_multi_vector_sub(Plane self, MultiVector other) {
    return MultiVector(vec2(0.0) - other.g0, vec4(0.0) - other.g1, vec3(0.0) - other.g2, vec3(0.0) - other.g3, self.g0 - other.g4);
}

MultiVector plane_multi_vector_geometric_product(Plane self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(0.0, -1.0) - vec2(self.g0.w) * vec2(other.g4.w, other.g1.w) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(0.0, -1.0), vec4(self.g0.y) * vec4(other.g3.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g3.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g3.x, other.g3.y, other.g3.z, other.g0.y) + vec4(self.g0.x) * vec4(other.g3.x) * vec4(0.0, 0.0, 0.0, -1.0), vec3(self.g0.x) * vec3(other.g4.w, other.g1.z, other.g1.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.y) * vec3(other.g1.z, other.g4.w, other.g1.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g0.z) * vec3(other.g1.y, other.g1.x, other.g4.w) * vec3(1.0, -1.0, 1.0) - vec3(self.g0.w) * vec3(other.g4.x, other.g4.y, other.g4.z), vec3(0.0) - vec3(self.g0.w) * vec3(other.g1.x, other.g1.y, other.g1.z), vec4(self.g0.y) * vec4(other.g3.z, other.g0.x, other.g3.x, other.g3.z) * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g0.z) * vec4(other.g3.y, other.g3.x, other.g0.x, other.g3.y) * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g0.w) * vec4(other.g2.x, other.g2.y, other.g2.z, other.g0.x) + vec4(self.g0.x) * vec4(other.g0.x, other.g3.z, other.g3.y, other.g0.x) * vec4(1.0, 1.0, -1.0, 0.0));
}

MultiVector plane_multi_vector_geometric_anti_product(Plane self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * vec2(other.g1.x, other.g4.x) * vec2(-1.0, 1.0) + vec2(self.g0.y) * vec2(other.g1.y, other.g4.y) * vec2(-1.0, 1.0) + vec2(self.g0.z) * vec2(other.g1.z, other.g4.z) * vec2(-1.0, 1.0) + vec2(self.g0.w, self.g0.x) * vec2(other.g1.w, other.g1.x) * vec2(-1.0, 0.0), vec4(self.g0.x) * vec4(other.g0.x, other.g3.z, other.g3.y, other.g2.x) * vec4(-1.0, 1.0, -1.0, -1.0) + vec4(self.g0.y) * vec4(other.g3.z, other.g0.x, other.g3.x, other.g2.y) * vec4(-1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g3.y, other.g3.x, other.g0.x, other.g2.z) * vec4(1.0, -1.0, -1.0, -1.0) + self.g0.wwwx * vec4(other.g2.x, other.g2.y, other.g2.z, other.g2.x) * vec4(1.0, 1.0, 1.0, 0.0), vec3(self.g0.x) * vec3(other.g1.w, other.g4.z, other.g4.y) * vec3(-1.0, 1.0, -1.0) + vec3(self.g0.y) * vec3(other.g4.z, other.g1.w, other.g4.x) * vec3(-1.0, -1.0, 1.0) + vec3(self.g0.z) * vec3(other.g4.y, other.g4.x, other.g1.w) * vec3(1.0, -1.0, -1.0), vec3(self.g0.x) * vec3(other.g4.w, other.g1.z, other.g1.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g1.z, other.g4.w, other.g1.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g1.y, other.g1.x, other.g4.w) * vec3(-1.0, 1.0, 1.0) - vec3(self.g0.w) * vec3(other.g4.x, other.g4.y, other.g4.z), vec4(self.g0.x) * vec4(other.g0.y, other.g2.z, other.g2.y, other.g3.x) * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g0.y) * vec4(other.g2.z, other.g0.y, other.g2.x, other.g3.y) * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g2.y, other.g2.x, other.g0.y, other.g3.z) * vec4(-1.0, 1.0, 1.0, 1.0) + self.g0.xxxw * vec4(other.g0.x, other.g0.x, other.g0.x, other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0));
}

MultiVector plane_multi_vector_regressive_product(Plane self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g0.w) * vec2(other.g1.w) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(-1.0, 0.0), vec4(self.g0.y) * vec4(other.g3.z, other.g3.z, other.g3.x, other.g2.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g3.y, other.g3.x, other.g3.y, other.g2.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g2.x, other.g2.y, other.g2.z, other.g2.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g3.x, other.g3.z, other.g3.y, other.g2.x) * vec4(0.0, 1.0, -1.0, -1.0), vec3(self.g0.y) * vec3(other.g4.z, other.g4.z, other.g4.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * vec3(other.g4.y, other.g4.x, other.g4.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * vec3(other.g4.x, other.g4.z, other.g4.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g0.w) * vec3(other.g4.x, other.g4.y, other.g4.z) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g4.w), self.g0 * vec4(other.g0.y));
}

MultiVector plane_multi_vector_anti_wedge(Plane self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g0.w) * vec2(other.g1.w) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(-1.0, 0.0), vec4(self.g0.y) * vec4(other.g3.z, other.g3.z, other.g3.x, other.g2.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g3.y, other.g3.x, other.g3.y, other.g2.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g2.x, other.g2.y, other.g2.z, other.g2.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g3.x, other.g3.z, other.g3.y, other.g2.x) * vec4(0.0, 1.0, -1.0, -1.0), vec3(self.g0.y) * vec3(other.g4.z, other.g4.z, other.g4.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * vec3(other.g4.y, other.g4.x, other.g4.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * vec3(other.g4.x, other.g4.z, other.g4.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g0.w) * vec3(other.g4.x, other.g4.y, other.g4.z) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g4.w), self.g0 * vec4(other.g0.y));
}

MultiVector plane_multi_vector_meet(Plane self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g0.w) * vec2(other.g1.w) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(-1.0, 0.0), vec4(self.g0.y) * vec4(other.g3.z, other.g3.z, other.g3.x, other.g2.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g3.y, other.g3.x, other.g3.y, other.g2.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g2.x, other.g2.y, other.g2.z, other.g2.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g3.x, other.g3.z, other.g3.y, other.g2.x) * vec4(0.0, 1.0, -1.0, -1.0), vec3(self.g0.y) * vec3(other.g4.z, other.g4.z, other.g4.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * vec3(other.g4.y, other.g4.x, other.g4.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * vec3(other.g4.x, other.g4.z, other.g4.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g0.w) * vec3(other.g4.x, other.g4.y, other.g4.z) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g4.w), self.g0 * vec4(other.g0.y));
}

MultiVector plane_multi_vector_outer_product(Plane self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g0.w) * vec2(other.g1.w) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(0.0, -1.0), vec4(0.0), vec3(0.0), vec3(0.0), self.g0 * vec4(other.g0.x));
}

MultiVector plane_multi_vector_wedge(Plane self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g0.w) * vec2(other.g1.w) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(0.0, -1.0), vec4(0.0), vec3(0.0), vec3(0.0), self.g0 * vec4(other.g0.x));
}

MultiVector plane_multi_vector_join(Plane self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g0.w) * vec2(other.g1.w) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(0.0, -1.0), vec4(0.0), vec3(0.0), vec3(0.0), self.g0 * vec4(other.g0.x));
}

MultiVector plane_multi_vector_inner_product(Plane self, MultiVector other) {
    return MultiVector(vec2(self.g0.w, self.g0.x) * vec2(other.g4.w, other.g4.x) * vec2(-1.0, 0.0), vec4(self.g0.y) * vec4(other.g3.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g3.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g3.x, other.g3.y, other.g3.z, other.g0.y) + vec4(self.g0.x) * vec4(other.g3.x) * vec4(0.0, 0.0, 0.0, -1.0), vec3(self.g0.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * vec3(other.g1.x, other.g1.z, other.g1.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g0.w) * vec3(other.g1.x, other.g1.y, other.g1.z), self.g0 * vec4(other.g0.x));
}

MultiVector plane_multi_vector_inner_anti_product(Plane self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g4.y) * vec2(0.0, 1.0) + vec2(self.g0.z) * vec2(other.g4.z) * vec2(0.0, 1.0) + vec2(self.g0.x) * vec2(other.g4.x) * vec2(0.0, 1.0), self.g0.xyzx * vec4(other.g0.x) * vec4(-1.0, -1.0, -1.0, 0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.w) * vec3(-1.0), vec3(self.g0.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g1.x, other.g1.z, other.g1.y) * vec3(0.0, -1.0, 1.0), vec4(self.g0.x) * vec4(other.g0.y, other.g2.z, other.g2.y, other.g3.x) * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g0.y) * vec4(other.g2.z, other.g0.y, other.g2.x, other.g3.y) * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g2.y, other.g2.x, other.g0.y, other.g3.z) * vec4(-1.0, 1.0, 1.0, 1.0) + self.g0.xxxw * vec4(other.g0.x, other.g0.x, other.g0.x, other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0));
}

MultiVector plane_multi_vector_left_contraction(Plane self, MultiVector other) {
    return MultiVector(vec2(self.g0.w, self.g0.x) * vec2(other.g4.w, other.g4.x) * vec2(-1.0, 0.0), self.g0.xxxw * vec4(other.g0.x, other.g0.x, other.g0.x, other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0), vec3(0.0), vec3(0.0), vec4(0.0));
}

MultiVector plane_multi_vector_right_contraction(Plane self, MultiVector other) {
    return MultiVector(vec2(self.g0.w, self.g0.x) * vec2(other.g4.w, other.g4.x) * vec2(-1.0, 0.0), vec4(self.g0.y) * vec4(other.g3.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g3.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * vec4(other.g3.x, other.g3.y, other.g3.z, other.g3.x) * vec4(1.0, 1.0, 1.0, -1.0), vec3(self.g0.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * vec3(other.g1.x, other.g1.z, other.g1.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g0.w) * vec3(other.g1.x, other.g1.y, other.g1.z), self.g0 * vec4(other.g0.x));
}

MultiVector plane_multi_vector_left_anti_contraction(Plane self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g4.y) * vec2(0.0, 1.0) + vec2(self.g0.z) * vec2(other.g4.z) * vec2(0.0, 1.0) + vec2(self.g0.x) * vec2(other.g4.x) * vec2(0.0, 1.0), self.g0.xyzx * vec4(other.g0.x) * vec4(-1.0, -1.0, -1.0, 0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.w) * vec3(-1.0), vec3(self.g0.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g1.x, other.g1.z, other.g1.y) * vec3(0.0, -1.0, 1.0), vec4(self.g0.y) * vec4(other.g2.z, other.g2.z, other.g2.x, other.g3.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g2.y, other.g2.x, other.g2.y, other.g3.z) * vec4(-1.0, 1.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g2.x, other.g2.z, other.g2.y, other.g3.x) * vec4(0.0, -1.0, 1.0, 1.0));
}

MultiVector plane_multi_vector_right_anti_contraction(Plane self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g4.y) * vec2(0.0, 1.0) + vec2(self.g0.z) * vec2(other.g4.z) * vec2(0.0, 1.0) + vec2(self.g0.x) * vec2(other.g4.x) * vec2(0.0, 1.0), vec4(0.0), vec3(0.0), vec3(0.0), self.g0 * vec4(other.g0.y));
}

Scalar plane_multi_vector_scalar_product(Plane self, MultiVector other) {
    return Scalar(0.0 - self.g0.w * other.g4.w);
}

Scalar plane_multi_vector_dot(Plane self, MultiVector other) {
    return Scalar(0.0 - self.g0.w * other.g4.w);
}

AntiScalar plane_multi_vector_anti_scalar_product(Plane self, MultiVector other) {
    return AntiScalar(self.g0.x * other.g4.x + self.g0.y * other.g4.y + self.g0.z * other.g4.z);
}

AntiScalar plane_multi_vector_anti_dot(Plane self, MultiVector other) {
    return AntiScalar(self.g0.x * other.g4.x + self.g0.y * other.g4.y + self.g0.z * other.g4.z);
}

Motor motor_scalar_geometric_product(Motor self, Scalar other) {
    return Motor(self.g0 * vec4(other.g0), self.g1 * vec3(other.g0));
}

MultiVector motor_scalar_geometric_anti_product(Motor self, Scalar other) {
    return MultiVector(vec2(self.g0.w, self.g0.x) * vec2(other.g0) * vec2(1.0, 0.0), vec4(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0), vec4(0.0));
}

Scalar motor_scalar_regressive_product(Motor self, Scalar other) {
    return Scalar(self.g0.w * other.g0);
}

Scalar motor_scalar_anti_wedge(Motor self, Scalar other) {
    return Scalar(self.g0.w * other.g0);
}

Scalar motor_scalar_meet(Motor self, Scalar other) {
    return Scalar(self.g0.w * other.g0);
}

Motor motor_scalar_outer_product(Motor self, Scalar other) {
    return Motor(self.g0 * vec4(other.g0), self.g1 * vec3(other.g0));
}

Motor motor_scalar_wedge(Motor self, Scalar other) {
    return Motor(self.g0 * vec4(other.g0), self.g1 * vec3(other.g0));
}

Motor motor_scalar_join(Motor self, Scalar other) {
    return Motor(self.g0 * vec4(other.g0), self.g1 * vec3(other.g0));
}

Motor motor_scalar_inner_product(Motor self, Scalar other) {
    return Motor(self.g0 * vec4(other.g0), self.g1 * vec3(other.g0));
}

MultiVector motor_scalar_inner_anti_product(Motor self, Scalar other) {
    return MultiVector(vec2(self.g0.w, self.g0.x) * vec2(other.g0) * vec2(1.0, 0.0), vec4(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0), vec4(0.0));
}

Motor motor_scalar_right_contraction(Motor self, Scalar other) {
    return Motor(self.g0 * vec4(other.g0), self.g1 * vec3(other.g0));
}

MultiVector motor_scalar_left_anti_contraction(Motor self, Scalar other) {
    return MultiVector(vec2(self.g0.w, self.g0.x) * vec2(other.g0) * vec2(1.0, 0.0), vec4(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0), vec4(0.0));
}

AntiScalar motor_anti_scalar_into(Motor self) {
    return AntiScalar(self.g0.w);
}

Motor motor_anti_scalar_add(Motor self, AntiScalar other) {
    return Motor(self.g0 + vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0), self.g1);
}

Motor motor_anti_scalar_sub(Motor self, AntiScalar other) {
    return Motor(self.g0 - vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0), self.g1);
}

Rotor motor_anti_scalar_geometric_product(Motor self, AntiScalar other) {
    return Rotor(vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.x) * vec4(other.g0) * vec4(1.0, 1.0, 1.0, 0.0));
}

Motor motor_anti_scalar_geometric_anti_product(Motor self, AntiScalar other) {
    return Motor(self.g0 * vec4(other.g0), self.g1 * vec3(other.g0));
}

Motor motor_anti_scalar_regressive_product(Motor self, AntiScalar other) {
    return Motor(self.g0 * vec4(other.g0), self.g1 * vec3(other.g0));
}

Motor motor_anti_scalar_anti_wedge(Motor self, AntiScalar other) {
    return Motor(self.g0 * vec4(other.g0), self.g1 * vec3(other.g0));
}

Motor motor_anti_scalar_meet(Motor self, AntiScalar other) {
    return Motor(self.g0 * vec4(other.g0), self.g1 * vec3(other.g0));
}

Rotor motor_anti_scalar_inner_product(Motor self, AntiScalar other) {
    return Rotor(vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.x) * vec4(other.g0) * vec4(1.0, 1.0, 1.0, 0.0));
}

Motor motor_anti_scalar_inner_anti_product(Motor self, AntiScalar other) {
    return Motor(self.g0 * vec4(other.g0), self.g1 * vec3(other.g0));
}

Rotor motor_anti_scalar_left_contraction(Motor self, AntiScalar other) {
    return Rotor(vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.x) * vec4(other.g0) * vec4(1.0, 1.0, 1.0, 0.0));
}

AntiScalar motor_anti_scalar_left_anti_contraction(Motor self, AntiScalar other) {
    return AntiScalar(self.g0.w * other.g0);
}

Motor motor_anti_scalar_right_anti_contraction(Motor self, AntiScalar other) {
    return Motor(self.g0 * vec4(other.g0), self.g1 * vec3(other.g0));
}

AntiScalar motor_anti_scalar_anti_scalar_product(Motor self, AntiScalar other) {
    return AntiScalar(self.g0.w * other.g0);
}

AntiScalar motor_anti_scalar_anti_dot(Motor self, AntiScalar other) {
    return AntiScalar(self.g0.w * other.g0);
}

Motor motor_homogeneous_magnitude_geometric_product(Motor self, HomogeneousMagnitude other) {
    return Motor(vec4(self.g1.x) * vec4(other.g0.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.y) * vec4(0.0, 0.0, 1.0, 0.0) + self.g0 * vec4(other.g0.x), self.g1 * vec3(other.g0.x));
}

MultiVector motor_homogeneous_magnitude_geometric_anti_product(Motor self, HomogeneousMagnitude other) {
    return MultiVector(vec2(self.g0.w) * other.g0, vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), vec3(self.g1.x) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g1.y) * vec3(other.g0.y) * vec3(0.0, 1.0, 0.0) + vec3(self.g1.z) * vec3(other.g0.y) * vec3(0.0, 0.0, 1.0) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x), vec4(0.0));
}

MultiVector motor_homogeneous_magnitude_regressive_product(Motor self, HomogeneousMagnitude other) {
    return MultiVector(vec2(self.g0.w) * other.g0, vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), self.g1 * vec3(other.g0.y), vec4(0.0));
}

MultiVector motor_homogeneous_magnitude_anti_wedge(Motor self, HomogeneousMagnitude other) {
    return MultiVector(vec2(self.g0.w) * other.g0, vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), self.g1 * vec3(other.g0.y), vec4(0.0));
}

MultiVector motor_homogeneous_magnitude_meet(Motor self, HomogeneousMagnitude other) {
    return MultiVector(vec2(self.g0.w) * other.g0, vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), self.g1 * vec3(other.g0.y), vec4(0.0));
}

Motor motor_homogeneous_magnitude_outer_product(Motor self, HomogeneousMagnitude other) {
    return Motor(self.g0 * vec4(other.g0.x), self.g1 * vec3(other.g0.x));
}

Motor motor_homogeneous_magnitude_wedge(Motor self, HomogeneousMagnitude other) {
    return Motor(self.g0 * vec4(other.g0.x), self.g1 * vec3(other.g0.x));
}

Motor motor_homogeneous_magnitude_join(Motor self, HomogeneousMagnitude other) {
    return Motor(self.g0 * vec4(other.g0.x), self.g1 * vec3(other.g0.x));
}

Motor motor_homogeneous_magnitude_inner_product(Motor self, HomogeneousMagnitude other) {
    return Motor(vec4(self.g1.x) * vec4(other.g0.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.y) * vec4(0.0, 0.0, 1.0, 0.0) + self.g0 * vec4(other.g0.x), self.g1 * vec3(other.g0.x));
}

MultiVector motor_homogeneous_magnitude_inner_anti_product(Motor self, HomogeneousMagnitude other) {
    return MultiVector(vec2(self.g0.w) * other.g0, vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), vec3(self.g1.x) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g1.y) * vec3(other.g0.y) * vec3(0.0, 1.0, 0.0) + vec3(self.g1.z) * vec3(other.g0.y) * vec3(0.0, 0.0, 1.0) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x), vec4(0.0));
}

Rotor motor_homogeneous_magnitude_left_contraction(Motor self, HomogeneousMagnitude other) {
    return Rotor(vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.x) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Motor motor_homogeneous_magnitude_right_contraction(Motor self, HomogeneousMagnitude other) {
    return Motor(self.g0 * vec4(other.g0.x), self.g1 * vec3(other.g0.x));
}

MultiVector motor_homogeneous_magnitude_left_anti_contraction(Motor self, HomogeneousMagnitude other) {
    return MultiVector(vec2(self.g0.w) * other.g0, vec4(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x), vec4(0.0));
}

Motor motor_homogeneous_magnitude_right_anti_contraction(Motor self, HomogeneousMagnitude other) {
    return Motor(self.g0 * vec4(other.g0.y), self.g1 * vec3(other.g0.y));
}

AntiScalar motor_homogeneous_magnitude_anti_scalar_product(Motor self, HomogeneousMagnitude other) {
    return AntiScalar(self.g0.w * other.g0.y);
}

AntiScalar motor_homogeneous_magnitude_anti_dot(Motor self, HomogeneousMagnitude other) {
    return AntiScalar(self.g0.w * other.g0.y);
}

Flector motor_point_geometric_product(Motor self, Point other) {
    return Flector(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0), vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.w) * other.g0.xyzx * vec4(-1.0, -1.0, -1.0, 0.0) + vec4(self.g1.x) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, 0.0));
}

Flector motor_point_geometric_anti_product(Motor self, Point other) {
    return Flector(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.w) * other.g0 + vec4(self.g1.x) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.w) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.w) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, 0.0), vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Point motor_point_regressive_product(Motor self, Point other) {
    return Point(vec4(self.g0.w) * other.g0);
}

Point motor_point_anti_wedge(Motor self, Point other) {
    return Point(vec4(self.g0.w) * other.g0);
}

Point motor_point_meet(Motor self, Point other) {
    return Point(vec4(self.g0.w) * other.g0);
}

Plane motor_point_outer_product(Motor self, Point other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.x) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, 0.0));
}

Plane motor_point_wedge(Motor self, Point other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.x) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, 0.0));
}

Plane motor_point_join(Motor self, Point other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.x) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, 0.0));
}

Flector motor_point_inner_product(Motor self, Point other) {
    return Flector(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0), self.g0.wwwx * other.g0.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

Flector motor_point_inner_anti_product(Motor self, Point other) {
    return Flector(vec4(self.g0.w) * other.g0, vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Flector motor_point_right_contraction(Motor self, Point other) {
    return Flector(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0), self.g0.wwwx * other.g0.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

Flector motor_point_left_anti_contraction(Motor self, Point other) {
    return Flector(vec4(self.g0.w) * other.g0, vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Line motor_line_into(Motor self) {
    return Line(vec3(self.g0.x, self.g0.y, self.g0.z), self.g1);
}

Motor motor_line_add(Motor self, Line other) {
    return Motor(self.g0 + vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0), self.g1 + other.g1);
}

Motor motor_line_sub(Motor self, Line other) {
    return Motor(self.g0 - vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0), self.g1 - other.g1);
}

MultiVector motor_line_geometric_product(Motor self, Line other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(0.0, -1.0) - vec2(self.g1.x) * vec2(other.g1.x, other.g0.x) - vec2(self.g1.y) * vec2(other.g1.y, other.g0.y) - vec2(self.g1.z) * vec2(other.g1.z, other.g0.z) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(0.0, -1.0), vec4(0.0), vec3(self.g0.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g0.w) * other.g1 + vec3(self.g1.x) * other.g0.zzy * vec3(0.0, 1.0, -1.0) + vec3(self.g1.y) * other.g0.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * other.g0.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * other.g1.xzy * vec3(0.0, 1.0, -1.0), vec3(self.g1.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g1.x) * other.g1.xzy * vec3(0.0, 1.0, -1.0), vec4(0.0));
}

MultiVector motor_line_geometric_anti_product(Motor self, Line other) {
    return MultiVector(vec2(0.0) - vec2(self.g0.x) * vec2(other.g1.x, other.g0.x) - vec2(self.g0.y) * vec2(other.g1.y, other.g0.y) - vec2(self.g0.z) * vec2(other.g1.z, other.g0.z) + vec2(self.g1.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.y) * other.g0.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * other.g0.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.w) * other.g0 + vec3(self.g0.x) * other.g0.xzy * vec3(0.0, -1.0, 1.0), vec3(self.g0.y) * other.g1.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * other.g1.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.w) * other.g1 + vec3(self.g1.x) * other.g0.zzy * vec3(0.0, -1.0, 1.0) + vec3(self.g1.y) * other.g0.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * other.g0.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * other.g1.xzy * vec3(0.0, -1.0, 1.0), vec4(0.0));
}

MultiVector motor_line_regressive_product(Motor self, Line other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g1.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.w) * other.g0, vec3(self.g0.w) * other.g1, vec4(0.0));
}

MultiVector motor_line_anti_wedge(Motor self, Line other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g1.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.w) * other.g0, vec3(self.g0.w) * other.g1, vec4(0.0));
}

MultiVector motor_line_meet(Motor self, Line other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g1.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.w) * other.g0, vec3(self.g0.w) * other.g1, vec4(0.0));
}

AntiScalar motor_line_outer_product(Motor self, Line other) {
    return AntiScalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

AntiScalar motor_line_wedge(Motor self, Line other) {
    return AntiScalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

AntiScalar motor_line_join(Motor self, Line other) {
    return AntiScalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

MultiVector motor_line_inner_product(Motor self, Line other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g1.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.w) * other.g1, vec3(0.0), vec4(0.0));
}

Motor motor_line_inner_anti_product(Motor self, Line other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, -1.0), vec3(self.g0.w) * other.g1);
}

Scalar motor_line_left_contraction(Motor self, Line other) {
    return Scalar(0.0 - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z);
}

MultiVector motor_line_right_contraction(Motor self, Line other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g1.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.w) * other.g1, vec3(0.0), vec4(0.0));
}

Motor motor_line_left_anti_contraction(Motor self, Line other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, -1.0), vec3(self.g0.w) * other.g1);
}

AntiScalar motor_line_right_anti_contraction(Motor self, Line other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

Scalar motor_line_scalar_product(Motor self, Line other) {
    return Scalar(0.0 - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z);
}

Scalar motor_line_dot(Motor self, Line other) {
    return Scalar(0.0 - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z);
}

AntiScalar motor_line_anti_scalar_product(Motor self, Line other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

AntiScalar motor_line_anti_dot(Motor self, Line other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

Flector motor_plane_geometric_product(Motor self, Plane other) {
    return Flector(vec4(self.g1.x) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + self.g0.xxxw * other.g0.xxxw * vec4(0.0, 0.0, 0.0, -1.0), vec4(self.g1.x) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + self.g0.xyzx * other.g0.wwwx * vec4(-1.0, -1.0, -1.0, 0.0));
}

Flector motor_plane_geometric_anti_product(Motor self, Plane other) {
    return Flector(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.x) * other.g0.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0), vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.w) * other.g0 + vec4(self.g1.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, 0.0));
}

Flector motor_plane_regressive_product(Motor self, Plane other) {
    return Flector(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.x) * other.g0.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0), vec4(self.g0.w) * other.g0);
}

Flector motor_plane_anti_wedge(Motor self, Plane other) {
    return Flector(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.x) * other.g0.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0), vec4(self.g0.w) * other.g0);
}

Flector motor_plane_meet(Motor self, Plane other) {
    return Flector(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.x) * other.g0.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0), vec4(self.g0.w) * other.g0);
}

Point motor_plane_inner_product(Motor self, Plane other) {
    return Point(vec4(self.g1.x) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + self.g0.xxxw * other.g0.xxxw * vec4(0.0, 0.0, 0.0, -1.0));
}

Plane motor_plane_inner_anti_product(Motor self, Plane other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.w) * other.g0 + vec4(self.g1.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, 0.0));
}

Point motor_plane_left_contraction(Motor self, Plane other) {
    return Point(vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Point motor_plane_right_contraction(Motor self, Plane other) {
    return Point(self.g0.xxxw * other.g0.xxxw * vec4(0.0, 0.0, 0.0, -1.0));
}

Plane motor_plane_left_anti_contraction(Motor self, Plane other) {
    return Plane(vec4(self.g0.w) * other.g0);
}

Plane motor_plane_right_anti_contraction(Motor self, Plane other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, 0.0));
}

Motor motor_motor_add(Motor self, Motor other) {
    return Motor(self.g0 + other.g0, self.g1 + other.g1);
}

Motor motor_motor_sub(Motor self, Motor other) {
    return Motor(self.g0 - other.g0, self.g1 - other.g1);
}

Motor motor_motor_mul(Motor self, Motor other) {
    return Motor(self.g0 * other.g0, self.g1 * other.g1);
}

Motor motor_motor_div(Motor self, Motor other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.w) * vec4(1.0, 1.0, 1.0, 1.0), vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(1.0, 1.0, 1.0) / vec3(other.g1.x, other.g1.y, other.g1.z) * vec3(1.0, 1.0, 1.0));
}

MultiVector motor_motor_geometric_product(Motor self, Motor other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(0.0, -1.0) - vec2(self.g1.x) * vec2(other.g1.x, other.g0.x) - vec2(self.g1.y) * vec2(other.g1.y, other.g0.y) - vec2(self.g1.z) * vec2(other.g1.z, other.g0.z) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(0.0, -1.0), vec4(0.0), vec3(self.g0.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g0.w) * other.g1 + vec3(self.g1.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.y) * vec3(other.g0.z, other.g0.w, other.g0.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.w) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.x) * other.g1.xzy * vec3(0.0, 1.0, -1.0), vec3(self.g1.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g1.x) * other.g1.xzy * vec3(0.0, 1.0, -1.0), vec4(0.0));
}

MultiVector motor_motor_geometric_anti_product(Motor self, Motor other) {
    return MultiVector(vec2(0.0) - vec2(self.g0.x) * vec2(other.g1.x, other.g0.x) - vec2(self.g0.y) * vec2(other.g1.y, other.g0.y) - vec2(self.g0.z) * vec2(other.g1.z, other.g0.z) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g1.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x, self.g0.w) * vec2(other.g0.x, other.g0.w) * vec2(0.0, 1.0), vec4(0.0), vec3(self.g0.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g0.z, other.g0.w, other.g0.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.w) * vec3(-1.0, 1.0, 1.0) + vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.y) * other.g1.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * other.g1.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.w) * other.g1 + vec3(self.g1.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g0.z, other.g0.w, other.g0.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.w) * vec3(-1.0, 1.0, 1.0) + vec3(self.g0.x) * other.g1.xzy * vec3(0.0, -1.0, 1.0), vec4(0.0));
}

MultiVector motor_motor_regressive_product(Motor self, Motor other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g0.w) * vec2(other.g0.w) * vec2(0.0, 1.0) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g1.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w), vec3(self.g0.w) * other.g1 + self.g1 * vec3(other.g0.w), vec4(0.0));
}

MultiVector motor_motor_anti_wedge(Motor self, Motor other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g0.w) * vec2(other.g0.w) * vec2(0.0, 1.0) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g1.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w), vec3(self.g0.w) * other.g1 + self.g1 * vec3(other.g0.w), vec4(0.0));
}

MultiVector motor_motor_meet(Motor self, Motor other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g0.w) * vec2(other.g0.w) * vec2(0.0, 1.0) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g1.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w), vec3(self.g0.w) * other.g1 + self.g1 * vec3(other.g0.w), vec4(0.0));
}

AntiScalar motor_motor_outer_product(Motor self, Motor other) {
    return AntiScalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

AntiScalar motor_motor_wedge(Motor self, Motor other) {
    return AntiScalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

AntiScalar motor_motor_join(Motor self, Motor other) {
    return AntiScalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

MultiVector motor_motor_inner_product(Motor self, Motor other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g1.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.w) * other.g1 + self.g1 * vec3(other.g0.w), vec3(0.0), vec4(0.0));
}

Motor motor_motor_inner_anti_product(Motor self, Motor other) {
    return Motor(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g0 + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0), vec3(self.g0.w) * other.g1 + self.g1 * vec3(other.g0.w));
}

MultiVector motor_motor_left_contraction(Motor self, Motor other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g1.x) * vec2(-1.0, 0.0), vec4(0.0), self.g1 * vec3(other.g0.w), vec3(0.0), vec4(0.0));
}

MultiVector motor_motor_right_contraction(Motor self, Motor other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g1.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.w) * other.g1, vec3(0.0), vec4(0.0));
}

Motor motor_motor_left_anti_contraction(Motor self, Motor other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.w) * other.g0 + vec4(self.g0.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0), vec3(self.g0.w) * other.g1);
}

Motor motor_motor_right_anti_contraction(Motor self, Motor other) {
    return Motor(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0), self.g1 * vec3(other.g0.w));
}

Scalar motor_motor_scalar_product(Motor self, Motor other) {
    return Scalar(0.0 - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z);
}

Scalar motor_motor_dot(Motor self, Motor other) {
    return Scalar(0.0 - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z);
}

AntiScalar motor_motor_anti_scalar_product(Motor self, Motor other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z + self.g0.w * other.g0.w);
}

AntiScalar motor_motor_anti_dot(Motor self, Motor other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z + self.g0.w * other.g0.w);
}

Rotor motor_rotor_into(Motor self) {
    return Rotor(self.g0);
}

Motor motor_rotor_add(Motor self, Rotor other) {
    return Motor(self.g0 + other.g0, self.g1);
}

Motor motor_rotor_sub(Motor self, Rotor other) {
    return Motor(self.g0 - other.g0, self.g1);
}

Rotor motor_rotor_geometric_product(Motor self, Rotor other) {
    return Rotor(vec4(self.g1.x) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * other.g0.zwxy * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * other.g0.yxwz * vec4(1.0, -1.0, 1.0, -1.0));
}

MultiVector motor_rotor_geometric_anti_product(Motor self, Rotor other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g0.w) * vec2(other.g0.w) * vec2(0.0, 1.0) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g1.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(0.0, -1.0), vec4(0.0), vec3(self.g0.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g0.z, other.g0.w, other.g0.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.w) * vec3(-1.0, 1.0, 1.0) + vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g1.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g0.z, other.g0.w, other.g0.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.w) * vec3(-1.0, 1.0, 1.0), vec4(0.0));
}

MultiVector motor_rotor_regressive_product(Motor self, Rotor other) {
    return MultiVector(vec2(self.g1.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g1.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x, self.g0.w) * vec2(other.g0.x, other.g0.w) * vec2(0.0, 1.0), vec4(0.0), vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w), self.g1 * vec3(other.g0.w), vec4(0.0));
}

MultiVector motor_rotor_anti_wedge(Motor self, Rotor other) {
    return MultiVector(vec2(self.g1.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g1.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x, self.g0.w) * vec2(other.g0.x, other.g0.w) * vec2(0.0, 1.0), vec4(0.0), vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w), self.g1 * vec3(other.g0.w), vec4(0.0));
}

MultiVector motor_rotor_meet(Motor self, Rotor other) {
    return MultiVector(vec2(self.g1.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g1.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x, self.g0.w) * vec2(other.g0.x, other.g0.w) * vec2(0.0, 1.0), vec4(0.0), vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w), self.g1 * vec3(other.g0.w), vec4(0.0));
}

AntiScalar motor_rotor_outer_product(Motor self, Rotor other) {
    return AntiScalar(0.0 - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

AntiScalar motor_rotor_wedge(Motor self, Rotor other) {
    return AntiScalar(0.0 - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

AntiScalar motor_rotor_join(Motor self, Rotor other) {
    return AntiScalar(0.0 - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

Rotor motor_rotor_inner_product(Motor self, Rotor other) {
    return Rotor(vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.x) * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

Motor motor_rotor_inner_anti_product(Motor self, Rotor other) {
    return Motor(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g0 + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0), self.g1 * vec3(other.g0.w));
}

Rotor motor_rotor_left_contraction(Motor self, Rotor other) {
    return Rotor(vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.x) * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

Rotor motor_rotor_left_anti_contraction(Motor self, Rotor other) {
    return Rotor(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.w) * other.g0 + vec4(self.g0.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

Motor motor_rotor_right_anti_contraction(Motor self, Rotor other) {
    return Motor(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0), self.g1 * vec3(other.g0.w));
}

AntiScalar motor_rotor_anti_scalar_product(Motor self, Rotor other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z + self.g0.w * other.g0.w);
}

AntiScalar motor_rotor_anti_dot(Motor self, Rotor other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z + self.g0.w * other.g0.w);
}

Translator motor_translator_into(Motor self) {
    return Translator(vec4(self.g1.x, self.g1.y, self.g1.z, self.g0.w));
}

Motor motor_translator_add(Motor self, Translator other) {
    return Motor(self.g0 + other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0), self.g1 + vec3(other.g0.x, other.g0.y, other.g0.z));
}

Motor motor_translator_sub(Motor self, Translator other) {
    return Motor(self.g0 - other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0), self.g1 - vec3(other.g0.x, other.g0.y, other.g0.z));
}

MultiVector motor_translator_geometric_product(Motor self, Translator other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g1.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(0.0, -1.0), vec4(0.0), vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g1.x) * vec3(other.g0.w) * vec3(1.0, 0.0, 0.0) + vec3(self.g1.y) * vec3(other.g0.w) * vec3(0.0, 1.0, 0.0) + vec3(self.g1.z) * vec3(other.g0.w) * vec3(0.0, 0.0, 1.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0), vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g1.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0), vec4(0.0));
}

MultiVector motor_translator_geometric_anti_product(Motor self, Translator other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x, self.g0.w) * vec2(other.g0.x, other.g0.w) * vec2(-1.0, 1.0), vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w), vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g1.x) * vec3(other.g0.w) * vec3(1.0, 0.0, 0.0) + vec3(self.g1.y) * vec3(other.g0.w) * vec3(0.0, 1.0, 0.0) + vec3(self.g1.z) * vec3(other.g0.w) * vec3(0.0, 0.0, 1.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0), vec4(0.0));
}

MultiVector motor_translator_regressive_product(Motor self, Translator other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x, self.g0.w) * vec2(other.g0.x, other.g0.w) * vec2(-1.0, 1.0), vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w), vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g1 * vec3(other.g0.w), vec4(0.0));
}

MultiVector motor_translator_anti_wedge(Motor self, Translator other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x, self.g0.w) * vec2(other.g0.x, other.g0.w) * vec2(-1.0, 1.0), vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w), vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g1 * vec3(other.g0.w), vec4(0.0));
}

MultiVector motor_translator_meet(Motor self, Translator other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x, self.g0.w) * vec2(other.g0.x, other.g0.w) * vec2(-1.0, 1.0), vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w), vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g1 * vec3(other.g0.w), vec4(0.0));
}

AntiScalar motor_translator_outer_product(Motor self, Translator other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

AntiScalar motor_translator_wedge(Motor self, Translator other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

AntiScalar motor_translator_join(Motor self, Translator other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

MultiVector motor_translator_inner_product(Motor self, Translator other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g1 * vec3(other.g0.w), vec3(0.0), vec4(0.0));
}

Motor motor_translator_inner_anti_product(Motor self, Translator other) {
    return Motor(self.g0 * vec4(other.g0.w), vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g1 * vec3(other.g0.w));
}

MultiVector motor_translator_left_contraction(Motor self, Translator other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(-1.0, 0.0), vec4(0.0), self.g1 * vec3(other.g0.w), vec3(0.0), vec4(0.0));
}

MultiVector motor_translator_right_contraction(Motor self, Translator other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(0.0), vec4(0.0));
}

Translator motor_translator_left_anti_contraction(Motor self, Translator other) {
    return Translator(vec4(self.g0.w) * other.g0);
}

Motor motor_translator_right_anti_contraction(Motor self, Translator other) {
    return Motor(self.g0 * vec4(other.g0.w), self.g1 * vec3(other.g0.w));
}

Scalar motor_translator_scalar_product(Motor self, Translator other) {
    return Scalar(0.0 - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

Scalar motor_translator_dot(Motor self, Translator other) {
    return Scalar(0.0 - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

AntiScalar motor_translator_anti_scalar_product(Motor self, Translator other) {
    return AntiScalar(self.g0.w * other.g0.w);
}

AntiScalar motor_translator_anti_dot(Motor self, Translator other) {
    return AntiScalar(self.g0.w * other.g0.w);
}

Flector motor_flector_geometric_product(Motor self, Flector other) {
    return Flector(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g1.w) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.z, other.g1.w, other.g0.x, other.g1.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g1.w, other.g1.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0), vec4(self.g0.y) * vec4(other.g0.z, other.g1.w, other.g0.x, other.g0.z) * vec4(1.0, -1.0, -1.0, 0.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g1.w, other.g0.y) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g0.w) * other.g0.xyzx * vec4(-1.0, -1.0, -1.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.x) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4(-1.0, -1.0, 1.0, 0.0));
}

Flector motor_flector_geometric_anti_product(Motor self, Flector other) {
    return Flector(vec4(self.g0.x) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g0.z, other.g1.w, other.g0.x, other.g1.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g1.w, other.g1.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g0 + vec4(self.g1.y) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g1.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g1.y) * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, -1.0, 1.0, 0.0), vec4(self.g0.x) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g1 + vec4(self.g1.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

Flector motor_flector_regressive_product(Motor self, Flector other) {
    return Flector(vec4(self.g0.y) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g0 + vec4(self.g1.x) * other.g1.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g1.y) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g1.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g1.wxxx * vec4(1.0, 0.0, 0.0, -1.0), vec4(self.g0.w) * other.g1);
}

Flector motor_flector_anti_wedge(Motor self, Flector other) {
    return Flector(vec4(self.g0.y) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g0 + vec4(self.g1.x) * other.g1.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g1.y) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g1.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g1.wxxx * vec4(1.0, 0.0, 0.0, -1.0), vec4(self.g0.w) * other.g1);
}

Flector motor_flector_meet(Motor self, Flector other) {
    return Flector(vec4(self.g0.y) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g0 + vec4(self.g1.x) * other.g1.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g1.y) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g1.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g1.wxxx * vec4(1.0, 0.0, 0.0, -1.0), vec4(self.g0.w) * other.g1);
}

Plane motor_flector_outer_product(Motor self, Flector other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.x) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, 0.0));
}

Plane motor_flector_wedge(Motor self, Flector other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.x) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, 0.0));
}

Plane motor_flector_join(Motor self, Flector other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.x) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, 0.0));
}

Flector motor_flector_inner_product(Motor self, Flector other) {
    return Flector(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g1.w) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.z, other.g1.w, other.g0.x, other.g1.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g1.w, other.g1.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0), self.g0.wwwx * other.g0.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

Flector motor_flector_inner_anti_product(Motor self, Flector other) {
    return Flector(vec4(self.g0.w) * other.g0, vec4(self.g0.x) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g1 + vec4(self.g1.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

Point motor_flector_left_contraction(Motor self, Flector other) {
    return Point(vec4(self.g1.y) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.x) * other.g1.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Flector motor_flector_right_contraction(Motor self, Flector other) {
    return Flector(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g1.w) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.x) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0), self.g0.wwwx * other.g0.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

Flector motor_flector_left_anti_contraction(Motor self, Flector other) {
    return Flector(vec4(self.g0.w) * other.g0, vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g1 + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Plane motor_flector_right_anti_contraction(Motor self, Flector other) {
    return Plane(vec4(self.g0.y) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g1.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x) * other.g1.xzyx * vec4(0.0, -1.0, 1.0, 0.0));
}

MultiVector motor_multi_vector_add(Motor self, MultiVector other) {
    return MultiVector(vec2(self.g0.x, self.g0.w) * vec2(0.0, 1.0) + other.g0, other.g1, vec3(self.g0.x, self.g0.y, self.g0.z) + other.g2, self.g1 + other.g3, other.g4);
}

MultiVector motor_multi_vector_sub(Motor self, MultiVector other) {
    return MultiVector(vec2(self.g0.x, self.g0.w) * vec2(0.0, 1.0) - other.g0, vec4(0.0) - other.g1, vec3(self.g0.x, self.g0.y, self.g0.z) - other.g2, self.g1 - other.g3, vec4(0.0) - other.g4);
}

MultiVector motor_multi_vector_geometric_product(Motor self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g3.y) * vec2(0.0, -1.0) + vec2(self.g0.z) * vec2(other.g3.z) * vec2(0.0, -1.0) + vec2(self.g0.w) * vec2(other.g0.x) * vec2(0.0, 1.0) - vec2(self.g1.x) * vec2(other.g3.x, other.g2.x) - vec2(self.g1.y) * vec2(other.g3.y, other.g2.y) - vec2(self.g1.z) * vec2(other.g3.z, other.g2.z) + vec2(self.g0.x) * vec2(other.g3.x) * vec2(0.0, -1.0), vec4(self.g0.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g4.w) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g4.w, other.g1.z, other.g1.y, other.g4.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g1.z, other.g4.w, other.g1.x, other.g4.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g4.w, other.g4.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, 1.0), vec3(self.g0.x) * vec3(other.g0.x, other.g3.z, other.g3.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.y) * vec3(other.g3.z, other.g0.x, other.g3.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g0.z) * vec3(other.g3.y, other.g3.x, other.g0.x) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.w) * other.g3 + vec3(self.g1.x) * vec3(other.g0.y, other.g2.z, other.g2.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.y) * vec3(other.g2.z, other.g0.y, other.g2.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g1.z) * vec3(other.g2.y, other.g2.x, other.g0.y) * vec3(1.0, -1.0, 1.0), vec3(self.g1.x) * vec3(other.g0.x, other.g3.z, other.g3.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.y) * vec3(other.g3.z, other.g0.x, other.g3.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g1.z) * vec3(other.g3.y, other.g3.x, other.g0.x) * vec3(1.0, -1.0, 1.0), vec4(self.g0.y) * vec4(other.g1.z, other.g4.w, other.g1.x, other.g1.z) * vec4(1.0, -1.0, -1.0, 0.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g4.w, other.g1.y) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g0.w) * other.g1.xyzx * vec4(-1.0, -1.0, -1.0, 0.0) + vec4(self.g1.x) * vec4(other.g1.w, other.g4.z, other.g4.y, other.g1.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g4.z, other.g1.w, other.g4.x, other.g1.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g4.y, other.g4.x, other.g1.w, other.g1.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.x) * vec4(other.g4.w, other.g1.z, other.g1.y, other.g4.x) * vec4(-1.0, -1.0, 1.0, 0.0));
}

MultiVector motor_multi_vector_geometric_anti_product(Motor self, MultiVector other) {
    return MultiVector(vec2(0.0) - vec2(self.g0.x) * vec2(other.g3.x, other.g2.x) - vec2(self.g0.y) * vec2(other.g3.y, other.g2.y) - vec2(self.g0.z) * vec2(other.g3.z, other.g2.z) + vec2(self.g0.w) * other.g0 + vec2(self.g1.y) * vec2(other.g2.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g2.z) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g2.x) * vec2(-1.0, 0.0), vec4(self.g0.x) * vec4(other.g4.w, other.g1.z, other.g1.y, other.g4.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g1.z, other.g4.w, other.g1.x, other.g4.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g4.w, other.g4.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g1 + vec4(self.g1.y) * vec4(other.g4.z, other.g1.w, other.g4.x, other.g4.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g1.z) * vec4(other.g4.y, other.g4.x, other.g1.w, other.g4.y) * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g1.w, other.g4.z, other.g4.y, other.g1.x) * vec4(1.0, -1.0, 1.0, 0.0), vec3(self.g0.x) * vec3(other.g0.y, other.g2.z, other.g2.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g2.z, other.g0.y, other.g2.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g2.y, other.g2.x, other.g0.y) * vec3(-1.0, 1.0, 1.0) + vec3(self.g0.w) * other.g2, vec3(self.g0.x) * vec3(other.g0.x, other.g3.z, other.g3.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g3.z, other.g0.x, other.g3.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g3.y, other.g3.x, other.g0.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g0.w) * other.g3 + vec3(self.g1.x) * vec3(other.g0.y, other.g2.z, other.g2.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g2.z, other.g0.y, other.g2.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.z) * vec3(other.g2.y, other.g2.x, other.g0.y) * vec3(-1.0, 1.0, 1.0), vec4(self.g0.x) * vec4(other.g1.w, other.g4.z, other.g4.y, other.g1.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g4.z, other.g1.w, other.g4.x, other.g1.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g4.y, other.g4.x, other.g1.w, other.g1.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g4 + vec4(self.g1.y) * vec4(other.g4.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g4.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g4.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

MultiVector motor_multi_vector_regressive_product(Motor self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g3.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g3.z) * vec2(-1.0, 0.0) + vec2(self.g0.w) * other.g0 + vec2(self.g1.x) * vec2(other.g2.x) * vec2(-1.0, 0.0) + vec2(self.g1.y) * vec2(other.g2.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g2.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g3.x) * vec2(-1.0, 0.0), vec4(self.g0.y) * other.g4.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g4.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g1 + vec4(self.g1.x) * other.g4.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g1.y) * other.g4.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g4.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g4.wxxx * vec4(1.0, 0.0, 0.0, -1.0), vec3(self.g0.w) * other.g2 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), vec3(self.g0.w) * other.g3 + self.g1 * vec3(other.g0.y), vec4(self.g0.w) * other.g4);
}

MultiVector motor_multi_vector_anti_wedge(Motor self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g3.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g3.z) * vec2(-1.0, 0.0) + vec2(self.g0.w) * other.g0 + vec2(self.g1.x) * vec2(other.g2.x) * vec2(-1.0, 0.0) + vec2(self.g1.y) * vec2(other.g2.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g2.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g3.x) * vec2(-1.0, 0.0), vec4(self.g0.y) * other.g4.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g4.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g1 + vec4(self.g1.x) * other.g4.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g1.y) * other.g4.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g4.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g4.wxxx * vec4(1.0, 0.0, 0.0, -1.0), vec3(self.g0.w) * other.g2 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), vec3(self.g0.w) * other.g3 + self.g1 * vec3(other.g0.y), vec4(self.g0.w) * other.g4);
}

MultiVector motor_multi_vector_meet(Motor self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g3.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g3.z) * vec2(-1.0, 0.0) + vec2(self.g0.w) * other.g0 + vec2(self.g1.x) * vec2(other.g2.x) * vec2(-1.0, 0.0) + vec2(self.g1.y) * vec2(other.g2.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g2.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g3.x) * vec2(-1.0, 0.0), vec4(self.g0.y) * other.g4.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g4.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g1 + vec4(self.g1.x) * other.g4.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g1.y) * other.g4.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g4.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g4.wxxx * vec4(1.0, 0.0, 0.0, -1.0), vec3(self.g0.w) * other.g2 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), vec3(self.g0.w) * other.g3 + self.g1 * vec3(other.g0.y), vec4(self.g0.w) * other.g4);
}

MultiVector motor_multi_vector_outer_product(Motor self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g3.y) * vec2(0.0, -1.0) + vec2(self.g0.z) * vec2(other.g3.z) * vec2(0.0, -1.0) + vec2(self.g0.w) * vec2(other.g0.x) * vec2(0.0, 1.0) + vec2(self.g1.x) * vec2(other.g2.x) * vec2(0.0, -1.0) + vec2(self.g1.y) * vec2(other.g2.y) * vec2(0.0, -1.0) + vec2(self.g1.z) * vec2(other.g2.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g3.x) * vec2(0.0, -1.0), vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x), self.g1 * vec3(other.g0.x), vec4(self.g0.y) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g1.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.x) * other.g1.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g1.xzyx * vec4(0.0, -1.0, 1.0, 0.0));
}

MultiVector motor_multi_vector_wedge(Motor self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g3.y) * vec2(0.0, -1.0) + vec2(self.g0.z) * vec2(other.g3.z) * vec2(0.0, -1.0) + vec2(self.g0.w) * vec2(other.g0.x) * vec2(0.0, 1.0) + vec2(self.g1.x) * vec2(other.g2.x) * vec2(0.0, -1.0) + vec2(self.g1.y) * vec2(other.g2.y) * vec2(0.0, -1.0) + vec2(self.g1.z) * vec2(other.g2.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g3.x) * vec2(0.0, -1.0), vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x), self.g1 * vec3(other.g0.x), vec4(self.g0.y) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g1.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.x) * other.g1.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g1.xzyx * vec4(0.0, -1.0, 1.0, 0.0));
}

MultiVector motor_multi_vector_join(Motor self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g3.y) * vec2(0.0, -1.0) + vec2(self.g0.z) * vec2(other.g3.z) * vec2(0.0, -1.0) + vec2(self.g0.w) * vec2(other.g0.x) * vec2(0.0, 1.0) + vec2(self.g1.x) * vec2(other.g2.x) * vec2(0.0, -1.0) + vec2(self.g1.y) * vec2(other.g2.y) * vec2(0.0, -1.0) + vec2(self.g1.z) * vec2(other.g2.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g3.x) * vec2(0.0, -1.0), vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x), self.g1 * vec3(other.g0.x), vec4(self.g0.y) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g1.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.x) * other.g1.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g1.xzyx * vec4(0.0, -1.0, 1.0, 0.0));
}

MultiVector motor_multi_vector_inner_product(Motor self, MultiVector other) {
    return MultiVector(vec2(self.g1.x) * vec2(other.g3.x) * vec2(-1.0, 0.0) + vec2(self.g1.y) * vec2(other.g3.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g3.z) * vec2(-1.0, 0.0) + vec2(self.g0.x, self.g0.w) * vec2(other.g0.x) * vec2(0.0, 1.0), vec4(self.g0.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g4.w) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g4.w, other.g1.z, other.g1.y, other.g4.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g1.z, other.g4.w, other.g1.x, other.g4.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g4.w, other.g4.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, 1.0), vec3(self.g0.w) * other.g3 + vec3(self.g1.x) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g1.y) * vec3(other.g0.y) * vec3(0.0, 1.0, 0.0) + vec3(self.g1.z) * vec3(other.g0.y) * vec3(0.0, 0.0, 1.0) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x), self.g1 * vec3(other.g0.x), self.g0.wwwx * other.g1.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

MultiVector motor_multi_vector_inner_anti_product(Motor self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g2.y) * vec2(0.0, -1.0) + vec2(self.g0.z) * vec2(other.g2.z) * vec2(0.0, -1.0) + vec2(self.g0.w) * other.g0 + vec2(self.g0.x) * vec2(other.g2.x) * vec2(0.0, -1.0), vec4(self.g0.w) * other.g1, vec3(self.g0.w) * other.g2 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), vec3(self.g0.w) * other.g3 + vec3(self.g1.x) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g1.y) * vec3(other.g0.y) * vec3(0.0, 1.0, 0.0) + vec3(self.g1.z) * vec3(other.g0.y) * vec3(0.0, 0.0, 1.0) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x), vec4(self.g0.x) * vec4(other.g1.w, other.g4.z, other.g4.y, other.g1.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g4.z, other.g1.w, other.g4.x, other.g1.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g4.y, other.g4.x, other.g1.w, other.g1.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g4 + vec4(self.g1.y) * vec4(other.g4.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g4.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g4.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

MultiVector motor_multi_vector_left_contraction(Motor self, MultiVector other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g3.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g3.z) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g3.x) * vec2(-1.0, 0.0), vec4(self.g1.y) * other.g4.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g4.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.x) * other.g4.wxxx * vec4(1.0, 0.0, 0.0, -1.0), self.g1 * vec3(other.g0.y), vec3(0.0), vec4(0.0));
}

MultiVector motor_multi_vector_right_contraction(Motor self, MultiVector other) {
    return MultiVector(vec2(self.g1.x) * vec2(other.g3.x) * vec2(-1.0, 0.0) + vec2(self.g1.y) * vec2(other.g3.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g3.z) * vec2(-1.0, 0.0) + vec2(self.g0.x, self.g0.w) * vec2(other.g0.x) * vec2(0.0, 1.0), vec4(self.g0.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g4.w) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.x) * other.g1.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g1.y) * other.g1.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g1.z) * other.g1.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, 1.0), vec3(self.g0.w) * other.g3 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x), self.g1 * vec3(other.g0.x), self.g0.wwwx * other.g1.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

MultiVector motor_multi_vector_left_anti_contraction(Motor self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g2.y) * vec2(0.0, -1.0) + vec2(self.g0.z) * vec2(other.g2.z) * vec2(0.0, -1.0) + vec2(self.g0.w) * other.g0 + vec2(self.g0.x) * vec2(other.g2.x) * vec2(0.0, -1.0), vec4(self.g0.w) * other.g1, vec3(self.g0.w) * other.g2, vec3(self.g0.w) * other.g3 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x), vec4(self.g0.y) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g4 + vec4(self.g0.x) * other.g1.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

MultiVector motor_multi_vector_right_anti_contraction(Motor self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g2.y) * vec2(0.0, -1.0) + vec2(self.g0.z) * vec2(other.g2.z) * vec2(0.0, -1.0) + vec2(self.g0.w) * vec2(other.g0.y) * vec2(0.0, 1.0) + vec2(self.g0.x) * vec2(other.g2.x) * vec2(0.0, -1.0), vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), self.g1 * vec3(other.g0.y), vec4(self.g0.y) * other.g4.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g4.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.x) * vec4(other.g4.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * vec4(other.g4.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g4.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x) * other.g4.xzyx * vec4(0.0, -1.0, 1.0, 0.0));
}

Scalar motor_multi_vector_scalar_product(Motor self, MultiVector other) {
    return Scalar(0.0 - self.g1.x * other.g3.x - self.g1.y * other.g3.y - self.g1.z * other.g3.z);
}

Scalar motor_multi_vector_dot(Motor self, MultiVector other) {
    return Scalar(0.0 - self.g1.x * other.g3.x - self.g1.y * other.g3.y - self.g1.z * other.g3.z);
}

AntiScalar motor_multi_vector_anti_scalar_product(Motor self, MultiVector other) {
    return AntiScalar(0.0 - self.g0.x * other.g2.x - self.g0.y * other.g2.y - self.g0.z * other.g2.z + self.g0.w * other.g0.y);
}

AntiScalar motor_multi_vector_anti_dot(Motor self, MultiVector other) {
    return AntiScalar(0.0 - self.g0.x * other.g2.x - self.g0.y * other.g2.y - self.g0.z * other.g2.z + self.g0.w * other.g0.y);
}

Rotor rotor_scalar_geometric_product(Rotor self, Scalar other) {
    return Rotor(self.g0 * vec4(other.g0));
}

MultiVector rotor_scalar_geometric_anti_product(Rotor self, Scalar other) {
    return MultiVector(vec2(self.g0.w, self.g0.x) * vec2(other.g0) * vec2(1.0, 0.0), vec4(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0), vec4(0.0));
}

Scalar rotor_scalar_regressive_product(Rotor self, Scalar other) {
    return Scalar(self.g0.w * other.g0);
}

Scalar rotor_scalar_anti_wedge(Rotor self, Scalar other) {
    return Scalar(self.g0.w * other.g0);
}

Scalar rotor_scalar_meet(Rotor self, Scalar other) {
    return Scalar(self.g0.w * other.g0);
}

Rotor rotor_scalar_outer_product(Rotor self, Scalar other) {
    return Rotor(self.g0 * vec4(other.g0));
}

Rotor rotor_scalar_wedge(Rotor self, Scalar other) {
    return Rotor(self.g0 * vec4(other.g0));
}

Rotor rotor_scalar_join(Rotor self, Scalar other) {
    return Rotor(self.g0 * vec4(other.g0));
}

Rotor rotor_scalar_inner_product(Rotor self, Scalar other) {
    return Rotor(self.g0 * vec4(other.g0));
}

MultiVector rotor_scalar_inner_anti_product(Rotor self, Scalar other) {
    return MultiVector(vec2(self.g0.w, self.g0.x) * vec2(other.g0) * vec2(1.0, 0.0), vec4(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0), vec4(0.0));
}

Rotor rotor_scalar_right_contraction(Rotor self, Scalar other) {
    return Rotor(self.g0 * vec4(other.g0));
}

MultiVector rotor_scalar_left_anti_contraction(Rotor self, Scalar other) {
    return MultiVector(vec2(self.g0.w, self.g0.x) * vec2(other.g0) * vec2(1.0, 0.0), vec4(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0), vec4(0.0));
}

AntiScalar rotor_anti_scalar_into(Rotor self) {
    return AntiScalar(self.g0.w);
}

Rotor rotor_anti_scalar_add(Rotor self, AntiScalar other) {
    return Rotor(self.g0 + vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0));
}

Rotor rotor_anti_scalar_sub(Rotor self, AntiScalar other) {
    return Rotor(self.g0 - vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0));
}

Rotor rotor_anti_scalar_geometric_anti_product(Rotor self, AntiScalar other) {
    return Rotor(self.g0 * vec4(other.g0));
}

Rotor rotor_anti_scalar_regressive_product(Rotor self, AntiScalar other) {
    return Rotor(self.g0 * vec4(other.g0));
}

Rotor rotor_anti_scalar_anti_wedge(Rotor self, AntiScalar other) {
    return Rotor(self.g0 * vec4(other.g0));
}

Rotor rotor_anti_scalar_meet(Rotor self, AntiScalar other) {
    return Rotor(self.g0 * vec4(other.g0));
}

Rotor rotor_anti_scalar_inner_anti_product(Rotor self, AntiScalar other) {
    return Rotor(self.g0 * vec4(other.g0));
}

AntiScalar rotor_anti_scalar_left_anti_contraction(Rotor self, AntiScalar other) {
    return AntiScalar(self.g0.w * other.g0);
}

Rotor rotor_anti_scalar_right_anti_contraction(Rotor self, AntiScalar other) {
    return Rotor(self.g0 * vec4(other.g0));
}

AntiScalar rotor_anti_scalar_anti_scalar_product(Rotor self, AntiScalar other) {
    return AntiScalar(self.g0.w * other.g0);
}

AntiScalar rotor_anti_scalar_anti_dot(Rotor self, AntiScalar other) {
    return AntiScalar(self.g0.w * other.g0);
}

Rotor rotor_homogeneous_magnitude_geometric_product(Rotor self, HomogeneousMagnitude other) {
    return Rotor(self.g0 * vec4(other.g0.x));
}

MultiVector rotor_homogeneous_magnitude_geometric_anti_product(Rotor self, HomogeneousMagnitude other) {
    return MultiVector(vec2(self.g0.w) * other.g0, vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x), vec4(0.0));
}

MultiVector rotor_homogeneous_magnitude_regressive_product(Rotor self, HomogeneousMagnitude other) {
    return MultiVector(vec2(self.g0.w) * other.g0, vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), vec3(0.0), vec4(0.0));
}

MultiVector rotor_homogeneous_magnitude_anti_wedge(Rotor self, HomogeneousMagnitude other) {
    return MultiVector(vec2(self.g0.w) * other.g0, vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), vec3(0.0), vec4(0.0));
}

MultiVector rotor_homogeneous_magnitude_meet(Rotor self, HomogeneousMagnitude other) {
    return MultiVector(vec2(self.g0.w) * other.g0, vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), vec3(0.0), vec4(0.0));
}

Rotor rotor_homogeneous_magnitude_outer_product(Rotor self, HomogeneousMagnitude other) {
    return Rotor(self.g0 * vec4(other.g0.x));
}

Rotor rotor_homogeneous_magnitude_wedge(Rotor self, HomogeneousMagnitude other) {
    return Rotor(self.g0 * vec4(other.g0.x));
}

Rotor rotor_homogeneous_magnitude_join(Rotor self, HomogeneousMagnitude other) {
    return Rotor(self.g0 * vec4(other.g0.x));
}

Rotor rotor_homogeneous_magnitude_inner_product(Rotor self, HomogeneousMagnitude other) {
    return Rotor(self.g0 * vec4(other.g0.x));
}

MultiVector rotor_homogeneous_magnitude_inner_anti_product(Rotor self, HomogeneousMagnitude other) {
    return MultiVector(vec2(self.g0.w) * other.g0, vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x), vec4(0.0));
}

Rotor rotor_homogeneous_magnitude_right_contraction(Rotor self, HomogeneousMagnitude other) {
    return Rotor(self.g0 * vec4(other.g0.x));
}

MultiVector rotor_homogeneous_magnitude_left_anti_contraction(Rotor self, HomogeneousMagnitude other) {
    return MultiVector(vec2(self.g0.w) * other.g0, vec4(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x), vec4(0.0));
}

Rotor rotor_homogeneous_magnitude_right_anti_contraction(Rotor self, HomogeneousMagnitude other) {
    return Rotor(self.g0 * vec4(other.g0.y));
}

AntiScalar rotor_homogeneous_magnitude_anti_scalar_product(Rotor self, HomogeneousMagnitude other) {
    return AntiScalar(self.g0.w * other.g0.y);
}

AntiScalar rotor_homogeneous_magnitude_anti_dot(Rotor self, HomogeneousMagnitude other) {
    return AntiScalar(self.g0.w * other.g0.y);
}

Flector rotor_point_geometric_product(Rotor self, Point other) {
    return Flector(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0), vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.w) * other.g0.xyzx * vec4(-1.0, -1.0, -1.0, 0.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, 0.0));
}

Flector rotor_point_geometric_anti_product(Rotor self, Point other) {
    return Flector(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.w) * other.g0 + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, 0.0), vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Point rotor_point_regressive_product(Rotor self, Point other) {
    return Point(vec4(self.g0.w) * other.g0);
}

Point rotor_point_anti_wedge(Rotor self, Point other) {
    return Point(vec4(self.g0.w) * other.g0);
}

Point rotor_point_meet(Rotor self, Point other) {
    return Point(vec4(self.g0.w) * other.g0);
}

Plane rotor_point_outer_product(Rotor self, Point other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, 0.0));
}

Plane rotor_point_wedge(Rotor self, Point other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, 0.0));
}

Plane rotor_point_join(Rotor self, Point other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, 0.0));
}

Flector rotor_point_inner_product(Rotor self, Point other) {
    return Flector(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0), self.g0.wwwx * other.g0.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

Flector rotor_point_inner_anti_product(Rotor self, Point other) {
    return Flector(vec4(self.g0.w) * other.g0, vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Flector rotor_point_right_contraction(Rotor self, Point other) {
    return Flector(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0), self.g0.wwwx * other.g0.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

Flector rotor_point_left_anti_contraction(Rotor self, Point other) {
    return Flector(vec4(self.g0.w) * other.g0, vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Motor rotor_line_add(Rotor self, Line other) {
    return Motor(self.g0 + vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0), other.g1);
}

Motor rotor_line_sub(Rotor self, Line other) {
    return Motor(self.g0 - vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0), vec3(0.0) - other.g1);
}

Rotor rotor_line_geometric_product(Rotor self, Line other) {
    return Rotor(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

MultiVector rotor_line_geometric_anti_product(Rotor self, Line other) {
    return MultiVector(vec2(0.0) - vec2(self.g0.x) * vec2(other.g1.x, other.g0.x) - vec2(self.g0.y) * vec2(other.g1.y, other.g0.y) - vec2(self.g0.z) * vec2(other.g1.z, other.g0.z), vec4(0.0), vec3(self.g0.y) * other.g0.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * other.g0.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.w) * other.g0 + vec3(self.g0.x) * other.g0.xzy * vec3(0.0, -1.0, 1.0), vec3(self.g0.y) * other.g1.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * other.g1.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.w) * other.g1 + vec3(self.g0.x) * other.g1.xzy * vec3(0.0, -1.0, 1.0), vec4(0.0));
}

MultiVector rotor_line_regressive_product(Rotor self, Line other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.w) * other.g0, vec3(self.g0.w) * other.g1, vec4(0.0));
}

MultiVector rotor_line_anti_wedge(Rotor self, Line other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.w) * other.g0, vec3(self.g0.w) * other.g1, vec4(0.0));
}

MultiVector rotor_line_meet(Rotor self, Line other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.w) * other.g0, vec3(self.g0.w) * other.g1, vec4(0.0));
}

AntiScalar rotor_line_outer_product(Rotor self, Line other) {
    return AntiScalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z);
}

AntiScalar rotor_line_wedge(Rotor self, Line other) {
    return AntiScalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z);
}

AntiScalar rotor_line_join(Rotor self, Line other) {
    return AntiScalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z);
}

Rotor rotor_line_inner_product(Rotor self, Line other) {
    return Rotor(self.g0.wwwx * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Motor rotor_line_inner_anti_product(Rotor self, Line other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, -1.0), vec3(self.g0.w) * other.g1);
}

Rotor rotor_line_right_contraction(Rotor self, Line other) {
    return Rotor(self.g0.wwwx * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Motor rotor_line_left_anti_contraction(Rotor self, Line other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, -1.0), vec3(self.g0.w) * other.g1);
}

AntiScalar rotor_line_right_anti_contraction(Rotor self, Line other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

AntiScalar rotor_line_anti_scalar_product(Rotor self, Line other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

AntiScalar rotor_line_anti_dot(Rotor self, Line other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

Flector rotor_plane_geometric_product(Rotor self, Plane other) {
    return Flector(self.g0.xxxw * other.g0.xxxw * vec4(0.0, 0.0, 0.0, -1.0), self.g0.xyzx * other.g0.wwwx * vec4(-1.0, -1.0, -1.0, 0.0));
}

Flector rotor_plane_geometric_anti_product(Rotor self, Plane other) {
    return Flector(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0), vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.w) * other.g0 + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, 0.0));
}

Flector rotor_plane_regressive_product(Rotor self, Plane other) {
    return Flector(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0), vec4(self.g0.w) * other.g0);
}

Flector rotor_plane_anti_wedge(Rotor self, Plane other) {
    return Flector(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0), vec4(self.g0.w) * other.g0);
}

Flector rotor_plane_meet(Rotor self, Plane other) {
    return Flector(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0), vec4(self.g0.w) * other.g0);
}

Point rotor_plane_inner_product(Rotor self, Plane other) {
    return Point(self.g0.xxxw * other.g0.xxxw * vec4(0.0, 0.0, 0.0, -1.0));
}

Plane rotor_plane_inner_anti_product(Rotor self, Plane other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.w) * other.g0 + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, 0.0));
}

Point rotor_plane_right_contraction(Rotor self, Plane other) {
    return Point(self.g0.xxxw * other.g0.xxxw * vec4(0.0, 0.0, 0.0, -1.0));
}

Plane rotor_plane_left_anti_contraction(Rotor self, Plane other) {
    return Plane(vec4(self.g0.w) * other.g0);
}

Plane rotor_plane_right_anti_contraction(Rotor self, Plane other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, 0.0));
}

Motor rotor_motor_add(Rotor self, Motor other) {
    return Motor(self.g0 + other.g0, other.g1);
}

Motor rotor_motor_sub(Rotor self, Motor other) {
    return Motor(self.g0 - other.g0, vec3(0.0) - other.g1);
}

Rotor rotor_motor_geometric_product(Rotor self, Motor other) {
    return Rotor(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

MultiVector rotor_motor_geometric_anti_product(Rotor self, Motor other) {
    return MultiVector(vec2(0.0) - vec2(self.g0.x) * vec2(other.g1.x, other.g0.x) - vec2(self.g0.y) * vec2(other.g1.y, other.g0.y) - vec2(self.g0.z) * vec2(other.g1.z, other.g0.z) + vec2(self.g0.x, self.g0.w) * vec2(other.g0.x, other.g0.w) * vec2(0.0, 1.0), vec4(0.0), vec3(self.g0.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g0.z, other.g0.w, other.g0.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.w) * vec3(-1.0, 1.0, 1.0) + vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.y) * other.g1.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * other.g1.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.w) * other.g1 + vec3(self.g0.x) * other.g1.xzy * vec3(0.0, -1.0, 1.0), vec4(0.0));
}

MultiVector rotor_motor_regressive_product(Rotor self, Motor other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g0.w) * vec2(other.g0.w) * vec2(0.0, 1.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w), vec3(self.g0.w) * other.g1, vec4(0.0));
}

MultiVector rotor_motor_anti_wedge(Rotor self, Motor other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g0.w) * vec2(other.g0.w) * vec2(0.0, 1.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w), vec3(self.g0.w) * other.g1, vec4(0.0));
}

MultiVector rotor_motor_meet(Rotor self, Motor other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g0.w) * vec2(other.g0.w) * vec2(0.0, 1.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w), vec3(self.g0.w) * other.g1, vec4(0.0));
}

AntiScalar rotor_motor_outer_product(Rotor self, Motor other) {
    return AntiScalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z);
}

AntiScalar rotor_motor_wedge(Rotor self, Motor other) {
    return AntiScalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z);
}

AntiScalar rotor_motor_join(Rotor self, Motor other) {
    return AntiScalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z);
}

Rotor rotor_motor_inner_product(Rotor self, Motor other) {
    return Rotor(self.g0.wwwx * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Motor rotor_motor_inner_anti_product(Rotor self, Motor other) {
    return Motor(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g0 + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0), vec3(self.g0.w) * other.g1);
}

Rotor rotor_motor_right_contraction(Rotor self, Motor other) {
    return Rotor(self.g0.wwwx * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Motor rotor_motor_left_anti_contraction(Rotor self, Motor other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.w) * other.g0 + vec4(self.g0.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0), vec3(self.g0.w) * other.g1);
}

Rotor rotor_motor_right_anti_contraction(Rotor self, Motor other) {
    return Rotor(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

AntiScalar rotor_motor_anti_scalar_product(Rotor self, Motor other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z + self.g0.w * other.g0.w);
}

AntiScalar rotor_motor_anti_dot(Rotor self, Motor other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z + self.g0.w * other.g0.w);
}

Rotor rotor_rotor_add(Rotor self, Rotor other) {
    return Rotor(self.g0 + other.g0);
}

Rotor rotor_rotor_sub(Rotor self, Rotor other) {
    return Rotor(self.g0 - other.g0);
}

Rotor rotor_rotor_mul(Rotor self, Rotor other) {
    return Rotor(self.g0 * other.g0);
}

Rotor rotor_rotor_div(Rotor self, Rotor other) {
    return Rotor(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.w) * vec4(1.0, 1.0, 1.0, 1.0));
}

Rotor rotor_rotor_geometric_anti_product(Rotor self, Rotor other) {
    return Rotor(vec4(self.g0.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * other.g0.zwxy * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g0);
}

Rotor rotor_rotor_regressive_product(Rotor self, Rotor other) {
    return Rotor(vec4(self.g0.w) * other.g0 + self.g0.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

Rotor rotor_rotor_anti_wedge(Rotor self, Rotor other) {
    return Rotor(vec4(self.g0.w) * other.g0 + self.g0.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

Rotor rotor_rotor_meet(Rotor self, Rotor other) {
    return Rotor(vec4(self.g0.w) * other.g0 + self.g0.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

Rotor rotor_rotor_inner_anti_product(Rotor self, Rotor other) {
    return Rotor(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g0 + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Rotor rotor_rotor_left_anti_contraction(Rotor self, Rotor other) {
    return Rotor(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.w) * other.g0 + vec4(self.g0.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

Rotor rotor_rotor_right_anti_contraction(Rotor self, Rotor other) {
    return Rotor(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

AntiScalar rotor_rotor_anti_scalar_product(Rotor self, Rotor other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z + self.g0.w * other.g0.w);
}

AntiScalar rotor_rotor_anti_dot(Rotor self, Rotor other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z + self.g0.w * other.g0.w);
}

Motor rotor_translator_add(Rotor self, Translator other) {
    return Motor(self.g0 + other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0), vec3(other.g0.x, other.g0.y, other.g0.z));
}

Motor rotor_translator_sub(Rotor self, Translator other) {
    return Motor(self.g0 - other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0), vec3(0.0) - vec3(other.g0.x, other.g0.y, other.g0.z));
}

Rotor rotor_translator_geometric_product(Rotor self, Translator other) {
    return Rotor(vec4(self.g0.y) * other.g0.zzxy * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.yxyz * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, 1.0, -1.0, -1.0));
}

MultiVector rotor_translator_geometric_anti_product(Rotor self, Translator other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x, self.g0.w) * vec2(other.g0.x, other.g0.w) * vec2(-1.0, 1.0), vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w), vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0), vec4(0.0));
}

MultiVector rotor_translator_regressive_product(Rotor self, Translator other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x, self.g0.w) * vec2(other.g0.x, other.g0.w) * vec2(-1.0, 1.0), vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w), vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(0.0));
}

MultiVector rotor_translator_anti_wedge(Rotor self, Translator other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x, self.g0.w) * vec2(other.g0.x, other.g0.w) * vec2(-1.0, 1.0), vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w), vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(0.0));
}

MultiVector rotor_translator_meet(Rotor self, Translator other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x, self.g0.w) * vec2(other.g0.x, other.g0.w) * vec2(-1.0, 1.0), vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w), vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(0.0));
}

AntiScalar rotor_translator_outer_product(Rotor self, Translator other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

AntiScalar rotor_translator_wedge(Rotor self, Translator other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

AntiScalar rotor_translator_join(Rotor self, Translator other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

Rotor rotor_translator_inner_product(Rotor self, Translator other) {
    return Rotor(self.g0.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0));
}

Motor rotor_translator_inner_anti_product(Rotor self, Translator other) {
    return Motor(self.g0 * vec4(other.g0.w), vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z));
}

Rotor rotor_translator_right_contraction(Rotor self, Translator other) {
    return Rotor(self.g0.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0));
}

Translator rotor_translator_left_anti_contraction(Rotor self, Translator other) {
    return Translator(vec4(self.g0.w) * other.g0);
}

Rotor rotor_translator_right_anti_contraction(Rotor self, Translator other) {
    return Rotor(self.g0 * vec4(other.g0.w));
}

AntiScalar rotor_translator_anti_scalar_product(Rotor self, Translator other) {
    return AntiScalar(self.g0.w * other.g0.w);
}

AntiScalar rotor_translator_anti_dot(Rotor self, Translator other) {
    return AntiScalar(self.g0.w * other.g0.w);
}

Flector rotor_flector_geometric_product(Rotor self, Flector other) {
    return Flector(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g1.w) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0), vec4(self.g0.y) * vec4(other.g0.z, other.g1.w, other.g0.x, other.g0.z) * vec4(1.0, -1.0, -1.0, 0.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g1.w, other.g0.y) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g0.w) * other.g0.xyzx * vec4(-1.0, -1.0, -1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4(-1.0, -1.0, 1.0, 0.0));
}

Flector rotor_flector_geometric_anti_product(Rotor self, Flector other) {
    return Flector(vec4(self.g0.x) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g0.z, other.g1.w, other.g0.x, other.g1.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g1.w, other.g1.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g0, vec4(self.g0.x) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g1);
}

Flector rotor_flector_regressive_product(Rotor self, Flector other) {
    return Flector(vec4(self.g0.y) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g0 + vec4(self.g0.x) * other.g1.wxxx * vec4(1.0, 0.0, 0.0, -1.0), vec4(self.g0.w) * other.g1);
}

Flector rotor_flector_anti_wedge(Rotor self, Flector other) {
    return Flector(vec4(self.g0.y) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g0 + vec4(self.g0.x) * other.g1.wxxx * vec4(1.0, 0.0, 0.0, -1.0), vec4(self.g0.w) * other.g1);
}

Flector rotor_flector_meet(Rotor self, Flector other) {
    return Flector(vec4(self.g0.y) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g0 + vec4(self.g0.x) * other.g1.wxxx * vec4(1.0, 0.0, 0.0, -1.0), vec4(self.g0.w) * other.g1);
}

Plane rotor_flector_outer_product(Rotor self, Flector other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, 0.0));
}

Plane rotor_flector_wedge(Rotor self, Flector other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, 0.0));
}

Plane rotor_flector_join(Rotor self, Flector other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, 0.0));
}

Flector rotor_flector_inner_product(Rotor self, Flector other) {
    return Flector(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g1.w) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0), self.g0.wwwx * other.g0.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

Flector rotor_flector_inner_anti_product(Rotor self, Flector other) {
    return Flector(vec4(self.g0.w) * other.g0, vec4(self.g0.x) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g1);
}

Flector rotor_flector_right_contraction(Rotor self, Flector other) {
    return Flector(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g1.w) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0), self.g0.wwwx * other.g0.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

Flector rotor_flector_left_anti_contraction(Rotor self, Flector other) {
    return Flector(vec4(self.g0.w) * other.g0, vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g1 + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Plane rotor_flector_right_anti_contraction(Rotor self, Flector other) {
    return Plane(vec4(self.g0.y) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g1.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g1.xzyx * vec4(0.0, -1.0, 1.0, 0.0));
}

MultiVector rotor_multi_vector_add(Rotor self, MultiVector other) {
    return MultiVector(vec2(self.g0.x, self.g0.w) * vec2(0.0, 1.0) + other.g0, other.g1, vec3(self.g0.x, self.g0.y, self.g0.z) + other.g2, other.g3, other.g4);
}

MultiVector rotor_multi_vector_sub(Rotor self, MultiVector other) {
    return MultiVector(vec2(self.g0.x, self.g0.w) * vec2(0.0, 1.0) - other.g0, vec4(0.0) - other.g1, vec3(self.g0.x, self.g0.y, self.g0.z) - other.g2, vec3(0.0) - other.g3, vec4(0.0) - other.g4);
}

MultiVector rotor_multi_vector_geometric_product(Rotor self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g3.y) * vec2(0.0, -1.0) + vec2(self.g0.z) * vec2(other.g3.z) * vec2(0.0, -1.0) + vec2(self.g0.w) * vec2(other.g0.x) * vec2(0.0, 1.0) + vec2(self.g0.x) * vec2(other.g3.x) * vec2(0.0, -1.0), vec4(self.g0.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g4.w) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, 1.0), vec3(self.g0.x) * vec3(other.g0.x, other.g3.z, other.g3.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.y) * vec3(other.g3.z, other.g0.x, other.g3.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g0.z) * vec3(other.g3.y, other.g3.x, other.g0.x) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.w) * other.g3, vec3(0.0), vec4(self.g0.y) * vec4(other.g1.z, other.g4.w, other.g1.x, other.g1.z) * vec4(1.0, -1.0, -1.0, 0.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g4.w, other.g1.y) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g0.w) * other.g1.xyzx * vec4(-1.0, -1.0, -1.0, 0.0) + vec4(self.g0.x) * vec4(other.g4.w, other.g1.z, other.g1.y, other.g4.x) * vec4(-1.0, -1.0, 1.0, 0.0));
}

MultiVector rotor_multi_vector_geometric_anti_product(Rotor self, MultiVector other) {
    return MultiVector(vec2(0.0) - vec2(self.g0.x) * vec2(other.g3.x, other.g2.x) - vec2(self.g0.y) * vec2(other.g3.y, other.g2.y) - vec2(self.g0.z) * vec2(other.g3.z, other.g2.z) + vec2(self.g0.w) * other.g0, vec4(self.g0.x) * vec4(other.g4.w, other.g1.z, other.g1.y, other.g4.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g1.z, other.g4.w, other.g1.x, other.g4.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g4.w, other.g4.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g1, vec3(self.g0.x) * vec3(other.g0.y, other.g2.z, other.g2.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g2.z, other.g0.y, other.g2.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g2.y, other.g2.x, other.g0.y) * vec3(-1.0, 1.0, 1.0) + vec3(self.g0.w) * other.g2, vec3(self.g0.x) * vec3(other.g0.x, other.g3.z, other.g3.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g3.z, other.g0.x, other.g3.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g3.y, other.g3.x, other.g0.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g0.w) * other.g3, vec4(self.g0.x) * vec4(other.g1.w, other.g4.z, other.g4.y, other.g1.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g4.z, other.g1.w, other.g4.x, other.g1.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g4.y, other.g4.x, other.g1.w, other.g1.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g4);
}

MultiVector rotor_multi_vector_regressive_product(Rotor self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g3.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g3.z) * vec2(-1.0, 0.0) + vec2(self.g0.w) * other.g0 + vec2(self.g0.x) * vec2(other.g3.x) * vec2(-1.0, 0.0), vec4(self.g0.y) * other.g4.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g4.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g1 + vec4(self.g0.x) * other.g4.wxxx * vec4(1.0, 0.0, 0.0, -1.0), vec3(self.g0.w) * other.g2 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), vec3(self.g0.w) * other.g3, vec4(self.g0.w) * other.g4);
}

MultiVector rotor_multi_vector_anti_wedge(Rotor self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g3.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g3.z) * vec2(-1.0, 0.0) + vec2(self.g0.w) * other.g0 + vec2(self.g0.x) * vec2(other.g3.x) * vec2(-1.0, 0.0), vec4(self.g0.y) * other.g4.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g4.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g1 + vec4(self.g0.x) * other.g4.wxxx * vec4(1.0, 0.0, 0.0, -1.0), vec3(self.g0.w) * other.g2 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), vec3(self.g0.w) * other.g3, vec4(self.g0.w) * other.g4);
}

MultiVector rotor_multi_vector_meet(Rotor self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g3.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g3.z) * vec2(-1.0, 0.0) + vec2(self.g0.w) * other.g0 + vec2(self.g0.x) * vec2(other.g3.x) * vec2(-1.0, 0.0), vec4(self.g0.y) * other.g4.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g4.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g1 + vec4(self.g0.x) * other.g4.wxxx * vec4(1.0, 0.0, 0.0, -1.0), vec3(self.g0.w) * other.g2 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), vec3(self.g0.w) * other.g3, vec4(self.g0.w) * other.g4);
}

MultiVector rotor_multi_vector_outer_product(Rotor self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g3.y) * vec2(0.0, -1.0) + vec2(self.g0.z) * vec2(other.g3.z) * vec2(0.0, -1.0) + vec2(self.g0.w) * vec2(other.g0.x) * vec2(0.0, 1.0) + vec2(self.g0.x) * vec2(other.g3.x) * vec2(0.0, -1.0), vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x), vec3(0.0), vec4(self.g0.y) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g1.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g1.xzyx * vec4(0.0, -1.0, 1.0, 0.0));
}

MultiVector rotor_multi_vector_wedge(Rotor self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g3.y) * vec2(0.0, -1.0) + vec2(self.g0.z) * vec2(other.g3.z) * vec2(0.0, -1.0) + vec2(self.g0.w) * vec2(other.g0.x) * vec2(0.0, 1.0) + vec2(self.g0.x) * vec2(other.g3.x) * vec2(0.0, -1.0), vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x), vec3(0.0), vec4(self.g0.y) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g1.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g1.xzyx * vec4(0.0, -1.0, 1.0, 0.0));
}

MultiVector rotor_multi_vector_join(Rotor self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g3.y) * vec2(0.0, -1.0) + vec2(self.g0.z) * vec2(other.g3.z) * vec2(0.0, -1.0) + vec2(self.g0.w) * vec2(other.g0.x) * vec2(0.0, 1.0) + vec2(self.g0.x) * vec2(other.g3.x) * vec2(0.0, -1.0), vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x), vec3(0.0), vec4(self.g0.y) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g1.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g1.xzyx * vec4(0.0, -1.0, 1.0, 0.0));
}

MultiVector rotor_multi_vector_inner_product(Rotor self, MultiVector other) {
    return MultiVector(vec2(self.g0.x, self.g0.w) * vec2(other.g0.x) * vec2(0.0, 1.0), vec4(self.g0.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g4.w) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, 1.0), vec3(self.g0.w) * other.g3 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x), vec3(0.0), self.g0.wwwx * other.g1.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

MultiVector rotor_multi_vector_inner_anti_product(Rotor self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g2.y) * vec2(0.0, -1.0) + vec2(self.g0.z) * vec2(other.g2.z) * vec2(0.0, -1.0) + vec2(self.g0.w) * other.g0 + vec2(self.g0.x) * vec2(other.g2.x) * vec2(0.0, -1.0), vec4(self.g0.w) * other.g1, vec3(self.g0.w) * other.g2 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), vec3(self.g0.w) * other.g3 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x), vec4(self.g0.x) * vec4(other.g1.w, other.g4.z, other.g4.y, other.g1.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g4.z, other.g1.w, other.g4.x, other.g1.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g4.y, other.g4.x, other.g1.w, other.g1.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g4);
}

MultiVector rotor_multi_vector_right_contraction(Rotor self, MultiVector other) {
    return MultiVector(vec2(self.g0.x, self.g0.w) * vec2(other.g0.x) * vec2(0.0, 1.0), vec4(self.g0.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g4.w) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, 1.0), vec3(self.g0.w) * other.g3 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x), vec3(0.0), self.g0.wwwx * other.g1.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

MultiVector rotor_multi_vector_left_anti_contraction(Rotor self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g2.y) * vec2(0.0, -1.0) + vec2(self.g0.z) * vec2(other.g2.z) * vec2(0.0, -1.0) + vec2(self.g0.w) * other.g0 + vec2(self.g0.x) * vec2(other.g2.x) * vec2(0.0, -1.0), vec4(self.g0.w) * other.g1, vec3(self.g0.w) * other.g2, vec3(self.g0.w) * other.g3 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x), vec4(self.g0.y) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g4 + vec4(self.g0.x) * other.g1.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

MultiVector rotor_multi_vector_right_anti_contraction(Rotor self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g2.y) * vec2(0.0, -1.0) + vec2(self.g0.z) * vec2(other.g2.z) * vec2(0.0, -1.0) + vec2(self.g0.w) * vec2(other.g0.y) * vec2(0.0, 1.0) + vec2(self.g0.x) * vec2(other.g2.x) * vec2(0.0, -1.0), vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), vec3(0.0), vec4(self.g0.y) * other.g4.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g4.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g4.xzyx * vec4(0.0, -1.0, 1.0, 0.0));
}

AntiScalar rotor_multi_vector_anti_scalar_product(Rotor self, MultiVector other) {
    return AntiScalar(0.0 - self.g0.x * other.g2.x - self.g0.y * other.g2.y - self.g0.z * other.g2.z + self.g0.w * other.g0.y);
}

AntiScalar rotor_multi_vector_anti_dot(Rotor self, MultiVector other) {
    return AntiScalar(0.0 - self.g0.x * other.g2.x - self.g0.y * other.g2.y - self.g0.z * other.g2.z + self.g0.w * other.g0.y);
}

Translator translator_scalar_geometric_product(Translator self, Scalar other) {
    return Translator(self.g0 * vec4(other.g0));
}

Scalar translator_scalar_geometric_anti_product(Translator self, Scalar other) {
    return Scalar(self.g0.w * other.g0);
}

Scalar translator_scalar_regressive_product(Translator self, Scalar other) {
    return Scalar(self.g0.w * other.g0);
}

Scalar translator_scalar_anti_wedge(Translator self, Scalar other) {
    return Scalar(self.g0.w * other.g0);
}

Scalar translator_scalar_meet(Translator self, Scalar other) {
    return Scalar(self.g0.w * other.g0);
}

Translator translator_scalar_outer_product(Translator self, Scalar other) {
    return Translator(self.g0 * vec4(other.g0));
}

Translator translator_scalar_wedge(Translator self, Scalar other) {
    return Translator(self.g0 * vec4(other.g0));
}

Translator translator_scalar_join(Translator self, Scalar other) {
    return Translator(self.g0 * vec4(other.g0));
}

Translator translator_scalar_inner_product(Translator self, Scalar other) {
    return Translator(self.g0 * vec4(other.g0));
}

Scalar translator_scalar_inner_anti_product(Translator self, Scalar other) {
    return Scalar(self.g0.w * other.g0);
}

Translator translator_scalar_right_contraction(Translator self, Scalar other) {
    return Translator(self.g0 * vec4(other.g0));
}

Scalar translator_scalar_left_anti_contraction(Translator self, Scalar other) {
    return Scalar(self.g0.w * other.g0);
}

AntiScalar translator_anti_scalar_into(Translator self) {
    return AntiScalar(self.g0.w);
}

Translator translator_anti_scalar_add(Translator self, AntiScalar other) {
    return Translator(self.g0 + vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0));
}

Translator translator_anti_scalar_sub(Translator self, AntiScalar other) {
    return Translator(self.g0 - vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0));
}

Rotor translator_anti_scalar_geometric_product(Translator self, AntiScalar other) {
    return Rotor(self.g0.xyzx * vec4(other.g0) * vec4(1.0, 1.0, 1.0, 0.0));
}

Translator translator_anti_scalar_geometric_anti_product(Translator self, AntiScalar other) {
    return Translator(self.g0 * vec4(other.g0));
}

Translator translator_anti_scalar_regressive_product(Translator self, AntiScalar other) {
    return Translator(self.g0 * vec4(other.g0));
}

Translator translator_anti_scalar_anti_wedge(Translator self, AntiScalar other) {
    return Translator(self.g0 * vec4(other.g0));
}

Translator translator_anti_scalar_meet(Translator self, AntiScalar other) {
    return Translator(self.g0 * vec4(other.g0));
}

Rotor translator_anti_scalar_inner_product(Translator self, AntiScalar other) {
    return Rotor(self.g0.xyzx * vec4(other.g0) * vec4(1.0, 1.0, 1.0, 0.0));
}

Translator translator_anti_scalar_inner_anti_product(Translator self, AntiScalar other) {
    return Translator(self.g0 * vec4(other.g0));
}

Rotor translator_anti_scalar_left_contraction(Translator self, AntiScalar other) {
    return Rotor(self.g0.xyzx * vec4(other.g0) * vec4(1.0, 1.0, 1.0, 0.0));
}

AntiScalar translator_anti_scalar_left_anti_contraction(Translator self, AntiScalar other) {
    return AntiScalar(self.g0.w * other.g0);
}

Translator translator_anti_scalar_right_anti_contraction(Translator self, AntiScalar other) {
    return Translator(self.g0 * vec4(other.g0));
}

AntiScalar translator_anti_scalar_anti_scalar_product(Translator self, AntiScalar other) {
    return AntiScalar(self.g0.w * other.g0);
}

AntiScalar translator_anti_scalar_anti_dot(Translator self, AntiScalar other) {
    return AntiScalar(self.g0.w * other.g0);
}

Motor translator_homogeneous_magnitude_geometric_product(Translator self, HomogeneousMagnitude other) {
    return Motor(self.g0 * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x));
}

MultiVector translator_homogeneous_magnitude_geometric_anti_product(Translator self, HomogeneousMagnitude other) {
    return MultiVector(vec2(self.g0.w) * other.g0, vec4(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), vec4(0.0));
}

MultiVector translator_homogeneous_magnitude_regressive_product(Translator self, HomogeneousMagnitude other) {
    return MultiVector(vec2(self.g0.w) * other.g0, vec4(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), vec4(0.0));
}

MultiVector translator_homogeneous_magnitude_anti_wedge(Translator self, HomogeneousMagnitude other) {
    return MultiVector(vec2(self.g0.w) * other.g0, vec4(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), vec4(0.0));
}

MultiVector translator_homogeneous_magnitude_meet(Translator self, HomogeneousMagnitude other) {
    return MultiVector(vec2(self.g0.w) * other.g0, vec4(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), vec4(0.0));
}

Translator translator_homogeneous_magnitude_outer_product(Translator self, HomogeneousMagnitude other) {
    return Translator(self.g0 * vec4(other.g0.x));
}

Translator translator_homogeneous_magnitude_wedge(Translator self, HomogeneousMagnitude other) {
    return Translator(self.g0 * vec4(other.g0.x));
}

Translator translator_homogeneous_magnitude_join(Translator self, HomogeneousMagnitude other) {
    return Translator(self.g0 * vec4(other.g0.x));
}

Motor translator_homogeneous_magnitude_inner_product(Translator self, HomogeneousMagnitude other) {
    return Motor(self.g0 * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x));
}

MultiVector translator_homogeneous_magnitude_inner_anti_product(Translator self, HomogeneousMagnitude other) {
    return MultiVector(vec2(self.g0.w) * other.g0, vec4(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), vec4(0.0));
}

Rotor translator_homogeneous_magnitude_left_contraction(Translator self, HomogeneousMagnitude other) {
    return Rotor(self.g0.xyzx * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Translator translator_homogeneous_magnitude_right_contraction(Translator self, HomogeneousMagnitude other) {
    return Translator(self.g0 * vec4(other.g0.x));
}

HomogeneousMagnitude translator_homogeneous_magnitude_left_anti_contraction(Translator self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(vec2(self.g0.w) * other.g0);
}

Translator translator_homogeneous_magnitude_right_anti_contraction(Translator self, HomogeneousMagnitude other) {
    return Translator(self.g0 * vec4(other.g0.y));
}

AntiScalar translator_homogeneous_magnitude_anti_scalar_product(Translator self, HomogeneousMagnitude other) {
    return AntiScalar(self.g0.w * other.g0.y);
}

AntiScalar translator_homogeneous_magnitude_anti_dot(Translator self, HomogeneousMagnitude other) {
    return AntiScalar(self.g0.w * other.g0.y);
}

Flector translator_point_geometric_product(Translator self, Point other) {
    return Flector(vec4(self.g0.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, 1.0, -1.0, 0.0), vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g0.xyzx * vec4(-1.0, -1.0, -1.0, 0.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Point translator_point_geometric_anti_product(Translator self, Point other) {
    return Point(vec4(self.g0.w) * other.g0 + self.g0.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

Point translator_point_regressive_product(Translator self, Point other) {
    return Point(vec4(self.g0.w) * other.g0);
}

Point translator_point_anti_wedge(Translator self, Point other) {
    return Point(vec4(self.g0.w) * other.g0);
}

Point translator_point_meet(Translator self, Point other) {
    return Point(vec4(self.g0.w) * other.g0);
}

Plane translator_point_outer_product(Translator self, Point other) {
    return Plane(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Plane translator_point_wedge(Translator self, Point other) {
    return Plane(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Plane translator_point_join(Translator self, Point other) {
    return Plane(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Flector translator_point_inner_product(Translator self, Point other) {
    return Flector(vec4(self.g0.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, 1.0, -1.0, 0.0), self.g0.wwwx * other.g0.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

Point translator_point_inner_anti_product(Translator self, Point other) {
    return Point(vec4(self.g0.w) * other.g0);
}

Flector translator_point_right_contraction(Translator self, Point other) {
    return Flector(vec4(self.g0.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, 1.0, -1.0, 0.0), self.g0.wwwx * other.g0.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

Point translator_point_left_anti_contraction(Translator self, Point other) {
    return Point(vec4(self.g0.w) * other.g0);
}

Motor translator_line_add(Translator self, Line other) {
    return Motor(self.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0) + vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0), vec3(self.g0.x, self.g0.y, self.g0.z) + other.g1);
}

Motor translator_line_sub(Translator self, Line other) {
    return Motor(self.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0) - vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0), vec3(self.g0.x, self.g0.y, self.g0.z) - other.g1);
}

MultiVector translator_line_geometric_product(Translator self, Line other) {
    return MultiVector(vec2(0.0) - vec2(self.g0.x) * vec2(other.g1.x, other.g0.x) - vec2(self.g0.y) * vec2(other.g1.y, other.g0.y) - vec2(self.g0.z) * vec2(other.g1.z, other.g0.z), vec4(0.0), vec3(self.g0.y) * other.g0.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * other.g0.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g0.w) * other.g1 + vec3(self.g0.x) * other.g0.xzy * vec3(0.0, 1.0, -1.0), vec3(self.g0.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * other.g1.xzy * vec3(0.0, 1.0, -1.0), vec4(0.0));
}

MultiVector translator_line_geometric_anti_product(Translator self, Line other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.w) * other.g0, vec3(self.g0.y) * other.g0.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * other.g0.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.w) * other.g1 + vec3(self.g0.x) * other.g0.xzy * vec3(0.0, -1.0, 1.0), vec4(0.0));
}

MultiVector translator_line_regressive_product(Translator self, Line other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.w) * other.g0, vec3(self.g0.w) * other.g1, vec4(0.0));
}

MultiVector translator_line_anti_wedge(Translator self, Line other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.w) * other.g0, vec3(self.g0.w) * other.g1, vec4(0.0));
}

MultiVector translator_line_meet(Translator self, Line other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.w) * other.g0, vec3(self.g0.w) * other.g1, vec4(0.0));
}

AntiScalar translator_line_outer_product(Translator self, Line other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

AntiScalar translator_line_wedge(Translator self, Line other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

AntiScalar translator_line_join(Translator self, Line other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

MultiVector translator_line_inner_product(Translator self, Line other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.w) * other.g1, vec3(0.0), vec4(0.0));
}

Line translator_line_inner_anti_product(Translator self, Line other) {
    return Line(vec3(self.g0.w) * other.g0, vec3(self.g0.w) * other.g1);
}

Scalar translator_line_left_contraction(Translator self, Line other) {
    return Scalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z);
}

MultiVector translator_line_right_contraction(Translator self, Line other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.w) * other.g1, vec3(0.0), vec4(0.0));
}

Line translator_line_left_anti_contraction(Translator self, Line other) {
    return Line(vec3(self.g0.w) * other.g0, vec3(self.g0.w) * other.g1);
}

Scalar translator_line_scalar_product(Translator self, Line other) {
    return Scalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z);
}

Scalar translator_line_dot(Translator self, Line other) {
    return Scalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z);
}

Flector translator_plane_geometric_product(Translator self, Plane other) {
    return Flector(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0), vec4(self.g0.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, 1.0, -1.0, 0.0));
}

Flector translator_plane_geometric_anti_product(Translator self, Plane other) {
    return Flector(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, 0.0), vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.w) * other.g0 + vec4(self.g0.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

Flector translator_plane_regressive_product(Translator self, Plane other) {
    return Flector(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, 0.0), vec4(self.g0.w) * other.g0);
}

Flector translator_plane_anti_wedge(Translator self, Plane other) {
    return Flector(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, 0.0), vec4(self.g0.w) * other.g0);
}

Flector translator_plane_meet(Translator self, Plane other) {
    return Flector(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, 0.0), vec4(self.g0.w) * other.g0);
}

Point translator_plane_inner_product(Translator self, Plane other) {
    return Point(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Plane translator_plane_inner_anti_product(Translator self, Plane other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.w) * other.g0 + vec4(self.g0.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

Point translator_plane_left_contraction(Translator self, Plane other) {
    return Point(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Point translator_plane_right_contraction(Translator self, Plane other) {
    return Point(self.g0.xxxw * other.g0.xxxw * vec4(0.0, 0.0, 0.0, -1.0));
}

Plane translator_plane_left_anti_contraction(Translator self, Plane other) {
    return Plane(vec4(self.g0.w) * other.g0);
}

Plane translator_plane_right_anti_contraction(Translator self, Plane other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

Motor translator_motor_add(Translator self, Motor other) {
    return Motor(self.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0) + other.g0, vec3(self.g0.x, self.g0.y, self.g0.z) + other.g1);
}

Motor translator_motor_sub(Translator self, Motor other) {
    return Motor(self.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0) - other.g0, vec3(self.g0.x, self.g0.y, self.g0.z) - other.g1);
}

MultiVector translator_motor_geometric_product(Translator self, Motor other) {
    return MultiVector(vec2(0.0) - vec2(self.g0.x) * vec2(other.g1.x, other.g0.x) - vec2(self.g0.y) * vec2(other.g1.y, other.g0.y) - vec2(self.g0.z) * vec2(other.g1.z, other.g0.z), vec4(0.0), vec3(self.g0.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.y) * vec3(other.g0.z, other.g0.w, other.g0.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.w) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.w) * other.g1, vec3(self.g0.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * other.g1.xzy * vec3(0.0, 1.0, -1.0), vec4(0.0));
}

MultiVector translator_motor_geometric_anti_product(Translator self, Motor other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x, self.g0.w) * vec2(other.g0.x, other.g0.w) * vec2(-1.0, 1.0), vec4(0.0), vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g0.z, other.g0.w, other.g0.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.w) * vec3(-1.0, 1.0, 1.0) + vec3(self.g0.w) * other.g1, vec4(0.0));
}

MultiVector translator_motor_regressive_product(Translator self, Motor other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x, self.g0.w) * vec2(other.g0.x, other.g0.w) * vec2(-1.0, 1.0), vec4(0.0), vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.w) * other.g1 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w), vec4(0.0));
}

MultiVector translator_motor_anti_wedge(Translator self, Motor other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x, self.g0.w) * vec2(other.g0.x, other.g0.w) * vec2(-1.0, 1.0), vec4(0.0), vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.w) * other.g1 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w), vec4(0.0));
}

MultiVector translator_motor_meet(Translator self, Motor other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x, self.g0.w) * vec2(other.g0.x, other.g0.w) * vec2(-1.0, 1.0), vec4(0.0), vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.w) * other.g1 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w), vec4(0.0));
}

AntiScalar translator_motor_outer_product(Translator self, Motor other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

AntiScalar translator_motor_wedge(Translator self, Motor other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

AntiScalar translator_motor_join(Translator self, Motor other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

MultiVector translator_motor_inner_product(Translator self, Motor other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.w) * other.g1 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w), vec3(0.0), vec4(0.0));
}

Motor translator_motor_inner_anti_product(Translator self, Motor other) {
    return Motor(vec4(self.g0.w) * other.g0, vec3(self.g0.w) * other.g1 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w));
}

MultiVector translator_motor_left_contraction(Translator self, Motor other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w), vec3(0.0), vec4(0.0));
}

MultiVector translator_motor_right_contraction(Translator self, Motor other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.w) * other.g1, vec3(0.0), vec4(0.0));
}

Motor translator_motor_left_anti_contraction(Translator self, Motor other) {
    return Motor(vec4(self.g0.w) * other.g0, vec3(self.g0.w) * other.g1);
}

Translator translator_motor_right_anti_contraction(Translator self, Motor other) {
    return Translator(self.g0 * vec4(other.g0.w));
}

Scalar translator_motor_scalar_product(Translator self, Motor other) {
    return Scalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z);
}

Scalar translator_motor_dot(Translator self, Motor other) {
    return Scalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z);
}

AntiScalar translator_motor_anti_scalar_product(Translator self, Motor other) {
    return AntiScalar(self.g0.w * other.g0.w);
}

AntiScalar translator_motor_anti_dot(Translator self, Motor other) {
    return AntiScalar(self.g0.w * other.g0.w);
}

Motor translator_rotor_add(Translator self, Rotor other) {
    return Motor(self.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0) + other.g0, vec3(self.g0.x, self.g0.y, self.g0.z));
}

Motor translator_rotor_sub(Translator self, Rotor other) {
    return Motor(self.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0) - other.g0, vec3(self.g0.x, self.g0.y, self.g0.z));
}

Rotor translator_rotor_geometric_product(Translator self, Rotor other) {
    return Rotor(vec4(self.g0.x) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.y) * other.g0.zwxy * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.yxwz * vec4(1.0, -1.0, 1.0, -1.0));
}

MultiVector translator_rotor_geometric_anti_product(Translator self, Rotor other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x, self.g0.w) * vec2(other.g0.x, other.g0.w) * vec2(-1.0, 1.0), vec4(0.0), vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g0.z, other.g0.w, other.g0.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.w) * vec3(-1.0, 1.0, 1.0), vec4(0.0));
}

MultiVector translator_rotor_regressive_product(Translator self, Rotor other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x, self.g0.w) * vec2(other.g0.x, other.g0.w) * vec2(-1.0, 1.0), vec4(0.0), vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w), vec4(0.0));
}

MultiVector translator_rotor_anti_wedge(Translator self, Rotor other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x, self.g0.w) * vec2(other.g0.x, other.g0.w) * vec2(-1.0, 1.0), vec4(0.0), vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w), vec4(0.0));
}

MultiVector translator_rotor_meet(Translator self, Rotor other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x, self.g0.w) * vec2(other.g0.x, other.g0.w) * vec2(-1.0, 1.0), vec4(0.0), vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w), vec4(0.0));
}

AntiScalar translator_rotor_outer_product(Translator self, Rotor other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

AntiScalar translator_rotor_wedge(Translator self, Rotor other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

AntiScalar translator_rotor_join(Translator self, Rotor other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

Rotor translator_rotor_inner_product(Translator self, Rotor other) {
    return Rotor(self.g0.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

Motor translator_rotor_inner_anti_product(Translator self, Rotor other) {
    return Motor(vec4(self.g0.w) * other.g0, vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w));
}

Rotor translator_rotor_left_contraction(Translator self, Rotor other) {
    return Rotor(self.g0.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

Rotor translator_rotor_left_anti_contraction(Translator self, Rotor other) {
    return Rotor(vec4(self.g0.w) * other.g0);
}

Translator translator_rotor_right_anti_contraction(Translator self, Rotor other) {
    return Translator(self.g0 * vec4(other.g0.w));
}

AntiScalar translator_rotor_anti_scalar_product(Translator self, Rotor other) {
    return AntiScalar(self.g0.w * other.g0.w);
}

AntiScalar translator_rotor_anti_dot(Translator self, Rotor other) {
    return AntiScalar(self.g0.w * other.g0.w);
}

Translator translator_translator_add(Translator self, Translator other) {
    return Translator(self.g0 + other.g0);
}

Translator translator_translator_sub(Translator self, Translator other) {
    return Translator(self.g0 - other.g0);
}

Translator translator_translator_mul(Translator self, Translator other) {
    return Translator(self.g0 * other.g0);
}

Translator translator_translator_div(Translator self, Translator other) {
    return Translator(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.w) * vec4(1.0, 1.0, 1.0, 1.0));
}

MultiVector translator_translator_geometric_product(Translator self, Translator other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w), vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0), vec4(0.0));
}

Translator translator_translator_geometric_anti_product(Translator self, Translator other) {
    return Translator(vec4(self.g0.w) * other.g0 + self.g0.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

Translator translator_translator_regressive_product(Translator self, Translator other) {
    return Translator(vec4(self.g0.w) * other.g0 + self.g0.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

Translator translator_translator_anti_wedge(Translator self, Translator other) {
    return Translator(vec4(self.g0.w) * other.g0 + self.g0.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

Translator translator_translator_meet(Translator self, Translator other) {
    return Translator(vec4(self.g0.w) * other.g0 + self.g0.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

MultiVector translator_translator_inner_product(Translator self, Translator other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w), vec3(0.0), vec4(0.0));
}

Translator translator_translator_inner_anti_product(Translator self, Translator other) {
    return Translator(vec4(self.g0.w) * other.g0 + self.g0.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

MultiVector translator_translator_left_contraction(Translator self, Translator other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w), vec3(0.0), vec4(0.0));
}

MultiVector translator_translator_right_contraction(Translator self, Translator other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(0.0), vec4(0.0));
}

Translator translator_translator_left_anti_contraction(Translator self, Translator other) {
    return Translator(vec4(self.g0.w) * other.g0);
}

Translator translator_translator_right_anti_contraction(Translator self, Translator other) {
    return Translator(self.g0 * vec4(other.g0.w));
}

Scalar translator_translator_scalar_product(Translator self, Translator other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

Scalar translator_translator_dot(Translator self, Translator other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

AntiScalar translator_translator_anti_scalar_product(Translator self, Translator other) {
    return AntiScalar(self.g0.w * other.g0.w);
}

AntiScalar translator_translator_anti_dot(Translator self, Translator other) {
    return AntiScalar(self.g0.w * other.g0.w);
}

Flector translator_flector_geometric_product(Translator self, Flector other) {
    return Flector(vec4(self.g0.x) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.y) * vec4(other.g0.z, other.g1.w, other.g0.x, other.g1.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g1.w, other.g1.z) * vec4(1.0, -1.0, 1.0, -1.0) + self.g0.xxxw * other.g1.xxxw * vec4(0.0, 0.0, 0.0, -1.0), vec4(self.g0.x) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.y) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4(1.0, -1.0, 1.0, -1.0) + self.g0.wwwx * other.g0.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

Flector translator_flector_geometric_anti_product(Translator self, Flector other) {
    return Flector(vec4(self.g0.y) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g1.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g1.y) * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0 + vec4(self.g0.x) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, -1.0, 1.0, 0.0), vec4(self.g0.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.w) * other.g1 + vec4(self.g0.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

Flector translator_flector_regressive_product(Translator self, Flector other) {
    return Flector(vec4(self.g0.y) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g1.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.w) * other.g0 + vec4(self.g0.x) * other.g1.xzyx * vec4(0.0, -1.0, 1.0, 0.0), vec4(self.g0.w) * other.g1);
}

Flector translator_flector_anti_wedge(Translator self, Flector other) {
    return Flector(vec4(self.g0.y) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g1.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.w) * other.g0 + vec4(self.g0.x) * other.g1.xzyx * vec4(0.0, -1.0, 1.0, 0.0), vec4(self.g0.w) * other.g1);
}

Flector translator_flector_meet(Translator self, Flector other) {
    return Flector(vec4(self.g0.y) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g1.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.w) * other.g0 + vec4(self.g0.x) * other.g1.xzyx * vec4(0.0, -1.0, 1.0, 0.0), vec4(self.g0.w) * other.g1);
}

Plane translator_flector_outer_product(Translator self, Flector other) {
    return Plane(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Plane translator_flector_wedge(Translator self, Flector other) {
    return Plane(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Plane translator_flector_join(Translator self, Flector other) {
    return Plane(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Flector translator_flector_inner_product(Translator self, Flector other) {
    return Flector(vec4(self.g0.x) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.y) * vec4(other.g0.z, other.g1.w, other.g0.x, other.g1.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g1.w, other.g1.z) * vec4(1.0, -1.0, 1.0, -1.0) + self.g0.xxxw * other.g1.xxxw * vec4(0.0, 0.0, 0.0, -1.0), self.g0.wwwx * other.g0.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

Flector translator_flector_inner_anti_product(Translator self, Flector other) {
    return Flector(vec4(self.g0.w) * other.g0, vec4(self.g0.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.w) * other.g1 + vec4(self.g0.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

Point translator_flector_left_contraction(Translator self, Flector other) {
    return Point(vec4(self.g0.y) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g1.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Flector translator_flector_right_contraction(Translator self, Flector other) {
    return Flector(vec4(self.g0.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + self.g0.xxxw * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.w) * vec4(0.0, 1.0, -1.0, -1.0), self.g0.wwwx * other.g0.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

Flector translator_flector_left_anti_contraction(Translator self, Flector other) {
    return Flector(vec4(self.g0.w) * other.g0, vec4(self.g0.w) * other.g1);
}

Plane translator_flector_right_anti_contraction(Translator self, Flector other) {
    return Plane(vec4(self.g0.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

MultiVector translator_multi_vector_add(Translator self, MultiVector other) {
    return MultiVector(vec2(self.g0.x, self.g0.w) * vec2(0.0, 1.0) + other.g0, other.g1, other.g2, vec3(self.g0.x, self.g0.y, self.g0.z) + other.g3, other.g4);
}

MultiVector translator_multi_vector_sub(Translator self, MultiVector other) {
    return MultiVector(vec2(self.g0.x, self.g0.w) * vec2(0.0, 1.0) - other.g0, vec4(0.0) - other.g1, vec3(0.0) - other.g2, vec3(self.g0.x, self.g0.y, self.g0.z) - other.g3, vec4(0.0) - other.g4);
}

MultiVector translator_multi_vector_geometric_product(Translator self, MultiVector other) {
    return MultiVector(vec2(0.0) - vec2(self.g0.x) * vec2(other.g3.x, other.g2.x) - vec2(self.g0.y) * vec2(other.g3.y, other.g2.y) - vec2(self.g0.z) * vec2(other.g3.z, other.g2.z) + vec2(self.g0.x, self.g0.w) * vec2(other.g0.x) * vec2(0.0, 1.0), vec4(self.g0.x) * vec4(other.g4.w, other.g1.z, other.g1.y, other.g4.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.y) * vec4(other.g1.z, other.g4.w, other.g1.x, other.g4.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g4.w, other.g4.z) * vec4(1.0, -1.0, 1.0, -1.0) + self.g0.xxxw * other.g4.xxxw * vec4(0.0, 0.0, 0.0, -1.0), vec3(self.g0.x) * vec3(other.g0.y, other.g2.z, other.g2.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.y) * vec3(other.g2.z, other.g0.y, other.g2.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g0.z) * vec3(other.g2.y, other.g2.x, other.g0.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.w) * other.g3, vec3(self.g0.x) * vec3(other.g0.x, other.g3.z, other.g3.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.y) * vec3(other.g3.z, other.g0.x, other.g3.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g0.z) * vec3(other.g3.y, other.g3.x, other.g0.x) * vec3(1.0, -1.0, 1.0), vec4(self.g0.x) * vec4(other.g1.w, other.g4.z, other.g4.y, other.g1.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.y) * vec4(other.g4.z, other.g1.w, other.g4.x, other.g1.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g4.y, other.g4.x, other.g1.w, other.g1.z) * vec4(1.0, -1.0, 1.0, -1.0) + self.g0.wwwx * other.g1.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

MultiVector translator_multi_vector_geometric_anti_product(Translator self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g2.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g2.z) * vec2(-1.0, 0.0) + vec2(self.g0.w) * other.g0 + vec2(self.g0.x) * vec2(other.g2.x) * vec2(-1.0, 0.0), vec4(self.g0.y) * vec4(other.g4.z, other.g1.w, other.g4.x, other.g4.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g0.z) * vec4(other.g4.y, other.g4.x, other.g1.w, other.g4.y) * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g0.w) * other.g1 + vec4(self.g0.x) * vec4(other.g1.w, other.g4.z, other.g4.y, other.g1.x) * vec4(1.0, -1.0, 1.0, 0.0), vec3(self.g0.w) * other.g2, vec3(self.g0.x) * vec3(other.g0.y, other.g2.z, other.g2.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g2.z, other.g0.y, other.g2.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g2.y, other.g2.x, other.g0.y) * vec3(-1.0, 1.0, 1.0) + vec3(self.g0.w) * other.g3, vec4(self.g0.y) * vec4(other.g4.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g4.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.w) * other.g4 + vec4(self.g0.x) * vec4(other.g4.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

MultiVector translator_multi_vector_regressive_product(Translator self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g2.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g2.z) * vec2(-1.0, 0.0) + vec2(self.g0.w) * other.g0 + vec2(self.g0.x) * vec2(other.g2.x) * vec2(-1.0, 0.0), vec4(self.g0.y) * other.g4.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g4.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.w) * other.g1 + vec4(self.g0.x) * other.g4.xzyx * vec4(0.0, -1.0, 1.0, 0.0), vec3(self.g0.w) * other.g2, vec3(self.g0.w) * other.g3 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), vec4(self.g0.w) * other.g4);
}

MultiVector translator_multi_vector_anti_wedge(Translator self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g2.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g2.z) * vec2(-1.0, 0.0) + vec2(self.g0.w) * other.g0 + vec2(self.g0.x) * vec2(other.g2.x) * vec2(-1.0, 0.0), vec4(self.g0.y) * other.g4.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g4.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.w) * other.g1 + vec4(self.g0.x) * other.g4.xzyx * vec4(0.0, -1.0, 1.0, 0.0), vec3(self.g0.w) * other.g2, vec3(self.g0.w) * other.g3 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), vec4(self.g0.w) * other.g4);
}

MultiVector translator_multi_vector_meet(Translator self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g2.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g2.z) * vec2(-1.0, 0.0) + vec2(self.g0.w) * other.g0 + vec2(self.g0.x) * vec2(other.g2.x) * vec2(-1.0, 0.0), vec4(self.g0.y) * other.g4.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g4.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.w) * other.g1 + vec4(self.g0.x) * other.g4.xzyx * vec4(0.0, -1.0, 1.0, 0.0), vec3(self.g0.w) * other.g2, vec3(self.g0.w) * other.g3 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), vec4(self.g0.w) * other.g4);
}

MultiVector translator_multi_vector_outer_product(Translator self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g2.y) * vec2(0.0, -1.0) + vec2(self.g0.z) * vec2(other.g2.z) * vec2(0.0, -1.0) + vec2(self.g0.w) * vec2(other.g0.x) * vec2(0.0, 1.0) + vec2(self.g0.x) * vec2(other.g2.x) * vec2(0.0, -1.0), vec4(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x), vec4(self.g0.y) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g1.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

MultiVector translator_multi_vector_wedge(Translator self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g2.y) * vec2(0.0, -1.0) + vec2(self.g0.z) * vec2(other.g2.z) * vec2(0.0, -1.0) + vec2(self.g0.w) * vec2(other.g0.x) * vec2(0.0, 1.0) + vec2(self.g0.x) * vec2(other.g2.x) * vec2(0.0, -1.0), vec4(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x), vec4(self.g0.y) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g1.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

MultiVector translator_multi_vector_join(Translator self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g2.y) * vec2(0.0, -1.0) + vec2(self.g0.z) * vec2(other.g2.z) * vec2(0.0, -1.0) + vec2(self.g0.w) * vec2(other.g0.x) * vec2(0.0, 1.0) + vec2(self.g0.x) * vec2(other.g2.x) * vec2(0.0, -1.0), vec4(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x), vec4(self.g0.y) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g1.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

MultiVector translator_multi_vector_inner_product(Translator self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g3.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g3.z) * vec2(-1.0, 0.0) + vec2(self.g0.w) * vec2(other.g0.x) * vec2(0.0, 1.0) + vec2(self.g0.x) * vec2(other.g3.x) * vec2(-1.0, 0.0), vec4(self.g0.x) * vec4(other.g4.w, other.g1.z, other.g1.y, other.g4.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.y) * vec4(other.g1.z, other.g4.w, other.g1.x, other.g4.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g4.w, other.g4.z) * vec4(1.0, -1.0, 1.0, -1.0) + self.g0.xxxw * other.g4.xxxw * vec4(0.0, 0.0, 0.0, -1.0), vec3(self.g0.w) * other.g3 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x), self.g0.wwwx * other.g1.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

MultiVector translator_multi_vector_inner_anti_product(Translator self, MultiVector other) {
    return MultiVector(vec2(self.g0.w) * other.g0, vec4(self.g0.w) * other.g1, vec3(self.g0.w) * other.g2, vec3(self.g0.w) * other.g3 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), vec4(self.g0.y) * vec4(other.g4.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g4.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.w) * other.g4 + vec4(self.g0.x) * vec4(other.g4.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

MultiVector translator_multi_vector_left_contraction(Translator self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g3.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g3.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g3.x) * vec2(-1.0, 0.0), vec4(self.g0.y) * other.g4.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g4.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g4.wxxx * vec4(1.0, 0.0, 0.0, -1.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), vec3(0.0), vec4(0.0));
}

MultiVector translator_multi_vector_right_contraction(Translator self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g3.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g3.z) * vec2(-1.0, 0.0) + vec2(self.g0.w) * vec2(other.g0.x) * vec2(0.0, 1.0) + vec2(self.g0.x) * vec2(other.g3.x) * vec2(-1.0, 0.0), vec4(self.g0.y) * other.g1.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * other.g1.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + self.g0.xxxw * vec4(other.g1.x, other.g1.z, other.g1.y, other.g4.w) * vec4(0.0, 1.0, -1.0, -1.0), vec3(self.g0.w) * other.g3, vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x), self.g0.wwwx * other.g1.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

MultiVector translator_multi_vector_left_anti_contraction(Translator self, MultiVector other) {
    return MultiVector(vec2(self.g0.w) * other.g0, vec4(self.g0.w) * other.g1, vec3(self.g0.w) * other.g2, vec3(self.g0.w) * other.g3, vec4(self.g0.w) * other.g4);
}

MultiVector translator_multi_vector_right_anti_contraction(Translator self, MultiVector other) {
    return MultiVector(vec2(self.g0.x, self.g0.w) * other.g0 * vec2(0.0, 1.0), vec4(0.0), vec3(0.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), vec4(self.g0.y) * vec4(other.g4.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g4.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x) * vec4(other.g4.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

Scalar translator_multi_vector_scalar_product(Translator self, MultiVector other) {
    return Scalar(0.0 - self.g0.x * other.g3.x - self.g0.y * other.g3.y - self.g0.z * other.g3.z);
}

Scalar translator_multi_vector_dot(Translator self, MultiVector other) {
    return Scalar(0.0 - self.g0.x * other.g3.x - self.g0.y * other.g3.y - self.g0.z * other.g3.z);
}

AntiScalar translator_multi_vector_anti_scalar_product(Translator self, MultiVector other) {
    return AntiScalar(self.g0.w * other.g0.y);
}

AntiScalar translator_multi_vector_anti_dot(Translator self, MultiVector other) {
    return AntiScalar(self.g0.w * other.g0.y);
}

Flector flector_scalar_geometric_product(Flector self, Scalar other) {
    return Flector(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

Flector flector_scalar_geometric_anti_product(Flector self, Scalar other) {
    return Flector(self.g1.xyzx * vec4(other.g0) * vec4(-1.0, -1.0, -1.0, 0.0), self.g0.xxxw * vec4(other.g0) * vec4(0.0, 0.0, 0.0, -1.0));
}

Flector flector_scalar_outer_product(Flector self, Scalar other) {
    return Flector(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

Flector flector_scalar_wedge(Flector self, Scalar other) {
    return Flector(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

Flector flector_scalar_join(Flector self, Scalar other) {
    return Flector(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

Flector flector_scalar_inner_product(Flector self, Scalar other) {
    return Flector(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

Flector flector_scalar_inner_anti_product(Flector self, Scalar other) {
    return Flector(self.g1.xyzx * vec4(other.g0) * vec4(-1.0, -1.0, -1.0, 0.0), self.g0.xxxw * vec4(other.g0) * vec4(0.0, 0.0, 0.0, -1.0));
}

Flector flector_scalar_right_contraction(Flector self, Scalar other) {
    return Flector(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

Flector flector_scalar_left_anti_contraction(Flector self, Scalar other) {
    return Flector(self.g1.xyzx * vec4(other.g0) * vec4(-1.0, -1.0, -1.0, 0.0), self.g0.xxxw * vec4(other.g0) * vec4(0.0, 0.0, 0.0, -1.0));
}

Flector flector_anti_scalar_geometric_product(Flector self, AntiScalar other) {
    return Flector(self.g1.xxxw * vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0), self.g0.xyzx * vec4(other.g0) * vec4(1.0, 1.0, 1.0, 0.0));
}

Flector flector_anti_scalar_geometric_anti_product(Flector self, AntiScalar other) {
    return Flector(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

Flector flector_anti_scalar_regressive_product(Flector self, AntiScalar other) {
    return Flector(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

Flector flector_anti_scalar_anti_wedge(Flector self, AntiScalar other) {
    return Flector(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

Flector flector_anti_scalar_meet(Flector self, AntiScalar other) {
    return Flector(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

Flector flector_anti_scalar_inner_product(Flector self, AntiScalar other) {
    return Flector(self.g1.xxxw * vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0), self.g0.xyzx * vec4(other.g0) * vec4(1.0, 1.0, 1.0, 0.0));
}

Flector flector_anti_scalar_inner_anti_product(Flector self, AntiScalar other) {
    return Flector(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

Flector flector_anti_scalar_left_contraction(Flector self, AntiScalar other) {
    return Flector(self.g1.xxxw * vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0), self.g0.xyzx * vec4(other.g0) * vec4(1.0, 1.0, 1.0, 0.0));
}

Flector flector_anti_scalar_right_anti_contraction(Flector self, AntiScalar other) {
    return Flector(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

Flector flector_homogeneous_magnitude_geometric_product(Flector self, HomogeneousMagnitude other) {
    return Flector(vec4(self.g1.w) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + self.g0 * vec4(other.g0.x), vec4(self.g1.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g0.x, self.g0.y, self.g0.z, self.g1.w) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x));
}

Flector flector_homogeneous_magnitude_geometric_anti_product(Flector self, HomogeneousMagnitude other) {
    return Flector(vec4(self.g1.x) * vec4(other.g0.x) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.x) * vec4(0.0, -1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.x) * vec4(0.0, 0.0, -1.0, 0.0) + self.g0 * vec4(other.g0.y), vec4(self.g1.w) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x, self.g1.y, self.g1.z, self.g0.w) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x) * vec4(1.0, 1.0, 1.0, -1.0));
}

Flector flector_homogeneous_magnitude_regressive_product(Flector self, HomogeneousMagnitude other) {
    return Flector(self.g0 * vec4(other.g0.y), self.g1 * vec4(other.g0.y));
}

Flector flector_homogeneous_magnitude_anti_wedge(Flector self, HomogeneousMagnitude other) {
    return Flector(self.g0 * vec4(other.g0.y), self.g1 * vec4(other.g0.y));
}

Flector flector_homogeneous_magnitude_meet(Flector self, HomogeneousMagnitude other) {
    return Flector(self.g0 * vec4(other.g0.y), self.g1 * vec4(other.g0.y));
}

Flector flector_homogeneous_magnitude_outer_product(Flector self, HomogeneousMagnitude other) {
    return Flector(self.g0 * vec4(other.g0.x), self.g1 * vec4(other.g0.x));
}

Flector flector_homogeneous_magnitude_wedge(Flector self, HomogeneousMagnitude other) {
    return Flector(self.g0 * vec4(other.g0.x), self.g1 * vec4(other.g0.x));
}

Flector flector_homogeneous_magnitude_join(Flector self, HomogeneousMagnitude other) {
    return Flector(self.g0 * vec4(other.g0.x), self.g1 * vec4(other.g0.x));
}

Flector flector_homogeneous_magnitude_inner_product(Flector self, HomogeneousMagnitude other) {
    return Flector(vec4(self.g1.w) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + self.g0 * vec4(other.g0.x), vec4(self.g1.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g0.x, self.g0.y, self.g0.z, self.g1.w) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x));
}

Flector flector_homogeneous_magnitude_inner_anti_product(Flector self, HomogeneousMagnitude other) {
    return Flector(vec4(self.g1.x) * vec4(other.g0.x) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.x) * vec4(0.0, -1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.x) * vec4(0.0, 0.0, -1.0, 0.0) + self.g0 * vec4(other.g0.y), vec4(self.g1.w) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x, self.g1.y, self.g1.z, self.g0.w) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x) * vec4(1.0, 1.0, 1.0, -1.0));
}

Flector flector_homogeneous_magnitude_left_contraction(Flector self, HomogeneousMagnitude other) {
    return Flector(self.g1.xxxw * vec4(other.g0.x, other.g0.x, other.g0.x, other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0), self.g0.xyzx * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Flector flector_homogeneous_magnitude_right_contraction(Flector self, HomogeneousMagnitude other) {
    return Flector(self.g0 * vec4(other.g0.x), self.g1 * vec4(other.g0.x));
}

Flector flector_homogeneous_magnitude_left_anti_contraction(Flector self, HomogeneousMagnitude other) {
    return Flector(self.g1.xyzx * vec4(other.g0.x) * vec4(-1.0, -1.0, -1.0, 0.0), self.g0.xxxw * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

Flector flector_homogeneous_magnitude_right_anti_contraction(Flector self, HomogeneousMagnitude other) {
    return Flector(self.g0 * vec4(other.g0.y), self.g1 * vec4(other.g0.y));
}

Point flector_point_into(Flector self) {
    return Point(self.g0);
}

Flector flector_point_add(Flector self, Point other) {
    return Flector(self.g0 + other.g0, self.g1);
}

Flector flector_point_sub(Flector self, Point other) {
    return Flector(self.g0 - other.g0, self.g1);
}

MultiVector flector_point_geometric_product(Flector self, Point other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(1.0, 0.0) + vec2(self.g1.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g1.w) * vec2(other.g0.w) * vec2(0.0, -1.0) + vec2(self.g0.x, self.g1.x) * vec2(other.g0.x) * vec2(1.0, -1.0), vec4(0.0), vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g1.x) * vec3(other.g0.z, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0) + vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w) * vec3(-1.0), vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) - vec3(self.g1.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0), vec4(0.0));
}

MultiVector flector_point_geometric_anti_product(Flector self, Point other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g1.w) * vec2(other.g0.w) * vec2(-1.0, 0.0) + vec2(self.g1.x, self.g0.w) * vec2(other.g0.x, other.g0.w) * vec2(-1.0), vec4(0.0), vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g0.w) * vec3(-1.0), vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g1.x) * vec3(other.g0.z, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w) * vec3(-1.0), vec4(0.0));
}

Scalar flector_point_regressive_product(Flector self, Point other) {
    return Scalar(0.0 - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z - self.g1.w * other.g0.w);
}

Scalar flector_point_anti_wedge(Flector self, Point other) {
    return Scalar(0.0 - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z - self.g1.w * other.g0.w);
}

Scalar flector_point_meet(Flector self, Point other) {
    return Scalar(0.0 - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z - self.g1.w * other.g0.w);
}

Motor flector_point_outer_product(Flector self, Point other) {
    return Motor(vec4(self.g0.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x, self.g0.y, self.g0.z, self.g1.x) * other.g0.wwwx * vec4(-1.0), vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0));
}

Motor flector_point_wedge(Flector self, Point other) {
    return Motor(vec4(self.g0.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x, self.g0.y, self.g0.z, self.g1.x) * other.g0.wwwx * vec4(-1.0), vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0));
}

Motor flector_point_join(Flector self, Point other) {
    return Motor(vec4(self.g0.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x, self.g0.y, self.g0.z, self.g1.x) * other.g0.wwwx * vec4(-1.0), vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0));
}

MultiVector flector_point_inner_product(Flector self, Point other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(1.0, 0.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(1.0, 0.0), vec4(0.0), vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g1.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g1.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(0.0));
}

Motor flector_point_inner_anti_product(Flector self, Point other) {
    return Motor(vec4(self.g1.x, self.g1.y, self.g1.z, self.g0.w) * vec4(other.g0.w) * vec4(-1.0), vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0));
}

Scalar flector_point_left_contraction(Flector self, Point other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

MultiVector flector_point_right_contraction(Flector self, Point other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(1.0, 0.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(1.0, 0.0), vec4(0.0), vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g1.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g1.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(0.0));
}

Motor flector_point_left_anti_contraction(Flector self, Point other) {
    return Motor(vec4(self.g1.x, self.g1.y, self.g1.z, self.g0.w) * vec4(other.g0.w) * vec4(-1.0), vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0));
}

AntiScalar flector_point_right_anti_contraction(Flector self, Point other) {
    return AntiScalar(0.0 - self.g0.w * other.g0.w);
}

Scalar flector_point_scalar_product(Flector self, Point other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

Scalar flector_point_dot(Flector self, Point other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

AntiScalar flector_point_anti_scalar_product(Flector self, Point other) {
    return AntiScalar(0.0 - self.g0.w * other.g0.w);
}

AntiScalar flector_point_anti_dot(Flector self, Point other) {
    return AntiScalar(0.0 - self.g0.w * other.g0.w);
}

Flector flector_line_geometric_product(Flector self, Line other) {
    return Flector(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0), vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g1.z, other.g1.z, other.g1.y, other.g1.z) * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g1.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.z) * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.y) * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

Flector flector_line_geometric_anti_product(Flector self, Line other) {
    return Flector(vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g0.z) * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.y) * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(-1.0, -1.0, -1.0, 0.0) + vec4(self.g1.x) * vec4(other.g1.z, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g0.x) * vec4(0.0, -1.0, 1.0, 0.0), vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g0.z, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, -1.0, 1.0, 1.0) + vec4(self.g1.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(-1.0, 1.0, 0.0, 1.0) + self.g0.wwwx * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, -1.0));
}

Point flector_line_regressive_product(Flector self, Line other) {
    return Point(vec4(self.g1.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

Point flector_line_anti_wedge(Flector self, Line other) {
    return Point(vec4(self.g1.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

Point flector_line_meet(Flector self, Line other) {
    return Point(vec4(self.g1.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

Plane flector_line_outer_product(Flector self, Line other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

Plane flector_line_wedge(Flector self, Line other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

Plane flector_line_join(Flector self, Line other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

Point flector_line_inner_product(Flector self, Line other) {
    return Point(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

Plane flector_line_inner_anti_product(Flector self, Line other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g0.z, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, -1.0, 1.0, 1.0) + vec4(self.g1.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(-1.0, 1.0, 0.0, 1.0) + self.g0.wwwx * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, -1.0));
}

Point flector_line_left_contraction(Flector self, Line other) {
    return Point(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

Point flector_line_right_contraction(Flector self, Line other) {
    return Point(vec4(self.g1.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g1.wwwx * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, -1.0));
}

Plane flector_line_left_anti_contraction(Flector self, Line other) {
    return Plane(vec4(self.g1.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(-1.0, 1.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, -1.0, 1.0, 1.0));
}

Plane flector_line_right_anti_contraction(Flector self, Line other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, -1.0));
}

Plane flector_plane_into(Flector self) {
    return Plane(self.g1);
}

Flector flector_plane_add(Flector self, Plane other) {
    return Flector(self.g0, self.g1 + other.g0);
}

Flector flector_plane_sub(Flector self, Plane other) {
    return Flector(self.g0, self.g1 - other.g0);
}

MultiVector flector_plane_geometric_product(Flector self, Plane other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(0.0, 1.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(0.0, 1.0) + vec2(self.g0.w) * vec2(other.g0.w) * vec2(0.0, 1.0) + vec2(self.g1.w, self.g0.x) * vec2(other.g0.w, other.g0.x) * vec2(-1.0, 1.0), vec4(0.0), vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.y) * vec3(other.g0.w) * vec3(0.0, 1.0, 0.0) + vec3(self.g1.z) * vec3(other.g0.w) * vec3(0.0, 0.0, 1.0) - vec3(self.g1.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g1.x, self.g0.x, self.g0.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, -1.0, 1.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w) * vec3(-1.0), vec4(0.0));
}

MultiVector flector_plane_geometric_anti_product(Flector self, Plane other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(1.0, 0.0) + vec2(self.g0.w) * vec2(other.g0.w) * vec2(1.0, 0.0) + vec2(self.g1.y) * vec2(other.g0.y) * vec2(0.0, 1.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(0.0, 1.0) + vec2(self.g0.x, self.g1.x) * vec2(other.g0.x), vec4(0.0), vec3(0.0) - vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g1.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0), vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g1.y) * vec3(other.g0.w) * vec3(0.0, 1.0, 0.0) + vec3(self.g1.z) * vec3(other.g0.w) * vec3(0.0, 0.0, 1.0) - vec3(self.g1.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g1.x, self.g0.x, self.g0.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, 1.0, -1.0), vec4(0.0));
}

MultiVector flector_plane_regressive_product(Flector self, Plane other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(1.0, 0.0) + vec2(self.g0.w) * vec2(other.g0.w) * vec2(1.0, 0.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(1.0, 0.0), vec4(0.0), vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g1.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g1.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g0.w), vec4(0.0));
}

MultiVector flector_plane_anti_wedge(Flector self, Plane other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(1.0, 0.0) + vec2(self.g0.w) * vec2(other.g0.w) * vec2(1.0, 0.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(1.0, 0.0), vec4(0.0), vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g1.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g1.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g0.w), vec4(0.0));
}

MultiVector flector_plane_meet(Flector self, Plane other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(1.0, 0.0) + vec2(self.g0.w) * vec2(other.g0.w) * vec2(1.0, 0.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(1.0, 0.0), vec4(0.0), vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g1.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g1.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g0.w), vec4(0.0));
}

AntiScalar flector_plane_outer_product(Flector self, Plane other) {
    return AntiScalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z + self.g0.w * other.g0.w);
}

AntiScalar flector_plane_wedge(Flector self, Plane other) {
    return AntiScalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z + self.g0.w * other.g0.w);
}

AntiScalar flector_plane_join(Flector self, Plane other) {
    return AntiScalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z + self.g0.w * other.g0.w);
}

MultiVector flector_plane_inner_product(Flector self, Plane other) {
    return MultiVector(vec2(self.g1.w, self.g1.x) * vec2(other.g0.w, other.g0.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w) * vec3(-1.0), vec4(0.0));
}

Motor flector_plane_inner_anti_product(Flector self, Plane other) {
    return Motor(vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.w, self.g0.w, self.g0.w, self.g1.x) * other.g0.xyzx * vec4(-1.0, -1.0, -1.0, 1.0), vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0));
}

MultiVector flector_plane_left_contraction(Flector self, Plane other) {
    return MultiVector(vec2(self.g1.w, self.g1.x) * vec2(other.g0.w, other.g0.x) * vec2(-1.0, 0.0), vec4(0.0), vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w) * vec3(-1.0), vec4(0.0));
}

Scalar flector_plane_right_contraction(Flector self, Plane other) {
    return Scalar(0.0 - self.g1.w * other.g0.w);
}

AntiScalar flector_plane_left_anti_contraction(Flector self, Plane other) {
    return AntiScalar(self.g1.x * other.g0.x + self.g1.y * other.g0.y + self.g1.z * other.g0.z);
}

Motor flector_plane_right_anti_contraction(Flector self, Plane other) {
    return Motor(vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.w, self.g0.w, self.g0.w, self.g1.x) * other.g0.xyzx * vec4(-1.0, -1.0, -1.0, 1.0), vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0));
}

Scalar flector_plane_scalar_product(Flector self, Plane other) {
    return Scalar(0.0 - self.g1.w * other.g0.w);
}

Scalar flector_plane_dot(Flector self, Plane other) {
    return Scalar(0.0 - self.g1.w * other.g0.w);
}

AntiScalar flector_plane_anti_scalar_product(Flector self, Plane other) {
    return AntiScalar(self.g1.x * other.g0.x + self.g1.y * other.g0.y + self.g1.z * other.g0.z);
}

AntiScalar flector_plane_anti_dot(Flector self, Plane other) {
    return AntiScalar(self.g1.x * other.g0.x + self.g1.y * other.g0.y + self.g1.z * other.g0.z);
}

Flector flector_motor_geometric_product(Flector self, Motor other) {
    return Flector(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g0.w) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0), vec4(self.g0.x) * vec4(other.g0.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.y) * vec4(other.g0.z, other.g0.w, other.g0.x, other.g1.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.w, other.g1.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g1.x) * vec4(other.g1.z, other.g1.z, other.g1.y, other.g1.z) * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g1.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.z) * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.y) * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g1.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + self.g0.wwwx * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Flector flector_motor_geometric_anti_product(Flector self, Motor other) {
    return Flector(vec4(self.g0.y) * other.g0.zwxz * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxwy * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g0.w) * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g1.x) * vec4(other.g1.z, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, 0.0), vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g0.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g1.y) * vec4(other.g0.z, other.g0.w, other.g0.x, other.g1.y) * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g0.w, other.g1.z) * vec4(-1.0, 1.0, 1.0, 1.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + self.g0.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

Flector flector_motor_regressive_product(Flector self, Motor other) {
    return Flector(vec4(self.g1.x) * vec4(other.g1.z, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + self.g0 * vec4(other.g0.w), self.g1 * vec4(other.g0.w));
}

Flector flector_motor_anti_wedge(Flector self, Motor other) {
    return Flector(vec4(self.g1.x) * vec4(other.g1.z, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + self.g0 * vec4(other.g0.w), self.g1 * vec4(other.g0.w));
}

Flector flector_motor_meet(Flector self, Motor other) {
    return Flector(vec4(self.g1.x) * vec4(other.g1.z, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + self.g0 * vec4(other.g0.w), self.g1 * vec4(other.g0.w));
}

Plane flector_motor_outer_product(Flector self, Motor other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

Plane flector_motor_wedge(Flector self, Motor other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

Plane flector_motor_join(Flector self, Motor other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

Flector flector_motor_inner_product(Flector self, Motor other) {
    return Flector(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g0.w) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0), self.g0.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

Flector flector_motor_inner_anti_product(Flector self, Motor other) {
    return Flector(self.g0 * vec4(other.g0.w), vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g0.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g1.y) * vec4(other.g0.z, other.g0.w, other.g0.x, other.g1.y) * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g0.w, other.g1.z) * vec4(-1.0, 1.0, 1.0, 1.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + self.g0.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

Flector flector_motor_left_contraction(Flector self, Motor other) {
    return Flector(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0), self.g0.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

Point flector_motor_right_contraction(Flector self, Motor other) {
    return Point(vec4(self.g1.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g1.wwwx * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, -1.0));
}

Plane flector_motor_left_anti_contraction(Flector self, Motor other) {
    return Plane(vec4(self.g1.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(-1.0, 1.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, -1.0, 1.0, 1.0));
}

Flector flector_motor_right_anti_contraction(Flector self, Motor other) {
    return Flector(self.g0 * vec4(other.g0.w), vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.w) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.w) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + self.g0.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

Flector flector_rotor_geometric_product(Flector self, Rotor other) {
    return Flector(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0), vec4(self.g0.y) * other.g0.zwxz * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g0.z) * other.g0.yxwy * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g1.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 0.0));
}

Flector flector_rotor_geometric_anti_product(Flector self, Rotor other) {
    return Flector(vec4(self.g0.y) * other.g0.zwxz * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxwy * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + self.g0.xxxw * other.g0.wzyw * vec4(1.0, -1.0, 1.0, 1.0), vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.x) * other.g0.wzyw * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g1.y) * other.g0.zwxz * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g1.z) * other.g0.yxwy * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + self.g0.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

Flector flector_rotor_regressive_product(Flector self, Rotor other) {
    return Flector(vec4(self.g1.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + self.g0 * vec4(other.g0.w), self.g1 * vec4(other.g0.w));
}

Flector flector_rotor_anti_wedge(Flector self, Rotor other) {
    return Flector(vec4(self.g1.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + self.g0 * vec4(other.g0.w), self.g1 * vec4(other.g0.w));
}

Flector flector_rotor_meet(Flector self, Rotor other) {
    return Flector(vec4(self.g1.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + self.g0 * vec4(other.g0.w), self.g1 * vec4(other.g0.w));
}

Plane flector_rotor_outer_product(Flector self, Rotor other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, 1.0, -1.0, 0.0));
}

Plane flector_rotor_wedge(Flector self, Rotor other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, 1.0, -1.0, 0.0));
}

Plane flector_rotor_join(Flector self, Rotor other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, 1.0, -1.0, 0.0));
}

Flector flector_rotor_inner_product(Flector self, Rotor other) {
    return Flector(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0), self.g0.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

Flector flector_rotor_inner_anti_product(Flector self, Rotor other) {
    return Flector(self.g0 * vec4(other.g0.w), vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.x) * other.g0.wzyw * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g1.y) * other.g0.zwxz * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g1.z) * other.g0.yxwy * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + self.g0.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

Flector flector_rotor_left_contraction(Flector self, Rotor other) {
    return Flector(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0), self.g0.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

Plane flector_rotor_left_anti_contraction(Flector self, Rotor other) {
    return Plane(vec4(self.g1.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, 0.0));
}

Flector flector_rotor_right_anti_contraction(Flector self, Rotor other) {
    return Flector(self.g0 * vec4(other.g0.w), vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.w) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.w) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + self.g0.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

Flector flector_translator_geometric_product(Flector self, Translator other) {
    return Flector(vec4(self.g0.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.w) * other.g0 + vec4(self.g0.x, self.g0.x, self.g0.x, self.g1.x) * other.g0.xzyx * vec4(0.0, 1.0, -1.0, -1.0), vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g0.x, self.g1.x, self.g1.x, self.g0.x) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, -1.0));
}

Flector flector_translator_geometric_anti_product(Flector self, Translator other) {
    return Flector(vec4(self.g0.w) * other.g0 * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g1.x) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + self.g0.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0), vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, 1.0) + vec4(self.g1.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, 1.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, 1.0));
}

Flector flector_translator_regressive_product(Flector self, Translator other) {
    return Flector(vec4(self.g1.x) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + self.g0 * vec4(other.g0.w), self.g1 * vec4(other.g0.w));
}

Flector flector_translator_anti_wedge(Flector self, Translator other) {
    return Flector(vec4(self.g1.x) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + self.g0 * vec4(other.g0.w), self.g1 * vec4(other.g0.w));
}

Flector flector_translator_meet(Flector self, Translator other) {
    return Flector(vec4(self.g1.x) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + self.g0 * vec4(other.g0.w), self.g1 * vec4(other.g0.w));
}

Plane flector_translator_outer_product(Flector self, Translator other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

Plane flector_translator_wedge(Flector self, Translator other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

Plane flector_translator_join(Flector self, Translator other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

Flector flector_translator_inner_product(Flector self, Translator other) {
    return Flector(vec4(self.g0.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.w) * other.g0 + vec4(self.g0.x, self.g0.x, self.g0.x, self.g1.x) * other.g0.xzyx * vec4(0.0, 1.0, -1.0, -1.0), self.g0.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

Flector flector_translator_inner_anti_product(Flector self, Translator other) {
    return Flector(self.g0 * vec4(other.g0.w), vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, 1.0) + vec4(self.g1.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, 1.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, 1.0));
}

Flector flector_translator_left_contraction(Flector self, Translator other) {
    return Flector(vec4(self.g0.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g0.x, self.g0.x, self.g0.x, self.g1.w) * other.g0.xzyw * vec4(0.0, 1.0, -1.0, 1.0), self.g0.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

Point flector_translator_right_contraction(Flector self, Translator other) {
    return Point(vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g1.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

Plane flector_translator_left_anti_contraction(Flector self, Translator other) {
    return Plane(vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0));
}

Flector flector_translator_right_anti_contraction(Flector self, Translator other) {
    return Flector(self.g0 * vec4(other.g0.w), self.g1 * vec4(other.g0.w));
}

Flector flector_flector_add(Flector self, Flector other) {
    return Flector(self.g0 + other.g0, self.g1 + other.g1);
}

Flector flector_flector_sub(Flector self, Flector other) {
    return Flector(self.g0 - other.g0, self.g1 - other.g1);
}

Flector flector_flector_mul(Flector self, Flector other) {
    return Flector(self.g0 * other.g0, self.g1 * other.g1);
}

Flector flector_flector_div(Flector self, Flector other) {
    return Flector(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.w) * vec4(1.0, 1.0, 1.0, 1.0), vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.w) * vec4(1.0, 1.0, 1.0, 1.0));
}

MultiVector flector_flector_geometric_product(Flector self, Flector other) {
    return MultiVector(vec2(self.g0.x) * vec2(other.g0.x, other.g1.x) + vec2(self.g0.y) * vec2(other.g0.y, other.g1.y) + vec2(self.g0.z) * vec2(other.g0.z, other.g1.z) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g1.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(0.0, -1.0) - vec2(self.g1.w) * vec2(other.g1.w, other.g0.w) + vec2(self.g0.x, self.g0.w) * vec2(other.g1.x, other.g1.w) * vec2(0.0, 1.0), vec4(0.0), vec3(self.g0.x) * vec3(other.g0.w, other.g1.z, other.g1.y) * vec3(-1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g1.z, other.g0.w, other.g1.x) * vec3(1.0, -1.0, -1.0) + vec3(self.g0.z) * vec3(other.g1.y, other.g1.x, other.g0.w) * vec3(-1.0, 1.0, -1.0) + vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g1.x) * vec3(other.g1.w, other.g0.z, other.g0.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.y) * vec3(other.g0.z, other.g1.w, other.g0.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g1.w) * vec3(1.0, -1.0, 1.0) - vec3(self.g1.w) * vec3(other.g1.x, other.g1.y, other.g1.z), vec3(self.g0.x) * vec3(other.g1.w, other.g0.z, other.g0.y) * vec3(-1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g0.z, other.g1.w, other.g0.x) * vec3(1.0, -1.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g1.w) * vec3(-1.0, 1.0, -1.0) - vec3(self.g1.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(0.0));
}

MultiVector flector_flector_geometric_anti_product(Flector self, Flector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(1.0, 0.0) + vec2(self.g0.w) * vec2(other.g1.w, other.g0.w) * vec2(1.0, -1.0) + vec2(self.g1.x) * vec2(other.g0.x, other.g1.x) * vec2(-1.0, 1.0) + vec2(self.g1.y) * vec2(other.g0.y, other.g1.y) * vec2(-1.0, 1.0) + vec2(self.g1.z) * vec2(other.g0.z, other.g1.z) * vec2(-1.0, 1.0) + vec2(self.g1.w) * vec2(other.g0.w) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(1.0, 0.0), vec4(0.0), vec3(0.0) - vec3(self.g0.w) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g1.x) * vec3(other.g0.w, other.g1.z, other.g1.y) * vec3(-1.0, 1.0, -1.0) + vec3(self.g1.y) * vec3(other.g1.z, other.g0.w, other.g1.x) * vec3(-1.0, -1.0, 1.0) + vec3(self.g1.z) * vec3(other.g1.y, other.g1.x, other.g0.w) * vec3(1.0, -1.0, -1.0), vec3(self.g0.x) * vec3(other.g0.w, other.g1.z, other.g1.y) * vec3(-1.0, 1.0, -1.0) + vec3(self.g0.y) * vec3(other.g1.z, other.g0.w, other.g1.x) * vec3(-1.0, -1.0, 1.0) + vec3(self.g0.z) * vec3(other.g1.y, other.g1.x, other.g0.w) * vec3(1.0, -1.0, -1.0) + vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g1.x) * vec3(other.g1.w, other.g0.z, other.g0.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g0.z, other.g1.w, other.g0.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g1.w) * vec3(-1.0, 1.0, 1.0) - vec3(self.g1.w) * vec3(other.g1.x, other.g1.y, other.g1.z), vec4(0.0));
}

MultiVector flector_flector_regressive_product(Flector self, Flector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(1.0, 0.0) + vec2(self.g0.w) * vec2(other.g1.w) * vec2(1.0, 0.0) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g1.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g1.w) * vec2(other.g0.w) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(1.0, 0.0), vec4(0.0), vec3(self.g1.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g1.x) * vec3(other.g1.x, other.g1.z, other.g1.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g1.w) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g1.w), vec4(0.0));
}

MultiVector flector_flector_anti_wedge(Flector self, Flector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(1.0, 0.0) + vec2(self.g0.w) * vec2(other.g1.w) * vec2(1.0, 0.0) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g1.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g1.w) * vec2(other.g0.w) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(1.0, 0.0), vec4(0.0), vec3(self.g1.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g1.x) * vec3(other.g1.x, other.g1.z, other.g1.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g1.w) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g1.w), vec4(0.0));
}

MultiVector flector_flector_meet(Flector self, Flector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(1.0, 0.0) + vec2(self.g0.w) * vec2(other.g1.w) * vec2(1.0, 0.0) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g1.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g1.w) * vec2(other.g0.w) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(1.0, 0.0), vec4(0.0), vec3(self.g1.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g1.x) * vec3(other.g1.x, other.g1.z, other.g1.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g1.w) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g1.w), vec4(0.0));
}

Motor flector_flector_outer_product(Flector self, Flector other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.w, other.g0.w, other.g0.w, other.g1.y) * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.w, other.g0.w, other.g0.w, other.g1.z) * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.w) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x) * vec4(other.g0.w, other.g0.x, other.g0.x, other.g1.x) * vec4(-1.0, 0.0, 0.0, 1.0), vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0));
}

Motor flector_flector_wedge(Flector self, Flector other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.w, other.g0.w, other.g0.w, other.g1.y) * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.w, other.g0.w, other.g0.w, other.g1.z) * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.w) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x) * vec4(other.g0.w, other.g0.x, other.g0.x, other.g1.x) * vec4(-1.0, 0.0, 0.0, 1.0), vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0));
}

Motor flector_flector_join(Flector self, Flector other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.w, other.g0.w, other.g0.w, other.g1.y) * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.w, other.g0.w, other.g0.w, other.g1.z) * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.w) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x) * vec4(other.g0.w, other.g0.x, other.g0.x, other.g1.x) * vec4(-1.0, 0.0, 0.0, 1.0), vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0));
}

MultiVector flector_flector_inner_product(Flector self, Flector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(1.0, 0.0) + vec2(self.g1.w) * vec2(other.g1.w) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(1.0, 0.0), vec4(0.0), vec3(self.g0.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.x) * vec3(other.g0.z, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0) + vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * vec3(other.g1.x, other.g1.z, other.g1.y) * vec3(0.0, -1.0, 1.0), vec3(0.0) - vec3(self.g1.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.w) * vec3(-1.0), vec4(0.0));
}

Motor flector_flector_inner_anti_product(Flector self, Flector other) {
    return Motor(vec4(0.0) - vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g0.w) + vec4(self.g1.y) * vec4(other.g0.w, other.g0.w, other.g0.w, other.g1.y) * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g1.z) * vec4(other.g0.w, other.g0.w, other.g0.w, other.g1.z) * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g1.x) * vec4(other.g0.w, other.g0.x, other.g0.x, other.g1.x) * vec4(-1.0, 0.0, 0.0, 1.0), vec3(self.g0.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g1.x) * vec3(other.g0.z, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g1.x, other.g1.z, other.g1.y) * vec3(0.0, 1.0, -1.0));
}

MultiVector flector_flector_left_contraction(Flector self, Flector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(1.0, 0.0) + vec2(self.g1.w) * vec2(other.g1.w) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(1.0, 0.0), vec4(0.0), vec3(self.g0.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g1.x, other.g1.z, other.g1.y) * vec3(0.0, -1.0, 1.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.w) * vec3(-1.0), vec4(0.0));
}

MultiVector flector_flector_right_contraction(Flector self, Flector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g0.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(1.0, 0.0) + vec2(self.g1.w) * vec2(other.g1.w) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(1.0, 0.0), vec4(0.0), vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g1.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g1.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(0.0));
}

Motor flector_flector_left_anti_contraction(Flector self, Flector other) {
    return Motor(vec4(self.g1.x) * vec4(other.g0.w, other.g0.w, other.g0.w, other.g1.x) * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g1.y) * vec4(other.g0.w, other.g0.w, other.g0.w, other.g1.y) * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g1.z) * vec4(other.g0.w, other.g0.w, other.g0.w, other.g1.z) * vec4(0.0, 0.0, -1.0, 1.0) + self.g0.xxxw * other.g0.xxxw * vec4(0.0, 0.0, 0.0, -1.0), vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0));
}

Motor flector_flector_right_anti_contraction(Flector self, Flector other) {
    return Motor(vec4(0.0) - vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g0.w) + vec4(self.g1.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, 1.0), vec3(self.g0.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * vec3(other.g1.x, other.g1.z, other.g1.y) * vec3(0.0, 1.0, -1.0));
}

Scalar flector_flector_scalar_product(Flector self, Flector other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z - self.g1.w * other.g1.w);
}

Scalar flector_flector_dot(Flector self, Flector other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z - self.g1.w * other.g1.w);
}

AntiScalar flector_flector_anti_scalar_product(Flector self, Flector other) {
    return AntiScalar(0.0 - self.g0.w * other.g0.w + self.g1.x * other.g1.x + self.g1.y * other.g1.y + self.g1.z * other.g1.z);
}

AntiScalar flector_flector_anti_dot(Flector self, Flector other) {
    return AntiScalar(0.0 - self.g0.w * other.g0.w + self.g1.x * other.g1.x + self.g1.y * other.g1.y + self.g1.z * other.g1.z);
}

MultiVector flector_multi_vector_add(Flector self, MultiVector other) {
    return MultiVector(other.g0, self.g0 + other.g1, other.g2, other.g3, self.g1 + other.g4);
}

MultiVector flector_multi_vector_sub(Flector self, MultiVector other) {
    return MultiVector(vec2(0.0) - other.g0, self.g0 - other.g1, vec3(0.0) - other.g2, vec3(0.0) - other.g3, self.g1 - other.g4);
}

MultiVector flector_multi_vector_geometric_product(Flector self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * vec2(other.g1.x, other.g4.x) + vec2(self.g0.y) * vec2(other.g1.y, other.g4.y) + vec2(self.g0.z) * vec2(other.g1.z, other.g4.z) + vec2(self.g1.x) * vec2(other.g1.x) * vec2(0.0, -1.0) + vec2(self.g1.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g1.z) * vec2(other.g1.z) * vec2(0.0, -1.0) - vec2(self.g1.w) * vec2(other.g4.w, other.g1.w) + vec2(self.g0.x, self.g0.w) * vec2(other.g4.x, other.g4.w) * vec2(0.0, 1.0), vec4(self.g0.x) * vec4(other.g0.x, other.g3.z, other.g3.y, other.g2.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.y) * vec4(other.g3.z, other.g0.x, other.g3.x, other.g2.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g3.y, other.g3.x, other.g0.x, other.g2.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g1.x) * vec4(other.g3.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * vec4(other.g3.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g3.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g3.x, other.g3.y, other.g3.z, other.g0.y) + self.g0.xxxw * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0), vec3(self.g0.x) * vec3(other.g1.w, other.g4.z, other.g4.y) * vec3(-1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g4.z, other.g1.w, other.g4.x) * vec3(1.0, -1.0, -1.0) + vec3(self.g0.z) * vec3(other.g4.y, other.g4.x, other.g1.w) * vec3(-1.0, 1.0, -1.0) + vec3(self.g0.w) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g1.x) * vec3(other.g4.w, other.g1.z, other.g1.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.y) * vec3(other.g1.z, other.g4.w, other.g1.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g1.z) * vec3(other.g1.y, other.g1.x, other.g4.w) * vec3(1.0, -1.0, 1.0) - vec3(self.g1.w) * vec3(other.g4.x, other.g4.y, other.g4.z), vec3(self.g0.x) * vec3(other.g4.w, other.g1.z, other.g1.y) * vec3(-1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g1.z, other.g4.w, other.g1.x) * vec3(1.0, -1.0, -1.0) + vec3(self.g0.z) * vec3(other.g1.y, other.g1.x, other.g4.w) * vec3(-1.0, 1.0, -1.0) - vec3(self.g1.w) * vec3(other.g1.x, other.g1.y, other.g1.z), vec4(self.g0.x) * vec4(other.g0.y, other.g2.z, other.g2.y, other.g3.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.y) * vec4(other.g2.z, other.g0.y, other.g2.x, other.g3.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g2.y, other.g2.x, other.g0.y, other.g3.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g1.x) * vec4(other.g0.x, other.g3.z, other.g3.y, other.g0.x) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g1.y) * vec4(other.g3.z, other.g0.x, other.g3.x, other.g3.z) * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g1.z) * vec4(other.g3.y, other.g3.x, other.g0.x, other.g3.y) * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g2.x, other.g2.y, other.g2.z, other.g0.x) + self.g0.wwwx * vec4(other.g3.x, other.g3.y, other.g3.z, other.g3.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

MultiVector flector_multi_vector_geometric_anti_product(Flector self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g4.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g4.z) * vec2(1.0, 0.0) + vec2(self.g0.w) * vec2(other.g4.w, other.g1.w) * vec2(1.0, -1.0) + vec2(self.g1.x) * vec2(other.g1.x, other.g4.x) * vec2(-1.0, 1.0) + vec2(self.g1.y) * vec2(other.g1.y, other.g4.y) * vec2(-1.0, 1.0) + vec2(self.g1.z) * vec2(other.g1.z, other.g4.z) * vec2(-1.0, 1.0) + vec2(self.g1.w) * vec2(other.g1.w) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g4.x) * vec2(1.0, 0.0), vec4(self.g0.y) * vec4(other.g2.z, other.g0.y, other.g2.x, other.g2.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g0.z) * vec4(other.g2.y, other.g2.x, other.g0.y, other.g2.y) * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g0.w) * vec4(other.g3.x, other.g3.y, other.g3.z, other.g0.y) * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g1.x) * vec4(other.g0.x, other.g3.z, other.g3.y, other.g2.x) * vec4(-1.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g3.z, other.g0.x, other.g3.x, other.g2.y) * vec4(-1.0, -1.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g3.y, other.g3.x, other.g0.x, other.g2.z) * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g1.w) * vec4(other.g2.x, other.g2.y, other.g2.z, other.g2.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.y, other.g2.z, other.g2.y, other.g0.x) * vec4(1.0, -1.0, 1.0, 0.0), vec3(0.0) - vec3(self.g0.w) * vec3(other.g4.x, other.g4.y, other.g4.z) + vec3(self.g1.x) * vec3(other.g1.w, other.g4.z, other.g4.y) * vec3(-1.0, 1.0, -1.0) + vec3(self.g1.y) * vec3(other.g4.z, other.g1.w, other.g4.x) * vec3(-1.0, -1.0, 1.0) + vec3(self.g1.z) * vec3(other.g4.y, other.g4.x, other.g1.w) * vec3(1.0, -1.0, -1.0), vec3(self.g0.x) * vec3(other.g1.w, other.g4.z, other.g4.y) * vec3(-1.0, 1.0, -1.0) + vec3(self.g0.y) * vec3(other.g4.z, other.g1.w, other.g4.x) * vec3(-1.0, -1.0, 1.0) + vec3(self.g0.z) * vec3(other.g4.y, other.g4.x, other.g1.w) * vec3(1.0, -1.0, -1.0) + vec3(self.g0.w) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g1.x) * vec3(other.g4.w, other.g1.z, other.g1.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g1.z, other.g4.w, other.g1.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.z) * vec3(other.g1.y, other.g1.x, other.g4.w) * vec3(-1.0, 1.0, 1.0) - vec3(self.g1.w) * vec3(other.g4.x, other.g4.y, other.g4.z), vec4(self.g0.y) * vec4(other.g2.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g2.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g2.x, other.g2.y, other.g2.z, other.g0.x) * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g1.x) * vec4(other.g0.y, other.g2.z, other.g2.y, other.g3.x) * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g1.y) * vec4(other.g2.z, other.g0.y, other.g2.x, other.g3.y) * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g1.z) * vec4(other.g2.y, other.g2.x, other.g0.y, other.g3.z) * vec4(-1.0, 1.0, 1.0, 1.0) + vec4(self.g1.w) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g2.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

MultiVector flector_multi_vector_regressive_product(Flector self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g4.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g4.z) * vec2(1.0, 0.0) + vec2(self.g0.w) * vec2(other.g4.w) * vec2(1.0, 0.0) + vec2(self.g1.x) * vec2(other.g1.x) * vec2(-1.0, 0.0) + vec2(self.g1.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g1.w) * vec2(other.g1.w) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g4.x) * vec2(1.0, 0.0), vec4(self.g1.x) * vec4(other.g3.z, other.g3.z, other.g3.y, other.g2.x) * vec4(0.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g3.z, other.g3.z, other.g3.x, other.g2.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g3.y, other.g3.x, other.g3.y, other.g2.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g2.x, other.g2.y, other.g2.z, other.g2.x) * vec4(1.0, 1.0, 1.0, 0.0) + self.g0 * vec4(other.g0.y), vec3(self.g1.y) * vec3(other.g4.z, other.g4.z, other.g4.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * vec3(other.g4.y, other.g4.x, other.g4.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g1.x) * vec3(other.g4.x, other.g4.z, other.g4.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g1.w) * vec3(other.g4.x, other.g4.y, other.g4.z) + vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g4.w), self.g1 * vec4(other.g0.y));
}

MultiVector flector_multi_vector_anti_wedge(Flector self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g4.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g4.z) * vec2(1.0, 0.0) + vec2(self.g0.w) * vec2(other.g4.w) * vec2(1.0, 0.0) + vec2(self.g1.x) * vec2(other.g1.x) * vec2(-1.0, 0.0) + vec2(self.g1.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g1.w) * vec2(other.g1.w) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g4.x) * vec2(1.0, 0.0), vec4(self.g1.x) * vec4(other.g3.z, other.g3.z, other.g3.y, other.g2.x) * vec4(0.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g3.z, other.g3.z, other.g3.x, other.g2.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g3.y, other.g3.x, other.g3.y, other.g2.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g2.x, other.g2.y, other.g2.z, other.g2.x) * vec4(1.0, 1.0, 1.0, 0.0) + self.g0 * vec4(other.g0.y), vec3(self.g1.y) * vec3(other.g4.z, other.g4.z, other.g4.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * vec3(other.g4.y, other.g4.x, other.g4.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g1.x) * vec3(other.g4.x, other.g4.z, other.g4.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g1.w) * vec3(other.g4.x, other.g4.y, other.g4.z) + vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g4.w), self.g1 * vec4(other.g0.y));
}

MultiVector flector_multi_vector_meet(Flector self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g4.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g4.z) * vec2(1.0, 0.0) + vec2(self.g0.w) * vec2(other.g4.w) * vec2(1.0, 0.0) + vec2(self.g1.x) * vec2(other.g1.x) * vec2(-1.0, 0.0) + vec2(self.g1.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g1.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g1.w) * vec2(other.g1.w) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g4.x) * vec2(1.0, 0.0), vec4(self.g1.x) * vec4(other.g3.z, other.g3.z, other.g3.y, other.g2.x) * vec4(0.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g3.z, other.g3.z, other.g3.x, other.g2.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g3.y, other.g3.x, other.g3.y, other.g2.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g2.x, other.g2.y, other.g2.z, other.g2.x) * vec4(1.0, 1.0, 1.0, 0.0) + self.g0 * vec4(other.g0.y), vec3(self.g1.y) * vec3(other.g4.z, other.g4.z, other.g4.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * vec3(other.g4.y, other.g4.x, other.g4.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g1.x) * vec3(other.g4.x, other.g4.z, other.g4.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g1.w) * vec3(other.g4.x, other.g4.y, other.g4.z) + vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g4.w), self.g1 * vec4(other.g0.y));
}

MultiVector flector_multi_vector_outer_product(Flector self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g4.y) * vec2(0.0, 1.0) + vec2(self.g0.z) * vec2(other.g4.z) * vec2(0.0, 1.0) + vec2(self.g0.w) * vec2(other.g4.w) * vec2(0.0, 1.0) + vec2(self.g1.x) * vec2(other.g1.x) * vec2(0.0, -1.0) + vec2(self.g1.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g1.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g1.w) * vec2(other.g1.w) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g4.x) * vec2(0.0, 1.0), self.g0 * vec4(other.g0.x), vec3(self.g0.w) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.w) * vec3(-1.0), vec3(self.g0.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g1.x, other.g1.z, other.g1.y) * vec3(0.0, -1.0, 1.0), vec4(self.g0.y) * vec4(other.g2.z, other.g2.z, other.g2.x, other.g3.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g2.y, other.g2.x, other.g2.y, other.g3.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g3.x, other.g3.y, other.g3.z, other.g3.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g2.x, other.g2.z, other.g2.y, other.g3.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

MultiVector flector_multi_vector_wedge(Flector self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g4.y) * vec2(0.0, 1.0) + vec2(self.g0.z) * vec2(other.g4.z) * vec2(0.0, 1.0) + vec2(self.g0.w) * vec2(other.g4.w) * vec2(0.0, 1.0) + vec2(self.g1.x) * vec2(other.g1.x) * vec2(0.0, -1.0) + vec2(self.g1.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g1.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g1.w) * vec2(other.g1.w) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g4.x) * vec2(0.0, 1.0), self.g0 * vec4(other.g0.x), vec3(self.g0.w) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.w) * vec3(-1.0), vec3(self.g0.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g1.x, other.g1.z, other.g1.y) * vec3(0.0, -1.0, 1.0), vec4(self.g0.y) * vec4(other.g2.z, other.g2.z, other.g2.x, other.g3.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g2.y, other.g2.x, other.g2.y, other.g3.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g3.x, other.g3.y, other.g3.z, other.g3.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g2.x, other.g2.z, other.g2.y, other.g3.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

MultiVector flector_multi_vector_join(Flector self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g4.y) * vec2(0.0, 1.0) + vec2(self.g0.z) * vec2(other.g4.z) * vec2(0.0, 1.0) + vec2(self.g0.w) * vec2(other.g4.w) * vec2(0.0, 1.0) + vec2(self.g1.x) * vec2(other.g1.x) * vec2(0.0, -1.0) + vec2(self.g1.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g1.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g1.w) * vec2(other.g1.w) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g4.x) * vec2(0.0, 1.0), self.g0 * vec4(other.g0.x), vec3(self.g0.w) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.w) * vec3(-1.0), vec3(self.g0.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g1.x, other.g1.z, other.g1.y) * vec3(0.0, -1.0, 1.0), vec4(self.g0.y) * vec4(other.g2.z, other.g2.z, other.g2.x, other.g3.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g2.y, other.g2.x, other.g2.y, other.g3.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g3.x, other.g3.y, other.g3.z, other.g3.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g2.x, other.g2.z, other.g2.y, other.g3.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

MultiVector flector_multi_vector_inner_product(Flector self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(1.0, 0.0) + vec2(self.g1.w) * vec2(other.g4.w) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(1.0, 0.0), vec4(self.g0.x) * vec4(other.g0.x, other.g3.z, other.g3.y, other.g2.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.y) * vec4(other.g3.z, other.g0.x, other.g3.x, other.g2.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g3.y, other.g3.x, other.g0.x, other.g2.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g1.x) * vec4(other.g3.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * vec4(other.g3.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g3.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g3.x, other.g3.y, other.g3.z, other.g0.y) + self.g0.xxxw * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0), vec3(self.g0.y) * vec3(other.g4.z, other.g4.z, other.g4.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g4.y, other.g4.x, other.g4.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.x) * vec3(other.g1.z, other.g1.z, other.g1.y) * vec3(0.0, 1.0, -1.0) + vec3(self.g1.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * vec3(other.g4.x, other.g4.z, other.g4.y) * vec3(0.0, -1.0, 1.0), vec3(0.0) - vec3(self.g1.w) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g4.w) * vec3(-1.0), vec4(self.g1.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g0.x, self.g0.y, self.g0.z, self.g1.w) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x));
}

MultiVector flector_multi_vector_inner_anti_product(Flector self, MultiVector other) {
    return MultiVector(vec2(self.g1.x) * vec2(other.g4.x) * vec2(0.0, 1.0) + vec2(self.g1.y) * vec2(other.g4.y) * vec2(0.0, 1.0) + vec2(self.g1.z) * vec2(other.g4.z) * vec2(0.0, 1.0) + vec2(self.g0.x, self.g0.w) * vec2(other.g1.x, other.g1.w) * vec2(0.0, -1.0), vec4(self.g1.x) * vec4(other.g0.x) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.x) * vec4(0.0, -1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.x) * vec4(0.0, 0.0, -1.0, 0.0) + self.g0 * vec4(other.g0.y), vec3(0.0) - vec3(self.g0.w) * vec3(other.g4.x, other.g4.y, other.g4.z) + vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g1.w) * vec3(-1.0), vec3(self.g0.y) * vec3(other.g4.z, other.g4.z, other.g4.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * vec3(other.g4.y, other.g4.x, other.g4.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g1.x) * vec3(other.g1.z, other.g1.z, other.g1.y) * vec3(0.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g4.x, other.g4.z, other.g4.y) * vec3(0.0, 1.0, -1.0), vec4(self.g0.y) * vec4(other.g2.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g2.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g2.x, other.g2.y, other.g2.z, other.g0.x) * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g1.x) * vec4(other.g0.y, other.g2.z, other.g2.y, other.g3.x) * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g1.y) * vec4(other.g2.z, other.g0.y, other.g2.x, other.g3.y) * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g1.z) * vec4(other.g2.y, other.g2.x, other.g0.y, other.g3.z) * vec4(-1.0, 1.0, 1.0, 1.0) + vec4(self.g1.w) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g2.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

MultiVector flector_multi_vector_left_contraction(Flector self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(1.0, 0.0) + vec2(self.g1.w) * vec2(other.g4.w) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(1.0, 0.0), vec4(self.g0.y) * vec4(other.g3.z, other.g3.z, other.g3.x, other.g2.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g3.y, other.g3.x, other.g3.y, other.g2.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g3.x, other.g3.z, other.g3.y, other.g2.x) * vec4(0.0, 1.0, -1.0, -1.0), vec3(self.g0.y) * vec3(other.g4.z, other.g4.z, other.g4.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g4.y, other.g4.x, other.g4.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g4.x, other.g4.z, other.g4.y) * vec3(0.0, -1.0, 1.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g4.w) * vec3(-1.0), self.g0.xyzx * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

MultiVector flector_multi_vector_right_contraction(Flector self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * vec2(other.g1.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(1.0, 0.0) + vec2(self.g1.w) * vec2(other.g4.w) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(1.0, 0.0), vec4(self.g1.x) * vec4(other.g3.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * vec4(other.g3.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g3.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g3.x, other.g3.y, other.g3.z, other.g3.x) * vec4(1.0, 1.0, 1.0, 0.0) + self.g0 * vec4(other.g0.x), vec3(self.g1.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g1.x) * vec3(other.g1.x, other.g1.z, other.g1.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g1.w) * vec3(other.g1.x, other.g1.y, other.g1.z), self.g1 * vec4(other.g0.x));
}

MultiVector flector_multi_vector_left_anti_contraction(Flector self, MultiVector other) {
    return MultiVector(vec2(self.g1.x) * vec2(other.g4.x) * vec2(0.0, 1.0) + vec2(self.g1.y) * vec2(other.g4.y) * vec2(0.0, 1.0) + vec2(self.g1.z) * vec2(other.g4.z) * vec2(0.0, 1.0) + vec2(self.g0.x, self.g0.w) * vec2(other.g1.x, other.g1.w) * vec2(0.0, -1.0), self.g1.xyzx * vec4(other.g0.x) * vec4(-1.0, -1.0, -1.0, 0.0), vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g1.w) * vec3(-1.0), vec3(self.g1.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.x) * vec3(other.g1.x, other.g1.z, other.g1.y) * vec3(0.0, -1.0, 1.0), vec4(self.g1.x) * vec4(other.g2.z, other.g2.z, other.g2.y, other.g3.x) * vec4(0.0, -1.0, 1.0, 1.0) + vec4(self.g1.y) * vec4(other.g2.z, other.g2.z, other.g2.x, other.g3.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g1.z) * vec4(other.g2.y, other.g2.x, other.g2.y, other.g3.z) * vec4(-1.0, 1.0, 0.0, 1.0) + self.g0.xxxw * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

MultiVector flector_multi_vector_right_anti_contraction(Flector self, MultiVector other) {
    return MultiVector(vec2(self.g1.x) * vec2(other.g4.x) * vec2(0.0, 1.0) + vec2(self.g1.y) * vec2(other.g4.y) * vec2(0.0, 1.0) + vec2(self.g1.z) * vec2(other.g4.z) * vec2(0.0, 1.0) + vec2(self.g0.x, self.g0.w) * vec2(other.g1.x, other.g1.w) * vec2(0.0, -1.0), self.g0 * vec4(other.g0.y), vec3(0.0) - vec3(self.g0.w) * vec3(other.g4.x, other.g4.y, other.g4.z), vec3(self.g0.y) * vec3(other.g4.z, other.g4.z, other.g4.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * vec3(other.g4.y, other.g4.x, other.g4.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * vec3(other.g4.x, other.g4.z, other.g4.y) * vec3(0.0, 1.0, -1.0), vec4(self.g0.y) * vec4(other.g2.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g2.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g0.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.y) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + self.g0.wwwx * vec4(other.g2.x, other.g2.y, other.g2.z, other.g2.x) * vec4(1.0, 1.0, 1.0, -1.0));
}

Scalar flector_multi_vector_scalar_product(Flector self, MultiVector other) {
    return Scalar(self.g0.x * other.g1.x + self.g0.y * other.g1.y + self.g0.z * other.g1.z - self.g1.w * other.g4.w);
}

Scalar flector_multi_vector_dot(Flector self, MultiVector other) {
    return Scalar(self.g0.x * other.g1.x + self.g0.y * other.g1.y + self.g0.z * other.g1.z - self.g1.w * other.g4.w);
}

AntiScalar flector_multi_vector_anti_scalar_product(Flector self, MultiVector other) {
    return AntiScalar(0.0 - self.g0.w * other.g1.w + self.g1.x * other.g4.x + self.g1.y * other.g4.y + self.g1.z * other.g4.z);
}

AntiScalar flector_multi_vector_anti_dot(Flector self, MultiVector other) {
    return AntiScalar(0.0 - self.g0.w * other.g1.w + self.g1.x * other.g4.x + self.g1.y * other.g4.y + self.g1.z * other.g4.z);
}

Scalar multi_vector_scalar_into(MultiVector self) {
    return Scalar(self.g0.x);
}

MultiVector multi_vector_scalar_add(MultiVector self, Scalar other) {
    return MultiVector(self.g0 + vec2(other.g0) * vec2(1.0, 0.0), self.g1, self.g2, self.g3, self.g4);
}

MultiVector multi_vector_scalar_sub(MultiVector self, Scalar other) {
    return MultiVector(self.g0 - vec2(other.g0) * vec2(1.0, 0.0), self.g1, self.g2, self.g3, self.g4);
}

MultiVector multi_vector_scalar_geometric_product(MultiVector self, Scalar other) {
    return MultiVector(self.g0 * vec2(other.g0), self.g1 * vec4(other.g0), self.g2 * vec3(other.g0), self.g3 * vec3(other.g0), self.g4 * vec4(other.g0));
}

MultiVector multi_vector_scalar_geometric_anti_product(MultiVector self, Scalar other) {
    return MultiVector(self.g0.yx * vec2(other.g0) * vec2(1.0, 0.0), self.g4.xyzx * vec4(other.g0) * vec4(-1.0, -1.0, -1.0, 0.0), vec3(0.0), self.g2 * vec3(other.g0), self.g1.xxxw * vec4(other.g0) * vec4(0.0, 0.0, 0.0, -1.0));
}

Scalar multi_vector_scalar_regressive_product(MultiVector self, Scalar other) {
    return Scalar(self.g0.y * other.g0);
}

Scalar multi_vector_scalar_anti_wedge(MultiVector self, Scalar other) {
    return Scalar(self.g0.y * other.g0);
}

Scalar multi_vector_scalar_meet(MultiVector self, Scalar other) {
    return Scalar(self.g0.y * other.g0);
}

MultiVector multi_vector_scalar_outer_product(MultiVector self, Scalar other) {
    return MultiVector(self.g0 * vec2(other.g0), self.g1 * vec4(other.g0), self.g2 * vec3(other.g0), self.g3 * vec3(other.g0), self.g4 * vec4(other.g0));
}

MultiVector multi_vector_scalar_wedge(MultiVector self, Scalar other) {
    return MultiVector(self.g0 * vec2(other.g0), self.g1 * vec4(other.g0), self.g2 * vec3(other.g0), self.g3 * vec3(other.g0), self.g4 * vec4(other.g0));
}

MultiVector multi_vector_scalar_join(MultiVector self, Scalar other) {
    return MultiVector(self.g0 * vec2(other.g0), self.g1 * vec4(other.g0), self.g2 * vec3(other.g0), self.g3 * vec3(other.g0), self.g4 * vec4(other.g0));
}

MultiVector multi_vector_scalar_inner_product(MultiVector self, Scalar other) {
    return MultiVector(self.g0 * vec2(other.g0), self.g1 * vec4(other.g0), self.g2 * vec3(other.g0), self.g3 * vec3(other.g0), self.g4 * vec4(other.g0));
}

MultiVector multi_vector_scalar_inner_anti_product(MultiVector self, Scalar other) {
    return MultiVector(self.g0.yx * vec2(other.g0) * vec2(1.0, 0.0), self.g4.xyzx * vec4(other.g0) * vec4(-1.0, -1.0, -1.0, 0.0), vec3(0.0), self.g2 * vec3(other.g0), self.g1.xxxw * vec4(other.g0) * vec4(0.0, 0.0, 0.0, -1.0));
}

Scalar multi_vector_scalar_left_contraction(MultiVector self, Scalar other) {
    return Scalar(self.g0.x * other.g0);
}

MultiVector multi_vector_scalar_right_contraction(MultiVector self, Scalar other) {
    return MultiVector(self.g0 * vec2(other.g0), self.g1 * vec4(other.g0), self.g2 * vec3(other.g0), self.g3 * vec3(other.g0), self.g4 * vec4(other.g0));
}

MultiVector multi_vector_scalar_left_anti_contraction(MultiVector self, Scalar other) {
    return MultiVector(self.g0.yx * vec2(other.g0) * vec2(1.0, 0.0), self.g4.xyzx * vec4(other.g0) * vec4(-1.0, -1.0, -1.0, 0.0), vec3(0.0), self.g2 * vec3(other.g0), self.g1.xxxw * vec4(other.g0) * vec4(0.0, 0.0, 0.0, -1.0));
}

Scalar multi_vector_scalar_scalar_product(MultiVector self, Scalar other) {
    return Scalar(self.g0.x * other.g0);
}

Scalar multi_vector_scalar_dot(MultiVector self, Scalar other) {
    return Scalar(self.g0.x * other.g0);
}

AntiScalar multi_vector_anti_scalar_into(MultiVector self) {
    return AntiScalar(self.g0.y);
}

MultiVector multi_vector_anti_scalar_add(MultiVector self, AntiScalar other) {
    return MultiVector(self.g0 + vec2(other.g0) * vec2(0.0, 1.0), self.g1, self.g2, self.g3, self.g4);
}

MultiVector multi_vector_anti_scalar_sub(MultiVector self, AntiScalar other) {
    return MultiVector(self.g0 - vec2(other.g0) * vec2(0.0, 1.0), self.g1, self.g2, self.g3, self.g4);
}

MultiVector multi_vector_anti_scalar_geometric_product(MultiVector self, AntiScalar other) {
    return MultiVector(vec2(self.g0.x) * vec2(other.g0) * vec2(0.0, 1.0), self.g4.xxxw * vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0), self.g3 * vec3(other.g0), vec3(0.0), self.g1.xyzx * vec4(other.g0) * vec4(1.0, 1.0, 1.0, 0.0));
}

MultiVector multi_vector_anti_scalar_geometric_anti_product(MultiVector self, AntiScalar other) {
    return MultiVector(self.g0 * vec2(other.g0), self.g1 * vec4(other.g0), self.g2 * vec3(other.g0), self.g3 * vec3(other.g0), self.g4 * vec4(other.g0));
}

MultiVector multi_vector_anti_scalar_regressive_product(MultiVector self, AntiScalar other) {
    return MultiVector(self.g0 * vec2(other.g0), self.g1 * vec4(other.g0), self.g2 * vec3(other.g0), self.g3 * vec3(other.g0), self.g4 * vec4(other.g0));
}

MultiVector multi_vector_anti_scalar_anti_wedge(MultiVector self, AntiScalar other) {
    return MultiVector(self.g0 * vec2(other.g0), self.g1 * vec4(other.g0), self.g2 * vec3(other.g0), self.g3 * vec3(other.g0), self.g4 * vec4(other.g0));
}

MultiVector multi_vector_anti_scalar_meet(MultiVector self, AntiScalar other) {
    return MultiVector(self.g0 * vec2(other.g0), self.g1 * vec4(other.g0), self.g2 * vec3(other.g0), self.g3 * vec3(other.g0), self.g4 * vec4(other.g0));
}

AntiScalar multi_vector_anti_scalar_outer_product(MultiVector self, AntiScalar other) {
    return AntiScalar(self.g0.x * other.g0);
}

AntiScalar multi_vector_anti_scalar_wedge(MultiVector self, AntiScalar other) {
    return AntiScalar(self.g0.x * other.g0);
}

AntiScalar multi_vector_anti_scalar_join(MultiVector self, AntiScalar other) {
    return AntiScalar(self.g0.x * other.g0);
}

MultiVector multi_vector_anti_scalar_inner_product(MultiVector self, AntiScalar other) {
    return MultiVector(vec2(self.g0.x) * vec2(other.g0) * vec2(0.0, 1.0), self.g4.xxxw * vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0), self.g3 * vec3(other.g0), vec3(0.0), self.g1.xyzx * vec4(other.g0) * vec4(1.0, 1.0, 1.0, 0.0));
}

MultiVector multi_vector_anti_scalar_inner_anti_product(MultiVector self, AntiScalar other) {
    return MultiVector(self.g0 * vec2(other.g0), self.g1 * vec4(other.g0), self.g2 * vec3(other.g0), self.g3 * vec3(other.g0), self.g4 * vec4(other.g0));
}

MultiVector multi_vector_anti_scalar_left_contraction(MultiVector self, AntiScalar other) {
    return MultiVector(vec2(self.g0.x) * vec2(other.g0) * vec2(0.0, 1.0), self.g4.xxxw * vec4(other.g0) * vec4(0.0, 0.0, 0.0, 1.0), self.g3 * vec3(other.g0), vec3(0.0), self.g1.xyzx * vec4(other.g0) * vec4(1.0, 1.0, 1.0, 0.0));
}

AntiScalar multi_vector_anti_scalar_left_anti_contraction(MultiVector self, AntiScalar other) {
    return AntiScalar(self.g0.y * other.g0);
}

MultiVector multi_vector_anti_scalar_right_anti_contraction(MultiVector self, AntiScalar other) {
    return MultiVector(self.g0 * vec2(other.g0), self.g1 * vec4(other.g0), self.g2 * vec3(other.g0), self.g3 * vec3(other.g0), self.g4 * vec4(other.g0));
}

AntiScalar multi_vector_anti_scalar_anti_scalar_product(MultiVector self, AntiScalar other) {
    return AntiScalar(self.g0.y * other.g0);
}

AntiScalar multi_vector_anti_scalar_anti_dot(MultiVector self, AntiScalar other) {
    return AntiScalar(self.g0.y * other.g0);
}

HomogeneousMagnitude multi_vector_homogeneous_magnitude_into(MultiVector self) {
    return HomogeneousMagnitude(self.g0);
}

MultiVector multi_vector_homogeneous_magnitude_add(MultiVector self, HomogeneousMagnitude other) {
    return MultiVector(self.g0 + other.g0, self.g1, self.g2, self.g3, self.g4);
}

MultiVector multi_vector_homogeneous_magnitude_sub(MultiVector self, HomogeneousMagnitude other) {
    return MultiVector(self.g0 - other.g0, self.g1, self.g2, self.g3, self.g4);
}

MultiVector multi_vector_homogeneous_magnitude_geometric_product(MultiVector self, HomogeneousMagnitude other) {
    return MultiVector(vec2(self.g0.x) * other.g0 + self.g0 * vec2(other.g0.x) * vec2(0.0, 1.0), vec4(self.g4.w) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + self.g1 * vec4(other.g0.x), vec3(self.g3.x) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g3.y) * vec3(other.g0.y) * vec3(0.0, 1.0, 0.0) + vec3(self.g3.z) * vec3(other.g0.y) * vec3(0.0, 0.0, 1.0) + self.g2 * vec3(other.g0.x), self.g3 * vec3(other.g0.x), vec4(self.g4.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g4.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g4.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.x, self.g1.y, self.g1.z, self.g4.w) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x));
}

MultiVector multi_vector_homogeneous_magnitude_geometric_anti_product(MultiVector self, HomogeneousMagnitude other) {
    return MultiVector(vec2(self.g0.y) * other.g0 + vec2(self.g0.x) * other.g0.yx * vec2(1.0, 0.0), vec4(self.g4.x) * vec4(other.g0.x) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g4.y) * vec4(other.g0.x) * vec4(0.0, -1.0, 0.0, 0.0) + vec4(self.g4.z) * vec4(other.g0.x) * vec4(0.0, 0.0, -1.0, 0.0) + self.g1 * vec4(other.g0.y), self.g2 * vec3(other.g0.y), vec3(self.g3.x) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g3.y) * vec3(other.g0.y) * vec3(0.0, 1.0, 0.0) + vec3(self.g3.z) * vec3(other.g0.y) * vec3(0.0, 0.0, 1.0) + self.g2 * vec3(other.g0.x), vec4(self.g4.w) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g4.x, self.g4.y, self.g4.z, self.g1.w) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x) * vec4(1.0, 1.0, 1.0, -1.0));
}

MultiVector multi_vector_homogeneous_magnitude_regressive_product(MultiVector self, HomogeneousMagnitude other) {
    return MultiVector(vec2(self.g0.y) * other.g0 + vec2(self.g0.x) * other.g0.yx * vec2(1.0, 0.0), self.g1 * vec4(other.g0.y), self.g2 * vec3(other.g0.y), self.g3 * vec3(other.g0.y), self.g4 * vec4(other.g0.y));
}

MultiVector multi_vector_homogeneous_magnitude_anti_wedge(MultiVector self, HomogeneousMagnitude other) {
    return MultiVector(vec2(self.g0.y) * other.g0 + vec2(self.g0.x) * other.g0.yx * vec2(1.0, 0.0), self.g1 * vec4(other.g0.y), self.g2 * vec3(other.g0.y), self.g3 * vec3(other.g0.y), self.g4 * vec4(other.g0.y));
}

MultiVector multi_vector_homogeneous_magnitude_meet(MultiVector self, HomogeneousMagnitude other) {
    return MultiVector(vec2(self.g0.y) * other.g0 + vec2(self.g0.x) * other.g0.yx * vec2(1.0, 0.0), self.g1 * vec4(other.g0.y), self.g2 * vec3(other.g0.y), self.g3 * vec3(other.g0.y), self.g4 * vec4(other.g0.y));
}

MultiVector multi_vector_homogeneous_magnitude_outer_product(MultiVector self, HomogeneousMagnitude other) {
    return MultiVector(vec2(self.g0.x) * other.g0 + self.g0 * vec2(other.g0.x) * vec2(0.0, 1.0), self.g1 * vec4(other.g0.x), self.g2 * vec3(other.g0.x), self.g3 * vec3(other.g0.x), self.g4 * vec4(other.g0.x));
}

MultiVector multi_vector_homogeneous_magnitude_wedge(MultiVector self, HomogeneousMagnitude other) {
    return MultiVector(vec2(self.g0.x) * other.g0 + self.g0 * vec2(other.g0.x) * vec2(0.0, 1.0), self.g1 * vec4(other.g0.x), self.g2 * vec3(other.g0.x), self.g3 * vec3(other.g0.x), self.g4 * vec4(other.g0.x));
}

MultiVector multi_vector_homogeneous_magnitude_join(MultiVector self, HomogeneousMagnitude other) {
    return MultiVector(vec2(self.g0.x) * other.g0 + self.g0 * vec2(other.g0.x) * vec2(0.0, 1.0), self.g1 * vec4(other.g0.x), self.g2 * vec3(other.g0.x), self.g3 * vec3(other.g0.x), self.g4 * vec4(other.g0.x));
}

MultiVector multi_vector_homogeneous_magnitude_inner_product(MultiVector self, HomogeneousMagnitude other) {
    return MultiVector(vec2(self.g0.x) * other.g0 + self.g0 * vec2(other.g0.x) * vec2(0.0, 1.0), vec4(self.g4.w) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + self.g1 * vec4(other.g0.x), vec3(self.g3.x) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g3.y) * vec3(other.g0.y) * vec3(0.0, 1.0, 0.0) + vec3(self.g3.z) * vec3(other.g0.y) * vec3(0.0, 0.0, 1.0) + self.g2 * vec3(other.g0.x), self.g3 * vec3(other.g0.x), vec4(self.g4.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g4.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g4.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.x, self.g1.y, self.g1.z, self.g4.w) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x));
}

MultiVector multi_vector_homogeneous_magnitude_inner_anti_product(MultiVector self, HomogeneousMagnitude other) {
    return MultiVector(vec2(self.g0.y) * other.g0 + vec2(self.g0.x) * other.g0.yx * vec2(1.0, 0.0), vec4(self.g4.x) * vec4(other.g0.x) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g4.y) * vec4(other.g0.x) * vec4(0.0, -1.0, 0.0, 0.0) + vec4(self.g4.z) * vec4(other.g0.x) * vec4(0.0, 0.0, -1.0, 0.0) + self.g1 * vec4(other.g0.y), self.g2 * vec3(other.g0.y), vec3(self.g3.x) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g3.y) * vec3(other.g0.y) * vec3(0.0, 1.0, 0.0) + vec3(self.g3.z) * vec3(other.g0.y) * vec3(0.0, 0.0, 1.0) + self.g2 * vec3(other.g0.x), vec4(self.g4.w) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g4.x, self.g4.y, self.g4.z, self.g1.w) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x) * vec4(1.0, 1.0, 1.0, -1.0));
}

MultiVector multi_vector_homogeneous_magnitude_left_contraction(MultiVector self, HomogeneousMagnitude other) {
    return MultiVector(vec2(self.g0.x) * other.g0, self.g4.xxxw * vec4(other.g0.x, other.g0.x, other.g0.x, other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0), self.g3 * vec3(other.g0.y), vec3(0.0), self.g1.xyzx * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

MultiVector multi_vector_homogeneous_magnitude_right_contraction(MultiVector self, HomogeneousMagnitude other) {
    return MultiVector(self.g0 * vec2(other.g0.x), self.g1 * vec4(other.g0.x), self.g2 * vec3(other.g0.x), self.g3 * vec3(other.g0.x), self.g4 * vec4(other.g0.x));
}

MultiVector multi_vector_homogeneous_magnitude_left_anti_contraction(MultiVector self, HomogeneousMagnitude other) {
    return MultiVector(vec2(self.g0.y) * other.g0, self.g4.xyzx * vec4(other.g0.x) * vec4(-1.0, -1.0, -1.0, 0.0), vec3(0.0), self.g2 * vec3(other.g0.x), self.g1.xxxw * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

MultiVector multi_vector_homogeneous_magnitude_right_anti_contraction(MultiVector self, HomogeneousMagnitude other) {
    return MultiVector(self.g0 * vec2(other.g0.y), self.g1 * vec4(other.g0.y), self.g2 * vec3(other.g0.y), self.g3 * vec3(other.g0.y), self.g4 * vec4(other.g0.y));
}

Scalar multi_vector_homogeneous_magnitude_scalar_product(MultiVector self, HomogeneousMagnitude other) {
    return Scalar(self.g0.x * other.g0.x);
}

Scalar multi_vector_homogeneous_magnitude_dot(MultiVector self, HomogeneousMagnitude other) {
    return Scalar(self.g0.x * other.g0.x);
}

AntiScalar multi_vector_homogeneous_magnitude_anti_scalar_product(MultiVector self, HomogeneousMagnitude other) {
    return AntiScalar(self.g0.y * other.g0.y);
}

AntiScalar multi_vector_homogeneous_magnitude_anti_dot(MultiVector self, HomogeneousMagnitude other) {
    return AntiScalar(self.g0.y * other.g0.y);
}

Point multi_vector_point_into(MultiVector self) {
    return Point(self.g1);
}

MultiVector multi_vector_point_add(MultiVector self, Point other) {
    return MultiVector(self.g0, self.g1 + other.g0, self.g2, self.g3, self.g4);
}

MultiVector multi_vector_point_sub(MultiVector self, Point other) {
    return MultiVector(self.g0, self.g1 - other.g0, self.g2, self.g3, self.g4);
}

MultiVector multi_vector_point_geometric_product(MultiVector self, Point other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g0.y) * vec2(1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(1.0, 0.0) + vec2(self.g4.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g4.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g4.w) * vec2(other.g0.w) * vec2(0.0, -1.0) + vec2(self.g1.x, self.g4.x) * vec2(other.g0.x) * vec2(1.0, -1.0), vec4(self.g0.x) * other.g0 + vec4(self.g2.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g2.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g3.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g3.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g2.x, self.g3.x, self.g3.x, self.g2.x) * other.g0.xzyx * vec4(0.0, 1.0, -1.0, 1.0), vec3(self.g1.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g4.x) * vec3(other.g0.z, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0) + vec3(self.g4.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g4.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g0.w) * vec3(-1.0), vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) - vec3(self.g4.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g1.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0), vec4(self.g2.x) * other.g0.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g2.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g2.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g3.x) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g3.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g3.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.y, self.g0.y, self.g0.y, self.g0.x) * other.g0.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

MultiVector multi_vector_point_geometric_anti_product(MultiVector self, Point other) {
    return MultiVector(vec2(self.g4.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g4.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g4.w) * vec2(other.g0.w) * vec2(-1.0, 0.0) + vec2(self.g4.x, self.g1.w) * vec2(other.g0.x, other.g0.w) * vec2(-1.0), vec4(self.g0.y) * other.g0 + vec4(self.g2.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g2.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g3.y) * vec4(other.g0.w) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g3.z) * vec4(other.g0.w) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g3.x, self.g2.x, self.g2.x, self.g2.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, 0.0), vec3(self.g4.x, self.g4.y, self.g4.z) * vec3(other.g0.w) * vec3(-1.0), vec3(self.g1.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g4.x) * vec3(other.g0.z, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0) + vec3(self.g4.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g4.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g0.w) * vec3(-1.0), vec4(self.g2.x) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g2.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g2.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0));
}

MultiVector multi_vector_point_regressive_product(MultiVector self, Point other) {
    return MultiVector(vec2(self.g4.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g4.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g4.w) * vec2(other.g0.w) * vec2(-1.0, 0.0) + vec2(self.g4.x) * vec2(other.g0.x) * vec2(-1.0, 0.0), vec4(self.g0.y) * other.g0, vec3(0.0), vec3(0.0), vec4(0.0));
}

MultiVector multi_vector_point_anti_wedge(MultiVector self, Point other) {
    return MultiVector(vec2(self.g4.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g4.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g4.w) * vec2(other.g0.w) * vec2(-1.0, 0.0) + vec2(self.g4.x) * vec2(other.g0.x) * vec2(-1.0, 0.0), vec4(self.g0.y) * other.g0, vec3(0.0), vec3(0.0), vec4(0.0));
}

MultiVector multi_vector_point_meet(MultiVector self, Point other) {
    return MultiVector(vec2(self.g4.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g4.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g4.w) * vec2(other.g0.w) * vec2(-1.0, 0.0) + vec2(self.g4.x) * vec2(other.g0.x) * vec2(-1.0, 0.0), vec4(self.g0.y) * other.g0, vec3(0.0), vec3(0.0), vec4(0.0));
}

MultiVector multi_vector_point_outer_product(MultiVector self, Point other) {
    return MultiVector(vec2(self.g4.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g4.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g4.w) * vec2(other.g0.w) * vec2(0.0, -1.0) + vec2(self.g4.x) * vec2(other.g0.x) * vec2(0.0, -1.0), vec4(self.g0.x) * other.g0, vec3(self.g1.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g0.w) * vec3(-1.0), vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0), vec4(self.g2.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g2.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g3.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g3.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g3.x, self.g2.x, self.g2.x, self.g3.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0));
}

MultiVector multi_vector_point_wedge(MultiVector self, Point other) {
    return MultiVector(vec2(self.g4.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g4.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g4.w) * vec2(other.g0.w) * vec2(0.0, -1.0) + vec2(self.g4.x) * vec2(other.g0.x) * vec2(0.0, -1.0), vec4(self.g0.x) * other.g0, vec3(self.g1.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g0.w) * vec3(-1.0), vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0), vec4(self.g2.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g2.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g3.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g3.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g3.x, self.g2.x, self.g2.x, self.g3.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0));
}

MultiVector multi_vector_point_join(MultiVector self, Point other) {
    return MultiVector(vec2(self.g4.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g4.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g4.w) * vec2(other.g0.w) * vec2(0.0, -1.0) + vec2(self.g4.x) * vec2(other.g0.x) * vec2(0.0, -1.0), vec4(self.g0.x) * other.g0, vec3(self.g1.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g0.w) * vec3(-1.0), vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0), vec4(self.g2.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g2.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g3.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g3.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g3.x, self.g2.x, self.g2.x, self.g3.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0));
}

MultiVector multi_vector_point_inner_product(MultiVector self, Point other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g0.y) * vec2(1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(1.0, 0.0) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(1.0, 0.0), vec4(self.g0.x) * other.g0 + vec4(self.g2.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g2.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g3.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g3.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g2.x, self.g3.x, self.g3.x, self.g2.x) * other.g0.xzyx * vec4(0.0, 1.0, -1.0, 1.0), vec3(self.g4.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g4.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g4.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g4.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(self.g0.y, self.g0.y, self.g0.y, self.g0.x) * other.g0.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

MultiVector multi_vector_point_inner_anti_product(MultiVector self, Point other) {
    return MultiVector(vec2(self.g1.x, self.g1.w) * vec2(other.g0.x, other.g0.w) * vec2(0.0, -1.0), vec4(self.g0.y) * other.g0, vec3(self.g4.x, self.g4.y, self.g4.z) * vec3(other.g0.w) * vec3(-1.0), vec3(self.g4.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g4.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g4.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0), vec4(self.g2.x) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g2.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g2.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0));
}

MultiVector multi_vector_point_left_contraction(MultiVector self, Point other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g0.y) * vec2(1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(1.0, 0.0) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(1.0, 0.0), vec4(self.g0.x) * other.g0, vec3(0.0), vec3(0.0), vec4(0.0));
}

MultiVector multi_vector_point_right_contraction(MultiVector self, Point other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g0.y) * vec2(1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(1.0, 0.0) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(1.0, 0.0), vec4(self.g2.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g2.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g3.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g3.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g2.x, self.g3.x, self.g3.x, self.g2.x) * other.g0.xzyx * vec4(0.0, 1.0, -1.0, 1.0), vec3(self.g4.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g4.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g4.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g4.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(self.g0.y, self.g0.y, self.g0.y, self.g0.x) * other.g0.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

MultiVector multi_vector_point_left_anti_contraction(MultiVector self, Point other) {
    return MultiVector(vec2(self.g1.x, self.g1.w) * vec2(other.g0.x, other.g0.w) * vec2(0.0, -1.0), vec4(self.g0.y) * other.g0, vec3(self.g4.x, self.g4.y, self.g4.z) * vec3(other.g0.w) * vec3(-1.0), vec3(self.g4.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g4.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g4.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0), vec4(self.g2.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g2.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g2.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

MultiVector multi_vector_point_right_anti_contraction(MultiVector self, Point other) {
    return MultiVector(vec2(self.g1.x, self.g1.w) * vec2(other.g0.x, other.g0.w) * vec2(0.0, -1.0), vec4(0.0), vec3(0.0), vec3(0.0), vec4(self.g0.x) * other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0));
}

Scalar multi_vector_point_scalar_product(MultiVector self, Point other) {
    return Scalar(self.g1.x * other.g0.x + self.g1.y * other.g0.y + self.g1.z * other.g0.z);
}

Scalar multi_vector_point_dot(MultiVector self, Point other) {
    return Scalar(self.g1.x * other.g0.x + self.g1.y * other.g0.y + self.g1.z * other.g0.z);
}

AntiScalar multi_vector_point_anti_scalar_product(MultiVector self, Point other) {
    return AntiScalar(0.0 - self.g1.w * other.g0.w);
}

AntiScalar multi_vector_point_anti_dot(MultiVector self, Point other) {
    return AntiScalar(0.0 - self.g1.w * other.g0.w);
}

Line multi_vector_line_into(MultiVector self) {
    return Line(self.g2, self.g3);
}

MultiVector multi_vector_line_add(MultiVector self, Line other) {
    return MultiVector(self.g0, self.g1, self.g2 + other.g0, self.g3 + other.g1, self.g4);
}

MultiVector multi_vector_line_sub(MultiVector self, Line other) {
    return MultiVector(self.g0, self.g1, self.g2 - other.g0, self.g3 - other.g1, self.g4);
}

MultiVector multi_vector_line_geometric_product(MultiVector self, Line other) {
    return MultiVector(vec2(self.g2.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g1.z) * vec2(0.0, -1.0) - vec2(self.g3.x) * vec2(other.g1.x, other.g0.x) - vec2(self.g3.y) * vec2(other.g1.y, other.g0.y) - vec2(self.g3.z) * vec2(other.g1.z, other.g0.z) + vec2(self.g2.x) * vec2(other.g1.x) * vec2(0.0, -1.0), vec4(self.g1.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g4.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0), vec3(self.g0.x) * other.g0 + vec3(self.g0.y) * other.g1 + vec3(self.g2.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g2.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g3.x) * other.g0.zzy * vec3(0.0, 1.0, -1.0) + vec3(self.g3.y) * other.g0.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g3.z) * other.g0.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g2.x) * other.g1.xzy * vec3(0.0, 1.0, -1.0), vec3(self.g0.x) * other.g1 + vec3(self.g3.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g3.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g3.x) * other.g1.xzy * vec3(0.0, 1.0, -1.0), vec4(self.g1.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g4.x) * vec4(other.g1.z, other.g1.z, other.g1.y, other.g1.z) * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g4.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.z) * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g4.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.y) * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g4.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

MultiVector multi_vector_line_geometric_anti_product(MultiVector self, Line other) {
    return MultiVector(vec2(0.0) - vec2(self.g2.x) * vec2(other.g1.x, other.g0.x) - vec2(self.g2.y) * vec2(other.g1.y, other.g0.y) - vec2(self.g2.z) * vec2(other.g1.z, other.g0.z) + vec2(self.g3.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g3.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g3.x) * vec2(other.g0.x) * vec2(-1.0, 0.0), vec4(self.g1.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g0.z) * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.y) * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(-1.0, -1.0, -1.0, 0.0) + vec4(self.g4.x) * vec4(other.g1.z, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0) + vec4(self.g4.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g4.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g4.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g0.x) * vec4(0.0, -1.0, 1.0, 0.0), vec3(self.g0.y) * other.g0 + vec3(self.g2.y) * other.g0.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g2.z) * other.g0.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g2.x) * other.g0.xzy * vec3(0.0, -1.0, 1.0), vec3(self.g0.x) * other.g0 + vec3(self.g0.y) * other.g1 + vec3(self.g2.y) * other.g1.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g2.z) * other.g1.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g3.x) * other.g0.zzy * vec3(0.0, -1.0, 1.0) + vec3(self.g3.y) * other.g0.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g3.z) * other.g0.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g2.x) * other.g1.xzy * vec3(0.0, -1.0, 1.0), vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.x) * vec4(other.g0.z, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, -1.0, 1.0, 1.0) + vec4(self.g4.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g4.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(-1.0, 1.0, 0.0, 1.0) + self.g1.wwwx * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, -1.0));
}

MultiVector multi_vector_line_regressive_product(MultiVector self, Line other) {
    return MultiVector(vec2(self.g2.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g2.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g3.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g3.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g3.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g2.x) * vec2(other.g1.x) * vec2(-1.0, 0.0), vec4(self.g4.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g4.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g4.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g4.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0), vec3(self.g0.y) * other.g0, vec3(self.g0.y) * other.g1, vec4(0.0));
}

MultiVector multi_vector_line_anti_wedge(MultiVector self, Line other) {
    return MultiVector(vec2(self.g2.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g2.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g3.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g3.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g3.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g2.x) * vec2(other.g1.x) * vec2(-1.0, 0.0), vec4(self.g4.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g4.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g4.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g4.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0), vec3(self.g0.y) * other.g0, vec3(self.g0.y) * other.g1, vec4(0.0));
}

MultiVector multi_vector_line_meet(MultiVector self, Line other) {
    return MultiVector(vec2(self.g2.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g2.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g3.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g3.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g3.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g2.x) * vec2(other.g1.x) * vec2(-1.0, 0.0), vec4(self.g4.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g4.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g4.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g4.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0), vec3(self.g0.y) * other.g0, vec3(self.g0.y) * other.g1, vec4(0.0));
}

MultiVector multi_vector_line_outer_product(MultiVector self, Line other) {
    return MultiVector(vec2(self.g2.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g3.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g3.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g3.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g2.x) * vec2(other.g1.x) * vec2(0.0, -1.0), vec4(0.0), vec3(self.g0.x) * other.g0, vec3(self.g0.x) * other.g1, vec4(self.g1.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

MultiVector multi_vector_line_wedge(MultiVector self, Line other) {
    return MultiVector(vec2(self.g2.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g3.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g3.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g3.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g2.x) * vec2(other.g1.x) * vec2(0.0, -1.0), vec4(0.0), vec3(self.g0.x) * other.g0, vec3(self.g0.x) * other.g1, vec4(self.g1.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

MultiVector multi_vector_line_join(MultiVector self, Line other) {
    return MultiVector(vec2(self.g2.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g3.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g3.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g3.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g2.x) * vec2(other.g1.x) * vec2(0.0, -1.0), vec4(0.0), vec3(self.g0.x) * other.g0, vec3(self.g0.x) * other.g1, vec4(self.g1.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

MultiVector multi_vector_line_inner_product(MultiVector self, Line other) {
    return MultiVector(vec2(self.g3.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g3.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g3.x) * vec2(other.g1.x) * vec2(-1.0, 0.0), vec4(self.g1.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g4.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0), vec3(self.g0.x) * other.g0 + vec3(self.g0.y) * other.g1, vec3(self.g0.x) * other.g1, vec4(0.0));
}

MultiVector multi_vector_line_inner_anti_product(MultiVector self, Line other) {
    return MultiVector(vec2(self.g2.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g2.x) * vec2(other.g0.x) * vec2(0.0, -1.0), vec4(0.0), vec3(self.g0.y) * other.g0, vec3(self.g0.x) * other.g0 + vec3(self.g0.y) * other.g1, vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.x) * vec4(other.g0.z, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, -1.0, 1.0, 1.0) + vec4(self.g4.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g4.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(-1.0, 1.0, 0.0, 1.0) + self.g1.wwwx * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, -1.0));
}

MultiVector multi_vector_line_left_contraction(MultiVector self, Line other) {
    return MultiVector(vec2(self.g3.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g3.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g3.x) * vec2(other.g1.x) * vec2(-1.0, 0.0), vec4(self.g1.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0), vec3(self.g0.x) * other.g0, vec3(self.g0.x) * other.g1, vec4(0.0));
}

MultiVector multi_vector_line_right_contraction(MultiVector self, Line other) {
    return MultiVector(vec2(self.g3.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g3.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g3.x) * vec2(other.g1.x) * vec2(-1.0, 0.0), vec4(self.g4.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g4.wwwx * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, -1.0), vec3(self.g0.y) * other.g1, vec3(0.0), vec4(0.0));
}

MultiVector multi_vector_line_left_anti_contraction(MultiVector self, Line other) {
    return MultiVector(vec2(self.g2.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g2.x) * vec2(other.g0.x) * vec2(0.0, -1.0), vec4(0.0), vec3(self.g0.y) * other.g0, vec3(self.g0.y) * other.g1, vec4(self.g4.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g4.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(-1.0, 1.0, 0.0, 1.0) + vec4(self.g4.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, -1.0, 1.0, 1.0));
}

MultiVector multi_vector_line_right_anti_contraction(MultiVector self, Line other) {
    return MultiVector(vec2(self.g2.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g2.x) * vec2(other.g0.x) * vec2(0.0, -1.0), vec4(0.0), vec3(0.0), vec3(self.g0.x) * other.g0, vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g1.wwwx * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, -1.0));
}

Scalar multi_vector_line_scalar_product(MultiVector self, Line other) {
    return Scalar(0.0 - self.g3.x * other.g1.x - self.g3.y * other.g1.y - self.g3.z * other.g1.z);
}

Scalar multi_vector_line_dot(MultiVector self, Line other) {
    return Scalar(0.0 - self.g3.x * other.g1.x - self.g3.y * other.g1.y - self.g3.z * other.g1.z);
}

AntiScalar multi_vector_line_anti_scalar_product(MultiVector self, Line other) {
    return AntiScalar(0.0 - self.g2.x * other.g0.x - self.g2.y * other.g0.y - self.g2.z * other.g0.z);
}

AntiScalar multi_vector_line_anti_dot(MultiVector self, Line other) {
    return AntiScalar(0.0 - self.g2.x * other.g0.x - self.g2.y * other.g0.y - self.g2.z * other.g0.z);
}

Plane multi_vector_plane_into(MultiVector self) {
    return Plane(self.g4);
}

MultiVector multi_vector_plane_add(MultiVector self, Plane other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3, self.g4 + other.g0);
}

MultiVector multi_vector_plane_sub(MultiVector self, Plane other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3, self.g4 - other.g0);
}

MultiVector multi_vector_plane_geometric_product(MultiVector self, Plane other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g0.y) * vec2(0.0, 1.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(0.0, 1.0) + vec2(self.g1.w) * vec2(other.g0.w) * vec2(0.0, 1.0) + vec2(self.g4.w, self.g1.x) * vec2(other.g0.w, other.g0.x) * vec2(-1.0, 1.0), vec4(self.g3.x) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g3.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g3.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x, self.g0.x, self.g0.x, self.g0.y) * other.g0.xxxw * vec4(0.0, 0.0, 0.0, -1.0), vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g4.y) * vec3(other.g0.w) * vec3(0.0, 1.0, 0.0) + vec3(self.g4.z) * vec3(other.g0.w) * vec3(0.0, 0.0, 1.0) - vec3(self.g4.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g4.x, self.g1.x, self.g1.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, -1.0, 1.0), vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g0.w) * vec3(-1.0), vec4(self.g0.x) * other.g0 + vec4(self.g3.x) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g3.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g3.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g2.x, self.g2.y, self.g2.z, self.g2.x) * other.g0.wwwx * vec4(-1.0, -1.0, -1.0, 0.0));
}

MultiVector multi_vector_plane_geometric_anti_product(MultiVector self, Plane other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g0.y) * vec2(1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(1.0, 0.0) + vec2(self.g1.w) * vec2(other.g0.w) * vec2(1.0, 0.0) + vec2(self.g4.y) * vec2(other.g0.y) * vec2(0.0, 1.0) + vec2(self.g4.z) * vec2(other.g0.z) * vec2(0.0, 1.0) + vec2(self.g1.x, self.g4.x) * vec2(other.g0.x), vec4(self.g2.x) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g2.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g2.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g3.x) * other.g0.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g3.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g3.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0), vec3(0.0) - vec3(self.g1.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g4.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g4.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g4.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0), vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g4.y) * vec3(other.g0.w) * vec3(0.0, 1.0, 0.0) + vec3(self.g4.z) * vec3(other.g0.w) * vec3(0.0, 0.0, 1.0) - vec3(self.g4.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g4.x, self.g1.x, self.g1.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, 1.0, -1.0), vec4(self.g0.y) * other.g0 + vec4(self.g2.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g2.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g3.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g3.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g2.x, self.g2.x, self.g2.x, self.g3.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, -1.0));
}

MultiVector multi_vector_plane_regressive_product(MultiVector self, Plane other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g0.y) * vec2(1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(1.0, 0.0) + vec2(self.g1.w) * vec2(other.g0.w) * vec2(1.0, 0.0) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(1.0, 0.0), vec4(self.g2.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g2.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g3.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g3.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g2.x, self.g3.x, self.g3.x, self.g2.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0), vec3(self.g4.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g4.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g4.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g4.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g4.x, self.g4.y, self.g4.z) * vec3(other.g0.w), vec4(self.g0.y) * other.g0);
}

MultiVector multi_vector_plane_anti_wedge(MultiVector self, Plane other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g0.y) * vec2(1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(1.0, 0.0) + vec2(self.g1.w) * vec2(other.g0.w) * vec2(1.0, 0.0) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(1.0, 0.0), vec4(self.g2.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g2.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g3.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g3.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g2.x, self.g3.x, self.g3.x, self.g2.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0), vec3(self.g4.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g4.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g4.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g4.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g4.x, self.g4.y, self.g4.z) * vec3(other.g0.w), vec4(self.g0.y) * other.g0);
}

MultiVector multi_vector_plane_meet(MultiVector self, Plane other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g0.y) * vec2(1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(1.0, 0.0) + vec2(self.g1.w) * vec2(other.g0.w) * vec2(1.0, 0.0) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(1.0, 0.0), vec4(self.g2.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g2.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g3.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g3.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g2.x, self.g3.x, self.g3.x, self.g2.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0), vec3(self.g4.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g4.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g4.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g4.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g4.x, self.g4.y, self.g4.z) * vec3(other.g0.w), vec4(self.g0.y) * other.g0);
}

MultiVector multi_vector_plane_outer_product(MultiVector self, Plane other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g0.y) * vec2(0.0, 1.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(0.0, 1.0) + vec2(self.g1.w) * vec2(other.g0.w) * vec2(0.0, 1.0) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(0.0, 1.0), vec4(0.0), vec3(0.0), vec3(0.0), vec4(self.g0.x) * other.g0);
}

MultiVector multi_vector_plane_wedge(MultiVector self, Plane other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g0.y) * vec2(0.0, 1.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(0.0, 1.0) + vec2(self.g1.w) * vec2(other.g0.w) * vec2(0.0, 1.0) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(0.0, 1.0), vec4(0.0), vec3(0.0), vec3(0.0), vec4(self.g0.x) * other.g0);
}

MultiVector multi_vector_plane_join(MultiVector self, Plane other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g0.y) * vec2(0.0, 1.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(0.0, 1.0) + vec2(self.g1.w) * vec2(other.g0.w) * vec2(0.0, 1.0) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(0.0, 1.0), vec4(0.0), vec3(0.0), vec3(0.0), vec4(self.g0.x) * other.g0);
}

MultiVector multi_vector_plane_inner_product(MultiVector self, Plane other) {
    return MultiVector(vec2(self.g4.w, self.g4.x) * vec2(other.g0.w, other.g0.x) * vec2(-1.0, 0.0), vec4(self.g3.x) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g3.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g3.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x, self.g0.x, self.g0.x, self.g0.y) * other.g0.xxxw * vec4(0.0, 0.0, 0.0, -1.0), vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0), vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g0.w) * vec3(-1.0), vec4(self.g0.x) * other.g0);
}

MultiVector multi_vector_plane_inner_anti_product(MultiVector self, Plane other) {
    return MultiVector(vec2(self.g4.y) * vec2(other.g0.y) * vec2(0.0, 1.0) + vec2(self.g4.z) * vec2(other.g0.z) * vec2(0.0, 1.0) + vec2(self.g4.x) * vec2(other.g0.x) * vec2(0.0, 1.0), vec4(self.g0.x) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0), vec3(0.0) - vec3(self.g1.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g1.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0), vec4(self.g0.y) * other.g0 + vec4(self.g2.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g2.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g3.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g3.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g2.x, self.g2.x, self.g2.x, self.g3.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, -1.0));
}

MultiVector multi_vector_plane_left_contraction(MultiVector self, Plane other) {
    return MultiVector(vec2(self.g4.w, self.g4.x) * vec2(other.g0.w, other.g0.x) * vec2(-1.0, 0.0), vec4(self.g3.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g3.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g3.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0), vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0), vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g0.w) * vec3(-1.0), vec4(self.g0.x) * other.g0);
}

MultiVector multi_vector_plane_right_contraction(MultiVector self, Plane other) {
    return MultiVector(vec2(self.g4.w, self.g4.x) * vec2(other.g0.w, other.g0.x) * vec2(-1.0, 0.0), vec4(self.g0.x, self.g0.x, self.g0.x, self.g0.y) * other.g0.xxxw * vec4(0.0, 0.0, 0.0, -1.0), vec3(0.0), vec3(0.0), vec4(0.0));
}

MultiVector multi_vector_plane_left_anti_contraction(MultiVector self, Plane other) {
    return MultiVector(vec2(self.g4.y) * vec2(other.g0.y) * vec2(0.0, 1.0) + vec2(self.g4.z) * vec2(other.g0.z) * vec2(0.0, 1.0) + vec2(self.g4.x) * vec2(other.g0.x) * vec2(0.0, 1.0), vec4(0.0), vec3(0.0), vec3(0.0), vec4(self.g0.y) * other.g0);
}

MultiVector multi_vector_plane_right_anti_contraction(MultiVector self, Plane other) {
    return MultiVector(vec2(self.g4.y) * vec2(other.g0.y) * vec2(0.0, 1.0) + vec2(self.g4.z) * vec2(other.g0.z) * vec2(0.0, 1.0) + vec2(self.g4.x) * vec2(other.g0.x) * vec2(0.0, 1.0), vec4(self.g0.x) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0), vec3(0.0) - vec3(self.g1.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g1.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0), vec4(self.g2.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g2.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g3.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g3.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g2.x, self.g2.x, self.g2.x, self.g3.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, -1.0));
}

Scalar multi_vector_plane_scalar_product(MultiVector self, Plane other) {
    return Scalar(0.0 - self.g4.w * other.g0.w);
}

Scalar multi_vector_plane_dot(MultiVector self, Plane other) {
    return Scalar(0.0 - self.g4.w * other.g0.w);
}

AntiScalar multi_vector_plane_anti_scalar_product(MultiVector self, Plane other) {
    return AntiScalar(self.g4.x * other.g0.x + self.g4.y * other.g0.y + self.g4.z * other.g0.z);
}

AntiScalar multi_vector_plane_anti_dot(MultiVector self, Plane other) {
    return AntiScalar(self.g4.x * other.g0.x + self.g4.y * other.g0.y + self.g4.z * other.g0.z);
}

Motor multi_vector_motor_into(MultiVector self) {
    return Motor(vec4(self.g2.x, self.g2.y, self.g2.z, self.g0.y), self.g3);
}

MultiVector multi_vector_motor_add(MultiVector self, Motor other) {
    return MultiVector(self.g0 + vec2(other.g0.x, other.g0.w) * vec2(0.0, 1.0), self.g1, self.g2 + vec3(other.g0.x, other.g0.y, other.g0.z), self.g3 + other.g1, self.g4);
}

MultiVector multi_vector_motor_sub(MultiVector self, Motor other) {
    return MultiVector(self.g0 - vec2(other.g0.x, other.g0.w) * vec2(0.0, 1.0), self.g1, self.g2 - vec3(other.g0.x, other.g0.y, other.g0.z), self.g3 - other.g1, self.g4);
}

MultiVector multi_vector_motor_geometric_product(MultiVector self, Motor other) {
    return MultiVector(vec2(self.g2.x) * vec2(other.g1.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g1.z) * vec2(0.0, -1.0) - vec2(self.g3.x) * vec2(other.g1.x, other.g0.x) - vec2(self.g3.y) * vec2(other.g1.y, other.g0.y) - vec2(self.g3.z) * vec2(other.g1.z, other.g0.z) + vec2(self.g0.x) * vec2(other.g0.x, other.g0.w) * vec2(0.0, 1.0), vec4(self.g1.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g4.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g0.w) + vec4(self.g1.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g0.y) * other.g1 + vec3(self.g2.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g2.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g3.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g3.y) * vec3(other.g0.z, other.g0.w, other.g0.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g3.z) * vec3(other.g0.y, other.g0.x, other.g0.w) * vec3(1.0, -1.0, 1.0) + vec3(self.g2.x) * other.g1.xzy * vec3(0.0, 1.0, -1.0), vec3(self.g0.x) * other.g1 + vec3(self.g3.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g3.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g3.x) * other.g1.xzy * vec3(0.0, 1.0, -1.0), vec4(self.g1.x) * vec4(other.g0.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.z, other.g0.w, other.g0.x, other.g1.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g0.w, other.g1.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g4.x) * vec4(other.g1.z, other.g1.z, other.g1.y, other.g1.z) * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g4.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.z) * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g4.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.y) * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g4.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + self.g1.wwwx * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

MultiVector multi_vector_motor_geometric_anti_product(MultiVector self, Motor other) {
    return MultiVector(vec2(0.0) - vec2(self.g2.x) * vec2(other.g1.x, other.g0.x) - vec2(self.g2.y) * vec2(other.g1.y, other.g0.y) - vec2(self.g2.z) * vec2(other.g1.z, other.g0.z) + vec2(self.g3.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g3.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g3.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + self.g0 * vec2(other.g0.w), vec4(self.g1.y) * other.g0.zwxz * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g1.z) * other.g0.yxwy * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g0.w) * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g4.x) * vec4(other.g1.z, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0) + vec4(self.g4.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g4.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g4.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, 0.0), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g2.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g2.y) * vec3(other.g0.z, other.g0.w, other.g0.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g2.z) * vec3(other.g0.y, other.g0.x, other.g0.w) * vec3(-1.0, 1.0, 1.0), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g0.y) * other.g1 + vec3(self.g2.y) * other.g1.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g2.z) * other.g1.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g3.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g3.y) * vec3(other.g0.z, other.g0.w, other.g0.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g3.z) * vec3(other.g0.y, other.g0.x, other.g0.w) * vec3(-1.0, 1.0, 1.0) + vec3(self.g2.x) * other.g1.xzy * vec3(0.0, -1.0, 1.0), vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.x) * vec4(other.g0.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g4.y) * vec4(other.g0.z, other.g0.w, other.g0.x, other.g1.y) * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g4.z) * vec4(other.g0.y, other.g0.x, other.g0.w, other.g1.z) * vec4(-1.0, 1.0, 1.0, 1.0) + vec4(self.g4.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + self.g1.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

MultiVector multi_vector_motor_regressive_product(MultiVector self, Motor other) {
    return MultiVector(vec2(self.g2.x) * vec2(other.g1.x) * vec2(-1.0, 0.0) + vec2(self.g2.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g2.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g3.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g3.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g3.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + self.g0 * vec2(other.g0.w), vec4(self.g4.x) * vec4(other.g1.z, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0) + vec4(self.g4.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g4.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g4.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + self.g1 * vec4(other.g0.w), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g2 * vec3(other.g0.w), vec3(self.g0.y) * other.g1 + self.g3 * vec3(other.g0.w), self.g4 * vec4(other.g0.w));
}

MultiVector multi_vector_motor_anti_wedge(MultiVector self, Motor other) {
    return MultiVector(vec2(self.g2.x) * vec2(other.g1.x) * vec2(-1.0, 0.0) + vec2(self.g2.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g2.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g3.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g3.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g3.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + self.g0 * vec2(other.g0.w), vec4(self.g4.x) * vec4(other.g1.z, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0) + vec4(self.g4.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g4.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g4.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + self.g1 * vec4(other.g0.w), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g2 * vec3(other.g0.w), vec3(self.g0.y) * other.g1 + self.g3 * vec3(other.g0.w), self.g4 * vec4(other.g0.w));
}

MultiVector multi_vector_motor_meet(MultiVector self, Motor other) {
    return MultiVector(vec2(self.g2.x) * vec2(other.g1.x) * vec2(-1.0, 0.0) + vec2(self.g2.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g2.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g3.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g3.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g3.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + self.g0 * vec2(other.g0.w), vec4(self.g4.x) * vec4(other.g1.z, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0) + vec4(self.g4.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g4.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g4.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + self.g1 * vec4(other.g0.w), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g2 * vec3(other.g0.w), vec3(self.g0.y) * other.g1 + self.g3 * vec3(other.g0.w), self.g4 * vec4(other.g0.w));
}

MultiVector multi_vector_motor_outer_product(MultiVector self, Motor other) {
    return MultiVector(vec2(self.g2.x) * vec2(other.g1.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g3.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g3.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g3.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g0.x, other.g0.w) * vec2(0.0, 1.0), vec4(0.0), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.x) * other.g1, vec4(self.g1.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

MultiVector multi_vector_motor_wedge(MultiVector self, Motor other) {
    return MultiVector(vec2(self.g2.x) * vec2(other.g1.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g3.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g3.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g3.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g0.x, other.g0.w) * vec2(0.0, 1.0), vec4(0.0), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.x) * other.g1, vec4(self.g1.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

MultiVector multi_vector_motor_join(MultiVector self, Motor other) {
    return MultiVector(vec2(self.g2.x) * vec2(other.g1.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g3.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g3.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g3.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g0.x, other.g0.w) * vec2(0.0, 1.0), vec4(0.0), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.x) * other.g1, vec4(self.g1.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

MultiVector multi_vector_motor_inner_product(MultiVector self, Motor other) {
    return MultiVector(vec2(self.g3.x) * vec2(other.g1.x) * vec2(-1.0, 0.0) + vec2(self.g3.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g3.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g0.x, other.g0.w) * vec2(0.0, 1.0), vec4(self.g1.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g4.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g0.w) + vec4(self.g1.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g0.y) * other.g1 + self.g3 * vec3(other.g0.w), vec3(self.g0.x) * other.g1, self.g1.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

MultiVector multi_vector_motor_inner_anti_product(MultiVector self, Motor other) {
    return MultiVector(vec2(self.g2.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + self.g0 * vec2(other.g0.w), self.g1 * vec4(other.g0.w), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g2 * vec3(other.g0.w), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g0.y) * other.g1 + self.g3 * vec3(other.g0.w), vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.x) * vec4(other.g0.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g4.y) * vec4(other.g0.z, other.g0.w, other.g0.x, other.g1.y) * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g4.z) * vec4(other.g0.y, other.g0.x, other.g0.w, other.g1.z) * vec4(-1.0, 1.0, 1.0, 1.0) + vec4(self.g4.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + self.g1.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

MultiVector multi_vector_motor_left_contraction(MultiVector self, Motor other) {
    return MultiVector(vec2(self.g3.x) * vec2(other.g1.x) * vec2(-1.0, 0.0) + vec2(self.g3.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g3.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g0.x, other.g0.w) * vec2(0.0, 1.0), vec4(self.g1.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g4.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g3 * vec3(other.g0.w), vec3(self.g0.x) * other.g1, self.g1.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

MultiVector multi_vector_motor_right_contraction(MultiVector self, Motor other) {
    return MultiVector(vec2(self.g3.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g3.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g3.x) * vec2(other.g1.x) * vec2(-1.0, 0.0), vec4(self.g4.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g4.wwwx * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, -1.0), vec3(self.g0.y) * other.g1, vec3(0.0), vec4(0.0));
}

MultiVector multi_vector_motor_left_anti_contraction(MultiVector self, Motor other) {
    return MultiVector(vec2(self.g2.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + self.g0 * vec2(other.g0.x, other.g0.w) * vec2(0.0, 1.0), vec4(0.0), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.y) * other.g1, vec4(self.g4.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g4.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(-1.0, 1.0, 0.0, 1.0) + vec4(self.g4.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, -1.0, 1.0, 1.0));
}

MultiVector multi_vector_motor_right_anti_contraction(MultiVector self, Motor other) {
    return MultiVector(vec2(self.g2.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + self.g0 * vec2(other.g0.w), self.g1 * vec4(other.g0.w), self.g2 * vec3(other.g0.w), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g3 * vec3(other.g0.w), vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.x) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g4.y) * vec4(other.g0.w) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g4.z) * vec4(other.g0.w) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g4.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + self.g1.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

Scalar multi_vector_motor_scalar_product(MultiVector self, Motor other) {
    return Scalar(0.0 - self.g3.x * other.g1.x - self.g3.y * other.g1.y - self.g3.z * other.g1.z);
}

Scalar multi_vector_motor_dot(MultiVector self, Motor other) {
    return Scalar(0.0 - self.g3.x * other.g1.x - self.g3.y * other.g1.y - self.g3.z * other.g1.z);
}

AntiScalar multi_vector_motor_anti_scalar_product(MultiVector self, Motor other) {
    return AntiScalar(self.g0.y * other.g0.w - self.g2.x * other.g0.x - self.g2.y * other.g0.y - self.g2.z * other.g0.z);
}

AntiScalar multi_vector_motor_anti_dot(MultiVector self, Motor other) {
    return AntiScalar(self.g0.y * other.g0.w - self.g2.x * other.g0.x - self.g2.y * other.g0.y - self.g2.z * other.g0.z);
}

Rotor multi_vector_rotor_into(MultiVector self) {
    return Rotor(vec4(self.g2.x, self.g2.y, self.g2.z, self.g0.y));
}

MultiVector multi_vector_rotor_add(MultiVector self, Rotor other) {
    return MultiVector(self.g0 + vec2(other.g0.x, other.g0.w) * vec2(0.0, 1.0), self.g1, self.g2 + vec3(other.g0.x, other.g0.y, other.g0.z), self.g3, self.g4);
}

MultiVector multi_vector_rotor_sub(MultiVector self, Rotor other) {
    return MultiVector(self.g0 - vec2(other.g0.x, other.g0.w) * vec2(0.0, 1.0), self.g1, self.g2 - vec3(other.g0.x, other.g0.y, other.g0.z), self.g3, self.g4);
}

MultiVector multi_vector_rotor_geometric_product(MultiVector self, Rotor other) {
    return MultiVector(vec2(self.g3.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g3.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g3.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g0.x, other.g0.w) * vec2(0.0, 1.0), vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g3.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g3.y) * vec3(other.g0.z, other.g0.w, other.g0.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g3.z) * vec3(other.g0.y, other.g0.x, other.g0.w) * vec3(1.0, -1.0, 1.0), vec3(0.0), vec4(self.g1.y) * other.g0.zwxz * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g1.z) * other.g0.yxwy * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g4.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 0.0));
}

MultiVector multi_vector_rotor_geometric_anti_product(MultiVector self, Rotor other) {
    return MultiVector(vec2(self.g2.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g3.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g3.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g3.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + self.g0 * vec2(other.g0.w), vec4(self.g1.y) * other.g0.zwxz * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g1.z) * other.g0.yxwy * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g4.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + self.g1.xxxw * other.g0.wzyw * vec4(1.0, -1.0, 1.0, 1.0), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g2.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g2.y) * vec3(other.g0.z, other.g0.w, other.g0.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g2.z) * vec3(other.g0.y, other.g0.x, other.g0.w) * vec3(-1.0, 1.0, 1.0), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g3.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g3.y) * vec3(other.g0.z, other.g0.w, other.g0.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g3.z) * vec3(other.g0.y, other.g0.x, other.g0.w) * vec3(-1.0, 1.0, 1.0), vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.x) * other.g0.wzyw * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g4.y) * other.g0.zwxz * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g4.z) * other.g0.yxwy * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g4.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + self.g1.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

MultiVector multi_vector_rotor_regressive_product(MultiVector self, Rotor other) {
    return MultiVector(vec2(self.g3.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g3.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g3.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + self.g0 * vec2(other.g0.w), vec4(self.g4.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + self.g1 * vec4(other.g0.w), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g2 * vec3(other.g0.w), self.g3 * vec3(other.g0.w), self.g4 * vec4(other.g0.w));
}

MultiVector multi_vector_rotor_anti_wedge(MultiVector self, Rotor other) {
    return MultiVector(vec2(self.g3.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g3.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g3.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + self.g0 * vec2(other.g0.w), vec4(self.g4.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + self.g1 * vec4(other.g0.w), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g2 * vec3(other.g0.w), self.g3 * vec3(other.g0.w), self.g4 * vec4(other.g0.w));
}

MultiVector multi_vector_rotor_meet(MultiVector self, Rotor other) {
    return MultiVector(vec2(self.g3.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g3.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g3.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + self.g0 * vec2(other.g0.w), vec4(self.g4.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + self.g1 * vec4(other.g0.w), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g2 * vec3(other.g0.w), self.g3 * vec3(other.g0.w), self.g4 * vec4(other.g0.w));
}

MultiVector multi_vector_rotor_outer_product(MultiVector self, Rotor other) {
    return MultiVector(vec2(self.g3.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g3.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g3.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g0.x, other.g0.w) * vec2(0.0, 1.0), vec4(0.0), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(0.0), vec4(self.g1.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g1.x) * other.g0.xzyx * vec4(0.0, 1.0, -1.0, 0.0));
}

MultiVector multi_vector_rotor_wedge(MultiVector self, Rotor other) {
    return MultiVector(vec2(self.g3.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g3.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g3.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g0.x, other.g0.w) * vec2(0.0, 1.0), vec4(0.0), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(0.0), vec4(self.g1.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g1.x) * other.g0.xzyx * vec4(0.0, 1.0, -1.0, 0.0));
}

MultiVector multi_vector_rotor_join(MultiVector self, Rotor other) {
    return MultiVector(vec2(self.g3.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g3.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g3.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g0.x, other.g0.w) * vec2(0.0, 1.0), vec4(0.0), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(0.0), vec4(self.g1.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g1.x) * other.g0.xzyx * vec4(0.0, 1.0, -1.0, 0.0));
}

MultiVector multi_vector_rotor_inner_product(MultiVector self, Rotor other) {
    return MultiVector(vec2(self.g0.x) * vec2(other.g0.x, other.g0.w) * vec2(0.0, 1.0), vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g3 * vec3(other.g0.w), vec3(0.0), self.g1.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

MultiVector multi_vector_rotor_inner_anti_product(MultiVector self, Rotor other) {
    return MultiVector(vec2(self.g2.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + self.g0 * vec2(other.g0.w), self.g1 * vec4(other.g0.w), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g2 * vec3(other.g0.w), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g3 * vec3(other.g0.w), vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.x) * other.g0.wzyw * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g4.y) * other.g0.zwxz * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g4.z) * other.g0.yxwy * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g4.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + self.g1.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

MultiVector multi_vector_rotor_left_contraction(MultiVector self, Rotor other) {
    return MultiVector(vec2(self.g0.x) * vec2(other.g0.x, other.g0.w) * vec2(0.0, 1.0), vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g3 * vec3(other.g0.w), vec3(0.0), self.g1.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

MultiVector multi_vector_rotor_left_anti_contraction(MultiVector self, Rotor other) {
    return MultiVector(vec2(self.g2.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + self.g0 * vec2(other.g0.x, other.g0.w) * vec2(0.0, 1.0), vec4(0.0), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(0.0), vec4(self.g4.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g4.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g4.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, 0.0));
}

MultiVector multi_vector_rotor_right_anti_contraction(MultiVector self, Rotor other) {
    return MultiVector(vec2(self.g2.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + self.g0 * vec2(other.g0.w), self.g1 * vec4(other.g0.w), self.g2 * vec3(other.g0.w), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g3 * vec3(other.g0.w), vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.x) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g4.y) * vec4(other.g0.w) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g4.z) * vec4(other.g0.w) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g4.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + self.g1.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

AntiScalar multi_vector_rotor_anti_scalar_product(MultiVector self, Rotor other) {
    return AntiScalar(self.g0.y * other.g0.w - self.g2.x * other.g0.x - self.g2.y * other.g0.y - self.g2.z * other.g0.z);
}

AntiScalar multi_vector_rotor_anti_dot(MultiVector self, Rotor other) {
    return AntiScalar(self.g0.y * other.g0.w - self.g2.x * other.g0.x - self.g2.y * other.g0.y - self.g2.z * other.g0.z);
}

Translator multi_vector_translator_into(MultiVector self) {
    return Translator(vec4(self.g3.x, self.g3.y, self.g3.z, self.g0.y));
}

MultiVector multi_vector_translator_add(MultiVector self, Translator other) {
    return MultiVector(self.g0 + vec2(other.g0.x, other.g0.w) * vec2(0.0, 1.0), self.g1, self.g2, self.g3 + vec3(other.g0.x, other.g0.y, other.g0.z), self.g4);
}

MultiVector multi_vector_translator_sub(MultiVector self, Translator other) {
    return MultiVector(self.g0 - vec2(other.g0.x, other.g0.w) * vec2(0.0, 1.0), self.g1, self.g2, self.g3 - vec3(other.g0.x, other.g0.y, other.g0.z), self.g4);
}

MultiVector multi_vector_translator_geometric_product(MultiVector self, Translator other) {
    return MultiVector(vec2(self.g2.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g3.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g3.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g3.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g0.x, other.g0.w) * vec2(0.0, 1.0), vec4(self.g1.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g4.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.w) * other.g0 + vec4(self.g1.x, self.g1.x, self.g1.x, self.g4.x) * other.g0.xzyx * vec4(0.0, 1.0, -1.0, -1.0), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g2.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g2.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g3.y) * vec3(other.g0.w) * vec3(0.0, 1.0, 0.0) + vec3(self.g3.z) * vec3(other.g0.w) * vec3(0.0, 0.0, 1.0) + vec3(self.g3.x, self.g2.x, self.g2.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, 1.0, -1.0), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g3.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g3.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g3.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0), vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g4.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g4.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g1.x, self.g4.x, self.g4.x, self.g1.x) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, -1.0));
}

MultiVector multi_vector_translator_geometric_anti_product(MultiVector self, Translator other) {
    return MultiVector(vec2(self.g2.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g2.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g2.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + self.g0 * vec2(other.g0.w), vec4(self.g1.w) * other.g0 * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g4.x) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g4.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g4.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + self.g1.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0), self.g2 * vec3(other.g0.w), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g2.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g2.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g3.y) * vec3(other.g0.w) * vec3(0.0, 1.0, 0.0) + vec3(self.g3.z) * vec3(other.g0.w) * vec3(0.0, 0.0, 1.0) + vec3(self.g3.x, self.g2.x, self.g2.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, -1.0, 1.0), vec4(self.g4.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, 1.0) + vec4(self.g4.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, 1.0) + vec4(self.g4.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g4.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, 1.0));
}

MultiVector multi_vector_translator_regressive_product(MultiVector self, Translator other) {
    return MultiVector(vec2(self.g2.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g2.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g2.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + self.g0 * vec2(other.g0.w), vec4(self.g4.x) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g4.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g4.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + self.g1 * vec4(other.g0.w), self.g2 * vec3(other.g0.w), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g3 * vec3(other.g0.w), self.g4 * vec4(other.g0.w));
}

MultiVector multi_vector_translator_anti_wedge(MultiVector self, Translator other) {
    return MultiVector(vec2(self.g2.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g2.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g2.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + self.g0 * vec2(other.g0.w), vec4(self.g4.x) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g4.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g4.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + self.g1 * vec4(other.g0.w), self.g2 * vec3(other.g0.w), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g3 * vec3(other.g0.w), self.g4 * vec4(other.g0.w));
}

MultiVector multi_vector_translator_meet(MultiVector self, Translator other) {
    return MultiVector(vec2(self.g2.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g2.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g2.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + self.g0 * vec2(other.g0.w), vec4(self.g4.x) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g4.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g4.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + self.g1 * vec4(other.g0.w), self.g2 * vec3(other.g0.w), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g3 * vec3(other.g0.w), self.g4 * vec4(other.g0.w));
}

MultiVector multi_vector_translator_outer_product(MultiVector self, Translator other) {
    return MultiVector(vec2(self.g2.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g0.x, other.g0.w) * vec2(0.0, 1.0), vec4(0.0), vec3(0.0), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g1.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

MultiVector multi_vector_translator_wedge(MultiVector self, Translator other) {
    return MultiVector(vec2(self.g2.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g0.x, other.g0.w) * vec2(0.0, 1.0), vec4(0.0), vec3(0.0), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g1.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

MultiVector multi_vector_translator_join(MultiVector self, Translator other) {
    return MultiVector(vec2(self.g2.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g0.x, other.g0.w) * vec2(0.0, 1.0), vec4(0.0), vec3(0.0), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g1.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

MultiVector multi_vector_translator_inner_product(MultiVector self, Translator other) {
    return MultiVector(vec2(self.g3.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g3.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g3.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g0.x, other.g0.w) * vec2(0.0, 1.0), vec4(self.g1.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g4.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.w) * other.g0 + vec4(self.g1.x, self.g1.x, self.g1.x, self.g4.x) * other.g0.xzyx * vec4(0.0, 1.0, -1.0, -1.0), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g3 * vec3(other.g0.w), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z), self.g1.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

MultiVector multi_vector_translator_inner_anti_product(MultiVector self, Translator other) {
    return MultiVector(self.g0 * vec2(other.g0.w), self.g1 * vec4(other.g0.w), self.g2 * vec3(other.g0.w), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g3 * vec3(other.g0.w), vec4(self.g4.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, 1.0) + vec4(self.g4.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, 1.0) + vec4(self.g4.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g4.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, 1.0));
}

MultiVector multi_vector_translator_left_contraction(MultiVector self, Translator other) {
    return MultiVector(vec2(self.g3.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g3.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g3.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x) * vec2(other.g0.x, other.g0.w) * vec2(0.0, 1.0), vec4(self.g1.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g1.x, self.g1.x, self.g1.x, self.g4.w) * other.g0.xzyw * vec4(0.0, 1.0, -1.0, 1.0), self.g3 * vec3(other.g0.w), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z), self.g1.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

MultiVector multi_vector_translator_right_contraction(MultiVector self, Translator other) {
    return MultiVector(vec2(self.g3.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g3.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g3.x) * vec2(other.g0.x) * vec2(-1.0, 0.0), vec4(self.g4.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g4.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(0.0), vec4(0.0));
}

MultiVector multi_vector_translator_left_anti_contraction(MultiVector self, Translator other) {
    return MultiVector(self.g0 * vec2(other.g0.x, other.g0.w) * vec2(0.0, 1.0), vec4(0.0), vec3(0.0), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(self.g4.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g4.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g4.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0));
}

MultiVector multi_vector_translator_right_anti_contraction(MultiVector self, Translator other) {
    return MultiVector(self.g0 * vec2(other.g0.w), self.g1 * vec4(other.g0.w), self.g2 * vec3(other.g0.w), self.g3 * vec3(other.g0.w), self.g4 * vec4(other.g0.w));
}

Scalar multi_vector_translator_scalar_product(MultiVector self, Translator other) {
    return Scalar(0.0 - self.g3.x * other.g0.x - self.g3.y * other.g0.y - self.g3.z * other.g0.z);
}

Scalar multi_vector_translator_dot(MultiVector self, Translator other) {
    return Scalar(0.0 - self.g3.x * other.g0.x - self.g3.y * other.g0.y - self.g3.z * other.g0.z);
}

AntiScalar multi_vector_translator_anti_scalar_product(MultiVector self, Translator other) {
    return AntiScalar(self.g0.y * other.g0.w);
}

AntiScalar multi_vector_translator_anti_dot(MultiVector self, Translator other) {
    return AntiScalar(self.g0.y * other.g0.w);
}

Flector multi_vector_flector_into(MultiVector self) {
    return Flector(self.g1, self.g4);
}

MultiVector multi_vector_flector_add(MultiVector self, Flector other) {
    return MultiVector(self.g0, self.g1 + other.g0, self.g2, self.g3, self.g4 + other.g1);
}

MultiVector multi_vector_flector_sub(MultiVector self, Flector other) {
    return MultiVector(self.g0, self.g1 - other.g0, self.g2, self.g3, self.g4 - other.g1);
}

MultiVector multi_vector_flector_geometric_product(MultiVector self, Flector other) {
    return MultiVector(vec2(self.g1.x) * vec2(other.g0.x, other.g1.x) + vec2(self.g1.y) * vec2(other.g0.y, other.g1.y) + vec2(self.g1.z) * vec2(other.g0.z, other.g1.z) + vec2(self.g4.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g4.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g4.z) * vec2(other.g0.z) * vec2(0.0, -1.0) - vec2(self.g4.w) * vec2(other.g1.w, other.g0.w) + vec2(self.g1.x, self.g1.w) * vec2(other.g1.x, other.g1.w) * vec2(0.0, 1.0), vec4(self.g0.x) * other.g0 + vec4(self.g2.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g2.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g2.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g3.x) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g3.y) * vec4(other.g0.z, other.g1.w, other.g0.x, other.g1.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g3.z) * vec4(other.g0.y, other.g0.x, other.g1.w, other.g1.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.x, self.g0.x, self.g0.x, self.g0.y) * other.g1.xxxw * vec4(0.0, 0.0, 0.0, -1.0), vec3(self.g1.x) * vec3(other.g0.w, other.g1.z, other.g1.y) * vec3(-1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g1.z, other.g0.w, other.g1.x) * vec3(1.0, -1.0, -1.0) + vec3(self.g1.z) * vec3(other.g1.y, other.g1.x, other.g0.w) * vec3(-1.0, 1.0, -1.0) + vec3(self.g1.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g4.x) * vec3(other.g1.w, other.g0.z, other.g0.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g4.y) * vec3(other.g0.z, other.g1.w, other.g0.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g4.z) * vec3(other.g0.y, other.g0.x, other.g1.w) * vec3(1.0, -1.0, 1.0) - vec3(self.g4.w) * vec3(other.g1.x, other.g1.y, other.g1.z), vec3(self.g1.x) * vec3(other.g1.w, other.g0.z, other.g0.y) * vec3(-1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g0.z, other.g1.w, other.g0.x) * vec3(1.0, -1.0, -1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g1.w) * vec3(-1.0, 1.0, -1.0) - vec3(self.g4.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(self.g0.x) * other.g1 + vec4(self.g2.x) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g1.w) * vec4(-1.0, -1.0, 1.0, 0.0) + vec4(self.g2.y) * vec4(other.g0.z, other.g1.w, other.g0.x, other.g0.z) * vec4(1.0, -1.0, -1.0, 0.0) + vec4(self.g2.z) * vec4(other.g0.y, other.g0.x, other.g1.w, other.g0.y) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g3.x) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g3.y) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g3.z) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y, self.g0.y, self.g0.y, self.g0.x) * other.g0.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

MultiVector multi_vector_flector_geometric_anti_product(MultiVector self, Flector other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g1.y) * vec2(1.0, 0.0) + vec2(self.g1.z) * vec2(other.g1.z) * vec2(1.0, 0.0) + vec2(self.g1.w) * vec2(other.g1.w, other.g0.w) * vec2(1.0, -1.0) + vec2(self.g4.x) * vec2(other.g0.x, other.g1.x) * vec2(-1.0, 1.0) + vec2(self.g4.y) * vec2(other.g0.y, other.g1.y) * vec2(-1.0, 1.0) + vec2(self.g4.z) * vec2(other.g0.z, other.g1.z) * vec2(-1.0, 1.0) + vec2(self.g4.w) * vec2(other.g0.w) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g1.x) * vec2(1.0, 0.0), vec4(self.g0.y) * other.g0 + vec4(self.g2.x) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g2.y) * vec4(other.g0.z, other.g1.w, other.g0.x, other.g1.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g2.z) * vec4(other.g0.y, other.g0.x, other.g1.w, other.g1.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g3.x) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.w) * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g3.y) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g1.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g3.z) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g1.y) * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * other.g1.xyzx * vec4(1.0, 1.0, 1.0, 0.0), vec3(0.0) - vec3(self.g1.w) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g4.x) * vec3(other.g0.w, other.g1.z, other.g1.y) * vec3(-1.0, 1.0, -1.0) + vec3(self.g4.y) * vec3(other.g1.z, other.g0.w, other.g1.x) * vec3(-1.0, -1.0, 1.0) + vec3(self.g4.z) * vec3(other.g1.y, other.g1.x, other.g0.w) * vec3(1.0, -1.0, -1.0), vec3(self.g1.x) * vec3(other.g0.w, other.g1.z, other.g1.y) * vec3(-1.0, 1.0, -1.0) + vec3(self.g1.y) * vec3(other.g1.z, other.g0.w, other.g1.x) * vec3(-1.0, -1.0, 1.0) + vec3(self.g1.z) * vec3(other.g1.y, other.g1.x, other.g0.w) * vec3(1.0, -1.0, -1.0) + vec3(self.g1.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g4.x) * vec3(other.g1.w, other.g0.z, other.g0.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g4.y) * vec3(other.g0.z, other.g1.w, other.g0.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g4.z) * vec3(other.g0.y, other.g0.x, other.g1.w) * vec3(-1.0, 1.0, 1.0) - vec3(self.g4.w) * vec3(other.g1.x, other.g1.y, other.g1.z), vec4(self.g0.y) * other.g1 + vec4(self.g2.x) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g2.y) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g2.z) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g3.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g3.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g3.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x) * other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0));
}

MultiVector multi_vector_flector_regressive_product(MultiVector self, Flector other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g1.y) * vec2(1.0, 0.0) + vec2(self.g1.z) * vec2(other.g1.z) * vec2(1.0, 0.0) + vec2(self.g1.w) * vec2(other.g1.w) * vec2(1.0, 0.0) + vec2(self.g4.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g4.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g4.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g4.w) * vec2(other.g0.w) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g1.x) * vec2(1.0, 0.0), vec4(self.g0.y) * other.g0 + vec4(self.g2.y) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g2.z) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g3.y) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g3.z) * other.g1.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g2.x, self.g3.x, self.g3.x, self.g2.x) * other.g1.wzyx * vec4(1.0, -1.0, 1.0, -1.0), vec3(self.g4.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g4.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g4.x) * vec3(other.g1.x, other.g1.z, other.g1.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g4.w) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g4.x, self.g4.y, self.g4.z) * vec3(other.g1.w), vec4(self.g0.y) * other.g1);
}

MultiVector multi_vector_flector_anti_wedge(MultiVector self, Flector other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g1.y) * vec2(1.0, 0.0) + vec2(self.g1.z) * vec2(other.g1.z) * vec2(1.0, 0.0) + vec2(self.g1.w) * vec2(other.g1.w) * vec2(1.0, 0.0) + vec2(self.g4.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g4.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g4.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g4.w) * vec2(other.g0.w) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g1.x) * vec2(1.0, 0.0), vec4(self.g0.y) * other.g0 + vec4(self.g2.y) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g2.z) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g3.y) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g3.z) * other.g1.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g2.x, self.g3.x, self.g3.x, self.g2.x) * other.g1.wzyx * vec4(1.0, -1.0, 1.0, -1.0), vec3(self.g4.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g4.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g4.x) * vec3(other.g1.x, other.g1.z, other.g1.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g4.w) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g4.x, self.g4.y, self.g4.z) * vec3(other.g1.w), vec4(self.g0.y) * other.g1);
}

MultiVector multi_vector_flector_meet(MultiVector self, Flector other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g1.y) * vec2(1.0, 0.0) + vec2(self.g1.z) * vec2(other.g1.z) * vec2(1.0, 0.0) + vec2(self.g1.w) * vec2(other.g1.w) * vec2(1.0, 0.0) + vec2(self.g4.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g4.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g4.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g4.w) * vec2(other.g0.w) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g1.x) * vec2(1.0, 0.0), vec4(self.g0.y) * other.g0 + vec4(self.g2.y) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g2.z) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g3.y) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g3.z) * other.g1.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g2.x, self.g3.x, self.g3.x, self.g2.x) * other.g1.wzyx * vec4(1.0, -1.0, 1.0, -1.0), vec3(self.g4.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g4.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g4.x) * vec3(other.g1.x, other.g1.z, other.g1.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g4.w) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g4.x, self.g4.y, self.g4.z) * vec3(other.g1.w), vec4(self.g0.y) * other.g1);
}

MultiVector multi_vector_flector_outer_product(MultiVector self, Flector other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g1.y) * vec2(0.0, 1.0) + vec2(self.g1.z) * vec2(other.g1.z) * vec2(0.0, 1.0) + vec2(self.g1.w) * vec2(other.g1.w) * vec2(0.0, 1.0) + vec2(self.g4.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g4.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g4.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g4.w) * vec2(other.g0.w) * vec2(0.0, -1.0) + vec2(self.g1.x) * vec2(other.g1.x) * vec2(0.0, 1.0), vec4(self.g0.x) * other.g0, vec3(self.g1.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g0.w) * vec3(-1.0), vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0), vec4(self.g0.x) * other.g1 + vec4(self.g2.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g2.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g3.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g3.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g3.x, self.g2.x, self.g2.x, self.g3.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0));
}

MultiVector multi_vector_flector_wedge(MultiVector self, Flector other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g1.y) * vec2(0.0, 1.0) + vec2(self.g1.z) * vec2(other.g1.z) * vec2(0.0, 1.0) + vec2(self.g1.w) * vec2(other.g1.w) * vec2(0.0, 1.0) + vec2(self.g4.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g4.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g4.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g4.w) * vec2(other.g0.w) * vec2(0.0, -1.0) + vec2(self.g1.x) * vec2(other.g1.x) * vec2(0.0, 1.0), vec4(self.g0.x) * other.g0, vec3(self.g1.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g0.w) * vec3(-1.0), vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0), vec4(self.g0.x) * other.g1 + vec4(self.g2.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g2.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g3.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g3.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g3.x, self.g2.x, self.g2.x, self.g3.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0));
}

MultiVector multi_vector_flector_join(MultiVector self, Flector other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g1.y) * vec2(0.0, 1.0) + vec2(self.g1.z) * vec2(other.g1.z) * vec2(0.0, 1.0) + vec2(self.g1.w) * vec2(other.g1.w) * vec2(0.0, 1.0) + vec2(self.g4.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g4.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g4.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g4.w) * vec2(other.g0.w) * vec2(0.0, -1.0) + vec2(self.g1.x) * vec2(other.g1.x) * vec2(0.0, 1.0), vec4(self.g0.x) * other.g0, vec3(self.g1.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g0.w) * vec3(-1.0), vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0), vec4(self.g0.x) * other.g1 + vec4(self.g2.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g2.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g3.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g3.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g3.x, self.g2.x, self.g2.x, self.g3.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0));
}

MultiVector multi_vector_flector_inner_product(MultiVector self, Flector other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g0.y) * vec2(1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(1.0, 0.0) + vec2(self.g4.w) * vec2(other.g1.w) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(1.0, 0.0), vec4(self.g0.x) * other.g0 + vec4(self.g2.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g2.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g2.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g3.x) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g3.y) * vec4(other.g0.z, other.g1.w, other.g0.x, other.g1.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g3.z) * vec4(other.g0.y, other.g0.x, other.g1.w, other.g1.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.x, self.g0.x, self.g0.x, self.g0.y) * other.g1.xxxw * vec4(0.0, 0.0, 0.0, -1.0), vec3(self.g1.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g4.x) * vec3(other.g0.z, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0) + vec3(self.g4.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g4.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g1.x) * vec3(other.g1.x, other.g1.z, other.g1.y) * vec3(0.0, -1.0, 1.0), vec3(0.0) - vec3(self.g4.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g1.w) * vec3(-1.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.y, self.g0.y, self.g0.y, self.g0.x) * other.g0.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

MultiVector multi_vector_flector_inner_anti_product(MultiVector self, Flector other) {
    return MultiVector(vec2(self.g4.x) * vec2(other.g1.x) * vec2(0.0, 1.0) + vec2(self.g4.y) * vec2(other.g1.y) * vec2(0.0, 1.0) + vec2(self.g4.z) * vec2(other.g1.z) * vec2(0.0, 1.0) + vec2(self.g1.x, self.g1.w) * vec2(other.g0.x, other.g0.w) * vec2(0.0, -1.0), vec4(self.g0.y) * other.g0 + vec4(self.g0.x) * other.g1.xyzx * vec4(1.0, 1.0, 1.0, 0.0), vec3(0.0) - vec3(self.g1.w) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g4.x, self.g4.y, self.g4.z) * vec3(other.g0.w) * vec3(-1.0), vec3(self.g1.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g4.x) * vec3(other.g0.z, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0) + vec3(self.g4.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g4.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.x) * vec3(other.g1.x, other.g1.z, other.g1.y) * vec3(0.0, 1.0, -1.0), vec4(self.g0.y) * other.g1 + vec4(self.g2.x) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g2.y) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g2.z) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g3.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g3.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g3.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x) * other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0));
}

MultiVector multi_vector_flector_left_contraction(MultiVector self, Flector other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g0.y) * vec2(1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(1.0, 0.0) + vec2(self.g4.w) * vec2(other.g1.w) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(1.0, 0.0), vec4(self.g0.x) * other.g0 + vec4(self.g3.y) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g3.z) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g3.x) * other.g1.wxxx * vec4(1.0, 0.0, 0.0, -1.0), vec3(self.g1.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.x) * vec3(other.g1.x, other.g1.z, other.g1.y) * vec3(0.0, -1.0, 1.0), vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g1.w) * vec3(-1.0), vec4(self.g0.x) * other.g1);
}

MultiVector multi_vector_flector_right_contraction(MultiVector self, Flector other) {
    return MultiVector(vec2(self.g1.y) * vec2(other.g0.y) * vec2(1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(1.0, 0.0) + vec2(self.g4.w) * vec2(other.g1.w) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(1.0, 0.0), vec4(self.g2.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g2.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g2.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g3.x) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g3.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g3.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g0.x, self.g0.x, self.g0.x, self.g0.y) * other.g1.xxxw * vec4(0.0, 0.0, 0.0, -1.0), vec3(self.g4.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g4.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g4.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g4.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec4(self.g0.y, self.g0.y, self.g0.y, self.g0.x) * other.g0.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

MultiVector multi_vector_flector_left_anti_contraction(MultiVector self, Flector other) {
    return MultiVector(vec2(self.g4.x) * vec2(other.g1.x) * vec2(0.0, 1.0) + vec2(self.g4.y) * vec2(other.g1.y) * vec2(0.0, 1.0) + vec2(self.g4.z) * vec2(other.g1.z) * vec2(0.0, 1.0) + vec2(self.g1.x, self.g1.w) * vec2(other.g0.x, other.g0.w) * vec2(0.0, -1.0), vec4(self.g0.y) * other.g0, vec3(self.g4.x, self.g4.y, self.g4.z) * vec3(other.g0.w) * vec3(-1.0), vec3(self.g4.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g4.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g4.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0), vec4(self.g0.y) * other.g1 + vec4(self.g2.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g2.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g2.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

MultiVector multi_vector_flector_right_anti_contraction(MultiVector self, Flector other) {
    return MultiVector(vec2(self.g4.x) * vec2(other.g1.x) * vec2(0.0, 1.0) + vec2(self.g4.y) * vec2(other.g1.y) * vec2(0.0, 1.0) + vec2(self.g4.z) * vec2(other.g1.z) * vec2(0.0, 1.0) + vec2(self.g1.x, self.g1.w) * vec2(other.g0.x, other.g0.w) * vec2(0.0, -1.0), vec4(self.g0.x) * other.g1.xyzx * vec4(1.0, 1.0, 1.0, 0.0), vec3(0.0) - vec3(self.g1.w) * vec3(other.g1.x, other.g1.y, other.g1.z), vec3(self.g1.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g1.x) * vec3(other.g1.x, other.g1.z, other.g1.y) * vec3(0.0, 1.0, -1.0), vec4(self.g2.x) * other.g1.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g2.y) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g2.z) * other.g1.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g3.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g3.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g3.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x) * other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0));
}

Scalar multi_vector_flector_scalar_product(MultiVector self, Flector other) {
    return Scalar(self.g1.x * other.g0.x + self.g1.y * other.g0.y + self.g1.z * other.g0.z - self.g4.w * other.g1.w);
}

Scalar multi_vector_flector_dot(MultiVector self, Flector other) {
    return Scalar(self.g1.x * other.g0.x + self.g1.y * other.g0.y + self.g1.z * other.g0.z - self.g4.w * other.g1.w);
}

AntiScalar multi_vector_flector_anti_scalar_product(MultiVector self, Flector other) {
    return AntiScalar(0.0 - self.g1.w * other.g0.w + self.g4.x * other.g1.x + self.g4.y * other.g1.y + self.g4.z * other.g1.z);
}

AntiScalar multi_vector_flector_anti_dot(MultiVector self, Flector other) {
    return AntiScalar(0.0 - self.g1.w * other.g0.w + self.g4.x * other.g1.x + self.g4.y * other.g1.y + self.g4.z * other.g1.z);
}

MultiVector multi_vector_multi_vector_add(MultiVector self, MultiVector other) {
    return MultiVector(self.g0 + other.g0, self.g1 + other.g1, self.g2 + other.g2, self.g3 + other.g3, self.g4 + other.g4);
}

MultiVector multi_vector_multi_vector_sub(MultiVector self, MultiVector other) {
    return MultiVector(self.g0 - other.g0, self.g1 - other.g1, self.g2 - other.g2, self.g3 - other.g3, self.g4 - other.g4);
}

MultiVector multi_vector_multi_vector_mul(MultiVector self, MultiVector other) {
    return MultiVector(self.g0 * other.g0, self.g1 * other.g1, self.g2 * other.g2, self.g3 * other.g3, self.g4 * other.g4);
}

MultiVector multi_vector_multi_vector_div(MultiVector self, MultiVector other) {
    return MultiVector(vec2(self.g0.x, self.g0.y) * vec2(1.0, 1.0) / vec2(other.g0.x, other.g0.y) * vec2(1.0, 1.0), vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.w) * vec4(1.0, 1.0, 1.0, 1.0), vec3(self.g2.x, self.g2.y, self.g2.z) * vec3(1.0, 1.0, 1.0) / vec3(other.g2.x, other.g2.y, other.g2.z) * vec3(1.0, 1.0, 1.0), vec3(self.g3.x, self.g3.y, self.g3.z) * vec3(1.0, 1.0, 1.0) / vec3(other.g3.x, other.g3.y, other.g3.z) * vec3(1.0, 1.0, 1.0), vec4(self.g4.x, self.g4.y, self.g4.z, self.g4.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g4.x, other.g4.y, other.g4.z, other.g4.w) * vec4(1.0, 1.0, 1.0, 1.0));
}

MultiVector multi_vector_multi_vector_geometric_product(MultiVector self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * other.g0 + vec2(self.g1.x) * vec2(other.g1.x, other.g4.x) + vec2(self.g1.y) * vec2(other.g1.y, other.g4.y) + vec2(self.g1.z) * vec2(other.g1.z, other.g4.z) + vec2(self.g1.w) * vec2(other.g4.w) * vec2(0.0, 1.0) + vec2(self.g2.x) * vec2(other.g3.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g3.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g3.z) * vec2(0.0, -1.0) - vec2(self.g3.x) * vec2(other.g3.x, other.g2.x) - vec2(self.g3.y) * vec2(other.g3.y, other.g2.y) - vec2(self.g3.z) * vec2(other.g3.z, other.g2.z) + vec2(self.g4.x) * vec2(other.g1.x) * vec2(0.0, -1.0) + vec2(self.g4.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g4.z) * vec2(other.g1.z) * vec2(0.0, -1.0) - vec2(self.g4.w) * vec2(other.g4.w, other.g1.w) + self.g0 * vec2(other.g0.x) * vec2(0.0, 1.0), vec4(self.g0.x) * other.g1 + vec4(self.g1.x) * vec4(other.g0.x, other.g3.z, other.g3.y, other.g2.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g3.z, other.g0.x, other.g3.x, other.g2.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g3.y, other.g3.x, other.g0.x, other.g2.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g1.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g2.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g2.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g2.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g3.x) * vec4(other.g4.w, other.g1.z, other.g1.y, other.g4.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g3.y) * vec4(other.g1.z, other.g4.w, other.g1.x, other.g4.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g3.z) * vec4(other.g1.y, other.g1.x, other.g4.w, other.g4.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g4.x) * vec4(other.g3.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.y) * vec4(other.g3.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.z) * vec4(other.g3.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.w) * vec4(other.g3.x, other.g3.y, other.g3.z, other.g0.y) + vec4(self.g0.x, self.g0.x, self.g0.x, self.g0.y) * other.g4.xxxw * vec4(0.0, 0.0, 0.0, -1.0), vec3(self.g0.x) * other.g2 + vec3(self.g0.y) * other.g3 + vec3(self.g1.x) * vec3(other.g1.w, other.g4.z, other.g4.y) * vec3(-1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g4.z, other.g1.w, other.g4.x) * vec3(1.0, -1.0, -1.0) + vec3(self.g1.z) * vec3(other.g4.y, other.g4.x, other.g1.w) * vec3(-1.0, 1.0, -1.0) + vec3(self.g1.w) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g2.x) * vec3(other.g0.x, other.g3.z, other.g3.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g2.y) * vec3(other.g3.z, other.g0.x, other.g3.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g2.z) * vec3(other.g3.y, other.g3.x, other.g0.x) * vec3(1.0, -1.0, 1.0) + vec3(self.g3.x) * vec3(other.g0.y, other.g2.z, other.g2.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g3.y) * vec3(other.g2.z, other.g0.y, other.g2.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g3.z) * vec3(other.g2.y, other.g2.x, other.g0.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g4.x) * vec3(other.g4.w, other.g1.z, other.g1.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g4.y) * vec3(other.g1.z, other.g4.w, other.g1.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g4.z) * vec3(other.g1.y, other.g1.x, other.g4.w) * vec3(1.0, -1.0, 1.0) - vec3(self.g4.w) * vec3(other.g4.x, other.g4.y, other.g4.z), vec3(self.g0.x) * other.g3 + vec3(self.g1.x) * vec3(other.g4.w, other.g1.z, other.g1.y) * vec3(-1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g1.z, other.g4.w, other.g1.x) * vec3(1.0, -1.0, -1.0) + vec3(self.g1.z) * vec3(other.g1.y, other.g1.x, other.g4.w) * vec3(-1.0, 1.0, -1.0) + vec3(self.g3.x) * vec3(other.g0.x, other.g3.z, other.g3.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g3.y) * vec3(other.g3.z, other.g0.x, other.g3.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g3.z) * vec3(other.g3.y, other.g3.x, other.g0.x) * vec3(1.0, -1.0, 1.0) - vec3(self.g4.w) * vec3(other.g1.x, other.g1.y, other.g1.z), vec4(self.g0.x) * other.g4 + vec4(self.g1.x) * vec4(other.g0.y, other.g2.z, other.g2.y, other.g3.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g2.z, other.g0.y, other.g2.x, other.g3.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g2.y, other.g2.x, other.g0.y, other.g3.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g1.w) * vec4(other.g3.x, other.g3.y, other.g3.z, other.g3.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g2.x) * vec4(other.g4.w, other.g1.z, other.g1.y, other.g4.w) * vec4(-1.0, -1.0, 1.0, 0.0) + vec4(self.g2.y) * vec4(other.g1.z, other.g4.w, other.g1.x, other.g1.z) * vec4(1.0, -1.0, -1.0, 0.0) + vec4(self.g2.z) * vec4(other.g1.y, other.g1.x, other.g4.w, other.g1.y) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g3.x) * vec4(other.g1.w, other.g4.z, other.g4.y, other.g1.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g3.y) * vec4(other.g4.z, other.g1.w, other.g4.x, other.g1.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g3.z) * vec4(other.g4.y, other.g4.x, other.g1.w, other.g1.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g4.x) * vec4(other.g0.x, other.g3.z, other.g3.y, other.g0.x) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g4.y) * vec4(other.g3.z, other.g0.x, other.g3.x, other.g3.z) * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g4.z) * vec4(other.g3.y, other.g3.x, other.g0.x, other.g3.y) * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g4.w) * vec4(other.g2.x, other.g2.y, other.g2.z, other.g0.x) + vec4(self.g0.y, self.g0.y, self.g0.y, self.g0.x) * other.g1.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

MultiVector multi_vector_multi_vector_geometric_anti_product(MultiVector self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * other.g0 + vec2(self.g1.x) * vec2(other.g4.x) * vec2(1.0, 0.0) + vec2(self.g1.y) * vec2(other.g4.y) * vec2(1.0, 0.0) + vec2(self.g1.z) * vec2(other.g4.z) * vec2(1.0, 0.0) + vec2(self.g1.w) * vec2(other.g4.w, other.g1.w) * vec2(1.0, -1.0) - vec2(self.g2.x) * vec2(other.g3.x, other.g2.x) - vec2(self.g2.y) * vec2(other.g3.y, other.g2.y) - vec2(self.g2.z) * vec2(other.g3.z, other.g2.z) + vec2(self.g3.x) * vec2(other.g2.x) * vec2(-1.0, 0.0) + vec2(self.g3.y) * vec2(other.g2.y) * vec2(-1.0, 0.0) + vec2(self.g3.z) * vec2(other.g2.z) * vec2(-1.0, 0.0) + vec2(self.g4.x) * vec2(other.g1.x, other.g4.x) * vec2(-1.0, 1.0) + vec2(self.g4.y) * vec2(other.g1.y, other.g4.y) * vec2(-1.0, 1.0) + vec2(self.g4.z) * vec2(other.g1.z, other.g4.z) * vec2(-1.0, 1.0) + vec2(self.g4.w) * vec2(other.g1.w) * vec2(-1.0, 0.0) + vec2(self.g0.x) * other.g0.yx * vec2(1.0, 0.0), vec4(self.g0.y) * other.g1 + vec4(self.g1.x) * vec4(other.g0.y, other.g2.z, other.g2.y, other.g0.y) * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g1.y) * vec4(other.g2.z, other.g0.y, other.g2.x, other.g2.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g1.z) * vec4(other.g2.y, other.g2.x, other.g0.y, other.g2.y) * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g3.x, other.g3.y, other.g3.z, other.g0.y) * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g2.x) * vec4(other.g4.w, other.g1.z, other.g1.y, other.g4.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g2.y) * vec4(other.g1.z, other.g4.w, other.g1.x, other.g4.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g2.z) * vec4(other.g1.y, other.g1.x, other.g4.w, other.g4.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g3.x) * vec4(other.g1.w, other.g4.z, other.g4.y, other.g1.w) * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g3.y) * vec4(other.g4.z, other.g1.w, other.g4.x, other.g4.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g3.z) * vec4(other.g4.y, other.g4.x, other.g1.w, other.g4.y) * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g4.x) * vec4(other.g0.x, other.g3.z, other.g3.y, other.g2.x) * vec4(-1.0, 1.0, -1.0, -1.0) + vec4(self.g4.y) * vec4(other.g3.z, other.g0.x, other.g3.x, other.g2.y) * vec4(-1.0, -1.0, 1.0, -1.0) + vec4(self.g4.z) * vec4(other.g3.y, other.g3.x, other.g0.x, other.g2.z) * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g4.w) * vec4(other.g2.x, other.g2.y, other.g2.z, other.g2.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * other.g4.xyzx * vec4(1.0, 1.0, 1.0, 0.0), vec3(self.g0.y) * other.g2 - vec3(self.g1.w) * vec3(other.g4.x, other.g4.y, other.g4.z) + vec3(self.g2.x) * vec3(other.g0.y, other.g2.z, other.g2.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g2.y) * vec3(other.g2.z, other.g0.y, other.g2.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g2.z) * vec3(other.g2.y, other.g2.x, other.g0.y) * vec3(-1.0, 1.0, 1.0) + vec3(self.g4.x) * vec3(other.g1.w, other.g4.z, other.g4.y) * vec3(-1.0, 1.0, -1.0) + vec3(self.g4.y) * vec3(other.g4.z, other.g1.w, other.g4.x) * vec3(-1.0, -1.0, 1.0) + vec3(self.g4.z) * vec3(other.g4.y, other.g4.x, other.g1.w) * vec3(1.0, -1.0, -1.0), vec3(self.g0.x) * other.g2 + vec3(self.g0.y) * other.g3 + vec3(self.g1.x) * vec3(other.g1.w, other.g4.z, other.g4.y) * vec3(-1.0, 1.0, -1.0) + vec3(self.g1.y) * vec3(other.g4.z, other.g1.w, other.g4.x) * vec3(-1.0, -1.0, 1.0) + vec3(self.g1.z) * vec3(other.g4.y, other.g4.x, other.g1.w) * vec3(1.0, -1.0, -1.0) + vec3(self.g1.w) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g2.x) * vec3(other.g0.x, other.g3.z, other.g3.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g2.y) * vec3(other.g3.z, other.g0.x, other.g3.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g2.z) * vec3(other.g3.y, other.g3.x, other.g0.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g3.x) * vec3(other.g0.y, other.g2.z, other.g2.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g3.y) * vec3(other.g2.z, other.g0.y, other.g2.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g3.z) * vec3(other.g2.y, other.g2.x, other.g0.y) * vec3(-1.0, 1.0, 1.0) + vec3(self.g4.x) * vec3(other.g4.w, other.g1.z, other.g1.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g4.y) * vec3(other.g1.z, other.g4.w, other.g1.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g4.z) * vec3(other.g1.y, other.g1.x, other.g4.w) * vec3(-1.0, 1.0, 1.0) - vec3(self.g4.w) * vec3(other.g4.x, other.g4.y, other.g4.z), vec4(self.g0.y) * other.g4 + vec4(self.g1.x) * vec4(other.g2.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * vec4(other.g2.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g2.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g2.x, other.g2.y, other.g2.z, other.g0.x) * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g2.x) * vec4(other.g1.w, other.g4.z, other.g4.y, other.g1.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g2.y) * vec4(other.g4.z, other.g1.w, other.g4.x, other.g1.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g2.z) * vec4(other.g4.y, other.g4.x, other.g1.w, other.g1.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g3.x) * vec4(other.g4.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g3.y) * vec4(other.g4.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g3.z) * vec4(other.g4.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.x) * vec4(other.g0.y, other.g2.z, other.g2.y, other.g3.x) * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g4.y) * vec4(other.g2.z, other.g0.y, other.g2.x, other.g3.y) * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g4.z) * vec4(other.g2.y, other.g2.x, other.g0.y, other.g3.z) * vec4(-1.0, 1.0, 1.0, 1.0) + vec4(self.g4.w) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * other.g1.xxxw * vec4(0.0, 0.0, 0.0, 1.0));
}

MultiVector multi_vector_multi_vector_regressive_product(MultiVector self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * other.g0 + vec2(self.g1.x) * vec2(other.g4.x) * vec2(1.0, 0.0) + vec2(self.g1.y) * vec2(other.g4.y) * vec2(1.0, 0.0) + vec2(self.g1.z) * vec2(other.g4.z) * vec2(1.0, 0.0) + vec2(self.g1.w) * vec2(other.g4.w) * vec2(1.0, 0.0) + vec2(self.g2.x) * vec2(other.g3.x) * vec2(-1.0, 0.0) + vec2(self.g2.y) * vec2(other.g3.y) * vec2(-1.0, 0.0) + vec2(self.g2.z) * vec2(other.g3.z) * vec2(-1.0, 0.0) + vec2(self.g3.x) * vec2(other.g2.x) * vec2(-1.0, 0.0) + vec2(self.g3.y) * vec2(other.g2.y) * vec2(-1.0, 0.0) + vec2(self.g3.z) * vec2(other.g2.z) * vec2(-1.0, 0.0) + vec2(self.g4.x) * vec2(other.g1.x) * vec2(-1.0, 0.0) + vec2(self.g4.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g4.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g4.w) * vec2(other.g1.w) * vec2(-1.0, 0.0) + vec2(self.g0.x) * other.g0.yx * vec2(1.0, 0.0), vec4(self.g0.y) * other.g1 + vec4(self.g2.x) * other.g4.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g2.y) * other.g4.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g2.z) * other.g4.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g3.x) * other.g4.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g3.y) * other.g4.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g3.z) * other.g4.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g4.x) * vec4(other.g3.z, other.g3.z, other.g3.y, other.g2.x) * vec4(0.0, 1.0, -1.0, -1.0) + vec4(self.g4.y) * vec4(other.g3.z, other.g3.z, other.g3.x, other.g2.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g4.z) * vec4(other.g3.y, other.g3.x, other.g3.y, other.g2.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g4.w) * vec4(other.g2.x, other.g2.y, other.g2.z, other.g2.x) * vec4(1.0, 1.0, 1.0, 0.0) + self.g1 * vec4(other.g0.y), vec3(self.g0.y) * other.g2 + vec3(self.g4.x) * vec3(other.g4.z, other.g4.z, other.g4.y) * vec3(0.0, 1.0, -1.0) + vec3(self.g4.y) * vec3(other.g4.z, other.g4.z, other.g4.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g4.z) * vec3(other.g4.y, other.g4.x, other.g4.y) * vec3(1.0, -1.0, 0.0) + self.g2 * vec3(other.g0.y), vec3(self.g0.y) * other.g3 + vec3(self.g4.x) * vec3(other.g4.w) * vec3(1.0, 0.0, 0.0) + vec3(self.g4.y) * vec3(other.g4.w) * vec3(0.0, 1.0, 0.0) + vec3(self.g4.z) * vec3(other.g4.w) * vec3(0.0, 0.0, 1.0) - vec3(self.g4.w) * vec3(other.g4.x, other.g4.y, other.g4.z) + self.g3 * vec3(other.g0.y), vec4(self.g0.y) * other.g4 + self.g4 * vec4(other.g0.y));
}

MultiVector multi_vector_multi_vector_anti_wedge(MultiVector self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * other.g0 + vec2(self.g1.x) * vec2(other.g4.x) * vec2(1.0, 0.0) + vec2(self.g1.y) * vec2(other.g4.y) * vec2(1.0, 0.0) + vec2(self.g1.z) * vec2(other.g4.z) * vec2(1.0, 0.0) + vec2(self.g1.w) * vec2(other.g4.w) * vec2(1.0, 0.0) + vec2(self.g2.x) * vec2(other.g3.x) * vec2(-1.0, 0.0) + vec2(self.g2.y) * vec2(other.g3.y) * vec2(-1.0, 0.0) + vec2(self.g2.z) * vec2(other.g3.z) * vec2(-1.0, 0.0) + vec2(self.g3.x) * vec2(other.g2.x) * vec2(-1.0, 0.0) + vec2(self.g3.y) * vec2(other.g2.y) * vec2(-1.0, 0.0) + vec2(self.g3.z) * vec2(other.g2.z) * vec2(-1.0, 0.0) + vec2(self.g4.x) * vec2(other.g1.x) * vec2(-1.0, 0.0) + vec2(self.g4.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g4.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g4.w) * vec2(other.g1.w) * vec2(-1.0, 0.0) + vec2(self.g0.x) * other.g0.yx * vec2(1.0, 0.0), vec4(self.g0.y) * other.g1 + vec4(self.g2.x) * other.g4.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g2.y) * other.g4.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g2.z) * other.g4.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g3.x) * other.g4.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g3.y) * other.g4.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g3.z) * other.g4.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g4.x) * vec4(other.g3.z, other.g3.z, other.g3.y, other.g2.x) * vec4(0.0, 1.0, -1.0, -1.0) + vec4(self.g4.y) * vec4(other.g3.z, other.g3.z, other.g3.x, other.g2.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g4.z) * vec4(other.g3.y, other.g3.x, other.g3.y, other.g2.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g4.w) * vec4(other.g2.x, other.g2.y, other.g2.z, other.g2.x) * vec4(1.0, 1.0, 1.0, 0.0) + self.g1 * vec4(other.g0.y), vec3(self.g0.y) * other.g2 + vec3(self.g4.x) * vec3(other.g4.z, other.g4.z, other.g4.y) * vec3(0.0, 1.0, -1.0) + vec3(self.g4.y) * vec3(other.g4.z, other.g4.z, other.g4.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g4.z) * vec3(other.g4.y, other.g4.x, other.g4.y) * vec3(1.0, -1.0, 0.0) + self.g2 * vec3(other.g0.y), vec3(self.g0.y) * other.g3 + vec3(self.g4.x) * vec3(other.g4.w) * vec3(1.0, 0.0, 0.0) + vec3(self.g4.y) * vec3(other.g4.w) * vec3(0.0, 1.0, 0.0) + vec3(self.g4.z) * vec3(other.g4.w) * vec3(0.0, 0.0, 1.0) - vec3(self.g4.w) * vec3(other.g4.x, other.g4.y, other.g4.z) + self.g3 * vec3(other.g0.y), vec4(self.g0.y) * other.g4 + self.g4 * vec4(other.g0.y));
}

MultiVector multi_vector_multi_vector_meet(MultiVector self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * other.g0 + vec2(self.g1.x) * vec2(other.g4.x) * vec2(1.0, 0.0) + vec2(self.g1.y) * vec2(other.g4.y) * vec2(1.0, 0.0) + vec2(self.g1.z) * vec2(other.g4.z) * vec2(1.0, 0.0) + vec2(self.g1.w) * vec2(other.g4.w) * vec2(1.0, 0.0) + vec2(self.g2.x) * vec2(other.g3.x) * vec2(-1.0, 0.0) + vec2(self.g2.y) * vec2(other.g3.y) * vec2(-1.0, 0.0) + vec2(self.g2.z) * vec2(other.g3.z) * vec2(-1.0, 0.0) + vec2(self.g3.x) * vec2(other.g2.x) * vec2(-1.0, 0.0) + vec2(self.g3.y) * vec2(other.g2.y) * vec2(-1.0, 0.0) + vec2(self.g3.z) * vec2(other.g2.z) * vec2(-1.0, 0.0) + vec2(self.g4.x) * vec2(other.g1.x) * vec2(-1.0, 0.0) + vec2(self.g4.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g4.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g4.w) * vec2(other.g1.w) * vec2(-1.0, 0.0) + vec2(self.g0.x) * other.g0.yx * vec2(1.0, 0.0), vec4(self.g0.y) * other.g1 + vec4(self.g2.x) * other.g4.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g2.y) * other.g4.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g2.z) * other.g4.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g3.x) * other.g4.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g3.y) * other.g4.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g3.z) * other.g4.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g4.x) * vec4(other.g3.z, other.g3.z, other.g3.y, other.g2.x) * vec4(0.0, 1.0, -1.0, -1.0) + vec4(self.g4.y) * vec4(other.g3.z, other.g3.z, other.g3.x, other.g2.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g4.z) * vec4(other.g3.y, other.g3.x, other.g3.y, other.g2.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g4.w) * vec4(other.g2.x, other.g2.y, other.g2.z, other.g2.x) * vec4(1.0, 1.0, 1.0, 0.0) + self.g1 * vec4(other.g0.y), vec3(self.g0.y) * other.g2 + vec3(self.g4.x) * vec3(other.g4.z, other.g4.z, other.g4.y) * vec3(0.0, 1.0, -1.0) + vec3(self.g4.y) * vec3(other.g4.z, other.g4.z, other.g4.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g4.z) * vec3(other.g4.y, other.g4.x, other.g4.y) * vec3(1.0, -1.0, 0.0) + self.g2 * vec3(other.g0.y), vec3(self.g0.y) * other.g3 + vec3(self.g4.x) * vec3(other.g4.w) * vec3(1.0, 0.0, 0.0) + vec3(self.g4.y) * vec3(other.g4.w) * vec3(0.0, 1.0, 0.0) + vec3(self.g4.z) * vec3(other.g4.w) * vec3(0.0, 0.0, 1.0) - vec3(self.g4.w) * vec3(other.g4.x, other.g4.y, other.g4.z) + self.g3 * vec3(other.g0.y), vec4(self.g0.y) * other.g4 + self.g4 * vec4(other.g0.y));
}

MultiVector multi_vector_multi_vector_outer_product(MultiVector self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * other.g0 + vec2(self.g1.x) * vec2(other.g4.x) * vec2(0.0, 1.0) + vec2(self.g1.y) * vec2(other.g4.y) * vec2(0.0, 1.0) + vec2(self.g1.z) * vec2(other.g4.z) * vec2(0.0, 1.0) + vec2(self.g1.w) * vec2(other.g4.w) * vec2(0.0, 1.0) + vec2(self.g2.x) * vec2(other.g3.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g3.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g3.z) * vec2(0.0, -1.0) + vec2(self.g3.x) * vec2(other.g2.x) * vec2(0.0, -1.0) + vec2(self.g3.y) * vec2(other.g2.y) * vec2(0.0, -1.0) + vec2(self.g3.z) * vec2(other.g2.z) * vec2(0.0, -1.0) + vec2(self.g4.x) * vec2(other.g1.x) * vec2(0.0, -1.0) + vec2(self.g4.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g4.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g4.w) * vec2(other.g1.w) * vec2(0.0, -1.0) + self.g0 * vec2(other.g0.x) * vec2(0.0, 1.0), vec4(self.g0.x) * other.g1 + self.g1 * vec4(other.g0.x), vec3(self.g0.x) * other.g2 + vec3(self.g1.w) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g2.x) * vec3(other.g0.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g2.y) * vec3(other.g0.x) * vec3(0.0, 1.0, 0.0) + vec3(self.g2.z) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0) + vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g1.w) * vec3(-1.0), vec3(self.g0.x) * other.g3 + vec3(self.g1.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g3.x) * vec3(other.g0.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g3.y) * vec3(other.g0.x) * vec3(0.0, 1.0, 0.0) + vec3(self.g3.z) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0) + vec3(self.g1.x) * vec3(other.g1.x, other.g1.z, other.g1.y) * vec3(0.0, -1.0, 1.0), vec4(self.g0.x) * other.g4 + vec4(self.g1.y) * vec4(other.g2.z, other.g2.z, other.g2.x, other.g3.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g2.y, other.g2.x, other.g2.y, other.g3.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g3.x, other.g3.y, other.g3.z, other.g3.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g2.x) * other.g1.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g2.y) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g2.z) * other.g1.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g3.x) * other.g1.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g3.y) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g3.z) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g4.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g4.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g4.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g4.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g2.x, other.g2.z, other.g2.y, other.g3.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

MultiVector multi_vector_multi_vector_wedge(MultiVector self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * other.g0 + vec2(self.g1.x) * vec2(other.g4.x) * vec2(0.0, 1.0) + vec2(self.g1.y) * vec2(other.g4.y) * vec2(0.0, 1.0) + vec2(self.g1.z) * vec2(other.g4.z) * vec2(0.0, 1.0) + vec2(self.g1.w) * vec2(other.g4.w) * vec2(0.0, 1.0) + vec2(self.g2.x) * vec2(other.g3.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g3.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g3.z) * vec2(0.0, -1.0) + vec2(self.g3.x) * vec2(other.g2.x) * vec2(0.0, -1.0) + vec2(self.g3.y) * vec2(other.g2.y) * vec2(0.0, -1.0) + vec2(self.g3.z) * vec2(other.g2.z) * vec2(0.0, -1.0) + vec2(self.g4.x) * vec2(other.g1.x) * vec2(0.0, -1.0) + vec2(self.g4.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g4.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g4.w) * vec2(other.g1.w) * vec2(0.0, -1.0) + self.g0 * vec2(other.g0.x) * vec2(0.0, 1.0), vec4(self.g0.x) * other.g1 + self.g1 * vec4(other.g0.x), vec3(self.g0.x) * other.g2 + vec3(self.g1.w) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g2.x) * vec3(other.g0.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g2.y) * vec3(other.g0.x) * vec3(0.0, 1.0, 0.0) + vec3(self.g2.z) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0) + vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g1.w) * vec3(-1.0), vec3(self.g0.x) * other.g3 + vec3(self.g1.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g3.x) * vec3(other.g0.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g3.y) * vec3(other.g0.x) * vec3(0.0, 1.0, 0.0) + vec3(self.g3.z) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0) + vec3(self.g1.x) * vec3(other.g1.x, other.g1.z, other.g1.y) * vec3(0.0, -1.0, 1.0), vec4(self.g0.x) * other.g4 + vec4(self.g1.y) * vec4(other.g2.z, other.g2.z, other.g2.x, other.g3.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g2.y, other.g2.x, other.g2.y, other.g3.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g3.x, other.g3.y, other.g3.z, other.g3.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g2.x) * other.g1.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g2.y) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g2.z) * other.g1.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g3.x) * other.g1.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g3.y) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g3.z) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g4.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g4.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g4.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g4.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g2.x, other.g2.z, other.g2.y, other.g3.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

MultiVector multi_vector_multi_vector_join(MultiVector self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * other.g0 + vec2(self.g1.x) * vec2(other.g4.x) * vec2(0.0, 1.0) + vec2(self.g1.y) * vec2(other.g4.y) * vec2(0.0, 1.0) + vec2(self.g1.z) * vec2(other.g4.z) * vec2(0.0, 1.0) + vec2(self.g1.w) * vec2(other.g4.w) * vec2(0.0, 1.0) + vec2(self.g2.x) * vec2(other.g3.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g3.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g3.z) * vec2(0.0, -1.0) + vec2(self.g3.x) * vec2(other.g2.x) * vec2(0.0, -1.0) + vec2(self.g3.y) * vec2(other.g2.y) * vec2(0.0, -1.0) + vec2(self.g3.z) * vec2(other.g2.z) * vec2(0.0, -1.0) + vec2(self.g4.x) * vec2(other.g1.x) * vec2(0.0, -1.0) + vec2(self.g4.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g4.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g4.w) * vec2(other.g1.w) * vec2(0.0, -1.0) + self.g0 * vec2(other.g0.x) * vec2(0.0, 1.0), vec4(self.g0.x) * other.g1 + self.g1 * vec4(other.g0.x), vec3(self.g0.x) * other.g2 + vec3(self.g1.w) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g2.x) * vec3(other.g0.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g2.y) * vec3(other.g0.x) * vec3(0.0, 1.0, 0.0) + vec3(self.g2.z) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0) + vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g1.w) * vec3(-1.0), vec3(self.g0.x) * other.g3 + vec3(self.g1.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g3.x) * vec3(other.g0.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g3.y) * vec3(other.g0.x) * vec3(0.0, 1.0, 0.0) + vec3(self.g3.z) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0) + vec3(self.g1.x) * vec3(other.g1.x, other.g1.z, other.g1.y) * vec3(0.0, -1.0, 1.0), vec4(self.g0.x) * other.g4 + vec4(self.g1.y) * vec4(other.g2.z, other.g2.z, other.g2.x, other.g3.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g2.y, other.g2.x, other.g2.y, other.g3.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g3.x, other.g3.y, other.g3.z, other.g3.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g2.x) * other.g1.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g2.y) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g2.z) * other.g1.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g3.x) * other.g1.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g3.y) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g3.z) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g4.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g4.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g4.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g4.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g2.x, other.g2.z, other.g2.y, other.g3.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

MultiVector multi_vector_multi_vector_inner_product(MultiVector self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * other.g0 + vec2(self.g1.x) * vec2(other.g1.x) * vec2(1.0, 0.0) + vec2(self.g1.y) * vec2(other.g1.y) * vec2(1.0, 0.0) + vec2(self.g1.z) * vec2(other.g1.z) * vec2(1.0, 0.0) + vec2(self.g3.x) * vec2(other.g3.x) * vec2(-1.0, 0.0) + vec2(self.g3.y) * vec2(other.g3.y) * vec2(-1.0, 0.0) + vec2(self.g3.z) * vec2(other.g3.z) * vec2(-1.0, 0.0) + vec2(self.g4.w) * vec2(other.g4.w) * vec2(-1.0, 0.0) + self.g0 * vec2(other.g0.x) * vec2(0.0, 1.0), vec4(self.g0.x) * other.g1 + vec4(self.g1.x) * vec4(other.g0.x, other.g3.z, other.g3.y, other.g2.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g3.z, other.g0.x, other.g3.x, other.g2.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g3.y, other.g3.x, other.g0.x, other.g2.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g1.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g2.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g2.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g2.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g3.x) * vec4(other.g4.w, other.g1.z, other.g1.y, other.g4.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g3.y) * vec4(other.g1.z, other.g4.w, other.g1.x, other.g4.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g3.z) * vec4(other.g1.y, other.g1.x, other.g4.w, other.g4.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g4.x) * vec4(other.g3.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.y) * vec4(other.g3.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.z) * vec4(other.g3.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.w) * vec4(other.g3.x, other.g3.y, other.g3.z, other.g0.y) + vec4(self.g0.x, self.g0.x, self.g0.x, self.g0.y) * other.g4.xxxw * vec4(0.0, 0.0, 0.0, -1.0), vec3(self.g0.x) * other.g2 + vec3(self.g0.y) * other.g3 + vec3(self.g1.y) * vec3(other.g4.z, other.g4.z, other.g4.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g4.y, other.g4.x, other.g4.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g2.x) * vec3(other.g0.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g2.y) * vec3(other.g0.x) * vec3(0.0, 1.0, 0.0) + vec3(self.g2.z) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0) + vec3(self.g3.x) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g3.y) * vec3(other.g0.y) * vec3(0.0, 1.0, 0.0) + vec3(self.g3.z) * vec3(other.g0.y) * vec3(0.0, 0.0, 1.0) + vec3(self.g4.x) * vec3(other.g1.z, other.g1.z, other.g1.y) * vec3(0.0, 1.0, -1.0) + vec3(self.g4.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g4.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g1.x) * vec3(other.g4.x, other.g4.z, other.g4.y) * vec3(0.0, -1.0, 1.0), vec3(self.g0.x) * other.g3 + vec3(self.g3.x) * vec3(other.g0.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g3.y) * vec3(other.g0.x) * vec3(0.0, 1.0, 0.0) + vec3(self.g3.z) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0) - vec3(self.g4.w) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g4.w) * vec3(-1.0), vec4(self.g0.x) * other.g4 + vec4(self.g1.x) * vec4(other.g0.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.y) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g4.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g4.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g4.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g4.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.y, self.g0.y, self.g0.y, self.g0.x) * other.g1.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

MultiVector multi_vector_multi_vector_inner_anti_product(MultiVector self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * other.g0 + vec2(self.g1.w) * vec2(other.g1.w) * vec2(0.0, -1.0) + vec2(self.g2.x) * vec2(other.g2.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g2.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g2.z) * vec2(0.0, -1.0) + vec2(self.g4.x) * vec2(other.g4.x) * vec2(0.0, 1.0) + vec2(self.g4.y) * vec2(other.g4.y) * vec2(0.0, 1.0) + vec2(self.g4.z) * vec2(other.g4.z) * vec2(0.0, 1.0) + vec2(self.g0.x) * other.g0.yx * vec2(1.0, 0.0), vec4(self.g0.y) * other.g1 + vec4(self.g1.x) * vec4(other.g0.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.y) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g4.x) * vec4(other.g0.x) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g4.y) * vec4(other.g0.x) * vec4(0.0, -1.0, 0.0, 0.0) + vec4(self.g4.z) * vec4(other.g0.x) * vec4(0.0, 0.0, -1.0, 0.0) + vec4(self.g0.x) * other.g4.xyzx * vec4(1.0, 1.0, 1.0, 0.0), vec3(self.g0.y) * other.g2 - vec3(self.g1.w) * vec3(other.g4.x, other.g4.y, other.g4.z) + vec3(self.g4.x) * vec3(other.g1.w) * vec3(-1.0, 0.0, 0.0) + vec3(self.g4.y) * vec3(other.g1.w) * vec3(0.0, -1.0, 0.0) + vec3(self.g4.z) * vec3(other.g1.w) * vec3(0.0, 0.0, -1.0) + self.g2 * vec3(other.g0.y), vec3(self.g0.x) * other.g2 + vec3(self.g0.y) * other.g3 + vec3(self.g1.y) * vec3(other.g4.z, other.g4.z, other.g4.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * vec3(other.g4.y, other.g4.x, other.g4.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g2.x) * vec3(other.g0.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g2.y) * vec3(other.g0.x) * vec3(0.0, 1.0, 0.0) + vec3(self.g2.z) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0) + vec3(self.g3.x) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g3.y) * vec3(other.g0.y) * vec3(0.0, 1.0, 0.0) + vec3(self.g3.z) * vec3(other.g0.y) * vec3(0.0, 0.0, 1.0) + vec3(self.g4.x) * vec3(other.g1.z, other.g1.z, other.g1.y) * vec3(0.0, -1.0, 1.0) + vec3(self.g4.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g4.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.x) * vec3(other.g4.x, other.g4.z, other.g4.y) * vec3(0.0, 1.0, -1.0), vec4(self.g0.y) * other.g4 + vec4(self.g1.x) * vec4(other.g2.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * vec4(other.g2.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g2.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g2.x, other.g2.y, other.g2.z, other.g0.x) * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g2.x) * vec4(other.g1.w, other.g4.z, other.g4.y, other.g1.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g2.y) * vec4(other.g4.z, other.g1.w, other.g4.x, other.g1.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g2.z) * vec4(other.g4.y, other.g4.x, other.g1.w, other.g1.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g3.x) * vec4(other.g4.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g3.y) * vec4(other.g4.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g3.z) * vec4(other.g4.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.x) * vec4(other.g0.y, other.g2.z, other.g2.y, other.g3.x) * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g4.y) * vec4(other.g2.z, other.g0.y, other.g2.x, other.g3.y) * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g4.z) * vec4(other.g2.y, other.g2.x, other.g0.y, other.g3.z) * vec4(-1.0, 1.0, 1.0, 1.0) + vec4(self.g4.w) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * other.g1.xxxw * vec4(0.0, 0.0, 0.0, 1.0));
}

MultiVector multi_vector_multi_vector_left_contraction(MultiVector self, MultiVector other) {
    return MultiVector(vec2(self.g0.x) * other.g0 + vec2(self.g1.y) * vec2(other.g1.y) * vec2(1.0, 0.0) + vec2(self.g1.z) * vec2(other.g1.z) * vec2(1.0, 0.0) + vec2(self.g3.x) * vec2(other.g3.x) * vec2(-1.0, 0.0) + vec2(self.g3.y) * vec2(other.g3.y) * vec2(-1.0, 0.0) + vec2(self.g3.z) * vec2(other.g3.z) * vec2(-1.0, 0.0) + vec2(self.g4.w) * vec2(other.g4.w) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g1.x) * vec2(1.0, 0.0), vec4(self.g0.x) * other.g1 + vec4(self.g1.y) * vec4(other.g3.z, other.g3.z, other.g3.x, other.g2.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g3.y, other.g3.x, other.g3.y, other.g2.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g3.x) * other.g4.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g3.y) * other.g4.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g3.z) * other.g4.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g4.w) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g3.x, other.g3.z, other.g3.y, other.g2.x) * vec4(0.0, 1.0, -1.0, -1.0), vec3(self.g0.x) * other.g2 + vec3(self.g1.y) * vec3(other.g4.z, other.g4.z, other.g4.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g4.y, other.g4.x, other.g4.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g3.x) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g3.y) * vec3(other.g0.y) * vec3(0.0, 1.0, 0.0) + vec3(self.g3.z) * vec3(other.g0.y) * vec3(0.0, 0.0, 1.0) + vec3(self.g1.x) * vec3(other.g4.x, other.g4.z, other.g4.y) * vec3(0.0, -1.0, 1.0), vec3(self.g0.x) * other.g3 + vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g4.w) * vec3(-1.0), vec4(self.g0.x) * other.g4 + self.g1.xyzx * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

MultiVector multi_vector_multi_vector_right_contraction(MultiVector self, MultiVector other) {
    return MultiVector(vec2(self.g1.x) * vec2(other.g1.x) * vec2(1.0, 0.0) + vec2(self.g1.y) * vec2(other.g1.y) * vec2(1.0, 0.0) + vec2(self.g1.z) * vec2(other.g1.z) * vec2(1.0, 0.0) + vec2(self.g3.x) * vec2(other.g3.x) * vec2(-1.0, 0.0) + vec2(self.g3.y) * vec2(other.g3.y) * vec2(-1.0, 0.0) + vec2(self.g3.z) * vec2(other.g3.z) * vec2(-1.0, 0.0) + vec2(self.g4.w) * vec2(other.g4.w) * vec2(-1.0, 0.0) + self.g0 * vec2(other.g0.x), vec4(self.g1.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g2.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g2.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g2.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g3.x) * other.g1.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g3.y) * other.g1.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g3.z) * other.g1.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g4.x) * vec4(other.g3.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.y) * vec4(other.g3.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.z) * vec4(other.g3.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.w) * vec4(other.g3.x, other.g3.y, other.g3.z, other.g3.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x, self.g0.x, self.g0.x, self.g0.y) * other.g4.xxxw * vec4(0.0, 0.0, 0.0, -1.0), vec3(self.g0.y) * other.g3 + vec3(self.g4.x) * vec3(other.g1.z, other.g1.z, other.g1.y) * vec3(0.0, 1.0, -1.0) + vec3(self.g4.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g4.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(1.0, -1.0, 0.0) + self.g2 * vec3(other.g0.x), vec3(0.0) - vec3(self.g4.w) * vec3(other.g1.x, other.g1.y, other.g1.z) + self.g3 * vec3(other.g0.x), vec4(self.g4.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g4.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g4.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g4.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.y, self.g0.y, self.g0.y, self.g0.x) * other.g1.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

MultiVector multi_vector_multi_vector_left_anti_contraction(MultiVector self, MultiVector other) {
    return MultiVector(vec2(self.g0.y) * other.g0 + vec2(self.g2.x) * vec2(other.g2.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g2.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g2.z) * vec2(0.0, -1.0) + vec2(self.g4.x) * vec2(other.g4.x) * vec2(0.0, 1.0) + vec2(self.g4.y) * vec2(other.g4.y) * vec2(0.0, 1.0) + vec2(self.g4.z) * vec2(other.g4.z) * vec2(0.0, 1.0) + vec2(self.g1.x, self.g1.w) * vec2(other.g1.x, other.g1.w) * vec2(0.0, -1.0), vec4(self.g0.y) * other.g1 + self.g4.xyzx * vec4(other.g0.x) * vec4(-1.0, -1.0, -1.0, 0.0), vec3(self.g0.y) * other.g2 + vec3(self.g4.x, self.g4.y, self.g4.z) * vec3(other.g1.w) * vec3(-1.0), vec3(self.g0.y) * other.g3 + vec3(self.g4.x) * vec3(other.g1.z, other.g1.z, other.g1.y) * vec3(0.0, -1.0, 1.0) + vec3(self.g4.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g4.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(-1.0, 1.0, 0.0) + self.g2 * vec3(other.g0.x), vec4(self.g0.y) * other.g4 + vec4(self.g2.x) * other.g1.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g2.y) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g2.z) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g4.x) * vec4(other.g2.z, other.g2.z, other.g2.y, other.g3.x) * vec4(0.0, -1.0, 1.0, 1.0) + vec4(self.g4.y) * vec4(other.g2.z, other.g2.z, other.g2.x, other.g3.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g4.z) * vec4(other.g2.y, other.g2.x, other.g2.y, other.g3.z) * vec4(-1.0, 1.0, 0.0, 1.0) + self.g1.xxxw * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

MultiVector multi_vector_multi_vector_right_anti_contraction(MultiVector self, MultiVector other) {
    return MultiVector(vec2(self.g1.w) * vec2(other.g1.w) * vec2(0.0, -1.0) + vec2(self.g2.x) * vec2(other.g2.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g2.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g2.z) * vec2(0.0, -1.0) + vec2(self.g4.x) * vec2(other.g4.x) * vec2(0.0, 1.0) + vec2(self.g4.y) * vec2(other.g4.y) * vec2(0.0, 1.0) + vec2(self.g4.z) * vec2(other.g4.z) * vec2(0.0, 1.0) + self.g0 * vec2(other.g0.y), vec4(self.g1.x) * vec4(other.g0.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.y) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * other.g4.xyzx * vec4(1.0, 1.0, 1.0, 0.0), vec3(0.0) - vec3(self.g1.w) * vec3(other.g4.x, other.g4.y, other.g4.z) + self.g2 * vec3(other.g0.y), vec3(self.g0.x) * other.g2 + vec3(self.g1.y) * vec3(other.g4.z, other.g4.z, other.g4.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * vec3(other.g4.y, other.g4.x, other.g4.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g3.x) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g3.y) * vec3(other.g0.y) * vec3(0.0, 1.0, 0.0) + vec3(self.g3.z) * vec3(other.g0.y) * vec3(0.0, 0.0, 1.0) + vec3(self.g1.x) * vec3(other.g4.x, other.g4.z, other.g4.y) * vec3(0.0, 1.0, -1.0), vec4(self.g1.x) * vec4(other.g2.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * vec4(other.g2.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g2.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g2.x, other.g2.y, other.g2.z, other.g2.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g2.x) * other.g4.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g2.y) * other.g4.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g2.z) * other.g4.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g3.x) * vec4(other.g4.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g3.y) * vec4(other.g4.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g3.z) * vec4(other.g4.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.x) * vec4(other.g0.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g4.y) * vec4(other.g0.y) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g4.z) * vec4(other.g0.y) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g4.w) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * other.g1.xxxw * vec4(0.0, 0.0, 0.0, 1.0));
}

Scalar multi_vector_multi_vector_scalar_product(MultiVector self, MultiVector other) {
    return Scalar(self.g0.x * other.g0.x + self.g1.x * other.g1.x + self.g1.y * other.g1.y + self.g1.z * other.g1.z - self.g3.x * other.g3.x - self.g3.y * other.g3.y - self.g3.z * other.g3.z - self.g4.w * other.g4.w);
}

Scalar multi_vector_multi_vector_dot(MultiVector self, MultiVector other) {
    return Scalar(self.g0.x * other.g0.x + self.g1.x * other.g1.x + self.g1.y * other.g1.y + self.g1.z * other.g1.z - self.g3.x * other.g3.x - self.g3.y * other.g3.y - self.g3.z * other.g3.z - self.g4.w * other.g4.w);
}

AntiScalar multi_vector_multi_vector_anti_scalar_product(MultiVector self, MultiVector other) {
    return AntiScalar(self.g0.y * other.g0.y - self.g1.w * other.g1.w - self.g2.x * other.g2.x - self.g2.y * other.g2.y - self.g2.z * other.g2.z + self.g4.x * other.g4.x + self.g4.y * other.g4.y + self.g4.z * other.g4.z);
}

AntiScalar multi_vector_multi_vector_anti_dot(MultiVector self, MultiVector other) {
    return AntiScalar(self.g0.y * other.g0.y - self.g1.w * other.g1.w - self.g2.x * other.g2.x - self.g2.y * other.g2.y - self.g2.z * other.g2.z + self.g4.x * other.g4.x + self.g4.y * other.g4.y + self.g4.z * other.g4.z);
}

Scalar scalar_squared_magnitude(Scalar self) {
    return scalar_scalar_scalar_product(self, scalar_reversal(self));
}

Scalar scalar_magnitude(Scalar self) {
    return Scalar(sqrt(scalar_squared_magnitude(self).g0));
}

Scalar scalar_bulk_norm(Scalar self) {
    return Scalar(sqrt(scalar_squared_magnitude(self).g0));
}

Scalar homogeneous_magnitude_squared_magnitude(HomogeneousMagnitude self) {
    return homogeneous_magnitude_homogeneous_magnitude_scalar_product(self, homogeneous_magnitude_reversal(self));
}

Scalar homogeneous_magnitude_magnitude(HomogeneousMagnitude self) {
    return Scalar(sqrt(homogeneous_magnitude_squared_magnitude(self).g0));
}

Scalar homogeneous_magnitude_bulk_norm(HomogeneousMagnitude self) {
    return Scalar(sqrt(homogeneous_magnitude_squared_magnitude(self).g0));
}

Scalar point_squared_magnitude(Point self) {
    return point_point_scalar_product(self, point_reversal(self));
}

Scalar point_magnitude(Point self) {
    return Scalar(sqrt(point_squared_magnitude(self).g0));
}

Scalar point_bulk_norm(Point self) {
    return Scalar(sqrt(point_squared_magnitude(self).g0));
}

Scalar line_squared_magnitude(Line self) {
    return line_line_scalar_product(self, line_reversal(self));
}

Scalar line_magnitude(Line self) {
    return Scalar(sqrt(line_squared_magnitude(self).g0));
}

Scalar line_bulk_norm(Line self) {
    return Scalar(sqrt(line_squared_magnitude(self).g0));
}

Scalar plane_squared_magnitude(Plane self) {
    return plane_plane_scalar_product(self, plane_reversal(self));
}

Scalar plane_magnitude(Plane self) {
    return Scalar(sqrt(plane_squared_magnitude(self).g0));
}

Scalar plane_bulk_norm(Plane self) {
    return Scalar(sqrt(plane_squared_magnitude(self).g0));
}

Scalar motor_squared_magnitude(Motor self) {
    return motor_motor_scalar_product(self, motor_reversal(self));
}

Scalar motor_magnitude(Motor self) {
    return Scalar(sqrt(motor_squared_magnitude(self).g0));
}

Scalar motor_bulk_norm(Motor self) {
    return Scalar(sqrt(motor_squared_magnitude(self).g0));
}

Scalar translator_squared_magnitude(Translator self) {
    return translator_translator_scalar_product(self, translator_reversal(self));
}

Scalar translator_magnitude(Translator self) {
    return Scalar(sqrt(translator_squared_magnitude(self).g0));
}

Scalar translator_bulk_norm(Translator self) {
    return Scalar(sqrt(translator_squared_magnitude(self).g0));
}

Scalar flector_squared_magnitude(Flector self) {
    return flector_flector_scalar_product(self, flector_reversal(self));
}

Scalar flector_magnitude(Flector self) {
    return Scalar(sqrt(flector_squared_magnitude(self).g0));
}

Scalar flector_bulk_norm(Flector self) {
    return Scalar(sqrt(flector_squared_magnitude(self).g0));
}

Scalar multi_vector_squared_magnitude(MultiVector self) {
    return multi_vector_multi_vector_scalar_product(self, multi_vector_reversal(self));
}

Scalar multi_vector_magnitude(MultiVector self) {
    return Scalar(sqrt(multi_vector_squared_magnitude(self).g0));
}

Scalar multi_vector_bulk_norm(MultiVector self) {
    return Scalar(sqrt(multi_vector_squared_magnitude(self).g0));
}

AntiScalar homogeneous_magnitude_squared_anti_magnitude(HomogeneousMagnitude self) {
    return homogeneous_magnitude_homogeneous_magnitude_anti_scalar_product(self, homogeneous_magnitude_anti_reversal(self));
}

AntiScalar homogeneous_magnitude_weight_norm(HomogeneousMagnitude self) {
    return AntiScalar(sqrt(homogeneous_magnitude_squared_anti_magnitude(self).g0));
}

HomogeneousMagnitude homogeneous_magnitude_geometric_norm(HomogeneousMagnitude self) {
    return scalar_anti_scalar_add(homogeneous_magnitude_bulk_norm(self), homogeneous_magnitude_weight_norm(self));
}

AntiScalar point_squared_anti_magnitude(Point self) {
    return point_point_anti_scalar_product(self, point_anti_reversal(self));
}

AntiScalar point_weight_norm(Point self) {
    return AntiScalar(sqrt(point_squared_anti_magnitude(self).g0));
}

HomogeneousMagnitude point_geometric_norm(Point self) {
    return scalar_anti_scalar_add(point_bulk_norm(self), point_weight_norm(self));
}

AntiScalar line_squared_anti_magnitude(Line self) {
    return line_line_anti_scalar_product(self, line_anti_reversal(self));
}

AntiScalar line_weight_norm(Line self) {
    return AntiScalar(sqrt(line_squared_anti_magnitude(self).g0));
}

HomogeneousMagnitude line_geometric_norm(Line self) {
    return scalar_anti_scalar_add(line_bulk_norm(self), line_weight_norm(self));
}

AntiScalar plane_squared_anti_magnitude(Plane self) {
    return plane_plane_anti_scalar_product(self, plane_anti_reversal(self));
}

AntiScalar plane_weight_norm(Plane self) {
    return AntiScalar(sqrt(plane_squared_anti_magnitude(self).g0));
}

HomogeneousMagnitude plane_geometric_norm(Plane self) {
    return scalar_anti_scalar_add(plane_bulk_norm(self), plane_weight_norm(self));
}

AntiScalar motor_squared_anti_magnitude(Motor self) {
    return motor_motor_anti_scalar_product(self, motor_anti_reversal(self));
}

AntiScalar motor_weight_norm(Motor self) {
    return AntiScalar(sqrt(motor_squared_anti_magnitude(self).g0));
}

HomogeneousMagnitude motor_geometric_norm(Motor self) {
    return scalar_anti_scalar_add(motor_bulk_norm(self), motor_weight_norm(self));
}

AntiScalar translator_squared_anti_magnitude(Translator self) {
    return translator_translator_anti_scalar_product(self, translator_anti_reversal(self));
}

AntiScalar translator_weight_norm(Translator self) {
    return AntiScalar(sqrt(translator_squared_anti_magnitude(self).g0));
}

HomogeneousMagnitude translator_geometric_norm(Translator self) {
    return scalar_anti_scalar_add(translator_bulk_norm(self), translator_weight_norm(self));
}

AntiScalar flector_squared_anti_magnitude(Flector self) {
    return flector_flector_anti_scalar_product(self, flector_anti_reversal(self));
}

AntiScalar flector_weight_norm(Flector self) {
    return AntiScalar(sqrt(flector_squared_anti_magnitude(self).g0));
}

HomogeneousMagnitude flector_geometric_norm(Flector self) {
    return scalar_anti_scalar_add(flector_bulk_norm(self), flector_weight_norm(self));
}

AntiScalar multi_vector_squared_anti_magnitude(MultiVector self) {
    return multi_vector_multi_vector_anti_scalar_product(self, multi_vector_anti_reversal(self));
}

AntiScalar multi_vector_weight_norm(MultiVector self) {
    return AntiScalar(sqrt(multi_vector_squared_anti_magnitude(self).g0));
}

HomogeneousMagnitude multi_vector_geometric_norm(MultiVector self) {
    return scalar_anti_scalar_add(multi_vector_bulk_norm(self), multi_vector_weight_norm(self));
}

Scalar scalar_scale(Scalar self, float other) {
    return scalar_scalar_geometric_product(self, Scalar(other));
}

Scalar scalar_signum(Scalar self) {
    return scalar_scalar_geometric_product(self, Scalar(1.0 / scalar_magnitude(self).g0));
}

HomogeneousMagnitude homogeneous_magnitude_scale(HomogeneousMagnitude self, float other) {
    return homogeneous_magnitude_scalar_geometric_product(self, Scalar(other));
}

HomogeneousMagnitude homogeneous_magnitude_signum(HomogeneousMagnitude self) {
    return homogeneous_magnitude_scalar_geometric_product(self, Scalar(1.0 / homogeneous_magnitude_magnitude(self).g0));
}

Point point_scale(Point self, float other) {
    return point_scalar_geometric_product(self, Scalar(other));
}

Point point_signum(Point self) {
    return point_scalar_geometric_product(self, Scalar(1.0 / point_magnitude(self).g0));
}

Line line_scale(Line self, float other) {
    return line_scalar_geometric_product(self, Scalar(other));
}

Line line_signum(Line self) {
    return line_scalar_geometric_product(self, Scalar(1.0 / line_magnitude(self).g0));
}

Plane plane_scale(Plane self, float other) {
    return plane_scalar_geometric_product(self, Scalar(other));
}

Plane plane_signum(Plane self) {
    return plane_scalar_geometric_product(self, Scalar(1.0 / plane_magnitude(self).g0));
}

Motor motor_scale(Motor self, float other) {
    return motor_scalar_geometric_product(self, Scalar(other));
}

Motor motor_signum(Motor self) {
    return motor_scalar_geometric_product(self, Scalar(1.0 / motor_magnitude(self).g0));
}

Translator translator_scale(Translator self, float other) {
    return translator_scalar_geometric_product(self, Scalar(other));
}

Translator translator_signum(Translator self) {
    return translator_scalar_geometric_product(self, Scalar(1.0 / translator_magnitude(self).g0));
}

Flector flector_scale(Flector self, float other) {
    return flector_scalar_geometric_product(self, Scalar(other));
}

Flector flector_signum(Flector self) {
    return flector_scalar_geometric_product(self, Scalar(1.0 / flector_magnitude(self).g0));
}

MultiVector multi_vector_scale(MultiVector self, float other) {
    return multi_vector_scalar_geometric_product(self, Scalar(other));
}

MultiVector multi_vector_signum(MultiVector self) {
    return multi_vector_scalar_geometric_product(self, Scalar(1.0 / multi_vector_magnitude(self).g0));
}

Scalar scalar_inverse(Scalar self) {
    return scalar_scalar_geometric_product(scalar_reversal(self), Scalar(1.0 / scalar_squared_magnitude(self).g0));
}

HomogeneousMagnitude homogeneous_magnitude_inverse(HomogeneousMagnitude self) {
    return homogeneous_magnitude_scalar_geometric_product(homogeneous_magnitude_reversal(self), Scalar(1.0 / homogeneous_magnitude_squared_magnitude(self).g0));
}

Point point_inverse(Point self) {
    return point_scalar_geometric_product(point_reversal(self), Scalar(1.0 / point_squared_magnitude(self).g0));
}

Line line_inverse(Line self) {
    return line_scalar_geometric_product(line_reversal(self), Scalar(1.0 / line_squared_magnitude(self).g0));
}

Plane plane_inverse(Plane self) {
    return plane_scalar_geometric_product(plane_reversal(self), Scalar(1.0 / plane_squared_magnitude(self).g0));
}

Motor motor_inverse(Motor self) {
    return motor_scalar_geometric_product(motor_reversal(self), Scalar(1.0 / motor_squared_magnitude(self).g0));
}

Translator translator_inverse(Translator self) {
    return translator_scalar_geometric_product(translator_reversal(self), Scalar(1.0 / translator_squared_magnitude(self).g0));
}

Flector flector_inverse(Flector self) {
    return flector_scalar_geometric_product(flector_reversal(self), Scalar(1.0 / flector_squared_magnitude(self).g0));
}

MultiVector multi_vector_inverse(MultiVector self) {
    return multi_vector_scalar_geometric_product(multi_vector_reversal(self), Scalar(1.0 / multi_vector_squared_magnitude(self).g0));
}

HomogeneousMagnitude homogeneous_magnitude_unitize(HomogeneousMagnitude self) {
    return homogeneous_magnitude_scalar_geometric_product(self, Scalar(1.0 / homogeneous_magnitude_weight_norm(self).g0));
}

Point point_unitize(Point self) {
    return point_scalar_geometric_product(self, Scalar(1.0 / point_weight_norm(self).g0));
}

Line line_unitize(Line self) {
    return line_scalar_geometric_product(self, Scalar(1.0 / line_weight_norm(self).g0));
}

Plane plane_unitize(Plane self) {
    return plane_scalar_geometric_product(self, Scalar(1.0 / plane_weight_norm(self).g0));
}

Motor motor_unitize(Motor self) {
    return motor_scalar_geometric_product(self, Scalar(1.0 / motor_weight_norm(self).g0));
}

Translator translator_unitize(Translator self) {
    return translator_scalar_geometric_product(self, Scalar(1.0 / translator_weight_norm(self).g0));
}

Flector flector_unitize(Flector self) {
    return flector_scalar_geometric_product(self, Scalar(1.0 / flector_weight_norm(self).g0));
}

MultiVector multi_vector_unitize(MultiVector self) {
    return multi_vector_scalar_geometric_product(self, Scalar(1.0 / multi_vector_weight_norm(self).g0));
}

Plane anti_scalar_attitude(AntiScalar self) {
    return anti_scalar_plane_regressive_product(self, Plane(vec4(0.0, 0.0, 0.0, 1.0)));
}

Plane homogeneous_magnitude_attitude(HomogeneousMagnitude self) {
    return homogeneous_magnitude_plane_regressive_product(self, Plane(vec4(0.0, 0.0, 0.0, 1.0)));
}

Scalar point_attitude(Point self) {
    return point_plane_regressive_product(self, Plane(vec4(0.0, 0.0, 0.0, 1.0)));
}

Point line_attitude(Line self) {
    return line_plane_regressive_product(self, Plane(vec4(0.0, 0.0, 0.0, 1.0)));
}

Line plane_attitude(Plane self) {
    return plane_plane_regressive_product(self, Plane(vec4(0.0, 0.0, 0.0, 1.0)));
}

Flector motor_attitude(Motor self) {
    return motor_plane_regressive_product(self, Plane(vec4(0.0, 0.0, 0.0, 1.0)));
}

Flector rotor_attitude(Rotor self) {
    return rotor_plane_regressive_product(self, Plane(vec4(0.0, 0.0, 0.0, 1.0)));
}

Flector translator_attitude(Translator self) {
    return translator_plane_regressive_product(self, Plane(vec4(0.0, 0.0, 0.0, 1.0)));
}

MultiVector flector_attitude(Flector self) {
    return flector_plane_regressive_product(self, Plane(vec4(0.0, 0.0, 0.0, 1.0)));
}

MultiVector multi_vector_attitude(MultiVector self) {
    return multi_vector_plane_regressive_product(self, Plane(vec4(0.0, 0.0, 0.0, 1.0)));
}

Scalar scalar_powi(Scalar self, int exponent) {
    if(exponent == 0) {
        return scalar_one();
    }
    Scalar x = (exponent < 0) ? scalar_inverse(self) : self;
    Scalar y = scalar_one();
    int n = abs(exponent);
    while(1 < n) {
        if((n & 1) == 1) {
            y = scalar_scalar_geometric_product(x, y);
        }
        x = scalar_scalar_geometric_product(x, x);
        n = n >> 1;
    }
    return scalar_scalar_geometric_product(x, y);
}

HomogeneousMagnitude homogeneous_magnitude_powi(HomogeneousMagnitude self, int exponent) {
    if(exponent == 0) {
        return homogeneous_magnitude_one();
    }
    HomogeneousMagnitude x = (exponent < 0) ? homogeneous_magnitude_inverse(self) : self;
    HomogeneousMagnitude y = homogeneous_magnitude_one();
    int n = abs(exponent);
    while(1 < n) {
        if((n & 1) == 1) {
            y = homogeneous_magnitude_homogeneous_magnitude_geometric_product(x, y);
        }
        x = homogeneous_magnitude_homogeneous_magnitude_geometric_product(x, x);
        n = n >> 1;
    }
    return homogeneous_magnitude_homogeneous_magnitude_geometric_product(x, y);
}

MultiVector multi_vector_powi(MultiVector self, int exponent) {
    if(exponent == 0) {
        return multi_vector_one();
    }
    MultiVector x = (exponent < 0) ? multi_vector_inverse(self) : self;
    MultiVector y = multi_vector_one();
    int n = abs(exponent);
    while(1 < n) {
        if((n & 1) == 1) {
            y = multi_vector_multi_vector_geometric_product(x, y);
        }
        x = multi_vector_multi_vector_geometric_product(x, x);
        n = n >> 1;
    }
    return multi_vector_multi_vector_geometric_product(x, y);
}

Scalar scalar_scalar_geometric_quotient(Scalar self, Scalar other) {
    return scalar_scalar_geometric_product(self, scalar_inverse(other));
}

HomogeneousMagnitude scalar_homogeneous_magnitude_geometric_quotient(Scalar self, HomogeneousMagnitude other) {
    return scalar_homogeneous_magnitude_geometric_product(self, homogeneous_magnitude_inverse(other));
}

Point scalar_point_geometric_quotient(Scalar self, Point other) {
    return scalar_point_geometric_product(self, point_inverse(other));
}

Line scalar_line_geometric_quotient(Scalar self, Line other) {
    return scalar_line_geometric_product(self, line_inverse(other));
}

Plane scalar_plane_geometric_quotient(Scalar self, Plane other) {
    return scalar_plane_geometric_product(self, plane_inverse(other));
}

Motor scalar_motor_geometric_quotient(Scalar self, Motor other) {
    return scalar_motor_geometric_product(self, motor_inverse(other));
}

Translator scalar_translator_geometric_quotient(Scalar self, Translator other) {
    return scalar_translator_geometric_product(self, translator_inverse(other));
}

Flector scalar_flector_geometric_quotient(Scalar self, Flector other) {
    return scalar_flector_geometric_product(self, flector_inverse(other));
}

MultiVector scalar_multi_vector_geometric_quotient(Scalar self, MultiVector other) {
    return scalar_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

AntiScalar anti_scalar_scalar_geometric_quotient(AntiScalar self, Scalar other) {
    return anti_scalar_scalar_geometric_product(self, scalar_inverse(other));
}

AntiScalar anti_scalar_homogeneous_magnitude_geometric_quotient(AntiScalar self, HomogeneousMagnitude other) {
    return anti_scalar_homogeneous_magnitude_geometric_product(self, homogeneous_magnitude_inverse(other));
}

Plane anti_scalar_point_geometric_quotient(AntiScalar self, Point other) {
    return anti_scalar_point_geometric_product(self, point_inverse(other));
}

Rotor anti_scalar_line_geometric_quotient(AntiScalar self, Line other) {
    return anti_scalar_line_geometric_product(self, line_inverse(other));
}

Point anti_scalar_plane_geometric_quotient(AntiScalar self, Plane other) {
    return anti_scalar_plane_geometric_product(self, plane_inverse(other));
}

Rotor anti_scalar_motor_geometric_quotient(AntiScalar self, Motor other) {
    return anti_scalar_motor_geometric_product(self, motor_inverse(other));
}

Rotor anti_scalar_translator_geometric_quotient(AntiScalar self, Translator other) {
    return anti_scalar_translator_geometric_product(self, translator_inverse(other));
}

Flector anti_scalar_flector_geometric_quotient(AntiScalar self, Flector other) {
    return anti_scalar_flector_geometric_product(self, flector_inverse(other));
}

MultiVector anti_scalar_multi_vector_geometric_quotient(AntiScalar self, MultiVector other) {
    return anti_scalar_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

HomogeneousMagnitude homogeneous_magnitude_scalar_geometric_quotient(HomogeneousMagnitude self, Scalar other) {
    return homogeneous_magnitude_scalar_geometric_product(self, scalar_inverse(other));
}

HomogeneousMagnitude homogeneous_magnitude_homogeneous_magnitude_geometric_quotient(HomogeneousMagnitude self, HomogeneousMagnitude other) {
    return homogeneous_magnitude_homogeneous_magnitude_geometric_product(self, homogeneous_magnitude_inverse(other));
}

Flector homogeneous_magnitude_point_geometric_quotient(HomogeneousMagnitude self, Point other) {
    return homogeneous_magnitude_point_geometric_product(self, point_inverse(other));
}

Line homogeneous_magnitude_line_geometric_quotient(HomogeneousMagnitude self, Line other) {
    return homogeneous_magnitude_line_geometric_product(self, line_inverse(other));
}

Flector homogeneous_magnitude_plane_geometric_quotient(HomogeneousMagnitude self, Plane other) {
    return homogeneous_magnitude_plane_geometric_product(self, plane_inverse(other));
}

Motor homogeneous_magnitude_motor_geometric_quotient(HomogeneousMagnitude self, Motor other) {
    return homogeneous_magnitude_motor_geometric_product(self, motor_inverse(other));
}

Motor homogeneous_magnitude_translator_geometric_quotient(HomogeneousMagnitude self, Translator other) {
    return homogeneous_magnitude_translator_geometric_product(self, translator_inverse(other));
}

Flector homogeneous_magnitude_flector_geometric_quotient(HomogeneousMagnitude self, Flector other) {
    return homogeneous_magnitude_flector_geometric_product(self, flector_inverse(other));
}

MultiVector homogeneous_magnitude_multi_vector_geometric_quotient(HomogeneousMagnitude self, MultiVector other) {
    return homogeneous_magnitude_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

Point point_scalar_geometric_quotient(Point self, Scalar other) {
    return point_scalar_geometric_product(self, scalar_inverse(other));
}

Flector point_homogeneous_magnitude_geometric_quotient(Point self, HomogeneousMagnitude other) {
    return point_homogeneous_magnitude_geometric_product(self, homogeneous_magnitude_inverse(other));
}

MultiVector point_point_geometric_quotient(Point self, Point other) {
    return point_point_geometric_product(self, point_inverse(other));
}

Flector point_line_geometric_quotient(Point self, Line other) {
    return point_line_geometric_product(self, line_inverse(other));
}

Motor point_plane_geometric_quotient(Point self, Plane other) {
    return point_plane_geometric_product(self, plane_inverse(other));
}

Flector point_motor_geometric_quotient(Point self, Motor other) {
    return point_motor_geometric_product(self, motor_inverse(other));
}

Flector point_translator_geometric_quotient(Point self, Translator other) {
    return point_translator_geometric_product(self, translator_inverse(other));
}

MultiVector point_flector_geometric_quotient(Point self, Flector other) {
    return point_flector_geometric_product(self, flector_inverse(other));
}

MultiVector point_multi_vector_geometric_quotient(Point self, MultiVector other) {
    return point_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

Line line_scalar_geometric_quotient(Line self, Scalar other) {
    return line_scalar_geometric_product(self, scalar_inverse(other));
}

Line line_homogeneous_magnitude_geometric_quotient(Line self, HomogeneousMagnitude other) {
    return line_homogeneous_magnitude_geometric_product(self, homogeneous_magnitude_inverse(other));
}

Flector line_point_geometric_quotient(Line self, Point other) {
    return line_point_geometric_product(self, point_inverse(other));
}

MultiVector line_line_geometric_quotient(Line self, Line other) {
    return line_line_geometric_product(self, line_inverse(other));
}

Flector line_plane_geometric_quotient(Line self, Plane other) {
    return line_plane_geometric_product(self, plane_inverse(other));
}

MultiVector line_motor_geometric_quotient(Line self, Motor other) {
    return line_motor_geometric_product(self, motor_inverse(other));
}

MultiVector line_translator_geometric_quotient(Line self, Translator other) {
    return line_translator_geometric_product(self, translator_inverse(other));
}

Flector line_flector_geometric_quotient(Line self, Flector other) {
    return line_flector_geometric_product(self, flector_inverse(other));
}

MultiVector line_multi_vector_geometric_quotient(Line self, MultiVector other) {
    return line_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

Plane plane_scalar_geometric_quotient(Plane self, Scalar other) {
    return plane_scalar_geometric_product(self, scalar_inverse(other));
}

Flector plane_homogeneous_magnitude_geometric_quotient(Plane self, HomogeneousMagnitude other) {
    return plane_homogeneous_magnitude_geometric_product(self, homogeneous_magnitude_inverse(other));
}

Motor plane_point_geometric_quotient(Plane self, Point other) {
    return plane_point_geometric_product(self, point_inverse(other));
}

Flector plane_line_geometric_quotient(Plane self, Line other) {
    return plane_line_geometric_product(self, line_inverse(other));
}

MultiVector plane_plane_geometric_quotient(Plane self, Plane other) {
    return plane_plane_geometric_product(self, plane_inverse(other));
}

Flector plane_motor_geometric_quotient(Plane self, Motor other) {
    return plane_motor_geometric_product(self, motor_inverse(other));
}

Flector plane_translator_geometric_quotient(Plane self, Translator other) {
    return plane_translator_geometric_product(self, translator_inverse(other));
}

MultiVector plane_flector_geometric_quotient(Plane self, Flector other) {
    return plane_flector_geometric_product(self, flector_inverse(other));
}

MultiVector plane_multi_vector_geometric_quotient(Plane self, MultiVector other) {
    return plane_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

Motor motor_scalar_geometric_quotient(Motor self, Scalar other) {
    return motor_scalar_geometric_product(self, scalar_inverse(other));
}

Motor motor_homogeneous_magnitude_geometric_quotient(Motor self, HomogeneousMagnitude other) {
    return motor_homogeneous_magnitude_geometric_product(self, homogeneous_magnitude_inverse(other));
}

Flector motor_point_geometric_quotient(Motor self, Point other) {
    return motor_point_geometric_product(self, point_inverse(other));
}

MultiVector motor_line_geometric_quotient(Motor self, Line other) {
    return motor_line_geometric_product(self, line_inverse(other));
}

Flector motor_plane_geometric_quotient(Motor self, Plane other) {
    return motor_plane_geometric_product(self, plane_inverse(other));
}

MultiVector motor_motor_geometric_quotient(Motor self, Motor other) {
    return motor_motor_geometric_product(self, motor_inverse(other));
}

MultiVector motor_translator_geometric_quotient(Motor self, Translator other) {
    return motor_translator_geometric_product(self, translator_inverse(other));
}

Flector motor_flector_geometric_quotient(Motor self, Flector other) {
    return motor_flector_geometric_product(self, flector_inverse(other));
}

MultiVector motor_multi_vector_geometric_quotient(Motor self, MultiVector other) {
    return motor_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

Rotor rotor_scalar_geometric_quotient(Rotor self, Scalar other) {
    return rotor_scalar_geometric_product(self, scalar_inverse(other));
}

Rotor rotor_homogeneous_magnitude_geometric_quotient(Rotor self, HomogeneousMagnitude other) {
    return rotor_homogeneous_magnitude_geometric_product(self, homogeneous_magnitude_inverse(other));
}

Flector rotor_point_geometric_quotient(Rotor self, Point other) {
    return rotor_point_geometric_product(self, point_inverse(other));
}

Rotor rotor_line_geometric_quotient(Rotor self, Line other) {
    return rotor_line_geometric_product(self, line_inverse(other));
}

Flector rotor_plane_geometric_quotient(Rotor self, Plane other) {
    return rotor_plane_geometric_product(self, plane_inverse(other));
}

Rotor rotor_motor_geometric_quotient(Rotor self, Motor other) {
    return rotor_motor_geometric_product(self, motor_inverse(other));
}

Rotor rotor_translator_geometric_quotient(Rotor self, Translator other) {
    return rotor_translator_geometric_product(self, translator_inverse(other));
}

Flector rotor_flector_geometric_quotient(Rotor self, Flector other) {
    return rotor_flector_geometric_product(self, flector_inverse(other));
}

MultiVector rotor_multi_vector_geometric_quotient(Rotor self, MultiVector other) {
    return rotor_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

Translator translator_scalar_geometric_quotient(Translator self, Scalar other) {
    return translator_scalar_geometric_product(self, scalar_inverse(other));
}

Motor translator_homogeneous_magnitude_geometric_quotient(Translator self, HomogeneousMagnitude other) {
    return translator_homogeneous_magnitude_geometric_product(self, homogeneous_magnitude_inverse(other));
}

Flector translator_point_geometric_quotient(Translator self, Point other) {
    return translator_point_geometric_product(self, point_inverse(other));
}

MultiVector translator_line_geometric_quotient(Translator self, Line other) {
    return translator_line_geometric_product(self, line_inverse(other));
}

Flector translator_plane_geometric_quotient(Translator self, Plane other) {
    return translator_plane_geometric_product(self, plane_inverse(other));
}

MultiVector translator_motor_geometric_quotient(Translator self, Motor other) {
    return translator_motor_geometric_product(self, motor_inverse(other));
}

MultiVector translator_translator_geometric_quotient(Translator self, Translator other) {
    return translator_translator_geometric_product(self, translator_inverse(other));
}

Flector translator_flector_geometric_quotient(Translator self, Flector other) {
    return translator_flector_geometric_product(self, flector_inverse(other));
}

MultiVector translator_multi_vector_geometric_quotient(Translator self, MultiVector other) {
    return translator_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

Flector flector_scalar_geometric_quotient(Flector self, Scalar other) {
    return flector_scalar_geometric_product(self, scalar_inverse(other));
}

Flector flector_homogeneous_magnitude_geometric_quotient(Flector self, HomogeneousMagnitude other) {
    return flector_homogeneous_magnitude_geometric_product(self, homogeneous_magnitude_inverse(other));
}

MultiVector flector_point_geometric_quotient(Flector self, Point other) {
    return flector_point_geometric_product(self, point_inverse(other));
}

Flector flector_line_geometric_quotient(Flector self, Line other) {
    return flector_line_geometric_product(self, line_inverse(other));
}

MultiVector flector_plane_geometric_quotient(Flector self, Plane other) {
    return flector_plane_geometric_product(self, plane_inverse(other));
}

Flector flector_motor_geometric_quotient(Flector self, Motor other) {
    return flector_motor_geometric_product(self, motor_inverse(other));
}

Flector flector_translator_geometric_quotient(Flector self, Translator other) {
    return flector_translator_geometric_product(self, translator_inverse(other));
}

MultiVector flector_flector_geometric_quotient(Flector self, Flector other) {
    return flector_flector_geometric_product(self, flector_inverse(other));
}

MultiVector flector_multi_vector_geometric_quotient(Flector self, MultiVector other) {
    return flector_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

MultiVector multi_vector_scalar_geometric_quotient(MultiVector self, Scalar other) {
    return multi_vector_scalar_geometric_product(self, scalar_inverse(other));
}

MultiVector multi_vector_homogeneous_magnitude_geometric_quotient(MultiVector self, HomogeneousMagnitude other) {
    return multi_vector_homogeneous_magnitude_geometric_product(self, homogeneous_magnitude_inverse(other));
}

MultiVector multi_vector_point_geometric_quotient(MultiVector self, Point other) {
    return multi_vector_point_geometric_product(self, point_inverse(other));
}

MultiVector multi_vector_line_geometric_quotient(MultiVector self, Line other) {
    return multi_vector_line_geometric_product(self, line_inverse(other));
}

MultiVector multi_vector_plane_geometric_quotient(MultiVector self, Plane other) {
    return multi_vector_plane_geometric_product(self, plane_inverse(other));
}

MultiVector multi_vector_motor_geometric_quotient(MultiVector self, Motor other) {
    return multi_vector_motor_geometric_product(self, motor_inverse(other));
}

MultiVector multi_vector_translator_geometric_quotient(MultiVector self, Translator other) {
    return multi_vector_translator_geometric_product(self, translator_inverse(other));
}

MultiVector multi_vector_flector_geometric_quotient(MultiVector self, Flector other) {
    return multi_vector_flector_geometric_product(self, flector_inverse(other));
}

MultiVector multi_vector_multi_vector_geometric_quotient(MultiVector self, MultiVector other) {
    return multi_vector_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

Scalar scalar_scalar_transformation(Scalar self, Scalar other) {
    return scalar_scalar_geometric_product(scalar_scalar_geometric_product(self, other), scalar_reversal(self));
}

AntiScalar scalar_anti_scalar_transformation(Scalar self, AntiScalar other) {
    return anti_scalar_scalar_geometric_product(scalar_anti_scalar_geometric_product(self, other), scalar_reversal(self));
}

HomogeneousMagnitude scalar_homogeneous_magnitude_transformation(Scalar self, HomogeneousMagnitude other) {
    return homogeneous_magnitude_scalar_geometric_product(scalar_homogeneous_magnitude_geometric_product(self, other), scalar_reversal(self));
}

Point scalar_point_transformation(Scalar self, Point other) {
    return point_scalar_geometric_product(scalar_point_geometric_product(self, other), scalar_reversal(self));
}

Line scalar_line_transformation(Scalar self, Line other) {
    return line_scalar_geometric_product(scalar_line_geometric_product(self, other), scalar_reversal(self));
}

Plane scalar_plane_transformation(Scalar self, Plane other) {
    return plane_scalar_geometric_product(scalar_plane_geometric_product(self, other), scalar_reversal(self));
}

Motor scalar_motor_transformation(Scalar self, Motor other) {
    return motor_scalar_geometric_product(scalar_motor_geometric_product(self, other), scalar_reversal(self));
}

Rotor scalar_rotor_transformation(Scalar self, Rotor other) {
    return rotor_scalar_geometric_product(scalar_rotor_geometric_product(self, other), scalar_reversal(self));
}

Translator scalar_translator_transformation(Scalar self, Translator other) {
    return translator_scalar_geometric_product(scalar_translator_geometric_product(self, other), scalar_reversal(self));
}

Flector scalar_flector_transformation(Scalar self, Flector other) {
    return flector_scalar_geometric_product(scalar_flector_geometric_product(self, other), scalar_reversal(self));
}

MultiVector scalar_multi_vector_transformation(Scalar self, MultiVector other) {
    return multi_vector_scalar_geometric_product(scalar_multi_vector_geometric_product(self, other), scalar_reversal(self));
}

Point anti_scalar_point_transformation(AntiScalar self, Point other) {
    return plane_anti_scalar_geometric_product(anti_scalar_point_geometric_product(self, other), anti_scalar_reversal(self));
}

Plane anti_scalar_plane_transformation(AntiScalar self, Plane other) {
    return point_anti_scalar_geometric_product(anti_scalar_plane_geometric_product(self, other), anti_scalar_reversal(self));
}

Flector anti_scalar_flector_transformation(AntiScalar self, Flector other) {
    return flector_anti_scalar_geometric_product(anti_scalar_flector_geometric_product(self, other), anti_scalar_reversal(self));
}

MultiVector anti_scalar_multi_vector_transformation(AntiScalar self, MultiVector other) {
    return multi_vector_anti_scalar_geometric_product(anti_scalar_multi_vector_geometric_product(self, other), anti_scalar_reversal(self));
}

Scalar homogeneous_magnitude_scalar_transformation(HomogeneousMagnitude self, Scalar other) {
    return homogeneous_magnitude_scalar_into(homogeneous_magnitude_homogeneous_magnitude_geometric_product(homogeneous_magnitude_scalar_geometric_product(self, other), homogeneous_magnitude_reversal(self)));
}

AntiScalar homogeneous_magnitude_anti_scalar_transformation(HomogeneousMagnitude self, AntiScalar other) {
    return anti_scalar_homogeneous_magnitude_geometric_product(homogeneous_magnitude_anti_scalar_geometric_product(self, other), homogeneous_magnitude_reversal(self));
}

HomogeneousMagnitude homogeneous_magnitude_homogeneous_magnitude_transformation(HomogeneousMagnitude self, HomogeneousMagnitude other) {
    return homogeneous_magnitude_homogeneous_magnitude_geometric_product(homogeneous_magnitude_homogeneous_magnitude_geometric_product(self, other), homogeneous_magnitude_reversal(self));
}

Point homogeneous_magnitude_point_transformation(HomogeneousMagnitude self, Point other) {
    return flector_point_into(flector_homogeneous_magnitude_geometric_product(homogeneous_magnitude_point_geometric_product(self, other), homogeneous_magnitude_reversal(self)));
}

Line homogeneous_magnitude_line_transformation(HomogeneousMagnitude self, Line other) {
    return line_homogeneous_magnitude_geometric_product(homogeneous_magnitude_line_geometric_product(self, other), homogeneous_magnitude_reversal(self));
}

Plane homogeneous_magnitude_plane_transformation(HomogeneousMagnitude self, Plane other) {
    return flector_plane_into(flector_homogeneous_magnitude_geometric_product(homogeneous_magnitude_plane_geometric_product(self, other), homogeneous_magnitude_reversal(self)));
}

Motor homogeneous_magnitude_motor_transformation(HomogeneousMagnitude self, Motor other) {
    return motor_homogeneous_magnitude_geometric_product(homogeneous_magnitude_motor_geometric_product(self, other), homogeneous_magnitude_reversal(self));
}

Rotor homogeneous_magnitude_rotor_transformation(HomogeneousMagnitude self, Rotor other) {
    return rotor_homogeneous_magnitude_geometric_product(homogeneous_magnitude_rotor_geometric_product(self, other), homogeneous_magnitude_reversal(self));
}

Translator homogeneous_magnitude_translator_transformation(HomogeneousMagnitude self, Translator other) {
    return motor_translator_into(motor_homogeneous_magnitude_geometric_product(homogeneous_magnitude_translator_geometric_product(self, other), homogeneous_magnitude_reversal(self)));
}

Flector homogeneous_magnitude_flector_transformation(HomogeneousMagnitude self, Flector other) {
    return flector_homogeneous_magnitude_geometric_product(homogeneous_magnitude_flector_geometric_product(self, other), homogeneous_magnitude_reversal(self));
}

MultiVector homogeneous_magnitude_multi_vector_transformation(HomogeneousMagnitude self, MultiVector other) {
    return multi_vector_homogeneous_magnitude_geometric_product(homogeneous_magnitude_multi_vector_geometric_product(self, other), homogeneous_magnitude_reversal(self));
}

Scalar point_scalar_transformation(Point self, Scalar other) {
    return multi_vector_scalar_into(point_point_geometric_product(point_scalar_geometric_product(self, other), point_reversal(self)));
}

AntiScalar point_anti_scalar_transformation(Point self, AntiScalar other) {
    return motor_anti_scalar_into(plane_point_geometric_product(point_anti_scalar_geometric_product(self, other), point_reversal(self)));
}

HomogeneousMagnitude point_homogeneous_magnitude_transformation(Point self, HomogeneousMagnitude other) {
    return multi_vector_homogeneous_magnitude_into(flector_point_geometric_product(point_homogeneous_magnitude_geometric_product(self, other), point_reversal(self)));
}

Point point_point_transformation(Point self, Point other) {
    return multi_vector_point_into(multi_vector_point_geometric_product(point_point_geometric_product(self, other), point_reversal(self)));
}

Line point_line_transformation(Point self, Line other) {
    return multi_vector_line_into(flector_point_geometric_product(point_line_geometric_product(self, other), point_reversal(self)));
}

Plane point_plane_transformation(Point self, Plane other) {
    return flector_plane_into(motor_point_geometric_product(point_plane_geometric_product(self, other), point_reversal(self)));
}

Motor point_motor_transformation(Point self, Motor other) {
    return multi_vector_motor_into(flector_point_geometric_product(point_motor_geometric_product(self, other), point_reversal(self)));
}

Rotor point_rotor_transformation(Point self, Rotor other) {
    return multi_vector_rotor_into(flector_point_geometric_product(point_rotor_geometric_product(self, other), point_reversal(self)));
}

Translator point_translator_transformation(Point self, Translator other) {
    return multi_vector_translator_into(flector_point_geometric_product(point_translator_geometric_product(self, other), point_reversal(self)));
}

Flector point_flector_transformation(Point self, Flector other) {
    return multi_vector_flector_into(multi_vector_point_geometric_product(point_flector_geometric_product(self, other), point_reversal(self)));
}

MultiVector point_multi_vector_transformation(Point self, MultiVector other) {
    return multi_vector_point_geometric_product(point_multi_vector_geometric_product(self, other), point_reversal(self));
}

Scalar line_scalar_transformation(Line self, Scalar other) {
    return multi_vector_scalar_into(line_line_geometric_product(line_scalar_geometric_product(self, other), line_reversal(self)));
}

AntiScalar line_anti_scalar_transformation(Line self, AntiScalar other) {
    return rotor_anti_scalar_into(rotor_line_geometric_product(line_anti_scalar_geometric_product(self, other), line_reversal(self)));
}

HomogeneousMagnitude line_homogeneous_magnitude_transformation(Line self, HomogeneousMagnitude other) {
    return multi_vector_homogeneous_magnitude_into(line_line_geometric_product(line_homogeneous_magnitude_geometric_product(self, other), line_reversal(self)));
}

Point line_point_transformation(Line self, Point other) {
    return flector_point_into(flector_line_geometric_product(line_point_geometric_product(self, other), line_reversal(self)));
}

Line line_line_transformation(Line self, Line other) {
    return multi_vector_line_into(multi_vector_line_geometric_product(line_line_geometric_product(self, other), line_reversal(self)));
}

Plane line_plane_transformation(Line self, Plane other) {
    return flector_plane_into(flector_line_geometric_product(line_plane_geometric_product(self, other), line_reversal(self)));
}

Motor line_motor_transformation(Line self, Motor other) {
    return multi_vector_motor_into(multi_vector_line_geometric_product(line_motor_geometric_product(self, other), line_reversal(self)));
}

Rotor line_rotor_transformation(Line self, Rotor other) {
    return rotor_line_geometric_product(line_rotor_geometric_product(self, other), line_reversal(self));
}

Translator line_translator_transformation(Line self, Translator other) {
    return multi_vector_translator_into(multi_vector_line_geometric_product(line_translator_geometric_product(self, other), line_reversal(self)));
}

Flector line_flector_transformation(Line self, Flector other) {
    return flector_line_geometric_product(line_flector_geometric_product(self, other), line_reversal(self));
}

MultiVector line_multi_vector_transformation(Line self, MultiVector other) {
    return multi_vector_line_geometric_product(line_multi_vector_geometric_product(self, other), line_reversal(self));
}

Scalar plane_scalar_transformation(Plane self, Scalar other) {
    return multi_vector_scalar_into(plane_plane_geometric_product(plane_scalar_geometric_product(self, other), plane_reversal(self)));
}

AntiScalar plane_anti_scalar_transformation(Plane self, AntiScalar other) {
    return motor_anti_scalar_into(point_plane_geometric_product(plane_anti_scalar_geometric_product(self, other), plane_reversal(self)));
}

HomogeneousMagnitude plane_homogeneous_magnitude_transformation(Plane self, HomogeneousMagnitude other) {
    return multi_vector_homogeneous_magnitude_into(flector_plane_geometric_product(plane_homogeneous_magnitude_geometric_product(self, other), plane_reversal(self)));
}

Point plane_point_transformation(Plane self, Point other) {
    return flector_point_into(motor_plane_geometric_product(plane_point_geometric_product(self, other), plane_reversal(self)));
}

Line plane_line_transformation(Plane self, Line other) {
    return multi_vector_line_into(flector_plane_geometric_product(plane_line_geometric_product(self, other), plane_reversal(self)));
}

Plane plane_plane_transformation(Plane self, Plane other) {
    return multi_vector_plane_into(multi_vector_plane_geometric_product(plane_plane_geometric_product(self, other), plane_reversal(self)));
}

Motor plane_motor_transformation(Plane self, Motor other) {
    return multi_vector_motor_into(flector_plane_geometric_product(plane_motor_geometric_product(self, other), plane_reversal(self)));
}

Rotor plane_rotor_transformation(Plane self, Rotor other) {
    return multi_vector_rotor_into(flector_plane_geometric_product(plane_rotor_geometric_product(self, other), plane_reversal(self)));
}

Translator plane_translator_transformation(Plane self, Translator other) {
    return multi_vector_translator_into(flector_plane_geometric_product(plane_translator_geometric_product(self, other), plane_reversal(self)));
}

Flector plane_flector_transformation(Plane self, Flector other) {
    return multi_vector_flector_into(multi_vector_plane_geometric_product(plane_flector_geometric_product(self, other), plane_reversal(self)));
}

MultiVector plane_multi_vector_transformation(Plane self, MultiVector other) {
    return multi_vector_plane_geometric_product(plane_multi_vector_geometric_product(self, other), plane_reversal(self));
}

Scalar motor_scalar_transformation(Motor self, Scalar other) {
    return multi_vector_scalar_into(motor_motor_geometric_product(motor_scalar_geometric_product(self, other), motor_reversal(self)));
}

AntiScalar motor_anti_scalar_transformation(Motor self, AntiScalar other) {
    return rotor_anti_scalar_into(rotor_motor_geometric_product(motor_anti_scalar_geometric_product(self, other), motor_reversal(self)));
}

HomogeneousMagnitude motor_homogeneous_magnitude_transformation(Motor self, HomogeneousMagnitude other) {
    return multi_vector_homogeneous_magnitude_into(motor_motor_geometric_product(motor_homogeneous_magnitude_geometric_product(self, other), motor_reversal(self)));
}

Point motor_point_transformation(Motor self, Point other) {
    return flector_point_into(flector_motor_geometric_product(motor_point_geometric_product(self, other), motor_reversal(self)));
}

Line motor_line_transformation(Motor self, Line other) {
    return multi_vector_line_into(multi_vector_motor_geometric_product(motor_line_geometric_product(self, other), motor_reversal(self)));
}

Plane motor_plane_transformation(Motor self, Plane other) {
    return flector_plane_into(flector_motor_geometric_product(motor_plane_geometric_product(self, other), motor_reversal(self)));
}

Motor motor_motor_transformation(Motor self, Motor other) {
    return multi_vector_motor_into(multi_vector_motor_geometric_product(motor_motor_geometric_product(self, other), motor_reversal(self)));
}

Rotor motor_rotor_transformation(Motor self, Rotor other) {
    return rotor_motor_geometric_product(motor_rotor_geometric_product(self, other), motor_reversal(self));
}

Translator motor_translator_transformation(Motor self, Translator other) {
    return multi_vector_translator_into(multi_vector_motor_geometric_product(motor_translator_geometric_product(self, other), motor_reversal(self)));
}

Flector motor_flector_transformation(Motor self, Flector other) {
    return flector_motor_geometric_product(motor_flector_geometric_product(self, other), motor_reversal(self));
}

MultiVector motor_multi_vector_transformation(Motor self, MultiVector other) {
    return multi_vector_motor_geometric_product(motor_multi_vector_geometric_product(self, other), motor_reversal(self));
}

Point rotor_point_transformation(Rotor self, Point other) {
    return flector_point_into(flector_rotor_geometric_product(rotor_point_geometric_product(self, other), rotor_reversal(self)));
}

Plane rotor_plane_transformation(Rotor self, Plane other) {
    return flector_plane_into(flector_rotor_geometric_product(rotor_plane_geometric_product(self, other), rotor_reversal(self)));
}

Flector rotor_flector_transformation(Rotor self, Flector other) {
    return flector_rotor_geometric_product(rotor_flector_geometric_product(self, other), rotor_reversal(self));
}

MultiVector rotor_multi_vector_transformation(Rotor self, MultiVector other) {
    return multi_vector_rotor_geometric_product(rotor_multi_vector_geometric_product(self, other), rotor_reversal(self));
}

Scalar translator_scalar_transformation(Translator self, Scalar other) {
    return multi_vector_scalar_into(translator_translator_geometric_product(translator_scalar_geometric_product(self, other), translator_reversal(self)));
}

AntiScalar translator_anti_scalar_transformation(Translator self, AntiScalar other) {
    return rotor_anti_scalar_into(rotor_translator_geometric_product(translator_anti_scalar_geometric_product(self, other), translator_reversal(self)));
}

HomogeneousMagnitude translator_homogeneous_magnitude_transformation(Translator self, HomogeneousMagnitude other) {
    return multi_vector_homogeneous_magnitude_into(motor_translator_geometric_product(translator_homogeneous_magnitude_geometric_product(self, other), translator_reversal(self)));
}

Point translator_point_transformation(Translator self, Point other) {
    return flector_point_into(flector_translator_geometric_product(translator_point_geometric_product(self, other), translator_reversal(self)));
}

Line translator_line_transformation(Translator self, Line other) {
    return multi_vector_line_into(multi_vector_translator_geometric_product(translator_line_geometric_product(self, other), translator_reversal(self)));
}

Plane translator_plane_transformation(Translator self, Plane other) {
    return flector_plane_into(flector_translator_geometric_product(translator_plane_geometric_product(self, other), translator_reversal(self)));
}

Motor translator_motor_transformation(Translator self, Motor other) {
    return multi_vector_motor_into(multi_vector_translator_geometric_product(translator_motor_geometric_product(self, other), translator_reversal(self)));
}

Rotor translator_rotor_transformation(Translator self, Rotor other) {
    return rotor_translator_geometric_product(translator_rotor_geometric_product(self, other), translator_reversal(self));
}

Translator translator_translator_transformation(Translator self, Translator other) {
    return multi_vector_translator_into(multi_vector_translator_geometric_product(translator_translator_geometric_product(self, other), translator_reversal(self)));
}

Flector translator_flector_transformation(Translator self, Flector other) {
    return flector_translator_geometric_product(translator_flector_geometric_product(self, other), translator_reversal(self));
}

MultiVector translator_multi_vector_transformation(Translator self, MultiVector other) {
    return multi_vector_translator_geometric_product(translator_multi_vector_geometric_product(self, other), translator_reversal(self));
}

Scalar flector_scalar_transformation(Flector self, Scalar other) {
    return multi_vector_scalar_into(flector_flector_geometric_product(flector_scalar_geometric_product(self, other), flector_reversal(self)));
}

AntiScalar flector_anti_scalar_transformation(Flector self, AntiScalar other) {
    return multi_vector_anti_scalar_into(flector_flector_geometric_product(flector_anti_scalar_geometric_product(self, other), flector_reversal(self)));
}

HomogeneousMagnitude flector_homogeneous_magnitude_transformation(Flector self, HomogeneousMagnitude other) {
    return multi_vector_homogeneous_magnitude_into(flector_flector_geometric_product(flector_homogeneous_magnitude_geometric_product(self, other), flector_reversal(self)));
}

Point flector_point_transformation(Flector self, Point other) {
    return multi_vector_point_into(multi_vector_flector_geometric_product(flector_point_geometric_product(self, other), flector_reversal(self)));
}

Line flector_line_transformation(Flector self, Line other) {
    return multi_vector_line_into(flector_flector_geometric_product(flector_line_geometric_product(self, other), flector_reversal(self)));
}

Plane flector_plane_transformation(Flector self, Plane other) {
    return multi_vector_plane_into(multi_vector_flector_geometric_product(flector_plane_geometric_product(self, other), flector_reversal(self)));
}

Motor flector_motor_transformation(Flector self, Motor other) {
    return multi_vector_motor_into(flector_flector_geometric_product(flector_motor_geometric_product(self, other), flector_reversal(self)));
}

Rotor flector_rotor_transformation(Flector self, Rotor other) {
    return multi_vector_rotor_into(flector_flector_geometric_product(flector_rotor_geometric_product(self, other), flector_reversal(self)));
}

Translator flector_translator_transformation(Flector self, Translator other) {
    return multi_vector_translator_into(flector_flector_geometric_product(flector_translator_geometric_product(self, other), flector_reversal(self)));
}

Flector flector_flector_transformation(Flector self, Flector other) {
    return multi_vector_flector_into(multi_vector_flector_geometric_product(flector_flector_geometric_product(self, other), flector_reversal(self)));
}

MultiVector flector_multi_vector_transformation(Flector self, MultiVector other) {
    return multi_vector_flector_geometric_product(flector_multi_vector_geometric_product(self, other), flector_reversal(self));
}

Scalar multi_vector_scalar_transformation(MultiVector self, Scalar other) {
    return multi_vector_scalar_into(multi_vector_multi_vector_geometric_product(multi_vector_scalar_geometric_product(self, other), multi_vector_reversal(self)));
}

AntiScalar multi_vector_anti_scalar_transformation(MultiVector self, AntiScalar other) {
    return multi_vector_anti_scalar_into(multi_vector_multi_vector_geometric_product(multi_vector_anti_scalar_geometric_product(self, other), multi_vector_reversal(self)));
}

HomogeneousMagnitude multi_vector_homogeneous_magnitude_transformation(MultiVector self, HomogeneousMagnitude other) {
    return multi_vector_homogeneous_magnitude_into(multi_vector_multi_vector_geometric_product(multi_vector_homogeneous_magnitude_geometric_product(self, other), multi_vector_reversal(self)));
}

Point multi_vector_point_transformation(MultiVector self, Point other) {
    return multi_vector_point_into(multi_vector_multi_vector_geometric_product(multi_vector_point_geometric_product(self, other), multi_vector_reversal(self)));
}

Line multi_vector_line_transformation(MultiVector self, Line other) {
    return multi_vector_line_into(multi_vector_multi_vector_geometric_product(multi_vector_line_geometric_product(self, other), multi_vector_reversal(self)));
}

Plane multi_vector_plane_transformation(MultiVector self, Plane other) {
    return multi_vector_plane_into(multi_vector_multi_vector_geometric_product(multi_vector_plane_geometric_product(self, other), multi_vector_reversal(self)));
}

Motor multi_vector_motor_transformation(MultiVector self, Motor other) {
    return multi_vector_motor_into(multi_vector_multi_vector_geometric_product(multi_vector_motor_geometric_product(self, other), multi_vector_reversal(self)));
}

Rotor multi_vector_rotor_transformation(MultiVector self, Rotor other) {
    return multi_vector_rotor_into(multi_vector_multi_vector_geometric_product(multi_vector_rotor_geometric_product(self, other), multi_vector_reversal(self)));
}

Translator multi_vector_translator_transformation(MultiVector self, Translator other) {
    return multi_vector_translator_into(multi_vector_multi_vector_geometric_product(multi_vector_translator_geometric_product(self, other), multi_vector_reversal(self)));
}

Flector multi_vector_flector_transformation(MultiVector self, Flector other) {
    return multi_vector_flector_into(multi_vector_multi_vector_geometric_product(multi_vector_flector_geometric_product(self, other), multi_vector_reversal(self)));
}

MultiVector multi_vector_multi_vector_transformation(MultiVector self, MultiVector other) {
    return multi_vector_multi_vector_geometric_product(multi_vector_multi_vector_geometric_product(self, other), multi_vector_reversal(self));
}

Point scalar_point_sandwich(Scalar self, Point other) {
    return plane_scalar_geometric_anti_product(scalar_point_geometric_anti_product(self, other), scalar_anti_reversal(self));
}

Scalar scalar_line_sandwich(Scalar self, Line other) {
    return translator_scalar_geometric_anti_product(scalar_line_geometric_anti_product(self, other), scalar_anti_reversal(self));
}

Plane scalar_plane_sandwich(Scalar self, Plane other) {
    return point_scalar_geometric_anti_product(scalar_plane_geometric_anti_product(self, other), scalar_anti_reversal(self));
}

Motor scalar_motor_sandwich(Scalar self, Motor other) {
    return multi_vector_motor_into(multi_vector_scalar_geometric_anti_product(scalar_motor_geometric_anti_product(self, other), scalar_anti_reversal(self)));
}

Rotor scalar_rotor_sandwich(Scalar self, Rotor other) {
    return multi_vector_rotor_into(multi_vector_scalar_geometric_anti_product(scalar_rotor_geometric_anti_product(self, other), scalar_anti_reversal(self)));
}

Flector scalar_flector_sandwich(Scalar self, Flector other) {
    return flector_scalar_geometric_anti_product(scalar_flector_geometric_anti_product(self, other), scalar_anti_reversal(self));
}

MultiVector scalar_multi_vector_sandwich(Scalar self, MultiVector other) {
    return multi_vector_scalar_geometric_anti_product(scalar_multi_vector_geometric_anti_product(self, other), scalar_anti_reversal(self));
}

Scalar anti_scalar_scalar_sandwich(AntiScalar self, Scalar other) {
    return scalar_anti_scalar_geometric_anti_product(anti_scalar_scalar_geometric_anti_product(self, other), anti_scalar_anti_reversal(self));
}

AntiScalar anti_scalar_anti_scalar_sandwich(AntiScalar self, AntiScalar other) {
    return anti_scalar_anti_scalar_geometric_anti_product(anti_scalar_anti_scalar_geometric_anti_product(self, other), anti_scalar_anti_reversal(self));
}

HomogeneousMagnitude anti_scalar_homogeneous_magnitude_sandwich(AntiScalar self, HomogeneousMagnitude other) {
    return homogeneous_magnitude_anti_scalar_geometric_anti_product(anti_scalar_homogeneous_magnitude_geometric_anti_product(self, other), anti_scalar_anti_reversal(self));
}

Point anti_scalar_point_sandwich(AntiScalar self, Point other) {
    return point_anti_scalar_geometric_anti_product(anti_scalar_point_geometric_anti_product(self, other), anti_scalar_anti_reversal(self));
}

Line anti_scalar_line_sandwich(AntiScalar self, Line other) {
    return line_anti_scalar_geometric_anti_product(anti_scalar_line_geometric_anti_product(self, other), anti_scalar_anti_reversal(self));
}

Plane anti_scalar_plane_sandwich(AntiScalar self, Plane other) {
    return plane_anti_scalar_geometric_anti_product(anti_scalar_plane_geometric_anti_product(self, other), anti_scalar_anti_reversal(self));
}

Motor anti_scalar_motor_sandwich(AntiScalar self, Motor other) {
    return motor_anti_scalar_geometric_anti_product(anti_scalar_motor_geometric_anti_product(self, other), anti_scalar_anti_reversal(self));
}

Rotor anti_scalar_rotor_sandwich(AntiScalar self, Rotor other) {
    return rotor_anti_scalar_geometric_anti_product(anti_scalar_rotor_geometric_anti_product(self, other), anti_scalar_anti_reversal(self));
}

Translator anti_scalar_translator_sandwich(AntiScalar self, Translator other) {
    return translator_anti_scalar_geometric_anti_product(anti_scalar_translator_geometric_anti_product(self, other), anti_scalar_anti_reversal(self));
}

Flector anti_scalar_flector_sandwich(AntiScalar self, Flector other) {
    return flector_anti_scalar_geometric_anti_product(anti_scalar_flector_geometric_anti_product(self, other), anti_scalar_anti_reversal(self));
}

MultiVector anti_scalar_multi_vector_sandwich(AntiScalar self, MultiVector other) {
    return multi_vector_anti_scalar_geometric_anti_product(anti_scalar_multi_vector_geometric_anti_product(self, other), anti_scalar_anti_reversal(self));
}

Scalar homogeneous_magnitude_scalar_sandwich(HomogeneousMagnitude self, Scalar other) {
    return scalar_homogeneous_magnitude_geometric_anti_product(homogeneous_magnitude_scalar_geometric_anti_product(self, other), homogeneous_magnitude_anti_reversal(self));
}

AntiScalar homogeneous_magnitude_anti_scalar_sandwich(HomogeneousMagnitude self, AntiScalar other) {
    return homogeneous_magnitude_anti_scalar_into(homogeneous_magnitude_homogeneous_magnitude_geometric_anti_product(homogeneous_magnitude_anti_scalar_geometric_anti_product(self, other), homogeneous_magnitude_anti_reversal(self)));
}

HomogeneousMagnitude homogeneous_magnitude_homogeneous_magnitude_sandwich(HomogeneousMagnitude self, HomogeneousMagnitude other) {
    return homogeneous_magnitude_homogeneous_magnitude_geometric_anti_product(homogeneous_magnitude_homogeneous_magnitude_geometric_anti_product(self, other), homogeneous_magnitude_anti_reversal(self));
}

Point homogeneous_magnitude_point_sandwich(HomogeneousMagnitude self, Point other) {
    return flector_point_into(flector_homogeneous_magnitude_geometric_anti_product(homogeneous_magnitude_point_geometric_anti_product(self, other), homogeneous_magnitude_anti_reversal(self)));
}

Line homogeneous_magnitude_line_sandwich(HomogeneousMagnitude self, Line other) {
    return line_homogeneous_magnitude_geometric_anti_product(homogeneous_magnitude_line_geometric_anti_product(self, other), homogeneous_magnitude_anti_reversal(self));
}

Plane homogeneous_magnitude_plane_sandwich(HomogeneousMagnitude self, Plane other) {
    return flector_plane_into(flector_homogeneous_magnitude_geometric_anti_product(homogeneous_magnitude_plane_geometric_anti_product(self, other), homogeneous_magnitude_anti_reversal(self)));
}

Motor homogeneous_magnitude_motor_sandwich(HomogeneousMagnitude self, Motor other) {
    return multi_vector_motor_into(multi_vector_homogeneous_magnitude_geometric_anti_product(homogeneous_magnitude_motor_geometric_anti_product(self, other), homogeneous_magnitude_anti_reversal(self)));
}

Rotor homogeneous_magnitude_rotor_sandwich(HomogeneousMagnitude self, Rotor other) {
    return multi_vector_rotor_into(multi_vector_homogeneous_magnitude_geometric_anti_product(homogeneous_magnitude_rotor_geometric_anti_product(self, other), homogeneous_magnitude_anti_reversal(self)));
}

Translator homogeneous_magnitude_translator_sandwich(HomogeneousMagnitude self, Translator other) {
    return multi_vector_translator_into(multi_vector_homogeneous_magnitude_geometric_anti_product(homogeneous_magnitude_translator_geometric_anti_product(self, other), homogeneous_magnitude_anti_reversal(self)));
}

Flector homogeneous_magnitude_flector_sandwich(HomogeneousMagnitude self, Flector other) {
    return flector_homogeneous_magnitude_geometric_anti_product(homogeneous_magnitude_flector_geometric_anti_product(self, other), homogeneous_magnitude_anti_reversal(self));
}

MultiVector homogeneous_magnitude_multi_vector_sandwich(HomogeneousMagnitude self, MultiVector other) {
    return multi_vector_homogeneous_magnitude_geometric_anti_product(homogeneous_magnitude_multi_vector_geometric_anti_product(self, other), homogeneous_magnitude_anti_reversal(self));
}

Scalar point_scalar_sandwich(Point self, Scalar other) {
    return multi_vector_scalar_into(plane_point_geometric_anti_product(point_scalar_geometric_anti_product(self, other), point_anti_reversal(self)));
}

AntiScalar point_anti_scalar_sandwich(Point self, AntiScalar other) {
    return translator_anti_scalar_into(point_point_geometric_anti_product(point_anti_scalar_geometric_anti_product(self, other), point_anti_reversal(self)));
}

HomogeneousMagnitude point_homogeneous_magnitude_sandwich(Point self, HomogeneousMagnitude other) {
    return multi_vector_homogeneous_magnitude_into(flector_point_geometric_anti_product(point_homogeneous_magnitude_geometric_anti_product(self, other), point_anti_reversal(self)));
}

Point point_point_sandwich(Point self, Point other) {
    return translator_point_geometric_anti_product(point_point_geometric_anti_product(self, other), point_anti_reversal(self));
}

Line point_line_sandwich(Point self, Line other) {
    return multi_vector_line_into(flector_point_geometric_anti_product(point_line_geometric_anti_product(self, other), point_anti_reversal(self)));
}

Plane point_plane_sandwich(Point self, Plane other) {
    return multi_vector_plane_into(multi_vector_point_geometric_anti_product(point_plane_geometric_anti_product(self, other), point_anti_reversal(self)));
}

Motor point_motor_sandwich(Point self, Motor other) {
    return multi_vector_motor_into(flector_point_geometric_anti_product(point_motor_geometric_anti_product(self, other), point_anti_reversal(self)));
}

Rotor point_rotor_sandwich(Point self, Rotor other) {
    return multi_vector_rotor_into(flector_point_geometric_anti_product(point_rotor_geometric_anti_product(self, other), point_anti_reversal(self)));
}

Translator point_translator_sandwich(Point self, Translator other) {
    return point_point_geometric_anti_product(point_translator_geometric_anti_product(self, other), point_anti_reversal(self));
}

Flector point_flector_sandwich(Point self, Flector other) {
    return multi_vector_flector_into(multi_vector_point_geometric_anti_product(point_flector_geometric_anti_product(self, other), point_anti_reversal(self)));
}

MultiVector point_multi_vector_sandwich(Point self, MultiVector other) {
    return multi_vector_point_geometric_anti_product(point_multi_vector_geometric_anti_product(self, other), point_anti_reversal(self));
}

Scalar line_scalar_sandwich(Line self, Scalar other) {
    return multi_vector_scalar_into(translator_line_geometric_anti_product(line_scalar_geometric_anti_product(self, other), line_anti_reversal(self)));
}

AntiScalar line_anti_scalar_sandwich(Line self, AntiScalar other) {
    return multi_vector_anti_scalar_into(line_line_geometric_anti_product(line_anti_scalar_geometric_anti_product(self, other), line_anti_reversal(self)));
}

HomogeneousMagnitude line_homogeneous_magnitude_sandwich(Line self, HomogeneousMagnitude other) {
    return multi_vector_homogeneous_magnitude_into(line_line_geometric_anti_product(line_homogeneous_magnitude_geometric_anti_product(self, other), line_anti_reversal(self)));
}

Point line_point_sandwich(Line self, Point other) {
    return flector_point_into(flector_line_geometric_anti_product(line_point_geometric_anti_product(self, other), line_anti_reversal(self)));
}

Line line_line_sandwich(Line self, Line other) {
    return multi_vector_line_into(multi_vector_line_geometric_anti_product(line_line_geometric_anti_product(self, other), line_anti_reversal(self)));
}

Plane line_plane_sandwich(Line self, Plane other) {
    return flector_plane_into(flector_line_geometric_anti_product(line_plane_geometric_anti_product(self, other), line_anti_reversal(self)));
}

Motor line_motor_sandwich(Line self, Motor other) {
    return multi_vector_motor_into(multi_vector_line_geometric_anti_product(line_motor_geometric_anti_product(self, other), line_anti_reversal(self)));
}

Rotor line_rotor_sandwich(Line self, Rotor other) {
    return multi_vector_rotor_into(multi_vector_line_geometric_anti_product(line_rotor_geometric_anti_product(self, other), line_anti_reversal(self)));
}

Translator line_translator_sandwich(Line self, Translator other) {
    return multi_vector_translator_into(multi_vector_line_geometric_anti_product(line_translator_geometric_anti_product(self, other), line_anti_reversal(self)));
}

Flector line_flector_sandwich(Line self, Flector other) {
    return flector_line_geometric_anti_product(line_flector_geometric_anti_product(self, other), line_anti_reversal(self));
}

MultiVector line_multi_vector_sandwich(Line self, MultiVector other) {
    return multi_vector_line_geometric_anti_product(line_multi_vector_geometric_anti_product(self, other), line_anti_reversal(self));
}

Scalar plane_scalar_sandwich(Plane self, Scalar other) {
    return multi_vector_scalar_into(point_plane_geometric_anti_product(plane_scalar_geometric_anti_product(self, other), plane_anti_reversal(self)));
}

AntiScalar plane_anti_scalar_sandwich(Plane self, AntiScalar other) {
    return motor_anti_scalar_into(plane_plane_geometric_anti_product(plane_anti_scalar_geometric_anti_product(self, other), plane_anti_reversal(self)));
}

HomogeneousMagnitude plane_homogeneous_magnitude_sandwich(Plane self, HomogeneousMagnitude other) {
    return multi_vector_homogeneous_magnitude_into(flector_plane_geometric_anti_product(plane_homogeneous_magnitude_geometric_anti_product(self, other), plane_anti_reversal(self)));
}

Point plane_point_sandwich(Plane self, Point other) {
    return multi_vector_point_into(multi_vector_plane_geometric_anti_product(plane_point_geometric_anti_product(self, other), plane_anti_reversal(self)));
}

Line plane_line_sandwich(Plane self, Line other) {
    return multi_vector_line_into(flector_plane_geometric_anti_product(plane_line_geometric_anti_product(self, other), plane_anti_reversal(self)));
}

Plane plane_plane_sandwich(Plane self, Plane other) {
    return flector_plane_into(motor_plane_geometric_anti_product(plane_plane_geometric_anti_product(self, other), plane_anti_reversal(self)));
}

Motor plane_motor_sandwich(Plane self, Motor other) {
    return multi_vector_motor_into(flector_plane_geometric_anti_product(plane_motor_geometric_anti_product(self, other), plane_anti_reversal(self)));
}

Rotor plane_rotor_sandwich(Plane self, Rotor other) {
    return multi_vector_rotor_into(flector_plane_geometric_anti_product(plane_rotor_geometric_anti_product(self, other), plane_anti_reversal(self)));
}

Translator plane_translator_sandwich(Plane self, Translator other) {
    return multi_vector_translator_into(flector_plane_geometric_anti_product(plane_translator_geometric_anti_product(self, other), plane_anti_reversal(self)));
}

Flector plane_flector_sandwich(Plane self, Flector other) {
    return multi_vector_flector_into(multi_vector_plane_geometric_anti_product(plane_flector_geometric_anti_product(self, other), plane_anti_reversal(self)));
}

MultiVector plane_multi_vector_sandwich(Plane self, MultiVector other) {
    return multi_vector_plane_geometric_anti_product(plane_multi_vector_geometric_anti_product(self, other), plane_anti_reversal(self));
}

Scalar motor_scalar_sandwich(Motor self, Scalar other) {
    return multi_vector_scalar_into(multi_vector_motor_geometric_anti_product(motor_scalar_geometric_anti_product(self, other), motor_anti_reversal(self)));
}

AntiScalar motor_anti_scalar_sandwich(Motor self, AntiScalar other) {
    return multi_vector_anti_scalar_into(motor_motor_geometric_anti_product(motor_anti_scalar_geometric_anti_product(self, other), motor_anti_reversal(self)));
}

HomogeneousMagnitude motor_homogeneous_magnitude_sandwich(Motor self, HomogeneousMagnitude other) {
    return multi_vector_homogeneous_magnitude_into(multi_vector_motor_geometric_anti_product(motor_homogeneous_magnitude_geometric_anti_product(self, other), motor_anti_reversal(self)));
}

Point motor_point_sandwich(Motor self, Point other) {
    return flector_point_into(flector_motor_geometric_anti_product(motor_point_geometric_anti_product(self, other), motor_anti_reversal(self)));
}

Line motor_line_sandwich(Motor self, Line other) {
    return multi_vector_line_into(multi_vector_motor_geometric_anti_product(motor_line_geometric_anti_product(self, other), motor_anti_reversal(self)));
}

Plane motor_plane_sandwich(Motor self, Plane other) {
    return flector_plane_into(flector_motor_geometric_anti_product(motor_plane_geometric_anti_product(self, other), motor_anti_reversal(self)));
}

Motor motor_motor_sandwich(Motor self, Motor other) {
    return multi_vector_motor_into(multi_vector_motor_geometric_anti_product(motor_motor_geometric_anti_product(self, other), motor_anti_reversal(self)));
}

Rotor motor_rotor_sandwich(Motor self, Rotor other) {
    return multi_vector_rotor_into(multi_vector_motor_geometric_anti_product(motor_rotor_geometric_anti_product(self, other), motor_anti_reversal(self)));
}

Translator motor_translator_sandwich(Motor self, Translator other) {
    return multi_vector_translator_into(multi_vector_motor_geometric_anti_product(motor_translator_geometric_anti_product(self, other), motor_anti_reversal(self)));
}

Flector motor_flector_sandwich(Motor self, Flector other) {
    return flector_motor_geometric_anti_product(motor_flector_geometric_anti_product(self, other), motor_anti_reversal(self));
}

MultiVector motor_multi_vector_sandwich(Motor self, MultiVector other) {
    return multi_vector_motor_geometric_anti_product(motor_multi_vector_geometric_anti_product(self, other), motor_anti_reversal(self));
}

Scalar rotor_scalar_sandwich(Rotor self, Scalar other) {
    return multi_vector_scalar_into(multi_vector_rotor_geometric_anti_product(rotor_scalar_geometric_anti_product(self, other), rotor_anti_reversal(self)));
}

AntiScalar rotor_anti_scalar_sandwich(Rotor self, AntiScalar other) {
    return rotor_anti_scalar_into(rotor_rotor_geometric_anti_product(rotor_anti_scalar_geometric_anti_product(self, other), rotor_anti_reversal(self)));
}

HomogeneousMagnitude rotor_homogeneous_magnitude_sandwich(Rotor self, HomogeneousMagnitude other) {
    return multi_vector_homogeneous_magnitude_into(multi_vector_rotor_geometric_anti_product(rotor_homogeneous_magnitude_geometric_anti_product(self, other), rotor_anti_reversal(self)));
}

Point rotor_point_sandwich(Rotor self, Point other) {
    return flector_point_into(flector_rotor_geometric_anti_product(rotor_point_geometric_anti_product(self, other), rotor_anti_reversal(self)));
}

Line rotor_line_sandwich(Rotor self, Line other) {
    return multi_vector_line_into(multi_vector_rotor_geometric_anti_product(rotor_line_geometric_anti_product(self, other), rotor_anti_reversal(self)));
}

Plane rotor_plane_sandwich(Rotor self, Plane other) {
    return flector_plane_into(flector_rotor_geometric_anti_product(rotor_plane_geometric_anti_product(self, other), rotor_anti_reversal(self)));
}

Motor rotor_motor_sandwich(Rotor self, Motor other) {
    return multi_vector_motor_into(multi_vector_rotor_geometric_anti_product(rotor_motor_geometric_anti_product(self, other), rotor_anti_reversal(self)));
}

Rotor rotor_rotor_sandwich(Rotor self, Rotor other) {
    return rotor_rotor_geometric_anti_product(rotor_rotor_geometric_anti_product(self, other), rotor_anti_reversal(self));
}

Translator rotor_translator_sandwich(Rotor self, Translator other) {
    return multi_vector_translator_into(multi_vector_rotor_geometric_anti_product(rotor_translator_geometric_anti_product(self, other), rotor_anti_reversal(self)));
}

Flector rotor_flector_sandwich(Rotor self, Flector other) {
    return flector_rotor_geometric_anti_product(rotor_flector_geometric_anti_product(self, other), rotor_anti_reversal(self));
}

MultiVector rotor_multi_vector_sandwich(Rotor self, MultiVector other) {
    return multi_vector_rotor_geometric_anti_product(rotor_multi_vector_geometric_anti_product(self, other), rotor_anti_reversal(self));
}

Scalar translator_scalar_sandwich(Translator self, Scalar other) {
    return scalar_translator_geometric_anti_product(translator_scalar_geometric_anti_product(self, other), translator_anti_reversal(self));
}

AntiScalar translator_anti_scalar_sandwich(Translator self, AntiScalar other) {
    return translator_anti_scalar_into(translator_translator_geometric_anti_product(translator_anti_scalar_geometric_anti_product(self, other), translator_anti_reversal(self)));
}

HomogeneousMagnitude translator_homogeneous_magnitude_sandwich(Translator self, HomogeneousMagnitude other) {
    return multi_vector_homogeneous_magnitude_into(multi_vector_translator_geometric_anti_product(translator_homogeneous_magnitude_geometric_anti_product(self, other), translator_anti_reversal(self)));
}

Point translator_point_sandwich(Translator self, Point other) {
    return point_translator_geometric_anti_product(translator_point_geometric_anti_product(self, other), translator_anti_reversal(self));
}

Line translator_line_sandwich(Translator self, Line other) {
    return multi_vector_line_into(multi_vector_translator_geometric_anti_product(translator_line_geometric_anti_product(self, other), translator_anti_reversal(self)));
}

Plane translator_plane_sandwich(Translator self, Plane other) {
    return flector_plane_into(flector_translator_geometric_anti_product(translator_plane_geometric_anti_product(self, other), translator_anti_reversal(self)));
}

Motor translator_motor_sandwich(Translator self, Motor other) {
    return multi_vector_motor_into(multi_vector_translator_geometric_anti_product(translator_motor_geometric_anti_product(self, other), translator_anti_reversal(self)));
}

Rotor translator_rotor_sandwich(Translator self, Rotor other) {
    return multi_vector_rotor_into(multi_vector_translator_geometric_anti_product(translator_rotor_geometric_anti_product(self, other), translator_anti_reversal(self)));
}

Translator translator_translator_sandwich(Translator self, Translator other) {
    return translator_translator_geometric_anti_product(translator_translator_geometric_anti_product(self, other), translator_anti_reversal(self));
}

Flector translator_flector_sandwich(Translator self, Flector other) {
    return flector_translator_geometric_anti_product(translator_flector_geometric_anti_product(self, other), translator_anti_reversal(self));
}

MultiVector translator_multi_vector_sandwich(Translator self, MultiVector other) {
    return multi_vector_translator_geometric_anti_product(translator_multi_vector_geometric_anti_product(self, other), translator_anti_reversal(self));
}

Scalar flector_scalar_sandwich(Flector self, Scalar other) {
    return multi_vector_scalar_into(flector_flector_geometric_anti_product(flector_scalar_geometric_anti_product(self, other), flector_anti_reversal(self)));
}

AntiScalar flector_anti_scalar_sandwich(Flector self, AntiScalar other) {
    return multi_vector_anti_scalar_into(flector_flector_geometric_anti_product(flector_anti_scalar_geometric_anti_product(self, other), flector_anti_reversal(self)));
}

HomogeneousMagnitude flector_homogeneous_magnitude_sandwich(Flector self, HomogeneousMagnitude other) {
    return multi_vector_homogeneous_magnitude_into(flector_flector_geometric_anti_product(flector_homogeneous_magnitude_geometric_anti_product(self, other), flector_anti_reversal(self)));
}

Point flector_point_sandwich(Flector self, Point other) {
    return multi_vector_point_into(multi_vector_flector_geometric_anti_product(flector_point_geometric_anti_product(self, other), flector_anti_reversal(self)));
}

Line flector_line_sandwich(Flector self, Line other) {
    return multi_vector_line_into(flector_flector_geometric_anti_product(flector_line_geometric_anti_product(self, other), flector_anti_reversal(self)));
}

Plane flector_plane_sandwich(Flector self, Plane other) {
    return multi_vector_plane_into(multi_vector_flector_geometric_anti_product(flector_plane_geometric_anti_product(self, other), flector_anti_reversal(self)));
}

Motor flector_motor_sandwich(Flector self, Motor other) {
    return multi_vector_motor_into(flector_flector_geometric_anti_product(flector_motor_geometric_anti_product(self, other), flector_anti_reversal(self)));
}

Rotor flector_rotor_sandwich(Flector self, Rotor other) {
    return multi_vector_rotor_into(flector_flector_geometric_anti_product(flector_rotor_geometric_anti_product(self, other), flector_anti_reversal(self)));
}

Translator flector_translator_sandwich(Flector self, Translator other) {
    return multi_vector_translator_into(flector_flector_geometric_anti_product(flector_translator_geometric_anti_product(self, other), flector_anti_reversal(self)));
}

Flector flector_flector_sandwich(Flector self, Flector other) {
    return multi_vector_flector_into(multi_vector_flector_geometric_anti_product(flector_flector_geometric_anti_product(self, other), flector_anti_reversal(self)));
}

MultiVector flector_multi_vector_sandwich(Flector self, MultiVector other) {
    return multi_vector_flector_geometric_anti_product(flector_multi_vector_geometric_anti_product(self, other), flector_anti_reversal(self));
}

Scalar multi_vector_scalar_sandwich(MultiVector self, Scalar other) {
    return multi_vector_scalar_into(multi_vector_multi_vector_geometric_anti_product(multi_vector_scalar_geometric_anti_product(self, other), multi_vector_anti_reversal(self)));
}

AntiScalar multi_vector_anti_scalar_sandwich(MultiVector self, AntiScalar other) {
    return multi_vector_anti_scalar_into(multi_vector_multi_vector_geometric_anti_product(multi_vector_anti_scalar_geometric_anti_product(self, other), multi_vector_anti_reversal(self)));
}

HomogeneousMagnitude multi_vector_homogeneous_magnitude_sandwich(MultiVector self, HomogeneousMagnitude other) {
    return multi_vector_homogeneous_magnitude_into(multi_vector_multi_vector_geometric_anti_product(multi_vector_homogeneous_magnitude_geometric_anti_product(self, other), multi_vector_anti_reversal(self)));
}

Point multi_vector_point_sandwich(MultiVector self, Point other) {
    return multi_vector_point_into(multi_vector_multi_vector_geometric_anti_product(multi_vector_point_geometric_anti_product(self, other), multi_vector_anti_reversal(self)));
}

Line multi_vector_line_sandwich(MultiVector self, Line other) {
    return multi_vector_line_into(multi_vector_multi_vector_geometric_anti_product(multi_vector_line_geometric_anti_product(self, other), multi_vector_anti_reversal(self)));
}

Plane multi_vector_plane_sandwich(MultiVector self, Plane other) {
    return multi_vector_plane_into(multi_vector_multi_vector_geometric_anti_product(multi_vector_plane_geometric_anti_product(self, other), multi_vector_anti_reversal(self)));
}

Motor multi_vector_motor_sandwich(MultiVector self, Motor other) {
    return multi_vector_motor_into(multi_vector_multi_vector_geometric_anti_product(multi_vector_motor_geometric_anti_product(self, other), multi_vector_anti_reversal(self)));
}

Rotor multi_vector_rotor_sandwich(MultiVector self, Rotor other) {
    return multi_vector_rotor_into(multi_vector_multi_vector_geometric_anti_product(multi_vector_rotor_geometric_anti_product(self, other), multi_vector_anti_reversal(self)));
}

Translator multi_vector_translator_sandwich(MultiVector self, Translator other) {
    return multi_vector_translator_into(multi_vector_multi_vector_geometric_anti_product(multi_vector_translator_geometric_anti_product(self, other), multi_vector_anti_reversal(self)));
}

Flector multi_vector_flector_sandwich(MultiVector self, Flector other) {
    return multi_vector_flector_into(multi_vector_multi_vector_geometric_anti_product(multi_vector_flector_geometric_anti_product(self, other), multi_vector_anti_reversal(self)));
}

MultiVector multi_vector_multi_vector_sandwich(MultiVector self, MultiVector other) {
    return multi_vector_multi_vector_geometric_anti_product(multi_vector_multi_vector_geometric_anti_product(self, other), multi_vector_anti_reversal(self));
}

HomogeneousMagnitude scalar_anti_scalar_distance(Scalar self, AntiScalar other) {
    return scalar_anti_scalar_add(plane_bulk_norm(anti_scalar_attitude(scalar_anti_scalar_outer_product(self, other))), plane_weight_norm(scalar_plane_outer_product(self, anti_scalar_attitude(other))));
}

HomogeneousMagnitude scalar_homogeneous_magnitude_distance(Scalar self, HomogeneousMagnitude other) {
    return scalar_anti_scalar_add(plane_bulk_norm(homogeneous_magnitude_attitude(scalar_homogeneous_magnitude_outer_product(self, other))), plane_weight_norm(scalar_plane_outer_product(self, homogeneous_magnitude_attitude(other))));
}

HomogeneousMagnitude scalar_line_distance(Scalar self, Line other) {
    return scalar_anti_scalar_add(point_bulk_norm(line_attitude(scalar_line_outer_product(self, other))), point_weight_norm(scalar_point_outer_product(self, line_attitude(other))));
}

HomogeneousMagnitude scalar_plane_distance(Scalar self, Plane other) {
    return scalar_anti_scalar_add(line_bulk_norm(plane_attitude(scalar_plane_outer_product(self, other))), line_weight_norm(scalar_line_outer_product(self, plane_attitude(other))));
}

HomogeneousMagnitude scalar_motor_distance(Scalar self, Motor other) {
    return scalar_anti_scalar_add(flector_bulk_norm(motor_attitude(scalar_motor_outer_product(self, other))), flector_weight_norm(scalar_flector_outer_product(self, motor_attitude(other))));
}

HomogeneousMagnitude scalar_rotor_distance(Scalar self, Rotor other) {
    return scalar_anti_scalar_add(flector_bulk_norm(rotor_attitude(scalar_rotor_outer_product(self, other))), flector_weight_norm(scalar_flector_outer_product(self, rotor_attitude(other))));
}

HomogeneousMagnitude scalar_translator_distance(Scalar self, Translator other) {
    return scalar_anti_scalar_add(flector_bulk_norm(translator_attitude(scalar_translator_outer_product(self, other))), flector_weight_norm(scalar_flector_outer_product(self, translator_attitude(other))));
}

HomogeneousMagnitude scalar_flector_distance(Scalar self, Flector other) {
    return scalar_anti_scalar_add(multi_vector_bulk_norm(flector_attitude(scalar_flector_outer_product(self, other))), multi_vector_weight_norm(scalar_multi_vector_outer_product(self, flector_attitude(other))));
}

HomogeneousMagnitude scalar_multi_vector_distance(Scalar self, MultiVector other) {
    return scalar_anti_scalar_add(multi_vector_bulk_norm(multi_vector_attitude(scalar_multi_vector_outer_product(self, other))), multi_vector_weight_norm(scalar_multi_vector_outer_product(self, multi_vector_attitude(other))));
}

HomogeneousMagnitude homogeneous_magnitude_anti_scalar_distance(HomogeneousMagnitude self, AntiScalar other) {
    return scalar_anti_scalar_add(plane_bulk_norm(anti_scalar_attitude(homogeneous_magnitude_anti_scalar_outer_product(self, other))), plane_weight_norm(homogeneous_magnitude_plane_outer_product(self, anti_scalar_attitude(other))));
}

HomogeneousMagnitude homogeneous_magnitude_homogeneous_magnitude_distance(HomogeneousMagnitude self, HomogeneousMagnitude other) {
    return scalar_anti_scalar_add(plane_bulk_norm(homogeneous_magnitude_attitude(homogeneous_magnitude_homogeneous_magnitude_outer_product(self, other))), plane_weight_norm(homogeneous_magnitude_plane_outer_product(self, homogeneous_magnitude_attitude(other))));
}

HomogeneousMagnitude homogeneous_magnitude_point_distance(HomogeneousMagnitude self, Point other) {
    return scalar_anti_scalar_add(scalar_bulk_norm(point_attitude(homogeneous_magnitude_point_outer_product(self, other))), homogeneous_magnitude_weight_norm(homogeneous_magnitude_scalar_outer_product(self, point_attitude(other))));
}

HomogeneousMagnitude homogeneous_magnitude_line_distance(HomogeneousMagnitude self, Line other) {
    return scalar_anti_scalar_add(point_bulk_norm(line_attitude(homogeneous_magnitude_line_outer_product(self, other))), point_weight_norm(homogeneous_magnitude_point_outer_product(self, line_attitude(other))));
}

HomogeneousMagnitude homogeneous_magnitude_plane_distance(HomogeneousMagnitude self, Plane other) {
    return scalar_anti_scalar_add(line_bulk_norm(plane_attitude(homogeneous_magnitude_plane_outer_product(self, other))), line_weight_norm(homogeneous_magnitude_line_outer_product(self, plane_attitude(other))));
}

HomogeneousMagnitude homogeneous_magnitude_motor_distance(HomogeneousMagnitude self, Motor other) {
    return scalar_anti_scalar_add(flector_bulk_norm(motor_attitude(homogeneous_magnitude_motor_outer_product(self, other))), flector_weight_norm(homogeneous_magnitude_flector_outer_product(self, motor_attitude(other))));
}

HomogeneousMagnitude homogeneous_magnitude_rotor_distance(HomogeneousMagnitude self, Rotor other) {
    return scalar_anti_scalar_add(flector_bulk_norm(rotor_attitude(homogeneous_magnitude_rotor_outer_product(self, other))), flector_weight_norm(homogeneous_magnitude_flector_outer_product(self, rotor_attitude(other))));
}

HomogeneousMagnitude homogeneous_magnitude_translator_distance(HomogeneousMagnitude self, Translator other) {
    return scalar_anti_scalar_add(flector_bulk_norm(translator_attitude(homogeneous_magnitude_translator_outer_product(self, other))), flector_weight_norm(homogeneous_magnitude_flector_outer_product(self, translator_attitude(other))));
}

HomogeneousMagnitude homogeneous_magnitude_flector_distance(HomogeneousMagnitude self, Flector other) {
    return scalar_anti_scalar_add(multi_vector_bulk_norm(flector_attitude(homogeneous_magnitude_flector_outer_product(self, other))), multi_vector_weight_norm(homogeneous_magnitude_multi_vector_outer_product(self, flector_attitude(other))));
}

HomogeneousMagnitude homogeneous_magnitude_multi_vector_distance(HomogeneousMagnitude self, MultiVector other) {
    return scalar_anti_scalar_add(multi_vector_bulk_norm(multi_vector_attitude(homogeneous_magnitude_multi_vector_outer_product(self, other))), multi_vector_weight_norm(homogeneous_magnitude_multi_vector_outer_product(self, multi_vector_attitude(other))));
}

HomogeneousMagnitude point_point_distance(Point self, Point other) {
    return scalar_anti_scalar_add(point_bulk_norm(line_attitude(point_point_outer_product(self, other))), point_weight_norm(point_scalar_outer_product(self, point_attitude(other))));
}

HomogeneousMagnitude point_line_distance(Point self, Line other) {
    return scalar_anti_scalar_add(line_bulk_norm(plane_attitude(point_line_outer_product(self, other))), line_weight_norm(point_point_outer_product(self, line_attitude(other))));
}

HomogeneousMagnitude point_plane_distance(Point self, Plane other) {
    return scalar_anti_scalar_add(plane_bulk_norm(anti_scalar_attitude(point_plane_outer_product(self, other))), plane_weight_norm(point_line_outer_product(self, plane_attitude(other))));
}

HomogeneousMagnitude point_motor_distance(Point self, Motor other) {
    return scalar_anti_scalar_add(line_bulk_norm(plane_attitude(point_motor_outer_product(self, other))), motor_weight_norm(point_flector_outer_product(self, motor_attitude(other))));
}

HomogeneousMagnitude point_rotor_distance(Point self, Rotor other) {
    return scalar_anti_scalar_add(line_bulk_norm(plane_attitude(point_rotor_outer_product(self, other))), motor_weight_norm(point_flector_outer_product(self, rotor_attitude(other))));
}

HomogeneousMagnitude point_translator_distance(Point self, Translator other) {
    return scalar_anti_scalar_add(line_bulk_norm(plane_attitude(point_translator_outer_product(self, other))), motor_weight_norm(point_flector_outer_product(self, translator_attitude(other))));
}

HomogeneousMagnitude point_flector_distance(Point self, Flector other) {
    return scalar_anti_scalar_add(flector_bulk_norm(motor_attitude(point_flector_outer_product(self, other))), multi_vector_weight_norm(point_multi_vector_outer_product(self, flector_attitude(other))));
}

HomogeneousMagnitude point_multi_vector_distance(Point self, MultiVector other) {
    return scalar_anti_scalar_add(multi_vector_bulk_norm(multi_vector_attitude(point_multi_vector_outer_product(self, other))), multi_vector_weight_norm(point_multi_vector_outer_product(self, multi_vector_attitude(other))));
}

HomogeneousMagnitude line_point_distance(Line self, Point other) {
    return scalar_anti_scalar_add(line_bulk_norm(plane_attitude(line_point_outer_product(self, other))), line_weight_norm(line_scalar_outer_product(self, point_attitude(other))));
}

HomogeneousMagnitude line_line_distance(Line self, Line other) {
    return scalar_anti_scalar_add(plane_bulk_norm(anti_scalar_attitude(line_line_outer_product(self, other))), plane_weight_norm(line_point_outer_product(self, line_attitude(other))));
}

HomogeneousMagnitude line_motor_distance(Line self, Motor other) {
    return scalar_anti_scalar_add(plane_bulk_norm(anti_scalar_attitude(line_motor_outer_product(self, other))), plane_weight_norm(line_flector_outer_product(self, motor_attitude(other))));
}

HomogeneousMagnitude line_rotor_distance(Line self, Rotor other) {
    return scalar_anti_scalar_add(plane_bulk_norm(anti_scalar_attitude(line_rotor_outer_product(self, other))), plane_weight_norm(line_flector_outer_product(self, rotor_attitude(other))));
}

HomogeneousMagnitude line_translator_distance(Line self, Translator other) {
    return scalar_anti_scalar_add(plane_bulk_norm(anti_scalar_attitude(line_translator_outer_product(self, other))), plane_weight_norm(line_flector_outer_product(self, translator_attitude(other))));
}

HomogeneousMagnitude line_flector_distance(Line self, Flector other) {
    return scalar_anti_scalar_add(line_bulk_norm(plane_attitude(line_flector_outer_product(self, other))), multi_vector_weight_norm(line_multi_vector_outer_product(self, flector_attitude(other))));
}

HomogeneousMagnitude line_multi_vector_distance(Line self, MultiVector other) {
    return scalar_anti_scalar_add(multi_vector_bulk_norm(multi_vector_attitude(line_multi_vector_outer_product(self, other))), multi_vector_weight_norm(line_multi_vector_outer_product(self, multi_vector_attitude(other))));
}

HomogeneousMagnitude plane_point_distance(Plane self, Point other) {
    return scalar_anti_scalar_add(plane_bulk_norm(anti_scalar_attitude(plane_point_outer_product(self, other))), plane_weight_norm(plane_scalar_outer_product(self, point_attitude(other))));
}

HomogeneousMagnitude plane_flector_distance(Plane self, Flector other) {
    return scalar_anti_scalar_add(plane_bulk_norm(anti_scalar_attitude(plane_flector_outer_product(self, other))), multi_vector_weight_norm(plane_multi_vector_outer_product(self, flector_attitude(other))));
}

HomogeneousMagnitude plane_multi_vector_distance(Plane self, MultiVector other) {
    return scalar_anti_scalar_add(multi_vector_bulk_norm(multi_vector_attitude(plane_multi_vector_outer_product(self, other))), multi_vector_weight_norm(plane_multi_vector_outer_product(self, multi_vector_attitude(other))));
}

HomogeneousMagnitude motor_point_distance(Motor self, Point other) {
    return scalar_anti_scalar_add(line_bulk_norm(plane_attitude(motor_point_outer_product(self, other))), motor_weight_norm(motor_scalar_outer_product(self, point_attitude(other))));
}

HomogeneousMagnitude motor_line_distance(Motor self, Line other) {
    return scalar_anti_scalar_add(plane_bulk_norm(anti_scalar_attitude(motor_line_outer_product(self, other))), plane_weight_norm(motor_point_outer_product(self, line_attitude(other))));
}

HomogeneousMagnitude motor_motor_distance(Motor self, Motor other) {
    return scalar_anti_scalar_add(plane_bulk_norm(anti_scalar_attitude(motor_motor_outer_product(self, other))), plane_weight_norm(motor_flector_outer_product(self, motor_attitude(other))));
}

HomogeneousMagnitude motor_rotor_distance(Motor self, Rotor other) {
    return scalar_anti_scalar_add(plane_bulk_norm(anti_scalar_attitude(motor_rotor_outer_product(self, other))), plane_weight_norm(motor_flector_outer_product(self, rotor_attitude(other))));
}

HomogeneousMagnitude motor_translator_distance(Motor self, Translator other) {
    return scalar_anti_scalar_add(plane_bulk_norm(anti_scalar_attitude(motor_translator_outer_product(self, other))), plane_weight_norm(motor_flector_outer_product(self, translator_attitude(other))));
}

HomogeneousMagnitude motor_flector_distance(Motor self, Flector other) {
    return scalar_anti_scalar_add(line_bulk_norm(plane_attitude(motor_flector_outer_product(self, other))), multi_vector_weight_norm(motor_multi_vector_outer_product(self, flector_attitude(other))));
}

HomogeneousMagnitude motor_multi_vector_distance(Motor self, MultiVector other) {
    return scalar_anti_scalar_add(multi_vector_bulk_norm(multi_vector_attitude(motor_multi_vector_outer_product(self, other))), multi_vector_weight_norm(motor_multi_vector_outer_product(self, multi_vector_attitude(other))));
}

HomogeneousMagnitude rotor_line_distance(Rotor self, Line other) {
    return scalar_anti_scalar_add(plane_bulk_norm(anti_scalar_attitude(rotor_line_outer_product(self, other))), plane_weight_norm(rotor_point_outer_product(self, line_attitude(other))));
}

HomogeneousMagnitude rotor_motor_distance(Rotor self, Motor other) {
    return scalar_anti_scalar_add(plane_bulk_norm(anti_scalar_attitude(rotor_motor_outer_product(self, other))), plane_weight_norm(rotor_flector_outer_product(self, motor_attitude(other))));
}

HomogeneousMagnitude rotor_translator_distance(Rotor self, Translator other) {
    return scalar_anti_scalar_add(plane_bulk_norm(anti_scalar_attitude(rotor_translator_outer_product(self, other))), plane_weight_norm(rotor_flector_outer_product(self, translator_attitude(other))));
}

HomogeneousMagnitude rotor_flector_distance(Rotor self, Flector other) {
    return scalar_anti_scalar_add(line_bulk_norm(plane_attitude(rotor_flector_outer_product(self, other))), multi_vector_weight_norm(rotor_multi_vector_outer_product(self, flector_attitude(other))));
}

HomogeneousMagnitude rotor_multi_vector_distance(Rotor self, MultiVector other) {
    return scalar_anti_scalar_add(multi_vector_bulk_norm(multi_vector_attitude(rotor_multi_vector_outer_product(self, other))), multi_vector_weight_norm(rotor_multi_vector_outer_product(self, multi_vector_attitude(other))));
}

HomogeneousMagnitude translator_point_distance(Translator self, Point other) {
    return scalar_anti_scalar_add(line_bulk_norm(plane_attitude(translator_point_outer_product(self, other))), translator_weight_norm(translator_scalar_outer_product(self, point_attitude(other))));
}

HomogeneousMagnitude translator_line_distance(Translator self, Line other) {
    return scalar_anti_scalar_add(plane_bulk_norm(anti_scalar_attitude(translator_line_outer_product(self, other))), plane_weight_norm(translator_point_outer_product(self, line_attitude(other))));
}

HomogeneousMagnitude translator_motor_distance(Translator self, Motor other) {
    return scalar_anti_scalar_add(plane_bulk_norm(anti_scalar_attitude(translator_motor_outer_product(self, other))), plane_weight_norm(translator_flector_outer_product(self, motor_attitude(other))));
}

HomogeneousMagnitude translator_rotor_distance(Translator self, Rotor other) {
    return scalar_anti_scalar_add(plane_bulk_norm(anti_scalar_attitude(translator_rotor_outer_product(self, other))), plane_weight_norm(translator_flector_outer_product(self, rotor_attitude(other))));
}

HomogeneousMagnitude translator_flector_distance(Translator self, Flector other) {
    return scalar_anti_scalar_add(line_bulk_norm(plane_attitude(translator_flector_outer_product(self, other))), multi_vector_weight_norm(translator_multi_vector_outer_product(self, flector_attitude(other))));
}

HomogeneousMagnitude translator_multi_vector_distance(Translator self, MultiVector other) {
    return scalar_anti_scalar_add(multi_vector_bulk_norm(multi_vector_attitude(translator_multi_vector_outer_product(self, other))), multi_vector_weight_norm(translator_multi_vector_outer_product(self, multi_vector_attitude(other))));
}

HomogeneousMagnitude flector_point_distance(Flector self, Point other) {
    return scalar_anti_scalar_add(flector_bulk_norm(motor_attitude(flector_point_outer_product(self, other))), flector_weight_norm(flector_scalar_outer_product(self, point_attitude(other))));
}

HomogeneousMagnitude flector_line_distance(Flector self, Line other) {
    return scalar_anti_scalar_add(line_bulk_norm(plane_attitude(flector_line_outer_product(self, other))), motor_weight_norm(flector_point_outer_product(self, line_attitude(other))));
}

HomogeneousMagnitude flector_plane_distance(Flector self, Plane other) {
    return scalar_anti_scalar_add(plane_bulk_norm(anti_scalar_attitude(flector_plane_outer_product(self, other))), plane_weight_norm(flector_line_outer_product(self, plane_attitude(other))));
}

HomogeneousMagnitude flector_motor_distance(Flector self, Motor other) {
    return scalar_anti_scalar_add(line_bulk_norm(plane_attitude(flector_motor_outer_product(self, other))), motor_weight_norm(flector_flector_outer_product(self, motor_attitude(other))));
}

HomogeneousMagnitude flector_rotor_distance(Flector self, Rotor other) {
    return scalar_anti_scalar_add(line_bulk_norm(plane_attitude(flector_rotor_outer_product(self, other))), motor_weight_norm(flector_flector_outer_product(self, rotor_attitude(other))));
}

HomogeneousMagnitude flector_translator_distance(Flector self, Translator other) {
    return scalar_anti_scalar_add(line_bulk_norm(plane_attitude(flector_translator_outer_product(self, other))), motor_weight_norm(flector_flector_outer_product(self, translator_attitude(other))));
}

HomogeneousMagnitude flector_flector_distance(Flector self, Flector other) {
    return scalar_anti_scalar_add(flector_bulk_norm(motor_attitude(flector_flector_outer_product(self, other))), multi_vector_weight_norm(flector_multi_vector_outer_product(self, flector_attitude(other))));
}

HomogeneousMagnitude flector_multi_vector_distance(Flector self, MultiVector other) {
    return scalar_anti_scalar_add(multi_vector_bulk_norm(multi_vector_attitude(flector_multi_vector_outer_product(self, other))), multi_vector_weight_norm(flector_multi_vector_outer_product(self, multi_vector_attitude(other))));
}

HomogeneousMagnitude multi_vector_anti_scalar_distance(MultiVector self, AntiScalar other) {
    return scalar_anti_scalar_add(plane_bulk_norm(anti_scalar_attitude(multi_vector_anti_scalar_outer_product(self, other))), multi_vector_weight_norm(multi_vector_plane_outer_product(self, anti_scalar_attitude(other))));
}

HomogeneousMagnitude multi_vector_homogeneous_magnitude_distance(MultiVector self, HomogeneousMagnitude other) {
    return scalar_anti_scalar_add(multi_vector_bulk_norm(multi_vector_attitude(multi_vector_homogeneous_magnitude_outer_product(self, other))), multi_vector_weight_norm(multi_vector_plane_outer_product(self, homogeneous_magnitude_attitude(other))));
}

HomogeneousMagnitude multi_vector_point_distance(MultiVector self, Point other) {
    return scalar_anti_scalar_add(multi_vector_bulk_norm(multi_vector_attitude(multi_vector_point_outer_product(self, other))), multi_vector_weight_norm(multi_vector_scalar_outer_product(self, point_attitude(other))));
}

HomogeneousMagnitude multi_vector_line_distance(MultiVector self, Line other) {
    return scalar_anti_scalar_add(multi_vector_bulk_norm(multi_vector_attitude(multi_vector_line_outer_product(self, other))), multi_vector_weight_norm(multi_vector_point_outer_product(self, line_attitude(other))));
}

HomogeneousMagnitude multi_vector_plane_distance(MultiVector self, Plane other) {
    return scalar_anti_scalar_add(multi_vector_bulk_norm(multi_vector_attitude(multi_vector_plane_outer_product(self, other))), multi_vector_weight_norm(multi_vector_line_outer_product(self, plane_attitude(other))));
}

HomogeneousMagnitude multi_vector_motor_distance(MultiVector self, Motor other) {
    return scalar_anti_scalar_add(multi_vector_bulk_norm(multi_vector_attitude(multi_vector_motor_outer_product(self, other))), multi_vector_weight_norm(multi_vector_flector_outer_product(self, motor_attitude(other))));
}

HomogeneousMagnitude multi_vector_rotor_distance(MultiVector self, Rotor other) {
    return scalar_anti_scalar_add(multi_vector_bulk_norm(multi_vector_attitude(multi_vector_rotor_outer_product(self, other))), multi_vector_weight_norm(multi_vector_flector_outer_product(self, rotor_attitude(other))));
}

HomogeneousMagnitude multi_vector_translator_distance(MultiVector self, Translator other) {
    return scalar_anti_scalar_add(multi_vector_bulk_norm(multi_vector_attitude(multi_vector_translator_outer_product(self, other))), multi_vector_weight_norm(multi_vector_flector_outer_product(self, translator_attitude(other))));
}

HomogeneousMagnitude multi_vector_flector_distance(MultiVector self, Flector other) {
    return scalar_anti_scalar_add(multi_vector_bulk_norm(multi_vector_attitude(multi_vector_flector_outer_product(self, other))), multi_vector_weight_norm(multi_vector_multi_vector_outer_product(self, flector_attitude(other))));
}

HomogeneousMagnitude multi_vector_multi_vector_distance(MultiVector self, MultiVector other) {
    return scalar_anti_scalar_add(multi_vector_bulk_norm(multi_vector_attitude(multi_vector_multi_vector_outer_product(self, other))), multi_vector_weight_norm(multi_vector_multi_vector_outer_product(self, multi_vector_attitude(other))));
}

Scalar point_scalar_invert(Point self, Scalar other) {
    return point_scalar_sandwich(point_unitize(self), other);
}

AntiScalar point_anti_scalar_invert(Point self, AntiScalar other) {
    return point_anti_scalar_sandwich(point_unitize(self), other);
}

HomogeneousMagnitude point_homogeneous_magnitude_invert(Point self, HomogeneousMagnitude other) {
    return point_homogeneous_magnitude_sandwich(point_unitize(self), other);
}

Point point_point_invert(Point self, Point other) {
    return point_point_sandwich(point_unitize(self), other);
}

Line point_line_invert(Point self, Line other) {
    return point_line_sandwich(point_unitize(self), other);
}

Plane point_plane_invert(Point self, Plane other) {
    return point_plane_sandwich(point_unitize(self), other);
}

Motor point_motor_invert(Point self, Motor other) {
    return point_motor_sandwich(point_unitize(self), other);
}

Rotor point_rotor_invert(Point self, Rotor other) {
    return point_rotor_sandwich(point_unitize(self), other);
}

Translator point_translator_invert(Point self, Translator other) {
    return point_translator_sandwich(point_unitize(self), other);
}

Flector point_flector_invert(Point self, Flector other) {
    return point_flector_sandwich(point_unitize(self), other);
}

MultiVector point_multi_vector_invert(Point self, MultiVector other) {
    return point_multi_vector_sandwich(point_unitize(self), other);
}

Scalar plane_scalar_reflect(Plane self, Scalar other) {
    return plane_scalar_sandwich(plane_unitize(self), other);
}

AntiScalar plane_anti_scalar_reflect(Plane self, AntiScalar other) {
    return plane_anti_scalar_sandwich(plane_unitize(self), other);
}

HomogeneousMagnitude plane_homogeneous_magnitude_reflect(Plane self, HomogeneousMagnitude other) {
    return plane_homogeneous_magnitude_sandwich(plane_unitize(self), other);
}

Point plane_point_reflect(Plane self, Point other) {
    return plane_point_sandwich(plane_unitize(self), other);
}

Line plane_line_reflect(Plane self, Line other) {
    return plane_line_sandwich(plane_unitize(self), other);
}

Plane plane_plane_reflect(Plane self, Plane other) {
    return plane_plane_sandwich(plane_unitize(self), other);
}

Motor plane_motor_reflect(Plane self, Motor other) {
    return plane_motor_sandwich(plane_unitize(self), other);
}

Rotor plane_rotor_reflect(Plane self, Rotor other) {
    return plane_rotor_sandwich(plane_unitize(self), other);
}

Translator plane_translator_reflect(Plane self, Translator other) {
    return plane_translator_sandwich(plane_unitize(self), other);
}

Flector plane_flector_reflect(Plane self, Flector other) {
    return plane_flector_sandwich(plane_unitize(self), other);
}

MultiVector plane_multi_vector_reflect(Plane self, MultiVector other) {
    return plane_multi_vector_sandwich(plane_unitize(self), other);
}

Scalar scalar_bulk(Scalar self) {
    return Scalar(self.g0 * 1.0);
}

Scalar scalar_weight(Scalar self) {
    return Scalar(self.g0 * 0.0);
}

AntiScalar anti_scalar_bulk(AntiScalar self) {
    return AntiScalar(self.g0 * 0.0);
}

AntiScalar anti_scalar_weight(AntiScalar self) {
    return AntiScalar(self.g0 * 1.0);
}

Scalar homogeneous_magnitude_bulk(HomogeneousMagnitude self) {
    return Scalar(self.g0.x * 1.0);
}

AntiScalar homogeneous_magnitude_weight(HomogeneousMagnitude self) {
    return AntiScalar(self.g0.y * 1.0);
}

Point point_bulk(Point self) {
    return Point(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.w) * vec4(1.0, 1.0, 1.0, 0.0));
}

Point point_weight(Point self) {
    return Point(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.w) * vec4(0.0, 0.0, 0.0, 1.0));
}

Line line_bulk(Line self) {
    return Line(vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(0.0, 0.0, 0.0), vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(1.0, 1.0, 1.0));
}

Line line_weight(Line self) {
    return Line(vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(1.0, 1.0, 1.0), vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(0.0, 0.0, 0.0));
}

Plane plane_bulk(Plane self) {
    return Plane(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.w) * vec4(0.0, 0.0, 0.0, 1.0));
}

Plane plane_weight(Plane self) {
    return Plane(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.w) * vec4(1.0, 1.0, 1.0, 0.0));
}

Translator motor_bulk(Motor self) {
    return Translator(vec4(self.g1.x, self.g1.y, self.g1.z, self.g0.w) * vec4(1.0, 1.0, 1.0, 0.0));
}

Rotor motor_weight(Motor self) {
    return Rotor(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.w) * vec4(1.0, 1.0, 1.0, 1.0));
}

Rotor rotor_bulk(Rotor self) {
    return Rotor(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.w) * vec4(0.0, 0.0, 0.0, 0.0));
}

Rotor rotor_weight(Rotor self) {
    return Rotor(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.w) * vec4(1.0, 1.0, 1.0, 1.0));
}

Translator translator_bulk(Translator self) {
    return Translator(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.w) * vec4(1.0, 1.0, 1.0, 0.0));
}

AntiScalar translator_weight(Translator self) {
    return AntiScalar(self.g0.w * 1.0);
}

Flector flector_bulk(Flector self) {
    return Flector(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.w) * vec4(1.0, 1.0, 1.0, 0.0), vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.w) * vec4(0.0, 0.0, 0.0, 1.0));
}

Flector flector_weight(Flector self) {
    return Flector(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.w) * vec4(0.0, 0.0, 0.0, 1.0), vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.w) * vec4(1.0, 1.0, 1.0, 0.0));
}

MultiVector multi_vector_bulk(MultiVector self) {
    return MultiVector(vec2(self.g0.x, self.g0.y) * vec2(1.0, 0.0), vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.w) * vec4(1.0, 1.0, 1.0, 0.0), vec3(self.g2.x, self.g2.y, self.g2.z) * vec3(0.0, 0.0, 0.0), vec3(self.g3.x, self.g3.y, self.g3.z) * vec3(1.0, 1.0, 1.0), vec4(self.g4.x, self.g4.y, self.g4.z, self.g4.w) * vec4(0.0, 0.0, 0.0, 1.0));
}

MultiVector multi_vector_weight(MultiVector self) {
    return MultiVector(vec2(self.g0.x, self.g0.y) * vec2(0.0, 1.0), vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.w) * vec4(0.0, 0.0, 0.0, 1.0), vec3(self.g2.x, self.g2.y, self.g2.z) * vec3(1.0, 1.0, 1.0), vec3(self.g3.x, self.g3.y, self.g3.z) * vec3(0.0, 0.0, 0.0), vec4(self.g4.x, self.g4.y, self.g4.z, self.g4.w) * vec4(1.0, 1.0, 1.0, 0.0));
}

AntiScalar scalar_right_bulk_dual(Scalar self) {
    return scalar_right_complement(scalar_bulk(self));
}

Scalar anti_scalar_right_bulk_dual(AntiScalar self) {
    return anti_scalar_right_complement(anti_scalar_bulk(self));
}

AntiScalar homogeneous_magnitude_right_bulk_dual(HomogeneousMagnitude self) {
    return scalar_right_complement(homogeneous_magnitude_bulk(self));
}

Plane point_right_bulk_dual(Point self) {
    return point_right_complement(point_bulk(self));
}

Line line_right_bulk_dual(Line self) {
    return line_right_complement(line_bulk(self));
}

Point plane_right_bulk_dual(Plane self) {
    return plane_right_complement(plane_bulk(self));
}

Flector flector_right_bulk_dual(Flector self) {
    return flector_right_complement(flector_bulk(self));
}

MultiVector multi_vector_right_bulk_dual(MultiVector self) {
    return multi_vector_right_complement(multi_vector_bulk(self));
}

AntiScalar scalar_right_weight_dual(Scalar self) {
    return scalar_right_complement(scalar_weight(self));
}

Scalar anti_scalar_right_weight_dual(AntiScalar self) {
    return anti_scalar_right_complement(anti_scalar_weight(self));
}

Scalar homogeneous_magnitude_right_weight_dual(HomogeneousMagnitude self) {
    return anti_scalar_right_complement(homogeneous_magnitude_weight(self));
}

Plane point_right_weight_dual(Point self) {
    return point_right_complement(point_weight(self));
}

Line line_right_weight_dual(Line self) {
    return line_right_complement(line_weight(self));
}

Point plane_right_weight_dual(Plane self) {
    return plane_right_complement(plane_weight(self));
}

Scalar translator_right_weight_dual(Translator self) {
    return anti_scalar_right_complement(translator_weight(self));
}

Flector flector_right_weight_dual(Flector self) {
    return flector_right_complement(flector_weight(self));
}

MultiVector multi_vector_right_weight_dual(MultiVector self) {
    return multi_vector_right_complement(multi_vector_weight(self));
}

AntiScalar scalar_left_bulk_dual(Scalar self) {
    return scalar_left_complement(scalar_bulk(self));
}

Scalar anti_scalar_left_bulk_dual(AntiScalar self) {
    return anti_scalar_left_complement(anti_scalar_bulk(self));
}

AntiScalar homogeneous_magnitude_left_bulk_dual(HomogeneousMagnitude self) {
    return scalar_left_complement(homogeneous_magnitude_bulk(self));
}

Plane point_left_bulk_dual(Point self) {
    return point_left_complement(point_bulk(self));
}

Line line_left_bulk_dual(Line self) {
    return line_left_complement(line_bulk(self));
}

Point plane_left_bulk_dual(Plane self) {
    return plane_left_complement(plane_bulk(self));
}

Flector flector_left_bulk_dual(Flector self) {
    return flector_left_complement(flector_bulk(self));
}

MultiVector multi_vector_left_bulk_dual(MultiVector self) {
    return multi_vector_left_complement(multi_vector_bulk(self));
}

AntiScalar scalar_left_weight_dual(Scalar self) {
    return scalar_left_complement(scalar_weight(self));
}

Scalar anti_scalar_left_weight_dual(AntiScalar self) {
    return anti_scalar_left_complement(anti_scalar_weight(self));
}

Scalar homogeneous_magnitude_left_weight_dual(HomogeneousMagnitude self) {
    return anti_scalar_left_complement(homogeneous_magnitude_weight(self));
}

Plane point_left_weight_dual(Point self) {
    return point_left_complement(point_weight(self));
}

Line line_left_weight_dual(Line self) {
    return line_left_complement(line_weight(self));
}

Point plane_left_weight_dual(Plane self) {
    return plane_left_complement(plane_weight(self));
}

Scalar translator_left_weight_dual(Translator self) {
    return anti_scalar_left_complement(translator_weight(self));
}

Flector flector_left_weight_dual(Flector self) {
    return flector_left_complement(flector_weight(self));
}

MultiVector multi_vector_left_weight_dual(MultiVector self) {
    return multi_vector_left_complement(multi_vector_weight(self));
}

Scalar scalar_scalar_bulk_contraction(Scalar self, Scalar other) {
    return scalar_anti_scalar_anti_wedge(self, scalar_right_bulk_dual(other));
}

Scalar scalar_homogeneous_magnitude_bulk_contraction(Scalar self, HomogeneousMagnitude other) {
    return scalar_anti_scalar_anti_wedge(self, homogeneous_magnitude_right_bulk_dual(other));
}

Scalar scalar_multi_vector_bulk_contraction(Scalar self, MultiVector other) {
    return scalar_multi_vector_anti_wedge(self, multi_vector_right_bulk_dual(other));
}

AntiScalar anti_scalar_scalar_bulk_contraction(AntiScalar self, Scalar other) {
    return anti_scalar_anti_scalar_anti_wedge(self, scalar_right_bulk_dual(other));
}

Scalar anti_scalar_anti_scalar_bulk_contraction(AntiScalar self, AntiScalar other) {
    return anti_scalar_scalar_anti_wedge(self, anti_scalar_right_bulk_dual(other));
}

AntiScalar anti_scalar_homogeneous_magnitude_bulk_contraction(AntiScalar self, HomogeneousMagnitude other) {
    return anti_scalar_anti_scalar_anti_wedge(self, homogeneous_magnitude_right_bulk_dual(other));
}

Plane anti_scalar_point_bulk_contraction(AntiScalar self, Point other) {
    return anti_scalar_plane_anti_wedge(self, point_right_bulk_dual(other));
}

Line anti_scalar_line_bulk_contraction(AntiScalar self, Line other) {
    return anti_scalar_line_anti_wedge(self, line_right_bulk_dual(other));
}

Point anti_scalar_plane_bulk_contraction(AntiScalar self, Plane other) {
    return anti_scalar_point_anti_wedge(self, plane_right_bulk_dual(other));
}

Flector anti_scalar_flector_bulk_contraction(AntiScalar self, Flector other) {
    return anti_scalar_flector_anti_wedge(self, flector_right_bulk_dual(other));
}

MultiVector anti_scalar_multi_vector_bulk_contraction(AntiScalar self, MultiVector other) {
    return anti_scalar_multi_vector_anti_wedge(self, multi_vector_right_bulk_dual(other));
}

HomogeneousMagnitude homogeneous_magnitude_scalar_bulk_contraction(HomogeneousMagnitude self, Scalar other) {
    return homogeneous_magnitude_anti_scalar_anti_wedge(self, scalar_right_bulk_dual(other));
}

Scalar homogeneous_magnitude_anti_scalar_bulk_contraction(HomogeneousMagnitude self, AntiScalar other) {
    return homogeneous_magnitude_scalar_anti_wedge(self, anti_scalar_right_bulk_dual(other));
}

HomogeneousMagnitude homogeneous_magnitude_homogeneous_magnitude_bulk_contraction(HomogeneousMagnitude self, HomogeneousMagnitude other) {
    return homogeneous_magnitude_anti_scalar_anti_wedge(self, homogeneous_magnitude_right_bulk_dual(other));
}

Plane homogeneous_magnitude_point_bulk_contraction(HomogeneousMagnitude self, Point other) {
    return homogeneous_magnitude_plane_anti_wedge(self, point_right_bulk_dual(other));
}

Line homogeneous_magnitude_line_bulk_contraction(HomogeneousMagnitude self, Line other) {
    return homogeneous_magnitude_line_anti_wedge(self, line_right_bulk_dual(other));
}

Point homogeneous_magnitude_plane_bulk_contraction(HomogeneousMagnitude self, Plane other) {
    return homogeneous_magnitude_point_anti_wedge(self, plane_right_bulk_dual(other));
}

Flector homogeneous_magnitude_flector_bulk_contraction(HomogeneousMagnitude self, Flector other) {
    return homogeneous_magnitude_flector_anti_wedge(self, flector_right_bulk_dual(other));
}

MultiVector homogeneous_magnitude_multi_vector_bulk_contraction(HomogeneousMagnitude self, MultiVector other) {
    return homogeneous_magnitude_multi_vector_anti_wedge(self, multi_vector_right_bulk_dual(other));
}

Point point_scalar_bulk_contraction(Point self, Scalar other) {
    return point_anti_scalar_anti_wedge(self, scalar_right_bulk_dual(other));
}

Point point_homogeneous_magnitude_bulk_contraction(Point self, HomogeneousMagnitude other) {
    return point_anti_scalar_anti_wedge(self, homogeneous_magnitude_right_bulk_dual(other));
}

Scalar point_point_bulk_contraction(Point self, Point other) {
    return point_plane_anti_wedge(self, point_right_bulk_dual(other));
}

Scalar point_flector_bulk_contraction(Point self, Flector other) {
    return point_flector_anti_wedge(self, flector_right_bulk_dual(other));
}

MultiVector point_multi_vector_bulk_contraction(Point self, MultiVector other) {
    return point_multi_vector_anti_wedge(self, multi_vector_right_bulk_dual(other));
}

Line line_scalar_bulk_contraction(Line self, Scalar other) {
    return line_anti_scalar_anti_wedge(self, scalar_right_bulk_dual(other));
}

Line line_homogeneous_magnitude_bulk_contraction(Line self, HomogeneousMagnitude other) {
    return line_anti_scalar_anti_wedge(self, homogeneous_magnitude_right_bulk_dual(other));
}

Point line_point_bulk_contraction(Line self, Point other) {
    return line_plane_anti_wedge(self, point_right_bulk_dual(other));
}

Scalar line_line_bulk_contraction(Line self, Line other) {
    return line_line_anti_wedge(self, line_right_bulk_dual(other));
}

Point line_flector_bulk_contraction(Line self, Flector other) {
    return line_flector_anti_wedge(self, flector_right_bulk_dual(other));
}

MultiVector line_multi_vector_bulk_contraction(Line self, MultiVector other) {
    return line_multi_vector_anti_wedge(self, multi_vector_right_bulk_dual(other));
}

Plane plane_scalar_bulk_contraction(Plane self, Scalar other) {
    return plane_anti_scalar_anti_wedge(self, scalar_right_bulk_dual(other));
}

Plane plane_homogeneous_magnitude_bulk_contraction(Plane self, HomogeneousMagnitude other) {
    return plane_anti_scalar_anti_wedge(self, homogeneous_magnitude_right_bulk_dual(other));
}

Line plane_point_bulk_contraction(Plane self, Point other) {
    return plane_plane_anti_wedge(self, point_right_bulk_dual(other));
}

Point plane_line_bulk_contraction(Plane self, Line other) {
    return plane_line_anti_wedge(self, line_right_bulk_dual(other));
}

Scalar plane_plane_bulk_contraction(Plane self, Plane other) {
    return plane_point_anti_wedge(self, plane_right_bulk_dual(other));
}

MultiVector plane_flector_bulk_contraction(Plane self, Flector other) {
    return plane_flector_anti_wedge(self, flector_right_bulk_dual(other));
}

MultiVector plane_multi_vector_bulk_contraction(Plane self, MultiVector other) {
    return plane_multi_vector_anti_wedge(self, multi_vector_right_bulk_dual(other));
}

Motor motor_scalar_bulk_contraction(Motor self, Scalar other) {
    return motor_anti_scalar_anti_wedge(self, scalar_right_bulk_dual(other));
}

Scalar motor_anti_scalar_bulk_contraction(Motor self, AntiScalar other) {
    return motor_scalar_anti_wedge(self, anti_scalar_right_bulk_dual(other));
}

Motor motor_homogeneous_magnitude_bulk_contraction(Motor self, HomogeneousMagnitude other) {
    return motor_anti_scalar_anti_wedge(self, homogeneous_magnitude_right_bulk_dual(other));
}

Flector motor_point_bulk_contraction(Motor self, Point other) {
    return motor_plane_anti_wedge(self, point_right_bulk_dual(other));
}

MultiVector motor_line_bulk_contraction(Motor self, Line other) {
    return motor_line_anti_wedge(self, line_right_bulk_dual(other));
}

Point motor_plane_bulk_contraction(Motor self, Plane other) {
    return motor_point_anti_wedge(self, plane_right_bulk_dual(other));
}

Flector motor_flector_bulk_contraction(Motor self, Flector other) {
    return motor_flector_anti_wedge(self, flector_right_bulk_dual(other));
}

MultiVector motor_multi_vector_bulk_contraction(Motor self, MultiVector other) {
    return motor_multi_vector_anti_wedge(self, multi_vector_right_bulk_dual(other));
}

Rotor rotor_scalar_bulk_contraction(Rotor self, Scalar other) {
    return rotor_anti_scalar_anti_wedge(self, scalar_right_bulk_dual(other));
}

Scalar rotor_anti_scalar_bulk_contraction(Rotor self, AntiScalar other) {
    return rotor_scalar_anti_wedge(self, anti_scalar_right_bulk_dual(other));
}

Rotor rotor_homogeneous_magnitude_bulk_contraction(Rotor self, HomogeneousMagnitude other) {
    return rotor_anti_scalar_anti_wedge(self, homogeneous_magnitude_right_bulk_dual(other));
}

Flector rotor_point_bulk_contraction(Rotor self, Point other) {
    return rotor_plane_anti_wedge(self, point_right_bulk_dual(other));
}

MultiVector rotor_line_bulk_contraction(Rotor self, Line other) {
    return rotor_line_anti_wedge(self, line_right_bulk_dual(other));
}

Point rotor_plane_bulk_contraction(Rotor self, Plane other) {
    return rotor_point_anti_wedge(self, plane_right_bulk_dual(other));
}

Flector rotor_flector_bulk_contraction(Rotor self, Flector other) {
    return rotor_flector_anti_wedge(self, flector_right_bulk_dual(other));
}

MultiVector rotor_multi_vector_bulk_contraction(Rotor self, MultiVector other) {
    return rotor_multi_vector_anti_wedge(self, multi_vector_right_bulk_dual(other));
}

Translator translator_scalar_bulk_contraction(Translator self, Scalar other) {
    return translator_anti_scalar_anti_wedge(self, scalar_right_bulk_dual(other));
}

Scalar translator_anti_scalar_bulk_contraction(Translator self, AntiScalar other) {
    return translator_scalar_anti_wedge(self, anti_scalar_right_bulk_dual(other));
}

Translator translator_homogeneous_magnitude_bulk_contraction(Translator self, HomogeneousMagnitude other) {
    return translator_anti_scalar_anti_wedge(self, homogeneous_magnitude_right_bulk_dual(other));
}

Flector translator_point_bulk_contraction(Translator self, Point other) {
    return translator_plane_anti_wedge(self, point_right_bulk_dual(other));
}

MultiVector translator_line_bulk_contraction(Translator self, Line other) {
    return translator_line_anti_wedge(self, line_right_bulk_dual(other));
}

Point translator_plane_bulk_contraction(Translator self, Plane other) {
    return translator_point_anti_wedge(self, plane_right_bulk_dual(other));
}

Flector translator_flector_bulk_contraction(Translator self, Flector other) {
    return translator_flector_anti_wedge(self, flector_right_bulk_dual(other));
}

MultiVector translator_multi_vector_bulk_contraction(Translator self, MultiVector other) {
    return translator_multi_vector_anti_wedge(self, multi_vector_right_bulk_dual(other));
}

Flector flector_scalar_bulk_contraction(Flector self, Scalar other) {
    return flector_anti_scalar_anti_wedge(self, scalar_right_bulk_dual(other));
}

Flector flector_homogeneous_magnitude_bulk_contraction(Flector self, HomogeneousMagnitude other) {
    return flector_anti_scalar_anti_wedge(self, homogeneous_magnitude_right_bulk_dual(other));
}

MultiVector flector_point_bulk_contraction(Flector self, Point other) {
    return flector_plane_anti_wedge(self, point_right_bulk_dual(other));
}

Point flector_line_bulk_contraction(Flector self, Line other) {
    return flector_line_anti_wedge(self, line_right_bulk_dual(other));
}

Scalar flector_plane_bulk_contraction(Flector self, Plane other) {
    return flector_point_anti_wedge(self, plane_right_bulk_dual(other));
}

MultiVector flector_flector_bulk_contraction(Flector self, Flector other) {
    return flector_flector_anti_wedge(self, flector_right_bulk_dual(other));
}

MultiVector flector_multi_vector_bulk_contraction(Flector self, MultiVector other) {
    return flector_multi_vector_anti_wedge(self, multi_vector_right_bulk_dual(other));
}

MultiVector multi_vector_scalar_bulk_contraction(MultiVector self, Scalar other) {
    return multi_vector_anti_scalar_anti_wedge(self, scalar_right_bulk_dual(other));
}

Scalar multi_vector_anti_scalar_bulk_contraction(MultiVector self, AntiScalar other) {
    return multi_vector_scalar_anti_wedge(self, anti_scalar_right_bulk_dual(other));
}

MultiVector multi_vector_homogeneous_magnitude_bulk_contraction(MultiVector self, HomogeneousMagnitude other) {
    return multi_vector_anti_scalar_anti_wedge(self, homogeneous_magnitude_right_bulk_dual(other));
}

MultiVector multi_vector_point_bulk_contraction(MultiVector self, Point other) {
    return multi_vector_plane_anti_wedge(self, point_right_bulk_dual(other));
}

MultiVector multi_vector_line_bulk_contraction(MultiVector self, Line other) {
    return multi_vector_line_anti_wedge(self, line_right_bulk_dual(other));
}

MultiVector multi_vector_plane_bulk_contraction(MultiVector self, Plane other) {
    return multi_vector_point_anti_wedge(self, plane_right_bulk_dual(other));
}

MultiVector multi_vector_flector_bulk_contraction(MultiVector self, Flector other) {
    return multi_vector_flector_anti_wedge(self, flector_right_bulk_dual(other));
}

MultiVector multi_vector_multi_vector_bulk_contraction(MultiVector self, MultiVector other) {
    return multi_vector_multi_vector_anti_wedge(self, multi_vector_right_bulk_dual(other));
}

Scalar scalar_scalar_weight_contraction(Scalar self, Scalar other) {
    return scalar_anti_scalar_anti_wedge(self, scalar_right_weight_dual(other));
}

Scalar scalar_multi_vector_weight_contraction(Scalar self, MultiVector other) {
    return scalar_multi_vector_anti_wedge(self, multi_vector_right_weight_dual(other));
}

AntiScalar anti_scalar_scalar_weight_contraction(AntiScalar self, Scalar other) {
    return anti_scalar_anti_scalar_anti_wedge(self, scalar_right_weight_dual(other));
}

Scalar anti_scalar_anti_scalar_weight_contraction(AntiScalar self, AntiScalar other) {
    return anti_scalar_scalar_anti_wedge(self, anti_scalar_right_weight_dual(other));
}

Scalar anti_scalar_homogeneous_magnitude_weight_contraction(AntiScalar self, HomogeneousMagnitude other) {
    return anti_scalar_scalar_anti_wedge(self, homogeneous_magnitude_right_weight_dual(other));
}

Plane anti_scalar_point_weight_contraction(AntiScalar self, Point other) {
    return anti_scalar_plane_anti_wedge(self, point_right_weight_dual(other));
}

Line anti_scalar_line_weight_contraction(AntiScalar self, Line other) {
    return anti_scalar_line_anti_wedge(self, line_right_weight_dual(other));
}

Point anti_scalar_plane_weight_contraction(AntiScalar self, Plane other) {
    return anti_scalar_point_anti_wedge(self, plane_right_weight_dual(other));
}

Scalar anti_scalar_translator_weight_contraction(AntiScalar self, Translator other) {
    return anti_scalar_scalar_anti_wedge(self, translator_right_weight_dual(other));
}

Flector anti_scalar_flector_weight_contraction(AntiScalar self, Flector other) {
    return anti_scalar_flector_anti_wedge(self, flector_right_weight_dual(other));
}

MultiVector anti_scalar_multi_vector_weight_contraction(AntiScalar self, MultiVector other) {
    return anti_scalar_multi_vector_anti_wedge(self, multi_vector_right_weight_dual(other));
}

HomogeneousMagnitude homogeneous_magnitude_scalar_weight_contraction(HomogeneousMagnitude self, Scalar other) {
    return homogeneous_magnitude_anti_scalar_anti_wedge(self, scalar_right_weight_dual(other));
}

Scalar homogeneous_magnitude_anti_scalar_weight_contraction(HomogeneousMagnitude self, AntiScalar other) {
    return homogeneous_magnitude_scalar_anti_wedge(self, anti_scalar_right_weight_dual(other));
}

Scalar homogeneous_magnitude_homogeneous_magnitude_weight_contraction(HomogeneousMagnitude self, HomogeneousMagnitude other) {
    return homogeneous_magnitude_scalar_anti_wedge(self, homogeneous_magnitude_right_weight_dual(other));
}

Plane homogeneous_magnitude_point_weight_contraction(HomogeneousMagnitude self, Point other) {
    return homogeneous_magnitude_plane_anti_wedge(self, point_right_weight_dual(other));
}

Line homogeneous_magnitude_line_weight_contraction(HomogeneousMagnitude self, Line other) {
    return homogeneous_magnitude_line_anti_wedge(self, line_right_weight_dual(other));
}

Point homogeneous_magnitude_plane_weight_contraction(HomogeneousMagnitude self, Plane other) {
    return homogeneous_magnitude_point_anti_wedge(self, plane_right_weight_dual(other));
}

Scalar homogeneous_magnitude_translator_weight_contraction(HomogeneousMagnitude self, Translator other) {
    return homogeneous_magnitude_scalar_anti_wedge(self, translator_right_weight_dual(other));
}

Flector homogeneous_magnitude_flector_weight_contraction(HomogeneousMagnitude self, Flector other) {
    return homogeneous_magnitude_flector_anti_wedge(self, flector_right_weight_dual(other));
}

MultiVector homogeneous_magnitude_multi_vector_weight_contraction(HomogeneousMagnitude self, MultiVector other) {
    return homogeneous_magnitude_multi_vector_anti_wedge(self, multi_vector_right_weight_dual(other));
}

Point point_scalar_weight_contraction(Point self, Scalar other) {
    return point_anti_scalar_anti_wedge(self, scalar_right_weight_dual(other));
}

Scalar point_point_weight_contraction(Point self, Point other) {
    return point_plane_anti_wedge(self, point_right_weight_dual(other));
}

Scalar point_flector_weight_contraction(Point self, Flector other) {
    return point_flector_anti_wedge(self, flector_right_weight_dual(other));
}

MultiVector point_multi_vector_weight_contraction(Point self, MultiVector other) {
    return point_multi_vector_anti_wedge(self, multi_vector_right_weight_dual(other));
}

Line line_scalar_weight_contraction(Line self, Scalar other) {
    return line_anti_scalar_anti_wedge(self, scalar_right_weight_dual(other));
}

Point line_point_weight_contraction(Line self, Point other) {
    return line_plane_anti_wedge(self, point_right_weight_dual(other));
}

Scalar line_line_weight_contraction(Line self, Line other) {
    return line_line_anti_wedge(self, line_right_weight_dual(other));
}

Point line_flector_weight_contraction(Line self, Flector other) {
    return line_flector_anti_wedge(self, flector_right_weight_dual(other));
}

MultiVector line_multi_vector_weight_contraction(Line self, MultiVector other) {
    return line_multi_vector_anti_wedge(self, multi_vector_right_weight_dual(other));
}

Plane plane_scalar_weight_contraction(Plane self, Scalar other) {
    return plane_anti_scalar_anti_wedge(self, scalar_right_weight_dual(other));
}

Line plane_point_weight_contraction(Plane self, Point other) {
    return plane_plane_anti_wedge(self, point_right_weight_dual(other));
}

Point plane_line_weight_contraction(Plane self, Line other) {
    return plane_line_anti_wedge(self, line_right_weight_dual(other));
}

Scalar plane_plane_weight_contraction(Plane self, Plane other) {
    return plane_point_anti_wedge(self, plane_right_weight_dual(other));
}

MultiVector plane_flector_weight_contraction(Plane self, Flector other) {
    return plane_flector_anti_wedge(self, flector_right_weight_dual(other));
}

MultiVector plane_multi_vector_weight_contraction(Plane self, MultiVector other) {
    return plane_multi_vector_anti_wedge(self, multi_vector_right_weight_dual(other));
}

Motor motor_scalar_weight_contraction(Motor self, Scalar other) {
    return motor_anti_scalar_anti_wedge(self, scalar_right_weight_dual(other));
}

Scalar motor_anti_scalar_weight_contraction(Motor self, AntiScalar other) {
    return motor_scalar_anti_wedge(self, anti_scalar_right_weight_dual(other));
}

Scalar motor_homogeneous_magnitude_weight_contraction(Motor self, HomogeneousMagnitude other) {
    return motor_scalar_anti_wedge(self, homogeneous_magnitude_right_weight_dual(other));
}

Flector motor_point_weight_contraction(Motor self, Point other) {
    return motor_plane_anti_wedge(self, point_right_weight_dual(other));
}

MultiVector motor_line_weight_contraction(Motor self, Line other) {
    return motor_line_anti_wedge(self, line_right_weight_dual(other));
}

Point motor_plane_weight_contraction(Motor self, Plane other) {
    return motor_point_anti_wedge(self, plane_right_weight_dual(other));
}

Scalar motor_translator_weight_contraction(Motor self, Translator other) {
    return motor_scalar_anti_wedge(self, translator_right_weight_dual(other));
}

Flector motor_flector_weight_contraction(Motor self, Flector other) {
    return motor_flector_anti_wedge(self, flector_right_weight_dual(other));
}

MultiVector motor_multi_vector_weight_contraction(Motor self, MultiVector other) {
    return motor_multi_vector_anti_wedge(self, multi_vector_right_weight_dual(other));
}

Rotor rotor_scalar_weight_contraction(Rotor self, Scalar other) {
    return rotor_anti_scalar_anti_wedge(self, scalar_right_weight_dual(other));
}

Scalar rotor_anti_scalar_weight_contraction(Rotor self, AntiScalar other) {
    return rotor_scalar_anti_wedge(self, anti_scalar_right_weight_dual(other));
}

Scalar rotor_homogeneous_magnitude_weight_contraction(Rotor self, HomogeneousMagnitude other) {
    return rotor_scalar_anti_wedge(self, homogeneous_magnitude_right_weight_dual(other));
}

Flector rotor_point_weight_contraction(Rotor self, Point other) {
    return rotor_plane_anti_wedge(self, point_right_weight_dual(other));
}

MultiVector rotor_line_weight_contraction(Rotor self, Line other) {
    return rotor_line_anti_wedge(self, line_right_weight_dual(other));
}

Point rotor_plane_weight_contraction(Rotor self, Plane other) {
    return rotor_point_anti_wedge(self, plane_right_weight_dual(other));
}

Scalar rotor_translator_weight_contraction(Rotor self, Translator other) {
    return rotor_scalar_anti_wedge(self, translator_right_weight_dual(other));
}

Flector rotor_flector_weight_contraction(Rotor self, Flector other) {
    return rotor_flector_anti_wedge(self, flector_right_weight_dual(other));
}

MultiVector rotor_multi_vector_weight_contraction(Rotor self, MultiVector other) {
    return rotor_multi_vector_anti_wedge(self, multi_vector_right_weight_dual(other));
}

Translator translator_scalar_weight_contraction(Translator self, Scalar other) {
    return translator_anti_scalar_anti_wedge(self, scalar_right_weight_dual(other));
}

Scalar translator_anti_scalar_weight_contraction(Translator self, AntiScalar other) {
    return translator_scalar_anti_wedge(self, anti_scalar_right_weight_dual(other));
}

Scalar translator_homogeneous_magnitude_weight_contraction(Translator self, HomogeneousMagnitude other) {
    return translator_scalar_anti_wedge(self, homogeneous_magnitude_right_weight_dual(other));
}

Flector translator_point_weight_contraction(Translator self, Point other) {
    return translator_plane_anti_wedge(self, point_right_weight_dual(other));
}

MultiVector translator_line_weight_contraction(Translator self, Line other) {
    return translator_line_anti_wedge(self, line_right_weight_dual(other));
}

Point translator_plane_weight_contraction(Translator self, Plane other) {
    return translator_point_anti_wedge(self, plane_right_weight_dual(other));
}

Scalar translator_translator_weight_contraction(Translator self, Translator other) {
    return translator_scalar_anti_wedge(self, translator_right_weight_dual(other));
}

Flector translator_flector_weight_contraction(Translator self, Flector other) {
    return translator_flector_anti_wedge(self, flector_right_weight_dual(other));
}

MultiVector translator_multi_vector_weight_contraction(Translator self, MultiVector other) {
    return translator_multi_vector_anti_wedge(self, multi_vector_right_weight_dual(other));
}

Flector flector_scalar_weight_contraction(Flector self, Scalar other) {
    return flector_anti_scalar_anti_wedge(self, scalar_right_weight_dual(other));
}

MultiVector flector_point_weight_contraction(Flector self, Point other) {
    return flector_plane_anti_wedge(self, point_right_weight_dual(other));
}

Point flector_line_weight_contraction(Flector self, Line other) {
    return flector_line_anti_wedge(self, line_right_weight_dual(other));
}

Scalar flector_plane_weight_contraction(Flector self, Plane other) {
    return flector_point_anti_wedge(self, plane_right_weight_dual(other));
}

MultiVector flector_flector_weight_contraction(Flector self, Flector other) {
    return flector_flector_anti_wedge(self, flector_right_weight_dual(other));
}

MultiVector flector_multi_vector_weight_contraction(Flector self, MultiVector other) {
    return flector_multi_vector_anti_wedge(self, multi_vector_right_weight_dual(other));
}

MultiVector multi_vector_scalar_weight_contraction(MultiVector self, Scalar other) {
    return multi_vector_anti_scalar_anti_wedge(self, scalar_right_weight_dual(other));
}

Scalar multi_vector_anti_scalar_weight_contraction(MultiVector self, AntiScalar other) {
    return multi_vector_scalar_anti_wedge(self, anti_scalar_right_weight_dual(other));
}

Scalar multi_vector_homogeneous_magnitude_weight_contraction(MultiVector self, HomogeneousMagnitude other) {
    return multi_vector_scalar_anti_wedge(self, homogeneous_magnitude_right_weight_dual(other));
}

MultiVector multi_vector_point_weight_contraction(MultiVector self, Point other) {
    return multi_vector_plane_anti_wedge(self, point_right_weight_dual(other));
}

MultiVector multi_vector_line_weight_contraction(MultiVector self, Line other) {
    return multi_vector_line_anti_wedge(self, line_right_weight_dual(other));
}

MultiVector multi_vector_plane_weight_contraction(MultiVector self, Plane other) {
    return multi_vector_point_anti_wedge(self, plane_right_weight_dual(other));
}

Scalar multi_vector_translator_weight_contraction(MultiVector self, Translator other) {
    return multi_vector_scalar_anti_wedge(self, translator_right_weight_dual(other));
}

MultiVector multi_vector_flector_weight_contraction(MultiVector self, Flector other) {
    return multi_vector_flector_anti_wedge(self, flector_right_weight_dual(other));
}

MultiVector multi_vector_multi_vector_weight_contraction(MultiVector self, MultiVector other) {
    return multi_vector_multi_vector_anti_wedge(self, multi_vector_right_weight_dual(other));
}

AntiScalar scalar_scalar_bulk_expansion(Scalar self, Scalar other) {
    return scalar_anti_scalar_wedge(self, scalar_right_bulk_dual(other));
}

Scalar scalar_anti_scalar_bulk_expansion(Scalar self, AntiScalar other) {
    return scalar_scalar_wedge(self, anti_scalar_right_bulk_dual(other));
}

AntiScalar scalar_homogeneous_magnitude_bulk_expansion(Scalar self, HomogeneousMagnitude other) {
    return scalar_anti_scalar_wedge(self, homogeneous_magnitude_right_bulk_dual(other));
}

Plane scalar_point_bulk_expansion(Scalar self, Point other) {
    return scalar_plane_wedge(self, point_right_bulk_dual(other));
}

Line scalar_line_bulk_expansion(Scalar self, Line other) {
    return scalar_line_wedge(self, line_right_bulk_dual(other));
}

Point scalar_plane_bulk_expansion(Scalar self, Plane other) {
    return scalar_point_wedge(self, plane_right_bulk_dual(other));
}

Flector scalar_flector_bulk_expansion(Scalar self, Flector other) {
    return scalar_flector_wedge(self, flector_right_bulk_dual(other));
}

MultiVector scalar_multi_vector_bulk_expansion(Scalar self, MultiVector other) {
    return scalar_multi_vector_wedge(self, multi_vector_right_bulk_dual(other));
}

AntiScalar anti_scalar_anti_scalar_bulk_expansion(AntiScalar self, AntiScalar other) {
    return anti_scalar_scalar_wedge(self, anti_scalar_right_bulk_dual(other));
}

AntiScalar anti_scalar_multi_vector_bulk_expansion(AntiScalar self, MultiVector other) {
    return anti_scalar_multi_vector_wedge(self, multi_vector_right_bulk_dual(other));
}

AntiScalar homogeneous_magnitude_scalar_bulk_expansion(HomogeneousMagnitude self, Scalar other) {
    return homogeneous_magnitude_anti_scalar_wedge(self, scalar_right_bulk_dual(other));
}

HomogeneousMagnitude homogeneous_magnitude_anti_scalar_bulk_expansion(HomogeneousMagnitude self, AntiScalar other) {
    return homogeneous_magnitude_scalar_wedge(self, anti_scalar_right_bulk_dual(other));
}

AntiScalar homogeneous_magnitude_homogeneous_magnitude_bulk_expansion(HomogeneousMagnitude self, HomogeneousMagnitude other) {
    return homogeneous_magnitude_anti_scalar_wedge(self, homogeneous_magnitude_right_bulk_dual(other));
}

Plane homogeneous_magnitude_point_bulk_expansion(HomogeneousMagnitude self, Point other) {
    return homogeneous_magnitude_plane_wedge(self, point_right_bulk_dual(other));
}

Line homogeneous_magnitude_line_bulk_expansion(HomogeneousMagnitude self, Line other) {
    return homogeneous_magnitude_line_wedge(self, line_right_bulk_dual(other));
}

Point homogeneous_magnitude_plane_bulk_expansion(HomogeneousMagnitude self, Plane other) {
    return homogeneous_magnitude_point_wedge(self, plane_right_bulk_dual(other));
}

Flector homogeneous_magnitude_flector_bulk_expansion(HomogeneousMagnitude self, Flector other) {
    return homogeneous_magnitude_flector_wedge(self, flector_right_bulk_dual(other));
}

MultiVector homogeneous_magnitude_multi_vector_bulk_expansion(HomogeneousMagnitude self, MultiVector other) {
    return homogeneous_magnitude_multi_vector_wedge(self, multi_vector_right_bulk_dual(other));
}

Point point_anti_scalar_bulk_expansion(Point self, AntiScalar other) {
    return point_scalar_wedge(self, anti_scalar_right_bulk_dual(other));
}

AntiScalar point_point_bulk_expansion(Point self, Point other) {
    return point_plane_wedge(self, point_right_bulk_dual(other));
}

Plane point_line_bulk_expansion(Point self, Line other) {
    return point_line_wedge(self, line_right_bulk_dual(other));
}

Line point_plane_bulk_expansion(Point self, Plane other) {
    return point_point_wedge(self, plane_right_bulk_dual(other));
}

Motor point_flector_bulk_expansion(Point self, Flector other) {
    return point_flector_wedge(self, flector_right_bulk_dual(other));
}

MultiVector point_multi_vector_bulk_expansion(Point self, MultiVector other) {
    return point_multi_vector_wedge(self, multi_vector_right_bulk_dual(other));
}

Line line_anti_scalar_bulk_expansion(Line self, AntiScalar other) {
    return line_scalar_wedge(self, anti_scalar_right_bulk_dual(other));
}

AntiScalar line_line_bulk_expansion(Line self, Line other) {
    return line_line_wedge(self, line_right_bulk_dual(other));
}

Plane line_plane_bulk_expansion(Line self, Plane other) {
    return line_point_wedge(self, plane_right_bulk_dual(other));
}

Plane line_flector_bulk_expansion(Line self, Flector other) {
    return line_flector_wedge(self, flector_right_bulk_dual(other));
}

MultiVector line_multi_vector_bulk_expansion(Line self, MultiVector other) {
    return line_multi_vector_wedge(self, multi_vector_right_bulk_dual(other));
}

Plane plane_anti_scalar_bulk_expansion(Plane self, AntiScalar other) {
    return plane_scalar_wedge(self, anti_scalar_right_bulk_dual(other));
}

AntiScalar plane_plane_bulk_expansion(Plane self, Plane other) {
    return plane_point_wedge(self, plane_right_bulk_dual(other));
}

AntiScalar plane_flector_bulk_expansion(Plane self, Flector other) {
    return plane_flector_wedge(self, flector_right_bulk_dual(other));
}

MultiVector plane_multi_vector_bulk_expansion(Plane self, MultiVector other) {
    return plane_multi_vector_wedge(self, multi_vector_right_bulk_dual(other));
}

Motor motor_anti_scalar_bulk_expansion(Motor self, AntiScalar other) {
    return motor_scalar_wedge(self, anti_scalar_right_bulk_dual(other));
}

AntiScalar motor_line_bulk_expansion(Motor self, Line other) {
    return motor_line_wedge(self, line_right_bulk_dual(other));
}

Plane motor_plane_bulk_expansion(Motor self, Plane other) {
    return motor_point_wedge(self, plane_right_bulk_dual(other));
}

Plane motor_flector_bulk_expansion(Motor self, Flector other) {
    return motor_flector_wedge(self, flector_right_bulk_dual(other));
}

MultiVector motor_multi_vector_bulk_expansion(Motor self, MultiVector other) {
    return motor_multi_vector_wedge(self, multi_vector_right_bulk_dual(other));
}

Rotor rotor_anti_scalar_bulk_expansion(Rotor self, AntiScalar other) {
    return rotor_scalar_wedge(self, anti_scalar_right_bulk_dual(other));
}

AntiScalar rotor_line_bulk_expansion(Rotor self, Line other) {
    return rotor_line_wedge(self, line_right_bulk_dual(other));
}

Plane rotor_plane_bulk_expansion(Rotor self, Plane other) {
    return rotor_point_wedge(self, plane_right_bulk_dual(other));
}

Plane rotor_flector_bulk_expansion(Rotor self, Flector other) {
    return rotor_flector_wedge(self, flector_right_bulk_dual(other));
}

MultiVector rotor_multi_vector_bulk_expansion(Rotor self, MultiVector other) {
    return rotor_multi_vector_wedge(self, multi_vector_right_bulk_dual(other));
}

Translator translator_anti_scalar_bulk_expansion(Translator self, AntiScalar other) {
    return translator_scalar_wedge(self, anti_scalar_right_bulk_dual(other));
}

AntiScalar translator_line_bulk_expansion(Translator self, Line other) {
    return translator_line_wedge(self, line_right_bulk_dual(other));
}

Plane translator_plane_bulk_expansion(Translator self, Plane other) {
    return translator_point_wedge(self, plane_right_bulk_dual(other));
}

Plane translator_flector_bulk_expansion(Translator self, Flector other) {
    return translator_flector_wedge(self, flector_right_bulk_dual(other));
}

MultiVector translator_multi_vector_bulk_expansion(Translator self, MultiVector other) {
    return translator_multi_vector_wedge(self, multi_vector_right_bulk_dual(other));
}

Flector flector_anti_scalar_bulk_expansion(Flector self, AntiScalar other) {
    return flector_scalar_wedge(self, anti_scalar_right_bulk_dual(other));
}

AntiScalar flector_point_bulk_expansion(Flector self, Point other) {
    return flector_plane_wedge(self, point_right_bulk_dual(other));
}

Plane flector_line_bulk_expansion(Flector self, Line other) {
    return flector_line_wedge(self, line_right_bulk_dual(other));
}

Motor flector_plane_bulk_expansion(Flector self, Plane other) {
    return flector_point_wedge(self, plane_right_bulk_dual(other));
}

Motor flector_flector_bulk_expansion(Flector self, Flector other) {
    return flector_flector_wedge(self, flector_right_bulk_dual(other));
}

MultiVector flector_multi_vector_bulk_expansion(Flector self, MultiVector other) {
    return flector_multi_vector_wedge(self, multi_vector_right_bulk_dual(other));
}

AntiScalar multi_vector_scalar_bulk_expansion(MultiVector self, Scalar other) {
    return multi_vector_anti_scalar_wedge(self, scalar_right_bulk_dual(other));
}

MultiVector multi_vector_anti_scalar_bulk_expansion(MultiVector self, AntiScalar other) {
    return multi_vector_scalar_wedge(self, anti_scalar_right_bulk_dual(other));
}

AntiScalar multi_vector_homogeneous_magnitude_bulk_expansion(MultiVector self, HomogeneousMagnitude other) {
    return multi_vector_anti_scalar_wedge(self, homogeneous_magnitude_right_bulk_dual(other));
}

MultiVector multi_vector_point_bulk_expansion(MultiVector self, Point other) {
    return multi_vector_plane_wedge(self, point_right_bulk_dual(other));
}

MultiVector multi_vector_line_bulk_expansion(MultiVector self, Line other) {
    return multi_vector_line_wedge(self, line_right_bulk_dual(other));
}

MultiVector multi_vector_plane_bulk_expansion(MultiVector self, Plane other) {
    return multi_vector_point_wedge(self, plane_right_bulk_dual(other));
}

MultiVector multi_vector_flector_bulk_expansion(MultiVector self, Flector other) {
    return multi_vector_flector_wedge(self, flector_right_bulk_dual(other));
}

MultiVector multi_vector_multi_vector_bulk_expansion(MultiVector self, MultiVector other) {
    return multi_vector_multi_vector_wedge(self, multi_vector_right_bulk_dual(other));
}

AntiScalar scalar_scalar_weight_expansion(Scalar self, Scalar other) {
    return scalar_anti_scalar_wedge(self, scalar_right_weight_dual(other));
}

Scalar scalar_anti_scalar_weight_expansion(Scalar self, AntiScalar other) {
    return scalar_scalar_wedge(self, anti_scalar_right_weight_dual(other));
}

Scalar scalar_homogeneous_magnitude_weight_expansion(Scalar self, HomogeneousMagnitude other) {
    return scalar_scalar_wedge(self, homogeneous_magnitude_right_weight_dual(other));
}

Plane scalar_point_weight_expansion(Scalar self, Point other) {
    return scalar_plane_wedge(self, point_right_weight_dual(other));
}

Line scalar_line_weight_expansion(Scalar self, Line other) {
    return scalar_line_wedge(self, line_right_weight_dual(other));
}

Point scalar_plane_weight_expansion(Scalar self, Plane other) {
    return scalar_point_wedge(self, plane_right_weight_dual(other));
}

Scalar scalar_translator_weight_expansion(Scalar self, Translator other) {
    return scalar_scalar_wedge(self, translator_right_weight_dual(other));
}

Flector scalar_flector_weight_expansion(Scalar self, Flector other) {
    return scalar_flector_wedge(self, flector_right_weight_dual(other));
}

MultiVector scalar_multi_vector_weight_expansion(Scalar self, MultiVector other) {
    return scalar_multi_vector_wedge(self, multi_vector_right_weight_dual(other));
}

AntiScalar anti_scalar_anti_scalar_weight_expansion(AntiScalar self, AntiScalar other) {
    return anti_scalar_scalar_wedge(self, anti_scalar_right_weight_dual(other));
}

AntiScalar anti_scalar_homogeneous_magnitude_weight_expansion(AntiScalar self, HomogeneousMagnitude other) {
    return anti_scalar_scalar_wedge(self, homogeneous_magnitude_right_weight_dual(other));
}

AntiScalar anti_scalar_translator_weight_expansion(AntiScalar self, Translator other) {
    return anti_scalar_scalar_wedge(self, translator_right_weight_dual(other));
}

AntiScalar anti_scalar_multi_vector_weight_expansion(AntiScalar self, MultiVector other) {
    return anti_scalar_multi_vector_wedge(self, multi_vector_right_weight_dual(other));
}

AntiScalar homogeneous_magnitude_scalar_weight_expansion(HomogeneousMagnitude self, Scalar other) {
    return homogeneous_magnitude_anti_scalar_wedge(self, scalar_right_weight_dual(other));
}

HomogeneousMagnitude homogeneous_magnitude_anti_scalar_weight_expansion(HomogeneousMagnitude self, AntiScalar other) {
    return homogeneous_magnitude_scalar_wedge(self, anti_scalar_right_weight_dual(other));
}

HomogeneousMagnitude homogeneous_magnitude_homogeneous_magnitude_weight_expansion(HomogeneousMagnitude self, HomogeneousMagnitude other) {
    return homogeneous_magnitude_scalar_wedge(self, homogeneous_magnitude_right_weight_dual(other));
}

Plane homogeneous_magnitude_point_weight_expansion(HomogeneousMagnitude self, Point other) {
    return homogeneous_magnitude_plane_wedge(self, point_right_weight_dual(other));
}

Line homogeneous_magnitude_line_weight_expansion(HomogeneousMagnitude self, Line other) {
    return homogeneous_magnitude_line_wedge(self, line_right_weight_dual(other));
}

Point homogeneous_magnitude_plane_weight_expansion(HomogeneousMagnitude self, Plane other) {
    return homogeneous_magnitude_point_wedge(self, plane_right_weight_dual(other));
}

HomogeneousMagnitude homogeneous_magnitude_translator_weight_expansion(HomogeneousMagnitude self, Translator other) {
    return homogeneous_magnitude_scalar_wedge(self, translator_right_weight_dual(other));
}

Flector homogeneous_magnitude_flector_weight_expansion(HomogeneousMagnitude self, Flector other) {
    return homogeneous_magnitude_flector_wedge(self, flector_right_weight_dual(other));
}

MultiVector homogeneous_magnitude_multi_vector_weight_expansion(HomogeneousMagnitude self, MultiVector other) {
    return homogeneous_magnitude_multi_vector_wedge(self, multi_vector_right_weight_dual(other));
}

Point point_anti_scalar_weight_expansion(Point self, AntiScalar other) {
    return point_scalar_wedge(self, anti_scalar_right_weight_dual(other));
}

Point point_homogeneous_magnitude_weight_expansion(Point self, HomogeneousMagnitude other) {
    return point_scalar_wedge(self, homogeneous_magnitude_right_weight_dual(other));
}

AntiScalar point_point_weight_expansion(Point self, Point other) {
    return point_plane_wedge(self, point_right_weight_dual(other));
}

Plane point_line_weight_expansion(Point self, Line other) {
    return point_line_wedge(self, line_right_weight_dual(other));
}

Line point_plane_weight_expansion(Point self, Plane other) {
    return point_point_wedge(self, plane_right_weight_dual(other));
}

Point point_translator_weight_expansion(Point self, Translator other) {
    return point_scalar_wedge(self, translator_right_weight_dual(other));
}

Motor point_flector_weight_expansion(Point self, Flector other) {
    return point_flector_wedge(self, flector_right_weight_dual(other));
}

MultiVector point_multi_vector_weight_expansion(Point self, MultiVector other) {
    return point_multi_vector_wedge(self, multi_vector_right_weight_dual(other));
}

Line line_anti_scalar_weight_expansion(Line self, AntiScalar other) {
    return line_scalar_wedge(self, anti_scalar_right_weight_dual(other));
}

Line line_homogeneous_magnitude_weight_expansion(Line self, HomogeneousMagnitude other) {
    return line_scalar_wedge(self, homogeneous_magnitude_right_weight_dual(other));
}

AntiScalar line_line_weight_expansion(Line self, Line other) {
    return line_line_wedge(self, line_right_weight_dual(other));
}

Plane line_plane_weight_expansion(Line self, Plane other) {
    return line_point_wedge(self, plane_right_weight_dual(other));
}

Line line_translator_weight_expansion(Line self, Translator other) {
    return line_scalar_wedge(self, translator_right_weight_dual(other));
}

Plane line_flector_weight_expansion(Line self, Flector other) {
    return line_flector_wedge(self, flector_right_weight_dual(other));
}

MultiVector line_multi_vector_weight_expansion(Line self, MultiVector other) {
    return line_multi_vector_wedge(self, multi_vector_right_weight_dual(other));
}

Plane plane_anti_scalar_weight_expansion(Plane self, AntiScalar other) {
    return plane_scalar_wedge(self, anti_scalar_right_weight_dual(other));
}

Plane plane_homogeneous_magnitude_weight_expansion(Plane self, HomogeneousMagnitude other) {
    return plane_scalar_wedge(self, homogeneous_magnitude_right_weight_dual(other));
}

AntiScalar plane_plane_weight_expansion(Plane self, Plane other) {
    return plane_point_wedge(self, plane_right_weight_dual(other));
}

Plane plane_translator_weight_expansion(Plane self, Translator other) {
    return plane_scalar_wedge(self, translator_right_weight_dual(other));
}

AntiScalar plane_flector_weight_expansion(Plane self, Flector other) {
    return plane_flector_wedge(self, flector_right_weight_dual(other));
}

MultiVector plane_multi_vector_weight_expansion(Plane self, MultiVector other) {
    return plane_multi_vector_wedge(self, multi_vector_right_weight_dual(other));
}

Motor motor_anti_scalar_weight_expansion(Motor self, AntiScalar other) {
    return motor_scalar_wedge(self, anti_scalar_right_weight_dual(other));
}

Motor motor_homogeneous_magnitude_weight_expansion(Motor self, HomogeneousMagnitude other) {
    return motor_scalar_wedge(self, homogeneous_magnitude_right_weight_dual(other));
}

AntiScalar motor_line_weight_expansion(Motor self, Line other) {
    return motor_line_wedge(self, line_right_weight_dual(other));
}

Plane motor_plane_weight_expansion(Motor self, Plane other) {
    return motor_point_wedge(self, plane_right_weight_dual(other));
}

Motor motor_translator_weight_expansion(Motor self, Translator other) {
    return motor_scalar_wedge(self, translator_right_weight_dual(other));
}

Plane motor_flector_weight_expansion(Motor self, Flector other) {
    return motor_flector_wedge(self, flector_right_weight_dual(other));
}

MultiVector motor_multi_vector_weight_expansion(Motor self, MultiVector other) {
    return motor_multi_vector_wedge(self, multi_vector_right_weight_dual(other));
}

Rotor rotor_anti_scalar_weight_expansion(Rotor self, AntiScalar other) {
    return rotor_scalar_wedge(self, anti_scalar_right_weight_dual(other));
}

Rotor rotor_homogeneous_magnitude_weight_expansion(Rotor self, HomogeneousMagnitude other) {
    return rotor_scalar_wedge(self, homogeneous_magnitude_right_weight_dual(other));
}

AntiScalar rotor_line_weight_expansion(Rotor self, Line other) {
    return rotor_line_wedge(self, line_right_weight_dual(other));
}

Plane rotor_plane_weight_expansion(Rotor self, Plane other) {
    return rotor_point_wedge(self, plane_right_weight_dual(other));
}

Rotor rotor_translator_weight_expansion(Rotor self, Translator other) {
    return rotor_scalar_wedge(self, translator_right_weight_dual(other));
}

Plane rotor_flector_weight_expansion(Rotor self, Flector other) {
    return rotor_flector_wedge(self, flector_right_weight_dual(other));
}

MultiVector rotor_multi_vector_weight_expansion(Rotor self, MultiVector other) {
    return rotor_multi_vector_wedge(self, multi_vector_right_weight_dual(other));
}

Translator translator_anti_scalar_weight_expansion(Translator self, AntiScalar other) {
    return translator_scalar_wedge(self, anti_scalar_right_weight_dual(other));
}

Translator translator_homogeneous_magnitude_weight_expansion(Translator self, HomogeneousMagnitude other) {
    return translator_scalar_wedge(self, homogeneous_magnitude_right_weight_dual(other));
}

AntiScalar translator_line_weight_expansion(Translator self, Line other) {
    return translator_line_wedge(self, line_right_weight_dual(other));
}

Plane translator_plane_weight_expansion(Translator self, Plane other) {
    return translator_point_wedge(self, plane_right_weight_dual(other));
}

Translator translator_translator_weight_expansion(Translator self, Translator other) {
    return translator_scalar_wedge(self, translator_right_weight_dual(other));
}

Plane translator_flector_weight_expansion(Translator self, Flector other) {
    return translator_flector_wedge(self, flector_right_weight_dual(other));
}

MultiVector translator_multi_vector_weight_expansion(Translator self, MultiVector other) {
    return translator_multi_vector_wedge(self, multi_vector_right_weight_dual(other));
}

Flector flector_anti_scalar_weight_expansion(Flector self, AntiScalar other) {
    return flector_scalar_wedge(self, anti_scalar_right_weight_dual(other));
}

Flector flector_homogeneous_magnitude_weight_expansion(Flector self, HomogeneousMagnitude other) {
    return flector_scalar_wedge(self, homogeneous_magnitude_right_weight_dual(other));
}

AntiScalar flector_point_weight_expansion(Flector self, Point other) {
    return flector_plane_wedge(self, point_right_weight_dual(other));
}

Plane flector_line_weight_expansion(Flector self, Line other) {
    return flector_line_wedge(self, line_right_weight_dual(other));
}

Motor flector_plane_weight_expansion(Flector self, Plane other) {
    return flector_point_wedge(self, plane_right_weight_dual(other));
}

Flector flector_translator_weight_expansion(Flector self, Translator other) {
    return flector_scalar_wedge(self, translator_right_weight_dual(other));
}

Motor flector_flector_weight_expansion(Flector self, Flector other) {
    return flector_flector_wedge(self, flector_right_weight_dual(other));
}

MultiVector flector_multi_vector_weight_expansion(Flector self, MultiVector other) {
    return flector_multi_vector_wedge(self, multi_vector_right_weight_dual(other));
}

AntiScalar multi_vector_scalar_weight_expansion(MultiVector self, Scalar other) {
    return multi_vector_anti_scalar_wedge(self, scalar_right_weight_dual(other));
}

MultiVector multi_vector_anti_scalar_weight_expansion(MultiVector self, AntiScalar other) {
    return multi_vector_scalar_wedge(self, anti_scalar_right_weight_dual(other));
}

MultiVector multi_vector_homogeneous_magnitude_weight_expansion(MultiVector self, HomogeneousMagnitude other) {
    return multi_vector_scalar_wedge(self, homogeneous_magnitude_right_weight_dual(other));
}

MultiVector multi_vector_point_weight_expansion(MultiVector self, Point other) {
    return multi_vector_plane_wedge(self, point_right_weight_dual(other));
}

MultiVector multi_vector_line_weight_expansion(MultiVector self, Line other) {
    return multi_vector_line_wedge(self, line_right_weight_dual(other));
}

MultiVector multi_vector_plane_weight_expansion(MultiVector self, Plane other) {
    return multi_vector_point_wedge(self, plane_right_weight_dual(other));
}

MultiVector multi_vector_translator_weight_expansion(MultiVector self, Translator other) {
    return multi_vector_scalar_wedge(self, translator_right_weight_dual(other));
}

MultiVector multi_vector_flector_weight_expansion(MultiVector self, Flector other) {
    return multi_vector_flector_wedge(self, flector_right_weight_dual(other));
}

MultiVector multi_vector_multi_vector_weight_expansion(MultiVector self, MultiVector other) {
    return multi_vector_multi_vector_wedge(self, multi_vector_right_weight_dual(other));
}

Scalar scalar_scalar_project_onto(Scalar self, Scalar other) {
    return scalar_anti_scalar_anti_wedge(other, scalar_scalar_weight_expansion(self, other));
}

Scalar scalar_anti_scalar_project_onto(Scalar self, AntiScalar other) {
    return anti_scalar_scalar_anti_wedge(other, scalar_anti_scalar_weight_expansion(self, other));
}

Scalar scalar_homogeneous_magnitude_project_onto(Scalar self, HomogeneousMagnitude other) {
    return homogeneous_magnitude_scalar_anti_wedge(other, scalar_homogeneous_magnitude_weight_expansion(self, other));
}

Scalar scalar_point_project_onto(Scalar self, Point other) {
    return point_plane_anti_wedge(other, scalar_point_weight_expansion(self, other));
}

Scalar scalar_line_project_onto(Scalar self, Line other) {
    return line_line_anti_wedge(other, scalar_line_weight_expansion(self, other));
}

Scalar scalar_plane_project_onto(Scalar self, Plane other) {
    return plane_point_anti_wedge(other, scalar_plane_weight_expansion(self, other));
}

Scalar scalar_translator_project_onto(Scalar self, Translator other) {
    return translator_scalar_anti_wedge(other, scalar_translator_weight_expansion(self, other));
}

MultiVector scalar_flector_project_onto(Scalar self, Flector other) {
    return flector_flector_anti_wedge(other, scalar_flector_weight_expansion(self, other));
}

MultiVector scalar_multi_vector_project_onto(Scalar self, MultiVector other) {
    return multi_vector_multi_vector_anti_wedge(other, scalar_multi_vector_weight_expansion(self, other));
}

AntiScalar anti_scalar_anti_scalar_project_onto(AntiScalar self, AntiScalar other) {
    return anti_scalar_anti_scalar_anti_wedge(other, anti_scalar_anti_scalar_weight_expansion(self, other));
}

HomogeneousMagnitude anti_scalar_homogeneous_magnitude_project_onto(AntiScalar self, HomogeneousMagnitude other) {
    return homogeneous_magnitude_anti_scalar_anti_wedge(other, anti_scalar_homogeneous_magnitude_weight_expansion(self, other));
}

Translator anti_scalar_translator_project_onto(AntiScalar self, Translator other) {
    return translator_anti_scalar_anti_wedge(other, anti_scalar_translator_weight_expansion(self, other));
}

MultiVector anti_scalar_multi_vector_project_onto(AntiScalar self, MultiVector other) {
    return multi_vector_anti_scalar_anti_wedge(other, anti_scalar_multi_vector_weight_expansion(self, other));
}

Scalar homogeneous_magnitude_scalar_project_onto(HomogeneousMagnitude self, Scalar other) {
    return scalar_anti_scalar_anti_wedge(other, homogeneous_magnitude_scalar_weight_expansion(self, other));
}

HomogeneousMagnitude homogeneous_magnitude_anti_scalar_project_onto(HomogeneousMagnitude self, AntiScalar other) {
    return anti_scalar_homogeneous_magnitude_anti_wedge(other, homogeneous_magnitude_anti_scalar_weight_expansion(self, other));
}

HomogeneousMagnitude homogeneous_magnitude_homogeneous_magnitude_project_onto(HomogeneousMagnitude self, HomogeneousMagnitude other) {
    return homogeneous_magnitude_homogeneous_magnitude_anti_wedge(other, homogeneous_magnitude_homogeneous_magnitude_weight_expansion(self, other));
}

Scalar homogeneous_magnitude_point_project_onto(HomogeneousMagnitude self, Point other) {
    return point_plane_anti_wedge(other, homogeneous_magnitude_point_weight_expansion(self, other));
}

Scalar homogeneous_magnitude_line_project_onto(HomogeneousMagnitude self, Line other) {
    return line_line_anti_wedge(other, homogeneous_magnitude_line_weight_expansion(self, other));
}

Scalar homogeneous_magnitude_plane_project_onto(HomogeneousMagnitude self, Plane other) {
    return plane_point_anti_wedge(other, homogeneous_magnitude_plane_weight_expansion(self, other));
}

MultiVector homogeneous_magnitude_translator_project_onto(HomogeneousMagnitude self, Translator other) {
    return translator_homogeneous_magnitude_anti_wedge(other, homogeneous_magnitude_translator_weight_expansion(self, other));
}

MultiVector homogeneous_magnitude_flector_project_onto(HomogeneousMagnitude self, Flector other) {
    return flector_flector_anti_wedge(other, homogeneous_magnitude_flector_weight_expansion(self, other));
}

MultiVector homogeneous_magnitude_multi_vector_project_onto(HomogeneousMagnitude self, MultiVector other) {
    return multi_vector_multi_vector_anti_wedge(other, homogeneous_magnitude_multi_vector_weight_expansion(self, other));
}

Point point_anti_scalar_project_onto(Point self, AntiScalar other) {
    return anti_scalar_point_anti_wedge(other, point_anti_scalar_weight_expansion(self, other));
}

Point point_homogeneous_magnitude_project_onto(Point self, HomogeneousMagnitude other) {
    return homogeneous_magnitude_point_anti_wedge(other, point_homogeneous_magnitude_weight_expansion(self, other));
}

Point point_point_project_onto(Point self, Point other) {
    return point_anti_scalar_anti_wedge(other, point_point_weight_expansion(self, other));
}

Point point_line_project_onto(Point self, Line other) {
    return line_plane_anti_wedge(other, point_line_weight_expansion(self, other));
}

Point point_plane_project_onto(Point self, Plane other) {
    return plane_line_anti_wedge(other, point_plane_weight_expansion(self, other));
}

Point point_translator_project_onto(Point self, Translator other) {
    return translator_point_anti_wedge(other, point_translator_weight_expansion(self, other));
}

Flector point_flector_project_onto(Point self, Flector other) {
    return flector_motor_anti_wedge(other, point_flector_weight_expansion(self, other));
}

MultiVector point_multi_vector_project_onto(Point self, MultiVector other) {
    return multi_vector_multi_vector_anti_wedge(other, point_multi_vector_weight_expansion(self, other));
}

Line line_anti_scalar_project_onto(Line self, AntiScalar other) {
    return anti_scalar_line_anti_wedge(other, line_anti_scalar_weight_expansion(self, other));
}

Line line_homogeneous_magnitude_project_onto(Line self, HomogeneousMagnitude other) {
    return homogeneous_magnitude_line_anti_wedge(other, line_homogeneous_magnitude_weight_expansion(self, other));
}

Line line_line_project_onto(Line self, Line other) {
    return line_anti_scalar_anti_wedge(other, line_line_weight_expansion(self, other));
}

Line line_plane_project_onto(Line self, Plane other) {
    return plane_plane_anti_wedge(other, line_plane_weight_expansion(self, other));
}

MultiVector line_translator_project_onto(Line self, Translator other) {
    return translator_line_anti_wedge(other, line_translator_weight_expansion(self, other));
}

MultiVector line_flector_project_onto(Line self, Flector other) {
    return flector_plane_anti_wedge(other, line_flector_weight_expansion(self, other));
}

MultiVector line_multi_vector_project_onto(Line self, MultiVector other) {
    return multi_vector_multi_vector_anti_wedge(other, line_multi_vector_weight_expansion(self, other));
}

Plane plane_anti_scalar_project_onto(Plane self, AntiScalar other) {
    return anti_scalar_plane_anti_wedge(other, plane_anti_scalar_weight_expansion(self, other));
}

Plane plane_homogeneous_magnitude_project_onto(Plane self, HomogeneousMagnitude other) {
    return homogeneous_magnitude_plane_anti_wedge(other, plane_homogeneous_magnitude_weight_expansion(self, other));
}

Plane plane_plane_project_onto(Plane self, Plane other) {
    return plane_anti_scalar_anti_wedge(other, plane_plane_weight_expansion(self, other));
}

Flector plane_translator_project_onto(Plane self, Translator other) {
    return translator_plane_anti_wedge(other, plane_translator_weight_expansion(self, other));
}

Flector plane_flector_project_onto(Plane self, Flector other) {
    return flector_anti_scalar_anti_wedge(other, plane_flector_weight_expansion(self, other));
}

MultiVector plane_multi_vector_project_onto(Plane self, MultiVector other) {
    return multi_vector_multi_vector_anti_wedge(other, plane_multi_vector_weight_expansion(self, other));
}

Motor motor_anti_scalar_project_onto(Motor self, AntiScalar other) {
    return anti_scalar_motor_anti_wedge(other, motor_anti_scalar_weight_expansion(self, other));
}

MultiVector motor_homogeneous_magnitude_project_onto(Motor self, HomogeneousMagnitude other) {
    return homogeneous_magnitude_motor_anti_wedge(other, motor_homogeneous_magnitude_weight_expansion(self, other));
}

Line motor_line_project_onto(Motor self, Line other) {
    return line_anti_scalar_anti_wedge(other, motor_line_weight_expansion(self, other));
}

Line motor_plane_project_onto(Motor self, Plane other) {
    return plane_plane_anti_wedge(other, motor_plane_weight_expansion(self, other));
}

MultiVector motor_translator_project_onto(Motor self, Translator other) {
    return translator_motor_anti_wedge(other, motor_translator_weight_expansion(self, other));
}

MultiVector motor_flector_project_onto(Motor self, Flector other) {
    return flector_plane_anti_wedge(other, motor_flector_weight_expansion(self, other));
}

MultiVector motor_multi_vector_project_onto(Motor self, MultiVector other) {
    return multi_vector_multi_vector_anti_wedge(other, motor_multi_vector_weight_expansion(self, other));
}

Rotor rotor_anti_scalar_project_onto(Rotor self, AntiScalar other) {
    return anti_scalar_rotor_anti_wedge(other, rotor_anti_scalar_weight_expansion(self, other));
}

MultiVector rotor_homogeneous_magnitude_project_onto(Rotor self, HomogeneousMagnitude other) {
    return homogeneous_magnitude_rotor_anti_wedge(other, rotor_homogeneous_magnitude_weight_expansion(self, other));
}

Line rotor_line_project_onto(Rotor self, Line other) {
    return line_anti_scalar_anti_wedge(other, rotor_line_weight_expansion(self, other));
}

Line rotor_plane_project_onto(Rotor self, Plane other) {
    return plane_plane_anti_wedge(other, rotor_plane_weight_expansion(self, other));
}

MultiVector rotor_translator_project_onto(Rotor self, Translator other) {
    return translator_rotor_anti_wedge(other, rotor_translator_weight_expansion(self, other));
}

MultiVector rotor_flector_project_onto(Rotor self, Flector other) {
    return flector_plane_anti_wedge(other, rotor_flector_weight_expansion(self, other));
}

MultiVector rotor_multi_vector_project_onto(Rotor self, MultiVector other) {
    return multi_vector_multi_vector_anti_wedge(other, rotor_multi_vector_weight_expansion(self, other));
}

Translator translator_anti_scalar_project_onto(Translator self, AntiScalar other) {
    return anti_scalar_translator_anti_wedge(other, translator_anti_scalar_weight_expansion(self, other));
}

MultiVector translator_homogeneous_magnitude_project_onto(Translator self, HomogeneousMagnitude other) {
    return homogeneous_magnitude_translator_anti_wedge(other, translator_homogeneous_magnitude_weight_expansion(self, other));
}

Line translator_line_project_onto(Translator self, Line other) {
    return line_anti_scalar_anti_wedge(other, translator_line_weight_expansion(self, other));
}

Line translator_plane_project_onto(Translator self, Plane other) {
    return plane_plane_anti_wedge(other, translator_plane_weight_expansion(self, other));
}

Translator translator_translator_project_onto(Translator self, Translator other) {
    return translator_translator_anti_wedge(other, translator_translator_weight_expansion(self, other));
}

MultiVector translator_flector_project_onto(Translator self, Flector other) {
    return flector_plane_anti_wedge(other, translator_flector_weight_expansion(self, other));
}

MultiVector translator_multi_vector_project_onto(Translator self, MultiVector other) {
    return multi_vector_multi_vector_anti_wedge(other, translator_multi_vector_weight_expansion(self, other));
}

Flector flector_anti_scalar_project_onto(Flector self, AntiScalar other) {
    return anti_scalar_flector_anti_wedge(other, flector_anti_scalar_weight_expansion(self, other));
}

Flector flector_homogeneous_magnitude_project_onto(Flector self, HomogeneousMagnitude other) {
    return homogeneous_magnitude_flector_anti_wedge(other, flector_homogeneous_magnitude_weight_expansion(self, other));
}

Point flector_point_project_onto(Flector self, Point other) {
    return point_anti_scalar_anti_wedge(other, flector_point_weight_expansion(self, other));
}

Point flector_line_project_onto(Flector self, Line other) {
    return line_plane_anti_wedge(other, flector_line_weight_expansion(self, other));
}

Flector flector_plane_project_onto(Flector self, Plane other) {
    return plane_motor_anti_wedge(other, flector_plane_weight_expansion(self, other));
}

Flector flector_translator_project_onto(Flector self, Translator other) {
    return translator_flector_anti_wedge(other, flector_translator_weight_expansion(self, other));
}

Flector flector_flector_project_onto(Flector self, Flector other) {
    return flector_motor_anti_wedge(other, flector_flector_weight_expansion(self, other));
}

MultiVector flector_multi_vector_project_onto(Flector self, MultiVector other) {
    return multi_vector_multi_vector_anti_wedge(other, flector_multi_vector_weight_expansion(self, other));
}

Scalar multi_vector_scalar_project_onto(MultiVector self, Scalar other) {
    return scalar_anti_scalar_anti_wedge(other, multi_vector_scalar_weight_expansion(self, other));
}

MultiVector multi_vector_anti_scalar_project_onto(MultiVector self, AntiScalar other) {
    return anti_scalar_multi_vector_anti_wedge(other, multi_vector_anti_scalar_weight_expansion(self, other));
}

MultiVector multi_vector_homogeneous_magnitude_project_onto(MultiVector self, HomogeneousMagnitude other) {
    return homogeneous_magnitude_multi_vector_anti_wedge(other, multi_vector_homogeneous_magnitude_weight_expansion(self, other));
}

MultiVector multi_vector_point_project_onto(MultiVector self, Point other) {
    return point_multi_vector_anti_wedge(other, multi_vector_point_weight_expansion(self, other));
}

MultiVector multi_vector_line_project_onto(MultiVector self, Line other) {
    return line_multi_vector_anti_wedge(other, multi_vector_line_weight_expansion(self, other));
}

MultiVector multi_vector_plane_project_onto(MultiVector self, Plane other) {
    return plane_multi_vector_anti_wedge(other, multi_vector_plane_weight_expansion(self, other));
}

MultiVector multi_vector_translator_project_onto(MultiVector self, Translator other) {
    return translator_multi_vector_anti_wedge(other, multi_vector_translator_weight_expansion(self, other));
}

MultiVector multi_vector_flector_project_onto(MultiVector self, Flector other) {
    return flector_multi_vector_anti_wedge(other, multi_vector_flector_weight_expansion(self, other));
}

MultiVector multi_vector_multi_vector_project_onto(MultiVector self, MultiVector other) {
    return multi_vector_multi_vector_anti_wedge(other, multi_vector_multi_vector_weight_expansion(self, other));
}

Scalar scalar_scalar_anti_project_onto(Scalar self, Scalar other) {
    return scalar_scalar_wedge(other, scalar_scalar_weight_contraction(self, other));
}

MultiVector scalar_multi_vector_anti_project_onto(Scalar self, MultiVector other) {
    return multi_vector_scalar_wedge(other, scalar_multi_vector_weight_contraction(self, other));
}

AntiScalar anti_scalar_scalar_anti_project_onto(AntiScalar self, Scalar other) {
    return scalar_anti_scalar_wedge(other, anti_scalar_scalar_weight_contraction(self, other));
}

AntiScalar anti_scalar_anti_scalar_anti_project_onto(AntiScalar self, AntiScalar other) {
    return anti_scalar_scalar_wedge(other, anti_scalar_anti_scalar_weight_contraction(self, other));
}

HomogeneousMagnitude anti_scalar_homogeneous_magnitude_anti_project_onto(AntiScalar self, HomogeneousMagnitude other) {
    return homogeneous_magnitude_scalar_wedge(other, anti_scalar_homogeneous_magnitude_weight_contraction(self, other));
}

AntiScalar anti_scalar_point_anti_project_onto(AntiScalar self, Point other) {
    return point_plane_wedge(other, anti_scalar_point_weight_contraction(self, other));
}

AntiScalar anti_scalar_line_anti_project_onto(AntiScalar self, Line other) {
    return line_line_wedge(other, anti_scalar_line_weight_contraction(self, other));
}

AntiScalar anti_scalar_plane_anti_project_onto(AntiScalar self, Plane other) {
    return plane_point_wedge(other, anti_scalar_plane_weight_contraction(self, other));
}

Translator anti_scalar_translator_anti_project_onto(AntiScalar self, Translator other) {
    return translator_scalar_wedge(other, anti_scalar_translator_weight_contraction(self, other));
}

Motor anti_scalar_flector_anti_project_onto(AntiScalar self, Flector other) {
    return flector_flector_wedge(other, anti_scalar_flector_weight_contraction(self, other));
}

MultiVector anti_scalar_multi_vector_anti_project_onto(AntiScalar self, MultiVector other) {
    return multi_vector_multi_vector_wedge(other, anti_scalar_multi_vector_weight_contraction(self, other));
}

HomogeneousMagnitude homogeneous_magnitude_scalar_anti_project_onto(HomogeneousMagnitude self, Scalar other) {
    return scalar_homogeneous_magnitude_wedge(other, homogeneous_magnitude_scalar_weight_contraction(self, other));
}

AntiScalar homogeneous_magnitude_anti_scalar_anti_project_onto(HomogeneousMagnitude self, AntiScalar other) {
    return anti_scalar_scalar_wedge(other, homogeneous_magnitude_anti_scalar_weight_contraction(self, other));
}

HomogeneousMagnitude homogeneous_magnitude_homogeneous_magnitude_anti_project_onto(HomogeneousMagnitude self, HomogeneousMagnitude other) {
    return homogeneous_magnitude_scalar_wedge(other, homogeneous_magnitude_homogeneous_magnitude_weight_contraction(self, other));
}

AntiScalar homogeneous_magnitude_point_anti_project_onto(HomogeneousMagnitude self, Point other) {
    return point_plane_wedge(other, homogeneous_magnitude_point_weight_contraction(self, other));
}

AntiScalar homogeneous_magnitude_line_anti_project_onto(HomogeneousMagnitude self, Line other) {
    return line_line_wedge(other, homogeneous_magnitude_line_weight_contraction(self, other));
}

AntiScalar homogeneous_magnitude_plane_anti_project_onto(HomogeneousMagnitude self, Plane other) {
    return plane_point_wedge(other, homogeneous_magnitude_plane_weight_contraction(self, other));
}

Translator homogeneous_magnitude_translator_anti_project_onto(HomogeneousMagnitude self, Translator other) {
    return translator_scalar_wedge(other, homogeneous_magnitude_translator_weight_contraction(self, other));
}

Motor homogeneous_magnitude_flector_anti_project_onto(HomogeneousMagnitude self, Flector other) {
    return flector_flector_wedge(other, homogeneous_magnitude_flector_weight_contraction(self, other));
}

MultiVector homogeneous_magnitude_multi_vector_anti_project_onto(HomogeneousMagnitude self, MultiVector other) {
    return multi_vector_multi_vector_wedge(other, homogeneous_magnitude_multi_vector_weight_contraction(self, other));
}

Point point_scalar_anti_project_onto(Point self, Scalar other) {
    return scalar_point_wedge(other, point_scalar_weight_contraction(self, other));
}

Point point_point_anti_project_onto(Point self, Point other) {
    return point_scalar_wedge(other, point_point_weight_contraction(self, other));
}

Flector point_flector_anti_project_onto(Point self, Flector other) {
    return flector_scalar_wedge(other, point_flector_weight_contraction(self, other));
}

MultiVector point_multi_vector_anti_project_onto(Point self, MultiVector other) {
    return multi_vector_multi_vector_wedge(other, point_multi_vector_weight_contraction(self, other));
}

Line line_scalar_anti_project_onto(Line self, Scalar other) {
    return scalar_line_wedge(other, line_scalar_weight_contraction(self, other));
}

Line line_point_anti_project_onto(Line self, Point other) {
    return point_point_wedge(other, line_point_weight_contraction(self, other));
}

Line line_line_anti_project_onto(Line self, Line other) {
    return line_scalar_wedge(other, line_line_weight_contraction(self, other));
}

Motor line_flector_anti_project_onto(Line self, Flector other) {
    return flector_point_wedge(other, line_flector_weight_contraction(self, other));
}

MultiVector line_multi_vector_anti_project_onto(Line self, MultiVector other) {
    return multi_vector_multi_vector_wedge(other, line_multi_vector_weight_contraction(self, other));
}

Plane plane_scalar_anti_project_onto(Plane self, Scalar other) {
    return scalar_plane_wedge(other, plane_scalar_weight_contraction(self, other));
}

Plane plane_point_anti_project_onto(Plane self, Point other) {
    return point_line_wedge(other, plane_point_weight_contraction(self, other));
}

Plane plane_line_anti_project_onto(Plane self, Line other) {
    return line_point_wedge(other, plane_line_weight_contraction(self, other));
}

Plane plane_plane_anti_project_onto(Plane self, Plane other) {
    return plane_scalar_wedge(other, plane_plane_weight_contraction(self, other));
}

MultiVector plane_flector_anti_project_onto(Plane self, Flector other) {
    return flector_multi_vector_wedge(other, plane_flector_weight_contraction(self, other));
}

MultiVector plane_multi_vector_anti_project_onto(Plane self, MultiVector other) {
    return multi_vector_multi_vector_wedge(other, plane_multi_vector_weight_contraction(self, other));
}

Motor motor_scalar_anti_project_onto(Motor self, Scalar other) {
    return scalar_motor_wedge(other, motor_scalar_weight_contraction(self, other));
}

AntiScalar motor_anti_scalar_anti_project_onto(Motor self, AntiScalar other) {
    return anti_scalar_scalar_wedge(other, motor_anti_scalar_weight_contraction(self, other));
}

HomogeneousMagnitude motor_homogeneous_magnitude_anti_project_onto(Motor self, HomogeneousMagnitude other) {
    return homogeneous_magnitude_scalar_wedge(other, motor_homogeneous_magnitude_weight_contraction(self, other));
}

Motor motor_point_anti_project_onto(Motor self, Point other) {
    return point_flector_wedge(other, motor_point_weight_contraction(self, other));
}

MultiVector motor_line_anti_project_onto(Motor self, Line other) {
    return line_multi_vector_wedge(other, motor_line_weight_contraction(self, other));
}

AntiScalar motor_plane_anti_project_onto(Motor self, Plane other) {
    return plane_point_wedge(other, motor_plane_weight_contraction(self, other));
}

Translator motor_translator_anti_project_onto(Motor self, Translator other) {
    return translator_scalar_wedge(other, motor_translator_weight_contraction(self, other));
}

Motor motor_flector_anti_project_onto(Motor self, Flector other) {
    return flector_flector_wedge(other, motor_flector_weight_contraction(self, other));
}

MultiVector motor_multi_vector_anti_project_onto(Motor self, MultiVector other) {
    return multi_vector_multi_vector_wedge(other, motor_multi_vector_weight_contraction(self, other));
}

Rotor rotor_scalar_anti_project_onto(Rotor self, Scalar other) {
    return scalar_rotor_wedge(other, rotor_scalar_weight_contraction(self, other));
}

AntiScalar rotor_anti_scalar_anti_project_onto(Rotor self, AntiScalar other) {
    return anti_scalar_scalar_wedge(other, rotor_anti_scalar_weight_contraction(self, other));
}

HomogeneousMagnitude rotor_homogeneous_magnitude_anti_project_onto(Rotor self, HomogeneousMagnitude other) {
    return homogeneous_magnitude_scalar_wedge(other, rotor_homogeneous_magnitude_weight_contraction(self, other));
}

Motor rotor_point_anti_project_onto(Rotor self, Point other) {
    return point_flector_wedge(other, rotor_point_weight_contraction(self, other));
}

MultiVector rotor_line_anti_project_onto(Rotor self, Line other) {
    return line_multi_vector_wedge(other, rotor_line_weight_contraction(self, other));
}

AntiScalar rotor_plane_anti_project_onto(Rotor self, Plane other) {
    return plane_point_wedge(other, rotor_plane_weight_contraction(self, other));
}

Translator rotor_translator_anti_project_onto(Rotor self, Translator other) {
    return translator_scalar_wedge(other, rotor_translator_weight_contraction(self, other));
}

Motor rotor_flector_anti_project_onto(Rotor self, Flector other) {
    return flector_flector_wedge(other, rotor_flector_weight_contraction(self, other));
}

MultiVector rotor_multi_vector_anti_project_onto(Rotor self, MultiVector other) {
    return multi_vector_multi_vector_wedge(other, rotor_multi_vector_weight_contraction(self, other));
}

Translator translator_scalar_anti_project_onto(Translator self, Scalar other) {
    return scalar_translator_wedge(other, translator_scalar_weight_contraction(self, other));
}

AntiScalar translator_anti_scalar_anti_project_onto(Translator self, AntiScalar other) {
    return anti_scalar_scalar_wedge(other, translator_anti_scalar_weight_contraction(self, other));
}

HomogeneousMagnitude translator_homogeneous_magnitude_anti_project_onto(Translator self, HomogeneousMagnitude other) {
    return homogeneous_magnitude_scalar_wedge(other, translator_homogeneous_magnitude_weight_contraction(self, other));
}

Motor translator_point_anti_project_onto(Translator self, Point other) {
    return point_flector_wedge(other, translator_point_weight_contraction(self, other));
}

MultiVector translator_line_anti_project_onto(Translator self, Line other) {
    return line_multi_vector_wedge(other, translator_line_weight_contraction(self, other));
}

AntiScalar translator_plane_anti_project_onto(Translator self, Plane other) {
    return plane_point_wedge(other, translator_plane_weight_contraction(self, other));
}

Translator translator_translator_anti_project_onto(Translator self, Translator other) {
    return translator_scalar_wedge(other, translator_translator_weight_contraction(self, other));
}

Motor translator_flector_anti_project_onto(Translator self, Flector other) {
    return flector_flector_wedge(other, translator_flector_weight_contraction(self, other));
}

MultiVector translator_multi_vector_anti_project_onto(Translator self, MultiVector other) {
    return multi_vector_multi_vector_wedge(other, translator_multi_vector_weight_contraction(self, other));
}

Flector flector_scalar_anti_project_onto(Flector self, Scalar other) {
    return scalar_flector_wedge(other, flector_scalar_weight_contraction(self, other));
}

MultiVector flector_point_anti_project_onto(Flector self, Point other) {
    return point_multi_vector_wedge(other, flector_point_weight_contraction(self, other));
}

Plane flector_line_anti_project_onto(Flector self, Line other) {
    return line_point_wedge(other, flector_line_weight_contraction(self, other));
}

Plane flector_plane_anti_project_onto(Flector self, Plane other) {
    return plane_scalar_wedge(other, flector_plane_weight_contraction(self, other));
}

MultiVector flector_flector_anti_project_onto(Flector self, Flector other) {
    return flector_multi_vector_wedge(other, flector_flector_weight_contraction(self, other));
}

MultiVector flector_multi_vector_anti_project_onto(Flector self, MultiVector other) {
    return multi_vector_multi_vector_wedge(other, flector_multi_vector_weight_contraction(self, other));
}

MultiVector multi_vector_scalar_anti_project_onto(MultiVector self, Scalar other) {
    return scalar_multi_vector_wedge(other, multi_vector_scalar_weight_contraction(self, other));
}

AntiScalar multi_vector_anti_scalar_anti_project_onto(MultiVector self, AntiScalar other) {
    return anti_scalar_scalar_wedge(other, multi_vector_anti_scalar_weight_contraction(self, other));
}

HomogeneousMagnitude multi_vector_homogeneous_magnitude_anti_project_onto(MultiVector self, HomogeneousMagnitude other) {
    return homogeneous_magnitude_scalar_wedge(other, multi_vector_homogeneous_magnitude_weight_contraction(self, other));
}

MultiVector multi_vector_point_anti_project_onto(MultiVector self, Point other) {
    return point_multi_vector_wedge(other, multi_vector_point_weight_contraction(self, other));
}

MultiVector multi_vector_line_anti_project_onto(MultiVector self, Line other) {
    return line_multi_vector_wedge(other, multi_vector_line_weight_contraction(self, other));
}

MultiVector multi_vector_plane_anti_project_onto(MultiVector self, Plane other) {
    return plane_multi_vector_wedge(other, multi_vector_plane_weight_contraction(self, other));
}

Translator multi_vector_translator_anti_project_onto(MultiVector self, Translator other) {
    return translator_scalar_wedge(other, multi_vector_translator_weight_contraction(self, other));
}

MultiVector multi_vector_flector_anti_project_onto(MultiVector self, Flector other) {
    return flector_multi_vector_wedge(other, multi_vector_flector_weight_contraction(self, other));
}

MultiVector multi_vector_multi_vector_anti_project_onto(MultiVector self, MultiVector other) {
    return multi_vector_multi_vector_wedge(other, multi_vector_multi_vector_weight_contraction(self, other));
}


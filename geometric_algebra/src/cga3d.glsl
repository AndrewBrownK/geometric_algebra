struct Scalar {
    // 1
    float g0;
};

struct AntiScalar {
    // e01234
    float g0;
};

struct HomogeneousMagnitude {
    // 1, e01234
    vec2 g0;
};

struct RadialPoint {
    // e0, e1, e2
    vec3 g0;
    // e3, e4
    vec2 g1;
};

struct FlatPoint {
    // e04, e14, e24, e34
    vec4 g0;
};

struct Dipole {
    // -e03, -e13, -e23
    vec3 g0;
    // e12, -e02, e01
    vec3 g1;
    // e04, e14, e24, e34
    vec4 g2;
};

struct Line {
    // -e034, -e134, -e234
    vec3 g0;
    // e124, -e024, e014
    vec3 g1;
};

struct Circle {
    // e123, -e023, e013, -e012
    vec4 g0;
    // -e034, -e134, -e234
    vec3 g1;
    // e124, -e024, e014
    vec3 g2;
};

struct Plane {
    // e1234, -e0234, e0134, -e0124
    vec4 g0;
};

struct Sphere {
    // e0123, -e0124
    vec2 g0;
    // e1234, -e0234, e0134
    vec3 g1;
};

struct Motor {
    // -e034, -e134, -e234, e01234
    vec4 g0;
    // e124, -e024, e014, e4
    vec4 g1;
};

struct Rotor {
    // -e034, -e134, -e234, e01234
    vec4 g0;
};

struct Translator {
    // e124, -e024, e014, e01234
    vec4 g0;
};

struct Flector {
    // e04, e14, e24, e34
    vec4 g0;
    // e1234, -e0234, e0134, -e0124
    vec4 g1;
};

struct Dilation {
    // e023, -e013, e012
    vec3 g0;
    // -e123, e01234
    vec2 g1;
};

struct MultiVector {
    // 1, e0123, e01234
    vec3 g0;
    // e0, e1, e2
    vec3 g1;
    // e3, e4
    vec2 g2;
    // e04, e14, e24, e34
    vec4 g3;
    // -e03, -e13, -e23
    vec3 g4;
    // e12, -e02, e01
    vec3 g5;
    // -e034, -e134, -e234
    vec3 g6;
    // e124, -e024, e014
    vec3 g7;
    // e123, -e023, e013, -e012
    vec4 g8;
    // e1234, -e0234, e0134, -e0124
    vec4 g9;
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
    return 5;
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

HomogeneousMagnitude scalar_anti_scalar_add(Scalar self, AntiScalar other) {
    return HomogeneousMagnitude(vec2(self.g0) * vec2(1.0, 0.0) + vec2(other.g0) * vec2(0.0, 1.0));
}

HomogeneousMagnitude scalar_anti_scalar_sub(Scalar self, AntiScalar other) {
    return HomogeneousMagnitude(vec2(self.g0) * vec2(1.0, 0.0) - vec2(other.g0) * vec2(0.0, 1.0));
}

AntiScalar scalar_anti_scalar_geometric_product(Scalar self, AntiScalar other) {
    return AntiScalar(self.g0 * other.g0);
}

Scalar scalar_anti_scalar_regressive_product(Scalar self, AntiScalar other) {
    return Scalar(self.g0 * other.g0);
}

AntiScalar scalar_anti_scalar_outer_product(Scalar self, AntiScalar other) {
    return AntiScalar(self.g0 * other.g0);
}

AntiScalar scalar_anti_scalar_inner_product(Scalar self, AntiScalar other) {
    return AntiScalar(self.g0 * other.g0);
}

Scalar scalar_anti_scalar_geometric_anti_product(Scalar self, AntiScalar other) {
    return Scalar(self.g0 * other.g0);
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

Scalar scalar_homogeneous_magnitude_regressive_product(Scalar self, HomogeneousMagnitude other) {
    return Scalar(self.g0 * other.g0.y);
}

HomogeneousMagnitude scalar_homogeneous_magnitude_outer_product(Scalar self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(vec2(self.g0) * other.g0);
}

HomogeneousMagnitude scalar_homogeneous_magnitude_inner_product(Scalar self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(vec2(self.g0) * other.g0);
}

Scalar scalar_homogeneous_magnitude_geometric_anti_product(Scalar self, HomogeneousMagnitude other) {
    return Scalar(self.g0 * other.g0.y);
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

RadialPoint scalar_radial_point_geometric_product(Scalar self, RadialPoint other) {
    return RadialPoint(vec3(self.g0) * other.g0, vec2(self.g0) * other.g1);
}

RadialPoint scalar_radial_point_outer_product(Scalar self, RadialPoint other) {
    return RadialPoint(vec3(self.g0) * other.g0, vec2(self.g0) * other.g1);
}

RadialPoint scalar_radial_point_inner_product(Scalar self, RadialPoint other) {
    return RadialPoint(vec3(self.g0) * other.g0, vec2(self.g0) * other.g1);
}

RadialPoint scalar_radial_point_left_contraction(Scalar self, RadialPoint other) {
    return RadialPoint(vec3(self.g0) * other.g0, vec2(self.g0) * other.g1);
}

FlatPoint scalar_flat_point_geometric_product(Scalar self, FlatPoint other) {
    return FlatPoint(vec4(self.g0) * other.g0);
}

FlatPoint scalar_flat_point_outer_product(Scalar self, FlatPoint other) {
    return FlatPoint(vec4(self.g0) * other.g0);
}

FlatPoint scalar_flat_point_inner_product(Scalar self, FlatPoint other) {
    return FlatPoint(vec4(self.g0) * other.g0);
}

FlatPoint scalar_flat_point_left_contraction(Scalar self, FlatPoint other) {
    return FlatPoint(vec4(self.g0) * other.g0);
}

Dipole scalar_dipole_geometric_product(Scalar self, Dipole other) {
    return Dipole(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1, vec4(self.g0) * other.g2);
}

Dipole scalar_dipole_outer_product(Scalar self, Dipole other) {
    return Dipole(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1, vec4(self.g0) * other.g2);
}

Dipole scalar_dipole_inner_product(Scalar self, Dipole other) {
    return Dipole(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1, vec4(self.g0) * other.g2);
}

Dipole scalar_dipole_left_contraction(Scalar self, Dipole other) {
    return Dipole(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1, vec4(self.g0) * other.g2);
}

Line scalar_line_geometric_product(Scalar self, Line other) {
    return Line(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Line scalar_line_outer_product(Scalar self, Line other) {
    return Line(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Line scalar_line_inner_product(Scalar self, Line other) {
    return Line(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Line scalar_line_left_contraction(Scalar self, Line other) {
    return Line(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Circle scalar_circle_geometric_product(Scalar self, Circle other) {
    return Circle(vec4(self.g0) * other.g0, vec3(self.g0) * other.g1, vec3(self.g0) * other.g2);
}

Circle scalar_circle_outer_product(Scalar self, Circle other) {
    return Circle(vec4(self.g0) * other.g0, vec3(self.g0) * other.g1, vec3(self.g0) * other.g2);
}

Circle scalar_circle_inner_product(Scalar self, Circle other) {
    return Circle(vec4(self.g0) * other.g0, vec3(self.g0) * other.g1, vec3(self.g0) * other.g2);
}

Circle scalar_circle_left_contraction(Scalar self, Circle other) {
    return Circle(vec4(self.g0) * other.g0, vec3(self.g0) * other.g1, vec3(self.g0) * other.g2);
}

Plane scalar_plane_geometric_product(Scalar self, Plane other) {
    return Plane(vec4(self.g0) * other.g0);
}

Plane scalar_plane_outer_product(Scalar self, Plane other) {
    return Plane(vec4(self.g0) * other.g0);
}

Plane scalar_plane_inner_product(Scalar self, Plane other) {
    return Plane(vec4(self.g0) * other.g0);
}

Plane scalar_plane_left_contraction(Scalar self, Plane other) {
    return Plane(vec4(self.g0) * other.g0);
}

Sphere scalar_sphere_geometric_product(Scalar self, Sphere other) {
    return Sphere(vec2(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Sphere scalar_sphere_outer_product(Scalar self, Sphere other) {
    return Sphere(vec2(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Sphere scalar_sphere_inner_product(Scalar self, Sphere other) {
    return Sphere(vec2(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Sphere scalar_sphere_left_contraction(Scalar self, Sphere other) {
    return Sphere(vec2(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Motor scalar_motor_geometric_product(Scalar self, Motor other) {
    return Motor(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

Scalar scalar_motor_regressive_product(Scalar self, Motor other) {
    return Scalar(self.g0 * other.g0.w);
}

Motor scalar_motor_outer_product(Scalar self, Motor other) {
    return Motor(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

Motor scalar_motor_inner_product(Scalar self, Motor other) {
    return Motor(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

Motor scalar_motor_left_contraction(Scalar self, Motor other) {
    return Motor(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

Rotor scalar_rotor_geometric_product(Scalar self, Rotor other) {
    return Rotor(vec4(self.g0) * other.g0);
}

Scalar scalar_rotor_regressive_product(Scalar self, Rotor other) {
    return Scalar(self.g0 * other.g0.w);
}

Rotor scalar_rotor_outer_product(Scalar self, Rotor other) {
    return Rotor(vec4(self.g0) * other.g0);
}

Rotor scalar_rotor_inner_product(Scalar self, Rotor other) {
    return Rotor(vec4(self.g0) * other.g0);
}

Rotor scalar_rotor_left_contraction(Scalar self, Rotor other) {
    return Rotor(vec4(self.g0) * other.g0);
}

Translator scalar_translator_geometric_product(Scalar self, Translator other) {
    return Translator(vec4(self.g0) * other.g0);
}

Scalar scalar_translator_regressive_product(Scalar self, Translator other) {
    return Scalar(self.g0 * other.g0.w);
}

Translator scalar_translator_outer_product(Scalar self, Translator other) {
    return Translator(vec4(self.g0) * other.g0);
}

Translator scalar_translator_inner_product(Scalar self, Translator other) {
    return Translator(vec4(self.g0) * other.g0);
}

Scalar scalar_translator_geometric_anti_product(Scalar self, Translator other) {
    return Scalar(self.g0 * other.g0.w);
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

Flector scalar_flector_outer_product(Scalar self, Flector other) {
    return Flector(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

Flector scalar_flector_inner_product(Scalar self, Flector other) {
    return Flector(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

Flector scalar_flector_left_contraction(Scalar self, Flector other) {
    return Flector(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

Dilation scalar_dilation_geometric_product(Scalar self, Dilation other) {
    return Dilation(vec3(self.g0) * other.g0, vec2(self.g0) * other.g1);
}

Scalar scalar_dilation_regressive_product(Scalar self, Dilation other) {
    return Scalar(self.g0 * other.g1.y);
}

Dilation scalar_dilation_outer_product(Scalar self, Dilation other) {
    return Dilation(vec3(self.g0) * other.g0, vec2(self.g0) * other.g1);
}

Dilation scalar_dilation_inner_product(Scalar self, Dilation other) {
    return Dilation(vec3(self.g0) * other.g0, vec2(self.g0) * other.g1);
}

Scalar scalar_dilation_geometric_anti_product(Scalar self, Dilation other) {
    return Scalar(self.g0 * other.g1.y);
}

Scalar scalar_dilation_inner_anti_product(Scalar self, Dilation other) {
    return Scalar(self.g0 * other.g1.y);
}

Dilation scalar_dilation_left_contraction(Scalar self, Dilation other) {
    return Dilation(vec3(self.g0) * other.g0, vec2(self.g0) * other.g1);
}

Scalar scalar_dilation_right_anti_contraction(Scalar self, Dilation other) {
    return Scalar(self.g0 * other.g1.y);
}

MultiVector scalar_multi_vector_add(Scalar self, MultiVector other) {
    return MultiVector(vec3(self.g0) * vec3(1.0, 0.0, 0.0) + other.g0, other.g1, other.g2, other.g3, other.g4, other.g5, other.g6, other.g7, other.g8, other.g9);
}

MultiVector scalar_multi_vector_sub(Scalar self, MultiVector other) {
    return MultiVector(vec3(self.g0) * vec3(1.0, 0.0, 0.0) - other.g0, vec3(0.0) - other.g1, vec2(0.0) - other.g2, vec4(0.0) - other.g3, vec3(0.0) - other.g4, vec3(0.0) - other.g5, vec3(0.0) - other.g6, vec3(0.0) - other.g7, vec4(0.0) - other.g8, vec4(0.0) - other.g9);
}

MultiVector scalar_multi_vector_geometric_product(Scalar self, MultiVector other) {
    return MultiVector(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1, vec2(self.g0) * other.g2, vec4(self.g0) * other.g3, vec3(self.g0) * other.g4, vec3(self.g0) * other.g5, vec3(self.g0) * other.g6, vec3(self.g0) * other.g7, vec4(self.g0) * other.g8, vec4(self.g0) * other.g9);
}

Scalar scalar_multi_vector_regressive_product(Scalar self, MultiVector other) {
    return Scalar(self.g0 * other.g0.z);
}

MultiVector scalar_multi_vector_outer_product(Scalar self, MultiVector other) {
    return MultiVector(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1, vec2(self.g0) * other.g2, vec4(self.g0) * other.g3, vec3(self.g0) * other.g4, vec3(self.g0) * other.g5, vec3(self.g0) * other.g6, vec3(self.g0) * other.g7, vec4(self.g0) * other.g8, vec4(self.g0) * other.g9);
}

MultiVector scalar_multi_vector_inner_product(Scalar self, MultiVector other) {
    return MultiVector(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1, vec2(self.g0) * other.g2, vec4(self.g0) * other.g3, vec3(self.g0) * other.g4, vec3(self.g0) * other.g5, vec3(self.g0) * other.g6, vec3(self.g0) * other.g7, vec4(self.g0) * other.g8, vec4(self.g0) * other.g9);
}

MultiVector scalar_multi_vector_left_contraction(Scalar self, MultiVector other) {
    return MultiVector(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1, vec2(self.g0) * other.g2, vec4(self.g0) * other.g3, vec3(self.g0) * other.g4, vec3(self.g0) * other.g5, vec3(self.g0) * other.g6, vec3(self.g0) * other.g7, vec4(self.g0) * other.g8, vec4(self.g0) * other.g9);
}

Scalar scalar_multi_vector_right_contraction(Scalar self, MultiVector other) {
    return Scalar(self.g0 * other.g0.x);
}

Scalar scalar_multi_vector_scalar_product(Scalar self, MultiVector other) {
    return Scalar(self.g0 * other.g0.x);
}

Scalar scalar_squared_magnitude(Scalar self) {
    return scalar_scalar_scalar_product(self, scalar_reversal(self));
}

Scalar scalar_magnitude(Scalar self) {
    return Scalar(sqrt(scalar_squared_magnitude(self).g0));
}

Scalar scalar_scale(Scalar self, float other) {
    return scalar_scalar_geometric_product(self, Scalar(other));
}

Scalar scalar_signum(Scalar self) {
    return scalar_scalar_geometric_product(self, Scalar(1.0 / scalar_magnitude(self).g0));
}

Scalar scalar_inverse(Scalar self) {
    return scalar_scalar_geometric_product(scalar_reversal(self), Scalar(1.0 / scalar_squared_magnitude(self).g0));
}

AntiScalar anti_scalar_zero() {
    return AntiScalar(0.0);
}

AntiScalar anti_scalar_one() {
    return AntiScalar(0.0);
}

int anti_scalar_grade(AntiScalar self) {
    return 5;
}

int anti_scalar_anti_grade(AntiScalar self) {
    return 0;
}

AntiScalar anti_scalar_neg(AntiScalar self) {
    return AntiScalar(self.g0 * -1.0);
}

AntiScalar anti_scalar_automorphism(AntiScalar self) {
    return AntiScalar(self.g0 * -1.0);
}

AntiScalar anti_scalar_reversal(AntiScalar self) {
    return AntiScalar(self.g0);
}

AntiScalar anti_scalar_conjugation(AntiScalar self) {
    return AntiScalar(self.g0 * -1.0);
}

Scalar anti_scalar_dual(AntiScalar self) {
    return Scalar(self.g0);
}

AntiScalar anti_scalar_anti_reversal(AntiScalar self) {
    return AntiScalar(self.g0 * -1.0);
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

HomogeneousMagnitude anti_scalar_scalar_add(AntiScalar self, Scalar other) {
    return HomogeneousMagnitude(vec2(self.g0) * vec2(0.0, 1.0) + vec2(other.g0) * vec2(1.0, 0.0));
}

HomogeneousMagnitude anti_scalar_scalar_sub(AntiScalar self, Scalar other) {
    return HomogeneousMagnitude(vec2(self.g0) * vec2(0.0, 1.0) - vec2(other.g0) * vec2(1.0, 0.0));
}

AntiScalar anti_scalar_scalar_geometric_product(AntiScalar self, Scalar other) {
    return AntiScalar(self.g0 * other.g0);
}

Scalar anti_scalar_scalar_regressive_product(AntiScalar self, Scalar other) {
    return Scalar(self.g0 * other.g0);
}

AntiScalar anti_scalar_scalar_outer_product(AntiScalar self, Scalar other) {
    return AntiScalar(self.g0 * other.g0);
}

AntiScalar anti_scalar_scalar_inner_product(AntiScalar self, Scalar other) {
    return AntiScalar(self.g0 * other.g0);
}

Scalar anti_scalar_scalar_geometric_anti_product(AntiScalar self, Scalar other) {
    return Scalar(self.g0 * other.g0);
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

AntiScalar anti_scalar_anti_scalar_regressive_product(AntiScalar self, AntiScalar other) {
    return AntiScalar(self.g0 * other.g0);
}

AntiScalar anti_scalar_anti_scalar_geometric_anti_product(AntiScalar self, AntiScalar other) {
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

HomogeneousMagnitude anti_scalar_homogeneous_magnitude_add(AntiScalar self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(vec2(self.g0) * vec2(0.0, 1.0) + other.g0);
}

HomogeneousMagnitude anti_scalar_homogeneous_magnitude_sub(AntiScalar self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(vec2(self.g0) * vec2(0.0, 1.0) - other.g0);
}

AntiScalar anti_scalar_homogeneous_magnitude_geometric_product(AntiScalar self, HomogeneousMagnitude other) {
    return AntiScalar(self.g0 * other.g0.x);
}

HomogeneousMagnitude anti_scalar_homogeneous_magnitude_regressive_product(AntiScalar self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(vec2(self.g0) * other.g0);
}

AntiScalar anti_scalar_homogeneous_magnitude_outer_product(AntiScalar self, HomogeneousMagnitude other) {
    return AntiScalar(self.g0 * other.g0.x);
}

AntiScalar anti_scalar_homogeneous_magnitude_inner_product(AntiScalar self, HomogeneousMagnitude other) {
    return AntiScalar(self.g0 * other.g0.x);
}

HomogeneousMagnitude anti_scalar_homogeneous_magnitude_geometric_anti_product(AntiScalar self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(vec2(self.g0) * other.g0);
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

RadialPoint anti_scalar_radial_point_regressive_product(AntiScalar self, RadialPoint other) {
    return RadialPoint(vec3(self.g0) * other.g0, vec2(self.g0) * other.g1);
}

RadialPoint anti_scalar_radial_point_geometric_anti_product(AntiScalar self, RadialPoint other) {
    return RadialPoint(vec3(self.g0) * other.g0, vec2(self.g0) * other.g1);
}

RadialPoint anti_scalar_radial_point_inner_anti_product(AntiScalar self, RadialPoint other) {
    return RadialPoint(vec3(self.g0) * other.g0, vec2(self.g0) * other.g1);
}

RadialPoint anti_scalar_radial_point_left_anti_contraction(AntiScalar self, RadialPoint other) {
    return RadialPoint(vec3(self.g0) * other.g0, vec2(self.g0) * other.g1);
}

FlatPoint anti_scalar_flat_point_regressive_product(AntiScalar self, FlatPoint other) {
    return FlatPoint(vec4(self.g0) * other.g0);
}

FlatPoint anti_scalar_flat_point_geometric_anti_product(AntiScalar self, FlatPoint other) {
    return FlatPoint(vec4(self.g0) * other.g0);
}

FlatPoint anti_scalar_flat_point_inner_anti_product(AntiScalar self, FlatPoint other) {
    return FlatPoint(vec4(self.g0) * other.g0);
}

FlatPoint anti_scalar_flat_point_left_anti_contraction(AntiScalar self, FlatPoint other) {
    return FlatPoint(vec4(self.g0) * other.g0);
}

Dipole anti_scalar_dipole_regressive_product(AntiScalar self, Dipole other) {
    return Dipole(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1, vec4(self.g0) * other.g2);
}

Dipole anti_scalar_dipole_geometric_anti_product(AntiScalar self, Dipole other) {
    return Dipole(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1, vec4(self.g0) * other.g2);
}

Dipole anti_scalar_dipole_inner_anti_product(AntiScalar self, Dipole other) {
    return Dipole(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1, vec4(self.g0) * other.g2);
}

Dipole anti_scalar_dipole_left_anti_contraction(AntiScalar self, Dipole other) {
    return Dipole(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1, vec4(self.g0) * other.g2);
}

Line anti_scalar_line_regressive_product(AntiScalar self, Line other) {
    return Line(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Line anti_scalar_line_geometric_anti_product(AntiScalar self, Line other) {
    return Line(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Line anti_scalar_line_inner_anti_product(AntiScalar self, Line other) {
    return Line(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Line anti_scalar_line_left_anti_contraction(AntiScalar self, Line other) {
    return Line(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Circle anti_scalar_circle_regressive_product(AntiScalar self, Circle other) {
    return Circle(vec4(self.g0) * other.g0, vec3(self.g0) * other.g1, vec3(self.g0) * other.g2);
}

Circle anti_scalar_circle_geometric_anti_product(AntiScalar self, Circle other) {
    return Circle(vec4(self.g0) * other.g0, vec3(self.g0) * other.g1, vec3(self.g0) * other.g2);
}

Circle anti_scalar_circle_inner_anti_product(AntiScalar self, Circle other) {
    return Circle(vec4(self.g0) * other.g0, vec3(self.g0) * other.g1, vec3(self.g0) * other.g2);
}

Circle anti_scalar_circle_left_anti_contraction(AntiScalar self, Circle other) {
    return Circle(vec4(self.g0) * other.g0, vec3(self.g0) * other.g1, vec3(self.g0) * other.g2);
}

Plane anti_scalar_plane_regressive_product(AntiScalar self, Plane other) {
    return Plane(vec4(self.g0) * other.g0);
}

Plane anti_scalar_plane_geometric_anti_product(AntiScalar self, Plane other) {
    return Plane(vec4(self.g0) * other.g0);
}

Plane anti_scalar_plane_inner_anti_product(AntiScalar self, Plane other) {
    return Plane(vec4(self.g0) * other.g0);
}

Plane anti_scalar_plane_left_anti_contraction(AntiScalar self, Plane other) {
    return Plane(vec4(self.g0) * other.g0);
}

Sphere anti_scalar_sphere_regressive_product(AntiScalar self, Sphere other) {
    return Sphere(vec2(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Sphere anti_scalar_sphere_geometric_anti_product(AntiScalar self, Sphere other) {
    return Sphere(vec2(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Sphere anti_scalar_sphere_inner_anti_product(AntiScalar self, Sphere other) {
    return Sphere(vec2(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Sphere anti_scalar_sphere_left_anti_contraction(AntiScalar self, Sphere other) {
    return Sphere(vec2(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Motor anti_scalar_motor_add(AntiScalar self, Motor other) {
    return Motor(vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) + other.g0, other.g1);
}

Motor anti_scalar_motor_sub(AntiScalar self, Motor other) {
    return Motor(vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) - other.g0, vec4(0.0) - other.g1);
}

Motor anti_scalar_motor_regressive_product(AntiScalar self, Motor other) {
    return Motor(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

Motor anti_scalar_motor_geometric_anti_product(AntiScalar self, Motor other) {
    return Motor(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

Motor anti_scalar_motor_inner_anti_product(AntiScalar self, Motor other) {
    return Motor(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

Motor anti_scalar_motor_left_anti_contraction(AntiScalar self, Motor other) {
    return Motor(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

AntiScalar anti_scalar_motor_right_anti_contraction(AntiScalar self, Motor other) {
    return AntiScalar(self.g0 * other.g0.w);
}

AntiScalar anti_scalar_motor_anti_scalar_product(AntiScalar self, Motor other) {
    return AntiScalar(self.g0 * other.g0.w);
}

Rotor anti_scalar_rotor_add(AntiScalar self, Rotor other) {
    return Rotor(vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) + other.g0);
}

Rotor anti_scalar_rotor_sub(AntiScalar self, Rotor other) {
    return Rotor(vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) - other.g0);
}

Rotor anti_scalar_rotor_regressive_product(AntiScalar self, Rotor other) {
    return Rotor(vec4(self.g0) * other.g0);
}

Rotor anti_scalar_rotor_geometric_anti_product(AntiScalar self, Rotor other) {
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

Translator anti_scalar_translator_add(AntiScalar self, Translator other) {
    return Translator(vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) + other.g0);
}

Translator anti_scalar_translator_sub(AntiScalar self, Translator other) {
    return Translator(vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) - other.g0);
}

Translator anti_scalar_translator_regressive_product(AntiScalar self, Translator other) {
    return Translator(vec4(self.g0) * other.g0);
}

Translator anti_scalar_translator_geometric_anti_product(AntiScalar self, Translator other) {
    return Translator(vec4(self.g0) * other.g0);
}

Translator anti_scalar_translator_inner_anti_product(AntiScalar self, Translator other) {
    return Translator(vec4(self.g0) * other.g0);
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

Flector anti_scalar_flector_regressive_product(AntiScalar self, Flector other) {
    return Flector(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

Flector anti_scalar_flector_geometric_anti_product(AntiScalar self, Flector other) {
    return Flector(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

Flector anti_scalar_flector_inner_anti_product(AntiScalar self, Flector other) {
    return Flector(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

Flector anti_scalar_flector_left_anti_contraction(AntiScalar self, Flector other) {
    return Flector(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

Dilation anti_scalar_dilation_add(AntiScalar self, Dilation other) {
    return Dilation(other.g0, vec2(self.g0) * vec2(0.0, 1.0) + other.g1);
}

Dilation anti_scalar_dilation_sub(AntiScalar self, Dilation other) {
    return Dilation(vec3(0.0) - other.g0, vec2(self.g0) * vec2(0.0, 1.0) - other.g1);
}

Dilation anti_scalar_dilation_regressive_product(AntiScalar self, Dilation other) {
    return Dilation(vec3(self.g0) * other.g0, vec2(self.g0) * other.g1);
}

Dilation anti_scalar_dilation_geometric_anti_product(AntiScalar self, Dilation other) {
    return Dilation(vec3(self.g0) * other.g0, vec2(self.g0) * other.g1);
}

Dilation anti_scalar_dilation_inner_anti_product(AntiScalar self, Dilation other) {
    return Dilation(vec3(self.g0) * other.g0, vec2(self.g0) * other.g1);
}

Dilation anti_scalar_dilation_left_anti_contraction(AntiScalar self, Dilation other) {
    return Dilation(vec3(self.g0) * other.g0, vec2(self.g0) * other.g1);
}

AntiScalar anti_scalar_dilation_right_anti_contraction(AntiScalar self, Dilation other) {
    return AntiScalar(self.g0 * other.g1.y);
}

AntiScalar anti_scalar_dilation_anti_scalar_product(AntiScalar self, Dilation other) {
    return AntiScalar(self.g0 * other.g1.y);
}

MultiVector anti_scalar_multi_vector_add(AntiScalar self, MultiVector other) {
    return MultiVector(vec3(self.g0) * vec3(0.0, 0.0, 1.0) + other.g0, other.g1, other.g2, other.g3, other.g4, other.g5, other.g6, other.g7, other.g8, other.g9);
}

MultiVector anti_scalar_multi_vector_sub(AntiScalar self, MultiVector other) {
    return MultiVector(vec3(self.g0) * vec3(0.0, 0.0, 1.0) - other.g0, vec3(0.0) - other.g1, vec2(0.0) - other.g2, vec4(0.0) - other.g3, vec3(0.0) - other.g4, vec3(0.0) - other.g5, vec3(0.0) - other.g6, vec3(0.0) - other.g7, vec4(0.0) - other.g8, vec4(0.0) - other.g9);
}

MultiVector anti_scalar_multi_vector_regressive_product(AntiScalar self, MultiVector other) {
    return MultiVector(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1, vec2(self.g0) * other.g2, vec4(self.g0) * other.g3, vec3(self.g0) * other.g4, vec3(self.g0) * other.g5, vec3(self.g0) * other.g6, vec3(self.g0) * other.g7, vec4(self.g0) * other.g8, vec4(self.g0) * other.g9);
}

AntiScalar anti_scalar_multi_vector_outer_product(AntiScalar self, MultiVector other) {
    return AntiScalar(self.g0 * other.g0.x);
}

MultiVector anti_scalar_multi_vector_geometric_anti_product(AntiScalar self, MultiVector other) {
    return MultiVector(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1, vec2(self.g0) * other.g2, vec4(self.g0) * other.g3, vec3(self.g0) * other.g4, vec3(self.g0) * other.g5, vec3(self.g0) * other.g6, vec3(self.g0) * other.g7, vec4(self.g0) * other.g8, vec4(self.g0) * other.g9);
}

MultiVector anti_scalar_multi_vector_inner_anti_product(AntiScalar self, MultiVector other) {
    return MultiVector(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1, vec2(self.g0) * other.g2, vec4(self.g0) * other.g3, vec3(self.g0) * other.g4, vec3(self.g0) * other.g5, vec3(self.g0) * other.g6, vec3(self.g0) * other.g7, vec4(self.g0) * other.g8, vec4(self.g0) * other.g9);
}

MultiVector anti_scalar_multi_vector_left_anti_contraction(AntiScalar self, MultiVector other) {
    return MultiVector(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1, vec2(self.g0) * other.g2, vec4(self.g0) * other.g3, vec3(self.g0) * other.g4, vec3(self.g0) * other.g5, vec3(self.g0) * other.g6, vec3(self.g0) * other.g7, vec4(self.g0) * other.g8, vec4(self.g0) * other.g9);
}

AntiScalar anti_scalar_multi_vector_right_anti_contraction(AntiScalar self, MultiVector other) {
    return AntiScalar(self.g0 * other.g0.z);
}

AntiScalar anti_scalar_multi_vector_anti_scalar_product(AntiScalar self, MultiVector other) {
    return AntiScalar(self.g0 * other.g0.z);
}

AntiScalar anti_scalar_scale(AntiScalar self, float other) {
    return anti_scalar_scalar_geometric_product(self, Scalar(other));
}

Circle anti_scalar_attitude(AntiScalar self) {
    return anti_scalar_circle_regressive_product(self, Circle(vec4(0.0, 0.0, 0.0, 1.0), vec3(0.0), vec3(0.0)));
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
    return HomogeneousMagnitude(self.g0 * vec2(1.0, -1.0));
}

HomogeneousMagnitude homogeneous_magnitude_reversal(HomogeneousMagnitude self) {
    return HomogeneousMagnitude(self.g0);
}

HomogeneousMagnitude homogeneous_magnitude_conjugation(HomogeneousMagnitude self) {
    return HomogeneousMagnitude(self.g0 * vec2(1.0, -1.0));
}

HomogeneousMagnitude homogeneous_magnitude_dual(HomogeneousMagnitude self) {
    return HomogeneousMagnitude(self.g0.yx);
}

HomogeneousMagnitude homogeneous_magnitude_anti_reversal(HomogeneousMagnitude self) {
    return HomogeneousMagnitude(self.g0 * vec2(1.0, -1.0));
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

Scalar homogeneous_magnitude_scalar_regressive_product(HomogeneousMagnitude self, Scalar other) {
    return Scalar(self.g0.y * other.g0);
}

HomogeneousMagnitude homogeneous_magnitude_scalar_outer_product(HomogeneousMagnitude self, Scalar other) {
    return HomogeneousMagnitude(self.g0 * vec2(other.g0));
}

HomogeneousMagnitude homogeneous_magnitude_scalar_inner_product(HomogeneousMagnitude self, Scalar other) {
    return HomogeneousMagnitude(self.g0 * vec2(other.g0));
}

Scalar homogeneous_magnitude_scalar_geometric_anti_product(HomogeneousMagnitude self, Scalar other) {
    return Scalar(self.g0.y * other.g0);
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

HomogeneousMagnitude homogeneous_magnitude_anti_scalar_regressive_product(HomogeneousMagnitude self, AntiScalar other) {
    return HomogeneousMagnitude(self.g0 * vec2(other.g0));
}

AntiScalar homogeneous_magnitude_anti_scalar_outer_product(HomogeneousMagnitude self, AntiScalar other) {
    return AntiScalar(self.g0.x * other.g0);
}

AntiScalar homogeneous_magnitude_anti_scalar_inner_product(HomogeneousMagnitude self, AntiScalar other) {
    return AntiScalar(self.g0.x * other.g0);
}

HomogeneousMagnitude homogeneous_magnitude_anti_scalar_geometric_anti_product(HomogeneousMagnitude self, AntiScalar other) {
    return HomogeneousMagnitude(self.g0 * vec2(other.g0));
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

HomogeneousMagnitude homogeneous_magnitude_homogeneous_magnitude_regressive_product(HomogeneousMagnitude self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(vec2(self.g0.y) * other.g0 + vec2(self.g0.x) * other.g0.yx * vec2(1.0, 0.0));
}

HomogeneousMagnitude homogeneous_magnitude_homogeneous_magnitude_outer_product(HomogeneousMagnitude self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(vec2(self.g0.x) * other.g0 + self.g0 * vec2(other.g0.x) * vec2(0.0, 1.0));
}

HomogeneousMagnitude homogeneous_magnitude_homogeneous_magnitude_inner_product(HomogeneousMagnitude self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(vec2(self.g0.x) * other.g0 + self.g0 * vec2(other.g0.x) * vec2(0.0, 1.0));
}

HomogeneousMagnitude homogeneous_magnitude_homogeneous_magnitude_geometric_anti_product(HomogeneousMagnitude self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(vec2(self.g0.y) * other.g0 + vec2(self.g0.x) * other.g0.yx * vec2(1.0, 0.0));
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

AntiScalar homogeneous_magnitude_homogeneous_magnitude_anti_scalar_product(HomogeneousMagnitude self, HomogeneousMagnitude other) {
    return AntiScalar(self.g0.y * other.g0.y);
}

RadialPoint homogeneous_magnitude_radial_point_regressive_product(HomogeneousMagnitude self, RadialPoint other) {
    return RadialPoint(vec3(self.g0.y) * other.g0, vec2(self.g0.y) * other.g1);
}

RadialPoint homogeneous_magnitude_radial_point_outer_product(HomogeneousMagnitude self, RadialPoint other) {
    return RadialPoint(vec3(self.g0.x) * other.g0, vec2(self.g0.x) * other.g1);
}

RadialPoint homogeneous_magnitude_radial_point_geometric_anti_product(HomogeneousMagnitude self, RadialPoint other) {
    return RadialPoint(vec3(self.g0.y) * other.g0, vec2(self.g0.y) * other.g1);
}

RadialPoint homogeneous_magnitude_radial_point_inner_anti_product(HomogeneousMagnitude self, RadialPoint other) {
    return RadialPoint(vec3(self.g0.y) * other.g0, vec2(self.g0.y) * other.g1);
}

RadialPoint homogeneous_magnitude_radial_point_left_contraction(HomogeneousMagnitude self, RadialPoint other) {
    return RadialPoint(vec3(self.g0.x) * other.g0, vec2(self.g0.x) * other.g1);
}

RadialPoint homogeneous_magnitude_radial_point_left_anti_contraction(HomogeneousMagnitude self, RadialPoint other) {
    return RadialPoint(vec3(self.g0.y) * other.g0, vec2(self.g0.y) * other.g1);
}

FlatPoint homogeneous_magnitude_flat_point_geometric_product(HomogeneousMagnitude self, FlatPoint other) {
    return FlatPoint(vec4(self.g0.x) * other.g0);
}

FlatPoint homogeneous_magnitude_flat_point_regressive_product(HomogeneousMagnitude self, FlatPoint other) {
    return FlatPoint(vec4(self.g0.y) * other.g0);
}

FlatPoint homogeneous_magnitude_flat_point_outer_product(HomogeneousMagnitude self, FlatPoint other) {
    return FlatPoint(vec4(self.g0.x) * other.g0);
}

FlatPoint homogeneous_magnitude_flat_point_inner_product(HomogeneousMagnitude self, FlatPoint other) {
    return FlatPoint(vec4(self.g0.x) * other.g0);
}

FlatPoint homogeneous_magnitude_flat_point_left_contraction(HomogeneousMagnitude self, FlatPoint other) {
    return FlatPoint(vec4(self.g0.x) * other.g0);
}

FlatPoint homogeneous_magnitude_flat_point_left_anti_contraction(HomogeneousMagnitude self, FlatPoint other) {
    return FlatPoint(vec4(self.g0.y) * other.g0);
}

Dipole homogeneous_magnitude_dipole_regressive_product(HomogeneousMagnitude self, Dipole other) {
    return Dipole(vec3(self.g0.y) * other.g0, vec3(self.g0.y) * other.g1, vec4(self.g0.y) * other.g2);
}

Dipole homogeneous_magnitude_dipole_outer_product(HomogeneousMagnitude self, Dipole other) {
    return Dipole(vec3(self.g0.x) * other.g0, vec3(self.g0.x) * other.g1, vec4(self.g0.x) * other.g2);
}

Dipole homogeneous_magnitude_dipole_left_contraction(HomogeneousMagnitude self, Dipole other) {
    return Dipole(vec3(self.g0.x) * other.g0, vec3(self.g0.x) * other.g1, vec4(self.g0.x) * other.g2);
}

Dipole homogeneous_magnitude_dipole_left_anti_contraction(HomogeneousMagnitude self, Dipole other) {
    return Dipole(vec3(self.g0.y) * other.g0, vec3(self.g0.y) * other.g1, vec4(self.g0.y) * other.g2);
}

Line homogeneous_magnitude_line_geometric_product(HomogeneousMagnitude self, Line other) {
    return Line(vec3(self.g0.x) * other.g0, vec3(self.g0.x) * other.g1);
}

Line homogeneous_magnitude_line_regressive_product(HomogeneousMagnitude self, Line other) {
    return Line(vec3(self.g0.y) * other.g0, vec3(self.g0.y) * other.g1);
}

Line homogeneous_magnitude_line_outer_product(HomogeneousMagnitude self, Line other) {
    return Line(vec3(self.g0.x) * other.g0, vec3(self.g0.x) * other.g1);
}

Line homogeneous_magnitude_line_inner_product(HomogeneousMagnitude self, Line other) {
    return Line(vec3(self.g0.x) * other.g0, vec3(self.g0.x) * other.g1);
}

Line homogeneous_magnitude_line_left_contraction(HomogeneousMagnitude self, Line other) {
    return Line(vec3(self.g0.x) * other.g0, vec3(self.g0.x) * other.g1);
}

Line homogeneous_magnitude_line_left_anti_contraction(HomogeneousMagnitude self, Line other) {
    return Line(vec3(self.g0.y) * other.g0, vec3(self.g0.y) * other.g1);
}

Circle homogeneous_magnitude_circle_regressive_product(HomogeneousMagnitude self, Circle other) {
    return Circle(vec4(self.g0.y) * other.g0, vec3(self.g0.y) * other.g1, vec3(self.g0.y) * other.g2);
}

Circle homogeneous_magnitude_circle_outer_product(HomogeneousMagnitude self, Circle other) {
    return Circle(vec4(self.g0.x) * other.g0, vec3(self.g0.x) * other.g1, vec3(self.g0.x) * other.g2);
}

Circle homogeneous_magnitude_circle_left_contraction(HomogeneousMagnitude self, Circle other) {
    return Circle(vec4(self.g0.x) * other.g0, vec3(self.g0.x) * other.g1, vec3(self.g0.x) * other.g2);
}

Circle homogeneous_magnitude_circle_left_anti_contraction(HomogeneousMagnitude self, Circle other) {
    return Circle(vec4(self.g0.y) * other.g0, vec3(self.g0.y) * other.g1, vec3(self.g0.y) * other.g2);
}

Plane homogeneous_magnitude_plane_geometric_product(HomogeneousMagnitude self, Plane other) {
    return Plane(vec4(self.g0.x) * other.g0);
}

Plane homogeneous_magnitude_plane_regressive_product(HomogeneousMagnitude self, Plane other) {
    return Plane(vec4(self.g0.y) * other.g0);
}

Plane homogeneous_magnitude_plane_outer_product(HomogeneousMagnitude self, Plane other) {
    return Plane(vec4(self.g0.x) * other.g0);
}

Plane homogeneous_magnitude_plane_inner_product(HomogeneousMagnitude self, Plane other) {
    return Plane(vec4(self.g0.x) * other.g0);
}

Plane homogeneous_magnitude_plane_left_contraction(HomogeneousMagnitude self, Plane other) {
    return Plane(vec4(self.g0.x) * other.g0);
}

Plane homogeneous_magnitude_plane_left_anti_contraction(HomogeneousMagnitude self, Plane other) {
    return Plane(vec4(self.g0.y) * other.g0);
}

Sphere homogeneous_magnitude_sphere_geometric_product(HomogeneousMagnitude self, Sphere other) {
    return Sphere(vec2(self.g0.x) * other.g0, vec3(self.g0.x) * other.g1);
}

Sphere homogeneous_magnitude_sphere_regressive_product(HomogeneousMagnitude self, Sphere other) {
    return Sphere(vec2(self.g0.y) * other.g0, vec3(self.g0.y) * other.g1);
}

Sphere homogeneous_magnitude_sphere_outer_product(HomogeneousMagnitude self, Sphere other) {
    return Sphere(vec2(self.g0.x) * other.g0, vec3(self.g0.x) * other.g1);
}

Sphere homogeneous_magnitude_sphere_inner_product(HomogeneousMagnitude self, Sphere other) {
    return Sphere(vec2(self.g0.x) * other.g0, vec3(self.g0.x) * other.g1);
}

Sphere homogeneous_magnitude_sphere_left_contraction(HomogeneousMagnitude self, Sphere other) {
    return Sphere(vec2(self.g0.x) * other.g0, vec3(self.g0.x) * other.g1);
}

Sphere homogeneous_magnitude_sphere_left_anti_contraction(HomogeneousMagnitude self, Sphere other) {
    return Sphere(vec2(self.g0.y) * other.g0, vec3(self.g0.y) * other.g1);
}

Motor homogeneous_magnitude_motor_geometric_product(HomogeneousMagnitude self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0, vec4(self.g0.x) * other.g1);
}

Motor homogeneous_magnitude_motor_outer_product(HomogeneousMagnitude self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0, vec4(self.g0.x) * other.g1);
}

Motor homogeneous_magnitude_motor_inner_product(HomogeneousMagnitude self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0, vec4(self.g0.x) * other.g1);
}

Motor homogeneous_magnitude_motor_left_contraction(HomogeneousMagnitude self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0, vec4(self.g0.x) * other.g1);
}

Motor homogeneous_magnitude_motor_left_anti_contraction(HomogeneousMagnitude self, Motor other) {
    return Motor(vec4(self.g0.y) * other.g0, vec4(self.g0.y) * other.g1);
}

AntiScalar homogeneous_magnitude_motor_anti_scalar_product(HomogeneousMagnitude self, Motor other) {
    return AntiScalar(self.g0.y * other.g0.w);
}

Rotor homogeneous_magnitude_rotor_geometric_product(HomogeneousMagnitude self, Rotor other) {
    return Rotor(vec4(self.g0.x) * other.g0);
}

Rotor homogeneous_magnitude_rotor_outer_product(HomogeneousMagnitude self, Rotor other) {
    return Rotor(vec4(self.g0.x) * other.g0);
}

Rotor homogeneous_magnitude_rotor_inner_product(HomogeneousMagnitude self, Rotor other) {
    return Rotor(vec4(self.g0.x) * other.g0);
}

Rotor homogeneous_magnitude_rotor_left_contraction(HomogeneousMagnitude self, Rotor other) {
    return Rotor(vec4(self.g0.x) * other.g0);
}

Rotor homogeneous_magnitude_rotor_left_anti_contraction(HomogeneousMagnitude self, Rotor other) {
    return Rotor(vec4(self.g0.y) * other.g0);
}

AntiScalar homogeneous_magnitude_rotor_anti_scalar_product(HomogeneousMagnitude self, Rotor other) {
    return AntiScalar(self.g0.y * other.g0.w);
}

Translator homogeneous_magnitude_translator_geometric_product(HomogeneousMagnitude self, Translator other) {
    return Translator(vec4(self.g0.x) * other.g0);
}

Translator homogeneous_magnitude_translator_outer_product(HomogeneousMagnitude self, Translator other) {
    return Translator(vec4(self.g0.x) * other.g0);
}

Translator homogeneous_magnitude_translator_inner_product(HomogeneousMagnitude self, Translator other) {
    return Translator(vec4(self.g0.x) * other.g0);
}

Translator homogeneous_magnitude_translator_left_contraction(HomogeneousMagnitude self, Translator other) {
    return Translator(vec4(self.g0.x) * other.g0);
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

Flector homogeneous_magnitude_flector_geometric_product(HomogeneousMagnitude self, Flector other) {
    return Flector(vec4(self.g0.x) * other.g0, vec4(self.g0.x) * other.g1);
}

Flector homogeneous_magnitude_flector_regressive_product(HomogeneousMagnitude self, Flector other) {
    return Flector(vec4(self.g0.y) * other.g0, vec4(self.g0.y) * other.g1);
}

Flector homogeneous_magnitude_flector_outer_product(HomogeneousMagnitude self, Flector other) {
    return Flector(vec4(self.g0.x) * other.g0, vec4(self.g0.x) * other.g1);
}

Flector homogeneous_magnitude_flector_inner_product(HomogeneousMagnitude self, Flector other) {
    return Flector(vec4(self.g0.x) * other.g0, vec4(self.g0.x) * other.g1);
}

Flector homogeneous_magnitude_flector_left_contraction(HomogeneousMagnitude self, Flector other) {
    return Flector(vec4(self.g0.x) * other.g0, vec4(self.g0.x) * other.g1);
}

Flector homogeneous_magnitude_flector_left_anti_contraction(HomogeneousMagnitude self, Flector other) {
    return Flector(vec4(self.g0.y) * other.g0, vec4(self.g0.y) * other.g1);
}

Dilation homogeneous_magnitude_dilation_outer_product(HomogeneousMagnitude self, Dilation other) {
    return Dilation(vec3(self.g0.x) * other.g0, vec2(self.g0.x) * other.g1);
}

Dilation homogeneous_magnitude_dilation_left_contraction(HomogeneousMagnitude self, Dilation other) {
    return Dilation(vec3(self.g0.x) * other.g0, vec2(self.g0.x) * other.g1);
}

Dilation homogeneous_magnitude_dilation_left_anti_contraction(HomogeneousMagnitude self, Dilation other) {
    return Dilation(vec3(self.g0.y) * other.g0, vec2(self.g0.y) * other.g1);
}

HomogeneousMagnitude homogeneous_magnitude_dilation_right_anti_contraction(HomogeneousMagnitude self, Dilation other) {
    return HomogeneousMagnitude(self.g0 * vec2(other.g1.y));
}

AntiScalar homogeneous_magnitude_dilation_anti_scalar_product(HomogeneousMagnitude self, Dilation other) {
    return AntiScalar(self.g0.y * other.g1.y);
}

MultiVector homogeneous_magnitude_multi_vector_add(HomogeneousMagnitude self, MultiVector other) {
    return MultiVector(vec3(self.g0.x, self.g0.x, self.g0.y) * vec3(1.0, 0.0, 1.0) + other.g0, other.g1, other.g2, other.g3, other.g4, other.g5, other.g6, other.g7, other.g8, other.g9);
}

MultiVector homogeneous_magnitude_multi_vector_sub(HomogeneousMagnitude self, MultiVector other) {
    return MultiVector(vec3(self.g0.x, self.g0.x, self.g0.y) * vec3(1.0, 0.0, 1.0) - other.g0, vec3(0.0) - other.g1, vec2(0.0) - other.g2, vec4(0.0) - other.g3, vec3(0.0) - other.g4, vec3(0.0) - other.g5, vec3(0.0) - other.g6, vec3(0.0) - other.g7, vec4(0.0) - other.g8, vec4(0.0) - other.g9);
}

MultiVector homogeneous_magnitude_multi_vector_geometric_product(HomogeneousMagnitude self, MultiVector other) {
    return MultiVector(vec3(self.g0.x) * other.g0 + vec3(self.g0.x, self.g0.x, self.g0.y) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0), vec3(self.g0.x) * other.g1, vec2(self.g0.x) * other.g2, vec4(self.g0.x) * other.g3 + vec4(self.g0.x, self.g0.x, self.g0.x, self.g0.y) * other.g8.xxxw * vec4(0.0, 0.0, 0.0, 1.0), vec3(self.g0.x) * other.g4, vec3(self.g0.x) * other.g5, vec3(self.g0.x) * other.g6 + vec3(self.g0.y) * other.g5, vec3(self.g0.x) * other.g7, vec4(self.g0.x) * other.g8, vec4(self.g0.x) * other.g9 + vec4(self.g0.y, self.g0.y, self.g0.y, self.g0.x) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

MultiVector homogeneous_magnitude_multi_vector_regressive_product(HomogeneousMagnitude self, MultiVector other) {
    return MultiVector(vec3(self.g0.y) * other.g0 + vec3(self.g0.x) * other.g0.zxx * vec3(1.0, 0.0, 0.0), vec3(self.g0.y) * other.g1, vec2(self.g0.y) * other.g2, vec4(self.g0.y) * other.g3, vec3(self.g0.y) * other.g4, vec3(self.g0.y) * other.g5, vec3(self.g0.y) * other.g6, vec3(self.g0.y) * other.g7, vec4(self.g0.y) * other.g8, vec4(self.g0.y) * other.g9);
}

MultiVector homogeneous_magnitude_multi_vector_outer_product(HomogeneousMagnitude self, MultiVector other) {
    return MultiVector(vec3(self.g0.x) * other.g0 + vec3(self.g0.x, self.g0.x, self.g0.y) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0), vec3(self.g0.x) * other.g1, vec2(self.g0.x) * other.g2, vec4(self.g0.x) * other.g3, vec3(self.g0.x) * other.g4, vec3(self.g0.x) * other.g5, vec3(self.g0.x) * other.g6, vec3(self.g0.x) * other.g7, vec4(self.g0.x) * other.g8, vec4(self.g0.x) * other.g9);
}

MultiVector homogeneous_magnitude_multi_vector_inner_product(HomogeneousMagnitude self, MultiVector other) {
    return MultiVector(vec3(self.g0.x) * other.g0 + vec3(self.g0.x, self.g0.x, self.g0.y) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0), vec3(self.g0.x) * other.g1, vec2(self.g0.x) * other.g2, vec4(self.g0.x) * other.g3 + vec4(self.g0.x, self.g0.x, self.g0.x, self.g0.y) * other.g8.xxxw * vec4(0.0, 0.0, 0.0, 1.0), vec3(self.g0.x) * other.g4, vec3(self.g0.x) * other.g5, vec3(self.g0.x) * other.g6 + vec3(self.g0.y) * other.g5, vec3(self.g0.x) * other.g7, vec4(self.g0.x) * other.g8, vec4(self.g0.x) * other.g9 + vec4(self.g0.y, self.g0.y, self.g0.y, self.g0.x) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

MultiVector homogeneous_magnitude_multi_vector_geometric_anti_product(HomogeneousMagnitude self, MultiVector other) {
    return MultiVector(vec3(self.g0.y) * other.g0 + vec3(self.g0.x) * other.g0.zxx * vec3(1.0, 0.0, 0.0), vec3(self.g0.x) * vec3(other.g9.x, other.g9.y, other.g9.z) + vec3(self.g0.y) * other.g1, vec2(self.g0.y) * other.g2, vec4(self.g0.y) * other.g3, vec3(self.g0.y) * other.g4, vec3(self.g0.x) * other.g6 + vec3(self.g0.y) * other.g5, vec3(self.g0.y) * other.g6, vec3(self.g0.y) * other.g7, vec4(self.g0.y) * other.g8 + vec4(self.g0.x) * other.g3.xxxw * vec4(0.0, 0.0, 0.0, 1.0), vec4(self.g0.y) * other.g9);
}

MultiVector homogeneous_magnitude_multi_vector_inner_anti_product(HomogeneousMagnitude self, MultiVector other) {
    return MultiVector(vec3(self.g0.y) * other.g0 + vec3(self.g0.x) * other.g0.zxx * vec3(1.0, 0.0, 0.0), vec3(self.g0.x) * vec3(other.g9.x, other.g9.y, other.g9.z) + vec3(self.g0.y) * other.g1, vec2(self.g0.y) * other.g2, vec4(self.g0.y) * other.g3, vec3(self.g0.y) * other.g4, vec3(self.g0.x) * other.g6 + vec3(self.g0.y) * other.g5, vec3(self.g0.y) * other.g6, vec3(self.g0.y) * other.g7, vec4(self.g0.y) * other.g8 + vec4(self.g0.x) * other.g3.xxxw * vec4(0.0, 0.0, 0.0, 1.0), vec4(self.g0.y) * other.g9);
}

MultiVector homogeneous_magnitude_multi_vector_left_contraction(HomogeneousMagnitude self, MultiVector other) {
    return MultiVector(vec3(self.g0.x) * other.g0, vec3(self.g0.x) * other.g1, vec2(self.g0.x) * other.g2, vec4(self.g0.x) * other.g3, vec3(self.g0.x) * other.g4, vec3(self.g0.x) * other.g5, vec3(self.g0.x) * other.g6, vec3(self.g0.x) * other.g7, vec4(self.g0.x) * other.g8, vec4(self.g0.x) * other.g9);
}

MultiVector homogeneous_magnitude_multi_vector_left_anti_contraction(HomogeneousMagnitude self, MultiVector other) {
    return MultiVector(vec3(self.g0.y) * other.g0, vec3(self.g0.y) * other.g1, vec2(self.g0.y) * other.g2, vec4(self.g0.y) * other.g3, vec3(self.g0.y) * other.g4, vec3(self.g0.y) * other.g5, vec3(self.g0.y) * other.g6, vec3(self.g0.y) * other.g7, vec4(self.g0.y) * other.g8, vec4(self.g0.y) * other.g9);
}

Scalar homogeneous_magnitude_multi_vector_scalar_product(HomogeneousMagnitude self, MultiVector other) {
    return Scalar(self.g0.x * other.g0.x);
}

AntiScalar homogeneous_magnitude_multi_vector_anti_scalar_product(HomogeneousMagnitude self, MultiVector other) {
    return AntiScalar(self.g0.y * other.g0.z);
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

AntiScalar homogeneous_magnitude_squared_anti_magnitude(HomogeneousMagnitude self) {
    return homogeneous_magnitude_homogeneous_magnitude_anti_scalar_product(self, homogeneous_magnitude_anti_reversal(self));
}

AntiScalar homogeneous_magnitude_weight_norm(HomogeneousMagnitude self) {
    return AntiScalar(sqrt(homogeneous_magnitude_squared_anti_magnitude(self).g0));
}

HomogeneousMagnitude homogeneous_magnitude_geometric_norm(HomogeneousMagnitude self) {
    return scalar_anti_scalar_add(homogeneous_magnitude_bulk_norm(self), homogeneous_magnitude_weight_norm(self));
}

HomogeneousMagnitude homogeneous_magnitude_scale(HomogeneousMagnitude self, float other) {
    return homogeneous_magnitude_scalar_geometric_product(self, Scalar(other));
}

HomogeneousMagnitude homogeneous_magnitude_signum(HomogeneousMagnitude self) {
    return homogeneous_magnitude_scalar_geometric_product(self, Scalar(1.0 / homogeneous_magnitude_magnitude(self).g0));
}

HomogeneousMagnitude homogeneous_magnitude_inverse(HomogeneousMagnitude self) {
    return homogeneous_magnitude_scalar_geometric_product(homogeneous_magnitude_reversal(self), Scalar(1.0 / homogeneous_magnitude_squared_magnitude(self).g0));
}

HomogeneousMagnitude homogeneous_magnitude_unitize(HomogeneousMagnitude self) {
    return homogeneous_magnitude_scalar_geometric_product(self, Scalar(1.0 / homogeneous_magnitude_weight_norm(self).g0));
}

Circle homogeneous_magnitude_attitude(HomogeneousMagnitude self) {
    return homogeneous_magnitude_circle_regressive_product(self, Circle(vec4(0.0, 0.0, 0.0, 1.0), vec3(0.0), vec3(0.0)));
}

RadialPoint radial_point_zero() {
    return RadialPoint(vec3(0.0), vec2(0.0));
}

RadialPoint radial_point_one() {
    return RadialPoint(vec3(0.0), vec2(0.0));
}

int radial_point_grade(RadialPoint self) {
    return 1;
}

int radial_point_anti_grade(RadialPoint self) {
    return 4;
}

RadialPoint radial_point_neg(RadialPoint self) {
    return RadialPoint(self.g0 * vec3(-1.0), self.g1 * vec2(-1.0));
}

RadialPoint radial_point_automorphism(RadialPoint self) {
    return RadialPoint(self.g0 * vec3(-1.0), self.g1 * vec2(-1.0));
}

RadialPoint radial_point_reversal(RadialPoint self) {
    return RadialPoint(self.g0, self.g1);
}

RadialPoint radial_point_conjugation(RadialPoint self) {
    return RadialPoint(self.g0 * vec3(-1.0), self.g1 * vec2(-1.0));
}

Sphere radial_point_dual(RadialPoint self) {
    return Sphere(self.g1.yx, self.g0);
}

RadialPoint radial_point_anti_reversal(RadialPoint self) {
    return RadialPoint(self.g0 * vec3(-1.0), self.g1 * vec2(-1.0));
}

Sphere radial_point_right_complement(RadialPoint self) {
    return Sphere(self.g1.yx, self.g0);
}

Sphere radial_point_left_complement(RadialPoint self) {
    return Sphere(self.g1.yx, self.g0);
}

RadialPoint radial_point_double_complement(RadialPoint self) {
    return RadialPoint(self.g0, self.g1);
}

RadialPoint radial_point_scalar_geometric_product(RadialPoint self, Scalar other) {
    return RadialPoint(self.g0 * vec3(other.g0), self.g1 * vec2(other.g0));
}

RadialPoint radial_point_scalar_outer_product(RadialPoint self, Scalar other) {
    return RadialPoint(self.g0 * vec3(other.g0), self.g1 * vec2(other.g0));
}

RadialPoint radial_point_scalar_inner_product(RadialPoint self, Scalar other) {
    return RadialPoint(self.g0 * vec3(other.g0), self.g1 * vec2(other.g0));
}

RadialPoint radial_point_scalar_right_contraction(RadialPoint self, Scalar other) {
    return RadialPoint(self.g0 * vec3(other.g0), self.g1 * vec2(other.g0));
}

RadialPoint radial_point_anti_scalar_regressive_product(RadialPoint self, AntiScalar other) {
    return RadialPoint(self.g0 * vec3(other.g0), self.g1 * vec2(other.g0));
}

RadialPoint radial_point_anti_scalar_geometric_anti_product(RadialPoint self, AntiScalar other) {
    return RadialPoint(self.g0 * vec3(other.g0), self.g1 * vec2(other.g0));
}

RadialPoint radial_point_anti_scalar_inner_anti_product(RadialPoint self, AntiScalar other) {
    return RadialPoint(self.g0 * vec3(other.g0), self.g1 * vec2(other.g0));
}

RadialPoint radial_point_anti_scalar_right_anti_contraction(RadialPoint self, AntiScalar other) {
    return RadialPoint(self.g0 * vec3(other.g0), self.g1 * vec2(other.g0));
}

RadialPoint radial_point_homogeneous_magnitude_regressive_product(RadialPoint self, HomogeneousMagnitude other) {
    return RadialPoint(self.g0 * vec3(other.g0.y), self.g1 * vec2(other.g0.y));
}

RadialPoint radial_point_homogeneous_magnitude_outer_product(RadialPoint self, HomogeneousMagnitude other) {
    return RadialPoint(self.g0 * vec3(other.g0.x), self.g1 * vec2(other.g0.x));
}

RadialPoint radial_point_homogeneous_magnitude_geometric_anti_product(RadialPoint self, HomogeneousMagnitude other) {
    return RadialPoint(self.g0 * vec3(other.g0.y), self.g1 * vec2(other.g0.y));
}

RadialPoint radial_point_homogeneous_magnitude_inner_anti_product(RadialPoint self, HomogeneousMagnitude other) {
    return RadialPoint(self.g0 * vec3(other.g0.y), self.g1 * vec2(other.g0.y));
}

RadialPoint radial_point_homogeneous_magnitude_right_contraction(RadialPoint self, HomogeneousMagnitude other) {
    return RadialPoint(self.g0 * vec3(other.g0.x), self.g1 * vec2(other.g0.x));
}

RadialPoint radial_point_homogeneous_magnitude_right_anti_contraction(RadialPoint self, HomogeneousMagnitude other) {
    return RadialPoint(self.g0 * vec3(other.g0.y), self.g1 * vec2(other.g0.y));
}

RadialPoint radial_point_radial_point_add(RadialPoint self, RadialPoint other) {
    return RadialPoint(self.g0 + other.g0, self.g1 + other.g1);
}

RadialPoint radial_point_radial_point_sub(RadialPoint self, RadialPoint other) {
    return RadialPoint(self.g0 - other.g0, self.g1 - other.g1);
}

RadialPoint radial_point_radial_point_mul(RadialPoint self, RadialPoint other) {
    return RadialPoint(self.g0 * other.g0, self.g1 * other.g1);
}

RadialPoint radial_point_radial_point_div(RadialPoint self, RadialPoint other) {
    return RadialPoint(vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(1.0, 1.0, 1.0) / vec3(other.g0.x, other.g0.y, other.g0.z) * vec3(1.0, 1.0, 1.0), vec2(self.g1.x, self.g1.y) * vec2(1.0, 1.0) / vec2(other.g1.x, other.g1.y) * vec2(1.0, 1.0));
}

Dipole radial_point_radial_point_outer_product(RadialPoint self, RadialPoint other) {
    return Dipole(vec3(self.g1.x) * other.g0 + self.g0 * vec3(other.g1.x) * vec3(-1.0), vec3(self.g0.y) * other.g0.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * other.g0.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * other.g0.xzy * vec3(0.0, -1.0, 1.0), vec4(self.g1.x) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, 1.0) - vec4(self.g1.y) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.x) + vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(other.g1.y, other.g1.y, other.g1.y, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Scalar radial_point_radial_point_inner_product(RadialPoint self, RadialPoint other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

Scalar radial_point_radial_point_left_contraction(RadialPoint self, RadialPoint other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

Scalar radial_point_radial_point_right_contraction(RadialPoint self, RadialPoint other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

Scalar radial_point_radial_point_scalar_product(RadialPoint self, RadialPoint other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

Line radial_point_flat_point_outer_product(RadialPoint self, FlatPoint other) {
    return Line(vec3(self.g1.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g0 * vec3(other.g0.w) * vec3(-1.0), vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0));
}

Circle radial_point_dipole_outer_product(RadialPoint self, Dipole other) {
    return Circle(vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0), vec3(self.g1.x) * vec3(other.g2.x, other.g2.y, other.g2.z) + vec3(self.g1.y) * other.g0 + self.g0 * vec3(other.g2.w) * vec3(-1.0), vec3(self.g0.y) * vec3(other.g2.z, other.g2.z, other.g2.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g2.y, other.g2.x, other.g2.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.y) * other.g1 + vec3(self.g0.x) * vec3(other.g2.x, other.g2.z, other.g2.y) * vec3(0.0, -1.0, 1.0));
}

RadialPoint radial_point_dipole_inner_product(RadialPoint self, Dipole other) {
    return RadialPoint(vec3(self.g0.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * other.g1.xzy * vec3(0.0, 1.0, -1.0), vec2(self.g0.x) * vec2(other.g0.x, other.g2.x) * vec2(-1.0, 1.0) + vec2(self.g0.y) * vec2(other.g0.y, other.g2.y) * vec2(-1.0, 1.0) + vec2(self.g0.z) * vec2(other.g0.z, other.g2.z) * vec2(-1.0, 1.0));
}

RadialPoint radial_point_dipole_left_contraction(RadialPoint self, Dipole other) {
    return RadialPoint(vec3(self.g0.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * other.g1.xzy * vec3(0.0, 1.0, -1.0), vec2(self.g0.x) * vec2(other.g0.x, other.g2.x) * vec2(-1.0, 1.0) + vec2(self.g0.y) * vec2(other.g0.y, other.g2.y) * vec2(-1.0, 1.0) + vec2(self.g0.z) * vec2(other.g0.z, other.g2.z) * vec2(-1.0, 1.0));
}

Flector radial_point_line_geometric_product(RadialPoint self, Line other) {
    return Flector(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0), vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

Plane radial_point_line_outer_product(RadialPoint self, Line other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

FlatPoint radial_point_line_inner_product(RadialPoint self, Line other) {
    return FlatPoint(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

FlatPoint radial_point_line_left_contraction(RadialPoint self, Line other) {
    return FlatPoint(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

Sphere radial_point_circle_outer_product(RadialPoint self, Circle other) {
    return Sphere(vec2(self.g0.x) * vec2(other.g0.x, other.g2.x) * vec2(1.0, -1.0) + vec2(self.g0.y) * vec2(other.g0.y, other.g2.y) * vec2(1.0, -1.0) + vec2(self.g0.z) * vec2(other.g0.z, other.g2.z) * vec2(1.0, -1.0) + self.g1 * vec2(other.g0.w) * vec2(1.0, -1.0), vec3(self.g0.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g1.x) * other.g2 - vec3(self.g1.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g0.x) * other.g1.xzy * vec3(0.0, 1.0, -1.0));
}

Dipole radial_point_circle_inner_product(RadialPoint self, Circle other) {
    return Dipole(vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0), self.g0 * vec3(other.g0.w) * vec3(-1.0), vec4(self.g0.y) * vec4(other.g2.z, other.g2.z, other.g2.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g2.y, other.g2.x, other.g2.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.x) * vec4(other.g2.x, other.g2.z, other.g2.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

Dipole radial_point_circle_left_contraction(RadialPoint self, Circle other) {
    return Dipole(vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0), self.g0 * vec3(other.g0.w) * vec3(-1.0), vec4(self.g0.y) * vec4(other.g2.z, other.g2.z, other.g2.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g2.y, other.g2.x, other.g2.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.x) * vec4(other.g2.x, other.g2.z, other.g2.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

Scalar radial_point_plane_regressive_product(RadialPoint self, Plane other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z + self.g1.x * other.g0.w);
}

AntiScalar radial_point_plane_outer_product(RadialPoint self, Plane other) {
    return AntiScalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z + self.g1.x * other.g0.w);
}

Line radial_point_plane_inner_product(RadialPoint self, Plane other) {
    return Line(vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0), self.g0 * vec3(other.g0.w) * vec3(-1.0));
}

Line radial_point_plane_left_contraction(RadialPoint self, Plane other) {
    return Line(vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0), self.g0 * vec3(other.g0.w) * vec3(-1.0));
}

Scalar radial_point_sphere_regressive_product(RadialPoint self, Sphere other) {
    return Scalar(self.g0.x * other.g1.x + self.g0.y * other.g1.y + self.g0.z * other.g1.z + self.g1.x * other.g0.y + self.g1.y * other.g0.x);
}

AntiScalar radial_point_sphere_outer_product(RadialPoint self, Sphere other) {
    return AntiScalar(self.g0.x * other.g1.x + self.g0.y * other.g1.y + self.g0.z * other.g1.z + self.g1.x * other.g0.y + self.g1.y * other.g0.x);
}

Flector radial_point_motor_geometric_product(RadialPoint self, Motor other) {
    return Flector(vec4(self.g0.x) * vec4(other.g1.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.y) * vec4(other.g1.z, other.g1.w, other.g1.x, other.g0.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.w, other.g0.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g1.x) * other.g1.xxxw * vec4(0.0, 0.0, 0.0, 1.0), vec4(self.g0.x) * vec4(other.g0.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.y) * vec4(other.g0.z, other.g0.w, other.g0.x, other.g1.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.w, other.g1.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g1.x) * other.g1.xyzx * vec4(1.0, 1.0, 1.0, 0.0));
}

RadialPoint radial_point_motor_regressive_product(RadialPoint self, Motor other) {
    return RadialPoint(self.g0 * vec3(other.g0.w), self.g1 * vec2(other.g0.w));
}

Flector radial_point_motor_outer_product(RadialPoint self, Motor other) {
    return Flector(vec4(self.g1.x) * vec4(other.g1.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * other.g1.wwwx * vec4(1.0, 1.0, 1.0, 0.0), vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.x) * other.g1.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

RadialPoint radial_point_rotor_regressive_product(RadialPoint self, Rotor other) {
    return RadialPoint(self.g0 * vec3(other.g0.w), self.g1 * vec2(other.g0.w));
}

RadialPoint radial_point_translator_regressive_product(RadialPoint self, Translator other) {
    return RadialPoint(self.g0 * vec3(other.g0.w), self.g1 * vec2(other.g0.w));
}

Plane radial_point_translator_outer_product(RadialPoint self, Translator other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.x) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

RadialPoint radial_point_translator_geometric_anti_product(RadialPoint self, Translator other) {
    return RadialPoint(vec3(0.0) - vec3(self.g1.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g0 * vec3(other.g0.w), self.g1 * vec2(other.g0.w));
}

RadialPoint radial_point_translator_inner_anti_product(RadialPoint self, Translator other) {
    return RadialPoint(self.g0 * vec3(other.g0.w), self.g1 * vec2(other.g0.w));
}

RadialPoint radial_point_translator_right_anti_contraction(RadialPoint self, Translator other) {
    return RadialPoint(self.g0 * vec3(other.g0.w), self.g1 * vec2(other.g0.w));
}

Motor radial_point_flector_geometric_product(RadialPoint self, Flector other) {
    return Motor(vec4(self.g0.x) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g1.x) * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g0.y) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g1.y) * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g1.z) * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g1.x) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.w), vec4(self.g0.x) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g0.x) * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g0.y) * vec4(other.g0.z, other.g1.w, other.g0.x, other.g0.y) * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g1.w, other.g0.z) * vec4(-1.0, 1.0, -1.0, 1.0));
}

Scalar radial_point_flector_regressive_product(RadialPoint self, Flector other) {
    return Scalar(self.g0.x * other.g1.x + self.g0.y * other.g1.y + self.g0.z * other.g1.z + self.g1.x * other.g1.w);
}

RadialPoint radial_point_dilation_regressive_product(RadialPoint self, Dilation other) {
    return RadialPoint(self.g0 * vec3(other.g1.y), self.g1 * vec2(other.g1.y));
}

Sphere radial_point_dilation_outer_product(RadialPoint self, Dilation other) {
    return Sphere(vec2(self.g0.y) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g1.y) * vec2(other.g0.z) * vec2(0.0, 1.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(-1.0, 0.0), vec3(self.g1.y) * vec3(other.g1.x, other.g0.x, other.g0.y));
}

RadialPoint radial_point_dilation_geometric_anti_product(RadialPoint self, Dilation other) {
    return RadialPoint(vec3(0.0) - vec3(self.g1.y) * vec3(other.g1.x, other.g0.x, other.g0.y) + self.g0 * vec3(other.g1.y), self.g1 * vec2(other.g1.y));
}

RadialPoint radial_point_dilation_inner_anti_product(RadialPoint self, Dilation other) {
    return RadialPoint(self.g0 * vec3(other.g1.y), self.g1 * vec2(other.g1.y));
}

RadialPoint radial_point_dilation_right_anti_contraction(RadialPoint self, Dilation other) {
    return RadialPoint(self.g0 * vec3(other.g1.y), self.g1 * vec2(other.g1.y));
}

MultiVector radial_point_multi_vector_add(RadialPoint self, MultiVector other) {
    return MultiVector(other.g0, self.g0 + other.g1, self.g1 + other.g2, other.g3, other.g4, other.g5, other.g6, other.g7, other.g8, other.g9);
}

MultiVector radial_point_multi_vector_sub(RadialPoint self, MultiVector other) {
    return MultiVector(vec3(0.0) - other.g0, self.g0 - other.g1, self.g1 - other.g2, vec4(0.0) - other.g3, vec3(0.0) - other.g4, vec3(0.0) - other.g5, vec3(0.0) - other.g6, vec3(0.0) - other.g7, vec4(0.0) - other.g8, vec4(0.0) - other.g9);
}

MultiVector radial_point_multi_vector_geometric_product(RadialPoint self, MultiVector other) {
    return MultiVector(vec3(self.g0.x) * vec3(other.g1.x, other.g8.x, other.g9.x) + vec3(self.g0.y) * vec3(other.g1.y, other.g8.y, other.g9.y) + vec3(self.g0.z) * vec3(other.g1.z, other.g8.z, other.g9.z) + vec3(self.g1.y) * vec3(other.g0.y) * vec3(0.0, 0.0, 1.0) + vec3(self.g1.x) * vec3(other.g8.x, other.g8.w, other.g9.w) * vec3(0.0, 1.0, 1.0), vec3(self.g0.x) * vec3(other.g0.x, other.g5.z, other.g5.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.y) * vec3(other.g5.z, other.g0.x, other.g5.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g0.z) * vec3(other.g5.y, other.g5.x, other.g0.x) * vec3(1.0, -1.0, 1.0), vec2(self.g0.x) * vec2(other.g4.x, other.g3.x) * vec2(-1.0, 1.0) + vec2(self.g0.y) * vec2(other.g4.y, other.g3.y) * vec2(-1.0, 1.0) + vec2(self.g0.z) * vec2(other.g4.z, other.g3.z) * vec2(-1.0, 1.0) + self.g1 * vec2(other.g0.x), vec4(self.g0.x) * vec4(other.g2.y, other.g7.z, other.g7.y, other.g6.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.y) * vec4(other.g7.z, other.g2.y, other.g7.x, other.g6.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g7.y, other.g7.x, other.g2.y, other.g6.z) * vec4(1.0, -1.0, 1.0, -1.0) - vec4(self.g1.y) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g2.x) + vec4(self.g1.x) * vec4(other.g2.x, other.g2.x, other.g2.x, other.g2.y) * vec4(0.0, 0.0, 0.0, 1.0), vec3(self.g0.x) * vec3(other.g2.x, other.g8.z, other.g8.y) * vec3(-1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g8.z, other.g2.x, other.g8.x) * vec3(1.0, -1.0, -1.0) + vec3(self.g0.z) * vec3(other.g8.y, other.g8.x, other.g2.x) * vec3(-1.0, 1.0, -1.0) + vec3(self.g1.x) * other.g1, vec3(self.g0.x) * vec3(other.g8.w, other.g1.z, other.g1.y) * vec3(-1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g1.z, other.g8.w, other.g1.x) * vec3(1.0, -1.0, -1.0) + vec3(self.g0.z) * vec3(other.g1.y, other.g1.x, other.g8.w) * vec3(-1.0, 1.0, -1.0), vec3(self.g0.x) * vec3(other.g3.w, other.g9.z, other.g9.y) * vec3(-1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g9.z, other.g3.w, other.g9.x) * vec3(1.0, -1.0, -1.0) + vec3(self.g0.z) * vec3(other.g9.y, other.g9.x, other.g3.w) * vec3(-1.0, 1.0, -1.0) + vec3(self.g1.x) * vec3(other.g3.x, other.g3.y, other.g3.z) + vec3(self.g1.y) * other.g4, vec3(self.g0.x) * vec3(other.g9.w, other.g3.z, other.g3.y) * vec3(-1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g3.z, other.g9.w, other.g3.x) * vec3(1.0, -1.0, -1.0) + vec3(self.g0.z) * vec3(other.g3.y, other.g3.x, other.g9.w) * vec3(-1.0, 1.0, -1.0) + vec3(self.g1.y) * other.g5, vec4(self.g0.x) * vec4(other.g0.y, other.g4.z, other.g4.y, other.g5.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.y) * vec4(other.g4.z, other.g0.y, other.g4.x, other.g5.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g4.y, other.g4.x, other.g0.y, other.g5.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g1.x) * vec4(other.g5.x, other.g5.y, other.g5.z, other.g5.x) * vec4(1.0, 1.0, 1.0, 0.0), vec4(self.g0.x) * vec4(other.g0.z, other.g6.z, other.g6.y, other.g7.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.y) * vec4(other.g6.z, other.g0.z, other.g6.x, other.g7.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g6.y, other.g6.x, other.g0.z, other.g7.z) * vec4(1.0, -1.0, 1.0, -1.0) - vec4(self.g1.y) * other.g8 + vec4(self.g1.x) * vec4(other.g7.x, other.g7.y, other.g7.z, other.g7.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Scalar radial_point_multi_vector_scalar_product(RadialPoint self, MultiVector other) {
    return Scalar(self.g0.x * other.g1.x + self.g0.y * other.g1.y + self.g0.z * other.g1.z);
}

Scalar radial_point_squared_magnitude(RadialPoint self) {
    return radial_point_radial_point_scalar_product(self, radial_point_reversal(self));
}

Scalar radial_point_magnitude(RadialPoint self) {
    return Scalar(sqrt(radial_point_squared_magnitude(self).g0));
}

RadialPoint radial_point_scale(RadialPoint self, float other) {
    return radial_point_scalar_geometric_product(self, Scalar(other));
}

RadialPoint radial_point_signum(RadialPoint self) {
    return radial_point_scalar_geometric_product(self, Scalar(1.0 / radial_point_magnitude(self).g0));
}

RadialPoint radial_point_inverse(RadialPoint self) {
    return radial_point_scalar_geometric_product(radial_point_reversal(self), Scalar(1.0 / radial_point_squared_magnitude(self).g0));
}

FlatPoint flat_point_zero() {
    return FlatPoint(vec4(0.0));
}

FlatPoint flat_point_one() {
    return FlatPoint(vec4(0.0));
}

int flat_point_grade(FlatPoint self) {
    return 2;
}

int flat_point_anti_grade(FlatPoint self) {
    return 3;
}

FlatPoint flat_point_neg(FlatPoint self) {
    return FlatPoint(self.g0 * vec4(-1.0));
}

FlatPoint flat_point_automorphism(FlatPoint self) {
    return FlatPoint(self.g0);
}

FlatPoint flat_point_reversal(FlatPoint self) {
    return FlatPoint(self.g0 * vec4(-1.0));
}

FlatPoint flat_point_conjugation(FlatPoint self) {
    return FlatPoint(self.g0 * vec4(-1.0));
}

FlatPoint flat_point_anti_reversal(FlatPoint self) {
    return FlatPoint(self.g0 * vec4(-1.0));
}

FlatPoint flat_point_double_complement(FlatPoint self) {
    return FlatPoint(self.g0);
}

FlatPoint flat_point_scalar_geometric_product(FlatPoint self, Scalar other) {
    return FlatPoint(self.g0 * vec4(other.g0));
}

FlatPoint flat_point_scalar_outer_product(FlatPoint self, Scalar other) {
    return FlatPoint(self.g0 * vec4(other.g0));
}

FlatPoint flat_point_scalar_inner_product(FlatPoint self, Scalar other) {
    return FlatPoint(self.g0 * vec4(other.g0));
}

FlatPoint flat_point_scalar_right_contraction(FlatPoint self, Scalar other) {
    return FlatPoint(self.g0 * vec4(other.g0));
}

FlatPoint flat_point_anti_scalar_regressive_product(FlatPoint self, AntiScalar other) {
    return FlatPoint(self.g0 * vec4(other.g0));
}

FlatPoint flat_point_anti_scalar_geometric_anti_product(FlatPoint self, AntiScalar other) {
    return FlatPoint(self.g0 * vec4(other.g0));
}

FlatPoint flat_point_anti_scalar_inner_anti_product(FlatPoint self, AntiScalar other) {
    return FlatPoint(self.g0 * vec4(other.g0));
}

FlatPoint flat_point_anti_scalar_right_anti_contraction(FlatPoint self, AntiScalar other) {
    return FlatPoint(self.g0 * vec4(other.g0));
}

FlatPoint flat_point_homogeneous_magnitude_geometric_product(FlatPoint self, HomogeneousMagnitude other) {
    return FlatPoint(self.g0 * vec4(other.g0.x));
}

FlatPoint flat_point_homogeneous_magnitude_regressive_product(FlatPoint self, HomogeneousMagnitude other) {
    return FlatPoint(self.g0 * vec4(other.g0.y));
}

FlatPoint flat_point_homogeneous_magnitude_outer_product(FlatPoint self, HomogeneousMagnitude other) {
    return FlatPoint(self.g0 * vec4(other.g0.x));
}

FlatPoint flat_point_homogeneous_magnitude_inner_product(FlatPoint self, HomogeneousMagnitude other) {
    return FlatPoint(self.g0 * vec4(other.g0.x));
}

FlatPoint flat_point_homogeneous_magnitude_right_contraction(FlatPoint self, HomogeneousMagnitude other) {
    return FlatPoint(self.g0 * vec4(other.g0.x));
}

FlatPoint flat_point_homogeneous_magnitude_right_anti_contraction(FlatPoint self, HomogeneousMagnitude other) {
    return FlatPoint(self.g0 * vec4(other.g0.y));
}

Line flat_point_radial_point_outer_product(FlatPoint self, RadialPoint other) {
    return Line(vec3(0.0) - vec3(self.g0.w) * other.g0 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.x), vec3(self.g0.y) * other.g0.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * other.g0.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * other.g0.xzy * vec3(0.0, 1.0, -1.0));
}

FlatPoint flat_point_flat_point_add(FlatPoint self, FlatPoint other) {
    return FlatPoint(self.g0 + other.g0);
}

FlatPoint flat_point_flat_point_sub(FlatPoint self, FlatPoint other) {
    return FlatPoint(self.g0 - other.g0);
}

FlatPoint flat_point_flat_point_mul(FlatPoint self, FlatPoint other) {
    return FlatPoint(self.g0 * other.g0);
}

FlatPoint flat_point_flat_point_div(FlatPoint self, FlatPoint other) {
    return FlatPoint(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.w) * vec4(1.0, 1.0, 1.0, 1.0));
}

Translator flat_point_flat_point_geometric_anti_product(FlatPoint self, FlatPoint other) {
    return Translator(vec4(self.g0.w) * other.g0 * vec4(1.0, 1.0, 1.0, -1.0) + self.g0.xyzx * other.g0.wwwx * vec4(-1.0, -1.0, -1.0, 0.0));
}

AntiScalar flat_point_flat_point_inner_anti_product(FlatPoint self, FlatPoint other) {
    return AntiScalar(0.0 - self.g0.w * other.g0.w);
}

AntiScalar flat_point_flat_point_left_anti_contraction(FlatPoint self, FlatPoint other) {
    return AntiScalar(0.0 - self.g0.w * other.g0.w);
}

AntiScalar flat_point_flat_point_right_anti_contraction(FlatPoint self, FlatPoint other) {
    return AntiScalar(0.0 - self.g0.w * other.g0.w);
}

AntiScalar flat_point_flat_point_anti_scalar_product(FlatPoint self, FlatPoint other) {
    return AntiScalar(0.0 - self.g0.w * other.g0.w);
}

Dipole flat_point_dipole_add(FlatPoint self, Dipole other) {
    return Dipole(other.g0, other.g1, self.g0 + other.g2);
}

Dipole flat_point_dipole_sub(FlatPoint self, Dipole other) {
    return Dipole(vec3(0.0) - other.g0, vec3(0.0) - other.g1, self.g0 - other.g2);
}

Flector flat_point_dipole_geometric_product(FlatPoint self, Dipole other) {
    return Flector(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0), vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

Plane flat_point_dipole_outer_product(FlatPoint self, Dipole other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

AntiScalar flat_point_dipole_inner_anti_product(FlatPoint self, Dipole other) {
    return AntiScalar(0.0 - self.g0.w * other.g2.w);
}

AntiScalar flat_point_dipole_left_anti_contraction(FlatPoint self, Dipole other) {
    return AntiScalar(0.0 - self.g0.w * other.g2.w);
}

AntiScalar flat_point_dipole_right_anti_contraction(FlatPoint self, Dipole other) {
    return AntiScalar(0.0 - self.g0.w * other.g2.w);
}

AntiScalar flat_point_dipole_anti_scalar_product(FlatPoint self, Dipole other) {
    return AntiScalar(0.0 - self.g0.w * other.g2.w);
}

Plane flat_point_line_inner_anti_product(FlatPoint self, Line other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, -1.0));
}

Plane flat_point_line_right_anti_contraction(FlatPoint self, Line other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, -1.0));
}

Scalar flat_point_circle_regressive_product(FlatPoint self, Circle other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

AntiScalar flat_point_circle_outer_product(FlatPoint self, Circle other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

Plane flat_point_circle_inner_anti_product(FlatPoint self, Circle other) {
    return Plane(vec4(self.g0.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, -1.0));
}

Plane flat_point_circle_right_anti_contraction(FlatPoint self, Circle other) {
    return Plane(vec4(self.g0.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, -1.0));
}

Flector flat_point_plane_add(FlatPoint self, Plane other) {
    return Flector(self.g0, other.g0);
}

Flector flat_point_plane_sub(FlatPoint self, Plane other) {
    return Flector(self.g0, vec4(0.0) - other.g0);
}

Line flat_point_plane_inner_anti_product(FlatPoint self, Plane other) {
    return Line(vec3(0.0) - vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0));
}

Line flat_point_plane_right_anti_contraction(FlatPoint self, Plane other) {
    return Line(vec3(0.0) - vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0));
}

RadialPoint flat_point_sphere_regressive_product(FlatPoint self, Sphere other) {
    return RadialPoint(vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x) * vec3(-1.0), vec2(self.g0.y) * vec2(other.g1.y) * vec2(0.0, 1.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(0.0, 1.0) + vec2(self.g0.w) * other.g0 * vec2(-1.0, 1.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(0.0, 1.0));
}

Line flat_point_sphere_inner_anti_product(FlatPoint self, Sphere other) {
    return Line(vec3(0.0) - vec3(self.g0.w) * other.g1, vec3(self.g0.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * other.g1.xzy * vec3(0.0, 1.0, -1.0));
}

Line flat_point_sphere_right_anti_contraction(FlatPoint self, Sphere other) {
    return Line(vec3(0.0) - vec3(self.g0.w) * other.g1, vec3(self.g0.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * other.g1.xzy * vec3(0.0, 1.0, -1.0));
}

FlatPoint flat_point_motor_regressive_product(FlatPoint self, Motor other) {
    return FlatPoint(self.g0 * vec4(other.g0.w));
}

Flector flat_point_motor_geometric_anti_product(FlatPoint self, Motor other) {
    return Flector(vec4(self.g0.y) * other.g0.zwxz * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxwy * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g0.w) * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g0.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, 0.0), vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.w) * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

Flector flat_point_motor_inner_anti_product(FlatPoint self, Motor other) {
    return Flector(self.g0 * vec4(other.g0.w), vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.w) * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

Flector flat_point_motor_right_anti_contraction(FlatPoint self, Motor other) {
    return Flector(self.g0 * vec4(other.g0.w), vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

FlatPoint flat_point_rotor_regressive_product(FlatPoint self, Rotor other) {
    return FlatPoint(self.g0 * vec4(other.g0.w));
}

Flector flat_point_rotor_geometric_anti_product(FlatPoint self, Rotor other) {
    return Flector(vec4(self.g0.y) * other.g0.zwxz * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxwy * vec4(-1.0, 1.0, 1.0, 0.0) + self.g0.xxxw * other.g0.wzyw * vec4(1.0, -1.0, 1.0, 1.0), vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

Flector flat_point_rotor_inner_anti_product(FlatPoint self, Rotor other) {
    return Flector(self.g0 * vec4(other.g0.w), vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

Flector flat_point_rotor_right_anti_contraction(FlatPoint self, Rotor other) {
    return Flector(self.g0 * vec4(other.g0.w), vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

FlatPoint flat_point_translator_regressive_product(FlatPoint self, Translator other) {
    return FlatPoint(self.g0 * vec4(other.g0.w));
}

FlatPoint flat_point_translator_geometric_anti_product(FlatPoint self, Translator other) {
    return FlatPoint(vec4(self.g0.w) * other.g0 * vec4(-1.0, -1.0, -1.0, 1.0) + self.g0.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

FlatPoint flat_point_translator_inner_anti_product(FlatPoint self, Translator other) {
    return FlatPoint(self.g0 * vec4(other.g0.w));
}

FlatPoint flat_point_translator_right_anti_contraction(FlatPoint self, Translator other) {
    return FlatPoint(self.g0 * vec4(other.g0.w));
}

Flector flat_point_flector_add(FlatPoint self, Flector other) {
    return Flector(self.g0 + other.g0, other.g1);
}

Flector flat_point_flector_sub(FlatPoint self, Flector other) {
    return Flector(self.g0 - other.g0, vec4(0.0) - other.g1);
}

Motor flat_point_flector_geometric_anti_product(FlatPoint self, Flector other) {
    return Motor(vec4(0.0) - vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g0.w), vec4(self.g0.x) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g1.x) * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g0.y) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g1.y) * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g1.z) * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.w));
}

AntiScalar flat_point_flector_left_anti_contraction(FlatPoint self, Flector other) {
    return AntiScalar(0.0 - self.g0.w * other.g0.w);
}

AntiScalar flat_point_flector_anti_scalar_product(FlatPoint self, Flector other) {
    return AntiScalar(0.0 - self.g0.w * other.g0.w);
}

AntiScalar flat_point_dilation_outer_product(FlatPoint self, Dilation other) {
    return AntiScalar(self.g0.x * other.g1.x + self.g0.y * other.g0.x + self.g0.z * other.g0.y + self.g0.w * other.g0.z);
}

FlatPoint flat_point_dilation_inner_anti_product(FlatPoint self, Dilation other) {
    return FlatPoint(self.g0 * vec4(other.g1.y));
}

FlatPoint flat_point_dilation_right_anti_contraction(FlatPoint self, Dilation other) {
    return FlatPoint(self.g0 * vec4(other.g1.y));
}

MultiVector flat_point_multi_vector_add(FlatPoint self, MultiVector other) {
    return MultiVector(other.g0, other.g1, other.g2, self.g0 + other.g3, other.g4, other.g5, other.g6, other.g7, other.g8, other.g9);
}

MultiVector flat_point_multi_vector_sub(FlatPoint self, MultiVector other) {
    return MultiVector(vec3(0.0) - other.g0, vec3(0.0) - other.g1, vec2(0.0) - other.g2, self.g0 - other.g3, vec3(0.0) - other.g4, vec3(0.0) - other.g5, vec3(0.0) - other.g6, vec3(0.0) - other.g7, vec4(0.0) - other.g8, vec4(0.0) - other.g9);
}

MultiVector flat_point_multi_vector_geometric_anti_product(FlatPoint self, MultiVector other) {
    return MultiVector(vec3(self.g0.y) * vec3(other.g8.y) * vec3(-1.0, 0.0, 0.0) + vec3(self.g0.z) * vec3(other.g8.z) * vec3(-1.0, 0.0, 0.0) + vec3(self.g0.w) * vec3(other.g8.w, other.g2.x, other.g3.w) * vec3(-1.0, 1.0, -1.0) + vec3(self.g0.x) * vec3(other.g8.x) * vec3(-1.0, 0.0, 0.0), vec3(self.g0.x) * vec3(other.g0.y, other.g4.z, other.g4.y) * vec3(-1.0, 1.0, -1.0) + vec3(self.g0.y) * vec3(other.g4.z, other.g0.y, other.g4.x) * vec3(-1.0, -1.0, 1.0) + vec3(self.g0.z) * vec3(other.g4.y, other.g4.x, other.g0.y) * vec3(1.0, -1.0, -1.0) + vec3(self.g0.w) * other.g5, vec2(self.g0.y) * vec2(other.g9.y) * vec2(0.0, 1.0) + vec2(self.g0.z) * vec2(other.g9.z) * vec2(0.0, 1.0) + vec2(self.g0.w) * vec2(other.g0.y, other.g9.w) * vec2(-1.0, 1.0) + vec2(self.g0.x) * vec2(other.g9.x) * vec2(0.0, 1.0), vec4(self.g0.y) * vec4(other.g6.z, other.g0.z, other.g6.x, other.g6.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g0.z) * vec4(other.g6.y, other.g6.x, other.g0.z, other.g6.y) * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g0.w) * vec4(other.g7.x, other.g7.y, other.g7.z, other.g0.z) * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.z, other.g6.z, other.g6.y, other.g0.x) * vec4(1.0, -1.0, 1.0, 0.0), vec3(self.g0.w) * vec3(other.g8.x, other.g8.y, other.g8.z), vec3(self.g0.x) * vec3(other.g2.x, other.g8.z, other.g8.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g8.z, other.g2.x, other.g8.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g8.y, other.g8.x, other.g2.x) * vec3(-1.0, 1.0, 1.0) - vec3(self.g0.w) * other.g1, vec3(0.0) - vec3(self.g0.w) * vec3(other.g9.x, other.g9.y, other.g9.z), vec3(self.g0.x) * vec3(other.g3.w, other.g9.z, other.g9.y) * vec3(-1.0, 1.0, -1.0) + vec3(self.g0.y) * vec3(other.g9.z, other.g3.w, other.g9.x) * vec3(-1.0, -1.0, 1.0) + vec3(self.g0.z) * vec3(other.g9.y, other.g9.x, other.g3.w) * vec3(1.0, -1.0, -1.0) + vec3(self.g0.w) * vec3(other.g3.x, other.g3.y, other.g3.z), vec4(self.g0.y) * vec4(other.g4.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g4.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g4.x, other.g4.y, other.g4.z, other.g0.x) * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g0.x) * vec4(other.g4.x) * vec4(0.0, 0.0, 0.0, 1.0), vec4(self.g0.y) * vec4(other.g6.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g6.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g6.x, other.g6.y, other.g6.z, other.g2.y) * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.x) * vec4(other.g6.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

AntiScalar flat_point_multi_vector_anti_scalar_product(FlatPoint self, MultiVector other) {
    return AntiScalar(0.0 - self.g0.w * other.g3.w);
}

FlatPoint flat_point_scale(FlatPoint self, float other) {
    return flat_point_scalar_geometric_product(self, Scalar(other));
}

Scalar flat_point_attitude(FlatPoint self) {
    return flat_point_circle_regressive_product(self, Circle(vec4(0.0, 0.0, 0.0, 1.0), vec3(0.0), vec3(0.0)));
}

Dipole dipole_zero() {
    return Dipole(vec3(0.0), vec3(0.0), vec4(0.0));
}

Dipole dipole_one() {
    return Dipole(vec3(0.0), vec3(0.0), vec4(0.0));
}

int dipole_grade(Dipole self) {
    return 2;
}

int dipole_anti_grade(Dipole self) {
    return 3;
}

Dipole dipole_neg(Dipole self) {
    return Dipole(self.g0 * vec3(-1.0), self.g1 * vec3(-1.0), self.g2 * vec4(-1.0));
}

Dipole dipole_automorphism(Dipole self) {
    return Dipole(self.g0, self.g1, self.g2);
}

Dipole dipole_reversal(Dipole self) {
    return Dipole(self.g0 * vec3(-1.0), self.g1 * vec3(-1.0), self.g2 * vec4(-1.0));
}

Dipole dipole_conjugation(Dipole self) {
    return Dipole(self.g0 * vec3(-1.0), self.g1 * vec3(-1.0), self.g2 * vec4(-1.0));
}

Circle dipole_dual(Dipole self) {
    return Circle(self.g2 * vec4(-1.0), self.g1 * vec3(-1.0), self.g0 * vec3(-1.0));
}

Dipole dipole_anti_reversal(Dipole self) {
    return Dipole(self.g0 * vec3(-1.0), self.g1 * vec3(-1.0), self.g2 * vec4(-1.0));
}

Circle dipole_right_complement(Dipole self) {
    return Circle(self.g2 * vec4(-1.0), self.g1 * vec3(-1.0), self.g0 * vec3(-1.0));
}

Circle dipole_left_complement(Dipole self) {
    return Circle(self.g2 * vec4(-1.0), self.g1 * vec3(-1.0), self.g0 * vec3(-1.0));
}

Dipole dipole_double_complement(Dipole self) {
    return Dipole(self.g0, self.g1, self.g2);
}

Dipole dipole_scalar_geometric_product(Dipole self, Scalar other) {
    return Dipole(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0), self.g2 * vec4(other.g0));
}

Dipole dipole_scalar_outer_product(Dipole self, Scalar other) {
    return Dipole(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0), self.g2 * vec4(other.g0));
}

Dipole dipole_scalar_inner_product(Dipole self, Scalar other) {
    return Dipole(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0), self.g2 * vec4(other.g0));
}

Dipole dipole_scalar_right_contraction(Dipole self, Scalar other) {
    return Dipole(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0), self.g2 * vec4(other.g0));
}

Dipole dipole_anti_scalar_regressive_product(Dipole self, AntiScalar other) {
    return Dipole(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0), self.g2 * vec4(other.g0));
}

Dipole dipole_anti_scalar_geometric_anti_product(Dipole self, AntiScalar other) {
    return Dipole(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0), self.g2 * vec4(other.g0));
}

Dipole dipole_anti_scalar_inner_anti_product(Dipole self, AntiScalar other) {
    return Dipole(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0), self.g2 * vec4(other.g0));
}

Dipole dipole_anti_scalar_right_anti_contraction(Dipole self, AntiScalar other) {
    return Dipole(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0), self.g2 * vec4(other.g0));
}

Dipole dipole_homogeneous_magnitude_regressive_product(Dipole self, HomogeneousMagnitude other) {
    return Dipole(self.g0 * vec3(other.g0.y), self.g1 * vec3(other.g0.y), self.g2 * vec4(other.g0.y));
}

Dipole dipole_homogeneous_magnitude_outer_product(Dipole self, HomogeneousMagnitude other) {
    return Dipole(self.g0 * vec3(other.g0.x), self.g1 * vec3(other.g0.x), self.g2 * vec4(other.g0.x));
}

Dipole dipole_homogeneous_magnitude_right_contraction(Dipole self, HomogeneousMagnitude other) {
    return Dipole(self.g0 * vec3(other.g0.x), self.g1 * vec3(other.g0.x), self.g2 * vec4(other.g0.x));
}

Dipole dipole_homogeneous_magnitude_right_anti_contraction(Dipole self, HomogeneousMagnitude other) {
    return Dipole(self.g0 * vec3(other.g0.y), self.g1 * vec3(other.g0.y), self.g2 * vec4(other.g0.y));
}

Circle dipole_radial_point_outer_product(Dipole self, RadialPoint other) {
    return Circle(vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g0.z) * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.y) * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.x) * vec4(other.g1.x, other.g1.x, other.g1.x, other.g0.x) * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * vec4(other.g1.x, other.g1.x, other.g1.x, other.g0.y) * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.x, other.g1.x, other.g1.x, other.g0.z) * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g0.x) * vec4(0.0, -1.0, 1.0, 0.0), vec3(self.g2.x) * vec3(other.g1.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g2.y) * vec3(other.g1.x) * vec3(0.0, 1.0, 0.0) + vec3(self.g2.z) * vec3(other.g1.x) * vec3(0.0, 0.0, 1.0) - vec3(self.g2.w) * other.g0 + self.g0 * vec3(other.g1.y), vec3(self.g2.x) * other.g0.zzy * vec3(0.0, 1.0, -1.0) + vec3(self.g2.y) * other.g0.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g2.z) * other.g0.yxy * vec3(1.0, -1.0, 0.0) + self.g1 * vec3(other.g1.y));
}

RadialPoint dipole_radial_point_inner_product(Dipole self, RadialPoint other) {
    return RadialPoint(vec3(self.g1.y) * other.g0.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * other.g0.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g1.x) * other.g0.xzy * vec3(0.0, 1.0, -1.0), vec2(self.g0.y) * vec2(other.g0.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(1.0, 0.0) + vec2(self.g2.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(1.0, 0.0));
}

RadialPoint dipole_radial_point_right_contraction(Dipole self, RadialPoint other) {
    return RadialPoint(vec3(self.g1.y) * other.g0.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * other.g0.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g1.x) * other.g0.xzy * vec3(0.0, 1.0, -1.0), vec2(self.g0.y) * vec2(other.g0.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(1.0, 0.0) + vec2(self.g2.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(1.0, 0.0));
}

FlatPoint dipole_flat_point_into(Dipole self) {
    return FlatPoint(self.g2);
}

Dipole dipole_flat_point_add(Dipole self, FlatPoint other) {
    return Dipole(self.g0, self.g1, self.g2 + other.g0);
}

Dipole dipole_flat_point_sub(Dipole self, FlatPoint other) {
    return Dipole(self.g0, self.g1, self.g2 - other.g0);
}

Flector dipole_flat_point_geometric_product(Dipole self, FlatPoint other) {
    return Flector(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g0.x, self.g1.x, self.g1.x, self.g0.x) * other.g0.xzyx * vec4(0.0, 1.0, -1.0, 1.0), vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.x, self.g0.x, self.g0.x, self.g1.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0));
}

Plane dipole_flat_point_outer_product(Dipole self, FlatPoint other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.x, self.g0.x, self.g0.x, self.g1.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0));
}

AntiScalar dipole_flat_point_inner_anti_product(Dipole self, FlatPoint other) {
    return AntiScalar(0.0 - self.g2.w * other.g0.w);
}

AntiScalar dipole_flat_point_left_anti_contraction(Dipole self, FlatPoint other) {
    return AntiScalar(0.0 - self.g2.w * other.g0.w);
}

AntiScalar dipole_flat_point_right_anti_contraction(Dipole self, FlatPoint other) {
    return AntiScalar(0.0 - self.g2.w * other.g0.w);
}

AntiScalar dipole_flat_point_anti_scalar_product(Dipole self, FlatPoint other) {
    return AntiScalar(0.0 - self.g2.w * other.g0.w);
}

Dipole dipole_dipole_add(Dipole self, Dipole other) {
    return Dipole(self.g0 + other.g0, self.g1 + other.g1, self.g2 + other.g2);
}

Dipole dipole_dipole_sub(Dipole self, Dipole other) {
    return Dipole(self.g0 - other.g0, self.g1 - other.g1, self.g2 - other.g2);
}

Dipole dipole_dipole_mul(Dipole self, Dipole other) {
    return Dipole(self.g0 * other.g0, self.g1 * other.g1, self.g2 * other.g2);
}

Dipole dipole_dipole_div(Dipole self, Dipole other) {
    return Dipole(vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(1.0, 1.0, 1.0) / vec3(other.g0.x, other.g0.y, other.g0.z) * vec3(1.0, 1.0, 1.0), vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(1.0, 1.0, 1.0) / vec3(other.g1.x, other.g1.y, other.g1.z) * vec3(1.0, 1.0, 1.0), vec4(self.g2.x, self.g2.y, self.g2.z, self.g2.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g2.x, other.g2.y, other.g2.z, other.g2.w) * vec4(1.0, 1.0, 1.0, 1.0));
}

Sphere dipole_dipole_outer_product(Dipole self, Dipole other) {
    return Sphere(vec2(self.g0.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) - vec2(self.g1.x) * vec2(other.g0.x, other.g2.x) - vec2(self.g1.y) * vec2(other.g0.y, other.g2.y) - vec2(self.g1.z) * vec2(other.g0.z, other.g2.z) + vec2(self.g2.x) * vec2(other.g1.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(-1.0, 0.0), vec3(self.g0.y) * vec3(other.g2.z, other.g2.z, other.g2.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g2.y, other.g2.x, other.g2.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.y) * vec3(other.g2.w) * vec3(0.0, 1.0, 0.0) + vec3(self.g1.z) * vec3(other.g2.w) * vec3(0.0, 0.0, 1.0) + vec3(self.g2.x) * other.g0.zzy * vec3(0.0, 1.0, -1.0) + vec3(self.g2.y) * other.g0.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g2.z) * other.g0.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g2.w) * other.g1 + vec3(self.g1.x, self.g0.x, self.g0.x) * vec3(other.g2.w, other.g2.z, other.g2.y) * vec3(1.0, -1.0, 1.0));
}

Scalar dipole_dipole_inner_product(Dipole self, Dipole other) {
    return Scalar(0.0 - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z);
}

AntiScalar dipole_dipole_inner_anti_product(Dipole self, Dipole other) {
    return AntiScalar(0.0 - self.g2.w * other.g2.w);
}

Scalar dipole_dipole_left_contraction(Dipole self, Dipole other) {
    return Scalar(0.0 - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z);
}

Scalar dipole_dipole_right_contraction(Dipole self, Dipole other) {
    return Scalar(0.0 - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z);
}

AntiScalar dipole_dipole_left_anti_contraction(Dipole self, Dipole other) {
    return AntiScalar(0.0 - self.g2.w * other.g2.w);
}

AntiScalar dipole_dipole_right_anti_contraction(Dipole self, Dipole other) {
    return AntiScalar(0.0 - self.g2.w * other.g2.w);
}

Scalar dipole_dipole_scalar_product(Dipole self, Dipole other) {
    return Scalar(0.0 - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z);
}

AntiScalar dipole_dipole_anti_scalar_product(Dipole self, Dipole other) {
    return AntiScalar(0.0 - self.g2.w * other.g2.w);
}

Motor dipole_line_geometric_product(Dipole self, Line other) {
    return Motor(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g0.z, other.g0.z, other.g0.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0), vec4(self.g1.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

Scalar dipole_line_regressive_product(Dipole self, Line other) {
    return Scalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

AntiScalar dipole_line_outer_product(Dipole self, Line other) {
    return AntiScalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

Sphere dipole_line_inner_anti_product(Dipole self, Line other) {
    return Sphere(vec2(self.g0.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g2.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(-1.0, 0.0), vec3(self.g2.w) * other.g0);
}

Sphere dipole_line_right_anti_contraction(Dipole self, Line other) {
    return Sphere(vec2(self.g0.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g2.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(-1.0, 0.0), vec3(self.g2.w) * other.g0);
}

Scalar dipole_circle_regressive_product(Dipole self, Circle other) {
    return Scalar(0.0 - self.g0.x * other.g2.x - self.g0.y * other.g2.y - self.g0.z * other.g2.z - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z - self.g2.x * other.g0.x - self.g2.y * other.g0.y - self.g2.z * other.g0.z - self.g2.w * other.g0.w);
}

AntiScalar dipole_circle_outer_product(Dipole self, Circle other) {
    return AntiScalar(0.0 - self.g0.x * other.g2.x - self.g0.y * other.g2.y - self.g0.z * other.g2.z - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z - self.g2.x * other.g0.x - self.g2.y * other.g0.y - self.g2.z * other.g0.z - self.g2.w * other.g0.w);
}

RadialPoint dipole_circle_inner_product(Dipole self, Circle other) {
    return RadialPoint(self.g1 * vec3(other.g0.w), vec2(0.0) - vec2(self.g1.x) * vec2(other.g0.x, other.g2.x) - vec2(self.g1.y) * vec2(other.g0.y, other.g2.y) - vec2(self.g1.z) * vec2(other.g0.z, other.g2.z));
}

Sphere dipole_circle_inner_anti_product(Dipole self, Circle other) {
    return Sphere(vec2(self.g0.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g2.x) * vec2(other.g1.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(-1.0, 0.0), vec3(self.g2.w) * other.g1);
}

RadialPoint dipole_circle_left_contraction(Dipole self, Circle other) {
    return RadialPoint(self.g1 * vec3(other.g0.w), vec2(0.0) - vec2(self.g1.x) * vec2(other.g0.x, other.g2.x) - vec2(self.g1.y) * vec2(other.g0.y, other.g2.y) - vec2(self.g1.z) * vec2(other.g0.z, other.g2.z));
}

Sphere dipole_circle_right_anti_contraction(Dipole self, Circle other) {
    return Sphere(vec2(self.g0.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g2.x) * vec2(other.g1.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(-1.0, 0.0), vec3(self.g2.w) * other.g1);
}

RadialPoint dipole_plane_regressive_product(Dipole self, Plane other) {
    return RadialPoint(vec3(self.g1.x) * vec3(other.g0.z, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + self.g0 * vec3(other.g0.w), vec2(self.g0.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g2.x) * vec2(other.g0.x) * vec2(0.0, 1.0) + vec2(self.g2.y) * vec2(other.g0.y) * vec2(0.0, 1.0) + vec2(self.g2.z) * vec2(other.g0.z) * vec2(0.0, 1.0) + vec2(self.g2.w) * vec2(other.g0.w) * vec2(0.0, 1.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(-1.0, 0.0));
}

FlatPoint dipole_plane_inner_product(Dipole self, Plane other) {
    return FlatPoint(vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Circle dipole_plane_inner_anti_product(Dipole self, Plane other) {
    return Circle(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x, self.g0.x, self.g0.x, self.g1.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, -1.0), vec3(0.0) - vec3(self.g2.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g2.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g2.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g2.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0));
}

FlatPoint dipole_plane_left_contraction(Dipole self, Plane other) {
    return FlatPoint(vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Circle dipole_plane_right_anti_contraction(Dipole self, Plane other) {
    return Circle(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x, self.g0.x, self.g0.x, self.g1.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, -1.0), vec3(0.0) - vec3(self.g2.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec3(self.g2.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g2.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g2.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0));
}

RadialPoint dipole_sphere_regressive_product(Dipole self, Sphere other) {
    return RadialPoint(vec3(self.g1.x) * other.g1.zzy * vec3(0.0, -1.0, 1.0) + vec3(self.g1.y) * other.g1.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * other.g1.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g2.x) * vec3(other.g0.x) * vec3(-1.0, 0.0, 0.0) + vec3(self.g2.y) * vec3(other.g0.x) * vec3(0.0, -1.0, 0.0) + vec3(self.g2.z) * vec3(other.g0.x) * vec3(0.0, 0.0, -1.0) + self.g0 * vec3(other.g0.y), vec2(self.g0.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g2.x) * vec2(other.g1.x) * vec2(0.0, 1.0) + vec2(self.g2.y) * vec2(other.g1.y) * vec2(0.0, 1.0) + vec2(self.g2.z) * vec2(other.g1.z) * vec2(0.0, 1.0) + vec2(self.g2.w) * other.g0 * vec2(-1.0, 1.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(-1.0, 0.0));
}

Circle dipole_sphere_inner_anti_product(Dipole self, Sphere other) {
    return Circle(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.z) * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.y) * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x, self.g0.x, self.g0.x, self.g1.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g1.x) * vec4(0.0, -1.0, 1.0, -1.0), vec3(0.0) - vec3(self.g2.w) * other.g1, vec3(self.g2.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g2.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g2.x) * other.g1.xzy * vec3(0.0, 1.0, -1.0));
}

Circle dipole_sphere_right_anti_contraction(Dipole self, Sphere other) {
    return Circle(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.z) * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.y) * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x, self.g0.x, self.g0.x, self.g1.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g1.x) * vec4(0.0, -1.0, 1.0, -1.0), vec3(0.0) - vec3(self.g2.w) * other.g1, vec3(self.g2.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g2.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g2.x) * other.g1.xzy * vec3(0.0, 1.0, -1.0));
}

Motor dipole_motor_geometric_product(Dipole self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g1.wzyx * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.y) * other.g1.zwxy * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g1.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g1.x) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * other.g0.zwxy * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * other.g0.yxwz * vec4(1.0, -1.0, 1.0, -1.0), vec4(self.g1.x) * other.g1.wzyx * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * other.g1.zwxy * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * other.g1.yxwz * vec4(1.0, -1.0, 1.0, -1.0));
}

Rotor dipole_rotor_geometric_product(Dipole self, Rotor other) {
    return Rotor(vec4(self.g1.x) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * other.g0.zwxy * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * other.g0.yxwz * vec4(1.0, -1.0, 1.0, -1.0));
}

AntiScalar dipole_rotor_outer_product(Dipole self, Rotor other) {
    return AntiScalar(0.0 - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

Motor dipole_translator_geometric_product(Dipole self, Translator other) {
    return Motor(vec4(self.g0.y) * other.g0.zzxy * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.yxyz * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.w) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.w) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.x, self.g0.x, self.g0.x, self.g0.x) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, -1.0), vec4(self.g1.y) * other.g0.zzxy * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * other.g0.yxyz * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.x) * other.g0.xzyx * vec4(0.0, 1.0, -1.0, -1.0));
}

AntiScalar dipole_translator_outer_product(Dipole self, Translator other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

Dipole dipole_translator_inner_anti_product(Dipole self, Translator other) {
    return Dipole(self.g0 * vec3(other.g0.w), self.g1 * vec3(other.g0.w), self.g2 * vec4(other.g0.w));
}

Dipole dipole_translator_right_anti_contraction(Dipole self, Translator other) {
    return Dipole(self.g0 * vec3(other.g0.w), self.g1 * vec3(other.g0.w), self.g2 * vec4(other.g0.w));
}

Flector dipole_flector_geometric_product(Dipole self, Flector other) {
    return Flector(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.z, other.g1.w, other.g0.x, other.g1.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g1.w, other.g1.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0), vec4(self.g0.y) * vec4(other.g0.z, other.g1.w, other.g0.x, other.g0.z) * vec4(1.0, -1.0, -1.0, 0.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g1.w, other.g0.y) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.x) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4(-1.0, -1.0, 1.0, 0.0));
}

RadialPoint dipole_flector_regressive_product(Dipole self, Flector other) {
    return RadialPoint(vec3(self.g1.x) * vec3(other.g1.z, other.g1.z, other.g1.y) * vec3(0.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(-1.0, 1.0, 0.0) + self.g0 * vec3(other.g1.w), vec2(self.g0.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g2.x) * vec2(other.g1.x) * vec2(0.0, 1.0) + vec2(self.g2.y) * vec2(other.g1.y) * vec2(0.0, 1.0) + vec2(self.g2.z) * vec2(other.g1.z) * vec2(0.0, 1.0) + vec2(self.g2.w) * vec2(other.g1.w) * vec2(0.0, 1.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(-1.0, 0.0));
}

Plane dipole_flector_outer_product(Dipole self, Flector other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.x, self.g0.x, self.g0.x, self.g1.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0));
}

FlatPoint dipole_flector_inner_product(Dipole self, Flector other) {
    return FlatPoint(vec4(self.g1.y) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.x) * other.g1.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

FlatPoint dipole_flector_left_contraction(Dipole self, Flector other) {
    return FlatPoint(vec4(self.g1.y) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.x) * other.g1.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

AntiScalar dipole_flector_left_anti_contraction(Dipole self, Flector other) {
    return AntiScalar(0.0 - self.g2.w * other.g0.w);
}

AntiScalar dipole_flector_anti_scalar_product(Dipole self, Flector other) {
    return AntiScalar(0.0 - self.g2.w * other.g0.w);
}

AntiScalar dipole_dilation_outer_product(Dipole self, Dilation other) {
    return AntiScalar(self.g2.x * other.g1.x + self.g2.y * other.g0.x + self.g2.z * other.g0.y + self.g2.w * other.g0.z);
}

Dipole dipole_dilation_inner_anti_product(Dipole self, Dilation other) {
    return Dipole(self.g0 * vec3(other.g1.y), self.g1 * vec3(other.g1.y), self.g2 * vec4(other.g1.y));
}

Dipole dipole_dilation_right_anti_contraction(Dipole self, Dilation other) {
    return Dipole(self.g0 * vec3(other.g1.y), self.g1 * vec3(other.g1.y), self.g2 * vec4(other.g1.y));
}

MultiVector dipole_multi_vector_add(Dipole self, MultiVector other) {
    return MultiVector(other.g0, other.g1, other.g2, self.g2 + other.g3, self.g0 + other.g4, self.g1 + other.g5, other.g6, other.g7, other.g8, other.g9);
}

MultiVector dipole_multi_vector_sub(Dipole self, MultiVector other) {
    return MultiVector(vec3(0.0) - other.g0, vec3(0.0) - other.g1, vec2(0.0) - other.g2, self.g2 - other.g3, self.g0 - other.g4, self.g1 - other.g5, vec3(0.0) - other.g6, vec3(0.0) - other.g7, vec4(0.0) - other.g8, vec4(0.0) - other.g9);
}

MultiVector dipole_multi_vector_geometric_product(Dipole self, MultiVector other) {
    return MultiVector(vec3(self.g0.y) * vec3(other.g5.y, other.g5.y, other.g7.y) * vec3(0.0, -1.0, -1.0) + vec3(self.g0.z) * vec3(other.g5.z, other.g5.z, other.g7.z) * vec3(0.0, -1.0, -1.0) - vec3(self.g1.x) * vec3(other.g5.x, other.g4.x, other.g6.x) - vec3(self.g1.y) * vec3(other.g5.y, other.g4.y, other.g6.y) - vec3(self.g1.z) * vec3(other.g5.z, other.g4.z, other.g6.z) + vec3(self.g2.x) * vec3(other.g8.x) * vec3(0.0, 0.0, -1.0) + vec3(self.g2.y) * vec3(other.g8.y) * vec3(0.0, 0.0, -1.0) + vec3(self.g2.z) * vec3(other.g8.z) * vec3(0.0, 0.0, -1.0) + vec3(self.g2.w) * vec3(other.g8.w) * vec3(0.0, 0.0, -1.0) + vec3(self.g0.x) * vec3(other.g5.x, other.g5.x, other.g7.x) * vec3(0.0, -1.0, -1.0), vec3(self.g1.x) * vec3(other.g8.w, other.g1.z, other.g1.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.y) * vec3(other.g1.z, other.g8.w, other.g1.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g1.z) * vec3(other.g1.y, other.g1.x, other.g8.w) * vec3(1.0, -1.0, 1.0), vec2(self.g0.y) * vec2(other.g1.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(1.0, 0.0) - vec2(self.g1.x) * vec2(other.g8.x, other.g7.x) - vec2(self.g1.y) * vec2(other.g8.y, other.g7.y) - vec2(self.g1.z) * vec2(other.g8.z, other.g7.z) + vec2(self.g2.x) * vec2(other.g1.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(1.0, 0.0), vec4(self.g0.y) * vec4(other.g3.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g3.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g9.w, other.g3.z, other.g3.y, other.g9.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g3.z, other.g9.w, other.g3.x, other.g9.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g3.y, other.g3.x, other.g9.w, other.g9.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g2.x) * vec4(other.g0.x, other.g5.z, other.g5.y, other.g4.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g2.y) * vec4(other.g5.z, other.g0.x, other.g5.x, other.g4.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g2.z) * vec4(other.g5.y, other.g5.x, other.g0.x, other.g4.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g2.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g3.x) * vec4(0.0, 0.0, 0.0, 1.0), vec3(self.g0.x) * vec3(other.g0.x, other.g5.z, other.g5.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.y) * vec3(other.g5.z, other.g0.x, other.g5.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g0.z) * vec3(other.g5.y, other.g5.x, other.g0.x) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.x) * vec3(other.g0.y, other.g4.z, other.g4.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.y) * vec3(other.g4.z, other.g0.y, other.g4.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g1.z) * vec3(other.g4.y, other.g4.x, other.g0.y) * vec3(1.0, -1.0, 1.0), vec3(self.g1.x) * vec3(other.g0.x, other.g5.z, other.g5.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.y) * vec3(other.g5.z, other.g0.x, other.g5.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g1.z) * vec3(other.g5.y, other.g5.x, other.g0.x) * vec3(1.0, -1.0, 1.0), vec3(self.g0.x) * vec3(other.g2.y, other.g7.z, other.g7.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.y) * vec3(other.g7.z, other.g2.y, other.g7.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g0.z) * vec3(other.g7.y, other.g7.x, other.g2.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.x) * vec3(other.g0.z, other.g6.z, other.g6.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.y) * vec3(other.g6.z, other.g0.z, other.g6.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g1.z) * vec3(other.g6.y, other.g6.x, other.g0.z) * vec3(1.0, -1.0, 1.0) + vec3(self.g2.x) * vec3(other.g2.x, other.g8.z, other.g8.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g2.y) * vec3(other.g8.z, other.g2.x, other.g8.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g2.z) * vec3(other.g8.y, other.g8.x, other.g2.x) * vec3(1.0, -1.0, 1.0) - vec3(self.g2.w) * other.g1, vec3(self.g1.x) * vec3(other.g2.y, other.g7.z, other.g7.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.y) * vec3(other.g7.z, other.g2.y, other.g7.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g1.z) * vec3(other.g7.y, other.g7.x, other.g2.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g2.x) * vec3(other.g8.w, other.g1.z, other.g1.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g2.y) * vec3(other.g1.z, other.g8.w, other.g1.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g2.z) * vec3(other.g1.y, other.g1.x, other.g8.w) * vec3(1.0, -1.0, 1.0), vec4(self.g0.y) * vec4(other.g1.z, other.g8.w, other.g1.x, other.g1.z) * vec4(1.0, -1.0, -1.0, 0.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g8.w, other.g1.y) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g1.x) * vec4(other.g2.x, other.g8.z, other.g8.y, other.g1.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g8.z, other.g2.x, other.g8.x, other.g1.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g8.y, other.g8.x, other.g2.x, other.g1.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.x) * vec4(other.g8.w, other.g1.z, other.g1.y, other.g8.x) * vec4(-1.0, -1.0, 1.0, 0.0), vec4(self.g0.y) * vec4(other.g3.z, other.g9.w, other.g3.x, other.g3.z) * vec4(1.0, -1.0, -1.0, 0.0) + vec4(self.g0.z) * vec4(other.g3.y, other.g3.x, other.g9.w, other.g3.y) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g1.x) * vec4(other.g3.w, other.g9.z, other.g9.y, other.g3.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g9.z, other.g3.w, other.g9.x, other.g3.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g9.y, other.g9.x, other.g3.w, other.g3.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g2.x) * vec4(other.g0.y, other.g4.z, other.g4.y, other.g5.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g2.y) * vec4(other.g4.z, other.g0.y, other.g4.x, other.g5.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g2.z) * vec4(other.g4.y, other.g4.x, other.g0.y, other.g5.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g2.w) * vec4(other.g5.x, other.g5.y, other.g5.z, other.g5.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g9.w, other.g3.z, other.g3.y, other.g9.x) * vec4(-1.0, -1.0, 1.0, 0.0));
}

MultiVector dipole_multi_vector_geometric_anti_product(Dipole self, MultiVector other) {
    return MultiVector(vec3(self.g0.y) * vec3(other.g7.y, other.g6.y, other.g7.y) * vec3(-1.0, -1.0, 0.0) + vec3(self.g0.z) * vec3(other.g7.z, other.g6.z, other.g7.z) * vec3(-1.0, -1.0, 0.0) + vec3(self.g1.x) * vec3(other.g6.x) * vec3(-1.0, 0.0, 0.0) + vec3(self.g1.y) * vec3(other.g6.y) * vec3(-1.0, 0.0, 0.0) + vec3(self.g1.z) * vec3(other.g6.z) * vec3(-1.0, 0.0, 0.0) + vec3(self.g2.x) * vec3(other.g8.x) * vec3(-1.0, 0.0, 0.0) + vec3(self.g2.y) * vec3(other.g8.y) * vec3(-1.0, 0.0, 0.0) + vec3(self.g2.z) * vec3(other.g8.z) * vec3(-1.0, 0.0, 0.0) + vec3(self.g2.w) * vec3(other.g8.w, other.g2.x, other.g3.w) * vec3(-1.0, 1.0, -1.0) + vec3(self.g0.x) * vec3(other.g7.x, other.g6.x, other.g7.x) * vec3(-1.0, -1.0, 0.0), vec3(self.g0.x) * vec3(other.g9.w, other.g3.z, other.g3.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g3.z, other.g9.w, other.g3.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g3.y, other.g3.x, other.g9.w) * vec3(-1.0, 1.0, 1.0) + vec3(self.g1.x) * vec3(other.g3.w, other.g9.z, other.g9.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g9.z, other.g3.w, other.g9.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.z) * vec3(other.g9.y, other.g9.x, other.g3.w) * vec3(-1.0, 1.0, 1.0) + vec3(self.g2.x) * vec3(other.g0.y, other.g4.z, other.g4.y) * vec3(-1.0, 1.0, -1.0) + vec3(self.g2.y) * vec3(other.g4.z, other.g0.y, other.g4.x) * vec3(-1.0, -1.0, 1.0) + vec3(self.g2.z) * vec3(other.g4.y, other.g4.x, other.g0.y) * vec3(1.0, -1.0, -1.0) + vec3(self.g2.w) * other.g5, vec2(self.g0.y) * vec2(other.g9.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g9.z) * vec2(-1.0, 0.0) + vec2(self.g2.x) * vec2(other.g9.x) * vec2(0.0, 1.0) + vec2(self.g2.y) * vec2(other.g9.y) * vec2(0.0, 1.0) + vec2(self.g2.z) * vec2(other.g9.z) * vec2(0.0, 1.0) + vec2(self.g2.w) * vec2(other.g0.y, other.g9.w) * vec2(-1.0, 1.0) + vec2(self.g0.x) * vec2(other.g9.x) * vec2(-1.0, 0.0), vec4(self.g2.y) * vec4(other.g6.z, other.g0.z, other.g6.x, other.g6.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g2.z) * vec4(other.g6.y, other.g6.x, other.g0.z, other.g6.y) * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g2.w) * vec4(other.g7.x, other.g7.y, other.g7.z, other.g0.z) * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g2.x) * vec4(other.g0.z, other.g6.z, other.g6.y, other.g0.x) * vec4(1.0, -1.0, 1.0, 0.0), vec3(self.g0.x) * vec3(other.g0.z, other.g6.z, other.g6.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g6.z, other.g0.z, other.g6.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g6.y, other.g6.x, other.g0.z) * vec3(-1.0, 1.0, 1.0) + vec3(self.g2.w) * vec3(other.g8.x, other.g8.y, other.g8.z), vec3(self.g0.x) * vec3(other.g2.y, other.g7.z, other.g7.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g7.z, other.g2.y, other.g7.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g7.y, other.g7.x, other.g2.y) * vec3(-1.0, 1.0, 1.0) + vec3(self.g1.x) * vec3(other.g0.z, other.g6.z, other.g6.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g6.z, other.g0.z, other.g6.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.z) * vec3(other.g6.y, other.g6.x, other.g0.z) * vec3(-1.0, 1.0, 1.0) + vec3(self.g2.x) * vec3(other.g2.x, other.g8.z, other.g8.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g2.y) * vec3(other.g8.z, other.g2.x, other.g8.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g2.z) * vec3(other.g8.y, other.g8.x, other.g2.x) * vec3(-1.0, 1.0, 1.0) - vec3(self.g2.w) * other.g1, vec3(0.0) - vec3(self.g2.w) * vec3(other.g9.x, other.g9.y, other.g9.z), vec3(self.g2.x) * vec3(other.g3.w, other.g9.z, other.g9.y) * vec3(-1.0, 1.0, -1.0) + vec3(self.g2.y) * vec3(other.g9.z, other.g3.w, other.g9.x) * vec3(-1.0, -1.0, 1.0) + vec3(self.g2.z) * vec3(other.g9.y, other.g9.x, other.g3.w) * vec3(1.0, -1.0, -1.0) + vec3(self.g2.w) * vec3(other.g3.x, other.g3.y, other.g3.z), vec4(self.g0.x) * vec4(other.g3.w, other.g9.z, other.g9.y, other.g3.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g9.z, other.g3.w, other.g9.x, other.g3.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g9.y, other.g9.x, other.g3.w, other.g3.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.y) * vec4(other.g9.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g9.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g2.x) * vec4(other.g4.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g2.y) * vec4(other.g4.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g2.z) * vec4(other.g4.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g2.w) * vec4(other.g4.x, other.g4.y, other.g4.z, other.g0.x) * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g1.x) * vec4(other.g9.x) * vec4(0.0, 0.0, 0.0, -1.0), vec4(self.g2.y) * vec4(other.g6.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g2.z) * vec4(other.g6.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g2.w) * vec4(other.g6.x, other.g6.y, other.g6.z, other.g2.y) * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g2.x) * vec4(other.g6.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

Scalar dipole_multi_vector_scalar_product(Dipole self, MultiVector other) {
    return Scalar(0.0 - self.g1.x * other.g5.x - self.g1.y * other.g5.y - self.g1.z * other.g5.z);
}

AntiScalar dipole_multi_vector_anti_scalar_product(Dipole self, MultiVector other) {
    return AntiScalar(0.0 - self.g2.w * other.g3.w);
}

Scalar dipole_squared_magnitude(Dipole self) {
    return dipole_dipole_scalar_product(self, dipole_reversal(self));
}

Scalar dipole_magnitude(Dipole self) {
    return Scalar(sqrt(dipole_squared_magnitude(self).g0));
}

Scalar dipole_bulk_norm(Dipole self) {
    return Scalar(sqrt(dipole_squared_magnitude(self).g0));
}

AntiScalar dipole_squared_anti_magnitude(Dipole self) {
    return dipole_dipole_anti_scalar_product(self, dipole_anti_reversal(self));
}

AntiScalar dipole_weight_norm(Dipole self) {
    return AntiScalar(sqrt(dipole_squared_anti_magnitude(self).g0));
}

HomogeneousMagnitude dipole_geometric_norm(Dipole self) {
    return scalar_anti_scalar_add(dipole_bulk_norm(self), dipole_weight_norm(self));
}

Dipole dipole_scale(Dipole self, float other) {
    return dipole_scalar_geometric_product(self, Scalar(other));
}

Dipole dipole_signum(Dipole self) {
    return dipole_scalar_geometric_product(self, Scalar(1.0 / dipole_magnitude(self).g0));
}

Dipole dipole_inverse(Dipole self) {
    return dipole_scalar_geometric_product(dipole_reversal(self), Scalar(1.0 / dipole_squared_magnitude(self).g0));
}

Dipole dipole_unitize(Dipole self) {
    return dipole_scalar_geometric_product(self, Scalar(1.0 / dipole_weight_norm(self).g0));
}

Scalar dipole_attitude(Dipole self) {
    return dipole_circle_regressive_product(self, Circle(vec4(0.0, 0.0, 0.0, 1.0), vec3(0.0), vec3(0.0)));
}

Line line_zero() {
    return Line(vec3(0.0), vec3(0.0));
}

Line line_one() {
    return Line(vec3(0.0), vec3(0.0));
}

int line_grade(Line self) {
    return 3;
}

int line_anti_grade(Line self) {
    return 2;
}

Line line_neg(Line self) {
    return Line(self.g0 * vec3(-1.0), self.g1 * vec3(-1.0));
}

Line line_automorphism(Line self) {
    return Line(self.g0 * vec3(-1.0), self.g1 * vec3(-1.0));
}

Line line_reversal(Line self) {
    return Line(self.g0 * vec3(-1.0), self.g1 * vec3(-1.0));
}

Line line_conjugation(Line self) {
    return Line(self.g0, self.g1);
}

Line line_anti_reversal(Line self) {
    return Line(self.g0, self.g1);
}

Line line_double_complement(Line self) {
    return Line(self.g0, self.g1);
}

Line line_scalar_geometric_product(Line self, Scalar other) {
    return Line(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0));
}

Line line_scalar_outer_product(Line self, Scalar other) {
    return Line(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0));
}

Line line_scalar_inner_product(Line self, Scalar other) {
    return Line(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0));
}

Line line_scalar_right_contraction(Line self, Scalar other) {
    return Line(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0));
}

Line line_anti_scalar_regressive_product(Line self, AntiScalar other) {
    return Line(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0));
}

Line line_anti_scalar_geometric_anti_product(Line self, AntiScalar other) {
    return Line(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0));
}

Line line_anti_scalar_inner_anti_product(Line self, AntiScalar other) {
    return Line(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0));
}

Line line_anti_scalar_right_anti_contraction(Line self, AntiScalar other) {
    return Line(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0));
}

Line line_homogeneous_magnitude_geometric_product(Line self, HomogeneousMagnitude other) {
    return Line(self.g0 * vec3(other.g0.x), self.g1 * vec3(other.g0.x));
}

Line line_homogeneous_magnitude_regressive_product(Line self, HomogeneousMagnitude other) {
    return Line(self.g0 * vec3(other.g0.y), self.g1 * vec3(other.g0.y));
}

Line line_homogeneous_magnitude_outer_product(Line self, HomogeneousMagnitude other) {
    return Line(self.g0 * vec3(other.g0.x), self.g1 * vec3(other.g0.x));
}

Line line_homogeneous_magnitude_inner_product(Line self, HomogeneousMagnitude other) {
    return Line(self.g0 * vec3(other.g0.x), self.g1 * vec3(other.g0.x));
}

Line line_homogeneous_magnitude_right_contraction(Line self, HomogeneousMagnitude other) {
    return Line(self.g0 * vec3(other.g0.x), self.g1 * vec3(other.g0.x));
}

Line line_homogeneous_magnitude_right_anti_contraction(Line self, HomogeneousMagnitude other) {
    return Line(self.g0 * vec3(other.g0.y), self.g1 * vec3(other.g0.y));
}

Flector line_radial_point_geometric_product(Line self, RadialPoint other) {
    return Flector(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g0.z) * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.y) * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x, self.g1.x, self.g1.x, self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g0.x) * vec4(0.0, -1.0, 1.0, -1.0), vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g0.z) * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.y) * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g1.x) * vec4(other.g1.x, other.g1.x, other.g1.x, other.g0.x) * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g1.y) * vec4(other.g1.x, other.g1.x, other.g1.x, other.g0.y) * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g1.z) * vec4(other.g1.x, other.g1.x, other.g1.x, other.g0.z) * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g0.x) * vec4(0.0, 1.0, -1.0, 0.0));
}

Plane line_radial_point_outer_product(Line self, RadialPoint other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g0.z) * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.y) * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g1.x) * vec4(other.g1.x, other.g1.x, other.g1.x, other.g0.x) * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g1.y) * vec4(other.g1.x, other.g1.x, other.g1.x, other.g0.y) * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g1.z) * vec4(other.g1.x, other.g1.x, other.g1.x, other.g0.z) * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g0.x) * vec4(0.0, 1.0, -1.0, 0.0));
}

FlatPoint line_radial_point_inner_product(Line self, RadialPoint other) {
    return FlatPoint(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g0.z) * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.y) * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x, self.g1.x, self.g1.x, self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g0.x) * vec4(0.0, -1.0, 1.0, -1.0));
}

FlatPoint line_radial_point_right_contraction(Line self, RadialPoint other) {
    return FlatPoint(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g0.z) * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.y) * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x, self.g1.x, self.g1.x, self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g0.x) * vec4(0.0, -1.0, 1.0, -1.0));
}

Plane line_flat_point_inner_anti_product(Line self, FlatPoint other) {
    return Plane(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Plane line_flat_point_left_anti_contraction(Line self, FlatPoint other) {
    return Plane(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Motor line_dipole_geometric_product(Line self, Dipole other) {
    return Motor(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g0.z, other.g0.z, other.g0.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0), vec4(self.g1.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

Scalar line_dipole_regressive_product(Line self, Dipole other) {
    return Scalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

AntiScalar line_dipole_outer_product(Line self, Dipole other) {
    return AntiScalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

Sphere line_dipole_inner_anti_product(Line self, Dipole other) {
    return Sphere(vec2(0.0) - vec2(self.g0.x) * vec2(other.g0.x, other.g2.x) - vec2(self.g0.y) * vec2(other.g0.y, other.g2.y) - vec2(self.g0.z) * vec2(other.g0.z, other.g2.z), self.g0 * vec3(other.g2.w));
}

Sphere line_dipole_left_anti_contraction(Line self, Dipole other) {
    return Sphere(vec2(0.0) - vec2(self.g0.x) * vec2(other.g0.x, other.g2.x) - vec2(self.g0.y) * vec2(other.g0.y, other.g2.y) - vec2(self.g0.z) * vec2(other.g0.z, other.g2.z), self.g0 * vec3(other.g2.w));
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

Motor line_line_geometric_anti_product(Line self, Line other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g0.y) * vec4(1.0, 0.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.z) * vec4(-1.0, 1.0, 0.0, -1.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g0.x) * vec4(0.0, -1.0, 1.0, -1.0), vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.y) * vec4(1.0, 0.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.z) * vec4(-1.0, 1.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g0.z, other.g0.z, other.g0.y, other.g0.x) * vec4(0.0, -1.0, 1.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g0.y) * vec4(1.0, 0.0, -1.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.z) * vec4(-1.0, 1.0, 0.0, -1.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g1.x) * vec4(0.0, -1.0, 1.0, -1.0));
}

AntiScalar line_line_inner_anti_product(Line self, Line other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

AntiScalar line_line_left_anti_contraction(Line self, Line other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

AntiScalar line_line_right_anti_contraction(Line self, Line other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

AntiScalar line_line_anti_scalar_product(Line self, Line other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

Circle line_circle_add(Line self, Circle other) {
    return Circle(other.g0, self.g0 + other.g1, self.g1 + other.g2);
}

Circle line_circle_sub(Line self, Circle other) {
    return Circle(vec4(0.0) - other.g0, self.g0 - other.g1, self.g1 - other.g2);
}

RadialPoint line_circle_regressive_product(Line self, Circle other) {
    return RadialPoint(vec3(self.g1.x) * vec3(other.g0.z, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + self.g0 * vec3(other.g0.w), vec2(0.0) - vec2(self.g0.x) * vec2(other.g0.x, other.g2.x) - vec2(self.g0.y) * vec2(other.g0.y, other.g2.y) - vec2(self.g0.z) * vec2(other.g0.z, other.g2.z) + vec2(self.g1.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g1.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g1.x) * vec2(other.g1.x) * vec2(0.0, -1.0));
}

AntiScalar line_circle_inner_anti_product(Line self, Circle other) {
    return AntiScalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z);
}

AntiScalar line_circle_left_anti_contraction(Line self, Circle other) {
    return AntiScalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z);
}

AntiScalar line_circle_right_anti_contraction(Line self, Circle other) {
    return AntiScalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z);
}

AntiScalar line_circle_anti_scalar_product(Line self, Circle other) {
    return AntiScalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z);
}

FlatPoint line_plane_regressive_product(Line self, Plane other) {
    return FlatPoint(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x, self.g1.x, self.g1.x, self.g0.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0));
}

Flector line_plane_geometric_anti_product(Line self, Plane other) {
    return Flector(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x, self.g1.x, self.g1.x, self.g0.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0), vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x, self.g0.x, self.g0.x, self.g1.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, -1.0));
}

Plane line_plane_inner_anti_product(Line self, Plane other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x, self.g0.x, self.g0.x, self.g1.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, -1.0));
}

Plane line_plane_right_anti_contraction(Line self, Plane other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x, self.g0.x, self.g0.x, self.g1.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, -1.0));
}

Dipole line_sphere_regressive_product(Line self, Sphere other) {
    return Dipole(self.g0 * vec3(other.g0.x), self.g1 * vec3(other.g0.x), vec4(self.g0.y) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g1.y) * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g1.z) * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.x) * vec4(other.g1.z, other.g1.z, other.g1.y, other.g1.z) * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g1.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.z) * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.y) * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.y, other.g0.x, other.g0.x, other.g1.x) * vec4(1.0, 0.0, 0.0, -1.0));
}

Plane line_sphere_inner_anti_product(Line self, Sphere other) {
    return Plane(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.z) * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.y) * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x, self.g0.x, self.g0.x, self.g1.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g1.x) * vec4(0.0, -1.0, 1.0, -1.0));
}

Plane line_sphere_right_anti_contraction(Line self, Sphere other) {
    return Plane(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.z) * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.y) * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x, self.g0.x, self.g0.x, self.g1.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g1.x) * vec4(0.0, -1.0, 1.0, -1.0));
}

Motor line_motor_add(Line self, Motor other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + other.g0, vec4(self.g1.x, self.g1.y, self.g1.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + other.g1);
}

Motor line_motor_sub(Line self, Motor other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) - other.g0, vec4(self.g1.x, self.g1.y, self.g1.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) - other.g1);
}

Motor line_motor_geometric_anti_product(Line self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * other.g0.zwxy * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0), vec4(self.g0.x) * other.g1.wzyx * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * other.g1.zwxy * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * other.g1.yxwz * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g1.y) * other.g0.zwxy * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.z) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0));
}

Translator line_motor_left_anti_contraction(Line self, Motor other) {
    return Translator(vec4(self.g0.y) * vec4(other.g1.w, other.g1.w, other.g1.w, other.g0.y) * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.w, other.g1.w, other.g1.w, other.g0.z) * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * vec4(other.g1.w, other.g1.x, other.g1.x, other.g0.x) * vec4(1.0, 0.0, 0.0, -1.0));
}

AntiScalar line_motor_anti_scalar_product(Line self, Motor other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

Motor line_rotor_geometric_anti_product(Line self, Rotor other) {
    return Motor(vec4(self.g0.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * other.g0.zwxy * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0), vec4(self.g1.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g1.y) * other.g0.zwxy * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.z) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0));
}

AntiScalar line_rotor_left_anti_contraction(Line self, Rotor other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

AntiScalar line_rotor_anti_scalar_product(Line self, Rotor other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

Line line_translator_inner_anti_product(Line self, Translator other) {
    return Line(self.g0 * vec3(other.g0.w), self.g1 * vec3(other.g0.w));
}

Line line_translator_right_anti_contraction(Line self, Translator other) {
    return Line(self.g0 * vec3(other.g0.w), self.g1 * vec3(other.g0.w));
}

FlatPoint line_flector_regressive_product(Line self, Flector other) {
    return FlatPoint(vec4(self.g0.y) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.y) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g1.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x, self.g1.x, self.g1.x, self.g0.x) * other.g1.wzyx * vec4(1.0, -1.0, 1.0, -1.0));
}

Flector line_flector_geometric_anti_product(Line self, Flector other) {
    return Flector(vec4(self.g0.x) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g0.z, other.g1.w, other.g0.x, other.g1.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g1.w, other.g1.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.y) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g1.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g1.y) * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, -1.0, 1.0, 0.0), vec4(self.g0.x) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

Plane line_flector_inner_anti_product(Line self, Flector other) {
    return Plane(vec4(self.g0.x) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

Plane line_flector_left_anti_contraction(Line self, Flector other) {
    return Plane(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Plane line_flector_right_anti_contraction(Line self, Flector other) {
    return Plane(vec4(self.g0.y) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g1.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x, self.g0.x, self.g0.x, self.g1.x) * other.g1.xzyx * vec4(0.0, -1.0, 1.0, -1.0));
}

Line line_dilation_inner_anti_product(Line self, Dilation other) {
    return Line(self.g0 * vec3(other.g1.y), self.g1 * vec3(other.g1.y));
}

Line line_dilation_right_anti_contraction(Line self, Dilation other) {
    return Line(self.g0 * vec3(other.g1.y), self.g1 * vec3(other.g1.y));
}

MultiVector line_multi_vector_add(Line self, MultiVector other) {
    return MultiVector(other.g0, other.g1, other.g2, other.g3, other.g4, other.g5, self.g0 + other.g6, self.g1 + other.g7, other.g8, other.g9);
}

MultiVector line_multi_vector_sub(Line self, MultiVector other) {
    return MultiVector(vec3(0.0) - other.g0, vec3(0.0) - other.g1, vec2(0.0) - other.g2, vec4(0.0) - other.g3, vec3(0.0) - other.g4, vec3(0.0) - other.g5, self.g0 - other.g6, self.g1 - other.g7, vec4(0.0) - other.g8, vec4(0.0) - other.g9);
}

MultiVector line_multi_vector_geometric_anti_product(Line self, MultiVector other) {
    return MultiVector(vec3(0.0) - vec3(self.g0.x) * vec3(other.g5.x, other.g4.x, other.g6.x) - vec3(self.g0.y) * vec3(other.g5.y, other.g4.y, other.g6.y) - vec3(self.g0.z) * vec3(other.g5.z, other.g4.z, other.g6.z) + vec3(self.g1.y) * vec3(other.g4.y) * vec3(-1.0, 0.0, 0.0) + vec3(self.g1.z) * vec3(other.g4.z) * vec3(-1.0, 0.0, 0.0) + vec3(self.g1.x) * vec3(other.g4.x) * vec3(-1.0, 0.0, 0.0), vec3(self.g0.x) * vec3(other.g8.w, other.g1.z, other.g1.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g1.z, other.g8.w, other.g1.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g1.y, other.g1.x, other.g8.w) * vec3(-1.0, 1.0, 1.0) + vec3(self.g1.x) * vec3(other.g2.x, other.g8.z, other.g8.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g8.z, other.g2.x, other.g8.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.z) * vec3(other.g8.y, other.g8.x, other.g2.x) * vec3(-1.0, 1.0, 1.0), vec2(0.0) - vec2(self.g0.x) * vec2(other.g8.x, other.g7.x) - vec2(self.g0.y) * vec2(other.g8.y, other.g7.y) - vec2(self.g0.z) * vec2(other.g8.z, other.g7.z) + vec2(self.g1.y) * vec2(other.g6.y) * vec2(0.0, -1.0) + vec2(self.g1.z) * vec2(other.g6.z) * vec2(0.0, -1.0) + vec2(self.g1.x) * vec2(other.g6.x) * vec2(0.0, -1.0), vec4(self.g0.x) * vec4(other.g9.w, other.g3.z, other.g3.y, other.g9.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g3.z, other.g9.w, other.g3.x, other.g9.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g3.y, other.g3.x, other.g9.w, other.g9.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.y) * vec4(other.g9.z, other.g3.w, other.g9.x, other.g9.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g1.z) * vec4(other.g9.y, other.g9.x, other.g3.w, other.g9.y) * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g3.w, other.g9.z, other.g9.y, other.g3.x) * vec4(1.0, -1.0, 1.0, 0.0), vec3(self.g0.x) * vec3(other.g0.y, other.g4.z, other.g4.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g4.z, other.g0.y, other.g4.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g4.y, other.g4.x, other.g0.y) * vec3(-1.0, 1.0, 1.0), vec3(self.g0.x) * vec3(other.g0.x, other.g5.z, other.g5.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g5.z, other.g0.x, other.g5.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g5.y, other.g5.x, other.g0.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g1.x) * vec3(other.g0.y, other.g4.z, other.g4.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g4.z, other.g0.y, other.g4.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.z) * vec3(other.g4.y, other.g4.x, other.g0.y) * vec3(-1.0, 1.0, 1.0), vec3(self.g0.x) * vec3(other.g0.z, other.g6.z, other.g6.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g6.z, other.g0.z, other.g6.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g6.y, other.g6.x, other.g0.z) * vec3(-1.0, 1.0, 1.0), vec3(self.g0.x) * vec3(other.g2.y, other.g7.z, other.g7.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g7.z, other.g2.y, other.g7.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g7.y, other.g7.x, other.g2.y) * vec3(-1.0, 1.0, 1.0) + vec3(self.g1.x) * vec3(other.g0.z, other.g6.z, other.g6.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g6.z, other.g0.z, other.g6.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.z) * vec3(other.g6.y, other.g6.x, other.g0.z) * vec3(-1.0, 1.0, 1.0), vec4(self.g0.x) * vec4(other.g2.x, other.g8.z, other.g8.y, other.g1.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g8.z, other.g2.x, other.g8.x, other.g1.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g8.y, other.g8.x, other.g2.x, other.g1.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.y) * vec4(other.g8.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g8.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g8.x) * vec4(0.0, 0.0, 0.0, -1.0), vec4(self.g0.x) * vec4(other.g3.w, other.g9.z, other.g9.y, other.g3.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g9.z, other.g3.w, other.g9.x, other.g3.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g9.y, other.g9.x, other.g3.w, other.g3.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.y) * vec4(other.g9.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g9.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g9.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

AntiScalar line_multi_vector_anti_scalar_product(Line self, MultiVector other) {
    return AntiScalar(0.0 - self.g0.x * other.g6.x - self.g0.y * other.g6.y - self.g0.z * other.g6.z);
}

Line line_scale(Line self, float other) {
    return line_scalar_geometric_product(self, Scalar(other));
}

RadialPoint line_attitude(Line self) {
    return line_circle_regressive_product(self, Circle(vec4(0.0, 0.0, 0.0, 1.0), vec3(0.0), vec3(0.0)));
}

Circle circle_zero() {
    return Circle(vec4(0.0), vec3(0.0), vec3(0.0));
}

Circle circle_one() {
    return Circle(vec4(0.0), vec3(0.0), vec3(0.0));
}

int circle_grade(Circle self) {
    return 3;
}

int circle_anti_grade(Circle self) {
    return 2;
}

Circle circle_neg(Circle self) {
    return Circle(self.g0 * vec4(-1.0), self.g1 * vec3(-1.0), self.g2 * vec3(-1.0));
}

Circle circle_automorphism(Circle self) {
    return Circle(self.g0 * vec4(-1.0), self.g1 * vec3(-1.0), self.g2 * vec3(-1.0));
}

Circle circle_reversal(Circle self) {
    return Circle(self.g0 * vec4(-1.0), self.g1 * vec3(-1.0), self.g2 * vec3(-1.0));
}

Circle circle_conjugation(Circle self) {
    return Circle(self.g0, self.g1, self.g2);
}

Dipole circle_dual(Circle self) {
    return Dipole(self.g2 * vec3(-1.0), self.g1 * vec3(-1.0), self.g0 * vec4(-1.0));
}

Circle circle_anti_reversal(Circle self) {
    return Circle(self.g0, self.g1, self.g2);
}

Dipole circle_right_complement(Circle self) {
    return Dipole(self.g2 * vec3(-1.0), self.g1 * vec3(-1.0), self.g0 * vec4(-1.0));
}

Dipole circle_left_complement(Circle self) {
    return Dipole(self.g2 * vec3(-1.0), self.g1 * vec3(-1.0), self.g0 * vec4(-1.0));
}

Circle circle_double_complement(Circle self) {
    return Circle(self.g0, self.g1, self.g2);
}

Circle circle_scalar_geometric_product(Circle self, Scalar other) {
    return Circle(self.g0 * vec4(other.g0), self.g1 * vec3(other.g0), self.g2 * vec3(other.g0));
}

Circle circle_scalar_outer_product(Circle self, Scalar other) {
    return Circle(self.g0 * vec4(other.g0), self.g1 * vec3(other.g0), self.g2 * vec3(other.g0));
}

Circle circle_scalar_inner_product(Circle self, Scalar other) {
    return Circle(self.g0 * vec4(other.g0), self.g1 * vec3(other.g0), self.g2 * vec3(other.g0));
}

Circle circle_scalar_right_contraction(Circle self, Scalar other) {
    return Circle(self.g0 * vec4(other.g0), self.g1 * vec3(other.g0), self.g2 * vec3(other.g0));
}

Circle circle_anti_scalar_regressive_product(Circle self, AntiScalar other) {
    return Circle(self.g0 * vec4(other.g0), self.g1 * vec3(other.g0), self.g2 * vec3(other.g0));
}

Circle circle_anti_scalar_geometric_anti_product(Circle self, AntiScalar other) {
    return Circle(self.g0 * vec4(other.g0), self.g1 * vec3(other.g0), self.g2 * vec3(other.g0));
}

Circle circle_anti_scalar_inner_anti_product(Circle self, AntiScalar other) {
    return Circle(self.g0 * vec4(other.g0), self.g1 * vec3(other.g0), self.g2 * vec3(other.g0));
}

Circle circle_anti_scalar_right_anti_contraction(Circle self, AntiScalar other) {
    return Circle(self.g0 * vec4(other.g0), self.g1 * vec3(other.g0), self.g2 * vec3(other.g0));
}

Circle circle_homogeneous_magnitude_regressive_product(Circle self, HomogeneousMagnitude other) {
    return Circle(self.g0 * vec4(other.g0.y), self.g1 * vec3(other.g0.y), self.g2 * vec3(other.g0.y));
}

Circle circle_homogeneous_magnitude_outer_product(Circle self, HomogeneousMagnitude other) {
    return Circle(self.g0 * vec4(other.g0.x), self.g1 * vec3(other.g0.x), self.g2 * vec3(other.g0.x));
}

Circle circle_homogeneous_magnitude_right_contraction(Circle self, HomogeneousMagnitude other) {
    return Circle(self.g0 * vec4(other.g0.x), self.g1 * vec3(other.g0.x), self.g2 * vec3(other.g0.x));
}

Circle circle_homogeneous_magnitude_right_anti_contraction(Circle self, HomogeneousMagnitude other) {
    return Circle(self.g0 * vec4(other.g0.y), self.g1 * vec3(other.g0.y), self.g2 * vec3(other.g0.y));
}

Sphere circle_radial_point_outer_product(Circle self, RadialPoint other) {
    return Sphere(vec2(self.g0.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.w) * other.g1 * vec2(-1.0, 1.0) + vec2(self.g2.x) * vec2(other.g0.x) * vec2(0.0, 1.0) + vec2(self.g2.y) * vec2(other.g0.y) * vec2(0.0, 1.0) + vec2(self.g2.z) * vec2(other.g0.z) * vec2(0.0, 1.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(-1.0, 0.0), vec3(self.g1.x) * other.g0.zzy * vec3(0.0, 1.0, -1.0) + vec3(self.g1.y) * other.g0.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * other.g0.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g2.x) * vec3(other.g1.x) * vec3(-1.0, 0.0, 0.0) + vec3(self.g2.y) * vec3(other.g1.x) * vec3(0.0, -1.0, 0.0) + vec3(self.g2.z) * vec3(other.g1.x) * vec3(0.0, 0.0, -1.0) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.y));
}

Dipole circle_radial_point_inner_product(Circle self, RadialPoint other) {
    return Dipole(vec3(self.g0.y) * other.g0.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * other.g0.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * other.g0.xzy * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g0.w) * other.g0, vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g2.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g0.z) * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g2.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.y) * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.x, self.g2.x, self.g2.x, self.g1.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g0.x) * vec4(0.0, -1.0, 1.0, -1.0));
}

Dipole circle_radial_point_right_contraction(Circle self, RadialPoint other) {
    return Dipole(vec3(self.g0.y) * other.g0.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * other.g0.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * other.g0.xzy * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g0.w) * other.g0, vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g2.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g0.z) * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g2.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.y) * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.x, self.g2.x, self.g2.x, self.g1.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g0.x) * vec4(0.0, -1.0, 1.0, -1.0));
}

Scalar circle_flat_point_regressive_product(Circle self, FlatPoint other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

AntiScalar circle_flat_point_outer_product(Circle self, FlatPoint other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

Plane circle_flat_point_inner_anti_product(Circle self, FlatPoint other) {
    return Plane(vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Plane circle_flat_point_left_anti_contraction(Circle self, FlatPoint other) {
    return Plane(vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Scalar circle_dipole_regressive_product(Circle self, Dipole other) {
    return Scalar(0.0 - self.g0.x * other.g2.x - self.g0.y * other.g2.y - self.g0.z * other.g2.z - self.g0.w * other.g2.w - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z - self.g2.x * other.g0.x - self.g2.y * other.g0.y - self.g2.z * other.g0.z);
}

AntiScalar circle_dipole_outer_product(Circle self, Dipole other) {
    return AntiScalar(0.0 - self.g0.x * other.g2.x - self.g0.y * other.g2.y - self.g0.z * other.g2.z - self.g0.w * other.g2.w - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z - self.g2.x * other.g0.x - self.g2.y * other.g0.y - self.g2.z * other.g0.z);
}

RadialPoint circle_dipole_inner_product(Circle self, Dipole other) {
    return RadialPoint(vec3(self.g0.w) * other.g1, vec2(self.g0.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g2.x) * vec2(other.g1.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(-1.0, 0.0));
}

Sphere circle_dipole_inner_anti_product(Circle self, Dipole other) {
    return Sphere(vec2(0.0) - vec2(self.g1.x) * vec2(other.g0.x, other.g2.x) - vec2(self.g1.y) * vec2(other.g0.y, other.g2.y) - vec2(self.g1.z) * vec2(other.g0.z, other.g2.z), self.g1 * vec3(other.g2.w));
}

RadialPoint circle_dipole_right_contraction(Circle self, Dipole other) {
    return RadialPoint(vec3(self.g0.w) * other.g1, vec2(self.g0.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g2.x) * vec2(other.g1.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(-1.0, 0.0));
}

Sphere circle_dipole_left_anti_contraction(Circle self, Dipole other) {
    return Sphere(vec2(0.0) - vec2(self.g1.x) * vec2(other.g0.x, other.g2.x) - vec2(self.g1.y) * vec2(other.g0.y, other.g2.y) - vec2(self.g1.z) * vec2(other.g0.z, other.g2.z), self.g1 * vec3(other.g2.w));
}

Line circle_line_into(Circle self) {
    return Line(self.g1, self.g2);
}

Circle circle_line_add(Circle self, Line other) {
    return Circle(self.g0, self.g1 + other.g0, self.g2 + other.g1);
}

Circle circle_line_sub(Circle self, Line other) {
    return Circle(self.g0, self.g1 - other.g0, self.g2 - other.g1);
}

RadialPoint circle_line_regressive_product(Circle self, Line other) {
    return RadialPoint(vec3(self.g0.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g0.w) * other.g0 + vec3(self.g0.x) * other.g1.xzy * vec3(0.0, 1.0, -1.0), vec2(self.g0.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g1.x) * vec2(0.0, -1.0) + vec2(self.g1.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g1.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g2.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(-1.0, 0.0));
}

AntiScalar circle_line_inner_anti_product(Circle self, Line other) {
    return AntiScalar(0.0 - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

AntiScalar circle_line_left_anti_contraction(Circle self, Line other) {
    return AntiScalar(0.0 - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

AntiScalar circle_line_right_anti_contraction(Circle self, Line other) {
    return AntiScalar(0.0 - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

AntiScalar circle_line_anti_scalar_product(Circle self, Line other) {
    return AntiScalar(0.0 - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

Circle circle_circle_add(Circle self, Circle other) {
    return Circle(self.g0 + other.g0, self.g1 + other.g1, self.g2 + other.g2);
}

Circle circle_circle_sub(Circle self, Circle other) {
    return Circle(self.g0 - other.g0, self.g1 - other.g1, self.g2 - other.g2);
}

Circle circle_circle_mul(Circle self, Circle other) {
    return Circle(self.g0 * other.g0, self.g1 * other.g1, self.g2 * other.g2);
}

Circle circle_circle_div(Circle self, Circle other) {
    return Circle(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.w) * vec4(1.0, 1.0, 1.0, 1.0), vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(1.0, 1.0, 1.0) / vec3(other.g1.x, other.g1.y, other.g1.z) * vec3(1.0, 1.0, 1.0), vec3(self.g2.x, self.g2.y, self.g2.z) * vec3(1.0, 1.0, 1.0) / vec3(other.g2.x, other.g2.y, other.g2.z) * vec3(1.0, 1.0, 1.0));
}

RadialPoint circle_circle_regressive_product(Circle self, Circle other) {
    return RadialPoint(vec3(self.g0.y) * other.g2.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * other.g2.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g0.w) * other.g1 + vec3(self.g1.x) * vec3(other.g0.w) * vec3(1.0, 0.0, 0.0) + vec3(self.g1.y) * vec3(other.g0.w) * vec3(0.0, 1.0, 0.0) + vec3(self.g1.z) * vec3(other.g0.w) * vec3(0.0, 0.0, 1.0) + vec3(self.g2.x) * vec3(other.g0.z, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0) + vec3(self.g2.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g2.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * other.g2.xzy * vec3(0.0, 1.0, -1.0), vec2(self.g0.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) - vec2(self.g1.x) * vec2(other.g0.x, other.g2.x) - vec2(self.g1.y) * vec2(other.g0.y, other.g2.y) - vec2(self.g1.z) * vec2(other.g0.z, other.g2.z) + vec2(self.g2.x) * vec2(other.g1.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(-1.0, 0.0));
}

Scalar circle_circle_inner_product(Circle self, Circle other) {
    return Scalar(0.0 - self.g0.w * other.g0.w);
}

AntiScalar circle_circle_inner_anti_product(Circle self, Circle other) {
    return AntiScalar(0.0 - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z);
}

Scalar circle_circle_left_contraction(Circle self, Circle other) {
    return Scalar(0.0 - self.g0.w * other.g0.w);
}

Scalar circle_circle_right_contraction(Circle self, Circle other) {
    return Scalar(0.0 - self.g0.w * other.g0.w);
}

AntiScalar circle_circle_left_anti_contraction(Circle self, Circle other) {
    return AntiScalar(0.0 - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z);
}

AntiScalar circle_circle_right_anti_contraction(Circle self, Circle other) {
    return AntiScalar(0.0 - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z);
}

Scalar circle_circle_scalar_product(Circle self, Circle other) {
    return Scalar(0.0 - self.g0.w * other.g0.w);
}

AntiScalar circle_circle_anti_scalar_product(Circle self, Circle other) {
    return AntiScalar(0.0 - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z);
}

Dipole circle_plane_regressive_product(Circle self, Plane other) {
    return Dipole(vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w), vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g2.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g2.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.x, self.g2.x, self.g2.x, self.g1.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0));
}

Sphere circle_plane_inner_anti_product(Circle self, Plane other) {
    return Sphere(vec2(self.g0.y) * vec2(other.g0.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(1.0, 0.0) + vec2(self.g2.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(1.0, 0.0), vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0));
}

Sphere circle_plane_right_anti_contraction(Circle self, Plane other) {
    return Sphere(vec2(self.g0.y) * vec2(other.g0.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(1.0, 0.0) + vec2(self.g2.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(1.0, 0.0), vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0));
}

Dipole circle_sphere_regressive_product(Circle self, Sphere other) {
    return Dipole(vec3(self.g0.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g1.x) * vec3(other.g0.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g1.y) * vec3(other.g0.x) * vec3(0.0, 1.0, 0.0) + vec3(self.g1.z) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0) + vec3(self.g0.x) * other.g1.xzy * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g0.w) * other.g1 + vec3(self.g2.x) * vec3(other.g0.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g2.y) * vec3(other.g0.x) * vec3(0.0, 1.0, 0.0) + vec3(self.g2.z) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), vec4(self.g1.y) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g1.y) * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g1.z) * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g2.x) * vec4(other.g1.z, other.g1.z, other.g1.y, other.g1.z) * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g2.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.z) * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g2.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.y) * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.y, other.g0.x, other.g0.x, other.g1.x) * vec4(1.0, 0.0, 0.0, -1.0));
}

Sphere circle_sphere_inner_anti_product(Circle self, Sphere other) {
    return Sphere(vec2(self.g0.y) * vec2(other.g1.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(1.0, 0.0) + vec2(self.g2.x) * vec2(other.g1.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(1.0, 0.0), vec3(self.g1.y) * other.g1.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * other.g1.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.x) * other.g1.xzy * vec3(0.0, -1.0, 1.0));
}

Sphere circle_sphere_right_anti_contraction(Circle self, Sphere other) {
    return Sphere(vec2(self.g0.y) * vec2(other.g1.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(1.0, 0.0) + vec2(self.g2.x) * vec2(other.g1.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(1.0, 0.0), vec3(self.g1.y) * other.g1.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * other.g1.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.x) * other.g1.xzy * vec3(0.0, -1.0, 1.0));
}

Flector circle_motor_geometric_product(Circle self, Motor other) {
    return Flector(vec4(self.g0.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g0.w) + vec4(self.g0.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0), vec4(self.g0.y) * other.g1.zwxz * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g0.z) * other.g1.yxwy * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.w) + vec4(self.g0.x) * other.g1.wzyx * vec4(1.0, 1.0, -1.0, 0.0));
}

Plane circle_motor_outer_product(Circle self, Motor other) {
    return Plane(self.g0 * vec4(other.g1.w));
}

Translator circle_motor_left_anti_contraction(Circle self, Motor other) {
    return Translator(vec4(self.g1.y) * vec4(other.g1.w, other.g1.w, other.g1.w, other.g0.y) * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.w, other.g1.w, other.g1.w, other.g0.z) * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.x) * vec4(other.g1.w, other.g1.x, other.g1.x, other.g0.x) * vec4(1.0, 0.0, 0.0, -1.0));
}

AntiScalar circle_motor_anti_scalar_product(Circle self, Motor other) {
    return AntiScalar(0.0 - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

AntiScalar circle_rotor_left_anti_contraction(Circle self, Rotor other) {
    return AntiScalar(0.0 - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

AntiScalar circle_rotor_anti_scalar_product(Circle self, Rotor other) {
    return AntiScalar(0.0 - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

Circle circle_translator_inner_anti_product(Circle self, Translator other) {
    return Circle(self.g0 * vec4(other.g0.w), self.g1 * vec3(other.g0.w), self.g2 * vec3(other.g0.w));
}

Circle circle_translator_right_anti_contraction(Circle self, Translator other) {
    return Circle(self.g0 * vec4(other.g0.w), self.g1 * vec3(other.g0.w), self.g2 * vec3(other.g0.w));
}

Motor circle_flector_geometric_product(Circle self, Flector other) {
    return Motor(vec4(self.g0.x) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g0.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.y) * vec4(other.g0.z, other.g1.w, other.g0.x, other.g0.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g1.w, other.g0.z) * vec4(1.0, -1.0, 1.0, -1.0) - vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g0.w), vec4(0.0) - vec4(self.g0.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.w));
}

AntiScalar circle_flector_outer_product(Circle self, Flector other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

Sphere circle_flector_inner_anti_product(Circle self, Flector other) {
    return Sphere(vec2(self.g0.y) * vec2(other.g1.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(1.0, 0.0) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g1.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g2.x) * vec2(other.g1.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(1.0, 0.0), vec3(self.g1.x) * vec3(other.g0.w, other.g1.z, other.g1.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g1.z, other.g0.w, other.g1.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.z) * vec3(other.g1.y, other.g1.x, other.g0.w) * vec3(-1.0, 1.0, 1.0));
}

Plane circle_flector_left_anti_contraction(Circle self, Flector other) {
    return Plane(vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Sphere circle_flector_right_anti_contraction(Circle self, Flector other) {
    return Sphere(vec2(self.g0.y) * vec2(other.g1.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(1.0, 0.0) + vec2(self.g2.x) * vec2(other.g1.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(1.0, 0.0), vec3(self.g1.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.x) * vec3(other.g1.x, other.g1.z, other.g1.y) * vec3(0.0, -1.0, 1.0));
}

Circle circle_dilation_inner_anti_product(Circle self, Dilation other) {
    return Circle(self.g0 * vec4(other.g1.y), self.g1 * vec3(other.g1.y), self.g2 * vec3(other.g1.y));
}

Scalar circle_dilation_right_contraction(Circle self, Dilation other) {
    return Scalar(self.g0.w * other.g0.z);
}

Circle circle_dilation_right_anti_contraction(Circle self, Dilation other) {
    return Circle(self.g0 * vec4(other.g1.y), self.g1 * vec3(other.g1.y), self.g2 * vec3(other.g1.y));
}

Scalar circle_dilation_scalar_product(Circle self, Dilation other) {
    return Scalar(self.g0.w * other.g0.z);
}

MultiVector circle_multi_vector_add(Circle self, MultiVector other) {
    return MultiVector(other.g0, other.g1, other.g2, other.g3, other.g4, other.g5, self.g1 + other.g6, self.g2 + other.g7, self.g0 + other.g8, other.g9);
}

MultiVector circle_multi_vector_sub(Circle self, MultiVector other) {
    return MultiVector(vec3(0.0) - other.g0, vec3(0.0) - other.g1, vec2(0.0) - other.g2, vec4(0.0) - other.g3, vec3(0.0) - other.g4, vec3(0.0) - other.g5, self.g1 - other.g6, self.g2 - other.g7, self.g0 - other.g8, vec4(0.0) - other.g9);
}

MultiVector circle_multi_vector_geometric_product(Circle self, MultiVector other) {
    return MultiVector(vec3(self.g0.y) * vec3(other.g1.y, other.g1.y, other.g3.y) * vec3(0.0, -1.0, -1.0) + vec3(self.g0.z) * vec3(other.g1.z, other.g1.z, other.g3.z) * vec3(0.0, -1.0, -1.0) - vec3(self.g0.w) * vec3(other.g8.w, other.g2.x, other.g3.w) + vec3(self.g1.x) * vec3(other.g5.x) * vec3(0.0, 0.0, -1.0) + vec3(self.g1.y) * vec3(other.g5.y) * vec3(0.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g5.z) * vec3(0.0, 0.0, -1.0) + vec3(self.g2.x) * vec3(other.g4.x) * vec3(0.0, 0.0, -1.0) + vec3(self.g2.y) * vec3(other.g4.y) * vec3(0.0, 0.0, -1.0) + vec3(self.g2.z) * vec3(other.g4.z) * vec3(0.0, 0.0, -1.0) + vec3(self.g0.x) * vec3(other.g1.x, other.g1.x, other.g3.x) * vec3(0.0, -1.0, -1.0), vec3(self.g0.w) * other.g5, vec2(self.g0.y) * vec2(other.g5.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g5.z) * vec2(-1.0, 0.0) + vec2(self.g0.w) * vec2(other.g0.y, other.g9.w) * vec2(1.0, -1.0) + vec2(self.g2.x) * vec2(other.g5.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g5.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g5.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g5.x) * vec2(-1.0, 0.0), vec4(self.g0.y) * vec4(other.g7.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g7.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g7.x, other.g7.y, other.g7.z, other.g0.z) + vec4(self.g1.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g2.x) * vec4(other.g8.w, other.g1.z, other.g1.y, other.g8.x) * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g2.y) * vec4(other.g1.z, other.g8.w, other.g1.x, other.g8.y) * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g2.z) * vec4(other.g1.y, other.g1.x, other.g8.w, other.g8.z) * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g0.x) * vec4(other.g7.x) * vec4(0.0, 0.0, 0.0, -1.0), vec3(self.g0.x) * vec3(other.g8.w, other.g1.z, other.g1.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.y) * vec3(other.g1.z, other.g8.w, other.g1.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g0.z) * vec3(other.g1.y, other.g1.x, other.g8.w) * vec3(1.0, -1.0, 1.0) - vec3(self.g0.w) * vec3(other.g8.x, other.g8.y, other.g8.z), vec3(0.0) - vec3(self.g0.w) * other.g1, vec3(self.g0.x) * vec3(other.g9.w, other.g3.z, other.g3.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.y) * vec3(other.g3.z, other.g9.w, other.g3.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g0.z) * vec3(other.g3.y, other.g3.x, other.g9.w) * vec3(1.0, -1.0, 1.0) - vec3(self.g0.w) * vec3(other.g9.x, other.g9.y, other.g9.z) + vec3(self.g1.x) * vec3(other.g0.x, other.g5.z, other.g5.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.y) * vec3(other.g5.z, other.g0.x, other.g5.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g1.z) * vec3(other.g5.y, other.g5.x, other.g0.x) * vec3(1.0, -1.0, 1.0) + vec3(self.g2.x) * vec3(other.g0.y, other.g4.z, other.g4.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g2.y) * vec3(other.g4.z, other.g0.y, other.g4.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g2.z) * vec3(other.g4.y, other.g4.x, other.g0.y) * vec3(1.0, -1.0, 1.0), vec3(0.0) - vec3(self.g0.w) * vec3(other.g3.x, other.g3.y, other.g3.z) + vec3(self.g2.x) * vec3(other.g0.x, other.g5.z, other.g5.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g2.y) * vec3(other.g5.z, other.g0.x, other.g5.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g2.z) * vec3(other.g5.y, other.g5.x, other.g0.x) * vec3(1.0, -1.0, 1.0), vec4(self.g0.y) * vec4(other.g5.z, other.g0.x, other.g5.x, other.g5.z) * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g0.z) * vec4(other.g5.y, other.g5.x, other.g0.x, other.g5.y) * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g0.w) * vec4(other.g4.x, other.g4.y, other.g4.z, other.g0.x) + vec4(self.g0.x) * vec4(other.g0.x, other.g5.z, other.g5.y, other.g0.x) * vec4(1.0, 1.0, -1.0, 0.0), vec4(self.g0.y) * vec4(other.g7.z, other.g2.y, other.g7.x, other.g7.z) * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g0.z) * vec4(other.g7.y, other.g7.x, other.g2.y, other.g7.y) * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g0.w) * vec4(other.g6.x, other.g6.y, other.g6.z, other.g2.y) + vec4(self.g1.x) * vec4(other.g8.w, other.g1.z, other.g1.y, other.g8.w) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g1.y) * vec4(other.g1.z, other.g8.w, other.g1.x, other.g1.z) * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g8.w, other.g1.y) * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g2.x) * vec4(other.g2.x, other.g8.z, other.g8.y, other.g1.x) * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g2.y) * vec4(other.g8.z, other.g2.x, other.g8.x, other.g1.y) * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g2.z) * vec4(other.g8.y, other.g8.x, other.g2.x, other.g1.z) * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g0.x) * vec4(other.g2.y, other.g7.z, other.g7.y, other.g2.x) * vec4(1.0, 1.0, -1.0, 0.0));
}

MultiVector circle_multi_vector_geometric_anti_product(Circle self, MultiVector other) {
    return MultiVector(vec3(self.g0.y) * vec3(other.g3.y, other.g9.y, other.g3.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.z) * vec3(other.g3.z, other.g9.z, other.g3.z) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.w) * vec3(other.g3.w) * vec3(-1.0, 0.0, 0.0) - vec3(self.g1.x) * vec3(other.g5.x, other.g4.x, other.g6.x) - vec3(self.g1.y) * vec3(other.g5.y, other.g4.y, other.g6.y) - vec3(self.g1.z) * vec3(other.g5.z, other.g4.z, other.g6.z) + vec3(self.g2.x) * vec3(other.g4.x) * vec3(-1.0, 0.0, 0.0) + vec3(self.g2.y) * vec3(other.g4.y) * vec3(-1.0, 0.0, 0.0) + vec3(self.g2.z) * vec3(other.g4.z) * vec3(-1.0, 0.0, 0.0) + vec3(self.g0.x) * vec3(other.g3.x, other.g9.x, other.g3.x) * vec3(-1.0, 1.0, 0.0), vec3(self.g0.x) * vec3(other.g2.y, other.g7.z, other.g7.y) * vec3(-1.0, 1.0, -1.0) + vec3(self.g0.y) * vec3(other.g7.z, other.g2.y, other.g7.x) * vec3(-1.0, -1.0, 1.0) + vec3(self.g0.z) * vec3(other.g7.y, other.g7.x, other.g2.y) * vec3(1.0, -1.0, -1.0) + vec3(self.g0.w) * other.g6 + vec3(self.g1.x) * vec3(other.g8.w, other.g1.z, other.g1.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g1.z, other.g8.w, other.g1.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.z) * vec3(other.g1.y, other.g1.x, other.g8.w) * vec3(-1.0, 1.0, 1.0) + vec3(self.g2.x) * vec3(other.g2.x, other.g8.z, other.g8.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g2.y) * vec3(other.g8.z, other.g2.x, other.g8.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g2.z) * vec3(other.g8.y, other.g8.x, other.g2.x) * vec3(-1.0, 1.0, 1.0), vec2(self.g0.y) * vec2(other.g6.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g6.z) * vec2(-1.0, 0.0) - vec2(self.g1.x) * vec2(other.g8.x, other.g7.x) - vec2(self.g1.y) * vec2(other.g8.y, other.g7.y) - vec2(self.g1.z) * vec2(other.g8.z, other.g7.z) + vec2(self.g2.x) * vec2(other.g6.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g6.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g6.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g6.x) * vec2(-1.0, 0.0), vec4(self.g1.x) * vec4(other.g9.w, other.g3.z, other.g3.y, other.g9.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g1.y) * vec4(other.g3.z, other.g9.w, other.g3.x, other.g9.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.z) * vec4(other.g3.y, other.g3.x, other.g9.w, other.g9.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g2.y) * vec4(other.g9.z, other.g3.w, other.g9.x, other.g9.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g2.z) * vec4(other.g9.y, other.g9.x, other.g3.w, other.g9.y) * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g2.x) * vec4(other.g3.w, other.g9.z, other.g9.y, other.g3.x) * vec4(1.0, -1.0, 1.0, 0.0), vec3(self.g0.x) * vec3(other.g3.w, other.g9.z, other.g9.y) * vec3(-1.0, 1.0, -1.0) + vec3(self.g0.y) * vec3(other.g9.z, other.g3.w, other.g9.x) * vec3(-1.0, -1.0, 1.0) + vec3(self.g0.z) * vec3(other.g9.y, other.g9.x, other.g3.w) * vec3(1.0, -1.0, -1.0) + vec3(self.g1.x) * vec3(other.g0.y, other.g4.z, other.g4.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g4.z, other.g0.y, other.g4.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.z) * vec3(other.g4.y, other.g4.x, other.g0.y) * vec3(-1.0, 1.0, 1.0), vec3(self.g0.x) * vec3(other.g9.w, other.g3.z, other.g3.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g3.z, other.g9.w, other.g3.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g3.y, other.g3.x, other.g9.w) * vec3(-1.0, 1.0, 1.0) - vec3(self.g0.w) * vec3(other.g9.x, other.g9.y, other.g9.z) + vec3(self.g1.x) * vec3(other.g0.x, other.g5.z, other.g5.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g5.z, other.g0.x, other.g5.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.z) * vec3(other.g5.y, other.g5.x, other.g0.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g2.x) * vec3(other.g0.y, other.g4.z, other.g4.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g2.y) * vec3(other.g4.z, other.g0.y, other.g4.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g2.z) * vec3(other.g4.y, other.g4.x, other.g0.y) * vec3(-1.0, 1.0, 1.0), vec3(self.g1.x) * vec3(other.g0.z, other.g6.z, other.g6.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g6.z, other.g0.z, other.g6.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.z) * vec3(other.g6.y, other.g6.x, other.g0.z) * vec3(-1.0, 1.0, 1.0), vec3(self.g1.x) * vec3(other.g2.y, other.g7.z, other.g7.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g7.z, other.g2.y, other.g7.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.z) * vec3(other.g7.y, other.g7.x, other.g2.y) * vec3(-1.0, 1.0, 1.0) + vec3(self.g2.x) * vec3(other.g0.z, other.g6.z, other.g6.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g2.y) * vec3(other.g6.z, other.g0.z, other.g6.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g2.z) * vec3(other.g6.y, other.g6.x, other.g0.z) * vec3(-1.0, 1.0, 1.0), vec4(self.g0.x) * vec4(other.g0.z, other.g6.z, other.g6.y, other.g7.x) * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g0.y) * vec4(other.g6.z, other.g0.z, other.g6.x, other.g7.y) * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g6.y, other.g6.x, other.g0.z, other.g7.z) * vec4(-1.0, 1.0, 1.0, 1.0) + vec4(self.g1.x) * vec4(other.g2.x, other.g8.z, other.g8.y, other.g1.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g1.y) * vec4(other.g8.z, other.g2.x, other.g8.x, other.g1.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.z) * vec4(other.g8.y, other.g8.x, other.g2.x, other.g1.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g2.x) * vec4(other.g8.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g2.y) * vec4(other.g8.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g2.z) * vec4(other.g8.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.xxxw * vec4(other.g0.x, other.g0.x, other.g0.x, other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0), vec4(self.g1.x) * vec4(other.g3.w, other.g9.z, other.g9.y, other.g3.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g1.y) * vec4(other.g9.z, other.g3.w, other.g9.x, other.g3.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.z) * vec4(other.g9.y, other.g9.x, other.g3.w, other.g3.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g2.y) * vec4(other.g9.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g2.z) * vec4(other.g9.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g2.x) * vec4(other.g9.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

Scalar circle_multi_vector_scalar_product(Circle self, MultiVector other) {
    return Scalar(0.0 - self.g0.w * other.g8.w);
}

AntiScalar circle_multi_vector_anti_scalar_product(Circle self, MultiVector other) {
    return AntiScalar(0.0 - self.g1.x * other.g6.x - self.g1.y * other.g6.y - self.g1.z * other.g6.z);
}

Scalar circle_squared_magnitude(Circle self) {
    return circle_circle_scalar_product(self, circle_reversal(self));
}

Scalar circle_magnitude(Circle self) {
    return Scalar(sqrt(circle_squared_magnitude(self).g0));
}

Scalar circle_bulk_norm(Circle self) {
    return Scalar(sqrt(circle_squared_magnitude(self).g0));
}

AntiScalar circle_squared_anti_magnitude(Circle self) {
    return circle_circle_anti_scalar_product(self, circle_anti_reversal(self));
}

AntiScalar circle_weight_norm(Circle self) {
    return AntiScalar(sqrt(circle_squared_anti_magnitude(self).g0));
}

HomogeneousMagnitude circle_geometric_norm(Circle self) {
    return scalar_anti_scalar_add(circle_bulk_norm(self), circle_weight_norm(self));
}

Circle circle_scale(Circle self, float other) {
    return circle_scalar_geometric_product(self, Scalar(other));
}

Circle circle_signum(Circle self) {
    return circle_scalar_geometric_product(self, Scalar(1.0 / circle_magnitude(self).g0));
}

Circle circle_inverse(Circle self) {
    return circle_scalar_geometric_product(circle_reversal(self), Scalar(1.0 / circle_squared_magnitude(self).g0));
}

Circle circle_unitize(Circle self) {
    return circle_scalar_geometric_product(self, Scalar(1.0 / circle_weight_norm(self).g0));
}

RadialPoint circle_attitude(Circle self) {
    return circle_circle_regressive_product(self, Circle(vec4(0.0, 0.0, 0.0, 1.0), vec3(0.0), vec3(0.0)));
}

Plane plane_zero() {
    return Plane(vec4(0.0));
}

Plane plane_one() {
    return Plane(vec4(0.0));
}

int plane_grade(Plane self) {
    return 4;
}

int plane_anti_grade(Plane self) {
    return 1;
}

Plane plane_neg(Plane self) {
    return Plane(self.g0 * vec4(-1.0));
}

Plane plane_automorphism(Plane self) {
    return Plane(self.g0);
}

Plane plane_reversal(Plane self) {
    return Plane(self.g0);
}

Plane plane_conjugation(Plane self) {
    return Plane(self.g0);
}

Plane plane_anti_reversal(Plane self) {
    return Plane(self.g0);
}

Plane plane_double_complement(Plane self) {
    return Plane(self.g0);
}

Plane plane_scalar_geometric_product(Plane self, Scalar other) {
    return Plane(self.g0 * vec4(other.g0));
}

Plane plane_scalar_outer_product(Plane self, Scalar other) {
    return Plane(self.g0 * vec4(other.g0));
}

Plane plane_scalar_inner_product(Plane self, Scalar other) {
    return Plane(self.g0 * vec4(other.g0));
}

Plane plane_scalar_right_contraction(Plane self, Scalar other) {
    return Plane(self.g0 * vec4(other.g0));
}

Plane plane_anti_scalar_regressive_product(Plane self, AntiScalar other) {
    return Plane(self.g0 * vec4(other.g0));
}

Plane plane_anti_scalar_geometric_anti_product(Plane self, AntiScalar other) {
    return Plane(self.g0 * vec4(other.g0));
}

Plane plane_anti_scalar_inner_anti_product(Plane self, AntiScalar other) {
    return Plane(self.g0 * vec4(other.g0));
}

Plane plane_anti_scalar_right_anti_contraction(Plane self, AntiScalar other) {
    return Plane(self.g0 * vec4(other.g0));
}

Plane plane_homogeneous_magnitude_geometric_product(Plane self, HomogeneousMagnitude other) {
    return Plane(self.g0 * vec4(other.g0.x));
}

Plane plane_homogeneous_magnitude_regressive_product(Plane self, HomogeneousMagnitude other) {
    return Plane(self.g0 * vec4(other.g0.y));
}

Plane plane_homogeneous_magnitude_outer_product(Plane self, HomogeneousMagnitude other) {
    return Plane(self.g0 * vec4(other.g0.x));
}

Plane plane_homogeneous_magnitude_inner_product(Plane self, HomogeneousMagnitude other) {
    return Plane(self.g0 * vec4(other.g0.x));
}

Plane plane_homogeneous_magnitude_right_contraction(Plane self, HomogeneousMagnitude other) {
    return Plane(self.g0 * vec4(other.g0.x));
}

Plane plane_homogeneous_magnitude_right_anti_contraction(Plane self, HomogeneousMagnitude other) {
    return Plane(self.g0 * vec4(other.g0.y));
}

Scalar plane_radial_point_regressive_product(Plane self, RadialPoint other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z + self.g0.w * other.g1.x);
}

AntiScalar plane_radial_point_outer_product(Plane self, RadialPoint other) {
    return AntiScalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z + self.g0.w * other.g1.x);
}

Line plane_radial_point_inner_product(Plane self, RadialPoint other) {
    return Line(vec3(self.g0.y) * other.g0.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * other.g0.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * other.g0.xzy * vec3(0.0, -1.0, 1.0), vec3(self.g0.w) * other.g0);
}

Line plane_radial_point_right_contraction(Plane self, RadialPoint other) {
    return Line(vec3(self.g0.y) * other.g0.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * other.g0.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * other.g0.xzy * vec3(0.0, -1.0, 1.0), vec3(self.g0.w) * other.g0);
}

Flector plane_flat_point_add(Plane self, FlatPoint other) {
    return Flector(other.g0, self.g0);
}

Flector plane_flat_point_sub(Plane self, FlatPoint other) {
    return Flector(vec4(0.0) - other.g0, self.g0);
}

Line plane_flat_point_inner_anti_product(Plane self, FlatPoint other) {
    return Line(vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w) * vec3(-1.0), vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0));
}

Line plane_flat_point_left_anti_contraction(Plane self, FlatPoint other) {
    return Line(vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w) * vec3(-1.0), vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0));
}

RadialPoint plane_dipole_regressive_product(Plane self, Dipole other) {
    return RadialPoint(vec3(self.g0.y) * other.g1.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * other.g1.yxy * vec3(-1.0, 1.0, 0.0) - vec3(self.g0.w) * other.g0 + vec3(self.g0.x) * other.g1.xzy * vec3(0.0, -1.0, 1.0), vec2(self.g0.x) * vec2(other.g0.x, other.g2.x) * vec2(1.0, -1.0) + vec2(self.g0.y) * vec2(other.g0.y, other.g2.y) * vec2(1.0, -1.0) + vec2(self.g0.z) * vec2(other.g0.z, other.g2.z) * vec2(1.0, -1.0) + vec2(self.g0.x, self.g0.w) * vec2(other.g2.x, other.g2.w) * vec2(0.0, -1.0));
}

FlatPoint plane_dipole_inner_product(Plane self, Dipole other) {
    return FlatPoint(vec4(self.g0.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, -1.0));
}

Circle plane_dipole_inner_anti_product(Plane self, Dipole other) {
    return Circle(vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g2.w) * vec3(-1.0), vec3(self.g0.y) * vec3(other.g2.z, other.g2.z, other.g2.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g2.y, other.g2.x, other.g2.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g2.x, other.g2.z, other.g2.y) * vec3(0.0, -1.0, 1.0));
}

FlatPoint plane_dipole_right_contraction(Plane self, Dipole other) {
    return FlatPoint(vec4(self.g0.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, -1.0));
}

Circle plane_dipole_left_anti_contraction(Plane self, Dipole other) {
    return Circle(vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0), vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g2.w) * vec3(-1.0), vec3(self.g0.y) * vec3(other.g2.z, other.g2.z, other.g2.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g2.y, other.g2.x, other.g2.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g2.x, other.g2.z, other.g2.y) * vec3(0.0, -1.0, 1.0));
}

FlatPoint plane_line_regressive_product(Plane self, Line other) {
    return FlatPoint(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

Flector plane_line_geometric_anti_product(Plane self, Line other) {
    return Flector(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0), vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(-1.0, 1.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, -1.0, 1.0, 1.0));
}

Plane plane_line_inner_anti_product(Plane self, Line other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(-1.0, 1.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, -1.0, 1.0, 1.0));
}

Plane plane_line_left_anti_contraction(Plane self, Line other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(-1.0, 1.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, -1.0, 1.0, 1.0));
}

Dipole plane_circle_regressive_product(Plane self, Circle other) {
    return Dipole(vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0), vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w) * vec3(-1.0), vec4(self.g0.y) * vec4(other.g2.z, other.g2.z, other.g2.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g2.y, other.g2.x, other.g2.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g2.x, other.g2.z, other.g2.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

Sphere plane_circle_inner_anti_product(Plane self, Circle other) {
    return Sphere(vec2(self.g0.x) * vec2(other.g0.x, other.g2.x) * vec2(-1.0, 1.0) + vec2(self.g0.y) * vec2(other.g0.y, other.g2.y) * vec2(-1.0, 1.0) + vec2(self.g0.z) * vec2(other.g0.z, other.g2.z) * vec2(-1.0, 1.0), vec3(self.g0.y) * other.g1.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * other.g1.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * other.g1.xzy * vec3(0.0, -1.0, 1.0));
}

Sphere plane_circle_left_anti_contraction(Plane self, Circle other) {
    return Sphere(vec2(self.g0.x) * vec2(other.g0.x, other.g2.x) * vec2(-1.0, 1.0) + vec2(self.g0.y) * vec2(other.g0.y, other.g2.y) * vec2(-1.0, 1.0) + vec2(self.g0.z) * vec2(other.g0.z, other.g2.z) * vec2(-1.0, 1.0), vec3(self.g0.y) * other.g1.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * other.g1.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * other.g1.xzy * vec3(0.0, -1.0, 1.0));
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

Line plane_plane_regressive_product(Plane self, Plane other) {
    return Line(vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w));
}

AntiScalar plane_plane_inner_anti_product(Plane self, Plane other) {
    return AntiScalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

AntiScalar plane_plane_left_anti_contraction(Plane self, Plane other) {
    return AntiScalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

AntiScalar plane_plane_right_anti_contraction(Plane self, Plane other) {
    return AntiScalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

AntiScalar plane_plane_anti_scalar_product(Plane self, Plane other) {
    return AntiScalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z);
}

Sphere plane_sphere_add(Plane self, Sphere other) {
    return Sphere(vec2(self.g0.x, self.g0.w) * vec2(0.0, 1.0) + other.g0, vec3(self.g0.x, self.g0.y, self.g0.z) + other.g1);
}

Sphere plane_sphere_sub(Plane self, Sphere other) {
    return Sphere(vec2(self.g0.x, self.g0.w) * vec2(0.0, 1.0) - other.g0, vec3(self.g0.x, self.g0.y, self.g0.z) - other.g1);
}

Circle plane_sphere_regressive_product(Plane self, Sphere other) {
    return Circle(self.g0 * vec4(other.g0.x) * vec4(-1.0), vec3(self.g0.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * other.g1.xzy * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g0.w) * other.g1 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y));
}

AntiScalar plane_sphere_inner_anti_product(Plane self, Sphere other) {
    return AntiScalar(self.g0.x * other.g1.x + self.g0.y * other.g1.y + self.g0.z * other.g1.z);
}

AntiScalar plane_sphere_left_anti_contraction(Plane self, Sphere other) {
    return AntiScalar(self.g0.x * other.g1.x + self.g0.y * other.g1.y + self.g0.z * other.g1.z);
}

AntiScalar plane_sphere_right_anti_contraction(Plane self, Sphere other) {
    return AntiScalar(self.g0.x * other.g1.x + self.g0.y * other.g1.y + self.g0.z * other.g1.z);
}

AntiScalar plane_sphere_anti_scalar_product(Plane self, Sphere other) {
    return AntiScalar(self.g0.x * other.g1.x + self.g0.y * other.g1.y + self.g0.z * other.g1.z);
}

Flector plane_motor_regressive_product(Plane self, Motor other) {
    return Flector(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0), self.g0 * vec4(other.g0.w));
}

Flector plane_motor_geometric_anti_product(Plane self, Motor other) {
    return Flector(vec4(self.g0.x) * vec4(other.g1.w, other.g1.z, other.g1.y, other.g0.x) * vec4(-1.0, 1.0, -1.0, -1.0) + vec4(self.g0.y) * vec4(other.g1.z, other.g1.w, other.g1.x, other.g0.y) * vec4(-1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.w, other.g0.z) * vec4(1.0, -1.0, -1.0, -1.0) + self.g0.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0), vec4(self.g0.x) * vec4(other.g0.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g0.y) * vec4(other.g0.z, other.g0.w, other.g0.x, other.g1.y) * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.w, other.g1.z) * vec4(-1.0, 1.0, 1.0, 1.0) + self.g0.xxxw * other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0));
}

Plane plane_motor_right_anti_contraction(Plane self, Motor other) {
    return Plane(self.g0 * vec4(other.g0.w));
}

Flector plane_rotor_regressive_product(Plane self, Rotor other) {
    return Flector(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0), self.g0 * vec4(other.g0.w));
}

Flector plane_rotor_geometric_anti_product(Plane self, Rotor other) {
    return Flector(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0), vec4(self.g0.y) * other.g0.zwxz * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxwy * vec4(-1.0, 1.0, 1.0, 0.0) + self.g0.xxxw * other.g0.wzyw * vec4(1.0, -1.0, 1.0, 1.0));
}

Plane plane_rotor_inner_anti_product(Plane self, Rotor other) {
    return Plane(vec4(self.g0.y) * other.g0.zwxz * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxwy * vec4(-1.0, 1.0, 1.0, 0.0) + self.g0.xxxw * other.g0.wzyw * vec4(1.0, -1.0, 1.0, 1.0));
}

Plane plane_rotor_right_anti_contraction(Plane self, Rotor other) {
    return Plane(self.g0 * vec4(other.g0.w));
}

Plane plane_translator_inner_anti_product(Plane self, Translator other) {
    return Plane(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, 1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, 1.0));
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

Motor plane_flector_geometric_anti_product(Plane self, Flector other) {
    return Motor(vec4(self.g0.x) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g1.x) * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g0.y) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g1.y) * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g1.z) * vec4(1.0, -1.0, -1.0, 1.0), vec4(self.g0.x) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g0.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g0.z, other.g1.w, other.g0.x, other.g0.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g1.w, other.g0.z) * vec4(-1.0, 1.0, 1.0, -1.0) - vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g0.w));
}

AntiScalar plane_flector_right_anti_contraction(Plane self, Flector other) {
    return AntiScalar(self.g0.x * other.g1.x + self.g0.y * other.g1.y + self.g0.z * other.g1.z);
}

AntiScalar plane_flector_anti_scalar_product(Plane self, Flector other) {
    return AntiScalar(self.g0.x * other.g1.x + self.g0.y * other.g1.y + self.g0.z * other.g1.z);
}

Sphere plane_dilation_inner_anti_product(Plane self, Dilation other) {
    return Sphere(vec2(self.g0.y) * vec2(other.g0.x) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.y) * vec2(1.0, 0.0) + vec2(self.g0.x, self.g0.w) * other.g1, vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.y));
}

Plane plane_dilation_right_anti_contraction(Plane self, Dilation other) {
    return Plane(self.g0 * vec4(other.g1.y));
}

MultiVector plane_multi_vector_add(Plane self, MultiVector other) {
    return MultiVector(other.g0, other.g1, other.g2, other.g3, other.g4, other.g5, other.g6, other.g7, other.g8, self.g0 + other.g9);
}

MultiVector plane_multi_vector_sub(Plane self, MultiVector other) {
    return MultiVector(vec3(0.0) - other.g0, vec3(0.0) - other.g1, vec2(0.0) - other.g2, vec4(0.0) - other.g3, vec3(0.0) - other.g4, vec3(0.0) - other.g5, vec3(0.0) - other.g6, vec3(0.0) - other.g7, vec4(0.0) - other.g8, self.g0 - other.g9);
}

MultiVector plane_multi_vector_geometric_anti_product(Plane self, MultiVector other) {
    return MultiVector(vec3(self.g0.x) * vec3(other.g1.x, other.g8.x, other.g9.x) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g1.y, other.g8.y, other.g9.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.z) * vec3(other.g1.z, other.g8.z, other.g9.z) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.w, self.g0.x, self.g0.x) * vec3(other.g2.x) * vec3(1.0, 0.0, 0.0), vec3(self.g0.x) * vec3(other.g0.x, other.g5.z, other.g5.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g5.z, other.g0.x, other.g5.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g5.y, other.g5.x, other.g0.x) * vec3(-1.0, 1.0, 1.0) - vec3(self.g0.w) * other.g4, vec2(self.g0.x) * vec2(other.g4.x, other.g3.x) * vec2(1.0, -1.0) + vec2(self.g0.y) * vec2(other.g4.y, other.g3.y) * vec2(1.0, -1.0) + vec2(self.g0.z) * vec2(other.g4.z, other.g3.z) * vec2(1.0, -1.0) + vec2(self.g0.x, self.g0.w) * vec2(other.g3.x, other.g3.w) * vec2(0.0, -1.0), vec4(self.g0.x) * vec4(other.g2.y, other.g7.z, other.g7.y, other.g6.x) * vec4(-1.0, 1.0, -1.0, -1.0) + vec4(self.g0.y) * vec4(other.g7.z, other.g2.y, other.g7.x, other.g6.y) * vec4(-1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g7.y, other.g7.x, other.g2.y, other.g6.z) * vec4(1.0, -1.0, -1.0, -1.0) + self.g0.wwwx * vec4(other.g6.x, other.g6.y, other.g6.z, other.g6.x) * vec4(1.0, 1.0, 1.0, 0.0), vec3(self.g0.x) * vec3(other.g2.x, other.g8.z, other.g8.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g8.z, other.g2.x, other.g8.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g8.y, other.g8.x, other.g2.x) * vec3(-1.0, 1.0, 1.0), vec3(self.g0.x) * vec3(other.g8.w, other.g1.z, other.g1.y) * vec3(-1.0, 1.0, -1.0) + vec3(self.g0.y) * vec3(other.g1.z, other.g8.w, other.g1.x) * vec3(-1.0, -1.0, 1.0) + vec3(self.g0.z) * vec3(other.g1.y, other.g1.x, other.g8.w) * vec3(1.0, -1.0, -1.0) + vec3(self.g0.w) * vec3(other.g8.x, other.g8.y, other.g8.z), vec3(self.g0.x) * vec3(other.g3.w, other.g9.z, other.g9.y) * vec3(-1.0, 1.0, -1.0) + vec3(self.g0.y) * vec3(other.g9.z, other.g3.w, other.g9.x) * vec3(-1.0, -1.0, 1.0) + vec3(self.g0.z) * vec3(other.g9.y, other.g9.x, other.g3.w) * vec3(1.0, -1.0, -1.0), vec3(self.g0.x) * vec3(other.g9.w, other.g3.z, other.g3.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g3.z, other.g9.w, other.g3.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g3.y, other.g3.x, other.g9.w) * vec3(-1.0, 1.0, 1.0) - vec3(self.g0.w) * vec3(other.g9.x, other.g9.y, other.g9.z), vec4(self.g0.x) * vec4(other.g0.y, other.g4.z, other.g4.y, other.g5.x) * vec4(-1.0, 1.0, -1.0, -1.0) + vec4(self.g0.y) * vec4(other.g4.z, other.g0.y, other.g4.x, other.g5.y) * vec4(-1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g4.y, other.g4.x, other.g0.y, other.g5.z) * vec4(1.0, -1.0, -1.0, -1.0) + self.g0.xxxw * vec4(other.g0.x, other.g0.x, other.g0.x, other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0), vec4(self.g0.x) * vec4(other.g0.z, other.g6.z, other.g6.y, other.g7.x) * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g0.y) * vec4(other.g6.z, other.g0.z, other.g6.x, other.g7.y) * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g6.y, other.g6.x, other.g0.z, other.g7.z) * vec4(-1.0, 1.0, 1.0, 1.0) + self.g0.xxxw * vec4(other.g0.x, other.g0.x, other.g0.x, other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0));
}

AntiScalar plane_multi_vector_anti_scalar_product(Plane self, MultiVector other) {
    return AntiScalar(self.g0.x * other.g9.x + self.g0.y * other.g9.y + self.g0.z * other.g9.z);
}

Plane plane_scale(Plane self, float other) {
    return plane_scalar_geometric_product(self, Scalar(other));
}

Dipole plane_attitude(Plane self) {
    return plane_circle_regressive_product(self, Circle(vec4(0.0, 0.0, 0.0, 1.0), vec3(0.0), vec3(0.0)));
}

Sphere sphere_zero() {
    return Sphere(vec2(0.0), vec3(0.0));
}

Sphere sphere_one() {
    return Sphere(vec2(0.0), vec3(0.0));
}

int sphere_grade(Sphere self) {
    return 4;
}

int sphere_anti_grade(Sphere self) {
    return 1;
}

Sphere sphere_neg(Sphere self) {
    return Sphere(self.g0 * vec2(-1.0), self.g1 * vec3(-1.0));
}

Sphere sphere_automorphism(Sphere self) {
    return Sphere(self.g0, self.g1);
}

Sphere sphere_reversal(Sphere self) {
    return Sphere(self.g0, self.g1);
}

Sphere sphere_conjugation(Sphere self) {
    return Sphere(self.g0, self.g1);
}

RadialPoint sphere_dual(Sphere self) {
    return RadialPoint(self.g1, self.g0.yx);
}

Sphere sphere_anti_reversal(Sphere self) {
    return Sphere(self.g0, self.g1);
}

RadialPoint sphere_right_complement(Sphere self) {
    return RadialPoint(self.g1, self.g0.yx);
}

RadialPoint sphere_left_complement(Sphere self) {
    return RadialPoint(self.g1, self.g0.yx);
}

Sphere sphere_double_complement(Sphere self) {
    return Sphere(self.g0, self.g1);
}

Sphere sphere_scalar_geometric_product(Sphere self, Scalar other) {
    return Sphere(self.g0 * vec2(other.g0), self.g1 * vec3(other.g0));
}

Sphere sphere_scalar_outer_product(Sphere self, Scalar other) {
    return Sphere(self.g0 * vec2(other.g0), self.g1 * vec3(other.g0));
}

Sphere sphere_scalar_inner_product(Sphere self, Scalar other) {
    return Sphere(self.g0 * vec2(other.g0), self.g1 * vec3(other.g0));
}

Sphere sphere_scalar_right_contraction(Sphere self, Scalar other) {
    return Sphere(self.g0 * vec2(other.g0), self.g1 * vec3(other.g0));
}

Sphere sphere_anti_scalar_regressive_product(Sphere self, AntiScalar other) {
    return Sphere(self.g0 * vec2(other.g0), self.g1 * vec3(other.g0));
}

Sphere sphere_anti_scalar_geometric_anti_product(Sphere self, AntiScalar other) {
    return Sphere(self.g0 * vec2(other.g0), self.g1 * vec3(other.g0));
}

Sphere sphere_anti_scalar_inner_anti_product(Sphere self, AntiScalar other) {
    return Sphere(self.g0 * vec2(other.g0), self.g1 * vec3(other.g0));
}

Sphere sphere_anti_scalar_right_anti_contraction(Sphere self, AntiScalar other) {
    return Sphere(self.g0 * vec2(other.g0), self.g1 * vec3(other.g0));
}

Sphere sphere_homogeneous_magnitude_geometric_product(Sphere self, HomogeneousMagnitude other) {
    return Sphere(self.g0 * vec2(other.g0.x), self.g1 * vec3(other.g0.x));
}

Sphere sphere_homogeneous_magnitude_regressive_product(Sphere self, HomogeneousMagnitude other) {
    return Sphere(self.g0 * vec2(other.g0.y), self.g1 * vec3(other.g0.y));
}

Sphere sphere_homogeneous_magnitude_outer_product(Sphere self, HomogeneousMagnitude other) {
    return Sphere(self.g0 * vec2(other.g0.x), self.g1 * vec3(other.g0.x));
}

Sphere sphere_homogeneous_magnitude_inner_product(Sphere self, HomogeneousMagnitude other) {
    return Sphere(self.g0 * vec2(other.g0.x), self.g1 * vec3(other.g0.x));
}

Sphere sphere_homogeneous_magnitude_right_contraction(Sphere self, HomogeneousMagnitude other) {
    return Sphere(self.g0 * vec2(other.g0.x), self.g1 * vec3(other.g0.x));
}

Sphere sphere_homogeneous_magnitude_right_anti_contraction(Sphere self, HomogeneousMagnitude other) {
    return Sphere(self.g0 * vec2(other.g0.y), self.g1 * vec3(other.g0.y));
}

Scalar sphere_radial_point_regressive_product(Sphere self, RadialPoint other) {
    return Scalar(self.g0.x * other.g1.y + self.g0.y * other.g1.x + self.g1.x * other.g0.x + self.g1.y * other.g0.y + self.g1.z * other.g0.z);
}

AntiScalar sphere_radial_point_outer_product(Sphere self, RadialPoint other) {
    return AntiScalar(self.g0.x * other.g1.y + self.g0.y * other.g1.x + self.g1.x * other.g0.x + self.g1.y * other.g0.y + self.g1.z * other.g0.z);
}

RadialPoint sphere_flat_point_regressive_product(Sphere self, FlatPoint other) {
    return RadialPoint(vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g1.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g1.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + self.g0 * vec2(other.g0.w) * vec2(1.0, -1.0));
}

Line sphere_flat_point_inner_anti_product(Sphere self, FlatPoint other) {
    return Line(self.g1 * vec3(other.g0.w) * vec3(-1.0), vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0));
}

Line sphere_flat_point_left_anti_contraction(Sphere self, FlatPoint other) {
    return Line(self.g1 * vec3(other.g0.w) * vec3(-1.0), vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0));
}

RadialPoint sphere_dipole_regressive_product(Sphere self, Dipole other) {
    return RadialPoint(vec3(self.g0.x) * vec3(other.g2.x, other.g2.y, other.g2.z) - vec3(self.g0.y) * other.g0 + vec3(self.g1.y) * other.g1.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * other.g1.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.x) * other.g1.xzy * vec3(0.0, -1.0, 1.0), vec2(self.g1.x) * vec2(other.g0.x, other.g2.x) * vec2(1.0, -1.0) + vec2(self.g1.y) * vec2(other.g0.y, other.g2.y) * vec2(1.0, -1.0) + vec2(self.g1.z) * vec2(other.g0.z, other.g2.z) * vec2(1.0, -1.0) + self.g0 * vec2(other.g2.w) * vec2(1.0, -1.0));
}

Circle sphere_dipole_inner_anti_product(Sphere self, Dipole other) {
    return Circle(vec4(self.g1.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0), self.g1 * vec3(other.g2.w) * vec3(-1.0), vec3(self.g1.y) * vec3(other.g2.z, other.g2.z, other.g2.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g2.y, other.g2.x, other.g2.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.x) * vec3(other.g2.x, other.g2.z, other.g2.y) * vec3(0.0, -1.0, 1.0));
}

Circle sphere_dipole_left_anti_contraction(Sphere self, Dipole other) {
    return Circle(vec4(self.g1.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0), self.g1 * vec3(other.g2.w) * vec3(-1.0), vec3(self.g1.y) * vec3(other.g2.z, other.g2.z, other.g2.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g2.y, other.g2.x, other.g2.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.x) * vec3(other.g2.x, other.g2.z, other.g2.y) * vec3(0.0, -1.0, 1.0));
}

Dipole sphere_line_regressive_product(Sphere self, Line other) {
    return Dipole(vec3(self.g0.x) * other.g0, vec3(self.g0.x) * other.g1, vec4(self.g1.x) * vec4(other.g1.z, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.y, self.g0.y, self.g0.y, self.g0.x) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Plane sphere_line_inner_anti_product(Sphere self, Line other) {
    return Plane(vec4(self.g1.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(-1.0, 1.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, -1.0, 1.0, 1.0));
}

Plane sphere_line_left_anti_contraction(Sphere self, Line other) {
    return Plane(vec4(self.g1.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(-1.0, 1.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, -1.0, 1.0, 1.0));
}

Dipole sphere_circle_regressive_product(Sphere self, Circle other) {
    return Dipole(vec3(self.g0.x) * other.g1 + vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0), vec3(self.g0.x) * other.g2 + vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g1 * vec3(other.g0.w) * vec3(-1.0), vec4(self.g1.x) * vec4(other.g2.z, other.g2.z, other.g2.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g2.z, other.g2.z, other.g2.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g2.y, other.g2.x, other.g2.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.y, self.g0.y, self.g0.y, self.g0.x) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Sphere sphere_circle_inner_anti_product(Sphere self, Circle other) {
    return Sphere(vec2(self.g1.x) * vec2(other.g0.x, other.g2.x) * vec2(-1.0, 1.0) + vec2(self.g1.y) * vec2(other.g0.y, other.g2.y) * vec2(-1.0, 1.0) + vec2(self.g1.z) * vec2(other.g0.z, other.g2.z) * vec2(-1.0, 1.0), vec3(self.g1.y) * other.g1.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * other.g1.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.x) * other.g1.xzy * vec3(0.0, -1.0, 1.0));
}

Sphere sphere_circle_left_anti_contraction(Sphere self, Circle other) {
    return Sphere(vec2(self.g1.x) * vec2(other.g0.x, other.g2.x) * vec2(-1.0, 1.0) + vec2(self.g1.y) * vec2(other.g0.y, other.g2.y) * vec2(-1.0, 1.0) + vec2(self.g1.z) * vec2(other.g0.z, other.g2.z) * vec2(-1.0, 1.0), vec3(self.g1.y) * other.g1.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * other.g1.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.x) * other.g1.xzy * vec3(0.0, -1.0, 1.0));
}

Plane sphere_plane_into(Sphere self) {
    return Plane(vec4(self.g1.x, self.g1.y, self.g1.z, self.g0.y));
}

Sphere sphere_plane_add(Sphere self, Plane other) {
    return Sphere(self.g0 + vec2(other.g0.x, other.g0.w) * vec2(0.0, 1.0), self.g1 + vec3(other.g0.x, other.g0.y, other.g0.z));
}

Sphere sphere_plane_sub(Sphere self, Plane other) {
    return Sphere(self.g0 - vec2(other.g0.x, other.g0.w) * vec2(0.0, 1.0), self.g1 - vec3(other.g0.x, other.g0.y, other.g0.z));
}

Circle sphere_plane_regressive_product(Sphere self, Plane other) {
    return Circle(vec4(self.g0.x) * other.g0, vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g1.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g1 * vec3(other.g0.w));
}

AntiScalar sphere_plane_inner_anti_product(Sphere self, Plane other) {
    return AntiScalar(self.g1.x * other.g0.x + self.g1.y * other.g0.y + self.g1.z * other.g0.z);
}

AntiScalar sphere_plane_left_anti_contraction(Sphere self, Plane other) {
    return AntiScalar(self.g1.x * other.g0.x + self.g1.y * other.g0.y + self.g1.z * other.g0.z);
}

AntiScalar sphere_plane_right_anti_contraction(Sphere self, Plane other) {
    return AntiScalar(self.g1.x * other.g0.x + self.g1.y * other.g0.y + self.g1.z * other.g0.z);
}

AntiScalar sphere_plane_anti_scalar_product(Sphere self, Plane other) {
    return AntiScalar(self.g1.x * other.g0.x + self.g1.y * other.g0.y + self.g1.z * other.g0.z);
}

Sphere sphere_sphere_add(Sphere self, Sphere other) {
    return Sphere(self.g0 + other.g0, self.g1 + other.g1);
}

Sphere sphere_sphere_sub(Sphere self, Sphere other) {
    return Sphere(self.g0 - other.g0, self.g1 - other.g1);
}

Sphere sphere_sphere_mul(Sphere self, Sphere other) {
    return Sphere(self.g0 * other.g0, self.g1 * other.g1);
}

Sphere sphere_sphere_div(Sphere self, Sphere other) {
    return Sphere(vec2(self.g0.x, self.g0.y) * vec2(1.0, 1.0) / vec2(other.g0.x, other.g0.y) * vec2(1.0, 1.0), vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(1.0, 1.0, 1.0) / vec3(other.g1.x, other.g1.y, other.g1.z) * vec3(1.0, 1.0, 1.0));
}

Circle sphere_sphere_regressive_product(Sphere self, Sphere other) {
    return Circle(vec4(self.g0.x) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g0.y) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.x) * vec4(0.0, -1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.x) * vec4(0.0, 0.0, -1.0, 0.0) + vec4(self.g0.x, self.g0.x, self.g0.x, self.g0.y) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0), vec3(self.g1.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g1.x) * other.g1.xzy * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g0.y) * other.g1 + self.g1 * vec3(other.g0.y));
}

AntiScalar sphere_sphere_inner_anti_product(Sphere self, Sphere other) {
    return AntiScalar(self.g1.x * other.g1.x + self.g1.y * other.g1.y + self.g1.z * other.g1.z);
}

AntiScalar sphere_sphere_left_anti_contraction(Sphere self, Sphere other) {
    return AntiScalar(self.g1.x * other.g1.x + self.g1.y * other.g1.y + self.g1.z * other.g1.z);
}

AntiScalar sphere_sphere_right_anti_contraction(Sphere self, Sphere other) {
    return AntiScalar(self.g1.x * other.g1.x + self.g1.y * other.g1.y + self.g1.z * other.g1.z);
}

AntiScalar sphere_sphere_anti_scalar_product(Sphere self, Sphere other) {
    return AntiScalar(self.g1.x * other.g1.x + self.g1.y * other.g1.y + self.g1.z * other.g1.z);
}

Rotor sphere_motor_geometric_product(Sphere self, Motor other) {
    return Rotor(vec4(self.g0.x) * other.g1);
}

AntiScalar sphere_motor_outer_product(Sphere self, Motor other) {
    return AntiScalar(self.g0.x * other.g1.w);
}

Sphere sphere_motor_right_anti_contraction(Sphere self, Motor other) {
    return Sphere(self.g0 * vec2(other.g0.w), self.g1 * vec3(other.g0.w));
}

Sphere sphere_rotor_inner_anti_product(Sphere self, Rotor other) {
    return Sphere(self.g0 * vec2(other.g0.w), vec3(self.g1.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g0.z, other.g0.w, other.g0.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.w) * vec3(-1.0, 1.0, 1.0));
}

Sphere sphere_rotor_right_anti_contraction(Sphere self, Rotor other) {
    return Sphere(self.g0 * vec2(other.g0.w), self.g1 * vec3(other.g0.w));
}

Sphere sphere_translator_inner_anti_product(Sphere self, Translator other) {
    return Sphere(vec2(self.g1.x) * vec2(other.g0.x) * vec2(0.0, 1.0) + vec2(self.g1.y) * vec2(other.g0.y) * vec2(0.0, 1.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(0.0, 1.0) + self.g0 * vec2(other.g0.w), self.g1 * vec3(other.g0.w));
}

Sphere sphere_translator_right_anti_contraction(Sphere self, Translator other) {
    return Sphere(self.g0 * vec2(other.g0.w), self.g1 * vec3(other.g0.w));
}

AntiScalar sphere_flector_right_anti_contraction(Sphere self, Flector other) {
    return AntiScalar(self.g1.x * other.g1.x + self.g1.y * other.g1.y + self.g1.z * other.g1.z);
}

AntiScalar sphere_flector_anti_scalar_product(Sphere self, Flector other) {
    return AntiScalar(self.g1.x * other.g1.x + self.g1.y * other.g1.y + self.g1.z * other.g1.z);
}

Sphere sphere_dilation_inner_anti_product(Sphere self, Dilation other) {
    return Sphere(vec2(self.g1.x) * vec2(other.g1.x) * vec2(1.0, 0.0) + vec2(self.g1.y) * vec2(other.g0.x) * vec2(1.0, 0.0) + vec2(self.g1.z) * vec2(other.g0.y) * vec2(1.0, 0.0) + self.g0 * vec2(other.g1.y), self.g1 * vec3(other.g1.y));
}

Sphere sphere_dilation_right_anti_contraction(Sphere self, Dilation other) {
    return Sphere(self.g0 * vec2(other.g1.y), self.g1 * vec3(other.g1.y));
}

MultiVector sphere_multi_vector_add(Sphere self, MultiVector other) {
    return MultiVector(vec3(self.g0.x) * vec3(0.0, 1.0, 0.0) + other.g0, other.g1, other.g2, other.g3, other.g4, other.g5, other.g6, other.g7, other.g8, vec4(self.g1.x, self.g1.y, self.g1.z, self.g0.y) + other.g9);
}

MultiVector sphere_multi_vector_sub(Sphere self, MultiVector other) {
    return MultiVector(vec3(self.g0.x) * vec3(0.0, 1.0, 0.0) - other.g0, vec3(0.0) - other.g1, vec2(0.0) - other.g2, vec4(0.0) - other.g3, vec3(0.0) - other.g4, vec3(0.0) - other.g5, vec3(0.0) - other.g6, vec3(0.0) - other.g7, vec4(0.0) - other.g8, vec4(self.g1.x, self.g1.y, self.g1.z, self.g0.y) - other.g9);
}

MultiVector sphere_multi_vector_geometric_anti_product(Sphere self, MultiVector other) {
    return MultiVector(vec3(self.g0.y) * vec3(other.g2.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g1.x) * vec3(other.g1.x, other.g8.x, other.g9.x) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g1.y, other.g8.y, other.g9.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.z) * vec3(other.g1.z, other.g8.z, other.g9.z) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.x) * vec3(other.g2.y, other.g0.z, other.g2.x) * vec3(1.0, 1.0, 0.0), vec3(self.g0.x) * vec3(other.g3.x, other.g3.y, other.g3.z) - vec3(self.g0.y) * other.g4 + vec3(self.g1.x) * vec3(other.g0.x, other.g5.z, other.g5.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g5.z, other.g0.x, other.g5.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.z) * vec3(other.g5.y, other.g5.x, other.g0.x) * vec3(-1.0, 1.0, 1.0), vec2(self.g1.x) * vec2(other.g4.x, other.g3.x) * vec2(1.0, -1.0) + vec2(self.g1.y) * vec2(other.g4.y, other.g3.y) * vec2(1.0, -1.0) + vec2(self.g1.z) * vec2(other.g4.z, other.g3.z) * vec2(1.0, -1.0) + self.g0 * vec2(other.g3.w) * vec2(1.0, -1.0), vec4(self.g1.x) * vec4(other.g2.y, other.g7.z, other.g7.y, other.g6.x) * vec4(-1.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g7.z, other.g2.y, other.g7.x, other.g6.y) * vec4(-1.0, -1.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g7.y, other.g7.x, other.g2.y, other.g6.z) * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g0.y, self.g0.y, self.g0.y, self.g0.x) * vec4(other.g6.x, other.g6.y, other.g6.z, other.g6.x) * vec4(1.0, 1.0, 1.0, 0.0), vec3(self.g0.x) * other.g6 + vec3(self.g1.x) * vec3(other.g2.x, other.g8.z, other.g8.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g8.z, other.g2.x, other.g8.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.z) * vec3(other.g8.y, other.g8.x, other.g2.x) * vec3(-1.0, 1.0, 1.0), vec3(self.g0.x) * other.g7 + vec3(self.g0.y) * vec3(other.g8.x, other.g8.y, other.g8.z) + vec3(self.g1.x) * vec3(other.g8.w, other.g1.z, other.g1.y) * vec3(-1.0, 1.0, -1.0) + vec3(self.g1.y) * vec3(other.g1.z, other.g8.w, other.g1.x) * vec3(-1.0, -1.0, 1.0) + vec3(self.g1.z) * vec3(other.g1.y, other.g1.x, other.g8.w) * vec3(1.0, -1.0, -1.0), vec3(self.g1.x) * vec3(other.g3.w, other.g9.z, other.g9.y) * vec3(-1.0, 1.0, -1.0) + vec3(self.g1.y) * vec3(other.g9.z, other.g3.w, other.g9.x) * vec3(-1.0, -1.0, 1.0) + vec3(self.g1.z) * vec3(other.g9.y, other.g9.x, other.g3.w) * vec3(1.0, -1.0, -1.0), vec3(0.0) - vec3(self.g0.y) * vec3(other.g9.x, other.g9.y, other.g9.z) + vec3(self.g1.x) * vec3(other.g9.w, other.g3.z, other.g3.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g3.z, other.g9.w, other.g3.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.z) * vec3(other.g3.y, other.g3.x, other.g9.w) * vec3(-1.0, 1.0, 1.0), vec4(self.g0.x) * other.g9 + vec4(self.g1.x) * vec4(other.g0.y, other.g4.z, other.g4.y, other.g5.x) * vec4(-1.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g4.z, other.g0.y, other.g4.x, other.g5.y) * vec4(-1.0, -1.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g4.y, other.g4.x, other.g0.y, other.g5.z) * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g0.x, self.g0.x, self.g0.x, self.g0.y) * vec4(other.g0.x, other.g0.x, other.g0.x, other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0), vec4(self.g1.x) * vec4(other.g0.z, other.g6.z, other.g6.y, other.g7.x) * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g1.y) * vec4(other.g6.z, other.g0.z, other.g6.x, other.g7.y) * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g1.z) * vec4(other.g6.y, other.g6.x, other.g0.z, other.g7.z) * vec4(-1.0, 1.0, 1.0, 1.0) + vec4(self.g0.x, self.g0.x, self.g0.x, self.g0.y) * vec4(other.g0.x, other.g0.x, other.g0.x, other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0));
}

AntiScalar sphere_multi_vector_anti_scalar_product(Sphere self, MultiVector other) {
    return AntiScalar(self.g1.x * other.g9.x + self.g1.y * other.g9.y + self.g1.z * other.g9.z);
}

Sphere sphere_scale(Sphere self, float other) {
    return sphere_scalar_geometric_product(self, Scalar(other));
}

Dipole sphere_attitude(Sphere self) {
    return sphere_circle_regressive_product(self, Circle(vec4(0.0, 0.0, 0.0, 1.0), vec3(0.0), vec3(0.0)));
}

Motor motor_zero() {
    return Motor(vec4(0.0), vec4(0.0));
}

Motor motor_one() {
    return Motor(vec4(0.0), vec4(0.0));
}

Motor motor_neg(Motor self) {
    return Motor(self.g0 * vec4(-1.0), self.g1 * vec4(-1.0));
}

Motor motor_automorphism(Motor self) {
    return Motor(self.g0 * vec4(-1.0), self.g1 * vec4(-1.0));
}

Motor motor_reversal(Motor self) {
    return Motor(self.g0 * vec4(-1.0, -1.0, -1.0, 1.0), self.g1 * vec4(-1.0, -1.0, -1.0, 1.0));
}

Motor motor_conjugation(Motor self) {
    return Motor(self.g0 * vec4(1.0, 1.0, 1.0, -1.0), self.g1 * vec4(1.0, 1.0, 1.0, -1.0));
}

Motor motor_anti_reversal(Motor self) {
    return Motor(self.g0 * vec4(1.0, 1.0, 1.0, -1.0), self.g1 * vec4(1.0, 1.0, 1.0, -1.0));
}

Motor motor_double_complement(Motor self) {
    return Motor(self.g0, self.g1);
}

Motor motor_scalar_geometric_product(Motor self, Scalar other) {
    return Motor(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

Scalar motor_scalar_regressive_product(Motor self, Scalar other) {
    return Scalar(self.g0.w * other.g0);
}

Motor motor_scalar_outer_product(Motor self, Scalar other) {
    return Motor(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

Motor motor_scalar_inner_product(Motor self, Scalar other) {
    return Motor(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

Motor motor_scalar_right_contraction(Motor self, Scalar other) {
    return Motor(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
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

Motor motor_anti_scalar_regressive_product(Motor self, AntiScalar other) {
    return Motor(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

Motor motor_anti_scalar_geometric_anti_product(Motor self, AntiScalar other) {
    return Motor(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

Motor motor_anti_scalar_inner_anti_product(Motor self, AntiScalar other) {
    return Motor(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

AntiScalar motor_anti_scalar_left_anti_contraction(Motor self, AntiScalar other) {
    return AntiScalar(self.g0.w * other.g0);
}

Motor motor_anti_scalar_right_anti_contraction(Motor self, AntiScalar other) {
    return Motor(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

AntiScalar motor_anti_scalar_anti_scalar_product(Motor self, AntiScalar other) {
    return AntiScalar(self.g0.w * other.g0);
}

Motor motor_homogeneous_magnitude_geometric_product(Motor self, HomogeneousMagnitude other) {
    return Motor(self.g0 * vec4(other.g0.x), self.g1 * vec4(other.g0.x));
}

Motor motor_homogeneous_magnitude_outer_product(Motor self, HomogeneousMagnitude other) {
    return Motor(self.g0 * vec4(other.g0.x), self.g1 * vec4(other.g0.x));
}

Motor motor_homogeneous_magnitude_inner_product(Motor self, HomogeneousMagnitude other) {
    return Motor(self.g0 * vec4(other.g0.x), self.g1 * vec4(other.g0.x));
}

Motor motor_homogeneous_magnitude_right_contraction(Motor self, HomogeneousMagnitude other) {
    return Motor(self.g0 * vec4(other.g0.x), self.g1 * vec4(other.g0.x));
}

Motor motor_homogeneous_magnitude_right_anti_contraction(Motor self, HomogeneousMagnitude other) {
    return Motor(self.g0 * vec4(other.g0.y), self.g1 * vec4(other.g0.y));
}

AntiScalar motor_homogeneous_magnitude_anti_scalar_product(Motor self, HomogeneousMagnitude other) {
    return AntiScalar(self.g0.w * other.g0.y);
}

Flector motor_radial_point_geometric_product(Motor self, RadialPoint other) {
    return Flector(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g0.z) * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.y) * vec4(-1.0, 1.0, 0.0, 0.0) - vec4(self.g1.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.x) + vec4(self.g0.x, self.g1.x, self.g1.x, self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g0.x) * vec4(0.0, -1.0, 1.0, -1.0), vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g0.z) * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.y) * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g1.x, other.g1.x, other.g1.x, other.g0.x) * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g1.y) * vec4(other.g1.x, other.g1.x, other.g1.x, other.g0.y) * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g1.z) * vec4(other.g1.x, other.g1.x, other.g1.x, other.g0.z) * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g0.x) * vec4(0.0, 1.0, -1.0, 0.0));
}

RadialPoint motor_radial_point_regressive_product(Motor self, RadialPoint other) {
    return RadialPoint(vec3(self.g0.w) * other.g0, vec2(self.g0.w) * other.g1);
}

Flector motor_radial_point_outer_product(Motor self, RadialPoint other) {
    return Flector(vec4(0.0) - vec4(self.g1.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.x), vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g0.z) * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.y) * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g1.x) * vec4(other.g1.x, other.g1.x, other.g1.x, other.g0.x) * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g1.y) * vec4(other.g1.x, other.g1.x, other.g1.x, other.g0.y) * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g1.z) * vec4(other.g1.x, other.g1.x, other.g1.x, other.g0.z) * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g0.x) * vec4(0.0, 1.0, -1.0, 0.0));
}

FlatPoint motor_flat_point_regressive_product(Motor self, FlatPoint other) {
    return FlatPoint(vec4(self.g0.w) * other.g0);
}

Flector motor_flat_point_geometric_anti_product(Motor self, FlatPoint other) {
    return Flector(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.w) * other.g0 + vec4(self.g1.y) * vec4(other.g0.w) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.w) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.x, self.g0.x, self.g0.x, self.g0.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, 0.0), vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Flector motor_flat_point_inner_anti_product(Motor self, FlatPoint other) {
    return Flector(vec4(self.g0.w) * other.g0, vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Flector motor_flat_point_left_anti_contraction(Motor self, FlatPoint other) {
    return Flector(vec4(self.g0.w) * other.g0, vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Motor motor_dipole_geometric_product(Motor self, Dipole other) {
    return Motor(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.z, other.g0.z, other.g0.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0), vec4(self.g1.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

Line motor_line_into(Motor self) {
    return Line(vec3(self.g0.x, self.g0.y, self.g0.z), vec3(self.g1.x, self.g1.y, self.g1.z));
}

Motor motor_line_add(Motor self, Line other) {
    return Motor(self.g0 + vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0), self.g1 + vec4(other.g1.x, other.g1.y, other.g1.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Motor motor_line_sub(Motor self, Line other) {
    return Motor(self.g0 - vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0), self.g1 - vec4(other.g1.x, other.g1.y, other.g1.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Motor motor_line_geometric_anti_product(Motor self, Line other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g0.y) * vec4(1.0, 0.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.z) * vec4(-1.0, 1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g0.x) * vec4(0.0, -1.0, 1.0, -1.0), vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.y) * vec4(1.0, 0.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.z) * vec4(-1.0, 1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.z, other.g0.z, other.g0.y, other.g0.x) * vec4(0.0, -1.0, 1.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g0.y) * vec4(1.0, 0.0, -1.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.z) * vec4(-1.0, 1.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g1.x) * vec4(0.0, -1.0, 1.0, -1.0));
}

Translator motor_line_right_anti_contraction(Motor self, Line other) {
    return Translator(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.w, self.g1.w, self.g1.w, self.g0.x) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, -1.0));
}

AntiScalar motor_line_anti_scalar_product(Motor self, Line other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

Flector motor_circle_geometric_product(Motor self, Circle other) {
    return Flector(vec4(self.g1.x) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g1.z) * other.g0.wwwz * vec4(0.0, 0.0, -1.0, 1.0) + self.g0.xxxw * other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0), vec4(self.g1.x) * other.g0.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) - vec4(self.g1.w) * other.g0 + self.g0.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

Plane motor_circle_outer_product(Motor self, Circle other) {
    return Plane(vec4(0.0) - vec4(self.g1.w) * other.g0);
}

Translator motor_circle_right_anti_contraction(Motor self, Circle other) {
    return Translator(vec4(self.g0.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.w, self.g1.w, self.g1.w, self.g0.x) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, -1.0));
}

AntiScalar motor_circle_anti_scalar_product(Motor self, Circle other) {
    return AntiScalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z);
}

Flector motor_plane_regressive_product(Motor self, Plane other) {
    return Flector(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x, self.g1.x, self.g1.x, self.g0.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0), vec4(self.g0.w) * other.g0);
}

Flector motor_plane_geometric_anti_product(Motor self, Plane other) {
    return Flector(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x, self.g1.x, self.g1.x, self.g0.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0), vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.w) * other.g0 + vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x, self.g0.x, self.g0.x, self.g1.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, -1.0));
}

Plane motor_plane_left_anti_contraction(Motor self, Plane other) {
    return Plane(vec4(self.g0.w) * other.g0);
}

Rotor motor_sphere_geometric_product(Motor self, Sphere other) {
    return Rotor(self.g1 * vec4(other.g0.x));
}

AntiScalar motor_sphere_outer_product(Motor self, Sphere other) {
    return AntiScalar(self.g1.w * other.g0.x);
}

Sphere motor_sphere_left_anti_contraction(Motor self, Sphere other) {
    return Sphere(vec2(self.g0.w) * other.g0, vec3(self.g0.w) * other.g1);
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
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.w) * vec4(1.0, 1.0, 1.0, 1.0), vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.w) * vec4(1.0, 1.0, 1.0, 1.0));
}

Motor motor_motor_regressive_product(Motor self, Motor other) {
    return Motor(vec4(self.g0.w) * other.g0 + self.g0.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0), vec4(self.g0.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.w) * other.g1 + vec4(self.g1.x) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

Motor motor_motor_geometric_anti_product(Motor self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * other.g0.zwxy * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g0, vec4(self.g0.x) * other.g1.wzyx * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * other.g1.zwxy * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * other.g1.yxwz * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g1 + vec4(self.g1.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g1.y) * other.g0.zwxy * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.z) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.w) * other.g0);
}

Motor motor_motor_inner_anti_product(Motor self, Motor other) {
    return Motor(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g0 + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0), vec4(self.g0.w) * other.g1 + vec4(self.g1.x) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.w) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.w) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * other.g0 + self.g0.xyzx * other.g1.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

Motor motor_motor_left_anti_contraction(Motor self, Motor other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.w) * other.g0 + vec4(self.g0.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0), vec4(self.g0.w) * other.g1 + self.g0.xyzx * other.g1.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

Motor motor_motor_right_anti_contraction(Motor self, Motor other) {
    return Motor(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0), vec4(self.g1.w) * other.g0 + self.g1.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

AntiScalar motor_motor_anti_scalar_product(Motor self, Motor other) {
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

Motor motor_rotor_regressive_product(Motor self, Rotor other) {
    return Motor(vec4(self.g0.w) * other.g0 + self.g0.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0), vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Motor motor_rotor_geometric_anti_product(Motor self, Rotor other) {
    return Motor(vec4(self.g0.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * other.g0.zwxy * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g0, vec4(self.g1.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g1.y) * other.g0.zwxy * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.z) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.w) * other.g0);
}

Motor motor_rotor_inner_anti_product(Motor self, Rotor other) {
    return Motor(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g0 + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0), vec4(self.g1.w) * other.g0 + self.g1.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

Rotor motor_rotor_left_anti_contraction(Motor self, Rotor other) {
    return Rotor(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.w) * other.g0 + vec4(self.g0.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

Motor motor_rotor_right_anti_contraction(Motor self, Rotor other) {
    return Motor(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0), vec4(self.g1.w) * other.g0 + self.g1.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

AntiScalar motor_rotor_anti_scalar_product(Motor self, Rotor other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z + self.g0.w * other.g0.w);
}

Translator motor_translator_into(Motor self) {
    return Translator(vec4(self.g1.x, self.g1.y, self.g1.z, self.g0.w));
}

Motor motor_translator_add(Motor self, Translator other) {
    return Motor(self.g0 + other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0), self.g1 + other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0));
}

Motor motor_translator_sub(Motor self, Translator other) {
    return Motor(self.g0 - other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0), self.g1 - other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0));
}

Motor motor_translator_regressive_product(Motor self, Translator other) {
    return Motor(self.g0 * vec4(other.g0.w), vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.w) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.w) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + self.g0.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

Motor motor_translator_geometric_anti_product(Motor self, Translator other) {
    return Motor(self.g0 * vec4(other.g0.w), vec4(self.g0.y) * other.g0.zzxy * vec4(1.0, 0.0, -1.0, -1.0) + vec4(self.g0.z) * other.g0.yxyz * vec4(-1.0, 1.0, 0.0, -1.0) + vec4(self.g0.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.w) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.w) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x, self.g0.x, self.g0.x, self.g0.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0));
}

Motor motor_translator_inner_anti_product(Motor self, Translator other) {
    return Motor(self.g0 * vec4(other.g0.w), vec4(self.g1.x) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.w) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.w) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g0.w, self.g0.w, self.g0.w, self.g1.w) * other.g0);
}

Translator motor_translator_left_anti_contraction(Motor self, Translator other) {
    return Translator(vec4(self.g0.w) * other.g0);
}

Motor motor_translator_right_anti_contraction(Motor self, Translator other) {
    return Motor(self.g0 * vec4(other.g0.w), self.g1 * vec4(other.g0.w));
}

AntiScalar motor_translator_anti_scalar_product(Motor self, Translator other) {
    return AntiScalar(self.g0.w * other.g0.w);
}

Flector motor_flector_regressive_product(Motor self, Flector other) {
    return Flector(vec4(self.g0.y) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g0 + vec4(self.g1.y) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g1.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x, self.g1.x, self.g1.x, self.g0.x) * other.g1.wzyx * vec4(1.0, -1.0, 1.0, -1.0), vec4(self.g0.w) * other.g1);
}

Flector motor_flector_geometric_anti_product(Motor self, Flector other) {
    return Flector(vec4(self.g0.x) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g0.z, other.g1.w, other.g0.x, other.g1.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g1.w, other.g1.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g0 + vec4(self.g1.y) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g1.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g1.y) * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g1.w) * other.g1.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, -1.0, 1.0, 0.0), vec4(self.g0.x) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g1 + vec4(self.g1.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

Flector motor_flector_inner_anti_product(Motor self, Flector other) {
    return Flector(vec4(self.g0.w) * other.g0 + self.g1.wwwx * other.g1.xyzx * vec4(1.0, 1.0, 1.0, 0.0), vec4(self.g0.x) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g1 + vec4(self.g1.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

Flector motor_flector_left_anti_contraction(Motor self, Flector other) {
    return Flector(vec4(self.g0.w) * other.g0, vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g1 + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Flector motor_dilation_geometric_product(Motor self, Dilation other) {
    return Flector(vec4(self.g1.x) * vec4(other.g0.z, other.g0.z, other.g0.z, other.g1.x) * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.z, other.g0.z, other.g0.z, other.g0.x) * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z, other.g0.z, other.g0.z, other.g0.y) * vec4(0.0, 0.0, 1.0, -1.0) + self.g0.xxxw * vec4(other.g0.x, other.g0.x, other.g0.x, other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0), vec4(self.g1.x) * vec4(other.g0.y, other.g0.y, other.g0.x, other.g0.y) * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.y, other.g0.y, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.x, other.g1.x, other.g0.x, other.g0.x) * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g1.w) * vec4(other.g1.x, other.g0.x, other.g0.y, other.g0.z) + self.g0.xyzx * vec4(other.g0.z, other.g0.z, other.g0.z, other.g0.x) * vec4(-1.0, -1.0, -1.0, 0.0));
}

Plane motor_dilation_outer_product(Motor self, Dilation other) {
    return Plane(vec4(self.g1.w) * vec4(other.g1.x, other.g0.x, other.g0.y, other.g0.z));
}

Dilation motor_dilation_left_anti_contraction(Motor self, Dilation other) {
    return Dilation(vec3(self.g0.w) * other.g0, vec2(self.g0.w) * other.g1);
}

Motor motor_dilation_right_anti_contraction(Motor self, Dilation other) {
    return Motor(self.g0 * vec4(other.g1.y), self.g1 * vec4(other.g1.y));
}

AntiScalar motor_dilation_anti_scalar_product(Motor self, Dilation other) {
    return AntiScalar(self.g0.w * other.g1.y);
}

MultiVector motor_multi_vector_add(Motor self, MultiVector other) {
    return MultiVector(vec3(self.g0.x, self.g0.x, self.g0.w) * vec3(0.0, 0.0, 1.0) + other.g0, other.g1, vec2(self.g0.x, self.g1.w) * vec2(0.0, 1.0) + other.g2, other.g3, other.g4, other.g5, vec3(self.g0.x, self.g0.y, self.g0.z) + other.g6, vec3(self.g1.x, self.g1.y, self.g1.z) + other.g7, other.g8, other.g9);
}

MultiVector motor_multi_vector_sub(Motor self, MultiVector other) {
    return MultiVector(vec3(self.g0.x, self.g0.x, self.g0.w) * vec3(0.0, 0.0, 1.0) - other.g0, vec3(0.0) - other.g1, vec2(self.g0.x, self.g1.w) * vec2(0.0, 1.0) - other.g2, vec4(0.0) - other.g3, vec3(0.0) - other.g4, vec3(0.0) - other.g5, vec3(self.g0.x, self.g0.y, self.g0.z) - other.g6, vec3(self.g1.x, self.g1.y, self.g1.z) - other.g7, vec4(0.0) - other.g8, vec4(0.0) - other.g9);
}

MultiVector motor_multi_vector_regressive_product(Motor self, MultiVector other) {
    return MultiVector(vec3(self.g0.y) * vec3(other.g5.y) * vec3(-1.0, 0.0, 0.0) + vec3(self.g0.z) * vec3(other.g5.z) * vec3(-1.0, 0.0, 0.0) + vec3(self.g0.w) * other.g0 + vec3(self.g1.x) * vec3(other.g4.x) * vec3(-1.0, 0.0, 0.0) + vec3(self.g1.y) * vec3(other.g4.y) * vec3(-1.0, 0.0, 0.0) + vec3(self.g1.z) * vec3(other.g4.z) * vec3(-1.0, 0.0, 0.0) + vec3(self.g1.w) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g0.x) * vec3(other.g5.x) * vec3(-1.0, 0.0, 0.0), vec3(self.g0.w) * other.g1 + vec3(self.g1.x) * vec3(other.g8.z, other.g8.z, other.g8.y) * vec3(0.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g8.z, other.g8.z, other.g8.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g8.y, other.g8.x, other.g8.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g8.w), vec2(0.0) - vec2(self.g0.x) * vec2(other.g8.x, other.g7.x) - vec2(self.g0.y) * vec2(other.g8.y, other.g7.y) - vec2(self.g0.z) * vec2(other.g8.z, other.g7.z) + vec2(self.g0.w) * other.g2 + vec2(self.g1.y) * vec2(other.g6.y) * vec2(0.0, -1.0) + vec2(self.g1.z) * vec2(other.g6.z) * vec2(0.0, -1.0) + vec2(self.g1.w) * vec2(other.g0.z) * vec2(0.0, 1.0) + vec2(self.g1.x) * vec2(other.g6.x) * vec2(0.0, -1.0), vec4(self.g0.y) * other.g9.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g9.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g3 + vec4(self.g1.y) * other.g9.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g9.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x, self.g1.x, self.g1.x, self.g0.x) * other.g9.wzyx * vec4(1.0, -1.0, 1.0, -1.0), vec3(self.g0.w) * other.g4 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), vec3(self.g0.w) * other.g5 + vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g0.y), vec3(self.g0.w) * other.g6 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.z), vec3(self.g0.w) * other.g7 + vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(other.g0.z), vec4(self.g0.w) * other.g8, vec4(self.g0.w) * other.g9);
}

MultiVector motor_multi_vector_geometric_anti_product(Motor self, MultiVector other) {
    return MultiVector(vec3(0.0) - vec3(self.g0.x) * vec3(other.g5.x, other.g4.x, other.g6.x) - vec3(self.g0.y) * vec3(other.g5.y, other.g4.y, other.g6.y) - vec3(self.g0.z) * vec3(other.g5.z, other.g4.z, other.g6.z) + vec3(self.g0.w) * other.g0 + vec3(self.g1.y) * vec3(other.g4.y) * vec3(-1.0, 0.0, 0.0) + vec3(self.g1.z) * vec3(other.g4.z) * vec3(-1.0, 0.0, 0.0) + vec3(self.g1.w) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g1.x) * vec3(other.g4.x) * vec3(-1.0, 0.0, 0.0), vec3(self.g0.x) * vec3(other.g8.w, other.g1.z, other.g1.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g1.z, other.g8.w, other.g1.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g1.y, other.g1.x, other.g8.w) * vec3(-1.0, 1.0, 1.0) + vec3(self.g0.w) * other.g1 + vec3(self.g1.x) * vec3(other.g2.x, other.g8.z, other.g8.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g8.z, other.g2.x, other.g8.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.z) * vec3(other.g8.y, other.g8.x, other.g2.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g1.w) * vec3(other.g8.x, other.g8.y, other.g8.z), vec2(0.0) - vec2(self.g0.x) * vec2(other.g8.x, other.g7.x) - vec2(self.g0.y) * vec2(other.g8.y, other.g7.y) - vec2(self.g0.z) * vec2(other.g8.z, other.g7.z) + vec2(self.g0.w) * other.g2 + vec2(self.g1.y) * vec2(other.g6.y) * vec2(0.0, -1.0) + vec2(self.g1.z) * vec2(other.g6.z) * vec2(0.0, -1.0) + vec2(self.g1.w) * vec2(other.g0.z) * vec2(0.0, 1.0) + vec2(self.g1.x) * vec2(other.g6.x) * vec2(0.0, -1.0), vec4(self.g0.x) * vec4(other.g9.w, other.g3.z, other.g3.y, other.g9.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g3.z, other.g9.w, other.g3.x, other.g9.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g3.y, other.g3.x, other.g9.w, other.g9.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g3 + vec4(self.g1.y) * vec4(other.g9.z, other.g3.w, other.g9.x, other.g9.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g1.z) * vec4(other.g9.y, other.g9.x, other.g3.w, other.g9.y) * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g1.w) * other.g9.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g3.w, other.g9.z, other.g9.y, other.g3.x) * vec4(1.0, -1.0, 1.0, 0.0), vec3(self.g0.x) * vec3(other.g0.y, other.g4.z, other.g4.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g4.z, other.g0.y, other.g4.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g4.y, other.g4.x, other.g0.y) * vec3(-1.0, 1.0, 1.0) + vec3(self.g0.w) * other.g4, vec3(self.g0.x) * vec3(other.g0.x, other.g5.z, other.g5.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g5.z, other.g0.x, other.g5.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g5.y, other.g5.x, other.g0.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g0.w) * other.g5 + vec3(self.g1.x) * vec3(other.g0.y, other.g4.z, other.g4.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g4.z, other.g0.y, other.g4.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.z) * vec3(other.g4.y, other.g4.x, other.g0.y) * vec3(-1.0, 1.0, 1.0) + vec3(self.g1.w) * other.g4, vec3(self.g0.x) * vec3(other.g0.z, other.g6.z, other.g6.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g6.z, other.g0.z, other.g6.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g6.y, other.g6.x, other.g0.z) * vec3(-1.0, 1.0, 1.0) + vec3(self.g0.w) * other.g6, vec3(self.g0.x) * vec3(other.g2.y, other.g7.z, other.g7.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g7.z, other.g2.y, other.g7.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g7.y, other.g7.x, other.g2.y) * vec3(-1.0, 1.0, 1.0) + vec3(self.g0.w) * other.g7 + vec3(self.g1.x) * vec3(other.g0.z, other.g6.z, other.g6.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g6.z, other.g0.z, other.g6.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.z) * vec3(other.g6.y, other.g6.x, other.g0.z) * vec3(-1.0, 1.0, 1.0) + vec3(self.g1.w) * other.g6, vec4(self.g0.x) * vec4(other.g2.x, other.g8.z, other.g8.y, other.g1.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g8.z, other.g2.x, other.g8.x, other.g1.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g8.y, other.g8.x, other.g2.x, other.g1.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g8 + vec4(self.g1.y) * vec4(other.g8.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g8.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g2.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g8.x) * vec4(0.0, 0.0, 0.0, -1.0), vec4(self.g0.x) * vec4(other.g3.w, other.g9.z, other.g9.y, other.g3.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g9.z, other.g3.w, other.g9.x, other.g3.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g9.y, other.g9.x, other.g3.w, other.g3.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g9 + vec4(self.g1.y) * vec4(other.g9.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g9.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g3.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g9.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

MultiVector motor_multi_vector_inner_anti_product(Motor self, MultiVector other) {
    return MultiVector(vec3(self.g0.y) * vec3(other.g4.y, other.g4.y, other.g6.y) * vec3(0.0, -1.0, -1.0) + vec3(self.g0.z) * vec3(other.g4.z, other.g4.z, other.g6.z) * vec3(0.0, -1.0, -1.0) + vec3(self.g0.w) * other.g0 + vec3(self.g0.x) * vec3(other.g4.x, other.g4.x, other.g6.x) * vec3(0.0, -1.0, -1.0), vec3(self.g0.w) * other.g1, vec2(self.g0.w) * other.g2 + vec2(self.g1.x, self.g1.w) * vec2(other.g0.x, other.g0.z) * vec2(0.0, 1.0), vec4(self.g0.w) * other.g3 + self.g1.wwwx * other.g9.xyzx * vec4(1.0, 1.0, 1.0, 0.0), vec3(self.g0.w) * other.g4, vec3(self.g0.w) * other.g5 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x), vec3(self.g0.w) * other.g6 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.z), vec3(self.g0.w) * other.g7 + vec3(self.g1.x) * vec3(other.g0.z) * vec3(1.0, 0.0, 0.0) + vec3(self.g1.y) * vec3(other.g0.z) * vec3(0.0, 1.0, 0.0) + vec3(self.g1.z) * vec3(other.g0.z) * vec3(0.0, 0.0, 1.0) + vec3(self.g1.w) * other.g6 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g2.y), vec4(self.g0.y) * vec4(other.g2.x, other.g2.x, other.g2.x, other.g1.y) * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g2.x, other.g2.x, other.g2.x, other.g1.z) * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g8 + vec4(self.g0.x) * vec4(other.g2.x, other.g2.x, other.g2.x, other.g1.x) * vec4(1.0, 0.0, 0.0, -1.0), vec4(self.g0.x) * vec4(other.g3.w, other.g9.z, other.g9.y, other.g3.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g9.z, other.g3.w, other.g9.x, other.g3.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g9.y, other.g9.x, other.g3.w, other.g3.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g9 + vec4(self.g1.y) * vec4(other.g9.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g9.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g3.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g9.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

MultiVector motor_multi_vector_left_anti_contraction(Motor self, MultiVector other) {
    return MultiVector(vec3(self.g0.y) * vec3(other.g4.y, other.g4.y, other.g6.y) * vec3(0.0, -1.0, -1.0) + vec3(self.g0.z) * vec3(other.g4.z, other.g4.z, other.g6.z) * vec3(0.0, -1.0, -1.0) + vec3(self.g0.w) * other.g0 + vec3(self.g0.x) * vec3(other.g4.x, other.g4.x, other.g6.x) * vec3(0.0, -1.0, -1.0), vec3(self.g0.w) * other.g1, vec2(self.g0.w) * other.g2, vec4(self.g0.w) * other.g3, vec3(self.g0.w) * other.g4, vec3(self.g0.w) * other.g5 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x), vec3(self.g0.w) * other.g6, vec3(self.g0.w) * other.g7 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g2.y), vec4(self.g0.y) * vec4(other.g2.x, other.g2.x, other.g2.x, other.g1.y) * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g2.x, other.g2.x, other.g2.x, other.g1.z) * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g8 + vec4(self.g0.x) * vec4(other.g2.x, other.g2.x, other.g2.x, other.g1.x) * vec4(1.0, 0.0, 0.0, -1.0), vec4(self.g0.y) * other.g3.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g3.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g9 + vec4(self.g0.x) * other.g3.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

AntiScalar motor_multi_vector_anti_scalar_product(Motor self, MultiVector other) {
    return AntiScalar(0.0 - self.g0.x * other.g6.x - self.g0.y * other.g6.y - self.g0.z * other.g6.z + self.g0.w * other.g0.z);
}

Motor motor_scale(Motor self, float other) {
    return motor_scalar_geometric_product(self, Scalar(other));
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
    return Rotor(self.g0 * vec4(-1.0));
}

Rotor rotor_reversal(Rotor self) {
    return Rotor(self.g0 * vec4(-1.0, -1.0, -1.0, 1.0));
}

Rotor rotor_conjugation(Rotor self) {
    return Rotor(self.g0 * vec4(1.0, 1.0, 1.0, -1.0));
}

Rotor rotor_anti_reversal(Rotor self) {
    return Rotor(self.g0 * vec4(1.0, 1.0, 1.0, -1.0));
}

Rotor rotor_double_complement(Rotor self) {
    return Rotor(self.g0);
}

Rotor rotor_scalar_geometric_product(Rotor self, Scalar other) {
    return Rotor(self.g0 * vec4(other.g0));
}

Scalar rotor_scalar_regressive_product(Rotor self, Scalar other) {
    return Scalar(self.g0.w * other.g0);
}

Rotor rotor_scalar_outer_product(Rotor self, Scalar other) {
    return Rotor(self.g0 * vec4(other.g0));
}

Rotor rotor_scalar_inner_product(Rotor self, Scalar other) {
    return Rotor(self.g0 * vec4(other.g0));
}

Rotor rotor_scalar_right_contraction(Rotor self, Scalar other) {
    return Rotor(self.g0 * vec4(other.g0));
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

Rotor rotor_anti_scalar_regressive_product(Rotor self, AntiScalar other) {
    return Rotor(self.g0 * vec4(other.g0));
}

Rotor rotor_anti_scalar_geometric_anti_product(Rotor self, AntiScalar other) {
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

Rotor rotor_homogeneous_magnitude_geometric_product(Rotor self, HomogeneousMagnitude other) {
    return Rotor(self.g0 * vec4(other.g0.x));
}

Rotor rotor_homogeneous_magnitude_outer_product(Rotor self, HomogeneousMagnitude other) {
    return Rotor(self.g0 * vec4(other.g0.x));
}

Rotor rotor_homogeneous_magnitude_inner_product(Rotor self, HomogeneousMagnitude other) {
    return Rotor(self.g0 * vec4(other.g0.x));
}

Rotor rotor_homogeneous_magnitude_right_contraction(Rotor self, HomogeneousMagnitude other) {
    return Rotor(self.g0 * vec4(other.g0.x));
}

Rotor rotor_homogeneous_magnitude_right_anti_contraction(Rotor self, HomogeneousMagnitude other) {
    return Rotor(self.g0 * vec4(other.g0.y));
}

AntiScalar rotor_homogeneous_magnitude_anti_scalar_product(Rotor self, HomogeneousMagnitude other) {
    return AntiScalar(self.g0.w * other.g0.y);
}

RadialPoint rotor_radial_point_regressive_product(Rotor self, RadialPoint other) {
    return RadialPoint(vec3(self.g0.w) * other.g0, vec2(self.g0.w) * other.g1);
}

FlatPoint rotor_flat_point_regressive_product(Rotor self, FlatPoint other) {
    return FlatPoint(vec4(self.g0.w) * other.g0);
}

Flector rotor_flat_point_geometric_anti_product(Rotor self, FlatPoint other) {
    return Flector(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.w) * other.g0 + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, 0.0), vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Flector rotor_flat_point_inner_anti_product(Rotor self, FlatPoint other) {
    return Flector(vec4(self.g0.w) * other.g0, vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Flector rotor_flat_point_left_anti_contraction(Rotor self, FlatPoint other) {
    return Flector(vec4(self.g0.w) * other.g0, vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Rotor rotor_dipole_geometric_product(Rotor self, Dipole other) {
    return Rotor(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

AntiScalar rotor_dipole_outer_product(Rotor self, Dipole other) {
    return AntiScalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z);
}

Motor rotor_line_geometric_anti_product(Rotor self, Line other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g0.y) * vec4(1.0, 0.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.z) * vec4(-1.0, 1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g0.x) * vec4(0.0, -1.0, 1.0, -1.0), vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.y) * vec4(1.0, 0.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.z) * vec4(-1.0, 1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g1.x) * vec4(0.0, -1.0, 1.0, -1.0));
}

AntiScalar rotor_line_right_anti_contraction(Rotor self, Line other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

AntiScalar rotor_line_anti_scalar_product(Rotor self, Line other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

AntiScalar rotor_circle_right_anti_contraction(Rotor self, Circle other) {
    return AntiScalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z);
}

AntiScalar rotor_circle_anti_scalar_product(Rotor self, Circle other) {
    return AntiScalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z);
}

Flector rotor_plane_regressive_product(Rotor self, Plane other) {
    return Flector(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0), vec4(self.g0.w) * other.g0);
}

Flector rotor_plane_geometric_anti_product(Rotor self, Plane other) {
    return Flector(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0), vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.w) * other.g0 + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, 0.0));
}

Plane rotor_plane_inner_anti_product(Rotor self, Plane other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.w) * other.g0 + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, 0.0));
}

Plane rotor_plane_left_anti_contraction(Rotor self, Plane other) {
    return Plane(vec4(self.g0.w) * other.g0);
}

Sphere rotor_sphere_inner_anti_product(Rotor self, Sphere other) {
    return Sphere(vec2(self.g0.w) * other.g0, vec3(self.g0.y) * other.g1.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * other.g1.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.w) * other.g1 + vec3(self.g0.x) * other.g1.xzy * vec3(0.0, -1.0, 1.0));
}

Sphere rotor_sphere_left_anti_contraction(Rotor self, Sphere other) {
    return Sphere(vec2(self.g0.w) * other.g0, vec3(self.g0.w) * other.g1);
}

Motor rotor_motor_add(Rotor self, Motor other) {
    return Motor(self.g0 + other.g0, other.g1);
}

Motor rotor_motor_sub(Rotor self, Motor other) {
    return Motor(self.g0 - other.g0, vec4(0.0) - other.g1);
}

Motor rotor_motor_regressive_product(Rotor self, Motor other) {
    return Motor(vec4(self.g0.w) * other.g0 + self.g0.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0), vec4(self.g0.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.w) * other.g1 + vec4(self.g0.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

Motor rotor_motor_geometric_anti_product(Rotor self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * other.g0.zwxy * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g0, vec4(self.g0.x) * other.g1.wzyx * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * other.g1.zwxy * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * other.g1.yxwz * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g1);
}

Motor rotor_motor_inner_anti_product(Rotor self, Motor other) {
    return Motor(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g0 + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0), vec4(self.g0.w) * other.g1 + self.g0.xyzx * other.g1.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

Motor rotor_motor_left_anti_contraction(Rotor self, Motor other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.w) * other.g0 + vec4(self.g0.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0), vec4(self.g0.w) * other.g1 + self.g0.xyzx * other.g1.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

Rotor rotor_motor_right_anti_contraction(Rotor self, Motor other) {
    return Rotor(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

AntiScalar rotor_motor_anti_scalar_product(Rotor self, Motor other) {
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

Rotor rotor_rotor_regressive_product(Rotor self, Rotor other) {
    return Rotor(vec4(self.g0.w) * other.g0 + self.g0.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

Rotor rotor_rotor_geometric_anti_product(Rotor self, Rotor other) {
    return Rotor(vec4(self.g0.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * other.g0.zwxy * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g0);
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

Motor rotor_translator_regressive_product(Rotor self, Translator other) {
    return Motor(self.g0 * vec4(other.g0.w), vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

Motor rotor_translator_geometric_anti_product(Rotor self, Translator other) {
    return Motor(self.g0 * vec4(other.g0.w), vec4(self.g0.y) * other.g0.zzxy * vec4(1.0, 0.0, -1.0, -1.0) + vec4(self.g0.z) * other.g0.yxyz * vec4(-1.0, 1.0, 0.0, -1.0) + vec4(self.g0.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, -1.0));
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

Flector rotor_flector_regressive_product(Rotor self, Flector other) {
    return Flector(vec4(self.g0.y) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g0 + vec4(self.g0.x) * other.g1.wxxx * vec4(1.0, 0.0, 0.0, -1.0), vec4(self.g0.w) * other.g1);
}

Flector rotor_flector_geometric_anti_product(Rotor self, Flector other) {
    return Flector(vec4(self.g0.x) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g0.z, other.g1.w, other.g0.x, other.g1.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g1.w, other.g1.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g0, vec4(self.g0.x) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g1);
}

Flector rotor_flector_inner_anti_product(Rotor self, Flector other) {
    return Flector(vec4(self.g0.w) * other.g0, vec4(self.g0.x) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g1);
}

Flector rotor_flector_left_anti_contraction(Rotor self, Flector other) {
    return Flector(vec4(self.g0.w) * other.g0, vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g1 + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Dilation rotor_dilation_left_anti_contraction(Rotor self, Dilation other) {
    return Dilation(vec3(self.g0.w) * other.g0, vec2(self.g0.w) * other.g1);
}

Rotor rotor_dilation_right_anti_contraction(Rotor self, Dilation other) {
    return Rotor(self.g0 * vec4(other.g1.y));
}

AntiScalar rotor_dilation_anti_scalar_product(Rotor self, Dilation other) {
    return AntiScalar(self.g0.w * other.g1.y);
}

MultiVector rotor_multi_vector_add(Rotor self, MultiVector other) {
    return MultiVector(vec3(self.g0.x, self.g0.x, self.g0.w) * vec3(0.0, 0.0, 1.0) + other.g0, other.g1, other.g2, other.g3, other.g4, other.g5, vec3(self.g0.x, self.g0.y, self.g0.z) + other.g6, other.g7, other.g8, other.g9);
}

MultiVector rotor_multi_vector_sub(Rotor self, MultiVector other) {
    return MultiVector(vec3(self.g0.x, self.g0.x, self.g0.w) * vec3(0.0, 0.0, 1.0) - other.g0, vec3(0.0) - other.g1, vec2(0.0) - other.g2, vec4(0.0) - other.g3, vec3(0.0) - other.g4, vec3(0.0) - other.g5, vec3(self.g0.x, self.g0.y, self.g0.z) - other.g6, vec3(0.0) - other.g7, vec4(0.0) - other.g8, vec4(0.0) - other.g9);
}

MultiVector rotor_multi_vector_regressive_product(Rotor self, MultiVector other) {
    return MultiVector(vec3(self.g0.y) * vec3(other.g5.y) * vec3(-1.0, 0.0, 0.0) + vec3(self.g0.z) * vec3(other.g5.z) * vec3(-1.0, 0.0, 0.0) + vec3(self.g0.w) * other.g0 + vec3(self.g0.x) * vec3(other.g5.x) * vec3(-1.0, 0.0, 0.0), vec3(self.g0.w) * other.g1 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g8.w), vec2(0.0) - vec2(self.g0.x) * vec2(other.g8.x, other.g7.x) - vec2(self.g0.y) * vec2(other.g8.y, other.g7.y) - vec2(self.g0.z) * vec2(other.g8.z, other.g7.z) + vec2(self.g0.w) * other.g2, vec4(self.g0.y) * other.g9.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g9.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g3 + vec4(self.g0.x) * other.g9.wxxx * vec4(1.0, 0.0, 0.0, -1.0), vec3(self.g0.w) * other.g4 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), vec3(self.g0.w) * other.g5, vec3(self.g0.w) * other.g6 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.z), vec3(self.g0.w) * other.g7, vec4(self.g0.w) * other.g8, vec4(self.g0.w) * other.g9);
}

MultiVector rotor_multi_vector_geometric_anti_product(Rotor self, MultiVector other) {
    return MultiVector(vec3(0.0) - vec3(self.g0.x) * vec3(other.g5.x, other.g4.x, other.g6.x) - vec3(self.g0.y) * vec3(other.g5.y, other.g4.y, other.g6.y) - vec3(self.g0.z) * vec3(other.g5.z, other.g4.z, other.g6.z) + vec3(self.g0.w) * other.g0, vec3(self.g0.x) * vec3(other.g8.w, other.g1.z, other.g1.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g1.z, other.g8.w, other.g1.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g1.y, other.g1.x, other.g8.w) * vec3(-1.0, 1.0, 1.0) + vec3(self.g0.w) * other.g1, vec2(0.0) - vec2(self.g0.x) * vec2(other.g8.x, other.g7.x) - vec2(self.g0.y) * vec2(other.g8.y, other.g7.y) - vec2(self.g0.z) * vec2(other.g8.z, other.g7.z) + vec2(self.g0.w) * other.g2, vec4(self.g0.x) * vec4(other.g9.w, other.g3.z, other.g3.y, other.g9.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g3.z, other.g9.w, other.g3.x, other.g9.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g3.y, other.g3.x, other.g9.w, other.g9.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g3, vec3(self.g0.x) * vec3(other.g0.y, other.g4.z, other.g4.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g4.z, other.g0.y, other.g4.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g4.y, other.g4.x, other.g0.y) * vec3(-1.0, 1.0, 1.0) + vec3(self.g0.w) * other.g4, vec3(self.g0.x) * vec3(other.g0.x, other.g5.z, other.g5.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g5.z, other.g0.x, other.g5.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g5.y, other.g5.x, other.g0.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g0.w) * other.g5, vec3(self.g0.x) * vec3(other.g0.z, other.g6.z, other.g6.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g6.z, other.g0.z, other.g6.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g6.y, other.g6.x, other.g0.z) * vec3(-1.0, 1.0, 1.0) + vec3(self.g0.w) * other.g6, vec3(self.g0.x) * vec3(other.g2.y, other.g7.z, other.g7.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g7.z, other.g2.y, other.g7.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g7.y, other.g7.x, other.g2.y) * vec3(-1.0, 1.0, 1.0) + vec3(self.g0.w) * other.g7, vec4(self.g0.x) * vec4(other.g2.x, other.g8.z, other.g8.y, other.g1.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g8.z, other.g2.x, other.g8.x, other.g1.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g8.y, other.g8.x, other.g2.x, other.g1.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g8, vec4(self.g0.x) * vec4(other.g3.w, other.g9.z, other.g9.y, other.g3.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g9.z, other.g3.w, other.g9.x, other.g3.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g9.y, other.g9.x, other.g3.w, other.g3.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g9);
}

MultiVector rotor_multi_vector_inner_anti_product(Rotor self, MultiVector other) {
    return MultiVector(vec3(self.g0.y) * vec3(other.g4.y, other.g4.y, other.g6.y) * vec3(0.0, -1.0, -1.0) + vec3(self.g0.z) * vec3(other.g4.z, other.g4.z, other.g6.z) * vec3(0.0, -1.0, -1.0) + vec3(self.g0.w) * other.g0 + vec3(self.g0.x) * vec3(other.g4.x, other.g4.x, other.g6.x) * vec3(0.0, -1.0, -1.0), vec3(self.g0.w) * other.g1, vec2(self.g0.w) * other.g2, vec4(self.g0.w) * other.g3, vec3(self.g0.w) * other.g4, vec3(self.g0.w) * other.g5 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x), vec3(self.g0.w) * other.g6 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.z), vec3(self.g0.w) * other.g7 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g2.y), vec4(self.g0.y) * vec4(other.g2.x, other.g2.x, other.g2.x, other.g1.y) * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g2.x, other.g2.x, other.g2.x, other.g1.z) * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g8 + vec4(self.g0.x) * vec4(other.g2.x, other.g2.x, other.g2.x, other.g1.x) * vec4(1.0, 0.0, 0.0, -1.0), vec4(self.g0.x) * vec4(other.g3.w, other.g9.z, other.g9.y, other.g3.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g9.z, other.g3.w, other.g9.x, other.g3.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * vec4(other.g9.y, other.g9.x, other.g3.w, other.g3.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g9);
}

MultiVector rotor_multi_vector_left_anti_contraction(Rotor self, MultiVector other) {
    return MultiVector(vec3(self.g0.y) * vec3(other.g4.y, other.g4.y, other.g6.y) * vec3(0.0, -1.0, -1.0) + vec3(self.g0.z) * vec3(other.g4.z, other.g4.z, other.g6.z) * vec3(0.0, -1.0, -1.0) + vec3(self.g0.w) * other.g0 + vec3(self.g0.x) * vec3(other.g4.x, other.g4.x, other.g6.x) * vec3(0.0, -1.0, -1.0), vec3(self.g0.w) * other.g1, vec2(self.g0.w) * other.g2, vec4(self.g0.w) * other.g3, vec3(self.g0.w) * other.g4, vec3(self.g0.w) * other.g5 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x), vec3(self.g0.w) * other.g6, vec3(self.g0.w) * other.g7 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g2.y), vec4(self.g0.y) * vec4(other.g2.x, other.g2.x, other.g2.x, other.g1.y) * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g2.x, other.g2.x, other.g2.x, other.g1.z) * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g8 + vec4(self.g0.x) * vec4(other.g2.x, other.g2.x, other.g2.x, other.g1.x) * vec4(1.0, 0.0, 0.0, -1.0), vec4(self.g0.y) * other.g3.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g3.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g9 + vec4(self.g0.x) * other.g3.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

AntiScalar rotor_multi_vector_anti_scalar_product(Rotor self, MultiVector other) {
    return AntiScalar(0.0 - self.g0.x * other.g6.x - self.g0.y * other.g6.y - self.g0.z * other.g6.z + self.g0.w * other.g0.z);
}

Rotor rotor_scale(Rotor self, float other) {
    return rotor_scalar_geometric_product(self, Scalar(other));
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
    return Translator(self.g0 * vec4(-1.0));
}

Translator translator_reversal(Translator self) {
    return Translator(self.g0 * vec4(-1.0, -1.0, -1.0, 1.0));
}

Translator translator_conjugation(Translator self) {
    return Translator(self.g0 * vec4(1.0, 1.0, 1.0, -1.0));
}

Translator translator_anti_reversal(Translator self) {
    return Translator(self.g0 * vec4(1.0, 1.0, 1.0, -1.0));
}

Translator translator_double_complement(Translator self) {
    return Translator(self.g0);
}

Translator translator_scalar_geometric_product(Translator self, Scalar other) {
    return Translator(self.g0 * vec4(other.g0));
}

Scalar translator_scalar_regressive_product(Translator self, Scalar other) {
    return Scalar(self.g0.w * other.g0);
}

Translator translator_scalar_outer_product(Translator self, Scalar other) {
    return Translator(self.g0 * vec4(other.g0));
}

Translator translator_scalar_inner_product(Translator self, Scalar other) {
    return Translator(self.g0 * vec4(other.g0));
}

Scalar translator_scalar_geometric_anti_product(Translator self, Scalar other) {
    return Scalar(self.g0.w * other.g0);
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

Translator translator_anti_scalar_regressive_product(Translator self, AntiScalar other) {
    return Translator(self.g0 * vec4(other.g0));
}

Translator translator_anti_scalar_geometric_anti_product(Translator self, AntiScalar other) {
    return Translator(self.g0 * vec4(other.g0));
}

Translator translator_anti_scalar_inner_anti_product(Translator self, AntiScalar other) {
    return Translator(self.g0 * vec4(other.g0));
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

Translator translator_homogeneous_magnitude_geometric_product(Translator self, HomogeneousMagnitude other) {
    return Translator(self.g0 * vec4(other.g0.x));
}

Translator translator_homogeneous_magnitude_outer_product(Translator self, HomogeneousMagnitude other) {
    return Translator(self.g0 * vec4(other.g0.x));
}

Translator translator_homogeneous_magnitude_inner_product(Translator self, HomogeneousMagnitude other) {
    return Translator(self.g0 * vec4(other.g0.x));
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

RadialPoint translator_radial_point_regressive_product(Translator self, RadialPoint other) {
    return RadialPoint(vec3(self.g0.w) * other.g0, vec2(self.g0.w) * other.g1);
}

Plane translator_radial_point_outer_product(Translator self, RadialPoint other) {
    return Plane(vec4(self.g0.y) * vec4(other.g1.x, other.g1.x, other.g1.x, other.g0.y) * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g1.x, other.g1.x, other.g1.x, other.g0.z) * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.x, other.g1.x, other.g0.x) * vec4(-1.0, 0.0, 0.0, 1.0));
}

RadialPoint translator_radial_point_geometric_anti_product(Translator self, RadialPoint other) {
    return RadialPoint(vec3(self.g0.w) * other.g0 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g1.x), vec2(self.g0.w) * other.g1);
}

RadialPoint translator_radial_point_inner_anti_product(Translator self, RadialPoint other) {
    return RadialPoint(vec3(self.g0.w) * other.g0, vec2(self.g0.w) * other.g1);
}

RadialPoint translator_radial_point_left_anti_contraction(Translator self, RadialPoint other) {
    return RadialPoint(vec3(self.g0.w) * other.g0, vec2(self.g0.w) * other.g1);
}

FlatPoint translator_flat_point_regressive_product(Translator self, FlatPoint other) {
    return FlatPoint(vec4(self.g0.w) * other.g0);
}

FlatPoint translator_flat_point_geometric_anti_product(Translator self, FlatPoint other) {
    return FlatPoint(vec4(self.g0.w) * other.g0 + self.g0.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

FlatPoint translator_flat_point_inner_anti_product(Translator self, FlatPoint other) {
    return FlatPoint(vec4(self.g0.w) * other.g0);
}

FlatPoint translator_flat_point_left_anti_contraction(Translator self, FlatPoint other) {
    return FlatPoint(vec4(self.g0.w) * other.g0);
}

Motor translator_dipole_geometric_product(Translator self, Dipole other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0), vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

AntiScalar translator_dipole_outer_product(Translator self, Dipole other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
}

Dipole translator_dipole_inner_anti_product(Translator self, Dipole other) {
    return Dipole(vec3(self.g0.w) * other.g0, vec3(self.g0.w) * other.g1, vec4(self.g0.w) * other.g2);
}

Dipole translator_dipole_left_anti_contraction(Translator self, Dipole other) {
    return Dipole(vec3(self.g0.w) * other.g0, vec3(self.g0.w) * other.g1, vec4(self.g0.w) * other.g2);
}

Line translator_line_inner_anti_product(Translator self, Line other) {
    return Line(vec3(self.g0.w) * other.g0, vec3(self.g0.w) * other.g1);
}

Line translator_line_left_anti_contraction(Translator self, Line other) {
    return Line(vec3(self.g0.w) * other.g0, vec3(self.g0.w) * other.g1);
}

Circle translator_circle_inner_anti_product(Translator self, Circle other) {
    return Circle(vec4(self.g0.w) * other.g0, vec3(self.g0.w) * other.g1, vec3(self.g0.w) * other.g2);
}

Circle translator_circle_left_anti_contraction(Translator self, Circle other) {
    return Circle(vec4(self.g0.w) * other.g0, vec3(self.g0.w) * other.g1, vec3(self.g0.w) * other.g2);
}

Plane translator_plane_inner_anti_product(Translator self, Plane other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.w) * other.g0 + vec4(self.g0.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

Plane translator_plane_left_anti_contraction(Translator self, Plane other) {
    return Plane(vec4(self.g0.w) * other.g0);
}

Sphere translator_sphere_inner_anti_product(Translator self, Sphere other) {
    return Sphere(vec2(self.g0.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g0.w) * other.g0 + vec2(self.g0.x) * vec2(other.g1.x) * vec2(0.0, -1.0), vec3(self.g0.w) * other.g1);
}

Sphere translator_sphere_left_anti_contraction(Translator self, Sphere other) {
    return Sphere(vec2(self.g0.w) * other.g0, vec3(self.g0.w) * other.g1);
}

Motor translator_motor_add(Translator self, Motor other) {
    return Motor(self.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0) + other.g0, self.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + other.g1);
}

Motor translator_motor_sub(Translator self, Motor other) {
    return Motor(self.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0) - other.g0, self.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) - other.g1);
}

Motor translator_motor_regressive_product(Translator self, Motor other) {
    return Motor(vec4(self.g0.w) * other.g0, vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g1 + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Motor translator_motor_geometric_anti_product(Translator self, Motor other) {
    return Motor(vec4(self.g0.w) * other.g0, vec4(self.g0.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * other.g0.zwxy * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g1);
}

Motor translator_motor_inner_anti_product(Translator self, Motor other) {
    return Motor(vec4(self.g0.w) * other.g0, vec4(self.g0.w) * other.g1 + self.g0.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

Motor translator_motor_left_anti_contraction(Translator self, Motor other) {
    return Motor(vec4(self.g0.w) * other.g0, vec4(self.g0.w) * other.g1);
}

Translator translator_motor_right_anti_contraction(Translator self, Motor other) {
    return Translator(self.g0 * vec4(other.g0.w));
}

AntiScalar translator_motor_anti_scalar_product(Translator self, Motor other) {
    return AntiScalar(self.g0.w * other.g0.w);
}

Motor translator_rotor_regressive_product(Translator self, Rotor other) {
    return Motor(vec4(self.g0.w) * other.g0, vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

Motor translator_rotor_geometric_anti_product(Translator self, Rotor other) {
    return Motor(vec4(self.g0.w) * other.g0, vec4(self.g0.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * other.g0.zwxy * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.z) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0));
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

Translator translator_translator_regressive_product(Translator self, Translator other) {
    return Translator(vec4(self.g0.w) * other.g0 + self.g0.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

Translator translator_translator_geometric_anti_product(Translator self, Translator other) {
    return Translator(vec4(self.g0.w) * other.g0 + self.g0.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

Translator translator_translator_inner_anti_product(Translator self, Translator other) {
    return Translator(vec4(self.g0.w) * other.g0 + self.g0.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

Translator translator_translator_left_anti_contraction(Translator self, Translator other) {
    return Translator(vec4(self.g0.w) * other.g0);
}

Translator translator_translator_right_anti_contraction(Translator self, Translator other) {
    return Translator(self.g0 * vec4(other.g0.w));
}

AntiScalar translator_translator_anti_scalar_product(Translator self, Translator other) {
    return AntiScalar(self.g0.w * other.g0.w);
}

Flector translator_flector_regressive_product(Translator self, Flector other) {
    return Flector(vec4(self.g0.y) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g1.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.w) * other.g0 + vec4(self.g0.x) * other.g1.xzyx * vec4(0.0, -1.0, 1.0, 0.0), vec4(self.g0.w) * other.g1);
}

Flector translator_flector_geometric_anti_product(Translator self, Flector other) {
    return Flector(vec4(self.g0.y) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g1.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g1.y) * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0 + vec4(self.g0.x) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, -1.0, 1.0, 0.0), vec4(self.g0.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.w) * other.g1 + vec4(self.g0.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

Flector translator_flector_inner_anti_product(Translator self, Flector other) {
    return Flector(vec4(self.g0.w) * other.g0, vec4(self.g0.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.w) * other.g1 + vec4(self.g0.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

Flector translator_flector_left_anti_contraction(Translator self, Flector other) {
    return Flector(vec4(self.g0.w) * other.g0, vec4(self.g0.w) * other.g1);
}

Dilation translator_dilation_left_anti_contraction(Translator self, Dilation other) {
    return Dilation(vec3(self.g0.w) * other.g0, vec2(self.g0.w) * other.g1);
}

Translator translator_dilation_right_anti_contraction(Translator self, Dilation other) {
    return Translator(self.g0 * vec4(other.g1.y));
}

AntiScalar translator_dilation_anti_scalar_product(Translator self, Dilation other) {
    return AntiScalar(self.g0.w * other.g1.y);
}

MultiVector translator_multi_vector_add(Translator self, MultiVector other) {
    return MultiVector(vec3(self.g0.x, self.g0.x, self.g0.w) * vec3(0.0, 0.0, 1.0) + other.g0, other.g1, other.g2, other.g3, other.g4, other.g5, other.g6, vec3(self.g0.x, self.g0.y, self.g0.z) + other.g7, other.g8, other.g9);
}

MultiVector translator_multi_vector_sub(Translator self, MultiVector other) {
    return MultiVector(vec3(self.g0.x, self.g0.x, self.g0.w) * vec3(0.0, 0.0, 1.0) - other.g0, vec3(0.0) - other.g1, vec2(0.0) - other.g2, vec4(0.0) - other.g3, vec3(0.0) - other.g4, vec3(0.0) - other.g5, vec3(0.0) - other.g6, vec3(self.g0.x, self.g0.y, self.g0.z) - other.g7, vec4(0.0) - other.g8, vec4(0.0) - other.g9);
}

MultiVector translator_multi_vector_regressive_product(Translator self, MultiVector other) {
    return MultiVector(vec3(self.g0.y) * vec3(other.g4.y) * vec3(-1.0, 0.0, 0.0) + vec3(self.g0.z) * vec3(other.g4.z) * vec3(-1.0, 0.0, 0.0) + vec3(self.g0.w) * other.g0 + vec3(self.g0.x) * vec3(other.g4.x) * vec3(-1.0, 0.0, 0.0), vec3(self.g0.y) * vec3(other.g8.z, other.g8.z, other.g8.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g8.y, other.g8.x, other.g8.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.w) * other.g1 + vec3(self.g0.x) * vec3(other.g8.x, other.g8.z, other.g8.y) * vec3(0.0, -1.0, 1.0), vec2(self.g0.y) * vec2(other.g6.y) * vec2(0.0, -1.0) + vec2(self.g0.z) * vec2(other.g6.z) * vec2(0.0, -1.0) + vec2(self.g0.w) * other.g2 + vec2(self.g0.x) * vec2(other.g6.x) * vec2(0.0, -1.0), vec4(self.g0.y) * other.g9.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g9.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.w) * other.g3 + vec4(self.g0.x) * other.g9.xzyx * vec4(0.0, -1.0, 1.0, 0.0), vec3(self.g0.w) * other.g4, vec3(self.g0.w) * other.g5 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), vec3(self.g0.w) * other.g6, vec3(self.g0.w) * other.g7 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.z), vec4(self.g0.w) * other.g8, vec4(self.g0.w) * other.g9);
}

MultiVector translator_multi_vector_geometric_anti_product(Translator self, MultiVector other) {
    return MultiVector(vec3(self.g0.y) * vec3(other.g4.y) * vec3(-1.0, 0.0, 0.0) + vec3(self.g0.z) * vec3(other.g4.z) * vec3(-1.0, 0.0, 0.0) + vec3(self.g0.w) * other.g0 + vec3(self.g0.x) * vec3(other.g4.x) * vec3(-1.0, 0.0, 0.0), vec3(self.g0.x) * vec3(other.g2.x, other.g8.z, other.g8.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g8.z, other.g2.x, other.g8.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g8.y, other.g8.x, other.g2.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g0.w) * other.g1, vec2(self.g0.y) * vec2(other.g6.y) * vec2(0.0, -1.0) + vec2(self.g0.z) * vec2(other.g6.z) * vec2(0.0, -1.0) + vec2(self.g0.w) * other.g2 + vec2(self.g0.x) * vec2(other.g6.x) * vec2(0.0, -1.0), vec4(self.g0.y) * vec4(other.g9.z, other.g3.w, other.g9.x, other.g9.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g0.z) * vec4(other.g9.y, other.g9.x, other.g3.w, other.g9.y) * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g0.w) * other.g3 + vec4(self.g0.x) * vec4(other.g3.w, other.g9.z, other.g9.y, other.g3.x) * vec4(1.0, -1.0, 1.0, 0.0), vec3(self.g0.w) * other.g4, vec3(self.g0.x) * vec3(other.g0.y, other.g4.z, other.g4.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g4.z, other.g0.y, other.g4.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g4.y, other.g4.x, other.g0.y) * vec3(-1.0, 1.0, 1.0) + vec3(self.g0.w) * other.g5, vec3(self.g0.w) * other.g6, vec3(self.g0.x) * vec3(other.g0.z, other.g6.z, other.g6.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g6.z, other.g0.z, other.g6.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g6.y, other.g6.x, other.g0.z) * vec3(-1.0, 1.0, 1.0) + vec3(self.g0.w) * other.g7, vec4(self.g0.y) * vec4(other.g8.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g8.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.w) * other.g8 + vec4(self.g0.x) * vec4(other.g8.x) * vec4(0.0, 0.0, 0.0, -1.0), vec4(self.g0.y) * vec4(other.g9.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g9.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.w) * other.g9 + vec4(self.g0.x) * vec4(other.g9.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

MultiVector translator_multi_vector_inner_anti_product(Translator self, MultiVector other) {
    return MultiVector(vec3(self.g0.w) * other.g0, vec3(self.g0.w) * other.g1, vec2(self.g0.w) * other.g2, vec4(self.g0.w) * other.g3, vec3(self.g0.w) * other.g4, vec3(self.g0.w) * other.g5, vec3(self.g0.w) * other.g6, vec3(self.g0.w) * other.g7 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.z), vec4(self.g0.w) * other.g8, vec4(self.g0.y) * vec4(other.g9.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g9.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.w) * other.g9 + vec4(self.g0.x) * vec4(other.g9.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

MultiVector translator_multi_vector_left_anti_contraction(Translator self, MultiVector other) {
    return MultiVector(vec3(self.g0.w) * other.g0, vec3(self.g0.w) * other.g1, vec2(self.g0.w) * other.g2, vec4(self.g0.w) * other.g3, vec3(self.g0.w) * other.g4, vec3(self.g0.w) * other.g5, vec3(self.g0.w) * other.g6, vec3(self.g0.w) * other.g7, vec4(self.g0.w) * other.g8, vec4(self.g0.w) * other.g9);
}

AntiScalar translator_multi_vector_anti_scalar_product(Translator self, MultiVector other) {
    return AntiScalar(self.g0.w * other.g0.z);
}

Translator translator_scale(Translator self, float other) {
    return translator_scalar_geometric_product(self, Scalar(other));
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
    return Flector(self.g0, self.g1);
}

Flector flector_reversal(Flector self) {
    return Flector(self.g0 * vec4(-1.0), self.g1);
}

Flector flector_conjugation(Flector self) {
    return Flector(self.g0 * vec4(-1.0), self.g1);
}

Flector flector_anti_reversal(Flector self) {
    return Flector(self.g0 * vec4(-1.0), self.g1);
}

Flector flector_double_complement(Flector self) {
    return Flector(self.g0, self.g1);
}

Flector flector_scalar_geometric_product(Flector self, Scalar other) {
    return Flector(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

Flector flector_scalar_outer_product(Flector self, Scalar other) {
    return Flector(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

Flector flector_scalar_inner_product(Flector self, Scalar other) {
    return Flector(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

Flector flector_scalar_right_contraction(Flector self, Scalar other) {
    return Flector(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

Flector flector_anti_scalar_regressive_product(Flector self, AntiScalar other) {
    return Flector(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

Flector flector_anti_scalar_geometric_anti_product(Flector self, AntiScalar other) {
    return Flector(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

Flector flector_anti_scalar_inner_anti_product(Flector self, AntiScalar other) {
    return Flector(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

Flector flector_anti_scalar_right_anti_contraction(Flector self, AntiScalar other) {
    return Flector(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

Flector flector_homogeneous_magnitude_geometric_product(Flector self, HomogeneousMagnitude other) {
    return Flector(self.g0 * vec4(other.g0.x), self.g1 * vec4(other.g0.x));
}

Flector flector_homogeneous_magnitude_regressive_product(Flector self, HomogeneousMagnitude other) {
    return Flector(self.g0 * vec4(other.g0.y), self.g1 * vec4(other.g0.y));
}

Flector flector_homogeneous_magnitude_outer_product(Flector self, HomogeneousMagnitude other) {
    return Flector(self.g0 * vec4(other.g0.x), self.g1 * vec4(other.g0.x));
}

Flector flector_homogeneous_magnitude_inner_product(Flector self, HomogeneousMagnitude other) {
    return Flector(self.g0 * vec4(other.g0.x), self.g1 * vec4(other.g0.x));
}

Flector flector_homogeneous_magnitude_right_contraction(Flector self, HomogeneousMagnitude other) {
    return Flector(self.g0 * vec4(other.g0.x), self.g1 * vec4(other.g0.x));
}

Flector flector_homogeneous_magnitude_right_anti_contraction(Flector self, HomogeneousMagnitude other) {
    return Flector(self.g0 * vec4(other.g0.y), self.g1 * vec4(other.g0.y));
}

Motor flector_radial_point_geometric_product(Flector self, RadialPoint other) {
    return Motor(vec4(self.g0.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(-1.0, -1.0, -1.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.z, other.g0.z, other.g0.y, other.g0.x) * vec4(0.0, -1.0, 1.0, 1.0) + vec4(self.g1.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g0.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.z) * vec4(-1.0, 1.0, 0.0, 1.0) + vec4(self.g0.x, self.g0.y, self.g0.z, self.g1.w) * vec4(other.g1.x), vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

Scalar flector_radial_point_regressive_product(Flector self, RadialPoint other) {
    return Scalar(self.g1.x * other.g0.x + self.g1.y * other.g0.y + self.g1.z * other.g0.z + self.g1.w * other.g1.x);
}

FlatPoint flector_flat_point_into(Flector self) {
    return FlatPoint(self.g0);
}

Flector flector_flat_point_add(Flector self, FlatPoint other) {
    return Flector(self.g0 + other.g0, self.g1);
}

Flector flector_flat_point_sub(Flector self, FlatPoint other) {
    return Flector(self.g0 - other.g0, self.g1);
}

Motor flector_flat_point_geometric_anti_product(Flector self, FlatPoint other) {
    return Motor(vec4(self.g1.x, self.g1.y, self.g1.z, self.g0.w) * vec4(other.g0.w) * vec4(-1.0), vec4(self.g0.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x) * other.g0.zzyx * vec4(0.0, -1.0, 1.0, -1.0) + vec4(self.g1.y) * other.g0.zzxy * vec4(1.0, 0.0, -1.0, -1.0) + vec4(self.g1.z) * other.g0.yxyz * vec4(-1.0, 1.0, 0.0, -1.0) + vec4(self.g0.x, self.g0.y, self.g0.z, self.g1.w) * vec4(other.g0.w) * vec4(-1.0));
}

AntiScalar flector_flat_point_right_anti_contraction(Flector self, FlatPoint other) {
    return AntiScalar(0.0 - self.g0.w * other.g0.w);
}

AntiScalar flector_flat_point_anti_scalar_product(Flector self, FlatPoint other) {
    return AntiScalar(0.0 - self.g0.w * other.g0.w);
}

Flector flector_dipole_geometric_product(Flector self, Dipole other) {
    return Flector(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0), vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g1.z, other.g1.z, other.g1.y, other.g1.z) * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g1.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.z) * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.y) * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

RadialPoint flector_dipole_regressive_product(Flector self, Dipole other) {
    return RadialPoint(vec3(self.g1.y) * other.g1.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * other.g1.yxy * vec3(-1.0, 1.0, 0.0) - vec3(self.g1.w) * other.g0 + vec3(self.g1.x) * other.g1.xzy * vec3(0.0, -1.0, 1.0), vec2(self.g1.x) * vec2(other.g0.x, other.g2.x) * vec2(1.0, -1.0) + vec2(self.g1.y) * vec2(other.g0.y, other.g2.y) * vec2(1.0, -1.0) + vec2(self.g1.z) * vec2(other.g0.z, other.g2.z) * vec2(1.0, -1.0) + vec2(self.g1.x, self.g1.w) * vec2(other.g2.x, other.g2.w) * vec2(0.0, -1.0));
}

Plane flector_dipole_outer_product(Flector self, Dipole other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

FlatPoint flector_dipole_inner_product(Flector self, Dipole other) {
    return FlatPoint(vec4(self.g1.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g1.wwwx * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, -1.0));
}

FlatPoint flector_dipole_right_contraction(Flector self, Dipole other) {
    return FlatPoint(vec4(self.g1.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g1.wwwx * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, -1.0));
}

AntiScalar flector_dipole_right_anti_contraction(Flector self, Dipole other) {
    return AntiScalar(0.0 - self.g0.w * other.g2.w);
}

AntiScalar flector_dipole_anti_scalar_product(Flector self, Dipole other) {
    return AntiScalar(0.0 - self.g0.w * other.g2.w);
}

FlatPoint flector_line_regressive_product(Flector self, Line other) {
    return FlatPoint(vec4(self.g1.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

Flector flector_line_geometric_anti_product(Flector self, Line other) {
    return Flector(vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g0.z) * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.y) * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(-1.0, -1.0, -1.0, 0.0) + vec4(self.g1.x) * vec4(other.g1.z, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g0.x) * vec4(0.0, -1.0, 1.0, 0.0), vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g0.z, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, -1.0, 1.0, 1.0) + vec4(self.g1.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(-1.0, 1.0, 0.0, 1.0) + self.g0.wwwx * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, -1.0));
}

Plane flector_line_inner_anti_product(Flector self, Line other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g0.z, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, -1.0, 1.0, 1.0) + vec4(self.g1.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(-1.0, 1.0, 0.0, 1.0) + self.g0.wwwx * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, -1.0));
}

Plane flector_line_left_anti_contraction(Flector self, Line other) {
    return Plane(vec4(self.g1.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(-1.0, 1.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, -1.0, 1.0, 1.0));
}

Plane flector_line_right_anti_contraction(Flector self, Line other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, -1.0));
}

Motor flector_circle_geometric_product(Flector self, Circle other) {
    return Motor(vec4(self.g0.y) * other.g0.zzxy * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.yxyz * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.w) * vec4(0.0, -1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.w) * vec4(0.0, 0.0, -1.0, 0.0) + vec4(self.g1.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x, self.g0.x, self.g0.x, self.g0.x) * other.g0.wzyx * vec4(-1.0, 1.0, -1.0, -1.0), vec4(self.g0.x, self.g0.y, self.g0.z, self.g1.w) * vec4(other.g0.w));
}

AntiScalar flector_circle_outer_product(Flector self, Circle other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

Sphere flector_circle_inner_anti_product(Flector self, Circle other) {
    return Sphere(vec2(self.g0.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g1.x) * vec2(other.g0.x, other.g2.x) * vec2(-1.0, 1.0) + vec2(self.g1.y) * vec2(other.g0.y, other.g2.y) * vec2(-1.0, 1.0) + vec2(self.g1.z) * vec2(other.g0.z, other.g2.z) * vec2(-1.0, 1.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(0.0, -1.0), vec3(self.g0.w) * other.g1 + vec3(self.g1.y) * other.g1.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * other.g1.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.x) * other.g1.xzy * vec3(0.0, -1.0, 1.0));
}

Sphere flector_circle_left_anti_contraction(Flector self, Circle other) {
    return Sphere(vec2(self.g1.x) * vec2(other.g0.x, other.g2.x) * vec2(-1.0, 1.0) + vec2(self.g1.y) * vec2(other.g0.y, other.g2.y) * vec2(-1.0, 1.0) + vec2(self.g1.z) * vec2(other.g0.z, other.g2.z) * vec2(-1.0, 1.0), vec3(self.g1.y) * other.g1.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * other.g1.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.x) * other.g1.xzy * vec3(0.0, -1.0, 1.0));
}

Plane flector_circle_right_anti_contraction(Flector self, Circle other) {
    return Plane(vec4(self.g0.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, -1.0));
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

Motor flector_plane_geometric_anti_product(Flector self, Plane other) {
    return Motor(vec4(self.g1.x) * other.g0.zzyx * vec4(0.0, 1.0, -1.0, 1.0) + vec4(self.g1.y) * other.g0.zzxy * vec4(-1.0, 0.0, 1.0, 1.0) + vec4(self.g1.z) * other.g0.yxyz * vec4(1.0, -1.0, 0.0, 1.0) + self.g0.wwwx * other.g0.xyzx * vec4(-1.0, -1.0, -1.0, 0.0), vec4(self.g0.y) * other.g0.zzxy * vec4(-1.0, 0.0, 1.0, 1.0) + vec4(self.g0.z) * other.g0.yxyz * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.y) * vec4(other.g0.w) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.w) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * other.g0.xyzx * vec4(-1.0, -1.0, -1.0, 0.0) + vec4(self.g1.x, self.g0.x, self.g0.x, self.g0.x) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0));
}

AntiScalar flector_plane_left_anti_contraction(Flector self, Plane other) {
    return AntiScalar(self.g1.x * other.g0.x + self.g1.y * other.g0.y + self.g1.z * other.g0.z);
}

AntiScalar flector_plane_anti_scalar_product(Flector self, Plane other) {
    return AntiScalar(self.g1.x * other.g0.x + self.g1.y * other.g0.y + self.g1.z * other.g0.z);
}

AntiScalar flector_sphere_left_anti_contraction(Flector self, Sphere other) {
    return AntiScalar(self.g1.x * other.g1.x + self.g1.y * other.g1.y + self.g1.z * other.g1.z);
}

AntiScalar flector_sphere_anti_scalar_product(Flector self, Sphere other) {
    return AntiScalar(self.g1.x * other.g1.x + self.g1.y * other.g1.y + self.g1.z * other.g1.z);
}

Flector flector_motor_regressive_product(Flector self, Motor other) {
    return Flector(vec4(self.g1.x) * vec4(other.g1.z, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + self.g0 * vec4(other.g0.w), self.g1 * vec4(other.g0.w));
}

Flector flector_motor_geometric_anti_product(Flector self, Motor other) {
    return Flector(vec4(self.g0.y) * other.g0.zwxz * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxwy * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g0.w) * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g1.x) * vec4(other.g1.w, other.g1.z, other.g1.y, other.g0.x) * vec4(-1.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g1.z, other.g1.w, other.g1.x, other.g0.y) * vec4(-1.0, -1.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g1.w, other.g0.z) * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g1.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, 0.0), vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.w) * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g1.x) * vec4(other.g0.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g1.y) * vec4(other.g0.z, other.g0.w, other.g0.x, other.g1.y) * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g0.w, other.g1.z) * vec4(-1.0, 1.0, 1.0, 1.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

Flector flector_motor_inner_anti_product(Flector self, Motor other) {
    return Flector(vec4(self.g1.x) * vec4(other.g1.w) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g1.w) * vec4(0.0, -1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g1.w) * vec4(0.0, 0.0, -1.0, 0.0) + self.g0 * vec4(other.g0.w), vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.w) * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g1.x) * vec4(other.g0.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g1.y) * vec4(other.g0.z, other.g0.w, other.g0.x, other.g1.y) * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g0.w, other.g1.z) * vec4(-1.0, 1.0, 1.0, 1.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

Flector flector_motor_right_anti_contraction(Flector self, Motor other) {
    return Flector(self.g0 * vec4(other.g0.w), vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.w) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.w) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + self.g0.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

Flector flector_rotor_regressive_product(Flector self, Rotor other) {
    return Flector(vec4(self.g1.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + self.g0 * vec4(other.g0.w), self.g1 * vec4(other.g0.w));
}

Flector flector_rotor_geometric_anti_product(Flector self, Rotor other) {
    return Flector(vec4(self.g0.y) * other.g0.zwxz * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g0.z) * other.g0.yxwy * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + self.g0.xxxw * other.g0.wzyw * vec4(1.0, -1.0, 1.0, 1.0), vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.x) * other.g0.wzyw * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g1.y) * other.g0.zwxz * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g1.z) * other.g0.yxwy * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + self.g0.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

Flector flector_rotor_inner_anti_product(Flector self, Rotor other) {
    return Flector(self.g0 * vec4(other.g0.w), vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.x) * other.g0.wzyw * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g1.y) * other.g0.zwxz * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g1.z) * other.g0.yxwy * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + self.g0.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

Flector flector_rotor_right_anti_contraction(Flector self, Rotor other) {
    return Flector(self.g0 * vec4(other.g0.w), vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.w) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.w) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + self.g0.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

Flector flector_translator_regressive_product(Flector self, Translator other) {
    return Flector(vec4(self.g1.x) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + self.g0 * vec4(other.g0.w), self.g1 * vec4(other.g0.w));
}

Flector flector_translator_geometric_anti_product(Flector self, Translator other) {
    return Flector(vec4(self.g0.w) * other.g0 * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g1.x) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + self.g0.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0), vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, 1.0) + vec4(self.g1.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, 1.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, 1.0));
}

Flector flector_translator_inner_anti_product(Flector self, Translator other) {
    return Flector(self.g0 * vec4(other.g0.w), vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, 1.0) + vec4(self.g1.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, 1.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, 1.0));
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

Motor flector_flector_geometric_anti_product(Flector self, Flector other) {
    return Motor(vec4(0.0) - vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g0.w) + vec4(self.g1.x) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g1.x) * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g1.y) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g1.y) * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g1.z) * vec4(1.0, -1.0, -1.0, 1.0), vec4(self.g0.x) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g1.x) * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g0.y) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g1.y) * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g1.z) * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.w) + vec4(self.g1.x) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g0.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.z, other.g1.w, other.g0.x, other.g0.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g1.w, other.g0.z) * vec4(-1.0, 1.0, 1.0, -1.0) - vec4(self.g1.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g0.w));
}

AntiScalar flector_flector_anti_scalar_product(Flector self, Flector other) {
    return AntiScalar(0.0 - self.g0.w * other.g0.w + self.g1.x * other.g1.x + self.g1.y * other.g1.y + self.g1.z * other.g1.z);
}

Motor flector_dilation_geometric_product(Flector self, Dilation other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.y, other.g0.y, other.g1.x, other.g0.x) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.x, other.g1.x, other.g0.x, other.g0.y) * vec4(-1.0, 1.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.y) * vec4(other.g0.z) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g1.x, other.g0.x, other.g0.y, other.g1.x) * vec4(-1.0, -1.0, -1.0, 0.0) + vec4(self.g1.x, self.g0.x, self.g0.x, self.g0.x) * vec4(other.g0.z, other.g0.y, other.g0.x, other.g1.x) * vec4(1.0, -1.0, 1.0, 1.0), vec4(self.g0.x, self.g0.y, self.g0.z, self.g1.w) * vec4(other.g0.z) * vec4(-1.0));
}

AntiScalar flector_dilation_outer_product(Flector self, Dilation other) {
    return AntiScalar(self.g0.x * other.g1.x + self.g0.y * other.g0.x + self.g0.z * other.g0.y + self.g0.w * other.g0.z);
}

Flector flector_dilation_right_anti_contraction(Flector self, Dilation other) {
    return Flector(self.g0 * vec4(other.g1.y), self.g1 * vec4(other.g1.y));
}

MultiVector flector_multi_vector_add(Flector self, MultiVector other) {
    return MultiVector(other.g0, other.g1, other.g2, self.g0 + other.g3, other.g4, other.g5, other.g6, other.g7, other.g8, self.g1 + other.g9);
}

MultiVector flector_multi_vector_sub(Flector self, MultiVector other) {
    return MultiVector(vec3(0.0) - other.g0, vec3(0.0) - other.g1, vec2(0.0) - other.g2, self.g0 - other.g3, vec3(0.0) - other.g4, vec3(0.0) - other.g5, vec3(0.0) - other.g6, vec3(0.0) - other.g7, vec4(0.0) - other.g8, self.g1 - other.g9);
}

MultiVector flector_multi_vector_geometric_anti_product(Flector self, MultiVector other) {
    return MultiVector(vec3(self.g0.y) * vec3(other.g8.y) * vec3(-1.0, 0.0, 0.0) + vec3(self.g0.z) * vec3(other.g8.z) * vec3(-1.0, 0.0, 0.0) + vec3(self.g0.w) * vec3(other.g8.w, other.g2.x, other.g3.w) * vec3(-1.0, 1.0, -1.0) + vec3(self.g1.x) * vec3(other.g1.x, other.g8.x, other.g9.x) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g1.y, other.g8.y, other.g9.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.z) * vec3(other.g1.z, other.g8.z, other.g9.z) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.w) * vec3(other.g2.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g0.x) * vec3(other.g8.x) * vec3(-1.0, 0.0, 0.0), vec3(self.g0.x) * vec3(other.g0.y, other.g4.z, other.g4.y) * vec3(-1.0, 1.0, -1.0) + vec3(self.g0.y) * vec3(other.g4.z, other.g0.y, other.g4.x) * vec3(-1.0, -1.0, 1.0) + vec3(self.g0.z) * vec3(other.g4.y, other.g4.x, other.g0.y) * vec3(1.0, -1.0, -1.0) + vec3(self.g0.w) * other.g5 + vec3(self.g1.x) * vec3(other.g0.x, other.g5.z, other.g5.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g5.z, other.g0.x, other.g5.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.z) * vec3(other.g5.y, other.g5.x, other.g0.x) * vec3(-1.0, 1.0, 1.0) - vec3(self.g1.w) * other.g4, vec2(self.g0.y) * vec2(other.g9.y) * vec2(0.0, 1.0) + vec2(self.g0.z) * vec2(other.g9.z) * vec2(0.0, 1.0) + vec2(self.g0.w) * vec2(other.g0.y, other.g9.w) * vec2(-1.0, 1.0) + vec2(self.g1.x) * vec2(other.g4.x, other.g3.x) * vec2(1.0, -1.0) + vec2(self.g1.y) * vec2(other.g4.y, other.g3.y) * vec2(1.0, -1.0) + vec2(self.g1.z) * vec2(other.g4.z, other.g3.z) * vec2(1.0, -1.0) + vec2(self.g1.w) * vec2(other.g3.w) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g9.x) * vec2(0.0, 1.0), vec4(self.g0.y) * vec4(other.g6.z, other.g0.z, other.g6.x, other.g6.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g0.z) * vec4(other.g6.y, other.g6.x, other.g0.z, other.g6.y) * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g0.w) * vec4(other.g7.x, other.g7.y, other.g7.z, other.g0.z) * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g1.x) * vec4(other.g2.y, other.g7.z, other.g7.y, other.g6.x) * vec4(-1.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g7.z, other.g2.y, other.g7.x, other.g6.y) * vec4(-1.0, -1.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g7.y, other.g7.x, other.g2.y, other.g6.z) * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g1.w) * vec4(other.g6.x, other.g6.y, other.g6.z, other.g6.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.z, other.g6.z, other.g6.y, other.g0.x) * vec4(1.0, -1.0, 1.0, 0.0), vec3(self.g0.w) * vec3(other.g8.x, other.g8.y, other.g8.z) + vec3(self.g1.x) * vec3(other.g2.x, other.g8.z, other.g8.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g8.z, other.g2.x, other.g8.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.z) * vec3(other.g8.y, other.g8.x, other.g2.x) * vec3(-1.0, 1.0, 1.0), vec3(self.g0.x) * vec3(other.g2.x, other.g8.z, other.g8.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g8.z, other.g2.x, other.g8.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g8.y, other.g8.x, other.g2.x) * vec3(-1.0, 1.0, 1.0) - vec3(self.g0.w) * other.g1 + vec3(self.g1.x) * vec3(other.g8.w, other.g1.z, other.g1.y) * vec3(-1.0, 1.0, -1.0) + vec3(self.g1.y) * vec3(other.g1.z, other.g8.w, other.g1.x) * vec3(-1.0, -1.0, 1.0) + vec3(self.g1.z) * vec3(other.g1.y, other.g1.x, other.g8.w) * vec3(1.0, -1.0, -1.0) + vec3(self.g1.w) * vec3(other.g8.x, other.g8.y, other.g8.z), vec3(0.0) - vec3(self.g0.w) * vec3(other.g9.x, other.g9.y, other.g9.z) + vec3(self.g1.x) * vec3(other.g3.w, other.g9.z, other.g9.y) * vec3(-1.0, 1.0, -1.0) + vec3(self.g1.y) * vec3(other.g9.z, other.g3.w, other.g9.x) * vec3(-1.0, -1.0, 1.0) + vec3(self.g1.z) * vec3(other.g9.y, other.g9.x, other.g3.w) * vec3(1.0, -1.0, -1.0), vec3(self.g0.x) * vec3(other.g3.w, other.g9.z, other.g9.y) * vec3(-1.0, 1.0, -1.0) + vec3(self.g0.y) * vec3(other.g9.z, other.g3.w, other.g9.x) * vec3(-1.0, -1.0, 1.0) + vec3(self.g0.z) * vec3(other.g9.y, other.g9.x, other.g3.w) * vec3(1.0, -1.0, -1.0) + vec3(self.g0.w) * vec3(other.g3.x, other.g3.y, other.g3.z) + vec3(self.g1.x) * vec3(other.g9.w, other.g3.z, other.g3.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g3.z, other.g9.w, other.g3.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.z) * vec3(other.g3.y, other.g3.x, other.g9.w) * vec3(-1.0, 1.0, 1.0) - vec3(self.g1.w) * vec3(other.g9.x, other.g9.y, other.g9.z), vec4(self.g0.y) * vec4(other.g4.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g4.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g4.x, other.g4.y, other.g4.z, other.g0.x) * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g1.x) * vec4(other.g0.y, other.g4.z, other.g4.y, other.g5.x) * vec4(-1.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g4.z, other.g0.y, other.g4.x, other.g5.y) * vec4(-1.0, -1.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g4.y, other.g4.x, other.g0.y, other.g5.z) * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g1.w) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x) * vec4(other.g4.x) * vec4(0.0, 0.0, 0.0, 1.0), vec4(self.g0.y) * vec4(other.g6.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g6.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g6.x, other.g6.y, other.g6.z, other.g2.y) * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g1.x) * vec4(other.g0.z, other.g6.z, other.g6.y, other.g7.x) * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g1.y) * vec4(other.g6.z, other.g0.z, other.g6.x, other.g7.y) * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g1.z) * vec4(other.g6.y, other.g6.x, other.g0.z, other.g7.z) * vec4(-1.0, 1.0, 1.0, 1.0) + vec4(self.g1.w) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g6.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

AntiScalar flector_multi_vector_anti_scalar_product(Flector self, MultiVector other) {
    return AntiScalar(0.0 - self.g0.w * other.g3.w + self.g1.x * other.g9.x + self.g1.y * other.g9.y + self.g1.z * other.g9.z);
}

Flector flector_scale(Flector self, float other) {
    return flector_scalar_geometric_product(self, Scalar(other));
}

Dilation dilation_zero() {
    return Dilation(vec3(0.0), vec2(0.0));
}

Dilation dilation_one() {
    return Dilation(vec3(0.0), vec2(0.0));
}

Dilation dilation_neg(Dilation self) {
    return Dilation(self.g0 * vec3(-1.0), self.g1 * vec2(-1.0));
}

Dilation dilation_automorphism(Dilation self) {
    return Dilation(self.g0 * vec3(-1.0), self.g1 * vec2(-1.0));
}

Dilation dilation_reversal(Dilation self) {
    return Dilation(self.g0 * vec3(-1.0), self.g1 * vec2(-1.0, 1.0));
}

Dilation dilation_conjugation(Dilation self) {
    return Dilation(self.g0, self.g1 * vec2(1.0, -1.0));
}

Dilation dilation_anti_reversal(Dilation self) {
    return Dilation(self.g0, self.g1 * vec2(1.0, -1.0));
}

Dilation dilation_double_complement(Dilation self) {
    return Dilation(self.g0, self.g1);
}

Dilation dilation_scalar_geometric_product(Dilation self, Scalar other) {
    return Dilation(self.g0 * vec3(other.g0), self.g1 * vec2(other.g0));
}

Scalar dilation_scalar_regressive_product(Dilation self, Scalar other) {
    return Scalar(self.g1.y * other.g0);
}

Dilation dilation_scalar_outer_product(Dilation self, Scalar other) {
    return Dilation(self.g0 * vec3(other.g0), self.g1 * vec2(other.g0));
}

Dilation dilation_scalar_inner_product(Dilation self, Scalar other) {
    return Dilation(self.g0 * vec3(other.g0), self.g1 * vec2(other.g0));
}

Scalar dilation_scalar_geometric_anti_product(Dilation self, Scalar other) {
    return Scalar(self.g1.y * other.g0);
}

Scalar dilation_scalar_inner_anti_product(Dilation self, Scalar other) {
    return Scalar(self.g1.y * other.g0);
}

Dilation dilation_scalar_right_contraction(Dilation self, Scalar other) {
    return Dilation(self.g0 * vec3(other.g0), self.g1 * vec2(other.g0));
}

Scalar dilation_scalar_left_anti_contraction(Dilation self, Scalar other) {
    return Scalar(self.g1.y * other.g0);
}

AntiScalar dilation_anti_scalar_into(Dilation self) {
    return AntiScalar(self.g1.y);
}

Dilation dilation_anti_scalar_add(Dilation self, AntiScalar other) {
    return Dilation(self.g0, self.g1 + vec2(other.g0) * vec2(0.0, 1.0));
}

Dilation dilation_anti_scalar_sub(Dilation self, AntiScalar other) {
    return Dilation(self.g0, self.g1 - vec2(other.g0) * vec2(0.0, 1.0));
}

Dilation dilation_anti_scalar_regressive_product(Dilation self, AntiScalar other) {
    return Dilation(self.g0 * vec3(other.g0), self.g1 * vec2(other.g0));
}

Dilation dilation_anti_scalar_geometric_anti_product(Dilation self, AntiScalar other) {
    return Dilation(self.g0 * vec3(other.g0), self.g1 * vec2(other.g0));
}

Dilation dilation_anti_scalar_inner_anti_product(Dilation self, AntiScalar other) {
    return Dilation(self.g0 * vec3(other.g0), self.g1 * vec2(other.g0));
}

AntiScalar dilation_anti_scalar_left_anti_contraction(Dilation self, AntiScalar other) {
    return AntiScalar(self.g1.y * other.g0);
}

Dilation dilation_anti_scalar_right_anti_contraction(Dilation self, AntiScalar other) {
    return Dilation(self.g0 * vec3(other.g0), self.g1 * vec2(other.g0));
}

AntiScalar dilation_anti_scalar_anti_scalar_product(Dilation self, AntiScalar other) {
    return AntiScalar(self.g1.y * other.g0);
}

Dilation dilation_homogeneous_magnitude_outer_product(Dilation self, HomogeneousMagnitude other) {
    return Dilation(self.g0 * vec3(other.g0.x), self.g1 * vec2(other.g0.x));
}

Dilation dilation_homogeneous_magnitude_right_contraction(Dilation self, HomogeneousMagnitude other) {
    return Dilation(self.g0 * vec3(other.g0.x), self.g1 * vec2(other.g0.x));
}

HomogeneousMagnitude dilation_homogeneous_magnitude_left_anti_contraction(Dilation self, HomogeneousMagnitude other) {
    return HomogeneousMagnitude(vec2(self.g1.y) * other.g0);
}

Dilation dilation_homogeneous_magnitude_right_anti_contraction(Dilation self, HomogeneousMagnitude other) {
    return Dilation(self.g0 * vec3(other.g0.y), self.g1 * vec2(other.g0.y));
}

AntiScalar dilation_homogeneous_magnitude_anti_scalar_product(Dilation self, HomogeneousMagnitude other) {
    return AntiScalar(self.g1.y * other.g0.y);
}

RadialPoint dilation_radial_point_regressive_product(Dilation self, RadialPoint other) {
    return RadialPoint(vec3(self.g1.y) * other.g0, vec2(self.g1.y) * other.g1);
}

Sphere dilation_radial_point_outer_product(Dilation self, RadialPoint other) {
    return Sphere(vec2(self.g0.y) * vec2(other.g0.z) * vec2(1.0, 0.0) + vec2(self.g0.z) * other.g1 * vec2(1.0, -1.0) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(1.0, 0.0) + vec2(self.g0.x) * vec2(other.g0.y, other.g0.x) * vec2(1.0, 0.0), vec3(self.g1.x) * vec3(other.g1.y) * vec3(-1.0, 0.0, 0.0) + self.g0.xxy * vec3(other.g1.x, other.g1.y, other.g1.y) * vec3(0.0, -1.0, -1.0));
}

RadialPoint dilation_radial_point_geometric_anti_product(Dilation self, RadialPoint other) {
    return RadialPoint(vec3(self.g1.x) * vec3(other.g1.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g1.y) * other.g0 + self.g0.xxy * vec3(other.g1.x, other.g1.y, other.g1.y) * vec3(0.0, 1.0, 1.0), vec2(self.g1.y) * other.g1);
}

RadialPoint dilation_radial_point_inner_anti_product(Dilation self, RadialPoint other) {
    return RadialPoint(vec3(self.g1.y) * other.g0, vec2(self.g1.y) * other.g1);
}

RadialPoint dilation_radial_point_left_anti_contraction(Dilation self, RadialPoint other) {
    return RadialPoint(vec3(self.g1.y) * other.g0, vec2(self.g1.y) * other.g1);
}

AntiScalar dilation_flat_point_outer_product(Dilation self, FlatPoint other) {
    return AntiScalar(self.g0.x * other.g0.y + self.g0.y * other.g0.z + self.g0.z * other.g0.w + self.g1.x * other.g0.x);
}

FlatPoint dilation_flat_point_inner_anti_product(Dilation self, FlatPoint other) {
    return FlatPoint(vec4(self.g1.y) * other.g0);
}

FlatPoint dilation_flat_point_left_anti_contraction(Dilation self, FlatPoint other) {
    return FlatPoint(vec4(self.g1.y) * other.g0);
}

AntiScalar dilation_dipole_outer_product(Dilation self, Dipole other) {
    return AntiScalar(self.g0.x * other.g2.y + self.g0.y * other.g2.z + self.g0.z * other.g2.w + self.g1.x * other.g2.x);
}

Dipole dilation_dipole_inner_anti_product(Dilation self, Dipole other) {
    return Dipole(vec3(self.g1.y) * other.g0, vec3(self.g1.y) * other.g1, vec4(self.g1.y) * other.g2);
}

Dipole dilation_dipole_left_anti_contraction(Dilation self, Dipole other) {
    return Dipole(vec3(self.g1.y) * other.g0, vec3(self.g1.y) * other.g1, vec4(self.g1.y) * other.g2);
}

Line dilation_line_inner_anti_product(Dilation self, Line other) {
    return Line(vec3(self.g1.y) * other.g0, vec3(self.g1.y) * other.g1);
}

Line dilation_line_left_anti_contraction(Dilation self, Line other) {
    return Line(vec3(self.g1.y) * other.g0, vec3(self.g1.y) * other.g1);
}

Circle dilation_circle_inner_anti_product(Dilation self, Circle other) {
    return Circle(vec4(self.g1.y) * other.g0, vec3(self.g1.y) * other.g1, vec3(self.g1.y) * other.g2);
}

Scalar dilation_circle_left_contraction(Dilation self, Circle other) {
    return Scalar(self.g0.z * other.g0.w);
}

Circle dilation_circle_left_anti_contraction(Dilation self, Circle other) {
    return Circle(vec4(self.g1.y) * other.g0, vec3(self.g1.y) * other.g1, vec3(self.g1.y) * other.g2);
}

Scalar dilation_circle_scalar_product(Dilation self, Circle other) {
    return Scalar(self.g0.z * other.g0.w);
}

Sphere dilation_plane_inner_anti_product(Dilation self, Plane other) {
    return Sphere(vec2(self.g0.y) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g1.y) * vec2(other.g0.w) * vec2(0.0, 1.0) + vec2(self.g0.x) * vec2(other.g0.y, other.g0.x) * vec2(-1.0, 0.0), vec3(self.g1.y) * vec3(other.g0.x, other.g0.y, other.g0.z));
}

Plane dilation_plane_left_anti_contraction(Dilation self, Plane other) {
    return Plane(vec4(self.g1.y) * other.g0);
}

Sphere dilation_sphere_inner_anti_product(Dilation self, Sphere other) {
    return Sphere(vec2(self.g0.y) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g1.x) * vec2(other.g1.x) * vec2(-1.0, 0.0) + vec2(self.g1.y) * other.g0 + vec2(self.g0.x) * vec2(other.g1.y, other.g1.x) * vec2(-1.0, 0.0), vec3(self.g1.y) * other.g1);
}

Sphere dilation_sphere_left_anti_contraction(Dilation self, Sphere other) {
    return Sphere(vec2(self.g1.y) * other.g0, vec3(self.g1.y) * other.g1);
}

Flector dilation_motor_geometric_product(Dilation self, Motor other) {
    return Flector(vec4(self.g0.y) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, 1.0) - vec4(self.g0.z) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g0.w) + vec4(self.g1.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * other.g1.xxxy * vec4(0.0, 0.0, 0.0, 1.0), vec4(self.g0.y) * other.g1.yxwy * vec4(-1.0, 1.0, -1.0, 0.0) - vec4(self.g0.z) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.w) + vec4(self.g1.x) * other.g1.wzyw * vec4(-1.0, -1.0, 1.0, 0.0) + vec4(self.g0.x) * other.g1.zwxx * vec4(1.0, -1.0, -1.0, 0.0));
}

Plane dilation_motor_outer_product(Dilation self, Motor other) {
    return Plane(vec4(self.g1.x) * vec4(other.g1.w) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * other.g1.xwww * vec4(0.0, -1.0, -1.0, -1.0));
}

Motor dilation_motor_left_anti_contraction(Dilation self, Motor other) {
    return Motor(vec4(self.g1.y) * other.g0, vec4(self.g1.y) * other.g1);
}

Dilation dilation_motor_right_anti_contraction(Dilation self, Motor other) {
    return Dilation(self.g0 * vec3(other.g0.w), self.g1 * vec2(other.g0.w));
}

AntiScalar dilation_motor_anti_scalar_product(Dilation self, Motor other) {
    return AntiScalar(self.g1.y * other.g0.w);
}

Rotor dilation_rotor_left_anti_contraction(Dilation self, Rotor other) {
    return Rotor(vec4(self.g1.y) * other.g0);
}

Dilation dilation_rotor_right_anti_contraction(Dilation self, Rotor other) {
    return Dilation(self.g0 * vec3(other.g0.w), self.g1 * vec2(other.g0.w));
}

AntiScalar dilation_rotor_anti_scalar_product(Dilation self, Rotor other) {
    return AntiScalar(self.g1.y * other.g0.w);
}

Translator dilation_translator_left_anti_contraction(Dilation self, Translator other) {
    return Translator(vec4(self.g1.y) * other.g0);
}

Dilation dilation_translator_right_anti_contraction(Dilation self, Translator other) {
    return Dilation(self.g0 * vec3(other.g0.w), self.g1 * vec2(other.g0.w));
}

AntiScalar dilation_translator_anti_scalar_product(Dilation self, Translator other) {
    return AntiScalar(self.g1.y * other.g0.w);
}

Motor dilation_flector_geometric_product(Dilation self, Flector other) {
    return Motor(vec4(self.g0.x) * vec4(other.g0.z, other.g1.w, other.g0.x, other.g0.y) * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.y) * vec4(other.g0.y, other.g0.x, other.g1.w, other.g0.z) * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g0.w) + vec4(self.g1.x) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g0.x) * vec4(-1.0, -1.0, 1.0, 1.0), vec4(self.g0.z) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.w));
}

AntiScalar dilation_flector_outer_product(Dilation self, Flector other) {
    return AntiScalar(self.g0.x * other.g0.y + self.g0.y * other.g0.z + self.g0.z * other.g0.w + self.g1.x * other.g0.x);
}

Flector dilation_flector_left_anti_contraction(Dilation self, Flector other) {
    return Flector(vec4(self.g1.y) * other.g0, vec4(self.g1.y) * other.g1);
}

Dilation dilation_dilation_add(Dilation self, Dilation other) {
    return Dilation(self.g0 + other.g0, self.g1 + other.g1);
}

Dilation dilation_dilation_sub(Dilation self, Dilation other) {
    return Dilation(self.g0 - other.g0, self.g1 - other.g1);
}

Dilation dilation_dilation_mul(Dilation self, Dilation other) {
    return Dilation(self.g0 * other.g0, self.g1 * other.g1);
}

Dilation dilation_dilation_div(Dilation self, Dilation other) {
    return Dilation(vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(1.0, 1.0, 1.0) / vec3(other.g0.x, other.g0.y, other.g0.z) * vec3(1.0, 1.0, 1.0), vec2(self.g1.x, self.g1.y) * vec2(1.0, 1.0) / vec2(other.g1.x, other.g1.y) * vec2(1.0, 1.0));
}

Dilation dilation_dilation_regressive_product(Dilation self, Dilation other) {
    return Dilation(vec3(self.g1.y) * other.g0 + self.g0 * vec3(other.g1.y), vec2(self.g1.y) * other.g1 + vec2(self.g1.x) * other.g1.yx * vec2(1.0, 0.0));
}

Dilation dilation_dilation_geometric_anti_product(Dilation self, Dilation other) {
    return Dilation(vec3(self.g1.y) * other.g0 + self.g0 * vec3(other.g1.y), vec2(self.g1.y) * other.g1 + vec2(self.g1.x) * other.g1.yx * vec2(1.0, 0.0));
}

Dilation dilation_dilation_inner_anti_product(Dilation self, Dilation other) {
    return Dilation(vec3(self.g1.y) * other.g0 + self.g0 * vec3(other.g1.y), vec2(self.g1.y) * other.g1 + vec2(self.g1.x) * other.g1.yx * vec2(1.0, 0.0));
}

Dilation dilation_dilation_left_anti_contraction(Dilation self, Dilation other) {
    return Dilation(vec3(self.g1.y) * other.g0, vec2(self.g1.y) * other.g1);
}

Dilation dilation_dilation_right_anti_contraction(Dilation self, Dilation other) {
    return Dilation(self.g0 * vec3(other.g1.y), self.g1 * vec2(other.g1.y));
}

Scalar dilation_dilation_scalar_product(Dilation self, Dilation other) {
    return Scalar(0.0 - self.g0.z * other.g0.z);
}

AntiScalar dilation_dilation_anti_scalar_product(Dilation self, Dilation other) {
    return AntiScalar(self.g1.y * other.g1.y);
}

MultiVector dilation_multi_vector_geometric_product(Dilation self, MultiVector other) {
    return MultiVector(vec3(self.g0.y) * vec3(other.g1.z, other.g1.z, other.g3.z) * vec3(0.0, 1.0, 1.0) + vec3(self.g0.z) * vec3(other.g8.w, other.g2.x, other.g3.w) + vec3(self.g1.x) * vec3(other.g1.x, other.g1.x, other.g3.x) * vec3(0.0, 1.0, 1.0) + vec3(self.g1.y) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0) + vec3(self.g0.x) * vec3(other.g1.x, other.g1.y, other.g3.y) * vec3(0.0, 1.0, 1.0), vec3(0.0) - vec3(self.g0.z) * other.g5, vec2(self.g0.y) * vec2(other.g5.z) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.y, other.g9.w) * vec2(-1.0, 1.0) + vec2(self.g1.x) * vec2(other.g5.x) * vec2(1.0, 0.0) + vec2(self.g0.x) * vec2(other.g5.y, other.g5.x) * vec2(1.0, 0.0), vec4(self.g0.y) * vec4(other.g7.z) * vec4(0.0, 0.0, 0.0, 1.0) - vec4(self.g0.z) * vec4(other.g7.x, other.g7.y, other.g7.z, other.g0.z) + vec4(self.g1.x) * vec4(other.g7.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.y) * vec4(other.g8.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g7.x, other.g7.x, other.g7.x, other.g7.y) * vec4(0.0, 0.0, 0.0, 1.0), vec3(self.g0.x) * vec3(other.g1.z, other.g8.w, other.g1.x) * vec3(1.0, -1.0, -1.0) + vec3(self.g0.y) * vec3(other.g1.y, other.g1.x, other.g8.w) * vec3(-1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g8.x, other.g8.y, other.g8.z) + vec3(self.g1.x) * vec3(other.g8.w, other.g1.z, other.g1.y) * vec3(-1.0, -1.0, 1.0), vec3(self.g0.z) * other.g1, vec3(self.g0.x) * vec3(other.g3.z, other.g9.w, other.g3.x) * vec3(1.0, -1.0, -1.0) + vec3(self.g0.y) * vec3(other.g3.y, other.g3.x, other.g9.w) * vec3(-1.0, 1.0, -1.0) + vec3(self.g0.z) * vec3(other.g9.x, other.g9.y, other.g9.z) + vec3(self.g1.x) * vec3(other.g9.w, other.g3.z, other.g3.y) * vec3(-1.0, -1.0, 1.0) + vec3(self.g1.y) * other.g5, vec3(self.g0.z) * vec3(other.g3.x, other.g3.y, other.g3.z), vec4(self.g0.y) * vec4(other.g5.y, other.g5.x, other.g0.x, other.g5.y) * vec4(-1.0, 1.0, -1.0, 0.0) - vec4(self.g0.z) * vec4(other.g4.x, other.g4.y, other.g4.z, other.g0.x) + vec4(self.g1.x) * vec4(other.g0.x, other.g5.z, other.g5.y, other.g0.x) * vec4(-1.0, -1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g5.z, other.g0.x, other.g5.x, other.g5.x) * vec4(1.0, -1.0, -1.0, 0.0), vec4(self.g0.y) * vec4(other.g7.y, other.g7.x, other.g2.y, other.g7.y) * vec4(-1.0, 1.0, -1.0, 0.0) - vec4(self.g0.z) * vec4(other.g6.x, other.g6.y, other.g6.z, other.g2.y) + vec4(self.g1.x) * vec4(other.g2.y, other.g7.z, other.g7.y, other.g2.y) * vec4(-1.0, -1.0, 1.0, 0.0) + vec4(self.g1.y) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g7.z, other.g2.y, other.g7.x, other.g7.x) * vec4(1.0, -1.0, -1.0, 0.0));
}

MultiVector dilation_multi_vector_regressive_product(Dilation self, MultiVector other) {
    return MultiVector(vec3(self.g0.y) * vec3(other.g3.z) * vec3(1.0, 0.0, 0.0) + vec3(self.g0.z) * vec3(other.g3.w) * vec3(1.0, 0.0, 0.0) + vec3(self.g1.x) * vec3(other.g3.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g1.y) * other.g0 + vec3(self.g0.x) * vec3(other.g3.y, other.g3.x, other.g3.x) * vec3(1.0, 0.0, 0.0), vec3(self.g0.y) * other.g7.yxy * vec3(-1.0, 1.0, 0.0) - vec3(self.g0.z) * other.g6 + vec3(self.g1.x) * other.g7.zzy * vec3(0.0, -1.0, 1.0) + vec3(self.g1.y) * other.g1 + vec3(self.g0.x) * other.g7.zxx * vec3(1.0, 0.0, -1.0), vec2(self.g0.y) * vec2(other.g6.z) * vec2(1.0, 0.0) + vec2(self.g1.x) * vec2(other.g6.x) * vec2(1.0, 0.0) + vec2(self.g1.y) * other.g2 + vec2(self.g0.x) * vec2(other.g6.y, other.g6.x) * vec2(1.0, 0.0), vec4(self.g1.y) * other.g3, vec3(self.g0.y) * vec3(other.g9.y, other.g9.x, other.g9.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.x) * vec3(other.g9.z, other.g9.z, other.g9.y) * vec3(0.0, -1.0, 1.0) + vec3(self.g1.y) * other.g4 + vec3(self.g0.x) * vec3(other.g9.z, other.g9.x, other.g9.x) * vec3(1.0, 0.0, -1.0), vec3(self.g0.z) * vec3(other.g9.x, other.g9.y, other.g9.z) + vec3(self.g1.x) * vec3(other.g9.w) * vec3(-1.0, 0.0, 0.0) + vec3(self.g1.y) * other.g5 + self.g0.xxy * vec3(other.g9.x, other.g9.w, other.g9.w) * vec3(0.0, -1.0, -1.0), vec3(self.g1.y) * other.g6, vec3(self.g1.y) * other.g7, vec4(self.g1.x) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * other.g8 + vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(other.g0.x, other.g0.z, other.g0.z, other.g0.z) * vec4(0.0, -1.0, -1.0, -1.0), vec4(self.g1.y) * other.g9);
}

MultiVector dilation_multi_vector_geometric_anti_product(Dilation self, MultiVector other) {
    return MultiVector(vec3(self.g0.y) * vec3(other.g3.z, other.g9.z, other.g3.z) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.z) * vec3(other.g3.w) * vec3(1.0, 0.0, 0.0) + vec3(self.g1.x) * vec3(other.g3.x, other.g9.x, other.g3.x) * vec3(1.0, -1.0, 0.0) + vec3(self.g1.y) * other.g0 + vec3(self.g0.x) * vec3(other.g3.y, other.g9.y, other.g3.x) * vec3(1.0, -1.0, 0.0), vec3(self.g0.x) * vec3(other.g7.z, other.g2.y, other.g7.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.y) * vec3(other.g7.y, other.g7.x, other.g2.y) * vec3(-1.0, 1.0, 1.0) - vec3(self.g0.z) * other.g6 + vec3(self.g1.x) * vec3(other.g2.y, other.g7.z, other.g7.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.y) * other.g1, vec2(self.g0.y) * vec2(other.g6.z) * vec2(1.0, 0.0) + vec2(self.g1.x) * vec2(other.g6.x) * vec2(1.0, 0.0) + vec2(self.g1.y) * other.g2 + vec2(self.g0.x) * vec2(other.g6.y, other.g6.x) * vec2(1.0, 0.0), vec4(self.g1.y) * other.g3, vec3(self.g0.x) * vec3(other.g9.z, other.g3.w, other.g9.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.y) * vec3(other.g9.y, other.g9.x, other.g3.w) * vec3(-1.0, 1.0, 1.0) + vec3(self.g1.x) * vec3(other.g3.w, other.g9.z, other.g9.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.y) * other.g4, vec3(self.g0.x) * vec3(other.g3.z, other.g9.w, other.g3.x) * vec3(-1.0, -1.0, 1.0) + vec3(self.g0.y) * vec3(other.g3.y, other.g3.x, other.g9.w) * vec3(1.0, -1.0, -1.0) + vec3(self.g0.z) * vec3(other.g9.x, other.g9.y, other.g9.z) + vec3(self.g1.x) * vec3(other.g9.w, other.g3.z, other.g3.y) * vec3(-1.0, 1.0, -1.0) + vec3(self.g1.y) * other.g5, vec3(self.g1.y) * other.g6, vec3(self.g1.y) * other.g7, vec4(self.g0.x) * vec4(other.g6.z, other.g0.z, other.g6.x, other.g7.y) * vec4(-1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * vec4(other.g6.y, other.g6.x, other.g0.z, other.g7.z) * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g1.x) * vec4(other.g0.z, other.g6.z, other.g6.y, other.g7.x) * vec4(-1.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * other.g8 + vec4(self.g0.x, self.g0.x, self.g0.x, self.g0.z) * vec4(other.g0.x, other.g0.x, other.g0.x, other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0), vec4(self.g1.y) * other.g9);
}

MultiVector dilation_multi_vector_inner_anti_product(Dilation self, MultiVector other) {
    return MultiVector(vec3(self.g0.y) * vec3(other.g9.z) * vec3(0.0, -1.0, 0.0) + vec3(self.g1.x) * vec3(other.g9.x) * vec3(0.0, -1.0, 0.0) + vec3(self.g1.y) * other.g0 + vec3(self.g0.x) * vec3(other.g9.x, other.g9.y, other.g9.x) * vec3(0.0, -1.0, 0.0), vec3(self.g1.y) * other.g1, vec2(self.g1.y) * other.g2, vec4(self.g1.y) * other.g3, vec3(self.g1.y) * other.g4, vec3(self.g1.y) * other.g5, vec3(self.g1.y) * other.g6, vec3(self.g1.y) * other.g7, vec4(self.g1.x) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * other.g8 + vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(other.g0.x, other.g0.z, other.g0.z, other.g0.z) * vec4(0.0, -1.0, -1.0, -1.0), vec4(self.g1.y) * other.g9);
}

MultiVector dilation_multi_vector_left_anti_contraction(Dilation self, MultiVector other) {
    return MultiVector(vec3(self.g1.y) * other.g0, vec3(self.g1.y) * other.g1, vec2(self.g1.y) * other.g2, vec4(self.g1.y) * other.g3, vec3(self.g1.y) * other.g4, vec3(self.g1.y) * other.g5, vec3(self.g1.y) * other.g6, vec3(self.g1.y) * other.g7, vec4(self.g1.y) * other.g8, vec4(self.g1.y) * other.g9);
}

Scalar dilation_multi_vector_scalar_product(Dilation self, MultiVector other) {
    return Scalar(self.g0.z * other.g8.w);
}

AntiScalar dilation_multi_vector_anti_scalar_product(Dilation self, MultiVector other) {
    return AntiScalar(self.g1.y * other.g0.z);
}

Scalar dilation_squared_magnitude(Dilation self) {
    return dilation_dilation_scalar_product(self, dilation_reversal(self));
}

Scalar dilation_magnitude(Dilation self) {
    return Scalar(sqrt(dilation_squared_magnitude(self).g0));
}

Scalar dilation_bulk_norm(Dilation self) {
    return Scalar(sqrt(dilation_squared_magnitude(self).g0));
}

AntiScalar dilation_squared_anti_magnitude(Dilation self) {
    return dilation_dilation_anti_scalar_product(self, dilation_anti_reversal(self));
}

AntiScalar dilation_weight_norm(Dilation self) {
    return AntiScalar(sqrt(dilation_squared_anti_magnitude(self).g0));
}

HomogeneousMagnitude dilation_geometric_norm(Dilation self) {
    return scalar_anti_scalar_add(dilation_bulk_norm(self), dilation_weight_norm(self));
}

Dilation dilation_scale(Dilation self, float other) {
    return dilation_scalar_geometric_product(self, Scalar(other));
}

Dilation dilation_signum(Dilation self) {
    return dilation_scalar_geometric_product(self, Scalar(1.0 / dilation_magnitude(self).g0));
}

Dilation dilation_inverse(Dilation self) {
    return dilation_scalar_geometric_product(dilation_reversal(self), Scalar(1.0 / dilation_squared_magnitude(self).g0));
}

Dilation dilation_unitize(Dilation self) {
    return dilation_scalar_geometric_product(self, Scalar(1.0 / dilation_weight_norm(self).g0));
}

MultiVector multi_vector_zero() {
    return MultiVector(vec3(0.0), vec3(0.0), vec2(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0));
}

MultiVector multi_vector_one() {
    return MultiVector(vec3(1.0, 0.0, 0.0), vec3(0.0), vec2(0.0), vec4(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec3(0.0), vec4(0.0), vec4(0.0));
}

MultiVector multi_vector_neg(MultiVector self) {
    return MultiVector(self.g0 * vec3(-1.0), self.g1 * vec3(-1.0), self.g2 * vec2(-1.0), self.g3 * vec4(-1.0), self.g4 * vec3(-1.0), self.g5 * vec3(-1.0), self.g6 * vec3(-1.0), self.g7 * vec3(-1.0), self.g8 * vec4(-1.0), self.g9 * vec4(-1.0));
}

MultiVector multi_vector_automorphism(MultiVector self) {
    return MultiVector(self.g0 * vec3(1.0, 1.0, -1.0), self.g1 * vec3(-1.0), self.g2 * vec2(-1.0), self.g3, self.g4, self.g5, self.g6 * vec3(-1.0), self.g7 * vec3(-1.0), self.g8 * vec4(-1.0), self.g9);
}

MultiVector multi_vector_reversal(MultiVector self) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3 * vec4(-1.0), self.g4 * vec3(-1.0), self.g5 * vec3(-1.0), self.g6 * vec3(-1.0), self.g7 * vec3(-1.0), self.g8 * vec4(-1.0), self.g9);
}

MultiVector multi_vector_conjugation(MultiVector self) {
    return MultiVector(self.g0 * vec3(1.0, 1.0, -1.0), self.g1 * vec3(-1.0), self.g2 * vec2(-1.0), self.g3 * vec4(-1.0), self.g4 * vec3(-1.0), self.g5 * vec3(-1.0), self.g6, self.g7, self.g8, self.g9);
}

MultiVector multi_vector_dual(MultiVector self) {
    return MultiVector(vec3(self.g0.z, self.g2.y, self.g0.x), vec3(self.g9.x, self.g9.y, self.g9.z), vec2(self.g9.w, self.g0.y), self.g8 * vec4(-1.0), self.g7 * vec3(-1.0), self.g6 * vec3(-1.0), self.g5 * vec3(-1.0), self.g4 * vec3(-1.0), self.g3 * vec4(-1.0), vec4(self.g1.x, self.g1.y, self.g1.z, self.g2.x));
}

MultiVector multi_vector_anti_reversal(MultiVector self) {
    return MultiVector(self.g0 * vec3(1.0, 1.0, -1.0), self.g1 * vec3(-1.0), self.g2 * vec2(-1.0), self.g3 * vec4(-1.0), self.g4 * vec3(-1.0), self.g5 * vec3(-1.0), self.g6, self.g7, self.g8, self.g9);
}

MultiVector multi_vector_right_complement(MultiVector self) {
    return MultiVector(vec3(self.g0.z, self.g2.y, self.g0.x), vec3(self.g9.x, self.g9.y, self.g9.z), vec2(self.g9.w, self.g0.y), self.g8 * vec4(-1.0), self.g7 * vec3(-1.0), self.g6 * vec3(-1.0), self.g5 * vec3(-1.0), self.g4 * vec3(-1.0), self.g3 * vec4(-1.0), vec4(self.g1.x, self.g1.y, self.g1.z, self.g2.x));
}

MultiVector multi_vector_left_complement(MultiVector self) {
    return MultiVector(vec3(self.g0.z, self.g2.y, self.g0.x), vec3(self.g9.x, self.g9.y, self.g9.z), vec2(self.g9.w, self.g0.y), self.g8 * vec4(-1.0), self.g7 * vec3(-1.0), self.g6 * vec3(-1.0), self.g5 * vec3(-1.0), self.g4 * vec3(-1.0), self.g3 * vec4(-1.0), vec4(self.g1.x, self.g1.y, self.g1.z, self.g2.x));
}

MultiVector multi_vector_double_complement(MultiVector self) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3, self.g4, self.g5, self.g6, self.g7, self.g8, self.g9);
}

Scalar multi_vector_scalar_into(MultiVector self) {
    return Scalar(self.g0.x);
}

MultiVector multi_vector_scalar_add(MultiVector self, Scalar other) {
    return MultiVector(self.g0 + vec3(other.g0) * vec3(1.0, 0.0, 0.0), self.g1, self.g2, self.g3, self.g4, self.g5, self.g6, self.g7, self.g8, self.g9);
}

MultiVector multi_vector_scalar_sub(MultiVector self, Scalar other) {
    return MultiVector(self.g0 - vec3(other.g0) * vec3(1.0, 0.0, 0.0), self.g1, self.g2, self.g3, self.g4, self.g5, self.g6, self.g7, self.g8, self.g9);
}

MultiVector multi_vector_scalar_geometric_product(MultiVector self, Scalar other) {
    return MultiVector(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0), self.g2 * vec2(other.g0), self.g3 * vec4(other.g0), self.g4 * vec3(other.g0), self.g5 * vec3(other.g0), self.g6 * vec3(other.g0), self.g7 * vec3(other.g0), self.g8 * vec4(other.g0), self.g9 * vec4(other.g0));
}

Scalar multi_vector_scalar_regressive_product(MultiVector self, Scalar other) {
    return Scalar(self.g0.z * other.g0);
}

MultiVector multi_vector_scalar_outer_product(MultiVector self, Scalar other) {
    return MultiVector(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0), self.g2 * vec2(other.g0), self.g3 * vec4(other.g0), self.g4 * vec3(other.g0), self.g5 * vec3(other.g0), self.g6 * vec3(other.g0), self.g7 * vec3(other.g0), self.g8 * vec4(other.g0), self.g9 * vec4(other.g0));
}

MultiVector multi_vector_scalar_inner_product(MultiVector self, Scalar other) {
    return MultiVector(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0), self.g2 * vec2(other.g0), self.g3 * vec4(other.g0), self.g4 * vec3(other.g0), self.g5 * vec3(other.g0), self.g6 * vec3(other.g0), self.g7 * vec3(other.g0), self.g8 * vec4(other.g0), self.g9 * vec4(other.g0));
}

Scalar multi_vector_scalar_left_contraction(MultiVector self, Scalar other) {
    return Scalar(self.g0.x * other.g0);
}

MultiVector multi_vector_scalar_right_contraction(MultiVector self, Scalar other) {
    return MultiVector(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0), self.g2 * vec2(other.g0), self.g3 * vec4(other.g0), self.g4 * vec3(other.g0), self.g5 * vec3(other.g0), self.g6 * vec3(other.g0), self.g7 * vec3(other.g0), self.g8 * vec4(other.g0), self.g9 * vec4(other.g0));
}

Scalar multi_vector_scalar_scalar_product(MultiVector self, Scalar other) {
    return Scalar(self.g0.x * other.g0);
}

AntiScalar multi_vector_anti_scalar_into(MultiVector self) {
    return AntiScalar(self.g0.z);
}

MultiVector multi_vector_anti_scalar_add(MultiVector self, AntiScalar other) {
    return MultiVector(self.g0 + vec3(other.g0) * vec3(0.0, 0.0, 1.0), self.g1, self.g2, self.g3, self.g4, self.g5, self.g6, self.g7, self.g8, self.g9);
}

MultiVector multi_vector_anti_scalar_sub(MultiVector self, AntiScalar other) {
    return MultiVector(self.g0 - vec3(other.g0) * vec3(0.0, 0.0, 1.0), self.g1, self.g2, self.g3, self.g4, self.g5, self.g6, self.g7, self.g8, self.g9);
}

MultiVector multi_vector_anti_scalar_regressive_product(MultiVector self, AntiScalar other) {
    return MultiVector(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0), self.g2 * vec2(other.g0), self.g3 * vec4(other.g0), self.g4 * vec3(other.g0), self.g5 * vec3(other.g0), self.g6 * vec3(other.g0), self.g7 * vec3(other.g0), self.g8 * vec4(other.g0), self.g9 * vec4(other.g0));
}

AntiScalar multi_vector_anti_scalar_outer_product(MultiVector self, AntiScalar other) {
    return AntiScalar(self.g0.x * other.g0);
}

MultiVector multi_vector_anti_scalar_geometric_anti_product(MultiVector self, AntiScalar other) {
    return MultiVector(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0), self.g2 * vec2(other.g0), self.g3 * vec4(other.g0), self.g4 * vec3(other.g0), self.g5 * vec3(other.g0), self.g6 * vec3(other.g0), self.g7 * vec3(other.g0), self.g8 * vec4(other.g0), self.g9 * vec4(other.g0));
}

MultiVector multi_vector_anti_scalar_inner_anti_product(MultiVector self, AntiScalar other) {
    return MultiVector(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0), self.g2 * vec2(other.g0), self.g3 * vec4(other.g0), self.g4 * vec3(other.g0), self.g5 * vec3(other.g0), self.g6 * vec3(other.g0), self.g7 * vec3(other.g0), self.g8 * vec4(other.g0), self.g9 * vec4(other.g0));
}

AntiScalar multi_vector_anti_scalar_left_anti_contraction(MultiVector self, AntiScalar other) {
    return AntiScalar(self.g0.z * other.g0);
}

MultiVector multi_vector_anti_scalar_right_anti_contraction(MultiVector self, AntiScalar other) {
    return MultiVector(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0), self.g2 * vec2(other.g0), self.g3 * vec4(other.g0), self.g4 * vec3(other.g0), self.g5 * vec3(other.g0), self.g6 * vec3(other.g0), self.g7 * vec3(other.g0), self.g8 * vec4(other.g0), self.g9 * vec4(other.g0));
}

AntiScalar multi_vector_anti_scalar_anti_scalar_product(MultiVector self, AntiScalar other) {
    return AntiScalar(self.g0.z * other.g0);
}

HomogeneousMagnitude multi_vector_homogeneous_magnitude_into(MultiVector self) {
    return HomogeneousMagnitude(vec2(self.g0.x, self.g0.z));
}

MultiVector multi_vector_homogeneous_magnitude_add(MultiVector self, HomogeneousMagnitude other) {
    return MultiVector(self.g0 + vec3(other.g0.x, other.g0.x, other.g0.y) * vec3(1.0, 0.0, 1.0), self.g1, self.g2, self.g3, self.g4, self.g5, self.g6, self.g7, self.g8, self.g9);
}

MultiVector multi_vector_homogeneous_magnitude_sub(MultiVector self, HomogeneousMagnitude other) {
    return MultiVector(self.g0 - vec3(other.g0.x, other.g0.x, other.g0.y) * vec3(1.0, 0.0, 1.0), self.g1, self.g2, self.g3, self.g4, self.g5, self.g6, self.g7, self.g8, self.g9);
}

MultiVector multi_vector_homogeneous_magnitude_geometric_product(MultiVector self, HomogeneousMagnitude other) {
    return MultiVector(vec3(self.g0.z) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0) + self.g0.xyx * vec3(other.g0.x, other.g0.x, other.g0.y), self.g1 * vec3(other.g0.x), self.g2 * vec2(other.g0.x), vec4(self.g8.w) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + self.g3 * vec4(other.g0.x), self.g4 * vec3(other.g0.x), self.g5 * vec3(other.g0.x), vec3(self.g6.x) * vec3(other.g0.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g6.y) * vec3(other.g0.x) * vec3(0.0, 1.0, 0.0) + vec3(self.g6.z) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0) + self.g5 * vec3(other.g0.y), self.g7 * vec3(other.g0.x), self.g8 * vec4(other.g0.x), vec4(self.g9.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g9.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g9.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g9.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.x) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

MultiVector multi_vector_homogeneous_magnitude_regressive_product(MultiVector self, HomogeneousMagnitude other) {
    return MultiVector(vec3(self.g0.z) * vec3(other.g0.x, other.g0.x, other.g0.y) * vec3(1.0, 0.0, 1.0) + self.g0.xyx * vec3(other.g0.y, other.g0.y, other.g0.x) * vec3(1.0, 1.0, 0.0), self.g1 * vec3(other.g0.y), self.g2 * vec2(other.g0.y), self.g3 * vec4(other.g0.y), self.g4 * vec3(other.g0.y), self.g5 * vec3(other.g0.y), self.g6 * vec3(other.g0.y), self.g7 * vec3(other.g0.y), self.g8 * vec4(other.g0.y), self.g9 * vec4(other.g0.y));
}

MultiVector multi_vector_homogeneous_magnitude_outer_product(MultiVector self, HomogeneousMagnitude other) {
    return MultiVector(vec3(self.g0.z) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0) + self.g0.xyx * vec3(other.g0.x, other.g0.x, other.g0.y), self.g1 * vec3(other.g0.x), self.g2 * vec2(other.g0.x), self.g3 * vec4(other.g0.x), self.g4 * vec3(other.g0.x), self.g5 * vec3(other.g0.x), self.g6 * vec3(other.g0.x), self.g7 * vec3(other.g0.x), self.g8 * vec4(other.g0.x), self.g9 * vec4(other.g0.x));
}

MultiVector multi_vector_homogeneous_magnitude_inner_product(MultiVector self, HomogeneousMagnitude other) {
    return MultiVector(vec3(self.g0.z) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0) + self.g0.xyx * vec3(other.g0.x, other.g0.x, other.g0.y), self.g1 * vec3(other.g0.x), self.g2 * vec2(other.g0.x), vec4(self.g8.w) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + self.g3 * vec4(other.g0.x), self.g4 * vec3(other.g0.x), self.g5 * vec3(other.g0.x), vec3(self.g6.x) * vec3(other.g0.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g6.y) * vec3(other.g0.x) * vec3(0.0, 1.0, 0.0) + vec3(self.g6.z) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0) + self.g5 * vec3(other.g0.y), self.g7 * vec3(other.g0.x), self.g8 * vec4(other.g0.x), vec4(self.g9.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g9.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g9.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g9.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.x) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

MultiVector multi_vector_homogeneous_magnitude_geometric_anti_product(MultiVector self, HomogeneousMagnitude other) {
    return MultiVector(vec3(self.g0.z) * vec3(other.g0.x, other.g0.x, other.g0.y) * vec3(1.0, 0.0, 1.0) + self.g0.xyx * vec3(other.g0.y, other.g0.y, other.g0.x) * vec3(1.0, 1.0, 0.0), vec3(self.g9.x) * vec3(other.g0.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g9.y) * vec3(other.g0.x) * vec3(0.0, 1.0, 0.0) + vec3(self.g9.z) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0) + self.g1 * vec3(other.g0.y), self.g2 * vec2(other.g0.y), self.g3 * vec4(other.g0.y), self.g4 * vec3(other.g0.y), vec3(self.g6.x) * vec3(other.g0.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g6.y) * vec3(other.g0.x) * vec3(0.0, 1.0, 0.0) + vec3(self.g6.z) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0) + self.g5 * vec3(other.g0.y), self.g6 * vec3(other.g0.y), self.g7 * vec3(other.g0.y), vec4(self.g8.w) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g8.x, self.g8.y, self.g8.z, self.g3.w) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x), self.g9 * vec4(other.g0.y));
}

MultiVector multi_vector_homogeneous_magnitude_inner_anti_product(MultiVector self, HomogeneousMagnitude other) {
    return MultiVector(vec3(self.g0.z) * vec3(other.g0.x, other.g0.x, other.g0.y) * vec3(1.0, 0.0, 1.0) + self.g0.xyx * vec3(other.g0.y, other.g0.y, other.g0.x) * vec3(1.0, 1.0, 0.0), vec3(self.g9.x) * vec3(other.g0.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g9.y) * vec3(other.g0.x) * vec3(0.0, 1.0, 0.0) + vec3(self.g9.z) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0) + self.g1 * vec3(other.g0.y), self.g2 * vec2(other.g0.y), self.g3 * vec4(other.g0.y), self.g4 * vec3(other.g0.y), vec3(self.g6.x) * vec3(other.g0.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g6.y) * vec3(other.g0.x) * vec3(0.0, 1.0, 0.0) + vec3(self.g6.z) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0) + self.g5 * vec3(other.g0.y), self.g6 * vec3(other.g0.y), self.g7 * vec3(other.g0.y), vec4(self.g8.w) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g8.x, self.g8.y, self.g8.z, self.g3.w) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x), self.g9 * vec4(other.g0.y));
}

MultiVector multi_vector_homogeneous_magnitude_right_contraction(MultiVector self, HomogeneousMagnitude other) {
    return MultiVector(self.g0 * vec3(other.g0.x), self.g1 * vec3(other.g0.x), self.g2 * vec2(other.g0.x), self.g3 * vec4(other.g0.x), self.g4 * vec3(other.g0.x), self.g5 * vec3(other.g0.x), self.g6 * vec3(other.g0.x), self.g7 * vec3(other.g0.x), self.g8 * vec4(other.g0.x), self.g9 * vec4(other.g0.x));
}

MultiVector multi_vector_homogeneous_magnitude_right_anti_contraction(MultiVector self, HomogeneousMagnitude other) {
    return MultiVector(self.g0 * vec3(other.g0.y), self.g1 * vec3(other.g0.y), self.g2 * vec2(other.g0.y), self.g3 * vec4(other.g0.y), self.g4 * vec3(other.g0.y), self.g5 * vec3(other.g0.y), self.g6 * vec3(other.g0.y), self.g7 * vec3(other.g0.y), self.g8 * vec4(other.g0.y), self.g9 * vec4(other.g0.y));
}

Scalar multi_vector_homogeneous_magnitude_scalar_product(MultiVector self, HomogeneousMagnitude other) {
    return Scalar(self.g0.x * other.g0.x);
}

AntiScalar multi_vector_homogeneous_magnitude_anti_scalar_product(MultiVector self, HomogeneousMagnitude other) {
    return AntiScalar(self.g0.z * other.g0.y);
}

RadialPoint multi_vector_radial_point_into(MultiVector self) {
    return RadialPoint(self.g1, self.g2);
}

MultiVector multi_vector_radial_point_add(MultiVector self, RadialPoint other) {
    return MultiVector(self.g0, self.g1 + other.g0, self.g2 + other.g1, self.g3, self.g4, self.g5, self.g6, self.g7, self.g8, self.g9);
}

MultiVector multi_vector_radial_point_sub(MultiVector self, RadialPoint other) {
    return MultiVector(self.g0, self.g1 - other.g0, self.g2 - other.g1, self.g3, self.g4, self.g5, self.g6, self.g7, self.g8, self.g9);
}

MultiVector multi_vector_radial_point_geometric_product(MultiVector self, RadialPoint other) {
    return MultiVector(vec3(self.g1.x) * vec3(other.g0.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g1.y) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g1.z) * vec3(other.g0.z) * vec3(1.0, 0.0, 0.0) + vec3(self.g8.x) * vec3(other.g0.x) * vec3(0.0, -1.0, 0.0) + vec3(self.g8.y) * vec3(other.g0.y) * vec3(0.0, -1.0, 0.0) + vec3(self.g8.z) * vec3(other.g0.z) * vec3(0.0, -1.0, 0.0) + vec3(self.g8.w) * vec3(other.g1.x) * vec3(0.0, -1.0, 0.0) + vec3(self.g9.x) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0) + vec3(self.g9.y) * vec3(other.g0.y) * vec3(0.0, 0.0, 1.0) + vec3(self.g9.z) * vec3(other.g0.z) * vec3(0.0, 0.0, 1.0) + vec3(self.g9.w) * vec3(other.g1.x) * vec3(0.0, 0.0, 1.0) + self.g0.xxy * vec3(other.g1.x, other.g1.x, other.g1.y) * vec3(0.0, 0.0, 1.0), vec3(self.g0.x) * other.g0 + vec3(self.g5.y) * other.g0.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g5.z) * other.g0.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g5.x) * other.g0.xzy * vec3(0.0, 1.0, -1.0), vec2(self.g0.x) * other.g1 + vec2(self.g3.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g3.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g4.x) * vec2(other.g0.x) * vec2(1.0, 0.0) + vec2(self.g4.y) * vec2(other.g0.y) * vec2(1.0, 0.0) + vec2(self.g4.z) * vec2(other.g0.z) * vec2(1.0, 0.0) + vec2(self.g3.x) * vec2(other.g0.x) * vec2(0.0, -1.0), vec4(self.g2.x) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, 1.0) - vec4(self.g2.y) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.x) + vec4(self.g6.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g6.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g6.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g7.x) * vec4(other.g0.z, other.g0.z, other.g0.y, other.g0.z) * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g7.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g0.z) * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g7.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.y) * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.x) * vec4(other.g1.y, other.g1.y, other.g1.y, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0), vec3(self.g2.x) * other.g0 + vec3(self.g8.x) * other.g0.zzy * vec3(0.0, 1.0, -1.0) + vec3(self.g8.y) * other.g0.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g8.z) * other.g0.yxy * vec3(1.0, -1.0, 0.0) + self.g1 * vec3(other.g1.x) * vec3(-1.0), vec3(self.g1.y) * other.g0.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * other.g0.yxy * vec3(-1.0, 1.0, 0.0) - vec3(self.g8.w) * other.g0 + vec3(self.g1.x) * other.g0.xzy * vec3(0.0, -1.0, 1.0), vec3(0.0) - vec3(self.g3.w) * other.g0 + vec3(self.g4.x) * vec3(other.g1.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g4.y) * vec3(other.g1.y) * vec3(0.0, 1.0, 0.0) + vec3(self.g4.z) * vec3(other.g1.y) * vec3(0.0, 0.0, 1.0) + vec3(self.g9.x) * other.g0.zzy * vec3(0.0, -1.0, 1.0) + vec3(self.g9.y) * other.g0.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g9.z) * other.g0.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g3.x, self.g3.y, self.g3.z) * vec3(other.g1.x), vec3(self.g3.y) * other.g0.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g3.z) * other.g0.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g5.x) * vec3(other.g1.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g5.y) * vec3(other.g1.y) * vec3(0.0, 1.0, 0.0) + vec3(self.g5.z) * vec3(other.g1.y) * vec3(0.0, 0.0, 1.0) + vec3(self.g9.w) * other.g0 + vec3(self.g3.x) * other.g0.xzy * vec3(0.0, 1.0, -1.0), vec4(self.g4.x) * vec4(other.g0.z, other.g0.z, other.g0.y, other.g0.z) * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g4.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g0.z) * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g4.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.y) * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g5.x) * vec4(other.g1.x, other.g1.x, other.g1.x, other.g0.x) * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g5.y) * vec4(other.g1.x, other.g1.x, other.g1.x, other.g0.y) * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g5.z) * vec4(other.g1.x, other.g1.x, other.g1.x, other.g0.z) * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.y, self.g0.y, self.g0.y, self.g0.x) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(-1.0, -1.0, -1.0, 0.0), vec4(self.g6.x) * vec4(other.g0.z, other.g0.z, other.g0.y, other.g0.z) * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g6.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g0.z) * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g6.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.y) * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g7.x) * vec4(other.g1.x, other.g1.x, other.g1.x, other.g0.x) * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g7.y) * vec4(other.g1.x, other.g1.x, other.g1.x, other.g0.y) * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g7.z) * vec4(other.g1.x, other.g1.x, other.g1.x, other.g0.z) * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g8.x) * vec4(other.g1.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g8.y) * vec4(other.g1.y) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g8.z) * vec4(other.g1.y) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g8.w) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.z, self.g0.z, self.g0.z, self.g0.x) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Scalar multi_vector_radial_point_scalar_product(MultiVector self, RadialPoint other) {
    return Scalar(self.g1.x * other.g0.x + self.g1.y * other.g0.y + self.g1.z * other.g0.z);
}

FlatPoint multi_vector_flat_point_into(MultiVector self) {
    return FlatPoint(self.g3);
}

MultiVector multi_vector_flat_point_add(MultiVector self, FlatPoint other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3 + other.g0, self.g4, self.g5, self.g6, self.g7, self.g8, self.g9);
}

MultiVector multi_vector_flat_point_sub(MultiVector self, FlatPoint other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3 - other.g0, self.g4, self.g5, self.g6, self.g7, self.g8, self.g9);
}

MultiVector multi_vector_flat_point_geometric_anti_product(MultiVector self, FlatPoint other) {
    return MultiVector(vec3(self.g3.w) * vec3(other.g0.w) * vec3(0.0, 0.0, -1.0) + vec3(self.g8.x) * vec3(other.g0.x) * vec3(-1.0, 0.0, 0.0) + vec3(self.g8.y) * vec3(other.g0.y) * vec3(-1.0, 0.0, 0.0) + vec3(self.g8.z) * vec3(other.g0.z) * vec3(-1.0, 0.0, 0.0) + vec3(self.g8.w) * vec3(other.g0.w) * vec3(-1.0, 0.0, 0.0) + vec3(self.g2.x) * vec3(other.g0.x, other.g0.w, other.g0.x) * vec3(0.0, -1.0, 0.0), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g4.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g4.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g5.y) * vec3(other.g0.w) * vec3(0.0, 1.0, 0.0) + vec3(self.g5.z) * vec3(other.g0.w) * vec3(0.0, 0.0, 1.0) + vec3(self.g5.x, self.g4.x, self.g4.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, -1.0, 1.0), vec2(self.g9.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g9.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g9.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g9.w) * vec2(other.g0.w) * vec2(0.0, -1.0) + vec2(self.g0.y, self.g0.x) * vec2(other.g0.w, other.g0.x) * vec2(1.0, 0.0), vec4(self.g0.z) * other.g0 + vec4(self.g6.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g6.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g7.y) * vec4(other.g0.w) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g7.z) * vec4(other.g0.w) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g7.x, self.g6.x, self.g6.x, self.g6.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, 0.0), vec3(self.g8.x, self.g8.y, self.g8.z) * vec3(other.g0.w) * vec3(-1.0), vec3(self.g2.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g8.x) * vec3(other.g0.z, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0) + vec3(self.g8.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g8.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + self.g1 * vec3(other.g0.w) * vec3(-1.0), vec3(self.g9.x, self.g9.y, self.g9.z) * vec3(other.g0.w) * vec3(-1.0), vec3(self.g3.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g9.x) * vec3(other.g0.z, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0) + vec3(self.g9.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g9.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g3.x, self.g3.y, self.g3.z) * vec3(other.g0.w) * vec3(-1.0), vec4(self.g4.x) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g4.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g4.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0), vec4(self.g6.x) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g6.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g6.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g2.x, self.g2.x, self.g2.x, self.g2.y) * other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0));
}

AntiScalar multi_vector_flat_point_anti_scalar_product(MultiVector self, FlatPoint other) {
    return AntiScalar(0.0 - self.g3.w * other.g0.w);
}

Dipole multi_vector_dipole_into(MultiVector self) {
    return Dipole(self.g4, self.g5, self.g3);
}

MultiVector multi_vector_dipole_add(MultiVector self, Dipole other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3 + other.g2, self.g4 + other.g0, self.g5 + other.g1, self.g6, self.g7, self.g8, self.g9);
}

MultiVector multi_vector_dipole_sub(MultiVector self, Dipole other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3 - other.g2, self.g4 - other.g0, self.g5 - other.g1, self.g6, self.g7, self.g8, self.g9);
}

MultiVector multi_vector_dipole_geometric_product(MultiVector self, Dipole other) {
    return MultiVector(vec3(self.g4.y) * vec3(other.g1.y) * vec3(0.0, -1.0, 0.0) + vec3(self.g4.z) * vec3(other.g1.z) * vec3(0.0, -1.0, 0.0) + vec3(self.g5.x) * vec3(other.g1.x, other.g0.x, other.g1.x) * vec3(-1.0, -1.0, 0.0) + vec3(self.g5.y) * vec3(other.g1.y, other.g0.y, other.g1.y) * vec3(-1.0, -1.0, 0.0) + vec3(self.g5.z) * vec3(other.g1.z, other.g0.z, other.g1.z) * vec3(-1.0, -1.0, 0.0) + vec3(self.g6.y) * vec3(other.g1.y) * vec3(0.0, 0.0, -1.0) + vec3(self.g6.z) * vec3(other.g1.z) * vec3(0.0, 0.0, -1.0) + vec3(self.g7.x) * vec3(other.g0.x) * vec3(0.0, 0.0, -1.0) + vec3(self.g7.y) * vec3(other.g0.y) * vec3(0.0, 0.0, -1.0) + vec3(self.g7.z) * vec3(other.g0.z) * vec3(0.0, 0.0, -1.0) + vec3(self.g8.x) * vec3(other.g2.x) * vec3(0.0, 0.0, -1.0) + vec3(self.g8.y) * vec3(other.g2.y) * vec3(0.0, 0.0, -1.0) + vec3(self.g8.z) * vec3(other.g2.z) * vec3(0.0, 0.0, -1.0) + vec3(self.g8.w) * vec3(other.g2.w) * vec3(0.0, 0.0, -1.0) + vec3(self.g4.x, self.g4.x, self.g6.x) * vec3(other.g1.x) * vec3(0.0, -1.0, -1.0), vec3(self.g1.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g8.w) * other.g1 + vec3(self.g1.x) * other.g1.xzy * vec3(0.0, 1.0, -1.0), vec2(self.g1.x) * vec2(other.g0.x, other.g2.x) * vec2(-1.0, 1.0) + vec2(self.g1.y) * vec2(other.g0.y, other.g2.y) * vec2(-1.0, 1.0) + vec2(self.g1.z) * vec2(other.g0.z, other.g2.z) * vec2(-1.0, 1.0) + vec2(self.g7.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g7.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g8.x) * vec2(other.g1.x) * vec2(-1.0, 0.0) + vec2(self.g8.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g8.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g7.x) * vec2(other.g1.x) * vec2(0.0, -1.0), vec4(self.g0.x) * other.g2 + vec4(self.g3.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g3.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g4.x) * vec4(other.g2.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g4.y) * vec4(other.g2.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g4.z) * vec4(other.g2.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g5.x) * other.g2.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g5.y) * other.g2.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g5.z) * other.g2.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g9.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g9.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g9.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g9.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g3.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0), vec3(self.g0.x) * other.g0 + vec3(self.g0.y) * other.g1 + vec3(self.g4.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g4.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g5.x) * other.g0.zzy * vec3(0.0, 1.0, -1.0) + vec3(self.g5.y) * other.g0.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g5.z) * other.g0.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g4.x) * other.g1.xzy * vec3(0.0, 1.0, -1.0), vec3(self.g0.x) * other.g1 + vec3(self.g5.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g5.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g5.x) * other.g1.xzy * vec3(0.0, 1.0, -1.0), vec3(self.g0.z) * other.g1 + vec3(self.g2.x) * vec3(other.g2.x, other.g2.y, other.g2.z) + vec3(self.g2.y) * other.g0 + vec3(self.g6.x) * other.g1.zzy * vec3(0.0, 1.0, -1.0) + vec3(self.g6.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g6.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g7.x) * other.g0.zzy * vec3(0.0, 1.0, -1.0) + vec3(self.g7.y) * other.g0.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g7.z) * other.g0.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g8.x) * vec3(other.g2.z, other.g2.z, other.g2.y) * vec3(0.0, 1.0, -1.0) + vec3(self.g8.y) * vec3(other.g2.z, other.g2.z, other.g2.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g8.z) * vec3(other.g2.y, other.g2.x, other.g2.y) * vec3(1.0, -1.0, 0.0) + self.g1 * vec3(other.g2.w) * vec3(-1.0), vec3(self.g1.y) * vec3(other.g2.z, other.g2.z, other.g2.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g2.y, other.g2.x, other.g2.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g2.y) * other.g1 + vec3(self.g7.x) * other.g1.zzy * vec3(0.0, 1.0, -1.0) + vec3(self.g7.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g7.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) - vec3(self.g8.w) * vec3(other.g2.x, other.g2.y, other.g2.z) + vec3(self.g1.x) * vec3(other.g2.x, other.g2.z, other.g2.y) * vec3(0.0, -1.0, 1.0), vec4(self.g1.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g2.x) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g8.x) * vec4(other.g1.z, other.g1.z, other.g1.y, other.g1.z) * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g8.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.z) * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g8.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.y) * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g8.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0), vec4(self.g3.x) * vec4(other.g0.z, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0) + vec4(self.g3.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g3.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g3.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g4.x) * other.g2.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g4.y) * other.g2.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g4.z) * other.g2.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g5.x) * other.g2.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g5.y) * other.g2.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g5.z) * other.g2.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g9.x) * vec4(other.g1.z, other.g1.z, other.g1.y, other.g1.z) * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g9.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.z) * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g9.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.y) * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g9.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.y, self.g0.y, self.g0.y, self.g0.x) * other.g2.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

MultiVector multi_vector_dipole_geometric_anti_product(MultiVector self, Dipole other) {
    return MultiVector(vec3(self.g3.w) * vec3(other.g2.w) * vec3(0.0, 0.0, -1.0) + vec3(self.g6.x) * vec3(other.g1.x, other.g0.x, other.g1.x) * vec3(-1.0, -1.0, 0.0) + vec3(self.g6.y) * vec3(other.g1.y, other.g0.y, other.g1.y) * vec3(-1.0, -1.0, 0.0) + vec3(self.g6.z) * vec3(other.g1.z, other.g0.z, other.g1.z) * vec3(-1.0, -1.0, 0.0) + vec3(self.g7.x) * vec3(other.g0.x) * vec3(-1.0, 0.0, 0.0) + vec3(self.g7.y) * vec3(other.g0.y) * vec3(-1.0, 0.0, 0.0) + vec3(self.g7.z) * vec3(other.g0.z) * vec3(-1.0, 0.0, 0.0) + vec3(self.g8.x) * vec3(other.g2.x) * vec3(-1.0, 0.0, 0.0) + vec3(self.g8.y) * vec3(other.g2.y) * vec3(-1.0, 0.0, 0.0) + vec3(self.g8.z) * vec3(other.g2.z) * vec3(-1.0, 0.0, 0.0) + vec3(self.g8.w) * vec3(other.g2.w) * vec3(-1.0, 0.0, 0.0) + vec3(self.g2.x) * vec3(other.g2.x, other.g2.w, other.g2.x) * vec3(0.0, -1.0, 0.0), vec3(self.g0.y) * vec3(other.g2.x, other.g2.y, other.g2.z) + vec3(self.g3.y) * other.g0.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g3.z) * other.g0.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g3.w) * other.g1 + vec3(self.g4.x) * vec3(other.g2.z, other.g2.z, other.g2.y) * vec3(0.0, -1.0, 1.0) + vec3(self.g4.y) * vec3(other.g2.z, other.g2.z, other.g2.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g4.z) * vec3(other.g2.y, other.g2.x, other.g2.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g5.x) * vec3(other.g2.w) * vec3(1.0, 0.0, 0.0) + vec3(self.g5.y) * vec3(other.g2.w) * vec3(0.0, 1.0, 0.0) + vec3(self.g5.z) * vec3(other.g2.w) * vec3(0.0, 0.0, 1.0) + vec3(self.g9.x) * other.g1.zzy * vec3(0.0, -1.0, 1.0) + vec3(self.g9.y) * other.g1.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g9.z) * other.g1.yxy * vec3(-1.0, 1.0, 0.0) - vec3(self.g9.w) * other.g0 + vec3(self.g3.x) * other.g0.xzy * vec3(0.0, 1.0, -1.0), vec2(self.g9.x) * vec2(other.g0.x, other.g2.x) * vec2(1.0, -1.0) + vec2(self.g9.y) * vec2(other.g0.y, other.g2.y) * vec2(1.0, -1.0) + vec2(self.g9.z) * vec2(other.g0.z, other.g2.z) * vec2(1.0, -1.0) + vec2(self.g9.w) * vec2(other.g2.w) * vec2(0.0, -1.0) + vec2(self.g0.y, self.g0.x) * vec2(other.g2.w, other.g2.x) * vec2(1.0, 0.0), vec4(self.g0.z) * other.g2 + vec4(self.g6.y) * other.g2.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g6.z) * other.g2.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g7.y) * vec4(other.g2.w) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g7.z) * vec4(other.g2.w) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g7.x, self.g6.x, self.g6.x, self.g6.x) * other.g2.wzyx * vec4(1.0, -1.0, 1.0, 0.0), vec3(self.g0.z) * other.g0 + vec3(self.g6.y) * other.g0.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g6.z) * other.g0.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g8.x) * vec3(other.g2.w) * vec3(-1.0, 0.0, 0.0) + vec3(self.g8.y) * vec3(other.g2.w) * vec3(0.0, -1.0, 0.0) + vec3(self.g8.z) * vec3(other.g2.w) * vec3(0.0, 0.0, -1.0) + vec3(self.g6.x) * other.g0.xzy * vec3(0.0, -1.0, 1.0), vec3(self.g0.z) * other.g1 + vec3(self.g2.x) * vec3(other.g2.x, other.g2.y, other.g2.z) + vec3(self.g2.y) * other.g0 + vec3(self.g6.x) * other.g1.zzy * vec3(0.0, -1.0, 1.0) + vec3(self.g6.y) * other.g1.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g6.z) * other.g1.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g7.x) * other.g0.zzy * vec3(0.0, -1.0, 1.0) + vec3(self.g7.y) * other.g0.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g7.z) * other.g0.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g8.x) * vec3(other.g2.z, other.g2.z, other.g2.y) * vec3(0.0, -1.0, 1.0) + vec3(self.g8.y) * vec3(other.g2.z, other.g2.z, other.g2.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g8.z) * vec3(other.g2.y, other.g2.x, other.g2.y) * vec3(-1.0, 1.0, 0.0) + self.g1 * vec3(other.g2.w) * vec3(-1.0), vec3(self.g9.x, self.g9.y, self.g9.z) * vec3(other.g2.w) * vec3(-1.0), vec3(self.g3.w) * vec3(other.g2.x, other.g2.y, other.g2.z) + vec3(self.g9.x) * vec3(other.g2.z, other.g2.z, other.g2.y) * vec3(0.0, -1.0, 1.0) + vec3(self.g9.y) * vec3(other.g2.z, other.g2.z, other.g2.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g9.z) * vec3(other.g2.y, other.g2.x, other.g2.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g3.x, self.g3.y, self.g3.z) * vec3(other.g2.w) * vec3(-1.0), vec4(self.g3.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g3.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g3.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g3.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(-1.0, -1.0, -1.0, 0.0) + vec4(self.g4.x) * other.g2.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g4.y) * other.g2.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g4.z) * other.g2.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g9.x) * vec4(other.g0.z, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0) + vec4(self.g9.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g9.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.x) * other.g2.xxxw * vec4(0.0, 0.0, 0.0, 1.0), vec4(self.g6.x) * other.g2.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g6.y) * other.g2.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g6.z) * other.g2.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g2.x, self.g2.x, self.g2.x, self.g2.y) * other.g2.xxxw * vec4(0.0, 0.0, 0.0, 1.0));
}

Scalar multi_vector_dipole_scalar_product(MultiVector self, Dipole other) {
    return Scalar(0.0 - self.g5.x * other.g1.x - self.g5.y * other.g1.y - self.g5.z * other.g1.z);
}

AntiScalar multi_vector_dipole_anti_scalar_product(MultiVector self, Dipole other) {
    return AntiScalar(0.0 - self.g3.w * other.g2.w);
}

Line multi_vector_line_into(MultiVector self) {
    return Line(self.g6, self.g7);
}

MultiVector multi_vector_line_add(MultiVector self, Line other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3, self.g4, self.g5, self.g6 + other.g0, self.g7 + other.g1, self.g8, self.g9);
}

MultiVector multi_vector_line_sub(MultiVector self, Line other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3, self.g4, self.g5, self.g6 - other.g0, self.g7 - other.g1, self.g8, self.g9);
}

MultiVector multi_vector_line_geometric_anti_product(MultiVector self, Line other) {
    return MultiVector(vec3(self.g4.y) * vec3(other.g1.y, other.g0.y, other.g1.y) * vec3(-1.0, -1.0, 0.0) + vec3(self.g4.z) * vec3(other.g1.z, other.g0.z, other.g1.z) * vec3(-1.0, -1.0, 0.0) + vec3(self.g5.x) * vec3(other.g0.x) * vec3(-1.0, 0.0, 0.0) + vec3(self.g5.y) * vec3(other.g0.y) * vec3(-1.0, 0.0, 0.0) + vec3(self.g5.z) * vec3(other.g0.z) * vec3(-1.0, 0.0, 0.0) + vec3(self.g6.y) * vec3(other.g0.y) * vec3(0.0, 0.0, -1.0) + vec3(self.g6.z) * vec3(other.g0.z) * vec3(0.0, 0.0, -1.0) + vec3(self.g4.x, self.g4.x, self.g6.x) * vec3(other.g1.x, other.g0.x, other.g0.x) * vec3(-1.0), vec3(self.g1.y) * other.g0.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * other.g0.yxy * vec3(-1.0, 1.0, 0.0) - vec3(self.g2.x) * other.g1 + vec3(self.g8.x) * other.g1.zzy * vec3(0.0, 1.0, -1.0) + vec3(self.g8.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g8.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g8.w) * other.g0 + vec3(self.g1.x) * other.g0.xzy * vec3(0.0, -1.0, 1.0), vec2(self.g6.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g6.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g7.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g7.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g7.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g8.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g8.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g8.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g6.x) * vec2(other.g1.x) * vec2(0.0, -1.0), vec4(self.g3.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g0.z) * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g3.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.y) * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g3.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(-1.0, -1.0, -1.0, 0.0) + vec4(self.g9.x) * vec4(other.g1.z, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0) + vec4(self.g9.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g9.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g9.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g3.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g0.x) * vec4(0.0, -1.0, 1.0, 0.0), vec3(self.g0.y) * other.g0 + vec3(self.g4.y) * other.g0.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g4.z) * other.g0.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g4.x) * other.g0.xzy * vec3(0.0, -1.0, 1.0), vec3(self.g0.x) * other.g0 + vec3(self.g0.y) * other.g1 + vec3(self.g4.y) * other.g1.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g4.z) * other.g1.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g5.x) * other.g0.zzy * vec3(0.0, -1.0, 1.0) + vec3(self.g5.y) * other.g0.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g5.z) * other.g0.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g4.x) * other.g1.xzy * vec3(0.0, -1.0, 1.0), vec3(self.g0.z) * other.g0 + vec3(self.g6.y) * other.g0.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g6.z) * other.g0.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g6.x) * other.g0.xzy * vec3(0.0, -1.0, 1.0), vec3(self.g0.z) * other.g1 + vec3(self.g2.y) * other.g0 + vec3(self.g6.y) * other.g1.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g6.z) * other.g1.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g7.x) * other.g0.zzy * vec3(0.0, -1.0, 1.0) + vec3(self.g7.y) * other.g0.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g7.z) * other.g0.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g6.x) * other.g1.xzy * vec3(0.0, -1.0, 1.0), vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g2.x) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g8.x) * vec4(other.g0.z, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, -1.0, 1.0, 1.0) + vec4(self.g8.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g8.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(-1.0, 1.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0), vec4(self.g3.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g3.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g9.x) * vec4(other.g0.z, other.g0.z, other.g0.y, other.g1.x) * vec4(0.0, -1.0, 1.0, 1.0) + vec4(self.g9.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g1.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g9.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g1.z) * vec4(-1.0, 1.0, 0.0, 1.0) + self.g3.wwwx * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, -1.0));
}

AntiScalar multi_vector_line_anti_scalar_product(MultiVector self, Line other) {
    return AntiScalar(0.0 - self.g6.x * other.g0.x - self.g6.y * other.g0.y - self.g6.z * other.g0.z);
}

Circle multi_vector_circle_into(MultiVector self) {
    return Circle(self.g8, self.g6, self.g7);
}

MultiVector multi_vector_circle_add(MultiVector self, Circle other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3, self.g4, self.g5, self.g6 + other.g1, self.g7 + other.g2, self.g8 + other.g0, self.g9);
}

MultiVector multi_vector_circle_sub(MultiVector self, Circle other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3, self.g4, self.g5, self.g6 - other.g1, self.g7 - other.g2, self.g8 - other.g0, self.g9);
}

MultiVector multi_vector_circle_geometric_product(MultiVector self, Circle other) {
    return MultiVector(vec3(self.g1.y) * vec3(other.g0.y) * vec3(0.0, 1.0, 0.0) + vec3(self.g1.z) * vec3(other.g0.z) * vec3(0.0, 1.0, 0.0) + vec3(self.g2.x) * vec3(other.g0.w) * vec3(0.0, 1.0, 0.0) + vec3(self.g3.x) * vec3(other.g0.x) * vec3(0.0, 0.0, -1.0) + vec3(self.g3.y) * vec3(other.g0.y) * vec3(0.0, 0.0, -1.0) + vec3(self.g3.z) * vec3(other.g0.z) * vec3(0.0, 0.0, -1.0) + vec3(self.g3.w) * vec3(other.g0.w) * vec3(0.0, 0.0, -1.0) + vec3(self.g4.x) * vec3(other.g2.x) * vec3(0.0, 0.0, -1.0) + vec3(self.g4.y) * vec3(other.g2.y) * vec3(0.0, 0.0, -1.0) + vec3(self.g4.z) * vec3(other.g2.z) * vec3(0.0, 0.0, -1.0) + vec3(self.g5.x) * vec3(other.g1.x) * vec3(0.0, 0.0, -1.0) + vec3(self.g5.y) * vec3(other.g1.y) * vec3(0.0, 0.0, -1.0) + vec3(self.g5.z) * vec3(other.g1.z) * vec3(0.0, 0.0, -1.0) + vec3(self.g8.w) * vec3(other.g0.w) * vec3(-1.0, 0.0, 0.0) + vec3(self.g1.x) * vec3(other.g0.x) * vec3(0.0, 1.0, 0.0), self.g5 * vec3(other.g0.w), vec2(0.0) - vec2(self.g5.x) * vec2(other.g0.x, other.g2.x) - vec2(self.g5.y) * vec2(other.g0.y, other.g2.y) - vec2(self.g5.z) * vec2(other.g0.z, other.g2.z) + vec2(self.g9.w) * vec2(other.g0.w) * vec2(0.0, 1.0) + vec2(self.g0.y, self.g0.x) * vec2(other.g0.w, other.g0.x) * vec2(-1.0, 0.0), vec4(self.g1.x) * vec4(other.g2.z, other.g2.z, other.g2.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g2.z, other.g2.z, other.g2.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g2.y, other.g2.x, other.g2.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g7.x) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g7.y) * other.g0.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g7.z) * other.g0.wwwz * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g8.x) * vec4(other.g2.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g8.y) * vec4(other.g2.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g8.z) * vec4(other.g2.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g8.w) * vec4(other.g2.x, other.g2.y, other.g2.z, other.g2.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x, self.g0.x, self.g0.x, self.g0.z) * other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0), vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g8.x) * vec3(other.g0.w) * vec3(1.0, 0.0, 0.0) + vec3(self.g8.y) * vec3(other.g0.w) * vec3(0.0, 1.0, 0.0) + vec3(self.g8.z) * vec3(other.g0.w) * vec3(0.0, 0.0, 1.0) - vec3(self.g8.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g1.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0), self.g1 * vec3(other.g0.w) * vec3(-1.0), vec3(self.g0.x) * other.g1 + vec3(self.g0.y) * other.g2 + vec3(self.g3.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g3.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g4.x) * other.g2.zzy * vec3(0.0, 1.0, -1.0) + vec3(self.g4.y) * other.g2.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g4.z) * other.g2.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g5.x) * other.g1.zzy * vec3(0.0, 1.0, -1.0) + vec3(self.g5.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g5.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g9.y) * vec3(other.g0.w) * vec3(0.0, -1.0, 0.0) + vec3(self.g9.z) * vec3(other.g0.w) * vec3(0.0, 0.0, -1.0) + vec3(self.g9.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g9.x, self.g3.x, self.g3.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(-1.0, 1.0, -1.0), vec3(self.g0.x) * other.g2 + vec3(self.g5.x) * other.g2.zzy * vec3(0.0, 1.0, -1.0) + vec3(self.g5.y) * other.g2.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g5.z) * other.g2.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g3.x, self.g3.y, self.g3.z) * vec3(other.g0.w), vec4(self.g0.x) * other.g0 + vec4(self.g5.x) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g5.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g5.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g4.x, self.g4.y, self.g4.z, self.g4.x) * other.g0.wwwx * vec4(-1.0, -1.0, -1.0, 0.0), vec4(self.g1.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g2.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g2.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g2.x) * vec4(other.g2.x, other.g2.y, other.g2.z, other.g2.x) * vec4(1.0, 1.0, 1.0, 0.0) - vec4(self.g2.y) * other.g0 + vec4(self.g6.x) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g6.y) * vec4(other.g0.w) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g6.z) * vec4(other.g0.w) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g7.x) * other.g0.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g7.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g7.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g8.x) * vec4(other.g2.z, other.g2.z, other.g2.y, other.g2.z) * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g8.y) * vec4(other.g2.z, other.g2.z, other.g2.x, other.g2.z) * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g8.z) * vec4(other.g2.y, other.g2.x, other.g2.y, other.g2.y) * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g8.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g2.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

MultiVector multi_vector_circle_geometric_anti_product(MultiVector self, Circle other) {
    return MultiVector(vec3(self.g3.y) * vec3(other.g0.y) * vec3(-1.0, 0.0, 0.0) + vec3(self.g3.z) * vec3(other.g0.z) * vec3(-1.0, 0.0, 0.0) + vec3(self.g3.w) * vec3(other.g0.w) * vec3(-1.0, 0.0, 0.0) + vec3(self.g4.x) * vec3(other.g2.x, other.g1.x, other.g2.x) * vec3(-1.0, -1.0, 0.0) + vec3(self.g4.y) * vec3(other.g2.y, other.g1.y, other.g2.y) * vec3(-1.0, -1.0, 0.0) + vec3(self.g4.z) * vec3(other.g2.z, other.g1.z, other.g2.z) * vec3(-1.0, -1.0, 0.0) + vec3(self.g5.x) * vec3(other.g1.x) * vec3(-1.0, 0.0, 0.0) + vec3(self.g5.y) * vec3(other.g1.y) * vec3(-1.0, 0.0, 0.0) + vec3(self.g5.z) * vec3(other.g1.z) * vec3(-1.0, 0.0, 0.0) + vec3(self.g6.x) * vec3(other.g1.x) * vec3(0.0, 0.0, -1.0) + vec3(self.g6.y) * vec3(other.g1.y) * vec3(0.0, 0.0, -1.0) + vec3(self.g6.z) * vec3(other.g1.z) * vec3(0.0, 0.0, -1.0) + vec3(self.g9.y) * vec3(other.g0.y) * vec3(0.0, -1.0, 0.0) + vec3(self.g9.z) * vec3(other.g0.z) * vec3(0.0, -1.0, 0.0) + vec3(self.g3.x, self.g9.x, self.g3.x) * vec3(other.g0.x) * vec3(-1.0, -1.0, 0.0), vec3(self.g1.y) * other.g1.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * other.g1.yxy * vec3(-1.0, 1.0, 0.0) - vec3(self.g2.x) * other.g2 + vec3(self.g2.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g6.x) * vec3(other.g0.w) * vec3(1.0, 0.0, 0.0) + vec3(self.g6.y) * vec3(other.g0.w) * vec3(0.0, 1.0, 0.0) + vec3(self.g6.z) * vec3(other.g0.w) * vec3(0.0, 0.0, 1.0) + vec3(self.g7.x) * vec3(other.g0.z, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0) + vec3(self.g7.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g7.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g8.x) * other.g2.zzy * vec3(0.0, 1.0, -1.0) + vec3(self.g8.y) * other.g2.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g8.z) * other.g2.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g8.w) * other.g1 + vec3(self.g1.x) * other.g1.xzy * vec3(0.0, -1.0, 1.0), vec2(0.0) - vec2(self.g6.x) * vec2(other.g0.x, other.g2.x) - vec2(self.g6.y) * vec2(other.g0.y, other.g2.y) - vec2(self.g6.z) * vec2(other.g0.z, other.g2.z) + vec2(self.g7.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g7.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g8.x) * vec2(other.g1.x) * vec2(-1.0, 0.0) + vec2(self.g8.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g8.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g7.x) * vec2(other.g1.x) * vec2(0.0, -1.0), vec4(self.g3.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.z) * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g3.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.y) * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g3.w) * vec4(other.g2.x, other.g2.y, other.g2.z, other.g2.x) * vec4(-1.0, -1.0, -1.0, 0.0) + vec4(self.g9.x) * vec4(other.g2.z, other.g2.z, other.g2.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0) + vec4(self.g9.y) * vec4(other.g2.z, other.g2.z, other.g2.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g9.z) * vec4(other.g2.y, other.g2.x, other.g2.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g9.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g3.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g1.x) * vec4(0.0, -1.0, 1.0, 0.0), vec3(self.g0.y) * other.g1 + vec3(self.g3.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g4.y) * other.g1.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g4.z) * other.g1.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g9.x) * vec3(other.g0.z, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0) + vec3(self.g9.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g9.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g4.x) * other.g1.xzy * vec3(0.0, -1.0, 1.0), vec3(self.g0.x) * other.g1 + vec3(self.g0.y) * other.g2 + vec3(self.g3.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g3.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g4.x) * other.g2.zzy * vec3(0.0, -1.0, 1.0) + vec3(self.g4.y) * other.g2.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g4.z) * other.g2.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g5.x) * other.g1.zzy * vec3(0.0, -1.0, 1.0) + vec3(self.g5.y) * other.g1.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g5.z) * other.g1.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g9.y) * vec3(other.g0.w) * vec3(0.0, -1.0, 0.0) + vec3(self.g9.z) * vec3(other.g0.w) * vec3(0.0, 0.0, -1.0) + vec3(self.g9.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g9.x, self.g3.x, self.g3.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(-1.0, -1.0, 1.0), vec3(self.g0.z) * other.g1 + vec3(self.g6.y) * other.g1.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g6.z) * other.g1.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g6.x) * other.g1.xzy * vec3(0.0, -1.0, 1.0), vec3(self.g0.z) * other.g2 + vec3(self.g2.y) * other.g1 + vec3(self.g6.y) * other.g2.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g6.z) * other.g2.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g7.x) * other.g1.zzy * vec3(0.0, -1.0, 1.0) + vec3(self.g7.y) * other.g1.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g7.z) * other.g1.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g6.x) * other.g2.xzy * vec3(0.0, -1.0, 1.0), vec4(self.g0.z) * other.g0 + vec4(self.g1.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g2.x) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g6.x) * other.g0.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g6.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g6.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g7.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g7.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g7.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g8.x) * vec4(other.g1.z, other.g1.z, other.g1.y, other.g2.x) * vec4(0.0, -1.0, 1.0, 1.0) + vec4(self.g8.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g2.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g8.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g2.z) * vec4(-1.0, 1.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0), vec4(self.g3.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g3.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g9.x) * vec4(other.g1.z, other.g1.z, other.g1.y, other.g2.x) * vec4(0.0, -1.0, 1.0, 1.0) + vec4(self.g9.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g2.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g9.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g2.z) * vec4(-1.0, 1.0, 0.0, 1.0) + self.g3.wwwx * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, -1.0));
}

Scalar multi_vector_circle_scalar_product(MultiVector self, Circle other) {
    return Scalar(0.0 - self.g8.w * other.g0.w);
}

AntiScalar multi_vector_circle_anti_scalar_product(MultiVector self, Circle other) {
    return AntiScalar(0.0 - self.g6.x * other.g1.x - self.g6.y * other.g1.y - self.g6.z * other.g1.z);
}

Plane multi_vector_plane_into(MultiVector self) {
    return Plane(self.g9);
}

MultiVector multi_vector_plane_add(MultiVector self, Plane other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3, self.g4, self.g5, self.g6, self.g7, self.g8, self.g9 + other.g0);
}

MultiVector multi_vector_plane_sub(MultiVector self, Plane other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3, self.g4, self.g5, self.g6, self.g7, self.g8, self.g9 - other.g0);
}

MultiVector multi_vector_plane_geometric_anti_product(MultiVector self, Plane other) {
    return MultiVector(vec3(self.g1.y) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g1.z) * vec3(other.g0.z) * vec3(1.0, 0.0, 0.0) + vec3(self.g2.x) * vec3(other.g0.w) * vec3(1.0, 0.0, 0.0) + vec3(self.g8.x) * vec3(other.g0.x) * vec3(0.0, 1.0, 0.0) + vec3(self.g8.y) * vec3(other.g0.y) * vec3(0.0, 1.0, 0.0) + vec3(self.g8.z) * vec3(other.g0.z) * vec3(0.0, 1.0, 0.0) + vec3(self.g9.x) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0) + vec3(self.g9.y) * vec3(other.g0.y) * vec3(0.0, 0.0, 1.0) + vec3(self.g9.z) * vec3(other.g0.z) * vec3(0.0, 0.0, 1.0) + vec3(self.g1.x) * vec3(other.g0.x) * vec3(1.0, 0.0, 0.0), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g5.x) * vec3(other.g0.z, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0) + vec3(self.g5.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g5.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + self.g4 * vec3(other.g0.w), vec2(self.g3.y) * vec2(other.g0.y) * vec2(0.0, 1.0) + vec2(self.g3.z) * vec2(other.g0.z) * vec2(0.0, 1.0) + vec2(self.g3.w) * vec2(other.g0.w) * vec2(0.0, 1.0) + vec2(self.g4.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g4.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g4.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g3.x) * vec2(other.g0.x) * vec2(0.0, 1.0), vec4(self.g6.x) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g6.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g6.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g7.x) * other.g0.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g7.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g7.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g2.y, self.g2.y, self.g2.y, self.g2.x) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0), vec3(0.0) - vec3(self.g2.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g8.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g8.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g8.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0), vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g8.x) * vec3(other.g0.w) * vec3(1.0, 0.0, 0.0) + vec3(self.g8.y) * vec3(other.g0.w) * vec3(0.0, 1.0, 0.0) + vec3(self.g8.z) * vec3(other.g0.w) * vec3(0.0, 0.0, 1.0) - vec3(self.g8.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g1.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g3.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g9.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g9.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g9.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0), vec3(self.g3.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g3.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g9.y) * vec3(other.g0.w) * vec3(0.0, 1.0, 0.0) + vec3(self.g9.z) * vec3(other.g0.w) * vec3(0.0, 0.0, 1.0) - vec3(self.g9.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g9.x, self.g3.x, self.g3.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, 1.0, -1.0), vec4(self.g0.y) * other.g0 + vec4(self.g4.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g4.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g5.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g5.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.x, self.g4.x, self.g4.x, self.g5.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, -1.0), vec4(self.g0.z) * other.g0 + vec4(self.g6.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g6.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g7.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g7.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g6.x, self.g6.x, self.g6.x, self.g7.x) * other.g0.xzyx * vec4(0.0, -1.0, 1.0, -1.0));
}

AntiScalar multi_vector_plane_anti_scalar_product(MultiVector self, Plane other) {
    return AntiScalar(self.g9.x * other.g0.x + self.g9.y * other.g0.y + self.g9.z * other.g0.z);
}

Sphere multi_vector_sphere_into(MultiVector self) {
    return Sphere(vec2(self.g0.y, self.g9.w), vec3(self.g9.x, self.g9.y, self.g9.z));
}

MultiVector multi_vector_sphere_add(MultiVector self, Sphere other) {
    return MultiVector(self.g0 + vec3(other.g0.x) * vec3(0.0, 1.0, 0.0), self.g1, self.g2, self.g3, self.g4, self.g5, self.g6, self.g7, self.g8, self.g9 + vec4(other.g1.x, other.g1.y, other.g1.z, other.g0.y));
}

MultiVector multi_vector_sphere_sub(MultiVector self, Sphere other) {
    return MultiVector(self.g0 - vec3(other.g0.x) * vec3(0.0, 1.0, 0.0), self.g1, self.g2, self.g3, self.g4, self.g5, self.g6, self.g7, self.g8, self.g9 - vec4(other.g1.x, other.g1.y, other.g1.z, other.g0.y));
}

MultiVector multi_vector_sphere_geometric_anti_product(MultiVector self, Sphere other) {
    return MultiVector(vec3(self.g1.x) * vec3(other.g1.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g1.y) * vec3(other.g1.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g1.z) * vec3(other.g1.z) * vec3(1.0, 0.0, 0.0) + vec3(self.g2.x) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g2.y) * vec3(other.g0.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g8.x) * vec3(other.g1.x) * vec3(0.0, 1.0, 0.0) + vec3(self.g8.y) * vec3(other.g1.y) * vec3(0.0, 1.0, 0.0) + vec3(self.g8.z) * vec3(other.g1.z) * vec3(0.0, 1.0, 0.0) + vec3(self.g9.x) * vec3(other.g1.x) * vec3(0.0, 0.0, 1.0) + vec3(self.g9.y) * vec3(other.g1.y) * vec3(0.0, 0.0, 1.0) + vec3(self.g9.z) * vec3(other.g1.z) * vec3(0.0, 0.0, 1.0) + self.g0.xzx * vec3(other.g0.x) * vec3(0.0, 1.0, 0.0), vec3(self.g0.x) * other.g1 + vec3(self.g4.x) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g4.y) * vec3(other.g0.y) * vec3(0.0, 1.0, 0.0) + vec3(self.g4.z) * vec3(other.g0.y) * vec3(0.0, 0.0, 1.0) + vec3(self.g5.x) * other.g1.zzy * vec3(0.0, -1.0, 1.0) + vec3(self.g5.y) * other.g1.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g5.z) * other.g1.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g3.x, self.g3.y, self.g3.z) * vec3(other.g0.x) * vec3(-1.0), vec2(self.g3.y) * vec2(other.g1.y) * vec2(0.0, 1.0) + vec2(self.g3.z) * vec2(other.g1.z) * vec2(0.0, 1.0) + vec2(self.g3.w) * other.g0 * vec2(-1.0, 1.0) + vec2(self.g4.x) * vec2(other.g1.x) * vec2(-1.0, 0.0) + vec2(self.g4.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g4.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g3.x) * vec2(other.g1.x) * vec2(0.0, 1.0), vec4(self.g6.x) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g1.x) * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g6.y) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g1.y) * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g6.z) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g1.z) * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g7.x) * vec4(other.g1.z, other.g1.z, other.g1.y, other.g1.z) * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g7.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.z) * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g7.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.y) * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g2.y, self.g2.y, self.g2.y, self.g2.x) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0), vec3(0.0) - vec3(self.g2.x) * other.g1 + vec3(self.g8.x) * other.g1.zzy * vec3(0.0, 1.0, -1.0) + vec3(self.g8.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g8.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + self.g6 * vec3(other.g0.x), vec3(self.g1.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g7.x) * vec3(other.g0.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g7.y) * vec3(other.g0.x) * vec3(0.0, 1.0, 0.0) + vec3(self.g7.z) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0) + vec3(self.g8.x) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g8.y) * vec3(other.g0.y) * vec3(0.0, 1.0, 0.0) + vec3(self.g8.z) * vec3(other.g0.y) * vec3(0.0, 0.0, 1.0) - vec3(self.g8.w) * other.g1 + vec3(self.g1.x) * other.g1.xzy * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g3.w) * other.g1 + vec3(self.g9.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g9.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g9.x) * other.g1.xzy * vec3(0.0, 1.0, -1.0), vec3(self.g3.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g3.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g9.x) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g9.y) * vec3(other.g0.y) * vec3(0.0, 1.0, 0.0) + vec3(self.g9.z) * vec3(other.g0.y) * vec3(0.0, 0.0, 1.0) - vec3(self.g9.w) * other.g1 + vec3(self.g3.x) * other.g1.xzy * vec3(0.0, 1.0, -1.0), vec4(self.g0.y) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g0.y) + vec4(self.g4.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.z) * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g4.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.y) * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g5.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g5.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g9.x) * vec4(other.g0.x) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g9.y) * vec4(other.g0.x) * vec4(0.0, -1.0, 0.0, 0.0) + vec4(self.g9.z) * vec4(other.g0.x) * vec4(0.0, 0.0, -1.0, 0.0) + vec4(self.g9.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g4.x, self.g4.x, self.g4.x, self.g5.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g1.x) * vec4(0.0, -1.0, 1.0, -1.0), vec4(self.g0.z) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g0.y) + vec4(self.g6.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.z) * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g6.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.y) * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g7.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g7.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g6.x, self.g6.x, self.g6.x, self.g7.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g1.x) * vec4(0.0, -1.0, 1.0, -1.0));
}

AntiScalar multi_vector_sphere_anti_scalar_product(MultiVector self, Sphere other) {
    return AntiScalar(self.g9.x * other.g1.x + self.g9.y * other.g1.y + self.g9.z * other.g1.z);
}

Motor multi_vector_motor_into(MultiVector self) {
    return Motor(vec4(self.g6.x, self.g6.y, self.g6.z, self.g0.z), vec4(self.g7.x, self.g7.y, self.g7.z, self.g2.y));
}

MultiVector multi_vector_motor_add(MultiVector self, Motor other) {
    return MultiVector(self.g0 + vec3(other.g0.x, other.g0.x, other.g0.w) * vec3(0.0, 0.0, 1.0), self.g1, self.g2 + vec2(other.g0.x, other.g1.w) * vec2(0.0, 1.0), self.g3, self.g4, self.g5, self.g6 + vec3(other.g0.x, other.g0.y, other.g0.z), self.g7 + vec3(other.g1.x, other.g1.y, other.g1.z), self.g8, self.g9);
}

MultiVector multi_vector_motor_sub(MultiVector self, Motor other) {
    return MultiVector(self.g0 - vec3(other.g0.x, other.g0.x, other.g0.w) * vec3(0.0, 0.0, 1.0), self.g1, self.g2 - vec2(other.g0.x, other.g1.w) * vec2(0.0, 1.0), self.g3, self.g4, self.g5, self.g6 - vec3(other.g0.x, other.g0.y, other.g0.z), self.g7 - vec3(other.g1.x, other.g1.y, other.g1.z), self.g8, self.g9);
}

MultiVector multi_vector_motor_regressive_product(MultiVector self, Motor other) {
    return MultiVector(vec3(self.g0.y) * vec3(other.g1.w, other.g0.w, other.g1.w) * vec3(1.0, 1.0, 0.0) + vec3(self.g4.x) * vec3(other.g1.x) * vec3(-1.0, 0.0, 0.0) + vec3(self.g4.y) * vec3(other.g1.y) * vec3(-1.0, 0.0, 0.0) + vec3(self.g4.z) * vec3(other.g1.z) * vec3(-1.0, 0.0, 0.0) + vec3(self.g5.x) * vec3(other.g0.x) * vec3(-1.0, 0.0, 0.0) + vec3(self.g5.y) * vec3(other.g0.y) * vec3(-1.0, 0.0, 0.0) + vec3(self.g5.z) * vec3(other.g0.z) * vec3(-1.0, 0.0, 0.0) + self.g0.xxz * vec3(other.g0.w, other.g0.x, other.g0.w) * vec3(1.0, 0.0, 1.0), vec3(self.g8.x) * vec3(other.g1.z, other.g1.z, other.g1.y) * vec3(0.0, 1.0, -1.0) + vec3(self.g8.y) * vec3(other.g1.z, other.g1.z, other.g1.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g8.z) * vec3(other.g1.y, other.g1.x, other.g1.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g8.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g1 * vec3(other.g0.w), vec2(self.g2.x) * vec2(other.g0.w) * vec2(1.0, 0.0) + vec2(self.g2.y) * vec2(other.g0.w) * vec2(0.0, 1.0) + vec2(self.g6.x) * vec2(other.g1.x) * vec2(0.0, -1.0) + vec2(self.g6.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g6.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g7.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g7.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g7.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g8.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g8.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g8.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x, self.g0.z) * vec2(other.g1.x, other.g1.w) * vec2(0.0, 1.0), vec4(self.g9.x) * vec4(other.g1.z, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0) + vec4(self.g9.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g9.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g9.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + self.g3 * vec4(other.g0.w), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g4 * vec3(other.g0.w), vec3(self.g0.y) * vec3(other.g1.x, other.g1.y, other.g1.z) + self.g5 * vec3(other.g0.w), vec3(self.g0.z) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g6 * vec3(other.g0.w), vec3(self.g0.z) * vec3(other.g1.x, other.g1.y, other.g1.z) + self.g7 * vec3(other.g0.w), self.g8 * vec4(other.g0.w), self.g9 * vec4(other.g0.w));
}

MultiVector multi_vector_motor_geometric_anti_product(MultiVector self, Motor other) {
    return MultiVector(vec3(self.g0.y) * vec3(other.g1.w, other.g0.w, other.g1.w) * vec3(1.0, 1.0, 0.0) + vec3(self.g4.x) * vec3(other.g1.x, other.g0.x, other.g1.x) * vec3(-1.0, -1.0, 0.0) + vec3(self.g4.y) * vec3(other.g1.y, other.g0.y, other.g1.y) * vec3(-1.0, -1.0, 0.0) + vec3(self.g4.z) * vec3(other.g1.z, other.g0.z, other.g1.z) * vec3(-1.0, -1.0, 0.0) + vec3(self.g5.x) * vec3(other.g0.x) * vec3(-1.0, 0.0, 0.0) + vec3(self.g5.y) * vec3(other.g0.y) * vec3(-1.0, 0.0, 0.0) + vec3(self.g5.z) * vec3(other.g0.z) * vec3(-1.0, 0.0, 0.0) + vec3(self.g6.x) * vec3(other.g0.x) * vec3(0.0, 0.0, -1.0) + vec3(self.g6.y) * vec3(other.g0.y) * vec3(0.0, 0.0, -1.0) + vec3(self.g6.z) * vec3(other.g0.z) * vec3(0.0, 0.0, -1.0) + self.g0.xxz * vec3(other.g0.w, other.g0.x, other.g0.w) * vec3(1.0, 0.0, 1.0), vec3(self.g1.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g0.z, other.g0.w, other.g0.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.w) * vec3(-1.0, 1.0, 1.0) - vec3(self.g2.x) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g8.x) * vec3(other.g1.w, other.g1.z, other.g1.y) * vec3(-1.0, 1.0, -1.0) + vec3(self.g8.y) * vec3(other.g1.z, other.g1.w, other.g1.x) * vec3(-1.0, -1.0, 1.0) + vec3(self.g8.z) * vec3(other.g1.y, other.g1.x, other.g1.w) * vec3(1.0, -1.0, -1.0) + vec3(self.g8.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g2.x) * vec2(other.g0.w) * vec2(1.0, 0.0) + vec2(self.g2.y) * vec2(other.g0.w) * vec2(0.0, 1.0) + vec2(self.g6.x) * vec2(other.g1.x) * vec2(0.0, -1.0) + vec2(self.g6.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g6.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g7.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g7.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g7.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g8.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g8.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g8.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g0.x, self.g0.z) * vec2(other.g1.x, other.g1.w) * vec2(0.0, 1.0), vec4(self.g3.y) * other.g0.zwxz * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g3.z) * other.g0.yxwy * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g3.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g0.w) * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g9.x) * vec4(other.g1.w, other.g1.z, other.g1.y, other.g0.x) * vec4(-1.0, 1.0, -1.0, -1.0) + vec4(self.g9.y) * vec4(other.g1.z, other.g1.w, other.g1.x, other.g0.y) * vec4(-1.0, -1.0, 1.0, -1.0) + vec4(self.g9.z) * vec4(other.g1.y, other.g1.x, other.g1.w, other.g0.z) * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g9.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g3.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, 0.0), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g4.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g4.y) * vec3(other.g0.z, other.g0.w, other.g0.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g4.z) * vec3(other.g0.y, other.g0.x, other.g0.w) * vec3(-1.0, 1.0, 1.0), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g0.y) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g4.x) * vec3(other.g1.w, other.g1.z, other.g1.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g4.y) * vec3(other.g1.z, other.g1.w, other.g1.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g4.z) * vec3(other.g1.y, other.g1.x, other.g1.w) * vec3(-1.0, 1.0, 1.0) + vec3(self.g5.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g5.y) * vec3(other.g0.z, other.g0.w, other.g0.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g5.z) * vec3(other.g0.y, other.g0.x, other.g0.w) * vec3(-1.0, 1.0, 1.0), vec3(self.g0.z) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g6.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g6.y) * vec3(other.g0.z, other.g0.w, other.g0.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g6.z) * vec3(other.g0.y, other.g0.x, other.g0.w) * vec3(-1.0, 1.0, 1.0), vec3(self.g0.z) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g2.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g6.x) * vec3(other.g1.w, other.g1.z, other.g1.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g6.y) * vec3(other.g1.z, other.g1.w, other.g1.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g6.z) * vec3(other.g1.y, other.g1.x, other.g1.w) * vec3(-1.0, 1.0, 1.0) + vec3(self.g7.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g7.y) * vec3(other.g0.z, other.g0.w, other.g0.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g7.z) * vec3(other.g0.y, other.g0.x, other.g0.w) * vec3(-1.0, 1.0, 1.0), vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g2.x) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.w) * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g8.x) * vec4(other.g0.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g8.y) * vec4(other.g0.z, other.g0.w, other.g0.x, other.g1.y) * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g8.z) * vec4(other.g0.y, other.g0.x, other.g0.w, other.g1.z) * vec4(-1.0, 1.0, 1.0, 1.0) + vec4(self.g8.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0), vec4(self.g3.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g3.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g3.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.w) * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g9.x) * vec4(other.g0.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g9.y) * vec4(other.g0.z, other.g0.w, other.g0.x, other.g1.y) * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g9.z) * vec4(other.g0.y, other.g0.x, other.g0.w, other.g1.z) * vec4(-1.0, 1.0, 1.0, 1.0) + vec4(self.g9.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g3.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

MultiVector multi_vector_motor_inner_anti_product(MultiVector self, Motor other) {
    return MultiVector(vec3(self.g4.x) * vec3(other.g0.x) * vec3(0.0, -1.0, 0.0) + vec3(self.g4.y) * vec3(other.g0.y) * vec3(0.0, -1.0, 0.0) + vec3(self.g4.z) * vec3(other.g0.z) * vec3(0.0, -1.0, 0.0) + vec3(self.g6.x) * vec3(other.g0.x) * vec3(0.0, 0.0, -1.0) + vec3(self.g6.y) * vec3(other.g0.y) * vec3(0.0, 0.0, -1.0) + vec3(self.g6.z) * vec3(other.g0.z) * vec3(0.0, 0.0, -1.0) + self.g0 * vec3(other.g0.w), self.g1 * vec3(other.g0.w), vec2(self.g2.x) * vec2(other.g0.w) * vec2(1.0, 0.0) + vec2(self.g2.y) * vec2(other.g0.w) * vec2(0.0, 1.0) + vec2(self.g0.x, self.g0.z) * vec2(other.g1.x, other.g1.w) * vec2(0.0, 1.0), vec4(self.g9.x) * vec4(other.g1.w) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g9.y) * vec4(other.g1.w) * vec4(0.0, -1.0, 0.0, 0.0) + vec4(self.g9.z) * vec4(other.g1.w) * vec4(0.0, 0.0, -1.0, 0.0) + self.g3 * vec4(other.g0.w), self.g4 * vec3(other.g0.w), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g5 * vec3(other.g0.w), vec3(self.g0.z) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g6 * vec3(other.g0.w), vec3(self.g0.z) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g2.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g7.x) * vec3(other.g0.w) * vec3(1.0, 0.0, 0.0) + vec3(self.g7.y) * vec3(other.g0.w) * vec3(0.0, 1.0, 0.0) + vec3(self.g7.z) * vec3(other.g0.w) * vec3(0.0, 0.0, 1.0) + self.g6 * vec3(other.g1.w), vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g2.x) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g8.x) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g8.y) * vec4(other.g0.w) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g8.z) * vec4(other.g0.w) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g8.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0), vec4(self.g3.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g3.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g3.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.w) * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g9.x) * vec4(other.g0.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g9.y) * vec4(other.g0.z, other.g0.w, other.g0.x, other.g1.y) * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g9.z) * vec4(other.g0.y, other.g0.x, other.g0.w, other.g1.z) * vec4(-1.0, 1.0, 1.0, 1.0) + vec4(self.g9.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g3.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0));
}

MultiVector multi_vector_motor_right_anti_contraction(MultiVector self, Motor other) {
    return MultiVector(vec3(self.g4.x) * vec3(other.g0.x) * vec3(0.0, -1.0, 0.0) + vec3(self.g4.y) * vec3(other.g0.y) * vec3(0.0, -1.0, 0.0) + vec3(self.g4.z) * vec3(other.g0.z) * vec3(0.0, -1.0, 0.0) + vec3(self.g6.x) * vec3(other.g0.x) * vec3(0.0, 0.0, -1.0) + vec3(self.g6.y) * vec3(other.g0.y) * vec3(0.0, 0.0, -1.0) + vec3(self.g6.z) * vec3(other.g0.z) * vec3(0.0, 0.0, -1.0) + self.g0 * vec3(other.g0.w), self.g1 * vec3(other.g0.w), self.g2 * vec2(other.g0.w), self.g3 * vec4(other.g0.w), self.g4 * vec3(other.g0.w), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g5 * vec3(other.g0.w), self.g6 * vec3(other.g0.w), vec3(self.g2.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g7 * vec3(other.g0.w), vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g2.x) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g8.x) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g8.y) * vec4(other.g0.w) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g8.z) * vec4(other.g0.w) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g8.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0), vec4(self.g3.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g3.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g9.x) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g9.y) * vec4(other.g0.w) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g9.z) * vec4(other.g0.w) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g9.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + self.g3.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

AntiScalar multi_vector_motor_anti_scalar_product(MultiVector self, Motor other) {
    return AntiScalar(self.g0.z * other.g0.w - self.g6.x * other.g0.x - self.g6.y * other.g0.y - self.g6.z * other.g0.z);
}

Rotor multi_vector_rotor_into(MultiVector self) {
    return Rotor(vec4(self.g6.x, self.g6.y, self.g6.z, self.g0.z));
}

MultiVector multi_vector_rotor_add(MultiVector self, Rotor other) {
    return MultiVector(self.g0 + vec3(other.g0.x, other.g0.x, other.g0.w) * vec3(0.0, 0.0, 1.0), self.g1, self.g2, self.g3, self.g4, self.g5, self.g6 + vec3(other.g0.x, other.g0.y, other.g0.z), self.g7, self.g8, self.g9);
}

MultiVector multi_vector_rotor_sub(MultiVector self, Rotor other) {
    return MultiVector(self.g0 - vec3(other.g0.x, other.g0.x, other.g0.w) * vec3(0.0, 0.0, 1.0), self.g1, self.g2, self.g3, self.g4, self.g5, self.g6 - vec3(other.g0.x, other.g0.y, other.g0.z), self.g7, self.g8, self.g9);
}

MultiVector multi_vector_rotor_regressive_product(MultiVector self, Rotor other) {
    return MultiVector(vec3(self.g5.x) * vec3(other.g0.x) * vec3(-1.0, 0.0, 0.0) + vec3(self.g5.y) * vec3(other.g0.y) * vec3(-1.0, 0.0, 0.0) + vec3(self.g5.z) * vec3(other.g0.z) * vec3(-1.0, 0.0, 0.0) + self.g0 * vec3(other.g0.w), vec3(self.g8.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g1 * vec3(other.g0.w), vec2(self.g7.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g7.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g7.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g8.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g8.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g8.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + self.g2 * vec2(other.g0.w), vec4(self.g9.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g9.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g9.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g9.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + self.g3 * vec4(other.g0.w), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g4 * vec3(other.g0.w), self.g5 * vec3(other.g0.w), vec3(self.g0.z) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g6 * vec3(other.g0.w), self.g7 * vec3(other.g0.w), self.g8 * vec4(other.g0.w), self.g9 * vec4(other.g0.w));
}

MultiVector multi_vector_rotor_geometric_anti_product(MultiVector self, Rotor other) {
    return MultiVector(vec3(self.g4.x) * vec3(other.g0.x) * vec3(0.0, -1.0, 0.0) + vec3(self.g4.y) * vec3(other.g0.y) * vec3(0.0, -1.0, 0.0) + vec3(self.g4.z) * vec3(other.g0.z) * vec3(0.0, -1.0, 0.0) + vec3(self.g5.x) * vec3(other.g0.x) * vec3(-1.0, 0.0, 0.0) + vec3(self.g5.y) * vec3(other.g0.y) * vec3(-1.0, 0.0, 0.0) + vec3(self.g5.z) * vec3(other.g0.z) * vec3(-1.0, 0.0, 0.0) + vec3(self.g6.x) * vec3(other.g0.x) * vec3(0.0, 0.0, -1.0) + vec3(self.g6.y) * vec3(other.g0.y) * vec3(0.0, 0.0, -1.0) + vec3(self.g6.z) * vec3(other.g0.z) * vec3(0.0, 0.0, -1.0) + self.g0 * vec3(other.g0.w), vec3(self.g1.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g0.z, other.g0.w, other.g0.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.w) * vec3(-1.0, 1.0, 1.0) + vec3(self.g8.w) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g7.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g7.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g7.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g8.x) * vec2(other.g0.x) * vec2(-1.0, 0.0) + vec2(self.g8.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g8.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + self.g2 * vec2(other.g0.w), vec4(self.g3.y) * other.g0.zwxz * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g3.z) * other.g0.yxwy * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g9.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g9.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g9.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g9.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + self.g3.xxxw * other.g0.wzyw * vec4(1.0, -1.0, 1.0, 1.0), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g4.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g4.y) * vec3(other.g0.z, other.g0.w, other.g0.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g4.z) * vec3(other.g0.y, other.g0.x, other.g0.w) * vec3(-1.0, 1.0, 1.0), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g5.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g5.y) * vec3(other.g0.z, other.g0.w, other.g0.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g5.z) * vec3(other.g0.y, other.g0.x, other.g0.w) * vec3(-1.0, 1.0, 1.0), vec3(self.g0.z) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g6.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g6.y) * vec3(other.g0.z, other.g0.w, other.g0.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g6.z) * vec3(other.g0.y, other.g0.x, other.g0.w) * vec3(-1.0, 1.0, 1.0), vec3(self.g2.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g7.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g7.y) * vec3(other.g0.z, other.g0.w, other.g0.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g7.z) * vec3(other.g0.y, other.g0.x, other.g0.w) * vec3(-1.0, 1.0, 1.0), vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g2.x) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g8.x) * other.g0.wzyw * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g8.y) * other.g0.zwxz * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g8.z) * other.g0.yxwy * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g8.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0), vec4(self.g3.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g3.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g9.x) * other.g0.wzyw * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g9.y) * other.g0.zwxz * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g9.z) * other.g0.yxwy * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g9.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + self.g3.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

MultiVector multi_vector_rotor_inner_anti_product(MultiVector self, Rotor other) {
    return MultiVector(vec3(self.g4.x) * vec3(other.g0.x) * vec3(0.0, -1.0, 0.0) + vec3(self.g4.y) * vec3(other.g0.y) * vec3(0.0, -1.0, 0.0) + vec3(self.g4.z) * vec3(other.g0.z) * vec3(0.0, -1.0, 0.0) + vec3(self.g6.x) * vec3(other.g0.x) * vec3(0.0, 0.0, -1.0) + vec3(self.g6.y) * vec3(other.g0.y) * vec3(0.0, 0.0, -1.0) + vec3(self.g6.z) * vec3(other.g0.z) * vec3(0.0, 0.0, -1.0) + self.g0 * vec3(other.g0.w), self.g1 * vec3(other.g0.w), self.g2 * vec2(other.g0.w), self.g3 * vec4(other.g0.w), self.g4 * vec3(other.g0.w), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g5 * vec3(other.g0.w), vec3(self.g0.z) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g6 * vec3(other.g0.w), vec3(self.g2.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g7 * vec3(other.g0.w), vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g2.x) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g8.x) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g8.y) * vec4(other.g0.w) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g8.z) * vec4(other.g0.w) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g8.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0), vec4(self.g3.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g3.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g9.x) * other.g0.wzyw * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g9.y) * other.g0.zwxz * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g9.z) * other.g0.yxwy * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g9.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + self.g3.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

MultiVector multi_vector_rotor_right_anti_contraction(MultiVector self, Rotor other) {
    return MultiVector(vec3(self.g4.x) * vec3(other.g0.x) * vec3(0.0, -1.0, 0.0) + vec3(self.g4.y) * vec3(other.g0.y) * vec3(0.0, -1.0, 0.0) + vec3(self.g4.z) * vec3(other.g0.z) * vec3(0.0, -1.0, 0.0) + vec3(self.g6.x) * vec3(other.g0.x) * vec3(0.0, 0.0, -1.0) + vec3(self.g6.y) * vec3(other.g0.y) * vec3(0.0, 0.0, -1.0) + vec3(self.g6.z) * vec3(other.g0.z) * vec3(0.0, 0.0, -1.0) + self.g0 * vec3(other.g0.w), self.g1 * vec3(other.g0.w), self.g2 * vec2(other.g0.w), self.g3 * vec4(other.g0.w), self.g4 * vec3(other.g0.w), vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g5 * vec3(other.g0.w), self.g6 * vec3(other.g0.w), vec3(self.g2.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g7 * vec3(other.g0.w), vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g2.x) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g8.x) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g8.y) * vec4(other.g0.w) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g8.z) * vec4(other.g0.w) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g8.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0), vec4(self.g3.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g3.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g9.x) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g9.y) * vec4(other.g0.w) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g9.z) * vec4(other.g0.w) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g9.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + self.g3.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

AntiScalar multi_vector_rotor_anti_scalar_product(MultiVector self, Rotor other) {
    return AntiScalar(self.g0.z * other.g0.w - self.g6.x * other.g0.x - self.g6.y * other.g0.y - self.g6.z * other.g0.z);
}

Translator multi_vector_translator_into(MultiVector self) {
    return Translator(vec4(self.g7.x, self.g7.y, self.g7.z, self.g0.z));
}

MultiVector multi_vector_translator_add(MultiVector self, Translator other) {
    return MultiVector(self.g0 + vec3(other.g0.x, other.g0.x, other.g0.w) * vec3(0.0, 0.0, 1.0), self.g1, self.g2, self.g3, self.g4, self.g5, self.g6, self.g7 + vec3(other.g0.x, other.g0.y, other.g0.z), self.g8, self.g9);
}

MultiVector multi_vector_translator_sub(MultiVector self, Translator other) {
    return MultiVector(self.g0 - vec3(other.g0.x, other.g0.x, other.g0.w) * vec3(0.0, 0.0, 1.0), self.g1, self.g2, self.g3, self.g4, self.g5, self.g6, self.g7 - vec3(other.g0.x, other.g0.y, other.g0.z), self.g8, self.g9);
}

MultiVector multi_vector_translator_regressive_product(MultiVector self, Translator other) {
    return MultiVector(vec3(self.g4.x) * vec3(other.g0.x) * vec3(-1.0, 0.0, 0.0) + vec3(self.g4.y) * vec3(other.g0.y) * vec3(-1.0, 0.0, 0.0) + vec3(self.g4.z) * vec3(other.g0.z) * vec3(-1.0, 0.0, 0.0) + self.g0 * vec3(other.g0.w), vec3(self.g8.x) * vec3(other.g0.z, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0) + vec3(self.g8.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g8.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + self.g1 * vec3(other.g0.w), vec2(self.g6.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g6.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g6.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + self.g2 * vec2(other.g0.w), vec4(self.g9.x) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g9.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g9.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + self.g3 * vec4(other.g0.w), self.g4 * vec3(other.g0.w), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g5 * vec3(other.g0.w), self.g6 * vec3(other.g0.w), vec3(self.g0.z) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g7 * vec3(other.g0.w), self.g8 * vec4(other.g0.w), self.g9 * vec4(other.g0.w));
}

MultiVector multi_vector_translator_geometric_anti_product(MultiVector self, Translator other) {
    return MultiVector(vec3(self.g4.x) * vec3(other.g0.x) * vec3(-1.0, 0.0, 0.0) + vec3(self.g4.y) * vec3(other.g0.y) * vec3(-1.0, 0.0, 0.0) + vec3(self.g4.z) * vec3(other.g0.z) * vec3(-1.0, 0.0, 0.0) + self.g0 * vec3(other.g0.w), vec3(0.0) - vec3(self.g2.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g8.x) * vec3(other.g0.z, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0) + vec3(self.g8.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g8.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + self.g1 * vec3(other.g0.w), vec2(self.g6.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g6.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g6.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + self.g2 * vec2(other.g0.w), vec4(self.g3.w) * other.g0 * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g9.x) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g9.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g9.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + self.g3.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0), self.g4 * vec3(other.g0.w), vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g4.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g4.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g5.y) * vec3(other.g0.w) * vec3(0.0, 1.0, 0.0) + vec3(self.g5.z) * vec3(other.g0.w) * vec3(0.0, 0.0, 1.0) + vec3(self.g5.x, self.g4.x, self.g4.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, -1.0, 1.0), self.g6 * vec3(other.g0.w), vec3(self.g0.z) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g6.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g6.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g7.y) * vec3(other.g0.w) * vec3(0.0, 1.0, 0.0) + vec3(self.g7.z) * vec3(other.g0.w) * vec3(0.0, 0.0, 1.0) + vec3(self.g7.x, self.g6.x, self.g6.x) * vec3(other.g0.w, other.g0.z, other.g0.y) * vec3(1.0, -1.0, 1.0), vec4(self.g8.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, 1.0) + vec4(self.g8.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, 1.0) + vec4(self.g8.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g8.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, 1.0), vec4(self.g9.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, 1.0) + vec4(self.g9.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, 1.0) + vec4(self.g9.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g9.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, 1.0));
}

MultiVector multi_vector_translator_inner_anti_product(MultiVector self, Translator other) {
    return MultiVector(self.g0 * vec3(other.g0.w), self.g1 * vec3(other.g0.w), self.g2 * vec2(other.g0.w), self.g3 * vec4(other.g0.w), self.g4 * vec3(other.g0.w), self.g5 * vec3(other.g0.w), self.g6 * vec3(other.g0.w), vec3(self.g0.z) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g7 * vec3(other.g0.w), self.g8 * vec4(other.g0.w), vec4(self.g9.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, 1.0) + vec4(self.g9.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, 1.0) + vec4(self.g9.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g9.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, 1.0));
}

MultiVector multi_vector_translator_right_anti_contraction(MultiVector self, Translator other) {
    return MultiVector(self.g0 * vec3(other.g0.w), self.g1 * vec3(other.g0.w), self.g2 * vec2(other.g0.w), self.g3 * vec4(other.g0.w), self.g4 * vec3(other.g0.w), self.g5 * vec3(other.g0.w), self.g6 * vec3(other.g0.w), self.g7 * vec3(other.g0.w), self.g8 * vec4(other.g0.w), self.g9 * vec4(other.g0.w));
}

AntiScalar multi_vector_translator_anti_scalar_product(MultiVector self, Translator other) {
    return AntiScalar(self.g0.z * other.g0.w);
}

Flector multi_vector_flector_into(MultiVector self) {
    return Flector(self.g3, self.g9);
}

MultiVector multi_vector_flector_add(MultiVector self, Flector other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3 + other.g0, self.g4, self.g5, self.g6, self.g7, self.g8, self.g9 + other.g1);
}

MultiVector multi_vector_flector_sub(MultiVector self, Flector other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3 - other.g0, self.g4, self.g5, self.g6, self.g7, self.g8, self.g9 - other.g1);
}

MultiVector multi_vector_flector_geometric_anti_product(MultiVector self, Flector other) {
    return MultiVector(vec3(self.g1.y) * vec3(other.g1.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g1.z) * vec3(other.g1.z) * vec3(1.0, 0.0, 0.0) + vec3(self.g2.x) * vec3(other.g1.w, other.g0.w, other.g1.w) * vec3(1.0, -1.0, 0.0) + vec3(self.g3.w) * vec3(other.g0.w) * vec3(0.0, 0.0, -1.0) + vec3(self.g8.x) * vec3(other.g0.x, other.g1.x, other.g0.x) * vec3(-1.0, 1.0, 0.0) + vec3(self.g8.y) * vec3(other.g0.y, other.g1.y, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g8.z) * vec3(other.g0.z, other.g1.z, other.g0.z) * vec3(-1.0, 1.0, 0.0) + vec3(self.g8.w) * vec3(other.g0.w) * vec3(-1.0, 0.0, 0.0) + vec3(self.g9.x) * vec3(other.g1.x) * vec3(0.0, 0.0, 1.0) + vec3(self.g9.y) * vec3(other.g1.y) * vec3(0.0, 0.0, 1.0) + vec3(self.g9.z) * vec3(other.g1.z) * vec3(0.0, 0.0, 1.0) + vec3(self.g1.x) * vec3(other.g1.x) * vec3(1.0, 0.0, 0.0), vec3(self.g0.x) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g4.x) * vec3(other.g1.w, other.g0.z, other.g0.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g4.y) * vec3(other.g0.z, other.g1.w, other.g0.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g4.z) * vec3(other.g0.y, other.g0.x, other.g1.w) * vec3(-1.0, 1.0, 1.0) + vec3(self.g5.x) * vec3(other.g0.w, other.g1.z, other.g1.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g5.y) * vec3(other.g1.z, other.g0.w, other.g1.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g5.z) * vec3(other.g1.y, other.g1.x, other.g0.w) * vec3(-1.0, 1.0, 1.0), vec2(self.g3.x) * vec2(other.g1.x) * vec2(0.0, 1.0) + vec2(self.g3.y) * vec2(other.g1.y) * vec2(0.0, 1.0) + vec2(self.g3.z) * vec2(other.g1.z) * vec2(0.0, 1.0) + vec2(self.g3.w) * vec2(other.g1.w) * vec2(0.0, 1.0) + vec2(self.g4.x) * vec2(other.g1.x) * vec2(-1.0, 0.0) + vec2(self.g4.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g4.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g9.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g9.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g9.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g9.w) * vec2(other.g0.w) * vec2(0.0, -1.0) + vec2(self.g0.y, self.g0.x) * vec2(other.g0.w, other.g0.x) * vec2(1.0, 0.0), vec4(self.g0.z) * other.g0 + vec4(self.g6.x) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g1.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g6.y) * vec4(other.g0.z, other.g1.w, other.g0.x, other.g1.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g6.z) * vec4(other.g0.y, other.g0.x, other.g1.w, other.g1.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g7.x) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.w) * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g7.y) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g1.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g7.z) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g1.y) * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g2.y, self.g2.y, self.g2.y, self.g2.x) * other.g1.xyzx * vec4(1.0, 1.0, 1.0, 0.0), vec3(0.0) - vec3(self.g2.x) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g8.x) * vec3(other.g0.w, other.g1.z, other.g1.y) * vec3(-1.0, 1.0, -1.0) + vec3(self.g8.y) * vec3(other.g1.z, other.g0.w, other.g1.x) * vec3(-1.0, -1.0, 1.0) + vec3(self.g8.z) * vec3(other.g1.y, other.g1.x, other.g0.w) * vec3(1.0, -1.0, -1.0), vec3(self.g1.x) * vec3(other.g0.w, other.g1.z, other.g1.y) * vec3(-1.0, 1.0, -1.0) + vec3(self.g1.y) * vec3(other.g1.z, other.g0.w, other.g1.x) * vec3(-1.0, -1.0, 1.0) + vec3(self.g1.z) * vec3(other.g1.y, other.g1.x, other.g0.w) * vec3(1.0, -1.0, -1.0) + vec3(self.g2.x) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g8.x) * vec3(other.g1.w, other.g0.z, other.g0.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g8.y) * vec3(other.g0.z, other.g1.w, other.g0.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g8.z) * vec3(other.g0.y, other.g0.x, other.g1.w) * vec3(-1.0, 1.0, 1.0) - vec3(self.g8.w) * vec3(other.g1.x, other.g1.y, other.g1.z), vec3(0.0) - vec3(self.g3.w) * vec3(other.g1.x, other.g1.y, other.g1.z) + vec3(self.g9.x) * vec3(other.g0.w, other.g1.z, other.g1.y) * vec3(-1.0, 1.0, -1.0) + vec3(self.g9.y) * vec3(other.g1.z, other.g0.w, other.g1.x) * vec3(-1.0, -1.0, 1.0) + vec3(self.g9.z) * vec3(other.g1.y, other.g1.x, other.g0.w) * vec3(1.0, -1.0, -1.0), vec3(self.g3.x) * vec3(other.g0.w, other.g1.z, other.g1.y) * vec3(-1.0, 1.0, -1.0) + vec3(self.g3.y) * vec3(other.g1.z, other.g0.w, other.g1.x) * vec3(-1.0, -1.0, 1.0) + vec3(self.g3.z) * vec3(other.g1.y, other.g1.x, other.g0.w) * vec3(1.0, -1.0, -1.0) + vec3(self.g3.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g9.x) * vec3(other.g1.w, other.g0.z, other.g0.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g9.y) * vec3(other.g0.z, other.g1.w, other.g0.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g9.z) * vec3(other.g0.y, other.g0.x, other.g1.w) * vec3(-1.0, 1.0, 1.0) - vec3(self.g9.w) * vec3(other.g1.x, other.g1.y, other.g1.z), vec4(self.g0.y) * other.g1 + vec4(self.g4.x) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g4.y) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g4.z) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g5.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g5.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g5.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x) * other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0), vec4(self.g0.z) * other.g1 + vec4(self.g6.x) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g0.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g6.y) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g0.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g6.z) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g0.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g7.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g7.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g7.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g2.x, self.g2.x, self.g2.x, self.g2.y) * other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0));
}

AntiScalar multi_vector_flector_anti_scalar_product(MultiVector self, Flector other) {
    return AntiScalar(0.0 - self.g3.w * other.g0.w + self.g9.x * other.g1.x + self.g9.y * other.g1.y + self.g9.z * other.g1.z);
}

Dilation multi_vector_dilation_into(MultiVector self) {
    return Dilation(vec3(self.g8.y, self.g8.z, self.g8.w) * vec3(-1.0), vec2(self.g8.x, self.g0.z) * vec2(-1.0, 1.0));
}

MultiVector multi_vector_dilation_geometric_product(MultiVector self, Dilation other) {
    return MultiVector(vec3(self.g1.y) * vec3(other.g0.x) * vec3(0.0, -1.0, 0.0) + vec3(self.g1.z) * vec3(other.g0.y) * vec3(0.0, -1.0, 0.0) + vec3(self.g2.x) * vec3(other.g0.z) * vec3(0.0, -1.0, 0.0) + vec3(self.g3.x) * vec3(other.g1.x) * vec3(0.0, 0.0, 1.0) + vec3(self.g3.y) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0) + vec3(self.g3.z) * vec3(other.g0.y) * vec3(0.0, 0.0, 1.0) + vec3(self.g3.w) * vec3(other.g0.z) * vec3(0.0, 0.0, 1.0) + vec3(self.g8.w) * vec3(other.g0.z) * vec3(1.0, 0.0, 0.0) + vec3(self.g0.x, self.g1.x, self.g0.x) * vec3(other.g1.x, other.g1.x, other.g1.y) * vec3(0.0, -1.0, 1.0), self.g5 * vec3(other.g0.z) * vec3(-1.0), vec2(self.g5.x) * vec2(other.g1.x) * vec2(1.0, 0.0) + vec2(self.g5.y) * vec2(other.g0.x) * vec2(1.0, 0.0) + vec2(self.g5.z) * vec2(other.g0.y) * vec2(1.0, 0.0) + vec2(self.g9.w) * vec2(other.g0.z) * vec2(0.0, -1.0) + vec2(self.g0.y, self.g0.x) * vec2(other.g0.z, other.g0.x) * vec2(1.0, 0.0), vec4(self.g7.x) * vec4(other.g0.z, other.g0.z, other.g0.z, other.g1.x) * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g7.y) * vec4(other.g0.z, other.g0.z, other.g0.z, other.g0.x) * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g7.z) * vec4(other.g0.z, other.g0.z, other.g0.z, other.g0.y) * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g8.w) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x, self.g0.x, self.g0.x, self.g0.z) * vec4(other.g0.x, other.g0.x, other.g0.x, other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0), vec3(self.g1.y) * vec3(other.g0.y, other.g0.y, other.g1.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * vec3(other.g0.x, other.g1.x, other.g0.x) * vec3(1.0, -1.0, 0.0) + vec3(self.g8.x) * vec3(other.g0.z) * vec3(-1.0, 0.0, 0.0) + vec3(self.g8.y) * vec3(other.g0.z) * vec3(0.0, -1.0, 0.0) + vec3(self.g8.z) * vec3(other.g0.z) * vec3(0.0, 0.0, -1.0) + vec3(self.g8.w) * vec3(other.g1.x, other.g0.x, other.g0.y) + vec3(self.g1.x) * other.g0.xyx * vec3(0.0, 1.0, -1.0), self.g1 * vec3(other.g0.z), vec3(self.g3.y) * vec3(other.g0.y, other.g0.y, other.g1.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g3.z) * vec3(other.g0.x, other.g1.x, other.g0.x) * vec3(-1.0, 1.0, 0.0) + vec3(self.g5.x) * vec3(other.g1.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g5.y) * vec3(other.g1.y) * vec3(0.0, 1.0, 0.0) + vec3(self.g5.z) * vec3(other.g1.y) * vec3(0.0, 0.0, 1.0) + vec3(self.g9.y) * vec3(other.g0.z) * vec3(0.0, 1.0, 0.0) + vec3(self.g9.z) * vec3(other.g0.z) * vec3(0.0, 0.0, 1.0) - vec3(self.g9.w) * vec3(other.g1.x, other.g0.x, other.g0.y) + vec3(self.g9.x, self.g3.x, self.g3.x) * other.g0.zyx * vec3(1.0, -1.0, 1.0), vec3(self.g3.x, self.g3.y, self.g3.z) * vec3(other.g0.z) * vec3(-1.0), vec4(0.0) - vec4(self.g0.x) * vec4(other.g1.x, other.g0.x, other.g0.y, other.g0.z) + vec4(self.g5.x) * vec4(other.g0.y, other.g0.y, other.g0.x, other.g0.y) * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g5.y) * vec4(other.g0.y, other.g0.y, other.g1.x, other.g0.y) * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g5.z) * vec4(other.g0.x, other.g1.x, other.g0.x, other.g0.x) * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g4.x, self.g4.y, self.g4.z, self.g4.x) * vec4(other.g0.z, other.g0.z, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0), vec4(self.g2.y) * vec4(other.g1.x, other.g0.x, other.g0.y, other.g0.z) + vec4(self.g6.x) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g6.y) * vec4(other.g0.z) * vec4(0.0, -1.0, 0.0, 0.0) + vec4(self.g6.z) * vec4(other.g0.z) * vec4(0.0, 0.0, -1.0, 0.0) + vec4(self.g7.x) * vec4(other.g0.y, other.g0.y, other.g0.x, other.g0.y) * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g7.y) * vec4(other.g0.y, other.g0.y, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g7.z) * vec4(other.g0.x, other.g1.x, other.g0.x, other.g0.x) * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.x) * vec4(other.g1.y, other.g1.y, other.g1.y, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

MultiVector multi_vector_dilation_regressive_product(MultiVector self, Dilation other) {
    return MultiVector(vec3(self.g3.x) * vec3(other.g1.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g3.y) * vec3(other.g0.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g3.z) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g3.w) * vec3(other.g0.z) * vec3(1.0, 0.0, 0.0) + self.g0 * vec3(other.g1.y), vec3(self.g6.x) * vec3(other.g0.z) * vec3(-1.0, 0.0, 0.0) + vec3(self.g6.y) * vec3(other.g0.z) * vec3(0.0, -1.0, 0.0) + vec3(self.g6.z) * vec3(other.g0.z) * vec3(0.0, 0.0, -1.0) + vec3(self.g7.x) * other.g0.yyx * vec3(0.0, 1.0, -1.0) + vec3(self.g7.y) * vec3(other.g0.y, other.g0.y, other.g1.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g7.z) * vec3(other.g0.x, other.g1.x, other.g0.x) * vec3(1.0, -1.0, 0.0) + self.g1 * vec3(other.g1.y), vec2(self.g6.x) * vec2(other.g1.x) * vec2(1.0, 0.0) + vec2(self.g6.y) * vec2(other.g0.x) * vec2(1.0, 0.0) + vec2(self.g6.z) * vec2(other.g0.y) * vec2(1.0, 0.0) + self.g2 * vec2(other.g1.y), self.g3 * vec4(other.g1.y), vec3(self.g9.x) * other.g0.yyx * vec3(0.0, 1.0, -1.0) + vec3(self.g9.y) * vec3(other.g0.y, other.g0.y, other.g1.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g9.z) * vec3(other.g0.x, other.g1.x, other.g0.x) * vec3(1.0, -1.0, 0.0) + self.g4 * vec3(other.g1.y), vec3(self.g9.x) * vec3(other.g0.z) * vec3(1.0, 0.0, 0.0) + vec3(self.g9.y) * vec3(other.g0.z) * vec3(0.0, 1.0, 0.0) + vec3(self.g9.z) * vec3(other.g0.z) * vec3(0.0, 0.0, 1.0) - vec3(self.g9.w) * vec3(other.g1.x, other.g0.x, other.g0.y) + self.g5 * vec3(other.g1.y), self.g6 * vec3(other.g1.y), self.g7 * vec3(other.g1.y), vec4(0.0) - vec4(self.g0.z) * vec4(other.g1.x, other.g0.x, other.g0.y, other.g0.z) + self.g8 * vec4(other.g1.y), self.g9 * vec4(other.g1.y));
}

MultiVector multi_vector_dilation_geometric_anti_product(MultiVector self, Dilation other) {
    return MultiVector(vec3(self.g3.x) * vec3(other.g1.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g3.y) * vec3(other.g0.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g3.z) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g3.w) * vec3(other.g0.z) * vec3(1.0, 0.0, 0.0) + vec3(self.g9.x) * vec3(other.g1.x) * vec3(0.0, 1.0, 0.0) + vec3(self.g9.y) * vec3(other.g0.x) * vec3(0.0, 1.0, 0.0) + vec3(self.g9.z) * vec3(other.g0.y) * vec3(0.0, 1.0, 0.0) + self.g0 * vec3(other.g1.y), vec3(0.0) - vec3(self.g2.y) * vec3(other.g1.x, other.g0.x, other.g0.y) + vec3(self.g6.x) * vec3(other.g0.z) * vec3(-1.0, 0.0, 0.0) + vec3(self.g6.y) * vec3(other.g0.z) * vec3(0.0, -1.0, 0.0) + vec3(self.g6.z) * vec3(other.g0.z) * vec3(0.0, 0.0, -1.0) + vec3(self.g7.x) * other.g0.yyx * vec3(0.0, 1.0, -1.0) + vec3(self.g7.y) * vec3(other.g0.y, other.g0.y, other.g1.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g7.z) * vec3(other.g0.x, other.g1.x, other.g0.x) * vec3(1.0, -1.0, 0.0) + self.g1 * vec3(other.g1.y), vec2(self.g6.x) * vec2(other.g1.x) * vec2(1.0, 0.0) + vec2(self.g6.y) * vec2(other.g0.x) * vec2(1.0, 0.0) + vec2(self.g6.z) * vec2(other.g0.y) * vec2(1.0, 0.0) + self.g2 * vec2(other.g1.y), self.g3 * vec4(other.g1.y), vec3(0.0) - vec3(self.g3.w) * vec3(other.g1.x, other.g0.x, other.g0.y) + vec3(self.g9.x) * other.g0.yyx * vec3(0.0, 1.0, -1.0) + vec3(self.g9.y) * vec3(other.g0.y, other.g0.y, other.g1.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g9.z) * vec3(other.g0.x, other.g1.x, other.g0.x) * vec3(1.0, -1.0, 0.0) + self.g4 * vec3(other.g1.y), vec3(self.g3.y) * vec3(other.g0.y, other.g0.y, other.g1.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g3.z) * vec3(other.g0.x, other.g1.x, other.g0.x) * vec3(1.0, -1.0, 0.0) + vec3(self.g5.x) * vec3(other.g1.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g5.y) * vec3(other.g1.y) * vec3(0.0, 1.0, 0.0) + vec3(self.g5.z) * vec3(other.g1.y) * vec3(0.0, 0.0, 1.0) + vec3(self.g9.y) * vec3(other.g0.z) * vec3(0.0, 1.0, 0.0) + vec3(self.g9.z) * vec3(other.g0.z) * vec3(0.0, 0.0, 1.0) - vec3(self.g9.w) * vec3(other.g1.x, other.g0.x, other.g0.y) + vec3(self.g9.x, self.g3.x, self.g3.x) * other.g0.zyx * vec3(1.0, 1.0, -1.0), self.g6 * vec3(other.g1.y), self.g7 * vec3(other.g1.y), vec4(0.0) - vec4(self.g0.z) * vec4(other.g1.x, other.g0.x, other.g0.y, other.g0.z) + vec4(self.g6.y) * vec4(other.g0.y, other.g0.y, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g6.z) * vec4(other.g0.x, other.g1.x, other.g0.x, other.g0.x) * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g7.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g7.z) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g8.x) * vec4(other.g1.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g8.y) * vec4(other.g1.y) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g8.z) * vec4(other.g1.y) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g8.w) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g6.x, self.g6.x, self.g6.x, self.g7.y) * vec4(other.g0.x, other.g0.y, other.g0.x, other.g0.x) * vec4(0.0, 1.0, -1.0, 1.0), self.g9 * vec4(other.g1.y));
}

MultiVector multi_vector_dilation_inner_anti_product(MultiVector self, Dilation other) {
    return MultiVector(vec3(self.g9.x) * vec3(other.g1.x) * vec3(0.0, 1.0, 0.0) + vec3(self.g9.y) * vec3(other.g0.x) * vec3(0.0, 1.0, 0.0) + vec3(self.g9.z) * vec3(other.g0.y) * vec3(0.0, 1.0, 0.0) + self.g0 * vec3(other.g1.y), self.g1 * vec3(other.g1.y), self.g2 * vec2(other.g1.y), self.g3 * vec4(other.g1.y), self.g4 * vec3(other.g1.y), self.g5 * vec3(other.g1.y), self.g6 * vec3(other.g1.y), self.g7 * vec3(other.g1.y), vec4(0.0) - vec4(self.g0.z) * vec4(other.g1.x, other.g0.x, other.g0.y, other.g0.z) + self.g8 * vec4(other.g1.y), self.g9 * vec4(other.g1.y));
}

MultiVector multi_vector_dilation_right_anti_contraction(MultiVector self, Dilation other) {
    return MultiVector(self.g0 * vec3(other.g1.y), self.g1 * vec3(other.g1.y), self.g2 * vec2(other.g1.y), self.g3 * vec4(other.g1.y), self.g4 * vec3(other.g1.y), self.g5 * vec3(other.g1.y), self.g6 * vec3(other.g1.y), self.g7 * vec3(other.g1.y), self.g8 * vec4(other.g1.y), self.g9 * vec4(other.g1.y));
}

Scalar multi_vector_dilation_scalar_product(MultiVector self, Dilation other) {
    return Scalar(self.g8.w * other.g0.z);
}

AntiScalar multi_vector_dilation_anti_scalar_product(MultiVector self, Dilation other) {
    return AntiScalar(self.g0.z * other.g1.y);
}

MultiVector multi_vector_multi_vector_add(MultiVector self, MultiVector other) {
    return MultiVector(self.g0 + other.g0, self.g1 + other.g1, self.g2 + other.g2, self.g3 + other.g3, self.g4 + other.g4, self.g5 + other.g5, self.g6 + other.g6, self.g7 + other.g7, self.g8 + other.g8, self.g9 + other.g9);
}

MultiVector multi_vector_multi_vector_sub(MultiVector self, MultiVector other) {
    return MultiVector(self.g0 - other.g0, self.g1 - other.g1, self.g2 - other.g2, self.g3 - other.g3, self.g4 - other.g4, self.g5 - other.g5, self.g6 - other.g6, self.g7 - other.g7, self.g8 - other.g8, self.g9 - other.g9);
}

MultiVector multi_vector_multi_vector_mul(MultiVector self, MultiVector other) {
    return MultiVector(self.g0 * other.g0, self.g1 * other.g1, self.g2 * other.g2, self.g3 * other.g3, self.g4 * other.g4, self.g5 * other.g5, self.g6 * other.g6, self.g7 * other.g7, self.g8 * other.g8, self.g9 * other.g9);
}

MultiVector multi_vector_multi_vector_div(MultiVector self, MultiVector other) {
    return MultiVector(vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(1.0, 1.0, 1.0) / vec3(other.g0.x, other.g0.y, other.g0.z) * vec3(1.0, 1.0, 1.0), vec3(self.g1.x, self.g1.y, self.g1.z) * vec3(1.0, 1.0, 1.0) / vec3(other.g1.x, other.g1.y, other.g1.z) * vec3(1.0, 1.0, 1.0), vec2(self.g2.x, self.g2.y) * vec2(1.0, 1.0) / vec2(other.g2.x, other.g2.y) * vec2(1.0, 1.0), vec4(self.g3.x, self.g3.y, self.g3.z, self.g3.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g3.x, other.g3.y, other.g3.z, other.g3.w) * vec4(1.0, 1.0, 1.0, 1.0), vec3(self.g4.x, self.g4.y, self.g4.z) * vec3(1.0, 1.0, 1.0) / vec3(other.g4.x, other.g4.y, other.g4.z) * vec3(1.0, 1.0, 1.0), vec3(self.g5.x, self.g5.y, self.g5.z) * vec3(1.0, 1.0, 1.0) / vec3(other.g5.x, other.g5.y, other.g5.z) * vec3(1.0, 1.0, 1.0), vec3(self.g6.x, self.g6.y, self.g6.z) * vec3(1.0, 1.0, 1.0) / vec3(other.g6.x, other.g6.y, other.g6.z) * vec3(1.0, 1.0, 1.0), vec3(self.g7.x, self.g7.y, self.g7.z) * vec3(1.0, 1.0, 1.0) / vec3(other.g7.x, other.g7.y, other.g7.z) * vec3(1.0, 1.0, 1.0), vec4(self.g8.x, self.g8.y, self.g8.z, self.g8.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g8.x, other.g8.y, other.g8.z, other.g8.w) * vec4(1.0, 1.0, 1.0, 1.0), vec4(self.g9.x, self.g9.y, self.g9.z, self.g9.w) * vec4(1.0, 1.0, 1.0, 1.0) / vec4(other.g9.x, other.g9.y, other.g9.z, other.g9.w) * vec4(1.0, 1.0, 1.0, 1.0));
}

MultiVector multi_vector_multi_vector_geometric_product(MultiVector self, MultiVector other) {
    return MultiVector(vec3(self.g0.x) * other.g0 + vec3(self.g0.z) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0) + vec3(self.g1.x) * vec3(other.g1.x, other.g8.x, other.g9.x) + vec3(self.g1.y) * vec3(other.g1.y, other.g8.y, other.g9.y) + vec3(self.g1.z) * vec3(other.g1.z, other.g8.z, other.g9.z) + vec3(self.g2.x) * vec3(other.g8.w, other.g8.w, other.g9.w) * vec3(0.0, 1.0, 1.0) + vec3(self.g2.y) * vec3(other.g0.y) * vec3(0.0, 0.0, 1.0) + vec3(self.g3.x) * vec3(other.g8.x) * vec3(0.0, 0.0, -1.0) + vec3(self.g3.y) * vec3(other.g8.y) * vec3(0.0, 0.0, -1.0) + vec3(self.g3.z) * vec3(other.g8.z) * vec3(0.0, 0.0, -1.0) + vec3(self.g3.w) * vec3(other.g8.w) * vec3(0.0, 0.0, -1.0) + vec3(self.g4.x) * vec3(other.g5.x, other.g5.x, other.g7.x) * vec3(0.0, -1.0, -1.0) + vec3(self.g4.y) * vec3(other.g5.y, other.g5.y, other.g7.y) * vec3(0.0, -1.0, -1.0) + vec3(self.g4.z) * vec3(other.g5.z, other.g5.z, other.g7.z) * vec3(0.0, -1.0, -1.0) - vec3(self.g5.x) * vec3(other.g5.x, other.g4.x, other.g6.x) - vec3(self.g5.y) * vec3(other.g5.y, other.g4.y, other.g6.y) - vec3(self.g5.z) * vec3(other.g5.z, other.g4.z, other.g6.z) + vec3(self.g6.x) * vec3(other.g5.x) * vec3(0.0, 0.0, -1.0) + vec3(self.g6.y) * vec3(other.g5.y) * vec3(0.0, 0.0, -1.0) + vec3(self.g6.z) * vec3(other.g5.z) * vec3(0.0, 0.0, -1.0) + vec3(self.g7.x) * vec3(other.g4.x) * vec3(0.0, 0.0, -1.0) + vec3(self.g7.y) * vec3(other.g4.y) * vec3(0.0, 0.0, -1.0) + vec3(self.g7.z) * vec3(other.g4.z) * vec3(0.0, 0.0, -1.0) + vec3(self.g8.x) * vec3(other.g1.x, other.g1.x, other.g3.x) * vec3(0.0, -1.0, -1.0) + vec3(self.g8.y) * vec3(other.g1.y, other.g1.y, other.g3.y) * vec3(0.0, -1.0, -1.0) + vec3(self.g8.z) * vec3(other.g1.z, other.g1.z, other.g3.z) * vec3(0.0, -1.0, -1.0) - vec3(self.g8.w) * vec3(other.g8.w, other.g2.x, other.g3.w) + vec3(self.g9.x) * vec3(other.g1.x) * vec3(0.0, 0.0, 1.0) + vec3(self.g9.y) * vec3(other.g1.y) * vec3(0.0, 0.0, 1.0) + vec3(self.g9.z) * vec3(other.g1.z) * vec3(0.0, 0.0, 1.0) + vec3(self.g9.w) * vec3(other.g2.x) * vec3(0.0, 0.0, 1.0) + self.g0.xyy * vec3(other.g0.x, other.g0.x, other.g2.y) * vec3(0.0, 1.0, 1.0), vec3(self.g0.x) * other.g1 + vec3(self.g1.x) * vec3(other.g0.x, other.g5.z, other.g5.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.y) * vec3(other.g5.z, other.g0.x, other.g5.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g1.z) * vec3(other.g5.y, other.g5.x, other.g0.x) * vec3(1.0, -1.0, 1.0) + vec3(self.g5.x) * vec3(other.g8.w, other.g1.z, other.g1.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g5.y) * vec3(other.g1.z, other.g8.w, other.g1.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g5.z) * vec3(other.g1.y, other.g1.x, other.g8.w) * vec3(1.0, -1.0, 1.0) + vec3(self.g8.w) * other.g5, vec2(self.g0.x) * other.g2 + vec2(self.g1.x) * vec2(other.g4.x, other.g3.x) * vec2(-1.0, 1.0) + vec2(self.g1.y) * vec2(other.g4.y, other.g3.y) * vec2(-1.0, 1.0) + vec2(self.g1.z) * vec2(other.g4.z, other.g3.z) * vec2(-1.0, 1.0) + vec2(self.g2.x) * vec2(other.g0.x) * vec2(1.0, 0.0) + vec2(self.g2.y) * vec2(other.g0.x) * vec2(0.0, 1.0) + vec2(self.g3.x) * vec2(other.g1.x) * vec2(0.0, -1.0) + vec2(self.g3.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g3.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g4.x) * vec2(other.g1.x) * vec2(1.0, 0.0) + vec2(self.g4.y) * vec2(other.g1.y) * vec2(1.0, 0.0) + vec2(self.g4.z) * vec2(other.g1.z) * vec2(1.0, 0.0) - vec2(self.g5.x) * vec2(other.g8.x, other.g7.x) - vec2(self.g5.y) * vec2(other.g8.y, other.g7.y) - vec2(self.g5.z) * vec2(other.g8.z, other.g7.z) + vec2(self.g7.x) * vec2(other.g5.x) * vec2(0.0, -1.0) + vec2(self.g7.y) * vec2(other.g5.y) * vec2(0.0, -1.0) + vec2(self.g7.z) * vec2(other.g5.z) * vec2(0.0, -1.0) + vec2(self.g8.x) * vec2(other.g5.x) * vec2(-1.0, 0.0) + vec2(self.g8.y) * vec2(other.g5.y) * vec2(-1.0, 0.0) + vec2(self.g8.z) * vec2(other.g5.z) * vec2(-1.0, 0.0) + vec2(self.g8.w) * vec2(other.g0.y, other.g9.w) * vec2(1.0, -1.0) + vec2(self.g9.w) * vec2(other.g8.w) * vec2(0.0, 1.0) + vec2(self.g0.y, self.g0.x) * vec2(other.g8.w, other.g8.x) * vec2(-1.0, 0.0), vec4(self.g0.x) * other.g3 + vec4(self.g0.z) * vec4(other.g8.w) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g2.y, other.g7.z, other.g7.y, other.g6.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g7.z, other.g2.y, other.g7.x, other.g6.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g7.y, other.g7.x, other.g2.y, other.g6.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g2.x) * vec4(other.g2.y) * vec4(0.0, 0.0, 0.0, 1.0) - vec4(self.g2.y) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g2.x) + vec4(self.g3.x) * vec4(other.g0.x, other.g5.z, other.g5.y, other.g4.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g3.y) * vec4(other.g5.z, other.g0.x, other.g5.x, other.g4.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g3.z) * vec4(other.g5.y, other.g5.x, other.g0.x, other.g4.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g3.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g4.x) * vec4(other.g3.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g4.y) * vec4(other.g3.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g4.z) * vec4(other.g3.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g5.x) * vec4(other.g9.w, other.g3.z, other.g3.y, other.g9.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g5.y) * vec4(other.g3.z, other.g9.w, other.g3.x, other.g9.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g5.z) * vec4(other.g3.y, other.g3.x, other.g9.w, other.g9.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g6.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g6.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g6.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g7.x) * vec4(other.g8.w, other.g1.z, other.g1.y, other.g8.x) * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g7.y) * vec4(other.g1.z, other.g8.w, other.g1.x, other.g8.y) * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g7.z) * vec4(other.g1.y, other.g1.x, other.g8.w, other.g8.z) * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g8.x) * vec4(other.g7.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g8.y) * vec4(other.g7.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g8.z) * vec4(other.g7.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g8.w) * vec4(other.g7.x, other.g7.y, other.g7.z, other.g0.z) + vec4(self.g9.x) * vec4(other.g5.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g9.y) * vec4(other.g5.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g9.z) * vec4(other.g5.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g9.w) * vec4(other.g5.x, other.g5.y, other.g5.z, other.g0.y) + vec4(self.g0.x, self.g0.x, self.g0.x, self.g0.y) * other.g9.xxxw * vec4(0.0, 0.0, 0.0, -1.0), vec3(self.g0.x) * other.g4 + vec3(self.g0.y) * other.g5 + vec3(self.g1.x) * vec3(other.g2.x, other.g8.z, other.g8.y) * vec3(-1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g8.z, other.g2.x, other.g8.x) * vec3(1.0, -1.0, -1.0) + vec3(self.g1.z) * vec3(other.g8.y, other.g8.x, other.g2.x) * vec3(-1.0, 1.0, -1.0) + vec3(self.g2.x) * other.g1 + vec3(self.g4.x) * vec3(other.g0.x, other.g5.z, other.g5.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g4.y) * vec3(other.g5.z, other.g0.x, other.g5.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g4.z) * vec3(other.g5.y, other.g5.x, other.g0.x) * vec3(1.0, -1.0, 1.0) + vec3(self.g5.x) * vec3(other.g0.y, other.g4.z, other.g4.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g5.y) * vec3(other.g4.z, other.g0.y, other.g4.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g5.z) * vec3(other.g4.y, other.g4.x, other.g0.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g8.x) * vec3(other.g8.w, other.g1.z, other.g1.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g8.y) * vec3(other.g1.z, other.g8.w, other.g1.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g8.z) * vec3(other.g1.y, other.g1.x, other.g8.w) * vec3(1.0, -1.0, 1.0) - vec3(self.g8.w) * vec3(other.g8.x, other.g8.y, other.g8.z), vec3(self.g0.x) * other.g5 + vec3(self.g1.x) * vec3(other.g8.w, other.g1.z, other.g1.y) * vec3(-1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g1.z, other.g8.w, other.g1.x) * vec3(1.0, -1.0, -1.0) + vec3(self.g1.z) * vec3(other.g1.y, other.g1.x, other.g8.w) * vec3(-1.0, 1.0, -1.0) + vec3(self.g5.x) * vec3(other.g0.x, other.g5.z, other.g5.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g5.y) * vec3(other.g5.z, other.g0.x, other.g5.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g5.z) * vec3(other.g5.y, other.g5.x, other.g0.x) * vec3(1.0, -1.0, 1.0) - vec3(self.g8.w) * other.g1, vec3(self.g0.x) * other.g6 + vec3(self.g0.y) * other.g7 + vec3(self.g0.z) * other.g5 + vec3(self.g1.x) * vec3(other.g3.w, other.g9.z, other.g9.y) * vec3(-1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g9.z, other.g3.w, other.g9.x) * vec3(1.0, -1.0, -1.0) + vec3(self.g1.z) * vec3(other.g9.y, other.g9.x, other.g3.w) * vec3(-1.0, 1.0, -1.0) + vec3(self.g2.x) * vec3(other.g3.x, other.g3.y, other.g3.z) + vec3(self.g2.y) * other.g4 + vec3(self.g3.x) * vec3(other.g2.x, other.g8.z, other.g8.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g3.y) * vec3(other.g8.z, other.g2.x, other.g8.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g3.z) * vec3(other.g8.y, other.g8.x, other.g2.x) * vec3(1.0, -1.0, 1.0) - vec3(self.g3.w) * other.g1 + vec3(self.g4.x) * vec3(other.g2.y, other.g7.z, other.g7.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g4.y) * vec3(other.g7.z, other.g2.y, other.g7.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g4.z) * vec3(other.g7.y, other.g7.x, other.g2.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g5.x) * vec3(other.g0.z, other.g6.z, other.g6.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g5.y) * vec3(other.g6.z, other.g0.z, other.g6.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g5.z) * vec3(other.g6.y, other.g6.x, other.g0.z) * vec3(1.0, -1.0, 1.0) + vec3(self.g6.x) * vec3(other.g0.x, other.g5.z, other.g5.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g6.y) * vec3(other.g5.z, other.g0.x, other.g5.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g6.z) * vec3(other.g5.y, other.g5.x, other.g0.x) * vec3(1.0, -1.0, 1.0) + vec3(self.g7.x) * vec3(other.g0.y, other.g4.z, other.g4.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g7.y) * vec3(other.g4.z, other.g0.y, other.g4.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g7.z) * vec3(other.g4.y, other.g4.x, other.g0.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g8.x) * vec3(other.g9.w, other.g3.z, other.g3.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g8.y) * vec3(other.g3.z, other.g9.w, other.g3.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g8.z) * vec3(other.g3.y, other.g3.x, other.g9.w) * vec3(1.0, -1.0, 1.0) - vec3(self.g8.w) * vec3(other.g9.x, other.g9.y, other.g9.z) + vec3(self.g9.x) * vec3(other.g8.w, other.g1.z, other.g1.y) * vec3(-1.0, -1.0, 1.0) + vec3(self.g9.y) * vec3(other.g1.z, other.g8.w, other.g1.x) * vec3(1.0, -1.0, -1.0) + vec3(self.g9.z) * vec3(other.g1.y, other.g1.x, other.g8.w) * vec3(-1.0, 1.0, -1.0) + vec3(self.g9.w) * vec3(other.g8.x, other.g8.y, other.g8.z), vec3(self.g0.x) * other.g7 + vec3(self.g1.x) * vec3(other.g9.w, other.g3.z, other.g3.y) * vec3(-1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g3.z, other.g9.w, other.g3.x) * vec3(1.0, -1.0, -1.0) + vec3(self.g1.z) * vec3(other.g3.y, other.g3.x, other.g9.w) * vec3(-1.0, 1.0, -1.0) + vec3(self.g2.y) * other.g5 + vec3(self.g3.x) * vec3(other.g8.w, other.g1.z, other.g1.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g3.y) * vec3(other.g1.z, other.g8.w, other.g1.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g3.z) * vec3(other.g1.y, other.g1.x, other.g8.w) * vec3(1.0, -1.0, 1.0) + vec3(self.g5.x) * vec3(other.g2.y, other.g7.z, other.g7.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g5.y) * vec3(other.g7.z, other.g2.y, other.g7.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g5.z) * vec3(other.g7.y, other.g7.x, other.g2.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g7.x) * vec3(other.g0.x, other.g5.z, other.g5.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g7.y) * vec3(other.g5.z, other.g0.x, other.g5.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g7.z) * vec3(other.g5.y, other.g5.x, other.g0.x) * vec3(1.0, -1.0, 1.0) - vec3(self.g8.w) * vec3(other.g3.x, other.g3.y, other.g3.z) + vec3(self.g9.w) * other.g1, vec4(self.g0.x) * other.g8 + vec4(self.g1.x) * vec4(other.g0.y, other.g4.z, other.g4.y, other.g5.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g4.z, other.g0.y, other.g4.x, other.g5.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g4.y, other.g4.x, other.g0.y, other.g5.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g2.x) * vec4(other.g5.x, other.g5.y, other.g5.z, other.g5.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g4.x) * vec4(other.g8.w, other.g1.z, other.g1.y, other.g8.w) * vec4(-1.0, -1.0, 1.0, 0.0) + vec4(self.g4.y) * vec4(other.g1.z, other.g8.w, other.g1.x, other.g1.z) * vec4(1.0, -1.0, -1.0, 0.0) + vec4(self.g4.z) * vec4(other.g1.y, other.g1.x, other.g8.w, other.g1.y) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g5.x) * vec4(other.g2.x, other.g8.z, other.g8.y, other.g1.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g5.y) * vec4(other.g8.z, other.g2.x, other.g8.x, other.g1.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g5.z) * vec4(other.g8.y, other.g8.x, other.g2.x, other.g1.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g8.x) * vec4(other.g0.x, other.g5.z, other.g5.y, other.g0.x) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g8.y) * vec4(other.g5.z, other.g0.x, other.g5.x, other.g5.z) * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g8.z) * vec4(other.g5.y, other.g5.x, other.g0.x, other.g5.y) * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g8.w) * vec4(other.g4.x, other.g4.y, other.g4.z, other.g0.x) + vec4(self.g0.y, self.g0.y, self.g0.y, self.g0.x) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(-1.0, -1.0, -1.0, 0.0), vec4(self.g0.x) * other.g9 + vec4(self.g0.z) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.z, other.g6.z, other.g6.y, other.g7.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g6.z, other.g0.z, other.g6.x, other.g7.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g6.y, other.g6.x, other.g0.z, other.g7.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g2.x) * vec4(other.g7.x, other.g7.y, other.g7.z, other.g7.x) * vec4(1.0, 1.0, 1.0, 0.0) - vec4(self.g2.y) * other.g8 + vec4(self.g3.x) * vec4(other.g0.y, other.g4.z, other.g4.y, other.g5.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g3.y) * vec4(other.g4.z, other.g0.y, other.g4.x, other.g5.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g3.z) * vec4(other.g4.y, other.g4.x, other.g0.y, other.g5.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g3.w) * vec4(other.g5.x, other.g5.y, other.g5.z, other.g5.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g4.x) * vec4(other.g9.w, other.g3.z, other.g3.y, other.g9.w) * vec4(-1.0, -1.0, 1.0, 0.0) + vec4(self.g4.y) * vec4(other.g3.z, other.g9.w, other.g3.x, other.g3.z) * vec4(1.0, -1.0, -1.0, 0.0) + vec4(self.g4.z) * vec4(other.g3.y, other.g3.x, other.g9.w, other.g3.y) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g5.x) * vec4(other.g3.w, other.g9.z, other.g9.y, other.g3.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g5.y) * vec4(other.g9.z, other.g3.w, other.g9.x, other.g3.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g5.z) * vec4(other.g9.y, other.g9.x, other.g3.w, other.g3.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g6.x) * vec4(other.g8.w, other.g1.z, other.g1.y, other.g8.w) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g6.y) * vec4(other.g1.z, other.g8.w, other.g1.x, other.g1.z) * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g6.z) * vec4(other.g1.y, other.g1.x, other.g8.w, other.g1.y) * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g7.x) * vec4(other.g2.x, other.g8.z, other.g8.y, other.g1.x) * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g7.y) * vec4(other.g8.z, other.g2.x, other.g8.x, other.g1.y) * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g7.z) * vec4(other.g8.y, other.g8.x, other.g2.x, other.g1.z) * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g8.x) * vec4(other.g2.y, other.g7.z, other.g7.y, other.g2.y) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g8.y) * vec4(other.g7.z, other.g2.y, other.g7.x, other.g7.z) * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g8.z) * vec4(other.g7.y, other.g7.x, other.g2.y, other.g7.y) * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g8.w) * vec4(other.g6.x, other.g6.y, other.g6.z, other.g2.y) + vec4(self.g9.x) * vec4(other.g0.x, other.g5.z, other.g5.y, other.g0.x) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g9.y) * vec4(other.g5.z, other.g0.x, other.g5.x, other.g5.z) * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g9.z) * vec4(other.g5.y, other.g5.x, other.g0.x, other.g5.y) * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g9.w) * vec4(other.g4.x, other.g4.y, other.g4.z, other.g0.x) + vec4(self.g0.y, self.g0.y, self.g0.y, self.g0.x) * other.g3.xyzx * vec4(-1.0, -1.0, -1.0, 0.0));
}

MultiVector multi_vector_multi_vector_regressive_product(MultiVector self, MultiVector other) {
    return MultiVector(vec3(self.g0.y) * vec3(other.g2.y, other.g0.z, other.g2.y) * vec3(1.0, 1.0, 0.0) + vec3(self.g0.z) * other.g0 + vec3(self.g1.x) * vec3(other.g9.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g1.y) * vec3(other.g9.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g1.z) * vec3(other.g9.z) * vec3(1.0, 0.0, 0.0) + vec3(self.g2.x) * vec3(other.g9.w) * vec3(1.0, 0.0, 0.0) + vec3(self.g2.y) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g3.x) * vec3(other.g8.x) * vec3(-1.0, 0.0, 0.0) + vec3(self.g3.y) * vec3(other.g8.y) * vec3(-1.0, 0.0, 0.0) + vec3(self.g3.z) * vec3(other.g8.z) * vec3(-1.0, 0.0, 0.0) + vec3(self.g3.w) * vec3(other.g8.w) * vec3(-1.0, 0.0, 0.0) + vec3(self.g4.x) * vec3(other.g7.x) * vec3(-1.0, 0.0, 0.0) + vec3(self.g4.y) * vec3(other.g7.y) * vec3(-1.0, 0.0, 0.0) + vec3(self.g4.z) * vec3(other.g7.z) * vec3(-1.0, 0.0, 0.0) + vec3(self.g5.x) * vec3(other.g6.x) * vec3(-1.0, 0.0, 0.0) + vec3(self.g5.y) * vec3(other.g6.y) * vec3(-1.0, 0.0, 0.0) + vec3(self.g5.z) * vec3(other.g6.z) * vec3(-1.0, 0.0, 0.0) + vec3(self.g6.x) * vec3(other.g5.x) * vec3(-1.0, 0.0, 0.0) + vec3(self.g6.y) * vec3(other.g5.y) * vec3(-1.0, 0.0, 0.0) + vec3(self.g6.z) * vec3(other.g5.z) * vec3(-1.0, 0.0, 0.0) + vec3(self.g7.x) * vec3(other.g4.x) * vec3(-1.0, 0.0, 0.0) + vec3(self.g7.y) * vec3(other.g4.y) * vec3(-1.0, 0.0, 0.0) + vec3(self.g7.z) * vec3(other.g4.z) * vec3(-1.0, 0.0, 0.0) + vec3(self.g8.x) * vec3(other.g3.x) * vec3(-1.0, 0.0, 0.0) + vec3(self.g8.y) * vec3(other.g3.y) * vec3(-1.0, 0.0, 0.0) + vec3(self.g8.z) * vec3(other.g3.z) * vec3(-1.0, 0.0, 0.0) + vec3(self.g8.w) * vec3(other.g3.w) * vec3(-1.0, 0.0, 0.0) + vec3(self.g9.x) * vec3(other.g1.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g9.y) * vec3(other.g1.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g9.z) * vec3(other.g1.z) * vec3(1.0, 0.0, 0.0) + vec3(self.g9.w) * vec3(other.g2.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g0.x) * other.g0.zxx * vec3(1.0, 0.0, 0.0), vec3(self.g0.y) * vec3(other.g3.x, other.g3.y, other.g3.z) + vec3(self.g0.z) * other.g1 + vec3(self.g3.x) * vec3(other.g0.y) * vec3(-1.0, 0.0, 0.0) + vec3(self.g3.y) * vec3(other.g0.y) * vec3(0.0, -1.0, 0.0) + vec3(self.g3.z) * vec3(other.g0.y) * vec3(0.0, 0.0, -1.0) + vec3(self.g4.x) * vec3(other.g9.w) * vec3(1.0, 0.0, 0.0) + vec3(self.g4.y) * vec3(other.g9.w) * vec3(0.0, 1.0, 0.0) + vec3(self.g4.z) * vec3(other.g9.w) * vec3(0.0, 0.0, 1.0) + vec3(self.g5.x) * vec3(other.g9.z, other.g9.z, other.g9.y) * vec3(0.0, -1.0, 1.0) + vec3(self.g5.y) * vec3(other.g9.z, other.g9.z, other.g9.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g5.z) * vec3(other.g9.y, other.g9.x, other.g9.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g6.x) * vec3(other.g8.w) * vec3(1.0, 0.0, 0.0) + vec3(self.g6.y) * vec3(other.g8.w) * vec3(0.0, 1.0, 0.0) + vec3(self.g6.z) * vec3(other.g8.w) * vec3(0.0, 0.0, 1.0) + vec3(self.g7.x) * vec3(other.g8.z, other.g8.z, other.g8.y) * vec3(0.0, -1.0, 1.0) + vec3(self.g7.y) * vec3(other.g8.z, other.g8.z, other.g8.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g7.z) * vec3(other.g8.y, other.g8.x, other.g8.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g8.x) * other.g7.zzy * vec3(0.0, 1.0, -1.0) + vec3(self.g8.y) * other.g7.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g8.z) * other.g7.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g8.w) * other.g6 + vec3(self.g9.x) * other.g5.zzy * vec3(0.0, -1.0, 1.0) + vec3(self.g9.y) * other.g5.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g9.z) * other.g5.yxy * vec3(-1.0, 1.0, 0.0) - vec3(self.g9.w) * other.g4 + self.g1 * vec3(other.g0.z), vec2(self.g0.z) * other.g2 + vec2(self.g2.x) * vec2(other.g0.z) * vec2(1.0, 0.0) + vec2(self.g2.y) * vec2(other.g0.z) * vec2(0.0, 1.0) + vec2(self.g3.x) * vec2(other.g9.x) * vec2(0.0, 1.0) + vec2(self.g3.y) * vec2(other.g9.y) * vec2(0.0, 1.0) + vec2(self.g3.z) * vec2(other.g9.z) * vec2(0.0, 1.0) + vec2(self.g3.w) * vec2(other.g0.y, other.g9.w) * vec2(-1.0, 1.0) + vec2(self.g4.x) * vec2(other.g9.x) * vec2(-1.0, 0.0) + vec2(self.g4.y) * vec2(other.g9.y) * vec2(-1.0, 0.0) + vec2(self.g4.z) * vec2(other.g9.z) * vec2(-1.0, 0.0) - vec2(self.g6.x) * vec2(other.g8.x, other.g7.x) - vec2(self.g6.y) * vec2(other.g8.y, other.g7.y) - vec2(self.g6.z) * vec2(other.g8.z, other.g7.z) + vec2(self.g7.x) * vec2(other.g6.x) * vec2(0.0, -1.0) + vec2(self.g7.y) * vec2(other.g6.y) * vec2(0.0, -1.0) + vec2(self.g7.z) * vec2(other.g6.z) * vec2(0.0, -1.0) + vec2(self.g8.x) * vec2(other.g6.x) * vec2(-1.0, 0.0) + vec2(self.g8.y) * vec2(other.g6.y) * vec2(-1.0, 0.0) + vec2(self.g8.z) * vec2(other.g6.z) * vec2(-1.0, 0.0) + vec2(self.g9.x) * vec2(other.g4.x, other.g3.x) * vec2(1.0, -1.0) + vec2(self.g9.y) * vec2(other.g4.y, other.g3.y) * vec2(1.0, -1.0) + vec2(self.g9.z) * vec2(other.g4.z, other.g3.z) * vec2(1.0, -1.0) + vec2(self.g9.w) * vec2(other.g3.w) * vec2(0.0, -1.0) + vec2(self.g0.y, self.g0.x) * vec2(other.g3.w, other.g3.x) * vec2(1.0, 0.0), vec4(self.g0.z) * other.g3 + vec4(self.g6.x) * other.g9.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g6.y) * other.g9.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g6.z) * other.g9.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g7.x) * other.g9.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g7.y) * other.g9.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g7.z) * other.g9.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g9.x) * vec4(other.g7.z, other.g7.z, other.g7.y, other.g6.x) * vec4(0.0, 1.0, -1.0, -1.0) + vec4(self.g9.y) * vec4(other.g7.z, other.g7.z, other.g7.x, other.g6.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g9.z) * vec4(other.g7.y, other.g7.x, other.g7.y, other.g6.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g9.w) * vec4(other.g6.x, other.g6.y, other.g6.z, other.g6.x) * vec4(1.0, 1.0, 1.0, 0.0) + self.g3 * vec4(other.g0.z), vec3(self.g0.y) * other.g6 + vec3(self.g0.z) * other.g4 + vec3(self.g6.x) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g6.y) * vec3(other.g0.y) * vec3(0.0, 1.0, 0.0) + vec3(self.g6.z) * vec3(other.g0.y) * vec3(0.0, 0.0, 1.0) + vec3(self.g8.x) * vec3(other.g9.z, other.g9.z, other.g9.y) * vec3(0.0, 1.0, -1.0) + vec3(self.g8.y) * vec3(other.g9.z, other.g9.z, other.g9.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g8.z) * vec3(other.g9.y, other.g9.x, other.g9.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g9.x) * vec3(other.g8.z, other.g8.z, other.g8.y) * vec3(0.0, -1.0, 1.0) + vec3(self.g9.y) * vec3(other.g8.z, other.g8.z, other.g8.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g9.z) * vec3(other.g8.y, other.g8.x, other.g8.y) * vec3(-1.0, 1.0, 0.0) + self.g4 * vec3(other.g0.z), vec3(self.g0.y) * other.g7 + vec3(self.g0.z) * other.g5 + vec3(self.g7.x) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g7.y) * vec3(other.g0.y) * vec3(0.0, 1.0, 0.0) + vec3(self.g7.z) * vec3(other.g0.y) * vec3(0.0, 0.0, 1.0) + vec3(self.g8.x) * vec3(other.g9.w) * vec3(1.0, 0.0, 0.0) + vec3(self.g8.y) * vec3(other.g9.w) * vec3(0.0, 1.0, 0.0) + vec3(self.g8.z) * vec3(other.g9.w) * vec3(0.0, 0.0, 1.0) - vec3(self.g8.w) * vec3(other.g9.x, other.g9.y, other.g9.z) + vec3(self.g9.x) * vec3(other.g8.w) * vec3(-1.0, 0.0, 0.0) + vec3(self.g9.y) * vec3(other.g8.w) * vec3(0.0, -1.0, 0.0) + vec3(self.g9.z) * vec3(other.g8.w) * vec3(0.0, 0.0, -1.0) + vec3(self.g9.w) * vec3(other.g8.x, other.g8.y, other.g8.z) + self.g5 * vec3(other.g0.z), vec3(self.g0.z) * other.g6 + vec3(self.g9.x) * vec3(other.g9.z, other.g9.z, other.g9.y) * vec3(0.0, 1.0, -1.0) + vec3(self.g9.y) * vec3(other.g9.z, other.g9.z, other.g9.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g9.z) * vec3(other.g9.y, other.g9.x, other.g9.y) * vec3(1.0, -1.0, 0.0) + self.g6 * vec3(other.g0.z), vec3(self.g0.z) * other.g7 + vec3(self.g9.x) * vec3(other.g9.w) * vec3(1.0, 0.0, 0.0) + vec3(self.g9.y) * vec3(other.g9.w) * vec3(0.0, 1.0, 0.0) + vec3(self.g9.z) * vec3(other.g9.w) * vec3(0.0, 0.0, 1.0) - vec3(self.g9.w) * vec3(other.g9.x, other.g9.y, other.g9.z) + self.g7 * vec3(other.g0.z), vec4(self.g0.y) * other.g9 + vec4(self.g0.z) * other.g8 + vec4(self.g9.x) * vec4(other.g0.y) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g9.y) * vec4(other.g0.y) * vec4(0.0, -1.0, 0.0, 0.0) + vec4(self.g9.z) * vec4(other.g0.y) * vec4(0.0, 0.0, -1.0, 0.0) + vec4(self.g9.w) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + self.g8 * vec4(other.g0.z), vec4(self.g0.z) * other.g9 + self.g9 * vec4(other.g0.z));
}

MultiVector multi_vector_multi_vector_outer_product(MultiVector self, MultiVector other) {
    return MultiVector(vec3(self.g0.x) * other.g0 + vec3(self.g0.z) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0) + vec3(self.g1.x) * vec3(other.g8.x, other.g8.x, other.g9.x) * vec3(0.0, 1.0, 1.0) + vec3(self.g1.y) * vec3(other.g8.y, other.g8.y, other.g9.y) * vec3(0.0, 1.0, 1.0) + vec3(self.g1.z) * vec3(other.g8.z, other.g8.z, other.g9.z) * vec3(0.0, 1.0, 1.0) + vec3(self.g2.x) * vec3(other.g8.w, other.g8.w, other.g9.w) * vec3(0.0, 1.0, 1.0) + vec3(self.g2.y) * vec3(other.g0.y) * vec3(0.0, 0.0, 1.0) + vec3(self.g3.x) * vec3(other.g8.x) * vec3(0.0, 0.0, -1.0) + vec3(self.g3.y) * vec3(other.g8.y) * vec3(0.0, 0.0, -1.0) + vec3(self.g3.z) * vec3(other.g8.z) * vec3(0.0, 0.0, -1.0) + vec3(self.g3.w) * vec3(other.g8.w) * vec3(0.0, 0.0, -1.0) + vec3(self.g4.x) * vec3(other.g5.x, other.g5.x, other.g7.x) * vec3(0.0, -1.0, -1.0) + vec3(self.g4.y) * vec3(other.g5.y, other.g5.y, other.g7.y) * vec3(0.0, -1.0, -1.0) + vec3(self.g4.z) * vec3(other.g5.z, other.g5.z, other.g7.z) * vec3(0.0, -1.0, -1.0) + vec3(self.g5.x) * vec3(other.g4.x, other.g4.x, other.g6.x) * vec3(0.0, -1.0, -1.0) + vec3(self.g5.y) * vec3(other.g4.y, other.g4.y, other.g6.y) * vec3(0.0, -1.0, -1.0) + vec3(self.g5.z) * vec3(other.g4.z, other.g4.z, other.g6.z) * vec3(0.0, -1.0, -1.0) + vec3(self.g6.x) * vec3(other.g5.x) * vec3(0.0, 0.0, -1.0) + vec3(self.g6.y) * vec3(other.g5.y) * vec3(0.0, 0.0, -1.0) + vec3(self.g6.z) * vec3(other.g5.z) * vec3(0.0, 0.0, -1.0) + vec3(self.g7.x) * vec3(other.g4.x) * vec3(0.0, 0.0, -1.0) + vec3(self.g7.y) * vec3(other.g4.y) * vec3(0.0, 0.0, -1.0) + vec3(self.g7.z) * vec3(other.g4.z) * vec3(0.0, 0.0, -1.0) + vec3(self.g8.x) * vec3(other.g1.x, other.g1.x, other.g3.x) * vec3(0.0, -1.0, -1.0) + vec3(self.g8.y) * vec3(other.g1.y, other.g1.y, other.g3.y) * vec3(0.0, -1.0, -1.0) + vec3(self.g8.z) * vec3(other.g1.z, other.g1.z, other.g3.z) * vec3(0.0, -1.0, -1.0) + vec3(self.g8.w) * vec3(other.g2.x, other.g2.x, other.g3.w) * vec3(0.0, -1.0, -1.0) + vec3(self.g9.x) * vec3(other.g1.x) * vec3(0.0, 0.0, 1.0) + vec3(self.g9.y) * vec3(other.g1.y) * vec3(0.0, 0.0, 1.0) + vec3(self.g9.z) * vec3(other.g1.z) * vec3(0.0, 0.0, 1.0) + vec3(self.g9.w) * vec3(other.g2.x) * vec3(0.0, 0.0, 1.0) + self.g0.xyy * vec3(other.g0.x, other.g0.x, other.g2.y) * vec3(0.0, 1.0, 1.0), vec3(self.g0.x) * other.g1 + self.g1 * vec3(other.g0.x), vec2(self.g0.x) * other.g2 + self.g2 * vec2(other.g0.x), vec4(self.g0.x) * other.g3 + vec4(self.g2.x) * vec4(other.g2.y) * vec4(0.0, 0.0, 0.0, 1.0) - vec4(self.g2.y) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g2.x) + vec4(self.g3.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g3.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g3.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g3.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.x) * vec4(other.g2.y, other.g2.y, other.g2.y, other.g2.x) * vec4(1.0, 1.0, 1.0, 0.0), vec3(self.g0.x) * other.g4 + vec3(self.g2.x) * other.g1 + vec3(self.g4.x) * vec3(other.g0.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g4.y) * vec3(other.g0.x) * vec3(0.0, 1.0, 0.0) + vec3(self.g4.z) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0) + self.g1 * vec3(other.g2.x) * vec3(-1.0), vec3(self.g0.x) * other.g5 + vec3(self.g1.y) * other.g1.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * other.g1.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g5.y) * vec3(other.g0.x) * vec3(0.0, 1.0, 0.0) + vec3(self.g5.z) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0) + vec3(self.g5.x, self.g1.x, self.g1.x) * vec3(other.g0.x, other.g1.z, other.g1.y) * vec3(1.0, -1.0, 1.0), vec3(self.g0.x) * other.g6 + vec3(self.g2.x) * vec3(other.g3.x, other.g3.y, other.g3.z) + vec3(self.g2.y) * other.g4 + vec3(self.g3.x) * vec3(other.g2.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g3.y) * vec3(other.g2.x) * vec3(0.0, 1.0, 0.0) + vec3(self.g3.z) * vec3(other.g2.x) * vec3(0.0, 0.0, 1.0) - vec3(self.g3.w) * other.g1 + vec3(self.g4.x) * vec3(other.g2.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g4.y) * vec3(other.g2.y) * vec3(0.0, 1.0, 0.0) + vec3(self.g4.z) * vec3(other.g2.y) * vec3(0.0, 0.0, 1.0) + vec3(self.g6.x) * vec3(other.g0.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g6.y) * vec3(other.g0.x) * vec3(0.0, 1.0, 0.0) + vec3(self.g6.z) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0) + self.g1 * vec3(other.g3.w) * vec3(-1.0), vec3(self.g0.x) * other.g7 + vec3(self.g1.y) * vec3(other.g3.z, other.g3.z, other.g3.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g3.y, other.g3.x, other.g3.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g2.y) * other.g5 + vec3(self.g3.x) * other.g1.zzy * vec3(0.0, 1.0, -1.0) + vec3(self.g3.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g3.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g5.x) * vec3(other.g2.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g5.y) * vec3(other.g2.y) * vec3(0.0, 1.0, 0.0) + vec3(self.g5.z) * vec3(other.g2.y) * vec3(0.0, 0.0, 1.0) + vec3(self.g7.x) * vec3(other.g0.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g7.y) * vec3(other.g0.x) * vec3(0.0, 1.0, 0.0) + vec3(self.g7.z) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0) + vec3(self.g1.x) * vec3(other.g3.x, other.g3.z, other.g3.y) * vec3(0.0, -1.0, 1.0), vec4(self.g0.x) * other.g8 + vec4(self.g1.y) * vec4(other.g4.z, other.g4.z, other.g4.x, other.g5.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g4.y, other.g4.x, other.g4.y, other.g5.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g2.x) * vec4(other.g5.x, other.g5.y, other.g5.z, other.g5.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g4.x) * vec4(other.g1.z, other.g1.z, other.g1.y, other.g1.z) * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g4.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.z) * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g4.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.y) * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g5.x) * vec4(other.g2.x, other.g2.x, other.g2.x, other.g1.x) * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g5.y) * vec4(other.g2.x, other.g2.x, other.g2.x, other.g1.y) * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g5.z) * vec4(other.g2.x, other.g2.x, other.g2.x, other.g1.z) * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g8.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g8.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g8.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g8.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g4.x, other.g4.z, other.g4.y, other.g5.x) * vec4(0.0, 1.0, -1.0, -1.0), vec4(self.g0.x) * other.g9 + vec4(self.g1.y) * vec4(other.g6.z, other.g6.z, other.g6.x, other.g7.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g6.y, other.g6.x, other.g6.y, other.g7.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g2.x) * vec4(other.g7.x, other.g7.y, other.g7.z, other.g7.x) * vec4(1.0, 1.0, 1.0, 0.0) - vec4(self.g2.y) * other.g8 + vec4(self.g3.x) * vec4(other.g4.z, other.g4.z, other.g4.y, other.g5.x) * vec4(0.0, 1.0, -1.0, -1.0) + vec4(self.g3.y) * vec4(other.g4.z, other.g4.z, other.g4.x, other.g5.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g3.z) * vec4(other.g4.y, other.g4.x, other.g4.y, other.g5.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g3.w) * vec4(other.g5.x, other.g5.y, other.g5.z, other.g5.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g4.x) * other.g3.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g4.y) * other.g3.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g4.z) * other.g3.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g5.x) * other.g3.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g5.y) * other.g3.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g5.z) * other.g3.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g6.x) * vec4(other.g1.z, other.g1.z, other.g1.y, other.g1.z) * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g6.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.z) * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g6.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.y) * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g7.x) * vec4(other.g2.x, other.g2.x, other.g2.x, other.g1.x) * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g7.y) * vec4(other.g2.x, other.g2.x, other.g2.x, other.g1.y) * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g7.z) * vec4(other.g2.x, other.g2.x, other.g2.x, other.g1.z) * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g8.x) * vec4(other.g2.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g8.y) * vec4(other.g2.y) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g8.z) * vec4(other.g2.y) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g8.w) * vec4(other.g2.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g9.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g9.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g9.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g9.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g6.x, other.g6.z, other.g6.y, other.g7.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

MultiVector multi_vector_multi_vector_inner_product(MultiVector self, MultiVector other) {
    return MultiVector(vec3(self.g0.x) * other.g0 + vec3(self.g1.y) * vec3(other.g1.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g1.z) * vec3(other.g1.z) * vec3(1.0, 0.0, 0.0) + vec3(self.g5.x) * vec3(other.g5.x) * vec3(-1.0, 0.0, 0.0) + vec3(self.g5.y) * vec3(other.g5.y) * vec3(-1.0, 0.0, 0.0) + vec3(self.g5.z) * vec3(other.g5.z) * vec3(-1.0, 0.0, 0.0) + vec3(self.g8.w) * vec3(other.g8.w) * vec3(-1.0, 0.0, 0.0) + vec3(self.g1.x, self.g0.y, self.g0.z) * vec3(other.g1.x, other.g0.x, other.g0.x), vec3(self.g0.x) * other.g1 + vec3(self.g1.x) * vec3(other.g0.x, other.g5.z, other.g5.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.y) * vec3(other.g5.z, other.g0.x, other.g5.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g1.z) * vec3(other.g5.y, other.g5.x, other.g0.x) * vec3(1.0, -1.0, 1.0) + vec3(self.g5.x) * vec3(other.g8.w, other.g1.z, other.g1.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g5.y) * vec3(other.g1.z, other.g8.w, other.g1.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g5.z) * vec3(other.g1.y, other.g1.x, other.g8.w) * vec3(1.0, -1.0, 1.0) + vec3(self.g8.w) * other.g5, vec2(self.g0.x) * other.g2 + vec2(self.g1.x) * vec2(other.g4.x, other.g3.x) * vec2(-1.0, 1.0) + vec2(self.g1.y) * vec2(other.g4.y, other.g3.y) * vec2(-1.0, 1.0) + vec2(self.g1.z) * vec2(other.g4.z, other.g3.z) * vec2(-1.0, 1.0) + vec2(self.g2.x) * vec2(other.g0.x) * vec2(1.0, 0.0) + vec2(self.g2.y) * vec2(other.g0.x) * vec2(0.0, 1.0) + vec2(self.g3.x) * vec2(other.g1.x) * vec2(0.0, -1.0) + vec2(self.g3.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g3.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g4.x) * vec2(other.g1.x) * vec2(1.0, 0.0) + vec2(self.g4.y) * vec2(other.g1.y) * vec2(1.0, 0.0) + vec2(self.g4.z) * vec2(other.g1.z) * vec2(1.0, 0.0) - vec2(self.g5.x) * vec2(other.g8.x, other.g7.x) - vec2(self.g5.y) * vec2(other.g8.y, other.g7.y) - vec2(self.g5.z) * vec2(other.g8.z, other.g7.z) + vec2(self.g7.x) * vec2(other.g5.x) * vec2(0.0, -1.0) + vec2(self.g7.y) * vec2(other.g5.y) * vec2(0.0, -1.0) + vec2(self.g7.z) * vec2(other.g5.z) * vec2(0.0, -1.0) + vec2(self.g8.x) * vec2(other.g5.x) * vec2(-1.0, 0.0) + vec2(self.g8.y) * vec2(other.g5.y) * vec2(-1.0, 0.0) + vec2(self.g8.z) * vec2(other.g5.z) * vec2(-1.0, 0.0) + vec2(self.g8.w) * vec2(other.g0.y, other.g9.w) * vec2(1.0, -1.0) + vec2(self.g9.w) * vec2(other.g8.w) * vec2(0.0, 1.0) + vec2(self.g0.y, self.g0.x) * vec2(other.g8.w, other.g8.x) * vec2(-1.0, 0.0), vec4(self.g0.x) * other.g3 + vec4(self.g1.x) * vec4(other.g7.z, other.g7.z, other.g7.y, other.g6.x) * vec4(0.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g7.z, other.g7.z, other.g7.x, other.g6.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g7.y, other.g7.x, other.g7.y, other.g6.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g3.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g3.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g3.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g3.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g5.x) * other.g9.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g5.y) * other.g9.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g5.z) * other.g9.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g6.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g6.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g6.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g7.x) * vec4(other.g1.z, other.g1.z, other.g1.y, other.g1.z) * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g7.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.z) * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g7.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.y) * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g8.w) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g9.x) * vec4(other.g5.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g9.y) * vec4(other.g5.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g9.z) * vec4(other.g5.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g9.w) * vec4(other.g5.x, other.g5.y, other.g5.z, other.g5.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x, self.g0.x, self.g0.x, self.g0.z) * other.g8.xxxw * vec4(0.0, 0.0, 0.0, 1.0), vec3(self.g0.x) * other.g4 + vec3(self.g0.y) * other.g5 + vec3(self.g1.y) * vec3(other.g8.z, other.g8.z, other.g8.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g8.y, other.g8.x, other.g8.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g4.x) * vec3(other.g0.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g4.y) * vec3(other.g0.x) * vec3(0.0, 1.0, 0.0) + vec3(self.g4.z) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0) + vec3(self.g5.x) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g5.y) * vec3(other.g0.y) * vec3(0.0, 1.0, 0.0) + vec3(self.g5.z) * vec3(other.g0.y) * vec3(0.0, 0.0, 1.0) + vec3(self.g8.x) * other.g1.zzy * vec3(0.0, 1.0, -1.0) + vec3(self.g8.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g8.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g1.x) * vec3(other.g8.x, other.g8.z, other.g8.y) * vec3(0.0, -1.0, 1.0), vec3(self.g0.x) * other.g5 + vec3(self.g5.x) * vec3(other.g0.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g5.y) * vec3(other.g0.x) * vec3(0.0, 1.0, 0.0) + vec3(self.g5.z) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0) - vec3(self.g8.w) * other.g1 + self.g1 * vec3(other.g8.w) * vec3(-1.0), vec3(self.g0.x) * other.g6 + vec3(self.g0.z) * other.g5 + vec3(self.g1.y) * vec3(other.g9.z, other.g9.z, other.g9.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g9.y, other.g9.x, other.g9.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g5.x) * vec3(other.g0.z) * vec3(1.0, 0.0, 0.0) + vec3(self.g5.y) * vec3(other.g0.z) * vec3(0.0, 1.0, 0.0) + vec3(self.g5.z) * vec3(other.g0.z) * vec3(0.0, 0.0, 1.0) + vec3(self.g6.x) * vec3(other.g0.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g6.y) * vec3(other.g0.x) * vec3(0.0, 1.0, 0.0) + vec3(self.g6.z) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0) + vec3(self.g9.x) * other.g1.zzy * vec3(0.0, -1.0, 1.0) + vec3(self.g9.y) * other.g1.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g9.z) * other.g1.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.x) * vec3(other.g9.x, other.g9.z, other.g9.y) * vec3(0.0, -1.0, 1.0), vec3(self.g0.x) * other.g7 + vec3(self.g7.x) * vec3(other.g0.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g7.y) * vec3(other.g0.x) * vec3(0.0, 1.0, 0.0) + vec3(self.g7.z) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0) + vec3(self.g9.w) * other.g1 + self.g1 * vec3(other.g9.w) * vec3(-1.0), vec4(self.g0.x) * other.g8 + vec4(self.g1.x) * vec4(other.g0.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.y) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g8.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g8.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g8.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g8.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.y, self.g0.y, self.g0.y, self.g0.x) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(-1.0, -1.0, -1.0, 0.0), vec4(self.g0.x) * other.g9 + vec4(self.g1.x) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.z) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g9.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g9.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g9.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g9.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.z, self.g0.z, self.g0.z, self.g0.x) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

MultiVector multi_vector_multi_vector_geometric_anti_product(MultiVector self, MultiVector other) {
    return MultiVector(vec3(self.g0.y) * vec3(other.g2.y, other.g0.z, other.g2.y) * vec3(1.0, 1.0, 0.0) + vec3(self.g0.z) * other.g0 + vec3(self.g1.x) * vec3(other.g9.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g1.y) * vec3(other.g9.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g1.z) * vec3(other.g9.z) * vec3(1.0, 0.0, 0.0) + vec3(self.g2.x) * vec3(other.g9.w, other.g3.w, other.g9.w) * vec3(1.0, -1.0, 0.0) + vec3(self.g2.y) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g3.x) * vec3(other.g8.x) * vec3(-1.0, 0.0, 0.0) + vec3(self.g3.y) * vec3(other.g8.y) * vec3(-1.0, 0.0, 0.0) + vec3(self.g3.z) * vec3(other.g8.z) * vec3(-1.0, 0.0, 0.0) + vec3(self.g3.w) * vec3(other.g8.w, other.g2.x, other.g3.w) * vec3(-1.0, 1.0, -1.0) + vec3(self.g4.x) * vec3(other.g7.x, other.g6.x, other.g7.x) * vec3(-1.0, -1.0, 0.0) + vec3(self.g4.y) * vec3(other.g7.y, other.g6.y, other.g7.y) * vec3(-1.0, -1.0, 0.0) + vec3(self.g4.z) * vec3(other.g7.z, other.g6.z, other.g7.z) * vec3(-1.0, -1.0, 0.0) + vec3(self.g5.x) * vec3(other.g6.x) * vec3(-1.0, 0.0, 0.0) + vec3(self.g5.y) * vec3(other.g6.y) * vec3(-1.0, 0.0, 0.0) + vec3(self.g5.z) * vec3(other.g6.z) * vec3(-1.0, 0.0, 0.0) - vec3(self.g6.x) * vec3(other.g5.x, other.g4.x, other.g6.x) - vec3(self.g6.y) * vec3(other.g5.y, other.g4.y, other.g6.y) - vec3(self.g6.z) * vec3(other.g5.z, other.g4.z, other.g6.z) + vec3(self.g7.x) * vec3(other.g4.x) * vec3(-1.0, 0.0, 0.0) + vec3(self.g7.y) * vec3(other.g4.y) * vec3(-1.0, 0.0, 0.0) + vec3(self.g7.z) * vec3(other.g4.z) * vec3(-1.0, 0.0, 0.0) + vec3(self.g8.x) * vec3(other.g3.x, other.g9.x, other.g3.x) * vec3(-1.0, 1.0, 0.0) + vec3(self.g8.y) * vec3(other.g3.y, other.g9.y, other.g3.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g8.z) * vec3(other.g3.z, other.g9.z, other.g3.z) * vec3(-1.0, 1.0, 0.0) + vec3(self.g8.w) * vec3(other.g3.w) * vec3(-1.0, 0.0, 0.0) + vec3(self.g9.x) * vec3(other.g1.x, other.g8.x, other.g9.x) * vec3(1.0, -1.0, 1.0) + vec3(self.g9.y) * vec3(other.g1.y, other.g8.y, other.g9.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g9.z) * vec3(other.g1.z, other.g8.z, other.g9.z) * vec3(1.0, -1.0, 1.0) + vec3(self.g9.w) * vec3(other.g2.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g0.x) * other.g0.zxx * vec3(1.0, 0.0, 0.0), vec3(self.g0.x) * vec3(other.g9.x, other.g9.y, other.g9.z) + vec3(self.g0.y) * vec3(other.g3.x, other.g3.y, other.g3.z) + vec3(self.g0.z) * other.g1 + vec3(self.g1.x) * vec3(other.g0.z, other.g6.z, other.g6.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g6.z, other.g0.z, other.g6.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.z) * vec3(other.g6.y, other.g6.x, other.g0.z) * vec3(-1.0, 1.0, 1.0) - vec3(self.g2.x) * other.g7 + vec3(self.g2.y) * vec3(other.g8.x, other.g8.y, other.g8.z) + vec3(self.g3.x) * vec3(other.g0.y, other.g4.z, other.g4.y) * vec3(-1.0, 1.0, -1.0) + vec3(self.g3.y) * vec3(other.g4.z, other.g0.y, other.g4.x) * vec3(-1.0, -1.0, 1.0) + vec3(self.g3.z) * vec3(other.g4.y, other.g4.x, other.g0.y) * vec3(1.0, -1.0, -1.0) + vec3(self.g3.w) * other.g5 + vec3(self.g4.x) * vec3(other.g9.w, other.g3.z, other.g3.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g4.y) * vec3(other.g3.z, other.g9.w, other.g3.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g4.z) * vec3(other.g3.y, other.g3.x, other.g9.w) * vec3(-1.0, 1.0, 1.0) + vec3(self.g5.x) * vec3(other.g3.w, other.g9.z, other.g9.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g5.y) * vec3(other.g9.z, other.g3.w, other.g9.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g5.z) * vec3(other.g9.y, other.g9.x, other.g3.w) * vec3(-1.0, 1.0, 1.0) + vec3(self.g6.x) * vec3(other.g8.w, other.g1.z, other.g1.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g6.y) * vec3(other.g1.z, other.g8.w, other.g1.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g6.z) * vec3(other.g1.y, other.g1.x, other.g8.w) * vec3(-1.0, 1.0, 1.0) + vec3(self.g7.x) * vec3(other.g2.x, other.g8.z, other.g8.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g7.y) * vec3(other.g8.z, other.g2.x, other.g8.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g7.z) * vec3(other.g8.y, other.g8.x, other.g2.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g8.x) * vec3(other.g2.y, other.g7.z, other.g7.y) * vec3(-1.0, 1.0, -1.0) + vec3(self.g8.y) * vec3(other.g7.z, other.g2.y, other.g7.x) * vec3(-1.0, -1.0, 1.0) + vec3(self.g8.z) * vec3(other.g7.y, other.g7.x, other.g2.y) * vec3(1.0, -1.0, -1.0) + vec3(self.g8.w) * other.g6 + vec3(self.g9.x) * vec3(other.g0.x, other.g5.z, other.g5.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g9.y) * vec3(other.g5.z, other.g0.x, other.g5.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g9.z) * vec3(other.g5.y, other.g5.x, other.g0.x) * vec3(-1.0, 1.0, 1.0) - vec3(self.g9.w) * other.g4, vec2(self.g0.z) * other.g2 + vec2(self.g2.x) * vec2(other.g0.z) * vec2(1.0, 0.0) + vec2(self.g2.y) * vec2(other.g0.z) * vec2(0.0, 1.0) + vec2(self.g3.x) * vec2(other.g9.x) * vec2(0.0, 1.0) + vec2(self.g3.y) * vec2(other.g9.y) * vec2(0.0, 1.0) + vec2(self.g3.z) * vec2(other.g9.z) * vec2(0.0, 1.0) + vec2(self.g3.w) * vec2(other.g0.y, other.g9.w) * vec2(-1.0, 1.0) + vec2(self.g4.x) * vec2(other.g9.x) * vec2(-1.0, 0.0) + vec2(self.g4.y) * vec2(other.g9.y) * vec2(-1.0, 0.0) + vec2(self.g4.z) * vec2(other.g9.z) * vec2(-1.0, 0.0) - vec2(self.g6.x) * vec2(other.g8.x, other.g7.x) - vec2(self.g6.y) * vec2(other.g8.y, other.g7.y) - vec2(self.g6.z) * vec2(other.g8.z, other.g7.z) + vec2(self.g7.x) * vec2(other.g6.x) * vec2(0.0, -1.0) + vec2(self.g7.y) * vec2(other.g6.y) * vec2(0.0, -1.0) + vec2(self.g7.z) * vec2(other.g6.z) * vec2(0.0, -1.0) + vec2(self.g8.x) * vec2(other.g6.x) * vec2(-1.0, 0.0) + vec2(self.g8.y) * vec2(other.g6.y) * vec2(-1.0, 0.0) + vec2(self.g8.z) * vec2(other.g6.z) * vec2(-1.0, 0.0) + vec2(self.g9.x) * vec2(other.g4.x, other.g3.x) * vec2(1.0, -1.0) + vec2(self.g9.y) * vec2(other.g4.y, other.g3.y) * vec2(1.0, -1.0) + vec2(self.g9.z) * vec2(other.g4.z, other.g3.z) * vec2(1.0, -1.0) + vec2(self.g9.w) * vec2(other.g3.w) * vec2(0.0, -1.0) + vec2(self.g0.y, self.g0.x) * vec2(other.g3.w, other.g3.x) * vec2(1.0, 0.0), vec4(self.g0.z) * other.g3 + vec4(self.g3.x) * vec4(other.g0.z, other.g6.z, other.g6.y, other.g0.z) * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g3.y) * vec4(other.g6.z, other.g0.z, other.g6.x, other.g6.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g3.z) * vec4(other.g6.y, other.g6.x, other.g0.z, other.g6.y) * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g3.w) * vec4(other.g7.x, other.g7.y, other.g7.z, other.g0.z) * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g6.x) * vec4(other.g9.w, other.g3.z, other.g3.y, other.g9.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g6.y) * vec4(other.g3.z, other.g9.w, other.g3.x, other.g9.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g6.z) * vec4(other.g3.y, other.g3.x, other.g9.w, other.g9.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g7.x) * vec4(other.g3.w, other.g9.z, other.g9.y, other.g3.w) * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g7.y) * vec4(other.g9.z, other.g3.w, other.g9.x, other.g9.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g7.z) * vec4(other.g9.y, other.g9.x, other.g3.w, other.g9.y) * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g9.x) * vec4(other.g2.y, other.g7.z, other.g7.y, other.g6.x) * vec4(-1.0, 1.0, -1.0, -1.0) + vec4(self.g9.y) * vec4(other.g7.z, other.g2.y, other.g7.x, other.g6.y) * vec4(-1.0, -1.0, 1.0, -1.0) + vec4(self.g9.z) * vec4(other.g7.y, other.g7.x, other.g2.y, other.g6.z) * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g9.w) * vec4(other.g6.x, other.g6.y, other.g6.z, other.g6.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g2.y, self.g2.y, self.g2.y, self.g2.x) * other.g9.xyzx * vec4(1.0, 1.0, 1.0, 0.0), vec3(self.g0.y) * other.g6 + vec3(self.g0.z) * other.g4 - vec3(self.g2.x) * vec3(other.g9.x, other.g9.y, other.g9.z) + vec3(self.g3.w) * vec3(other.g8.x, other.g8.y, other.g8.z) + vec3(self.g4.x) * vec3(other.g0.z, other.g6.z, other.g6.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g4.y) * vec3(other.g6.z, other.g0.z, other.g6.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g4.z) * vec3(other.g6.y, other.g6.x, other.g0.z) * vec3(-1.0, 1.0, 1.0) + vec3(self.g6.x) * vec3(other.g0.y, other.g4.z, other.g4.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g6.y) * vec3(other.g4.z, other.g0.y, other.g4.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g6.z) * vec3(other.g4.y, other.g4.x, other.g0.y) * vec3(-1.0, 1.0, 1.0) + vec3(self.g8.x) * vec3(other.g3.w, other.g9.z, other.g9.y) * vec3(-1.0, 1.0, -1.0) + vec3(self.g8.y) * vec3(other.g9.z, other.g3.w, other.g9.x) * vec3(-1.0, -1.0, 1.0) + vec3(self.g8.z) * vec3(other.g9.y, other.g9.x, other.g3.w) * vec3(1.0, -1.0, -1.0) + vec3(self.g9.x) * vec3(other.g2.x, other.g8.z, other.g8.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g9.y) * vec3(other.g8.z, other.g2.x, other.g8.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g9.z) * vec3(other.g8.y, other.g8.x, other.g2.x) * vec3(-1.0, 1.0, 1.0), vec3(self.g0.x) * other.g6 + vec3(self.g0.y) * other.g7 + vec3(self.g0.z) * other.g5 + vec3(self.g1.x) * vec3(other.g3.w, other.g9.z, other.g9.y) * vec3(-1.0, 1.0, -1.0) + vec3(self.g1.y) * vec3(other.g9.z, other.g3.w, other.g9.x) * vec3(-1.0, -1.0, 1.0) + vec3(self.g1.z) * vec3(other.g9.y, other.g9.x, other.g3.w) * vec3(1.0, -1.0, -1.0) + vec3(self.g2.x) * vec3(other.g3.x, other.g3.y, other.g3.z) + vec3(self.g2.y) * other.g4 + vec3(self.g3.x) * vec3(other.g2.x, other.g8.z, other.g8.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g3.y) * vec3(other.g8.z, other.g2.x, other.g8.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g3.z) * vec3(other.g8.y, other.g8.x, other.g2.x) * vec3(-1.0, 1.0, 1.0) - vec3(self.g3.w) * other.g1 + vec3(self.g4.x) * vec3(other.g2.y, other.g7.z, other.g7.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g4.y) * vec3(other.g7.z, other.g2.y, other.g7.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g4.z) * vec3(other.g7.y, other.g7.x, other.g2.y) * vec3(-1.0, 1.0, 1.0) + vec3(self.g5.x) * vec3(other.g0.z, other.g6.z, other.g6.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g5.y) * vec3(other.g6.z, other.g0.z, other.g6.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g5.z) * vec3(other.g6.y, other.g6.x, other.g0.z) * vec3(-1.0, 1.0, 1.0) + vec3(self.g6.x) * vec3(other.g0.x, other.g5.z, other.g5.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g6.y) * vec3(other.g5.z, other.g0.x, other.g5.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g6.z) * vec3(other.g5.y, other.g5.x, other.g0.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g7.x) * vec3(other.g0.y, other.g4.z, other.g4.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g7.y) * vec3(other.g4.z, other.g0.y, other.g4.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g7.z) * vec3(other.g4.y, other.g4.x, other.g0.y) * vec3(-1.0, 1.0, 1.0) + vec3(self.g8.x) * vec3(other.g9.w, other.g3.z, other.g3.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g8.y) * vec3(other.g3.z, other.g9.w, other.g3.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g8.z) * vec3(other.g3.y, other.g3.x, other.g9.w) * vec3(-1.0, 1.0, 1.0) - vec3(self.g8.w) * vec3(other.g9.x, other.g9.y, other.g9.z) + vec3(self.g9.x) * vec3(other.g8.w, other.g1.z, other.g1.y) * vec3(-1.0, 1.0, -1.0) + vec3(self.g9.y) * vec3(other.g1.z, other.g8.w, other.g1.x) * vec3(-1.0, -1.0, 1.0) + vec3(self.g9.z) * vec3(other.g1.y, other.g1.x, other.g8.w) * vec3(1.0, -1.0, -1.0) + vec3(self.g9.w) * vec3(other.g8.x, other.g8.y, other.g8.z), vec3(self.g0.z) * other.g6 - vec3(self.g3.w) * vec3(other.g9.x, other.g9.y, other.g9.z) + vec3(self.g6.x) * vec3(other.g0.z, other.g6.z, other.g6.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g6.y) * vec3(other.g6.z, other.g0.z, other.g6.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g6.z) * vec3(other.g6.y, other.g6.x, other.g0.z) * vec3(-1.0, 1.0, 1.0) + vec3(self.g9.x) * vec3(other.g3.w, other.g9.z, other.g9.y) * vec3(-1.0, 1.0, -1.0) + vec3(self.g9.y) * vec3(other.g9.z, other.g3.w, other.g9.x) * vec3(-1.0, -1.0, 1.0) + vec3(self.g9.z) * vec3(other.g9.y, other.g9.x, other.g3.w) * vec3(1.0, -1.0, -1.0), vec3(self.g0.z) * other.g7 + vec3(self.g2.y) * other.g6 + vec3(self.g3.x) * vec3(other.g3.w, other.g9.z, other.g9.y) * vec3(-1.0, 1.0, -1.0) + vec3(self.g3.y) * vec3(other.g9.z, other.g3.w, other.g9.x) * vec3(-1.0, -1.0, 1.0) + vec3(self.g3.z) * vec3(other.g9.y, other.g9.x, other.g3.w) * vec3(1.0, -1.0, -1.0) + vec3(self.g3.w) * vec3(other.g3.x, other.g3.y, other.g3.z) + vec3(self.g6.x) * vec3(other.g2.y, other.g7.z, other.g7.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g6.y) * vec3(other.g7.z, other.g2.y, other.g7.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g6.z) * vec3(other.g7.y, other.g7.x, other.g2.y) * vec3(-1.0, 1.0, 1.0) + vec3(self.g7.x) * vec3(other.g0.z, other.g6.z, other.g6.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g7.y) * vec3(other.g6.z, other.g0.z, other.g6.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g7.z) * vec3(other.g6.y, other.g6.x, other.g0.z) * vec3(-1.0, 1.0, 1.0) + vec3(self.g9.x) * vec3(other.g9.w, other.g3.z, other.g3.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g9.y) * vec3(other.g3.z, other.g9.w, other.g3.x) * vec3(1.0, 1.0, -1.0) + vec3(self.g9.z) * vec3(other.g3.y, other.g3.x, other.g9.w) * vec3(-1.0, 1.0, 1.0) - vec3(self.g9.w) * vec3(other.g9.x, other.g9.y, other.g9.z), vec4(self.g0.y) * other.g9 + vec4(self.g0.z) * other.g8 + vec4(self.g1.x) * vec4(other.g6.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * vec4(other.g6.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g6.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g2.x) * vec4(other.g6.x, other.g6.y, other.g6.z, other.g2.y) * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g2.y) * vec4(other.g2.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g3.x) * vec4(other.g4.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g3.y) * vec4(other.g4.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g3.z) * vec4(other.g4.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g3.w) * vec4(other.g4.x, other.g4.y, other.g4.z, other.g0.x) * vec4(-1.0, -1.0, -1.0, 1.0) + vec4(self.g4.x) * vec4(other.g3.w, other.g9.z, other.g9.y, other.g3.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g4.y) * vec4(other.g9.z, other.g3.w, other.g9.x, other.g3.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g4.z) * vec4(other.g9.y, other.g9.x, other.g3.w, other.g3.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g5.x) * vec4(other.g9.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g5.y) * vec4(other.g9.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g5.z) * vec4(other.g9.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g6.x) * vec4(other.g2.x, other.g8.z, other.g8.y, other.g1.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g6.y) * vec4(other.g8.z, other.g2.x, other.g8.x, other.g1.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g6.z) * vec4(other.g8.y, other.g8.x, other.g2.x, other.g1.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g7.x) * vec4(other.g8.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g7.y) * vec4(other.g8.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g7.z) * vec4(other.g8.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g8.x) * vec4(other.g0.z, other.g6.z, other.g6.y, other.g7.x) * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g8.y) * vec4(other.g6.z, other.g0.z, other.g6.x, other.g7.y) * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g8.z) * vec4(other.g6.y, other.g6.x, other.g0.z, other.g7.z) * vec4(-1.0, 1.0, 1.0, 1.0) + vec4(self.g8.w) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g9.x) * vec4(other.g0.y, other.g4.z, other.g4.y, other.g5.x) * vec4(-1.0, 1.0, -1.0, -1.0) + vec4(self.g9.y) * vec4(other.g4.z, other.g0.y, other.g4.x, other.g5.y) * vec4(-1.0, -1.0, 1.0, -1.0) + vec4(self.g9.z) * vec4(other.g4.y, other.g4.x, other.g0.y, other.g5.z) * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g9.w) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.x) * other.g3.xxxw * vec4(0.0, 0.0, 0.0, 1.0), vec4(self.g0.z) * other.g9 + vec4(self.g3.x) * vec4(other.g6.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g3.y) * vec4(other.g6.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g3.z) * vec4(other.g6.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g3.w) * vec4(other.g6.x, other.g6.y, other.g6.z, other.g2.y) * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g6.x) * vec4(other.g3.w, other.g9.z, other.g9.y, other.g3.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g6.y) * vec4(other.g9.z, other.g3.w, other.g9.x, other.g3.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g6.z) * vec4(other.g9.y, other.g9.x, other.g3.w, other.g3.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g7.x) * vec4(other.g9.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g7.y) * vec4(other.g9.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g7.z) * vec4(other.g9.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g9.x) * vec4(other.g0.z, other.g6.z, other.g6.y, other.g7.x) * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g9.y) * vec4(other.g6.z, other.g0.z, other.g6.x, other.g7.y) * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g9.z) * vec4(other.g6.y, other.g6.x, other.g0.z, other.g7.z) * vec4(-1.0, 1.0, 1.0, 1.0) + vec4(self.g9.w) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g2.x, self.g2.x, self.g2.x, self.g2.y) * other.g3.xxxw * vec4(0.0, 0.0, 0.0, 1.0));
}

MultiVector multi_vector_multi_vector_inner_anti_product(MultiVector self, MultiVector other) {
    return MultiVector(vec3(self.g0.z) * other.g0 + vec3(self.g2.x) * vec3(other.g3.w) * vec3(0.0, -1.0, 0.0) + vec3(self.g3.w) * vec3(other.g2.x, other.g2.x, other.g3.w) * vec3(0.0, 1.0, -1.0) + vec3(self.g4.x) * vec3(other.g6.x) * vec3(0.0, -1.0, 0.0) + vec3(self.g4.y) * vec3(other.g6.y) * vec3(0.0, -1.0, 0.0) + vec3(self.g4.z) * vec3(other.g6.z) * vec3(0.0, -1.0, 0.0) + vec3(self.g6.x) * vec3(other.g4.x, other.g4.x, other.g6.x) * vec3(0.0, -1.0, -1.0) + vec3(self.g6.y) * vec3(other.g4.y, other.g4.y, other.g6.y) * vec3(0.0, -1.0, -1.0) + vec3(self.g6.z) * vec3(other.g4.z, other.g4.z, other.g6.z) * vec3(0.0, -1.0, -1.0) + vec3(self.g8.x) * vec3(other.g9.x) * vec3(0.0, 1.0, 0.0) + vec3(self.g8.y) * vec3(other.g9.y) * vec3(0.0, 1.0, 0.0) + vec3(self.g8.z) * vec3(other.g9.z) * vec3(0.0, 1.0, 0.0) + vec3(self.g9.x) * vec3(other.g8.x, other.g8.x, other.g9.x) * vec3(0.0, -1.0, 1.0) + vec3(self.g9.y) * vec3(other.g8.y, other.g8.y, other.g9.y) * vec3(0.0, -1.0, 1.0) + vec3(self.g9.z) * vec3(other.g8.z, other.g8.z, other.g9.z) * vec3(0.0, -1.0, 1.0) + self.g0.xyx * other.g0.zzx * vec3(1.0, 1.0, 0.0), vec3(self.g0.x) * vec3(other.g9.x, other.g9.y, other.g9.z) + vec3(self.g0.z) * other.g1 + vec3(self.g9.x) * vec3(other.g0.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g9.y) * vec3(other.g0.x) * vec3(0.0, 1.0, 0.0) + vec3(self.g9.z) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0) + self.g1 * vec3(other.g0.z), vec2(self.g0.z) * other.g2 + self.g2 * vec2(other.g0.z), vec4(self.g0.z) * other.g3 + vec4(self.g3.x) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g3.y) * vec4(other.g0.z) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g3.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g3.w) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g9.x) * vec4(other.g2.y) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g9.y) * vec4(other.g2.y) * vec4(0.0, -1.0, 0.0, 0.0) + vec4(self.g9.z) * vec4(other.g2.y) * vec4(0.0, 0.0, -1.0, 0.0) + vec4(self.g2.y, self.g2.y, self.g2.y, self.g2.x) * other.g9.xyzx * vec4(1.0, 1.0, 1.0, 0.0), vec3(self.g0.z) * other.g4 - vec3(self.g2.x) * vec3(other.g9.x, other.g9.y, other.g9.z) + vec3(self.g9.x) * vec3(other.g2.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g9.y) * vec3(other.g2.x) * vec3(0.0, 1.0, 0.0) + vec3(self.g9.z) * vec3(other.g2.x) * vec3(0.0, 0.0, 1.0) + self.g4 * vec3(other.g0.z), vec3(self.g0.x) * other.g6 + vec3(self.g0.z) * other.g5 + vec3(self.g1.y) * vec3(other.g9.z, other.g9.z, other.g9.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * vec3(other.g9.y, other.g9.x, other.g9.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g5.x) * vec3(other.g0.z) * vec3(1.0, 0.0, 0.0) + vec3(self.g5.y) * vec3(other.g0.z) * vec3(0.0, 1.0, 0.0) + vec3(self.g5.z) * vec3(other.g0.z) * vec3(0.0, 0.0, 1.0) + vec3(self.g6.x) * vec3(other.g0.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g6.y) * vec3(other.g0.x) * vec3(0.0, 1.0, 0.0) + vec3(self.g6.z) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0) + vec3(self.g9.x) * other.g1.zzy * vec3(0.0, 1.0, -1.0) + vec3(self.g9.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g9.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g1.x) * vec3(other.g9.x, other.g9.z, other.g9.y) * vec3(0.0, 1.0, -1.0), vec3(self.g0.z) * other.g6 - vec3(self.g3.w) * vec3(other.g9.x, other.g9.y, other.g9.z) + vec3(self.g9.x) * vec3(other.g3.w) * vec3(-1.0, 0.0, 0.0) + vec3(self.g9.y) * vec3(other.g3.w) * vec3(0.0, -1.0, 0.0) + vec3(self.g9.z) * vec3(other.g3.w) * vec3(0.0, 0.0, -1.0) + self.g6 * vec3(other.g0.z), vec3(self.g0.z) * other.g7 + vec3(self.g2.y) * other.g6 + vec3(self.g3.y) * vec3(other.g9.z, other.g9.z, other.g9.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g3.z) * vec3(other.g9.y, other.g9.x, other.g9.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g6.x) * vec3(other.g2.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g6.y) * vec3(other.g2.y) * vec3(0.0, 1.0, 0.0) + vec3(self.g6.z) * vec3(other.g2.y) * vec3(0.0, 0.0, 1.0) + vec3(self.g7.x) * vec3(other.g0.z) * vec3(1.0, 0.0, 0.0) + vec3(self.g7.y) * vec3(other.g0.z) * vec3(0.0, 1.0, 0.0) + vec3(self.g7.z) * vec3(other.g0.z) * vec3(0.0, 0.0, 1.0) + vec3(self.g9.x) * vec3(other.g3.z, other.g3.z, other.g3.y) * vec3(0.0, -1.0, 1.0) + vec3(self.g9.y) * vec3(other.g3.z, other.g3.z, other.g3.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g9.z) * vec3(other.g3.y, other.g3.x, other.g3.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g3.x) * vec3(other.g9.x, other.g9.z, other.g9.y) * vec3(0.0, 1.0, -1.0), vec4(self.g0.z) * other.g8 + vec4(self.g1.x) * vec4(other.g6.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * vec4(other.g6.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g6.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g2.x) * vec4(other.g6.x, other.g6.y, other.g6.z, other.g6.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g3.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g4.y) * other.g9.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g4.z) * other.g9.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g5.x) * vec4(other.g9.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g5.y) * vec4(other.g9.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g5.z) * vec4(other.g9.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g6.x) * vec4(other.g2.x, other.g2.x, other.g2.x, other.g1.x) * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g6.y) * vec4(other.g2.x, other.g2.x, other.g2.x, other.g1.y) * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g6.z) * vec4(other.g2.x, other.g2.x, other.g2.x, other.g1.z) * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g8.x) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g8.y) * vec4(other.g0.z) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g8.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g8.w) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g9.x) * vec4(other.g4.z, other.g4.z, other.g4.y, other.g5.x) * vec4(0.0, 1.0, -1.0, -1.0) + vec4(self.g9.y) * vec4(other.g4.z, other.g4.z, other.g4.x, other.g5.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g9.z) * vec4(other.g4.y, other.g4.x, other.g4.y, other.g5.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.x, self.g4.x, self.g4.x, self.g0.x) * vec4(other.g3.x, other.g9.z, other.g9.y, other.g3.w) * vec4(0.0, -1.0, 1.0, 1.0), vec4(self.g0.z) * other.g9 + vec4(self.g3.x) * vec4(other.g6.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g3.y) * vec4(other.g6.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g3.z) * vec4(other.g6.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g3.w) * vec4(other.g6.x, other.g6.y, other.g6.z, other.g2.y) * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g6.x) * vec4(other.g3.w, other.g9.z, other.g9.y, other.g3.x) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g6.y) * vec4(other.g9.z, other.g3.w, other.g9.x, other.g3.y) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g6.z) * vec4(other.g9.y, other.g9.x, other.g3.w, other.g3.z) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g7.x) * vec4(other.g9.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g7.y) * vec4(other.g9.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g7.z) * vec4(other.g9.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g9.x) * vec4(other.g0.z, other.g6.z, other.g6.y, other.g7.x) * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g9.y) * vec4(other.g6.z, other.g0.z, other.g6.x, other.g7.y) * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g9.z) * vec4(other.g6.y, other.g6.x, other.g0.z, other.g7.z) * vec4(-1.0, 1.0, 1.0, 1.0) + vec4(self.g9.w) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g2.x, self.g2.x, self.g2.x, self.g2.y) * other.g3.xxxw * vec4(0.0, 0.0, 0.0, 1.0));
}

MultiVector multi_vector_multi_vector_left_contraction(MultiVector self, MultiVector other) {
    return MultiVector(vec3(self.g0.x) * other.g0 + vec3(self.g1.y) * vec3(other.g1.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g1.z) * vec3(other.g1.z) * vec3(1.0, 0.0, 0.0) + vec3(self.g5.x) * vec3(other.g5.x) * vec3(-1.0, 0.0, 0.0) + vec3(self.g5.y) * vec3(other.g5.y) * vec3(-1.0, 0.0, 0.0) + vec3(self.g5.z) * vec3(other.g5.z) * vec3(-1.0, 0.0, 0.0) + vec3(self.g8.w) * vec3(other.g8.w) * vec3(-1.0, 0.0, 0.0) + vec3(self.g1.x) * vec3(other.g1.x) * vec3(1.0, 0.0, 0.0), vec3(self.g0.x) * other.g1 + vec3(self.g1.y) * other.g5.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * other.g5.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g5.x) * vec3(other.g8.w) * vec3(1.0, 0.0, 0.0) + vec3(self.g5.y) * vec3(other.g8.w) * vec3(0.0, 1.0, 0.0) + vec3(self.g5.z) * vec3(other.g8.w) * vec3(0.0, 0.0, 1.0) + vec3(self.g1.x) * other.g5.xzy * vec3(0.0, 1.0, -1.0), vec2(self.g0.x) * other.g2 + vec2(self.g1.x) * vec2(other.g4.x, other.g3.x) * vec2(-1.0, 1.0) + vec2(self.g1.y) * vec2(other.g4.y, other.g3.y) * vec2(-1.0, 1.0) + vec2(self.g1.z) * vec2(other.g4.z, other.g3.z) * vec2(-1.0, 1.0) - vec2(self.g5.x) * vec2(other.g8.x, other.g7.x) - vec2(self.g5.y) * vec2(other.g8.y, other.g7.y) - vec2(self.g5.z) * vec2(other.g8.z, other.g7.z) + vec2(self.g8.w) * vec2(other.g0.y, other.g9.w) * vec2(1.0, -1.0), vec4(self.g0.x) * other.g3 + vec4(self.g1.y) * vec4(other.g7.z, other.g7.z, other.g7.x, other.g6.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g7.y, other.g7.x, other.g7.y, other.g6.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g5.x) * other.g9.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g5.y) * other.g9.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g5.z) * other.g9.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g8.w) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g7.x, other.g7.z, other.g7.y, other.g6.x) * vec4(0.0, 1.0, -1.0, -1.0), vec3(self.g0.x) * other.g4 + vec3(self.g1.y) * vec3(other.g8.z, other.g8.z, other.g8.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g8.y, other.g8.x, other.g8.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g5.x) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g5.y) * vec3(other.g0.y) * vec3(0.0, 1.0, 0.0) + vec3(self.g5.z) * vec3(other.g0.y) * vec3(0.0, 0.0, 1.0) + vec3(self.g1.x) * vec3(other.g8.x, other.g8.z, other.g8.y) * vec3(0.0, -1.0, 1.0), vec3(self.g0.x) * other.g5 + self.g1 * vec3(other.g8.w) * vec3(-1.0), vec3(self.g0.x) * other.g6 + vec3(self.g1.y) * vec3(other.g9.z, other.g9.z, other.g9.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g9.y, other.g9.x, other.g9.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g5.x) * vec3(other.g0.z) * vec3(1.0, 0.0, 0.0) + vec3(self.g5.y) * vec3(other.g0.z) * vec3(0.0, 1.0, 0.0) + vec3(self.g5.z) * vec3(other.g0.z) * vec3(0.0, 0.0, 1.0) + vec3(self.g1.x) * vec3(other.g9.x, other.g9.z, other.g9.y) * vec3(0.0, -1.0, 1.0), vec3(self.g0.x) * other.g7 + self.g1 * vec3(other.g9.w) * vec3(-1.0), vec4(self.g0.x) * other.g8 + vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.x) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0), vec4(self.g0.x) * other.g9 + vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.x) * vec4(other.g0.z, other.g0.z, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

MultiVector multi_vector_multi_vector_right_contraction(MultiVector self, MultiVector other) {
    return MultiVector(vec3(self.g1.x) * vec3(other.g1.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g1.y) * vec3(other.g1.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g1.z) * vec3(other.g1.z) * vec3(1.0, 0.0, 0.0) + vec3(self.g5.x) * vec3(other.g5.x) * vec3(-1.0, 0.0, 0.0) + vec3(self.g5.y) * vec3(other.g5.y) * vec3(-1.0, 0.0, 0.0) + vec3(self.g5.z) * vec3(other.g5.z) * vec3(-1.0, 0.0, 0.0) + vec3(self.g8.w) * vec3(other.g8.w) * vec3(-1.0, 0.0, 0.0) + self.g0 * vec3(other.g0.x), vec3(self.g5.x) * other.g1.zzy * vec3(0.0, 1.0, -1.0) + vec3(self.g5.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g5.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g8.w) * other.g5 + self.g1 * vec3(other.g0.x), vec2(self.g2.x) * vec2(other.g0.x) * vec2(1.0, 0.0) + vec2(self.g2.y) * vec2(other.g0.x) * vec2(0.0, 1.0) + vec2(self.g3.x) * vec2(other.g1.x) * vec2(0.0, -1.0) + vec2(self.g3.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g3.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g4.x) * vec2(other.g1.x) * vec2(1.0, 0.0) + vec2(self.g4.y) * vec2(other.g1.y) * vec2(1.0, 0.0) + vec2(self.g4.z) * vec2(other.g1.z) * vec2(1.0, 0.0) + vec2(self.g7.x) * vec2(other.g5.x) * vec2(0.0, -1.0) + vec2(self.g7.y) * vec2(other.g5.y) * vec2(0.0, -1.0) + vec2(self.g7.z) * vec2(other.g5.z) * vec2(0.0, -1.0) + vec2(self.g8.x) * vec2(other.g5.x) * vec2(-1.0, 0.0) + vec2(self.g8.y) * vec2(other.g5.y) * vec2(-1.0, 0.0) + vec2(self.g8.z) * vec2(other.g5.z) * vec2(-1.0, 0.0) + vec2(self.g9.w) * vec2(other.g8.w) * vec2(0.0, 1.0) + vec2(self.g0.y, self.g0.x) * vec2(other.g8.w, other.g8.x) * vec2(-1.0, 0.0), vec4(self.g3.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g3.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g3.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g3.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g6.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g6.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g6.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g7.x) * vec4(other.g1.z, other.g1.z, other.g1.y, other.g1.z) * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g7.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.z) * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g7.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.y) * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g9.x) * vec4(other.g5.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g9.y) * vec4(other.g5.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g9.z) * vec4(other.g5.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g9.w) * vec4(other.g5.x, other.g5.y, other.g5.z, other.g5.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x, self.g0.x, self.g0.x, self.g0.z) * other.g8.xxxw * vec4(0.0, 0.0, 0.0, 1.0), vec3(self.g0.y) * other.g5 + vec3(self.g8.x) * other.g1.zzy * vec3(0.0, 1.0, -1.0) + vec3(self.g8.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g8.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + self.g4 * vec3(other.g0.x), vec3(0.0) - vec3(self.g8.w) * other.g1 + self.g5 * vec3(other.g0.x), vec3(self.g0.z) * other.g5 + vec3(self.g9.x) * other.g1.zzy * vec3(0.0, -1.0, 1.0) + vec3(self.g9.y) * other.g1.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g9.z) * other.g1.yxy * vec3(-1.0, 1.0, 0.0) + self.g6 * vec3(other.g0.x), vec3(self.g9.w) * other.g1 + self.g7 * vec3(other.g0.x), vec4(self.g8.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g8.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g8.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g8.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.y, self.g0.y, self.g0.y, self.g0.x) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(-1.0, -1.0, -1.0, 0.0), vec4(self.g9.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g9.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g9.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g9.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.z, self.g0.z, self.g0.z, self.g0.x) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

MultiVector multi_vector_multi_vector_left_anti_contraction(MultiVector self, MultiVector other) {
    return MultiVector(vec3(self.g0.z) * other.g0 + vec3(self.g6.x) * vec3(other.g4.x, other.g4.x, other.g6.x) * vec3(0.0, -1.0, -1.0) + vec3(self.g6.y) * vec3(other.g4.y, other.g4.y, other.g6.y) * vec3(0.0, -1.0, -1.0) + vec3(self.g6.z) * vec3(other.g4.z, other.g4.z, other.g6.z) * vec3(0.0, -1.0, -1.0) + vec3(self.g9.x) * vec3(other.g8.x, other.g8.x, other.g9.x) * vec3(0.0, -1.0, 1.0) + vec3(self.g9.y) * vec3(other.g8.y, other.g8.y, other.g9.y) * vec3(0.0, -1.0, 1.0) + vec3(self.g9.z) * vec3(other.g8.z, other.g8.z, other.g9.z) * vec3(0.0, -1.0, 1.0) + vec3(self.g3.x, self.g3.w, self.g3.w) * vec3(other.g2.x, other.g2.x, other.g3.w) * vec3(0.0, 1.0, -1.0), vec3(self.g0.z) * other.g1 + vec3(self.g9.x, self.g9.y, self.g9.z) * vec3(other.g0.x), vec2(self.g0.z) * other.g2, vec4(self.g0.z) * other.g3 + self.g9.xyzx * vec4(other.g2.y, other.g2.y, other.g2.y, other.g2.x) * vec4(-1.0, -1.0, -1.0, 0.0), vec3(self.g0.z) * other.g4 + vec3(self.g9.x, self.g9.y, self.g9.z) * vec3(other.g2.x), vec3(self.g0.z) * other.g5 + vec3(self.g9.x) * other.g1.zzy * vec3(0.0, 1.0, -1.0) + vec3(self.g9.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g9.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + self.g6 * vec3(other.g0.x), vec3(self.g0.z) * other.g6 + vec3(self.g9.x, self.g9.y, self.g9.z) * vec3(other.g3.w) * vec3(-1.0), vec3(self.g0.z) * other.g7 + vec3(self.g9.x) * vec3(other.g3.z, other.g3.z, other.g3.y) * vec3(0.0, -1.0, 1.0) + vec3(self.g9.y) * vec3(other.g3.z, other.g3.z, other.g3.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g9.z) * vec3(other.g3.y, other.g3.x, other.g3.y) * vec3(-1.0, 1.0, 0.0) + self.g6 * vec3(other.g2.y), vec4(self.g0.z) * other.g8 + vec4(self.g6.x) * vec4(other.g2.x, other.g2.x, other.g2.x, other.g1.x) * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g6.y) * vec4(other.g2.x, other.g2.x, other.g2.x, other.g1.y) * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g6.z) * vec4(other.g2.x, other.g2.x, other.g2.x, other.g1.z) * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g9.x) * vec4(other.g4.z, other.g4.z, other.g4.y, other.g5.x) * vec4(0.0, 1.0, -1.0, -1.0) + vec4(self.g9.y) * vec4(other.g4.z, other.g4.z, other.g4.x, other.g5.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g9.z) * vec4(other.g4.y, other.g4.x, other.g4.y, other.g5.z) * vec4(1.0, -1.0, 0.0, -1.0) + self.g3.xxxw * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0), vec4(self.g0.z) * other.g9 + vec4(self.g6.x) * other.g3.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g6.y) * other.g3.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g6.z) * other.g3.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g9.x) * vec4(other.g6.z, other.g6.z, other.g6.y, other.g7.x) * vec4(0.0, -1.0, 1.0, 1.0) + vec4(self.g9.y) * vec4(other.g6.z, other.g6.z, other.g6.x, other.g7.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g9.z) * vec4(other.g6.y, other.g6.x, other.g6.y, other.g7.z) * vec4(-1.0, 1.0, 0.0, 1.0) + self.g3.xxxw * vec4(other.g2.x, other.g2.x, other.g2.x, other.g2.y) * vec4(0.0, 0.0, 0.0, -1.0));
}

MultiVector multi_vector_multi_vector_right_anti_contraction(MultiVector self, MultiVector other) {
    return MultiVector(vec3(self.g2.x) * vec3(other.g3.w) * vec3(0.0, -1.0, 0.0) + vec3(self.g3.w) * vec3(other.g3.w) * vec3(0.0, 0.0, -1.0) + vec3(self.g4.x) * vec3(other.g6.x) * vec3(0.0, -1.0, 0.0) + vec3(self.g4.y) * vec3(other.g6.y) * vec3(0.0, -1.0, 0.0) + vec3(self.g4.z) * vec3(other.g6.z) * vec3(0.0, -1.0, 0.0) + vec3(self.g6.x) * vec3(other.g6.x) * vec3(0.0, 0.0, -1.0) + vec3(self.g6.y) * vec3(other.g6.y) * vec3(0.0, 0.0, -1.0) + vec3(self.g6.z) * vec3(other.g6.z) * vec3(0.0, 0.0, -1.0) + vec3(self.g8.x) * vec3(other.g9.x) * vec3(0.0, 1.0, 0.0) + vec3(self.g8.y) * vec3(other.g9.y) * vec3(0.0, 1.0, 0.0) + vec3(self.g8.z) * vec3(other.g9.z) * vec3(0.0, 1.0, 0.0) + vec3(self.g9.x) * vec3(other.g9.x) * vec3(0.0, 0.0, 1.0) + vec3(self.g9.y) * vec3(other.g9.y) * vec3(0.0, 0.0, 1.0) + vec3(self.g9.z) * vec3(other.g9.z) * vec3(0.0, 0.0, 1.0) + self.g0 * vec3(other.g0.z), vec3(self.g0.x) * vec3(other.g9.x, other.g9.y, other.g9.z) + self.g1 * vec3(other.g0.z), self.g2 * vec2(other.g0.z), vec4(self.g3.x) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g3.y) * vec4(other.g0.z) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g3.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g3.w) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g2.y, self.g2.y, self.g2.y, self.g2.x) * other.g9.xyzx * vec4(1.0, 1.0, 1.0, 0.0), vec3(0.0) - vec3(self.g2.x) * vec3(other.g9.x, other.g9.y, other.g9.z) + self.g4 * vec3(other.g0.z), vec3(self.g0.x) * other.g6 + vec3(self.g1.y) * vec3(other.g9.z, other.g9.z, other.g9.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * vec3(other.g9.y, other.g9.x, other.g9.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g5.x) * vec3(other.g0.z) * vec3(1.0, 0.0, 0.0) + vec3(self.g5.y) * vec3(other.g0.z) * vec3(0.0, 1.0, 0.0) + vec3(self.g5.z) * vec3(other.g0.z) * vec3(0.0, 0.0, 1.0) + vec3(self.g1.x) * vec3(other.g9.x, other.g9.z, other.g9.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g3.w) * vec3(other.g9.x, other.g9.y, other.g9.z) + self.g6 * vec3(other.g0.z), vec3(self.g2.y) * other.g6 + vec3(self.g3.y) * vec3(other.g9.z, other.g9.z, other.g9.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g3.z) * vec3(other.g9.y, other.g9.x, other.g9.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g7.x) * vec3(other.g0.z) * vec3(1.0, 0.0, 0.0) + vec3(self.g7.y) * vec3(other.g0.z) * vec3(0.0, 1.0, 0.0) + vec3(self.g7.z) * vec3(other.g0.z) * vec3(0.0, 0.0, 1.0) + vec3(self.g3.x) * vec3(other.g9.x, other.g9.z, other.g9.y) * vec3(0.0, 1.0, -1.0), vec4(self.g1.x) * vec4(other.g6.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * vec4(other.g6.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g6.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g2.x) * vec4(other.g6.x, other.g6.y, other.g6.z, other.g6.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g4.y) * other.g9.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g4.z) * other.g9.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g5.x) * vec4(other.g9.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g5.y) * vec4(other.g9.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g5.z) * vec4(other.g9.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g8.x) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g8.y) * vec4(other.g0.z) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g8.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g8.w) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x, self.g4.x, self.g4.x, self.g0.x) * vec4(other.g3.x, other.g9.z, other.g9.y, other.g3.w) * vec4(0.0, -1.0, 1.0, 1.0), vec4(self.g3.x) * vec4(other.g6.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g3.y) * vec4(other.g6.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g3.z) * vec4(other.g6.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g3.w) * vec4(other.g6.x, other.g6.y, other.g6.z, other.g6.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g6.x) * other.g9.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g6.y) * other.g9.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g6.z) * other.g9.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g7.x) * vec4(other.g9.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g7.y) * vec4(other.g9.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g7.z) * vec4(other.g9.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g9.x) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g9.y) * vec4(other.g0.z) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g9.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g9.w) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g2.x, self.g2.x, self.g2.x, self.g2.y) * other.g3.xxxw * vec4(0.0, 0.0, 0.0, 1.0));
}

Scalar multi_vector_multi_vector_scalar_product(MultiVector self, MultiVector other) {
    return Scalar(self.g0.x * other.g0.x + self.g1.x * other.g1.x + self.g1.y * other.g1.y + self.g1.z * other.g1.z - self.g5.x * other.g5.x - self.g5.y * other.g5.y - self.g5.z * other.g5.z - self.g8.w * other.g8.w);
}

AntiScalar multi_vector_multi_vector_anti_scalar_product(MultiVector self, MultiVector other) {
    return AntiScalar(self.g0.z * other.g0.z - self.g3.w * other.g3.w - self.g6.x * other.g6.x - self.g6.y * other.g6.y - self.g6.z * other.g6.z + self.g9.x * other.g9.x + self.g9.y * other.g9.y + self.g9.z * other.g9.z);
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

AntiScalar multi_vector_squared_anti_magnitude(MultiVector self) {
    return multi_vector_multi_vector_anti_scalar_product(self, multi_vector_anti_reversal(self));
}

AntiScalar multi_vector_weight_norm(MultiVector self) {
    return AntiScalar(sqrt(multi_vector_squared_anti_magnitude(self).g0));
}

HomogeneousMagnitude multi_vector_geometric_norm(MultiVector self) {
    return scalar_anti_scalar_add(multi_vector_bulk_norm(self), multi_vector_weight_norm(self));
}

MultiVector multi_vector_scale(MultiVector self, float other) {
    return multi_vector_scalar_geometric_product(self, Scalar(other));
}

MultiVector multi_vector_signum(MultiVector self) {
    return multi_vector_scalar_geometric_product(self, Scalar(1.0 / multi_vector_magnitude(self).g0));
}

MultiVector multi_vector_inverse(MultiVector self) {
    return multi_vector_scalar_geometric_product(multi_vector_reversal(self), Scalar(1.0 / multi_vector_squared_magnitude(self).g0));
}

MultiVector multi_vector_unitize(MultiVector self) {
    return multi_vector_scalar_geometric_product(self, Scalar(1.0 / multi_vector_weight_norm(self).g0));
}

AntiScalar anti_scalar_homogeneous_magnitude_geometric_quotient(AntiScalar self, HomogeneousMagnitude other) {
    return anti_scalar_homogeneous_magnitude_geometric_product(self, homogeneous_magnitude_inverse(other));
}

AntiScalar anti_scalar_scalar_geometric_quotient(AntiScalar self, Scalar other) {
    return anti_scalar_scalar_geometric_product(self, scalar_inverse(other));
}

Flector circle_flector_transformation(Circle self, Flector other) {
    return motor_circle_geometric_product(circle_flector_geometric_product(self, other), circle_reversal(self));
}

Motor circle_motor_transformation(Circle self, Motor other) {
    return flector_circle_geometric_product(circle_motor_geometric_product(self, other), circle_reversal(self));
}

MultiVector circle_multi_vector_geometric_quotient(Circle self, MultiVector other) {
    return circle_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

MultiVector circle_multi_vector_transformation(Circle self, MultiVector other) {
    return multi_vector_circle_geometric_product(circle_multi_vector_geometric_product(self, other), circle_reversal(self));
}

Circle circle_scalar_geometric_quotient(Circle self, Scalar other) {
    return circle_scalar_geometric_product(self, scalar_inverse(other));
}

Flector dilation_flector_transformation(Dilation self, Flector other) {
    return motor_dilation_geometric_product(dilation_flector_geometric_product(self, other), dilation_reversal(self));
}

Motor dilation_motor_transformation(Dilation self, Motor other) {
    return flector_dilation_geometric_product(dilation_motor_geometric_product(self, other), dilation_reversal(self));
}

MultiVector dilation_multi_vector_geometric_quotient(Dilation self, MultiVector other) {
    return dilation_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

MultiVector dilation_multi_vector_transformation(Dilation self, MultiVector other) {
    return multi_vector_dilation_geometric_product(dilation_multi_vector_geometric_product(self, other), dilation_reversal(self));
}

Dilation dilation_scalar_geometric_quotient(Dilation self, Scalar other) {
    return dilation_scalar_geometric_product(self, scalar_inverse(other));
}

FlatPoint dipole_flat_point_transformation(Dipole self, FlatPoint other) {
    return flector_flat_point_into(flector_dipole_geometric_product(dipole_flat_point_geometric_product(self, other), dipole_reversal(self)));
}

Flector dipole_flector_transformation(Dipole self, Flector other) {
    return flector_dipole_geometric_product(dipole_flector_geometric_product(self, other), dipole_reversal(self));
}

Line dipole_line_transformation(Dipole self, Line other) {
    return motor_line_into(motor_dipole_geometric_product(dipole_line_geometric_product(self, other), dipole_reversal(self)));
}

Motor dipole_motor_transformation(Dipole self, Motor other) {
    return motor_dipole_geometric_product(dipole_motor_geometric_product(self, other), dipole_reversal(self));
}

MultiVector dipole_multi_vector_geometric_quotient(Dipole self, MultiVector other) {
    return dipole_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

MultiVector dipole_multi_vector_transformation(Dipole self, MultiVector other) {
    return multi_vector_dipole_geometric_product(dipole_multi_vector_geometric_product(self, other), dipole_reversal(self));
}

Rotor dipole_rotor_transformation(Dipole self, Rotor other) {
    return rotor_dipole_geometric_product(dipole_rotor_geometric_product(self, other), dipole_reversal(self));
}

Dipole dipole_scalar_geometric_quotient(Dipole self, Scalar other) {
    return dipole_scalar_geometric_product(self, scalar_inverse(other));
}

Translator dipole_translator_transformation(Dipole self, Translator other) {
    return motor_translator_into(motor_dipole_geometric_product(dipole_translator_geometric_product(self, other), dipole_reversal(self)));
}

Flector flat_point_dipole_geometric_quotient(FlatPoint self, Dipole other) {
    return flat_point_dipole_geometric_product(self, dipole_inverse(other));
}

FlatPoint flat_point_homogeneous_magnitude_geometric_quotient(FlatPoint self, HomogeneousMagnitude other) {
    return flat_point_homogeneous_magnitude_geometric_product(self, homogeneous_magnitude_inverse(other));
}

FlatPoint flat_point_scalar_geometric_quotient(FlatPoint self, Scalar other) {
    return flat_point_scalar_geometric_product(self, scalar_inverse(other));
}

Motor flector_circle_geometric_quotient(Flector self, Circle other) {
    return flector_circle_geometric_product(self, circle_inverse(other));
}

Motor flector_dilation_geometric_quotient(Flector self, Dilation other) {
    return flector_dilation_geometric_product(self, dilation_inverse(other));
}

Flector flector_dipole_geometric_quotient(Flector self, Dipole other) {
    return flector_dipole_geometric_product(self, dipole_inverse(other));
}

Flector flector_homogeneous_magnitude_geometric_quotient(Flector self, HomogeneousMagnitude other) {
    return flector_homogeneous_magnitude_geometric_product(self, homogeneous_magnitude_inverse(other));
}

Motor flector_radial_point_geometric_quotient(Flector self, RadialPoint other) {
    return flector_radial_point_geometric_product(self, radial_point_inverse(other));
}

Flector flector_scalar_geometric_quotient(Flector self, Scalar other) {
    return flector_scalar_geometric_product(self, scalar_inverse(other));
}

AntiScalar homogeneous_magnitude_anti_scalar_transformation(HomogeneousMagnitude self, AntiScalar other) {
    return anti_scalar_homogeneous_magnitude_geometric_product(homogeneous_magnitude_anti_scalar_geometric_product(self, other), homogeneous_magnitude_reversal(self));
}

FlatPoint homogeneous_magnitude_flat_point_transformation(HomogeneousMagnitude self, FlatPoint other) {
    return flat_point_homogeneous_magnitude_geometric_product(homogeneous_magnitude_flat_point_geometric_product(self, other), homogeneous_magnitude_reversal(self));
}

Flector homogeneous_magnitude_flector_transformation(HomogeneousMagnitude self, Flector other) {
    return flector_homogeneous_magnitude_geometric_product(homogeneous_magnitude_flector_geometric_product(self, other), homogeneous_magnitude_reversal(self));
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

HomogeneousMagnitude homogeneous_magnitude_homogeneous_magnitude_geometric_quotient(HomogeneousMagnitude self, HomogeneousMagnitude other) {
    return homogeneous_magnitude_homogeneous_magnitude_geometric_product(self, homogeneous_magnitude_inverse(other));
}

HomogeneousMagnitude homogeneous_magnitude_homogeneous_magnitude_transformation(HomogeneousMagnitude self, HomogeneousMagnitude other) {
    return homogeneous_magnitude_homogeneous_magnitude_geometric_product(homogeneous_magnitude_homogeneous_magnitude_geometric_product(self, other), homogeneous_magnitude_reversal(self));
}

Line homogeneous_magnitude_line_transformation(HomogeneousMagnitude self, Line other) {
    return line_homogeneous_magnitude_geometric_product(homogeneous_magnitude_line_geometric_product(self, other), homogeneous_magnitude_reversal(self));
}

Motor homogeneous_magnitude_motor_transformation(HomogeneousMagnitude self, Motor other) {
    return motor_homogeneous_magnitude_geometric_product(homogeneous_magnitude_motor_geometric_product(self, other), homogeneous_magnitude_reversal(self));
}

MultiVector homogeneous_magnitude_multi_vector_geometric_quotient(HomogeneousMagnitude self, MultiVector other) {
    return homogeneous_magnitude_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

MultiVector homogeneous_magnitude_multi_vector_transformation(HomogeneousMagnitude self, MultiVector other) {
    return multi_vector_homogeneous_magnitude_geometric_product(homogeneous_magnitude_multi_vector_geometric_product(self, other), homogeneous_magnitude_reversal(self));
}

Plane homogeneous_magnitude_plane_transformation(HomogeneousMagnitude self, Plane other) {
    return plane_homogeneous_magnitude_geometric_product(homogeneous_magnitude_plane_geometric_product(self, other), homogeneous_magnitude_reversal(self));
}

Rotor homogeneous_magnitude_rotor_transformation(HomogeneousMagnitude self, Rotor other) {
    return rotor_homogeneous_magnitude_geometric_product(homogeneous_magnitude_rotor_geometric_product(self, other), homogeneous_magnitude_reversal(self));
}

HomogeneousMagnitude homogeneous_magnitude_scalar_geometric_quotient(HomogeneousMagnitude self, Scalar other) {
    return homogeneous_magnitude_scalar_geometric_product(self, scalar_inverse(other));
}

Scalar homogeneous_magnitude_scalar_transformation(HomogeneousMagnitude self, Scalar other) {
    return homogeneous_magnitude_scalar_into(homogeneous_magnitude_homogeneous_magnitude_geometric_product(homogeneous_magnitude_scalar_geometric_product(self, other), homogeneous_magnitude_reversal(self)));
}

Sphere homogeneous_magnitude_sphere_transformation(HomogeneousMagnitude self, Sphere other) {
    return sphere_homogeneous_magnitude_geometric_product(homogeneous_magnitude_sphere_geometric_product(self, other), homogeneous_magnitude_reversal(self));
}

Translator homogeneous_magnitude_translator_transformation(HomogeneousMagnitude self, Translator other) {
    return translator_homogeneous_magnitude_geometric_product(homogeneous_magnitude_translator_geometric_product(self, other), homogeneous_magnitude_reversal(self));
}

Motor line_dipole_geometric_quotient(Line self, Dipole other) {
    return line_dipole_geometric_product(self, dipole_inverse(other));
}

Line line_homogeneous_magnitude_geometric_quotient(Line self, HomogeneousMagnitude other) {
    return line_homogeneous_magnitude_geometric_product(self, homogeneous_magnitude_inverse(other));
}

Flector line_radial_point_geometric_quotient(Line self, RadialPoint other) {
    return line_radial_point_geometric_product(self, radial_point_inverse(other));
}

Line line_scalar_geometric_quotient(Line self, Scalar other) {
    return line_scalar_geometric_product(self, scalar_inverse(other));
}

Flector motor_circle_geometric_quotient(Motor self, Circle other) {
    return motor_circle_geometric_product(self, circle_inverse(other));
}

Flector motor_dilation_geometric_quotient(Motor self, Dilation other) {
    return motor_dilation_geometric_product(self, dilation_inverse(other));
}

Motor motor_dipole_geometric_quotient(Motor self, Dipole other) {
    return motor_dipole_geometric_product(self, dipole_inverse(other));
}

Motor motor_homogeneous_magnitude_geometric_quotient(Motor self, HomogeneousMagnitude other) {
    return motor_homogeneous_magnitude_geometric_product(self, homogeneous_magnitude_inverse(other));
}

Flector motor_radial_point_geometric_quotient(Motor self, RadialPoint other) {
    return motor_radial_point_geometric_product(self, radial_point_inverse(other));
}

Motor motor_scalar_geometric_quotient(Motor self, Scalar other) {
    return motor_scalar_geometric_product(self, scalar_inverse(other));
}

MultiVector multi_vector_circle_geometric_quotient(MultiVector self, Circle other) {
    return multi_vector_circle_geometric_product(self, circle_inverse(other));
}

Circle multi_vector_circle_transformation(MultiVector self, Circle other) {
    return multi_vector_circle_into(multi_vector_multi_vector_geometric_product(multi_vector_circle_geometric_product(self, other), multi_vector_reversal(self)));
}

MultiVector multi_vector_dilation_geometric_quotient(MultiVector self, Dilation other) {
    return multi_vector_dilation_geometric_product(self, dilation_inverse(other));
}

Dilation multi_vector_dilation_transformation(MultiVector self, Dilation other) {
    return multi_vector_dilation_into(multi_vector_multi_vector_geometric_product(multi_vector_dilation_geometric_product(self, other), multi_vector_reversal(self)));
}

MultiVector multi_vector_dipole_geometric_quotient(MultiVector self, Dipole other) {
    return multi_vector_dipole_geometric_product(self, dipole_inverse(other));
}

Dipole multi_vector_dipole_transformation(MultiVector self, Dipole other) {
    return multi_vector_dipole_into(multi_vector_multi_vector_geometric_product(multi_vector_dipole_geometric_product(self, other), multi_vector_reversal(self)));
}

MultiVector multi_vector_homogeneous_magnitude_geometric_quotient(MultiVector self, HomogeneousMagnitude other) {
    return multi_vector_homogeneous_magnitude_geometric_product(self, homogeneous_magnitude_inverse(other));
}

HomogeneousMagnitude multi_vector_homogeneous_magnitude_transformation(MultiVector self, HomogeneousMagnitude other) {
    return multi_vector_homogeneous_magnitude_into(multi_vector_multi_vector_geometric_product(multi_vector_homogeneous_magnitude_geometric_product(self, other), multi_vector_reversal(self)));
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

MultiVector multi_vector_multi_vector_geometric_quotient(MultiVector self, MultiVector other) {
    return multi_vector_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

MultiVector multi_vector_multi_vector_transformation(MultiVector self, MultiVector other) {
    return multi_vector_multi_vector_geometric_product(multi_vector_multi_vector_geometric_product(self, other), multi_vector_reversal(self));
}

MultiVector multi_vector_radial_point_geometric_quotient(MultiVector self, RadialPoint other) {
    return multi_vector_radial_point_geometric_product(self, radial_point_inverse(other));
}

RadialPoint multi_vector_radial_point_transformation(MultiVector self, RadialPoint other) {
    return multi_vector_radial_point_into(multi_vector_multi_vector_geometric_product(multi_vector_radial_point_geometric_product(self, other), multi_vector_reversal(self)));
}

MultiVector multi_vector_scalar_geometric_quotient(MultiVector self, Scalar other) {
    return multi_vector_scalar_geometric_product(self, scalar_inverse(other));
}

Scalar multi_vector_scalar_transformation(MultiVector self, Scalar other) {
    return multi_vector_scalar_into(multi_vector_multi_vector_geometric_product(multi_vector_scalar_geometric_product(self, other), multi_vector_reversal(self)));
}

Plane plane_homogeneous_magnitude_geometric_quotient(Plane self, HomogeneousMagnitude other) {
    return plane_homogeneous_magnitude_geometric_product(self, homogeneous_magnitude_inverse(other));
}

Plane plane_scalar_geometric_quotient(Plane self, Scalar other) {
    return plane_scalar_geometric_product(self, scalar_inverse(other));
}

Flector radial_point_flector_transformation(RadialPoint self, Flector other) {
    return motor_radial_point_geometric_product(radial_point_flector_geometric_product(self, other), radial_point_reversal(self));
}

Line radial_point_line_transformation(RadialPoint self, Line other) {
    return motor_line_into(flector_radial_point_geometric_product(radial_point_line_geometric_product(self, other), radial_point_reversal(self)));
}

Motor radial_point_motor_transformation(RadialPoint self, Motor other) {
    return flector_radial_point_geometric_product(radial_point_motor_geometric_product(self, other), radial_point_reversal(self));
}

MultiVector radial_point_multi_vector_geometric_quotient(RadialPoint self, MultiVector other) {
    return radial_point_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

MultiVector radial_point_multi_vector_transformation(RadialPoint self, MultiVector other) {
    return multi_vector_radial_point_geometric_product(radial_point_multi_vector_geometric_product(self, other), radial_point_reversal(self));
}

RadialPoint radial_point_scalar_geometric_quotient(RadialPoint self, Scalar other) {
    return radial_point_scalar_geometric_product(self, scalar_inverse(other));
}

Rotor rotor_dipole_geometric_quotient(Rotor self, Dipole other) {
    return rotor_dipole_geometric_product(self, dipole_inverse(other));
}

Rotor rotor_homogeneous_magnitude_geometric_quotient(Rotor self, HomogeneousMagnitude other) {
    return rotor_homogeneous_magnitude_geometric_product(self, homogeneous_magnitude_inverse(other));
}

Rotor rotor_scalar_geometric_quotient(Rotor self, Scalar other) {
    return rotor_scalar_geometric_product(self, scalar_inverse(other));
}

AntiScalar scalar_anti_scalar_transformation(Scalar self, AntiScalar other) {
    return anti_scalar_scalar_geometric_product(scalar_anti_scalar_geometric_product(self, other), scalar_reversal(self));
}

Circle scalar_circle_geometric_quotient(Scalar self, Circle other) {
    return scalar_circle_geometric_product(self, circle_inverse(other));
}

Circle scalar_circle_transformation(Scalar self, Circle other) {
    return circle_scalar_geometric_product(scalar_circle_geometric_product(self, other), scalar_reversal(self));
}

Dilation scalar_dilation_geometric_quotient(Scalar self, Dilation other) {
    return scalar_dilation_geometric_product(self, dilation_inverse(other));
}

Dilation scalar_dilation_transformation(Scalar self, Dilation other) {
    return dilation_scalar_geometric_product(scalar_dilation_geometric_product(self, other), scalar_reversal(self));
}

Dipole scalar_dipole_geometric_quotient(Scalar self, Dipole other) {
    return scalar_dipole_geometric_product(self, dipole_inverse(other));
}

Dipole scalar_dipole_transformation(Scalar self, Dipole other) {
    return dipole_scalar_geometric_product(scalar_dipole_geometric_product(self, other), scalar_reversal(self));
}

FlatPoint scalar_flat_point_transformation(Scalar self, FlatPoint other) {
    return flat_point_scalar_geometric_product(scalar_flat_point_geometric_product(self, other), scalar_reversal(self));
}

Flector scalar_flector_transformation(Scalar self, Flector other) {
    return flector_scalar_geometric_product(scalar_flector_geometric_product(self, other), scalar_reversal(self));
}

HomogeneousMagnitude scalar_homogeneous_magnitude_geometric_quotient(Scalar self, HomogeneousMagnitude other) {
    return scalar_homogeneous_magnitude_geometric_product(self, homogeneous_magnitude_inverse(other));
}

HomogeneousMagnitude scalar_homogeneous_magnitude_transformation(Scalar self, HomogeneousMagnitude other) {
    return homogeneous_magnitude_scalar_geometric_product(scalar_homogeneous_magnitude_geometric_product(self, other), scalar_reversal(self));
}

Line scalar_line_transformation(Scalar self, Line other) {
    return line_scalar_geometric_product(scalar_line_geometric_product(self, other), scalar_reversal(self));
}

Motor scalar_motor_transformation(Scalar self, Motor other) {
    return motor_scalar_geometric_product(scalar_motor_geometric_product(self, other), scalar_reversal(self));
}

MultiVector scalar_multi_vector_geometric_quotient(Scalar self, MultiVector other) {
    return scalar_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

MultiVector scalar_multi_vector_transformation(Scalar self, MultiVector other) {
    return multi_vector_scalar_geometric_product(scalar_multi_vector_geometric_product(self, other), scalar_reversal(self));
}

Plane scalar_plane_transformation(Scalar self, Plane other) {
    return plane_scalar_geometric_product(scalar_plane_geometric_product(self, other), scalar_reversal(self));
}

RadialPoint scalar_radial_point_geometric_quotient(Scalar self, RadialPoint other) {
    return scalar_radial_point_geometric_product(self, radial_point_inverse(other));
}

RadialPoint scalar_radial_point_transformation(Scalar self, RadialPoint other) {
    return radial_point_scalar_geometric_product(scalar_radial_point_geometric_product(self, other), scalar_reversal(self));
}

Rotor scalar_rotor_transformation(Scalar self, Rotor other) {
    return rotor_scalar_geometric_product(scalar_rotor_geometric_product(self, other), scalar_reversal(self));
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

Scalar scalar_scalar_geometric_quotient(Scalar self, Scalar other) {
    return scalar_scalar_geometric_product(self, scalar_inverse(other));
}

Scalar scalar_scalar_transformation(Scalar self, Scalar other) {
    return scalar_scalar_geometric_product(scalar_scalar_geometric_product(self, other), scalar_reversal(self));
}

Sphere scalar_sphere_transformation(Scalar self, Sphere other) {
    return sphere_scalar_geometric_product(scalar_sphere_geometric_product(self, other), scalar_reversal(self));
}

Translator scalar_translator_transformation(Scalar self, Translator other) {
    return translator_scalar_geometric_product(scalar_translator_geometric_product(self, other), scalar_reversal(self));
}

Sphere sphere_homogeneous_magnitude_geometric_quotient(Sphere self, HomogeneousMagnitude other) {
    return sphere_homogeneous_magnitude_geometric_product(self, homogeneous_magnitude_inverse(other));
}

Sphere sphere_scalar_geometric_quotient(Sphere self, Scalar other) {
    return sphere_scalar_geometric_product(self, scalar_inverse(other));
}

Motor translator_dipole_geometric_quotient(Translator self, Dipole other) {
    return translator_dipole_geometric_product(self, dipole_inverse(other));
}

Translator translator_homogeneous_magnitude_geometric_quotient(Translator self, HomogeneousMagnitude other) {
    return translator_homogeneous_magnitude_geometric_product(self, homogeneous_magnitude_inverse(other));
}

Translator translator_scalar_geometric_quotient(Translator self, Scalar other) {
    return translator_scalar_geometric_product(self, scalar_inverse(other));
}

HomogeneousMagnitude circle_dipole_euclidean_distance(Circle self, Dipole other) {
    return scalar_anti_scalar_add(circle_bulk_norm(anti_scalar_attitude(circle_dipole_outer_product(self, other))), circle_weight_norm(circle_scalar_outer_product(self, dipole_attitude(other))));
}

HomogeneousMagnitude circle_flat_point_euclidean_distance(Circle self, FlatPoint other) {
    return scalar_anti_scalar_add(circle_bulk_norm(anti_scalar_attitude(circle_flat_point_outer_product(self, other))), circle_weight_norm(circle_scalar_outer_product(self, flat_point_attitude(other))));
}

HomogeneousMagnitude dilation_dipole_euclidean_distance(Dilation self, Dipole other) {
    return scalar_anti_scalar_add(circle_bulk_norm(anti_scalar_attitude(dilation_dipole_outer_product(self, other))), dilation_weight_norm(dilation_scalar_outer_product(self, dipole_attitude(other))));
}

HomogeneousMagnitude dilation_flat_point_euclidean_distance(Dilation self, FlatPoint other) {
    return scalar_anti_scalar_add(circle_bulk_norm(anti_scalar_attitude(dilation_flat_point_outer_product(self, other))), dilation_weight_norm(dilation_scalar_outer_product(self, flat_point_attitude(other))));
}

HomogeneousMagnitude dipole_circle_euclidean_distance(Dipole self, Circle other) {
    return scalar_anti_scalar_add(circle_bulk_norm(anti_scalar_attitude(dipole_circle_outer_product(self, other))), circle_weight_norm(dipole_radial_point_outer_product(self, circle_attitude(other))));
}

HomogeneousMagnitude dipole_dipole_euclidean_distance(Dipole self, Dipole other) {
    return scalar_anti_scalar_add(dipole_bulk_norm(sphere_attitude(dipole_dipole_outer_product(self, other))), dipole_weight_norm(dipole_scalar_outer_product(self, dipole_attitude(other))));
}

HomogeneousMagnitude dipole_flat_point_euclidean_distance(Dipole self, FlatPoint other) {
    return scalar_anti_scalar_add(dipole_bulk_norm(plane_attitude(dipole_flat_point_outer_product(self, other))), dipole_weight_norm(dipole_scalar_outer_product(self, flat_point_attitude(other))));
}

HomogeneousMagnitude dipole_line_euclidean_distance(Dipole self, Line other) {
    return scalar_anti_scalar_add(circle_bulk_norm(anti_scalar_attitude(dipole_line_outer_product(self, other))), circle_weight_norm(dipole_radial_point_outer_product(self, line_attitude(other))));
}

HomogeneousMagnitude homogeneous_magnitude_anti_scalar_euclidean_distance(HomogeneousMagnitude self, AntiScalar other) {
    return scalar_anti_scalar_add(circle_bulk_norm(anti_scalar_attitude(homogeneous_magnitude_anti_scalar_outer_product(self, other))), circle_weight_norm(homogeneous_magnitude_circle_outer_product(self, anti_scalar_attitude(other))));
}

HomogeneousMagnitude homogeneous_magnitude_homogeneous_magnitude_euclidean_distance(HomogeneousMagnitude self, HomogeneousMagnitude other) {
    return scalar_anti_scalar_add(circle_bulk_norm(homogeneous_magnitude_attitude(homogeneous_magnitude_homogeneous_magnitude_outer_product(self, other))), circle_weight_norm(homogeneous_magnitude_circle_outer_product(self, homogeneous_magnitude_attitude(other))));
}

HomogeneousMagnitude homogeneous_magnitude_plane_euclidean_distance(HomogeneousMagnitude self, Plane other) {
    return scalar_anti_scalar_add(dipole_bulk_norm(plane_attitude(homogeneous_magnitude_plane_outer_product(self, other))), dipole_weight_norm(homogeneous_magnitude_dipole_outer_product(self, plane_attitude(other))));
}

HomogeneousMagnitude homogeneous_magnitude_sphere_euclidean_distance(HomogeneousMagnitude self, Sphere other) {
    return scalar_anti_scalar_add(dipole_bulk_norm(sphere_attitude(homogeneous_magnitude_sphere_outer_product(self, other))), dipole_weight_norm(homogeneous_magnitude_dipole_outer_product(self, sphere_attitude(other))));
}

HomogeneousMagnitude radial_point_circle_euclidean_distance(RadialPoint self, Circle other) {
    return scalar_anti_scalar_add(dipole_bulk_norm(sphere_attitude(radial_point_circle_outer_product(self, other))), dipole_weight_norm(radial_point_radial_point_outer_product(self, circle_attitude(other))));
}

HomogeneousMagnitude radial_point_line_euclidean_distance(RadialPoint self, Line other) {
    return scalar_anti_scalar_add(dipole_bulk_norm(plane_attitude(radial_point_line_outer_product(self, other))), dipole_weight_norm(radial_point_radial_point_outer_product(self, line_attitude(other))));
}

HomogeneousMagnitude radial_point_plane_euclidean_distance(RadialPoint self, Plane other) {
    return scalar_anti_scalar_add(circle_bulk_norm(anti_scalar_attitude(radial_point_plane_outer_product(self, other))), circle_weight_norm(radial_point_dipole_outer_product(self, plane_attitude(other))));
}

HomogeneousMagnitude radial_point_sphere_euclidean_distance(RadialPoint self, Sphere other) {
    return scalar_anti_scalar_add(circle_bulk_norm(anti_scalar_attitude(radial_point_sphere_outer_product(self, other))), circle_weight_norm(radial_point_dipole_outer_product(self, sphere_attitude(other))));
}

HomogeneousMagnitude scalar_anti_scalar_euclidean_distance(Scalar self, AntiScalar other) {
    return scalar_anti_scalar_add(circle_bulk_norm(anti_scalar_attitude(scalar_anti_scalar_outer_product(self, other))), circle_weight_norm(scalar_circle_outer_product(self, anti_scalar_attitude(other))));
}

HomogeneousMagnitude scalar_homogeneous_magnitude_euclidean_distance(Scalar self, HomogeneousMagnitude other) {
    return scalar_anti_scalar_add(circle_bulk_norm(homogeneous_magnitude_attitude(scalar_homogeneous_magnitude_outer_product(self, other))), circle_weight_norm(scalar_circle_outer_product(self, homogeneous_magnitude_attitude(other))));
}

HomogeneousMagnitude scalar_plane_euclidean_distance(Scalar self, Plane other) {
    return scalar_anti_scalar_add(dipole_bulk_norm(plane_attitude(scalar_plane_outer_product(self, other))), dipole_weight_norm(scalar_dipole_outer_product(self, plane_attitude(other))));
}

HomogeneousMagnitude scalar_sphere_euclidean_distance(Scalar self, Sphere other) {
    return scalar_anti_scalar_add(dipole_bulk_norm(sphere_attitude(scalar_sphere_outer_product(self, other))), dipole_weight_norm(scalar_dipole_outer_product(self, sphere_attitude(other))));
}


struct Scalar {
    // 1
    float g0;
};

struct AntiScalar {
    // e01234
    float g0;
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

AntiScalar scalar_anti_scalar_left_contraction(Scalar self, AntiScalar other) {
    return AntiScalar(self.g0 * other.g0);
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

Translator scalar_translator_left_contraction(Scalar self, Translator other) {
    return Translator(vec4(self.g0) * other.g0);
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

AntiScalar anti_scalar_scalar_right_contraction(AntiScalar self, Scalar other) {
    return AntiScalar(self.g0 * other.g0);
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

RadialPoint anti_scalar_radial_point_regressive_product(AntiScalar self, RadialPoint other) {
    return RadialPoint(vec3(self.g0) * other.g0, vec2(self.g0) * other.g1);
}

FlatPoint anti_scalar_flat_point_regressive_product(AntiScalar self, FlatPoint other) {
    return FlatPoint(vec4(self.g0) * other.g0);
}

Dipole anti_scalar_dipole_regressive_product(AntiScalar self, Dipole other) {
    return Dipole(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1, vec4(self.g0) * other.g2);
}

Line anti_scalar_line_regressive_product(AntiScalar self, Line other) {
    return Line(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Circle anti_scalar_circle_regressive_product(AntiScalar self, Circle other) {
    return Circle(vec4(self.g0) * other.g0, vec3(self.g0) * other.g1, vec3(self.g0) * other.g2);
}

Plane anti_scalar_plane_regressive_product(AntiScalar self, Plane other) {
    return Plane(vec4(self.g0) * other.g0);
}

Sphere anti_scalar_sphere_regressive_product(AntiScalar self, Sphere other) {
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

Rotor anti_scalar_rotor_add(AntiScalar self, Rotor other) {
    return Rotor(vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) + other.g0);
}

Rotor anti_scalar_rotor_sub(AntiScalar self, Rotor other) {
    return Rotor(vec4(self.g0) * vec4(0.0, 0.0, 0.0, 1.0) - other.g0);
}

Rotor anti_scalar_rotor_regressive_product(AntiScalar self, Rotor other) {
    return Rotor(vec4(self.g0) * other.g0);
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

Flector anti_scalar_flector_regressive_product(AntiScalar self, Flector other) {
    return Flector(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
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

AntiScalar anti_scalar_scale(AntiScalar self, float other) {
    return anti_scalar_scalar_geometric_product(self, Scalar(other));
}

RadialPoint radial_point_zero() {
    return RadialPoint(vec3(0.0), vec2(0.0));
}

RadialPoint radial_point_one() {
    return RadialPoint(vec3(0.0), vec2(0.0));
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

Motor radial_point_flector_geometric_product(RadialPoint self, Flector other) {
    return Motor(vec4(self.g0.x) * vec4(other.g0.w, other.g1.z, other.g1.y, other.g1.x) * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g0.y) * vec4(other.g1.z, other.g0.w, other.g1.x, other.g1.y) * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g0.w, other.g1.z) * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g1.x) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.w), vec4(self.g0.x) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g0.x) * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g0.y) * vec4(other.g0.z, other.g1.w, other.g0.x, other.g0.y) * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g1.w, other.g0.z) * vec4(-1.0, 1.0, -1.0, 1.0));
}

Scalar radial_point_flector_regressive_product(RadialPoint self, Flector other) {
    return Scalar(self.g0.x * other.g1.x + self.g0.y * other.g1.y + self.g0.z * other.g1.z + self.g1.x * other.g1.w);
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

Scalar flat_point_circle_regressive_product(FlatPoint self, Circle other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

AntiScalar flat_point_circle_outer_product(FlatPoint self, Circle other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

Flector flat_point_plane_add(FlatPoint self, Plane other) {
    return Flector(self.g0, other.g0);
}

Flector flat_point_plane_sub(FlatPoint self, Plane other) {
    return Flector(self.g0, vec4(0.0) - other.g0);
}

RadialPoint flat_point_sphere_regressive_product(FlatPoint self, Sphere other) {
    return RadialPoint(vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.x) * vec3(-1.0), vec2(self.g0.y) * vec2(other.g1.y) * vec2(0.0, 1.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(0.0, 1.0) + vec2(self.g0.w) * other.g0 * vec2(-1.0, 1.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(0.0, 1.0));
}

FlatPoint flat_point_motor_regressive_product(FlatPoint self, Motor other) {
    return FlatPoint(self.g0 * vec4(other.g0.w));
}

FlatPoint flat_point_rotor_regressive_product(FlatPoint self, Rotor other) {
    return FlatPoint(self.g0 * vec4(other.g0.w));
}

FlatPoint flat_point_translator_regressive_product(FlatPoint self, Translator other) {
    return FlatPoint(self.g0 * vec4(other.g0.w));
}

Flector flat_point_flector_add(FlatPoint self, Flector other) {
    return Flector(self.g0 + other.g0, other.g1);
}

Flector flat_point_flector_sub(FlatPoint self, Flector other) {
    return Flector(self.g0 - other.g0, vec4(0.0) - other.g1);
}

MultiVector flat_point_multi_vector_add(FlatPoint self, MultiVector other) {
    return MultiVector(other.g0, other.g1, other.g2, self.g0 + other.g3, other.g4, other.g5, other.g6, other.g7, other.g8, other.g9);
}

MultiVector flat_point_multi_vector_sub(FlatPoint self, MultiVector other) {
    return MultiVector(vec3(0.0) - other.g0, vec3(0.0) - other.g1, vec2(0.0) - other.g2, self.g0 - other.g3, vec3(0.0) - other.g4, vec3(0.0) - other.g5, vec3(0.0) - other.g6, vec3(0.0) - other.g7, vec4(0.0) - other.g8, vec4(0.0) - other.g9);
}

FlatPoint flat_point_scale(FlatPoint self, float other) {
    return flat_point_scalar_geometric_product(self, Scalar(other));
}

Dipole dipole_zero() {
    return Dipole(vec3(0.0), vec3(0.0), vec4(0.0));
}

Dipole dipole_one() {
    return Dipole(vec3(0.0), vec3(0.0), vec4(0.0));
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

Scalar dipole_dipole_left_contraction(Dipole self, Dipole other) {
    return Scalar(0.0 - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z);
}

Scalar dipole_dipole_right_contraction(Dipole self, Dipole other) {
    return Scalar(0.0 - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z);
}

Scalar dipole_dipole_scalar_product(Dipole self, Dipole other) {
    return Scalar(0.0 - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z);
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

Scalar dipole_circle_regressive_product(Dipole self, Circle other) {
    return Scalar(0.0 - self.g0.x * other.g2.x - self.g0.y * other.g2.y - self.g0.z * other.g2.z - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z - self.g2.x * other.g0.x - self.g2.y * other.g0.y - self.g2.z * other.g0.z - self.g2.w * other.g0.w);
}

AntiScalar dipole_circle_outer_product(Dipole self, Circle other) {
    return AntiScalar(0.0 - self.g0.x * other.g2.x - self.g0.y * other.g2.y - self.g0.z * other.g2.z - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z - self.g2.x * other.g0.x - self.g2.y * other.g0.y - self.g2.z * other.g0.z - self.g2.w * other.g0.w);
}

RadialPoint dipole_circle_inner_product(Dipole self, Circle other) {
    return RadialPoint(self.g1 * vec3(other.g0.w), vec2(0.0) - vec2(self.g1.x) * vec2(other.g0.x, other.g2.x) - vec2(self.g1.y) * vec2(other.g0.y, other.g2.y) - vec2(self.g1.z) * vec2(other.g0.z, other.g2.z));
}

RadialPoint dipole_circle_left_contraction(Dipole self, Circle other) {
    return RadialPoint(self.g1 * vec3(other.g0.w), vec2(0.0) - vec2(self.g1.x) * vec2(other.g0.x, other.g2.x) - vec2(self.g1.y) * vec2(other.g0.y, other.g2.y) - vec2(self.g1.z) * vec2(other.g0.z, other.g2.z));
}

RadialPoint dipole_plane_regressive_product(Dipole self, Plane other) {
    return RadialPoint(vec3(self.g1.x) * vec3(other.g0.z, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + self.g0 * vec3(other.g0.w), vec2(self.g0.y) * vec2(other.g0.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g0.z) * vec2(-1.0, 0.0) + vec2(self.g2.x) * vec2(other.g0.x) * vec2(0.0, 1.0) + vec2(self.g2.y) * vec2(other.g0.y) * vec2(0.0, 1.0) + vec2(self.g2.z) * vec2(other.g0.z) * vec2(0.0, 1.0) + vec2(self.g2.w) * vec2(other.g0.w) * vec2(0.0, 1.0) + vec2(self.g0.x) * vec2(other.g0.x) * vec2(-1.0, 0.0));
}

FlatPoint dipole_plane_inner_product(Dipole self, Plane other) {
    return FlatPoint(vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

FlatPoint dipole_plane_left_contraction(Dipole self, Plane other) {
    return FlatPoint(vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
}

RadialPoint dipole_sphere_regressive_product(Dipole self, Sphere other) {
    return RadialPoint(vec3(self.g1.x) * other.g1.zzy * vec3(0.0, -1.0, 1.0) + vec3(self.g1.y) * other.g1.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * other.g1.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g2.x) * vec3(other.g0.x) * vec3(-1.0, 0.0, 0.0) + vec3(self.g2.y) * vec3(other.g0.x) * vec3(0.0, -1.0, 0.0) + vec3(self.g2.z) * vec3(other.g0.x) * vec3(0.0, 0.0, -1.0) + self.g0 * vec3(other.g0.y), vec2(self.g0.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g2.x) * vec2(other.g1.x) * vec2(0.0, 1.0) + vec2(self.g2.y) * vec2(other.g1.y) * vec2(0.0, 1.0) + vec2(self.g2.z) * vec2(other.g1.z) * vec2(0.0, 1.0) + vec2(self.g2.w) * other.g0 * vec2(-1.0, 1.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(-1.0, 0.0));
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

MultiVector dipole_multi_vector_add(Dipole self, MultiVector other) {
    return MultiVector(other.g0, other.g1, other.g2, self.g2 + other.g3, self.g0 + other.g4, self.g1 + other.g5, other.g6, other.g7, other.g8, other.g9);
}

MultiVector dipole_multi_vector_sub(Dipole self, MultiVector other) {
    return MultiVector(vec3(0.0) - other.g0, vec3(0.0) - other.g1, vec2(0.0) - other.g2, self.g2 - other.g3, self.g0 - other.g4, self.g1 - other.g5, vec3(0.0) - other.g6, vec3(0.0) - other.g7, vec4(0.0) - other.g8, vec4(0.0) - other.g9);
}

MultiVector dipole_multi_vector_geometric_product(Dipole self, MultiVector other) {
    return MultiVector(vec3(self.g0.y) * vec3(other.g5.y, other.g5.y, other.g7.y) * vec3(0.0, -1.0, -1.0) + vec3(self.g0.z) * vec3(other.g5.z, other.g5.z, other.g7.z) * vec3(0.0, -1.0, -1.0) - vec3(self.g1.x) * vec3(other.g5.x, other.g4.x, other.g6.x) - vec3(self.g1.y) * vec3(other.g5.y, other.g4.y, other.g6.y) - vec3(self.g1.z) * vec3(other.g5.z, other.g4.z, other.g6.z) + vec3(self.g2.x) * vec3(other.g8.x) * vec3(0.0, 0.0, -1.0) + vec3(self.g2.y) * vec3(other.g8.y) * vec3(0.0, 0.0, -1.0) + vec3(self.g2.z) * vec3(other.g8.z) * vec3(0.0, 0.0, -1.0) + vec3(self.g2.w) * vec3(other.g8.w) * vec3(0.0, 0.0, -1.0) + vec3(self.g0.x) * vec3(other.g5.x, other.g5.x, other.g7.x) * vec3(0.0, -1.0, -1.0), vec3(self.g1.x) * vec3(other.g8.w, other.g1.z, other.g1.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.y) * vec3(other.g1.z, other.g8.w, other.g1.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g1.z) * vec3(other.g1.y, other.g1.x, other.g8.w) * vec3(1.0, -1.0, 1.0), vec2(self.g0.y) * vec2(other.g1.y) * vec2(1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(1.0, 0.0) - vec2(self.g1.x) * vec2(other.g8.x, other.g7.x) - vec2(self.g1.y) * vec2(other.g8.y, other.g7.y) - vec2(self.g1.z) * vec2(other.g8.z, other.g7.z) + vec2(self.g2.x) * vec2(other.g1.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(1.0, 0.0), vec4(self.g0.y) * vec4(other.g3.y) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g3.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g9.w, other.g3.z, other.g3.y, other.g9.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g3.z, other.g9.w, other.g3.x, other.g9.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g3.y, other.g3.x, other.g9.w, other.g9.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g2.x) * vec4(other.g0.x, other.g5.z, other.g5.y, other.g4.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g2.y) * vec4(other.g5.z, other.g0.x, other.g5.x, other.g4.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g2.z) * vec4(other.g5.y, other.g5.x, other.g0.x, other.g4.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g2.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g3.x) * vec4(0.0, 0.0, 0.0, 1.0), vec3(self.g0.x) * vec3(other.g0.x, other.g5.z, other.g5.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.y) * vec3(other.g5.z, other.g0.x, other.g5.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g0.z) * vec3(other.g5.y, other.g5.x, other.g0.x) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.x) * vec3(other.g0.y, other.g4.z, other.g4.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.y) * vec3(other.g4.z, other.g0.y, other.g4.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g1.z) * vec3(other.g4.y, other.g4.x, other.g0.y) * vec3(1.0, -1.0, 1.0), vec3(self.g1.x) * vec3(other.g0.x, other.g5.z, other.g5.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.y) * vec3(other.g5.z, other.g0.x, other.g5.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g1.z) * vec3(other.g5.y, other.g5.x, other.g0.x) * vec3(1.0, -1.0, 1.0), vec3(self.g0.x) * vec3(other.g2.y, other.g7.z, other.g7.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g0.y) * vec3(other.g7.z, other.g2.y, other.g7.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g0.z) * vec3(other.g7.y, other.g7.x, other.g2.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g1.x) * vec3(other.g0.z, other.g6.z, other.g6.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.y) * vec3(other.g6.z, other.g0.z, other.g6.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g1.z) * vec3(other.g6.y, other.g6.x, other.g0.z) * vec3(1.0, -1.0, 1.0) + vec3(self.g2.x) * vec3(other.g2.x, other.g8.z, other.g8.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g2.y) * vec3(other.g8.z, other.g2.x, other.g8.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g2.z) * vec3(other.g8.y, other.g8.x, other.g2.x) * vec3(1.0, -1.0, 1.0) - vec3(self.g2.w) * other.g1, vec3(self.g1.x) * vec3(other.g2.y, other.g7.z, other.g7.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g1.y) * vec3(other.g7.z, other.g2.y, other.g7.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g1.z) * vec3(other.g7.y, other.g7.x, other.g2.y) * vec3(1.0, -1.0, 1.0) + vec3(self.g2.x) * vec3(other.g8.w, other.g1.z, other.g1.y) * vec3(1.0, 1.0, -1.0) + vec3(self.g2.y) * vec3(other.g1.z, other.g8.w, other.g1.x) * vec3(-1.0, 1.0, 1.0) + vec3(self.g2.z) * vec3(other.g1.y, other.g1.x, other.g8.w) * vec3(1.0, -1.0, 1.0), vec4(self.g0.y) * vec4(other.g1.z, other.g8.w, other.g1.x, other.g1.z) * vec4(1.0, -1.0, -1.0, 0.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g8.w, other.g1.y) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g1.x) * vec4(other.g2.x, other.g8.z, other.g8.y, other.g1.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g8.z, other.g2.x, other.g8.x, other.g1.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g8.y, other.g8.x, other.g2.x, other.g1.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.x) * vec4(other.g8.w, other.g1.z, other.g1.y, other.g8.x) * vec4(-1.0, -1.0, 1.0, 0.0), vec4(self.g0.y) * vec4(other.g3.z, other.g9.w, other.g3.x, other.g3.z) * vec4(1.0, -1.0, -1.0, 0.0) + vec4(self.g0.z) * vec4(other.g3.y, other.g3.x, other.g9.w, other.g3.y) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g1.x) * vec4(other.g3.w, other.g9.z, other.g9.y, other.g3.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g9.z, other.g3.w, other.g9.x, other.g3.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g9.y, other.g9.x, other.g3.w, other.g3.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g2.x) * vec4(other.g0.y, other.g4.z, other.g4.y, other.g5.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g2.y) * vec4(other.g4.z, other.g0.y, other.g4.x, other.g5.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g2.z) * vec4(other.g4.y, other.g4.x, other.g0.y, other.g5.z) * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g2.w) * vec4(other.g5.x, other.g5.y, other.g5.z, other.g5.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g9.w, other.g3.z, other.g3.y, other.g9.x) * vec4(-1.0, -1.0, 1.0, 0.0));
}

Scalar dipole_multi_vector_scalar_product(Dipole self, MultiVector other) {
    return Scalar(0.0 - self.g1.x * other.g5.x - self.g1.y * other.g5.y - self.g1.z * other.g5.z);
}

Scalar dipole_squared_magnitude(Dipole self) {
    return dipole_dipole_scalar_product(self, dipole_reversal(self));
}

Scalar dipole_magnitude(Dipole self) {
    return Scalar(sqrt(dipole_squared_magnitude(self).g0));
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

Line line_zero() {
    return Line(vec3(0.0), vec3(0.0));
}

Line line_one() {
    return Line(vec3(0.0), vec3(0.0));
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

Motor line_dipole_geometric_product(Line self, Dipole other) {
    return Motor(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g0.z, other.g0.z, other.g0.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0), vec4(self.g1.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

Scalar line_dipole_regressive_product(Line self, Dipole other) {
    return Scalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
}

AntiScalar line_dipole_outer_product(Line self, Dipole other) {
    return AntiScalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z - self.g1.x * other.g0.x - self.g1.y * other.g0.y - self.g1.z * other.g0.z);
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

Circle line_circle_add(Line self, Circle other) {
    return Circle(other.g0, self.g0 + other.g1, self.g1 + other.g2);
}

Circle line_circle_sub(Line self, Circle other) {
    return Circle(vec4(0.0) - other.g0, self.g0 - other.g1, self.g1 - other.g2);
}

RadialPoint line_circle_regressive_product(Line self, Circle other) {
    return RadialPoint(vec3(self.g1.x) * vec3(other.g0.z, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0) + vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + self.g0 * vec3(other.g0.w), vec2(0.0) - vec2(self.g0.x) * vec2(other.g0.x, other.g2.x) - vec2(self.g0.y) * vec2(other.g0.y, other.g2.y) - vec2(self.g0.z) * vec2(other.g0.z, other.g2.z) + vec2(self.g1.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g1.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g1.x) * vec2(other.g1.x) * vec2(0.0, -1.0));
}

FlatPoint line_plane_regressive_product(Line self, Plane other) {
    return FlatPoint(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x, self.g1.x, self.g1.x, self.g0.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0));
}

Dipole line_sphere_regressive_product(Line self, Sphere other) {
    return Dipole(self.g0 * vec3(other.g0.x), self.g1 * vec3(other.g0.x), vec4(self.g0.y) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g1.y) * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g1.z) * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.x) * vec4(other.g1.z, other.g1.z, other.g1.y, other.g1.z) * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g1.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.z) * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.y) * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.y, other.g0.x, other.g0.x, other.g1.x) * vec4(1.0, 0.0, 0.0, -1.0));
}

Motor line_motor_add(Line self, Motor other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + other.g0, vec4(self.g1.x, self.g1.y, self.g1.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + other.g1);
}

Motor line_motor_sub(Line self, Motor other) {
    return Motor(vec4(self.g0.x, self.g0.y, self.g0.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) - other.g0, vec4(self.g1.x, self.g1.y, self.g1.z, self.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) - other.g1);
}

FlatPoint line_flector_regressive_product(Line self, Flector other) {
    return FlatPoint(vec4(self.g0.y) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.y) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g1.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x, self.g1.x, self.g1.x, self.g0.x) * other.g1.wzyx * vec4(1.0, -1.0, 1.0, -1.0));
}

MultiVector line_multi_vector_add(Line self, MultiVector other) {
    return MultiVector(other.g0, other.g1, other.g2, other.g3, other.g4, other.g5, self.g0 + other.g6, self.g1 + other.g7, other.g8, other.g9);
}

MultiVector line_multi_vector_sub(Line self, MultiVector other) {
    return MultiVector(vec3(0.0) - other.g0, vec3(0.0) - other.g1, vec2(0.0) - other.g2, vec4(0.0) - other.g3, vec3(0.0) - other.g4, vec3(0.0) - other.g5, self.g0 - other.g6, self.g1 - other.g7, vec4(0.0) - other.g8, vec4(0.0) - other.g9);
}

Line line_scale(Line self, float other) {
    return line_scalar_geometric_product(self, Scalar(other));
}

Circle circle_zero() {
    return Circle(vec4(0.0), vec3(0.0), vec3(0.0));
}

Circle circle_one() {
    return Circle(vec4(0.0), vec3(0.0), vec3(0.0));
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

Scalar circle_dipole_regressive_product(Circle self, Dipole other) {
    return Scalar(0.0 - self.g0.x * other.g2.x - self.g0.y * other.g2.y - self.g0.z * other.g2.z - self.g0.w * other.g2.w - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z - self.g2.x * other.g0.x - self.g2.y * other.g0.y - self.g2.z * other.g0.z);
}

AntiScalar circle_dipole_outer_product(Circle self, Dipole other) {
    return AntiScalar(0.0 - self.g0.x * other.g2.x - self.g0.y * other.g2.y - self.g0.z * other.g2.z - self.g0.w * other.g2.w - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z - self.g2.x * other.g0.x - self.g2.y * other.g0.y - self.g2.z * other.g0.z);
}

RadialPoint circle_dipole_inner_product(Circle self, Dipole other) {
    return RadialPoint(vec3(self.g0.w) * other.g1, vec2(self.g0.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g2.x) * vec2(other.g1.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(-1.0, 0.0));
}

RadialPoint circle_dipole_right_contraction(Circle self, Dipole other) {
    return RadialPoint(vec3(self.g0.w) * other.g1, vec2(self.g0.y) * vec2(other.g1.y) * vec2(-1.0, 0.0) + vec2(self.g0.z) * vec2(other.g1.z) * vec2(-1.0, 0.0) + vec2(self.g2.x) * vec2(other.g1.x) * vec2(0.0, -1.0) + vec2(self.g2.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g2.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g0.x) * vec2(other.g1.x) * vec2(-1.0, 0.0));
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

Scalar circle_circle_left_contraction(Circle self, Circle other) {
    return Scalar(0.0 - self.g0.w * other.g0.w);
}

Scalar circle_circle_right_contraction(Circle self, Circle other) {
    return Scalar(0.0 - self.g0.w * other.g0.w);
}

Scalar circle_circle_scalar_product(Circle self, Circle other) {
    return Scalar(0.0 - self.g0.w * other.g0.w);
}

Dipole circle_plane_regressive_product(Circle self, Plane other) {
    return Dipole(vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w), vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g2.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g2.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.x, self.g2.x, self.g2.x, self.g1.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0));
}

Dipole circle_sphere_regressive_product(Circle self, Sphere other) {
    return Dipole(vec3(self.g0.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g1.x) * vec3(other.g0.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g1.y) * vec3(other.g0.x) * vec3(0.0, 1.0, 0.0) + vec3(self.g1.z) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0) + vec3(self.g0.x) * other.g1.xzy * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g0.w) * other.g1 + vec3(self.g2.x) * vec3(other.g0.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g2.y) * vec3(other.g0.x) * vec3(0.0, 1.0, 0.0) + vec3(self.g2.z) * vec3(other.g0.x) * vec3(0.0, 0.0, 1.0) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y), vec4(self.g1.y) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g1.y) * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g1.z) * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g2.x) * vec4(other.g1.z, other.g1.z, other.g1.y, other.g1.z) * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g2.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.z) * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g2.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.y) * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.y, other.g0.x, other.g0.x, other.g1.x) * vec4(1.0, 0.0, 0.0, -1.0));
}

Flector circle_motor_geometric_product(Circle self, Motor other) {
    return Flector(vec4(self.g0.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g0.w) + vec4(self.g0.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0), vec4(self.g0.y) * other.g1.zwxz * vec4(-1.0, 1.0, 1.0, 0.0) + vec4(self.g0.z) * other.g1.yxwy * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.w) + vec4(self.g0.x) * other.g1.wzyx * vec4(1.0, 1.0, -1.0, 0.0));
}

Plane circle_motor_outer_product(Circle self, Motor other) {
    return Plane(self.g0 * vec4(other.g1.w));
}

Motor circle_flector_geometric_product(Circle self, Flector other) {
    return Motor(vec4(self.g0.x) * vec4(other.g1.w, other.g0.z, other.g0.y, other.g0.x) * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g0.y) * vec4(other.g0.z, other.g1.w, other.g0.x, other.g0.y) * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g1.w, other.g0.z) * vec4(1.0, -1.0, 1.0, -1.0) - vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g0.w), vec4(0.0) - vec4(self.g0.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g1.w));
}

AntiScalar circle_flector_outer_product(Circle self, Flector other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
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

Scalar circle_multi_vector_scalar_product(Circle self, MultiVector other) {
    return Scalar(0.0 - self.g0.w * other.g8.w);
}

Scalar circle_squared_magnitude(Circle self) {
    return circle_circle_scalar_product(self, circle_reversal(self));
}

Scalar circle_magnitude(Circle self) {
    return Scalar(sqrt(circle_squared_magnitude(self).g0));
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

Plane plane_zero() {
    return Plane(vec4(0.0));
}

Plane plane_one() {
    return Plane(vec4(0.0));
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

RadialPoint plane_dipole_regressive_product(Plane self, Dipole other) {
    return RadialPoint(vec3(self.g0.y) * other.g1.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * other.g1.yxy * vec3(-1.0, 1.0, 0.0) - vec3(self.g0.w) * other.g0 + vec3(self.g0.x) * other.g1.xzy * vec3(0.0, -1.0, 1.0), vec2(self.g0.x) * vec2(other.g0.x, other.g2.x) * vec2(1.0, -1.0) + vec2(self.g0.y) * vec2(other.g0.y, other.g2.y) * vec2(1.0, -1.0) + vec2(self.g0.z) * vec2(other.g0.z, other.g2.z) * vec2(1.0, -1.0) + vec2(self.g0.x, self.g0.w) * vec2(other.g2.x, other.g2.w) * vec2(0.0, -1.0));
}

FlatPoint plane_dipole_inner_product(Plane self, Dipole other) {
    return FlatPoint(vec4(self.g0.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, -1.0));
}

FlatPoint plane_dipole_right_contraction(Plane self, Dipole other) {
    return FlatPoint(vec4(self.g0.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, -1.0));
}

FlatPoint plane_line_regressive_product(Plane self, Line other) {
    return FlatPoint(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

Dipole plane_circle_regressive_product(Plane self, Circle other) {
    return Dipole(vec3(self.g0.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0), vec3(self.g0.w) * vec3(other.g0.x, other.g0.y, other.g0.z) + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.w) * vec3(-1.0), vec4(self.g0.y) * vec4(other.g2.z, other.g2.z, other.g2.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g2.y, other.g2.x, other.g2.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g2.x, other.g2.z, other.g2.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
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

Sphere plane_sphere_add(Plane self, Sphere other) {
    return Sphere(vec2(self.g0.x, self.g0.w) * vec2(0.0, 1.0) + other.g0, vec3(self.g0.x, self.g0.y, self.g0.z) + other.g1);
}

Sphere plane_sphere_sub(Plane self, Sphere other) {
    return Sphere(vec2(self.g0.x, self.g0.w) * vec2(0.0, 1.0) - other.g0, vec3(self.g0.x, self.g0.y, self.g0.z) - other.g1);
}

Circle plane_sphere_regressive_product(Plane self, Sphere other) {
    return Circle(self.g0 * vec4(other.g0.x) * vec4(-1.0), vec3(self.g0.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x) * other.g1.xzy * vec3(0.0, 1.0, -1.0), vec3(0.0) - vec3(self.g0.w) * other.g1 + vec3(self.g0.x, self.g0.y, self.g0.z) * vec3(other.g0.y));
}

Flector plane_motor_regressive_product(Plane self, Motor other) {
    return Flector(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0), self.g0 * vec4(other.g0.w));
}

Flector plane_rotor_regressive_product(Plane self, Rotor other) {
    return Flector(vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0), self.g0 * vec4(other.g0.w));
}

Flector plane_flector_add(Plane self, Flector other) {
    return Flector(other.g0, self.g0 + other.g1);
}

Flector plane_flector_sub(Plane self, Flector other) {
    return Flector(vec4(0.0) - other.g0, self.g0 - other.g1);
}

MultiVector plane_multi_vector_add(Plane self, MultiVector other) {
    return MultiVector(other.g0, other.g1, other.g2, other.g3, other.g4, other.g5, other.g6, other.g7, other.g8, self.g0 + other.g9);
}

MultiVector plane_multi_vector_sub(Plane self, MultiVector other) {
    return MultiVector(vec3(0.0) - other.g0, vec3(0.0) - other.g1, vec2(0.0) - other.g2, vec4(0.0) - other.g3, vec3(0.0) - other.g4, vec3(0.0) - other.g5, vec3(0.0) - other.g6, vec3(0.0) - other.g7, vec4(0.0) - other.g8, self.g0 - other.g9);
}

Plane plane_scale(Plane self, float other) {
    return plane_scalar_geometric_product(self, Scalar(other));
}

Sphere sphere_zero() {
    return Sphere(vec2(0.0), vec3(0.0));
}

Sphere sphere_one() {
    return Sphere(vec2(0.0), vec3(0.0));
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

Scalar sphere_radial_point_regressive_product(Sphere self, RadialPoint other) {
    return Scalar(self.g0.x * other.g1.y + self.g0.y * other.g1.x + self.g1.x * other.g0.x + self.g1.y * other.g0.y + self.g1.z * other.g0.z);
}

AntiScalar sphere_radial_point_outer_product(Sphere self, RadialPoint other) {
    return AntiScalar(self.g0.x * other.g1.y + self.g0.y * other.g1.x + self.g1.x * other.g0.x + self.g1.y * other.g0.y + self.g1.z * other.g0.z);
}

RadialPoint sphere_flat_point_regressive_product(Sphere self, FlatPoint other) {
    return RadialPoint(vec3(self.g0.x) * vec3(other.g0.x, other.g0.y, other.g0.z), vec2(self.g1.x) * vec2(other.g0.x) * vec2(0.0, -1.0) + vec2(self.g1.y) * vec2(other.g0.y) * vec2(0.0, -1.0) + vec2(self.g1.z) * vec2(other.g0.z) * vec2(0.0, -1.0) + self.g0 * vec2(other.g0.w) * vec2(1.0, -1.0));
}

RadialPoint sphere_dipole_regressive_product(Sphere self, Dipole other) {
    return RadialPoint(vec3(self.g0.x) * vec3(other.g2.x, other.g2.y, other.g2.z) - vec3(self.g0.y) * other.g0 + vec3(self.g1.y) * other.g1.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * other.g1.yxy * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.x) * other.g1.xzy * vec3(0.0, -1.0, 1.0), vec2(self.g1.x) * vec2(other.g0.x, other.g2.x) * vec2(1.0, -1.0) + vec2(self.g1.y) * vec2(other.g0.y, other.g2.y) * vec2(1.0, -1.0) + vec2(self.g1.z) * vec2(other.g0.z, other.g2.z) * vec2(1.0, -1.0) + self.g0 * vec2(other.g2.w) * vec2(1.0, -1.0));
}

Dipole sphere_line_regressive_product(Sphere self, Line other) {
    return Dipole(vec3(self.g0.x) * other.g0, vec3(self.g0.x) * other.g1, vec4(self.g1.x) * vec4(other.g1.z, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.y, self.g0.y, self.g0.y, self.g0.x) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Dipole sphere_circle_regressive_product(Sphere self, Circle other) {
    return Dipole(vec3(self.g0.x) * other.g1 + vec3(self.g1.y) * vec3(other.g0.z, other.g0.z, other.g0.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g0.y, other.g0.x, other.g0.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g1.x) * vec3(other.g0.x, other.g0.z, other.g0.y) * vec3(0.0, -1.0, 1.0), vec3(self.g0.x) * other.g2 + vec3(self.g0.y) * vec3(other.g0.x, other.g0.y, other.g0.z) + self.g1 * vec3(other.g0.w) * vec3(-1.0), vec4(self.g1.x) * vec4(other.g2.z, other.g2.z, other.g2.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g2.z, other.g2.z, other.g2.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g2.y, other.g2.x, other.g2.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.y, self.g0.y, self.g0.y, self.g0.x) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0));
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

Rotor sphere_motor_geometric_product(Sphere self, Motor other) {
    return Rotor(vec4(self.g0.x) * other.g1);
}

AntiScalar sphere_motor_outer_product(Sphere self, Motor other) {
    return AntiScalar(self.g0.x * other.g1.w);
}

MultiVector sphere_multi_vector_add(Sphere self, MultiVector other) {
    return MultiVector(vec3(self.g0.x) * vec3(0.0, 1.0, 0.0) + other.g0, other.g1, other.g2, other.g3, other.g4, other.g5, other.g6, other.g7, other.g8, vec4(self.g1.x, self.g1.y, self.g1.z, self.g0.y) + other.g9);
}

MultiVector sphere_multi_vector_sub(Sphere self, MultiVector other) {
    return MultiVector(vec3(self.g0.x) * vec3(0.0, 1.0, 0.0) - other.g0, vec3(0.0) - other.g1, vec2(0.0) - other.g2, vec4(0.0) - other.g3, vec3(0.0) - other.g4, vec3(0.0) - other.g5, vec3(0.0) - other.g6, vec3(0.0) - other.g7, vec4(0.0) - other.g8, vec4(self.g1.x, self.g1.y, self.g1.z, self.g0.y) - other.g9);
}

Sphere sphere_scale(Sphere self, float other) {
    return sphere_scalar_geometric_product(self, Scalar(other));
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

Flector motor_circle_geometric_product(Motor self, Circle other) {
    return Flector(vec4(self.g1.x) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g1.z) * other.g0.wwwz * vec4(0.0, 0.0, -1.0, 1.0) + self.g0.xxxw * other.g0.xxxw * vec4(0.0, 0.0, 0.0, 1.0), vec4(self.g1.x) * other.g0.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) - vec4(self.g1.w) * other.g0 + self.g0.xyzx * other.g0.wwwx * vec4(1.0, 1.0, 1.0, 0.0));
}

Plane motor_circle_outer_product(Motor self, Circle other) {
    return Plane(vec4(0.0) - vec4(self.g1.w) * other.g0);
}

Flector motor_plane_regressive_product(Motor self, Plane other) {
    return Flector(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x, self.g1.x, self.g1.x, self.g0.x) * other.g0.wzyx * vec4(1.0, -1.0, 1.0, -1.0), vec4(self.g0.w) * other.g0);
}

Rotor motor_sphere_geometric_product(Motor self, Sphere other) {
    return Rotor(self.g1 * vec4(other.g0.x));
}

AntiScalar motor_sphere_outer_product(Motor self, Sphere other) {
    return AntiScalar(self.g1.w * other.g0.x);
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

Flector motor_flector_regressive_product(Motor self, Flector other) {
    return Flector(vec4(self.g0.y) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g0 + vec4(self.g1.y) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g1.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.x, self.g1.x, self.g1.x, self.g0.x) * other.g1.wzyx * vec4(1.0, -1.0, 1.0, -1.0), vec4(self.g0.w) * other.g1);
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

RadialPoint rotor_radial_point_regressive_product(Rotor self, RadialPoint other) {
    return RadialPoint(vec3(self.g0.w) * other.g0, vec2(self.g0.w) * other.g1);
}

FlatPoint rotor_flat_point_regressive_product(Rotor self, FlatPoint other) {
    return FlatPoint(vec4(self.g0.w) * other.g0);
}

Rotor rotor_dipole_geometric_product(Rotor self, Dipole other) {
    return Rotor(vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

AntiScalar rotor_dipole_outer_product(Rotor self, Dipole other) {
    return AntiScalar(0.0 - self.g0.x * other.g1.x - self.g0.y * other.g1.y - self.g0.z * other.g1.z);
}

Flector rotor_plane_regressive_product(Rotor self, Plane other) {
    return Flector(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0), vec4(self.g0.w) * other.g0);
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

Motor rotor_translator_regressive_product(Rotor self, Translator other) {
    return Motor(self.g0 * vec4(other.g0.w), vec4(self.g0.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + self.g0.wwwx * other.g0.xyzx * vec4(1.0, 1.0, 1.0, -1.0));
}

Flector rotor_flector_regressive_product(Rotor self, Flector other) {
    return Flector(vec4(self.g0.y) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.w) * other.g0 + vec4(self.g0.x) * other.g1.wxxx * vec4(1.0, 0.0, 0.0, -1.0), vec4(self.g0.w) * other.g1);
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

Translator translator_scalar_right_contraction(Translator self, Scalar other) {
    return Translator(self.g0 * vec4(other.g0));
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

RadialPoint translator_radial_point_regressive_product(Translator self, RadialPoint other) {
    return RadialPoint(vec3(self.g0.w) * other.g0, vec2(self.g0.w) * other.g1);
}

Plane translator_radial_point_outer_product(Translator self, RadialPoint other) {
    return Plane(vec4(self.g0.y) * vec4(other.g1.x, other.g1.x, other.g1.x, other.g0.y) * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g1.x, other.g1.x, other.g1.x, other.g0.z) * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.x, other.g1.x, other.g0.x) * vec4(-1.0, 0.0, 0.0, 1.0));
}

FlatPoint translator_flat_point_regressive_product(Translator self, FlatPoint other) {
    return FlatPoint(vec4(self.g0.w) * other.g0);
}

Motor translator_dipole_geometric_product(Translator self, Dipole other) {
    return Motor(vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g0.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.x, other.g0.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.z, other.g0.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0), vec4(self.g0.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g1.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

AntiScalar translator_dipole_outer_product(Translator self, Dipole other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z);
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

Motor translator_rotor_regressive_product(Translator self, Rotor other) {
    return Motor(vec4(self.g0.w) * other.g0, vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.z) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g0.x) * other.g0.wxxx * vec4(1.0, 0.0, 0.0, -1.0));
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

Flector translator_flector_regressive_product(Translator self, Flector other) {
    return Flector(vec4(self.g0.y) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g0.z) * other.g1.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.w) * other.g0 + vec4(self.g0.x) * other.g1.xzyx * vec4(0.0, -1.0, 1.0, 0.0), vec4(self.g0.w) * other.g1);
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

FlatPoint flector_line_regressive_product(Flector self, Line other) {
    return FlatPoint(vec4(self.g1.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.w) * vec4(other.g0.x, other.g0.y, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g1.x, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0));
}

Motor flector_circle_geometric_product(Flector self, Circle other) {
    return Motor(vec4(self.g0.y) * other.g0.zzxy * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.yxyz * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.w) * vec4(0.0, -1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.w) * vec4(0.0, 0.0, -1.0, 0.0) + vec4(self.g1.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g1.x, self.g0.x, self.g0.x, self.g0.x) * other.g0.wzyx * vec4(-1.0, 1.0, -1.0, -1.0), vec4(self.g0.x, self.g0.y, self.g0.z, self.g1.w) * vec4(other.g0.w));
}

AntiScalar flector_circle_outer_product(Flector self, Circle other) {
    return AntiScalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
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

Flector flector_motor_regressive_product(Flector self, Motor other) {
    return Flector(vec4(self.g1.x) * vec4(other.g1.z, other.g1.z, other.g1.y, other.g0.x) * vec4(0.0, 1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g0.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g0.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g1.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + self.g0 * vec4(other.g0.w), self.g1 * vec4(other.g0.w));
}

Flector flector_rotor_regressive_product(Flector self, Rotor other) {
    return Flector(vec4(self.g1.x) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g1.w) * other.g0.xyzx * vec4(1.0, 1.0, 1.0, 0.0) + self.g0 * vec4(other.g0.w), self.g1 * vec4(other.g0.w));
}

Flector flector_translator_regressive_product(Flector self, Translator other) {
    return Flector(vec4(self.g1.x) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g1.z) * other.g0.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + self.g0 * vec4(other.g0.w), self.g1 * vec4(other.g0.w));
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

MultiVector flector_multi_vector_add(Flector self, MultiVector other) {
    return MultiVector(other.g0, other.g1, other.g2, self.g0 + other.g3, other.g4, other.g5, other.g6, other.g7, other.g8, self.g1 + other.g9);
}

MultiVector flector_multi_vector_sub(Flector self, MultiVector other) {
    return MultiVector(vec3(0.0) - other.g0, vec3(0.0) - other.g1, vec2(0.0) - other.g2, self.g0 - other.g3, vec3(0.0) - other.g4, vec3(0.0) - other.g5, vec3(0.0) - other.g6, vec3(0.0) - other.g7, vec4(0.0) - other.g8, self.g1 - other.g9);
}

Flector flector_scale(Flector self, float other) {
    return flector_scalar_geometric_product(self, Scalar(other));
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

Scalar multi_vector_dipole_scalar_product(MultiVector self, Dipole other) {
    return Scalar(0.0 - self.g5.x * other.g1.x - self.g5.y * other.g1.y - self.g5.z * other.g1.z);
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

Scalar multi_vector_circle_scalar_product(MultiVector self, Circle other) {
    return Scalar(0.0 - self.g8.w * other.g0.w);
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

Sphere multi_vector_sphere_into(MultiVector self) {
    return Sphere(vec2(self.g0.y, self.g9.w), vec3(self.g9.x, self.g9.y, self.g9.z));
}

MultiVector multi_vector_sphere_add(MultiVector self, Sphere other) {
    return MultiVector(self.g0 + vec3(other.g0.x) * vec3(0.0, 1.0, 0.0), self.g1, self.g2, self.g3, self.g4, self.g5, self.g6, self.g7, self.g8, self.g9 + vec4(other.g1.x, other.g1.y, other.g1.z, other.g0.y));
}

MultiVector multi_vector_sphere_sub(MultiVector self, Sphere other) {
    return MultiVector(self.g0 - vec3(other.g0.x) * vec3(0.0, 1.0, 0.0), self.g1, self.g2, self.g3, self.g4, self.g5, self.g6, self.g7, self.g8, self.g9 - vec4(other.g1.x, other.g1.y, other.g1.z, other.g0.y));
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

Flector multi_vector_flector_into(MultiVector self) {
    return Flector(self.g3, self.g9);
}

MultiVector multi_vector_flector_add(MultiVector self, Flector other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3 + other.g0, self.g4, self.g5, self.g6, self.g7, self.g8, self.g9 + other.g1);
}

MultiVector multi_vector_flector_sub(MultiVector self, Flector other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3 - other.g0, self.g4, self.g5, self.g6, self.g7, self.g8, self.g9 - other.g1);
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

MultiVector multi_vector_multi_vector_left_contraction(MultiVector self, MultiVector other) {
    return MultiVector(vec3(self.g0.x) * other.g0 + vec3(self.g1.y) * vec3(other.g1.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g1.z) * vec3(other.g1.z) * vec3(1.0, 0.0, 0.0) + vec3(self.g5.x) * vec3(other.g5.x) * vec3(-1.0, 0.0, 0.0) + vec3(self.g5.y) * vec3(other.g5.y) * vec3(-1.0, 0.0, 0.0) + vec3(self.g5.z) * vec3(other.g5.z) * vec3(-1.0, 0.0, 0.0) + vec3(self.g8.w) * vec3(other.g8.w) * vec3(-1.0, 0.0, 0.0) + vec3(self.g1.x) * vec3(other.g1.x) * vec3(1.0, 0.0, 0.0), vec3(self.g0.x) * other.g1 + vec3(self.g1.y) * other.g5.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g1.z) * other.g5.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g5.x) * vec3(other.g8.w) * vec3(1.0, 0.0, 0.0) + vec3(self.g5.y) * vec3(other.g8.w) * vec3(0.0, 1.0, 0.0) + vec3(self.g5.z) * vec3(other.g8.w) * vec3(0.0, 0.0, 1.0) + vec3(self.g1.x) * other.g5.xzy * vec3(0.0, 1.0, -1.0), vec2(self.g0.x) * other.g2 + vec2(self.g1.x) * vec2(other.g4.x, other.g3.x) * vec2(-1.0, 1.0) + vec2(self.g1.y) * vec2(other.g4.y, other.g3.y) * vec2(-1.0, 1.0) + vec2(self.g1.z) * vec2(other.g4.z, other.g3.z) * vec2(-1.0, 1.0) - vec2(self.g5.x) * vec2(other.g8.x, other.g7.x) - vec2(self.g5.y) * vec2(other.g8.y, other.g7.y) - vec2(self.g5.z) * vec2(other.g8.z, other.g7.z) + vec2(self.g8.w) * vec2(other.g0.y, other.g9.w) * vec2(1.0, -1.0), vec4(self.g0.x) * other.g3 + vec4(self.g1.y) * vec4(other.g7.z, other.g7.z, other.g7.x, other.g6.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g7.y, other.g7.x, other.g7.y, other.g6.z) * vec4(1.0, -1.0, 0.0, -1.0) + vec4(self.g5.x) * other.g9.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g5.y) * other.g9.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g5.z) * other.g9.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g8.w) * vec4(other.g0.z) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g7.x, other.g7.z, other.g7.y, other.g6.x) * vec4(0.0, 1.0, -1.0, -1.0), vec3(self.g0.x) * other.g4 + vec3(self.g1.y) * vec3(other.g8.z, other.g8.z, other.g8.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g8.y, other.g8.x, other.g8.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g5.x) * vec3(other.g0.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g5.y) * vec3(other.g0.y) * vec3(0.0, 1.0, 0.0) + vec3(self.g5.z) * vec3(other.g0.y) * vec3(0.0, 0.0, 1.0) + vec3(self.g1.x) * vec3(other.g8.x, other.g8.z, other.g8.y) * vec3(0.0, -1.0, 1.0), vec3(self.g0.x) * other.g5 + self.g1 * vec3(other.g8.w) * vec3(-1.0), vec3(self.g0.x) * other.g6 + vec3(self.g1.y) * vec3(other.g9.z, other.g9.z, other.g9.x) * vec3(1.0, 0.0, -1.0) + vec3(self.g1.z) * vec3(other.g9.y, other.g9.x, other.g9.y) * vec3(-1.0, 1.0, 0.0) + vec3(self.g5.x) * vec3(other.g0.z) * vec3(1.0, 0.0, 0.0) + vec3(self.g5.y) * vec3(other.g0.z) * vec3(0.0, 1.0, 0.0) + vec3(self.g5.z) * vec3(other.g0.z) * vec3(0.0, 0.0, 1.0) + vec3(self.g1.x) * vec3(other.g9.x, other.g9.z, other.g9.y) * vec3(0.0, -1.0, 1.0), vec3(self.g0.x) * other.g7 + self.g1 * vec3(other.g9.w) * vec3(-1.0), vec4(self.g0.x) * other.g8 + vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.x) * vec4(other.g0.y, other.g0.y, other.g0.y, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0), vec4(self.g0.x) * other.g9 + vec4(self.g1.x, self.g1.y, self.g1.z, self.g1.x) * vec4(other.g0.z, other.g0.z, other.g0.z, other.g0.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

MultiVector multi_vector_multi_vector_right_contraction(MultiVector self, MultiVector other) {
    return MultiVector(vec3(self.g1.x) * vec3(other.g1.x) * vec3(1.0, 0.0, 0.0) + vec3(self.g1.y) * vec3(other.g1.y) * vec3(1.0, 0.0, 0.0) + vec3(self.g1.z) * vec3(other.g1.z) * vec3(1.0, 0.0, 0.0) + vec3(self.g5.x) * vec3(other.g5.x) * vec3(-1.0, 0.0, 0.0) + vec3(self.g5.y) * vec3(other.g5.y) * vec3(-1.0, 0.0, 0.0) + vec3(self.g5.z) * vec3(other.g5.z) * vec3(-1.0, 0.0, 0.0) + vec3(self.g8.w) * vec3(other.g8.w) * vec3(-1.0, 0.0, 0.0) + self.g0 * vec3(other.g0.x), vec3(self.g5.x) * other.g1.zzy * vec3(0.0, 1.0, -1.0) + vec3(self.g5.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g5.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + vec3(self.g8.w) * other.g5 + self.g1 * vec3(other.g0.x), vec2(self.g2.x) * vec2(other.g0.x) * vec2(1.0, 0.0) + vec2(self.g2.y) * vec2(other.g0.x) * vec2(0.0, 1.0) + vec2(self.g3.x) * vec2(other.g1.x) * vec2(0.0, -1.0) + vec2(self.g3.y) * vec2(other.g1.y) * vec2(0.0, -1.0) + vec2(self.g3.z) * vec2(other.g1.z) * vec2(0.0, -1.0) + vec2(self.g4.x) * vec2(other.g1.x) * vec2(1.0, 0.0) + vec2(self.g4.y) * vec2(other.g1.y) * vec2(1.0, 0.0) + vec2(self.g4.z) * vec2(other.g1.z) * vec2(1.0, 0.0) + vec2(self.g7.x) * vec2(other.g5.x) * vec2(0.0, -1.0) + vec2(self.g7.y) * vec2(other.g5.y) * vec2(0.0, -1.0) + vec2(self.g7.z) * vec2(other.g5.z) * vec2(0.0, -1.0) + vec2(self.g8.x) * vec2(other.g5.x) * vec2(-1.0, 0.0) + vec2(self.g8.y) * vec2(other.g5.y) * vec2(-1.0, 0.0) + vec2(self.g8.z) * vec2(other.g5.z) * vec2(-1.0, 0.0) + vec2(self.g9.w) * vec2(other.g8.w) * vec2(0.0, 1.0) + vec2(self.g0.y, self.g0.x) * vec2(other.g8.w, other.g8.x) * vec2(-1.0, 0.0), vec4(self.g3.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g3.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g3.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g3.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g6.x) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g6.y) * vec4(other.g1.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g6.z) * vec4(other.g1.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g7.x) * vec4(other.g1.z, other.g1.z, other.g1.y, other.g1.z) * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g7.y) * vec4(other.g1.z, other.g1.z, other.g1.x, other.g1.z) * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g7.z) * vec4(other.g1.y, other.g1.x, other.g1.y, other.g1.y) * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g9.x) * vec4(other.g5.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g9.y) * vec4(other.g5.y) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g9.z) * vec4(other.g5.z) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g9.w) * vec4(other.g5.x, other.g5.y, other.g5.z, other.g5.x) * vec4(1.0, 1.0, 1.0, 0.0) + vec4(self.g0.x, self.g0.x, self.g0.x, self.g0.z) * other.g8.xxxw * vec4(0.0, 0.0, 0.0, 1.0), vec3(self.g0.y) * other.g5 + vec3(self.g8.x) * other.g1.zzy * vec3(0.0, 1.0, -1.0) + vec3(self.g8.y) * other.g1.zzx * vec3(-1.0, 0.0, 1.0) + vec3(self.g8.z) * other.g1.yxy * vec3(1.0, -1.0, 0.0) + self.g4 * vec3(other.g0.x), vec3(0.0) - vec3(self.g8.w) * other.g1 + self.g5 * vec3(other.g0.x), vec3(self.g0.z) * other.g5 + vec3(self.g9.x) * other.g1.zzy * vec3(0.0, -1.0, 1.0) + vec3(self.g9.y) * other.g1.zzx * vec3(1.0, 0.0, -1.0) + vec3(self.g9.z) * other.g1.yxy * vec3(-1.0, 1.0, 0.0) + self.g6 * vec3(other.g0.x), vec3(self.g9.w) * other.g1 + self.g7 * vec3(other.g0.x), vec4(self.g8.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g8.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g8.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g8.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.y, self.g0.y, self.g0.y, self.g0.x) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(-1.0, -1.0, -1.0, 0.0), vec4(self.g9.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g9.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g9.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g9.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.z, self.g0.z, self.g0.z, self.g0.x) * vec4(other.g1.x, other.g1.y, other.g1.z, other.g1.x) * vec4(1.0, 1.0, 1.0, 0.0));
}

Scalar multi_vector_multi_vector_scalar_product(MultiVector self, MultiVector other) {
    return Scalar(self.g0.x * other.g0.x + self.g1.x * other.g1.x + self.g1.y * other.g1.y + self.g1.z * other.g1.z - self.g5.x * other.g5.x - self.g5.y * other.g5.y - self.g5.z * other.g5.z - self.g8.w * other.g8.w);
}

Scalar multi_vector_squared_magnitude(MultiVector self) {
    return multi_vector_multi_vector_scalar_product(self, multi_vector_reversal(self));
}

Scalar multi_vector_magnitude(MultiVector self) {
    return Scalar(sqrt(multi_vector_squared_magnitude(self).g0));
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

FlatPoint flat_point_scalar_geometric_quotient(FlatPoint self, Scalar other) {
    return flat_point_scalar_geometric_product(self, scalar_inverse(other));
}

Motor flector_circle_geometric_quotient(Flector self, Circle other) {
    return flector_circle_geometric_product(self, circle_inverse(other));
}

Flector flector_dipole_geometric_quotient(Flector self, Dipole other) {
    return flector_dipole_geometric_product(self, dipole_inverse(other));
}

Motor flector_radial_point_geometric_quotient(Flector self, RadialPoint other) {
    return flector_radial_point_geometric_product(self, radial_point_inverse(other));
}

Flector flector_scalar_geometric_quotient(Flector self, Scalar other) {
    return flector_scalar_geometric_product(self, scalar_inverse(other));
}

Motor line_dipole_geometric_quotient(Line self, Dipole other) {
    return line_dipole_geometric_product(self, dipole_inverse(other));
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

Motor motor_dipole_geometric_quotient(Motor self, Dipole other) {
    return motor_dipole_geometric_product(self, dipole_inverse(other));
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

MultiVector multi_vector_dipole_geometric_quotient(MultiVector self, Dipole other) {
    return multi_vector_dipole_geometric_product(self, dipole_inverse(other));
}

Dipole multi_vector_dipole_transformation(MultiVector self, Dipole other) {
    return multi_vector_dipole_into(multi_vector_multi_vector_geometric_product(multi_vector_dipole_geometric_product(self, other), multi_vector_reversal(self)));
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

Sphere sphere_scalar_geometric_quotient(Sphere self, Scalar other) {
    return sphere_scalar_geometric_product(self, scalar_inverse(other));
}

Motor translator_dipole_geometric_quotient(Translator self, Dipole other) {
    return translator_dipole_geometric_product(self, dipole_inverse(other));
}

Translator translator_scalar_geometric_quotient(Translator self, Scalar other) {
    return translator_scalar_geometric_product(self, scalar_inverse(other));
}


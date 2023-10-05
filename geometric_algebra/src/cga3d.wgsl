struct Scalar {
    g0_: f32,
}

struct AntiScalar {
    g0_: f32,
}

struct RadialPoint {
    g0_: vec3<f32>,
    g1_: vec2<f32>,
}

struct FlatPoint {
    g0_: vec4<f32>,
}

struct Dipole {
    g0_: vec3<f32>,
    g1_: vec3<f32>,
    g2_: vec4<f32>,
}

struct Line {
    g0_: vec3<f32>,
    g1_: vec3<f32>,
}

struct Circle {
    g0_: vec4<f32>,
    g1_: vec3<f32>,
    g2_: vec3<f32>,
}

struct Plane {
    g0_: vec4<f32>,
}

struct Sphere {
    g0_: f32,
    g1_: vec4<f32>,
}

struct Motor {
    g0_: vec4<f32>,
    g1_: vec4<f32>,
}

struct Rotor {
    g0_: vec4<f32>,
}

struct Translator {
    g0_: vec4<f32>,
}

struct Flector {
    g0_: vec4<f32>,
    g1_: vec4<f32>,
}

struct MultiVector {
    g0_: vec3<f32>,
    g1_: vec3<f32>,
    g2_: vec2<f32>,
    g3_: vec4<f32>,
    g4_: vec3<f32>,
    g5_: vec3<f32>,
    g6_: vec3<f32>,
    g7_: vec3<f32>,
    g8_: vec4<f32>,
    g9_: vec4<f32>,
}

fn scalar_zero() -> Scalar {
    return Scalar(0.0);
}

fn scalar_one() -> Scalar {
    return Scalar(1.0);
}

fn scalar_neg(self_: Scalar) -> Scalar {
    var self_1: Scalar;

    self_1 = self_;
    let _e2: Scalar = self_1;
    return Scalar((_e2.g0_ * -(1.0)));
}

fn scalar_automorphism(self_2: Scalar) -> Scalar {
    var self_3: Scalar;

    self_3 = self_2;
    let _e2: Scalar = self_3;
    return Scalar(_e2.g0_);
}

fn scalar_reversal(self_4: Scalar) -> Scalar {
    var self_5: Scalar;

    self_5 = self_4;
    let _e2: Scalar = self_5;
    return Scalar(_e2.g0_);
}

fn scalar_conjugation(self_6: Scalar) -> Scalar {
    var self_7: Scalar;

    self_7 = self_6;
    let _e2: Scalar = self_7;
    return Scalar(_e2.g0_);
}

fn scalar_scalar_add(self_8: Scalar, other: Scalar) -> Scalar {
    var self_9: Scalar;
    var other_1: Scalar;

    self_9 = self_8;
    other_1 = other;
    let _e4: Scalar = self_9;
    let _e6: Scalar = other_1;
    return Scalar((_e4.g0_ + _e6.g0_));
}

fn scalar_scalar_sub(self_10: Scalar, other_2: Scalar) -> Scalar {
    var self_11: Scalar;
    var other_3: Scalar;

    self_11 = self_10;
    other_3 = other_2;
    let _e4: Scalar = self_11;
    let _e6: Scalar = other_3;
    return Scalar((_e4.g0_ - _e6.g0_));
}

fn scalar_scalar_mul(self_12: Scalar, other_4: Scalar) -> Scalar {
    var self_13: Scalar;
    var other_5: Scalar;

    self_13 = self_12;
    other_5 = other_4;
    let _e4: Scalar = self_13;
    let _e6: Scalar = other_5;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_scalar_div(self_14: Scalar, other_6: Scalar) -> Scalar {
    var self_15: Scalar;
    var other_7: Scalar;

    self_15 = self_14;
    other_7 = other_6;
    let _e4: Scalar = self_15;
    let _e8: Scalar = other_7;
    return Scalar((((_e4.g0_ * 1.0) / _e8.g0_) * 1.0));
}

fn scalar_scalar_geometric_product(self_16: Scalar, other_8: Scalar) -> Scalar {
    var self_17: Scalar;
    var other_9: Scalar;

    self_17 = self_16;
    other_9 = other_8;
    let _e4: Scalar = self_17;
    let _e6: Scalar = other_9;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_scalar_outer_product(self_18: Scalar, other_10: Scalar) -> Scalar {
    var self_19: Scalar;
    var other_11: Scalar;

    self_19 = self_18;
    other_11 = other_10;
    let _e4: Scalar = self_19;
    let _e6: Scalar = other_11;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_scalar_inner_product(self_20: Scalar, other_12: Scalar) -> Scalar {
    var self_21: Scalar;
    var other_13: Scalar;

    self_21 = self_20;
    other_13 = other_12;
    let _e4: Scalar = self_21;
    let _e6: Scalar = other_13;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_scalar_left_contraction(self_22: Scalar, other_14: Scalar) -> Scalar {
    var self_23: Scalar;
    var other_15: Scalar;

    self_23 = self_22;
    other_15 = other_14;
    let _e4: Scalar = self_23;
    let _e6: Scalar = other_15;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_scalar_right_contraction(self_24: Scalar, other_16: Scalar) -> Scalar {
    var self_25: Scalar;
    var other_17: Scalar;

    self_25 = self_24;
    other_17 = other_16;
    let _e4: Scalar = self_25;
    let _e6: Scalar = other_17;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_scalar_scalar_product(self_26: Scalar, other_18: Scalar) -> Scalar {
    var self_27: Scalar;
    var other_19: Scalar;

    self_27 = self_26;
    other_19 = other_18;
    let _e4: Scalar = self_27;
    let _e6: Scalar = other_19;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_geometric_product(self_28: Scalar, other_20: AntiScalar) -> AntiScalar {
    var self_29: Scalar;
    var other_21: AntiScalar;

    self_29 = self_28;
    other_21 = other_20;
    let _e4: Scalar = self_29;
    let _e6: AntiScalar = other_21;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_outer_product(self_30: Scalar, other_22: AntiScalar) -> AntiScalar {
    var self_31: Scalar;
    var other_23: AntiScalar;

    self_31 = self_30;
    other_23 = other_22;
    let _e4: Scalar = self_31;
    let _e6: AntiScalar = other_23;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_inner_product(self_32: Scalar, other_24: AntiScalar) -> AntiScalar {
    var self_33: Scalar;
    var other_25: AntiScalar;

    self_33 = self_32;
    other_25 = other_24;
    let _e4: Scalar = self_33;
    let _e6: AntiScalar = other_25;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_left_contraction(self_34: Scalar, other_26: AntiScalar) -> AntiScalar {
    var self_35: Scalar;
    var other_27: AntiScalar;

    self_35 = self_34;
    other_27 = other_26;
    let _e4: Scalar = self_35;
    let _e6: AntiScalar = other_27;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn scalar_radial_point_geometric_product(self_36: Scalar, other_28: RadialPoint) -> RadialPoint {
    var self_37: Scalar;
    var other_29: RadialPoint;

    self_37 = self_36;
    other_29 = other_28;
    let _e4: Scalar = self_37;
    let _e7: RadialPoint = other_29;
    let _e10: Scalar = self_37;
    let _e13: RadialPoint = other_29;
    return RadialPoint((vec3<f32>(_e4.g0_) * _e7.g0_), (vec2<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_radial_point_outer_product(self_38: Scalar, other_30: RadialPoint) -> RadialPoint {
    var self_39: Scalar;
    var other_31: RadialPoint;

    self_39 = self_38;
    other_31 = other_30;
    let _e4: Scalar = self_39;
    let _e7: RadialPoint = other_31;
    let _e10: Scalar = self_39;
    let _e13: RadialPoint = other_31;
    return RadialPoint((vec3<f32>(_e4.g0_) * _e7.g0_), (vec2<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_radial_point_inner_product(self_40: Scalar, other_32: RadialPoint) -> RadialPoint {
    var self_41: Scalar;
    var other_33: RadialPoint;

    self_41 = self_40;
    other_33 = other_32;
    let _e4: Scalar = self_41;
    let _e7: RadialPoint = other_33;
    let _e10: Scalar = self_41;
    let _e13: RadialPoint = other_33;
    return RadialPoint((vec3<f32>(_e4.g0_) * _e7.g0_), (vec2<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_radial_point_left_contraction(self_42: Scalar, other_34: RadialPoint) -> RadialPoint {
    var self_43: Scalar;
    var other_35: RadialPoint;

    self_43 = self_42;
    other_35 = other_34;
    let _e4: Scalar = self_43;
    let _e7: RadialPoint = other_35;
    let _e10: Scalar = self_43;
    let _e13: RadialPoint = other_35;
    return RadialPoint((vec3<f32>(_e4.g0_) * _e7.g0_), (vec2<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_flat_point_geometric_product(self_44: Scalar, other_36: FlatPoint) -> FlatPoint {
    var self_45: Scalar;
    var other_37: FlatPoint;

    self_45 = self_44;
    other_37 = other_36;
    let _e4: Scalar = self_45;
    let _e7: FlatPoint = other_37;
    return FlatPoint((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_flat_point_outer_product(self_46: Scalar, other_38: FlatPoint) -> FlatPoint {
    var self_47: Scalar;
    var other_39: FlatPoint;

    self_47 = self_46;
    other_39 = other_38;
    let _e4: Scalar = self_47;
    let _e7: FlatPoint = other_39;
    return FlatPoint((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_flat_point_inner_product(self_48: Scalar, other_40: FlatPoint) -> FlatPoint {
    var self_49: Scalar;
    var other_41: FlatPoint;

    self_49 = self_48;
    other_41 = other_40;
    let _e4: Scalar = self_49;
    let _e7: FlatPoint = other_41;
    return FlatPoint((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_flat_point_left_contraction(self_50: Scalar, other_42: FlatPoint) -> FlatPoint {
    var self_51: Scalar;
    var other_43: FlatPoint;

    self_51 = self_50;
    other_43 = other_42;
    let _e4: Scalar = self_51;
    let _e7: FlatPoint = other_43;
    return FlatPoint((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_dipole_geometric_product(self_52: Scalar, other_44: Dipole) -> Dipole {
    var self_53: Scalar;
    var other_45: Dipole;

    self_53 = self_52;
    other_45 = other_44;
    let _e4: Scalar = self_53;
    let _e7: Dipole = other_45;
    let _e10: Scalar = self_53;
    let _e13: Dipole = other_45;
    let _e16: Scalar = self_53;
    let _e19: Dipole = other_45;
    return Dipole((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_), (vec4<f32>(_e16.g0_) * _e19.g2_));
}

fn scalar_dipole_outer_product(self_54: Scalar, other_46: Dipole) -> Dipole {
    var self_55: Scalar;
    var other_47: Dipole;

    self_55 = self_54;
    other_47 = other_46;
    let _e4: Scalar = self_55;
    let _e7: Dipole = other_47;
    let _e10: Scalar = self_55;
    let _e13: Dipole = other_47;
    let _e16: Scalar = self_55;
    let _e19: Dipole = other_47;
    return Dipole((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_), (vec4<f32>(_e16.g0_) * _e19.g2_));
}

fn scalar_dipole_inner_product(self_56: Scalar, other_48: Dipole) -> Dipole {
    var self_57: Scalar;
    var other_49: Dipole;

    self_57 = self_56;
    other_49 = other_48;
    let _e4: Scalar = self_57;
    let _e7: Dipole = other_49;
    let _e10: Scalar = self_57;
    let _e13: Dipole = other_49;
    let _e16: Scalar = self_57;
    let _e19: Dipole = other_49;
    return Dipole((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_), (vec4<f32>(_e16.g0_) * _e19.g2_));
}

fn scalar_dipole_left_contraction(self_58: Scalar, other_50: Dipole) -> Dipole {
    var self_59: Scalar;
    var other_51: Dipole;

    self_59 = self_58;
    other_51 = other_50;
    let _e4: Scalar = self_59;
    let _e7: Dipole = other_51;
    let _e10: Scalar = self_59;
    let _e13: Dipole = other_51;
    let _e16: Scalar = self_59;
    let _e19: Dipole = other_51;
    return Dipole((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_), (vec4<f32>(_e16.g0_) * _e19.g2_));
}

fn scalar_line_geometric_product(self_60: Scalar, other_52: Line) -> Line {
    var self_61: Scalar;
    var other_53: Line;

    self_61 = self_60;
    other_53 = other_52;
    let _e4: Scalar = self_61;
    let _e7: Line = other_53;
    let _e10: Scalar = self_61;
    let _e13: Line = other_53;
    return Line((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_line_outer_product(self_62: Scalar, other_54: Line) -> Line {
    var self_63: Scalar;
    var other_55: Line;

    self_63 = self_62;
    other_55 = other_54;
    let _e4: Scalar = self_63;
    let _e7: Line = other_55;
    let _e10: Scalar = self_63;
    let _e13: Line = other_55;
    return Line((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_line_inner_product(self_64: Scalar, other_56: Line) -> Line {
    var self_65: Scalar;
    var other_57: Line;

    self_65 = self_64;
    other_57 = other_56;
    let _e4: Scalar = self_65;
    let _e7: Line = other_57;
    let _e10: Scalar = self_65;
    let _e13: Line = other_57;
    return Line((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_line_left_contraction(self_66: Scalar, other_58: Line) -> Line {
    var self_67: Scalar;
    var other_59: Line;

    self_67 = self_66;
    other_59 = other_58;
    let _e4: Scalar = self_67;
    let _e7: Line = other_59;
    let _e10: Scalar = self_67;
    let _e13: Line = other_59;
    return Line((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_circle_geometric_product(self_68: Scalar, other_60: Circle) -> Circle {
    var self_69: Scalar;
    var other_61: Circle;

    self_69 = self_68;
    other_61 = other_60;
    let _e4: Scalar = self_69;
    let _e7: Circle = other_61;
    let _e10: Scalar = self_69;
    let _e13: Circle = other_61;
    let _e16: Scalar = self_69;
    let _e19: Circle = other_61;
    return Circle((vec4<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_), (vec3<f32>(_e16.g0_) * _e19.g2_));
}

fn scalar_circle_outer_product(self_70: Scalar, other_62: Circle) -> Circle {
    var self_71: Scalar;
    var other_63: Circle;

    self_71 = self_70;
    other_63 = other_62;
    let _e4: Scalar = self_71;
    let _e7: Circle = other_63;
    let _e10: Scalar = self_71;
    let _e13: Circle = other_63;
    let _e16: Scalar = self_71;
    let _e19: Circle = other_63;
    return Circle((vec4<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_), (vec3<f32>(_e16.g0_) * _e19.g2_));
}

fn scalar_circle_inner_product(self_72: Scalar, other_64: Circle) -> Circle {
    var self_73: Scalar;
    var other_65: Circle;

    self_73 = self_72;
    other_65 = other_64;
    let _e4: Scalar = self_73;
    let _e7: Circle = other_65;
    let _e10: Scalar = self_73;
    let _e13: Circle = other_65;
    let _e16: Scalar = self_73;
    let _e19: Circle = other_65;
    return Circle((vec4<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_), (vec3<f32>(_e16.g0_) * _e19.g2_));
}

fn scalar_circle_left_contraction(self_74: Scalar, other_66: Circle) -> Circle {
    var self_75: Scalar;
    var other_67: Circle;

    self_75 = self_74;
    other_67 = other_66;
    let _e4: Scalar = self_75;
    let _e7: Circle = other_67;
    let _e10: Scalar = self_75;
    let _e13: Circle = other_67;
    let _e16: Scalar = self_75;
    let _e19: Circle = other_67;
    return Circle((vec4<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_), (vec3<f32>(_e16.g0_) * _e19.g2_));
}

fn scalar_plane_geometric_product(self_76: Scalar, other_68: Plane) -> Plane {
    var self_77: Scalar;
    var other_69: Plane;

    self_77 = self_76;
    other_69 = other_68;
    let _e4: Scalar = self_77;
    let _e7: Plane = other_69;
    return Plane((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_plane_outer_product(self_78: Scalar, other_70: Plane) -> Plane {
    var self_79: Scalar;
    var other_71: Plane;

    self_79 = self_78;
    other_71 = other_70;
    let _e4: Scalar = self_79;
    let _e7: Plane = other_71;
    return Plane((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_plane_inner_product(self_80: Scalar, other_72: Plane) -> Plane {
    var self_81: Scalar;
    var other_73: Plane;

    self_81 = self_80;
    other_73 = other_72;
    let _e4: Scalar = self_81;
    let _e7: Plane = other_73;
    return Plane((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_plane_left_contraction(self_82: Scalar, other_74: Plane) -> Plane {
    var self_83: Scalar;
    var other_75: Plane;

    self_83 = self_82;
    other_75 = other_74;
    let _e4: Scalar = self_83;
    let _e7: Plane = other_75;
    return Plane((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_sphere_geometric_product(self_84: Scalar, other_76: Sphere) -> Sphere {
    var self_85: Scalar;
    var other_77: Sphere;

    self_85 = self_84;
    other_77 = other_76;
    let _e4: Scalar = self_85;
    let _e6: Sphere = other_77;
    let _e9: Scalar = self_85;
    let _e12: Sphere = other_77;
    return Sphere((_e4.g0_ * _e6.g0_), (vec4<f32>(_e9.g0_) * _e12.g1_));
}

fn scalar_sphere_outer_product(self_86: Scalar, other_78: Sphere) -> Sphere {
    var self_87: Scalar;
    var other_79: Sphere;

    self_87 = self_86;
    other_79 = other_78;
    let _e4: Scalar = self_87;
    let _e6: Sphere = other_79;
    let _e9: Scalar = self_87;
    let _e12: Sphere = other_79;
    return Sphere((_e4.g0_ * _e6.g0_), (vec4<f32>(_e9.g0_) * _e12.g1_));
}

fn scalar_sphere_inner_product(self_88: Scalar, other_80: Sphere) -> Sphere {
    var self_89: Scalar;
    var other_81: Sphere;

    self_89 = self_88;
    other_81 = other_80;
    let _e4: Scalar = self_89;
    let _e6: Sphere = other_81;
    let _e9: Scalar = self_89;
    let _e12: Sphere = other_81;
    return Sphere((_e4.g0_ * _e6.g0_), (vec4<f32>(_e9.g0_) * _e12.g1_));
}

fn scalar_sphere_left_contraction(self_90: Scalar, other_82: Sphere) -> Sphere {
    var self_91: Scalar;
    var other_83: Sphere;

    self_91 = self_90;
    other_83 = other_82;
    let _e4: Scalar = self_91;
    let _e6: Sphere = other_83;
    let _e9: Scalar = self_91;
    let _e12: Sphere = other_83;
    return Sphere((_e4.g0_ * _e6.g0_), (vec4<f32>(_e9.g0_) * _e12.g1_));
}

fn scalar_motor_geometric_product(self_92: Scalar, other_84: Motor) -> Motor {
    var self_93: Scalar;
    var other_85: Motor;

    self_93 = self_92;
    other_85 = other_84;
    let _e4: Scalar = self_93;
    let _e7: Motor = other_85;
    let _e10: Scalar = self_93;
    let _e13: Motor = other_85;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_motor_outer_product(self_94: Scalar, other_86: Motor) -> Motor {
    var self_95: Scalar;
    var other_87: Motor;

    self_95 = self_94;
    other_87 = other_86;
    let _e4: Scalar = self_95;
    let _e7: Motor = other_87;
    let _e10: Scalar = self_95;
    let _e13: Motor = other_87;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_motor_inner_product(self_96: Scalar, other_88: Motor) -> Motor {
    var self_97: Scalar;
    var other_89: Motor;

    self_97 = self_96;
    other_89 = other_88;
    let _e4: Scalar = self_97;
    let _e7: Motor = other_89;
    let _e10: Scalar = self_97;
    let _e13: Motor = other_89;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_motor_left_contraction(self_98: Scalar, other_90: Motor) -> Motor {
    var self_99: Scalar;
    var other_91: Motor;

    self_99 = self_98;
    other_91 = other_90;
    let _e4: Scalar = self_99;
    let _e7: Motor = other_91;
    let _e10: Scalar = self_99;
    let _e13: Motor = other_91;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_rotor_geometric_product(self_100: Scalar, other_92: Rotor) -> Rotor {
    var self_101: Scalar;
    var other_93: Rotor;

    self_101 = self_100;
    other_93 = other_92;
    let _e4: Scalar = self_101;
    let _e7: Rotor = other_93;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_rotor_outer_product(self_102: Scalar, other_94: Rotor) -> Rotor {
    var self_103: Scalar;
    var other_95: Rotor;

    self_103 = self_102;
    other_95 = other_94;
    let _e4: Scalar = self_103;
    let _e7: Rotor = other_95;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_rotor_inner_product(self_104: Scalar, other_96: Rotor) -> Rotor {
    var self_105: Scalar;
    var other_97: Rotor;

    self_105 = self_104;
    other_97 = other_96;
    let _e4: Scalar = self_105;
    let _e7: Rotor = other_97;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_rotor_left_contraction(self_106: Scalar, other_98: Rotor) -> Rotor {
    var self_107: Scalar;
    var other_99: Rotor;

    self_107 = self_106;
    other_99 = other_98;
    let _e4: Scalar = self_107;
    let _e7: Rotor = other_99;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_translator_geometric_product(self_108: Scalar, other_100: Translator) -> Translator {
    var self_109: Scalar;
    var other_101: Translator;

    self_109 = self_108;
    other_101 = other_100;
    let _e4: Scalar = self_109;
    let _e7: Translator = other_101;
    return Translator((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_translator_outer_product(self_110: Scalar, other_102: Translator) -> Translator {
    var self_111: Scalar;
    var other_103: Translator;

    self_111 = self_110;
    other_103 = other_102;
    let _e4: Scalar = self_111;
    let _e7: Translator = other_103;
    return Translator((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_translator_inner_product(self_112: Scalar, other_104: Translator) -> Translator {
    var self_113: Scalar;
    var other_105: Translator;

    self_113 = self_112;
    other_105 = other_104;
    let _e4: Scalar = self_113;
    let _e7: Translator = other_105;
    return Translator((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_translator_left_contraction(self_114: Scalar, other_106: Translator) -> Translator {
    var self_115: Scalar;
    var other_107: Translator;

    self_115 = self_114;
    other_107 = other_106;
    let _e4: Scalar = self_115;
    let _e7: Translator = other_107;
    return Translator((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_flector_geometric_product(self_116: Scalar, other_108: Flector) -> Flector {
    var self_117: Scalar;
    var other_109: Flector;

    self_117 = self_116;
    other_109 = other_108;
    let _e4: Scalar = self_117;
    let _e7: Flector = other_109;
    let _e10: Scalar = self_117;
    let _e13: Flector = other_109;
    return Flector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_flector_outer_product(self_118: Scalar, other_110: Flector) -> Flector {
    var self_119: Scalar;
    var other_111: Flector;

    self_119 = self_118;
    other_111 = other_110;
    let _e4: Scalar = self_119;
    let _e7: Flector = other_111;
    let _e10: Scalar = self_119;
    let _e13: Flector = other_111;
    return Flector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_flector_inner_product(self_120: Scalar, other_112: Flector) -> Flector {
    var self_121: Scalar;
    var other_113: Flector;

    self_121 = self_120;
    other_113 = other_112;
    let _e4: Scalar = self_121;
    let _e7: Flector = other_113;
    let _e10: Scalar = self_121;
    let _e13: Flector = other_113;
    return Flector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_flector_left_contraction(self_122: Scalar, other_114: Flector) -> Flector {
    var self_123: Scalar;
    var other_115: Flector;

    self_123 = self_122;
    other_115 = other_114;
    let _e4: Scalar = self_123;
    let _e7: Flector = other_115;
    let _e10: Scalar = self_123;
    let _e13: Flector = other_115;
    return Flector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_multi_vector_add(self_124: Scalar, other_116: MultiVector) -> MultiVector {
    var self_125: Scalar;
    var other_117: MultiVector;

    self_125 = self_124;
    other_117 = other_116;
    let _e4: Scalar = self_125;
    let _e12: MultiVector = other_117;
    let _e15: MultiVector = other_117;
    let _e17: MultiVector = other_117;
    let _e19: MultiVector = other_117;
    let _e21: MultiVector = other_117;
    let _e23: MultiVector = other_117;
    let _e25: MultiVector = other_117;
    let _e27: MultiVector = other_117;
    let _e29: MultiVector = other_117;
    let _e31: MultiVector = other_117;
    return MultiVector(((vec3<f32>(_e4.g0_) * vec3<f32>(1.0, 0.0, 0.0)) + _e12.g0_), _e15.g1_, _e17.g2_, _e19.g3_, _e21.g4_, _e23.g5_, _e25.g6_, _e27.g7_, _e29.g8_, _e31.g9_);
}

fn scalar_multi_vector_sub(self_126: Scalar, other_118: MultiVector) -> MultiVector {
    var self_127: Scalar;
    var other_119: MultiVector;

    self_127 = self_126;
    other_119 = other_118;
    let _e4: Scalar = self_127;
    let _e12: MultiVector = other_119;
    let _e17: MultiVector = other_119;
    let _e22: MultiVector = other_119;
    let _e27: MultiVector = other_119;
    let _e32: MultiVector = other_119;
    let _e37: MultiVector = other_119;
    let _e42: MultiVector = other_119;
    let _e47: MultiVector = other_119;
    let _e52: MultiVector = other_119;
    let _e57: MultiVector = other_119;
    return MultiVector(((vec3<f32>(_e4.g0_) * vec3<f32>(1.0, 0.0, 0.0)) - _e12.g0_), (vec3<f32>(0.0) - _e17.g1_), (vec2<f32>(0.0) - _e22.g2_), (vec4<f32>(0.0) - _e27.g3_), (vec3<f32>(0.0) - _e32.g4_), (vec3<f32>(0.0) - _e37.g5_), (vec3<f32>(0.0) - _e42.g6_), (vec3<f32>(0.0) - _e47.g7_), (vec4<f32>(0.0) - _e52.g8_), (vec4<f32>(0.0) - _e57.g9_));
}

fn scalar_multi_vector_geometric_product(self_128: Scalar, other_120: MultiVector) -> MultiVector {
    var self_129: Scalar;
    var other_121: MultiVector;

    self_129 = self_128;
    other_121 = other_120;
    let _e4: Scalar = self_129;
    let _e7: MultiVector = other_121;
    let _e10: Scalar = self_129;
    let _e13: MultiVector = other_121;
    let _e16: Scalar = self_129;
    let _e19: MultiVector = other_121;
    let _e22: Scalar = self_129;
    let _e25: MultiVector = other_121;
    let _e28: Scalar = self_129;
    let _e31: MultiVector = other_121;
    let _e34: Scalar = self_129;
    let _e37: MultiVector = other_121;
    let _e40: Scalar = self_129;
    let _e43: MultiVector = other_121;
    let _e46: Scalar = self_129;
    let _e49: MultiVector = other_121;
    let _e52: Scalar = self_129;
    let _e55: MultiVector = other_121;
    let _e58: Scalar = self_129;
    let _e61: MultiVector = other_121;
    return MultiVector((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_), (vec2<f32>(_e16.g0_) * _e19.g2_), (vec4<f32>(_e22.g0_) * _e25.g3_), (vec3<f32>(_e28.g0_) * _e31.g4_), (vec3<f32>(_e34.g0_) * _e37.g5_), (vec3<f32>(_e40.g0_) * _e43.g6_), (vec3<f32>(_e46.g0_) * _e49.g7_), (vec4<f32>(_e52.g0_) * _e55.g8_), (vec4<f32>(_e58.g0_) * _e61.g9_));
}

fn scalar_multi_vector_outer_product(self_130: Scalar, other_122: MultiVector) -> MultiVector {
    var self_131: Scalar;
    var other_123: MultiVector;

    self_131 = self_130;
    other_123 = other_122;
    let _e4: Scalar = self_131;
    let _e7: MultiVector = other_123;
    let _e10: Scalar = self_131;
    let _e13: MultiVector = other_123;
    let _e16: Scalar = self_131;
    let _e19: MultiVector = other_123;
    let _e22: Scalar = self_131;
    let _e25: MultiVector = other_123;
    let _e28: Scalar = self_131;
    let _e31: MultiVector = other_123;
    let _e34: Scalar = self_131;
    let _e37: MultiVector = other_123;
    let _e40: Scalar = self_131;
    let _e43: MultiVector = other_123;
    let _e46: Scalar = self_131;
    let _e49: MultiVector = other_123;
    let _e52: Scalar = self_131;
    let _e55: MultiVector = other_123;
    let _e58: Scalar = self_131;
    let _e61: MultiVector = other_123;
    return MultiVector((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_), (vec2<f32>(_e16.g0_) * _e19.g2_), (vec4<f32>(_e22.g0_) * _e25.g3_), (vec3<f32>(_e28.g0_) * _e31.g4_), (vec3<f32>(_e34.g0_) * _e37.g5_), (vec3<f32>(_e40.g0_) * _e43.g6_), (vec3<f32>(_e46.g0_) * _e49.g7_), (vec4<f32>(_e52.g0_) * _e55.g8_), (vec4<f32>(_e58.g0_) * _e61.g9_));
}

fn scalar_multi_vector_inner_product(self_132: Scalar, other_124: MultiVector) -> MultiVector {
    var self_133: Scalar;
    var other_125: MultiVector;

    self_133 = self_132;
    other_125 = other_124;
    let _e4: Scalar = self_133;
    let _e7: MultiVector = other_125;
    let _e10: Scalar = self_133;
    let _e13: MultiVector = other_125;
    let _e16: Scalar = self_133;
    let _e19: MultiVector = other_125;
    let _e22: Scalar = self_133;
    let _e25: MultiVector = other_125;
    let _e28: Scalar = self_133;
    let _e31: MultiVector = other_125;
    let _e34: Scalar = self_133;
    let _e37: MultiVector = other_125;
    let _e40: Scalar = self_133;
    let _e43: MultiVector = other_125;
    let _e46: Scalar = self_133;
    let _e49: MultiVector = other_125;
    let _e52: Scalar = self_133;
    let _e55: MultiVector = other_125;
    let _e58: Scalar = self_133;
    let _e61: MultiVector = other_125;
    return MultiVector((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_), (vec2<f32>(_e16.g0_) * _e19.g2_), (vec4<f32>(_e22.g0_) * _e25.g3_), (vec3<f32>(_e28.g0_) * _e31.g4_), (vec3<f32>(_e34.g0_) * _e37.g5_), (vec3<f32>(_e40.g0_) * _e43.g6_), (vec3<f32>(_e46.g0_) * _e49.g7_), (vec4<f32>(_e52.g0_) * _e55.g8_), (vec4<f32>(_e58.g0_) * _e61.g9_));
}

fn scalar_multi_vector_left_contraction(self_134: Scalar, other_126: MultiVector) -> MultiVector {
    var self_135: Scalar;
    var other_127: MultiVector;

    self_135 = self_134;
    other_127 = other_126;
    let _e4: Scalar = self_135;
    let _e7: MultiVector = other_127;
    let _e10: Scalar = self_135;
    let _e13: MultiVector = other_127;
    let _e16: Scalar = self_135;
    let _e19: MultiVector = other_127;
    let _e22: Scalar = self_135;
    let _e25: MultiVector = other_127;
    let _e28: Scalar = self_135;
    let _e31: MultiVector = other_127;
    let _e34: Scalar = self_135;
    let _e37: MultiVector = other_127;
    let _e40: Scalar = self_135;
    let _e43: MultiVector = other_127;
    let _e46: Scalar = self_135;
    let _e49: MultiVector = other_127;
    let _e52: Scalar = self_135;
    let _e55: MultiVector = other_127;
    let _e58: Scalar = self_135;
    let _e61: MultiVector = other_127;
    return MultiVector((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_), (vec2<f32>(_e16.g0_) * _e19.g2_), (vec4<f32>(_e22.g0_) * _e25.g3_), (vec3<f32>(_e28.g0_) * _e31.g4_), (vec3<f32>(_e34.g0_) * _e37.g5_), (vec3<f32>(_e40.g0_) * _e43.g6_), (vec3<f32>(_e46.g0_) * _e49.g7_), (vec4<f32>(_e52.g0_) * _e55.g8_), (vec4<f32>(_e58.g0_) * _e61.g9_));
}

fn scalar_multi_vector_right_contraction(self_136: Scalar, other_128: MultiVector) -> Scalar {
    var self_137: Scalar;
    var other_129: MultiVector;

    self_137 = self_136;
    other_129 = other_128;
    let _e4: Scalar = self_137;
    let _e6: MultiVector = other_129;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_multi_vector_scalar_product(self_138: Scalar, other_130: MultiVector) -> Scalar {
    var self_139: Scalar;
    var other_131: MultiVector;

    self_139 = self_138;
    other_131 = other_130;
    let _e4: Scalar = self_139;
    let _e6: MultiVector = other_131;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_squared_magnitude(self_140: Scalar) -> Scalar {
    var self_141: Scalar;

    self_141 = self_140;
    let _e4: Scalar = self_141;
    let _e5: Scalar = scalar_reversal(_e4);
    let _e6: Scalar = self_141;
    let _e8: Scalar = self_141;
    let _e9: Scalar = scalar_reversal(_e8);
    let _e10: Scalar = scalar_scalar_scalar_product(_e6, _e9);
    return _e10;
}

fn scalar_magnitude(self_142: Scalar) -> Scalar {
    var self_143: Scalar;

    self_143 = self_142;
    let _e3: Scalar = self_143;
    let _e4: Scalar = scalar_squared_magnitude(_e3);
    let _e7: Scalar = self_143;
    let _e8: Scalar = scalar_squared_magnitude(_e7);
    return Scalar(sqrt(_e8.g0_));
}

fn scalar_scale(self_144: Scalar, other_132: f32) -> Scalar {
    var self_145: Scalar;
    var other_133: f32;

    self_145 = self_144;
    other_133 = other_132;
    let _e5: f32 = other_133;
    let _e7: Scalar = self_145;
    let _e8: f32 = other_133;
    let _e10: Scalar = scalar_scalar_geometric_product(_e7, Scalar(_e8));
    return _e10;
}

fn scalar_signum(self_146: Scalar) -> Scalar {
    var self_147: Scalar;

    self_147 = self_146;
    let _e5: Scalar = self_147;
    let _e6: Scalar = scalar_magnitude(_e5);
    let _e10: Scalar = self_147;
    let _e13: Scalar = self_147;
    let _e14: Scalar = scalar_magnitude(_e13);
    let _e18: Scalar = scalar_scalar_geometric_product(_e10, Scalar((1.0 / _e14.g0_)));
    return _e18;
}

fn scalar_inverse(self_148: Scalar) -> Scalar {
    var self_149: Scalar;

    self_149 = self_148;
    let _e3: Scalar = self_149;
    let _e4: Scalar = scalar_reversal(_e3);
    let _e7: Scalar = self_149;
    let _e8: Scalar = scalar_squared_magnitude(_e7);
    let _e13: Scalar = self_149;
    let _e14: Scalar = scalar_reversal(_e13);
    let _e17: Scalar = self_149;
    let _e18: Scalar = scalar_squared_magnitude(_e17);
    let _e22: Scalar = scalar_scalar_geometric_product(_e14, Scalar((1.0 / _e18.g0_)));
    return _e22;
}

fn anti_scalar_zero() -> AntiScalar {
    return AntiScalar(0.0);
}

fn anti_scalar_one() -> AntiScalar {
    return AntiScalar(0.0);
}

fn anti_scalar_neg(self_150: AntiScalar) -> AntiScalar {
    var self_151: AntiScalar;

    self_151 = self_150;
    let _e2: AntiScalar = self_151;
    return AntiScalar((_e2.g0_ * -(1.0)));
}

fn anti_scalar_automorphism(self_152: AntiScalar) -> AntiScalar {
    var self_153: AntiScalar;

    self_153 = self_152;
    let _e2: AntiScalar = self_153;
    return AntiScalar((_e2.g0_ * -(1.0)));
}

fn anti_scalar_reversal(self_154: AntiScalar) -> AntiScalar {
    var self_155: AntiScalar;

    self_155 = self_154;
    let _e2: AntiScalar = self_155;
    return AntiScalar(_e2.g0_);
}

fn anti_scalar_conjugation(self_156: AntiScalar) -> AntiScalar {
    var self_157: AntiScalar;

    self_157 = self_156;
    let _e2: AntiScalar = self_157;
    return AntiScalar((_e2.g0_ * -(1.0)));
}

fn anti_scalar_scalar_geometric_product(self_158: AntiScalar, other_134: Scalar) -> AntiScalar {
    var self_159: AntiScalar;
    var other_135: Scalar;

    self_159 = self_158;
    other_135 = other_134;
    let _e4: AntiScalar = self_159;
    let _e6: Scalar = other_135;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_outer_product(self_160: AntiScalar, other_136: Scalar) -> AntiScalar {
    var self_161: AntiScalar;
    var other_137: Scalar;

    self_161 = self_160;
    other_137 = other_136;
    let _e4: AntiScalar = self_161;
    let _e6: Scalar = other_137;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_inner_product(self_162: AntiScalar, other_138: Scalar) -> AntiScalar {
    var self_163: AntiScalar;
    var other_139: Scalar;

    self_163 = self_162;
    other_139 = other_138;
    let _e4: AntiScalar = self_163;
    let _e6: Scalar = other_139;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_right_contraction(self_164: AntiScalar, other_140: Scalar) -> AntiScalar {
    var self_165: AntiScalar;
    var other_141: Scalar;

    self_165 = self_164;
    other_141 = other_140;
    let _e4: AntiScalar = self_165;
    let _e6: Scalar = other_141;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_add(self_166: AntiScalar, other_142: AntiScalar) -> AntiScalar {
    var self_167: AntiScalar;
    var other_143: AntiScalar;

    self_167 = self_166;
    other_143 = other_142;
    let _e4: AntiScalar = self_167;
    let _e6: AntiScalar = other_143;
    return AntiScalar((_e4.g0_ + _e6.g0_));
}

fn anti_scalar_anti_scalar_sub(self_168: AntiScalar, other_144: AntiScalar) -> AntiScalar {
    var self_169: AntiScalar;
    var other_145: AntiScalar;

    self_169 = self_168;
    other_145 = other_144;
    let _e4: AntiScalar = self_169;
    let _e6: AntiScalar = other_145;
    return AntiScalar((_e4.g0_ - _e6.g0_));
}

fn anti_scalar_anti_scalar_mul(self_170: AntiScalar, other_146: AntiScalar) -> AntiScalar {
    var self_171: AntiScalar;
    var other_147: AntiScalar;

    self_171 = self_170;
    other_147 = other_146;
    let _e4: AntiScalar = self_171;
    let _e6: AntiScalar = other_147;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_div(self_172: AntiScalar, other_148: AntiScalar) -> AntiScalar {
    var self_173: AntiScalar;
    var other_149: AntiScalar;

    self_173 = self_172;
    other_149 = other_148;
    let _e4: AntiScalar = self_173;
    let _e8: AntiScalar = other_149;
    return AntiScalar((((_e4.g0_ * 1.0) / _e8.g0_) * 1.0));
}

fn anti_scalar_motor_add(self_174: AntiScalar, other_150: Motor) -> Motor {
    var self_175: AntiScalar;
    var other_151: Motor;

    self_175 = self_174;
    other_151 = other_150;
    let _e4: AntiScalar = self_175;
    let _e13: Motor = other_151;
    let _e16: Motor = other_151;
    return Motor(((vec4<f32>(_e4.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + _e13.g0_), _e16.g1_);
}

fn anti_scalar_motor_sub(self_176: AntiScalar, other_152: Motor) -> Motor {
    var self_177: AntiScalar;
    var other_153: Motor;

    self_177 = self_176;
    other_153 = other_152;
    let _e4: AntiScalar = self_177;
    let _e13: Motor = other_153;
    let _e18: Motor = other_153;
    return Motor(((vec4<f32>(_e4.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) - _e13.g0_), (vec4<f32>(0.0) - _e18.g1_));
}

fn anti_scalar_rotor_add(self_178: AntiScalar, other_154: Rotor) -> Rotor {
    var self_179: AntiScalar;
    var other_155: Rotor;

    self_179 = self_178;
    other_155 = other_154;
    let _e4: AntiScalar = self_179;
    let _e13: Rotor = other_155;
    return Rotor(((vec4<f32>(_e4.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + _e13.g0_));
}

fn anti_scalar_rotor_sub(self_180: AntiScalar, other_156: Rotor) -> Rotor {
    var self_181: AntiScalar;
    var other_157: Rotor;

    self_181 = self_180;
    other_157 = other_156;
    let _e4: AntiScalar = self_181;
    let _e13: Rotor = other_157;
    return Rotor(((vec4<f32>(_e4.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) - _e13.g0_));
}

fn anti_scalar_translator_add(self_182: AntiScalar, other_158: Translator) -> Translator {
    var self_183: AntiScalar;
    var other_159: Translator;

    self_183 = self_182;
    other_159 = other_158;
    let _e4: AntiScalar = self_183;
    let _e13: Translator = other_159;
    return Translator(((vec4<f32>(_e4.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + _e13.g0_));
}

fn anti_scalar_translator_sub(self_184: AntiScalar, other_160: Translator) -> Translator {
    var self_185: AntiScalar;
    var other_161: Translator;

    self_185 = self_184;
    other_161 = other_160;
    let _e4: AntiScalar = self_185;
    let _e13: Translator = other_161;
    return Translator(((vec4<f32>(_e4.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) - _e13.g0_));
}

fn anti_scalar_multi_vector_add(self_186: AntiScalar, other_162: MultiVector) -> MultiVector {
    var self_187: AntiScalar;
    var other_163: MultiVector;

    self_187 = self_186;
    other_163 = other_162;
    let _e4: AntiScalar = self_187;
    let _e12: MultiVector = other_163;
    let _e15: MultiVector = other_163;
    let _e17: MultiVector = other_163;
    let _e19: MultiVector = other_163;
    let _e21: MultiVector = other_163;
    let _e23: MultiVector = other_163;
    let _e25: MultiVector = other_163;
    let _e27: MultiVector = other_163;
    let _e29: MultiVector = other_163;
    let _e31: MultiVector = other_163;
    return MultiVector(((vec3<f32>(_e4.g0_) * vec3<f32>(0.0, 0.0, 1.0)) + _e12.g0_), _e15.g1_, _e17.g2_, _e19.g3_, _e21.g4_, _e23.g5_, _e25.g6_, _e27.g7_, _e29.g8_, _e31.g9_);
}

fn anti_scalar_multi_vector_sub(self_188: AntiScalar, other_164: MultiVector) -> MultiVector {
    var self_189: AntiScalar;
    var other_165: MultiVector;

    self_189 = self_188;
    other_165 = other_164;
    let _e4: AntiScalar = self_189;
    let _e12: MultiVector = other_165;
    let _e17: MultiVector = other_165;
    let _e22: MultiVector = other_165;
    let _e27: MultiVector = other_165;
    let _e32: MultiVector = other_165;
    let _e37: MultiVector = other_165;
    let _e42: MultiVector = other_165;
    let _e47: MultiVector = other_165;
    let _e52: MultiVector = other_165;
    let _e57: MultiVector = other_165;
    return MultiVector(((vec3<f32>(_e4.g0_) * vec3<f32>(0.0, 0.0, 1.0)) - _e12.g0_), (vec3<f32>(0.0) - _e17.g1_), (vec2<f32>(0.0) - _e22.g2_), (vec4<f32>(0.0) - _e27.g3_), (vec3<f32>(0.0) - _e32.g4_), (vec3<f32>(0.0) - _e37.g5_), (vec3<f32>(0.0) - _e42.g6_), (vec3<f32>(0.0) - _e47.g7_), (vec4<f32>(0.0) - _e52.g8_), (vec4<f32>(0.0) - _e57.g9_));
}

fn anti_scalar_multi_vector_outer_product(self_190: AntiScalar, other_166: MultiVector) -> AntiScalar {
    var self_191: AntiScalar;
    var other_167: MultiVector;

    self_191 = self_190;
    other_167 = other_166;
    let _e4: AntiScalar = self_191;
    let _e6: MultiVector = other_167;
    return AntiScalar((_e4.g0_ * _e6.g0_.x));
}

fn anti_scalar_scale(self_192: AntiScalar, other_168: f32) -> AntiScalar {
    var self_193: AntiScalar;
    var other_169: f32;

    self_193 = self_192;
    other_169 = other_168;
    let _e5: f32 = other_169;
    let _e7: AntiScalar = self_193;
    let _e8: f32 = other_169;
    let _e10: AntiScalar = anti_scalar_scalar_geometric_product(_e7, Scalar(_e8));
    return _e10;
}

fn radial_point_zero() -> RadialPoint {
    return RadialPoint(vec3<f32>(0.0), vec2<f32>(0.0));
}

fn radial_point_one() -> RadialPoint {
    return RadialPoint(vec3<f32>(0.0), vec2<f32>(0.0));
}

fn radial_point_neg(self_194: RadialPoint) -> RadialPoint {
    var self_195: RadialPoint;

    self_195 = self_194;
    let _e2: RadialPoint = self_195;
    let _e8: RadialPoint = self_195;
    return RadialPoint((_e2.g0_ * vec3<f32>(-(1.0))), (_e8.g1_ * vec2<f32>(-(1.0))));
}

fn radial_point_automorphism(self_196: RadialPoint) -> RadialPoint {
    var self_197: RadialPoint;

    self_197 = self_196;
    let _e2: RadialPoint = self_197;
    let _e8: RadialPoint = self_197;
    return RadialPoint((_e2.g0_ * vec3<f32>(-(1.0))), (_e8.g1_ * vec2<f32>(-(1.0))));
}

fn radial_point_reversal(self_198: RadialPoint) -> RadialPoint {
    var self_199: RadialPoint;

    self_199 = self_198;
    let _e2: RadialPoint = self_199;
    let _e4: RadialPoint = self_199;
    return RadialPoint(_e2.g0_, _e4.g1_);
}

fn radial_point_conjugation(self_200: RadialPoint) -> RadialPoint {
    var self_201: RadialPoint;

    self_201 = self_200;
    let _e2: RadialPoint = self_201;
    let _e8: RadialPoint = self_201;
    return RadialPoint((_e2.g0_ * vec3<f32>(-(1.0))), (_e8.g1_ * vec2<f32>(-(1.0))));
}

fn radial_point_scalar_geometric_product(self_202: RadialPoint, other_170: Scalar) -> RadialPoint {
    var self_203: RadialPoint;
    var other_171: Scalar;

    self_203 = self_202;
    other_171 = other_170;
    let _e4: RadialPoint = self_203;
    let _e6: Scalar = other_171;
    let _e10: RadialPoint = self_203;
    let _e12: Scalar = other_171;
    return RadialPoint((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec2<f32>(_e12.g0_)));
}

fn radial_point_scalar_outer_product(self_204: RadialPoint, other_172: Scalar) -> RadialPoint {
    var self_205: RadialPoint;
    var other_173: Scalar;

    self_205 = self_204;
    other_173 = other_172;
    let _e4: RadialPoint = self_205;
    let _e6: Scalar = other_173;
    let _e10: RadialPoint = self_205;
    let _e12: Scalar = other_173;
    return RadialPoint((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec2<f32>(_e12.g0_)));
}

fn radial_point_scalar_inner_product(self_206: RadialPoint, other_174: Scalar) -> RadialPoint {
    var self_207: RadialPoint;
    var other_175: Scalar;

    self_207 = self_206;
    other_175 = other_174;
    let _e4: RadialPoint = self_207;
    let _e6: Scalar = other_175;
    let _e10: RadialPoint = self_207;
    let _e12: Scalar = other_175;
    return RadialPoint((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec2<f32>(_e12.g0_)));
}

fn radial_point_scalar_right_contraction(self_208: RadialPoint, other_176: Scalar) -> RadialPoint {
    var self_209: RadialPoint;
    var other_177: Scalar;

    self_209 = self_208;
    other_177 = other_176;
    let _e4: RadialPoint = self_209;
    let _e6: Scalar = other_177;
    let _e10: RadialPoint = self_209;
    let _e12: Scalar = other_177;
    return RadialPoint((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec2<f32>(_e12.g0_)));
}

fn radial_point_radial_point_add(self_210: RadialPoint, other_178: RadialPoint) -> RadialPoint {
    var self_211: RadialPoint;
    var other_179: RadialPoint;

    self_211 = self_210;
    other_179 = other_178;
    let _e4: RadialPoint = self_211;
    let _e6: RadialPoint = other_179;
    let _e9: RadialPoint = self_211;
    let _e11: RadialPoint = other_179;
    return RadialPoint((_e4.g0_ + _e6.g0_), (_e9.g1_ + _e11.g1_));
}

fn radial_point_radial_point_sub(self_212: RadialPoint, other_180: RadialPoint) -> RadialPoint {
    var self_213: RadialPoint;
    var other_181: RadialPoint;

    self_213 = self_212;
    other_181 = other_180;
    let _e4: RadialPoint = self_213;
    let _e6: RadialPoint = other_181;
    let _e9: RadialPoint = self_213;
    let _e11: RadialPoint = other_181;
    return RadialPoint((_e4.g0_ - _e6.g0_), (_e9.g1_ - _e11.g1_));
}

fn radial_point_radial_point_mul(self_214: RadialPoint, other_182: RadialPoint) -> RadialPoint {
    var self_215: RadialPoint;
    var other_183: RadialPoint;

    self_215 = self_214;
    other_183 = other_182;
    let _e4: RadialPoint = self_215;
    let _e6: RadialPoint = other_183;
    let _e9: RadialPoint = self_215;
    let _e11: RadialPoint = other_183;
    return RadialPoint((_e4.g0_ * _e6.g0_), (_e9.g1_ * _e11.g1_));
}

fn radial_point_radial_point_div(self_216: RadialPoint, other_184: RadialPoint) -> RadialPoint {
    var self_217: RadialPoint;
    var other_185: RadialPoint;

    self_217 = self_216;
    other_185 = other_184;
    let _e4: RadialPoint = self_217;
    let _e7: RadialPoint = self_217;
    let _e10: RadialPoint = self_217;
    let _e19: RadialPoint = other_185;
    let _e22: RadialPoint = other_185;
    let _e25: RadialPoint = other_185;
    let _e35: RadialPoint = self_217;
    let _e38: RadialPoint = self_217;
    let _e46: RadialPoint = other_185;
    let _e49: RadialPoint = other_185;
    return RadialPoint((((vec3<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e19.g0_.x, _e22.g0_.y, _e25.g0_.z)) * vec3<f32>(1.0, 1.0, 1.0)), (((vec2<f32>(_e35.g1_.x, _e38.g1_.y) * vec2<f32>(1.0, 1.0)) / vec2<f32>(_e46.g1_.x, _e49.g1_.y)) * vec2<f32>(1.0, 1.0)));
}

fn radial_point_radial_point_outer_product(self_218: RadialPoint, other_186: RadialPoint) -> Dipole {
    var self_219: RadialPoint;
    var other_187: RadialPoint;

    self_219 = self_218;
    other_187 = other_186;
    let _e4: RadialPoint = self_219;
    let _e8: RadialPoint = other_187;
    let _e11: RadialPoint = self_219;
    let _e13: RadialPoint = other_187;
    let _e23: RadialPoint = self_219;
    let _e27: RadialPoint = other_187;
    let _e37: RadialPoint = self_219;
    let _e41: RadialPoint = other_187;
    let _e52: RadialPoint = self_219;
    let _e56: RadialPoint = other_187;
    let _e67: RadialPoint = self_219;
    let _e71: RadialPoint = other_187;
    let _e82: RadialPoint = self_219;
    let _e86: RadialPoint = other_187;
    let _e89: RadialPoint = other_187;
    let _e92: RadialPoint = other_187;
    let _e95: RadialPoint = other_187;
    let _e101: RadialPoint = self_219;
    let _e104: RadialPoint = self_219;
    let _e107: RadialPoint = self_219;
    let _e110: RadialPoint = self_219;
    let _e114: RadialPoint = other_187;
    let _e117: RadialPoint = other_187;
    let _e120: RadialPoint = other_187;
    let _e123: RadialPoint = other_187;
    return Dipole(((vec3<f32>(_e4.g1_.x) * _e8.g0_) + ((_e11.g0_ * vec3<f32>(_e13.g1_.x)) * vec3<f32>(-(1.0)))), ((((vec3<f32>(_e23.g0_.y) * _e27.g0_.zzx) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e37.g0_.z) * _e41.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e52.g0_.x) * _e56.g0_.xzy) * vec3<f32>(0.0, -(1.0), 1.0))), ((((vec4<f32>(_e67.g1_.x) * vec4<f32>(_e71.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) - (vec4<f32>(_e82.g1_.y) * vec4<f32>(_e86.g0_.x, _e89.g0_.y, _e92.g0_.z, _e95.g1_.x))) + ((vec4<f32>(_e101.g0_.x, _e104.g0_.y, _e107.g0_.z, _e110.g0_.x) * vec4<f32>(_e114.g1_.y, _e117.g1_.y, _e120.g1_.y, _e123.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn radial_point_radial_point_inner_product(self_220: RadialPoint, other_188: RadialPoint) -> Scalar {
    var self_221: RadialPoint;
    var other_189: RadialPoint;

    self_221 = self_220;
    other_189 = other_188;
    let _e4: RadialPoint = self_221;
    let _e7: RadialPoint = other_189;
    let _e11: RadialPoint = self_221;
    let _e14: RadialPoint = other_189;
    let _e19: RadialPoint = self_221;
    let _e22: RadialPoint = other_189;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn radial_point_radial_point_left_contraction(self_222: RadialPoint, other_190: RadialPoint) -> Scalar {
    var self_223: RadialPoint;
    var other_191: RadialPoint;

    self_223 = self_222;
    other_191 = other_190;
    let _e4: RadialPoint = self_223;
    let _e7: RadialPoint = other_191;
    let _e11: RadialPoint = self_223;
    let _e14: RadialPoint = other_191;
    let _e19: RadialPoint = self_223;
    let _e22: RadialPoint = other_191;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn radial_point_radial_point_right_contraction(self_224: RadialPoint, other_192: RadialPoint) -> Scalar {
    var self_225: RadialPoint;
    var other_193: RadialPoint;

    self_225 = self_224;
    other_193 = other_192;
    let _e4: RadialPoint = self_225;
    let _e7: RadialPoint = other_193;
    let _e11: RadialPoint = self_225;
    let _e14: RadialPoint = other_193;
    let _e19: RadialPoint = self_225;
    let _e22: RadialPoint = other_193;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn radial_point_radial_point_scalar_product(self_226: RadialPoint, other_194: RadialPoint) -> Scalar {
    var self_227: RadialPoint;
    var other_195: RadialPoint;

    self_227 = self_226;
    other_195 = other_194;
    let _e4: RadialPoint = self_227;
    let _e7: RadialPoint = other_195;
    let _e11: RadialPoint = self_227;
    let _e14: RadialPoint = other_195;
    let _e19: RadialPoint = self_227;
    let _e22: RadialPoint = other_195;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn radial_point_flat_point_outer_product(self_228: RadialPoint, other_196: FlatPoint) -> Line {
    var self_229: RadialPoint;
    var other_197: FlatPoint;

    self_229 = self_228;
    other_197 = other_196;
    let _e4: RadialPoint = self_229;
    let _e8: FlatPoint = other_197;
    let _e11: FlatPoint = other_197;
    let _e14: FlatPoint = other_197;
    let _e19: RadialPoint = self_229;
    let _e21: FlatPoint = other_197;
    let _e31: RadialPoint = self_229;
    let _e35: FlatPoint = other_197;
    let _e38: FlatPoint = other_197;
    let _e41: FlatPoint = other_197;
    let _e52: RadialPoint = self_229;
    let _e56: FlatPoint = other_197;
    let _e59: FlatPoint = other_197;
    let _e62: FlatPoint = other_197;
    let _e74: RadialPoint = self_229;
    let _e78: FlatPoint = other_197;
    let _e81: FlatPoint = other_197;
    let _e84: FlatPoint = other_197;
    return Line(((vec3<f32>(_e4.g1_.x) * vec3<f32>(_e8.g0_.x, _e11.g0_.y, _e14.g0_.z)) + ((_e19.g0_ * vec3<f32>(_e21.g0_.w)) * vec3<f32>(-(1.0)))), ((((vec3<f32>(_e31.g0_.y) * vec3<f32>(_e35.g0_.z, _e38.g0_.z, _e41.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e52.g0_.z) * vec3<f32>(_e56.g0_.y, _e59.g0_.x, _e62.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e74.g0_.x) * vec3<f32>(_e78.g0_.x, _e81.g0_.z, _e84.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn radial_point_dipole_outer_product(self_230: RadialPoint, other_198: Dipole) -> Circle {
    var self_231: RadialPoint;
    var other_199: Dipole;

    self_231 = self_230;
    other_199 = other_198;
    let _e4: RadialPoint = self_231;
    let _e8: Dipole = other_199;
    let _e11: Dipole = other_199;
    let _e14: Dipole = other_199;
    let _e17: Dipole = other_199;
    let _e30: RadialPoint = self_231;
    let _e34: Dipole = other_199;
    let _e37: Dipole = other_199;
    let _e40: Dipole = other_199;
    let _e43: Dipole = other_199;
    let _e57: RadialPoint = self_231;
    let _e61: Dipole = other_199;
    let _e64: Dipole = other_199;
    let _e67: Dipole = other_199;
    let _e70: Dipole = other_199;
    let _e82: RadialPoint = self_231;
    let _e86: Dipole = other_199;
    let _e89: Dipole = other_199;
    let _e92: Dipole = other_199;
    let _e95: Dipole = other_199;
    let _e109: RadialPoint = self_231;
    let _e113: Dipole = other_199;
    let _e116: Dipole = other_199;
    let _e119: Dipole = other_199;
    let _e124: RadialPoint = self_231;
    let _e128: Dipole = other_199;
    let _e132: RadialPoint = self_231;
    let _e134: Dipole = other_199;
    let _e144: RadialPoint = self_231;
    let _e148: Dipole = other_199;
    let _e151: Dipole = other_199;
    let _e154: Dipole = other_199;
    let _e165: RadialPoint = self_231;
    let _e169: Dipole = other_199;
    let _e172: Dipole = other_199;
    let _e175: Dipole = other_199;
    let _e187: RadialPoint = self_231;
    let _e191: Dipole = other_199;
    let _e195: RadialPoint = self_231;
    let _e199: Dipole = other_199;
    let _e202: Dipole = other_199;
    let _e205: Dipole = other_199;
    return Circle((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.y, _e37.g0_.x, _e40.g0_.y, _e43.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g1_.x) * vec4<f32>(_e61.g1_.x, _e64.g1_.y, _e67.g1_.z, _e70.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g0_.x) * vec4<f32>(_e86.g0_.x, _e89.g0_.z, _e92.g0_.y, _e95.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), (((vec3<f32>(_e109.g1_.x) * vec3<f32>(_e113.g2_.x, _e116.g2_.y, _e119.g2_.z)) + (vec3<f32>(_e124.g1_.y) * _e128.g0_)) + ((_e132.g0_ * vec3<f32>(_e134.g2_.w)) * vec3<f32>(-(1.0)))), (((((vec3<f32>(_e144.g0_.y) * vec3<f32>(_e148.g2_.z, _e151.g2_.z, _e154.g2_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e165.g0_.z) * vec3<f32>(_e169.g2_.y, _e172.g2_.x, _e175.g2_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + (vec3<f32>(_e187.g1_.y) * _e191.g1_)) + ((vec3<f32>(_e195.g0_.x) * vec3<f32>(_e199.g2_.x, _e202.g2_.z, _e205.g2_.y)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn radial_point_dipole_inner_product(self_232: RadialPoint, other_200: Dipole) -> RadialPoint {
    var self_233: RadialPoint;
    var other_201: Dipole;

    self_233 = self_232;
    other_201 = other_200;
    let _e4: RadialPoint = self_233;
    let _e8: Dipole = other_201;
    let _e18: RadialPoint = self_233;
    let _e22: Dipole = other_201;
    let _e33: RadialPoint = self_233;
    let _e37: Dipole = other_201;
    let _e48: RadialPoint = self_233;
    let _e52: Dipole = other_201;
    let _e55: Dipole = other_201;
    let _e65: RadialPoint = self_233;
    let _e69: Dipole = other_201;
    let _e72: Dipole = other_201;
    let _e83: RadialPoint = self_233;
    let _e87: Dipole = other_201;
    let _e90: Dipole = other_201;
    return RadialPoint(((((vec3<f32>(_e4.g0_.y) * _e8.g1_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e18.g0_.z) * _e22.g1_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e33.g0_.x) * _e37.g1_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))), ((((vec2<f32>(_e48.g0_.x) * vec2<f32>(_e52.g0_.x, _e55.g2_.x)) * vec2<f32>(-(1.0), 1.0)) + ((vec2<f32>(_e65.g0_.y) * vec2<f32>(_e69.g0_.y, _e72.g2_.y)) * vec2<f32>(-(1.0), 1.0))) + ((vec2<f32>(_e83.g0_.z) * vec2<f32>(_e87.g0_.z, _e90.g2_.z)) * vec2<f32>(-(1.0), 1.0))));
}

fn radial_point_dipole_left_contraction(self_234: RadialPoint, other_202: Dipole) -> RadialPoint {
    var self_235: RadialPoint;
    var other_203: Dipole;

    self_235 = self_234;
    other_203 = other_202;
    let _e4: RadialPoint = self_235;
    let _e8: Dipole = other_203;
    let _e18: RadialPoint = self_235;
    let _e22: Dipole = other_203;
    let _e33: RadialPoint = self_235;
    let _e37: Dipole = other_203;
    let _e48: RadialPoint = self_235;
    let _e52: Dipole = other_203;
    let _e55: Dipole = other_203;
    let _e65: RadialPoint = self_235;
    let _e69: Dipole = other_203;
    let _e72: Dipole = other_203;
    let _e83: RadialPoint = self_235;
    let _e87: Dipole = other_203;
    let _e90: Dipole = other_203;
    return RadialPoint(((((vec3<f32>(_e4.g0_.y) * _e8.g1_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e18.g0_.z) * _e22.g1_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e33.g0_.x) * _e37.g1_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))), ((((vec2<f32>(_e48.g0_.x) * vec2<f32>(_e52.g0_.x, _e55.g2_.x)) * vec2<f32>(-(1.0), 1.0)) + ((vec2<f32>(_e65.g0_.y) * vec2<f32>(_e69.g0_.y, _e72.g2_.y)) * vec2<f32>(-(1.0), 1.0))) + ((vec2<f32>(_e83.g0_.z) * vec2<f32>(_e87.g0_.z, _e90.g2_.z)) * vec2<f32>(-(1.0), 1.0))));
}

fn radial_point_line_geometric_product(self_236: RadialPoint, other_204: Line) -> Flector {
    var self_237: RadialPoint;
    var other_205: Line;

    self_237 = self_236;
    other_205 = other_204;
    let _e4: RadialPoint = self_237;
    let _e8: Line = other_205;
    let _e11: Line = other_205;
    let _e14: Line = other_205;
    let _e17: Line = other_205;
    let _e30: RadialPoint = self_237;
    let _e34: Line = other_205;
    let _e37: Line = other_205;
    let _e40: Line = other_205;
    let _e43: Line = other_205;
    let _e57: RadialPoint = self_237;
    let _e61: Line = other_205;
    let _e64: Line = other_205;
    let _e67: Line = other_205;
    let _e70: Line = other_205;
    let _e84: RadialPoint = self_237;
    let _e88: Line = other_205;
    let _e91: Line = other_205;
    let _e94: Line = other_205;
    let _e97: Line = other_205;
    let _e110: RadialPoint = self_237;
    let _e114: Line = other_205;
    let _e117: Line = other_205;
    let _e120: Line = other_205;
    let _e123: Line = other_205;
    let _e137: RadialPoint = self_237;
    let _e141: Line = other_205;
    let _e144: Line = other_205;
    let _e147: Line = other_205;
    let _e150: Line = other_205;
    let _e162: RadialPoint = self_237;
    let _e166: Line = other_205;
    let _e169: Line = other_205;
    let _e172: Line = other_205;
    let _e175: Line = other_205;
    return Flector(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.x) * vec4<f32>(_e61.g1_.x, _e64.g1_.z, _e67.g1_.y, _e70.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), (((((vec4<f32>(_e84.g0_.y) * vec4<f32>(_e88.g0_.z, _e91.g0_.z, _e94.g0_.x, _e97.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e110.g0_.z) * vec4<f32>(_e114.g0_.y, _e117.g0_.x, _e120.g0_.y, _e123.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e137.g1_.x) * vec4<f32>(_e141.g1_.x, _e144.g1_.y, _e147.g1_.z, _e150.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e162.g0_.x) * vec4<f32>(_e166.g0_.x, _e169.g0_.z, _e172.g0_.y, _e175.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn radial_point_line_outer_product(self_238: RadialPoint, other_206: Line) -> Plane {
    var self_239: RadialPoint;
    var other_207: Line;

    self_239 = self_238;
    other_207 = other_206;
    let _e4: RadialPoint = self_239;
    let _e8: Line = other_207;
    let _e11: Line = other_207;
    let _e14: Line = other_207;
    let _e17: Line = other_207;
    let _e30: RadialPoint = self_239;
    let _e34: Line = other_207;
    let _e37: Line = other_207;
    let _e40: Line = other_207;
    let _e43: Line = other_207;
    let _e57: RadialPoint = self_239;
    let _e61: Line = other_207;
    let _e64: Line = other_207;
    let _e67: Line = other_207;
    let _e70: Line = other_207;
    let _e82: RadialPoint = self_239;
    let _e86: Line = other_207;
    let _e89: Line = other_207;
    let _e92: Line = other_207;
    let _e95: Line = other_207;
    return Plane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.y, _e37.g0_.x, _e40.g0_.y, _e43.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g1_.x) * vec4<f32>(_e61.g1_.x, _e64.g1_.y, _e67.g1_.z, _e70.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g0_.x) * vec4<f32>(_e86.g0_.x, _e89.g0_.z, _e92.g0_.y, _e95.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn radial_point_line_inner_product(self_240: RadialPoint, other_208: Line) -> FlatPoint {
    var self_241: RadialPoint;
    var other_209: Line;

    self_241 = self_240;
    other_209 = other_208;
    let _e4: RadialPoint = self_241;
    let _e8: Line = other_209;
    let _e11: Line = other_209;
    let _e14: Line = other_209;
    let _e17: Line = other_209;
    let _e30: RadialPoint = self_241;
    let _e34: Line = other_209;
    let _e37: Line = other_209;
    let _e40: Line = other_209;
    let _e43: Line = other_209;
    let _e57: RadialPoint = self_241;
    let _e61: Line = other_209;
    let _e64: Line = other_209;
    let _e67: Line = other_209;
    let _e70: Line = other_209;
    return FlatPoint(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.x) * vec4<f32>(_e61.g1_.x, _e64.g1_.z, _e67.g1_.y, _e70.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn radial_point_line_left_contraction(self_242: RadialPoint, other_210: Line) -> FlatPoint {
    var self_243: RadialPoint;
    var other_211: Line;

    self_243 = self_242;
    other_211 = other_210;
    let _e4: RadialPoint = self_243;
    let _e8: Line = other_211;
    let _e11: Line = other_211;
    let _e14: Line = other_211;
    let _e17: Line = other_211;
    let _e30: RadialPoint = self_243;
    let _e34: Line = other_211;
    let _e37: Line = other_211;
    let _e40: Line = other_211;
    let _e43: Line = other_211;
    let _e57: RadialPoint = self_243;
    let _e61: Line = other_211;
    let _e64: Line = other_211;
    let _e67: Line = other_211;
    let _e70: Line = other_211;
    return FlatPoint(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.x) * vec4<f32>(_e61.g1_.x, _e64.g1_.z, _e67.g1_.y, _e70.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn radial_point_circle_outer_product(self_244: RadialPoint, other_212: Circle) -> Sphere {
    var self_245: RadialPoint;
    var other_213: Circle;

    self_245 = self_244;
    other_213 = other_212;
    let _e4: RadialPoint = self_245;
    let _e7: Circle = other_213;
    let _e11: RadialPoint = self_245;
    let _e14: Circle = other_213;
    let _e19: RadialPoint = self_245;
    let _e22: Circle = other_213;
    let _e27: RadialPoint = self_245;
    let _e30: Circle = other_213;
    let _e35: RadialPoint = self_245;
    let _e39: Circle = other_213;
    let _e42: Circle = other_213;
    let _e45: Circle = other_213;
    let _e48: Circle = other_213;
    let _e61: RadialPoint = self_245;
    let _e65: Circle = other_213;
    let _e68: Circle = other_213;
    let _e71: Circle = other_213;
    let _e74: Circle = other_213;
    let _e88: RadialPoint = self_245;
    let _e92: Circle = other_213;
    let _e95: Circle = other_213;
    let _e98: Circle = other_213;
    let _e101: Circle = other_213;
    let _e113: RadialPoint = self_245;
    let _e117: Circle = other_213;
    let _e121: RadialPoint = self_245;
    let _e125: Circle = other_213;
    let _e128: Circle = other_213;
    let _e131: Circle = other_213;
    let _e134: Circle = other_213;
    return Sphere(((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)) + (_e27.g1_.x * _e30.g0_.w)), ((((((vec4<f32>(_e35.g0_.y) * vec4<f32>(_e39.g1_.z, _e42.g1_.z, _e45.g1_.x, _e48.g2_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e61.g0_.z) * vec4<f32>(_e65.g1_.y, _e68.g1_.x, _e71.g1_.y, _e74.g2_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e88.g1_.x) * vec4<f32>(_e92.g2_.x, _e95.g2_.y, _e98.g2_.z, _e101.g2_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) - (vec4<f32>(_e113.g1_.y) * _e117.g0_)) + ((vec4<f32>(_e121.g0_.x) * vec4<f32>(_e125.g1_.x, _e128.g1_.z, _e131.g1_.y, _e134.g2_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn radial_point_circle_inner_product(self_246: RadialPoint, other_214: Circle) -> Dipole {
    var self_247: RadialPoint;
    var other_215: Circle;

    self_247 = self_246;
    other_215 = other_214;
    let _e4: RadialPoint = self_247;
    let _e8: Circle = other_215;
    let _e11: Circle = other_215;
    let _e14: Circle = other_215;
    let _e25: RadialPoint = self_247;
    let _e29: Circle = other_215;
    let _e32: Circle = other_215;
    let _e35: Circle = other_215;
    let _e47: RadialPoint = self_247;
    let _e51: Circle = other_215;
    let _e54: Circle = other_215;
    let _e57: Circle = other_215;
    let _e69: RadialPoint = self_247;
    let _e71: Circle = other_215;
    let _e80: RadialPoint = self_247;
    let _e84: Circle = other_215;
    let _e87: Circle = other_215;
    let _e90: Circle = other_215;
    let _e93: Circle = other_215;
    let _e106: RadialPoint = self_247;
    let _e110: Circle = other_215;
    let _e113: Circle = other_215;
    let _e116: Circle = other_215;
    let _e119: Circle = other_215;
    let _e133: RadialPoint = self_247;
    let _e137: Circle = other_215;
    let _e140: Circle = other_215;
    let _e143: Circle = other_215;
    let _e146: Circle = other_215;
    return Dipole(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.y, _e32.g0_.x, _e35.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x, _e54.g0_.z, _e57.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))), ((_e69.g0_ * vec3<f32>(_e71.g0_.w)) * vec3<f32>(-(1.0))), ((((vec4<f32>(_e80.g0_.y) * vec4<f32>(_e84.g2_.z, _e87.g2_.z, _e90.g2_.x, _e93.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e106.g0_.z) * vec4<f32>(_e110.g2_.y, _e113.g2_.x, _e116.g2_.y, _e119.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e133.g0_.x) * vec4<f32>(_e137.g2_.x, _e140.g2_.z, _e143.g2_.y, _e146.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn radial_point_circle_left_contraction(self_248: RadialPoint, other_216: Circle) -> Dipole {
    var self_249: RadialPoint;
    var other_217: Circle;

    self_249 = self_248;
    other_217 = other_216;
    let _e4: RadialPoint = self_249;
    let _e8: Circle = other_217;
    let _e11: Circle = other_217;
    let _e14: Circle = other_217;
    let _e25: RadialPoint = self_249;
    let _e29: Circle = other_217;
    let _e32: Circle = other_217;
    let _e35: Circle = other_217;
    let _e47: RadialPoint = self_249;
    let _e51: Circle = other_217;
    let _e54: Circle = other_217;
    let _e57: Circle = other_217;
    let _e69: RadialPoint = self_249;
    let _e71: Circle = other_217;
    let _e80: RadialPoint = self_249;
    let _e84: Circle = other_217;
    let _e87: Circle = other_217;
    let _e90: Circle = other_217;
    let _e93: Circle = other_217;
    let _e106: RadialPoint = self_249;
    let _e110: Circle = other_217;
    let _e113: Circle = other_217;
    let _e116: Circle = other_217;
    let _e119: Circle = other_217;
    let _e133: RadialPoint = self_249;
    let _e137: Circle = other_217;
    let _e140: Circle = other_217;
    let _e143: Circle = other_217;
    let _e146: Circle = other_217;
    return Dipole(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.y, _e32.g0_.x, _e35.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x, _e54.g0_.z, _e57.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))), ((_e69.g0_ * vec3<f32>(_e71.g0_.w)) * vec3<f32>(-(1.0))), ((((vec4<f32>(_e80.g0_.y) * vec4<f32>(_e84.g2_.z, _e87.g2_.z, _e90.g2_.x, _e93.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e106.g0_.z) * vec4<f32>(_e110.g2_.y, _e113.g2_.x, _e116.g2_.y, _e119.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e133.g0_.x) * vec4<f32>(_e137.g2_.x, _e140.g2_.z, _e143.g2_.y, _e146.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn radial_point_plane_outer_product(self_250: RadialPoint, other_218: Plane) -> AntiScalar {
    var self_251: RadialPoint;
    var other_219: Plane;

    self_251 = self_250;
    other_219 = other_218;
    let _e4: RadialPoint = self_251;
    let _e7: Plane = other_219;
    let _e11: RadialPoint = self_251;
    let _e14: Plane = other_219;
    let _e19: RadialPoint = self_251;
    let _e22: Plane = other_219;
    let _e27: RadialPoint = self_251;
    let _e30: Plane = other_219;
    return AntiScalar(((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)) + (_e27.g1_.x * _e30.g0_.w)));
}

fn radial_point_plane_inner_product(self_252: RadialPoint, other_220: Plane) -> Line {
    var self_253: RadialPoint;
    var other_221: Plane;

    self_253 = self_252;
    other_221 = other_220;
    let _e4: RadialPoint = self_253;
    let _e8: Plane = other_221;
    let _e11: Plane = other_221;
    let _e14: Plane = other_221;
    let _e25: RadialPoint = self_253;
    let _e29: Plane = other_221;
    let _e32: Plane = other_221;
    let _e35: Plane = other_221;
    let _e47: RadialPoint = self_253;
    let _e51: Plane = other_221;
    let _e54: Plane = other_221;
    let _e57: Plane = other_221;
    let _e69: RadialPoint = self_253;
    let _e71: Plane = other_221;
    return Line(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.y, _e32.g0_.x, _e35.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x, _e54.g0_.z, _e57.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))), ((_e69.g0_ * vec3<f32>(_e71.g0_.w)) * vec3<f32>(-(1.0))));
}

fn radial_point_plane_left_contraction(self_254: RadialPoint, other_222: Plane) -> Line {
    var self_255: RadialPoint;
    var other_223: Plane;

    self_255 = self_254;
    other_223 = other_222;
    let _e4: RadialPoint = self_255;
    let _e8: Plane = other_223;
    let _e11: Plane = other_223;
    let _e14: Plane = other_223;
    let _e25: RadialPoint = self_255;
    let _e29: Plane = other_223;
    let _e32: Plane = other_223;
    let _e35: Plane = other_223;
    let _e47: RadialPoint = self_255;
    let _e51: Plane = other_223;
    let _e54: Plane = other_223;
    let _e57: Plane = other_223;
    let _e69: RadialPoint = self_255;
    let _e71: Plane = other_223;
    return Line(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.y, _e32.g0_.x, _e35.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x, _e54.g0_.z, _e57.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))), ((_e69.g0_ * vec3<f32>(_e71.g0_.w)) * vec3<f32>(-(1.0))));
}

fn radial_point_sphere_outer_product(self_256: RadialPoint, other_224: Sphere) -> AntiScalar {
    var self_257: RadialPoint;
    var other_225: Sphere;

    self_257 = self_256;
    other_225 = other_224;
    let _e4: RadialPoint = self_257;
    let _e7: Sphere = other_225;
    let _e11: RadialPoint = self_257;
    let _e14: Sphere = other_225;
    let _e19: RadialPoint = self_257;
    let _e22: Sphere = other_225;
    let _e27: RadialPoint = self_257;
    let _e30: Sphere = other_225;
    let _e35: RadialPoint = self_257;
    let _e38: Sphere = other_225;
    return AntiScalar((((((_e4.g0_.x * _e7.g1_.x) + (_e11.g0_.y * _e14.g1_.y)) + (_e19.g0_.z * _e22.g1_.z)) + (_e27.g1_.x * _e30.g1_.w)) + (_e35.g1_.y * _e38.g0_)));
}

fn radial_point_motor_geometric_product(self_258: RadialPoint, other_226: Motor) -> Flector {
    var self_259: RadialPoint;
    var other_227: Motor;

    self_259 = self_258;
    other_227 = other_226;
    let _e4: RadialPoint = self_259;
    let _e8: Motor = other_227;
    let _e11: Motor = other_227;
    let _e14: Motor = other_227;
    let _e17: Motor = other_227;
    let _e30: RadialPoint = self_259;
    let _e34: Motor = other_227;
    let _e37: Motor = other_227;
    let _e40: Motor = other_227;
    let _e43: Motor = other_227;
    let _e57: RadialPoint = self_259;
    let _e61: Motor = other_227;
    let _e64: Motor = other_227;
    let _e67: Motor = other_227;
    let _e70: Motor = other_227;
    let _e84: RadialPoint = self_259;
    let _e88: Motor = other_227;
    let _e99: RadialPoint = self_259;
    let _e103: Motor = other_227;
    let _e106: Motor = other_227;
    let _e109: Motor = other_227;
    let _e112: Motor = other_227;
    let _e125: RadialPoint = self_259;
    let _e129: Motor = other_227;
    let _e132: Motor = other_227;
    let _e135: Motor = other_227;
    let _e138: Motor = other_227;
    let _e152: RadialPoint = self_259;
    let _e156: Motor = other_227;
    let _e159: Motor = other_227;
    let _e162: Motor = other_227;
    let _e165: Motor = other_227;
    let _e179: RadialPoint = self_259;
    let _e183: Motor = other_227;
    return Flector((((((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g1_.w, _e11.g1_.z, _e14.g1_.y, _e17.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e30.g0_.y) * vec4<f32>(_e34.g1_.z, _e37.g1_.w, _e40.g1_.x, _e43.g0_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.z) * vec4<f32>(_e61.g1_.y, _e64.g1_.x, _e67.g1_.w, _e70.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e84.g1_.x) * _e88.g1_.xxxw) * vec4<f32>(0.0, 0.0, 0.0, 1.0))), (((((vec4<f32>(_e99.g0_.x) * vec4<f32>(_e103.g0_.w, _e106.g0_.z, _e109.g0_.y, _e112.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e125.g0_.y) * vec4<f32>(_e129.g0_.z, _e132.g0_.w, _e135.g0_.x, _e138.g1_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e152.g0_.z) * vec4<f32>(_e156.g0_.y, _e159.g0_.x, _e162.g0_.w, _e165.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e179.g1_.x) * _e183.g1_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn radial_point_motor_outer_product(self_260: RadialPoint, other_228: Motor) -> Flector {
    var self_261: RadialPoint;
    var other_229: Motor;

    self_261 = self_260;
    other_229 = other_228;
    let _e4: RadialPoint = self_261;
    let _e8: Motor = other_229;
    let _e19: RadialPoint = self_261;
    let _e22: RadialPoint = self_261;
    let _e25: RadialPoint = self_261;
    let _e28: RadialPoint = self_261;
    let _e32: Motor = other_229;
    let _e43: RadialPoint = self_261;
    let _e47: Motor = other_229;
    let _e50: Motor = other_229;
    let _e53: Motor = other_229;
    let _e56: Motor = other_229;
    let _e69: RadialPoint = self_261;
    let _e73: Motor = other_229;
    let _e76: Motor = other_229;
    let _e79: Motor = other_229;
    let _e82: Motor = other_229;
    let _e96: RadialPoint = self_261;
    let _e100: Motor = other_229;
    let _e111: RadialPoint = self_261;
    let _e115: Motor = other_229;
    let _e118: Motor = other_229;
    let _e121: Motor = other_229;
    let _e124: Motor = other_229;
    return Flector((((vec4<f32>(_e4.g1_.x) * vec4<f32>(_e8.g1_.w)) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + ((vec4<f32>(_e19.g0_.x, _e22.g0_.y, _e25.g0_.z, _e28.g0_.x) * _e32.g1_.wwwx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), (((((vec4<f32>(_e43.g0_.y) * vec4<f32>(_e47.g0_.z, _e50.g0_.z, _e53.g0_.x, _e56.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e69.g0_.z) * vec4<f32>(_e73.g0_.y, _e76.g0_.x, _e79.g0_.y, _e82.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e96.g1_.x) * _e100.g1_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e111.g0_.x) * vec4<f32>(_e115.g0_.x, _e118.g0_.z, _e121.g0_.y, _e124.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn radial_point_translator_outer_product(self_262: RadialPoint, other_230: Translator) -> Plane {
    var self_263: RadialPoint;
    var other_231: Translator;

    self_263 = self_262;
    other_231 = other_230;
    let _e4: RadialPoint = self_263;
    let _e8: Translator = other_231;
    let _e20: RadialPoint = self_263;
    let _e24: Translator = other_231;
    let _e37: RadialPoint = self_263;
    let _e41: Translator = other_231;
    let _e52: RadialPoint = self_263;
    let _e56: Translator = other_231;
    return Plane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e37.g1_.x) * _e41.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e52.g0_.x) * vec4<f32>(_e56.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))));
}

fn radial_point_flector_geometric_product(self_264: RadialPoint, other_232: Flector) -> Motor {
    var self_265: RadialPoint;
    var other_233: Flector;

    self_265 = self_264;
    other_233 = other_232;
    let _e4: RadialPoint = self_265;
    let _e8: Flector = other_233;
    let _e11: Flector = other_233;
    let _e14: Flector = other_233;
    let _e17: Flector = other_233;
    let _e30: RadialPoint = self_265;
    let _e34: Flector = other_233;
    let _e37: Flector = other_233;
    let _e40: Flector = other_233;
    let _e43: Flector = other_233;
    let _e57: RadialPoint = self_265;
    let _e61: Flector = other_233;
    let _e64: Flector = other_233;
    let _e67: Flector = other_233;
    let _e70: Flector = other_233;
    let _e84: RadialPoint = self_265;
    let _e88: Flector = other_233;
    let _e91: Flector = other_233;
    let _e94: Flector = other_233;
    let _e97: Flector = other_233;
    let _e103: RadialPoint = self_265;
    let _e107: Flector = other_233;
    let _e110: Flector = other_233;
    let _e113: Flector = other_233;
    let _e116: Flector = other_233;
    let _e129: RadialPoint = self_265;
    let _e133: Flector = other_233;
    let _e136: Flector = other_233;
    let _e139: Flector = other_233;
    let _e142: Flector = other_233;
    let _e156: RadialPoint = self_265;
    let _e160: Flector = other_233;
    let _e163: Flector = other_233;
    let _e166: Flector = other_233;
    let _e169: Flector = other_233;
    return Motor((((((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g0_.w, _e11.g1_.z, _e14.g1_.y, _e17.g1_.x)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0)) + ((vec4<f32>(_e30.g0_.y) * vec4<f32>(_e34.g1_.z, _e37.g0_.w, _e40.g1_.x, _e43.g1_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e57.g0_.z) * vec4<f32>(_e61.g1_.y, _e64.g1_.x, _e67.g0_.w, _e70.g1_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + (vec4<f32>(_e84.g1_.x) * vec4<f32>(_e88.g0_.x, _e91.g0_.y, _e94.g0_.z, _e97.g1_.w))), ((((vec4<f32>(_e103.g0_.x) * vec4<f32>(_e107.g1_.w, _e110.g0_.z, _e113.g0_.y, _e116.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0)) + ((vec4<f32>(_e129.g0_.y) * vec4<f32>(_e133.g0_.z, _e136.g1_.w, _e139.g0_.x, _e142.g0_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e156.g0_.z) * vec4<f32>(_e160.g0_.y, _e163.g0_.x, _e166.g1_.w, _e169.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn radial_point_multi_vector_add(self_266: RadialPoint, other_234: MultiVector) -> MultiVector {
    var self_267: RadialPoint;
    var other_235: MultiVector;

    self_267 = self_266;
    other_235 = other_234;
    let _e4: MultiVector = other_235;
    let _e6: RadialPoint = self_267;
    let _e8: MultiVector = other_235;
    let _e11: RadialPoint = self_267;
    let _e13: MultiVector = other_235;
    let _e16: MultiVector = other_235;
    let _e18: MultiVector = other_235;
    let _e20: MultiVector = other_235;
    let _e22: MultiVector = other_235;
    let _e24: MultiVector = other_235;
    let _e26: MultiVector = other_235;
    let _e28: MultiVector = other_235;
    return MultiVector(_e4.g0_, (_e6.g0_ + _e8.g1_), (_e11.g1_ + _e13.g2_), _e16.g3_, _e18.g4_, _e20.g5_, _e22.g6_, _e24.g7_, _e26.g8_, _e28.g9_);
}

fn radial_point_multi_vector_sub(self_268: RadialPoint, other_236: MultiVector) -> MultiVector {
    var self_269: RadialPoint;
    var other_237: MultiVector;

    self_269 = self_268;
    other_237 = other_236;
    let _e6: MultiVector = other_237;
    let _e9: RadialPoint = self_269;
    let _e11: MultiVector = other_237;
    let _e14: RadialPoint = self_269;
    let _e16: MultiVector = other_237;
    let _e21: MultiVector = other_237;
    let _e26: MultiVector = other_237;
    let _e31: MultiVector = other_237;
    let _e36: MultiVector = other_237;
    let _e41: MultiVector = other_237;
    let _e46: MultiVector = other_237;
    let _e51: MultiVector = other_237;
    return MultiVector((vec3<f32>(0.0) - _e6.g0_), (_e9.g0_ - _e11.g1_), (_e14.g1_ - _e16.g2_), (vec4<f32>(0.0) - _e21.g3_), (vec3<f32>(0.0) - _e26.g4_), (vec3<f32>(0.0) - _e31.g5_), (vec3<f32>(0.0) - _e36.g6_), (vec3<f32>(0.0) - _e41.g7_), (vec4<f32>(0.0) - _e46.g8_), (vec4<f32>(0.0) - _e51.g9_));
}

fn radial_point_multi_vector_geometric_product(self_270: RadialPoint, other_238: MultiVector) -> MultiVector {
    var self_271: RadialPoint;
    var other_239: MultiVector;

    self_271 = self_270;
    other_239 = other_238;
    let _e4: RadialPoint = self_271;
    let _e8: MultiVector = other_239;
    let _e11: MultiVector = other_239;
    let _e14: MultiVector = other_239;
    let _e19: RadialPoint = self_271;
    let _e23: MultiVector = other_239;
    let _e26: MultiVector = other_239;
    let _e29: MultiVector = other_239;
    let _e35: RadialPoint = self_271;
    let _e39: MultiVector = other_239;
    let _e42: MultiVector = other_239;
    let _e45: MultiVector = other_239;
    let _e51: RadialPoint = self_271;
    let _e55: MultiVector = other_239;
    let _e66: RadialPoint = self_271;
    let _e70: MultiVector = other_239;
    let _e73: MultiVector = other_239;
    let _e76: MultiVector = other_239;
    let _e87: RadialPoint = self_271;
    let _e91: MultiVector = other_239;
    let _e94: MultiVector = other_239;
    let _e97: MultiVector = other_239;
    let _e108: RadialPoint = self_271;
    let _e112: MultiVector = other_239;
    let _e115: MultiVector = other_239;
    let _e118: MultiVector = other_239;
    let _e130: RadialPoint = self_271;
    let _e134: MultiVector = other_239;
    let _e137: MultiVector = other_239;
    let _e140: MultiVector = other_239;
    let _e152: RadialPoint = self_271;
    let _e156: MultiVector = other_239;
    let _e159: MultiVector = other_239;
    let _e169: RadialPoint = self_271;
    let _e173: MultiVector = other_239;
    let _e176: MultiVector = other_239;
    let _e187: RadialPoint = self_271;
    let _e191: MultiVector = other_239;
    let _e194: MultiVector = other_239;
    let _e205: RadialPoint = self_271;
    let _e207: MultiVector = other_239;
    let _e213: RadialPoint = self_271;
    let _e217: MultiVector = other_239;
    let _e220: MultiVector = other_239;
    let _e223: MultiVector = other_239;
    let _e226: MultiVector = other_239;
    let _e239: RadialPoint = self_271;
    let _e243: MultiVector = other_239;
    let _e246: MultiVector = other_239;
    let _e249: MultiVector = other_239;
    let _e252: MultiVector = other_239;
    let _e266: RadialPoint = self_271;
    let _e270: MultiVector = other_239;
    let _e273: MultiVector = other_239;
    let _e276: MultiVector = other_239;
    let _e279: MultiVector = other_239;
    let _e293: RadialPoint = self_271;
    let _e297: MultiVector = other_239;
    let _e300: MultiVector = other_239;
    let _e303: MultiVector = other_239;
    let _e306: MultiVector = other_239;
    let _e312: RadialPoint = self_271;
    let _e316: MultiVector = other_239;
    let _e319: MultiVector = other_239;
    let _e322: MultiVector = other_239;
    let _e325: MultiVector = other_239;
    let _e337: RadialPoint = self_271;
    let _e341: MultiVector = other_239;
    let _e344: MultiVector = other_239;
    let _e347: MultiVector = other_239;
    let _e359: RadialPoint = self_271;
    let _e363: MultiVector = other_239;
    let _e366: MultiVector = other_239;
    let _e369: MultiVector = other_239;
    let _e382: RadialPoint = self_271;
    let _e386: MultiVector = other_239;
    let _e389: MultiVector = other_239;
    let _e392: MultiVector = other_239;
    let _e405: RadialPoint = self_271;
    let _e409: MultiVector = other_239;
    let _e413: RadialPoint = self_271;
    let _e417: MultiVector = other_239;
    let _e420: MultiVector = other_239;
    let _e423: MultiVector = other_239;
    let _e435: RadialPoint = self_271;
    let _e439: MultiVector = other_239;
    let _e442: MultiVector = other_239;
    let _e445: MultiVector = other_239;
    let _e458: RadialPoint = self_271;
    let _e462: MultiVector = other_239;
    let _e465: MultiVector = other_239;
    let _e468: MultiVector = other_239;
    let _e481: RadialPoint = self_271;
    let _e485: MultiVector = other_239;
    let _e488: MultiVector = other_239;
    let _e491: MultiVector = other_239;
    let _e503: RadialPoint = self_271;
    let _e507: MultiVector = other_239;
    let _e510: MultiVector = other_239;
    let _e513: MultiVector = other_239;
    let _e526: RadialPoint = self_271;
    let _e530: MultiVector = other_239;
    let _e533: MultiVector = other_239;
    let _e536: MultiVector = other_239;
    let _e549: RadialPoint = self_271;
    let _e553: MultiVector = other_239;
    let _e556: MultiVector = other_239;
    let _e559: MultiVector = other_239;
    let _e565: RadialPoint = self_271;
    let _e569: MultiVector = other_239;
    let _e573: RadialPoint = self_271;
    let _e577: MultiVector = other_239;
    let _e580: MultiVector = other_239;
    let _e583: MultiVector = other_239;
    let _e595: RadialPoint = self_271;
    let _e599: MultiVector = other_239;
    let _e602: MultiVector = other_239;
    let _e605: MultiVector = other_239;
    let _e618: RadialPoint = self_271;
    let _e622: MultiVector = other_239;
    let _e625: MultiVector = other_239;
    let _e628: MultiVector = other_239;
    let _e641: RadialPoint = self_271;
    let _e645: MultiVector = other_239;
    let _e649: RadialPoint = self_271;
    let _e653: MultiVector = other_239;
    let _e656: MultiVector = other_239;
    let _e659: MultiVector = other_239;
    let _e662: MultiVector = other_239;
    let _e675: RadialPoint = self_271;
    let _e679: MultiVector = other_239;
    let _e682: MultiVector = other_239;
    let _e685: MultiVector = other_239;
    let _e688: MultiVector = other_239;
    let _e702: RadialPoint = self_271;
    let _e706: MultiVector = other_239;
    let _e709: MultiVector = other_239;
    let _e712: MultiVector = other_239;
    let _e715: MultiVector = other_239;
    let _e729: RadialPoint = self_271;
    let _e733: MultiVector = other_239;
    let _e736: MultiVector = other_239;
    let _e739: MultiVector = other_239;
    let _e742: MultiVector = other_239;
    let _e754: RadialPoint = self_271;
    let _e758: MultiVector = other_239;
    let _e761: MultiVector = other_239;
    let _e764: MultiVector = other_239;
    let _e767: MultiVector = other_239;
    let _e780: RadialPoint = self_271;
    let _e784: MultiVector = other_239;
    let _e787: MultiVector = other_239;
    let _e790: MultiVector = other_239;
    let _e793: MultiVector = other_239;
    let _e807: RadialPoint = self_271;
    let _e811: MultiVector = other_239;
    let _e814: MultiVector = other_239;
    let _e817: MultiVector = other_239;
    let _e820: MultiVector = other_239;
    let _e834: RadialPoint = self_271;
    let _e838: MultiVector = other_239;
    let _e842: RadialPoint = self_271;
    let _e846: MultiVector = other_239;
    let _e849: MultiVector = other_239;
    let _e852: MultiVector = other_239;
    let _e855: MultiVector = other_239;
    return MultiVector((((((vec3<f32>(_e4.g0_.x) * vec3<f32>(_e8.g1_.x, _e11.g8_.x, _e14.g9_.x)) + (vec3<f32>(_e19.g0_.y) * vec3<f32>(_e23.g1_.y, _e26.g8_.y, _e29.g9_.y))) + (vec3<f32>(_e35.g0_.z) * vec3<f32>(_e39.g1_.z, _e42.g8_.z, _e45.g9_.z))) + ((vec3<f32>(_e51.g1_.y) * vec3<f32>(_e55.g0_.y)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e66.g1_.x) * vec3<f32>(_e70.g8_.x, _e73.g8_.w, _e76.g9_.w)) * vec3<f32>(0.0, 1.0, 1.0))), ((((vec3<f32>(_e87.g0_.x) * vec3<f32>(_e91.g0_.x, _e94.g5_.z, _e97.g5_.y)) * vec3<f32>(1.0, 1.0, -(1.0))) + ((vec3<f32>(_e108.g0_.y) * vec3<f32>(_e112.g5_.z, _e115.g0_.x, _e118.g5_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e130.g0_.z) * vec3<f32>(_e134.g5_.y, _e137.g5_.x, _e140.g0_.x)) * vec3<f32>(1.0, -(1.0), 1.0))), (((((vec2<f32>(_e152.g0_.x) * vec2<f32>(_e156.g4_.x, _e159.g3_.x)) * vec2<f32>(-(1.0), 1.0)) + ((vec2<f32>(_e169.g0_.y) * vec2<f32>(_e173.g4_.y, _e176.g3_.y)) * vec2<f32>(-(1.0), 1.0))) + ((vec2<f32>(_e187.g0_.z) * vec2<f32>(_e191.g4_.z, _e194.g3_.z)) * vec2<f32>(-(1.0), 1.0))) + (_e205.g1_ * vec2<f32>(_e207.g0_.x))), ((((((vec4<f32>(_e213.g0_.x) * vec4<f32>(_e217.g2_.y, _e220.g7_.z, _e223.g7_.y, _e226.g6_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e239.g0_.y) * vec4<f32>(_e243.g7_.z, _e246.g2_.y, _e249.g7_.x, _e252.g6_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e266.g0_.z) * vec4<f32>(_e270.g7_.y, _e273.g7_.x, _e276.g2_.y, _e279.g6_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) - (vec4<f32>(_e293.g1_.y) * vec4<f32>(_e297.g1_.x, _e300.g1_.y, _e303.g1_.z, _e306.g2_.x))) + ((vec4<f32>(_e312.g1_.x) * vec4<f32>(_e316.g2_.x, _e319.g2_.x, _e322.g2_.x, _e325.g2_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))), (((((vec3<f32>(_e337.g0_.x) * vec3<f32>(_e341.g2_.x, _e344.g8_.z, _e347.g8_.y)) * vec3<f32>(-(1.0), -(1.0), 1.0)) + ((vec3<f32>(_e359.g0_.y) * vec3<f32>(_e363.g8_.z, _e366.g2_.x, _e369.g8_.x)) * vec3<f32>(1.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e382.g0_.z) * vec3<f32>(_e386.g8_.y, _e389.g8_.x, _e392.g2_.x)) * vec3<f32>(-(1.0), 1.0, -(1.0)))) + (vec3<f32>(_e405.g1_.x) * _e409.g1_)), ((((vec3<f32>(_e413.g0_.x) * vec3<f32>(_e417.g8_.w, _e420.g1_.z, _e423.g1_.y)) * vec3<f32>(-(1.0), -(1.0), 1.0)) + ((vec3<f32>(_e435.g0_.y) * vec3<f32>(_e439.g1_.z, _e442.g8_.w, _e445.g1_.x)) * vec3<f32>(1.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e458.g0_.z) * vec3<f32>(_e462.g1_.y, _e465.g1_.x, _e468.g8_.w)) * vec3<f32>(-(1.0), 1.0, -(1.0)))), ((((((vec3<f32>(_e481.g0_.x) * vec3<f32>(_e485.g3_.w, _e488.g9_.z, _e491.g9_.y)) * vec3<f32>(-(1.0), -(1.0), 1.0)) + ((vec3<f32>(_e503.g0_.y) * vec3<f32>(_e507.g9_.z, _e510.g3_.w, _e513.g9_.x)) * vec3<f32>(1.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e526.g0_.z) * vec3<f32>(_e530.g9_.y, _e533.g9_.x, _e536.g3_.w)) * vec3<f32>(-(1.0), 1.0, -(1.0)))) + (vec3<f32>(_e549.g1_.x) * vec3<f32>(_e553.g3_.x, _e556.g3_.y, _e559.g3_.z))) + (vec3<f32>(_e565.g1_.y) * _e569.g4_)), (((((vec3<f32>(_e573.g0_.x) * vec3<f32>(_e577.g9_.w, _e580.g3_.z, _e583.g3_.y)) * vec3<f32>(-(1.0), -(1.0), 1.0)) + ((vec3<f32>(_e595.g0_.y) * vec3<f32>(_e599.g3_.z, _e602.g9_.w, _e605.g3_.x)) * vec3<f32>(1.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e618.g0_.z) * vec3<f32>(_e622.g3_.y, _e625.g3_.x, _e628.g9_.w)) * vec3<f32>(-(1.0), 1.0, -(1.0)))) + (vec3<f32>(_e641.g1_.y) * _e645.g5_)), (((((vec4<f32>(_e649.g0_.x) * vec4<f32>(_e653.g0_.y, _e656.g4_.z, _e659.g4_.y, _e662.g5_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e675.g0_.y) * vec4<f32>(_e679.g4_.z, _e682.g0_.y, _e685.g4_.x, _e688.g5_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e702.g0_.z) * vec4<f32>(_e706.g4_.y, _e709.g4_.x, _e712.g0_.y, _e715.g5_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e729.g1_.x) * vec4<f32>(_e733.g5_.x, _e736.g5_.y, _e739.g5_.z, _e742.g5_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), ((((((vec4<f32>(_e754.g0_.x) * vec4<f32>(_e758.g0_.z, _e761.g6_.z, _e764.g6_.y, _e767.g7_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e780.g0_.y) * vec4<f32>(_e784.g6_.z, _e787.g0_.z, _e790.g6_.x, _e793.g7_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e807.g0_.z) * vec4<f32>(_e811.g6_.y, _e814.g6_.x, _e817.g0_.z, _e820.g7_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) - (vec4<f32>(_e834.g1_.y) * _e838.g8_)) + ((vec4<f32>(_e842.g1_.x) * vec4<f32>(_e846.g7_.x, _e849.g7_.y, _e852.g7_.z, _e855.g7_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn radial_point_multi_vector_scalar_product(self_272: RadialPoint, other_240: MultiVector) -> Scalar {
    var self_273: RadialPoint;
    var other_241: MultiVector;

    self_273 = self_272;
    other_241 = other_240;
    let _e4: RadialPoint = self_273;
    let _e7: MultiVector = other_241;
    let _e11: RadialPoint = self_273;
    let _e14: MultiVector = other_241;
    let _e19: RadialPoint = self_273;
    let _e22: MultiVector = other_241;
    return Scalar((((_e4.g0_.x * _e7.g1_.x) + (_e11.g0_.y * _e14.g1_.y)) + (_e19.g0_.z * _e22.g1_.z)));
}

fn radial_point_squared_magnitude(self_274: RadialPoint) -> Scalar {
    var self_275: RadialPoint;

    self_275 = self_274;
    let _e4: RadialPoint = self_275;
    let _e5: RadialPoint = radial_point_reversal(_e4);
    let _e6: RadialPoint = self_275;
    let _e8: RadialPoint = self_275;
    let _e9: RadialPoint = radial_point_reversal(_e8);
    let _e10: Scalar = radial_point_radial_point_scalar_product(_e6, _e9);
    return _e10;
}

fn radial_point_magnitude(self_276: RadialPoint) -> Scalar {
    var self_277: RadialPoint;

    self_277 = self_276;
    let _e3: RadialPoint = self_277;
    let _e4: Scalar = radial_point_squared_magnitude(_e3);
    let _e7: RadialPoint = self_277;
    let _e8: Scalar = radial_point_squared_magnitude(_e7);
    return Scalar(sqrt(_e8.g0_));
}

fn radial_point_scale(self_278: RadialPoint, other_242: f32) -> RadialPoint {
    var self_279: RadialPoint;
    var other_243: f32;

    self_279 = self_278;
    other_243 = other_242;
    let _e5: f32 = other_243;
    let _e7: RadialPoint = self_279;
    let _e8: f32 = other_243;
    let _e10: RadialPoint = radial_point_scalar_geometric_product(_e7, Scalar(_e8));
    return _e10;
}

fn radial_point_signum(self_280: RadialPoint) -> RadialPoint {
    var self_281: RadialPoint;

    self_281 = self_280;
    let _e5: RadialPoint = self_281;
    let _e6: Scalar = radial_point_magnitude(_e5);
    let _e10: RadialPoint = self_281;
    let _e13: RadialPoint = self_281;
    let _e14: Scalar = radial_point_magnitude(_e13);
    let _e18: RadialPoint = radial_point_scalar_geometric_product(_e10, Scalar((1.0 / _e14.g0_)));
    return _e18;
}

fn radial_point_inverse(self_282: RadialPoint) -> RadialPoint {
    var self_283: RadialPoint;

    self_283 = self_282;
    let _e3: RadialPoint = self_283;
    let _e4: RadialPoint = radial_point_reversal(_e3);
    let _e7: RadialPoint = self_283;
    let _e8: Scalar = radial_point_squared_magnitude(_e7);
    let _e13: RadialPoint = self_283;
    let _e14: RadialPoint = radial_point_reversal(_e13);
    let _e17: RadialPoint = self_283;
    let _e18: Scalar = radial_point_squared_magnitude(_e17);
    let _e22: RadialPoint = radial_point_scalar_geometric_product(_e14, Scalar((1.0 / _e18.g0_)));
    return _e22;
}

fn flat_point_zero() -> FlatPoint {
    return FlatPoint(vec4<f32>(0.0));
}

fn flat_point_one() -> FlatPoint {
    return FlatPoint(vec4<f32>(0.0));
}

fn flat_point_neg(self_284: FlatPoint) -> FlatPoint {
    var self_285: FlatPoint;

    self_285 = self_284;
    let _e2: FlatPoint = self_285;
    return FlatPoint((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn flat_point_automorphism(self_286: FlatPoint) -> FlatPoint {
    var self_287: FlatPoint;

    self_287 = self_286;
    let _e2: FlatPoint = self_287;
    return FlatPoint(_e2.g0_);
}

fn flat_point_reversal(self_288: FlatPoint) -> FlatPoint {
    var self_289: FlatPoint;

    self_289 = self_288;
    let _e2: FlatPoint = self_289;
    return FlatPoint((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn flat_point_conjugation(self_290: FlatPoint) -> FlatPoint {
    var self_291: FlatPoint;

    self_291 = self_290;
    let _e2: FlatPoint = self_291;
    return FlatPoint((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn flat_point_scalar_geometric_product(self_292: FlatPoint, other_244: Scalar) -> FlatPoint {
    var self_293: FlatPoint;
    var other_245: Scalar;

    self_293 = self_292;
    other_245 = other_244;
    let _e4: FlatPoint = self_293;
    let _e6: Scalar = other_245;
    return FlatPoint((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn flat_point_scalar_outer_product(self_294: FlatPoint, other_246: Scalar) -> FlatPoint {
    var self_295: FlatPoint;
    var other_247: Scalar;

    self_295 = self_294;
    other_247 = other_246;
    let _e4: FlatPoint = self_295;
    let _e6: Scalar = other_247;
    return FlatPoint((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn flat_point_scalar_inner_product(self_296: FlatPoint, other_248: Scalar) -> FlatPoint {
    var self_297: FlatPoint;
    var other_249: Scalar;

    self_297 = self_296;
    other_249 = other_248;
    let _e4: FlatPoint = self_297;
    let _e6: Scalar = other_249;
    return FlatPoint((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn flat_point_scalar_right_contraction(self_298: FlatPoint, other_250: Scalar) -> FlatPoint {
    var self_299: FlatPoint;
    var other_251: Scalar;

    self_299 = self_298;
    other_251 = other_250;
    let _e4: FlatPoint = self_299;
    let _e6: Scalar = other_251;
    return FlatPoint((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn flat_point_radial_point_outer_product(self_300: FlatPoint, other_252: RadialPoint) -> Line {
    var self_301: FlatPoint;
    var other_253: RadialPoint;

    self_301 = self_300;
    other_253 = other_252;
    let _e6: FlatPoint = self_301;
    let _e10: RadialPoint = other_253;
    let _e14: FlatPoint = self_301;
    let _e17: FlatPoint = self_301;
    let _e20: FlatPoint = self_301;
    let _e24: RadialPoint = other_253;
    let _e30: FlatPoint = self_301;
    let _e34: RadialPoint = other_253;
    let _e44: FlatPoint = self_301;
    let _e48: RadialPoint = other_253;
    let _e59: FlatPoint = self_301;
    let _e63: RadialPoint = other_253;
    return Line(((vec3<f32>(0.0) - (vec3<f32>(_e6.g0_.w) * _e10.g0_)) + (vec3<f32>(_e14.g0_.x, _e17.g0_.y, _e20.g0_.z) * vec3<f32>(_e24.g1_.x))), ((((vec3<f32>(_e30.g0_.y) * _e34.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e44.g0_.z) * _e48.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e59.g0_.x) * _e63.g0_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn flat_point_flat_point_add(self_302: FlatPoint, other_254: FlatPoint) -> FlatPoint {
    var self_303: FlatPoint;
    var other_255: FlatPoint;

    self_303 = self_302;
    other_255 = other_254;
    let _e4: FlatPoint = self_303;
    let _e6: FlatPoint = other_255;
    return FlatPoint((_e4.g0_ + _e6.g0_));
}

fn flat_point_flat_point_sub(self_304: FlatPoint, other_256: FlatPoint) -> FlatPoint {
    var self_305: FlatPoint;
    var other_257: FlatPoint;

    self_305 = self_304;
    other_257 = other_256;
    let _e4: FlatPoint = self_305;
    let _e6: FlatPoint = other_257;
    return FlatPoint((_e4.g0_ - _e6.g0_));
}

fn flat_point_flat_point_mul(self_306: FlatPoint, other_258: FlatPoint) -> FlatPoint {
    var self_307: FlatPoint;
    var other_259: FlatPoint;

    self_307 = self_306;
    other_259 = other_258;
    let _e4: FlatPoint = self_307;
    let _e6: FlatPoint = other_259;
    return FlatPoint((_e4.g0_ * _e6.g0_));
}

fn flat_point_flat_point_div(self_308: FlatPoint, other_260: FlatPoint) -> FlatPoint {
    var self_309: FlatPoint;
    var other_261: FlatPoint;

    self_309 = self_308;
    other_261 = other_260;
    let _e4: FlatPoint = self_309;
    let _e7: FlatPoint = self_309;
    let _e10: FlatPoint = self_309;
    let _e13: FlatPoint = self_309;
    let _e23: FlatPoint = other_261;
    let _e26: FlatPoint = other_261;
    let _e29: FlatPoint = other_261;
    let _e32: FlatPoint = other_261;
    return FlatPoint((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn flat_point_dipole_add(self_310: FlatPoint, other_262: Dipole) -> Dipole {
    var self_311: FlatPoint;
    var other_263: Dipole;

    self_311 = self_310;
    other_263 = other_262;
    let _e4: Dipole = other_263;
    let _e6: Dipole = other_263;
    let _e8: FlatPoint = self_311;
    let _e10: Dipole = other_263;
    return Dipole(_e4.g0_, _e6.g1_, (_e8.g0_ + _e10.g2_));
}

fn flat_point_dipole_sub(self_312: FlatPoint, other_264: Dipole) -> Dipole {
    var self_313: FlatPoint;
    var other_265: Dipole;

    self_313 = self_312;
    other_265 = other_264;
    let _e6: Dipole = other_265;
    let _e11: Dipole = other_265;
    let _e14: FlatPoint = self_313;
    let _e16: Dipole = other_265;
    return Dipole((vec3<f32>(0.0) - _e6.g0_), (vec3<f32>(0.0) - _e11.g1_), (_e14.g0_ - _e16.g2_));
}

fn flat_point_dipole_geometric_product(self_314: FlatPoint, other_266: Dipole) -> Flector {
    var self_315: FlatPoint;
    var other_267: Dipole;

    self_315 = self_314;
    other_267 = other_266;
    let _e4: FlatPoint = self_315;
    let _e8: Dipole = other_267;
    let _e11: Dipole = other_267;
    let _e14: Dipole = other_267;
    let _e17: Dipole = other_267;
    let _e30: FlatPoint = self_315;
    let _e34: Dipole = other_267;
    let _e37: Dipole = other_267;
    let _e40: Dipole = other_267;
    let _e43: Dipole = other_267;
    let _e57: FlatPoint = self_315;
    let _e61: Dipole = other_267;
    let _e64: Dipole = other_267;
    let _e67: Dipole = other_267;
    let _e70: Dipole = other_267;
    let _e84: FlatPoint = self_315;
    let _e88: Dipole = other_267;
    let _e91: Dipole = other_267;
    let _e94: Dipole = other_267;
    let _e97: Dipole = other_267;
    let _e110: FlatPoint = self_315;
    let _e114: Dipole = other_267;
    let _e117: Dipole = other_267;
    let _e120: Dipole = other_267;
    let _e123: Dipole = other_267;
    let _e137: FlatPoint = self_315;
    let _e141: Dipole = other_267;
    let _e144: Dipole = other_267;
    let _e147: Dipole = other_267;
    let _e150: Dipole = other_267;
    let _e162: FlatPoint = self_315;
    let _e166: Dipole = other_267;
    let _e169: Dipole = other_267;
    let _e172: Dipole = other_267;
    let _e175: Dipole = other_267;
    return Flector(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.x) * vec4<f32>(_e61.g1_.x, _e64.g1_.z, _e67.g1_.y, _e70.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), (((((vec4<f32>(_e84.g0_.y) * vec4<f32>(_e88.g0_.z, _e91.g0_.z, _e94.g0_.x, _e97.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e110.g0_.z) * vec4<f32>(_e114.g0_.y, _e117.g0_.x, _e120.g0_.y, _e123.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e137.g0_.w) * vec4<f32>(_e141.g1_.x, _e144.g1_.y, _e147.g1_.z, _e150.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e162.g0_.x) * vec4<f32>(_e166.g0_.x, _e169.g0_.z, _e172.g0_.y, _e175.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn flat_point_dipole_outer_product(self_316: FlatPoint, other_268: Dipole) -> Plane {
    var self_317: FlatPoint;
    var other_269: Dipole;

    self_317 = self_316;
    other_269 = other_268;
    let _e4: FlatPoint = self_317;
    let _e8: Dipole = other_269;
    let _e11: Dipole = other_269;
    let _e14: Dipole = other_269;
    let _e17: Dipole = other_269;
    let _e30: FlatPoint = self_317;
    let _e34: Dipole = other_269;
    let _e37: Dipole = other_269;
    let _e40: Dipole = other_269;
    let _e43: Dipole = other_269;
    let _e57: FlatPoint = self_317;
    let _e61: Dipole = other_269;
    let _e64: Dipole = other_269;
    let _e67: Dipole = other_269;
    let _e70: Dipole = other_269;
    let _e82: FlatPoint = self_317;
    let _e86: Dipole = other_269;
    let _e89: Dipole = other_269;
    let _e92: Dipole = other_269;
    let _e95: Dipole = other_269;
    return Plane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.y, _e37.g0_.x, _e40.g0_.y, _e43.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g1_.x, _e64.g1_.y, _e67.g1_.z, _e70.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g0_.x) * vec4<f32>(_e86.g0_.x, _e89.g0_.z, _e92.g0_.y, _e95.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn flat_point_circle_outer_product(self_318: FlatPoint, other_270: Circle) -> AntiScalar {
    var self_319: FlatPoint;
    var other_271: Circle;

    self_319 = self_318;
    other_271 = other_270;
    let _e5: FlatPoint = self_319;
    let _e8: Circle = other_271;
    let _e13: FlatPoint = self_319;
    let _e16: Circle = other_271;
    let _e21: FlatPoint = self_319;
    let _e24: Circle = other_271;
    let _e29: FlatPoint = self_319;
    let _e32: Circle = other_271;
    return AntiScalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g0_.w * _e32.g0_.w)));
}

fn flat_point_plane_add(self_320: FlatPoint, other_272: Plane) -> Flector {
    var self_321: FlatPoint;
    var other_273: Plane;

    self_321 = self_320;
    other_273 = other_272;
    let _e4: FlatPoint = self_321;
    let _e6: Plane = other_273;
    return Flector(_e4.g0_, _e6.g0_);
}

fn flat_point_plane_sub(self_322: FlatPoint, other_274: Plane) -> Flector {
    var self_323: FlatPoint;
    var other_275: Plane;

    self_323 = self_322;
    other_275 = other_274;
    let _e4: FlatPoint = self_323;
    let _e8: Plane = other_275;
    return Flector(_e4.g0_, (vec4<f32>(0.0) - _e8.g0_));
}

fn flat_point_flector_add(self_324: FlatPoint, other_276: Flector) -> Flector {
    var self_325: FlatPoint;
    var other_277: Flector;

    self_325 = self_324;
    other_277 = other_276;
    let _e4: FlatPoint = self_325;
    let _e6: Flector = other_277;
    let _e9: Flector = other_277;
    return Flector((_e4.g0_ + _e6.g0_), _e9.g1_);
}

fn flat_point_flector_sub(self_326: FlatPoint, other_278: Flector) -> Flector {
    var self_327: FlatPoint;
    var other_279: Flector;

    self_327 = self_326;
    other_279 = other_278;
    let _e4: FlatPoint = self_327;
    let _e6: Flector = other_279;
    let _e11: Flector = other_279;
    return Flector((_e4.g0_ - _e6.g0_), (vec4<f32>(0.0) - _e11.g1_));
}

fn flat_point_multi_vector_add(self_328: FlatPoint, other_280: MultiVector) -> MultiVector {
    var self_329: FlatPoint;
    var other_281: MultiVector;

    self_329 = self_328;
    other_281 = other_280;
    let _e4: MultiVector = other_281;
    let _e6: MultiVector = other_281;
    let _e8: MultiVector = other_281;
    let _e10: FlatPoint = self_329;
    let _e12: MultiVector = other_281;
    let _e15: MultiVector = other_281;
    let _e17: MultiVector = other_281;
    let _e19: MultiVector = other_281;
    let _e21: MultiVector = other_281;
    let _e23: MultiVector = other_281;
    let _e25: MultiVector = other_281;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, (_e10.g0_ + _e12.g3_), _e15.g4_, _e17.g5_, _e19.g6_, _e21.g7_, _e23.g8_, _e25.g9_);
}

fn flat_point_multi_vector_sub(self_330: FlatPoint, other_282: MultiVector) -> MultiVector {
    var self_331: FlatPoint;
    var other_283: MultiVector;

    self_331 = self_330;
    other_283 = other_282;
    let _e6: MultiVector = other_283;
    let _e11: MultiVector = other_283;
    let _e16: MultiVector = other_283;
    let _e19: FlatPoint = self_331;
    let _e21: MultiVector = other_283;
    let _e26: MultiVector = other_283;
    let _e31: MultiVector = other_283;
    let _e36: MultiVector = other_283;
    let _e41: MultiVector = other_283;
    let _e46: MultiVector = other_283;
    let _e51: MultiVector = other_283;
    return MultiVector((vec3<f32>(0.0) - _e6.g0_), (vec3<f32>(0.0) - _e11.g1_), (vec2<f32>(0.0) - _e16.g2_), (_e19.g0_ - _e21.g3_), (vec3<f32>(0.0) - _e26.g4_), (vec3<f32>(0.0) - _e31.g5_), (vec3<f32>(0.0) - _e36.g6_), (vec3<f32>(0.0) - _e41.g7_), (vec4<f32>(0.0) - _e46.g8_), (vec4<f32>(0.0) - _e51.g9_));
}

fn flat_point_scale(self_332: FlatPoint, other_284: f32) -> FlatPoint {
    var self_333: FlatPoint;
    var other_285: f32;

    self_333 = self_332;
    other_285 = other_284;
    let _e5: f32 = other_285;
    let _e7: FlatPoint = self_333;
    let _e8: f32 = other_285;
    let _e10: FlatPoint = flat_point_scalar_geometric_product(_e7, Scalar(_e8));
    return _e10;
}

fn dipole_zero() -> Dipole {
    return Dipole(vec3<f32>(0.0), vec3<f32>(0.0), vec4<f32>(0.0));
}

fn dipole_one() -> Dipole {
    return Dipole(vec3<f32>(0.0), vec3<f32>(0.0), vec4<f32>(0.0));
}

fn dipole_neg(self_334: Dipole) -> Dipole {
    var self_335: Dipole;

    self_335 = self_334;
    let _e2: Dipole = self_335;
    let _e8: Dipole = self_335;
    let _e14: Dipole = self_335;
    return Dipole((_e2.g0_ * vec3<f32>(-(1.0))), (_e8.g1_ * vec3<f32>(-(1.0))), (_e14.g2_ * vec4<f32>(-(1.0))));
}

fn dipole_automorphism(self_336: Dipole) -> Dipole {
    var self_337: Dipole;

    self_337 = self_336;
    let _e2: Dipole = self_337;
    let _e4: Dipole = self_337;
    let _e6: Dipole = self_337;
    return Dipole(_e2.g0_, _e4.g1_, _e6.g2_);
}

fn dipole_reversal(self_338: Dipole) -> Dipole {
    var self_339: Dipole;

    self_339 = self_338;
    let _e2: Dipole = self_339;
    let _e8: Dipole = self_339;
    let _e14: Dipole = self_339;
    return Dipole((_e2.g0_ * vec3<f32>(-(1.0))), (_e8.g1_ * vec3<f32>(-(1.0))), (_e14.g2_ * vec4<f32>(-(1.0))));
}

fn dipole_conjugation(self_340: Dipole) -> Dipole {
    var self_341: Dipole;

    self_341 = self_340;
    let _e2: Dipole = self_341;
    let _e8: Dipole = self_341;
    let _e14: Dipole = self_341;
    return Dipole((_e2.g0_ * vec3<f32>(-(1.0))), (_e8.g1_ * vec3<f32>(-(1.0))), (_e14.g2_ * vec4<f32>(-(1.0))));
}

fn dipole_scalar_geometric_product(self_342: Dipole, other_286: Scalar) -> Dipole {
    var self_343: Dipole;
    var other_287: Scalar;

    self_343 = self_342;
    other_287 = other_286;
    let _e4: Dipole = self_343;
    let _e6: Scalar = other_287;
    let _e10: Dipole = self_343;
    let _e12: Scalar = other_287;
    let _e16: Dipole = self_343;
    let _e18: Scalar = other_287;
    return Dipole((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)), (_e16.g2_ * vec4<f32>(_e18.g0_)));
}

fn dipole_scalar_outer_product(self_344: Dipole, other_288: Scalar) -> Dipole {
    var self_345: Dipole;
    var other_289: Scalar;

    self_345 = self_344;
    other_289 = other_288;
    let _e4: Dipole = self_345;
    let _e6: Scalar = other_289;
    let _e10: Dipole = self_345;
    let _e12: Scalar = other_289;
    let _e16: Dipole = self_345;
    let _e18: Scalar = other_289;
    return Dipole((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)), (_e16.g2_ * vec4<f32>(_e18.g0_)));
}

fn dipole_scalar_inner_product(self_346: Dipole, other_290: Scalar) -> Dipole {
    var self_347: Dipole;
    var other_291: Scalar;

    self_347 = self_346;
    other_291 = other_290;
    let _e4: Dipole = self_347;
    let _e6: Scalar = other_291;
    let _e10: Dipole = self_347;
    let _e12: Scalar = other_291;
    let _e16: Dipole = self_347;
    let _e18: Scalar = other_291;
    return Dipole((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)), (_e16.g2_ * vec4<f32>(_e18.g0_)));
}

fn dipole_scalar_right_contraction(self_348: Dipole, other_292: Scalar) -> Dipole {
    var self_349: Dipole;
    var other_293: Scalar;

    self_349 = self_348;
    other_293 = other_292;
    let _e4: Dipole = self_349;
    let _e6: Scalar = other_293;
    let _e10: Dipole = self_349;
    let _e12: Scalar = other_293;
    let _e16: Dipole = self_349;
    let _e18: Scalar = other_293;
    return Dipole((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)), (_e16.g2_ * vec4<f32>(_e18.g0_)));
}

fn dipole_radial_point_outer_product(self_350: Dipole, other_294: RadialPoint) -> Circle {
    var self_351: Dipole;
    var other_295: RadialPoint;

    self_351 = self_350;
    other_295 = other_294;
    let _e4: Dipole = self_351;
    let _e8: RadialPoint = other_295;
    let _e11: RadialPoint = other_295;
    let _e14: RadialPoint = other_295;
    let _e17: RadialPoint = other_295;
    let _e29: Dipole = self_351;
    let _e33: RadialPoint = other_295;
    let _e36: RadialPoint = other_295;
    let _e39: RadialPoint = other_295;
    let _e42: RadialPoint = other_295;
    let _e55: Dipole = self_351;
    let _e59: RadialPoint = other_295;
    let _e62: RadialPoint = other_295;
    let _e65: RadialPoint = other_295;
    let _e68: RadialPoint = other_295;
    let _e81: Dipole = self_351;
    let _e85: RadialPoint = other_295;
    let _e88: RadialPoint = other_295;
    let _e91: RadialPoint = other_295;
    let _e94: RadialPoint = other_295;
    let _e107: Dipole = self_351;
    let _e111: RadialPoint = other_295;
    let _e114: RadialPoint = other_295;
    let _e117: RadialPoint = other_295;
    let _e120: RadialPoint = other_295;
    let _e133: Dipole = self_351;
    let _e137: RadialPoint = other_295;
    let _e140: RadialPoint = other_295;
    let _e143: RadialPoint = other_295;
    let _e146: RadialPoint = other_295;
    let _e159: Dipole = self_351;
    let _e163: RadialPoint = other_295;
    let _e173: Dipole = self_351;
    let _e177: RadialPoint = other_295;
    let _e188: Dipole = self_351;
    let _e192: RadialPoint = other_295;
    let _e203: Dipole = self_351;
    let _e207: RadialPoint = other_295;
    let _e211: Dipole = self_351;
    let _e213: RadialPoint = other_295;
    let _e219: Dipole = self_351;
    let _e223: RadialPoint = other_295;
    let _e233: Dipole = self_351;
    let _e237: RadialPoint = other_295;
    let _e248: Dipole = self_351;
    let _e252: RadialPoint = other_295;
    let _e263: Dipole = self_351;
    let _e265: RadialPoint = other_295;
    return Circle((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g0_.z)) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.x, _e39.g0_.y, _e42.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e55.g1_.x) * vec4<f32>(_e59.g1_.x, _e62.g1_.x, _e65.g1_.x, _e68.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e81.g1_.y) * vec4<f32>(_e85.g1_.x, _e88.g1_.x, _e91.g1_.x, _e94.g0_.y)) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e107.g1_.z) * vec4<f32>(_e111.g1_.x, _e114.g1_.x, _e117.g1_.x, _e120.g0_.z)) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e133.g0_.x) * vec4<f32>(_e137.g0_.x, _e140.g0_.z, _e143.g0_.y, _e146.g0_.x)) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))), ((((((vec3<f32>(_e159.g2_.x) * vec3<f32>(_e163.g1_.x)) * vec3<f32>(1.0, 0.0, 0.0)) + ((vec3<f32>(_e173.g2_.y) * vec3<f32>(_e177.g1_.x)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e188.g2_.z) * vec3<f32>(_e192.g1_.x)) * vec3<f32>(0.0, 0.0, 1.0))) - (vec3<f32>(_e203.g2_.w) * _e207.g0_)) + (_e211.g0_ * vec3<f32>(_e213.g1_.y))), (((((vec3<f32>(_e219.g2_.x) * _e223.g0_.zzy) * vec3<f32>(0.0, 1.0, -(1.0))) + ((vec3<f32>(_e233.g2_.y) * _e237.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e248.g2_.z) * _e252.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + (_e263.g1_ * vec3<f32>(_e265.g1_.y))));
}

fn dipole_radial_point_inner_product(self_352: Dipole, other_296: RadialPoint) -> RadialPoint {
    var self_353: Dipole;
    var other_297: RadialPoint;

    self_353 = self_352;
    other_297 = other_296;
    let _e4: Dipole = self_353;
    let _e8: RadialPoint = other_297;
    let _e18: Dipole = self_353;
    let _e22: RadialPoint = other_297;
    let _e33: Dipole = self_353;
    let _e37: RadialPoint = other_297;
    let _e48: Dipole = self_353;
    let _e52: RadialPoint = other_297;
    let _e61: Dipole = self_353;
    let _e65: RadialPoint = other_297;
    let _e75: Dipole = self_353;
    let _e79: RadialPoint = other_297;
    let _e90: Dipole = self_353;
    let _e94: RadialPoint = other_297;
    let _e105: Dipole = self_353;
    let _e109: RadialPoint = other_297;
    let _e120: Dipole = self_353;
    let _e124: RadialPoint = other_297;
    return RadialPoint(((((vec3<f32>(_e4.g1_.y) * _e8.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e18.g1_.z) * _e22.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e33.g1_.x) * _e37.g0_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))), (((((((vec2<f32>(_e48.g0_.y) * vec2<f32>(_e52.g0_.y)) * vec2<f32>(1.0, 0.0)) + ((vec2<f32>(_e61.g0_.z) * vec2<f32>(_e65.g0_.z)) * vec2<f32>(1.0, 0.0))) + ((vec2<f32>(_e75.g2_.x) * vec2<f32>(_e79.g0_.x)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e90.g2_.y) * vec2<f32>(_e94.g0_.y)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e105.g2_.z) * vec2<f32>(_e109.g0_.z)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e120.g0_.x) * vec2<f32>(_e124.g0_.x)) * vec2<f32>(1.0, 0.0))));
}

fn dipole_radial_point_right_contraction(self_354: Dipole, other_298: RadialPoint) -> RadialPoint {
    var self_355: Dipole;
    var other_299: RadialPoint;

    self_355 = self_354;
    other_299 = other_298;
    let _e4: Dipole = self_355;
    let _e8: RadialPoint = other_299;
    let _e18: Dipole = self_355;
    let _e22: RadialPoint = other_299;
    let _e33: Dipole = self_355;
    let _e37: RadialPoint = other_299;
    let _e48: Dipole = self_355;
    let _e52: RadialPoint = other_299;
    let _e61: Dipole = self_355;
    let _e65: RadialPoint = other_299;
    let _e75: Dipole = self_355;
    let _e79: RadialPoint = other_299;
    let _e90: Dipole = self_355;
    let _e94: RadialPoint = other_299;
    let _e105: Dipole = self_355;
    let _e109: RadialPoint = other_299;
    let _e120: Dipole = self_355;
    let _e124: RadialPoint = other_299;
    return RadialPoint(((((vec3<f32>(_e4.g1_.y) * _e8.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e18.g1_.z) * _e22.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e33.g1_.x) * _e37.g0_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))), (((((((vec2<f32>(_e48.g0_.y) * vec2<f32>(_e52.g0_.y)) * vec2<f32>(1.0, 0.0)) + ((vec2<f32>(_e61.g0_.z) * vec2<f32>(_e65.g0_.z)) * vec2<f32>(1.0, 0.0))) + ((vec2<f32>(_e75.g2_.x) * vec2<f32>(_e79.g0_.x)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e90.g2_.y) * vec2<f32>(_e94.g0_.y)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e105.g2_.z) * vec2<f32>(_e109.g0_.z)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e120.g0_.x) * vec2<f32>(_e124.g0_.x)) * vec2<f32>(1.0, 0.0))));
}

fn dipole_flat_point_into(self_356: Dipole) -> FlatPoint {
    var self_357: Dipole;

    self_357 = self_356;
    let _e2: Dipole = self_357;
    return FlatPoint(_e2.g2_);
}

fn dipole_flat_point_add(self_358: Dipole, other_300: FlatPoint) -> Dipole {
    var self_359: Dipole;
    var other_301: FlatPoint;

    self_359 = self_358;
    other_301 = other_300;
    let _e4: Dipole = self_359;
    let _e6: Dipole = self_359;
    let _e8: Dipole = self_359;
    let _e10: FlatPoint = other_301;
    return Dipole(_e4.g0_, _e6.g1_, (_e8.g2_ + _e10.g0_));
}

fn dipole_flat_point_sub(self_360: Dipole, other_302: FlatPoint) -> Dipole {
    var self_361: Dipole;
    var other_303: FlatPoint;

    self_361 = self_360;
    other_303 = other_302;
    let _e4: Dipole = self_361;
    let _e6: Dipole = self_361;
    let _e8: Dipole = self_361;
    let _e10: FlatPoint = other_303;
    return Dipole(_e4.g0_, _e6.g1_, (_e8.g2_ - _e10.g0_));
}

fn dipole_flat_point_geometric_product(self_362: Dipole, other_304: FlatPoint) -> Flector {
    var self_363: Dipole;
    var other_305: FlatPoint;

    self_363 = self_362;
    other_305 = other_304;
    let _e4: Dipole = self_363;
    let _e8: FlatPoint = other_305;
    let _e19: Dipole = self_363;
    let _e23: FlatPoint = other_305;
    let _e35: Dipole = self_363;
    let _e39: FlatPoint = other_305;
    let _e51: Dipole = self_363;
    let _e55: FlatPoint = other_305;
    let _e67: Dipole = self_363;
    let _e70: Dipole = self_363;
    let _e73: Dipole = self_363;
    let _e76: Dipole = self_363;
    let _e80: FlatPoint = other_305;
    let _e92: Dipole = self_363;
    let _e96: FlatPoint = other_305;
    let _e107: Dipole = self_363;
    let _e111: FlatPoint = other_305;
    let _e123: Dipole = self_363;
    let _e127: FlatPoint = other_305;
    let _e139: Dipole = self_363;
    let _e143: FlatPoint = other_305;
    let _e155: Dipole = self_363;
    let _e158: Dipole = self_363;
    let _e161: Dipole = self_363;
    let _e164: Dipole = self_363;
    let _e168: FlatPoint = other_305;
    return Flector(((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.yxyy) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e67.g0_.x, _e70.g1_.x, _e73.g1_.x, _e76.g0_.x) * _e80.g0_.xzyx) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))), ((((((vec4<f32>(_e92.g0_.y) * _e96.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e107.g0_.z) * _e111.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e123.g1_.y) * _e127.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e139.g1_.z) * _e143.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e155.g1_.x, _e158.g0_.x, _e161.g0_.x, _e164.g1_.x) * _e168.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn dipole_flat_point_outer_product(self_364: Dipole, other_306: FlatPoint) -> Plane {
    var self_365: Dipole;
    var other_307: FlatPoint;

    self_365 = self_364;
    other_307 = other_306;
    let _e4: Dipole = self_365;
    let _e8: FlatPoint = other_307;
    let _e19: Dipole = self_365;
    let _e23: FlatPoint = other_307;
    let _e35: Dipole = self_365;
    let _e39: FlatPoint = other_307;
    let _e51: Dipole = self_365;
    let _e55: FlatPoint = other_307;
    let _e67: Dipole = self_365;
    let _e70: Dipole = self_365;
    let _e73: Dipole = self_365;
    let _e76: Dipole = self_365;
    let _e80: FlatPoint = other_307;
    return Plane(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e67.g1_.x, _e70.g0_.x, _e73.g0_.x, _e76.g1_.x) * _e80.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn dipole_dipole_add(self_366: Dipole, other_308: Dipole) -> Dipole {
    var self_367: Dipole;
    var other_309: Dipole;

    self_367 = self_366;
    other_309 = other_308;
    let _e4: Dipole = self_367;
    let _e6: Dipole = other_309;
    let _e9: Dipole = self_367;
    let _e11: Dipole = other_309;
    let _e14: Dipole = self_367;
    let _e16: Dipole = other_309;
    return Dipole((_e4.g0_ + _e6.g0_), (_e9.g1_ + _e11.g1_), (_e14.g2_ + _e16.g2_));
}

fn dipole_dipole_sub(self_368: Dipole, other_310: Dipole) -> Dipole {
    var self_369: Dipole;
    var other_311: Dipole;

    self_369 = self_368;
    other_311 = other_310;
    let _e4: Dipole = self_369;
    let _e6: Dipole = other_311;
    let _e9: Dipole = self_369;
    let _e11: Dipole = other_311;
    let _e14: Dipole = self_369;
    let _e16: Dipole = other_311;
    return Dipole((_e4.g0_ - _e6.g0_), (_e9.g1_ - _e11.g1_), (_e14.g2_ - _e16.g2_));
}

fn dipole_dipole_mul(self_370: Dipole, other_312: Dipole) -> Dipole {
    var self_371: Dipole;
    var other_313: Dipole;

    self_371 = self_370;
    other_313 = other_312;
    let _e4: Dipole = self_371;
    let _e6: Dipole = other_313;
    let _e9: Dipole = self_371;
    let _e11: Dipole = other_313;
    let _e14: Dipole = self_371;
    let _e16: Dipole = other_313;
    return Dipole((_e4.g0_ * _e6.g0_), (_e9.g1_ * _e11.g1_), (_e14.g2_ * _e16.g2_));
}

fn dipole_dipole_div(self_372: Dipole, other_314: Dipole) -> Dipole {
    var self_373: Dipole;
    var other_315: Dipole;

    self_373 = self_372;
    other_315 = other_314;
    let _e4: Dipole = self_373;
    let _e7: Dipole = self_373;
    let _e10: Dipole = self_373;
    let _e19: Dipole = other_315;
    let _e22: Dipole = other_315;
    let _e25: Dipole = other_315;
    let _e35: Dipole = self_373;
    let _e38: Dipole = self_373;
    let _e41: Dipole = self_373;
    let _e50: Dipole = other_315;
    let _e53: Dipole = other_315;
    let _e56: Dipole = other_315;
    let _e66: Dipole = self_373;
    let _e69: Dipole = self_373;
    let _e72: Dipole = self_373;
    let _e75: Dipole = self_373;
    let _e85: Dipole = other_315;
    let _e88: Dipole = other_315;
    let _e91: Dipole = other_315;
    let _e94: Dipole = other_315;
    return Dipole((((vec3<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e19.g0_.x, _e22.g0_.y, _e25.g0_.z)) * vec3<f32>(1.0, 1.0, 1.0)), (((vec3<f32>(_e35.g1_.x, _e38.g1_.y, _e41.g1_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e50.g1_.x, _e53.g1_.y, _e56.g1_.z)) * vec3<f32>(1.0, 1.0, 1.0)), (((vec4<f32>(_e66.g2_.x, _e69.g2_.y, _e72.g2_.z, _e75.g2_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e85.g2_.x, _e88.g2_.y, _e91.g2_.z, _e94.g2_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn dipole_dipole_outer_product(self_374: Dipole, other_316: Dipole) -> Sphere {
    var self_375: Dipole;
    var other_317: Dipole;

    self_375 = self_374;
    other_317 = other_316;
    let _e5: Dipole = self_375;
    let _e8: Dipole = other_317;
    let _e13: Dipole = self_375;
    let _e16: Dipole = other_317;
    let _e21: Dipole = self_375;
    let _e24: Dipole = other_317;
    let _e29: Dipole = self_375;
    let _e32: Dipole = other_317;
    let _e37: Dipole = self_375;
    let _e40: Dipole = other_317;
    let _e45: Dipole = self_375;
    let _e48: Dipole = other_317;
    let _e53: Dipole = self_375;
    let _e57: Dipole = other_317;
    let _e68: Dipole = self_375;
    let _e72: Dipole = other_317;
    let _e84: Dipole = self_375;
    let _e88: Dipole = other_317;
    let _e100: Dipole = self_375;
    let _e104: Dipole = other_317;
    let _e116: Dipole = self_375;
    let _e120: Dipole = other_317;
    let _e123: Dipole = other_317;
    let _e126: Dipole = other_317;
    let _e129: Dipole = other_317;
    let _e143: Dipole = self_375;
    let _e147: Dipole = other_317;
    let _e150: Dipole = other_317;
    let _e153: Dipole = other_317;
    let _e156: Dipole = other_317;
    let _e170: Dipole = self_375;
    let _e174: Dipole = other_317;
    let _e177: Dipole = other_317;
    let _e180: Dipole = other_317;
    let _e183: Dipole = other_317;
    let _e197: Dipole = self_375;
    let _e201: Dipole = other_317;
    let _e204: Dipole = other_317;
    let _e207: Dipole = other_317;
    let _e210: Dipole = other_317;
    let _e222: Dipole = self_375;
    let _e225: Dipole = self_375;
    let _e228: Dipole = self_375;
    let _e231: Dipole = self_375;
    let _e235: Dipole = other_317;
    return Sphere(((((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)) - (_e29.g1_.x * _e32.g0_.x)) - (_e37.g1_.y * _e40.g0_.y)) - (_e45.g1_.z * _e48.g0_.z)), ((((((((((vec4<f32>(_e53.g0_.y) * _e57.g2_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e68.g0_.z) * _e72.g2_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e84.g1_.y) * _e88.g2_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e100.g1_.z) * _e104.g2_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e116.g2_.x) * vec4<f32>(_e120.g0_.z, _e123.g0_.z, _e126.g0_.y, _e129.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e143.g2_.y) * vec4<f32>(_e147.g0_.z, _e150.g0_.z, _e153.g0_.x, _e156.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e170.g2_.z) * vec4<f32>(_e174.g0_.y, _e177.g0_.x, _e180.g0_.y, _e183.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e197.g2_.w) * vec4<f32>(_e201.g1_.x, _e204.g1_.y, _e207.g1_.z, _e210.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e222.g1_.x, _e225.g0_.x, _e228.g0_.x, _e231.g1_.x) * _e235.g2_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn dipole_dipole_inner_product(self_376: Dipole, other_318: Dipole) -> Scalar {
    var self_377: Dipole;
    var other_319: Dipole;

    self_377 = self_376;
    other_319 = other_318;
    let _e5: Dipole = self_377;
    let _e8: Dipole = other_319;
    let _e13: Dipole = self_377;
    let _e16: Dipole = other_319;
    let _e21: Dipole = self_377;
    let _e24: Dipole = other_319;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g1_.x)) - (_e13.g1_.y * _e16.g1_.y)) - (_e21.g1_.z * _e24.g1_.z)));
}

fn dipole_dipole_left_contraction(self_378: Dipole, other_320: Dipole) -> Scalar {
    var self_379: Dipole;
    var other_321: Dipole;

    self_379 = self_378;
    other_321 = other_320;
    let _e5: Dipole = self_379;
    let _e8: Dipole = other_321;
    let _e13: Dipole = self_379;
    let _e16: Dipole = other_321;
    let _e21: Dipole = self_379;
    let _e24: Dipole = other_321;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g1_.x)) - (_e13.g1_.y * _e16.g1_.y)) - (_e21.g1_.z * _e24.g1_.z)));
}

fn dipole_dipole_right_contraction(self_380: Dipole, other_322: Dipole) -> Scalar {
    var self_381: Dipole;
    var other_323: Dipole;

    self_381 = self_380;
    other_323 = other_322;
    let _e5: Dipole = self_381;
    let _e8: Dipole = other_323;
    let _e13: Dipole = self_381;
    let _e16: Dipole = other_323;
    let _e21: Dipole = self_381;
    let _e24: Dipole = other_323;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g1_.x)) - (_e13.g1_.y * _e16.g1_.y)) - (_e21.g1_.z * _e24.g1_.z)));
}

fn dipole_dipole_scalar_product(self_382: Dipole, other_324: Dipole) -> Scalar {
    var self_383: Dipole;
    var other_325: Dipole;

    self_383 = self_382;
    other_325 = other_324;
    let _e5: Dipole = self_383;
    let _e8: Dipole = other_325;
    let _e13: Dipole = self_383;
    let _e16: Dipole = other_325;
    let _e21: Dipole = self_383;
    let _e24: Dipole = other_325;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g1_.x)) - (_e13.g1_.y * _e16.g1_.y)) - (_e21.g1_.z * _e24.g1_.z)));
}

fn dipole_line_geometric_product(self_384: Dipole, other_326: Line) -> Motor {
    var self_385: Dipole;
    var other_327: Line;

    self_385 = self_384;
    other_327 = other_326;
    let _e4: Dipole = self_385;
    let _e8: Line = other_327;
    let _e11: Line = other_327;
    let _e14: Line = other_327;
    let _e17: Line = other_327;
    let _e30: Dipole = self_385;
    let _e34: Line = other_327;
    let _e37: Line = other_327;
    let _e40: Line = other_327;
    let _e43: Line = other_327;
    let _e57: Dipole = self_385;
    let _e61: Line = other_327;
    let _e64: Line = other_327;
    let _e67: Line = other_327;
    let _e70: Line = other_327;
    let _e84: Dipole = self_385;
    let _e88: Line = other_327;
    let _e91: Line = other_327;
    let _e94: Line = other_327;
    let _e97: Line = other_327;
    let _e111: Dipole = self_385;
    let _e115: Line = other_327;
    let _e118: Line = other_327;
    let _e121: Line = other_327;
    let _e124: Line = other_327;
    let _e138: Dipole = self_385;
    let _e142: Line = other_327;
    let _e145: Line = other_327;
    let _e148: Line = other_327;
    let _e151: Line = other_327;
    let _e165: Dipole = self_385;
    let _e169: Line = other_327;
    let _e172: Line = other_327;
    let _e175: Line = other_327;
    let _e178: Line = other_327;
    let _e191: Dipole = self_385;
    let _e195: Line = other_327;
    let _e198: Line = other_327;
    let _e201: Line = other_327;
    let _e204: Line = other_327;
    let _e218: Dipole = self_385;
    let _e222: Line = other_327;
    let _e225: Line = other_327;
    let _e228: Line = other_327;
    let _e231: Line = other_327;
    return Motor((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g1_.x) * vec4<f32>(_e61.g0_.z, _e64.g0_.z, _e67.g0_.y, _e70.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e84.g1_.y) * vec4<f32>(_e88.g0_.z, _e91.g0_.z, _e94.g0_.x, _e97.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e111.g1_.z) * vec4<f32>(_e115.g0_.y, _e118.g0_.x, _e121.g0_.y, _e124.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e138.g0_.x) * vec4<f32>(_e142.g1_.x, _e145.g1_.z, _e148.g1_.y, _e151.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), ((((vec4<f32>(_e165.g1_.y) * vec4<f32>(_e169.g1_.z, _e172.g1_.z, _e175.g1_.x, _e178.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e191.g1_.z) * vec4<f32>(_e195.g1_.y, _e198.g1_.x, _e201.g1_.y, _e204.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e218.g1_.x) * vec4<f32>(_e222.g1_.x, _e225.g1_.z, _e228.g1_.y, _e231.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn dipole_line_outer_product(self_386: Dipole, other_328: Line) -> AntiScalar {
    var self_387: Dipole;
    var other_329: Line;

    self_387 = self_386;
    other_329 = other_328;
    let _e5: Dipole = self_387;
    let _e8: Line = other_329;
    let _e13: Dipole = self_387;
    let _e16: Line = other_329;
    let _e21: Dipole = self_387;
    let _e24: Line = other_329;
    let _e29: Dipole = self_387;
    let _e32: Line = other_329;
    let _e37: Dipole = self_387;
    let _e40: Line = other_329;
    let _e45: Dipole = self_387;
    let _e48: Line = other_329;
    return AntiScalar(((((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)) - (_e29.g1_.x * _e32.g0_.x)) - (_e37.g1_.y * _e40.g0_.y)) - (_e45.g1_.z * _e48.g0_.z)));
}

fn dipole_circle_outer_product(self_388: Dipole, other_330: Circle) -> AntiScalar {
    var self_389: Dipole;
    var other_331: Circle;

    self_389 = self_388;
    other_331 = other_330;
    let _e5: Dipole = self_389;
    let _e8: Circle = other_331;
    let _e13: Dipole = self_389;
    let _e16: Circle = other_331;
    let _e21: Dipole = self_389;
    let _e24: Circle = other_331;
    let _e29: Dipole = self_389;
    let _e32: Circle = other_331;
    let _e37: Dipole = self_389;
    let _e40: Circle = other_331;
    let _e45: Dipole = self_389;
    let _e48: Circle = other_331;
    let _e53: Dipole = self_389;
    let _e56: Circle = other_331;
    let _e61: Dipole = self_389;
    let _e64: Circle = other_331;
    let _e69: Dipole = self_389;
    let _e72: Circle = other_331;
    let _e77: Dipole = self_389;
    let _e80: Circle = other_331;
    return AntiScalar(((((((((((0.0 - (_e5.g0_.x * _e8.g2_.x)) - (_e13.g0_.y * _e16.g2_.y)) - (_e21.g0_.z * _e24.g2_.z)) - (_e29.g1_.x * _e32.g1_.x)) - (_e37.g1_.y * _e40.g1_.y)) - (_e45.g1_.z * _e48.g1_.z)) - (_e53.g2_.x * _e56.g0_.x)) - (_e61.g2_.y * _e64.g0_.y)) - (_e69.g2_.z * _e72.g0_.z)) - (_e77.g2_.w * _e80.g0_.w)));
}

fn dipole_circle_inner_product(self_390: Dipole, other_332: Circle) -> RadialPoint {
    var self_391: Dipole;
    var other_333: Circle;

    self_391 = self_390;
    other_333 = other_332;
    let _e4: Dipole = self_391;
    let _e6: Circle = other_333;
    let _e13: Dipole = self_391;
    let _e17: Circle = other_333;
    let _e20: Circle = other_333;
    let _e26: Dipole = self_391;
    let _e30: Circle = other_333;
    let _e33: Circle = other_333;
    let _e39: Dipole = self_391;
    let _e43: Circle = other_333;
    let _e46: Circle = other_333;
    return RadialPoint((_e4.g1_ * vec3<f32>(_e6.g0_.w)), (((vec2<f32>(0.0) - (vec2<f32>(_e13.g1_.x) * vec2<f32>(_e17.g0_.x, _e20.g2_.x))) - (vec2<f32>(_e26.g1_.y) * vec2<f32>(_e30.g0_.y, _e33.g2_.y))) - (vec2<f32>(_e39.g1_.z) * vec2<f32>(_e43.g0_.z, _e46.g2_.z))));
}

fn dipole_circle_left_contraction(self_392: Dipole, other_334: Circle) -> RadialPoint {
    var self_393: Dipole;
    var other_335: Circle;

    self_393 = self_392;
    other_335 = other_334;
    let _e4: Dipole = self_393;
    let _e6: Circle = other_335;
    let _e13: Dipole = self_393;
    let _e17: Circle = other_335;
    let _e20: Circle = other_335;
    let _e26: Dipole = self_393;
    let _e30: Circle = other_335;
    let _e33: Circle = other_335;
    let _e39: Dipole = self_393;
    let _e43: Circle = other_335;
    let _e46: Circle = other_335;
    return RadialPoint((_e4.g1_ * vec3<f32>(_e6.g0_.w)), (((vec2<f32>(0.0) - (vec2<f32>(_e13.g1_.x) * vec2<f32>(_e17.g0_.x, _e20.g2_.x))) - (vec2<f32>(_e26.g1_.y) * vec2<f32>(_e30.g0_.y, _e33.g2_.y))) - (vec2<f32>(_e39.g1_.z) * vec2<f32>(_e43.g0_.z, _e46.g2_.z))));
}

fn dipole_plane_inner_product(self_394: Dipole, other_336: Plane) -> FlatPoint {
    var self_395: Dipole;
    var other_337: Plane;

    self_395 = self_394;
    other_337 = other_336;
    let _e4: Dipole = self_395;
    let _e8: Plane = other_337;
    let _e19: Dipole = self_395;
    let _e23: Plane = other_337;
    let _e35: Dipole = self_395;
    let _e39: Plane = other_337;
    return FlatPoint(((((vec4<f32>(_e4.g1_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g1_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.x) * _e39.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn dipole_plane_left_contraction(self_396: Dipole, other_338: Plane) -> FlatPoint {
    var self_397: Dipole;
    var other_339: Plane;

    self_397 = self_396;
    other_339 = other_338;
    let _e4: Dipole = self_397;
    let _e8: Plane = other_339;
    let _e19: Dipole = self_397;
    let _e23: Plane = other_339;
    let _e35: Dipole = self_397;
    let _e39: Plane = other_339;
    return FlatPoint(((((vec4<f32>(_e4.g1_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g1_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.x) * _e39.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn dipole_motor_geometric_product(self_398: Dipole, other_340: Motor) -> Motor {
    var self_399: Dipole;
    var other_341: Motor;

    self_399 = self_398;
    other_341 = other_340;
    let _e4: Dipole = self_399;
    let _e8: Motor = other_341;
    let _e20: Dipole = self_399;
    let _e24: Motor = other_341;
    let _e37: Dipole = self_399;
    let _e41: Motor = other_341;
    let _e54: Dipole = self_399;
    let _e58: Motor = other_341;
    let _e71: Dipole = self_399;
    let _e75: Motor = other_341;
    let _e88: Dipole = self_399;
    let _e92: Motor = other_341;
    let _e105: Dipole = self_399;
    let _e109: Motor = other_341;
    let _e121: Dipole = self_399;
    let _e125: Motor = other_341;
    let _e138: Dipole = self_399;
    let _e142: Motor = other_341;
    return Motor((((((((vec4<f32>(_e4.g0_.x) * _e8.g1_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e20.g0_.y) * _e24.g1_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e37.g0_.z) * _e41.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e54.g1_.x) * _e58.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e71.g1_.y) * _e75.g0_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e88.g1_.z) * _e92.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))), ((((vec4<f32>(_e105.g1_.x) * _e109.g1_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e121.g1_.y) * _e125.g1_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e138.g1_.z) * _e142.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn dipole_rotor_geometric_product(self_400: Dipole, other_342: Rotor) -> Rotor {
    var self_401: Dipole;
    var other_343: Rotor;

    self_401 = self_400;
    other_343 = other_342;
    let _e4: Dipole = self_401;
    let _e8: Rotor = other_343;
    let _e20: Dipole = self_401;
    let _e24: Rotor = other_343;
    let _e37: Dipole = self_401;
    let _e41: Rotor = other_343;
    return Rotor(((((vec4<f32>(_e4.g1_.x) * _e8.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e20.g1_.y) * _e24.g0_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e37.g1_.z) * _e41.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn dipole_rotor_outer_product(self_402: Dipole, other_344: Rotor) -> AntiScalar {
    var self_403: Dipole;
    var other_345: Rotor;

    self_403 = self_402;
    other_345 = other_344;
    let _e5: Dipole = self_403;
    let _e8: Rotor = other_345;
    let _e13: Dipole = self_403;
    let _e16: Rotor = other_345;
    let _e21: Dipole = self_403;
    let _e24: Rotor = other_345;
    return AntiScalar((((0.0 - (_e5.g1_.x * _e8.g0_.x)) - (_e13.g1_.y * _e16.g0_.y)) - (_e21.g1_.z * _e24.g0_.z)));
}

fn dipole_translator_geometric_product(self_404: Dipole, other_346: Translator) -> Motor {
    var self_405: Dipole;
    var other_347: Translator;

    self_405 = self_404;
    other_347 = other_346;
    let _e4: Dipole = self_405;
    let _e8: Translator = other_347;
    let _e20: Dipole = self_405;
    let _e24: Translator = other_347;
    let _e37: Dipole = self_405;
    let _e41: Translator = other_347;
    let _e53: Dipole = self_405;
    let _e57: Translator = other_347;
    let _e69: Dipole = self_405;
    let _e72: Dipole = self_405;
    let _e75: Dipole = self_405;
    let _e78: Dipole = self_405;
    let _e82: Translator = other_347;
    let _e95: Dipole = self_405;
    let _e99: Translator = other_347;
    let _e111: Dipole = self_405;
    let _e115: Translator = other_347;
    let _e128: Dipole = self_405;
    let _e132: Translator = other_347;
    return Motor(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxy) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * _e24.g0_.yxyz) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e37.g1_.y) * vec4<f32>(_e41.g0_.w)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e53.g1_.z) * vec4<f32>(_e57.g0_.w)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e69.g1_.x, _e72.g0_.x, _e75.g0_.x, _e78.g0_.x) * _e82.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))), ((((vec4<f32>(_e95.g1_.y) * _e99.g0_.zzxy) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e111.g1_.z) * _e115.g0_.yxyz) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e128.g1_.x) * _e132.g0_.xzyx) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn dipole_translator_outer_product(self_406: Dipole, other_348: Translator) -> AntiScalar {
    var self_407: Dipole;
    var other_349: Translator;

    self_407 = self_406;
    other_349 = other_348;
    let _e5: Dipole = self_407;
    let _e8: Translator = other_349;
    let _e13: Dipole = self_407;
    let _e16: Translator = other_349;
    let _e21: Dipole = self_407;
    let _e24: Translator = other_349;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn dipole_flector_geometric_product(self_408: Dipole, other_350: Flector) -> Flector {
    var self_409: Dipole;
    var other_351: Flector;

    self_409 = self_408;
    other_351 = other_350;
    let _e4: Dipole = self_409;
    let _e8: Flector = other_351;
    let _e19: Dipole = self_409;
    let _e23: Flector = other_351;
    let _e35: Dipole = self_409;
    let _e39: Flector = other_351;
    let _e42: Flector = other_351;
    let _e45: Flector = other_351;
    let _e48: Flector = other_351;
    let _e62: Dipole = self_409;
    let _e66: Flector = other_351;
    let _e69: Flector = other_351;
    let _e72: Flector = other_351;
    let _e75: Flector = other_351;
    let _e89: Dipole = self_409;
    let _e93: Flector = other_351;
    let _e96: Flector = other_351;
    let _e99: Flector = other_351;
    let _e102: Flector = other_351;
    let _e116: Dipole = self_409;
    let _e120: Flector = other_351;
    let _e132: Dipole = self_409;
    let _e136: Flector = other_351;
    let _e139: Flector = other_351;
    let _e142: Flector = other_351;
    let _e145: Flector = other_351;
    let _e158: Dipole = self_409;
    let _e162: Flector = other_351;
    let _e165: Flector = other_351;
    let _e168: Flector = other_351;
    let _e171: Flector = other_351;
    let _e185: Dipole = self_409;
    let _e189: Flector = other_351;
    let _e192: Flector = other_351;
    let _e195: Flector = other_351;
    let _e198: Flector = other_351;
    let _e212: Dipole = self_409;
    let _e216: Flector = other_351;
    let _e219: Flector = other_351;
    let _e222: Flector = other_351;
    let _e225: Flector = other_351;
    let _e239: Dipole = self_409;
    let _e243: Flector = other_351;
    let _e246: Flector = other_351;
    let _e249: Flector = other_351;
    let _e252: Flector = other_351;
    let _e266: Dipole = self_409;
    let _e270: Flector = other_351;
    let _e273: Flector = other_351;
    let _e276: Flector = other_351;
    let _e279: Flector = other_351;
    return Flector((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g1_.x) * vec4<f32>(_e39.g1_.w, _e42.g0_.z, _e45.g0_.y, _e48.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e62.g1_.y) * vec4<f32>(_e66.g0_.z, _e69.g1_.w, _e72.g0_.x, _e75.g1_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e89.g1_.z) * vec4<f32>(_e93.g0_.y, _e96.g0_.x, _e99.g1_.w, _e102.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e116.g0_.x) * vec4<f32>(_e120.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))), (((((((vec4<f32>(_e132.g0_.y) * vec4<f32>(_e136.g0_.z, _e139.g1_.w, _e142.g0_.x, _e145.g0_.z)) * vec4<f32>(1.0, -(1.0), -(1.0), 0.0)) + ((vec4<f32>(_e158.g0_.z) * vec4<f32>(_e162.g0_.y, _e165.g0_.x, _e168.g1_.w, _e171.g0_.y)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e185.g1_.x) * vec4<f32>(_e189.g0_.w, _e192.g1_.z, _e195.g1_.y, _e198.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e212.g1_.y) * vec4<f32>(_e216.g1_.z, _e219.g0_.w, _e222.g1_.x, _e225.g0_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e239.g1_.z) * vec4<f32>(_e243.g1_.y, _e246.g1_.x, _e249.g0_.w, _e252.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e266.g0_.x) * vec4<f32>(_e270.g1_.w, _e273.g0_.z, _e276.g0_.y, _e279.g1_.x)) * vec4<f32>(-(1.0), -(1.0), 1.0, 0.0))));
}

fn dipole_flector_outer_product(self_410: Dipole, other_352: Flector) -> Plane {
    var self_411: Dipole;
    var other_353: Flector;

    self_411 = self_410;
    other_353 = other_352;
    let _e4: Dipole = self_411;
    let _e8: Flector = other_353;
    let _e19: Dipole = self_411;
    let _e23: Flector = other_353;
    let _e35: Dipole = self_411;
    let _e39: Flector = other_353;
    let _e51: Dipole = self_411;
    let _e55: Flector = other_353;
    let _e67: Dipole = self_411;
    let _e70: Dipole = self_411;
    let _e73: Dipole = self_411;
    let _e76: Dipole = self_411;
    let _e80: Flector = other_353;
    return Plane(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e67.g1_.x, _e70.g0_.x, _e73.g0_.x, _e76.g1_.x) * _e80.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn dipole_flector_inner_product(self_412: Dipole, other_354: Flector) -> FlatPoint {
    var self_413: Dipole;
    var other_355: Flector;

    self_413 = self_412;
    other_355 = other_354;
    let _e4: Dipole = self_413;
    let _e8: Flector = other_355;
    let _e19: Dipole = self_413;
    let _e23: Flector = other_355;
    let _e35: Dipole = self_413;
    let _e39: Flector = other_355;
    return FlatPoint(((((vec4<f32>(_e4.g1_.y) * _e8.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g1_.z) * _e23.g1_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.x) * _e39.g1_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn dipole_flector_left_contraction(self_414: Dipole, other_356: Flector) -> FlatPoint {
    var self_415: Dipole;
    var other_357: Flector;

    self_415 = self_414;
    other_357 = other_356;
    let _e4: Dipole = self_415;
    let _e8: Flector = other_357;
    let _e19: Dipole = self_415;
    let _e23: Flector = other_357;
    let _e35: Dipole = self_415;
    let _e39: Flector = other_357;
    return FlatPoint(((((vec4<f32>(_e4.g1_.y) * _e8.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g1_.z) * _e23.g1_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.x) * _e39.g1_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn dipole_multi_vector_add(self_416: Dipole, other_358: MultiVector) -> MultiVector {
    var self_417: Dipole;
    var other_359: MultiVector;

    self_417 = self_416;
    other_359 = other_358;
    let _e4: MultiVector = other_359;
    let _e6: MultiVector = other_359;
    let _e8: MultiVector = other_359;
    let _e10: Dipole = self_417;
    let _e12: MultiVector = other_359;
    let _e15: Dipole = self_417;
    let _e17: MultiVector = other_359;
    let _e20: Dipole = self_417;
    let _e22: MultiVector = other_359;
    let _e25: MultiVector = other_359;
    let _e27: MultiVector = other_359;
    let _e29: MultiVector = other_359;
    let _e31: MultiVector = other_359;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, (_e10.g2_ + _e12.g3_), (_e15.g0_ + _e17.g4_), (_e20.g1_ + _e22.g5_), _e25.g6_, _e27.g7_, _e29.g8_, _e31.g9_);
}

fn dipole_multi_vector_sub(self_418: Dipole, other_360: MultiVector) -> MultiVector {
    var self_419: Dipole;
    var other_361: MultiVector;

    self_419 = self_418;
    other_361 = other_360;
    let _e6: MultiVector = other_361;
    let _e11: MultiVector = other_361;
    let _e16: MultiVector = other_361;
    let _e19: Dipole = self_419;
    let _e21: MultiVector = other_361;
    let _e24: Dipole = self_419;
    let _e26: MultiVector = other_361;
    let _e29: Dipole = self_419;
    let _e31: MultiVector = other_361;
    let _e36: MultiVector = other_361;
    let _e41: MultiVector = other_361;
    let _e46: MultiVector = other_361;
    let _e51: MultiVector = other_361;
    return MultiVector((vec3<f32>(0.0) - _e6.g0_), (vec3<f32>(0.0) - _e11.g1_), (vec2<f32>(0.0) - _e16.g2_), (_e19.g2_ - _e21.g3_), (_e24.g0_ - _e26.g4_), (_e29.g1_ - _e31.g5_), (vec3<f32>(0.0) - _e36.g6_), (vec3<f32>(0.0) - _e41.g7_), (vec4<f32>(0.0) - _e46.g8_), (vec4<f32>(0.0) - _e51.g9_));
}

fn dipole_multi_vector_geometric_product(self_420: Dipole, other_362: MultiVector) -> MultiVector {
    var self_421: Dipole;
    var other_363: MultiVector;

    self_421 = self_420;
    other_363 = other_362;
    let _e4: Dipole = self_421;
    let _e8: MultiVector = other_363;
    let _e11: MultiVector = other_363;
    let _e14: MultiVector = other_363;
    let _e26: Dipole = self_421;
    let _e30: MultiVector = other_363;
    let _e33: MultiVector = other_363;
    let _e36: MultiVector = other_363;
    let _e49: Dipole = self_421;
    let _e53: MultiVector = other_363;
    let _e56: MultiVector = other_363;
    let _e59: MultiVector = other_363;
    let _e65: Dipole = self_421;
    let _e69: MultiVector = other_363;
    let _e72: MultiVector = other_363;
    let _e75: MultiVector = other_363;
    let _e81: Dipole = self_421;
    let _e85: MultiVector = other_363;
    let _e88: MultiVector = other_363;
    let _e91: MultiVector = other_363;
    let _e97: Dipole = self_421;
    let _e101: MultiVector = other_363;
    let _e113: Dipole = self_421;
    let _e117: MultiVector = other_363;
    let _e129: Dipole = self_421;
    let _e133: MultiVector = other_363;
    let _e145: Dipole = self_421;
    let _e149: MultiVector = other_363;
    let _e161: Dipole = self_421;
    let _e165: MultiVector = other_363;
    let _e168: MultiVector = other_363;
    let _e171: MultiVector = other_363;
    let _e184: Dipole = self_421;
    let _e188: MultiVector = other_363;
    let _e191: MultiVector = other_363;
    let _e194: MultiVector = other_363;
    let _e205: Dipole = self_421;
    let _e209: MultiVector = other_363;
    let _e212: MultiVector = other_363;
    let _e215: MultiVector = other_363;
    let _e227: Dipole = self_421;
    let _e231: MultiVector = other_363;
    let _e234: MultiVector = other_363;
    let _e237: MultiVector = other_363;
    let _e249: Dipole = self_421;
    let _e253: MultiVector = other_363;
    let _e262: Dipole = self_421;
    let _e266: MultiVector = other_363;
    let _e276: Dipole = self_421;
    let _e280: MultiVector = other_363;
    let _e283: MultiVector = other_363;
    let _e289: Dipole = self_421;
    let _e293: MultiVector = other_363;
    let _e296: MultiVector = other_363;
    let _e302: Dipole = self_421;
    let _e306: MultiVector = other_363;
    let _e309: MultiVector = other_363;
    let _e315: Dipole = self_421;
    let _e319: MultiVector = other_363;
    let _e330: Dipole = self_421;
    let _e334: MultiVector = other_363;
    let _e345: Dipole = self_421;
    let _e349: MultiVector = other_363;
    let _e360: Dipole = self_421;
    let _e364: MultiVector = other_363;
    let _e374: Dipole = self_421;
    let _e378: MultiVector = other_363;
    let _e389: Dipole = self_421;
    let _e393: MultiVector = other_363;
    let _e405: Dipole = self_421;
    let _e409: MultiVector = other_363;
    let _e412: MultiVector = other_363;
    let _e415: MultiVector = other_363;
    let _e418: MultiVector = other_363;
    let _e432: Dipole = self_421;
    let _e436: MultiVector = other_363;
    let _e439: MultiVector = other_363;
    let _e442: MultiVector = other_363;
    let _e445: MultiVector = other_363;
    let _e459: Dipole = self_421;
    let _e463: MultiVector = other_363;
    let _e466: MultiVector = other_363;
    let _e469: MultiVector = other_363;
    let _e472: MultiVector = other_363;
    let _e486: Dipole = self_421;
    let _e490: MultiVector = other_363;
    let _e493: MultiVector = other_363;
    let _e496: MultiVector = other_363;
    let _e499: MultiVector = other_363;
    let _e513: Dipole = self_421;
    let _e517: MultiVector = other_363;
    let _e520: MultiVector = other_363;
    let _e523: MultiVector = other_363;
    let _e526: MultiVector = other_363;
    let _e540: Dipole = self_421;
    let _e544: MultiVector = other_363;
    let _e547: MultiVector = other_363;
    let _e550: MultiVector = other_363;
    let _e553: MultiVector = other_363;
    let _e567: Dipole = self_421;
    let _e571: MultiVector = other_363;
    let _e583: Dipole = self_421;
    let _e587: MultiVector = other_363;
    let _e599: Dipole = self_421;
    let _e603: MultiVector = other_363;
    let _e606: MultiVector = other_363;
    let _e609: MultiVector = other_363;
    let _e620: Dipole = self_421;
    let _e624: MultiVector = other_363;
    let _e627: MultiVector = other_363;
    let _e630: MultiVector = other_363;
    let _e642: Dipole = self_421;
    let _e646: MultiVector = other_363;
    let _e649: MultiVector = other_363;
    let _e652: MultiVector = other_363;
    let _e664: Dipole = self_421;
    let _e668: MultiVector = other_363;
    let _e671: MultiVector = other_363;
    let _e674: MultiVector = other_363;
    let _e686: Dipole = self_421;
    let _e690: MultiVector = other_363;
    let _e693: MultiVector = other_363;
    let _e696: MultiVector = other_363;
    let _e708: Dipole = self_421;
    let _e712: MultiVector = other_363;
    let _e715: MultiVector = other_363;
    let _e718: MultiVector = other_363;
    let _e730: Dipole = self_421;
    let _e734: MultiVector = other_363;
    let _e737: MultiVector = other_363;
    let _e740: MultiVector = other_363;
    let _e751: Dipole = self_421;
    let _e755: MultiVector = other_363;
    let _e758: MultiVector = other_363;
    let _e761: MultiVector = other_363;
    let _e773: Dipole = self_421;
    let _e777: MultiVector = other_363;
    let _e780: MultiVector = other_363;
    let _e783: MultiVector = other_363;
    let _e795: Dipole = self_421;
    let _e799: MultiVector = other_363;
    let _e802: MultiVector = other_363;
    let _e805: MultiVector = other_363;
    let _e816: Dipole = self_421;
    let _e820: MultiVector = other_363;
    let _e823: MultiVector = other_363;
    let _e826: MultiVector = other_363;
    let _e838: Dipole = self_421;
    let _e842: MultiVector = other_363;
    let _e845: MultiVector = other_363;
    let _e848: MultiVector = other_363;
    let _e860: Dipole = self_421;
    let _e864: MultiVector = other_363;
    let _e867: MultiVector = other_363;
    let _e870: MultiVector = other_363;
    let _e882: Dipole = self_421;
    let _e886: MultiVector = other_363;
    let _e889: MultiVector = other_363;
    let _e892: MultiVector = other_363;
    let _e904: Dipole = self_421;
    let _e908: MultiVector = other_363;
    let _e911: MultiVector = other_363;
    let _e914: MultiVector = other_363;
    let _e926: Dipole = self_421;
    let _e930: MultiVector = other_363;
    let _e933: MultiVector = other_363;
    let _e936: MultiVector = other_363;
    let _e948: Dipole = self_421;
    let _e952: MultiVector = other_363;
    let _e955: MultiVector = other_363;
    let _e958: MultiVector = other_363;
    let _e970: Dipole = self_421;
    let _e974: MultiVector = other_363;
    let _e977: MultiVector = other_363;
    let _e980: MultiVector = other_363;
    let _e992: Dipole = self_421;
    let _e996: MultiVector = other_363;
    let _e1000: Dipole = self_421;
    let _e1004: MultiVector = other_363;
    let _e1007: MultiVector = other_363;
    let _e1010: MultiVector = other_363;
    let _e1021: Dipole = self_421;
    let _e1025: MultiVector = other_363;
    let _e1028: MultiVector = other_363;
    let _e1031: MultiVector = other_363;
    let _e1043: Dipole = self_421;
    let _e1047: MultiVector = other_363;
    let _e1050: MultiVector = other_363;
    let _e1053: MultiVector = other_363;
    let _e1065: Dipole = self_421;
    let _e1069: MultiVector = other_363;
    let _e1072: MultiVector = other_363;
    let _e1075: MultiVector = other_363;
    let _e1087: Dipole = self_421;
    let _e1091: MultiVector = other_363;
    let _e1094: MultiVector = other_363;
    let _e1097: MultiVector = other_363;
    let _e1109: Dipole = self_421;
    let _e1113: MultiVector = other_363;
    let _e1116: MultiVector = other_363;
    let _e1119: MultiVector = other_363;
    let _e1131: Dipole = self_421;
    let _e1135: MultiVector = other_363;
    let _e1138: MultiVector = other_363;
    let _e1141: MultiVector = other_363;
    let _e1144: MultiVector = other_363;
    let _e1157: Dipole = self_421;
    let _e1161: MultiVector = other_363;
    let _e1164: MultiVector = other_363;
    let _e1167: MultiVector = other_363;
    let _e1170: MultiVector = other_363;
    let _e1184: Dipole = self_421;
    let _e1188: MultiVector = other_363;
    let _e1191: MultiVector = other_363;
    let _e1194: MultiVector = other_363;
    let _e1197: MultiVector = other_363;
    let _e1211: Dipole = self_421;
    let _e1215: MultiVector = other_363;
    let _e1218: MultiVector = other_363;
    let _e1221: MultiVector = other_363;
    let _e1224: MultiVector = other_363;
    let _e1238: Dipole = self_421;
    let _e1242: MultiVector = other_363;
    let _e1245: MultiVector = other_363;
    let _e1248: MultiVector = other_363;
    let _e1251: MultiVector = other_363;
    let _e1265: Dipole = self_421;
    let _e1269: MultiVector = other_363;
    let _e1272: MultiVector = other_363;
    let _e1275: MultiVector = other_363;
    let _e1278: MultiVector = other_363;
    let _e1292: Dipole = self_421;
    let _e1296: MultiVector = other_363;
    let _e1299: MultiVector = other_363;
    let _e1302: MultiVector = other_363;
    let _e1305: MultiVector = other_363;
    let _e1318: Dipole = self_421;
    let _e1322: MultiVector = other_363;
    let _e1325: MultiVector = other_363;
    let _e1328: MultiVector = other_363;
    let _e1331: MultiVector = other_363;
    let _e1345: Dipole = self_421;
    let _e1349: MultiVector = other_363;
    let _e1352: MultiVector = other_363;
    let _e1355: MultiVector = other_363;
    let _e1358: MultiVector = other_363;
    let _e1372: Dipole = self_421;
    let _e1376: MultiVector = other_363;
    let _e1379: MultiVector = other_363;
    let _e1382: MultiVector = other_363;
    let _e1385: MultiVector = other_363;
    let _e1399: Dipole = self_421;
    let _e1403: MultiVector = other_363;
    let _e1406: MultiVector = other_363;
    let _e1409: MultiVector = other_363;
    let _e1412: MultiVector = other_363;
    let _e1426: Dipole = self_421;
    let _e1430: MultiVector = other_363;
    let _e1433: MultiVector = other_363;
    let _e1436: MultiVector = other_363;
    let _e1439: MultiVector = other_363;
    let _e1453: Dipole = self_421;
    let _e1457: MultiVector = other_363;
    let _e1460: MultiVector = other_363;
    let _e1463: MultiVector = other_363;
    let _e1466: MultiVector = other_363;
    let _e1480: Dipole = self_421;
    let _e1484: MultiVector = other_363;
    let _e1487: MultiVector = other_363;
    let _e1490: MultiVector = other_363;
    let _e1493: MultiVector = other_363;
    let _e1507: Dipole = self_421;
    let _e1511: MultiVector = other_363;
    let _e1514: MultiVector = other_363;
    let _e1517: MultiVector = other_363;
    let _e1520: MultiVector = other_363;
    let _e1532: Dipole = self_421;
    let _e1536: MultiVector = other_363;
    let _e1539: MultiVector = other_363;
    let _e1542: MultiVector = other_363;
    let _e1545: MultiVector = other_363;
    return MultiVector((((((((((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g5_.y, _e11.g5_.y, _e14.g7_.y)) * vec3<f32>(0.0, -(1.0), -(1.0))) + ((vec3<f32>(_e26.g0_.z) * vec3<f32>(_e30.g5_.z, _e33.g5_.z, _e36.g7_.z)) * vec3<f32>(0.0, -(1.0), -(1.0)))) - (vec3<f32>(_e49.g1_.x) * vec3<f32>(_e53.g5_.x, _e56.g4_.x, _e59.g6_.x))) - (vec3<f32>(_e65.g1_.y) * vec3<f32>(_e69.g5_.y, _e72.g4_.y, _e75.g6_.y))) - (vec3<f32>(_e81.g1_.z) * vec3<f32>(_e85.g5_.z, _e88.g4_.z, _e91.g6_.z))) + ((vec3<f32>(_e97.g2_.x) * vec3<f32>(_e101.g8_.x)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e113.g2_.y) * vec3<f32>(_e117.g8_.y)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e129.g2_.z) * vec3<f32>(_e133.g8_.z)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e145.g2_.w) * vec3<f32>(_e149.g8_.w)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e161.g0_.x) * vec3<f32>(_e165.g5_.x, _e168.g5_.x, _e171.g7_.x)) * vec3<f32>(0.0, -(1.0), -(1.0)))), ((((vec3<f32>(_e184.g1_.x) * vec3<f32>(_e188.g8_.w, _e191.g1_.z, _e194.g1_.y)) * vec3<f32>(1.0, 1.0, -(1.0))) + ((vec3<f32>(_e205.g1_.y) * vec3<f32>(_e209.g1_.z, _e212.g8_.w, _e215.g1_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e227.g1_.z) * vec3<f32>(_e231.g1_.y, _e234.g1_.x, _e237.g8_.w)) * vec3<f32>(1.0, -(1.0), 1.0))), ((((((((((vec2<f32>(_e249.g0_.y) * vec2<f32>(_e253.g1_.y)) * vec2<f32>(1.0, 0.0)) + ((vec2<f32>(_e262.g0_.z) * vec2<f32>(_e266.g1_.z)) * vec2<f32>(1.0, 0.0))) - (vec2<f32>(_e276.g1_.x) * vec2<f32>(_e280.g8_.x, _e283.g7_.x))) - (vec2<f32>(_e289.g1_.y) * vec2<f32>(_e293.g8_.y, _e296.g7_.y))) - (vec2<f32>(_e302.g1_.z) * vec2<f32>(_e306.g8_.z, _e309.g7_.z))) + ((vec2<f32>(_e315.g2_.x) * vec2<f32>(_e319.g1_.x)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e330.g2_.y) * vec2<f32>(_e334.g1_.y)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e345.g2_.z) * vec2<f32>(_e349.g1_.z)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e360.g0_.x) * vec2<f32>(_e364.g1_.x)) * vec2<f32>(1.0, 0.0))), (((((((((((vec4<f32>(_e374.g0_.y) * vec4<f32>(_e378.g3_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + ((vec4<f32>(_e389.g0_.z) * vec4<f32>(_e393.g3_.z)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e405.g1_.x) * vec4<f32>(_e409.g9_.w, _e412.g3_.z, _e415.g3_.y, _e418.g9_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e432.g1_.y) * vec4<f32>(_e436.g3_.z, _e439.g9_.w, _e442.g3_.x, _e445.g9_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e459.g1_.z) * vec4<f32>(_e463.g3_.y, _e466.g3_.x, _e469.g9_.w, _e472.g9_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e486.g2_.x) * vec4<f32>(_e490.g0_.x, _e493.g5_.z, _e496.g5_.y, _e499.g4_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e513.g2_.y) * vec4<f32>(_e517.g5_.z, _e520.g0_.x, _e523.g5_.x, _e526.g4_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e540.g2_.z) * vec4<f32>(_e544.g5_.y, _e547.g5_.x, _e550.g0_.x, _e553.g4_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e567.g2_.w) * vec4<f32>(_e571.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e583.g0_.x) * vec4<f32>(_e587.g3_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))), (((((((vec3<f32>(_e599.g0_.x) * vec3<f32>(_e603.g0_.x, _e606.g5_.z, _e609.g5_.y)) * vec3<f32>(1.0, 1.0, -(1.0))) + ((vec3<f32>(_e620.g0_.y) * vec3<f32>(_e624.g5_.z, _e627.g0_.x, _e630.g5_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e642.g0_.z) * vec3<f32>(_e646.g5_.y, _e649.g5_.x, _e652.g0_.x)) * vec3<f32>(1.0, -(1.0), 1.0))) + ((vec3<f32>(_e664.g1_.x) * vec3<f32>(_e668.g0_.y, _e671.g4_.z, _e674.g4_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e686.g1_.y) * vec3<f32>(_e690.g4_.z, _e693.g0_.y, _e696.g4_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e708.g1_.z) * vec3<f32>(_e712.g4_.y, _e715.g4_.x, _e718.g0_.y)) * vec3<f32>(1.0, -(1.0), 1.0))), ((((vec3<f32>(_e730.g1_.x) * vec3<f32>(_e734.g0_.x, _e737.g5_.z, _e740.g5_.y)) * vec3<f32>(1.0, 1.0, -(1.0))) + ((vec3<f32>(_e751.g1_.y) * vec3<f32>(_e755.g5_.z, _e758.g0_.x, _e761.g5_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e773.g1_.z) * vec3<f32>(_e777.g5_.y, _e780.g5_.x, _e783.g0_.x)) * vec3<f32>(1.0, -(1.0), 1.0))), (((((((((((vec3<f32>(_e795.g0_.x) * vec3<f32>(_e799.g2_.y, _e802.g7_.z, _e805.g7_.y)) * vec3<f32>(1.0, 1.0, -(1.0))) + ((vec3<f32>(_e816.g0_.y) * vec3<f32>(_e820.g7_.z, _e823.g2_.y, _e826.g7_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e838.g0_.z) * vec3<f32>(_e842.g7_.y, _e845.g7_.x, _e848.g2_.y)) * vec3<f32>(1.0, -(1.0), 1.0))) + ((vec3<f32>(_e860.g1_.x) * vec3<f32>(_e864.g0_.z, _e867.g6_.z, _e870.g6_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e882.g1_.y) * vec3<f32>(_e886.g6_.z, _e889.g0_.z, _e892.g6_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e904.g1_.z) * vec3<f32>(_e908.g6_.y, _e911.g6_.x, _e914.g0_.z)) * vec3<f32>(1.0, -(1.0), 1.0))) + ((vec3<f32>(_e926.g2_.x) * vec3<f32>(_e930.g2_.x, _e933.g8_.z, _e936.g8_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e948.g2_.y) * vec3<f32>(_e952.g8_.z, _e955.g2_.x, _e958.g8_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e970.g2_.z) * vec3<f32>(_e974.g8_.y, _e977.g8_.x, _e980.g2_.x)) * vec3<f32>(1.0, -(1.0), 1.0))) - (vec3<f32>(_e992.g2_.w) * _e996.g1_)), (((((((vec3<f32>(_e1000.g1_.x) * vec3<f32>(_e1004.g2_.y, _e1007.g7_.z, _e1010.g7_.y)) * vec3<f32>(1.0, 1.0, -(1.0))) + ((vec3<f32>(_e1021.g1_.y) * vec3<f32>(_e1025.g7_.z, _e1028.g2_.y, _e1031.g7_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e1043.g1_.z) * vec3<f32>(_e1047.g7_.y, _e1050.g7_.x, _e1053.g2_.y)) * vec3<f32>(1.0, -(1.0), 1.0))) + ((vec3<f32>(_e1065.g2_.x) * vec3<f32>(_e1069.g8_.w, _e1072.g1_.z, _e1075.g1_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e1087.g2_.y) * vec3<f32>(_e1091.g1_.z, _e1094.g8_.w, _e1097.g1_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e1109.g2_.z) * vec3<f32>(_e1113.g1_.y, _e1116.g1_.x, _e1119.g8_.w)) * vec3<f32>(1.0, -(1.0), 1.0))), (((((((vec4<f32>(_e1131.g0_.y) * vec4<f32>(_e1135.g1_.z, _e1138.g8_.w, _e1141.g1_.x, _e1144.g1_.z)) * vec4<f32>(1.0, -(1.0), -(1.0), 0.0)) + ((vec4<f32>(_e1157.g0_.z) * vec4<f32>(_e1161.g1_.y, _e1164.g1_.x, _e1167.g8_.w, _e1170.g1_.y)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e1184.g1_.x) * vec4<f32>(_e1188.g2_.x, _e1191.g8_.z, _e1194.g8_.y, _e1197.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e1211.g1_.y) * vec4<f32>(_e1215.g8_.z, _e1218.g2_.x, _e1221.g8_.x, _e1224.g1_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e1238.g1_.z) * vec4<f32>(_e1242.g8_.y, _e1245.g8_.x, _e1248.g2_.x, _e1251.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e1265.g0_.x) * vec4<f32>(_e1269.g8_.w, _e1272.g1_.z, _e1275.g1_.y, _e1278.g8_.x)) * vec4<f32>(-(1.0), -(1.0), 1.0, 0.0))), (((((((((((vec4<f32>(_e1292.g0_.y) * vec4<f32>(_e1296.g3_.z, _e1299.g9_.w, _e1302.g3_.x, _e1305.g3_.z)) * vec4<f32>(1.0, -(1.0), -(1.0), 0.0)) + ((vec4<f32>(_e1318.g0_.z) * vec4<f32>(_e1322.g3_.y, _e1325.g3_.x, _e1328.g9_.w, _e1331.g3_.y)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e1345.g1_.x) * vec4<f32>(_e1349.g3_.w, _e1352.g9_.z, _e1355.g9_.y, _e1358.g3_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e1372.g1_.y) * vec4<f32>(_e1376.g9_.z, _e1379.g3_.w, _e1382.g9_.x, _e1385.g3_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e1399.g1_.z) * vec4<f32>(_e1403.g9_.y, _e1406.g9_.x, _e1409.g3_.w, _e1412.g3_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e1426.g2_.x) * vec4<f32>(_e1430.g0_.y, _e1433.g4_.z, _e1436.g4_.y, _e1439.g5_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e1453.g2_.y) * vec4<f32>(_e1457.g4_.z, _e1460.g0_.y, _e1463.g4_.x, _e1466.g5_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e1480.g2_.z) * vec4<f32>(_e1484.g4_.y, _e1487.g4_.x, _e1490.g0_.y, _e1493.g5_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e1507.g2_.w) * vec4<f32>(_e1511.g5_.x, _e1514.g5_.y, _e1517.g5_.z, _e1520.g5_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e1532.g0_.x) * vec4<f32>(_e1536.g9_.w, _e1539.g3_.z, _e1542.g3_.y, _e1545.g9_.x)) * vec4<f32>(-(1.0), -(1.0), 1.0, 0.0))));
}

fn dipole_multi_vector_scalar_product(self_422: Dipole, other_364: MultiVector) -> Scalar {
    var self_423: Dipole;
    var other_365: MultiVector;

    self_423 = self_422;
    other_365 = other_364;
    let _e5: Dipole = self_423;
    let _e8: MultiVector = other_365;
    let _e13: Dipole = self_423;
    let _e16: MultiVector = other_365;
    let _e21: Dipole = self_423;
    let _e24: MultiVector = other_365;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g5_.x)) - (_e13.g1_.y * _e16.g5_.y)) - (_e21.g1_.z * _e24.g5_.z)));
}

fn dipole_squared_magnitude(self_424: Dipole) -> Scalar {
    var self_425: Dipole;

    self_425 = self_424;
    let _e4: Dipole = self_425;
    let _e5: Dipole = dipole_reversal(_e4);
    let _e6: Dipole = self_425;
    let _e8: Dipole = self_425;
    let _e9: Dipole = dipole_reversal(_e8);
    let _e10: Scalar = dipole_dipole_scalar_product(_e6, _e9);
    return _e10;
}

fn dipole_magnitude(self_426: Dipole) -> Scalar {
    var self_427: Dipole;

    self_427 = self_426;
    let _e3: Dipole = self_427;
    let _e4: Scalar = dipole_squared_magnitude(_e3);
    let _e7: Dipole = self_427;
    let _e8: Scalar = dipole_squared_magnitude(_e7);
    return Scalar(sqrt(_e8.g0_));
}

fn dipole_scale(self_428: Dipole, other_366: f32) -> Dipole {
    var self_429: Dipole;
    var other_367: f32;

    self_429 = self_428;
    other_367 = other_366;
    let _e5: f32 = other_367;
    let _e7: Dipole = self_429;
    let _e8: f32 = other_367;
    let _e10: Dipole = dipole_scalar_geometric_product(_e7, Scalar(_e8));
    return _e10;
}

fn dipole_signum(self_430: Dipole) -> Dipole {
    var self_431: Dipole;

    self_431 = self_430;
    let _e5: Dipole = self_431;
    let _e6: Scalar = dipole_magnitude(_e5);
    let _e10: Dipole = self_431;
    let _e13: Dipole = self_431;
    let _e14: Scalar = dipole_magnitude(_e13);
    let _e18: Dipole = dipole_scalar_geometric_product(_e10, Scalar((1.0 / _e14.g0_)));
    return _e18;
}

fn dipole_inverse(self_432: Dipole) -> Dipole {
    var self_433: Dipole;

    self_433 = self_432;
    let _e3: Dipole = self_433;
    let _e4: Dipole = dipole_reversal(_e3);
    let _e7: Dipole = self_433;
    let _e8: Scalar = dipole_squared_magnitude(_e7);
    let _e13: Dipole = self_433;
    let _e14: Dipole = dipole_reversal(_e13);
    let _e17: Dipole = self_433;
    let _e18: Scalar = dipole_squared_magnitude(_e17);
    let _e22: Dipole = dipole_scalar_geometric_product(_e14, Scalar((1.0 / _e18.g0_)));
    return _e22;
}

fn line_zero() -> Line {
    return Line(vec3<f32>(0.0), vec3<f32>(0.0));
}

fn line_one() -> Line {
    return Line(vec3<f32>(0.0), vec3<f32>(0.0));
}

fn line_neg(self_434: Line) -> Line {
    var self_435: Line;

    self_435 = self_434;
    let _e2: Line = self_435;
    let _e8: Line = self_435;
    return Line((_e2.g0_ * vec3<f32>(-(1.0))), (_e8.g1_ * vec3<f32>(-(1.0))));
}

fn line_automorphism(self_436: Line) -> Line {
    var self_437: Line;

    self_437 = self_436;
    let _e2: Line = self_437;
    let _e8: Line = self_437;
    return Line((_e2.g0_ * vec3<f32>(-(1.0))), (_e8.g1_ * vec3<f32>(-(1.0))));
}

fn line_reversal(self_438: Line) -> Line {
    var self_439: Line;

    self_439 = self_438;
    let _e2: Line = self_439;
    let _e8: Line = self_439;
    return Line((_e2.g0_ * vec3<f32>(-(1.0))), (_e8.g1_ * vec3<f32>(-(1.0))));
}

fn line_conjugation(self_440: Line) -> Line {
    var self_441: Line;

    self_441 = self_440;
    let _e2: Line = self_441;
    let _e4: Line = self_441;
    return Line(_e2.g0_, _e4.g1_);
}

fn line_scalar_geometric_product(self_442: Line, other_368: Scalar) -> Line {
    var self_443: Line;
    var other_369: Scalar;

    self_443 = self_442;
    other_369 = other_368;
    let _e4: Line = self_443;
    let _e6: Scalar = other_369;
    let _e10: Line = self_443;
    let _e12: Scalar = other_369;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn line_scalar_outer_product(self_444: Line, other_370: Scalar) -> Line {
    var self_445: Line;
    var other_371: Scalar;

    self_445 = self_444;
    other_371 = other_370;
    let _e4: Line = self_445;
    let _e6: Scalar = other_371;
    let _e10: Line = self_445;
    let _e12: Scalar = other_371;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn line_scalar_inner_product(self_446: Line, other_372: Scalar) -> Line {
    var self_447: Line;
    var other_373: Scalar;

    self_447 = self_446;
    other_373 = other_372;
    let _e4: Line = self_447;
    let _e6: Scalar = other_373;
    let _e10: Line = self_447;
    let _e12: Scalar = other_373;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn line_scalar_right_contraction(self_448: Line, other_374: Scalar) -> Line {
    var self_449: Line;
    var other_375: Scalar;

    self_449 = self_448;
    other_375 = other_374;
    let _e4: Line = self_449;
    let _e6: Scalar = other_375;
    let _e10: Line = self_449;
    let _e12: Scalar = other_375;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn line_radial_point_geometric_product(self_450: Line, other_376: RadialPoint) -> Flector {
    var self_451: Line;
    var other_377: RadialPoint;

    self_451 = self_450;
    other_377 = other_376;
    let _e4: Line = self_451;
    let _e8: RadialPoint = other_377;
    let _e20: Line = self_451;
    let _e24: RadialPoint = other_377;
    let _e37: Line = self_451;
    let _e41: RadialPoint = other_377;
    let _e44: RadialPoint = other_377;
    let _e47: RadialPoint = other_377;
    let _e50: RadialPoint = other_377;
    let _e63: Line = self_451;
    let _e67: RadialPoint = other_377;
    let _e70: RadialPoint = other_377;
    let _e73: RadialPoint = other_377;
    let _e76: RadialPoint = other_377;
    let _e89: Line = self_451;
    let _e92: Line = self_451;
    let _e95: Line = self_451;
    let _e98: Line = self_451;
    let _e102: RadialPoint = other_377;
    let _e105: RadialPoint = other_377;
    let _e108: RadialPoint = other_377;
    let _e111: RadialPoint = other_377;
    let _e125: Line = self_451;
    let _e129: RadialPoint = other_377;
    let _e132: RadialPoint = other_377;
    let _e135: RadialPoint = other_377;
    let _e138: RadialPoint = other_377;
    let _e150: Line = self_451;
    let _e154: RadialPoint = other_377;
    let _e157: RadialPoint = other_377;
    let _e160: RadialPoint = other_377;
    let _e163: RadialPoint = other_377;
    let _e176: Line = self_451;
    let _e180: RadialPoint = other_377;
    let _e183: RadialPoint = other_377;
    let _e186: RadialPoint = other_377;
    let _e189: RadialPoint = other_377;
    let _e202: Line = self_451;
    let _e206: RadialPoint = other_377;
    let _e209: RadialPoint = other_377;
    let _e212: RadialPoint = other_377;
    let _e215: RadialPoint = other_377;
    let _e228: Line = self_451;
    let _e232: RadialPoint = other_377;
    let _e235: RadialPoint = other_377;
    let _e238: RadialPoint = other_377;
    let _e241: RadialPoint = other_377;
    let _e254: Line = self_451;
    let _e258: RadialPoint = other_377;
    let _e261: RadialPoint = other_377;
    let _e264: RadialPoint = other_377;
    let _e267: RadialPoint = other_377;
    return Flector(((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e37.g1_.y) * vec4<f32>(_e41.g0_.z, _e44.g0_.z, _e47.g0_.x, _e50.g0_.z)) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e63.g1_.z) * vec4<f32>(_e67.g0_.y, _e70.g0_.x, _e73.g0_.y, _e76.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e89.g0_.x, _e92.g1_.x, _e95.g1_.x, _e98.g0_.x) * vec4<f32>(_e102.g0_.x, _e105.g0_.z, _e108.g0_.y, _e111.g0_.x)) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))), (((((((vec4<f32>(_e125.g0_.y) * vec4<f32>(_e129.g0_.z, _e132.g0_.z, _e135.g0_.x, _e138.g0_.z)) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e150.g0_.z) * vec4<f32>(_e154.g0_.y, _e157.g0_.x, _e160.g0_.y, _e163.g0_.y)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e176.g1_.x) * vec4<f32>(_e180.g1_.x, _e183.g1_.x, _e186.g1_.x, _e189.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e202.g1_.y) * vec4<f32>(_e206.g1_.x, _e209.g1_.x, _e212.g1_.x, _e215.g0_.y)) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e228.g1_.z) * vec4<f32>(_e232.g1_.x, _e235.g1_.x, _e238.g1_.x, _e241.g0_.z)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e254.g0_.x) * vec4<f32>(_e258.g0_.x, _e261.g0_.z, _e264.g0_.y, _e267.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))));
}

fn line_radial_point_outer_product(self_452: Line, other_378: RadialPoint) -> Plane {
    var self_453: Line;
    var other_379: RadialPoint;

    self_453 = self_452;
    other_379 = other_378;
    let _e4: Line = self_453;
    let _e8: RadialPoint = other_379;
    let _e11: RadialPoint = other_379;
    let _e14: RadialPoint = other_379;
    let _e17: RadialPoint = other_379;
    let _e29: Line = self_453;
    let _e33: RadialPoint = other_379;
    let _e36: RadialPoint = other_379;
    let _e39: RadialPoint = other_379;
    let _e42: RadialPoint = other_379;
    let _e55: Line = self_453;
    let _e59: RadialPoint = other_379;
    let _e62: RadialPoint = other_379;
    let _e65: RadialPoint = other_379;
    let _e68: RadialPoint = other_379;
    let _e81: Line = self_453;
    let _e85: RadialPoint = other_379;
    let _e88: RadialPoint = other_379;
    let _e91: RadialPoint = other_379;
    let _e94: RadialPoint = other_379;
    let _e107: Line = self_453;
    let _e111: RadialPoint = other_379;
    let _e114: RadialPoint = other_379;
    let _e117: RadialPoint = other_379;
    let _e120: RadialPoint = other_379;
    let _e133: Line = self_453;
    let _e137: RadialPoint = other_379;
    let _e140: RadialPoint = other_379;
    let _e143: RadialPoint = other_379;
    let _e146: RadialPoint = other_379;
    return Plane((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g0_.z)) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.x, _e39.g0_.y, _e42.g0_.y)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e55.g1_.x) * vec4<f32>(_e59.g1_.x, _e62.g1_.x, _e65.g1_.x, _e68.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e81.g1_.y) * vec4<f32>(_e85.g1_.x, _e88.g1_.x, _e91.g1_.x, _e94.g0_.y)) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e107.g1_.z) * vec4<f32>(_e111.g1_.x, _e114.g1_.x, _e117.g1_.x, _e120.g0_.z)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e133.g0_.x) * vec4<f32>(_e137.g0_.x, _e140.g0_.z, _e143.g0_.y, _e146.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))));
}

fn line_radial_point_inner_product(self_454: Line, other_380: RadialPoint) -> FlatPoint {
    var self_455: Line;
    var other_381: RadialPoint;

    self_455 = self_454;
    other_381 = other_380;
    let _e4: Line = self_455;
    let _e8: RadialPoint = other_381;
    let _e20: Line = self_455;
    let _e24: RadialPoint = other_381;
    let _e37: Line = self_455;
    let _e41: RadialPoint = other_381;
    let _e44: RadialPoint = other_381;
    let _e47: RadialPoint = other_381;
    let _e50: RadialPoint = other_381;
    let _e63: Line = self_455;
    let _e67: RadialPoint = other_381;
    let _e70: RadialPoint = other_381;
    let _e73: RadialPoint = other_381;
    let _e76: RadialPoint = other_381;
    let _e89: Line = self_455;
    let _e92: Line = self_455;
    let _e95: Line = self_455;
    let _e98: Line = self_455;
    let _e102: RadialPoint = other_381;
    let _e105: RadialPoint = other_381;
    let _e108: RadialPoint = other_381;
    let _e111: RadialPoint = other_381;
    return FlatPoint(((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e37.g1_.y) * vec4<f32>(_e41.g0_.z, _e44.g0_.z, _e47.g0_.x, _e50.g0_.z)) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e63.g1_.z) * vec4<f32>(_e67.g0_.y, _e70.g0_.x, _e73.g0_.y, _e76.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e89.g0_.x, _e92.g1_.x, _e95.g1_.x, _e98.g0_.x) * vec4<f32>(_e102.g0_.x, _e105.g0_.z, _e108.g0_.y, _e111.g0_.x)) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))));
}

fn line_radial_point_right_contraction(self_456: Line, other_382: RadialPoint) -> FlatPoint {
    var self_457: Line;
    var other_383: RadialPoint;

    self_457 = self_456;
    other_383 = other_382;
    let _e4: Line = self_457;
    let _e8: RadialPoint = other_383;
    let _e20: Line = self_457;
    let _e24: RadialPoint = other_383;
    let _e37: Line = self_457;
    let _e41: RadialPoint = other_383;
    let _e44: RadialPoint = other_383;
    let _e47: RadialPoint = other_383;
    let _e50: RadialPoint = other_383;
    let _e63: Line = self_457;
    let _e67: RadialPoint = other_383;
    let _e70: RadialPoint = other_383;
    let _e73: RadialPoint = other_383;
    let _e76: RadialPoint = other_383;
    let _e89: Line = self_457;
    let _e92: Line = self_457;
    let _e95: Line = self_457;
    let _e98: Line = self_457;
    let _e102: RadialPoint = other_383;
    let _e105: RadialPoint = other_383;
    let _e108: RadialPoint = other_383;
    let _e111: RadialPoint = other_383;
    return FlatPoint(((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e37.g1_.y) * vec4<f32>(_e41.g0_.z, _e44.g0_.z, _e47.g0_.x, _e50.g0_.z)) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e63.g1_.z) * vec4<f32>(_e67.g0_.y, _e70.g0_.x, _e73.g0_.y, _e76.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e89.g0_.x, _e92.g1_.x, _e95.g1_.x, _e98.g0_.x) * vec4<f32>(_e102.g0_.x, _e105.g0_.z, _e108.g0_.y, _e111.g0_.x)) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))));
}

fn line_dipole_geometric_product(self_458: Line, other_384: Dipole) -> Motor {
    var self_459: Line;
    var other_385: Dipole;

    self_459 = self_458;
    other_385 = other_384;
    let _e4: Line = self_459;
    let _e8: Dipole = other_385;
    let _e11: Dipole = other_385;
    let _e14: Dipole = other_385;
    let _e17: Dipole = other_385;
    let _e30: Line = self_459;
    let _e34: Dipole = other_385;
    let _e37: Dipole = other_385;
    let _e40: Dipole = other_385;
    let _e43: Dipole = other_385;
    let _e57: Line = self_459;
    let _e61: Dipole = other_385;
    let _e64: Dipole = other_385;
    let _e67: Dipole = other_385;
    let _e70: Dipole = other_385;
    let _e84: Line = self_459;
    let _e88: Dipole = other_385;
    let _e91: Dipole = other_385;
    let _e94: Dipole = other_385;
    let _e97: Dipole = other_385;
    let _e111: Line = self_459;
    let _e115: Dipole = other_385;
    let _e118: Dipole = other_385;
    let _e121: Dipole = other_385;
    let _e124: Dipole = other_385;
    let _e138: Line = self_459;
    let _e142: Dipole = other_385;
    let _e145: Dipole = other_385;
    let _e148: Dipole = other_385;
    let _e151: Dipole = other_385;
    let _e165: Line = self_459;
    let _e169: Dipole = other_385;
    let _e172: Dipole = other_385;
    let _e175: Dipole = other_385;
    let _e178: Dipole = other_385;
    let _e191: Line = self_459;
    let _e195: Dipole = other_385;
    let _e198: Dipole = other_385;
    let _e201: Dipole = other_385;
    let _e204: Dipole = other_385;
    let _e218: Line = self_459;
    let _e222: Dipole = other_385;
    let _e225: Dipole = other_385;
    let _e228: Dipole = other_385;
    let _e231: Dipole = other_385;
    return Motor((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g1_.x) * vec4<f32>(_e61.g0_.z, _e64.g0_.z, _e67.g0_.y, _e70.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e84.g1_.y) * vec4<f32>(_e88.g0_.z, _e91.g0_.z, _e94.g0_.x, _e97.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e111.g1_.z) * vec4<f32>(_e115.g0_.y, _e118.g0_.x, _e121.g0_.y, _e124.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e138.g0_.x) * vec4<f32>(_e142.g1_.x, _e145.g1_.z, _e148.g1_.y, _e151.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), ((((vec4<f32>(_e165.g1_.y) * vec4<f32>(_e169.g1_.z, _e172.g1_.z, _e175.g1_.x, _e178.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e191.g1_.z) * vec4<f32>(_e195.g1_.y, _e198.g1_.x, _e201.g1_.y, _e204.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e218.g1_.x) * vec4<f32>(_e222.g1_.x, _e225.g1_.z, _e228.g1_.y, _e231.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn line_dipole_outer_product(self_460: Line, other_386: Dipole) -> AntiScalar {
    var self_461: Line;
    var other_387: Dipole;

    self_461 = self_460;
    other_387 = other_386;
    let _e5: Line = self_461;
    let _e8: Dipole = other_387;
    let _e13: Line = self_461;
    let _e16: Dipole = other_387;
    let _e21: Line = self_461;
    let _e24: Dipole = other_387;
    let _e29: Line = self_461;
    let _e32: Dipole = other_387;
    let _e37: Line = self_461;
    let _e40: Dipole = other_387;
    let _e45: Line = self_461;
    let _e48: Dipole = other_387;
    return AntiScalar(((((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)) - (_e29.g1_.x * _e32.g0_.x)) - (_e37.g1_.y * _e40.g0_.y)) - (_e45.g1_.z * _e48.g0_.z)));
}

fn line_line_add(self_462: Line, other_388: Line) -> Line {
    var self_463: Line;
    var other_389: Line;

    self_463 = self_462;
    other_389 = other_388;
    let _e4: Line = self_463;
    let _e6: Line = other_389;
    let _e9: Line = self_463;
    let _e11: Line = other_389;
    return Line((_e4.g0_ + _e6.g0_), (_e9.g1_ + _e11.g1_));
}

fn line_line_sub(self_464: Line, other_390: Line) -> Line {
    var self_465: Line;
    var other_391: Line;

    self_465 = self_464;
    other_391 = other_390;
    let _e4: Line = self_465;
    let _e6: Line = other_391;
    let _e9: Line = self_465;
    let _e11: Line = other_391;
    return Line((_e4.g0_ - _e6.g0_), (_e9.g1_ - _e11.g1_));
}

fn line_line_mul(self_466: Line, other_392: Line) -> Line {
    var self_467: Line;
    var other_393: Line;

    self_467 = self_466;
    other_393 = other_392;
    let _e4: Line = self_467;
    let _e6: Line = other_393;
    let _e9: Line = self_467;
    let _e11: Line = other_393;
    return Line((_e4.g0_ * _e6.g0_), (_e9.g1_ * _e11.g1_));
}

fn line_line_div(self_468: Line, other_394: Line) -> Line {
    var self_469: Line;
    var other_395: Line;

    self_469 = self_468;
    other_395 = other_394;
    let _e4: Line = self_469;
    let _e7: Line = self_469;
    let _e10: Line = self_469;
    let _e19: Line = other_395;
    let _e22: Line = other_395;
    let _e25: Line = other_395;
    let _e35: Line = self_469;
    let _e38: Line = self_469;
    let _e41: Line = self_469;
    let _e50: Line = other_395;
    let _e53: Line = other_395;
    let _e56: Line = other_395;
    return Line((((vec3<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e19.g0_.x, _e22.g0_.y, _e25.g0_.z)) * vec3<f32>(1.0, 1.0, 1.0)), (((vec3<f32>(_e35.g1_.x, _e38.g1_.y, _e41.g1_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e50.g1_.x, _e53.g1_.y, _e56.g1_.z)) * vec3<f32>(1.0, 1.0, 1.0)));
}

fn line_circle_add(self_470: Line, other_396: Circle) -> Circle {
    var self_471: Line;
    var other_397: Circle;

    self_471 = self_470;
    other_397 = other_396;
    let _e4: Circle = other_397;
    let _e6: Line = self_471;
    let _e8: Circle = other_397;
    let _e11: Line = self_471;
    let _e13: Circle = other_397;
    return Circle(_e4.g0_, (_e6.g0_ + _e8.g1_), (_e11.g1_ + _e13.g2_));
}

fn line_circle_sub(self_472: Line, other_398: Circle) -> Circle {
    var self_473: Line;
    var other_399: Circle;

    self_473 = self_472;
    other_399 = other_398;
    let _e6: Circle = other_399;
    let _e9: Line = self_473;
    let _e11: Circle = other_399;
    let _e14: Line = self_473;
    let _e16: Circle = other_399;
    return Circle((vec4<f32>(0.0) - _e6.g0_), (_e9.g0_ - _e11.g1_), (_e14.g1_ - _e16.g2_));
}

fn line_motor_add(self_474: Line, other_400: Motor) -> Motor {
    var self_475: Line;
    var other_401: Motor;

    self_475 = self_474;
    other_401 = other_400;
    let _e4: Line = self_475;
    let _e7: Line = self_475;
    let _e10: Line = self_475;
    let _e13: Line = self_475;
    let _e23: Motor = other_401;
    let _e26: Line = self_475;
    let _e29: Line = self_475;
    let _e32: Line = self_475;
    let _e35: Line = self_475;
    let _e45: Motor = other_401;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) + _e23.g0_), ((vec4<f32>(_e26.g1_.x, _e29.g1_.y, _e32.g1_.z, _e35.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) + _e45.g1_));
}

fn line_motor_sub(self_476: Line, other_402: Motor) -> Motor {
    var self_477: Line;
    var other_403: Motor;

    self_477 = self_476;
    other_403 = other_402;
    let _e4: Line = self_477;
    let _e7: Line = self_477;
    let _e10: Line = self_477;
    let _e13: Line = self_477;
    let _e23: Motor = other_403;
    let _e26: Line = self_477;
    let _e29: Line = self_477;
    let _e32: Line = self_477;
    let _e35: Line = self_477;
    let _e45: Motor = other_403;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) - _e23.g0_), ((vec4<f32>(_e26.g1_.x, _e29.g1_.y, _e32.g1_.z, _e35.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) - _e45.g1_));
}

fn line_multi_vector_add(self_478: Line, other_404: MultiVector) -> MultiVector {
    var self_479: Line;
    var other_405: MultiVector;

    self_479 = self_478;
    other_405 = other_404;
    let _e4: MultiVector = other_405;
    let _e6: MultiVector = other_405;
    let _e8: MultiVector = other_405;
    let _e10: MultiVector = other_405;
    let _e12: MultiVector = other_405;
    let _e14: MultiVector = other_405;
    let _e16: Line = self_479;
    let _e18: MultiVector = other_405;
    let _e21: Line = self_479;
    let _e23: MultiVector = other_405;
    let _e26: MultiVector = other_405;
    let _e28: MultiVector = other_405;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, _e10.g3_, _e12.g4_, _e14.g5_, (_e16.g0_ + _e18.g6_), (_e21.g1_ + _e23.g7_), _e26.g8_, _e28.g9_);
}

fn line_multi_vector_sub(self_480: Line, other_406: MultiVector) -> MultiVector {
    var self_481: Line;
    var other_407: MultiVector;

    self_481 = self_480;
    other_407 = other_406;
    let _e6: MultiVector = other_407;
    let _e11: MultiVector = other_407;
    let _e16: MultiVector = other_407;
    let _e21: MultiVector = other_407;
    let _e26: MultiVector = other_407;
    let _e31: MultiVector = other_407;
    let _e34: Line = self_481;
    let _e36: MultiVector = other_407;
    let _e39: Line = self_481;
    let _e41: MultiVector = other_407;
    let _e46: MultiVector = other_407;
    let _e51: MultiVector = other_407;
    return MultiVector((vec3<f32>(0.0) - _e6.g0_), (vec3<f32>(0.0) - _e11.g1_), (vec2<f32>(0.0) - _e16.g2_), (vec4<f32>(0.0) - _e21.g3_), (vec3<f32>(0.0) - _e26.g4_), (vec3<f32>(0.0) - _e31.g5_), (_e34.g0_ - _e36.g6_), (_e39.g1_ - _e41.g7_), (vec4<f32>(0.0) - _e46.g8_), (vec4<f32>(0.0) - _e51.g9_));
}

fn line_scale(self_482: Line, other_408: f32) -> Line {
    var self_483: Line;
    var other_409: f32;

    self_483 = self_482;
    other_409 = other_408;
    let _e5: f32 = other_409;
    let _e7: Line = self_483;
    let _e8: f32 = other_409;
    let _e10: Line = line_scalar_geometric_product(_e7, Scalar(_e8));
    return _e10;
}

fn circle_zero() -> Circle {
    return Circle(vec4<f32>(0.0), vec3<f32>(0.0), vec3<f32>(0.0));
}

fn circle_one() -> Circle {
    return Circle(vec4<f32>(0.0), vec3<f32>(0.0), vec3<f32>(0.0));
}

fn circle_neg(self_484: Circle) -> Circle {
    var self_485: Circle;

    self_485 = self_484;
    let _e2: Circle = self_485;
    let _e8: Circle = self_485;
    let _e14: Circle = self_485;
    return Circle((_e2.g0_ * vec4<f32>(-(1.0))), (_e8.g1_ * vec3<f32>(-(1.0))), (_e14.g2_ * vec3<f32>(-(1.0))));
}

fn circle_automorphism(self_486: Circle) -> Circle {
    var self_487: Circle;

    self_487 = self_486;
    let _e2: Circle = self_487;
    let _e8: Circle = self_487;
    let _e14: Circle = self_487;
    return Circle((_e2.g0_ * vec4<f32>(-(1.0))), (_e8.g1_ * vec3<f32>(-(1.0))), (_e14.g2_ * vec3<f32>(-(1.0))));
}

fn circle_reversal(self_488: Circle) -> Circle {
    var self_489: Circle;

    self_489 = self_488;
    let _e2: Circle = self_489;
    let _e8: Circle = self_489;
    let _e14: Circle = self_489;
    return Circle((_e2.g0_ * vec4<f32>(-(1.0))), (_e8.g1_ * vec3<f32>(-(1.0))), (_e14.g2_ * vec3<f32>(-(1.0))));
}

fn circle_conjugation(self_490: Circle) -> Circle {
    var self_491: Circle;

    self_491 = self_490;
    let _e2: Circle = self_491;
    let _e4: Circle = self_491;
    let _e6: Circle = self_491;
    return Circle(_e2.g0_, _e4.g1_, _e6.g2_);
}

fn circle_scalar_geometric_product(self_492: Circle, other_410: Scalar) -> Circle {
    var self_493: Circle;
    var other_411: Scalar;

    self_493 = self_492;
    other_411 = other_410;
    let _e4: Circle = self_493;
    let _e6: Scalar = other_411;
    let _e10: Circle = self_493;
    let _e12: Scalar = other_411;
    let _e16: Circle = self_493;
    let _e18: Scalar = other_411;
    return Circle((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)), (_e16.g2_ * vec3<f32>(_e18.g0_)));
}

fn circle_scalar_outer_product(self_494: Circle, other_412: Scalar) -> Circle {
    var self_495: Circle;
    var other_413: Scalar;

    self_495 = self_494;
    other_413 = other_412;
    let _e4: Circle = self_495;
    let _e6: Scalar = other_413;
    let _e10: Circle = self_495;
    let _e12: Scalar = other_413;
    let _e16: Circle = self_495;
    let _e18: Scalar = other_413;
    return Circle((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)), (_e16.g2_ * vec3<f32>(_e18.g0_)));
}

fn circle_scalar_inner_product(self_496: Circle, other_414: Scalar) -> Circle {
    var self_497: Circle;
    var other_415: Scalar;

    self_497 = self_496;
    other_415 = other_414;
    let _e4: Circle = self_497;
    let _e6: Scalar = other_415;
    let _e10: Circle = self_497;
    let _e12: Scalar = other_415;
    let _e16: Circle = self_497;
    let _e18: Scalar = other_415;
    return Circle((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)), (_e16.g2_ * vec3<f32>(_e18.g0_)));
}

fn circle_scalar_right_contraction(self_498: Circle, other_416: Scalar) -> Circle {
    var self_499: Circle;
    var other_417: Scalar;

    self_499 = self_498;
    other_417 = other_416;
    let _e4: Circle = self_499;
    let _e6: Scalar = other_417;
    let _e10: Circle = self_499;
    let _e12: Scalar = other_417;
    let _e16: Circle = self_499;
    let _e18: Scalar = other_417;
    return Circle((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)), (_e16.g2_ * vec3<f32>(_e18.g0_)));
}

fn circle_radial_point_outer_product(self_500: Circle, other_418: RadialPoint) -> Sphere {
    var self_501: Circle;
    var other_419: RadialPoint;

    self_501 = self_500;
    other_419 = other_418;
    let _e5: Circle = self_501;
    let _e8: RadialPoint = other_419;
    let _e13: Circle = self_501;
    let _e16: RadialPoint = other_419;
    let _e21: Circle = self_501;
    let _e24: RadialPoint = other_419;
    let _e29: Circle = self_501;
    let _e32: RadialPoint = other_419;
    let _e37: Circle = self_501;
    let _e41: RadialPoint = other_419;
    let _e44: RadialPoint = other_419;
    let _e47: RadialPoint = other_419;
    let _e50: RadialPoint = other_419;
    let _e62: Circle = self_501;
    let _e66: RadialPoint = other_419;
    let _e69: RadialPoint = other_419;
    let _e72: RadialPoint = other_419;
    let _e75: RadialPoint = other_419;
    let _e88: Circle = self_501;
    let _e92: RadialPoint = other_419;
    let _e95: RadialPoint = other_419;
    let _e98: RadialPoint = other_419;
    let _e101: RadialPoint = other_419;
    let _e114: Circle = self_501;
    let _e118: RadialPoint = other_419;
    let _e121: RadialPoint = other_419;
    let _e124: RadialPoint = other_419;
    let _e127: RadialPoint = other_419;
    let _e140: Circle = self_501;
    let _e144: RadialPoint = other_419;
    let _e147: RadialPoint = other_419;
    let _e150: RadialPoint = other_419;
    let _e153: RadialPoint = other_419;
    let _e166: Circle = self_501;
    let _e170: RadialPoint = other_419;
    let _e173: RadialPoint = other_419;
    let _e176: RadialPoint = other_419;
    let _e179: RadialPoint = other_419;
    let _e192: Circle = self_501;
    let _e194: RadialPoint = other_419;
    return Sphere(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g0_.w * _e32.g1_.x)), ((((((((vec4<f32>(_e37.g1_.x) * vec4<f32>(_e41.g0_.z, _e44.g0_.z, _e47.g0_.y, _e50.g0_.z)) * vec4<f32>(0.0, 1.0, -(1.0), 0.0)) + ((vec4<f32>(_e62.g1_.y) * vec4<f32>(_e66.g0_.z, _e69.g0_.z, _e72.g0_.x, _e75.g0_.z)) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e88.g1_.z) * vec4<f32>(_e92.g0_.y, _e95.g0_.x, _e98.g0_.y, _e101.g0_.y)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e114.g2_.x) * vec4<f32>(_e118.g1_.x, _e121.g1_.x, _e124.g1_.x, _e127.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e140.g2_.y) * vec4<f32>(_e144.g1_.x, _e147.g1_.x, _e150.g1_.x, _e153.g0_.y)) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e166.g2_.z) * vec4<f32>(_e170.g1_.x, _e173.g1_.x, _e176.g1_.x, _e179.g0_.z)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + (_e192.g0_ * vec4<f32>(_e194.g1_.y))));
}

fn circle_radial_point_inner_product(self_502: Circle, other_420: RadialPoint) -> Dipole {
    var self_503: Circle;
    var other_421: RadialPoint;

    self_503 = self_502;
    other_421 = other_420;
    let _e4: Circle = self_503;
    let _e8: RadialPoint = other_421;
    let _e18: Circle = self_503;
    let _e22: RadialPoint = other_421;
    let _e33: Circle = self_503;
    let _e37: RadialPoint = other_421;
    let _e50: Circle = self_503;
    let _e54: RadialPoint = other_421;
    let _e58: Circle = self_503;
    let _e62: RadialPoint = other_421;
    let _e74: Circle = self_503;
    let _e78: RadialPoint = other_421;
    let _e91: Circle = self_503;
    let _e95: RadialPoint = other_421;
    let _e98: RadialPoint = other_421;
    let _e101: RadialPoint = other_421;
    let _e104: RadialPoint = other_421;
    let _e117: Circle = self_503;
    let _e121: RadialPoint = other_421;
    let _e124: RadialPoint = other_421;
    let _e127: RadialPoint = other_421;
    let _e130: RadialPoint = other_421;
    let _e143: Circle = self_503;
    let _e146: Circle = self_503;
    let _e149: Circle = self_503;
    let _e152: Circle = self_503;
    let _e156: RadialPoint = other_421;
    let _e159: RadialPoint = other_421;
    let _e162: RadialPoint = other_421;
    let _e165: RadialPoint = other_421;
    return Dipole(((((vec3<f32>(_e4.g0_.y) * _e8.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e18.g0_.z) * _e22.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e33.g0_.x) * _e37.g0_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))), (vec3<f32>(0.0) - (vec3<f32>(_e50.g0_.w) * _e54.g0_)), ((((((vec4<f32>(_e58.g1_.y) * vec4<f32>(_e62.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e74.g1_.z) * vec4<f32>(_e78.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e91.g2_.y) * vec4<f32>(_e95.g0_.z, _e98.g0_.z, _e101.g0_.x, _e104.g0_.z)) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e117.g2_.z) * vec4<f32>(_e121.g0_.y, _e124.g0_.x, _e127.g0_.y, _e130.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e143.g1_.x, _e146.g2_.x, _e149.g2_.x, _e152.g1_.x) * vec4<f32>(_e156.g0_.x, _e159.g0_.z, _e162.g0_.y, _e165.g0_.x)) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))));
}

fn circle_radial_point_right_contraction(self_504: Circle, other_422: RadialPoint) -> Dipole {
    var self_505: Circle;
    var other_423: RadialPoint;

    self_505 = self_504;
    other_423 = other_422;
    let _e4: Circle = self_505;
    let _e8: RadialPoint = other_423;
    let _e18: Circle = self_505;
    let _e22: RadialPoint = other_423;
    let _e33: Circle = self_505;
    let _e37: RadialPoint = other_423;
    let _e50: Circle = self_505;
    let _e54: RadialPoint = other_423;
    let _e58: Circle = self_505;
    let _e62: RadialPoint = other_423;
    let _e74: Circle = self_505;
    let _e78: RadialPoint = other_423;
    let _e91: Circle = self_505;
    let _e95: RadialPoint = other_423;
    let _e98: RadialPoint = other_423;
    let _e101: RadialPoint = other_423;
    let _e104: RadialPoint = other_423;
    let _e117: Circle = self_505;
    let _e121: RadialPoint = other_423;
    let _e124: RadialPoint = other_423;
    let _e127: RadialPoint = other_423;
    let _e130: RadialPoint = other_423;
    let _e143: Circle = self_505;
    let _e146: Circle = self_505;
    let _e149: Circle = self_505;
    let _e152: Circle = self_505;
    let _e156: RadialPoint = other_423;
    let _e159: RadialPoint = other_423;
    let _e162: RadialPoint = other_423;
    let _e165: RadialPoint = other_423;
    return Dipole(((((vec3<f32>(_e4.g0_.y) * _e8.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e18.g0_.z) * _e22.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e33.g0_.x) * _e37.g0_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))), (vec3<f32>(0.0) - (vec3<f32>(_e50.g0_.w) * _e54.g0_)), ((((((vec4<f32>(_e58.g1_.y) * vec4<f32>(_e62.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e74.g1_.z) * vec4<f32>(_e78.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e91.g2_.y) * vec4<f32>(_e95.g0_.z, _e98.g0_.z, _e101.g0_.x, _e104.g0_.z)) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e117.g2_.z) * vec4<f32>(_e121.g0_.y, _e124.g0_.x, _e127.g0_.y, _e130.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e143.g1_.x, _e146.g2_.x, _e149.g2_.x, _e152.g1_.x) * vec4<f32>(_e156.g0_.x, _e159.g0_.z, _e162.g0_.y, _e165.g0_.x)) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))));
}

fn circle_flat_point_outer_product(self_506: Circle, other_424: FlatPoint) -> AntiScalar {
    var self_507: Circle;
    var other_425: FlatPoint;

    self_507 = self_506;
    other_425 = other_424;
    let _e5: Circle = self_507;
    let _e8: FlatPoint = other_425;
    let _e13: Circle = self_507;
    let _e16: FlatPoint = other_425;
    let _e21: Circle = self_507;
    let _e24: FlatPoint = other_425;
    let _e29: Circle = self_507;
    let _e32: FlatPoint = other_425;
    return AntiScalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g0_.w * _e32.g0_.w)));
}

fn circle_dipole_outer_product(self_508: Circle, other_426: Dipole) -> AntiScalar {
    var self_509: Circle;
    var other_427: Dipole;

    self_509 = self_508;
    other_427 = other_426;
    let _e5: Circle = self_509;
    let _e8: Dipole = other_427;
    let _e13: Circle = self_509;
    let _e16: Dipole = other_427;
    let _e21: Circle = self_509;
    let _e24: Dipole = other_427;
    let _e29: Circle = self_509;
    let _e32: Dipole = other_427;
    let _e37: Circle = self_509;
    let _e40: Dipole = other_427;
    let _e45: Circle = self_509;
    let _e48: Dipole = other_427;
    let _e53: Circle = self_509;
    let _e56: Dipole = other_427;
    let _e61: Circle = self_509;
    let _e64: Dipole = other_427;
    let _e69: Circle = self_509;
    let _e72: Dipole = other_427;
    let _e77: Circle = self_509;
    let _e80: Dipole = other_427;
    return AntiScalar(((((((((((0.0 - (_e5.g0_.x * _e8.g2_.x)) - (_e13.g0_.y * _e16.g2_.y)) - (_e21.g0_.z * _e24.g2_.z)) - (_e29.g0_.w * _e32.g2_.w)) - (_e37.g1_.x * _e40.g1_.x)) - (_e45.g1_.y * _e48.g1_.y)) - (_e53.g1_.z * _e56.g1_.z)) - (_e61.g2_.x * _e64.g0_.x)) - (_e69.g2_.y * _e72.g0_.y)) - (_e77.g2_.z * _e80.g0_.z)));
}

fn circle_dipole_inner_product(self_510: Circle, other_428: Dipole) -> RadialPoint {
    var self_511: Circle;
    var other_429: Dipole;

    self_511 = self_510;
    other_429 = other_428;
    let _e4: Circle = self_511;
    let _e8: Dipole = other_429;
    let _e11: Circle = self_511;
    let _e15: Dipole = other_429;
    let _e25: Circle = self_511;
    let _e29: Dipole = other_429;
    let _e40: Circle = self_511;
    let _e44: Dipole = other_429;
    let _e55: Circle = self_511;
    let _e59: Dipole = other_429;
    let _e70: Circle = self_511;
    let _e74: Dipole = other_429;
    let _e85: Circle = self_511;
    let _e89: Dipole = other_429;
    return RadialPoint((vec3<f32>(_e4.g0_.w) * _e8.g1_), (((((((vec2<f32>(_e11.g0_.y) * vec2<f32>(_e15.g1_.y)) * vec2<f32>(-(1.0), 0.0)) + ((vec2<f32>(_e25.g0_.z) * vec2<f32>(_e29.g1_.z)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e40.g2_.x) * vec2<f32>(_e44.g1_.x)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e55.g2_.y) * vec2<f32>(_e59.g1_.y)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e70.g2_.z) * vec2<f32>(_e74.g1_.z)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e85.g0_.x) * vec2<f32>(_e89.g1_.x)) * vec2<f32>(-(1.0), 0.0))));
}

fn circle_dipole_right_contraction(self_512: Circle, other_430: Dipole) -> RadialPoint {
    var self_513: Circle;
    var other_431: Dipole;

    self_513 = self_512;
    other_431 = other_430;
    let _e4: Circle = self_513;
    let _e8: Dipole = other_431;
    let _e11: Circle = self_513;
    let _e15: Dipole = other_431;
    let _e25: Circle = self_513;
    let _e29: Dipole = other_431;
    let _e40: Circle = self_513;
    let _e44: Dipole = other_431;
    let _e55: Circle = self_513;
    let _e59: Dipole = other_431;
    let _e70: Circle = self_513;
    let _e74: Dipole = other_431;
    let _e85: Circle = self_513;
    let _e89: Dipole = other_431;
    return RadialPoint((vec3<f32>(_e4.g0_.w) * _e8.g1_), (((((((vec2<f32>(_e11.g0_.y) * vec2<f32>(_e15.g1_.y)) * vec2<f32>(-(1.0), 0.0)) + ((vec2<f32>(_e25.g0_.z) * vec2<f32>(_e29.g1_.z)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e40.g2_.x) * vec2<f32>(_e44.g1_.x)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e55.g2_.y) * vec2<f32>(_e59.g1_.y)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e70.g2_.z) * vec2<f32>(_e74.g1_.z)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e85.g0_.x) * vec2<f32>(_e89.g1_.x)) * vec2<f32>(-(1.0), 0.0))));
}

fn circle_line_into(self_514: Circle) -> Line {
    var self_515: Circle;

    self_515 = self_514;
    let _e2: Circle = self_515;
    let _e4: Circle = self_515;
    return Line(_e2.g1_, _e4.g2_);
}

fn circle_line_add(self_516: Circle, other_432: Line) -> Circle {
    var self_517: Circle;
    var other_433: Line;

    self_517 = self_516;
    other_433 = other_432;
    let _e4: Circle = self_517;
    let _e6: Circle = self_517;
    let _e8: Line = other_433;
    let _e11: Circle = self_517;
    let _e13: Line = other_433;
    return Circle(_e4.g0_, (_e6.g1_ + _e8.g0_), (_e11.g2_ + _e13.g1_));
}

fn circle_line_sub(self_518: Circle, other_434: Line) -> Circle {
    var self_519: Circle;
    var other_435: Line;

    self_519 = self_518;
    other_435 = other_434;
    let _e4: Circle = self_519;
    let _e6: Circle = self_519;
    let _e8: Line = other_435;
    let _e11: Circle = self_519;
    let _e13: Line = other_435;
    return Circle(_e4.g0_, (_e6.g1_ - _e8.g0_), (_e11.g2_ - _e13.g1_));
}

fn circle_circle_add(self_520: Circle, other_436: Circle) -> Circle {
    var self_521: Circle;
    var other_437: Circle;

    self_521 = self_520;
    other_437 = other_436;
    let _e4: Circle = self_521;
    let _e6: Circle = other_437;
    let _e9: Circle = self_521;
    let _e11: Circle = other_437;
    let _e14: Circle = self_521;
    let _e16: Circle = other_437;
    return Circle((_e4.g0_ + _e6.g0_), (_e9.g1_ + _e11.g1_), (_e14.g2_ + _e16.g2_));
}

fn circle_circle_sub(self_522: Circle, other_438: Circle) -> Circle {
    var self_523: Circle;
    var other_439: Circle;

    self_523 = self_522;
    other_439 = other_438;
    let _e4: Circle = self_523;
    let _e6: Circle = other_439;
    let _e9: Circle = self_523;
    let _e11: Circle = other_439;
    let _e14: Circle = self_523;
    let _e16: Circle = other_439;
    return Circle((_e4.g0_ - _e6.g0_), (_e9.g1_ - _e11.g1_), (_e14.g2_ - _e16.g2_));
}

fn circle_circle_mul(self_524: Circle, other_440: Circle) -> Circle {
    var self_525: Circle;
    var other_441: Circle;

    self_525 = self_524;
    other_441 = other_440;
    let _e4: Circle = self_525;
    let _e6: Circle = other_441;
    let _e9: Circle = self_525;
    let _e11: Circle = other_441;
    let _e14: Circle = self_525;
    let _e16: Circle = other_441;
    return Circle((_e4.g0_ * _e6.g0_), (_e9.g1_ * _e11.g1_), (_e14.g2_ * _e16.g2_));
}

fn circle_circle_div(self_526: Circle, other_442: Circle) -> Circle {
    var self_527: Circle;
    var other_443: Circle;

    self_527 = self_526;
    other_443 = other_442;
    let _e4: Circle = self_527;
    let _e7: Circle = self_527;
    let _e10: Circle = self_527;
    let _e13: Circle = self_527;
    let _e23: Circle = other_443;
    let _e26: Circle = other_443;
    let _e29: Circle = other_443;
    let _e32: Circle = other_443;
    let _e43: Circle = self_527;
    let _e46: Circle = self_527;
    let _e49: Circle = self_527;
    let _e58: Circle = other_443;
    let _e61: Circle = other_443;
    let _e64: Circle = other_443;
    let _e74: Circle = self_527;
    let _e77: Circle = self_527;
    let _e80: Circle = self_527;
    let _e89: Circle = other_443;
    let _e92: Circle = other_443;
    let _e95: Circle = other_443;
    return Circle((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)), (((vec3<f32>(_e43.g1_.x, _e46.g1_.y, _e49.g1_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e58.g1_.x, _e61.g1_.y, _e64.g1_.z)) * vec3<f32>(1.0, 1.0, 1.0)), (((vec3<f32>(_e74.g2_.x, _e77.g2_.y, _e80.g2_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e89.g2_.x, _e92.g2_.y, _e95.g2_.z)) * vec3<f32>(1.0, 1.0, 1.0)));
}

fn circle_circle_inner_product(self_528: Circle, other_444: Circle) -> Scalar {
    var self_529: Circle;
    var other_445: Circle;

    self_529 = self_528;
    other_445 = other_444;
    let _e5: Circle = self_529;
    let _e8: Circle = other_445;
    return Scalar((0.0 - (_e5.g0_.w * _e8.g0_.w)));
}

fn circle_circle_left_contraction(self_530: Circle, other_446: Circle) -> Scalar {
    var self_531: Circle;
    var other_447: Circle;

    self_531 = self_530;
    other_447 = other_446;
    let _e5: Circle = self_531;
    let _e8: Circle = other_447;
    return Scalar((0.0 - (_e5.g0_.w * _e8.g0_.w)));
}

fn circle_circle_right_contraction(self_532: Circle, other_448: Circle) -> Scalar {
    var self_533: Circle;
    var other_449: Circle;

    self_533 = self_532;
    other_449 = other_448;
    let _e5: Circle = self_533;
    let _e8: Circle = other_449;
    return Scalar((0.0 - (_e5.g0_.w * _e8.g0_.w)));
}

fn circle_circle_scalar_product(self_534: Circle, other_450: Circle) -> Scalar {
    var self_535: Circle;
    var other_451: Circle;

    self_535 = self_534;
    other_451 = other_450;
    let _e5: Circle = self_535;
    let _e8: Circle = other_451;
    return Scalar((0.0 - (_e5.g0_.w * _e8.g0_.w)));
}

fn circle_motor_geometric_product(self_536: Circle, other_452: Motor) -> Flector {
    var self_537: Circle;
    var other_453: Motor;

    self_537 = self_536;
    other_453 = other_452;
    let _e4: Circle = self_537;
    let _e8: Motor = other_453;
    let _e20: Circle = self_537;
    let _e24: Motor = other_453;
    let _e37: Circle = self_537;
    let _e41: Motor = other_453;
    let _e44: Motor = other_453;
    let _e47: Motor = other_453;
    let _e50: Motor = other_453;
    let _e56: Circle = self_537;
    let _e60: Motor = other_453;
    let _e73: Circle = self_537;
    let _e77: Motor = other_453;
    let _e88: Circle = self_537;
    let _e92: Motor = other_453;
    let _e104: Circle = self_537;
    let _e108: Motor = other_453;
    let _e111: Motor = other_453;
    let _e114: Motor = other_453;
    let _e117: Motor = other_453;
    let _e123: Circle = self_537;
    let _e127: Motor = other_453;
    return Flector((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + (vec4<f32>(_e37.g0_.w) * vec4<f32>(_e41.g1_.x, _e44.g1_.y, _e47.g1_.z, _e50.g0_.w))) + ((vec4<f32>(_e56.g0_.x) * vec4<f32>(_e60.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))), (((((vec4<f32>(_e73.g0_.y) * _e77.g1_.zwxz) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0)) + ((vec4<f32>(_e88.g0_.z) * _e92.g1_.yxwy) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + (vec4<f32>(_e104.g0_.w) * vec4<f32>(_e108.g0_.x, _e111.g0_.y, _e114.g0_.z, _e117.g1_.w))) + ((vec4<f32>(_e123.g0_.x) * _e127.g1_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))));
}

fn circle_motor_outer_product(self_538: Circle, other_454: Motor) -> Plane {
    var self_539: Circle;
    var other_455: Motor;

    self_539 = self_538;
    other_455 = other_454;
    let _e4: Circle = self_539;
    let _e6: Motor = other_455;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g1_.w)));
}

fn circle_flector_geometric_product(self_540: Circle, other_456: Flector) -> Motor {
    var self_541: Circle;
    var other_457: Flector;

    self_541 = self_540;
    other_457 = other_456;
    let _e4: Circle = self_541;
    let _e8: Flector = other_457;
    let _e11: Flector = other_457;
    let _e14: Flector = other_457;
    let _e17: Flector = other_457;
    let _e30: Circle = self_541;
    let _e34: Flector = other_457;
    let _e37: Flector = other_457;
    let _e40: Flector = other_457;
    let _e43: Flector = other_457;
    let _e57: Circle = self_541;
    let _e61: Flector = other_457;
    let _e64: Flector = other_457;
    let _e67: Flector = other_457;
    let _e70: Flector = other_457;
    let _e84: Circle = self_541;
    let _e88: Flector = other_457;
    let _e91: Flector = other_457;
    let _e94: Flector = other_457;
    let _e97: Flector = other_457;
    let _e105: Circle = self_541;
    let _e109: Flector = other_457;
    let _e112: Flector = other_457;
    let _e115: Flector = other_457;
    let _e118: Flector = other_457;
    return Motor((((((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g1_.w, _e11.g0_.z, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e30.g0_.y) * vec4<f32>(_e34.g0_.z, _e37.g1_.w, _e40.g0_.x, _e43.g0_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.z) * vec4<f32>(_e61.g0_.y, _e64.g0_.x, _e67.g1_.w, _e70.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) - (vec4<f32>(_e84.g0_.w) * vec4<f32>(_e88.g1_.x, _e91.g1_.y, _e94.g1_.z, _e97.g0_.w))), (vec4<f32>(0.0) - (vec4<f32>(_e105.g0_.w) * vec4<f32>(_e109.g0_.x, _e112.g0_.y, _e115.g0_.z, _e118.g1_.w))));
}

fn circle_flector_outer_product(self_542: Circle, other_458: Flector) -> AntiScalar {
    var self_543: Circle;
    var other_459: Flector;

    self_543 = self_542;
    other_459 = other_458;
    let _e5: Circle = self_543;
    let _e8: Flector = other_459;
    let _e13: Circle = self_543;
    let _e16: Flector = other_459;
    let _e21: Circle = self_543;
    let _e24: Flector = other_459;
    let _e29: Circle = self_543;
    let _e32: Flector = other_459;
    return AntiScalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g0_.w * _e32.g0_.w)));
}

fn circle_multi_vector_add(self_544: Circle, other_460: MultiVector) -> MultiVector {
    var self_545: Circle;
    var other_461: MultiVector;

    self_545 = self_544;
    other_461 = other_460;
    let _e4: MultiVector = other_461;
    let _e6: MultiVector = other_461;
    let _e8: MultiVector = other_461;
    let _e10: MultiVector = other_461;
    let _e12: MultiVector = other_461;
    let _e14: MultiVector = other_461;
    let _e16: Circle = self_545;
    let _e18: MultiVector = other_461;
    let _e21: Circle = self_545;
    let _e23: MultiVector = other_461;
    let _e26: Circle = self_545;
    let _e28: MultiVector = other_461;
    let _e31: MultiVector = other_461;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, _e10.g3_, _e12.g4_, _e14.g5_, (_e16.g1_ + _e18.g6_), (_e21.g2_ + _e23.g7_), (_e26.g0_ + _e28.g8_), _e31.g9_);
}

fn circle_multi_vector_sub(self_546: Circle, other_462: MultiVector) -> MultiVector {
    var self_547: Circle;
    var other_463: MultiVector;

    self_547 = self_546;
    other_463 = other_462;
    let _e6: MultiVector = other_463;
    let _e11: MultiVector = other_463;
    let _e16: MultiVector = other_463;
    let _e21: MultiVector = other_463;
    let _e26: MultiVector = other_463;
    let _e31: MultiVector = other_463;
    let _e34: Circle = self_547;
    let _e36: MultiVector = other_463;
    let _e39: Circle = self_547;
    let _e41: MultiVector = other_463;
    let _e44: Circle = self_547;
    let _e46: MultiVector = other_463;
    let _e51: MultiVector = other_463;
    return MultiVector((vec3<f32>(0.0) - _e6.g0_), (vec3<f32>(0.0) - _e11.g1_), (vec2<f32>(0.0) - _e16.g2_), (vec4<f32>(0.0) - _e21.g3_), (vec3<f32>(0.0) - _e26.g4_), (vec3<f32>(0.0) - _e31.g5_), (_e34.g1_ - _e36.g6_), (_e39.g2_ - _e41.g7_), (_e44.g0_ - _e46.g8_), (vec4<f32>(0.0) - _e51.g9_));
}

fn circle_multi_vector_geometric_product(self_548: Circle, other_464: MultiVector) -> MultiVector {
    var self_549: Circle;
    var other_465: MultiVector;

    self_549 = self_548;
    other_465 = other_464;
    let _e4: Circle = self_549;
    let _e8: MultiVector = other_465;
    let _e11: MultiVector = other_465;
    let _e14: MultiVector = other_465;
    let _e26: Circle = self_549;
    let _e30: MultiVector = other_465;
    let _e33: MultiVector = other_465;
    let _e36: MultiVector = other_465;
    let _e49: Circle = self_549;
    let _e53: MultiVector = other_465;
    let _e56: MultiVector = other_465;
    let _e59: MultiVector = other_465;
    let _e65: Circle = self_549;
    let _e69: MultiVector = other_465;
    let _e81: Circle = self_549;
    let _e85: MultiVector = other_465;
    let _e97: Circle = self_549;
    let _e101: MultiVector = other_465;
    let _e113: Circle = self_549;
    let _e117: MultiVector = other_465;
    let _e129: Circle = self_549;
    let _e133: MultiVector = other_465;
    let _e145: Circle = self_549;
    let _e149: MultiVector = other_465;
    let _e161: Circle = self_549;
    let _e165: MultiVector = other_465;
    let _e168: MultiVector = other_465;
    let _e171: MultiVector = other_465;
    let _e184: Circle = self_549;
    let _e188: MultiVector = other_465;
    let _e191: Circle = self_549;
    let _e195: MultiVector = other_465;
    let _e205: Circle = self_549;
    let _e209: MultiVector = other_465;
    let _e220: Circle = self_549;
    let _e224: MultiVector = other_465;
    let _e227: MultiVector = other_465;
    let _e238: Circle = self_549;
    let _e242: MultiVector = other_465;
    let _e253: Circle = self_549;
    let _e257: MultiVector = other_465;
    let _e268: Circle = self_549;
    let _e272: MultiVector = other_465;
    let _e283: Circle = self_549;
    let _e287: MultiVector = other_465;
    let _e298: Circle = self_549;
    let _e302: MultiVector = other_465;
    let _e314: Circle = self_549;
    let _e318: MultiVector = other_465;
    let _e331: Circle = self_549;
    let _e335: MultiVector = other_465;
    let _e338: MultiVector = other_465;
    let _e341: MultiVector = other_465;
    let _e344: MultiVector = other_465;
    let _e350: Circle = self_549;
    let _e354: MultiVector = other_465;
    let _e367: Circle = self_549;
    let _e371: MultiVector = other_465;
    let _e384: Circle = self_549;
    let _e388: MultiVector = other_465;
    let _e401: Circle = self_549;
    let _e405: MultiVector = other_465;
    let _e408: MultiVector = other_465;
    let _e411: MultiVector = other_465;
    let _e414: MultiVector = other_465;
    let _e428: Circle = self_549;
    let _e432: MultiVector = other_465;
    let _e435: MultiVector = other_465;
    let _e438: MultiVector = other_465;
    let _e441: MultiVector = other_465;
    let _e455: Circle = self_549;
    let _e459: MultiVector = other_465;
    let _e462: MultiVector = other_465;
    let _e465: MultiVector = other_465;
    let _e468: MultiVector = other_465;
    let _e482: Circle = self_549;
    let _e486: MultiVector = other_465;
    let _e499: Circle = self_549;
    let _e503: MultiVector = other_465;
    let _e506: MultiVector = other_465;
    let _e509: MultiVector = other_465;
    let _e520: Circle = self_549;
    let _e524: MultiVector = other_465;
    let _e527: MultiVector = other_465;
    let _e530: MultiVector = other_465;
    let _e542: Circle = self_549;
    let _e546: MultiVector = other_465;
    let _e549: MultiVector = other_465;
    let _e552: MultiVector = other_465;
    let _e564: Circle = self_549;
    let _e568: MultiVector = other_465;
    let _e571: MultiVector = other_465;
    let _e574: MultiVector = other_465;
    let _e582: Circle = self_549;
    let _e586: MultiVector = other_465;
    let _e590: Circle = self_549;
    let _e594: MultiVector = other_465;
    let _e597: MultiVector = other_465;
    let _e600: MultiVector = other_465;
    let _e611: Circle = self_549;
    let _e615: MultiVector = other_465;
    let _e618: MultiVector = other_465;
    let _e621: MultiVector = other_465;
    let _e633: Circle = self_549;
    let _e637: MultiVector = other_465;
    let _e640: MultiVector = other_465;
    let _e643: MultiVector = other_465;
    let _e655: Circle = self_549;
    let _e659: MultiVector = other_465;
    let _e662: MultiVector = other_465;
    let _e665: MultiVector = other_465;
    let _e671: Circle = self_549;
    let _e675: MultiVector = other_465;
    let _e678: MultiVector = other_465;
    let _e681: MultiVector = other_465;
    let _e693: Circle = self_549;
    let _e697: MultiVector = other_465;
    let _e700: MultiVector = other_465;
    let _e703: MultiVector = other_465;
    let _e715: Circle = self_549;
    let _e719: MultiVector = other_465;
    let _e722: MultiVector = other_465;
    let _e725: MultiVector = other_465;
    let _e737: Circle = self_549;
    let _e741: MultiVector = other_465;
    let _e744: MultiVector = other_465;
    let _e747: MultiVector = other_465;
    let _e759: Circle = self_549;
    let _e763: MultiVector = other_465;
    let _e766: MultiVector = other_465;
    let _e769: MultiVector = other_465;
    let _e781: Circle = self_549;
    let _e785: MultiVector = other_465;
    let _e788: MultiVector = other_465;
    let _e791: MultiVector = other_465;
    let _e805: Circle = self_549;
    let _e809: MultiVector = other_465;
    let _e812: MultiVector = other_465;
    let _e815: MultiVector = other_465;
    let _e821: Circle = self_549;
    let _e825: MultiVector = other_465;
    let _e828: MultiVector = other_465;
    let _e831: MultiVector = other_465;
    let _e843: Circle = self_549;
    let _e847: MultiVector = other_465;
    let _e850: MultiVector = other_465;
    let _e853: MultiVector = other_465;
    let _e865: Circle = self_549;
    let _e869: MultiVector = other_465;
    let _e872: MultiVector = other_465;
    let _e875: MultiVector = other_465;
    let _e887: Circle = self_549;
    let _e891: MultiVector = other_465;
    let _e894: MultiVector = other_465;
    let _e897: MultiVector = other_465;
    let _e900: MultiVector = other_465;
    let _e912: Circle = self_549;
    let _e916: MultiVector = other_465;
    let _e919: MultiVector = other_465;
    let _e922: MultiVector = other_465;
    let _e925: MultiVector = other_465;
    let _e938: Circle = self_549;
    let _e942: MultiVector = other_465;
    let _e945: MultiVector = other_465;
    let _e948: MultiVector = other_465;
    let _e951: MultiVector = other_465;
    let _e957: Circle = self_549;
    let _e961: MultiVector = other_465;
    let _e964: MultiVector = other_465;
    let _e967: MultiVector = other_465;
    let _e970: MultiVector = other_465;
    let _e983: Circle = self_549;
    let _e987: MultiVector = other_465;
    let _e990: MultiVector = other_465;
    let _e993: MultiVector = other_465;
    let _e996: MultiVector = other_465;
    let _e1008: Circle = self_549;
    let _e1012: MultiVector = other_465;
    let _e1015: MultiVector = other_465;
    let _e1018: MultiVector = other_465;
    let _e1021: MultiVector = other_465;
    let _e1034: Circle = self_549;
    let _e1038: MultiVector = other_465;
    let _e1041: MultiVector = other_465;
    let _e1044: MultiVector = other_465;
    let _e1047: MultiVector = other_465;
    let _e1053: Circle = self_549;
    let _e1057: MultiVector = other_465;
    let _e1060: MultiVector = other_465;
    let _e1063: MultiVector = other_465;
    let _e1066: MultiVector = other_465;
    let _e1079: Circle = self_549;
    let _e1083: MultiVector = other_465;
    let _e1086: MultiVector = other_465;
    let _e1089: MultiVector = other_465;
    let _e1092: MultiVector = other_465;
    let _e1105: Circle = self_549;
    let _e1109: MultiVector = other_465;
    let _e1112: MultiVector = other_465;
    let _e1115: MultiVector = other_465;
    let _e1118: MultiVector = other_465;
    let _e1131: Circle = self_549;
    let _e1135: MultiVector = other_465;
    let _e1138: MultiVector = other_465;
    let _e1141: MultiVector = other_465;
    let _e1144: MultiVector = other_465;
    let _e1158: Circle = self_549;
    let _e1162: MultiVector = other_465;
    let _e1165: MultiVector = other_465;
    let _e1168: MultiVector = other_465;
    let _e1171: MultiVector = other_465;
    let _e1185: Circle = self_549;
    let _e1189: MultiVector = other_465;
    let _e1192: MultiVector = other_465;
    let _e1195: MultiVector = other_465;
    let _e1198: MultiVector = other_465;
    let _e1212: Circle = self_549;
    let _e1216: MultiVector = other_465;
    let _e1219: MultiVector = other_465;
    let _e1222: MultiVector = other_465;
    let _e1225: MultiVector = other_465;
    return MultiVector((((((((((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g1_.y, _e11.g1_.y, _e14.g3_.y)) * vec3<f32>(0.0, -(1.0), -(1.0))) + ((vec3<f32>(_e26.g0_.z) * vec3<f32>(_e30.g1_.z, _e33.g1_.z, _e36.g3_.z)) * vec3<f32>(0.0, -(1.0), -(1.0)))) - (vec3<f32>(_e49.g0_.w) * vec3<f32>(_e53.g8_.w, _e56.g2_.x, _e59.g3_.w))) + ((vec3<f32>(_e65.g1_.x) * vec3<f32>(_e69.g5_.x)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e81.g1_.y) * vec3<f32>(_e85.g5_.y)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e97.g1_.z) * vec3<f32>(_e101.g5_.z)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e113.g2_.x) * vec3<f32>(_e117.g4_.x)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e129.g2_.y) * vec3<f32>(_e133.g4_.y)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e145.g2_.z) * vec3<f32>(_e149.g4_.z)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e161.g0_.x) * vec3<f32>(_e165.g1_.x, _e168.g1_.x, _e171.g3_.x)) * vec3<f32>(0.0, -(1.0), -(1.0)))), (vec3<f32>(_e184.g0_.w) * _e188.g5_), ((((((((vec2<f32>(_e191.g0_.y) * vec2<f32>(_e195.g5_.y)) * vec2<f32>(-(1.0), 0.0)) + ((vec2<f32>(_e205.g0_.z) * vec2<f32>(_e209.g5_.z)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e220.g0_.w) * vec2<f32>(_e224.g0_.y, _e227.g9_.w)) * vec2<f32>(1.0, -(1.0)))) + ((vec2<f32>(_e238.g2_.x) * vec2<f32>(_e242.g5_.x)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e253.g2_.y) * vec2<f32>(_e257.g5_.y)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e268.g2_.z) * vec2<f32>(_e272.g5_.z)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e283.g0_.x) * vec2<f32>(_e287.g5_.x)) * vec2<f32>(-(1.0), 0.0))), (((((((((((vec4<f32>(_e298.g0_.y) * vec4<f32>(_e302.g7_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e314.g0_.z) * vec4<f32>(_e318.g7_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + (vec4<f32>(_e331.g0_.w) * vec4<f32>(_e335.g7_.x, _e338.g7_.y, _e341.g7_.z, _e344.g0_.z))) + ((vec4<f32>(_e350.g1_.x) * vec4<f32>(_e354.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e367.g1_.y) * vec4<f32>(_e371.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e384.g1_.z) * vec4<f32>(_e388.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e401.g2_.x) * vec4<f32>(_e405.g8_.w, _e408.g1_.z, _e411.g1_.y, _e414.g8_.x)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e428.g2_.y) * vec4<f32>(_e432.g1_.z, _e435.g8_.w, _e438.g1_.x, _e441.g8_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e455.g2_.z) * vec4<f32>(_e459.g1_.y, _e462.g1_.x, _e465.g8_.w, _e468.g8_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e482.g0_.x) * vec4<f32>(_e486.g7_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))), (((((vec3<f32>(_e499.g0_.x) * vec3<f32>(_e503.g8_.w, _e506.g1_.z, _e509.g1_.y)) * vec3<f32>(1.0, 1.0, -(1.0))) + ((vec3<f32>(_e520.g0_.y) * vec3<f32>(_e524.g1_.z, _e527.g8_.w, _e530.g1_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e542.g0_.z) * vec3<f32>(_e546.g1_.y, _e549.g1_.x, _e552.g8_.w)) * vec3<f32>(1.0, -(1.0), 1.0))) - (vec3<f32>(_e564.g0_.w) * vec3<f32>(_e568.g8_.x, _e571.g8_.y, _e574.g8_.z))), (vec3<f32>(0.0) - (vec3<f32>(_e582.g0_.w) * _e586.g1_)), (((((((((((vec3<f32>(_e590.g0_.x) * vec3<f32>(_e594.g9_.w, _e597.g3_.z, _e600.g3_.y)) * vec3<f32>(1.0, 1.0, -(1.0))) + ((vec3<f32>(_e611.g0_.y) * vec3<f32>(_e615.g3_.z, _e618.g9_.w, _e621.g3_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e633.g0_.z) * vec3<f32>(_e637.g3_.y, _e640.g3_.x, _e643.g9_.w)) * vec3<f32>(1.0, -(1.0), 1.0))) - (vec3<f32>(_e655.g0_.w) * vec3<f32>(_e659.g9_.x, _e662.g9_.y, _e665.g9_.z))) + ((vec3<f32>(_e671.g1_.x) * vec3<f32>(_e675.g0_.x, _e678.g5_.z, _e681.g5_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e693.g1_.y) * vec3<f32>(_e697.g5_.z, _e700.g0_.x, _e703.g5_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e715.g1_.z) * vec3<f32>(_e719.g5_.y, _e722.g5_.x, _e725.g0_.x)) * vec3<f32>(1.0, -(1.0), 1.0))) + ((vec3<f32>(_e737.g2_.x) * vec3<f32>(_e741.g0_.y, _e744.g4_.z, _e747.g4_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e759.g2_.y) * vec3<f32>(_e763.g4_.z, _e766.g0_.y, _e769.g4_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e781.g2_.z) * vec3<f32>(_e785.g4_.y, _e788.g4_.x, _e791.g0_.y)) * vec3<f32>(1.0, -(1.0), 1.0))), ((((vec3<f32>(0.0) - (vec3<f32>(_e805.g0_.w) * vec3<f32>(_e809.g3_.x, _e812.g3_.y, _e815.g3_.z))) + ((vec3<f32>(_e821.g2_.x) * vec3<f32>(_e825.g0_.x, _e828.g5_.z, _e831.g5_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e843.g2_.y) * vec3<f32>(_e847.g5_.z, _e850.g0_.x, _e853.g5_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e865.g2_.z) * vec3<f32>(_e869.g5_.y, _e872.g5_.x, _e875.g0_.x)) * vec3<f32>(1.0, -(1.0), 1.0))), (((((vec4<f32>(_e887.g0_.y) * vec4<f32>(_e891.g5_.z, _e894.g0_.x, _e897.g5_.x, _e900.g5_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0)) + ((vec4<f32>(_e912.g0_.z) * vec4<f32>(_e916.g5_.y, _e919.g5_.x, _e922.g0_.x, _e925.g5_.y)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + (vec4<f32>(_e938.g0_.w) * vec4<f32>(_e942.g4_.x, _e945.g4_.y, _e948.g4_.z, _e951.g0_.x))) + ((vec4<f32>(_e957.g0_.x) * vec4<f32>(_e961.g0_.x, _e964.g5_.z, _e967.g5_.y, _e970.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))), (((((((((((vec4<f32>(_e983.g0_.y) * vec4<f32>(_e987.g7_.z, _e990.g2_.y, _e993.g7_.x, _e996.g7_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0)) + ((vec4<f32>(_e1008.g0_.z) * vec4<f32>(_e1012.g7_.y, _e1015.g7_.x, _e1018.g2_.y, _e1021.g7_.y)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + (vec4<f32>(_e1034.g0_.w) * vec4<f32>(_e1038.g6_.x, _e1041.g6_.y, _e1044.g6_.z, _e1047.g2_.y))) + ((vec4<f32>(_e1053.g1_.x) * vec4<f32>(_e1057.g8_.w, _e1060.g1_.z, _e1063.g1_.y, _e1066.g8_.w)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e1079.g1_.y) * vec4<f32>(_e1083.g1_.z, _e1086.g8_.w, _e1089.g1_.x, _e1092.g1_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0))) + ((vec4<f32>(_e1105.g1_.z) * vec4<f32>(_e1109.g1_.y, _e1112.g1_.x, _e1115.g8_.w, _e1118.g1_.y)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e1131.g2_.x) * vec4<f32>(_e1135.g2_.x, _e1138.g8_.z, _e1141.g8_.y, _e1144.g1_.x)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e1158.g2_.y) * vec4<f32>(_e1162.g8_.z, _e1165.g2_.x, _e1168.g8_.x, _e1171.g1_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e1185.g2_.z) * vec4<f32>(_e1189.g8_.y, _e1192.g8_.x, _e1195.g2_.x, _e1198.g1_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e1212.g0_.x) * vec4<f32>(_e1216.g2_.y, _e1219.g7_.z, _e1222.g7_.y, _e1225.g2_.x)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))));
}

fn circle_multi_vector_scalar_product(self_550: Circle, other_466: MultiVector) -> Scalar {
    var self_551: Circle;
    var other_467: MultiVector;

    self_551 = self_550;
    other_467 = other_466;
    let _e5: Circle = self_551;
    let _e8: MultiVector = other_467;
    return Scalar((0.0 - (_e5.g0_.w * _e8.g8_.w)));
}

fn circle_squared_magnitude(self_552: Circle) -> Scalar {
    var self_553: Circle;

    self_553 = self_552;
    let _e4: Circle = self_553;
    let _e5: Circle = circle_reversal(_e4);
    let _e6: Circle = self_553;
    let _e8: Circle = self_553;
    let _e9: Circle = circle_reversal(_e8);
    let _e10: Scalar = circle_circle_scalar_product(_e6, _e9);
    return _e10;
}

fn circle_magnitude(self_554: Circle) -> Scalar {
    var self_555: Circle;

    self_555 = self_554;
    let _e3: Circle = self_555;
    let _e4: Scalar = circle_squared_magnitude(_e3);
    let _e7: Circle = self_555;
    let _e8: Scalar = circle_squared_magnitude(_e7);
    return Scalar(sqrt(_e8.g0_));
}

fn circle_scale(self_556: Circle, other_468: f32) -> Circle {
    var self_557: Circle;
    var other_469: f32;

    self_557 = self_556;
    other_469 = other_468;
    let _e5: f32 = other_469;
    let _e7: Circle = self_557;
    let _e8: f32 = other_469;
    let _e10: Circle = circle_scalar_geometric_product(_e7, Scalar(_e8));
    return _e10;
}

fn circle_signum(self_558: Circle) -> Circle {
    var self_559: Circle;

    self_559 = self_558;
    let _e5: Circle = self_559;
    let _e6: Scalar = circle_magnitude(_e5);
    let _e10: Circle = self_559;
    let _e13: Circle = self_559;
    let _e14: Scalar = circle_magnitude(_e13);
    let _e18: Circle = circle_scalar_geometric_product(_e10, Scalar((1.0 / _e14.g0_)));
    return _e18;
}

fn circle_inverse(self_560: Circle) -> Circle {
    var self_561: Circle;

    self_561 = self_560;
    let _e3: Circle = self_561;
    let _e4: Circle = circle_reversal(_e3);
    let _e7: Circle = self_561;
    let _e8: Scalar = circle_squared_magnitude(_e7);
    let _e13: Circle = self_561;
    let _e14: Circle = circle_reversal(_e13);
    let _e17: Circle = self_561;
    let _e18: Scalar = circle_squared_magnitude(_e17);
    let _e22: Circle = circle_scalar_geometric_product(_e14, Scalar((1.0 / _e18.g0_)));
    return _e22;
}

fn plane_zero() -> Plane {
    return Plane(vec4<f32>(0.0));
}

fn plane_one() -> Plane {
    return Plane(vec4<f32>(0.0));
}

fn plane_neg(self_562: Plane) -> Plane {
    var self_563: Plane;

    self_563 = self_562;
    let _e2: Plane = self_563;
    return Plane((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn plane_automorphism(self_564: Plane) -> Plane {
    var self_565: Plane;

    self_565 = self_564;
    let _e2: Plane = self_565;
    return Plane(_e2.g0_);
}

fn plane_reversal(self_566: Plane) -> Plane {
    var self_567: Plane;

    self_567 = self_566;
    let _e2: Plane = self_567;
    return Plane(_e2.g0_);
}

fn plane_conjugation(self_568: Plane) -> Plane {
    var self_569: Plane;

    self_569 = self_568;
    let _e2: Plane = self_569;
    return Plane(_e2.g0_);
}

fn plane_scalar_geometric_product(self_570: Plane, other_470: Scalar) -> Plane {
    var self_571: Plane;
    var other_471: Scalar;

    self_571 = self_570;
    other_471 = other_470;
    let _e4: Plane = self_571;
    let _e6: Scalar = other_471;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn plane_scalar_outer_product(self_572: Plane, other_472: Scalar) -> Plane {
    var self_573: Plane;
    var other_473: Scalar;

    self_573 = self_572;
    other_473 = other_472;
    let _e4: Plane = self_573;
    let _e6: Scalar = other_473;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn plane_scalar_inner_product(self_574: Plane, other_474: Scalar) -> Plane {
    var self_575: Plane;
    var other_475: Scalar;

    self_575 = self_574;
    other_475 = other_474;
    let _e4: Plane = self_575;
    let _e6: Scalar = other_475;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn plane_scalar_right_contraction(self_576: Plane, other_476: Scalar) -> Plane {
    var self_577: Plane;
    var other_477: Scalar;

    self_577 = self_576;
    other_477 = other_476;
    let _e4: Plane = self_577;
    let _e6: Scalar = other_477;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn plane_radial_point_outer_product(self_578: Plane, other_478: RadialPoint) -> AntiScalar {
    var self_579: Plane;
    var other_479: RadialPoint;

    self_579 = self_578;
    other_479 = other_478;
    let _e4: Plane = self_579;
    let _e7: RadialPoint = other_479;
    let _e11: Plane = self_579;
    let _e14: RadialPoint = other_479;
    let _e19: Plane = self_579;
    let _e22: RadialPoint = other_479;
    let _e27: Plane = self_579;
    let _e30: RadialPoint = other_479;
    return AntiScalar(((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)) + (_e27.g0_.w * _e30.g1_.x)));
}

fn plane_radial_point_inner_product(self_580: Plane, other_480: RadialPoint) -> Line {
    var self_581: Plane;
    var other_481: RadialPoint;

    self_581 = self_580;
    other_481 = other_480;
    let _e4: Plane = self_581;
    let _e8: RadialPoint = other_481;
    let _e18: Plane = self_581;
    let _e22: RadialPoint = other_481;
    let _e33: Plane = self_581;
    let _e37: RadialPoint = other_481;
    let _e48: Plane = self_581;
    let _e52: RadialPoint = other_481;
    return Line(((((vec3<f32>(_e4.g0_.y) * _e8.g0_.zzx) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e18.g0_.z) * _e22.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e33.g0_.x) * _e37.g0_.xzy) * vec3<f32>(0.0, -(1.0), 1.0))), (vec3<f32>(_e48.g0_.w) * _e52.g0_));
}

fn plane_radial_point_right_contraction(self_582: Plane, other_482: RadialPoint) -> Line {
    var self_583: Plane;
    var other_483: RadialPoint;

    self_583 = self_582;
    other_483 = other_482;
    let _e4: Plane = self_583;
    let _e8: RadialPoint = other_483;
    let _e18: Plane = self_583;
    let _e22: RadialPoint = other_483;
    let _e33: Plane = self_583;
    let _e37: RadialPoint = other_483;
    let _e48: Plane = self_583;
    let _e52: RadialPoint = other_483;
    return Line(((((vec3<f32>(_e4.g0_.y) * _e8.g0_.zzx) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e18.g0_.z) * _e22.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e33.g0_.x) * _e37.g0_.xzy) * vec3<f32>(0.0, -(1.0), 1.0))), (vec3<f32>(_e48.g0_.w) * _e52.g0_));
}

fn plane_flat_point_add(self_584: Plane, other_484: FlatPoint) -> Flector {
    var self_585: Plane;
    var other_485: FlatPoint;

    self_585 = self_584;
    other_485 = other_484;
    let _e4: FlatPoint = other_485;
    let _e6: Plane = self_585;
    return Flector(_e4.g0_, _e6.g0_);
}

fn plane_flat_point_sub(self_586: Plane, other_486: FlatPoint) -> Flector {
    var self_587: Plane;
    var other_487: FlatPoint;

    self_587 = self_586;
    other_487 = other_486;
    let _e6: FlatPoint = other_487;
    let _e9: Plane = self_587;
    return Flector((vec4<f32>(0.0) - _e6.g0_), _e9.g0_);
}

fn plane_dipole_inner_product(self_588: Plane, other_488: Dipole) -> FlatPoint {
    var self_589: Plane;
    var other_489: Dipole;

    self_589 = self_588;
    other_489 = other_488;
    let _e4: Plane = self_589;
    let _e8: Dipole = other_489;
    let _e20: Plane = self_589;
    let _e24: Dipole = other_489;
    let _e37: Plane = self_589;
    let _e40: Dipole = other_489;
    let _e43: Dipole = other_489;
    let _e46: Dipole = other_489;
    let _e49: Dipole = other_489;
    return FlatPoint(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.wwwx * vec4<f32>(_e40.g1_.x, _e43.g1_.y, _e46.g1_.z, _e49.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn plane_dipole_right_contraction(self_590: Plane, other_490: Dipole) -> FlatPoint {
    var self_591: Plane;
    var other_491: Dipole;

    self_591 = self_590;
    other_491 = other_490;
    let _e4: Plane = self_591;
    let _e8: Dipole = other_491;
    let _e20: Plane = self_591;
    let _e24: Dipole = other_491;
    let _e37: Plane = self_591;
    let _e40: Dipole = other_491;
    let _e43: Dipole = other_491;
    let _e46: Dipole = other_491;
    let _e49: Dipole = other_491;
    return FlatPoint(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.wwwx * vec4<f32>(_e40.g1_.x, _e43.g1_.y, _e46.g1_.z, _e49.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn plane_plane_add(self_592: Plane, other_492: Plane) -> Plane {
    var self_593: Plane;
    var other_493: Plane;

    self_593 = self_592;
    other_493 = other_492;
    let _e4: Plane = self_593;
    let _e6: Plane = other_493;
    return Plane((_e4.g0_ + _e6.g0_));
}

fn plane_plane_sub(self_594: Plane, other_494: Plane) -> Plane {
    var self_595: Plane;
    var other_495: Plane;

    self_595 = self_594;
    other_495 = other_494;
    let _e4: Plane = self_595;
    let _e6: Plane = other_495;
    return Plane((_e4.g0_ - _e6.g0_));
}

fn plane_plane_mul(self_596: Plane, other_496: Plane) -> Plane {
    var self_597: Plane;
    var other_497: Plane;

    self_597 = self_596;
    other_497 = other_496;
    let _e4: Plane = self_597;
    let _e6: Plane = other_497;
    return Plane((_e4.g0_ * _e6.g0_));
}

fn plane_plane_div(self_598: Plane, other_498: Plane) -> Plane {
    var self_599: Plane;
    var other_499: Plane;

    self_599 = self_598;
    other_499 = other_498;
    let _e4: Plane = self_599;
    let _e7: Plane = self_599;
    let _e10: Plane = self_599;
    let _e13: Plane = self_599;
    let _e23: Plane = other_499;
    let _e26: Plane = other_499;
    let _e29: Plane = other_499;
    let _e32: Plane = other_499;
    return Plane((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn plane_sphere_add(self_600: Plane, other_500: Sphere) -> Sphere {
    var self_601: Plane;
    var other_501: Sphere;

    self_601 = self_600;
    other_501 = other_500;
    let _e4: Sphere = other_501;
    let _e6: Plane = self_601;
    let _e8: Sphere = other_501;
    return Sphere(_e4.g0_, (_e6.g0_ + _e8.g1_));
}

fn plane_sphere_sub(self_602: Plane, other_502: Sphere) -> Sphere {
    var self_603: Plane;
    var other_503: Sphere;

    self_603 = self_602;
    other_503 = other_502;
    let _e5: Sphere = other_503;
    let _e8: Plane = self_603;
    let _e10: Sphere = other_503;
    return Sphere((0.0 - _e5.g0_), (_e8.g0_ - _e10.g1_));
}

fn plane_flector_add(self_604: Plane, other_504: Flector) -> Flector {
    var self_605: Plane;
    var other_505: Flector;

    self_605 = self_604;
    other_505 = other_504;
    let _e4: Flector = other_505;
    let _e6: Plane = self_605;
    let _e8: Flector = other_505;
    return Flector(_e4.g0_, (_e6.g0_ + _e8.g1_));
}

fn plane_flector_sub(self_606: Plane, other_506: Flector) -> Flector {
    var self_607: Plane;
    var other_507: Flector;

    self_607 = self_606;
    other_507 = other_506;
    let _e6: Flector = other_507;
    let _e9: Plane = self_607;
    let _e11: Flector = other_507;
    return Flector((vec4<f32>(0.0) - _e6.g0_), (_e9.g0_ - _e11.g1_));
}

fn plane_multi_vector_add(self_608: Plane, other_508: MultiVector) -> MultiVector {
    var self_609: Plane;
    var other_509: MultiVector;

    self_609 = self_608;
    other_509 = other_508;
    let _e4: MultiVector = other_509;
    let _e6: MultiVector = other_509;
    let _e8: MultiVector = other_509;
    let _e10: MultiVector = other_509;
    let _e12: MultiVector = other_509;
    let _e14: MultiVector = other_509;
    let _e16: MultiVector = other_509;
    let _e18: MultiVector = other_509;
    let _e20: MultiVector = other_509;
    let _e22: Plane = self_609;
    let _e24: MultiVector = other_509;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, _e10.g3_, _e12.g4_, _e14.g5_, _e16.g6_, _e18.g7_, _e20.g8_, (_e22.g0_ + _e24.g9_));
}

fn plane_multi_vector_sub(self_610: Plane, other_510: MultiVector) -> MultiVector {
    var self_611: Plane;
    var other_511: MultiVector;

    self_611 = self_610;
    other_511 = other_510;
    let _e6: MultiVector = other_511;
    let _e11: MultiVector = other_511;
    let _e16: MultiVector = other_511;
    let _e21: MultiVector = other_511;
    let _e26: MultiVector = other_511;
    let _e31: MultiVector = other_511;
    let _e36: MultiVector = other_511;
    let _e41: MultiVector = other_511;
    let _e46: MultiVector = other_511;
    let _e49: Plane = self_611;
    let _e51: MultiVector = other_511;
    return MultiVector((vec3<f32>(0.0) - _e6.g0_), (vec3<f32>(0.0) - _e11.g1_), (vec2<f32>(0.0) - _e16.g2_), (vec4<f32>(0.0) - _e21.g3_), (vec3<f32>(0.0) - _e26.g4_), (vec3<f32>(0.0) - _e31.g5_), (vec3<f32>(0.0) - _e36.g6_), (vec3<f32>(0.0) - _e41.g7_), (vec4<f32>(0.0) - _e46.g8_), (_e49.g0_ - _e51.g9_));
}

fn plane_scale(self_612: Plane, other_512: f32) -> Plane {
    var self_613: Plane;
    var other_513: f32;

    self_613 = self_612;
    other_513 = other_512;
    let _e5: f32 = other_513;
    let _e7: Plane = self_613;
    let _e8: f32 = other_513;
    let _e10: Plane = plane_scalar_geometric_product(_e7, Scalar(_e8));
    return _e10;
}

fn sphere_zero() -> Sphere {
    return Sphere(0.0, vec4<f32>(0.0));
}

fn sphere_one() -> Sphere {
    return Sphere(0.0, vec4<f32>(0.0));
}

fn sphere_neg(self_614: Sphere) -> Sphere {
    var self_615: Sphere;

    self_615 = self_614;
    let _e2: Sphere = self_615;
    let _e7: Sphere = self_615;
    return Sphere((_e2.g0_ * -(1.0)), (_e7.g1_ * vec4<f32>(-(1.0))));
}

fn sphere_automorphism(self_616: Sphere) -> Sphere {
    var self_617: Sphere;

    self_617 = self_616;
    let _e2: Sphere = self_617;
    let _e4: Sphere = self_617;
    return Sphere(_e2.g0_, _e4.g1_);
}

fn sphere_reversal(self_618: Sphere) -> Sphere {
    var self_619: Sphere;

    self_619 = self_618;
    let _e2: Sphere = self_619;
    let _e4: Sphere = self_619;
    return Sphere(_e2.g0_, _e4.g1_);
}

fn sphere_conjugation(self_620: Sphere) -> Sphere {
    var self_621: Sphere;

    self_621 = self_620;
    let _e2: Sphere = self_621;
    let _e4: Sphere = self_621;
    return Sphere(_e2.g0_, _e4.g1_);
}

fn sphere_scalar_geometric_product(self_622: Sphere, other_514: Scalar) -> Sphere {
    var self_623: Sphere;
    var other_515: Scalar;

    self_623 = self_622;
    other_515 = other_514;
    let _e4: Sphere = self_623;
    let _e6: Scalar = other_515;
    let _e9: Sphere = self_623;
    let _e11: Scalar = other_515;
    return Sphere((_e4.g0_ * _e6.g0_), (_e9.g1_ * vec4<f32>(_e11.g0_)));
}

fn sphere_scalar_outer_product(self_624: Sphere, other_516: Scalar) -> Sphere {
    var self_625: Sphere;
    var other_517: Scalar;

    self_625 = self_624;
    other_517 = other_516;
    let _e4: Sphere = self_625;
    let _e6: Scalar = other_517;
    let _e9: Sphere = self_625;
    let _e11: Scalar = other_517;
    return Sphere((_e4.g0_ * _e6.g0_), (_e9.g1_ * vec4<f32>(_e11.g0_)));
}

fn sphere_scalar_inner_product(self_626: Sphere, other_518: Scalar) -> Sphere {
    var self_627: Sphere;
    var other_519: Scalar;

    self_627 = self_626;
    other_519 = other_518;
    let _e4: Sphere = self_627;
    let _e6: Scalar = other_519;
    let _e9: Sphere = self_627;
    let _e11: Scalar = other_519;
    return Sphere((_e4.g0_ * _e6.g0_), (_e9.g1_ * vec4<f32>(_e11.g0_)));
}

fn sphere_scalar_right_contraction(self_628: Sphere, other_520: Scalar) -> Sphere {
    var self_629: Sphere;
    var other_521: Scalar;

    self_629 = self_628;
    other_521 = other_520;
    let _e4: Sphere = self_629;
    let _e6: Scalar = other_521;
    let _e9: Sphere = self_629;
    let _e11: Scalar = other_521;
    return Sphere((_e4.g0_ * _e6.g0_), (_e9.g1_ * vec4<f32>(_e11.g0_)));
}

fn sphere_radial_point_outer_product(self_630: Sphere, other_522: RadialPoint) -> AntiScalar {
    var self_631: Sphere;
    var other_523: RadialPoint;

    self_631 = self_630;
    other_523 = other_522;
    let _e4: Sphere = self_631;
    let _e6: RadialPoint = other_523;
    let _e10: Sphere = self_631;
    let _e13: RadialPoint = other_523;
    let _e18: Sphere = self_631;
    let _e21: RadialPoint = other_523;
    let _e26: Sphere = self_631;
    let _e29: RadialPoint = other_523;
    let _e34: Sphere = self_631;
    let _e37: RadialPoint = other_523;
    return AntiScalar((((((_e4.g0_ * _e6.g1_.y) + (_e10.g1_.x * _e13.g0_.x)) + (_e18.g1_.y * _e21.g0_.y)) + (_e26.g1_.z * _e29.g0_.z)) + (_e34.g1_.w * _e37.g1_.x)));
}

fn sphere_plane_into(self_632: Sphere) -> Plane {
    var self_633: Sphere;

    self_633 = self_632;
    let _e2: Sphere = self_633;
    return Plane(_e2.g1_);
}

fn sphere_plane_add(self_634: Sphere, other_524: Plane) -> Sphere {
    var self_635: Sphere;
    var other_525: Plane;

    self_635 = self_634;
    other_525 = other_524;
    let _e4: Sphere = self_635;
    let _e6: Sphere = self_635;
    let _e8: Plane = other_525;
    return Sphere(_e4.g0_, (_e6.g1_ + _e8.g0_));
}

fn sphere_plane_sub(self_636: Sphere, other_526: Plane) -> Sphere {
    var self_637: Sphere;
    var other_527: Plane;

    self_637 = self_636;
    other_527 = other_526;
    let _e4: Sphere = self_637;
    let _e6: Sphere = self_637;
    let _e8: Plane = other_527;
    return Sphere(_e4.g0_, (_e6.g1_ - _e8.g0_));
}

fn sphere_sphere_add(self_638: Sphere, other_528: Sphere) -> Sphere {
    var self_639: Sphere;
    var other_529: Sphere;

    self_639 = self_638;
    other_529 = other_528;
    let _e4: Sphere = self_639;
    let _e6: Sphere = other_529;
    let _e9: Sphere = self_639;
    let _e11: Sphere = other_529;
    return Sphere((_e4.g0_ + _e6.g0_), (_e9.g1_ + _e11.g1_));
}

fn sphere_sphere_sub(self_640: Sphere, other_530: Sphere) -> Sphere {
    var self_641: Sphere;
    var other_531: Sphere;

    self_641 = self_640;
    other_531 = other_530;
    let _e4: Sphere = self_641;
    let _e6: Sphere = other_531;
    let _e9: Sphere = self_641;
    let _e11: Sphere = other_531;
    return Sphere((_e4.g0_ - _e6.g0_), (_e9.g1_ - _e11.g1_));
}

fn sphere_sphere_mul(self_642: Sphere, other_532: Sphere) -> Sphere {
    var self_643: Sphere;
    var other_533: Sphere;

    self_643 = self_642;
    other_533 = other_532;
    let _e4: Sphere = self_643;
    let _e6: Sphere = other_533;
    let _e9: Sphere = self_643;
    let _e11: Sphere = other_533;
    return Sphere((_e4.g0_ * _e6.g0_), (_e9.g1_ * _e11.g1_));
}

fn sphere_sphere_div(self_644: Sphere, other_534: Sphere) -> Sphere {
    var self_645: Sphere;
    var other_535: Sphere;

    self_645 = self_644;
    other_535 = other_534;
    let _e4: Sphere = self_645;
    let _e8: Sphere = other_535;
    let _e13: Sphere = self_645;
    let _e16: Sphere = self_645;
    let _e19: Sphere = self_645;
    let _e22: Sphere = self_645;
    let _e32: Sphere = other_535;
    let _e35: Sphere = other_535;
    let _e38: Sphere = other_535;
    let _e41: Sphere = other_535;
    return Sphere((((_e4.g0_ * 1.0) / _e8.g0_) * 1.0), (((vec4<f32>(_e13.g1_.x, _e16.g1_.y, _e19.g1_.z, _e22.g1_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e32.g1_.x, _e35.g1_.y, _e38.g1_.z, _e41.g1_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn sphere_motor_geometric_product(self_646: Sphere, other_536: Motor) -> Rotor {
    var self_647: Sphere;
    var other_537: Motor;

    self_647 = self_646;
    other_537 = other_536;
    let _e4: Sphere = self_647;
    let _e7: Motor = other_537;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g1_));
}

fn sphere_motor_outer_product(self_648: Sphere, other_538: Motor) -> AntiScalar {
    var self_649: Sphere;
    var other_539: Motor;

    self_649 = self_648;
    other_539 = other_538;
    let _e4: Sphere = self_649;
    let _e6: Motor = other_539;
    return AntiScalar((_e4.g0_ * _e6.g1_.w));
}

fn sphere_multi_vector_add(self_650: Sphere, other_540: MultiVector) -> MultiVector {
    var self_651: Sphere;
    var other_541: MultiVector;

    self_651 = self_650;
    other_541 = other_540;
    let _e4: Sphere = self_651;
    let _e12: MultiVector = other_541;
    let _e15: MultiVector = other_541;
    let _e17: MultiVector = other_541;
    let _e19: MultiVector = other_541;
    let _e21: MultiVector = other_541;
    let _e23: MultiVector = other_541;
    let _e25: MultiVector = other_541;
    let _e27: MultiVector = other_541;
    let _e29: MultiVector = other_541;
    let _e31: Sphere = self_651;
    let _e33: MultiVector = other_541;
    return MultiVector(((vec3<f32>(_e4.g0_) * vec3<f32>(0.0, 1.0, 0.0)) + _e12.g0_), _e15.g1_, _e17.g2_, _e19.g3_, _e21.g4_, _e23.g5_, _e25.g6_, _e27.g7_, _e29.g8_, (_e31.g1_ + _e33.g9_));
}

fn sphere_multi_vector_sub(self_652: Sphere, other_542: MultiVector) -> MultiVector {
    var self_653: Sphere;
    var other_543: MultiVector;

    self_653 = self_652;
    other_543 = other_542;
    let _e4: Sphere = self_653;
    let _e12: MultiVector = other_543;
    let _e17: MultiVector = other_543;
    let _e22: MultiVector = other_543;
    let _e27: MultiVector = other_543;
    let _e32: MultiVector = other_543;
    let _e37: MultiVector = other_543;
    let _e42: MultiVector = other_543;
    let _e47: MultiVector = other_543;
    let _e52: MultiVector = other_543;
    let _e55: Sphere = self_653;
    let _e57: MultiVector = other_543;
    return MultiVector(((vec3<f32>(_e4.g0_) * vec3<f32>(0.0, 1.0, 0.0)) - _e12.g0_), (vec3<f32>(0.0) - _e17.g1_), (vec2<f32>(0.0) - _e22.g2_), (vec4<f32>(0.0) - _e27.g3_), (vec3<f32>(0.0) - _e32.g4_), (vec3<f32>(0.0) - _e37.g5_), (vec3<f32>(0.0) - _e42.g6_), (vec3<f32>(0.0) - _e47.g7_), (vec4<f32>(0.0) - _e52.g8_), (_e55.g1_ - _e57.g9_));
}

fn sphere_scale(self_654: Sphere, other_544: f32) -> Sphere {
    var self_655: Sphere;
    var other_545: f32;

    self_655 = self_654;
    other_545 = other_544;
    let _e5: f32 = other_545;
    let _e7: Sphere = self_655;
    let _e8: f32 = other_545;
    let _e10: Sphere = sphere_scalar_geometric_product(_e7, Scalar(_e8));
    return _e10;
}

fn motor_zero() -> Motor {
    return Motor(vec4<f32>(0.0), vec4<f32>(0.0));
}

fn motor_one() -> Motor {
    return Motor(vec4<f32>(0.0), vec4<f32>(0.0));
}

fn motor_neg(self_656: Motor) -> Motor {
    var self_657: Motor;

    self_657 = self_656;
    let _e2: Motor = self_657;
    let _e8: Motor = self_657;
    return Motor((_e2.g0_ * vec4<f32>(-(1.0))), (_e8.g1_ * vec4<f32>(-(1.0))));
}

fn motor_automorphism(self_658: Motor) -> Motor {
    var self_659: Motor;

    self_659 = self_658;
    let _e2: Motor = self_659;
    let _e8: Motor = self_659;
    return Motor((_e2.g0_ * vec4<f32>(-(1.0))), (_e8.g1_ * vec4<f32>(-(1.0))));
}

fn motor_reversal(self_660: Motor) -> Motor {
    var self_661: Motor;

    self_661 = self_660;
    let _e2: Motor = self_661;
    let _e13: Motor = self_661;
    return Motor((_e2.g0_ * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0)), (_e13.g1_ * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0)));
}

fn motor_conjugation(self_662: Motor) -> Motor {
    var self_663: Motor;

    self_663 = self_662;
    let _e2: Motor = self_663;
    let _e11: Motor = self_663;
    return Motor((_e2.g0_ * vec4<f32>(1.0, 1.0, 1.0, -(1.0))), (_e11.g1_ * vec4<f32>(1.0, 1.0, 1.0, -(1.0))));
}

fn motor_scalar_geometric_product(self_664: Motor, other_546: Scalar) -> Motor {
    var self_665: Motor;
    var other_547: Scalar;

    self_665 = self_664;
    other_547 = other_546;
    let _e4: Motor = self_665;
    let _e6: Scalar = other_547;
    let _e10: Motor = self_665;
    let _e12: Scalar = other_547;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn motor_scalar_outer_product(self_666: Motor, other_548: Scalar) -> Motor {
    var self_667: Motor;
    var other_549: Scalar;

    self_667 = self_666;
    other_549 = other_548;
    let _e4: Motor = self_667;
    let _e6: Scalar = other_549;
    let _e10: Motor = self_667;
    let _e12: Scalar = other_549;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn motor_scalar_inner_product(self_668: Motor, other_550: Scalar) -> Motor {
    var self_669: Motor;
    var other_551: Scalar;

    self_669 = self_668;
    other_551 = other_550;
    let _e4: Motor = self_669;
    let _e6: Scalar = other_551;
    let _e10: Motor = self_669;
    let _e12: Scalar = other_551;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn motor_scalar_right_contraction(self_670: Motor, other_552: Scalar) -> Motor {
    var self_671: Motor;
    var other_553: Scalar;

    self_671 = self_670;
    other_553 = other_552;
    let _e4: Motor = self_671;
    let _e6: Scalar = other_553;
    let _e10: Motor = self_671;
    let _e12: Scalar = other_553;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn motor_anti_scalar_into(self_672: Motor) -> AntiScalar {
    var self_673: Motor;

    self_673 = self_672;
    let _e2: Motor = self_673;
    return AntiScalar(_e2.g0_.w);
}

fn motor_anti_scalar_add(self_674: Motor, other_554: AntiScalar) -> Motor {
    var self_675: Motor;
    var other_555: AntiScalar;

    self_675 = self_674;
    other_555 = other_554;
    let _e4: Motor = self_675;
    let _e6: AntiScalar = other_555;
    let _e16: Motor = self_675;
    return Motor((_e4.g0_ + (vec4<f32>(_e6.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0))), _e16.g1_);
}

fn motor_anti_scalar_sub(self_676: Motor, other_556: AntiScalar) -> Motor {
    var self_677: Motor;
    var other_557: AntiScalar;

    self_677 = self_676;
    other_557 = other_556;
    let _e4: Motor = self_677;
    let _e6: AntiScalar = other_557;
    let _e16: Motor = self_677;
    return Motor((_e4.g0_ - (vec4<f32>(_e6.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0))), _e16.g1_);
}

fn motor_radial_point_geometric_product(self_678: Motor, other_558: RadialPoint) -> Flector {
    var self_679: Motor;
    var other_559: RadialPoint;

    self_679 = self_678;
    other_559 = other_558;
    let _e4: Motor = self_679;
    let _e8: RadialPoint = other_559;
    let _e20: Motor = self_679;
    let _e24: RadialPoint = other_559;
    let _e37: Motor = self_679;
    let _e41: RadialPoint = other_559;
    let _e44: RadialPoint = other_559;
    let _e47: RadialPoint = other_559;
    let _e50: RadialPoint = other_559;
    let _e63: Motor = self_679;
    let _e67: RadialPoint = other_559;
    let _e70: RadialPoint = other_559;
    let _e73: RadialPoint = other_559;
    let _e76: RadialPoint = other_559;
    let _e89: Motor = self_679;
    let _e93: RadialPoint = other_559;
    let _e96: RadialPoint = other_559;
    let _e99: RadialPoint = other_559;
    let _e102: RadialPoint = other_559;
    let _e108: Motor = self_679;
    let _e111: Motor = self_679;
    let _e114: Motor = self_679;
    let _e117: Motor = self_679;
    let _e121: RadialPoint = other_559;
    let _e124: RadialPoint = other_559;
    let _e127: RadialPoint = other_559;
    let _e130: RadialPoint = other_559;
    let _e144: Motor = self_679;
    let _e148: RadialPoint = other_559;
    let _e151: RadialPoint = other_559;
    let _e154: RadialPoint = other_559;
    let _e157: RadialPoint = other_559;
    let _e169: Motor = self_679;
    let _e173: RadialPoint = other_559;
    let _e176: RadialPoint = other_559;
    let _e179: RadialPoint = other_559;
    let _e182: RadialPoint = other_559;
    let _e195: Motor = self_679;
    let _e199: RadialPoint = other_559;
    let _e202: RadialPoint = other_559;
    let _e205: RadialPoint = other_559;
    let _e208: RadialPoint = other_559;
    let _e220: Motor = self_679;
    let _e224: RadialPoint = other_559;
    let _e227: RadialPoint = other_559;
    let _e230: RadialPoint = other_559;
    let _e233: RadialPoint = other_559;
    let _e246: Motor = self_679;
    let _e250: RadialPoint = other_559;
    let _e253: RadialPoint = other_559;
    let _e256: RadialPoint = other_559;
    let _e259: RadialPoint = other_559;
    let _e272: Motor = self_679;
    let _e276: RadialPoint = other_559;
    let _e279: RadialPoint = other_559;
    let _e282: RadialPoint = other_559;
    let _e285: RadialPoint = other_559;
    let _e298: Motor = self_679;
    let _e302: RadialPoint = other_559;
    let _e305: RadialPoint = other_559;
    let _e308: RadialPoint = other_559;
    let _e311: RadialPoint = other_559;
    return Flector((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e37.g1_.y) * vec4<f32>(_e41.g0_.z, _e44.g0_.z, _e47.g0_.x, _e50.g0_.z)) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e63.g1_.z) * vec4<f32>(_e67.g0_.y, _e70.g0_.x, _e73.g0_.y, _e76.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) - (vec4<f32>(_e89.g1_.w) * vec4<f32>(_e93.g0_.x, _e96.g0_.y, _e99.g0_.z, _e102.g1_.x))) + ((vec4<f32>(_e108.g0_.x, _e111.g1_.x, _e114.g1_.x, _e117.g0_.x) * vec4<f32>(_e121.g0_.x, _e124.g0_.z, _e127.g0_.y, _e130.g0_.x)) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))), ((((((((vec4<f32>(_e144.g0_.y) * vec4<f32>(_e148.g0_.z, _e151.g0_.z, _e154.g0_.x, _e157.g0_.z)) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e169.g0_.z) * vec4<f32>(_e173.g0_.y, _e176.g0_.x, _e179.g0_.y, _e182.g0_.y)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e195.g0_.w) * vec4<f32>(_e199.g0_.x, _e202.g0_.y, _e205.g0_.z, _e208.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e220.g1_.x) * vec4<f32>(_e224.g1_.x, _e227.g1_.x, _e230.g1_.x, _e233.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e246.g1_.y) * vec4<f32>(_e250.g1_.x, _e253.g1_.x, _e256.g1_.x, _e259.g0_.y)) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e272.g1_.z) * vec4<f32>(_e276.g1_.x, _e279.g1_.x, _e282.g1_.x, _e285.g0_.z)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e298.g0_.x) * vec4<f32>(_e302.g0_.x, _e305.g0_.z, _e308.g0_.y, _e311.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))));
}

fn motor_radial_point_outer_product(self_680: Motor, other_560: RadialPoint) -> Flector {
    var self_681: Motor;
    var other_561: RadialPoint;

    self_681 = self_680;
    other_561 = other_560;
    let _e6: Motor = self_681;
    let _e10: RadialPoint = other_561;
    let _e13: RadialPoint = other_561;
    let _e16: RadialPoint = other_561;
    let _e19: RadialPoint = other_561;
    let _e25: Motor = self_681;
    let _e29: RadialPoint = other_561;
    let _e32: RadialPoint = other_561;
    let _e35: RadialPoint = other_561;
    let _e38: RadialPoint = other_561;
    let _e50: Motor = self_681;
    let _e54: RadialPoint = other_561;
    let _e57: RadialPoint = other_561;
    let _e60: RadialPoint = other_561;
    let _e63: RadialPoint = other_561;
    let _e76: Motor = self_681;
    let _e80: RadialPoint = other_561;
    let _e83: RadialPoint = other_561;
    let _e86: RadialPoint = other_561;
    let _e89: RadialPoint = other_561;
    let _e102: Motor = self_681;
    let _e106: RadialPoint = other_561;
    let _e109: RadialPoint = other_561;
    let _e112: RadialPoint = other_561;
    let _e115: RadialPoint = other_561;
    let _e128: Motor = self_681;
    let _e132: RadialPoint = other_561;
    let _e135: RadialPoint = other_561;
    let _e138: RadialPoint = other_561;
    let _e141: RadialPoint = other_561;
    let _e154: Motor = self_681;
    let _e158: RadialPoint = other_561;
    let _e161: RadialPoint = other_561;
    let _e164: RadialPoint = other_561;
    let _e167: RadialPoint = other_561;
    return Flector((vec4<f32>(0.0) - (vec4<f32>(_e6.g1_.w) * vec4<f32>(_e10.g0_.x, _e13.g0_.y, _e16.g0_.z, _e19.g1_.x))), (((((((vec4<f32>(_e25.g0_.y) * vec4<f32>(_e29.g0_.z, _e32.g0_.z, _e35.g0_.x, _e38.g0_.z)) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e50.g0_.z) * vec4<f32>(_e54.g0_.y, _e57.g0_.x, _e60.g0_.y, _e63.g0_.y)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e76.g1_.x) * vec4<f32>(_e80.g1_.x, _e83.g1_.x, _e86.g1_.x, _e89.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e102.g1_.y) * vec4<f32>(_e106.g1_.x, _e109.g1_.x, _e112.g1_.x, _e115.g0_.y)) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e128.g1_.z) * vec4<f32>(_e132.g1_.x, _e135.g1_.x, _e138.g1_.x, _e141.g0_.z)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e154.g0_.x) * vec4<f32>(_e158.g0_.x, _e161.g0_.z, _e164.g0_.y, _e167.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))));
}

fn motor_dipole_geometric_product(self_682: Motor, other_562: Dipole) -> Motor {
    var self_683: Motor;
    var other_563: Dipole;

    self_683 = self_682;
    other_563 = other_562;
    let _e4: Motor = self_683;
    let _e8: Dipole = other_563;
    let _e11: Dipole = other_563;
    let _e14: Dipole = other_563;
    let _e17: Dipole = other_563;
    let _e30: Motor = self_683;
    let _e34: Dipole = other_563;
    let _e37: Dipole = other_563;
    let _e40: Dipole = other_563;
    let _e43: Dipole = other_563;
    let _e57: Motor = self_683;
    let _e61: Dipole = other_563;
    let _e64: Dipole = other_563;
    let _e67: Dipole = other_563;
    let _e70: Dipole = other_563;
    let _e82: Motor = self_683;
    let _e86: Dipole = other_563;
    let _e89: Dipole = other_563;
    let _e92: Dipole = other_563;
    let _e95: Dipole = other_563;
    let _e109: Motor = self_683;
    let _e113: Dipole = other_563;
    let _e116: Dipole = other_563;
    let _e119: Dipole = other_563;
    let _e122: Dipole = other_563;
    let _e136: Motor = self_683;
    let _e140: Dipole = other_563;
    let _e143: Dipole = other_563;
    let _e146: Dipole = other_563;
    let _e149: Dipole = other_563;
    let _e163: Motor = self_683;
    let _e167: Dipole = other_563;
    let _e170: Dipole = other_563;
    let _e173: Dipole = other_563;
    let _e176: Dipole = other_563;
    let _e188: Motor = self_683;
    let _e192: Dipole = other_563;
    let _e195: Dipole = other_563;
    let _e198: Dipole = other_563;
    let _e201: Dipole = other_563;
    let _e215: Motor = self_683;
    let _e219: Dipole = other_563;
    let _e222: Dipole = other_563;
    let _e225: Dipole = other_563;
    let _e228: Dipole = other_563;
    let _e241: Motor = self_683;
    let _e245: Dipole = other_563;
    let _e248: Dipole = other_563;
    let _e251: Dipole = other_563;
    let _e254: Dipole = other_563;
    let _e268: Motor = self_683;
    let _e272: Dipole = other_563;
    let _e275: Dipole = other_563;
    let _e278: Dipole = other_563;
    let _e281: Dipole = other_563;
    let _e293: Motor = self_683;
    let _e297: Dipole = other_563;
    let _e300: Dipole = other_563;
    let _e303: Dipole = other_563;
    let _e306: Dipole = other_563;
    return Motor((((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g1_.x, _e64.g1_.y, _e67.g1_.z, _e70.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g1_.x) * vec4<f32>(_e86.g0_.z, _e89.g0_.z, _e92.g0_.y, _e95.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e109.g1_.y) * vec4<f32>(_e113.g0_.z, _e116.g0_.z, _e119.g0_.x, _e122.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e136.g1_.z) * vec4<f32>(_e140.g0_.y, _e143.g0_.x, _e146.g0_.y, _e149.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e163.g1_.w) * vec4<f32>(_e167.g0_.x, _e170.g0_.y, _e173.g0_.z, _e176.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e188.g0_.x) * vec4<f32>(_e192.g1_.x, _e195.g1_.z, _e198.g1_.y, _e201.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), (((((vec4<f32>(_e215.g1_.y) * vec4<f32>(_e219.g1_.z, _e222.g1_.z, _e225.g1_.x, _e228.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e241.g1_.z) * vec4<f32>(_e245.g1_.y, _e248.g1_.x, _e251.g1_.y, _e254.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e268.g1_.w) * vec4<f32>(_e272.g1_.x, _e275.g1_.y, _e278.g1_.z, _e281.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e293.g1_.x) * vec4<f32>(_e297.g1_.x, _e300.g1_.z, _e303.g1_.y, _e306.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn motor_line_into(self_684: Motor) -> Line {
    var self_685: Motor;

    self_685 = self_684;
    let _e2: Motor = self_685;
    let _e5: Motor = self_685;
    let _e8: Motor = self_685;
    let _e12: Motor = self_685;
    let _e15: Motor = self_685;
    let _e18: Motor = self_685;
    return Line(vec3<f32>(_e2.g0_.x, _e5.g0_.y, _e8.g0_.z), vec3<f32>(_e12.g1_.x, _e15.g1_.y, _e18.g1_.z));
}

fn motor_line_add(self_686: Motor, other_564: Line) -> Motor {
    var self_687: Motor;
    var other_565: Line;

    self_687 = self_686;
    other_565 = other_564;
    let _e4: Motor = self_687;
    let _e6: Line = other_565;
    let _e9: Line = other_565;
    let _e12: Line = other_565;
    let _e15: Line = other_565;
    let _e26: Motor = self_687;
    let _e28: Line = other_565;
    let _e31: Line = other_565;
    let _e34: Line = other_565;
    let _e37: Line = other_565;
    return Motor((_e4.g0_ + (vec4<f32>(_e6.g0_.x, _e9.g0_.y, _e12.g0_.z, _e15.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), (_e26.g1_ + (vec4<f32>(_e28.g1_.x, _e31.g1_.y, _e34.g1_.z, _e37.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn motor_line_sub(self_688: Motor, other_566: Line) -> Motor {
    var self_689: Motor;
    var other_567: Line;

    self_689 = self_688;
    other_567 = other_566;
    let _e4: Motor = self_689;
    let _e6: Line = other_567;
    let _e9: Line = other_567;
    let _e12: Line = other_567;
    let _e15: Line = other_567;
    let _e26: Motor = self_689;
    let _e28: Line = other_567;
    let _e31: Line = other_567;
    let _e34: Line = other_567;
    let _e37: Line = other_567;
    return Motor((_e4.g0_ - (vec4<f32>(_e6.g0_.x, _e9.g0_.y, _e12.g0_.z, _e15.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), (_e26.g1_ - (vec4<f32>(_e28.g1_.x, _e31.g1_.y, _e34.g1_.z, _e37.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn motor_circle_geometric_product(self_690: Motor, other_568: Circle) -> Flector {
    var self_691: Motor;
    var other_569: Circle;

    self_691 = self_690;
    other_569 = other_568;
    let _e4: Motor = self_691;
    let _e8: Circle = other_569;
    let _e19: Motor = self_691;
    let _e23: Circle = other_569;
    let _e35: Motor = self_691;
    let _e39: Circle = other_569;
    let _e51: Motor = self_691;
    let _e54: Circle = other_569;
    let _e65: Motor = self_691;
    let _e69: Circle = other_569;
    let _e80: Motor = self_691;
    let _e84: Circle = other_569;
    let _e96: Motor = self_691;
    let _e100: Circle = other_569;
    let _e112: Motor = self_691;
    let _e116: Circle = other_569;
    let _e120: Motor = self_691;
    let _e123: Circle = other_569;
    return Flector((((((vec4<f32>(_e4.g1_.x) * _e8.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0)) + ((vec4<f32>(_e19.g1_.y) * _e23.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e35.g1_.z) * _e39.g0_.wwwz) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((_e51.g0_.xxxw * _e54.g0_.xxxw) * vec4<f32>(0.0, 0.0, 0.0, 1.0))), ((((((vec4<f32>(_e65.g1_.x) * _e69.g0_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0)) + ((vec4<f32>(_e80.g1_.y) * _e84.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e96.g1_.z) * _e100.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) - (vec4<f32>(_e112.g1_.w) * _e116.g0_)) + ((_e120.g0_.xyzx * _e123.g0_.wwwx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn motor_circle_outer_product(self_692: Motor, other_570: Circle) -> Plane {
    var self_693: Motor;
    var other_571: Circle;

    self_693 = self_692;
    other_571 = other_570;
    let _e6: Motor = self_693;
    let _e10: Circle = other_571;
    return Plane((vec4<f32>(0.0) - (vec4<f32>(_e6.g1_.w) * _e10.g0_)));
}

fn motor_sphere_geometric_product(self_694: Motor, other_572: Sphere) -> Rotor {
    var self_695: Motor;
    var other_573: Sphere;

    self_695 = self_694;
    other_573 = other_572;
    let _e4: Motor = self_695;
    let _e6: Sphere = other_573;
    return Rotor((_e4.g1_ * vec4<f32>(_e6.g0_)));
}

fn motor_sphere_outer_product(self_696: Motor, other_574: Sphere) -> AntiScalar {
    var self_697: Motor;
    var other_575: Sphere;

    self_697 = self_696;
    other_575 = other_574;
    let _e4: Motor = self_697;
    let _e7: Sphere = other_575;
    return AntiScalar((_e4.g1_.w * _e7.g0_));
}

fn motor_motor_add(self_698: Motor, other_576: Motor) -> Motor {
    var self_699: Motor;
    var other_577: Motor;

    self_699 = self_698;
    other_577 = other_576;
    let _e4: Motor = self_699;
    let _e6: Motor = other_577;
    let _e9: Motor = self_699;
    let _e11: Motor = other_577;
    return Motor((_e4.g0_ + _e6.g0_), (_e9.g1_ + _e11.g1_));
}

fn motor_motor_sub(self_700: Motor, other_578: Motor) -> Motor {
    var self_701: Motor;
    var other_579: Motor;

    self_701 = self_700;
    other_579 = other_578;
    let _e4: Motor = self_701;
    let _e6: Motor = other_579;
    let _e9: Motor = self_701;
    let _e11: Motor = other_579;
    return Motor((_e4.g0_ - _e6.g0_), (_e9.g1_ - _e11.g1_));
}

fn motor_motor_mul(self_702: Motor, other_580: Motor) -> Motor {
    var self_703: Motor;
    var other_581: Motor;

    self_703 = self_702;
    other_581 = other_580;
    let _e4: Motor = self_703;
    let _e6: Motor = other_581;
    let _e9: Motor = self_703;
    let _e11: Motor = other_581;
    return Motor((_e4.g0_ * _e6.g0_), (_e9.g1_ * _e11.g1_));
}

fn motor_motor_div(self_704: Motor, other_582: Motor) -> Motor {
    var self_705: Motor;
    var other_583: Motor;

    self_705 = self_704;
    other_583 = other_582;
    let _e4: Motor = self_705;
    let _e7: Motor = self_705;
    let _e10: Motor = self_705;
    let _e13: Motor = self_705;
    let _e23: Motor = other_583;
    let _e26: Motor = other_583;
    let _e29: Motor = other_583;
    let _e32: Motor = other_583;
    let _e43: Motor = self_705;
    let _e46: Motor = self_705;
    let _e49: Motor = self_705;
    let _e52: Motor = self_705;
    let _e62: Motor = other_583;
    let _e65: Motor = other_583;
    let _e68: Motor = other_583;
    let _e71: Motor = other_583;
    return Motor((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)), (((vec4<f32>(_e43.g1_.x, _e46.g1_.y, _e49.g1_.z, _e52.g1_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e62.g1_.x, _e65.g1_.y, _e68.g1_.z, _e71.g1_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn motor_rotor_into(self_706: Motor) -> Rotor {
    var self_707: Motor;

    self_707 = self_706;
    let _e2: Motor = self_707;
    return Rotor(_e2.g0_);
}

fn motor_rotor_add(self_708: Motor, other_584: Rotor) -> Motor {
    var self_709: Motor;
    var other_585: Rotor;

    self_709 = self_708;
    other_585 = other_584;
    let _e4: Motor = self_709;
    let _e6: Rotor = other_585;
    let _e9: Motor = self_709;
    return Motor((_e4.g0_ + _e6.g0_), _e9.g1_);
}

fn motor_rotor_sub(self_710: Motor, other_586: Rotor) -> Motor {
    var self_711: Motor;
    var other_587: Rotor;

    self_711 = self_710;
    other_587 = other_586;
    let _e4: Motor = self_711;
    let _e6: Rotor = other_587;
    let _e9: Motor = self_711;
    return Motor((_e4.g0_ - _e6.g0_), _e9.g1_);
}

fn motor_translator_into(self_712: Motor) -> Translator {
    var self_713: Motor;

    self_713 = self_712;
    let _e2: Motor = self_713;
    let _e5: Motor = self_713;
    let _e8: Motor = self_713;
    let _e11: Motor = self_713;
    return Translator(vec4<f32>(_e2.g1_.x, _e5.g1_.y, _e8.g1_.z, _e11.g0_.w));
}

fn motor_translator_add(self_714: Motor, other_588: Translator) -> Motor {
    var self_715: Motor;
    var other_589: Translator;

    self_715 = self_714;
    other_589 = other_588;
    let _e4: Motor = self_715;
    let _e6: Translator = other_589;
    let _e16: Motor = self_715;
    let _e18: Translator = other_589;
    return Motor((_e4.g0_ + (_e6.g0_.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0))), (_e16.g1_ + (_e18.g0_.xyzx * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn motor_translator_sub(self_716: Motor, other_590: Translator) -> Motor {
    var self_717: Motor;
    var other_591: Translator;

    self_717 = self_716;
    other_591 = other_590;
    let _e4: Motor = self_717;
    let _e6: Translator = other_591;
    let _e16: Motor = self_717;
    let _e18: Translator = other_591;
    return Motor((_e4.g0_ - (_e6.g0_.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0))), (_e16.g1_ - (_e18.g0_.xyzx * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn motor_multi_vector_add(self_718: Motor, other_592: MultiVector) -> MultiVector {
    var self_719: Motor;
    var other_593: MultiVector;

    self_719 = self_718;
    other_593 = other_592;
    let _e4: Motor = self_719;
    let _e7: Motor = self_719;
    let _e10: Motor = self_719;
    let _e19: MultiVector = other_593;
    let _e22: MultiVector = other_593;
    let _e24: Motor = self_719;
    let _e27: Motor = self_719;
    let _e35: MultiVector = other_593;
    let _e38: MultiVector = other_593;
    let _e40: MultiVector = other_593;
    let _e42: MultiVector = other_593;
    let _e44: Motor = self_719;
    let _e47: Motor = self_719;
    let _e50: Motor = self_719;
    let _e54: MultiVector = other_593;
    let _e57: Motor = self_719;
    let _e60: Motor = self_719;
    let _e63: Motor = self_719;
    let _e67: MultiVector = other_593;
    let _e70: MultiVector = other_593;
    let _e72: MultiVector = other_593;
    return MultiVector(((vec3<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.w) * vec3<f32>(0.0, 0.0, 1.0)) + _e19.g0_), _e22.g1_, ((vec2<f32>(_e24.g0_.x, _e27.g1_.w) * vec2<f32>(0.0, 1.0)) + _e35.g2_), _e38.g3_, _e40.g4_, _e42.g5_, (vec3<f32>(_e44.g0_.x, _e47.g0_.y, _e50.g0_.z) + _e54.g6_), (vec3<f32>(_e57.g1_.x, _e60.g1_.y, _e63.g1_.z) + _e67.g7_), _e70.g8_, _e72.g9_);
}

fn motor_multi_vector_sub(self_720: Motor, other_594: MultiVector) -> MultiVector {
    var self_721: Motor;
    var other_595: MultiVector;

    self_721 = self_720;
    other_595 = other_594;
    let _e4: Motor = self_721;
    let _e7: Motor = self_721;
    let _e10: Motor = self_721;
    let _e19: MultiVector = other_595;
    let _e24: MultiVector = other_595;
    let _e27: Motor = self_721;
    let _e30: Motor = self_721;
    let _e38: MultiVector = other_595;
    let _e43: MultiVector = other_595;
    let _e48: MultiVector = other_595;
    let _e53: MultiVector = other_595;
    let _e56: Motor = self_721;
    let _e59: Motor = self_721;
    let _e62: Motor = self_721;
    let _e66: MultiVector = other_595;
    let _e69: Motor = self_721;
    let _e72: Motor = self_721;
    let _e75: Motor = self_721;
    let _e79: MultiVector = other_595;
    let _e84: MultiVector = other_595;
    let _e89: MultiVector = other_595;
    return MultiVector(((vec3<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.w) * vec3<f32>(0.0, 0.0, 1.0)) - _e19.g0_), (vec3<f32>(0.0) - _e24.g1_), ((vec2<f32>(_e27.g0_.x, _e30.g1_.w) * vec2<f32>(0.0, 1.0)) - _e38.g2_), (vec4<f32>(0.0) - _e43.g3_), (vec3<f32>(0.0) - _e48.g4_), (vec3<f32>(0.0) - _e53.g5_), (vec3<f32>(_e56.g0_.x, _e59.g0_.y, _e62.g0_.z) - _e66.g6_), (vec3<f32>(_e69.g1_.x, _e72.g1_.y, _e75.g1_.z) - _e79.g7_), (vec4<f32>(0.0) - _e84.g8_), (vec4<f32>(0.0) - _e89.g9_));
}

fn motor_scale(self_722: Motor, other_596: f32) -> Motor {
    var self_723: Motor;
    var other_597: f32;

    self_723 = self_722;
    other_597 = other_596;
    let _e5: f32 = other_597;
    let _e7: Motor = self_723;
    let _e8: f32 = other_597;
    let _e10: Motor = motor_scalar_geometric_product(_e7, Scalar(_e8));
    return _e10;
}

fn rotor_zero() -> Rotor {
    return Rotor(vec4<f32>(0.0));
}

fn rotor_one() -> Rotor {
    return Rotor(vec4<f32>(0.0));
}

fn rotor_neg(self_724: Rotor) -> Rotor {
    var self_725: Rotor;

    self_725 = self_724;
    let _e2: Rotor = self_725;
    return Rotor((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn rotor_automorphism(self_726: Rotor) -> Rotor {
    var self_727: Rotor;

    self_727 = self_726;
    let _e2: Rotor = self_727;
    return Rotor((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn rotor_reversal(self_728: Rotor) -> Rotor {
    var self_729: Rotor;

    self_729 = self_728;
    let _e2: Rotor = self_729;
    return Rotor((_e2.g0_ * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0)));
}

fn rotor_conjugation(self_730: Rotor) -> Rotor {
    var self_731: Rotor;

    self_731 = self_730;
    let _e2: Rotor = self_731;
    return Rotor((_e2.g0_ * vec4<f32>(1.0, 1.0, 1.0, -(1.0))));
}

fn rotor_scalar_geometric_product(self_732: Rotor, other_598: Scalar) -> Rotor {
    var self_733: Rotor;
    var other_599: Scalar;

    self_733 = self_732;
    other_599 = other_598;
    let _e4: Rotor = self_733;
    let _e6: Scalar = other_599;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn rotor_scalar_outer_product(self_734: Rotor, other_600: Scalar) -> Rotor {
    var self_735: Rotor;
    var other_601: Scalar;

    self_735 = self_734;
    other_601 = other_600;
    let _e4: Rotor = self_735;
    let _e6: Scalar = other_601;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn rotor_scalar_inner_product(self_736: Rotor, other_602: Scalar) -> Rotor {
    var self_737: Rotor;
    var other_603: Scalar;

    self_737 = self_736;
    other_603 = other_602;
    let _e4: Rotor = self_737;
    let _e6: Scalar = other_603;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn rotor_scalar_right_contraction(self_738: Rotor, other_604: Scalar) -> Rotor {
    var self_739: Rotor;
    var other_605: Scalar;

    self_739 = self_738;
    other_605 = other_604;
    let _e4: Rotor = self_739;
    let _e6: Scalar = other_605;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn rotor_anti_scalar_into(self_740: Rotor) -> AntiScalar {
    var self_741: Rotor;

    self_741 = self_740;
    let _e2: Rotor = self_741;
    return AntiScalar(_e2.g0_.w);
}

fn rotor_anti_scalar_add(self_742: Rotor, other_606: AntiScalar) -> Rotor {
    var self_743: Rotor;
    var other_607: AntiScalar;

    self_743 = self_742;
    other_607 = other_606;
    let _e4: Rotor = self_743;
    let _e6: AntiScalar = other_607;
    return Rotor((_e4.g0_ + (vec4<f32>(_e6.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0))));
}

fn rotor_anti_scalar_sub(self_744: Rotor, other_608: AntiScalar) -> Rotor {
    var self_745: Rotor;
    var other_609: AntiScalar;

    self_745 = self_744;
    other_609 = other_608;
    let _e4: Rotor = self_745;
    let _e6: AntiScalar = other_609;
    return Rotor((_e4.g0_ - (vec4<f32>(_e6.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0))));
}

fn rotor_dipole_geometric_product(self_746: Rotor, other_610: Dipole) -> Rotor {
    var self_747: Rotor;
    var other_611: Dipole;

    self_747 = self_746;
    other_611 = other_610;
    let _e4: Rotor = self_747;
    let _e8: Dipole = other_611;
    let _e11: Dipole = other_611;
    let _e14: Dipole = other_611;
    let _e17: Dipole = other_611;
    let _e30: Rotor = self_747;
    let _e34: Dipole = other_611;
    let _e37: Dipole = other_611;
    let _e40: Dipole = other_611;
    let _e43: Dipole = other_611;
    let _e57: Rotor = self_747;
    let _e61: Dipole = other_611;
    let _e64: Dipole = other_611;
    let _e67: Dipole = other_611;
    let _e70: Dipole = other_611;
    let _e82: Rotor = self_747;
    let _e86: Dipole = other_611;
    let _e89: Dipole = other_611;
    let _e92: Dipole = other_611;
    let _e95: Dipole = other_611;
    return Rotor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g1_.x, _e64.g1_.y, _e67.g1_.z, _e70.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g0_.x) * vec4<f32>(_e86.g1_.x, _e89.g1_.z, _e92.g1_.y, _e95.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn rotor_dipole_outer_product(self_748: Rotor, other_612: Dipole) -> AntiScalar {
    var self_749: Rotor;
    var other_613: Dipole;

    self_749 = self_748;
    other_613 = other_612;
    let _e5: Rotor = self_749;
    let _e8: Dipole = other_613;
    let _e13: Rotor = self_749;
    let _e16: Dipole = other_613;
    let _e21: Rotor = self_749;
    let _e24: Dipole = other_613;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)));
}

fn rotor_motor_add(self_750: Rotor, other_614: Motor) -> Motor {
    var self_751: Rotor;
    var other_615: Motor;

    self_751 = self_750;
    other_615 = other_614;
    let _e4: Rotor = self_751;
    let _e6: Motor = other_615;
    let _e9: Motor = other_615;
    return Motor((_e4.g0_ + _e6.g0_), _e9.g1_);
}

fn rotor_motor_sub(self_752: Rotor, other_616: Motor) -> Motor {
    var self_753: Rotor;
    var other_617: Motor;

    self_753 = self_752;
    other_617 = other_616;
    let _e4: Rotor = self_753;
    let _e6: Motor = other_617;
    let _e11: Motor = other_617;
    return Motor((_e4.g0_ - _e6.g0_), (vec4<f32>(0.0) - _e11.g1_));
}

fn rotor_rotor_add(self_754: Rotor, other_618: Rotor) -> Rotor {
    var self_755: Rotor;
    var other_619: Rotor;

    self_755 = self_754;
    other_619 = other_618;
    let _e4: Rotor = self_755;
    let _e6: Rotor = other_619;
    return Rotor((_e4.g0_ + _e6.g0_));
}

fn rotor_rotor_sub(self_756: Rotor, other_620: Rotor) -> Rotor {
    var self_757: Rotor;
    var other_621: Rotor;

    self_757 = self_756;
    other_621 = other_620;
    let _e4: Rotor = self_757;
    let _e6: Rotor = other_621;
    return Rotor((_e4.g0_ - _e6.g0_));
}

fn rotor_rotor_mul(self_758: Rotor, other_622: Rotor) -> Rotor {
    var self_759: Rotor;
    var other_623: Rotor;

    self_759 = self_758;
    other_623 = other_622;
    let _e4: Rotor = self_759;
    let _e6: Rotor = other_623;
    return Rotor((_e4.g0_ * _e6.g0_));
}

fn rotor_rotor_div(self_760: Rotor, other_624: Rotor) -> Rotor {
    var self_761: Rotor;
    var other_625: Rotor;

    self_761 = self_760;
    other_625 = other_624;
    let _e4: Rotor = self_761;
    let _e7: Rotor = self_761;
    let _e10: Rotor = self_761;
    let _e13: Rotor = self_761;
    let _e23: Rotor = other_625;
    let _e26: Rotor = other_625;
    let _e29: Rotor = other_625;
    let _e32: Rotor = other_625;
    return Rotor((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn rotor_multi_vector_add(self_762: Rotor, other_626: MultiVector) -> MultiVector {
    var self_763: Rotor;
    var other_627: MultiVector;

    self_763 = self_762;
    other_627 = other_626;
    let _e4: Rotor = self_763;
    let _e7: Rotor = self_763;
    let _e10: Rotor = self_763;
    let _e19: MultiVector = other_627;
    let _e22: MultiVector = other_627;
    let _e24: MultiVector = other_627;
    let _e26: MultiVector = other_627;
    let _e28: MultiVector = other_627;
    let _e30: MultiVector = other_627;
    let _e32: Rotor = self_763;
    let _e35: Rotor = self_763;
    let _e38: Rotor = self_763;
    let _e42: MultiVector = other_627;
    let _e45: MultiVector = other_627;
    let _e47: MultiVector = other_627;
    let _e49: MultiVector = other_627;
    return MultiVector(((vec3<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.w) * vec3<f32>(0.0, 0.0, 1.0)) + _e19.g0_), _e22.g1_, _e24.g2_, _e26.g3_, _e28.g4_, _e30.g5_, (vec3<f32>(_e32.g0_.x, _e35.g0_.y, _e38.g0_.z) + _e42.g6_), _e45.g7_, _e47.g8_, _e49.g9_);
}

fn rotor_multi_vector_sub(self_764: Rotor, other_628: MultiVector) -> MultiVector {
    var self_765: Rotor;
    var other_629: MultiVector;

    self_765 = self_764;
    other_629 = other_628;
    let _e4: Rotor = self_765;
    let _e7: Rotor = self_765;
    let _e10: Rotor = self_765;
    let _e19: MultiVector = other_629;
    let _e24: MultiVector = other_629;
    let _e29: MultiVector = other_629;
    let _e34: MultiVector = other_629;
    let _e39: MultiVector = other_629;
    let _e44: MultiVector = other_629;
    let _e47: Rotor = self_765;
    let _e50: Rotor = self_765;
    let _e53: Rotor = self_765;
    let _e57: MultiVector = other_629;
    let _e62: MultiVector = other_629;
    let _e67: MultiVector = other_629;
    let _e72: MultiVector = other_629;
    return MultiVector(((vec3<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.w) * vec3<f32>(0.0, 0.0, 1.0)) - _e19.g0_), (vec3<f32>(0.0) - _e24.g1_), (vec2<f32>(0.0) - _e29.g2_), (vec4<f32>(0.0) - _e34.g3_), (vec3<f32>(0.0) - _e39.g4_), (vec3<f32>(0.0) - _e44.g5_), (vec3<f32>(_e47.g0_.x, _e50.g0_.y, _e53.g0_.z) - _e57.g6_), (vec3<f32>(0.0) - _e62.g7_), (vec4<f32>(0.0) - _e67.g8_), (vec4<f32>(0.0) - _e72.g9_));
}

fn rotor_scale(self_766: Rotor, other_630: f32) -> Rotor {
    var self_767: Rotor;
    var other_631: f32;

    self_767 = self_766;
    other_631 = other_630;
    let _e5: f32 = other_631;
    let _e7: Rotor = self_767;
    let _e8: f32 = other_631;
    let _e10: Rotor = rotor_scalar_geometric_product(_e7, Scalar(_e8));
    return _e10;
}

fn translator_zero() -> Translator {
    return Translator(vec4<f32>(0.0));
}

fn translator_one() -> Translator {
    return Translator(vec4<f32>(0.0));
}

fn translator_neg(self_768: Translator) -> Translator {
    var self_769: Translator;

    self_769 = self_768;
    let _e2: Translator = self_769;
    return Translator((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn translator_automorphism(self_770: Translator) -> Translator {
    var self_771: Translator;

    self_771 = self_770;
    let _e2: Translator = self_771;
    return Translator((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn translator_reversal(self_772: Translator) -> Translator {
    var self_773: Translator;

    self_773 = self_772;
    let _e2: Translator = self_773;
    return Translator((_e2.g0_ * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0)));
}

fn translator_conjugation(self_774: Translator) -> Translator {
    var self_775: Translator;

    self_775 = self_774;
    let _e2: Translator = self_775;
    return Translator((_e2.g0_ * vec4<f32>(1.0, 1.0, 1.0, -(1.0))));
}

fn translator_scalar_geometric_product(self_776: Translator, other_632: Scalar) -> Translator {
    var self_777: Translator;
    var other_633: Scalar;

    self_777 = self_776;
    other_633 = other_632;
    let _e4: Translator = self_777;
    let _e6: Scalar = other_633;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn translator_scalar_outer_product(self_778: Translator, other_634: Scalar) -> Translator {
    var self_779: Translator;
    var other_635: Scalar;

    self_779 = self_778;
    other_635 = other_634;
    let _e4: Translator = self_779;
    let _e6: Scalar = other_635;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn translator_scalar_inner_product(self_780: Translator, other_636: Scalar) -> Translator {
    var self_781: Translator;
    var other_637: Scalar;

    self_781 = self_780;
    other_637 = other_636;
    let _e4: Translator = self_781;
    let _e6: Scalar = other_637;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn translator_scalar_right_contraction(self_782: Translator, other_638: Scalar) -> Translator {
    var self_783: Translator;
    var other_639: Scalar;

    self_783 = self_782;
    other_639 = other_638;
    let _e4: Translator = self_783;
    let _e6: Scalar = other_639;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn translator_anti_scalar_into(self_784: Translator) -> AntiScalar {
    var self_785: Translator;

    self_785 = self_784;
    let _e2: Translator = self_785;
    return AntiScalar(_e2.g0_.w);
}

fn translator_anti_scalar_add(self_786: Translator, other_640: AntiScalar) -> Translator {
    var self_787: Translator;
    var other_641: AntiScalar;

    self_787 = self_786;
    other_641 = other_640;
    let _e4: Translator = self_787;
    let _e6: AntiScalar = other_641;
    return Translator((_e4.g0_ + (vec4<f32>(_e6.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0))));
}

fn translator_anti_scalar_sub(self_788: Translator, other_642: AntiScalar) -> Translator {
    var self_789: Translator;
    var other_643: AntiScalar;

    self_789 = self_788;
    other_643 = other_642;
    let _e4: Translator = self_789;
    let _e6: AntiScalar = other_643;
    return Translator((_e4.g0_ - (vec4<f32>(_e6.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0))));
}

fn translator_radial_point_outer_product(self_790: Translator, other_644: RadialPoint) -> Plane {
    var self_791: Translator;
    var other_645: RadialPoint;

    self_791 = self_790;
    other_645 = other_644;
    let _e4: Translator = self_791;
    let _e8: RadialPoint = other_645;
    let _e11: RadialPoint = other_645;
    let _e14: RadialPoint = other_645;
    let _e17: RadialPoint = other_645;
    let _e29: Translator = self_791;
    let _e33: RadialPoint = other_645;
    let _e36: RadialPoint = other_645;
    let _e39: RadialPoint = other_645;
    let _e42: RadialPoint = other_645;
    let _e55: Translator = self_791;
    let _e59: RadialPoint = other_645;
    let _e62: RadialPoint = other_645;
    let _e65: RadialPoint = other_645;
    let _e68: RadialPoint = other_645;
    return Plane(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.x, _e11.g1_.x, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(0.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g1_.x, _e36.g1_.x, _e39.g1_.x, _e42.g0_.z)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e55.g0_.x) * vec4<f32>(_e59.g1_.x, _e62.g1_.x, _e65.g1_.x, _e68.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))));
}

fn translator_dipole_geometric_product(self_792: Translator, other_646: Dipole) -> Motor {
    var self_793: Translator;
    var other_647: Dipole;

    self_793 = self_792;
    other_647 = other_646;
    let _e4: Translator = self_793;
    let _e8: Dipole = other_647;
    let _e11: Dipole = other_647;
    let _e14: Dipole = other_647;
    let _e17: Dipole = other_647;
    let _e30: Translator = self_793;
    let _e34: Dipole = other_647;
    let _e37: Dipole = other_647;
    let _e40: Dipole = other_647;
    let _e43: Dipole = other_647;
    let _e57: Translator = self_793;
    let _e61: Dipole = other_647;
    let _e64: Dipole = other_647;
    let _e67: Dipole = other_647;
    let _e70: Dipole = other_647;
    let _e82: Translator = self_793;
    let _e86: Dipole = other_647;
    let _e89: Dipole = other_647;
    let _e92: Dipole = other_647;
    let _e95: Dipole = other_647;
    let _e109: Translator = self_793;
    let _e113: Dipole = other_647;
    let _e116: Dipole = other_647;
    let _e119: Dipole = other_647;
    let _e122: Dipole = other_647;
    let _e135: Translator = self_793;
    let _e139: Dipole = other_647;
    let _e142: Dipole = other_647;
    let _e145: Dipole = other_647;
    let _e148: Dipole = other_647;
    let _e162: Translator = self_793;
    let _e166: Dipole = other_647;
    let _e169: Dipole = other_647;
    let _e172: Dipole = other_647;
    let _e175: Dipole = other_647;
    return Motor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.y, _e37.g0_.x, _e40.g0_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g1_.x, _e64.g1_.y, _e67.g1_.z, _e70.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g0_.x) * vec4<f32>(_e86.g0_.x, _e89.g0_.z, _e92.g0_.y, _e95.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), ((((vec4<f32>(_e109.g0_.y) * vec4<f32>(_e113.g1_.z, _e116.g1_.z, _e119.g1_.x, _e122.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e135.g0_.z) * vec4<f32>(_e139.g1_.y, _e142.g1_.x, _e145.g1_.y, _e148.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e162.g0_.x) * vec4<f32>(_e166.g1_.x, _e169.g1_.z, _e172.g1_.y, _e175.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn translator_dipole_outer_product(self_794: Translator, other_648: Dipole) -> AntiScalar {
    var self_795: Translator;
    var other_649: Dipole;

    self_795 = self_794;
    other_649 = other_648;
    let _e5: Translator = self_795;
    let _e8: Dipole = other_649;
    let _e13: Translator = self_795;
    let _e16: Dipole = other_649;
    let _e21: Translator = self_795;
    let _e24: Dipole = other_649;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn translator_motor_add(self_796: Translator, other_650: Motor) -> Motor {
    var self_797: Translator;
    var other_651: Motor;

    self_797 = self_796;
    other_651 = other_650;
    let _e4: Translator = self_797;
    let _e13: Motor = other_651;
    let _e16: Translator = self_797;
    let _e25: Motor = other_651;
    return Motor(((_e4.g0_.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + _e13.g0_), ((_e16.g0_.xyzx * vec4<f32>(1.0, 1.0, 1.0, 0.0)) + _e25.g1_));
}

fn translator_motor_sub(self_798: Translator, other_652: Motor) -> Motor {
    var self_799: Translator;
    var other_653: Motor;

    self_799 = self_798;
    other_653 = other_652;
    let _e4: Translator = self_799;
    let _e13: Motor = other_653;
    let _e16: Translator = self_799;
    let _e25: Motor = other_653;
    return Motor(((_e4.g0_.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0)) - _e13.g0_), ((_e16.g0_.xyzx * vec4<f32>(1.0, 1.0, 1.0, 0.0)) - _e25.g1_));
}

fn translator_translator_add(self_800: Translator, other_654: Translator) -> Translator {
    var self_801: Translator;
    var other_655: Translator;

    self_801 = self_800;
    other_655 = other_654;
    let _e4: Translator = self_801;
    let _e6: Translator = other_655;
    return Translator((_e4.g0_ + _e6.g0_));
}

fn translator_translator_sub(self_802: Translator, other_656: Translator) -> Translator {
    var self_803: Translator;
    var other_657: Translator;

    self_803 = self_802;
    other_657 = other_656;
    let _e4: Translator = self_803;
    let _e6: Translator = other_657;
    return Translator((_e4.g0_ - _e6.g0_));
}

fn translator_translator_mul(self_804: Translator, other_658: Translator) -> Translator {
    var self_805: Translator;
    var other_659: Translator;

    self_805 = self_804;
    other_659 = other_658;
    let _e4: Translator = self_805;
    let _e6: Translator = other_659;
    return Translator((_e4.g0_ * _e6.g0_));
}

fn translator_translator_div(self_806: Translator, other_660: Translator) -> Translator {
    var self_807: Translator;
    var other_661: Translator;

    self_807 = self_806;
    other_661 = other_660;
    let _e4: Translator = self_807;
    let _e7: Translator = self_807;
    let _e10: Translator = self_807;
    let _e13: Translator = self_807;
    let _e23: Translator = other_661;
    let _e26: Translator = other_661;
    let _e29: Translator = other_661;
    let _e32: Translator = other_661;
    return Translator((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn translator_multi_vector_add(self_808: Translator, other_662: MultiVector) -> MultiVector {
    var self_809: Translator;
    var other_663: MultiVector;

    self_809 = self_808;
    other_663 = other_662;
    let _e4: Translator = self_809;
    let _e7: Translator = self_809;
    let _e10: Translator = self_809;
    let _e19: MultiVector = other_663;
    let _e22: MultiVector = other_663;
    let _e24: MultiVector = other_663;
    let _e26: MultiVector = other_663;
    let _e28: MultiVector = other_663;
    let _e30: MultiVector = other_663;
    let _e32: MultiVector = other_663;
    let _e34: Translator = self_809;
    let _e37: Translator = self_809;
    let _e40: Translator = self_809;
    let _e44: MultiVector = other_663;
    let _e47: MultiVector = other_663;
    let _e49: MultiVector = other_663;
    return MultiVector(((vec3<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.w) * vec3<f32>(0.0, 0.0, 1.0)) + _e19.g0_), _e22.g1_, _e24.g2_, _e26.g3_, _e28.g4_, _e30.g5_, _e32.g6_, (vec3<f32>(_e34.g0_.x, _e37.g0_.y, _e40.g0_.z) + _e44.g7_), _e47.g8_, _e49.g9_);
}

fn translator_multi_vector_sub(self_810: Translator, other_664: MultiVector) -> MultiVector {
    var self_811: Translator;
    var other_665: MultiVector;

    self_811 = self_810;
    other_665 = other_664;
    let _e4: Translator = self_811;
    let _e7: Translator = self_811;
    let _e10: Translator = self_811;
    let _e19: MultiVector = other_665;
    let _e24: MultiVector = other_665;
    let _e29: MultiVector = other_665;
    let _e34: MultiVector = other_665;
    let _e39: MultiVector = other_665;
    let _e44: MultiVector = other_665;
    let _e49: MultiVector = other_665;
    let _e52: Translator = self_811;
    let _e55: Translator = self_811;
    let _e58: Translator = self_811;
    let _e62: MultiVector = other_665;
    let _e67: MultiVector = other_665;
    let _e72: MultiVector = other_665;
    return MultiVector(((vec3<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.w) * vec3<f32>(0.0, 0.0, 1.0)) - _e19.g0_), (vec3<f32>(0.0) - _e24.g1_), (vec2<f32>(0.0) - _e29.g2_), (vec4<f32>(0.0) - _e34.g3_), (vec3<f32>(0.0) - _e39.g4_), (vec3<f32>(0.0) - _e44.g5_), (vec3<f32>(0.0) - _e49.g6_), (vec3<f32>(_e52.g0_.x, _e55.g0_.y, _e58.g0_.z) - _e62.g7_), (vec4<f32>(0.0) - _e67.g8_), (vec4<f32>(0.0) - _e72.g9_));
}

fn translator_scale(self_812: Translator, other_666: f32) -> Translator {
    var self_813: Translator;
    var other_667: f32;

    self_813 = self_812;
    other_667 = other_666;
    let _e5: f32 = other_667;
    let _e7: Translator = self_813;
    let _e8: f32 = other_667;
    let _e10: Translator = translator_scalar_geometric_product(_e7, Scalar(_e8));
    return _e10;
}

fn flector_zero() -> Flector {
    return Flector(vec4<f32>(0.0), vec4<f32>(0.0));
}

fn flector_one() -> Flector {
    return Flector(vec4<f32>(0.0), vec4<f32>(0.0));
}

fn flector_neg(self_814: Flector) -> Flector {
    var self_815: Flector;

    self_815 = self_814;
    let _e2: Flector = self_815;
    let _e8: Flector = self_815;
    return Flector((_e2.g0_ * vec4<f32>(-(1.0))), (_e8.g1_ * vec4<f32>(-(1.0))));
}

fn flector_automorphism(self_816: Flector) -> Flector {
    var self_817: Flector;

    self_817 = self_816;
    let _e2: Flector = self_817;
    let _e4: Flector = self_817;
    return Flector(_e2.g0_, _e4.g1_);
}

fn flector_reversal(self_818: Flector) -> Flector {
    var self_819: Flector;

    self_819 = self_818;
    let _e2: Flector = self_819;
    let _e8: Flector = self_819;
    return Flector((_e2.g0_ * vec4<f32>(-(1.0))), _e8.g1_);
}

fn flector_conjugation(self_820: Flector) -> Flector {
    var self_821: Flector;

    self_821 = self_820;
    let _e2: Flector = self_821;
    let _e8: Flector = self_821;
    return Flector((_e2.g0_ * vec4<f32>(-(1.0))), _e8.g1_);
}

fn flector_scalar_geometric_product(self_822: Flector, other_668: Scalar) -> Flector {
    var self_823: Flector;
    var other_669: Scalar;

    self_823 = self_822;
    other_669 = other_668;
    let _e4: Flector = self_823;
    let _e6: Scalar = other_669;
    let _e10: Flector = self_823;
    let _e12: Scalar = other_669;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn flector_scalar_outer_product(self_824: Flector, other_670: Scalar) -> Flector {
    var self_825: Flector;
    var other_671: Scalar;

    self_825 = self_824;
    other_671 = other_670;
    let _e4: Flector = self_825;
    let _e6: Scalar = other_671;
    let _e10: Flector = self_825;
    let _e12: Scalar = other_671;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn flector_scalar_inner_product(self_826: Flector, other_672: Scalar) -> Flector {
    var self_827: Flector;
    var other_673: Scalar;

    self_827 = self_826;
    other_673 = other_672;
    let _e4: Flector = self_827;
    let _e6: Scalar = other_673;
    let _e10: Flector = self_827;
    let _e12: Scalar = other_673;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn flector_scalar_right_contraction(self_828: Flector, other_674: Scalar) -> Flector {
    var self_829: Flector;
    var other_675: Scalar;

    self_829 = self_828;
    other_675 = other_674;
    let _e4: Flector = self_829;
    let _e6: Scalar = other_675;
    let _e10: Flector = self_829;
    let _e12: Scalar = other_675;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn flector_radial_point_geometric_product(self_830: Flector, other_676: RadialPoint) -> Motor {
    var self_831: Flector;
    var other_677: RadialPoint;

    self_831 = self_830;
    other_677 = other_676;
    let _e4: Flector = self_831;
    let _e8: RadialPoint = other_677;
    let _e11: RadialPoint = other_677;
    let _e14: RadialPoint = other_677;
    let _e17: RadialPoint = other_677;
    let _e31: Flector = self_831;
    let _e35: RadialPoint = other_677;
    let _e38: RadialPoint = other_677;
    let _e41: RadialPoint = other_677;
    let _e44: RadialPoint = other_677;
    let _e57: Flector = self_831;
    let _e61: RadialPoint = other_677;
    let _e64: RadialPoint = other_677;
    let _e67: RadialPoint = other_677;
    let _e70: RadialPoint = other_677;
    let _e83: Flector = self_831;
    let _e87: RadialPoint = other_677;
    let _e90: RadialPoint = other_677;
    let _e93: RadialPoint = other_677;
    let _e96: RadialPoint = other_677;
    let _e109: Flector = self_831;
    let _e112: Flector = self_831;
    let _e115: Flector = self_831;
    let _e118: Flector = self_831;
    let _e122: RadialPoint = other_677;
    let _e128: Flector = self_831;
    let _e132: RadialPoint = other_677;
    let _e135: RadialPoint = other_677;
    let _e138: RadialPoint = other_677;
    let _e141: RadialPoint = other_677;
    let _e154: Flector = self_831;
    let _e158: RadialPoint = other_677;
    let _e161: RadialPoint = other_677;
    let _e164: RadialPoint = other_677;
    let _e167: RadialPoint = other_677;
    let _e181: Flector = self_831;
    let _e185: RadialPoint = other_677;
    let _e188: RadialPoint = other_677;
    let _e191: RadialPoint = other_677;
    let _e194: RadialPoint = other_677;
    let _e206: Flector = self_831;
    let _e210: RadialPoint = other_677;
    let _e213: RadialPoint = other_677;
    let _e216: RadialPoint = other_677;
    let _e219: RadialPoint = other_677;
    return Motor(((((((vec4<f32>(_e4.g0_.w) * vec4<f32>(_e8.g0_.x, _e11.g0_.y, _e14.g0_.z, _e17.g0_.x)) * vec4<f32>(-(1.0), -(1.0), -(1.0), 0.0)) + ((vec4<f32>(_e31.g1_.x) * vec4<f32>(_e35.g0_.z, _e38.g0_.z, _e41.g0_.y, _e44.g0_.x)) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e57.g1_.y) * vec4<f32>(_e61.g0_.z, _e64.g0_.z, _e67.g0_.x, _e70.g0_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e83.g1_.z) * vec4<f32>(_e87.g0_.y, _e90.g0_.x, _e93.g0_.y, _e96.g0_.z)) * vec4<f32>(-(1.0), 1.0, 0.0, 1.0))) + (vec4<f32>(_e109.g0_.x, _e112.g0_.y, _e115.g0_.z, _e118.g1_.w) * vec4<f32>(_e122.g1_.x))), (((((vec4<f32>(_e128.g0_.y) * vec4<f32>(_e132.g0_.z, _e135.g0_.z, _e138.g0_.x, _e141.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e154.g0_.z) * vec4<f32>(_e158.g0_.y, _e161.g0_.x, _e164.g0_.y, _e167.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e181.g1_.w) * vec4<f32>(_e185.g0_.x, _e188.g0_.y, _e191.g0_.z, _e194.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e206.g0_.x) * vec4<f32>(_e210.g0_.x, _e213.g0_.z, _e216.g0_.y, _e219.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn flector_flat_point_into(self_832: Flector) -> FlatPoint {
    var self_833: Flector;

    self_833 = self_832;
    let _e2: Flector = self_833;
    return FlatPoint(_e2.g0_);
}

fn flector_flat_point_add(self_834: Flector, other_678: FlatPoint) -> Flector {
    var self_835: Flector;
    var other_679: FlatPoint;

    self_835 = self_834;
    other_679 = other_678;
    let _e4: Flector = self_835;
    let _e6: FlatPoint = other_679;
    let _e9: Flector = self_835;
    return Flector((_e4.g0_ + _e6.g0_), _e9.g1_);
}

fn flector_flat_point_sub(self_836: Flector, other_680: FlatPoint) -> Flector {
    var self_837: Flector;
    var other_681: FlatPoint;

    self_837 = self_836;
    other_681 = other_680;
    let _e4: Flector = self_837;
    let _e6: FlatPoint = other_681;
    let _e9: Flector = self_837;
    return Flector((_e4.g0_ - _e6.g0_), _e9.g1_);
}

fn flector_dipole_geometric_product(self_838: Flector, other_682: Dipole) -> Flector {
    var self_839: Flector;
    var other_683: Dipole;

    self_839 = self_838;
    other_683 = other_682;
    let _e4: Flector = self_839;
    let _e8: Dipole = other_683;
    let _e11: Dipole = other_683;
    let _e14: Dipole = other_683;
    let _e17: Dipole = other_683;
    let _e30: Flector = self_839;
    let _e34: Dipole = other_683;
    let _e37: Dipole = other_683;
    let _e40: Dipole = other_683;
    let _e43: Dipole = other_683;
    let _e57: Flector = self_839;
    let _e61: Dipole = other_683;
    let _e74: Flector = self_839;
    let _e78: Dipole = other_683;
    let _e91: Flector = self_839;
    let _e95: Dipole = other_683;
    let _e108: Flector = self_839;
    let _e112: Dipole = other_683;
    let _e115: Dipole = other_683;
    let _e118: Dipole = other_683;
    let _e121: Dipole = other_683;
    let _e133: Flector = self_839;
    let _e137: Dipole = other_683;
    let _e140: Dipole = other_683;
    let _e143: Dipole = other_683;
    let _e146: Dipole = other_683;
    let _e160: Flector = self_839;
    let _e164: Dipole = other_683;
    let _e167: Dipole = other_683;
    let _e170: Dipole = other_683;
    let _e173: Dipole = other_683;
    let _e186: Flector = self_839;
    let _e190: Dipole = other_683;
    let _e193: Dipole = other_683;
    let _e196: Dipole = other_683;
    let _e199: Dipole = other_683;
    let _e213: Flector = self_839;
    let _e217: Dipole = other_683;
    let _e220: Dipole = other_683;
    let _e223: Dipole = other_683;
    let _e226: Dipole = other_683;
    let _e238: Flector = self_839;
    let _e242: Dipole = other_683;
    let _e245: Dipole = other_683;
    let _e248: Dipole = other_683;
    let _e251: Dipole = other_683;
    let _e264: Flector = self_839;
    let _e268: Dipole = other_683;
    let _e271: Dipole = other_683;
    let _e274: Dipole = other_683;
    let _e277: Dipole = other_683;
    let _e290: Flector = self_839;
    let _e294: Dipole = other_683;
    let _e297: Dipole = other_683;
    let _e300: Dipole = other_683;
    let _e303: Dipole = other_683;
    let _e316: Flector = self_839;
    let _e320: Dipole = other_683;
    let _e323: Dipole = other_683;
    let _e326: Dipole = other_683;
    let _e329: Dipole = other_683;
    let _e341: Flector = self_839;
    let _e345: Dipole = other_683;
    let _e348: Dipole = other_683;
    let _e351: Dipole = other_683;
    let _e354: Dipole = other_683;
    return Flector(((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g1_.x) * vec4<f32>(_e61.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e74.g1_.y) * vec4<f32>(_e78.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e91.g1_.z) * vec4<f32>(_e95.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e108.g1_.w) * vec4<f32>(_e112.g1_.x, _e115.g1_.y, _e118.g1_.z, _e121.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e133.g0_.x) * vec4<f32>(_e137.g1_.x, _e140.g1_.z, _e143.g1_.y, _e146.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), (((((((((vec4<f32>(_e160.g0_.y) * vec4<f32>(_e164.g0_.z, _e167.g0_.z, _e170.g0_.x, _e173.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e186.g0_.z) * vec4<f32>(_e190.g0_.y, _e193.g0_.x, _e196.g0_.y, _e199.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e213.g0_.w) * vec4<f32>(_e217.g1_.x, _e220.g1_.y, _e223.g1_.z, _e226.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e238.g1_.x) * vec4<f32>(_e242.g1_.z, _e245.g1_.z, _e248.g1_.y, _e251.g1_.z)) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e264.g1_.y) * vec4<f32>(_e268.g1_.z, _e271.g1_.z, _e274.g1_.x, _e277.g1_.z)) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e290.g1_.z) * vec4<f32>(_e294.g1_.y, _e297.g1_.x, _e300.g1_.y, _e303.g1_.y)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e316.g1_.w) * vec4<f32>(_e320.g0_.x, _e323.g0_.y, _e326.g0_.z, _e329.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e341.g0_.x) * vec4<f32>(_e345.g0_.x, _e348.g0_.z, _e351.g0_.y, _e354.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn flector_dipole_outer_product(self_840: Flector, other_684: Dipole) -> Plane {
    var self_841: Flector;
    var other_685: Dipole;

    self_841 = self_840;
    other_685 = other_684;
    let _e4: Flector = self_841;
    let _e8: Dipole = other_685;
    let _e11: Dipole = other_685;
    let _e14: Dipole = other_685;
    let _e17: Dipole = other_685;
    let _e30: Flector = self_841;
    let _e34: Dipole = other_685;
    let _e37: Dipole = other_685;
    let _e40: Dipole = other_685;
    let _e43: Dipole = other_685;
    let _e57: Flector = self_841;
    let _e61: Dipole = other_685;
    let _e64: Dipole = other_685;
    let _e67: Dipole = other_685;
    let _e70: Dipole = other_685;
    let _e82: Flector = self_841;
    let _e86: Dipole = other_685;
    let _e89: Dipole = other_685;
    let _e92: Dipole = other_685;
    let _e95: Dipole = other_685;
    return Plane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.y, _e37.g0_.x, _e40.g0_.y, _e43.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g1_.x, _e64.g1_.y, _e67.g1_.z, _e70.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g0_.x) * vec4<f32>(_e86.g0_.x, _e89.g0_.z, _e92.g0_.y, _e95.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn flector_dipole_inner_product(self_842: Flector, other_686: Dipole) -> FlatPoint {
    var self_843: Flector;
    var other_687: Dipole;

    self_843 = self_842;
    other_687 = other_686;
    let _e4: Flector = self_843;
    let _e8: Dipole = other_687;
    let _e20: Flector = self_843;
    let _e24: Dipole = other_687;
    let _e37: Flector = self_843;
    let _e40: Dipole = other_687;
    let _e43: Dipole = other_687;
    let _e46: Dipole = other_687;
    let _e49: Dipole = other_687;
    return FlatPoint(((((vec4<f32>(_e4.g1_.y) * vec4<f32>(_e8.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g1_.z) * vec4<f32>(_e24.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g1_.wwwx * vec4<f32>(_e40.g1_.x, _e43.g1_.y, _e46.g1_.z, _e49.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn flector_dipole_right_contraction(self_844: Flector, other_688: Dipole) -> FlatPoint {
    var self_845: Flector;
    var other_689: Dipole;

    self_845 = self_844;
    other_689 = other_688;
    let _e4: Flector = self_845;
    let _e8: Dipole = other_689;
    let _e20: Flector = self_845;
    let _e24: Dipole = other_689;
    let _e37: Flector = self_845;
    let _e40: Dipole = other_689;
    let _e43: Dipole = other_689;
    let _e46: Dipole = other_689;
    let _e49: Dipole = other_689;
    return FlatPoint(((((vec4<f32>(_e4.g1_.y) * vec4<f32>(_e8.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g1_.z) * vec4<f32>(_e24.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g1_.wwwx * vec4<f32>(_e40.g1_.x, _e43.g1_.y, _e46.g1_.z, _e49.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn flector_circle_geometric_product(self_846: Flector, other_690: Circle) -> Motor {
    var self_847: Flector;
    var other_691: Circle;

    self_847 = self_846;
    other_691 = other_690;
    let _e4: Flector = self_847;
    let _e8: Circle = other_691;
    let _e20: Flector = self_847;
    let _e24: Circle = other_691;
    let _e37: Flector = self_847;
    let _e41: Circle = other_691;
    let _e54: Flector = self_847;
    let _e58: Circle = other_691;
    let _e71: Flector = self_847;
    let _e75: Circle = other_691;
    let _e88: Flector = self_847;
    let _e92: Circle = other_691;
    let _e103: Flector = self_847;
    let _e106: Flector = self_847;
    let _e109: Flector = self_847;
    let _e112: Flector = self_847;
    let _e116: Circle = other_691;
    let _e130: Flector = self_847;
    let _e133: Flector = self_847;
    let _e136: Flector = self_847;
    let _e139: Flector = self_847;
    let _e143: Circle = other_691;
    return Motor(((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxy) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * _e24.g0_.yxyz) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e37.g0_.w) * vec4<f32>(_e41.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e54.g1_.y) * vec4<f32>(_e58.g0_.w)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e71.g1_.z) * vec4<f32>(_e75.g0_.w)) * vec4<f32>(0.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e88.g1_.w) * _e92.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e103.g1_.x, _e106.g0_.x, _e109.g0_.x, _e112.g0_.x) * _e116.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))), (vec4<f32>(_e130.g0_.x, _e133.g0_.y, _e136.g0_.z, _e139.g1_.w) * vec4<f32>(_e143.g0_.w)));
}

fn flector_circle_outer_product(self_848: Flector, other_692: Circle) -> AntiScalar {
    var self_849: Flector;
    var other_693: Circle;

    self_849 = self_848;
    other_693 = other_692;
    let _e5: Flector = self_849;
    let _e8: Circle = other_693;
    let _e13: Flector = self_849;
    let _e16: Circle = other_693;
    let _e21: Flector = self_849;
    let _e24: Circle = other_693;
    let _e29: Flector = self_849;
    let _e32: Circle = other_693;
    return AntiScalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g0_.w * _e32.g0_.w)));
}

fn flector_plane_into(self_850: Flector) -> Plane {
    var self_851: Flector;

    self_851 = self_850;
    let _e2: Flector = self_851;
    return Plane(_e2.g1_);
}

fn flector_plane_add(self_852: Flector, other_694: Plane) -> Flector {
    var self_853: Flector;
    var other_695: Plane;

    self_853 = self_852;
    other_695 = other_694;
    let _e4: Flector = self_853;
    let _e6: Flector = self_853;
    let _e8: Plane = other_695;
    return Flector(_e4.g0_, (_e6.g1_ + _e8.g0_));
}

fn flector_plane_sub(self_854: Flector, other_696: Plane) -> Flector {
    var self_855: Flector;
    var other_697: Plane;

    self_855 = self_854;
    other_697 = other_696;
    let _e4: Flector = self_855;
    let _e6: Flector = self_855;
    let _e8: Plane = other_697;
    return Flector(_e4.g0_, (_e6.g1_ - _e8.g0_));
}

fn flector_flector_add(self_856: Flector, other_698: Flector) -> Flector {
    var self_857: Flector;
    var other_699: Flector;

    self_857 = self_856;
    other_699 = other_698;
    let _e4: Flector = self_857;
    let _e6: Flector = other_699;
    let _e9: Flector = self_857;
    let _e11: Flector = other_699;
    return Flector((_e4.g0_ + _e6.g0_), (_e9.g1_ + _e11.g1_));
}

fn flector_flector_sub(self_858: Flector, other_700: Flector) -> Flector {
    var self_859: Flector;
    var other_701: Flector;

    self_859 = self_858;
    other_701 = other_700;
    let _e4: Flector = self_859;
    let _e6: Flector = other_701;
    let _e9: Flector = self_859;
    let _e11: Flector = other_701;
    return Flector((_e4.g0_ - _e6.g0_), (_e9.g1_ - _e11.g1_));
}

fn flector_flector_mul(self_860: Flector, other_702: Flector) -> Flector {
    var self_861: Flector;
    var other_703: Flector;

    self_861 = self_860;
    other_703 = other_702;
    let _e4: Flector = self_861;
    let _e6: Flector = other_703;
    let _e9: Flector = self_861;
    let _e11: Flector = other_703;
    return Flector((_e4.g0_ * _e6.g0_), (_e9.g1_ * _e11.g1_));
}

fn flector_flector_div(self_862: Flector, other_704: Flector) -> Flector {
    var self_863: Flector;
    var other_705: Flector;

    self_863 = self_862;
    other_705 = other_704;
    let _e4: Flector = self_863;
    let _e7: Flector = self_863;
    let _e10: Flector = self_863;
    let _e13: Flector = self_863;
    let _e23: Flector = other_705;
    let _e26: Flector = other_705;
    let _e29: Flector = other_705;
    let _e32: Flector = other_705;
    let _e43: Flector = self_863;
    let _e46: Flector = self_863;
    let _e49: Flector = self_863;
    let _e52: Flector = self_863;
    let _e62: Flector = other_705;
    let _e65: Flector = other_705;
    let _e68: Flector = other_705;
    let _e71: Flector = other_705;
    return Flector((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)), (((vec4<f32>(_e43.g1_.x, _e46.g1_.y, _e49.g1_.z, _e52.g1_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e62.g1_.x, _e65.g1_.y, _e68.g1_.z, _e71.g1_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn flector_multi_vector_add(self_864: Flector, other_706: MultiVector) -> MultiVector {
    var self_865: Flector;
    var other_707: MultiVector;

    self_865 = self_864;
    other_707 = other_706;
    let _e4: MultiVector = other_707;
    let _e6: MultiVector = other_707;
    let _e8: MultiVector = other_707;
    let _e10: Flector = self_865;
    let _e12: MultiVector = other_707;
    let _e15: MultiVector = other_707;
    let _e17: MultiVector = other_707;
    let _e19: MultiVector = other_707;
    let _e21: MultiVector = other_707;
    let _e23: MultiVector = other_707;
    let _e25: Flector = self_865;
    let _e27: MultiVector = other_707;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, (_e10.g0_ + _e12.g3_), _e15.g4_, _e17.g5_, _e19.g6_, _e21.g7_, _e23.g8_, (_e25.g1_ + _e27.g9_));
}

fn flector_multi_vector_sub(self_866: Flector, other_708: MultiVector) -> MultiVector {
    var self_867: Flector;
    var other_709: MultiVector;

    self_867 = self_866;
    other_709 = other_708;
    let _e6: MultiVector = other_709;
    let _e11: MultiVector = other_709;
    let _e16: MultiVector = other_709;
    let _e19: Flector = self_867;
    let _e21: MultiVector = other_709;
    let _e26: MultiVector = other_709;
    let _e31: MultiVector = other_709;
    let _e36: MultiVector = other_709;
    let _e41: MultiVector = other_709;
    let _e46: MultiVector = other_709;
    let _e49: Flector = self_867;
    let _e51: MultiVector = other_709;
    return MultiVector((vec3<f32>(0.0) - _e6.g0_), (vec3<f32>(0.0) - _e11.g1_), (vec2<f32>(0.0) - _e16.g2_), (_e19.g0_ - _e21.g3_), (vec3<f32>(0.0) - _e26.g4_), (vec3<f32>(0.0) - _e31.g5_), (vec3<f32>(0.0) - _e36.g6_), (vec3<f32>(0.0) - _e41.g7_), (vec4<f32>(0.0) - _e46.g8_), (_e49.g1_ - _e51.g9_));
}

fn flector_scale(self_868: Flector, other_710: f32) -> Flector {
    var self_869: Flector;
    var other_711: f32;

    self_869 = self_868;
    other_711 = other_710;
    let _e5: f32 = other_711;
    let _e7: Flector = self_869;
    let _e8: f32 = other_711;
    let _e10: Flector = flector_scalar_geometric_product(_e7, Scalar(_e8));
    return _e10;
}

fn multi_vector_zero() -> MultiVector {
    return MultiVector(vec3<f32>(0.0), vec3<f32>(0.0), vec2<f32>(0.0), vec4<f32>(0.0), vec3<f32>(0.0), vec3<f32>(0.0), vec3<f32>(0.0), vec3<f32>(0.0), vec4<f32>(0.0), vec4<f32>(0.0));
}

fn multi_vector_one() -> MultiVector {
    return MultiVector(vec3<f32>(1.0, 0.0, 0.0), vec3<f32>(0.0), vec2<f32>(0.0), vec4<f32>(0.0), vec3<f32>(0.0), vec3<f32>(0.0), vec3<f32>(0.0), vec3<f32>(0.0), vec4<f32>(0.0), vec4<f32>(0.0));
}

fn multi_vector_neg(self_870: MultiVector) -> MultiVector {
    var self_871: MultiVector;

    self_871 = self_870;
    let _e2: MultiVector = self_871;
    let _e8: MultiVector = self_871;
    let _e14: MultiVector = self_871;
    let _e20: MultiVector = self_871;
    let _e26: MultiVector = self_871;
    let _e32: MultiVector = self_871;
    let _e38: MultiVector = self_871;
    let _e44: MultiVector = self_871;
    let _e50: MultiVector = self_871;
    let _e56: MultiVector = self_871;
    return MultiVector((_e2.g0_ * vec3<f32>(-(1.0))), (_e8.g1_ * vec3<f32>(-(1.0))), (_e14.g2_ * vec2<f32>(-(1.0))), (_e20.g3_ * vec4<f32>(-(1.0))), (_e26.g4_ * vec3<f32>(-(1.0))), (_e32.g5_ * vec3<f32>(-(1.0))), (_e38.g6_ * vec3<f32>(-(1.0))), (_e44.g7_ * vec3<f32>(-(1.0))), (_e50.g8_ * vec4<f32>(-(1.0))), (_e56.g9_ * vec4<f32>(-(1.0))));
}

fn multi_vector_automorphism(self_872: MultiVector) -> MultiVector {
    var self_873: MultiVector;

    self_873 = self_872;
    let _e2: MultiVector = self_873;
    let _e10: MultiVector = self_873;
    let _e16: MultiVector = self_873;
    let _e22: MultiVector = self_873;
    let _e24: MultiVector = self_873;
    let _e26: MultiVector = self_873;
    let _e28: MultiVector = self_873;
    let _e34: MultiVector = self_873;
    let _e40: MultiVector = self_873;
    let _e46: MultiVector = self_873;
    return MultiVector((_e2.g0_ * vec3<f32>(1.0, 1.0, -(1.0))), (_e10.g1_ * vec3<f32>(-(1.0))), (_e16.g2_ * vec2<f32>(-(1.0))), _e22.g3_, _e24.g4_, _e26.g5_, (_e28.g6_ * vec3<f32>(-(1.0))), (_e34.g7_ * vec3<f32>(-(1.0))), (_e40.g8_ * vec4<f32>(-(1.0))), _e46.g9_);
}

fn multi_vector_reversal(self_874: MultiVector) -> MultiVector {
    var self_875: MultiVector;

    self_875 = self_874;
    let _e2: MultiVector = self_875;
    let _e4: MultiVector = self_875;
    let _e6: MultiVector = self_875;
    let _e8: MultiVector = self_875;
    let _e14: MultiVector = self_875;
    let _e20: MultiVector = self_875;
    let _e26: MultiVector = self_875;
    let _e32: MultiVector = self_875;
    let _e38: MultiVector = self_875;
    let _e44: MultiVector = self_875;
    return MultiVector(_e2.g0_, _e4.g1_, _e6.g2_, (_e8.g3_ * vec4<f32>(-(1.0))), (_e14.g4_ * vec3<f32>(-(1.0))), (_e20.g5_ * vec3<f32>(-(1.0))), (_e26.g6_ * vec3<f32>(-(1.0))), (_e32.g7_ * vec3<f32>(-(1.0))), (_e38.g8_ * vec4<f32>(-(1.0))), _e44.g9_);
}

fn multi_vector_conjugation(self_876: MultiVector) -> MultiVector {
    var self_877: MultiVector;

    self_877 = self_876;
    let _e2: MultiVector = self_877;
    let _e10: MultiVector = self_877;
    let _e16: MultiVector = self_877;
    let _e22: MultiVector = self_877;
    let _e28: MultiVector = self_877;
    let _e34: MultiVector = self_877;
    let _e40: MultiVector = self_877;
    let _e42: MultiVector = self_877;
    let _e44: MultiVector = self_877;
    let _e46: MultiVector = self_877;
    return MultiVector((_e2.g0_ * vec3<f32>(1.0, 1.0, -(1.0))), (_e10.g1_ * vec3<f32>(-(1.0))), (_e16.g2_ * vec2<f32>(-(1.0))), (_e22.g3_ * vec4<f32>(-(1.0))), (_e28.g4_ * vec3<f32>(-(1.0))), (_e34.g5_ * vec3<f32>(-(1.0))), _e40.g6_, _e42.g7_, _e44.g8_, _e46.g9_);
}

fn multi_vector_scalar_into(self_878: MultiVector) -> Scalar {
    var self_879: MultiVector;

    self_879 = self_878;
    let _e2: MultiVector = self_879;
    return Scalar(_e2.g0_.x);
}

fn multi_vector_scalar_add(self_880: MultiVector, other_712: Scalar) -> MultiVector {
    var self_881: MultiVector;
    var other_713: Scalar;

    self_881 = self_880;
    other_713 = other_712;
    let _e4: MultiVector = self_881;
    let _e6: Scalar = other_713;
    let _e15: MultiVector = self_881;
    let _e17: MultiVector = self_881;
    let _e19: MultiVector = self_881;
    let _e21: MultiVector = self_881;
    let _e23: MultiVector = self_881;
    let _e25: MultiVector = self_881;
    let _e27: MultiVector = self_881;
    let _e29: MultiVector = self_881;
    let _e31: MultiVector = self_881;
    return MultiVector((_e4.g0_ + (vec3<f32>(_e6.g0_) * vec3<f32>(1.0, 0.0, 0.0))), _e15.g1_, _e17.g2_, _e19.g3_, _e21.g4_, _e23.g5_, _e25.g6_, _e27.g7_, _e29.g8_, _e31.g9_);
}

fn multi_vector_scalar_sub(self_882: MultiVector, other_714: Scalar) -> MultiVector {
    var self_883: MultiVector;
    var other_715: Scalar;

    self_883 = self_882;
    other_715 = other_714;
    let _e4: MultiVector = self_883;
    let _e6: Scalar = other_715;
    let _e15: MultiVector = self_883;
    let _e17: MultiVector = self_883;
    let _e19: MultiVector = self_883;
    let _e21: MultiVector = self_883;
    let _e23: MultiVector = self_883;
    let _e25: MultiVector = self_883;
    let _e27: MultiVector = self_883;
    let _e29: MultiVector = self_883;
    let _e31: MultiVector = self_883;
    return MultiVector((_e4.g0_ - (vec3<f32>(_e6.g0_) * vec3<f32>(1.0, 0.0, 0.0))), _e15.g1_, _e17.g2_, _e19.g3_, _e21.g4_, _e23.g5_, _e25.g6_, _e27.g7_, _e29.g8_, _e31.g9_);
}

fn multi_vector_scalar_geometric_product(self_884: MultiVector, other_716: Scalar) -> MultiVector {
    var self_885: MultiVector;
    var other_717: Scalar;

    self_885 = self_884;
    other_717 = other_716;
    let _e4: MultiVector = self_885;
    let _e6: Scalar = other_717;
    let _e10: MultiVector = self_885;
    let _e12: Scalar = other_717;
    let _e16: MultiVector = self_885;
    let _e18: Scalar = other_717;
    let _e22: MultiVector = self_885;
    let _e24: Scalar = other_717;
    let _e28: MultiVector = self_885;
    let _e30: Scalar = other_717;
    let _e34: MultiVector = self_885;
    let _e36: Scalar = other_717;
    let _e40: MultiVector = self_885;
    let _e42: Scalar = other_717;
    let _e46: MultiVector = self_885;
    let _e48: Scalar = other_717;
    let _e52: MultiVector = self_885;
    let _e54: Scalar = other_717;
    let _e58: MultiVector = self_885;
    let _e60: Scalar = other_717;
    return MultiVector((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)), (_e16.g2_ * vec2<f32>(_e18.g0_)), (_e22.g3_ * vec4<f32>(_e24.g0_)), (_e28.g4_ * vec3<f32>(_e30.g0_)), (_e34.g5_ * vec3<f32>(_e36.g0_)), (_e40.g6_ * vec3<f32>(_e42.g0_)), (_e46.g7_ * vec3<f32>(_e48.g0_)), (_e52.g8_ * vec4<f32>(_e54.g0_)), (_e58.g9_ * vec4<f32>(_e60.g0_)));
}

fn multi_vector_scalar_outer_product(self_886: MultiVector, other_718: Scalar) -> MultiVector {
    var self_887: MultiVector;
    var other_719: Scalar;

    self_887 = self_886;
    other_719 = other_718;
    let _e4: MultiVector = self_887;
    let _e6: Scalar = other_719;
    let _e10: MultiVector = self_887;
    let _e12: Scalar = other_719;
    let _e16: MultiVector = self_887;
    let _e18: Scalar = other_719;
    let _e22: MultiVector = self_887;
    let _e24: Scalar = other_719;
    let _e28: MultiVector = self_887;
    let _e30: Scalar = other_719;
    let _e34: MultiVector = self_887;
    let _e36: Scalar = other_719;
    let _e40: MultiVector = self_887;
    let _e42: Scalar = other_719;
    let _e46: MultiVector = self_887;
    let _e48: Scalar = other_719;
    let _e52: MultiVector = self_887;
    let _e54: Scalar = other_719;
    let _e58: MultiVector = self_887;
    let _e60: Scalar = other_719;
    return MultiVector((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)), (_e16.g2_ * vec2<f32>(_e18.g0_)), (_e22.g3_ * vec4<f32>(_e24.g0_)), (_e28.g4_ * vec3<f32>(_e30.g0_)), (_e34.g5_ * vec3<f32>(_e36.g0_)), (_e40.g6_ * vec3<f32>(_e42.g0_)), (_e46.g7_ * vec3<f32>(_e48.g0_)), (_e52.g8_ * vec4<f32>(_e54.g0_)), (_e58.g9_ * vec4<f32>(_e60.g0_)));
}

fn multi_vector_scalar_inner_product(self_888: MultiVector, other_720: Scalar) -> MultiVector {
    var self_889: MultiVector;
    var other_721: Scalar;

    self_889 = self_888;
    other_721 = other_720;
    let _e4: MultiVector = self_889;
    let _e6: Scalar = other_721;
    let _e10: MultiVector = self_889;
    let _e12: Scalar = other_721;
    let _e16: MultiVector = self_889;
    let _e18: Scalar = other_721;
    let _e22: MultiVector = self_889;
    let _e24: Scalar = other_721;
    let _e28: MultiVector = self_889;
    let _e30: Scalar = other_721;
    let _e34: MultiVector = self_889;
    let _e36: Scalar = other_721;
    let _e40: MultiVector = self_889;
    let _e42: Scalar = other_721;
    let _e46: MultiVector = self_889;
    let _e48: Scalar = other_721;
    let _e52: MultiVector = self_889;
    let _e54: Scalar = other_721;
    let _e58: MultiVector = self_889;
    let _e60: Scalar = other_721;
    return MultiVector((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)), (_e16.g2_ * vec2<f32>(_e18.g0_)), (_e22.g3_ * vec4<f32>(_e24.g0_)), (_e28.g4_ * vec3<f32>(_e30.g0_)), (_e34.g5_ * vec3<f32>(_e36.g0_)), (_e40.g6_ * vec3<f32>(_e42.g0_)), (_e46.g7_ * vec3<f32>(_e48.g0_)), (_e52.g8_ * vec4<f32>(_e54.g0_)), (_e58.g9_ * vec4<f32>(_e60.g0_)));
}

fn multi_vector_scalar_left_contraction(self_890: MultiVector, other_722: Scalar) -> Scalar {
    var self_891: MultiVector;
    var other_723: Scalar;

    self_891 = self_890;
    other_723 = other_722;
    let _e4: MultiVector = self_891;
    let _e7: Scalar = other_723;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn multi_vector_scalar_right_contraction(self_892: MultiVector, other_724: Scalar) -> MultiVector {
    var self_893: MultiVector;
    var other_725: Scalar;

    self_893 = self_892;
    other_725 = other_724;
    let _e4: MultiVector = self_893;
    let _e6: Scalar = other_725;
    let _e10: MultiVector = self_893;
    let _e12: Scalar = other_725;
    let _e16: MultiVector = self_893;
    let _e18: Scalar = other_725;
    let _e22: MultiVector = self_893;
    let _e24: Scalar = other_725;
    let _e28: MultiVector = self_893;
    let _e30: Scalar = other_725;
    let _e34: MultiVector = self_893;
    let _e36: Scalar = other_725;
    let _e40: MultiVector = self_893;
    let _e42: Scalar = other_725;
    let _e46: MultiVector = self_893;
    let _e48: Scalar = other_725;
    let _e52: MultiVector = self_893;
    let _e54: Scalar = other_725;
    let _e58: MultiVector = self_893;
    let _e60: Scalar = other_725;
    return MultiVector((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)), (_e16.g2_ * vec2<f32>(_e18.g0_)), (_e22.g3_ * vec4<f32>(_e24.g0_)), (_e28.g4_ * vec3<f32>(_e30.g0_)), (_e34.g5_ * vec3<f32>(_e36.g0_)), (_e40.g6_ * vec3<f32>(_e42.g0_)), (_e46.g7_ * vec3<f32>(_e48.g0_)), (_e52.g8_ * vec4<f32>(_e54.g0_)), (_e58.g9_ * vec4<f32>(_e60.g0_)));
}

fn multi_vector_scalar_scalar_product(self_894: MultiVector, other_726: Scalar) -> Scalar {
    var self_895: MultiVector;
    var other_727: Scalar;

    self_895 = self_894;
    other_727 = other_726;
    let _e4: MultiVector = self_895;
    let _e7: Scalar = other_727;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn multi_vector_anti_scalar_into(self_896: MultiVector) -> AntiScalar {
    var self_897: MultiVector;

    self_897 = self_896;
    let _e2: MultiVector = self_897;
    return AntiScalar(_e2.g0_.z);
}

fn multi_vector_anti_scalar_add(self_898: MultiVector, other_728: AntiScalar) -> MultiVector {
    var self_899: MultiVector;
    var other_729: AntiScalar;

    self_899 = self_898;
    other_729 = other_728;
    let _e4: MultiVector = self_899;
    let _e6: AntiScalar = other_729;
    let _e15: MultiVector = self_899;
    let _e17: MultiVector = self_899;
    let _e19: MultiVector = self_899;
    let _e21: MultiVector = self_899;
    let _e23: MultiVector = self_899;
    let _e25: MultiVector = self_899;
    let _e27: MultiVector = self_899;
    let _e29: MultiVector = self_899;
    let _e31: MultiVector = self_899;
    return MultiVector((_e4.g0_ + (vec3<f32>(_e6.g0_) * vec3<f32>(0.0, 0.0, 1.0))), _e15.g1_, _e17.g2_, _e19.g3_, _e21.g4_, _e23.g5_, _e25.g6_, _e27.g7_, _e29.g8_, _e31.g9_);
}

fn multi_vector_anti_scalar_sub(self_900: MultiVector, other_730: AntiScalar) -> MultiVector {
    var self_901: MultiVector;
    var other_731: AntiScalar;

    self_901 = self_900;
    other_731 = other_730;
    let _e4: MultiVector = self_901;
    let _e6: AntiScalar = other_731;
    let _e15: MultiVector = self_901;
    let _e17: MultiVector = self_901;
    let _e19: MultiVector = self_901;
    let _e21: MultiVector = self_901;
    let _e23: MultiVector = self_901;
    let _e25: MultiVector = self_901;
    let _e27: MultiVector = self_901;
    let _e29: MultiVector = self_901;
    let _e31: MultiVector = self_901;
    return MultiVector((_e4.g0_ - (vec3<f32>(_e6.g0_) * vec3<f32>(0.0, 0.0, 1.0))), _e15.g1_, _e17.g2_, _e19.g3_, _e21.g4_, _e23.g5_, _e25.g6_, _e27.g7_, _e29.g8_, _e31.g9_);
}

fn multi_vector_anti_scalar_outer_product(self_902: MultiVector, other_732: AntiScalar) -> AntiScalar {
    var self_903: MultiVector;
    var other_733: AntiScalar;

    self_903 = self_902;
    other_733 = other_732;
    let _e4: MultiVector = self_903;
    let _e7: AntiScalar = other_733;
    return AntiScalar((_e4.g0_.x * _e7.g0_));
}

fn multi_vector_radial_point_into(self_904: MultiVector) -> RadialPoint {
    var self_905: MultiVector;

    self_905 = self_904;
    let _e2: MultiVector = self_905;
    let _e4: MultiVector = self_905;
    return RadialPoint(_e2.g1_, _e4.g2_);
}

fn multi_vector_radial_point_add(self_906: MultiVector, other_734: RadialPoint) -> MultiVector {
    var self_907: MultiVector;
    var other_735: RadialPoint;

    self_907 = self_906;
    other_735 = other_734;
    let _e4: MultiVector = self_907;
    let _e6: MultiVector = self_907;
    let _e8: RadialPoint = other_735;
    let _e11: MultiVector = self_907;
    let _e13: RadialPoint = other_735;
    let _e16: MultiVector = self_907;
    let _e18: MultiVector = self_907;
    let _e20: MultiVector = self_907;
    let _e22: MultiVector = self_907;
    let _e24: MultiVector = self_907;
    let _e26: MultiVector = self_907;
    let _e28: MultiVector = self_907;
    return MultiVector(_e4.g0_, (_e6.g1_ + _e8.g0_), (_e11.g2_ + _e13.g1_), _e16.g3_, _e18.g4_, _e20.g5_, _e22.g6_, _e24.g7_, _e26.g8_, _e28.g9_);
}

fn multi_vector_radial_point_sub(self_908: MultiVector, other_736: RadialPoint) -> MultiVector {
    var self_909: MultiVector;
    var other_737: RadialPoint;

    self_909 = self_908;
    other_737 = other_736;
    let _e4: MultiVector = self_909;
    let _e6: MultiVector = self_909;
    let _e8: RadialPoint = other_737;
    let _e11: MultiVector = self_909;
    let _e13: RadialPoint = other_737;
    let _e16: MultiVector = self_909;
    let _e18: MultiVector = self_909;
    let _e20: MultiVector = self_909;
    let _e22: MultiVector = self_909;
    let _e24: MultiVector = self_909;
    let _e26: MultiVector = self_909;
    let _e28: MultiVector = self_909;
    return MultiVector(_e4.g0_, (_e6.g1_ - _e8.g0_), (_e11.g2_ - _e13.g1_), _e16.g3_, _e18.g4_, _e20.g5_, _e22.g6_, _e24.g7_, _e26.g8_, _e28.g9_);
}

fn multi_vector_radial_point_geometric_product(self_910: MultiVector, other_738: RadialPoint) -> MultiVector {
    var self_911: MultiVector;
    var other_739: RadialPoint;

    self_911 = self_910;
    other_739 = other_738;
    let _e4: MultiVector = self_911;
    let _e8: RadialPoint = other_739;
    let _e18: MultiVector = self_911;
    let _e22: RadialPoint = other_739;
    let _e33: MultiVector = self_911;
    let _e37: RadialPoint = other_739;
    let _e48: MultiVector = self_911;
    let _e52: RadialPoint = other_739;
    let _e64: MultiVector = self_911;
    let _e68: RadialPoint = other_739;
    let _e80: MultiVector = self_911;
    let _e84: RadialPoint = other_739;
    let _e96: MultiVector = self_911;
    let _e100: RadialPoint = other_739;
    let _e112: MultiVector = self_911;
    let _e116: RadialPoint = other_739;
    let _e127: MultiVector = self_911;
    let _e131: RadialPoint = other_739;
    let _e142: MultiVector = self_911;
    let _e146: RadialPoint = other_739;
    let _e157: MultiVector = self_911;
    let _e161: RadialPoint = other_739;
    let _e172: MultiVector = self_911;
    let _e175: RadialPoint = other_739;
    let _e178: RadialPoint = other_739;
    let _e181: RadialPoint = other_739;
    let _e192: MultiVector = self_911;
    let _e196: RadialPoint = other_739;
    let _e199: MultiVector = self_911;
    let _e203: RadialPoint = other_739;
    let _e214: MultiVector = self_911;
    let _e218: RadialPoint = other_739;
    let _e229: MultiVector = self_911;
    let _e233: RadialPoint = other_739;
    let _e244: MultiVector = self_911;
    let _e248: RadialPoint = other_739;
    let _e251: MultiVector = self_911;
    let _e255: RadialPoint = other_739;
    let _e266: MultiVector = self_911;
    let _e270: RadialPoint = other_739;
    let _e281: MultiVector = self_911;
    let _e285: RadialPoint = other_739;
    let _e295: MultiVector = self_911;
    let _e299: RadialPoint = other_739;
    let _e309: MultiVector = self_911;
    let _e313: RadialPoint = other_739;
    let _e323: MultiVector = self_911;
    let _e327: RadialPoint = other_739;
    let _e338: MultiVector = self_911;
    let _e342: RadialPoint = other_739;
    let _e353: MultiVector = self_911;
    let _e357: RadialPoint = other_739;
    let _e360: RadialPoint = other_739;
    let _e363: RadialPoint = other_739;
    let _e366: RadialPoint = other_739;
    let _e372: MultiVector = self_911;
    let _e376: RadialPoint = other_739;
    let _e389: MultiVector = self_911;
    let _e393: RadialPoint = other_739;
    let _e406: MultiVector = self_911;
    let _e410: RadialPoint = other_739;
    let _e423: MultiVector = self_911;
    let _e427: RadialPoint = other_739;
    let _e430: RadialPoint = other_739;
    let _e433: RadialPoint = other_739;
    let _e436: RadialPoint = other_739;
    let _e449: MultiVector = self_911;
    let _e453: RadialPoint = other_739;
    let _e456: RadialPoint = other_739;
    let _e459: RadialPoint = other_739;
    let _e462: RadialPoint = other_739;
    let _e475: MultiVector = self_911;
    let _e479: RadialPoint = other_739;
    let _e482: RadialPoint = other_739;
    let _e485: RadialPoint = other_739;
    let _e488: RadialPoint = other_739;
    let _e501: MultiVector = self_911;
    let _e504: MultiVector = self_911;
    let _e507: MultiVector = self_911;
    let _e510: MultiVector = self_911;
    let _e514: RadialPoint = other_739;
    let _e517: RadialPoint = other_739;
    let _e520: RadialPoint = other_739;
    let _e523: RadialPoint = other_739;
    let _e535: MultiVector = self_911;
    let _e539: RadialPoint = other_739;
    let _e542: MultiVector = self_911;
    let _e546: RadialPoint = other_739;
    let _e557: MultiVector = self_911;
    let _e561: RadialPoint = other_739;
    let _e572: MultiVector = self_911;
    let _e576: RadialPoint = other_739;
    let _e587: MultiVector = self_911;
    let _e589: RadialPoint = other_739;
    let _e599: MultiVector = self_911;
    let _e603: RadialPoint = other_739;
    let _e613: MultiVector = self_911;
    let _e617: RadialPoint = other_739;
    let _e628: MultiVector = self_911;
    let _e632: RadialPoint = other_739;
    let _e636: MultiVector = self_911;
    let _e640: RadialPoint = other_739;
    let _e653: MultiVector = self_911;
    let _e657: RadialPoint = other_739;
    let _e661: MultiVector = self_911;
    let _e665: RadialPoint = other_739;
    let _e676: MultiVector = self_911;
    let _e680: RadialPoint = other_739;
    let _e691: MultiVector = self_911;
    let _e695: RadialPoint = other_739;
    let _e706: MultiVector = self_911;
    let _e710: RadialPoint = other_739;
    let _e721: MultiVector = self_911;
    let _e725: RadialPoint = other_739;
    let _e736: MultiVector = self_911;
    let _e740: RadialPoint = other_739;
    let _e751: MultiVector = self_911;
    let _e754: MultiVector = self_911;
    let _e757: MultiVector = self_911;
    let _e761: RadialPoint = other_739;
    let _e767: MultiVector = self_911;
    let _e771: RadialPoint = other_739;
    let _e781: MultiVector = self_911;
    let _e785: RadialPoint = other_739;
    let _e796: MultiVector = self_911;
    let _e800: RadialPoint = other_739;
    let _e811: MultiVector = self_911;
    let _e815: RadialPoint = other_739;
    let _e826: MultiVector = self_911;
    let _e830: RadialPoint = other_739;
    let _e841: MultiVector = self_911;
    let _e845: RadialPoint = other_739;
    let _e849: MultiVector = self_911;
    let _e853: RadialPoint = other_739;
    let _e864: MultiVector = self_911;
    let _e868: RadialPoint = other_739;
    let _e871: RadialPoint = other_739;
    let _e874: RadialPoint = other_739;
    let _e877: RadialPoint = other_739;
    let _e889: MultiVector = self_911;
    let _e893: RadialPoint = other_739;
    let _e896: RadialPoint = other_739;
    let _e899: RadialPoint = other_739;
    let _e902: RadialPoint = other_739;
    let _e915: MultiVector = self_911;
    let _e919: RadialPoint = other_739;
    let _e922: RadialPoint = other_739;
    let _e925: RadialPoint = other_739;
    let _e928: RadialPoint = other_739;
    let _e941: MultiVector = self_911;
    let _e945: RadialPoint = other_739;
    let _e948: RadialPoint = other_739;
    let _e951: RadialPoint = other_739;
    let _e954: RadialPoint = other_739;
    let _e967: MultiVector = self_911;
    let _e971: RadialPoint = other_739;
    let _e974: RadialPoint = other_739;
    let _e977: RadialPoint = other_739;
    let _e980: RadialPoint = other_739;
    let _e993: MultiVector = self_911;
    let _e997: RadialPoint = other_739;
    let _e1000: RadialPoint = other_739;
    let _e1003: RadialPoint = other_739;
    let _e1006: RadialPoint = other_739;
    let _e1019: MultiVector = self_911;
    let _e1022: MultiVector = self_911;
    let _e1025: MultiVector = self_911;
    let _e1028: MultiVector = self_911;
    let _e1032: RadialPoint = other_739;
    let _e1035: RadialPoint = other_739;
    let _e1038: RadialPoint = other_739;
    let _e1041: RadialPoint = other_739;
    let _e1056: MultiVector = self_911;
    let _e1060: RadialPoint = other_739;
    let _e1063: RadialPoint = other_739;
    let _e1066: RadialPoint = other_739;
    let _e1069: RadialPoint = other_739;
    let _e1081: MultiVector = self_911;
    let _e1085: RadialPoint = other_739;
    let _e1088: RadialPoint = other_739;
    let _e1091: RadialPoint = other_739;
    let _e1094: RadialPoint = other_739;
    let _e1107: MultiVector = self_911;
    let _e1111: RadialPoint = other_739;
    let _e1114: RadialPoint = other_739;
    let _e1117: RadialPoint = other_739;
    let _e1120: RadialPoint = other_739;
    let _e1133: MultiVector = self_911;
    let _e1137: RadialPoint = other_739;
    let _e1140: RadialPoint = other_739;
    let _e1143: RadialPoint = other_739;
    let _e1146: RadialPoint = other_739;
    let _e1159: MultiVector = self_911;
    let _e1163: RadialPoint = other_739;
    let _e1166: RadialPoint = other_739;
    let _e1169: RadialPoint = other_739;
    let _e1172: RadialPoint = other_739;
    let _e1185: MultiVector = self_911;
    let _e1189: RadialPoint = other_739;
    let _e1192: RadialPoint = other_739;
    let _e1195: RadialPoint = other_739;
    let _e1198: RadialPoint = other_739;
    let _e1211: MultiVector = self_911;
    let _e1215: RadialPoint = other_739;
    let _e1227: MultiVector = self_911;
    let _e1231: RadialPoint = other_739;
    let _e1243: MultiVector = self_911;
    let _e1247: RadialPoint = other_739;
    let _e1259: MultiVector = self_911;
    let _e1263: RadialPoint = other_739;
    let _e1275: MultiVector = self_911;
    let _e1278: MultiVector = self_911;
    let _e1281: MultiVector = self_911;
    let _e1284: MultiVector = self_911;
    let _e1288: RadialPoint = other_739;
    let _e1291: RadialPoint = other_739;
    let _e1294: RadialPoint = other_739;
    let _e1297: RadialPoint = other_739;
    return MultiVector((((((((((((((vec3<f32>(_e4.g1_.x) * vec3<f32>(_e8.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0)) + ((vec3<f32>(_e18.g1_.y) * vec3<f32>(_e22.g0_.y)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e33.g1_.z) * vec3<f32>(_e37.g0_.z)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e48.g8_.x) * vec3<f32>(_e52.g0_.x)) * vec3<f32>(0.0, -(1.0), 0.0))) + ((vec3<f32>(_e64.g8_.y) * vec3<f32>(_e68.g0_.y)) * vec3<f32>(0.0, -(1.0), 0.0))) + ((vec3<f32>(_e80.g8_.z) * vec3<f32>(_e84.g0_.z)) * vec3<f32>(0.0, -(1.0), 0.0))) + ((vec3<f32>(_e96.g8_.w) * vec3<f32>(_e100.g1_.x)) * vec3<f32>(0.0, -(1.0), 0.0))) + ((vec3<f32>(_e112.g9_.x) * vec3<f32>(_e116.g0_.x)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e127.g9_.y) * vec3<f32>(_e131.g0_.y)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e142.g9_.z) * vec3<f32>(_e146.g0_.z)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e157.g9_.w) * vec3<f32>(_e161.g1_.x)) * vec3<f32>(0.0, 0.0, 1.0))) + ((_e172.g0_.xxy * vec3<f32>(_e175.g1_.x, _e178.g1_.x, _e181.g1_.y)) * vec3<f32>(0.0, 0.0, 1.0))), ((((vec3<f32>(_e192.g0_.x) * _e196.g0_) + ((vec3<f32>(_e199.g5_.y) * _e203.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e214.g5_.z) * _e218.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e229.g5_.x) * _e233.g0_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))), (((((((vec2<f32>(_e244.g0_.x) * _e248.g1_) + ((vec2<f32>(_e251.g3_.y) * vec2<f32>(_e255.g0_.y)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e266.g3_.z) * vec2<f32>(_e270.g0_.z)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e281.g4_.x) * vec2<f32>(_e285.g0_.x)) * vec2<f32>(1.0, 0.0))) + ((vec2<f32>(_e295.g4_.y) * vec2<f32>(_e299.g0_.y)) * vec2<f32>(1.0, 0.0))) + ((vec2<f32>(_e309.g4_.z) * vec2<f32>(_e313.g0_.z)) * vec2<f32>(1.0, 0.0))) + ((vec2<f32>(_e323.g3_.x) * vec2<f32>(_e327.g0_.x)) * vec2<f32>(0.0, -(1.0)))), ((((((((((vec4<f32>(_e338.g2_.x) * vec4<f32>(_e342.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) - (vec4<f32>(_e353.g2_.y) * vec4<f32>(_e357.g0_.x, _e360.g0_.y, _e363.g0_.z, _e366.g1_.x))) + ((vec4<f32>(_e372.g6_.x) * vec4<f32>(_e376.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e389.g6_.y) * vec4<f32>(_e393.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e406.g6_.z) * vec4<f32>(_e410.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e423.g7_.x) * vec4<f32>(_e427.g0_.z, _e430.g0_.z, _e433.g0_.y, _e436.g0_.z)) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e449.g7_.y) * vec4<f32>(_e453.g0_.z, _e456.g0_.z, _e459.g0_.x, _e462.g0_.z)) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e475.g7_.z) * vec4<f32>(_e479.g0_.y, _e482.g0_.x, _e485.g0_.y, _e488.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e501.g1_.x, _e504.g1_.y, _e507.g1_.z, _e510.g1_.x) * vec4<f32>(_e514.g1_.y, _e517.g1_.y, _e520.g1_.y, _e523.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), (((((vec3<f32>(_e535.g2_.x) * _e539.g0_) + ((vec3<f32>(_e542.g8_.x) * _e546.g0_.zzy) * vec3<f32>(0.0, 1.0, -(1.0)))) + ((vec3<f32>(_e557.g8_.y) * _e561.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e572.g8_.z) * _e576.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((_e587.g1_ * vec3<f32>(_e589.g1_.x)) * vec3<f32>(-(1.0)))), (((((vec3<f32>(_e599.g1_.y) * _e603.g0_.zzx) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e613.g1_.z) * _e617.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) - (vec3<f32>(_e628.g8_.w) * _e632.g0_)) + ((vec3<f32>(_e636.g1_.x) * _e640.g0_.xzy) * vec3<f32>(0.0, -(1.0), 1.0))), ((((((((vec3<f32>(0.0) - (vec3<f32>(_e653.g3_.w) * _e657.g0_)) + ((vec3<f32>(_e661.g4_.x) * vec3<f32>(_e665.g1_.y)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e676.g4_.y) * vec3<f32>(_e680.g1_.y)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e691.g4_.z) * vec3<f32>(_e695.g1_.y)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e706.g9_.x) * _e710.g0_.zzy) * vec3<f32>(0.0, -(1.0), 1.0))) + ((vec3<f32>(_e721.g9_.y) * _e725.g0_.zzx) * vec3<f32>(1.0, 0.0, -(1.0)))) + ((vec3<f32>(_e736.g9_.z) * _e740.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + (vec3<f32>(_e751.g3_.x, _e754.g3_.y, _e757.g3_.z) * vec3<f32>(_e761.g1_.x))), ((((((((vec3<f32>(_e767.g3_.y) * _e771.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e781.g3_.z) * _e785.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e796.g5_.x) * vec3<f32>(_e800.g1_.y)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e811.g5_.y) * vec3<f32>(_e815.g1_.y)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e826.g5_.z) * vec3<f32>(_e830.g1_.y)) * vec3<f32>(0.0, 0.0, 1.0))) + (vec3<f32>(_e841.g9_.w) * _e845.g0_)) + ((vec3<f32>(_e849.g3_.x) * _e853.g0_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))), ((((((((vec4<f32>(_e864.g4_.x) * vec4<f32>(_e868.g0_.z, _e871.g0_.z, _e874.g0_.y, _e877.g0_.z)) * vec4<f32>(0.0, -(1.0), 1.0, 0.0)) + ((vec4<f32>(_e889.g4_.y) * vec4<f32>(_e893.g0_.z, _e896.g0_.z, _e899.g0_.x, _e902.g0_.z)) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e915.g4_.z) * vec4<f32>(_e919.g0_.y, _e922.g0_.x, _e925.g0_.y, _e928.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e941.g5_.x) * vec4<f32>(_e945.g1_.x, _e948.g1_.x, _e951.g1_.x, _e954.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e967.g5_.y) * vec4<f32>(_e971.g1_.x, _e974.g1_.x, _e977.g1_.x, _e980.g0_.y)) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e993.g5_.z) * vec4<f32>(_e997.g1_.x, _e1000.g1_.x, _e1003.g1_.x, _e1006.g0_.z)) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e1019.g0_.y, _e1022.g0_.y, _e1025.g0_.y, _e1028.g0_.x) * vec4<f32>(_e1032.g0_.x, _e1035.g0_.y, _e1038.g0_.z, _e1041.g0_.x)) * vec4<f32>(-(1.0), -(1.0), -(1.0), 0.0))), ((((((((((((vec4<f32>(_e1056.g6_.x) * vec4<f32>(_e1060.g0_.z, _e1063.g0_.z, _e1066.g0_.y, _e1069.g0_.z)) * vec4<f32>(0.0, 1.0, -(1.0), 0.0)) + ((vec4<f32>(_e1081.g6_.y) * vec4<f32>(_e1085.g0_.z, _e1088.g0_.z, _e1091.g0_.x, _e1094.g0_.z)) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e1107.g6_.z) * vec4<f32>(_e1111.g0_.y, _e1114.g0_.x, _e1117.g0_.y, _e1120.g0_.y)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e1133.g7_.x) * vec4<f32>(_e1137.g1_.x, _e1140.g1_.x, _e1143.g1_.x, _e1146.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e1159.g7_.y) * vec4<f32>(_e1163.g1_.x, _e1166.g1_.x, _e1169.g1_.x, _e1172.g0_.y)) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e1185.g7_.z) * vec4<f32>(_e1189.g1_.x, _e1192.g1_.x, _e1195.g1_.x, _e1198.g0_.z)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e1211.g8_.x) * vec4<f32>(_e1215.g1_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e1227.g8_.y) * vec4<f32>(_e1231.g1_.y)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e1243.g8_.z) * vec4<f32>(_e1247.g1_.y)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e1259.g8_.w) * vec4<f32>(_e1263.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e1275.g0_.z, _e1278.g0_.z, _e1281.g0_.z, _e1284.g0_.x) * vec4<f32>(_e1288.g0_.x, _e1291.g0_.y, _e1294.g0_.z, _e1297.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn multi_vector_radial_point_scalar_product(self_912: MultiVector, other_740: RadialPoint) -> Scalar {
    var self_913: MultiVector;
    var other_741: RadialPoint;

    self_913 = self_912;
    other_741 = other_740;
    let _e4: MultiVector = self_913;
    let _e7: RadialPoint = other_741;
    let _e11: MultiVector = self_913;
    let _e14: RadialPoint = other_741;
    let _e19: MultiVector = self_913;
    let _e22: RadialPoint = other_741;
    return Scalar((((_e4.g1_.x * _e7.g0_.x) + (_e11.g1_.y * _e14.g0_.y)) + (_e19.g1_.z * _e22.g0_.z)));
}

fn multi_vector_flat_point_into(self_914: MultiVector) -> FlatPoint {
    var self_915: MultiVector;

    self_915 = self_914;
    let _e2: MultiVector = self_915;
    return FlatPoint(_e2.g3_);
}

fn multi_vector_flat_point_add(self_916: MultiVector, other_742: FlatPoint) -> MultiVector {
    var self_917: MultiVector;
    var other_743: FlatPoint;

    self_917 = self_916;
    other_743 = other_742;
    let _e4: MultiVector = self_917;
    let _e6: MultiVector = self_917;
    let _e8: MultiVector = self_917;
    let _e10: MultiVector = self_917;
    let _e12: FlatPoint = other_743;
    let _e15: MultiVector = self_917;
    let _e17: MultiVector = self_917;
    let _e19: MultiVector = self_917;
    let _e21: MultiVector = self_917;
    let _e23: MultiVector = self_917;
    let _e25: MultiVector = self_917;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, (_e10.g3_ + _e12.g0_), _e15.g4_, _e17.g5_, _e19.g6_, _e21.g7_, _e23.g8_, _e25.g9_);
}

fn multi_vector_flat_point_sub(self_918: MultiVector, other_744: FlatPoint) -> MultiVector {
    var self_919: MultiVector;
    var other_745: FlatPoint;

    self_919 = self_918;
    other_745 = other_744;
    let _e4: MultiVector = self_919;
    let _e6: MultiVector = self_919;
    let _e8: MultiVector = self_919;
    let _e10: MultiVector = self_919;
    let _e12: FlatPoint = other_745;
    let _e15: MultiVector = self_919;
    let _e17: MultiVector = self_919;
    let _e19: MultiVector = self_919;
    let _e21: MultiVector = self_919;
    let _e23: MultiVector = self_919;
    let _e25: MultiVector = self_919;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, (_e10.g3_ - _e12.g0_), _e15.g4_, _e17.g5_, _e19.g6_, _e21.g7_, _e23.g8_, _e25.g9_);
}

fn multi_vector_dipole_into(self_920: MultiVector) -> Dipole {
    var self_921: MultiVector;

    self_921 = self_920;
    let _e2: MultiVector = self_921;
    let _e4: MultiVector = self_921;
    let _e6: MultiVector = self_921;
    return Dipole(_e2.g4_, _e4.g5_, _e6.g3_);
}

fn multi_vector_dipole_add(self_922: MultiVector, other_746: Dipole) -> MultiVector {
    var self_923: MultiVector;
    var other_747: Dipole;

    self_923 = self_922;
    other_747 = other_746;
    let _e4: MultiVector = self_923;
    let _e6: MultiVector = self_923;
    let _e8: MultiVector = self_923;
    let _e10: MultiVector = self_923;
    let _e12: Dipole = other_747;
    let _e15: MultiVector = self_923;
    let _e17: Dipole = other_747;
    let _e20: MultiVector = self_923;
    let _e22: Dipole = other_747;
    let _e25: MultiVector = self_923;
    let _e27: MultiVector = self_923;
    let _e29: MultiVector = self_923;
    let _e31: MultiVector = self_923;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, (_e10.g3_ + _e12.g2_), (_e15.g4_ + _e17.g0_), (_e20.g5_ + _e22.g1_), _e25.g6_, _e27.g7_, _e29.g8_, _e31.g9_);
}

fn multi_vector_dipole_sub(self_924: MultiVector, other_748: Dipole) -> MultiVector {
    var self_925: MultiVector;
    var other_749: Dipole;

    self_925 = self_924;
    other_749 = other_748;
    let _e4: MultiVector = self_925;
    let _e6: MultiVector = self_925;
    let _e8: MultiVector = self_925;
    let _e10: MultiVector = self_925;
    let _e12: Dipole = other_749;
    let _e15: MultiVector = self_925;
    let _e17: Dipole = other_749;
    let _e20: MultiVector = self_925;
    let _e22: Dipole = other_749;
    let _e25: MultiVector = self_925;
    let _e27: MultiVector = self_925;
    let _e29: MultiVector = self_925;
    let _e31: MultiVector = self_925;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, (_e10.g3_ - _e12.g2_), (_e15.g4_ - _e17.g0_), (_e20.g5_ - _e22.g1_), _e25.g6_, _e27.g7_, _e29.g8_, _e31.g9_);
}

fn multi_vector_dipole_geometric_product(self_926: MultiVector, other_750: Dipole) -> MultiVector {
    var self_927: MultiVector;
    var other_751: Dipole;

    self_927 = self_926;
    other_751 = other_750;
    let _e4: MultiVector = self_927;
    let _e8: Dipole = other_751;
    let _e19: MultiVector = self_927;
    let _e23: Dipole = other_751;
    let _e35: MultiVector = self_927;
    let _e39: Dipole = other_751;
    let _e42: Dipole = other_751;
    let _e45: Dipole = other_751;
    let _e58: MultiVector = self_927;
    let _e62: Dipole = other_751;
    let _e65: Dipole = other_751;
    let _e68: Dipole = other_751;
    let _e81: MultiVector = self_927;
    let _e85: Dipole = other_751;
    let _e88: Dipole = other_751;
    let _e91: Dipole = other_751;
    let _e104: MultiVector = self_927;
    let _e108: Dipole = other_751;
    let _e120: MultiVector = self_927;
    let _e124: Dipole = other_751;
    let _e136: MultiVector = self_927;
    let _e140: Dipole = other_751;
    let _e152: MultiVector = self_927;
    let _e156: Dipole = other_751;
    let _e168: MultiVector = self_927;
    let _e172: Dipole = other_751;
    let _e184: MultiVector = self_927;
    let _e188: Dipole = other_751;
    let _e200: MultiVector = self_927;
    let _e204: Dipole = other_751;
    let _e216: MultiVector = self_927;
    let _e220: Dipole = other_751;
    let _e232: MultiVector = self_927;
    let _e236: Dipole = other_751;
    let _e248: MultiVector = self_927;
    let _e251: MultiVector = self_927;
    let _e254: MultiVector = self_927;
    let _e258: Dipole = other_751;
    let _e271: MultiVector = self_927;
    let _e275: Dipole = other_751;
    let _e285: MultiVector = self_927;
    let _e289: Dipole = other_751;
    let _e300: MultiVector = self_927;
    let _e304: Dipole = other_751;
    let _e308: MultiVector = self_927;
    let _e312: Dipole = other_751;
    let _e323: MultiVector = self_927;
    let _e327: Dipole = other_751;
    let _e330: Dipole = other_751;
    let _e340: MultiVector = self_927;
    let _e344: Dipole = other_751;
    let _e347: Dipole = other_751;
    let _e358: MultiVector = self_927;
    let _e362: Dipole = other_751;
    let _e365: Dipole = other_751;
    let _e376: MultiVector = self_927;
    let _e380: Dipole = other_751;
    let _e391: MultiVector = self_927;
    let _e395: Dipole = other_751;
    let _e406: MultiVector = self_927;
    let _e410: Dipole = other_751;
    let _e421: MultiVector = self_927;
    let _e425: Dipole = other_751;
    let _e436: MultiVector = self_927;
    let _e440: Dipole = other_751;
    let _e451: MultiVector = self_927;
    let _e455: Dipole = other_751;
    let _e466: MultiVector = self_927;
    let _e470: Dipole = other_751;
    let _e473: MultiVector = self_927;
    let _e477: Dipole = other_751;
    let _e480: Dipole = other_751;
    let _e483: Dipole = other_751;
    let _e486: Dipole = other_751;
    let _e500: MultiVector = self_927;
    let _e504: Dipole = other_751;
    let _e507: Dipole = other_751;
    let _e510: Dipole = other_751;
    let _e513: Dipole = other_751;
    let _e527: MultiVector = self_927;
    let _e531: Dipole = other_751;
    let _e543: MultiVector = self_927;
    let _e547: Dipole = other_751;
    let _e559: MultiVector = self_927;
    let _e563: Dipole = other_751;
    let _e575: MultiVector = self_927;
    let _e579: Dipole = other_751;
    let _e591: MultiVector = self_927;
    let _e595: Dipole = other_751;
    let _e607: MultiVector = self_927;
    let _e611: Dipole = other_751;
    let _e623: MultiVector = self_927;
    let _e627: Dipole = other_751;
    let _e640: MultiVector = self_927;
    let _e644: Dipole = other_751;
    let _e657: MultiVector = self_927;
    let _e661: Dipole = other_751;
    let _e674: MultiVector = self_927;
    let _e678: Dipole = other_751;
    let _e681: Dipole = other_751;
    let _e684: Dipole = other_751;
    let _e687: Dipole = other_751;
    let _e699: MultiVector = self_927;
    let _e703: Dipole = other_751;
    let _e706: Dipole = other_751;
    let _e709: Dipole = other_751;
    let _e712: Dipole = other_751;
    let _e726: MultiVector = self_927;
    let _e730: Dipole = other_751;
    let _e733: MultiVector = self_927;
    let _e737: Dipole = other_751;
    let _e741: MultiVector = self_927;
    let _e745: Dipole = other_751;
    let _e756: MultiVector = self_927;
    let _e760: Dipole = other_751;
    let _e771: MultiVector = self_927;
    let _e775: Dipole = other_751;
    let _e786: MultiVector = self_927;
    let _e790: Dipole = other_751;
    let _e801: MultiVector = self_927;
    let _e805: Dipole = other_751;
    let _e816: MultiVector = self_927;
    let _e820: Dipole = other_751;
    let _e831: MultiVector = self_927;
    let _e835: Dipole = other_751;
    let _e838: MultiVector = self_927;
    let _e842: Dipole = other_751;
    let _e853: MultiVector = self_927;
    let _e857: Dipole = other_751;
    let _e868: MultiVector = self_927;
    let _e872: Dipole = other_751;
    let _e883: MultiVector = self_927;
    let _e887: Dipole = other_751;
    let _e890: MultiVector = self_927;
    let _e894: Dipole = other_751;
    let _e897: Dipole = other_751;
    let _e900: Dipole = other_751;
    let _e906: MultiVector = self_927;
    let _e910: Dipole = other_751;
    let _e914: MultiVector = self_927;
    let _e918: Dipole = other_751;
    let _e929: MultiVector = self_927;
    let _e933: Dipole = other_751;
    let _e944: MultiVector = self_927;
    let _e948: Dipole = other_751;
    let _e959: MultiVector = self_927;
    let _e963: Dipole = other_751;
    let _e974: MultiVector = self_927;
    let _e978: Dipole = other_751;
    let _e989: MultiVector = self_927;
    let _e993: Dipole = other_751;
    let _e1004: MultiVector = self_927;
    let _e1008: Dipole = other_751;
    let _e1011: Dipole = other_751;
    let _e1014: Dipole = other_751;
    let _e1026: MultiVector = self_927;
    let _e1030: Dipole = other_751;
    let _e1033: Dipole = other_751;
    let _e1036: Dipole = other_751;
    let _e1048: MultiVector = self_927;
    let _e1052: Dipole = other_751;
    let _e1055: Dipole = other_751;
    let _e1058: Dipole = other_751;
    let _e1070: MultiVector = self_927;
    let _e1072: Dipole = other_751;
    let _e1082: MultiVector = self_927;
    let _e1086: Dipole = other_751;
    let _e1089: Dipole = other_751;
    let _e1092: Dipole = other_751;
    let _e1103: MultiVector = self_927;
    let _e1107: Dipole = other_751;
    let _e1110: Dipole = other_751;
    let _e1113: Dipole = other_751;
    let _e1125: MultiVector = self_927;
    let _e1129: Dipole = other_751;
    let _e1133: MultiVector = self_927;
    let _e1137: Dipole = other_751;
    let _e1148: MultiVector = self_927;
    let _e1152: Dipole = other_751;
    let _e1163: MultiVector = self_927;
    let _e1167: Dipole = other_751;
    let _e1178: MultiVector = self_927;
    let _e1182: Dipole = other_751;
    let _e1185: Dipole = other_751;
    let _e1188: Dipole = other_751;
    let _e1194: MultiVector = self_927;
    let _e1198: Dipole = other_751;
    let _e1201: Dipole = other_751;
    let _e1204: Dipole = other_751;
    let _e1216: MultiVector = self_927;
    let _e1220: Dipole = other_751;
    let _e1223: Dipole = other_751;
    let _e1226: Dipole = other_751;
    let _e1229: Dipole = other_751;
    let _e1242: MultiVector = self_927;
    let _e1246: Dipole = other_751;
    let _e1249: Dipole = other_751;
    let _e1252: Dipole = other_751;
    let _e1255: Dipole = other_751;
    let _e1269: MultiVector = self_927;
    let _e1273: Dipole = other_751;
    let _e1276: Dipole = other_751;
    let _e1279: Dipole = other_751;
    let _e1282: Dipole = other_751;
    let _e1294: MultiVector = self_927;
    let _e1298: Dipole = other_751;
    let _e1301: Dipole = other_751;
    let _e1304: Dipole = other_751;
    let _e1307: Dipole = other_751;
    let _e1320: MultiVector = self_927;
    let _e1324: Dipole = other_751;
    let _e1327: Dipole = other_751;
    let _e1330: Dipole = other_751;
    let _e1333: Dipole = other_751;
    let _e1346: MultiVector = self_927;
    let _e1350: Dipole = other_751;
    let _e1353: Dipole = other_751;
    let _e1356: Dipole = other_751;
    let _e1359: Dipole = other_751;
    let _e1372: MultiVector = self_927;
    let _e1376: Dipole = other_751;
    let _e1379: Dipole = other_751;
    let _e1382: Dipole = other_751;
    let _e1385: Dipole = other_751;
    let _e1397: MultiVector = self_927;
    let _e1401: Dipole = other_751;
    let _e1404: Dipole = other_751;
    let _e1407: Dipole = other_751;
    let _e1410: Dipole = other_751;
    let _e1424: MultiVector = self_927;
    let _e1428: Dipole = other_751;
    let _e1431: Dipole = other_751;
    let _e1434: Dipole = other_751;
    let _e1437: Dipole = other_751;
    let _e1450: MultiVector = self_927;
    let _e1454: Dipole = other_751;
    let _e1457: Dipole = other_751;
    let _e1460: Dipole = other_751;
    let _e1463: Dipole = other_751;
    let _e1477: MultiVector = self_927;
    let _e1481: Dipole = other_751;
    let _e1484: Dipole = other_751;
    let _e1487: Dipole = other_751;
    let _e1490: Dipole = other_751;
    let _e1504: MultiVector = self_927;
    let _e1508: Dipole = other_751;
    let _e1511: Dipole = other_751;
    let _e1514: Dipole = other_751;
    let _e1517: Dipole = other_751;
    let _e1529: MultiVector = self_927;
    let _e1533: Dipole = other_751;
    let _e1545: MultiVector = self_927;
    let _e1549: Dipole = other_751;
    let _e1561: MultiVector = self_927;
    let _e1565: Dipole = other_751;
    let _e1577: MultiVector = self_927;
    let _e1581: Dipole = other_751;
    let _e1593: MultiVector = self_927;
    let _e1597: Dipole = other_751;
    let _e1609: MultiVector = self_927;
    let _e1613: Dipole = other_751;
    let _e1625: MultiVector = self_927;
    let _e1629: Dipole = other_751;
    let _e1632: Dipole = other_751;
    let _e1635: Dipole = other_751;
    let _e1638: Dipole = other_751;
    let _e1651: MultiVector = self_927;
    let _e1655: Dipole = other_751;
    let _e1658: Dipole = other_751;
    let _e1661: Dipole = other_751;
    let _e1664: Dipole = other_751;
    let _e1677: MultiVector = self_927;
    let _e1681: Dipole = other_751;
    let _e1684: Dipole = other_751;
    let _e1687: Dipole = other_751;
    let _e1690: Dipole = other_751;
    let _e1703: MultiVector = self_927;
    let _e1707: Dipole = other_751;
    let _e1710: Dipole = other_751;
    let _e1713: Dipole = other_751;
    let _e1716: Dipole = other_751;
    let _e1728: MultiVector = self_927;
    let _e1731: MultiVector = self_927;
    let _e1734: MultiVector = self_927;
    let _e1737: MultiVector = self_927;
    let _e1741: Dipole = other_751;
    return MultiVector(((((((((((((((((vec3<f32>(_e4.g4_.y) * vec3<f32>(_e8.g1_.y)) * vec3<f32>(0.0, -(1.0), 0.0)) + ((vec3<f32>(_e19.g4_.z) * vec3<f32>(_e23.g1_.z)) * vec3<f32>(0.0, -(1.0), 0.0))) + ((vec3<f32>(_e35.g5_.x) * vec3<f32>(_e39.g1_.x, _e42.g0_.x, _e45.g1_.x)) * vec3<f32>(-(1.0), -(1.0), 0.0))) + ((vec3<f32>(_e58.g5_.y) * vec3<f32>(_e62.g1_.y, _e65.g0_.y, _e68.g1_.y)) * vec3<f32>(-(1.0), -(1.0), 0.0))) + ((vec3<f32>(_e81.g5_.z) * vec3<f32>(_e85.g1_.z, _e88.g0_.z, _e91.g1_.z)) * vec3<f32>(-(1.0), -(1.0), 0.0))) + ((vec3<f32>(_e104.g6_.y) * vec3<f32>(_e108.g1_.y)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e120.g6_.z) * vec3<f32>(_e124.g1_.z)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e136.g7_.x) * vec3<f32>(_e140.g0_.x)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e152.g7_.y) * vec3<f32>(_e156.g0_.y)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e168.g7_.z) * vec3<f32>(_e172.g0_.z)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e184.g8_.x) * vec3<f32>(_e188.g2_.x)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e200.g8_.y) * vec3<f32>(_e204.g2_.y)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e216.g8_.z) * vec3<f32>(_e220.g2_.z)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e232.g8_.w) * vec3<f32>(_e236.g2_.w)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e248.g4_.x, _e251.g4_.x, _e254.g6_.x) * vec3<f32>(_e258.g1_.x)) * vec3<f32>(0.0, -(1.0), -(1.0)))), (((((vec3<f32>(_e271.g1_.y) * _e275.g1_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e285.g1_.z) * _e289.g1_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + (vec3<f32>(_e300.g8_.w) * _e304.g1_)) + ((vec3<f32>(_e308.g1_.x) * _e312.g1_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))), ((((((((((vec2<f32>(_e323.g1_.x) * vec2<f32>(_e327.g0_.x, _e330.g2_.x)) * vec2<f32>(-(1.0), 1.0)) + ((vec2<f32>(_e340.g1_.y) * vec2<f32>(_e344.g0_.y, _e347.g2_.y)) * vec2<f32>(-(1.0), 1.0))) + ((vec2<f32>(_e358.g1_.z) * vec2<f32>(_e362.g0_.z, _e365.g2_.z)) * vec2<f32>(-(1.0), 1.0))) + ((vec2<f32>(_e376.g7_.y) * vec2<f32>(_e380.g1_.y)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e391.g7_.z) * vec2<f32>(_e395.g1_.z)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e406.g8_.x) * vec2<f32>(_e410.g1_.x)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e421.g8_.y) * vec2<f32>(_e425.g1_.y)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e436.g8_.z) * vec2<f32>(_e440.g1_.z)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e451.g7_.x) * vec2<f32>(_e455.g1_.x)) * vec2<f32>(0.0, -(1.0)))), ((((((((((((((vec4<f32>(_e466.g0_.x) * _e470.g2_) + ((vec4<f32>(_e473.g3_.y) * vec4<f32>(_e477.g1_.z, _e480.g1_.z, _e483.g1_.x, _e486.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e500.g3_.z) * vec4<f32>(_e504.g1_.y, _e507.g1_.x, _e510.g1_.y, _e513.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e527.g4_.x) * vec4<f32>(_e531.g2_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e543.g4_.y) * vec4<f32>(_e547.g2_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e559.g4_.z) * vec4<f32>(_e563.g2_.z)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e575.g5_.x) * _e579.g2_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e591.g5_.y) * _e595.g2_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e607.g5_.z) * _e611.g2_.yxyy) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e623.g9_.x) * vec4<f32>(_e627.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e640.g9_.y) * vec4<f32>(_e644.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e657.g9_.z) * vec4<f32>(_e661.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e674.g9_.w) * vec4<f32>(_e678.g1_.x, _e681.g1_.y, _e684.g1_.z, _e687.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e699.g3_.x) * vec4<f32>(_e703.g1_.x, _e706.g1_.z, _e709.g1_.y, _e712.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), ((((((((vec3<f32>(_e726.g0_.x) * _e730.g0_) + (vec3<f32>(_e733.g0_.y) * _e737.g1_)) + ((vec3<f32>(_e741.g4_.y) * _e745.g1_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e756.g4_.z) * _e760.g1_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e771.g5_.x) * _e775.g0_.zzy) * vec3<f32>(0.0, 1.0, -(1.0)))) + ((vec3<f32>(_e786.g5_.y) * _e790.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e801.g5_.z) * _e805.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e816.g4_.x) * _e820.g1_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))), ((((vec3<f32>(_e831.g0_.x) * _e835.g1_) + ((vec3<f32>(_e838.g5_.y) * _e842.g1_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e853.g5_.z) * _e857.g1_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e868.g5_.x) * _e872.g1_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))), (((((((((((((vec3<f32>(_e883.g0_.z) * _e887.g1_) + (vec3<f32>(_e890.g2_.x) * vec3<f32>(_e894.g2_.x, _e897.g2_.y, _e900.g2_.z))) + (vec3<f32>(_e906.g2_.y) * _e910.g0_)) + ((vec3<f32>(_e914.g6_.x) * _e918.g1_.zzy) * vec3<f32>(0.0, 1.0, -(1.0)))) + ((vec3<f32>(_e929.g6_.y) * _e933.g1_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e944.g6_.z) * _e948.g1_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e959.g7_.x) * _e963.g0_.zzy) * vec3<f32>(0.0, 1.0, -(1.0)))) + ((vec3<f32>(_e974.g7_.y) * _e978.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e989.g7_.z) * _e993.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e1004.g8_.x) * vec3<f32>(_e1008.g2_.z, _e1011.g2_.z, _e1014.g2_.y)) * vec3<f32>(0.0, 1.0, -(1.0)))) + ((vec3<f32>(_e1026.g8_.y) * vec3<f32>(_e1030.g2_.z, _e1033.g2_.z, _e1036.g2_.x)) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e1048.g8_.z) * vec3<f32>(_e1052.g2_.y, _e1055.g2_.x, _e1058.g2_.y)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((_e1070.g1_ * vec3<f32>(_e1072.g2_.w)) * vec3<f32>(-(1.0)))), (((((((((vec3<f32>(_e1082.g1_.y) * vec3<f32>(_e1086.g2_.z, _e1089.g2_.z, _e1092.g2_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e1103.g1_.z) * vec3<f32>(_e1107.g2_.y, _e1110.g2_.x, _e1113.g2_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + (vec3<f32>(_e1125.g2_.y) * _e1129.g1_)) + ((vec3<f32>(_e1133.g7_.x) * _e1137.g1_.zzy) * vec3<f32>(0.0, 1.0, -(1.0)))) + ((vec3<f32>(_e1148.g7_.y) * _e1152.g1_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e1163.g7_.z) * _e1167.g1_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) - (vec3<f32>(_e1178.g8_.w) * vec3<f32>(_e1182.g2_.x, _e1185.g2_.y, _e1188.g2_.z))) + ((vec3<f32>(_e1194.g1_.x) * vec3<f32>(_e1198.g2_.x, _e1201.g2_.z, _e1204.g2_.y)) * vec3<f32>(0.0, -(1.0), 1.0))), (((((((((vec4<f32>(_e1216.g1_.y) * vec4<f32>(_e1220.g0_.z, _e1223.g0_.z, _e1226.g0_.x, _e1229.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e1242.g1_.z) * vec4<f32>(_e1246.g0_.y, _e1249.g0_.x, _e1252.g0_.y, _e1255.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e1269.g2_.x) * vec4<f32>(_e1273.g1_.x, _e1276.g1_.y, _e1279.g1_.z, _e1282.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e1294.g8_.x) * vec4<f32>(_e1298.g1_.z, _e1301.g1_.z, _e1304.g1_.y, _e1307.g1_.z)) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e1320.g8_.y) * vec4<f32>(_e1324.g1_.z, _e1327.g1_.z, _e1330.g1_.x, _e1333.g1_.z)) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e1346.g8_.z) * vec4<f32>(_e1350.g1_.y, _e1353.g1_.x, _e1356.g1_.y, _e1359.g1_.y)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e1372.g8_.w) * vec4<f32>(_e1376.g0_.x, _e1379.g0_.y, _e1382.g0_.z, _e1385.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e1397.g1_.x) * vec4<f32>(_e1401.g0_.x, _e1404.g0_.z, _e1407.g0_.y, _e1410.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), ((((((((((((((((vec4<f32>(_e1424.g3_.x) * vec4<f32>(_e1428.g0_.z, _e1431.g0_.z, _e1434.g0_.y, _e1437.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e1450.g3_.y) * vec4<f32>(_e1454.g0_.z, _e1457.g0_.z, _e1460.g0_.x, _e1463.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e1477.g3_.z) * vec4<f32>(_e1481.g0_.y, _e1484.g0_.x, _e1487.g0_.y, _e1490.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e1504.g3_.w) * vec4<f32>(_e1508.g1_.x, _e1511.g1_.y, _e1514.g1_.z, _e1517.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e1529.g4_.x) * _e1533.g2_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e1545.g4_.y) * _e1549.g2_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e1561.g4_.z) * _e1565.g2_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e1577.g5_.x) * _e1581.g2_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e1593.g5_.y) * _e1597.g2_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e1609.g5_.z) * _e1613.g2_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e1625.g9_.x) * vec4<f32>(_e1629.g1_.z, _e1632.g1_.z, _e1635.g1_.y, _e1638.g1_.z)) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e1651.g9_.y) * vec4<f32>(_e1655.g1_.z, _e1658.g1_.z, _e1661.g1_.x, _e1664.g1_.z)) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e1677.g9_.z) * vec4<f32>(_e1681.g1_.y, _e1684.g1_.x, _e1687.g1_.y, _e1690.g1_.y)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e1703.g9_.w) * vec4<f32>(_e1707.g0_.x, _e1710.g0_.y, _e1713.g0_.z, _e1716.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e1728.g0_.y, _e1731.g0_.y, _e1734.g0_.y, _e1737.g0_.x) * _e1741.g2_.xyzx) * vec4<f32>(-(1.0), -(1.0), -(1.0), 0.0))));
}

fn multi_vector_dipole_scalar_product(self_928: MultiVector, other_752: Dipole) -> Scalar {
    var self_929: MultiVector;
    var other_753: Dipole;

    self_929 = self_928;
    other_753 = other_752;
    let _e5: MultiVector = self_929;
    let _e8: Dipole = other_753;
    let _e13: MultiVector = self_929;
    let _e16: Dipole = other_753;
    let _e21: MultiVector = self_929;
    let _e24: Dipole = other_753;
    return Scalar((((0.0 - (_e5.g5_.x * _e8.g1_.x)) - (_e13.g5_.y * _e16.g1_.y)) - (_e21.g5_.z * _e24.g1_.z)));
}

fn multi_vector_line_into(self_930: MultiVector) -> Line {
    var self_931: MultiVector;

    self_931 = self_930;
    let _e2: MultiVector = self_931;
    let _e4: MultiVector = self_931;
    return Line(_e2.g6_, _e4.g7_);
}

fn multi_vector_line_add(self_932: MultiVector, other_754: Line) -> MultiVector {
    var self_933: MultiVector;
    var other_755: Line;

    self_933 = self_932;
    other_755 = other_754;
    let _e4: MultiVector = self_933;
    let _e6: MultiVector = self_933;
    let _e8: MultiVector = self_933;
    let _e10: MultiVector = self_933;
    let _e12: MultiVector = self_933;
    let _e14: MultiVector = self_933;
    let _e16: MultiVector = self_933;
    let _e18: Line = other_755;
    let _e21: MultiVector = self_933;
    let _e23: Line = other_755;
    let _e26: MultiVector = self_933;
    let _e28: MultiVector = self_933;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, _e10.g3_, _e12.g4_, _e14.g5_, (_e16.g6_ + _e18.g0_), (_e21.g7_ + _e23.g1_), _e26.g8_, _e28.g9_);
}

fn multi_vector_line_sub(self_934: MultiVector, other_756: Line) -> MultiVector {
    var self_935: MultiVector;
    var other_757: Line;

    self_935 = self_934;
    other_757 = other_756;
    let _e4: MultiVector = self_935;
    let _e6: MultiVector = self_935;
    let _e8: MultiVector = self_935;
    let _e10: MultiVector = self_935;
    let _e12: MultiVector = self_935;
    let _e14: MultiVector = self_935;
    let _e16: MultiVector = self_935;
    let _e18: Line = other_757;
    let _e21: MultiVector = self_935;
    let _e23: Line = other_757;
    let _e26: MultiVector = self_935;
    let _e28: MultiVector = self_935;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, _e10.g3_, _e12.g4_, _e14.g5_, (_e16.g6_ - _e18.g0_), (_e21.g7_ - _e23.g1_), _e26.g8_, _e28.g9_);
}

fn multi_vector_circle_into(self_936: MultiVector) -> Circle {
    var self_937: MultiVector;

    self_937 = self_936;
    let _e2: MultiVector = self_937;
    let _e4: MultiVector = self_937;
    let _e6: MultiVector = self_937;
    return Circle(_e2.g8_, _e4.g6_, _e6.g7_);
}

fn multi_vector_circle_add(self_938: MultiVector, other_758: Circle) -> MultiVector {
    var self_939: MultiVector;
    var other_759: Circle;

    self_939 = self_938;
    other_759 = other_758;
    let _e4: MultiVector = self_939;
    let _e6: MultiVector = self_939;
    let _e8: MultiVector = self_939;
    let _e10: MultiVector = self_939;
    let _e12: MultiVector = self_939;
    let _e14: MultiVector = self_939;
    let _e16: MultiVector = self_939;
    let _e18: Circle = other_759;
    let _e21: MultiVector = self_939;
    let _e23: Circle = other_759;
    let _e26: MultiVector = self_939;
    let _e28: Circle = other_759;
    let _e31: MultiVector = self_939;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, _e10.g3_, _e12.g4_, _e14.g5_, (_e16.g6_ + _e18.g1_), (_e21.g7_ + _e23.g2_), (_e26.g8_ + _e28.g0_), _e31.g9_);
}

fn multi_vector_circle_sub(self_940: MultiVector, other_760: Circle) -> MultiVector {
    var self_941: MultiVector;
    var other_761: Circle;

    self_941 = self_940;
    other_761 = other_760;
    let _e4: MultiVector = self_941;
    let _e6: MultiVector = self_941;
    let _e8: MultiVector = self_941;
    let _e10: MultiVector = self_941;
    let _e12: MultiVector = self_941;
    let _e14: MultiVector = self_941;
    let _e16: MultiVector = self_941;
    let _e18: Circle = other_761;
    let _e21: MultiVector = self_941;
    let _e23: Circle = other_761;
    let _e26: MultiVector = self_941;
    let _e28: Circle = other_761;
    let _e31: MultiVector = self_941;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, _e10.g3_, _e12.g4_, _e14.g5_, (_e16.g6_ - _e18.g1_), (_e21.g7_ - _e23.g2_), (_e26.g8_ - _e28.g0_), _e31.g9_);
}

fn multi_vector_circle_geometric_product(self_942: MultiVector, other_762: Circle) -> MultiVector {
    var self_943: MultiVector;
    var other_763: Circle;

    self_943 = self_942;
    other_763 = other_762;
    let _e4: MultiVector = self_943;
    let _e8: Circle = other_763;
    let _e18: MultiVector = self_943;
    let _e22: Circle = other_763;
    let _e33: MultiVector = self_943;
    let _e37: Circle = other_763;
    let _e48: MultiVector = self_943;
    let _e52: Circle = other_763;
    let _e64: MultiVector = self_943;
    let _e68: Circle = other_763;
    let _e80: MultiVector = self_943;
    let _e84: Circle = other_763;
    let _e96: MultiVector = self_943;
    let _e100: Circle = other_763;
    let _e112: MultiVector = self_943;
    let _e116: Circle = other_763;
    let _e128: MultiVector = self_943;
    let _e132: Circle = other_763;
    let _e144: MultiVector = self_943;
    let _e148: Circle = other_763;
    let _e160: MultiVector = self_943;
    let _e164: Circle = other_763;
    let _e176: MultiVector = self_943;
    let _e180: Circle = other_763;
    let _e192: MultiVector = self_943;
    let _e196: Circle = other_763;
    let _e208: MultiVector = self_943;
    let _e212: Circle = other_763;
    let _e224: MultiVector = self_943;
    let _e228: Circle = other_763;
    let _e239: MultiVector = self_943;
    let _e241: Circle = other_763;
    let _e248: MultiVector = self_943;
    let _e252: Circle = other_763;
    let _e255: Circle = other_763;
    let _e261: MultiVector = self_943;
    let _e265: Circle = other_763;
    let _e268: Circle = other_763;
    let _e274: MultiVector = self_943;
    let _e278: Circle = other_763;
    let _e281: Circle = other_763;
    let _e287: MultiVector = self_943;
    let _e291: Circle = other_763;
    let _e301: MultiVector = self_943;
    let _e304: MultiVector = self_943;
    let _e308: Circle = other_763;
    let _e311: Circle = other_763;
    let _e322: MultiVector = self_943;
    let _e326: Circle = other_763;
    let _e329: Circle = other_763;
    let _e332: Circle = other_763;
    let _e335: Circle = other_763;
    let _e348: MultiVector = self_943;
    let _e352: Circle = other_763;
    let _e355: Circle = other_763;
    let _e358: Circle = other_763;
    let _e361: Circle = other_763;
    let _e375: MultiVector = self_943;
    let _e379: Circle = other_763;
    let _e382: Circle = other_763;
    let _e385: Circle = other_763;
    let _e388: Circle = other_763;
    let _e402: MultiVector = self_943;
    let _e406: Circle = other_763;
    let _e418: MultiVector = self_943;
    let _e422: Circle = other_763;
    let _e434: MultiVector = self_943;
    let _e438: Circle = other_763;
    let _e450: MultiVector = self_943;
    let _e454: Circle = other_763;
    let _e467: MultiVector = self_943;
    let _e471: Circle = other_763;
    let _e484: MultiVector = self_943;
    let _e488: Circle = other_763;
    let _e501: MultiVector = self_943;
    let _e505: Circle = other_763;
    let _e508: Circle = other_763;
    let _e511: Circle = other_763;
    let _e514: Circle = other_763;
    let _e526: MultiVector = self_943;
    let _e529: MultiVector = self_943;
    let _e532: MultiVector = self_943;
    let _e535: MultiVector = self_943;
    let _e539: Circle = other_763;
    let _e550: MultiVector = self_943;
    let _e554: Circle = other_763;
    let _e557: Circle = other_763;
    let _e560: Circle = other_763;
    let _e571: MultiVector = self_943;
    let _e575: Circle = other_763;
    let _e578: Circle = other_763;
    let _e581: Circle = other_763;
    let _e593: MultiVector = self_943;
    let _e597: Circle = other_763;
    let _e608: MultiVector = self_943;
    let _e612: Circle = other_763;
    let _e623: MultiVector = self_943;
    let _e627: Circle = other_763;
    let _e638: MultiVector = self_943;
    let _e642: Circle = other_763;
    let _e645: Circle = other_763;
    let _e648: Circle = other_763;
    let _e654: MultiVector = self_943;
    let _e658: Circle = other_763;
    let _e661: Circle = other_763;
    let _e664: Circle = other_763;
    let _e676: MultiVector = self_943;
    let _e678: Circle = other_763;
    let _e687: MultiVector = self_943;
    let _e691: Circle = other_763;
    let _e694: MultiVector = self_943;
    let _e698: Circle = other_763;
    let _e702: MultiVector = self_943;
    let _e706: Circle = other_763;
    let _e709: Circle = other_763;
    let _e712: Circle = other_763;
    let _e724: MultiVector = self_943;
    let _e728: Circle = other_763;
    let _e731: Circle = other_763;
    let _e734: Circle = other_763;
    let _e746: MultiVector = self_943;
    let _e750: Circle = other_763;
    let _e761: MultiVector = self_943;
    let _e765: Circle = other_763;
    let _e776: MultiVector = self_943;
    let _e780: Circle = other_763;
    let _e791: MultiVector = self_943;
    let _e795: Circle = other_763;
    let _e806: MultiVector = self_943;
    let _e810: Circle = other_763;
    let _e821: MultiVector = self_943;
    let _e825: Circle = other_763;
    let _e836: MultiVector = self_943;
    let _e840: Circle = other_763;
    let _e852: MultiVector = self_943;
    let _e856: Circle = other_763;
    let _e868: MultiVector = self_943;
    let _e872: Circle = other_763;
    let _e875: Circle = other_763;
    let _e878: Circle = other_763;
    let _e884: MultiVector = self_943;
    let _e887: MultiVector = self_943;
    let _e890: MultiVector = self_943;
    let _e894: Circle = other_763;
    let _e897: Circle = other_763;
    let _e900: Circle = other_763;
    let _e913: MultiVector = self_943;
    let _e917: Circle = other_763;
    let _e920: MultiVector = self_943;
    let _e924: Circle = other_763;
    let _e935: MultiVector = self_943;
    let _e939: Circle = other_763;
    let _e950: MultiVector = self_943;
    let _e954: Circle = other_763;
    let _e965: MultiVector = self_943;
    let _e968: MultiVector = self_943;
    let _e971: MultiVector = self_943;
    let _e975: Circle = other_763;
    let _e981: MultiVector = self_943;
    let _e985: Circle = other_763;
    let _e988: MultiVector = self_943;
    let _e992: Circle = other_763;
    let _e1004: MultiVector = self_943;
    let _e1008: Circle = other_763;
    let _e1020: MultiVector = self_943;
    let _e1024: Circle = other_763;
    let _e1036: MultiVector = self_943;
    let _e1039: MultiVector = self_943;
    let _e1042: MultiVector = self_943;
    let _e1045: MultiVector = self_943;
    let _e1049: Circle = other_763;
    let _e1063: MultiVector = self_943;
    let _e1067: Circle = other_763;
    let _e1070: Circle = other_763;
    let _e1073: Circle = other_763;
    let _e1076: Circle = other_763;
    let _e1089: MultiVector = self_943;
    let _e1093: Circle = other_763;
    let _e1096: Circle = other_763;
    let _e1099: Circle = other_763;
    let _e1102: Circle = other_763;
    let _e1116: MultiVector = self_943;
    let _e1120: Circle = other_763;
    let _e1123: Circle = other_763;
    let _e1126: Circle = other_763;
    let _e1129: Circle = other_763;
    let _e1141: MultiVector = self_943;
    let _e1145: Circle = other_763;
    let _e1149: MultiVector = self_943;
    let _e1153: Circle = other_763;
    let _e1165: MultiVector = self_943;
    let _e1169: Circle = other_763;
    let _e1181: MultiVector = self_943;
    let _e1185: Circle = other_763;
    let _e1197: MultiVector = self_943;
    let _e1201: Circle = other_763;
    let _e1213: MultiVector = self_943;
    let _e1217: Circle = other_763;
    let _e1229: MultiVector = self_943;
    let _e1233: Circle = other_763;
    let _e1245: MultiVector = self_943;
    let _e1249: Circle = other_763;
    let _e1252: Circle = other_763;
    let _e1255: Circle = other_763;
    let _e1258: Circle = other_763;
    let _e1271: MultiVector = self_943;
    let _e1275: Circle = other_763;
    let _e1278: Circle = other_763;
    let _e1281: Circle = other_763;
    let _e1284: Circle = other_763;
    let _e1297: MultiVector = self_943;
    let _e1301: Circle = other_763;
    let _e1304: Circle = other_763;
    let _e1307: Circle = other_763;
    let _e1310: Circle = other_763;
    let _e1323: MultiVector = self_943;
    let _e1327: Circle = other_763;
    let _e1330: Circle = other_763;
    let _e1333: Circle = other_763;
    let _e1336: Circle = other_763;
    let _e1348: MultiVector = self_943;
    let _e1352: Circle = other_763;
    let _e1355: Circle = other_763;
    let _e1358: Circle = other_763;
    let _e1361: Circle = other_763;
    return MultiVector(((((((((((((((((vec3<f32>(_e4.g1_.y) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(0.0, 1.0, 0.0)) + ((vec3<f32>(_e18.g1_.z) * vec3<f32>(_e22.g0_.z)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e33.g2_.x) * vec3<f32>(_e37.g0_.w)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e48.g3_.x) * vec3<f32>(_e52.g0_.x)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e64.g3_.y) * vec3<f32>(_e68.g0_.y)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e80.g3_.z) * vec3<f32>(_e84.g0_.z)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e96.g3_.w) * vec3<f32>(_e100.g0_.w)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e112.g4_.x) * vec3<f32>(_e116.g2_.x)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e128.g4_.y) * vec3<f32>(_e132.g2_.y)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e144.g4_.z) * vec3<f32>(_e148.g2_.z)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e160.g5_.x) * vec3<f32>(_e164.g1_.x)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e176.g5_.y) * vec3<f32>(_e180.g1_.y)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e192.g5_.z) * vec3<f32>(_e196.g1_.z)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e208.g8_.w) * vec3<f32>(_e212.g0_.w)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e224.g1_.x) * vec3<f32>(_e228.g0_.x)) * vec3<f32>(0.0, 1.0, 0.0))), (_e239.g5_ * vec3<f32>(_e241.g0_.w)), (((((vec2<f32>(0.0) - (vec2<f32>(_e248.g5_.x) * vec2<f32>(_e252.g0_.x, _e255.g2_.x))) - (vec2<f32>(_e261.g5_.y) * vec2<f32>(_e265.g0_.y, _e268.g2_.y))) - (vec2<f32>(_e274.g5_.z) * vec2<f32>(_e278.g0_.z, _e281.g2_.z))) + ((vec2<f32>(_e287.g9_.w) * vec2<f32>(_e291.g0_.w)) * vec2<f32>(0.0, 1.0))) + ((vec2<f32>(_e301.g0_.y, _e304.g0_.x) * vec2<f32>(_e308.g0_.w, _e311.g0_.x)) * vec2<f32>(-(1.0), 0.0))), ((((((((((((vec4<f32>(_e322.g1_.x) * vec4<f32>(_e326.g2_.z, _e329.g2_.z, _e332.g2_.y, _e335.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e348.g1_.y) * vec4<f32>(_e352.g2_.z, _e355.g2_.z, _e358.g2_.x, _e361.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e375.g1_.z) * vec4<f32>(_e379.g2_.y, _e382.g2_.x, _e385.g2_.y, _e388.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e402.g7_.x) * _e406.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e418.g7_.y) * _e422.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e434.g7_.z) * _e438.g0_.wwwz) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e450.g8_.x) * vec4<f32>(_e454.g2_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e467.g8_.y) * vec4<f32>(_e471.g2_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e484.g8_.z) * vec4<f32>(_e488.g2_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e501.g8_.w) * vec4<f32>(_e505.g2_.x, _e508.g2_.y, _e511.g2_.z, _e514.g2_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e526.g0_.x, _e529.g0_.x, _e532.g0_.x, _e535.g0_.z) * _e539.g0_.xxxw) * vec4<f32>(0.0, 0.0, 0.0, 1.0))), ((((((((vec3<f32>(_e550.g1_.y) * vec3<f32>(_e554.g0_.z, _e557.g0_.z, _e560.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e571.g1_.z) * vec3<f32>(_e575.g0_.y, _e578.g0_.x, _e581.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e593.g8_.x) * vec3<f32>(_e597.g0_.w)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e608.g8_.y) * vec3<f32>(_e612.g0_.w)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e623.g8_.z) * vec3<f32>(_e627.g0_.w)) * vec3<f32>(0.0, 0.0, 1.0))) - (vec3<f32>(_e638.g8_.w) * vec3<f32>(_e642.g0_.x, _e645.g0_.y, _e648.g0_.z))) + ((vec3<f32>(_e654.g1_.x) * vec3<f32>(_e658.g0_.x, _e661.g0_.z, _e664.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))), ((_e676.g1_ * vec3<f32>(_e678.g0_.w)) * vec3<f32>(-(1.0))), ((((((((((((((vec3<f32>(_e687.g0_.x) * _e691.g1_) + (vec3<f32>(_e694.g0_.y) * _e698.g2_)) + ((vec3<f32>(_e702.g3_.y) * vec3<f32>(_e706.g0_.z, _e709.g0_.z, _e712.g0_.x)) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e724.g3_.z) * vec3<f32>(_e728.g0_.y, _e731.g0_.x, _e734.g0_.y)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e746.g4_.x) * _e750.g2_.zzy) * vec3<f32>(0.0, 1.0, -(1.0)))) + ((vec3<f32>(_e761.g4_.y) * _e765.g2_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e776.g4_.z) * _e780.g2_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e791.g5_.x) * _e795.g1_.zzy) * vec3<f32>(0.0, 1.0, -(1.0)))) + ((vec3<f32>(_e806.g5_.y) * _e810.g1_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e821.g5_.z) * _e825.g1_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e836.g9_.y) * vec3<f32>(_e840.g0_.w)) * vec3<f32>(0.0, -(1.0), 0.0))) + ((vec3<f32>(_e852.g9_.z) * vec3<f32>(_e856.g0_.w)) * vec3<f32>(0.0, 0.0, -(1.0)))) + (vec3<f32>(_e868.g9_.w) * vec3<f32>(_e872.g0_.x, _e875.g0_.y, _e878.g0_.z))) + ((vec3<f32>(_e884.g9_.x, _e887.g3_.x, _e890.g3_.x) * vec3<f32>(_e894.g0_.w, _e897.g0_.z, _e900.g0_.y)) * vec3<f32>(-(1.0), 1.0, -(1.0)))), (((((vec3<f32>(_e913.g0_.x) * _e917.g2_) + ((vec3<f32>(_e920.g5_.x) * _e924.g2_.zzy) * vec3<f32>(0.0, 1.0, -(1.0)))) + ((vec3<f32>(_e935.g5_.y) * _e939.g2_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e950.g5_.z) * _e954.g2_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + (vec3<f32>(_e965.g3_.x, _e968.g3_.y, _e971.g3_.z) * vec3<f32>(_e975.g0_.w))), (((((vec4<f32>(_e981.g0_.x) * _e985.g0_) + ((vec4<f32>(_e988.g5_.x) * _e992.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e1004.g5_.y) * _e1008.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e1020.g5_.z) * _e1024.g0_.yxyy) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e1036.g4_.x, _e1039.g4_.y, _e1042.g4_.z, _e1045.g4_.x) * _e1049.g0_.wwwx) * vec4<f32>(-(1.0), -(1.0), -(1.0), 0.0))), ((((((((((((((((vec4<f32>(_e1063.g1_.y) * vec4<f32>(_e1067.g1_.z, _e1070.g1_.z, _e1073.g1_.x, _e1076.g2_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e1089.g1_.z) * vec4<f32>(_e1093.g1_.y, _e1096.g1_.x, _e1099.g1_.y, _e1102.g2_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e1116.g2_.x) * vec4<f32>(_e1120.g2_.x, _e1123.g2_.y, _e1126.g2_.z, _e1129.g2_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) - (vec4<f32>(_e1141.g2_.y) * _e1145.g0_)) + ((vec4<f32>(_e1149.g6_.x) * vec4<f32>(_e1153.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e1165.g6_.y) * vec4<f32>(_e1169.g0_.w)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e1181.g6_.z) * vec4<f32>(_e1185.g0_.w)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e1197.g7_.x) * _e1201.g0_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e1213.g7_.y) * _e1217.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e1229.g7_.z) * _e1233.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e1245.g8_.x) * vec4<f32>(_e1249.g2_.z, _e1252.g2_.z, _e1255.g2_.y, _e1258.g2_.z)) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e1271.g8_.y) * vec4<f32>(_e1275.g2_.z, _e1278.g2_.z, _e1281.g2_.x, _e1284.g2_.z)) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e1297.g8_.z) * vec4<f32>(_e1301.g2_.y, _e1304.g2_.x, _e1307.g2_.y, _e1310.g2_.y)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e1323.g8_.w) * vec4<f32>(_e1327.g1_.x, _e1330.g1_.y, _e1333.g1_.z, _e1336.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e1348.g1_.x) * vec4<f32>(_e1352.g1_.x, _e1355.g1_.z, _e1358.g1_.y, _e1361.g2_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn multi_vector_circle_scalar_product(self_944: MultiVector, other_764: Circle) -> Scalar {
    var self_945: MultiVector;
    var other_765: Circle;

    self_945 = self_944;
    other_765 = other_764;
    let _e5: MultiVector = self_945;
    let _e8: Circle = other_765;
    return Scalar((0.0 - (_e5.g8_.w * _e8.g0_.w)));
}

fn multi_vector_plane_into(self_946: MultiVector) -> Plane {
    var self_947: MultiVector;

    self_947 = self_946;
    let _e2: MultiVector = self_947;
    return Plane(_e2.g9_);
}

fn multi_vector_plane_add(self_948: MultiVector, other_766: Plane) -> MultiVector {
    var self_949: MultiVector;
    var other_767: Plane;

    self_949 = self_948;
    other_767 = other_766;
    let _e4: MultiVector = self_949;
    let _e6: MultiVector = self_949;
    let _e8: MultiVector = self_949;
    let _e10: MultiVector = self_949;
    let _e12: MultiVector = self_949;
    let _e14: MultiVector = self_949;
    let _e16: MultiVector = self_949;
    let _e18: MultiVector = self_949;
    let _e20: MultiVector = self_949;
    let _e22: MultiVector = self_949;
    let _e24: Plane = other_767;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, _e10.g3_, _e12.g4_, _e14.g5_, _e16.g6_, _e18.g7_, _e20.g8_, (_e22.g9_ + _e24.g0_));
}

fn multi_vector_plane_sub(self_950: MultiVector, other_768: Plane) -> MultiVector {
    var self_951: MultiVector;
    var other_769: Plane;

    self_951 = self_950;
    other_769 = other_768;
    let _e4: MultiVector = self_951;
    let _e6: MultiVector = self_951;
    let _e8: MultiVector = self_951;
    let _e10: MultiVector = self_951;
    let _e12: MultiVector = self_951;
    let _e14: MultiVector = self_951;
    let _e16: MultiVector = self_951;
    let _e18: MultiVector = self_951;
    let _e20: MultiVector = self_951;
    let _e22: MultiVector = self_951;
    let _e24: Plane = other_769;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, _e10.g3_, _e12.g4_, _e14.g5_, _e16.g6_, _e18.g7_, _e20.g8_, (_e22.g9_ - _e24.g0_));
}

fn multi_vector_sphere_into(self_952: MultiVector) -> Sphere {
    var self_953: MultiVector;

    self_953 = self_952;
    let _e2: MultiVector = self_953;
    let _e5: MultiVector = self_953;
    return Sphere(_e2.g0_.y, _e5.g9_);
}

fn multi_vector_sphere_add(self_954: MultiVector, other_770: Sphere) -> MultiVector {
    var self_955: MultiVector;
    var other_771: Sphere;

    self_955 = self_954;
    other_771 = other_770;
    let _e4: MultiVector = self_955;
    let _e6: Sphere = other_771;
    let _e15: MultiVector = self_955;
    let _e17: MultiVector = self_955;
    let _e19: MultiVector = self_955;
    let _e21: MultiVector = self_955;
    let _e23: MultiVector = self_955;
    let _e25: MultiVector = self_955;
    let _e27: MultiVector = self_955;
    let _e29: MultiVector = self_955;
    let _e31: MultiVector = self_955;
    let _e33: Sphere = other_771;
    return MultiVector((_e4.g0_ + (vec3<f32>(_e6.g0_) * vec3<f32>(0.0, 1.0, 0.0))), _e15.g1_, _e17.g2_, _e19.g3_, _e21.g4_, _e23.g5_, _e25.g6_, _e27.g7_, _e29.g8_, (_e31.g9_ + _e33.g1_));
}

fn multi_vector_sphere_sub(self_956: MultiVector, other_772: Sphere) -> MultiVector {
    var self_957: MultiVector;
    var other_773: Sphere;

    self_957 = self_956;
    other_773 = other_772;
    let _e4: MultiVector = self_957;
    let _e6: Sphere = other_773;
    let _e15: MultiVector = self_957;
    let _e17: MultiVector = self_957;
    let _e19: MultiVector = self_957;
    let _e21: MultiVector = self_957;
    let _e23: MultiVector = self_957;
    let _e25: MultiVector = self_957;
    let _e27: MultiVector = self_957;
    let _e29: MultiVector = self_957;
    let _e31: MultiVector = self_957;
    let _e33: Sphere = other_773;
    return MultiVector((_e4.g0_ - (vec3<f32>(_e6.g0_) * vec3<f32>(0.0, 1.0, 0.0))), _e15.g1_, _e17.g2_, _e19.g3_, _e21.g4_, _e23.g5_, _e25.g6_, _e27.g7_, _e29.g8_, (_e31.g9_ - _e33.g1_));
}

fn multi_vector_motor_into(self_958: MultiVector) -> Motor {
    var self_959: MultiVector;

    self_959 = self_958;
    let _e2: MultiVector = self_959;
    let _e5: MultiVector = self_959;
    let _e8: MultiVector = self_959;
    let _e11: MultiVector = self_959;
    let _e15: MultiVector = self_959;
    let _e18: MultiVector = self_959;
    let _e21: MultiVector = self_959;
    let _e24: MultiVector = self_959;
    return Motor(vec4<f32>(_e2.g6_.x, _e5.g6_.y, _e8.g6_.z, _e11.g0_.z), vec4<f32>(_e15.g7_.x, _e18.g7_.y, _e21.g7_.z, _e24.g2_.y));
}

fn multi_vector_motor_add(self_960: MultiVector, other_774: Motor) -> MultiVector {
    var self_961: MultiVector;
    var other_775: Motor;

    self_961 = self_960;
    other_775 = other_774;
    let _e4: MultiVector = self_961;
    let _e6: Motor = other_775;
    let _e9: Motor = other_775;
    let _e12: Motor = other_775;
    let _e22: MultiVector = self_961;
    let _e24: MultiVector = self_961;
    let _e26: Motor = other_775;
    let _e29: Motor = other_775;
    let _e38: MultiVector = self_961;
    let _e40: MultiVector = self_961;
    let _e42: MultiVector = self_961;
    let _e44: MultiVector = self_961;
    let _e46: Motor = other_775;
    let _e49: Motor = other_775;
    let _e52: Motor = other_775;
    let _e57: MultiVector = self_961;
    let _e59: Motor = other_775;
    let _e62: Motor = other_775;
    let _e65: Motor = other_775;
    let _e70: MultiVector = self_961;
    let _e72: MultiVector = self_961;
    return MultiVector((_e4.g0_ + (vec3<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.w) * vec3<f32>(0.0, 0.0, 1.0))), _e22.g1_, (_e24.g2_ + (vec2<f32>(_e26.g0_.x, _e29.g1_.w) * vec2<f32>(0.0, 1.0))), _e38.g3_, _e40.g4_, _e42.g5_, (_e44.g6_ + vec3<f32>(_e46.g0_.x, _e49.g0_.y, _e52.g0_.z)), (_e57.g7_ + vec3<f32>(_e59.g1_.x, _e62.g1_.y, _e65.g1_.z)), _e70.g8_, _e72.g9_);
}

fn multi_vector_motor_sub(self_962: MultiVector, other_776: Motor) -> MultiVector {
    var self_963: MultiVector;
    var other_777: Motor;

    self_963 = self_962;
    other_777 = other_776;
    let _e4: MultiVector = self_963;
    let _e6: Motor = other_777;
    let _e9: Motor = other_777;
    let _e12: Motor = other_777;
    let _e22: MultiVector = self_963;
    let _e24: MultiVector = self_963;
    let _e26: Motor = other_777;
    let _e29: Motor = other_777;
    let _e38: MultiVector = self_963;
    let _e40: MultiVector = self_963;
    let _e42: MultiVector = self_963;
    let _e44: MultiVector = self_963;
    let _e46: Motor = other_777;
    let _e49: Motor = other_777;
    let _e52: Motor = other_777;
    let _e57: MultiVector = self_963;
    let _e59: Motor = other_777;
    let _e62: Motor = other_777;
    let _e65: Motor = other_777;
    let _e70: MultiVector = self_963;
    let _e72: MultiVector = self_963;
    return MultiVector((_e4.g0_ - (vec3<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.w) * vec3<f32>(0.0, 0.0, 1.0))), _e22.g1_, (_e24.g2_ - (vec2<f32>(_e26.g0_.x, _e29.g1_.w) * vec2<f32>(0.0, 1.0))), _e38.g3_, _e40.g4_, _e42.g5_, (_e44.g6_ - vec3<f32>(_e46.g0_.x, _e49.g0_.y, _e52.g0_.z)), (_e57.g7_ - vec3<f32>(_e59.g1_.x, _e62.g1_.y, _e65.g1_.z)), _e70.g8_, _e72.g9_);
}

fn multi_vector_rotor_into(self_964: MultiVector) -> Rotor {
    var self_965: MultiVector;

    self_965 = self_964;
    let _e2: MultiVector = self_965;
    let _e5: MultiVector = self_965;
    let _e8: MultiVector = self_965;
    let _e11: MultiVector = self_965;
    return Rotor(vec4<f32>(_e2.g6_.x, _e5.g6_.y, _e8.g6_.z, _e11.g0_.z));
}

fn multi_vector_rotor_add(self_966: MultiVector, other_778: Rotor) -> MultiVector {
    var self_967: MultiVector;
    var other_779: Rotor;

    self_967 = self_966;
    other_779 = other_778;
    let _e4: MultiVector = self_967;
    let _e6: Rotor = other_779;
    let _e9: Rotor = other_779;
    let _e12: Rotor = other_779;
    let _e22: MultiVector = self_967;
    let _e24: MultiVector = self_967;
    let _e26: MultiVector = self_967;
    let _e28: MultiVector = self_967;
    let _e30: MultiVector = self_967;
    let _e32: MultiVector = self_967;
    let _e34: Rotor = other_779;
    let _e37: Rotor = other_779;
    let _e40: Rotor = other_779;
    let _e45: MultiVector = self_967;
    let _e47: MultiVector = self_967;
    let _e49: MultiVector = self_967;
    return MultiVector((_e4.g0_ + (vec3<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.w) * vec3<f32>(0.0, 0.0, 1.0))), _e22.g1_, _e24.g2_, _e26.g3_, _e28.g4_, _e30.g5_, (_e32.g6_ + vec3<f32>(_e34.g0_.x, _e37.g0_.y, _e40.g0_.z)), _e45.g7_, _e47.g8_, _e49.g9_);
}

fn multi_vector_rotor_sub(self_968: MultiVector, other_780: Rotor) -> MultiVector {
    var self_969: MultiVector;
    var other_781: Rotor;

    self_969 = self_968;
    other_781 = other_780;
    let _e4: MultiVector = self_969;
    let _e6: Rotor = other_781;
    let _e9: Rotor = other_781;
    let _e12: Rotor = other_781;
    let _e22: MultiVector = self_969;
    let _e24: MultiVector = self_969;
    let _e26: MultiVector = self_969;
    let _e28: MultiVector = self_969;
    let _e30: MultiVector = self_969;
    let _e32: MultiVector = self_969;
    let _e34: Rotor = other_781;
    let _e37: Rotor = other_781;
    let _e40: Rotor = other_781;
    let _e45: MultiVector = self_969;
    let _e47: MultiVector = self_969;
    let _e49: MultiVector = self_969;
    return MultiVector((_e4.g0_ - (vec3<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.w) * vec3<f32>(0.0, 0.0, 1.0))), _e22.g1_, _e24.g2_, _e26.g3_, _e28.g4_, _e30.g5_, (_e32.g6_ - vec3<f32>(_e34.g0_.x, _e37.g0_.y, _e40.g0_.z)), _e45.g7_, _e47.g8_, _e49.g9_);
}

fn multi_vector_translator_into(self_970: MultiVector) -> Translator {
    var self_971: MultiVector;

    self_971 = self_970;
    let _e2: MultiVector = self_971;
    let _e5: MultiVector = self_971;
    let _e8: MultiVector = self_971;
    let _e11: MultiVector = self_971;
    return Translator(vec4<f32>(_e2.g7_.x, _e5.g7_.y, _e8.g7_.z, _e11.g0_.z));
}

fn multi_vector_translator_add(self_972: MultiVector, other_782: Translator) -> MultiVector {
    var self_973: MultiVector;
    var other_783: Translator;

    self_973 = self_972;
    other_783 = other_782;
    let _e4: MultiVector = self_973;
    let _e6: Translator = other_783;
    let _e9: Translator = other_783;
    let _e12: Translator = other_783;
    let _e22: MultiVector = self_973;
    let _e24: MultiVector = self_973;
    let _e26: MultiVector = self_973;
    let _e28: MultiVector = self_973;
    let _e30: MultiVector = self_973;
    let _e32: MultiVector = self_973;
    let _e34: MultiVector = self_973;
    let _e36: Translator = other_783;
    let _e39: Translator = other_783;
    let _e42: Translator = other_783;
    let _e47: MultiVector = self_973;
    let _e49: MultiVector = self_973;
    return MultiVector((_e4.g0_ + (vec3<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.w) * vec3<f32>(0.0, 0.0, 1.0))), _e22.g1_, _e24.g2_, _e26.g3_, _e28.g4_, _e30.g5_, _e32.g6_, (_e34.g7_ + vec3<f32>(_e36.g0_.x, _e39.g0_.y, _e42.g0_.z)), _e47.g8_, _e49.g9_);
}

fn multi_vector_translator_sub(self_974: MultiVector, other_784: Translator) -> MultiVector {
    var self_975: MultiVector;
    var other_785: Translator;

    self_975 = self_974;
    other_785 = other_784;
    let _e4: MultiVector = self_975;
    let _e6: Translator = other_785;
    let _e9: Translator = other_785;
    let _e12: Translator = other_785;
    let _e22: MultiVector = self_975;
    let _e24: MultiVector = self_975;
    let _e26: MultiVector = self_975;
    let _e28: MultiVector = self_975;
    let _e30: MultiVector = self_975;
    let _e32: MultiVector = self_975;
    let _e34: MultiVector = self_975;
    let _e36: Translator = other_785;
    let _e39: Translator = other_785;
    let _e42: Translator = other_785;
    let _e47: MultiVector = self_975;
    let _e49: MultiVector = self_975;
    return MultiVector((_e4.g0_ - (vec3<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.w) * vec3<f32>(0.0, 0.0, 1.0))), _e22.g1_, _e24.g2_, _e26.g3_, _e28.g4_, _e30.g5_, _e32.g6_, (_e34.g7_ - vec3<f32>(_e36.g0_.x, _e39.g0_.y, _e42.g0_.z)), _e47.g8_, _e49.g9_);
}

fn multi_vector_flector_into(self_976: MultiVector) -> Flector {
    var self_977: MultiVector;

    self_977 = self_976;
    let _e2: MultiVector = self_977;
    let _e4: MultiVector = self_977;
    return Flector(_e2.g3_, _e4.g9_);
}

fn multi_vector_flector_add(self_978: MultiVector, other_786: Flector) -> MultiVector {
    var self_979: MultiVector;
    var other_787: Flector;

    self_979 = self_978;
    other_787 = other_786;
    let _e4: MultiVector = self_979;
    let _e6: MultiVector = self_979;
    let _e8: MultiVector = self_979;
    let _e10: MultiVector = self_979;
    let _e12: Flector = other_787;
    let _e15: MultiVector = self_979;
    let _e17: MultiVector = self_979;
    let _e19: MultiVector = self_979;
    let _e21: MultiVector = self_979;
    let _e23: MultiVector = self_979;
    let _e25: MultiVector = self_979;
    let _e27: Flector = other_787;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, (_e10.g3_ + _e12.g0_), _e15.g4_, _e17.g5_, _e19.g6_, _e21.g7_, _e23.g8_, (_e25.g9_ + _e27.g1_));
}

fn multi_vector_flector_sub(self_980: MultiVector, other_788: Flector) -> MultiVector {
    var self_981: MultiVector;
    var other_789: Flector;

    self_981 = self_980;
    other_789 = other_788;
    let _e4: MultiVector = self_981;
    let _e6: MultiVector = self_981;
    let _e8: MultiVector = self_981;
    let _e10: MultiVector = self_981;
    let _e12: Flector = other_789;
    let _e15: MultiVector = self_981;
    let _e17: MultiVector = self_981;
    let _e19: MultiVector = self_981;
    let _e21: MultiVector = self_981;
    let _e23: MultiVector = self_981;
    let _e25: MultiVector = self_981;
    let _e27: Flector = other_789;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, (_e10.g3_ - _e12.g0_), _e15.g4_, _e17.g5_, _e19.g6_, _e21.g7_, _e23.g8_, (_e25.g9_ - _e27.g1_));
}

fn multi_vector_multi_vector_add(self_982: MultiVector, other_790: MultiVector) -> MultiVector {
    var self_983: MultiVector;
    var other_791: MultiVector;

    self_983 = self_982;
    other_791 = other_790;
    let _e4: MultiVector = self_983;
    let _e6: MultiVector = other_791;
    let _e9: MultiVector = self_983;
    let _e11: MultiVector = other_791;
    let _e14: MultiVector = self_983;
    let _e16: MultiVector = other_791;
    let _e19: MultiVector = self_983;
    let _e21: MultiVector = other_791;
    let _e24: MultiVector = self_983;
    let _e26: MultiVector = other_791;
    let _e29: MultiVector = self_983;
    let _e31: MultiVector = other_791;
    let _e34: MultiVector = self_983;
    let _e36: MultiVector = other_791;
    let _e39: MultiVector = self_983;
    let _e41: MultiVector = other_791;
    let _e44: MultiVector = self_983;
    let _e46: MultiVector = other_791;
    let _e49: MultiVector = self_983;
    let _e51: MultiVector = other_791;
    return MultiVector((_e4.g0_ + _e6.g0_), (_e9.g1_ + _e11.g1_), (_e14.g2_ + _e16.g2_), (_e19.g3_ + _e21.g3_), (_e24.g4_ + _e26.g4_), (_e29.g5_ + _e31.g5_), (_e34.g6_ + _e36.g6_), (_e39.g7_ + _e41.g7_), (_e44.g8_ + _e46.g8_), (_e49.g9_ + _e51.g9_));
}

fn multi_vector_multi_vector_sub(self_984: MultiVector, other_792: MultiVector) -> MultiVector {
    var self_985: MultiVector;
    var other_793: MultiVector;

    self_985 = self_984;
    other_793 = other_792;
    let _e4: MultiVector = self_985;
    let _e6: MultiVector = other_793;
    let _e9: MultiVector = self_985;
    let _e11: MultiVector = other_793;
    let _e14: MultiVector = self_985;
    let _e16: MultiVector = other_793;
    let _e19: MultiVector = self_985;
    let _e21: MultiVector = other_793;
    let _e24: MultiVector = self_985;
    let _e26: MultiVector = other_793;
    let _e29: MultiVector = self_985;
    let _e31: MultiVector = other_793;
    let _e34: MultiVector = self_985;
    let _e36: MultiVector = other_793;
    let _e39: MultiVector = self_985;
    let _e41: MultiVector = other_793;
    let _e44: MultiVector = self_985;
    let _e46: MultiVector = other_793;
    let _e49: MultiVector = self_985;
    let _e51: MultiVector = other_793;
    return MultiVector((_e4.g0_ - _e6.g0_), (_e9.g1_ - _e11.g1_), (_e14.g2_ - _e16.g2_), (_e19.g3_ - _e21.g3_), (_e24.g4_ - _e26.g4_), (_e29.g5_ - _e31.g5_), (_e34.g6_ - _e36.g6_), (_e39.g7_ - _e41.g7_), (_e44.g8_ - _e46.g8_), (_e49.g9_ - _e51.g9_));
}

fn multi_vector_multi_vector_mul(self_986: MultiVector, other_794: MultiVector) -> MultiVector {
    var self_987: MultiVector;
    var other_795: MultiVector;

    self_987 = self_986;
    other_795 = other_794;
    let _e4: MultiVector = self_987;
    let _e6: MultiVector = other_795;
    let _e9: MultiVector = self_987;
    let _e11: MultiVector = other_795;
    let _e14: MultiVector = self_987;
    let _e16: MultiVector = other_795;
    let _e19: MultiVector = self_987;
    let _e21: MultiVector = other_795;
    let _e24: MultiVector = self_987;
    let _e26: MultiVector = other_795;
    let _e29: MultiVector = self_987;
    let _e31: MultiVector = other_795;
    let _e34: MultiVector = self_987;
    let _e36: MultiVector = other_795;
    let _e39: MultiVector = self_987;
    let _e41: MultiVector = other_795;
    let _e44: MultiVector = self_987;
    let _e46: MultiVector = other_795;
    let _e49: MultiVector = self_987;
    let _e51: MultiVector = other_795;
    return MultiVector((_e4.g0_ * _e6.g0_), (_e9.g1_ * _e11.g1_), (_e14.g2_ * _e16.g2_), (_e19.g3_ * _e21.g3_), (_e24.g4_ * _e26.g4_), (_e29.g5_ * _e31.g5_), (_e34.g6_ * _e36.g6_), (_e39.g7_ * _e41.g7_), (_e44.g8_ * _e46.g8_), (_e49.g9_ * _e51.g9_));
}

fn multi_vector_multi_vector_div(self_988: MultiVector, other_796: MultiVector) -> MultiVector {
    var self_989: MultiVector;
    var other_797: MultiVector;

    self_989 = self_988;
    other_797 = other_796;
    let _e4: MultiVector = self_989;
    let _e7: MultiVector = self_989;
    let _e10: MultiVector = self_989;
    let _e19: MultiVector = other_797;
    let _e22: MultiVector = other_797;
    let _e25: MultiVector = other_797;
    let _e35: MultiVector = self_989;
    let _e38: MultiVector = self_989;
    let _e41: MultiVector = self_989;
    let _e50: MultiVector = other_797;
    let _e53: MultiVector = other_797;
    let _e56: MultiVector = other_797;
    let _e66: MultiVector = self_989;
    let _e69: MultiVector = self_989;
    let _e77: MultiVector = other_797;
    let _e80: MultiVector = other_797;
    let _e89: MultiVector = self_989;
    let _e92: MultiVector = self_989;
    let _e95: MultiVector = self_989;
    let _e98: MultiVector = self_989;
    let _e108: MultiVector = other_797;
    let _e111: MultiVector = other_797;
    let _e114: MultiVector = other_797;
    let _e117: MultiVector = other_797;
    let _e128: MultiVector = self_989;
    let _e131: MultiVector = self_989;
    let _e134: MultiVector = self_989;
    let _e143: MultiVector = other_797;
    let _e146: MultiVector = other_797;
    let _e149: MultiVector = other_797;
    let _e159: MultiVector = self_989;
    let _e162: MultiVector = self_989;
    let _e165: MultiVector = self_989;
    let _e174: MultiVector = other_797;
    let _e177: MultiVector = other_797;
    let _e180: MultiVector = other_797;
    let _e190: MultiVector = self_989;
    let _e193: MultiVector = self_989;
    let _e196: MultiVector = self_989;
    let _e205: MultiVector = other_797;
    let _e208: MultiVector = other_797;
    let _e211: MultiVector = other_797;
    let _e221: MultiVector = self_989;
    let _e224: MultiVector = self_989;
    let _e227: MultiVector = self_989;
    let _e236: MultiVector = other_797;
    let _e239: MultiVector = other_797;
    let _e242: MultiVector = other_797;
    let _e252: MultiVector = self_989;
    let _e255: MultiVector = self_989;
    let _e258: MultiVector = self_989;
    let _e261: MultiVector = self_989;
    let _e271: MultiVector = other_797;
    let _e274: MultiVector = other_797;
    let _e277: MultiVector = other_797;
    let _e280: MultiVector = other_797;
    let _e291: MultiVector = self_989;
    let _e294: MultiVector = self_989;
    let _e297: MultiVector = self_989;
    let _e300: MultiVector = self_989;
    let _e310: MultiVector = other_797;
    let _e313: MultiVector = other_797;
    let _e316: MultiVector = other_797;
    let _e319: MultiVector = other_797;
    return MultiVector((((vec3<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e19.g0_.x, _e22.g0_.y, _e25.g0_.z)) * vec3<f32>(1.0, 1.0, 1.0)), (((vec3<f32>(_e35.g1_.x, _e38.g1_.y, _e41.g1_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e50.g1_.x, _e53.g1_.y, _e56.g1_.z)) * vec3<f32>(1.0, 1.0, 1.0)), (((vec2<f32>(_e66.g2_.x, _e69.g2_.y) * vec2<f32>(1.0, 1.0)) / vec2<f32>(_e77.g2_.x, _e80.g2_.y)) * vec2<f32>(1.0, 1.0)), (((vec4<f32>(_e89.g3_.x, _e92.g3_.y, _e95.g3_.z, _e98.g3_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e108.g3_.x, _e111.g3_.y, _e114.g3_.z, _e117.g3_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)), (((vec3<f32>(_e128.g4_.x, _e131.g4_.y, _e134.g4_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e143.g4_.x, _e146.g4_.y, _e149.g4_.z)) * vec3<f32>(1.0, 1.0, 1.0)), (((vec3<f32>(_e159.g5_.x, _e162.g5_.y, _e165.g5_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e174.g5_.x, _e177.g5_.y, _e180.g5_.z)) * vec3<f32>(1.0, 1.0, 1.0)), (((vec3<f32>(_e190.g6_.x, _e193.g6_.y, _e196.g6_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e205.g6_.x, _e208.g6_.y, _e211.g6_.z)) * vec3<f32>(1.0, 1.0, 1.0)), (((vec3<f32>(_e221.g7_.x, _e224.g7_.y, _e227.g7_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e236.g7_.x, _e239.g7_.y, _e242.g7_.z)) * vec3<f32>(1.0, 1.0, 1.0)), (((vec4<f32>(_e252.g8_.x, _e255.g8_.y, _e258.g8_.z, _e261.g8_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e271.g8_.x, _e274.g8_.y, _e277.g8_.z, _e280.g8_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)), (((vec4<f32>(_e291.g9_.x, _e294.g9_.y, _e297.g9_.z, _e300.g9_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e310.g9_.x, _e313.g9_.y, _e316.g9_.z, _e319.g9_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn multi_vector_multi_vector_geometric_product(self_990: MultiVector, other_798: MultiVector) -> MultiVector {
    var self_991: MultiVector;
    var other_799: MultiVector;

    self_991 = self_990;
    other_799 = other_798;
    let _e4: MultiVector = self_991;
    let _e8: MultiVector = other_799;
    let _e11: MultiVector = self_991;
    let _e15: MultiVector = other_799;
    let _e26: MultiVector = self_991;
    let _e30: MultiVector = other_799;
    let _e33: MultiVector = other_799;
    let _e36: MultiVector = other_799;
    let _e42: MultiVector = self_991;
    let _e46: MultiVector = other_799;
    let _e49: MultiVector = other_799;
    let _e52: MultiVector = other_799;
    let _e58: MultiVector = self_991;
    let _e62: MultiVector = other_799;
    let _e65: MultiVector = other_799;
    let _e68: MultiVector = other_799;
    let _e74: MultiVector = self_991;
    let _e78: MultiVector = other_799;
    let _e81: MultiVector = other_799;
    let _e84: MultiVector = other_799;
    let _e95: MultiVector = self_991;
    let _e99: MultiVector = other_799;
    let _e110: MultiVector = self_991;
    let _e114: MultiVector = other_799;
    let _e126: MultiVector = self_991;
    let _e130: MultiVector = other_799;
    let _e142: MultiVector = self_991;
    let _e146: MultiVector = other_799;
    let _e158: MultiVector = self_991;
    let _e162: MultiVector = other_799;
    let _e174: MultiVector = self_991;
    let _e178: MultiVector = other_799;
    let _e181: MultiVector = other_799;
    let _e184: MultiVector = other_799;
    let _e197: MultiVector = self_991;
    let _e201: MultiVector = other_799;
    let _e204: MultiVector = other_799;
    let _e207: MultiVector = other_799;
    let _e220: MultiVector = self_991;
    let _e224: MultiVector = other_799;
    let _e227: MultiVector = other_799;
    let _e230: MultiVector = other_799;
    let _e243: MultiVector = self_991;
    let _e247: MultiVector = other_799;
    let _e250: MultiVector = other_799;
    let _e253: MultiVector = other_799;
    let _e259: MultiVector = self_991;
    let _e263: MultiVector = other_799;
    let _e266: MultiVector = other_799;
    let _e269: MultiVector = other_799;
    let _e275: MultiVector = self_991;
    let _e279: MultiVector = other_799;
    let _e282: MultiVector = other_799;
    let _e285: MultiVector = other_799;
    let _e291: MultiVector = self_991;
    let _e295: MultiVector = other_799;
    let _e307: MultiVector = self_991;
    let _e311: MultiVector = other_799;
    let _e323: MultiVector = self_991;
    let _e327: MultiVector = other_799;
    let _e339: MultiVector = self_991;
    let _e343: MultiVector = other_799;
    let _e355: MultiVector = self_991;
    let _e359: MultiVector = other_799;
    let _e371: MultiVector = self_991;
    let _e375: MultiVector = other_799;
    let _e387: MultiVector = self_991;
    let _e391: MultiVector = other_799;
    let _e394: MultiVector = other_799;
    let _e397: MultiVector = other_799;
    let _e410: MultiVector = self_991;
    let _e414: MultiVector = other_799;
    let _e417: MultiVector = other_799;
    let _e420: MultiVector = other_799;
    let _e433: MultiVector = self_991;
    let _e437: MultiVector = other_799;
    let _e440: MultiVector = other_799;
    let _e443: MultiVector = other_799;
    let _e456: MultiVector = self_991;
    let _e460: MultiVector = other_799;
    let _e463: MultiVector = other_799;
    let _e466: MultiVector = other_799;
    let _e472: MultiVector = self_991;
    let _e476: MultiVector = other_799;
    let _e487: MultiVector = self_991;
    let _e491: MultiVector = other_799;
    let _e502: MultiVector = self_991;
    let _e506: MultiVector = other_799;
    let _e517: MultiVector = self_991;
    let _e521: MultiVector = other_799;
    let _e532: MultiVector = self_991;
    let _e535: MultiVector = other_799;
    let _e538: MultiVector = other_799;
    let _e541: MultiVector = other_799;
    let _e552: MultiVector = self_991;
    let _e556: MultiVector = other_799;
    let _e559: MultiVector = self_991;
    let _e563: MultiVector = other_799;
    let _e566: MultiVector = other_799;
    let _e569: MultiVector = other_799;
    let _e581: MultiVector = self_991;
    let _e585: MultiVector = other_799;
    let _e588: MultiVector = other_799;
    let _e591: MultiVector = other_799;
    let _e603: MultiVector = self_991;
    let _e607: MultiVector = other_799;
    let _e610: MultiVector = other_799;
    let _e613: MultiVector = other_799;
    let _e625: MultiVector = self_991;
    let _e629: MultiVector = other_799;
    let _e632: MultiVector = other_799;
    let _e635: MultiVector = other_799;
    let _e647: MultiVector = self_991;
    let _e651: MultiVector = other_799;
    let _e654: MultiVector = other_799;
    let _e657: MultiVector = other_799;
    let _e669: MultiVector = self_991;
    let _e673: MultiVector = other_799;
    let _e676: MultiVector = other_799;
    let _e679: MultiVector = other_799;
    let _e691: MultiVector = self_991;
    let _e695: MultiVector = other_799;
    let _e699: MultiVector = self_991;
    let _e703: MultiVector = other_799;
    let _e706: MultiVector = self_991;
    let _e710: MultiVector = other_799;
    let _e713: MultiVector = other_799;
    let _e724: MultiVector = self_991;
    let _e728: MultiVector = other_799;
    let _e731: MultiVector = other_799;
    let _e742: MultiVector = self_991;
    let _e746: MultiVector = other_799;
    let _e749: MultiVector = other_799;
    let _e760: MultiVector = self_991;
    let _e764: MultiVector = other_799;
    let _e774: MultiVector = self_991;
    let _e778: MultiVector = other_799;
    let _e788: MultiVector = self_991;
    let _e792: MultiVector = other_799;
    let _e803: MultiVector = self_991;
    let _e807: MultiVector = other_799;
    let _e818: MultiVector = self_991;
    let _e822: MultiVector = other_799;
    let _e833: MultiVector = self_991;
    let _e837: MultiVector = other_799;
    let _e847: MultiVector = self_991;
    let _e851: MultiVector = other_799;
    let _e861: MultiVector = self_991;
    let _e865: MultiVector = other_799;
    let _e875: MultiVector = self_991;
    let _e879: MultiVector = other_799;
    let _e882: MultiVector = other_799;
    let _e888: MultiVector = self_991;
    let _e892: MultiVector = other_799;
    let _e895: MultiVector = other_799;
    let _e901: MultiVector = self_991;
    let _e905: MultiVector = other_799;
    let _e908: MultiVector = other_799;
    let _e914: MultiVector = self_991;
    let _e918: MultiVector = other_799;
    let _e929: MultiVector = self_991;
    let _e933: MultiVector = other_799;
    let _e944: MultiVector = self_991;
    let _e948: MultiVector = other_799;
    let _e959: MultiVector = self_991;
    let _e963: MultiVector = other_799;
    let _e974: MultiVector = self_991;
    let _e978: MultiVector = other_799;
    let _e989: MultiVector = self_991;
    let _e993: MultiVector = other_799;
    let _e1004: MultiVector = self_991;
    let _e1008: MultiVector = other_799;
    let _e1011: MultiVector = other_799;
    let _e1022: MultiVector = self_991;
    let _e1026: MultiVector = other_799;
    let _e1036: MultiVector = self_991;
    let _e1039: MultiVector = self_991;
    let _e1043: MultiVector = other_799;
    let _e1046: MultiVector = other_799;
    let _e1057: MultiVector = self_991;
    let _e1061: MultiVector = other_799;
    let _e1064: MultiVector = self_991;
    let _e1068: MultiVector = other_799;
    let _e1080: MultiVector = self_991;
    let _e1084: MultiVector = other_799;
    let _e1087: MultiVector = other_799;
    let _e1090: MultiVector = other_799;
    let _e1093: MultiVector = other_799;
    let _e1107: MultiVector = self_991;
    let _e1111: MultiVector = other_799;
    let _e1114: MultiVector = other_799;
    let _e1117: MultiVector = other_799;
    let _e1120: MultiVector = other_799;
    let _e1134: MultiVector = self_991;
    let _e1138: MultiVector = other_799;
    let _e1141: MultiVector = other_799;
    let _e1144: MultiVector = other_799;
    let _e1147: MultiVector = other_799;
    let _e1161: MultiVector = self_991;
    let _e1165: MultiVector = other_799;
    let _e1177: MultiVector = self_991;
    let _e1181: MultiVector = other_799;
    let _e1184: MultiVector = other_799;
    let _e1187: MultiVector = other_799;
    let _e1190: MultiVector = other_799;
    let _e1196: MultiVector = self_991;
    let _e1200: MultiVector = other_799;
    let _e1203: MultiVector = other_799;
    let _e1206: MultiVector = other_799;
    let _e1209: MultiVector = other_799;
    let _e1223: MultiVector = self_991;
    let _e1227: MultiVector = other_799;
    let _e1230: MultiVector = other_799;
    let _e1233: MultiVector = other_799;
    let _e1236: MultiVector = other_799;
    let _e1250: MultiVector = self_991;
    let _e1254: MultiVector = other_799;
    let _e1257: MultiVector = other_799;
    let _e1260: MultiVector = other_799;
    let _e1263: MultiVector = other_799;
    let _e1277: MultiVector = self_991;
    let _e1281: MultiVector = other_799;
    let _e1293: MultiVector = self_991;
    let _e1297: MultiVector = other_799;
    let _e1309: MultiVector = self_991;
    let _e1313: MultiVector = other_799;
    let _e1325: MultiVector = self_991;
    let _e1329: MultiVector = other_799;
    let _e1341: MultiVector = self_991;
    let _e1345: MultiVector = other_799;
    let _e1348: MultiVector = other_799;
    let _e1351: MultiVector = other_799;
    let _e1354: MultiVector = other_799;
    let _e1368: MultiVector = self_991;
    let _e1372: MultiVector = other_799;
    let _e1375: MultiVector = other_799;
    let _e1378: MultiVector = other_799;
    let _e1381: MultiVector = other_799;
    let _e1395: MultiVector = self_991;
    let _e1399: MultiVector = other_799;
    let _e1402: MultiVector = other_799;
    let _e1405: MultiVector = other_799;
    let _e1408: MultiVector = other_799;
    let _e1422: MultiVector = self_991;
    let _e1426: MultiVector = other_799;
    let _e1439: MultiVector = self_991;
    let _e1443: MultiVector = other_799;
    let _e1456: MultiVector = self_991;
    let _e1460: MultiVector = other_799;
    let _e1473: MultiVector = self_991;
    let _e1477: MultiVector = other_799;
    let _e1480: MultiVector = other_799;
    let _e1483: MultiVector = other_799;
    let _e1486: MultiVector = other_799;
    let _e1500: MultiVector = self_991;
    let _e1504: MultiVector = other_799;
    let _e1507: MultiVector = other_799;
    let _e1510: MultiVector = other_799;
    let _e1513: MultiVector = other_799;
    let _e1527: MultiVector = self_991;
    let _e1531: MultiVector = other_799;
    let _e1534: MultiVector = other_799;
    let _e1537: MultiVector = other_799;
    let _e1540: MultiVector = other_799;
    let _e1554: MultiVector = self_991;
    let _e1558: MultiVector = other_799;
    let _e1571: MultiVector = self_991;
    let _e1575: MultiVector = other_799;
    let _e1588: MultiVector = self_991;
    let _e1592: MultiVector = other_799;
    let _e1605: MultiVector = self_991;
    let _e1609: MultiVector = other_799;
    let _e1612: MultiVector = other_799;
    let _e1615: MultiVector = other_799;
    let _e1618: MultiVector = other_799;
    let _e1624: MultiVector = self_991;
    let _e1628: MultiVector = other_799;
    let _e1641: MultiVector = self_991;
    let _e1645: MultiVector = other_799;
    let _e1658: MultiVector = self_991;
    let _e1662: MultiVector = other_799;
    let _e1675: MultiVector = self_991;
    let _e1679: MultiVector = other_799;
    let _e1682: MultiVector = other_799;
    let _e1685: MultiVector = other_799;
    let _e1688: MultiVector = other_799;
    let _e1694: MultiVector = self_991;
    let _e1697: MultiVector = self_991;
    let _e1700: MultiVector = self_991;
    let _e1703: MultiVector = self_991;
    let _e1707: MultiVector = other_799;
    let _e1719: MultiVector = self_991;
    let _e1723: MultiVector = other_799;
    let _e1726: MultiVector = self_991;
    let _e1730: MultiVector = other_799;
    let _e1734: MultiVector = self_991;
    let _e1738: MultiVector = other_799;
    let _e1741: MultiVector = other_799;
    let _e1744: MultiVector = other_799;
    let _e1757: MultiVector = self_991;
    let _e1761: MultiVector = other_799;
    let _e1764: MultiVector = other_799;
    let _e1767: MultiVector = other_799;
    let _e1780: MultiVector = self_991;
    let _e1784: MultiVector = other_799;
    let _e1787: MultiVector = other_799;
    let _e1790: MultiVector = other_799;
    let _e1803: MultiVector = self_991;
    let _e1807: MultiVector = other_799;
    let _e1811: MultiVector = self_991;
    let _e1815: MultiVector = other_799;
    let _e1818: MultiVector = other_799;
    let _e1821: MultiVector = other_799;
    let _e1833: MultiVector = self_991;
    let _e1837: MultiVector = other_799;
    let _e1840: MultiVector = other_799;
    let _e1843: MultiVector = other_799;
    let _e1855: MultiVector = self_991;
    let _e1859: MultiVector = other_799;
    let _e1862: MultiVector = other_799;
    let _e1865: MultiVector = other_799;
    let _e1877: MultiVector = self_991;
    let _e1881: MultiVector = other_799;
    let _e1884: MultiVector = other_799;
    let _e1887: MultiVector = other_799;
    let _e1899: MultiVector = self_991;
    let _e1903: MultiVector = other_799;
    let _e1906: MultiVector = other_799;
    let _e1909: MultiVector = other_799;
    let _e1921: MultiVector = self_991;
    let _e1925: MultiVector = other_799;
    let _e1928: MultiVector = other_799;
    let _e1931: MultiVector = other_799;
    let _e1943: MultiVector = self_991;
    let _e1947: MultiVector = other_799;
    let _e1950: MultiVector = other_799;
    let _e1953: MultiVector = other_799;
    let _e1965: MultiVector = self_991;
    let _e1969: MultiVector = other_799;
    let _e1972: MultiVector = other_799;
    let _e1975: MultiVector = other_799;
    let _e1987: MultiVector = self_991;
    let _e1991: MultiVector = other_799;
    let _e1994: MultiVector = other_799;
    let _e1997: MultiVector = other_799;
    let _e2009: MultiVector = self_991;
    let _e2013: MultiVector = other_799;
    let _e2016: MultiVector = other_799;
    let _e2019: MultiVector = other_799;
    let _e2025: MultiVector = self_991;
    let _e2029: MultiVector = other_799;
    let _e2032: MultiVector = self_991;
    let _e2036: MultiVector = other_799;
    let _e2039: MultiVector = other_799;
    let _e2042: MultiVector = other_799;
    let _e2055: MultiVector = self_991;
    let _e2059: MultiVector = other_799;
    let _e2062: MultiVector = other_799;
    let _e2065: MultiVector = other_799;
    let _e2078: MultiVector = self_991;
    let _e2082: MultiVector = other_799;
    let _e2085: MultiVector = other_799;
    let _e2088: MultiVector = other_799;
    let _e2101: MultiVector = self_991;
    let _e2105: MultiVector = other_799;
    let _e2108: MultiVector = other_799;
    let _e2111: MultiVector = other_799;
    let _e2123: MultiVector = self_991;
    let _e2127: MultiVector = other_799;
    let _e2130: MultiVector = other_799;
    let _e2133: MultiVector = other_799;
    let _e2145: MultiVector = self_991;
    let _e2149: MultiVector = other_799;
    let _e2152: MultiVector = other_799;
    let _e2155: MultiVector = other_799;
    let _e2167: MultiVector = self_991;
    let _e2171: MultiVector = other_799;
    let _e2175: MultiVector = self_991;
    let _e2179: MultiVector = other_799;
    let _e2182: MultiVector = self_991;
    let _e2186: MultiVector = other_799;
    let _e2190: MultiVector = self_991;
    let _e2194: MultiVector = other_799;
    let _e2198: MultiVector = self_991;
    let _e2202: MultiVector = other_799;
    let _e2205: MultiVector = other_799;
    let _e2208: MultiVector = other_799;
    let _e2221: MultiVector = self_991;
    let _e2225: MultiVector = other_799;
    let _e2228: MultiVector = other_799;
    let _e2231: MultiVector = other_799;
    let _e2244: MultiVector = self_991;
    let _e2248: MultiVector = other_799;
    let _e2251: MultiVector = other_799;
    let _e2254: MultiVector = other_799;
    let _e2267: MultiVector = self_991;
    let _e2271: MultiVector = other_799;
    let _e2274: MultiVector = other_799;
    let _e2277: MultiVector = other_799;
    let _e2283: MultiVector = self_991;
    let _e2287: MultiVector = other_799;
    let _e2291: MultiVector = self_991;
    let _e2295: MultiVector = other_799;
    let _e2298: MultiVector = other_799;
    let _e2301: MultiVector = other_799;
    let _e2313: MultiVector = self_991;
    let _e2317: MultiVector = other_799;
    let _e2320: MultiVector = other_799;
    let _e2323: MultiVector = other_799;
    let _e2335: MultiVector = self_991;
    let _e2339: MultiVector = other_799;
    let _e2342: MultiVector = other_799;
    let _e2345: MultiVector = other_799;
    let _e2357: MultiVector = self_991;
    let _e2361: MultiVector = other_799;
    let _e2365: MultiVector = self_991;
    let _e2369: MultiVector = other_799;
    let _e2372: MultiVector = other_799;
    let _e2375: MultiVector = other_799;
    let _e2387: MultiVector = self_991;
    let _e2391: MultiVector = other_799;
    let _e2394: MultiVector = other_799;
    let _e2397: MultiVector = other_799;
    let _e2409: MultiVector = self_991;
    let _e2413: MultiVector = other_799;
    let _e2416: MultiVector = other_799;
    let _e2419: MultiVector = other_799;
    let _e2431: MultiVector = self_991;
    let _e2435: MultiVector = other_799;
    let _e2438: MultiVector = other_799;
    let _e2441: MultiVector = other_799;
    let _e2453: MultiVector = self_991;
    let _e2457: MultiVector = other_799;
    let _e2460: MultiVector = other_799;
    let _e2463: MultiVector = other_799;
    let _e2475: MultiVector = self_991;
    let _e2479: MultiVector = other_799;
    let _e2482: MultiVector = other_799;
    let _e2485: MultiVector = other_799;
    let _e2497: MultiVector = self_991;
    let _e2501: MultiVector = other_799;
    let _e2504: MultiVector = other_799;
    let _e2507: MultiVector = other_799;
    let _e2519: MultiVector = self_991;
    let _e2523: MultiVector = other_799;
    let _e2526: MultiVector = other_799;
    let _e2529: MultiVector = other_799;
    let _e2541: MultiVector = self_991;
    let _e2545: MultiVector = other_799;
    let _e2548: MultiVector = other_799;
    let _e2551: MultiVector = other_799;
    let _e2563: MultiVector = self_991;
    let _e2567: MultiVector = other_799;
    let _e2570: MultiVector = other_799;
    let _e2573: MultiVector = other_799;
    let _e2585: MultiVector = self_991;
    let _e2589: MultiVector = other_799;
    let _e2592: MultiVector = other_799;
    let _e2595: MultiVector = other_799;
    let _e2607: MultiVector = self_991;
    let _e2611: MultiVector = other_799;
    let _e2614: MultiVector = other_799;
    let _e2617: MultiVector = other_799;
    let _e2629: MultiVector = self_991;
    let _e2633: MultiVector = other_799;
    let _e2636: MultiVector = other_799;
    let _e2639: MultiVector = other_799;
    let _e2651: MultiVector = self_991;
    let _e2655: MultiVector = other_799;
    let _e2658: MultiVector = other_799;
    let _e2661: MultiVector = other_799;
    let _e2673: MultiVector = self_991;
    let _e2677: MultiVector = other_799;
    let _e2680: MultiVector = other_799;
    let _e2683: MultiVector = other_799;
    let _e2695: MultiVector = self_991;
    let _e2699: MultiVector = other_799;
    let _e2702: MultiVector = other_799;
    let _e2705: MultiVector = other_799;
    let _e2711: MultiVector = self_991;
    let _e2715: MultiVector = other_799;
    let _e2718: MultiVector = other_799;
    let _e2721: MultiVector = other_799;
    let _e2734: MultiVector = self_991;
    let _e2738: MultiVector = other_799;
    let _e2741: MultiVector = other_799;
    let _e2744: MultiVector = other_799;
    let _e2757: MultiVector = self_991;
    let _e2761: MultiVector = other_799;
    let _e2764: MultiVector = other_799;
    let _e2767: MultiVector = other_799;
    let _e2780: MultiVector = self_991;
    let _e2784: MultiVector = other_799;
    let _e2787: MultiVector = other_799;
    let _e2790: MultiVector = other_799;
    let _e2796: MultiVector = self_991;
    let _e2800: MultiVector = other_799;
    let _e2803: MultiVector = self_991;
    let _e2807: MultiVector = other_799;
    let _e2810: MultiVector = other_799;
    let _e2813: MultiVector = other_799;
    let _e2826: MultiVector = self_991;
    let _e2830: MultiVector = other_799;
    let _e2833: MultiVector = other_799;
    let _e2836: MultiVector = other_799;
    let _e2849: MultiVector = self_991;
    let _e2853: MultiVector = other_799;
    let _e2856: MultiVector = other_799;
    let _e2859: MultiVector = other_799;
    let _e2872: MultiVector = self_991;
    let _e2876: MultiVector = other_799;
    let _e2880: MultiVector = self_991;
    let _e2884: MultiVector = other_799;
    let _e2887: MultiVector = other_799;
    let _e2890: MultiVector = other_799;
    let _e2902: MultiVector = self_991;
    let _e2906: MultiVector = other_799;
    let _e2909: MultiVector = other_799;
    let _e2912: MultiVector = other_799;
    let _e2924: MultiVector = self_991;
    let _e2928: MultiVector = other_799;
    let _e2931: MultiVector = other_799;
    let _e2934: MultiVector = other_799;
    let _e2946: MultiVector = self_991;
    let _e2950: MultiVector = other_799;
    let _e2953: MultiVector = other_799;
    let _e2956: MultiVector = other_799;
    let _e2968: MultiVector = self_991;
    let _e2972: MultiVector = other_799;
    let _e2975: MultiVector = other_799;
    let _e2978: MultiVector = other_799;
    let _e2990: MultiVector = self_991;
    let _e2994: MultiVector = other_799;
    let _e2997: MultiVector = other_799;
    let _e3000: MultiVector = other_799;
    let _e3012: MultiVector = self_991;
    let _e3016: MultiVector = other_799;
    let _e3019: MultiVector = other_799;
    let _e3022: MultiVector = other_799;
    let _e3034: MultiVector = self_991;
    let _e3038: MultiVector = other_799;
    let _e3041: MultiVector = other_799;
    let _e3044: MultiVector = other_799;
    let _e3056: MultiVector = self_991;
    let _e3060: MultiVector = other_799;
    let _e3063: MultiVector = other_799;
    let _e3066: MultiVector = other_799;
    let _e3078: MultiVector = self_991;
    let _e3082: MultiVector = other_799;
    let _e3085: MultiVector = other_799;
    let _e3088: MultiVector = other_799;
    let _e3094: MultiVector = self_991;
    let _e3098: MultiVector = other_799;
    let _e3102: MultiVector = self_991;
    let _e3106: MultiVector = other_799;
    let _e3109: MultiVector = self_991;
    let _e3113: MultiVector = other_799;
    let _e3116: MultiVector = other_799;
    let _e3119: MultiVector = other_799;
    let _e3122: MultiVector = other_799;
    let _e3136: MultiVector = self_991;
    let _e3140: MultiVector = other_799;
    let _e3143: MultiVector = other_799;
    let _e3146: MultiVector = other_799;
    let _e3149: MultiVector = other_799;
    let _e3163: MultiVector = self_991;
    let _e3167: MultiVector = other_799;
    let _e3170: MultiVector = other_799;
    let _e3173: MultiVector = other_799;
    let _e3176: MultiVector = other_799;
    let _e3190: MultiVector = self_991;
    let _e3194: MultiVector = other_799;
    let _e3197: MultiVector = other_799;
    let _e3200: MultiVector = other_799;
    let _e3203: MultiVector = other_799;
    let _e3215: MultiVector = self_991;
    let _e3219: MultiVector = other_799;
    let _e3222: MultiVector = other_799;
    let _e3225: MultiVector = other_799;
    let _e3228: MultiVector = other_799;
    let _e3242: MultiVector = self_991;
    let _e3246: MultiVector = other_799;
    let _e3249: MultiVector = other_799;
    let _e3252: MultiVector = other_799;
    let _e3255: MultiVector = other_799;
    let _e3269: MultiVector = self_991;
    let _e3273: MultiVector = other_799;
    let _e3276: MultiVector = other_799;
    let _e3279: MultiVector = other_799;
    let _e3282: MultiVector = other_799;
    let _e3296: MultiVector = self_991;
    let _e3300: MultiVector = other_799;
    let _e3303: MultiVector = other_799;
    let _e3306: MultiVector = other_799;
    let _e3309: MultiVector = other_799;
    let _e3323: MultiVector = self_991;
    let _e3327: MultiVector = other_799;
    let _e3330: MultiVector = other_799;
    let _e3333: MultiVector = other_799;
    let _e3336: MultiVector = other_799;
    let _e3350: MultiVector = self_991;
    let _e3354: MultiVector = other_799;
    let _e3357: MultiVector = other_799;
    let _e3360: MultiVector = other_799;
    let _e3363: MultiVector = other_799;
    let _e3377: MultiVector = self_991;
    let _e3381: MultiVector = other_799;
    let _e3384: MultiVector = other_799;
    let _e3387: MultiVector = other_799;
    let _e3390: MultiVector = other_799;
    let _e3403: MultiVector = self_991;
    let _e3407: MultiVector = other_799;
    let _e3410: MultiVector = other_799;
    let _e3413: MultiVector = other_799;
    let _e3416: MultiVector = other_799;
    let _e3429: MultiVector = self_991;
    let _e3433: MultiVector = other_799;
    let _e3436: MultiVector = other_799;
    let _e3439: MultiVector = other_799;
    let _e3442: MultiVector = other_799;
    let _e3455: MultiVector = self_991;
    let _e3459: MultiVector = other_799;
    let _e3462: MultiVector = other_799;
    let _e3465: MultiVector = other_799;
    let _e3468: MultiVector = other_799;
    let _e3474: MultiVector = self_991;
    let _e3477: MultiVector = self_991;
    let _e3480: MultiVector = self_991;
    let _e3483: MultiVector = self_991;
    let _e3487: MultiVector = other_799;
    let _e3490: MultiVector = other_799;
    let _e3493: MultiVector = other_799;
    let _e3496: MultiVector = other_799;
    let _e3511: MultiVector = self_991;
    let _e3515: MultiVector = other_799;
    let _e3518: MultiVector = self_991;
    let _e3522: MultiVector = other_799;
    let _e3525: MultiVector = other_799;
    let _e3528: MultiVector = other_799;
    let _e3531: MultiVector = other_799;
    let _e3543: MultiVector = self_991;
    let _e3547: MultiVector = other_799;
    let _e3550: MultiVector = other_799;
    let _e3553: MultiVector = other_799;
    let _e3556: MultiVector = other_799;
    let _e3570: MultiVector = self_991;
    let _e3574: MultiVector = other_799;
    let _e3577: MultiVector = other_799;
    let _e3580: MultiVector = other_799;
    let _e3583: MultiVector = other_799;
    let _e3597: MultiVector = self_991;
    let _e3601: MultiVector = other_799;
    let _e3604: MultiVector = other_799;
    let _e3607: MultiVector = other_799;
    let _e3610: MultiVector = other_799;
    let _e3624: MultiVector = self_991;
    let _e3628: MultiVector = other_799;
    let _e3631: MultiVector = other_799;
    let _e3634: MultiVector = other_799;
    let _e3637: MultiVector = other_799;
    let _e3649: MultiVector = self_991;
    let _e3653: MultiVector = other_799;
    let _e3657: MultiVector = self_991;
    let _e3661: MultiVector = other_799;
    let _e3664: MultiVector = other_799;
    let _e3667: MultiVector = other_799;
    let _e3670: MultiVector = other_799;
    let _e3684: MultiVector = self_991;
    let _e3688: MultiVector = other_799;
    let _e3691: MultiVector = other_799;
    let _e3694: MultiVector = other_799;
    let _e3697: MultiVector = other_799;
    let _e3711: MultiVector = self_991;
    let _e3715: MultiVector = other_799;
    let _e3718: MultiVector = other_799;
    let _e3721: MultiVector = other_799;
    let _e3724: MultiVector = other_799;
    let _e3738: MultiVector = self_991;
    let _e3742: MultiVector = other_799;
    let _e3745: MultiVector = other_799;
    let _e3748: MultiVector = other_799;
    let _e3751: MultiVector = other_799;
    let _e3763: MultiVector = self_991;
    let _e3767: MultiVector = other_799;
    let _e3770: MultiVector = other_799;
    let _e3773: MultiVector = other_799;
    let _e3776: MultiVector = other_799;
    let _e3790: MultiVector = self_991;
    let _e3794: MultiVector = other_799;
    let _e3797: MultiVector = other_799;
    let _e3800: MultiVector = other_799;
    let _e3803: MultiVector = other_799;
    let _e3817: MultiVector = self_991;
    let _e3821: MultiVector = other_799;
    let _e3824: MultiVector = other_799;
    let _e3827: MultiVector = other_799;
    let _e3830: MultiVector = other_799;
    let _e3844: MultiVector = self_991;
    let _e3848: MultiVector = other_799;
    let _e3851: MultiVector = other_799;
    let _e3854: MultiVector = other_799;
    let _e3857: MultiVector = other_799;
    let _e3871: MultiVector = self_991;
    let _e3875: MultiVector = other_799;
    let _e3878: MultiVector = other_799;
    let _e3881: MultiVector = other_799;
    let _e3884: MultiVector = other_799;
    let _e3898: MultiVector = self_991;
    let _e3902: MultiVector = other_799;
    let _e3905: MultiVector = other_799;
    let _e3908: MultiVector = other_799;
    let _e3911: MultiVector = other_799;
    let _e3925: MultiVector = self_991;
    let _e3929: MultiVector = other_799;
    let _e3932: MultiVector = other_799;
    let _e3935: MultiVector = other_799;
    let _e3938: MultiVector = other_799;
    let _e3951: MultiVector = self_991;
    let _e3955: MultiVector = other_799;
    let _e3958: MultiVector = other_799;
    let _e3961: MultiVector = other_799;
    let _e3964: MultiVector = other_799;
    let _e3977: MultiVector = self_991;
    let _e3981: MultiVector = other_799;
    let _e3984: MultiVector = other_799;
    let _e3987: MultiVector = other_799;
    let _e3990: MultiVector = other_799;
    let _e4003: MultiVector = self_991;
    let _e4007: MultiVector = other_799;
    let _e4010: MultiVector = other_799;
    let _e4013: MultiVector = other_799;
    let _e4016: MultiVector = other_799;
    let _e4030: MultiVector = self_991;
    let _e4034: MultiVector = other_799;
    let _e4037: MultiVector = other_799;
    let _e4040: MultiVector = other_799;
    let _e4043: MultiVector = other_799;
    let _e4057: MultiVector = self_991;
    let _e4061: MultiVector = other_799;
    let _e4064: MultiVector = other_799;
    let _e4067: MultiVector = other_799;
    let _e4070: MultiVector = other_799;
    let _e4084: MultiVector = self_991;
    let _e4088: MultiVector = other_799;
    let _e4091: MultiVector = other_799;
    let _e4094: MultiVector = other_799;
    let _e4097: MultiVector = other_799;
    let _e4110: MultiVector = self_991;
    let _e4114: MultiVector = other_799;
    let _e4117: MultiVector = other_799;
    let _e4120: MultiVector = other_799;
    let _e4123: MultiVector = other_799;
    let _e4136: MultiVector = self_991;
    let _e4140: MultiVector = other_799;
    let _e4143: MultiVector = other_799;
    let _e4146: MultiVector = other_799;
    let _e4149: MultiVector = other_799;
    let _e4162: MultiVector = self_991;
    let _e4166: MultiVector = other_799;
    let _e4169: MultiVector = other_799;
    let _e4172: MultiVector = other_799;
    let _e4175: MultiVector = other_799;
    let _e4181: MultiVector = self_991;
    let _e4185: MultiVector = other_799;
    let _e4188: MultiVector = other_799;
    let _e4191: MultiVector = other_799;
    let _e4194: MultiVector = other_799;
    let _e4207: MultiVector = self_991;
    let _e4211: MultiVector = other_799;
    let _e4214: MultiVector = other_799;
    let _e4217: MultiVector = other_799;
    let _e4220: MultiVector = other_799;
    let _e4233: MultiVector = self_991;
    let _e4237: MultiVector = other_799;
    let _e4240: MultiVector = other_799;
    let _e4243: MultiVector = other_799;
    let _e4246: MultiVector = other_799;
    let _e4259: MultiVector = self_991;
    let _e4263: MultiVector = other_799;
    let _e4266: MultiVector = other_799;
    let _e4269: MultiVector = other_799;
    let _e4272: MultiVector = other_799;
    let _e4278: MultiVector = self_991;
    let _e4281: MultiVector = self_991;
    let _e4284: MultiVector = self_991;
    let _e4287: MultiVector = self_991;
    let _e4291: MultiVector = other_799;
    return MultiVector(((((((((((((((((((((((((((((((((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((vec3<f32>(_e11.g0_.z) * vec3<f32>(_e15.g0_.x)) * vec3<f32>(0.0, 0.0, 1.0))) + (vec3<f32>(_e26.g1_.x) * vec3<f32>(_e30.g1_.x, _e33.g8_.x, _e36.g9_.x))) + (vec3<f32>(_e42.g1_.y) * vec3<f32>(_e46.g1_.y, _e49.g8_.y, _e52.g9_.y))) + (vec3<f32>(_e58.g1_.z) * vec3<f32>(_e62.g1_.z, _e65.g8_.z, _e68.g9_.z))) + ((vec3<f32>(_e74.g2_.x) * vec3<f32>(_e78.g8_.w, _e81.g8_.w, _e84.g9_.w)) * vec3<f32>(0.0, 1.0, 1.0))) + ((vec3<f32>(_e95.g2_.y) * vec3<f32>(_e99.g0_.y)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e110.g3_.x) * vec3<f32>(_e114.g8_.x)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e126.g3_.y) * vec3<f32>(_e130.g8_.y)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e142.g3_.z) * vec3<f32>(_e146.g8_.z)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e158.g3_.w) * vec3<f32>(_e162.g8_.w)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e174.g4_.x) * vec3<f32>(_e178.g5_.x, _e181.g5_.x, _e184.g7_.x)) * vec3<f32>(0.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e197.g4_.y) * vec3<f32>(_e201.g5_.y, _e204.g5_.y, _e207.g7_.y)) * vec3<f32>(0.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e220.g4_.z) * vec3<f32>(_e224.g5_.z, _e227.g5_.z, _e230.g7_.z)) * vec3<f32>(0.0, -(1.0), -(1.0)))) - (vec3<f32>(_e243.g5_.x) * vec3<f32>(_e247.g5_.x, _e250.g4_.x, _e253.g6_.x))) - (vec3<f32>(_e259.g5_.y) * vec3<f32>(_e263.g5_.y, _e266.g4_.y, _e269.g6_.y))) - (vec3<f32>(_e275.g5_.z) * vec3<f32>(_e279.g5_.z, _e282.g4_.z, _e285.g6_.z))) + ((vec3<f32>(_e291.g6_.x) * vec3<f32>(_e295.g5_.x)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e307.g6_.y) * vec3<f32>(_e311.g5_.y)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e323.g6_.z) * vec3<f32>(_e327.g5_.z)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e339.g7_.x) * vec3<f32>(_e343.g4_.x)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e355.g7_.y) * vec3<f32>(_e359.g4_.y)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e371.g7_.z) * vec3<f32>(_e375.g4_.z)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e387.g8_.x) * vec3<f32>(_e391.g1_.x, _e394.g1_.x, _e397.g3_.x)) * vec3<f32>(0.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e410.g8_.y) * vec3<f32>(_e414.g1_.y, _e417.g1_.y, _e420.g3_.y)) * vec3<f32>(0.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e433.g8_.z) * vec3<f32>(_e437.g1_.z, _e440.g1_.z, _e443.g3_.z)) * vec3<f32>(0.0, -(1.0), -(1.0)))) - (vec3<f32>(_e456.g8_.w) * vec3<f32>(_e460.g8_.w, _e463.g2_.x, _e466.g3_.w))) + ((vec3<f32>(_e472.g9_.x) * vec3<f32>(_e476.g1_.x)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e487.g9_.y) * vec3<f32>(_e491.g1_.y)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e502.g9_.z) * vec3<f32>(_e506.g1_.z)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e517.g9_.w) * vec3<f32>(_e521.g2_.x)) * vec3<f32>(0.0, 0.0, 1.0))) + ((_e532.g0_.xyy * vec3<f32>(_e535.g0_.x, _e538.g0_.x, _e541.g2_.y)) * vec3<f32>(0.0, 1.0, 1.0))), ((((((((vec3<f32>(_e552.g0_.x) * _e556.g1_) + ((vec3<f32>(_e559.g1_.x) * vec3<f32>(_e563.g0_.x, _e566.g5_.z, _e569.g5_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e581.g1_.y) * vec3<f32>(_e585.g5_.z, _e588.g0_.x, _e591.g5_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e603.g1_.z) * vec3<f32>(_e607.g5_.y, _e610.g5_.x, _e613.g0_.x)) * vec3<f32>(1.0, -(1.0), 1.0))) + ((vec3<f32>(_e625.g5_.x) * vec3<f32>(_e629.g8_.w, _e632.g1_.z, _e635.g1_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e647.g5_.y) * vec3<f32>(_e651.g1_.z, _e654.g8_.w, _e657.g1_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e669.g5_.z) * vec3<f32>(_e673.g1_.y, _e676.g1_.x, _e679.g8_.w)) * vec3<f32>(1.0, -(1.0), 1.0))) + (vec3<f32>(_e691.g8_.w) * _e695.g5_)), ((((((((((((((((((((((((vec2<f32>(_e699.g0_.x) * _e703.g2_) + ((vec2<f32>(_e706.g1_.x) * vec2<f32>(_e710.g4_.x, _e713.g3_.x)) * vec2<f32>(-(1.0), 1.0))) + ((vec2<f32>(_e724.g1_.y) * vec2<f32>(_e728.g4_.y, _e731.g3_.y)) * vec2<f32>(-(1.0), 1.0))) + ((vec2<f32>(_e742.g1_.z) * vec2<f32>(_e746.g4_.z, _e749.g3_.z)) * vec2<f32>(-(1.0), 1.0))) + ((vec2<f32>(_e760.g2_.x) * vec2<f32>(_e764.g0_.x)) * vec2<f32>(1.0, 0.0))) + ((vec2<f32>(_e774.g2_.y) * vec2<f32>(_e778.g0_.x)) * vec2<f32>(0.0, 1.0))) + ((vec2<f32>(_e788.g3_.x) * vec2<f32>(_e792.g1_.x)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e803.g3_.y) * vec2<f32>(_e807.g1_.y)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e818.g3_.z) * vec2<f32>(_e822.g1_.z)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e833.g4_.x) * vec2<f32>(_e837.g1_.x)) * vec2<f32>(1.0, 0.0))) + ((vec2<f32>(_e847.g4_.y) * vec2<f32>(_e851.g1_.y)) * vec2<f32>(1.0, 0.0))) + ((vec2<f32>(_e861.g4_.z) * vec2<f32>(_e865.g1_.z)) * vec2<f32>(1.0, 0.0))) - (vec2<f32>(_e875.g5_.x) * vec2<f32>(_e879.g8_.x, _e882.g7_.x))) - (vec2<f32>(_e888.g5_.y) * vec2<f32>(_e892.g8_.y, _e895.g7_.y))) - (vec2<f32>(_e901.g5_.z) * vec2<f32>(_e905.g8_.z, _e908.g7_.z))) + ((vec2<f32>(_e914.g7_.x) * vec2<f32>(_e918.g5_.x)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e929.g7_.y) * vec2<f32>(_e933.g5_.y)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e944.g7_.z) * vec2<f32>(_e948.g5_.z)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e959.g8_.x) * vec2<f32>(_e963.g5_.x)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e974.g8_.y) * vec2<f32>(_e978.g5_.y)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e989.g8_.z) * vec2<f32>(_e993.g5_.z)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e1004.g8_.w) * vec2<f32>(_e1008.g0_.y, _e1011.g9_.w)) * vec2<f32>(1.0, -(1.0)))) + ((vec2<f32>(_e1022.g9_.w) * vec2<f32>(_e1026.g8_.w)) * vec2<f32>(0.0, 1.0))) + ((vec2<f32>(_e1036.g0_.y, _e1039.g0_.x) * vec2<f32>(_e1043.g8_.w, _e1046.g8_.x)) * vec2<f32>(-(1.0), 0.0))), ((((((((((((((((((((((((((((((((vec4<f32>(_e1057.g0_.x) * _e1061.g3_) + ((vec4<f32>(_e1064.g0_.z) * vec4<f32>(_e1068.g8_.w)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e1080.g1_.x) * vec4<f32>(_e1084.g2_.y, _e1087.g7_.z, _e1090.g7_.y, _e1093.g6_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e1107.g1_.y) * vec4<f32>(_e1111.g7_.z, _e1114.g2_.y, _e1117.g7_.x, _e1120.g6_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e1134.g1_.z) * vec4<f32>(_e1138.g7_.y, _e1141.g7_.x, _e1144.g2_.y, _e1147.g6_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e1161.g2_.x) * vec4<f32>(_e1165.g2_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) - (vec4<f32>(_e1177.g2_.y) * vec4<f32>(_e1181.g1_.x, _e1184.g1_.y, _e1187.g1_.z, _e1190.g2_.x))) + ((vec4<f32>(_e1196.g3_.x) * vec4<f32>(_e1200.g0_.x, _e1203.g5_.z, _e1206.g5_.y, _e1209.g4_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e1223.g3_.y) * vec4<f32>(_e1227.g5_.z, _e1230.g0_.x, _e1233.g5_.x, _e1236.g4_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e1250.g3_.z) * vec4<f32>(_e1254.g5_.y, _e1257.g5_.x, _e1260.g0_.x, _e1263.g4_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e1277.g3_.w) * vec4<f32>(_e1281.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e1293.g4_.x) * vec4<f32>(_e1297.g3_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e1309.g4_.y) * vec4<f32>(_e1313.g3_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e1325.g4_.z) * vec4<f32>(_e1329.g3_.z)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e1341.g5_.x) * vec4<f32>(_e1345.g9_.w, _e1348.g3_.z, _e1351.g3_.y, _e1354.g9_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e1368.g5_.y) * vec4<f32>(_e1372.g3_.z, _e1375.g9_.w, _e1378.g3_.x, _e1381.g9_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e1395.g5_.z) * vec4<f32>(_e1399.g3_.y, _e1402.g3_.x, _e1405.g9_.w, _e1408.g9_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e1422.g6_.x) * vec4<f32>(_e1426.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e1439.g6_.y) * vec4<f32>(_e1443.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e1456.g6_.z) * vec4<f32>(_e1460.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e1473.g7_.x) * vec4<f32>(_e1477.g8_.w, _e1480.g1_.z, _e1483.g1_.y, _e1486.g8_.x)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e1500.g7_.y) * vec4<f32>(_e1504.g1_.z, _e1507.g8_.w, _e1510.g1_.x, _e1513.g8_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e1527.g7_.z) * vec4<f32>(_e1531.g1_.y, _e1534.g1_.x, _e1537.g8_.w, _e1540.g8_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e1554.g8_.x) * vec4<f32>(_e1558.g7_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e1571.g8_.y) * vec4<f32>(_e1575.g7_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e1588.g8_.z) * vec4<f32>(_e1592.g7_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + (vec4<f32>(_e1605.g8_.w) * vec4<f32>(_e1609.g7_.x, _e1612.g7_.y, _e1615.g7_.z, _e1618.g0_.z))) + ((vec4<f32>(_e1624.g9_.x) * vec4<f32>(_e1628.g5_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e1641.g9_.y) * vec4<f32>(_e1645.g5_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e1658.g9_.z) * vec4<f32>(_e1662.g5_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + (vec4<f32>(_e1675.g9_.w) * vec4<f32>(_e1679.g5_.x, _e1682.g5_.y, _e1685.g5_.z, _e1688.g0_.y))) + ((vec4<f32>(_e1694.g0_.x, _e1697.g0_.x, _e1700.g0_.x, _e1703.g0_.y) * _e1707.g9_.xxxw) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))), ((((((((((((((((vec3<f32>(_e1719.g0_.x) * _e1723.g4_) + (vec3<f32>(_e1726.g0_.y) * _e1730.g5_)) + ((vec3<f32>(_e1734.g1_.x) * vec3<f32>(_e1738.g2_.x, _e1741.g8_.z, _e1744.g8_.y)) * vec3<f32>(-(1.0), -(1.0), 1.0))) + ((vec3<f32>(_e1757.g1_.y) * vec3<f32>(_e1761.g8_.z, _e1764.g2_.x, _e1767.g8_.x)) * vec3<f32>(1.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e1780.g1_.z) * vec3<f32>(_e1784.g8_.y, _e1787.g8_.x, _e1790.g2_.x)) * vec3<f32>(-(1.0), 1.0, -(1.0)))) + (vec3<f32>(_e1803.g2_.x) * _e1807.g1_)) + ((vec3<f32>(_e1811.g4_.x) * vec3<f32>(_e1815.g0_.x, _e1818.g5_.z, _e1821.g5_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e1833.g4_.y) * vec3<f32>(_e1837.g5_.z, _e1840.g0_.x, _e1843.g5_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e1855.g4_.z) * vec3<f32>(_e1859.g5_.y, _e1862.g5_.x, _e1865.g0_.x)) * vec3<f32>(1.0, -(1.0), 1.0))) + ((vec3<f32>(_e1877.g5_.x) * vec3<f32>(_e1881.g0_.y, _e1884.g4_.z, _e1887.g4_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e1899.g5_.y) * vec3<f32>(_e1903.g4_.z, _e1906.g0_.y, _e1909.g4_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e1921.g5_.z) * vec3<f32>(_e1925.g4_.y, _e1928.g4_.x, _e1931.g0_.y)) * vec3<f32>(1.0, -(1.0), 1.0))) + ((vec3<f32>(_e1943.g8_.x) * vec3<f32>(_e1947.g8_.w, _e1950.g1_.z, _e1953.g1_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e1965.g8_.y) * vec3<f32>(_e1969.g1_.z, _e1972.g8_.w, _e1975.g1_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e1987.g8_.z) * vec3<f32>(_e1991.g1_.y, _e1994.g1_.x, _e1997.g8_.w)) * vec3<f32>(1.0, -(1.0), 1.0))) - (vec3<f32>(_e2009.g8_.w) * vec3<f32>(_e2013.g8_.x, _e2016.g8_.y, _e2019.g8_.z))), ((((((((vec3<f32>(_e2025.g0_.x) * _e2029.g5_) + ((vec3<f32>(_e2032.g1_.x) * vec3<f32>(_e2036.g8_.w, _e2039.g1_.z, _e2042.g1_.y)) * vec3<f32>(-(1.0), -(1.0), 1.0))) + ((vec3<f32>(_e2055.g1_.y) * vec3<f32>(_e2059.g1_.z, _e2062.g8_.w, _e2065.g1_.x)) * vec3<f32>(1.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e2078.g1_.z) * vec3<f32>(_e2082.g1_.y, _e2085.g1_.x, _e2088.g8_.w)) * vec3<f32>(-(1.0), 1.0, -(1.0)))) + ((vec3<f32>(_e2101.g5_.x) * vec3<f32>(_e2105.g0_.x, _e2108.g5_.z, _e2111.g5_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e2123.g5_.y) * vec3<f32>(_e2127.g5_.z, _e2130.g0_.x, _e2133.g5_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e2145.g5_.z) * vec3<f32>(_e2149.g5_.y, _e2152.g5_.x, _e2155.g0_.x)) * vec3<f32>(1.0, -(1.0), 1.0))) - (vec3<f32>(_e2167.g8_.w) * _e2171.g1_)), ((((((((((((((((((((((((((((((((vec3<f32>(_e2175.g0_.x) * _e2179.g6_) + (vec3<f32>(_e2182.g0_.y) * _e2186.g7_)) + (vec3<f32>(_e2190.g0_.z) * _e2194.g5_)) + ((vec3<f32>(_e2198.g1_.x) * vec3<f32>(_e2202.g3_.w, _e2205.g9_.z, _e2208.g9_.y)) * vec3<f32>(-(1.0), -(1.0), 1.0))) + ((vec3<f32>(_e2221.g1_.y) * vec3<f32>(_e2225.g9_.z, _e2228.g3_.w, _e2231.g9_.x)) * vec3<f32>(1.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e2244.g1_.z) * vec3<f32>(_e2248.g9_.y, _e2251.g9_.x, _e2254.g3_.w)) * vec3<f32>(-(1.0), 1.0, -(1.0)))) + (vec3<f32>(_e2267.g2_.x) * vec3<f32>(_e2271.g3_.x, _e2274.g3_.y, _e2277.g3_.z))) + (vec3<f32>(_e2283.g2_.y) * _e2287.g4_)) + ((vec3<f32>(_e2291.g3_.x) * vec3<f32>(_e2295.g2_.x, _e2298.g8_.z, _e2301.g8_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e2313.g3_.y) * vec3<f32>(_e2317.g8_.z, _e2320.g2_.x, _e2323.g8_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e2335.g3_.z) * vec3<f32>(_e2339.g8_.y, _e2342.g8_.x, _e2345.g2_.x)) * vec3<f32>(1.0, -(1.0), 1.0))) - (vec3<f32>(_e2357.g3_.w) * _e2361.g1_)) + ((vec3<f32>(_e2365.g4_.x) * vec3<f32>(_e2369.g2_.y, _e2372.g7_.z, _e2375.g7_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e2387.g4_.y) * vec3<f32>(_e2391.g7_.z, _e2394.g2_.y, _e2397.g7_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e2409.g4_.z) * vec3<f32>(_e2413.g7_.y, _e2416.g7_.x, _e2419.g2_.y)) * vec3<f32>(1.0, -(1.0), 1.0))) + ((vec3<f32>(_e2431.g5_.x) * vec3<f32>(_e2435.g0_.z, _e2438.g6_.z, _e2441.g6_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e2453.g5_.y) * vec3<f32>(_e2457.g6_.z, _e2460.g0_.z, _e2463.g6_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e2475.g5_.z) * vec3<f32>(_e2479.g6_.y, _e2482.g6_.x, _e2485.g0_.z)) * vec3<f32>(1.0, -(1.0), 1.0))) + ((vec3<f32>(_e2497.g6_.x) * vec3<f32>(_e2501.g0_.x, _e2504.g5_.z, _e2507.g5_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e2519.g6_.y) * vec3<f32>(_e2523.g5_.z, _e2526.g0_.x, _e2529.g5_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e2541.g6_.z) * vec3<f32>(_e2545.g5_.y, _e2548.g5_.x, _e2551.g0_.x)) * vec3<f32>(1.0, -(1.0), 1.0))) + ((vec3<f32>(_e2563.g7_.x) * vec3<f32>(_e2567.g0_.y, _e2570.g4_.z, _e2573.g4_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e2585.g7_.y) * vec3<f32>(_e2589.g4_.z, _e2592.g0_.y, _e2595.g4_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e2607.g7_.z) * vec3<f32>(_e2611.g4_.y, _e2614.g4_.x, _e2617.g0_.y)) * vec3<f32>(1.0, -(1.0), 1.0))) + ((vec3<f32>(_e2629.g8_.x) * vec3<f32>(_e2633.g9_.w, _e2636.g3_.z, _e2639.g3_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e2651.g8_.y) * vec3<f32>(_e2655.g3_.z, _e2658.g9_.w, _e2661.g3_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e2673.g8_.z) * vec3<f32>(_e2677.g3_.y, _e2680.g3_.x, _e2683.g9_.w)) * vec3<f32>(1.0, -(1.0), 1.0))) - (vec3<f32>(_e2695.g8_.w) * vec3<f32>(_e2699.g9_.x, _e2702.g9_.y, _e2705.g9_.z))) + ((vec3<f32>(_e2711.g9_.x) * vec3<f32>(_e2715.g8_.w, _e2718.g1_.z, _e2721.g1_.y)) * vec3<f32>(-(1.0), -(1.0), 1.0))) + ((vec3<f32>(_e2734.g9_.y) * vec3<f32>(_e2738.g1_.z, _e2741.g8_.w, _e2744.g1_.x)) * vec3<f32>(1.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e2757.g9_.z) * vec3<f32>(_e2761.g1_.y, _e2764.g1_.x, _e2767.g8_.w)) * vec3<f32>(-(1.0), 1.0, -(1.0)))) + (vec3<f32>(_e2780.g9_.w) * vec3<f32>(_e2784.g8_.x, _e2787.g8_.y, _e2790.g8_.z))), ((((((((((((((((vec3<f32>(_e2796.g0_.x) * _e2800.g7_) + ((vec3<f32>(_e2803.g1_.x) * vec3<f32>(_e2807.g9_.w, _e2810.g3_.z, _e2813.g3_.y)) * vec3<f32>(-(1.0), -(1.0), 1.0))) + ((vec3<f32>(_e2826.g1_.y) * vec3<f32>(_e2830.g3_.z, _e2833.g9_.w, _e2836.g3_.x)) * vec3<f32>(1.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e2849.g1_.z) * vec3<f32>(_e2853.g3_.y, _e2856.g3_.x, _e2859.g9_.w)) * vec3<f32>(-(1.0), 1.0, -(1.0)))) + (vec3<f32>(_e2872.g2_.y) * _e2876.g5_)) + ((vec3<f32>(_e2880.g3_.x) * vec3<f32>(_e2884.g8_.w, _e2887.g1_.z, _e2890.g1_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e2902.g3_.y) * vec3<f32>(_e2906.g1_.z, _e2909.g8_.w, _e2912.g1_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e2924.g3_.z) * vec3<f32>(_e2928.g1_.y, _e2931.g1_.x, _e2934.g8_.w)) * vec3<f32>(1.0, -(1.0), 1.0))) + ((vec3<f32>(_e2946.g5_.x) * vec3<f32>(_e2950.g2_.y, _e2953.g7_.z, _e2956.g7_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e2968.g5_.y) * vec3<f32>(_e2972.g7_.z, _e2975.g2_.y, _e2978.g7_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e2990.g5_.z) * vec3<f32>(_e2994.g7_.y, _e2997.g7_.x, _e3000.g2_.y)) * vec3<f32>(1.0, -(1.0), 1.0))) + ((vec3<f32>(_e3012.g7_.x) * vec3<f32>(_e3016.g0_.x, _e3019.g5_.z, _e3022.g5_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e3034.g7_.y) * vec3<f32>(_e3038.g5_.z, _e3041.g0_.x, _e3044.g5_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e3056.g7_.z) * vec3<f32>(_e3060.g5_.y, _e3063.g5_.x, _e3066.g0_.x)) * vec3<f32>(1.0, -(1.0), 1.0))) - (vec3<f32>(_e3078.g8_.w) * vec3<f32>(_e3082.g3_.x, _e3085.g3_.y, _e3088.g3_.z))) + (vec3<f32>(_e3094.g9_.w) * _e3098.g1_)), ((((((((((((((((vec4<f32>(_e3102.g0_.x) * _e3106.g8_) + ((vec4<f32>(_e3109.g1_.x) * vec4<f32>(_e3113.g0_.y, _e3116.g4_.z, _e3119.g4_.y, _e3122.g5_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e3136.g1_.y) * vec4<f32>(_e3140.g4_.z, _e3143.g0_.y, _e3146.g4_.x, _e3149.g5_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e3163.g1_.z) * vec4<f32>(_e3167.g4_.y, _e3170.g4_.x, _e3173.g0_.y, _e3176.g5_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e3190.g2_.x) * vec4<f32>(_e3194.g5_.x, _e3197.g5_.y, _e3200.g5_.z, _e3203.g5_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e3215.g4_.x) * vec4<f32>(_e3219.g8_.w, _e3222.g1_.z, _e3225.g1_.y, _e3228.g8_.w)) * vec4<f32>(-(1.0), -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e3242.g4_.y) * vec4<f32>(_e3246.g1_.z, _e3249.g8_.w, _e3252.g1_.x, _e3255.g1_.z)) * vec4<f32>(1.0, -(1.0), -(1.0), 0.0))) + ((vec4<f32>(_e3269.g4_.z) * vec4<f32>(_e3273.g1_.y, _e3276.g1_.x, _e3279.g8_.w, _e3282.g1_.y)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e3296.g5_.x) * vec4<f32>(_e3300.g2_.x, _e3303.g8_.z, _e3306.g8_.y, _e3309.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e3323.g5_.y) * vec4<f32>(_e3327.g8_.z, _e3330.g2_.x, _e3333.g8_.x, _e3336.g1_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e3350.g5_.z) * vec4<f32>(_e3354.g8_.y, _e3357.g8_.x, _e3360.g2_.x, _e3363.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e3377.g8_.x) * vec4<f32>(_e3381.g0_.x, _e3384.g5_.z, _e3387.g5_.y, _e3390.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e3403.g8_.y) * vec4<f32>(_e3407.g5_.z, _e3410.g0_.x, _e3413.g5_.x, _e3416.g5_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0))) + ((vec4<f32>(_e3429.g8_.z) * vec4<f32>(_e3433.g5_.y, _e3436.g5_.x, _e3439.g0_.x, _e3442.g5_.y)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + (vec4<f32>(_e3455.g8_.w) * vec4<f32>(_e3459.g4_.x, _e3462.g4_.y, _e3465.g4_.z, _e3468.g0_.x))) + ((vec4<f32>(_e3474.g0_.y, _e3477.g0_.y, _e3480.g0_.y, _e3483.g0_.x) * vec4<f32>(_e3487.g1_.x, _e3490.g1_.y, _e3493.g1_.z, _e3496.g1_.x)) * vec4<f32>(-(1.0), -(1.0), -(1.0), 0.0))), ((((((((((((((((((((((((((((((((vec4<f32>(_e3511.g0_.x) * _e3515.g9_) + ((vec4<f32>(_e3518.g0_.z) * vec4<f32>(_e3522.g1_.x, _e3525.g1_.y, _e3528.g1_.z, _e3531.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e3543.g1_.x) * vec4<f32>(_e3547.g0_.z, _e3550.g6_.z, _e3553.g6_.y, _e3556.g7_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e3570.g1_.y) * vec4<f32>(_e3574.g6_.z, _e3577.g0_.z, _e3580.g6_.x, _e3583.g7_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e3597.g1_.z) * vec4<f32>(_e3601.g6_.y, _e3604.g6_.x, _e3607.g0_.z, _e3610.g7_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e3624.g2_.x) * vec4<f32>(_e3628.g7_.x, _e3631.g7_.y, _e3634.g7_.z, _e3637.g7_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) - (vec4<f32>(_e3649.g2_.y) * _e3653.g8_)) + ((vec4<f32>(_e3657.g3_.x) * vec4<f32>(_e3661.g0_.y, _e3664.g4_.z, _e3667.g4_.y, _e3670.g5_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e3684.g3_.y) * vec4<f32>(_e3688.g4_.z, _e3691.g0_.y, _e3694.g4_.x, _e3697.g5_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e3711.g3_.z) * vec4<f32>(_e3715.g4_.y, _e3718.g4_.x, _e3721.g0_.y, _e3724.g5_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e3738.g3_.w) * vec4<f32>(_e3742.g5_.x, _e3745.g5_.y, _e3748.g5_.z, _e3751.g5_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e3763.g4_.x) * vec4<f32>(_e3767.g9_.w, _e3770.g3_.z, _e3773.g3_.y, _e3776.g9_.w)) * vec4<f32>(-(1.0), -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e3790.g4_.y) * vec4<f32>(_e3794.g3_.z, _e3797.g9_.w, _e3800.g3_.x, _e3803.g3_.z)) * vec4<f32>(1.0, -(1.0), -(1.0), 0.0))) + ((vec4<f32>(_e3817.g4_.z) * vec4<f32>(_e3821.g3_.y, _e3824.g3_.x, _e3827.g9_.w, _e3830.g3_.y)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e3844.g5_.x) * vec4<f32>(_e3848.g3_.w, _e3851.g9_.z, _e3854.g9_.y, _e3857.g3_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e3871.g5_.y) * vec4<f32>(_e3875.g9_.z, _e3878.g3_.w, _e3881.g9_.x, _e3884.g3_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e3898.g5_.z) * vec4<f32>(_e3902.g9_.y, _e3905.g9_.x, _e3908.g3_.w, _e3911.g3_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e3925.g6_.x) * vec4<f32>(_e3929.g8_.w, _e3932.g1_.z, _e3935.g1_.y, _e3938.g8_.w)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e3951.g6_.y) * vec4<f32>(_e3955.g1_.z, _e3958.g8_.w, _e3961.g1_.x, _e3964.g1_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0))) + ((vec4<f32>(_e3977.g6_.z) * vec4<f32>(_e3981.g1_.y, _e3984.g1_.x, _e3987.g8_.w, _e3990.g1_.y)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e4003.g7_.x) * vec4<f32>(_e4007.g2_.x, _e4010.g8_.z, _e4013.g8_.y, _e4016.g1_.x)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e4030.g7_.y) * vec4<f32>(_e4034.g8_.z, _e4037.g2_.x, _e4040.g8_.x, _e4043.g1_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e4057.g7_.z) * vec4<f32>(_e4061.g8_.y, _e4064.g8_.x, _e4067.g2_.x, _e4070.g1_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e4084.g8_.x) * vec4<f32>(_e4088.g2_.y, _e4091.g7_.z, _e4094.g7_.y, _e4097.g2_.y)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e4110.g8_.y) * vec4<f32>(_e4114.g7_.z, _e4117.g2_.y, _e4120.g7_.x, _e4123.g7_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0))) + ((vec4<f32>(_e4136.g8_.z) * vec4<f32>(_e4140.g7_.y, _e4143.g7_.x, _e4146.g2_.y, _e4149.g7_.y)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + (vec4<f32>(_e4162.g8_.w) * vec4<f32>(_e4166.g6_.x, _e4169.g6_.y, _e4172.g6_.z, _e4175.g2_.y))) + ((vec4<f32>(_e4181.g9_.x) * vec4<f32>(_e4185.g0_.x, _e4188.g5_.z, _e4191.g5_.y, _e4194.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e4207.g9_.y) * vec4<f32>(_e4211.g5_.z, _e4214.g0_.x, _e4217.g5_.x, _e4220.g5_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0))) + ((vec4<f32>(_e4233.g9_.z) * vec4<f32>(_e4237.g5_.y, _e4240.g5_.x, _e4243.g0_.x, _e4246.g5_.y)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + (vec4<f32>(_e4259.g9_.w) * vec4<f32>(_e4263.g4_.x, _e4266.g4_.y, _e4269.g4_.z, _e4272.g0_.x))) + ((vec4<f32>(_e4278.g0_.y, _e4281.g0_.y, _e4284.g0_.y, _e4287.g0_.x) * _e4291.g3_.xyzx) * vec4<f32>(-(1.0), -(1.0), -(1.0), 0.0))));
}

fn multi_vector_multi_vector_outer_product(self_992: MultiVector, other_800: MultiVector) -> MultiVector {
    var self_993: MultiVector;
    var other_801: MultiVector;

    self_993 = self_992;
    other_801 = other_800;
    let _e4: MultiVector = self_993;
    let _e8: MultiVector = other_801;
    let _e11: MultiVector = self_993;
    let _e15: MultiVector = other_801;
    let _e26: MultiVector = self_993;
    let _e30: MultiVector = other_801;
    let _e33: MultiVector = other_801;
    let _e36: MultiVector = other_801;
    let _e47: MultiVector = self_993;
    let _e51: MultiVector = other_801;
    let _e54: MultiVector = other_801;
    let _e57: MultiVector = other_801;
    let _e68: MultiVector = self_993;
    let _e72: MultiVector = other_801;
    let _e75: MultiVector = other_801;
    let _e78: MultiVector = other_801;
    let _e89: MultiVector = self_993;
    let _e93: MultiVector = other_801;
    let _e96: MultiVector = other_801;
    let _e99: MultiVector = other_801;
    let _e110: MultiVector = self_993;
    let _e114: MultiVector = other_801;
    let _e125: MultiVector = self_993;
    let _e129: MultiVector = other_801;
    let _e141: MultiVector = self_993;
    let _e145: MultiVector = other_801;
    let _e157: MultiVector = self_993;
    let _e161: MultiVector = other_801;
    let _e173: MultiVector = self_993;
    let _e177: MultiVector = other_801;
    let _e189: MultiVector = self_993;
    let _e193: MultiVector = other_801;
    let _e196: MultiVector = other_801;
    let _e199: MultiVector = other_801;
    let _e212: MultiVector = self_993;
    let _e216: MultiVector = other_801;
    let _e219: MultiVector = other_801;
    let _e222: MultiVector = other_801;
    let _e235: MultiVector = self_993;
    let _e239: MultiVector = other_801;
    let _e242: MultiVector = other_801;
    let _e245: MultiVector = other_801;
    let _e258: MultiVector = self_993;
    let _e262: MultiVector = other_801;
    let _e265: MultiVector = other_801;
    let _e268: MultiVector = other_801;
    let _e281: MultiVector = self_993;
    let _e285: MultiVector = other_801;
    let _e288: MultiVector = other_801;
    let _e291: MultiVector = other_801;
    let _e304: MultiVector = self_993;
    let _e308: MultiVector = other_801;
    let _e311: MultiVector = other_801;
    let _e314: MultiVector = other_801;
    let _e327: MultiVector = self_993;
    let _e331: MultiVector = other_801;
    let _e343: MultiVector = self_993;
    let _e347: MultiVector = other_801;
    let _e359: MultiVector = self_993;
    let _e363: MultiVector = other_801;
    let _e375: MultiVector = self_993;
    let _e379: MultiVector = other_801;
    let _e391: MultiVector = self_993;
    let _e395: MultiVector = other_801;
    let _e407: MultiVector = self_993;
    let _e411: MultiVector = other_801;
    let _e423: MultiVector = self_993;
    let _e427: MultiVector = other_801;
    let _e430: MultiVector = other_801;
    let _e433: MultiVector = other_801;
    let _e446: MultiVector = self_993;
    let _e450: MultiVector = other_801;
    let _e453: MultiVector = other_801;
    let _e456: MultiVector = other_801;
    let _e469: MultiVector = self_993;
    let _e473: MultiVector = other_801;
    let _e476: MultiVector = other_801;
    let _e479: MultiVector = other_801;
    let _e492: MultiVector = self_993;
    let _e496: MultiVector = other_801;
    let _e499: MultiVector = other_801;
    let _e502: MultiVector = other_801;
    let _e515: MultiVector = self_993;
    let _e519: MultiVector = other_801;
    let _e530: MultiVector = self_993;
    let _e534: MultiVector = other_801;
    let _e545: MultiVector = self_993;
    let _e549: MultiVector = other_801;
    let _e560: MultiVector = self_993;
    let _e564: MultiVector = other_801;
    let _e575: MultiVector = self_993;
    let _e578: MultiVector = other_801;
    let _e581: MultiVector = other_801;
    let _e584: MultiVector = other_801;
    let _e595: MultiVector = self_993;
    let _e599: MultiVector = other_801;
    let _e602: MultiVector = self_993;
    let _e604: MultiVector = other_801;
    let _e610: MultiVector = self_993;
    let _e614: MultiVector = other_801;
    let _e617: MultiVector = self_993;
    let _e619: MultiVector = other_801;
    let _e625: MultiVector = self_993;
    let _e629: MultiVector = other_801;
    let _e632: MultiVector = self_993;
    let _e636: MultiVector = other_801;
    let _e648: MultiVector = self_993;
    let _e652: MultiVector = other_801;
    let _e655: MultiVector = other_801;
    let _e658: MultiVector = other_801;
    let _e661: MultiVector = other_801;
    let _e667: MultiVector = self_993;
    let _e671: MultiVector = other_801;
    let _e683: MultiVector = self_993;
    let _e687: MultiVector = other_801;
    let _e699: MultiVector = self_993;
    let _e703: MultiVector = other_801;
    let _e715: MultiVector = self_993;
    let _e719: MultiVector = other_801;
    let _e731: MultiVector = self_993;
    let _e734: MultiVector = self_993;
    let _e737: MultiVector = self_993;
    let _e740: MultiVector = self_993;
    let _e744: MultiVector = other_801;
    let _e747: MultiVector = other_801;
    let _e750: MultiVector = other_801;
    let _e753: MultiVector = other_801;
    let _e765: MultiVector = self_993;
    let _e769: MultiVector = other_801;
    let _e772: MultiVector = self_993;
    let _e776: MultiVector = other_801;
    let _e780: MultiVector = self_993;
    let _e784: MultiVector = other_801;
    let _e795: MultiVector = self_993;
    let _e799: MultiVector = other_801;
    let _e810: MultiVector = self_993;
    let _e814: MultiVector = other_801;
    let _e825: MultiVector = self_993;
    let _e827: MultiVector = other_801;
    let _e837: MultiVector = self_993;
    let _e841: MultiVector = other_801;
    let _e844: MultiVector = self_993;
    let _e848: MultiVector = other_801;
    let _e859: MultiVector = self_993;
    let _e863: MultiVector = other_801;
    let _e874: MultiVector = self_993;
    let _e878: MultiVector = other_801;
    let _e889: MultiVector = self_993;
    let _e893: MultiVector = other_801;
    let _e904: MultiVector = self_993;
    let _e907: MultiVector = self_993;
    let _e910: MultiVector = self_993;
    let _e914: MultiVector = other_801;
    let _e917: MultiVector = other_801;
    let _e920: MultiVector = other_801;
    let _e932: MultiVector = self_993;
    let _e936: MultiVector = other_801;
    let _e939: MultiVector = self_993;
    let _e943: MultiVector = other_801;
    let _e946: MultiVector = other_801;
    let _e949: MultiVector = other_801;
    let _e955: MultiVector = self_993;
    let _e959: MultiVector = other_801;
    let _e963: MultiVector = self_993;
    let _e967: MultiVector = other_801;
    let _e978: MultiVector = self_993;
    let _e982: MultiVector = other_801;
    let _e993: MultiVector = self_993;
    let _e997: MultiVector = other_801;
    let _e1008: MultiVector = self_993;
    let _e1012: MultiVector = other_801;
    let _e1016: MultiVector = self_993;
    let _e1020: MultiVector = other_801;
    let _e1031: MultiVector = self_993;
    let _e1035: MultiVector = other_801;
    let _e1046: MultiVector = self_993;
    let _e1050: MultiVector = other_801;
    let _e1061: MultiVector = self_993;
    let _e1065: MultiVector = other_801;
    let _e1076: MultiVector = self_993;
    let _e1080: MultiVector = other_801;
    let _e1091: MultiVector = self_993;
    let _e1095: MultiVector = other_801;
    let _e1106: MultiVector = self_993;
    let _e1108: MultiVector = other_801;
    let _e1118: MultiVector = self_993;
    let _e1122: MultiVector = other_801;
    let _e1125: MultiVector = self_993;
    let _e1129: MultiVector = other_801;
    let _e1132: MultiVector = other_801;
    let _e1135: MultiVector = other_801;
    let _e1147: MultiVector = self_993;
    let _e1151: MultiVector = other_801;
    let _e1154: MultiVector = other_801;
    let _e1157: MultiVector = other_801;
    let _e1169: MultiVector = self_993;
    let _e1173: MultiVector = other_801;
    let _e1177: MultiVector = self_993;
    let _e1181: MultiVector = other_801;
    let _e1192: MultiVector = self_993;
    let _e1196: MultiVector = other_801;
    let _e1207: MultiVector = self_993;
    let _e1211: MultiVector = other_801;
    let _e1222: MultiVector = self_993;
    let _e1226: MultiVector = other_801;
    let _e1237: MultiVector = self_993;
    let _e1241: MultiVector = other_801;
    let _e1252: MultiVector = self_993;
    let _e1256: MultiVector = other_801;
    let _e1267: MultiVector = self_993;
    let _e1271: MultiVector = other_801;
    let _e1282: MultiVector = self_993;
    let _e1286: MultiVector = other_801;
    let _e1297: MultiVector = self_993;
    let _e1301: MultiVector = other_801;
    let _e1312: MultiVector = self_993;
    let _e1316: MultiVector = other_801;
    let _e1319: MultiVector = other_801;
    let _e1322: MultiVector = other_801;
    let _e1334: MultiVector = self_993;
    let _e1338: MultiVector = other_801;
    let _e1341: MultiVector = self_993;
    let _e1345: MultiVector = other_801;
    let _e1348: MultiVector = other_801;
    let _e1351: MultiVector = other_801;
    let _e1354: MultiVector = other_801;
    let _e1368: MultiVector = self_993;
    let _e1372: MultiVector = other_801;
    let _e1375: MultiVector = other_801;
    let _e1378: MultiVector = other_801;
    let _e1381: MultiVector = other_801;
    let _e1395: MultiVector = self_993;
    let _e1399: MultiVector = other_801;
    let _e1402: MultiVector = other_801;
    let _e1405: MultiVector = other_801;
    let _e1408: MultiVector = other_801;
    let _e1420: MultiVector = self_993;
    let _e1424: MultiVector = other_801;
    let _e1427: MultiVector = other_801;
    let _e1430: MultiVector = other_801;
    let _e1433: MultiVector = other_801;
    let _e1446: MultiVector = self_993;
    let _e1450: MultiVector = other_801;
    let _e1453: MultiVector = other_801;
    let _e1456: MultiVector = other_801;
    let _e1459: MultiVector = other_801;
    let _e1472: MultiVector = self_993;
    let _e1476: MultiVector = other_801;
    let _e1479: MultiVector = other_801;
    let _e1482: MultiVector = other_801;
    let _e1485: MultiVector = other_801;
    let _e1498: MultiVector = self_993;
    let _e1502: MultiVector = other_801;
    let _e1505: MultiVector = other_801;
    let _e1508: MultiVector = other_801;
    let _e1511: MultiVector = other_801;
    let _e1524: MultiVector = self_993;
    let _e1528: MultiVector = other_801;
    let _e1531: MultiVector = other_801;
    let _e1534: MultiVector = other_801;
    let _e1537: MultiVector = other_801;
    let _e1550: MultiVector = self_993;
    let _e1554: MultiVector = other_801;
    let _e1557: MultiVector = other_801;
    let _e1560: MultiVector = other_801;
    let _e1563: MultiVector = other_801;
    let _e1576: MultiVector = self_993;
    let _e1580: MultiVector = other_801;
    let _e1592: MultiVector = self_993;
    let _e1596: MultiVector = other_801;
    let _e1608: MultiVector = self_993;
    let _e1612: MultiVector = other_801;
    let _e1624: MultiVector = self_993;
    let _e1628: MultiVector = other_801;
    let _e1640: MultiVector = self_993;
    let _e1644: MultiVector = other_801;
    let _e1647: MultiVector = other_801;
    let _e1650: MultiVector = other_801;
    let _e1653: MultiVector = other_801;
    let _e1667: MultiVector = self_993;
    let _e1671: MultiVector = other_801;
    let _e1674: MultiVector = self_993;
    let _e1678: MultiVector = other_801;
    let _e1681: MultiVector = other_801;
    let _e1684: MultiVector = other_801;
    let _e1687: MultiVector = other_801;
    let _e1701: MultiVector = self_993;
    let _e1705: MultiVector = other_801;
    let _e1708: MultiVector = other_801;
    let _e1711: MultiVector = other_801;
    let _e1714: MultiVector = other_801;
    let _e1728: MultiVector = self_993;
    let _e1732: MultiVector = other_801;
    let _e1735: MultiVector = other_801;
    let _e1738: MultiVector = other_801;
    let _e1741: MultiVector = other_801;
    let _e1753: MultiVector = self_993;
    let _e1757: MultiVector = other_801;
    let _e1761: MultiVector = self_993;
    let _e1765: MultiVector = other_801;
    let _e1768: MultiVector = other_801;
    let _e1771: MultiVector = other_801;
    let _e1774: MultiVector = other_801;
    let _e1788: MultiVector = self_993;
    let _e1792: MultiVector = other_801;
    let _e1795: MultiVector = other_801;
    let _e1798: MultiVector = other_801;
    let _e1801: MultiVector = other_801;
    let _e1815: MultiVector = self_993;
    let _e1819: MultiVector = other_801;
    let _e1822: MultiVector = other_801;
    let _e1825: MultiVector = other_801;
    let _e1828: MultiVector = other_801;
    let _e1842: MultiVector = self_993;
    let _e1846: MultiVector = other_801;
    let _e1849: MultiVector = other_801;
    let _e1852: MultiVector = other_801;
    let _e1855: MultiVector = other_801;
    let _e1867: MultiVector = self_993;
    let _e1871: MultiVector = other_801;
    let _e1883: MultiVector = self_993;
    let _e1887: MultiVector = other_801;
    let _e1899: MultiVector = self_993;
    let _e1903: MultiVector = other_801;
    let _e1915: MultiVector = self_993;
    let _e1919: MultiVector = other_801;
    let _e1931: MultiVector = self_993;
    let _e1935: MultiVector = other_801;
    let _e1947: MultiVector = self_993;
    let _e1951: MultiVector = other_801;
    let _e1963: MultiVector = self_993;
    let _e1967: MultiVector = other_801;
    let _e1970: MultiVector = other_801;
    let _e1973: MultiVector = other_801;
    let _e1976: MultiVector = other_801;
    let _e1989: MultiVector = self_993;
    let _e1993: MultiVector = other_801;
    let _e1996: MultiVector = other_801;
    let _e1999: MultiVector = other_801;
    let _e2002: MultiVector = other_801;
    let _e2015: MultiVector = self_993;
    let _e2019: MultiVector = other_801;
    let _e2022: MultiVector = other_801;
    let _e2025: MultiVector = other_801;
    let _e2028: MultiVector = other_801;
    let _e2041: MultiVector = self_993;
    let _e2045: MultiVector = other_801;
    let _e2048: MultiVector = other_801;
    let _e2051: MultiVector = other_801;
    let _e2054: MultiVector = other_801;
    let _e2067: MultiVector = self_993;
    let _e2071: MultiVector = other_801;
    let _e2074: MultiVector = other_801;
    let _e2077: MultiVector = other_801;
    let _e2080: MultiVector = other_801;
    let _e2093: MultiVector = self_993;
    let _e2097: MultiVector = other_801;
    let _e2100: MultiVector = other_801;
    let _e2103: MultiVector = other_801;
    let _e2106: MultiVector = other_801;
    let _e2119: MultiVector = self_993;
    let _e2123: MultiVector = other_801;
    let _e2135: MultiVector = self_993;
    let _e2139: MultiVector = other_801;
    let _e2151: MultiVector = self_993;
    let _e2155: MultiVector = other_801;
    let _e2167: MultiVector = self_993;
    let _e2171: MultiVector = other_801;
    let _e2183: MultiVector = self_993;
    let _e2187: MultiVector = other_801;
    let _e2199: MultiVector = self_993;
    let _e2203: MultiVector = other_801;
    let _e2215: MultiVector = self_993;
    let _e2219: MultiVector = other_801;
    let _e2231: MultiVector = self_993;
    let _e2235: MultiVector = other_801;
    let _e2247: MultiVector = self_993;
    let _e2251: MultiVector = other_801;
    let _e2254: MultiVector = other_801;
    let _e2257: MultiVector = other_801;
    let _e2260: MultiVector = other_801;
    return MultiVector(((((((((((((((((((((((((((((((((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((vec3<f32>(_e11.g0_.z) * vec3<f32>(_e15.g0_.x)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e26.g1_.x) * vec3<f32>(_e30.g8_.x, _e33.g8_.x, _e36.g9_.x)) * vec3<f32>(0.0, 1.0, 1.0))) + ((vec3<f32>(_e47.g1_.y) * vec3<f32>(_e51.g8_.y, _e54.g8_.y, _e57.g9_.y)) * vec3<f32>(0.0, 1.0, 1.0))) + ((vec3<f32>(_e68.g1_.z) * vec3<f32>(_e72.g8_.z, _e75.g8_.z, _e78.g9_.z)) * vec3<f32>(0.0, 1.0, 1.0))) + ((vec3<f32>(_e89.g2_.x) * vec3<f32>(_e93.g8_.w, _e96.g8_.w, _e99.g9_.w)) * vec3<f32>(0.0, 1.0, 1.0))) + ((vec3<f32>(_e110.g2_.y) * vec3<f32>(_e114.g0_.y)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e125.g3_.x) * vec3<f32>(_e129.g8_.x)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e141.g3_.y) * vec3<f32>(_e145.g8_.y)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e157.g3_.z) * vec3<f32>(_e161.g8_.z)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e173.g3_.w) * vec3<f32>(_e177.g8_.w)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e189.g4_.x) * vec3<f32>(_e193.g5_.x, _e196.g5_.x, _e199.g7_.x)) * vec3<f32>(0.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e212.g4_.y) * vec3<f32>(_e216.g5_.y, _e219.g5_.y, _e222.g7_.y)) * vec3<f32>(0.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e235.g4_.z) * vec3<f32>(_e239.g5_.z, _e242.g5_.z, _e245.g7_.z)) * vec3<f32>(0.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e258.g5_.x) * vec3<f32>(_e262.g4_.x, _e265.g4_.x, _e268.g6_.x)) * vec3<f32>(0.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e281.g5_.y) * vec3<f32>(_e285.g4_.y, _e288.g4_.y, _e291.g6_.y)) * vec3<f32>(0.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e304.g5_.z) * vec3<f32>(_e308.g4_.z, _e311.g4_.z, _e314.g6_.z)) * vec3<f32>(0.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e327.g6_.x) * vec3<f32>(_e331.g5_.x)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e343.g6_.y) * vec3<f32>(_e347.g5_.y)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e359.g6_.z) * vec3<f32>(_e363.g5_.z)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e375.g7_.x) * vec3<f32>(_e379.g4_.x)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e391.g7_.y) * vec3<f32>(_e395.g4_.y)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e407.g7_.z) * vec3<f32>(_e411.g4_.z)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e423.g8_.x) * vec3<f32>(_e427.g1_.x, _e430.g1_.x, _e433.g3_.x)) * vec3<f32>(0.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e446.g8_.y) * vec3<f32>(_e450.g1_.y, _e453.g1_.y, _e456.g3_.y)) * vec3<f32>(0.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e469.g8_.z) * vec3<f32>(_e473.g1_.z, _e476.g1_.z, _e479.g3_.z)) * vec3<f32>(0.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e492.g8_.w) * vec3<f32>(_e496.g2_.x, _e499.g2_.x, _e502.g3_.w)) * vec3<f32>(0.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e515.g9_.x) * vec3<f32>(_e519.g1_.x)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e530.g9_.y) * vec3<f32>(_e534.g1_.y)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e545.g9_.z) * vec3<f32>(_e549.g1_.z)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e560.g9_.w) * vec3<f32>(_e564.g2_.x)) * vec3<f32>(0.0, 0.0, 1.0))) + ((_e575.g0_.xyy * vec3<f32>(_e578.g0_.x, _e581.g0_.x, _e584.g2_.y)) * vec3<f32>(0.0, 1.0, 1.0))), ((vec3<f32>(_e595.g0_.x) * _e599.g1_) + (_e602.g1_ * vec3<f32>(_e604.g0_.x))), ((vec2<f32>(_e610.g0_.x) * _e614.g2_) + (_e617.g2_ * vec2<f32>(_e619.g0_.x))), ((((((((vec4<f32>(_e625.g0_.x) * _e629.g3_) + ((vec4<f32>(_e632.g2_.x) * vec4<f32>(_e636.g2_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) - (vec4<f32>(_e648.g2_.y) * vec4<f32>(_e652.g1_.x, _e655.g1_.y, _e658.g1_.z, _e661.g2_.x))) + ((vec4<f32>(_e667.g3_.x) * vec4<f32>(_e671.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e683.g3_.y) * vec4<f32>(_e687.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e699.g3_.z) * vec4<f32>(_e703.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e715.g3_.w) * vec4<f32>(_e719.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e731.g1_.x, _e734.g1_.y, _e737.g1_.z, _e740.g1_.x) * vec4<f32>(_e744.g2_.y, _e747.g2_.y, _e750.g2_.y, _e753.g2_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), ((((((vec3<f32>(_e765.g0_.x) * _e769.g4_) + (vec3<f32>(_e772.g2_.x) * _e776.g1_)) + ((vec3<f32>(_e780.g4_.x) * vec3<f32>(_e784.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e795.g4_.y) * vec3<f32>(_e799.g0_.x)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e810.g4_.z) * vec3<f32>(_e814.g0_.x)) * vec3<f32>(0.0, 0.0, 1.0))) + ((_e825.g1_ * vec3<f32>(_e827.g2_.x)) * vec3<f32>(-(1.0)))), ((((((vec3<f32>(_e837.g0_.x) * _e841.g5_) + ((vec3<f32>(_e844.g1_.y) * _e848.g1_.zzx) * vec3<f32>(1.0, 0.0, -(1.0)))) + ((vec3<f32>(_e859.g1_.z) * _e863.g1_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e874.g5_.y) * vec3<f32>(_e878.g0_.x)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e889.g5_.z) * vec3<f32>(_e893.g0_.x)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e904.g5_.x, _e907.g1_.x, _e910.g1_.x) * vec3<f32>(_e914.g0_.x, _e917.g1_.z, _e920.g1_.y)) * vec3<f32>(1.0, -(1.0), 1.0))), ((((((((((((((vec3<f32>(_e932.g0_.x) * _e936.g6_) + (vec3<f32>(_e939.g2_.x) * vec3<f32>(_e943.g3_.x, _e946.g3_.y, _e949.g3_.z))) + (vec3<f32>(_e955.g2_.y) * _e959.g4_)) + ((vec3<f32>(_e963.g3_.x) * vec3<f32>(_e967.g2_.x)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e978.g3_.y) * vec3<f32>(_e982.g2_.x)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e993.g3_.z) * vec3<f32>(_e997.g2_.x)) * vec3<f32>(0.0, 0.0, 1.0))) - (vec3<f32>(_e1008.g3_.w) * _e1012.g1_)) + ((vec3<f32>(_e1016.g4_.x) * vec3<f32>(_e1020.g2_.y)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e1031.g4_.y) * vec3<f32>(_e1035.g2_.y)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e1046.g4_.z) * vec3<f32>(_e1050.g2_.y)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e1061.g6_.x) * vec3<f32>(_e1065.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e1076.g6_.y) * vec3<f32>(_e1080.g0_.x)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e1091.g6_.z) * vec3<f32>(_e1095.g0_.x)) * vec3<f32>(0.0, 0.0, 1.0))) + ((_e1106.g1_ * vec3<f32>(_e1108.g3_.w)) * vec3<f32>(-(1.0)))), ((((((((((((((vec3<f32>(_e1118.g0_.x) * _e1122.g7_) + ((vec3<f32>(_e1125.g1_.y) * vec3<f32>(_e1129.g3_.z, _e1132.g3_.z, _e1135.g3_.x)) * vec3<f32>(1.0, 0.0, -(1.0)))) + ((vec3<f32>(_e1147.g1_.z) * vec3<f32>(_e1151.g3_.y, _e1154.g3_.x, _e1157.g3_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + (vec3<f32>(_e1169.g2_.y) * _e1173.g5_)) + ((vec3<f32>(_e1177.g3_.x) * _e1181.g1_.zzy) * vec3<f32>(0.0, 1.0, -(1.0)))) + ((vec3<f32>(_e1192.g3_.y) * _e1196.g1_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e1207.g3_.z) * _e1211.g1_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e1222.g5_.x) * vec3<f32>(_e1226.g2_.y)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e1237.g5_.y) * vec3<f32>(_e1241.g2_.y)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e1252.g5_.z) * vec3<f32>(_e1256.g2_.y)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e1267.g7_.x) * vec3<f32>(_e1271.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e1282.g7_.y) * vec3<f32>(_e1286.g0_.x)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e1297.g7_.z) * vec3<f32>(_e1301.g0_.x)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e1312.g1_.x) * vec3<f32>(_e1316.g3_.x, _e1319.g3_.z, _e1322.g3_.y)) * vec3<f32>(0.0, -(1.0), 1.0))), (((((((((((((((vec4<f32>(_e1334.g0_.x) * _e1338.g8_) + ((vec4<f32>(_e1341.g1_.y) * vec4<f32>(_e1345.g4_.z, _e1348.g4_.z, _e1351.g4_.x, _e1354.g5_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e1368.g1_.z) * vec4<f32>(_e1372.g4_.y, _e1375.g4_.x, _e1378.g4_.y, _e1381.g5_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e1395.g2_.x) * vec4<f32>(_e1399.g5_.x, _e1402.g5_.y, _e1405.g5_.z, _e1408.g5_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e1420.g4_.x) * vec4<f32>(_e1424.g1_.z, _e1427.g1_.z, _e1430.g1_.y, _e1433.g1_.z)) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e1446.g4_.y) * vec4<f32>(_e1450.g1_.z, _e1453.g1_.z, _e1456.g1_.x, _e1459.g1_.z)) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e1472.g4_.z) * vec4<f32>(_e1476.g1_.y, _e1479.g1_.x, _e1482.g1_.y, _e1485.g1_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e1498.g5_.x) * vec4<f32>(_e1502.g2_.x, _e1505.g2_.x, _e1508.g2_.x, _e1511.g1_.x)) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e1524.g5_.y) * vec4<f32>(_e1528.g2_.x, _e1531.g2_.x, _e1534.g2_.x, _e1537.g1_.y)) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e1550.g5_.z) * vec4<f32>(_e1554.g2_.x, _e1557.g2_.x, _e1560.g2_.x, _e1563.g1_.z)) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e1576.g8_.x) * vec4<f32>(_e1580.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e1592.g8_.y) * vec4<f32>(_e1596.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e1608.g8_.z) * vec4<f32>(_e1612.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e1624.g8_.w) * vec4<f32>(_e1628.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e1640.g1_.x) * vec4<f32>(_e1644.g4_.x, _e1647.g4_.z, _e1650.g4_.y, _e1653.g5_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), ((((((((((((((((((((((((((((((vec4<f32>(_e1667.g0_.x) * _e1671.g9_) + ((vec4<f32>(_e1674.g1_.y) * vec4<f32>(_e1678.g6_.z, _e1681.g6_.z, _e1684.g6_.x, _e1687.g7_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e1701.g1_.z) * vec4<f32>(_e1705.g6_.y, _e1708.g6_.x, _e1711.g6_.y, _e1714.g7_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e1728.g2_.x) * vec4<f32>(_e1732.g7_.x, _e1735.g7_.y, _e1738.g7_.z, _e1741.g7_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) - (vec4<f32>(_e1753.g2_.y) * _e1757.g8_)) + ((vec4<f32>(_e1761.g3_.x) * vec4<f32>(_e1765.g4_.z, _e1768.g4_.z, _e1771.g4_.y, _e1774.g5_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e1788.g3_.y) * vec4<f32>(_e1792.g4_.z, _e1795.g4_.z, _e1798.g4_.x, _e1801.g5_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e1815.g3_.z) * vec4<f32>(_e1819.g4_.y, _e1822.g4_.x, _e1825.g4_.y, _e1828.g5_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e1842.g3_.w) * vec4<f32>(_e1846.g5_.x, _e1849.g5_.y, _e1852.g5_.z, _e1855.g5_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e1867.g4_.x) * _e1871.g3_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e1883.g4_.y) * _e1887.g3_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e1899.g4_.z) * _e1903.g3_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e1915.g5_.x) * _e1919.g3_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e1931.g5_.y) * _e1935.g3_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e1947.g5_.z) * _e1951.g3_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e1963.g6_.x) * vec4<f32>(_e1967.g1_.z, _e1970.g1_.z, _e1973.g1_.y, _e1976.g1_.z)) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e1989.g6_.y) * vec4<f32>(_e1993.g1_.z, _e1996.g1_.z, _e1999.g1_.x, _e2002.g1_.z)) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e2015.g6_.z) * vec4<f32>(_e2019.g1_.y, _e2022.g1_.x, _e2025.g1_.y, _e2028.g1_.y)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e2041.g7_.x) * vec4<f32>(_e2045.g2_.x, _e2048.g2_.x, _e2051.g2_.x, _e2054.g1_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e2067.g7_.y) * vec4<f32>(_e2071.g2_.x, _e2074.g2_.x, _e2077.g2_.x, _e2080.g1_.y)) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e2093.g7_.z) * vec4<f32>(_e2097.g2_.x, _e2100.g2_.x, _e2103.g2_.x, _e2106.g1_.z)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e2119.g8_.x) * vec4<f32>(_e2123.g2_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e2135.g8_.y) * vec4<f32>(_e2139.g2_.y)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e2151.g8_.z) * vec4<f32>(_e2155.g2_.y)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e2167.g8_.w) * vec4<f32>(_e2171.g2_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e2183.g9_.x) * vec4<f32>(_e2187.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e2199.g9_.y) * vec4<f32>(_e2203.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e2215.g9_.z) * vec4<f32>(_e2219.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e2231.g9_.w) * vec4<f32>(_e2235.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e2247.g1_.x) * vec4<f32>(_e2251.g6_.x, _e2254.g6_.z, _e2257.g6_.y, _e2260.g7_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn multi_vector_multi_vector_inner_product(self_994: MultiVector, other_802: MultiVector) -> MultiVector {
    var self_995: MultiVector;
    var other_803: MultiVector;

    self_995 = self_994;
    other_803 = other_802;
    let _e4: MultiVector = self_995;
    let _e8: MultiVector = other_803;
    let _e11: MultiVector = self_995;
    let _e15: MultiVector = other_803;
    let _e26: MultiVector = self_995;
    let _e30: MultiVector = other_803;
    let _e41: MultiVector = self_995;
    let _e45: MultiVector = other_803;
    let _e57: MultiVector = self_995;
    let _e61: MultiVector = other_803;
    let _e73: MultiVector = self_995;
    let _e77: MultiVector = other_803;
    let _e89: MultiVector = self_995;
    let _e93: MultiVector = other_803;
    let _e105: MultiVector = self_995;
    let _e108: MultiVector = self_995;
    let _e111: MultiVector = self_995;
    let _e115: MultiVector = other_803;
    let _e118: MultiVector = other_803;
    let _e121: MultiVector = other_803;
    let _e127: MultiVector = self_995;
    let _e131: MultiVector = other_803;
    let _e134: MultiVector = self_995;
    let _e138: MultiVector = other_803;
    let _e141: MultiVector = other_803;
    let _e144: MultiVector = other_803;
    let _e156: MultiVector = self_995;
    let _e160: MultiVector = other_803;
    let _e163: MultiVector = other_803;
    let _e166: MultiVector = other_803;
    let _e178: MultiVector = self_995;
    let _e182: MultiVector = other_803;
    let _e185: MultiVector = other_803;
    let _e188: MultiVector = other_803;
    let _e200: MultiVector = self_995;
    let _e204: MultiVector = other_803;
    let _e207: MultiVector = other_803;
    let _e210: MultiVector = other_803;
    let _e222: MultiVector = self_995;
    let _e226: MultiVector = other_803;
    let _e229: MultiVector = other_803;
    let _e232: MultiVector = other_803;
    let _e244: MultiVector = self_995;
    let _e248: MultiVector = other_803;
    let _e251: MultiVector = other_803;
    let _e254: MultiVector = other_803;
    let _e266: MultiVector = self_995;
    let _e270: MultiVector = other_803;
    let _e274: MultiVector = self_995;
    let _e278: MultiVector = other_803;
    let _e281: MultiVector = self_995;
    let _e285: MultiVector = other_803;
    let _e288: MultiVector = other_803;
    let _e299: MultiVector = self_995;
    let _e303: MultiVector = other_803;
    let _e306: MultiVector = other_803;
    let _e317: MultiVector = self_995;
    let _e321: MultiVector = other_803;
    let _e324: MultiVector = other_803;
    let _e335: MultiVector = self_995;
    let _e339: MultiVector = other_803;
    let _e349: MultiVector = self_995;
    let _e353: MultiVector = other_803;
    let _e363: MultiVector = self_995;
    let _e367: MultiVector = other_803;
    let _e378: MultiVector = self_995;
    let _e382: MultiVector = other_803;
    let _e393: MultiVector = self_995;
    let _e397: MultiVector = other_803;
    let _e408: MultiVector = self_995;
    let _e412: MultiVector = other_803;
    let _e422: MultiVector = self_995;
    let _e426: MultiVector = other_803;
    let _e436: MultiVector = self_995;
    let _e440: MultiVector = other_803;
    let _e450: MultiVector = self_995;
    let _e454: MultiVector = other_803;
    let _e457: MultiVector = other_803;
    let _e463: MultiVector = self_995;
    let _e467: MultiVector = other_803;
    let _e470: MultiVector = other_803;
    let _e476: MultiVector = self_995;
    let _e480: MultiVector = other_803;
    let _e483: MultiVector = other_803;
    let _e489: MultiVector = self_995;
    let _e493: MultiVector = other_803;
    let _e504: MultiVector = self_995;
    let _e508: MultiVector = other_803;
    let _e519: MultiVector = self_995;
    let _e523: MultiVector = other_803;
    let _e534: MultiVector = self_995;
    let _e538: MultiVector = other_803;
    let _e549: MultiVector = self_995;
    let _e553: MultiVector = other_803;
    let _e564: MultiVector = self_995;
    let _e568: MultiVector = other_803;
    let _e579: MultiVector = self_995;
    let _e583: MultiVector = other_803;
    let _e586: MultiVector = other_803;
    let _e597: MultiVector = self_995;
    let _e601: MultiVector = other_803;
    let _e611: MultiVector = self_995;
    let _e614: MultiVector = self_995;
    let _e618: MultiVector = other_803;
    let _e621: MultiVector = other_803;
    let _e632: MultiVector = self_995;
    let _e636: MultiVector = other_803;
    let _e639: MultiVector = self_995;
    let _e643: MultiVector = other_803;
    let _e646: MultiVector = other_803;
    let _e649: MultiVector = other_803;
    let _e652: MultiVector = other_803;
    let _e666: MultiVector = self_995;
    let _e670: MultiVector = other_803;
    let _e673: MultiVector = other_803;
    let _e676: MultiVector = other_803;
    let _e679: MultiVector = other_803;
    let _e693: MultiVector = self_995;
    let _e697: MultiVector = other_803;
    let _e700: MultiVector = other_803;
    let _e703: MultiVector = other_803;
    let _e706: MultiVector = other_803;
    let _e720: MultiVector = self_995;
    let _e724: MultiVector = other_803;
    let _e736: MultiVector = self_995;
    let _e740: MultiVector = other_803;
    let _e752: MultiVector = self_995;
    let _e756: MultiVector = other_803;
    let _e768: MultiVector = self_995;
    let _e772: MultiVector = other_803;
    let _e784: MultiVector = self_995;
    let _e788: MultiVector = other_803;
    let _e800: MultiVector = self_995;
    let _e804: MultiVector = other_803;
    let _e816: MultiVector = self_995;
    let _e820: MultiVector = other_803;
    let _e832: MultiVector = self_995;
    let _e836: MultiVector = other_803;
    let _e849: MultiVector = self_995;
    let _e853: MultiVector = other_803;
    let _e866: MultiVector = self_995;
    let _e870: MultiVector = other_803;
    let _e883: MultiVector = self_995;
    let _e887: MultiVector = other_803;
    let _e890: MultiVector = other_803;
    let _e893: MultiVector = other_803;
    let _e896: MultiVector = other_803;
    let _e909: MultiVector = self_995;
    let _e913: MultiVector = other_803;
    let _e916: MultiVector = other_803;
    let _e919: MultiVector = other_803;
    let _e922: MultiVector = other_803;
    let _e935: MultiVector = self_995;
    let _e939: MultiVector = other_803;
    let _e942: MultiVector = other_803;
    let _e945: MultiVector = other_803;
    let _e948: MultiVector = other_803;
    let _e961: MultiVector = self_995;
    let _e965: MultiVector = other_803;
    let _e977: MultiVector = self_995;
    let _e981: MultiVector = other_803;
    let _e994: MultiVector = self_995;
    let _e998: MultiVector = other_803;
    let _e1011: MultiVector = self_995;
    let _e1015: MultiVector = other_803;
    let _e1028: MultiVector = self_995;
    let _e1032: MultiVector = other_803;
    let _e1035: MultiVector = other_803;
    let _e1038: MultiVector = other_803;
    let _e1041: MultiVector = other_803;
    let _e1053: MultiVector = self_995;
    let _e1056: MultiVector = self_995;
    let _e1059: MultiVector = self_995;
    let _e1062: MultiVector = self_995;
    let _e1066: MultiVector = other_803;
    let _e1077: MultiVector = self_995;
    let _e1081: MultiVector = other_803;
    let _e1084: MultiVector = self_995;
    let _e1088: MultiVector = other_803;
    let _e1092: MultiVector = self_995;
    let _e1096: MultiVector = other_803;
    let _e1099: MultiVector = other_803;
    let _e1102: MultiVector = other_803;
    let _e1114: MultiVector = self_995;
    let _e1118: MultiVector = other_803;
    let _e1121: MultiVector = other_803;
    let _e1124: MultiVector = other_803;
    let _e1136: MultiVector = self_995;
    let _e1140: MultiVector = other_803;
    let _e1151: MultiVector = self_995;
    let _e1155: MultiVector = other_803;
    let _e1166: MultiVector = self_995;
    let _e1170: MultiVector = other_803;
    let _e1181: MultiVector = self_995;
    let _e1185: MultiVector = other_803;
    let _e1196: MultiVector = self_995;
    let _e1200: MultiVector = other_803;
    let _e1211: MultiVector = self_995;
    let _e1215: MultiVector = other_803;
    let _e1226: MultiVector = self_995;
    let _e1230: MultiVector = other_803;
    let _e1241: MultiVector = self_995;
    let _e1245: MultiVector = other_803;
    let _e1256: MultiVector = self_995;
    let _e1260: MultiVector = other_803;
    let _e1271: MultiVector = self_995;
    let _e1275: MultiVector = other_803;
    let _e1278: MultiVector = other_803;
    let _e1281: MultiVector = other_803;
    let _e1293: MultiVector = self_995;
    let _e1297: MultiVector = other_803;
    let _e1300: MultiVector = self_995;
    let _e1304: MultiVector = other_803;
    let _e1315: MultiVector = self_995;
    let _e1319: MultiVector = other_803;
    let _e1330: MultiVector = self_995;
    let _e1334: MultiVector = other_803;
    let _e1345: MultiVector = self_995;
    let _e1349: MultiVector = other_803;
    let _e1353: MultiVector = self_995;
    let _e1355: MultiVector = other_803;
    let _e1365: MultiVector = self_995;
    let _e1369: MultiVector = other_803;
    let _e1372: MultiVector = self_995;
    let _e1376: MultiVector = other_803;
    let _e1380: MultiVector = self_995;
    let _e1384: MultiVector = other_803;
    let _e1387: MultiVector = other_803;
    let _e1390: MultiVector = other_803;
    let _e1402: MultiVector = self_995;
    let _e1406: MultiVector = other_803;
    let _e1409: MultiVector = other_803;
    let _e1412: MultiVector = other_803;
    let _e1424: MultiVector = self_995;
    let _e1428: MultiVector = other_803;
    let _e1439: MultiVector = self_995;
    let _e1443: MultiVector = other_803;
    let _e1454: MultiVector = self_995;
    let _e1458: MultiVector = other_803;
    let _e1469: MultiVector = self_995;
    let _e1473: MultiVector = other_803;
    let _e1484: MultiVector = self_995;
    let _e1488: MultiVector = other_803;
    let _e1499: MultiVector = self_995;
    let _e1503: MultiVector = other_803;
    let _e1514: MultiVector = self_995;
    let _e1518: MultiVector = other_803;
    let _e1529: MultiVector = self_995;
    let _e1533: MultiVector = other_803;
    let _e1544: MultiVector = self_995;
    let _e1548: MultiVector = other_803;
    let _e1559: MultiVector = self_995;
    let _e1563: MultiVector = other_803;
    let _e1566: MultiVector = other_803;
    let _e1569: MultiVector = other_803;
    let _e1581: MultiVector = self_995;
    let _e1585: MultiVector = other_803;
    let _e1588: MultiVector = self_995;
    let _e1592: MultiVector = other_803;
    let _e1603: MultiVector = self_995;
    let _e1607: MultiVector = other_803;
    let _e1618: MultiVector = self_995;
    let _e1622: MultiVector = other_803;
    let _e1633: MultiVector = self_995;
    let _e1637: MultiVector = other_803;
    let _e1641: MultiVector = self_995;
    let _e1643: MultiVector = other_803;
    let _e1653: MultiVector = self_995;
    let _e1657: MultiVector = other_803;
    let _e1660: MultiVector = self_995;
    let _e1664: MultiVector = other_803;
    let _e1676: MultiVector = self_995;
    let _e1680: MultiVector = other_803;
    let _e1692: MultiVector = self_995;
    let _e1696: MultiVector = other_803;
    let _e1708: MultiVector = self_995;
    let _e1712: MultiVector = other_803;
    let _e1724: MultiVector = self_995;
    let _e1728: MultiVector = other_803;
    let _e1740: MultiVector = self_995;
    let _e1744: MultiVector = other_803;
    let _e1756: MultiVector = self_995;
    let _e1760: MultiVector = other_803;
    let _e1772: MultiVector = self_995;
    let _e1775: MultiVector = self_995;
    let _e1778: MultiVector = self_995;
    let _e1781: MultiVector = self_995;
    let _e1785: MultiVector = other_803;
    let _e1788: MultiVector = other_803;
    let _e1791: MultiVector = other_803;
    let _e1794: MultiVector = other_803;
    let _e1809: MultiVector = self_995;
    let _e1813: MultiVector = other_803;
    let _e1816: MultiVector = self_995;
    let _e1820: MultiVector = other_803;
    let _e1832: MultiVector = self_995;
    let _e1836: MultiVector = other_803;
    let _e1848: MultiVector = self_995;
    let _e1852: MultiVector = other_803;
    let _e1864: MultiVector = self_995;
    let _e1868: MultiVector = other_803;
    let _e1880: MultiVector = self_995;
    let _e1884: MultiVector = other_803;
    let _e1896: MultiVector = self_995;
    let _e1900: MultiVector = other_803;
    let _e1912: MultiVector = self_995;
    let _e1916: MultiVector = other_803;
    let _e1928: MultiVector = self_995;
    let _e1931: MultiVector = self_995;
    let _e1934: MultiVector = self_995;
    let _e1937: MultiVector = self_995;
    let _e1941: MultiVector = other_803;
    let _e1944: MultiVector = other_803;
    let _e1947: MultiVector = other_803;
    let _e1950: MultiVector = other_803;
    return MultiVector(((((((((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((vec3<f32>(_e11.g1_.y) * vec3<f32>(_e15.g1_.y)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e26.g1_.z) * vec3<f32>(_e30.g1_.z)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e41.g5_.x) * vec3<f32>(_e45.g5_.x)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e57.g5_.y) * vec3<f32>(_e61.g5_.y)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e73.g5_.z) * vec3<f32>(_e77.g5_.z)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e89.g8_.w) * vec3<f32>(_e93.g8_.w)) * vec3<f32>(-(1.0), 0.0, 0.0))) + (vec3<f32>(_e105.g1_.x, _e108.g0_.y, _e111.g0_.z) * vec3<f32>(_e115.g1_.x, _e118.g0_.x, _e121.g0_.x))), ((((((((vec3<f32>(_e127.g0_.x) * _e131.g1_) + ((vec3<f32>(_e134.g1_.x) * vec3<f32>(_e138.g0_.x, _e141.g5_.z, _e144.g5_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e156.g1_.y) * vec3<f32>(_e160.g5_.z, _e163.g0_.x, _e166.g5_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e178.g1_.z) * vec3<f32>(_e182.g5_.y, _e185.g5_.x, _e188.g0_.x)) * vec3<f32>(1.0, -(1.0), 1.0))) + ((vec3<f32>(_e200.g5_.x) * vec3<f32>(_e204.g8_.w, _e207.g1_.z, _e210.g1_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e222.g5_.y) * vec3<f32>(_e226.g1_.z, _e229.g8_.w, _e232.g1_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e244.g5_.z) * vec3<f32>(_e248.g1_.y, _e251.g1_.x, _e254.g8_.w)) * vec3<f32>(1.0, -(1.0), 1.0))) + (vec3<f32>(_e266.g8_.w) * _e270.g5_)), ((((((((((((((((((((((((vec2<f32>(_e274.g0_.x) * _e278.g2_) + ((vec2<f32>(_e281.g1_.x) * vec2<f32>(_e285.g4_.x, _e288.g3_.x)) * vec2<f32>(-(1.0), 1.0))) + ((vec2<f32>(_e299.g1_.y) * vec2<f32>(_e303.g4_.y, _e306.g3_.y)) * vec2<f32>(-(1.0), 1.0))) + ((vec2<f32>(_e317.g1_.z) * vec2<f32>(_e321.g4_.z, _e324.g3_.z)) * vec2<f32>(-(1.0), 1.0))) + ((vec2<f32>(_e335.g2_.x) * vec2<f32>(_e339.g0_.x)) * vec2<f32>(1.0, 0.0))) + ((vec2<f32>(_e349.g2_.y) * vec2<f32>(_e353.g0_.x)) * vec2<f32>(0.0, 1.0))) + ((vec2<f32>(_e363.g3_.x) * vec2<f32>(_e367.g1_.x)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e378.g3_.y) * vec2<f32>(_e382.g1_.y)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e393.g3_.z) * vec2<f32>(_e397.g1_.z)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e408.g4_.x) * vec2<f32>(_e412.g1_.x)) * vec2<f32>(1.0, 0.0))) + ((vec2<f32>(_e422.g4_.y) * vec2<f32>(_e426.g1_.y)) * vec2<f32>(1.0, 0.0))) + ((vec2<f32>(_e436.g4_.z) * vec2<f32>(_e440.g1_.z)) * vec2<f32>(1.0, 0.0))) - (vec2<f32>(_e450.g5_.x) * vec2<f32>(_e454.g8_.x, _e457.g7_.x))) - (vec2<f32>(_e463.g5_.y) * vec2<f32>(_e467.g8_.y, _e470.g7_.y))) - (vec2<f32>(_e476.g5_.z) * vec2<f32>(_e480.g8_.z, _e483.g7_.z))) + ((vec2<f32>(_e489.g7_.x) * vec2<f32>(_e493.g5_.x)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e504.g7_.y) * vec2<f32>(_e508.g5_.y)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e519.g7_.z) * vec2<f32>(_e523.g5_.z)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e534.g8_.x) * vec2<f32>(_e538.g5_.x)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e549.g8_.y) * vec2<f32>(_e553.g5_.y)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e564.g8_.z) * vec2<f32>(_e568.g5_.z)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e579.g8_.w) * vec2<f32>(_e583.g0_.y, _e586.g9_.w)) * vec2<f32>(1.0, -(1.0)))) + ((vec2<f32>(_e597.g9_.w) * vec2<f32>(_e601.g8_.w)) * vec2<f32>(0.0, 1.0))) + ((vec2<f32>(_e611.g0_.y, _e614.g0_.x) * vec2<f32>(_e618.g8_.w, _e621.g8_.x)) * vec2<f32>(-(1.0), 0.0))), (((((((((((((((((((((((vec4<f32>(_e632.g0_.x) * _e636.g3_) + ((vec4<f32>(_e639.g1_.x) * vec4<f32>(_e643.g7_.z, _e646.g7_.z, _e649.g7_.y, _e652.g6_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e666.g1_.y) * vec4<f32>(_e670.g7_.z, _e673.g7_.z, _e676.g7_.x, _e679.g6_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e693.g1_.z) * vec4<f32>(_e697.g7_.y, _e700.g7_.x, _e703.g7_.y, _e706.g6_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e720.g3_.x) * vec4<f32>(_e724.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e736.g3_.y) * vec4<f32>(_e740.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e752.g3_.z) * vec4<f32>(_e756.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e768.g3_.w) * vec4<f32>(_e772.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e784.g5_.x) * _e788.g9_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e800.g5_.y) * _e804.g9_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e816.g5_.z) * _e820.g9_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e832.g6_.x) * vec4<f32>(_e836.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e849.g6_.y) * vec4<f32>(_e853.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e866.g6_.z) * vec4<f32>(_e870.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e883.g7_.x) * vec4<f32>(_e887.g1_.z, _e890.g1_.z, _e893.g1_.y, _e896.g1_.z)) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e909.g7_.y) * vec4<f32>(_e913.g1_.z, _e916.g1_.z, _e919.g1_.x, _e922.g1_.z)) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e935.g7_.z) * vec4<f32>(_e939.g1_.y, _e942.g1_.x, _e945.g1_.y, _e948.g1_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e961.g8_.w) * vec4<f32>(_e965.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e977.g9_.x) * vec4<f32>(_e981.g5_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e994.g9_.y) * vec4<f32>(_e998.g5_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e1011.g9_.z) * vec4<f32>(_e1015.g5_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e1028.g9_.w) * vec4<f32>(_e1032.g5_.x, _e1035.g5_.y, _e1038.g5_.z, _e1041.g5_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e1053.g0_.x, _e1056.g0_.x, _e1059.g0_.x, _e1062.g0_.z) * _e1066.g8_.xxxw) * vec4<f32>(0.0, 0.0, 0.0, 1.0))), ((((((((((((((vec3<f32>(_e1077.g0_.x) * _e1081.g4_) + (vec3<f32>(_e1084.g0_.y) * _e1088.g5_)) + ((vec3<f32>(_e1092.g1_.y) * vec3<f32>(_e1096.g8_.z, _e1099.g8_.z, _e1102.g8_.x)) * vec3<f32>(1.0, 0.0, -(1.0)))) + ((vec3<f32>(_e1114.g1_.z) * vec3<f32>(_e1118.g8_.y, _e1121.g8_.x, _e1124.g8_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e1136.g4_.x) * vec3<f32>(_e1140.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e1151.g4_.y) * vec3<f32>(_e1155.g0_.x)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e1166.g4_.z) * vec3<f32>(_e1170.g0_.x)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e1181.g5_.x) * vec3<f32>(_e1185.g0_.y)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e1196.g5_.y) * vec3<f32>(_e1200.g0_.y)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e1211.g5_.z) * vec3<f32>(_e1215.g0_.y)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e1226.g8_.x) * _e1230.g1_.zzy) * vec3<f32>(0.0, 1.0, -(1.0)))) + ((vec3<f32>(_e1241.g8_.y) * _e1245.g1_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e1256.g8_.z) * _e1260.g1_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e1271.g1_.x) * vec3<f32>(_e1275.g8_.x, _e1278.g8_.z, _e1281.g8_.y)) * vec3<f32>(0.0, -(1.0), 1.0))), ((((((vec3<f32>(_e1293.g0_.x) * _e1297.g5_) + ((vec3<f32>(_e1300.g5_.x) * vec3<f32>(_e1304.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e1315.g5_.y) * vec3<f32>(_e1319.g0_.x)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e1330.g5_.z) * vec3<f32>(_e1334.g0_.x)) * vec3<f32>(0.0, 0.0, 1.0))) - (vec3<f32>(_e1345.g8_.w) * _e1349.g1_)) + ((_e1353.g1_ * vec3<f32>(_e1355.g8_.w)) * vec3<f32>(-(1.0)))), ((((((((((((((vec3<f32>(_e1365.g0_.x) * _e1369.g6_) + (vec3<f32>(_e1372.g0_.z) * _e1376.g5_)) + ((vec3<f32>(_e1380.g1_.y) * vec3<f32>(_e1384.g9_.z, _e1387.g9_.z, _e1390.g9_.x)) * vec3<f32>(1.0, 0.0, -(1.0)))) + ((vec3<f32>(_e1402.g1_.z) * vec3<f32>(_e1406.g9_.y, _e1409.g9_.x, _e1412.g9_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e1424.g5_.x) * vec3<f32>(_e1428.g0_.z)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e1439.g5_.y) * vec3<f32>(_e1443.g0_.z)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e1454.g5_.z) * vec3<f32>(_e1458.g0_.z)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e1469.g6_.x) * vec3<f32>(_e1473.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e1484.g6_.y) * vec3<f32>(_e1488.g0_.x)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e1499.g6_.z) * vec3<f32>(_e1503.g0_.x)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e1514.g9_.x) * _e1518.g1_.zzy) * vec3<f32>(0.0, -(1.0), 1.0))) + ((vec3<f32>(_e1529.g9_.y) * _e1533.g1_.zzx) * vec3<f32>(1.0, 0.0, -(1.0)))) + ((vec3<f32>(_e1544.g9_.z) * _e1548.g1_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e1559.g1_.x) * vec3<f32>(_e1563.g9_.x, _e1566.g9_.z, _e1569.g9_.y)) * vec3<f32>(0.0, -(1.0), 1.0))), ((((((vec3<f32>(_e1581.g0_.x) * _e1585.g7_) + ((vec3<f32>(_e1588.g7_.x) * vec3<f32>(_e1592.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e1603.g7_.y) * vec3<f32>(_e1607.g0_.x)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e1618.g7_.z) * vec3<f32>(_e1622.g0_.x)) * vec3<f32>(0.0, 0.0, 1.0))) + (vec3<f32>(_e1633.g9_.w) * _e1637.g1_)) + ((_e1641.g1_ * vec3<f32>(_e1643.g9_.w)) * vec3<f32>(-(1.0)))), (((((((((vec4<f32>(_e1653.g0_.x) * _e1657.g8_) + ((vec4<f32>(_e1660.g1_.x) * vec4<f32>(_e1664.g0_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e1676.g1_.y) * vec4<f32>(_e1680.g0_.y)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e1692.g1_.z) * vec4<f32>(_e1696.g0_.y)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e1708.g8_.x) * vec4<f32>(_e1712.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e1724.g8_.y) * vec4<f32>(_e1728.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e1740.g8_.z) * vec4<f32>(_e1744.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e1756.g8_.w) * vec4<f32>(_e1760.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e1772.g0_.y, _e1775.g0_.y, _e1778.g0_.y, _e1781.g0_.x) * vec4<f32>(_e1785.g1_.x, _e1788.g1_.y, _e1791.g1_.z, _e1794.g1_.x)) * vec4<f32>(-(1.0), -(1.0), -(1.0), 0.0))), (((((((((vec4<f32>(_e1809.g0_.x) * _e1813.g9_) + ((vec4<f32>(_e1816.g1_.x) * vec4<f32>(_e1820.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e1832.g1_.y) * vec4<f32>(_e1836.g0_.z)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e1848.g1_.z) * vec4<f32>(_e1852.g0_.z)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e1864.g9_.x) * vec4<f32>(_e1868.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e1880.g9_.y) * vec4<f32>(_e1884.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e1896.g9_.z) * vec4<f32>(_e1900.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e1912.g9_.w) * vec4<f32>(_e1916.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e1928.g0_.z, _e1931.g0_.z, _e1934.g0_.z, _e1937.g0_.x) * vec4<f32>(_e1941.g1_.x, _e1944.g1_.y, _e1947.g1_.z, _e1950.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn multi_vector_multi_vector_left_contraction(self_996: MultiVector, other_804: MultiVector) -> MultiVector {
    var self_997: MultiVector;
    var other_805: MultiVector;

    self_997 = self_996;
    other_805 = other_804;
    let _e4: MultiVector = self_997;
    let _e8: MultiVector = other_805;
    let _e11: MultiVector = self_997;
    let _e15: MultiVector = other_805;
    let _e26: MultiVector = self_997;
    let _e30: MultiVector = other_805;
    let _e41: MultiVector = self_997;
    let _e45: MultiVector = other_805;
    let _e57: MultiVector = self_997;
    let _e61: MultiVector = other_805;
    let _e73: MultiVector = self_997;
    let _e77: MultiVector = other_805;
    let _e89: MultiVector = self_997;
    let _e93: MultiVector = other_805;
    let _e105: MultiVector = self_997;
    let _e109: MultiVector = other_805;
    let _e120: MultiVector = self_997;
    let _e124: MultiVector = other_805;
    let _e127: MultiVector = self_997;
    let _e131: MultiVector = other_805;
    let _e142: MultiVector = self_997;
    let _e146: MultiVector = other_805;
    let _e157: MultiVector = self_997;
    let _e161: MultiVector = other_805;
    let _e172: MultiVector = self_997;
    let _e176: MultiVector = other_805;
    let _e187: MultiVector = self_997;
    let _e191: MultiVector = other_805;
    let _e202: MultiVector = self_997;
    let _e206: MultiVector = other_805;
    let _e217: MultiVector = self_997;
    let _e221: MultiVector = other_805;
    let _e224: MultiVector = self_997;
    let _e228: MultiVector = other_805;
    let _e231: MultiVector = other_805;
    let _e242: MultiVector = self_997;
    let _e246: MultiVector = other_805;
    let _e249: MultiVector = other_805;
    let _e260: MultiVector = self_997;
    let _e264: MultiVector = other_805;
    let _e267: MultiVector = other_805;
    let _e278: MultiVector = self_997;
    let _e282: MultiVector = other_805;
    let _e285: MultiVector = other_805;
    let _e291: MultiVector = self_997;
    let _e295: MultiVector = other_805;
    let _e298: MultiVector = other_805;
    let _e304: MultiVector = self_997;
    let _e308: MultiVector = other_805;
    let _e311: MultiVector = other_805;
    let _e317: MultiVector = self_997;
    let _e321: MultiVector = other_805;
    let _e324: MultiVector = other_805;
    let _e335: MultiVector = self_997;
    let _e339: MultiVector = other_805;
    let _e342: MultiVector = self_997;
    let _e346: MultiVector = other_805;
    let _e349: MultiVector = other_805;
    let _e352: MultiVector = other_805;
    let _e355: MultiVector = other_805;
    let _e369: MultiVector = self_997;
    let _e373: MultiVector = other_805;
    let _e376: MultiVector = other_805;
    let _e379: MultiVector = other_805;
    let _e382: MultiVector = other_805;
    let _e396: MultiVector = self_997;
    let _e400: MultiVector = other_805;
    let _e412: MultiVector = self_997;
    let _e416: MultiVector = other_805;
    let _e428: MultiVector = self_997;
    let _e432: MultiVector = other_805;
    let _e444: MultiVector = self_997;
    let _e448: MultiVector = other_805;
    let _e460: MultiVector = self_997;
    let _e464: MultiVector = other_805;
    let _e467: MultiVector = other_805;
    let _e470: MultiVector = other_805;
    let _e473: MultiVector = other_805;
    let _e487: MultiVector = self_997;
    let _e491: MultiVector = other_805;
    let _e494: MultiVector = self_997;
    let _e498: MultiVector = other_805;
    let _e501: MultiVector = other_805;
    let _e504: MultiVector = other_805;
    let _e516: MultiVector = self_997;
    let _e520: MultiVector = other_805;
    let _e523: MultiVector = other_805;
    let _e526: MultiVector = other_805;
    let _e538: MultiVector = self_997;
    let _e542: MultiVector = other_805;
    let _e553: MultiVector = self_997;
    let _e557: MultiVector = other_805;
    let _e568: MultiVector = self_997;
    let _e572: MultiVector = other_805;
    let _e583: MultiVector = self_997;
    let _e587: MultiVector = other_805;
    let _e590: MultiVector = other_805;
    let _e593: MultiVector = other_805;
    let _e605: MultiVector = self_997;
    let _e609: MultiVector = other_805;
    let _e612: MultiVector = self_997;
    let _e614: MultiVector = other_805;
    let _e624: MultiVector = self_997;
    let _e628: MultiVector = other_805;
    let _e631: MultiVector = self_997;
    let _e635: MultiVector = other_805;
    let _e638: MultiVector = other_805;
    let _e641: MultiVector = other_805;
    let _e653: MultiVector = self_997;
    let _e657: MultiVector = other_805;
    let _e660: MultiVector = other_805;
    let _e663: MultiVector = other_805;
    let _e675: MultiVector = self_997;
    let _e679: MultiVector = other_805;
    let _e690: MultiVector = self_997;
    let _e694: MultiVector = other_805;
    let _e705: MultiVector = self_997;
    let _e709: MultiVector = other_805;
    let _e720: MultiVector = self_997;
    let _e724: MultiVector = other_805;
    let _e727: MultiVector = other_805;
    let _e730: MultiVector = other_805;
    let _e742: MultiVector = self_997;
    let _e746: MultiVector = other_805;
    let _e749: MultiVector = self_997;
    let _e751: MultiVector = other_805;
    let _e761: MultiVector = self_997;
    let _e765: MultiVector = other_805;
    let _e768: MultiVector = self_997;
    let _e771: MultiVector = self_997;
    let _e774: MultiVector = self_997;
    let _e777: MultiVector = self_997;
    let _e781: MultiVector = other_805;
    let _e784: MultiVector = other_805;
    let _e787: MultiVector = other_805;
    let _e790: MultiVector = other_805;
    let _e802: MultiVector = self_997;
    let _e806: MultiVector = other_805;
    let _e809: MultiVector = self_997;
    let _e812: MultiVector = self_997;
    let _e815: MultiVector = self_997;
    let _e818: MultiVector = self_997;
    let _e822: MultiVector = other_805;
    let _e825: MultiVector = other_805;
    let _e828: MultiVector = other_805;
    let _e831: MultiVector = other_805;
    return MultiVector(((((((((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((vec3<f32>(_e11.g1_.y) * vec3<f32>(_e15.g1_.y)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e26.g1_.z) * vec3<f32>(_e30.g1_.z)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e41.g5_.x) * vec3<f32>(_e45.g5_.x)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e57.g5_.y) * vec3<f32>(_e61.g5_.y)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e73.g5_.z) * vec3<f32>(_e77.g5_.z)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e89.g8_.w) * vec3<f32>(_e93.g8_.w)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e105.g1_.x) * vec3<f32>(_e109.g1_.x)) * vec3<f32>(1.0, 0.0, 0.0))), (((((((vec3<f32>(_e120.g0_.x) * _e124.g1_) + ((vec3<f32>(_e127.g1_.y) * _e131.g5_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e142.g1_.z) * _e146.g5_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e157.g5_.x) * vec3<f32>(_e161.g8_.w)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e172.g5_.y) * vec3<f32>(_e176.g8_.w)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e187.g5_.z) * vec3<f32>(_e191.g8_.w)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e202.g1_.x) * _e206.g5_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))), ((((((((vec2<f32>(_e217.g0_.x) * _e221.g2_) + ((vec2<f32>(_e224.g1_.x) * vec2<f32>(_e228.g4_.x, _e231.g3_.x)) * vec2<f32>(-(1.0), 1.0))) + ((vec2<f32>(_e242.g1_.y) * vec2<f32>(_e246.g4_.y, _e249.g3_.y)) * vec2<f32>(-(1.0), 1.0))) + ((vec2<f32>(_e260.g1_.z) * vec2<f32>(_e264.g4_.z, _e267.g3_.z)) * vec2<f32>(-(1.0), 1.0))) - (vec2<f32>(_e278.g5_.x) * vec2<f32>(_e282.g8_.x, _e285.g7_.x))) - (vec2<f32>(_e291.g5_.y) * vec2<f32>(_e295.g8_.y, _e298.g7_.y))) - (vec2<f32>(_e304.g5_.z) * vec2<f32>(_e308.g8_.z, _e311.g7_.z))) + ((vec2<f32>(_e317.g8_.w) * vec2<f32>(_e321.g0_.y, _e324.g9_.w)) * vec2<f32>(1.0, -(1.0)))), ((((((((vec4<f32>(_e335.g0_.x) * _e339.g3_) + ((vec4<f32>(_e342.g1_.y) * vec4<f32>(_e346.g7_.z, _e349.g7_.z, _e352.g7_.x, _e355.g6_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e369.g1_.z) * vec4<f32>(_e373.g7_.y, _e376.g7_.x, _e379.g7_.y, _e382.g6_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e396.g5_.x) * _e400.g9_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e412.g5_.y) * _e416.g9_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e428.g5_.z) * _e432.g9_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e444.g8_.w) * vec4<f32>(_e448.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e460.g1_.x) * vec4<f32>(_e464.g7_.x, _e467.g7_.z, _e470.g7_.y, _e473.g6_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), (((((((vec3<f32>(_e487.g0_.x) * _e491.g4_) + ((vec3<f32>(_e494.g1_.y) * vec3<f32>(_e498.g8_.z, _e501.g8_.z, _e504.g8_.x)) * vec3<f32>(1.0, 0.0, -(1.0)))) + ((vec3<f32>(_e516.g1_.z) * vec3<f32>(_e520.g8_.y, _e523.g8_.x, _e526.g8_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e538.g5_.x) * vec3<f32>(_e542.g0_.y)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e553.g5_.y) * vec3<f32>(_e557.g0_.y)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e568.g5_.z) * vec3<f32>(_e572.g0_.y)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e583.g1_.x) * vec3<f32>(_e587.g8_.x, _e590.g8_.z, _e593.g8_.y)) * vec3<f32>(0.0, -(1.0), 1.0))), ((vec3<f32>(_e605.g0_.x) * _e609.g5_) + ((_e612.g1_ * vec3<f32>(_e614.g8_.w)) * vec3<f32>(-(1.0)))), (((((((vec3<f32>(_e624.g0_.x) * _e628.g6_) + ((vec3<f32>(_e631.g1_.y) * vec3<f32>(_e635.g9_.z, _e638.g9_.z, _e641.g9_.x)) * vec3<f32>(1.0, 0.0, -(1.0)))) + ((vec3<f32>(_e653.g1_.z) * vec3<f32>(_e657.g9_.y, _e660.g9_.x, _e663.g9_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e675.g5_.x) * vec3<f32>(_e679.g0_.z)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e690.g5_.y) * vec3<f32>(_e694.g0_.z)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e705.g5_.z) * vec3<f32>(_e709.g0_.z)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e720.g1_.x) * vec3<f32>(_e724.g9_.x, _e727.g9_.z, _e730.g9_.y)) * vec3<f32>(0.0, -(1.0), 1.0))), ((vec3<f32>(_e742.g0_.x) * _e746.g7_) + ((_e749.g1_ * vec3<f32>(_e751.g9_.w)) * vec3<f32>(-(1.0)))), ((vec4<f32>(_e761.g0_.x) * _e765.g8_) + ((vec4<f32>(_e768.g1_.x, _e771.g1_.y, _e774.g1_.z, _e777.g1_.x) * vec4<f32>(_e781.g0_.y, _e784.g0_.y, _e787.g0_.y, _e790.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), ((vec4<f32>(_e802.g0_.x) * _e806.g9_) + ((vec4<f32>(_e809.g1_.x, _e812.g1_.y, _e815.g1_.z, _e818.g1_.x) * vec4<f32>(_e822.g0_.z, _e825.g0_.z, _e828.g0_.z, _e831.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn multi_vector_multi_vector_right_contraction(self_998: MultiVector, other_806: MultiVector) -> MultiVector {
    var self_999: MultiVector;
    var other_807: MultiVector;

    self_999 = self_998;
    other_807 = other_806;
    let _e4: MultiVector = self_999;
    let _e8: MultiVector = other_807;
    let _e18: MultiVector = self_999;
    let _e22: MultiVector = other_807;
    let _e33: MultiVector = self_999;
    let _e37: MultiVector = other_807;
    let _e48: MultiVector = self_999;
    let _e52: MultiVector = other_807;
    let _e64: MultiVector = self_999;
    let _e68: MultiVector = other_807;
    let _e80: MultiVector = self_999;
    let _e84: MultiVector = other_807;
    let _e96: MultiVector = self_999;
    let _e100: MultiVector = other_807;
    let _e112: MultiVector = self_999;
    let _e114: MultiVector = other_807;
    let _e120: MultiVector = self_999;
    let _e124: MultiVector = other_807;
    let _e134: MultiVector = self_999;
    let _e138: MultiVector = other_807;
    let _e149: MultiVector = self_999;
    let _e153: MultiVector = other_807;
    let _e164: MultiVector = self_999;
    let _e168: MultiVector = other_807;
    let _e172: MultiVector = self_999;
    let _e174: MultiVector = other_807;
    let _e180: MultiVector = self_999;
    let _e184: MultiVector = other_807;
    let _e193: MultiVector = self_999;
    let _e197: MultiVector = other_807;
    let _e207: MultiVector = self_999;
    let _e211: MultiVector = other_807;
    let _e222: MultiVector = self_999;
    let _e226: MultiVector = other_807;
    let _e237: MultiVector = self_999;
    let _e241: MultiVector = other_807;
    let _e252: MultiVector = self_999;
    let _e256: MultiVector = other_807;
    let _e266: MultiVector = self_999;
    let _e270: MultiVector = other_807;
    let _e280: MultiVector = self_999;
    let _e284: MultiVector = other_807;
    let _e294: MultiVector = self_999;
    let _e298: MultiVector = other_807;
    let _e309: MultiVector = self_999;
    let _e313: MultiVector = other_807;
    let _e324: MultiVector = self_999;
    let _e328: MultiVector = other_807;
    let _e339: MultiVector = self_999;
    let _e343: MultiVector = other_807;
    let _e354: MultiVector = self_999;
    let _e358: MultiVector = other_807;
    let _e369: MultiVector = self_999;
    let _e373: MultiVector = other_807;
    let _e384: MultiVector = self_999;
    let _e388: MultiVector = other_807;
    let _e398: MultiVector = self_999;
    let _e401: MultiVector = self_999;
    let _e405: MultiVector = other_807;
    let _e408: MultiVector = other_807;
    let _e419: MultiVector = self_999;
    let _e423: MultiVector = other_807;
    let _e434: MultiVector = self_999;
    let _e438: MultiVector = other_807;
    let _e450: MultiVector = self_999;
    let _e454: MultiVector = other_807;
    let _e466: MultiVector = self_999;
    let _e470: MultiVector = other_807;
    let _e482: MultiVector = self_999;
    let _e486: MultiVector = other_807;
    let _e499: MultiVector = self_999;
    let _e503: MultiVector = other_807;
    let _e516: MultiVector = self_999;
    let _e520: MultiVector = other_807;
    let _e533: MultiVector = self_999;
    let _e537: MultiVector = other_807;
    let _e540: MultiVector = other_807;
    let _e543: MultiVector = other_807;
    let _e546: MultiVector = other_807;
    let _e559: MultiVector = self_999;
    let _e563: MultiVector = other_807;
    let _e566: MultiVector = other_807;
    let _e569: MultiVector = other_807;
    let _e572: MultiVector = other_807;
    let _e585: MultiVector = self_999;
    let _e589: MultiVector = other_807;
    let _e592: MultiVector = other_807;
    let _e595: MultiVector = other_807;
    let _e598: MultiVector = other_807;
    let _e611: MultiVector = self_999;
    let _e615: MultiVector = other_807;
    let _e628: MultiVector = self_999;
    let _e632: MultiVector = other_807;
    let _e645: MultiVector = self_999;
    let _e649: MultiVector = other_807;
    let _e662: MultiVector = self_999;
    let _e666: MultiVector = other_807;
    let _e669: MultiVector = other_807;
    let _e672: MultiVector = other_807;
    let _e675: MultiVector = other_807;
    let _e687: MultiVector = self_999;
    let _e690: MultiVector = self_999;
    let _e693: MultiVector = self_999;
    let _e696: MultiVector = self_999;
    let _e700: MultiVector = other_807;
    let _e711: MultiVector = self_999;
    let _e715: MultiVector = other_807;
    let _e718: MultiVector = self_999;
    let _e722: MultiVector = other_807;
    let _e733: MultiVector = self_999;
    let _e737: MultiVector = other_807;
    let _e748: MultiVector = self_999;
    let _e752: MultiVector = other_807;
    let _e763: MultiVector = self_999;
    let _e765: MultiVector = other_807;
    let _e773: MultiVector = self_999;
    let _e777: MultiVector = other_807;
    let _e781: MultiVector = self_999;
    let _e783: MultiVector = other_807;
    let _e789: MultiVector = self_999;
    let _e793: MultiVector = other_807;
    let _e796: MultiVector = self_999;
    let _e800: MultiVector = other_807;
    let _e811: MultiVector = self_999;
    let _e815: MultiVector = other_807;
    let _e826: MultiVector = self_999;
    let _e830: MultiVector = other_807;
    let _e841: MultiVector = self_999;
    let _e843: MultiVector = other_807;
    let _e849: MultiVector = self_999;
    let _e853: MultiVector = other_807;
    let _e856: MultiVector = self_999;
    let _e858: MultiVector = other_807;
    let _e864: MultiVector = self_999;
    let _e868: MultiVector = other_807;
    let _e879: MultiVector = self_999;
    let _e883: MultiVector = other_807;
    let _e895: MultiVector = self_999;
    let _e899: MultiVector = other_807;
    let _e911: MultiVector = self_999;
    let _e915: MultiVector = other_807;
    let _e927: MultiVector = self_999;
    let _e930: MultiVector = self_999;
    let _e933: MultiVector = self_999;
    let _e936: MultiVector = self_999;
    let _e940: MultiVector = other_807;
    let _e943: MultiVector = other_807;
    let _e946: MultiVector = other_807;
    let _e949: MultiVector = other_807;
    let _e964: MultiVector = self_999;
    let _e968: MultiVector = other_807;
    let _e979: MultiVector = self_999;
    let _e983: MultiVector = other_807;
    let _e995: MultiVector = self_999;
    let _e999: MultiVector = other_807;
    let _e1011: MultiVector = self_999;
    let _e1015: MultiVector = other_807;
    let _e1027: MultiVector = self_999;
    let _e1030: MultiVector = self_999;
    let _e1033: MultiVector = self_999;
    let _e1036: MultiVector = self_999;
    let _e1040: MultiVector = other_807;
    let _e1043: MultiVector = other_807;
    let _e1046: MultiVector = other_807;
    let _e1049: MultiVector = other_807;
    return MultiVector((((((((((vec3<f32>(_e4.g1_.x) * vec3<f32>(_e8.g1_.x)) * vec3<f32>(1.0, 0.0, 0.0)) + ((vec3<f32>(_e18.g1_.y) * vec3<f32>(_e22.g1_.y)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e33.g1_.z) * vec3<f32>(_e37.g1_.z)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e48.g5_.x) * vec3<f32>(_e52.g5_.x)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e64.g5_.y) * vec3<f32>(_e68.g5_.y)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e80.g5_.z) * vec3<f32>(_e84.g5_.z)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e96.g8_.w) * vec3<f32>(_e100.g8_.w)) * vec3<f32>(-(1.0), 0.0, 0.0))) + (_e112.g0_ * vec3<f32>(_e114.g0_.x))), ((((((vec3<f32>(_e120.g5_.x) * _e124.g1_.zzy) * vec3<f32>(0.0, 1.0, -(1.0))) + ((vec3<f32>(_e134.g5_.y) * _e138.g1_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e149.g5_.z) * _e153.g1_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + (vec3<f32>(_e164.g8_.w) * _e168.g5_)) + (_e172.g1_ * vec3<f32>(_e174.g0_.x))), (((((((((((((((((vec2<f32>(_e180.g2_.x) * vec2<f32>(_e184.g0_.x)) * vec2<f32>(1.0, 0.0)) + ((vec2<f32>(_e193.g2_.y) * vec2<f32>(_e197.g0_.x)) * vec2<f32>(0.0, 1.0))) + ((vec2<f32>(_e207.g3_.x) * vec2<f32>(_e211.g1_.x)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e222.g3_.y) * vec2<f32>(_e226.g1_.y)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e237.g3_.z) * vec2<f32>(_e241.g1_.z)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e252.g4_.x) * vec2<f32>(_e256.g1_.x)) * vec2<f32>(1.0, 0.0))) + ((vec2<f32>(_e266.g4_.y) * vec2<f32>(_e270.g1_.y)) * vec2<f32>(1.0, 0.0))) + ((vec2<f32>(_e280.g4_.z) * vec2<f32>(_e284.g1_.z)) * vec2<f32>(1.0, 0.0))) + ((vec2<f32>(_e294.g7_.x) * vec2<f32>(_e298.g5_.x)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e309.g7_.y) * vec2<f32>(_e313.g5_.y)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e324.g7_.z) * vec2<f32>(_e328.g5_.z)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e339.g8_.x) * vec2<f32>(_e343.g5_.x)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e354.g8_.y) * vec2<f32>(_e358.g5_.y)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e369.g8_.z) * vec2<f32>(_e373.g5_.z)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e384.g9_.w) * vec2<f32>(_e388.g8_.w)) * vec2<f32>(0.0, 1.0))) + ((vec2<f32>(_e398.g0_.y, _e401.g0_.x) * vec2<f32>(_e405.g8_.w, _e408.g8_.x)) * vec2<f32>(-(1.0), 0.0))), ((((((((((((((((vec4<f32>(_e419.g3_.x) * vec4<f32>(_e423.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e434.g3_.y) * vec4<f32>(_e438.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e450.g3_.z) * vec4<f32>(_e454.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e466.g3_.w) * vec4<f32>(_e470.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e482.g6_.x) * vec4<f32>(_e486.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e499.g6_.y) * vec4<f32>(_e503.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e516.g6_.z) * vec4<f32>(_e520.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e533.g7_.x) * vec4<f32>(_e537.g1_.z, _e540.g1_.z, _e543.g1_.y, _e546.g1_.z)) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e559.g7_.y) * vec4<f32>(_e563.g1_.z, _e566.g1_.z, _e569.g1_.x, _e572.g1_.z)) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e585.g7_.z) * vec4<f32>(_e589.g1_.y, _e592.g1_.x, _e595.g1_.y, _e598.g1_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e611.g9_.x) * vec4<f32>(_e615.g5_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e628.g9_.y) * vec4<f32>(_e632.g5_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e645.g9_.z) * vec4<f32>(_e649.g5_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e662.g9_.w) * vec4<f32>(_e666.g5_.x, _e669.g5_.y, _e672.g5_.z, _e675.g5_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e687.g0_.x, _e690.g0_.x, _e693.g0_.x, _e696.g0_.z) * _e700.g8_.xxxw) * vec4<f32>(0.0, 0.0, 0.0, 1.0))), (((((vec3<f32>(_e711.g0_.y) * _e715.g5_) + ((vec3<f32>(_e718.g8_.x) * _e722.g1_.zzy) * vec3<f32>(0.0, 1.0, -(1.0)))) + ((vec3<f32>(_e733.g8_.y) * _e737.g1_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e748.g8_.z) * _e752.g1_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + (_e763.g4_ * vec3<f32>(_e765.g0_.x))), ((vec3<f32>(0.0) - (vec3<f32>(_e773.g8_.w) * _e777.g1_)) + (_e781.g5_ * vec3<f32>(_e783.g0_.x))), (((((vec3<f32>(_e789.g0_.z) * _e793.g5_) + ((vec3<f32>(_e796.g9_.x) * _e800.g1_.zzy) * vec3<f32>(0.0, -(1.0), 1.0))) + ((vec3<f32>(_e811.g9_.y) * _e815.g1_.zzx) * vec3<f32>(1.0, 0.0, -(1.0)))) + ((vec3<f32>(_e826.g9_.z) * _e830.g1_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + (_e841.g6_ * vec3<f32>(_e843.g0_.x))), ((vec3<f32>(_e849.g9_.w) * _e853.g1_) + (_e856.g7_ * vec3<f32>(_e858.g0_.x))), ((((((vec4<f32>(_e864.g8_.x) * vec4<f32>(_e868.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e879.g8_.y) * vec4<f32>(_e883.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e895.g8_.z) * vec4<f32>(_e899.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e911.g8_.w) * vec4<f32>(_e915.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e927.g0_.y, _e930.g0_.y, _e933.g0_.y, _e936.g0_.x) * vec4<f32>(_e940.g1_.x, _e943.g1_.y, _e946.g1_.z, _e949.g1_.x)) * vec4<f32>(-(1.0), -(1.0), -(1.0), 0.0))), ((((((vec4<f32>(_e964.g9_.x) * vec4<f32>(_e968.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e979.g9_.y) * vec4<f32>(_e983.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e995.g9_.z) * vec4<f32>(_e999.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e1011.g9_.w) * vec4<f32>(_e1015.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e1027.g0_.z, _e1030.g0_.z, _e1033.g0_.z, _e1036.g0_.x) * vec4<f32>(_e1040.g1_.x, _e1043.g1_.y, _e1046.g1_.z, _e1049.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn multi_vector_multi_vector_scalar_product(self_1000: MultiVector, other_808: MultiVector) -> Scalar {
    var self_1001: MultiVector;
    var other_809: MultiVector;

    self_1001 = self_1000;
    other_809 = other_808;
    let _e4: MultiVector = self_1001;
    let _e7: MultiVector = other_809;
    let _e11: MultiVector = self_1001;
    let _e14: MultiVector = other_809;
    let _e19: MultiVector = self_1001;
    let _e22: MultiVector = other_809;
    let _e27: MultiVector = self_1001;
    let _e30: MultiVector = other_809;
    let _e35: MultiVector = self_1001;
    let _e38: MultiVector = other_809;
    let _e43: MultiVector = self_1001;
    let _e46: MultiVector = other_809;
    let _e51: MultiVector = self_1001;
    let _e54: MultiVector = other_809;
    let _e59: MultiVector = self_1001;
    let _e62: MultiVector = other_809;
    return Scalar(((((((((_e4.g0_.x * _e7.g0_.x) + (_e11.g1_.x * _e14.g1_.x)) + (_e19.g1_.y * _e22.g1_.y)) + (_e27.g1_.z * _e30.g1_.z)) - (_e35.g5_.x * _e38.g5_.x)) - (_e43.g5_.y * _e46.g5_.y)) - (_e51.g5_.z * _e54.g5_.z)) - (_e59.g8_.w * _e62.g8_.w)));
}

fn multi_vector_squared_magnitude(self_1002: MultiVector) -> Scalar {
    var self_1003: MultiVector;

    self_1003 = self_1002;
    let _e4: MultiVector = self_1003;
    let _e5: MultiVector = multi_vector_reversal(_e4);
    let _e6: MultiVector = self_1003;
    let _e8: MultiVector = self_1003;
    let _e9: MultiVector = multi_vector_reversal(_e8);
    let _e10: Scalar = multi_vector_multi_vector_scalar_product(_e6, _e9);
    return _e10;
}

fn multi_vector_magnitude(self_1004: MultiVector) -> Scalar {
    var self_1005: MultiVector;

    self_1005 = self_1004;
    let _e3: MultiVector = self_1005;
    let _e4: Scalar = multi_vector_squared_magnitude(_e3);
    let _e7: MultiVector = self_1005;
    let _e8: Scalar = multi_vector_squared_magnitude(_e7);
    return Scalar(sqrt(_e8.g0_));
}

fn multi_vector_scale(self_1006: MultiVector, other_810: f32) -> MultiVector {
    var self_1007: MultiVector;
    var other_811: f32;

    self_1007 = self_1006;
    other_811 = other_810;
    let _e5: f32 = other_811;
    let _e7: MultiVector = self_1007;
    let _e8: f32 = other_811;
    let _e10: MultiVector = multi_vector_scalar_geometric_product(_e7, Scalar(_e8));
    return _e10;
}

fn multi_vector_signum(self_1008: MultiVector) -> MultiVector {
    var self_1009: MultiVector;

    self_1009 = self_1008;
    let _e5: MultiVector = self_1009;
    let _e6: Scalar = multi_vector_magnitude(_e5);
    let _e10: MultiVector = self_1009;
    let _e13: MultiVector = self_1009;
    let _e14: Scalar = multi_vector_magnitude(_e13);
    let _e18: MultiVector = multi_vector_scalar_geometric_product(_e10, Scalar((1.0 / _e14.g0_)));
    return _e18;
}

fn multi_vector_inverse(self_1010: MultiVector) -> MultiVector {
    var self_1011: MultiVector;

    self_1011 = self_1010;
    let _e3: MultiVector = self_1011;
    let _e4: MultiVector = multi_vector_reversal(_e3);
    let _e7: MultiVector = self_1011;
    let _e8: Scalar = multi_vector_squared_magnitude(_e7);
    let _e13: MultiVector = self_1011;
    let _e14: MultiVector = multi_vector_reversal(_e13);
    let _e17: MultiVector = self_1011;
    let _e18: Scalar = multi_vector_squared_magnitude(_e17);
    let _e22: MultiVector = multi_vector_scalar_geometric_product(_e14, Scalar((1.0 / _e18.g0_)));
    return _e22;
}

fn anti_scalar_scalar_geometric_quotient(self_1012: AntiScalar, other_812: Scalar) -> AntiScalar {
    var self_1013: AntiScalar;
    var other_813: Scalar;

    self_1013 = self_1012;
    other_813 = other_812;
    let _e6: Scalar = other_813;
    let _e7: Scalar = scalar_inverse(_e6);
    let _e8: AntiScalar = self_1013;
    let _e10: Scalar = other_813;
    let _e11: Scalar = scalar_inverse(_e10);
    let _e12: AntiScalar = anti_scalar_scalar_geometric_product(_e8, _e11);
    return _e12;
}

fn circle_flector_transformation(self_1014: Circle, other_814: Flector) -> Flector {
    var self_1015: Circle;
    var other_815: Flector;

    self_1015 = self_1014;
    other_815 = other_814;
    let _e6: Circle = self_1015;
    let _e7: Flector = other_815;
    let _e8: Motor = circle_flector_geometric_product(_e6, _e7);
    let _e10: Circle = self_1015;
    let _e11: Circle = circle_reversal(_e10);
    let _e14: Circle = self_1015;
    let _e15: Flector = other_815;
    let _e16: Motor = circle_flector_geometric_product(_e14, _e15);
    let _e18: Circle = self_1015;
    let _e19: Circle = circle_reversal(_e18);
    let _e20: Flector = motor_circle_geometric_product(_e16, _e19);
    return _e20;
}

fn circle_motor_transformation(self_1016: Circle, other_816: Motor) -> Motor {
    var self_1017: Circle;
    var other_817: Motor;

    self_1017 = self_1016;
    other_817 = other_816;
    let _e6: Circle = self_1017;
    let _e7: Motor = other_817;
    let _e8: Flector = circle_motor_geometric_product(_e6, _e7);
    let _e10: Circle = self_1017;
    let _e11: Circle = circle_reversal(_e10);
    let _e14: Circle = self_1017;
    let _e15: Motor = other_817;
    let _e16: Flector = circle_motor_geometric_product(_e14, _e15);
    let _e18: Circle = self_1017;
    let _e19: Circle = circle_reversal(_e18);
    let _e20: Motor = flector_circle_geometric_product(_e16, _e19);
    return _e20;
}

fn circle_multi_vector_geometric_quotient(self_1018: Circle, other_818: MultiVector) -> MultiVector {
    var self_1019: Circle;
    var other_819: MultiVector;

    self_1019 = self_1018;
    other_819 = other_818;
    let _e6: MultiVector = other_819;
    let _e7: MultiVector = multi_vector_inverse(_e6);
    let _e8: Circle = self_1019;
    let _e10: MultiVector = other_819;
    let _e11: MultiVector = multi_vector_inverse(_e10);
    let _e12: MultiVector = circle_multi_vector_geometric_product(_e8, _e11);
    return _e12;
}

fn circle_multi_vector_transformation(self_1020: Circle, other_820: MultiVector) -> MultiVector {
    var self_1021: Circle;
    var other_821: MultiVector;

    self_1021 = self_1020;
    other_821 = other_820;
    let _e6: Circle = self_1021;
    let _e7: MultiVector = other_821;
    let _e8: MultiVector = circle_multi_vector_geometric_product(_e6, _e7);
    let _e10: Circle = self_1021;
    let _e11: Circle = circle_reversal(_e10);
    let _e14: Circle = self_1021;
    let _e15: MultiVector = other_821;
    let _e16: MultiVector = circle_multi_vector_geometric_product(_e14, _e15);
    let _e18: Circle = self_1021;
    let _e19: Circle = circle_reversal(_e18);
    let _e20: MultiVector = multi_vector_circle_geometric_product(_e16, _e19);
    return _e20;
}

fn circle_scalar_geometric_quotient(self_1022: Circle, other_822: Scalar) -> Circle {
    var self_1023: Circle;
    var other_823: Scalar;

    self_1023 = self_1022;
    other_823 = other_822;
    let _e6: Scalar = other_823;
    let _e7: Scalar = scalar_inverse(_e6);
    let _e8: Circle = self_1023;
    let _e10: Scalar = other_823;
    let _e11: Scalar = scalar_inverse(_e10);
    let _e12: Circle = circle_scalar_geometric_product(_e8, _e11);
    return _e12;
}

fn dipole_flat_point_transformation(self_1024: Dipole, other_824: FlatPoint) -> FlatPoint {
    var self_1025: Dipole;
    var other_825: FlatPoint;

    self_1025 = self_1024;
    other_825 = other_824;
    let _e6: Dipole = self_1025;
    let _e7: FlatPoint = other_825;
    let _e8: Flector = dipole_flat_point_geometric_product(_e6, _e7);
    let _e10: Dipole = self_1025;
    let _e11: Dipole = dipole_reversal(_e10);
    let _e14: Dipole = self_1025;
    let _e15: FlatPoint = other_825;
    let _e16: Flector = dipole_flat_point_geometric_product(_e14, _e15);
    let _e18: Dipole = self_1025;
    let _e19: Dipole = dipole_reversal(_e18);
    let _e20: Flector = flector_dipole_geometric_product(_e16, _e19);
    let _e23: Dipole = self_1025;
    let _e24: FlatPoint = other_825;
    let _e25: Flector = dipole_flat_point_geometric_product(_e23, _e24);
    let _e27: Dipole = self_1025;
    let _e28: Dipole = dipole_reversal(_e27);
    let _e31: Dipole = self_1025;
    let _e32: FlatPoint = other_825;
    let _e33: Flector = dipole_flat_point_geometric_product(_e31, _e32);
    let _e35: Dipole = self_1025;
    let _e36: Dipole = dipole_reversal(_e35);
    let _e37: Flector = flector_dipole_geometric_product(_e33, _e36);
    let _e38: FlatPoint = flector_flat_point_into(_e37);
    return _e38;
}

fn dipole_flector_transformation(self_1026: Dipole, other_826: Flector) -> Flector {
    var self_1027: Dipole;
    var other_827: Flector;

    self_1027 = self_1026;
    other_827 = other_826;
    let _e6: Dipole = self_1027;
    let _e7: Flector = other_827;
    let _e8: Flector = dipole_flector_geometric_product(_e6, _e7);
    let _e10: Dipole = self_1027;
    let _e11: Dipole = dipole_reversal(_e10);
    let _e14: Dipole = self_1027;
    let _e15: Flector = other_827;
    let _e16: Flector = dipole_flector_geometric_product(_e14, _e15);
    let _e18: Dipole = self_1027;
    let _e19: Dipole = dipole_reversal(_e18);
    let _e20: Flector = flector_dipole_geometric_product(_e16, _e19);
    return _e20;
}

fn dipole_line_transformation(self_1028: Dipole, other_828: Line) -> Line {
    var self_1029: Dipole;
    var other_829: Line;

    self_1029 = self_1028;
    other_829 = other_828;
    let _e6: Dipole = self_1029;
    let _e7: Line = other_829;
    let _e8: Motor = dipole_line_geometric_product(_e6, _e7);
    let _e10: Dipole = self_1029;
    let _e11: Dipole = dipole_reversal(_e10);
    let _e14: Dipole = self_1029;
    let _e15: Line = other_829;
    let _e16: Motor = dipole_line_geometric_product(_e14, _e15);
    let _e18: Dipole = self_1029;
    let _e19: Dipole = dipole_reversal(_e18);
    let _e20: Motor = motor_dipole_geometric_product(_e16, _e19);
    let _e23: Dipole = self_1029;
    let _e24: Line = other_829;
    let _e25: Motor = dipole_line_geometric_product(_e23, _e24);
    let _e27: Dipole = self_1029;
    let _e28: Dipole = dipole_reversal(_e27);
    let _e31: Dipole = self_1029;
    let _e32: Line = other_829;
    let _e33: Motor = dipole_line_geometric_product(_e31, _e32);
    let _e35: Dipole = self_1029;
    let _e36: Dipole = dipole_reversal(_e35);
    let _e37: Motor = motor_dipole_geometric_product(_e33, _e36);
    let _e38: Line = motor_line_into(_e37);
    return _e38;
}

fn dipole_motor_transformation(self_1030: Dipole, other_830: Motor) -> Motor {
    var self_1031: Dipole;
    var other_831: Motor;

    self_1031 = self_1030;
    other_831 = other_830;
    let _e6: Dipole = self_1031;
    let _e7: Motor = other_831;
    let _e8: Motor = dipole_motor_geometric_product(_e6, _e7);
    let _e10: Dipole = self_1031;
    let _e11: Dipole = dipole_reversal(_e10);
    let _e14: Dipole = self_1031;
    let _e15: Motor = other_831;
    let _e16: Motor = dipole_motor_geometric_product(_e14, _e15);
    let _e18: Dipole = self_1031;
    let _e19: Dipole = dipole_reversal(_e18);
    let _e20: Motor = motor_dipole_geometric_product(_e16, _e19);
    return _e20;
}

fn dipole_multi_vector_geometric_quotient(self_1032: Dipole, other_832: MultiVector) -> MultiVector {
    var self_1033: Dipole;
    var other_833: MultiVector;

    self_1033 = self_1032;
    other_833 = other_832;
    let _e6: MultiVector = other_833;
    let _e7: MultiVector = multi_vector_inverse(_e6);
    let _e8: Dipole = self_1033;
    let _e10: MultiVector = other_833;
    let _e11: MultiVector = multi_vector_inverse(_e10);
    let _e12: MultiVector = dipole_multi_vector_geometric_product(_e8, _e11);
    return _e12;
}

fn dipole_multi_vector_transformation(self_1034: Dipole, other_834: MultiVector) -> MultiVector {
    var self_1035: Dipole;
    var other_835: MultiVector;

    self_1035 = self_1034;
    other_835 = other_834;
    let _e6: Dipole = self_1035;
    let _e7: MultiVector = other_835;
    let _e8: MultiVector = dipole_multi_vector_geometric_product(_e6, _e7);
    let _e10: Dipole = self_1035;
    let _e11: Dipole = dipole_reversal(_e10);
    let _e14: Dipole = self_1035;
    let _e15: MultiVector = other_835;
    let _e16: MultiVector = dipole_multi_vector_geometric_product(_e14, _e15);
    let _e18: Dipole = self_1035;
    let _e19: Dipole = dipole_reversal(_e18);
    let _e20: MultiVector = multi_vector_dipole_geometric_product(_e16, _e19);
    return _e20;
}

fn dipole_rotor_transformation(self_1036: Dipole, other_836: Rotor) -> Rotor {
    var self_1037: Dipole;
    var other_837: Rotor;

    self_1037 = self_1036;
    other_837 = other_836;
    let _e6: Dipole = self_1037;
    let _e7: Rotor = other_837;
    let _e8: Rotor = dipole_rotor_geometric_product(_e6, _e7);
    let _e10: Dipole = self_1037;
    let _e11: Dipole = dipole_reversal(_e10);
    let _e14: Dipole = self_1037;
    let _e15: Rotor = other_837;
    let _e16: Rotor = dipole_rotor_geometric_product(_e14, _e15);
    let _e18: Dipole = self_1037;
    let _e19: Dipole = dipole_reversal(_e18);
    let _e20: Rotor = rotor_dipole_geometric_product(_e16, _e19);
    return _e20;
}

fn dipole_scalar_geometric_quotient(self_1038: Dipole, other_838: Scalar) -> Dipole {
    var self_1039: Dipole;
    var other_839: Scalar;

    self_1039 = self_1038;
    other_839 = other_838;
    let _e6: Scalar = other_839;
    let _e7: Scalar = scalar_inverse(_e6);
    let _e8: Dipole = self_1039;
    let _e10: Scalar = other_839;
    let _e11: Scalar = scalar_inverse(_e10);
    let _e12: Dipole = dipole_scalar_geometric_product(_e8, _e11);
    return _e12;
}

fn dipole_translator_transformation(self_1040: Dipole, other_840: Translator) -> Translator {
    var self_1041: Dipole;
    var other_841: Translator;

    self_1041 = self_1040;
    other_841 = other_840;
    let _e6: Dipole = self_1041;
    let _e7: Translator = other_841;
    let _e8: Motor = dipole_translator_geometric_product(_e6, _e7);
    let _e10: Dipole = self_1041;
    let _e11: Dipole = dipole_reversal(_e10);
    let _e14: Dipole = self_1041;
    let _e15: Translator = other_841;
    let _e16: Motor = dipole_translator_geometric_product(_e14, _e15);
    let _e18: Dipole = self_1041;
    let _e19: Dipole = dipole_reversal(_e18);
    let _e20: Motor = motor_dipole_geometric_product(_e16, _e19);
    let _e23: Dipole = self_1041;
    let _e24: Translator = other_841;
    let _e25: Motor = dipole_translator_geometric_product(_e23, _e24);
    let _e27: Dipole = self_1041;
    let _e28: Dipole = dipole_reversal(_e27);
    let _e31: Dipole = self_1041;
    let _e32: Translator = other_841;
    let _e33: Motor = dipole_translator_geometric_product(_e31, _e32);
    let _e35: Dipole = self_1041;
    let _e36: Dipole = dipole_reversal(_e35);
    let _e37: Motor = motor_dipole_geometric_product(_e33, _e36);
    let _e38: Translator = motor_translator_into(_e37);
    return _e38;
}

fn flat_point_dipole_geometric_quotient(self_1042: FlatPoint, other_842: Dipole) -> Flector {
    var self_1043: FlatPoint;
    var other_843: Dipole;

    self_1043 = self_1042;
    other_843 = other_842;
    let _e6: Dipole = other_843;
    let _e7: Dipole = dipole_inverse(_e6);
    let _e8: FlatPoint = self_1043;
    let _e10: Dipole = other_843;
    let _e11: Dipole = dipole_inverse(_e10);
    let _e12: Flector = flat_point_dipole_geometric_product(_e8, _e11);
    return _e12;
}

fn flat_point_scalar_geometric_quotient(self_1044: FlatPoint, other_844: Scalar) -> FlatPoint {
    var self_1045: FlatPoint;
    var other_845: Scalar;

    self_1045 = self_1044;
    other_845 = other_844;
    let _e6: Scalar = other_845;
    let _e7: Scalar = scalar_inverse(_e6);
    let _e8: FlatPoint = self_1045;
    let _e10: Scalar = other_845;
    let _e11: Scalar = scalar_inverse(_e10);
    let _e12: FlatPoint = flat_point_scalar_geometric_product(_e8, _e11);
    return _e12;
}

fn flector_circle_geometric_quotient(self_1046: Flector, other_846: Circle) -> Motor {
    var self_1047: Flector;
    var other_847: Circle;

    self_1047 = self_1046;
    other_847 = other_846;
    let _e6: Circle = other_847;
    let _e7: Circle = circle_inverse(_e6);
    let _e8: Flector = self_1047;
    let _e10: Circle = other_847;
    let _e11: Circle = circle_inverse(_e10);
    let _e12: Motor = flector_circle_geometric_product(_e8, _e11);
    return _e12;
}

fn flector_dipole_geometric_quotient(self_1048: Flector, other_848: Dipole) -> Flector {
    var self_1049: Flector;
    var other_849: Dipole;

    self_1049 = self_1048;
    other_849 = other_848;
    let _e6: Dipole = other_849;
    let _e7: Dipole = dipole_inverse(_e6);
    let _e8: Flector = self_1049;
    let _e10: Dipole = other_849;
    let _e11: Dipole = dipole_inverse(_e10);
    let _e12: Flector = flector_dipole_geometric_product(_e8, _e11);
    return _e12;
}

fn flector_radial_point_geometric_quotient(self_1050: Flector, other_850: RadialPoint) -> Motor {
    var self_1051: Flector;
    var other_851: RadialPoint;

    self_1051 = self_1050;
    other_851 = other_850;
    let _e6: RadialPoint = other_851;
    let _e7: RadialPoint = radial_point_inverse(_e6);
    let _e8: Flector = self_1051;
    let _e10: RadialPoint = other_851;
    let _e11: RadialPoint = radial_point_inverse(_e10);
    let _e12: Motor = flector_radial_point_geometric_product(_e8, _e11);
    return _e12;
}

fn flector_scalar_geometric_quotient(self_1052: Flector, other_852: Scalar) -> Flector {
    var self_1053: Flector;
    var other_853: Scalar;

    self_1053 = self_1052;
    other_853 = other_852;
    let _e6: Scalar = other_853;
    let _e7: Scalar = scalar_inverse(_e6);
    let _e8: Flector = self_1053;
    let _e10: Scalar = other_853;
    let _e11: Scalar = scalar_inverse(_e10);
    let _e12: Flector = flector_scalar_geometric_product(_e8, _e11);
    return _e12;
}

fn line_dipole_geometric_quotient(self_1054: Line, other_854: Dipole) -> Motor {
    var self_1055: Line;
    var other_855: Dipole;

    self_1055 = self_1054;
    other_855 = other_854;
    let _e6: Dipole = other_855;
    let _e7: Dipole = dipole_inverse(_e6);
    let _e8: Line = self_1055;
    let _e10: Dipole = other_855;
    let _e11: Dipole = dipole_inverse(_e10);
    let _e12: Motor = line_dipole_geometric_product(_e8, _e11);
    return _e12;
}

fn line_radial_point_geometric_quotient(self_1056: Line, other_856: RadialPoint) -> Flector {
    var self_1057: Line;
    var other_857: RadialPoint;

    self_1057 = self_1056;
    other_857 = other_856;
    let _e6: RadialPoint = other_857;
    let _e7: RadialPoint = radial_point_inverse(_e6);
    let _e8: Line = self_1057;
    let _e10: RadialPoint = other_857;
    let _e11: RadialPoint = radial_point_inverse(_e10);
    let _e12: Flector = line_radial_point_geometric_product(_e8, _e11);
    return _e12;
}

fn line_scalar_geometric_quotient(self_1058: Line, other_858: Scalar) -> Line {
    var self_1059: Line;
    var other_859: Scalar;

    self_1059 = self_1058;
    other_859 = other_858;
    let _e6: Scalar = other_859;
    let _e7: Scalar = scalar_inverse(_e6);
    let _e8: Line = self_1059;
    let _e10: Scalar = other_859;
    let _e11: Scalar = scalar_inverse(_e10);
    let _e12: Line = line_scalar_geometric_product(_e8, _e11);
    return _e12;
}

fn motor_circle_geometric_quotient(self_1060: Motor, other_860: Circle) -> Flector {
    var self_1061: Motor;
    var other_861: Circle;

    self_1061 = self_1060;
    other_861 = other_860;
    let _e6: Circle = other_861;
    let _e7: Circle = circle_inverse(_e6);
    let _e8: Motor = self_1061;
    let _e10: Circle = other_861;
    let _e11: Circle = circle_inverse(_e10);
    let _e12: Flector = motor_circle_geometric_product(_e8, _e11);
    return _e12;
}

fn motor_dipole_geometric_quotient(self_1062: Motor, other_862: Dipole) -> Motor {
    var self_1063: Motor;
    var other_863: Dipole;

    self_1063 = self_1062;
    other_863 = other_862;
    let _e6: Dipole = other_863;
    let _e7: Dipole = dipole_inverse(_e6);
    let _e8: Motor = self_1063;
    let _e10: Dipole = other_863;
    let _e11: Dipole = dipole_inverse(_e10);
    let _e12: Motor = motor_dipole_geometric_product(_e8, _e11);
    return _e12;
}

fn motor_radial_point_geometric_quotient(self_1064: Motor, other_864: RadialPoint) -> Flector {
    var self_1065: Motor;
    var other_865: RadialPoint;

    self_1065 = self_1064;
    other_865 = other_864;
    let _e6: RadialPoint = other_865;
    let _e7: RadialPoint = radial_point_inverse(_e6);
    let _e8: Motor = self_1065;
    let _e10: RadialPoint = other_865;
    let _e11: RadialPoint = radial_point_inverse(_e10);
    let _e12: Flector = motor_radial_point_geometric_product(_e8, _e11);
    return _e12;
}

fn motor_scalar_geometric_quotient(self_1066: Motor, other_866: Scalar) -> Motor {
    var self_1067: Motor;
    var other_867: Scalar;

    self_1067 = self_1066;
    other_867 = other_866;
    let _e6: Scalar = other_867;
    let _e7: Scalar = scalar_inverse(_e6);
    let _e8: Motor = self_1067;
    let _e10: Scalar = other_867;
    let _e11: Scalar = scalar_inverse(_e10);
    let _e12: Motor = motor_scalar_geometric_product(_e8, _e11);
    return _e12;
}

fn multi_vector_circle_geometric_quotient(self_1068: MultiVector, other_868: Circle) -> MultiVector {
    var self_1069: MultiVector;
    var other_869: Circle;

    self_1069 = self_1068;
    other_869 = other_868;
    let _e6: Circle = other_869;
    let _e7: Circle = circle_inverse(_e6);
    let _e8: MultiVector = self_1069;
    let _e10: Circle = other_869;
    let _e11: Circle = circle_inverse(_e10);
    let _e12: MultiVector = multi_vector_circle_geometric_product(_e8, _e11);
    return _e12;
}

fn multi_vector_circle_transformation(self_1070: MultiVector, other_870: Circle) -> Circle {
    var self_1071: MultiVector;
    var other_871: Circle;

    self_1071 = self_1070;
    other_871 = other_870;
    let _e6: MultiVector = self_1071;
    let _e7: Circle = other_871;
    let _e8: MultiVector = multi_vector_circle_geometric_product(_e6, _e7);
    let _e10: MultiVector = self_1071;
    let _e11: MultiVector = multi_vector_reversal(_e10);
    let _e14: MultiVector = self_1071;
    let _e15: Circle = other_871;
    let _e16: MultiVector = multi_vector_circle_geometric_product(_e14, _e15);
    let _e18: MultiVector = self_1071;
    let _e19: MultiVector = multi_vector_reversal(_e18);
    let _e20: MultiVector = multi_vector_multi_vector_geometric_product(_e16, _e19);
    let _e23: MultiVector = self_1071;
    let _e24: Circle = other_871;
    let _e25: MultiVector = multi_vector_circle_geometric_product(_e23, _e24);
    let _e27: MultiVector = self_1071;
    let _e28: MultiVector = multi_vector_reversal(_e27);
    let _e31: MultiVector = self_1071;
    let _e32: Circle = other_871;
    let _e33: MultiVector = multi_vector_circle_geometric_product(_e31, _e32);
    let _e35: MultiVector = self_1071;
    let _e36: MultiVector = multi_vector_reversal(_e35);
    let _e37: MultiVector = multi_vector_multi_vector_geometric_product(_e33, _e36);
    let _e38: Circle = multi_vector_circle_into(_e37);
    return _e38;
}

fn multi_vector_dipole_geometric_quotient(self_1072: MultiVector, other_872: Dipole) -> MultiVector {
    var self_1073: MultiVector;
    var other_873: Dipole;

    self_1073 = self_1072;
    other_873 = other_872;
    let _e6: Dipole = other_873;
    let _e7: Dipole = dipole_inverse(_e6);
    let _e8: MultiVector = self_1073;
    let _e10: Dipole = other_873;
    let _e11: Dipole = dipole_inverse(_e10);
    let _e12: MultiVector = multi_vector_dipole_geometric_product(_e8, _e11);
    return _e12;
}

fn multi_vector_dipole_transformation(self_1074: MultiVector, other_874: Dipole) -> Dipole {
    var self_1075: MultiVector;
    var other_875: Dipole;

    self_1075 = self_1074;
    other_875 = other_874;
    let _e6: MultiVector = self_1075;
    let _e7: Dipole = other_875;
    let _e8: MultiVector = multi_vector_dipole_geometric_product(_e6, _e7);
    let _e10: MultiVector = self_1075;
    let _e11: MultiVector = multi_vector_reversal(_e10);
    let _e14: MultiVector = self_1075;
    let _e15: Dipole = other_875;
    let _e16: MultiVector = multi_vector_dipole_geometric_product(_e14, _e15);
    let _e18: MultiVector = self_1075;
    let _e19: MultiVector = multi_vector_reversal(_e18);
    let _e20: MultiVector = multi_vector_multi_vector_geometric_product(_e16, _e19);
    let _e23: MultiVector = self_1075;
    let _e24: Dipole = other_875;
    let _e25: MultiVector = multi_vector_dipole_geometric_product(_e23, _e24);
    let _e27: MultiVector = self_1075;
    let _e28: MultiVector = multi_vector_reversal(_e27);
    let _e31: MultiVector = self_1075;
    let _e32: Dipole = other_875;
    let _e33: MultiVector = multi_vector_dipole_geometric_product(_e31, _e32);
    let _e35: MultiVector = self_1075;
    let _e36: MultiVector = multi_vector_reversal(_e35);
    let _e37: MultiVector = multi_vector_multi_vector_geometric_product(_e33, _e36);
    let _e38: Dipole = multi_vector_dipole_into(_e37);
    return _e38;
}

fn multi_vector_powi(self_1076: MultiVector, exponent: i32) -> MultiVector {
    var self_1077: MultiVector;
    var exponent_1: i32;
    var local: MultiVector;
    var x: MultiVector;
    var y: MultiVector;
    var n: i32;

    self_1077 = self_1076;
    exponent_1 = exponent;
    let _e4: i32 = exponent_1;
    if (_e4 == 0) {
        {
            let _e7: MultiVector = multi_vector_one();
            return _e7;
        }
    }
    let _e8: i32 = exponent_1;
    if (_e8 < 0) {
        let _e12: MultiVector = self_1077;
        let _e13: MultiVector = multi_vector_inverse(_e12);
        local = _e13;
    } else {
        let _e14: MultiVector = self_1077;
        local = _e14;
    }
    let _e16: MultiVector = local;
    x = _e16;
    let _e18: MultiVector = multi_vector_one();
    y = _e18;
    let _e21: i32 = exponent_1;
    n = abs(_e21);
    loop {
        let _e25: i32 = n;
        if !((1 < _e25)) {
            break;
        }
        {
            let _e28: i32 = n;
            if ((_e28 & 1) == 1) {
                {
                    let _e35: MultiVector = x;
                    let _e36: MultiVector = y;
                    let _e37: MultiVector = multi_vector_multi_vector_geometric_product(_e35, _e36);
                    y = _e37;
                }
            }
            let _e40: MultiVector = x;
            let _e41: MultiVector = x;
            let _e42: MultiVector = multi_vector_multi_vector_geometric_product(_e40, _e41);
            x = _e42;
            let _e43: i32 = n;
            n = (_e43 >> u32(1));
        }
    }
    let _e49: MultiVector = x;
    let _e50: MultiVector = y;
    let _e51: MultiVector = multi_vector_multi_vector_geometric_product(_e49, _e50);
    return _e51;
}

fn multi_vector_multi_vector_geometric_quotient(self_1078: MultiVector, other_876: MultiVector) -> MultiVector {
    var self_1079: MultiVector;
    var other_877: MultiVector;

    self_1079 = self_1078;
    other_877 = other_876;
    let _e6: MultiVector = other_877;
    let _e7: MultiVector = multi_vector_inverse(_e6);
    let _e8: MultiVector = self_1079;
    let _e10: MultiVector = other_877;
    let _e11: MultiVector = multi_vector_inverse(_e10);
    let _e12: MultiVector = multi_vector_multi_vector_geometric_product(_e8, _e11);
    return _e12;
}

fn multi_vector_multi_vector_transformation(self_1080: MultiVector, other_878: MultiVector) -> MultiVector {
    var self_1081: MultiVector;
    var other_879: MultiVector;

    self_1081 = self_1080;
    other_879 = other_878;
    let _e6: MultiVector = self_1081;
    let _e7: MultiVector = other_879;
    let _e8: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e7);
    let _e10: MultiVector = self_1081;
    let _e11: MultiVector = multi_vector_reversal(_e10);
    let _e14: MultiVector = self_1081;
    let _e15: MultiVector = other_879;
    let _e16: MultiVector = multi_vector_multi_vector_geometric_product(_e14, _e15);
    let _e18: MultiVector = self_1081;
    let _e19: MultiVector = multi_vector_reversal(_e18);
    let _e20: MultiVector = multi_vector_multi_vector_geometric_product(_e16, _e19);
    return _e20;
}

fn multi_vector_radial_point_geometric_quotient(self_1082: MultiVector, other_880: RadialPoint) -> MultiVector {
    var self_1083: MultiVector;
    var other_881: RadialPoint;

    self_1083 = self_1082;
    other_881 = other_880;
    let _e6: RadialPoint = other_881;
    let _e7: RadialPoint = radial_point_inverse(_e6);
    let _e8: MultiVector = self_1083;
    let _e10: RadialPoint = other_881;
    let _e11: RadialPoint = radial_point_inverse(_e10);
    let _e12: MultiVector = multi_vector_radial_point_geometric_product(_e8, _e11);
    return _e12;
}

fn multi_vector_radial_point_transformation(self_1084: MultiVector, other_882: RadialPoint) -> RadialPoint {
    var self_1085: MultiVector;
    var other_883: RadialPoint;

    self_1085 = self_1084;
    other_883 = other_882;
    let _e6: MultiVector = self_1085;
    let _e7: RadialPoint = other_883;
    let _e8: MultiVector = multi_vector_radial_point_geometric_product(_e6, _e7);
    let _e10: MultiVector = self_1085;
    let _e11: MultiVector = multi_vector_reversal(_e10);
    let _e14: MultiVector = self_1085;
    let _e15: RadialPoint = other_883;
    let _e16: MultiVector = multi_vector_radial_point_geometric_product(_e14, _e15);
    let _e18: MultiVector = self_1085;
    let _e19: MultiVector = multi_vector_reversal(_e18);
    let _e20: MultiVector = multi_vector_multi_vector_geometric_product(_e16, _e19);
    let _e23: MultiVector = self_1085;
    let _e24: RadialPoint = other_883;
    let _e25: MultiVector = multi_vector_radial_point_geometric_product(_e23, _e24);
    let _e27: MultiVector = self_1085;
    let _e28: MultiVector = multi_vector_reversal(_e27);
    let _e31: MultiVector = self_1085;
    let _e32: RadialPoint = other_883;
    let _e33: MultiVector = multi_vector_radial_point_geometric_product(_e31, _e32);
    let _e35: MultiVector = self_1085;
    let _e36: MultiVector = multi_vector_reversal(_e35);
    let _e37: MultiVector = multi_vector_multi_vector_geometric_product(_e33, _e36);
    let _e38: RadialPoint = multi_vector_radial_point_into(_e37);
    return _e38;
}

fn multi_vector_scalar_geometric_quotient(self_1086: MultiVector, other_884: Scalar) -> MultiVector {
    var self_1087: MultiVector;
    var other_885: Scalar;

    self_1087 = self_1086;
    other_885 = other_884;
    let _e6: Scalar = other_885;
    let _e7: Scalar = scalar_inverse(_e6);
    let _e8: MultiVector = self_1087;
    let _e10: Scalar = other_885;
    let _e11: Scalar = scalar_inverse(_e10);
    let _e12: MultiVector = multi_vector_scalar_geometric_product(_e8, _e11);
    return _e12;
}

fn multi_vector_scalar_transformation(self_1088: MultiVector, other_886: Scalar) -> Scalar {
    var self_1089: MultiVector;
    var other_887: Scalar;

    self_1089 = self_1088;
    other_887 = other_886;
    let _e6: MultiVector = self_1089;
    let _e7: Scalar = other_887;
    let _e8: MultiVector = multi_vector_scalar_geometric_product(_e6, _e7);
    let _e10: MultiVector = self_1089;
    let _e11: MultiVector = multi_vector_reversal(_e10);
    let _e14: MultiVector = self_1089;
    let _e15: Scalar = other_887;
    let _e16: MultiVector = multi_vector_scalar_geometric_product(_e14, _e15);
    let _e18: MultiVector = self_1089;
    let _e19: MultiVector = multi_vector_reversal(_e18);
    let _e20: MultiVector = multi_vector_multi_vector_geometric_product(_e16, _e19);
    let _e23: MultiVector = self_1089;
    let _e24: Scalar = other_887;
    let _e25: MultiVector = multi_vector_scalar_geometric_product(_e23, _e24);
    let _e27: MultiVector = self_1089;
    let _e28: MultiVector = multi_vector_reversal(_e27);
    let _e31: MultiVector = self_1089;
    let _e32: Scalar = other_887;
    let _e33: MultiVector = multi_vector_scalar_geometric_product(_e31, _e32);
    let _e35: MultiVector = self_1089;
    let _e36: MultiVector = multi_vector_reversal(_e35);
    let _e37: MultiVector = multi_vector_multi_vector_geometric_product(_e33, _e36);
    let _e38: Scalar = multi_vector_scalar_into(_e37);
    return _e38;
}

fn plane_scalar_geometric_quotient(self_1090: Plane, other_888: Scalar) -> Plane {
    var self_1091: Plane;
    var other_889: Scalar;

    self_1091 = self_1090;
    other_889 = other_888;
    let _e6: Scalar = other_889;
    let _e7: Scalar = scalar_inverse(_e6);
    let _e8: Plane = self_1091;
    let _e10: Scalar = other_889;
    let _e11: Scalar = scalar_inverse(_e10);
    let _e12: Plane = plane_scalar_geometric_product(_e8, _e11);
    return _e12;
}

fn radial_point_flector_transformation(self_1092: RadialPoint, other_890: Flector) -> Flector {
    var self_1093: RadialPoint;
    var other_891: Flector;

    self_1093 = self_1092;
    other_891 = other_890;
    let _e6: RadialPoint = self_1093;
    let _e7: Flector = other_891;
    let _e8: Motor = radial_point_flector_geometric_product(_e6, _e7);
    let _e10: RadialPoint = self_1093;
    let _e11: RadialPoint = radial_point_reversal(_e10);
    let _e14: RadialPoint = self_1093;
    let _e15: Flector = other_891;
    let _e16: Motor = radial_point_flector_geometric_product(_e14, _e15);
    let _e18: RadialPoint = self_1093;
    let _e19: RadialPoint = radial_point_reversal(_e18);
    let _e20: Flector = motor_radial_point_geometric_product(_e16, _e19);
    return _e20;
}

fn radial_point_line_transformation(self_1094: RadialPoint, other_892: Line) -> Line {
    var self_1095: RadialPoint;
    var other_893: Line;

    self_1095 = self_1094;
    other_893 = other_892;
    let _e6: RadialPoint = self_1095;
    let _e7: Line = other_893;
    let _e8: Flector = radial_point_line_geometric_product(_e6, _e7);
    let _e10: RadialPoint = self_1095;
    let _e11: RadialPoint = radial_point_reversal(_e10);
    let _e14: RadialPoint = self_1095;
    let _e15: Line = other_893;
    let _e16: Flector = radial_point_line_geometric_product(_e14, _e15);
    let _e18: RadialPoint = self_1095;
    let _e19: RadialPoint = radial_point_reversal(_e18);
    let _e20: Motor = flector_radial_point_geometric_product(_e16, _e19);
    let _e23: RadialPoint = self_1095;
    let _e24: Line = other_893;
    let _e25: Flector = radial_point_line_geometric_product(_e23, _e24);
    let _e27: RadialPoint = self_1095;
    let _e28: RadialPoint = radial_point_reversal(_e27);
    let _e31: RadialPoint = self_1095;
    let _e32: Line = other_893;
    let _e33: Flector = radial_point_line_geometric_product(_e31, _e32);
    let _e35: RadialPoint = self_1095;
    let _e36: RadialPoint = radial_point_reversal(_e35);
    let _e37: Motor = flector_radial_point_geometric_product(_e33, _e36);
    let _e38: Line = motor_line_into(_e37);
    return _e38;
}

fn radial_point_motor_transformation(self_1096: RadialPoint, other_894: Motor) -> Motor {
    var self_1097: RadialPoint;
    var other_895: Motor;

    self_1097 = self_1096;
    other_895 = other_894;
    let _e6: RadialPoint = self_1097;
    let _e7: Motor = other_895;
    let _e8: Flector = radial_point_motor_geometric_product(_e6, _e7);
    let _e10: RadialPoint = self_1097;
    let _e11: RadialPoint = radial_point_reversal(_e10);
    let _e14: RadialPoint = self_1097;
    let _e15: Motor = other_895;
    let _e16: Flector = radial_point_motor_geometric_product(_e14, _e15);
    let _e18: RadialPoint = self_1097;
    let _e19: RadialPoint = radial_point_reversal(_e18);
    let _e20: Motor = flector_radial_point_geometric_product(_e16, _e19);
    return _e20;
}

fn radial_point_multi_vector_geometric_quotient(self_1098: RadialPoint, other_896: MultiVector) -> MultiVector {
    var self_1099: RadialPoint;
    var other_897: MultiVector;

    self_1099 = self_1098;
    other_897 = other_896;
    let _e6: MultiVector = other_897;
    let _e7: MultiVector = multi_vector_inverse(_e6);
    let _e8: RadialPoint = self_1099;
    let _e10: MultiVector = other_897;
    let _e11: MultiVector = multi_vector_inverse(_e10);
    let _e12: MultiVector = radial_point_multi_vector_geometric_product(_e8, _e11);
    return _e12;
}

fn radial_point_multi_vector_transformation(self_1100: RadialPoint, other_898: MultiVector) -> MultiVector {
    var self_1101: RadialPoint;
    var other_899: MultiVector;

    self_1101 = self_1100;
    other_899 = other_898;
    let _e6: RadialPoint = self_1101;
    let _e7: MultiVector = other_899;
    let _e8: MultiVector = radial_point_multi_vector_geometric_product(_e6, _e7);
    let _e10: RadialPoint = self_1101;
    let _e11: RadialPoint = radial_point_reversal(_e10);
    let _e14: RadialPoint = self_1101;
    let _e15: MultiVector = other_899;
    let _e16: MultiVector = radial_point_multi_vector_geometric_product(_e14, _e15);
    let _e18: RadialPoint = self_1101;
    let _e19: RadialPoint = radial_point_reversal(_e18);
    let _e20: MultiVector = multi_vector_radial_point_geometric_product(_e16, _e19);
    return _e20;
}

fn radial_point_scalar_geometric_quotient(self_1102: RadialPoint, other_900: Scalar) -> RadialPoint {
    var self_1103: RadialPoint;
    var other_901: Scalar;

    self_1103 = self_1102;
    other_901 = other_900;
    let _e6: Scalar = other_901;
    let _e7: Scalar = scalar_inverse(_e6);
    let _e8: RadialPoint = self_1103;
    let _e10: Scalar = other_901;
    let _e11: Scalar = scalar_inverse(_e10);
    let _e12: RadialPoint = radial_point_scalar_geometric_product(_e8, _e11);
    return _e12;
}

fn rotor_dipole_geometric_quotient(self_1104: Rotor, other_902: Dipole) -> Rotor {
    var self_1105: Rotor;
    var other_903: Dipole;

    self_1105 = self_1104;
    other_903 = other_902;
    let _e6: Dipole = other_903;
    let _e7: Dipole = dipole_inverse(_e6);
    let _e8: Rotor = self_1105;
    let _e10: Dipole = other_903;
    let _e11: Dipole = dipole_inverse(_e10);
    let _e12: Rotor = rotor_dipole_geometric_product(_e8, _e11);
    return _e12;
}

fn rotor_scalar_geometric_quotient(self_1106: Rotor, other_904: Scalar) -> Rotor {
    var self_1107: Rotor;
    var other_905: Scalar;

    self_1107 = self_1106;
    other_905 = other_904;
    let _e6: Scalar = other_905;
    let _e7: Scalar = scalar_inverse(_e6);
    let _e8: Rotor = self_1107;
    let _e10: Scalar = other_905;
    let _e11: Scalar = scalar_inverse(_e10);
    let _e12: Rotor = rotor_scalar_geometric_product(_e8, _e11);
    return _e12;
}

fn scalar_anti_scalar_transformation(self_1108: Scalar, other_906: AntiScalar) -> AntiScalar {
    var self_1109: Scalar;
    var other_907: AntiScalar;

    self_1109 = self_1108;
    other_907 = other_906;
    let _e6: Scalar = self_1109;
    let _e7: AntiScalar = other_907;
    let _e8: AntiScalar = scalar_anti_scalar_geometric_product(_e6, _e7);
    let _e10: Scalar = self_1109;
    let _e11: Scalar = scalar_reversal(_e10);
    let _e14: Scalar = self_1109;
    let _e15: AntiScalar = other_907;
    let _e16: AntiScalar = scalar_anti_scalar_geometric_product(_e14, _e15);
    let _e18: Scalar = self_1109;
    let _e19: Scalar = scalar_reversal(_e18);
    let _e20: AntiScalar = anti_scalar_scalar_geometric_product(_e16, _e19);
    return _e20;
}

fn scalar_circle_geometric_quotient(self_1110: Scalar, other_908: Circle) -> Circle {
    var self_1111: Scalar;
    var other_909: Circle;

    self_1111 = self_1110;
    other_909 = other_908;
    let _e6: Circle = other_909;
    let _e7: Circle = circle_inverse(_e6);
    let _e8: Scalar = self_1111;
    let _e10: Circle = other_909;
    let _e11: Circle = circle_inverse(_e10);
    let _e12: Circle = scalar_circle_geometric_product(_e8, _e11);
    return _e12;
}

fn scalar_circle_transformation(self_1112: Scalar, other_910: Circle) -> Circle {
    var self_1113: Scalar;
    var other_911: Circle;

    self_1113 = self_1112;
    other_911 = other_910;
    let _e6: Scalar = self_1113;
    let _e7: Circle = other_911;
    let _e8: Circle = scalar_circle_geometric_product(_e6, _e7);
    let _e10: Scalar = self_1113;
    let _e11: Scalar = scalar_reversal(_e10);
    let _e14: Scalar = self_1113;
    let _e15: Circle = other_911;
    let _e16: Circle = scalar_circle_geometric_product(_e14, _e15);
    let _e18: Scalar = self_1113;
    let _e19: Scalar = scalar_reversal(_e18);
    let _e20: Circle = circle_scalar_geometric_product(_e16, _e19);
    return _e20;
}

fn scalar_dipole_geometric_quotient(self_1114: Scalar, other_912: Dipole) -> Dipole {
    var self_1115: Scalar;
    var other_913: Dipole;

    self_1115 = self_1114;
    other_913 = other_912;
    let _e6: Dipole = other_913;
    let _e7: Dipole = dipole_inverse(_e6);
    let _e8: Scalar = self_1115;
    let _e10: Dipole = other_913;
    let _e11: Dipole = dipole_inverse(_e10);
    let _e12: Dipole = scalar_dipole_geometric_product(_e8, _e11);
    return _e12;
}

fn scalar_dipole_transformation(self_1116: Scalar, other_914: Dipole) -> Dipole {
    var self_1117: Scalar;
    var other_915: Dipole;

    self_1117 = self_1116;
    other_915 = other_914;
    let _e6: Scalar = self_1117;
    let _e7: Dipole = other_915;
    let _e8: Dipole = scalar_dipole_geometric_product(_e6, _e7);
    let _e10: Scalar = self_1117;
    let _e11: Scalar = scalar_reversal(_e10);
    let _e14: Scalar = self_1117;
    let _e15: Dipole = other_915;
    let _e16: Dipole = scalar_dipole_geometric_product(_e14, _e15);
    let _e18: Scalar = self_1117;
    let _e19: Scalar = scalar_reversal(_e18);
    let _e20: Dipole = dipole_scalar_geometric_product(_e16, _e19);
    return _e20;
}

fn scalar_flat_point_transformation(self_1118: Scalar, other_916: FlatPoint) -> FlatPoint {
    var self_1119: Scalar;
    var other_917: FlatPoint;

    self_1119 = self_1118;
    other_917 = other_916;
    let _e6: Scalar = self_1119;
    let _e7: FlatPoint = other_917;
    let _e8: FlatPoint = scalar_flat_point_geometric_product(_e6, _e7);
    let _e10: Scalar = self_1119;
    let _e11: Scalar = scalar_reversal(_e10);
    let _e14: Scalar = self_1119;
    let _e15: FlatPoint = other_917;
    let _e16: FlatPoint = scalar_flat_point_geometric_product(_e14, _e15);
    let _e18: Scalar = self_1119;
    let _e19: Scalar = scalar_reversal(_e18);
    let _e20: FlatPoint = flat_point_scalar_geometric_product(_e16, _e19);
    return _e20;
}

fn scalar_flector_transformation(self_1120: Scalar, other_918: Flector) -> Flector {
    var self_1121: Scalar;
    var other_919: Flector;

    self_1121 = self_1120;
    other_919 = other_918;
    let _e6: Scalar = self_1121;
    let _e7: Flector = other_919;
    let _e8: Flector = scalar_flector_geometric_product(_e6, _e7);
    let _e10: Scalar = self_1121;
    let _e11: Scalar = scalar_reversal(_e10);
    let _e14: Scalar = self_1121;
    let _e15: Flector = other_919;
    let _e16: Flector = scalar_flector_geometric_product(_e14, _e15);
    let _e18: Scalar = self_1121;
    let _e19: Scalar = scalar_reversal(_e18);
    let _e20: Flector = flector_scalar_geometric_product(_e16, _e19);
    return _e20;
}

fn scalar_line_transformation(self_1122: Scalar, other_920: Line) -> Line {
    var self_1123: Scalar;
    var other_921: Line;

    self_1123 = self_1122;
    other_921 = other_920;
    let _e6: Scalar = self_1123;
    let _e7: Line = other_921;
    let _e8: Line = scalar_line_geometric_product(_e6, _e7);
    let _e10: Scalar = self_1123;
    let _e11: Scalar = scalar_reversal(_e10);
    let _e14: Scalar = self_1123;
    let _e15: Line = other_921;
    let _e16: Line = scalar_line_geometric_product(_e14, _e15);
    let _e18: Scalar = self_1123;
    let _e19: Scalar = scalar_reversal(_e18);
    let _e20: Line = line_scalar_geometric_product(_e16, _e19);
    return _e20;
}

fn scalar_motor_transformation(self_1124: Scalar, other_922: Motor) -> Motor {
    var self_1125: Scalar;
    var other_923: Motor;

    self_1125 = self_1124;
    other_923 = other_922;
    let _e6: Scalar = self_1125;
    let _e7: Motor = other_923;
    let _e8: Motor = scalar_motor_geometric_product(_e6, _e7);
    let _e10: Scalar = self_1125;
    let _e11: Scalar = scalar_reversal(_e10);
    let _e14: Scalar = self_1125;
    let _e15: Motor = other_923;
    let _e16: Motor = scalar_motor_geometric_product(_e14, _e15);
    let _e18: Scalar = self_1125;
    let _e19: Scalar = scalar_reversal(_e18);
    let _e20: Motor = motor_scalar_geometric_product(_e16, _e19);
    return _e20;
}

fn scalar_multi_vector_geometric_quotient(self_1126: Scalar, other_924: MultiVector) -> MultiVector {
    var self_1127: Scalar;
    var other_925: MultiVector;

    self_1127 = self_1126;
    other_925 = other_924;
    let _e6: MultiVector = other_925;
    let _e7: MultiVector = multi_vector_inverse(_e6);
    let _e8: Scalar = self_1127;
    let _e10: MultiVector = other_925;
    let _e11: MultiVector = multi_vector_inverse(_e10);
    let _e12: MultiVector = scalar_multi_vector_geometric_product(_e8, _e11);
    return _e12;
}

fn scalar_multi_vector_transformation(self_1128: Scalar, other_926: MultiVector) -> MultiVector {
    var self_1129: Scalar;
    var other_927: MultiVector;

    self_1129 = self_1128;
    other_927 = other_926;
    let _e6: Scalar = self_1129;
    let _e7: MultiVector = other_927;
    let _e8: MultiVector = scalar_multi_vector_geometric_product(_e6, _e7);
    let _e10: Scalar = self_1129;
    let _e11: Scalar = scalar_reversal(_e10);
    let _e14: Scalar = self_1129;
    let _e15: MultiVector = other_927;
    let _e16: MultiVector = scalar_multi_vector_geometric_product(_e14, _e15);
    let _e18: Scalar = self_1129;
    let _e19: Scalar = scalar_reversal(_e18);
    let _e20: MultiVector = multi_vector_scalar_geometric_product(_e16, _e19);
    return _e20;
}

fn scalar_plane_transformation(self_1130: Scalar, other_928: Plane) -> Plane {
    var self_1131: Scalar;
    var other_929: Plane;

    self_1131 = self_1130;
    other_929 = other_928;
    let _e6: Scalar = self_1131;
    let _e7: Plane = other_929;
    let _e8: Plane = scalar_plane_geometric_product(_e6, _e7);
    let _e10: Scalar = self_1131;
    let _e11: Scalar = scalar_reversal(_e10);
    let _e14: Scalar = self_1131;
    let _e15: Plane = other_929;
    let _e16: Plane = scalar_plane_geometric_product(_e14, _e15);
    let _e18: Scalar = self_1131;
    let _e19: Scalar = scalar_reversal(_e18);
    let _e20: Plane = plane_scalar_geometric_product(_e16, _e19);
    return _e20;
}

fn scalar_radial_point_geometric_quotient(self_1132: Scalar, other_930: RadialPoint) -> RadialPoint {
    var self_1133: Scalar;
    var other_931: RadialPoint;

    self_1133 = self_1132;
    other_931 = other_930;
    let _e6: RadialPoint = other_931;
    let _e7: RadialPoint = radial_point_inverse(_e6);
    let _e8: Scalar = self_1133;
    let _e10: RadialPoint = other_931;
    let _e11: RadialPoint = radial_point_inverse(_e10);
    let _e12: RadialPoint = scalar_radial_point_geometric_product(_e8, _e11);
    return _e12;
}

fn scalar_radial_point_transformation(self_1134: Scalar, other_932: RadialPoint) -> RadialPoint {
    var self_1135: Scalar;
    var other_933: RadialPoint;

    self_1135 = self_1134;
    other_933 = other_932;
    let _e6: Scalar = self_1135;
    let _e7: RadialPoint = other_933;
    let _e8: RadialPoint = scalar_radial_point_geometric_product(_e6, _e7);
    let _e10: Scalar = self_1135;
    let _e11: Scalar = scalar_reversal(_e10);
    let _e14: Scalar = self_1135;
    let _e15: RadialPoint = other_933;
    let _e16: RadialPoint = scalar_radial_point_geometric_product(_e14, _e15);
    let _e18: Scalar = self_1135;
    let _e19: Scalar = scalar_reversal(_e18);
    let _e20: RadialPoint = radial_point_scalar_geometric_product(_e16, _e19);
    return _e20;
}

fn scalar_rotor_transformation(self_1136: Scalar, other_934: Rotor) -> Rotor {
    var self_1137: Scalar;
    var other_935: Rotor;

    self_1137 = self_1136;
    other_935 = other_934;
    let _e6: Scalar = self_1137;
    let _e7: Rotor = other_935;
    let _e8: Rotor = scalar_rotor_geometric_product(_e6, _e7);
    let _e10: Scalar = self_1137;
    let _e11: Scalar = scalar_reversal(_e10);
    let _e14: Scalar = self_1137;
    let _e15: Rotor = other_935;
    let _e16: Rotor = scalar_rotor_geometric_product(_e14, _e15);
    let _e18: Scalar = self_1137;
    let _e19: Scalar = scalar_reversal(_e18);
    let _e20: Rotor = rotor_scalar_geometric_product(_e16, _e19);
    return _e20;
}

fn scalar_powi(self_1138: Scalar, exponent_2: i32) -> Scalar {
    var self_1139: Scalar;
    var exponent_3: i32;
    var local_1: Scalar;
    var x_1: Scalar;
    var y_1: Scalar;
    var n_1: i32;

    self_1139 = self_1138;
    exponent_3 = exponent_2;
    let _e4: i32 = exponent_3;
    if (_e4 == 0) {
        {
            let _e7: Scalar = scalar_one();
            return _e7;
        }
    }
    let _e8: i32 = exponent_3;
    if (_e8 < 0) {
        let _e12: Scalar = self_1139;
        let _e13: Scalar = scalar_inverse(_e12);
        local_1 = _e13;
    } else {
        let _e14: Scalar = self_1139;
        local_1 = _e14;
    }
    let _e16: Scalar = local_1;
    x_1 = _e16;
    let _e18: Scalar = scalar_one();
    y_1 = _e18;
    let _e21: i32 = exponent_3;
    n_1 = abs(_e21);
    loop {
        let _e25: i32 = n_1;
        if !((1 < _e25)) {
            break;
        }
        {
            let _e28: i32 = n_1;
            if ((_e28 & 1) == 1) {
                {
                    let _e35: Scalar = x_1;
                    let _e36: Scalar = y_1;
                    let _e37: Scalar = scalar_scalar_geometric_product(_e35, _e36);
                    y_1 = _e37;
                }
            }
            let _e40: Scalar = x_1;
            let _e41: Scalar = x_1;
            let _e42: Scalar = scalar_scalar_geometric_product(_e40, _e41);
            x_1 = _e42;
            let _e43: i32 = n_1;
            n_1 = (_e43 >> u32(1));
        }
    }
    let _e49: Scalar = x_1;
    let _e50: Scalar = y_1;
    let _e51: Scalar = scalar_scalar_geometric_product(_e49, _e50);
    return _e51;
}

fn scalar_scalar_geometric_quotient(self_1140: Scalar, other_936: Scalar) -> Scalar {
    var self_1141: Scalar;
    var other_937: Scalar;

    self_1141 = self_1140;
    other_937 = other_936;
    let _e6: Scalar = other_937;
    let _e7: Scalar = scalar_inverse(_e6);
    let _e8: Scalar = self_1141;
    let _e10: Scalar = other_937;
    let _e11: Scalar = scalar_inverse(_e10);
    let _e12: Scalar = scalar_scalar_geometric_product(_e8, _e11);
    return _e12;
}

fn scalar_scalar_transformation(self_1142: Scalar, other_938: Scalar) -> Scalar {
    var self_1143: Scalar;
    var other_939: Scalar;

    self_1143 = self_1142;
    other_939 = other_938;
    let _e6: Scalar = self_1143;
    let _e7: Scalar = other_939;
    let _e8: Scalar = scalar_scalar_geometric_product(_e6, _e7);
    let _e10: Scalar = self_1143;
    let _e11: Scalar = scalar_reversal(_e10);
    let _e14: Scalar = self_1143;
    let _e15: Scalar = other_939;
    let _e16: Scalar = scalar_scalar_geometric_product(_e14, _e15);
    let _e18: Scalar = self_1143;
    let _e19: Scalar = scalar_reversal(_e18);
    let _e20: Scalar = scalar_scalar_geometric_product(_e16, _e19);
    return _e20;
}

fn scalar_sphere_transformation(self_1144: Scalar, other_940: Sphere) -> Sphere {
    var self_1145: Scalar;
    var other_941: Sphere;

    self_1145 = self_1144;
    other_941 = other_940;
    let _e6: Scalar = self_1145;
    let _e7: Sphere = other_941;
    let _e8: Sphere = scalar_sphere_geometric_product(_e6, _e7);
    let _e10: Scalar = self_1145;
    let _e11: Scalar = scalar_reversal(_e10);
    let _e14: Scalar = self_1145;
    let _e15: Sphere = other_941;
    let _e16: Sphere = scalar_sphere_geometric_product(_e14, _e15);
    let _e18: Scalar = self_1145;
    let _e19: Scalar = scalar_reversal(_e18);
    let _e20: Sphere = sphere_scalar_geometric_product(_e16, _e19);
    return _e20;
}

fn scalar_translator_transformation(self_1146: Scalar, other_942: Translator) -> Translator {
    var self_1147: Scalar;
    var other_943: Translator;

    self_1147 = self_1146;
    other_943 = other_942;
    let _e6: Scalar = self_1147;
    let _e7: Translator = other_943;
    let _e8: Translator = scalar_translator_geometric_product(_e6, _e7);
    let _e10: Scalar = self_1147;
    let _e11: Scalar = scalar_reversal(_e10);
    let _e14: Scalar = self_1147;
    let _e15: Translator = other_943;
    let _e16: Translator = scalar_translator_geometric_product(_e14, _e15);
    let _e18: Scalar = self_1147;
    let _e19: Scalar = scalar_reversal(_e18);
    let _e20: Translator = translator_scalar_geometric_product(_e16, _e19);
    return _e20;
}

fn sphere_scalar_geometric_quotient(self_1148: Sphere, other_944: Scalar) -> Sphere {
    var self_1149: Sphere;
    var other_945: Scalar;

    self_1149 = self_1148;
    other_945 = other_944;
    let _e6: Scalar = other_945;
    let _e7: Scalar = scalar_inverse(_e6);
    let _e8: Sphere = self_1149;
    let _e10: Scalar = other_945;
    let _e11: Scalar = scalar_inverse(_e10);
    let _e12: Sphere = sphere_scalar_geometric_product(_e8, _e11);
    return _e12;
}

fn translator_dipole_geometric_quotient(self_1150: Translator, other_946: Dipole) -> Motor {
    var self_1151: Translator;
    var other_947: Dipole;

    self_1151 = self_1150;
    other_947 = other_946;
    let _e6: Dipole = other_947;
    let _e7: Dipole = dipole_inverse(_e6);
    let _e8: Translator = self_1151;
    let _e10: Dipole = other_947;
    let _e11: Dipole = dipole_inverse(_e10);
    let _e12: Motor = translator_dipole_geometric_product(_e8, _e11);
    return _e12;
}

fn translator_scalar_geometric_quotient(self_1152: Translator, other_948: Scalar) -> Translator {
    var self_1153: Translator;
    var other_949: Scalar;

    self_1153 = self_1152;
    other_949 = other_948;
    let _e6: Scalar = other_949;
    let _e7: Scalar = scalar_inverse(_e6);
    let _e8: Translator = self_1153;
    let _e10: Scalar = other_949;
    let _e11: Scalar = scalar_inverse(_e10);
    let _e12: Translator = translator_scalar_geometric_product(_e8, _e11);
    return _e12;
}

fn main_1() {
    return;
}

@compute @workgroup_size(1, 1, 1) 
fn main() {
    main_1();
    return;
}
